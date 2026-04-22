# EXPECTED: passes
# BAD PRACTICE: shape-only assertions on a fallible function's return.
#   The Critic subagent (critic-elixir, mode test-check) detects this by
#   reading the source and flagging `assert is_struct/2`, `assert is_integer/1`,
#   and `refute is_nil/1` as the sole evidence about the return value. ExUnit
#   itself passes — runtime exit is 0 per the schema exit-code contract. The
#   rule-schema doc calls out exactly this pattern.
Mix.install([])

ExUnit.start(autorun: true)

defmodule StrongAssertionsBadTest do
  use ExUnit.Case, async: true

  defmodule UserService do
    @moduledoc false

    defstruct [:id, :name, :role]

    def fetch(id) when is_integer(id) do
      %__MODULE__{id: id, name: "Default", role: :viewer}
    end
  end

  test "fetch/1 returns a user (shape-only — antipattern)" do
    user = UserService.fetch(42)

    # Each of these passes even if role were :banned, name were "", or id were wrong.
    assert is_struct(user, UserService)
    assert is_integer(user.id)
    refute is_nil(user.role)
  end
end
