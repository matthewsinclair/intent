# EXPECTED: passes
# BAD PRACTICE: catch-all rescues returning :ok, discarded side-effect results,
#   and case-fallback arms that convert errors to nil.
#   The Critic subagent (critic-elixir, mode code) detects this by reading the
#   source — `rescue _ -> :ok`, `_ = fallible_call()`, `_ -> nil` on a case
#   fallback after a call with a failure mode. Elixir runs the code fine; the
#   antipattern is that every failure becomes success-shaped and operators
#   never learn the system is broken.
Mix.install([])

defmodule NoSilentFailures.BadExample do
  @moduledoc false

  defmodule EventError, do: defexception([:message])

  def risky_insert(%{kind: :boom}), do: raise(EventError, message: "boom")
  def risky_insert(event), do: event

  def notify_stakeholders(%{notify?: false}), do: {:error, :notify_disabled}
  def notify_stakeholders(_), do: {:ok, :notified}

  def charge(%{card: "declined"}), do: {:error, :declined}
  def charge(%{card: _}), do: {:ok, %{receipt_id: "r-1"}}

  # Antipattern #1 — catch-all rescue returning :ok.
  def record_event(event) do
    try do
      _ = risky_insert(event)
      :ok
    rescue
      _ -> :ok
    end
  end

  # Antipattern #2 — discarded fallible result.
  def process(params) do
    _ = notify_stakeholders(params)
    {:ok, :done}
  end

  # Antipattern #3 — case fallback swallows errors to nil.
  def maybe_charge(card) do
    case charge(%{card: card}) do
      {:ok, receipt} -> receipt
      _ -> nil
    end
  end
end

alias NoSilentFailures.BadExample, as: B

:ok = B.record_event(%{id: 1})
:ok = B.record_event(%{kind: :boom})

{:ok, :done} = B.process(%{notify?: true})
{:ok, :done} = B.process(%{notify?: false})

%{receipt_id: "r-1"} = B.maybe_charge("4242")
nil = B.maybe_charge("declined")

IO.puts("no-silent-failures: bad example ran (antipattern — see RULE.md)")
