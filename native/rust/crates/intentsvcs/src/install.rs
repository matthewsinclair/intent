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
  #[error(
    "refusing to replace the recorded Intent install root {existing} with {candidate}, which is {why}"
  )]
  WouldReplaceARealInstall {
    existing: String,
    candidate: String,
    why: String,
  },
  #[error("cannot write the install-root pointer: {0}")]
  Pointer(#[source] std::io::Error),
  #[error("{pointer} reads back as {read} after writing {wrote}")]
  PointerDisagrees {
    pointer: String,
    wrote: String,
    read: String,
  },
  #[error(
    "intent install publishes only from a dev tree, and this binary runs from the {tree} tree at {root}"
  )]
  NotDev { tree: Tree, root: String },
  #[error("intent install publishes only a clean runtime set, and {root} is not: {why}")]
  RuntimeSetDirty { root: String, why: String },
  #[error("the pair at {dir} is not one build: intent names {intent} and intentd names {intentd}")]
  SplitPair {
    dir: String,
    intent: String,
    intentd: String,
  },
  #[error("intent install needs a prefix: none was given and INTENT_INSTALL_PREFIX is not set")]
  NoPrefix,
  #[error("intent install refuses the prefix {prefix}: {why}")]
  PrefixRefused { prefix: String, why: String },
  #[error("intent install could not publish to {prefix}: {why}")]
  Publish { prefix: String, why: String },
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
      // **THE REMEDY IS NOT 'REINSTALL', AND IT NAMES THE DELIBERATE PATH.**
      // Both roots are installs and the pointer is intact; what is refused is
      // one REPLACING the other by accident. So the remedy says what was kept,
      // and gives the two ways to mean it -- because a refusal with no way
      // through is the kind an operator works around by editing the file,
      // which is the one writer this pointer must not grow.
      Self::WouldReplaceARealInstall { .. } => {
        "nothing was recorded -- the pointer still names the install it did. This usually means a test suite or a build ran `intent bootstrap` from a scratch checkout in passing. If you MEANT to move this machine's install, remove the pointer (by default ~/.local/share/intent/home) and run `intent bootstrap` again, or run it from the root you mean to deliver from".to_string()
      }
      Self::Pointer(_) => {
        "the install-root pointer under $XDG_DATA_HOME/intent (by default ~/.local/share/intent) could not be written -- check that the directory is writable, then re-run".to_string()
      }
      // **THE TWO-WRITERS CASE, AND IT IS THE ONE THAT MUST NOT SAY 'RETRY'.**
      // The write returned success and the file says something else, so
      // something other than Intent is writing it. Retrying races that writer
      // and would eventually succeed by luck, which is the worst outcome: a
      // pointer that looks settled while two things disagree about it.
      Self::PointerDisagrees { .. } => {
        "something other than Intent is writing the install-root pointer -- do NOT re-run until you know what. The pointer has exactly one writer by design, and a retry here races the other one".to_string()
      }
      Self::NotDev { .. } => {
        "run `intent install` with the dev checkout's own binary (its `native/rust/target/release/intent`); an opt tree is republished from dev with `intent install --force`, and a brew tree is upgraded with `brew upgrade intent`".to_string()
      }
      Self::RuntimeSetDirty { .. } => {
        "commit the support paths' changes and rebuild the pair at HEAD with `bin/devbin build all`, then run this again; nothing was written".to_string()
      }
      Self::SplitPair { .. } => {
        "rebuild both binaries from one commit with `bin/devbin build all`, then run this again; nothing was written".to_string()
      }
      Self::NoPrefix => {
        "pass `--prefix <dir>`, or set INTENT_INSTALL_PREFIX in this sleeve's environment; there is no default, because a publish to the wrong place looks the same as one to the right place".to_string()
      }
      Self::PrefixRefused { .. } => {
        "nothing was written. Choose a prefix Intent may publish to: a missing or empty directory, or an opt tree Intent published, with `--force` to replace it".to_string()
      }
      Self::Publish { .. } => {
        "the detail above says which step failed; a prefix that was there before is left in place or moved back, and the staging directory beside it is removed".to_string()
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

/// The Homebrew formula this project ships as, which names its keg directory
/// under `Cellar/` and its link under `opt/`.
const BREW_FORMULA: &str = "intent";

/// The path to RECORD for an install root. On a Homebrew install that is not
/// the root [`resolve`] finds (issue `0527`).
///
/// **A HOMEBREW KEG IS VERSIONED, AND `brew upgrade` DELETES THE OLD ONE.**
/// [`resolve`] canonicalises the executable, as it must, so on a brew install
/// the root it finds is `<prefix>/Cellar/intent/<version>/libexec`. A pointer
/// recording that names a directory the next upgrade's cleanup removes, and
/// from then on the project shim refuses every gated commit until `intent
/// bootstrap` runs again. `<prefix>/opt/intent` is Homebrew's link to whichever
/// keg is current, so a pointer through it survives every upgrade.
///
/// **THE STABLE PATH IS RECORDED ONLY WHEN IT RESOLVES TO THIS SAME KEG.** A
/// keg run directly while `opt` links a different version is recorded as it
/// is, because the pointer must name the install this binary belongs to. So is
/// every root outside a Cellar: a source checkout, a release build, a CI runner.
///
/// PURE in the sense [`resolve`] is: the only reads are of the paths it is
/// handed, so a planted Cellar-and-opt tree drives every arm. `root` is taken
/// as [`resolve`] returns it, already canonical.
pub fn stable_root(root: &Path) -> PathBuf {
  let Some(keg) = brew_keg(root) else {
    return root.to_path_buf();
  };
  // `<prefix>/Cellar/intent/<version>`: the prefix is three levels up.
  let Some(prefix) = keg.ancestors().nth(3) else {
    return root.to_path_buf();
  };
  let opt = prefix.join("opt").join(BREW_FORMULA);
  if canonical(&opt) != keg {
    return root.to_path_buf();
  }
  match root.strip_prefix(keg) {
    Ok(rest) if !rest.as_os_str().is_empty() => opt.join(rest),
    _ => opt,
  }
}

/// The versioned keg `root` sits in, when it sits in one of this formula's:
/// `<prefix>/Cellar/intent/<version>`.
///
/// **READ FROM THE PATH AS SPELLED, NEVER CANONICALISED.** `opt`'s path to a
/// keg resolves to that keg, so a canonical reading would call the stable path
/// [`stable_root`] records a versioned one. [`stable_root`] asks this of a root
/// [`resolve`] already canonicalised; [`gate_resolution_at`] asks it of the
/// pointer's line exactly as it was written.
fn brew_keg(root: &Path) -> Option<&Path> {
  root.ancestors().find(|keg| {
    let formula = keg.parent();
    formula.and_then(Path::file_name) == Some(std::ffi::OsStr::new(BREW_FORMULA))
      && formula.and_then(Path::parent).and_then(Path::file_name)
        == Some(std::ffi::OsStr::new("Cellar"))
  })
}

/// Whether two paths name the same install: compared by what they resolve to,
/// never by how they are spelled (issue `0527`). The pointer records
/// [`stable_root`] while [`home`] answers the canonical keg, so on a brew
/// install one directory arrives as two different strings.
pub fn same_install(a: &Path, b: &Path) -> bool {
  canonical(a) == canonical(b)
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
///
/// **A HOOK IS A `.sh` IN THAT DIRECTORY, AND WHAT IS NOT A HOOK CARRIES A
/// DIFFERENT EXTENSION** (AC-24.4, 2026-09-12). `index-freshness.bash` lives
/// beside the hook bodies and is SOURCED by them rather than run; it is not a
/// name this roster may contain, and `hook_compat.rs` reads every `*.sh` there
/// as a shipped hook, so the convention keeps both statements true at once
/// without carving an exemption out of a population -- a filtered exemption and
/// an accident look identical to the filter.
pub const HOOKS: &[&str] = &[
  "session-context",
  "require-in-session",
  "post-tool-advisory",
  "post-tool-symbol-context",
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
/// It is recorded through [`stable_root`], which names the same directory by a
/// path a Homebrew upgrade does not delete (issue `0527`).
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
  let root = stable_root(&home()?);
  let pointer = crate::userstate::home_pointer()
    .map_err(|e| InstallError::Pointer(std::io::Error::other(e.to_string())))?;
  publish_home_at(&root, &pointer)
}

/// What makes a root a SCRATCH root: one that will not outlive the pointer
/// naming it.
///
/// **THIS IS A CLASSIFICATION, NOT A VERDICT** (issue `0492`). Being scratch is
/// not on its own a reason to refuse -- a fresh machine, a CI runner and
/// `gyges` may each have nothing better to publish, and refusing there would
/// leave them with no install pointer at all. What it decides is whether this
/// candidate may REPLACE a pointer that already names a real one. See
/// [`replacement_refused`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scratch {
  /// A linked `git worktree` -- deleted when its work is done.
  LinkedWorktree { gitdir: String },
  /// Under the system temporary directory -- does not survive a reboot.
  UnderSystemTemp { temp: PathBuf },
}

impl std::fmt::Display for Scratch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::LinkedWorktree { gitdir } => {
        write!(f, "a linked git worktree, whose .git names {gitdir}")
      }
      Self::UnderSystemTemp { temp } => {
        write!(
          f,
          "under this machine's temporary directory {}",
          temp.display()
        )
      }
    }
  }
}

/// The system temporary directories a root is scratch for living under.
///
/// **`std::env::temp_dir()` ALONE IS NOT ENOUGH ON THIS PLATFORM, AND THE
/// INCIDENT PROVES IT.** macOS gives each session a private `TMPDIR` under
/// `/var/folders/`, so `temp_dir()` answers that and says nothing about
/// `/tmp` -- and the worktree that redirected every gate on 2026-09-19 was
/// under `/private/tmp`, which `temp_dir()` would have passed. The fixed roots
/// are listed beside it rather than instead of it: `TMPDIR` is the one a
/// caller can move, and the others are the ones a caller cannot.
fn system_temp_roots() -> Vec<PathBuf> {
  let mut roots = vec![std::env::temp_dir()];
  for fixed in ["/tmp", "/private/tmp", "/var/tmp", "/private/var/tmp"] {
    roots.push(PathBuf::from(fixed));
  }
  roots
}

/// The `gitdir:` target a linked worktree's `.git` FILE names, if it is one.
///
/// **READS `.git` RATHER THAN RUNNING `git`.** A linked worktree's `.git` is a
/// FILE reading `gitdir: <path>/.git/worktrees/<name>`, where the main
/// checkout's is a directory -- so the distinction is on disk and needs no
/// subprocess, no `git` on `PATH`, and no repository to be readable.
fn gitdir_target(root: &Path) -> Option<String> {
  let dot_git = root.join(".git");
  if !dot_git.is_file() {
    return None;
  }
  let text = std::fs::read_to_string(&dot_git).ok()?;
  let target = text.lines().next()?.strip_prefix("gitdir:")?.trim();
  Some(target.to_string())
}

/// PURE. Whether a root is scratch, given everything that had to be read.
///
/// **EVERY INPUT IS A PARAMETER SO BOTH VERDICTS ARE REACHABLE WITHOUT A
/// FILESYSTEM** -- the same split [`home`] and [`resolve`] already use. `root`
/// and `temps` arrive canonicalised by [`scratch`]; `gitdir` is the line
/// `gitdir_target` read, or `None`.
///
/// The `worktrees` component is what separates a linked worktree from a
/// SUBMODULE, whose `.git` is also a file and whose target names `modules`.
pub fn scratch_within(root: &Path, temps: &[PathBuf], gitdir: Option<&str>) -> Option<Scratch> {
  if let Some(target) = gitdir
    && Path::new(target)
      .components()
      .any(|c| c.as_os_str() == "worktrees")
  {
    return Some(Scratch::LinkedWorktree {
      gitdir: target.to_string(),
    });
  }
  temps
    .iter()
    .find(|temp| root.starts_with(temp))
    .map(|temp| Scratch::UnderSystemTemp { temp: temp.clone() })
}

/// [`scratch_within`] with this machine's answers read for it.
///
/// **CANONICALISED ON BOTH SIDES, BECAUSE `/tmp` IS A SYMLINK HERE.** macOS
/// resolves `/tmp` to `/private/tmp`, so a root named one way and a temporary
/// root named the other are the same directory and a textual prefix test
/// misses it. A path that cannot be canonicalised is compared as written -- it
/// does not exist, so there is nothing to resolve and nothing to miss.
pub fn scratch(root: &Path) -> Option<Scratch> {
  let real = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
  let temps: Vec<PathBuf> = system_temp_roots()
    .into_iter()
    .map(|t| std::fs::canonicalize(&t).unwrap_or(t))
    .collect();
  scratch_within(&real, &temps, gitdir_target(root).as_deref())
}

/// A publish that would point the machine at a tree that is about to vanish.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replacement {
  pub existing: PathBuf,
  pub candidate: PathBuf,
  pub why: Scratch,
}

/// PURE. Whether this candidate may replace what the pointer already names.
///
/// **THE SUBJECT IS THE REPLACEMENT, NOT THE ROOT** (vc's ruling, 2026-09-20,
/// on issue `0492`). Refusing every scratch root outright was the first shape
/// of this fix and it was wrong twice over: it would have refused a fresh
/// machine, a CI runner and `gyges`, none of which has anything better to
/// publish; and it made `intent bootstrap` refuse from ANY worktree checkout,
/// which turned a suite's verdict into a function of where the checkout lives.
///
/// So the refusal is narrowed to the thing that actually happened: **a pointer
/// naming a real install, silently replaced by a scratch one.** Everything else
/// publishes as it always did --
///
///   - no pointer, or an empty one: a first install has nothing to lose;
///   - a pointer naming a root that is GONE: it is already broken, and
///     anything that resolves is an improvement;
///   - a pointer already naming a scratch root: the machine is in the state
///     this refusal exists to prevent, and refusing to move would trap it
///     there. This is the clause that lets the 2026-09-19 pointer be repaired
///     by an ordinary `intent bootstrap` from the tree it should have named.
///
/// `existing_exists` is passed rather than read so that both verdicts are
/// reachable without conjuring a real install on disk.
pub fn replacement_refused(
  candidate: &Path,
  candidate_scratch: Option<&Scratch>,
  existing: Option<&Path>,
  existing_scratch: Option<&Scratch>,
  existing_exists: bool,
) -> Option<Replacement> {
  let why = candidate_scratch?;
  let existing = existing?;
  if !existing_exists || existing_scratch.is_some() {
    return None;
  }
  Some(Replacement {
    existing: existing.to_path_buf(),
    candidate: candidate.to_path_buf(),
    why: why.clone(),
  })
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

  // **AFTER `is_install` AND BEFORE THE WRITE.** After, because *not an
  // install* is the more basic fault and must keep its own words; before,
  // because the whole point is that nothing reaches the file -- the pointer a
  // refused publish would overwrite is the one every gate on the machine is
  // currently using, and it must still be there afterwards.
  //
  // The decision itself is [`replacement_refused`], which is pure; everything
  // here is the reading it needs.
  let existing = previous.as_deref().map(Path::new);
  if let Some(refusal) = replacement_refused(
    root,
    scratch(root).as_ref(),
    existing,
    existing.and_then(scratch).as_ref(),
    existing.is_some_and(is_install),
  ) {
    return Err(InstallError::WouldReplaceARealInstall {
      existing: refusal.existing.display().to_string(),
      candidate: refusal.candidate.display().to_string(),
      why: refusal.why.to_string(),
    });
  }

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

/// Where this machine's pre-commit gate resolves, and how that answer stands
/// against the binary asking (issue `0533`).
///
/// **ONE ANSWER, AND EVERY DOOR RENDERS IT** (vc's ruling, 2026-09-23).
/// `intent bootstrap --check`, the `Gate root:` line in `intent info`, and
/// `intent claude upgrade`'s warnings and divergence note each render this
/// value and none re-derives any part of it. Until it existed nothing in Intent
/// answered the question, so a consumer that needed it copied the shim's
/// resolution into its own tooling, a second reading of one file kept in step
/// by hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateResolution {
  /// The file the shim reads. `None` when there is no per-user directory to
  /// hold one, which leaves the shim nothing to read either.
  pub pointer: Option<PathBuf>,
  /// What that file answers today.
  pub state: PointerState,
  /// This binary's own install root, when it has one.
  pub this_binary: Option<PathBuf>,
  /// The pointer names an install other than `this_binary`'s. Compared as
  /// installs, never as strings: on a Homebrew install the pointer names the
  /// `opt` link and [`home`] answers the keg it links (issue `0527`).
  pub divergent: bool,
  /// The pointer names a versioned Homebrew keg, which the next `brew upgrade`
  /// deletes. A `bootstrap` from 3.2.0 or earlier recorded one (issue `0527`).
  pub versioned_keg: bool,
  /// The pointer file exists and names nothing. The gate reads that as absent,
  /// as it reads a missing file, and `bootstrap --check` says which it was
  /// (vc's ruling, 2026-09-23), so the two causes stay apart without a state.
  pub empty_pointer: bool,
}

impl GateResolution {
  /// Whether the gate can run: the pointer names an install AND that install
  /// holds the gate body. The shim's `--where` exits 0 on exactly this and 1
  /// on everything else.
  ///
  /// **RESOLVING IS NOT RUNNING** (issue 0561). This answered `Resolves` alone,
  /// so `bootstrap --check` said OK for an install whose
  /// `lib/templates/hooks/pre-commit.sh` was gone while the shim refused every
  /// commit with its FAILURE 3, naming that file.
  pub fn can_run(&self) -> bool {
    self.gate().is_some_and(|gate| gate.is_file())
  }

  /// The gate body the shim execs, when the pointer names an install.
  pub fn gate(&self) -> Option<PathBuf> {
    match &self.state {
      PointerState::Resolves { root } => Some(gate_script(root)),
      PointerState::Absent | PointerState::Unusable { .. } => None,
    }
  }
}

/// [`GateResolution`] for this machine, against `this_binary`: the root
/// [`home`] resolved, or `None` when it resolved none. A binary outside any
/// install can still say where the gate resolves, because the gate never asks
/// the binary.
pub fn gate_resolution(this_binary: Option<&Path>) -> GateResolution {
  // An unlocatable per-user directory and an absent pointer inside one are the
  // same answer to the question asked -- the shim finds nothing either way, and
  // the remedy is the same.
  let pointer = crate::userstate::home_pointer().ok();
  gate_resolution_at(pointer.as_deref(), this_binary)
}

/// The half with both paths handed in, so every state is drivable against a
/// planted pointer -- the split [`pointer_state_at`] makes for the state alone.
pub fn gate_resolution_at(pointer: Option<&Path>, this_binary: Option<&Path>) -> GateResolution {
  let state = pointer.map_or(PointerState::Absent, pointer_state_at);
  let (divergent, versioned_keg) = match &state {
    PointerState::Resolves { root } => (
      this_binary.is_some_and(|binary| !same_install(root, binary)),
      brew_keg(root).is_some(),
    ),
    PointerState::Absent | PointerState::Unusable { .. } => (false, false),
  };
  let empty_pointer = matches!(state, PointerState::Absent)
    && pointer
      .is_some_and(|p| std::fs::read_to_string(p).is_ok_and(|text| first_line(&text).is_empty()));
  GateResolution {
    pointer: pointer.map(Path::to_path_buf),
    state,
    this_binary: this_binary.map(Path::to_path_buf),
    divergent,
    versioned_keg,
    empty_pointer,
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
  let first = first_line(&text);
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

/// The pointer's one line, as the shim reads it with `head -n 1`, trimmed.
fn first_line(text: &str) -> &str {
  text.lines().next().unwrap_or_default().trim()
}

/// Where a shipped hook script lives, given the install root.
pub fn hook_script(home: &Path, name: &str) -> PathBuf {
  home
    .join(MARKER)
    .join(".claude/scripts")
    .join(format!("{name}.sh"))
}

/// The pre-commit gate body, given the install root: the path the project shim
/// execs, which its `--where` prints as `gate:`.
pub fn gate_script(root: &Path) -> PathBuf {
  root.join(MARKER).join("hooks/pre-commit.sh")
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
/// the expiry), which is why it sources nothing and carries its own `error`.
/// **It does no project discovery and no node lookup** (issue 0609): the
/// `claude start` door resolves both through the binary's own doors and runs
/// it from the project root with the workstream and its display name, so what
/// it keeps is composing the session's context and exec-ing `claude`. It did
/// its own walk-up until 0609, a second discovery that accepted layouts the
/// binary refuses.
pub fn cwi_script(install: &Path) -> PathBuf {
  install.join("intent/plugins/claude/bin/intent_claude_cwi")
}

/// Read a `[intent-source-<key>:...]` marker out of a binary ON DISK.
///
/// **THE ONE READER** (issue `0235`). `intentd`'s web footer uses it to name
/// the `intent` beside it, and `intent daemon status` uses it to name the
/// `intentd` beside it -- which was the half of that pair that had no way to be
/// read. `build-support/source_commit.rs` emits the markers as self-delimiting
/// literals, so the read is: find the opening, stop at the `]`.
///
/// **A FILE IS NOT A RUNNING IMAGE.** This answers what the binary at `path`
/// is now; a process started from that path before a rebuild is a different
/// build, and only asking the process says which (`wire::Op::Build`). `None`
/// means this file could not say, and the caller says so rather than guessing.
pub fn embedded_marker(binary: &Path, key: &str) -> Option<String> {
  let open = format!("[intent-source-{key}:");
  let open = open.as_bytes();
  let bytes = std::fs::read(binary).ok()?;
  let at = bytes.windows(open.len()).position(|w| w == open)? + open.len();
  let end = at + bytes[at..].iter().position(|b| *b == b']')?;
  String::from_utf8(bytes[at..end].to_vec()).ok()
}

/// The manifest an opt or brew tree carries at its root (devbin's
/// `docs/dev-opt-brew.md`, T3). A dev tree has none.
pub const MANIFEST: &str = ".intent-install";

/// The `tool` an Intent manifest names.
const TOOL: &str = "intent";

/// The support paths of the runtime set (T1): what the pair reads and execs
/// beside itself, as git names them. The runtime set is the pair built from one
/// commit plus these.
///
/// **THE ONE HOME.** The packaging reads this list from the pair it names with
/// `intent install --print-runtime-set` (`bin/.devbin/cmd/shared/support.lib`),
/// and keeps no copy of its own: a pair behind HEAD is dirty by T2 and cannot
/// pass the checks that ask, so an older binary's list never decides.
pub const RUNTIME_SET: &[&str] = &[
  "lib/templates",
  "intent/plugins/claude",
  "intent/plugins/agents/plugin.json",
];

/// Which tree the running binary belongs to (T4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tree {
  /// The git checkout, running the pair cargo built in it.
  Dev,
  /// A tree `intent install --prefix` published, its manifest naming Intent.
  Opt,
  /// Homebrew's keg, by its manifest or, for a keg from before the contract,
  /// by its root resolving under the formula's opt link.
  Brew,
  /// None of those, which doctor fails.
  Unknown,
}

impl Tree {
  /// The word the version line prints.
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Dev => "dev",
      Self::Opt => "opt",
      Self::Brew => "brew",
      Self::Unknown => "unknown",
    }
  }
}

impl std::fmt::Display for Tree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

/// What a binary knows about its own build: its name, `CARGO_PKG_VERSION`, and
/// the commit and kind `build-support/source_commit.rs` stamped into it. Each
/// binary passes its own, so the line names the build that is running.
#[derive(Debug, Clone, Copy)]
pub struct Build<'a> {
  pub name: &'a str,
  pub version: &'a str,
  /// `<sha>`, `dirty-<sha>` or `unknown`, as `INTENT_SOURCE_COMMIT` carries it.
  pub commit: &'a str,
  /// `release` or `dev`, the provenance word printed last.
  pub kind: &'a str,
}

/// The running tree, as the one function [`tree_at`] reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeId {
  pub tree: Tree,
  /// Absolute; a brew root is named by its opt link.
  pub root: PathBuf,
  /// The full commit the line names: the manifest's `source_commit` in opt
  /// and brew, the commit the binary was built from in dev. `None` is
  /// `unknown`, and [`TreeId::why`] says why.
  pub sha: Option<String>,
  /// Dev only: the support set differs from the built commit, or the binary
  /// was built from a dirty tree.
  pub dirty: bool,
  /// Dev only: HEAD when this was read, so doctor can say it moved past the
  /// build.
  pub head: Option<String>,
  /// Why the tree or the sha is unknown.
  pub why: Option<String>,
}

/// The running binary's tree. Never fails: a binary that resolves no install
/// still prints a line, naming its tree `unknown` and its own directory.
pub fn tree(commit: &str) -> TreeId {
  match home() {
    Ok(root) => tree_at(&root, commit),
    Err(e) => TreeId {
      tree: Tree::Unknown,
      root: std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf))
        .unwrap_or_default(),
      sha: built_sha(commit),
      dirty: false,
      head: None,
      why: Some(e.to_string()),
    },
  }
}

/// THE TREE FUNCTION (T4): which tree `root` is, in the contract's order --
/// brew when the manifest says `managed_by: brew`, opt when its `tool` is
/// `intent`, dev when the root is a git work tree with no manifest, unknown
/// otherwise. A keg with no manifest answers brew by its opt link (T3 as
/// amended), which is read only after the manifest. `root` is as [`resolve`]
/// returns it, canonical; `commit` is the binary's `INTENT_SOURCE_COMMIT`.
pub fn tree_at(root: &Path, commit: &str) -> TreeId {
  let built = built_sha(commit);
  let id = |tree, root: PathBuf, sha, why| TreeId {
    tree,
    root,
    sha,
    dirty: false,
    head: None,
    why,
  };
  match std::fs::read_to_string(root.join(MANIFEST)) {
    Ok(text) => {
      let header = manifest_header(&text);
      let sha = header
        .get("source_commit")
        .filter(|s| is_sha(s))
        .cloned()
        .or(built);
      if header.get("managed_by").map(String::as_str) == Some("brew") {
        id(Tree::Brew, stable_root(root), sha, None)
      } else if header.get("tool").map(String::as_str) == Some(TOOL) {
        id(Tree::Opt, root.to_path_buf(), sha, None)
      } else {
        let tool = header.get("tool").map_or("no tool", String::as_str);
        id(
          Tree::Unknown,
          root.to_path_buf(),
          sha,
          Some(format!(
            "{} names `{tool}`, not intent, and says it is not managed by brew",
            root.join(MANIFEST).display()
          )),
        )
      }
    }
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
      let stable = stable_root(root);
      if stable != root {
        id(Tree::Brew, stable, built, None)
      } else if root.join(".git").exists() {
        dev_at(root, commit)
      } else {
        id(
          Tree::Unknown,
          root.to_path_buf(),
          built,
          Some(format!(
            "{} has no {MANIFEST} and is not a git work tree",
            root.display()
          )),
        )
      }
    }
    Err(e) => id(
      Tree::Unknown,
      root.to_path_buf(),
      built,
      Some(format!(
        "cannot read {}: {e}",
        root.join(MANIFEST).display()
      )),
    ),
  }
}

/// A dev tree: the sha is the built commit, never HEAD, and `+dirty` follows
/// it while the support set differs from that commit in git (untracked files
/// included, since the dev pair reads the working tree) or the binary was
/// built from a dirty tree. When git cannot answer, the sha is `unknown`.
fn dev_at(root: &Path, commit: &str) -> TreeId {
  let mut id = TreeId {
    tree: Tree::Dev,
    root: root.to_path_buf(),
    sha: None,
    dirty: false,
    head: None,
    why: None,
  };
  let Some(sha) = built_sha(commit) else {
    id.why = Some(format!(
      "this binary names no commit it was built from (`{commit}`)"
    ));
    return id;
  };
  match support_differs(root, &sha) {
    Ok(differs) => {
      id.dirty = differs || commit.starts_with("dirty-");
      id.head = git_out(root, &["rev-parse", "HEAD"]).ok();
      id.sha = Some(sha);
    }
    Err(why) => id.why = Some(why.to_string()),
  }
  id
}

/// What a read of git in a tree could not do. The tree function keeps its
/// text as [`TreeId::why`], and a publish carries it into its refusal.
#[derive(Debug, thiserror::Error)]
enum GitRead {
  #[error("git could not be run in {root}: {source}")]
  Spawn {
    root: String,
    #[source]
    source: std::io::Error,
  },
  #[error("`git {args}` refused: {stderr}")]
  Refused { args: String, stderr: String },
}

/// Whether the support set in `root`'s working tree differs from `sha`.
fn support_differs(root: &Path, sha: &str) -> Result<bool, GitRead> {
  let mut diff = vec!["diff", "--quiet", sha, "--"];
  diff.extend_from_slice(RUNTIME_SET);
  let out = git(root, &diff)?;
  let changed = match out.status.code() {
    Some(0) => false,
    Some(1) => true,
    _ => return Err(git_refusal(&diff, &out)),
  };
  let mut others = vec!["ls-files", "--others", "--exclude-standard", "--"];
  others.extend_from_slice(RUNTIME_SET);
  Ok(changed || !git_out(root, &others)?.is_empty())
}

/// The variables git exports into a hook that name the caller's repository.
const CALLER_GIT_ENV: &[&str] = &[
  "GIT_DIR",
  "GIT_WORK_TREE",
  "GIT_INDEX_FILE",
  "GIT_COMMON_DIR",
  "GIT_PREFIX",
];

/// THE spawn of git into a named tree other than the caller's repository
/// (issue 0645): `git -C <root>` with [`CALLER_GIT_ENV`] removed from the
/// child, so it resolves `root`'s own `.git` and nothing of the caller's.
///
/// **A hook in a linked worktree exports an absolute `GIT_DIR`**, and a child
/// that inherits it reads that repository whatever `-C` says: the tree function
/// read Molt's repository for Intent's commit, called it `bad object`, and
/// doctor refused every gated commit from a worktree in every estate. The main
/// checkout passed only because its `GIT_DIR` is the relative `.git`.
///
/// **The project-tree spawns must NOT use this** (`gitstate`, `critic`'s staged
/// read, `canon`, and doctor's hook reads of the project): they run in the
/// repository the hook belongs to, and the inherited environment is how git
/// finds that worktree's own index, including the temporary one a
/// `commit --only` hands its hook in `GIT_INDEX_FILE`.
fn git(root: &Path, args: &[&str]) -> Result<std::process::Output, GitRead> {
  let mut cmd = std::process::Command::new("git");
  for var in CALLER_GIT_ENV {
    cmd.env_remove(var);
  }
  cmd
    .arg("-C")
    .arg(root)
    .args(args)
    .output()
    .map_err(|source| GitRead::Spawn {
      root: root.display().to_string(),
      source,
    })
}

fn git_out(root: &Path, args: &[&str]) -> Result<String, GitRead> {
  let out = git(root, args)?;
  if !out.status.success() {
    return Err(git_refusal(args, &out));
  }
  Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn git_refusal(args: &[&str], out: &std::process::Output) -> GitRead {
  GitRead::Refused {
    args: args.join(" "),
    stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
  }
}

/// The commit a stamp names, without its `dirty-` marker; `None` for
/// `unknown` or anything else that is not a sha.
fn built_sha(commit: &str) -> Option<String> {
  let sha = commit.strip_prefix("dirty-").unwrap_or(commit);
  is_sha(sha).then(|| sha.to_string())
}

fn is_sha(s: &str) -> bool {
  s.len() >= 12 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

/// A manifest's header (T3): `# key: value` lines, read until the body.
fn manifest_header(text: &str) -> std::collections::BTreeMap<String, String> {
  text
    .lines()
    .map_while(|line| line.strip_prefix("# "))
    .filter_map(|line| line.split_once(": "))
    .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
    .collect()
}

/// THE VERSION LINE (T4), one formatter for both printers:
/// `<name> <version> (<tree>, <sha12>[+dirty]) <kind>`.
///
/// **NO ROOT** (issue 0647, hv's ruling of 2026-10-25, decision 82): where
/// the binary is, `which intent` already says, so the line names the build and
/// nothing else; the tree's root stays in doctor's install-tree finding, whose
/// subject it is.
///
/// **THE PROVENANCE WORD STAYS LAST** so the fields before it read as the
/// contract's line; it is read by no shell (the gate's self-provenance arm and
/// `verify_pair` read the `[intent-source-commit:...]` marker inside the
/// binary instead).
pub fn version_line(build: &Build<'_>, id: &TreeId) -> String {
  let sha = match &id.sha {
    Some(sha) => {
      let short = &sha[..sha.len().min(12)];
      let dirty = if id.dirty && id.tree == Tree::Dev {
        "+dirty"
      } else {
        ""
      };
      format!("{short}{dirty}")
    }
    None => "unknown".to_string(),
  };
  format!(
    "{} {} ({}, {sha}) {}",
    build.name, build.version, id.tree, build.kind
  )
}

/// What doctor says about the running tree: an unknown tree, or a dev tree
/// whose git cannot be read, fails; HEAD past the build is said, not counted.
pub fn tree_findings(id: &TreeId) -> Vec<crate::finding::Finding> {
  use crate::finding::{Finding, FindingClass};
  let at = id.root.display().to_string();
  let why = id.why.clone().unwrap_or_default();
  match (id.tree, &id.sha, &id.head) {
    (Tree::Unknown, _, _) => vec![Finding::new(
      at,
      FindingClass::InstallTreeUnknown,
      format!("this binary's tree is unknown: {why}"),
    )],
    (Tree::Dev, None, _) => vec![Finding::new(
      at,
      FindingClass::InstallTreeUnknown,
      format!("this dev tree's git cannot be read, so the line names no commit: {why}"),
    )],
    (Tree::Dev, Some(sha), Some(head)) if head != sha => vec![Finding::new(
      at,
      FindingClass::Advisory,
      format!(
        "HEAD ({}) has moved past the commit this pair was built from ({}); `bin/devbin build all` builds HEAD",
        &head[..head.len().min(12)],
        &sha[..sha.len().min(12)]
      ),
    )],
    (Tree::Opt | Tree::Brew, _, _) => manifest_findings(id),
    _ => Vec::new(),
  }
}

/// The keys every manifest header carries (T3), in the order Intent writes
/// them. A manifest missing one is in an older form.
const MANIFEST_KEYS: [&str; 4] = ["tool", "version", "source_commit", "managed_by"];

/// Where the opt tree goes when `--prefix` is not given, declared by the
/// sleeve and read by `intent install` alone (T1). [`publish`] spells the read
/// with the literal name, so `no_intent_home.rs` sees it in its one file.
pub const PREFIX_VAR: &str = "INTENT_INSTALL_PREFIX";

/// Who manages a published tree, the manifest's `managed_by`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedBy {
  Intent,
  Brew,
}

impl ManagedBy {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Intent => TOOL,
      Self::Brew => "brew",
    }
  }
}

/// What `intent install` published.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Publication {
  pub prefix: PathBuf,
  /// An opt tree Intent published was there and `--force` replaced it.
  pub replaced: bool,
  /// Every runtime file, relative to the prefix, as the manifest lists them.
  pub files: Vec<String>,
}

/// `intent install [--prefix <dir>] [--force] [--managed-by brew]` (T2): the
/// running pair and the support paths git names, published into the keg's
/// layout with a manifest. The prefix is `prefix`, else [`PREFIX_VAR`]; the
/// pair is the running binary's own directory.
pub fn publish(
  build: &Build<'_>,
  prefix: Option<&Path>,
  force: bool,
  managed_by: ManagedBy,
) -> Result<Publication, InstallError> {
  let root = home()?;
  let exe = std::env::current_exe().map_err(InstallError::Exe)?;
  let pair = canonical(&exe)
    .parent()
    .map(Path::to_path_buf)
    .unwrap_or_default();
  let prefix = prefix
    .map(Path::to_path_buf)
    .or_else(|| std::env::var_os("INTENT_INSTALL_PREFIX").map(PathBuf::from));
  publish_at(&root, &pair, build, prefix.as_deref(), force, managed_by)
}

/// The pure-in-its-inputs half of [`publish`]: every refusal comes before the
/// first write, the tree is staged beside the prefix and swapped in by rename,
/// and a prefix that was there is moved back when the swap fails.
pub fn publish_at(
  root: &Path,
  pair: &Path,
  build: &Build<'_>,
  prefix: Option<&Path>,
  force: bool,
  managed_by: ManagedBy,
) -> Result<Publication, InstallError> {
  let id = tree_at(root, build.commit);
  if id.tree != Tree::Dev {
    return Err(InstallError::NotDev {
      tree: id.tree,
      root: id.root.display().to_string(),
    });
  }
  let dirty = |why: String| InstallError::RuntimeSetDirty {
    root: root.display().to_string(),
    why,
  };
  let Some(sha) = id.sha.clone() else {
    return Err(dirty(id.why.clone().unwrap_or_default()));
  };
  if id.dirty {
    return Err(dirty(
      "a support path differs from the commit the pair was built from, or the pair was built from a dirty tree".to_string(),
    ));
  }
  if id.head.as_deref() != Some(sha.as_str()) {
    return Err(dirty(format!(
      "HEAD ({}) has moved past the commit the pair was built from ({})",
      id.head.as_deref().unwrap_or("unreadable"),
      &sha[..12]
    )));
  }
  // The prefix before the pair: its absence is the same answer from every
  // route, where the pair is wherever the running binary happens to be.
  let prefix = prefix.ok_or(InstallError::NoPrefix)?;
  let daemon = pair.join("intentd");
  let daemon_commit = embedded_marker(&daemon, "commit").unwrap_or_else(|| "no commit".to_string());
  if !pair.join("intent").is_file() || daemon_commit != build.commit {
    return Err(InstallError::SplitPair {
      dir: pair.display().to_string(),
      intent: build.commit.to_string(),
      intentd: daemon_commit,
    });
  }
  let refused = |why: String| InstallError::PrefixRefused {
    prefix: prefix.display().to_string(),
    why,
  };
  if !prefix.is_absolute() || prefix.parent().is_none() {
    return Err(refused("it must be an absolute path below /".to_string()));
  }
  let landed = landed_path(prefix);
  if landed.starts_with(root) || root.starts_with(&landed) {
    return Err(refused(format!(
      "it overlaps the dev tree {} it would publish",
      root.display()
    )));
  }
  let replaced = match prefix_fault(prefix, force, managed_by) {
    Ok(replaced) => replaced,
    Err(why) => return Err(refused(why.to_string())),
  };
  let mut files = git_files(root).map_err(|e| dirty(e.to_string()))?;
  if files.is_empty() {
    return Err(dirty("git names no file in the runtime set".to_string()));
  }
  files.extend(["bin/intent".to_string(), "bin/intentd".to_string()]);
  files.sort();
  let failed = |why: String| InstallError::Publish {
    prefix: prefix.display().to_string(),
    why,
  };
  let parent = prefix.parent().unwrap_or(prefix);
  std::fs::create_dir_all(parent)
    .map_err(|e| failed(format!("{} could not be made: {e}", parent.display())))?;
  let stage = tempfile::Builder::new()
    .prefix(".intent-install.")
    .tempdir_in(parent)
    .map_err(|e| {
      failed(format!(
        "no staging directory could be made in {}: {e}",
        parent.display()
      ))
    })?;
  for file in &files {
    let from = match file.strip_prefix("bin/") {
      Some(name) if file == "bin/intent" || file == "bin/intentd" => pair.join(name),
      _ => root.join(file),
    };
    let to = stage.path().join(file);
    if let Some(dir) = to.parent() {
      std::fs::create_dir_all(dir).map_err(|e| failed(format!("{}: {e}", dir.display())))?;
    }
    std::fs::copy(&from, &to)
      .map_err(|e| failed(format!("{} could not be copied: {e}", from.display())))?;
  }
  let manifest = render_manifest(stage.path(), build.version, &sha, managed_by, &files)
    .map_err(|e| failed(format!("the manifest could not be written: {e}")))?;
  std::fs::write(stage.path().join(MANIFEST), manifest)
    .map_err(|e| failed(format!("the manifest could not be written: {e}")))?;
  swap_in(stage, prefix).map_err(|e| failed(e.to_string()))?;
  Ok(Publication {
    prefix: prefix.to_path_buf(),
    replaced,
    files,
  })
}

/// Where a path lands on disk: its nearest existing ancestor canonicalised,
/// with the rest appended, so a prefix that does not exist yet still compares
/// against the dev tree by where it would be.
fn landed_path(path: &Path) -> PathBuf {
  let mut rest = Vec::new();
  let mut at = path;
  loop {
    if let Ok(real) = std::fs::canonicalize(at) {
      return rest.iter().rev().fold(real, |p, c| p.join(c));
    }
    match (at.parent(), at.file_name()) {
      (Some(up), Some(name)) => {
        rest.push(name.to_os_string());
        at = up;
      }
      _ => return path.to_path_buf(),
    }
  }
}

/// Why a prefix cannot take a publish; the refusal carries its text.
#[derive(Debug, thiserror::Error)]
enum PrefixFault {
  #[error("it is not a directory")]
  NotADirectory,
  #[error("its {MANIFEST} says tool: {tool}, not intent, so it is left as it is")]
  OtherTool { tool: String },
  #[error("it is Homebrew's install of intent; upgrade it with `brew upgrade intent`")]
  Brew,
  #[error("it already holds an install of intent; pass --force to replace it")]
  NeedsForce,
  #[error(
    "its {MANIFEST} says managed_by: {held}, which is neither intent nor brew, so it is left as it is"
  )]
  OtherManager { held: String },
  #[error(
    "its {MANIFEST} names no managed_by, so it is no install Intent can replace, and it is left as it is"
  )]
  NoManager,
  #[error("it holds files Intent did not write ({held}), so it is left as it is")]
  Foreign { held: String },
  #[error("{what} cannot be read: {source}")]
  Unreadable {
    what: String,
    #[source]
    source: std::io::Error,
  },
}

/// Why `prefix` cannot take this publish, or whether it replaces a tree.
/// A missing or empty directory is fresh; an opt tree Intent wrote is replaced
/// only with `--force`; a tree Homebrew manages only by Homebrew; anything
/// else is refused, `--force` or not, since a replace removes what was there.
fn prefix_fault(prefix: &Path, force: bool, managed_by: ManagedBy) -> Result<bool, PrefixFault> {
  if prefix.exists() && !prefix.is_dir() {
    return Err(PrefixFault::NotADirectory);
  }
  if !prefix.is_dir() {
    return Ok(false);
  }
  match std::fs::read_to_string(prefix.join(MANIFEST)) {
    Ok(text) => {
      let header = manifest_header(&text);
      let held = header.get("managed_by").map(String::as_str);
      match header.get("tool").map(String::as_str) {
        Some(TOOL) => {}
        tool => {
          return Err(PrefixFault::OtherTool {
            tool: tool.unwrap_or("nothing").to_string(),
          });
        }
      }
      match held {
        Some("brew") if managed_by != ManagedBy::Brew => Err(PrefixFault::Brew),
        Some(TOOL | "brew") if !force => Err(PrefixFault::NeedsForce),
        Some(TOOL | "brew") => Ok(true),
        Some(other) => Err(PrefixFault::OtherManager {
          held: other.to_string(),
        }),
        None => Err(PrefixFault::NoManager),
      }
    }
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
      let held: Vec<String> = std::fs::read_dir(prefix)
        .map_err(|source| PrefixFault::Unreadable {
          what: "it".to_string(),
          source,
        })?
        .filter_map(Result::ok)
        .take(5)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
      if held.is_empty() {
        Ok(false)
      } else {
        Err(PrefixFault::Foreign {
          held: held.join(" "),
        })
      }
    }
    Err(source) => Err(PrefixFault::Unreadable {
      what: format!("its {MANIFEST}"),
      source,
    }),
  }
}

/// The support files git names under [`RUNTIME_SET`], relative to `root`.
fn git_files(root: &Path) -> Result<Vec<String>, GitRead> {
  let mut args = vec!["ls-files", "--"];
  args.extend_from_slice(RUNTIME_SET);
  Ok(
    git_out(root, &args)?
      .lines()
      .filter(|line| !line.is_empty())
      .map(str::to_string)
      .collect(),
  )
}

/// T3's manifest for the files staged under `tree`: the header, then one
/// `shasum -a 256 -c` line per file, in `files`' order (sorted by the caller,
/// bytewise, as `LC_ALL=C sort` sorts the paths).
fn render_manifest(
  tree: &Path,
  version: &str,
  sha: &str,
  managed_by: ManagedBy,
  files: &[String],
) -> std::io::Result<String> {
  let mut out = String::new();
  for (key, value) in MANIFEST_KEYS
    .iter()
    .zip([TOOL, version, sha, managed_by.as_str()])
  {
    out.push_str(&format!("# {key}: {value}\n"));
  }
  for file in files {
    let bytes = std::fs::read(tree.join(file))?;
    out.push_str(&format!("{}  {file}\n", crate::sync::sha256_of(&bytes)));
  }
  Ok(out)
}

/// Why the swap failed, and where anything it could not put back is.
#[derive(Debug, thiserror::Error)]
enum SwapFailed {
  #[error("{prefix} could not be moved aside, so it is unchanged: {source}{left}")]
  Aside {
    prefix: String,
    left: Left,
    #[source]
    source: std::io::Error,
  },
  #[error("the new tree could not be moved to {prefix}: {source}{left}")]
  In {
    prefix: String,
    left: Left,
    #[source]
    source: std::io::Error,
  },
  #[error(
    "the new tree could not be moved to {prefix} ({source}), nor the old one back; it is at {old}{left}"
  )]
  InAndBack {
    prefix: String,
    old: String,
    left: Left,
    #[source]
    source: std::io::Error,
  },
  #[error("the new tree is in place and the old one could not be removed from {old}: {source}")]
  RemoveOld {
    old: String,
    #[source]
    source: std::io::Error,
  },
}

/// A staging directory a failed swap could not remove, said rather than
/// dropped: empty when the cleanup worked.
#[derive(Debug)]
struct Left(Option<PathBuf>);

impl std::fmt::Display for Left {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.0 {
      Some(stage) => write!(f, "; the staging directory is left at {}", stage.display()),
      None => Ok(()),
    }
  }
}

/// Remove the staging directory after a failure, saying where it is left
/// when that fails too.
fn clean_up(staged: &Path) -> Left {
  Left(
    std::fs::remove_dir_all(staged)
      .err()
      .map(|_| staged.to_path_buf()),
  )
}

/// Swap the staged tree in at `prefix`, moving a tree that was there aside
/// first and back if the swap fails.
fn swap_in(stage: tempfile::TempDir, prefix: &Path) -> Result<(), SwapFailed> {
  let staged = stage.keep();
  let old = PathBuf::from(format!("{}.old", staged.display()));
  let had = prefix.is_dir();
  let empty = had
    && std::fs::read_dir(prefix)
      .map(|mut d| d.next().is_none())
      .unwrap_or(false);
  if had {
    let aside = if empty {
      std::fs::remove_dir(prefix)
    } else {
      std::fs::rename(prefix, &old)
    };
    if let Err(source) = aside {
      return Err(SwapFailed::Aside {
        prefix: prefix.display().to_string(),
        left: clean_up(&staged),
        source,
      });
    }
  }
  if let Err(source) = std::fs::rename(&staged, prefix) {
    let left = clean_up(&staged);
    if had && !empty && std::fs::rename(&old, prefix).is_err() {
      return Err(SwapFailed::InAndBack {
        prefix: prefix.display().to_string(),
        old: old.display().to_string(),
        left,
        source,
      });
    }
    return Err(SwapFailed::In {
      prefix: prefix.display().to_string(),
      left,
      source,
    });
  }
  if had && !empty {
    std::fs::remove_dir_all(&old).map_err(|source| SwapFailed::RemoveOld {
      old: old.display().to_string(),
      source,
    })?;
  }
  Ok(())
}

/// Doctor's manifest arms in opt and brew (T3): every file the body lists is
/// checked against its sum, and every runtime file the body does not list is
/// named; a manifest in an older form is named with its remedy, a failure in
/// opt and a note in brew, where a keg from before the contract has none.
fn manifest_findings(id: &TreeId) -> Vec<crate::finding::Finding> {
  use crate::finding::{Finding, FindingClass};
  let path = id.root.join(MANIFEST);
  let at = path.display().to_string();
  let text = std::fs::read_to_string(&path).ok();
  let header = text.as_deref().map(manifest_header).unwrap_or_default();
  let current = MANIFEST_KEYS.iter().all(|k| header.contains_key(*k));
  match (id.tree, current) {
    (Tree::Brew, false) => {
      return vec![Finding::new(
        at,
        FindingClass::Advisory,
        "this keg's manifest is in an older form, or it has none: only a release whose formula writes the new form fixes it, with `brew upgrade intent`",
      )];
    }
    (Tree::Opt, false) => {
      return vec![Finding::new(
        at,
        FindingClass::InstallManifestOld,
        format!(
          "the manifest is in an older form, missing {}",
          MANIFEST_KEYS
            .iter()
            .filter(|k| !header.contains_key(**k))
            .copied()
            .collect::<Vec<_>>()
            .join(", ")
        ),
      )];
    }
    _ => {}
  }
  let mut listed = std::collections::BTreeSet::new();
  let mut out = Vec::new();
  for line in text.as_deref().unwrap_or_default().lines() {
    let Some((sum, file)) = line.split_once("  ") else {
      continue;
    };
    if line.starts_with('#') || sum.len() != 64 {
      continue;
    }
    listed.insert(file.to_string());
    let drift = match std::fs::read(id.root.join(file)) {
      Err(_) => Some("is missing"),
      Ok(bytes) if crate::sync::sha256_of(&bytes) != sum => Some("has changed"),
      Ok(_) => None,
    };
    if let Some(drift) = drift {
      out.push(Finding::new(
        id.root.join(file).display().to_string(),
        FindingClass::InstallManifestDrift,
        format!("{file} {drift} since the tree was published"),
      ));
    }
  }
  let mut present = Vec::new();
  for top in RUNTIME_SET.iter().copied().chain(["bin"]) {
    walk_files(&id.root, &id.root.join(top), &mut present);
  }
  for file in present.into_iter().filter(|f| !listed.contains(f)) {
    out.push(Finding::new(
      id.root.join(&file).display().to_string(),
      FindingClass::InstallManifestDrift,
      format!("{file} is not in the manifest"),
    ));
  }
  out
}

/// Every file at or under `at`, relative to `root`.
fn walk_files(root: &Path, at: &Path, out: &mut Vec<String>) {
  if at.is_file() {
    if let Ok(rel) = at.strip_prefix(root) {
      out.push(rel.to_string_lossy().into_owned());
    }
    return;
  }
  let Ok(entries) = std::fs::read_dir(at) else {
    return;
  };
  for entry in entries.filter_map(Result::ok) {
    walk_files(root, &entry.path(), out);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn an_embedded_marker_is_read_to_its_bracket_and_absence_is_none() {
    let dir = tempfile::tempdir().expect("tempdir");
    let bin = dir.path().join("fake");
    // Literals pack with no separator in rodata, which is why the bracket is
    // the delimiter: the byte after it belongs to something else.
    std::fs::write(
      &bin,
      b"\x00junk[intent-source-commit:dirty-abc123]unsafe\x00",
    )
    .unwrap();
    assert_eq!(
      embedded_marker(&bin, "commit"),
      Some("dirty-abc123".to_string())
    );
    assert_eq!(embedded_marker(&bin, "version"), None);
    assert_eq!(embedded_marker(&dir.path().join("absent"), "commit"), None);
  }

  /// A tree shaped like an install, plus one that is not.
  fn install_at(root: &Path) {
    std::fs::create_dir_all(root.join(MARKER).join(".claude/scripts")).unwrap();
    std::fs::create_dir_all(root.join(MARKER).join("hooks")).unwrap();
    std::fs::write(gate_script(root), "#!/usr/bin/env bash\n").unwrap();
  }

  /// **THE CLASSIFIER, BOTH WAYS, WITH NO FILESYSTEM** (issue `0492`).
  ///
  /// A linked worktree and a submodule BOTH have a `.git` file, so the
  /// submodule row is the control that makes the `worktrees` component
  /// load-bearing rather than incidental: without it the classifier would
  /// refuse a legitimate root and name a worktree that does not exist.
  #[test]
  fn a_root_is_scratch_for_being_a_linked_worktree_or_living_in_the_temp_dir() {
    let temps = vec![PathBuf::from("/scratchtmp")];

    assert_eq!(
      scratch_within(
        Path::new("/home/me/prj/Intent"),
        &temps,
        Some("/home/me/prj/Intent/.git/worktrees/wt-0481base"),
      ),
      Some(Scratch::LinkedWorktree {
        gitdir: "/home/me/prj/Intent/.git/worktrees/wt-0481base".to_string(),
      })
    );

    // The control: a submodule's `.git` is a file too, and it is NOT scratch.
    assert_eq!(
      scratch_within(
        Path::new("/home/me/prj/Super/sub"),
        &temps,
        Some("/home/me/prj/Super/.git/modules/sub"),
      ),
      None
    );

    assert_eq!(
      scratch_within(Path::new("/scratchtmp/build/Intent"), &temps, None),
      Some(Scratch::UnderSystemTemp {
        temp: PathBuf::from("/scratchtmp"),
      })
    );

    // An ordinary checkout: `.git` is a directory, so no gitdir line, and it
    // lives nowhere temporary.
    assert_eq!(
      scratch_within(Path::new("/home/me/prj/Intent"), &temps, None),
      None
    );
  }

  /// **THE DECISION, DRIVEN TO BOTH VERDICTS AND TO EVERY ALLOWING CLAUSE.**
  ///
  /// The refusing row is the 2026-09-19 incident in one line: a real install
  /// replaced by a scratch worktree. The four allowing rows are the states that
  /// MUST keep publishing -- a fresh machine, a pointer whose root is gone, a
  /// machine already pointed at scratch (which is what makes the incident
  /// repairable by an ordinary `bootstrap`), and a candidate that is not
  /// scratch at all.
  #[test]
  fn only_a_scratch_root_replacing_a_live_real_one_is_refused() {
    let worktree = Scratch::LinkedWorktree {
      gitdir: "/repo/.git/worktrees/wt".to_string(),
    };
    let candidate = Path::new("/tmp/wt");
    let real = Path::new("/home/me/prj/Intent");

    // REFUSED: the incident.
    assert_eq!(
      replacement_refused(candidate, Some(&worktree), Some(real), None, true),
      Some(Replacement {
        existing: real.to_path_buf(),
        candidate: candidate.to_path_buf(),
        why: worktree.clone(),
      })
    );

    // A fresh machine: no pointer at all.
    assert_eq!(
      replacement_refused(candidate, Some(&worktree), None, None, false),
      None
    );
    // The pointer names a root that is gone: already broken.
    assert_eq!(
      replacement_refused(candidate, Some(&worktree), Some(real), None, false),
      None
    );
    // Already pointed at scratch: refusing here would trap the machine in the
    // very state this refusal exists to prevent.
    assert_eq!(
      replacement_refused(
        candidate,
        Some(&worktree),
        Some(real),
        Some(&worktree),
        true
      ),
      None
    );
    // The candidate is an ordinary checkout: nothing to refuse.
    assert_eq!(
      replacement_refused(real, None, Some(real), None, true),
      None
    );
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

  /// A planted Homebrew layout, as `brew install` leaves one: the keg at
  /// `<prefix>/Cellar/intent/<version>/libexec` carrying the marker and the
  /// binary, `<prefix>/bin/intent` linked into it, and `<prefix>/opt/intent`
  /// linked to the keg. Planting a second version moves both links to it, which
  /// is `brew link`'s half of an upgrade.
  fn plant_keg(prefix: &Path, version: &str) {
    let libexec = prefix
      .join("Cellar")
      .join(BREW_FORMULA)
      .join(version)
      .join("libexec");
    install_at(&libexec);
    std::fs::create_dir_all(libexec.join("bin")).unwrap();
    std::fs::write(libexec.join("bin/intent"), b"").unwrap();
    link(
      prefix,
      "bin/intent",
      &format!("../Cellar/intent/{version}/libexec/bin/intent"),
    );
    link(prefix, "opt/intent", &format!("../Cellar/intent/{version}"));
  }

  /// Point `<prefix>/<name>` at `target`, replacing a link already there.
  fn link(prefix: &Path, name: &str, target: &str) {
    let at = prefix.join(name);
    std::fs::create_dir_all(at.parent().unwrap()).unwrap();
    if at.symlink_metadata().is_ok() {
      std::fs::remove_file(&at).unwrap();
    }
    std::os::unix::fs::symlink(target, &at).unwrap();
  }

  /// **ISSUE `0527`: ON A BREW LAYOUT THE POINTER NAMES THE `opt` LINK.**
  /// Resolved through the `bin` link, as a user's `intent` is, the walk finds
  /// the versioned keg, and what is recorded is `opt`'s path to that keg.
  #[test]
  fn a_brew_keg_is_recorded_by_its_opt_link() {
    let dir = tmp("brew");
    let prefix = canonical(dir.path());
    plant_keg(&prefix, "9.9.9");

    let root = resolve(&prefix.join("bin/intent")).unwrap();
    assert_eq!(root, prefix.join("Cellar/intent/9.9.9/libexec"));
    let recorded = stable_root(&root);
    assert_eq!(recorded, prefix.join("opt/intent/libexec"));

    let pointer = prefix.join("share/intent/home");
    assert_eq!(
      publish_home_at(&recorded, &pointer).unwrap(),
      Published::Written {
        root: recorded.clone()
      }
    );
  }

  /// **ISSUE `0527`: AN UPGRADE LEAVES THE POINTER WORKING.** The upgrade is
  /// driven as brew performs it: a new keg, the links moved to it, and the old
  /// keg deleted by the cleanup. The pointer an older bootstrap wrote is the
  /// control -- it names the versioned keg, so the same upgrade breaks it.
  #[test]
  fn after_an_upgrade_the_recorded_path_is_unchanged_and_still_an_install() {
    let dir = tmp("brew-upgrade");
    let prefix = canonical(dir.path());
    plant_keg(&prefix, "9.9.9");
    let pointer = prefix.join("share/intent/home");
    let before = stable_root(&resolve(&prefix.join("bin/intent")).unwrap());
    publish_home_at(&before, &pointer).unwrap();
    let old = prefix.join("Cellar/intent/9.9.9/libexec");
    let versioned = prefix.join("share/intent/versioned");
    std::fs::write(&versioned, format!("{}\n", old.display())).unwrap();

    plant_keg(&prefix, "9.9.10");
    std::fs::remove_dir_all(prefix.join("Cellar/intent/9.9.9")).unwrap();

    assert_eq!(
      pointer_state_at(&versioned),
      PointerState::Unusable {
        root: old.display().to_string()
      },
      "the control: a pointer naming the versioned keg stops resolving at the upgrade"
    );
    let after = stable_root(&resolve(&prefix.join("bin/intent")).unwrap());
    assert_eq!(after, before);
    assert_eq!(
      publish_home_at(&after, &pointer).unwrap(),
      Published::Unchanged {
        root: after.clone()
      }
    );
    assert_eq!(
      pointer_state_at(&pointer),
      PointerState::Resolves { root: after }
    );
  }

  /// **ISSUE `0527`: EVERY OTHER ROOT IS RECORDED EXACTLY AS IT IS.** A source
  /// checkout; a keg run directly while `opt` links another version, which must
  /// not be recorded as the version it is not; a keg with no `opt` link; and
  /// another formula's keg.
  #[test]
  fn a_root_that_opt_does_not_resolve_to_is_recorded_as_it_is() {
    let dir = tmp("not-brew");
    let base = canonical(dir.path());

    let checkout = base.join("src/intent");
    install_at(&checkout);
    assert_eq!(stable_root(&checkout), checkout);

    plant_keg(&base, "9.9.9");
    plant_keg(&base, "9.9.10");
    let older = base.join("Cellar/intent/9.9.9/libexec");
    assert_eq!(stable_root(&older), older, "opt links 9.9.10, not 9.9.9");

    let unlinked = base.join("unlinked/Cellar/intent/1.0.0/libexec");
    install_at(&unlinked);
    assert_eq!(stable_root(&unlinked), unlinked, "no opt link at all");

    let other = base.join("Cellar/other/1.0.0/libexec");
    install_at(&other);
    link(&base, "opt/other", "../Cellar/other/1.0.0");
    assert_eq!(stable_root(&other), other, "another formula's keg");
  }

  /// The `opt` path and the keg it links are one install, and two kegs are two:
  /// the comparison the CLI's divergence note makes (issue `0527`).
  #[test]
  fn two_spellings_of_one_install_are_the_same_install() {
    let dir = tmp("same");
    let prefix = canonical(dir.path());
    plant_keg(&prefix, "9.9.9");
    let keg = prefix.join("Cellar/intent/9.9.9/libexec");
    assert!(same_install(&prefix.join("opt/intent/libexec"), &keg));

    plant_keg(&prefix, "9.9.10");
    assert!(!same_install(&prefix.join("opt/intent/libexec"), &keg));
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

  /// **THE TWO STATES THE GATE CANNOT RUN IN, AND THE ONE WITH NOWHERE TO LOOK**
  /// (issue `0533`). Each is `can_run` false, which is `--where`'s rc 1, and
  /// none has a gate to name or a root to compare.
  #[test]
  fn a_gate_that_cannot_run_is_said_to_and_names_no_gate() {
    let dir = tmp("gate-cannot");
    let binary = dir.path().join("binary");
    install_at(&binary);
    let empty = dir.path().join("empty");
    std::fs::write(&empty, "\n").unwrap();
    let elsewhere = dir.path().join("not-an-install");
    std::fs::create_dir_all(&elsewhere).unwrap();
    let unusable = dir.path().join("unusable");
    std::fs::write(&unusable, format!("{}\n", elsewhere.display())).unwrap();

    for (pointer, state, empty_pointer) in [
      (Some(dir.path().join("absent")), PointerState::Absent, false),
      (Some(empty), PointerState::Absent, true),
      (
        Some(unusable),
        PointerState::Unusable {
          root: elsewhere.display().to_string(),
        },
        false,
      ),
      (None, PointerState::Absent, false),
    ] {
      let gate = gate_resolution_at(pointer.as_deref(), Some(binary.as_path()));
      assert_eq!(gate.state, state, "{pointer:?}");
      assert_eq!(gate.empty_pointer, empty_pointer, "{pointer:?}");
      assert_eq!(gate.pointer, pointer);
      assert!(!gate.can_run(), "{pointer:?}");
      assert_eq!(gate.gate(), None, "{pointer:?}");
      assert!(!gate.divergent && !gate.versioned_keg, "{gate:?}");
    }
  }

  /// **AN INSTALL WITHOUT ITS GATE BODY RESOLVES AND CANNOT RUN** (issue
  /// `0561`). The shim refuses every commit in this state with its FAILURE 3,
  /// so `can_run` answering from the pointer alone said OK for a gate that
  /// never runs. The control is the same install with the body restored.
  #[test]
  fn an_install_without_its_gate_body_resolves_and_cannot_run() {
    let dir = tmp("gate-no-body");
    let base = canonical(dir.path());
    let root = base.join("root");
    install_at(&root);
    std::fs::remove_file(gate_script(&root)).unwrap();
    let pointer = base.join("home");
    std::fs::write(&pointer, format!("{}\n", root.display())).unwrap();

    let gate = gate_resolution_at(Some(pointer.as_path()), Some(root.as_path()));
    assert!(
      matches!(gate.state, PointerState::Resolves { .. }),
      "{gate:?}"
    );
    assert!(
      !gate.can_run(),
      "no gate body, and can_run said yes: {gate:?}"
    );

    install_at(&root);
    let gate = gate_resolution_at(Some(pointer.as_path()), Some(root.as_path()));
    assert!(
      gate.can_run(),
      "the control: the body is back and can_run said no: {gate:?}"
    );
  }

  /// **ONE INSTALL, TWO SPELLINGS, NO NOTE; TWO INSTALLS, THE NOTE** (issue
  /// `0533`). The control for the divergent arm is the same pointer read by a
  /// binary of the same install through a link, which a textual comparison
  /// would call a second install.
  #[test]
  fn a_gate_in_another_install_runs_and_is_divergent() {
    let dir = tmp("gate-divergent");
    let base = canonical(dir.path());
    let root = base.join("root");
    install_at(&root);
    let other = base.join("other");
    install_at(&other);
    let alias = base.join("alias");
    std::os::unix::fs::symlink(&root, &alias).unwrap();
    let pointer = base.join("home");
    std::fs::write(&pointer, format!("{}\n", root.display())).unwrap();

    let same = gate_resolution_at(Some(pointer.as_path()), Some(alias.as_path()));
    assert!(same.can_run());
    assert!(!same.divergent, "one install, two spellings: {same:?}");
    assert_eq!(
      same.gate(),
      Some(root.join("lib/templates/hooks/pre-commit.sh"))
    );

    let divergent = gate_resolution_at(Some(pointer.as_path()), Some(other.as_path()));
    assert!(
      divergent.can_run(),
      "a different install still runs the gate"
    );
    assert!(divergent.divergent, "{divergent:?}");
    assert_eq!(divergent.this_binary, Some(other));

    let unknown = gate_resolution_at(Some(pointer.as_path()), None);
    assert!(unknown.can_run());
    assert!(
      !unknown.divergent,
      "a binary with no install has nothing to differ from: {unknown:?}"
    );
  }

  /// **A POINTER NAMING A VERSIONED KEG RUNS TODAY AND IS NOTED** (issues
  /// `0527` and `0533`). It names a directory `brew upgrade` deletes, whether
  /// `opt` still links it or has moved on. The control is `opt`'s spelling of
  /// the same keg, which resolves to it and must not be noted.
  #[test]
  fn a_pointer_naming_a_versioned_keg_is_noted_and_its_opt_link_is_not() {
    let dir = tmp("gate-keg");
    let prefix = canonical(dir.path());
    plant_keg(&prefix, "9.9.9");
    let keg = prefix.join("Cellar/intent/9.9.9/libexec");
    let binary = resolve(&prefix.join("bin/intent")).unwrap();
    let pointer = prefix.join("share/intent/home");

    std::fs::create_dir_all(pointer.parent().unwrap()).unwrap();
    std::fs::write(&pointer, format!("{}\n", keg.display())).unwrap();
    let versioned = gate_resolution_at(Some(pointer.as_path()), Some(binary.as_path()));
    assert!(versioned.can_run(), "the keg is there today: {versioned:?}");
    assert!(versioned.versioned_keg, "{versioned:?}");
    assert!(!versioned.divergent, "{versioned:?}");

    let opt = stable_root(&binary);
    std::fs::write(&pointer, format!("{}\n", opt.display())).unwrap();
    let stable = gate_resolution_at(Some(pointer.as_path()), Some(binary.as_path()));
    assert!(stable.can_run());
    assert!(
      !stable.versioned_keg,
      "the opt link survives an upgrade: {stable:?}"
    );
    assert!(!stable.divergent, "{stable:?}");

    plant_keg(&prefix, "9.9.10");
    std::fs::write(&pointer, format!("{}\n", keg.display())).unwrap();
    let behind = gate_resolution_at(Some(pointer.as_path()), Some(binary.as_path()));
    assert!(behind.versioned_keg, "opt has moved on: {behind:?}");
  }
}
