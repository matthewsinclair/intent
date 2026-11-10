---
verblock: "2026-11-10:v3.67: vc - ST0092's terms are ratified and cc builds WP-02"
intent_version: 3.2.8
---

# Work In Progress

## DOING

- 3.3.0, the one next release, carrying everything outstanding, on hv's word of 2026-11-10 (hv directive 1, vc decision 109): there is no 3.2.9, and nothing rides a release after. Since hv's word that day to make Intent ready for its release (vc decision 116), devbin-vc holds hv's pen and the estate's desk while hv is away, and vc directs cc, dc and ic. Devbin 0.1.21 is re-cutting, and Intent's sweep of it waits on devbin-vc's word that the tap carries it. Each seat's lane is on vc's board (vc todo 77), and every heavy run is booked with the desk.
- ST0092, Claude Team profiles (vc decisions 107, 111 and 121, vc todo 76): WIP, gate 5 of 65. hv ratified the terms document, `intent/plugins/claude/teams/TERMS.md`, on 2026-11-10 (hv decision 34), and WP-01 is done. cc builds WP-02, then WP-03, WP-06 with issue 0675, WP-05, WP-07, WP-04 and WP-09, with WP-08 beside any; ic reads each bank's surface rows and builds WP-10; vc judges each bank. The rulings made since the packages were written are in the bodies of WP-02 to WP-09. Anything driven live runs on a clone of Intent's board until the cut. The estate sweep, WP-11, follows the cut.
- ST0091, Intent cuts through devbin's release core: the four packages are on main and judged, and its gate reads 10 of 12 (vc decision 119, vc todo 71). AC-01.3 waits on the sweep of Devbin's next release, which carries the pre-flight seam (devbin 0327) and the release commit's trailer (devbin 0328); dc then wires `release.preflight` and writes the arm AT-01.6 cites. AC-04.3 is hv's first cut through the core, which is 3.3.0, with an annotated tag. Intent keeps its own macOS publish and asset names in 3.3.0, and the cut publishes through one `after:` command that stops at its first failure (vc decision 118). docs/reference is regenerated against v3.2.8 as the last unit before the cut, and `gate reference` refuses until then.

## TODO

- Owed by hv to vc (vc todo 77): the run of the one-message rewrite just before the push, and a ruling on the client's name that upstream already carries (vc watchout 34); whether issues 0678 and 0667 ride 3.3.0 (vc recommends 0678 does not, and 0667 is unbuilt and unassigned); the Desk sentence in the ratified terms, which says one vc on a host where vc's ruling for WP-08 says the board's mediator; whether Intent keeps an AT's own red-then-green; and hv's eye on ST0090's row on a 3.3.0 build.
- The push of main to local and upstream alone (vc hold 8) waits on four things. hv runs the rewrite of one commit message, with every seat held. A build all at the HEAD to be pushed passes on the desk, with every seat holding its commits until the push. A Linux preflight at that HEAD passes on kovacs. Then devbin-vc pushes on vc's word naming that HEAD (vc decision 90). It carries everything on main since v3.2.8. No release rides on it.
- Issue 0678, declared build and test commands: filed by ic when 0672 was narrowed, not built, and hv's to place (vc recommends after 3.3.0). The FMT-02 set in the shared trope catalogue belongs to llm-tropes first and is parked for hv.
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
