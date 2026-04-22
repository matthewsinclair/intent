# EXPECTED: passes
# BAD PRACTICE: GenServer callbacks with no @impl true annotation.
#   The Critic subagent (critic-elixir, mode code) detects this by reading the
#   source — a module that `use`s a behaviour but has un-annotated `init/1`,
#   `handle_call/3`, `handle_cast/2`, `handle_info/2`, etc. Elixir runs the
#   code fine — runtime exit is 0 per the schema exit-code contract. The
#   antipattern is that a one-letter typo in a callback name would silently
#   dispatch to a custom function rather than failing at compile time.
Mix.install([])

defmodule ImplTrueOnCallbacks.BadExample do
  use GenServer

  def start_link(initial), do: GenServer.start_link(__MODULE__, initial)
  def ping(pid), do: GenServer.call(pid, :ping)
  def count(pid), do: GenServer.call(pid, :count)

  # --- Callbacks, no @impl true (the antipattern)
  def init(initial), do: {:ok, %{count: initial}}
  def handle_call(:ping, _from, state), do: {:reply, :pong, state}
  def handle_call(:count, _from, %{count: n} = state), do: {:reply, n, state}
end

alias ImplTrueOnCallbacks.BadExample, as: B

{:ok, pid} = B.start_link(41)
:pong = B.ping(pid)
41 = B.count(pid)

IO.puts("impl-true-on-callbacks: bad example ran (antipattern — see RULE.md)")
