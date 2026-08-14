---
node: ic
name: Interface Claude
role: interface
session_id: 0482e68a-709f-45b1-ab98-44bc9c962bd1
heartbeat_at: 2026-08-14T13:59Z
status: paused
focus: "Released after the localfold. ST0056 parity deep pass delivered and handed to vc; three hv rulings outstanding, to be put to hv in plain language on the bounce."
claims: []
---

# Interface Claude (ic)

## DOING

- **Nothing in flight.** Released at the localfold, 2026-08-14. The parity deep pass is delivered, committed and handed over (`vc/inbox.ic.md` 13:19, `cc/inbox.ic.md` 13:20). Session detail archived to `.history/20260814/`.

## TODO

1. **Put the three asks to hv in plain language** -- the first thing on resume. hv wants to ratify them, not read them again; they are stated in "Open asks for hv" below.
2. **Re-run `burn.sh` to fold in the 95th test file.** `e685e90` added `tests/unit/at_lint_wp_scope.bats` after my baseline. Verified green and burning 5/5 in the main tree, but the register's 94 rows are the 94 files at `69d42a7` and it has no row. Cheap; do it before anyone quotes the register as complete.
3. **Per-test rows for the 40 `split` files** -- the second pass, blocked on vc ruling the `corrected` class. Deliberately not guessed in pass one.

## Open asks for hv

Stated here so they survive a compact and can be put in plain language without re-deriving them.

1. **What is ic's charter?** Proposed: the dispatch-table SSOT and everything rendered from it -- command surface, help, voice and exit codes, MCP tool list, and the `intent llm` agent guide. That maps onto WP-05's SSOT half, WP-06's register and WP-09 entirely. Consequence if unratified: ic keeps taking one-off errands and nobody owns the interface as a whole.
2. **The whiteboard roster row.** `intent/whiteboard/README.md:13` still reads "Intent is CLI plus data, not UX, so there is deliberately no interface node; the roster is three." False since ic was scaffolded. It is hv's file and hv's scope call, so ic has not edited it. Same stale-record class the project keeps paying for.
3. **The usage-convention ruling.** Does v3 reproduce v2's failure voice faithfully, or correct it? The census now exists to rule on: 45 stderr-only, 12 stdout-only, 2 both; `--help` failing on 10 of 27 commands; three commands accepting unknown flags with exit 0. Bundles naturally with the stdout-error item already in cc's queue. **Blocking**: the parity contract cannot be finished without it, and clap forces the unknown-flag half whether or not anyone decides.

## Watch-outs

- **ic writes into `intent/st/ST0056/**` only where vc directs it.** vc owns the thread and sent both deliverables to `parity/`. The claims rule governs who _decides_ a thread's shape, not who may ever write a file into its tree -- ic's first-pickup reading of it was wrong and is corrected in `.history/20260814/wip.md`. Editing `acceptance.md` or WP files remains vc's, not ic's.
- **ic cannot certify a green suite.** matts runs the full suite externally; ic runs single-file bats and worktree sweeps. Anything ic reports as green is evidence, never certification, and must be labelled that way when handed to a peer.
- **Announce before touching shared test infrastructure.** The BATS estate is shared -- cc's pre-flight and `bin/release` both run it. Done for `87a315b`; the rule stands for the next sweep.
- **Read `bin/**`, never mutate it.** `~/.local/bin/intent` symlinks into this repo, so every project on this machine runs whatever state those files are in. Running commands to capture output is fine; anything that modifies a binary goes in a sacrificial worktree. Note `intent claude rules index` mutates `INTENT_HOME` (it rewrote `rules/index.json` in the worktree under test) -- it reads like a query and is not one.
- **The two obvious sources for a command surface both lie.** `bin/intent`'s dispatch has explicit arms for 8 commands and a `*)` default mapping `intent <foo>` to `bin/intent_<foo>`, so the surface is files on disk, not case arms. `bin/intent_help` hand-maintains its lists behind a skip list (`bin/intent_help:93`) and still describes `upgrade` as "Upgrade from STP to Intent v2.1.0" (`bin/intent_help:71`) at v2.19.0. Enumerate and run; do not read and transcribe.
- **`INTENT_HOME` is inherited, and `bin/intent:12` only self-resolves it when unset.** Any probe or harness that does not pass it explicitly silently measures the developer's live tree instead of the worktree under test. Cost the first probe run.

- **Vendoring into `bin/` has a blast radius the gates cannot see.** Seven bats files scan `bin/` broadly rather than by name, and `set_e_increment_guard.bats:14` greps it **recursively**; cc's devbin landing put 99 more files inside that needle. Verified clean on 2026-08-14 (98 tests, 0 failing), but the exposure is permanent and any future vendoring inherits it.

## Decisions

- (2026-08-14) **The inventory is a generated, revision-stamped artefact, not a written document.** Directly from cc's ruling that a measured figure must name its subject and revision or it is a rumour with a decimal point -- the 314-vs-1639 AT-row error is the scar. The surface gets re-measured after cc's consumer sweeps and again during WP-06; a hand-typed list cannot be diffed and starts decaying the moment it lands.
- (2026-08-14) **Measure what a test EXERCISES by redirecting the thing under test, not by reading its assertions.** The burn ratio -- run twice, once with `INTENT_BIN=/usr/bin/false`, and diff the failures -- classified 94 files mechanically and twice found what reading could not: `treeindex_commands.bats` reads as 53 CLI tests and is 53 tests bypassing the dispatcher, and `claude_with_intent.bats` reported zero burn while looking like a CLI test because it aliased the binary through a spelling four grep passes had missed. **The mutation check is not only a guard on the harness; it is the classifier.**
- (2026-08-14) **My own tooling failed four times in one session, in the exact shapes this project keeps recording, and every one was silent.** A `BASH_SOURCE`-derived path under zsh made 20 probes fail identically in `cd` and report a uniform rc=1 surface that read like data. An invalid ERE made 13 scripts report "no dispatch found" -- caught only because the error was not swallowed. `set -e` killed a sweep at the verification grep that SUCCEEDS by matching nothing, leaving the estate half-rewritten. A `printf` format beginning with `-` was parsed as an option and silently dropped a line from 26 generated files. **Instrument the instrument: every harness here now refuses loudly rather than emitting a plausible empty surface.**
- (2026-08-14) **A test the classifier cannot bucket is named, never guessed.** The refuse-lossy discipline from `at lint --fix` applies to classification as much as to migration: a wrong keep/retire call is silent lost coverage in v3, which is the same defect the AT grammar existed to kill.
- (2026-08-14) **A generated filename can collide with a reserved one.** A generated `claude.md` was loaded as this directory's `CLAUDE.md` on a case-insensitive filesystem and injected into the session as instructions; `agents.md` shadows `AGENTS.md` identically. Fixed with a `cmd-` prefix. Carries into v3, whose view renderer generates far more files than this pass did.
