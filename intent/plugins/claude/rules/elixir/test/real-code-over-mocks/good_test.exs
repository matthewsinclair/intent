# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-TEST-006 (real-code-over-mocks):
#   the real `Accounts` module is exercised directly; only the external mailer
#   boundary is stubbed. The test verifies the actual integration path —
#   Accounts.get_user!/1 really runs, Notifications really pulls a real user's
#   email, the mailer sees the real shape.
Mix.install([])

ExUnit.start(autorun: true)

defmodule RealCodeOverMocksGoodTest do
  use ExUnit.Case, async: false

  # --- Real domain code (not mocked)
  defmodule Accounts do
    @users %{1 => %{id: 1, email: "alice@test.com", name: "Alice"}}

    def get_user!(id) do
      Map.fetch!(@users, id)
    end
  end

  # --- External boundary: mailer behaviour + real stub (what a real external
  #     dependency would look like). The stub records what it was called with
  #     in the Process dictionary so the test can assert on the real shape.
  defmodule Mailer do
    @callback deliver(email :: map()) :: {:ok, map()} | {:error, term()}
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
    def send_welcome(user_id, mailer \\ TestMailer) do
      user = Accounts.get_user!(user_id)
      email = %{to: user.email, subject: "Welcome, #{user.name}", body: "..."}

      case mailer.deliver(email) do
        {:ok, _} -> :ok
        {:error, reason} -> {:error, reason}
      end
    end
  end

  test "success: sends welcome email with the real user's address" do
    assert :ok = Notifications.send_welcome(1)

    delivered = Process.get(:delivered_email)
    assert delivered.to == "alice@test.com"
    assert delivered.subject == "Welcome, Alice"
  end
end
