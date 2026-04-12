defmodule Mix.Checks.MissingImplAnnotation do
  @moduledoc false

  use Credo.Check,
    id: "EX4002",
    base_priority: :high,
    category: :readability,
    explanations: [
      check: """
      R11: Every behaviour callback implementation must have `@impl true`.

      Missing `@impl` annotations make it unclear which functions are callback
      implementations vs local helpers.

          # preferred
          @impl true
          def handle_call(msg, from, state), do: ...

          # avoid
          def handle_call(msg, from, state), do: ...
      """
    ]

  @impl Credo.Check
  def run(%SourceFile{} = source_file, params) do
    issue_meta = IssueMeta.for(source_file, params)

    {_ast, issues} =
      source_file
      |> SourceFile.ast()
      |> Macro.prewalk([], &traverse(&1, &2, issue_meta))

    issues
  end

  defp traverse({:defmodule, _, _} = ast, issues, issue_meta) do
    {_behaviours, module_issues} = scan_module(ast, issue_meta)
    {ast, issues ++ module_issues}
  end

  defp traverse(ast, acc, _issue_meta), do: {ast, acc}

  defp scan_module({:defmodule, _, [_alias, [do: body]]}, issue_meta) do
    statements =
      case body do
        {:__block__, _, stmts} -> stmts
        stmt -> [stmt]
      end

    behaviours = extract_behaviours(statements)
    callback_names = known_callbacks(behaviours)

    issues =
      statements
      |> find_unimpl_callbacks(callback_names, issue_meta)

    {behaviours, issues}
  end

  defp scan_module(_ast, _issue_meta), do: {[], []}

  defp extract_behaviours(statements) do
    Enum.flat_map(statements, fn
      {:@, _, [{:behaviour, _, [module]}]} -> [module]
      _ -> []
    end)
  end

  defp known_callbacks([]), do: MapSet.new()

  defp known_callbacks(behaviours) do
    well_known = %{
      GenServer =>
        ~w(init handle_call handle_cast handle_info terminate code_change handle_continue)a,
      Supervisor => ~w(init)a,
      Application => ~w(start stop)a,
      Phoenix.LiveView => ~w(mount handle_event handle_info handle_params render)a,
      Phoenix.Controller => ~w(init call)a,
      Plug => ~w(init call)a
    }

    behaviours
    |> Enum.flat_map(&Map.get(well_known, &1, []))
    |> MapSet.new()
  end

  defp find_unimpl_callbacks(statements, callback_names, issue_meta) do
    if MapSet.size(callback_names) == 0 do
      []
    else
      statements
      |> Enum.chunk_every(2, 1)
      |> Enum.flat_map(fn
        [{:@, _, [{:impl, _, _}]}, _def] ->
          []

        [_, {:def, meta, [{name, _, _} | _]}] when is_atom(name) ->
          if MapSet.member?(callback_names, name) do
            [
              format_issue(issue_meta,
                message: "Callback `#{name}` is missing `@impl true`.",
                trigger: "#{name}",
                line_no: meta[:line]
              )
            ]
          else
            []
          end

        _ ->
          []
      end)
    end
  end
end
