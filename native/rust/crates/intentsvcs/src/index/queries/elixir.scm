; Intent's symbol query for Elixir (ST0076 WP-01). It replaces
; tree-sitter-elixir's own tags.scm for this language, definitions and
; references alike. The capture vocabulary is stated in rust.scm and read by
; `index::symbols` for every language; this file adds one capture:
;   @subkind  a definition whose subkind is this node's text, so `def`, `defp`,
;             `defmacro`, `defguard` and `defdelegate` are kept apart as spelled
; and one property:
;   (#set! name.from "container")  a definition with no name of its own, named
;             after the module it sits in (`defstruct`, `defexception`)
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

; ---- references, unqualified (level 1)

; A module attribute is not a call: `@doc "..."` parses as one.
(unary_operator
  operator: "@"
  operand: (call target: (identifier) @ignore))

; The definition forms and the special forms are syntax, not calls.
(call
  target: (identifier) @ignore
  (#any-of? @ignore "def" "defp" "defdelegate" "defguard" "defguardp" "defmacro" "defmacrop" "defn" "defnp" "defmodule" "defprotocol" "defimpl" "defstruct" "defexception" "defoverridable" "alias" "case" "cond" "else" "for" "if" "import" "quote" "raise" "receive" "require" "reraise" "super" "throw" "try" "unless" "unquote" "unquote_splicing" "use" "with"))

(call
  target: [
    (identifier) @name
    (dot right: (identifier) @name)
  ]) @reference.call

(binary_operator
  operator: "|>"
  right: (identifier) @name) @reference.call

(alias) @name @reference.module
