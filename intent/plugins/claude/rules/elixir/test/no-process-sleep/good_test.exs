# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-TEST-002 (no-process-sleep):
#   synchronise by `GenServer.call` (serialises after any pending cast) or by
#   `assert_receive` (explicit message wait). No wall-clock sleeping.
Mix.install([])

ExUnit.start(autorun: true)

defmodule NoProcessSleepGoodTest do
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

  defmodule Notifier do
    def ping_async(parent) do
      spawn(fn -> send(parent, {:done, :ok}) end)
    end
  end

  test "counter increments after async cast (sync-via-call)" do
    {:ok, pid} = Counter.start_link(0)
    Counter.increment(pid)
    # The call cannot return until the preceding cast is processed.
    assert Counter.value(pid) == 1
  end

  test "notifier delivers completion message (sync-via-receive)" do
    Notifier.ping_async(self())
    assert_receive {:done, :ok}, 1000
  end
end
