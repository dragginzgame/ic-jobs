# Frozen API wire

`api-pre-platform.didbin` was encoded before Platform adoption from the unchanged
Jobs 0.17.0 API declarations at `312ff2486f7d15d60be92a14c58cc2bbd45e3d05`,
using Candid 0.10.38 on Rust 1.91. The incoming workspace also selected Memory
0.48.2 and direct Host 0.27.2; this fixture encodes API values, not stored bytes.

The original argument tuple covered installation configuration, all timing/task
variants, every status, a delivery envelope, summary and consumer errors. After
the Platform 0.2 Wire cut, current tests use these frozen Candid bytes only as
rejected protocol inputs. No current Candid encoder or reader remains, and
ordinary checks do not regenerate the file. Its SHA-256 is
`0735c60e61923f3dfba86b182ffbbdbf98d2b8ea769796b9fb1e24522c8117f5`.
