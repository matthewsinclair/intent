# LLM-config rationalisation -- the gap analysis and the transformation plan, RECOVERED

**RECOVERED 2026-09-03 15:20Z BY vc FROM THE PRE-COMPACT TRANSCRIPT.** Both documents below were delivered to hv in chat on 2026-09-03 and written to no disk. vc's board named that as a gap in the 15:03Z fold -- _"if hv wants them durable they must be re-stated"_ -- and this file discharges it by RECOVERY rather than by re-statement, which is the stronger of the two: a re-statement is a second authoring and would drift from what hv actually read.

**PROVENANCE, so a later reader can check rather than trust.** Source is `~/.claude/projects/-Users-matts-Devel-prj-Intent/d6fb337d-3328-4360-865e-04ff4ba194e7.jsonl`, assistant text blocks at JSONL line indices 957 (gap analysis) and 1154 (transformation plan), extracted programmatically and pasted without editing. The transcript is per-session and per-machine; it is NOT a durable record, which is why this file exists.

**WHAT IS AND IS NOT VERBATIM.** The two fenced documents are the delivered text unaltered, INCLUDING their em dashes and their chat-register punctuation, which are not Intent house style. That is deliberate: house-styling a quotation makes it a copy. The pre-commit formatter will still normalise markdown emphasis and collapse interior double spaces across this whole file, because `.history/` is not in `.prettierignore` -- so **this file is content-faithful and is NOT byte-faithful, and no reader should cite it as bytes.** Widening `.prettierignore` to cover `.history/` may well be correct on that file's own stated criterion (single writer AND existing content is evidence), but it is a gated file with fleet reach and it is not a change to take on a general restart.

**A KNOWN DEFECT IN THIS FILE, DECLARED RATHER THAN FIXED.** It carries THREE `#` headings -- its own and one inside each quotation -- which breaks `IN-PR-STYLE-003` (one H1 per document). Demoting the quoted ones would edit the quotations to satisfy a rule about the container, and the container is an archive. Declared here so a critic hit on this path is a known exemption and not a new find.

**STATUS: UNEXECUTED.** Nothing below is built. The plan's own closing section names three decisions owed by hv, and all three were still owed when this was recovered. The target state that both documents refer to is the artefact at https://claude.ai/code/artifact/d3da2edf-23fd-42b1-bc92-05c33bf1a3a8 -- twelve invariants, three mechanisms, approved by hv 2026-09-03.

**ONE CORRECTION TO vc's OWN BOARD.** The board and the fold both call this a _five-phase_ plan. It has SIX phases, Phase 0 through Phase 5. Phase 0 is the one-line fix that may ride 3.0.1 and the other five are the programme, so _five phases plus a phase zero_ is a fair reading of how it was described -- but the board's number is wrong as written and is corrected here rather than quietly.

**WHERE THIS GOES NEXT.** When hv rules on the steel thread, this content moves into that thread and this file becomes a dated record rather than a live claim. Until then it is the only durable home.

---

## DOCUMENT 1 -- GAP ANALYSIS, as delivered

Gap analysis complete. The headline is better than I expected: **the hard part is already built.**

---

# Gap analysis — target vs. now

## A. Already works — do not rebuild

| what                                     | evidence                                                                                                                           |
| ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| The rule library                         | `intent/plugins/claude/rules/**` with a typed API: `Library::rules()`, `Library::show(id)`                                         |
| `Rule` carries everything an index needs | `id`, `language`, `category`, `severity`, **`title`**, `provenance`, `path`                                                        |
| The critics                              | thin, read the library at invocation — **they never forked while every hand-kept index did.** That's the proof the model works     |
| The user-owned distinction               | `seed_if_absent` draws the line correctly                                                                                          |
| CLAUDE.md user region                    | `<!-- user:start -->` exists and works                                                                                             |
| The backup                               | D35 rolling snapshots in `intentsvcs::backup`                                                                                      |
| Report-then-apply                        | `intent claude upgrade` already reports by default and writes on `--apply` — that _is_ the status/sync split, for one artefact set |

**`Rule.title` is the gloss.** So the derived index is `Library::rules()` → filter `agnostic` → emit `id + title`. Against an existing typed API.

## B. Exists, wired to the wrong input — the crux

`intent agents generate` renders `AGENTS.md` and emits **exactly 4** rule ids. It reads `lib/templates/llm/_AGENTS.md` — a hand-kept template — not the library.

**Nothing in the codebase derives an index from the library. Zero call sites.**

So the generator exists, the render path exists, the library API exists, and **the wire between them is the missing piece.** That's the highest-value change in the whole programme and it's small.

## C. Exists but deliberately unwired

`agents generate` is emit-only. `render.rs:8216` states it plainly: _"generate is wired and its four siblings are not, and that is a chosen boundary rather than partial delivery"_ — because wiring the write half would have picked between two backup mechanisms by accident.

**You made that call. D35.** The stated blocker is gone.

## D. Doesn't exist

- The `intent llm` family (`status` / `diff` / `sync`)
- Index derivation from the library
- Provenance headers on generated files
- **The coherence check** — does any canon file reference a file that doesn't exist. This makes invariant 7 enforceable, and **it goes red today**: fresh `CLAUDE.md` references `AGENTS.md` four times and init doesn't create it
- Block-scoped diff for user-owned files (`usage-rules.md`, `.intent_critic.yml`)

## E. Must be deleted

`intent/llm/RULES.md` and `RULES-<lang>.md` · the four forked language templates · `_RULES.md` and `_ARCHITECTURE.md` templates (9 lines each — they exist only to seed the directory that's going). `_ARCHETYPES.md` and `_DEPENDENCY_GRAPH.md` have **one source reference each** and are likely dead — I haven't confirmed whether that single ref is real wiring or a mention.

## F. Wrong content a sync would propagate today

| template                      | ids | asserts "four" |
| ----------------------------- | --- | -------------- |
| `_AGENTS.md`                  | 4   | yes            |
| `_CLAUDE.md`                  | —   | yes            |
| `_RULES.md`                   | —   | yes            |
| `templates/_default/RULES.md` | 4   | yes            |

## The one ordering constraint

**Derive the index before shipping sync.** In that order, sync distributes a correct index to the fleet. In the other order, it distributes the current wrong one — faithfully, to every project, reporting green. Consistency around a wrong answer is worse than drift, because drift leaves one project accidentally right.

## Shape

| piece                             | size  |
| --------------------------------- | ----- |
| Index derivation from the library | **S** |
| Wire the `agents sync` write half | **S** |
| Provenance headers                | **S** |
| Coherence check                   | **S** |
| Template content repair           | **S** |
| `intent llm` verb family          | **M** |
| Deletions + migration prune       | **M** |
| Fresh-init completeness           | **M** |
| **Total**                         | **L** |

Not XL — because A, B and C mean most of the machinery is already standing.

## Scope I did not cover

`intent claude skills sync` reports _"4 need a decision"_ on every run and **I have not measured the four** — seeing the message requires a `sync`, which is a write, and that's on your list rather than mine to take.

---

Ready to build the transformation plan against this whenever you want it. I'd sequence it so the index derivation and the template repair land together as one commit — that's the pair that makes everything downstream safe.
---

## DOCUMENT 2 -- TRANSFORMATION PLAN, as delivered

**The recovered block opens with two paragraphs closing out an UNRELATED thread** -- lamplight-cc's refutation, and their reason for not banking it as a fourth watch-out. They are kept because they were in the delivered message and trimming a quotation to fit its heading is how a quotation stops being one. The plan proper begins at `# Transformation plan`.

lamplight-cc verified the refutation in our tree and closed out. One detail worth your attention: they deliberately **didn't** bank it as a new watch-out — their board already carries the class three ways, and a fourth restatement _"would be the exact duplication I spent today folding out of that file, and it would make the class less findable, not more."_ That's Highlander applied to a watch-out board, which is the thing we're designing, arriving unprompted.

Thread closed. Here's the last deliverable.

---

# Transformation plan

## Framing first: this does not ride v3.0.1

Your scope ruling is explicit — ST0056 and its feeders, nothing wider. This is a new steel thread that lands **after the tag**. One exception below.

## Phase 0 — one line, rides 3.0.1 as a defect

`in-standards/SKILL.md:18` states a writer that v3 retired, in canon and in every installed copy, in the skill every session loads at boot. **XS.** It's a shipped false statement that has already caused wrong work in another estate; it shouldn't wait for a programme.

## Phase 1 — the source of truth _(one commit, and the ordering is load-bearing)_

- Derive the agnostic index from `Library::rules()` — `id` + `title`
- Emit it into `_AGENTS.md` and `_CLAUDE.md` at generation time
- **Strip the hardcoded index from every template in the same commit**
- Drift test for `usage-rules.md`'s shared block
- Enforcement: every `IN-*` reference in a canon file is an id the library serves

**Why one commit:** a derived index plus a template that still hardcodes one is two homes again. And **derivation must precede sync** — in the other order, sync distributes the current wrong index to ten estates, faithfully, reporting green.

**Size: S–M.** Small because `Library::rules()` exists and `Rule.title` is the gloss.

## Phase 2 — the manifest and the verbs

- Declare the manifest: path → disposition (`derived` / `tested` / `user-owned` / `retired`) → writer verb
- **Writer enumeration from source** — the orphan test is _does a verb write this_, per invariant 5
- `intent llm status` — managed state, undeclared files, and writer verification
- `intent llm diff`, `intent llm sync` (D35 backup; block-scoped for user-owned)
- Wire the `agents sync` write half — your D35 call removed the stated blocker

**Size: M.**

## Phase 3 — provenance and coherence

- Provenance header on every derived file, in the **anchored** form `views.rs` already parses
- `doctor` coherence arm: no canon file references a file that doesn't exist

**This arm goes red the day it lands** — fresh `CLAUDE.md` references `AGENTS.md` four times and init doesn't create it. That's deliberate: it's the failing test Phase 5 closes. Fittingly, the discipline is `IN-AG-RED-CONTROL-001` — **one of the two rules the programme's own defect hid from every index.**

**Size: S.**

## Phase 4 — the prune

Delete `intent/llm/RULES*.md`, the four forked language templates, and the dead templates. Migration prunes them in consumer estates, fail-forward. **Adopt-or-delete, never stamp-on-write.**

**Size: M.**

## Phase 5 — fresh init completeness

`init` lays down the full canon set including `AGENTS.md`. Phase 3's coherence arm goes green.

**Size: M.**

---

## Cross-cutting

**Every instrument built here ships both controls** — a positive it must find, a negative it must not match. Five broken probes returned clean zeros across two estates in one morning; that's the discipline that stops the sixth.

## Canary

**Lamplight.** It has the worst case — twenty unmarked files spanning three genuine dispositions — an active vc who's already offered to test, and it's the estate whose experience shaped the design. ST0035's fleet rollout surfaced three canon-installer rough edges through canary discipline; this has more surface than that did.

## Three things I need from you

1. **A steel thread** — I'd propose one rather than create it, since `st new` is a write and the scope is yours.
2. **Verb naming** — `intent llm` vs something else. Still your call.
3. **Whether Phase 0 rides 3.0.1** or waits with the rest.

Total: **L**, unchanged. The hard parts were already standing before we started.
