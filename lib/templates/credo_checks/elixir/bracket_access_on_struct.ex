defmodule Mix.Checks.BracketAccessOnStruct do
  @moduledoc false

  use Credo.Check,
    id: "EX4008",
    base_priority: :high,
    category: :warning,
    explanations: [
      check: """
      R16: Do not use bracket access on structs.

      Structs do not implement the `Access` behaviour. Using `struct[:field]`
      will crash at runtime with a `UndefinedFunctionError`.

          # preferred
          user.name

          # avoid -- crashes at runtime
          user[:name]
      """
    ]

  @impl Credo.Check
  def run(%SourceFile{} = source_file, params) do
    issue_meta = IssueMeta.for(source_file, params)

    source_file
    |> SourceFile.ast()
    |> find_bracket_access_on_structs(issue_meta)
  end

  defp find_bracket_access_on_structs(ast, issue_meta) do
    struct_vars = collect_struct_vars(ast)

    if MapSet.size(struct_vars) == 0 do
      []
    else
      {_, issues} = Macro.prewalk(ast, [], &check_access(&1, &2, struct_vars, issue_meta))
      issues
    end
  end

  # Collect variables that are bound to struct literals.
  # Matches: %Module{} = var, var = %Module{}, and function params like fn(%Module{} = var)
  defp collect_struct_vars(ast) do
    {_, vars} =
      Macro.prewalk(ast, MapSet.new(), fn
        # %Module{} = var
        {:=, _, [{:%, _, _}, {var_name, _, nil}]} = node, acc when is_atom(var_name) ->
          {node, MapSet.put(acc, var_name)}

        # var = %Module{}
        {:=, _, [{var_name, _, nil}, {:%, _, _}]} = node, acc when is_atom(var_name) ->
          {node, MapSet.put(acc, var_name)}

        # Function head pattern match: fn(%Module{} = var) or def foo(%Module{} = var)
        {:=, _, [{:%, _, _}, {var_name, _, ctx}]} = node, acc
        when is_atom(var_name) and is_atom(ctx) ->
          {node, MapSet.put(acc, var_name)}

        {:=, _, [{var_name, _, ctx}, {:%, _, _}]} = node, acc
        when is_atom(var_name) and is_atom(ctx) ->
          {node, MapSet.put(acc, var_name)}

        node, acc ->
          {node, acc}
      end)

    vars
  end

  # Bracket access compiles to Access.get(var, key)
  defp check_access(
         {{:., _, [Access, :get]}, meta, [{var_name, _, nil}, _key]} = ast,
         issues,
         struct_vars,
         issue_meta
       )
       when is_atom(var_name) do
    if MapSet.member?(struct_vars, var_name) do
      issue =
        format_issue(issue_meta,
          message:
            "Bracket access `#{var_name}[:key]` on a struct variable. Use dot access instead.",
          trigger: "#{var_name}",
          line_no: meta[:line]
        )

      {ast, [issue | issues]}
    else
      {ast, issues}
    end
  end

  defp check_access(ast, issues, _struct_vars, _issue_meta), do: {ast, issues}
end
