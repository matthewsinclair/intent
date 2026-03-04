# Module Registry - [[PROJECT_NAME]]

> **The Highlander Rule**: There can be only one module per concern.
> ALWAYS check this file before creating a new module. If a module already owns that concern, use it.
> When you must create a new module, register it here FIRST, then create the file.

## Registry

| Concern | THE Module | Notes |
| ------- | ---------- | ----- |

<!-- Add entries as modules are created. Group by domain. Example:

### Auth

| Concern            | THE Module           | Notes                        |
| ------------------ | -------------------- | ---------------------------- |
| User authentication | MyApp.Auth.Guardian  | JWT tokens, session handling |
| Authorization       | MyApp.Auth.Policy    | Role-based access control    |

### Core

| Concern            | THE Module           | Notes                        |
| ------------------ | -------------------- | ---------------------------- |
| Email delivery      | MyApp.Core.Mailer    | Swoosh adapter               |
| Background jobs     | MyApp.Core.Jobs      | Oban worker coordinator      |

### Content

| Concern            | THE Module           | Notes                        |
| ------------------ | -------------------- | ---------------------------- |
| File uploads        | MyApp.Content.Upload | S3 storage, image processing |

-->

## How to Use This File

1. **Before creating a new module**: Search this table. If the concern is listed, use that module.
2. **When adding a new module**: Add a row here first, then create the file.
3. **When refactoring**: Update this table to reflect the new module ownership.
4. **When removing a module**: Remove its row from this table.

Violations of the Highlander Rule (duplicate modules for the same concern) are the #1 source of code quality debt.
