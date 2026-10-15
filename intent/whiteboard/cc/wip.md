---
node: cc
name: Control Claude
role: control
session_id: 499621e2-fc3c-4e68-95a3-009e5e8b84c4
heartbeat_at: 2026-10-15 10:11Z
status: active
focus: "ST0083, the agent contracts lane, on vc's order of 2026-10-15 (hv's word relayed; hold 31 archived): WP-01 first and alone in a detached worktree, the @AGENTS.md import, banked for vc; then WP-02 and WP-03 stacked; ST0081's WP-03 waits on kovacs; no push, no release"
claims: [ST0081, ST0083]
---

# Control Claude (cc)

## DOING

- ST0081 WP-06 LANDED at 11017d7bd on vc's judgement of bank -c (refs/bank/cc/0081/wp06-brew-formula-c, landed patch-id 35a43378080c, the bank's), and it STAYS WIP: AC-06.1 is satisfied only by kovacs's brew rehearsal, which also measures whether a machine that never trusted matthewsinclair/intent installs after a plain brew tap or refuses until brew trust (Homebrew 7 loads only trusted taps; hv has the question through vc, and AC-06.1, docs/install.md and the macOS clean install wait on it). bin/int brew formula and smoke replace int macos formula and smoke; shared/brew.lib renders the formula for both release steps from the bytes its base URL serves; publish refuses only its own assets, by name, and pushes only a complete formula, else bin/int brew formula --push at a terminal; --dev-tap and --tap let the VM prove it first. The banks before it: -b fixed vc's defect (brew_fetch's exit dropped a hashing refusal unread), and -c re-padded MODULES.md's table after the staged-format guard refused -b's landing (the widest cell shrank). NO PUSH, NO RELEASE. (edited)
- RESUME (cc, localfold for hv's shutdown of 2026-10-14, on hv's word relayed by zeop-sc and by vc). LANDED TODAY, each on vc's judgement: 0595 at c775f61ef with its staged_version follow-up at 7ee4fedd5 (doing 80; 0595 stays open until kovacs runs tests/unit/linux_doctor.bats on Linux, ORDER 3b), and ST0081 WP-06 at 11017d7bd (doing 81; WIP until AC-06.1). IN HAND, NOTHING LANDED: (1) THE KOVACS ORDER DRAFTS, with vc for review before kovacs-sysadm gets them, BANKED as blobs of their exact bytes: refs/bank/cc/0081/kovacs/o5-at051.sh (sha256 8fe1526b1561), o6-brew.sh (113a9d3b3a13) and o7-drive.sh (f89b59ae948d); each bash -n clean, shellcheck 0.11.0 at exit 0, on o3-build.sh's conventions without a census; o7 keeps Rust off the drive's PATH (a departure vc may overrule) and drives AC-07.1's hooks as settings.json declares them, leaving the in-session half to a real session. (2) WP-03, docs/install.md#linux, READ AND NOT WRITTEN, so nothing is banked: a bare `## Linux` section (Homebrew on Linux, brew install matthewsinclair/intent/intent, intent bootstrap, intent doctor in a new project), the Homebrew section's macOS-only paragraph, the --at-login sentence (WP-05's refusal off macOS, "nothing was written"), the libexec layout line under what an install contains, docs/concepts/working-in-a-team.md:505's "published for macOS on Apple silicon only", and two lines in CHANGELOG's 3.2.4 (Added: the Linux packages and the Homebrew install on Linux; Fixed: WP-05's --at-login refusal), composed with dc's refs/bank/dc/324/changelog in both orders and prettier --check before banking, with the trust step marked PENDING o6's measurement; it lands only after kovacs measures the trust question. STANDING: defects kovacs finds come to cc as issues, fixed in banks vc judges; no heavy run without vc's GO; kovacs's permission rule is NOT yet in (hv adds it; vc's correction) and ORDER 3 has no GO. FIRST STEP AT THE NEXT START: /in-session and the pickup, hold 31 until hv's word, then WP-03's bank. NO PUSH, NO RELEASE.
- ST0083, THE AGENT CONTRACTS LANE (vc's order of 2026-10-15 on hv's "Ok, proceed", relayed by vc). cc claims ST0083. LANDED, each on vc's judgement with the bank's patch-id: WP-01 at 16f351a90 (271b9eae66ef), the @AGENTS.md import, WP-01 DONE with AC-01.1 satisfied on both layouts (8a8509258); WP-02 at a85dfbdf0 (a2246ee4ecc9), the contracts key, Contracts::dir and sync::root_file_path, the scope reading the layout per build (an absent config reads as root, an unreadable or third-value one surfaces), canon and agents sync/validate placing the contracts, the daemon watching .claude/; AT-02.1 to AT-02.3 green, AC-02.1 and AC-02.3 satisfied. WP-03 at 2d9b4de1a (ebde4dc9dec0), init --contracts and agents layout; AT-03.1 and AT-03.2 green; AC-02.2, AC-03.1 and AC-03.2 satisfied; WP-02 and WP-03 DONE (93ac91b0a); 6 of 14 criteria. Intent's own CLAUDE.md carries the import at 82550ffda; issue 0598 (retire or restate CLAUDE.md's copy of the four-rule index) is filed for the next line. RECORDED: init renders llm/ and prj/ templates embedded at build time, so the key and the import reach init on this machine only after build all, which is owed before the push. NOW: WP-05 (the deny rules) shape to vc after a docs check on Claude Code's rule semantics, then WP-07 (Dropbox mark), WP-04 (doctor) and WP-06 (deinit), each shape first and one bank each; WP-08 LAST on the tree after dc's train lands, its CHANGELOG lines composed on dc's refs/bank/dc/324/changelog (c8cbaf6bf) in both orders. STANDING: every Rust bank owes the workspace suites after cargo build -p intentd, CI's clippy, doc and fmt lines and the shell half; one heavy run at a time, each on vc's GO after a census; questions go to vc. ST0081 (doing 82): 0595 closed (88db489b9); AT-05.1 green on kovacs and WP-05 done (a4e58d45a); o7-b green and noted on AC-07.1, WP-07 started (5c581f9d9), its session half owed to hv; o6 waits on Linuxbrew on kovacs; WP-03's docs bank waits on o6's trust measurement. NO PUSH, NO RELEASE. (edited)

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
