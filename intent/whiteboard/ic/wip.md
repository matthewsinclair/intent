---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15T08:31Z
status: paused
focus: "Nothing owed. Five gates PASS; register + pertest complete at c60cdbd; deviate class empty after hv retired treeindex. Two items sit with others: AC-03.4 with vc, provenance-check wiring with hv."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**Nothing is owed by this node.** Both open items are with someone else, and neither is this node's to advance unilaterally.

1. **AC-03.4's skew check is UNDER VC REVIEW.** hv routed it there rather than answering it. Everything vc needs is in their inbox (08:29Z): the artefact is in sync so nothing is broken; the argument is the `f0d6e64` incident; **two of the three conditions already exist** (AC-03.2 requires idempotence through the formatter, `lib_mdfmt.sh` makes the render a fixed point), so a regenerate-and-diff will not cry wolf. Only wiring and an owner are missing. **Do not build it while it is under review.** Wire whatever vc rules.
2. **`provenance_check.sh` wiring is with hv.** vc ruled the home -- **pre-commit, not `doctor`, and not that night**. Reason on the record so nobody re-derives it: the failure is that a split provenance LANDS, and a report only helps if someone runs and reads it. The clock guard is the precedent and it answers the bypass objection -- it fires only on what the current commit touches, so a two-step regeneration stays legitimate provided both artefacts land together, and **a commit landing one alone IS the failure, not the workflow.** Tool is standalone and green today.

### State at fold

    gates      WP-01 4/4 · WP-02 5/5 · WP-03 8/8 · WP-05 4/4
    register   98 rows @ c60cdbd -- keep 31, pending 40, out-of-scope 20, retire 7, deviate 0
    pertest    487 rows / 40 files @ c60cdbd -- --verify 249 verified, 0 stale, 0 unverifiable
    checks     drift, provenance, both formatters clean
    remotes    local + upstream both current

**The `deviate` class is EMPTY and will not stay that way.** It held one file; hv retired treeindex, so it retired too. cc notes `output_width.bats`'s sixth test is already a deviation in waiting. When one appears, `parity.md:32` requires a D-number ratified in design.md BEFORE the port lands, and the register's `ratification` column is where it goes -- `RATIFICATIONS` in `lib_classify.sh` is empty _as the answer_, not as an omission.

**Retired commands are PRESENT AND REFUSING, not absent** (cc's ruling, their lane). The treeindex row stays exactly as landed at `0434223` -- `disposition: retire`, entry present. cc makes the binary and `dispatch_ssot.rs` agree with it; do not work around the guard at the table end.

## Watch-outs

- **ic cannot certify a green suite.** matts owns the authoritative run. Everything from this node is evidence; label it that way.
- **Read `bin/**`, never mutate it** -- `~/.local/bin/intent` symlinks into this repo and every project on the machine runs whatever state those files are in. Sacrificial worktree for anything that writes. `crates/**` is safe.
- **A RULE THAT DEPENDS ON ITS AUTHOR REMEMBERING IT AT THE MOMENT OF USE IS NOT A CONTROL, IT IS A HOPE WITH GOOD PHRASING** (vc; now `parity.md`'s twelfth measurement rule, with cc's compression: _a control refuses; documentation reminds; only one is load-bearing_). Proven the hard way -- in one night three nodes broke three rules **while enforcing them**: cc read a corpus through `| head` with `| head` already on their own board; vc fabricated four timestamps while writing the clock rule; ic reintroduced a provenance split an hour after disproving it. The only two things that held both REFUSED: the clock guard, and `lib_corpus.sh`.
- **Read `date -u` FIRST, then write the stamp.** Recurred 2026-08-15 in the very file carrying this warning -- `01:15Z` typed while the clock said `01:14Z`, from composing the line before reading. **Check A's 120s jitter would have let it through.** Never compose surrounding text first, and never correct a bad stamp from memory.
- **A guard verified in one harness is not verified -- it is verified in THAT harness.** `corpus_require` was green under `set -uo pipefail` and DEAD under `set -euo pipefail`, exiting 1 with EMPTY stderr against a baseline four files short.
- **A grep cannot tell a call site from a string being searched for.** Three times in `lib_classify.sh` alone. Every needle now carries a complement case asking what it must NOT match; `classify_calibrate` runs 11 before either generator will classify anything.
- **Quoting layers are where a needle stops matching without saying so, and the tell is that the ERROR catches it, not review.** Mine: unescaped backticks in the double-quoted `OVERRIDES` string ran `retire` as a command. cc's: `\n\n` in a `python -c` string becoming real newlines and silently voiding a mutation.
- **A result that is right by COINCIDENCE is worse than a wrong one, because it certifies the method.** cc's `git remote -v | head -4` was complete -- two remotes times two lines is exactly four. Enumerate with bare `git remote`; never `head`.
- **`git commit --only <paths>` does NOT protect a file two nodes both edit.** It scopes to paths and takes whatever is in the working tree there. For genuinely shared files (MODULES.md), land the row in the same commit that creates the module.
- **The two obvious sources for a command surface both lie.** The surface is files on disk (`bin/intent`'s `*)` default), not case arms; `bin/intent_help` hand-maintains its list behind a skip list. Enumerate and run; never read and transcribe.
- **This shell is zsh**: command-prefix assignments evaluate left to right, so `A="$A/x" B="$A/y"` gives B the already-reassigned A. Bash does not.

## Decisions -- standing

Working decisions live in the artefacts that carry them; a second copy here is the drift Highlander exists to stop. See `.history/20260815/` for this session's, each with the file that now holds it. Three remain because they govern how this node behaves rather than what any file says:

- **Read the other boards before you speak.** Two of three asks to hv were already on vc's agenda. Costs one command.
- **Audit yourself before you confess, and check the audit with the same rigour either way.** Under pressure the reflex is to confess first; mine looked wrong and were not. A false admission is fabrication too.
- **The convenient answer is the one that needs checking hardest, because nothing else will check it for you** (vc, who went to reclassify a row to `retire` -- which would have dissolved both their question and mine -- and refused it on provenance instead).

## Open asks for hv

1. **The usage-convention scope ruling** -- still the one question that clears 15 pending dispatch-table rows. Observed and recorded per entry: 45 stderr-only / 12 stdout-only / 2 both; `--help` failing on 10 of 27; three commands taking unknown flags at exit 0. Targets blank and marked `pending-hv` -- a blank marked pending is honest, a guess is not. **INV-07 shrank by one**: treeindex's instance is moot now the command retires.
2. **`intent critic` overloads exit 2 four ways** -- findings-present (INV-04), bare invocation, unknown flag, bad positional; the unknown-flag path leaks grep's own error as the command's voice. The only pending item with a LIVE consumer: the pre-commit gate reads this exit code, so "findings" and "you typed it wrong" are indistinguishable to it today.
3. **Wire `provenance_check.sh` into pre-commit** -- vc ruled the home and deferred the wiring; the argument is in DOING above, so this needs an authorisation rather than a re-derivation.
