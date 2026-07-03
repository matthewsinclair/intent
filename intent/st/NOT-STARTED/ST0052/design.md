# Design - ST0052: Author project-type pack

## Approach

Reuse Intent's existing "language pack" mechanism to carry a non-code discipline. A pack is four assets -- a canon template, a rule library, a critic subagent, and an essentials skill -- activated per project by the `languages` array in `intent/.config/config.json` and loaded by `/in-session`. The `author` pack is the first non-code instance; building it establishes the pattern a later `content` pack copies. Delivery is phased so the one schema-evolution step (a new `AU` language code) lands first and everything else builds on it. Two hv refinements shape the critic: a two-form detrope integration (D5) and `languages`-scoped critic dispatch (D7). See tasks.md for the work-package breakdown.

## Design Decisions

### D1 -- Reuse the `languages` axis (ratified: Approach A)

Model `author`/`content` as more packs on the existing axis, not a parallel `domain:` field. Rationale: there is ONE pack-activation mechanism (rules + critic + skill + canon); a second axis would duplicate it (Highlander smell). Single-axis also composes -- a courseware repo can legitimately be `languages: [elixir, author]`, loading both packs, which a `language xor domain` taxonomy cannot express. The word "language" is stretched, not the mechanism; softening the vocabulary (Approach B) is a deferred follow-up, not a blocker (see Alternatives).

### D2 -- New language code `AU` (RATIFIED 2026-07-03)

"Reuse the `languages` axis" is open at the canon-template/config/`in-session` layer (`intent lang init author` needs only a `templates/author/` dir -- verified in `bin/intent_lang`, no allowlist). It is CLOSED at the rule-library/critic/ID layer: the language code is a fixed enum. Admitting `author` as a first-class pack means adding code `AU` (language `author`) to the ID/validation layer. v1 sites (the headless-runner site is deferred with D4):

| Site                                                    | What changes                                          |
| ------------------------------------------------------- | ----------------------------------------------------- |
| `rules/_schema/rule-schema.md`                          | `language` enum gains `author`                        |
| `rules/_schema/id-scheme.md:155` + codes table          | language-codes table gains `AU`; regex doc gains `AU` |
| `rules/_schema/index-generator.md:128`                  | duplicate regex doc gains `AU`                        |
| `intent/plugins/claude/bin/intent_claude_rules:161-162` | validator regex (check + error string) gains `AU`     |

The regex `^IN-(AG\|EX\|RS\|SW\|LU\|SH)-...` is duplicated across those four files (a mild pre-existing Highlander wrinkle; WP01 updates all copies consistently rather than refactoring the duplication now). `bin/intent_doctor` carries no language allowlist (checked -- nothing there). `bin/intent_critic` (the headless pre-commit runner) is NOT touched in v1 -- it is the deferred headless-prose-gate follow-up (D4).

### D3 -- Two-tier critic: mechanical (default) vs judgment (on instruction)

Author rules split by how they are detected:

- **Mechanical tier (default)** -- simple greppable Detection (banned filler, `e.g.`-not-`eg`, vanity metrics, front-matter/objective presence, heading hygiene, and the mechanical trope pass -- D5). Runs on every `critic-author` review. Satisfies the ST0039 strict-proxy contract, so it can later drive a headless gate.
- **Judgment tier (on instruction)** -- critic-as-reader (voice/register consistency, cross-chapter continuity, show-don't-tell) and the full LLM detrope (D5). No greppable proxy; runs only when directly invoked.

### D4 -- v1 is on-demand + detrope only; headless prose gate deferred (RATIFIED 2026-07-03)

`bin/intent_critic` routes staged files to a critic by EXTENSION -> language (`elixir:*.ex`). Prose is `.md`, which overlaps docs and a future `content` pack -- extension alone cannot route a `.md` file to `author`. v1 ships the on-demand `Task(critic-author)` + detrope only; the headless `intent critic author` pre-commit gate (needing path-based file-selection, eg an `author_paths:` in `.intent_critic.yml`) is a follow-up. This keeps `bin/intent_critic` untouched in v1.

### D5 -- Two-form detrope integration (hv refinement)

critic-author integrates detrope in two forms:

- **Mechanical (default)** -- a greppable trope pass runs on every review. This surface does NOT exist in Intent yet: `in-detrope` ships `SKILL.md` + `data/trope-catalog.md` (the LLM skill + the 44-trope catalogue) only; the greppable indicators live in Utilz (`indicators.txt`), not here. So v1 builds a mechanical trope-indicator surface in Intent, sourced Highlander from the shared `llm-tropes` upstream (the same upstream `trope-catalog.md` is vendored from). Form TBD in WP02/WP03: either ⚙ author-pack rule Detection heuristics, or a vendored `data/trope-indicators` file critic-author greps. No trope catalogue is re-authored -- only the mechanical detection layer is added.
- **Full LLM (on direct instruction only)** -- the existing `/in-detrope` skill (contextual, stylometric diagnosis). Per the critic contract, critic-author does NOT invoke the skill itself; by default it emits a handoff recommendation (the diogenes-handoff pattern -- "run `/in-detrope` for full trope diagnosis"), and the human / top-level agent runs it under direct instruction. detrope stays the single home for trope knowledge; the author pack references it, never forks it.

### D6 -- Author critic categories/modes diverge from code critics

Code critics use `review`/`test-check` modes mapping to `rules/<lang>/code|test`. Authoring has no "tests"; `critic-author` defines its own categories (proposed: `style` mechanical + `craft` judgment) and its own mode verbs, loading `rules/agnostic/*` + `rules/author/*`. The critic contract is a template each `agent.md` specialises -- this divergence is allowed and documented here.

### D7 -- `languages`-scoped critic dispatch (hv refinement -- mostly already holds)

An author project must not run the code critics by default; a project that is both code and author runs both. This is ALREADY the architecture: `/in-review` reads `languages` from config and "for each language listed, dispatch to its critic subagent ... if `languages` is empty, no language-specific critic runs." So an author-only project (`languages: [author]`) already will not dispatch `critic-elixir`/`critic-shell`/etc -- the exclusion falls out for free. The only work: add the `author -> critic-author` mapping to the `/in-review` dispatch list (+ a Task example) and the `author` row to the `/in-session` fan-out. A mixed `[elixir, author]` dispatches both, each narrowed to its own subtree (in-review's existing polyglot rule). This is the concrete payoff of Approach A over a separate `domain:` axis.

## Architecture

### Pack anatomy (the four assets)

| Asset          | Location                                                                 | Role                                                                                   |
| -------------- | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| Canon template | `intent/plugins/agents/templates/author/RULES.md` (+ `ARCHITECTURE.md`)  | copied to `intent/llm/RULES-author.md` by `intent lang init author`                    |
| Rule library   | `intent/plugins/claude/rules/author/<category>/<slug>/RULE.md`           | Detection heuristics; served by `intent claude rules`; read by the critic              |
| Critic         | `intent/plugins/claude/subagents/critic-author/{agent.md,metadata.json}` | prose/pedagogy reader (Read/Grep/Bash; no Write); two-tier (D3); two-form detrope (D5) |
| Skill          | `intent/plugins/claude/skills/in-author-essentials/SKILL.md`             | the authoring pipeline; loaded by `/in-session` when `author` is declared              |

`rules/author/index.json` entries are generated, not hand-authored (regenerate after adding rules). Registration: `subagents/.manifest/global-agents.json`.

### Proposed starter rule set

`worn` = mechanical/default; `read` = judgment/on-instruction. Numbering illustrative (WP02 assigns final `IN-AU-*` IDs).

| Rule                                                      | Category | Tier | Harvested from                          |
| --------------------------------------------------------- | -------- | ---- | --------------------------------------- |
| banned filler + `eg`-not-`e.g.` + house style             | style    | worn | global `CLAUDE.md`                      |
| no vanity metrics; T-shirt sizing in prose                | style    | worn | global `CLAUDE.md` + Intent memory      |
| chapter/module front-matter + learning objectives present | style    | worn | (courseware convention)                 |
| heading hygiene (one H1, no skipped levels)               | style    | worn | (shared with a future content pack)     |
| mechanical trope pass (greppable tells)                   | style    | worn | `llm-tropes` -> new Intent surface (D5) |
| full trope / stylometry diagnosis                         | craft    | read | -> `/in-detrope` handoff (D5)           |
| voice / register consistency (no marketing drift)         | craft    | read | critic-as-reader                        |
| cross-chapter continuity (terms, no contradictions)       | craft    | read | critic-as-reader                        |
| citation / attribution discipline                         | craft    | read | critic-as-reader                        |

### Activation + dispatch path

`languages: [..., author]` in config -> `/in-session` loads `/in-author-essentials` -> author writes -> `/in-review` (or a direct `Task`) dispatches `critic-author` (and, only if code languages are ALSO declared, the matching code critics on their subtrees) -> critic-author runs the mechanical tier by default (incl. the mechanical trope pass) and recommends `/in-detrope` for the full LLM diagnosis under direct instruction.

## Alternatives Considered

- **Approach B -- generalise the axis word (`languages` -> `packs`/`disciplines`).** Highlander-cleanest (one honest word for one mechanism) and a fail-forward rename is on-brand, but it touches the config field, `intent lang`, `--lang` flags, and docs fleet-wide. Deferred: build one or two packs on Approach A first, promote to B once the shape is proven. Not rejected -- sequenced.
- **Approach C -- separate `domain:` axis.** Keeps "language" pure but stands up a second pack-activation system doing structurally the same job -- a Highlander violation at the architecture level, and it blocks the `[elixir, author]` composition and the free code-critic exclusion (D7). Rejected.
- **Option 2 -- canon-only critic (no formal rule library).** `critic-author` reads the `RULES-author.md` canon checklist instead of a `rules/author/**` pack, avoiding the `AU` schema bump. Rejected: it diverges the critic from the established rule-library pattern (a second critic-loading model = Highlander smell) and forgoes `intent claude rules` / index / validate. First-class is worth the small bump.

## Decisions ratified (hv, 2026-07-03)

1. **`AU` language-code schema bump approved** (D2) -- model `author` as a first-class rule-library language across the v1 ID/validation sites.
2. **v1 headless-gate scope approved** (D4) -- on-demand `Task(critic-author)` + detrope only; defer the pre-commit prose gate.
3. **Two-form detrope** (D5) and **`languages`-scoped dispatch** (D7) are hv refinements folded into the design.

Remaining sub-decision (resolve in WP02/WP03, not blocking): the exact form of the mechanical trope surface (⚙ rule Detection vs a vendored `data/trope-indicators`).
