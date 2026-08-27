//! **An error that says what to DO about itself, and ONE rendering of it.**
//!
//! AC-04.4 asks that every error be typed and render a remedy with its full
//! cause chain. That was true of `FacadeError` and of nothing else, and the way
//! it was untrue is the finding rather than the count (vc, Highlander finding
//! 4): **there were two conventions, and the difference between them is
//! invisible until something renders them uniformly.**
//!
//! - Four enums carried `remedy()` as a method, rendered separately from the
//!   message.
//! - `InstallError` baked its remedy INTO its own `#[error(...)]` string, so
//!   the remedy arrived inside `{e}`. Anything rendering `error: {e}\n  remedy:
//!   {}` against it prints the remedy twice; `intent info` printed it once only
//!   because it did not use a remedy at all.
//! - Three carried none, so a caller had nothing to offer and wrote a remedy at
//!   the call site or omitted one.
//!
//! **A shape that can represent the same fact two ways eventually holds both**,
//! which is the argument this repository has made about `scope`/`scope_legacy`
//! and about `file`/`legacy`. This is that argument applied to errors.
//!
//! The trait carries the CONTENT as the required method and the FORM as a
//! default one. Content differs per error and must; form differs per error only
//! by accident, and every accident is a message an operator reads differently
//! depending on which subsystem failed.

/// The three tokens the operator-facing error format is made of.
///
/// **ONE HOME FOR THE SPELLING, BECAUSE THE FORMAT HAS MORE PRODUCERS THAN THIS
/// TRAIT** (vc, 2026-08-27, closing the residue of Highlander finding F4). The
/// trait guarantees the format for anything that IMPLEMENTS it, and the module
/// header is right that an implementor cannot invent a second rendering. But
/// `Failure::Error(String)` takes a finished string, and the renderer builds 76
/// operator-facing `error: ` literals by hand -- so the format has a second
/// producer that the trait cannot reach. `error_literal_shape.rs` now checks
/// those agree, and to parse them it had to spell the same three tokens a THIRD
/// time.
///
/// **THAT IS THE FINDING ONE LEVEL UP: a checker that re-encodes what it checks
/// agrees with itself.** Rename `remedy: ` here with the tokens copied into the
/// test, and the trait's output moves, the 76 literals do not, and the checker
/// greps for the old spelling and passes -- reporting agreement at the exact
/// moment it was lost. Reading them from here removes that: the test's parse and
/// the trait's output cannot disagree about the spelling, because there is only
/// one.
///
/// Not `const fn` and not assembled here: these are the tokens, and the ORDER
/// they appear in is the property `error_literal_shape.rs` asserts. Two
/// different things, deliberately in two places.
pub const ERROR_PREFIX: &str = "error: ";
/// The cause-chain token, with the two-space indent the operator sees.
pub const CAUSED_BY_PREFIX: &str = "  caused by: ";
/// The remedy token, with the two-space indent the operator sees.
pub const REMEDY_PREFIX: &str = "  remedy: ";

/// An error that can state its own remedy.
///
/// **`remedy` is required and `render` is defaulted, which is the whole point
/// of it being a trait rather than a convention.** An implementor cannot forget
/// the remedy -- the compiler asks for it -- and cannot accidentally invent a
/// second rendering, because it does not have to write one.
pub trait Remedy {
  /// What the operator should DO. Never a restatement of the message, and
  /// never advice that cannot be acted on: a remedy that reads as a lead and
  /// goes nowhere is worse than none, because it costs a search first.
  fn remedy(&self) -> String;

  /// **THE rendering: message, full cause chain, remedy.**
  ///
  /// Lowercase `error:` is v2's voice (INV-01, issue 0023) and the cause chain
  /// is walked rather than summarised -- the innermost cause is usually the
  /// only line that names a path, a permission or a syscall, and it is the
  /// line a summary drops.
  fn render(&self) -> String
  where
    Self: std::error::Error,
  {
    let mut out = format!("{ERROR_PREFIX}{self}");
    let mut source = std::error::Error::source(self);
    while let Some(cause) = source {
      out.push_str(&format!("\n{CAUSED_BY_PREFIX}{cause}"));
      source = cause.source();
    }
    out.push_str(&format!("\n{REMEDY_PREFIX}{}", self.remedy()));
    out
  }
}
