# EXPECTED: passes
# BAD PRACTICE: fallible functions returning `nil` or bare atoms for failure.
#   The Critic subagent (critic-elixir, mode code) detects this by reading the
#   source — functions named fetch/find/load that return `nil` as the failure
#   signal, or `:error` as a bare atom. Elixir runs the code fine; the
#   antipattern is that callers cannot distinguish "not found" from "something
#   genuinely went wrong", and nil-chained access crashes far from the cause.
Mix.install([])

defmodule TaggedTupleReturns.BadExample do
  @moduledoc false

  defmodule Store do
    @data %{1 => %{id: 1, email: "alice@test", role: :admin}}

    def fetch(id) when is_integer(id) do
      Map.get(@data, id)
    end
  end

  def validate(%{email: email}) when is_binary(email) and byte_size(email) > 0 do
    :ok
  end

  def validate(_), do: :error

  def create_account(params) do
    case validate(params) do
      :ok -> %{email: params.email, persisted: true}
      :error -> nil
    end
  end
end

alias TaggedTupleReturns.BadExample, as: B

%{id: 1} = B.Store.fetch(1)
nil = B.Store.fetch(99)

%{persisted: true} = B.create_account(%{email: "bob@test"})
nil = B.create_account(%{})

IO.puts("tagged-tuple-returns: bad example ran (antipattern — see RULE.md)")
