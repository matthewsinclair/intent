# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-TEST-001 (strong-assertions):
#   pattern-match assertion that pins every field the function is contracted
#   to produce. Any regression in id, name, or role fails this test with a
#   precise diagnostic at the binding site.
Mix.install([])

ExUnit.start(autorun: true)

defmodule StrongAssertionsGoodTest do
  use ExUnit.Case, async: true

  defmodule UserService do
    @moduledoc false

    defstruct [:id, :name, :role]

    def fetch(id) when is_integer(id) do
      %__MODULE__{id: id, name: "Default", role: :viewer}
    end
  end

  test "fetch/1 returns the user with the requested id and default viewer role" do
    assert %UserService{id: 42, name: "Default", role: :viewer} = UserService.fetch(42)
  end

  test "fetch/1 propagates the requested id into the returned struct" do
    assert %UserService{id: id} = UserService.fetch(7)
    assert id == 7
  end
end
