defmodule CriticFixtures.Elixir.WouldMiss.ServiceTest do
  use ExUnit.Case, async: true

  describe "Accounts.Service.fetch/1" do
    test "returns the user with the requested id and default viewer role" do
      assert %Accounts.User{id: 42, name: "Default", role: :viewer} =
               Accounts.Service.fetch(42)
    end
  end

  describe "Counter" do
    test "value reflects preceding increment via synchronous call" do
      {:ok, pid} = start_supervised({Counter, 0})
      Counter.increment(pid)
      assert Counter.value(pid) == 1
    end
  end

  describe "Orders.create/1" do
    test "creates order with pending status for valid params" do
      assert {:ok, %Orders.Order{status: :pending}} =
               Orders.create(%{item: "widget"})
    end

    test "returns a missing-item error tuple for invalid params" do
      assert {:error, :missing_item} = Orders.create(%{})
    end
  end
end
