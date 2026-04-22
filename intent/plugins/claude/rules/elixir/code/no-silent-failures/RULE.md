---
id: IN-EX-CODE-005
language: elixir
category: code
severity: critical
title: No silent failures
summary: >
  Never rescue and swallow, never ignore a returned `{:error, _}`, never use
  `_ = fallible_call()` to suppress a warning. Every failure is either handled
  deliberately (propagated, retried, logged-and-tagged) or allowed to crash.
  "It didn't work but nobody notices" is the worst possible outcome.
principles:
  - no-silent-errors
applies_when:
  - "Calling a function that can return `{:error, _}`"
  - "Wrapping code in `try/rescue`"
  - "Assigning a result to `_` or an unused variable"
  - "Catching all exceptions (`rescue _ -> ...`) without inspecting"
applies_to:
  - "lib/**/*.ex"
does_not_apply_when:
  - "Returning `{:error, reason}` from the calling function is the intentional response"
  - "`try/rescue` where the rescued exception is inspected, logged, and converted to a tagged return"
  - "Fire-and-forget notifications where the business-level contract genuinely does not care if delivery failed (rare — usually wrong)"
references:
  - IN-AG-NO-SILENT-001
related_rules:
  - IN-EX-CODE-002
  - IN-EX-CODE-004
aliases: []
tags:
  - elixir
  - error-handling
  - no-silent-errors
status: active
version: 1
---

# No silent failures

Elixir's "let it crash" philosophy requires that failures be _visible_ — to the supervisor, to the caller, to the operator. Rescuing an exception and returning `:ok` anyway; pattern-matching only on `{:ok, _}` and letting `{:error, _}` fall into an `_` clause; piping a result into something that ignores it — all are ways of turning a loud failure into a silent one. The system keeps running but the invariant is broken.

## Problem

Four failure modes:

1. **`try/rescue _ -> :ok`.** Rescuing the raised exception, discarding it, and returning `:ok` turns every crash into a no-op success. The call looks clean in production logs; the bug surfaces days later as missing data.
2. **Partial pattern match.** `{:ok, _} = risky_call()` crashes loudly when the call returns `{:error, _}`, which is often what you want. But `case risky_call() do; {:ok, v} -> v; _ -> nil end` silently swallows every error and converts it to `nil`, joining the family of "`nil` means two things".
3. **Unused result binding.** `_ = side_effecting_call()` or `side_effecting_call()` with no binding at all — the warning goes away but nobody checks if the side effect actually happened. The compiler says "no unused result" and you say "yes, but I didn't handle the error either".
4. **`Task.async |> Task.await |> rescue _`.** Spawning async work and rescuing any failure it raises converts "this background job died" into "we have no idea if it ran".

The pattern is always the same: the call can fail, the code pretends it cannot, and the failure disappears from operational visibility.

## Detection

Signals:

- `rescue _ ->` (catch-all rescue without inspecting the exception)
- `rescue _ -> :ok` or `rescue _ -> nil` or `rescue _ -> {:ok, _}`
- `_ = call_that_can_fail(...)` on its own line
- A `case` whose fallback clause is `_ ->` and which returns a value suspiciously close to the success shape
- `{:ok, _} = call()` in contexts where you can argue the right behaviour is "return an error tuple", not "crash the process" (a harder one — Critic flags for review)
- `try do ... rescue _ -> ... end` where the rescue branch does not log, inspect, or re-tag the exception

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
grep -rnE 'rescue _ -> (\:ok|nil)' lib/
grep -rnE '^[[:space:]]+_ = [a-z_]+' lib/
```

The reliable structural signal is "if this call fails, does anything in the system know it failed?"

## Bad

See `bad.exs` for the runnable form. Inline:

```elixir
def record_event(event) do
  try do
    Repo.insert!(%Event{payload: event})
    :ok
  rescue
    _ -> :ok
  end
end

def process(params) do
  _ = notify_stakeholders(params)
  {:ok, :done}
end

def charge(card) do
  case Payments.charge(card) do
    {:ok, receipt} -> receipt
    _ -> nil
  end
end
```

Every failure is converted to a success-shaped return. The caller cannot tell the difference.

## Good

See `good.exs` for the runnable form. Inline:

```elixir
def record_event(event) do
  try do
    {:ok, Repo.insert!(%Event{payload: event})}
  rescue
    error ->
      Logger.error("record_event failed: #{inspect(error)}")
      {:error, {:event_write_failed, Exception.message(error)}}
  end
end

def process(params) do
  with {:ok, :notified} <- notify_stakeholders(params) do
    {:ok, :done}
  end
end

def charge(card) do
  case Payments.charge(card) do
    {:ok, receipt} -> {:ok, receipt}
    {:error, reason} -> {:error, {:payment_declined, reason}}
  end
end
```

Every failure has a handler: log it, tag it, return it. The caller knows what happened.

## When This Applies

- Any `try/rescue` block.
- Any call to a function that can return `{:error, _}`.
- Any side-effecting call whose result is being discarded.
- Any `case` with an `_ ->` fallback where the call has a failure mode.

## When This Does Not Apply

- **Intentional propagation.** `{:ok, v} <- call()` inside a `with` is not silent — the `{:error, _}` short-circuits the `with` and is returned to the caller. That is handling by delegation.
- **Rescue with attribution.** `rescue e in SomeError -> {:error, {:specific, e.message}}` is loud: the exception is caught, inspected, typed, and returned as data.
- **Fire-and-forget with documented contract.** `Logger.info("...")` is fire-and-forget by design — logger failures should not take down the business path. Flag any non-Logger call that relies on this exemption.

A good test: "if this fails silently in production at 3am, does the oncall engineer find out?" If no, you have a silent failure.

## Further Reading

- [Intent `IN-AG-NO-SILENT-001`](../../../agnostic/no-silent-errors/RULE.md) — the agnostic version of this rule.
- [Intent `IN-EX-CODE-002` tagged-tuple-returns](../tagged-tuple-returns/RULE.md) — tagged tuples are the return shape this rule presumes.
- [Intent `IN-EX-CODE-004` with-for-railway](../with-for-railway/RULE.md) — `with` propagates failures without writing handlers.
- [Elixir docs — `try`, `rescue`, `catch`](https://hexdocs.pm/elixir/Kernel.SpecialForms.html#try/1) — language reference.
