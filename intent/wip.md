---
verblock: "2026-10-30:v3.43: vc - back from the compact, the 3.2.8 cut's step 2 under way"
intent_version: 3.2.7
---

# Work In Progress

## DOING

- Fleet sweep on hv's word (vc decision 104): every estate is back from hv's compact and lands its post-compact pickup; Laksa works the CV panel and 0049 under the desk, and the other estates hold on their parked lists. vc holds the estate desk.
- ST0087 is 18/19: AC-05.1 waits for hv, back from dinner, to run the banked ac051-v3.sh with `!` in dc's session (dc's classifier refused it). vc grants dc's queued ask on bats,cargo,heavy,launchsvc and sends WINDOW START first; then WP-05 done and ST0087 done.
- The 3.2.8 cut's step 2 (intent/docs/releasing.md): ic regenerates docs/reference against v3.2.7 and the explorer shots, and cc re-drives docs/known-defects.md against the dev pair at 9b5e3e4fe8a1. The CHANGELOG's 3.2.8 section is complete: 0663 is the only user-facing change since v3.2.7.

## TODO

- hv, once ST0087 closes: the Linux preflight on kovacs at that HEAD (`bin/int prepush --preflight-covers HEAD` prints its two lines, and only board-only commits land between it and the push), then the 3.2.8 cut (0663, which hv approved on the preview: "LGTM. Ship it."), its step 4 and the site. The cut's own push carries main.
- Parked for hv: issues 0649, 0650 and 0661 (its class is wider: `claude skills`, `subagents` and `rules`, and list/show silently accepting `--dry-run`); threads ST0060, ST0077, ST0082, ST0084, ST0085, ST0089.
- Found 2026-10-30, parked for hv: the home pointer names the dev tree while the machine rests on brew; a def search held a core about 9 minutes on Laksa's index, and a bare Elixir module name answers empty; in a Swift class carrying `@main`, the outline and `--kind def` put every member at the class's `@main` line (utilz-ic's finding in Utilz, reproduced by utilz-vc); the Elixir critic's rules apply to `lib/**/*.ex`, so `bin/lib/*.exs` has no gate coverage; `wb archive message --handled` with no `--to` also archives messages to hv, and wb has no unarchive (vc's fold order did that in Laksa, dc's 37 and vc's one, and in Conflab, dc's one and ic's two; all still in the store).
- Parked for hv: the older evidence in `~/work/intent-vc/` on kovacs stays until hv rules (vc, 2026-10-30); dc's AC-05.2 files there may go once ST0087 is done, since they are banked.
- Deferred: the critic contract for installed subagents; devbin 0227's formula renderer (precondition met); the intentd CPU item (not reproduced 2026-10-30); ic's `/refresh`; devbin-vc's repo-setup question and `--ivault`.
- Carried for hv: Trellis's session hooks, Cdtempl's uncommitted work, the gyges clean-Mac re-proof, the Sinclair-Shared book recreate, the Conflab and Baize repairs, a brew ruby on kovacs.
- The other estates' parked lists are in their own records: Utilz hold 24, Gtools' intent/wip.md, Laksa's hold 20 and intent/wip.md, Conflab's intent/wip.md, Devbin hold 34.
