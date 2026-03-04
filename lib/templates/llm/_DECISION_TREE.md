# Decision Tree - Where Does This Code Belong?

> Use this tree when you're about to write new code. Walk through the questions to find the right location.
> Always cross-reference MODULES.md -- if a module already owns that concern, put the code there.

## Elixir/Phoenix Decision Tree

### Step 1: What kind of code is it?

**Is it a database query or data transformation?**

- Ash resource action or domain module
- NOT in a controller, LiveView, or GenServer

**Is it business logic (validation, orchestration, calculation)?**

- Dedicated service module in `lib/myapp/services/` or domain context
- NOT in a controller or LiveView

**Is it HTTP request/response handling?**

- Controller (thin: parse params -> call service -> render)
- Controller should be <50 lines per action

**Is it a real-time UI interaction?**

- LiveView (thin: delegate to service modules)
- Two-phase mount: `connected?/1` guard in mount
- `handle_event` should call a service, not contain logic

**Is it a background job?**

- Oban worker with `@impl perform/1`
- Job should call a service module, not contain logic

**Is it long-running state?**

- GenServer with `@impl` on all callbacks
- Business logic lives in a separate module, GenServer just manages state

**Is it a CLI command?**

- Command module (thin: parse args -> call service -> format output)

### Step 2: Does a module already own this?

1. Check MODULES.md
2. If yes: add code to that module
3. If no: register in MODULES.md first, then create the module

### Step 3: Anti-patterns

If you're tempted to...

| Temptation                                         | Correct Location                       |
| -------------------------------------------------- | -------------------------------------- |
| Put a query in a controller                        | Ash resource action or context module  |
| Put business logic in a LiveView                   | Service module                         |
| Put formatting in a service                        | View helper or component               |
| Create a second module for the same concern        | Use the existing one (Highlander Rule) |
| Put state management logic in a GenServer callback | Separate service module                |
| Put validation in a controller                     | Ash changeset or service module        |
| Put HTML in a LiveView module                      | Component (function or live)           |

## Generic Decision Tree (Non-Elixir)

### Where does it go?

1. **Data access** -> Repository/data layer (not in handlers/controllers)
2. **Business logic** -> Service/domain layer (not in UI or API layer)
3. **Request handling** -> Controller/handler (thin: parse, delegate, respond)
4. **UI rendering** -> View/template layer (no business logic)
5. **Background work** -> Worker/job layer (delegates to services)
6. **Shared utilities** -> Only if used by 3+ callers; otherwise inline

### The same rules apply everywhere

- Thin handlers, fat services
- One module per concern (check the registry)
- Register before you create
