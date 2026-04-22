---
id: IN-AG-NO-SILENT-001
language: agnostic
category: architecture
severity: critical
title: No Silent Errors
summary: >
  Every error path is handled explicitly. No empty rescue clauses, no
  discarded `Result`s or error tuples, no `try`/`catch` that catches and
  continues without surfacing the failure. When a failure cannot be
  handled, let it crash; never pretend it did not happen.
principles:
  - no-silent-errors
  - pfic
applies_when:
  - "Library code, domain logic, and any code with downstream consumers"
  - "Long-running processes where a swallowed error today becomes a mysterious outage tomorrow"
  - "Coordinators receiving tagged-result tuples from the domain"
does_not_apply_when:
  - "Best-effort telemetry, logging, or cache-warming where a failure is genuinely ignorable"
  - "Supervisor `let it crash` boundaries where the supervisor is the explicit handler"
  - "Fire-and-forget notifications where the caller has decided the downstream outcome is not their concern and logs the attempt"
references:
  - IN-AG-PFIC-001
related_rules:
  - IN-AG-THIN-COORD-001
concretised_by:
  - IN-EX-CODE-005
  - IN-EX-CODE-004
  - IN-RS-CODE-001
  - IN-SW-CODE-002
  - IN-LU-CODE-004
aliases: []
status: active
version: 1
---

# No Silent Errors

Every error path is handled explicitly. Swallowed errors hide failures, degrade the system silently, and make root cause analysis impossible days or weeks later.

## Problem

Silent errors rot systems from the inside:

1. **Users see wrong answers, not failures.** A "write to cache" step silently fails. The read path falls through to a slower source and returns stale data. No error is logged; the only signal is a user complaining that the number looks off.
2. **Diagnostics become forensics.** A swallowed exception does not appear in logs, traces, or metrics. The operator responding to an incident has no thread to pull. They work from user reports backwards, guessing at which of dozens of silently-failing subsystems is to blame.
3. **Wrong contracts propagate.** A function that claims to return `Result<T, E>` but discards its `Err` arm and returns a default `T` violates its own contract. Callers assume success. The bug is now in the type system's face and invisible.

The cure is a hard rule: every failure mode gets a named response. The response can be "recover and continue," "log and abort," "return a tagged error to the caller," or "let the supervisor crash us." It cannot be "do nothing and keep going."

## Detection

Signals in code:

- `rescue _ -> nil` or equivalent (`except: pass`, `catch { }`, `.unwrap_or_default()` where the caller meant "success or failure").
- A returned `Result` or `{:ok, _} | {:error, _}` tuple where the caller binds only the happy path and discards the error tuple implicitly (e.g. `{:ok, value} = fallible()` in a place where `{:error, _}` is possible).
- `try { ... } catch (Exception e) { /* intentionally blank */ }`.
- A call that returns a fallible value whose return value is ignored: `_ = Repo.insert(changeset)`, `some_http_call();` in a language where the compiler does not force the handling.
- A log statement at the error site with no accompanying propagation: `Logger.error("failed")` followed by `nil` returned as if success.

Structural signals:

- Coordinators that pattern-match only on `{:ok, _}` and fall through on `{:error, _}` — the match becomes silently flaky.
- Error paths in `with` blocks or railway pipelines that hit an `else` clause which returns a default value instead of the tagged error.
- A supervisor that restarts its children but never surfaces why.

## Bad

```
# Empty rescue — the classic silent failure.
def save(changeset) do
  try do
    Repo.insert!(changeset)
  rescue
    _ -> nil
  end
end
```

The function promises to save. It returns `nil` on any failure. The caller has no way to distinguish "saved" from "silently failed." The failure is invisible to logs, to tests, and to operators.

```
# Discarded Result — Rust-style equivalent.
fn write_config(path: &Path, cfg: &Config) {
  let _ = std::fs::write(path, cfg.to_string());  // write may fail; we throw that away
}
```

```
# Catching and continuing — Java-style equivalent.
public void notify(User user) {
  try {
    mailer.send(user);
  } catch (Exception e) {
    // ignored; we tried
  }
}
```

All three share the same flaw: the caller cannot tell the function failed; operators cannot tell why the system is mis-behaving.

## Good

```
# Tagged result — explicit failure surface.
def save(changeset) do
  case Repo.insert(changeset) do
    {:ok, record} -> {:ok, record}
    {:error, changeset} -> {:error, {:validation, changeset}}
  end
end
```

```
# Let it crash, deliberately — the supervisor is the handler.
def process_message!(message) do
  # Any failure crashes the process; the supervisor restarts.
  # The contract is "this function always succeeds or the process dies."
  # The '!' suffix in the name signals this to callers.
  :ok = do_work(message)
end
```

```
# Best-effort with logged attempt — explicit decision.
def fire_and_forget_notify(user) do
  case Mailer.send(user) do
    {:ok, _} -> :ok
    {:error, reason} -> Logger.warning("notify failed for #{user.id}: #{inspect(reason)}"); :ok
  end
end
```

Each case is explicit about what happens on failure. A reviewer can tell from the code what the system will do. An operator responding to an incident has a log line or a crash report to work from.

## When This Applies

- **Library code.** Consumers of libraries cannot know the library's internals. Swallowed errors inside a library turn into silent misbehaviour in every consumer.
- **Domain logic.** The domain is the system of record. Losing an error inside the domain loses the ground truth.
- **Long-lived processes.** Servers, GenServers, workers. A swallowed error today is a mystery outage next month.
- **Coordinators.** When a coordinator receives `{:error, _}` from the domain, it must decide: translate to a transport error, retry, or propagate. Discarding it silently is never correct.

## When This Does Not Apply

- **Best-effort telemetry and logging.** If a metrics emitter fails to emit, blocking the happy path is worse than dropping the datum. A logged-and-dropped failure here is correct; the key is the "logged" part — failure still has a trail.
- **Supervisor boundaries.** In `let it crash` systems, the supervisor is the explicit error handler. A process that crashes is not silent — the supervisor records the restart. The whole-function rule of "don't swallow" does not apply to the code path where the process lets itself die.
- **Fire-and-forget with an explicit decision.** If the caller has decided "a failure to notify the user is acceptable," that decision must be visible in code: a `case` that handles both arms and returns `:ok` either way, with a log line on the error arm. The decision is explicit; the failure is still recorded.
- **Idempotent retries inside a bounded retry budget.** Swallowing the first three attempts' errors while preserving the last one's is acceptable when the retry logic is visible and the final failure surfaces. The budget itself is the contract.

The distinguishing question: **would an operator, reading only the code, know what the system does when this call fails?** If yes, the rule is satisfied (even if the answer is "nothing"). If no, the rule is violated.

## Further Reading

- Joe Armstrong, "Making reliable distributed systems in the presence of software errors" (1993 thesis) — the origin of "let it crash" and the discipline that silent errors are worse than crashes.
- Rob Pike, "Errors are values" — the Go community's framing; error paths are data, not exceptions.
- Intent `IN-AG-PFIC-001` — pure cores return tagged results; coordinators must handle both arms.
- Intent `IN-AG-THIN-COORD-001` — coordinators are where domain errors are translated to transport errors; a coordinator that swallows the error arm is also violating Thin Coordinator.
- Concretising rules: `IN-EX-CODE-005` (Elixir tagged tuples + no empty rescues), `IN-EX-CODE-004` (Elixir `with`-railway with explicit `else`), `IN-RS-CODE-001` (Rust Result over panic), `IN-SW-CODE-002` (Swift optionals over sentinels), `IN-LU-CODE-004` (Lua pcall for error boundaries).
