---
id: IN-LU-CODE-001
language: lua
category: code
severity: critical
title: local over global
summary: >
  Declare every variable with `local`. Assignment without `local` creates
  or mutates a global, which collides with sibling modules, Lua's
  standard library, and any host application's embedded scripts.
principles:
  - honest-data
  - no-silent-errors
applies_when:
  - "Writing any Lua module that will be loaded alongside other modules"
  - "Embedding Lua inside a host application (game engine, Neovim, Redis)"
  - "Adapting a snippet from a tutorial that uses bare assignments"
applies_to:
  - "**/*.lua"
does_not_apply_when:
  - "Deliberately exposing a global API for host-application introspection"
  - "Interactive REPL experimentation (not committed code)"
references:
  - IN-AG-HIGHLANDER-001
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-LU-CODE-005
aliases: []
tags:
  - lua
  - globals
  - scoping
status: active
version: 1
---

# local over global

Lua assignment defaults to global. `local` is the single keyword that stops every module from accidentally colonising the shared environment.

## Problem

In Lua, `x = 5` inside a module sets a field on the global table (`_G.x`). Two modules that each write `state = {}` share the same `state`. A tutorial that says `function greet(name) ... end` creates `_G.greet`, which then overwrites any identically named helper from a sibling. Inside a host application — Neovim, Redis, a game engine — the host often exposes globals (`vim`, `redis`, `love`) that a missing `local` can shadow or clobber without warning.

Lua does not warn about any of this. The collision surfaces later as "my function is suddenly broken" or "the plugin stopped working after I added a new module". The single-keyword fix is free at authoring time and nearly impossible to retrofit without a linter.

## Detection

Static signals:

- Assignments at module top-level without `local` (`x = 5`, `M = {}`).
- `function name() end` at module scope — this is `name = function() end`, which is global.
- Loop-local variables missing `local`: `for i = 1, 10 do s = s .. tostring(i) end` — `s` leaks as a global.
- Variables introduced inside `if` blocks without `local`.

Linters: `luacheck` reports unused and unscoped globals out of the box; `.luacheckrc` can enforce a strict global policy.

## Bad

```lua
-- grep/fuzzy.lua
function match(pattern, subject)
  result = string.find(subject, pattern)
  return result ~= nil
end

counter = 0
function incr()
  counter = counter + 1
end
```

`match`, `result`, `counter`, and `incr` are all globals. Another module's `counter` stomps this one.

## Good

```lua
-- grep/fuzzy.lua
local M = {}

function M.match(pattern, subject)
  local result = string.find(subject, pattern)
  return result ~= nil
end

local counter = 0
function M.incr()
  counter = counter + 1
end

return M
```

Nothing leaks. The caller writes `local fuzzy = require("grep.fuzzy")` and reaches in via `fuzzy.match`.

## When This Applies

- Every module that will be `require`d by another module.
- Every plugin for Neovim, every script embedded in Redis, every configuration file for a game engine using Lua.
- Every function declaration inside a module unless the intent is to expose a global callable (rare, document when used).

## When This Does Not Apply

- Host applications that document specific globals as the contract (`vim.cmd`, `love.graphics`) — those are already declared by the host, not by your module.
- One-off scripts executed once and discarded; even then, `local` costs nothing.
- REPL experimentation: the REPL is a single global scope by design.

## Further Reading

- Programming in Lua (Ierusalimschy), ch. 4 "Statements — Local Variables and Blocks" (<https://www.lua.org/pil/4.2.html>)
- Lua 5.3 Reference Manual, §3.3.7 "Local Declarations" (<https://www.lua.org/manual/5.3/manual.html#3.3.7>)
- luacheck (<https://github.com/mpeterv/luacheck>)
- IN-AG-HIGHLANDER-001 — name collisions are a Highlander violation at the module level
- IN-AG-NO-SILENT-001 — global writes are silent errors by construction
