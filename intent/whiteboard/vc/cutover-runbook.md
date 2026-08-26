# The canonical Intent v3 config/setup -- TARGET STATE

**Owner: vc. Definitive as of 2026-08-26 11:11Z. This is the target state every project is configured to. hv: _"No correspondence will be entered into."_ hv is AFK from ~11:00Z; vc holds the pen with authority over intent-{cc,dc,ic} and devbin-{vc,cc}. Rulings made under that delegation are marked as such and hv can overrule any of them.**

## Scope

**TWENTY-ONE PROJECTS. COUNT FROM THE LIST, NEVER FROM THE NUMBER.** This runbook has been wrong about the count twice today: it said FIFTEEN while listing sixteen names (dc subtracted Riffle, which has nothing wired and is therefore the project most likely to be skipped, because it is the one with nothing there to notice); then it said SIXTEEN from a depth-limited walk of `~/Devel/prj/*/` that could not see three projects nested one level deeper. **The census predicate that finds them all, from devbin-cc, is `find ~/Devel/prj -maxdepth 6 -type f -path '*/intent/.config/config.json' | sed -E '/\.backup\/|\/deps\/|\.worktrees\/|_Archive\//d'` -- the filter is load-bearing, because `arca_cli` and `arca_config` are also checked out as Elixir deps inside five other projects and an unfiltered sweep counts them six times each.** It returns 22; minus `Intentv2` that is the 21 below. Every one has its own git toplevel, its own `.git/hooks/pre-commit` and its own `.claude/settings.json` -- driven, not assumed.

`A3/a3-content` (2.10.0), `Anvil` (2.13.0), `Arca/arca_cli` (2.18.0), `Arca/arca_config` (2.18.0), `Arca/arca_notionex` (2.10.0), `Baize` (2.19.0), `Cdsync` (2.18.0), `Conflab` (2.19.0), `Courses` (2.14.0), `Courses/002 Agentic Coding` (2.10.0 -- its own repo, nested inside Courses), `Devbin` (2.18.0), `Intent` (3.0.0-dev), `Laksa` (2.19.0), `Lamplight` (2.19.0), `MicroGPTEx` (2.13.0), `Molt` (2.13.0), `Molt-flynn` (2.11.5), `Molt-matts` (2.11.5), `Prolix` (2.13.0), `Riffle` (2.18.0), `Utilz` (2.18.0).

**Versions were driven from each `config.json` at 10:55Z. Four are AT the 2.19.0 floor and take one hop; sixteen are BELOW it and take two; Intent is already v3.**

**TWO EXCEPTIONS, DIFFERENT KINDS:**

- **`~/Devel/prj/Intentv2` -- NOT TOUCHED AT ANY STAGE. NOT A PROJECT.** It is the TOOL TREE other projects run. It carries a project config, so **every census finds it and answers truthfully** -- and `int local status` warns about it unprompted: _"THAT TREE IS THE TOOL OTHER PROJECTS RUN. NEVER MIGRATE IT... Migrating it takes the CLI away from everything else."_ **The reason is recorded with the exclusion because a bare "skip it" gets re-added by the next person who regenerates the list from a census: the census is right, and the list is what carries the knowledge.** (The directory is `Intentv2`, with the `v`.) **USING IT AS THE TOOL -- `~/Devel/prj/Intentv2/bin/intent upgrade` inside another project -- writes nothing into it and is REQUIRED for hop 1 on the sixteen below-floor projects.** The exclusion is about the project, not the binary.
- **`~/Devel/prj/Intent`** -- in scope, and additionally gets `use` to swap the brew-installed v3 against its own dev build. See APPLY step 5 for what `use` can and cannot be.

**Do not disposition what a project has today. Overwrite it.** Variation is the disease, not the data.

## TARGET STATE -- what every project ends up with

### 1. `.claude/settings.json` -- verbatim from `lib/templates/.claude/settings.json`

Three lifecycle hooks, **all dispatching the CLI door**, no absolute paths, no `${CLAUDE_PROJECT_DIR}` script indirection, no inline `echo`:

| event              | matcher                           | command                                 | timeout |
| ------------------ | --------------------------------- | --------------------------------------- | ------- |
| `SessionStart`     | `startup\|resume\|clear\|compact` | `intent claude hook session-context`    | 3000    |
| `UserPromptSubmit` | `""`                              | `intent claude hook require-in-session` | 2000    |
| `Stop`             | `""`                              | `intent claude hook session-finish`     | 3000    |

**`post-tool-advisory` is the fourth name the dispatcher knows (`install.rs:141`) and is deliberately wired NOWHERE.** Shipped, dispatchable, uninvoked. That is the canonical state, not an omission. **The door is bare `intent`, resolved on PATH -- so until the brew flip (APPLY step 4) every hook in every project runs the frozen v2, which answers `claude hook` rc=0 in a v3 tree (driven) and injects v2-shaped context. Not a lockout; stale until the flip.**

### 2. Root canon -- from `lib/templates/llm/_*`

`AGENTS.md` (generated -- `intent agents sync`), `CLAUDE.md`, `usage-rules.md`. **Never hand-edit `AGENTS.md`.** **`CLAUDE.md` is regenerated ONLY with its own consent** -- the tool holds a copy that lacks the `lib/templates/llm/_CLAUDE.md` footer and names `--force` as the way through. See APPLY step 3 for the rule on when to force.

### 3. `intent/.config/config.json`

**Required:** `intent_version` (`3.0.0-dev` today; re-stamps through the ordinary `intent upgrade` when hv tags), `project_name`, `author`, `languages` (explicit array -- filesystem detection is retired). **NOT required: `intent_dir`** -- v3 defaults it (`project.rs` `default_intent_dir`) and `stamp_version` does not write it, so an absent key reading as `intent` IS the canonical configuration; requiring it demanded a key the tool cannot produce. **Minted, not authored:** `project_id`, a UUID written once by the migration. **Preserved if present:** `plugins`, `st_prefix`, `todo`. **Where `project_name` or `author` is MISSING (Laksa is), the tool will not repair it -- `config set` is unwired -- so hand-fix BEFORE hop 2 and name it HAND-FINISHED.**

### 4. `.git/hooks/pre-commit`

Chained to `pre-commit.intent` via chain-block markers, **EXACTLY ONCE**. Region-edited, never regenerated -- consumers carry hand-authored wiring below the block, and some wire guards canon has never heard of. **Two marker spellings exist and both are canonical once landed: v2 wrote `# intent-chain-block:start` / `:end`; `825c48db` wrote `# >>> intent-chain-block >>>` / `<<<`. The fix cc is landing emits the colon form and detects both, anchored on the marker LINE, because Laksa's hook has a prose line mentioning `intent-chain-block`.**

### 5. Also from canon

`.intent_critic.yml` and `usage-rules.md` (seeded when absent, never overwritten, not force-overwritable), `_treeindexignore`, and the `intent/` tree. **v3's `upgrade` adds `intent/events.jsonl`, `intent/st/steel_threads.md`, `intent/todo.md` and gitignores `intent/.cache/` and `intent/.treeindex/`.**

## APPLY

1. **THE BINARY IS `intent3` -- THE CURRENCY-GUARDED WRAPPER AT `Intent/bin/intent3` -- NEVER THE RAW BINARY AND NEVER BARE `intent`.** Bare `intent` is the frozen v2 until the flip; its refusal inside a v3 tree is not a broken project. The wrapper refuses with _"this binary is behind HEAD -- remedy: run 'int local build'"_ whenever a non-test file under `native/rust` has changed since the pair was built (`currency.lib`; test files do not count). **That refusal is the mechanism that stops a stale binary migrating a project: when it fires, STOP and wait for dc's rebuild; do not fall through to `target/release/intent`.** A `dirty-` binary may read canon and never write it. **The pair's commit stamp -- `int local status`, never read off this page -- goes into every migration commit body.**
2. **REWRITE EVERY PROJECT ON THE LIST TO THE TARGET STATE ABOVE -- BY THE TOOL, NOT BY HAND.** vc's ruling under hv's grant, and hv confirmed it: _"by the tool -- Yes, that is better. But if things fail, then fix forward (manually if necessary)."_ hand-editing delivers _"the same configuration"_ ONCE and nothing holds it tomorrow; a tool makes it a property rather than a snapshot, and a tool-written state can be RE-ASSERTED AND COMPARED. **The tool is `intent claude upgrade --apply` (`825c48db` + cc's chain-block fix).** **IF THE TOOL FAILS ON A PROJECT, FIX FORWARD -- MANUALLY IF NECESSARY (hv).** What is NOT permitted is a project left in a state nobody recorded: **if you hand-finish anything, write HAND-FINISHED and what in the commit body**, because the next reader assumes the tool did it. **ONE NON-NEGOTIABLE PROPERTY: IT MUST PRESERVE WIRING IT HAS NEVER HEARD OF** -- Lamplight chains guards that are not in Intent canon; cc's six arms assert every original hook line survives in order. **ONE NAMED COMMIT PER PROJECT, so the rollback is `git revert` on a single sha.** Do not spread a project across commits and do not batch projects into one. **On a project with a live fleet (Lamplight has ` M mix.lock`), commit `--only` your own paths and check `git show --stat HEAD` against the length of your pathspec** -- what the command DID against what it was TOLD, which is the gap two estates fell through today.
3. **THE RECIPE, driven end to end on sandbox copies of Molt-flynn (2.11.5) and Devbin (2.18.0) before any live write:**

   ```
   cd <proj>
   ~/Devel/prj/Intentv2/bin/intent upgrade < /dev/null   # hop 1, BELOW-FLOOR ONLY. THE REDIRECT IS LOAD-BEARING: 0071 hangs without it.
   intent3 upgrade                                        # hop 2. Stamps 3.0.0-dev, writes 3, converges .gitignore. ATOMIC: refuses and writes nothing on a content defect.
   intent3 claude upgrade --apply                         # hop 3. Writes the canon; holds CLAUDE.md without the marker.
   intent3 claude upgrade --apply                         # again: MUST report 0 written. NECESSARY, NOT SUFFICIENT -- see below.
   bash ~/Devel/prj/Intent/intent/whiteboard/vc/verify-canonical.sh <proj>   # MUST be 0 failed before the commit
   git add -A && git commit -m "intent: migrate to v3 canonical (built from <pair sha>)"
   ```

   **`.backup/` MUST BE IGNORED BEFORE `git add -A`** (ic). Hop 1 writes `.backup/backup-<stamp>/` and nothing in the two-hop ignores it; on Riffle `-A` would commit 86 files of pre-migration state into permanent history and the one-commit rollback would revert 86 files instead of a dozen. The standing rule is `.backup/` ignored everywhere. Driven across all 21 with a child-path `check-ignore`: **Courses, Devbin and Riffle are NOT ignored**; the other eighteen carry `.backup/backup-*` or equivalent. On those three, add `.backup/` to `.gitignore` before the commit and write HAND-FINISHED: .backup/ gitignored in the body. The verifier now refuses a project where it is not ignored, so a 0-failed verifier run is the assertion.

   **HOP 1 CREATES THE HOOK.** v2 2.19.0's upgrade rewrites `.claude/settings.json` to the three doors AND installs a colon-form chain block where none existed (ic, driven on Riffle). **So there is no hook-less subset that is safe ahead of cc's fix -- a survey of hooks taken before hop 1 measures a state the recipe destroys.** Check hook state AFTER hop 1, never before.

   **THE SECOND `--apply` DOES NOT CATCH A DOUBLED HOOK.** `825c48db` doubled the chain block exactly once and then reported `0 written` on the doubled state forever -- a converger at the wrong fixed point, which the idempotence control blesses. **Only the verifier's marker COUNT sees it.** Three instruments -- `0 written`, hop 3's `written:` line, a `grep -q` -- all read the doubled hook as correct because none of them counted (ic): agreement is not corroboration when every instrument asks the same question.

   **`CLAUDE.md`: `diff` BEFORE `--force`.** Hop 3 holds any `CLAUDE.md` without the generated footer. On Molt-flynn the 82-line "hand-authored" file was the v2.11 boilerplate template without a footer -- `--force` removed 60 lines of old template and lost nothing. **Where the removed lines are NOT Intent template text, that is project content: carry it into the new file's `## Project-specific` by hand and write HAND-FINISHED in the body.** hv ruled one configuration everywhere; hv did not rule that project instructions are deleted. Write `--force` in the body when you use it. **`--force` DROPS THE `<!-- user:start -->` / `<!-- user:end -->` BLOCK: the template carries the markers (`_CLAUDE.md:47-50`) and `canon.rs` has no code that preserves what is between them (driven at the writer, devbin-cc's question). Devbin's block is load-bearing -- it exists because an earlier regeneration ate a fix. Until the tool preserves it, a populated user block is project content under the rule above: carry it by hand, name it.**

   **HOP 2 CAN REFUSE ON CONTENT.** Devbin fails atomically -- `sqlite: UNIQUE constraint failed: tests.thread_id, tests.id` -- on a duplicate AT id inside one thread. No version census predicts it, **and no grep does either: an AT-uniqueness sweep across the fleet produced two confirmed false positives (Intent's ST0056, already in the store at 139/139; Riffle, where the "duplicate" was a prose citation) because cross-references read as declarations.** **HOP 2 IS THE ORACLE** (ic): it refuses atomically, writes nothing, and NAMES the thread and id. On a refusal, read that one thread by eye -- every pair, two DECLARATIONS each, citations excluded -- fix every real pair in one named commit, hop 2 again. **Nobody edits a ratified contract off a prediction.**

   **The commit goes THROUGH the project's own pre-commit gate with v2 still on PATH** (driven: rc=0, `guards: 1 ran, 3 skipped`). Committing is not blocked on brew. But with v2 on PATH the critic in a v3-stamped tree is not certainly running, so **every project is re-verified after the flip.**

4. **BREW.** `int macos prepare` -> `formula` -> **`publish` (hv's ALONE).** The generated formula's URLs are GitHub release assets and no v3 release exists, so a plain `brew install` 404s until hv publishes. **dc proves everything but the network hop by preseeding brew's download cache with the staged artefacts under the byte-identical formula** (cache-name rule positively controlled against `brew --cache --formula`). Cellar layout: everything under `libexec`, `bin` gets symlinks, `lib/templates/` beside the binary -- a keg that installs only the binary runs, answers `--version`, and cannot render canon (`cmd/macos:81`). **ACCEPTANCE, all driven on the brew-resolved binary:** (a) from a non-checkout dir, bare `intent claude upgrade` NAMES the five artefacts; (b) `intent info | sed -n '/^ *INTENT_HOME:/ { s/^ *INTENT_HOME: *//; s/ *$//; p; }'` resolves to a dir carrying `lib/templates/hooks/pre-commit-guards.sh` -- **eleven devbin-vendored estates resolve their guard runner that way and FAIL OPEN if it is absent**; (c) positive control: INTENT_HOME pointed at a dir with no `lib/templates/` yields `NO guard ran for this commit -- this install has no guard runner.` **Version string stays `3.0.0-dev` (vc ruling under delegation): it is TRUE of the binary until hv tags, and a bump is release engineering.** **THE FLIP -- `brew link`, `/opt/homebrew/bin/intent` at PATH position 1 -- is SEQUENCED, not automatic:** it moves every hook door and every gate on this machine to v3 in one instant. After (a)-(c), after the verifier has been re-run on a canary with the brew binary, on vc's word.
5. **`use dev|prod` -- `int local use`, shipped `39111261`, AND IT IS MACHINE-WIDE BECAUSE PATH IS.** Driven chain (dc): `use dev` = brew unlink + pin + repoint `~/.local/bin/intent` (PATH 17, the binding that wins today) at the dev release binary, then MEASURE what resolves and die if it is not the flavour asked for; `use prod` = unpin + link, PATH 1 wins, 17 harmlessly shadowed. Both drive `verify_hook_door` with stdin closed afterwards, because a copied binary with no `lib/templates` above it refuses every hook at exit 1 and Claude Code does not block on 1. **THE CATCH FOR hv: there is no project-scoped swap through PATH.** `use dev` puts the WHOLE FLEET on this checkout's build, not just Intent; `use prod` reverses it once a formula exists. **The project-scoped dev spelling that already exists is `intent3`.** If hv's model is _"Intent gets dev, the rest get brew"_, that is `intent3` inside Intent and `use prod` for the machine -- recorded on hv's board as a question with this as the interim ruling.

## VERIFY -- per project, after

- `bash intent/whiteboard/vc/verify-canonical.sh <proj>` -- and `--self-test` first if you have any doubt about the instrument. It prints which canon it compared against.
- The second `--apply` wrote 0. Necessary, not sufficient.
- `git show --stat HEAD` against your own pathspec.
- After the flip: `intent --version` and `intent info` answer from the brew binary; the three hooks answer **rc=0**; an unknown hook answers **rc=1**; **never rc=2 on this surface**; `intent ac gate <ST>` runs.
- **Use WHOLE-FILE hashes for any before/after comparison.**

## Settled -- do not re-raise

- **`Intent/bin/intent`** is v2's bash wrapper, PATH position 22, shadowed, never resolves. `AC-12.1` prunes `bin/` at the cut.
- **The stale `INTENT_HOME` at `~/.zshrc:37`** cannot hold a v3 tool on v2 guards -- v3 never reads it (`install.rs:20`). ONE switch, not two.
- **The hook fail-closed lockout does not fire.** Driven four ways at four v2 versions: all 0, unknown 1. And in a project rewritten to canon but not yet migrated, the v3 binary answers all four hook names rc=0 while `st list` refuses at rc=1 in the same fixture (ic) -- the stamp is genuinely read and `claude hook` is genuinely exempt.
- **The one-way door is dissolved.**
- **`~/bin/intent` at PATH position 19** is shadowed today and becomes the silent v2 fallback the moment 17 moves. `use` measures what resolves rather than assuming, which catches 19 without naming it.
- **The v2 upgrade hang (`0071`) is worked around, not fixed:** stdin closed. It is a shipped-surface fix in both trees and is NOT being built today -- _"we're not fixing 2 unless it's broken and stopping you working"_ (hv), and the redirect means it is not stopping anyone.
