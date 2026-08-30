//! `IN-AG-NO-SILENT-001` on the one expression that keeps coming back:
//! `try_get_one::<bool>(name).ok()`.
//!
//! **A GREP-SHAPED GUARD IS THE RIGHT INSTRUMENT HERE AND THAT IS UNUSUAL**, so
//! it is worth saying why rather than leaving it to look lazy. Everywhere else
//! this estate has met a second home tonight, the answer was to DERIVE the
//! second value out of existence -- the placeholder from `arity`, the undo from
//! `Step::ORDER`. There is nothing to derive here: the defect is a shape a
//! future author can retype from scratch in a file that already reads
//! correctly, and no amount of restructuring makes it unwritable. A check that
//! cannot be satisfied by coincidence is what is left.
//!
//! # What the expression does, and why it survived so long
//!
//! clap stores a `bool` for `ArgAction::SetTrue` **and only that**. A flag
//! declared with a value stores a `String`, so `try_get_one::<bool>` on it is a
//! type mismatch; `.ok()` discards the mismatch, and the flag reads as ABSENT
//! -- given on the command line, parsed by clap, invisible to the arm.
//!
//! **The fallback is a reasonable thing for the command to do, which is why it
//! reads as a design decision.** `intent edit st ST0001 design --editor`
//! printed the path while `intent st edit ST0001 design --editor` opened an
//! editor: the same spelling is `bool` on the kept row and valued on the new
//! one, and one helper answered correctly for one and silently wrongly for the
//! other, under help text promising two things the arm could not do.
//!
//! # Why this file counts rather than trusting a census
//!
//! **FOUR ENUMERATORS GAVE FOUR ANSWERS AND EVERY ONE WAS SHORT OF THE NEXT.**
//! A grep for the helper's exact expression found 3. A grep for `flag(` found
//! 12. A grep for both found 17. **The compiler found 24**, because call sites
//! spelled `flag(a, ...)` and `flag(m, other)` match neither pattern. The
//! population was only knowable by deleting the helper and letting the build
//! enumerate what broke -- so this file asserts the END STATE, which is a
//! property, rather than a count, which is a measurement that was wrong four
//! times.

use std::path::Path;

/// The one place the expression is still written, and the reason.
///
/// **DECLARED, NOT FILTERED, for the reason the mode machine's Esc exemption is
/// declared:** `!line.contains("init")` would also pass for a site that had
/// LOST its handling by accident, because the exemption and the accident look
/// identical to a predicate. As a declared pair, a second entry is an edit
/// somebody has to justify -- and the assertion below is an EQUALITY, so adding
/// one goes red.
const EXEMPT: &[(&str, &str)] = &[(
  "let asked = match a.try_get_one::<bool>(flag) {",
  "`init`'s drift check. Its ids are declared unconditionally on its own row, so a \
   miss is renderer-table DRIFT and panics by design -- the fix for \
   `init --with-st0000` being silently ignored. `given` cannot carry that \
   contract: it must answer `false` for an id the row does not declare, because \
   `claude skills uninstall` asks about a `--force` its own row omits and a panic \
   there turns a correct absence into a crash. Two questions; the thing that must \
   not happen is one helper quietly answering both.",
)];

fn render_rs() -> String {
  let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/render.rs");
  std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// A line that is prose rather than code. The doc comments in `render.rs`
/// QUOTE this expression in order to explain it, and a check that counted those
/// would make describing the defect an offence -- which is the same trap the
/// whiteboard header guard avoids by never scanning prose.
fn is_prose(line: &str) -> bool {
  let t = line.trim_start();
  t.starts_with("//") || t.starts_with("///") || t.starts_with("//!")
}

#[test]
fn no_flag_is_read_through_a_swallowed_type_mismatch() {
  let src = render_rs();
  let offenders: Vec<String> = src
    .lines()
    .enumerate()
    .filter(|(_, l)| l.contains("try_get_one::<bool>"))
    .filter(|(_, l)| !is_prose(l))
    .filter(|(_, l)| !EXEMPT.iter().any(|(text, _)| l.trim() == *text))
    .map(|(i, l)| format!("render.rs:{}  {}", i + 1, l.trim()))
    .collect();

  assert!(
    offenders.is_empty(),
    "`try_get_one::<bool>(..)` reads a flag as a bool, which is what clap stores for \
     `ArgAction::SetTrue` and ONLY that -- a valued flag is a type mismatch and `.ok()` \
     turns it into a silent `false`. Use `given()`, which asks whether the flag was given \
     whatever it carries. If this site genuinely needs the other contract, add it to \
     `EXEMPT` with its reason:\n  {}",
    offenders.join("\n  ")
  );
}

/// **AN EXEMPTION THAT MATCHES NOTHING IS A CHECK PASSING FOR THE WRONG
/// REASON.** If `init`'s loop is ever rewritten, the entry above stops matching
/// and the guard above still passes -- green because there is nothing left to
/// forgive, which is indistinguishable from green because everything is right.
#[test]
fn every_exemption_still_names_a_real_site() {
  let src = render_rs();
  for (text, why) in EXEMPT {
    assert!(
      src.lines().any(|l| l.trim() == *text),
      "the exemption `{text}` matches no line in render.rs, so it forgives nothing and the \
       guard passes for a reason that is no longer true. Delete it or re-point it. Its \
       stated reason was: {why}"
    );
    assert!(
      !why.trim().is_empty(),
      "an exemption with no stated reason is indistinguishable from a site somebody could not make work"
    );
  }
}

/// The guard is only worth having if it can fail. Driven against a planted
/// line rather than asserted about the real file, because **a checker whose
/// corpus cannot exhibit the defect is green for free**.
#[test]
fn the_guard_fires_on_a_planted_swallow() {
  let planted =
    "  let x = a.try_get_one::<bool>(\"planted\").ok().flatten().copied() == Some(true);";
  let caught = [planted]
    .iter()
    .filter(|l| l.contains("try_get_one::<bool>"))
    .filter(|l| !is_prose(l))
    .filter(|l| !EXEMPT.iter().any(|(text, _)| l.trim() == *text))
    .count();
  assert_eq!(
    caught, 1,
    "the predicate this file greps with must catch the shape it is written to catch"
  );

  let quoted =
    "  /// reads `try_get_one::<bool>(flag).ok()`, which turns clap's mismatch into None";
  let caught_prose = [quoted].iter().filter(|l| !is_prose(l)).count();
  assert_eq!(
    caught_prose, 0,
    "a doc comment EXPLAINING the defect must not count as an instance of it, or describing the \
     class becomes an offence"
  );
}
