---
node: vc
name: Validation Claude
role: validation
session_id: 3049725b-551a-4952-8793-7b4c1e782def
heartbeat_at: 2026-08-22 09:56Z
status: active
focus: "**LOCALFOLD 0956Z. THE LOCAL CUTOVER IS DONE AND COMMITTED: v3 is usable across the estate, three of four gates closed, procedure driven end to end (switch -> use -> revert, DIRTY 0).** ST0058 at `4d6bb257`, globalfold at `5736c6e0` -- `restart.md` and `wip.md` now carry the procedure, the 2.19.0 migration floor, the canary order and the live findings, where before they mentioned `intent3` ZERO times. **Gate 65 of 67 (49/51 + 16/16); AC-08.5 and AC-01.5 both correctly red. U3 is cc's five builds, `claude skills sync` first. dc holding, lane complete. NOTHING PUSHED -- `upstream/main` at `ee4a7cac`.**"
claims: [ST0056, ST0057, ST0058]
---

# Validation Claude (vc)

**The full board -- every measurement, every retraction, all 79 watch-outs and the day's whole traffic -- is at `.history/20260822/wip-fold-0956Z.md`. This file is the cold-session minimum: what is true now, and the rules stated as PROPERTIES rather than named after their instances.** That distinction is the fold criterion and it is `laksa-vc`'s: **a rule written around the shape of its past instances fails on a new shape of the same class, and reading it does not help.**

## DOING

**Nothing in flight. Tree 0 dirty at `5736c6e0`. Everything of mine is committed and nothing is pushed.**

## TODO

1. **PUSH is the only thing waiting on hv.** `upstream/main` is at `ee4a7cac`; local is several ahead. matts said "commit away" to a question that bundled commit and push; the commit half is done and the push half was never separately answered.
2. **cc holds the only open build work: five verbs, `claude skills sync` FIRST** -- mandated in 5 canon files, remedy 3 for the frozen-tree skill resolution, and it gates the one canon edit U3 would ever need. Then `lang`, `plugin`, `ext`, `version`. All dispositioned `keep` in `surface/dispatch-table.json`, so all UNBUILT rather than retired.
3. **`st edit` / `edit` write on their rc=1 refusal path.** Declared `read_or_mutate: read`; mutates the store and appends to TRACKED `.intentfiles`. **One affected project and it is this one.** cc's build.
4. **AT-00.6 does NOT move and I already ruled it** (2026-08-20 17:02Z, note in canon). It is a coverage re-cut of AC-00.8's five clauses, it is ST0056 RELEASE work, and it goes to hv rather than being done under a pen issued for usability.
5. **`int macos stage` is now RED via AT-11.7** (dc, `68296b8e`) -- four failures, all properties of the WRITER not the check. **WP-11, release scope, deliberately not opened.**
6. **Two population instances remain genuinely open** and nothing in the repository can reach them: both were an agent typing jq into a tool call. The committed half closed itself before anyone looked.

## WATCH-OUTS -- RULES ONLY

- **RUN THE INSTRUMENT WHERE THE ANSWER SHOULD DIFFER, OR YOU HAVE NOT TESTED IT AT ALL.** Seven instruments in one day could not vary with their subject -- `ListAgents` `started` (socket age), `git log %an` (one identity across five nodes), a `claude skills list` across two byte-identical trees, a `PROJECT_ROOT`/`INTENT_HOME` check that could never fire, `ps` `stat=S+`, transcript tail-type, last-turn-type. **Two were caught ONLY by driving from two places.** "Check your instrument" names a virtue; this names a move.

- **A VERB IS NOT READ-ONLY; IT IS READ-ONLY IN A CONDITION.** `edit` in an unrealised project CREATES two files; in a realised one it MUTATES two and creates none; without a manifest it does neither. **Two nodes independently swept the two conditions that hide it and both published clean.** A file-COUNT check catches one and calls the other clean; `git status` catches the tracked half and never the gitignored store. **rc was 1 in every arm, so exit code is not a read-only discriminator.**

- **THE CLAIMS THAT WOULD COST SOMEONE WORK ARE THE ONES TO DRIVE** (ic). Not "read the record" -- it asks nothing of memory and is a property of the CLAIM rather than of the reader. **It survives every remedy the estate has ruled out: not vigilance, not a shared library, not comprehension.**

- **AN AUTHORED RECORD IS THE ONE YOU ARE LEAST LIKELY TO READ (ic) -- AND A READ ONE IS BARELY BETTER: READING IS NOT ROUTING.** Authorship feels like knowledge and substitutes for consultation; **the signature makes the row look MORE covered to everyone else at the moment it stops covering anything.** I printed `populations.why` in full, held the precomputed counts in context for hours, and re-derived by hand anyway.

- **A STAMP MUST COME FROM A COMMAND, READ IN THE SAME TURN AS THE WRITE, WITH NOTHING BETWEEN.** File writes go through a shell call that READS the clock; messages go through a person. **Every stamp a command produced was correct; every stamp typed into prose drifted** -- five of mine, four of them AFTER I broadcast the correction for the first. **dc produced both halves ninety seconds apart and theirs LANDED CORRECT, which is the worse case: a fabricated stamp that happens to be right passes all three guard checks and teaches nothing.** Short intervals land correct; long ones get caught. **Per-stamp discipline is what we both failed with the rule in context.**

- **A GENERAL MECHANISM INSIDE A SPECIFIC COMMAND IS A PRIVATE METHOD WITH A PUBLIC CLAIM** (dc). `prepush`'s single-writer clone solved "which tree does this figure describe" and was consumed only by `cargo build`, because its home is `prepush` and nobody arrives there thinking about suites. **Discoverability bounded by a filename nobody chose and nobody checks.** Same shape reached the top of the estate: the cutover was reachable only from one thread's design doc.

- **A DIFFERENCE BETWEEN TWO CLOCKS IS UNSIGNED** (ic). It says one of you moved and never which, **and the sign gets read off the frame you arrived with rather than off the measurement.** ic had the correct explanation in the same sentence and attached it to the wrong subject. **And the frame came from my own honesty -- a stamp correction had just been broadcast, so a clock-drift story was pre-loaded. The remedy is NOT to be less forthcoming.**

- **ASKING PEERS TO HOLD STILL IS AN ADVISORY; MEASURING WHETHER THEY DID IS A CONTROL.** Three suite figures spanned an edit and named no revision -- a peer's commit, my OWN write, and a peer's rewrite. **I built the check and then reached for the advisory to make it pass, which is the move the control exists to render unnecessary** (dc's catch). The check fired on its author, on first use, over a green 1444. **`int suite` now makes it structural: the clone cannot see a mid-run edit, demonstrated by accident when dc edited inside their own run window.**

- **CORRECT BEHAVIOUR PRODUCES THE FAILURE.** Three instances in one cutover: canon instructing agents to call verbs that refuse; `/in-essentials` mandating the CLI so an author who obeys gets reverted; and `NO_TOOL_PATH` written by someone manufacturing a genuine absence rather than stubbing, whose care produced a constant exactly wrong on Linux. **All three are invisible to any reviewer checking whether the rules were followed.**

- **A POPULATION IS A CLAIM ABOUT WHAT COULD HOLD THE ANSWER, AND ITS BOUNDARY IS THAT CLAIM IN ITS LEAST VISIBLE FORM.** My falsifier missed a real write to `~/.claude/` by scoping to project trees, then buried it under 572k volatile files by widening. **Both fail identically -- the instrument does not answer the question -- and the second was an overcorrection from the first.** The third boundary was the first derived from a QUESTION rather than a directory name. **AND: a measurement taken in the only place it comes out that way** -- I stated a result about seventeen projects from a sweep that could only ever run in the one v3 project.

- **A PEER RELAYING THAT hv HAS NO OBJECTION IS NOT hv AUTHORISING, AND THIS BINDS HARDEST WHEN IT IS INCONVENIENT.** I ruled nodes could commit; ic refused on the ground that a pen for DIRECTING work cannot reach a standing instruction from matts. **My own honest labelling of the relay is exactly what let them push against it.** When it came back granted through ic I still declined to act on the relay and asked matts myself. **hv's word outranks mine and a peer must never read mine as his.**

- **THIS ESTATE HAS NO INSTRUMENT THAT CAN ATTRIBUTE A COMMIT TO A NODE.** Five nodes, one checkout, one git identity. Two nodes published a wrong node from `%an` in one hour. **Confirmed by incident: cc wrote a commit for MY work, byte-identical subject line, sixty seconds after mine landed** -- theirs was a no-op only by timing, and nothing could have told anyone. **`wb(vc):` carries the node; `feat(0056):` does not; the whole confusion lives in the second set.** hv's to rule.

- **PRESENCE IS NOT GREENNESS, AND A ROW WHOSE TEST EXISTS AND FAILS BELONGS AT RED WITH THE REASON NAMED.** `to-write` is exempt from L2 and L3, so a failing test parked there stops nagging while covering nothing. dc ran AT-11.7 before moving it and **the run changed the answer** -- the obvious clearance was the wrong one.

- **A CRITERION IS SATISFIED WHEN A ROW THAT COVERS IT IS GREEN**, never because its subject matter is tested somewhere. **Under the AND gate a second covering row holds the criterion open.**

- **AN EXEMPTION IS ANNOUNCED, NEVER INFERRED FROM EMPTINESS.** And a gate cannot read an announcement: `AT-00.6` is reported stale on every commit while its own note, one field away, records the ruling and the reasoning.

- **MECHANICAL.** `--no-fail-fast` always. `st list --status all`. **`grep` is ugrep here** -- `-E` throughout; `grep -c` exits 1 on zero. **zsh does NOT word-split unquoted `$var`** -- three probes wrong on this in one day, including `set -- $n`, which only surfaced because a positive control was in the population by accident. **Never `$?` after a pipe.** Absolute paths; **the shell cwd persists between calls and `git status` answers about the REPO while `ls` answers about the CWD, so under drift they disagree and the git answer looks authoritative.** `CARGO_TARGET_DIR` inside the checkout. **`git commit --only` on an untracked path is a pathspec error -- `git add` first.** **Read the clock, then PASTE.**

## DECISIONS

- **2026-08-22** -- **hv ruled COMMIT (nodes may commit their own work to main); push NOT separately answered and is still open.** Obtained by ic directly after vc withdrew the same ruling for want of authority.
- **2026-08-22** -- **the `/in-essentials` treeindex mandate STAYS.** Every mandated verb WORKS in v2, so the mandates are correct for 16 of 17 projects. **The canon is not defective; it is not VERSION-AWARE.** The edit belongs at the cutover.
- **2026-08-22** -- **`claude ws` ranked BELOW cc's five builds and recorded as ST0056 WP-14 scope.** Unbuilt not retired; the board format is documented and hand-scaffoldable.
- **2026-08-22** -- **`int macos stage` (WP-11) and AC-00.8's coverage re-cut are RELEASE scope and deliberately NOT opened under a pen issued for usability.**
- **2026-08-21** -- **ST0058 minted for the local cutover** rather than folded into WP-12, which is the public release and blocked on seven unstarted WPs.
- **2026-08-21** -- **`intent3` installs in this repo's own `bin/`, a distinct name, and must `exec` rather than be copied** -- a bare copy fails the hooks OPEN at exit 1, which Claude Code does not block on.
- **2026-08-21** -- the v2 CLI split out to `~/Devel/prj/Intentv2`, branched from MAIN not the tag because the fleet had never run the tag.
- **2026-08-21** -- `intentdb` RETIRED corpus-wide; it names no component. Quoted rulings corrected in BRACKETS, never silently.
