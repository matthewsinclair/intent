---
description: "Phoenix LiveView lifecycle rules: two-phase mount, streams, async loading, thin LiveViews, components"
---

# Phoenix LiveView Essentials

LiveView lifecycle and rendering rules enforced on every LiveView module. These are mandatory -- no exceptions.

## Rules

### 1. Two-phase mount -- guard async operations with `connected?(socket)`

Mount is called twice: once for static HTML render (disconnected), once for WebSocket (connected). Never subscribe to PubSub, start timers, or spawn async work during the static render.

```elixir
# BAD -- subscribes during static render
@impl true
def mount(_params, _session, socket) do
  Phoenix.PubSub.subscribe(MyApp.PubSub, "updates")
  {:ok, assign(socket, :items, load_items())}
end

# GOOD -- guard with connected?
@impl true
def mount(_params, _session, socket) do
  if connected?(socket) do
    Phoenix.PubSub.subscribe(MyApp.PubSub, "updates")
  end

  {:ok, assign(socket, :items, load_items())}
end
```

### 2. Streams for large or dynamic lists

Never assign full collections that grow or update frequently. Use `stream/3` for memory-efficient rendering where only changes are sent to the client.

```elixir
# BAD -- full list re-rendered on every update
@impl true
def mount(_params, _session, socket) do
  {:ok, assign(socket, :messages, list_messages())}
end

def handle_info({:new_message, msg}, socket) do
  {:noreply, update(socket, :messages, &[msg | &1])}
end

# GOOD -- stream, only diffs sent
@impl true
def mount(_params, _session, socket) do
  {:ok, stream(socket, :messages, list_messages())}
end

def handle_info({:new_message, msg}, socket) do
  {:noreply, stream_insert(socket, :messages, msg, at: 0)}
end
```

Template requires `phx-update="stream"`:

```heex
<ul id="messages" phx-update="stream">
  <li :for={{dom_id, msg} <- @streams.messages} id={dom_id}>{msg.body}</li>
</ul>
```

### 3. `@impl true` on all LiveView callbacks

Every callback must be annotated. Catches typos at compile time and makes callback vs custom function distinction clear.

```elixir
# BAD
def mount(_params, _session, socket), do: {:ok, socket}
def handle_event("click", _params, socket), do: {:noreply, socket}

# GOOD
@impl true
def mount(_params, _session, socket), do: {:ok, socket}

@impl true
def handle_event("click", _params, socket), do: {:noreply, socket}
```

### 4. Thin LiveViews -- domain logic in context/domain modules

LiveViews are coordinators. They assign state, dispatch to domain, and update assigns. No business logic, data transformation, or aggregation queries.

```elixir
# BAD -- business logic in LiveView
@impl true
def handle_event("publish", %{"id" => id}, socket) do
  post = MyApp.Content.get_post!(id)
  if post.status == :draft and post.word_count > 100 do
    MyApp.Content.update_post(post, %{status: :published, published_at: DateTime.utc_now()})
    # send notification, update analytics...
  end
end

# GOOD -- domain function handles all logic
@impl true
def handle_event("publish", %{"id" => id}, socket) do
  case MyApp.Content.publish_post(id, actor: socket.assigns.current_user) do
    {:ok, post} -> {:noreply, stream_insert(socket, :posts, post)}
    {:error, reason} -> {:noreply, put_flash(socket, :error, reason)}
  end
end
```

### 5. `push_navigate` vs `push_patch` -- correct semantics

`push_patch` stays in the same LiveView (triggers `handle_params`). `push_navigate` goes to a different LiveView (triggers full mount).

```elixir
# GOOD -- same LiveView, updating filters
def handle_event("filter", %{"status" => status}, socket) do
  {:noreply, push_patch(socket, to: ~p"/posts?status=#{status}")}
end

# GOOD -- different LiveView, navigating away
def handle_event("view_details", %{"id" => id}, socket) do
  {:noreply, push_navigate(socket, to: ~p"/posts/#{id}")}
end

# BAD -- push_patch to a different LiveView (won't remount)
def handle_event("go_home", _params, socket) do
  {:noreply, push_patch(socket, to: ~p"/")}
end
```

### 6. `assign_async` for non-blocking data loading

Never block mount with expensive operations. Use `assign_async/3` for concurrent data loading after initial render.

```elixir
# BAD -- blocks initial render
@impl true
def mount(_params, _session, socket) do
  stats = MyApp.Analytics.compute_stats!()  # slow query
  {:ok, assign(socket, :stats, stats)}
end

# GOOD -- non-blocking, shows loading state
@impl true
def mount(_params, _session, socket) do
  {:ok,
   assign_async(socket, :stats, fn ->
     {:ok, %{stats: MyApp.Analytics.compute_stats!()}}
   end)}
end
```

Handle async states in template:

```heex
<.async_result :let={stats} assign={@stats}>
  <:loading>Computing stats...</:loading>
  <:failed :let={_reason}>Failed to load stats</:failed>
  <div>Total: {stats.total}</div>
</.async_result>
```

### 7. Extract repeated HEEX into reusable components

When the same HTML structure appears twice, extract it into a function component with typed attributes.

```elixir
# BAD -- duplicated markup across LiveViews
~H"""
<span class="badge badge-green">Active</span>
...
<span class="badge badge-red">Inactive</span>
"""

# GOOD -- reusable component
attr :color, :atom, values: [:green, :red, :yellow, :gray], required: true
attr :label, :string, required: true

def status_badge(assigns) do
  ~H"""
  <span class={["badge", "badge-#{@color}"]}>{@label}</span>
  """
end
```

Use `attr` declarations for compile-time validation of component inputs.
