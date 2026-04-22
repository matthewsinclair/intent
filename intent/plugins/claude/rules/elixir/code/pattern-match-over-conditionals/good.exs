# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-CODE-001 (pattern-match-over-conditionals):
#   multi-clause function heads destructure on the shape of the argument; guards
#   handle the type-class tail. Adding a new role is one new clause, not a nested
#   refactor. An unhandled shape raises FunctionClauseError at the binding site.
Mix.install([])

defmodule PatternMatchOverConditionals.GoodExample do
  @moduledoc false

  def process(%{status: :active, role: :admin}), do: :allowed
  def process(%{status: :active, role: :editor}), do: :allowed
  def process(%{status: :active}), do: :denied
  def process(%{status: :suspended}), do: :locked
  def process(_), do: :inactive

  def format(value) when is_binary(value), do: String.trim(value)
  def format(value) when is_integer(value), do: Integer.to_string(value)
  def format(value), do: inspect(value)
end

alias PatternMatchOverConditionals.GoodExample, as: G

:allowed = G.process(%{status: :active, role: :admin})
:allowed = G.process(%{status: :active, role: :editor})
:denied = G.process(%{status: :active, role: :viewer})
:locked = G.process(%{status: :suspended, role: :admin})
:inactive = G.process(%{status: :banned})

"abc" = G.format("  abc  ")
"42" = G.format(42)
":atom" = G.format(:atom)

IO.puts("pattern-match-over-conditionals: good example ok")
