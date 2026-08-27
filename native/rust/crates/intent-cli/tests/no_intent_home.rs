//! AT-11.3 / AC-11.3: **the shipped binary does not depend on the developer's
//! environment.**
//!
//! `INTENT_HOME` is the instance the criterion names, because it is v2's:
//! the shell tool reads it to find its rule library and its templates, so a v2
//! `intent` on a machine without it is broken. WP-11 is the row that turns
//! `brew install` into the install story, and a brew-installed binary meets a
//! machine with **no clone, no checkout and no `INTENT_HOME`**. If v3 had
//! inherited that dependency, every developer machine here would still be
//! green, because every developer machine here has it set.
//!
//! **The estate had already decided this three times, in comments, and never
//! once in a control.** `render.rs:49` -- "the file's presence, never an
//! environment variable (issue 0025)". `project.rs:239` -- "The marker is the
//! config file's own presence -- never an environment". `views.rs:6` -- "no
//! locale, no absolute paths, no environment". Three statements of the rule,
//! zero mechanisms that refuse it. A comment reminds; only a control refuses,
//! and this file is the control the comments have been standing in for.
//!
//! **The structural test is an ALLOWLIST, not a ban on `INTENT_HOME`, and that
//! is the point rather than scope creep.** A needle list forbids only what its
//! author thought of, so it is defeated by the next variable somebody invents
//! -- and the criterion is not "this one name is absent", it is that the
//! binary works with no developer environment at all. Asserting that the set
//! of variables the shipped source reads is EXACTLY `{COLUMNS}` catches any
//! new environment dependency whatever it is called, including one added by a
//! command that does not exist yet.
//!
//! **Which matters because the risky commands are the unwritten ones.** As of
//! this file, `init`, `bootstrap`, `export`, `ingest`, `backup` and `mcp` all
//! answer "not implemented yet" -- and those are precisely the ones that will
//! want to resolve a home when they land. A behavioural test can only drive
//! what exists; the structural test covers the code the day it is written.
//!
//! Out of scope, deliberately. `env!("CARGO_PKG_VERSION")` is resolved by the
//! compiler and baked into the binary, so it is not a runtime environment read
//! at all. `env::args()` is argv and `env::current_dir()` is the working
//! directory: both are how the user invoked the tool, not how their machine is
//! configured, and a tool that ignored them would be broken.
//!
//! **HIGHLANDER, named rather than quietly duplicated:** `sources()` and
//! `code_of()` below are the same walk as `intentsvcs/tests/one_clock.rs`.
//! Rust integration tests cannot share a helper across crates without a
//! dev-dependency crate to hold it, and AT-11.3 names this path, so the
//! duplication is structural rather than chosen. Raised with cc, whose crate
//! `one_clock.rs` sits in, as a `test-support` crate question -- not
//! unilaterally restructured, because a shared walker is their call.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use testkit::workspace_root;

/// The only environment variable the shipped surface may read.
///
/// `COLUMNS` is terminal geometry: it describes the device the output is going
/// to, not the machine's configuration, and it is absent-tolerant by
/// construction (`render.rs` falls back when it cannot be parsed).
///
/// **`USER` WAS GRANTED BY hv ON 2026-08-27, FOR `bootstrap` AND CONFINED THE
/// SAME WAY `HOME` IS.** It is the operator's identity, and `bootstrap` is the
/// one verb whose job is recording who this machine belongs to -- v2 has
/// written `"author": "${USER}"` into the global config since 2.0.0
/// (`bin/intent_bootstrap:126`), so this grant restores a behaviour rather
/// than inventing one.
///
/// **THE ALTERNATIVE WAS MEASURED BEFORE THE RULING RATHER THAN ARGUED AFTER
/// IT.** `git config --get user.name` needs no grant at all -- the surface
/// already shells out to `git` in four places -- and it was rejected on the
/// VALUE, not the mechanism: it answers `Matthew Sinclair` where every
/// existing project config in this estate, and the root `CLAUDE.md` author
/// line, say `matts`. A grant-free path that writes a different identity than
/// the fleet already carries is not the cheaper option, it is a silent
/// divergence with a tidy implementation.
///
/// **AND THE ABSENT CASE IS NOT A FAILURE.** Unlike `HOME`, whose absence
/// means per-user state cannot exist at all, an unset `USER` just means the
/// author is unknown -- `bootstrap` records the config without it and says so.
const ALLOWED: &[&str] = &["COLUMNS", "HOME", "USER"];

/// A variable that may be read, but in exactly ONE file.
///
/// **`HOME` WAS GRANTED BY hv ON 2026-08-22 SO THAT PER-USER STATE COULD BE
/// REACHED AT ALL** -- `intent claude skills` manages `~/.claude/skills/` and
/// `~/.intent/`, and could not be built while the surface read only `COLUMNS`.
/// The ask was routed through vc, the invariant was driven in both directions
/// first (green at HEAD; a planted read refused by name), and the row is here
/// rather than being a quiet addition, which is what this test's own failure
/// message demands.
///
/// **BUT THE GRANT THAT WAS ASKED FOR WAS NARROWER THAN A ROW IN `ALLOWED`.**
/// The question was whether per-user state may be reached, not whether any
/// file may consult the environment -- and an entry in `ALLOWED` alone would
/// have answered the second. Confining it keeps the audit surface one file
/// wide and keeps a second reader failing exactly the way an unapproved
/// variable does.
///
/// A new entry here needs the same thing the `ALLOWED` row needed: a ruling,
/// and a reason written down beside it.
const CONFINED: &[(&str, &str)] = &[
  ("HOME", "crates/intentsvcs/src/userstate.rs"),
  ("USER", "crates/intentsvcs/src/userstate.rs"),
];

/// Every `.rs` under every crate's `src/`, discovered by walking.
///
/// **`src/` only, unlike `one_clock.rs`, and the difference is the rule rather
/// than an oversight.** A clock in a fixture is a defect because the confected
/// value becomes the thing the assertion trusts. A test reading an environment
/// variable is not: tests do not ship, and `corpus_machine_independence.rs`
/// reads and sets `HOME` and `GIT_CONFIG_GLOBAL` deliberately, to build the
/// controlled environment that makes it honest. This file does the same below.
/// Scanning `tests/` would forbid the technique that proves the property.
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

/// Strip line comments, so this file's own prose and the three source comments
/// that state the rule in English cannot trip the scan that enforces it.
fn code_of(path: &Path) -> String {
  std::fs::read_to_string(path)
    .unwrap_or_default()
    .lines()
    .filter(|l| !l.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n")
}

/// What a single `env::var*` call site reads.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Read {
  /// `env::var("NAME")` / `env::var_os("NAME")` -- a literal name.
  Named(String),
  /// `env::var(some_expression)` -- the name is computed, so no allowlist can
  /// see it. Treated as a violation on its own: a read nobody can enumerate is
  /// worse than one that is merely disallowed.
  Dynamic,
  /// `env::vars()` / `env::vars_os()` -- the whole environment at once, which
  /// defeats a name-based allowlist entirely.
  Wholesale,
}

/// `use` statements that import a FUNCTION out of `std::env`, which is the one
/// way to read the environment while leaving no `env::` at the call site.
///
/// **FOUND BY vc CANARYING THIS FILE, 2026-08-15, and the mechanism is sharper
/// than a missing needle.** `use std::env::var as read_env;` DOES contain
/// `env::var`, so `env_reads` finds it, looks at what follows, sees ` as
/// read_env;` rather than `(`, and correctly concludes it is not a call. That
/// judgement is right -- and the aliasing happened on that very line, while the
/// real call site (`read_env("INTENT_HOME")`) carries no trace of `env` at all.
/// **The one line that reveals the aliasing is exactly the line the call
/// detector is designed to ignore.** Demonstrated with a passing test over a
/// shipped `src/` file that read `INTENT_HOME`.
///
/// **It is this file's own argument one level down.** The allowlist was chosen
/// because a needle list forbids only what its author thought of -- and it was
/// **name-complete and syntax-incomplete**: exhaustive over every variable NAME
/// while resting on a needle for the CALL SYNTAX. This closes the second axis.
///
/// **Importing the MODULE is fine and is deliberately still allowed.** `use
/// std::env;` leaves every call site reading `env::var("NAME")`, which the
/// scanner sees. Only importing the function moves the read somewhere the scan
/// cannot follow, so the rule is: **name the module at the call site.** Cheap,
/// because the shipped code already does exactly that.
fn env_imports(code: &str) -> Vec<String> {
  code
    .lines()
    .map(str::trim)
    .filter(|l| l.starts_with("use ") || l.starts_with("pub use "))
    // `env::var` covers `var`, `var_os` and any `as` alias of either; `env::{`
    // covers the brace group, which does not contain `env::var` as a substring
    // at all and would otherwise be the next way through.
    .filter(|l| l.contains("env::var") || l.contains("env::{"))
    .map(str::to_string)
    .collect()
}

/// Every runtime environment read in one file's code.
fn env_reads(code: &str) -> Vec<Read> {
  const NEEDLE: &str = "env::var";
  let mut out = Vec::new();
  let mut from = 0;

  while let Some(hit) = code[from..].find(NEEDLE) {
    let at = from + hit;
    let rest = &code[at + NEEDLE.len()..];
    from = at + NEEDLE.len();

    // `env::vars(` / `env::vars_os(` -- `env::var` is a prefix of `env::vars`,
    // so this arm has to come first or every wholesale read reads as a named
    // one with a broken argument.
    if rest.starts_with("s(") || rest.starts_with("s_os(") {
      out.push(Read::Wholesale);
      continue;
    }

    let open = if rest.starts_with("_os(") {
      at + NEEDLE.len() + 4
    } else if rest.starts_with('(') {
      at + NEEDLE.len() + 1
    } else {
      // Something else beginning `env::var`, eg an identifier. Not a call.
      continue;
    };

    let arg = code[open..].trim_start();
    if let Some(body) = arg.strip_prefix('"') {
      match body.find('"') {
        Some(end) => out.push(Read::Named(body[..end].to_string())),
        None => out.push(Read::Dynamic),
      }
    } else {
      out.push(Read::Dynamic);
    }
  }
  out
}

#[test]
fn the_shipped_surface_reads_exactly_one_environment_variable() {
  let root = workspace_root();
  let files = sources(&root);

  // The fixture proves itself. A walk that found nothing would agree with
  // every allowlist ever written, and it would agree silently.
  assert!(
    files.len() > 10,
    "the source walk found only {} files under {}/crates/*/src -- the walk is broken, and a broken walk passes this test vacuously",
    files.len(),
    root.display()
  );

  let mut named = BTreeSet::new();
  let mut offenders = Vec::new();

  for file in &files {
    let shown = file
      .strip_prefix(&root)
      .unwrap_or(file)
      .display()
      .to_string();
    let code = code_of(file);
    for import in env_imports(&code) {
      offenders.push(format!(
        "{shown}: `{import}` imports an environment reader, so its call sites carry no `env::` for this scan to find -- write `std::env::var(\"NAME\")` in full instead"
      ));
    }
    for read in env_reads(&code) {
      match read {
        Read::Named(name) => {
          if !ALLOWED.contains(&name.as_str()) {
            offenders.push(format!("{shown}: reads ${name}"));
          }
          if let Some((_, sole)) = CONFINED.iter().find(|(n, _)| *n == name)
            && shown.replace('\\', "/") != *sole
          {
            offenders.push(format!(
              "{shown}: reads ${name}, which is allowed only in {sole} -- route it through that module rather than reading it here"
            ));
          }
          named.insert(name);
        }
        Read::Dynamic => offenders.push(format!(
          "{shown}: reads an environment variable whose name is computed"
        )),
        Read::Wholesale => {
          offenders.push(format!("{shown}: reads the whole environment (env::vars)"))
        }
      }
    }
  }

  assert!(
    offenders.is_empty(),
    "the shipped surface reads an environment variable it is not allowed to:\n  {}\n\nAC-11.3: a brew-installed binary meets a machine with no clone and no \
     developer environment. If this read is genuinely required, it needs an hv ruling and a row in ALLOWED, not a quiet addition -- every machine here has \
     the variable set, so nothing else will fail.",
    offenders.join("\n  ")
  );

  // Second half of proving the fixture: the scanner must actually FIND the one
  // read we know is there. Without this, a scanner broken in a way that
  // returns nothing passes the assertion above by finding no offenders.
  assert!(
    named.contains("COLUMNS"),
    "the scanner found no read of $COLUMNS, which render.rs demonstrably does -- so the scanner is broken and its clean result above means nothing. Found: {named:?}"
  );
}

/// The three environments a machine can be in, as far as `INTENT_HOME` goes.
const CASES: &[(&str, Option<&str>)] = &[
  ("absent", None),
  ("garbage", Some("/nonexistent/intent/home")),
  ("plausible", Some("/usr")),
];

/// Commands that reach Intent's own logic rather than stopping at clap.
///
/// **This list is REPRESENTATIVE and the totality comes from the structural
/// test above; saying so is the point.** Measured against the binary: most
/// bare invocations are refused by clap for a missing subcommand, which
/// exercises no Intent code and would pad this list with rows that prove
/// nothing. These three do reach it -- `doctor` and `sync` run project
/// resolution (the walk up from the working directory that a home variable
/// would plausibly influence), and `schema` renders the embedded assets, which
/// is the rust-embed path a "read templates from disk" override would touch.
const REACHES_INTENT: &[&[&str]] = &[&["doctor"], &["sync"], &["schema"], &["--version"]];

/// Build the command for one case. **One recipe, used by both the oracle and
/// the real runs**, so what the oracle proves is what the runs did.
fn command(program: &str, args: &[&str], home: &Path, cwd: &Path, case: Option<&str>) -> Command {
  let mut cmd = Command::new(program);
  cmd.args(args).current_dir(cwd).env("HOME", home);
  match case {
    Some(value) => cmd.env("INTENT_HOME", value),
    None => cmd.env_remove("INTENT_HOME"),
  };
  cmd
}

#[test]
fn intent_home_changes_nothing_a_user_can_see() {
  let home = tempfile::tempdir().expect("temp HOME");
  let cwd = tempfile::tempdir().expect("temp cwd");

  // The fixture proves itself, via a shell as the oracle. Setting the variable
  // is the whole experiment, so a differential run with the plumbing silently
  // not taking would compare three identical environments and pass -- the
  // shape that already fooled this node once today, when every row of a probe
  // failed identically and read as a consistent result.
  for (label, value) in CASES {
    let out = command(
      "/bin/sh",
      &["-c", "printf %s \"${INTENT_HOME-<unset>}\""],
      home.path(),
      cwd.path(),
      *value,
    )
    .output()
    .expect("run the oracle shell");
    let seen = String::from_utf8_lossy(&out.stdout).to_string();
    let want = value.unwrap_or("<unset>");
    assert_eq!(
      seen, want,
      "the {label} case did not reach the child process: expected INTENT_HOME to be {want:?}, the child saw {seen:?}. Every comparison below would have been \
       three copies of one environment."
    );
  }

  // One cwd and one HOME across all three cases: the working directory appears
  // in some of these messages, so three tempdirs would differ for a reason
  // that has nothing to do with the property.
  for args in REACHES_INTENT {
    let mut results = Vec::new();
    for (label, value) in CASES {
      let out = command(
        env!("CARGO_BIN_EXE_intent"),
        args,
        home.path(),
        cwd.path(),
        *value,
      )
      .output()
      .expect("run the v3 binary");
      results.push((
        label,
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
      ));
    }

    let (base_label, base_code, base_out, base_err) = &results[0];
    for (label, code, stdout, stderr) in &results[1..] {
      assert_eq!(
        (code, stdout, stderr),
        (base_code, base_out, base_err),
        "`intent {}` behaves differently with INTENT_HOME {label} than with it {base_label}, so its behaviour depends on the developer's environment (AC-11.3)",
        args.join(" ")
      );
    }
  }
}
