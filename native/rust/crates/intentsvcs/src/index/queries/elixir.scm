; Intent's symbol query for Elixir (ST0076 WP-01 and WP-03). It replaces
; tree-sitter-elixir's own tags.scm for this language, definitions and
; references alike. The capture vocabulary is stated in rust.scm and read by
; `index::symbols` for every language; this file adds these captures:
;   @subkind         a row whose subkind is this node's text, so `def`, `defp`,
;                    `defmacro`, `defguard` and `defdelegate` are kept apart as
;                    spelled, and a directive is its own word (`use`, `alias`)
;   @qualifier.self  `__MODULE__` as a qualifier, standing for the enclosing
;                    module's name
;   @self            `__MODULE__` inside a name, expanded the same way and never
;                    a qualifier
;   @name.base       the base a brace entry's name is written after
;   @arity           an arity as written (`&Repo.get/2`), read in place of a count
;   @alias           an alias form, whose @alias.path, @alias.base and @alias.as
;                    put short names in scope; the @alias.as node is never a row
; and these properties:
;   (#set! name.from "container")   a definition with no name of its own, named
;                                   after the module it sits in (`defstruct`)
;   (#set! name.expand "alias")     the row's name, not a qualifier, has the
;                                   file's aliases and `__MODULE__` expanded
;   (#set! arity.piped "true")      one argument arrives by the pipe, unwritten
;   (#set! alias.scope "do_block")  an alias holds to the end of the nearest
;                                   ancestor of this kind
; Where two patterns match one name node for one row kind, the EARLIER pattern
; in this file wins.

; ---- modules, protocols and implementations: definitions that contain others

(call
  target: (identifier) @ignore
  (arguments . (alias) @name)
  (#eq? @ignore "defmodule")) @definition.module @container.module

(call
  target: (identifier) @ignore
  (arguments . (alias) @name)
  (#eq? @ignore "defprotocol")) @definition.protocol @container.protocol

(call
  target: (identifier) @ignore
  (arguments . (alias) @name)
  (#eq? @ignore "defimpl")) @definition.impl @container.impl

; ---- functions, macros, guards and delegates: one row per clause

; `def name(args)`, and `def name(args) when guard`
(call
  target: (identifier) @subkind
  (arguments
    .
    [
      (call target: (identifier) @name (arguments) @params)
      (binary_operator
        left: (call target: (identifier) @name (arguments) @params)
        operator: "when")
    ])
  (#any-of? @subkind "def" "defp" "defmacro" "defmacrop" "defguard" "defguardp" "defdelegate" "defn" "defnp")) @definition.function

; `def name do` and `def name, do: ...`: no argument list, so arity 0
(call
  target: (identifier) @subkind
  (arguments
    .
    [
      (identifier) @name @params
      (binary_operator
        left: (identifier) @name @params
        operator: "when")
    ])
  (#any-of? @subkind "def" "defp" "defmacro" "defmacrop" "defguard" "defguardp" "defdelegate" "defn" "defnp")) @definition.function

(arguments (binary_operator operator: "\\\\") @param.optional)
(arguments (_) @param)

(call
  target: (identifier) @ignore
  (#eq? @ignore "defstruct")
  (#set! name.from "container")) @definition.struct

(call
  target: (identifier) @ignore
  (#eq? @ignore "defexception")
  (#set! name.from "container")) @definition.exception

; ---- the file's own aliases (WP-03)
;
; An `alias` form expands the first segment of every qualifier after it, to the
; end of the do-block that holds it: `alias MyApp.Repo` makes `Repo.get` mean
; `MyApp.Repo.get`. `as:`, the brace form, `__MODULE__` and an Erlang module
; (`alias :crypto, as: Crypto`) are the forms whose meaning the file's own syntax
; makes certain; `import` and `use` expand nothing. The short name an `as:`
; gives is not a reference to a module. An alias written in a `do:` keyword
; body has no do-block of its own, so it holds to the end of the enclosing one:
; a known gap.
(call
  target: (identifier) @ignore
  (arguments
    .
    [
      (alias) @alias.path
      (atom) @alias.path
      (dot left: (identifier) @qualifier.self right: (alias)) @alias.path
      (dot left: (alias) @alias.base right: (tuple (alias) @alias.path))
      (dot left: (identifier) @qualifier.self @alias.base right: (tuple (alias) @alias.path))
    ]
    (keywords (pair value: (alias) @alias.as))?)
  (#eq? @ignore "alias")
  (#set! alias.scope "do_block")) @alias

; ---- references, qualified (level 2, WP-03)
;
; A reference's span is its name node, as in rust.scm. These come before the
; unqualified patterns, so a qualified row wins its name node, and the pipe and
; capture patterns come before the plain remote call for the same reason.

; `x |> Repo.get(id)`: the piped value is the first argument, so the arity is
; one more than written.
(binary_operator
  operator: "|>"
  right: (call
    target: (dot
      left: [(alias) (atom)] @qualifier
      right: (identifier) @name @reference.call)
    (arguments) @params)
  (#set! arity.piped "true"))

(binary_operator
  operator: "|>"
  right: (call
    target: (dot
      left: (identifier) @qualifier.self
      right: (identifier) @name @reference.call)
    (arguments) @params)
  (#eq? @qualifier.self "__MODULE__")
  (#set! arity.piped "true"))

; `&Repo.get/2`: the arity is the one written after the slash.
(unary_operator
  operator: "&"
  operand: (binary_operator
    left: (call
      target: (dot
        left: [(alias) (atom)] @qualifier
        right: (identifier) @name @reference.call))
    operator: "/"
    right: (_) @arity))

; `__MODULE__.f(x)`: the qualifier is the enclosing module.
(call
  target: (dot
    left: (identifier) @qualifier.self
    right: (identifier) @name @reference.call)
  (arguments) @params
  (#eq? @qualifier.self "__MODULE__"))

; A remote call keeps its module: `Repo.get(id)` is `get` qualified by `Repo`,
; and `:ets.new(name, opts)` is `new` qualified by `:ets`.
(call
  target: (dot
    left: [(alias) (atom)] @qualifier
    right: (identifier) @name @reference.call)
  (arguments) @params)

(call
  target: (dot
    left: [(alias) (atom)] @qualifier
    right: (identifier) @name @reference.call))

; ---- directives (level 1, WP-03)
;
; `use`, `import`, `require` and `alias` each name a module. The row's name is
; the whole module with the file's aliases and `__MODULE__` expanded, or an
; Erlang module as written (`:crypto`), and the directive is its subkind.
(call
  target: (identifier) @subkind
  (arguments . [(alias) (atom)] @name @reference.module)
  (#any-of? @subkind "use" "import" "require" "alias")
  (#set! name.expand "alias"))

; `alias __MODULE__.Sub`: named from the enclosing module, and the `Sub` it
; writes is part of that name rather than a module reference of its own.
(call
  target: (identifier) @subkind
  (arguments
    .
    (dot left: (identifier) @self right: (alias) @ignore) @name @reference.module)
  (#eq? @self "__MODULE__")
  (#any-of? @subkind "use" "import" "require" "alias")
  (#set! name.expand "alias"))

; `alias MyApp.{Repo, Mailer}` and `alias __MODULE__.{Router, Endpoint}`: one
; row per entry, each named whole from its base, and the base is part of those
; names rather than a module reference of its own.
(call
  target: (identifier) @subkind
  (arguments
    .
    (dot
      left: (alias) @name.base @ignore
      right: (tuple (alias) @name @reference.module)))
  (#any-of? @subkind "use" "import" "require" "alias")
  (#set! name.expand "alias"))

(call
  target: (identifier) @subkind
  (arguments
    .
    (dot
      left: (identifier) @name.base @self
      right: (tuple (alias) @name @reference.module)))
  (#eq? @self "__MODULE__")
  (#any-of? @subkind "use" "import" "require" "alias")
  (#set! name.expand "alias"))

; ---- references, unqualified (level 1)

; A module attribute is not a call: `@doc "..."` parses as one.
(unary_operator
  operator: "@"
  operand: (call target: (identifier) @ignore))

; The definition forms and the special forms are syntax, not calls.
(call
  target: (identifier) @ignore
  (#any-of? @ignore "def" "defp" "defdelegate" "defguard" "defguardp" "defmacro" "defmacrop" "defn" "defnp" "defmodule" "defprotocol" "defimpl" "defstruct" "defexception" "defoverridable" "alias" "case" "cond" "else" "for" "if" "import" "quote" "raise" "receive" "require" "reraise" "super" "throw" "try" "unless" "unquote" "unquote_splicing" "use" "with"))

; An unqualified reference spans its name node too: a bare pipe target's
; enclosing node is the whole pipeline, which starts lines before the name.
(call
  target: [
    (identifier) @name @reference.call
    (dot right: (identifier) @name @reference.call)
  ])

(binary_operator
  operator: "|>"
  right: (identifier) @name @reference.call)

; `&price/1`: a local capture is a reference to the function it names, with the
; arity written after the slash, so a function used only by capture is used.
(unary_operator
  operator: "&"
  operand: (binary_operator
    left: (identifier) @name @reference.call
    operator: "/"
    right: (_) @arity))

; A module named anywhere else, with the file's aliases expanded.
((alias) @name @reference.module
  (#set! name.expand "alias"))
