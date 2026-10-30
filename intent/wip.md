---
verblock: "2026-10-30:v3.44: vc - marked to market on hv's order; hv controls build and release"
intent_version: 3.2.7
---

# Work In Progress

## DOING

- hv controls build and release from 2026-10-30 18:12Z ("I'm take control of the build and release process. I want every Cladue to mark-to-market and check in everthing to head on main and then I will run the full test suite in each case manually."). Every Intent node has marked and checked in to HEAD on main; hv runs each project's full suite by hand. vc still holds the estate desk, with nothing granted.

## TODO

- ST0087 is 18/19: AC-05.1 ran at 18:21Z on hv's `!` in dc's session and failed at the dev leg (driver rc=1; banked under refs/bank/dc/0087-05/run1/). After `intent use dev` both links and intentd were the dev pair, and ~/.local/bin was first on the leg's PATH, but the leg shell's bare `intent --version` answered brew 3.2.7: zsh had hashed /opt/homebrew/bin/intent when it ran `intent use dev` before the link existed, and reused it until a `rehash` (dc's probe, run1/hashprobe/). So the kit's legs owe a `rehash` after each switch, and a re-run needs hv's `!`; hv rules, and nothing re-runs without hv. Parked with it for hv, from dc's mark (cb8ddf9d8): doctor's shadow check reads PATH's first intent through a child /bin/sh, so it was right by its definition, but neither doctor nor bare `intent use` compares the binary running it with the one the link set serves, and `intent use <tree>` does not tell a shell that already ran intent to `rehash`.
- hv: the 3.2.8 cut (0663, which hv approved on the preview: "LGTM. Ship it."), after ST0087. Step 2 of intent/docs/releasing.md is done: docs/reference 63424a6db, docs/known-defects.md 745cd4365, the explorer shots byte-identical, the CHANGELOG's 3.2.8 section complete. Left: the Linux preflight at the cut's HEAD (kovacs-sysadm fetches and reads it; hv starts its second line with `!` there), then step 3 onward.
- Parked for hv: issues 0649, 0650 and 0661 (its class is wider: `claude skills`, `subagents` and `rules`, and list/show silently accepting `--dry-run`); threads ST0060, ST0077, ST0082, ST0084, ST0085, ST0089.
- Found 2026-10-30, parked for hv: the home pointer names the dev tree while the machine rests on brew; a def search held a core about 9 minutes on Laksa's index, and a bare Elixir module name answers empty; in a Swift class carrying `@main`, the outline and `--kind def` put every member at the class's `@main` line (utilz-ic's finding in Utilz, reproduced by utilz-vc); the Elixir critic's rules apply to `lib/**/*.ex`, so `bin/lib/*.exs` has no gate coverage; `wb archive message --handled` with no `--to` also archives messages to hv, and wb has no unarchive (vc's fold order did that in Laksa, dc's 37 and vc's one, and in Conflab, dc's one and ic's two; all still in the store).
- Parked for hv: the older evidence in `~/work/intent-vc/` on kovacs stays until hv rules (vc, 2026-10-30); dc's AC-05.2 files there may go once ST0087 is done, since they are banked.
- Deferred: the critic contract for installed subagents; devbin 0227's formula renderer (precondition met); the intentd CPU item (not reproduced 2026-10-30); ic's `/refresh`; devbin-vc's repo-setup question and `--ivault`.
- Carried for hv: Trellis's session hooks, Cdtempl's uncommitted work, the gyges clean-Mac re-proof, the Sinclair-Shared book recreate, the Conflab and Baize repairs, a brew ruby on kovacs.
- Banked outside the estates' trees, for hv to rule: Laksa's CV panel (ST0111 AC-05.2, tmp/ic/st0111panel/BANK.md) and 0049 (tmp/dc/0049/); Gtools' branches st19/w3, st19/g8 and local/ic-wp08. The other estates' parked lists are in their own records: Utilz hold 24, Gtools' intent/wip.md, Laksa's hold 20 and intent/wip.md, Conflab's intent/wip.md, Devbin hold 34.
