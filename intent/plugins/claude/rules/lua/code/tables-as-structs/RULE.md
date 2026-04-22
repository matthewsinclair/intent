---
id: IN-LU-CODE-002
language: lua
category: code
severity: recommendation
title: Tables as structs, not OOP reflex
summary: >
  Lua tables are the universal data structure. Use them as plain records
  (`{name = "Alice", age = 30}`) by default. Reach for metatables and
  class-like inheritance only when polymorphism is genuinely needed.
principles:
  - pfic
applies_when:
  - "Representing domain data (users, events, configuration)"
  - "Modelling a small set of related values together"
  - "Porting an OOP design from another language to Lua"
does_not_apply_when:
  - "Genuine polymorphism where dispatch on type is a natural fit"
  - "Host-framework conventions that require metatable-based classes (Love2D, Defold)"
  - "Libraries where class-like semantics are documented expectations"
references:
  - IN-AG-PFIC-001
related_rules:
  - IN-LU-CODE-003
aliases: []
tags:
  - lua
  - tables
  - style
status: active
version: 1
---

# Tables as structs, not OOP reflex

A Lua table is already a struct, a map, and a namespace. Wrap it in `setmetatable` only when the wrapping earns its keep.

## Problem

Programmers arriving from Java, Ruby, or Python often port class hierarchies into Lua by stacking metatables (`__index`) into inheritance chains. The result compiles and runs, but the idioms fight the language: the programmer writes `Animal:new()`, `Dog:new()` with `self.__index = self` boilerplate, and then pays a dispatch cost on every field access. Lua's native style is flat data plus free functions; "classes" are a pattern built on top, not a primitive.

The cost is not only conceptual. Metatable lookup chains make debugging harder (`self.name` silently walks `__index` links), and stack traces become noisy. Reserving metatables for real polymorphism (closed-set dispatch, operator overloading) keeps the 90% case simple.

## Detection

Static signals:

- `setmetatable(obj, {__index = Parent})` chains more than one level deep.
- `function Class:new(...)` constructors followed by `function Class:method(...)` definitions where there is only one "class" in play.
- Tables with method fields but no genuine variation in behaviour across instances.
- OOP idioms (`self`, `super`, `instance_of`) in code that only ever instantiates one subtype.

## Bad

```lua
local Animal = {}
Animal.__index = Animal

function Animal:new(name)
  local self = setmetatable({}, Animal)
  self.name = name
  return self
end

function Animal:describe()
  return "animal named " .. self.name
end

local Dog = setmetatable({}, {__index = Animal})
Dog.__index = Dog

function Dog:new(name, breed)
  local self = Animal.new(self, name)
  self.breed = breed
  return self
end
```

Two metatables, a constructor chain, `self.__index = self` incantations — for what is just a pair of records with a name and optional breed.

## Good

```lua
local M = {}

function M.new_animal(name)
  return {kind = "animal", name = name}
end

function M.new_dog(name, breed)
  return {kind = "animal", name = name, breed = breed}
end

function M.describe(a)
  if a.breed then
    return string.format("%s (%s)", a.name, a.breed)
  end
  return a.name
end

return M
```

Plain tables carry the data. Free functions operate on them. Anyone can `print(a.name)` without stepping through metatables.

## When This Applies

- Modelling domain records: user, event, config, message, request.
- Bundling related function outputs: return `{ok = true, value = ...}` rather than crafting a class.
- Migrating OOP-heavy code from another language: resist the urge to replicate the class shape.

## When This Does Not Apply

- Genuine polymorphism across many types: an AST node dispatcher, an event handler registry with open-ended implementations.
- Frameworks where class-like idioms are the convention: Love2D's `Object`, Defold scripts, some Neovim plugin frameworks.
- Operator overloading: `__add`, `__eq`, `__tostring` are the right tool for vector maths, currency, time deltas.

## Further Reading

- Programming in Lua (Ierusalimschy), ch. 16 "Object-Oriented Programming" (<https://www.lua.org/pil/16.html>)
- Lua Style Guide (<https://github.com/Olivine-Labs/lua-style-guide>)
- Roberto Ierusalimschy, "Programming in Lua" (book) — the chapter deliberately presents OO as an optional layer
- IN-LU-CODE-003 — metatables sparingly complements this rule for the cases where they _are_ warranted
