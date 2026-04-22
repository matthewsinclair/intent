# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-TEST-004 (start-supervised):
#   setup uses `start_supervised!/2` so ExUnit tears the process down when
#   the test ends. No leak between tests; no name collision with later tests.
Mix.install([])

ExUnit.start(autorun: true)

defmodule StartSupervisedGoodTest do
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
    pid = start_supervised!({Cache, initial: %{}})
    %{cache: pid}
  end

  test "cache stores and retrieves a value", %{cache: pid} do
    :ok = Cache.put(pid, :k, 42)
    assert Cache.get(pid, :k) == 42
  end

  test "cache returns nil for missing keys", %{cache: pid} do
    assert Cache.get(pid, :missing) == nil
  end
end
