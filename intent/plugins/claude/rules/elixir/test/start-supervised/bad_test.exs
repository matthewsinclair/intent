# EXPECTED: passes
# BAD PRACTICE: Agent.start_link in setup without start_supervised.
#   The Critic subagent (critic-elixir, mode test) detects this by reading the
#   source — `Agent.start_link`, `GenServer.start_link`, or `Task.start_link`
#   in `setup`/`setup_all`/test body with no `start_supervised`. ExUnit passes
#   fine in isolation; the antipattern is that the Agent outlives the test
#   and — in a real suite — would collide with later tests that start a
#   process under the same registered name, or would leak accumulated state.
Mix.install([])

ExUnit.start(autorun: true)

defmodule StartSupervisedBadTest do
  use ExUnit.Case, async: true

  defmodule Cache do
    use Agent

    def start_link(opts) do
      initial = Keyword.get(opts, :initial, %{})
      Agent.start_link(fn -> initial end)
    end

    def get(pid, key), do: Agent.get(pid, &Map.get(&1, key))
    def put(pid, key, value), do: Agent.update(pid, &Map.put(&1, key, value))
  end

  setup do
    # Antipattern — bare start_link, no supervision, no on_exit.
    {:ok, pid} = Cache.start_link(initial: %{})
    %{cache: pid}
  end

  test "cache stores and retrieves a value", %{cache: pid} do
    :ok = Cache.put(pid, :k, 42)
    assert Cache.get(pid, :k) == 42
  end
end
