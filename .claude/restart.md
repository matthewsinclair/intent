# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the UserPromptSubmit gate. (Languages: shell only; no whiteboard in this project.)
2. **Verify the tree.** v2.11.12 is the shipped baseline. **ST0044 is COMPLETE** -- closed through its own acceptance gate, relocated to `intent/st/COMPLETED/ST0044/`. The next thread is **ST0043**.
3. **Read `intent/st/ST0043/info.md` + `intent/st/ST0043/acceptance.md`** -- Architecture-B design + the drafted (pending-ratification) ACs for the upgrade rethink.

## Active: ST0043 -- Rethink `intent upgrade` (targets v2.12.0 minor)

WIP, not started. Architecture-B design in `intent/st/ST0043/info.md`; ACs drafted in `intent/st/ST0043/acceptance.md` -- ratify them open-gate with matts before any code (see `working-with-llms.md` D11). ST0043 owns the upgrade-subsystem deletions deliberately excluded from ST0042, plus the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit. Own session, its own v2.12.0 minor. (ST0044 ships separately as the v2.11.13 patch.)

Run ST0043 through the same five-step ST0044 established: verifier (matts) ratifies ACs -> builder writes red-first ATs -> verifier witnesses RED -> builder builds to green -> repeat. `intent ac` / `intent at` instrument the contract; `intent ac gate ST0043` is the close-gate `st done` will consult.

## ST0044 -- acceptance.md + AC/AT process (COMPLETE, pending release)

Done 2026-06-14, dogfooded on itself with matts as verifier, closed through the close-gate it built (16/16 ACs). `acceptance.md` is now a default steel-thread doc; `intent ac` / `intent at` + the opt-in / legacy-safe close-gate are live; the five-step is documented in `working-with-llms.md` D11 with pointers in `/in-plan` / `/in-verify` / `/in-finish`. Full detail: `intent/st/COMPLETED/ST0044/`; ledger `intent/done.md`; narrative `intent/history/v2.11.13.md`.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` (repo convention) only when matts asks. User runs the full suite externally -- single-file bats runs are fine. matts is the acceptance verifier (ratifies ACs, witnesses RED, signs off non-test ACs). An AT's cited `path::name` must match the real `@test` name; `intent at` status-set wipes any trailing note (status token stays first), so lap detail goes on the WP Coverage line.
