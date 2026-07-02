# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Intent has a whiteboard (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.14.0 SHIPPED (ST0050 intent todo + ST0051 width)

v2.14.0 shipped (tag `v2.14.0`, release commit `c7842f1`, both remotes + GitHub release); Intent self-upgraded 2.13.1 -> 2.14.0 clean (post-tag wrap `a6f6662`; `intent doctor` green). A minor -- new command surface. **ST0050** -- `intent todo`, a flat DOING/TODO/DONE projection of `intent/st/**` into `intent/todo.md` that cannot drift (checkboxes derived from real `status:`): minimal markdown + keyed-by-bucket `--json`; `done`/`notdone`/`toggle` verbs wrapping `intent st/wp` (inheriting the ST0048 close-gate); `done --flush`/`--prune` + the `## DONE:<T>` sticky watermark; `completed:` upgraded to an ISO 8601 timestamp (legacy `%Y%m%d` tolerated everywhere it is read). Six WPs, closed 23/23; dogfooded (first ISO `completed:` stamp). **ST0051** -- `intent st sync --write` hardcoded generated-file width 80 -> config `dft_width` (default 120) + a Highlander `get_default_width`; stdout keeps terminal width; `--width` overrides. Both in `intent/st/COMPLETED/`. vc independently audited ST0050: PASS -- ship-clean. matts accepted the sticky-watermark model.

### v2.14.1 follow-ups (from the vc audit -- all non-blocking)

1. **AC-01.8 enumeration Highlander** -- `intent todo`'s markdown + JSON emitters each re-walk `intent/st/**` and duplicate the `norm < since` predicate; AC-01.8 over-claims "no second traversal". Unify the enumeration or reword the AC (a weakening -> matts' nod). Field-extraction Highlander is already fine.
2. **AT-name traceability** -- `acceptance.md` AT `::names` don't match the real bats `@test` names; align them.
3. **`intent upgrade` false-no-op + `scripts/release` `confirm()`** -- in-session `./bin/intent upgrade` reported "already at 2.14.0" while config.json was 2.13.1 (a normal `intent upgrade` read 2.13.1 + stamped); confirm `detect_project_version` can't skip the config stamp for a fleet member. And the push `confirm()` reads raw stdin + strict `[yY]`, so a stray End-key escape aborted the first push -- read `/dev/tty` and tolerate stray input.

### Fleet

Members pick up v2.14.0 on their next `intent upgrade` -- the new `intent todo` + the ISO `completed:` stamp (backward-tolerant, no migration). Excludes Pplr, Sites-in-Laksa, llm-tropes.

## v2.14+ backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `scripts/release` v2 polish (config.json bump still a manual post-tag wrap; plus the confirm() hardening above); `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `scripts/release --no-confirm`.
