# Ash/Ecto Essentials

Database access rules for Ash Framework projects. All database access goes through Ash -- never raw Ecto in application code. Authoritative reference: `deps/ash/usage-rules.md`.

## Rules

### 1. All database access through domain code interfaces

NEVER call `Ash.get!/2`, `Ash.read!/2`, `Ash.create!/2`, or `Ash.load!/2` directly in LiveViews, controllers, or other web modules. Always go through the domain's code interface.

```elixir
# BAD -- direct Ash calls in LiveView
def mount(%{"id" => id}, _session, socket) do
  post = MyApp.Content.Post |> Ash.get!(id) |> Ash.load!([:author])
  {:ok, assign(socket, :post, post)}
end

# GOOD -- domain code interface
def mount(%{"id" => id}, _session, socket) do
  post = MyApp.Content.get_post!(id, load: [:author], actor: socket.assigns.current_user)
  {:ok, assign(socket, :post, post)}
end
```

### 2. `mix ash.codegen` for migrations, never `mix ecto.gen.migration`

Ash reads your resource definitions and generates correct migrations. Never write Ecto migrations by hand.

```bash
# GOOD
mix ash.codegen add_user_role
mix ash.migrate

# BAD
mix ecto.gen.migration add_user_role
mix ecto.migrate
```

### 3. Set actor on query/changeset, not on action call

The actor must be set when building the query or changeset, not when executing the action.

```elixir
# BAD -- actor on action call
Post
|> Ash.Query.for_read(:read, %{})
|> Ash.read!(actor: current_user)

# GOOD -- actor on query
Post
|> Ash.Query.for_read(:read, %{}, actor: current_user)
|> Ash.read!()

# GOOD -- code interfaces handle this correctly
MyApp.Content.list_posts!(actor: current_user)
```

### 4. Prefer code interface options over manual Ash.Query pipelines

Use `query: [filter: ..., sort: ..., limit: ...]` on code interface calls instead of building `Ash.Query` pipelines in web modules.

```elixir
# BAD -- Ash.Query pipeline in LiveView
require Ash.Query
posts =
  MyApp.Content.Post
  |> Ash.Query.filter(status: :published)
  |> Ash.Query.sort(inserted_at: :desc)
  |> Ash.Query.limit(20)
  |> Ash.read!(actor: current_user)

# GOOD -- code interface with query options
posts = MyApp.Content.list_posts!(
  query: [filter: [status: :published], sort: [inserted_at: :desc], limit: 20],
  actor: current_user
)
```

### 5. Custom change/validation modules, not anonymous functions

Put Ash change and validation logic in dedicated modules, not inline anonymous functions.

```elixir
# BAD -- anonymous function
actions do
  create :create do
    change fn changeset, _context ->
      title = Ash.Changeset.get_attribute(changeset, :title)
      Ash.Changeset.change_attribute(changeset, :slug, Slug.slugify(title))
    end
  end
end

# GOOD -- dedicated module
actions do
  create :create do
    change MyApp.Content.Changes.SlugifyTitle
  end
end
```

### 6. Atomic changes preferred; `require_atomic? false` only when necessary

Implement `atomic/3` callback for database-level operations. Only use `require_atomic? false` when the change genuinely cannot be expressed atomically.

```elixir
# GOOD -- atomic change
defmodule MyApp.Changes.IncrementCount do
  use Ash.Resource.Change

  @impl true
  def atomic(_changeset, _opts, _context) do
    {:atomic, %{view_count: expr(view_count + 1)}}
  end
end

# OK -- non-atomic only when truly necessary (external API call)
defmodule MyApp.Changes.GeocodeAddress do
  use Ash.Resource.Change

  @impl true
  def change(changeset, _opts, _context) do
    # Must call external geocoding API -- cannot be atomic
    address = Ash.Changeset.get_attribute(changeset, :address)
    {:ok, coords} = GeocodingService.lookup(address)
    Ash.Changeset.change_attributes(changeset, %{lat: coords.lat, lng: coords.lng})
  end
end
```

### 7. `Ash.Query.filter` is a macro -- always `require Ash.Query` at module level

Forgetting `require` causes a confusing compile error. The `require` must go at the top of the module (with other requires, in alphabetical order), never inside a function body.

```elixir
# BAD -- no require, will fail to compile
defmodule MyApp.Accounts do
  def list_active_users do
    MyApp.Accounts.User
    |> Ash.Query.filter(active: true)
    |> Ash.read!()
  end
end

# BAD -- require inside function body (fails Credo, bad practice)
def list_active_users do
  require Ash.Query
  # ...
end

# GOOD -- require at module level
defmodule MyApp.Accounts do
  require Ash.Query

  def list_active_users do
    MyApp.Accounts.User
    |> Ash.Query.filter(active: true)
    |> Ash.read!()
  end
end
```
