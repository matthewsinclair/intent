---
node: ic
name: Interface Claude
role: interface
session_id: 9031fbab-80b1-4564-a880-93c9e79197d4
heartbeat_at: 2026-10-27 05:44Z
status: active
focus: "Folded and released for hv's end of day, holding until hv says go on: doing 135 is Part B's six-bank train (page v2 third, hv's pipeline-cell-width-v4 sixth, tree 229c6053f4a2 on ddb749ca7), judged when molt-vc grants Intent's slot, a red fixed forward in tmp/wt-ic-0086b, then ic's live look at the cards; todo 58 owes design.md's cell sentence after the landing; hold 12 is ic's docs/reference regeneration at the cut (hv's ruling); hold 15 waits for molt-vc's Spotlight word; no claim, no push, no release"
claims: []
---

# Interface Claude (ic)

## DOING

- RESUME STATE (ic, end-of-day localfold on hv's word via molt-vc, 2026-10-26): ST0086 WP-04 Part B, the page face of AC-04.6, on vc's orders (vc's word). Part B's train, as vc records it, is six banks applied in order on main: cc's refs/bank/cc/0086/wp04b-status-v2 (patch-id 4ae2585f7479) and wp04b-wait-v2 (2421b3d9719d), then ic's refs/bank/ic/0086/wp04b-page-v2 (blob a10e09b24211, patch-id 8126a568631b), then cc's wp04b-init-v1 and wp04b-ats-v1, then ic's refs/bank/ic/0086/pipeline-cell-width-v4 (blob f2dc986d8c7c, patch-id e058514c0734). On 00eb61a2e the tree after five is 2ca7692e71da and after six is c45a889677a8 (ic's and vc's measurement, 17:17Z); on ddb749ca7 all six still apply in order and the tree after six is 229c6053f4a2 (ic's temp-index measurement, which vc noted for the judge). Page v2: each lane lists its queued asks under its holders (`lanes[].queue`), and a controller.json that will not parse reads `control  record UNREADABLE (named below)` in the band, never NO CONTROLLER, with `controller record UNREADABLE <path> -- <why>` first below the band; the page-keys test requires `controller_unreadable` and carries the lane arm (ST0086 AT-04.6). Cell-width v4 is hv's direct ask in ic's session ("We need to adjust the width of these things. That's bonkers.", then four lines for the main line, 18rem, and two lines for the meta line): `.cell` is `flex: 0 0 18rem; min-width: 0`, `.what` clamps at four lines and `.meta` at two, CSS only in shell.html, no CHANGELOG hunk; v1 to v3 are superseded. The judge runs when molt-vc grants Intent's slot (Intent is behind Devbin and Utilz); its render leg runs ic's render tools v3 (refs/bank/ic/tools/render-states.sh 96c9f5840582, prelude.js 942b1e87c794). A red comes back to ic in tmp/wt-ic-0086b (kept until the landing); if a bank under ic's moves, ic re-banks over the new tree. After build all, ic looks at the cards on the live page. No claim: the work package is cc's. No push, no release. (edited)

## TODO

- After pipeline-cell-width-v4 lands with Part B's train, add the cells' shape to ST0086 design.md's pipeline paragraph (as-written lags as-built). The draft is banked at refs/bank/ic/0086/design-cells-v1 (blob 93bf3e0a61c3, patch-id 917d2f6ce431) over design.md blob a435b11823b6: one sentence after "oldest ask first.", reading "Every cell is 18rem wide whatever its text (hv's ask in ic's session, 2026-10-26): its main line, what the request runs, wraps to at most four lines and its meta line to two before either ends in an ellipsis, and its title carries the request's id, that text in full and its requester." Each clause was read against cell-width-v4's CSS (`.cell` at `flex: 0 0 18rem` with `min-width: 0`, `.what` clamped at four lines, `.meta` at two) and the title shell.html's `cell()` sets (id, what, requester). The paragraph is one line, so if cc's design.md edits after the green judge reach it first, the sentence is re-applied by hand onto the new file. It lands through `intent st attach ST0086 design.md --from <file>`, or the file plus `intent sync --to-store`, inside a chain, board and doc paths only. (edited)

## Holds

- docs/reference regeneration at the 3.2.7 cut (was todo 52): hv ruled on 2026-10-26 that ic regenerates docs/reference at the cut (vc relayed it in its chain for decision 96). The step is intent/docs/releasing.md's step 2, read 2026-10-27 on vc's order: from this tree, BASE is `git describe --tags --abbrev=0`; gen_reference.sh and gen_cut_surface.sh both run at `--rev HEAD --baseline "$BASE"`, the second with `--out` naming the file cut-surface.md, into scratch first with the diff read, and both halves land in ONE commit before the driver runs; reference_current_check.sh says when the set is current. Neither generator runs a binary, so the regeneration puts no load on the machine, but it sits on the cut's final HEAD, because Part B's wait-v2 and init-v1 move the register again. At 4bcfdf1e4 the check exits 1, stale against v3.2.6 on every page, with intent-estate.md new. RELEASES WHEN: the 3.2.7 cut begins (hv's call) with everything it carries landed. (edited)
- Remove ic's stale worktrees, all but tmp/wt-ic-0086b (Part B's, kept until the landing): tmp/wt-ic-0086, -0086p, -0086t, -0086w, -0087, -0652, -a4 (A4, issue 0655, landed at a6ce22c3b with patch-id 9dacf04295a4), -app, -arms2 and -tuidoc. 0652, arms2, a4 and tuidoc hold uncommitted files (counted 2026-10-26 17:20Z), so check each against its landed commit or bank from outside it first, by in-whiteboard's Folding step (0654, with 0656's fix-forward), then `git -C <path> reset --hard` and a plain `git worktree remove`, never --force. RELEASES WHEN: molt-vc says hv's ~/Devel Spotlight exclusion is in. Until then no node removes any worktree (vc's order after 0654). (edited)

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
