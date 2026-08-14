# Claude Code Session Restart -- narrative state

## Current state (2026-08-14)

**ST0056 -- Intent v3.0.0 -- is the live work.** Same-day sequence: v2.19.0 shipped in the morning (tag `071c612`, fifteen issues 0009-0023, narrative `intent/history/v2.19.0.md`); in the afternoon hv opened ST0056 and the architecture was ratified in a rubber-duck session with vc. Everything decided lives in `intent/st/ST0056/design.md` -- read it before touching anything v3. The shape: a reified schema-as-truth model (the intentsvcs Rust type layer generates JSON Schema + SQL DDL + GraphQL SDL faces, all committed and drift-checked), committed JSON as durable truth, a rebuildable per-project SQLite DB as runtime truth (`rm intent.db` always safe, no DB migrations ever), markdown demoted to generated views + authored prose under the authored-once principle (no mixed files), strict validate-or-refuse ingest, `intentsvcs` as sole owner of DB and file canon with the CLI dual-mode (in-process facade calls, or GraphQL to one machine-level intentd serving N projects), MCP as the primary agent write surface, migration floored at v2.19.0 (two-hop; the v2 ledger is never reimplemented), Homebrew via cargo-dist as a core deliverable, intentd IN the 3.0.0 gate (hv ruling, one major release + 3.0.x patches). Prior art: Lamplight `native/cli` (dispatch-spine SSOT, typed errors with remedies, MCP bridge) and Conflab `native/daemon` (conflabd is nearly the intentd stack: async-graphql + axum, rmcp streamable HTTP, CLI-owned launchd lifecycle, mgmt plane, debounced watching, policy-stamp self-healing). Cloud seams ship in 3.0.0 without cloud code: project_id UUID, principal on every facade call, append-only event log, reserved server config block.

**The 12-WP ladder is cut and WP-01 (design canon) is WIP.** The acceptance contract (`intent/st/ST0056/acceptance.md`) carries the ST-level v3.0.0 gate (AC-00.1..8, lint-clean); WP-02..12 AC/AT groups are a WP-01 deliverable. WP-01's remaining work: data-model spec + first JSON Schema draft, migration spec, the parity contract (v2 command-surface inventory + keep/retire/deviate register), the full-ladder acceptance contract, and four open questions closed as decision-log additions (one binary vs two -- lean two; launchd label; 3.0.0 subscription extent; `.cache` layout). vc drives ST0056 on direct hv assignment (`claims: [ST0056]`).

## Next

1. **hv pre-kickoff check-in**: review design.md + ladder; amend or ratify; then WP-01 completes.
2. **cc's lane, unchanged and now v3-load-bearing**: the consumer sweeps (Lamplight first; baseline `intent/analysis/20260814-lamplight-at-sweep-baseline.md`, 1639 AT rows at `15dbccc92`) are WP-10's fleet-corpus prep -- the migration fixture is the post-sweep trees at named revisions. cc runs post-sweep counts as stop condition; vc re-runs independently as the record.
3. **IC-able now (design-neutral, v2-side)**: the v2 command-surface inventory and the BATS `INTENT_BIN` retarget + per-test classification -- both feed WP-01's parity contract.
4. The rest of `intent/wip.md` Next Up (credo_checks fleet, fleet pushes, hv-ruling queue).

## Release checklist (carry forward -- v2 cuts, and the spirit carries to v3)

1. Tree clean, suite green. **Clean means ALL of it** -- `bin/release` aborts on anything dirty outside its five sidecars, including another node's whiteboard board.
2. **Write the release docs BEFORE the cut** (`intent/history/<v>.md` + `docs/releases/<v>/RELEASE_NOTES.md`) so the tag carries them. Adopted at v2.19.0.
3. `bin/release --minor` -- interactive, NEVER `--no-confirm`. Pre-flight re-runs doctor + the full suite (not behind the dry-run guard), stamps five sidecars, dates the CHANGELOG, tags, pushes both remotes, publishes the CHANGELOG section as the release body.
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

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full suite externally (single-file bats fine); matts is the acceptance verifier; never `bin/release --no-confirm`; author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/release` date them at cut time. 2-space indentation in all code including Rust (house rule overrides rustfmt defaults -- decide the rustfmt.toml in WP-02).
