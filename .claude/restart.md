# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell at HEAD; cc's uncommitted spread adds rust + others.) Whiteboard present (`intent/whiteboard/`, hv+cc+vc+ic) -- `/in-session` chains `/in-whiteboard pickup`. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md` + `intent/restart.md`.**

## State: ST0056 (Intent v3.0.0) underway; WP-01 + WP-02 DONE

**v2.19.0 shipped the morning of 2026-08-14 (tag `071c612`); ST0056 opened the same afternoon.** The architecture is ratified in `intent/st/ST0056/design.md` (D01-D36): schema-as-truth (Rust types generate JSON Schema + DDL + SDL faces, committed and drift-checked), **the intentdb as the DURABLE SSOT with everything on disk a secondary artefact** (D01 REVERSED by hv 2026-08-15 -- "committed JSON canon as durable truth, rebuildable per-project SQLite, `rm` always safe, no DB migrations ever" is false in every clause; do not reason from it), the committed extract as the interchange that travels while the DB never leaves the machine (D34), migrations normal, `rm intent.db` not an operation (D36), md as generated views + authored prose, `intentsvcs` as sole owner of DB and files, CLI dual-mode (in-process or GraphQL to one machine-level intentd), intentd IN the 3.0.0 gate, migration floored at v2.19.0.

**Roles (hv ruling): cc builds, ic runs parity/interface, vc stewards** (contract, WP-close verification, hv interface; holds the ST0056 claim). WP-01 (design canon) and WP-02 (workspace + reified model) are closed through the gate; **WP-03 (ingest, views, sync) is cc's next**. The estate is pushed to both remotes, CI green twice on `736033d` (first rust run 31812129560: macOS+Linux, fmt/clippy/tests; plus Intent Tests). devbin is adopted: `bin/int`, and `bin/release` is now `bin/int build release` (`bin/intent` untouched).

**The consumer-sweep program is DEAD** (Lamplight's hv ruled AT remediation on Done work off; their ~1158 legacy rows are permanent). hv-ruled migration policy in `migration.md`: CLOSED threads convert lossless-by-carrying; LIVE threads stay BLOCKED-until-clean; neither ever lossy. `organize` (both faces) is planned vestigial by construction. v2 maintenance is DEFAULT-DEFER, show-stoppers only.

**A whiteboard clock guard is live in pre-commit** (`ddac6ba` + `98ce764`): commits adding board stamps without a trailing `Z`, postdating the commit, or sending an inbox backwards are REFUSED. Stamp from `date -u`, never rounded up.

## Next

1. **WP-03 (cc)**: strict ingest, deterministic views, sync engine -- read `migration.md` as landed. vc reviews at the close claim.
2. **vc**: spec the marked-legacy AT form in `data-model.md` before WP-08 (the carry policy's model consequence).
3. **ic**: per-test register rows for the 40 `split` files (`corrected` is ratified); charter + roster-row asks still open with hv.
4. **v2 carries (default-defer)**: credo_checks fleet issues (hv running); fleet pushes Utilz `0171297` + Lamplight `7058fd3a8` (re-verify still unpushed first); cc's parked "hv decides" queue.

## Standing lessons (this cycle)

Grep for a Highlander rule, never read for it. Mutation-test every guard -- and the canary must come from the same fixture and branch the test drives (applied is not reached). A migrator must not do half of a two-ended migration. Diagnose by running, not reading; run the real path in a sacrificial copy. Verify the premise of a queued action at the moment you act on it. A record names the commit it covers, never "HEAD"; a measured figure names its subject and revision.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks; **always `git commit --only <paths>`** (a bare commit sweeps a peer's staged index). Whiteboard stamps carry a trailing `Z`. matts runs the full suite externally. matts is the acceptance verifier. NEVER `--no-confirm` on the release. Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/int build release` date them at cut time.
