---
node: vc
name: Validation Claude
role: validation
session_id: b1060b1c-548a-4974-810e-73691ac00c67
heartbeat_at: 2026-08-29 19:14Z
status: paused
focus: "THE PUSH GATE IS GREEN AT `f0098c7b` -- hv was blocked and is not any more. fmt was always clean; 113 clippy findings were LINTS promoted by `-D warnings`, not compile errors. The 113 was not the population: compilation aborts per crate, so three more sat behind the abort and only a run WITHOUT `-D warnings` counted them all. Refuted my own no-toolchain-pin hypothesis rather than confirming it. The fmt+clippy arms are OUT of prepush on hv's ruling -- CI ran the identical commands all along. ON THE BOUNCE: the store model-inconsistency doctor reports, ST0065 to hv live, ST0068 AC-02.1/02.3."
claims: [ST0056, ST0057, ST0058, ST0060, ST0066, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`.** Pre-fold body verbatim at `.history/20260829/wip-fold-1716Z.md` (46KB); everything dropped here is there.

## DOING

**THE PUSH GATE IS GREEN AND THE LINT ARMS ARE OUT OF IT (`f0098c7b`, 7 files).** hv could not push; measured, not characterised: rustfmt CLEAN (positive-controlled), code COMPILED (clippy without `-D warnings` rc=0), 113 lints promoted to errors. **106 `result_large_err` from ONE cause** -- `FacadeError::Organize` carrying `PreconditionsUnmet`'s `Verdict` + threads past 128 bytes -- deferred with scoped allows on hv's ruling, boxing sequenced after the tag in 0136's shape. **Controlled BOTH ways: an extra `-D` still returns rc=101, and force-warning the lint back on still surfaces 110, so it is suppressed rather than absent.**

**THE KEEPER IS THAT 113 WAS NOT THE POPULATION.** Compilation aborts PER CRATE, so the first run counted only what it REACHED; three more findings sat behind the abort in integration-test crates and surfaced one at a time as each fix let the build get further. **The complete count only came from a run WITHOUT `-D warnings`, where a warning does not stop the build. The instrument that ENDS the count is not the instrument that TAKES it.**

**AND I REFUTED MY OWN HYPOTHESIS RATHER THAN CONFIRMING IT.** The no-toolchain-pin flag was mine, and it was the obvious cause. It is wrong: one rust in the Cellar (1.98.0, installed 2026-08-23) and the last green push was 2026-08-26, so the toolchain PREDATES the green. This incident is not evidence for a pin and I will not let it be cited as such.

**cc RULED store.rs AND THE RULING BEAT MY FRAMING.** I read an orphaned doc block as documenting a DELETED function and offered delete-and-quote-in-the-commit. cc: the SUBJECT MOVED, to `ProjectStateEdit::SetTodoWatermark`. Reuniting it with the variant keeps the reasoning at an ADDRESS; a commit message keeps the bytes and loses the address. **Clippy's own machine-applicable fix would have merged it into `set_todo_watermark` -- a function that TAKES a mark -- making its doc assert the INVERSE of D42.** An auto-fix that writes a false document is why `--fix` did not get run blind.

**MINE AGAINST ME: I wrote an apostrophe inside a single-quoted bash `printf` in the gate itself.** Hard syntax error, caught by `bash -n`. I read my own edit instead of driving it, one turn after telling hv that a `tail`-shaped instrument cannot distinguish quiet from discarded.

**DEP SURVEY DONE, READ-ONLY.** Nine direct workspace deps, 245 packages locked. **Two declarations are load-bearing and get re-checked after, not assumed:** `jsonschema` at `default-features = false` (defaults pull reqwest + TLS and a network-reachable `$ref` resolver) and `clap` with `features = ["string"]` (the surface is built from runtime table data). **No `.tool-versions` -- this project declares no toolchain pin**, so the build takes whatever `rustc` is on PATH (1.98.0) and a dep refresh is less reproducible than it looks. Flagged to hv, not a blocker.

**ST0068 IS 4/9 AND THE DOC SET IS VERIFIED END TO END.** AC-01.1, AC-02.2, AC-02.4, AC-04.1 satisfied, each positive-controlled. 126 verbs across 27 pages agree with the register and the tag; 61 links, 0 broken; no basename collides with an agent-instruction file. **Remaining: AC-02.1 walkthrough, AC-02.3 defect coverage, AC-03.1/03.2 (Laksa's build and its decisions reaching their agent), AC-04.2 (same tag as v3.0.1).**

**LAKSA IS BUILDING.** Kickoff sent to laksa-{cc,vc} on hv's instruction. laksa-cc and laksa-ic build; laksa-vc validates. **§1 was rewritten after their review: the prefix list is CURATED and now says so, the selection rule is written down, and the counts are withdrawn as unreproducible against a corpus two sessions are committing into.**

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
