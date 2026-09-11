---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-11 17:18Z
status: active
focus: "LOCALFOLD 2026-09-11 17:18Z FOR THE COMPACT -- not a release. AC-00.6 BANKED, NOTHING ON MAIN: critic --rules fix (S) + two v2 test deletes in wt-cut, rulings B-G not yet applied, critic_config 6 with vc. On the bounce: WAIT for vc's word that dc's prune is on main. Suites only in a private worktree under an isolated HOME."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

**LOCALFOLD 2026-09-11 17:18Z FOR THE COMPACT -- not a release. NOTHING OF MINE IS ON MAIN OR DIRTY IN THE SHARED TREE.** The pre-fold board is at `.history/20260911/wip-prefold-1650Z.md`; the rest is in `git log`.

**AC-00.6 (THE CUT), vc's ROUTING, hv CONFIRMED LIVE -- BANKED, NOT LANDED.** My eight mixed bats files. **LANDING ORDER (vc): vc's canon commit, then dc's prune, then my critic fix (its own commit, `AC-00.6` and `critic --rules` in the subject), then my test commit (`AC-00.6` in the subject), each rebased. On the bounce WAIT for vc's word that dc's prune is on main.** Send vc both shas and the per-file counts.

- **Worktree:** scratchpad `wt-cut`, detached at `e70b667f`, v2's `bin/intent*` deleted locally (unstaged -- never part of a patch), its own release build at `native/rust/target/release/intent`. Run bats as `HOME=<scratchpad>/hcut INTENT_BIN=<wt-cut>/native/rust/target/release/intent bats tests/unit/<f>.bats`.
- **Critic fix, DONE, came in at S:** `Library::at(root, ext)` in `rules.rs` (`new` builds through it); the critic roots at `--rules <dir>` and refuses a non-directory by name. Patch: scratchpad `ac006-critic-rules.patch` (sha256 `d208f18fc63598ea`), applies to main.
- **Deletes, DONE (vc confirmed):** `critic_arming_census` "absent tool: a tool-armed rule reports NOT RUN rather than passing quietly" (v2 critic wording); `no_absolute_home_paths` "the canon engine has no INTENT_HOME substitution left to reintroduce it" (greps the deleted v2 `intent_claude_upgrade`). Patch: scratchpad `ac006-cc.patch` (`4c04cc36446f2377`).
- **vc's rulings B-G, NOT YET APPLIED:** (B) `critic_report_format` 5 and 6 read `.findings` (and `.rule`, not v2's `.rule_id`; 6 asserts `.findings == []`). (C) DELETE `claude_md_template` 8, naming `agents_sync_parity.rs::the_four_rule_index_is_byte_identical_in_every_template_that_carries_it`. (D) `claude_md_template` 14: git identity `user.name TestUser` in the fixture instead of `INTENT_AUTHOR`, assert TestUser in CLAUDE.md and config -- if v3 honours no author source, STOP and tell vc. (E) the `co_language_code_guard` fixture to an IN-CO id no canon rule declares. (F) `devbin_seal_disagreement` 9's grep to `resolve:513`'s current form, keeping no-elif. (G) DELETE `ext_seed_validity` 16, citing `0177`.
- **With vc for ruling:** `critic_config` 6 went red once `--rules` worked -- its fixture `config.json` lacks `intent_version`, `Project::discover` errors, and the critic's `.ok()` drops `.intent_critic.yml` silently. Fixture or product.
- **Counts now (fix + two deletes):** claude_md_template 12/2, co_language_code_guard 3/1, critic_arming_census 18/0, critic_config 9/1, critic_report_format 5/2, devbin_seal_disagreement 8/1, ext_seed_validity 17/1, no_absolute_home_paths 6/0. Each must end N/N.

## TODO

- **Empty.** Read the lane column in `intent/wip.md`, never a copy here.

## Holds -- mine, with the CONDITION that releases each

- **POST-CUT (culled from the 3.0.1 loop 2026-09-11):** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; none is 3.0.1 work.

## Watch-outs

**FOLDED AGGRESSIVELY 2026-09-11 FOR THE 3.0.1 BOUNCE: TWELVE FAMILIES KEPT, EACH CUT TO ITS RULE AND ITS BOUNDARY.** Nothing was merged away, and the full text with every instance is at `.history/20260911/wip-prefold-aggressive-0917Z.md`. **Read the family there before arguing with it.**

**W1. THE INSTRUMENT ANSWERED A QUESTION ADJACENT TO THE ONE ASKED, AND ANSWERED IT CORRECTLY.** The dominant family; it arrives while I am being careful. **NOT AN INSTANCE:** a claim that names the FIELD read and the INSTRUMENT that read it, driven to both verdicts on a subject that can exhibit the failure.

**W2. CONTROLS, OR THE READING IS NOT EVIDENCE.** A control that cannot distinguish _safe_ from _never tried_ is not a control, and it must vary the axis the check reads. **NOT AN INSTANCE:** a control that would FAIL under the broken instrument, over a fixture that can tell the mutated quantity from its neighbours.

**W3. POPULATION, DENOMINATOR AND SAMPLE SIZE.** A precision figure is a claim about the CORPUS. **NOT AN INSTANCE:** a count whose population is stated, whose exclusions carry their reasons, and which states n and the variance.

**W4. ONE NAME, TWO ARTEFACTS.** A NAME resolves to TWO artefacts, a check examines ONE, and the claim is phrased about the NAME. **NOT AN INSTANCE:** a verdict that names the artefact it examined.

**W5. THE SHARED CHECKOUT.** Canon cannot be split, so every canon commit is silently multi-node. **NOT AN INSTANCE:** a change confined to a private worktree, or to a file no peer has touched, verified by looking rather than assumed. **THE ONLY SAFE WRITE:** `git add` your paths, then `git commit --only` them, retry the SAME command against a peer's index lock, never remove the lock, and judge success by `git log`, not by the loop. **A test run here compiles peers' UNCOMMITTED edits too** (2026-09-11: I pinned a red on `6e478ec4` by recency, and it was dc's mid-edit `render.rs`; the wrong sha now stands in `86071c36`'s message). Attribute a red only after re-running with `git status --short native/` empty. **AND RUN THE intentsvcs SUITES ONLY FROM A PRIVATE WORKTREE** (2026-09-11): `attachment_drift_detected.rs` runs `doctor` on `repo_root()`, `doctor::db_checks` calls `Store::open` on that project's store, and `Store::open` migrates. `repo_root()` is baked in at compile time (`testkit/src/lib.rs:128`), so a test binary built in the shared checkout reaches the live store from any CWD. My run with rung 18 in the tree moved the live store 17 -> 18 and locked every 17 binary out; hv recovered it by a rebuild. The vector is in `d0777bc8`'s message. **AND UNDER AN ISOLATED `HOME` TOO** (same day): my `cargo test -p intent-cli` in that worktree ran `intent bootstrap` in-process under the real `HOME`, and `publish_home()` repointed `~/.intent/home` at the worktree -- remove the worktree and the pre-commit shim refuses every commit in the repo. ic caught it; hv restores the pointer, never me.

**W6. A CLAIM THAT DOES NOT FEEL LIKE A CLAIM IS THE ONE TO DRIVE.** **NOT AN INSTANCE:** a premise driven in the same turn, against an artefact of the right era.

**W7. THE RIGHT ANSWER WAS PRESENT, CAPABLE, AND NOT REACHED.** **NOT AN INSTANCE:** a question answered after reading the artefact that OWNS the decision, not only the ones that describe it.

**W8. A PEER CHANNEL HAS NO TERMINATING CONDITION, AND DELIVERY IS NOT THE WRITE.** **NOT AN INSTANCE:** an exchange with a stated terminating condition, whose delivery was confirmed with the asker.

**W9. CANON, THE DAEMON, AND POSTCONDITIONS THAT ARE TRUE OFTEN ENOUGH.** **NOT AN INSTANCE:** a canon write verified PAST the ingest on a structured read of the value. **The read-verify-retry loop is a REQUIREMENT:** a running intentd can revert the last write of a burst about a second after it reports ok, and `event_log` is the only provenance surface.

**W10. THIS BOX AND THIS SHELL.** **NOT AN INSTANCE:** a command whose quoting and exit code were driven rather than assumed. **An exit code that IS the finding never goes through a pipe** (`if cmd | tail` tests tail's status). **The Bash tool's shell is zsh:** unquoted `$var` does not word-split and an unmatched glob aborts the command, and both return a plausible silence rather than an error.

**W11. FOLDING THIS BOARD.** **NOT AN INSTANCE:** a cut keyed on EXECUTION, with the pre-fold banked first and the asker confirmed.

**W12. A PRECONDITION THAT IS CORRECT CAN STILL BE A MIGRATION, AND THE ONLY WAY TO TELL IS TO COUNT.** **NOT AN INSTANCE:** a fix whose blast radius was counted across the real population BEFORE it was proposed.

## Decisions -- rulings in force and the lessons of this bounce

**CUT 2026-09-11 16:50Z TO RULINGS STILL IN FORCE AND THIS BOUNCE'S LESSONS.** Everything else is verbatim in `.history/20260911/wip-prefold-1650Z.md`.

- (2026-09-11, cc) **A HOLD'S CONDITION NAMES THE ARTEFACT IT WAITS ON, NOT A STRING.** The working form is `git log --since=<when> --grep <id> -- <path>`.
- (2026-09-11, cc) **A FIX THAT REMOVES A REFUSAL MUST FIRST ASK WHAT THE REFUSAL WAS PROTECTING.** `0084`'s UTF-8 refusal was the only thing stopping canon from naming a sidecar no door wrote; driving the fix through the verbs, not just the unit, is what showed it.
- (2026-09-11, cc) **A SUITE THAT OPENS THE ESTATE IT LIVES IN IS A WRITER, AND A TEST RUN IS A DEPLOY TO IT.** Twice in one day: the live store's rung (via `doctor`) and `~/.intent/home` (via `bootstrap`). Isolate the repo AND `HOME`, and read both back after the run.
- (2026-09-09, hv) **EVERYTHING SHIPS AS 3.0.1. NOT MINE TO RE-OPEN.** The new-surface argument is dead, not deferred.
- (2026-08-31, hv) **`config`, `ext` AND `learn` SHIP DECLARED-AND-UNBUILT IN 3.0.1.** A general ruling stated later does not vacate a specific one.
