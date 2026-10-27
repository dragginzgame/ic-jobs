use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

use ic_jobs::Outcome;
use ic_memory::DefaultMemoryImpl;
use ic_platform::cdk;
use ic_timers::{
    TimerCadence, TimerIdentity, WatchdogReconcileState, WatchdogRegistration, WatchdogRunResult,
    reconcile_watchdog,
};
use ic_wire::native;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    api::{Create, Delivery, Error, Init, Summary, View},
    delivery_wire::{
        Method as DeliveryMethod, RECEIPT_ID, RECEIVE_ID, REQUEST_LIMITS, RESPONSE_LIMITS,
        decode_reply,
    },
    management_wire::{self as control, Method},
    queue::{Committed, DispatchBatch, Queue, QueueTotals},
};

thread_local! {
    static QUEUE: RefCell<Option<Queue<DefaultMemoryImpl>>> = const { RefCell::new(None) };
    static WATCHDOG: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    // Fault control belongs only to this unpublished qualification canister.
    static TRAP_WATCHDOG_BATCH: Cell<bool> = const { Cell::new(false) };
}

fn required<T>(result: Result<T, impl std::fmt::Debug>) -> T {
    result.unwrap_or_else(|error| cdk::api::trap(format!("consumer boundary failed: {error:?}")))
}

fn with_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<T, Error>,
) -> Result<T, Error> {
    QUEUE.with(|queue| operation(queue.borrow_mut().as_mut().ok_or(Error::InvalidStorage)?))
}

fn with_authorized_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<T, Error>,
) -> Result<T, Error> {
    with_queue(|queue| {
        queue.authorize(cdk::api::msg_caller())?;
        operation(queue)
    })
}

fn update_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<Committed<T>, Error>,
) -> Result<T, Error> {
    Ok(reconcile_committed(with_authorized_queue(operation)?))
}

fn reconcile_committed<T>(committed: Committed<T>) -> T {
    // Reconciliation failure traps the same message segment as the stable write.
    reconcile_at(committed.next_due_ns);
    committed.value
}

fn reconcile_at(due: Option<u64>) {
    WATCHDOG.with(|watchdog| {
        required(reconcile_watchdog(
            &mut watchdog.borrow_mut(),
            &required(TimerIdentity::try_new(
                "jobs-test-consumer",
                "queue",
                "dispatch",
            )),
            required(TimerCadence::new(Duration::from_secs(1))),
            due.map_or(
                WatchdogReconcileState::Inactive,
                WatchdogReconcileState::ScheduledAt,
            ),
            |_| wake(),
        ))
    });
}

fn admit_control_input() -> Result<(), String> {
    if ic_platform::ic0::msg_arg_data_size() > control::REQUEST_LIMITS.bytes {
        return Err("management request exceeds Wire frame limit".into());
    }
    Ok(())
}

fn decode_control<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    required(native::decode_request(ID, &bytes, control::REQUEST_LIMITS))
}

fn encode_control<T: Serialize, const ID: u64>(result: T) -> Vec<u8> {
    required(native::encode_response(
        ID,
        &result,
        control::RESPONSE_LIMITS,
    ))
}

fn encode_unit<const ID: u64>() -> Vec<u8> {
    encode_control::<_, ID>(())
}

fn recover(initial: Option<Init>) {
    let (queue, due) = required(Queue::open(DefaultMemoryImpl::default(), initial));
    // Open/restore the entire bounded snapshot before registering any callback.
    QUEUE.with(|slot| *slot.borrow_mut() = Some(queue));
    required(ic_timers::initialize_runtime());
    reconcile_at(due);
}

#[cdk::init]
fn init() {
    // Lifecycle macros have no guards. Admit length before copying init bytes,
    // then use the same bounded codec as the argument-bearing RPC hooks.
    required(admit_control_input());
    let config = decode_control::<Init, { Method::Init as u64 }>(cdk::api::msg_arg_data());
    recover(Some(config));
}

#[cdk::post_upgrade]
fn post_upgrade() {
    recover(None);
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::Create as u64 }>",
    encode_with = "encode_control::<_, { Method::Create as u64 }>"
)]
fn create(input: Create) -> Result<u64, Error> {
    update_queue(|queue| queue.create(input))
}

#[cdk::query(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::Inspect as u64 }>",
    encode_with = "encode_control::<_, { Method::Inspect as u64 }>"
)]
fn inspect(id: u64) -> Result<View, Error> {
    with_authorized_queue(|queue| queue.inspect(id))
}

#[cdk::query(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::List as u64 }>",
    encode_with = "encode_control::<_, { Method::List as u64 }>"
)]
fn list(after: Option<u64>, limit: u8) -> Result<Vec<View>, Error> {
    with_authorized_queue(|queue| queue.page(after, limit))
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::Cancel as u64 }>",
    encode_with = "encode_control::<_, { Method::Cancel as u64 }>"
)]
fn cancel(id: u64) -> Result<(), Error> {
    update_queue(|queue| queue.cancel(id, cdk::api::time()))
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::Pause as u64 }>",
    encode_with = "encode_control::<_, { Method::Pause as u64 }>"
)]
fn pause(id: u64) -> Result<(), Error> {
    update_queue(|queue| queue.pause(id, cdk::api::time()))
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::Resume as u64 }>",
    encode_with = "encode_control::<_, { Method::Resume as u64 }>"
)]
fn resume(id: u64) -> Result<(), Error> {
    update_queue(|queue| queue.resume(id, cdk::api::time()))
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::Reschedule as u64 }>",
    encode_with = "encode_control::<_, { Method::Reschedule as u64 }>"
)]
fn reschedule(id: u64, timing: crate::api::Timing) -> Result<(), Error> {
    update_queue(|queue| queue.reschedule(id, timing, cdk::api::time()))
}

#[cdk::query(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::Summary as u64 }>",
    encode_with = "encode_control::<_, { Method::Summary as u64 }>"
)]
fn summary() -> Result<Summary, Error> {
    let Committed {
        value: QueueTotals { counter, receipts },
        next_due_ns,
    } = with_authorized_queue(|queue| queue.summary())?;
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
    let committed = with_queue(|queue| queue.finish(delivery, cdk::api::time(), outcome))?;
    reconcile_committed(committed);
    Ok(())
}

async fn delivery_call(
    delivery: &Delivery,
    method: DeliveryMethod,
) -> Result<cdk::call::Response, cdk::call::CallFailed> {
    let bytes = required(native::encode_request(
        method.id(),
        delivery,
        REQUEST_LIMITS,
    ));
    cdk::call::Call::unbounded_wait(delivery.payload.destination, method.name())
        .with_raw_args(&bytes)
        .await
}

async fn delivery_confirmed(delivery: &Delivery) -> bool {
    let method = DeliveryMethod::Receive;
    let response = delivery_call(delivery, method).await;
    decode_reply::<Delivery, _>(method.id(), response).is_some_and(|receipt| receipt == *delivery)
}

async fn deliver(delivery: Delivery) {
    let confirmed = delivery_confirmed(&delivery).await;
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

fn launch_deliveries(deliveries: Vec<Delivery>) {
    for delivery in deliveries {
        cdk::futures::spawn(deliver(delivery));
    }
}

fn wake() -> WatchdogRunResult {
    let result = required(with_queue(|queue| queue.dispatch(cdk::api::time())));
    let DispatchBatch { deliveries, work } = result.value;
    if work > 0 && TRAP_WATCHDOG_BATCH.with(Cell::get) {
        // A stable write in this segment is not independently committed. No
        // external delivery escapes before this deliberate rollback boundary.
        cdk::api::trap("fixture: watchdog trap after stable batch write");
    }
    launch_deliveries(deliveries);
    // Work counts handled attempts, not confirmed external effects.
    WatchdogRunResult::completed_batch(work, result.next_due_ns)
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::DispatchDue as u64 }>",
    encode_with = "encode_control::<_, { Method::DispatchDue as u64 }>"
)]
fn dispatch_due() -> Result<(), Error> {
    launch_deliveries(dispatch_reconciled()?);
    Ok(())
}

fn dispatch_reconciled() -> Result<Vec<Delivery>, Error> {
    Ok(update_queue(|queue| queue.dispatch(cdk::api::time()))?.deliveries)
}

fn admit_delivery_input() -> Result<(), String> {
    // Platform runs guards before msg_arg_data copies the complete argument.
    if ic_platform::ic0::msg_arg_data_size() > REQUEST_LIMITS.bytes {
        return Err("delivery request exceeds Wire frame limit".into());
    }
    Ok(())
}

fn decode_delivery<const ID: u64>(bytes: Vec<u8>) -> Delivery {
    required(native::decode_request(ID, &bytes, REQUEST_LIMITS))
}

fn encode_delivery<T: Serialize, const ID: u64>(result: T) -> Vec<u8> {
    // Encoding failure traps in the same segment as the receipt write.
    required(native::encode_response(ID, &result, RESPONSE_LIMITS))
}

#[cdk::update(
    guard = "admit_delivery_input",
    decode_with = "decode_delivery::<RECEIVE_ID>",
    encode_with = "encode_delivery::<_, RECEIVE_ID>"
)]
fn receive_delivery(delivery: Delivery) -> Result<Delivery, Error> {
    if delivery.payload.destination != cdk::api::canister_self() {
        return Err(Error::ReceiptMismatch);
    }
    with_queue(|queue| queue.receive(cdk::api::msg_caller(), delivery))
}

#[cdk::update(
    guard = "admit_delivery_input",
    decode_with = "decode_delivery::<RECEIPT_ID>",
    encode_with = "encode_delivery::<_, RECEIPT_ID>"
)]
fn delivery_receipt(delivery: Delivery) -> Result<Option<Delivery>, Error> {
    with_queue(|queue| queue.receipt(cdk::api::msg_caller(), &delivery))
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::ReconcileDelivery as u64 }>",
    encode_with = "encode_control::<_, { Method::ReconcileDelivery as u64 }>"
)]
async fn reconcile_delivery(id: u64) -> Result<bool, Error> {
    let delivery = with_authorized_queue(|queue| queue.outstanding(id))?;
    let method = DeliveryMethod::Receipt;
    let response = delivery_call(&delivery, method).await;
    let receipt = decode_reply::<Option<Delivery>, _>(method.id(), response).flatten();
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
#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::TrapWatchdogBatch as u64 }>",
    encode_with = "encode_control::<_, { Method::TrapWatchdogBatch as u64 }>"
)]
fn fixture_trap_watchdog_batch(enabled: bool) -> Result<(), Error> {
    with_authorized_queue(|_| Ok(()))?;
    TRAP_WATCHDOG_BATCH.with(|fault| fault.set(enabled));
    Ok(())
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::TrapAfterCreate as u64 }>",
    encode_with = "encode_unit::<{ Method::TrapAfterCreate as u64 }>"
)]
fn fixture_trap_after_create(input: Create) {
    required(create(input));
    cdk::api::trap("fixture: trap after stable write and watchdog reconciliation");
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::TrapAfterPause as u64 }>",
    encode_with = "encode_unit::<{ Method::TrapAfterPause as u64 }>"
)]
fn fixture_trap_after_pause(id: u64) {
    required(pause(id));
    cdk::api::trap("fixture: trap after paused write and watchdog reconciliation");
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::TrapAfterReschedule as u64 }>",
    encode_with = "encode_unit::<{ Method::TrapAfterReschedule as u64 }>"
)]
fn fixture_trap_after_reschedule(id: u64, timing: crate::api::Timing) {
    required(reschedule(id, timing));
    cdk::api::trap("fixture: trap after rescheduled write and watchdog reconciliation");
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::CorruptRecord as u64 }>",
    encode_with = "encode_control::<_, { Method::CorruptRecord as u64 }>"
)]
fn fixture_corrupt_record(id: u64) -> Result<(), Error> {
    with_authorized_queue(|queue| queue.corrupt_record(id))
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::MarkUncertain as u64 }>",
    encode_with = "encode_control::<_, { Method::MarkUncertain as u64 }>"
)]
fn fixture_mark_uncertain(id: u64) -> Result<(), Error> {
    let delivery = with_authorized_queue(|queue| queue.outstanding(id))?;
    complete_delivery(&delivery, Outcome::Uncertain)?;
    Ok(())
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::DispatchTrapResult as u64 }>",
    encode_with = "encode_control::<_, { Method::DispatchTrapResult as u64 }>"
)]
async fn fixture_dispatch_trap_result(input: Create) -> Result<(), Error> {
    create(input)?;
    let deliveries = dispatch_reconciled()?;
    // Intent commits at the real inter-canister call boundary. The receipt lives
    // at the destination; a later sender trap cannot roll back that earlier call.
    if let Some(delivery) = deliveries.into_iter().next() {
        if !delivery_confirmed(&delivery).await {
            cdk::api::trap("fixture delivery was not confirmed");
        }
        cdk::api::trap("fixture: lost sender result after confirmed external effect");
    }
    Ok(())
}

#[cdk::update(
    guard = "admit_control_input",
    decode_with = "decode_control::<_, { Method::CreateBlocked as u64 }>",
    encode_with = "encode_control::<_, { Method::CreateBlocked as u64 }>"
)]
fn fixture_create_blocked(input: Create) -> Result<u64, Error> {
    let id = create(input)?;
    // Simulate interruption after committed intent but before dispatch. This
    // endpoint never sends an envelope or claims an absent receipt is retry safe.
    dispatch_reconciled()?;
    Ok(id)
}
