# The canonical Intent v3 config/setup -- TARGET STATE

**Owner: vc. Definitive as of 2026-08-26 14:05Z. This is the target state every project is configured to. hv: _"No correspondence will be entered into."_ hv was AFK ~11:00Z-11:50Z and is present since; vc holds the pen with authority over intent-{cc,dc,ic} and devbin-{vc,cc} and, since ~11:58Z, hv's first-hand push grant for `Intent` and `homebrew-intent`. Rulings made under that delegation are marked as such and hv can overrule any of them; hv's own first-hand rulings are on hv's board with their menus.**

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

## THE ENDGAME, AND THE WORD FOR IT

**"Landed" means files committed and the verifier at 0 failed. It is NOT success: a landed project's bare `intent` is the frozen v2 until the flip and refuses state commands at rc=2** (hv found this with `intent --version` in Baize and stopped the fleet). **The vocabulary is hops, named landings, the flip; ONLY THE FLIP EARNS "DONE"** (devbin-cc). The flip is `brew link` of a keg installed FROM THE TAP over the network, which needs a REAL release: tag, GitHub release with the notarised artefacts, formula pushed to `homebrew-intent`. **hv authorised those pushes first-hand on 2026-08-26 (verbatim in `hv/inbox.vc.md`, 11:58Z).** The critical path is: ic's ingest fix committed (full sweep to completion, done at `2aa82d17`; cc's bracket-aware `field()` at `56364200`) -> **hv fires `int build release v3.0.0` in hv's own terminal** (the verb bumps three version homes, commits, tags and pushes both remotes; its confirm reads from `/dev/tty` and this estate never passes `--no-confirm` from a tool session) -> dc builds once on the tag -> fleet migrates on that pair while dc runs prepare -> formula -> publish -> `brew install matthewsinclair/intent/intent` from the tap -> `brew link` on vc's word -> every project re-verified with v3 actually on PATH. No step is skipped and no step is reported as more than it is.

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

   **`CLAUDE.md`: `diff` BEFORE `--force` -- hv RULED IT FIRST-HAND (to dc): allow the overwrite, carry project content by hand.** Hop 3 holds any `CLAUDE.md` without the generated footer. On Molt-flynn the 82-line "hand-authored" file was the v2.11 boilerplate template without a footer -- `--force` removed 60 lines of old template and lost nothing. **Where the removed lines are NOT Intent template text, that is project content: corpus-test it against the other projects' `CLAUDE.md` (dc), carry it into the new file's user block under `## Project-specific` by hand, headings demoted one level, and write HAND-FINISHED plus what in the body.** So far the authored content found in that family is a commit-signature rule (Prolix, Anvil) and, on Laksa, 123 lines of real instructions -- spliced from the saved pre-migration file with a line-by-line presence check, never retyped. **THE USER BLOCK IS NOW CARRIED BY THE TOOL: cc's splice (`8ba6c026`) keeps whatever is between `<!-- user:start -->` / `<!-- user:end -->` byte-for-byte across `--apply` and `--force`; before it, the tool SUBSTITUTED the template's default block -- well-formed, author line, no gap -- on every generated `CLAUDE.md` with no `--force` needed (Baize lost 20 bytes of provenance on the plain path). Hand-carry is therefore scoped to OUTSIDE-the-markers content only (dc): carrying the block too leaves a project two copies of its own directive.** The verifier's `root canon` arm demands the generated footer, so a held file fails it by design -- that FAIL names the pending `--force`, not a defect.

   **HOP 2 CAN REFUSE ON CONTENT.** Devbin fails atomically -- `sqlite: UNIQUE constraint failed: tests.thread_id, tests.id` -- on a duplicate AT id inside one thread. No version census predicts it, **and no grep does either: an AT-uniqueness sweep across the fleet produced two confirmed false positives (Intent's ST0056, already in the store at 139/139; Riffle, where the "duplicate" was a prose citation) because cross-references read as declarations.** **HOP 2 IS THE ORACLE** (ic): it refuses atomically, writes nothing, and NAMES the thread and id. On a refusal, read that one thread by eye -- every pair, two DECLARATIONS each, citations excluded -- fix every real pair in one named commit, hop 2 again. **Nobody edits a ratified contract off a prediction.**

   **HOP 2 SILENTLY INVERTED AC SATISFACTION UNTIL ic's INGEST FIX (landing as a source commit; the pair built on it is the one exposed projects wait for).** `legacy.rs` matched `satisfied: yes` to end-of-field, so `satisfied: yes (hv signed off 2026-06-22)` fell to the catch-all and became UNSATISFIED, exit 0, note dropped -- Courses' completed ST0002 came out 8 of 10 unsatisfied. **Exposure is measured, not assumed:** `intent/whiteboard/vc/exposure.sh <proj> <ref>` counts hand-written parentheticals in the pre-migration `acceptance.md` files (a bash FILE, because an inline zsh loop over a newline list runs once and prints zeros -- it did, for every project, including the one with 8). **Lamplight 28, arca_cli 9, arca_config 3, Devbin 1, Courses 8 (reverted at `aa25be1`, re-migrates on the fixed pair); every other project 0, so its canon is right and it proceeds on the current pair.** The fix carries the note into the row's `evidence:` and REFUSES an unrecognised verdict.

   **TWO HOMES PER BUCKETED THREAD, LEFT AS-IS. DO NOT RUN `collapse-buckets.sh`.** v3 writes flat `intent/st/<ID>/` views and never relocates a thread out of v2's `COMPLETED/ CANCELLED/ NOT-STARTED/` buckets (`migrate.rs:47`, a documented hole). The old home carries authored prose canon does not (design/impl/tasks, and `acceptance.md`'s preamble with hv's ratification amendments). My collapse script's delete arm treated any old file with a flat counterpart as a superseded view and deleted that preamble -- ic caught it after it had run on three projects. **The migration commit carries the additive two-homes state; the body says so.** The collapse returns only under ic's rule -- delete only a line-subset of the counterpart, otherwise keep as `<name>.v2.<ext>` -- and only after a second node has read its dry run.

   **The commit goes THROUGH the project's own pre-commit gate with v2 still on PATH** (driven: rc=0, `guards: 1 ran, 3 skipped`). Committing is not blocked on brew. But with v2 on PATH the critic in a v3-stamped tree is not certainly running, so **every project is re-verified after the flip.**

4. **BREW.** `int macos prepare` -> `formula` -> **`publish` (hv's ALONE).** The generated formula's URLs are GitHub release assets and no v3 release exists, so a plain `brew install` 404s until hv publishes. **dc proves everything but the network hop by preseeding brew's download cache with the staged artefacts under the byte-identical formula** (cache-name rule positively controlled against `brew --cache --formula`). Cellar layout: everything under `libexec`, `bin` gets symlinks, `lib/templates/` beside the binary -- a keg that installs only the binary runs, answers `--version`, and cannot render canon (`cmd/macos:81`). **ACCEPTANCE, all driven on the brew-resolved binary:** (a) from a non-checkout dir, bare `intent claude upgrade` NAMES the five artefacts; (b) `intent info | sed -n '/^ *INTENT_HOME:/ { s/^ *INTENT_HOME: *//; s/ *$//; p; }'` resolves to a dir carrying `lib/templates/hooks/pre-commit-guards.sh` -- **eleven devbin-vendored estates resolve their guard runner that way and FAIL OPEN if it is absent**; (c) positive control: INTENT_HOME pointed at a dir with no `lib/templates/` yields `NO guard ran for this commit -- this install has no guard runner.` **Version string stays `3.0.0-dev` (vc ruling under delegation): it is TRUE of the binary until hv tags, and a bump is release engineering.** **THE FLIP -- `brew link`, `/opt/homebrew/bin/intent` at PATH position 1 -- is SEQUENCED, not automatic:** it moves every hook door and every gate on this machine to v3 in one instant. After (a)-(c), after the verifier has been re-run on a canary with the brew binary, on vc's word.
5. **`use dev|prod` -- `int local use`, shipped `39111261`, AND IT IS MACHINE-WIDE BECAUSE PATH IS.** Driven chain (dc): `use dev` = brew unlink + pin + repoint `~/.local/bin/intent` (PATH 17, the binding that wins today) at the dev release binary, then MEASURE what resolves and die if it is not the flavour asked for; `use prod` = unpin + link, PATH 1 wins, 17 harmlessly shadowed. Both drive `verify_hook_door` with stdin closed afterwards, because a copied binary with no `lib/templates` above it refuses every hook at exit 1 and Claude Code does not block on 1. **THE CATCH FOR hv: there is no project-scoped swap through PATH.** `use dev` puts the WHOLE FLEET on this checkout's build, not just Intent; `use prod` reverses it once a formula exists. **The project-scoped dev spelling that already exists is `intent3`.** If hv's model is _"Intent gets dev, the rest get brew"_, that is `intent3` inside Intent and `use prod` for the machine -- recorded on hv's board as a question with this as the interim ruling.

## VERIFY -- per project, after

- `bash intent/whiteboard/vc/verify-canonical.sh <proj>` -- and `--self-test` first if you have any doubt about the instrument. It prints which canon it compared against.
- The second `--apply` wrote 0. Necessary, not sufficient.
- `git show --stat HEAD` against your own pathspec.
- After the flip: `intent --version` and `intent info` answer from the brew binary; the three hooks answer **rc=0**; an unknown hook answers **rc=1**; **never rc=2 on this surface**; `intent ac gate <ST>` runs.
- **Use WHOLE-FILE hashes for any before/after comparison.**

## VERIFY -- THE REBUILD BATCH (written 2026-08-28 16:28Z, vc)

**THE PAIR ON PATH RIGHT NOW, read from the binary and not from any ledger:** `intent --version` -> `3.0.0 (4479264f7b6ec83829a3f7b80c70e332be6daf81)`; `shasum -a 256 native/rust/target/release/intent` -> `fd5e785d1cfee15d087d43da052c494b1d5ebf273a71fe86d5824880c070ff24`; `~/.local/bin/intent` is a symlink INTO `native/rust/target/release/`, so the build IS the delivery. **Both properties change on a rebuild, and neither is a HEAD-compare** -- the diagnostic that says the alarming thing on the healthy case.

**CORRECTION, MEASURED 2026-08-28 AND CARRIED WRONG UNTIL THEN:** my board listed **0110's renderer fix** as part of what this rebuild delivers. It is not. `f02fb55f` is an ANCESTOR of `4479264f` (`git merge-base --is-ancestor`), so it has been on PATH since the pair was built. And `0110` is still OPEN regardless, on the second cause cc split out as `0115` -- so neither "the rebuild delivers it" nor "it is fixed" was safe to say. **A claim outlives its basis and nothing announces it; this one would have gone to hv inside the rebuild GO.**

**WHAT THE BATCH ACTUALLY CARRIES:** 18 commits touch `native/rust` between `4479264f` and `1f27f128`, plus `0121` when cc lands it. Derived from `git log 4479264f..HEAD -- native/rust`, not from what any node reported.

**FIVE CHECKS WITH THEIR _BEFORE_ VERDICT ALREADY DRIVEN ON THE DELIVERED BINARY** -- each one proven to discriminate, so a pass after the rebuild means something:

| #   | what it proves                                                 | command                                        | BEFORE (driven, this pair) | AFTER must be             |
| --- | -------------------------------------------------------------- | ---------------------------------------------- | -------------------------- | ------------------------- |
| A   | `--date` on the closing verbs -- **Conflab's 50** (`bd5894df`) | `intent st done --help \| grep -c -- '--date'` | `0`                        | `>= 1`                    |
| B   | `st repair` retired (`abcb90f7`)                               | `intent st repair --help; echo $?`             | `rc=0` (it exists)         | nonzero                   |
| C   | bare `intent llm` answers -- ST0067's surface (`be3edf70`)     | `intent llm; echo $?`                          | `rc=2` (refuses)           | `rc=0`                    |
| D   | `llm usage_rules` answers (`be3edf70`)                         | `intent llm usage_rules; echo $?`              | `rc=2`                     | `rc=0`                    |
| E   | `st list` discloses its filter -- `0121`                       | `intent st list \| tail -2`                    | last row, no footer        | a footer naming the scope |

**NAMED, NOT PRE-DRIVEN, AND SAYING SO:** these have side effects on live canon or need another estate, so their before-verdict is UNMEASURED rather than quietly assumed --

- `ac new` on an existing id prints `replaced` and names what the default overwrote (`f3d15891` + `38e98942`). **Do not drive this on live canon: it is the destructive PUT of `0119`.** Fixture only.
- `agents sync` writes `_Not configured for this project._` for a DECLARED pack with no block -- the (k) invariant (`1fb7be15`). Writes `AGENTS.md`; drive on a fixture or accept the diff.
- `sync --to-store` stops printing both answers to one question (`557d220d`).
- `doctor` learns the second carrier shape and a disposition class stops naming a subject (`a12147c1`).
- a **Superseded** thread converts as cancelled rather than refusing the estate (`f6613495`) -- **Laksa's, and it only proves out at Laksa's migration.**
- the formatter exclusion reaches a CONSUMER repo, not just this one (`27654493`, `fe4515e8`).

**ORDER:** cc lands `0121` -> cc calls the batch and announces BY PROPERTIES -> vc puts the GO to hv -> the rebuild -> verify by the two properties at the top, then A-E -> **only then** ping conflab-vc, whose recovered input is waiting in Conflab issue `0010`.

## Settled -- do not re-raise

- **`Intent/bin/intent`** is v2's bash wrapper, PATH position 22, shadowed, never resolves. `AC-12.1` prunes `bin/` at the cut.
- **The stale `INTENT_HOME` at `~/.zshrc:37`** cannot hold a v3 tool on v2 guards -- v3 never reads it (`install.rs:20`). ONE switch, not two.
- **The hook fail-closed lockout does not fire.** Driven four ways at four v2 versions: all 0, unknown 1. And in a project rewritten to canon but not yet migrated, the v3 binary answers all four hook names rc=0 while `st list` refuses at rc=1 in the same fixture (ic) -- the stamp is genuinely read and `claude hook` is genuinely exempt.
- **The one-way door is dissolved.**
- **`~/bin/intent` at PATH position 19** is shadowed today and becomes the silent v2 fallback the moment 17 moves. `use` measures what resolves rather than assuming, which catches 19 without naming it.
- **The v2 upgrade hang (`0071`) is worked around, not fixed:** stdin closed. It is a shipped-surface fix in both trees and is NOT being built today -- _"we're not fixing 2 unless it's broken and stopping you working"_ (hv), and the redirect means it is not stopping anyone.

## 2026-08-26, THE TAG AND THE FLIP -- FOLDED TO ITS TOOL TRAPS

**212 chronological bullets, 172KB, collapsed here. Full text at `.history/20260827/cutover-runbook-fold-2209Z.md`.**

**WHAT THE DAY WAS:** `v3.0.0` tagged at `80d8b2ca` and published; the fleet flipped onto it; the re-conversion of estates whose migration had dropped AT and AC rows; and the discovery that a migrated view can read worse than its source. **All of it is closed.** Its RULES live in `vc/wip.md`'s watch-outs, its CONVENTIONS in `intent/restart.md`, and its class instances in the evening section below.

**WHAT IS KEPT IS THE TOOL, SHELL AND GIT TRAPS -- the things that would bite somebody tomorrow and are not written down anywhere else.**

### Shell and git

- **`git commit --only -- .` SKIPS UNTRACKED FILES SILENTLY, AT rc 0** (lamplight-vc, reproduced in a scratch repo). **With an explicit path `--only` ERRORS on an untracked file; with `.` it commits the tracked changes and prints a clean summary.** The two spellings differ in exactly the case that loses work.
- **`git checkout -- <file>` IN A WORKTREE HOLDING UNCOMMITTED WORK IS A REVERT, NOT A RESTORE** (cc). Undoing a mutation with it takes the file to HEAD and **discards the change under test**, so the next arm measures a tree nobody meant to create.
- **A MIXED RESET RESETS THE SHARED INDEX** (lamplight-vc). Before `git reset --mixed HEAD~1` on a shared checkout, **`git diff --cached` must be EMPTY** -- a peer's staged work is silently unstaged by the undo of somebody else's commit.
- **`git rev-list -1 HEAD -- <scope>` RETURNS EMPTY AT rc 0 when nothing touched the scope.** An empty answer reads as a clean one.
- **A `cd` INSIDE A COMPOUND COMMAND RETARGETS EVERY RELATIVE PATH AFTER IT** (vc, my own). A call beginning `cd ~/Devel/prj/Lamplight && ...` that then appended to `intent/whiteboard/vc/...` wrote into the wrong estate and committed it there.
- **`stat -f '%Sm'` PRINTS LOCAL, AND THE LABEL IS WHERE THE CLOCK ERROR HIDES** (dc, own catch). The epoch comparison was sound; a `Z` was appended to a local reading.
- **macOS `iconv -f UTF-8 -t UTF-8` IS NOT A UTF-8 VALIDATOR** (vc). It exits 1 with `Inappropriate ioctl for device` on **valid** files -- **and the positive control (a file containing `0xff`) failed with the SAME message**, so the control could not discriminate either.

### This estate's own machinery

- **CANON IS THE COMMITTED EXTRACT; THE LIVE STORE IS `intent/.cache/intent.db`** (devbin-vc, correcting their own account). **Editing canon does nothing until a sync** -- which is why a body can sit visible in the file while the tool reports it absent.
- **THE PROVENANCE STAMP'S SUBJECT IS THE BUILD'S INPUTS, NOT `native/rust`** (dc, found before a line was written). `dispatch.rs` carries `include_str!(".../surface/dispatch-table.json")`, **so `surface/` is a compiled input** and any currency scope that omits it is wrong.
- **A REBUILD LEAVES THE PAIR HALF-PRESENT FOR ~61 SECONDS** (dc, mechanism driven from source). `cargo clean` removes both binaries before the build, so **`intent` can be absent while `intentd` sits there alone** -- a state no single read distinguishes from a broken install.
- **HOP 1 WRITES v2's CANON OVER THE SHARED `~/.claude/skills`** (devbin-vc, reproduced by construction). The frozen v2 `upgrade` reaches `propagate_canon_skills` unconditionally, **so every hop 1 on a project below 2.19.0 rewrites all 25 installed skills with v2's versions -- and reports `0 updated` at rc 0 while doing it.** **STILL LIVE FOR CONFLAB**, which is the one estate still on 2.19.0.
- **`~/.local/bin/intent` IS A SYMLINK INTO THE SHARED BUILD OUTPUT.** A build IS the delivery; there is no install step and no window in which the fleet is on a previous binary.

### The selection, stated because it was a judgement

**I selected on the bullets' OPENING CLAIMS rather than by reading all 212 in full.** An opener is the bullet's claim, so the selection is defensible -- **but a durable fact carried under a narrative headline would have been missed, and I cannot say none was.** The archive is the safety: nothing is lost, only demoted.

## PORT LEDGER -- CLOSED (Conflab landed 2026-08-28; re-derived from the estates)

**THE PREVIOUS LEDGER WAS STALE AND SAID SO ABOUT THE WRONG THINGS.** It listed **Lamplight and Intent as `NOT STARTED`** -- both are on `3.0.0` and Intent is self-hosted on it -- and gave **Conflab's blocker as a `legacy.rs:1844` byte-index panic**, which was fixed. **A ledger of remaining work that names finished work as remaining is worse than no ledger**, because the reader plans against it. Same class as `intent/restart.md`'s expired routing hazard, in the document that says what is left.

**GROUND TRUTH, read from every `intent/.config/config.json` on this machine rather than from any record:**

```
on 3.0.0   Anvil Baize Cdsync Conflab Courses Devbin Intent Laksa Lamplight
           MicroGPTEx Molt Molt-flynn Molt-matts Prolix Riffle Utilz   (16 -- every project)
on 2.19.0  Intentv2  -- FROZEN by hv's standing rule; correct, leave it. It is the tool tree, not a project.
```

**CONFLAB LANDED 2026-08-28 12:38Z: `b02b93c4` (hop 2 + hop 3, one commit, 170 files) and `7652c9b4` (HAND-FINISHED: the four live threads' pre-hop `acceptance.md` carried as `acceptance.v2.md` attachments from the rollback sha `cd09d711`, plus a `.prettierignore` fence for `*.v2.*`).** conflab-cc's seven residue rows were repaired at Conflab `40cc789d` the night before; ingest read 0 blocking / 44 carried, and the 44 reconciled to the row post-hop (23 unknown-status + 7 unknown-scope + 14 field-not-recorded). Method, in order: Phase 0 clean rollback point; a FULL rsync copy including `.git/` run through the whole hop under a seeded decoy `HOME` (intent-dc), which found the two things the real run then hand-finished; census from the markdown (conflab-cc); whole-tree sha256 before/after plus a prose-loss probe with sentinels (intent-cc); the real hop by conflab-cc under conflab-vc's pen with the rehearsal's changed-path set as the STOP condition (it matched, 39 vs 40, the difference being the now-ignored `events.jsonl`); Phase 4 by every node read-only, and the one instrument that separates reached from biting -- conflab-ic's two-arm guard control on the live hook. **What the hop cannot fix and what the day filed: Intent issues 0100-0106.**

**AND ONE LIVE HAZARD SPECIFIC TO CONFLAB BEING ON 2.19.0: hop 1 rewrites all 25 shared `~/.claude/skills` with v2's versions and reports `0 updated` at rc 0.** Anyone running a v2 `upgrade` there clobbers the fleet's skills silently.

**WHAT THE PORT COST, KEPT BECAUSE IT IS THE ONLY PLACE IT IS RECORDED:** the migration dropped AT and AC rows per-thread rather than wholesale; the re-conversion was a tested script rather than a hand pass; three classes of lost citation across 19 rows fleet-wide; and `arca_cli`'s three PROSE `covers` clauses, which the v3 id gate cannot resolve, were repaired in the SOURCE because no reader fix reaches them.

## 2026-08-28, THE CONFLAB HOIST AND THE BOUNCE -- FOLDED BY CLASS

**The flat stub this replaces is in git at `c7d87712`; nothing from it is dropped. What happened: hoist `b02b93c4` + prose carry `7652c9b4` + guard proof `2b770740`, verified five nodes / three estates (census 123/531/133/141 exact, whole-tree manifest deleted 0, zero authored loss, binary `1dd65db8` end to end); hv's four bounce rulings at `3d5a710e`; AC-00.8 amended `1098ac0f`; `Superseded -> Cancelled` landed `4479264f` (source-only until the pair rebuild); 0110's first slice `f02fb55f`; the two WP-02 fiat-closes routed to conflab-vc; issues `0100`-`0113`.**

### The rehearsal is the instrument, and its changed-path set is the hop's STOP condition

- **ONLY A FULL COPY INCLUDING `.git/` TESTS THE HOOK DOOR** (intent-dc). A clone has no hooks, so it reports a clean install into an empty door and could not have reported anything else. The copy found both things the real hop had to hand-finish: `events.jsonl` unignored (0101) and the re-render dropping 147 lines of authored `acceptance.md` prose in the four WIP threads, including hv's own AT-03.1 retirement record. The rehearsal's 40-entry changed-path set became the real hop's STOP condition -- matched at 39 vs 40, the delta being the now-ignored `events.jsonl`.
- **`canon::hooks_dir` IS DOOR-AWARE** (`git rev-parse --git-path hooks`): on Conflab, `core.hooksPath` unset means `.git/hooks/`, and hop 3 replaced the 5090-byte v2 carrier in place with the 7332-byte shim. dc's alarm that apply writes `.githooks/` was Intent's own topology read as the function's fixed behaviour -- withdrawn by dc.

### A reporter reads the CARRIER and expects the GATE's properties -- one family, three instruments, and a time-variant

- **doctor's two** (0105 + 0106): `doctor.rs:1086` tests the carrier for a marker the shim never names; `:1203-1205` compares it to the gate template while canon installs the shim; `:1065`'s own doc comment states the assumption both checks violate. Every shim estate reads RED on the one command everyone runs, and a partial fix leaves it red.
- **guard_home_check's installed-copy arm** (0113, found closing this fold's own banked loose end): it greps the carrier for the gate body's `GUARD_HOME` assignment, so the current shim reads as "predates this template" forever -- and the printed remedy reinstalls the shim. **The NOTE printed on every commit the same afternoon the remedy verb was run.** Driven to both verdicts before filing: carrier 0 occurrences by construction, gate body 1, `--where` resolving this repo's gate at state OK.
- **The time-variant** (0112): `view_skew_check.sh` prints "their check is thread_view_skew_check.sh and no gate runs it. This is a GAP" -- true until dc gated the sibling on the 27th, now printed one section ABOVE that sibling examining 288 views in the same run. dc rightly ran the check by hand on this message's instruction while it was true; today it instructs readers to re-run what the gate just ran. **A permanent alarm and an expired one are one class at two timescales: the message is not re-derived when the mechanism moves.** Same day, same file family, the `claude skills sync` hazard in `restart.md` expired the same way and got the same treatment.

### A constant metric guards nothing; reach is not bite; agreement is not corroboration

- **A CONSTANT METRIC GUARDS NOTHING** (conflab-dc, via conflab-vc). "0 guard terms in the carrier" was cited pre-hop as evidence guards were absent and would have read 0 post-hop too -- the carrier is a locator, the roster is a layer down. Conflab's wip.md:37 prediction sat unfalsifiable for weeks under it. `guards: 4 ran` proves reach, never bite; only a refused commit proves bite.
- **TWO INSTRUMENTS AGREEING WAS THE TRAP TWICE** (intent-ic, intent-cc): a true premise ("names no guard runner") plus a stale pre-hop reading of a file replaced an hour earlier corroborated each other into "executes nothing", and it recruited the two nodes with the most standing to measure it. `--where` takes one second.
- **A CONTROL VALIDATES THE AXIS IT TESTS AND IS SILENT ABOUT THE ONE NEXT DOOR** (conflab-cc, then intent-cc, then conflab-vc, each in their own instrument within an hour): the prose probe's sentinels proved present-vs-absent and said nothing about normalisation symmetry -- 53 false losses; the WP counter's positive control passed on the thread whose column width it was tuned to. Filed as 0102.
- **A VALUE RETYPED OUT OF AN INSTRUMENT IS A SECOND HOME FOR THE FACT; A SECOND NOTATION DERIVED FROM ONE CALL IS NOT** (intent-dc): `751` beside `-rwx--x--x`, `11` failures beside a self-test that says 14, a 9/9 in a HIGH issue's table where three counters say 6/4. Print both notations from one read.

### Authority: a relay is not an authorisation, and the chain transmits what it fences

- **A RELAYED AUTHORISATION IS NOT ONE, AND IT COST TWO HOURS** (all four conflab nodes, correctly): each session needed hv's yes in its own terminal; the chain stalled on that, not on work, until hv routed Phase 0 through conflab-vc. The rule held under exactly the pressure it was written for -- the first relay today that would have STARTED something irreversible. The D2 fiat-close was routed the same way this afternoon: hv's ruling relayed AS a relay, named as vc's report, with conflab-vc's own session consent left to conflab-vc.
- **THE AUTHORITY CHAIN TRANSMITS THE METHOD IT IS MEANT TO FENCE** (devbin-vc + devbin-cc, jointly): a node that must direct another's unit cannot be blind to it; pre-registration bounds only what was predicted; the whiteboard has no information barrier and every mechanism it has is a broadcast. Held for hv as a protocol question, not built.
- **vc's OWN**: a cost put to hv ("Conflab is debug-linked") was dc's finding from the previous morning, expired within the day and carried; corrected on hv's board in place. "Only that view changes" announced a `--to-disk` whose reach is the whole extract (intent-ic). A Phase 4 spec ("expect 123 in all three places") would have read every dehydrated thread as data loss (intent-ic). A "cold clippy, minutes" warning was inverted -- no `.rs` is staged by a hop, the gate returns early (intent-dc).

### A claim outlives its basis -- and the repair, exercised three ways in one day

- **AC-00.8 amended to the ruled behaviour** (`1098ac0f`, hv's D4): "settings.json unchanged" was true of the pre-canon era and false of the ruled procedure; one clause became two, each scoped to the hop it is true of, AC-10.4's migration-verb byte-identity untouched. The repair names what was true, when it expired, and the measured shape (`b02b93c4` carries the rewrite) -- corrected forward, never rewritten.
- **THE SHARED INDEX, AGAIN**: intent-dc's evidence fold to 0105 rode into a peer's commit titled "correct the gate row" (`1104d7e8`) -- nothing lost, a commit claiming content its message never describes, recorded forward in `1d1e0f4f`. And `7fa3c013` is the inverse: an EMPTY commit whose message claims content -- the carrier gitignored, prettier collapsing the renderer's 12 blank AGENTS.md lines inside the commit window (0110; cc reproduced it in isolation). `add && commit --only && reset` is the standing form; the reset is what keeps the index honest under a staging hook (intent-ic measured it).
- **A STATUS VOCABULARY DEFECT IS REPAIRED AT THE MODEL, NOT AT THE INSTANCE** (hv's D3 sequencing, ruled twice, both first-hand): fix Intent first (`4479264f`), repair client projects after, and the two questions the mapping does NOT answer -- whether `Deferred` legitimately reads as not-started, and the structural `status_legacy` mirror -- each get their own ruling rather than riding the shipped one.

## 2026-08-27, THE HANDOVER MORNING -- FOLDED TO WHAT IS NOT RECORDED ELSEWHERE

**Twelve bullets. Most are now rules on `vc/wip.md` (hand over properties never values; a boss node's relays need the rule MORE; the marker is provenance not identity) or conventions in `intent/restart.md` (a build IS the delivery; the marker names the last commit touching a compiled input). What is kept is what those do not carry.**

- **`jq -r '.x[]?' | wc -l` COUNTS LINES, NOT ELEMENTS** (lamplight-vc). `-r` pretty-prints each object across one line per key, so 80 attachments x 6 keys reads as **480**. **The failure mode is that 480 is a perfectly plausible attachment count** -- not an error, not out of range, and nothing in the output says which it is.
- **PATH PRECEDENCE IS NOT A REGRESSION AND LOOKS EXACTLY LIKE ONE** (cc). After a `brew link`, a fixed blocker reappears -- verb rc=2, a flag gone -- **with nothing lost**: the dev symlinks are intact and still point at a build carrying everything. `/opt/homebrew/bin` simply won the PATH. **The symptom is identical to the fix having been reverted.**
- **UNLINK ALONE DOES NOT HOLD -- RE-PIN** (devbin-vc, adding the step vc's relay of hv's ruling omitted). The formula's PIN is the durability mechanism: **`brew upgrade` reinstalls and RE-LINKS**, silently putting the fleet back on the keg. **An unpinned unlink expires at the next upgrade and nothing announces it.**
- **RIGHT-BUT-INSUFFICIENT** (vc on devbin-vc). They measured seven guard bodies byte-identical, saw `pre-commit.sh` differ, and stopped at _does not affect today_. **The unasked question was what it affects TOMORROW: every `intent upgrade` in any estate would install the pre-ruling dispatcher.** A correct measurement, a correct conclusion about the present, and the wrong question.

## THE EVENING OF 2026-08-27 -- FOLDED BY CLASS, NOT BY CHRONOLOGY

**Twenty-five chronological entries collapsed here. Every attribution and every distinct MECHANISM is kept; the retellings are not. Full text at `.history/20260827/cutover-runbook-fold-2209Z.md`.**

### What shipped

Pair **`d395a5b5`**, current by property, **verified green at the commit it NAMES rather than at HEAD** (ic: 1374 passed, 0 failed, three independent readings, awk control seen to fire three times). Two windows, 54s and 69s, one node building, peers told before and after by properties.

Porter's **both** citation defects (`e935734d` + `eff618e8`), clock tolerance 0 (`3463f784`), the third stamp surface (`27b13f93`), AC-11.6 with a pty harness the estate never had (`102af78f`), AC-14.7 as one transaction (`05222011`), the `--to-disk` remedy **and its false premise** (`04bc607f`), `--severity` enforcement (`8174de80`), doctor's gate check (`3805f359`, `41c3e3f1`), the doc gate (`6c380e09`), (A2)'s body **inert** (`3b0063f3`), (B) into `apply` (`f8a78e05`, `22a75509`), the isolation fix (`9c2ba9ed`).

### The structural finding: the sweep hv reserved does not exist

**No v3 code path wrote `pre-commit.intent`.** Ruling 4's arm was present in the template and absent from every estate carrier **including this tree's** -- positive-controlled at 1 vs 0. Guard BODIES and the ROSTER propagate live from `INTENT_HOME`; only the carrier was frozen at install.

**dc by `cmp`, not by size: eleven estates carried `Intentv2`'s template BYTE-FOR-BYTE** at 26 Aug mtimes; Conflab's was installed 14 Aug **from a template already a month stale** (conflab-vc). **The installer ran -- it read the wrong source.**

> **The guard BODIES move with no ceremony available; the CARRIER cannot move at all. Both halves are one asymmetry.** (devbin-vc)

**That framing is what made hv's menu decidable.** hv ruled **both (A2) and (B), (A2) first**; then, on dc's measurement that installed carriers still run the critic themselves (13 invocations each, so rostering doubles every finding in fifteen estates), re-ruled **(A2)'s body lands INERT and the roster line waits on (B)**. cc's caution shaped it: the roster left the carrier at `042985c8` **because bodies propagated and the array naming them did not**, so a refusing arm going back in rebuilds that precondition -- **(A2) moves the same direction the roster went, the inverse of the incident rather than a repeat.**

### THE CLASS: a true result from an instrument that could not have answered differently

**Fourteen instances, five nodes, one evening. Listed by MECHANISM because the mechanisms differ and the lesson does not.**

| mechanism                                     | instance                                                                                                                                                                            |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| pattern that cannot match                     | dc's sweep grepped `pre-commit`, which carries the gate in **no** estate                                                                                                            |
| tautology as prediction                       | vc: a guarded estate **cannot** carry a post-guard violation -- the guard refuses it                                                                                                |
| corpus censored by the instrument under test  | dc: the guarded set **is** the set that passed the guard                                                                                                                            |
| green invariant under the change it certifies | dc: every check-A fixture is **+2 hours**, equally green at 120, at 0, and at an hour                                                                                               |
| correct aggregate over a wrong model          | cc: `12/62` was right and would have certified a fix leaving **32 rows broken**                                                                                                     |
| remedy whose scope exceeds its message        | cc: the bless rewrote **two** faces; hand-patching the visible one leaves the other drifted, suite green                                                                            |
| failure naming a cause it cannot observe      | cc: a **line-count ratio**, not a runaway delimiter toggle -- prose raises the denominator                                                                                          |
| ceremony answering a different question       | conflab-vc: three arms on a command that **cannot see** the variable                                                                                                                |
| a name read as a wire                         | baize-vc: `INTENT_HOME` exported and **inert**; the user had already driven it three ways                                                                                           |
| checklist as a lagging indicator              | conflab-vc: vc's pre-flight cleared every blocker the fleet had hit; `ingest` refused on three nobody had                                                                           |
| instrument that saw **too much**              | dc: 121 of 158 canon WPs "missing" files **is the DB-is-SSOT design**                                                                                                               |
| audit generated by the thing under audit      | devbin-vc: **a census whose denominator IS the pattern under test cannot report what the pattern misses**                                                                           |
| a green from a file with nothing in it        | dc: `sed` errored, the redirect created the file anyway, an **empty guard** returned rc=0 and was reported as a control passing                                                     |
| the subject changed underneath the test       | cc: `bootstrap` answered _not implemented yet_ until `431590a3`; **a probe for a refusal became a live setup command.** Every other unbuilt verb carries the same latent conversion |

**AND THE PIPE-rc TRAP HIT THREE NODES INDEPENDENTLY WITH NO CROSS-TALK** -- laksa-cc reading `sed`'s status, vc reading `head`'s, dc quoting `rc=0` for a `git checkout` that had printed `fatal:` **in a harness built to prove it failed.** Most traps spread by priming; **this one needs no transmission, so warning does not reduce the rate.** Mechanical defence only: capture the status **before** the cosmetic pipe. `PIPESTATUS` is a bashism, empty under zsh.

> **Four rounds of increasingly careful reasoning between three nodes moved us further from the answer. Nobody was careless; care was the wrong instrument.** (dc)

### THE COUNTER-CLASS: four nodes declined a green they were entitled to claim

**conflab-vc** refused to let vc's carrier prediction be scored -- _an unchanged file after an operation that did not happen distinguishes nothing._ **cc** declined to claim main was green from four passing targets. **dc** withdrew their own flake claim once `sort -u` proved the arm unstressable. **lamplight-vc** marked their guard reading **read-verified, not run-verified** -- a bar **cashed twice tonight, in both directions, unasked.**

> **The dominant class is not defeated by care. It is defeated by people declining results they are entitled to claim.**

### The remedy family -- four members, and the ambiguous one is worst

`bin/devbin hooks` named a verb that **does not repair what it reported**; the bless did **more** than its message showed; a red named a cause it **could not observe**; and `doctor` offers **two exits without discriminating them**.

> **A remedy that is correct and ambiguous is worse than one that is wrong, because nothing about it invites checking.** The wrong remedy fails loudly on first use; the ambiguous one succeeds.

**THE VACUOUS GATE** (laksa-cc's vc, after two wrong answers including dc's own): nothing was bypassed -- reinstate, start, done, every edge legal, **and `wp done` succeeded because the gate PASSED.** _A gate passing means every criterion IN SCOPE is satisfied; on a thin contract that is vacuously true._ **An empty contract is refused outright, so the hole is thin-but-nonempty: the gate blocks when there is nothing to check and passes when there is almost nothing.** Intent shows 0 findings, so nothing here to mis-remedy.

### Settled facts, still load-bearing

**`INTENT_HOME` resolution, on three legs, one of them execution:** the carrier parses one line of `intent info`; that line reads `Intent` in six estates under set/unset/forced-junk; **dc traced three installed carriers under `bash -x` and read the paths they OPENED.** Leg 3 cannot be blind because it is not a probe. **conflab-vc's refinement: a READ of the consumed value needs no sensitivity control, because there is no inference for insensitivity to corrupt.** hv repointed the pointer after ic proved `cargo test` was writing it; verified `state: OK` rc=0 with a negative control.

**THE LAMPLIGHT RE-RUN CRITERION, now arithmetically complete:** 1731 test rows, **74 broken, and 12 + 30 + 32 = 74 exactly** -- the whole of the broken population, not a decomposition anyone asserted. **Exactly those 74 to zero, split three ways, plus zero rows whose file value opens with a bracket. Never the 62.** More than 74 moving is a **regression signal**. The 30/32 sub-split is lamplight-vc's; vc verified only the coarse 12 + 62.

**AND THE STORE HAS NO RECOVERY SOURCE FOR THE 74.** `legacy.raw` sits on 1057 of 1731 rows and on **zero** of the 74 -- complementary **by construction**, being the other arm of one classification. **A resource whose coverage is precisely complementary to the need, and abundant enough to look like the answer.** The disk is the only route, now by measurement.

**Conflab did not migrate:** `intent ingest` refused on **7 blocking residue rows, all in `ST0121/acceptance.md`**. hv's `sync --to-disk` on Lamplight touched **zero `COMPLETED/` paths** -- verified two ways.

### vc's own errors, kept because the count matters more than any one

A tolerance ruling vc had told devbin-vc they would not make, **escalated correctly** and later ratified by hv as hv's own, with the **scope read explicitly held open**. Two Laksa claims damaged in relay. A peer's narrow green restated as a claim about the tree. A denominator from the wrong arm **while correcting an error of that kind**. A hypothesis built on a misdirecting failure message and passed on as a lead. A positive control that failed twice. A generosity that retired lamplight-vc's finding before it was understood. A stale "hv is AFK" repeated to five estates. **A hold ruled without any mechanism behind it** -- dc found it sitting dirty in a shared tree and parked it properly, with the rule: _"held for hv" is not a property of a dirty file._ A relay to cc that widened a measurement's **scope AND its timing** at once. And a slug spec whose load-bearing constraint was **backwards**, overruled by hv before cc built on it.

> **A hazard correctly identified from a mechanism wrongly assumed is a better failure than the reverse, because the hazard survives the correction and gets checked properly.** (dc)

**And a misroute caught from both ends at once:** lamplight-vc sent lamplight-cc's note to vc's socket; three of its claims **almost** fitted. _A misdelivered note that ALMOST reads as yours is worse than an obviously wrong one._

## Watch-out instances moved off the board, 2026-08-29 13:20Z

**The board's Watch-outs section says RULES ONLY and it had stopped being true again.** These are the full narratives for rules 22-27 as they stood at the 1318Z fold; the board keeps the rule and a one-line pointer. Moved rather than trimmed -- the evidence is what makes each rule citable, and it belongs here.

**22. Relay the instrument's OUTPUT, never a characterisation of it.** Measured 2026-08-28: an arm's `file EXISTS while the row says to-write` became `untracked in the tree` through two hops -- cc's phrasing, then my amplification with a check-your-commit framing -- inverting the claim and costing dc four commands aimed at the wrong property. The non-discriminating-falsifier rule's sibling: both are a relay substituting its model of the evidence for the evidence. NARROWED BY cc AGAINST THEMSELVES, and the narrow form is the keeper: the danger is characterising an instrument's output WHILE HOLDING A SECOND FACT about the same subject -- their `git status` sighting fused with the arm's finding into a third claim no instrument produced.

**23. For a TRACKED file, `git commit --only <paths>` with NO `git add` and NO reset -- the add is the half that sweeps, and `git reset -q -- .` resets the WHOLE index including a peer's staged work.** dc measured it live 2026-08-28: my board sat staged in the shared index, their `--only`-sans-add commit carried their file alone and left mine untouched -- and they checked WHICH explanation was true against git log rather than assuming the flattering one. `add` (then a reset scoped to EXACTLY the added paths, never `.`) remains only for NEW files. **AND THE LIMB THIS RULE DID NOT COVER, measured by cc at the cost of a commit (0134): `--only` TAKES THE WORKTREE, NOT YOUR STAGED VERSION.** There is NO way to commit a subset of a file's hunks while a peer is editing it -- a hand-staged `--cacheinfo` blob is REFRESHED AWAY by the gate itself before the commit -- so cc's commit carried two of dc's uncommitted hunks after they had verified with `git show :<path>` that it would not. **The only technique that works is a DETACHED WORKTREE.** Three nodes drive `--only` against a shared index daily and the new-files-only rule says nothing about this case.

**24. A REPO-LOCAL GATE ARM THAT READS THE WORKING TREE MAKES ONE NODE'S MID-EDIT EVERYONE'S REFUSAL.** Measured 2026-08-28: `corrected_check.sh:52` is `TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"` -- **the file on disk, no `git show`, no `HEAD:`, no index** -- so cc's UNSTAGED `0121` edit moving `st list` to `corrected` without `target.rulings` refused my commit of two paths, neither of them under `surface/`. For a solo repo that scope is right and probably deliberate; the failure is specific to a four-writer checkout. **THE RESPONSE IS NOT `--no-verify` AND IS NOT TOUCHING THE PEER'S FILE: it is to tell the peer their mid-edit is gating the checkout, and wait.** **CORRECTED BY cc AGAINST ME, 2026-08-28 18:23Z: my second claim here was WRONG and is struck.** I read `exit 2; its findings never gate` as 0093's shape -- an arm saying it cannot gate and then gating. cc read the RUNNER: `precommit:340-352` makes all five report-only by construction, exiting non-zero ONLY when they cannot measure at all, so for `corrected_check` that line is the runner DISAMBIGUATING -- not a finding, an instrument that could not answer -- and blocking on it is the correct polarity, because a green from an instrument that did not run is worth nothing. **cc then checked the SET rather than stopping at my instance, and found a real one: `rulings_check.sh` exits 1 on `BAD`/`DANGLING`, which are findings it measured perfectly well, while the runner prints `could not measure ... its findings never gate`. Every clause wrong for the case that produced it. Filed 0128.** So the sighting was right, my instance was wrong, and the class is real one instrument over -- which is exactly _a correct sighting with a wrong explanation_, committed by me and repaired by a peer who declined to inherit it.

**25. AN IDENTIFIER A TOOL ALLOCATES IS READ FROM THE TOOL'S OUTPUT, NEVER PREDICTED -- AND IN A SHARED CHECKOUT THE ALLOCATION ITSELF CAN COLLIDE.** Both halves measured 2026-08-28, both by me, in one sequence. **I typed cross-references into three issue bodies by predicting the next numbers**, filed them, and the numbers came out `0124/0126/0127` because cc filed `0125` between my two calls -- so `0124` now points at `0125` for "splice" and lands on cc's dispatch-table issue. **A wrong pointer that RESOLVES to a real issue about something else is 0088's class, and there is no `issues edit` to repair it (0090).** **And underneath it: `next_issue_number()` computes `max+1` from an in-memory snapshot taken at facade construction**, so cc and I allocated `0126` concurrently, the second write destroyed the first, and BOTH of us were told `created:` -- recoverable only because cc still had the text in session. Filed `0130` (high). **The narrowing matters and is the useful half: `apply_with_state` DIFFS against loaded canon rather than replacing it, so concurrent writes to DIFFERENT entities are safe** -- the loss needs both writers on the same key, which a stale-snapshot allocator guarantees. **The emptying guard did not fire because the overwriting write was not empty: it asks _is this emptying something_, and nobody asks _is this REPLACING something else_.**

**26. TWO ARTEFACTS DECLARED AS SECOND WITNESSES ARE A WITNESS PAIR ONLY IF BOTH ARE WATCHED -- AND THE UNWATCHED HALF IS USUALLY THE AUTHORITY.** Measured 2026-08-29 on `data-model.md`'s Machine 3 against `transitions.rs`. The tables are transcribed into code precisely so the two can be compared, **but only the transcription is machine-checked** -- `mutation_completeness.rs` and its siblings read the Rust, and every reference to the document in the whole source tree is a PROSE CITATION in a comment. **So drift cannot run both ways: it runs, by construction, in the direction where the RATIFIED document is the half that goes wrong and nothing can see it** -- which is the worse direction, because the copy is what implementers obey and the authority is what rulings land in. hv's fiat ruling reached the code the same hour and the table not at all; a peer's eye caught it, not an instrument. **AND THE REASON IT HAD NO OWNER IS THE SHARPER HALF: EACH HALF NAMED THE OTHER AS MASTER.** The page's status line says the Rust is the authored master (true of the SCHEMA FACE, which is what it was written about, and nothing on the page limits it to that); the code says the page is the ratified machine. **Neither side believed it was the one that had to be updated first, and both were reading their own file honestly.** **THE TRAP IN THE FIX: the two notations do not count the same things** -- one row per `(from, to)` here, one `Edge::` per verb-and-landing with a LIST of from-states there, so `ac.descope` is three rows and one edge. 14 rows against 10 declarations expanding to 14 pairs; **every count correct, none equal, so a detector that counts before it expands reds forever** (rule 6). Expand to pairs, then compare sets.

**27. A CENSUS TAKES ITS UNIT FROM THE SUBJECT'S IDENTITY, NEVER FROM THE FILESYSTEM'S -- AND SCANNING PER-PATH WHAT IS PER-THREAD INFLATES BY HOWEVER MANY HOMES THE SUBJECT HAS EVER HAD** (ic, against their own predictor, 2026-08-29). v2 kept threads in status-bucket directories and estates flattened them before hopping, so **one thread owns several historical paths, each holding a FROZEN SNAPSHOT from whenever it left that bucket.** Measured on Lamplight: **678 paths for 358 threads, 155 of them carrying 2-3 each.** The fleet aggregate I carried to hv was wrong by **3.2x -- 257 against 80** -- and Lamplight by 5.8x. **AND IT IS NOT MERELY DOUBLE-COUNTING, WHICH IS THE HALF THAT WOULD HAVE BEEN HARMLESS:** ic read the rows instead of trusting the delta and found `ST0052 AC-01.2` reading `satisfied: no` at a July bucket snapshot and `satisfied: yes` at the post-collapse blob -- **satisfied BEFORE the hop, and only the stale snapshot made it look destroyed.** A wrong verdict, not a duplicate. **THE CONTROL COULD NOT HAVE CAUGHT IT AND ITS PROVENANCE WAS PERFECT:** Conflab is UNCHANGED at 14 because its history produced no bucket snapshots, so the one estate with an independent in-estate measurement is the one estate the defect never touched. **That agreement was unarranged, cross-node, cross-method and genuine, and it was still blind** -- which is the form rule 1 cannot be defended against by picking a better control, only by asking what the control CANNOT see. Third instance in one day.
