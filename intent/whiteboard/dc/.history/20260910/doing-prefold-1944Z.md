# dc -- DOING/TODO verbatim before the 2026-09-10 1944Z localfold

Archived whole. An item removed without a trace is indistinguishable from one that was never here.

## DOING

**AC-12.1 -- HALF DELIVERED, HALF STOPPED AT A LINE THAT IS NOT MINE TO CROSS.**

`d8a8c070`: v2's dispatcher no longer advertises five doors v2 cannot serve. `125f601d8` (2026-08-30) deleted six plugin scripts and left `bin/intent` pointing at five of them, so `intent claude subagents|skills|upgrade|prime|rules` answered `Plugin command not found` and **both CI workflows went red on every push for eleven days** -- driven with `gh run list`, not relayed. Eight dispatch targets measured, five dead; after the commit every target the file names exists. **PRUNED, NOT RESTORED**: fail-forward, and bringing five scripts back would un-do completed ratified work. Parse-checked under `/bin/bash` 3.2.57 AND PATH bash 5.3 rather than run, because the machine was at load 235 and suites were stopped.

**IT DOES NOT TURN CI GREEN AND THE COMMIT SAYS SO.** The suites now meet `Unknown claude subcommand` instead of `Plugin command not found`. Removing the lie in the dispatcher is not removing the tests that assert it.

**THE STOP IS THE MORE VALUABLE HALF.** I was ONE COMMAND from deleting four bats suites. Checking what referenced them first is the only reason they are still here: all four are dispositioned **`keep`** in `parity/register.md` with the full burn **already recorded** (50/50, 39/39, 8/8, 2/2), and they are cited by `burn-baseline.tsv`, `lib_classify.sh` and four TAP baselines. **Deleting them re-bases the parity population and lands on `AC-06.1` -- a run only hv can make.** I would have re-based a baseline while fixing a red workflow, in one commit, with nothing saying so. [[W139]].

## TODO

**FOLDED 2026-09-10. Struck and delivered items are archived verbatim, not deleted** -- an item removed without a trace is indistinguishable from one that was never here.

**MINE, AND ALL OF IT IS BEHIND ONE hv RULING -- filed `d79eced9`, widened in the addendum:**

- **THREE SHIPPING DOORS ANSWER `not implemented yet`, AND ALL THREE POINT AT RETIRED SUBJECTS.** `st bootstrap` (WP-04), `agents template` (WP-07), `claude prime` (WP-07) -- cc found them, I verified each against `target/release` at `ec55b3ba` with BOTH controls (`st list` renders, `config` refuses). **`agents template`: hv's remedy is 2026-08-17 and the directory it enumerates lost its last reader at `b60f9ebb5` on 2026-08-26 -- nine days later.** `st bootstrap`: D3 installs a file `init` declares `NotByInit`, D8 depends on `claude prime`, D10 depends on `learn` -- both unwired. `claude prime`: consumes a file written by a date-stamping verb, against hv's clock ruling. **D2 is live, and it is the negative control that keeps this specific rather than a blanket claim.**
- **FOUR BATS SUITES DISPOSITIONED `keep`, TESTING DOORS PRUNED ELEVEN DAYS AGO.** Same class one layer down. See DOING.
- **`0270` OPTION 1 -- still held on hv's POST-TAG word.** No tag beyond v3.0.0; re-driven this morning, not recalled.
- **No smoke arm exercises `claude start` / `ws`.** Condition: a keg.

**THE ASK IS ONE RULING OVER SEVEN THINGS, NOT SEVEN ASKS.** Retire, re-point, or tell me the subject is less retired than measured. **I am not minting a disposition myself, and my seven are a FLOOR** -- 57 of 127 paths were never probed.

**UNBLOCKED AND FILED, WAITING ONLY ON WHERE IT SITS RELATIVE TO THE CUT:** `int local status` reports what is running and what the store weighs. Two requirements are IN the mechanism, not in a note: count by **executable**, never command-line text ([[W138]]); and report **store holders separately from process count**, because those cost writes and the two came apart today at 13 processes / 4 holders.

**BLOCKED ON hv, BY NAME:** `AC-12.1` + `AC-00.6` (one bats-suite/CI ruling -- and `AC-12.1` class (2) has a member the row does not name, `intent/plugins/agents/bin/intent_agents`); `AC-04.6` (a sixth ratified machine); `AC-00.16` (a criterion amendment -- its first conjunct is not machine-decidable as written and the instrument refuses to write its own denominator); `0267`'s strong form; `AC-11.7`'s wording.

**BLOCKED ON THE CUT:** `AC-07.7`, `AC-11.1`, `AC-11.4`, `AC-12.4`. **And the cut is behind THE PUSH, which is the single item that unblocks the most of this column -- four rows against the bats ruling's two.** I described this column as parked on the bats ruling for hours and that was imprecise; corrected to vc 2026-09-10.

**RECORDED, NOT MINE TO BUILD:** `0237`, `0271`, `0267`'s second half, `0298` (ic filed it citing my D-DELETE class), `0299` (mine, filed, engineering). **`0300` is litter I created with a write-path probe and cannot close, because closing an issue is an update and updates are failing.**

**THE STANDING BOUND ON WP-11, IF IT COMES BACK:** design-and-build only. No tap repo, no `gh release`, no `scripts/release`, no push to either remote. `AC-11.1` and `AC-11.4` are outward and are hv's.

## Holds
