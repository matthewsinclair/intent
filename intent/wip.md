---
verblock: "2026-11-10:v3.65: vc - issue 0661 is on main, and ST0091's WP-01 is L in two banks"
intent_version: 3.2.8
---

# Work In Progress

## DOING

- 3.3.0, the one next release, carrying everything outstanding, on hv's word of 2026-11-10 (hv directive 1, vc decision 109): there is no 3.2.9, and nothing rides a release after. cc, dc and ic are running, each directed by vc on hv's word of the same day (vc decision 111). dc builds ST0091, then issues 0670, 0665 and 0649. Issue 0661 is on main, and ic builds 0650, 0672, 0676 and 0677 and reads ST0092's surface (WP-10). cc builds ST0092's WP-02 to WP-09 with issue 0675 once the terms are ratified. Each seat's lane is on vc's board (vc todo 77). Every heavy run is booked with the estate's controller, agentic-engineering-vc since 2026-11-10. The cut also waits on Devbin's next release, which hv fires and which is swept into Intent's tree first (vc decision 113).
- ST0092, Claude Team profiles (vc decisions 107 and 111, vc todo 76): WIP, eleven packages. Intent's own board follows the terms by hand since 2026-11-10, on hv's word: six seats on the roster, the team brief on hv's board, and the roster README and `.claude/restart.md` in the thread's form. WP-01 is WIP: its terms document, `intent/plugins/claude/teams/TERMS.md`, waits on hv's ratification on the twelve points WP-01's body lists, and no code starts before that (AC-01.5). Until the cut the packages are driven on a clone of Intent's board, because a newer build's first touch of the real store stops every 3.2.8 reader. `intent wp list ST0092` names the packages and `intent ac gate ST0092` what is left.
- ST0091, Devbin ST0011's WP12 in Intent's tree: Intent cuts through devbin's release core. WIP since 2026-11-10: dc builds its four packages to vc's rulings on its re-inventory and its design (vc decisions 112 and 114, vc todo 71): WP-04's trigger change is banked as the train's first unit, WP-01 is L and follows as two banks, and vc judges the train in one run. The cut waits on Devbin's next release, which carries the pre-flight seam that the refusal of a live intentd needs (devbin 0327) and the release commit's trailer (devbin 0328), both ruled by hv on 2026-11-10 in devbin-vc's window as devbin-vc relayed them (vc decision 113); the sweep that brings it in comes before dc wires either. Intent's brew smoke stays Intent's and runs as an `after:` command. `release.remotes` is built as local and upstream until hv says which remotes a cut pushes to (vc watchout 33). Issue 0667's fix rides with AC-03.3. hv fires the first cut through the core, which is 3.3.0, and its tag is annotated, because the core's formula step refuses a lightweight one.

## TODO

- Owed by hv to vc (vc todo 77), in the order vc would take them: an answer on client names in Intent's committed record, which the push waits on and which gets dearer with each commit (vc watchout 34); the ratification of ST0092's terms document on the twelve points WP-01's body lists; which remotes a cut pushes to (vc watchout 33); whether the contract check becomes a release gate, which vc recommends (vc decision 114); whether Intent keeps an AT's own red-then-green on the code as it was; and the shape of issue 0665.
- The push of main to local and upstream (vc hold 8) waits on three things. hv answers on client names. A build all at the HEAD to be pushed passes on the controller's desk, with every seat holding its commits until the push. And a Linux preflight at that HEAD passes on kovacs (hv's `!` in kovacs-sysadm's session). Then devbin-vc pushes on vc's word naming that HEAD (vc decision 90). It carries the fixes for 0661, 0666, 0668, 0669, 0671, 0673 and 0674, which are on main. No release rides on it.
- Issue 0672 (high), in 3.3.0: `intent agents sync` writes language-default build and test lines into AGENTS.md that are wrong in most of the fleet. ic builds it in the full shape (M), on vc's ruling under the pen (vc todo 75): the lines are rendered from what the project declares. A fix reaches a project at the 3.3.0 cut and that project's next `intent agents sync`.
- Issues 0649, 0650 and 0665, in 3.3.0 (vc decision 109): 0650 is ic's next, 0649 is dc's (vc todo 77), and 0665 is dc's once hv has ruled its shape, which the issue leaves to its reporter. One that proves larger than it reads comes back to hv with its reason.
- Issue 0670, in 3.3.0, on hv's ruling "ambiguous lines stop", the measured 1-row shape recorded in the issue: dc builds it once ST0091's packages are on main, to vc's answers to dc's two shape questions, which are in the issue's record, and vc judges it with the fleet census re-run against the build, which must redden Devbin ST0011 AT-3.5 and nothing else.
- ST0090, issue 0664's fix: one criterion is left, AC-02.4. hv sees Intent.app's Rebuild Search Index row show its second line, `reconciled <age> ago`, after a rebuild, on a build at or after a54f2e9e0 whose CLI carries 8406f6a15: the 3.3.0 cut, or a dev app pointed at a dev CLI, which is the maintainer's alone and which dc builds again on vc's word, since the Debug app of 2026-11-01 is gone. vc then records that build on AC-02.4 and dc closes WP-02 (vc hold 6, dc hold 37). `intent ac gate ST0090` names what is left.
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
