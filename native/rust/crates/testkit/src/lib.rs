//! Test-only helpers shared across the workspace's integration tests.
//!
//! # Why this crate exists
//!
//! Rust gives integration tests no way to share a helper across crates: each
//! file under `tests/` is its own binary, and `mod common;` cannot cross a crate
//! boundary. So the only options are a dev-dependency crate or copy-paste, and
//! the estate had chosen copy-paste **nine times**.
//!
//! # What the nine copies actually were
//!
//! Not one helper duplicated nine times. **Two different functions returning two
//! different directories, wearing names similar enough that nobody noticed**:
//!
//! | name               | copies | returns                                    |
//! | ------------------ | ------ | ------------------------------------------ |
//! | `repo_root()`      | 5      | the repository root, carrying `schema/`    |
//! | `workspace_root()` | 4      | `native/rust`, the cargo workspace root    |
//!
//! They differ by two directory levels. The duplication was found by grepping
//! `repo_root`, which is why it was reported as four copies -- **the other five
//! were invisible to the search that found the first four.** Two names for
//! adjacent concepts is worse than nine copies of one name, because it defeats
//! the only tool anyone was going to use to look.
//!
//! Naming them apart is therefore load-bearing rather than cosmetic. Reaching
//! for "the root" and getting the wrong one of these fails as a missing file two
//! levels away from where you are looking.
//!
//! # Searched, never counted
//!
//! Both functions SEARCH for a structural marker; neither counts levels.
//!
//! `ancestors().nth(2)` and `.parent().parent()` were correct until `a1a949c`
//! moved every native source to `native/rust/`, at which point everything that
//! counted its way to the repository root broke at once -- and **a counted path
//! that is wrong does not fail where it was written; it fails as a file-not-found
//! somewhere else entirely.** One copy still counted when this crate replaced
//! them (`dep_graph_guard.rs`), which is the whole argument for having one home:
//! the fix was applied to the copies someone remembered.
//!
//! Counting is not *always* wrong -- `crates/<name>` sitting two levels under a
//! workspace root is a cargo convention, not a project choice. It is refused here
//! anyway, because the reason to prefer searching is that **the failure mode of a
//! search is a loud panic at the point of use, and the failure mode of a count is
//! a plausible wrong directory.**

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The REPOSITORY root: the directory carrying `schema/` and `surface/`.
///
/// Two levels above [`workspace_root`]. Use this to reach committed canon --
/// the schema faces, the surface dispatch table, `intent/`.
///
/// Deliberately not located by `.git`: a git worktree has a `.git` FILE rather
/// than a directory, and this estate runs sacrificial worktrees routinely, so a
/// `.git`-is-a-dir test would fail in exactly the environment used to verify
/// destructive changes.
pub fn repo_root() -> PathBuf {
  ancestor_where(|d| d.join("schema").is_dir() && d.join("surface").is_dir())
    .expect("a repository root carrying schema/ and surface/ above this crate")
}

/// The CARGO WORKSPACE root: `native/rust`, the directory holding `crates/`.
///
/// Two levels below [`repo_root`]. Use this to walk sources -- `crates/*/src`,
/// `crates/*/tests` -- and to read the workspace manifest.
///
/// Located by the `[workspace]` table, which is the definition of a workspace
/// root rather than a proxy for it. A `crates/`-is-a-dir test would also work
/// today and would silently pick the wrong directory the moment any crate grew
/// a `crates/` subdirectory of its own.
pub fn workspace_root() -> PathBuf {
  ancestor_where(|d| {
    let manifest = d.join("Cargo.toml");
    manifest.is_file()
      && std::fs::read_to_string(&manifest)
        .map(|s| s.lines().any(|l| l.trim_start().starts_with("[workspace]")))
        .unwrap_or(false)
  })
  .expect("a Cargo.toml declaring [workspace] above this crate")
}

/// A `HOME` for any test that spawns the `intent` binary.
///
/// **A TEST BINARY INHERITS THE OPERATOR'S REAL `HOME`, AND SOME VERBS WRITE
/// THERE.** `intent bootstrap` publishes `~/.intent/home`, the machine-global
/// install pointer the pre-commit shim resolves on every commit. On 2026-08-27
/// two arms of `dispatch_ssot` published it to a scratch worktree that was later
/// deleted, and the estate spent an evening with a pointer naming a directory
/// that did not exist.
///
/// **NEITHER ARM WAS WRONG WHEN IT WAS WRITTEN.** Both drive every shipped
/// family bare, looking for the ones that answer *is a known command that is not
/// implemented yet*, and `bootstrap` answered exactly that until it was
/// implemented. **The subject changed underneath the test** -- so this is not a
/// helper for tests that touch per-user state, it is a helper for tests that
/// cannot know whether they do.
///
/// **UNDER `target/` RATHER THAN THE SYSTEM TEMP DIRECTORY**, on purpose: it is
/// already build output, `cargo clean` removes it, and it never accumulates in
/// `/tmp` where nothing prunes it. Per-process, so parallel test binaries do not
/// share one.
///
/// **std ONLY.** This crate declares no dependencies and `dep_graph_guard.rs`
/// enforces that, so no `tempfile` here.
pub fn fixture_home() -> &'static Path {
  static DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
  DIR
    .get_or_init(|| {
      let dir = workspace_root()
        .join("target/test-home")
        .join(std::process::id().to_string());
      std::fs::create_dir_all(&dir).expect("create the fixture HOME");
      dir
    })
    .as_path()
}

/// The nearest ancestor of this crate's manifest directory satisfying `pred`.
///
/// `CARGO_MANIFEST_DIR` is the compiling crate's directory, which cargo sets for
/// every test binary, so this resolves relative to the caller rather than to the
/// process's working directory -- tests that `cd` into a tempdir still find the
/// tree they were built from.
fn ancestor_where(pred: impl Fn(&Path) -> bool) -> Option<PathBuf> {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .find(|d| pred(d))
    .map(Path::to_path_buf)
}

/// Mutation-proven. Replacing `workspace_root`'s body with `repo_root()` --
/// collapsing the distinction this crate exists to keep -- fails
/// `the_two_roots_are_not_the_same_directory` and
/// `each_root_carries_what_its_callers_reach_for`.
///
/// **`the_workspace_root_sits_under_the_repo_root` PASSES under that mutation**,
/// and is kept knowing so: it catches the different failure of a root escaping
/// the repository entirely (a search that ran off the top and returned `/`),
/// which neither of the others would notice. Recorded rather than quietly
/// retained -- a test that survives the obvious mutation needs its own reason.
///
/// The first attempt at this mutation did not apply: it computed the wrong root
/// as a discarded statement and returned the right one, so all three tests
/// passed. **An unapplied mutation reports "nothing failed", which is
/// indistinguishable from a test that does not check.** The map above was taken
/// only after printing the mutated function body and seeing it change.
/// Run a JUST-COPIED binary, retrying while Linux reports it busy.
///
/// **ETXTBSY IS A PROPERTY OF THIS HARNESS, NOT OF THE THING UNDER TEST.**
/// `fs::copy` closes its own destination handle, but a test binary is
/// multi-threaded and several of its tests fork (`Command::output`). A child
/// forked between another thread's open and close inherits that write fd, and
/// between `fork` and `execve` it still holds it -- Linux refuses to `execve`
/// a file any process has open for writing. macOS does not enforce that, which
/// is exactly why the macOS leg stays green while ubuntu reddens on
/// `Os { code: 26, kind: ExecutableFileBusy }`.
///
/// Bounded retry rather than a mutex: the window is microseconds, it belongs to
/// the harness, and serialising these tests would slow them to buy determinism
/// these assertions do not need. A retry that never succeeds still fails, and
/// says why.
///
/// Matched on `raw_os_error() == 26` rather than `ErrorKind::ExecutableFileBusy`
/// so this does not depend on that variant's stabilisation.
///
/// # Why it lives HERE rather than beside one of its callers
///
/// **It was written correctly, reasoned correctly, and applied to a population
/// of one when the population was three.** `eb4fe67c` fixed the site that had
/// just reddened CI, and its commit message says "the ONE
/// exec-of-a-just-copied-binary". Three integration tests copy
/// `CARGO_BIN_EXE_intent` out and exec the copy -- `embedded_init`,
/// `info_exit_code` and `migrated_guards_still_refuse` -- and over the twelve
/// `rust` runs to 2026-08-24 **four failed, all on this error, across two of
/// those three files**. The third has the identical exposure and had simply not
/// lost the race yet.
///
/// **The set came from what was in hand rather than from what the property
/// reaches**, and one grep for the copy call settles it. That is the same class
/// this crate's own header describes: a duplication found by grepping one name,
/// which could not see the copies wearing the other.
pub fn output_retrying_busy(mut build: impl FnMut() -> Command, what: &str) -> Output {
  const ETXTBSY: i32 = 26;
  let mut last = String::new();
  for _ in 0..100 {
    match build().output() {
      Ok(out) => return out,
      Err(e) if e.raw_os_error() == Some(ETXTBSY) => {
        last = e.to_string();
        std::thread::sleep(std::time::Duration::from_millis(20));
      }
      Err(e) => panic!("{what}: {e}"),
    }
  }
  panic!(
    "{what}: still busy after 100 attempts over ~2s ({last}) -- that is no longer a fork race"
  );
}

/// **A TEST FILE NOBODY DECLARES IS NEVER COMPILED AND SAYS NOTHING.**
///
/// `autotests = false` bought one linked binary instead of one per file, and it
/// inverted the failure rather than removing it. Before: a stray `tests/quick.rs`
/// silently became another target. After: a stray `tests/quick.rs` silently
/// becomes NOTHING -- not compiled, not run, not reported. **Both are silent;
/// only the second can lose coverage that someone believed they had**, which is
/// why TN001 refuses to ship the consolidation without this check.
///
/// Adopted from Laksa, which hit the same inversion taking hv's 2026-08-27
/// ruling and built the guard the same morning (`01c00c91f`).
///
/// **IT LIVES HERE BECAUSE IT WAS COPY-PASTED THREE TIMES AND NOTHING HELD THE
/// COPIES TOGETHER.** `intent-cli`, `intentd` and `intentsvcs` each carried a
/// byte-identical 93-line file with no drift test between them -- the exact
/// shape this crate's own header was written about, arriving in the guard that
/// TN001 calls non-optional. A guard is an IMPLEMENTATION, not an index, so a
/// drift test would not have licensed the copies.
///
/// **KEYED ON `#[path]`, NEVER ON THE MOD NAME.** Only the path decides what
/// gets compiled: `#[path = "a.rs"] mod b;` compiles `a.rs`, and a guard reading
/// `mod b` would report `b.rs` -- a file that need not exist -- as covered while
/// `a.rs` went unchecked. The two agree by convention everywhere in this tree
/// today, which is exactly the condition under which reading the wrong one
/// passes.
///
/// Call it from a crate's suite with its own `CARGO_MANIFEST_DIR`.
pub fn assert_no_orphan_suite_members(manifest_dir: &str) {
  let root = Path::new(manifest_dir);
  let dir = root.join("tests");
  let suite = std::fs::read_to_string(dir.join("suite.rs")).expect("the suite must exist");
  let manifest = std::fs::read_to_string(root.join("Cargo.toml")).expect("the manifest must exist");

  let declared = declared_suite_paths(&suite);
  let independent = independently_declared_targets(&manifest);

  let mut orphans: Vec<String> = Vec::new();
  for entry in std::fs::read_dir(&dir).expect("tests/ must be readable") {
    let entry = entry.expect("a readable dir entry");
    if !entry.file_type().expect("a file type").is_file() {
      continue;
    }
    let name = entry.file_name().to_string_lossy().to_string();
    if !name.ends_with(".rs") || name == "suite.rs" {
      continue;
    }
    if declared.contains(&name) || independent.contains(&name) {
      continue;
    }
    orphans.push(name);
  }
  orphans.sort();

  assert!(
    orphans.is_empty(),
    "these files under tests/ are compiled by NOTHING and run NOWHERE -- declare each in \
     tests/suite.rs as `#[path = \"<name>\"] mod <stem>;`, or give it its own [[test]] in \
     Cargo.toml if it must stay a separate process: {orphans:?}"
  );

  // The guard's own positive control. A guard over a set it cannot read is a
  // guard that passes over an empty directory, and an empty `declared` would
  // make every branch above vacuous while reporting exactly the same green.
  assert!(
    !declared.is_empty(),
    "no `#[path]` declarations parsed out of {} -- this guard was reading nothing \
     and would have passed over any orphan at all",
    dir.join("suite.rs").display()
  );
}

/// Files that legitimately are not suite members: anything carrying its OWN
/// `[[test]]` target in the manifest -- read from the manifest rather than
/// listed here, so adding an isolated target cannot desynchronise from the
/// guard.
fn independently_declared_targets(manifest: &str) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  for line in manifest.lines() {
    let line = line.trim();
    if let Some(rest) = line.strip_prefix("path = \"tests/")
      && let Some(name) = rest.strip_suffix("\"")
    {
      out.insert(name.to_string());
    }
  }
  out
}

fn declared_suite_paths(suite: &str) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  for line in suite.lines() {
    let line = line.trim();
    if let Some(rest) = line.strip_prefix("#[path = \"")
      && let Some(p) = rest.strip_suffix("\"]")
    {
      out.insert(p.to_string());
    }
  }
  out
}

/// What a sweep did, so a caller can assert on it rather than trust it.
///
/// A `()`-returning sweep is indistinguishable from a sweep that matched
/// nothing, which is the failure this estate keeps meeting: an instrument whose
/// silence and whose success look identical.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SweepReport {
  /// Directories whose NAME said they were an abandoned-fixture candidate.
  pub examined: usize,
  /// Candidates whose creating process is gone, and which were removed.
  pub removed: usize,
  /// Candidates whose creating process is still alive, deliberately left.
  pub kept_live: usize,
  /// The live-process set could not be read, so NOTHING was removed.
  pub refused: bool,
}

/// Remove `/tmp` fixture directories whose creating process is gone.
///
/// # Why this runs at START and never at exit
///
/// Every fixture home already has a `Drop` that removes it, and `/tmp` held
/// **902 of them** when this was written. `Drop` is the exit path, and the exit
/// path is exactly the one that does not run when a test binary is killed --
/// which is the condition that produced the whole population. **A cleanup that
/// only runs on the happy path cleans up only what did not need cleaning.**
///
/// So this is called before the first fixture of a run is created. It cleans up
/// after the PREVIOUS run's corpses rather than its own, which is the only
/// ordering that survives its own process being killed.
///
/// # The discriminator is STRUCTURAL, not a list of prefixes
///
/// A candidate is a directory whose name ends `-<pid>-<counter>` and begins
/// `intent`. **A prefix list would have been wrong on the day it was written:**
/// the six creation sites spell four families (`intent-fixture-*`,
/// `intentd-proj`, `intentd-home`, and the `execwitness` variant that wears an
/// `intent-fixture-` prefix), and reading the code found fewer families than
/// reading the disk did. The trailing `-<pid>-<counter>` is what every creation
/// site actually produces, so matching on it catches families nobody listed.
///
/// **AND IT IS WHAT EXCLUDES THE THINGS THAT ARE NOT FIXTURES.** `/tmp/intent`
/// is the `in-session` gate's sentinel directory and `/tmp/intentfiles.new` is
/// a stray file; both start with `intent` and neither carries the suffix, so
/// neither is a candidate. A prefix-only match would have deleted the sentinel
/// directory out from under every running Claude Code session on this machine.
///
/// # Refusing beats guessing
///
/// If the live-process set cannot be read, this removes NOTHING and says so in
/// [`SweepReport::refused`]. **A sweep that cannot tell live from dead and
/// deletes anyway is worse than the leak**: the live fixtures belong to test
/// runs in progress, including other developers' on a shared machine.
///
/// Pid reuse can only make this KEEP an abandoned directory (its dead pid now
/// names some live process), never delete a live one -- a live fixture's own
/// process is alive by construction. The failure direction is a leak that
/// persists, which is safe.
///
/// # `/tmp` literally, not `temp_dir()`
///
/// The creation sites hardcode `PathBuf::from("/tmp")`, and on macOS
/// `std::env::temp_dir()` honours `TMPDIR`, which is a per-user directory and
/// NOT `/tmp`. Reading a different directory than the writers write to is the
/// population-mismatch defect; this deliberately matches the writers. When
/// `WP-03` gives spawning one home, this constant collapses with theirs.
pub fn sweep_abandoned_fixtures() -> SweepReport {
  let mut report = SweepReport::default();
  let Some(live) = live_pids() else {
    report.refused = true;
    return report;
  };
  let Ok(entries) = std::fs::read_dir("/tmp") else {
    return report;
  };
  for entry in entries.flatten() {
    let name = entry.file_name();
    let Some(name) = name.to_str() else { continue };
    let Some(pid) = abandoned_fixture_pid(name) else {
      continue;
    };
    if !entry.path().is_dir() {
      continue;
    }
    report.examined += 1;
    if live.contains(&pid) {
      report.kept_live += 1;
    } else if std::fs::remove_dir_all(entry.path()).is_ok() {
      report.removed += 1;
    }
  }
  report
}

/// The creating pid of a fixture directory name, or `None` if the name is not
/// one.
///
/// Split out so the naming contract is testable WITHOUT touching `/tmp`: the
/// exclusions above (`intent`, `intentfiles.new`) are assertions about this
/// function, and a test that had to plant real directories to check them would
/// be slower and would race every other suite on the machine.
pub fn abandoned_fixture_pid(name: &str) -> Option<u32> {
  if !name.starts_with("intent") {
    return None;
  }
  let (rest, counter) = name.rsplit_once('-')?;
  let (_family, pid) = rest.rsplit_once('-')?;
  // BOTH trailing segments must be numeric. `intent-fixture-browse-wp-absent`
  // ends in a word, and reading `absent` as a counter would make its parent
  // segment the "pid" -- a name-shaped match on a directory that is not one.
  counter.parse::<u32>().ok()?;
  pid.parse().ok()
}

/// Every live pid on this machine, or `None` if the question could not be
/// answered.
///
/// `ps` rather than `/proc`, which does not exist on macOS, and rather than
/// `kill -0` per candidate, which would be one process spawn per directory --
/// 902 of them on the day this was written.
fn live_pids() -> Option<BTreeSet<u32>> {
  let out = Command::new("ps").args(["-Ao", "pid="]).output().ok()?;
  if !out.status.success() {
    return None;
  }
  let set: BTreeSet<u32> = String::from_utf8_lossy(&out.stdout)
    .split_whitespace()
    .filter_map(|p| p.parse().ok())
    .collect();
  // An EMPTY set means `ps` answered and told us nothing, which cannot be true
  // -- this process is alive. Treating it as "no pid is live" would sweep every
  // fixture on the machine, including running ones.
  if set.is_empty() { None } else { Some(set) }
}

/// The at-START hook every `/tmp` fixture creation site calls before it creates
/// its first directory.
///
/// Separate from [`sweep_abandoned_fixtures`] on purpose: the worker returns a
/// [`SweepReport`] and is therefore assertable, while this one is idempotent per
/// process and returns nothing, which is what a hook wants and what a test
/// cannot check. **Collapsing them would make the worker unassertable after its
/// first call** -- the second call would report zero removals and a test could
/// not tell that from a sweep that does nothing.
pub fn sweep_once() {
  static ONCE: std::sync::Once = std::sync::Once::new();
  ONCE.call_once(|| {
    let _ = sweep_abandoned_fixtures();
  });
}
