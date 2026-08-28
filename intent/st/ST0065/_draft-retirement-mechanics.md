# DRAFT -- retirement mechanics: `in-start`, `in-next`, `_wip.md`

**DRAFT ONLY. NOTHING HERE HAS BEEN EXECUTED.** Written 2026-08-28 16:35Z by ic, for vc, under hv's 2026-08-28 ruling that `in-next` retires and hv's adoption of the WP-01/WP-02 verdicts. **No file has been deleted, no skill uninstalled, no template changed.** This document exists so the retirement is sequenced by someone other than the person who proposed it.

## 0. The finding that governs everything below

**A canon skill retirement is a THREE-LOCATION lifecycle operation, and deleting the canon file is the LAST step rather than the operation itself.**

The reason is mechanical and I measured it rather than assumed it:

**`sync` never prunes a skill whose canon source has vanished.** `skills.rs:690-697` -- when `origins()` yields nothing, sync pushes `Outcome::SourceMissing` and `continue`s. It does not remove the installed tree and it does not remove the manifest entry.

**And `intent upgrade` cannot rescue that**, because upgrade's skill step _is_ that sync: `bin/intent_upgrade:202` runs `intent claude skills sync || true`.

**So a canon-only delete strands the skill on every machine that has it, permanently, and every future sync reports `SourceMissing` at it forever.** The skill would keep loading into sessions -- `~/.claude/skills/` is what Claude Code reads, not canon -- so the retired skill would go on being _invoked_ while the project believed it was gone.

That is the whole reason this is a draft and not a commit.

## 1. Measured mechanics

| Fact                                      | Evidence                                                                                                                                                                               |
| ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| sync does not prune a vanished source     | `skills.rs:690-697`, `Outcome::SourceMissing` then `continue`                                                                                                                          |
| upgrade's skill step is that same sync    | `bin/intent_upgrade:202`                                                                                                                                                               |
| `uninstall` exists and is the only pruner | `intent claude skills uninstall <NAME>...` resolves, rc=0                                                                                                                              |
| **uninstall does NOT read canon**         | `skills.rs:846+` -- manifest + installed dir only. **So ordering is free: it works before or after the delete**                                                                        |
| uninstall removes only what it wrote      | ruling 5 (vc, 2026-08-22): _a sync may remove what it INSTALLED; it may not remove what it FOUND_                                                                                      |
| **ruling 5's precondition is MET here**   | both entries record `files: ['SKILL.md']` in `~/.intent/skills/installed-skills.v3.json`. A v2 entry has no file list and would remove nothing; these are v3 entries and prune cleanly |
| baseline is clean today                   | canon 25, installed 25, **zero orphans in either direction**                                                                                                                           |

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

**Note what is NOT on this list:** `lib/templates/llm/_usage-rules.md` is clean. Only Intent's own root `usage-rules.md` carries those rows, so **the fleet template needs no change** and no downstream project inherits the dangle.

## 3. Proposed order

Sequenced so that **no intermediate state has a live pointer to a missing target**, and so the destructive step is last.

1. **Edit the six live references** (sites 1, 2, 5, 6, 7, 8). After this step the two skills are unreferenced but still present and still working -- a safe resting state that can sit indefinitely.
2. **`intent claude skills uninstall in-start in-next`** -- prunes the installed copies and the manifest entries. Removes exactly the recorded `SKILL.md` from each, leaves anything an operator dropped in, prunes the emptied dirs.
3. **Delete `intent/plugins/claude/skills/in-start/` and `in-next/` from canon.**
4. **Verify:** canon count and installed count agree and are both 23; a `sync` reports no `SourceMissing`; the reference grep returns zero live hits with the control still returning 94 for `in-session`.

**Steps 2 and 3 are order-free** because uninstall does not read canon -- but doing 2 first means no step ever leaves an orphan, which is worth more than the flexibility.

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

**I have executed none of this.** In particular I have not run `intent claude skills uninstall`, and I would not: it writes outside the repo into the user's `~/.claude/skills/` and `~/.intent/`, which is the user's environment rather than the project's, and the project is deliberately holding delivery for cc's batch. **Step 2 is the one step in this document that needs an explicit human instruction rather than a peer's approval.**
