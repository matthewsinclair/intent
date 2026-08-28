# WP-02 catalogue: the `/in-*` skills

**Read-and-catalogue leg. NOTHING HAS BEEN EDITED.** Verdicts are recommendations to hv, per vc's assignment (14:32Z). Same vocabulary as WP-01: **keep** / **correct** / **retire** / **MISGUIDED**.

**COVERAGE, STATED HONESTLY.** 25 skills. A mechanical sweep covers all 25 (verb resolution, load path, chain integrity, script presence, cross-references, duplication). A close read covers 18 -- the 8 auto-loaded into this session plus `in-start`, `in-next`, `in-finish`, `in-plan`, `in-verify`, `in-debug`, `in-review`, `in-tca-init`, `in-autopsy` and `in-cost-analysis`. **The remaining 7 have been swept, not read** (`in-ash-ecto-essentials`, `in-detrope`, `in-phoenix-liveview`, and four of the five TCA skills), and no verdict below is offered on a skill I have only swept.

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

**It also carries stale content the retirement did not reach.** Its step 2 -- "Read project rules: `CLAUDE.md`, `intent/llm/MODULES.md`, `intent/llm/DECISION_TREE.md`" -- is **verbatim `in-standards` step 1**, including the retired MODULES.md. That makes it one of twenty instruction sites for a file `init` refuses to create (WP-01 finding 2).

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

**COUNT CORRECTED.** The table above lists the sites I found with a path-qualified grep (`llm/MODULES.md`). Re-measured with the bare pattern: **20 mentions across 12 files**, including 4 in `in-plan`, 3 in `lib/templates/prime/operational-knowledge.md`, 1 in `in-review`, and **1 each in two Elixir archetype templates -- so generated modules carry the reminder too.** See WP-01 finding 2 for the full table and for the note on how the first count went wrong.

**FILED AS ISSUE 0122** on vc's ask (14:57Z) -- a live CLI defect independent of this thread, so the catalogue cites it rather than carrying it alone. The issue body leads with **FIX THE STRING, NOT THE EXIT CODE** and scopes itself to the two CLI strings plus the hook message that routes into them; the twenty documentation sites stay here, as ST0065's to catalogue and hv's to rule on.

**Verdict: CORRECT.** One remedy string (0122), plus twenty instruction sites. The verbs' rc=1 is not a defect -- **noting that explicitly, here and in the issue, so nobody "fixes" the wrong half.**

---

## 3b. `in-finish` step 4 rebuilds the duplication the 08-24 fold removed. **CORRECT (third instance of the theme)**

**What `in-finish` is, first: the best-maintained skill in the set.** It is current with v3 -- localfold/globalfold scopes, the `acceptance.md` close-gate, `intent ac descope | withdraw` as the two honest exits, `intent at lint`. Nothing stale in it. **KEEP.**

**The defect is one instruction inside it.** Step 4 says:

> Rewrite `.claude/restart.md` with: **WIP/TODO focus** for Claude Code startup; concise pointers to current work

**`.claude/restart.md` now opens by declaring it holds no such thing:**

> _"This file is the ENTRY POINT and nothing else. The state lives in `intent/wip.md` (current work) and `intent/restart.md` (narrative + traps + conventions). **It was three copies of one narrative until 2026-08-24**, each opening with a banner saying it superseded everything below it -- which is the tell that nobody was deleting, only prepending... **If you find yourself writing a supersedes banner, DELETE WHAT IT SUPERSEDES INSTEAD.**"_

**"WIP/TODO focus" is state, and state is exactly what the file was folded to stop carrying.** The 2026-08-24 fold repaired the three artefacts; the instruction that produced them runs unchanged at every session wrap. The file has to carry a warning banner **because** the skill that rewrites it still says otherwise -- the artefact is defending itself against its own generator.

**Verdict: CORRECT.** One instruction in an otherwise excellent skill. Note the general principle is already written down in the right words -- it is just written in the artefact instead of in the skill that would prevent the problem.

**This settles the `.claude/restart.md` question left open below:** the file is wanted, its role is entry-point-only, and `in-finish` step 4 is what needs the edit -- not the file, and not the four skills that merely read it.

---

## 3c. `in-plan` is the densest MODULES.md site in the corpus. **CORRECT**

**Four mentions -- more than any other file**, and they are load-bearing rather than incidental: step 2 _"check MODULES.md first"_ and _"register in MODULES.md first"_; step 5 _"Highlander Rule: No duplicated code paths. Check MODULES.md."_; and a Red Flags row answering _"I already know the codebase"_ with _"Check MODULES.md anyway. Memory drifts."_

**The skill whose entire job is to be followed before any code is written is the densest site of the one instruction that cannot be followed.** On a fresh v3 project all four route to a file `init` refuses to create.

**A second, smaller thing in the same skill.** Step 5 opens _"These rules apply to ALL languages (Elixir, Rust, Swift, Lua)"_, but step 4's list of skills to load offers only `/in-essentials` and four Elixir ones. A Rust or shell project following step 4 loads nothing for its own language -- which is correct, because no such essentials skill exists, but the skill does not say so and the adjacent sentence implies otherwise.

**Verdict: CORRECT.** Both are text fixes, and `in-plan`'s plan-quality standards (no placeholders, specific file paths, small steps, verification per step) are good and should survive untouched.

---

## 3d. The workflow core is current and healthy. **KEEP, all four**

Recorded because a review that only lists defects misrepresents the set.

- **`in-verify`** -- the strongest skill in the corpus. Fresh evidence per claim, verification in the current message ("context compaction may have removed prior evidence"), and fully current with v3: red-first ATs via `intent at red` / `green`, the close-gate, and the enforced AT citation grammar.
- **`in-finish`** -- see 3b. Current throughout; one instruction to fix.
- **`in-debug`** -- four phases plus the 3-Strike Rule ("three failed fixes usually means you are solving the wrong problem"). Language-agnostic, nothing stale.
- **`in-review`** -- current with the v3 language model: reads the `languages` array, dispatches per-language critics, handles prose disciplines and polyglot subtrees, honours `.intent_critic.yml`. Carries one MODULES.md mention in its agnostic checklist (part of the class above).

---

## 4. Verified negative -- do not re-find these

- **Every `intent` verb the skills instruct exists.** 30 distinct invocations extracted across all 25 SKILL.md files; all 30 resolve, top-level and subcommand, checked via `--help` rather than by driving mutating verbs. **The instrument was positive-controlled** -- an injected `intent frobnicate` fires it -- because a clean sweep from an untested checker is worth nothing.
- **The TCA family is intact and costs no session budget.** 5 skills / 34747 bytes, all on-demand; `intent/docs/total-codebase-audit.md` (60008 bytes) present; all three scripts present. Live documented feature, not orphaned. **KEEP.**
- **`Red Flags` tables in 15 of 25 skills** are a house convention, not duplication. No verdict.
- **Every `chains_to` target resolves**, and every script and data file the skills reference is present. The TCA family chains as a coherent pipeline: `init -> audit -> synthesize -> remediate -> finish -> in-finish`.
- **`in-cost-analysis` does NOT contradict the T-shirt-sizing rule.** It emits hours and rates, and `CLAUDE.md:77` bans clock-time estimates -- but that rule governs **sizing project work** (ST/WP scope), while this skill does **retrospective valuation of an existing codebase**. Different activities. **Recorded so the next reviewer does not file it**, because it looks like a conflict and is not one.
- **`in-autopsy`'s Elixir dependency is disclosed.** Its implementation is `autopsy.exs` and would not run in a non-Elixir project; the skill says so at line 145 (_"requires Elixir to be installed"_). Handled, not a defect.

---

## 5. Open, not concluded

- **13 skills swept but not read** (`in-finish` has since been read -- finding 3b). No verdict offered on them; the mechanical sweep found nothing dangling in any.
- **`.claude/restart.md` is RESOLVED** -- see finding 3b. The file is wanted and entry-point-only; `in-finish` step 4 is the site that needs the edit. It is absent from a fresh project because it is created at the first globalfold rather than at init, which is coherent. `in-essentials` rule 5 also instructs updating it and should be read against 3b before either is changed.

---

## Summary

| #   | Item                                                                            | Verdict                            |
| --- | ------------------------------------------------------------------------------- | ---------------------------------- |
| 1   | `in-start` -- second session-start skill, competes with the enforced one        | **RETIRE**                         |
| 2   | `in-next` -- procedural scaffolding for default behaviour                       | **RETIRE (candidate, hv's call)**  |
| 3   | MODULES.md remedy string in `modules check` / `find`, plus 20 instruction sites | **CORRECT**                        |
| 3b  | `in-finish` step 4 rewrites state into the entry-point-only file                | **CORRECT** (skill otherwise KEEP) |
| 3c  | `in-plan` -- 4 MODULES.md mentions, the densest site in the corpus              | **CORRECT** (plan standards KEEP)  |
| 3d  | `in-verify`, `in-finish`, `in-debug`, `in-review` -- the workflow core          | **KEEP, all four**                 |
| 4   | TCA family, verb resolution, chain integrity, Red Flags convention              | **KEEP / verified negative**       |
| 5   | `in-whiteboard` at 35% of session budget                                        | no defect; budget fact for hv      |

**The theme matches WP-01's, and `in-finish` step 4 is its clearest instance.** Every defect here is a document or a string that outlived the mechanism it describes -- a skill duplicating the entry point the gate now enforces, a remedy naming a canon behaviour retired four days ago, and a wrap-up step rebuilding the duplication a fold removed on 08-24. **Nothing found in this half is wrong about how to code; the errors are all about what the tool currently is.**

**THREE RETIREMENTS, EACH FIXED ON ONE SIDE ONLY.** `init` stopped writing MODULES.md and twenty instructions still name it. The 08-24 fold collapsed the three restart files and `in-finish` step 4 still rebuilds them. The `UserPromptSubmit` gate became the session entry point and `in-start` still offers a second one. **Worth hv seeing as one decision rather than three, because the remedy is identical in every case: when a mechanism is retired, the instructions that drive it are part of the retirement, not a follow-up.**

**AND THE COUNTING ITSELF IS EVIDENCE FOR THAT.** The MODULES.md figure went 4 -> 6 -> 20 across this review because my first two greps were fitted to the surface form I had already seen. **A retirement that leaves its instructions running is hard to size precisely because the survivors are scattered across templates, skills, hooks, code generators and the CLI's own strings** -- no single grep shape finds them, and each partial count reads like a complete one.
