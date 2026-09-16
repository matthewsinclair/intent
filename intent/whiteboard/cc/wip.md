---
node: cc
name: Control Claude
role: control
session_id: e3744ab1-9442-4c6e-81f7-fcfee1d1af21
heartbeat_at: 2026-09-16 15:05Z
status: active
focus: "Localfolded before hv's compact, 2026-09-16: bank 1 (0411, 0415, 0414) landed at 5b72cb12c. Next, on vc's hashes: bank 2 (0410, 0417, 0413), then 0402 (code only). See the IN FLIGHT todo. NO RELEASE, NO PUSH."
claims: []
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.
- **IN FLIGHT: RESUME HERE (cc, 2026-09-16 localfold before hv's compact; continue on the bounce).** BANK 1 (0411, 0415, 0414) LANDED at 5b72cb12c on vc's word; the three are closed. NEXT, each its own chain on the HEAD hash vc sends, with CHAIN START and CHAIN END to vc, ic and dc: (1) BANK 2 (0410, 0417, 0413), refs/bank/cc/wbstore-bank2 = 9e5d132d, judged green by vc; kit `bash <scratchpad>/land/land-bank2.sh <hash>`. (2) 0402, refs/bank/cc/0402 = aef571e5, judged green by vc; code only; kit `land-0402.sh <hash>`. Its canon rewrite happens once in the quiet window (vc's ruling): after `devbin build all`, intentd stopped, `intent sync --to-disk`, then `git cat-file -p refs/bank/cc/0402-canon-check` (blob 49919a4b) run with the base proves only the end newlines moved (4 objectives, 3 contexts, 10 WP objectives at df2235e07). The scratchpad is /private/tmp/claude-501/-Users-matts-Devel-prj-Intent/7899f335-6bda-4b03-8cb5-347e62e84999/scratchpad; land/kit holds the notes and messages. The kits settle on intentd's CPU before EVERY store write (vc). A refused write gets one re-issue after lsof, then stop and report. Worktrees wt-wbstore and wt-stack there can be removed after 0402 lands. The reference set (docs/reference) is regenerated whole in the quiet window, not in a bank (vc). ic's 0400 lands after cc's three. NO RELEASE, NO PUSH.

## Holds

- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.

## Watch-outs

- **TRAPS WITH NO OTHER HOME (cc, folded at the 2026-09-15 wrap; the rest live in restart.md, a shared memory, a test or a guard).** A red arm must COMPILE on its base: assert on Display text or an existing variant. A leftover-code grep uses -F: an unescaped .generated() matched fn generated(). A landing symbol check greps a short token rustfmt cannot split. In scripts, no apostrophe inside ${1:?word}, and zsh runs a bare ==== as a command. A worktree sync-then-attach restores only whiteboard inbox renders and stops on any other tracked move. The critic gate lints the rule library's own bad examples, so that pair cannot be committed at all. The home pointer is ~/.local/share/intent/home, and restart.md line 25 still names ~/.intent/home (sent to vc). `grep` on this machine is ugrep, and a long `.{0,N}` repetition fails with "exceeds complexity limits" instead of matching. `.gitignore` ignores `target/` only, so a worktree build must use the in-tree `native/rust/target`: a `target-cc` directory is not ignored and would ride into a `git add -A` bank patch. An unsynced bank stack reds attachment_drift_detected and thread_prose_carried whenever a bank carries attachment documents without their thread's canon (ic's 0339 and as-written banks did): judge those two on the landed train, where each landing synced canon, never on the stack. `testkit::repo_root()` is the WORKTREE's root, so an estate test reads the tree it runs in. `at edit --note` replaces a row's note outright, and `at green/red/na --note` never appends either: it refuses with NoteWouldBeLost unless the passed text CONTAINS the existing note, then sets the note to that text. To add a line, pass the old note plus the line. A new AT row's cited file must carry the row's literal id, and `at new` and `at green` never read a citation (issue 0267), so only `at lint` and the gate see a missing one: 0338 (i)'s landing took ST0057's gate from PASS to BLOCKED on exactly that until a header line named the row. A new row starts `to-write`, and `at green` is legal only from `red` (issue 0337); `at green --note` on a row already green is accepted and its text must contain the old note. A daemon cost run needs an estate the size of the real one, so neither a fresh `init` (too small to show a walk) nor a worktree commit (the gate refuses it under an isolated HOME): a hookless clone of the repo is the scratch estate. `git grep -E` has no `\b`: a word-boundary pattern matches nothing and exits like a clean miss, which read as `finish` having no callers until `git grep -F 'finish('` found every one. Search for a literal with `-F`. A script that runs `git -C <worktree> apply <patch>` resolves a relative patch path inside the worktree, not the caller's directory: rebase-0331b.sh reset wt-0331 and then could not open regen/0331b-counts.patch, so its apply ran by hand. Pass the patch path absolute.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
