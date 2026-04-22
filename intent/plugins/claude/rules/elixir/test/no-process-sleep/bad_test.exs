# EXPECTED: passes
# BAD PRACTICE: Process.sleep used to synchronise with an async cast.
#   The Critic subagent (critic-elixir, mode test) detects this by reading the
#   source — `Process.sleep(N)` in a test body, followed by an assertion that
#   reads state from another process. ExUnit passes (the 50ms is enough on a
#   developer laptop) — runtime exit is 0 per the schema exit-code contract.
#   The antipattern is that this test is a race: it fails on slow CI or under
#   load, and wastes 50ms on every run on fast machines regardless.
Mix.install([])

ExUnit.start(autorun: true)

defmodule NoProcessSleepBadTest do
  use ExUnit.Case, async: true

  defmodule Counter do
    use GenServer

    def start_link(initial), do: GenServer.start_link(__MODULE__, initial)
    def increment(pid), do: GenServer.cast(pid, :increment)
    def value(pid), do: GenServer.call(pid, :value)

    @impl true
    def init(initial), do: {:ok, initial}

    @impl true
    def handle_cast(:increment, n), do: {:noreply, n + 1}

    @impl true
    def handle_call(:value, _from, n), do: {:reply, n, n}
  end

  test "counter increments after async cast (sleep-to-synchronise antipattern)" do
    {:ok, pid} = Counter.start_link(0)
    Counter.increment(pid)
    Process.sleep(50)
    assert Counter.value(pid) == 1
  end
end
