//! Rust's level-3 reader (ST0076 WP-05): rust-analyzer's SCIP export, read
//! into the [`Trace`] the level-3 core joins.
//!
//! **TWO HALVES, AND ONLY ONE OF THEM TOUCHES THE WORLD (IN-AG-PFIC-001).**
//! [`roots`], [`printable`] and [`trace_of`] are pure: which directories an
//! export runs over, how a SCIP symbol prints as a target, and what a decoded
//! index says about the bytes it was read from. [`RustAnalyzer`] spawns the
//! tool, reads the files and the index, and calls them.
//!
//! **THE EXPORT RUNS THE PROJECT'S OWN CODE.** rust-analyzer's `scip` runs the
//! workspace's build scripts and proc macros, and no switch it has stops that
//! (measured 2026-09-17), which is why only `intent index resolve` calls this.
//!
//! **AND IT CAN WRITE ONE FILE INTO THE PROJECT.** Where a workspace root holds
//! no `Cargo.lock`, cargo's metadata step writes one there, as any cargo
//! command would. The reader leaves it (vc, 2026-09-17): it never deletes or
//! moves a file in the project tree, because a lock that appears during an
//! export may be the user's own cargo's, and no stable switch moves it
//! (`--locked` refuses instead). What the export builds goes under Intent's own
//! directory.

use std::collections::BTreeMap;
use std::ffi::OsString;

use super::resolved::{Manifest, Read, Reference, Resolver, Scope, Trace, Unresolved};
use super::scip;

/// A reference to a local variable, closure or parameter: `local <n>` in SCIP,
/// which names no definition a reader outside the function can look up.
pub const LOCAL: &str = "local";
/// An occurrence spanning more than one line, which no name does.
pub const MULTILINE: &str = "multiline";
/// A reference whose text holds no letter, digit or underscore. The export
/// writes an overloaded operator's call on the operator and on the space either
/// side of it, so `a + b` gives three references to `<usize as
/// Add<Self>>::add()`, and no written row names an operator (vc, 2026-09-17).
pub const OPERATOR: &str = "operator";
/// A symbol this reader cannot print as a target.
pub const UNPRINTABLE: &str = "unprintable";

/// The file a Rust project is found by: a `Cargo.toml` anywhere the index
/// holds one. [`roots`] reads it, and so does a search answer deciding whether
/// Rust can be `unresolved` here (ST0076 WP-07).
pub const MANIFEST: Manifest = Manifest {
  name: "Cargo.toml",
  root_only: false,
};

/// The directories an export runs over: each `Cargo.toml` the index holds
/// whose directory has no ancestor holding another (vc, 2026-09-17). A
/// workspace's member crates sit under its root and are skipped; a
/// single-package crate is a root of its own. `""` is the project root.
pub fn roots(indexed: &[String]) -> Vec<String> {
  let dirs: Vec<&str> = indexed
    .iter()
    .filter_map(|path| MANIFEST.dir_of(path))
    .collect();
  let mut out: Vec<String> = dirs
    .iter()
    .filter(|dir| !dirs.iter().any(|other| contains(other, dir)))
    .map(|dir| dir.to_string())
    .collect();
  out.sort();
  out.dedup();
  out
}

/// Is `inner` strictly under `outer`?
fn contains(outer: &str, inner: &str) -> bool {
  outer != inner && (outer.is_empty() || inner.starts_with(&format!("{outer}/")))
}

/// A project path for a document the export wrote relative to `root`, or
/// `None` where it lies outside the project.
fn project_path(root: &str, relative: &str) -> Option<String> {
  let mut parts: Vec<&str> = root.split('/').filter(|p| !p.is_empty()).collect();
  for part in relative.split('/') {
    match part {
      "" | "." => {}
      ".." => {
        parts.pop()?;
      }
      name => parts.push(name),
    }
  }
  Some(parts.join("/"))
}

/// A SCIP symbol as the Rust path a person would write for it, with no tool or
/// crate version in it: `rust-analyzer cargo intentsvcs 3.0.3
/// store/Store#open().` prints as `intentsvcs::store::Store::open()`.
///
/// **ONE PRINTED NAME PER DEFINITION, SO WHAT A NAME IS STAYS IN IT.** Rust lets
/// a field and a method share a name, and a function and a module; measured on
/// this repository's export, 30 pairs printed alike until a callable carried
/// `()`. A derive or attribute macro carries `#[..]` (`core::cmp::#[Ord]`) for
/// the same reason: it shares its name with the trait it derives.
///
/// **AN IMPL PRINTS AS THE TYPE IT IMPLEMENTS, AND A TRAIT IMPL NAMES ITS
/// TRAIT**, because one method name can be defined once per trait on one type:
/// `address/impl#[AddressError][Remedy]remedy().` is
/// `intentsvcs::address::<AddressError as Remedy>::remedy()`. A self type keeps
/// its generics as written, so two impls for two instantiations do not print
/// as one target.
///
/// **THE MODULE IN A MEMBER'S PATH IS THE ONE ITS IMPL BLOCK IS WRITTEN IN**,
/// which is not always the one its type is defined in: `impl Store` written in
/// `facade/x.rs` prints its method as `intentsvcs::facade::x::Store::helper()`.
/// SCIP does not carry the type's own module, so the path is not made
/// canonical (vc, 2026-09-17).
pub fn printable(symbol: &str) -> Option<String> {
  let mut parts = symbol.splitn(5, ' ');
  let (Some(_scheme), Some(_manager), Some(package), Some(_version), Some(descriptors)) = (
    parts.next(),
    parts.next(),
    parts.next(),
    parts.next(),
    parts.next(),
  ) else {
    return None;
  };
  let mut segments = vec![package.replace('-', "_")];
  // An `impl#` descriptor waiting for its self type and trait.
  let mut open_impl: Option<(Option<String>, Option<String>)> = None;
  let mut rest = descriptors;
  while !rest.is_empty() {
    if let Some(inner) = rest.strip_prefix('[') {
      let (name, after) = identifier(inner)?;
      rest = after.strip_prefix(']')?;
      match open_impl.as_mut() {
        Some((self_type @ None, _)) => *self_type = Some(name),
        Some((Some(_), trait_name @ None)) => *trait_name = Some(name),
        _ => segments.push(format!("[{name}]")),
      }
      continue;
    }
    if let Some(inner) = rest.strip_prefix('(') {
      let (name, after) = identifier(inner)?;
      rest = after.strip_prefix(')')?;
      segments.push(format!("({name})"));
      continue;
    }
    let (name, after) = identifier(rest)?;
    let suffix = *after.as_bytes().first()?;
    rest = match suffix {
      b'/' | b'#' | b'.' | b':' | b'!' => &after[1..],
      b'(' => &after[after.find(").")? + 2..],
      _ => return None,
    };
    if let Some(open) = open_impl.take() {
      segments.push(impl_segment(open, false));
    }
    match suffix {
      b'#' if name == "impl" => open_impl = Some((None, None)),
      b'!' => segments.push(format!("{name}!")),
      b'(' => segments.push(format!("{name}()")),
      b':' => segments.push(format!("#[{name}]")),
      _ => segments.push(name),
    }
  }
  if let Some(open) = open_impl.take() {
    segments.push(impl_segment(open, true));
  }
  Some(segments.join("::"))
}

/// An impl's segment: `Type` or `<Type as Trait>` before a member, and the
/// impl block itself, when nothing follows, as `<impl Type>` or `<impl Trait
/// for Type>` so it never prints as the type.
fn impl_segment((self_type, trait_name): (Option<String>, Option<String>), alone: bool) -> String {
  let self_type = self_type.unwrap_or_else(|| "_".to_string());
  match (trait_name, alone) {
    (Some(t), false) => format!("<{self_type} as {t}>"),
    (None, false) => self_type,
    (Some(t), true) => format!("<impl {t} for {self_type}>"),
    (None, true) => format!("<impl {self_type}>"),
  }
}

/// A SCIP name: a run of `[A-Za-z0-9_+-$]`, or a backquoted name in which a
/// doubled backquote is one.
fn identifier(s: &str) -> Option<(String, &str)> {
  if let Some(quoted) = s.strip_prefix('`') {
    let mut name = String::new();
    let mut chars = quoted.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
      if c != '`' {
        name.push(c);
      } else if chars.peek().is_some_and(|(_, next)| *next == '`') {
        name.push('`');
        chars.next();
      } else {
        return Some((name, &quoted[i + 1..]));
      }
    }
    return None;
  }
  let end = s
    .find(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '_' | '+' | '-' | '$')))
    .unwrap_or(s.len());
  (end > 0).then(|| (s[..end].to_string(), &s[end..]))
}

/// What one export says about the files it read.
///
/// `root` is the directory the export ran over, as [`roots`] names it; `bytes`
/// holds the files the tool read, by project path, where the reader knows the
/// bytes did not change while the tool read them. A document with no entry
/// there is not listed as read, so its references are dropped as unread rather
/// than joined against bytes nobody can vouch for.
pub fn trace_of(
  root: &str,
  index: &scip::Index,
  bytes: &BTreeMap<String, Vec<u8>>,
) -> Result<Trace, Unresolved> {
  let mut trace = Trace::default();
  let documents: Vec<(Option<String>, &scip::Document)> = index
    .documents
    .iter()
    .map(|doc| (project_path(root, &doc.relative_path), doc))
    .collect();

  let mut defined: BTreeMap<&str, Vec<(&str, u32)>> = BTreeMap::new();
  for (path, doc) in &documents {
    let Some(path) = path else { continue };
    for occ in doc.occurrences.iter().filter(|o| o.is_definition()) {
      defined
        .entry(occ.symbol.as_str())
        .or_default()
        .push((path.as_str(), occ.line + 1));
    }
  }

  for (path, doc) in &documents {
    if !matches!(doc.position_encoding, 0 | scip::UTF8_COLUMNS) {
      return Err(Unresolved::Failed {
        path: path.clone(),
        line: None,
        detail: format!(
          "the export counts this file's columns in position encoding {}, and this reader reads byte columns only",
          doc.position_encoding
        ),
      });
    }
    let text = path.as_ref().and_then(|p| bytes.get(p));
    if let (Some(path), Some(text)) = (path, text) {
      trace.read.push(Read {
        path: path.clone(),
        sha256: crate::model::sha256_hex(text),
      });
    }
    let lines: Vec<&[u8]> = text.map_or_else(Vec::new, |t| t.split(|b| *b == b'\n').collect());
    for occ in doc.occurrences.iter().filter(|o| !o.is_definition()) {
      // Empty where the range does not slice its line, or where the file was
      // not read, and then the core decides what the reference is.
      let name = lines
        .get(occ.line as usize)
        .and_then(|line| line.get(occ.column as usize..occ.end_column as usize))
        .and_then(|name| std::str::from_utf8(name).ok())
        .unwrap_or_default();
      let reason = if occ.symbol.starts_with("local ") {
        Some(LOCAL)
      } else if occ.end_line != occ.line {
        Some(MULTILINE)
      } else if is_operator(name) {
        Some(OPERATOR)
      } else {
        None
      };
      if let Some(reason) = reason {
        *trace.excluded.entry(reason.to_string()).or_default() += 1;
        continue;
      }
      let Some(target) = printable(&occ.symbol) else {
        *trace.excluded.entry(UNPRINTABLE.to_string()).or_default() += 1;
        continue;
      };
      let (target_path, target_line) = match defined.get(occ.symbol.as_str()).map(Vec::as_slice) {
        Some([(p, l)]) => (Some(p.to_string()), Some(*l)),
        _ => (None, None),
      };
      trace.references.push(Reference {
        path: path.clone(),
        line: Some(occ.line + 1),
        name: name.to_string(),
        target,
        target_path,
        target_line,
      });
    }
  }
  Ok(trace)
}

/// Does a reference's text hold no letter, digit or underscore? **EMPTY TEXT
/// IS NOT AN OPERATOR**, and neither is `self.0`, `Self`, `crate`, `super` or a
/// raw identifier: those are references only the toolchain sees, which the join
/// counts as unmatched, and this word must not swallow them (vc, 2026-09-17).
fn is_operator(text: &str) -> bool {
  !text.is_empty() && !text.chars().any(|c| c.is_alphanumeric() || c == '_')
}

/// rust-analyzer, run as a program found on `PATH` or named outright.
#[derive(Debug, Clone)]
pub struct RustAnalyzer {
  /// `rust-analyzer` by default. An arm names a program that does not exist
  /// to prove the absence answer without editing `PATH`.
  pub program: OsString,
}

impl Default for RustAnalyzer {
  fn default() -> Self {
    Self {
      program: OsString::from("rust-analyzer"),
    }
  }
}

impl Resolver for RustAnalyzer {
  fn lang(&self) -> &'static str {
    "rust"
  }

  fn tool(&self) -> &'static str {
    "rust-analyzer"
  }

  fn manifest(&self) -> Manifest {
    MANIFEST
  }

  fn excludes(&self) -> &'static [&'static str] {
    &[LOCAL, MULTILINE, OPERATOR, UNPRINTABLE]
  }

  /// **`full` CHANGES NOTHING HERE, AND THAT IS NOT AN OMISSION.** Every
  /// export is of the whole workspace, and cargo decides for itself which
  /// build scripts and proc macros to rebuild, so there is no incremental
  /// state of the reader's own for a full run to discard.
  fn trace(&self, scope: &Scope<'_>) -> Result<Trace, Unresolved> {
    let roots = roots(scope.indexed);
    if roots.is_empty() {
      return Err(Unresolved::NotApplicable {
        detail: "the index holds no Cargo.toml".to_string(),
      });
    }
    let mut whole = Trace::default();
    for root in &roots {
      let part = self.export(scope, root)?;
      whole.read.extend(part.read);
      whole.references.extend(part.references);
      for (reason, n) in part.excluded {
        *whole.excluded.entry(reason).or_default() += n;
      }
    }
    Ok(whole)
  }
}

impl RustAnalyzer {
  /// One export, over one root.
  ///
  /// **THE BYTES ARE HASHED BEFORE THE EXPORT AND READ AGAIN AFTER IT**, and
  /// only a file whose two hashes agree is offered as read. The export takes
  /// tens of seconds on a real workspace, and a file saved during it holds
  /// bytes the tool may not have read; offered as read, its references would
  /// join against lines the tool never saw.
  fn export(&self, scope: &Scope<'_>, root: &str) -> Result<Trace, Unresolved> {
    let manifest = if root.is_empty() {
      "Cargo.toml".to_string()
    } else {
      format!("{root}/Cargo.toml")
    };
    let failed = |detail: String| Unresolved::Failed {
      path: Some(manifest.clone()),
      line: None,
      detail,
    };
    let bytes_of = |path: &str| std::fs::read(scope.root.join(path)).ok();
    let before: BTreeMap<&str, String> = scope
      .indexed
      .iter()
      .filter(|p| p.ends_with(".rs") && contains(root, p))
      .filter_map(|p| bytes_of(p).map(|b| (p.as_str(), crate::model::sha256_hex(&b))))
      .collect();

    let output = scope.cache.join(if root.is_empty() {
      "root.scip".to_string()
    } else {
      format!("{}.scip", root.replace('/', "%"))
    });
    let ran = std::process::Command::new(&self.program)
      .arg("scip")
      .arg(scope.root.join(root))
      .arg("--output")
      .arg(&output)
      .env("CARGO_TARGET_DIR", scope.cache.join("target"))
      .stdin(std::process::Stdio::null())
      .output();
    let out = match ran {
      Ok(out) => out,
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        return Err(Unresolved::Missing {
          detail: format!("`{}` is not on PATH", self.program.to_string_lossy()),
        });
      }
      Err(e) => return Err(failed(format!("rust-analyzer could not be started: {e}"))),
    };
    if !out.status.success() {
      let stderr = String::from_utf8_lossy(&out.stderr);
      let said = stderr
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .unwrap_or("it printed nothing");
      return Err(failed(format!(
        "rust-analyzer's export exited with {}: {said}",
        out.status
      )));
    }
    let bytes = std::fs::read(&output).map_err(|e| {
      failed(format!(
        "rust-analyzer wrote no index at {}: {e}",
        output.display()
      ))
    })?;
    let index = scip::decode(&bytes).map_err(|e| failed(e.to_string()))?;

    let mut unchanged = BTreeMap::new();
    for doc in &index.documents {
      let Some(path) = project_path(root, &doc.relative_path) else {
        continue;
      };
      if let Some(after) = bytes_of(&path)
        && before.get(path.as_str()) == Some(&crate::model::sha256_hex(&after))
      {
        unchanged.insert(path, after);
      }
    }
    trace_of(root, &index, &unchanged)
  }
}
