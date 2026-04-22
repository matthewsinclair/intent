---
id: IN-RS-CODE-003
language: rust
category: code
severity: recommendation
title: Traits over enums for behavioural polymorphism
summary: >
  Use enums for closed sets of _data_ variants and traits for open sets of
  _behaviour_. When every enum variant triggers a distinct block inside a
  sprawling `match`, the shape is a trait in disguise.
principles:
  - pfic
applies_when:
  - "Enums whose variants each carry different data and trigger different handlers"
  - "`match` expressions that dispatch to per-variant logic longer than 2-3 lines per arm"
  - "New variants require touching every `match` in the codebase"
applies_to:
  - "src/**/*.rs"
does_not_apply_when:
  - "Closed data shapes — AST nodes, message types, state-machine states — where exhaustive `match` is the point"
  - "Enums with fewer than 3 variants or uniform arm bodies"
  - "FFI or serialisation boundaries where the tagged union shape is part of the contract"
references:
  - IN-AG-PFIC-001
related_rules:
  - IN-RS-CODE-004
aliases: []
tags:
  - rust
  - traits
  - polymorphism
status: active
version: 1
---

# Traits over enums for behavioural polymorphism

An enum is the right shape for data. A trait is the right shape for behaviour. The test: when you add a new variant, does every `match` need to change?

## Problem

Enum dispatch compiles well and runs fast, so the first instinct for polymorphism in Rust is `enum Handler { Email(EmailHandler), Slack(SlackHandler), ... }` plus a big `match` in every caller. The pattern works until a new variant arrives — then every `match` in the crate demands a new arm, and compilation errors accumulate at places that had nothing to do with the new feature. What looked like ergonomics was actually a Highlander violation: the same dispatch table copied at every call site.

Traits invert the flow: the behaviour lives with the implementing type, and callers write `handler.handle(event)` without caring which concrete handler they hold. Adding a new handler adds one `impl`, not N match arms.

## Detection

Static signals:

- Enum variants each wrap a different struct (`Email(EmailHandler)`, `Slack(SlackHandler)`) and `match` arms each call a differently-named method on the wrapped value.
- `match` arms longer than 2-3 lines each; the body of each arm is a dispatch, not a data reshape.
- Adding a variant triggers compilation errors in 3+ unrelated files.
- A `handle` / `process` / `execute` method on the enum that is nothing but a `match` forwarding to inner types.

Clippy does not lint this directly; structural review is the detection mechanism.

## Bad

```rust
pub enum Handler {
  Email(EmailHandler),
  Slack(SlackHandler),
  Webhook(WebhookHandler),
}

impl Handler {
  pub fn handle(&self, event: &Event) -> Result<(), Error> {
    match self {
      Handler::Email(h)   => h.send_email(event),
      Handler::Slack(h)   => h.post_message(event),
      Handler::Webhook(h) => h.post_payload(event),
    }
  }
}
```

Every new channel (SMS, Teams, Discord) requires a new variant _and_ teaching every `match` in the dispatcher to forward to it.

## Good

```rust
pub trait Handler {
  fn handle(&self, event: &Event) -> Result<(), Error>;
}

pub struct EmailHandler   { /* ... */ }
pub struct SlackHandler   { /* ... */ }
pub struct WebhookHandler { /* ... */ }

impl Handler for EmailHandler   { fn handle(&self, e: &Event) -> Result<(), Error> { /* ... */ } }
impl Handler for SlackHandler   { fn handle(&self, e: &Event) -> Result<(), Error> { /* ... */ } }
impl Handler for WebhookHandler { fn handle(&self, e: &Event) -> Result<(), Error> { /* ... */ } }

pub fn dispatch(handler: &dyn Handler, event: &Event) -> Result<(), Error> {
  handler.handle(event)
}
```

Adding an SMS handler is one new struct and one new `impl`. No `match` arm, no ripple.

## When This Applies

- Behavioural polymorphism: "I have several things that all respond to the same verb".
- Plugin and extension points where external crates may provide new implementations.
- Rules, handlers, strategies, policies — the classic "strategy pattern" shapes.

## When This Does Not Apply

- Data shapes with a small, truly closed set of variants: `enum HttpMethod { Get, Post, Put, ... }`, `enum TokenKind { ... }`, AST nodes.
- State machines where exhaustive `match` enforces that every state is handled.
- Deserialisation targets where the variant set matches an external protocol (JSON discriminated unions, protobuf oneof).
- Small enums (2-3 variants) whose arm bodies are uniform or one-liners; a trait adds more ceremony than it removes.

## Further Reading

- The Rust Programming Language, ch. 17 "Object-Oriented Programming Features of Rust" (<https://doc.rust-lang.org/book/ch17-00-oop.html>)
- Rust API Guidelines, "Types are predictable" (<https://rust-lang.github.io/api-guidelines/>)
- "Casey Muratori on Hierarchies vs. Flatness" — arguments for enums over traits (the contrary view is legitimate for closed data)
- IN-AG-PFIC-001 — pattern matching is primary; traits are not replacements, they are a different tool
