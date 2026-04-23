defmodule CriticFixtures.Elixir.WouldCatch.Accounts do
  @moduledoc false

  def authorise(user) do
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

  def valid_email?(s), do: Regex.match?(~r/^[^@\s]+@[^@\s]+\.[^@\s]+$/, s)

  def charge(card) do
    case Payments.charge(card) do
      {:ok, receipt} -> receipt
      _ -> nil
    end
  end
end

defmodule CriticFixtures.Elixir.WouldCatch.Webhooks do
  @moduledoc false

  def valid_email?(s), do: Regex.match?(~r/^\S+@\S+$/, s)

  def deliver(payload) do
    try do
      HTTP.post(payload)
      :ok
    rescue
      _ -> :ok
    end
  end
end
