defmodule WorkerBee.Validation.FunctionalCoreRules do
  @moduledoc """
  Validation rules for functional core layer compliance.
  
  The functional core must be pure, composable, and free from side effects.
  These rules enforce the fundamental principles of Worker-Bee Driven Design.
  """

  @doc """
  Validates that functional core modules follow purity principles.
  """
  def validate_purity(content, file_path) do
    violations = []
    
    # Check for GenServer operations
    violations = violations ++ check_genserver_calls(content, file_path)
    
    # Check for process operations
    violations = violations ++ check_process_operations(content, file_path)
    
    # Check for file I/O operations
    violations = violations ++ check_file_operations(content, file_path)
    
    # Check for network operations
    violations = violations ++ check_network_operations(content, file_path)
    
    # Check for logging operations
    violations = violations ++ check_logging_operations(content, file_path)
    
    # Check for database operations
    violations = violations ++ check_database_operations(content, file_path)
    
    violations
  end

  @doc """
  Validates function composition patterns.
  """
  def validate_composition(content, file_path) do
    violations = []
    
    # Check for pipeline-friendly function design
    violations = violations ++ check_pipeline_design(content, file_path)
    
    # Check for proper error handling composition
    violations = violations ++ check_error_composition(content, file_path)
    
    # Check for function chaining patterns
    violations = violations ++ check_function_chaining(content, file_path)
    
    violations
  end

  @doc """
  Validates single responsibility principle adherence.
  """
  def validate_single_responsibility(content, file_path) do
    violations = []
    
    # Check function length and complexity
    violations = violations ++ check_function_complexity(content, file_path)
    
    # Check for mixed abstraction levels
    violations = violations ++ check_abstraction_levels(content, file_path)
    
    # Check for proper function naming
    violations = violations ++ check_function_naming(content, file_path)
    
    violations
  end

  @doc """
  Validates proper use of pattern matching.
  """
  def validate_pattern_matching(content, file_path) do
    violations = []
    
    # Check for pattern matching over conditionals
    violations = violations ++ check_pattern_vs_conditionals(content, file_path)
    
    # Check for guard clause usage
    violations = violations ++ check_guard_usage(content, file_path)
    
    # Check for multiple function heads
    violations = violations ++ check_function_heads(content, file_path)
    
    violations
  end

  # Private validation functions

  defp check_genserver_calls(content, file_path) do
    genserver_patterns = [
      ~r/GenServer\.(call|cast|start|start_link)/,
      ~r/Agent\.(get|update|start|start_link)/,
      ~r/Task\.(start|start_link|async)/
    ]
    
    Enum.flat_map(genserver_patterns, fn pattern ->
      case Regex.run(pattern, content, return: :index) do
        nil -> []
        _ -> [create_violation(:side_effect, :error, nil,
              "Functional core should not contain GenServer/Agent/Task operations",
              "functional_core_purity", file_path)]
      end
    end)
  end

  defp check_process_operations(content, file_path) do
    process_patterns = [
      ~r/spawn(_link|_monitor)?/,
      ~r/Process\.(send|send_after|exit|flag)/,
      ~r/receive\s+do/,
      ~r/:timer\./
    ]
    
    Enum.flat_map(process_patterns, fn pattern ->
      case Regex.run(pattern, content, return: :index) do
        nil -> []
        _ -> [create_violation(:side_effect, :error, nil,
              "Functional core should not perform process operations",
              "functional_core_purity", file_path)]
      end
    end)
  end

  defp check_file_operations(content, file_path) do
    file_patterns = [
      ~r/File\.(read|write|open|close|copy|rename|rm|mkdir)/,
      ~r/IO\.(puts|write|read|gets)/,
      ~r/Path\.(wildcard|expand)/
    ]
    
    Enum.flat_map(file_patterns, fn pattern ->
      case Regex.run(pattern, content, return: :index) do
        nil -> []
        _ -> [create_violation(:side_effect, :error, nil,
              "Functional core should not perform file I/O operations",
              "functional_core_purity", file_path)]
      end
    end)
  end

  defp check_network_operations(content, file_path) do
    network_patterns = [
      ~r/HTTPoison\./,
      ~r/Tesla\./,
      ~r/Req\./,
      ~r/:httpc\./,
      ~r/:gen_tcp/,
      ~r/:ssl/
    ]
    
    Enum.flat_map(network_patterns, fn pattern ->
      case Regex.run(pattern, content, return: :index) do
        nil -> []
        _ -> [create_violation(:side_effect, :error, nil,
              "Functional core should not perform network operations",
              "functional_core_purity", file_path)]
      end
    end)
  end

  defp check_logging_operations(content, file_path) do
    if Regex.match?(~r/Logger\.(info|debug|warn|error)/, content) do
      [create_violation(:side_effect, :warning, nil,
        "Consider moving logging to boundary layer",
        "functional_core_purity", file_path)]
    else
      []
    end
  end

  defp check_database_operations(content, file_path) do
    db_patterns = [
      ~r/Repo\.(get|insert|update|delete|all)/,
      ~r/Ecto\.Query/,
      ~r/from\s+\w+\s+in/,
      ~r/:mnesia\./
    ]
    
    Enum.flat_map(db_patterns, fn pattern ->
      case Regex.run(pattern, content, return: :index) do
        nil -> []
        _ -> [create_violation(:side_effect, :error, nil,
              "Functional core should not perform database operations",
              "functional_core_purity", file_path)]
      end
    end)
  end

  defp check_pipeline_design(content, file_path) do
    violations = []
    
    # Check for data-last function design
    functions = extract_function_definitions(content)
    
    Enum.reduce(functions, violations, fn {func_name, func_content, line_num}, acc ->
      if has_poor_pipeline_design?(func_content) do
        [create_violation(:poor_composition, :info, line_num,
          "Function '#{func_name}' could be more pipeline-friendly",
          "pipeline_design", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_error_composition(content, file_path) do
    violations = []
    
    # Check for proper tagged tuple usage
    if Regex.match?(~r/def \w+.*do/, content) and
       not Regex.match?(~r/\{:ok,|{:error,/, content) do
      violations = [create_violation(:poor_error_handling, :info, nil,
        "Consider using tagged tuples {:ok, result} or {:error, reason}",
        "error_composition", file_path) | violations]
    end
    
    # Check for with statement usage in complex functions
    complex_functions = find_complex_functions(content)
    
    Enum.reduce(complex_functions, violations, fn {func_name, line_num}, acc ->
      if not has_with_statement_nearby?(content, line_num) do
        [create_violation(:missing_error_composition, :info, line_num,
          "Complex function '#{func_name}' might benefit from 'with' statement",
          "error_composition", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_function_chaining(content, file_path) do
    violations = []
    
    # Check for excessive nesting instead of chaining
    if Regex.match?(~r/\(\s*\w+\(\s*\w+\(\s*\w+\(/, content) do
      violations = [create_violation(:poor_composition, :info, nil,
        "Consider using pipe operator instead of nested function calls",
        "function_chaining", file_path) | violations]
    end
    
    violations
  end

  defp check_function_complexity(content, file_path) do
    violations = []
    
    functions = extract_function_definitions(content)
    
    Enum.reduce(functions, violations, fn {func_name, func_content, line_num}, acc ->
      complexity_score = calculate_complexity(func_content)
      
      cond do
        complexity_score > 10 ->
          [create_violation(:high_complexity, :warning, line_num,
            "Function '#{func_name}' is too complex (score: #{complexity_score})",
            "function_complexity", file_path) | acc]
        
        complexity_score > 7 ->
          [create_violation(:moderate_complexity, :info, line_num,
            "Function '#{func_name}' could be simplified (score: #{complexity_score})",
            "function_complexity", file_path) | acc]
        
        true ->
          acc
      end
    end)
  end

  defp check_abstraction_levels(content, file_path) do
    violations = []
    
    functions = extract_function_definitions(content)
    
    Enum.reduce(functions, violations, fn {func_name, func_content, line_num}, acc ->
      if has_mixed_abstraction_levels?(func_content) do
        [create_violation(:mixed_abstraction, :warning, line_num,
          "Function '#{func_name}' mixes different abstraction levels",
          "abstraction_consistency", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_function_naming(content, file_path) do
    violations = []
    
    functions = extract_function_definitions(content)
    
    Enum.reduce(functions, violations, fn {func_name, _func_content, line_num}, acc ->
      cond do
        String.length(func_name) < 3 ->
          [create_violation(:poor_naming, :info, line_num,
            "Function name '#{func_name}' is too short",
            "function_naming", file_path) | acc]
        
        not Regex.match?(~r/^[a-z_][a-z0-9_]*[?!]?$/, func_name) ->
          [create_violation(:poor_naming, :warning, line_num,
            "Function name '#{func_name}' doesn't follow Elixir conventions",
            "function_naming", file_path) | acc]
        
        String.contains?(func_name, ["temp", "tmp", "test", "foo", "bar"]) ->
          [create_violation(:poor_naming, :info, line_num,
            "Function name '#{func_name}' appears to be a placeholder",
            "function_naming", file_path) | acc]
        
        true ->
          acc
      end
    end)
  end

  defp check_pattern_vs_conditionals(content, file_path) do
    violations = []
    
    # Look for if/else that could be pattern matching
    if_else_patterns = Regex.scan(~r/if\s+.*\s+do.*else.*end/s, content, return: :index)
    
    Enum.reduce(if_else_patterns, violations, fn [{start, _length}], acc ->
      line_num = count_lines_to_position(content, start)
      
      [create_violation(:suboptimal_pattern_matching, :info, line_num,
        "Consider using pattern matching instead of if/else",
        "pattern_matching_preference", file_path) | acc]
    end)
  end

  defp check_guard_usage(content, file_path) do
    violations = []
    
    # Check for type checks that could be guards
    type_check_patterns = [
      ~r/is_atom\(/,
      ~r/is_binary\(/,
      ~r/is_integer\(/,
      ~r/is_list\(/,
      ~r/is_map\(/
    ]
    
    Enum.reduce(type_check_patterns, violations, fn pattern, acc ->
      if Regex.match?(pattern, content) and 
         not Regex.match?(~r/when\s+is_\w+/, content) do
        [create_violation(:missing_guards, :info, nil,
          "Consider using guard clauses for type checks",
          "guard_usage", file_path) | acc]
      else
        acc
      end
    end)
  end

  defp check_function_heads(content, file_path) do
    violations = []
    
    # Check for functions that could benefit from multiple heads
    functions = extract_function_definitions(content)
    
    Enum.reduce(functions, violations, fn {func_name, func_content, line_num}, acc ->
      if has_complex_case_statements?(func_content) and
         not has_multiple_function_heads?(content, func_name) do
        [create_violation(:could_use_function_heads, :info, line_num,
          "Function '#{func_name}' could use multiple function heads instead of case",
          "function_heads", file_path) | acc]
      else
        acc
      end
    end)
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

  defp extract_function_definitions(content) do
    # Simplified function extraction
    content
    |> String.split("\n")
    |> Enum.with_index(1)
    |> Enum.reduce([], fn {line, line_num}, acc ->
      case Regex.run(~r/def\s+(\w+)/, line) do
        [_, func_name] -> 
          # Extract function content (simplified)
          func_content = extract_function_content(content, line_num)
          [{func_name, func_content, line_num} | acc]
        _ -> 
          acc
      end
    end)
    |> Enum.reverse()
  end

  defp extract_function_content(_content, _start_line) do
    # Simplified - in real implementation would parse to find function end
    ""
  end

  defp has_poor_pipeline_design?(_func_content) do
    # Simplified check
    false
  end

  defp find_complex_functions(content) do
    functions = extract_function_definitions(content)
    
    Enum.filter(functions, fn {_name, func_content, line_num} ->
      complexity = calculate_complexity(func_content)
      complexity > 5
    end)
    |> Enum.map(fn {name, _content, line_num} -> {name, line_num} end)
  end

  defp has_with_statement_nearby?(_content, _line_num) do
    # Simplified check
    false
  end

  defp calculate_complexity(func_content) do
    # Simplified complexity calculation
    complexity = 1
    
    complexity = complexity + Enum.count(Regex.scan(~r/if\s+/, func_content))
    complexity = complexity + Enum.count(Regex.scan(~r/case\s+/, func_content))
    complexity = complexity + Enum.count(Regex.scan(~r/cond\s+/, func_content))
    complexity = complexity + Enum.count(Regex.scan(~r/with\s+/, func_content))
    
    complexity
  end

  defp has_mixed_abstraction_levels?(_func_content) do
    # Simplified check - real implementation would analyze AST
    false
  end

  defp count_lines_to_position(content, position) do
    content
    |> String.slice(0, position)
    |> String.split("\n")
    |> length()
  end

  defp has_complex_case_statements?(func_content) do
    case_matches = Regex.scan(~r/case\s+.*\s+do/, func_content)
    length(case_matches) > 0
  end

  defp has_multiple_function_heads?(content, func_name) do
    function_heads = Regex.scan(~r/def\s+#{func_name}/, content)
    length(function_heads) > 1
  end
end