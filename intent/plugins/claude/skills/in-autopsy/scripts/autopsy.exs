#!/usr/bin/env elixir
# autopsy.exs - reads Claude Code session transcripts as evidence for /in-autopsy
# Copyright (c) 2026 Matthew Sinclair
# Licensed under the MIT License (see LICENSE file)
#
# The script extracts facts from a session: who wrote each entry, which
# commands ran, what they printed, and what the seat wrote. It keeps no rule.
# Each finding names where its rule lives, and the procedure in SKILL.md reads
# the rule there.
#
# Usage: elixir autopsy.exs --help

defmodule Autopsy.Transcript do
  @moduledoc """
  Turns one transcript line into a typed entry. It is the one reader of the
  wrapper markers: it names the source of each `user` entry and the session's
  seat. A transcript is input from outside, so a field of the wrong type is
  read as absent. Pure.
  """

  # The harness records who sent a `user` entry in its `origin` field. The
  # markers below are the fallback for a transcript that carries no `origin`.
  @compaction_markers ~r/This session is being continued|<compacted-conversation/i
  @subagent_markers ~r/<agent-message|\[Subagent hand-back\]/
  @seat_markers ~r/<cross-session-message|from-name=/
  @task_markers ~r/<task-notification>/
  @skill_markers ~r/Base directory for this skill|<command-name>|<command-message>/
  @local_command_markers ~r/<local-command-/

  @doc "Reads one line. A blank line is `:blank`, and a line it cannot read is `{:unread, reason}`."
  def read_line(text, line) do
    case String.trim(text) do
      "" ->
        :blank

      trimmed ->
        case JSON.decode(trimmed) do
          {:ok, %{} = obj} -> {:ok, entry(obj, line)}
          {:ok, _other} -> {:unread, "the line is JSON, and it is not an object"}
          {:error, reason} -> {:unread, "the line is not JSON: #{inspect(reason)}"}
        end
    end
  end

  defp entry(%{"type" => "assistant"} = obj, line) do
    blocks = blocks(content(obj))

    calls =
      for %{"type" => "tool_use", "id" => id, "name" => tool} = block <- blocks,
          is_binary(id) and is_binary(tool) do
        %{id: id, tool: tool, input: if(is_map(block["input"]), do: block["input"], else: %{})}
      end

    Map.merge(base(obj, line), %{kind: :assistant, texts: texts(blocks), calls: calls})
  end

  defp entry(%{"type" => "user"} = obj, line) do
    blocks = blocks(content(obj))
    text = blocks |> texts() |> Enum.join("\n")

    results =
      for %{"type" => "tool_result", "tool_use_id" => id} = block <- blocks, is_binary(id) do
        %{
          id: id,
          text: block["content"] |> blocks() |> texts() |> Enum.join("\n"),
          error: block["is_error"] == true
        }
      end

    {source, from} = source(obj, text, results != [])

    Map.merge(base(obj, line), %{
      kind: :user,
      source: source,
      from: from,
      text: text,
      results: results
    })
  end

  defp entry(
         %{"type" => "attachment", "attachment" => %{"type" => "queued_command"} = queued} = obj,
         line
       ) do
    text = queued["prompt"] |> blocks() |> texts() |> Enum.join("\n")
    {source, from} = source(queued, text, false)
    time = if is_binary(queued["timestamp"]), do: queued["timestamp"], else: obj["timestamp"]

    obj
    |> Map.put("timestamp", time)
    |> base(line)
    |> Map.merge(%{kind: :queued, source: source, from: from, text: text})
  end

  defp entry(%{"type" => "system", "subtype" => "compact_boundary"} = obj, line),
    do: Map.put(base(obj, line), :kind, :compact_boundary)

  defp entry(%{"type" => "agent-name", "agentName" => name}, line) when is_binary(name),
    do: %{kind: :agent_name, line: line, name: name}

  defp entry(_obj, line), do: %{kind: :other, line: line}

  defp base(obj, line),
    do: %{line: line, time: string(obj["timestamp"]), cwd: string(obj["cwd"])}

  defp string(value) when is_binary(value), do: value
  defp string(_value), do: nil

  defp content(%{"message" => %{"content" => content}}), do: content
  defp content(_obj), do: nil

  defp blocks(content) when is_binary(content), do: [%{"type" => "text", "text" => content}]
  defp blocks(content) when is_list(content), do: Enum.filter(content, &is_map/1)
  defp blocks(_content), do: []

  defp texts(blocks),
    do: for(%{"type" => "text", "text" => text} <- blocks, is_binary(text), do: text)

  @doc """
  Names who wrote a `user` entry, as `{source, from}`. The sources are
  `person`, `seat`, `subagent`, `task_notification`, `compaction_summary`,
  `skill_content`, `local_command`, `system_reminder`, `tool_result`, and
  `empty`. Only `person` is text that a person typed.
  """
  def source(obj, text, has_results) do
    origin = if is_map(obj["origin"]), do: obj["origin"], else: %{}

    cond do
      has_results ->
        {"tool_result", nil}

      obj["isCompactSummary"] == true ->
        {"compaction_summary", nil}

      origin["kind"] == "task-notification" ->
        {"task_notification", nil}

      origin["kind"] == "peer" and is_binary(origin["senderTaskId"]) ->
        {"subagent", string(origin["from"])}

      origin["kind"] == "peer" ->
        {"seat", string(origin["name"]) || string(origin["from"])}

      true ->
        source_by_marker(obj, text)
    end
  end

  defp source_by_marker(obj, text) do
    cond do
      Regex.match?(@compaction_markers, text) ->
        {"compaction_summary", nil}

      Regex.match?(@subagent_markers, text) ->
        {"subagent", nil}

      Regex.match?(@seat_markers, text) ->
        {"seat", from_name(text)}

      Regex.match?(@task_markers, text) ->
        {"task_notification", nil}

      Regex.match?(@skill_markers, text) ->
        {"skill_content", nil}

      Regex.match?(@local_command_markers, text) ->
        {"local_command", nil}

      String.starts_with?(String.trim_leading(text), "<system-reminder>") ->
        {"system_reminder", nil}

      obj["isMeta"] == true ->
        {"system_reminder", nil}

      String.trim(text) == "" ->
        {"empty", nil}

      true ->
        {"person", nil}
    end
  end

  defp from_name(text) do
    case Regex.run(~r/from-name="([^"]*)"/, text) do
      [_, name] -> name
      nil -> nil
    end
  end

  @doc "The seat's moniker from a session name, eg `dc` from `intent-dc`."
  def seat(nil), do: nil
  def seat(session_name), do: session_name |> String.split("-") |> List.last()
end

defmodule Autopsy.Turns do
  @moduledoc """
  Groups a session's entries into turns and joins each tool result to its
  tool call. A turn starts when someone speaks to the seat: a person, a
  seat, or a subagent. A task notification or a reminder starts no turn.
  Pure.
  """

  @speakers ~w(person seat subagent skill_content)

  @doc """
  Returns the session's name, its compaction boundaries, and three ordered
  lists: `acts` (tool calls with their results), `says` (the assistant's
  text), and `hears` (the `user` entries that are not tool results).
  """
  def build(entries) do
    results =
      for %{kind: :user, results: results, line: line} <- entries, result <- results, into: %{} do
        {result.id, Map.put(result, :line, line)}
      end

    start = %{turn: 0, ci: 0, acts: [], says: [], hears: [], compactions: [], name: nil}
    state = Enum.reduce(entries, start, &step(&1, &2, results))

    %{
      name: state.name,
      compactions: Enum.reverse(state.compactions),
      acts: Enum.reverse(state.acts),
      says: Enum.reverse(state.says),
      hears: Enum.reverse(state.hears)
    }
  end

  defp step(%{kind: :agent_name, name: name}, state, _results), do: %{state | name: name}

  defp step(%{kind: :compact_boundary} = entry, state, _results) do
    boundary = %{line: entry.line, time: entry.time}
    %{state | ci: state.ci + 1, compactions: [boundary | state.compactions]}
  end

  defp step(%{kind: :user, source: "tool_result"}, state, _results), do: state

  defp step(%{kind: :user, source: source} = entry, state, _results) do
    state = if source in @speakers, do: %{state | turn: state.turn + 1}, else: state
    %{state | hears: [heard(entry, state) | state.hears]}
  end

  defp step(%{kind: :queued} = entry, state, _results),
    do: %{state | hears: [heard(entry, state) | state.hears]}

  defp step(%{kind: :assistant} = entry, state, results) do
    says =
      for text <- entry.texts do
        %{line: entry.line, time: entry.time, turn: state.turn, ci: state.ci, text: text}
      end

    acts =
      for call <- entry.calls do
        result = Map.get(results, call.id)

        %{
          line: entry.line,
          time: entry.time,
          cwd: entry.cwd,
          turn: state.turn,
          ci: state.ci,
          tool: call.tool,
          input: call.input,
          result: result && result.text,
          result_line: result && result.line,
          error: result != nil and result.error
        }
      end

    %{state | says: Enum.reverse(says) ++ state.says, acts: Enum.reverse(acts) ++ state.acts}
  end

  defp step(_entry, state, _results), do: state

  defp heard(entry, state) do
    %{
      line: entry.line,
      time: entry.time,
      turn: state.turn,
      ci: state.ci,
      source: entry.source,
      from: entry.from,
      text: entry.text
    }
  end
end

defmodule Autopsy.Shell do
  @moduledoc """
  Splits a shell command into segments of words, and resolves the variables
  that the same command assigns. It reads a command's shape. It is not a
  shell: a subshell, a function, or a nested quote can defeat it, and then a
  word is marked as not resolved. Pure.
  """

  @name "[A-Za-z_][A-Za-z0-9_]*"
  # `<<TAG` opens a heredoc. `<<<` is a here-string, which has no body lines.
  @heredoc ~r/(?<!<)<<(?!<)-?\s*['"]?(#{@name})['"]?/
  # `$(cat <<'TAG'` is the usual way to give a commit its message.
  @cat_heredoc ~r/\$\(\s*cat\s+<<-?\s*['"]?(#{@name})['"]?\s*\z/
  @variable ~r/\$\{?(#{@name})\}?/

  @redirect_mark "\u0001"

  @doc """
  The mark the scanner puts before a `>` or `<` that is outside quotes. A
  quoted `>` is text, and only a marked one is a redirect.
  """
  def redirect_mark, do: @redirect_mark

  @doc "The command's segments, each a list of words with their quotes removed."
  def segments(command) when is_binary(command),
    do: command |> heredocs() |> scan(:n, 0, nil, [], [])

  def segments(_command), do: []

  # A heredoc's body is data, and its lines are not commands. The body of
  # `$(cat <<TAG` becomes the word it stands for.
  defp heredocs(command),
    do: command |> String.split("\n") |> heredocs([]) |> Enum.join("\n")

  defp heredocs([], kept), do: Enum.reverse(kept)

  defp heredocs([line | rest], kept) do
    case Regex.run(@cat_heredoc, line, return: :index) do
      [{at, _len}, {tag_at, tag_len}] ->
        {body, rest} = until_tag(rest, binary_part(line, tag_at, tag_len))
        {tail, rest} = after_close(rest)
        heredocs(rest, [binary_part(line, 0, at) <> escape(Enum.join(body, "\n")) <> tail | kept])

      nil ->
        case Regex.run(@heredoc, line) do
          [_, tag] -> heredocs(rest |> until_tag(tag) |> elem(1), [line | kept])
          nil -> heredocs(rest, [line | kept])
        end
    end
  end

  defp until_tag(lines, tag) do
    {body, rest} = Enum.split_while(lines, &(String.trim(&1) != tag))
    {body, Enum.drop(rest, 1)}
  end

  # The line after the tag closes `$(`. Its `)` goes, and its tail stays.
  defp after_close([line | rest]) do
    case Regex.run(~r/\A\s*\)(.*)\z/s, line) do
      [_, tail] -> {tail, rest}
      nil -> {"", [line | rest]}
    end
  end

  defp after_close([]), do: {"", []}

  # A backslash before each character makes the scanner take it as text.
  defp escape(text), do: Regex.replace(~r/[^A-Za-z0-9]/u, text, "\\\\\\0")

  # scan(rest, quote state, paren depth, open word, words of the segment, segments)
  defp scan(<<>>, _q, _d, word, words, segs),
    do: words |> push(word) |> close(segs) |> Enum.reverse()

  defp scan(<<"\\", c::utf8, rest::binary>>, q, d, word, words, segs) when q != :sq,
    do: scan(rest, q, d, add(word, <<c::utf8>>), words, segs)

  defp scan(<<"'", rest::binary>>, :n, d, word, words, segs),
    do: scan(rest, :sq, d, add(word, ""), words, segs)

  defp scan(<<"'", rest::binary>>, :sq, d, word, words, segs),
    do: scan(rest, :n, d, word, words, segs)

  defp scan(<<"\"", rest::binary>>, :n, d, word, words, segs),
    do: scan(rest, :dq, d, add(word, ""), words, segs)

  defp scan(<<"\"", rest::binary>>, :dq, d, word, words, segs),
    do: scan(rest, :n, d, word, words, segs)

  defp scan(<<c::utf8, rest::binary>>, q, d, word, words, segs) when q != :n,
    do: scan(rest, q, d, add(word, <<c::utf8>>), words, segs)

  defp scan(<<"$(", rest::binary>>, :n, d, word, words, segs),
    do: scan(rest, :n, d + 1, add(word, "$("), words, segs)

  defp scan(<<"(", rest::binary>>, :n, d, word, words, segs) do
    if d > 0 or (is_binary(word) and String.ends_with?(word, "=")) do
      scan(rest, :n, d + 1, add(word, "("), words, segs)
    else
      scan(rest, :n, d, word, words, segs)
    end
  end

  defp scan(<<")", rest::binary>>, :n, d, word, words, segs) when d > 0,
    do: scan(rest, :n, d - 1, add(word, ")"), words, segs)

  defp scan(<<")", rest::binary>>, :n, 0, word, words, segs),
    do: scan(rest, :n, 0, word, words, segs)

  defp scan(<<c::utf8, rest::binary>>, :n, d, word, words, segs) when d > 0,
    do: scan(rest, :n, d, add(word, <<c::utf8>>), words, segs)

  defp scan(<<"&&", rest::binary>>, :n, 0, word, words, segs),
    do: scan(rest, :n, 0, nil, [], words |> push(word) |> close(segs))

  defp scan(<<"||", rest::binary>>, :n, 0, word, words, segs),
    do: scan(rest, :n, 0, nil, [], words |> push(word) |> close(segs))

  defp scan(<<"&", rest::binary>>, :n, 0, word, words, segs) do
    # `2>&1` and `>&2` are redirects. A lone `&` ends the segment.
    if is_binary(word) and String.ends_with?(word, ">") do
      scan(rest, :n, 0, add(word, "&"), words, segs)
    else
      scan(rest, :n, 0, nil, [], words |> push(word) |> close(segs))
    end
  end

  defp scan(<<c, rest::binary>>, :n, 0, word, words, segs) when c in [?>, ?<],
    do: scan(rest, :n, 0, add(word, @redirect_mark <> <<c>>), words, segs)

  defp scan(<<c, rest::binary>>, :n, 0, word, words, segs) when c in [?;, ?|, ?\n],
    do: scan(rest, :n, 0, nil, [], words |> push(word) |> close(segs))

  defp scan(<<c, rest::binary>>, :n, 0, word, words, segs) when c in [?\s, ?\t, ?\r],
    do: scan(rest, :n, 0, nil, push(words, word), segs)

  defp scan(<<c::utf8, rest::binary>>, :n, 0, word, words, segs),
    do: scan(rest, :n, 0, add(word, <<c::utf8>>), words, segs)

  # Bytes that are not valid UTF-8 are kept as they are.
  defp scan(<<c, rest::binary>>, q, d, word, words, segs),
    do: scan(rest, q, d, add(word, <<c>>), words, segs)

  defp add(nil, text), do: text
  defp add(word, text), do: word <> text

  defp push(words, nil), do: words
  defp push(words, word), do: [word | words]

  defp close([], segs), do: segs
  defp close(words, segs), do: [Enum.reverse(words) | segs]

  @doc "Reads `NAME=value` or `NAME=(a b)` as `{name, [values]}`. Any other word is nil."
  def assignment(word) do
    case Regex.run(~r/\A(#{@name})=\((.*)\)\z/s, word) do
      [_, name, inner] ->
        {name, String.split(inner)}

      nil ->
        case Regex.run(~r/\A(#{@name})=(.*)\z/s, word) do
          [_, name, value] -> {name, [value]}
          nil -> nil
        end
    end
  end

  @doc """
  Expands a word with the variables the command assigned. It returns
  `{:ok, words}`, or `{:error, :unresolved}` when the word needs a value
  that the command's text does not hold.
  """
  def expand(word, env) do
    case Regex.run(~r/\A\$\{?(#{@name})(?:\[@\])?\}?\z/, word) do
      [_, name] -> known(Map.get(env, name, :unresolved))
      nil -> expand_inside(word, env)
    end
  end

  defp known(:unresolved), do: {:error, :unresolved}
  defp known(values), do: {:ok, values}

  defp expand_inside(word, env) do
    names = for [_, name] <- Regex.scan(@variable, word), do: name

    cond do
      String.contains?(word, ["$(", "`"]) ->
        {:error, :unresolved}

      Enum.all?(names, &match?([_one], Map.get(env, &1))) ->
        {:ok,
         [Regex.replace(@variable, word, fn _all, name -> env |> Map.fetch!(name) |> hd() end)]}

      true ->
        {:error, :unresolved}
    end
  end
end

defmodule Autopsy.Events do
  @moduledoc """
  Reads each tool call and its result as protocol events: an `intent` verb,
  a git commit, add, ref, or worktree act, a heavy run, a file write, a sent
  message, and what a verb printed. It reads an `intent` call by its shape,
  so a new verb needs no change here. Each event has a `seq`, its place in
  the session, because every event of one command has the same line. Pure.
  """

  alias Autopsy.Shell

  @commit_value_flags ~w(-F -C -c --file --author --date --reuse-message --reedit-message --fixup --squash --cleanup --trailer -t --template --pathspec-from-file)
  @worktree_value_flags ~w(-b -B --reason --expire)
  @wb_text_verbs ~w(ask announce add edit decide pickup)
  # `REFUSED` in capitals is a guard's verdict. The word in lower case is prose.
  @error_line ~r/^\s*(?i:error|refused|fatal):.*$|^.*\bREFUSED\b.*$/m
  @printed_line ~r/^\s*(wrote|removed|re-rendered):\s+(\S+)/m
  @commit_answer ~r/^\[(?:detached HEAD|[^\]\s]+)(?: \([^)]*\))? ([0-9a-f]{7,40})\]/m
  @exit_code ~r/\AExit code (\d+)/
  @redirect_to ~r/\A\d?(?:\x01>){1,2}([^&\x01].*)\z/s

  @doc "The events of a session's acts, in order. `config` gives the heavy-run patterns and the checkout."
  def from_acts(acts, config) do
    acts
    |> Enum.flat_map(&from_act(&1, config))
    |> Enum.with_index(fn event, seq -> Map.put(event, :seq, seq) end)
  end

  @doc "The shell command of a Bash act. It is empty for any other act."
  def command(%{tool: "Bash", input: %{"command" => command}}) when is_binary(command),
    do: command

  def command(_act), do: ""

  defp from_act(%{tool: "Bash"} = act, config) do
    {events, _env, _cwd} =
      act
      |> command()
      |> Shell.segments()
      |> Enum.reduce({[], %{}, act.cwd}, fn words, {events, env, cwd} ->
        {words, env} = take_assignments(words, env)
        {new, cwd} = segment(words, env, cwd, act, config)
        {events ++ new, env, cwd}
      end)

    events = with_commit_shas(events, text(act.result))
    events ++ output(act, events)
  end

  defp from_act(%{tool: "SendMessage"} = act, _config) do
    fields = %{to: text(act.input["to"]), written: text(act.input["message"]), where: "message"}
    [event(act, "message", fields)]
  end

  defp from_act(%{tool: tool} = act, config) when tool in ["Edit", "Write", "NotebookEdit"] do
    path = text(act.input["file_path"] || act.input["notebook_path"])
    written = text(act.input["content"] || act.input["new_string"] || act.input["new_source"])
    fields = %{path: relative(path, act.cwd, config), via: tool, written: written, where: "file"}
    [event(act, "file_write", fields)]
  end

  # An `intent` verb through MCP. Its input is a map, so each field is a flag.
  defp from_act(%{tool: "mcp__intent__intent_" <> name} = act, _config) do
    {noun, verb, args} = name |> String.split("_") |> noun_verb()
    flags = for {key, value} <- act.input, scalar?(value), into: %{}, do: {key, value}
    texts = for {_key, value} <- Enum.sort(act.input), is_binary(value), do: value
    intent = intent_event(act, {noun, verb, args}, flags, texts, act.tool, act.cwd, "mcp")
    [intent | output(act, [intent])]
  end

  defp from_act(_act, _config), do: []

  defp scalar?(value), do: is_binary(value) or is_boolean(value) or is_number(value)

  defp take_assignments([word | rest] = words, env) do
    case Shell.assignment(word) do
      {name, values} -> take_assignments(rest, Map.put(env, name, expand_all(values, env)))
      nil -> {words, env}
    end
  end

  defp take_assignments([], env), do: {[], env}

  defp expand_all(values, env) do
    Enum.reduce_while(values, [], fn value, acc ->
      case Shell.expand(value, env) do
        {:ok, list} -> {:cont, acc ++ list}
        {:error, :unresolved} -> {:halt, :unresolved}
      end
    end)
  end

  defp segment([], _env, cwd, _act, _config), do: {[], cwd}

  # After a `cd` to a directory the command's text does not hold, the
  # directory is not known, and it is nil for the segments that follow.
  defp segment(["cd", path | _], env, cwd, _act, _config) do
    case Shell.expand(path, env) do
      {:ok, [dir]} -> {[], join(cwd, dir)}
      _other -> {[], nil}
    end
  end

  defp segment(words, env, cwd, act, config) do
    {command, redirects} = split_redirects(words)

    events =
      cond do
        help?(command) ->
          []

        match?(["intent" | _], command) ->
          [shell_intent(act, tl(command), cwd)]

        match?(["git" | _], command) ->
          git(tl(command), env, cwd, act, config)

        match?(["tee" | _], command) ->
          writes(Enum.reject(tl(command), &flag?/1), cwd, act, config)

        heavy?(Enum.join(command, " "), config) ->
          [event(act, "heavy_run", %{cwd: cwd, command: Enum.join(command, " ")})]

        true ->
          []
      end

    {events ++ writes(redirects, cwd, act, config), cwd}
  end

  # A call for help runs nothing. `-h` means help only to `intent` and `git`.
  defp help?(command),
    do: "--help" in command or (List.first(command) in ["intent", "git"] and "-h" in command)

  # A redirect's target is a file the command writes. `2>&1` and `/dev/null` are not files.
  defp split_redirects(words) do
    mark = Shell.redirect_mark()
    to = [mark <> ">", mark <> ">" <> mark <> ">"]

    {command, targets, _pending} =
      Enum.reduce(words, {[], [], false}, fn word, {command, targets, pending} ->
        cond do
          pending ->
            {command, [word | targets], false}

          word in to ->
            {command, targets, true}

          Regex.match?(@redirect_to, word) ->
            {command, [hd(tl(Regex.run(@redirect_to, word))) | targets], false}

          String.contains?(word, mark) ->
            {command, targets, false}

          true ->
            {[word | command], targets, false}
        end
      end)

    files = targets |> Enum.reverse() |> Enum.reject(&String.starts_with?(&1, "/dev/"))
    {Enum.reverse(command), files}
  end

  defp writes(targets, cwd, act, config) do
    for target <- targets do
      event(act, "file_write", %{cwd: cwd, path: relative(target, cwd, config), via: "shell"})
    end
  end

  defp heavy?(_command, %{heavy: nil}), do: false
  defp heavy?(command, %{heavy: patterns}), do: Enum.any?(patterns, &Regex.match?(&1, command))

  # `intent <noun> <verb> <args>`, with its flags. A flag takes the next word
  # as its value unless that word is a flag too, so a word after a flag that
  # takes no value is read as that flag's value. `raw` keeps the exact words.
  defp shell_intent(act, words, cwd) do
    {positional, flags} = split_flags(words, [], %{})
    {_noun, _verb, args} = parts = noun_verb(positional)
    texts = args ++ for({"focus", value} <- flags, is_binary(value), do: value)
    intent_event(act, parts, flags, texts, Enum.join(["intent" | words], " "), cwd, "shell")
  end

  defp intent_event(act, {noun, verb, args}, flags, texts, raw, cwd, via) do
    written = if noun == "wb" and verb in @wb_text_verbs, do: Enum.join(texts, "\n")

    event(act, "intent", %{
      cwd: cwd,
      via: via,
      noun: noun,
      verb: verb,
      args: args,
      flags: flags,
      raw: raw,
      written: written,
      where: "wb"
    })
  end

  defp noun_verb([]), do: {nil, nil, []}
  defp noun_verb([noun]), do: {noun, nil, []}
  defp noun_verb([noun, verb | args]), do: {noun, verb, args}

  defp split_flags([], positional, flags), do: {Enum.reverse(positional), flags}

  defp split_flags(["--" <> flag | rest], positional, flags) do
    case {String.split(flag, "=", parts: 2), rest} do
      {[name, value], _rest} ->
        split_flags(rest, positional, Map.put(flags, name, value))

      {[name], [next | after_next]} ->
        if flag?(next),
          do: split_flags(rest, positional, Map.put(flags, name, true)),
          else: split_flags(after_next, positional, Map.put(flags, name, next))

      {[name], []} ->
        split_flags([], positional, Map.put(flags, name, true))
    end
  end

  defp split_flags([word | rest], positional, flags),
    do: split_flags(rest, [word | positional], flags)

  defp flag?(word), do: String.starts_with?(word, "-")

  defp git(words, env, cwd, act, config) do
    {cwd, words} = git_globals(words, cwd)

    case words do
      ["commit" | rest] ->
        [commit(rest, env, cwd, act, config)]

      ["add" | rest] ->
        [event(act, "git_add", %{cwd: cwd, all: Enum.any?(rest, &(&1 in ~w(-A --all .)))})]

      ["update-ref" | rest] ->
        [update_ref(rest, cwd, act)]

      ["worktree", sub | rest] when sub in ~w(add remove) ->
        [worktree(sub, rest, cwd, act)]

      _other ->
        []
    end
  end

  defp git_globals(["-C", dir | rest], cwd), do: git_globals(rest, join(cwd, dir))
  defp git_globals(["-c", _setting | rest], cwd), do: git_globals(rest, cwd)
  defp git_globals(words, cwd), do: {cwd, words}

  defp commit(words, env, cwd, act, config) do
    {paths, message, flags} = commit_words(words, [], nil, [])

    {resolved, unresolved} =
      Enum.reduce(paths, {[], false}, fn path, {resolved, unresolved} ->
        case Shell.expand(path, env) do
          {:ok, list} -> {resolved ++ Enum.map(list, &relative(&1, cwd, config)), unresolved}
          {:error, :unresolved} -> {resolved, true}
        end
      end)

    event(act, "git_commit", %{
      cwd: cwd,
      only: Enum.any?(flags, &(&1 == "--only" or short?(&1, "o"))),
      all: Enum.any?(flags, &(&1 == "--all" or short?(&1, "a"))),
      no_verify: Enum.any?(flags, &(&1 == "--no-verify" or short?(&1, "n"))),
      paths: resolved,
      paths_unresolved: unresolved,
      written: message,
      where: "commit"
    })
  end

  # A short flag can stand in a cluster, eg the `a` in `-am`.
  defp short?("--" <> _long, _letter), do: false
  defp short?("-" <> letters, letter), do: String.contains?(letters, letter)
  defp short?(_word, _letter), do: false

  # Returns the commit's paths, its message, and its flags.
  defp commit_words([], paths, message, flags), do: {Enum.reverse(paths), message, flags}

  defp commit_words(["--" | rest], paths, message, flags),
    do: {Enum.reverse(paths) ++ rest, message, flags}

  defp commit_words(["--message=" <> value | rest], paths, message, flags),
    do: commit_words(rest, paths, join_message(message, value), flags)

  defp commit_words(["--message", value | rest], paths, message, flags),
    do: commit_words(rest, paths, join_message(message, value), flags)

  defp commit_words([flag, _value | rest], paths, message, flags)
       when flag in @commit_value_flags,
       do: commit_words(rest, paths, message, flags)

  defp commit_words(["--" <> _ = flag | rest], paths, message, flags),
    do: commit_words(rest, paths, message, [flag | flags])

  defp commit_words(["-" <> letters = flag | rest], paths, message, flags) do
    # `-m` takes the next word, also at the end of a cluster such as `-am`.
    # `-mtext` holds its message.
    case {Regex.run(~r/\A([A-Za-ln-z]*)m(.*)\z/s, letters), rest} do
      {[_, _before, ""], [value | rest]} ->
        commit_words(rest, paths, join_message(message, value), [flag | flags])

      {[_, _before, value], _rest} when value != "" ->
        commit_words(rest, paths, join_message(message, value), [flag | flags])

      _other ->
        commit_words(rest, paths, message, [flag | flags])
    end
  end

  defp commit_words([path | rest], paths, message, flags),
    do: commit_words(rest, [path | paths], message, flags)

  defp join_message(nil, value), do: value
  defp join_message(message, value), do: message <> "\n\n" <> value

  defp update_ref(words, cwd, act) do
    fields = %{
      cwd: cwd,
      ref: words |> Enum.reject(&flag?/1) |> List.first(),
      delete: Enum.any?(words, &(&1 in ~w(-d --delete)))
    }

    event(act, "git_ref", fields)
  end

  defp worktree(sub, words, cwd, act) do
    event(act, "git_worktree", %{
      cwd: cwd,
      sub: sub,
      path: words |> worktree_positional() |> List.first(),
      force: Enum.any?(words, &(&1 in ~w(-f --force)))
    })
  end

  defp worktree_positional([flag, _value | rest]) when flag in @worktree_value_flags,
    do: worktree_positional(rest)

  defp worktree_positional(["-" <> _ | rest]), do: worktree_positional(rest)
  defp worktree_positional([word | rest]), do: [word | worktree_positional(rest)]
  defp worktree_positional([]), do: []

  # A commit's answer opens with `[branch sha]`. One commit in the act with
  # one answer gives that commit its sha.
  defp with_commit_shas(events, result) do
    shas = for [_, sha] <- Regex.scan(@commit_answer, result), do: sha

    case {Enum.filter(events, &(&1.kind == "git_commit")), shas} do
      {[_one], [sha]} ->
        Enum.map(events, fn
          %{kind: "git_commit"} = commit -> Map.put(commit, :sha, sha)
          other -> other
        end)

      _other ->
        events
    end
  end

  # What the act's result says: the paths an `intent` verb printed, every
  # error line, and the exit code of a failed command. `act_line` is the
  # line of the command, and `line` is the line of its result.
  defp output(%{result: nil}, _events), do: []

  defp output(act, events) do
    intent = Enum.any?(events, &(&1.kind == "intent"))
    printed = if intent, do: Regex.scan(@printed_line, act.result), else: []
    paths = fn label -> for [_, ^label, path] <- printed, do: path end
    errors = for [line | _] <- Regex.scan(@error_line, act.result), do: String.trim(line)

    exit_code =
      case act.error && Regex.run(@exit_code, act.result) do
        [_, code] -> String.to_integer(code)
        _other -> nil
      end

    fields = %{
      intent: intent,
      wrote: paths.("wrote"),
      removed: paths.("removed"),
      rerendered: paths.("re-rendered"),
      errors: errors,
      failed: act.error,
      exit_code: exit_code,
      act_line: act.line,
      command: act |> command() |> String.slice(0, 300)
    }

    if fields.wrote ++ fields.removed ++ fields.rerendered ++ errors == [] and not act.error do
      []
    else
      [act |> event("output", fields) |> Map.put(:line, act.result_line || act.line)]
    end
  end

  defp event(act, kind, fields) do
    Map.merge(
      %{kind: kind, line: act.line, time: act.time, cwd: act.cwd, turn: act.turn, ci: act.ci},
      fields
    )
  end

  defp text(value) when is_binary(value), do: value
  defp text(_value), do: ""

  # A path is resolved from the transcript's own text. `~` is left as it is,
  # because the home directory of the reader is not the seat's.
  defp join(nil, dir), do: dir
  defp join(_cwd, "~" <> _ = dir), do: dir
  defp join(cwd, dir), do: Path.expand(dir, cwd)

  @doc "A path relative to the checkout when it is inside it. Any other path stays as it is."
  def relative("", _cwd, _config), do: ""

  def relative(path, cwd, config) do
    case config[:checkout] do
      nil -> path
      checkout -> cwd |> join(path) |> Path.relative_to(checkout)
    end
  end
end

defmodule Autopsy.Stamps do
  @moduledoc """
  Reads the timestamps a seat wrote and compares each with the clock. Every
  transcript entry carries the time the machine recorded it, so a written
  stamp later than its own entry came from no clock. Pure.
  """

  @full ~r/(\d{4}-\d{2}-\d{2})[ T](\d{2}:\d{2})(?::\d{2}(?:\.\d+)?)?(Z?)/
  # A time with no date: `10:01Z`, or the `10:01:23 UTC` of a plain `date -u`.
  @time_only ~r/(?<![\d:])(\d{2}:\d{2})(?::\d{2})?(?:Z| UTC)/
  @entry_time ~r/\A(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2})/
  @claims_read ~r/date -u/
  @claim_window 40
  @half_day_minutes 720

  @doc "Every stamp in a text, as `%{key, time, full, z, at, len, raw}`. `key` is `YYYY-MM-DD HH:MM`."
  def extract(text) when is_binary(text) do
    full =
      for [{at, len}, {ds, dl}, {ts, tl}, {zs, zl}] <- Regex.scan(@full, text, return: :index) do
        time = binary_part(text, ts, tl)

        %{
          key: binary_part(text, ds, dl) <> " " <> time,
          time: time,
          full: true,
          z: binary_part(text, zs, zl) == "Z",
          at: at,
          len: len,
          raw: binary_part(text, at, len)
        }
      end

    # A full stamp's own time is not a second, time-only stamp.
    blanked =
      Enum.reduce(full, text, fn stamp, acc ->
        binary_part(acc, 0, stamp.at) <>
          String.duplicate(" ", stamp.len) <>
          binary_part(acc, stamp.at + stamp.len, byte_size(acc) - stamp.at - stamp.len)
      end)

    time_only =
      for [{at, len}, {ts, tl}] <- Regex.scan(@time_only, blanked, return: :index) do
        time = binary_part(text, ts, tl)

        %{
          key: nil,
          time: time,
          full: false,
          z: true,
          at: at,
          len: len,
          raw: binary_part(text, at, len)
        }
      end

    Enum.sort_by(full ++ time_only, & &1.at)
  end

  def extract(_text), do: []

  @doc """
  One record for each stamp a seat wrote, with what the session's own record
  says about it: `ahead` of the entry's time, `sourced` from text the session
  had before, and `read_in_turn` from a tool result of this turn or the one
  before.
  """
  def report(events, acts, hears) do
    seen = for heard <- hears, do: {heard.line, 0, :seen, heard.turn, heard.text}

    results =
      for %{result: text} = act <- acts, is_binary(text) do
        {act.result_line || act.line, 0, :result, act.turn, text}
      end

    writes =
      for %{written: text} = event <- events, is_binary(text) do
        {event.line, 1, :write, event.turn, event}
      end

    start = %{seen: MapSet.new(), times: MapSet.new(), by_turn: %{}, records: []}

    (seen ++ results ++ writes)
    |> Enum.sort_by(fn {line, order, _kind, _turn, _item} -> {line, order} end)
    |> Enum.reduce(start, &step/2)
    |> Map.fetch!(:records)
    |> Enum.reverse()
    |> Enum.uniq_by(&{&1.event.seq, &1.stamp.raw})
  end

  defp step({_line, _order, :write, turn, event}, state) do
    stamps = extract(event.written)
    none = MapSet.new()

    in_turn =
      MapSet.union(Map.get(state.by_turn, turn, none), Map.get(state.by_turn, turn - 1, none))

    records =
      for stamp <- stamps do
        sourced = known?(stamp, state.seen, state.times)

        %{
          event: event,
          stamp: stamp,
          # A stamp the session held before is a quote, so it is not ahead.
          ahead: not sourced and ahead?(stamp, event.time),
          sourced: sourced,
          read_in_turn: known?(stamp, in_turn, in_turn),
          claims_read: claims_read?(event.written, stamp)
        }
      end

    state |> remember(stamps) |> Map.update!(:records, &(Enum.reverse(records) ++ &1))
  end

  defp step({_line, _order, kind, turn, text}, state) do
    stamps = extract(text)
    state = remember(state, stamps)

    if kind == :result do
      keys = MapSet.new(Enum.flat_map(stamps, &[&1.key, &1.time]))
      %{state | by_turn: Map.update(state.by_turn, turn, keys, &MapSet.union(&1, keys))}
    else
      state
    end
  end

  defp remember(state, stamps) do
    %{
      state
      | seen: for(%{full: true, key: key} <- stamps, into: state.seen, do: key),
        times: for(%{time: time} <- stamps, into: state.times, do: time)
    }
  end

  defp known?(%{full: true, key: key}, keys, _times), do: MapSet.member?(keys, key)
  defp known?(%{full: false, time: time}, _keys, times), do: MapSet.member?(times, time)

  defp claims_read?(text, stamp) do
    from = stamp.at + stamp.len
    after_stamp = binary_part(text, from, min(@claim_window, byte_size(text) - from))
    Regex.match?(@claims_read, after_stamp)
  end

  # `time` is the entry's own ISO time, eg `2026-11-25T18:05:43.448Z`. An
  # entry with no such time gives no answer, so the stamp is not ahead.
  defp ahead?(stamp, time) when is_binary(time) do
    case Regex.run(@entry_time, time) do
      [_, date, clock] -> ahead?(stamp, date, clock)
      nil -> false
    end
  end

  defp ahead?(_stamp, _time), do: false

  defp ahead?(%{full: true, key: key}, date, clock),
    do: String.starts_with?(key, date) and key > date <> " " <> clock

  defp ahead?(%{full: false, time: stamp_clock}, _date, clock) do
    gap = minutes(stamp_clock) - minutes(clock)
    gap > 0 and gap < @half_day_minutes
  end

  defp minutes(<<h::binary-size(2), ":", m::binary-size(2)>>),
    do: String.to_integer(h) * 60 + String.to_integer(m)
end

defmodule Autopsy.Checks do
  @moduledoc """
  The mechanical checks. Each check is one function with one name, and it
  holds the shape it looks for. It holds no rule text: `source` names where
  the rule lives. A check returns its findings, and the caller records that
  it ran when it finds none. Order is by an event's `seq`. Pure.
  """

  @stamp_rule "/in-whiteboard: Every timestamp is READ FROM A CLOCK"
  @commit_rule "/in-essentials rule 9, and /in-whiteboard invariants 9 and 10"
  @ulid ~r/\b[0-9A-HJKMNP-TV-Z]{26}\b/

  @doc "Every check: its name, where its rule lives, the project values it needs, and its function."
  def all do
    [
      check("stamp_ahead_of_clock", @stamp_rule, [], &stamp_ahead_of_clock/1),
      check("stamp_without_read", @stamp_rule, [], &stamp_without_read/1),
      check("stamp_without_z", @stamp_rule, [], &stamp_without_z/1),
      check("commit_without_only", @commit_rule, [:checkout], &commit_without_only/1),
      check("add_all", @commit_rule, [:checkout], &add_all/1),
      check(
        "commit_path_not_printed",
        @commit_rule,
        [:checkout, :views],
        &commit_path_not_printed/1
      ),
      check(
        "printed_path_not_committed",
        @commit_rule,
        [:checkout],
        &printed_path_not_committed/1
      ),
      check("commit_of_rerendered", @commit_rule, [:checkout], &commit_of_rerendered/1),
      check("write_as_other_seat", "/in-whiteboard invariant 1", [:seat], &write_as_other_seat/1),
      check(
        "hand_edit_of_view",
        "/in-whiteboard: The files",
        [:checkout, :views],
        &hand_edit_of_view/1
      ),
      check(
        "release_then_work",
        "/in-whiteboard: `release` is for session end only",
        [],
        &release_then_work/1
      ),
      check(
        "ask_hv_past_mediator",
        "intent wb profile terms: The mediator",
        [:seat, :mediator],
        &ask_hv_past_mediator/1
      ),
      check(
        "heavy_run_without_slot",
        "AGENTS.md: Build and Test Commands",
        [:heavy],
        &heavy_run_without_slot/1
      ),
      check("slot_not_released", "intent llm guide: estate slots", [], &slot_not_released/1),
      check(
        "start_without_end",
        "the board's rule for START and END messages",
        [],
        &start_without_end/1
      ),
      check("at_red_without_run", "intent llm guide: at red", [:heavy], &at_red_without_run/1),
      check(
        "at_green_without_run",
        "intent llm guide: at green",
        [:heavy],
        &at_green_without_run/1
      ),
      check("fiat_close", "IN-AG-FIAT-001", [], &fiat_close/1),
      check("worktree_remove_forced", "/in-whiteboard: Folding", [], &worktree_remove_forced/1),
      check(
        "worktree_remove_without_attic",
        "the board's rule for attic refs",
        [],
        &worktree_remove_without_attic/1
      ),
      check(
        "refusal_repeated",
        "intent llm guide: the verb that refused",
        [],
        &refusal_repeated/1
      ),
      check("exit_not_acted_on", "IN-AG-NO-SILENT-001", [], &exit_not_acted_on/1)
    ]
  end

  defp check(name, source, needs, run), do: %{name: name, source: source, needs: needs, run: run}

  @doc """
  Runs every check on one session. Each answer is `{check, {:ran, findings}}`
  or `{check, {:not_run, reason}}`, so a clean answer and a check that did
  not run differ.
  """
  def run(session) do
    stamps = Autopsy.Stamps.report(session.events, session.acts, session.hears)
    session = Map.put(session, :stamps, stamps)

    for check <- all() do
      case Enum.find(check.needs, &(Map.get(session.config, &1) in [nil, []])) do
        nil ->
          tag = %{check: check.name, source: check.source}
          {check, {:ran, Enum.map(check.run.(session), &Map.merge(&1, tag))}}

        need ->
          {check, {:not_run, missing(need)}}
      end
    end
  end

  defp missing(:seat), do: "the session names no seat (it has no `agent-name` entry)"
  defp missing(:mediator), do: "no `--mediator` was given"
  defp missing(:checkout), do: "no `--checkout` was given"
  defp missing(:views), do: "no `--views` was given"
  defp missing(:heavy), do: "no `--heavy` was given"

  defp finding(at, quote, detail, extra \\ %{}) do
    Map.merge(
      %{
        line: at.line,
        time: at.time,
        quote: quote |> to_string() |> String.slice(0, 240),
        detail: detail
      },
      extra
    )
  end

  defp kind(session, kind), do: Enum.filter(session.events, &(&1.kind == kind))

  # The `seq` of the last event of a list that comes before `at`, or -1.
  defp previous(events, at),
    do: events |> Enum.filter(&(&1.seq < at.seq)) |> Enum.map(& &1.seq) |> Enum.max(fn -> -1 end)

  # --- stamps ---------------------------------------------------------------

  defp stamp_ahead_of_clock(session) do
    for %{ahead: true, event: event} = record <- session.stamps do
      detail = "written in a #{event.where} at #{event.time}, which is before the stamp"
      finding(event, record.stamp.raw, detail)
    end
  end

  defp stamp_without_read(session) do
    for %{ahead: false, event: event} = record <- session.stamps, unread?(record) do
      detail =
        if record.claims_read,
          do:
            "the text names `date -u`, and no tool result of this turn or the one before holds the value",
          else: "no earlier text of the session holds the value"

      finding(event, record.stamp.raw, detail <> " (written in a #{event.where})")
    end
  end

  defp unread?(%{claims_read: true} = record), do: not record.read_in_turn
  defp unread?(record), do: not record.sourced

  defp stamp_without_z(session) do
    for %{stamp: %{full: true, z: false}, event: event} = record <- session.stamps do
      finding(event, record.stamp.raw, "written in a #{event.where}")
    end
  end

  # --- commits --------------------------------------------------------------

  # True for an act in the shared checkout's root. A commit in a worktree is
  # another case, and so is an act whose directory is not known.
  defp shared?(%{cwd: cwd}, session) when is_binary(cwd), do: cwd == session.config.checkout
  defp shared?(_event, _session), do: false

  defp commit_without_only(session) do
    for commit <- kind(session, "git_commit"), shared?(commit, session), not commit.only do
      finding(commit, commit.written || "", "a commit in the shared checkout with no `--only`")
    end
  end

  defp add_all(session) do
    for add <- kind(session, "git_add"), shared?(add, session), add.all do
      finding(add, "git add", "every change in the shared checkout was staged")
    end
  end

  # Each commit in the shared checkout, with the paths the verbs printed
  # before it. A verb in the commit's own command counts as before it.
  defp with_printed(session) do
    outputs = kind(session, "output")

    for commit <- kind(session, "git_commit"), shared?(commit, session) do
      before = Enum.filter(outputs, &(&1.seq < commit.seq or &1.act_line == commit.line))
      wrote = before |> Enum.flat_map(&(&1.wrote ++ &1.removed)) |> norm(session) |> MapSet.new()
      rerendered = before |> Enum.flat_map(& &1.rerendered) |> norm(session) |> MapSet.new()
      {commit, wrote, rerendered}
    end
  end

  defp norm(paths, session) do
    Enum.map(paths, &Autopsy.Events.relative(&1, session.config.checkout, session.config))
  end

  defp commit_path_not_printed(session) do
    for {commit, wrote, _rerendered} <- with_printed(session),
        stray = Enum.filter(commit.paths, &(view?(&1, session) and not MapSet.member?(wrote, &1))),
        stray != [] do
      detail = "a committed path that no verb printed before the commit"
      finding(commit, Enum.join(stray, " "), detail)
    end
  end

  defp commit_of_rerendered(session) do
    for {commit, wrote, rerendered} <- with_printed(session),
        other =
          Enum.filter(
            commit.paths,
            &(MapSet.member?(rerendered, &1) and not MapSet.member?(wrote, &1))
          ),
        other != [] do
      detail =
        "a committed path that a verb printed as `re-rendered:`, which is another owner's file"

      finding(commit, Enum.join(other, " "), detail)
    end
  end

  # A lead: a pickup alone is not committed. A later commit whose paths are
  # not known can hold the path, so then there is no finding.
  defp printed_path_not_committed(session) do
    commits = for commit <- kind(session, "git_commit"), shared?(commit, session), do: commit

    for out <- kind(session, "output"),
        out.intent,
        later = Enum.filter(commits, &(&1.seq > out.seq or &1.line == out.act_line)),
        not Enum.any?(later, &(&1.paths_unresolved or &1.paths == [])),
        path <- norm(out.wrote ++ out.removed, session),
        not Enum.any?(later, &(path in &1.paths)) do
      detail = "a printed path that no later commit of the session names"
      finding(out, path, detail, %{lead: true})
    end
  end

  defp view?(path, session), do: Enum.any?(session.config.views, &Regex.match?(&1, path))

  # --- the whiteboard -------------------------------------------------------

  defp write_as_other_seat(session) do
    for %{flags: %{"node" => node}} = intent <- kind(session, "intent"),
        is_binary(node) and node != session.config.seat do
      finding(
        intent,
        intent.raw,
        "`--node #{node}` in the session of seat #{session.config.seat}"
      )
    end
  end

  defp hand_edit_of_view(session) do
    for write <- kind(session, "file_write"), view?(write.path, session) do
      finding(write, write.path, "written by #{write.via}")
    end
  end

  defp release_then_work(session) do
    for %{noun: "wb", verb: "release"} = release <- kind(session, "intent"),
        later = Enum.find(session.acts, &(&1.line > release.line)),
        later != nil do
      finding(release, release.raw, "the session's next act is at line #{later.line}")
    end
  end

  # The recipient is the first argument. A flag that takes no value can take
  # it as its value, so a flag's value is read too.
  defp ask_hv_past_mediator(session) do
    for %{noun: "wb", verb: "ask"} = ask <- kind(session, "intent"),
        List.first(ask.args) == "hv" or "hv" in Map.values(Map.delete(ask.flags, "node")),
        session.config.seat != session.config.mediator do
      detail = "seat #{session.config.seat} is not the mediator, #{session.config.mediator}"
      finding(ask, ask.raw, detail)
    end
  end

  # --- the Desk -------------------------------------------------------------

  defp slot_events(session) do
    for %{noun: "estate", verb: "slots", args: [sub | rest]} = slot <- kind(session, "intent"),
        sub in ~w(start release),
        do: {sub, List.first(rest), slot}
  end

  defp heavy_run_without_slot(session) do
    slots = slot_events(session)

    for run <- kind(session, "heavy_run"), open_slots(slots, run) == [] do
      detail =
        "no slot was open: no `intent estate slots start` came before it without its `release`"

      finding(run, run.command, detail)
    end
  end

  defp open_slots(slots, run) do
    Enum.reduce(slots, [], fn
      {_sub, _id, %{seq: seq}}, open when seq > run.seq -> open
      {"start", id, _slot}, open -> [id | open]
      {"release", id, _slot}, open -> List.delete(open, id)
    end)
  end

  defp slot_not_released(session) do
    slots = slot_events(session)

    for {"start", id, slot} <- slots,
        not Enum.any?(slots, fn {sub, other, later} ->
          sub == "release" and other == id and later.seq > slot.seq
        end) do
      finding(slot, slot.raw, "the session holds no `release` for this request")
    end
  end

  defp start_without_end(session) do
    messages = kind(session, "message")

    for start <- messages,
        Regex.match?(~r/^\s*START\b/m, start.written),
        not Enum.any?(messages, &ends?(&1, start)) do
      finding(start, start.written, "no later END message to #{start.to} for the same request")
    end
  end

  # An END that names no request id ends the START before it. When both name
  # ids, they must share one.
  defp ends?(message, start) do
    start_ids = ulids(start.written)
    end_ids = ulids(message.written)

    message.seq > start.seq and message.to == start.to and
      Regex.match?(~r/^\s*END\b/m, message.written) and
      (start_ids == [] or end_ids == [] or Enum.any?(start_ids, &(&1 in end_ids)))
  end

  defp ulids(text), do: @ulid |> Regex.scan(text) |> List.flatten()

  # --- acceptance tests -----------------------------------------------------

  defp at_red_without_run(session), do: at_without_run(session, "red")
  defp at_green_without_run(session), do: at_without_run(session, "green")

  # A run counts when it came after the session's previous `intent at` call.
  defp at_without_run(session, verb) do
    ats = for %{noun: "at"} = at <- kind(session, "intent"), do: at
    runs = kind(session, "heavy_run")

    for %{verb: ^verb} = at <- ats,
        from = previous(ats, at),
        not Enum.any?(runs, &(&1.seq > from and &1.seq < at.seq)) do
      finding(at, at.raw, "no run came between the `intent at` call before it and this call")
    end
  end

  defp fiat_close(session) do
    for %{noun: "fc"} = fc <- kind(session, "intent") do
      finding(fc, fc.raw, "`intent fc` is the person's verb")
    end
  end

  # --- worktrees ------------------------------------------------------------

  defp removes(session),
    do: for(%{sub: "remove"} = remove <- kind(session, "git_worktree"), do: remove)

  defp worktree_remove_forced(session) do
    for %{force: true} = remove <- removes(session) do
      finding(remove, remove.path || "", "removed with `--force`")
    end
  end

  # An attic ref counts when it was written after the session's previous remove.
  defp worktree_remove_without_attic(session) do
    removes = removes(session)

    attics =
      for %{ref: "refs/attic/" <> _, delete: false} = ref <- kind(session, "git_ref"), do: ref

    for remove <- removes,
        from = previous(removes, remove),
        not Enum.any?(attics, &(&1.seq > from and &1.seq < remove.seq)) do
      detail = "no `refs/attic/` ref was written between the remove before it and this remove"
      finding(remove, remove.path || "", detail)
    end
  end

  # --- what the tools said --------------------------------------------------

  defp refusal_repeated(session) do
    session
    |> kind("output")
    |> Enum.filter(&(&1.intent and &1.errors != []))
    |> Enum.group_by(&shape(hd(&1.errors)))
    |> Enum.flat_map(fn
      {_shape, [_once]} ->
        []

      {_shape, [first | again]} ->
        lines = Enum.map_join([first | again], ", ", & &1.line)
        [finding(hd(again), hd(first.errors), "the same refusal at lines #{lines}")]
    end)
  end

  # Two refusals have one shape when they differ only in ids, numbers, and quoted text.
  defp shape(error) do
    error
    |> String.replace(~r/'[^']*'|"[^"]*"|`[^`]*`/, "_")
    |> String.replace(@ulid, "_")
    |> String.replace(~r/\b[0-9a-f]{7,40}\b|\d+/, "_")
  end

  # A command failed, and the seat wrote no text in the rest of that turn.
  defp exit_not_acted_on(session) do
    for %{error: true} = act <- session.acts,
        not Enum.any?(session.says, &(&1.turn == act.turn and &1.line > act.line)) do
      quote = if act.tool == "Bash", do: Autopsy.Events.command(act), else: act.tool
      finding(act, quote, "the #{act.tool} call failed, and the turn has no text after it")
    end
  end
end

defmodule Autopsy.Signals do
  @moduledoc """
  Finds corrections by who made them: a person, a seat, the tool, or the
  seat itself. A person's signals are read only from entries a person typed.
  Pure.
  """

  # The word as a noun in plain text is not a signal. These forms announce one.
  @correction ~r/\bCORRECTION\b|\b[Cc]orrection:|\b[Aa] correction\b|\b[Ss]tamp correction\b/
  @retract ~r/\bretract/i

  @person [
    {~r/\bflag(ged)?\b/i, "flag"},
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

  @seat [
    {@correction, "correction"},
    {~r/\bruling changed\b/i, "ruling changed"},
    {~r/\b(was|were|is) wrong\b/i, "was wrong"},
    {@retract, "retract"},
    {~r/\bnot what (I|we|hv) asked\b/i, "not what was asked"},
    {~r/\bdo not\b.{0,40}\bagain\b/i, "do not, again"}
  ]

  @self [
    {@correction, "correction"},
    {~r/no clock gave|not read from a clock/i, "stamp not from a clock"},
    {~r/\bI was wrong\b|\bmy mistake\b|\bI stand corrected\b/i, "was wrong"},
    {~r/you'?re (right|correct)|\bgood catch\b/i, "you are right"},
    {~r/\bI apologi[sz]e\b|\bmy apologies\b|\bsorry about that\b/i, "apology"},
    {~r/\bI should have\b|\bI missed that\b/i, "should have"},
    {@retract, "retract"}
  ]

  @by_source %{"person" => @person, "seat" => @seat}

  @doc "The session's signals in line order, each with `by`, its labels, and a quote."
  def find(session) do
    heard =
      for %{source: source} = heard <- session.hears,
          is_map_key(@by_source, source),
          labels = labels(@by_source[source], heard.text),
          labels != [] do
        signal(heard, source, heard.from, labels, heard.text)
      end

    said =
      for say <- session.says, labels = labels(@self, say.text), labels != [] do
        signal(say, "self", nil, labels, say.text)
      end

    # A seat also speaks to a peer through the board, eg with `intent wb ask`.
    sent =
      for %{written: text, where: where} = event <- session.events,
          is_binary(text) and where in ["message", "wb"],
          labels = labels(@self, text),
          labels != [] do
        signal(event, "self", nil, labels, text)
      end

    tool =
      for %{kind: "output", errors: [error | _]} = out <- session.events do
        out |> signal("tool", nil, ["refusal"], error) |> Map.put(:command, out.command)
      end

    Enum.sort_by(heard ++ said ++ sent ++ tool, & &1.line)
  end

  defp labels(patterns, text),
    do: for({pattern, label} <- patterns, Regex.match?(pattern, text), do: label)

  defp signal(at, by, from, labels, text) do
    %{
      by: by,
      from: from,
      line: at.line,
      time: at.time,
      labels: labels,
      quote: String.slice(text, 0, 500),
      compaction_index: at.ci,
      post_compaction: at.ci > 0
    }
  end
end

defmodule Autopsy.Patterns do
  @moduledoc """
  Scans what the seat wrote with the pattern file of one run. The procedure
  writes that file from the rules it read, so this module holds no word
  list. It gives one record for each occurrence, with the quoted and negated
  marks. Pure.
  """

  @separator " :: "
  @negation_window 60
  @negation_cues ~r/(avoid|never use|do not use|don'?t use|without|stop using|banned|forbidden|instead of)/i

  @doc """
  Parses the pattern file's text. Each line is `scope :: pattern :: source`,
  where scope is `prose` or `command` and pattern is a regular expression.
  It returns `{:ok, patterns}` or `{:error, {line, reason}}`.
  """
  def parse(text) do
    lines =
      text
      |> String.split("\n")
      |> Enum.with_index(1)
      |> Enum.reject(fn {line, _n} ->
        String.trim(line) == "" or String.starts_with?(String.trim(line), "#")
      end)

    with {:ok, patterns} <- parse_lines(lines), do: {:ok, Enum.reverse(patterns)}
  end

  defp parse_lines(lines) do
    Enum.reduce_while(lines, {:ok, []}, fn {line, n}, {:ok, acc} ->
      case parse_line(line) do
        {:ok, pattern} -> {:cont, {:ok, [pattern | acc]}}
        {:error, reason} -> {:halt, {:error, {n, reason}}}
      end
    end)
  end

  # The `u` flag makes `.` match a character, so a match is valid UTF-8.
  defp parse_line(line) do
    with [scope, pattern, source]
         when scope in ["prose", "command"] and pattern != "" and source != "" <-
           line |> String.split(@separator, parts: 3) |> Enum.map(&String.trim/1),
         {:ok, regex} <- Regex.compile(pattern, "iu"),
         false <- Regex.match?(regex, "") do
      {:ok, %{scope: scope, pattern: pattern, regex: regex, source: source}}
    else
      {:error, {reason, at}} ->
        {:error, "the pattern is not a regular expression: #{reason} at #{at}"}

      true ->
        {:error, "the pattern matches the empty text, so it matches everywhere"}

      _other ->
        {:error, "the line is not `prose|command :: pattern :: source`"}
    end
  end

  @doc "One record for each occurrence of a pattern in the seat's own text or commands."
  def scan(patterns, session) do
    replies = for say <- session.says, do: {say, "reply", say.text}

    written =
      for %{written: text} = event <- session.events, is_binary(text) and String.valid?(text) do
        {event, event.where, text}
      end

    commands =
      for act <- session.acts, command = Autopsy.Events.command(act), command != "" do
        {act, "command", command}
      end

    targets = %{"prose" => replies ++ written, "command" => commands}

    for pattern <- patterns,
        {at, where, text} <- targets[pattern.scope],
        [{start, len} | _] <- Regex.scan(pattern.regex, text, return: :index) do
      prose = pattern.scope == "prose"

      %{
        pattern: pattern.pattern,
        source: pattern.source,
        scope: pattern.scope,
        where: where,
        line: at.line,
        time: at.time,
        match: binary_part(text, start, len),
        is_negated: prose and negated_at?(text, start, len),
        is_quoted: prose and quoted_at?(text, start, len),
        context: safe_slice(text, start - 60, len + 120)
      }
    end
  end

  # A cue beside the occurrence shows that the text states the rule.
  defp negated_at?(text, start, len) do
    Regex.match?(@negation_cues, safe_slice(text, start - @negation_window, @negation_window)) or
      Regex.match?(@negation_cues, safe_slice(text, start + len, @negation_window))
  end

  # A match inside backticks or quotation marks is the rule in discussion.
  defp quoted_at?(text, start, len),
    do: String.contains?(safe_slice(text, start - 40, len + 80), ["`", "\"", "“", "”"])

  # Regex indexes are byte offsets, so the slice is by bytes. A cut can split
  # a character, and the partial bytes are dropped so that the result is valid.
  defp safe_slice(text, start, len) do
    size = byte_size(text)
    from = max(0, min(start, size))
    count = max(0, min(len + min(start, 0), size - from))

    text
    |> binary_part(from, count)
    |> String.chunk(:valid)
    |> Enum.filter(&String.valid?/1)
    |> Enum.join()
  end
end

defmodule Autopsy.Report do
  @moduledoc """
  Builds the output document from the sessions: what each session holds
  inside the window, and each check's answer across the sessions. Pure.
  """

  @doc "One session's part of the document. `window` is `{since, until}`, each a stamp or nil."
  def session(session, patterns, window) do
    inside = &inside?(&1[:time], window)
    timed = session.says ++ session.hears ++ session.acts
    times = for(%{time: time} <- timed, is_binary(time), do: time) |> Enum.sort()

    %{
      id: session.id,
      name: session.name,
      seat: session.seat,
      from: List.first(times),
      to: List.last(times),
      compactions: session.compactions,
      user_entries:
        for(heard <- session.hears, do: Map.take(heard, [:line, :time, :source, :from])),
      unread: session.unread,
      events: session.events |> Enum.filter(inside) |> Enum.map(&render_event/1),
      checks:
        for({check, answer} <- Autopsy.Checks.run(session), do: {check, within(answer, inside)}),
      signals: session |> Autopsy.Signals.find() |> Enum.filter(inside),
      patterns: patterns && patterns |> Autopsy.Patterns.scan(session) |> Enum.filter(inside)
    }
  end

  defp within({:ran, findings}, inside), do: {:ran, Enum.filter(findings, inside)}
  defp within(not_run, _inside), do: not_run

  # An edge is in the transcript's own form, so the two compare as text. A
  # record with no time is kept: it is not shown to be outside the window.
  defp inside?(time, {since, until}) when is_binary(time) do
    (since == nil or String.slice(time, 0, byte_size(since)) >= since) and
      (until == nil or String.slice(time, 0, byte_size(until)) <= until)
  end

  defp inside?(_time, _window), do: true

  # The output gives a short quote of what was written. The checks read all of it.
  defp render_event(event) do
    case Map.pop(event, :written) do
      {text, event} when is_binary(text) ->
        Map.put(event, :written_quote, String.slice(text, 0, 200))

      {_none, event} ->
        event
    end
  end

  @doc "The whole document. `run` says how the script ran, and `results` are the sessions' parts."
  def document(run, results) do
    tag = fn records, session -> Enum.map(records, &Map.put(&1, :session, session.id)) end

    findings =
      for session <- results, {_check, {:ran, findings}} <- session.checks do
        tag.(findings, session)
      end

    %{
      run: run,
      sessions:
        Enum.map(
          results,
          &Map.take(&1, [:id, :name, :seat, :from, :to, :compactions, :user_entries])
        ),
      events: Enum.flat_map(results, &tag.(&1.events, &1)),
      findings: List.flatten(findings),
      checks_run: checks_run(results),
      signals: Enum.flat_map(results, &tag.(&1.signals, &1)),
      patterns: Enum.flat_map(results, &tag.(&1.patterns || [], &1)),
      unread: Enum.flat_map(results, &tag.(&1.unread, &1))
    }
  end

  defp checks_run(results) do
    for check <- Autopsy.Checks.all() do
      answers =
        for session <- results, {%{name: name}, answer} <- session.checks, name == check.name do
          {session.id, answer}
        end

      %{
        check: check.name,
        source: check.source,
        ran_on: for({id, {:ran, _findings}} <- answers, do: id),
        found_in: for({id, {:ran, [_ | _]}} <- answers, do: id),
        not_run: for({id, {:not_run, reason}} <- answers, do: %{session: id, reason: reason})
      }
    end
  end

  @doc "The summary's lines: each check by name with what it found. There is no total."
  def summary(document) do
    checks =
      for check <- document.checks_run do
        answer =
          cond do
            check.found_in != [] ->
              "found in " <> Enum.join(check.found_in, ", ")

            check.ran_on != [] ->
              "nothing"

            true ->
              "not run: " <>
                (check.not_run |> Enum.map(& &1.reason) |> Enum.uniq() |> Enum.join("; "))
          end

        "  #{check.check}: #{answer}"
      end

    unread =
      case document.unread do
        [] -> []
        lines -> ["  unread lines: " <> Enum.map_join(lines, ", ", &"#{&1.session}:#{&1.line}")]
      end

    checks ++ ["  pattern scan: #{document.run.pattern_scan}"] ++ unread
  end
end

defmodule Autopsy.CLI do
  @moduledoc """
  The impure coordinator: it reads arguments, files, and the clock, calls
  the pure modules, and writes the output. Every failure is reported with
  its remedy, and the exit is then non-zero.
  """

  @version "2"
  @usage_exit 2
  @failure_exit 1
  @default_days 7
  @day_seconds 86_400
  @value_flags ~w(--project --session --seat --since --until --days --patterns --views --heavy --checkout --mediator --output -o)
  @bool_flags ~w(--all --subagents --help -h)

  def main(argv) do
    opts = parse(argv, %{})
    if opts[:help], do: IO.puts(help()), else: run(opts)
  end

  # --- arguments ------------------------------------------------------------

  defp parse([], opts), do: opts

  defp parse([flag | rest], opts) when flag in @bool_flags,
    do: parse(rest, Map.put(opts, key(flag), true))

  defp parse([flag, value | rest], opts) when flag in @value_flags do
    key = key(flag)

    opts =
      if key in [:views, :heavy],
        do: Map.update(opts, key, [value], &(&1 ++ [value])),
        else: Map.put(opts, key, value)

    parse(rest, opts)
  end

  defp parse([flag], _opts) when flag in @value_flags,
    do:
      halt(
        @usage_exit,
        "`#{flag}` needs a value",
        "give the value after the flag; `--help` lists the arguments"
      )

  defp parse([unknown | _rest], _opts),
    do:
      halt(
        @usage_exit,
        "`#{unknown}` is not an argument of autopsy.exs",
        "`--help` lists the arguments"
      )

  defp key("-o"), do: :output
  defp key("-h"), do: :help
  defp key("--" <> name), do: String.to_atom(name)

  defp halt(code, what, remedy) do
    IO.puts(:stderr, "error: #{what}\n  remedy: #{remedy}")
    System.halt(code)
  end

  # --- the run --------------------------------------------------------------

  defp run(opts) do
    unless opts[:session] || opts[:seat] || opts[:all] do
      halt(
        @usage_exit,
        "no scope was given",
        "give `--seat <moniker>` for one seat's sessions, `--session <id>` for one session, or `--all` for every session of the project"
      )
    end

    now = DateTime.utc_now()
    config = config(opts)
    patterns = patterns(opts[:patterns])
    project_dir = project_dir(opts[:project])
    {since, _until} = window = window(opts, now)

    results =
      for path <- top_files(project_dir, opts[:session]),
          since == nil or modified(path) >= since,
          parent = read_session(path, nil, config),
          opts[:seat] == nil or parent.seat == opts[:seat],
          session <- [parent | subagents(path, parent, opts, since, config)] do
        Autopsy.Report.session(session, patterns, window)
      end

    if results == [] do
      halt(
        @failure_exit,
        "no session in #{project_dir} matches the scope and the window",
        "widen `--days` or `--since`, or check `--seat`, `--session`, and `--project`"
      )
    end

    run = %{
      version: @version,
      at: DateTime.to_iso8601(now),
      project_dir: project_dir,
      since: since,
      until: elem(window, 1),
      arguments: Map.drop(opts, [:help]),
      pattern_scan: if(patterns, do: "ran", else: "did not run: no `--patterns` file was given")
    }

    document = Autopsy.Report.document(run, results)
    write(opts[:output], document)
    IO.puts(:stderr, Enum.join(["" | Autopsy.Report.summary(document)], "\n"))
  end

  defp config(opts) do
    %{
      checkout: opts[:checkout] && Path.expand(opts[:checkout]),
      mediator: opts[:mediator],
      views: opts[:views] && Enum.map(opts[:views], &glob/1),
      heavy: opts[:heavy] && Enum.map(opts[:heavy], &regex(&1, "--heavy"))
    }
  end

  # `**` is any path, and `*` is any text inside one path segment.
  defp glob(pattern) do
    source =
      pattern
      |> Regex.escape()
      |> String.replace("\\*\\*", ".*")
      |> String.replace("\\*", "[^/]*")

    Regex.compile!("\\A" <> source <> "\\z")
  end

  defp regex(source, flag) do
    case Regex.compile(source) do
      {:ok, regex} ->
        regex

      {:error, {reason, at}} ->
        halt(
          @usage_exit,
          "the `#{flag}` value `#{source}` is not a regular expression: #{reason} at #{at}",
          "correct the pattern"
        )
    end
  end

  defp patterns(nil), do: nil

  defp patterns(path) do
    with {:ok, text} <- File.read(Path.expand(path)),
         {:ok, patterns} <- Autopsy.Patterns.parse(text) do
      patterns
    else
      {:error, {line, reason}} ->
        halt(
          @usage_exit,
          "#{path} line #{line}: #{reason}",
          "correct the line; each line is `prose|command :: pattern :: source`"
        )

      {:error, reason} ->
        halt(
          @usage_exit,
          "the pattern file #{path} cannot be read: #{:file.format_error(reason)}",
          "give the path of the pattern file the procedure wrote"
        )
    end
  end

  defp config_dir do
    case System.get_env("CLAUDE_CONFIG_DIR") do
      dir when dir in [nil, ""] -> Path.expand("~/.claude")
      dir -> Path.expand(dir)
    end
  end

  # With no `--project`, the project is the one Claude Code keeps for the
  # current directory. A name that matches more than one folder is refused.
  defp project_dir(nil) do
    with {:ok, cwd} <- File.cwd(),
         dir = Path.join([config_dir(), "projects", String.replace(cwd, ~r/[^A-Za-z0-9]/, "-")]),
         true <- File.dir?(dir) do
      dir
    else
      _other ->
        halt(
          @usage_exit,
          "Claude Code keeps no project folder for the current directory",
          "run from the project's root, or give `--project <name>`"
        )
    end
  end

  defp project_dir(name) do
    root = Path.join(config_dir(), "projects")

    matches =
      root
      |> Path.join("*")
      |> Path.wildcard()
      |> Enum.filter(
        &(File.dir?(&1) and
            String.contains?(String.downcase(Path.basename(&1)), String.downcase(name)))
      )

    case matches do
      [dir] ->
        dir

      [] ->
        halt(
          @usage_exit,
          "no project folder in #{root} matches `#{name}`",
          "give a part of the folder's name; `ls #{root}` lists them"
        )

      many ->
        names = Enum.map_join(many, "\n", &("    " <> Path.basename(&1)))

        halt(
          @usage_exit,
          "`#{name}` matches more than one project folder:\n" <> names,
          "give a name that matches one folder"
        )
    end
  end

  defp window(opts, now) do
    since =
      cond do
        opts[:since] -> stamp(opts[:since], "--since")
        opts[:days] -> days_before(now, opts[:days])
        opts[:session] -> nil
        true -> days_before(now, Integer.to_string(@default_days))
      end

    {since, opts[:until] && stamp(opts[:until], "--until")}
  end

  defp days_before(now, text) do
    case Integer.parse(text) do
      {days, ""} when days > 0 ->
        now |> DateTime.add(-days * @day_seconds) |> minute()

      _other ->
        halt(
          @usage_exit,
          "`--days #{text}` is not a whole number of days",
          "give a number, eg `--days 7`"
        )
    end
  end

  # A time to the minute, in the transcript's own form.
  defp minute(datetime), do: datetime |> DateTime.to_iso8601() |> String.slice(0, 16)

  # A window edge is a UTC stamp, eg `2026-11-25` or `2026-11-25 18:00Z`.
  defp stamp(text, flag) do
    case Regex.run(~r/\A(\d{4}-\d{2}-\d{2})(?:[ T](\d{2}:\d{2})(?::\d{2})?Z?)?\z/, text) do
      [_, date] ->
        date

      [_, date, time] ->
        date <> "T" <> time

      nil ->
        halt(
          @usage_exit,
          "`#{flag} #{text}` is not a UTC stamp",
          "give `YYYY-MM-DD` or `YYYY-MM-DD HH:MMZ`"
        )
    end
  end

  defp top_files(project_dir, session) do
    files = project_dir |> Path.join("*.jsonl") |> Path.wildcard() |> Enum.sort()

    if session,
      do: Enum.filter(files, &String.starts_with?(Path.basename(&1), session)),
      else: files
  end

  # A subagent holds no seat of its own. It works for its parent's seat.
  defp subagents(path, parent, opts, since, config) do
    for file <-
          path
          |> Path.rootname()
          |> Path.join("subagents/*.jsonl")
          |> Path.wildcard()
          |> Enum.sort(),
        opts[:subagents],
        since == nil or modified(file) >= since do
      read_session(file, parent, config)
    end
  end

  # A file that was not written since the window opened has no entry in it.
  defp modified(path) do
    case File.stat(path, time: :posix) do
      {:ok, %{mtime: mtime}} ->
        mtime |> DateTime.from_unix!() |> minute()

      {:error, reason} ->
        halt(
          @failure_exit,
          "#{path} cannot be read: #{:file.format_error(reason)}",
          "check the file's permissions"
        )
    end
  end

  defp read_session(path, parent, config) do
    IO.puts(:stderr, "reading #{Path.basename(path)}")

    text =
      case File.read(path) do
        {:ok, text} ->
          text

        {:error, reason} ->
          halt(
            @failure_exit,
            "#{path} cannot be read: #{:file.format_error(reason)}",
            "check the file's permissions"
          )
      end

    {entries, unread} =
      text
      |> String.split("\n")
      |> Enum.with_index(1)
      |> Enum.reduce({[], []}, fn {line_text, line}, {entries, unread} ->
        case Autopsy.Transcript.read_line(line_text, line) do
          {:ok, %{kind: :other}} -> {entries, unread}
          {:ok, entry} -> {[entry | entries], unread}
          :blank -> {entries, unread}
          {:unread, reason} -> {entries, [%{line: line, reason: reason} | unread]}
        end
      end)

    turns = entries |> Enum.reverse() |> Autopsy.Turns.build()
    name = Path.basename(path, ".jsonl")
    id = if parent, do: parent.id <> "/" <> name, else: name
    seat = if parent, do: parent.seat, else: Autopsy.Transcript.seat(turns.name)
    config = Map.put(config, :seat, seat)

    turns
    |> Map.merge(%{id: id, seat: seat, config: config, unread: Enum.reverse(unread)})
    |> then(&Map.put(&1, :events, Autopsy.Events.from_acts(&1.acts, config)))
  end

  defp write(nil, document), do: IO.puts(JSON.encode!(document))

  defp write(path, document) do
    case File.write(Path.expand(path), JSON.encode_to_iodata!(document)) do
      :ok ->
        IO.puts(:stderr, "wrote: #{path}")

      {:error, reason} ->
        halt(
          @failure_exit,
          "the output file #{path} cannot be written: #{:file.format_error(reason)}",
          "give a path in a directory that exists"
        )
    end
  end

  defp help do
    """
    autopsy.exs - reads Claude Code session transcripts as evidence for /in-autopsy

    Usage:
      elixir autopsy.exs <scope> [options]

    Scope (one is required):
      --seat MONIKER      The sessions of one seat, eg dc
      --session ID        One session, by its id or the start of its id
      --all               Every session of the project

    Window:
      --days N            Entries of the last N days (default: #{@default_days}; with --session, all of it)
      --since STAMP       Entries from a UTC stamp, eg 2026-11-25 or "2026-11-25 18:00Z"
      --until STAMP       Entries up to a UTC stamp

    Project values (a check that needs one does not run without it, and the output says so):
      --project NAME      The project folder, by a part of its name (default: the current directory's)
      --checkout PATH     The shared checkout's path
      --views GLOB        A path that only a verb writes, eg "intent/whiteboard/**" (repeat for more)
      --heavy REGEX       A command that needs a Desk slot, eg "^bin/devbin (build|gate|test)" (repeat for more)
      --mediator MONIKER  The seat through which hv is reached, eg vc

    Other:
      --patterns FILE     The pattern file of the run: `prose|command :: pattern :: source` on each line
      --subagents         Also read each session's subagents/ folder
      -o, --output FILE   Write the JSON document to FILE (default: stdout)
      -h, --help          Show this help

    Output:
      One JSON document: run, sessions, events, findings, checks_run, signals,
      patterns, and unread. Progress and the summary go to stderr. The script
      writes only the output file.
    """
  end
end

unless Code.ensure_loaded?(JSON) do
  IO.puts(
    :stderr,
    "error: this Elixir has no JSON module\n  remedy: install Elixir 1.18 or later; `elixir --version` shows the one in use"
  )

  System.halt(1)
end

Autopsy.CLI.main(System.argv())
