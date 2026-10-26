---
verblock: "2026-10-25:v3.24: vc - globalfold at hv's EOD"
intent_version: 3.2.6
---

# Work In Progress

Measure before believing anything below: `.claude/restart.md` carries the commands, `intent/restart.md` the rules every bank, train and landing runs under. No count appears in this file.

## DOING

Every item below is banked, verified by vc against its author's worktree byte for byte, and booked at molt-vc's desk. Tomorrow each one re-confirms its bytes against main's HEAD, or rebases and re-banks, and sends READY again; nothing runs until molt-vc's GO.

- **ST0086 WP-04 PART A, THE SLOT PROTOCOL (cc with ic; vc decisions 88 and 89).** It fills the estate page's pipeline strip, QUEUE and LANES, which read "nothing running" until a controller books through slots. Part A is `refs/bank/cc/0086/wp04a-v2` (patch-id 6cd0eaa98a4b) over main cb6e84344, and ic's page and TUI half is `refs/bank/ic/0086/wp04a-page-v3` (patch-id c932e3dd4ff4) on top; the combined tip e497aafef is the judged tree, one heavy (the whole workspace suite, fmt, both clippy legs, the critic per file, the generator check, ic's three-state page render). vc lands Part A, then the page, then a build all; molt-vc switches its desk from messages to `ask` only when it agrees. Part B follows: wait, watch, the prompt hook naming pending asks, `estate init`, the TUI.
- **0648, THE CRITIC'S NOTHING-ASKED EXIT AND THE SWIFT PROXIES (cc).** `refs/bank/cc/0648/v5` (patch-id ecd5196ea0e7) over 5be439e7d, after a red judge whose every failure was 0648's own and a leak vc caught (the issue number in INV-04's rule, which `intent llm guide` prints into every project). Exit 4 when no rule was asked of any file; the gate and the advisory pass on 4; IN-SW-CODE-006 arms `try!`, `as!` and a postfix force unwrap headless (vc decision 85). Judge booked; vc lands and closes 0648.
- **ST0087 WP-04, THE FORMULA AND THE DOCS ENDING ON BREW (dc; vc decision 80).** `refs/bank/dc/0087-04/v2` (patch-id 75ca61f5cc6b) over aaeaf3b16: the formula writes `libexec/.intent-install` with `managed_by: brew`, `int brew render` and `int brew publish` replace `formula` and `--push`, `int local install|use|uninstall` retire, releasing.md ends on `intent use brew`, install.md's brew upgrade is `brew update` then `brew upgrade` then `intent daemon restart`. The judge's smoke leg is green only when its failures are exactly the two arms that cannot pass before 3.2.7's own keg (the manifest, the linked keg).
- **ST0086 AC-02.2 AND AC-02.6 ARMS (ic).** `refs/bank/ic/0086/arms-v1` (patch-id 657530663c3a) over 7acd43211: the usage endpoint read through an `EndpointTools` seam with stubs and fixture tokens, every failure named; the census round reaping on the tick and reading the endpoint only when asked. Judge booked; vc lands and writes the AT rows.
- **INTENT.APP (ic).** The installed app predates ST0087 WP-03's `LoginShell.swift`; ic's app build and install is booked, then hv sees the menu.
- **THE UPSTREAM PUSH (devbin-vc under hv's (b); vc decision 90).** `local` is pushed to edea1302f. `upstream` was refused by the pre-push hook for want of a green Linux preflight; it is booked as a kovacs cargo run at 38d2b9775 for kovacs-sysadm, offline at EOD. A green covers its sha and later commits under `intent/whiteboard/` and `intent/.canon/events/` only: the record commits after 38d2b9775 need a fresh run at the HEAD of the day. After a green END, vc tells devbin-vc, who pushes fast-forward only.

## TODO

- **ST0086 WP-03's package state:** its arms landed and its criteria are green (AT-03.1, AT-03.2), but the package reads Not Started; ic runs `wp start` and `wp done`. ST0086 WP-02 reads WIP; its AC-02.2 and AC-02.6 arms above close it.
- **ST0087 WP-05, the proof per tree** (vc with dc): `use dev`, `use opt`, `use brew`, each with the version line, doctor and the shadow check, from a fresh login shell; the kovacs half after the tap carries 3.2.7.
- **3.2.7's cut** (decision 76: one release): the cut-side work (todo 50), the Linux check on kovacs, hv's cut at the terminal. Every estate then takes the new root templates (no `/in-standards`) and 0644's chain block at its own `intent claude upgrade --apply`, run by that estate's vc, at the latest in the fleet sweep after the cut.
- **AFTER 3.2.7, low:** 0649 (the critic defers two Rust rules to a clippy no gate runs), 0650 (`.githooks/` ships in release archives; export-ignore in a marked block), 0651 (the no-PM-state guard does not read the agent guide), and making the critic contract readable to installed subagents (hv's answer of 2026-10-25, decision 89).
- **CARRIED FROM 2026-10-24, NOT RE-MEASURED TODAY:** D55 and ST0081 (WIP, blocked on AC-02.1 and devbin's re-vendor); ST0085 (Triage, waiting on hv's read of its inventory); hv's hand on Trellis's session hooks, Cdtempl's uncommitted work, the gyges clean-Mac re-proof and the Sinclair-Shared book recreate; 0631-era items behind the line (ST0084, ST0060, ST0077, ST0082); hv decision 28's Conflab and Baize repairs; ic's `/refresh` in the explorer; the machine's resident load (hv's call).
