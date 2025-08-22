defmodule WorkerBee.Validation.DataRules do
  @moduledoc """
  Validation rules for data layer compliance.
  
  Data structures should be immutable, well-designed, and follow
  appropriate access patterns.
  """

  def validate_struct_definitions(content, file_path) do
    violations = []
    
    if Regex.match?(~r/defstruct/, content) do
      violations = violations ++ check_default_values(content, file_path)
      violations = violations ++ check_field_types(content, file_path)
      violations = violations ++ check_struct_documentation(content, file_path)
    end
    
    violations
  end

  def validate_data_immutability(content, file_path) do
    violations = []
    
    violations = violations ++ check_mutation_patterns(content, file_path)
    violations = violations ++ check_update_syntax(content, file_path)
    
    violations
  end

  def validate_data_structure_choice(content, file_path) do
    violations = []
    
    violations = violations ++ check_deep_nesting(content, file_path)
    violations = violations ++ check_access_patterns(content, file_path)
    
    violations
  end

  # Simplified implementation for transfer
  
  defp check_default_values(content, file_path) do
    if Regex.match?(~r/defstruct\s+\[/, content) and
       not Regex.match?(~r/defstruct.*:.*,/, content) do
      [create_violation(:struct_without_defaults, :info, nil,
        "Consider providing default values in struct definition",
        "struct_best_practices", file_path)]
    else
      []
    end
  end

  defp check_field_types(_content, _file_path), do: []
  defp check_struct_documentation(_content, _file_path), do: []
  defp check_mutation_patterns(_content, _file_path), do: []
  defp check_update_syntax(_content, _file_path), do: []
  
  defp check_deep_nesting(content, file_path) do
    if Regex.match?(~r/%\{.*%\{.*%\{/, content) do
      [create_violation(:deep_nesting, :warning, nil,
        "Deeply nested maps are hard to work with. Consider flattening or using structs",
        "data_structure_design", file_path)]
    else
      []
    end
  end

  defp check_access_patterns(_content, _file_path), do: []

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