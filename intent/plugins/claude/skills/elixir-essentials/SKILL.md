# Elixir Essentials

Core Elixir coding rules enforced on every line of generated code. These are mandatory — no exceptions.

## Rules

### 1. Multi-clause pattern matching over conditionals

NEVER use nested `if/case/cond` to branch on struct or map fields. Use multiple function heads with destructuring. Each clause should be a single expression.

```elixir
# BAD
def process(user) do
  if user.status == :active do
    if user.role == :admin do
      :allowed
    else
      :denied
    end
  else
    :inactive
  end
end

# GOOD
def process(%{status: :active, role: :admin}), do: :allowed
def process(%{status: :active}), do: :denied
def process(_), do: :inactive
```

Use guards for type-based or range-based decisions:

```elixir
# BAD
def format(value) do
  cond do
    is_binary(value) -> String.trim(value)
    is_integer(value) -> Integer.to_string(value)
    true -> inspect(value)
  end
end

# GOOD
def format(value) when is_binary(value), do: String.trim(value)
def format(value) when is_integer(value), do: Integer.to_string(value)
def format(value), do: inspect(value)
```

### 2. `@impl true` on all behaviour callbacks

Every callback function MUST have `@impl true`. This catches typos in function names at compile time and makes it obvious which functions are callbacks vs custom logic.

```elixir
# BAD
def mount(_params, _session, socket) do
  {:ok, socket}
end

# GOOD
@impl true
def mount(_params, _session, socket) do
  {:ok, socket}
end
```

Applies to: `mount/3`, `render/1`, `handle_event/3`, `handle_info/2`, `handle_params/2`, `init/1`, `handle_call/3`, `handle_cast/2`, `terminate/2`, and all other behaviour callbacks.

### 3. Tagged tuples for all fallible functions

Return `{:ok, result}` or `{:error, reason}`. Never bare values that might be nil.

```elixir
# BAD
def find_user(id) do
  Repo.get(User, id)  # returns nil on not found
end

# GOOD
def find_user(id) do
  case Repo.get(User, id) do
    nil -> {:error, :not_found}
    user -> {:ok, user}
  end
end
```

### 4. `with` for railway-oriented composition

Chain 2+ fallible operations using `with`. Normalize errors in private wrapper functions, not in `else` blocks.

```elixir
# BAD
def create_order(params) do
  case validate(params) do
    {:ok, validated} ->
      case charge_payment(validated) do
        {:ok, payment} ->
          case save_order(validated, payment) do
            {:ok, order} -> {:ok, order}
            error -> error
          end
        error -> error
      end
    error -> error
  end
end

# GOOD
def create_order(params) do
  with {:ok, validated} <- validate(params),
       {:ok, payment} <- charge_payment(validated),
       {:ok, order} <- save_order(validated, payment) do
    {:ok, order}
  end
end
```

### 5. Pipe operator for sequential transformations

2+ transformations use pipes. The first argument is always the data being transformed.

```elixir
# BAD
def process(raw_data) do
  trimmed = String.trim(raw_data)
  downcased = String.downcase(trimmed)
  String.replace(downcased, " ", "-")
end

# GOOD
def process(raw_data) do
  raw_data
  |> String.trim()
  |> String.downcase()
  |> String.replace(" ", "-")
end
```

### 6. Naming conventions

- `?` suffix for boolean-returning functions: `active?/1`, `valid?/1`
- `!` suffix for functions that raise on error: `fetch!/1`, `create!/1`
- `_` prefix for unused variables: `_params`, `_opts`
- Expressive variable names: `user_params` not `params`, `_record` not `_`

```elixir
# BAD
def check(user), do: user.active
def get(id), do: Repo.get!(User, id)
def process(_, data), do: transform(data)

# GOOD
def active?(user), do: user.active
def get_user!(id), do: Repo.get!(User, id)
def process(_user, data), do: transform(data)
```

### 7. Assertive data access

Use `struct.field` for required keys (fails fast). Use `map[:key]` only for truly optional keys. Pattern match to destructure and validate simultaneously.

```elixir
# BAD — defensive access on required fields
name = user[:name]
email = user[:email]

# GOOD — assertive access on required fields
name = user.name
email = user.email

# GOOD — pattern match to destructure
%{name: name, email: email} = user

# GOOD — optional key access (key genuinely might not exist)
nickname = user[:nickname]
```

### 8. No debug artifacts in committed code

No `IO.inspect/2`, no `dbg()`, no `IO.puts` for debugging. Use `dbg()` during development (better pipeline visibility than `IO.inspect`), but never commit either.

```elixir
# BAD — committed to source
def process(data) do
  data
  |> transform()
  |> IO.inspect(label: "after transform")
  |> finalize()
end

# GOOD — clean committed code
def process(data) do
  data
  |> transform()
  |> finalize()
end
```
