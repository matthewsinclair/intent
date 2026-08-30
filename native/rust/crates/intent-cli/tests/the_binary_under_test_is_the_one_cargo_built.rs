//! **A TEST THAT SPAWNS `intent` MUST SPAWN THE ONE CARGO BUILT FOR IT.**
//!
//! `env!("CARGO_BIN_EXE_intent")` is a compile-time constant cargo points at
//! the binary produced for THIS invocation, so it follows `CARGO_TARGET_DIR`
//! wherever it goes. `workspace_root().join("target/debug/intent")` is a fixed
//! path that ignores it. **The two spellings are indistinguishable in a green
//! run and differ in exactly the case that matters.**
//!
//! **THE EPISODE (dc, 2026-08-30).** Four test files used the fixed path, 20
//! arms between them. A node building into `native/rust/target/cc` therefore
//! ran all 20 against `target/debug/intent` -- whatever the last default
//! -target-dir build happened to leave. Measured on the day: sha `57c173c3` at
//! 43265752 bytes, while cargo had just built `0a2ebb7a` at 43441384 bytes for
//! that same test run. **Nothing reported the substitution, because there was
//! no failure to report: the arms passed, against the wrong artefact.**
//!
//! **THE PRIVATE TARGET DIR BEING INSIDE `target/` IS WHAT MADE IT SILENT.**
//! dc found it from a worktree whose target dir was OUTSIDE the tree, where the
//! fixed path did not resolve at all and all 7 arms of the first file died on
//! `NotFound` -- loud, immediate, and initially misattributed to their own
//! change. **The louder failure was the lucky one**; the quiet form is what
//! this guard exists to stop coming back.
//!
//! **WHY THIS IS NOT AN ARM OF `table_driven_tests_fixture_their_home.rs`.**
//! That file's `spawns_the_binary` accepts BOTH spellings, and for its own
//! question -- *does this test spawn the binary at all*, which gates whether it
//! must fixture `HOME` -- accepting both is correct. Narrowing it would blind
//! the HOME check to exactly these four files. **So the four did not drift
//! under a guard that failed; they drifted under no guard, beside one whose
//! NAME sounded like it covered them.** Putting binary resolution inside a file
//! called `..._fixture_their_home` would make a home whose name does not
//! describe its contents, which is the defect this estate keeps re-finding.
//!
//! **THE LIMIT, STATED RATHER THAN DISCOVERED LATER: this proves the SPELLING,
//! not that a spawn reached the intended inode.** A test that spells
//! `CARGO_BIN_EXE_intent` and then execs a sibling it resolved some other way
//! is outside what any source scan can see. `common/mod.rs`'s `intentd` panic
//! message is the standing reminder that the sibling problem is real and lives
//! one layer down.

use std::path::{Path, PathBuf};

use testkit::workspace_root;

/// A hardcoded build directory. Both profiles, because a test pinned to
/// `release` is the same defect with a slower failure.
const BUILD_DIRS: &[&str] = &["target/debug/", "target/release/"];

/// Turning a string into a path is what makes a mention a RESOLUTION.
///
/// **THE PREDICATE IS LINE-SCOPED AND THAT IS THE WHOLE PRECISION.** Two live
/// files legitimately name `target/debug/intentd` in prose -- a panic message
/// in `common/mod.rs` explaining the cross-package build trap, and a header in
/// `egest_refuses_to_empty_the_estate.rs` recounting a shared-binary episode.
/// **A file-scoped `contains` would red both**, and a guard that reds the
/// documentation of the hazard it guards against is one nobody keeps.
const CONSTRUCTORS: &[&str] = &[".join(", "Path::new(", "PathBuf::from(", "::from("];

#[test]
fn no_test_resolves_the_binary_by_a_hardcoded_path() {
  let root = workspace_root();
  let mut offenders: Vec<String> = Vec::new();

  for path in test_sources(&root) {
    if is_this_file(&path) {
      continue;
    }
    for (n, line) in code_of(&path).lines().enumerate() {
      if resolves_by_hand(line) {
        offenders.push(format!("{}:{}", shown(&root, &path), n + 1));
      }
    }
  }

  assert!(
    offenders.is_empty(),
    "these tests resolve a binary by a hardcoded path under `target/`, so they \
     spawn whatever the last default-target-dir build left rather than the \
     binary cargo built for this run -- and they will PASS or FAIL against it \
     without saying so. Use `env!(\"CARGO_BIN_EXE_<name>\")`:\n  {}",
    offenders.join("\n  ")
  );
}

/// **THE POPULATION IS ASSERTED BEFORE THE PROPERTY.** A scan whose corpus is
/// empty passes for free and reports a clean estate -- the failure this repo
/// has now met from four directions. If the walk breaks, or `tests/` moves,
/// the arm above goes green on nothing and says the same word it says when the
/// estate is genuinely clean.
#[test]
fn the_census_sees_the_tests_that_actually_spawn_the_binary() {
  let root = workspace_root();
  let spawners: Vec<String> = test_sources(&root)
    .into_iter()
    .filter(|path| !is_this_file(path))
    .filter(|path| code_of(path).contains("CARGO_BIN_EXE_"))
    .map(|path| shown(&root, &path))
    .collect();

  assert!(
    spawners.len() > 20,
    "only {} test file(s) resolve a binary through cargo, which is too few for \
     this estate -- the walk is not reaching the tests, so the guard beside \
     this one is passing vacuously",
    spawners.len()
  );
}

/// **DRIVEN TO BOTH VERDICTS, BECAUSE A PREDICATE THAT NEVER SAYS NO IS NOT A
/// PREDICATE.** The forbidden form and the two shapes that must stay legal are
/// all planted here, so the census above cannot pass merely by being unable to
/// recognise anything.
#[test]
fn the_predicate_separates_a_resolution_from_a_mention() {
  let offends = r#"  workspace_root().join("target/debug/intent")"#;
  assert!(
    resolves_by_hand(offends),
    "the predicate does not recognise the exact line this guard was written \
     for, so the census cannot fail on the defect it exists to catch"
  );

  let prose = r#"//! a shared `target/release/` binary reported zero threads"#;
  assert!(
    !resolves_by_hand(prose),
    "a doc comment naming a build directory is not a resolution"
  );

  let message = r#"panic!("the usual cause is that `target/debug/intentd` is stale")"#;
  assert!(
    !resolves_by_hand(message),
    "a panic message explaining the hazard is not an instance of it"
  );

  let correct = r#"  std::path::PathBuf::from(env!("CARGO_BIN_EXE_intent"))"#;
  assert!(
    !resolves_by_hand(correct),
    "the idiom this guard is steering everyone towards must not be an offence"
  );
}

/// A build directory named on the same line as a path constructor.
fn resolves_by_hand(line: &str) -> bool {
  BUILD_DIRS.iter().any(|d| line.contains(d)) && CONSTRUCTORS.iter().any(|c| line.contains(c))
}

/// **THE SCANNER EXCLUDES ITSELF, DERIVED FROM `file!()` AND NEVER FROM A
/// LITERAL NAME.** Every marker it looks for is spelled out above as a string,
/// so on its first run it would report itself as the estate's only offender --
/// the lesson `table_driven_tests_fixture_their_home.rs` records learning
/// twice. A literal filename here would silently stop excluding on a rename.
fn is_this_file(path: &Path) -> bool {
  path.ends_with(
    Path::new(file!())
      .file_name()
      .expect("this file has a name"),
  )
}

fn shown(root: &Path, path: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .display()
    .to_string()
}

/// Every `tests/**/*.rs` in every crate, discovered by walking rather than
/// listed: the act that invalidates a hand-kept roster (adding a test) is not
/// the act that updates it.
fn test_sources(root: &Path) -> Vec<PathBuf> {
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
    let tests = entry.path().join("tests");
    if tests.is_dir() {
      walk(&tests, &mut out);
    }
  }
  out
}

fn code_of(path: &Path) -> String {
  std::fs::read_to_string(path).unwrap_or_default()
}
