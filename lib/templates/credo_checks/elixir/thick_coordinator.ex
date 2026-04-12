defmodule Mix.Checks.ThickCoordinator do
  @moduledoc false

  use Credo.Check,
    id: "EX4005",
    base_priority: :normal,
    category: :design,
    param_defaults: [max_lines: 80],
    explanations: [
      check: """
      R2: Keep controllers and LiveViews thin.

      Coordinators (controllers, LiveViews) should delegate to service modules.
      When they exceed 80 lines, business
      logic has likely leaked in.

      Extract logic into a dedicated context or service module.
      """,
      params: [
        max_lines: "Maximum lines before triggering (default: 80)."
      ]
    ]

  @coordinator_indicators [
    "use Phoenix.Controller",
    "use Phoenix.LiveView",
    "use Phoenix.LiveComponent"
  ]

  @impl Credo.Check
  def run(%SourceFile{} = source_file, params) do
    max_lines = Params.get(params, :max_lines, __MODULE__)
    issue_meta = IssueMeta.for(source_file, params)

    source = SourceFile.source(source_file)

    if is_coordinator?(source, source_file.filename) do
      line_count = source |> String.split("\n") |> length()

      if line_count > max_lines do
        [
          format_issue(issue_meta,
            message:
              "Coordinator module has #{line_count} lines (max #{max_lines}). Extract business logic to a service module.",
            trigger: source_file.filename,
            line_no: 1
          )
        ]
      else
        []
      end
    else
      []
    end
  end

  defp is_coordinator?(source, filename) do
    # Strip content inside quote do...end blocks to avoid false positives
    # on *_web.ex entrypoint modules that delegate via macros
    check_source =
      if String.ends_with?(filename, "_web.ex") do
        strip_quote_blocks(source)
      else
        source
      end

    Enum.any?(@coordinator_indicators, &String.contains?(check_source, &1))
  end

  # Remove content between `quote do` and its matching `end` to avoid
  # flagging macro delegation as coordinator usage.
  defp strip_quote_blocks(source) do
    # Split on quote do blocks and remove their content
    # This is a heuristic -- handles the common single-level case
    source
    |> String.split(~r/quote\s+do\b/, parts: :infinity)
    |> Enum.map_join("", fn segment ->
      case String.split(segment, ~r/\bend\b/, parts: 2) do
        [_quoted, rest] -> rest
        [only] -> only
      end
    end)
  end
end
