defmodule WorkerBee.WddValidator do
  @moduledoc """
  WDD compliance validation engine.
  
  Validates Elixir code against Worker-Bee Driven Design principles
  based on the project's established WDD layer mapping.
  """

  alias WorkerBee.ProjectMapper

  defstruct [
    :project_map,
    :validation_results,
    :compliance_score,
    :violations,
    :recommendations
  ]

  @type validation_result :: %{
    file_path: String.t(),
    layer: atom(),
    violations: [violation()],
    score: float(),
    recommendations: [String.t()]
  }

  @type violation :: %{
    type: atom(),
    severity: :error | :warning | :info,
    line: integer() | nil,
    message: String.t(),
    rule: String.t()
  }

  @type t :: %__MODULE__{
    project_map: ProjectMapper.t(),
    validation_results: [validation_result()],
    compliance_score: float(),
    violations: [violation()],
    recommendations: [String.t()]
  }

  @doc """
  Validates the entire project against WDD compliance.
  """
  @spec validate_project(String.t()) :: {:ok, t()} | {:error, String.t()}
  def validate_project(project_path) do
    with {:ok, project_map} <- ProjectMapper.load_project_map(Path.join(project_path, ".wdd_project_map.yaml")),
         {:ok, file_list} <- get_project_files(project_path),
         validation_results <- validate_files(file_list, project_map),
         structure_analysis <- analyze_project_structure_changes(file_list, project_map) do
      
      all_recommendations = generate_recommendations(validation_results) ++ structure_analysis.recommendations
      
      validator_result = %__MODULE__{
        project_map: project_map,
        validation_results: validation_results,
        compliance_score: calculate_compliance_score(validation_results),
        violations: extract_violations(validation_results) ++ structure_analysis.violations,
        recommendations: all_recommendations
      }
      
      {:ok, validator_result}
    end
  end

  @doc """
  Validates a single file against WDD principles.
  """
  @spec validate_file(String.t(), ProjectMapper.t()) :: validation_result()
  def validate_file(file_path, project_map) do
    layer = determine_file_layer(file_path, project_map)
    
    case File.read(file_path) do
      {:ok, content} ->
        violations = validate_content(content, layer, file_path)
        
        %{
          file_path: file_path,
          layer: layer,
          violations: violations,
          score: calculate_file_score(violations),
          recommendations: generate_file_recommendations(violations, layer)
        }
      
      {:error, reason} ->
        %{
          file_path: file_path,
          layer: :unknown,
          violations: [%{
            type: :file_read_error,
            severity: :error,
            line: nil,
            message: "Cannot read file: #{reason}",
            rule: "file_accessibility"
          }],
          score: 0.0,
          recommendations: ["Ensure file is accessible and readable"]
        }
    end
  end

  @doc """
  Validates content against functional core principles.
  """
  def validate_functional_core(content, file_path) do
    violations = []
    
    violations = violations ++ check_side_effects(content, file_path)
    violations = violations ++ check_function_purity(content, file_path)
    violations = violations ++ check_composition_patterns(content, file_path)
    violations = violations ++ check_abstraction_levels(content, file_path)
    violations = violations ++ check_pattern_matching_usage(content, file_path)
    
    violations
  end

  @doc """
  Validates content against boundary layer principles.
  """
  def validate_boundary_layer(content, file_path) do
    violations = []
    
    violations = violations ++ check_genserver_patterns(content, file_path)
    violations = violations ++ check_error_handling(content, file_path)
    violations = violations ++ check_api_design(content, file_path)
    violations = violations ++ check_state_management(content, file_path)
    violations = violations ++ check_validation_placement(content, file_path)
    
    violations
  end

  @doc """
  Validates content against data layer principles.
  """
  def validate_data_layer(content, file_path) do
    violations = []
    
    violations = violations ++ check_struct_definitions(content, file_path)
    violations = violations ++ check_data_immutability(content, file_path)
    violations = violations ++ check_data_structure_choice(content, file_path)
    violations = violations ++ check_access_patterns(content, file_path)
    
    violations
  end

  @doc """
  Validates content against testing principles.
  """
  def validate_test_layer(content, file_path) do
    violations = []
    
    violations = violations ++ check_test_organization(content, file_path)
    violations = violations ++ check_test_behavior_focus(content, file_path)
    violations = violations ++ check_test_naming(content, file_path)
    violations = violations ++ check_setup_patterns(content, file_path)
    violations = violations ++ check_assertion_quality(content, file_path)
    
    violations
  end

  @doc """
  Analyzes project structure to detect when re-mapping might be needed.
  """
  def analyze_project_structure_changes(file_list, project_map) do
    violations = []
    recommendations = []
    
    # Find files outside mapped directories
    unmapped_files = find_unmapped_files(file_list, project_map)
    
    # Check for new directories that could be WDD layers
    new_directories = find_potential_new_layer_directories(file_list, project_map)
    
    # Check if project type indicators have changed
    type_changes = detect_project_type_changes(file_list, project_map)
    
    cond do
      length(unmapped_files) > 5 ->
        recommendations = [
          "Found #{length(unmapped_files)} files outside mapped WDD layers. Consider running 'mix wdd.remap' to update your project structure mapping."
        ] ++ recommendations
        
        violations = [
          create_structure_violation(:unmapped_files, :info, 
            "Multiple files found outside WDD layer mapping", 
            "project_structure_drift")
        ] ++ violations
      
      length(unmapped_files) > 0 ->
        recommendations = [
          "Found #{length(unmapped_files)} files outside mapped directories. You may want to update your WDD layer mapping."
        ] ++ recommendations
      
      length(new_directories) > 0 ->
        new_dir_names = Enum.map(new_directories, &Path.basename/1) |> Enum.join(", ")
        recommendations = [
          "Detected new directories (#{new_dir_names}) that could be WDD layers. Consider re-mapping if your project structure has evolved."
        ] ++ recommendations
      
      type_changes.has_changes? ->
        recommendations = [
          "Project type indicators suggest structural changes (#{type_changes.change_description}). Consider re-mapping your WDD layers."
        ] ++ recommendations
      
      true ->
        recommendations
    end
    
    %{
      violations: violations,
      recommendations: recommendations,
      unmapped_files: unmapped_files,
      new_directories: new_directories,
      type_changes: type_changes
    }
  end

  # Private helper functions

  defp get_project_files(project_path) do
    elixir_files = 
      project_path
      |> Path.join("**/*.{ex,exs}")
      |> Path.wildcard()
      |> Enum.reject(&String.contains?(&1, "/_build/"))
      |> Enum.reject(&String.contains?(&1, "/deps/"))
    
    {:ok, elixir_files}
  end

  defp validate_files(file_list, project_map) do
    Enum.map(file_list, fn file_path ->
      validate_file(file_path, project_map)
    end)
  end

  defp determine_file_layer(file_path, project_map) do
    layer_paths = project_map.layer_paths
    
    cond do
      path_matches?(file_path, Map.get(layer_paths, :data)) -> :data
      path_matches?(file_path, Map.get(layer_paths, :functions)) -> :functions
      path_matches?(file_path, Map.get(layer_paths, :boundaries)) -> :boundaries
      path_matches?(file_path, Map.get(layer_paths, :workers)) -> :workers
      path_matches?(file_path, Map.get(layer_paths, :lifecycles)) -> :lifecycles
      String.contains?(file_path, "test/") -> :tests
      true -> :unknown
    end
  end

  defp path_matches?(file_path, layer_path) when is_binary(layer_path) do
    String.contains?(file_path, layer_path)
  end
  
  defp path_matches?(_, _), do: false

  defp validate_content(content, layer, file_path) do
    case layer do
      :functions -> validate_functional_core(content, file_path)
      :boundaries -> validate_boundary_layer(content, file_path)
      :data -> validate_data_layer(content, file_path)
      :tests -> validate_test_layer(content, file_path)
      :workers -> validate_boundary_layer(content, file_path)  # Workers use boundary patterns
      :lifecycles -> validate_lifecycle_layer(content, file_path)
      _ -> []
    end
  end

  defp validate_lifecycle_layer(content, file_path) do
    violations = []
    
    violations = violations ++ check_supervision_patterns(content, file_path)
    violations = violations ++ check_application_structure(content, file_path)
    violations = violations ++ check_child_specs(content, file_path)
    
    violations
  end

  # Functional Core Validation Rules

  defp check_side_effects(content, file_path) do
    violations = []
    
    # Check for direct GenServer calls in functional core
    if Regex.match?(~r/GenServer\.(call|cast|start|start_link)/, content) do
      violations = [create_violation(:side_effect, :error, nil, 
        "Functional core should not contain GenServer calls", 
        "functional_core_purity", file_path) | violations]
    end
    
    # Check for direct process spawning
    if Regex.match?(~r/spawn(_link)?/, content) do
      violations = [create_violation(:side_effect, :error, nil,
        "Functional core should not spawn processes",
        "functional_core_purity", file_path) | violations]
    end
    
    # Check for file I/O operations
    if Regex.match?(~r/File\.(read|write|open)/, content) do
      violations = [create_violation(:side_effect, :error, nil,
        "Functional core should not perform file I/O",
        "functional_core_purity", file_path) | violations]
    end
    
    # Check for logging
    if Regex.match?(~r/Logger\.(info|debug|warn|error)/, content) do
      violations = [create_violation(:side_effect, :warning, nil,
        "Consider moving logging to boundary layer",
        "functional_core_purity", file_path) | violations]
    end
    
    violations
  end

  defp check_function_purity(content, file_path) do
    violations = []
    
    # Check for functions that don't return anything (side effect only)
    if Regex.match?(~r/def \w+.*do\s*\n.*\n\s*end\s*$/m, content) and
       not Regex.match?(~r/def \w+.*do\s*\n.*return|{:|\.|\w+\s*\n\s*end/m, content) do
      violations = [create_violation(:impure_function, :warning, nil,
        "Functions should return values rather than just performing side effects",
        "function_purity", file_path) | violations]
    end
    
    violations
  end

  defp check_composition_patterns(content, file_path) do
    violations = []
    
    # Check for proper pipe usage
    lines = String.split(content, "\n")
    Enum.with_index(lines)
    |> Enum.each(fn {line, index} ->
      if String.contains?(line, "|>") and String.contains?(line, "(") do
        if not Regex.match?(~r/\|>\s*\w+\(/, line) do
          violations = [create_violation(:poor_composition, :info, index + 1,
            "Consider using pipe-friendly function design",
            "composition_patterns", file_path) | violations]
        end
      end
    end)
    
    violations
  end

  defp check_abstraction_levels(content, file_path) do
    violations = []
    
    # Check for mixed abstraction levels in functions
    # This is a simplified check - real implementation would be more sophisticated
    functions = extract_functions(content)
    
    Enum.each(functions, fn {function_name, function_content, line_num} ->
      if has_mixed_abstraction_levels?(function_content) do
        violations = [create_violation(:mixed_abstraction, :warning, line_num,
          "Function '#{function_name}' mixes different abstraction levels",
          "single_abstraction_level", file_path) | violations]
      end
    end)
    
    violations
  end

  defp check_pattern_matching_usage(content, file_path) do
    violations = []
    
    # Check for if/else when pattern matching could be used
    if Regex.match?(~r/if\s+.*\s+do.*else.*end/s, content) and
       not Regex.match?(~r/case\s+.*\s+do/, content) do
      violations = [create_violation(:poor_pattern_matching, :info, nil,
        "Consider using pattern matching instead of if/else",
        "pattern_matching_preference", file_path) | violations]
    end
    
    violations
  end

  # Boundary Layer Validation Rules

  defp check_genserver_patterns(content, file_path) do
    violations = []
    
    if String.contains?(content, "use GenServer") do
      # Check for proper GenServer structure
      if not Regex.match?(~r/def handle_call/, content) and 
         not Regex.match?(~r/def handle_cast/, content) do
        violations = [create_violation(:incomplete_genserver, :warning, nil,
          "GenServer should implement handle_call or handle_cast",
          "genserver_completeness", file_path) | violations]
      end
      
      # Check for proper init function
      if not Regex.match?(~r/def init/, content) do
        violations = [create_violation(:missing_init, :error, nil,
          "GenServer must implement init/1 function",
          "genserver_structure", file_path) | violations]
      end
    end
    
    violations
  end

  defp check_error_handling(content, file_path) do
    violations = []
    
    # Check for with statements in boundary layer
    if not Regex.match?(~r/with\s+.*<-/, content) and
       Regex.match?(~r/def \w+.*do/, content) do
      violations = [create_violation(:missing_error_handling, :info, nil,
        "Consider using 'with' statements for error composition",
        "railway_oriented_programming", file_path) | violations]
    end
    
    # Check for proper tagged tuple returns
    if Regex.match?(~r/def \w+.*do/, content) and
       not Regex.match?(~r/\{:ok,|{:error,/, content) do
      violations = [create_violation(:untagged_returns, :warning, nil,
        "Consider returning tagged tuples {:ok, result} or {:error, reason}",
        "tagged_tuple_returns", file_path) | violations]
    end
    
    violations
  end

  defp check_api_design(content, file_path) do
    violations = []
    
    # Check for public functions without @spec
    functions = extract_public_functions(content)
    
    Enum.each(functions, fn {function_name, _, line_num} ->
      if not has_spec_for_function?(content, function_name) do
        violations = [create_violation(:missing_spec, :info, line_num,
          "Public function '#{function_name}' should have @spec",
          "api_documentation", file_path) | violations]
      end
    end)
    
    violations
  end

  defp check_state_management(content, file_path) do
    violations = []
    
    # Check for state mutations in functional core calls
    if String.contains?(content, "use GenServer") and
       Regex.match?(~r/def handle_.*\(.*state.*\)/s, content) do
      if not Regex.match?(~r/\{:reply,.*new_state\}|\{:noreply,.*new_state\}/s, content) do
        violations = [create_violation(:improper_state_management, :warning, nil,
          "GenServer callbacks should return proper state transitions",
          "state_management", file_path) | violations]
      end
    end
    
    violations
  end

  defp check_validation_placement(content, file_path) do
    violations = []
    
    # This would check if validations are properly placed at boundary
    # Implementation would be project-specific
    
    violations
  end

  # Data Layer Validation Rules

  defp check_struct_definitions(content, file_path) do
    violations = []
    
    if Regex.match?(~r/defstruct/, content) do
      # Check for default values
      if not Regex.match?(~r/defstruct.*:.*,/, content) do
        violations = [create_violation(:struct_without_defaults, :info, nil,
          "Consider providing default values in struct definition",
          "struct_best_practices", file_path) | violations]
      end
    end
    
    violations
  end

  defp check_data_immutability(content, file_path) do
    violations = []
    
    # Check for mutating operations (this is simplified)
    if Regex.match?(~r/Map\.put\(.*,.*,.*\)/, content) and
       String.contains?(content, "defstruct") do
      violations = [create_violation(:data_mutation, :info, nil,
        "Consider using struct update syntax: %{struct | field: value}",
        "immutability_patterns", file_path) | violations]
    end
    
    violations
  end

  defp check_data_structure_choice(content, file_path) do
    violations = []
    
    # Check for deeply nested maps
    if Regex.match?(~r/%\{.*%\{.*%\{/, content) do
      violations = [create_violation(:deep_nesting, :warning, nil,
        "Deeply nested maps are hard to work with. Consider flattening or using structs",
        "data_structure_design", file_path) | violations]
    end
    
    violations
  end

  defp check_access_patterns(content, file_path) do
    violations = []
    
    # This would analyze if access patterns match data structure choices
    # Implementation would be more sophisticated in practice
    
    violations
  end

  # Test Layer Validation Rules

  defp check_test_organization(content, file_path) do
    violations = []
    
    if String.contains?(file_path, "_test.exs") do
      # Check for describe blocks
      if Regex.match?(~r/test\s+"/, content) and 
         not Regex.match?(~r/describe\s+"/, content) do
        violations = [create_violation(:poor_test_organization, :info, nil,
          "Consider using 'describe' blocks to organize related tests",
          "test_organization", file_path) | violations]
      end
    end
    
    violations
  end

  defp check_test_behavior_focus(content, file_path) do
    violations = []
    
    # Check if tests focus on behavior rather than implementation
    if Regex.match?(~r/assert.*private_function/, content) do
      violations = [create_violation(:testing_implementation, :warning, nil,
        "Tests should focus on public behavior, not private implementation",
        "behavior_testing", file_path) | violations]
    end
    
    violations
  end

  defp check_test_naming(content, file_path) do
    violations = []
    
    tests = extract_test_names(content)
    
    Enum.each(tests, fn {test_name, line_num} ->
      if String.length(test_name) < 10 or not String.contains?(test_name, " ") do
        violations = [create_violation(:poor_test_naming, :info, line_num,
          "Test name '#{test_name}' should be more descriptive",
          "test_naming", file_path) | violations]
      end
    end)
    
    violations
  end

  defp check_setup_patterns(content, file_path) do
    violations = []
    
    # Check for repeated setup code
    if Enum.count(String.split(content, "setup"), fn _ -> true end) > 3 and
       not Regex.match?(~r/setup_all/, content) do
      violations = [create_violation(:repeated_setup, :info, nil,
        "Consider using setup_all or named setups to reduce duplication",
        "test_setup", file_path) | violations]
    end
    
    violations
  end

  defp check_assertion_quality(content, file_path) do
    violations = []
    
    # Check for generic assertions
    if Regex.match?(~r/assert\s+true/, content) or 
       Regex.match?(~r/assert\s+false/, content) do
      violations = [create_violation(:generic_assertions, :warning, nil,
        "Avoid generic assertions like 'assert true'. Use specific assertions",
        "assertion_quality", file_path) | violations]
    end
    
    violations
  end

  # Lifecycle Layer Validation Rules

  defp check_supervision_patterns(content, file_path) do
    violations = []
    
    if String.contains?(content, "Supervisor") do
      # Check for proper child specs
      if not Regex.match?(~r/children\s*=/, content) do
        violations = [create_violation(:missing_child_specs, :warning, nil,
          "Supervisor should define children specifications",
          "supervision_structure", file_path) | violations]
      end
    end
    
    violations
  end

  defp check_application_structure(content, file_path) do
    violations = []
    
    if String.contains?(content, "use Application") do
      # Check for proper start function
      if not Regex.match?(~r/def start/, content) do
        violations = [create_violation(:missing_start_function, :error, nil,
          "Application must implement start/2 function",
          "application_structure", file_path) | violations]
      end
    end
    
    violations
  end

  defp check_child_specs(content, file_path) do
    violations = []
    
    # Check for proper child specification format
    if Regex.match?(~r/children\s*=/, content) do
      if not Regex.match?(~r/\{.*,.*\}|\w+\.child_spec/, content) do
        violations = [create_violation(:improper_child_specs, :warning, nil,
          "Child specifications should follow proper format",
          "child_spec_format", file_path) | violations]
      end
    end
    
    violations
  end

  # Helper functions for validation logic

  defp create_violation(type, severity, line, message, rule, file_path) do
    %{
      type: type,
      severity: severity,
      line: line,
      message: message,
      rule: rule,
      file: file_path
    }
  end

  defp extract_functions(content) do
    # Simplified function extraction
    content
    |> String.split("\n")
    |> Enum.with_index()
    |> Enum.reduce([], fn {line, index}, acc ->
      case Regex.run(~r/def\s+(\w+)/, line) do
        [_, function_name] -> [{function_name, "", index + 1} | acc]
        _ -> acc
      end
    end)
    |> Enum.reverse()
  end

  defp extract_public_functions(content) do
    extract_functions(content)
    |> Enum.reject(fn {name, _, _} -> String.starts_with?(name, "_") end)
  end

  defp extract_test_names(content) do
    content
    |> String.split("\n")
    |> Enum.with_index()
    |> Enum.reduce([], fn {line, index}, acc ->
      case Regex.run(~r/test\s+"([^"]+)"/, line) do
        [_, test_name] -> [{test_name, index + 1} | acc]
        _ -> acc
      end
    end)
    |> Enum.reverse()
  end

  defp has_spec_for_function?(content, function_name) do
    Regex.match?(~r/@spec\s+#{function_name}/, content)
  end

  defp has_mixed_abstraction_levels?(_function_content) do
    # Simplified check - real implementation would analyze AST
    false
  end

  defp calculate_file_score(violations) do
    total_points = 100.0
    
    penalty = Enum.reduce(violations, 0.0, fn violation, acc ->
      penalty_value = case violation.severity do
        :error -> 10.0
        :warning -> 5.0
        :info -> 2.0
      end
      acc + penalty_value
    end)
    
    max(0.0, total_points - penalty)
  end

  defp calculate_compliance_score(validation_results) do
    if Enum.empty?(validation_results) do
      0.0
    else
      total_score = Enum.reduce(validation_results, 0.0, fn result, acc ->
        acc + result.score
      end)
      
      total_score / length(validation_results)
    end
  end

  defp extract_violations(validation_results) do
    Enum.flat_map(validation_results, fn result ->
      result.violations
    end)
  end

  defp generate_recommendations(validation_results) do
    violations = extract_violations(validation_results)
    
    violations
    |> Enum.group_by(fn violation -> violation.type end)
    |> Enum.map(fn {type, type_violations} ->
      count = length(type_violations)
      generate_recommendation_for_type(type, count)
    end)
    |> Enum.reject(&is_nil/1)
  end

  defp generate_file_recommendations(violations, layer) do
    recommendations = []
    
    recommendations = if Enum.any?(violations, fn v -> v.type == :side_effect end) do
      ["Move side effects to boundary layer"] ++ recommendations
    else
      recommendations
    end
    
    recommendations = if layer == :functions and 
                        Enum.any?(violations, fn v -> v.type == :poor_composition end) do
      ["Improve function composition with pipes"] ++ recommendations
    else
      recommendations
    end
    
    recommendations
  end

  defp generate_recommendation_for_type(type, count) do
    case type do
      :side_effect -> 
        "Found #{count} side effect(s) in functional core. Move these to boundary layer."
      :missing_spec -> 
        "#{count} public function(s) missing @spec. Add type specifications for better documentation."
      :poor_test_naming -> 
        "#{count} test(s) have poor naming. Use descriptive test names that explain behavior."
      :generic_assertions -> 
        "#{count} test(s) use generic assertions. Use specific assertions for better test clarity."
      _ -> 
        nil
    end
  end

  # Structure analysis helper functions

  defp find_unmapped_files(file_list, project_map) do
    mapped_paths = Map.values(project_map.layer_paths) |> Enum.reject(&is_nil/1)
    
    Enum.filter(file_list, fn file_path ->
      not is_file_in_mapped_layers?(file_path, mapped_paths) and 
      not is_standard_project_file?(file_path)
    end)
  end

  defp is_file_in_mapped_layers?(file_path, mapped_paths) do
    Enum.any?(mapped_paths, fn layer_path ->
      String.contains?(file_path, layer_path)
    end) or String.contains?(file_path, "test/")
  end

  defp is_standard_project_file?(file_path) do
    standard_patterns = [
      "mix.exs",
      "config/",
      "_build/",
      "deps/",
      ".git/",
      "README",
      "LICENSE"
    ]
    
    Enum.any?(standard_patterns, fn pattern ->
      String.contains?(file_path, pattern)
    end)
  end

  defp find_potential_new_layer_directories(file_list, project_map) do
    mapped_dirs = Map.values(project_map.layer_paths) 
                  |> Enum.reject(&is_nil/1)
                  |> Enum.map(&Path.dirname/1)
                  |> MapSet.new()
    
    all_dirs = file_list
               |> Enum.map(&Path.dirname/1)
               |> Enum.uniq()
               |> Enum.filter(fn dir -> 
                 String.contains?(dir, "/lib/") and not MapSet.member?(mapped_dirs, dir)
               end)
    
    # Look for directories with multiple Elixir files that could be new layers
    Enum.filter(all_dirs, fn dir ->
      files_in_dir = Enum.count(file_list, fn file -> String.starts_with?(file, dir) end)
      files_in_dir >= 3
    end)
  end

  defp detect_project_type_changes(file_list, project_map) do
    current_indicators = detect_current_project_indicators(file_list)
    
    type_change_detected = case project_map.project_type do
      :phoenix_web -> 
        not (current_indicators.has_phoenix_web? and current_indicators.has_web_files?)
      :phoenix_api -> 
        not (current_indicators.has_phoenix? and current_indicators.has_api_files?)
      :otp_application -> 
        not current_indicators.has_otp_patterns?
      :library -> 
        current_indicators.has_application_patterns?
      _ -> 
        false
    end
    
    change_description = if type_change_detected do
      describe_type_changes(project_map.project_type, current_indicators)
    else
      ""
    end
    
    %{
      has_changes?: type_change_detected,
      change_description: change_description,
      current_indicators: current_indicators
    }
  end

  defp detect_current_project_indicators(file_list) do
    content_samples = file_list
                      |> Enum.take(10)
                      |> Enum.map(&safe_read_file/1)
                      |> Enum.join("\n")
    
    %{
      has_phoenix?: String.contains?(content_samples, "Phoenix."),
      has_phoenix_web?: Enum.any?(file_list, &String.contains?(&1, "_web/")),
      has_web_files?: Enum.any?(file_list, &String.contains?(&1, "router.ex")),
      has_api_files?: Enum.any?(file_list, &String.contains?(&1, "api/")),
      has_otp_patterns?: String.contains?(content_samples, "GenServer") or String.contains?(content_samples, "Supervisor"),
      has_application_patterns?: String.contains?(content_samples, "use Application")
    }
  end

  defp safe_read_file(file_path) do
    case File.read(file_path) do
      {:ok, content} -> String.slice(content, 0, 1000)  # Read first 1KB for analysis
      {:error, _} -> ""
    end
  end

  defp describe_type_changes(original_type, current_indicators) do
    case original_type do
      :library when current_indicators.has_application_patterns? ->
        "library evolved into OTP application"
      :otp_application when current_indicators.has_phoenix? ->
        "OTP application now includes Phoenix"
      :phoenix_api when current_indicators.has_web_files? ->
        "Phoenix API now includes web components"
      _ ->
        "project structure has evolved"
    end
  end

  defp create_structure_violation(type, severity, message, rule) do
    %{
      type: type,
      severity: severity,
      line: nil,
      message: message,
      rule: rule,
      file: "project_structure"
    }
  end
end