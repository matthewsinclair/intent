# WP-02 catalogue: the `/in-*` skills

**Read-and-catalogue leg. NOTHING HAS BEEN EDITED.** Verdicts are recommendations to hv, per vc's assignment (14:32Z). Same vocabulary as WP-01: **keep** / **correct** / **retire** / **MISGUIDED**.

**COVERAGE, STATED HONESTLY.** 25 skills. A mechanical sweep covers all 25 (verb resolution, load path, cross-references, duplication). A close read covers 11 -- the 8 auto-loaded into this session plus `in-start`, `in-next` and `in-tca-init`. **The remaining 14 have been swept, not read**, and no verdict below is offered on a skill I have only swept.

---

## 0. The load-path split -- where the budget actually goes

|                                                             | count | bytes     |
| ----------------------------------------------------------- | ----- | --------- |
| **Auto-loaded** by `/in-session` (this project's languages) | 8     | **62094** |
| On-demand (invoked by name or chained)                      | 17    | 78673     |

**Only the first row is a per-session cost.** The 17 on-demand skills are free until invoked, which settles several questions before they are asked -- notably the TCA family (finding 5).

**`in-whiteboard` alone is 30284 bytes -- 49% of the auto-loaded skill bytes and ~35% of the whole session budget** (WP-01 finding 0). It is larger than CLAUDE.md, AGENTS.md and usage-rules.md combined. It auto-loads whenever `intent/whiteboard/` exists, so the cost is opt-in by directory presence and zero for a project without a board. **No defect found in it; this is a budget fact for hv, not a verdict.**

---

## 1. `in-start` -- a second session-start skill. **RETIRE**

**What it is:** "Session start: read restart files, review STs, orientation overview before coding." 1304 bytes, on-demand, `chains_to: [in-plan, in-next]`.

**Why it is a defect and not merely redundant.** The canon mandates one session entry point: `CLAUDE.md` says _"Run `/in-session` immediately after session start and after every `/compact` or context reset"_, and the `UserPromptSubmit` gate enforces it by blocking the first prompt. `in-start` is a **second, unenforced, competing** answer to the same question, and nothing chains to it.

**It also carries stale content the retirement did not reach.** Its step 2 -- "Read project rules: `CLAUDE.md`, `intent/llm/MODULES.md`, `intent/llm/DECISION_TREE.md`" -- is **verbatim `in-standards` step 1**, including the retired MODULES.md. That makes it instruction site #5 for a file `init` refuses to create (WP-01 finding 2).

**Verdict: RETIRE.** Its orientation content is either already in `/in-session` (which reads languages, loads skills, releases the gate, chains whiteboard pickup) or is the model's default behaviour. Two session-start skills is a Highlander violation in the workflow the project most wants to be deterministic.

---

## 2. `in-next` -- scaffolding for what the model does anyway. **RETIRE (candidate -- hv's call)**

**What it is:** 1199 bytes, on-demand, `chains_to: [in-plan]`. Four steps: review current state, identify the smallest coherent work unit, describe it in detail, wait for instructions.

**The test I am applying is the vendored source's own.** Cherny's recurring point -- _"There's nothing in the system prompt about looking through git history. It knows because the model is good"_ -- is the yardstick for whether a procedural skill earns its bytes. "Review state, pick the smallest next unit, describe it, wait" is default competent behaviour, not a technique the model needs taught. And step 4 ("Do not start coding") restates a discipline `in-start` and `in-plan` both already assert.

**Why this verdict is softer than finding 1.** `in-next` is on-demand, so it costs nothing unless someone invokes it, and it is not _wrong_ -- nobody is led anywhere bad by following it. It is redundant, not misguided. **Retire on grounds of "makes no sense any more" (hv's phrase) rather than on cost or correctness -- and that is a judgement about how the project wants to work, so it is hv's, not mine.**

---

## 3. The MODULES.md class reaches the CLI, not just the docs. **CORRECT**

WP-01 finding 2 counted documentation sites. The sweep found the class extends further:

| Site                                           | Kind                                                          |
| ---------------------------------------------- | ------------------------------------------------------------- |
| `_CLAUDE.md`, `_AGENTS.md`                     | canon templates (WP-01)                                       |
| `in-standards` SKILL.md x2                     | auto-loaded skill (WP-01)                                     |
| `in-start` SKILL.md                            | on-demand skill (**new**, finding 1)                          |
| `lib/templates/hooks/module_check_hook.json`   | **a hook** telling the operator to run `intent modules check` |
| `intent modules check` / `intent modules find` | **the CLI itself**                                            |

**Driven on the fresh-init fixture.** Both verbs fail correctly and loudly -- **true rc=1**, verified by capturing to a file rather than reading a pipeline's exit code. **That behaviour is right and should be kept.** What is wrong is the remedy they print:

> `remedy: intent/llm/MODULES.md is the module registry **the canon lays down**. Run `intent upgrade` if this project **predates** it, or create the file if this project has never carried one.`

**Both clauses are false on a project created seconds earlier by `intent init`.** The canon stopped laying it down on 2026-08-24 (`NotByInit`, "a hand-maintained index of a tree the store already indexes"), and the project predates nothing -- `intent upgrade` will not create it either, for the same reason. The remedy routes an operator to a command that cannot help, and it names the canon as the authority for a behaviour the canon deliberately ended.

**Verdict: CORRECT.** One remedy string, plus the five instruction sites. The verbs' rc=1 is not a defect -- **noting that explicitly so nobody "fixes" it.**

---

## 4. Verified negative -- do not re-find these

- **Every `intent` verb the skills instruct exists.** 30 distinct invocations extracted across all 25 SKILL.md files; all 30 resolve, top-level and subcommand, checked via `--help` rather than by driving mutating verbs. **The instrument was positive-controlled** -- an injected `intent frobnicate` fires it -- because a clean sweep from an untested checker is worth nothing.
- **The TCA family is intact and costs no session budget.** 5 skills / 34747 bytes, all on-demand; `intent/docs/total-codebase-audit.md` (60008 bytes) present; all three scripts present. Live documented feature, not orphaned. **KEEP.**
- **`Red Flags` tables in 15 of 25 skills** are a house convention, not duplication. No verdict.

---

## 5. Open, not concluded

- **14 skills swept but not read.** No verdict offered on them; the mechanical sweep found nothing dangling in any.
- **`.claude/restart.md`** is written by 4 skills (`in-essentials`, `in-finish`, `in-tca-finish`, `in-start`) and is **absent from a fresh project**. `in-start` handles this ("skip any that don't exist"); `in-essentials` rule 5 instructs updating it unconditionally. Flagged, not yet judged -- it needs a read of `in-finish` to say whether the file is still wanted at all.

---

## Summary

| #   | Item                                                                     | Verdict                           |
| --- | ------------------------------------------------------------------------ | --------------------------------- |
| 1   | `in-start` -- second session-start skill, competes with the enforced one | **RETIRE**                        |
| 2   | `in-next` -- procedural scaffolding for default behaviour                | **RETIRE (candidate, hv's call)** |
| 3   | MODULES.md remedy string in `modules check`/`find`                       | **CORRECT**                       |
| 4   | TCA family, verb resolution, Red Flags convention                        | **KEEP / verified negative**      |
| 5   | `in-whiteboard` at 35% of session budget                                 | no defect; budget fact for hv     |

**The theme matches WP-01's.** Every defect here is a document or a string that outlived the mechanism it describes -- a skill that duplicates the entry point the gate now enforces, and a remedy that names a canon behaviour retired four days ago. **Nothing found in this half is wrong about how to code; the errors are all about what the tool currently is.**
