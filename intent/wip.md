---
verblock: "2026-11-17:v3.99: vc - rehearsal 3 green; the 3.4.0 cut waits on hv's return from the move"
intent_version: 3.3.3
---

# Work In Progress

## DOING

- The 3.4.0 cut (hv decisions 58 and 60). No issue is open, and the green rehearsal the cut's pre-flight takes is kept in tmp/release/legs/kovacs (rehearsal 3 at 0d8c7a306). It waits on hv's return from the move, because the Desk grants nothing until then. Then dc books the Desk's hold of rhadamanth and kovacs for the cut's legs, vc folds every seat so none writes the canon tree, and hv runs in hv's own terminal `intent daemon stop`, then `bin/devbin release v3.4.0 --dry-run` (read every row; its four pre-flight checks must pass), then `bin/devbin release v3.4.0` (answer its one confirm; never `--yes`), and after its publish and `brew smoke --at-tag`: `intent use brew`, `intent --version`, `intent doctor`, and `intent daemon status`, with `intent daemon start` if it says absent. Then, optionally, releasing.md's Intent.app swap and the site's version. cc's hold 38 (re-drive the known-defects page on a red rehearsal) has nothing left to wait for: cc archives it at its next pickup.

## TODO

- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16, and issue 0703 made the shell suite independent of it.
- ST0089 stays parked (hv decision 59): its policy needs hv's call first, since "fold at a task boundary, not at a token count" conflicts with hv's compact-at-200-250k rule.
- The rest of the estate visit (hv decision 41): Intent's is done; the other estates follow, with the fleet's root files upgraded after this cut. Trellis moves anything bespoke out of its AGENTS.md first.
- ST0094, the estate's last two boards.
- Open with hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own. devbin-cc holds 0326 now.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list; the banks kept off main. Their text is `git show f16dce4a7:intent/wip.md`.
