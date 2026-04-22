# EXPECTED: passes
# BAD PRACTICE: two modules each carry their own email-validation regex.
#   The Critic subagent (critic-elixir, mode code) detects this by reading the
#   source — two different modules with a `valid_email?/1` function, each
#   containing its own regex. Elixir runs the code fine; the antipattern is
#   that the two regexes have already drifted (one accepts "a@b" without a
#   TLD; the other requires one), so "is this email valid?" depends on which
#   module you asked.
Mix.install([])

defmodule ModuleHighlander.BadExample do
  @moduledoc false

  defmodule Accounts do
    # Original, stricter regex
    def valid_email?(s) when is_binary(s) do
      Regex.match?(~r/^[^@\s]+@[^@\s]+\.[^@\s]+$/, s)
    end

    def valid_email?(_), do: false
  end

  defmodule Webhooks do
    # Forked copy — looser regex, nobody remembers why
    def valid_email?(s) when is_binary(s) do
      Regex.match?(~r/^\S+@\S+$/, s)
    end

    def valid_email?(_), do: false
  end
end

alias ModuleHighlander.BadExample, as: B

# The drift is observable: Accounts says "no", Webhooks says "yes"
false = B.Accounts.valid_email?("a@b")
true = B.Webhooks.valid_email?("a@b")

true = B.Accounts.valid_email?("alice@test.com")
true = B.Webhooks.valid_email?("alice@test.com")

IO.puts("module-highlander: bad example ran (antipattern — see RULE.md)")
