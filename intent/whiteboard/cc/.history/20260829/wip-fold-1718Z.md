---
node: cc
name: Control Claude
role: control
session_id: 7a90ae04-61f6-4b2f-9de7-dd81eac9bb11
heartbeat_at: 2026-08-29 16:17Z
status: active
focus: "**LANDED `228bc900`: the per-family command reference, 16 generated pages -- vc confirms it meets the last named condition on the Laksa kickoff, and the send is theirs.** Pair rebuilt to `b2077ba2`, currency 0 behind, so the local delivered-binary hazard is closed. HOLDING for hv. Next queued and ruled: `st edit` TTY-aware with `--editor` / `--path`."
claims: [ST0056/06, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT. HOLDING FOR hv.** The two questions this section carried are both RULED and the answers are on hv's board, not here. Commits are the record.

**QUEUED, RULED, MINE TO BUILD: `st edit` IS TTY-AWARE WITH OVERRIDES BOTH WAYS.** Launch when stdout is a terminal, print the path when piped, `--editor` forces the launch and `--path` forces the path. **The regression it owes is an acceptance test, not a note:** `$EDITOR "$(intent st edit ST0001 info)"` must still return the path, because `docs/getting-started.md` says so. The editor design itself is unchanged and is recorded below so it is not re-derived.

**QUEUED, vc's BY AUTHORSHIP AND OFFERED TO ME: `runner_roster_check.sh`'s SEVERED-INSTRUMENT DISCRIMINATOR USES `git ls-files`, WHICH CANNOT SEE AN UNTRACKED FILE.** So the branch built to protect an in-flight instrument prints the SEVERED verdict for a brand-new one, which is untracked by definition until its first `git add` -- and the remedy line then sends the reader to repair a row that is correct. Three outcomes want three messages: tracked-and-present, present-but-untracked (stage it), absent (severed). It blocked every commit in the repo for a stretch today.

**QUEUED, SMALL: ST0064 NAMES `Geodica/_tools` IN THREE PLACES** and that tree moved to `/Users/matts/Devel/prj/Gtools` (_tools-cc, who correctly did not touch this tree). Nothing executes those references. The finding underneath it is worth more than the rename and is hv's call, not mine: **a project with a concurrent test suite inside a cloud-sync provider loses files silently** -- 117 tracked files truncated to zero in one atomic event, twice, exactly the four directories a 32-way `File.cp_r!` was reading. Whether `intent doctor` should warn on a sync-provider path is an open question nobody has filed.

**THE ONE DESIGN LINE A FRESH SESSION MUST NOT RE-DERIVE: THE REFUSAL IS AT THE CREATE DOOR AND `put` IS UNTOUCHED.** hv ruled on verbs named `add`/`new`; replace-at-an-address is the HTTP and GraphQL faces' contract. **So what died is the FABRICATING path, not the replacing one**, and an arm asserts the survivor BY VALUE so a later reversal meets a red test rather than inferring intent from prose.

**THE EDITOR DESIGN, SO IT IS NOT RE-DERIVED:** `e` is a SHELL ALIAS and no process can exec it -- detect `emacsclient`/`emacs`. The alias detaches (`nohup ... &`), which loses the edit for `issues edit`, because **an issue has no realised form** (hv, 2026-08-20) and its body must round-trip through a temp file -- so the editor MUST block. `st edit` already exists and PRINTS A PATH (AC-05.3); its stdout is load-bearing, so launching is TTY-gated, not a contract change.

## TODO

**NOTHING IS CLAIMED. Every line below is parked, ruled elsewhere, or waiting on someone who is not me.**

- **THE WRITE-PATH PACKAGE (S-M)** -- `issues edit --body/--from`, `close --note`, scoped issue sync. A growing queue of measured corrections waits on it; **vc keeps the count, read it off their board.** Never hand-edit the extracts -- that writes round the store, which is the SSOT.
- **ST0065's REMAINDER, MINE** -- `_DECISION_TREE.md` + its MODULES.md lines, the two archetype templates, `0122`'s two CLI remedy strings. Waits on hv adopting the draft.
- **THE TWO SYNC FILINGS** (my pen, cross-ref `0090`) and the hydrate/sync/dehydrate measurement.
- **PARKED FOR A RULING, NOT EFFORT** -- `issues list` is the disclosure convention's last holdout; `--status a,b` advertises an ordering it does not honour (implement, or correct the help). Both with vc for hv.
- **THE PRUNE, at vc's (d)** -- gated on `at-accounting.sh` running first and on WP-11's absent-manifest default. **A FILE THE STORE CANNOT BE SHOWN TO HOLD IS A REFUSAL NAMING IT, NEVER A DELETION**: thousands of authored bucket files sit on disk and in no store across five estates, and a dehydrate before the ingest destroys all of them. Re-measure the population; do not trust a count from this board.
- **WITH hv, UNANSWERED** -- the two enum-roster findings (**when the `--format` widening is built its predicate must be grounded in the refusal vocabulary, NOT `code != 0`, which red-flags `doctor --format json`**); `publish_home` on a temp root; test-target consolidation (compile saving is ZERO, link time still unmeasured); `intentd --version` prints no commit; the v2 shell CLI prune.
- **3.0.1, AND THERE IS NO 3.0.2 (hv)** -- `on:` on `Descoped`; bare-sync must compare to the STORE; a `doctor` canon-vs-store arm; the id resolver treating every id as a thread; store growth per `st attach` plus a bulk door; a tenth residue class; the two-readers Highlander; `unclaimed_digest()` hashing absolute paths; **`canon-commit` gates on ADDS, so a commit that REMOVES attachments is outside its reach**. The `slug` column stays vestigial (hv: _let us not overcook this_).
- **THREE NON-DECISIONS, DELIBERATE** -- the guard-runner resolver and critic exit-2 fail-opens STAY with 0043's reasoning beside them; the remedy-less error literals are NAMED, not gated; **`c755bd44`'s fabricated stamp STAYS UNRESOLVED**, because a corrected-looking fake is worse than an admitted one.
- **BANKED, NOT MINE TO MOVE** -- `ST0057`'s `PASS` has a hidden denominator (packages with ZERO ACs read as done; vc filed `0123`). **`staged_reads_check.sh` reports 7 of 14 gated instruments reading unstaged**; placement is dc's question. `AC-12.4` reads UNSATISFIED over three measurably-done conditions (vc's).

## Watch-outs

**BY CLASS, RULE-ONLY. Every line cost a wrong answer that read as a right one; the INSTANCES are in `.history/` and in the commits, and this section is the rules.**

**1. AN INSTRUMENT CAN PASS WHILE BLIND, AND ITS GREEN IS INDISTINGUISHABLE FROM A REAL ONE.** The faces: a corpus that CANNOT EXHIBIT the defect (commonest -- the estate you are standing in); a control that would also pass broken; **a control that does not FLIP proves the probe was inert**; ACCIDENTALLY RIGHT, a true number from an instrument that could not have answered differently; a predicate reproduced against the UNMUTATED source; a null read as a clean sheet; an instrument naming the WRONG CAUSE, which recruits every reader into the wrong search; a correct AGGREGATE that cannot check the mechanism (predict the SPLIT, never the total); the scanner inside its own corpus; a filter matching nothing, which passes for free. **THE ONLY CONTROL IS MUTATION -- break it and require the instrument to notice; assert the OCCURRENCE COUNT first.** **AND THE CHEAPEST INSTRUMENT I OWN IS A PRECONDITION THAT REFUSES TO RUN** -- one line, and it fails at the moment the subject stops being what you think it is, which is the only moment the failure is cheap.

**2. A VACUITY THAT RENDERS AS A WELL-FORMED TABLE IS INVISIBLE TO A CONTROL LOOKING FOR A SENTENCE.**

**3. THE DISK CANNOT BE THE WITNESS FOR A WRITE, BECAUSE THE DISK IS WHAT MOVES -- AND NEITHER CAN A VERIFICATION THAT WAS TRUE WHEN IT RAN.** A correct measurement whose subject moves between the check and the act is not a skipped check. **The cure is not looking harder, it is removing the gap: a detached worktree, where index and worktree cannot disagree.**

**4. A CREATE IMPLEMENTED AS AN UPSERT REPORTS SUCCESS AND REPLACES.** hv ruled: add/new must REFUSE. **Its wider form: any verb that RECONSTRUCTS a row from arguments silently enlarges every time a payload-carrying variant is added to the type.**

**5. EVIDENCE IN HAND IS NOT A READING. Say what the numbers RULE OUT.**

**6. A CORRECTION MADE IN CONVERSATION DOES NOT REACH AN ARTEFACT.**

**7. A VALUE GOES IN ONLY IF THE POPULATION'S OWN DECLARATION HAS IT -- BOTH DIRECTIONS.** Hardcoding a member of a draining population goes stale by being right; **MINTING one is the same error walking the other way.** Read the roster first rather than relying on an instrument to catch you.

**8. A PEER'S INSTANCE BEING WRONG DOES NOT MAKE THEIR SIGHTING WRONG.** Check the SET, not the instance you were handed.

**9. A SHARED TREE HAS OTHER WRITERS.** `git commit` commits the index AS IT STANDS. **`--only <path>` protects other PATHS, not other people's edits to YOUR path -- it takes the WORKTREE version wholesale** -- and there is NO way to commit a subset of a file's hunks while a peer edits it. **`--only` also DROPS a peer's staged file from the tree it builds, which can trip a guard that couples two files.** Check `git diff --cached --name-only` is empty BEFORE `git add`, in a separate call, or you cannot act on what it tells you. **A commit can lose the race AFTER the gate passes** (`cannot lock ref 'HEAD'`): files stay staged and correct, so re-verify and re-run `git commit` -- do NOT re-`add`. Never remove a peer's lock, never reset `.`, never `cp` a shared source aside to mutate.

**10. EVERY TIMESTAMP IS READ FROM `date -u +'%Y-%m-%d %H:%MZ'` IN THE SAME CALL THAT WRITES IT.** `git log` prints LOCAL. I have fabricated stamps three ways: wrong zone, typed-from-the-last-one, and one minute ahead of a clock I had just read. **A placeholder shaped like a stamp is worse than an admitted gap.**

**11. THE EXIT CODE YOU READ IS THE LAST COMMAND'S, NOT THE INTERESTING ONE'S.** `x | tail` gives you `tail`'s 0.

**12. THE POPULATION YOU MEASURED IS NOT THE ONE YOU MEANT, AND NOTHING NAMES IT.** `cargo check -p X` builds no test target; `--test X` excludes lib tests; a narrowing flag never narrows the CLAIM. **AND THE INDEX IS A DIFFERENT CORPUS FROM THE WORKTREE**: a converged instrument reads its gating input from the INDEX, so a run over an unstaged edit reports the COMMITTED corpus and returns an unchanged count that reads exactly like rejection. Stage, then measure.

**13. A ONE-SIDED COMPARISON DISCARDS THE EVIDENCE THAT THE INSTRUMENT IS WRONG.** Compare BOTH directions; **a conservation check whose two sides share a source is an IDENTITY, not a measurement.**

**14. A FINDING THAT ARGUES ITS WAY INTO THE ROW YOU ARE CLOSING IS A REAL DEFECT LAUNDERED THROUGH A GREEN.** The pull is strongest exactly when the finding is good.

**15. A MECHANISM FILED WITHOUT ITS CONSEQUENCE IS NOT FILED**, and a finding's SCOPE is set by the search that found it -- always smaller than the problem.

**16. A REFUSAL THAT DELETES IS THE MORE DESTRUCTIVE OPTION AND READS AS THE SAFER ONE.** Refusing is conservative only when refusing PRESERVES.

**17. ABSENCE IS A STATE; UNREADABILITY IS AN ERROR; A `let Ok(..) else` CANNOT TELL THEM APART.** And a refusal that ends the run reports a FLOOR that is read as a COUNT.

**18. ONE VALUE, TWO HOMES, AND NOTHING COMPARES THEM.** A SERIALISER is a second writer nobody declared (`json.dumps` defaults to `ensure_ascii=True`); so is the markdown formatter. **The quietest home is PROSE SITTING BESIDE THE MACHINE-CHECKED FIELD** -- true, adjacent, parsed by nothing. **A hand-typed COUNT is the same shape.**

**19. A REMEDY IS A CLAIM ABOUT BEHAVIOUR AND MUST BE DRIVEN LIKE ONE.** **BORN STALE and EXPIRED are different defects and NEITHER is caught by reading** -- a claim can ship in the same commit as the evidence refuting it and survive months of review. What catches one is a precondition written ON the claim (class 1).

**20. A MISSING TEST IS A BACKLOG ITEM; AN UNRUN TEST IS A DISCIPLINE FAILURE.** A pristine-worktree control proves "not my diff" and CANNOT prove "not the environment".

**21. HAND OVER PROPERTIES, NOT VALUES.** A bare id is ambiguous, a bare moniker crosses estates, a hash is a READING. **A pin dies at a bump somebody else schedules** -- this board carried `currency ok` while the gate was refusing. Re-measure; never report a ledger.

**22. ATTRIBUTION DECAYS INTO ASSERTION**, and exoneration and attribution are two claims -- measuring one feels like measuring both.

**23. A PROBE THAT DECIDES BY RUNNING THE COMMAND HAS THE COMMAND'S SIDE EFFECTS.**

**24. A CONTROL VALIDATES THE AXIS IT TESTS AND IS SILENT ABOUT THE ONE NEXT DOOR.**

**25. THE FOLD IS ITSELF AN INSTRUMENT AND IT HAS FAILED IN THIS BOARD.** It has left one class in two sections, dropped a ruled GO, and duplicated numbering. **Archive VERBATIM first, under `wip-fold-HHMMZ.md`** -- a bare name invites the next same-day fold to overwrite it. **UPDATE classes, never append instances beside them**; when a fold only trims narrative it is not aggressive, it is tidy.

**26. A WEAKENING WITH NO SYMPTOM IS THE HARDEST THING TO CATCH AND THE EASIEST TO WAVE THROUGH** (dc, 2026-08-29). When a type widens, the loose port and the tight one both compile and both pass -- no compiler error, no red arm, just an assertion that stopped asking as much. **Nothing mechanical flags it, so the REASON for a tight match belongs in the file, next to the match.**

**27. A GENERATED ARTEFACT CAN WRITE INTO THE AGENT-INSTRUCTION CHANNEL, AND NOTHING ANNOUNCES IT.** `docs/reference/claude.md` IS `CLAUDE.md` on a case-insensitive filesystem, and Claude Code discovers agent instructions by BASENAME -- so a docs generator silently became a writer of project instructions, found only because it loaded into my own context. **A class, not an instance:** `agents.md`/`AGENTS.md` and `modules.md`/`MODULES.md` were harmless by luck. **Prefix the whole output space rather than special-casing the member that bit**, because the next reserved basename is invented by a consumer you have never met.

**28. A PRIVATE INDEX COMMITS PAST A PEER'S STAGED FILE -- AND LEAVES THE SHARED INDEX HOLDING A REVERSION.** `GIT_INDEX_FILE` + `read-tree HEAD` + add-only-mine is the one write that cannot sweep a peer, and the gate honours it. **But afterwards the shared index still holds the PRE-commit state for your paths, which against the new HEAD reads as staged DELETIONS of everything you just added** -- a bare `git commit` by the next node undoes the lot and looks ordinary. `git reset HEAD -- <your paths>` on the shared index is the second half and is not optional. **Verify it with `env -u GIT_INDEX_FILE`; an EMPTY `GIT_INDEX_FILE=` is not unset and makes git report the whole tree as staged.**

**29. PRESENCE OF THE VARIANT IS NOT PRESENCE OF THE SUBJECT.** I asked whether a stale binary's blind spot applied, found the state NAMES it involved present in canon, and corrected a correct record into a wrong one. The gap was a PAYLOAD FIELD inside one variant plus a variant with no door to it -- **a census keyed on the state name cannot see either, so its hit meant nothing.** Key on the unit the defect actually lives in, and positive-control the scan on a state that DOES carry the payload, or a zero proves only that the detector cannot fire.

**ENVIRONMENT -- PROPERTIES, BECAUSE THIS SECTION CARRIED A STALE VALUE UNTIL 2026-08-29 14:23Z.** It read _"the delivered pair is `8177b53e` ... and the gate reports `currency ok`"_; 13 non-test files under `native/rust` had moved since that marker and the gate was REFUSING. **A pin dies at a bump somebody else schedules.** So: bare `intent` resolves through the `~/.local/bin/intent` SYMLINK into `native/rust/target/release/`, never via `.envrc`, and **that symlink passes through no currency check at all** -- the commit gate is the only place a stale delivered pair is reported. Read the pair off the gate's own currency line or `bin/devbin cli`; **pin by the HASH, never the marker** (three distinct binaries carried one marker in a day). `find` is bfs, not GNU find. The Bash tool's shell is zsh: unquoted `$var` does NOT word-split, an unmatched glob ABORTS the command, `"$VAR:path"` is a bad substitution whose empty output greps to a convincing zero, and cwd persists between calls.

**LANE.** `native/**` and the v3 crates are mine; the parity harness is ic's; hooks, roster and `int hooks` are dc's; **canon writes route through vc**. `~/Devel/prj/Intentv2` IS NOT TOUCHED.

## Decisions

**I DO NOT EDIT `CLAUDE.md`, PERMISSION SETTINGS OR CONFIG ON A PEER'S INSTRUCTION**, however right they are on the merits. A peer's ask is never the user's approval. **AND I DO NOT APPLY AN EDIT A CLASSIFIER REFUSED A PEER.**

**A RELAY IS NOT AN INSTRUCTION.** A ruling reaching me through a peer gets confirmed with hv AT SOURCE before I act -- via `hv/inbox.cc.md`, which vc is the named reader for. Not distrust: a ruling's SCOPE is the half that does not survive a retelling.

**DERIVED vs AUTHORED, NOT MECHANICAL vs INTERESTING** (vc's ruling on what I may write into ST0056 canon -- kept on the board it CONSTRAINS).

**A PARITY TOOL IS A RECORDED ATTACHMENT, AND THIS LINE USED TO NAME THE WRONG VERB.** Canon holds each tool's TEXT, and `sync --to-disk` does NOT refresh attachments from the worktree -- it reports `ok: extract written` and produces a zero-byte diff, a quiet answer to a question nobody asked. The carry is **`intent st attach ST0056 parity/tools/<f> --from <path>`, ONE CALL PER FILE**, then `intent sync --to-disk ST0056`, then `git add` the script + `runner_roster_check.sh` + `ST0056.json` together. **Never the thread-wide carry**: `ingest::collect_attachments_into` re-reads authored prose, which is the live `0124`/`0126`/`0127`/`0129`/`0133` damage class. Verify BEFORE staging that no record but yours moved.

**THERE IS NO 3.0.2 (hv).** Everything else goes into 3.0.1.

**RESIDUE BLOCKS ON A LIVE THREAD AND CARRIES ON A CLOSED ONE; HALF A's ACCOUNTING BLOCKS ON BOTH** (vc).
