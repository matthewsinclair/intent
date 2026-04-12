#!/usr/bin/env elixir
# configure_credo.exs - Configure .credo.exs to load Intent custom checks
# Copyright (c) 2026 Matthew Sinclair
# Licensed under the MIT License (see LICENSE file)
#
# Usage:
#   elixir /path/to/intent/lib/scripts/configure_credo.exs
#   elixir /path/to/intent/lib/scripts/configure_credo.exs --remove-stale
#
# Runs in the target project's working directory.
# Idempotent -- safe to run on install and upgrade.

Mix.install([])

defmodule ConfigureCredo do
  @moduledoc false

  @custom_checks [
    {Mix.Checks.BracketAccessOnStruct, []},
    {Mix.Checks.DebugArtifacts, []},
    {Mix.Checks.HighlanderSuspect, []},
    {Mix.Checks.MapGetOnStruct, []},
    {Mix.Checks.MissingImplAnnotation, []},
    {Mix.Checks.ThickCoordinator, []}
  ]

  @stale_checks [
    Mix.Checks.BooleanOperators,
    Mix.Checks.DependencyGraph
  ]

  @requires_entry "credo_checks/"

  @default_credo_config """
  %{
    configs: [
      %{
        name: "default",
        files: %{
          included: [
            "lib/",
            "src/",
            "web/",
            "apps/*/lib/",
            "apps/*/src/"
          ],
          excluded: [~r"/_build/", ~r"/deps/", ~r"/node_modules/"]
        },
        requires: ["credo_checks/"],
        strict: false,
        parse_timeout: 5000,
        color: true,
        checks: %{
          enabled: [
            # Intent custom checks
            {Mix.Checks.BracketAccessOnStruct, []},
            {Mix.Checks.DebugArtifacts, []},
            {Mix.Checks.HighlanderSuspect, []},
            {Mix.Checks.MapGetOnStruct, []},
            {Mix.Checks.MissingImplAnnotation, []},
            {Mix.Checks.ThickCoordinator, []}
          ]
        }
      }
    ]
  }
  """

  def run(args) do
    remove_stale = "--remove-stale" in args

    cond do
      not File.exists?(".credo.exs") ->
        create_default_config()

      true ->
        patch_existing_config(remove_stale)
    end

    check_mix_exs_for_elixirc_paths()
  end

  defp create_default_config do
    File.write!(".credo.exs", @default_credo_config)
    IO.puts("configured: .credo.exs (created with #{length(@custom_checks)} custom checks)")
  end

  defp patch_existing_config(remove_stale) do
    config = parse_credo_config()

    case config do
      %{configs: configs} when is_list(configs) ->
        {updated_configs, changes} = patch_configs(configs, remove_stale)
        updated = %{config | configs: updated_configs}

        if changes > 0 do
          write_config(updated)

          IO.puts(
            "configured: .credo.exs (#{changes} change(s), #{length(@custom_checks)} checks registered)"
          )
        else
          IO.puts("ok: .credo.exs already configured (#{length(@custom_checks)} checks present)")
        end

      _ ->
        IO.puts(:stderr, "error: .credo.exs has unexpected structure (missing configs: key)")
        System.halt(1)
    end
  end

  defp parse_credo_config do
    case Code.eval_file(".credo.exs") do
      {config, _bindings} when is_map(config) ->
        config

      _ ->
        IO.puts(:stderr, "error: .credo.exs did not evaluate to a map")
        System.halt(1)
    end
  end

  defp patch_configs(configs, remove_stale) do
    Enum.map_reduce(configs, 0, fn config, total_changes ->
      {config, changes} = ensure_requires(config)
      {config, changes2} = ensure_checks(config)

      {config, changes3} =
        if remove_stale do
          remove_stale_checks(config)
        else
          {config, 0}
        end

      {config, total_changes + changes + changes2 + changes3}
    end)
  end

  defp ensure_requires(config) do
    current = Map.get(config, :requires, [])

    if @requires_entry in current do
      {config, 0}
    else
      {Map.put(config, :requires, current ++ [@requires_entry]), 1}
    end
  end

  defp ensure_checks(config) do
    current_checks = get_checks_list(config)
    existing_modules = extract_check_modules(current_checks)

    missing =
      @custom_checks
      |> Enum.reject(fn {mod, _opts} -> mod in existing_modules end)

    if missing == [] do
      {config, 0}
    else
      updated_checks = current_checks ++ missing
      {put_checks_list(config, updated_checks), length(missing)}
    end
  end

  defp remove_stale_checks(config) do
    current_checks = get_checks_list(config)

    {kept, removed_count} =
      Enum.reduce(current_checks, {[], 0}, fn check, {acc, count} ->
        mod = extract_module(check)

        if mod in @stale_checks do
          {acc, count + 1}
        else
          {[check | acc], count}
        end
      end)

    if removed_count > 0 do
      {put_checks_list(config, Enum.reverse(kept)), removed_count}
    else
      {config, 0}
    end
  end

  # Handle both old-style (flat list) and new-style (%{enabled: [...]}) checks
  defp get_checks_list(%{checks: %{enabled: checks}}) when is_list(checks), do: checks
  defp get_checks_list(%{checks: checks}) when is_list(checks), do: checks
  defp get_checks_list(_), do: []

  defp put_checks_list(%{checks: %{enabled: _}} = config, checks) do
    put_in(config, [:checks, :enabled], checks)
  end

  defp put_checks_list(%{checks: _} = config, checks) do
    Map.put(config, :checks, checks)
  end

  defp put_checks_list(config, checks) do
    Map.put(config, :checks, checks)
  end

  defp extract_check_modules(checks) do
    Enum.map(checks, &extract_module/1)
  end

  defp extract_module({mod, _opts}), do: mod
  defp extract_module({mod, _opts, _extra}), do: mod
  defp extract_module(mod) when is_atom(mod), do: mod
  defp extract_module(_), do: nil

  defp write_config(config) do
    content = inspect(config, pretty: true, limit: :infinity, width: 98)
    File.write!(".credo.exs", content <> "\n")
  end

  defp check_mix_exs_for_elixirc_paths do
    if File.exists?("mix.exs") do
      content = File.read!("mix.exs")

      if String.contains?(content, "credo_checks") do
        IO.puts(
          :stderr,
          "warning: \"credo_checks\" found in mix.exs -- this is incorrect and should be removed from elixirc_paths"
        )
      end
    end
  end
end

ConfigureCredo.run(System.argv())
