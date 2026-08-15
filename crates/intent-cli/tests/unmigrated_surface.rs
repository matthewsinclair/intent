//! AC-10.7, at the surface: NO shipped command answers a question about an
//! unmigrated project.
//!
//! The facade-level test (`intentsvcs/tests/unmigrated_project.rs`) proves the
//! gate refuses. It cannot prove that no command gets past the gate -- that is
//! a claim about the shape of the CLI, and the only honest way to make it is
//! to drive every command in the dispatch table and see.
//!
//! **Derived from the table, never hand-listed.** A hand-list certifies the
//! status quo: it is complete on the day it is written and silently incomplete
//! from the next verb onward, which is precisely how WP-06's long tail would
//! reintroduce this defect one command at a time. Reading the table means a
//! newly-wired verb is REQUIRED to be guarded the moment it is wired, and the
//! failure names it.
//!
//! **The property is the exit code, not the text**, and that is a correction
//! to this test's first draft. Classifying by what a command PRINTED meant
//! deciding whether "`ST0001` is not a work package" counted as guarded, and
//! every such judgement is a place to accidentally widen the pass condition
//! until the test agrees with whatever the code does. The defect being closed
//! was `intent st list` exiting **0** over an estate it could not see, so the
//! invariant is exactly that: on an unmigrated project, no command SUCCEEDS.
//! A refusal exits 1, an unwired verb exits 1, a usage error exits 1 -- only a
//! command that answered exits 0, and answering is the bug.
//!
//! The message-level counts below are reported rather than asserted, except
//! for one: at least one command must have been observed naming the migration.
//! Without that, a classification bug could make every command look fine by
//! making none of them run.

use std::path::Path;
use std::process::Command;

use intent_cli::dispatch;

/// Commands that legitimately never open a project. Each is exempt for a
/// stated reason -- an allow-list without reasons is a place to hide a bug.
fn reads_no_model(path: &str) -> Option<&'static str> {
  match path {
    // Prints the generated schema faces, which are compiled in via
    // `include_str!` and are the same bytes in every project.
    "schema" => Some("faces are compile-time constants; it never reads a project"),
    // Creates a project. Refusing before one exists would be a bootstrap
    // deadlock, and it is WP-06's remaining work in any case.
    "init" | "bootstrap" => Some("creates a project rather than reading one"),
    // The migrator itself, and the thing whose whole job is this state.
    "upgrade" => Some("it is the remedy; refusing it would leave no way out"),
    "doctor" => Some("reports the migration as a finding rather than refusing"),
    // Stdio and socket servers: wiring these later must not hang this test.
    "mcp" | "daemon" => Some("long-running servers -- excluded by construction"),
    "version" | "info" | "help" => Some("tool-level, not project-level"),
    _ => None,
  }
}

fn legacy_project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir .config");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"2.19.0\",\n  \"project_name\": \"Legacy\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"shell\"]\n}\n",
  )
  .expect("write config");
  let thread = dir.path().join("intent").join("st").join("ST0001");
  std::fs::create_dir_all(&thread).expect("mkdir thread");
  std::fs::write(
    thread.join("info.md"),
    "---\nstatus: In Progress\n---\n\n# ST0001: a real thread\n",
  )
  .expect("write v2 info.md");
  dir
}

/// Well-formed placeholder arguments, derived from each argument's declared
/// TYPE in the table.
///
/// By type rather than by position, because a malformed argument is rejected
/// before the project is ever opened -- so a lazy placeholder does not make
/// the test lenient, it makes it SHALLOW: the command never reaches the gate,
/// and the run proves nothing about it while looking like it did. The ids need
/// not exist; the gate sits at `Facade::open`, ahead of any lookup.
fn placeholder_args(entry: &dispatch::Entry) -> Vec<String> {
  let mut argv = Vec::new();
  for arg in &entry.args {
    // An optional slot with nothing to put in it -- a family header's verb.
    if arg.kind == "subcommand" && arg.values.is_empty() {
      continue;
    }
    if let Some(first) = arg.values.first() {
      argv.push(first.clone());
      continue;
    }
    argv.push(
      match arg.kind.as_str() {
        "st-id/NN" => "ST0001/01",
        "st-id" => "ST0001",
        "ac-id" => "AC-01.1",
        "at-id" => "AT-01.1",
        _ => "ST0001",
      }
      .to_string(),
    );
    // One value satisfies a variadic slot.
    if arg.arity.contains('n') || arg.arity.contains('+') {
      break;
    }
  }
  argv
}

fn run(root: &Path, argv: &[String]) -> (i32, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .output()
    .expect("run the v3 binary");
  let merged = format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  (out.status.code().unwrap_or(-1), merged)
}

#[test]
fn no_shipped_command_answers_from_an_unmigrated_project() {
  let dir = legacy_project();
  let table = dispatch::table();

  let mut answered: Vec<String> = Vec::new();
  let mut guarded = 0usize;
  let mut unwired = 0usize;
  let mut exempt = 0usize;
  let mut rejected = 0usize;

  for entry in dispatch::shipped_entries(&table) {
    let family = entry.path.split(' ').next().unwrap_or_default();
    if reads_no_model(&entry.path).is_some() || reads_no_model(family).is_some() {
      exempt += 1;
      continue;
    }

    let mut argv: Vec<String> = entry.path.split(' ').map(str::to_string).collect();
    argv.extend(placeholder_args(entry));
    let (code, output) = run(dir.path(), &argv);

    // THE assertion, collected rather than tripped one at a time so a failure
    // names every offender in one run instead of one per fix-and-rerun.
    if code == 0 {
      answered.push(format!(
        "`intent {}` SUCCEEDED (exit 0) over an estate it cannot see -- {}",
        argv.join(" "),
        if output.trim().is_empty() {
          "and printed nothing at all".to_string()
        } else {
          output
            .trim()
            .lines()
            .take(2)
            .collect::<Vec<_>>()
            .join(" / ")
        }
      ));
      continue;
    }
    if output.contains("has not been migrated") {
      guarded += 1;
    } else if output.contains("not yet wired to the facade") {
      unwired += 1;
    } else {
      rejected += 1;
    }
  }

  assert!(
    answered.is_empty(),
    "on an unmigrated project no command may succeed:\n{}",
    answered.join("\n")
  );
  assert!(
    guarded > 0,
    "no command was observed naming the migration, so this run proved nothing \
     -- suspect the sweep, not the code"
  );
  eprintln!("guarded {guarded}, unwired {unwired}, rejected-on-args {rejected}, exempt {exempt}");
}

/// The refusal goes to STDERR and the exit code is 1.
///
/// Worth pinning separately: the defect this closes was a command exiting 0
/// with nothing on either stream, so "it printed something somewhere" is not
/// the property. A script doing `intent st list > threads.txt` must get a
/// non-zero status and an empty file, not a file full of error text.
#[test]
fn the_refusal_is_a_failure_on_stderr_not_output_on_stdout() {
  let dir = legacy_project();
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "list"])
    .current_dir(dir.path())
    .output()
    .expect("run");

  assert_eq!(out.status.code(), Some(1), "INV-02: refusals exit 1");
  assert!(
    String::from_utf8_lossy(&out.stdout).is_empty(),
    "a refusal writes nothing to stdout"
  );
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(stderr.starts_with("error: "), "v2's voice: {stderr}");
  assert!(
    stderr.contains("ST0001"),
    "and it names the work it cannot see, which is what distinguishes this \
     from an empty project: {stderr}"
  );
}
