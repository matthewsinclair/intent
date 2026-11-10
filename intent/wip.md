---
verblock: "2026-11-10:v3.58: vc - one release, 3.3.0, carries everything outstanding"
intent_version: 3.2.8
---

# Work In Progress

## DOING

- 3.3.0, the one next release, carrying everything outstanding, on hv's word of 2026-11-10 (hv directive 1, vc decision 109): there is no 3.2.9, and nothing rides a release after. The train is vc todo 77. First the three banked fixes (0671, the testkit race and the sweep order; vc todo 74), then the push of main (vc hold 8). dc builds ST0091 and then issue 0670; cc builds ST0092's WP-02 to WP-09 and issue 0672; ic reads ST0092's surface (WP-10). Each has its lane in its inbox, and none has a session running. Every heavy run is booked with the estate's controller, agentic-engineering-vc since 2026-11-10.
- ST0092, Claude Team profiles (vc decision 107, vc todo 76): WIP on hv's start, eleven packages, none started. vc writes WP-01's terms document next, and hv ratifies it before any code. A spike runs beside it: one swarm set up by hand in the thread's form, whose findings go into the thread as they arrive. `intent wp list ST0092` names the packages and `intent ac gate ST0092` what is left.

## TODO

- The push of main to local and upstream (vc hold 8): after 3.3.0's first judged train is on main, with a build all at that HEAD on the controller's desk and a Linux preflight at that HEAD on kovacs (hv's `!` in kovacs-sysadm's session), devbin-vc pushes on vc's word naming that HEAD (vc decision 90). It carries the fixes for 0666, 0668 and 0669, which are on main. No release rides on it.
- Issue 0672 (high), in 3.3.0: `intent agents sync` writes language-default build and test lines into AGENTS.md that are wrong in most of the fleet. cc builds it in the full shape (M), on vc's ruling under the pen (vc todo 75): the lines are rendered from what the project declares. A fix reaches a project at the 3.3.0 cut and that project's next `intent agents sync`.
- Issues 0649, 0650, 0661 and 0665, in 3.3.0 (vc decision 109): vc reads each and gives it a lane, and one that proves larger than it reads comes back to hv with its reason.
- Issue 0670, in 3.3.0, on hv's ruling "ambiguous lines stop", the measured 1-row shape recorded in the issue: dc builds it once ST0091's packages are on main, vc first answers dc's two shape questions, and vc judges it with the fleet census re-run against the build, which must redden Devbin ST0011 AT-3.5 and nothing else.
- ST0090, issue 0664's fix: one criterion is left, AC-02.4. hv sees Intent.app's Rebuild Search Index row show its second line, `reconciled <age> ago`, after a rebuild, on a build at or after a54f2e9e0 whose CLI carries 8406f6a15: the 3.3.0 cut, or a dev app pointed at a dev CLI, which is the maintainer's alone. vc then records that build on AC-02.4 and dc closes WP-02 (vc hold 6, dc hold 37). `intent ac gate ST0090` names what is left.
- ST0091, Devbin ST0011's WP12 in Intent's tree: Intent cuts through devbin's release core. Devbin 0.1.19 and then 0.1.20 have been swept into Intent's tree (066bd45bb, 18a75d29c), so its packages wait for dc to build them; vc judges each, every run booked with the estate's controller (vc todo 71). Before the first cut through the core, hv says which remotes it pushes to and `release.remotes` is declared (vc watchout 33). Issue 0667's fix rides with AC-03.3, and dc's issue draft for the thread is banked at refs/bank/dc/fold5-20261031/st0091-wp12-issue-body.md. hv fires the first cut through the core, which is 3.3.0.
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
- Parked for hv: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089, which are not outstanding work and are not in 3.3.0 (vc decision 109).
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
