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
      When they exceed #{inspect(@default_params[:max_lines])} lines, business
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

    if is_coordinator?(source) do
      line_count = source |> String.split("\n") |> length()

      if line_count > max_lines do
        [format_issue(issue_meta,
          message: "Coordinator module has #{line_count} lines (max #{max_lines}). Extract business logic to a service module.",
          trigger: source_file.filename,
          line_no: 1
        )]
      else
        []
      end
    else
      []
    end
  end

  defp is_coordinator?(source) do
    Enum.any?(@coordinator_indicators, &String.contains?(source, &1))
  end
end
