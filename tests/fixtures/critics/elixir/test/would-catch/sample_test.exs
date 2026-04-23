defmodule CriticFixtures.Elixir.WouldCatch.ServiceTest do
  use ExUnit.Case

  test "fetch returns a user" do
    user = Accounts.Service.fetch(42)
    assert is_struct(user, Accounts.User)
    refute is_nil(user.role)
  end

  test "counter increments after cast" do
    {:ok, pid} = Counter.start_link(0)
    Counter.increment(pid)
    Process.sleep(100)
    assert Counter.value(pid) == 1
  end

  test "creates order with pending status" do
    case Orders.create(%{item: "widget"}) do
      {:ok, order} -> assert order.status == :pending
      {:error, _} -> flunk("should not fail")
    end
  end
end
