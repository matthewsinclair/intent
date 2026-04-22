---
id: IN-AG-PFIC-001
language: agnostic
category: architecture
severity: critical
title: Pure Function, Impure Coordination
summary: >
  Keep the domain core deterministic and side-effect-free. Push I/O, time,
  randomness, and external calls out to the boundary. Coordinators compose
  pure functions and handle the effects; the core is testable and referentially
  transparent.
principles:
  - pfic
  - functional-core
  - honest-data
applies_when:
  - "Any module that encodes domain logic: validation, calculation, transformation, decision"
  - "Any function deeper than one call away from I/O that still performs I/O"
  - "A refactor opportunity where tests are slow because the unit under test is indirectly hitting the network or filesystem"
does_not_apply_when:
  - "Boundary adapters — HTTP clients, database repositories, OS shims — whose entire purpose is to perform the side effect"
  - "One-shot scripts where the indirection cost outweighs the testability gain"
  - "Trivial procedural code that never becomes a library and has no consumers beyond its script file"
references: []
related_rules:
  - IN-AG-THIN-COORD-001
  - IN-AG-NO-SILENT-001
concretised_by:
  - IN-EX-CODE-004
  - IN-EX-PHX-001
aliases: []
status: active
version: 1
---

# Pure Function, Impure Coordination

The domain core is pure; the boundary is impure. Mixing the two makes the core untestable, non-parallelisable, and hard to reason about.

## Problem

A function that both computes and performs I/O is unreasonable in three ways:

1. **Tests become integration tests.** Asking "does this rule correctly deny orders over $10k?" should not require a database, a network, or a clock. When domain logic reaches out to side-effecting modules from inside itself, unit tests require the whole stack. Tests slow down, become flaky, and drift away from the property they were supposed to prove.
2. **Reasoning is non-local.** Reading a computation, the reader cannot tell from the code alone what its output will be for a given input, because the output depends on unspecified ambient state (which row the repo returned, what time the clock said, which random seed the RNG had). Behaviour becomes a function of history, not inputs.
3. **Concurrency is unsafe.** Pure functions are trivially parallelisable. Functions that perform effects under the hood are not. A refactor to concurrent execution — exactly the thing a functional language promises — becomes a deep rewrite rather than a pipe swap.

The cure is to separate concerns: the core receives data and returns data. The coordinator is allowed to do I/O, but it does nothing else of interest — it fetches, calls the core, writes the result.

## Detection

Scan for functions in domain-layer modules that:

- Read from or write to a database, file, network, or external API.
- Read the wall clock or system randomness without the value being passed in as a parameter.
- Log, emit telemetry, or send notifications as a byproduct of computing.
- Accept an opaque resource handle (a repo, a client, a connection) and call methods on it.

Structural signals:

- A domain module imports an HTTP, DB, or filesystem module.
- A "pure-looking" function name (`calculate_tax`, `validate_order`) that, on inspection, makes a call out of the bounded context.
- A test file for the domain module that needs `setup :database` or `setup :http_mock` to run.

The reverse signal — a truly pure function — is one whose tests can run with no fixtures beyond literals and whose call graph does not leave the module's own namespace.

## Bad

```
def calculate_tax(order) do
  region = Repo.get!(Region, order.region_id)     # I/O inside the domain
  rate = HttpClient.get!("/rates/#{region.code}")  # external call inside the domain
  Logger.info("computed tax for #{order.id}")      # side effect inside the domain
  order.subtotal * rate
end
```

Three effects in one function: database read, network call, logging. Testing requires a running database, a mocked HTTP client, and a captured log stream. The function's behaviour depends on ambient state that the caller cannot see.

## Good

```
# Pure core — takes data in, returns data out.
def calculate_tax(%Order{subtotal: subtotal}, %Region{rate: rate}) do
  subtotal * rate
end

# Impure coordinator — handles the effects, calls the pure core.
def settle_order(order_id) do
  with {:ok, order} <- Orders.get(order_id),
       {:ok, region} <- Regions.get_for_order(order),
       {:ok, rate} <- Rates.fetch(region.code),
       region_with_rate = %{region | rate: rate},
       tax = calculate_tax(order, region_with_rate),
       {:ok, _} <- Orders.record_tax(order, tax) do
    :telemetry.execute([:orders, :tax, :computed], %{tax: tax}, %{order_id: order.id})
    {:ok, tax}
  end
end
```

`calculate_tax/2` is a function of its arguments. Its test is one line. The coordinator is where the effects live; it too is testable, but via a different category of test.

## When This Applies

Any language where pure and impure functions can be distinguished (most of them). The rule applies most strictly to:

- **Domain modules** (`Orders`, `Accounts`, `Pricing`) — these are the functional core.
- **Long-lived services** where reasoning about state matters.
- **Libraries** intended to be depended on — consumers need to understand input/output, not read the library's full effect graph.

For Elixir, this concretises as `with`-railways in coordinators and pure modules for domain calculations. For Rust, as passing `impl FnOnce() -> T` boundaries and keeping `Result` logic pure. For Swift, as separating `async` boundary code from synchronous domain code.

## When This Does Not Apply

- **Boundary adapters.** A module named `PostgresRepo`, `StripeClient`, or `FileStore` exists entirely to perform effects. Demanding purity of it is nonsensical; wrap the effect but do not hide it.
- **One-shot scripts.** A 30-line script that reads a CSV, mutates a database, and exits has no use for functional-core separation. Optimise for clarity, not purity.
- **Trivial procedural glue.** A three-line `main` function that reads stdin, calls one library function, prints the result is not the place for this rule.
- **Performance-critical hot paths** where allocating new immutable structures inside a tight loop is measurably the bottleneck. These are rare; measure first.

The rule does not mean "no side effects anywhere." It means "side effects live at the boundary; the core is pure."

## Further Reading

- Gary Bernhardt, "Functional Core, Imperative Shell" (2012 RailsConf talk) — the canonical formulation.
- José Valim, "Mocks and Explicit Contracts" — argues for behaviour parameterisation over test doubles, a direct consequence of pure-core discipline.
- Intent `IN-AG-THIN-COORD-001` — coordinators stay thin, which follows directly from pushing logic into a pure core.
- Intent `IN-AG-NO-SILENT-001` — pure functions should be total or explicitly fallible; coordinators must not swallow the `{:error, _}` tuples a pure core emits.
- Concretising rules: `IN-EX-CODE-004` (Elixir `with`-railway), `IN-EX-PHX-001` (Phoenix thin controllers).
