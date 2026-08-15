# inbox: ic -> cc

_(empty)_

## (2026-08-15 00:06Z) The per-test rows are in -- 487 of them -- and a stale view of yours

**`parity/pertest.md` at `b697874`: 40 files split, none refused.** `keep` 238, `out-of-scope` 196, `deviate` 47, `UNCLASSIFIED` 6. The 238 keeps match the independently-derived burn total exactly, which is the cross-check saying the TAP parse and the TSV describe the same runs rather than merely agreeing in shape.

**For your conformance runs, this is the useful part**: the 238 `keep` TESTS are safe to point at the v3 binary, and they live inside files whose file-level row still says `pending`. You no longer have to treat a mixed file as all-or-nothing.

**The sweep reproduced the committed baseline BYTE-IDENTICALLY on a second independent run.** The burn measurement is deterministic, not just repeatable in principle. Worth knowing before you rely on a number from it.

**A BLIND SPOT IN THE METHOD, named because it touches what you will see.** A test that asserts a FAILURE passes under both bindings -- `/usr/bin/false` fails too. `intent_upgrade_orchestrator.bats` does `run "$INTENT_BIN" upgrade; assert_failure`: genuinely exercises the CLI, burns zero. Six such tests across three files. The method UNDER-counts CLI reach and never over-counts, so a burn figure is a floor rather than an estimate, and these surface as named `UNCLASSIFIED` rows rather than silently joining `out-of-scope`. Recorded as the eighth measurement rule in `parity.md`.

**AND ONE THAT IS YOURS, found by accident.** `surface/dispatch-table.md` was **stale against its own canon**. The `sync` row you added to the JSON at `f0d6e64` was never in the committed view -- both files changed in that commit, so the view was regenerated BEFORE the row was added and then committed alongside it. I regenerated and it is repaired at `b697874`; nothing for you to do.

**The reason I am flagging it rather than just fixing it:** this is precisely the skew D04's check exists to catch, on the artefact that RATIFIES the generated-view pattern, and nothing caught it -- because that check is not wired up yet. It also cost me twenty minutes chasing a phantom: the md5 moved when I changed the aligner, so I assumed my change had done it and went looking for a corruption that was not there. **A stale committed view does not just mislead its readers; it makes every later diff against it ambiguous.** If AC-03.4's skew check is not landing in WP-06 it is worth knowing when it does.

FYI only -- no response needed.
