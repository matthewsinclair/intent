# Elixir Anti-Patterns

This document outlines potential anti-patterns in Elixir, categorised into Code, Design, Process, and Meta-programming. Downloaded from https://hexdocs.pm/elixir/1.19.0-rc.0/what-anti-patterns.html.

## Code-related anti-patterns

This document outlines potential anti-patterns related to your code and particular Elixir idioms and features.

### Comments overuse

**Problem**

When you overuse comments or comment self-explanatory code, it can have the effect of making code _less readable_.

**Example**

```elixir
# Returns the Unix timestamp of 5 minutes from the current time
defp unix_five_min_from_now do
  # Get the current time
  now = DateTime.utc_now()

  # Convert it to a Unix timestamp
  unix_now = DateTime.to_unix(now, :second)

  # Add five minutes in seconds
  unix_now + (60 * 5)
end
```

**Refactoring**

Prefer clear and self-explanatory function names, module names, and variable names when possible. In the example above, the function name explains well what the function does, so you likely won't need the comment before it. The code also explains the operations well through variable names and clear function calls.

```elixir
@five_min_in_seconds 60 * 5

defp unix_five_min_from_now do
  now = DateTime.utc_now()
  unix_now = DateTime.to_unix(now, :second)
  unix_now + @five_min_in_seconds
end
```

We removed the unnecessary comments. We also added a `@five_min_in_seconds` module attribute, which serves the additional purpose of giving a name to the "magic" number `60 * 5`, making the code clearer and more expressive.

**Additional remarks**

Elixir makes a clear distinction between **documentation** and code comments. The language has built-in first-class support for documentation through `@doc`, `@moduledoc`, and more. See the "Writing documentation" guide for more information.

### Complex `else` clauses in `with`

**Problem**

This anti-pattern refers to `with` expressions that flatten all its error clauses into a single complex `else` block. This situation is harmful to the code readability and maintainability because it's difficult to know from which clause the error value came.

**Example**

An example of this anti-pattern, as shown below, is a function `open_decoded_file/1` that reads a Base64-encoded string content from a file and returns a decoded binary string. This function uses a `with` expression that needs to handle two possible errors, all of which are concentrated in a single complex `else` block.

```elixir
def open_decoded_file(path) do
  with {:ok, encoded} <- File.read(path),
       {:ok, decoded} <- Base.decode64(encoded) do
    {:ok, String.trim(decoded)}
  else
    {:error, _} -> {:error, :badfile}
    :error -> {:error, :badencoding}
  end
end
```

In the code above, it is unclear how each pattern on the left side of `<-` relates to their error at the end. The more patterns in a `with`, the less clear the code gets, and the more likely it is that unrelated failures will overlap each other.

**Refactoring**

In this situation, instead of concentrating all error handling within a single complex `else` block, it is better to normalise the return types in specific private functions. In this way, `with` can focus on the success case and the errors are normalised closer to where they happen, leading to better organised and maintainable code.

```elixir
def open_decoded_file(path) do
  with {:ok, encoded} <- file_read(path),
       {:ok, decoded} <- base_decode64(encoded) do
    {:ok, String.trim(decoded)}
  end
end

defp file_read(path) do
  case File.read(path) do
    {:ok, contents} -> {:ok, contents}
    {:error, _} -> {:error, :badfile}
  end
end

defp base_decode64(contents) do
  case Base.decode64(contents) do
    {:ok, decoded} -> {:ok, decoded}
    :error -> {:error, :badencoding}
  end
end
```

### Complex extractions in clauses

**Problem**

When we use multi-clause functions, it is possible to extract values in the clauses for further usage and for pattern matching/guard checking. This extraction itself does not represent an anti-pattern, but when you have _extractions made across several clauses and several arguments of the same function_, it becomes hard to know which extracted parts are used for pattern/guards and what is used only inside the function body. This anti-pattern is related to Unrelated multi-clause function, but with implications of its own. It impairs the code readability in a different way.

**Example**

The multi-clause function `drive/1` is extracting fields of an `%User{}` struct for usage in the clause expression (`age`) and for usage in the function body (`name`):

```elixir
def drive(%User{name: name, age: age}) when age >= 18 do
  "#{name} can drive"
end

def drive(%User{name: name, age: age}) when age < 18 do
  "#{name} cannot drive"
end
```

While the example above is small and does not constitute an anti-pattern, it is an example of mixed extraction and pattern matching. A situation where `drive/1` was more complex, having many more clauses, arguments, and extractions, would make it hard to know at a glance which variables are used for pattern/guards and which ones are not.

**Refactoring**

As shown below, a possible solution to this anti-pattern is to extract only pattern/guard related variables in the signature once you have many arguments or multiple clauses:

```elixir
def drive(%User{age: age} = user) when age >= 18 do
  %User{name: name} = user
  "#{name} can drive"
end

def drive(%User{age: age} = user) when age < 18 do
  %User{name: name} = user
  "#{name} cannot drive"
end
```

### Dynamic atom creation

**Problem**

An `Atom` is an Elixir basic type whose value is its own name. Atoms are often useful to identify resources or express the state, or result, of an operation. Creating atoms dynamically is not an anti-pattern by itself. However, atoms are not garbage collected by the Erlang Virtual Machine, so values of this type live in memory during a software's entire execution lifetime. The Erlang VM limits the number of atoms that can exist in an application by default to _1_048_576_, which is more than enough to cover all atoms defined in a program, but attempts to serve as an early limit for applications which are "leaking atoms" through dynamic creation.

For these reasons, creating atoms dynamically can be considered an anti-pattern when the developer has no control over how many atoms will be created during the software execution. This unpredictable scenario can expose the software to unexpected behaviour caused by excessive memory usage, or even by reaching the maximum number of _atoms_ possible.

**Example**

Picture yourself implementing code that converts string values into atoms. These strings could have been received from an external system, either as part of a request into our application, or as part of a response to your application. This dynamic and unpredictable scenario poses a security risk, as these uncontrolled conversions can potentially trigger out-of-memory errors.

```elixir
defmodule MyRequestHandler do
  def parse(%{"status" => status, "message" => message} = _payload) do
    %{status: String.to_atom(status), message: message}
  end
end
```

```elixir
iex> MyRequestHandler.parse(%{"status" => "ok", "message" => "all good"})
%{status: :ok, message: "all good"}
```

When we use the `String.to_atom/1` function to dynamically create an atom, it essentially gains potential access to create arbitrary atoms in our system, causing us to lose control over adhering to the limits established by the BEAM. This issue could be exploited by someone to create enough atoms to shut down a system.

**Refactoring**

To eliminate this anti-pattern, developers must either perform explicit conversions by mapping strings to atoms or replace the use of `String.to_atom/1` with `String.to_existing_atom/1`. An explicit conversion could be done as follows:

```elixir
defmodule MyRequestHandler do
  def parse(%{"status" => status, "message" => message} = _payload) do
    %{status: convert_status(status), message: message}
  end

  defp convert_status("ok"), do: :ok
  defp convert_status("error"), do: :error
  defp convert_status("redirect"), do: :redirect
end
```

```elixir
iex> MyRequestHandler.parse(%{"status" => "status_not_seen_anywhere", "message" => "all good"})
** (FunctionClauseError) no function clause matching in MyRequestHandler.convert_status/1
```

By explicitly listing all supported statuses, you guarantee only a limited number of conversions may happen. Passing an invalid status will lead to a function clause error.

An alternative is to use `String.to_existing_atom/1`, which will only convert a string to atom if the atom already exists in the system:

```elixir
defmodule MyRequestHandler do
  def parse(%{"status" => status, "message" => message} = _payload) do
    %{status: String.to_existing_atom(status), message: message}
  end
end
```

```elixir
iex> MyRequestHandler.parse(%{"status" => "status_not_seen_anywhere", "message" => "all good"})
** (ArgumentError) errors were found at the given arguments:

  * 1st argument: not an already existing atom
```

In such cases, passing an unknown status will raise as long as the status was not defined anywhere as an atom in the system. However, assuming `status` can be either `:ok`, `:error`, or `:redirect`, how can you guarantee those atoms exist? You must ensure those atoms exist somewhere **in the same module** where `String.to_existing_atom/1` is called. For example, if you had this code:

```elixir
defmodule MyRequestHandler do
  def parse(%{"status" => status, "message" => message} = _payload) do
    %{status: String.to_existing_atom(status), message: message}
  end

  def handle(%{status: status}) do
    case status do
      :ok -> ...
      :error -> ...
      :redirect -> ...
    end
  end
end
```

All valid statuses are defined as atoms within the same module, and that's enough. If you want to be explicit, you could also have a function that lists them:

```elixir
def valid_statuses do
  [:ok, :error, :redirect]
end
```

However, keep in mind using a module attribute or defining the atoms in the module body, outside of a function, are not sufficient, as the module body is only executed during compilation and it is not necessarily part of the compiled module loaded at runtime.

### Long parameter list

**Problem**

In a functional language like Elixir, functions tend to explicitly receive all inputs and return all relevant outputs, instead of relying on mutations or side-effects. As functions grow in complexity, the amount of arguments (parameters) they need to work with may grow, to a point where the function's interface becomes confusing and prone to errors during use.

**Example**

In the following example, the `loan/6` functions takes too many arguments, causing its interface to be confusing and potentially leading developers to introduce errors during calls to this function.

```elixir
defmodule Library do
  # Too many parameters that can be grouped!
  def loan(user_name, email, password, user_alias, book_title, book_ed) do
    ...
  end
end
```

**Refactoring**

To address this anti-pattern, related arguments can be grouped using key-value data structures, such as maps, structs, or even keyword lists in the case of optional arguments. This effectively reduces the number of arguments and the key-value data structures adds clarity to the caller.

For this particular example, the arguments to `loan/6` can be grouped into two different maps, thereby reducing its arity to `loan/2`:

```elixir
defmodule Library do
  def loan(%{name: name, email: email, password: password, alias: alias} = user,
           %{title: title, ed: ed} = book) do
    ...
  end
end
```

In some cases, the function with too many arguments may be a private function, which gives us more flexibility over how to separate the function arguments. One possible suggestion for such scenarios is to split the arguments in two maps (or tuples): one map keeps the data that may change, and the other keeps the data that won't change (read-only). This gives us a mechanical option to refactor the code.

Other times, a function may legitimately take half a dozen or more completely unrelated arguments. This may suggest the function is trying to do too much and would be better broken into multiple functions, each responsible for a smaller piece of the overall responsibility.

### Namespace trespassing

**Problem**

This anti-pattern manifests when a package author or a library defines modules outside of its "namespace". A library should use its name as a "prefix" for all of its modules. For example, a package named `:my_lib` should define all of its modules within the `MyLib` namespace, such as `MyLib.User`, `MyLib.SubModule`, `MyLib.Application`, and `MyLib` itself.

This is important because the Erlang VM can only load one instance of a module at a time. So if there are multiple libraries that define the same module, then they are incompatible with each other due to this limitation. By always using the library name as a prefix, it avoids module name clashes due to the unique prefix.

**Example**

This problem commonly manifests when writing an extension of another library. For example, imagine you are writing a package that adds authentication to Plug called `:plug_auth`. You must avoid defining modules within the `Plug` namespace:

```elixir
defmodule Plug.Auth do
  # ...
end
```

Even if `Plug` does not currently define a `Plug.Auth` module, it may add such a module in the future, which would ultimately conflict with `plug_auth`'s definition.

**Refactoring**

Given the package is named `:plug_auth`, it must define modules inside the `PlugAuth` namespace:

```elixir
defmodule PlugAuth do
  # ...
end
```

**Additional remarks**

There are few known exceptions to this anti-pattern:

- Protocol implementations are, by design, defined under the protocol namespace
- In some scenarios, the namespace owner may allow exceptions to this rule. For example, in Elixir itself, you defined custom Mix tasks by placing them under the `Mix.Tasks` namespace, such as `Mix.Tasks.PlugAuth`
- If you are the maintainer for both `plug` and `plug_auth`, then you may allow `plug_auth` to define modules with the `Plug` namespace, such as `Plug.Auth`. However, you are responsible for avoiding or managing any conflicts that may arise in the future

### Non-assertive map access

**Problem**

In Elixir, it is possible to access values from `Map`s, which are key-value data structures, either statically or dynamically.

When a key is expected to exist in a map, it must be accessed using the `map.key` notation, making it clear to developers (and the compiler) that the key must exist. If the key does not exist, an exception is raised (and in some cases also compiler warnings). This is also known as the static notation, as the key is known at the time of writing the code.

When a key is optional, the `map[:key]` notation must be used instead. This way, if the informed key does not exist, `nil` is returned. This is the dynamic notation, as it also supports dynamic key access, such as `map[some_var]`.

When you use `map[:key]` to access a key that always exists in the map, you are making the code less clear for developers and for the compiler, as they now need to work with the assumption the key may not be there. This mismatch may also make it harder to track certain bugs. If the key is unexpectedly missing, you will have a `nil` value propagate through the system, instead of raising on map access.

#### Comparison of map access notations

| Access notation | Key exists        | Key doesn't exist | Use case                                         |
| --------------- | ----------------- | ----------------- | ------------------------------------------------ |
| `map.key`       | Returns the value | Raises `KeyError` | Structs and maps with known atom keys            |
| `map[:key]`     | Returns the value | Returns `nil`     | Any `Access`-based data structure, optional keys |

**Example**

The function `plot/1` tries to draw a graphic to represent the position of a point in a Cartesian plane. This function receives a parameter of `Map` type with the point attributes, which can be a point of a 2D or 3D Cartesian coordinate system. This function uses dynamic access to retrieve values for the map keys:

```elixir
defmodule Graphics do
  def plot(point) do
    # Some other code...
    {point[:x], point[:y], point[:z]}
  end
end
```

```elixir
iex> point_2d = %{x: 2, y: 3}
%{x: 2, y: 3}
iex> point_3d = %{x: 5, y: 6, z: 7}
%{x: 5, y: 6, z: 7}
iex> Graphics.plot(point_2d)
{2, 3, nil}
iex> Graphics.plot(point_3d)
{5, 6, 7}
```

Given we want to plot both 2D and 3D points, the behaviour above is expected. But what happens if we forget to pass a point with either `:x` or `:y`?

```elixir
iex> bad_point = %{y: 3, z: 4}
%{y: 3, z: 4}
iex> Graphics.plot(bad_point)
{nil, 3, 4}
```

The behaviour above is unexpected because our function should not work with points without a `:x` key. This leads to subtle bugs, as we may now pass `nil` to another function, instead of raising early on, as shown next:

```elixir
iex> point_without_x = %{y: 10}
%{y: 10}
iex> {x, y, _} = Graphics.plot(point_without_x)
{nil, 10, nil}
iex> distance_from_origin = :math.sqrt(x * x + y * y)
** (ArithmeticError) bad argument in arithmetic expression
    :erlang.*(nil, nil)
```

The error above occurs later in the code because `nil` (from missing `:x`) is invalid for arithmetic operations, making it harder to identify the original issue.

**Refactoring**

To remove this anti-pattern, we must use the dynamic `map[:key]` syntax and the static `map.key` notation according to our requirements. We expect `:x` and `:y` to always exist, but not `:z`. The next code illustrates the refactoring of `plot/1`, removing this anti-pattern:

```elixir
defmodule Graphics do
  def plot(point) do
    # Some other code...
    {point.x, point.y, point[:z]}
  end
end
```

```elixir
iex> Graphics.plot(point_2d)
{2, 3, nil}
iex> Graphics.plot(bad_point)
** (KeyError) key :x not found in: %{y: 3, z: 4}
  graphic.ex:4: Graphics.plot/1
```

This is beneficial because:

1. It makes your expectations clear to others reading the code
2. It fails fast when required data is missing
3. It allows the compiler to provide warnings when accessing non-existent fields, particularly in compile-time structures like structs

Overall, the usage of `map.key` and `map[:key]` encode important information about your data structure, allowing developers to be clear about their intent. The `Access` module documentation also provides useful reference on this topic. You can also consider the `Map` module when working with maps of any keys, which contains functions for fetching keys (with or without default values), updating and removing keys, traversals, and more.

An alternative to refactor this anti-pattern is to use pattern matching, defining explicit clauses for 2D vs 3D points:

```elixir
defmodule Graphics do
  # 3d
  def plot(%{x: x, y: y, z: z}) do
    # Some other code...
    {x, y, z}
  end

  # 2d
  def plot(%{x: x, y: y}) do
    # Some other code...
    {x, y}
  end
end
```

Pattern-matching is specially useful when matching over multiple keys as well as on the values themselves at once. In the example above, the code will not only extract the values but also verify that the required keys exist. If we try to call `plot/1` with a map that doesn't have the required keys, we'll get a `FunctionClauseError`:

```elixir
iex> incomplete_point = %{x: 5}
%{x: 5}
iex> Graphics.plot(incomplete_point)
** (FunctionClauseError) no function clause matching in Graphics.plot/1

    The following arguments were given to Graphics.plot/1:

        # 1
        %{x: 5}
```

Another option is to use structs. By default, structs only support static access to its fields. In such scenarios, you may consider defining structs for both 2D and 3D points:

```elixir
defmodule Point2D do
  @enforce_keys [:x, :y]
  defstruct [x: nil, y: nil]
end
```

Generally speaking, structs are useful when sharing data structures across modules, at the cost of adding a compile time dependency between these modules. If module `A` uses a struct defined in module `B`, `A` must be recompiled if the fields in the struct `B` change.

In summary, Elixir provides several ways to access map values, each with different behaviours:

1. **Static access** (`map.key`): Fails fast when keys are missing, ideal for structs and maps with known atom keys
2. **Dynamic access** (`map[:key]`): Works with any `Access` data structure, suitable for optional fields, returns nil for missing keys
3. **Pattern matching**: Provides a powerful way to both extract values and ensure required map/struct keys exist in one operation

Choosing the right approach depends if the keys are known upfront or not. Static access and pattern matching are mostly equivalent (although pattern matching allows you to match on multiple keys at once, including matching on the struct name).

**Additional remarks**

This anti-pattern was formerly known as Accessing non-existent map/struct fields.

### Non-assertive pattern matching

**Problem**

Overall, Elixir systems are composed of many supervised processes, so the effects of an error are localised to a single process, and don't propagate to the entire application. A supervisor detects the failing process, reports it, and possibly restarts it. This anti-pattern arises when developers write defensive or imprecise code, capable of returning incorrect values which were not planned for, instead of programming in an assertive style through pattern matching and guards.

**Example**

The function `get_value/2` tries to extract a value from a specific key of a URL query string. As it is not implemented using pattern matching, `get_value/2` always returns a value, regardless of the format of the URL query string passed as a parameter in the call. Sometimes the returned value will be valid. However, if a URL query string with an unexpected format is used in the call, `get_value/2` will extract incorrect values from it:

```elixir
defmodule Extract do
  def get_value(string, desired_key) do
    parts = String.split(string, "&")

    Enum.find_value(parts, fn pair ->
      key_value = String.split(pair, "=")
      Enum.at(key_value, 0) == desired_key && Enum.at(key_value, 1)
    end)
  end
end
```

```elixir
# URL query string with the planned format - OK!
iex> Extract.get_value("name=Lucas&university=UFMG&lab=ASERG", "lab")
"ASERG"
iex> Extract.get_value("name=Lucas&university=UFMG&lab=ASERG", "university")
"UFMG"
# Unplanned URL query string format - Unplanned value extraction!
iex> Extract.get_value("name=Lucas&university=institution=UFMG&lab=ASERG", "university")
"institution"   # <= why not "institution=UFMG"? or only "UFMG"?
```

**Refactoring**

To remove this anti-pattern, `get_value/2` can be refactored through the use of pattern matching. So, if an unexpected URL query string format is used, the function will crash instead of returning an invalid value. This behaviour, shown below, allows clients to decide how to handle these errors and doesn't give a false impression that the code is working correctly when unexpected values are extracted:

```elixir
defmodule Extract do
  def get_value(string, desired_key) do
    parts = String.split(string, "&")

    Enum.find_value(parts, fn pair ->
      [key, value] = String.split(pair, "=") # <= pattern matching
      key == desired_key && value
    end)
  end
end
```

```elixir
# URL query string with the planned format - OK!
iex> Extract.get_value("name=Lucas&university=UFMG&lab=ASERG", "name")
"Lucas"
# Unplanned URL query string format - Crash explaining the problem to the client!
iex> Extract.get_value("name=Lucas&university=institution=UFMG&lab=ASERG", "university")
** (MatchError) no match of right hand side value: ["university", "institution", "UFMG"]
  extract.ex:7: anonymous fn/2 in Extract.get_value/2 # <= left hand: [key, value] pair
iex> Extract.get_value("name=Lucas&university&lab=ASERG", "university")
** (MatchError) no match of right hand side value: ["university"]
  extract.ex:7: anonymous fn/2 in Extract.get_value/2 # <= left hand: [key, value] pair
```

Elixir and pattern matching promote an assertive style of programming where you handle the known cases. Once an unexpected scenario arises, you can decide to address it accordingly based on practical examples, or conclude the scenario is indeed invalid and the exception is the desired choice.

`case/2` is another important construct in Elixir that help us write assertive code, by matching on specific patterns. For example, if a function returns `{:ok, ...}` or `{:error, ...}`, prefer to explicitly match on both patterns:

```elixir
case some_function(arg) do
  {:ok, value} -> # ...
  {:error, _} -> # ...
end
```

In particular, avoid matching solely on `_`, as shown below:

```elixir
case some_function(arg) do
  {:ok, value} -> # ...
  _ -> # ...
end
```

Matching on `_` is less clear in intent and it may hide bugs if `some_function/1` adds new return values in the future.

**Additional remarks**

This anti-pattern was formerly known as Speculative assumptions.

### Non-assertive truthiness

**Problem**

Elixir provides the concept of truthiness: `nil` and `false` are considered "falsy" and all other values are "truthy". Many constructs in the language, such as `&&/2`, `||/2`, and `!/1` handle truthy and falsy values. Using those operators is not an anti-pattern. However, using those operators when all operands are expected to be booleans, may be an anti-pattern.

**Example**

The simplest scenario where this anti-pattern manifests is in conditionals, such as:

```elixir
if is_binary(name) && is_integer(age) do
  # ...
else
  # ...
end
```

Given both operands of `&&/2` are booleans, the code is more generic than necessary, and potentially unclear.

**Refactoring**

To remove this anti-pattern, we can replace `&&/2`, `||/2`, and `!/1` by `and/2`, `or/2`, and `not/1` respectively. These operators assert at least their first argument is a boolean:

```elixir
if is_binary(name) and is_integer(age) do
  # ...
else
  # ...
end
```

This technique may be particularly important when working with Erlang code. Erlang does not have the concept of truthiness. It never returns `nil`, instead its functions may return `:error` or `:undefined` in places an Elixir developer would return `nil`. Therefore, to avoid accidentally interpreting `:undefined` or `:error` as a truthy value, you may prefer to use `and/2`, `or/2`, and `not/1` exclusively when interfacing with Erlang APIs.

### Structs with 32 fields or more

**Problem**

Structs in Elixir are implemented as compile-time maps, which have a predefined amount of fields. When structs have 32 or more fields, their internal representation in the Erlang Virtual Machines changes, potentially leading to bloating and higher memory usage.

**Example**

Any struct with 32 or more fields will be problematic:

```elixir
defmodule MyExample do
  defstruct [
    :field1,
    :field2,
    ...,
    :field35
  ]
end
```

The Erlang VM has two internal representations for maps: a flat map and a hash map. A flat map is represented internally as two tuples: one tuple containing the keys and another tuple holding the values. Whenever you update a flat map, the tuple keys are shared, reducing the amount of memory used by the update. A hash map has a more complex structure, which is efficient for a large amount of keys, but it does not share the key space.

Maps of up to 32 keys are represented as flat maps. All others are hash map. Structs _are_ maps (with a metadata field called `__struct__`) and so any struct with fewer than 32 fields is represented as a flat map. This allows us to optimise several struct operations, as we never add or remove fields to structs, we simply update them.

Furthermore, structs of the same name "instantiated" in the same module will share the same "tuple keys" at compilation times, as long as they have fewer than 32 fields. For example, in the following code:

```elixir
defmodule Example do
  def users do
    [%User{name: "John"}, %User{name: "Meg"}, ...]
  end
end
```

All user structs will point to the same tuple keys at compile-time, also reducing the memory cost of instantiating structs with `%MyStruct{...}` notation. This optimisation is also not available if the struct has 32 keys or more.

**Refactoring**

Removing this anti-pattern, in a nutshell, requires ensuring your struct has fewer than 32 fields. There are a few techniques you could apply:

- If the struct has "optional" fields, for example, fields which are initialised with nil, you could nest all optional fields into other field, called `:metadata`, `:optionals`, or similar. This could lead to benefits such as being able to use pattern matching to check if a field exists or not, instead of relying on `nil` values
- You could nest structs, by storing structs within other fields. Fields that are rarely read or written to are good candidates to be moved to a nested struct
- You could nest fields as tuples. For example, if two fields are always read or updated together, they could be moved to a tuple (or another composite data structure)

The challenge is to balance the changes above with API ergonomics, in particular, when fields may be frequently read and written to.

## Design-related anti-patterns

This document outlines potential anti-patterns related to your modules, functions, and the role they play within a codebase.

### Alternative return types

**Problem**

This anti-pattern refers to functions that receive options (typically as a _keyword list_ parameter) that drastically change their return type. Because options are optional and sometimes set dynamically, if they also change the return type, it may be hard to understand what the function actually returns.

**Example**

An example of this anti-pattern, as shown below, is when a function has many alternative return types, depending on the options received as a parameter.

```elixir
defmodule AlternativeInteger do
  @spec parse(String.t(), keyword()) :: integer() | {integer(), String.t()} | :error
  def parse(string, options \\ []) when is_list(options) do
    if Keyword.get(options, :discard_rest, false) do
      case Integer.parse(string) do
        {int, _rest} -> int
        :error -> :error
      end
    else
      Integer.parse(string)
    end
  end
end
```

```elixir
iex> AlternativeInteger.parse("13")
{13, ""}
iex> AlternativeInteger.parse("13", discard_rest: false)
{13, ""}
iex> AlternativeInteger.parse("13", discard_rest: true)
13
```

**Refactoring**

To refactor this anti-pattern, as shown next, add a specific function for each return type (for example, `parse_discard_rest/1`), no longer delegating this to options passed as arguments.

```elixir
defmodule AlternativeInteger do
  @spec parse(String.t()) :: {integer(), String.t()} | :error
  def parse(string) do
    Integer.parse(string)
  end

  @spec parse_discard_rest(String.t()) :: integer() | :error
  def parse_discard_rest(string) do
    case Integer.parse(string) do
      {int, _rest} -> int
      :error -> :error
    end
  end
end
```

```elixir
iex> AlternativeInteger.parse("13")
{13, ""}
iex> AlternativeInteger.parse_discard_rest("13")
13
```

### Boolean obsession

**Problem**

This anti-pattern happens when booleans are used instead of atoms to encode information. The usage of booleans themselves is not an anti-pattern, but whenever multiple booleans are used with overlapping states, replacing the booleans by atoms (or composite data types such as _tuples_) may lead to clearer code.

This is a special case of _Primitive obsession_, specific to boolean values.

**Example**

An example of this anti-pattern is a function that receives two or more options, such as `editor: true` and `admin: true`, to configure its behaviour in overlapping ways. In the code below, the `:editor` option has no effect if `:admin` is set, meaning that the `:admin` option has higher priority than `:editor`, and they are ultimately related.

```elixir
defmodule MyApp do
  def process(invoice, options \\ []) do
    cond do
      options[:admin] ->  # Is an admin
      options[:editor] -> # Is an editor
      true ->          # Is none
    end
  end
end
```

**Refactoring**

Instead of using multiple options, the code above could be refactored to receive a single option, called `:role`, that can be either `:admin`, `:editor`, or `:default`:

```elixir
defmodule MyApp do
  def process(invoice, options \\ []) do
    case Keyword.get(options, :role, :default) do
      :admin ->   # Is an admin
      :editor ->  # Is an editor
      :default -> # Is none
    end
  end
end
```

This anti-pattern may also happen in our own data structures. For example, we may define a `User` struct with two boolean fields, `:editor` and `:admin`, while a single field named `:role` may be preferred.

Finally, it is worth noting that using atoms may be preferred even when we have a single boolean argument/option. For example, consider an invoice which may be set as approved/unapproved. One option is to provide a function that expects a boolean:

```elixir
MyApp.update(invoice, approved: true)
```

However, using atoms may read better and make it simpler to add further states (such as pending) in the future:

```elixir
MyApp.update(invoice, status: :approved)
```

Remember booleans are internally represented as atoms. Therefore there is no performance penalty in one approach over the other.

### Exceptions for control-flow

**Problem**

This anti-pattern refers to code that uses `Exception`s for control flow. Exception handling itself does not represent an anti-pattern, but developers must prefer to use `case` and pattern matching to change the flow of their code, instead of `try/rescue`. In turn, library authors should provide developers with APIs to handle errors without relying on exception handling. When developers have no freedom to decide if an error is exceptional or not, this is considered an anti-pattern.

**Example**

An example of this anti-pattern, as shown below, is using `try/rescue` to deal with file operations:

```elixir
defmodule MyModule do
  def print_file(file) do
    try do
      IO.puts(File.read!(file))
    rescue
      e -> IO.puts(:stderr, Exception.message(e))
    end
  end
end
```

```elixir
iex> MyModule.print_file("valid_file")
This is a valid file!
:ok
iex> MyModule.print_file("invalid_file")
could not read file "invalid_file": no such file or directory
:ok
```

**Refactoring**

To refactor this anti-pattern, as shown next, use `File.read/1`, which returns tuples instead of raising when a file cannot be read:

```elixir
defmodule MyModule do
  def print_file(file) do
    case File.read(file) do
      {:ok, binary} -> IO.puts(binary)
      {:error, reason} -> IO.puts(:stderr, "could not read file #{file}: #{reason}")
    end
  end
end
```

This is only possible because the `File` module provides APIs for reading files with tuples as results (`File.read/1`), as well as a version that raises an exception (`File.read!/1`). The bang (exclamation point) is effectively part of Elixir's naming conventions.

Library authors are encouraged to follow the same practices. In practice, the bang variant is implemented on top of the non-raising version of the code. For example, `File.read!/1` is implemented as:

```elixir
def read!(path) do
  case read(path) do
    {:ok, binary} ->
      binary

    {:error, reason} ->
      raise File.Error, reason: reason, action: "read file", path: IO.chardata_to_string(path)
  end
end
```

A common practice followed by the community is to make the non-raising version return `{:ok, result}` or `{:error, Exception.t}`. For example, an HTTP client may return `{:ok, %HTTP.Response{}}` on success cases and `{:error, %HTTP.Error{}}` for failures, where `HTTP.Error` is implemented as an exception. This makes it convenient for anyone to raise an exception by simply calling `Kernel.raise/1`.

**Additional remarks**

This anti-pattern is of special importance to library authors and whenever writing functions that will be invoked by other developers and third-party code. Nevertheless, there are still scenarios where developers can afford to raise exceptions directly, for example:

- invalid arguments: it is expected that functions will raise for invalid arguments, as those are structural error and not semantic errors. For example, `File.read(123)` will always raise, because `123` is never a valid filename
- during tests, scripts, etc: those are common scenarios where you want your code to fail as soon as possible in case of errors. Using `!` functions, such as `File.read!/1`, allows you to do so quickly and with clear error messages
- some frameworks, such as Phoenix, allow developers to raise exceptions in their code and uses a protocol to convert these exceptions into semantic HTTP responses

This anti-pattern was formerly known as Using exceptions for control-flow.

### Primitive obsession

**Problem**

This anti-pattern happens when Elixir basic types (for example, _integer_, _float_, and _string_) are excessively used to carry structured information, rather than creating specific composite data types (for example, _tuples_, _maps_, and _structs_) that can better represent a domain.

**Example**

An example of this anti-pattern is the use of a single _string_ to represent an `Address`. An `Address` is a more complex structure than a simple basic (aka, primitive) value.

```elixir
defmodule MyApp do
  def extract_postal_code(address) when is_binary(address) do
    # Extract postal code with address...
  end

  def fill_in_country(address) when is_binary(address) do
    # Fill in missing country...
  end
end
```

While you may receive the `address` as a string from a database, web request, or a third-party, if you find yourself frequently manipulating or extracting information from the string, it is a good indicator you should convert the address into structured data.

Another example of this anti-pattern is using floating numbers to model money and currency, when richer data structures should be preferred.

**Refactoring**

Possible solutions to this anti-pattern is to use maps or structs to model our address. The example below creates an `Address` struct, better representing this domain through a composite type. Additionally, we introduce a `parse/1` function, that converts the string into an `Address`, which will simplify the logic of remaining functions. With this modification, we can extract each field of this composite type individually when needed.

```elixir
defmodule Address do
  defstruct [:street, :city, :state, :postal_code, :country]
end
```

```elixir
defmodule MyApp do
  def parse(address) when is_binary(address) do
    # Returns %Address{}
  end

  def extract_postal_code(%Address{} = address) do
    # Extract postal code with address...
  end

  def fill_in_country(%Address{} = address) do
    # Fill in missing country...
  end
end
```

### Unrelated multi-clause function

**Problem**

Using multi-clause functions is a powerful Elixir feature. However, some developers may abuse this feature to group _unrelated_ functionality, which is an anti-pattern.

**Example**

A frequent example of this usage of multi-clause functions occurs when developers mix unrelated business logic into the same function definition, in a way that the behaviour of each clause becomes completely distinct from the others. Such functions often have too broad specifications, making it difficult for other developers to understand and maintain them.

Some developers may use documentation mechanisms such as `@doc` annotations to compensate for poor code readability, however the documentation itself may end-up full of conditionals to describe how the function behaves for each different argument combination. This is a good indicator that the clauses are ultimately unrelated.

```elixir
@doc """
Updates a struct.

If given a product, it will...

If given an animal, it will...
"""
def update(%Product{count: count, material: material})  do
  # ...
end

def update(%Animal{count: count, skin: skin})  do
  # ...
end
```

If updating an animal is completely different from updating a product and requires a different set of rules, it may be worth splitting those over different functions or even different modules.

**Refactoring**

As shown below, a possible solution to this anti-pattern is to break the business rules that are mixed up in a single unrelated multi-clause function in simple functions. Each function can have a specific name and `@doc`, describing its behaviour and parameters received. While this refactoring sounds simple, it can impact the function's callers, so be careful!

```elixir
@doc """
Updates a product.

It will...
"""
def update_product(%Product{count: count, material: material}) do
  # ...
end

@doc """
Updates an animal.

It will...
"""
def update_animal(%Animal{count: count, skin: skin}) do
  # ...
end
```

These functions may still be implemented with multiple clauses, as long as the clauses group related functionality. For example, `update_product` could be in practice implemented as follows:

```elixir
def update_product(%Product{count: 0}) do
  # ...
end

def update_product(%Product{material: material})
    when material in ["metal", "glass"] do
  # ...
end

def update_product(%Product{material: material})
    when material not in ["metal", "glass"] do
  # ...
end
```

You can see this pattern in practice within Elixir itself. The `+/2` operator can add `Integer`s and `Float`s together, but not `String`s, which instead use the `<>/2` operator. In this sense, it is reasonable to handle integers and floats in the same operation, but strings are unrelated enough to deserve their own function.

You will also find examples in Elixir of functions that work with any struct, which would seemingly be an occurrence of this anti-pattern, such as `struct/2`:

```elixir
iex> struct(URI.parse("/foo/bar"), path: "/bar/baz")
%URI{
  scheme: nil,
  userinfo: nil,
  host: nil,
  port: nil,
  path: "/bar/baz",
  query: nil,
  fragment: nil
}
```

The difference here is that the `struct/2` function behaves precisely the same for any struct given, therefore there is no question of how the function handles different inputs. If the behaviour is clear and consistent for all inputs, then the anti-pattern does not take place.

### Using application configuration for libraries

**Problem**

The _application environment_ can be used to parameterise global values that can be used in an Elixir system. This mechanism can be very useful and therefore is not considered an anti-pattern by itself. However, library authors should avoid using the application environment to configure their library. The reason is exactly that the application environment is a **global** state, so there can only be a single value for each key in the environment for an application. This makes it impossible for multiple applications depending on the same library to configure the same aspect of the library in different ways.

**Example**

The `DashSplitter` module represents a library that configures the behaviour of its functions through the global application environment. These configurations are concentrated in the _config/config.exs_ file, shown below:

```elixir
import Config

config :app_config,
  parts: 3

import_config "#{config_env()}.exs"
```

One of the functions implemented by the `DashSplitter` library is `split/1`. This function aims to separate a string received via a parameter into a certain number of parts. The character used as a separator in `split/1` is always `"-"` and the number of parts the string is split into is defined globally by the application environment. This value is retrieved by the `split/1` function by calling `Application.fetch_env!/2`, as shown next:

```elixir
defmodule DashSplitter do
  def split(string) when is_binary(string) do
    parts = Application.fetch_env!(:app_config, :parts) # <= retrieve parameterised value
    String.split(string, "-", parts: parts)             # <= parts: 3
  end
end
```

Due to this parameterised value used by the `DashSplitter` library, all applications dependent on it can only use the `split/1` function with identical behaviour about the number of parts generated by string separation. Currently, this value is equal to 3, as we can see in the use examples shown below:

```elixir
iex> DashSplitter.split("Lucas-Francisco-Vegi")
["Lucas", "Francisco", "Vegi"]
iex> DashSplitter.split("Lucas-Francisco-da-Matta-Vegi")
["Lucas", "Francisco", "da-Matta-Vegi"]
```

**Refactoring**

To remove this anti-pattern, this type of configuration should be performed using a parameter passed to the function. The code shown below performs the refactoring of the `split/1` function by accepting keyword lists as a new optional parameter. With this new parameter, it is possible to modify the default behaviour of the function at the time of its call, allowing multiple different ways of using `split/2` within the same application:

```elixir
defmodule DashSplitter do
  def split(string, opts \\ []) when is_binary(string) and is_list(opts) do
    parts = Keyword.get(opts, :parts, 2) # <= default config of parts == 2
    String.split(string, "-", parts: parts)
  end
end
```

```elixir
iex> DashSplitter.split("Lucas-Francisco-da-Matta-Vegi", [parts: 5])
["Lucas", "Francisco", "da", "Matta", "Vegi"]
iex> DashSplitter.split("Lucas-Francisco-da-Matta-Vegi") #<= default config is used!
["Lucas", "Francisco-da-Matta-Vegi"]
```

Of course, not all uses of the application environment by libraries are incorrect. One example is using configuration to replace a component (or dependency) of a library by another that must behave the exact same. Consider a library that needs to parse CSV files. The library author may pick one package to use as default parser but allow its users to swap to different implementations via the application environment. At the end of the day, choosing a different CSV parser should not change the outcome, and library authors can even enforce this by defining behaviours with the exact semantics they expect.

**Additional remarks: Supervision trees**

In practice, libraries may require additional configuration beyond keyword lists. For example, if a library needs to start a supervision tree, how can the user of said library customise its supervision tree? Given the supervision tree itself is global (as it belongs to the library), library authors may be tempted to use the application configuration once more.

One solution is for the library to provide its own child specification, instead of starting the supervision tree itself. This allows the user to start all necessary processes under its own supervision tree, potentially passing custom configuration options during initialisation.

You can see this pattern in practice in projects like Nx and DNS Cluster. These libraries require that you list processes under your own supervision tree:

```elixir
children = [
  {DNSCluster, query: "my.subdomain"}
]
```

In such cases, if the users of `DNSCluster` need to configure DNSCluster per environment, they can be the ones reading from the application environment, without the library forcing them to:

```elixir
children = [
  {DNSCluster, query: Application.get_env(:my_app, :dns_cluster_query) || :ignore}
]
```

Some libraries, such as Ecto, allow you to pass your application name as an option (called `:otp_app` or similar) and then automatically read the environment from _your_ application. While this addresses the issue with the application environment being global, as they read from each individual application, it comes at the cost of some indirection, compared to the example above where users explicitly read their application environment from their own code, whenever desired.

**Additional remarks: Compile-time configuration**

A similar discussion entails compile-time configuration. What if a library author requires some configuration to be provided at compilation time?

Once again, instead of forcing users of your library to provide compile-time configuration, you may want to allow users of your library to generate the code themselves. That's the approach taken by libraries such as Ecto:

```elixir
defmodule MyApp.Repo do
  use Ecto.Repo, adapter: Ecto.Adapters.Postgres
end
```

Instead of forcing developers to share a single repository, Ecto allows its users to define as many repositories as they want. Given the `:adapter` configuration is required at compile-time, it is a required value on `use Ecto.Repo`. If developers want to configure the adapter per environment, then it is their choice:

```elixir
defmodule MyApp.Repo do
  use Ecto.Repo, adapter: Application.compile_env(:my_app, :repo_adapter)
end
```

On the other hand, code generation comes with its own anti-patterns, and must be considered carefully. That's to say: while using the application environment for libraries is discouraged, especially compile-time configuration, in some cases they may be the best option. For example, consider a library needs to parse CSV or JSON files to generate code based on data files. In such cases, it is best to provide reasonable defaults and make them customisable via the application environment, instead of asking each user of your library to generate the exact same code.

**Additional remarks: Mix tasks**

For Mix tasks and related tools, it may be necessary to provide per-project configuration. For example, imagine you have a `:linter` project, which supports setting the output file and the verbosity level. You may choose to configure it through application environment:

```elixir
config :linter,
  output_file: "/path/to/output.json",
  verbosity: 3
```

However, `Mix` allows tasks to read per-project configuration via `Mix.Project.config/0`. In this case, you can configure the `:linter` directly in the `mix.exs` file:

```elixir
def project do
  [
    app: :my_app,
    version: "1.0.0",
    linter: [
      output_file: "/path/to/output.json",
      verbosity: 3
    ],
    ...
  ]
end
```

Additionally, if a Mix task is available, you can also accept these options as command line arguments (see `OptionParser`):

```
mix linter --output-file /path/to/output.json --verbosity 3
```

## Process-related anti-patterns

This document outlines potential anti-patterns related to processes and process-based abstractions.

### Code organisation by process

**Problem**

This anti-pattern refers to code that is unnecessarily organised by processes. A process itself does not represent an anti-pattern, but it should only be used to model runtime properties (such as concurrency, access to shared resources, error isolation, etc). When you use a process for code organisation, it can create bottlenecks in the system.

**Example**

An example of this anti-pattern, as shown below, is a module that implements arithmetic operations (like `add` and `subtract`) by means of a `GenServer` process. If the number of calls to this single process grows, this code organisation can compromise the system performance, therefore becoming a bottleneck.

```elixir
defmodule Calculator do
  @moduledoc """
  Calculator that performs basic arithmetic operations.

  This code is unnecessarily organised in a GenServer process.
  """

  use GenServer

  def add(a, b, pid) do
    GenServer.call(pid, {:add, a, b})
  end

  def subtract(a, b, pid) do
    GenServer.call(pid, {:subtract, a, b})
  end

  @impl GenServer
  def init(init_arg) do
    {:ok, init_arg}
  end

  @impl GenServer
  def handle_call({:add, a, b}, _from, state) do
    {:reply, a + b, state}
  end

  def handle_call({:subtract, a, b}, _from, state) do
    {:reply, a - b, state}
  end
end
```

```elixir
iex> {:ok, pid} = GenServer.start_link(Calculator, :init)
{:ok, #PID<0.132.0>}
iex> Calculator.add(1, 5, pid)
6
iex> Calculator.subtract(2, 3, pid)
-1
```

**Refactoring**

In Elixir, as shown next, code organisation must be done only through modules and functions. Whenever possible, a library should not impose specific behaviour (such as parallelisation) on its users. It is better to delegate this behavioural decision to the developers of clients, thus increasing the potential for code reuse of a library.

```elixir
defmodule Calculator do
  def add(a, b) do
    a + b
  end

  def subtract(a, b) do
    a - b
  end
end
```

```elixir
iex> Calculator.add(1, 5)
6
iex> Calculator.subtract(2, 3)
-1
```

### Scattered process interfaces

**Problem**

In Elixir, the use of an `Agent`, a `GenServer`, or any other process abstraction is not an anti-pattern in itself. However, when the responsibility for direct interaction with a process is spread throughout the entire system, it can become problematic. This bad practice can increase the difficulty of code maintenance and make the code more prone to bugs.

**Example**

The following code seeks to illustrate this anti-pattern. The responsibility for interacting directly with the `Agent` is spread across four different modules (`A`, `B`, `C`, and `D`).

```elixir
defmodule A do
  def update(process) do
    # Some other code...
    Agent.update(process, fn _list -> 123 end)
  end
end
```

```elixir
defmodule B do
  def update(process) do
    # Some other code...
    Agent.update(process, fn content -> %{a: content} end)
  end
end
```

```elixir
defmodule C do
  def update(process) do
    # Some other code...
    Agent.update(process, fn content -> [:atom_value | content] end)
  end
end
```

```elixir
defmodule D do
  def get(process) do
    # Some other code...
    Agent.get(process, fn content -> content end)
  end
end
```

This spreading of responsibility can generate duplicated code and make code maintenance more difficult. Also, due to the lack of control over the format of the shared data, complex composed data can be shared. This freedom to use any format of data is dangerous and can induce developers to introduce bugs.

```elixir
# start an agent with initial state of an empty list
iex> {:ok, agent} = Agent.start_link(fn -> [] end)
{:ok, #PID<0.135.0>}

# many data formats (for example, List, Map, Integer, Atom) are
# combined through direct access spread across the entire system
iex> A.update(agent)
iex> B.update(agent)
iex> C.update(agent)

# state of shared information
iex> D.get(agent)
[:atom_value, %{a: 123}]
```

For a `GenServer` and other behaviours, this anti-pattern will manifest when scattering calls to `GenServer.call/3` and `GenServer.cast/2` throughout multiple modules, instead of encapsulating all the interaction with the `GenServer` in a single place.

**Refactoring**

Instead of spreading direct access to a process abstraction, such as `Agent`, over many places in the code, it is better to refactor this code by centralising the responsibility for interacting with a process in a single module. This refactoring improves maintainability by removing duplicated code; it also allows you to limit the accepted format for shared data, reducing bug-proneness. As shown below, the module `Foo.Bucket` is centralising the responsibility for interacting with the `Agent`. Any other place in the code that needs to access shared data must now delegate this action to `Foo.Bucket`. Also, `Foo.Bucket` now only allows data to be shared in `Map` format.

```elixir
defmodule Foo.Bucket do
  use Agent

  def start_link(_opts) do
    Agent.start_link(fn -> %{} end)
  end

  def get(bucket, key) do
    Agent.get(bucket, &Map.get(&1, key))
  end

  def put(bucket, key, value) do
    Agent.update(bucket, &Map.put(&1, key, value))
  end
end
```

The following are examples of how to delegate access to shared data (provided by an `Agent`) to `Foo.Bucket`.

```elixir
# start an agent through `Foo.Bucket`
iex> {:ok, bucket} = Foo.Bucket.start_link(%{})
{:ok, #PID<0.114.0>}

# add shared values to the keys `milk` and `beer`
iex> Foo.Bucket.put(bucket, "milk", 3)
iex> Foo.Bucket.put(bucket, "beer", 7)

# access shared data of specific keys
iex> Foo.Bucket.get(bucket, "beer")
7
iex> Foo.Bucket.get(bucket, "milk")
3
```

**Additional remarks**

This anti-pattern was formerly known as Agent obsession.

### Sending unnecessary data

**Problem**

Sending a message to a process can be an expensive operation if the message is big enough. That's because that message will be fully copied to the receiving process, which may be CPU and memory intensive. This is due to Erlang's "share nothing" architecture, where each process has its own memory, which simplifies and speeds up garbage collection.

This is more obvious when using `send/2`, `GenServer.call/3`, or the initial data in `GenServer.start_link/3`. Notably this also happens when using `spawn/1`, `Task.async/1`, `Task.async_stream/3`, and so on. It is more subtle here as the anonymous function passed to these functions captures the variables it references, and all captured variables will be copied over. By doing this, you can accidentally send way more data to a process than you actually need.

**Example**

Imagine you were to implement some simple reporting of IP addresses that made requests against your application. You want to do this asynchronously and not block processing, so you decide to use `spawn/1`. It may seem like a good idea to hand over the whole connection because we might need more data later. However passing the connection results in copying a lot of unnecessary data like the request body, params, etc.

```elixir
# log_request_ip send the ip to some external service
spawn(fn -> log_request_ip(conn) end)
```

This problem also occurs when accessing only the relevant parts:

```elixir
spawn(fn -> log_request_ip(conn.remote_ip) end)
```

This will still copy over all of `conn`, because the `conn` variable is being captured inside the spawned function. The function then extracts the `remote_ip` field, but only after the whole `conn` has been copied over.

`send/2` and the `GenServer` APIs also rely on message passing. In the example below, the `conn` is once again copied to the underlying `GenServer`:

```elixir
GenServer.cast(pid, {:report_ip_address, conn})
```

**Refactoring**

This anti-pattern has many potential remedies:

- Limit the data you send to the absolute necessary minimum instead of sending an entire struct. For example, don't send an entire `conn` struct if all you need is a couple of fields.
- If the only process that needs data is the one you are sending to, consider making the process fetch that data instead of passing it.
- Some abstractions, such as `:persistent_term`, allows you to share data between processes, as long as such data changes infrequently.

In our case, limiting the input data is a reasonable strategy. If all we need _right now_ is the IP address, then let's only work with that and make sure we're only passing the IP address into the closure, like so:

```elixir
ip_address = conn.remote_ip
spawn(fn -> log_request_ip(ip_address) end)
```

Or in the `GenServer` case:

```elixir
GenServer.cast(pid, {:report_ip_address, conn.remote_ip})
```

### Unsupervised processes

**Problem**

In Elixir, creating a process outside a supervision tree is not an anti-pattern in itself. However, when you spawn many long-running processes outside of supervision trees, this can make visibility and monitoring of these processes difficult, preventing developers from fully controlling their applications.

**Example**

The following code example seeks to illustrate a library responsible for maintaining a numerical `Counter` through a `GenServer` process _outside a supervision tree_. Multiple counters can be created simultaneously by a client (one process for each counter), making these _unsupervised_ processes difficult to manage. This can cause problems with the initialisation, restart, and shutdown of a system.

```elixir
defmodule Counter do
  @moduledoc """
  Global counter implemented through a GenServer process.
  """

  use GenServer

  @doc "Starts a counter process."
  def start_link(opts \\ []) do
    initial_value = Keyword.get(opts, :initial_value, 0)
    name = Keyword.get(opts, :name, __MODULE__)
    GenServer.start(__MODULE__, initial_value, name: name)
  end

  @doc "Gets the current value of the given counter."
  def get(pid_name \\ __MODULE__) do
    GenServer.call(pid_name, :get)
  end

  @doc "Bumps the value of the given counter."
  def bump(pid_name \\ __MODULE__, value) do
    GenServer.call(pid_name, {:bump, value})
  end

  @impl true
  def init(counter) do
    {:ok, counter}
  end

  @impl true
  def handle_call(:get, _from, counter) do
    {:reply, counter, counter}
  end

  def handle_call({:bump, value}, _from, counter) do
    {:reply, counter, counter + value}
  end
end
```

```elixir
iex> Counter.start_link()
{:ok, #PID<0.115.0>}
iex> Counter.get()
0
iex> Counter.start_link(initial_value: 15, name: :other_counter)
{:ok, #PID<0.120.0>}
iex> Counter.get(:other_counter)
15
iex> Counter.bump(:other_counter, -3)
12
iex> Counter.bump(Counter, 7)
7
```

**Refactoring**

To ensure that clients of a library have full control over their systems, regardless of the number of processes used and the lifetime of each one, all processes must be started inside a supervision tree. As shown below, this code uses a `Supervisor` as a supervision tree. When this Elixir application is started, two different counters (`Counter` and `:other_counter`) are also started as child processes of the `Supervisor` named `App.Supervisor`. One is initialised with `0`, the other with `15`. By means of this supervision tree, it is possible to manage the life cycle of all child processes (stopping or restarting each one), improving the visibility of the entire app.

```elixir
defmodule SupervisedProcess.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # With the default values for counter and name
      Counter,
      # With custom values for counter, name, and a custom ID
      Supervisor.child_spec(
        {Counter, name: :other_counter, initial_value: 15},
        id: :other_counter
      )
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: App.Supervisor)
  end
end
```

```elixir
iex> Supervisor.count_children(App.Supervisor)
%{active: 2, specs: 2, supervisors: 0, workers: 2}
iex> Counter.get(Counter)
0
iex> Counter.get(:other_counter)
15
iex> Counter.bump(Counter, 7)
7
iex> Supervisor.terminate_child(App.Supervisor, Counter)
iex> Supervisor.count_children(App.Supervisor) # Only one active child
%{active: 1, specs: 2, supervisors: 0, workers: 2}
iex> Counter.get(Counter) # The process was terminated
** (EXIT) no process: the process is not alive...
iex> Supervisor.restart_child(App.Supervisor, Counter)
iex> Counter.get(Counter) # After the restart, this process can be used again
0
```

## Meta-programming anti-patterns

This document outlines potential anti-patterns related to meta-programming.

### Compile-time dependencies

**Problem**

This anti-pattern is related to dependencies between files in Elixir. Because macros are used at compile-time, the use of any macro in Elixir adds a compile-time dependency to the module that defines the macro.

However, when macros are used in the body of a module, the arguments to the macro themselves may become compile-time dependencies. These dependencies may lead to dependency graphs where changing a single file causes several files to be recompiled.

**Example**

Let's take the `Plug` library as an example. The `Plug` project allows you to specify several modules, also known as plugs, which will be invoked whenever there is a request. As a user of `Plug`, you would use it as follows:

```elixir
defmodule MyApp do
  use Plug.Builder

  plug MyApp.Authentication
end
```

And imagine `Plug` has the following definitions of the macros above (simplified):

```elixir
defmodule Plug.Builder do
  defmacro __using__(_opts) do
    quote do
      Module.register_attribute(__MODULE__, :plugs, accumulate: true)
      @before_compile Plug.Builder
    end
  end

  defmacro plug(mod) do
    quote do
      @plugs unquote(mod)
    end
  end

  ...
end
```

The implementation accumulates all modules inside the `@plugs` module attribute. Right before the module is compiled, `Plug.Builder` will reads all modules stored in `@plugs` and compile them into a function, like this:

```elixir
def call(conn, _opts) do
  MyApp.Authentication.call(conn)
end
```

The trouble with the code above is that, because the `plug MyApp.Authentication` was invoked at compile-time, the module `MyApp.Authentication` is now a compile-time dependency of `MyApp`, even though `MyApp.Authentication` is never used at compile-time. If `MyApp.Authentication` depends on other modules, even at runtime, this can now lead to a large recompilation graph in case of changes.

**Refactoring**

To address this anti-pattern, a macro can expand literals within the context they are meant to be used, as follows:

```elixir
defmacro plug(mod) do
  mod = Macro.expand_literals(mod, %{__CALLER__ | function: {:call, 2}})

  quote do
    @plugs unquote(mod)
  end
end
```

In the example above, since `mod` is used only within the `call/2` function, we prematurely expand module reference as if it was inside the `call/2` function. Now `MyApp.Authentication` is only a runtime dependency of `MyApp`, no longer a compile-time one.

Note, however, the above must only be done if your macros do not attempt to invoke any function, access any struct, or any other metadata of the module at compile-time. If you interact with the module given to a macro anywhere outside of definition of a function, then you effectively have a compile-time dependency. And, even though you generally want to avoid them, it is not always possible.

In actual projects, developers may use `mix xref trace path/to/file.ex` to execute a file and have it print information about which modules it depends on, and if those modules are compile-time, runtime, or export dependencies. See `mix xref` for more information.

### Large code generation

**Problem**

This anti-pattern is related to macros that generate too much code. When a macro generates a large amount of code, it impacts how the compiler and/or the runtime work. The reason for this is that Elixir may have to expand, compile, and execute the code multiple times, which will make compilation slower and the resulting compiled artifacts larger.

**Example**

Imagine you are defining a router for a web application, where you could have macros like `get/2`. On every invocation of the macro (which could be hundreds), the code inside `get/2` will be expanded and compiled, which can generate a large volume of code overall.

```elixir
defmodule Routes do
  defmacro get(route, handler) do
    quote do
      route = unquote(route)
      handler = unquote(handler)

      if not is_binary(route) do
        raise ArgumentError, "route must be a binary"
      end

      if not is_atom(handler) do
        raise ArgumentError, "handler must be a module"
      end

      @store_route_for_compilation {route, handler}
    end
  end
end
```

**Refactoring**

To remove this anti-pattern, the developer should simplify the macro, delegating part of its work to other functions. As shown below, by encapsulating the code inside `quote/1` inside the function `__define__/3` instead, we reduce the code that is expanded and compiled on every invocation of the macro, and instead we dispatch to a function to do the bulk of the work.

```elixir
defmodule Routes do
  defmacro get(route, handler) do
    quote do
      Routes.__define__(__MODULE__, unquote(route), unquote(handler))
    end
  end

  def __define__(module, route, handler) do
    if not is_binary(route) do
      raise ArgumentError, "route must be a binary"
    end

    if not is_atom(handler) do
      raise ArgumentError, "handler must be a module"
    end

    Module.put_attribute(module, :store_route_for_compilation, {route, handler})
  end
end
```

### Unnecessary macros

**Problem**

_Macros_ are powerful meta-programming mechanisms that can be used in Elixir to extend the language. While using macros is not an anti-pattern in itself, this meta-programming mechanism should only be used when absolutely necessary. Whenever a macro is used, but it would have been possible to solve the same problem using functions or other existing Elixir structures, the code becomes unnecessarily more complex and less readable. Because macros are more difficult to implement and reason about, their indiscriminate use can compromise the evolution of a system, reducing its maintainability.

**Example**

The `MyMath` module implements the `sum/2` macro to perform the sum of two numbers received as parameters. While this code has no syntax errors and can be executed correctly to get the desired result, it is unnecessarily more complex. By implementing this functionality as a macro rather than a conventional function, the code became less clear:

```elixir
defmodule MyMath do
  defmacro sum(v1, v2) do
    quote do
      unquote(v1) + unquote(v2)
    end
  end
end
```

```elixir
iex> require MyMath
MyMath
iex> MyMath.sum(3, 5)
8
iex> MyMath.sum(3 + 1, 5 + 6)
15
```

**Refactoring**

To remove this anti-pattern, the developer must replace the unnecessary macro with structures that are simpler to write and understand, such as named functions. The code shown below is the result of the refactoring of the previous example. Basically, the `sum/2` macro has been transformed into a conventional named function. Note that the `require/2` call is no longer needed:

```elixir
defmodule MyMath do
  def sum(v1, v2) do # <= The macro became a named function
    v1 + v2
  end
end
```

```elixir
iex> MyMath.sum(3, 5)
8
iex> MyMath.sum(3+1, 5+6)
15
```

### `use` instead of `import`

**Problem**

Elixir has mechanisms such as `import/1`, `alias/1`, and `use/1` to establish dependencies between modules. Code implemented with these mechanisms does not characterise a smell by itself. However, while the `import/1` and `alias/1` directives have lexical scope and only facilitate a module calling functions of another, the `use/1` directive has a _broader scope_, which can be problematic.

The `use/1` directive allows a module to inject any type of code into another, including propagating dependencies. In this way, using the `use/1` directive makes code harder to read, because to understand exactly what will happen when it references a module, it is necessary to have knowledge of the internal details of the referenced module.

**Example**

The code shown below is an example of this anti-pattern. It defines three modules -- `ModuleA`, `Library`, and `ClientApp`. `ClientApp` is reusing code from the `Library` via the `use/1` directive, but is unaware of its internal details. This makes it harder for the author of `ClientApp` to visualise which modules and functionality are now available within its module. To make matters worse, `Library` also imports `ModuleA`, which defines a `foo/0` function that conflicts with a local function defined in `ClientApp`:

```elixir
defmodule ModuleA do
  def foo do
    "From Module A"
  end
end
```

```elixir
defmodule Library do
  defmacro __using__(_opts) do
    quote do
      import Library
      import ModuleA  # <= propagating dependencies!
    end
  end

  def from_lib do
    "From Library"
  end
end
```

```elixir
defmodule ClientApp do
  use Library

  def foo do
    "Local function from client app"
  end

  def from_client_app do
    from_lib() <> " - " <> foo()
  end
end
```

When we try to compile `ClientApp`, Elixir detects the conflict and throws the following error:

```
error: imported ModuleA.foo/0 conflicts with local function
  └ client_app.ex:4:
```

**Refactoring**

To remove this anti-pattern, we recommend library authors avoid providing `__using__/1` callbacks whenever it can be replaced by `alias/1` or `import/1` directives. In the following code, we assume `use Library` is no longer available and `ClientApp` was refactored in this way, and with that, the code is clearer and the conflict as previously shown no longer exists:

```elixir
defmodule ClientApp do
  import Library

  def foo do
    "Local function from client app"
  end

  def from_client_app do
    from_lib() <> " - " <> foo()
  end
end
```

```elixir
iex> ClientApp.from_client_app()
"From Library - Local function from client app"
```

**Additional remarks**

In situations where you need to do more than importing and aliasing modules, providing `use MyModule` may be necessary, as it provides a common extension point within the Elixir ecosystem.

Therefore, to provide guidance and clarity, we recommend library authors to include an admonition block in their `@moduledoc` that explains how `use MyModule` impacts the developer's code. As an example, the `GenServer` documentation outlines:

> #### `use GenServer`
>
> When you `use GenServer`, the `GenServer` module will set `@behaviour GenServer` and define a `child_spec/1` function, so your module can be used as a child in a supervision tree.

Think of this summary as a "Nutrition facts label" for code generation. Make sure to only list changes made to the public API of the module. For example, if `use Library` sets an internal attribute called `@_some_module_info` and this attribute is never meant to be public, avoid documenting it in the nutrition facts.

For convenience, the markup notation to generate the admonition block above is this:

```markdown
> #### `use GenServer` {: .info}
>
> When you `use GenServer`, the `GenServer` module will
> set `@behaviour GenServer` and define a `child_spec/1`
> function, so your module can be used as a child
> in a supervision tree.
```

### Untracked compile-time dependencies

**Problem**

This anti-pattern is the opposite of "Compile-time dependencies" and it happens when a compile-time dependency is accidentally bypassed, making the Elixir compiler unable to track dependencies and recompile files correctly. This happens when building aliases (in other words, module names) dynamically, either within a module or within a macro.

**Example**

For example, imagine you invoke a module at compile-time, you could write it as such:

```elixir
defmodule MyModule do
  SomeOtherModule.example()
end
```

In this case, Elixir knows `MyModule` is invoked `SomeOtherModule.example/0` outside of a function, and therefore at compile-time.

Elixir can also track module names even during dynamic calls:

```elixir
defmodule MyModule do
  mods = [OtherModule.Foo, OtherModule.Bar]

  for mod <- mods do
    mod.example()
  end
end
```

In the previous example, even though Elixir does not know which modules the function `example/0` was invoked on, it knows the modules `OtherModule.Foo` and `OtherModule.Bar` are referred outside of a function and therefore they become compile-time dependencies. If any of them change, Elixir will recompile `MyModule` itself.

However, you should not programatically generate the module names themselves, as that would make it impossible for Elixir to track them. More precisely, do not do this:

```elixir
defmodule MyModule do
  parts = [:Foo, :Bar]

  for part <- parts do
    Module.concat(OtherModule, part).example()
  end
end
```

In this case, because the whole module was generated, Elixir sees a dependency only to `OtherModule`, never to `OtherModule.Foo` and `OtherModule.Bar`, potentially leading to inconsistencies when recompiling projects.

A similar bug can happen when abusing the property that aliases are simply atoms, defining the atoms directly. In the case below, Elixir never sees the aliases, leading to untracked compile-time dependencies:

```elixir
defmodule MyModule do
  mods = [:"Elixir.OtherModule.Foo", :"Elixir.OtherModule.Bar"]

  for mod <- mods do
    mod.example()
  end
end
```

**Refactoring**

To address this anti-pattern, you should avoid defining module names programatically. For example, if you need to dispatch to multiple modules, do so by using full module names.

Instead of:

```elixir
defmodule MyModule do
  parts = [:Foo, :Bar]

  for part <- parts do
    Module.concat(OtherModule, part).example()
  end
end
```

Do:

```elixir
defmodule MyModule do
  mods = [OtherModule.Foo, OtherModule.Bar]

  for mod <- mods do
    mod.example()
  end
end
```

If you really need to define modules dynamically, you can do so via meta-programming, building the whole module name at compile-time:

```elixir
defmodule MyMacro do
  defmacro call_examples(parts) do
    for part <- parts do
      quote do
        # This builds OtherModule.Foo at compile-time
        OtherModule.unquote(part).example()
      end
    end
  end
end

defmodule MyModule do
  import MyMacro
  call_examples [:Foo, :Bar]
end
```

In actual projects, developers may use `mix xref trace path/to/file.ex` to execute a file and have it print information about which modules it depends on, and if those modules are compile-time, runtime, or export dependencies. This can help you debug if the dependencies are being properly tracked in relation to external modules. See `mix xref` for more information.
