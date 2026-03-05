defmodule Mix.Checks.BooleanOperators do
  @moduledoc false

  use Credo.Check,
    id: "EX4001",
    base_priority: :high,
    category: :readability,
    explanations: [
      check: """
      R8: Use `and`/`or` for boolean operands, `&&`/`||` for truthy/falsy operands.

      Using `&&`/`||` where `and`/`or` is correct obscures intent and risks
      unexpected truthy/falsy coercion.

          # preferred
          is_active and is_admin

          # avoid
          is_active && is_admin
      """
    ]

  @impl Credo.Check
  def run(%SourceFile{} = source_file, params) do
    issue_meta = IssueMeta.for(source_file, params)

    source_file
    |> Credo.Code.to_tokens()
    |> Enum.reduce([], fn
      {:bool_op, {line, col, _}, op}, issues when op in [:&&, :||] ->
        [issue_for(issue_meta, line, col, op) | issues]

      _token, issues ->
        issues
    end)
    |> Enum.reverse()
  end

  defp issue_for(issue_meta, line, col, op) do
    replacement = if op == :&&, do: "and", else: "or"

    format_issue(issue_meta,
      message: "Use `#{replacement}` instead of `#{op}` for boolean operands.",
      trigger: "#{op}",
      line_no: line,
      column: col
    )
  end
end
