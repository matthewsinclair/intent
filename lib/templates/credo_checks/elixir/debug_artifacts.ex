defmodule Mix.Checks.DebugArtifacts do
  @moduledoc false

  use Credo.Check,
    id: "EX4003",
    base_priority: :high,
    category: :warning,
    param_defaults: [excluded_paths: []],
    explanations: [
      check: """
      R15: Remove debug artifacts (`IO.inspect`, `IO.puts`, `dbg()`) from `lib/`.

      Debug calls left in production code cause noisy logs and may leak
      sensitive data.

          # avoid in lib/
          IO.inspect(data, label: "debug")
          dbg(value)
          IO.puts("got here")
      """,
      params: [
        excluded_paths:
          "List of path substrings to exclude (e.g. [\"lib/my_app/cli\", \"lib/mix\"])."
      ]
    ]

  @impl Credo.Check
  def run(%SourceFile{} = source_file, params) do
    filename = source_file.filename
    excluded_paths = Params.get(params, :excluded_paths, __MODULE__)

    # Only check files under lib/
    if String.contains?(filename, "/lib/") and not excluded?(filename, excluded_paths) do
      issue_meta = IssueMeta.for(source_file, params)

      source_file
      |> SourceFile.ast()
      |> find_debug_calls(issue_meta)
    else
      []
    end
  end

  defp excluded?(_filename, []), do: false

  defp excluded?(filename, excluded_paths) do
    Enum.any?(excluded_paths, &String.contains?(filename, &1))
  end

  defp find_debug_calls(ast, issue_meta) do
    {_, issues} = Macro.prewalk(ast, [], &do_traverse(&1, &2, issue_meta))
    issues
  end

  defp do_traverse(
         {{:., _, [{:__aliases__, _, [:IO]}, func]}, meta, _args} = ast,
         issues,
         issue_meta
       )
       when func in [:inspect, :puts] do
    issue =
      format_issue(issue_meta,
        message: "Debug artifact `IO.#{func}` found in lib/.",
        trigger: "IO.#{func}",
        line_no: meta[:line]
      )

    {ast, [issue | issues]}
  end

  defp do_traverse({:dbg, meta, _args} = ast, issues, issue_meta) do
    issue =
      format_issue(issue_meta,
        message: "Debug artifact `dbg()` found in lib/.",
        trigger: "dbg",
        line_no: meta[:line]
      )

    {ast, [issue | issues]}
  end

  defp do_traverse(ast, issues, _issue_meta), do: {ast, issues}
end
