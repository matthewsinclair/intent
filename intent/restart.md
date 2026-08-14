# Claude Code Session Restart -- narrative state

## Current state (2026-08-14)

**ST0056 -- Intent v3.0.0 -- is the live work.** Same-day sequence: v2.19.0 shipped in the morning (tag `071c612`, fifteen issues 0009-0023, narrative `intent/history/v2.19.0.md`); in the afternoon hv opened ST0056 and the architecture was ratified in a rubber-duck session with vc. Everything decided lives in `intent/st/ST0056/design.md` -- read it before touching anything v3. The shape: a reified schema-as-truth model (the intentsvcs Rust type layer generates JSON Schema + SQL DDL + GraphQL SDL faces, all committed and drift-checked), committed JSON as durable truth, a rebuildable per-project SQLite DB as runtime truth (`rm intent.db` always safe, no DB migrations ever), markdown demoted to generated views + authored prose under the authored-once principle (no mixed files), strict validate-or-refuse ingest, `intentsvcs` as sole owner of DB and file canon with the CLI dual-mode (in-process facade calls, or GraphQL to one machine-level intentd serving N projects), MCP as the primary agent write surface, migration floored at v2.19.0 (two-hop; the v2 ledger is never reimplemented), Homebrew via cargo-dist as a core deliverable, intentd IN the 3.0.0 gate (hv ruling, one major release + 3.0.x patches). Prior art: Lamplight `native/cli` (dispatch-spine SSOT, typed errors with remedies, MCP bridge) and Conflab `native/daemon` (conflabd is nearly the intentd stack: async-graphql + axum, rmcp streamable HTTP, CLI-owned launchd lifecycle, mgmt plane, debounced watching, policy-stamp self-healing). Cloud seams ship in 3.0.0 without cloud code: project_id UUID, principal on every facade call, append-only event log, reserved server config block.

**WP-01 is DONE (gate 4/4, hv-ratified) and WP-02 is at 4/6 in cc's hands.** The full contract stands at 62 ACs / 60 AT rows, lint-clean, with the WP-01 specs beside design.md: `data-model.md`, `migration.md`, `parity.md`. **hv ruling mid-day: cc and ic write the code; vc stewards** -- contract, verification at WP closes, hv interface; vc keeps the ST0056 claim in stewardship form. State at the fold: vc built and mutation-proved the WP-02 foundation pre-ruling (workspace, model-as-master, store with D01 as law, committed faces + drift guard, CI -- `5e4b766`); cc landed the SDL face (`732affa`) and is bringing **devbin** into `bin/` (entry point `bin/int`; `bin/release` moving to `bin/.devbin/cmd/build.d/release`; `bin/intent` untouched by design); ic delivered the whole parity deep pass (26 `parity/cmd-*.md` files, the 94-row register, the BATS estate retargeted through `INTENT_BIN` -- 711/1235 tests actually reach the CLI) and localfolded. cc also fixed 0024 (scoped `at lint`/`ac gate` dropping the WP scope; a scoped `--fix` rewrote OUTSIDE the scope) on a one-off hv go.

**The sweep program is dead, and that reshapes WP-10**: Lamplight is already at 2.19.0 and their hv ruled AT remediation on Done work off outright -- their ~1158 legacy-grammar rows are permanent. tasks.md and migration.md now say the corpus is the fleet AS IT IS, and migration.md carries the open policy question (BLOCKED-until-clean vs an estate that will never clean; likely answer lossless-by-carrying for CLOSED threads only).

## Next (the bounce agenda -- hv rulings first)

1. **Ratify or strike the `corrected` parity class** (drafted in parity.md from ic's census: unknown-flag exit 0, `--help` failing on 10/27, the 45/12/2 stream census).
2. **Migration policy for never-swept estates** (the open question in migration.md): lossless-by-carrying for CLOSED threads vs BLOCKED-until-clean for live ones.
3. **The organize Highlander** (ic): `intent organize` and `intent st organize` are two implementations of one job, both in MODULES.md; which survives into v3.
4. **AC-02.6 at WP-02 close**: red-until-WP-04 vs descope (the envelope test cannot exist before facade verbs). AC-02.1 flips on the first green CI run -- needs a push.
5. **v2 maintenance scope during the v3 build** -- still unstated as policy (0024 got a one-off go).
6. Then: WP-02 close review (vc), WP-03 (cc), the register's 95th file + per-test split rows (ic). Unchanged v2 carries: credo_checks fleet issues, the fleet pushes (Utilz `0171297`, Lamplight `7058fd3a8`).

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
