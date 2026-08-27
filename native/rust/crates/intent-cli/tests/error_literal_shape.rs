//! **THE SECOND PRODUCER OF THE ERROR FORMAT, WHICH `remedy.rs` DOES NOT
//! REACH.**
//!
//! `remedy.rs` says an implementor *"cannot accidentally invent a second
//! rendering, because it does not have to write one"*, and `MODULES.md` calls
//! it *"the ONE rendering of message + full cause chain + remedy"*. **Both are
//! true of the TRAIT and false of the SURFACE**, and the gap is one type:
//! `Failure::Error(String)` takes a FINISHED string, so a call site can write
//! the format itself -- and dozens do, in the same file that also calls
//! `Failure::Error(e.render())` a few lines away.
//!
//! Two producers of one format, agreeing today by convention, with nothing
//! enforcing that they keep agreeing. `error_remedies.rs` is pairwise and
//! rigorous and covers FACADE errors -- typed things implementing the trait.
//! It cannot see a formatted string.
//!
//! **THIS IS vc's HIGHLANDER FINDING 4 SURVIVING ITS OWN FIX** (raised by vc,
//! 2026-08-27, against a fix vc argued for). That finding was *two conventions,
//! and the difference is invisible until something renders them uniformly*. The
//! trait removed the NEED to hand-write a rendering; it never removed the
//! ABILITY, and `Failure::Error(String)` is where the ability lives. So the
//! original finding survived, in the half nobody looked at.
//!
//! # What this does NOT propose
//!
//! **Not that the hand-written literals are wrong, and not that they should
//! become typed errors.** Most are CLI argument validation -- `--body and
//! --from both give the issue its prose` has no error TYPE to hang a trait on,
//! `Remedy::render` requires `Self: std::error::Error`, and minting an enum
//! variant per validation to satisfy a shape would be worse than the problem.
//! Nothing here asks anyone to touch one of them.
//!
//! It turns *they agree by convention* into *they agree by construction*, and
//! nothing more.
//!
//! # What it gates, and what it only reports
//!
//! **GATES the ORDER**, because a literal out of order is a defect in the one
//! property the trait exists to guarantee: `error: ` opens, any `caused by: `
//! sits in the middle, `remedy: ` is last.
//!
//! **REPORTS the remedy-less**, and does not gate them. Whether a given error
//! is legitimately remedy-less has not been decided by anyone, and a gate that
//! forces a remedy onto an error that has none would buy the shape by making
//! the estate write advice that cannot be acted on -- which `unwired`'s own doc
//! calls worse than no remedy at all. The number is printed so the question can
//! be asked; it is not answered here.

use std::path::Path;

/// The renderer's source.
fn renderer() -> String {
  let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/render.rs");
  std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("could not read the renderer at {}: {e}", path.display()))
}

/// The source with the test module and whole-line comments removed.
///
/// **THE LIMIT IS STATED RATHER THAN DISCOVERED:** this drops lines whose
/// TRIMMED form opens a comment, which is where every prose mention in this
/// file actually lives. A `//` appearing after code on the same line is not
/// handled, and a literal opened on such a line would still be scanned. That is
/// a false POSITIVE risk only -- it can make this check complain about prose,
/// never make it miss a real literal -- and a complaint gets read while a miss
/// does not.
fn scannable(src: &str) -> String {
  let body = match src.find("#[cfg(test)]") {
    Some(i) => &src[..i],
    None => src,
  };
  body
    .lines()
    .map(|l| {
      let t = l.trim_start();
      if t.starts_with("//") || t.starts_with("*") {
        ""
      } else {
        l
      }
    })
    .collect::<Vec<_>>()
    .join("\n")
}

/// Every operator-facing error literal, as the operator would SEE it.
///
/// Returns the logical lines of each literal: the `\n` escape is what splits an
/// operator's screen, and a Rust `\`-at-end-of-line continuation is invisible to
/// them, so it is joined away here. Reading the file rather than running the
/// binary is deliberate -- most of these sites need an argument shape that
/// cannot be produced without driving the whole command.
fn error_literals(src: &str) -> Vec<Vec<String>> {
  let bytes: Vec<char> = src.chars().collect();
  let mut out = Vec::new();
  let mut i = 0;
  while i < bytes.len() {
    // A literal we care about always opens exactly `"error: `.
    if bytes[i] == '"' && src[i..].starts_with("\"error: ") {
      let mut j = i + 1;
      let mut logical: Vec<String> = Vec::new();
      let mut current = String::new();
      while j < bytes.len() {
        match bytes[j] {
          '"' => break,
          '\\' if j + 1 < bytes.len() => {
            match bytes[j + 1] {
              // The escape that ends an operator's line.
              'n' => {
                logical.push(std::mem::take(&mut current));
                j += 2;
              }
              // A Rust line continuation: backslash, newline, then the next
              // line's indentation. The operator never sees any of it.
              '\n' => {
                j += 2;
                while j < bytes.len() && (bytes[j] == ' ' || bytes[j] == '\t') {
                  j += 1;
                }
              }
              c => {
                current.push(c);
                j += 2;
              }
            }
          }
          c => {
            current.push(c);
            j += 1;
          }
        }
      }
      logical.push(current);
      out.push(logical);
      i = j + 1;
    } else {
      i += 1;
    }
  }
  out
}

/// **THE FIXTURE PROVES ITSELF.** A scan finding nothing would satisfy every
/// assertion below and would look exactly like a clean surface -- which is the
/// failure this estate has now met three times in one day.
#[test]
fn the_scan_reaches_the_renderer() {
  let found = error_literals(&scannable(&renderer()));
  assert!(
    found.len() > 20,
    "the scan found only {} error literal(s) in the renderer. Every assertion here iterates that set, so a scan reaching nothing passes in silence",
    found.len()
  );
}

/// **The parser is driven on constructed input, in both directions**, because
/// everything below is only as good as it is -- and it is the piece most likely
/// to be quietly wrong.
#[test]
fn the_parser_splits_on_what_an_operator_sees_and_not_on_source_layout() {
  // One logical line.
  let one = error_literals(r#"f("error: plain")"#);
  assert_eq!(one, vec![vec!["error: plain".to_string()]]);

  // `\n` splits; the continuation does not.
  let two = error_literals("f(\"error: a\\n  remedy: b\")");
  assert_eq!(
    two,
    vec![vec!["error: a".to_string(), "  remedy: b".to_string()]]
  );

  // A `\`-continuation joins, and the next line's indent is dropped.
  let joined = error_literals("f(\"error: a \\\n     tail\")");
  assert_eq!(joined, vec![vec!["error: a tail".to_string()]]);

  // An escaped quote does not end the literal.
  let quoted = error_literals(r#"f("error: a \"b\" c")"#);
  assert_eq!(quoted, vec![vec![r#"error: a "b" c"#.to_string()]]);
}

/// **THE GATE: the order the trait guarantees is the order the literals use.**
#[test]
fn every_hand_written_error_literal_has_the_trait_s_shape() {
  let literals = error_literals(&scannable(&renderer()));
  let mut wrong = Vec::new();

  for lit in &literals {
    let joined = lit.join("\\n");
    let remedy_at = lit
      .iter()
      .position(|l| l.trim_start().starts_with("remedy: "));
    let cause_positions: Vec<usize> = lit
      .iter()
      .enumerate()
      .filter(|(_, l)| l.trim_start().starts_with("caused by: "))
      .map(|(i, _)| i)
      .collect();

    // `error: ` opens. Guaranteed by how the scan finds them, asserted anyway
    // so the property is stated where a reader looks rather than implied by the
    // search pattern.
    if !lit[0].starts_with("error: ") {
      wrong.push(format!("does not open with `error: `: {joined}"));
      continue;
    }

    if let Some(r) = remedy_at {
      // **LAST, because a remedy with anything after it is advice the reader
      // scrolls past.** The trait puts it last by construction; a literal must
      // do it by hand.
      if r != lit.len() - 1 {
        wrong.push(format!("`remedy: ` is not the last line: {joined}"));
      }
      // **AND EVERY CAUSE BEFORE IT.** `error: … / caused by: … / remedy: …`
      // is the trait's order; a cause after the remedy reads as a second error.
      for c in &cause_positions {
        if *c > r {
          wrong.push(format!("`caused by: ` appears AFTER `remedy: `: {joined}"));
        }
      }
    }
  }

  assert!(
    wrong.is_empty(),
    "these hand-written error literals do not match the shape `Remedy::render` produces, so the renderer emits two different formats for one kind of message and an operator cannot tell which subsystem they are reading:\n  {}",
    wrong.join("\n  ")
  );
}

/// **REPORTED, NOT GATED: the literals that offer no remedy at all.**
///
/// Whether each is legitimately remedy-less is an open question nobody has
/// answered, and forcing one would produce advice that cannot be acted on. The
/// count is printed so the question can be ASKED -- a number nobody prints is a
/// question nobody asks.
#[test]
fn the_remedy_less_literals_are_named() {
  let literals = error_literals(&scannable(&renderer()));
  let without: Vec<String> = literals
    .iter()
    .filter(|l| !l.iter().any(|x| x.trim_start().starts_with("remedy: ")))
    .map(|l| l.join("\\n"))
    .collect();

  println!(
    "error-literal-shape: {} operator-facing error literal(s) in the renderer, {} of them with NO remedy:",
    literals.len(),
    without.len()
  );
  for line in &without {
    println!("  {line}");
  }
}
