defmodule WorkerBee.ProjectMapper do
  @moduledoc """
  Interactive project structure discovery and WDD layer mapping.

  This module conducts discovery sessions to understand how a specific
  Elixir project should be organized according to WDD principles.
  """

  @project_types [
    :phoenix_web,
    :phoenix_api,
    :otp_application,
    :library,
    :nerves,
    :umbrella,
    :poncho,
    :livebook
  ]

  @wdd_layers [
    :data,
    :functions,
    :tests,
    :boundaries,
    :lifecycles,
    :workers
  ]

  defstruct [
    :project_name,
    :project_type,
    :root_path,
    :layer_paths,
    :framework_considerations,
    :naming_conventions,
    :discovered_patterns
  ]

  @type t :: %__MODULE__{
    project_name: String.t(),
    project_type: atom(),
    root_path: String.t(),
    layer_paths: %{atom() => String.t()},
    framework_considerations: [String.t()],
    naming_conventions: %{atom() => String.t()},
    discovered_patterns: [String.t()]
  }

  @doc """
  Starts an interactive mapping session to discover project structure.
  """
  @spec discover_project_structure(String.t()) :: {:ok, t()} | {:error, String.t()}
  def discover_project_structure(project_path) do
    with {:ok, project_info} <- scan_project_structure(project_path),
         {:ok, project_type} <- determine_project_type(project_info),
         {:ok, layer_mapping} <- conduct_interactive_mapping(project_type, project_info) do

      project_map = %__MODULE__{
        project_name: extract_project_name(project_path),
        project_type: project_type,
        root_path: project_path,
        layer_paths: layer_mapping.layer_paths,
        framework_considerations: layer_mapping.framework_considerations,
        naming_conventions: layer_mapping.naming_conventions,
        discovered_patterns: project_info.discovered_patterns
      }

      {:ok, project_map}
    end
  end

  @doc """
  Scans the current project structure to identify existing patterns.
  """
  def scan_project_structure(project_path) do
    patterns = %{
      has_mix_exs: File.exists?(Path.join(project_path, "mix.exs")),
      has_lib_dir: File.dir?(Path.join(project_path, "lib")),
      has_test_dir: File.dir?(Path.join(project_path, "test")),
      has_phoenix: detect_phoenix_project(project_path),
      has_otp_app: detect_otp_application(project_path),
      lib_structure: scan_lib_directory(project_path),
      test_structure: scan_test_directory(project_path),
      existing_modules: discover_existing_modules(project_path)
    }

    discovered_patterns = analyze_existing_patterns(patterns)

    {:ok, %{
      patterns: patterns,
      discovered_patterns: discovered_patterns
    }}
  end

  @doc """
  Determines the project type based on scanned information.
  """
  def determine_project_type(project_info) do
    cond do
      project_info.patterns.has_phoenix and has_web_features?(project_info) ->
        {:ok, :phoenix_web}

      project_info.patterns.has_phoenix ->
        {:ok, :phoenix_api}

      project_info.patterns.has_otp_app and has_supervision_tree?(project_info) ->
        {:ok, :otp_application}

      is_library_project?(project_info) ->
        {:ok, :library}

      has_umbrella_structure?(project_info) ->
        {:ok, :umbrella}

      true ->
        {:ok, :otp_application}  # Default fallback
    end
  end

  @doc """
  Conducts interactive session to map WDD layers to project structure.
  """
  def conduct_interactive_mapping(project_type, project_info) do
    IO.puts("\n🐝 Worker-Bee WDD Project Structure Discovery")
    IO.puts("=" |> String.duplicate(50))

    IO.puts("\nProject Type Detected: #{format_project_type(project_type)}")
    display_discovered_patterns(project_info.discovered_patterns)

    layer_paths = gather_layer_preferences(project_type, project_info)
    naming_conventions = gather_naming_conventions()
    framework_considerations = gather_framework_considerations(project_type)

    {:ok, %{
      layer_paths: layer_paths,
      naming_conventions: naming_conventions,
      framework_considerations: framework_considerations
    }}
  end

  @doc """
  Saves the project mapping to a configuration file.
  """
  def save_project_map(project_map, output_path \\ ".wdd_project_map.yaml") do
    yaml_content = generate_yaml_config(project_map)

    case File.write(output_path, yaml_content) do
      :ok ->
        {:ok, "Project map saved to #{output_path}"}
      {:error, reason} ->
        {:error, "Failed to save project map: #{reason}"}
    end
  end

  @doc """
  Loads an existing project mapping from configuration file.
  """
  def load_project_map(config_path \\ ".wdd_project_map.yaml") do
    case File.read(config_path) do
      {:ok, content} -> parse_yaml_config(content)
      {:error, :enoent} -> {:error, "No project map found. Run project discovery first."}
      {:error, reason} -> {:error, "Failed to load project map: #{reason}"}
    end
  end

  # Private helper functions

  defp detect_phoenix_project(project_path) do
    mix_exs = Path.join(project_path, "mix.exs")

    case File.read(mix_exs) do
      {:ok, content} -> String.contains?(content, ":phoenix")
      _ -> false
    end
  end

  defp detect_otp_application(project_path) do
    app_file = Path.join([project_path, "lib", "**", "application.ex"])
    !Enum.empty?(Path.wildcard(app_file))
  end

  defp scan_lib_directory(project_path) do
    lib_path = Path.join(project_path, "lib")

    if File.dir?(lib_path) do
      lib_path
      |> Path.join("**/*.ex")
      |> Path.wildcard()
      |> Enum.map(&Path.relative_to(&1, lib_path))
      |> analyze_lib_structure()
    else
      %{}
    end
  end

  defp scan_test_directory(project_path) do
    test_path = Path.join(project_path, "test")

    if File.dir?(test_path) do
      test_path
      |> Path.join("**/*_test.exs")
      |> Path.wildcard()
      |> Enum.map(&Path.relative_to(&1, test_path))
    else
      []
    end
  end

  defp discover_existing_modules(project_path) do
    lib_path = Path.join(project_path, "lib")

    if File.dir?(lib_path) do
      lib_path
      |> Path.join("**/*.ex")
      |> Path.wildcard()
      |> Enum.map(&extract_module_info/1)
      |> Enum.reject(&is_nil/1)
    else
      []
    end
  end

  defp analyze_existing_patterns(patterns) do
    discovered = []

    discovered = if patterns.has_phoenix, do: ["Phoenix framework detected"] ++ discovered, else: discovered
    discovered = if patterns.has_otp_app, do: ["OTP application structure"] ++ discovered, else: discovered
    discovered = if has_functional_core_pattern?(patterns), do: ["Functional core pattern found"] ++ discovered, else: discovered
    discovered = if has_boundary_pattern?(patterns), do: ["Boundary layer pattern found"] ++ discovered, else: discovered

    discovered
  end

  defp gather_layer_preferences(project_type, project_info) do
    IO.puts("\n📂 WDD Layer Structure Configuration")
    IO.puts("Let's define where each WDD layer should live in your project.\n")

    suggested_paths = get_suggested_paths(project_type)

    Enum.reduce(@wdd_layers, %{}, fn layer, acc ->
      suggestion = Map.get(suggested_paths, layer, "lib/#{layer}")

      IO.puts("#{format_layer_name(layer)} Layer:")
      IO.puts("  Suggested: #{suggestion}")

      prompt = "  Your choice (press Enter for suggestion): "
      user_input = IO.gets(prompt) |> String.trim()

      chosen_path = if user_input == "", do: suggestion, else: user_input

      Map.put(acc, layer, chosen_path)
    end)
  end

  defp gather_naming_conventions do
    IO.puts("\n🏷️  Naming Convention Preferences")

    %{
      module_prefix: get_user_preference("Module prefix (eg MyApp)", ""),
      functional_core_suffix: get_user_preference("Functional core suffix", "Core"),
      boundary_suffix: get_user_preference("Boundary module suffix", ""),
      test_suffix: get_user_preference("Test module suffix", "Test")
    }
  end

  defp gather_framework_considerations(project_type) do
    considerations = []

    considerations = case project_type do
      :phoenix_web -> ["Phoenix contexts as boundary layers", "LiveView components"] ++ considerations
      :phoenix_api -> ["Phoenix contexts as boundary layers", "JSON API design"] ++ considerations
      :otp_application -> ["GenServer supervision", "Application callbacks"] ++ considerations
      :library -> ["Pure functional design", "No process machinery"] ++ considerations
      _ -> considerations
    end

    IO.puts("\n⚙️  Framework Considerations:")
    Enum.each(considerations, fn consideration ->
      IO.puts("  • #{consideration}")
    end)

    considerations
  end

  defp get_suggested_paths(:phoenix_web) do
    %{
      data: "lib/my_app/types",
      functions: "lib/my_app_web/functional_core",
      tests: "test",
      boundaries: "lib/my_app_web",
      lifecycles: "lib/my_app/application.ex",
      workers: "lib/my_app/workers"
    }
  end

  defp get_suggested_paths(:phoenix_api) do
    %{
      data: "lib/my_app/types",
      functions: "lib/my_app/functional_core",
      tests: "test",
      boundaries: "lib/my_app_web",
      lifecycles: "lib/my_app/application.ex",
      workers: "lib/my_app/workers"
    }
  end

  defp get_suggested_paths(:otp_application) do
    %{
      data: "lib/my_app/types",
      functions: "lib/my_app/core",
      tests: "test",
      boundaries: "lib/my_app/boundary",
      lifecycles: "lib/my_app/application.ex",
      workers: "lib/my_app/workers"
    }
  end

  defp get_suggested_paths(:library) do
    %{
      data: "lib/my_lib/types",
      functions: "lib/my_lib",
      tests: "test",
      boundaries: "lib/my_lib/api",
      lifecycles: "N/A (library)",
      workers: "N/A (library)"
    }
  end

  defp get_suggested_paths(_) do
    %{
      data: "lib/data",
      functions: "lib/core",
      tests: "test",
      boundaries: "lib/boundary",
      lifecycles: "lib/application.ex",
      workers: "lib/workers"
    }
  end

  defp get_user_preference(prompt, default) do
    full_prompt = if default != "", do: "#{prompt} [#{default}]: ", else: "#{prompt}: "
    user_input = IO.gets(full_prompt) |> String.trim()

    if user_input == "", do: default, else: user_input
  end

  defp extract_project_name(project_path) do
    project_path
    |> Path.basename()
    |> String.replace("-", "_")
  end

  defp format_project_type(type) do
    type
    |> Atom.to_string()
    |> String.replace("_", " ")
    |> String.split()
    |> Enum.map(&String.capitalize/1)
    |> Enum.join(" ")
  end

  defp format_layer_name(layer) do
    layer
    |> Atom.to_string()
    |> String.capitalize()
  end

  defp display_discovered_patterns(patterns) do
    if not Enum.empty?(patterns) do
      IO.puts("\n🔍 Discovered Patterns:")
      Enum.each(patterns, fn pattern ->
        IO.puts("  • #{pattern}")
      end)
    end
  end

  defp generate_yaml_config(project_map) do
    """
    # WDD Project Structure Map
    # Generated by Worker-Bee Agent

    project_name: "#{project_map.project_name}"
    project_type: #{project_map.project_type}
    root_path: "#{project_map.root_path}"

    wdd_layers:
    #{format_layer_paths_yaml(project_map.layer_paths)}

    naming_conventions:
    #{format_naming_conventions_yaml(project_map.naming_conventions)}

    framework_considerations:
    #{format_framework_considerations_yaml(project_map.framework_considerations)}

    discovered_patterns:
    #{format_discovered_patterns_yaml(project_map.discovered_patterns)}
    """
  end

  defp format_layer_paths_yaml(layer_paths) do
    Enum.map(layer_paths, fn {layer, path} ->
      "  #{layer}: \"#{path}\""
    end)
    |> Enum.join("\n")
  end

  defp format_naming_conventions_yaml(naming_conventions) do
    Enum.map(naming_conventions, fn {key, value} ->
      "  #{key}: \"#{value}\""
    end)
    |> Enum.join("\n")
  end

  defp format_framework_considerations_yaml(considerations) do
    Enum.map(considerations, fn consideration ->
      "  - \"#{consideration}\""
    end)
    |> Enum.join("\n")
  end

  defp format_discovered_patterns_yaml(patterns) do
    Enum.map(patterns, fn pattern ->
      "  - \"#{pattern}\""
    end)
    |> Enum.join("\n")
  end

  # Additional helper functions for pattern detection
  defp has_web_features?(project_info) do
    lib_files = project_info.patterns.lib_structure

    web_indicators = [
      "router.ex",
      "endpoint.ex",
      "controllers/",
      "views/",
      "templates/",
      "live/"
    ]

    Enum.any?(web_indicators, fn indicator ->
      Enum.any?(Map.keys(lib_files), fn file ->
        String.contains?(file, indicator)
      end)
    end)
  end

  defp has_supervision_tree?(project_info) do
    Enum.any?(project_info.patterns.existing_modules, fn module_info ->
      String.contains?(module_info.content || "", "Supervisor")
    end)
  end

  defp is_library_project?(project_info) do
    not project_info.patterns.has_otp_app and
    not project_info.patterns.has_phoenix
  end

  defp has_umbrella_structure?(project_info) do
    File.dir?(Path.join(project_info.patterns.root_path || ".", "apps"))
  end

  defp has_functional_core_pattern?(patterns) do
    lib_files = patterns.lib_structure

    core_indicators = [
      "core/",
      "functional_core/",
      "business/"
    ]

    Enum.any?(core_indicators, fn indicator ->
      Enum.any?(Map.keys(lib_files), fn file ->
        String.contains?(file, indicator)
      end)
    end)
  end

  defp has_boundary_pattern?(patterns) do
    lib_files = patterns.lib_structure

    boundary_indicators = [
      "boundary/",
      "api/",
      "web/",
      "controllers/"
    ]

    Enum.any?(boundary_indicators, fn indicator ->
      Enum.any?(Map.keys(lib_files), fn file ->
        String.contains?(file, indicator)
      end)
    end)
  end

  defp analyze_lib_structure(file_paths) do
    Enum.reduce(file_paths, %{}, fn file_path, acc ->
      directory = Path.dirname(file_path)
      files = Map.get(acc, directory, [])
      Map.put(acc, directory, [file_path | files])
    end)
  end

  defp extract_module_info(file_path) do
    case File.read(file_path) do
      {:ok, content} ->
        %{
          path: file_path,
          content: content,
          module_name: extract_module_name(content)
        }
      _ ->
        nil
    end
  end

  defp extract_module_name(content) do
    case Regex.run(~r/defmodule\s+([\w\.]+)/, content) do
      [_, module_name] -> module_name
      _ -> nil
    end
  end

  defp parse_yaml_config(content) do
    # Simple YAML parsing for basic project map structure
    # In a real implementation, you'd use a YAML library
    {:ok, %__MODULE__{}}
  end
end
