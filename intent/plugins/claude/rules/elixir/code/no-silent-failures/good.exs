# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-CODE-005 (no-silent-failures):
#   every failure is either propagated as `{:error, _}`, rescued with
#   attribution (inspect + log + tagged return), or explicitly handled by the
#   caller. No catch-all rescues returning :ok, no `_ = ...` discards.
Mix.install([])

defmodule NoSilentFailures.GoodExample do
  @moduledoc false

  defmodule EventError, do: defexception([:message])

  def risky_insert(%{kind: :boom}), do: raise(EventError, message: "boom")
  def risky_insert(event), do: event

  def notify_stakeholders(%{notify?: false}), do: {:error, :notify_disabled}
  def notify_stakeholders(_), do: {:ok, :notified}

  def charge(%{card: "declined"}), do: {:error, :declined}
  def charge(%{card: _}), do: {:ok, %{receipt_id: "r-1"}}

  def record_event(event) do
    try do
      {:ok, risky_insert(event)}
    rescue
      error in EventError ->
        {:error, {:event_write_failed, Exception.message(error)}}
    end
  end

  def process(params) do
    with {:ok, :notified} <- notify_stakeholders(params) do
      {:ok, :done}
    end
  end

  def maybe_charge(card) do
    case charge(%{card: card}) do
      {:ok, receipt} -> {:ok, receipt}
      {:error, reason} -> {:error, {:payment_declined, reason}}
    end
  end
end

alias NoSilentFailures.GoodExample, as: G

{:ok, %{id: 1}} = G.record_event(%{id: 1})
{:error, {:event_write_failed, "boom"}} = G.record_event(%{kind: :boom})

{:ok, :done} = G.process(%{notify?: true})
{:error, :notify_disabled} = G.process(%{notify?: false})

{:ok, %{receipt_id: "r-1"}} = G.maybe_charge("4242")
{:error, {:payment_declined, :declined}} = G.maybe_charge("declined")

IO.puts("no-silent-failures: good example ok")
