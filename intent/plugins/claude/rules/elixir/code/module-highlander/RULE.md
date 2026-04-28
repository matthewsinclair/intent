---
id: IN-EX-CODE-006
language: elixir
category: code
severity: warning
title: Module-level Highlander (one canonical home per concern)
summary: >
  Each concern — calculation, validation, parsing, formatting, domain action —
  has one module that owns it. Call that module from everywhere else. Never
  duplicate a validation regex, a date-formatter, or a business rule across
  two modules because one caller was easier to copy than to route.
principles:
  - highlander
applies_when:
  - "Adding a new function that performs a calculation, validation, parsing, or domain action"
  - "Noticing the same regex, constant, or helper appearing in two modules"
  - "Copying a function from one module to another because 'we need it here too'"
applies_to:
  - "lib/**/*.ex"
does_not_apply_when:
  - "Internal helpers specific to a module's implementation (truly private, not reused)"
  - "Tiny formatters (`def upper(s), do: String.upcase(s)`) where centralising is more cost than benefit"
  - "Framework-required callback names that must exist in each module (`mount/3`, `render/1`)"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-EX-CODE-001
aliases: []
tags:
  - elixir
  - highlander
  - module-boundaries
status: active
version: 1
---

# Module-level Highlander

"There can be only one." A concern belongs to one module, and that module is where everybody goes to get it. Email-address validation lives in one place. The ISO 8601 date formatter lives in one place. The "is this order cancellable?" predicate lives in one place. When the rule changes, you change it once and every caller gets the new behaviour.

## Problem

Three failure modes when Highlander is broken:

1. **Divergent copies.** `AccountsModule` validates emails with one regex; `WebhooksModule` validates them with a slightly different one. A user with `+tag@example.com` passes one and fails the other. Nobody realises until a customer reports missing notifications.
2. **Fix-in-one-only.** A bug is discovered in the email regex. Someone fixes it in `AccountsModule`. The `WebhooksModule` copy stays broken for six months because nobody grep'd for "email_regex" across the tree.
3. **Silent drift.** Two modules each have their own `price_with_tax/2`. One charges VAT at 20%, the other at 15% because it was forked before a tax change. The tests pass (each module's tests use its own function); the revenue does not.

Elixir's answer is the same as the agnostic answer: one module per concern, explicit `alias` / fully qualified calls from callers, no copies.

## Detection

Signals:

- Two modules with identically-named functions doing identical work (`is_valid_email/1`, `format_timestamp/1`, `cancellable?/1`).
- A regex, magic number, or business constant appearing in two or more modules.
- A comment in module B saying "copied from module A — keep in sync" (keeping in sync by comment is the definition of this antipattern).
- A helper module proliferation: `Accounts.Helpers`, `Webhooks.Helpers`, `Billing.Helpers`, each with overlapping utilities.

No greppable proxy is authoritative for this rule — the concern is fundamentally cross-file ("the same function name implementing the same logical concern in two modules"), which a single-file regex cannot express. A naive `def name(...)` pattern would false-positive on every public function definition, including behaviour-mandated callbacks (`Application.start/2`, `GenServer.init/1`, `Phoenix.LiveView.mount/3`) where the duplication is contractually required, not a Highlander violation. Apply this rule via the LLM-driven `critic-elixir` subagent during `/in-review`, which can read multiple files and reason about whether two same-named functions encode the same business rule.

The reliable structural signal is "if this concern's rule changes, how many places do I have to edit?" The answer must be one.

## Bad

See `bad.exs` for the runnable form. Inline:

```elixir
defmodule Accounts do
  def valid_email?(s), do: Regex.match?(~r/^[^@\s]+@[^@\s]+\.[^@\s]+$/, s)
end

defmodule Webhooks do
  # slightly different regex — missed when Accounts was updated last year
  def valid_email?(s), do: Regex.match?(~r/^\S+@\S+$/, s)
end
```

Two modules, two slightly-different regexes, two versions of "valid email".

## Good

See `good.exs` for the runnable form. Inline:

```elixir
defmodule EmailAddress do
  @moduledoc "Canonical home for email-address validation and normalisation."

  @regex ~r/^[^@\s]+@[^@\s]+\.[^@\s]+$/

  def valid?(s), do: Regex.match?(@regex, s)
  def normalise(s) when is_binary(s), do: String.downcase(String.trim(s))
end

defmodule Accounts do
  def register(%{email: email}) do
    if EmailAddress.valid?(email), do: {:ok, EmailAddress.normalise(email)}, else: {:error, :invalid_email}
  end
end

defmodule Webhooks do
  def deliver(%{to: email} = event) do
    if EmailAddress.valid?(email), do: {:ok, event}, else: {:error, :invalid_email}
  end
end
```

One regex, one module, two callers. Fixing the regex updates both callers automatically.

## When This Applies

- Any new function that performs a calculation, parse, validation, or domain action.
- Any existing function whose name appears in more than one module and does similar work.
- Constants, regexes, limits, timeouts — "magic values" that encode a business rule.

## When This Does Not Apply

- **Truly private helpers.** `defp`s that support a single module's implementation and are not reusable. If it would not make sense to call it from another module, it stays private.
- **Trivial formatters.** `def upper(s), do: String.upcase(s)` duplicated in two modules is not a Highlander issue; it is just using `String.upcase/1` inline. Extract only when the logic is non-trivial _or_ the rule could change.
- **Framework callbacks.** Every LiveView has `mount/3`; every Controller has `action/2`. These are contractual duplicates, not Highlander violations.

A good test: "if the underlying rule changed tomorrow, would I need to edit more than one file?" If yes, the rule has no Highlander home; extract one.

## Further Reading

- [Intent `IN-AG-HIGHLANDER-001`](../../../agnostic/highlander/RULE.md) — the cross-language rule.
- [Intent `MODULES.md`](../../../../../../llm/MODULES.md) — Intent's own module registry, the mechanism that enforces Highlander at the project level.
