# Claude Code Session Restart -- narrative state

## Current state (2026-08-14)

**ST0056 -- Intent v3.0.0 -- is the live work.** Same-day sequence: v2.19.0 shipped in the morning (tag `071c612`, fifteen issues 0009-0023, narrative `intent/history/v2.19.0.md`); in the afternoon hv opened ST0056 and the architecture was ratified in a rubber-duck session with vc. Everything decided lives in `intent/st/ST0056/design.md` -- read it before touching anything v3. The shape: a reified schema-as-truth model (the intentsvcs Rust type layer generates JSON Schema + SQL DDL + GraphQL SDL faces, all committed and drift-checked), committed JSON as durable truth, a rebuildable per-project SQLite DB as runtime truth (`rm intent.db` always safe, no DB migrations ever), markdown demoted to generated views + authored prose under the authored-once principle (no mixed files), strict validate-or-refuse ingest, `intentsvcs` as sole owner of DB and file canon with the CLI dual-mode (in-process facade calls, or GraphQL to one machine-level intentd serving N projects), MCP as the primary agent write surface, migration floored at v2.19.0 (two-hop; the v2 ledger is never reimplemented), Homebrew via cargo-dist as a core deliverable, intentd IN the 3.0.0 gate (hv ruling, one major release + 3.0.x patches). Prior art: Lamplight `native/cli` (dispatch-spine SSOT, typed errors with remedies, MCP bridge) and Conflab `native/daemon` (conflabd is nearly the intentd stack: async-graphql + axum, rmcp streamable HTTP, CLI-owned launchd lifecycle, mgmt plane, debounced watching, policy-stamp self-healing). Cloud seams ship in 3.0.0 without cloud code: project_id UUID, principal on every facade call, append-only event log, reserved server config block.

**WP-01 and WP-02 are DONE (gates 4/4 and 5/5); WP-03 (ingest, views, sync) is cc's next.** The full contract stands at 62 ACs / 60 AT rows, lint-clean, with the WP-01 specs beside design.md: `data-model.md`, `migration.md`, `parity.md`. **hv roles ruling: cc and ic write the code; vc stewards** (contract, verification at WP closes, hv interface) and holds the ST0056 claim. WP-02 closed on cc's claim after the AC-02.6 renumber to AC-04.5 (the envelope test cannot exist before WP-04's facade verbs; provenance on both rows and in WP-04's info). **The v3 estate is PUSHED to both remotes with CI green twice on `736033d`**: the first rust workflow run (31812129560 -- macOS+Linux, fmt --check + clippy -D warnings + tests, 1m47s) and the BATS Intent Tests; AC-02.1 is satisfied by that named evidence. devbin is adopted (`bin/int`; `bin/release` is now `bin/int build release`; `bin/intent` untouched by design; suite 1240 green at `3563ff4`). ic delivered the parity deep pass (26 `parity/cmd-*.md` files, the `INTENT_BIN` retarget -- 711/1235 tests reach the CLI -- and the register regenerated at `393a8e1`), then landed the **whiteboard clock guard** (`ddac6ba` + the `Re:`-anchor fix `98ce764`): the pre-commit chain now REFUSES commits adding board stamps without a trailing `Z`, postdating the commit, or sending an inbox backwards. 0024 (scoped `at lint`/`ac gate` dropping the WP scope) is closed at `e685e90` and vc-reviewed sound.

**The sweep program is dead, and that reshaped WP-10**: Lamplight is already at 2.19.0 and their hv ruled AT remediation on Done work off outright -- their ~1158 legacy-grammar rows are permanent. tasks.md and migration.md say the corpus is the fleet AS IT IS, and the policy is now RULED in migration.md: **closed-thread lossless-by-carrying** (legacy rows carried whole, marked legacy, nothing guessed); live threads keep BLOCKED-until-clean; neither class ever gets a lossy path.

## Next (post-bounce; all five rulings landed)

1. **WP-03 (cc)**: ingest, views, sync engine -- reading migration.md as landed. vc reviews at the close claim.
2. **Spec the marked-legacy AT form in data-model.md before WP-08** (the carry policy's named model consequence -- raw v2 reference carried verbatim beside the parsed fields).
3. **ic next session**: per-test rows for the 40 split files (`corrected` is RATIFIED); the charter + roster-row asks are still open with hv.
4. **Ruled and standing**: organize (both faces) planned vestigial by construction -- register rows retire, the two-implementation Highlander dissolves; v2 maintenance is DEFAULT-DEFER, show-stoppers only (cc's "hv decides" queue parks).
5. Unchanged v2 carries: credo_checks fleet issues (hv running), the fleet pushes (Utilz `0171297`, Lamplight `7058fd3a8` -- re-verify both are still unpushed before acting). The tree carries cc's uncommitted lang-init spread (config languages + per-lang `RULES-*`/`ARCHITECTURE-*` + AGENTS.md/RULES.md/installed-agents.json) -- cc's lane, left in place.

## Release checklist (carry forward -- v2 cuts, and the spirit carries to v3)

1. Tree clean, suite green. **Clean means ALL of it** -- `intent build release` aborts on anything dirty outside its five sidecars, including another node's whiteboard board.
2. **Write the release docs BEFORE the cut** (`intent/history/<v>.md` + `docs/releases/<v>/RELEASE_NOTES.md`) so the tag carries them. Adopted at v2.19.0.
3. `bin/int build release --minor` -- interactive, NEVER `--no-confirm`. Pre-flight re-runs doctor + the full suite (not behind the dry-run guard), stamps five sidecars, dates the CHANGELOG, tags, pushes both remotes, publishes the CHANGELOG section as the release body.
   3a. If it aborts after the sidecar commit, the documented recovery is `--skip-tests` -- which skips the ONE gate certifying HEAD. A recorded green is cheap while redundant and expensive at the single moment it is not.
4. Post-cut: flip `intent/done.md`, verify sidecars/tag/release body, globalfold.

## Standing lessons (carry forward)

- **Grep for a Highlander rule; do not read for it.** A guard scoped to what is already clean certifies the status quo.
- **Mutation-test every guard before believing it** -- and the mutation harness itself can lie: a substitution that silently matches nothing skips its restore; hard-fail on unchanged source.
- **A migrator must not do half of a two-ended migration.** Refuse + name everything beats guessing. (Now D-level canon in ST0056's migration spec.)
- **Diagnose by running, not reading; run the real path in a sacrificial copy.** `~/.local/bin/intent` symlinks into this repo -- never mutate `bin/**` in place.
- **A measured figure must name its subject and revision** (Lamplight `15dbccc92`, suite-green at `2769c40`) or it is a rumour with a decimal point.
- **A line number in a durable record is a fact with an expiry date** -- anchor on comment strings or symbols.

## Where detail lives

- ST0056: `design.md` (architecture + D01-D17), `tasks.md` (ladder + dependencies), `acceptance.md` (the gate), WP info files (per-WP work orders), `impl.md` (as-built, empty by design).
- v2.19.0: `intent/history/v2.19.0.md`; CHANGELOG `[2.19.0]`; `docs/releases/2.19.0/RELEASE_NOTES.md`; `intent/issues/CLOSED/0009..0023`; whiteboard `.history/20260814/` per node.
- Guards worth knowing: `at_grammar_lint.bats`, `ac_offscope_states.bats`, `st_enumeration.bats`, `objective_placeholder.bats`, `no_absolute_home_paths.bats`, `output_width.bats`, `helpers.bats`, `st_commands.bats`, `st_list_all_vocabulary.bats`, `credo_checks_residue.bats`, `no_template_fallback.bats`.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full suite externally (single-file bats fine); matts is the acceptance verifier; never `--no-confirm` on the release; author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `intent build release` date them at cut time. 2-space indentation in all code including Rust (house rule overrides rustfmt defaults -- decide the rustfmt.toml in WP-02).
