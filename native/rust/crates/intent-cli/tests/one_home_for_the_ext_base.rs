//! Every construction that takes an extension base asks the ONE function that
//! owns the answer, and none of them carries a literal.
//!
//! # The defect this pins, which was live and silent
//!
//! **TWO CALL SITES ANSWERED "DOES THIS BUILD SEE EXTENSIONS" AND THEY AGREED
//! ONLY BY COINCIDENCE.** `payload_lib` passed `userstate::ext_base()`, so
//! skills and subagents would have picked up a ruling the moment it landed.
//! `library()` passed a hardcoded `None`, so rules silently would not have.
//! One question, two values, and today's agreement rested entirely on
//! `ext_base()` happening to return `None` -- the divergence was latent, and
//! the ruling that resolves the seam is exactly what would have made it live.
//!
//! **THE CHEAPEST MOMENT TO COLLAPSE THAT IS WHILE BOTH ANSWERS AGREE**, because
//! there is nothing to adjudicate. After a ruling it is a merge of two
//! behaviours rather than of two call sites.
//!
//! # Why this is a SOURCE check and not a runtime one
//!
//! A runtime test -- construct both doors, assert they see the same base --
//! **passes under the broken tree too**, because both were `None`. It is the
//! control that would pass under the broken instrument, which is decoration.
//! The defect is a duplicated DECISION in the source, so the source is the
//! only corpus that can exhibit it.
//!
//! # Reach, stated at mint
//!
//! This sees calls whose ext argument is written at the call site. A base
//! routed through a local variable several statements up is outside the walk
//! by construction, and the check would report it clean. That is a real hole
//! and it is bounded by the same thing that makes the check worth having: the
//! estate's idiom is to pass `ext_base()` inline, and a future call that hides
//! the argument behind a binding is the shape to catch in review.
//!
//! **`sources`/`code_of` are duplicated from `no_intent_home.rs`, deliberately
//! and with the cost named.** Four test files already carry this walker --
//! integration tests are separate binaries -- so this is the estate's standing
//! practice rather than a new divergence. Collapsing all five into
//! `tests/common/` is worth doing and is NOT this change: a Highlander fix
//! smuggled into a test for an unrelated seam is how a small change becomes
//! unreviewable.

use std::path::{Path, PathBuf};

/// The function that owns whether extensions are visible.
const OWNER: &str = "ext_base()";

/// The constructors whose signature carries an extension base.
const TAKES_AN_EXT_BASE: &[&str] = &["Library::new(", "Payload::new("];

/// One construction found in the source, reduced to what this check judges.
#[derive(Debug, PartialEq, Eq)]
struct Call {
  file: String,
  ctor: String,
  args: String,
}

/// **THE VERDICT IS PURE, SO IT CAN BE DRIVEN BOTH WAYS** (IN-AG-PFIC-001).
/// The walk is the impure half and lives in the test that reads the tree.
fn offenders(calls: &[Call]) -> Vec<&Call> {
  calls.iter().filter(|c| !c.args.contains(OWNER)).collect()
}

fn sources(root: &Path) -> Vec<PathBuf> {
  fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(&path, out);
      } else if path.extension().is_some_and(|e| e == "rs") {
        out.push(path);
      }
    }
  }
  let mut out = Vec::new();
  let crates = root.join("crates");
  for entry in std::fs::read_dir(&crates)
    .expect("read the crates dir")
    .flatten()
  {
    let src = entry.path().join("src");
    if src.is_dir() {
      walk(&src, &mut out);
    }
  }
  out.sort();
  out
}

/// Strip line comments, so this file's own prose -- and the source comments
/// that state the rule in English, including the one naming the `None` that
/// used to sit here -- can neither satisfy the check nor trip it.
fn code_of(path: &Path) -> String {
  std::fs::read_to_string(path)
    .unwrap_or_default()
    .lines()
    .filter(|l| !l.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n")
}

/// The argument list of a call, by balancing parens from the opening one.
fn args_at(code: &str, open: usize) -> String {
  let bytes = code.as_bytes();
  let mut depth = 0usize;
  for (i, b) in bytes.iter().enumerate().skip(open) {
    match b {
      b'(' => depth += 1,
      b')' => {
        depth -= 1;
        if depth == 0 {
          return code[open + 1..i].to_string();
        }
      }
      _ => {}
    }
  }
  code[open..].to_string()
}

fn calls_in_tree() -> Vec<Call> {
  let root = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .and_then(Path::parent)
    .expect("the workspace root")
    .to_path_buf();
  let mut found = Vec::new();
  for file in sources(&root) {
    let code = code_of(&file);
    for ctor in TAKES_AN_EXT_BASE {
      let mut from = 0usize;
      while let Some(hit) = code[from..].find(ctor) {
        let at = from + hit;
        let open = at + ctor.len() - 1;
        found.push(Call {
          file: file
            .strip_prefix(&root)
            .unwrap_or(&file)
            .display()
            .to_string(),
          ctor: (*ctor).to_string(),
          args: args_at(&code, open),
        });
        from = at + ctor.len();
      }
    }
  }
  found
}

#[test]
fn every_ext_base_comes_from_the_function_that_owns_it() {
  let calls = calls_in_tree();

  // **A CHECK OVER AN EMPTY CORPUS IS A GREEN THAT MEANS NOTHING.** If a
  // rename empties the walk, this fails rather than passing silently -- the
  // vacuous-pass shape that `AT-11.5` names.
  assert!(
    !calls.is_empty(),
    "found no `Library::new(` or `Payload::new(` call in any crate's src/. \
     Either the constructors were renamed -- update TAKES_AN_EXT_BASE -- or \
     this walk is broken. A check with nothing to check must not report ok."
  );

  let bad = offenders(&calls);
  assert!(
    bad.is_empty(),
    "these constructions decide extension visibility for themselves instead of \
     asking `intentsvcs::userstate::{OWNER}`:\n{}\n\n\
     `ext_base()` is the one home for that decision and carries the reason it \
     is held. A literal here is a SECOND home: it agrees with the first only \
     while `ext_base()` returns `None`, and the ruling that changes that answer \
     is exactly what makes the two disagree -- silently, in the direction where \
     one door honours an operator's `$INTENT_EXT_DISABLE` and another ignores it.",
    bad
      .iter()
      .map(|c| format!("  {} -- {}{})", c.file, c.ctor, c.args.trim()))
      .collect::<Vec<_>>()
      .join("\n")
  );
}

#[test]
fn the_check_catches_a_hardcoded_base() {
  // Driven from the other side: the verdict must fire on the exact shape that
  // was live in `library()` before the collapse.
  let planted = vec![Call {
    file: "crates/intent-cli/src/render.rs".into(),
    ctor: "Library::new(".into(),
    args: "&home, None".into(),
  }];
  assert_eq!(
    offenders(&planted).len(),
    1,
    "the verdict must reject a hardcoded ext base -- this is the defect that shipped"
  );
}

#[test]
fn the_check_accepts_the_owner() {
  // The negative control. Without it, a verdict that rejected EVERYTHING would
  // pass the test above and look like a working guard.
  let good = vec![
    Call {
      file: "crates/intent-cli/src/render.rs".into(),
      ctor: "Library::new(".into(),
      args: "&home, intentsvcs::userstate::ext_base()".into(),
    },
    Call {
      file: "crates/intent-cli/src/render.rs".into(),
      ctor: "Payload::new(".into(),
      args: "kind, &install, userstate::ext_base(), target, manifest".into(),
    },
  ];
  assert!(
    offenders(&good).is_empty(),
    "a call that asks the owner must pass, or the guard rejects the fix as well as the defect"
  );
}
