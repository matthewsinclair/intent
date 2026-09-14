---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
heartbeat_at: 2026-09-14 22:01Z
status: paused
focus: "FOLDED FOR THE DAY 2026-09-14: 0338 (ii) BANKED, landing on tomorrow's matched control of the index arm, wt-cc as judged on d2491639b; then 0338 (i), 0377, 0331 comments, 0375; 0321 held for hv. NO RELEASE, NO PUSH."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **0375, LAST, AFTER 0338 (ii), 0338 (i), 0377 AND 0331 COMMENTS (vc, ruled at the fold).** At S: a sixth WbItemKind `directive`, legal on hv only (`wb add directive --node hv`; any other node refuses naming hv and the protocol's `## Standing directives` section). Views render it under `## Standing directives` on hv's board only. `wb migrate hv` carries the section's lines as directive items; a non-hv board carrying the section refuses at migrate naming it. A fold never archives a directive. No DDL (kind has no CHECK). Register rows: the kind's value row and `wb add`'s roster row. in-whiteboard SKILL.md hv section names the kind and verb, through the skill's own canon path. Close note names the Laksa follow-up (laksa-vc re-carries hv's seven directives).
- **0338 (ii) BANKED, NOT LANDED (2026-09-14).** Patch scratchpad/0338ii-banked-d2491639b.patch, wt-cc left exactly as judged on d2491639b. Judged: red arm on base; intentsvcs 1635/0; intent-cli 932/0; clippy and rustfmt clean; guarded worktree sync-then-attach (only the 12 inbox renders restored, ST0057 canon diff attachment-only); intentd 50/1 on a_source_edit_reaches_the_index_and_costs_canon_nothing (vc's matched-control test) at load 344, so it did not land under vc's fully-green rule. Kit: 0338ii-land.sh, commit-0338ii.txt, 0338ii-note.txt (0338 stays OPEN with a dated (ii) note). TOMORROW'S FIRST ACT (vc, 2026-09-14): the matched control, as the index's owner: the arm alone on the base and on the banked tree, same host state, one run each, no third. (ii) lands on its result, with a fresh full run only if a code file moved under it.
- **THEN, IN vc ORDER, EACH ITS OWN COMMIT, THE WORD REQUEST AFTER ITS FULL RUN, THE REPORT SENT IN THE LANDING TURN.** 0338 (i): organize and st hydrate carry bytes, text or the opaque blob, and an unloadable blob is a refusal naming the path, never hydrated; the contract rows stay open on 0338 naming hv. Then 0377 (NoCache on the canon registration, measured on one board commit), 0331's comments half, and 0375 (its ruled shape is the todo below). 0321 held for hv (the log stamp against D42).

## Holds

- **THE DEFECT LIST ITEMS hv DID NOT RULE.** The mixed-proxy silent drop, the rule proxies that contradict their own rule (item 9, with the gate-blocked pair), and the usage-error exit code, which is dc's to rule. **Released when hv rules them, or vc routes one to me.**
- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.
- **0366 LANDED AT 524f5f868; dc VERIFIES IT.** dc runs the two-arm harness (0366 through --daemon search on a fresh daemon, the unfixed pair as control) and then the full workspace suite on that checkout, every target --no-fail-fast. **Released when dc reports: green closes it, a red comes back to cc.**

## Watch-outs

- **THE SHARED CHECKOUT.** `git add <paths>` then `git commit --only <paths>` in ONE call; against a peer's index lock re-issue the SAME command, never remove the lock, and judge by `git log -1`, never by the loop.
- **THE GATE.** Never `--no-verify`. Capture a commit's WHOLE output and read `rc` and `git log -1` before believing it landed; a filtered refusal reads exactly like success. The gate lints the rule library's own bad examples, so that pair cannot be committed at all.
- **THIS SHELL IS ZSH.** Unquoted `$var` does not word-split, an unmatched glob aborts the call, a bash script's functions sourced here run as zsh (drive them from a `bash drive.sh`), and an exit code that IS the finding never goes through a pipe.
- **BUILDS AND SUITES.** Only from a private worktree's own in-tree build under an isolated `HOME`; read back the home pointer and the live store's mtime afterwards, because a test run deploys to the estate it lives in. Since dc's build done at e3c67792c (2026-09-14 07:25Z) the pointer is `~/.local/share/intent/home` (XDG), not `~/.intent/home`, and it names this tree.
- **A FIX IS JUDGED BY THE WHOLE WORKSPACE, ON THE BASE IT LANDS ON.** 2026-09-14: 0388 was judged by its own arms and put one_clock red on main (fixed 6e8c1f675); group 2 was judged, then applied textually onto a base that had moved under facade.rs, which is a merge nobody compiled. So every judging run carries the full intentsvcs and intent-cli suites (vc), and a moved base is rebased and run again, not re-applied. And a zsh $var holding many paths is ONE git pathspec: it matched nothing and read as "no commit touched them".
- **2026-09-14 LESSONS, EACH COST A STOP.** (a) NO STORE WRITE DURING A LANDING WINDOW WITHOUT vc's WORD, AND NEVER A RETRY LOOP ON database is locked: cc's wb loops held the store against ic's 0307 closure and dc's 0393 chain; one attempt, report, wait; a stop lifts on its named word, not on a condition that looks met. (b) A worktree store is disposable and stale: unscoped sync --to-store from the worktree build first, then st attach, restoring only whiteboard inbox renders and stopping on anything else. The claimed-stamp heading difference is expected (vc): 0382 renders through a worktree build while the committed files came from the delivered 3.0.3 pair, and the re-render is one mechanical commit after the next build all. (c) no_pm_state_in_output refuses internal ids (ST0057, and keep D57-9 out too) in any shipped string literal, remedies included; doc comments are not scanned. (d) A red arm must COMPILE on its base: assert on Display text or an existing variant. (e) A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). (f) A landing symbol check greps a short token rustfmt cannot split. (g) Scripts: no apostrophe inside ${1:?word}; zsh runs a bare ==== as a command.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
