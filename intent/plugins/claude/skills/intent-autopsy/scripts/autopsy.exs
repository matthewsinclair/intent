#!/usr/bin/env elixir
# autopsy.exs - Pre-process Claude Code JSONL session files for autopsy analysis
# Copyright (c) 2026 Matthew Sinclair
# Licensed under the MIT License (see LICENSE file)
#
# Usage:
#   elixir autopsy.exs --days 7
#   elixir autopsy.exs --days 7 --min-compactions 1
#   elixir autopsy.exs --days 7 --project Intent
#   elixir autopsy.exs --days 7 --banned-words "delve,unfortunately"
#   elixir autopsy.exs --days 7 --banned-file ~/banned-words.txt
#   elixir autopsy.exs --days 7 -o findings.json

Mix.install([{:jason, "~> 1.4"}])

defmodule Autopsy.Parser do
  @moduledoc """
  Parses JSONL session files. Extracts messages, compaction boundaries, and tool uses.
  """

  @doc "Parse a single JSONL file into a list of message maps."
  def parse_file(path) do
    path
    |> File.stream!()
    |> Stream.with_index()
    |> Stream.map(fn {line, idx} ->
      case Jason.decode(line) do
        {:ok, obj} -> Map.put(obj, "_line", idx + 1)
        {:error, _} -> nil
      end
    end)
    |> Stream.reject(&is_nil/1)
    |> Enum.to_list()
  end

  @doc "Extract user and assistant messages from parsed lines."
  def extract_messages(lines) do
    Enum.filter(lines, fn line ->
      line["type"] in ["user", "assistant"]
    end)
  end

  @doc "Extract compaction boundaries from parsed lines."
  def extract_compactions(lines) do
    lines
    |> Enum.filter(fn line ->
      line["type"] == "system" and line["subtype"] == "compact_boundary"
    end)
    |> Enum.map(fn line ->
      %{
        "line_number" => line["_line"],
        "timestamp" => line["timestamp"],
        "trigger" => get_in(line, ["compactMetadata", "trigger"]),
        "pre_tokens" => get_in(line, ["compactMetadata", "preTokens"]),
        "uuid" => line["uuid"]
      }
    end)
  end

  @doc "Extract tool use blocks from assistant messages."
  def extract_tool_uses(messages) do
    messages
    |> Enum.filter(&(&1["type"] == "assistant"))
    |> Enum.flat_map(fn msg ->
      content = get_in(msg, ["message", "content"]) || []

      case content do
        items when is_list(items) ->
          items
          |> Enum.filter(&(is_map(&1) and &1["type"] == "tool_use"))
          |> Enum.map(fn tool ->
            %{
              "line_number" => msg["_line"],
              "tool_name" => tool["name"],
              "input" => tool["input"],
              "session_id" => msg["sessionId"],
              "timestamp" => msg["timestamp"]
            }
          end)

        _ ->
          []
      end
    end)
  end

  @doc "Get text content from a user or assistant message."
  def message_text(msg) do
    content = get_in(msg, ["message", "content"]) || ""

    case content do
      text when is_binary(text) ->
        text

      items when is_list(items) ->
        items
        |> Enum.filter(&(is_map(&1) and &1["type"] == "text"))
        |> Enum.map(& &1["text"])
        |> Enum.join("\n")

      _ ->
        ""
    end
  end

  @doc "Determine if a message line number is after a given compaction boundary."
  def post_compaction?(line_number, compactions) do
    Enum.any?(compactions, fn c -> line_number > c["line_number"] end)
  end

  @doc "Get the compaction index (0-based count of compactions before this line)."
  def compaction_index(line_number, compactions) do
    Enum.count(compactions, fn c -> c["line_number"] < line_number end)
  end

  @doc "Find all JSONL session files for a project directory."
  def find_session_files(project_dir, opts \\ []) do
    days = Keyword.get(opts, :days, 7)
    cutoff = DateTime.utc_now() |> DateTime.add(-days * 86400)

    project_dir
    |> Path.join("*.jsonl")
    |> Path.wildcard()
    |> Enum.filter(fn path ->
      case File.stat(path, time: :posix) do
        {:ok, %{mtime: mtime}} ->
          {:ok, file_time} = DateTime.from_unix(mtime)
          DateTime.compare(file_time, cutoff) == :gt

        _ ->
          false
      end
    end)
    |> Enum.sort()
  end

  @doc "Resolve project directory from project name or use auto-detection."
  def resolve_project_dir(project_name) do
    claude_projects = Path.expand("~/.claude/projects")

    if project_name do
      # Find directories matching the project name
      claude_projects
      |> Path.join("*")
      |> Path.wildcard()
      |> Enum.filter(fn dir ->
        basename = Path.basename(dir)
        String.contains?(String.downcase(basename), String.downcase(project_name))
      end)
      |> case do
        [dir] -> {:ok, dir}
        [] -> {:error, "No project directory found matching '#{project_name}'"}
        dirs -> {:ok, Enum.max_by(dirs, &file_mtime/1)}
      end
    else
      # Use the most recently modified project directory
      claude_projects
      |> Path.join("*")
      |> Path.wildcard()
      |> Enum.filter(&File.dir?/1)
      |> case do
        [] -> {:error, "No project directories found in #{claude_projects}"}
        dirs -> {:ok, Enum.max_by(dirs, &file_mtime/1)}
      end
    end
  end

  defp file_mtime(path) do
    case File.stat(path, time: :posix) do
      {:ok, %{mtime: mtime}} -> mtime
      _ -> 0
    end
  end
end

defmodule Autopsy.Census do
  @moduledoc "Baseline session metrics."

  def compute(sessions) do
    total_messages =
      sessions
      |> Enum.map(fn s -> length(s.messages) end)
      |> Enum.sum()

    total_compactions =
      sessions
      |> Enum.map(fn s -> length(s.compactions) end)
      |> Enum.sum()

    total_bytes =
      sessions
      |> Enum.map(fn s -> s.byte_size end)
      |> Enum.sum()

    timestamps =
      sessions
      |> Enum.flat_map(fn s ->
        Enum.map(s.messages, fn m -> m["timestamp"] end)
      end)
      |> Enum.reject(&is_nil/1)
      |> Enum.sort()

    date_range =
      case {List.first(timestamps), List.last(timestamps)} do
        {nil, _} -> %{"from" => nil, "to" => nil}
        {first, last} -> %{"from" => String.slice(first, 0, 10), "to" => String.slice(last, 0, 10)}
      end

    %{
      "session_count" => length(sessions),
      "compaction_count" => total_compactions,
      "total_messages" => total_messages,
      "total_bytes" => total_bytes,
      "date_range" => date_range
    }
  end
end

defmodule Autopsy.Corrections do
  @moduledoc """
  Detects correction pairs (user corrected Claude, Claude acknowledged)
  and frustration signals.
  """

  @acknowledgment_patterns [
    ~r/you'?re right/i,
    ~r/I apologize/i,
    ~r/I was wrong/i,
    ~r/good catch/i,
    ~r/my mistake/i,
    ~r/my apologies/i,
    ~r/I should have/i,
    ~r/you'?re correct/i,
    ~r/I missed that/i,
    ~r/sorry about that/i,
    ~r/I stand corrected/i
  ]

  @frustration_patterns [
    {~r/how many times/i, "how many times"},
    {~r/I('ve| have) told you/i, "I told you"},
    {~r/you forgot/i, "you forgot"},
    {~r/wrong file/i, "wrong file"},
    {~r/did you (actually|even) (look|read|check)/i, "did you actually look"},
    {~r/I (just|already) (said|told|asked|mentioned)/i, "I already said"},
    {~r/again\?!?$/im, "again?"},
    {~r/no[,.]? (that'?s|this is) (not |wrong)/i, "that's wrong"},
    {~r/please (re-?read|look at|check) /i, "please reread"},
    {~r/you (keep|continue) /i, "you keep"},
    {~r/stop (doing|adding|creating)/i, "stop doing"},
    {~r/I said (no|don'?t|not|never)/i, "I said no"}
  ]

  @user_flag_patterns [
    ~r/\bflag\b/i,
    ~r/flag for autopsy/i,
    ~r/\bflagged\b/i
  ]

  def find_corrections(session) do
    messages = session.messages
    compactions = session.compactions

    messages
    |> Enum.with_index()
    |> Enum.flat_map(fn {msg, idx} ->
      if msg["type"] == "assistant" and is_acknowledgment?(msg) do
        # Look back for the user correction
        user_msg = find_preceding_user_message(messages, idx)

        if user_msg do
          line = msg["_line"]

          [
            %{
              "session_id" => session.session_id,
              "line_number" => line,
              "user_message" => String.slice(Autopsy.Parser.message_text(user_msg), 0, 500),
              "assistant_acknowledgment" => String.slice(Autopsy.Parser.message_text(msg), 0, 500),
              "is_post_compaction" => Autopsy.Parser.post_compaction?(line, compactions),
              "compaction_index" => Autopsy.Parser.compaction_index(line, compactions)
            }
          ]
        else
          []
        end
      else
        []
      end
    end)
  end

  def find_frustration_signals(session) do
    messages = session.messages
    compactions = session.compactions

    messages
    |> Enum.filter(&(&1["type"] == "user"))
    |> Enum.flat_map(fn msg ->
      text = Autopsy.Parser.message_text(msg)
      line = msg["_line"]

      matched =
        @frustration_patterns
        |> Enum.filter(fn {pattern, _label} -> Regex.match?(pattern, text) end)
        |> Enum.map(fn {_pattern, label} -> label end)

      if matched != [] do
        [
          %{
            "session_id" => session.session_id,
            "line_number" => line,
            "user_message" => String.slice(text, 0, 500),
            "signals" => matched,
            "is_post_compaction" => Autopsy.Parser.post_compaction?(line, compactions),
            "compaction_index" => Autopsy.Parser.compaction_index(line, compactions)
          }
        ]
      else
        []
      end
    end)
  end

  def find_user_flags(session) do
    messages = session.messages
    compactions = session.compactions

    messages
    |> Enum.filter(&(&1["type"] == "user"))
    |> Enum.flat_map(fn msg ->
      text = Autopsy.Parser.message_text(msg)
      line = msg["_line"]

      # Skip tool_result messages (they contain content from system, not user)
      content = get_in(msg, ["message", "content"])

      is_tool_result =
        is_list(content) and Enum.any?(content, &(is_map(&1) and &1["type"] == "tool_result"))

      if not is_tool_result and Enum.any?(@user_flag_patterns, &Regex.match?(&1, text)) do
        [
          %{
            "session_id" => session.session_id,
            "line_number" => line,
            "user_message" => String.slice(text, 0, 500),
            "is_post_compaction" => Autopsy.Parser.post_compaction?(line, compactions),
            "compaction_index" => Autopsy.Parser.compaction_index(line, compactions)
          }
        ]
      else
        []
      end
    end)
  end

  defp is_acknowledgment?(msg) do
    text = Autopsy.Parser.message_text(msg)
    Enum.any?(@acknowledgment_patterns, &Regex.match?(&1, text))
  end

  defp find_preceding_user_message(messages, assistant_idx) do
    messages
    |> Enum.take(assistant_idx)
    |> Enum.reverse()
    |> Enum.find(fn msg ->
      msg["type"] == "user" and has_text_content?(msg)
    end)
  end

  defp has_text_content?(msg) do
    content = get_in(msg, ["message", "content"])

    case content do
      text when is_binary(text) -> String.trim(text) != ""
      items when is_list(items) ->
        Enum.any?(items, fn
          %{"type" => "text", "text" => text} -> String.trim(text) != ""
          _ -> false
        end)
      _ -> false
    end
  end
end

defmodule Autopsy.Regressions do
  @moduledoc """
  Detects capability regressions -- Claude deferred when it could have acted.
  Includes legitimacy filtering.
  """

  @deferral_patterns [
    {~r/you'?ll need to (manually|log in|check|open|visit)/i, "manual deferral"},
    {~r/I (can'?t|cannot|don'?t have) (access|permission|ability)/i, "capability denial"},
    {~r/you should (manually|log in|check|open)/i, "suggestion deferral"},
    {~r/I'?m (not able|unable) to/i, "inability claim"},
    {~r/you would need to/i, "need-to deferral"},
    {~r/I don'?t have (the ability|access|a way)/i, "access denial"},
    {~r/outside (of )?my (capabilities|ability|scope)/i, "scope denial"}
  ]

  @legitimate_deferral_patterns [
    ~r/browser/i,
    ~r/log ?in/i,
    ~r/2FA|two.?factor|MFA/i,
    ~r/CAPTCHA/i,
    ~r/physical/i,
    ~r/password/i,
    ~r/credential/i,
    ~r/GUI|graphical/i,
    ~r/click|tap|swipe/i,
    ~r/phone|SMS|call/i
  ]

  @meta_discussion_patterns [
    ~r/deferral pattern/i,
    ~r/capability regression/i,
    ~r/autopsy/i,
    ~r/analyzing.*session/i,
    ~r/session.*analy/i
  ]

  def find_deferrals(session) do
    messages = session.messages
    compactions = session.compactions

    messages
    |> Enum.filter(&(&1["type"] == "assistant"))
    |> Enum.flat_map(fn msg ->
      text = Autopsy.Parser.message_text(msg)
      line = msg["_line"]

      # Skip meta-discussion about deferrals
      is_meta = Enum.any?(@meta_discussion_patterns, &Regex.match?(&1, text))

      if is_meta do
        []
      else
        matched =
          @deferral_patterns
          |> Enum.filter(fn {pattern, _label} -> Regex.match?(pattern, text) end)
          |> Enum.map(fn {_pattern, label} -> label end)

        if matched != [] do
          is_legitimate =
            Enum.any?(@legitimate_deferral_patterns, &Regex.match?(&1, text))

          [
            %{
              "session_id" => session.session_id,
              "line_number" => line,
              "assistant_message" => String.slice(text, 0, 500),
              "deferral_types" => matched,
              "is_legitimate" => is_legitimate,
              "is_post_compaction" => Autopsy.Parser.post_compaction?(line, compactions),
              "compaction_index" => Autopsy.Parser.compaction_index(line, compactions)
            }
          ]
        else
          []
        end
      end
    end)
  end
end

defmodule Autopsy.Rules do
  @moduledoc """
  Banned pattern deep scan -- checks both conversation text and tool_use inputs.
  """

  @negation_window 60

  defstruct [:pattern, :label, :regex]

  def load_banned_words(opts) do
    words_from_arg = parse_words_string(Keyword.get(opts, :banned_words))
    words_from_file = load_words_file(Keyword.get(opts, :banned_file))
    words_from_default = load_default_banned_words(Keyword.get(opts, :script_dir))

    (words_from_default ++ words_from_file ++ words_from_arg)
    |> Enum.uniq_by(fn %{pattern: p} -> String.downcase(p) end)
  end

  def scan_session(session, banned_words) do
    if banned_words == [] do
      []
    else
      conversation_violations = scan_conversation(session, banned_words)
      tool_use_violations = scan_tool_uses(session, banned_words)
      conversation_violations ++ tool_use_violations
    end
  end

  defp scan_conversation(session, banned_words) do
    session.messages
    |> Enum.flat_map(fn msg ->
      text = Autopsy.Parser.message_text(msg)
      line = msg["_line"]
      role = msg["type"]
      scan_text(text, banned_words, session.session_id, line, role, "conversation")
    end)
  end

  defp scan_tool_uses(session, banned_words) do
    session.tool_uses
    |> Enum.flat_map(fn tool_use ->
      # Scan tool input values
      input_text = flatten_input(tool_use["input"])
      line = tool_use["line_number"]
      tool_name = tool_use["tool_name"]

      scan_text(
        input_text,
        banned_words,
        session.session_id,
        line,
        "tool_use",
        "tool_use:#{tool_name}"
      )
    end)
  end

  defp scan_text(text, banned_words, session_id, line, role, source) do
    banned_words
    |> Enum.flat_map(fn word ->
      if Regex.match?(word.regex, text) do
        is_negated = check_negation(text, word.pattern)

        [
          %{
            "session_id" => session_id,
            "line_number" => line,
            "pattern" => word.pattern,
            "label" => word.label,
            "source" => source,
            "role" => role,
            "is_negated" => is_negated,
            "context" => extract_context(text, word.regex)
          }
        ]
      else
        []
      end
    end)
  end

  defp check_negation(text, pattern) do
    negation_words = ~r/(avoid|never use|don'?t use|not |no |without |stop using)/i

    # Find the pattern position
    case Regex.run(~r/#{Regex.escape(pattern)}/i, text, return: :index) do
      [{start, _len}] ->
        window_start = max(0, start - @negation_window)
        window = String.slice(text, window_start, @negation_window)
        Regex.match?(negation_words, window)

      _ ->
        false
    end
  end

  defp extract_context(text, regex) do
    case Regex.run(regex, text, return: :index) do
      [{start, len}] ->
        ctx_start = max(0, start - 40)
        ctx_end = min(String.length(text), start + len + 40)
        String.slice(text, ctx_start, ctx_end - ctx_start)

      _ ->
        ""
    end
  end

  defp flatten_input(nil), do: ""
  defp flatten_input(input) when is_binary(input), do: input

  defp flatten_input(input) when is_map(input) do
    input
    |> Map.values()
    |> Enum.map(&flatten_input/1)
    |> Enum.join("\n")
  end

  defp flatten_input(input) when is_list(input) do
    input
    |> Enum.map(&flatten_input/1)
    |> Enum.join("\n")
  end

  defp flatten_input(input), do: to_string(input)

  defp parse_words_string(nil), do: []

  defp parse_words_string(words_string) do
    words_string
    |> String.split(",")
    |> Enum.map(&String.trim/1)
    |> Enum.reject(&(&1 == ""))
    |> Enum.map(fn word ->
      %__MODULE__{
        pattern: word,
        label: "custom",
        regex: Regex.compile!(Regex.escape(word), "i")
      }
    end)
  end

  defp load_words_file(nil), do: []

  defp load_words_file(path) do
    expanded = Path.expand(path)

    if File.exists?(expanded) do
      expanded
      |> File.read!()
      |> parse_words_content()
    else
      IO.puts(:stderr, "Warning: banned words file not found: #{path}")
      []
    end
  end

  defp load_default_banned_words(nil), do: []

  defp load_default_banned_words(script_dir) do
    default_file = Path.join(script_dir, "banned-words.txt")

    if File.exists?(default_file) do
      default_file
      |> File.read!()
      |> parse_words_content()
    else
      []
    end
  end

  defp parse_words_content(content) do
    content
    |> String.split("\n")
    |> Enum.map(&String.trim/1)
    |> Enum.reject(&(&1 == "" or String.starts_with?(&1, "#")))
    |> Enum.map(fn line ->
      case String.split(line, "|", parts: 2) do
        [word, label] ->
          %__MODULE__{
            pattern: String.trim(word),
            label: String.trim(label),
            regex: Regex.compile!(Regex.escape(String.trim(word)), "i")
          }

        [word] ->
          %__MODULE__{
            pattern: String.trim(word),
            label: "custom",
            regex: Regex.compile!(Regex.escape(String.trim(word)), "i")
          }
      end
    end)
  end
end

defmodule Autopsy.CLI do
  @moduledoc "Argument parsing, orchestration, and JSON output."

  def main(args) do
    opts = parse_args(args)

    if opts[:help] do
      print_help()
    else
      run(opts)
    end
  end

  defp run(opts) do
    days = Keyword.get(opts, :days, 7)
    min_compactions = Keyword.get(opts, :min_compactions, 0)
    project = Keyword.get(opts, :project)
    output = Keyword.get(opts, :output)
    script_dir = Keyword.get(opts, :script_dir)

    # Resolve project directory
    project_dir =
      case Autopsy.Parser.resolve_project_dir(project) do
        {:ok, dir} ->
          IO.puts(:stderr, "Project directory: #{dir}")
          dir

        {:error, msg} ->
          IO.puts(:stderr, "Error: #{msg}")
          System.halt(1)
      end

    # Find session files
    session_files = Autopsy.Parser.find_session_files(project_dir, days: days)

    if session_files == [] do
      IO.puts(:stderr, "No session files found in the last #{days} days")
      System.halt(1)
    end

    IO.puts(:stderr, "Found #{length(session_files)} session files")

    # Load banned words
    banned_words = Autopsy.Rules.load_banned_words(opts ++ [script_dir: script_dir])

    if banned_words != [] do
      IO.puts(:stderr, "Loaded #{length(banned_words)} banned word patterns")
    end

    # Parse all sessions
    sessions =
      session_files
      |> Enum.map(fn path ->
        IO.puts(:stderr, "Parsing: #{Path.basename(path)}")
        lines = Autopsy.Parser.parse_file(path)
        messages = Autopsy.Parser.extract_messages(lines)
        compactions = Autopsy.Parser.extract_compactions(lines)
        tool_uses = Autopsy.Parser.extract_tool_uses(messages)
        byte_size = File.stat!(path).size

        session_id =
          path |> Path.basename() |> String.replace(".jsonl", "")

        %{
          session_id: session_id,
          path: path,
          messages: messages,
          compactions: compactions,
          tool_uses: tool_uses,
          byte_size: byte_size,
          line_count: length(lines)
        }
      end)
      |> Enum.filter(fn s ->
        length(s.compactions) >= min_compactions
      end)

    if sessions == [] do
      IO.puts(:stderr, "No sessions match filters (min_compactions: #{min_compactions})")
      System.halt(1)
    end

    IO.puts(:stderr, "Analyzing #{length(sessions)} sessions...")

    # Run analyses
    census = Autopsy.Census.compute(sessions)

    corrections =
      sessions |> Enum.flat_map(&Autopsy.Corrections.find_corrections/1)

    frustration_signals =
      sessions |> Enum.flat_map(&Autopsy.Corrections.find_frustration_signals/1)

    user_flags =
      sessions |> Enum.flat_map(&Autopsy.Corrections.find_user_flags/1)

    deferrals =
      sessions |> Enum.flat_map(&Autopsy.Regressions.find_deferrals/1)

    banned_violations =
      sessions |> Enum.flat_map(&Autopsy.Rules.scan_session(&1, banned_words))

    # Build output
    findings = %{
      "generated_at" => DateTime.utc_now() |> DateTime.to_iso8601(),
      "parameters" => %{
        "days" => days,
        "min_compactions" => min_compactions,
        "project_dir" => project_dir,
        "banned_word_count" => length(banned_words)
      },
      "census" => census,
      "corrections" => corrections,
      "frustration_signals" => frustration_signals,
      "user_flags" => user_flags,
      "deferrals" => deferrals,
      "banned_pattern_violations" => banned_violations
    }

    json = Jason.encode!(findings, pretty: true)

    case output do
      nil ->
        IO.puts(json)

      path ->
        File.write!(Path.expand(path), json)
        IO.puts(:stderr, "Findings written to #{path}")
    end

    # Print summary to stderr
    IO.puts(:stderr, "")
    IO.puts(:stderr, "=== Summary ===")
    IO.puts(:stderr, "Sessions: #{census["session_count"]}")
    IO.puts(:stderr, "Compactions: #{census["compaction_count"]}")
    IO.puts(:stderr, "Messages: #{census["total_messages"]}")
    IO.puts(:stderr, "Corrections: #{length(corrections)}")
    IO.puts(:stderr, "Frustration signals: #{length(frustration_signals)}")
    IO.puts(:stderr, "User flags: #{length(user_flags)}")
    IO.puts(:stderr, "Deferrals: #{length(deferrals)}")
    IO.puts(:stderr, "Banned violations: #{length(banned_violations)}")
  end

  defp parse_args(args) do
    parse_args(args, [])
  end

  defp parse_args([], acc), do: acc

  defp parse_args(["--help" | _rest], acc), do: [{:help, true} | acc]
  defp parse_args(["-h" | _rest], acc), do: [{:help, true} | acc]

  defp parse_args(["--days", days | rest], acc) do
    parse_args(rest, [{:days, String.to_integer(days)} | acc])
  end

  defp parse_args(["--min-compactions", n | rest], acc) do
    parse_args(rest, [{:min_compactions, String.to_integer(n)} | acc])
  end

  defp parse_args(["--project", name | rest], acc) do
    parse_args(rest, [{:project, name} | acc])
  end

  defp parse_args(["--banned-words", words | rest], acc) do
    parse_args(rest, [{:banned_words, words} | acc])
  end

  defp parse_args(["--banned-file", path | rest], acc) do
    parse_args(rest, [{:banned_file, path} | acc])
  end

  defp parse_args(["-o", path | rest], acc) do
    parse_args(rest, [{:output, path} | acc])
  end

  defp parse_args(["--output", path | rest], acc) do
    parse_args(rest, [{:output, path} | acc])
  end

  defp parse_args(["--script-dir", dir | rest], acc) do
    parse_args(rest, [{:script_dir, dir} | acc])
  end

  defp parse_args([_unknown | rest], acc) do
    parse_args(rest, acc)
  end

  defp print_help do
    IO.puts("""
    autopsy.exs - Pre-process Claude Code session files for autopsy analysis

    Usage:
      elixir autopsy.exs [options]

    Options:
      --days N              Analyze sessions from the last N days (default: 7)
      --min-compactions N   Only include sessions with at least N compactions (default: 0)
      --project NAME        Filter by project name (matches directory name)
      --banned-words WORDS  Comma-separated banned words to scan for
      --banned-file PATH    Path to banned words file (one per line, label after |)
      -o, --output PATH     Write JSON findings to file instead of stdout
      -h, --help            Show this help

    The script automatically loads banned-words.txt from its own directory.
    Additional words from --banned-words and --banned-file are merged in.

    Output:
      JSON findings to stdout (or file with -o). Progress/status to stderr.

    Examples:
      elixir autopsy.exs --days 7
      elixir autopsy.exs --days 7 --min-compactions 1
      elixir autopsy.exs --days 14 --project Intent -o findings.json
      elixir autopsy.exs --days 7 --banned-words "delve,unfortunately"
    """)
  end
end

# Determine script directory for finding default banned-words.txt
script_dir = __DIR__

# Inject script_dir into args processing
args = System.argv()

# If no explicit script_dir arg, prepend it
Autopsy.CLI.main(["--script-dir", script_dir | args])
