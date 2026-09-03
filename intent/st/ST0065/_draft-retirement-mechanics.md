# DRAFT -- retirement mechanics: `in-start`, `in-next`, `_wip.md`

**FOR hv; vc routes. EXECUTED -- STEPS 1, 2 AND 3 ARE DONE, AND THIS BANNER SAID THE OPPOSITE UNTIL 2026-09-03.** Written 2026-08-28 16:35Z by ic, for vc, under hv's 2026-08-28 ruling that `in-next` retires and hv's adoption of the WP-01/WP-02 verdicts, as a plan that had not been run. **It was run.** Measured 2026-09-03, not recalled: no live reference to either skill remains in canon, `intent/plugins/claude/skills/in-start/` and `in-next/` do not exist, and neither skill is present under `~/.claude/skills/`. **The original banner -- _NOTHING HERE HAS BEEN EXECUTED ... no file has been deleted, no skill uninstalled_ -- was this document's single most load-bearing claim, and it was false.** That is the class the document is about, so it is corrected in place rather than annotated. Only step 4, the verification, is outstanding, and its instrument was wrong; see below. This document existed so the retirement was sequenced by someone other than the person who proposed it, and the sequence held.

**Every step below carries a size, and where a cost is UNMEASURED the step says so.** Whole job as planned: **S** -- six one-line edits, one CLI call, two directory deletions, one verification. **Steps 1-3 are spent; step 4 is outstanding.** **AND THE JOB HAD A COST THE PLAN DID NOT NAME**: `uninstall` left `in-start/` and `in-next/` behind as empty directories under `~/.claude/skills/`, which hv later removed by hand (`0218`). **The ordering was the entire content of this document and the ordering held.** What failed was a claim about what a step would LEAVE BEHIND -- which no amount of sequencing care would have caught.

## 0. The finding that governs everything below

**A canon skill retirement is a THREE-LOCATION lifecycle operation, and deleting the canon file is the LAST step rather than the operation itself.**

The reason is mechanical and I measured it rather than assumed it:

**`sync` never prunes a skill whose canon source has vanished.** `payload.rs:835-841` and `911-917` (**the draft cited `skills.rs:690-697`; there is no `skills.rs` in the v3 crate and that address resolved to nothing when re-checked 2026-09-03** -- the DESCRIPTION was right and only the address was stale) -- when `origins()` yields nothing, sync pushes `Outcome::SourceMissing` and `continue`s. It does not remove the installed tree and it does not remove the manifest entry.

**And `intent upgrade` cannot rescue that**, because upgrade's skill step _is_ that sync: `bin/intent_upgrade:202` runs `intent claude skills sync || true`.

**So a canon-only delete strands the skill on every machine that has it, permanently, and every future sync reports `SourceMissing` at it forever.** The skill would keep loading into sessions -- `~/.claude/skills/` is what Claude Code reads, not canon -- so the retired skill would go on being _invoked_ while the project believed it was gone.

That is the whole reason this is a draft and not a commit.

## 1. Measured mechanics

| Fact                                      | Evidence                                                                                                                                                                                                                                                     |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| sync does not prune a vanished source     | `payload.rs:835-841` + `911-917`, `Outcome::SourceMissing` then `continue` (was `skills.rs:690-697` -- no such file)                                                                                                                                         |
| upgrade's skill step is that same sync    | `bin/intent_upgrade:202`                                                                                                                                                                                                                                     |
| `uninstall` exists and is the only pruner | `intent claude skills uninstall <NAME>...` resolves, rc=0                                                                                                                                                                                                    |
| **uninstall does NOT read canon**         | `payload.rs:1074+` -- manifest + installed dir only (was `skills.rs:846+` -- no such file). **So ordering is free: it works before or after the delete**                                                                                                     |
| uninstall removes only what it wrote      | ruling 5 (vc, 2026-08-22): _a sync may remove what it INSTALLED; it may not remove what it FOUND_                                                                                                                                                            |
| **ruling 5's precondition is MET here**   | both entries record `files: ['SKILL.md']` in `~/.intent/skills/installed-skills.v3.json`. A v2 entry has no file list and would remove nothing; these are v3 entries and prune cleanly                                                                       |
| baseline is clean today                   | POINT-IN-TIME 2026-08-28: canon 25, installed 25, **zero orphans in either direction**. Re-measured 2026-09-03 AFTER execution: canon 23, `SKILL.md` files 23, directories 23 -- and the directory figure agrees only because hv removed the residue by hand |

**The last row is why care is proportionate.** The estate has perfect canon/installed correspondence right now. A retirement done canon-first would be the **first orphan it has ever had**, and the tool would then report `SourceMissing` at it on every sync with no verb wired to resolve it except the one nobody thought to run.

## 2. Dangling-reference inventory

Enumerated by grep over live surfaces only -- skills, templates, docs, `bin`, crates, and the three root canon files -- **excluding the `intent/.canon/` thread records and `CHANGELOG.md`, which are historical records that a retirement does not invalidate.** Control: the same pattern in the same scope returns 94 for `in-session` and 0 for a skill that does not exist.

**8 references, 4 files:**

| #   | Site                                | What it is                                  | Disposition                                             |
| --- | ----------------------------------- | ------------------------------------------- | ------------------------------------------------------- |
| 1   | `in-plan/SKILL.md:3`                | `chains_to: ["in-next"]`                    | **must edit** -- frontmatter pointer to a retired skill |
| 2   | `in-plan/SKILL.md:68`               | `- /in-next -- pick the first work unit`    | **must edit** -- prose Skill Chain entry                |
| 3   | `in-start/SKILL.md:3`               | `chains_to: ["in-plan", "in-next"]`         | dies with the file                                      |
| 4   | `in-start/SKILL.md:49`              | `- /in-next -- if continuing existing work` | dies with the file                                      |
| 5   | `prime/operational-knowledge.md:18` | `/in-start` roster entry                    | **must edit** -- and this is WP-01 finding 8.2          |
| 6   | `prime/operational-knowledge.md:21` | `/in-next` roster entry                     | **must edit** -- same finding                           |
| 7   | `usage-rules.md:183`                | `/in-start` table row                       | **must edit**                                           |
| 8   | `usage-rules.md:184`                | `/in-next` table row                        | **must edit**                                           |

**Six need action; two die with their own file.** The deferred `in-start:23` MODULES.md site from the WP-01 catalogue resolves here exactly as recorded -- it dies with the file, which is why I declined to edit it.

**Note what is NOT on this list:** `lib/templates/llm/_usage-rules.md` is clean. Only Intent's own root `usage-rules.md` carries those rows, so **the fleet template needs no change** and no NEW project inherits the dangle.

**But sites 7 and 8 have a reach limit hv should know about, and I found it after first writing this section.** `usage-rules.md` is **seeded once and never synced** -- `canon.rs:316`, _"USER-OWNED FILES ARE SEEDED, NEVER SYNCED"_. So editing the root file fixes **this repository only**. Any existing fleet member whose `usage-rules.md` carries those rows keeps them **permanently**, because no central path will ever rewrite that file. **This is not a reason to delay the retirement** -- the rows name a skill that no longer exists, which is a stale pointer rather than a live hazard -- but _"the references are fixed"_ would be false if said without this qualification. Measured on this repo, whose own `usage-rules.md` still carries an unrelated wording my template fix corrected days ago and which will never arrive.

## 3. Proposed order

Sequenced so that **no intermediate state has a live pointer to a missing target**, and so the destructive step is last.

1. **Edit the six live references** (sites 1, 2, 5, 6, 7, 8). **XS** -- six single-line edits across four files, no logic. After this step the two skills are unreferenced but still present and still working -- **a safe resting state that can sit indefinitely**, which is what makes step 1 separable from the rest.
2. **`intent claude skills uninstall in-start in-next`** -- **DONE.** Prunes the installed copies and the manifest entries. Removes exactly the recorded `SKILL.md` from each, and leaves anything an operator dropped in. **XS to run, and it is the only irreversible step.**

   **THE CLAUSE _prunes the emptied dirs_ STOOD HERE AND IT IS FALSE OF THE ONE DIRECTORY THAT MATTERS** (`0218`; corrected at the source 2026-09-03). **And it is false for a more interesting reason than the issue supposed.** `uninstall` DOES call `prune_empty_dirs(&dir)` (`payload.rs:1131`), so the prune is not unimplemented. The pruner (`payload.rs:1402`) walks the tree and removes an empty directory **only `if dir != root`** -- and the root it is handed is the SKILL'S OWN DIRECTORY. **So it prunes empty SUBdirectories and is structurally guaranteed never to remove the skill directory itself.**

   **BOTH HALVES ARE CORRECT ABOUT THEIR OWN JOB AND NOTHING OWNS THE SEAM.** A general-purpose pruner must not delete the root it was handed; `uninstall` wants that exact directory gone. **No reading of either function finds this** -- it lives only in the relationship. Third recorded instance of the seam class (ic W19), and the first found by driving rather than by looking for the shape.

   _PROVENANCE OF THE ORIGINAL ERROR, which is why it is recorded rather than quietly deleted: the claim came from reading the MANIFEST -- truthful about files -- and INFERRING the directory behaviour from it. It was never driven. That is this estate's own staleness mechanism operating inside a document about mechanics._

3. **Delete `intent/plugins/claude/skills/in-start/` and `in-next/` from canon.** **XS**, two directory removals, recoverable from git.
4. **Verify -- OUTSTANDING, and the instrument this step named was the wrong one.**

   **COUNT `SKILL.md` FILES, NEVER DIRECTORIES.** `find ~/.claude/skills -maxdepth 2 -name SKILL.md | wc -l` against `ls intent/plugins/claude/skills/ | wc -l`. **A directory count answers a question about directories, and the question here is about skills** -- and since `uninstall` leaves the directory behind (step 2), the directory count reports a discrepancy that does not exist. Then: a `sync` reports no `SourceMissing`; the reference grep returns zero live hits with the control still firing for `in-session`. **XS**, and the control is the half that matters -- a zero from a grep that has not been seen to fire is not a verification. _The original `94` control figure is NOT re-driven here; treat it as unverified rather than as a target to match._

   **AS MEASURED 2026-09-03: canon 23, `SKILL.md` files 23, directories 23 -- all three agree. THE DIRECTORY FIGURE AGREES ONLY BECAUSE hv REMOVED THE TWO RESIDUE DIRECTORIES BY HAND.** A check that passes because somebody cleaned up outside the procedure has not passed. **On the next retirement, with `0218` remedy 1 unfixed, the directory count is wrong again** -- which is the whole reason this step is corrected rather than ticked.

**Steps 2 and 3 are order-free** because uninstall does not read canon -- but doing 2 first means no step ever leaves an orphan, which is worth more than the flexibility.

**AND THE ORDERING ANALYSIS ABOVE HAD A GAP THE EXECUTION FOUND (vc's sequencing correction, folded in at `ae6a83ce`): `sync` IS A REINSTALLER, AND IT IS NOT ONE OF THE NUMBERED STEPS.** `sync` installs FROM canon, so **a sync run between step 2 and step 3 puts both skills straight back.** The document reasons carefully about the order of the three steps it names and says nothing about the verb that undoes one of them. **Actually executed: references first (`d10da182`), then hv ran `sync` BEFORE `uninstall`, then the canon delete (`ae6a83ce`).**

    _The lesson is not about sync. It is that an ordering proof over the steps you listed says nothing about a step you did not list_ -- the same reach limit as an instrument bounded by what it checks rather than by what can happen.

## 4. `_wip.md` is a DIFFERENT kind of retirement, and should not ride this sequence

Measured separately, because I expected it to be the same shape and it is not:

- Template is `lib/templates/prj/_wip.md`; a fresh `intent init` **does** write `intent/wip.md` (1090 bytes, verified on a real init).
- It **is** embedded in the binary, so changing it needs the rebuild -- unlike the skills, which are read from disk.
- **There are no installed copies to prune.** Every emitted `wip.md` belongs to the project it was written into and is now that project's own working file. A retirement neither can nor should reach them.

So `_wip.md` has **no fleet-orphan problem and a rebuild dependency**, which is the exact inverse of the skills. Its retirement is a template change plus cc's `init.rs` disposition, and it is **cc's to execute** -- vc is holding them for this draft, and the answer this draft gives them is _the skills sequence does not apply to you; yours is a one-step template change gated on the rebuild._

**What the emitted file actually says, since the verdict turned on it:** it carries `verblock: "06 Mar 2025..."` stamped into every new project, and a `## Context for LLM` section whose advice is _"start by sharing this document"_ -- paste-into-chat guidance from before the tool had a session bootstrap, a gate, or a whiteboard. That is the MISGUIDED half of WP-01 finding 5, confirmed against the bytes a real init emits rather than against the template.

## 5. What I need a ruling on

1. **Reach.** The manifest is per-machine (`~/.intent/skills/`). Step 2 prunes **this** machine. Another machine, or another fleet member, keeps its installed copy until someone runs uninstall there. **There is no fleet push, and I am not proposing one be built for two skills.** But the retirement is not "done" in the sense the word implies, and I would rather that be stated than assumed.
2. **Is `SourceMissing` a sufficient signal?** It is a report with no remedy wired to it. If the answer is that retirements are rare enough to handle by hand, that is a fine answer -- but it should be a decision, not a gap.
3. **Sequencing against the rebuild.** Steps 1-4 need no rebuild (skills are read from disk). `_wip.md` does. So they can be decoupled, and I suggest they are.

## 6. Standing constraint I have observed

**SPENT -- AND IT HELD, WHICH IS THE PART WORTH RECORDING.** This section read _I have executed none of this_ until 2026-09-03, by which time all three steps had run; that sentence is corrected here for the same reason the banner was.

**AS WRITTEN:** _I have not run `intent claude skills uninstall`, and I would not: it writes outside the repo into the user's `~/.claude/skills/` and `~/.intent/`, which is the user's environment rather than the project's. Step 2 is the one step in this document that needs an explicit human instruction rather than a peer's approval._

**THAT CONSTRAINT WAS HONOURED AND THEN HONOURED AGAIN.** hv ran step 2, not a node. And when `uninstall` left `in-start/` and `in-next/` behind (`0218`), **the residue was hv's to remove for exactly the same reason** -- a node found it, filed it, and did not reach into `~/.claude/` to tidy it. **A constraint that survives contact with an inconvenience is the only kind worth writing down**, and this one was tested by a defect nobody anticipated when it was drafted.
