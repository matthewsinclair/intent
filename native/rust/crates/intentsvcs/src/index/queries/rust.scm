; Intent's symbol query for Rust (ST0076 WP-01 and WP-02). It replaces tree-sitter-rust's
; own tags.scm for this language, definitions and references alike.
;
; The capture vocabulary, read by `index::symbols` for every language:
;   @definition.<subkind>  a definition row; the node is its span
;   @reference.<subkind>   a reference row; the node is its span
;   @name                  the row's name
;   @container.<kind>      a node other rows inside it belong to
;   @trait                 the trait an impl container implements
;   @params                the node whose @param children give the arity
;   @qualifier             the path a reference was written with, as written;
;                          a row that has one is level 2
;   @param, @param.optional one parameter, counted in the arity
;   @ignore                a name node that is never a row
; Where two patterns match one name node for one row kind, the EARLIER pattern
; in this file wins, so the narrower pattern is written first.

; ---- containers

; A generic or path-qualified type contributes its last name: `impl<T> Foo<T>`
; and `impl a::Foo` are both containers named `Foo`.
(impl_item
  trait: [
    (type_identifier) @trait
    (generic_type type: (type_identifier) @trait)
    (scoped_type_identifier name: (type_identifier) @trait)
  ]
  type: [
    (type_identifier) @name
    (generic_type type: (type_identifier) @name)
    (scoped_type_identifier name: (type_identifier) @name)
  ]) @container.impl

(impl_item
  type: [
    (type_identifier) @name
    (generic_type type: (type_identifier) @name)
    (scoped_type_identifier name: (type_identifier) @name)
  ]
  !trait) @container.impl

; ---- methods and associated functions: inside an impl or a trait

(impl_item
  body: (declaration_list
    (function_item
      name: (identifier) @name
      parameters: (parameters (self_parameter)) @params) @definition.method))

(trait_item
  body: (declaration_list
    (function_item
      name: (identifier) @name
      parameters: (parameters (self_parameter)) @params) @definition.method))

(trait_item
  body: (declaration_list
    (function_signature_item
      name: (identifier) @name
      parameters: (parameters (self_parameter)) @params) @definition.method))

(impl_item
  body: (declaration_list
    (function_item
      name: (identifier) @name
      parameters: (parameters) @params) @definition.assoc_fn))

(trait_item
  body: (declaration_list
    (function_item
      name: (identifier) @name
      parameters: (parameters) @params) @definition.assoc_fn))

(trait_item
  body: (declaration_list
    (function_signature_item
      name: (identifier) @name
      parameters: (parameters) @params) @definition.assoc_fn))

; ---- functions: everywhere else

(function_item
  name: (identifier) @name
  parameters: (parameters) @params) @definition.function

(parameters (parameter) @param)
(parameters (variadic_parameter) @param)

; ---- types, and the definitions that contain others

(struct_item
  name: (type_identifier) @name) @definition.struct @container.struct

(union_item
  name: (type_identifier) @name) @definition.union @container.union

(enum_item
  name: (type_identifier) @name) @definition.enum @container.enum

(enum_variant
  name: (identifier) @name) @definition.variant @container.variant

(field_declaration
  name: (field_identifier) @name) @definition.field

(trait_item
  name: (type_identifier) @name) @definition.trait @container.trait

(type_item
  name: (type_identifier) @name) @definition.type

(associated_type
  name: (type_identifier) @name) @definition.type

(const_item
  name: (identifier) @name) @definition.const

(static_item
  name: (identifier) @name) @definition.static

(mod_item
  name: (identifier) @name) @definition.module @container.module

(macro_definition
  name: (identifier) @name) @definition.macro

; ---- references, qualified (level 2): the path as written, never resolved

; `AddressError::new(..)`, `crate::views::render(..)`, `Vec::<u8>::new(..)`
(call_expression
  function: (scoped_identifier
    path: (_) @qualifier
    name: (identifier) @name @reference.call))

(call_expression
  function: (generic_function
    function: (scoped_identifier
      path: (_) @qualifier
      name: (identifier) @name @reference.call)))

; `use a::b::C;` and `use a::b::{C, D};`
(use_declaration
  argument: (scoped_identifier
    path: (_) @qualifier
    name: (identifier) @name @reference.use))

(scoped_use_list
  path: (_) @qualifier
  list: (use_list
    (identifier) @name @reference.use))

; **THE SEGMENT A QUALIFIER ENDS IN IS A REFERENCE, AND THE SEGMENTS BEFORE IT
; ARE NOT.** In `a::b::Foo::new()`, `Foo` is used and `a` and `b` are its path.
; A pattern here matches at every depth of a chain, so "two paths deep" means
; any segment with at least one more segment between it and the tail.
[
  (scoped_identifier path: (scoped_identifier path: (scoped_identifier name: (identifier) @ignore)))
  (scoped_identifier path: (scoped_identifier path: (identifier) @ignore))
]

; A `use` path and a type's path name modules, and none of their segments is
; a use of its own: the imported name and the type are the references.
[
  (use_declaration argument: (scoped_identifier path: (scoped_identifier name: (identifier) @ignore)))
  (use_declaration argument: (scoped_identifier path: (identifier) @ignore))
  (use_as_clause path: (scoped_identifier path: (scoped_identifier name: (identifier) @ignore)))
  (use_as_clause path: (scoped_identifier path: (identifier) @ignore))
  (scoped_use_list path: (scoped_identifier name: (identifier) @ignore))
  (scoped_use_list path: (scoped_identifier path: (identifier) @ignore))
  (use_list (scoped_identifier path: (identifier) @ignore))
  (scoped_type_identifier path: (scoped_identifier name: (identifier) @ignore))
  (scoped_type_identifier path: (scoped_identifier path: (identifier) @ignore))
]

; `Self` names the type being implemented, which its impl already references.
((identifier) @ignore
  (#eq? @ignore "Self"))

; `AddressError::Refused`, `Self::LIMIT`: a path that is not called
(scoped_identifier
  path: (_) @qualifier
  name: (identifier) @name @reference.path)

; `Foo` in `Foo::new()`: a qualifier of one segment is itself a reference
(scoped_identifier
  path: (identifier) @name @reference.path)

; `store::Address` as a type
(scoped_type_identifier
  path: (_) @qualifier
  name: (type_identifier) @name @reference.type)

; ---- references inside a macro invocation: a token tree has no syntax, so
; these read the token shape, and anything that shape cannot name is a `token`

; `a::b(..)` and `Error::Refused` inside a macro: the qualifier is the one
; segment before the name
(token_tree
  [(identifier) (crate) (self) (super)] @qualifier
  .
  "::"
  .
  (identifier) @name @reference.call
  .
  (token_tree . "("))

(token_tree
  [(identifier) (crate) (self) (super)] @qualifier
  .
  "::"
  .
  (identifier) @name @reference.path)

; `AddressError` in `AddressError::Refused` inside a macro: a name before `::`
; is a path reference, qualified by the one segment before it where one is
; written
(token_tree
  (identifier) @name @reference.path
  .
  "::")

; `x.is_local()` inside a macro
(token_tree
  "."
  .
  (identifier) @name @reference.call
  .
  (token_tree . "("))

; `nearest_project(root)` and `format!(..)` inside a macro
(token_tree
  (identifier) @name @reference.macro
  .
  "!"
  .
  (token_tree))

(token_tree
  (identifier) @name @reference.call
  .
  (token_tree . "("))

(token_tree
  (identifier) @name @reference.token)

; ---- references, unqualified (level 1)

(call_expression
  function: (identifier) @name @reference.call)

(call_expression
  function: (field_expression
    field: (field_identifier) @name @reference.call))

(call_expression
  function: (generic_function
    function: [
      (identifier) @name @reference.call
      (field_expression field: (field_identifier) @name @reference.call)
    ]))

(macro_invocation
  macro: (identifier) @name @reference.macro)

; A type parameter's declaration names it; it is not a use.
(type_parameter
  name: (type_identifier) @ignore)

; Every other type name, the impl's type and trait included. A reference
; spans what was written, so it captures the name node.
(type_identifier) @name @reference.type
