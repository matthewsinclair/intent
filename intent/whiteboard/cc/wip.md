---
node: cc
name: Control Claude
role: control
session_id: 0bf64b47-09ab-4c8e-8e10-be9f54d29df7
heartbeat_at: 2026-08-19 20:17Z
status: paused
focus: "**FOLDED AND PAUSED (fold 3). WP-03 IS CLOSED AS WORK -- five rows green and the sixth built.** AT-03.1/02/03/04/05 landed; AT-03.6 `--staged` at `19268867`, byte-proven at the five episode commits, dc approved, and **the row waits only on a gating decision that is deliberately not mine.** **THE FINDING WAS BIGGER THAN THE ROW: every instrument that watches the wiring said it was correct at 11 gated, and only a planted divergence disagreed** -- a roster verifies a tool is DISPATCHED, nothing verifies that dispatching it MEASURES the right subject. **NEXT IS AC-00.4, CLAIMED AND FULLY DESIGNED ON THIS BOARD, NOT BUILT** -- the ROOT_FILES generator, template-not-heredoc, and deliberately NOT a view because a view under a thread dehydrates."
claims: [ST0056/10, ST0057/00, ST0057/01, ST0057/03]
---

# Control Claude (cc)

## THE MODEL -- canon, hv ratified

**D01 IS REVERSED: THE DB IS THE SSOT AND THE FILES ARE RE-CREATABLE.** Never cite the old "committed JSON durable / DB rebuildable / `rm` safe" wording; it is VOID. **D34: the committed extract is the interchange -- it TRAVELS while the DB never does.** D29: a gitignored path is never canon. **ST0057 is INSIDE the 3.0.0 gate** (hv).

**D42 -- TIME, and it has no clauses.** `date -u +'%Y-%m-%d %H:%MZ'`, read in its own step, trailing `Z`. `one_clock.rs` enforces it structurally and has caught me. **And reading it is not enough: I reported file times to matts with a `Z` while `stat` was printing LOCAL. The guard catches the stamp you write; it cannot catch the one you format wrong on the way out.**

## NEXT -- mine

**FOLDED 2026-08-19 20:17Z (fold 3). Today's settled narrative is at `.history/20260819/wip.md`.**

**WP-03 IS CLOSED AS WORK. Five rows green and the sixth built:** AT-03.1 `46ab2220`, AT-03.2 `49aa64db`, AT-03.4 `3c6d20f6`, AT-03.5 `18a9ab6b`, AT-03.3 `61b93440`, and AT-03.6's `--staged` at `19268867`. Also `239238df` (export typed errors) and `fb333464` (the daemon that could not open the store).

**1. ST0057 AC-00.4 -- CLAIMED, DESIGNED, NOT BUILT. This is the next build.** One of the eight unmet dehydration preconditions, and it was on NOBODY's claims until vc measured the union -- **each node tracked its own preconditions correctly and nothing tracked the whole.**

The design is decided, so do not re-derive it:

- **The prose goes in a TEMPLATE, never in the generator.** The project's own rule: _all generated content comes from `lib/templates/` via substitution; no inline heredocs duplicating template content._ v2's generator predates it and carries 19 sections inline across 835 lines of `intent/plugins/agents/bin/intent_agents`, with 15 language-conditional references.
- **Porting that prose into Rust is wrong twice**: it bakes v2's shipped-and-frozen content into the v3 binary, and it bakes THIS project's content into a tool every consumer installs.
- So: `lib/templates/llm/_AGENTS.md` as the skeleton -- v2 ships `_CLAUDE.md` and `_usage-rules.md` and no `_AGENTS.md` -- substitution on the four tokens the existing templates already use (**`[[PROJECT_NAME]]`, `[[INTENT_VERSION]]`, `[[DATE]]`, `[[AUTHOR]]`**), and language-conditional sections keyed on `config.languages`.
- **NOT joining `views::render_all`, and that is a hazard rather than a preference.** A `GeneratedView` under a thread DEHYDRATES; root files escape only because `thread_relative` returns `None` for them and `organize` skips them. Making them views puts three root files one classifier change away from removal -- and **`AGENTS.md` is the file this criterion exists because something already emptied it.**
- **AT-00.4 must NOT assert byte-equality against disk.** Those files are v2 v2.19.0 output and say so in their own footers. The property is _v3 can produce each of the three_, plus **the emptied-file case is detected**, that being the live instance the row cites.

**2. AT-03.6's ROW MOVE, waiting on dc.** My half is landed and byte-proven; **admitting the tool to the gate is dc's decision and vc's condition 3 says it is not mine.** Roster disposition untouched at 10 gated / 7 manual.

**3. REPORTED, NOT MINE, LIVE AT HEAD:** two `error_remedies` failures on dc's `FacadeError::Intentfiles`. Its `#[error]` is a fixed string that never interpolates the source, so every parse failure renders identically -- **and what it discards is the line number `.intentfiles`'s own header promises.** Same defect as the `unused variable: cause` warning, which was silenced with an underscore rather than fixed.

## Watch-outs -- the live set

**Folded 20:17Z. The full accumulated set is in `.history/20260819/wip.md`; these are the ones still unencoded and still biting.**

**A PEER'S RED CONCEALS MINE AND LOOKS EXACTLY LIKE AN ALL-CLEAR.** `cargo check --all-targets` aborts at the lib, so a peer's broken file means my own test targets are never compiled. I read four `facade.rs` errors, concluded _not mine_, and shipped `239238df` with a red target. **ic had named that exact observable to me the same morning.** matts found it by running the suite.

**A ZERO IS NOT A RESULT UNTIL THE CHECK HAS PRODUCED A NON-ZERO -- four instances in one day.** A short-circuited `cd` leaving `awk` on a stale file (a plausible `56 passed`); a `| tail` before `$?` reading the pipe's status; an empty grep after a cwd-drifted command; **a byte-identical comparison of two identical FAILURES, where every cell said IDENTICAL and rc=2 was the tell.** And a full-workspace run reporting `0 binaries` whose totals printed empty. **A run that reports no suites is not a run that reports no failures.**

**A DETECTOR THAT CANNOT TELL A SUBJECT FROM A MENTION TAXES WHOEVER DOCUMENTS THE REPAIR -- AND I BUILT IT WITHIN AN HOUR OF FINDING IT.** The critic reported my own doc comment as an instance of the defect it documented. Then my inbox broadcast tested for the `_(empty)_` sentinel with a SUBSTRING SEARCH, matched that token quoted inside my own earlier prose, and **wrote hv's inbox back unchanged at rc=0 -- silently dropping the human from two broadcasts.** The whiteboard header guard refuses to scan prose for precisely this reason.

**A STAMP AUTHORED IN THE SAME COMMAND THAT READS THE CLOCK IS FABRICATED.** Twice today: `16:45Z` for `16:38Z`, caught by the guard; `18:14Z` for `18:12Z`, caught by me. **D42 says read it in its own step, and that is the whole of the fix.**

**IN A FOUR-NODE CHECKOUT, PRESENCE IDENTIFIES A FILE AND NEVER ITS AUTHOR.** Three misattributions onto me in one day, each inferring the author from who was most recently active in the area. **The AT row that cites a file names its owner; `git log` and `git status` cannot, because the file is untracked precisely while it is in flight.**

**A COMMAND WHOSE BLAST RADIUS EXCEEDS WHAT IT WAS AIMED AT, RUN INSIDE A RIG.** `git reset --hard` to clear a planted divergence took my uncommitted prototype with it; scoped `git restore --staged <path>` was the right tool. dc did the same with `git stash pop` the same afternoon.

**A CONSISTENCY CHECK CONFIRMS THE PARTS AGREE WITH EACH OTHER AND SAYS NOTHING ABOUT WHETHER THEY AGREE WITH THE WORLD.** `runner_roster_check.sh` went green at 11 gated on a wiring that judged the wrong commit. **A roster verifies a tool is DISPATCHED; nothing verifies that dispatching it MEASURES the right subject.**

**A RECORDED REASON MADE FALSE BY A COMMIT SOMEWHERE ELSE, WITH NO INSTRUMENT WATCHING THE JOIN.** `canon_commit_check.sh` was held `manual` because there was no narrow attachment-sync verb; `sync --to-store <ID>` landed at `212b0075` and killed the reason. **A stale ROW is detectable because it cites a file; a stale REASON cites a condition, and conditions are not on disk.**

**`TZ=UTC git log --date=format:` DOES NOT RESPECT `TZ`. ONLY `--date=format-local:` DOES.** `format:` renders the commit's OWN recorded zone, so a fabricated-looking `Z` is produced by a careful command.

    format:        2026-08-19 08:31:17Z   <- the Z is a lie
    format-local:  2026-08-19 07:31:17Z   <- true
    date -u -r $(git log -1 --format=%at) <- independent cross-check

**A CRITERION MUST CLOSE EVERY DEGREE OF FREEDOM THAT LETS A PASSING TEST COEXIST WITH THE DEFECT** (vc): **INSTRUMENT** (which tool -- git and mtime give opposite answers), **DEPTH** (an internal subject reaches past the thing tested), **EXTENT** (a subset of the right kind, both figures observable so DEPTH passes it), **PIN** (measure at a named commit, never `HEAD`).

**ATTACHMENTS ARE DISK-FIRST; VIEWS ARE DISK-DISCARDED. SAME DIRECTORY, SAME APPARENT KIND, OPPOSITE DIRECTIONS, BOTH SILENT** (vc). Directly my lane, and the reason AC-03.4's remedy leads with _copy the file aside first_.

**EVERY COMMIT I MAKE TOUCHING AN ATTACHMENT LEAVES CANON DIVERGENT AT THAT COMMIT UNTIL A SYNC, AND A LATER SYNC REPAIRS THE NEXT COMMIT AND NEVER THAT ONE.** Proven on myself: I committed `--staged` into `canon_commit_check.sh`, which IS an attachment of ST0056, and **my own AT-03.4 caught the stale hash.** The compliant order is sync canon FIRST, then commit the file and canon together.

## Pending hv -- the ST0057 parity directory

**`intent/st/ST0057/parity/tools/` DOES NOT EXIST, AND THE FIRST TOOL WRITTEN THERE BECOMES ITS FORM BY DEFAULT RATHER THAN BY CHOICE.** ic proposed reviewing my first tool on the ground that ST0056's 43 siblings carry conventions the estate converged on slowly. **MEASURED, AND THEY DO NOT.** Denominator 43 (and it should be 39 -- four of the 43 ARE the libs):

    sources a lib_*.sh    SETTLED 7 of 39 (ic reconciled; command on record)
    # REACH heading       2-3
    [RECORDED: ...]       1
    DERIVED-BY:           1

**`lib_*.sh` is the ONLY inheritable precedent, and its real uptake is 7 of 39 -- SETTLED, not contested.** ic posted their command and decomposed it: `grep -lE '(source|\.) .*lib_[a-z]+\.sh'` leaked on **the word `SOURCED` inside a comment** (`lib_mdfmt.sh:4`, `lib_classify.sh:4`), and their strict reconstruction then DROPPED a real sourcer -- `coverage_map.sh:31` `. "$(dirname "${BASH_SOURCE[0]}")/lib_corpus.sh"` -- because they wrote `[^ ]*lib_` and that path contains SPACES. **Two failure modes, opposite directions, same author, same ten minutes.** My `.*` spanned the spaces, so both my reconstructions already held it. **Three independent routes now agree at 7.** **A DISAGREEMENT RECONCILED TO A COMMAND IS SETTLED; ONE RECONCILED TO A NUMBER IS NOT.** The single `[RECORDED: ...; DERIVED-BY: ...]` file is `of_n_labels_its_derivation.sh`, first appearing at `887345b1` **dated 2026-08-19 -- ic's own tool, authored TODAY**, cited by ic as the estate's slow convergence. ic self-diagnosed the circularity off my measurement; **the timeline makes it stronger than a sampling error, because there was no interval in which it could have become a convention.**

**MY OWN PROBES WERE WRONG TWICE IN THE SAME DIRECTION AND BOTH WERE LABEL-NOT-SUBJECT.** `grep -il reach` returned **27** (prose: "cannot reach", "UNREACHABLE"); the real REACH-heading count is 2. `grep -l 'lib_.*\.sh'` returned **14**; seven of those are the four libs naming each other plus **three files citing a lib as PRECEDENT IN PROSE**. **The 27 -> 10 -> 2 tightening happened BEFORE I reported, which is the only reason ic did not receive corroboration for a convention that does not exist.** Report after tightening, never before.

**THE ARGUMENT SURVIVES WITH ITS REASON REPLACED (fourth time today).** Not _a bad first tool deviates from an established form_ -- there is no established form. **In an empty directory the first tool IS the form.** So the "review" is not a review: it is ic and me deciding the ST0057 form once, deliberately, knowing 43 siblings largely did not converge, and ic should not supply it from a sample of one.

**AND THE ASSIGNMENT QUESTION IS NOT THE WP QUESTION** (ic, on the record at their own request). I claim ST0057/01, so AT-01.2 and AT-01.4 are IN MY SCOPE and there is no ownership dispute to adjudicate -- my handover to ic was itself the owner-from-kind error. **Assignment on the assignment's merits; ic's no-WP gap on the gate arithmetic; settled in the same breath, NEITHER DECIDING THE OTHER.** A structural gap that starts routing tasks has stopped being structural, and one that routes work to the node who raised it is the worst version.

**NOTHING IS WRITTEN INTO THAT DIRECTORY UNTIL matts RULES.**

## Standing rulings

- **THE 3.0.0 RELEASE GATE IS FULL DISK-TO-DB BIDI SYNC, WORKING AND USABLE. NOTHING ELSE CUTS IT** (hv, 2026-08-19, after saying it repeatedly to a board that had never once recorded it). **MEASURED ZERO: the phrase appears on exactly one of five boards, on vc's, as a QUESTION hv ASKED -- never as a directive -- and `hv/wip.md` has no `## Standing directives` section at all**, which the protocol provides for precisely this. **A DIRECTIVE WITH NO WRITE SURFACE IS A MEMORY, NOT A DIRECTIVE** -- the exact inverse of dc's `b645767a` finding that a write surface with no named reader is a queue. **AND MY OWN MORNING PLAN IS THE PROOF IT DOES NOT REACH ANYONE: I led with AT-10.12 and AC-10.13, which are `upgrade`, not sync, and nothing in the mechanism contradicted me.** Every node reads its own claims at pickup and the gate is in nobody's claims.
- **WHAT THE GATE ACTUALLY NEEDS IS MEASURED AND IS vc'S -- DO NOT RE-DERIVE IT** (`vc/wip.md` `## ON RESUME`). The round trip WORKS and is PROVEN: 12 of 18 WP-03 rows green, including canon -> DB -> canon (AT-03.8), `sync` naming its direction (AT-03.10), and content-hash external-edit detection (AT-03.3). **Three things decide usability and TWO ARE NOT ROWS AT ALL: (1) `intent sync` HAS NO SCOPE -- whole-estate only, and it has already ingested two peers' uncommitted instruments into canon TWICE IN ONE DAY; (2) AC-03.17's CHURN LOOP IS LIVE, HEAD 12 lines divergent from canon right now, so `doctor` reports zero skew TRUTHFULLY and is wrong minutes later; (3) `--to-store` is NOT read-only on disk.** **ALL THREE LAND IN MY LANE** -- `views.rs` is the renderer remedy and the `sync` scope is CLI surface. **(1) NEEDS ONE RULING FROM hv (consolidated item 14, still unruled); (2) NEEDS NOTHING AND IS MINE TODAY.**

- **EVERY ACCEPTANCE ID IS THREAD-QUALIFIED -- `ST00NN AC-XX.Y`, NEVER BARE.** Both threads carry an `AC-04.4`; four carry an unrelated `AT-03.6`. **An id resolving to a SATISFIED criterion is the worst ambiguity: it reads as DONE, which is the one failure mode that stops a reader looking further.**
- **THE ISSUE TRACKER IS FOR EXTERNAL USERS AGAINST A RELEASED VERSION** (hv). Everything found building v3 is work.
- **An uncarried file is NOT a disposition** (vc). **A REFUSAL IS RETIRED BY THE CHANGE THAT EXPIRES ITS REASON -- and re-deriving the reason is how you find out the reason was wrong.**
- **`treeindex` and handover RETIRE** -- a retired command is PRESENT AND REFUSING. **`EdgeKind::Incidental` STAYS with no user. `doctor --fix` is WITHDRAWN. `Outcome` is deliberately NOT `#[must_use]`.**
- **ANNOTATE, NEVER SUPPRESS, on a run verdict.** **v3 stays OFF PATH until dc repoints `~/.local/bin/intent`.**
- **`config.json` DOES NOT MOVE WITH `intent_dir`** -- `Project::config_path` always answers `intent/.config/config.json`, because something must be findable before anything is configured.

## Lane boundary

`native/**` and the v3 crates are mine. `bin/**` is not vc's to edit. The parity harness is ic's; `canon_commit_check.sh` and the pre-commit roster are dc's. **Canon writes route through vc.** `CARGO_TARGET_DIR=native/rust/target/cc` **for tests only** -- a release build must keep landing at `native/rust/target/release/` where four nodes read it, and nothing under `target/cc` is an artefact anyone else reads.

**Every commit I make touching an attachment leaves canon divergent at that commit until vc syncs, and a later sync repairs the NEXT commit and never that one** -- commit and ping vc; it is AC-08.5's missing operation, not my failure.
