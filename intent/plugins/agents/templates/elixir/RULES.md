# RULES.md

Mandatory coding rules for this project. Every statement is "must" or "never". These rules are enforced by installed skills and applied during code review by the Elixir subagent.

## Core Elixir Rules

### 1. Typed Data Access

Use assertive data access. `struct.field` for required keys (fails fast). `map[:key]` only for truly optional keys. Pattern match to destructure and validate simultaneously.

### 2. Thin Controllers and LiveViews

Controllers and LiveViews are coordinators only. They assign state, dispatch to domain, and update assigns. No business logic, data transformation, or aggregation queries. Allowed private functions: `handle_*`, `assign_*`, `load_*`.

### 3. No Helpers in Controllers

Private helper functions do NOT belong in LiveView or Controller modules. Move them to dedicated helper modules.

### 4. Component Extraction

Repeated HEEX patterns must be extracted into reusable components. When you see the same HTML structure appear twice, extract it.

### 5. Multi-Head Functions

Use multiple function heads with pattern matching instead of nested `if/case/cond`. Each clause should be a single clear expression. Use guards for type-based decisions.

### 6. The Highlander Rule

There can be only one. Every piece of business logic has exactly ONE authoritative implementation. If two modules do the same thing differently, one must be eliminated.

### 7. Assertive Data Access

Use `struct.field` for required keys, `map[:key]` for optional. Use `and`/`or` (not `&&`/`||`) when operands are guaranteed boolean. Use `@enforce_keys` for required struct fields.

### 8. Boolean Operators

Use `and`/`or` when operands are boolean. Use `&&`/`||` only when operands may be non-boolean (truthy/falsy).

### 9. Exhaustive With

Every `with` clause must have a matching `else` clause or all called functions must return tagged tuples. Never let unexpected errors silently pass through.

## Framework Rules

### Ash Framework

- All database access through domain code interfaces -- never `Ash.get!/2`, `Ash.read!/2` in web modules
- `mix ash.codegen <name>` for migrations, never `mix ecto.gen.migration`
- Set actor on query/changeset, not on action call
- Custom change/validation/preparation modules, not anonymous functions
- Calculations and aggregates over `Enum.map`/`Enum.reduce` post-load transforms
- Reference `deps/ash/usage-rules.md` for the full Ash style guide

### Phoenix

- Verified routes (`~p"..."`) only, never string-based routes
- `phx.gen.auth` scope pattern: use `@scope.user`, never `@current_user`
- Auth at the router level with plugs and `live_session` scopes
- Never duplicate `live_session` names

### LiveView

- Guard PubSub/async with `connected?(socket)` in mount
- Use `stream/3` for large or dynamic lists
- Use `assign_async/3` for non-blocking data loading
- `@impl true` on all callbacks

## Testing Rules

- Test the domain, not the UI -- primary coverage on domain/service modules
- `success:` / `failure:` prefix on all test names
- One assertion focus per test
- Use `!` (raising) functions for test setup, non-raising for testing error cases
- `async: true` by default, opt out only when necessary
- `System.unique_integer/1` for identity attributes in fixtures

## NEVER DO

- NEVER write backwards compatible code
- NEVER hardcode test data into framework code
- NEVER put business logic in LiveViews or Controllers
- NEVER duplicate a code path
- NEVER reach across domain boundaries
- NEVER use `Ash.get!/2` / `Ash.read!/2` / `Ash.load!/2` directly in web modules
- NEVER use anonymous functions for Ash changes/validations/preparations
- NEVER commit `dbg()` or `IO.inspect/2`
- NEVER put `require` inside a function body -- module level only
- NEVER use `@current_user` -- use scope-based assign pattern

<!-- Add project-specific rules below this line -->

## Project-Specific Rules

<!-- Add rules unique to this project here -->
