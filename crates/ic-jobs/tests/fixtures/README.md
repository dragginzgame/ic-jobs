# Frozen record producer

`records-075.json` was emitted by archived Jobs 0.7.5 commit
`060fb9d226389bab702132ba23f42bd0b887a690` on Rust 1.88 before the checked
configuration implementation. The release’s bounded transition-history test
produced valid records using its public creation, management, retry and outcome
operations. Representatives retain all schedule policies, states, retry budgets,
attempt counts, schedule rebasing and maximum timestamps. Strings preserve the
producer’s exact compact JSON encoding. The new reader restores those records
and compares exact re-encoding; it does not regenerate expected bytes.

The stable consumer separately freezes a snapshot from that producer source in
`apps/job-consumer/src/queue/fixtures/snapshot-075.json`. A second sparse hex fixture,
`apps/job-consumer/src/queue/fixtures/stable-image-075.json`, retains all bytes
of the old producer’s complete stable memory, including allocation/cell headers.
Zero bytes are implicit and nonzero runs are exact. MemoryRuntime republishes
its ledger on reopen in both releases; the Jobs cell remains byte-identical.
The current-memory-key reopen tests preserves blocked external effects and paused work without reset.
These fixtures qualify the maintained JSON codec, not arbitrary external codecs
or live IC upgrades. Generator source and logs are retained with local
qualification evidence.
