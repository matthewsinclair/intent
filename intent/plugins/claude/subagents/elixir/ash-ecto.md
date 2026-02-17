# Ash/Ecto Database Patterns Reference

Comprehensive reference for database access patterns in Ash Framework projects. **All database access goes through Ash — never raw Ecto in application code.**

Authoritative source: `deps/ash/usage-rules.md` (1,344 lines) and `deps/ash_ai/usage-rules.md` (523 lines). This document summarizes key patterns; consult the full usage-rules for edge cases.

## Core Principle: Ash-First, Never Raw Ecto

| Do this                             | Never this                                         |
| ----------------------------------- | -------------------------------------------------- |
| `mix ash.codegen <name>`            | `mix ecto.gen.migration`                           |
| `mix ash.migrate`                   | `mix ecto.migrate`                                 |
| `MyApp.Domain.get_thing!(id)`       | `Repo.get!(Thing, id)`                             |
| `MyApp.Domain.list_things!()`       | `Repo.all(Thing)`                                  |
| `MyApp.Domain.create_thing!(attrs)` | `%Thing{} \|> changeset(attrs) \|> Repo.insert!()` |
| Ash.Query                           | Ecto.Query in web modules                          |
| Ash policies                        | Manual auth checks in controllers                  |

Ecto knowledge is useful background (schemas, migrations, query building) but the Ash way is always preferred.

## Migrations

### Always Use `mix ash.codegen`

```bash
# Generate migration from resource changes
mix ash.codegen add_user_role

# Apply migrations
mix ash.migrate

# Rollback
mix ash.rollback
```

`ash.codegen` reads your resource definitions and generates the correct Ecto migration. Never write migrations by hand unless Ash cannot express the change (rare).

### Migration Best Practices

- Name migrations descriptively: `add_user_role`, `create_posts_table`, `add_index_on_email`
- Review generated migrations before running — Ash generates them, you own them
- Use `mix ash.codegen --check` in CI to verify no pending migrations
- For data migrations, create a separate Mix task — never put data changes in schema migrations

## Code Interfaces

Code interfaces are the public API for database access. Define them on the Ash Domain.

### Defining Code Interfaces

```elixir
defmodule MyApp.Accounts do
  use Ash.Domain

  resources do
    resource MyApp.Accounts.User do
      # Read operations
      define :get_user, action: :read, get_by: [:id]
      define :get_user_by_email, action: :read, get_by: [:email]
      define :list_users, action: :read

      # Write operations
      define :create_user, action: :create, args: [:email, :name]
      define :update_user, action: :update
      define :deactivate_user, action: :deactivate

      # Custom actions
      define :search_users, action: :search, args: [:query]
    end
  end
end
```

### Calling Code Interfaces

```elixir
# Always call through the domain module
user = MyApp.Accounts.get_user!(user_id, actor: current_user)
users = MyApp.Accounts.list_users!(actor: current_user)
user = MyApp.Accounts.create_user!("alice@example.com", "Alice", actor: current_user)

# With loading
user = MyApp.Accounts.get_user!(user_id, load: [:posts, :comments], actor: current_user)

# With query options
users = MyApp.Accounts.list_users!(
  query: [filter: [active: true], sort: [inserted_at: :desc], limit: 10],
  actor: current_user
)
```

### Code Interface Options

The `query:` option accepts filters, sorts, and limits without building manual `Ash.Query` pipelines:

```elixir
# GOOD — code interface with query options
MyApp.Content.list_posts!(
  query: [
    filter: [status: :published, author_id: user.id],
    sort: [published_at: :desc],
    limit: 20
  ],
  actor: current_user
)

# AVOID — manual Ash.Query pipeline in web modules
require Ash.Query
MyApp.Content.Post
|> Ash.Query.filter(status: :published, author_id: ^user.id)
|> Ash.Query.sort(published_at: :desc)
|> Ash.Query.limit(20)
|> Ash.read!(actor: current_user)
```

Manual `Ash.Query` is acceptable inside domain modules for complex queries. Never in web modules.

## Actor Placement

Set the actor on the query/changeset, not on the action call.

```elixir
# GOOD — actor on the query
Post
|> Ash.Query.for_read(:read, %{}, actor: current_user)
|> Ash.read!()

# BAD — actor on the action call
Post
|> Ash.Query.for_read(:read, %{})
|> Ash.read!(actor: current_user)

# GOOD — actor via code interface (handles placement correctly)
MyApp.Content.list_posts!(actor: current_user)
```

Code interfaces handle actor placement correctly — this is another reason to prefer them.

## Actions

### Action Design

Create specific, well-named actions rather than generic CRUD:

```elixir
# GOOD — specific actions
actions do
  create :register do
    accept [:email, :name]
    change MyApp.Accounts.Changes.HashPassword
    change MyApp.Accounts.Changes.GenerateConfirmationToken
  end

  update :deactivate do
    change set_attribute(:active, false)
    change set_attribute(:deactivated_at, &DateTime.utc_now/0)
  end

  update :change_email do
    accept [:email]
    validate MyApp.Accounts.Validations.EmailFormat
  end
end

# BAD — generic CRUD with conditional logic
actions do
  create :create do
    accept [:email, :name, :role, :active]
    # No specific business logic — just accepts everything
  end
end
```

### Error Handling

```elixir
# Use ! for "should always succeed" (let it crash)
user = MyApp.Accounts.get_user!(id, actor: current_user)

# Use non-raising for user-facing error handling
case MyApp.Accounts.create_user(email, name, actor: current_user) do
  {:ok, user} -> {:noreply, assign(socket, :user, user)}
  {:error, changeset} -> {:noreply, assign(socket, :form, to_form(changeset))}
end
```

## Changes, Validations, Preparations

### Custom Modules (Not Anonymous Functions)

```elixir
# GOOD — dedicated module
defmodule MyApp.Accounts.Changes.HashPassword do
  use Ash.Resource.Change

  @impl true
  def change(changeset, _opts, _context) do
    case Ash.Changeset.get_attribute(changeset, :password) do
      nil -> changeset
      password -> Ash.Changeset.change_attribute(changeset, :hashed_password, hash(password))
    end
  end
end

# BAD — anonymous function
change fn changeset, _context ->
  password = Ash.Changeset.get_attribute(changeset, :password)
  Ash.Changeset.change_attribute(changeset, :hashed_password, hash(password))
end
```

### Atomic Changes

Implement `atomic/3` when possible for database-level operations:

```elixir
defmodule MyApp.Content.Changes.IncrementViewCount do
  use Ash.Resource.Change

  @impl true
  def atomic(_changeset, _opts, _context) do
    {:atomic, %{view_count: expr(view_count + 1)}}
  end
end
```

Use `require_atomic? false` only when the change genuinely cannot be expressed atomically (external API calls, complex Elixir logic).

## Relationships

### Defining Relationships

```elixir
relationships do
  belongs_to :author, MyApp.Accounts.User do
    allow_nil? false
  end

  has_many :comments, MyApp.Content.Comment

  many_to_many :tags, MyApp.Content.Tag do
    through MyApp.Content.PostTag
  end
end
```

### Loading Relationships

```elixir
# Via code interface options
post = MyApp.Content.get_post!(id, load: [:author, :comments], actor: current_user)

# Nested loading
post = MyApp.Content.get_post!(id,
  load: [author: [:profile], comments: [:author]],
  actor: current_user
)
```

## Calculations and Aggregates

Push derived data to the resource definition — don't compute in Elixir after loading.

```elixir
# Calculation — derived value
calculations do
  calculate :full_name, :string, expr(first_name <> " " <> last_name)
  calculate :display_name, :string, expr(
    if(is_nil(nickname), first_name <> " " <> last_name, nickname)
  )
end

# Aggregate — summary of related data
aggregates do
  count :comment_count, :comments
  sum :total_score, :votes, :value
  first :latest_comment_body, :comments, :body do
    sort inserted_at: :desc
  end
end
```

### Loading Calculations/Aggregates

```elixir
# Load with the query
users = MyApp.Accounts.list_users!(
  load: [:full_name, :comment_count],
  actor: current_user
)
```

## Notifiers

Use Ash notifiers for side effects triggered by actions:

```elixir
defmodule MyApp.Accounts.Notifiers.WelcomeEmail do
  use Ash.Notifier

  @impl true
  def notify(%Ash.Notifier.Notification{resource: MyApp.Accounts.User, action: %{name: :register}} = notification) do
    MyApp.Mailer.send_welcome(notification.data)
    :ok
  end

  def notify(_), do: :ok
end
```

Register on the resource:

```elixir
resource do
  notifiers [MyApp.Accounts.Notifiers.WelcomeEmail]
end
```

## Ash.Query Reference

When you need `Ash.Query` (inside domain modules only), `require Ash.Query` at the **module level** (not inside function bodies — that fails Credo and is bad practice):

```elixir
defmodule MyApp.Content.Queries do
  require Ash.Query  # Module level — filter is a macro

  def recent_published(actor) do
    MyApp.Content.Post
    |> Ash.Query.filter(status: :published)
    |> Ash.Query.filter(inserted_at > ago(7, :day))
    |> Ash.Query.sort(inserted_at: :desc)
    |> Ash.Query.limit(10)
    |> Ash.Query.load([:author, :comment_count])
    |> Ash.read!(actor: actor)
  end
end
```

## AshAI Patterns

Reference: `deps/ash_ai/usage-rules.md`

Key patterns for AI integration:

- **Vectorization**: Use `AshAI.Extensions.Vector` for embedding-based search
- **Prompt-backed actions**: Define actions that use LLM prompts
- **MCP integration**: Expose Ash resources as MCP tools
- **Tool definitions**: Use `AshAI.Tool` to create LLM-callable tools from actions
