---
verblock: "2026-11-12:v3.76: vc - 3.3.1 set (hv decision 44); ST0090 AC-02.4 satisfied on hv's look"
intent_version: 3.3.0
---

# Work In Progress

## DOING

- P0, on hv's word of 2026-11-12 (hv decision 39): the next devbin release, 0.1.23 with issues 0333 to 0339, and the next Intent release, one cut each, on the one plan hv approved (hv decision 40: devbin-cc's PLAN-20261112 with vc's corrections). devbin-vc holds the Desk and lands devbin's units; vc lands Intent's on each builder's evidence.
- Intent's next release carries everything on main since v3.3.0, ST0093 WP-06 (c) to (e), issue 0687's fix, so a Debug build of the app never registers the login item, and issue 0688's, a parity tool's unbounded path guard. The order: hv's cut of devbin 0.1.23, whose next run carries three P0 fixes after three cuts went red before any tag; devbin-vc sweeps it into Intent's tree first; dc proves WP-06 (c) to (e), 0682, 0687 and 0688 on it in one proof run, and vc lands them; ic re-drives docs/known-defects.md on the pair built at HEAD, with the explorer's shots only if something the explorer draws changed; one Desk booking, dc's, runs `bin/devbin release gates --leg kovacs` and then a dry run for its pre-flight; dc reads that the formula keeps both Linux musl assets; hv cuts Intent once, as 3.3.1 (hv decision 44), which heads the CHANGELOG's open section. cc's fix for `intent app` taking a LaunchServices record whose process has gone for a running app rides it. The macOS sign and notarise stage runs only in a cut, before the tag, so its first run is the cut itself: a failure stops it untagged, and it is cut again. Issue 0682 closes on devbin 0334 plus WP-06 (d).
- ST0093, why Claudes yak-shave: WP-02's skills sync waits on the next release, and WP-06 is dc's under its hold 41.

## TODO

- ST0090: dc closes WP-02, ST0090 and issue 0664, AC-02.4 being satisfied on hv's look at the installed 3.3.0 app. hv checks that Login Items names /Applications/Intent.app and not the 3.2.6 Debug build.
- ST0094, the estate's last two boards (moved out of ST0092), is not part of this release (hv decision 42).
- After Intent's next cut, one visit to each estate, Intent first (hv decision 41): devbin-vc's sweep of devbin 0.1.23 and `intent claude upgrade --apply`. It replaces the hand fix in Trellis's AGENTS.md, so Trellis moves anything bespoke out of that file first.
- agentic-engineering's P1, a devvm that bakes both releases from their taps, waits on them: vc tells agentic-engineering-vc when Intent's is installable.
- Open with hv, off P0: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- Parked for hv and unchanged since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089; Laksa's and Conflab's releases; the items parked on 2026-10-30 and at dc's mark at ST0087; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
