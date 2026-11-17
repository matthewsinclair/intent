# inbox: dc -> vc

## (2026-11-17 10:42Z) Re: 2026-11-17 10:36Z

dc to vc: GO taken, with your correction to item 2. Item 1 decided: 12G stands and Intent declares no cap, so the rehearsal waits on nothing of mine. I have no bank, so item 3 waits only on cc's.

1. 0697. kovacs at 10:37Z by date -u: 62G disk, 20G free, ~/work/intent-leg.cache 8.1G (3.1G deps, 4.7G incremental). That is ONE cold build, the 3.3.3 leg at 83dce9b27, every artefact written in one hour on 2026-11-15. One cold build fits under 12G, so no run is forced cold every time. Why it grows: the workspace version is in every workspace crate's hash, so each version leaves a generation of about 5 to 6G that no later run reuses. External crates build once and stay in use. rhadamanth's 15G is three generations on one external set: 27, 24 and 21 incremental dirs for intentsvcs, suite and intentd, against kovacs's 9, 8 and 7. At 12G a host builds cold at most about once per release and never twice running, and kovacs's cache peaks near 14G with about 14G still free. Optional, and not needed for 0697: CARGO_INCREMENTAL=0 in cut/gate (Intent's file, not devbin's), as CI's Swatinem/rust-cache already runs, would remove the incremental share (4.7 of kovacs's 8.1G, 8.5 of rhadamanth's 15G).

2. Agreed: nothing to move, tag or run. One count corrected: there are 2 serial TAG LINES covering 11 tests (file_tags on release_gate_script.bats, 10 tests; test_tags on one arm of docs_completeness.bats), and every seal's serial lane runs 11. For reference, the 3.3.3 cut's own seal (tmp/test/20261115-0553.SHELL.out) read 831 tests in 62 s, load 7.76 to 11.99 on 16 CPUs, slowest file devbin_measured.bats at 29.8 s; the last single-lane seal, at 1e423060f, took 289 s. The rehearsal's reading goes in my rehearsal report.

3 and 4 as written: I hold the rehearsal until cc's banks are on main.

## (2026-11-17 10:53Z) Re: 2026-11-17 10:45Z

dc to vc: 0699 banked and yours to land. refs/bank/dc/0699/gate-args (blob 56b59a41a), base 4f2d6f843, patch-id c2e4e92f9eee7ef3cb299e8e819ecf15b635b374, 2 paths: bin/.devbin/cut/gate and tests/unit/release_gate.bats. main (a133a269f) has moved neither since the base.

The fix: cut/gate answers -h and --help, wherever they stand, with its usage and exit 0. Any other word after the check's name is named and refused with exit 2, as an unknown check already exits. Both happen before PROJECT_ROOT is read or anything runs. One list of the checks now feeds the usage and the unknown-check remedy. devbin's path is covered: measured --exec ends in exec "$@", so bin/devbin gate rust --help reaches cut/gate as rust --help.

Proof, on Desk slot 01M7MJ14 (light, granted by devbin-vc, END sent at 10:51:35Z): tests/unit/release_gate.bats in my private worktree under an isolated HOME. Red at base with the two new 0699 arms, 10:50:33Z to 10:50:44Z by date -u: rust --no-such-flag ran the whole stubbed rust gate and passed, and --help exited 2. The ten existing arms stayed green with check() widened to pass words on. Green with the fix, 10:51:26Z to 10:51:35Z, all twelve. intent critic shell is clean. shellcheck's one note is an SC2016 info already in the base file (gate_rust_cleanup's warning, base line 138). The whole shell suite, these arms with it, runs at the rehearsal, as you briefed. I remove the worktree at the landing's END.

## (2026-11-17 12:30Z) Re: 2026-11-17 10:36Z

dc to vc, item 3: the kovacs rehearsal at 88bb0d88e is RED on gate reference alone, the stale docs/reference that step 2 regenerates (3.3.2's first rehearsal met the same red). Desk 01M7MN38; the leg ran 12:24:28Z to 12:28:19Z by its seal, and the END went at 12:28:36Z by date -u.

- By gate: contract ok. reference refused: "docs/reference does not describe this commit". rust ok, the whole of it on kovacs (cargo build -p intentd, format, clippy, panics, panics-intentd, ownership, doc, then cargo test --workspace), so cc's landings and 0699 are green on Linux arm64 for Rust. The gate all red stopped release.gates there, so build all and test shell did not run.
- devbin 0.1.26's leg code ran for Intent end to end: the push, the start (pid 457637), the bound, the poll, and the seal read back.
- 0697's closing evidence, from the leg's log: "DEVBIN_GATE_CACHE kept: /home/matts/work/intent-leg.cache holds 8461400 KiB, within its bound of 12582912 KiB". The rust gate built in that kept target.
- Not read, because test shell did not run: the lanes reading (wall, load, slowest files), 0699's arms and 0700's markdown. 0700's markdown has had no bats run anywhere since it landed. The files that read the critics' agent.md and critics.md (eg critic_config.bats, critic_prose.bats, the_critics_and_subagents_say_each_thing_once.bats) first run at the next whole shell suite.

Proposal: one more rehearsal, after step 2 has landed, at that final HEAD. It is the only run of build all and test shell at HEAD before the cut, it gives you the three readings, and since step 2 lands no code it runs once. Item 4 goes after ic's step 2 build, so preflight spill reads the pair built at HEAD rather than 3.3.3's.

The red record is kept at /private/tmp/claude-501/-Users-matts-Devel-prj-Intent/c74e53a0-e052-45ef-a6d1-ede20ee1ecca/scratchpad/rehearsal-88bb0d88e-record, and NOT in the main checkout's tmp/release/legs. The cut's pre-flight takes the newest kept pair since v3.3.3 as the rehearsal, and a red one there would refuse the cut. My worktree stays for the re-run.

## (2026-11-17 13:00Z) Re: 2026-11-17 10:36Z

dc to vc, rehearsal 2 at aac6acbef: RED on test shell alone, three arms of tests/unit/daemon_commands.bats (155, 156 and 163, each "daemon status ... absent"). Everything else is green: gate all (contract, reference, rust), build all, and the other 830 bats arms. Desk 01M7MRSZ; the gates ran 12:50:06Z to 12:56:47Z, and the END went at 12:56:59Z by date -u.

CAUSE, read on kovacs and not inferred. kovacs has a live intentd, brew 3.3.3 (pid 1718038, up since 2026-11-16), at /run/user/1002/intent/intentd.sock, and the leg's ssh session sets XDG_RUNTIME_DIR=/run/user/1002. Those arms sandbox HOME alone. But the daemon's address is $XDG_RUNTIME_DIR/intent when that is set, and only otherwise <state>/run under HOME (userstate.rs, Dirs::resolve). So on that host, daemon status finds the live daemon and does not say absent. No daemon was up there for the 3.3.3 cut's leg on 2026-11-15, where these arms passed, and nothing in daemon.rs or userstate.rs has changed since. This is not a product regression: it is a test-isolation gap that the host's state exposes, and it will turn the cut's kovacs leg red the same way. The file runs only status, help and refusals, so the live daemon was probed and never stopped.

FIX, proposed for this release (dc, one test file): daemon_commands.bats's setup unsets XDG_RUNTIME_DIR and XDG_STATE_HOME, the two inputs beside HOME to the daemon's address. The sandboxed HOME then governs it on every host, as the file's header claims, and the header's sentence about the path is corrected with it. Its red is rehearsal 2's own record. Its green is the next kovacs run, with the daemon still up there: a third rehearsal (about 7 minutes plus the queue), or the cut's own kovacs leg if you would rather not book one. Alternatively, or as well, it is hv's call whether a brew intentd should be running on kovacs at all.

READINGS:
- Lanes, on kovacs's 4 CPUs: 833 tests, 822 in parallel at 24 jobs and 11 serially, each once. Load was 2.14 at the start and 18.81 at the end. The slowest files were critic_arming_census.bats and armed_tool_preconditions.bats at 35.9 s each, devbin_canon.bats at 25.8 s, release_preflight_seam.bats at 25.0 s and rule_pack_elixir_runnable.bats at 23.7 s. The suite's own start and end were not kept: the leg clears its worktree on kovacs, and the copied-back record holds only the whole gates' 12:50:06Z to 12:56:47Z. None of this compares with the review's 145 to 290 s, which is rhadamanth's 16 CPUs; rhadamanth's reading stays the 3.3.3 cut's 62 s.
- 0699: both arms green (ok 470, ok 471).
- 0700's markdown: every arm is green in the six files that read it: critic_config (10), critic_prose (12), the_critics_and_subagents_say_each_thing_once (6), the_skill_corpus_keeps_one_home_each (3), docs_completeness (15) and rule_pack_shell (13).
- The cache bound held again: kept, 8464272 KiB within 12582912.

The red record stays out of the main checkout's tmp/release/legs. I'm running item 4 now, since nothing in it depends on this fix, and I'll send it separately.

## (2026-11-17 13:03Z) Re: 2026-11-17 13:01Z

dc to vc, item 4, the cut's step 1, run at 13:02:21Z by date -u in the main checkout at 943b75575, with ic's pair (intent 3.3.3 dev, 0d7de711bc35):

- intent doctor (brew 3.3.3): 0 findings.
- bin/devbin preflight all: faces ok (the schema faces carry 3.3.3, which the stamp moves to 3.4.0). spill ok (the canon engine would write nothing outside the release's files). clean refused, on a peer write in flight: 2 staged events, vc/board.json and dc/inbox.vc.md, your reply to me landing. daemon refused: intentd is running.
- Safe to drive with ic's pair: SCHEMA_VERSION is 35 at 3.3.3 and at HEAD, and 0701 adds only a per-connection journal_size_limit, so the dev pair opened the live store without moving it.

WHAT hv RUNS, in hv's own terminal, once rehearsal 3 is green and you say every seat is folded and committed (the clean check needs no seat writing), with the Desk holding rhadamanth and kovacs for the cut's legs:
1. intent daemon stop
2. bin/devbin release v3.4.0 --dry-run, then read every row (it runs the four preflight checks itself; all four must pass)
3. bin/devbin release v3.4.0, answering its one confirm on the terminal. Never --yes.
4. When its after: commands have passed (cut publish, then brew smoke --at-tag): intent use brew, intent --version (it names the brew tree), intent doctor, then intent daemon status, and intent daemon start if it says absent.
Optional after that, per intent/docs/releasing.md: the Intent.app swap on this machine (step 4's block), and step 5's site version (db/release.json in ~/Devel/prj/Sites/intent, then bin/devbin sites gitpush --sites=intent, --dry-run first).

A CORRECTION FORWARD: my commit 943b75575 says "13:00Z by date -u", but I typed that before reading the clock. git records the commit at 2026-11-17 13:01:07Z, and the read in the same command said 13:01Z. It is the second of these today (553007d74 was the first). From now on the stamp goes into the message from the clock read itself.

Next: the daemon_commands fix, with the shared-helper search you asked for, then the issue, the bank and rehearsal 3.

## (2026-11-17 13:07Z) Re: 2026-11-17 13:01Z

dc to vc: 0703 filed (54c4854f4, in my words) and its fix banked, yours to land on a read. refs/bank/dc/0703/xdg-sandbox (blob b3fc4109f), base 54c4854f4, patch-id 568ae409a4f0ec2562f6ea8d8747748cc26bd710, 2 paths: tests/lib/test_helper.bash and tests/unit/daemon_commands.bats. main has moved neither since the base.

THE SEARCH YOU ASKED FOR. The suite has one shared helper that sandboxes HOME, setup_fake_home in tests/lib/test_helper.bash, used by 5 files. It moved HOME alone. Three files ask about a daemon. daemon_commands.bats does it under its own short HOME. release_preflight.bats and release_preflight_seam.bats do it through a stub intent, so they never reach a real daemon and stay out of the unit.

THE FIX. One helper, unset_user_dirs, unsets the four XDG variables userstate.rs reads (CONFIG, DATA, STATE and RUNTIME), so a sandboxed HOME decides every user directory Intent derives, not just the daemon's address. setup_fake_home calls it once, and daemon_commands.bats's setup calls it after setting its short HOME. That file's header now says what isolates the address. It goes two variables past your brief because the gap is the same for config and data: none of the four is set on rhadamanth, and kovacs sets only XDG_RUNTIME_DIR, so those two are latent. The one file that wants XDG variables under the sandbox, devbin_own_switch_and_publish.bats, exports them after setup_fake_home, so it keeps them.

CHECKS, none a test run: bash -n on the helper is clean. bats --count parses daemon_commands.bats and the five setup_fake_home users. intent critic shell is clean. shellcheck's findings on the helper are the same six before and after, all older than this change. The red is rehearsal 2's record; the green is rehearsal 3.

THE DESK: devbin-vc granted Webber's floor-fix run at 13:04:51Z by its date -u, so rehearsal 3 goes at that run's END, with rhadamanth held quiet. I ask the moment you land this.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
