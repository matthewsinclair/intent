# cc -- archived 2026-08-20 (fold 5)

The day's settled narrative, moved off the live board at 2026-08-20 09:08Z. Everything here LANDED; the live
board carries only what is still owed.

Landed today: `d4f12c0a` #144 -- `f0c2805c` board + the 250 partition -- `1e2bc65e` the gate arm --
`aa4c3ac0` canon repair -- `9e191824` the parse expiry, in the compliant order -- `d709d89c` board --
`1fe2122f` DO-NOT-PATH mechanism -- `7d20b666` AC-01.2 + AC-01.4 instruments -- `9e96ac75` both rows green --
`637b113a` board.

## NEXT -- mine

**Fold 4's narrative is at `.history/20260819/wip.md`. Today reshaped it by 06:47Z.**

**0. THE GATE ARM LANDED -- `1e2bc65e`, rostered `manual` awaiting dc.** `intent/st/ST0056/parity/tools/thread_view_skew_check.sh`. **Gated skew coverage was 1 of 269**: the sibling's `CHECKABLE` is ONE triple under `surface/`, and the missing 268 are the thread covers, acceptance contracts and WP covers. **130-150ms at `f0c2805c` on one machine against 2860-2940ms for the sibling**, so it would be the CHEAPEST gated instrument rather than the most expensive. **It forms no verdict** -- `views::skew` stays the single home and this parses one answer rather than computing a second. **It refuses at exit 2 when it cannot read doctor's summary**, because a text-reading gate whose needle stops matching goes green forever and nothing says so. Ten arms; the last two are a MENTION/SUBJECT pair taking identical decoy lines to opposite verdicts. **The parse carries a NAMED EXPIRY** (`9e191824`): when `doctor --json` lands, delete it rather than keeping both. **Ungateable before vc's `b082b488` -- the blocker was never the wiring.**

**0a. WP-01 IS 6 OF 7 -- AC-01.2 AND AC-01.4 GREEN** (`7d20b666` built, `9e96ac75` moved). `canon_clone_completeness.sh`: 97 of 97 clean, 96 of 97 with one artefact untracked-and-ignored, 97 of 97 restored -- same rig, one rule apart. **Its population is the MODEL, not the canon files: enumerating `.canon/*.json` asks whether TRACKED FILES CLONE, true by construction and green on an estate that had lost half its artefacts.** `canon_concurrent_diff.sh`: 2 edits -> 2 distinct paths, 1 edit -> rc=1. **`at green` is reachable only from `red`, so red-first is structural here rather than remembered.** Both row NOTES are now stale and are vc's. **The seventh row is AC-01.5 and nothing I can edit will meet it.**

**0c. AND THE DAY'S CLASS TURNED UP IN MY OWN INSTRUMENT.** `canon_concurrent_diff.sh` first compared the observed path count against the number of edits MADE, so `--one-edit` reduced both together -- one against one, self-consistent, green, **unable to fail**. **An arm whose expectation tracks its input is not a control.** Third variant of the shape today and the first that was mine; the expectation is pinned at two now.

**0b. AND AC-03.6 FIRED ON ME, UNPLANTED, WHILE DOING IT.** `1e2bc65e` edited `runner_roster_check.sh` -- an ATTACHMENT of ST0056 -- and committed it with canon naming the old bytes. `canon_commit_check.sh`: `ADDS 1 of 1` rc=1 at `1e2bc65e`, clean at `aa4c3ac0`, `ADDS 0 of 88` at `9e191824`. **`1e2bc65e` is permanently divergent in history.** **dc corrected my framing and the correction is the finding**: the rule being on my board is why the instrument exists, not why the commit diverged -- **a control that depends on the author remembering is not a control, it is a hope with a filename.** `manual` costs a divergent commit roughly whenever anyone is busy. dc: an unplanted positive control is strictly better evidence than a planted one, and their admission condition is substantially met.

**1. THE GATE HAS THREE DISPATCHERS AND THEY AGREE WITH EACH OTHER NOWHERE.** Measured at `483fbcfe`, 06:45Z, and vc converged on the identical table independently:

    guard                     pre-commit.intent   cmd/precommit   template roster   RUNS?
    whiteboard-clock-guard            1                 0                1           YES
    whiteboard-header-guard           0                 2                1           YES
    canon-ignore-guard                0                 0                2           NO
    append-only-guard                 0                 0                1           NO

**Two of four run and neither runs through the roster** -- each reaches its guard by a hard-coded path in a different file. **The roster is a THIRD OPINION about what runs, agreeing with neither dispatcher**, and it is the only artefact naming all four. `GUARDS_APPLY` occurs in exactly one file in the tree and in nothing under `.git/hooks/`. **The roster and the dispatcher it describes are ONE file in the repo and TWO on disk, so every check comparing them passes.** With dc; three shapes offered, none picked by me.

**2. AC-01.5 IS UNMEETABLE BY ANY EDIT TO THE GUARD, THE ROSTER, OR THE TEMPLATE.** `canon-ignore-guard.sh` is built, mutation-proven, rostered, and has zero call sites on git's path here. AT-01.5's `red` is correct; both recorded reasons are wrong -- not _not yet wired_, not _wired into a stale file_, but **wired into the roster in a repo whose commit path does not read the roster.** vc holds the canon reword. **A consumer on a fresh install is probably fine and I have NOT measured that** -- do not repeat it as though I had.

**3. #144 FIXED, MUTATION-PROVEN, UNCOMMITTED.** `claude_md_template.bats` asserted four placeholders; `b277013a` removed `[[DATE]]` deliberately. Split into a positive test over the three and **a negative assertion carrying the refusal.** The mutation is the finding: **planting `[[DATE]]` back leaves the three-placeholder test GREEN and moves only the negative one.** A trim records a removal; only a negative assertion defends it.

**4. THE 250-FILE OWNABILITY PARTITION -- DELIVERED 06:55Z, counted at `5b59a14c`, dirty 10. Durable copy in vc's inbox.**

    T  tool payload, not project content    187    intent/plugins/
    B  project content, needs a NEW sigil    59    docs 10, llm 14, history 18, eng 9, autopsy 3, analysis 2, wip/restart/done 3
    N  must never be an artefact               3    .config/config.json, .intentfiles, events.jsonl
    M  already model-derived                   1    todo.md
                                             ---
                                             250

**hv's 250 IS REALLY 59.** `intent/plugins/` resolves from `$INTENT_HOME` and this repo has it only because it IS its own -- **0 tracked in Lamplight, Laksa and Anvil.** **My first hypothesis died in the same probe: I expected `intent/docs/` to be tool payload too, and those consumers carry 61, 4 and 2 there** (`llm/` 21/12/6, `eng/` 0/38/11). A count varying by two orders of magnitude across consumers is project content by definition; only `plugins/` is uniformly absent. Three consumers, one machine -- a probe, not a fleet survey.

**OF THE 59, ZERO ARE OWNABLE BY AN EXISTING ARTEFACT, AND IT IS STRUCTURAL RATHER THAN A JUDGEMENT.** 58 of 59 are `.md`, so `ATTACHMENT_EXTENSIONS` is not the constraint. **Ownership flows ARTEFACT -> ITS OWN DIRECTORY**: `classify` answers only inside a thread dir, a thread realises `intent/st/<ID>/**` and nothing else, and none of the 59 belongs to ONE thread. **THE BLOCKER IS ARITY, NOT POLICY** -- two sigils, and a file with no owning artefact cannot be declared whatever anyone rules.

**`todo.md` IS THE PRECEDENT THE OTHER 59 WANT**: a `View` in `render_all` (`views.rs:951`) -- model-derived WITHOUT being artefact-owned, project-scoped, no manifest entry. **The sole non-`.md` is the known naming violator**, the `.webloc` in `docs/exemplars`, unownable twice over.

**5. AC-03.6 UNCHANGED.** `--staged` landed at `19268867`; nothing owed from me until dc drives the planted-divergence control. **AT-03.6's row text is stale** -- it still says `--staged` is what it needs. vc has it.

**NOT MINE ANY MORE: WP-10 and WP-09 are vc's** (vc moved WP-09 ahead -- `append-only-guard.sh` is one of the two that never fires and `events.jsonl` is the one artefact no rebuild can reconstruct). ST0011's missing completion date is vc's.
