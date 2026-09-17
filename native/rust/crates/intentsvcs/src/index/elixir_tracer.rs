//! Elixir's level-3 reader (ST0076 WP-06): the compiler's own tracer, run by
//! Mix into a build of Intent's own, read into the [`Trace`] the level-3 core
//! joins.
//!
//! **TWO HALVES, AND ONLY ONE OF THEM TOUCHES THE WORLD (IN-AG-PFIC-001).**
//! [`trace_of`], [`failure_of`], [`printed`] and [`first_clause`] are pure:
//! what a trace says, what a failed compile said, how a definition prints as a
//! target, and which of a function's clauses defines it. [`ElixirTracer`]
//! prepares the build, runs the compile, hashes the files and calls them.
//!
//! **THE COMPILE RUNS THE PROJECT'S OWN CODE**: its macros, the modules it
//! `use`s and its dependencies' code, which is why only `intent index resolve`
//! calls this.
//!
//! # A build of Intent's own
//!
//! **THE COMPILE WRITES UNDER THE RESOLVE CACHE AND NEVER THE PROJECT'S
//! `_build`** (vc, 2026-09-17, in AC-06.1), through `MIX_BUILD_PATH`. A shared
//! build lets the project's own compile consume the staleness an incremental
//! trace depends on: on a copy of Laksa, a file the user compiled was never
//! traced again. The first run seeds the build from the project's
//! `_build/dev/lib` where there is one ([`seed`]), and a build whose
//! `mix.lock` has changed since is made again. **A RUN THAT MAKES THE BUILD
//! FORCES THE COMPILE**, because a seeded build holds the user's compile state,
//! and a file the user compiled since the last run would read as up to date and
//! never be traced. Otherwise the first run and `--full` force, and every other
//! run is incremental. A dependency Mix must still build is built here, and
//! rebar3 writes its per-dependency cache files under `deps/`.
//!
//! **IT BUILDS [`ENV`] AND READS NO ENVIRONMENT VARIABLE** (vc, 2026-09-17,
//! AC-11.3): the compile is given `MIX_ENV` rather than inheriting it, so
//! Intent's own build is the same whatever shell started the run, and code only
//! another environment compiles, such as `test/support` under
//! `elixirc_paths(:test)`, is not traced.
//!
//! It reads a Mix project at the project root only (vc, 2026-09-17), which
//! [`MANIFEST`] says.
//!
//! # What a trace holds
//!
//! The tracer (`elixir_tracer.exs`) records calls and module references, each
//! with its line and column, and when each file started and finished. The
//! reference events it records are the population the core's conservation law
//! covers.
//!
//! **EXPANSION IS KEPT OUT TWICE** (vc decision 27). An event kind that is not
//! a reference, such as an alias or struct expansion or a module's definition,
//! is never recorded. And a call or module reference a macro generates arrives
//! as an ordinary event on the macro's line, where it can share a written
//! call's name: a written `to_string(total)` traces `Kernel.to_string/1`, and
//! its expansion traces `String.Chars.to_string/1` on the same line. **THE
//! COLUMN TELLS THEM APART.** Measured on Elixir 1.20.4 and OTP 29, every kind
//! of written reference carried `:column` in its meta and generated code
//! carried none, so an event with no column is excluded as [`EXPANSION`]. The
//! test cannot see a macro that reuses a written node's meta for a call it
//! generates, and that call is treated as written. An event nothing wrote that
//! gets past both finds no written row, and the join counts it unmatched.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, Stdio};

use super::resolved::{
  Manifest, OPERATOR, Read, Reference, Resolver, Scope, Trace, Unresolved, is_operator,
};
use super::symbols::Symbol;
use crate::model::sha256_hex;

/// A call or module reference a macro generated rather than the file wrote:
/// its meta carries no column.
pub const EXPANSION: &str = "expansion";
/// An imported call written inside a `quote`. The compiler names every arity
/// the import supplies, so the event asserts no one target (vc decision 27).
pub const QUOTED: &str = "quoted";

/// The file a Mix project is found by: `mix.exs`, at the project root only.
/// [`ElixirTracer`] reads it before it compiles, and so does a search answer
/// deciding whether Elixir can be `unresolved` here (ST0076 WP-07).
pub const MANIFEST: Manifest = Manifest {
  name: "mix.exs",
  root_only: true,
};

/// The Mix environment every run builds, and the name of its build directory.
pub const ENV: &str = "dev";

/// The tracer's source, embedded so that the one a run loads is this build's
/// own.
pub const TRACER: &str = include_str!("elixir_tracer.exs");
/// The tracer's module, as `mix compile --tracer` is told it.
pub const TRACER_MODULE: &str = "IntentLevel3Tracer";
/// What the tracer prints on stderr ahead of a fault it could not record.
pub const UNWRITTEN: &str = "intent-level3-tracer: the trace could not be written:";
/// The unit separator between a record's fields.
const FIELD: char = '\u{1f}';

/// What one traced compile says.
///
/// `roots` are the spellings of the project root a traced path can begin
/// with; `unchanged` holds each project path whose bytes hashed the same
/// before the compile and after it, with that hash; `forced` says the compile
/// was forced.
///
/// **A FILE IS READ ONLY IF IT COMPILED FROM START TO FINISH AND ITS BYTES DID
/// NOT MOVE MEANWHILE** (vc, 2026-09-17). An incremental compile starts only
/// the files it recompiles, so a file it left alone is not read, and the rows
/// an earlier run stored for it stay.
pub fn trace_of(
  log: &str,
  roots: &[String],
  unchanged: &BTreeMap<String, String>,
  forced: bool,
) -> Result<Trace, Unresolved> {
  let relative = |file: &str| {
    roots.iter().find_map(|root| {
      file
        .strip_prefix(root.as_str())
        .and_then(|rest| rest.strip_prefix('/'))
        .map(str::to_string)
    })
  };
  let unreadable = |record: &str| Unresolved::Failed {
    path: None,
    line: None,
    detail: format!("the tracer wrote a record this reader cannot read: {record:?}"),
  };
  let number = |field: &str, record: &str| match field {
    "" => Ok(None),
    digits => digits
      .parse::<u32>()
      .map(Some)
      .map_err(|_| unreadable(record)),
  };

  let mut trace = Trace::default();
  let mut exclude = |reason: &str| *trace.excluded.entry(reason.to_string()).or_default() += 1;
  let mut references = Vec::new();
  // Per file as the compiler named it: how often it started, and finished.
  let mut compiled: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
  let mut recorded = 0u64;
  let mut columned = false;
  for record in log.lines() {
    let fields: Vec<&str> = record.split(FIELD).collect();
    let (file, line, column, name, target) = match fields.as_slice() {
      ["S", file] => {
        compiled.entry(*file).or_default().0 += 1;
        continue;
      }
      ["E", file] => {
        compiled.entry(*file).or_default().1 += 1;
        continue;
      }
      ["Q", _, line] => {
        number(line, record)?;
        exclude(QUOTED);
        continue;
      }
      ["F", file, message] => {
        return Err(Unresolved::Failed {
          path: relative(file),
          line: None,
          detail: format!("the tracer failed on an event in {file}: {message}"),
        });
      }
      ["C", file, line, column, module, name, arity] => {
        let arity = number(arity, record)?.ok_or_else(|| unreadable(record))?;
        (
          *file,
          *line,
          *column,
          *name,
          format!("{module}.{name}/{arity}"),
        )
      }
      ["M", file, line, column, module] => (*file, *line, *column, *module, module.to_string()),
      _ => return Err(unreadable(record)),
    };
    let line = number(line, record)?;
    recorded += 1;
    if column.is_empty() {
      exclude(EXPANSION);
      continue;
    }
    number(column, record)?;
    columned = true;
    if is_operator(name) {
      exclude(OPERATOR);
      continue;
    }
    references.push(Reference {
      path: relative(file),
      line,
      name: name.to_string(),
      target,
      target_path: None,
      target_line: None,
    });
  }
  trace.references = references;

  if let Some((file, _)) = compiled
    .iter()
    .find(|(_, (started, finished))| started != finished)
  {
    return Err(Unresolved::Failed {
      path: relative(file),
      line: None,
      detail: format!("the compile started {file} and did not finish it"),
    });
  }
  // **NEVER A RUN THAT DROPS EVERYTHING AND READS AS CURRENT** (vc decision 27).
  // **ONLY A FORCED COMPILE CAN SHOW AN ELIXIR THAT RECORDS NO COLUMNS** (vc,
  // 2026-09-17): every `defmodule` traces a call with no column, so an
  // incremental compile of a file holding no written reference, such as an
  // empty module, records only columnless events, while a forced compile of a
  // project holding any written reference records at least one column.
  if forced && recorded > 0 && !columned {
    return Err(Unresolved::Failed {
      path: None,
      line: None,
      detail: "none of the compiler's references carried a column: this Elixir records no \
               columns, and level 3 tells a written reference from a macro's expansion by its \
               column"
        .to_string(),
    });
  }
  trace.read = compiled
    .keys()
    .filter_map(|file| {
      let path = relative(file)?;
      let sha256 = unchanged.get(&path)?.clone();
      Some(Read { path, sha256 })
    })
    .collect();
  Ok(trace)
}

/// What a compile that did not succeed said: the file and line Mix names,
/// where it names one, and the first error it printed.
///
/// **THE EXIT STATUS IS WHAT NAMES A FAILED COMPILE, AND THIS ONLY DESCRIBES
/// IT.** A syntax error fails before its file starts, and an undefined function
/// runs its file from start to finish and still exits 1 (measured on Elixir
/// 1.20.4). Mix names the file in `== Compilation error in file <path> ==` and
/// the line as `<path>:<line>` in the diagnostic; a failure that names no file,
/// such as a dependency that was never fetched, has no path.
pub fn failure_of(output: &str, status: &str) -> Unresolved {
  let path = output.lines().find_map(|line| {
    line
      .trim()
      .strip_prefix("== Compilation error in file ")?
      .strip_suffix(" ==")
      .map(str::to_string)
  });
  let line = path.as_deref().and_then(|path| {
    let at = format!("{path}:");
    output.match_indices(at.as_str()).find_map(|(i, _)| {
      let digits: String = output[i + at.len()..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
      digits.parse::<u32>().ok()
    })
  });
  let mut lines = output.lines().map(str::trim);
  let said = lines
    .clone()
    .find_map(|l| l.strip_prefix("error: "))
    .or_else(|| lines.clone().find(|l| l.starts_with("** (")))
    .or_else(|| lines.rfind(|l| !l.is_empty()))
    .unwrap_or("it printed nothing");
  Unresolved::Failed {
    path,
    line,
    detail: format!("the compile exited with {status}: {said}"),
  }
}

/// A definition row printed the way a call to it prints its target: a module
/// or protocol as `inspect` prints it, a function as `Module.name/arity`.
///
/// **A FUNCTION WITH DEFAULT ARGUMENTS PRINTS AT ITS FULL ARITY**, so a call at
/// a lower arity it also defines is placed nowhere. A clause inside `defimpl`
/// prints nothing: its module joins the protocol to the `for:` type, which the
/// row does not hold.
pub fn printed(def: &Symbol) -> Option<String> {
  match def.subkind.as_str() {
    "module" | "protocol" => Some(match &def.container {
      Some(container) => format!("{container}.{}", def.name),
      None => def.name.clone(),
    }),
    "def" | "defp" | "defmacro" | "defmacrop" | "defguard" | "defguardp" | "defdelegate"
    | "defn" | "defnp"
      if def.container_kind.as_deref() != Some("impl") =>
    {
      Some(format!(
        "{}.{}/{}",
        def.container.as_deref()?,
        def.name,
        def.arity?
      ))
    }
    _ => None,
  }
}

/// Which of the rows printing one target defines it: the first clause, when
/// every row is in one file. Elixir writes a row per clause, and a function's
/// clauses in one file are one function; rows in two files are two
/// definitions, and the core will not choose between them.
pub fn first_clause<'a>(defs: &[&'a Symbol]) -> Option<&'a Symbol> {
  let first = defs.first()?;
  if defs.iter().all(|def| def.path == first.path) {
    defs.iter().copied().min_by_key(|def| def.span.start_line)
  } else {
    None
  }
}

/// Copy a Mix build's `lib` directory, re-pointing every relative symlink at
/// what it resolved to from the directory the link was in.
///
/// **MIX LINKS `priv` RELATIVELY**, so a link copied as it is resolves to
/// nothing from its new place: on a copy of Laksa all 54 were broken, and Mix
/// re-pointed only the project's own. Each file keeps its modification time,
/// as the copy measured there did.
pub fn seed(from: &Path, to: &Path) -> std::io::Result<()> {
  std::fs::create_dir_all(to)?;
  for entry in std::fs::read_dir(from)? {
    let entry = entry?;
    let (source, target) = (entry.path(), to.join(entry.file_name()));
    let kind = entry.file_type()?;
    if kind.is_symlink() {
      // A relative link resolves from the directory it is in; an absolute one
      // replaces that directory in the join.
      std::os::unix::fs::symlink(from.join(std::fs::read_link(&source)?), &target)?;
    } else if kind.is_dir() {
      seed(&source, &target)?;
    } else {
      std::fs::copy(&source, &target)?;
      std::fs::File::open(&target)?.set_modified(entry.metadata()?.modified()?)?;
    }
  }
  Ok(())
}

/// Make the build directory ready for a run: `Ok(true)` when this run made it,
/// seeded or empty, so the compile must be forced.
///
/// **THE HASH OF `mix.lock` IS KEPT BESIDE THE BUILD** (`dev.lock`), and a
/// build with no hash kept, or another one, is made again: its dependencies
/// were built against a lock the project has left.
fn prepare(root: &Path, cache: &Path) -> std::io::Result<bool> {
  let (build, kept) = (cache.join(ENV), cache.join(format!("{ENV}.lock")));
  let lock = match std::fs::read(root.join("mix.lock")) {
    Ok(bytes) => sha256_hex(&bytes),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => "none".to_string(),
    Err(e) => return Err(e),
  };
  let before = match std::fs::read_to_string(&kept) {
    Ok(text) => Some(text),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
    Err(e) => return Err(e),
  };
  if build.is_dir() && before.as_deref() == Some(lock.as_str()) {
    return Ok(false);
  }
  if build.symlink_metadata().is_ok() {
    std::fs::remove_dir_all(&build)?;
  }
  let from = root.join("_build").join(ENV).join("lib");
  if from.is_dir() {
    seed(&from, &build.join("lib"))?;
  } else {
    std::fs::create_dir_all(&build)?;
  }
  std::fs::write(&kept, lock)?;
  Ok(true)
}

/// The hash of each of `paths` that can be read now, by project path.
fn digests(root: &Path, paths: &[&str]) -> BTreeMap<String, String> {
  paths
    .iter()
    .filter_map(|path| {
      let bytes = std::fs::read(root.join(path)).ok()?;
      Some((path.to_string(), sha256_hex(&bytes)))
    })
    .collect()
}

/// The spellings of the project root a traced path can begin with: as the
/// scope names it, and resolved, because the compiler names files under the
/// directory it runs in as the operating system resolved it (macOS's `/var` is
/// `/private/var`).
fn roots(root: &Path) -> Vec<String> {
  let mut out = vec![root.to_string_lossy().trim_end_matches('/').to_string()];
  if let Ok(real) = std::fs::canonicalize(root) {
    let real = real.to_string_lossy().trim_end_matches('/').to_string();
    if !out.contains(&real) {
      out.push(real);
    }
  }
  out
}

/// Elixir's compiler, run through Mix as a program found on `PATH` or named
/// outright.
#[derive(Debug, Clone)]
pub struct ElixirTracer {
  /// `elixir` by default. An arm names a program that does not exist to prove
  /// the absence answer without editing `PATH`.
  pub program: OsString,
}

impl Default for ElixirTracer {
  fn default() -> Self {
    Self {
      program: OsString::from("elixir"),
    }
  }
}

impl Resolver for ElixirTracer {
  fn lang(&self) -> &'static str {
    "elixir"
  }

  fn tool(&self) -> &'static str {
    "elixir"
  }

  fn manifest(&self) -> Manifest {
    MANIFEST
  }

  fn excludes(&self) -> &'static [&'static str] {
    &[EXPANSION, OPERATOR, QUOTED]
  }

  fn trace(&self, scope: &Scope<'_>) -> Result<Trace, Unresolved> {
    if !scope
      .indexed
      .iter()
      .any(|path| MANIFEST.dir_of(path).is_some())
    {
      return Err(Unresolved::NotApplicable {
        detail: format!("no `{}` at the project root", MANIFEST.name),
      });
    }
    self.compile(scope)
  }

  fn target_of(&self, def: &Symbol) -> Option<String> {
    printed(def)
  }

  fn locate<'a>(&self, defs: &[&'a Symbol]) -> Option<&'a Symbol> {
    first_clause(defs)
  }
}

impl ElixirTracer {
  /// One traced compile of the Mix project at the root.
  ///
  /// **THE FILES ARE HASHED BEFORE THE COMPILE AND READ AGAIN AFTER IT**, and
  /// only a file whose two hashes agree can be read, as the Rust reader does: a
  /// file saved during the compile holds bytes the compiler may not have read.
  fn compile(&self, scope: &Scope<'_>) -> Result<Trace, Unresolved> {
    let failed = |detail: String| Unresolved::Failed {
      path: None,
      line: None,
      detail,
    };
    let build = scope.cache.join(ENV);
    let made = prepare(scope.root, scope.cache).map_err(|e| {
      failed(format!(
        "Intent could not prepare its build directory {}: {e}",
        build.display()
      ))
    })?;
    let tracer = scope.cache.join("elixir_tracer.exs");
    std::fs::write(&tracer, TRACER).map_err(|e| {
      failed(format!(
        "Intent could not write its tracer to {}: {e}",
        tracer.display()
      ))
    })?;
    let log = scope.cache.join(format!("{ENV}.trace"));
    match std::fs::remove_file(&log) {
      Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
        return Err(failed(format!(
          "Intent could not clear the last trace at {}: {e}",
          log.display()
        )));
      }
      _ => {}
    }

    let sources: Vec<&str> = scope
      .indexed
      .iter()
      .map(String::as_str)
      .filter(|path| path.ends_with(".ex"))
      .collect();
    let before = digests(scope.root, &sources);
    let forced = scope.full || made;
    let mut command = Command::new(&self.program);
    command
      .arg("-r")
      .arg(&tracer)
      .args(["-S", "mix", "compile"]);
    if forced {
      command.arg("--force");
    }
    let ran = command
      .args(["--tracer", TRACER_MODULE])
      .current_dir(scope.root)
      .env("MIX_ENV", ENV)
      .env("MIX_BUILD_PATH", &build)
      .env("INTENT_TRACE", &log)
      .stdin(Stdio::null())
      .output();
    let out = match ran {
      Ok(out) => out,
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        return Err(Unresolved::Missing {
          detail: format!("`{}` is not on PATH", self.program.to_string_lossy()),
        });
      }
      Err(e) => return Err(failed(format!("elixir could not be started: {e}"))),
    };
    let said = format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    );
    if !out.status.success() {
      return Err(failure_of(&said, &out.status.to_string()));
    }
    if let Some(fault) = said.lines().find(|line| line.contains(UNWRITTEN)) {
      return Err(failed(fault.trim().to_string()));
    }
    let text = std::fs::read_to_string(&log).map_err(|e| {
      failed(format!(
        "the compile wrote no trace at {}: {e}",
        log.display()
      ))
    })?;
    // **A FORCED COMPILE THAT TRACED NOTHING NEVER CALLED THE TRACER**, since
    // forcing starts every file Mix compiles; stored, it would read as a
    // current run that resolved nothing.
    if forced && text.is_empty() && !sources.is_empty() {
      return Err(failed(
        "a forced compile of a project holding `.ex` files traced nothing: Mix never called \
         the tracer"
          .to_string(),
      ));
    }
    let after = digests(scope.root, &sources);
    let unchanged = before
      .into_iter()
      .filter(|(path, sha)| after.get(path) == Some(sha))
      .collect();
    trace_of(&text, &roots(scope.root), &unchanged, forced)
  }
}

#[cfg(test)]
mod tests {
  use super::super::symbols::{Span, SymbolKind};
  use super::*;

  const ROOT: &str = "/work/shop";

  /// A trace record, its fields joined as the tracer joins them.
  fn rec(fields: &[&str]) -> String {
    format!("{}\n", fields.join("\u{1f}"))
  }

  fn file(path: &str) -> String {
    format!("{ROOT}/{path}")
  }

  fn unchanged(paths: &[&str]) -> BTreeMap<String, String> {
    paths
      .iter()
      .map(|p| (p.to_string(), format!("sha-of-{p}")))
      .collect()
  }

  fn failed_detail(result: Result<Trace, Unresolved>) -> (Option<String>, String) {
    match result {
      Err(Unresolved::Failed { path, detail, .. }) => (path, detail),
      other => panic!("expected a failed run, got {other:?}"),
    }
  }

  #[test]
  fn a_trace_names_each_written_reference_and_counts_what_it_excludes_by_reason() {
    let shop = file("lib/shop.ex");
    let log = [
      rec(&["S", &shop]),
      rec(&["C", &shop, "11", "18", "Shop.Repo", "get", "2"]),
      rec(&["C", &shop, "16", "9", "Kernel", "to_string", "1"]),
      rec(&["C", &shop, "16", "", "String.Chars", "to_string", "1"]),
      rec(&["C", &shop, "18", "20", "Shop", "local", "1"]),
      rec(&["C", &shop, "18", "18", "Kernel", "+", "2"]),
      rec(&["C", &shop, "", "", "Enum", "reduce", "3"]),
      rec(&["M", &shop, "6", "7", "Shop.Schema"]),
      rec(&["M", &shop, "6", "", "Shop.Schema"]),
      rec(&["M", &shop, "3", "3", ":crypto"]),
      rec(&["Q", &shop, "36"]),
      rec(&[
        "C",
        "/usr/lib/elixir/lib/kernel.ex",
        "40",
        "5",
        "Kernel",
        "def",
        "2",
      ]),
      rec(&["E", &shop]),
    ]
    .concat();

    let trace = trace_of(
      &log,
      &[ROOT.to_string()],
      &unchanged(&["lib/shop.ex"]),
      true,
    )
    .expect("a complete trace reads");

    let got: Vec<_> = trace
      .references
      .iter()
      .map(|r| {
        (
          r.path.as_deref(),
          r.line,
          r.name.as_str(),
          r.target.as_str(),
        )
      })
      .collect();
    assert_eq!(
      got,
      vec![
        (Some("lib/shop.ex"), Some(11), "get", "Shop.Repo.get/2"),
        (
          Some("lib/shop.ex"),
          Some(16),
          "to_string",
          "Kernel.to_string/1"
        ),
        (Some("lib/shop.ex"), Some(18), "local", "Shop.local/1"),
        (Some("lib/shop.ex"), Some(6), "Shop.Schema", "Shop.Schema"),
        (Some("lib/shop.ex"), Some(3), ":crypto", ":crypto"),
        (None, Some(40), "def", "Kernel.def/2"),
      ],
      "a call names module.name/arity, a module reference names itself, and a file outside \
       the root has no project path"
    );
    assert_eq!(
      trace.excluded,
      BTreeMap::from([
        (EXPANSION.to_string(), 3),
        (OPERATOR.to_string(), 1),
        (QUOTED.to_string(), 1),
      ]),
      "the to_string expansion, use's generated reference and the lineless reduce carry no \
       column; `+` is an operator; the quoted import names no one target"
    );
    assert_eq!(
      trace.read,
      vec![Read {
        path: "lib/shop.ex".to_string(),
        sha256: "sha-of-lib/shop.ex".to_string(),
      }]
    );
  }

  #[test]
  fn a_file_is_read_only_if_it_finished_under_the_root_with_its_bytes_unmoved() {
    let (done, moved, left) = (
      file("lib/done.ex"),
      file("lib/moved.ex"),
      file("lib/left.ex"),
    );
    let log = [
      rec(&["S", &done]),
      rec(&["E", &done]),
      rec(&["S", &moved]),
      rec(&["E", &moved]),
      rec(&["C", &left, "2", "3", "Map", "get", "2"]),
    ]
    .concat();
    let resolved = format!("/private{ROOT}");

    let trace = trace_of(
      &log.replace(ROOT, &resolved),
      &[ROOT.to_string(), resolved.clone()],
      &unchanged(&["lib/done.ex", "lib/left.ex"]),
      false,
    )
    .expect("reads");

    assert_eq!(
      trace
        .read
        .iter()
        .map(|r| r.path.as_str())
        .collect::<Vec<_>>(),
      vec!["lib/done.ex"],
      "moved.ex changed during the compile and left.ex was not compiled this run, so neither \
       is read, and a root's resolved spelling is a root"
    );
    assert_eq!(trace.references[0].path.as_deref(), Some("lib/left.ex"));
  }

  #[test]
  fn an_unfinished_file_a_tracer_fault_a_torn_record_or_no_column_at_all_fails_the_run() {
    let a = file("lib/a.ex");
    let unfinished = [
      rec(&["S", &a]),
      rec(&["C", &a, "2", "3", "Map", "get", "2"]),
    ]
    .concat();
    assert_eq!(
      failed_detail(trace_of(
        &unfinished,
        &[ROOT.to_string()],
        &BTreeMap::new(),
        true
      )),
      (
        Some("lib/a.ex".to_string()),
        format!("the compile started {a} and did not finish it")
      )
    );

    let fault = rec(&["F", &a, "no match of right hand side value: :oops"]);
    let (path, detail) = failed_detail(trace_of(
      &fault,
      &[ROOT.to_string()],
      &BTreeMap::new(),
      true,
    ));
    assert_eq!(path.as_deref(), Some("lib/a.ex"));
    assert!(detail.contains(":oops"), "{detail}");

    let torn = format!("C\u{1f}{a}\u{1f}2\n");
    let (_, detail) = failed_detail(trace_of(&torn, &[ROOT.to_string()], &BTreeMap::new(), true));
    assert!(detail.contains("cannot read"), "{detail}");

    let columnless = [
      rec(&["S", &a]),
      rec(&["C", &a, "2", "", "Map", "get", "2"]),
      rec(&["M", &a, "2", "", "Map"]),
      rec(&["E", &a]),
    ]
    .concat();
    let (_, detail) = failed_detail(trace_of(
      &columnless,
      &[ROOT.to_string()],
      &BTreeMap::new(),
      true,
    ));
    assert!(
      detail.contains("records no columns"),
      "a forced compile whose every reference would drop as expansion fails rather than reading \
       as current: {detail}"
    );
    let empty_module = [
      rec(&["S", &a]),
      rec(&["C", &a, "1", "", ":elixir_utils", "noop", "0"]),
      rec(&["E", &a]),
    ]
    .concat();
    assert!(
      trace_of(&empty_module, &[ROOT.to_string()], &BTreeMap::new(), false).is_ok(),
      "an incremental compile of a file holding no written reference records only the \
       columnless call every `defmodule` traces, and that is not a failure"
    );

    assert_eq!(
      trace_of("", &[ROOT.to_string()], &BTreeMap::new(), true),
      Ok(Trace::default()),
      "an incremental compile that recompiled nothing traces nothing, and that is not a failure"
    );
  }

  #[test]
  fn a_failed_compile_is_named_by_the_file_and_line_mix_prints() {
    // Captured from Elixir 1.20.4's `mix compile`, stdout then stderr.
    let undefined = "Compiling 1 file (.ex)\n    error: undefined function nope/0 (expected Shop.Broken to define such a function or for it to be imported, but none are available)\n    \u{2502}\n  4 \u{2502}     nope()\n    \u{2502}     ^^^^\n    \u{2502}\n    \u{2514}\u{2500} lib/shop/broken.ex:4:5: Shop.Broken.run/1\n\n\n== Compilation error in file lib/shop/broken.ex ==\n** (CompileError) lib/shop/broken.ex: cannot compile module Shop.Broken (errors have been logged)\n\n";
    let syntax = "Compiling 1 file (.ex)\n\n== Compilation error in file lib/shop/broken.ex ==\n** (MismatchedDelimiterError) mismatched delimiter found on lib/shop/broken.ex:3:11:\n    error: unexpected token: ]\n    \u{2502}\n  3 \u{2502}     [x, (x]\n    \u{2502}\n    \u{2514}\u{2500} lib/shop/broken.ex:3:11\n";
    let unfetched = "Unchecked dependencies for environment dev:\n* jason (Hex package)\n  the dependency is not available, run \"mix deps.get\"\n** (Mix) Can't continue due to errors on dependencies\n";

    assert_eq!(
      failure_of(undefined, "exit status: 1"),
      Unresolved::Failed {
        path: Some("lib/shop/broken.ex".to_string()),
        line: Some(4),
        detail: "the compile exited with exit status: 1: undefined function nope/0 (expected \
                 Shop.Broken to define such a function or for it to be imported, but none are \
                 available)"
          .to_string(),
      }
    );
    assert!(matches!(
      failure_of(syntax, "exit status: 1"),
      Unresolved::Failed { path: Some(p), line: Some(3), detail } if p == "lib/shop/broken.ex" && detail.ends_with("unexpected token: ]")
    ));
    assert_eq!(
      failure_of(unfetched, "exit status: 1"),
      Unresolved::Failed {
        path: None,
        line: None,
        detail: "the compile exited with exit status: 1: ** (Mix) Can't continue due to errors \
                 on dependencies"
          .to_string(),
      }
    );
  }

  fn def(
    path: &str,
    subkind: &str,
    name: &str,
    line: u32,
    container: Option<(&str, &str)>,
    arity: Option<u32>,
  ) -> Symbol {
    Symbol {
      path: path.to_string(),
      lang: "elixir",
      name: name.to_string(),
      kind: SymbolKind::Def,
      span: Span {
        start_line: line,
        end_line: line,
      },
      subkind: subkind.to_string(),
      container: container.map(|(c, _)| c.to_string()),
      container_kind: container.map(|(_, k)| k.to_string()),
      trait_name: None,
      arity,
      arity_min: arity,
      qualifier: None,
      level: 1,
    }
  }

  #[test]
  fn a_definition_prints_the_target_a_call_names_and_its_first_clause_in_one_file_places_it() {
    let module = Some(("Intent.Store.Rows", "module"));
    let top = def("lib/store.ex", "module", "Intent.Store", 1, None, None);
    let nested = def(
      "lib/store.ex",
      "module",
      "Rows",
      2,
      Some(("Intent.Store", "module")),
      None,
    );
    let second = def("lib/store.ex", "def", "open", 5, module, Some(1));
    let first = def("lib/store.ex", "def", "open", 4, module, Some(1));
    let private = def("lib/store.ex", "defp", "count", 6, module, Some(0));
    let in_impl = def(
      "lib/size.ex",
      "def",
      "size",
      3,
      Some(("Size", "impl")),
      Some(1),
    );
    let elsewhere = def("test/support/store.ex", "def", "open", 9, module, Some(1));

    assert_eq!(
      [&top, &nested, &first, &private, &in_impl]
        .map(printed)
        .to_vec(),
      vec![
        Some("Intent.Store".to_string()),
        Some("Intent.Store.Rows".to_string()),
        Some("Intent.Store.Rows.open/1".to_string()),
        Some("Intent.Store.Rows.count/0".to_string()),
        None,
      ]
    );
    assert_eq!(
      first_clause(&[&second, &first]).map(|d| d.span.start_line),
      Some(4),
      "a function's clauses in one file are one function, defined at its first"
    );
    assert_eq!(
      first_clause(&[&first, &elsewhere]),
      None,
      "two files are two definitions"
    );
  }

  #[test]
  fn a_seed_repoints_every_relative_link_at_what_it_resolved_to_and_keeps_modification_times() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("priv")).expect("priv");
    std::fs::write(root.join("priv/data.txt"), "held").expect("data");
    let lib = root.join("_build/dev/lib");
    std::fs::create_dir_all(lib.join("shop/ebin")).expect("ebin");
    let beam = lib.join("shop/ebin/Elixir.Shop.beam");
    std::fs::write(&beam, "beam").expect("beam");
    let old = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
    std::fs::File::open(&beam)
      .and_then(|f| f.set_modified(old))
      .expect("age the beam");
    std::os::unix::fs::symlink("../../../../priv", lib.join("shop/priv")).expect("link");

    let to = root.join("intent/.cache/resolve/elixir/dev/lib");
    seed(&lib, &to).expect("seed");

    assert_eq!(
      std::fs::read_to_string(to.join("shop/priv/data.txt")).expect("the link resolves"),
      "held"
    );
    assert_eq!(
      std::fs::metadata(to.join("shop/ebin/Elixir.Shop.beam"))
        .and_then(|m| m.modified())
        .expect("mtime"),
      old
    );
  }

  #[test]
  fn the_build_is_made_on_the_first_run_and_again_only_when_mix_lock_moves() {
    let dir = tempfile::tempdir().expect("tempdir");
    let (root, cache) = (dir.path(), dir.path().join("intent/.cache/resolve/elixir"));
    std::fs::create_dir_all(root.join("_build/dev/lib/shop")).expect("build");
    std::fs::write(root.join("_build/dev/lib/shop/marker"), "seeded").expect("marker");
    std::fs::write(root.join("mix.lock"), "%{a: 1}").expect("lock");
    std::fs::create_dir_all(&cache).expect("cache");

    assert_eq!(
      prepare(root, &cache).ok(),
      Some(true),
      "the first run makes it"
    );
    assert!(
      cache.join("dev/lib/shop/marker").is_file(),
      "seeded from the project's build"
    );
    std::fs::write(cache.join("dev/lib/shop/compiled"), "by a run").expect("a run's output");
    assert_eq!(
      prepare(root, &cache).ok(),
      Some(false),
      "an unchanged lock keeps it"
    );
    assert!(cache.join("dev/lib/shop/compiled").is_file());

    std::fs::write(root.join("mix.lock"), "%{a: 2}").expect("lock moves");
    assert_eq!(
      prepare(root, &cache).ok(),
      Some(true),
      "a moved lock makes it again"
    );
    assert!(
      !cache.join("dev/lib/shop/compiled").exists(),
      "and nothing the old build held survives it"
    );
    assert!(
      root.join("_build/dev/lib/shop/marker").is_file()
        && !root.join("_build/dev/lib/shop/compiled").exists(),
      "the project's own build is only ever read"
    );
  }

  #[test]
  fn the_tracer_file_names_the_module_and_the_fault_prefix_this_reader_looks_for() {
    assert!(TRACER.contains(&format!("defmodule {TRACER_MODULE} do")));
    assert!(TRACER.contains(&format!("@unwritten \"{UNWRITTEN}\"")));
  }
}
