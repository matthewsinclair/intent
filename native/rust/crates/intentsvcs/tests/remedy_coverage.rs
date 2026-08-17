//! **Every error type this crate declares says what to DO about itself.**
//!
//! vc's Highlander finding 4. The state before this: four error enums carried
//! `remedy()` as a method, one baked its remedy into its own Display string,
//! and the rest carried none -- so a caller holding an error had no uniform way
//! to ask, and wrote a remedy at the call site or omitted one.
//!
//! **The embedded case is the one worth naming, because it is invisible until
//! something renders uniformly.** `InstallError` put its remedy inside
//! `#[error(...)]`, so it arrived as part of `{e}`; anything printing
//! `error: {e}` followed by a remedy line would have printed it twice, and
//! `intent info` escaped that only by printing no remedy at all.
//!
//! **The check is two-part, and neither part works alone.** The scan finds
//! every type deriving `thiserror::Error` in this crate's source. The proof is
//! a `implements::<T>()` line per type, which does not compile unless the trait
//! is implemented. **The scan cannot prove implementation and the proof cannot
//! find a type nobody wrote a line for**, so this file requires the two to
//! agree -- by reading its own source for the proof lines.
//!
//! Scanned by DERIVE rather than by a `*Error` name pattern, because one of
//! them is not named that way: `UnhonourableWindow` is a refusal like any
//! other, and a name-based roster would have missed it silently.

use std::collections::BTreeSet;
use std::path::PathBuf;

use intentsvcs::remedy::Remedy;
use testkit::workspace_root;

/// Compile-time proof: this does not build unless `E` implements the trait.
fn implements<E: Remedy>() {}

/// One line per error type in this crate. **The list is checked against the
/// source below**, so adding a type without adding a line here reds.
fn proofs() {
  implements::<intentsvcs::backup::BackupError>();
  implements::<intentsvcs::facade::FacadeError>();
  implements::<intentsvcs::ingest::IngestError>();
  implements::<intentsvcs::install::InstallError>();
  implements::<intentsvcs::event::JsonlError>();
  implements::<intentsvcs::project::ProjectError>();
  implements::<intentsvcs::project::UnhonourableWindow>();
  implements::<intentsvcs::finding::Refusal>();
  implements::<intentsvcs::store::StoreError>();
  implements::<intentsvcs::sync::SyncError>();
  implements::<intentsvcs::write_set::WriteError>();
}

fn src_dir() -> PathBuf {
  workspace_root().join("crates/intentsvcs/src")
}

/// Every type in this crate that derives `thiserror::Error`.
fn error_types() -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  let entries = std::fs::read_dir(src_dir()).expect("the intentsvcs source directory");
  for entry in entries.flatten() {
    let path = entry.path();
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
      continue;
    }
    let text = std::fs::read_to_string(&path).expect("read source");
    let mut armed = false;
    for line in text.lines() {
      let t = line.trim();
      if t.starts_with("#[derive(") && t.contains("thiserror::Error") {
        armed = true;
        continue;
      }
      if !armed {
        continue;
      }
      // Attributes between the derive and the item -- `#[error(...)]` on a
      // struct error sits exactly here.
      if t.starts_with("#[") || t.starts_with("//") || t.is_empty() {
        continue;
      }
      if let Some(rest) = t
        .strip_prefix("pub enum ")
        .or_else(|| t.strip_prefix("pub struct "))
      {
        let name: String = rest
          .chars()
          .take_while(|c| c.is_alphanumeric() || *c == '_')
          .collect();
        out.insert(name);
      }
      armed = false;
    }
  }
  out
}

/// This file's own source, so the proof lines can be compared to the scan.
fn own_source() -> &'static str {
  include_str!("remedy_coverage.rs")
}

/// The tail of a proof line for `name`, eg `::ProjectError>()`.
///
/// **The leading `::` and the trailing `>()` are both load-bearing.** Proof
/// lines are module-qualified (`intentsvcs::project::ProjectError`), so
/// matching the fully-qualified path would need this file to know each type's
/// module -- a second place the layout is written down. And matching the bare
/// NAME would find it in this file's own prose, so a type merely discussed in a
/// comment would report as proven.
fn suffix(name: &str) -> String {
  format!("::{name}>()")
}

/// **The fixture proves itself.** A scan that found nothing would satisfy every
/// assertion below.
#[test]
fn the_scan_finds_this_crates_error_types() {
  let found = error_types();
  assert!(
    found.len() >= 8,
    "the derive scan found {} error types in {}, which is fewer than this crate has -- the scan has stopped matching the source it reads, and a scan that finds \
     nothing agrees with every claim in this file",
    found.len(),
    src_dir().display()
  );
}

/// **Every error type has a proof line, and every proof line compiles.**
///
/// The compiler enforces the second half by existing; this enforces the first.
#[test]
fn every_error_type_states_a_remedy() {
  proofs();

  let source = own_source();
  let missing: Vec<String> = error_types()
    .into_iter()
    // ONE condition, not a disjunction. The first draft read
    // `!source.contains("implements::<intentsvcs::") || !source.contains(...)`,
    // whose first arm is always FALSE -- so the `||` did nothing, and a
    // disjunction with a constant arm is where a decorative guard hides. The
    // arm that was doing the work is the one that survived.
    .filter(|name| !source.contains(&suffix(name)))
    .collect();

  assert!(
    missing.is_empty(),
    "these types derive `thiserror::Error` and have no proof line in `proofs()`:\n  {}\n\n**Implement `intentsvcs::remedy::Remedy` for them and add the line.** \
     An error that cannot say what to do about itself makes its caller invent one, and the callers that invented one are how the same failure came to be \
     reported five different ways in v2. Do NOT put the remedy inside `#[error(...)]`: it then arrives as part of the message, and anything rendering a remedy \
     line prints it twice.",
    missing.join("\n  ")
  );
}

/// **The remedy is not inside the message.** One convention, checked.
///
/// This is the specific defect the trait replaced, and it is worth its own
/// assertion because it is invisible from the outside: an error whose Display
/// already contains its remedy renders correctly under a caller that prints no
/// remedy, and doubles under one that does.
#[test]
fn no_error_bakes_its_remedy_into_its_own_message() {
  let mut offenders = Vec::new();
  for entry in std::fs::read_dir(src_dir()).expect("source dir").flatten() {
    let path = entry.path();
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
      continue;
    }
    let text = std::fs::read_to_string(&path).expect("read source");
    let mut in_error_attr = false;
    for (n, line) in text.lines().enumerate() {
      let t = line.trim();
      if t.starts_with("#[error(") {
        in_error_attr = true;
      }
      if in_error_attr && t.contains("remedy:") {
        offenders.push(format!("{}:{}", path.display(), n + 1));
      }
      if in_error_attr && (t.ends_with(")]") || t.ends_with("\")]")) {
        in_error_attr = false;
      }
    }
  }

  assert!(
    offenders.is_empty(),
    "these `#[error(...)]` attributes contain their own remedy:\n  {}\n\nThe remedy belongs in `Remedy::remedy`, not in the message. `InstallError` did this \
     and the consequence is invisible from the type's own side: it renders correctly under a caller that prints no remedy line and DOUBLES under one that does",
    offenders.join("\n  ")
  );
}

/// **No two error types render the same remedy for the same fault**, checked
/// where it is cheapest to get wrong: the io-flavoured variants, which every
/// enum has and which are the most tempting to answer with one sentence.
///
/// `error_remedies.rs` makes this assertion pairwise across `FacadeError`'s
/// variants. This is the same rule ACROSS types, which that file cannot see.
#[test]
fn the_io_flavoured_remedies_are_not_one_sentence_repeated() {
  let remedies = [
    intentsvcs::project::ProjectError::Io {
      path: "p".to_string(),
      source: std::io::Error::other("x"),
    }
    .remedy(),
    intentsvcs::sync::SyncError::Io {
      path: "p".to_string(),
      source: std::io::Error::other("x"),
    }
    .remedy(),
    intentsvcs::write_set::WriteError::Io {
      path: "p".to_string(),
      source: std::io::Error::other("x"),
    }
    .remedy(),
  ];

  let unique: BTreeSet<&String> = remedies.iter().collect();
  assert_eq!(
    unique.len(),
    remedies.len(),
    "two error types answer an io failure with the same words: {remedies:?}\n\nThey are different faults -- one is a project that cannot be READ, one a sync \
     that cannot read a file, one a WRITE that failed and rolled back -- and the third's remedy has to say that nothing was left half-written, which is the \
     single most useful thing any of them can say"
  );
}
