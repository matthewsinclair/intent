---
id: IN-LU-CODE-004
language: lua
category: code
severity: critical
title: pcall for error boundaries, tagged results
summary: >
  Wrap fallible calls in `pcall` at the boundary where you need to recover.
  Return tagged results (`ok, result_or_err`) so callers can branch
  explicitly. Never use `error(...)` as a general control-flow mechanism
  and never ignore `pcall`'s first return value.
principles:
  - no-silent-errors
applies_when:
  - "Calling a library function that may `error(...)` on bad input"
  - "Implementing error boundaries around user-provided code (plugins, scripts)"
  - "Writing a module API that needs to express 'this can fail' in the return value"
applies_to:
  - "**/*.lua"
does_not_apply_when:
  - "Programmer errors that should crash and surface (invariant violations)"
  - "Startup code where failure is immediately fatal and a stack trace is wanted"
  - "Tests that use `assert` / `error` as the failure mechanism"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-LU-CODE-001
aliases: []
tags:
  - lua
  - error-handling
  - pcall
status: active
version: 1
---

# pcall for error boundaries, tagged results

Lua errors are exceptional. Either let them propagate and crash, or catch them deliberately with `pcall` and return a tagged result.

## Problem

Lua's error model is minimal: `error(msg)` raises, errors propagate up the call stack via `longjmp`-like mechanics, and `pcall(f, ...)` is the only way to catch them. Unlike Python's `try`/`except` or Java's typed exceptions, there is no structural help — a caller who forgets to wrap a risky call gets crashed; a caller who wraps too broadly silently swallows bugs.

The discipline: use `pcall` at exactly the boundary where recovery is meaningful. Inside the boundary, let `error` raise. At the boundary, call `pcall`, then branch explicitly on the `ok` flag, then return a tagged result to your own caller (`return true, value` or `return false, reason`). The boundary is explicit; above it, callers can `assert`, retry, log, or propagate on their own terms.

The worst shape is `pcall(f)` with the return values discarded: `pcall` becomes a silent error swallower that lets the program continue in an invalid state. Just as bad: `local _, _ = pcall(f)` with both returns thrown away.

## Detection

Static signals:

- `pcall(f, ...)` where the return values are discarded or bound to `_`.
- `pcall` wrapping a call that cannot `error` (defensive over-use).
- Module entry points that call risky libraries without any `pcall` — exception propagates to the embedding host.
- Error messages passed as bare strings with no structure: `error("bad")` vs `error({code = "bad_input", msg = "..."})`.
- `xpcall` without a handler that actually inspects the error.

## Bad

```lua
local function load_config(path)
  local content = io.open(path):read("*a")
  return dkjson.decode(content)
end

-- Caller.
local config = load_config("/etc/app.json")
```

`io.open` returns `nil` on failure; calling `:read` on `nil` errors; `dkjson.decode` errors on bad JSON. Nothing caught, nothing tagged — a missing file crashes the host application.

## Good

```lua
local dkjson = require("dkjson")

local M = {}

function M.load_config(path)
  local file, open_err = io.open(path, "r")
  if not file then
    return false, "open failed: " .. open_err
  end
  local content = file:read("*a")
  file:close()

  local ok, parsed = pcall(dkjson.decode, content)
  if not ok then
    return false, "json parse failed: " .. tostring(parsed)
  end
  return true, parsed
end

return M

-- Caller.
local ok, config_or_err = M.load_config("/etc/app.json")
if not ok then
  log.warn("config load failed: %s", config_or_err)
  return defaults()
end
```

Every failure has a named path. The caller branches on `ok`. No crash leaks out.

## When This Applies

- Any function that wraps a library known to call `error(...)` (dkjson, luasocket, host-application APIs).
- Module public APIs that want to express "this can fail" structurally, without using `error`.
- Plugin / extension architectures running user-supplied code.
- File I/O, network I/O, JSON parsing, regex compilation.

## When This Does Not Apply

- Invariant violations (a broken assertion about internal state) — `assert` / `error` crashes loudly, which is what you want.
- Startup: config file unreadable at program launch, let it crash with a stack trace.
- Tests: `assert` is the failure mechanism; `pcall` wrapping defeats the test.
- Hot paths where `pcall`'s cost (stack-unwind setup) matters and the call is known to be safe.

## Further Reading

- Programming in Lua (Ierusalimschy), ch. 8.4 "Error Handling and Exceptions" (<https://www.lua.org/pil/8.4.html>)
- Lua 5.3 Reference Manual §6.1 "Basic Functions" — `pcall`, `xpcall`, `error` (<https://www.lua.org/manual/5.3/manual.html#6.1>)
- Lua Wiki, "Error Handling" (<http://lua-users.org/wiki/ErrorHandling>)
- IN-AG-NO-SILENT-001 — ignoring `pcall`'s first return is a silent-error flagship example
