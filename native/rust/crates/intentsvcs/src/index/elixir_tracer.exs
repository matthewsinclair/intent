# Intent's Elixir level-3 tracer (ST0076 WP-06). `intent index resolve` writes
# this file into its own build directory, loads it with `elixir -r`, and names
# the module to `mix compile --tracer`. It writes one record per line to the
# file `INTENT_TRACE` names, its fields separated by the unit separator (0x1f):
#
#   S file                               the compiler started a file
#   E file                               and finished it
#   C file line column module name arity a call
#   M file line column module            a module named in the code
#   Q file line                          an imported call inside a quote
#   F file message                       a fault in this tracer
#
# A line or column is empty where the event's meta carries none. The reader in
# `index/elixir_tracer.rs` makes every decision about these records; this file
# only writes down what the compiler said.
#
# IT NEVER RAISES, because a tracer that raises aborts the project's compile.
# A fault is written as an `F` record, and the reader fails the run on it. A
# fault that cannot be written either goes to stderr under a prefix the reader
# looks for.

defmodule IntentLevel3Tracer do
  @moduledoc false

  @calls [:remote_function, :remote_macro, :imported_function, :imported_macro]
  @locals [:local_function, :local_macro]
  @directives [:import, :require]
  @recorded [:start, :stop, :alias_reference, :alias, :imported_quoted] ++
              @calls ++ @locals ++ @directives
  @unwritten "intent-level3-tracer: the trace could not be written:"

  def trace(event, env) do
    record(event, env)
  rescue
    e -> fault(env, event, Exception.format(:error, e, __STACKTRACE__))
  catch
    kind, value -> fault(env, event, Exception.format(kind, value, __STACKTRACE__))
  end

  defp record(:start, env), do: write!(["S", env.file])
  defp record(:stop, env), do: write!(["E", env.file])

  defp record({kind, meta, module, name, arity}, env) when kind in @calls,
    do: call(env, meta, module, name, arity)

  defp record({kind, meta, name, arity}, env) when kind in @locals,
    do: call(env, meta, env.module, name, arity)

  defp record({:alias_reference, meta, module}, env), do: module(env, meta, module)
  defp record({:alias, meta, module, _as, _opts}, env), do: module(env, meta, module)

  defp record({kind, meta, module, _opts}, env) when kind in @directives,
    do: module(env, meta, module)

  defp record({:imported_quoted, meta, _module, _name, _arities}, env),
    do: write!(["Q", env.file, position(meta, :line)])

  # A kind this tracer records, arriving in a shape it does not know, is a
  # fault and never an event to pass over: passed over, its references would
  # vanish from files that still read as compiled.
  defp record(event, _env)
       when is_tuple(event) and tuple_size(event) > 0 and elem(event, 0) in @recorded,
       do: raise("an event of a kind this tracer records, in a shape it does not know")

  defp record(_event, _env), do: :ok

  defp call(env, meta, module, name, arity) do
    write!([
      "C",
      env.file,
      position(meta, :line),
      position(meta, :column),
      inspect(module),
      Atom.to_string(name),
      Integer.to_string(arity)
    ])
  end

  defp module(env, meta, module) do
    write!(["M", env.file, position(meta, :line), position(meta, :column), inspect(module)])
  end

  defp position(meta, key) do
    case Keyword.get(meta, key) do
      n when is_integer(n) and n > 0 -> Integer.to_string(n)
      _ -> ""
    end
  end

  defp fault(env, event, message) do
    write!([
      "F",
      env.file,
      flatten("#{message} on #{inspect(event, limit: 20, printable_limit: 200)}")
    ])
  rescue
    e -> unwritten(env, message, Exception.message(e))
  catch
    kind, value -> unwritten(env, message, Exception.format_banner(kind, value))
  end

  defp unwritten(env, message, why) do
    IO.puts(:stderr, flatten("#{@unwritten} #{why}; the fault in #{env.file} was: #{message}"))
  end

  defp flatten(text), do: String.replace(text, ["\n", <<31>>], " ")

  defp write!(fields) do
    device = :persistent_term.get(__MODULE__)

    case :file.write(device, [Enum.intersperse(fields, <<31>>), ?\n]) do
      :ok -> :ok
      {:error, reason} -> raise "the trace file refused a write: #{inspect(reason)}"
    end
  end
end

:persistent_term.put(
  IntentLevel3Tracer,
  File.open!(System.fetch_env!("INTENT_TRACE"), [:write, :binary])
)
