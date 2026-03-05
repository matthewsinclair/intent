defmodule Mix.Checks.DependencyGraph do
  @moduledoc false

  use Credo.Check,
    id: "EX4007",
    base_priority: :normal,
    category: :design,
    run_on_all: true,
    explanations: [
      check: """
      D11: Dependency graph enforcement for umbrella applications.

      Reads dependency rules from `intent/llm/DEPENDENCY_GRAPH.md` and checks
      that alias/import/use statements do not reference forbidden apps.

      The rules file declares which apps a given app must NOT depend on.
      Dependencies are detected by mapping alias/import/use targets to app
      names based on module prefixes.
      """
    ]

  @dep_graph_file "intent/llm/DEPENDENCY_GRAPH.md"

  @impl Credo.Check
  def run(source_files, params) when is_list(source_files) do
    rules = parse_dependency_rules()

    if rules == %{} do
      # No rules file or no rules defined
      []
    else
      source_files
      |> Enum.filter(&in_umbrella_app?/1)
      |> Enum.flat_map(&check_file(&1, rules, params))
    end
  end

  def run(%SourceFile{} = source_file, params) do
    rules = parse_dependency_rules()

    if rules == %{} do
      []
    else
      check_file(source_file, rules, params)
    end
  end

  # Parse DEPENDENCY_GRAPH.md and return %{app => [forbidden_apps]}
  defp parse_dependency_rules do
    case File.read(@dep_graph_file) do
      {:ok, content} ->
        content
        |> String.split("\n")
        |> Enum.filter(&String.contains?(&1, "|"))
        |> Enum.reject(&header_or_separator?/1)
        |> Enum.reduce(%{}, fn line, acc ->
          case parse_rule_line(line) do
            {app, forbidden} when app != "" -> Map.put(acc, app, forbidden)
            _ -> acc
          end
        end)

      {:error, _} ->
        %{}
    end
  end

  defp header_or_separator?(line) do
    stripped = String.trim(line)
    String.contains?(stripped, "---") or
      String.contains?(stripped, "App") and String.contains?(stripped, "Must NOT")
  end

  defp parse_rule_line(line) do
    parts =
      line
      |> String.split("|")
      |> Enum.map(&String.trim/1)
      |> Enum.reject(&(&1 == ""))

    case parts do
      [app, _may_depend, must_not | _] ->
        forbidden =
          must_not
          |> String.split(",")
          |> Enum.map(&String.trim/1)
          |> Enum.reject(&(&1 == ""))

        {app, forbidden}

      [app, _may_depend] ->
        {app, []}

      _ ->
        {"", []}
    end
  end

  defp in_umbrella_app?(source_file) do
    String.contains?(source_file.filename, "/apps/")
  end

  # Determine which app a file belongs to from its path
  defp app_from_file(source_file) do
    case Regex.run(~r{apps/([^/]+)/}, source_file.filename) do
      [_, app] -> app
      _ -> nil
    end
  end

  defp check_file(source_file, rules, params) do
    app = app_from_file(source_file)

    if app == nil do
      []
    else
      forbidden = Map.get(rules, app, [])

      if forbidden == [] do
        []
      else
        source_file
        |> SourceFile.ast()
        |> find_dependency_violations(source_file, app, forbidden, params)
      end
    end
  end

  defp find_dependency_violations(ast, source_file, app, forbidden, params) do
    {_, violations} = Macro.prewalk(ast, [], fn
      {directive, meta, [{:__aliases__, _, parts} | _]} = node, acc
      when directive in [:alias, :import, :use] ->
        target_app = infer_app_from_module(parts)

        if target_app != nil and target_app != app and target_app in forbidden do
          issue_meta = IssueMeta.for(source_file, params)

          module_name = parts |> Enum.map(&to_string/1) |> Enum.join(".")
          issue = format_issue(issue_meta,
            message: "Forbidden dependency: `#{app}` must not depend on `#{target_app}` (via #{directive} #{module_name}).",
            trigger: "#{module_name}",
            line_no: meta[:line]
          )

          {node, [issue | acc]}
        else
          {node, acc}
        end

      node, acc ->
        {node, acc}
    end)

    violations
  end

  # Infer app name from module parts by converting the first part to snake_case
  defp infer_app_from_module([first | _]) do
    first
    |> to_string()
    |> Macro.underscore()
  end

  defp infer_app_from_module(_), do: nil
end
