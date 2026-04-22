---
id: IN-LU-TEST-001
language: lua
category: test
severity: warning
title: busted describe/it structure
summary: >
  Group tests into `describe` blocks with `it` cases. Use the framework's
  assertion helpers (`assert.equal`, `assert.same`, `assert.has_error`)
  rather than bare `assert(x == y)`. Structured tests compose with
  reporters, parallel runners, and failure diffs.
principles:
  - public-interface
applies_when:
  - "Writing tests with `busted` or another BDD-style Lua test framework"
  - "Porting ad-hoc `assert(x == y)` scripts into a runnable test suite"
  - "Adding regression tests alongside a bug fix"
applies_to:
  - "spec/**/*.lua"
  - "test/**/*.lua"
  - "tests/**/*.lua"
does_not_apply_when:
  - "Quick sanity scripts not part of the committed test suite"
  - "Self-check code inside a library using bare `assert` for invariants"
references:
  - IN-EX-TEST-001
related_rules: []
aliases: []
tags:
  - lua
  - testing
  - busted
status: active
version: 1
---

# busted describe/it structure

A test file without structure is a script. A test file with `describe`/`it` is documentation that runs.

## Problem

Lua's minimal test culture tempts authors to write test files as a linear sequence of `assert(...)` calls. They run, they pass, they fail. But they do not compose: no reporter can tell which test failed, no CI diff can show expected vs actual, no parallel runner can shard cases, no `focus`/`exclude` can iterate on a subset. When the test file grows to 500 lines, every failure becomes "something in tests.lua broke" with no context.

`busted` (the dominant Lua BDD framework) provides `describe("module", function() ... end)` and `it("does thing", function() assert.equal(...) end)` and assertion helpers that produce useful diffs. The shape is no more verbose than bare `assert`, and the test file becomes readable as a specification.

## Detection

Static signals:

- Test files containing only `assert(x == y)` calls at the top level, no `describe` or `it`.
- `assert.equals(a, b)` where `a` and `b` are structured tables — the default `==` does shallow equality; `assert.same` does deep equality and produces a diff on failure.
- Shared setup duplicated across multiple test cases where `before_each` would centralise it.
- Error-path tests using `pcall` + branch manually rather than `assert.has_error(f, pattern)`.

## Bad

```lua
-- tests/parser_tests.lua
local parser = require("parser")

assert(parser.parse("1 + 2") == 3)
assert(parser.parse("10 / 2") == 5)

local ok, err = pcall(parser.parse, "not math")
assert(not ok)
assert(err:match("invalid expression"))
```

The file runs, but a failure only tells you "assertion failed at line N". No structure, no reporter diff.

## Good

```lua
-- spec/parser_spec.lua
local parser = require("parser")

describe("parser", function()
  describe("arithmetic", function()
    it("adds two numbers", function()
      assert.equal(3, parser.parse("1 + 2"))
    end)

    it("divides cleanly", function()
      assert.equal(5, parser.parse("10 / 2"))
    end)
  end)

  describe("error cases", function()
    it("reports invalid expressions", function()
      assert.has_error(function() parser.parse("not math") end,
                       "invalid expression")
    end)
  end)
end)
```

Failures surface with the `describe` path, the `it` name, and a diff. Setup can be factored into `before_each` if needed. The file reads as documentation.

## When This Applies

- All test files committed to a Lua project.
- New regression tests added after a fix.
- Port work when bringing legacy Lua into a project with CI.

## When This Does Not Apply

- One-off check scripts run manually during debugging that are deleted afterwards.
- In-module self-checks using bare `assert` to enforce invariants at load time (these are not test suites; they are preconditions).
- Frameworks with their own test conventions (`luaunit`, `lust`); apply the equivalent structural discipline.

## Further Reading

- busted documentation (<https://olivinelabs.com/busted/>)
- `luaunit` docs — similar BDD structure if busted is unavailable (<https://github.com/bluebird75/luaunit>)
- Lua Style Guide — Testing (<https://github.com/Olivine-Labs/lua-style-guide>)
- IN-EX-TEST-001 — Elixir strong-assertion discipline mirrors this rule's intent
