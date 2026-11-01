---
verblock: "2026-11-01:v3.52: vc - fold after hv's go: the rulings in their issues, their fixes landing"
intent_version: 3.2.8
---

# Work In Progress

## DOING

- Issues 0666, 0668 and 0669, on hv's rulings of 2026-11-01, each recorded in its issue. 0666 (cc) and 0668 (dc) are judged green and land in that order on vc's word naming HEAD, 0668 re-banked as one CHANGELOG bullet under 0666's `### Fixed`. 0669 (cc) is rebased onto both and judged by vc before it lands. Each builder closes its issue in its landing chain (vc todo 73). hv ruled that 3.2.9 carries all three, each once judged green and landed; one not landed by the cut rides the next release.

## TODO

- Issue 0670, on hv's ruling "ambiguous lines stop", the measured 1-row shape recorded in the issue: its build waits for hv's word, because hv's word for 2026-11-01 is "ship ST0011, no new code and no new yaks". dc builds it, and vc judges it with the fleet census re-run against the build, which must redden Devbin ST0011 AT-3.5 and nothing else.
- ST0090, issue 0664's fix: one criterion is left, AC-02.4. hv sees Intent.app's Rebuild Search Index row show its second line, `reconciled <age> ago`, after a rebuild, on a build at or after a54f2e9e0 whose CLI carries 8406f6a15: the 3.2.9 cut, or a dev app pointed at a dev CLI, which is the maintainer's alone. vc then records that build on AC-02.4 and dc closes WP-02 (vc hold 6, dc hold 37). `intent ac gate ST0090` names what is left.
- ST0091, Devbin ST0011's WP12 in Intent's tree: Intent cuts through devbin's release core. Its packages wait for Devbin's 0.1.19 cut and the sweep after it, in which devbin-vc runs the upgrade in Intent's tree and commits the sweep pathspec inside an Intent chain with vc and dc (vc todo 71). dc then builds the packages and vc judges each, every run booked through devbin-vc's desk. Issue 0667's fix rides with AC-03.3, and dc's issue draft for the thread is banked at refs/bank/dc/fold5-20261031/st0091-wp12-issue-body.md. hv fires the first cut through the core.
- Laksa's and Conflab's releases wait for hv's next word. On rhadamanth, Conflab runs its checkout's build (520987af, v0.6.2-468) through ~/.local/bin and ~/bin links, with brew's 0.6.2 installed but unlinked, so it moves to brew at its next release.
- Devbin 0295, for the next devbin release: stamp first, test once, tag what passed. The kovacs run stays (hv).
- Parked for hv, from 2026-10-30:
  - Intent's cut wants one driver: five commands follow `build release`, and `bin/int brew publish` refuses under `!` because it confirms at a terminal. ST0091 retires it.
  - `bin/devbin macos app-install` leaves an unsigned dev build in /Applications that nothing switches back. Intent.app's title and menu show the CLI's version; only About shows the bundle's.
  - Gtools.app has no brew route: brew's `gtools app install` refuses, and the release ships no app. On gyges it builds only from a GUI login session, since codesign cannot reach the keychain from gyges-sysadm's Background session. Brew warns that the gtools formula's `post_install` is deprecated.
  - Utilz's tests.yml Rust job has no `timeout-minutes`, and prez's Chrome render has no timeout.
  - Gtools CI got ubuntu-24.04 where Utilz and Intent got 26.04; pinning it is hv's call.
  - Both Macs' CoreSimulator (1174.9.2) is older than Xcode's (1174.21.0).
  - `utilz doctor` said "gh is not installed" with /opt/homebrew/bin/gh on PATH (seen once, not traced).
- intentd's enrolment at login on rhadamanth is with hv in devbin-vc's TUI (2026-11-01). The 0666 fix does not change it by itself.
- After the landings: build all at HEAD and a green Linux preflight covering it. Then devbin-vc pushes local and upstream, fast-forward only, on vc's word naming that HEAD (vc decision 90). `git rev-list --count upstream/main..main` says how far main is ahead.
- Parked for hv: issues 0649, 0650, 0661, 0665 and 0671 (an intentd started without CLAUDE_CONFIG_DIR sees no Claude session, recorded only); threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089.
- Parked for hv, from dc's mark at ST0087 (cb8ddf9d8):
  - Doctor and bare `intent use` do not compare the running binary with the link set's, and `intent use <tree>` does not say to `rehash`.
  - The home pointer names the dev tree while the machine rests on brew.
  - A def search held a core about 9 minutes on Laksa's index, and a bare Elixir module name answers empty.
  - In a Swift `@main` class, the outline and `--kind def` put every member at the `@main` line.
  - `bin/lib/*.exs` has no Elixir critic coverage.
  - `wb archive message --handled` with no `--to` also archives messages to hv, and wb has no unarchive.
- Deferred: the critic contract for installed subagents; devbin 0227's formula renderer; the intentd CPU item; ic's `/refresh`; devbin-vc's repo-setup question and `--ivault`.
- Carried for hv: Trellis's session hooks, the gyges clean-Mac re-proof, the Sinclair-Shared book recreate, the Conflab and Baize repairs, and a brew ruby on kovacs.
- Off main for hv: Conflab's refs/bank/ic/todo24-v5 (the gate refuses it on 31 IN-SW-CODE-006 warnings already on main); Gtools' st19/g8 (conflicts in five files); Laksa's refused and conflicting banks. hv owes a read-only production query for duplicate content_assets rows before vc todo 60's fix deploys. Each estate keeps its own parked list in its intent/wip.md.
