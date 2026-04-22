# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-TEST-005 (no-control-flow-in-tests):
#   each test body is straight-line (setup, action, assert). Pattern-match
#   assertions (`assert {:ok, _} = call()`) replace `case`-with-flunk. Separate
#   roles produce separate tests instead of a single `if/else` test.
Mix.install([])

ExUnit.start(autorun: true)

defmodule NoControlFlowInTestsGoodTest do
  use ExUnit.Case, async: true

  defmodule Orders do
    def create(%{valid: true}), do: {:ok, %{id: 1, status: :pending}}
    def create(%{valid: false}), do: {:error, :invalid}
  end

  defmodule Access do
    def allowed?(%{role: :admin}), do: true
    def allowed?(%{role: _}), do: false
  end

  test "success: creates order with valid params" do
    assert {:ok, order} = Orders.create(%{valid: true})
    assert order.status == :pending
    assert order.id == 1
  end

  test "failure: rejects invalid params" do
    assert {:error, :invalid} = Orders.create(%{valid: false})
  end

  test "success: admin can view" do
    assert Access.allowed?(%{role: :admin})
  end

  test "success: non-admin cannot view" do
    refute Access.allowed?(%{role: :user})
  end
end
