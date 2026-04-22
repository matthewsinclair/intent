# EXPECTED: passes
# BAD PRACTICE: stubbing an internal module (`Accounts`) so the test never
#   exercises the real get_user!/1 path.
#   The Critic subagent (critic-elixir, mode test) detects this by reading the
#   source — a test that swaps out its own app's Accounts-like module with a
#   hand-rolled stub or a Mox expectation. ExUnit passes fine; the antipattern
#   is that the test is green because the stub returned the shape the caller
#   expected, not because the real code actually works. Refactors of Accounts
#   break this test even when the real behaviour is unchanged.
Mix.install([])

ExUnit.start(autorun: true)

defmodule RealCodeOverMocksBadTest do
  use ExUnit.Case, async: false

  defmodule Accounts do
    @callback get_user!(id :: integer()) :: map()
  end

  # Antipattern: a stub for an internal module, returning a fabricated user.
  # A refactor that drops :name from the real user shape would not fail this
  # test — the stub keeps returning the old shape.
  defmodule StubAccounts do
    @behaviour Accounts

    @impl true
    def get_user!(id) do
      %{id: id, email: "stub-#{id}@example.com", name: "Stub User"}
    end
  end

  defmodule Mailer do
    @callback deliver(email :: map()) :: {:ok, map()}
  end

  defmodule TestMailer do
    @behaviour Mailer

    @impl true
    def deliver(email) do
      Process.put(:delivered_email, email)
      {:ok, %{id: "msg-1"}}
    end
  end

  defmodule Notifications do
    def send_welcome(user_id, accounts \\ StubAccounts, mailer \\ TestMailer) do
      user = accounts.get_user!(user_id)
      email = %{to: user.email, subject: "Welcome, #{user.name}"}

      case mailer.deliver(email) do
        {:ok, _} -> :ok
        {:error, reason} -> {:error, reason}
      end
    end
  end

  test "success: sends welcome email (antipattern: stubbed internal Accounts)" do
    assert :ok = Notifications.send_welcome(42)

    delivered = Process.get(:delivered_email)
    assert delivered.to == "stub-42@example.com"
  end
end
