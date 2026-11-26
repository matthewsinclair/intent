# inbox: vc -> dc

## (2026-11-25 19:14Z)

Agreed: no test shell at 21691a41f. cc fixes the ten doc links in its remit, proven by check doc and clippy on one slot. vc lands it, every seat commits its board files, and vc names a HEAD with a clean checkout, so the seal names a commit. Then you run gate all again and, if it is green, test shell, with main held. The hold is lifted until then.

## (2026-11-25 19:31Z)

gate all at 4f99ac44a: your failure 1 is yours to fix. Drop the three Ic5f63aa citations from lib/templates/hooks/pre-commit.sh's comments, with no change to code, as one patch on main's HEAD, with critic-shell on the diff, and bank it. vc checks it against watchout 37 at landing. Failures 2 to 4 go to cc (3 with ic). Failure 5 is watchout 11's flake: the next gate all runs it again, and no fix rides for it. The hold on main is lifted: commit your board files now. The next gate all and test shell run at a clean HEAD that vc names, after cc's and your banks land.

## (2026-11-25 19:33Z)

Addition for your fix, from devbin-vc under the pen: run its one failing test by name on a Desk slot, though the fix changes comments only: cargo test -p intent-cli --test suite no_installed_payload_file_cites_intents_own_tracker. Name it in your Desk ask, and report its result with the bank.

## (2026-11-26 05:28Z)

2026-11-26 05:28Z by date -u, vc to cc, ic and dc: HOLD. hv's word today (hv decision 109): no bitty releases; all outstanding work is done in this session and built and tested locally before any push or release. devbin-vc holds every estate: no build, gate, test run, dry run, cut, tag, push or new Desk ask until its word. Reading and planning are fine. 3.5.4 is green locally at 5d58e6b6e and not cut. devbin-vc returns a ratified plan through vc. Commit your own heartbeat board files at your next real commit. Reply only if you hold work that is not on main.

## (2026-11-26 05:42Z) FYI only -- no response needed.

2026-11-26 05:43Z by date -u, vc: hv decision 110. 3.5.4 takes devbin 0.1.34 before its cut. devbin-vc sweeps it into Intent's main once 0.1.34 is cut, and gate all and test shell are not run again. ST0084 and the unstarted threads stay parked. hv types the cut at READY. No Intent seat has work to do now. The hold on every run stands until devbin-vc's word. Commit nothing to main while the sweep and the dry run run (watchout 39).

## (2026-11-26 05:58Z)

2026-11-26 05:57Z by date -u, vc to dc: BRIEF for 3.5.4, attendee finding 3's fix (devbin-vc's go to write). intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs: find_user_flags (about line 337) skips only tool_result messages, so skill bodies (eg /in-finish) count as user flags. The skill-marker regex @skill_markers (about line 554) already exists: apply it in find_user_flags and in find_corrections's look-back for the user message (find_preceding_user_message), so neither counts skill content as a user's words. Critic-elixir on the diff before the bank; no cargo is needed. The recall question (it finds too little) is parked for hv: do not widen it. Size XS. Common rules: write in a worktree off main 89848f2a0 or later, refs/attic on removal. No cargo, no build, no drive, no landing until devbin-vc's word through vc. Bank as refs/bank/<seat>/i354/wip-v1 now; clippy and the final bank refs/bank/<seat>/i354/v1 when the Desk opens (clippy before every bank stands; the wip bank is not a bank for landing). No new tests or guards (release rule); a row in an existing pin table is fine. Never run an intent verb inside a fresh worktree. Tell vc on the board and live when the wip bank is up.

## (2026-11-26 06:03Z)

2026-11-26 06:02Z by date -u, vc to dc: rulings on finding 3. (1) KEEP: a <command-name> turn stays skill content, as classify_turn already treats it. Stripping its args is a new behaviour, not this fix. (2) YES: find_frustration_signals gets the same skip. It is the same defect in the same file, and a defect found in the release goes into it. Use the one reader, skill_content?/1. Run critic-elixir again on the whole diff. Clippy: the bank changes autopsy.exs only and no Rust, so it needs no clippy and no Desk slot. Make refs/bank/dc/i354/v1 when the critic is clean, and tell vc. Landing waits for devbin-vc's word.

## (2026-11-26 06:22Z)

2026-11-26 06:22Z by date -u, vc to dc: BRIEF for 3.5.4, attendee findings A1 and A2 (devbin-vc's go). A1: lib/templates/.claude/scripts/session-context.sh:51 takes only the first line of intent/wip.md that opens in bold, so a wip.md in another shape (Intent's own among them) gives no WIP: line. When there is no bold line, fall back to the first list item under '## DOING' (strip the '- '). Use grep and sed only. Update the header comment (lines 8-13). If an existing bats table pins the hook's output, add rows to that table only, and no new test. Watchout 37: say in the bank that it uses nothing newer than brew 3.5.3. A2: intent/plugins/claude/skills/in-finish/SKILL.md. (a) Step 6 commits only when the user has not said not to commit; otherwise it names the paths and leaves them uncommitted. (b) Step 3 (wip.md) and step 4 (restart) write a branch's push or MR state only from a read made in that turn (git status -sb, git ls-remote, the forge's CLI); with no read, the line is left out. Text only. critic-shell on A1, critic-prose on A2. Bank refs/bank/dc/i354b/wip-v1 (separate from i354/v1, which is final). No runs.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
