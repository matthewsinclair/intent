defmodule CriticFixtures.Elixir.WouldMiss.EmailAddress do
  @moduledoc "Canonical home for email-address validation and normalisation."

  @regex ~r/^[^@\s]+@[^@\s]+\.[^@\s]+$/

  def valid?(s) when is_binary(s), do: Regex.match?(@regex, s)
  def normalise(s) when is_binary(s), do: String.downcase(String.trim(s))
end

defmodule CriticFixtures.Elixir.WouldMiss.Authorisation do
  @moduledoc false

  def authorise(%{status: :active, role: :admin}), do: :allowed
  def authorise(%{status: :active}), do: :denied
  def authorise(_), do: :inactive
end

defmodule CriticFixtures.Elixir.WouldMiss.Billing do
  @moduledoc false

  alias CriticFixtures.Elixir.WouldMiss.EmailAddress

  def register(%{email: email, card: card}) do
    with true <- EmailAddress.valid?(email),
         {:ok, receipt} <- Payments.charge(card) do
      {:ok, receipt}
    else
      false -> {:error, :invalid_email}
      {:error, reason} -> {:error, {:payment_declined, reason}}
    end
  end
end
