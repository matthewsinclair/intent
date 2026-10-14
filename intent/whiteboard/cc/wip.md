---
node: cc
name: Control Claude
role: control
session_id: 170c139d-4ce7-4ed3-a9ee-3da866f70f09
heartbeat_at: 2026-10-14 14:02Z
status: active
focus: "LOCALFOLDED 2026-10-14 for hv's shutdown: holding (hold 31) until hv's word; then doing 82, WP-03's bank first, with o5, o6 and o7 banked for vc's review; 0595 and ST0081 WP-06 landed, open on kovacs's runs; no push, no release"
claims: [ST0081, ISSUE:0595]
---

# Control Claude (cc)

## DOING

- 0595 LANDED at c775f61ef and its staged_version follow-up LANDED at 7ee4fedd5, each on vc's judgement, each landed patch-id the bank's (9dd962b6599e; 5f14289d2f16): shared/rusthost.lib's rust_host is the one reader of rustc -vV's host for cmd/linux and cmd/macos and refuses by name when rustc is absent, fails, or names no host; linux doctor lists rustc among its lacks and takes its triple from uname only when rustc is absent; staged_version refuses by name when the staged binary is missing or does not run, carrying its stderr. releasebuild.lib:166 device_of is unreachable and stays, on vc's word. 0595 STAYS OPEN until kovacs runs tests/unit/linux_doctor.bats on Linux, a light order vc sends once hv's permission rule is in. NO PUSH, NO RELEASE. (edited)
- ST0081 WP-06 LANDED at 11017d7bd on vc's judgement of bank -c (refs/bank/cc/0081/wp06-brew-formula-c, landed patch-id 35a43378080c, the bank's), and it STAYS WIP: AC-06.1 is satisfied only by kovacs's brew rehearsal, which also measures whether a machine that never trusted matthewsinclair/intent installs after a plain brew tap or refuses until brew trust (Homebrew 7 loads only trusted taps; hv has the question through vc, and AC-06.1, docs/install.md and the macOS clean install wait on it). bin/int brew formula and smoke replace int macos formula and smoke; shared/brew.lib renders the formula for both release steps from the bytes its base URL serves; publish refuses only its own assets, by name, and pushes only a complete formula, else bin/int brew formula --push at a terminal; --dev-tap and --tap let the VM prove it first. The banks before it: -b fixed vc's defect (brew_fetch's exit dropped a hashing refusal unread), and -c re-padded MODULES.md's table after the staged-format guard refused -b's landing (the widest cell shrank). NO PUSH, NO RELEASE. (edited)
- RESUME (cc, localfold for hv's shutdown of 2026-10-14, on hv's word relayed by zeop-sc and by vc). LANDED TODAY, each on vc's judgement: 0595 at c775f61ef with its staged_version follow-up at 7ee4fedd5 (doing 80; 0595 stays open until kovacs runs tests/unit/linux_doctor.bats on Linux, ORDER 3b), and ST0081 WP-06 at 11017d7bd (doing 81; WIP until AC-06.1). IN HAND, NOTHING LANDED: (1) THE KOVACS ORDER DRAFTS, with vc for review before kovacs-sysadm gets them, BANKED as blobs of their exact bytes: refs/bank/cc/0081/kovacs/o5-at051.sh (sha256 8fe1526b1561), o6-brew.sh (113a9d3b3a13) and o7-drive.sh (f89b59ae948d); each bash -n clean, shellcheck 0.11.0 at exit 0, on o3-build.sh's conventions without a census; o7 keeps Rust off the drive's PATH (a departure vc may overrule) and drives AC-07.1's hooks as settings.json declares them, leaving the in-session half to a real session. (2) WP-03, docs/install.md#linux, READ AND NOT WRITTEN, so nothing is banked: a bare `## Linux` section (Homebrew on Linux, brew install matthewsinclair/intent/intent, intent bootstrap, intent doctor in a new project), the Homebrew section's macOS-only paragraph, the --at-login sentence (WP-05's refusal off macOS, "nothing was written"), the libexec layout line under what an install contains, docs/concepts/working-in-a-team.md:505's "published for macOS on Apple silicon only", and two lines in CHANGELOG's 3.2.4 (Added: the Linux packages and the Homebrew install on Linux; Fixed: WP-05's --at-login refusal), composed with dc's refs/bank/dc/324/changelog in both orders and prettier --check before banking, with the trust step marked PENDING o6's measurement; it lands only after kovacs measures the trust question. STANDING: defects kovacs finds come to cc as issues, fixed in banks vc judges; no heavy run without vc's GO; kovacs's permission rule is NOT yet in (hv adds it; vc's correction) and ORDER 3 has no GO. FIRST STEP AT THE NEXT START: /in-session and the pickup, hold 31 until hv's word, then WP-03's bank. NO PUSH, NO RELEASE.

## TODO

_(none)_

## Holds

- **ALL OF cc's WORK IS HELD ON THE FOLD UNTIL hv's WORD** (hv's shutdown of 2026-10-14, relayed by zeop-sc and by vc): cc localfolded and committed its own records by path, and pushed nothing, because the push in Intent is hv's. When hv's word reaches cc, in cc's session or relayed by vc, archive this hold and resume from doing 82, WP-03's bank first.

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.
- UNTIL THE KOVACS RUN, THE PRE-COMMIT GATE'S STALE-AT ADVISORY NAMES ST0081's AT-05.1 ON EVERY COMMIT ("the file EXISTS while the row says to-write"). It is expected and advisory: vc ran stale_at_check.sh on 2026-10-12, which flags AT-05.1 and exits 0. The row stays to-write on vc's ruling, because its two arms are ignored on macOS and have run nowhere, and `red` would claim a failure nobody has seen. It clears when kovacs runs the arms red at 8c51de484 and green at the landed HEAD, and `intent at green ST0081 AT-05.1` records both.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
