//! AT-06.1 / AC-06.1 (ST0076 WP-06): Elixir's reader, the compiler's tracer
//! read into level 3 through the one door both languages use.
//!
//! **HERMETIC WHERE IT CAN BE, AND ONE ARM THAT IS NOT.** A trace of the tiny
//! Mix project below is checked in (`tests/fixtures/tracer/shop.trace`: `mix
//! compile --force` with Intent's tracer on Elixir 1.20.4 and OTP 29, the
//! project root written as `{ROOT}` and the fields separated by tabs), so the
//! parse and the join run against real compiler output in every build. The
//! build directory, when a compile is forced and what a failed compile records
//! are proven through a stand-in program, and the absence of the tool by naming
//! a program that does not exist. Only the arm under `needs_the_toolchain` runs
//! the real compiler, and it is ignored in CI and run by `bin/devbin test all`
//! (hv, 2026-09-17).

use crate::common::{Fixture, git_init_at};
use intentsvcs::facade::Facade;
use intentsvcs::index::elixir_tracer::{self, ElixirTracer};
use intentsvcs::index::resolved::{
  Manifest, OPERATOR, Outcome, Resolver, Scope, Trace, Unresolved,
};
use intentsvcs::index::symbols::Symbol;
use intentsvcs::model::sha256_hex;
use std::collections::BTreeMap;

const MIX_EXS: &str = r#"defmodule Shop.MixProject do
  use Mix.Project

  def project do
    [app: :shop, version: "0.1.0", elixir: "~> 1.15", deps: []]
  end
end
"#;

const SHOP: &str = r#"defmodule Shop do
  alias Shop.Repo
  use Shop.Schema

  def fetch(id) do
    order = Repo.get(Shop.Order, id)
    total = Map.get(order, :total, 0) + 1
    _ = Access.get(order, :total)
    _ = Keyword.get([total: total], :total)
    to_string(local(total))
  end

  def totals(orders), do: Enum.map(orders, &local/1)

  defp local(value), do: value
end
"#;

const SUPPORT: &str = r#"defmodule Shop.Repo do
  def get(_schema, id), do: %{id: id, total: 1}
end

defmodule Shop.Order do
  defstruct [:id, :total]
end

defmodule Shop.Schema do
  defmacro __using__(_opts) do
    quote do
      def schema?, do: true
    end
  end
end
"#;

/// The tracer's records for a forced compile of the project above.
const TRACE: &str = include_str!("fixtures/tracer/shop.trace");

/// The checked-in trace as the tracer writes it, for a project at `root`.
fn trace_at(root: &str) -> String {
  TRACE.replace("{ROOT}", root).replace('\t', "\u{1f}")
}

/// Declare elixir, which the fixture's default config does not.
fn declare_elixir(fx: &Fixture) {
  fx.write_file(
    "intent/.config/config.json",
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"dc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"elixir\"]\n}\n",
  );
}

/// The Mix project at the root of a project declaring elixir, indexed by the
/// real extractor.
fn shop() -> (Fixture, Facade) {
  let fx = Fixture::new();
  git_init_at(fx.root());
  fx.write_file(".gitignore", "intent/.cache/\n_build/\ndeps/\n");
  declare_elixir(&fx);
  fx.write_file("mix.exs", MIX_EXS);
  fx.write_file("lib/shop.ex", SHOP);
  fx.write_file("lib/shop/support.ex", SUPPORT);
  let mut facade = fx.facade_on_disk();
  facade.index_rebuild().expect("rebuild");
  (fx, facade)
}

/// A reader that reads the checked-in trace instead of running the compiler.
struct CheckedIn;

impl Resolver for CheckedIn {
  fn lang(&self) -> &'static str {
    "elixir"
  }
  fn tool(&self) -> &'static str {
    "elixir"
  }
  fn manifest(&self) -> Manifest {
    ElixirTracer::default().manifest()
  }
  fn excludes(&self) -> &'static [&'static str] {
    ElixirTracer::default().excludes()
  }
  fn trace(&self, scope: &Scope<'_>) -> Result<Trace, Unresolved> {
    let root = scope.root.to_string_lossy().to_string();
    let unchanged = BTreeMap::from([
      ("lib/shop.ex".to_string(), sha256_hex(SHOP.as_bytes())),
      (
        "lib/shop/support.ex".to_string(),
        sha256_hex(SUPPORT.as_bytes()),
      ),
    ]);
    // Captured from a forced compile.
    elixir_tracer::trace_of(&trace_at(&root), &[root], &unchanged, true)
  }
  fn target_of(&self, def: &Symbol) -> Option<String> {
    elixir_tracer::printed(def)
  }
  fn locate<'a>(&self, defs: &[&'a Symbol]) -> Option<&'a Symbol> {
    elixir_tracer::first_clause(defs)
  }
}

fn resolve(
  facade: &mut Facade,
  lang: Option<&str>,
  full: bool,
  reader: Box<dyn Resolver>,
) -> Outcome {
  facade
    .index_resolve(lang, full, &[reader])
    .expect("the door answers")
}

/// A resolved row as an arm expects it: line, name, target, and where the
/// target is defined when the project defines it.
type Expected = (
  u32,
  &'static str,
  &'static str,
  Option<&'static str>,
  Option<u32>,
);

/// The rows the written references in `lib/shop.ex` resolve to, from the trace
/// the compiler writes for them.
fn shop_rows() -> Vec<Expected> {
  let support = Some("lib/shop/support.ex");
  vec![
    (2, "Shop.Repo", "Shop.Repo", support, Some(1)),
    (3, "Shop.Schema", "Shop.Schema", support, Some(9)),
    (6, "get", "Shop.Repo.get/2", support, Some(2)),
    (7, "get", "Map.get/3", None, None),
    (8, "get", "Access.get/2", None, None),
    (9, "get", "Keyword.get/2", None, None),
    (10, "to_string", "Kernel.to_string/1", None, None),
    (10, "local", "Shop.local/1", Some("lib/shop.ex"), Some(15)),
    (13, "map", "Enum.map/2", None, None),
    (13, "local", "Shop.local/1", Some("lib/shop.ex"), Some(15)),
  ]
}

fn assert_shop_resolved(fx: &Fixture) {
  let rows = fx.resolved_in("lib/shop.ex");
  for (line, name, target, path, at) in shop_rows() {
    assert!(
      rows.contains(&(
        line,
        name.to_string(),
        target.to_string(),
        path.map(str::to_string),
        at
      )),
      "{:?} missing from {rows:#?}",
      (line, name, target, path, at)
    );
  }
  assert!(
    !rows
      .iter()
      .any(|(_, _, target, _, _)| target == "String.Chars.to_string/1"),
    "`to_string` expands to a call to `String.Chars.to_string/1` on the written line under the \
     written name, and the expansion carries no column, so it is not stored: {rows:#?}"
  );
}

#[test]
fn the_checked_in_trace_resolves_the_projects_references_to_what_the_compiler_named() {
  let (fx, mut facade) = shop();
  let outcome = resolve(&mut facade, Some("elixir"), false, Box::new(CheckedIn));
  let run = &outcome.resolution["elixir"];
  assert_eq!(run.state, "current", "{run:?}");

  let recorded = TRACE
    .lines()
    .filter(|l| matches!(l.split('\t').next(), Some("C" | "M" | "Q")))
    .count() as u64;
  let t = &run.tally;
  assert_eq!(
    t.matched + t.unmatched + t.dropped,
    recorded,
    "THE CONSERVATION LAW, on real compiler output: every reference event the tracer recorded \
     is counted once: {run:?}"
  );
  assert_shop_resolved(&fx);

  assert!(
    TRACE
      .lines()
      .any(|l| l == "M\t{ROOT}/lib/shop.ex\t3\t\tShop.Schema"),
    "the capture holds `use`'s generated reference to the module, with no column"
  );
  assert!(
    t.dropped_by
      .get(elixir_tracer::EXPANSION)
      .is_some_and(|n| *n > 0),
    "and it drops as expansion, while the written `use Shop.Schema` still resolves (vc decision \
     27): {run:?}"
  );
  assert_eq!(
    (
      t.dropped_by.get(OPERATOR),
      t.dropped_by.get(elixir_tracer::QUOTED)
    ),
    (Some(&1), Some(&1)),
    "the written `+` is an operator, and the `def` inside `__using__`'s quote names no one \
     target: {run:?}"
  );
}

#[test]
fn a_missing_elixir_is_named_and_never_answers_as_an_empty_tier() {
  let (_fx, mut facade) = shop();
  let absent = ElixirTracer {
    program: "elixir-that-is-not-installed".into(),
  };
  let outcome = resolve(&mut facade, Some("elixir"), false, Box::new(absent));
  let run = &outcome.resolution["elixir"];
  assert_eq!(run.state, "missing", "{run:?}");
  assert!(
    run
      .detail
      .as_deref()
      .is_some_and(|d| d.contains("elixir-that-is-not-installed")),
    "the record names the program it could not run: {run:?}"
  );
}

#[test]
fn a_project_with_no_mix_exs_at_its_root_is_not_applicable_even_with_one_below_it() {
  let fx = Fixture::new();
  git_init_at(fx.root());
  fx.write_file(".gitignore", "intent/.cache/\n");
  declare_elixir(&fx);
  fx.write_file("apps/shop/mix.exs", MIX_EXS);
  fx.write_file("apps/shop/lib/shop.ex", SHOP);
  let mut facade = fx.facade_on_disk();
  facade.index_rebuild().expect("rebuild");
  let never_run = ElixirTracer {
    program: "elixir-that-must-not-run".into(),
  };
  let outcome = resolve(&mut facade, None, false, Box::new(never_run));
  assert_eq!(
    outcome.not_applicable.get("elixir").map(String::as_str),
    Some("no `mix.exs` at the project root"),
    "a Mix project is read at the project root only, and the compiler is never run for one \
     below it (vc, 2026-09-17): {outcome:?}"
  );
}

/// A stand-in for `elixir`: it appends the `MIX_ENV` and `MIX_BUILD_PATH` it
/// was given and how it was run to `calls`, then acts on
/// the first flag file present in its directory. `fail` prints the error Mix
/// prints for an undefined function and exits 1; `unwritten` prints the line
/// the tracer prints when it cannot write, copies `trace` and exits 0; `empty`
/// writes an empty trace and exits 0. With no flag it copies `trace` to where
/// the tracer writes and exits 0.
fn stand_in(bin: &std::path::Path) -> std::path::PathBuf {
  use std::os::unix::fs::PermissionsExt;
  let program = bin.join("elixir");
  let dir = bin.display();
  std::fs::write(
    &program,
    format!(
      "#!/bin/sh\n\
       printf '%s %s %s\\n' \"$MIX_ENV\" \"$MIX_BUILD_PATH\" \"$*\" >> '{dir}/calls'\n\
       if [ -f '{dir}/fail' ]; then\n\
       printf '    error: undefined function nope/0\\n    lib/shop.ex:7:5: Shop.fetch/1\\n\\n== Compilation error in file lib/shop.ex ==\\n' >&2\n\
       exit 1\n\
       fi\n\
       if [ -f '{dir}/unwritten' ]; then\n\
       printf '{unwritten} no space left on device; the fault in lib/shop.ex was: boom\\n' >&2\n\
       fi\n\
       if [ -f '{dir}/empty' ]; then\n\
       : > \"$INTENT_TRACE\"\n\
       exit 0\n\
       fi\n\
       cp '{dir}/trace' \"$INTENT_TRACE\"\n",
      unwritten = elixir_tracer::UNWRITTEN
    ),
  )
  .expect("write the stand-in");
  std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("chmod");
  program
}

#[test]
fn the_compile_builds_in_intents_own_directory_and_is_forced_only_when_it_must_be() {
  let (fx, mut facade) = shop();
  let env = elixir_tracer::ENV;
  fx.write_file(
    &format!("_build/{env}/lib/shop/marker"),
    "the user's build\n",
  );
  fx.write_file("mix.lock", "%{}\n");
  let bin = tempfile::tempdir().expect("tempdir");
  std::fs::write(
    bin.path().join("trace"),
    trace_at(&fx.root().to_string_lossy()),
  )
  .expect("trace");
  let reader = || -> Box<dyn Resolver> {
    Box::new(ElixirTracer {
      program: stand_in(bin.path()).into(),
    })
  };
  let calls = || {
    std::fs::read_to_string(bin.path().join("calls"))
      .expect("calls")
      .lines()
      .map(str::to_string)
      .collect::<Vec<_>>()
  };
  let build = fx
    .root()
    .join(format!("intent/.cache/resolve/elixir/{env}"));

  let first = resolve(&mut facade, Some("elixir"), false, reader());
  assert_eq!(first.resolution["elixir"].state, "current", "{first:?}");
  assert_shop_resolved(&fx);
  assert!(
    build.join("lib/shop/marker").is_file(),
    "the first run seeds Intent's build from the project's own"
  );
  assert!(
    !fx
      .root()
      .join(format!("_build/{env}/lib/shop/elixir_tracer.exs"))
      .exists()
      && std::fs::read_dir(fx.root().join(format!("_build/{env}/lib/shop")))
        .expect("the user's build")
        .count()
        == 1,
    "and the project's `_build` is only ever read"
  );

  resolve(&mut facade, Some("elixir"), false, reader());
  resolve(&mut facade, Some("elixir"), true, reader());
  fx.write_file("mix.lock", "%{changed: true}\n");
  resolve(&mut facade, Some("elixir"), false, reader());

  let calls = calls();
  assert_eq!(calls.len(), 4, "{calls:#?}");
  let forced: Vec<bool> = calls.iter().map(|c| c.contains("--force")).collect();
  assert_eq!(
    forced,
    vec![true, false, true, true],
    "the first run forces, a run over an unchanged build is incremental, `--full` forces, and a \
     run that makes the build again because `mix.lock` moved forces: {calls:#?}"
  );
  assert!(
    calls.iter().all(|c| c.starts_with(&format!(
      "{env} {} -r {}",
      build.display(),
      fx.root()
        .join("intent/.cache/resolve/elixir/elixir_tracer.exs")
        .display()
    ))),
    "every compile builds `dev` under Intent's own directory with the build's own tracer, \
     whatever `MIX_ENV` the run inherited: {calls:#?}"
  );

  std::fs::write(bin.path().join("fail"), "").expect("fail");
  let failed = resolve(&mut facade, Some("elixir"), false, reader());
  let run = &failed.resolution["elixir"];
  assert_eq!(
    (run.state.as_str(), run.path.as_deref(), run.line),
    ("failed", Some("lib/shop.ex"), Some(7)),
    "a compile that fails is recorded with the file and line Mix names: {run:?}"
  );
  assert!(
    run
      .detail
      .as_deref()
      .is_some_and(|d| d.contains("undefined function nope/0")),
    "{run:?}"
  );
  assert_shop_resolved(&fx);
}

#[test]
fn a_fault_the_tracer_could_not_record_or_a_forced_compile_that_traced_nothing_fails_the_run() {
  let (fx, mut facade) = shop();
  let bin = tempfile::tempdir().expect("tempdir");
  std::fs::write(
    bin.path().join("trace"),
    trace_at(&fx.root().to_string_lossy()),
  )
  .expect("trace");
  let reader = || -> Box<dyn Resolver> {
    Box::new(ElixirTracer {
      program: stand_in(bin.path()).into(),
    })
  };

  std::fs::write(bin.path().join("unwritten"), "").expect("flag");
  let run = resolve(&mut facade, Some("elixir"), false, reader()).resolution["elixir"].clone();
  assert_eq!(run.state, "failed", "{run:?}");
  assert!(
    run
      .detail
      .as_deref()
      .is_some_and(|d| d.starts_with(elixir_tracer::UNWRITTEN) && d.contains("boom")),
    "a compile that succeeded while the tracer could not write its trace is failed by the line \
     the tracer printed, never stored from a trace with records missing: {run:?}"
  );
  assert!(
    fx.resolved_in("lib/shop.ex").is_empty(),
    "and it stores nothing"
  );

  std::fs::remove_file(bin.path().join("unwritten")).expect("flag");
  std::fs::write(bin.path().join("empty"), "").expect("flag");
  let run = resolve(&mut facade, Some("elixir"), true, reader()).resolution["elixir"].clone();
  assert_eq!(
    (run.state.as_str(), run.detail.as_deref()),
    (
      "failed",
      Some(
        "a forced compile of a project holding `.ex` files traced nothing: Mix never called the \
         tracer"
      )
    ),
    "a forced compile starts every file it compiles, so an empty trace means the tracer was never \
     called, and a current run resolving nothing would hide that: {run:?}"
  );
}

/// **RUN BY `bin/devbin test all`, NEVER IN CI** (hv, 2026-09-17): the real
/// compiler over the tiny project, end to end. It fails by name when the tool
/// is absent rather than skipping.
mod needs_the_toolchain {
  use super::*;

  #[test]
  #[ignore = "needs elixir on PATH; bin/devbin test all runs it"]
  fn the_elixir_compiler_resolves_the_tiny_project_end_to_end() {
    let (fx, mut facade) = shop();
    let outcome = resolve(
      &mut facade,
      Some("elixir"),
      false,
      Box::new(ElixirTracer::default()),
    );
    let run = &outcome.resolution["elixir"];
    assert_eq!(
      run.state, "current",
      "this arm needs elixir on PATH, and the run did not store: {run:?}"
    );
    assert_shop_resolved(&fx);
    assert!(
      !fx.root().join("_build").exists(),
      "the compile never writes the project's `_build`"
    );

    let again = resolve(
      &mut facade,
      Some("elixir"),
      false,
      Box::new(ElixirTracer::default()),
    );
    assert_eq!(again.resolution["elixir"].state, "current", "{again:?}");
    assert_shop_resolved(&fx);

    fx.write_file(
      "lib/shop.ex",
      &SHOP.replace("to_string(local(total))", "nope()"),
    );
    facade.index_refresh(None).expect("reconcile");
    let broken = resolve(
      &mut facade,
      Some("elixir"),
      false,
      Box::new(ElixirTracer::default()),
    );
    let run = &broken.resolution["elixir"];
    assert_eq!(
      (run.state.as_str(), run.path.as_deref(), run.line),
      ("failed", Some("lib/shop.ex"), Some(10)),
      "a compile that fails names the file and line, and stores nothing new: {run:?}"
    );
    assert_eq!(
      facade.index_status().expect("status").resolution["elixir"].stale,
      vec!["lib/shop.ex".to_string()],
      "the rows the last good run stored stay, marked stale by the file's hash"
    );
  }

  /// **THE TRACER NEVER RAISES**, driven in the real runtime rather than read:
  /// a recorded event it cannot print and a recorded kind in a shape it does
  /// not know are written as fault records, a fault it cannot write is printed
  /// under the prefix the reader looks for, and every call returns `:ok`, so a
  /// fault fails the run and never aborts the project's compile.
  #[test]
  #[ignore = "needs elixir on PATH; bin/devbin test all runs it"]
  fn the_tracer_turns_every_fault_into_a_record_or_a_prefixed_line_and_returns_ok() {
    let dir = tempfile::tempdir().expect("tempdir");
    let (source, log) = (
      dir.path().join("tracer.exs"),
      dir.path().join("fault.trace"),
    );
    std::fs::write(&source, elixir_tracer::TRACER).expect("tracer");
    let module = elixir_tracer::TRACER_MODULE;
    let script = format!(
      ":ok = {module}.trace({{:local_function, [line: 1, column: 1], \"not an atom\", 1}}, __ENV__)
       :ok = {module}.trace({{:alias, [line: 1], Foo}}, __ENV__)
       :persistent_term.erase({module})
       :ok = {module}.trace(:start, __ENV__)"
    );
    let out = std::process::Command::new("elixir")
      .arg("-r")
      .arg(&source)
      .args(["-e", &script])
      .env("INTENT_TRACE", &log)
      .output()
      .expect("this arm needs elixir on PATH");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "every call returned :ok: {stderr}");
    assert!(
      stderr
        .lines()
        .any(|l| l.starts_with(elixir_tracer::UNWRITTEN)),
      "the fault it could not write is printed under the prefix: {stderr}"
    );
    let text = std::fs::read_to_string(&log).expect("the trace");
    let faults: Vec<&str> = text.lines().filter(|l| l.starts_with("F\u{1f}")).collect();
    assert_eq!(faults.len(), 2, "{text:?}");
    assert!(
      faults[0].contains("not an atom") && faults[1].contains("in a shape it does not know"),
      "{faults:#?}"
    );
    assert!(
      matches!(
        elixir_tracer::trace_of(&text, &[], &BTreeMap::new(), true),
        Err(Unresolved::Failed { .. })
      ),
      "and a trace holding a fault record fails the run"
    );
  }
}
