---
verblock: "14 Aug 2026:v0.19: vc - 0020 folded in; v2.19.0 docs complete, cut pending"
intent_version: 2.18.0
---

NOTE: This file is the terse DONE ledger, newest first. Older entries roll into `./history/YYYYMM-done.md` month-by-month; verbose per-release narratives live at `./history/<version>.md`. DOING/TODO work lives in `./wip.md`.

# Done

## 2026-08-13/14 — v2.19.0 built + verified (issues 0009-0020); cut pending

- **Twelve issues closed end to end in one arc, no ST (standing fix-under-issue ruling), full suite GREEN at HEAD.** vc triaged all nine filed issues against HEAD with mechanical repros and wrote the work order; cc executed seven units; vc audited each wave; hv ratified four rulings and added two of its own. Two filed root causes were corrected before any fix was built (0014's bare-`and` claim; 0011's false-`Moved` claim), and two of vc's own claims later self-refuted into better fixes — the discipline cut both ways.
- **0017 (high, with 0014 + 0015): the AT row has a grammar and the tool enforces it.** Two anchored arms (test / non-test), every field reader one line over them; `intent at lint` L1-L5; `--fix` migrates only what is unambiguous; the gate honours the grammar from day one; an `at_grammar` ledger step sweeps consumers at upgrade. `--fix` itself was then fixed three times on real-estate evidence: performance + a `find -name` glob-class guessing hazard (`be24f23`), and lossiness — it stripped test names before the id existed in the test, destroying the only link; it now refuses what it cannot migrate without loss (`6f70d4e`, measured by cc@Lamplight). The home estate had been swept with the lossy version: 87 destroyed name links measured and restored verbatim from git into trailing notes (`ee44f63`).
- **0013 (+ hv's withdraw): an AC has four states.** `descope --to` / `rescope`, `withdraw --reason` / `reinstate`; detection by marker, never the `satisfied:` field; reported separately, never dropped from totals; a contract emptied entirely by off-scoping is REFUSED toward `acceptance: exempt`. Every state-changing verb carries its audit payload.
- **0011: one steel-thread enumerator.** `list_st_dirs`/`list_st_dirs_in` in helpers (bucket allowlist, one level, `info.md` required); five hand-rolled copies replaced — the last two found only by the mechanical guard, not by reading (twice). `organize` names a PROBED collision cause, finishes its sweep, exits non-zero; `doctor` flags duplicate ids.
- **0012: the whiteboard header block is NOT YAML, ruled and enforced.** Line-oriented `key: value`, quotes literal; hygiene rejects what actually breaks readers; `ws list` and hygiene read one value through `fm_get`.
- **0016: hooks resolve at runtime.** `intent claude hook <name>` (exec, stdin + exit code untouched); settings.json byte-identical everywhere; the canon engine's `[[INTENT_HOME]]` arm is gone; our own baked path removed; guard asserts no tracked config carries an absolute home path.
- **0009: AGENTS.md prerequisites follow the declared languages** (commands keep probes; node + bats exceptions stated); `intent upgrade` converges AGENTS.md after canon apply (`agents sync --check`).
- **0010: `st done`/`wp done` say so when the Objective was never written** — warn, never block, scoped to the one section; placeholder constants live beside their reader with a drift guard over all four generators.
- **0018 (hv ruling): the treeindex cache stops being project state.** 87 files untracked + ignored; consumers converge via the canon `.gitignore` seam; a tracked consumer cache is REPORTED with the exact `git rm`, never run.
- **0019 (vc, from a self-refuted residual): the canonical thread index actually indexes all threads.** `sync --write` composed the WIP-only default view into it, so the committed index decayed to empty at every release close — born wrong, restored to 55 rows; the index updater's five never-read arguments pruned; the delegation's stderr flows.
- **0020 (hv called it in before the cut, not after): `st list --status all` stops silently discarding what it cannot classify.** The `all` branch walked ten status literals and matched exactly, so a presentation ordering doubled as a membership test — anything outside the ten was dropped, no diagnostic, exit 0 — and 0019 had just made that reach committed state via `sync --write`. Membership now goes through `normalise_status` (the same comparison the branch immediately below already used); the ten literals collapse to the five canonical tokens they were spelling out; unplaceable rows are shown last and named on stderr; exit stays 0 because escalating would break index regeneration on exactly the estates that have the problem. Reproduced against unfixed code in a worktree first (1 row of 3), guard mutation-proven M1-M5. Our estate unaffected — 55 rows for 55 threads — which is why it survived the whole release.
- **Voice: `warning()` speaks the documented lowercase prefix** (`8aba5ab`).
- **Every guard in the release is mutation-checked** — five guards written during the cycle could not fail on first writing (invalid ERE swallowed by `|| true`, substring matches, a decoy at the wrong depth, a vacuous applied-probe) and each was caught by breaking the behaviour and watching for the failure. Suite green (hv-run, 2026-08-14). Nothing pushed; no tag; sidecars still 2.18.0 for `bin/release` to stamp at the cut.

## Older

- 2026-07 rolled to `intent/history/202607-done.md`; 2026-06 to `202606-done.md`; 2026-03 to `202603-done.md`. Verbose per-release narratives: `intent/history/<version>.md`.
