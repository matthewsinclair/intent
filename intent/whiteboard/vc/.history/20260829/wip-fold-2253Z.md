---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-08-29 20:43Z
status: active
focus: "SCOPE RULED WIDE by hv: WP-08/09/13/14/15/16 ALL in 3.0.1 plus a web view on the daemon; cost stated twice and taken. Work doled out to cc (round-trip, register shipped-derivation, WP-08+web, WP-13), dc (ST0066, WP-07 seventh criterion, smoke arm, WP-11, WP-14), ic (Assignment 2 doc half, four false doc homes behind cc, AC-00.5 door, WP-09). MINE: WP-15/16 criteria FIRST because neither can gate without them, then ST0068 AC-02.1/02.3, U3 narrowing, AT coverage gaps."
claims: [ST0056, ST0057, ST0058, ST0060, ST0066, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`.** Pre-fold body verbatim at `.history/20260829/wip-fold-1716Z.md` (46KB); everything dropped here is there.

## DOING

**NOTHING IN FLIGHT. PAUSED AT EOD.** Today's reasoning is in the COMMIT MESSAGES and in Watch-outs, and is not restated here. Pre-fold board verbatim at `.history/20260829/wip-fold-1915Z.md`.

## OPEN

1. **FOR hv -- 0143: was dropping `--skip-settings` deliberate?** v2 had it on `intent claude upgrade`; v3 has `--apply` and `--force` only, so a project that wants Intent without Claude Code lifecycle hooks cannot say so. **Under fail-forward, removing a flag is legitimate when it is a decision** -- nothing found records this as anything but a port-time omission. Doc half fixed and propagated.
2. **FOR hv -- THE RATIFIED GUARD COLUMN IS NOT A VOCABULARY, SO AXIS C CANNOT GATE.** Instrument landed `7a366c2d`; entry states and edges gate exactly. Machine 3's Guard cells hold effects and landing rules rather than preconditions -- 28 agree, 3 disagree, **6 UNMEASURED of 37**. The code is right and the column describes something else. **Giving it a controlled vocabulary is an edit to a ratified table, so hv's.** S either way; on the price list, not up for a ruling alone.
3. **THREE `st edit` DEFECTS, ROUTED TO cc, MINE TO TRACK.** The refusal on a generated view is CORRECT -- **the defect is that the register's DEFAULT for a path-printer is the one file it must refuse**. The refusal's remedy names `intent st` and **no `intent st` verb writes `objective` or `context`** (controlled: `title` is mentioned by four). And **`st edit ST9999 <file>` never reports an unknown thread** -- two different wrong stories depending on the file argument. **The third is the one to fix first.**
4. **0136's ~44-site `AcState::Computed` change lands after v3.0.1**; one-commit-or-split is mine at the cut and I lean split.
5. **hv's parked stack**, unchanged: mechanical window refusal; instruments placement; publish_home temp root; vacuous doctor remedy; the ratified-surface pile; Conflab's 4 contract-prose edits; Lamplight md-to-store; Laksa's DESCOPED token; cc's `issues list` holdout and `--status a,b` ordering; ST0066 minutia 3 (free text in `because` versus a structured field).
6. **STANDING FACTS.** `AcState::in_scope()` has NO CALLERS. Empty dirs survive at `~/.claude/skills/in-start/` and `in-next/` -- a directory-presence check reads them as installed; needs an out-of-repo write. **Zero `fiat` rows exist anywhere and no verb reaches the state** (`ac fc` unbuilt, declared an orphan), which is why 0137 is watched rather than escalated.

## Watch-outs

**Mechanisms only. The incidents are in the fold archives.**

1. **A true result from an instrument that could not have answered differently.** The dominant class. Positive-control the instrument, not the subject.
2. **Relaying is authoring.** A number you pass on is a number you asserted.
3. **A claim outlives its basis and nothing announces it.** A fact whose basis was withdrawn reads exactly like one that still holds.
4. **An option set distorts by what it prices and by what it binds.**
5. **Mechanism beats a note.**
6. **An instrument's defects must fail toward alarm, and a permanent alarm is the same defect.**
7. **The message is not the mechanism, and a remedy is not its description.**
8. **Warning a peer about a trap consumes them as an independent witness to it.** Blind by default; offer the number, never impose it.
9. **Audit before cleanup; convert before sweep; ingest before prune.**
10. **Shared checkout.** Never remove a peer's `index.lock`; never `cp` a shared source aside to mutate; only a detached worktree sees a broken published tree.
11. **"Held" is a property of a file that is not where an accident can reach it.**
12. **A test-isolation guard is scoped to the ambients it names; every unnamed ambient is unguarded by construction.**
13. **Canon is the SSOT for rows, not for prose.**
14. **Reading the write-up of a class is not protection from it. The remedy is never care.**
15. **A constant metric guards nothing.**
16. **Two sources agreeing is not corroboration when one is stale or blind** -- and the worse form is both blind in the same direction.
17. **A second notation typed from a first is a second home for the fact; one derived from a single read is not.**
18. **The renderer and the formatter are two writers of one file, and a verb between them cannot converge.**
19. **An authority chain transmits the method it was built to fence.**
20. **A warning about a closing window is a scheduling signal, and it recruits peers into racing the window.**
21. **A fold archive named plain `wip.md` invites the next same-day fold to overwrite it.** Always `wip-fold-HHMMZ.md`.
22. **Relay the instrument's OUTPUT, never a characterisation of it.**
23. **For a TRACKED file, `git commit --only <paths>` with NO `git add` and NO reset.** `add` plus a scoped commit for NEW files -- and **`--only` on a DIRECTORY stages new files under it**, which is how a peer's untracked work gets swept into your commit.
24. **A repo-local gate arm that reads the working tree makes one node's mid-edit everyone's refusal.**
25. **An identifier a tool allocates is read from the tool's output, never predicted** -- and in a shared checkout the allocation can move under you.
26. **Two artefacts declared as second witnesses are a witness pair only if both are watched, and the unwatched half is usually the authority.**
27. **A census takes its unit from the subject's identity, never from the filesystem's.** Per-thread, never per-path.
28. **An aliased `json_extract` in `GROUP BY` returns a silently wrong grouping** -- and it was right on the first sample. Use `GROUP BY 1`. General form: when the grouping key is an expression, the alias is a display name and which one the engine uses is not something the query says out loud.
29. **A menu that omits an option already chosen elsewhere manufactures a contradiction in the principal.** Ask what the other sessions have already been asked BEFORE writing options. A longer menu is not the defence -- mine failed because a false premise made the winning option unthinkable, not unlisted.
30. **`script -q` cannot allocate a pty from these sessions** (stdin is a socket), so every `[ -t 1 ]` probe returns NOT-TTY including the control. **The working instrument is an in-process pty allocator.** General form: when an instrument depends on a property of the SESSION rather than of the subject, its result is about the session.
31. **An artefact that arrives later defeats a check that ran once, even when somebody remembered.** Measured: `CLAUDE.md` at init, `AGENTS.md` at first `agents sync`, `.gitignore` at `upgrade` -- three arrival times, so a subject can be created, checked, PASS, and then grow the file nobody re-checks.
32. **A claim about a TOOL's output is not a different category from a claim about its subject.** I endorsed a peer's characterisation of build output and prescribed a fix for a defect that did not exist; the banner was there all along. Their cause was sharper: they piped through `tail -4` and reported the tool had said nothing. **A `tail` cannot distinguish "the tool was quiet" from "I discarded what it said".**

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **Announce a write to a SHARED file to everyone; announce a write to a CLAIM to the claim-holder. Canon is the first, not the second.** Claims predict who cares, not who is blocked.
- **Rule 13 canon edit path:** edit the extract, `intent sync --to-store <ID>` scoped, then `--to-disk <ID>`, then commit file and canon TOGETHER. **Sync reads the WORKTREE**, so committing first leaves that commit permanently divergent.
- **A surface claim travels with the thing that makes it checkable** (ic). For a verb that is "it is in the register" -- a ruling made this hour and a change landed this hour read identically in a message.
- **Docs are written against the CUT, never against `main`**, and the reference is GENERATED against a named revision.
- **Design decisions go to the Laksa design agent, not settled here** (hv). Each must carry the decision, the constraint it must respect, and what breaks if it goes the other way -- a flag saying only "open" hands the same ambiguity on in a tidier format.
- **Em dash in prose pages; `--` in generated reference pages**, because those render CLI text a reader copies.
- **v3.0.1 stays 3.0.1** with the four-new-verbs cost stated and taken by hv. I recommended 3.1.0, hv declined, I do not raise it again.
