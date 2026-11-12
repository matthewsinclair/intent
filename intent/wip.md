---
verblock: "2026-11-12:v3.80: vc - Intent's work before devbin's sweep is done; the rest waits on devbin 0.1.23"
intent_version: 3.3.0
---

# Work In Progress

## DOING

- P0, on hv's word of 2026-11-12 (hv decision 39): the next devbin release, 0.1.23 with issues 0333 to 0339, and the next Intent release, one cut each, on the one plan hv approved (hv decision 40: devbin-cc's PLAN-20261112 with vc's corrections). devbin-vc holds the Desk and lands devbin's units; vc lands Intent's on each builder's evidence.
- Intent 3.3.1 (hv decision 44) carries everything on main since v3.3.0, ST0093 WP-06 (c) to (e), issue 0689's fix, so `intent app` no longer takes a LaunchServices record of an exited app for the running one, and issue 0690's, so `intent daemon start`'s remedy names a piped stdin over ssh. Now: devbin's one proof run of its last fixes (0334-2, 0341-1, 0340-1 and 0340-2), then hv's history rewrite and cut of devbin 0.1.23, after three cuts went red before any tag; devbin-vc sweeps it into Intent's tree first; dc proves WP-06 (c) to (e), with the serial tags 0.1.23's 24-job floor needs, and 0682 on it, and vc lands them; one Desk booking, dc's, runs `bin/devbin release gates --leg kovacs` and then a dry run for its pre-flight; at the cut's HEAD, ic reads that reference_current_check.sh exits 0 and that DIRT_SCOPE has not moved since 9500cb77a, the pair docs/known-defects.md was driven on; dc reads that the formula keeps both Linux musl assets; hv cuts v3.3.1 once. The macOS sign and notarise stage runs only in a cut, before the tag, so its first run is the cut itself: a failure stops it untagged, and it is cut again. Issue 0682 closes on devbin 0334 plus WP-06 (d).
- ST0093, why Claudes yak-shave: WP-02's skills sync waits on the next release, and WP-06 is dc's under its hold 41.

## TODO

- hv checks that System Settings' Login Items names /Applications/Intent.app and not the 3.2.6 Debug build, which issue 0687's fix stops a Debug build from registering again.
- ST0094, the estate's last two boards (moved out of ST0092), is not part of this release (hv decision 42).
- After Intent's next cut, one visit to each estate, Intent first (hv decision 41): devbin-vc's sweep of devbin 0.1.23 and `intent claude upgrade --apply`. It replaces the hand fix in Trellis's AGENTS.md, so Trellis moves anything bespoke out of that file first.
- agentic-engineering's P1, a devvm that bakes both releases from their taps, waits on them: vc tells agentic-engineering-vc when Intent's is installable.
- Open with hv, off P0: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- Parked for hv and unchanged since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089; Laksa's and Conflab's releases; the items parked on 2026-10-30 and at dc's mark at ST0087; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
