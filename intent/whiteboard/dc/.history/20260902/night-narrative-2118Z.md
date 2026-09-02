# dc, night session 2026-09-02 -- narrative kept out of the board

Companion to `wip-prefold-2118Z.md` (sha `707b6622`, 75,510 bytes). The board carries what a peer needs at pickup; the reasoning lives here.

## The evening had one shape and it was not the one I expected

**Three of the last four items I was given died on their own premise.** Not on the work -- on the sentence the work was scoped against.

- **`0181`** asked the ratchet's single question, _did the capability move or was it never built_, and the answer was **neither**: `llm usage_rules --symlink` was built and does not do what its help says, in two of three arms.
- **`2(D)`** was scoped against the word `committed` in a header, where it named an artefact CLASS (extract, not store) and every later reader took it as a claim about a git read. Four carriers, one ambiguity, nobody wrong at any step -- and I had written it into the tool's own source that morning.
- **`0203`** says v2 answers `<family> help` on nine families. It answers on six, refuses three, and answers cleanly on three.

vc's general form is better than my three instances: **a row's premise is written once and re-read forever, and the remedy is the first thing that ever re-drives it.** Reading a row is not testing it. A filing is a measurement with no expiry date on it.

## The instrument is not the claim, five times, in five unrelated places

This is the thread that actually ran through the night.

1. **`--workspace` against `--all-targets`** -- exhaustive matches live in test targets a plain check does not build.
2. **`78 rows`** -- a `grep -c` LINE count reported as a ROW count, committed by me **inside the sentence correcting vc's field choice**, one commit after I applied that same rule to three other people's rows.
3. **vc's `82/19`** -- a bare-token grep counting a retired field's OBITUARY as its use, and then a comparison of a LINE count against an OCCURRENCE count of the same pattern, read as disagreement. **vc diagnosed it as a tool difference; I reproduced their numbers exactly on the same ugrep, so the tool was innocent.**
4. **`0208`'s census** -- counted the `wb()` prefix on the SUBJECT LINE and concluded identity was absent from 175 commits, while a second identity sat in the BODY of 312 of 313.
5. **`0217`** -- the parity apparatus only ever walks the TABLE, so nine `help` verbs present in the binary and absent from the table are outside its population by construction, and every instrument stayed green.

**And the fourth layer: vc's diagnosis OF the instrument-versus-claim class was itself an instance of it.** That was only visible because I re-ran their command instead of accepting the explanation.

## Three environmental reds, all reportable as findings

- `cargo fmt --manifest-path <x> --check` answers _Failed to find targets_ and exits 1.
- A relative binary path after a `cd` gives rc=127.
- 29 intent-cli tests failed with _no `intentd` was found beside this `intent` binary_ -- I had built only `--bin intent` into my private target dir.

**A red about the environment is indistinguishable from a red about the subject.** Each of the three would have been reportable, and the third had a plausible story attached (I had just changed clap's subcommand handling; daemon tests failing looked like mine).

## `0203` -- the checks were the whole job

The two checks vc and I insisted on, and which I refused to call a one-liner, were the entire value of the item.

**Clearing `disable_help_subcommand` gives clap two `help` subcommands. Debug PANICS on every invocation including `--help`; release returns rc=0 and WORKS**, because `debug_assert` is compiled out. **Loud in the profile nobody ships, silent in the profile everybody runs** -- and verifying against the shipped binary, the natural move, would have reported success.

Then two more that only driving could find: **clap generates its own family `help` even with the root disabled**, so the explicit verb duplicated it; and **a child pulled from a freshly built tree renders the wrong usage line and silently drops every global flag** until `root.build()` is called. The output was plausible and wrong until a byte-comparison said so.

## `0207`'s whole-suite ruling caught something it was not aimed at

vc ruled the re-run against `shorter` vs `not-contains` over 313 notes. It surfaced the `--symlink` retire's third half instead. **A precaution taken against one risk catching a different one is what a precaution IS**; if it only ever caught its target it would be a targeted check and would not need to stand.

## The retire had four halves and I had found one

`disposition: retire` was mine. **`gen_dispatch_table.sh` refused the row left `mutate` with its only writer withdrawn; `write_moves_only_what_changed.rs` refused an UNPROVEN roster still naming it; the compiled-in vocabulary check refused a `read` row declaring `recoverability`.** Three instruments, three halves, none looking for the others. **`0210` says a landing is one commit. It does not say how many halves a landing has, and these were not enumerable by reading.**

## The census, and what a negative result is worth

The `v2:` census closed at **one member**. The part that makes it a result rather than a small number is the **positive control**: `usage-rules.md` had to come out of the instrument or the instrument was discarded. And the sharpest correction inside it was distinguishing **candidates by shape from candidates by mechanism** -- `intent_help` expands `$INTENT_HOME/README.md` inside a heredoc, as text, and a printed path cannot pick the wrong file.

**`no_intent_home.rs` had named the shape eleven days earlier**: _every developer machine here would still be green, because every developer machine here has it set._

## Held, and why it mattered

hv said _boot, then hold_. vc ruled three writes in the same minutes. **A peer ruling is not the human word**, and vc said so themselves without being asked. The corollary I had to correct for on my own board: **I recorded the held queue as three items when it was five**, and a short count on a board is the same class as everything above.

vc's own version, recorded against themselves: **a router must track ruled-and-unexecuted separately from ruled, or the ruling's own record reads as its completion.**
