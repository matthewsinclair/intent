# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-CODE-002 (tagged-tuple-returns):
#   every fallible function returns `{:ok, value}` or `{:error, reason}`. The
#   caller composes with `with` and pattern matches on the tag — no nil-checks,
#   no silent failures.
Mix.install([])

defmodule TaggedTupleReturns.GoodExample do
  @moduledoc false

  defmodule Store do
    @data %{1 => %{id: 1, email: "alice@test", role: :admin}}

    def fetch(id) when is_integer(id) do
      case Map.get(@data, id) do
        nil -> {:error, :not_found}
        user -> {:ok, user}
      end
    end
  end

  def validate(%{email: email}) when is_binary(email) and byte_size(email) > 0 do
    {:ok, %{email: email}}
  end

  def validate(_), do: {:error, :invalid_params}

  def persist({:validated, record}), do: {:ok, Map.put(record, :persisted, true)}

  def create_account(params) do
    with {:ok, validated} <- validate(params),
         {:ok, stored} <- persist({:validated, validated}) do
      {:ok, stored}
    end
  end
end

alias TaggedTupleReturns.GoodExample, as: G

{:ok, %{id: 1}} = G.Store.fetch(1)
{:error, :not_found} = G.Store.fetch(99)

{:ok, %{persisted: true}} = G.create_account(%{email: "bob@test"})
{:error, :invalid_params} = G.create_account(%{})

IO.puts("tagged-tuple-returns: good example ok")
