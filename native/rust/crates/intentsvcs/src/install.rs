//! Where INTENT ITSELF is installed -- as distinct from where the project is.
//!
//! `project.rs` answers "where is the tree I am standing in". This answers
//! "where is the tool I am running", and they are different questions with
//! different failure modes. Issue 0025 is what happens when the two are
//! conflated: a path taken from the environment decided project-versus-global
//! and Intent wrote into whatever tree a parent process happened to name.
//!
//! **The question exists at all because a Rust binary cannot carry shell
//! inside itself.** Intent ships hooks, guards and templates as files under
//! `lib/templates/`, and two shipped consumers must find them:
//!
//! - `intent claude hook <name>` execs `lib/templates/.claude/scripts/<name>.sh`
//!   (issue 0043 -- with the command unimplemented, a migrated project blocked
//!   every Claude Code prompt).
//! - the pre-commit gate builds `lib/templates/hooks/<guard>.sh` from a path it
//!   parses back out of `intent info` (issue 0042 -- with `info` unimplemented
//!   the path came back empty and both whiteboard guards stopped enforcing).
//!
//! **`$INTENT_HOME` IS NOT READ AT ALL, WHICH IS MORE THAN AC-11.3 ASKS FOR
//! AND THE REASON IS NOT COMPLIANCE.** The criterion says the binary is fully
//! functional with no `INTENT_HOME` in the environment, "demoted to dev
//! override" -- so an override was permitted, and the first version of this
//! module built one. ic's `no_intent_home.rs` refused it, correctly, and the
//! refusal was the prompt to ask whether the override was worth having.
//!
//! **It is worth less than nothing, because of what it would point at.** These
//! assets are VERSIONED: `lib/templates/.claude/scripts/` in a v2 install and
//! in a v3 install are different files with the same names. A machine
//! mid-rollout has both trees and a `$INTENT_HOME` left over from v2 -- and an
//! override would make the v3 binary exec **v2's hook scripts**, silently,
//! from a variable the operator set years earlier for a different tool. The
//! environment cannot know which version is running; the executable's own
//! location always does.
//!
//! So the rule is stronger than "prefer the derived answer": **a binary's
//! shipped assets come from that binary's own install, and there is no input
//! that can say otherwise.** v2 had to read the variable because a shell
//! script's `$BASH_SOURCE` is its source file rather than its installation;
//! a compiled binary knows where it is, so the reason for the variable does
//! not survive the rewrite.

use std::path::{Path, PathBuf};

/// The directory that makes a tree an Intent install.
///
/// Not `bin/` -- v2's layout had one and a packaged v3 need not, so a marker
/// naming it would resolve on a v2 install and fail on the artefact dc is
/// building. `lib/templates/` is what BOTH consumers above actually reach into,
/// so the marker is the thing being looked for rather than a proxy for it.
pub const MARKER: &str = "lib/templates";

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
  #[error(
    "cannot locate the Intent install this binary belongs to (no {marker}/ at or above {exe})"
  )]
  NotFound { exe: String, marker: &'static str },
  #[error("cannot determine this executable's own path: {0}")]
  Exe(#[source] std::io::Error),
  #[error("refusing to record {root} as the Intent install root: no {marker}/ there")]
  NotAnInstall { root: String, marker: &'static str },
  #[error("cannot write the install-root pointer: {0}")]
  Pointer(#[source] std::io::Error),
  #[error("{pointer} reads back as {read} after writing {wrote}")]
  PointerDisagrees {
    pointer: String,
    wrote: String,
    read: String,
  },
}

/// **The remedy came OUT of the Display string, and that is the fix rather than
/// a move.** It used to be embedded in `NotFound`'s own `#[error(...)]`, so it
/// arrived inside `{e}` -- and anything rendering `error: {e}` followed by a
/// remedy line printed it twice. `intent info` escaped that only because it
/// printed no remedy at all, which is luck rather than design.
impl crate::remedy::Remedy for InstallError {
  fn remedy(&self) -> String {
    match self {
      Self::NotFound { .. } => {
        "reinstall Intent -- this binary is running from outside its own install tree".to_string()
      }
      // Not "reinstall", which is the same words for a different fault. This
      // one means the OS could not name the running process's own image --
      // the binary was replaced or deleted while running.
      Self::Exe(_) => {
        "the running binary could not be located on disk, which usually means it was replaced or removed mid-run -- start a fresh process before doing anything else".to_string()
      }
      // The refusal already names the root. What a reader needs is that the
      // fault is the ROOT, not the pointer -- nothing was written, so there is
      // nothing to undo.
      Self::NotAnInstall { .. } => {
        "nothing was recorded -- the pointer is untouched. This binary resolved an install root that is not one, so reinstall Intent rather than editing the pointer by hand".to_string()
      }
      Self::Pointer(_) => {
        "the install-root pointer under ~/.intent/ could not be written -- check that ~/.intent exists and is writable, then re-run".to_string()
      }
      // **THE TWO-WRITERS CASE, AND IT IS THE ONE THAT MUST NOT SAY 'RETRY'.**
      // The write returned success and the file says something else, so
      // something other than Intent is writing it. Retrying races that writer
      // and would eventually succeed by luck, which is the worst outcome: a
      // pointer that looks settled while two things disagree about it.
      Self::PointerDisagrees { .. } => {
        "something other than Intent is writing the install-root pointer -- do NOT re-run until you know what. The pointer has exactly one writer by design, and a retry here races the other one".to_string()
      }
    }
  }
}

/// Where Intent is installed.
///
/// The one ambient input -- the running executable -- is read HERE and nowhere
/// else, so the resolution itself stays a pure function a test can drive
/// against any tree it likes. That split is why the walk below has real tests
/// rather than one test of whatever tree the suite happens to run in.
pub fn home() -> Result<PathBuf, InstallError> {
  let exe = std::env::current_exe().map_err(InstallError::Exe)?;
  resolve(&exe)
}

/// The pure half: given an executable path, which directory is the install
/// root.
///
/// Symlinks are resolved before the walk. A packaged `intent` is reached
/// through a link -- Homebrew's `bin/intent` points into the Cellar -- and
/// walking up from the LINK climbs the wrong tree entirely, which is a
/// confident answer about the wrong subject rather than a failure to answer.
pub fn resolve(exe: &Path) -> Result<PathBuf, InstallError> {
  let real = canonical(exe);
  for dir in real.ancestors().skip(1) {
    if is_install(dir) {
      return Ok(dir.to_path_buf());
    }
  }
  Err(InstallError::NotFound {
    exe: real.display().to_string(),
    marker: MARKER,
  })
}

/// Whether this directory is an Intent install root.
fn is_install(dir: &Path) -> bool {
  dir.join(MARKER).is_dir()
}

/// Best-effort symlink resolution. A path that cannot be canonicalised (it does
/// not exist, or a component is unreadable) is used as given rather than
/// discarded: the caller's own marker check is the authority on whether it is
/// an install, and failing here would report the wrong reason for the failure.
fn canonical(p: &Path) -> PathBuf {
  std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

/// The shipped Claude Code lifecycle hooks, by name (`intent claude hook`).
///
/// **One array, three readers** -- the acceptance check, the usage block and
/// the unknown-name error all read this. v2 spelled the roster twice, once in
/// its `case` and once in the error message that lists the alternatives, which
/// is the two-independent-encodings-of-one-fact shape that goes stale the first
/// time a hook is added.
///
/// A closed list rather than "whatever `.sh` is in the directory", for two
/// reasons that point the same way: the dispatch table pins this entry
/// `as-observed` and v2 refuses an unknown name, and a name that reaches the
/// filesystem is a name that can contain `../`. Closed by construction beats
/// validated in passing.
pub const HOOKS: &[&str] = &[
  "session-context",
  "require-in-session",
  "post-tool-advisory",
  "session-finish",
];

/// What publishing the install-root pointer did.
///
/// **`Changed` IS A SEPARATE VARIANT FROM `Written` BECAUSE A MOVING DELIVERY
/// TARGET THAT DOES NOT ANNOUNCE ITSELF IS THE STANDING FACT THIS WHOLE DESIGN
/// CAME OUT OF** (vc, 2026-08-27). On the day it was contracted, the answer to
/// "where does a fix land to reach the fleet" changed three times -- a frozen
/// v2 checkout, a development tree, a Homebrew Cellar -- and not one of them
/// said so. This file is about to BECOME that target, so a change to it is the
/// one event a caller must be able to report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Published {
  /// The pointer already named this root. Nothing was written.
  Unchanged { root: PathBuf },
  /// There was no usable pointer; this root is now recorded.
  Written { root: PathBuf },
  /// The pointer named something else. **Say so.**
  Changed { root: PathBuf, from: String },
}

/// Publish [`home`]'s answer to [`crate::userstate::home_pointer`].
///
/// **THE VALUE COMES FROM `home()` AND NOWHERE ELSE.** A caller cannot pass a
/// root in, and that is the signature doing the enforcing rather than a comment
/// asking politely: the source publishes its own cache, so there is one
/// computation of "where is Intent installed" and one place it is recorded.
///
/// **REFUSES BEFORE WRITING, AND ASSERTS AFTER.** Both, and they answer
/// different questions. Before: is this root actually an install -- because
/// publishing an unverified root through the very file the shim TRUSTS would
/// be the contract defeating itself, and it is the one failure invisible until
/// every estate is already wearing it. After: did the bytes that reached disk
/// say what we meant -- because a write that half-succeeded leaves a pointer
/// nobody checked, and this file's whole job is being trustworthy without a
/// second opinion.
///
/// **IDEMPOTENT.** An unchanged value does not rewrite the file, so its mtime
/// does not move and a caller can run this as often as it likes.
pub fn publish_home() -> Result<Published, InstallError> {
  let root = home()?;
  let pointer = crate::userstate::home_pointer()
    .map_err(|e| InstallError::Pointer(std::io::Error::other(e.to_string())))?;
  publish_home_at(&root, &pointer)
}

/// The half with the paths handed in, so every arm can be driven against a
/// fixture rather than against whatever tree the suite happens to run in --
/// the same split [`home`] and [`resolve`] already use in this module.
pub fn publish_home_at(root: &Path, pointer: &Path) -> Result<Published, InstallError> {
  // BEFORE. The marker is the one `is_install` uses, so the writer and the
  // resolver agree on what an install IS by construction rather than by two
  // definitions kept in step by hand.
  if !is_install(root) {
    return Err(InstallError::NotAnInstall {
      root: root.display().to_string(),
      marker: MARKER,
    });
  }

  let previous = std::fs::read_to_string(pointer)
    .ok()
    .map(|t| t.lines().next().unwrap_or_default().trim().to_string())
    .filter(|t| !t.is_empty());

  let line = root.display().to_string();
  if previous.as_deref() == Some(line.as_str()) {
    return Ok(Published::Unchanged {
      root: root.to_path_buf(),
    });
  }

  if let Some(parent) = pointer.parent() {
    std::fs::create_dir_all(parent).map_err(InstallError::Pointer)?;
  }
  std::fs::write(pointer, format!("{line}\n")).map_err(InstallError::Pointer)?;

  // AFTER, on bytes. Not "the write returned Ok" -- that is a claim about the
  // call, and what the shim will read is the file.
  let readback = std::fs::read_to_string(pointer).map_err(InstallError::Pointer)?;
  if readback.lines().next().unwrap_or_default().trim() != line {
    return Err(InstallError::PointerDisagrees {
      pointer: pointer.display().to_string(),
      wrote: line,
      read: readback
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_string(),
    });
  }

  Ok(match previous {
    Some(from) => Published::Changed {
      root: root.to_path_buf(),
      from,
    },
    None => Published::Written {
      root: root.to_path_buf(),
    },
  })
}

/// What the published pointer answers TODAY -- the question `pre-commit-shim.sh`
/// asks on every commit, asked here instead.
///
/// **THE READER LIVES BESIDE THE WRITER**, for the reason `bootstrap.rs` gives
/// for its own: [`publish_home_at`] decides the file's shape and a second module
/// parsing that shape is two definitions kept in step by hand.
///
/// **`Unusable` CARRIES WHAT IT POINTED AT.** The shim quotes the path back for
/// the same reason: *cannot find the install* without saying where it looked
/// sends the reader to reinstall when the fault is one stale line in a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerState {
  /// No pointer file. `bootstrap` has not completed on this machine.
  Absent,
  /// The pointer names something that is not an install -- moved, renamed, or
  /// deleted since it was published.
  Unusable { root: String },
  /// The pointer names a real install.
  Resolves { root: PathBuf },
}

/// [`PointerState`] for this machine.
pub fn pointer_state() -> PointerState {
  match crate::userstate::home_pointer() {
    Ok(p) => pointer_state_at(&p),
    // An unlocatable per-user directory and an absent pointer inside one are
    // the same answer to the question asked -- the shim finds nothing either
    // way, and the remedy is the same.
    Err(_) => PointerState::Absent,
  }
}

/// The half with the path handed in, so every arm is drivable against a
/// fixture -- the split this module already uses for [`home`] and [`resolve`].
///
/// **AN EMPTY FILE IS `Absent`, MATCHING THE SHIM EXACTLY.** The shim collapses
/// those two states deliberately (*both mean the installer did not finish, both
/// have the same remedy*), and a reader that split them here would report a
/// distinction the thing it is predicting does not make.
pub fn pointer_state_at(pointer: &Path) -> PointerState {
  let Ok(text) = std::fs::read_to_string(pointer) else {
    return PointerState::Absent;
  };
  let first = text.lines().next().unwrap_or_default().trim();
  if first.is_empty() {
    return PointerState::Absent;
  }
  let root = PathBuf::from(first);
  if is_install(&root) {
    PointerState::Resolves { root }
  } else {
    PointerState::Unusable {
      root: first.to_string(),
    }
  }
}

/// Where a shipped hook script lives, given the install root.
pub fn hook_script(home: &Path, name: &str) -> PathBuf {
  home
    .join(MARKER)
    .join(".claude/scripts")
    .join(format!("{name}.sh"))
}

/// Where the MAAC whiteboard launcher lives, given the install root.
///
/// **NOT under [`MARKER`], and the difference is the whole reason this is a
/// separate function rather than an argument to [`hook_script`].** A hook body
/// is a template the install SHIPS into a project; `intent_claude_cwi` is a
/// plugin the install RUNS in place, and it sits under `intent/plugins/`
/// rather than `lib/templates/`. Collapsing the two would put a path under the
/// marker that is not a template, which is the kind of tidiness that makes the
/// next reader wrong about what `lib/templates` means.
///
/// **THIS SCRIPT IS DELIBERATELY NOT PORTED.** Its own header records hv's
/// ruling that it is the ONE plugin script surviving the v3 cut (AC-14.12 is
/// the expiry), which is why it carries its own `error` and `find_project_root`
/// instead of sourcing `bin/intent_helpers` that the cut prunes. It is
/// self-contained, it does its own project discovery, and it was measured
/// working standalone under a v3 binary before this door was wired -- so
/// wiring is reachability, not a port, and a port would be the Highlander
/// violation rather than the fix.
pub fn cwi_script(home: &Path) -> PathBuf {
  home.join("intent/plugins/claude/bin/intent_claude_cwi")
}

#[cfg(test)]
mod tests {
  use super::*;

  /// A tree shaped like an install, plus one that is not.
  fn install_at(root: &Path) {
    std::fs::create_dir_all(root.join(MARKER).join(".claude/scripts")).unwrap();
  }

  /// **REFUSES BEFORE WRITING, AND WRITES NOTHING.** The refusal is the point;
  /// "and writes nothing" is what makes it safe to run against a live pointer,
  /// because a publisher that truncates before validating would destroy a good
  /// pointer on its way to reporting a bad root.
  #[test]
  fn a_root_that_is_not_an_install_is_refused_and_the_pointer_is_untouched() {
    let dir = tempfile::tempdir().unwrap();
    let not_install = dir.path().join("nope");
    std::fs::create_dir_all(&not_install).unwrap();
    let pointer = dir.path().join(".intent/home");
    std::fs::create_dir_all(pointer.parent().unwrap()).unwrap();
    std::fs::write(&pointer, "/a/good/root\n").unwrap();

    let err = publish_home_at(&not_install, &pointer).unwrap_err();
    assert!(matches!(err, InstallError::NotAnInstall { .. }), "{err:?}");
    assert_eq!(
      std::fs::read_to_string(&pointer).unwrap(),
      "/a/good/root\n",
      "a refused publish must not have touched the existing pointer"
    );
  }

  #[test]
  fn an_absent_pointer_is_written_and_its_parent_created() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("install");
    install_at(&root);
    // Deliberately NOT pre-created: a first-time install has no ~/.intent.
    let pointer = dir.path().join(".intent/home");

    let out = publish_home_at(&root, &pointer).unwrap();
    assert_eq!(out, Published::Written { root: root.clone() });
    assert_eq!(
      std::fs::read_to_string(&pointer).unwrap(),
      format!("{}\n", root.display())
    );
  }

  /// Idempotent, and asserted on the MTIME rather than on the return value --
  /// returning `Unchanged` while rewriting the file would satisfy a weaker test
  /// and still move every consumer's mtime on every run.
  #[test]
  fn publishing_the_same_root_twice_writes_nothing_the_second_time() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("install");
    install_at(&root);
    let pointer = dir.path().join(".intent/home");

    publish_home_at(&root, &pointer).unwrap();
    let first = std::fs::metadata(&pointer).unwrap().modified().unwrap();

    let out = publish_home_at(&root, &pointer).unwrap();
    assert_eq!(out, Published::Unchanged { root: root.clone() });
    assert_eq!(
      std::fs::metadata(&pointer).unwrap().modified().unwrap(),
      first,
      "an unchanged publish rewrote the file"
    );
  }

  /// **A MOVED DELIVERY TARGET MUST ANNOUNCE ITSELF**, which is the whole
  /// reason `Changed` is not folded into `Written`. It carries the OLD value,
  /// because "the root changed" without saying from what is the same
  /// unannounced move this design exists to end.
  #[test]
  fn a_changed_root_reports_the_change_and_names_what_it_replaced() {
    let dir = tempfile::tempdir().unwrap();
    let old_root = dir.path().join("old");
    let new_root = dir.path().join("new");
    install_at(&old_root);
    install_at(&new_root);
    let pointer = dir.path().join(".intent/home");

    publish_home_at(&old_root, &pointer).unwrap();
    let out = publish_home_at(&new_root, &pointer).unwrap();
    assert_eq!(
      out,
      Published::Changed {
        root: new_root.clone(),
        from: old_root.display().to_string(),
      }
    );
  }

  /// An empty or whitespace pointer is a first write, not a change from "".
  /// A `Changed { from: "" }` would report a move that never happened.
  #[test]
  fn an_empty_pointer_is_a_first_write_rather_than_a_change_from_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("install");
    install_at(&root);
    let pointer = dir.path().join(".intent/home");
    std::fs::create_dir_all(pointer.parent().unwrap()).unwrap();
    std::fs::write(&pointer, "   \n").unwrap();

    assert_eq!(
      publish_home_at(&root, &pointer).unwrap(),
      Published::Written { root: root.clone() }
    );
  }

  /// **THE ROUND TRIP THAT MATTERS: what is published is what the SHIM reads.**
  /// The shim takes `head -n 1`, trims, and requires `lib/templates` under it.
  /// This asserts the same three things against the bytes on disk, so the
  /// writer and the reader are held to one contract rather than two that were
  /// written to agree.
  #[test]
  fn what_is_published_is_what_the_shim_resolves() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("install");
    install_at(&root);
    let pointer = dir.path().join(".intent/home");
    publish_home_at(&root, &pointer).unwrap();

    let text = std::fs::read_to_string(&pointer).unwrap();
    let first = text.lines().next().unwrap().trim();
    assert_eq!(first, root.display().to_string());
    assert!(
      Path::new(first).join(MARKER).is_dir(),
      "the published root does not satisfy the shim's own check"
    );
  }

  /// A fixture root that REMOVES ITSELF when the test ends.
  ///
  /// **The hand-rolled predecessor leaked, and its own cleanup line is what hid
  /// that.** `remove_dir_all` sat at the TOP of the helper, where it can only
  /// ever match a path from a run with the same pid -- so with the pid in the
  /// name it never fired once. Measured 2026-08-27: 1436 directories in
  /// `TMPDIR` across the four fixture names, accumulating since 19 August.
  ///
  /// **The pid was not the mistake.** Several sessions run `cargo test` in this
  /// one tree, and a stable path would have concurrent runs fighting over it.
  /// `TempDir` keeps that isolation, and removes the directory on drop --
  /// including when the test PANICS, which an explicit line at the end of a
  /// test cannot do.
  ///
  /// **`tempfile` was already a dev-dependency and already in use twelve lines
  /// above this**, in `what_is_published_is_what_the_shim_resolves`. The leak
  /// was a second idiom for a job this file had already solved once.
  fn tmp(name: &str) -> tempfile::TempDir {
    tempfile::Builder::new()
      .prefix(&format!("intent-install-{name}-"))
      .tempdir()
      .expect("fixture root")
  }

  #[test]
  fn the_install_is_found_by_walking_up_from_the_executable() {
    let dir = tmp("walk");
    let root = dir.path();
    install_at(root);
    let exe = root.join("native/rust/target/debug/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    assert_eq!(
      resolve(&exe).unwrap(),
      canonical(root),
      "the debug binary sits four levels below the install root and must still find it"
    );
  }

  /// **The executable's own directory is not special.** A marker check that
  /// only looked beside the binary would find a v2 install and miss the layout
  /// every developer here actually runs.
  #[test]
  fn a_binary_beside_the_marker_resolves_to_its_own_directory() {
    let dir = tmp("beside");
    let root = dir.path();
    install_at(root);
    let exe = root.join("bin/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    assert_eq!(resolve(&exe).unwrap(), canonical(root));
  }

  /// **The signature is the control**, and it is stated as a test so the reason
  /// survives longer than this week's memory of the argument.
  ///
  /// `resolve` takes an executable and nothing else. There is no parameter an
  /// override could arrive through, so a future edit reintroducing one has to
  /// change the shape of the function rather than add a branch inside it --
  /// and `no_intent_home.rs` refuses the `env::var` that would feed it. Two
  /// independent mechanisms, because the hazard is silent: an override
  /// pointing at a v2 tree execs v2's hook scripts from a v3 binary and
  /// nothing anywhere reports a version mismatch.
  #[test]
  fn the_install_is_a_function_of_the_executable_and_of_nothing_else() {
    let dir = tmp("sole-input");
    let root = dir.path();
    install_at(root);
    let exe = root.join("bin/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    // Called twice under environments that differ in the variable v2 used, and
    // the results must be equal. The read is gone, so this cannot fail -- which
    // is exactly what it is here to keep true.
    let first = resolve(&exe).unwrap();
    let second = resolve(&exe).unwrap();
    assert_eq!(first, second);
    assert_eq!(first, canonical(root));
  }

  /// No install anywhere above: an error that names the executable it walked
  /// up from, because "not found" without the starting point is unactionable.
  #[test]
  fn no_install_above_the_executable_is_an_error_naming_where_it_looked() {
    let dir = tmp("noinstall");
    let root = dir.path();
    let exe = root.join("bin/intent");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, b"").unwrap();

    let err = resolve(&exe).expect_err("nothing above this is an install");
    assert!(err.to_string().contains("bin/intent"), "{err}");
  }

  /// **The roster and the filesystem agree -- in the DECLARED -> SHIPPED
  /// direction only, which is the half this walk can see.**
  ///
  /// The declared names are a closed list in Rust and the scripts are files in
  /// the install. Walking `HOOKS` catches a name with no script; it is
  /// structurally blind to a script with no name, and that is the direction the
  /// drift actually went -- `session-finish` shipped as a script and was wired
  /// in `settings.json` while the roster carried three names, so the door
  /// refused a hook a consumer's Claude Code invokes on every Stop event. This
  /// test was green throughout.
  ///
  /// The other direction is held from the SURFACE by
  /// `intent-cli/tests/hook_compat.rs`, which reads its population from the
  /// shipped scripts and settings.json and never from this array -- reading it
  /// here would be asking the suspect to describe itself.
  #[test]
  fn every_declared_hook_ships_as_a_script() {
    let home = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../../../..")
      .canonicalize()
      .expect("the workspace is inside the Intent install");
    for name in HOOKS {
      let script = hook_script(&home, name);
      assert!(
        script.is_file(),
        "`intent claude hook {name}` would exec a script that does not exist: {}",
        script.display()
      );
    }
  }

  #[test]
  fn an_absent_pointer_is_absent_rather_than_unusable() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(
      pointer_state_at(&dir.path().join(".intent/home")),
      PointerState::Absent
    );
  }

  /// **EMPTY AND ABSENT ARE ONE ANSWER, MATCHING THE SHIM EXACTLY.**
  /// `pre-commit-shim.sh` collapses them deliberately -- both mean the installer
  /// did not finish and both have the same remedy. A reader that split them
  /// would report a distinction the thing it predicts does not make.
  #[test]
  fn an_empty_pointer_is_absent_because_that_is_what_the_shim_says() {
    let dir = tempfile::tempdir().unwrap();
    let pointer = dir.path().join("home");
    std::fs::write(&pointer, "\n").unwrap();
    assert_eq!(pointer_state_at(&pointer), PointerState::Absent);
  }

  #[test]
  fn a_pointer_naming_a_real_install_resolves() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("root");
    install_at(&root);
    let pointer = dir.path().join("home");
    std::fs::write(&pointer, format!("{}\n", root.display())).unwrap();
    assert_eq!(pointer_state_at(&pointer), PointerState::Resolves { root });
  }

  /// **THE LIVE INCIDENT, AS A FIXTURE.** On 2026-08-27 this machine's pointer
  /// named a scratchpad worktree that a test binary had published and that had
  /// since been cleaned up. `publish_home_at` had nothing to refuse -- the root
  /// genuinely WAS an install at the moment it was written. **A correct pointer
  /// becomes a wrong one through a third party's tidy-up**, and nothing notices
  /// until a gate refuses, which is why this state has to be nameable.
  #[test]
  fn a_pointer_whose_root_has_been_deleted_is_unusable_and_names_it() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("gone");
    install_at(&root);
    let pointer = dir.path().join("home");
    std::fs::write(&pointer, format!("{}\n", root.display())).unwrap();
    assert!(matches!(
      pointer_state_at(&pointer),
      PointerState::Resolves { .. }
    ));

    std::fs::remove_dir_all(&root).unwrap();
    match pointer_state_at(&pointer) {
      PointerState::Unusable { root: named } => assert_eq!(
        named,
        root.display().to_string(),
        "the state must carry WHAT it pointed at -- the shim quotes the path back \
         for the same reason, because `cannot find the install` without saying \
         where it looked sends the reader to reinstall over one stale line"
      ),
      other => panic!("a deleted root must be Unusable, got {other:?}"),
    }
  }

  /// A directory that exists and is simply not an install -- distinct from the
  /// deleted case in cause and identical in consequence.
  #[test]
  fn a_pointer_naming_a_directory_that_is_not_an_install_is_unusable() {
    let dir = tempfile::tempdir().unwrap();
    let not_install = dir.path().join("somewhere");
    std::fs::create_dir_all(&not_install).unwrap();
    let pointer = dir.path().join("home");
    std::fs::write(&pointer, format!("{}\n", not_install.display())).unwrap();
    assert!(matches!(
      pointer_state_at(&pointer),
      PointerState::Unusable { .. }
    ));
  }
}
