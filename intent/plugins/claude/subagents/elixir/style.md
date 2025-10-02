# Elixir Code Style Guide

This document outlines Elixir code style guidelines to improve readability, maintainability, and consistency.

## Module Organization

### Whitespace between import and alias

Use intentional whitespace to separate distinct concepts. `import` and `alias` are distinct concepts and should be separated.

```elixir
import Ecto.Query

alias MyApp.RankedVoting.Ballot
alias MyApp.RankedVoting.Vote
alias MyApp.Repo
```

### Alphabetically ordered alias declarations

List `alias` declarations in alphabetical order. This can be enforced via Credo's [`AliasOrder`](https://hexdocs.pm/credo/Credo.Check.Readability.AliasOrder.html).

Use a consistent order across `use`, `import`, `alias`, and `require` via [`StrictModuleLayout`](https://hexdocs.pm/credo/Credo.Check.Readability.StrictModuleLayout.html).

### Avoid multi-alias declarations

Prefer explicit alias declarations over multi-alias syntax:

```elixir
# Preferred
alias MyApp.RankedVoting.Ballot
alias MyApp.RankedVoting.Vote

# Avoid
alias MyApp.RankedVoting.{Ballot, Vote}
```

Multi-alias declarations make searching the codebase for module names more difficult. This can be enforced with Credo's [`MultiAlias`](https://hexdocs.pm/credo/Credo.Check.Readability.MultiAlias.html).

## Function Definitions

### Prefer multiline do/end functions

Unless expressing a simple stack of related single-line functions, prefer multiline `do/end` function declarations. This allows code editors to collapse functions cleanly during exploration.

```elixir
# Acceptable for simple related functions
defp page_title(:edit), do: "Edit Record"
defp page_title(_), do: "Create Record"

# Preferred for most functions
def get_record!(id) do
  Repo.get!(Record, id)
end
```

## Testing Style

### Use consistent DSL layout in test modules

Structure tests with `describe` blocks for the function under test, and prefix test names with `success` or `failure` to indicate expectations:

```elixir
describe "update_record/1" do
  test "success: updates a record title and fields" do
    # ...
  end

  test "failure: required field is missing" do
    # ...
  end

  test "failure: cannot update a published record" do
    # ...
  end
end
```

This organization helps keep test files navigable even when they grow to 2-3 times the line count of the module under test.

### Invest in test fixtures

Follow the Arrange-Act-Assert pattern with well-designed test fixtures. Test fixtures should:

- Provide functions for entity creation
- Expose known argument defaults
- Use actual domain context paths for creation (not raw SQL)
- Only use SQL injection when absolutely needed for performance or edge cases

```elixir
defmodule Support.Fixtures.RecordFixture do
  @moduledoc """
  Provides functions to allow tests to easily create and stage entities for testing.
  """

  @doc """
  Returns a map of valid attributes, allowing passed-in attributes to override defaults.
  """
  @spec valid_record_attributes(map()) :: map()
  def valid_record_attributes(attrs \\ %{}) do
    Enum.into(attrs, %{
      title: "Default Title",
      status: :draft,
      slug: "slug-#{System.unique_integer()}"
    })
  end

  @doc """
  Creates an entity in the Repo for the passed-in optional attributes.
  When not provided, all required attributes will be generated.
  """
  @spec record_fixture(map()) :: Record.t()
  def record_fixture(attrs \\ %{}) do
    attrs = valid_record_attributes(attrs)
    {:ok, record} = MyApp.Context.create_record(attrs)
    record
  end
end
```

### Use tiny_maps for concise test signatures

To keep test descriptions on a single line, use the [tiny_maps](https://github.com/abshierjoel/tiny_maps) library:

```elixir
test "success: submitting valid form creates record and redirects", ~M{view, record} do
  # ~M{view, record} expands to %{view: view, record: record}
end
```

This syntax is generally limited to test files but can be used in main source code when appropriate.

## Code Composition

### Compose code for clean line breaks

Prefer clean one-liners or avoid excessive indentation when breaking up complex terms. Separate logical groupings to improve readability.

```elixir
# Good: Separate payload assignment from pipeline
test "success: submitting valid form creates record", ~M{view} do
  payload = %{
    title: "Example Title",
    content: "Example Content",
    slug: "example-slug"
  }

  response =
    view
    |> form("form", record: payload)
    |> render_submit()

  assert {:error, {:redirect, %{to: redirect_target}}} = response
end
```

This separation:

- Distinguishes the arrange vs. act test phases
- Keeps the pipeline clean and readable
- Avoids complex multiline expressions with excessive indentation

### Embrace pipelines with custom utility functions

Create utility functions to enable clean pipelines for common patterns, particularly when tuple return values are required:

```elixir
# In a LiveView mount function
def mount(_params, _session, socket) do
  socket
  |> assign(:page_title, "Admin: Records")
  |> assign(:records, MyApp.Context.list_records())
  |> ok()
end

# Utility module
defmodule MyAppWeb.LiveViewPipes do
  @moduledoc """
  A collection of functions to help express pipes when processing live view responses.
  """

  alias Phoenix.LiveView.Socket

  @spec ok(Socket.t()) :: {:ok, Socket.t()}
  def ok(%Socket{} = socket), do: {:ok, socket}

  @spec noreply(Socket.t()) :: {:noreply, Socket.t()}
  def noreply(%Socket{} = socket), do: {:noreply, socket}
end
```

## Naming and Organization

### Be consistent between module names and filenames

Module names should directly correspond to file paths:

- `MyApp.Context.Record` → `lib/my_app/context/record.ex`
- `MyApp.Schema.User` → `lib/my_app/schema/user.ex`

Maintain this consistency across the entire project for easy navigation.

### Avoid abbreviations and prefer expressiveness

Use expressive variable names:

```elixir
# Preferred
def process(record_params) do
  # ...
end

# Avoid
def process(params) do
  # ...
end

# Preferred (when unused)
def process(_record) do
  # ...
end

# Avoid (when you need the value)
def process(_) do
  # ...
end
```

### Words matter: maintain ubiquitous language

Invest time in an expressive and consistent [ubiquitous language](https://martinfowler.com/bliki/UbiquitousLanguage.html) for your project:

- Find consistency regarding terms like `create` vs. `new`, `update` vs. `edit`, `submit` vs. `save`, `params` vs. `attributes`
- Document your project's ubiquitous language
- Continuously refine it as terms evolve
- Align with existing Elixir community norms where possible

## Documentation

### Write professional documentation

All public functions should include documentation, especially those representing the formalized domain API.

```elixir
@doc """
Publishes the given record.

Once a record is published, it can no longer be updated.
Only a published record is visible to end users.
"""
def publish_record(%Record{} = record) do
  # ...
end
```

Documentation guidelines:

- Start with a single-line, terse summary (used by `ex_doc` for function indexes)
- Review function summaries periodically for consistent phrasing
- Use backticks for module, function, or callback references (enables hyperlinks)
- Hard-wrap to 80 columns for better GitHub diffing and Markdown presentation

### Document your decisions

When making intentional design or process decisions with multiple valid approaches, document what was considered and why you chose your approach. Create decision records to help your future self and peers understand the reasoning.

### Make sure each FIXME has an issue URL

When using `FIXME` comments, include a link to a GitHub issue documenting the concern. This ensures all technical debt is tracked and can be addressed systematically.

## Type Specifications

### Craft typespecs to express your domain

All public functions should have a typespec. Private functions can also have typespecs when they improve code clarity or change confidence.

Make typespecs match the domain:

```elixir
# In the schema module
@type id :: Ecto.UUID.t()

# In the context module
@spec get_record(Record.id()) :: {:ok, Record.t()} | {:error, :not_found}
def get_record(record_id) do
  # ...
end
```

### Distinguish persisted vs. non-persisted struct types

For domain entities, create distinct types for persisted and non-persisted structs:

```elixir
@typedoc """
A type for a persisted record entity.
"""
@type t :: %__MODULE__{
        id: Ecto.UUID.t(),
        title: String.t(),
        description: String.t() | nil,
        slug: String.t(),
        published_at: DateTime.t() | nil
      }

@typedoc """
A type for the empty struct.

This type is helpful when you want to typespec a function that needs to accept
a non-persisted struct value.
"""
@type struct_t :: %__MODULE__{}
```

Use `@typedoc` documentation to explain the reasoning behind type choices.

### Use pattern matching and guards with typespecs

Combine typespecs with pattern matching and guards to be explicit about incoming argument expectations:

```elixir
@spec change_record(Record.t(), map()) :: Ecto.Changeset.t()
def change_record(%Record{} = record, attrs) when is_map(attrs) do
  # ...
end
```

While Dialyzer is not a runtime enforcement tool, pattern matching and guards are. They help enforce expectations earlier in the call stack and surface issues sooner.

## Dependency Management

### Document your dependencies

Add a minimal description before listing each dependency to help future maintainers understand why it was added:

```elixir
defp deps do
  [
    # Core framework
    {:phoenix, "~> 1.7"},

    # For database operations
    {:ecto_sql, "~> 3.10"},

    # To render Markdown
    {:earmark, "~> 1.4"},

    # For security scans
    {:sobelow, "~> 0.13", only: [:dev, :test], runtime: false}
  ]
end
```

### Document and validate function options

If a function accepts options as the final argument, document them and validate them:

```elixir
@doc """
Creates a changeset for the record.

## Options

  * `:action` - An optional atom applied to the changeset, useful for forms that
    look to a changeset's action to influence form behavior.
"""
@spec change_record(Record.t(), map(), keyword()) :: Ecto.Changeset.t()
def change_record(%Record{} = record, attrs, opts \\ []) do
  opts = Keyword.validate!(opts, action: nil)

  # ...
end
```

Validation ensures call sites don't include unexpected or typoed option keys and provides a clean space for default values.

## Database Design

### Strive for database precision

Be detail-oriented when building database tables:

- If something cannot be null, explicitly use `NOT NULL`
- Use `:text` for potentially long strings
- If using `:string` (which has length limits), enforce those limits in `Ecto.Changeset` so values aren't silently trimmed
- Avoid virtual fields on Ecto schemas (they're almost never the right answer)

## Version Control

### Compose PR titles for clarity and consistency

Work in focused PRs with clear titles indicating what is changing. Use prefixes like:

- `feat:` - New features
- `fix:` - Bug fixes
- `chore:` - Maintenance tasks
- `refactor:` - Code refactoring
- `docs:` - Documentation changes

These conventions enable automated release note generation and provide clear commit history.
