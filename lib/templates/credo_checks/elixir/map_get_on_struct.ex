defmodule Mix.Checks.MapGetOnStruct do
  @moduledoc false

  use Credo.Check,
    id: "EX4004",
    base_priority: :normal,
    category: :refactor,
    explanations: [
      check: """
      R7: Use dot access (`struct.field`) instead of `Map.get(struct, :field)`.

      Dot access on structs is clearer and raises on missing keys, catching
      typos at compile time.

          # preferred
          user.email

          # avoid
          Map.get(user, :email)
      """
    ]

  @impl Credo.Check
  def run(%SourceFile{} = source_file, params) do
    issue_meta = IssueMeta.for(source_file, params)

    source_file
    |> SourceFile.ast()
    |> find_map_get_calls(issue_meta)
  end

  defp find_map_get_calls(ast, issue_meta) do
    {_, issues} = Macro.prewalk(ast, [], &do_traverse(&1, &2, issue_meta))
    issues
  end

  defp do_traverse(
         {{:., _, [{:__aliases__, _, [:Map]}, :get]}, meta, [_map, key | _]} = ast,
         issues,
         issue_meta
       )
       when is_atom(key) do
    issue = format_issue(issue_meta,
      message: "Use dot access instead of `Map.get(struct, :#{key})`.",
      trigger: "Map.get",
      line_no: meta[:line]
    )
    {ast, [issue | issues]}
  end

  defp do_traverse(ast, issues, _issue_meta), do: {ast, issues}
end
