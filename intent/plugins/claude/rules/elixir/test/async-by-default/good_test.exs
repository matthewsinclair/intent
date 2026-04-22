# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-TEST-003 (async-by-default):
#   the test module declares `async: true`. ExUnit is free to run this case
#   concurrently with any other async case — there is no shared mutable state,
#   so isolation is preserved by construction.
Mix.install([])

ExUnit.start(autorun: true)

defmodule AsyncByDefaultGoodTest do
  use ExUnit.Case, async: true

  defmodule Math do
    def add(a, b), do: a + b
    def mul(a, b), do: a * b
  end

  test "adds two numbers" do
    assert Math.add(1, 2) == 3
  end

  test "multiplies two numbers" do
    assert Math.mul(3, 4) == 12
  end
end
