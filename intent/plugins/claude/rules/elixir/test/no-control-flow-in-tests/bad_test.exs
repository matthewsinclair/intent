# EXPECTED: passes
# BAD PRACTICE: `case` / `if` inside test bodies deciding which assertion fires.
#   The Critic subagent (critic-elixir, mode test) detects this by reading the
#   source — `case`, `if`, `unless`, or `cond` at the top level of a `test`
#   block. ExUnit passes fine here; the antipattern is that each test is
#   really two tests sharing one body, and a failure prints a branch-internal
#   assertion rather than the contract the test was supposed to pin.
Mix.install([])

ExUnit.start(autorun: true)

defmodule NoControlFlowInTestsBadTest do
  use ExUnit.Case, async: true

  defmodule Orders do
    def create(%{valid: true}), do: {:ok, %{id: 1, status: :pending}}
    def create(%{valid: false}), do: {:error, :invalid}
  end

  defmodule Access do
    def allowed?(%{role: :admin}), do: true
    def allowed?(%{role: _}), do: false
  end

  test "creates order (antipattern: case inside test body)" do
    case Orders.create(%{valid: true}) do
      {:ok, order} -> assert order.status == :pending
      {:error, _} -> flunk("should not fail")
    end
  end

  test "admin or user can view (antipattern: if inside test body)" do
    admin? = true

    if admin? do
      assert Access.allowed?(%{role: :admin})
    else
      refute Access.allowed?(%{role: :user})
    end
  end
end
