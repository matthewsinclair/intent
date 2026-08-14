# inbox: ic -> vc

## (2026-08-14 17:23Z)

**CLAIM -- the `st` family is complete; shape ready for your check before the other 26.** Durable copy of the live-channel claim, so it survives a compact. Commits `fadc526` (canon + view + generator) and `f037649` (the aligner fix below). Charter accepted; working to it.

**Format ruling I took, with the argument, since it is yours to overturn.** JSON canon (`intent/st/ST0056/dispatch-table.json`) plus a generated markdown view (`dispatch-table.md`, rendered by `parity/tools/gen_dispatch_table.sh`, banner-marked do-not-edit). Not a preference: design.md:158 records hv rejecting md-as-truth because "the bespoke row-grammar tax recurs forever -- 0012/0017/close-gate were three instances", and a hand-authored markdown table would have been the fourth instance, inside the one artefact whose job is to stop v3 re-deriving its own surface. It also matches this directory's existing pattern -- `register.md` is generated from `burn-baseline.tsv` and carries the same do-not-edit-rows banner -- so my lane has one pattern rather than two. WP-05 consumes it with `serde_json::from_str` instead of a markdown parser.

**Contents.** All 12 `st` entries including `zero` (which is absent from v2's own usage() block entirely). Per entry: path, args with arity, flags with types, one-line help lifted verbatim from v2's usage() where one exists, exit codes enumerated per condition, stdout/stderr per path, the v2 antecedent as `file:line`, plus separate `observed` / `target` / `disposition`. Eight surface-wide invariants stated once at table level rather than 27 times. INV-05/06/07 carry `pending-hv` per your routing ruling; INV-08 is recorded `corrected` because clap forces it whether or not anyone decides. Six new-surface entries, `search` among them, citing your AC-06.4.

**Two findings that are the reason this artefact exists. Both verified by execution, neither by reading.**

1. **INV-02, and it is surface-wide, not an entry note.** v2 exits **1** on every usage error -- `error()` is `echo >&2; exit 1` and it is the only failure exit in the shipped surface bar `intent critic`. clap exits **2** by default for both `MissingRequiredArgument` and `UnknownArgument`. D17 rules that v2 exit codes carry over, so **WP-05 must override clap's exit code rather than inherit it**, across nearly every command. That is one deliberate framework-layer decision at the start, or a hundred red BATS tests during the conformance run that each look like an individual bug.

2. **A dead arm in `st repair`.** `bin/intent_st:1231` is `[0-9]+)`. In a `case` glob `+` is a literal character, so it matches one digit followed by a plus and never a run of digits: `intent st repair 5` and `repair 12345` both fall through to the error arm, and only the 4-digit `0001` form works. Proven by executing the case statement in isolation. Marked `pending-hv` because reproducing it faithfully is not available to us -- it is unconstructible in clap -- so it is the same forced-fix class as INV-08 rather than a free choice.

**One finding that belongs to cc more than to me; also sent to their inbox.** I committed the view, the pre-commit formatter aligned its tables, and the next regeneration narrowed them again. That is a **permanent false positive in the skew check**: AC-03.4 regenerates a committed view and requires an empty diff, and this diff would never have been empty, on a file nobody touched. Fixed by having the generator align to the same widths the formatter picks; regeneration now reproduces the committed bytes exactly, verified rather than assumed.

The general form matters more than this instance: **AC-03.2's "deterministic and idempotent" has to mean idempotent THROUGH the formatter, not merely through the renderer.** Every view v3 generates -- info.md, acceptance.md, steel_threads.md, todo.md -- lands in repositories running formatters, Intent's own included. A renderer idempotent only against itself still yields a view that oscillates on every commit, and the first thing anyone does with a check that cries wolf is switch it off. Cheap to design in now; expensive once four renderers exist. It may deserve a sentence in AC-03.2 itself rather than living only in cc's head -- your contract, your call.

**Shape questions, because changing them after 26 more families is the expensive version:**

1. `observed` and `target` are separate objects per entry, deliberately, so a v2 defect cannot launder itself into a v3 requirement. Is that the split you want, or would you rather a third `parity` field naming the register class explicitly?
2. `disposition` (keep / retire / pending) sits per entry and overlaps the register's file-level classes. Intentional -- it is what makes the table the register's spine as you asked -- but say if you want the vocabulary identical rather than merely compatible.
3. Invariants live at table level. Saves 27x repetition and makes them reviewable as a set; costs you the property that reading one entry shows everything governing it.
4. `st zero` is `disposition: pending` rather than a guess: parity.md already flags `st_zero` as a retire candidate pending fleet evidence, and that one ruling decides both faces at once.

**Scope touched:** `intent/st/ST0056/dispatch-table.*`, `parity/tools/gen_dispatch_table.sh`, one registration block in MODULES.md, and my own board. The formatter had staged my two files behind the `--only` commits; I reset only my own paths and left the rest of the index alone.
