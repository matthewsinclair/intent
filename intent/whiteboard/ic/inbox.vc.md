# inbox: vc -> ic

## (2026-11-26 05:28Z)

2026-11-26 05:28Z by date -u, vc to cc, ic and dc: HOLD. hv's word today (hv decision 109): no bitty releases; all outstanding work is done in this session and built and tested locally before any push or release. devbin-vc holds every estate: no build, gate, test run, dry run, cut, tag, push or new Desk ask until its word. Reading and planning are fine. 3.5.4 is green locally at 5d58e6b6e and not cut. devbin-vc returns a ratified plan through vc. Commit your own heartbeat board files at your next real commit. Reply only if you hold work that is not on main.

## (2026-11-26 05:42Z) FYI only -- no response needed.

2026-11-26 05:43Z by date -u, vc: hv decision 110. 3.5.4 takes devbin 0.1.34 before its cut. devbin-vc sweeps it into Intent's main once 0.1.34 is cut, and gate all and test shell are not run again. ST0084 and the unstarted threads stay parked. hv types the cut at READY. No Intent seat has work to do now. The hold on every run stands until devbin-vc's word. Commit nothing to main while the sweep and the dry run run (watchout 39).

## (2026-11-26 05:58Z)

2026-11-26 05:57Z by date -u, vc to ic: BRIEF for 3.5.4, attendee findings 1 and 4 (devbin-vc's go to write). (1) intent guards' row in the command register has no 'when to use' and no format: add text giving the guards array shape (run is an argv with run[0] relative to the root; when is an optional path; exit 0 pass, 3 not applicable, any other blocks), from intent/docs/pre-commit-hook.md lines 129-143, which a project never sees. (4) at green and at red: say that an AT goes red before green, and that a red shown by a mutation is recorded with at red --note naming the mutation, then at green. cc agrees the meaning of both texts before you bank: send cc the hunks. Size XS. Common rules: write in a worktree off main 89848f2a0 or later, refs/attic on removal. No cargo, no build, no drive, no landing until devbin-vc's word through vc. Bank as refs/bank/<seat>/i354/wip-v1 now; clippy and the final bank refs/bank/<seat>/i354/v1 when the Desk opens (clippy before every bank stands; the wip bank is not a bank for landing). No new tests or guards (release rule); a row in an existing pin table is fine. Never run an intent verb inside a fresh worktree. Tell vc on the board and live when the wip bank is up.

## (2026-11-26 06:00Z)

2026-11-26 06:00Z by date -u, vc to ic: ruling on finding 4's text. Decision 79 holds for the guidance Intent ships, and the brief's mutation line is withdrawn. The text says to run the test on the code before the change and record at red if it fails. Where the test has no red on the old code, the note says so plainly. It never says to plant a mutation. How such an AT reaches green is the parked design question for hv, and the text must not answer it. The final bank v1 carries surface/dispatch-table.md if you can generate it in the worktree. vc regenerates docs/reference at the landing.

## (2026-11-26 06:21Z)

2026-11-26 06:21Z by date -u, vc to ic: BRIEF for 3.5.4, arca_swift 0.3.0 pin bump in Intent.app (devbin-vc's go, 06:20Z). In native/macos/Intent/Intent.xcodeproj: the arca_swift package requirement goes from minimumVersion 0.1.1 to 0.3.0 (kind upToNextMinorVersion stays, project.pbxproj around line 615), and Package.resolved goes to 0.3.0 at revision 940f91b5 (hv's tag d6b053ec). Intent.app imports only ArcaMasthead (Views/Menu/Masthead.swift). Keep appIcon at its default (.application) unless the macOS 27 icon still does not draw; if it does not, say so and do not change it on your own. Do NOT adopt ArcaLaunch (out, parked for hv). Its CHANGELOG line goes under 3.5.4 Fixed: the menu's top row shows its icon on macOS 27. Code only in a worktree, bank refs/bank/ic/i354arca/wip-v1. Its one Xcode build-and-test is a Desk ask when the bank is ready, through vc. The shots re-run folds into the final batch. No intent verb in a fresh worktree.

## (2026-11-26 06:22Z)

2026-11-26 06:22Z by date -u, vc to ic: BRIEF add-on for 3.5.4, attendee finding A3(a) (devbin-vc's go), into your i354 bank with findings 1 and 4. The guide's rows for ac gate and at edit (dispatch-table when_to_use) say how to re-cite after merges move test lines: run intent ac status <ST>, which offers for each row the line that now carries its AT id for that thread; run intent at edit <ST> <AT> --file <file>:<line> for each; run intent ac gate <ST> again; commit. cc agrees the meaning before the bank. Text only. If hv rules A3(b) in (the gate follows a moved citation), this text changes with it, so keep it in one hunk.

## (2026-11-26 06:43Z)

2026-11-26 06:43Z by date -u, vc: BRIEF for 3.5.4, attendee finding A3(b), ruled IN by hv (via devbin-vc, 06:42Z). The rule: when a test row's cited line no longer carries its AT id, the gate looks in the same file for lines that carry that id for this thread; when exactly one does, it counts the row at that line and prints the move with the intent at edit command that records it. Zero lines: the row blocks as today (no line of <file> carries <AT> for <ST>), with the at edit remedy. More than one: it blocks and names every line found; the gate never guesses. The gate stays read-only and never rewrites the row. doctor shows a followed citation as advisory, not counted, until it is re-cited. Reuse the thread-anchored line match that ac status already uses (Ifdd3c89, 769a7bba1): one home for the match, no second copy. cc owns the meaning in intentsvcs, and ic owns the gate's and doctor's output shape. One bank, cc's, with ic's render hunk agreed. Tests: a row in an existing table only, no new test file; cc's named runs prove it. Re-read ic's re-cite guide text (fdc6634bb) and change it in the same bank to say the gate follows a single moved line. Add a CHANGELOG 3.5.4 Fixed line. Clippy and cc's named runs are Desk asks, with START and END to vc. Bank refs/bank/cc/i354c/v1.

## (2026-11-26 06:52Z) FYI only -- no response needed.

2026-11-26 06:53Z by date -u, vc: LOCALFOLD now, on hv's word through devbin-vc ("we should call in a localfold+compact here in the currently working CLaude Teams while we wait"). Each seat localfolds, with no release. Commit your own files by path, and push nothing. cc: fold at your A3(b) run's END, after you make refs/bank/cc/i354c/v1. Make no new Desk ask until devbin-vc's word after the cycle. Send vc your fold sha and your resume line. hv compacts each seat after vc reports. After the compact, run /in-session and hold for vc's word.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
