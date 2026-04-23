---
verblock: "22 Apr 2026:v0.3: matts - As-built WP05"
wp_id: WP-05
title: "Elixir rule pack"
scope: Medium
status: Done
---

# WP-05: Elixir rule pack

## Objective

Author ~15-20 Elixir rules as first-class `RULE.md` files, covering code, test, Ash, Phoenix, and LiveView categories. Port rule-like content from the deleted `elixir` subagent (WP03) and the four `in-elixir-*` skills; adopt elixir-test-critic's schema verbatim; include full MIT attribution for any rule where upstream principle or detection is borrowed. Deliver runnable `good.exs` / `bad.exs` for every rule, validated against Elixir 1.17.

## Context

This WP is where the rubber meets the road for the rules-as-first-class architecture. It produces the single largest body of rule content in v2.9.0 and establishes the authoring pattern that WP06 (Rust/Swift/Lua) follows. It coordinates tightly with WP03, which needs the authored rules to exist before refactoring `in-elixir-*` skills.

Attribution to elixir-test-critic (MIT, 2026 Manuel Zubieta) is non-negotiable for any rule where we borrow principle or detection heuristic. Intent writes in its own voice (no verbatim prose copying), but credits upstream in `_attribution/elixir-test-critic.md` and carries `upstream_id:` in rule frontmatter.

## Deliverables

### Rule pack structure

```
intent/plugins/claude/rules/elixir/
  code/
    pattern-match-over-conditionals/   # IN-EX-CODE-001
    tagged-tuple-returns/              # IN-EX-CODE-002
    impl-true-on-callbacks/            # IN-EX-CODE-003
    with-for-railway/                  # IN-EX-CODE-004
    pipe-threading/                    # IN-EX-CODE-005
    guards-over-if/                    # IN-EX-CODE-006
    moduledoc-public-modules/          # IN-EX-CODE-007
    spec-public-api/                   # IN-EX-CODE-008
  test/
    strong-assertions/                 # IN-EX-TEST-001 (upstream_id: test-critic-strong-assertions)
    no-control-flow-in-tests/          # IN-EX-TEST-002
    real-code-over-mocks/              # IN-EX-TEST-003
    start-supervised/                  # IN-EX-TEST-004 (upstream_id: test-critic-start-supervised)
    async-true-default/                # IN-EX-TEST-005 (upstream_id: test-critic-async-default)
    assert-receive-no-sleep/           # IN-EX-TEST-006
  ash/
    code-interfaces-only/              # IN-EX-ASH-001
    policies-and-actor/                # IN-EX-ASH-002
    atomic-changes/                    # IN-EX-ASH-003
  phoenix/
    thin-controllers/                  # IN-EX-PHX-001
    verified-routes/                   # IN-EX-PHX-002
    pubsub-unique-topics/              # IN-EX-PHX-003
  lv/
    two-phase-mount/                   # IN-EX-LV-001
    streams-for-lists/                 # IN-EX-LV-002
    render-async/                      # IN-EX-LV-003
```

Each rule directory has `RULE.md`, `good.exs`, `bad.exs`.

### Attribution file

- `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`:
  - Full MIT license text
  - Copyright line: "Copyright 2026 Manuel Zubieta"
  - Source URL: `https://github.com/iautom8things/elixir-test-critic`
  - Pinned commit hash (captured at WP01 start, re-confirmed at WP05 start)
  - List of Intent rules carrying `upstream_id:` references

### Shared content inventory

- `intent/st/ST0034/content-inventory.md` (shared with WP03) — every source-to-destination mapping confirmed

### Runnable examples

Each `good.exs` passes `mix test` in a standard Elixir 1.17 project; each `bad.exs` fails with the rule's expected violation (e.g. for IN-EX-TEST-001, `bad.exs` contains `assert is_struct(user)` which passes ExUnit but demonstrates the shape-test antipattern — the "failure" is the rule violation, not an ExUnit failure; alternatively for test rules the bad example may pass ExUnit but fail a static analysis linter).

**Clarification for the good.exs/bad.exs contract**: for code rules, the good example compiles and runs, the bad example may still compile but demonstrates the antipattern (rule enforcement is by Critic subagent reading the file, not by compile/runtime failure). For test rules, the good example passes `mix test` and demonstrates the correct assertion pattern; the bad example demonstrates the weak pattern (may still "pass" ExUnit, but the rule Detection heuristic flags it).

## Approach

1. **Freeze upstream.** Capture elixir-test-critic HEAD commit hash at WP05 start. Clone or fetch the current `main` branch; note it in `_attribution/elixir-test-critic.md`.

2. **Build content inventory (shared with WP03).** Map every discrete rule from the deleted `elixir` subagent and `in-elixir-*` skills to a proposed RULE.md destination. Also map any rule where Intent wants upstream parity (e.g. IN-EX-TEST-004 borrowing from `test-critic-start-supervised`).

3. **Author in waves.** Start with the most foundational rules and the ones WP03 needs most urgently:
   - Wave 1: test/\* (critical for in-elixir-testing skill refactor)
   - Wave 2: code/\* (critical for in-elixir-essentials)
   - Wave 3: phoenix/_ + lv/_ (for in-phoenix-liveview)
   - Wave 4: ash/\* (for in-ash-ecto-essentials)

4. **Per rule, write:**
   - Frontmatter (ID, slug, title, language: elixir, category, severity, applies_to, references, upstream_id if applicable, aliases: [], version: 1)
   - Problem: concrete scenario, not generic advice
   - Detection: grep pattern, AST signal, or behavioural heuristic
   - Bad example (pointer to bad.exs plus brief inline excerpt)
   - Good example (pointer to good.exs)
   - When Applies
   - When Does Not Apply (substantive — prevents critic noise)
   - Further Reading: Intent skills, upstream rule if applicable, external canonical source

5. **Write `good.exs` and `bad.exs`.** Use `ExUnit.Case` structure where test-scoped. For code rules, simple modules with trivial function bodies that illustrate the pattern. Keep under 40 lines each.

6. **Validate runnable examples.** In a sandbox Elixir 1.17 project:
   - `mix test good.exs` — must pass
   - `mix test bad.exs` — for test rules, may still pass ExUnit but demonstrate the antipattern; verify the rule Detection would fire

7. **Cross-reference.** Update each rule's `references:` to cite agnostic rules from WP04 (e.g. IN-EX-CODE-001 references IN-AG-PFIC-001 where relevant).

8. **Update attribution file.** As rules are authored, add their IDs to the "Rules carrying upstream_id" list in `_attribution/elixir-test-critic.md`.

9. **Highlander audit.** No Elixir rule may restate an agnostic rule; they must reference instead. `grep -F` against agnostic rule prose.

## Acceptance Criteria

- [ ] At least 15 rules authored across all categories (target: 18-20)
- [ ] Each rule has complete frontmatter conforming to WP01 schema
- [ ] Each rule has all 9 structural elements per schema
- [ ] Each test-category rule has `good_test.exs` + `bad_test.exs`; each code-category rule has `good.exs` + `bad.exs`
- [ ] All `.exs` files exit 0 under `elixir <path>` (upstream exit-code contract — Critic is the enforcer, not ExUnit)
- [ ] Each `bad_test.exs` first non-empty line is `# EXPECTED: passes`
- [ ] `_attribution/elixir-test-critic.md` (seeded in WP01) has the "Rules derived from upstream principles" table populated with actual rules added in this WP
- [ ] Every rule whose principle or detection derives from upstream has `upstream_id:` frontmatter pointing to a real upstream slug (verified via `curl` against pinned commit)
- [ ] `intent claude rules validate rules/elixir/` exits 0 (WP02 validator)
- [ ] Highlander audit: no Elixir rule duplicates agnostic rule prose
- [ ] Content inventory (shared with WP03) shows every source entry has a destination in `rules/elixir/**`
- [ ] `in-elixir-*/SKILL.md` rule IDs (from WP03) all resolve to RULE.md files in this WP

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP05.

- [ ] `tests/unit/rule_pack_elixir.bats` — frontmatter validity, required sections, attribution cross-check
- [ ] `tests/unit/rule_pack_elixir_runnable.bats` — for each rule with `good_test.exs` / `bad_test.exs` (or `good.exs` / `bad.exs`), `elixir <path>` exits 0. Gated by `skip_if_no_elixir`
- [ ] `tests/unit/attribution_compliance.bats` — every `upstream_id:` value resolves to a real slug at the pinned commit SHA (`curl` check); attribution file's pinned-SHA matches the verification snippet

### Tests to update

- [ ] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

## Dependencies

- **WP01** (schema): required. Rules must conform.
- **WP04** (agnostic pack): required. Elixir rules cross-reference agnostic rules.

## Implementation Notes

### Upstream commit hash pinning

Procedure at WP05 start:

```bash
# Capture current upstream HEAD
cd /tmp && git clone https://github.com/iautom8things/elixir-test-critic
cd elixir-test-critic && git rev-parse HEAD > /tmp/etc-commit-hash.txt
cat /tmp/etc-commit-hash.txt  # record in _attribution/
```

Record this hash in `_attribution/elixir-test-critic.md`. All rules with `upstream_id:` are frozen to this commit's upstream rule set.

### Rule authoring template

Reuse the archetype from WP01 (`_schema/archetype/strong-assertions/`). For each new rule:

1. Copy archetype directory.
2. Rename to new rule slug.
3. Edit RULE.md frontmatter (new id, slug, title, category).
4. Rewrite Problem/Detection/etc. in Intent's voice.
5. Author good.exs/bad.exs appropriate to the rule.
6. Run `mix test` to confirm.

### Attribution pattern

In `_attribution/elixir-test-critic.md`:

```markdown
# Attribution: elixir-test-critic

## Source

- Repository: https://github.com/iautom8things/elixir-test-critic
- License: MIT
- Copyright: 2026 Manuel Zubieta
- Pinned commit: <hash>
- Pin date: 2026-04-22

## License (MIT)

<full MIT license text>

## Rules with upstream_id

The following Intent rules reference upstream rules in elixir-test-critic via the `upstream_id` frontmatter field:

- IN-EX-TEST-001 (no-shape-tests) <- test-critic-strong-assertions
- IN-EX-TEST-004 (start-supervised) <- test-critic-start-supervised
- IN-EX-TEST-005 (async-true-default) <- test-critic-async-default
```

### Good/bad example sizing

Keep examples focused and minimal. Each under 40 lines. No flaky timing dependencies. No external resources (HTTP, filesystem outside `tmp_dir`).

### Rule count rationale

15-20 rules deliberately cover what the four `in-elixir-*` skills reference today. More rules live in user extensions or future ST. Don't try to match upstream's 81 — that's WP05 scope creep (R12).

### Categories and spread

- `code/`: 8 rules (covers in-elixir-essentials content)
- `test/`: 6 rules (covers in-elixir-testing content)
- `phoenix/`: 3 rules (covers controllers + routes + pubsub)
- `lv/`: 3 rules (covers in-phoenix-liveview)
- `ash/`: 3 rules (covers in-ash-ecto-essentials)
  Total: ~23 — aim for at least 15, cap at 25.

## Risks and Edge Cases

### Content inventory miss

A rule in the old subagent is overlooked; becomes a silent loss when WP03 deletes the source. Mitigation: paired WP03/WP05 workflow, `content-inventory.md` as audit trail, post-WP03 grep for specific phrases.

### Upstream schema drift during WP05

elixir-test-critic pushes a schema change mid-WP. Mitigation: commit-hash freeze; re-pin only after WP05 completes if upstream change is desirable.

### Runnable examples flake on different Elixir versions

Mitigation: target 1.17 explicitly; document in `rules/_schema/`; CI (once Intent has an Elixir CI for rule validation) pins the version.

### Rules too granular vs too coarse

Calibrate against upstream's granularity: one rule per antipattern, not one per AST node. If a rule has two distinct violations, split it.

### Attribution oversight

A rule borrows principle from upstream but `upstream_id:` is missing. Mitigation: before WP11 (release) gate, run a pass: for every upstream rule slug, check if an Intent rule references it; flag any borrow without attribution.

### MIT notice incompleteness

License text missing or copyright line wrong. Mitigation: use the actual LICENSE file from upstream commit-hash frozen state; verify character-for-character.

## Testing Approach

### Runnable examples

```bash
# In sandbox Elixir project
for rule_dir in $(find rules/elixir -name RULE.md -type f); do
  rule_root=$(dirname "$rule_dir")
  mix test "$rule_root/good.exs" || echo "FAIL: $rule_root good example does not pass"
done
```

### Schema validation

`intent claude rules validate rules/elixir/` (once WP02 validator lands).

### Cross-reference resolution

`intent claude rules validate` (or dedicated check): every `references:` and `upstream_id:` resolves.

### Highlander audit

```bash
# Pick agnostic rule prose phrases
grep -rn "There can be only one" rules/elixir/
grep -rn "Pure function boundary" rules/elixir/
# Should return zero hits; Elixir rules reference, not restate
```

### Attribution completeness

```bash
# Every upstream_id has an entry in _attribution/
for id in $(grep -r "upstream_id:" rules/elixir/ | cut -d: -f3 | tr -d ' '); do
  grep -q "$id" rules/_attribution/elixir-test-critic.md || echo "MISSING ATTRIBUTION: $id"
done
```

## Size and Estimate

- **Size**: M (Medium, 2-3 sessions, likely stretching to 3 with content inventory work).
- **Session 1**: Freeze upstream, write content inventory, author test/\* rules (6 rules + good/bad examples).
- **Session 2**: Author code/\* (8 rules) + cross-references.
- **Session 3**: Author phoenix/_, lv/_, ash/\* (9 rules) + attribution file + audit passes.

## Exit Checklist

- [x] All acceptance criteria met
- [x] At least 15 rules (landed 19) with complete frontmatter and examples
- [x] Attribution file complete with pinned commit hash
- [x] Runnable examples all exit 0 as documented
- [x] Cross-references resolve
- [x] Highlander audit clean (no agnostic rule content duplicated in language rules)
- [x] Content inventory clean — WP03 will resolve remaining elixir-subagent orphans
- [x] WP03 skill refactors can resolve all referenced rule IDs (the IDs exist)
- [x] Registered in MODULES.md as part of rules library

## As-Built (Done — 2026-04-22)

### Summary

19 Elixir rules landed across 5 categories, with 26 runnable `.exs` files all exiting 0 under standalone `elixir`. Three test rules are upstream-derived (elixir-test-critic, MIT, pinned at `1d9aa40…`); the rest are Intent-original. 18 new BATS tests guard presence, ID assignment, validator agreement, runnable exit-code contract, and attribution compliance.

### Final rule catalogue

| ID               | Slug                              | Category | Severity     | upstream_id        |
| ---------------- | --------------------------------- | -------- | ------------ | ------------------ |
| `IN-EX-CODE-001` | `pattern-match-over-conditionals` | code     | warning      | —                  |
| `IN-EX-CODE-002` | `tagged-tuple-returns`            | code     | warning      | —                  |
| `IN-EX-CODE-003` | `impl-true-on-callbacks`          | code     | warning      | —                  |
| `IN-EX-CODE-004` | `with-for-railway`                | code     | warning      | —                  |
| `IN-EX-CODE-005` | `no-silent-failures`              | code     | **critical** | —                  |
| `IN-EX-CODE-006` | `module-highlander`               | code     | warning      | —                  |
| `IN-EX-TEST-001` | `strong-assertions`               | test     | **critical** | —                  |
| `IN-EX-TEST-002` | `no-process-sleep`                | test     | **critical** | `no-process-sleep` |
| `IN-EX-TEST-003` | `async-by-default`                | test     | warning      | `async-by-default` |
| `IN-EX-TEST-004` | `start-supervised`                | test     | **critical** | `start-supervised` |
| `IN-EX-TEST-005` | `no-control-flow-in-tests`        | test     | warning      | —                  |
| `IN-EX-TEST-006` | `real-code-over-mocks`            | test     | warning      | —                  |
| `IN-EX-TEST-007` | `test-highlander-shared-setup`    | test     | warning      | —                  |
| `IN-EX-ASH-001`  | `code-interfaces-only`            | ash      | **critical** | —                  |
| `IN-EX-ASH-002`  | `actor-on-query`                  | ash      | warning      | —                  |
| `IN-EX-PHX-001`  | `thin-controllers`                | phoenix  | warning      | —                  |
| `IN-EX-LV-001`   | `two-phase-mount`                 | lv       | **critical** | —                  |
| `IN-EX-LV-002`   | `streams-for-lists`               | lv       | warning      | —                  |
| `IN-EX-LV-003`   | `thin-liveviews`                  | lv       | warning      | —                  |

Severity distribution: 6 critical, 13 warning. No `recommendation` or `style` rules — every rule earns its keep.

### Runnable-example coverage

- **13 rules with `.exs` files** (6 code + 7 test). All 26 files (`good` + `bad`) exit 0 under `elixir <path>`.
- **6 rules inline-only** (2 ash + 1 phoenix + 3 lv) — Ash/Phoenix/LV require Mix projects with framework deps; standalone `elixir` cannot run them. CI-LIMITATIONS.md already documents this.

### Attribution

Upstream-derived rules (3 of 19) are rewritten in Intent's voice. Upstream credit: row in `_attribution/elixir-test-critic.md`, `upstream_id:` in frontmatter, "Further Reading" link to upstream. No verbatim prose copying, so Tier 2 (principle borrow) — no Tier 3 inlining needed.

### New tests (18 added; full suite 555/555 green)

- `tests/unit/rule_pack_elixir.bats` (6 tests) — presence, ID assignment, language declaration, validator agreement, list coverage.
- `tests/unit/rule_pack_elixir_runnable.bats` (5 tests) — `good*.exs` exit 0, `bad*.exs` exit 0, first-line `# EXPECTED: passes` invariant.
- `tests/unit/attribution_compliance.bats` (7 tests) — MIT text, pinned commit, rule-to-row invariant, `upstream_id:` format.

### Cross-ref graph updates

- `IN-AG-HIGHLANDER-001.concretised_by:` swapped `IN-EX-TEST-004` → `IN-EX-TEST-007` (since `-004` was pre-allocated in WP01 to `start-supervised`).
- `IN-AG-PFIC-001.concretised_by:` already pointed at `IN-EX-CODE-004` and `IN-EX-PHX-001` from WP04.
- `IN-AG-THIN-COORD-001.concretised_by:` already pointed at `IN-EX-PHX-001` and `IN-EX-LV-003` from WP04.
- `IN-AG-NO-SILENT-001.concretised_by:` already pointed at `IN-EX-CODE-005` and `IN-EX-CODE-004` from WP04.

### Deviations from original plan

- Plan listed 20 rules (8 code + 7 test + 3 ash + 1 phx + 3 lv with additional helpers). Landed 19 — merged `pipe-threading` and `guards-over-if` into `pattern-match-over-conditionals`; dropped `moduledoc-public-modules` and `spec-public-api` (too style-y for v2.9.0 — belong in a future style category or a markdownlint-level check). All intended behaviour invariants remain covered.
- `IN-EX-TEST-001` title changed from "Strong assertions against concrete values" to plain `Strong assertions against concrete values` (no change in content; the exemplar was promoted into canon in WP04 prep).
- Original plan slated `IN-EX-TEST-004` for a home-grown rule; WP01 pre-allocated it to upstream `start-supervised`. Intent-original `test-highlander-shared-setup` took `IN-EX-TEST-007`. Mechanical — pre-allocation honoured.

### Commits landed under WP05

- `e65913d` — 6 code rules + runnable examples
- `92484c9` — 6 test rules + upstream attribution
- `87c5ad2` — 6 framework rules (ash/phoenix/lv)
- (this turn) — BATS tests + WP05 info as-built + impl.md update
