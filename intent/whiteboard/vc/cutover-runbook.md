# The canonical Intent v3 config/setup -- TARGET STATE

**Owner: vc. Definitive as of 2026-08-26 10:52Z. This is the target state every project is configured to. hv: _"No correspondence will be entered into."_**

## Scope

**SIXTEEN PROJECTS.** **CORRECTED 2026-08-26 -- this runbook said FIFTEEN and then listed SIXTEEN NAMES. The list was right and the number was wrong.** 17 directories carry an `intent/.config/config.json`; minus `Intentv2` (the tool tree, not a project) that is **16**. The 15 came from dc subtracting **Riffle** as well -- but Riffle IS a project at 2.18.0 that merely has **no `.claude/settings.json`**, which is a STATE THE REWRITE FIXES, not a disqualification. **Riffle is the project most likely to be skipped, because it is the one with nothing there to notice.** Count from the list, never from the number. Anvil, Baize, Cdsync, Conflab, Courses, Devbin, Intent, Laksa, Lamplight, MicroGPTEx, Molt, Molt-flynn, Molt-matts, Prolix, Riffle, Utilz.

**TWO EXCEPTIONS, DIFFERENT KINDS:**

- **`~/Devel/prj/Intentv2` -- NOT TOUCHED AT ANY STAGE. NOT A PROJECT.** It is the TOOL TREE other projects run. It carries a project config, so **every census finds it and answers truthfully** -- vc's fleet list had it at 2.19.0, dc's census had it as a Form-B project, neither flagged it, and `int local status` warns about it unprompted: _"THAT TREE IS THE TOOL OTHER PROJECTS RUN. NEVER MIGRATE IT... Migrating it takes the CLI away from everything else."_ **The reason is recorded with the exclusion because a bare "skip it" gets re-added by the next person who regenerates the list from a census: the census is right, and the list is what carries the knowledge.** (The directory is `Intentv2`, with the `v`.)
- **`~/Devel/prj/Intent`** -- in scope, and additionally gets `use` to swap the brew-installed v3 against its own dev build.

**Do not disposition what a project has today. Overwrite it.** Variation is the disease, not the data.

## TARGET STATE -- what every project ends up with

### 1. `.claude/settings.json` -- verbatim from `lib/templates/.claude/settings.json`

Three lifecycle hooks, **all dispatching the CLI door**, no absolute paths, no `${CLAUDE_PROJECT_DIR}` script indirection, no inline `echo`:

| event              | matcher                           | command                                 | timeout |
| ------------------ | --------------------------------- | --------------------------------------- | ------- |
| `SessionStart`     | `startup\|resume\|clear\|compact` | `intent claude hook session-context`    | 3000    |
| `UserPromptSubmit` | `""`                              | `intent claude hook require-in-session` | 2000    |
| `Stop`             | `""`                              | `intent claude hook session-finish`     | 3000    |

**`post-tool-advisory` is the fourth name the dispatcher knows (`install.rs:141`) and is deliberately wired NOWHERE. **AND EVERY PROJECT NEEDS REWIRING, NOT JUST THE HARDCODED ONES (dc): Form-B projects wire `session-context` and `require-in-session` but NOT `session-finish` -- Baize reports 2 hooks against a target of 3.** Corroborated by lamplight-cc: Lamplight answers Stop with an inline `echo` and has never once dispatched `session-finish`. **So the cutover is SIXTEEN projects in TWO SHAPES, not nine.**** Shipped, dispatchable, uninvoked. That is the canonical state, not an omission.

### 2. Root canon -- from `lib/templates/llm/_*`

`AGENTS.md` (generated -- `intent agents sync`), `CLAUDE.md`, `usage-rules.md`. **Never hand-edit `AGENTS.md`.**

### 3. `intent/.config/config.json`

**Required:** `intent_version` (`3.0.0`), `project_name`, `author`, `intent_dir`, `languages` (explicit array -- filesystem detection is retired). **Minted, not authored:** `project_id`, a UUID written once by the migration and never re-minted. **Preserved if present:** `plugins`, `st_prefix`, `todo`. **Note:** v3's `write_config` materialises defaults for unset keys; the migration's `stamp_version` does NOT -- it inserts two keys into the raw JSON and leaves the rest alone.

### 4. `.git/hooks/pre-commit`

Chained to `pre-commit.intent` via the `intent-chain-block` markers. **Region-edited, never regenerated** -- consumers carry hand-authored wiring below the block, and some wire guards canon has never heard of.

### 5. Also from canon

`.intent_critic.yml`, `_treeindexignore`, and the `intent/` tree (`st/`, `docs/`, `llm/`, `plugins/`, `eng/`, `ref/`).

## APPLY

1. **QUIET WINDOW FOR THE BUILD.** The release pair is CLEAN-BUT-BEHIND: binaries stamp an earlier commit than HEAD and `verify_pair` demands source-commit == HEAD exactly. **DO NOT READ A SHA OFF THIS PAGE** -- drive `git log --oneline -1` and `git status --porcelain -- native/rust` at the moment you act. **`AC-11.6`'s shared-path permission is PERISHABLE and was already expired when this runbook first claimed it**: `native/rust` must be CLEAN, and any node editing `native/rust/crates/**` revokes it. Build to a **STAGING DIR**, never shared `target/release/`.
2. **REWRITE ALL 15 TO THE TARGET STATE ABOVE -- BY THE TOOL, NOT BY HAND.** vc's ruling under hv's grant, and it is the one call in this runbook that could cut against hv's phrasing (_"we just go into each project and REWRITE"_ reads like hand-editing). **Reasons, so hv can overrule cheaply:** hv's own goal is _"after today EVERYTHING will be using the same fucking configuration"_ -- **hand-editing delivers that ONCE and nothing holds it tomorrow; a tool makes it a property rather than a snapshot.** And **fifteen hand-written repos produce fifteen artefacts nobody can re-derive**: verification collapses to reading each by eye against a spec, which is the weakest instrument this estate owns. A tool-written state can be RE-ASSERTED AND COMPARED -- run it twice, diff nothing. **`intent claude upgrade`'s own description is already "Apply Claude canon to the project", which is hv's instruction word for word** (cc). cc is building the NARROW SLICE that writes the five artefacts -- not v2's 1435-line port. **The fleet waits on that slice.** **IF THE TOOL FAILS ON A PROJECT, FIX FORWARD -- MANUALLY IF NECESSARY (hv).** The tool is the mechanism, not a gate: a project the slice cannot handle gets brought to the target state by hand and the tool's gap is filed, rather than the fleet stopping. **What is NOT permitted is a project left in a state nobody recorded** -- if you hand-finish one, say which and why, because the next person will assume the tool did it. **ONE NON-NEGOTIABLE PROPERTY, from a consumer: IT MUST PRESERVE WIRING IT HAS NEVER HEARD OF** -- Lamplight chains four guards and two are not in Intent canon at all, so a canon-aware regenerator drops them silently. v2's `canon_insert_chain_block` is correct by construction (present -> return; absent -> stream line-by-line preserving everything); match it, do not reinvent it. **ONE NAMED COMMIT PER PROJECT, so the rollback is `git revert` on a single sha** -- that is WP-10's own rollback design and it is cheap only while the change is one commit over an estate git holds whole. **Do not spread a project's rewrite across commits and do not batch several projects into one.**
3. **MIGRATE.** Below `MIGRATION_FLOOR = (2,19,0)`, two-hop first: v2 `intent upgrade` **WITH STDIN CLOSED** -- `0071` blocks on an interactive read, has no `--yes`, and **HANGS rather than fails** for any non-TTY caller; its own body proves closed stdin completes at rc=0. **Never bump a stamp to skip the hop** -- four projects carry a current shape under a lagging stamp, and a bump papers over a genuinely unconverged project with a false green.
4. **BREW.** `prepare` -> `formula` -> `publish`. `cmd_formula` inherits stage's refusal: its only input is `SHA256SUMS.txt`, written only for artefacts proven signed and notarised. Tap `matthewsinclair/homebrew-intent` is empty on purpose; the formula is GENERATED by `int macos formula` and must not be hand-edited. **Nothing hashes until `verify_notarised` passes -- `codesign --force` rewrites the binary in place.**
5. **`use dev|prod`, `Intent` ONLY.** Adapt Conflab's -- most of its 367 lines do not apply (its app half has no Intent counterpart). What carries: **PIN** the formula on `dev` so `brew upgrade` cannot silently re-link, **UNPIN** before linking prod, and **VERIFY the switch took** rather than reporting the intent of one -- a switch that silently did not take is worse than one that failed, because everything downstream then measures the wrong binary. **`use` OWNS THE SWITCH ONLY AND DELEGATES REPORTING TO `int local status`** (cc's Highlander call, and it corrects this runbook's earlier wording): `local status` already walks PATH because position IS the answer, already refuses a set describing no single state, and already names the tool tree that must never be migrated. **A second PATH-resolution reporter is precisely the defect `local status` was built to end** -- the machine had four `intent` bindings and only one was written down.

## VERIFY -- per project, after

- `.claude/settings.json` matches canon byte-for-byte.
- `intent --version` and `intent info` answer; `intent info` prints a resolvable `INTENT_HOME`.
- The three hooks answer **rc=0**; an unknown hook answers **rc=1**. **Never rc=2 on this surface** -- Claude Code reads hook rc=2 as BLOCK, and a 2 wedges every prompt in every session in that project.
- `intent ac gate <ST>` runs.
- **Use WHOLE-FILE hashes for any before/after comparison.** Two nodes computed "the local region hash" of one file and got incomparable numbers because the region was never defined.

## Settled -- do not re-raise

- **`Intent/bin/intent`** is v2's bash wrapper (sha `c70c9ebdf905449c`) reading `$INTENT_HOME` where the Rust core refuses to. **PATH position 22, shadowed by 17, never resolves.** `AC-12.1` prunes `bin/` at the cut.
- **The stale `INTENT_HOME` at `~/.zshrc:37`** cannot hold a v3 tool on v2 guards -- **v3 never reads it** (`install.rs:20`: _"$INTENT_HOME IS NOT READ AT ALL"_). ONE switch, not two.
- **The hook fail-closed lockout does not fire.** Driven across v3+v3, v3+no-project, v2-binary+v3-tree, and v3-binary+v2-tree at 2.19.0 / 2.14.0 / 2.13.0 / 2.11.5: all 0, unknown 1.
- **The one-way door is dissolved.** Migrate-then-rewire is unrecoverable _by either tool's `claude upgrade` verb_; writing the files directly does not use that verb, so order no longer strands.
- **`~/bin/intent` at PATH position 19** is shadowed today and becomes the silent v2 fallback the moment 17 moves. Handle in the same operation.
