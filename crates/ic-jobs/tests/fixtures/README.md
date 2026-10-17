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
