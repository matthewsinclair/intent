# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-CODE-004 (with-for-railway):
#   three fallible steps composed with `with`. The happy path reads top-to-
#   bottom; any `{:error, _}` short-circuits and is returned as-is.
Mix.install([])

defmodule WithForRailway.GoodExample do
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
    with {:ok, validated} <- validate(params),
         {:ok, payment} <- charge_payment(validated),
         {:ok, order} <- save_order(validated, payment) do
      {:ok, order}
    end
  end
end

alias WithForRailway.GoodExample, as: G

{:ok, %{amount: 100, payment: "pay-100", status: :confirmed}} =
  G.create_order(%{amount: 100})

{:error, :invalid_params} = G.create_order(%{amount: -5})
{:error, :invalid_params} = G.create_order(%{})

IO.puts("with-for-railway: good example ok")
