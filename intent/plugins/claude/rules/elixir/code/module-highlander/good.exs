# EXPECTED: passes
#   Demonstrates the GOOD pattern for IN-EX-CODE-006 (module-highlander):
#   one canonical module owns email-address validation and normalisation;
#   every caller routes through it. A single change to the regex updates
#   every caller.
Mix.install([])

defmodule ModuleHighlander.GoodExample do
  @moduledoc false

  defmodule EmailAddress do
    @moduledoc "Canonical home for email-address validation and normalisation."

    @regex ~r/^[^@\s]+@[^@\s]+\.[^@\s]+$/

    def valid?(s) when is_binary(s), do: Regex.match?(@regex, s)
    def valid?(_), do: false

    def normalise(s) when is_binary(s), do: s |> String.trim() |> String.downcase()
  end

  defmodule Accounts do
    def register(%{email: email}) do
      normalised = EmailAddress.normalise(email)

      if EmailAddress.valid?(normalised) do
        {:ok, normalised}
      else
        {:error, :invalid_email}
      end
    end
  end

  defmodule Webhooks do
    def deliver(%{to: email} = event) do
      if EmailAddress.valid?(email) do
        {:ok, event}
      else
        {:error, :invalid_email}
      end
    end
  end
end

alias ModuleHighlander.GoodExample, as: G

{:ok, "alice@test.com"} = G.Accounts.register(%{email: "  Alice@Test.com  "})
{:error, :invalid_email} = G.Accounts.register(%{email: "bogus"})

{:ok, _} = G.Webhooks.deliver(%{to: "bob@test.com", payload: %{}})
{:error, :invalid_email} = G.Webhooks.deliver(%{to: "not-an-email", payload: %{}})

IO.puts("module-highlander: good example ok")
