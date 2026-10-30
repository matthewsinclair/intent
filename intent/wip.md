---
verblock: "2026-10-30:v3.48: vc - globalfold at hv's close"
intent_version: 3.2.8
---

# Work In Progress

## DOING

- Nothing is in flight. hv controls build and release.

## TODO

- Laksa's and Conflab's releases wait for hv's next word. On rhadamanth, Conflab runs its checkout's build (520987af, v0.6.2-468) through ~/.local/bin and ~/bin links, with brew's 0.6.2 installed but unlinked, so it moves to brew at its next release.
- Devbin 0295, for the next devbin release: stamp first, test once, tag what passed. The kovacs run stays (hv).
- Parked for hv, from 2026-10-30:
  - Intent's cut wants one driver: five commands follow `build release`, and `bin/int brew publish` refuses under `!` because it confirms at a terminal.
  - `bin/devbin macos app-install` leaves an unsigned dev build in /Applications that nothing switches back. Intent.app's title and menu show the CLI's version; only About shows the bundle's.
  - Gtools.app has no brew route: brew's `gtools app install` refuses, and the release ships no app. On gyges it builds only from a GUI login session, since codesign cannot reach the keychain from gyges-sysadm's Background session. Brew warns that the gtools formula's `post_install` is deprecated.
  - Utilz's tests.yml Rust job has no `timeout-minutes`, and prez's Chrome render has no timeout.
  - Gtools CI got ubuntu-24.04 where Utilz and Intent got 26.04; pinning it is hv's call.
  - Both Macs' CoreSimulator (1174.9.2) is older than Xcode's (1174.21.0).
  - `utilz doctor` said "gh is not installed" with /opt/homebrew/bin/gh on PATH (seen once, not traced).
- Parked for hv: issues 0649, 0650 and 0661; threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089.
- Parked for hv, from dc's mark at ST0087 (cb8ddf9d8):
  - Doctor and bare `intent use` do not compare the running binary with the link set's, and `intent use <tree>` does not say to `rehash`.
  - The home pointer names the dev tree while the machine rests on brew.
  - A def search held a core about 9 minutes on Laksa's index, and a bare Elixir module name answers empty.
  - In a Swift `@main` class, the outline and `--kind def` put every member at the `@main` line.
  - `bin/lib/*.exs` has no Elixir critic coverage.
  - `wb archive message --handled` with no `--to` also archives messages to hv, and wb has no unarchive.
- Parked for hv: Intent's git stash holds four entries no node owns: stash@{0} (2026-09-17, ST0076 WP-01 work at ee57d4288), stash@{1} (2026-09-01, all on main), and stash@{2} and stash@{3} (2025-08-20, v2 era).
- Deferred: the critic contract for installed subagents; devbin 0227's formula renderer; the intentd CPU item; ic's `/refresh`; devbin-vc's repo-setup question and `--ivault`.
- Carried for hv: Trellis's session hooks, the gyges clean-Mac re-proof, the Sinclair-Shared book recreate, the Conflab and Baize repairs, and a brew ruby on kovacs.
- Off main for hv: Conflab's refs/bank/ic/todo24-v5 (the gate refuses it on 31 IN-SW-CODE-006 warnings already on main); Gtools' st19/g8 (conflicts in five files); Laksa's refused and conflicting banks. hv owes a read-only production query for duplicate content_assets rows before vc todo 60's fix deploys. Each estate keeps its own parked list in its intent/wip.md.
