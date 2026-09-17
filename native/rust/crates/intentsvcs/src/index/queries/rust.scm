; Intent's symbol query for Rust (ST0076 WP-01). It replaces tree-sitter-rust's
; own tags.scm for this language, definitions and references alike.
;
; The capture vocabulary, read by `index::symbols` for every language:
;   @definition.<subkind>  a definition row; the node is its span
;   @reference.<subkind>   a reference row; the node is its span
;   @name                  the row's name
;   @container.<kind>      a node other rows inside it belong to
;   @trait                 the trait an impl container implements
;   @params                the node whose @param children give the arity
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

; ---- references, unqualified (level 1)

(call_expression
  function: (identifier) @name) @reference.call

(call_expression
  function: (field_expression
    field: (field_identifier) @name)) @reference.call

(macro_invocation
  macro: (identifier) @name) @reference.macro

; A reference spans what was written, so these capture the name node, never the
; impl it sits in.
(impl_item
  trait: (type_identifier) @name @reference.type)

(impl_item
  type: (type_identifier) @name @reference.type)
