---
verblock: "22 Apr 2026:v0.3: matts - Done"
wp_id: WP-06
title: "Rust Swift Lua rule packs"
scope: Large
status: Done
---

# WP-06: Rust, Swift, Lua rule packs

## As-Built (2026-04-22)

19 new rules shipped — 7 Rust + 6 Swift + 6 Lua — all inside WP06's 5-15-per-language cap. Every rule passes `intent claude rules validate`. Total rule count rose from 23 to 42 (+19).

### Rules shipped

#### Rust (7)

| ID             | Slug                                 | Category | Severity       |
| -------------- | ------------------------------------ | -------- | -------------- |
| IN-RS-CODE-001 | code/result-over-panic               | code     | critical       |
| IN-RS-CODE-002 | code/ownership-before-clone          | code     | warning        |
| IN-RS-CODE-003 | code/traits-over-enums-for-behaviour | code     | recommendation |
| IN-RS-CODE-004 | code/error-types-thiserror-anyhow    | code     | warning        |
| IN-RS-CODE-005 | code/lifetime-elision-first          | code     | style          |
| IN-RS-TEST-001 | test/cfg-test-colocated              | test     | warning        |
| IN-RS-TEST-002 | test/assert-matches-for-variants     | test     | warning        |

#### Swift (6)

| ID             | Slug                                | Category | Severity       |
| -------------- | ----------------------------------- | -------- | -------------- |
| IN-SW-CODE-001 | code/guard-over-nested-if           | code     | warning        |
| IN-SW-CODE-002 | code/optionals-over-sentinels       | code     | warning        |
| IN-SW-CODE-003 | code/structured-concurrency         | code     | warning        |
| IN-SW-CODE-004 | code/access-control-narrowest       | code     | recommendation |
| IN-SW-CODE-005 | code/codable-over-manual-json       | code     | warning        |
| IN-SW-TEST-001 | test/xctassertequal-specific-values | test     | warning        |

#### Lua (6)

| ID             | Slug                              | Category | Severity       |
| -------------- | --------------------------------- | -------- | -------------- |
| IN-LU-CODE-001 | code/local-over-global            | code     | critical       |
| IN-LU-CODE-002 | code/tables-as-structs            | code     | recommendation |
| IN-LU-CODE-003 | code/metatables-sparingly         | code     | warning        |
| IN-LU-CODE-004 | code/pcall-for-errors             | code     | critical       |
| IN-LU-CODE-005 | code/module-return-pattern        | code     | warning        |
| IN-LU-TEST-001 | test/busted-describe-it-structure | test     | warning        |

### Agnostic concretised_by updates

Extended every agnostic rule (except THIN-COORD, which has no Rust/Swift/Lua equivalent yet) to reference at least one rule per language:

- HIGHLANDER: +IN-RS-CODE-002, IN-SW-CODE-004, IN-LU-CODE-005
- PFIC: +IN-RS-CODE-003, IN-SW-CODE-001, IN-LU-CODE-002
- NO-SILENT-ERRORS: +IN-RS-CODE-001, IN-SW-CODE-002, IN-LU-CODE-004
- THIN-COORD: unchanged (Elixir-only concretisations until critic-rust/swift/lua land)

### Tests

- `tests/unit/rule_pack_rust.bats` — 9 tests
- `tests/unit/rule_pack_swift.bats` — 9 tests
- `tests/unit/rule_pack_lua.bats` — 9 tests

Each file enforces: catalog presence, total count invariant, canonical ID declaration, `language:` field matches, validator agreement, `intent claude rules list --lang <lang>` includes every id, fenced language-tagged code blocks in both ## Bad and ## Good sections, and no sibling `.rs` / `.swift` / `.lua` files (textual-only invariant per CI-LIMITATIONS.md).

Suite rose 592 -> 619 (+27).

### Deviations from plan

- BATS preprocessor quirk: two consecutive `@test` blocks whose bodies both embed literal triple-backtick strings caused one test to be silently dropped from the plan count. Worked around by moving the fence marker into a `local fence='...'` variable, which BATS's preprocessor handles cleanly.
- No per-pack `index.json` regeneration test — the root-level `intent claude rules index` invariant is covered by existing WP01 tests, and this WP's 27 BATS tests already fail if any rule is missing.
- Highlander cross-language audit conducted manually: rule bodies are independently written; shared agnostic IDs cited via `references:` not restated.

### Commits

- (current session) — Rust pack (7 rules) + Swift pack (6 rules) + Lua pack (6 rules); agnostic concretised_by extended; 3 BATS files (27 tests) = 619 total.

## Objective

Author language-idiomatic rules for Rust, Swift, and Lua. Establish pack structure and initial coverage for v2.9.0; designed to grow via user extensions and future STs. Each language gets at least 5 rules covering foundational idioms and one or two test-related rules.

## Context

Intent's TCA v3.0 (ST0028) validated a set of Rust and Swift rules for codebase audits. WP06 extends that foundation into first-class rule files that Critic subagents can apply to individual files, not just bulk audit runs. Lua coverage is new ground for Intent; scoped conservatively.

CI limitation: Intent does not have Rust/Swift/Lua CI environments. Runnable examples are textual only (fenced code blocks in RULE.md), not separate `good.rs` / `bad.rs` files. Validation happens when Critic subagents (WP07) run against real projects.

Scope cap: 5-15 rules per language for v2.9.0. More goes to user extensions or future STs. WP12 (risk R12 mitigation) enforces this cap.

## Deliverables

### Rust rule pack

```
intent/plugins/claude/rules/rust/
  code/
    result-over-panic/              # IN-RS-CODE-001
    ownership-before-clone/         # IN-RS-CODE-002
    traits-over-enums-for-behaviour/# IN-RS-CODE-003
    error-types-thiserror-anyhow/   # IN-RS-CODE-004
    lifetime-elision-first/         # IN-RS-CODE-005
  test/
    cfg-test-colocated/             # IN-RS-TEST-001
    assert-matches-for-variants/    # IN-RS-TEST-002
```

Target: 5 code + 2 test = 7 rules.

### Swift rule pack

```
intent/plugins/claude/rules/swift/
  code/
    guard-over-nested-if/           # IN-SW-CODE-001
    optionals-over-sentinels/       # IN-SW-CODE-002
    structured-concurrency/         # IN-SW-CODE-003
    access-control-narrowest/       # IN-SW-CODE-004
    codable-over-manual-json/       # IN-SW-CODE-005
  test/
    xctassertequal-specific-values/ # IN-SW-TEST-001
```

Target: 5 code + 1 test = 6 rules.

### Lua rule pack

```
intent/plugins/claude/rules/lua/
  code/
    local-over-global/              # IN-LU-CODE-001
    tables-as-structs/              # IN-LU-CODE-002
    metatables-sparingly/           # IN-LU-CODE-003
    pcall-for-errors/               # IN-LU-CODE-004
    module-return-pattern/          # IN-LU-CODE-005
  test/
    busted-describe-it-structure/   # IN-LU-TEST-001
```

Target: 5 code + 1 test = 6 rules.

### CI-LIMITATIONS documentation

- `rules/_schema/CI-LIMITATIONS.md` (from WP01) updated with Rust/Swift/Lua specifics: "Runnable examples for these languages are textual only in v2.9.0. Validation occurs at Critic invocation time against real projects. Future work: CI environments for each language."

### Rule citations

Each rule's Further Reading section cites canonical sources:

- Rust: The Rust Programming Language (TRPL), Rust API Guidelines, specific Clippy lint names
- Swift: Apple's Swift API Design Guidelines, Swift Evolution proposals
- Lua: Roberto Ierusalimschy's Programming in Lua, relevant style guides

## Approach

1. **Cross-reference TCA v3.0 (ST0028) validated rules.** That ST established a set of Rust and Swift audit rules. Anywhere this WP's rule overlaps TCA, use TCA's formulation as the canonical source and cite it.

2. **Author in language order.** Rust first (most established ecosystem in Intent's context), Swift second, Lua third.

3. **Per rule, write:**
   - Frontmatter (ID, slug, title, language, category, severity, applies_to, references, aliases, version)
   - Problem
   - Detection (language-specific heuristic: grep pattern, syntactic marker, AST signal in prose)
   - Bad example as fenced code block inline (no separate file)
   - Good example as fenced code block inline
   - When Applies
   - When Does Not Apply
   - Further Reading

4. **Cross-reference agnostic rules.** Each language rule's `references:` lists any agnostic rule it concretises (e.g. `IN-RS-CODE-001` references `IN-AG-NO-SILENT-001`; `IN-LU-CODE-004` also references `IN-AG-NO-SILENT-001`).

5. **Update agnostic `concretised_by:` lists.** After WP06 rules are authored, go back to WP04's agnostic rules and finalise their `concretised_by:` to include Rust/Swift/Lua concretising rule IDs.

6. **Highlander audit.** No Rust rule restates a Swift or Lua rule (or vice versa). Cross-language consistency without content duplication.

7. **Update CI-LIMITATIONS.** Document exactly which languages have runnable examples (Elixir: yes) vs textual only (Rust/Swift/Lua).

## Acceptance Criteria

- [ ] Rust pack: at least 5 rules (target 7) with full RULE.md
- [ ] Swift pack: at least 5 rules (target 6) with full RULE.md
- [ ] Lua pack: at least 5 rules (target 6) with full RULE.md
- [ ] Each rule has complete frontmatter conforming to schema
- [ ] Each rule has textual good/bad examples as fenced code blocks with correct `language:` tag (`rust`, `swift`, `lua`)
- [ ] All fenced code uses 2-space indentation per Intent standard (regardless of language default)
- [ ] Each rule cites at least one canonical source (book, guide, Evolution proposal, Clippy lint name)
- [ ] Every rule's `references:` field resolves (no dangling IDs)
- [ ] Agnostic rules (WP04) have `concretised_by:` fully populated with real Rust/Swift/Lua IDs
- [ ] `CI-LIMITATIONS.md` (from WP01) remains accurate for this pack
- [ ] Highlander audit: no cross-language rule prose duplication
- [ ] Rule content aligns with TCA v3.0 where overlap exists
- [ ] `intent claude rules validate rules/{rust,swift,lua}/` passes
- [ ] No `.rs` / `.swift` / `.lua` sibling files alongside any RULE.md (textual-only invariant)

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP06.

- [ ] `tests/unit/rule_pack_rust.bats` — frontmatter validity, required sections, fenced `rust` blocks in `## Bad` / `## Good`, no sibling `.rs` files
- [ ] `tests/unit/rule_pack_swift.bats` — same for Swift (`swift` fence tag)
- [ ] `tests/unit/rule_pack_lua.bats` — same for Lua (`lua` fence tag)

### Tests to update

- [ ] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

## Dependencies

- **WP01** (schema): required.
- **WP04** (agnostic pack): required. Language rules reference agnostic rules.

Loose coupling with WP07: WP07's Critic subagents enumerate these rules. If rule IDs shift during WP06, WP07 re-enumerates without issue since it reads RULE.md at runtime rather than embedding IDs.

## Implementation Notes

### Rust rule drafts

**IN-RS-CODE-001: Result over panic in library code**

- Problem: `.unwrap()` / `panic!()` in library code crashes dependent callers; library consumers cannot recover.
- Detection: `.unwrap()` or `.expect(...)` in non-test, non-main code. Clippy lint: `unwrap_used`, `expect_used`.
- Bad example: `let user = db.find(id).unwrap();`
- Good example: `let user = db.find(id).ok_or(Error::NotFound)?;`
- When applies: library code, error-returning functions, fallible operations.
- When does not apply: tests, prototypes, documented infallible invariants.
- Further reading: Rust API Guidelines, Clippy `unwrap_used`.

**IN-RS-CODE-002: Ownership before clone**

- Problem: Reflexive `.clone()` hides borrowing opportunities, creates unnecessary allocations.
- Detection: `.clone()` calls in hot paths; especially on large structures or when a borrow would suffice.
- Bad example: `fn process(data: Vec<Item>) { let copy = data.clone(); ... }`
- Good example: `fn process(data: &[Item]) { ... }`

**IN-RS-CODE-003: Traits over enums for behavioural polymorphism**

- Problem: Enum-based dispatch limits extensibility and bloats match arms.
- Detection: Enums with many variants where each variant encodes distinct behaviour (look for giant `match` statements that perform per-variant work).
- Good example: `trait Handler { fn handle(&self); }` with distinct impl types.
- When does not apply: closed sets of variants that truly are data (e.g. AST nodes).

**IN-RS-CODE-004: Error types — thiserror for libraries, anyhow for applications**

- Problem: `Box<dyn Error>` everywhere loses type info; `Result<T, String>` loses structure.
- Detection: Look for `Box<dyn Error>` in library APIs; look for missing error-type definitions.
- Good example: `#[derive(thiserror::Error)]` enum in libs; `anyhow::Result` in app main.
- When does not apply: tiny scripts, one-off utilities.

**IN-RS-CODE-005: Lifetime elision first**

- Problem: Explicit lifetimes where elision suffices adds noise.
- Detection: `fn f<'a>(x: &'a str) -> &'a str` where elision would work.
- Good example: `fn f(x: &str) -> &str`.

**IN-RS-TEST-001: `#[cfg(test)]` modules colocated with code**

- Problem: Tests in separate `tests/` directory cannot access private items; integration tests only.
- Detection: Unit tests that should exercise internals but live in `tests/`.
- Good example: `mod tests { use super::*; #[test] fn foo() {} }` inside the same file.

**IN-RS-TEST-002: `assert_matches!` for tagged variants**

- Problem: `assert_eq!` on an enum variant that wraps data is brittle and noisy.
- Detection: `assert_eq!(result, Ok(SomeStruct { ... }))` with long literals.
- Good example: `assert_matches!(result, Ok(SomeStruct { x: 1, .. }))`.

### Swift rule drafts

**IN-SW-CODE-001: guard over nested if**

- Problem: Deeply nested `if`/`else` obscures control flow; early exit via `guard` is the idiom.
- Bad example: `if let x = opt { if let y = other { ... } }`
- Good example: `guard let x = opt, let y = other else { return }`
- Cite: Apple's Swift API Design Guidelines.

**IN-SW-CODE-002: Optionals over sentinels**

- Problem: `-1`, `NSNotFound`, empty strings as sentinel values trade type safety for convenience.
- Bad example: `func indexOf(_ item: Item) -> Int { ... returns -1 if absent }`
- Good example: `func indexOf(_ item: Item) -> Int?`

**IN-SW-CODE-003: Structured concurrency over GCD**

- Problem: DispatchQueue callbacks lose type information and compose poorly.
- Bad example: `DispatchQueue.global().async { ... }` with completion handlers.
- Good example: `async/await` + `actor` types.
- When does not apply: maintaining pre-Swift 5.5 codebases; interop with C APIs.

**IN-SW-CODE-004: Access control — narrowest first**

- Problem: `public` by default exposes internals; `internal` is the Swift default for a reason.
- Detection: `public` on types/methods that aren't part of a library's public API.
- Good example: `private`, `fileprivate`, or `internal` (the default) before considering `public`.

**IN-SW-CODE-005: Codable over manual JSON parsing**

- Problem: Manual `JSONSerialization` + dictionary casting is error-prone and type-unsafe.
- Good example: `struct User: Codable { ... }` + `JSONDecoder`.

**IN-SW-TEST-001: XCTAssertEqual with specific values**

- Problem: `XCTAssertNotNil(user)` is a shape assertion; doesn't prove the user has the expected data.
- Good example: `XCTAssertEqual(user.name, "Alice")`.
- Cross-reference: IN-EX-TEST-001 (Elixir equivalent).

### Lua rule drafts

**IN-LU-CODE-001: `local` over global**

- Problem: Unintended globals pollute the shared environment and race with other modules.
- Detection: Variable assignment without `local` keyword.
- Good example: `local counter = 0`
- When does not apply: intentional global APIs exposed by the module.

**IN-LU-CODE-002: Tables as structs, not OOP reflex**

- Problem: Overusing metatables and inheritance in Lua fights the language's native style.
- Bad example: Deep OOP hierarchies with metatable inheritance chains.
- Good example: Plain tables with functions stored as fields, or module tables returning flat APIs.

**IN-LU-CODE-003: Metatables sparingly, document when used**

- Problem: Metatables create action-at-a-distance; unexpected index/newindex hooks.
- Detection: `setmetatable` calls without accompanying comment explaining the hook.
- When applies: reading ALL Lua that uses setmetatable.
- When does not apply: canonical idioms (e.g. OO via metatables in frameworks like Love2D).

**IN-LU-CODE-004: `pcall` for error boundaries, tagged results**

- Problem: Unhandled errors in Lua propagate via longjmp-like mechanism; swallowed errors silent.
- Good example: `local ok, result_or_err = pcall(risky_function)` with explicit check.
- Cross-reference: IN-AG-NO-SILENT-001.

**IN-LU-CODE-005: Module return pattern**

- Problem: Modules that don't return their API table (relying on globals) cannot be composed.
- Good example: `local M = {} M.foo = function() end return M`.

**IN-LU-TEST-001: busted describe/it structure**

- Problem: Ad-hoc `if/else` tests are hard to report on and parallelise.
- Good example: `describe("module", function() it("does thing", function() assert.equal(x, 1) end) end)`.
- Framework: busted or luaunit.

### Textual examples format

```markdown
## Bad example

\`\`\`rust
fn load(id: u32) -> User {
let user = db.find(id).unwrap(); // panics on missing
user
}
\`\`\`

## Good example

\`\`\`rust
fn load(id: u32) -> Result<User, Error> {
db.find(id).ok_or(Error::NotFound)
}
\`\`\`
```

No separate `good.rs` / `bad.rs` files; just fenced blocks in the RULE.md body. This matches the CI-LIMITATIONS decision.

## Risks and Edge Cases

### No CI validation (R10)

Textual examples can drift from actual Rust/Swift/Lua syntax over time. Mitigation: manual review; when Critic subagents run in real projects they surface issues; consider future CI integration.

### Lua dialect ambiguity

Lua 5.1, 5.3, 5.4, LuaJIT have minor syntactic/semantic differences. Scope to Lua 5.3+ for v2.9.0; document in `CI-LIMITATIONS.md`.

### Swift API volatility

Swift Evolution moves fast; structured concurrency rules may age. Mitigation: cite the Evolution proposal by number (e.g. SE-0296 for async/await) so future readers can check current status.

### Rust ecosystem shifts

Preferences (anyhow vs eyre, tokio vs async-std) evolve. Rules cite Clippy lint names where stable; avoid recommending specific libraries unless truly universal.

### Scope creep (R12)

Authoring 10+ rules per language is appealing. Cap at 5-15 per language (hard limit 15). More rules via user extensions.

### Cross-language Highlander

A rule about "error handling" exists in all three packs. Mitigation: each cites IN-AG-NO-SILENT-001 and differentiates on language-specific mechanics (Result vs throws vs pcall). Prose does not duplicate.

## Testing Approach

### Manual review

- Each rule scanned for:
  - Clear Problem statement
  - Concrete Detection heuristic
  - Syntactically valid code examples (manually eyeballed for correctness)
  - Specific source citation

### Schema validation

- `intent claude rules validate rules/rust/` passes
- Same for `rules/swift/` and `rules/lua/`

### Cross-language Highlander audit

```bash
# Pick a rule's signature phrase; should only appear in one file
grep -rn "Result over panic" rules/
grep -rn "guard over nested" rules/
```

### TCA v3.0 alignment check

For each WP06 rule that overlaps TCA v3.0's validated set: formulation matches; severity consistent.

### Critic dry-run (deferred to WP07)

When WP07 lands, each `critic-<lang>` is run against a "would-catch" fixture; each WP06 rule must produce at least one finding on the appropriate fixture.

## Size and Estimate

- **Size**: L (Large, 4-6 sessions). Three languages, research-heavy.
- **Session 1**: Rust rule authoring (7 rules).
- **Session 2**: Swift rule authoring (6 rules).
- **Session 3**: Lua rule authoring (6 rules).
- **Session 4**: Cross-references updated; agnostic `concretised_by:` finalised.
- **Session 5**: Highlander audit, TCA alignment check, schema validation fixes.
- **Session 6**: Buffer for research gaps, citation verification.

## Exit Checklist

- [ ] All acceptance criteria met
- [ ] ≥5 rules per language, all with complete frontmatter and textual examples
- [ ] Every rule cites a canonical source
- [ ] Cross-references resolve
- [ ] CI-LIMITATIONS.md documents the textual-only scope for these languages
- [ ] Agnostic rules updated with Rust/Swift/Lua `concretised_by:` IDs
- [ ] Highlander audit passes across languages
- [ ] TCA v3.0 alignment verified where overlap exists
- [ ] Registered in MODULES.md
