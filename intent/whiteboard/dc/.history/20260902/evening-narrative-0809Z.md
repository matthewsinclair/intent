# dc -- the evening of 2026-09-01, archived 2026-09-02 08:09Z

Kept because the reasoning is the useful part and the board is not its home. Three of the four items below are errors of mine, and they are kept as **two kinds rather than one count**, because the cures differ.

## The run itself

hv drove `dvb fullcycle` twice. Take 1 refused at phase 1 in one second and destroyed nothing. Take 2, with `--force`, went `clean rust` rc=0 (2.8G) -> `build all` rc=0 (1m41s, release profile through `guarded_release_build`, into the SHARED path because the dirt scope was empty) -> `test rust` rc=0 (1m01s). The darkening check stayed silent and both PATH symlinks resolved afterwards.

**The figure is hv's, at HEAD `04f31ad2` with the tree dirty, and the gate's own footer is right that it describes no commit.** Log at `tmp/fullcycle/20260901-1858.FULLCYC.out`.

## Design fault: I invented a mechanism rather than reading the failure (W44)

cc had 23 failures, all daemon-family. I reasoned from the FAMILY, found a real anomaly, and attached the failures to it: `clean` deleted `intentd` at 17:58Z and `build all` wrote it back at 18:00:23Z, so pid 66522 has been running orphaned inode `674102107` while the path holds `674931798`.

**The measurement was sound. cc replicated the inode independently. The inference was wrong.** The cause was `intent-cli/tests/common/mod.rs:782`, a deliberate staleness guard whose assertion message names the mechanism, the offending file and the fix in thirteen lines. cc had edited `nav.rs`; my run passed only because `build all` had just refreshed the sibling.

**devbin-vc's general form is the sharpest sentence anyone produced across the two days:** _the symptom was identical under both mechanisms, so nothing downstream of the symptom would ever have caught the wrong one. A wrong mechanism producing the right symptom has no natural corrective._ That is why this is not a small error: my story predicted daemon-family failures and so did the truth. Only reading the assertion separated them, and I carried the invented one to hv.

## Attention fault: I denied publishing a figure sitting in my own archive (W45)

I told vc I had never published `2022/0` and offered it as an instance of the day's class. It is on my own board twice -- `20260901/wip-prefold-1706Z.md:39` and `rust-consolidation-narrative-1334Z.md:15` -- correctly attributed there to hv's clean cycle.

**A localfold moves a claim off the live board and not out of the record. Peers read the archive; folding is not unpublishing.** And the misattribution that actually happened was one participant to the left of where I aimed it: the figure is hv's run, and cc, vc and I all carried it as a dc measurement, dropping whose box and whose cycle produced it.

## Design fault, outward: a meter measuring the wrong subject (W46)

`fullcycle` closed with `overhead: 4.87x -- 170s wall against 34.91s of reported test time`. Exact arithmetic; wrong subject. `DEVBIN_RUN_T0` spans the CYCLE, so the wall carries `clean` (2s) and `build` (1m41s) -- 103 of 170 seconds that were never tests. Test phase alone is 61s/34.91s = 1.75x, quiet, against a published scale where 1.14x is quiet and 80x is a machine being eaten.

`runlog:1027` declares three limits in its own code, deliberately. **This is a fourth, and the meter's self-check cannot reach it:** sub-1 reports UNDEFINED, which catches parallelism pushing the ratio DOWN; non-test work pushes it UP, into the range that reads as a finding. Filed to devbin-vc with an explicit instruction to verify it in `runlog` before acting, on the strength of W44 the same afternoon.

## What went out, and what came back

Two of my findings became other people's code. devbin's `0046` was corrected to my provenance mechanism (`write_errors_file` seals the log, `print_run_referent` runs after it -- ordering, not routing) after they verified it in their own source. The `fullcycle` notice was fixed on eleven estates at `bd323ea`, naming both of `clean`'s refusals and deliberately predicting neither.

**hv ruled `0047` option 3:** force only the blocked-binaries arm, keep the removal confirmation, because `--force` merges two consents and `fullcycle` discharges exactly one of them. Flagged with the ruling: **option 3 changes which refusal a headless run reaches**, so the notice just fixed needs re-reading -- the same class arriving through its own fix.

## The standing rule, which corrects one I had been prescribing

**A printed number gets read instead of run.** Stating a figure with its regenerating command beside it buys auditability and nothing for currency. LIVE figure: print the COMMAND ALONE. HISTORICAL figure: DATE it and say whose run it was.
