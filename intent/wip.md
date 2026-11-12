---
verblock: "2026-11-12:v3.85: vc - localfold for compact: WP-06 landed, cc fixes the view-count red"
intent_version: 3.3.0
---

# Work In Progress

## DOING

- P0, on hv's word of 2026-11-12 (hv decision 39): the next devbin release, 0.1.23 with issues 0333 to 0339, and the next Intent release, one cut each, on the one plan hv approved (hv decision 40: devbin-cc's PLAN-20261112 with vc's corrections). devbin-vc holds the Desk and lands devbin's units; vc lands Intent's on each builder's evidence.
- Intent 3.3.1 (hv decision 44), cut tonight by devbin-vc on vc's "ready to cut" (hv decision 49, confirming devbin decision 170), by intent/docs/releasing.md and only when green. devbin 0.1.23 is released and swept into Intent at a8a59219c, with arm 10 of the shared-artefact guard narrowed to leave bin/.devbin/lib/pages/ out; cc read it clear for brew. ST0093 WP-06 (c) to (e) is landed at 7be4d570f on dc's evidence: rhadamanth's suite green under devbin's lanes runner, and gyges' gate rust with a 1.7G TMPDIR free of v4's install-tree reds. Next, in order: (1) cc fixes intentsvcs/tests/no_view_claims_to_be_truth.rs:292 (1156 views walked against 1148 rendered, because the walk counts an inbox for every pair of boards and the team board renders none between hv and any seat but vc). It is red on main, and it would stop the cut's gate rust and the kovacs leg. (2) vc lands it and freezes the tree: every seat commits its board, and nothing writes to Intent's tree until the cut has finished. (3) dc asks devbin-vc's Desk for `bin/devbin release gates --leg kovacs` at that clean HEAD (devbin 0337 refuses kovacs at a cut unless this rehearsal went green at a commit with the same leg config), then the dry run; nothing changes release.legs, release.gates or what they name after it. (4) ic's two reads at that HEAD (hold 22). (5) vc sends devbin-vc "ready to cut" with the tip's full sha; devbin-vc runs step 0's `intent daemon stop`, the dry run and `bin/devbin release v3.3.1`, cut publish and brew smoke, then `intent use brew` and the app swap. The sign and notarise stage first runs in this cut, so a keychain or notary prompt may need hv. Issue 0682 closes on dc's Resolution before the freeze.
- ST0093, why Claudes yak-shave: WP-02's skills sync waits on the next release.

## TODO

- intent.laksa.io: ic pushes c14c60e on hv decision 50 and reads the live page for 3.3.0; the same step brings it to 3.3.1 after the cut.
- ST0094, the estate's last two boards (moved out of ST0092), is not part of this release (hv decision 42).
- After Intent's next cut, one visit to each estate, Intent first (hv decision 41): devbin-vc's sweep of devbin 0.1.23 and `intent claude upgrade --apply`. It replaces the hand fix in Trellis's AGENTS.md, so Trellis moves anything bespoke out of that file first.
- agentic-engineering's P1, a devvm that bakes both releases from their taps, waits on them: vc tells agentic-engineering-vc when Intent's is installable.
- Open with hv, off P0: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- Parked for hv and unchanged since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084, ST0085 and ST0089; Laksa's and Conflab's releases; the items parked on 2026-10-30 and at dc's mark at ST0087; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
