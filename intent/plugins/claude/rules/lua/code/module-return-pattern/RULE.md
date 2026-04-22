---
id: IN-LU-CODE-005
language: lua
category: code
severity: warning
title: Module return pattern
summary: >
  Every module ends with `return M` exposing a local table of public
  functions. Never rely on side effects on `_G` for modules to be usable;
  callers should work purely through the return value of `require`.
principles:
  - public-interface
  - no-silent-errors
applies_when:
  - "Writing a new Lua module that other files will require"
  - "Refactoring a global-heavy file into a proper module"
  - "Embedding Lua in a host application where module isolation matters"
applies_to:
  - "**/*.lua"
does_not_apply_when:
  - "Top-level application scripts that are entry points, not modules"
  - "Dot-files (`init.lua` for host apps) where the host's convention overrides"
references:
  - IN-AG-HIGHLANDER-001
  - IN-LU-CODE-001
related_rules:
  - IN-LU-CODE-001
aliases: []
tags:
  - lua
  - modules
  - require
status: active
version: 1
---

# Module return pattern

A module is what you `require`. Lua's conventional shape is "build a local table, attach functions, return it". Anything else turns globals into your module's API contract.

## Problem

Lua's `require(name)` loads the file, executes it, and caches its return value. If the file returns nothing but declares global functions, the caller has to know the global names out of band and trust that no other module uses the same names. The module's "API" is a side-effect on `_G`, which means dependencies are implicit, load order matters, and teardown is impossible.

The canonical shape — `local M = {}; function M.foo() ... end; return M` — gives the caller a handle they control. `require`'s cache keeps the table identity stable across multiple `require` calls, so calling modules can share state through it deliberately. Name collisions are impossible because every caller chose their own local name.

## Detection

Static signals:

- Files that end without a `return` statement.
- Modules that define `function module_name.foo(...) end` where `module_name` is a global (no `local` declaration).
- Files relying on the side effect of being `require`d rather than its return value.
- Inconsistent return shapes inside a single project: some modules return tables, some return functions, some return nil.

## Bad

```lua
-- logger.lua
log = {}
function log.info(msg) print("[INFO] " .. msg) end
function log.warn(msg) print("[WARN] " .. msg) end

-- usage from another file
require("logger")
log.info("hello")
```

`log` is a global. Another module loading `logger.lua` cannot pick a different name. Any other file creating `log = ...` silently clobbers it.

## Good

```lua
-- logger.lua
local M = {}

function M.info(msg) print("[INFO] " .. msg) end
function M.warn(msg) print("[WARN] " .. msg) end

return M

-- usage from another file
local logger = require("logger")
logger.info("hello")
```

Caller picks the local name. Nothing leaks. `require("logger")` returns the same table every time, so modules that want to share state can coordinate through it intentionally.

## When This Applies

- Every `.lua` file intended to be `require`d.
- Configuration modules that return tables (hosts like Neovim consume them via `require`).
- Libraries published to LuaRocks or vendored into host applications.

## When This Does Not Apply

- Application entry points executed with `lua script.lua` — they are not `require`d; no `return` is expected.
- Conventions imposed by a host application: Neovim's `init.lua`, some game engines' `main.lua`. Follow the host's contract there.
- Interactive REPL snippets.

## Further Reading

- Programming in Lua (Ierusalimschy), ch. 15 "Packages" (<https://www.lua.org/pil/15.html>)
- Lua 5.3 Reference Manual §6.3 "Modules" (<https://www.lua.org/manual/5.3/manual.html#6.3>)
- Lua Style Guide (<https://github.com/Olivine-Labs/lua-style-guide>)
- IN-LU-CODE-001 — `local` discipline is the foundation on which the module return pattern rests
