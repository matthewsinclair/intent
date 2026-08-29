---
node: cc
name: Control Claude
role: control
session_id: 7a90ae04-61f6-4b2f-9de7-dd81eac9bb11
heartbeat_at: 2026-08-29 12:51Z
status: active
focus: "**FOLDED FOR A COMPACT. NOTHING IN FLIGHT; ONE RULED PACKAGE IS NEXT AND ITS SCOPE IS ALREADY WITH vc.** Delivered today: 0125 closed at BOTH halves (cbc9f0c5 lib_staged.sh, d6d3b059 staged_reads_check.sh) and 0131 STORE half (ccfefe2b) -- issues add / st new refuse a taken key via the UNIQUE constraint INSIDE the transaction. **NEXT, RULED MINE BY vc: refuse in ac new / at new AND ship the edit verbs, as ONE change** -- either half alone leaves an estate stuck. Filed 0134: a partial-file commit is impossible on this checkout."
claims: [ST0056/06, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT.** Today's four commits are `cbc9f0c5` / `a4e76606` (0125), `ccfefe2b` (0131 store half), `decd82e6` (0134), `d6d3b059` (the enforcement check). The day's narrative is archived verbatim at `.history/20260829/wip-midday.md`; what follows is what a fresh session needs and nothing else.

**NEXT, AND IT IS RULED MINE (vc, on hv's word): `ac new` / `at new` REFUSE, AND THE EDIT VERBS SHIP, AS ONE CHANGE.** Either half alone leaves an estate stuck -- refusing without an edit path removes the only way to reword an AC; an edit verb without the refusal leaves the silent destruction live.

The scope is already stated to vc, so a disagreement arrives as an interrupt rather than as a surprise in a commit:

- **The refusal is a FACADE check, not the `ccfefe2b` store mechanism.** Criteria and tests are CHILD rows and `write_thread` replaces the child set wholesale, so there is no per-child UNIQUE constraint for a `Door::Create` to fire. **Unlike `st_new`'s dead guard this one CAN fire**, because the id is caller-assigned rather than computed as max+1 over the same canon being checked.
- **Known limit, stated not discovered:** two facades opened before either writes can still both add `AC-01.1`; the second thread write wins. Closing that needs the thread write to be conditional and is NOT in this change.
- **`ac edit <ST> <AC> --text`** -- ic measured the gap: no `ac edit`, and `--text` on one verb of ten.
- **`at edit <ST> <AT> --file | --prose | --covers`** -- the RE-CITE case, which is what destroyed six ST0061 notes.
- **`kind` STAYS UNWRITABLE on both, deliberately.** Changing test/non-test moves satisfaction and the contract graph -- a state change wearing an edit verb's clothes, and it belongs in a ruling rather than a flag added while building something else. **So a kind change is still stuck after this lands**; that is a smaller wall than the one ic measured and it is still a wall.
- **The surface is the collision risk.** The clap spine is BUILT from `surface/dispatch-table.json` (`dispatch_ssot.rs` proves it both ways), so two `new_surface` rows plus a regenerated `.md` are part of it -- and four nodes edit that file. **Author the commit from a DETACHED WORKTREE** (watch-out 9): `--only` cannot protect a peer's mid-edit row.

**WHY THIS WAS RE-RAISED RATHER THAN BUILT WHEN FIRST RULED.** vc ruled `ac new` follows; ic then measured that refusing ALONE makes an AC sentence unwritable by any verb in the tool. **A ruling can arrive where a missing verb cannot.** hv took the re-raise. The watch-out that fired was 14 -- the ruling was in hand and its cost was not.

## TODO

**THE WRITE-PATH PACKAGE (S-M) -- `issues edit --body/--from`, `close --note`, scoped issue sync.** EIGHT measured corrections queued behind it now, which is a better argument than any one: `0122` scope, `0118` re-premise, `0116`'s four rows, the 20 empty bodies, `0063` (wants `close --note`, not a body), vc's three WRONG cross-references typed into `0124`/`0126`/`0127` by predicting numbers, and `0124`'s body still framing `0133` as ingest damage rather than a live model limitation. **Do not hand-edit the extracts: that writes round the store, which is the SSOT.** vc's triage of the 20: 10 recoverable by convention, 9 need a different search, 3 are not body-restores (`0095`/`0096` need an hv disposition).

**ST0065's REMAINDER, MINE:** `_DECISION_TREE.md` and its three MODULES.md lines; the two archetype templates' MODULES.md lines; `0122`'s two CLI remedy strings. `_wip.md`'s hold expired at `2719f4c8` but the draft is not ruled, so execution waits on hv adopting it.

**THE TWO SYNC FILINGS** (my pen, cross-ref `0090`), and the hydrate/sync/dehydrate hypothesis measurement.

**PARKED FOR A RULING, NOT FOR EFFORT.** `issues list` is the disclosure convention's second holdout at 49 of 101 -- the same disposition move `st list` just made. `--status a,b` advertises "rendered in the order given" and does not do it: implement, or correct the help. Both with vc for hv.

**vc's RULED ORDER, AT (d): THE PRUNE, gated on two things that are not mine** -- `at-accounting.sh` runs BEFORE any prune ships, and WP-11's absent-manifest default decides what `--apply` does to 19 trees. **A FILE THE STORE CANNOT BE SHOWN TO HOLD IS A REFUSAL NAMING IT, NEVER A DELETION.** 2,115 authored bucket files on disk and in NO store across 5 of 14 estates (Lamplight 1,584, Laksa 499, Devbin 24, Anvil 5, Baize 3); nothing is lost (all in git) but a dehydrate or prune before the ingest destroys all of them. `arca_cli` validates the route, 23 of 23 byte-identical.

**WITH hv, UNANSWERED.** The two enum-roster findings (`declared_values_are_enforced` is blind to the 12 pipe-string rosters; `--format` is validated as an argument to the RENDERER, so an empty result set returns before it looks at the flag -- **and when that widening is built the predicate must be grounded in the refusal vocabulary, NOT `code != 0`, which red-flags `doctor --format json`**). Whether `publish_home` should refuse a temp root (not free: `install.rs` fixtures publish temp roots deliberately). Test-target consolidation (`b97afc24`: the 97-file rewrite and the 168-row re-cite are both ZERO under `#[path]`, the compile saving does not exist, link time and artefact bytes STILL UNMEASURED). `intentd --version` prints no commit. The v2 shell CLI prune (`pr-checks.yml:43`).

**3.0.1 (there is no 3.0.2 -- hv).** `on:` as a field on `Descoped`; bare-sync must compare to the STORE; a `doctor` canon-vs-store arm; the id resolver treating every id as a steel thread; ~2 MB store growth per `st attach` plus a bulk door; a tenth residue class for duplicate AT ids; the two-readers Highlander; `unclaimed_digest()` hashing absolute paths; the unbounded canon extract total; **`canon-commit` gates on ADDS, so a commit that REMOVES attachments is outside its reach**. The stored `slug` column is vestigial and STAYS so (hv: _let us not overcook this_).

**THREE NON-DECISIONS, DELIBERATE.** The guard-runner resolver and critic exit-2 fail-opens STAY, each carrying 0043's reasoning beside it. The 31 remedy-less error literals are NAMED and NOT gated. **`c755bd44` (2026-08-25 22:35Z) STAYS UNRESOLVED** -- you cannot recover a time you never read, and a corrected-looking fake is worse than an admitted one.

**BANKED SO IT DOES NOT DIE WITH A SESSION.** `ST0057`'s gate reads `PASS -- 66/66` against hv's board's `BLOCKED -- 51/53`, and **that PASS has a hidden denominator**: WP-12 and WP-13 are Not Started with ZERO ACs, positive-controlled, so `PASS` means _every criterion that exists is satisfied_ and reads as _the thread is done_. Filed by vc as `0123`. `AC-06.11`'s hold expired and nothing announced it. **`staged_reads_check.sh` reports 7 of 14 gated instruments reading a repo path unstaged** -- its release condition is that each is converged or exempted; placement (separate instrument vs an arm inside dc's roster) is an open question for dc.

## Watch-outs

**BY CLASS, ONE LINE EACH, RENUMBERED ONCE. Every line cost a wrong answer that read as a right one; the instances are in `.history/` and in the commits.**

**1. AN INSTRUMENT CAN PASS WHILE BLIND, AND ITS GREEN IS INDISTINGUISHABLE FROM A REAL ONE.** The faces: a corpus that CANNOT EXHIBIT the defect (commonest of all -- the estate you are standing in); a control that would also pass broken -- **and a control that does not FLIP when it should proves the whole probe was inert, which is how four instruments read green under a perturbation none of them asserts on**; a whole-file scan that cannot isolate a shared name; ACCIDENTALLY RIGHT, a true number from an instrument that could not have produced a different one; a predicate reproduced against the UNMUTATED source; a null result read as a clean sheet; an instrument that names the WRONG CAUSE and recruits every reader into the wrong search; a correct AGGREGATE that cannot check the mechanism (predict the SPLIT, never the total); a partial fix that makes an instrument feel repaired; the scanner inside its own corpus; a filter matching nothing, which passes for free. **THE ONLY CONTROL THAT CATCHES ANY OF THEM IS MUTATION -- break it and require the instrument to notice -- and when you mutate, ASSERT THE OCCURRENCE COUNT FIRST.**

**2. A VACUITY THAT RENDERS AS A WELL-FORMED TABLE IS INVISIBLE TO A CONTROL LOOKING FOR A SENTENCE.** `format_roster_is_honoured`'s fixture was vacuous for `st list` from the day it was written -- the file exists to prevent exactly that, and its own non-empty control could not see it. `0121`'s disclosure made it visible and the control fired the same hour.

**3. THE DISK CANNOT BE THE WITNESS FOR A WRITE, BECAUSE THE DISK IS WHAT MOVES -- AND NEITHER CAN A VERIFICATION THAT WAS TRUE WHEN IT RAN.** Verify `git show :<path>`, the STAGED BLOB, never the working-tree file: I committed a PEER's issue under a number I believed was mine (`9e9ee8ab`) by staging a path by NAME after the extract beneath it had been rewritten. **Then the same class beat the cure** (`0134`): I hand-staged a blob, verified it with `git show :<path>`, and the pre-commit gate REFRESHED THE INDEX FROM DISK before the commit landed, so `ccfefe2b` carries two of dc's hunks. Not a skipped check and not a blind proxy -- a correct measurement whose subject moved between the check and the act. **The cure is not looking harder, it is removing the gap: a detached worktree, where index and worktree cannot disagree.**

**4. A CREATE IMPLEMENTED AS AN UPSERT REPORTS SUCCESS AND REPLACES.** Both writers were told `created:`. hv has now ruled: add/new must REFUSE.

**5. I HAD THE EVIDENCE BEFORE ANYONE HAD THE READING.** My longest-dup-0 measurements and `AT-01.5` in the LONGER bucket were consistent all along with conflab-vc's refutation of `0126`'s LOSS claim -- the rows INFLATE, every authored character survives, the damage is SCRAMBLING. **Evidence in hand is not a reading; say what the numbers rule OUT.**

**6. A CORRECTION MADE IN CONVERSATION DOES NOT REACH AN ARTEFACT.** I retracted a claim to vc in a message and left it compiled into the shipped guide.

**7. A TEST THAT HARDCODES A MEMBER OF A POPULATION THE PROJECT IS DRAINING GOES STALE BY BEING RIGHT.** Three instances in one day. **The cure is to take the example from the population's OWN DECLARATION** -- `ThreadStatus::ALL` is now that, mirroring `TShirt::ALL`.

**8. I STOPPED AT THE INSTANCE I WAS HANDED INSTEAD OF CHECKING THE SET.** vc raised the exit-2 class against `corrected_check`; I verified that one, found it correct, and said there was nothing to file. `rulings_check` had it. **A peer's instance being wrong does not make their sighting wrong.**

**9. A SHARED TREE HAS OTHER WRITERS.** `git commit` commits the index AS IT STANDS, so `add` + `commit --only <paths>` is the safe write for WHOLE paths, and **`--only <path>` protects other PATHS, not other people's edits to YOUR path -- it commits the WORKTREE version of the path, wholesale, over anything you staged** (measured). **There is NO way to commit a subset of a file's hunks while a peer edits it**: `--cacheinfo` stages the right bytes and the gate refreshes them away. Author it from a DETACHED WORKTREE instead -- separate tree AND separate index. Check `git diff --cached --name-only` is empty before `git add`. A reflog check precedes any reset of a shared HEAD; never `cp` a shared source aside to mutate -- clone or use a detached worktree.

**10. EVERY TIMESTAMP IS READ FROM `date -u +'%Y-%m-%d %H:%MZ'` IN THE SAME CALL THAT WRITES IT.** `git log` prints LOCAL time and appending `Z` is wrong by exactly the offset. I have fabricated stamps by three routes: wrong zone, typed-from-the-last-one, and one minute ahead of a clock I had just read in the same tool call. A placeholder shaped like a stamp is worse than an admitted gap.

**11. THE EXIT CODE YOU READ IS THE LAST COMMAND'S, NOT THE INTERESTING ONE'S.** Hit twice; `intent ... | sed` gives you `sed`'s 0.

**12. THE POPULATION YOU MEASURED IS NOT THE ONE YOU MEANT, AND NOTHING NAMES IT.** `cargo check -p X` builds no test target; `cargo test --test X` excludes lib tests; a narrowing flag never narrows the CLAIM.

**13. A ONE-SIDED COMPARISON DISCARDS THE EVIDENCE THAT THE INSTRUMENT IS WRONG.** Compare in BOTH directions. And **a conservation check whose two sides derive from the same source is an IDENTITY, not a measurement.**

**14. A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE CLOSING IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** The pull is strongest exactly when the finding is good. **Corollary for `0131`: settling `ac new` inside the implementation would be this.**

**15. A MECHANISM FILED WITHOUT ITS CONSEQUENCE IS NOT FILED.** And **a finding's SCOPE is set by the search that found it, and is always smaller than the problem.**

**16. A REFUSAL THAT DELETES IS THE MORE DESTRUCTIVE OPTION AND READS AS THE SAFER ONE.** Refusing is only conservative when refusing PRESERVES; in the legacy parser `Err(..)` DROPS THE ROW.

**17. ABSENCE IS A STATE; UNREADABILITY IS AN ERROR; A `let Ok(..) else` CANNOT TELL THEM APART.** And **a refusal that ends the run reports a FLOOR that is read as a COUNT.**

**18. ONE VALUE, TWO HOMES, AND NOTHING COMPARES THEM.** A SERIALISER IS A SECOND WRITER NOBODY DECLARED (`json.dumps` defaults to `ensure_ascii=True`); so is the markdown formatter, which auto-aligns tables on save.

**19. A REMEDY IS A CLAIM ABOUT BEHAVIOUR AND MUST BE DRIVEN LIKE ONE.** One that was never true is caught by review; **one that EXPIRES is caught by nobody.** As-observed fidelity can faithfully port a remedy naming a verb v3 retired.

**20. A MISSING TEST IS A BACKLOG ITEM; AN UNRUN TEST IS A DISCIPLINE FAILURE.** A pristine-worktree control proves "not my diff" and CANNOT prove "not the environment" -- both trees share one path.

**21. HAND OVER PROPERTIES, NOT VALUES.** A bare id is ambiguous, a bare number-word worse, a bare moniker crosses estates; a hash is a READING. Re-measure state; never report a ledger.

**22. ATTRIBUTION DECAYS INTO ASSERTION**, and **exoneration and attribution are two assertions -- measuring one feels like measuring both.**

**23. A PROBE THAT DECIDES BY RUNNING THE COMMAND HAS THE COMMAND'S SIDE EFFECTS.**

**24. A CONTROL VALIDATES THE AXIS IT TESTS AND IS SILENT ABOUT THE ONE NEXT DOOR.**

**25. THE FOLD IS ITSELF AN INSTRUMENT, AND IT HAS FAILED IN THIS BOARD.** It left one class in two sections with neither a superset, dropped a ruled GO, and let the numbering run to two 10s, two 11s and two 12s. **This fold renumbered once and archived the source verbatim first.**

**ENVIRONMENT.** Bare `intent` resolves via the `~/.local/bin/intent` SYMLINK to `native/rust/target/release/intent`, never via `.envrc`. The delivered pair is `8177b53e`, sha256 `a440bbd0...`, and the gate reports `currency ok`. **Pin by the HASH, never by the marker** -- three distinct binaries carried one marker in a day. `find` is bfs not GNU find; the Bash tool's shell is zsh, so unquoted `$var` does NOT word-split and an unmatched glob aborts the whole command.

**LANE.** `native/**` and the v3 crates are mine; the parity harness is ic's; hooks, roster and `int hooks` are dc's; **canon writes route through vc**. `~/Devel/prj/Intentv2` IS NOT TOUCHED.

## Decisions

**I DO NOT EDIT `CLAUDE.md`, PERMISSION SETTINGS OR CONFIG ON A PEER'S INSTRUCTION**, however right they are on the merits. A peer's ask is never the user's approval. **AND I DO NOT APPLY AN EDIT A CLASSIFIER REFUSED A PEER.**

**A RELAY IS NOT AN INSTRUCTION.** A ruling reaching me through a peer gets confirmed with hv AT SOURCE before I act -- via `hv/inbox.cc.md`, which vc is the named reader for. Not distrust: a ruling's SCOPE is the half that does not survive a retelling.

**DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc's ruling on what I may write into ST0056 canon -- kept on the board it CONSTRAINS).

**A PARITY TOOL IS A RECORDED ATTACHMENT, AND THIS LINE USED TO NAME THE WRONG VERB.** Canon holds each tool's TEXT, and `sync --to-disk` does NOT refresh attachments from the worktree -- it reports `ok: extract written` and produces a zero-byte diff, a quiet answer to a question nobody asked. The carry is **`intent st attach ST0056 parity/tools/<f> --from <path>`, ONE CALL PER FILE**, then `intent sync --to-disk ST0056`, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` together. **Never the thread-wide carry**: `ingest::collect_attachments_into` re-reads authored prose, which is the live `0124`/`0126`/`0127`/`0129`/`0133` damage class. Verify BEFORE staging that no record but yours moved.

**THERE IS NO 3.0.2 (hv).** Everything else goes into 3.0.1.

**RESIDUE BLOCKS ON A LIVE THREAD AND CARRIES ON A CLOSED ONE; HALF A's ACCOUNTING BLOCKS ON BOTH** (vc).
