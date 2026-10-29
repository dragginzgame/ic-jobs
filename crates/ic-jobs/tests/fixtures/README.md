# Stored record fixtures

`uncertain-record.json` is hand-authored current-format JSON. Its outcome and
timestamp form one result. The codec test checks exact re-encoding and validated
restoration, including blocked dispatch until explicit result reconciliation.
The bounded public transition-history test separately covers every state and
checks restoration after management, retries and completion.

The consumer's frozen `snapshot-075.json` remains solely as a refusal fixture,
from archived Jobs 0.7.5 commit `060fb9d226389bab702132ba23f42bd0b887a690`.
The consumer rejects it on current Memory backing without replacing its cell.
Its sparse `stable-image-075.json` and `stable-image-memory040.json` fixtures
retain earlier physical images for the independent incompatible-ledger refusal
proof. Their zero bytes are implicit; nonzero runs and producer bytes remain exact.
These are negative inputs, not compatibility readers or an upgrade path.
Native fixture checks do not establish live IC upgrades or message rollback.

## Memory 0.56 allocation-ledger cut

The consumer's `stable-image-memory0552.json` freezes a complete three-page
Memory 0.55.2 installation from Jobs 0.17.47 at
`cdb48cc9d1c3281e2b898bcfe4a19e2a673b4560`, produced on Rust 1.91. It contains
one valid Running delivery intent. After dropping all earlier handles, the old
producer reconstructed the exact physical image, reopened it, recovered the
same delivery and refused redispatch of the blocked job.

The fixture retains ten nonzero runs and the unchanged Jobs JSON payload. Memory
0.56 refuses its retired CBOR ledger during recovery and fresh creation with
zero writes, unchanged pages and identical bytes. Independently decoding the
queue payload still uses Job::restore and leaves the job Running/blocked. This
separates the infrastructure format cut from unchanged job/effect authority;
it supplies neither a converter nor permission to discard retained work.
The fixture SHA-256 is
`780ffd07de7ccc8d47398fce67d223e62e6d3c37b2618af502fa299eeaa0c7a7`.
Generation inputs and old-producer qualification remain at
`/tmp/ic-jobs-tooling0702-memory056.8W4xazfe/old-producer/`.

## 0.16 refusal inputs

`running-record-015.json` freezes the previous timestamp-bearing Running record.
The consumer also retains `snapshot-running-015.json` and
`snapshot-receipt-015.json` under its queue fixtures and
`delivery-reply-015.didbin` under its reply fixtures. These freeze the old Running
snapshot, flat receipt snapshot and successful flat Candid delivery reply.

They were produced using the incoming 0.15 Job/Task/Delivery serializers at Jobs
`71761e0c8ce3c6b2a4ce0226b80691892aa57f63`, with the existing Build/consumer
cleanup and selected Timers 0.27 graph. Snapshot headers were assembled around
those typed values. The prior queue decoder admits both snapshots, and the prior
Candid decoder admits the reply; that focused qualification is retained in
`target/qualification/running-payload-cut/`. The capture helper and original
source remain in `/tmp/jobs-running-payload-cut.9krmjk9u/before-source/`.

Current checks refuse each changed shape, preserving refused stable payloads
through recovery and fresh-install refusal. No reader, converter or deployed
reset is added. The unchanged current Uncertain fixture above remains positive
blocked-restoration evidence.
