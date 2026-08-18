# Claude Code Session Restart -- narrative state

## Current state (as at `ce532a97`, 2026-08-18)

**This heading names a COMMIT, not just a date, and that is deliberate.** A restart file is read as CURRENT STATE and written as a snapshot of when its author typed; nothing used to mark which, and a cold session treated a four-day-old line as the next action. Anything below is true of `ce532a97` and of nothing else -- re-stamp it when you fold, and if you cannot say what it is current as at, that is the finding.

**ST0056 -- Intent v3.0.0 -- is the live work.** Same-day sequence: v2.19.0 shipped in the morning (tag `071c612`, fifteen issues 0009-0023, narrative `intent/history/v2.19.0.md`); in the afternoon hv opened ST0056 and the architecture was ratified in a rubber-duck session with vc. Everything decided lives in `intent/st/ST0056/design.md` -- read it before touching anything v3. The shape: a reified schema-as-truth model (the intentsvcs Rust type layer generates JSON Schema + SQL DDL + GraphQL SDL faces, all committed and drift-checked), **the intentdb as the DURABLE SSOT with everything on disk a secondary artefact** (D01 was REVERSED by hv on 2026-08-15 -- the old wording "committed JSON as durable truth, rebuildable SQLite as runtime truth, `rm intent.db` always safe, no DB migrations ever" is FALSE in every clause and must not be reasoned from), the committed `.json`/`.md` extract as the INTERCHANGE that travels while the DB never leaves the machine (D34), migrations NORMAL, `rm intent.db` ruled out of existence as an operation (D36), a 1-1 lossless db-entity-to-file mapping as the standing openness requirement (AC-02.6) which is what bidirectional sync is FOR, markdown demoted to generated views + authored prose under the authored-once principle (no mixed files), strict validate-or-refuse ingest, `intentsvcs` as sole owner of DB and file canon with the CLI dual-mode (in-process facade calls, or GraphQL to one machine-level intentd serving N projects), MCP as the primary agent write surface, migration floored at v2.19.0 (two-hop; the v2 ledger is never reimplemented), Homebrew via cargo-dist as a core deliverable, intentd IN the 3.0.0 gate (hv ruling, one major release + 3.0.x patches). Prior art: Lamplight `native/cli` (dispatch-spine SSOT, typed errors with remedies, MCP bridge) and Conflab `native/daemon` (conflabd is nearly the intentd stack: async-graphql + axum, rmcp streamable HTTP, CLI-owned launchd lifecycle, mgmt plane, debounced watching, policy-stamp self-healing). Cloud seams ship in 3.0.0 without cloud code: project_id UUID, principal on every facade call, append-only event log, reserved server config block.

**INTENT IS SELF-HOSTED ON v3 -- that is the single biggest change since this file last read true, and it reframes everything below.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist, and a v2 binary REFUSES a v3-declared tree at exit 2.

**Work package status, read from `intent/st/ST0056/WP/*/info.md` at this commit** -- WP-01, WP-02, WP-04 **Done**; WP-03, WP-05, WP-06, WP-10, WP-11 **WIP**; WP-07, WP-08, WP-09, WP-12, WP-13, WP-14, WP-15 **Not Started**. The earlier "WP-03 is cc's next" line was true on 2026-08-14 and has been false since; work has fanned out well past it.

**Contract: 114 AC rows and 114 AT rows** (`intent/st/ST0056/acceptance.md`). AT states tally 52 `to-write` / 39 `green` / 19 `n-a` / 4 `red`, which reconciles to 114. **The AC satisfaction tally does NOT reconcile** -- 93 of 114 rows carry a `satisfied: (yes|no) (computed)` marker -- so no satisfaction ratio is quoted here until that is explained. An unreconciled denominator is not a number.

**ST0057 EXISTS and carries the disk-model design** -- disk as a sparse projection of the store, six ruled decisions D57-1..D57-6: canon relocates to `intent/.canon/` still per-artefact; `.intentfiles` with generated and pinned regions; `intent organize` with four answers plus UNCLAIMED report-never-remove; `intent edit <ID>` rather than `wip`; a full text realisation into `.backup/text/<UTC>/` as a HUMAN fallback distinct from the dehydration gate; and the 165 `design.md`/`impl.md`/`tasks.md` ruled ATTACHMENTS with `THREAD_PROSE` deleted.

**hv rulings, 2026-08-18:** ST0057's disk model is **IN the 3.0.0 gate** -- hv verbatim: _"I need to be able to sit in a project, work with disk versions of the relevant artefacts, and have the db kept in sync as things change. So all of that has to happen before we do the 3.0.0 release."_ The regeneration runs BEFORE the critic gate's Half A, which means **the regeneration commit is unlinted by construction and that is an accepted cost, not an oversight**.

**THE PRE-COMMIT CRITIC GATE HAS BEEN DARK IN ALL FIVE LANGUAGES SINCE THE HOIST** (ic). `~/.local/bin/intent` is this repo's own v2 binary, the tree declares 3.0.0-dev, v2 refuses at exit 2, and `pre-commit.sh:289` turns exit 2 into a fail-open line -- so every commit since self-hosting is unlinted, and the gate has been announcing it once per language the whole time. Argument primed at `intent/st/ST0056/critic-gate.md`. Half B: 0 of 6 shell rules and 0 of 7 rust rules carry a greppable proxy, so **elixir is the only pack that can currently discriminate anything**.

**STRANDED 192 IS THE ONLY NUMBER GATING ANY DELETION**, and its subject is the PINNED corpus `hoist` @ `9b73e98f`, not the live tree -- so a regeneration of `intent/` cannot move it. What moves it is the MIGRATOR CODE. `LOST-PROSE 0` is the trap beside it: it means only that every section which HAS a destination reached it.

**The sweep program is dead, and that reshaped WP-10**: Lamplight is already at 2.19.0 and their hv ruled AT remediation on Done work off outright -- their ~1158 legacy-grammar rows are permanent. tasks.md and migration.md say the corpus is the fleet AS IT IS, and the policy is now RULED in migration.md: **closed-thread lossless-by-carrying** (legacy rows carried whole, marked legacy, nothing guessed); live threads keep BLOCKED-until-clean; neither class ever gets a lossy path.

## Next (as at `ce532a97`)

1. **cc -- IN FLIGHT, released by hv directly this session.** `Triage->Wip`, `has_end_date()`, delete `THREAD_PROSE`, fix `views::info`'s blank line, THEN regenerate. Two measurements, different subjects: a live-estate digest before/after the regeneration (no binary), and conservation on pinned `hoist` @ `9b73e98f` before/after the code change with the binary named verbatim on each half. The before-digest is taken (1323 of 1323, errors 0).
2. **dc -- HOLDING.** Critic gate Half A lands AFTER cc's regeneration, driven at a commit at-or-after it. The proof is a RED -- a staged elixir violation the hook refuses -- never a green.
3. **ic -- HOLDING.** Owns the `same_end_state_check.sh` run (real SIGKILL mid-migration, then re-run).
4. **vc.** The interruption property and AC-10.8 (egest symmetry) into the AC set -- **BLOCKED on hv answering whether the AC moratorium is lifted** (asked before the reboot, never answered; the moratorium's own terms were "until the hoist lands" and the hoist has landed). `same_end_state_check.sh` and `interrupt_rig.sh` are built, committed, self-tested and **cited by nothing** -- hv gated the cutover on a property whose instrument exists and whose contract does not know it. Also mine: ST0011 (`completed` NULL on the estate's one wrong row), after the regeneration.
5. **With hv, unruled:** D50 (`WpStatus` gains `Cancelled` vs cc's `status_legacy` companion -- context delivered this session, `status_legacy` still absent from the tree); whether a dedicated `--skip-rust-tests` should exist; `doctor` printing `intent v2.19.0` on a 3.0.0-dev project.
6. **Standing:** `organize` (both faces) planned vestigial by construction; v2 maintenance is DEFAULT-DEFER, show-stoppers only. v2 carries: credo_checks fleet issues (hv running); the Utilz `0171297` / Lamplight `7058fd3a8` pushes -- **re-verify both are still unpushed before acting**. Upstream FROZEN; v3 NOT on PATH; push `local` only.

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
