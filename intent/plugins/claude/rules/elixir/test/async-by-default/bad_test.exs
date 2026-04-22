# EXPECTED: passes
# BAD PRACTICE: test module without `async: true` for stateless logic.
#   The Critic subagent (critic-elixir, mode test) detects this by reading the
#   source — `use ExUnit.Case` with no `, async: true` in a module that does
#   not mutate Application env, named ETS, or singleton processes. ExUnit
#   passes fine; the antipattern is that the test runs sequentially for no
#   reason, slowing the suite and hiding any isolation bugs that would have
#   been caught by `async: true`.
Mix.install([])

ExUnit.start(autorun: true)

defmodule AsyncByDefaultBadTest do
  use ExUnit.Case

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
