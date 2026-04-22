---
id: IN-LU-CODE-003
language: lua
category: code
severity: warning
title: Metatables sparingly, document when used
summary: >
  Every `setmetatable` call must carry a comment explaining which hook is
  being installed and why. Metatables are action-at-a-distance — silent
  `__index` / `__newindex` / operator overloads surprise readers who
  expected ordinary table access.
principles:
  - public-interface
applies_when:
  - "Installing `__index`, `__newindex`, `__call`, `__add`, `__eq`, `__tostring`, or other metamethods"
  - "Using metatables to implement inheritance, proxies, or lazy fields"
  - "Writing libraries others will consume (metatable surprises bite consumers hardest)"
applies_to:
  - "**/*.lua"
does_not_apply_when:
  - "Idiomatic uses in frameworks (Love2D `Object`, OO via metatable chains in Defold) where the pattern is the convention"
  - "Operator overloading for domain types (vectors, currency) where the overload is the point"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-LU-CODE-002
aliases: []
tags:
  - lua
  - metatables
  - action-at-a-distance
status: active
version: 1
---

# Metatables sparingly, document when used

A reader can tell what a table does by looking at it — unless it has a metatable. Every `setmetatable` call changes that, so label it.

## Problem

Metatables let Lua customise fundamental operations: indexing, assignment, iteration, arithmetic, equality, stringification. They are powerful precisely because they are invisible at the call site — `obj.name` with a metatable installed might walk a lookup chain, fetch a computed value, or proxy to a remote object. The caller sees normal table access. A future reader debugging a mysterious behaviour has no way to tell from the call site that anything unusual is happening; they must go find the metatable.

The solution is not "never use metatables" — they are central to Lua's design for polymorphism, operator overloading, and controlled default-provision. The solution is to name them. Every `setmetatable` call carries a comment: which metamethods are installed, what they do, why they are needed. The cost is one line of comment; the benefit is that readers can navigate without trial-and-error.

## Detection

Static signals:

- `setmetatable(obj, ...)` calls with no adjacent comment (within 2 lines) explaining the installed metamethods.
- Metatables assembled inline (`setmetatable({}, {__index = ...})`) in code that is not obviously a constructor.
- Chained metatables where each level is an extension of the previous — frequently a sign of inheritance ceremony copied from other languages.
- Proxy patterns (`__index` returning values from another object) in code that has no documentation of the proxying.

## Bad

```lua
local LazyConfig = {}
LazyConfig.__index = function(t, key)
  if key == "database_url" then
    return os.getenv("DATABASE_URL")
  end
  return rawget(t, key)
end

local config = setmetatable({}, LazyConfig)

print(config.database_url)
```

The caller sees a table access. The metatable silently reroutes it to an environment variable. A reader of `config.database_url` has no way to know this without running the code or finding the metatable.

## Good

```lua
-- Config table with lazy environment-variable lookup.
-- Metatable installs __index which falls back to os.getenv for
-- selected keys. Used so call sites can write config.database_url
-- but real values come from the environment.
local LazyConfig = {}
LazyConfig.__index = function(t, key)
  if key == "database_url" then
    return os.getenv("DATABASE_URL")
  end
  return rawget(t, key)
end

local config = setmetatable({}, LazyConfig)
print(config.database_url)
```

Same code. The comment block names what the metatable does. A reviewer can decide in 10 seconds whether the access is ordinary.

## When This Applies

- Every `setmetatable` call in committed code.
- Library authors: metatables in public-consumable objects are the riskiest because consumers cannot see the wrapper without reading source.
- Any use of `__index`, `__newindex`, `__call`, `__tostring` — especially when the hook differs from what the reader would guess.

## When This Does Not Apply

- Canonical framework idioms where the metatable is a known convention: Love2D's `Object`, certain Neovim plugin frameworks, Defold scripts. A one-time comment in the project README is enough.
- Operator overloading for explicit domain types: `Vec2 + Vec2` via `__add` in a graphics library is idiomatic and self-documenting.
- Tests or scratch scripts that are not read by anyone else.

## Further Reading

- Programming in Lua (Ierusalimschy), ch. 13 "Metatables and Metamethods" (<https://www.lua.org/pil/13.html>)
- Lua 5.3 Reference Manual §2.4 "Metatables and Metamethods" (<https://www.lua.org/manual/5.3/manual.html#2.4>)
- "Lua Style Guide" (<https://github.com/Olivine-Labs/lua-style-guide>)
- IN-LU-CODE-002 — tables-as-structs is the default; metatables are the escape hatch
