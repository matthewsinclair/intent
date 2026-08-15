---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 08:56Z
status: active
focus: "view_skew_check.sh built and green (d470f62); measuring vc's precondition showed register.md is NOT re-derivable either, so TWO artefacts rest on their stamp alone. Both guards unwired and handed to dc, who came online owning pre-commit."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**Nothing is owed by this node.** All open items are with someone else, and none is this node's to advance unilaterally.

0. **`st_zero`'s VERB is with hv; do not act until it lands.** cc is relaying. hv ruled the SHAPE (it rehomes under `st`, the underscore goes -- it is the only one in the command surface). Only the verb is open, and both dispatch-table rows are held at `pending` at `f11b357` with the full evidence written into the canon so the ruling lands on something rather than being re-derived. The fact that decides it: **`intent st zero` ALREADY EXISTS** (`bin/intent_st:1610-1612`) and is **the only spelling the command's own usage documents** -- so this is a DELETION of the root face, not a rename, and the divergence on the documented face is zero. Choice is `st zero` (incumbent, zero divergence) vs `st initzero` (reads better -- `st`'s subcommands are verbs, `zero` is a noun -- but a coinage, and it buys a divergence row). **The retire question went MOOT, not answered**, same as INV-07 on treeindex; `parity.md:69` still carries the stale flag and vc has been told, as it is their file.
1. **TWO GUARDS ARE BUILT, GREEN, AND UNWIRED -- and the wiring is now `dc`'s lane, not mine.** `provenance_check.sh` and `view_skew_check.sh` (`d470f62`). **`dc` (DevX Claude) came online 2026-08-15 owning dev-x / build / git**, which is exactly where pre-commit lives; full handoff written to `dc/inbox.ic.md` at 08:55Z. It is **one authorisation, not two**, and it still needs hv -- dc takes it there; a peer's inbox note is not approval. vc ruled the home for both: **pre-commit, not `doctor`**. The reason, so nobody re-derives it: the failure is that a bad artefact LANDS, and a report only helps if someone runs and reads it. The clock guard is the precedent and answers the bypass objection -- it fires only on what the current commit touches, so a two-step regeneration stays legitimate provided both artefacts land together, and **a commit landing one alone IS the failure, not the workflow.** The skew check is path-triggered for the same reason a slow gate gets `--no-verify`d.

   **The argument to put to hv is vc's, and it is stronger than mine was:** `pertest.md` AND `register.md` cannot be re-derived from committed state by anything, at any price short of a full re-sweep -- `gen_pertest.sh` needs the ephemeral TAP, `gen_register.sh` needs the raw `burn.tsv` (tracked nowhere, not on disk) plus a detached worktree at the measured revision. **For both, the stamp is the only guard that exists, and the stamp check is the unwired one.**

2. **AC-03.4 is RULED AND CLOSED -- do not reopen it.** vc's ruling: not an AC, and not folded into `provenance_check.sh`. Both refusals right. Widening a product AC to cover apparatus would let AC-03.4 go red for reasons that say nothing about whether v3's skew check works -- the AC-05.3 error in another costume. And merging the two invariants puts stamps-agree and content-matches-canon behind one exit code, which is the `intent critic` exit-2 overload **I had filed against the old apparatus and then proposed reproducing in the new.**

### State at fold

    gates      WP-01 4/4 · WP-02 5/5 · WP-03 8/8 · WP-05 4/4
    register   98 rows @ c60cdbd -- keep 31, pending 40, out-of-scope 20, retire 7, deviate 0
    pertest    487 rows / 40 files @ c60cdbd -- --verify 249 verified, 0 stale, 0 unverifiable
    checks     drift ok/26 families, provenance one-rev-per-group, render a fixed point
    tree       Rust now at native/rust/crates/ (hv: native/{platform}/, macos reserved)

**The move cost my lane 6 prose references and nothing else** -- measured, not assumed: `parity/tools/*.sh` zero, `register.md`/`pertest.md` zero, because the burn corpus is `tests/**` and that did not move. The register and pertest are untouched by the reorganisation and did not need regenerating.

**The `deviate` class is EMPTY and will not stay that way.** It held one file; hv retired treeindex, so it retired too. cc notes `output_width.bats`'s sixth test is already a deviation in waiting. When one appears, `parity.md:32` requires a D-number ratified in design.md BEFORE the port lands, and the register's `ratification` column is where it goes -- `RATIFICATIONS` in `lib_classify.sh` is empty _as the answer_, not as an omission.

**Retired commands are PRESENT AND REFUSING, not absent** (cc's ruling, their lane). The treeindex row stays exactly as landed at `0434223` -- `disposition: retire`, entry present. cc makes the binary and `dispatch_ssot.rs` agree with it; do not work around the guard at the table end.

## Watch-outs

- **ic cannot certify a green suite.** matts owns the authoritative run. Everything from this node is evidence; label it that way.
- **Read `bin/**`, never mutate it** -- `~/.local/bin/intent` symlinks into this repo and every project on the machine runs whatever state those files are in. Sacrificial worktree for anything that writes. **`native/**` is the safe tree** (was `crates/**` until hv's 2026-08-15 reorganisation).
- **A RULE THAT DEPENDS ON ITS AUTHOR REMEMBERING IT AT THE MOMENT OF USE IS NOT A CONTROL, IT IS A HOPE WITH GOOD PHRASING** (vc; now `parity.md`'s twelfth measurement rule, with cc's compression: _a control refuses; documentation reminds; only one is load-bearing_). Proven the hard way -- in one night three nodes broke three rules **while enforcing them**: cc read a corpus through `| head` with `| head` already on their own board; vc fabricated four timestamps while writing the clock rule; ic reintroduced a provenance split an hour after disproving it. The only two things that held both REFUSED: the clock guard, and `lib_corpus.sh`.
- **Read `date -u` FIRST, then write the stamp.** Recurred 2026-08-15 in the very file carrying this warning -- `01:15Z` typed while the clock said `01:14Z`, from composing the line before reading. **Check A's 120s jitter would have let it through.** Never compose surrounding text first, and never correct a bad stamp from memory.
- **A VERIFICATION IS ONLY AS CURRENT AS THE THING IT READ, AND NOTHING TELLS YOU WHEN THAT EXPIRES.** Three instances in one morning, one per node. ic verified both Rust paths present on disk and committed the table -- they WERE present, and the tree moved again minutes later. cc's `native/rust/target/` held 1.2G compiled against the old `CARGO_MANIFEST_DIR` that cargo's fingerprint called FRESH, so `dep_graph_guard` passed alone and failed in the suite. Both are honest greens describing a world that had already moved. The fix is not more care; it is `gen_dispatch_table.sh` refusing to render a canon path that does not resolve. Put to vc as a candidate thirteenth measurement rule -- **their call, not filed unilaterally**, because two of the three were caught by existing mechanisms and it may be a restatement of "stamp what you measured".
- **A guard verified in one harness is not verified -- it is verified in THAT harness.** `corpus_require` was green under `set -uo pipefail` and DEAD under `set -euo pipefail`, exiting 1 with EMPTY stderr against a baseline four files short. **RECURRED 2026-08-15, written by this node with the warning on this board**: `X="$(grep ... | sort -u)"` in the new path check aborted the whole generator under `set -euo pipefail`, because grep exits 1 on no-match -- exit 1, empty stderr, no view, no explanation. **Only the zero-match mutation found it; reading never would.** Every pipeline whose emptiness is legitimate needs `|| true`, and that is now a comment in the file explaining why it is load-bearing rather than defensive noise.
- **A CAPABILITY CHECK THAT INSPECTS RATHER THAN EXERCISES IS NOT A CHECK.** "Does the generator honour `OUT`?" answered by grepping for an `OUT` variable said YES for `gen_register.sh`, which cannot be round-tripped at all -- it also needs `SP` (raw `burn.tsv`, tracked nowhere, not on disk) and `WT` (a detached worktree at the measured revision). Redirecting `OUT` dies at `SP: parameter null or not set`. **The test for "can this be re-derived" is regenerating it.** Same shape as a `Greppable proxy` the headless runner cannot honour -- ST0039 territory.
- **Before building a needle, COUNT WHAT IT WOULD MATCH.** The skew backstop was going to sniff the `GENERATED VIEW` banner. Of the 30 apparatus views **exactly one** carries a banner -- `register.md`, `pertest.md` and all 26 `cmd-*.md` have none. It would have covered one file and reported full coverage. Enumerate the directory and demand every member be classified; an unregistered view then cannot hide.
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
