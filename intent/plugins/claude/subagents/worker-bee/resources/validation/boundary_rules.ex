defmodule WorkerBee.Validation.BoundaryRules do
  @moduledoc """
  Validation rules for boundary layer compliance.
  
  The boundary layer handles state, side effects, and provides clean APIs
  while delegating business logic to the functional core.
  """

  @doc """
  Validates GenServer implementation patterns.
  """
  def validate_genserver_patterns(content, file_path) do
    violations = []
    
    if String.contains?(content, "use GenServer") do
      violations = violations ++ check_genserver_structure(content, file_path)
      violations = violations ++ check_callback_implementation(content, file_path)
      violations = violations ++ check_state_management(content, file_path)
      violations = violations ++ check_api_separation(content, file_path)
    end
    
    violations
  end

  @doc """
  Validates error handling patterns.
  """
  def validate_error_handling(content, file_path) do
    violations = []
    
    violations = violations ++ check_with_statements(content, file_path)
    violations = violations ++ check_tagged_tuples(content, file_path)
    violations = violations ++ check_error_propagation(content, file_path)
    
    violations
  end

  @doc """
  Validates API design patterns.
  """
  def validate_api_design(content, file_path) do
    violations = []
    
    violations = violations ++ check_function_specs(content, file_path)
    violations = violations ++ check_input_validation(content, file_path)
    violations = violations ++ check_api_consistency(content, file_path)
    violations = violations ++ check_backwards_compatibility(content, file_path)
    
    violations
  end

  @doc """
  Validates separation of concerns.
  """
  def validate_separation_of_concerns(content, file_path) do
    violations = []
    
    violations = violations ++ check_business_logic_delegation(content, file_path)
    violations = violations ++ check_side_effect_isolation(content, file_path)
    violations = violations ++ check_module_responsibilities(content, file_path)
    
    violations
  end

  # Private validation functions

  defp check_genserver_structure(content, file_path) do
    violations = []
    
    # Check for required callbacks
    required_callbacks = ["init"]
    missing_callbacks = Enum.filter(required_callbacks, fn callback ->
      not Regex.match?(~r/def #{callback}/, content)
    end)
    
    violations = Enum.reduce(missing_callbacks, violations, fn callback, acc ->
      [create_violation(:missing_callback, :error, nil,
        "GenServer must implement #{callback}/1 callback",
        "genserver_structure", file_path) | acc]
    end)
    
    # Check for handle_* implementation
    if not Regex.match?(~r/def handle_(call|cast|info)/, content) do
      violations = [create_violation(:incomplete_genserver, :warning, nil,
        "GenServer should implement at least one handle_* callback",
        "genserver_completeness", file_path) | violations]
    end
    
    violations
  end

  defp check_callback_implementation(content, file_path) do
    violations = []
    
    # Check handle_call return patterns
    handle_call_matches = Regex.scan(~r/def handle_call.*do(.*?)(?=def|\z)/s, content)
    
    Enum.reduce(handle_call_matches, violations, fn [_full_match, callback_body], acc ->
      if not Regex.match?(~r/\{:reply,.*,.*\}/, callback_body) do
        [create_violation(:improper_callback_return, :warning, nil,
          "handle_call should return {:reply, response, state}",
          "callback_patterns", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_state_management(content, file_path) do
    violations = []
    
    # Check for direct state mutation
    if Regex.match?(~r/state\s*=\s*.*/, content) and
       String.contains?(content, "use GenServer") do
      violations = [create_violation(:direct_state_mutation, :info, nil,
        "Consider functional state updates instead of direct mutation",
        "state_management", file_path) | violations]
    end
    
    # Check for state structure consistency
    if String.contains?(content, "use GenServer") and
       not Regex.match?(~r/@type state/, content) do
      violations = [create_violation(:missing_state_type, :info, nil,
        "Consider defining @type state :: ... for state structure",
        "state_documentation", file_path) | violations]
    end
    
    violations
  end

  defp check_api_separation(content, file_path) do
    violations = []
    
    # Check if client API and server callbacks are properly separated
    has_client_api = Regex.match?(~r/def (start_link|get_|set_|update_)/, content)
    has_server_callbacks = Regex.match?(~r/def (init|handle_)/, content)
    
    if has_client_api and has_server_callbacks do
      if not has_clear_api_separation?(content) do
        violations = [create_violation(:mixed_api_implementation, :info, nil,
          "Consider separating client API from server implementation with comments",
          "api_organization", file_path) | violations]
      end
    end
    
    violations
  end

  defp check_with_statements(content, file_path) do
    violations = []
    
    # Check for complex functions without with statements
    complex_functions = find_complex_functions(content)
    
    Enum.reduce(complex_functions, violations, fn {func_name, line_num}, acc ->
      func_content = extract_function_content(content, line_num)
      
      if has_multiple_operations?(func_content) and
         not Regex.match?(~r/with\s+.*<-/, func_content) do
        [create_violation(:missing_with_statement, :info, line_num,
          "Function '#{func_name}' could benefit from 'with' for error composition",
          "railway_oriented_programming", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_tagged_tuples(content, file_path) do
    violations = []
    
    # Check for untagged returns in public functions
    public_functions = extract_public_functions(content)
    
    Enum.reduce(public_functions, violations, fn {func_name, func_content, line_num}, acc ->
      if not has_tagged_returns?(func_content) and
         looks_like_operation_function?(func_name) do
        [create_violation(:untagged_returns, :info, line_num,
          "Function '#{func_name}' should return tagged tuples {:ok, result} or {:error, reason}",
          "tagged_tuple_returns", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_error_propagation(content, file_path) do
    violations = []
    
    # Check for proper error handling in with statements
    with_statements = Regex.scan(~r/with\s+.*do(.*?)(?:else(.*?))?end/s, content)
    
    Enum.reduce(with_statements, violations, fn [_full, _do_block, else_block], acc ->
      if else_block == "" or is_nil(else_block) do
        [create_violation(:missing_error_handling, :warning, nil,
          "'with' statement should have 'else' clause for error handling",
          "error_propagation", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_function_specs(content, file_path) do
    violations = []
    
    public_functions = extract_public_functions(content)
    
    Enum.reduce(public_functions, violations, fn {func_name, _func_content, line_num}, acc ->
      if not has_spec_before_function?(content, func_name, line_num) do
        [create_violation(:missing_spec, :info, line_num,
          "Public function '#{func_name}' should have @spec",
          "api_documentation", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_input_validation(content, file_path) do
    violations = []
    
    # Check if boundary functions validate input
    api_functions = extract_api_functions(content)
    
    Enum.reduce(api_functions, violations, fn {func_name, func_content, line_num}, acc ->
      if not has_input_validation?(func_content) and
         has_external_params?(func_content) do
        [create_violation(:missing_input_validation, :warning, line_num,
          "API function '#{func_name}' should validate input parameters",
          "input_validation", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_api_consistency(content, file_path) do
    violations = []
    
    # Check for consistent return patterns across API
    api_functions = extract_api_functions(content)
    return_patterns = Enum.map(api_functions, fn {_name, content, _line} ->
      extract_return_pattern(content)
    end)
    
    unique_patterns = Enum.uniq(return_patterns)
    
    if length(unique_patterns) > 2 do
      violations = [create_violation(:inconsistent_api, :info, nil,
        "API functions have inconsistent return patterns",
        "api_consistency", file_path) | violations]
    end
    
    violations
  end

  defp check_backwards_compatibility(content, file_path) do
    violations = []
    
    # Check for functions with many required parameters (hard to extend)
    functions = extract_function_definitions(content)
    
    Enum.reduce(functions, violations, fn {func_name, func_content, line_num}, acc ->
      param_count = count_required_parameters(func_content)
      
      if param_count > 4 do
        [create_violation(:too_many_parameters, :info, line_num,
          "Function '#{func_name}' has #{param_count} parameters. Consider using options map",
          "backwards_compatibility", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_business_logic_delegation(content, file_path) do
    violations = []
    
    # Check if boundary layer delegates to functional core
    if String.contains?(content, "use GenServer") do
      handle_functions = extract_handle_functions(content)
      
      Enum.reduce(handle_functions, violations, fn {func_name, func_content, line_num}, acc ->
        if has_complex_business_logic?(func_content) and
           not delegates_to_core?(func_content) do
          [create_violation(:business_logic_in_boundary, :warning, line_num,
            "#{func_name} contains business logic. Consider delegating to functional core",
            "separation_of_concerns", file_path) | acc]
        else
          acc
        end
      end)
    end
    
    violations
  end

  defp check_side_effect_isolation(content, file_path) do
    violations = []
    
    # This is actually expected in boundary layer, so we check for proper patterns
    side_effect_operations = [
      ~r/File\./,
      ~r/HTTPoison\./,
      ~r/Repo\./,
      ~r/Logger\./
    ]
    
    Enum.reduce(side_effect_operations, violations, fn pattern, acc ->
      if Regex.match?(pattern, content) do
        # Check if side effects are properly handled with error patterns
        if not has_proper_side_effect_handling?(content, pattern) do
          [create_violation(:improper_side_effect_handling, :info, nil,
            "Side effects should be wrapped with proper error handling",
            "side_effect_isolation", file_path) | acc]
        else
          acc
        end
      else
        acc
      end
    end)
  end

  defp check_module_responsibilities(content, file_path) do
    violations = []
    
    # Check if module has too many responsibilities
    responsibility_indicators = [
      {~r/use GenServer/, "process_management"},
      {~r/def.*validate/, "validation"},
      {~r/def.*format/, "formatting"},
      {~r/def.*parse/, "parsing"},
      {~r/File\./, "file_operations"},
      {~r/HTTPoison\./, "http_operations"}
    ]
    
    responsibilities = Enum.filter(responsibility_indicators, fn {pattern, _name} ->
      Regex.match?(pattern, content)
    end)
    
    if length(responsibilities) > 3 do
      responsibility_names = Enum.map(responsibilities, fn {_pattern, name} -> name end)
      violations = [create_violation(:too_many_responsibilities, :info, nil,
        "Module has too many responsibilities: #{Enum.join(responsibility_names, ", ")}",
        "single_responsibility", file_path) | violations]
    end
    
    violations
  end

  # Helper functions

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

  defp has_clear_api_separation?(content) do
    # Check for comments separating API from implementation
    Regex.match?(~r/# (Client API|Server|Implementation|Callbacks)/, content)
  end

  defp find_complex_functions(content) do
    functions = extract_function_definitions(content)
    
    Enum.filter(functions, fn {_name, func_content, _line} ->
      calculate_complexity(func_content) > 3
    end)
    |> Enum.map(fn {name, _content, line} -> {name, line} end)
  end

  defp extract_function_content(_content, _line_num) do
    # Simplified - real implementation would parse AST
    ""
  end

  defp has_multiple_operations?(func_content) do
    operation_count = 0
    operation_count = operation_count + Enum.count(Regex.scan(~r/\w+\.\w+\(/, func_content))
    operation_count = operation_count + Enum.count(Regex.scan(~r/GenServer\./, func_content))
    
    operation_count > 2
  end

  defp extract_public_functions(content) do
    extract_function_definitions(content)
    |> Enum.reject(fn {name, _content, _line} -> String.starts_with?(name, "_") end)
  end

  defp extract_function_definitions(content) do
    # Simplified function extraction
    content
    |> String.split("\n")
    |> Enum.with_index(1)
    |> Enum.reduce([], fn {line, line_num}, acc ->
      case Regex.run(~r/def\s+(\w+)/, line) do
        [_, func_name] -> 
          [{func_name, "", line_num} | acc]
        _ -> 
          acc
      end
    end)
    |> Enum.reverse()
  end

  defp has_tagged_returns?(func_content) do
    Regex.match?(~r/\{:ok,|\{:error,/, func_content)
  end

  defp looks_like_operation_function?(func_name) do
    operation_verbs = ["create", "update", "delete", "get", "fetch", "process", "handle", "perform"]
    Enum.any?(operation_verbs, &String.starts_with?(func_name, &1))
  end

  defp has_spec_before_function?(content, func_name, line_num) do
    lines = String.split(content, "\n")
    
    if line_num > 1 do
      previous_line = Enum.at(lines, line_num - 2, "")
      Regex.match?(~r/@spec\s+#{func_name}/, previous_line)
    else
      false
    end
  end

  defp extract_api_functions(content) do
    # Functions that look like public API (not handle_* or init)
    extract_function_definitions(content)
    |> Enum.reject(fn {name, _content, _line} -> 
      String.starts_with?(name, "_") or
      String.starts_with?(name, "handle_") or
      name == "init"
    end)
  end

  defp has_input_validation?(func_content) do
    validation_patterns = [
      ~r/when\s+is_/,
      ~r/validate/,
      ~r/\|>\s*check/,
      ~r/with\s+.*<-\s*.*validate/
    ]
    
    Enum.any?(validation_patterns, &Regex.match?(&1, func_content))
  end

  defp has_external_params?(func_content) do
    # Simple heuristic - functions with parameters likely have external input
    Regex.match?(~r/def\s+\w+\([^)]+\)/, func_content)
  end

  defp extract_return_pattern(func_content) do
    cond do
      Regex.match?(~r/\{:ok,.*\}/, func_content) -> :tagged_tuple
      Regex.match?(~r/\w+/, func_content) -> :direct_value
      true -> :unknown
    end
  end

  defp count_required_parameters(func_content) do
    case Regex.run(~r/def\s+\w+\(([^)]*)\)/, func_content) do
      [_, params_str] ->
        params_str
        |> String.split(",")
        |> Enum.reject(&String.contains?(&1, "\\\\"))  # Exclude default params
        |> length()
      _ ->
        0
    end
  end

  defp extract_handle_functions(content) do
    extract_function_definitions(content)
    |> Enum.filter(fn {name, _content, _line} -> 
      String.starts_with?(name, "handle_")
    end)
  end

  defp has_complex_business_logic?(func_content) do
    business_logic_indicators = [
      ~r/calculate/,
      ~r/compute/,
      ~r/process.*data/,
      ~r/transform/,
      ~r/aggregate/
    ]
    
    Enum.any?(business_logic_indicators, &Regex.match?(&1, func_content))
  end

  defp delegates_to_core?(func_content) do
    delegation_patterns = [
      ~r/\w+Core\./,
      ~r/\w+Service\./,
      ~r/\w+Logic\./
    ]
    
    Enum.any?(delegation_patterns, &Regex.match?(&1, func_content))
  end

  defp has_proper_side_effect_handling?(content, _pattern) do
    # Check if side effects are wrapped in proper error handling
    Regex.match?(~r/with\s+.*<-|case\s+.*do/, content)
  end

  defp calculate_complexity(func_content) do
    # Simplified complexity calculation
    complexity = 1
    complexity = complexity + Enum.count(Regex.scan(~r/if\s+/, func_content))
    complexity = complexity + Enum.count(Regex.scan(~r/case\s+/, func_content))
    complexity = complexity + Enum.count(Regex.scan(~r/with\s+/, func_content))
    complexity
  end
end