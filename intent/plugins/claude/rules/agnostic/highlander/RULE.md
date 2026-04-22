---
id: IN-AG-HIGHLANDER-001
language: agnostic
category: architecture
severity: critical
title: There can be only one
summary: >
  Never duplicate code paths, modules, or logic for the same concern. Two
  implementations drift over time: one gets a bug fix, the other does not.
  Before creating anything new, check the module registry.
principles:
  - highlander
applies_when:
  - "Validation, formatting, error handling, configuration loading, or I/O wrappers that could be implemented in more than one place"
  - "A new helper function whose responsibility overlaps an existing module"
  - "A CLI subcommand, skill, or rule that restates logic already owned elsewhere"
does_not_apply_when:
  - "Localisation files where the same key is legitimately translated into many languages"
  - "Test fixtures where repeated setup is clearer than an over-extracted helper"
  - "Vendor-specific adapters where each third-party integration is genuinely its own concern"
references: []
related_rules:
  - IN-AG-PFIC-001
  - IN-AG-THIN-COORD-001
concretised_by:
  - IN-EX-CODE-006
  - IN-EX-TEST-007
  - IN-RS-CODE-002
  - IN-SW-CODE-004
  - IN-LU-CODE-005
aliases: []
status: active
version: 1
---

# There can be only one

One concern, one module. Duplicate paths drift over time; the system behaves inconsistently depending on which path executes.

## Problem

Two implementations of the same concern diverge. A validation regex lives in `Users.Validators.Email` and a near-copy lives in `Admin.Utils.EmailCheck`. A reviewer lands a bug fix in one. The other accepts addresses the fixed one now rejects. Nothing breaks loudly; the system just becomes inconsistent depending on which module a given call site happened to import.

Three failure modes recur:

1. **Silent inconsistency.** Two coordinators call two different implementations. User-facing behaviour diverges across screens, endpoints, or pipelines with no error at the boundary.
2. **Partial fixes.** A security patch lands in one copy, not the other. Audits later find the unpatched clone.
3. **Invisible bloat.** The codebase grows without corresponding capability growth. Reviewers stop reading "similar-looking" modules because there are now too many.

The root cause is skipping the check: a contributor needs a utility, does not see one nearby, and writes a new one instead of registering their need against an existing registry of concerns.

## Detection

Look for modules, functions, or configuration blocks with overlapping responsibilities. Signals:

- Two modules whose names are near-synonyms (`Utils.Email` and `Validators.Email`; `http_client.py` and `api_client.py`).
- A function in module `A` and a near-identical function in module `C`, each called from different coordinators.
- The same regex, format string, or constant repeated verbatim in three or more files.
- A new PR that adds a helper function whose doctoring reads like the doctoring of an existing helper.

Structural signal: the project's module registry (Intent's `MODULES.md`, a module manifest in other ecosystems) has no entry for the new concern, yet the concern is being implemented.

Language-specific detection heuristics live in the concretising rules.

## Bad

Textual (agnostic tier — see `concretised_by:` for runnable examples):

```
# Module A
defmodule MyApp.Users.Validators do
  def valid_email?(email), do: Regex.match?(~r/^[^@]+@[^@]+\.[^@]+$/, email)
end

# Module B (created later, by a different contributor)
defmodule MyApp.Admin.EmailCheck do
  def check(email), do: String.contains?(email, "@") and String.contains?(email, ".")
end
```

Two "is this email valid" implementations. One is stricter than the other. Users succeed in one flow, fail in the other, with the same input.

## Good

Textual:

```
# Single authoritative module, listed in the project module registry.
defmodule MyApp.Email do
  @moduledoc "Email validation. The only place email rules live."
  def valid?(email), do: Regex.match?(~r/^[^@]+@[^@]+\.[^@]+$/, email)
end

# Every call site imports the one module.
alias MyApp.Email
Email.valid?(params["email"])
```

Before writing module B, the contributor checks the registry, finds `MyApp.Email`, and extends it if needed.

## When This Applies

Any code concern that could be implemented in more than one place:

- Validation of domain primitives (emails, phone numbers, identifiers, money).
- Error formatting and user-facing message generation.
- Configuration loading and parsing.
- I/O wrappers around external APIs, databases, filesystems.
- Data transformations between canonical shapes (JSON → struct, row → record).

Pre-flight is mandatory: before creating a new module, check the registry; before adding a public function, grep for prior art in adjacent modules.

## When This Does Not Apply

- **Localisation.** Repeating a key across `en.yml`, `fr.yml`, `de.yml` is not duplication — it is the entire point of localisation.
- **Test fixtures.** A setup block that reads as five straightforward lines in each of three tests can be clearer than an extracted helper whose indirection readers must chase.
- **Vendor-specific adapters.** If a service integrates with Stripe, Shopify, and PayPal, each vendor's client module is genuinely a distinct concern even if the shape looks similar. Shared abstraction over truly distinct APIs is premature coupling.
- **Generated code.** Two generated files that happen to share structure because a schema generated both are not duplication; the schema is the one source.

The test for whether the exception applies is whether changing the behaviour of one copy would be a bug if the other copy did not change. If yes, it is duplication and this rule applies. If no (translations, fixtures, adapters), the exception holds.

## Further Reading

- Intent `CLAUDE.md` — "Check before you create", "Register before you code" project rules.
- Intent `intent/llm/MODULES.md` — the authoritative module registry; the concrete enforcer of this rule for the Intent codebase itself.
- Andy Hunt & Dave Thomas, _The Pragmatic Programmer_ — the DRY principle as originally articulated (broader than code duplication; covers knowledge duplication).
- Concretising rules: `IN-EX-CODE-006` (Elixir module Highlander), `IN-EX-TEST-007` (Highlander for tests — shared setup, not repeated setup), `IN-RS-CODE-002` (Rust ownership before clone), `IN-SW-CODE-004` (Swift access control narrowest), `IN-LU-CODE-005` (Lua module return pattern).
