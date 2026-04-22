# EXPECTED: passes
# BAD PRACTICE: nested if/else on struct fields that could be a multi-clause match.
#   The Critic subagent (critic-elixir, mode code) detects this by reading the
#   source and flagging the `if user.status == ...` chain as an unfolded pattern
#   match. Elixir itself runs the code fine — runtime exit is 0 per the schema
#   exit-code contract. Adding `:suspended` as a fourth status silently falls
#   through to the `:inactive` branch because the author forgot to extend the
#   chain. Adding `:editor` as a fourth role means another nested `if`.
Mix.install([])

defmodule PatternMatchOverConditionals.BadExample do
  @moduledoc false

  def process(user) do
    if user.status == :active do
      if user.role == :admin do
        :allowed
      else
        :denied
      end
    else
      :inactive
    end
  end

  def format(value) do
    cond do
      is_binary(value) -> String.trim(value)
      is_integer(value) -> Integer.to_string(value)
      true -> inspect(value)
    end
  end
end

alias PatternMatchOverConditionals.BadExample, as: B

:allowed = B.process(%{status: :active, role: :admin})
:denied = B.process(%{status: :active, role: :editor})
:inactive = B.process(%{status: :suspended, role: :admin})

"abc" = B.format("  abc  ")
"42" = B.format(42)
":atom" = B.format(:atom)

IO.puts("pattern-match-over-conditionals: bad example ran (antipattern — see RULE.md)")
