defmodule WorkerBee.Validation.TestingRules do
  @moduledoc """
  Validation rules for testing layer compliance.
  
  Tests should focus on behavior, be well-organized, and provide
  comprehensive coverage of the application logic.
  """

  def validate_test_organization(content, file_path) do
    violations = []
    
    violations = violations ++ check_describe_blocks(content, file_path)
    violations = violations ++ check_test_naming(content, file_path)
    violations = violations ++ check_setup_patterns(content, file_path)
    
    violations
  end

  def validate_test_behavior_focus(content, file_path) do
    violations = []
    
    violations = violations ++ check_behavior_vs_implementation(content, file_path)
    violations = violations ++ check_assertion_quality(content, file_path)
    violations = violations ++ check_test_isolation(content, file_path)
    
    violations
  end

  # Implementation continues with comprehensive test validation rules...
  # This is a simplified version for the transfer

  defp check_describe_blocks(content, file_path) do
    if String.contains?(file_path, "_test.exs") and
       Regex.match?(~r/test\s+"/, content) and
       not Regex.match?(~r/describe\s+"/, content) do
      [create_violation(:missing_describe_blocks, :info, nil,
        "Consider using 'describe' blocks to organize related tests",
        "test_organization", file_path)]
    else
      []
    end
  end

  defp check_test_naming(content, file_path) do
    test_matches = Regex.scan(~r/test\s+"([^"]+)"/, content)
    
    Enum.flat_map(test_matches, fn [_full, test_name] ->
      if String.length(test_name) < 10 do
        [create_violation(:poor_test_naming, :info, nil,
          "Test name '#{test_name}' should be more descriptive",
          "test_naming", file_path)]
      else
        []
      end
    end)
  end

  defp check_behavior_vs_implementation(content, file_path) do
    if Regex.match?(~r/assert.*private_function/, content) do
      [create_violation(:testing_implementation, :warning, nil,
        "Tests should focus on public behavior, not private implementation",
        "behavior_testing", file_path)]
    else
      []
    end
  end

  defp check_assertion_quality(content, file_path) do
    violations = []
    
    if Regex.match?(~r/assert\s+true/, content) or 
       Regex.match?(~r/assert\s+false/, content) do
      violations = [create_violation(:generic_assertions, :warning, nil,
        "Avoid generic assertions like 'assert true'. Use specific assertions",
        "assertion_quality", file_path) | violations]
    end
    
    violations
  end

  defp check_setup_patterns(_content, _file_path), do: []
  defp check_test_isolation(_content, _file_path), do: []

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
end