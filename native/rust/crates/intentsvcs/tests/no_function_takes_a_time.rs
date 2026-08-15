//! **D42, the signature-level form: no function in the CLI or in `intentsvcs`
//! TAKES a time.** Functions may RETURN times, and every time they return was
//! set by SQLite on a record.
//!
//! hv, 2026-08-15, verbatim:
//!
//! > _"intent3 won't have any cli or intentsvcs functions that TAKE a time.
//! > There will be cli and intentsvcs functions that RETURN times, but those
//! > will have gone end-to-end thru the db where the time was SET BY SQLite."_
//!
//! **This is strictly stronger than `one_clock.rs` and it is a different
//! question.** That file asks whether anything READS a clock, which is a
//! question about statements; this one asks whether anything can be HANDED a
//! time, which is a question about signatures. A parameter is the durable
//! defect: a clock read is one site someone chose, whereas a time-typed
//! parameter is a standing invitation that will be accepted eventually
//! regardless of who is careful today. `one_clock.rs` would sail straight past
//! `fn record(when: String)`.
//!
//! It is also the form that needs no judgement. Asking where a caller got a
//! timestamp is a trace, and this estate has failed that trace three times in
//! one day; asking whether a signature accepts one is a grep.
//!
//! **`src/` only, deliberately.** hv's rule is about the shipped API surface. A
//! fixture may hand a recorded date to the RESTORE door -- that is preserving
//! history, which is the door's whole purpose -- and banning it in tests would
//! forbid testing the restore path at all. `one_clock.rs` covers `tests/` for
//! the different and narrower question of whether a fixture INVENTS a time.

use std::path::{Path, PathBuf};
use testkit::workspace_root;

/// Parameter names that denote a moment.
///
/// Deliberately a name test rather than a type test, because the types a time
/// arrives as are the same types everything else arrives as -- `String` and
/// `i64` carry no marking. **The limit is worth stating rather than leaving to
/// be discovered: a time smuggled in as `value: String` passes.** What this
/// catches is the realistic regression, which is someone reinstating a
/// parameter under the obvious name, and that is what every instance on this
/// estate has actually looked like.
const TIME_NAMES: &[&str] = &[
  "ts",
  "time",
  "timestamp",
  "now",
  "today",
  "date",
  "datetime",
  "when",
  "stamp",
  "mtime",
  "since",
  "until",
  "deadline",
  "expires",
  "expiry",
  "created_at",
  "updated_at",
  "written_at",
];

/// Types a time can actually be smuggled in as.
///
/// **The type half is what keeps the name half honest**, and it earns its place
/// on a real case: `write_thread(tx, t, stamp: Stamp)` has a parameter called
/// `stamp`, and `Stamp` is the enum naming WHICH DOOR the write is going
/// through -- create or restore. That is the mechanism enforcing D42, not a
/// violation of it, and a name-only check would have condemned it.
fn is_time_shaped(ty: &str) -> bool {
  let ty = ty.replace(['&', ' '], "");
  let ty = ty.trim_start_matches("mut");
  [
    "String",
    "str",
    "Option<String>",
    "Option<&str>",
    "Option<str>",
    "i64",
    "u64",
    "OffsetDateTime",
    "SystemTime",
    "NaiveDate",
    "NaiveDateTime",
    "DateTime",
  ]
  .contains(&ty)
}

/// Every `src/**/*.rs` in every crate, discovered by walking rather than
/// listed -- same reason as `one_clock.rs`: the act that invalidates a
/// hand-kept roster (adding a file) is not the act that updates it.
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
  for entry in std::fs::read_dir(root.join("crates"))
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

fn code_of(path: &Path) -> String {
  std::fs::read_to_string(path)
    .unwrap_or_default()
    .lines()
    .filter(|l| !l.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n")
}

/// Split a parameter list at commas that are not inside brackets, so
/// `Result<A, B>` and `(a, b)` stay whole.
fn split_params(params: &str) -> Vec<String> {
  let mut out = Vec::new();
  let mut depth = 0i32;
  let mut current = String::new();
  for ch in params.chars() {
    match ch {
      '<' | '(' | '[' => depth += 1,
      '>' | ')' | ']' => depth -= 1,
      ',' if depth == 0 => {
        out.push(std::mem::take(&mut current));
        continue;
      }
      _ => {}
    }
    current.push(ch);
  }
  if !current.trim().is_empty() {
    out.push(current);
  }
  out
}

/// Every `(name, type)` parameter of every `fn` in a source file.
fn parameters(code: &str) -> Vec<(String, String)> {
  let mut out = Vec::new();
  let bytes: Vec<char> = code.chars().collect();
  let mut idx = 0;
  while let Some(found) = code[idx..].find("fn ") {
    let start = idx + found;
    let Some(open_rel) = code[start..].find('(') else {
      break;
    };
    let open = start + open_rel;
    // Balanced scan to the closing paren, so a defaulted generic or a tuple
    // parameter does not end the list early.
    let mut depth = 0i32;
    let mut close = None;
    for (i, ch) in bytes.iter().enumerate().skip(open) {
      match ch {
        '(' => depth += 1,
        ')' => {
          depth -= 1;
          if depth == 0 {
            close = Some(i);
            break;
          }
        }
        _ => {}
      }
    }
    let Some(close) = close else { break };
    for param in split_params(&code[open + 1..close]) {
      // `&self` / `&mut self` carry no type annotation.
      let Some((name, ty)) = param.split_once(':') else {
        continue;
      };
      let name = name.trim().trim_start_matches("mut ").trim().to_string();
      out.push((name, ty.trim().to_string()));
    }
    idx = close;
  }
  out
}

fn offenders_in(code: &str) -> Vec<String> {
  parameters(code)
    .into_iter()
    .filter(|(name, ty)| TIME_NAMES.contains(&name.trim_start_matches('_')) && is_time_shaped(ty))
    .map(|(name, ty)| format!("{name}: {ty}"))
    .collect()
}

#[test]
fn no_function_in_the_shipped_surface_takes_a_time() {
  let root = workspace_root();
  let files = sources(&root);
  assert!(
    files.len() > 10,
    "precondition: the walk found the workspace ({} files)",
    files.len()
  );

  let mut found = Vec::new();
  for path in &files {
    let rel = path
      .strip_prefix(&root)
      .expect("under the root")
      .to_string_lossy()
      .replace('\\', "/");
    if rel.ends_with("tests/no_function_takes_a_time.rs") {
      continue;
    }
    for offender in offenders_in(&code_of(path)) {
      found.push(format!("{rel} -- {offender}"));
    }
  }

  assert!(
    found.is_empty(),
    "D42, signature form (hv, 2026-08-15): no CLI or intentsvcs function takes a time. A record is \
     stamped BY the write that creates it, so a parameter here is a value someone had to obtain \
     first -- and no provenance for it would make it acceptable. These accept one --\n  {}",
    found.join("\n  ")
  );
}

/// **The detector finds what it claims to find, and does not find what it must
/// not.** A guard reporting an empty list is indistinguishable from a guard
/// that cannot see, which is how a check comes to pass forever.
#[test]
fn the_detector_catches_a_time_parameter_and_spares_the_mechanism() {
  assert_eq!(
    offenders_in("fn record(ts: String) {}"),
    vec!["ts: String".to_string()],
    "the plainest form of the defect must be caught"
  );
  assert_eq!(
    offenders_in("pub fn backup(&self, today: &str, keep: usize) {}"),
    vec!["today: &str".to_string()],
    "and it must be found among other parameters, not only alone"
  );
  assert!(
    offenders_in("fn snapshot_into(&self, dest: &std::path::Path) {}").is_empty(),
    "a path is not a time"
  );

  // **The mechanism must survive the guard.** `Stamp` is the enum naming which
  // door a write is going through; a name-only check would condemn the thing
  // enforcing the rule.
  assert!(
    offenders_in("fn write_thread(tx: &Transaction<'_>, t: &Thread, stamp: Stamp) {}").is_empty(),
    "`stamp: Stamp` selects the create-or-restore door and carries no time"
  );

  // Generics must not end the parameter scan early.
  assert_eq!(
    offenders_in("fn f(a: Result<A, B>, when: String) {}"),
    vec!["when: String".to_string()],
    "a comma inside angle brackets does not end a parameter"
  );
}

/// **The one legitimate seam, named rather than left implied.**
///
/// `restore_event` accepts an `&Envelope`, and that envelope carries a `ts`. It
/// is not a counter-example: the parameter is a RECORD, and the stamp inside it
/// was set by SQLite on the machine that first recorded the event, then carried
/// through the committed extract. **Carrying a record is transport; taking a
/// time is authorship.** Stated here because an unexplained near-miss is what
/// gets "fixed" later by someone tidying up.
#[test]
fn carrying_a_record_that_holds_a_stamp_is_not_taking_a_time() {
  let store = std::fs::read_to_string(workspace_root().join("crates/intentsvcs/src/store.rs"))
    .expect("read the store");
  assert!(
    store.contains("pub fn restore_event(&self, e: &Envelope)"),
    "the restore door takes a RECORD; if its signature changed to take a time directly, that is \
     the thing this whole file forbids"
  );
  assert!(
    offenders_in("pub fn restore_event(&self, e: &Envelope) {}").is_empty(),
    "and the detector agrees: an envelope is not a timestamp"
  );
}
