# EXPECTED: passes
# BAD PRACTICE: nested `case` blocks over tagged tuples instead of `with`.
#   The Critic subagent (critic-elixir, mode code) detects this by reading the
#   source — three levels of nesting with `error -> error` forwarders. Elixir
#   runs the code fine; the antipattern is that the happy path is buried deep
#   and every new fallible step adds another indent and another forwarder.
Mix.install([])

defmodule WithForRailway.BadExample do
  @moduledoc false

  def validate(%{amount: amount}) when is_integer(amount) and amount > 0 do
    {:ok, %{amount: amount}}
  end

  def validate(_), do: {:error, :invalid_params}

  def charge_payment(%{amount: amount}) do
    {:ok, %{payment_id: "pay-#{amount}", amount: amount}}
  end

  def save_order(validated, payment) do
    {:ok, Map.merge(validated, %{payment: payment.payment_id, status: :confirmed})}
  end

  def create_order(params) do
    case validate(params) do
      {:ok, validated} ->
        case charge_payment(validated) do
          {:ok, payment} ->
            case save_order(validated, payment) do
              {:ok, order} -> {:ok, order}
              error -> error
            end

          error ->
            error
        end

      error ->
        error
    end
  end
end

alias WithForRailway.BadExample, as: B

{:ok, %{amount: 100, payment: "pay-100", status: :confirmed}} =
  B.create_order(%{amount: 100})

{:error, :invalid_params} = B.create_order(%{amount: -5})

IO.puts("with-for-railway: bad example ran (antipattern — see RULE.md)")
