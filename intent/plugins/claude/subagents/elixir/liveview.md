# LiveView Patterns Reference

Comprehensive reference for Phoenix LiveView operational patterns. Covers lifecycle, rendering, navigation, streams, and common pitfalls.

## Two-Phase Rendering

LiveView `mount/3` is called twice:

1. **Static render** (HTTP request) — generates initial HTML for fast page load. Socket is NOT connected. PubSub subscriptions, timers, and async operations should NOT start here.
2. **Connected render** (WebSocket) — establishes persistent connection. Socket IS connected. Start PubSub, timers, and async work here.

```elixir
@impl true
def mount(_params, _session, socket) do
  # Phase 1: Always runs — set up basic assigns
  socket = assign(socket, :page_title, "Dashboard")

  # Phase 2: Only when connected — start live features
  socket =
    if connected?(socket) do
      Phoenix.PubSub.subscribe(MyApp.PubSub, "updates")
      assign_async(socket, :stats, fn -> {:ok, %{stats: load_stats()}} end)
    else
      assign(socket, :stats, AsyncResult.loading())
    end

  {:ok, socket}
end
```

### Common Bug: PubSub in Static Render

```elixir
# BAD — subscribes during static render, message arrives before WebSocket connects
@impl true
def mount(_params, _session, socket) do
  Phoenix.PubSub.subscribe(MyApp.PubSub, "updates")
  {:ok, socket}
end

# GOOD — subscribe only when connected
@impl true
def mount(_params, _session, socket) do
  if connected?(socket) do
    Phoenix.PubSub.subscribe(MyApp.PubSub, "updates")
  end

  {:ok, socket}
end
```

## `assign_async/3` for Non-Blocking Data Loading

Never block `mount/3` with expensive data loading. Use `assign_async/3` to load data concurrently after the initial render.

```elixir
@impl true
def mount(_params, _session, socket) do
  socket =
    socket
    |> assign(:page_title, "Dashboard")
    |> assign_async(:recent_posts, fn ->
      {:ok, %{recent_posts: MyApp.Content.list_recent_posts!()}}
    end)
    |> assign_async(:user_stats, fn ->
      {:ok, %{user_stats: MyApp.Analytics.get_stats!()}}
    end)

  {:ok, socket}
end
```

In templates, handle the async states:

```heex
<.async_result :let={posts} assign={@recent_posts}>
  <:loading>Loading posts...</:loading>
  <:failed :let={_reason}>Failed to load posts</:failed>
  <ul>
    <li :for={post <- posts}>{post.title}</li>
  </ul>
</.async_result>
```

## Streams for Large Lists

Never assign full collections for lists that grow or update frequently. Use `stream/3` for memory-efficient list rendering.

```elixir
# BAD — full list in assigns, entire list re-rendered on any change
@impl true
def mount(_params, _session, socket) do
  {:ok, assign(socket, :messages, list_all_messages())}
end

@impl true
def handle_info({:new_message, message}, socket) do
  {:noreply, update(socket, :messages, &[message | &1])}
end

# GOOD — stream, only new items are rendered
@impl true
def mount(_params, _session, socket) do
  {:ok, stream(socket, :messages, list_all_messages())}
end

@impl true
def handle_info({:new_message, message}, socket) do
  {:noreply, stream_insert(socket, :messages, message, at: 0)}
end
```

In templates:

```heex
<ul id="messages" phx-update="stream">
  <li :for={{dom_id, message} <- @streams.messages} id={dom_id}>
    {message.body}
  </li>
</ul>
```

### Stream Operations

| Operation          | Function                                         | Use Case                  |
| ------------------ | ------------------------------------------------ | ------------------------- |
| Initialize         | `stream(socket, :items, items)`                  | Mount/reset               |
| Insert/update      | `stream_insert(socket, :items, item)`            | Add or update single item |
| Insert at position | `stream_insert(socket, :items, item, at: 0)`     | Prepend to list           |
| Delete             | `stream_delete(socket, :items, item)`            | Remove single item        |
| Reset              | `stream(socket, :items, new_items, reset: true)` | Replace entire stream     |

## Navigation: `push_navigate` vs `push_patch`

### `push_patch/2` — Same LiveView, Different Params

Updates URL and triggers `handle_params/3`. Does NOT remount. Use for filtering, pagination, sorting within the same view.

```elixir
# Navigating within the same LiveView
def handle_event("filter", %{"status" => status}, socket) do
  {:noreply, push_patch(socket, to: ~p"/posts?status=#{status}")}
end

@impl true
def handle_params(params, _uri, socket) do
  status = params["status"] || "all"
  posts = MyApp.Content.list_posts!(query: [filter: [status: status]])
  {:noreply, assign(socket, :posts, posts)}
end
```

### `push_navigate/2` — Different LiveView

Triggers full mount of the target LiveView. The current LiveView is unmounted.

```elixir
# Navigating to a different LiveView
def handle_event("view_post", %{"id" => id}, socket) do
  {:noreply, push_navigate(socket, to: ~p"/posts/#{id}")}
end
```

### Decision Guide

| Scenario                  | Use                                           | Why                             |
| ------------------------- | --------------------------------------------- | ------------------------------- |
| Pagination                | `push_patch`                                  | Same view, different data slice |
| Filtering                 | `push_patch`                                  | Same view, different filter     |
| Tab switching (same page) | `push_patch`                                  | Same view, different section    |
| Going to detail page      | `push_navigate`                               | Different LiveView              |
| Form submission redirect  | `push_navigate`                               | Different LiveView              |
| Redirect after action     | `redirect` (in controller) or `push_navigate` | Leave current context           |

## Component Patterns

### Function Components

```elixir
# In a component module
defmodule MyAppWeb.Components.UI.Badge do
  use Phoenix.Component

  attr :color, :atom, default: :gray, values: [:gray, :green, :red, :yellow]
  attr :label, :string, required: true

  def badge(assigns) do
    ~H"""
    <span class={["badge", badge_color(@color)]}>
      {@label}
    </span>
    """
  end

  defp badge_color(:gray), do: "badge-gray"
  defp badge_color(:green), do: "badge-green"
  defp badge_color(:red), do: "badge-red"
  defp badge_color(:yellow), do: "badge-yellow"
end
```

### Live Components

Use for stateful, interactive pieces within a LiveView:

```elixir
defmodule MyAppWeb.PostLive.FormComponent do
  use MyAppWeb, :live_component

  @impl true
  def render(assigns) do
    ~H"""
    <.form for={@form} phx-submit="save" phx-target={@myself}>
      <.input field={@form[:title]} label="Title" />
      <.button>Save</.button>
    </.form>
    """
  end

  @impl true
  def handle_event("save", %{"post" => params}, socket) do
    case MyApp.Content.create_post(params, actor: socket.assigns.current_user) do
      {:ok, post} -> {:noreply, push_navigate(socket, to: ~p"/posts/#{post.id}")}
      {:error, changeset} -> {:noreply, assign(socket, :form, to_form(changeset))}
    end
  end
end
```

## File Uploads

### Configuration

```elixir
@impl true
def mount(_params, _session, socket) do
  {:ok,
   socket
   |> allow_upload(:avatar,
     accept: ~w(.jpg .jpeg .png .webp),
     max_entries: 1,
     max_file_size: 5_000_000
   )}
end
```

### Handling Uploads

```elixir
@impl true
def handle_event("save", _params, socket) do
  uploaded_files =
    consume_uploaded_entries(socket, :avatar, fn %{path: path}, entry ->
      dest = Path.join(["priv", "static", "uploads", "#{entry.uuid}-#{entry.client_name}"])
      File.cp!(path, dest)
      {:ok, ~p"/uploads/#{Path.basename(dest)}"}
    end)

  {:noreply, update(socket, :uploaded_files, &(&1 ++ uploaded_files))}
end
```

### Upload Template

```heex
<.live_file_input upload={@uploads.avatar} />

<article :for={entry <- @uploads.avatar.entries}>
  <.live_img_preview entry={entry} />
  <progress value={entry.progress} max="100">{entry.progress}%</progress>
  <button phx-click="cancel-upload" phx-value-ref={entry.ref}>Cancel</button>
</article>
```

### Security Considerations

- Always validate file types server-side (`accept` option)
- Set reasonable `max_file_size` limits
- Generate unique filenames (use `entry.uuid`) to prevent path traversal
- Store uploads outside the web root in production (use cloud storage)
- Never trust `entry.client_name` for the final filename

## Error Handling

### Handling Errors in LiveView

```elixir
@impl true
def handle_event("delete", %{"id" => id}, socket) do
  case MyApp.Content.delete_post(id, actor: socket.assigns.current_user) do
    :ok ->
      {:noreply,
       socket
       |> put_flash(:info, "Post deleted")
       |> push_navigate(to: ~p"/posts")}

    {:error, _reason} ->
      {:noreply, put_flash(socket, :error, "Could not delete post")}
  end
end
```

### `handle_info/2` for Async Results

```elixir
@impl true
def handle_info({:new_post, post}, socket) do
  {:noreply, stream_insert(socket, :posts, post, at: 0)}
end

@impl true
def handle_info({ref, result}, socket) when is_reference(ref) do
  # Handle Task.async results
  Process.demonitor(ref, [:flush])
  {:noreply, assign(socket, :result, result)}
end
```

## LiveView Lifecycle Summary

```
HTTP Request
  └─> mount/3 (disconnected)
       └─> handle_params/3
            └─> render/1 → HTML sent to browser

WebSocket Connect
  └─> mount/3 (connected)
       └─> handle_params/3
            └─> render/1 → LiveView active

User Interaction
  └─> handle_event/3
       └─> render/1 → diff sent

URL Change (push_patch)
  └─> handle_params/3
       └─> render/1 → diff sent

PubSub / Process Messages
  └─> handle_info/2
       └─> render/1 → diff sent
```

## Common Pitfalls

1. **Blocking mount** — Use `assign_async` for expensive operations
2. **PubSub before connected** — Guard with `connected?(socket)`
3. **Full list assigns** — Use streams for dynamic lists
4. **Business logic in handle_event** — Delegate to domain modules
5. **Missing `@impl true`** — Always annotate callbacks
6. **`push_patch` to different LiveView** — Use `push_navigate` instead
7. **Forgetting `phx-update="stream"`** — Required on stream containers
8. **Not handling async error states** — Always handle `:loading` and `:failed`
