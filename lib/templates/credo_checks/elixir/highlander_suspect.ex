defmodule Mix.Checks.HighlanderSuspect do
  @moduledoc false

  use Credo.Check,
    id: "EX4006",
    base_priority: :normal,
    category: :design,
    run_on_all: true,
    explanations: [
      check: """
      R6: The Highlander Rule -- there can be only one.

      Flags public functions with the same name defined in multiple modules.
      Duplicate function names across modules often indicate duplicated
      responsibilities. Review and consolidate.

      This is a heuristic check. Common names like `new`, `get`, `list`,
      `create`, `update`, `delete`, `changeset`, `render` are excluded.
      """
    ]

  @common_names MapSet.new(~w(
    new get list create update delete changeset render
    init call handle_call handle_cast handle_info handle_event
    mount handle_params terminate code_change start_link child_spec
  )a)

  @impl Credo.Check
  def run(source_files, params) when is_list(source_files) do
    # Collect all public function definitions across files
    all_defs =
      source_files
      |> Enum.filter(&String.contains?(&1.filename, "/lib/"))
      |> Enum.flat_map(&extract_public_defs/1)

    # Group by function name, find duplicates
    duplicates =
      all_defs
      |> Enum.group_by(fn {name, arity, _module, _file, _line} -> {name, arity} end)
      |> Enum.filter(fn {_key, entries} -> length(entries) > 1 end)
      |> Enum.reject(fn {{name, _arity}, _entries} -> MapSet.member?(@common_names, name) end)

    # Generate issues for each duplicate group
    Enum.flat_map(duplicates, fn {{name, _arity}, entries} ->
      modules = entries |> Enum.map(fn {_, _, mod, _, _} -> mod end) |> Enum.join(", ")

      Enum.map(entries, fn {_name, arity, _module, source_file, line} ->
        issue_meta = IssueMeta.for(source_file, params)

        format_issue(issue_meta,
          message: "Highlander suspect: `#{name}/#{arity}` also defined in #{modules}.",
          trigger: "#{name}",
          line_no: line
        )
      end)
    end)
  end

  def run(%SourceFile{} = source_file, params) do
    # Single-file mode: cannot detect cross-module duplicates
    _ = {source_file, params}
    []
  end

  defp extract_public_defs(source_file) do
    module_name = extract_module_name(source_file)

    source_file
    |> SourceFile.ast()
    |> do_extract_defs(module_name, source_file)
  end

  defp do_extract_defs(ast, module_name, source_file) do
    {_, defs} = Macro.prewalk(ast, [], fn
      {:def, meta, [{name, _, args} | _]} = node, acc when is_atom(name) ->
        arity = if is_list(args), do: length(args), else: 0
        {node, [{name, arity, module_name, source_file, meta[:line]} | acc]}

      node, acc ->
        {node, acc}
    end)

    defs |> Enum.uniq_by(fn {name, arity, _, _, _} -> {name, arity} end)
  end

  defp extract_module_name(source_file) do
    source_file
    |> SourceFile.ast()
    |> do_extract_module_name()
  end

  defp do_extract_module_name({:defmodule, _, [{:__aliases__, _, parts} | _]}) do
    parts |> Enum.map(&to_string/1) |> Enum.join(".")
  end

  defp do_extract_module_name({:__block__, _, children}) do
    Enum.find_value(children, "Unknown", &do_extract_module_name/1)
  end

  defp do_extract_module_name(_), do: "Unknown"
end
