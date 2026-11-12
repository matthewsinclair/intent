---
verblock: "2026-11-12:v3.71: vc - the kovacs line goes: GNU parallel is there, and hv kept the TMPDIR export"
intent_version: 3.3.0
---

# Work In Progress

## DOING

- P0, on hv's word of 2026-11-12 (hv decision 39): the next devbin release, 0.1.23 with issues 0333 to 0339, and the next Intent release, one cut each, on the one plan hv approved (hv decision 40: devbin-cc's PLAN-20261112 with vc's corrections). devbin-vc holds the Desk and lands devbin's units; vc lands Intent's on each builder's evidence.
- Intent's next release carries everything on main since v3.3.0, and ST0093 WP-06 (c) to (e). The order: devbin's pre-cut, and hv's cut of devbin 0.1.23; devbin-vc sweeps it into Intent's tree first; dc builds WP-06 (c) to (e) on it with one proof run, and vc lands it; one Desk booking, dc's, runs `bin/devbin release gates --leg kovacs` and then a dry run for its pre-flight; ic regenerates docs/reference; dc reads that the formula keeps both Linux musl assets; hv cuts Intent once, at the number hv sets (vc recommends 3.3.1). The macOS sign and notarise stage runs only in a cut, before the tag, so its first run is the cut itself: a failure stops it untagged, and it is cut again. Issue 0682 closes on devbin 0334 plus WP-06 (d).
- ST0092, Claude Team profiles: `intent ac gate ST0092` names what is left. WP-09 closes on hv decision 41, which rules that no WP-09 class gates. WP-11 waits on Arca's supervisor committing its tree (AC-11.4), on molt-vc citing the terms in Molt's guide (AC-11.6), and on agentic-engineering's board converging through its own vc (AC-11.7).
- ST0093, why Claudes yak-shave: WP-02's skills sync waits on the next release, and WP-06 is dc's under its hold 41.

## TODO

- ST0090 AC-02.4: hv looks at the Rebuild Search Index row of /Applications/Intent.app (3.3.0); vc records it, and dc closes WP-02 and issue 0664.
- After Intent's next cut, one visit to each estate, Intent first (hv decision 41): devbin-vc's sweep of devbin 0.1.23 and `intent claude upgrade --apply`. It replaces the hand fix in Trellis's AGENTS.md, so Trellis moves anything bespoke out of that file first.
- agentic-engineering's P1, a devvm that bakes both releases from their taps, waits on them: vc tells agentic-engineering-vc when Intent's is installable.
- Open with hv, off P0: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- Parked for hv and unchanged since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089; Laksa's and Conflab's releases; the items parked on 2026-10-30 and at dc's mark at ST0087; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
