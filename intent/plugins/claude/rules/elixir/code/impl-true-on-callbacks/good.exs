# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-CODE-003 (impl-true-on-callbacks):
#   every GenServer callback is annotated with `@impl true`. A typo in the
#   callback name would now fail at compile time rather than silently
#   dispatching to a custom function.
Mix.install([])

defmodule ImplTrueOnCallbacks.GoodExample do
  use GenServer

  # --- Public API (not callbacks — correctly unannotated)
  def start_link(initial), do: GenServer.start_link(__MODULE__, initial)
  def ping(pid), do: GenServer.call(pid, :ping)
  def count(pid), do: GenServer.call(pid, :count)

  # --- Callbacks (each annotated)
  @impl true
  def init(initial), do: {:ok, %{count: initial}}

  @impl true
  def handle_call(:ping, _from, state), do: {:reply, :pong, state}

  @impl true
  def handle_call(:count, _from, %{count: n} = state), do: {:reply, n, state}
end

alias ImplTrueOnCallbacks.GoodExample, as: G

{:ok, pid} = G.start_link(41)
:pong = G.ping(pid)
41 = G.count(pid)

IO.puts("impl-true-on-callbacks: good example ok")
