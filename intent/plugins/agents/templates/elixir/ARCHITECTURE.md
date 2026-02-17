# ARCHITECTURE.md

System architecture and design decisions for this project.

## System Overview

<!-- Replace with a 2-3 sentence description of what the system does -->

This is a Phoenix/Ash web application.

## Domain Map

<!-- List your Ash domains and their responsibilities -->

```
lib/my_app/
├── accounts/        # User management, authentication
│   ├── accounts.ex  # Domain module (code interfaces)
│   ├── user.ex      # Resource
│   └── token.ex     # Resource
│
├── content/         # Core content domain
│   ├── content.ex   # Domain module
│   └── ...
│
└── notifications/   # Email, push, webhooks
    └── ...
```

### Domain Boundaries

<!-- Describe how domains interact -->

- **Accounts** -- owns user identity, authentication, authorization
- **Content** -- owns business content; calls Accounts for user references
- **Notifications** -- triggered by other domains via Ash notifiers

## Data Flow

<!-- Describe the typical request flow -->

```
Browser → Router → LiveView/Controller → Domain Code Interface → Ash Action → Database
                                                                     ↓
                                                               Notifiers → Side Effects
```

## Key Patterns

### Authentication

Using `phx.gen.auth` scope-based pattern. Auth handled at router level via plugs and `live_session` scopes. Access current user via `@scope.user`.

### Authorization

Ash policies on resources. `bypass` for admin. `can_action_name?/2` for UI conditionals.

### Background Jobs

<!-- Describe your background job setup: Oban, GenServer, etc. -->

### External Integrations

<!-- List external APIs, services, etc. -->

## Directory Structure

```
lib/
├── my_app/              # Domain layer
│   ├── accounts/        # Domain: user management
│   ├── content/         # Domain: core content
│   └── repo.ex          # Ecto Repo
├── my_app_web/          # Web layer
│   ├── components/      # Reusable HEEX components
│   ├── controllers/     # Traditional controllers
│   ├── live/            # LiveView modules
│   └── router.ex        # Route definitions
└── my_app.ex            # Application module
```

## Decision Log

<!-- Record significant architectural decisions -->

| Date       | Decision          | Rationale                                                      |
| ---------- | ----------------- | -------------------------------------------------------------- |
| YYYY-MM-DD | Use Ash Framework | Declarative resources, built-in authorization, code generation |
| YYYY-MM-DD | ...               | ...                                                            |
