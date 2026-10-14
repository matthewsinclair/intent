---
node: cc
name: Control Claude
role: control
session_id: 170c139d-4ce7-4ed3-a9ee-3da866f70f09
heartbeat_at: 2026-10-14 12:51Z
status: active
focus: "0595 BANKED (refs/bank/cc/0595/rust-host-speaks, doing 80), with vc for judgement and landing on vc's word; then WP-06 on its recorded shape; no push, no release"
claims: [ST0081, ISSUE:0595]
---

# Control Claude (cc)

## DOING

- RESUME (cc, localfold for hv's estate-wide compact cycle, 2026-10-14). ST0081 IS NOW THE HOMEBREW THREAD (vc's records at 7d1d2b8d4, on hv's word of 2026-10-14): Intent from Homebrew on Linux and macOS with no source checkout, and cc still claims it. ST0082 (Triage) is the WSL thread and depends on it; WP-04 is cancelled into ST0082; WP-06 (the formula installs on Linux, AC-06.1) and WP-07 (Intent on Ubuntu from its package, AC-07.1) are new; WP-03 is the Linux install through Homebrew at docs/install.md#linux, and its objective carries the page layout, vc's asks and the two CHANGELOG lines; AC-01.1 names aarch64 beside x86_64; hv's decision 17 stands for this Mac. THE DEMO IS OVER on vc's relay, so hold 29 and todo 40 are archived. That relay and vc's order went out between vc's clock reads of 09:59Z and 10:06Z; the 10:00Z stamp they carried was not read from a clock (vc's correction) and is unverifiable. kovacs-sysadm, a Claude session on the VM, runs the VM side on orders from vc and devbin-vc, which SUPERSEDES vc's ruling (1) of 2026-10-13: cc does NOT run the kovacs proof. vc's ORDER, read-only: (a) DONE AND BANKED. AT-05.1's test half is `git diff --binary 8c51de484 1c21a04ff --` the test file and suite.rs, on refs/bank/cc/0081/at051-test-half, blob 20712d9d2c1fc888b96e1bd8c797162dc5054811, base 8c51de484, patch-id 3f31071f3bb6, 2 files and 84 insertions, sha256 300357635a3e (vc verified it byte for byte). It applies clean to a `git archive` export of 8c51de484 and compiles there, because the test drives only the binary. The reds to expect at the base, read from source and unrun, are in vc's ORDER 5 word for word: the start arm reds at its LAST assertion (a LaunchAgents tree was written) only once `cargo build -p intentd` has put intentd beside intent in the same target dir, because the base resolves intentd before it writes the plist; the stop arm reds at its FIRST (it exits 0 with "not enrolled at login"); at HEAD both refuse before anything else. (b) DONE AND ACCEPTED under hv's pen (vc, all seven points and the recommendation): WP-06's shape, a proposal and not code; its text is wp06-shape.md in cc's session scratchpad, and vc's fold writes it into WP-06's objective because that file dies with the session. The headlines: keep the top-level url and override it in on_linux per arch; the macOS resources and the arm64 dependency move inside on_macos; the Linux branch installs the unpacked package into libexec; every hash comes from the release's own bytes; a dev-tap input lets kovacs prove it before any push; publish's asset-count refusal (cmd/macos:1709) refuses a release that already carries Linux packages, and its release notes say macOS only. (c) STAND BY: defects kovacs finds come to cc as issues, fixed in banks vc judges, and no heavy run without vc's GO. THE FIRST IS 0595 (filed at 65d5db612, ST0081 WP-01, cc's): `bin/int linux doctor` on a host with no Rust exits 127 with no output, because cmd/linux:42 runs `rustc -vV 2>/dev/null | sed` under `set -euo pipefail` and helpers' inherit_errexit, so the pipe's 127 ends host_triple before its die, and doctor calls host_triple before lacks(). Fix the silent 127, and make doctor list every lack rather than dying on host_triple first; how doctor gets an architecture without rustc is cc's call, and host_triple's "never from uname" rule is for build and package. Prove it with ONE bats arm (a shim PATH with bash 5 and no rustc), red on the Mac today and green with the fix; bank it and vc judges it. cmd/macos:226 carries the same pipe, and vc's fold adds it to 0595's body, so the fix covers both. The green on kovacs waits for hv. AFTER THE RESTART, IN ORDER (vc): 0595 first, then WP-06 on its recorded shape (the formula writer and smoke move out of cmd/macos to one home both release steps call; every sha256 read from the bytes the release serves; a dev-tap input of base URL plus hashes so kovacs proves the formula from file:// before any push; publish asks for its own assets by name and its notes drop macOS-only, in the same unit; size S). The publish finding goes to hv with D55. NO PUSH, NO RELEASE. (edited)
- 0595 LANDED at c775f61ef on vc's judgement (landed patch-id 9dd962b6599e, the bank's; vc verified): shared/rusthost.lib's rust_host is the one reader of rustc -vV's host for cmd/linux and cmd/macos and refuses by name when rustc is absent, fails, or names no host; linux doctor lists rustc among its lacks, taking its triple from uname only when rustc is absent; tests/unit/linux_doctor.bats proves it on a built PATH. 0595 STAYS OPEN for the follow-up vc ordered from the census: staged_version (cmd/macos:1390), whose missing or non-running staged binary ended a release step in silence, is BANKED at refs/bank/cc/0595/staged-version-speaks, blob bd4de018f8ed41f50ed41ad4442d0aea461d62b5, base c775f61ef, patch-id 5f14289d2f16, two arms in macos_release_assets.bats red at the base (127 and 6, no output) and green at the fix, with vc for judgement. releasebuild.lib:166 device_of is unreachable and stays, on vc's word. The green on kovacs waits on hv. Then WP-06 on its recorded shape. NO PUSH, NO RELEASE. (edited)

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

- NEVER DELETE A SPENT BANK REF, AND THE REASON IS NOT TIDINESS -- IT IS THAT THE DELETION IS UNRECOVERABLE AND TAKES THE BLOB WITH IT (vc's ruling, 2026-09-22, refusing dc's purge proposal). Measured: `.git/logs/refs/` holds `heads`, `remotes` and `stash` ONLY, there is no reflog for `refs/bank/*`, and `core.logAllRefUpdates=true` does not cover them. So a deleted bank ref cannot be recovered the way a deleted branch can. Worse, those refs are the ONLY thing keeping their patch blobs reachable: delete the ref and the blob is handed to the next `git gc`. cc holds 189 refs across 66 topics and they stay exactly where they are; vc's earlier decline of a ledger reconciliation stands on the same ground, that nothing turns on the number. A spent bank ref costs nothing where it sits.
- UNTIL THE KOVACS RUN, THE PRE-COMMIT GATE'S STALE-AT ADVISORY NAMES ST0081's AT-05.1 ON EVERY COMMIT ("the file EXISTS while the row says to-write"). It is expected and advisory: vc ran stale_at_check.sh on 2026-10-12, which flags AT-05.1 and exits 0. The row stays to-write on vc's ruling, because its two arms are ignored on macOS and have run nowhere, and `red` would claim a failure nobody has seen. It clears when kovacs runs the arms red at 8c51de484 and green at the landed HEAD, and `intent at green ST0081 AT-05.1` records both.

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
