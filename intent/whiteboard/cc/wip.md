---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-11 17:13Z
status: active
focus: "AC-00.6 (the cut): my eight mixed bats files. 2 v2 tests deleted in a private worktree, 13 v3 differences with vc for ruling; lands as ONE commit after dc's prune, rebased. Suites run only in a private worktree under an isolated HOME."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

**FOLDED 2026-09-11 16:50Z FOR THE COMPACT -- not a release. HOLDING WHILE vc CUTS 3.0.1.** The board before this fold is verbatim at `.history/20260911/wip-prefold-1650Z.md`, and every fix is in `git log` under its id. Every cc item is closed, re-driven by vc before each close (`0066`, `0084`, `0100`, `0145`, `0146`, `0153`, `0176`, `0231`), and hv's decisions 4 (`7d3ffe61`), 5 (`32958364`) and 7 (`56322937`; AC-01.7 satisfied at `23bca1b1`, notarised as `afbcaa0b`) are verified. The list has no workable row.

**AC-00.6 (THE CUT), ROUTED BY vc, CONFIRMED BY hv LIVE:** my eight mixed bats files -- `claude_md_template`, `co_language_code_guard`, `critic_arming_census`, `critic_config`, `critic_report_format`, `devbin_seal_disagreement`, `ext_seed_validity`, `no_absolute_home_paths`. Driven in private worktree `wt-cut` (v2 deleted locally, its own release build, isolated HOME). **Deleted (v2 subject):** `critic_arming_census` test 9 (v2 critic wording), `no_absolute_home_paths` test 4 (greps the deleted v2 `intent_claude_upgrade`). **With vc for ruling:** 13 tests in groups A-G (`critic --rules` not honoured; JSON shape; the template's rule index; `INTENT_AUTHOR`; an IN-CO fixture id collision; a stale devbin grep; `ext` unbuilt). Patch banked at scratchpad `ac006-cc.patch`. **Lands as ONE commit, `AC-00.6` in the subject, only after dc's prune is on main** -- then send vc the sha and the per-file counts.

`wt-cc` is removed: vc restored `~/.intent/home` on hv's word, and it reads the main checkout.

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
