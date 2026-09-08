//! No test hardcodes the VALUE of [`intentsvcs::install::MARKER`] (issue 0279).
//!
//! # Why a guard and not just a cleanup
//!
//! Eight test files across two crates held the literal `lib/templates` while
//! `install.rs` exported the constant that defines it -- and `is_install` is
//! exactly `dir.join(MARKER).is_dir()`, so the constant is what MAKES a
//! directory an install root. A hardcoded copy builds a tree the resolver does
//! not recognise the moment the constant moves, and the test then fails
//! somewhere else entirely, as a missing file rather than as a wrong fixture.
//!
//! **TWO OF THE EIGHT CARRIED `// install::MARKER -- what makes a tree an
//! install.` ON THE LINE DIRECTLY ABOVE THE LITERAL.** A comment naming the
//! constant beside its hardcoded value is worse than no comment: it proves the
//! author knew, which is the strongest available evidence that knowing is not
//! the mechanism that prevents this. Converging the eight without a guard fixes
//! today and leaves the ninth to whoever writes the next fixture.
//!
//! # The needle is DERIVED, which is what stops this file matching itself
//!
//! The pattern is built at run time from `MARKER` -- so this source never
//! contains the literal it searches for, and cannot appear in its own findings.
//! A guard written with the string spelled out would either flag itself forever
//! or need an exemption for its own path, and an exemption list is the thing
//! that quietly grows to cover the next real hit.

use std::path::{Path, PathBuf};

/// Every `.rs` file under a crate's `tests/`, one and two levels deep.
///
/// **BOTH DEPTHS, DELIBERATELY.** A one-level walk misses `tests/common/mod.rs`
/// -- which is precisely where a shared fixture lives, so a census run at one
/// level is blind to the file most likely to hold the helper.
fn test_sources(root: &Path) -> Vec<PathBuf> {
  let mut out = Vec::new();
  for crate_dir in std::fs::read_dir(root.join("crates")).expect("crates/") {
    let tests = crate_dir.expect("entry").path().join("tests");
    if !tests.is_dir() {
      continue;
    }
    let mut stack = vec![tests];
    while let Some(dir) = stack.pop() {
      for entry in std::fs::read_dir(&dir).expect("a tests dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
          stack.push(path);
        } else if path.extension().is_some_and(|e| e == "rs") {
          out.push(path);
        }
      }
    }
  }
  out
}

#[test]
fn no_test_hardcodes_the_install_marker() {
  // Built here rather than written out -- see the module note.
  let needle = format!("\"{}\"", intentsvcs::install::MARKER);

  let sources = test_sources(&testkit::workspace_root());
  assert!(
    sources.len() > 100,
    "the walk found only {} test sources; it is measuring the wrong tree",
    sources.len()
  );

  // POSITIVE CONTROL. A matcher that cannot fire reports a clean tree for the
  // same reason a broken one does, and the two are indistinguishable from the
  // verdict alone.
  let planted = format!("std::fs::create_dir_all(dir.join({needle}))");
  assert!(
    planted.contains(&needle),
    "the matcher cannot find a planted literal, so its silence means nothing"
  );

  let mut offenders = Vec::new();
  for path in &sources {
    let text = std::fs::read_to_string(path).expect("read a test source");
    for (n, line) in text.lines().enumerate() {
      // Comments may DISCUSS the value; only code may not hold it. This file's
      // own module note is the case that proves the distinction is needed.
      if line.trim_start().starts_with("//") {
        continue;
      }
      if line.contains(&needle) {
        offenders.push(format!("{}:{}", path.display(), n + 1));
      }
    }
  }

  assert!(
    offenders.is_empty(),
    "these tests hardcode the value of install::MARKER instead of naming the constant:\n  {}\n\
     Use `intentsvcs::install::MARKER`, or the fake-install helper in this crate's tests/common.",
    offenders.join("\n  ")
  );
}
