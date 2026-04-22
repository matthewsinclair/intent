---
id: IN-EX-PHX-001
language: elixir
category: phoenix
severity: warning
title: Thin Phoenix controllers
summary: >
  Controllers parse params, call the domain, render. No business logic, no
  data transformation beyond what the template needs, no conditional branching
  on domain state. A controller action is one screen of code; if it grows
  longer, the logic belongs in the domain.
principles:
  - thin-coordinator
applies_when:
  - "Any Phoenix controller action (index, show, create, update, delete, custom)"
  - "An action that is about to grow past ~15 lines"
  - "An action that branches on domain state (role, status, visibility)"
applies_to:
  - "lib/**/controllers/**/*.ex"
does_not_apply_when:
  - "Extremely thin actions already (parse + call + render) — there is nothing to thin"
  - "Framework-level concerns (error wrapping, content negotiation) that genuinely belong in the controller layer"
  - "Fallback controllers that normalise errors — their purpose is orchestration of shape"
references:
  - IN-AG-THIN-COORD-001
related_rules:
  - IN-EX-ASH-001
  - IN-EX-LV-003
aliases: []
tags:
  - elixir
  - phoenix
  - thin-coordinator
status: active
version: 1
---

# Thin Phoenix controllers

A controller action is a shell: read the request, call the domain, render the response. When the shell grows conditional logic, domain rules start living in the web layer. Business logic in controllers is invisible to every non-HTTP caller — Oban jobs, mix tasks, GraphQL resolvers, other controllers — because it is encoded as an HTTP request/response shape, not as a domain contract.

## Problem

Three failure modes when controllers fatten:

1. **Invisible business logic.** A controller that computes "is this post publishable?" inline means that a non-controller caller (Oban worker, CLI, GraphQL) has to re-compute it. Two copies of the rule; two places to fix when the rule changes.
2. **Policy duplication.** A controller that checks `if user.role == :admin do ...` before calling the domain either duplicates Ash policies (if the domain has them) or installs policies in the wrong place (if it doesn't). Ash policies live on the resource; if the controller is doing authorisation, the policy is wrong or missing.
3. **Untestable flow.** A 50-line controller action mixes parameter parsing, business logic, error handling, and response shaping. A test that covers the business logic has to go through the full HTTP request/response stack. The same logic in a domain function is tested directly.

Controllers should be ugly in exactly one way: they talk to HTTP. Everything else belongs in the domain.

## Detection

Signals:

- A controller action longer than ~15 lines.
- `if`, `case`, or `cond` on domain state (role, status, visibility) inside a controller action.
- A controller action calling more than one domain function and coordinating between the results.
- `Ash.*` calls inside a controller (see `IN-EX-ASH-001`).
- Computed values (totals, filters, derived state) being built in the controller before rendering.
- A controller's `create/2` action that wraps `Accounts.register_user/1` with validation, geocoding, email-dispatch, and analytics — that is five concerns, not one.

Greppable proxy (not authoritative; Critic confirms by reading body):

```bash
# Actions longer than 20 lines are suspect
awk '/def (index|show|create|update|delete|new|edit)/{start=NR; name=$0} /^[[:space:]]+end$/ && start && NR-start > 20 {print FILENAME":"start" "name; start=0}' lib/*_web/controllers/**/*.ex 2>/dev/null
```

The reliable structural signal is "would a non-HTTP caller (Oban, CLI, GraphQL) need this logic too?" If yes, it belongs in the domain.

## Bad

Inline:

```elixir
def create(conn, %{"order" => params}) do
  # validate
  if params["amount"] > 1000 and conn.assigns.current_user.role != :admin do
    conn |> put_flash(:error, "Too high") |> redirect(to: ~p"/orders/new")
  else
    # charge, log, notify — all inline
    case Payments.charge(params) do
      {:ok, receipt} ->
        order = Repo.insert!(%Order{amount: params["amount"], receipt: receipt.id})
        MyApp.Mailer.deliver(order_confirmation(order))
        Analytics.track("order_created", order_id: order.id)
        redirect(conn, to: ~p"/orders/#{order.id}")

      {:error, reason} ->
        conn |> put_flash(:error, inspect(reason)) |> redirect(to: ~p"/orders/new")
    end
  end
end
```

Five concerns in one action: validation, payment, persistence, notification, analytics. None of these is HTTP-specific.

## Good

Inline:

```elixir
def create(conn, %{"order" => params}) do
  case MyApp.Orders.place_order(params, actor: conn.assigns.current_user) do
    {:ok, order} ->
      redirect(conn, to: ~p"/orders/#{order.id}")

    {:error, {:validation, reason}} ->
      conn |> put_flash(:error, reason) |> redirect(to: ~p"/orders/new")

    {:error, {:payment, reason}} ->
      conn |> put_flash(:error, "Payment failed: #{inspect(reason)}") |> redirect(to: ~p"/orders/new")
  end
end
```

The controller parses `params`, calls the domain, and shapes the response. All business logic lives in `MyApp.Orders.place_order/2`, where Oban workers and GraphQL resolvers can reuse it.

## When This Applies

- Every controller action. No exceptions for "this one is special".
- Fallback controllers when they are taking on business-logic responsibilities instead of shape normalisation.

## When This Does Not Apply

- **Controllers that are already thin.** `def show(conn, %{"id" => id}), do: render(conn, :show, post: Content.get_post!(id, actor: conn.assigns.current_user))` is the target state; there is nothing to thin.
- **Framework-layer wrapping.** Plugs that wrap every action with authentication, content negotiation, or CORS handling are not business logic; they are framework concerns.
- **Fallback controllers that normalise errors.** A `FallbackController` mapping `{:error, :not_found}` to `put_status(404)` is an orchestration of shape, not business logic.

A good test: "if a new caller (Oban, GraphQL, mix task) needed this functionality, could it call a domain function, or would it have to re-implement logic that currently lives in the controller?" If the latter, thin the controller.

## Further Reading

- [Intent `IN-AG-THIN-COORD-001`](../../../agnostic/thin-coordinator/RULE.md) — the agnostic principle this rule concretises.
- [Intent `IN-EX-ASH-001` code-interfaces-only](../../ash/code-interfaces-only/RULE.md) — the Ash-specific rule that keeps controllers clean by construction.
- [Phoenix docs — controllers](https://hexdocs.pm/phoenix/Phoenix.Controller.html) — the language reference.
