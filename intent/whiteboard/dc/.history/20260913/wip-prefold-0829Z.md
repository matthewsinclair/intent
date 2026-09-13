---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 20:24Z
status: active
focus: "WP-14's migration verb (AC-14.9). The board READER is landed with its arms; the facade verb, the store writer and the AT row are next, and holds rebase onto cc's fifth kind. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/02, ST0069/14, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-2024Z.md`.** Everything landed is carried by its commits, not here.

## DOING -- WP-14's migration verb, AC-14.9

**The board READER is landed at `71028a2a3` with four arms; nothing calls it yet.** `wbmigrate::read_board` and `read_inbox` carry a hand-authored board into the model's shapes: header as line-oriented `key: value`, prose blocks AND bullet blocks both yielding items, every claimed stamp verbatim into `authored_at`, every hold refused by name with its file, line, text and reason.

**NEXT, in order.** The facade operation `wb_migrate(node)`; a store writer that can set `authored_at` -- the shipped `wb_insert_item` / `wb_insert_message` hardcode it NULL, correctly, because they are for live writes, so the migration needs that parameter added to the ONE insert per table rather than a second insert beside it; the `.history/` carry into `DocSection`; the CLI verb; then the AT row for AC-14.9 cited to the arms.

**HOLDS REBASE ONTO cc, NOT ONTO ME.** vc ruled `WbItemKind::Hold` is cc's to add, with a `wb add <kind> <text>` verb, ahead of the renderers. Until it lands my reader refuses every hold BY NAME; when it lands I re-point that arm and carry holds as holds, condition and all.

## TODO

- **The protocol half is NOT mine and is NOT to be started** (vc, 2026-09-12 20:23Z, superseding the earlier "if ic is still inside 0311"): it stays with ic after the row reviews unless ic hands it to me on the socket, and I do not begin it before the migration verb lands. If it ever does come to me: `/in-whiteboard` rewritten to the `intent wb` verbs (AC-14.10), and the four `cmd_ws_*` functions with every caller -- **the fifth is at `intent_claude_cwi:392`** -- deleted with an AT that drives the deletion (AC-14.12). Skill files: no em dashes, `eg` never `e.g.`, and the header block is not YAML.
- **Later, on vc's signal only**: one preflight line running ic's `contract_check.sh` (`intent/st/ST0056/parity/tools/contract_check.sh`; 0 clean, 1 findings, 2 environment/usage). **ROSTERED MANUAL, not gated** -- it exits 1 today on whiteboard faces cc has not built, which vc ruled stands. Positive-control it with its `MODEL` override before trusting a green, and keep exit 1 and exit 2 distinct in whatever the release script prints.
- **CHANGELOG**: ic writes the Added lines for the search packages; my Fixed lines stay mine.

## Holds

- **vc is DARK for hv's compact from 20:1xZ.** Condition: vc says they are back. Reports go to `intent/whiteboard/vc/inbox.dc.md` with a same-turn `date -u` stamp. Standing orders are in `inbox.vc.md` and nothing in them is new.
- **`WbItemKind::Hold` does not exist.** Condition: cc lands the fifth kind and its `wb add` verb. Until then my reader refuses holds by name and the migration is incomplete BY DESIGN rather than by omission.
- **Live boards stay hand-authored under both guards.** Condition: vc signals the cutover. `wb register` wrote configuration only; no board's CONTENT is in the store.
- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **AN ARM CAUGHT A DOOR THAT COMPILED, REPORTED NOTHING AND DID NOTHING.** `Plan::is_destructive` asks whether any STEP is destructive, and a v2 leftover is not a step -- so a plan whose only removals were leftovers took no removal branch at all. Both doors wired, empty report, untouched tree, nothing failing. **Review cannot see a branch that is never entered; the arm saw it on its first run.**
- **A `None` FROM A THREAD-SHAPED READER, INSIDE AN `&&` OR AN `is_some_and`, IS THE ANSWER "YES, DECLARED" -- TWICE, IN TWO SIBLING READERS.** The projection at 49a00fb80 and doctor at `f70441dc2`. Each was found by the ESTATE, never by a fixture, because every arm that reaches an issue goes through `issues add`, which declares it. **Where a question has two artefact kinds, write ONE predicate that answers for both, and have it return the OWNER rather than a boolean** -- a caller that re-derives the id has re-derived the rule with it.
- **MY FIXTURE'S POPULATION WAS NOT THE ESTATE'S, AND THE SAME TRAP IS LIVE IN WP-14.** Every real board writes DOING as prose and TODO as bullets; a bullets-only reader carries nothing from the busiest section of every board and reconciles perfectly against zero. **Build the fixture from the shape the estate actually has, not the shape the format documents.**
- **THE RETRY LOOP REPORTED A DETERMINISTIC GATE REFUSAL AS LOCK CONTENTION -- TWICE IN ONE DAY, ON TWO GATES.** Fifteen refusals were `IN-RS-CODE-004`, and the loop's "last failure, verbatim" printed EMPTY because the gate writes to a stream the capture did not hold. **A loop that cannot print the refusal it is retrying is not an instrument. Read a gate ONCE in the foreground.** A "last failure" that comes back empty is the loop saying it never saw one.
- **A CRITIC GREPS TEXT, SO PROSE NAMING A BANNED SHAPE TRIPS IT.** The last `IN-RS-CODE-004` warning was on my DOC COMMENT explaining why the banned shape was not used. Reword; do not argue with the instrument.
- **A DIAGNOSTIC THAT MUTATES IS NOT A DIAGNOSTIC.** I ran `git commit` three times to READ a gate's refusal; the first SUCCEEDED and landed `1f8c9fc08` under a message claiming three criteria.
- **SETTLE A DAEMON-FAMILY RED AGAINST THE BASE, NOT BY RE-RUNNING.** Structural dismissal is only available when the change CANNOT reach the arm -- verify "no caller" by grep before claiming it. When the change does touch the ingest path, run the UNMODIFIED base: the family is red there too, on a different arm each run, which is the load signature.
- **PRETTIER ON AN 852KB `MODULES.md` CHANGED EXACTLY ONE LINE**, measured on a copy before touching the real file. Probe the blast radius of a formatter on a huge shared file rather than assuming it is either safe or catastrophic.
- **A COMMIT MESSAGE IS A CLAIM ABOUT ITS OWN DIFF.** Read `git show --stat` against it every time.
- **REGISTER BEFORE YOU CODE**: `intent/llm/MODULES.md` gets the row first, and the gate wants prettier on it.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call**, literal paths, re-issue the SAME command on a lock refusal. Peers stage files mid-commit; `--only` is what keeps their work out of my landing.
- **Every suite and build from the private worktree with its IN-TREE target dir** under an isolated HOME, `CARGO_HOME` at the real one. **`--no-fail-fast` is a CARGO flag, before the `--`.** **Every drive script traps EXIT and stops the daemon it started** (vc's standing order); mine does, and it reaped my own orphan on its first run.
- **D42: a clock value goes into a board or a message only from a `date -u` read in the SAME CALL as the write.** I typed one by feel today and corrected it within the minute.
- **The Bash tool's shell is zsh**: unquoted `$var` does not word-split, an unquoted `--include=*.rs` aborts the whole command, and messages go in a file through `-F`.

## Decisions

- (2026-09-12) **`WbItemKind::Hold` is cc's, ruled by vc.** The fifth kind plus a `wb add <kind> <text>` verb, landing ahead of the renderers; my migration rebases onto it. The alternatives were refused: carrying holds as `Todo` silently reclassifies work that is deliberately NOT queued, and leaving markdown authoritative for holds alone is a board half in the store and half on disk.
- (2026-09-12) **WP-02's prune refusal is ALL OR NOTHING with the estate as the unit.** One unheld path refuses every removal and names each one, so one over-cap or unreadable file freezes an estate's prune until a human moves it. The conservative direction, and the refusal names the file and the remedy.
- devbin `0047` (hv, 2026-09-01): option 3, the split. Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.
