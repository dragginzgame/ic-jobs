use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

use ic_jobs::{Outcome, timers};
use ic_memory::ic_stable_structures::DefaultMemoryImpl;
use ic_timers::{
    TimerCadence, TimerCompletion, TimerIdentity, WatchdogRegistration, WatchdogRunResult,
};

use crate::{
    api::{Create, Delivery, Error, Init, Summary, View},
    queue::Queue,
};

thread_local! {
    static QUEUE: RefCell<Option<Queue<DefaultMemoryImpl>>> = const { RefCell::new(None) };
    static WATCHDOG: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    // Fault control belongs only to this unpublished qualification canister.
    static TRAP_WATCHDOG_BATCH: Cell<bool> = const { Cell::new(false) };
}

fn required<T>(result: Result<T, impl std::fmt::Debug>) -> T {
    result.unwrap_or_else(|error| ic_cdk::trap(format!("consumer boundary failed: {error:?}")))
}

fn with_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<T, Error>,
) -> Result<T, Error> {
    QUEUE.with(|queue| operation(queue.borrow_mut().as_mut().ok_or(Error::InvalidStorage)?))
}

fn authorize() -> Result<(), Error> {
    with_queue(|queue| queue.authorize(ic_cdk::api::msg_caller()))
}

fn update_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<T, Error>,
) -> Result<T, Error> {
    authorize()?;
    let result = with_queue(operation)?;
    // Reconciliation failure traps the same message segment as the stable write.
    reconcile();
    Ok(result)
}

fn reconcile() {
    let (due, _, _) = required(with_queue(|queue| queue.summary()));
    WATCHDOG.with(|watchdog| {
        required(timers::reconcile(
            &mut watchdog.borrow_mut(),
            &required(TimerIdentity::try_new(
                "jobs-test-consumer",
                "queue",
                "dispatch",
            )),
            required(TimerCadence::new(Duration::from_secs(1))),
            due,
            |_| wake(),
        ))
    });
}

fn recover(initial: Option<Init>) {
    let queue = required(Queue::open(DefaultMemoryImpl::default(), initial));
    // Open/restore the entire bounded snapshot before registering any callback.
    QUEUE.with(|slot| *slot.borrow_mut() = Some(queue));
    required(ic_timers::initialize_runtime());
    reconcile();
}

#[ic_cdk::init]
fn init(config: Init) {
    recover(Some(config));
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    recover(None);
}

#[ic_cdk::update]
fn create(input: Create) -> Result<u64, Error> {
    update_queue(|queue| queue.create(input))
}

#[ic_cdk::query]
fn inspect(id: u64) -> Result<View, Error> {
    authorize()?;
    with_queue(|queue| queue.inspect(id))
}

#[ic_cdk::query]
fn list(after: Option<u64>, limit: u8) -> Result<Vec<View>, Error> {
    authorize()?;
    with_queue(|queue| queue.page(after, limit))
}

#[ic_cdk::update]
fn cancel(id: u64) -> Result<(), Error> {
    update_queue(|queue| queue.cancel(id, ic_cdk::api::time()))
}

#[ic_cdk::update]
fn pause(id: u64) -> Result<(), Error> {
    update_queue(|queue| queue.pause(id, ic_cdk::api::time()))
}

#[ic_cdk::update]
fn resume(id: u64) -> Result<(), Error> {
    update_queue(|queue| queue.resume(id, ic_cdk::api::time()))
}

#[ic_cdk::update]
fn reschedule(id: u64, timing: crate::api::Timing) -> Result<(), Error> {
    update_queue(|queue| queue.reschedule(id, timing, ic_cdk::api::time()))
}

#[ic_cdk::query]
fn summary() -> Result<Summary, Error> {
    authorize()?;
    let (next_due_ns, counter, receipts) = with_queue(|queue| queue.summary())?;
    let armed = WATCHDOG.with(|watchdog| {
        watchdog
            .borrow()
            .as_ref()
            .is_some_and(|registration| required(registration.has_armed_wakeup()))
    });
    Ok(Summary {
        next_due_ns,
        counter,
        receipts,
        armed,
    })
}

fn complete_delivery(delivery: &Delivery, outcome: Outcome) -> Result<(), Error> {
    with_queue(|queue| queue.finish(delivery, ic_cdk::api::time(), outcome))?;
    reconcile();
    Ok(())
}

async fn deliver(delivery: Delivery) {
    let response = ic_cdk::call::Call::unbounded_wait(delivery.destination, "receive_delivery")
        .with_arg(&delivery)
        .await;
    let confirmed = response
        .ok()
        .and_then(|response| response.candid::<Result<Delivery, Error>>().ok())
        .and_then(Result::ok)
        .is_some_and(|receipt| receipt == delivery);
    // Transport rejection or codec failure cannot establish that no effect ran.
    required(complete_delivery(
        &delivery,
        if confirmed {
            Outcome::Success
        } else {
            Outcome::Uncertain
        },
    ));
}

fn wake() -> WatchdogRunResult {
    let (deliveries, work) = required(with_queue(|queue| queue.dispatch(ic_cdk::api::time())));
    if work > 0 && TRAP_WATCHDOG_BATCH.with(Cell::get) {
        // A stable write in this segment is not independently committed. No
        // external delivery escapes before this deliberate rollback boundary.
        ic_cdk::trap("fixture: watchdog trap after stable batch write");
    }
    for delivery in deliveries {
        ic_cdk::futures::spawn(deliver(delivery));
    }
    let (due, _, _) = required(with_queue(|queue| queue.summary()));
    let completion = if work == 0 {
        TimerCompletion::no_work()
    } else {
        // Work units count handled attempts, not successful external effects.
        TimerCompletion::success(work)
    };
    timers::complete_batch(completion, due)
}

#[ic_cdk::update]
fn dispatch_due() -> Result<(), Error> {
    let (deliveries, _) = update_queue(|queue| queue.dispatch(ic_cdk::api::time()))?;
    for delivery in deliveries {
        ic_cdk::futures::spawn(deliver(delivery));
    }
    Ok(())
}

#[ic_cdk::update]
fn receive_delivery(delivery: Delivery) -> Result<Delivery, Error> {
    if delivery.destination != ic_cdk::api::canister_self() {
        return Err(Error::ReceiptMismatch);
    }
    with_queue(|queue| queue.receive(ic_cdk::api::msg_caller(), delivery))
}

#[ic_cdk::update]
fn delivery_receipt(delivery: Delivery) -> Result<Option<Delivery>, Error> {
    with_queue(|queue| queue.receipt(ic_cdk::api::msg_caller(), &delivery))
}

#[ic_cdk::update]
async fn reconcile_delivery(id: u64) -> Result<bool, Error> {
    authorize()?;
    let delivery = with_queue(|queue| queue.outstanding(id))?;
    let response = ic_cdk::call::Call::unbounded_wait(delivery.destination, "delivery_receipt")
        .with_arg(&delivery)
        .await;
    let receipt = response
        .ok()
        .and_then(|response| response.candid::<Result<Option<Delivery>, Error>>().ok())
        .and_then(Result::ok)
        .flatten();
    let Some(receipt) = receipt else {
        return Ok(false);
    };
    if receipt != delivery {
        return Err(Error::ReceiptMismatch);
    }
    complete_delivery(&delivery, Outcome::Success)?;
    Ok(true)
}

// These explicitly named fault controls exist only in this unpublished test
// canister. They use real IC traps; there is no production cfg(test) substitute.
#[ic_cdk::update]
fn fixture_trap_watchdog_batch(enabled: bool) -> Result<(), Error> {
    authorize()?;
    TRAP_WATCHDOG_BATCH.with(|fault| fault.set(enabled));
    Ok(())
}

#[ic_cdk::update]
fn fixture_trap_after_create(input: Create) {
    required(create(input));
    ic_cdk::trap("fixture: trap after stable write and watchdog reconciliation");
}

#[ic_cdk::update]
fn fixture_trap_after_pause(id: u64) {
    required(pause(id));
    ic_cdk::trap("fixture: trap after paused write and watchdog reconciliation");
}

#[ic_cdk::update]
fn fixture_trap_after_reschedule(id: u64, timing: crate::api::Timing) {
    required(reschedule(id, timing));
    ic_cdk::trap("fixture: trap after rescheduled write and watchdog reconciliation");
}

#[ic_cdk::update]
fn fixture_corrupt_record(id: u64) -> Result<(), Error> {
    authorize()?;
    with_queue(|queue| queue.corrupt_record(id))
}

#[ic_cdk::update]
fn fixture_mark_uncertain(id: u64) -> Result<(), Error> {
    authorize()?;
    let delivery = with_queue(|queue| queue.outstanding(id))?;
    complete_delivery(&delivery, Outcome::Uncertain)?;
    Ok(())
}

#[ic_cdk::update]
async fn fixture_dispatch_trap_result(input: Create) -> Result<(), Error> {
    create(input)?;
    let (deliveries, _) = with_queue(|queue| queue.dispatch(ic_cdk::api::time()))?;
    reconcile();
    // Intent commits at the real inter-canister call boundary. The receipt lives
    // at the destination; a later sender trap cannot roll back that earlier call.
    if let Some(delivery) = deliveries.into_iter().next() {
        let response = required(
            ic_cdk::call::Call::unbounded_wait(delivery.destination, "receive_delivery")
                .with_arg(&delivery)
                .await,
        );
        let receipt = required(required(response.candid::<Result<Delivery, Error>>()));
        if receipt != delivery {
            ic_cdk::trap("fixture receipt mismatch");
        }
        ic_cdk::trap("fixture: lost sender result after confirmed external effect");
    }
    Ok(())
}

#[ic_cdk::update]
fn fixture_create_blocked(input: Create) -> Result<u64, Error> {
    let id = create(input)?;
    // Simulate interruption after committed intent but before dispatch. This
    // endpoint never sends an envelope or claims an absent receipt is retry safe.
    with_queue(|queue| queue.dispatch(ic_cdk::api::time()))?;
    reconcile();
    Ok(id)
}

ic_cdk::export_candid!();
