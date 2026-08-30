//! The payloads the `claude` plugin installs into Claude Code.
//!
//! **THE PLUGIN IS THE UNIT, NOT THE PAYLOAD KIND** (hv, 2026-08-30). `claude`
//! is an Intent plugin; `skills/` and `subagents/` are two things it ships, and
//! keeping an operator's Claude Code config matching Intent's canon is ONE
//! concern. This module was `skills.rs` and served one kind; it now serves both
//! through [`Kind`], which is the whole of the difference between them.
//!
//! **THE GENERALISATION COST FOUR METHODS AND CHANGED NO BEHAVIOUR.** Every
//! type here -- [`Entry`], [`Manifest`], [`Outcome`], [`Step`], [`Report`] --
//! already mentioned nothing about skills; only the canon path, the marker
//! filename, the target directory and the tree-versus-file shape ever did.
//! `skills_sync.rs` passed 25/25 across the change, which is the claim that
//! matters: the kind that already worked was not touched to make room.
//!
//! **AND IT IS ONE HOME BECAUSE THE ALTERNATIVE WAS TWO.** v3 had this module
//! and no subagents at all; the obvious way to close that gap was a second
//! module carrying a second copy of the manifest, the checksums and the
//! lifecycle. Two homes agree on the day they are written
//! (`IN-AG-HIGHLANDER-001`), and at a tag nobody has room for the second fix.
//!
//! AC-07.3. v2's implementation was `intent_claude_skills` and
//! `intent_claude_subagents` over a shared `claude_plugin_helpers.sh`. **v2's
//! decomposition is not why this one looks as it does** -- hv's ruling is that
//! parity here is FUNCTIONAL, so what v2 did internally is not a constraint on
//! v3, and the three ruled corrections below are corrections to BEHAVIOUR. Each
//! one is named at the code that carries it.
//!
//! **ROOTS COME FROM THE INSTALL, NEVER THE ENVIRONMENT** -- AC-11.3, and the
//! precedent `rules.rs` set for the same reason. The assets are VERSIONED, so a
//! machine mid-rollout has a v2 tree and a v3 tree and a `$INTENT_HOME` left
//! over from the first; reading it would make a v3 binary install **v2's
//! skills**. This is not a style preference here, it is the defect: skills
//! edited in a live checkout were being sourced from a FROZEN one because the
//! `intent` on `PATH` resolved its own root from its own location. Resolving
//! from `install::home()` makes that unconstructible -- there is no input that
//! can point this binary at another tree's skills.
//!
//! **EVERY AMBIENT PATH IS A PARAMETER, AND THAT IS A BLOCKED SEAM RATHER THAN
//! A STYLE.** The manifest lives under `~/.intent/skills/` and the target under
//! `~/.claude/skills/`, so this command needs `$HOME` -- the FILENAME is
//! [`MANIFEST_RELATIVE`]'s to state and is deliberately not repeated here,
//! because a paragraph about why `$HOME` is needed has no business pinning a
//! version-specific name, and the first draft of this one pinned v2's --
//! and AC-11.3's invariant permits the shipped surface exactly ONE environment
//! variable, `COLUMNS`, enforced structurally over every `src/**/*.rs` by
//! `no_intent_home::the_shipped_surface_reads_exactly_one_environment_variable`.
//! Driven both ways: green at HEAD, and a planted `std::env::var("HOME")` is
//! refused by name with *needs an hv ruling and a row in ALLOWED, not a quiet
//! addition*. So this module takes its paths and the CLI arm stays unwired
//! until hv rules. **The module is therefore fully drivable against tempdirs,
//! which is the same shape `rules.rs` uses for its own held ext ruling.**
//!
//! **v3 HAS ITS OWN MANIFEST AND NEVER READS OR WRITES v2's** (vc, 2026-08-22,
//! amending ruling 3). A version bump only helps a reader that checks versions,
//! and v2 does not and never will -- it is shipped. Writing a widened checksum
//! into v2's file, at v2's path, under a version string v2 ignores, produces a
//! **perpetual mutual clobber**: v2 finds `source != old` unconditionally (a
//! different function, so never equal), falls to its unguarded update branch and
//! overwrites; v3 then sees v2's SKILL.md checksums and overwrites back. Both
//! print `updated` every time and neither has anything to report. The condition
//! that makes it live is the ordinary cutover state -- **both binaries on `PATH`
//! together**. Separate paths make it unconstructible, and v2 keeps working
//! untouched: fail-forward means not carrying a defect forward, which is a
//! different sentence from breaking what you are forwarding from.
//!
//! **THERE IS NO CLOCK HERE AND NONE IS ADDED** (`one_clock.rs`; hv 2026-08-15,
//! time comes from the DB). `installed_at` is the **mtime of the file this
//! command just wrote** -- which is not a clock reading but a property the
//! filesystem already recorded, the distinction that file draws explicitly and
//! that `sync.rs:391` already relies on. A record stamped by the write that
//! creates it is the whole of D42; asking a clock and holding the answer across
//! a gap is what D42 forbids.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// **REUSED, NOT RESTATED.** A skill and a rule answer the same question --
/// did the tool ship this, or did the operator write it -- and it must not have
/// two answers (IN-AG-HIGHLANDER-001). The distinction matters for the same
/// reason in both: a shipped asset misbehaving is a bug report against Intent,
/// an operator's own is a question about this machine.
pub use crate::rules::Provenance;

/// The manifest schema this build writes.
///
/// **BUMPED FROM v2's `1.0.0` BECAUSE `checksum` NOW DENOTES A DIFFERENT
/// FUNCTION** (vc, 2026-08-22, ruling 3). A v2 entry read by a v3 comparison is
/// not a stale value, it is a value produced by another function, so every
/// comparison against it is meaningless rather than merely wrong. Leaving the
/// version at `1.0.0` would make one recorded number mean two things with
/// nothing saying which -- the class this estate has named all day: **a stored
/// value whose meaning changed without its version changing.**
pub const MANIFEST_VERSION: &str = "2.0.0";

/// The value of `checksum_scope` this build writes.
///
/// Declared as a FIELD as well as implied by the version, deliberately. The
/// version says *something changed*; the field says *what a checksum covers*,
/// which is the fact a reader actually needs. An implication a reader has to
/// know the history to decode is not a declaration.
pub const SCOPE_TREE: &str = "tree";

/// Where v3's manifest belongs, under the operator's `~/.intent`.
///
/// **DELIBERATELY NOT v2's `skills/installed-skills.json`** -- see the module
/// note on the mutual clobber. Same directory, so an operator looking for
/// either finds both together and can see that two tools are in play; a
/// filename v2 cannot produce, because v2 hardcodes its own and globs nothing.
///
/// The constant lives here rather than at the call site because the CLI arm
/// that would use it is held behind the AC-11.3 `$HOME` ruling, and a decision
/// recorded only in a message is a decision that has to be made twice.
pub const MANIFEST_RELATIVE: &str = "skills/installed-skills.v3.json";

/// A payload the `claude` plugin ships and installs into Claude Code.
///
/// **THE PLUGIN IS THE UNIT, NOT THE PAYLOAD KIND** (hv, 2026-08-30). `claude`
/// is an Intent plugin; `skills/` and `subagents/` are two things it ships, and
/// keeping an operator's Claude Code config matching Intent's canon is ONE
/// concern rather than two. v2 gave each kind its own command family -- five
/// verbs and seven -- over a shared 582-line helper, and v3 inherited the first
/// family and never built the second.
///
/// **EVERY DIFFERENCE BETWEEN THE KINDS IS IN THIS ENUM, AND THERE ARE ONLY
/// FOUR OF THEM.** Where the canon lives, what marks a directory as a real unit,
/// where installed copies land, and whether a unit installs as a tree or as a
/// single file. Everything else in this module -- the manifest, the checksums,
/// the install / sync / uninstall lifecycle, the whole `Outcome` vocabulary --
/// never needed to know, which is why it reads unchanged from when it served
/// one kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
  Skills,
  Agents,
}

impl Kind {
  /// The directory under `intent/plugins/claude/` holding this kind's canon.
  ///
  /// **`subagents` ON DISK, `agents` IN CLAUDE CODE, AND THE MISMATCH IS NOT
  /// OURS TO TIDY.** Intent has always called them subagents; Claude Code reads
  /// `~/.claude/agents/`. Renaming either side to match the other would break
  /// somebody's tree for a consistency nobody asked for.
  pub fn canon_subdir(self) -> &'static str {
    match self {
      Self::Skills => "skills",
      Self::Agents => "subagents",
    }
  }

  /// The file that must exist inside a canon directory for it to BE a unit.
  ///
  /// A directory without one is not a malformed unit, it is not a unit -- the
  /// distinction this module already draws for skills, kept for both.
  pub fn marker(self) -> &'static str {
    match self {
      Self::Skills => "SKILL.md",
      Self::Agents => "agent.md",
    }
  }

  /// Where installed units land under the operator's `~/.claude/`.
  pub fn target_subdir(self) -> &'static str {
    match self {
      Self::Skills => "skills",
      Self::Agents => "agents",
    }
  }

  /// This kind's manifest, relative to the operator's `~/.intent/`.
  pub fn manifest_relative(self) -> &'static str {
    match self {
      Self::Skills => MANIFEST_RELATIVE,
      Self::Agents => "subagents/installed-subagents.v3.json",
    }
  }

  /// Whether a unit installs as a whole directory or as one renamed file.
  ///
  /// **THE ONE STRUCTURAL DIFFERENCE, AND IT IS ASYMMETRIC.** A skill is a
  /// directory that lands as a directory. A subagent is a directory containing
  /// `agent.md` (plus a `metadata.json` Claude Code never reads) that lands as
  /// the single file `<name>.md`. So a subagent's SOURCE shape and TARGET shape
  /// differ, which a skill's never do.
  pub fn shape(self) -> Shape {
    match self {
      Self::Skills => Shape::Tree,
      Self::Agents => Shape::SingleFile,
    }
  }

  /// The `checksum_scope` token a manifest of this kind declares.
  pub fn scope_token(self) -> &'static str {
    match self {
      Self::Skills => SCOPE_TREE,
      Self::Agents => SCOPE_FILE,
    }
  }
}

/// How a unit's bytes move from canon to the operator's config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
  /// Every file under the unit's directory, at the same relative paths.
  Tree,
  /// The unit's marker file alone, renamed to `<name>.md`.
  SingleFile,
}

/// The `checksum_scope` a single-file payload writes.
///
/// **IT IS A DIFFERENT TOKEN BECAUSE IT COVERS A DIFFERENT THING, AND HASHING
/// THE WHOLE CANON DIRECTORY WOULD HAVE BEEN WRONG RATHER THAN MERELY WIDER.**
/// A subagent's canon directory holds `metadata.json` beside `agent.md`, and
/// only `agent.md` is installed. A tree hash would report an update as due
/// whenever the metadata moved, having changed nothing a consumer can see --
/// a checksum whose subject is not what it is being used to decide about.
/// **The rule for both kinds is the same one: hash what this payload
/// INSTALLS**, which for a tree is the tree and here is one file.
pub const SCOPE_FILE: &str = "file";

/// What a manifest's recorded `checksum` values cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
  /// The whole unit directory: every file's path and content.
  Tree,
  /// The single file this unit installs.
  ///
  /// See [`SCOPE_FILE`] for why a single-file payload does not simply reuse
  /// [`Scope::Tree`] over its canon directory.
  File,
  /// The manifest declares no scope this build understands.
  ///
  /// **AN UNDECLARED SCOPE MAKES EVERY `checksum` ABSENT, NOT STALE.** It is
  /// not a weaker value to be compared cautiously; it answers a different
  /// question, so it answers this one not at all.
  ///
  /// With v3 on its own path this is no longer how a v2 manifest arrives --
  /// v3 never opens one. What remains is a manifest hand-edited, or written by
  /// a LATER Intent whose scope this build cannot interpret, and both deserve
  /// the same answer: no usable baseline, so refuse rather than guess.
  Undeclared,
}

/// One installed skill, as the manifest records it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
  pub name: String,
  pub source_path: String,
  pub installed_at: String,
  pub checksum: String,
  /// Every file this tool wrote, relative to the installed skill directory.
  ///
  /// **THE FIELD THAT MAKES PRUNING POSSIBLE WITHOUT MAKING IT DESTRUCTIVE**
  /// (vc, 2026-08-22, ruling 5). The discriminator is *a sync may remove what
  /// it INSTALLED; it may not remove what it FOUND* -- and a checksum cannot
  /// express it, because it says whether the tree changed and never which files
  /// are ours. An operator who drops their own note into an installed skill
  /// directory must still have it after a sync.
  ///
  /// **Absent on a v2 entry**, which is exactly why `Scope::Undeclared` refuses
  /// rather than guesses: with no file list there is nothing a prune may
  /// safely touch.
  #[serde(default)]
  pub files: Vec<String>,
}

/// The installed-skills manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
  pub version: String,
  /// **`Option`, and the `None` is the whole of ruling 3.** A v2 manifest
  /// carries no such key; `serde(default)` reads that as `None` rather than
  /// failing, because refusing to parse a v2 manifest would leave an operator
  /// unable to run the command that explains the problem.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub checksum_scope: Option<String>,
  pub installed: Vec<Entry>,
}

impl Manifest {
  /// An empty manifest in this build's schema.
  pub fn empty(kind: Kind) -> Self {
    Self {
      version: MANIFEST_VERSION.to_string(),
      checksum_scope: Some(kind.scope_token().to_string()),
      installed: Vec::new(),
    }
  }

  /// What this manifest's checksums cover.
  ///
  /// **AN UNRECOGNISED VALUE IS `Undeclared`, NOT AN ERROR.** A manifest
  /// written by a LATER Intent than this one declares a scope this build cannot
  /// interpret, and the honest reading of a scope you do not understand is that
  /// you cannot use its checksums -- which is precisely what `Undeclared`
  /// already means. Erroring would make a forward-compatible file unreadable to
  /// the command that would have explained it.
  pub fn scope(&self) -> Scope {
    match self.checksum_scope.as_deref() {
      Some(SCOPE_TREE) => Scope::Tree,
      Some(SCOPE_FILE) => Scope::File,
      _ => Scope::Undeclared,
    }
  }

  fn find(&self, name: &str) -> Option<&Entry> {
    self.installed.iter().find(|e| e.name == name)
  }

  fn upsert(&mut self, entry: Entry) {
    self.installed.retain(|e| e.name != entry.name);
    self.installed.push(entry);
    self.installed.sort_by(|a, b| a.name.cmp(&b.name));
  }

  fn remove(&mut self, name: &str) {
    self.installed.retain(|e| e.name != name);
  }
}

#[derive(Debug, Error)]
pub enum PayloadError {
  #[error("cannot read or write {path}: {source}")]
  Io {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
  #[error("the payload manifest at {path} is not readable as JSON: {source}")]
  Manifest {
    path: PathBuf,
    #[source]
    source: serde_json::Error,
  },
  /// **A NAME THAT REACHES THE FILESYSTEM IS A NAME THAT CAN CONTAIN `../`.**
  /// Skill names are operator-supplied and are joined onto both the source root
  /// and the install target, so validation is not tidiness -- it is the only
  /// thing between an argument and an arbitrary path. `install.rs` makes the
  /// same argument about its hook roster and closes it by using a fixed list;
  /// skills cannot, because the set is open, so the name itself is constrained.
  #[error("`{name}` is not a usable skill name")]
  BadName { name: String },
  #[error("cannot read a timestamp from {path}: {source}")]
  Mtime {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
  #[error("cannot format the timestamp read from {path}: {source}")]
  Time {
    path: PathBuf,
    #[source]
    source: time::error::Format,
  },
}

impl crate::remedy::Remedy for PayloadError {
  fn remedy(&self) -> String {
    match self {
      Self::Io { path, .. } => format!(
        "check that {} exists and is writable. Skills are installed per-user, so this is usually a permissions question about your own home directory rather than anything about the project.",
        path.display()
      ),
      Self::Manifest { path, .. } => format!(
        "the manifest records what this tool installed, so it is safe to move {} aside and re-run `intent claude skills install` -- that rebuilds it. Do not hand-edit it: the checksums are what tell an upstream change from a local one.",
        path.display()
      ),
      Self::BadName { .. } => {
        "name a single skill directory -- letters, digits, `-` and `_` only. `intent claude skills list` prints the names this install carries.".to_string()
      }
      Self::Mtime { .. } | Self::Time { .. } => {
        "the file was written but its timestamp could not be read back, which usually means the filesystem does not record one. Re-run the command; if it persists, the manifest entry is still usable and only its `installed_at` is affected.".to_string()
      }
    }
  }
}

/// Where a skill's source lives.
#[derive(Debug, Clone)]
pub struct Origin {
  pub name: String,
  /// The skill's own directory, the thing that gets copied.
  pub dir: PathBuf,
  pub provenance: Provenance,
}

/// What happened to one skill.
/// Whether this build had a record of what it previously installed.
///
/// **THE WHOLE OF AC-07.3(d) TURNS ON THIS AND NOTHING ELSE.** With a record,
/// three-way comparison answers who moved what. Without one, an installed tree
/// that differs from source is EITHER an upstream change OR the operator's own
/// edit and **the information that would tell them apart was never written
/// down**, so no amount of looking recovers it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Baseline {
  /// The manifest recorded what this build wrote.
  Recorded,
  /// No usable record. **Force may still discard forward; it may not pretend
  /// to know what it discarded was.**
  Absent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
  Installed {
    files: usize,
  },
  Updated {
    written: usize,
    removed: Vec<String>,
  },
  UpToDate,
  /// **`--force` OVERRODE A TREE THAT HAD MOVED, AND `discarded` IS THE WHOLE
  /// REMEDY RATHER THAN A DIAGNOSTIC** (vc's ruling, 2026-08-22).
  ///
  /// An ordinary [`Outcome::Updated`] replaces a tree nobody had touched, so
  /// there is nothing to name. This one replaces the OPERATOR'S OWN WORK --
  /// `ModifiedLocally` or `Conflicted` with the hold overridden -- and once the
  /// copy has run, **the checksum is the only artefact that can identify what
  /// was there.** Printing `updated` for both would make the destructive run
  /// indistinguishable from the routine one, which is precisely v2's defect
  /// this module was built to end: v2 prints `update available` and overwrites,
  /// so the run that destroyed an edit reads exactly like the run that did not.
  ///
  /// **IT IS RAISED ONLY WHEN SOMETHING WAS ACTUALLY DISCARDED.** Forcing over
  /// a tree that matches its source destroys nothing, and reporting a discard
  /// there would teach an operator to ignore the line that matters.
  ///
  /// **WHAT THIS DOES NOT CLAIM: the checksum is not the content, and it cannot
  /// recover it.** It identifies; it does not restore. Said out loud because a
  /// remedy that reads as stronger than it is leaves an operator believing
  /// their edit is retrievable when it is gone.
  Forced {
    written: usize,
    removed: Vec<String>,
    discarded: String,
    /// **WHICH STATE THE FORCE RESOLVED, BECAUSE THE DISCARD MEANS DIFFERENT
    /// THINGS IN EACH** (vc, 2026-08-23, condition 2 of the grant).
    ///
    /// With a baseline recorded, the discarded bytes ARE the operator's edit and
    /// the report may say so. **With none, nobody can know that** -- AC-07.3(d)
    /// says the information distinguishing an upstream change from an edit was
    /// never recorded and is not recoverable -- so a report implying it was an
    /// edit would assert exactly the thing (d) says cannot be known.
    ///
    /// This is `target_moved` again: the flag says what was ASKED FOR, the state
    /// says what HAPPENED.
    baseline: Baseline,
  },
  /// Present, and no `--force` given. **The caller decides**, because prompting
  /// is a property of the terminal skin and not of the operation
  /// (IN-AG-THIN-COORD-001).
  AlreadyInstalled,
  /// The installed tree changed and the source did not.
  ModifiedLocally,
  /// **BOTH SIDES MOVED, AND v2 REPORTS IT AS AN ORDINARY UPDATE.**
  ///
  /// **v2's WRITE IS UNCONDITIONAL AND SITS DOWNSTREAM OF EVERY BRANCH**
  /// (`claude_plugin_helpers.sh:430`, re-read at source after a first reading
  /// of mine and a first reading of vc's both got the structure wrong). The
  /// `if`/`elif` above it chooses only what is PRINTED and whether it prompts;
  /// the only escapes from the comparison are `up to date` and a declined
  /// prompt, both of which `continue`. Everything else reaches the copy.
  ///
  /// So upstream changed AND the operator edited it misses the
  /// local-modification prompt -- that arm is guarded by `source == old` --
  /// takes the `elif`, prints **`update available`**, and overwrites. The
  /// operator sees the same two lines they would see for a routine upstream
  /// bump, with nothing distinguishing the run that destroyed their edit from
  /// the run that did not. `--force` is read only inside the arm that cannot
  /// run here, so it changes nothing either way.
  ///
  /// **THERE IS NO `add a condition to the elif` REPAIR, AND THAT IS WHY THE
  /// STRUCTURE MATTERS RATHER THAN JUST THE OUTCOME.** The comparison never
  /// guarded the write at all; it annotated it. A fix has to move the write.
  Conflicted,
  /// A skill is installed, its tree differs from source, and this build has no
  /// baseline for it -- no manifest entry, or a manifest whose checksum scope
  /// it cannot interpret. **Reported, never resolved** (vc, ruling 4, amended).
  ///
  /// **THE TRIGGER IS THE ABSENCE OF A BASELINE, NOT THE SHAPE OF A FILE.**
  /// vc's first form keyed it on the manifest being v2-shaped; with v3 on its
  /// own path that condition can no longer arise, and the replacement is both
  /// simpler and wider -- it also covers a skill somebody installed by hand,
  /// which no manifest ever knew about and which the original form missed.
  ///
  /// There is no recovery by cleverness. Rebaselining does not save it:
  /// recording the installed tree as the new baseline makes a locally-modified
  /// skill `source != old, target == old`, which is an ordinary update, so the
  /// operator's edit dies one command later instead of now. The information
  /// needed to tell an upstream change from a local one was never recorded.
  Undecidable,
  Removed {
    removed: Vec<String>,
    /// Files found in the installed directory that this tool never wrote, and
    /// therefore left alone.
    left: Vec<String>,
  },
  NotInstalled,
  SourceMissing,
}

/// One skill's result, plus anything the operator should know about it.
#[derive(Debug, Clone)]
pub struct Step {
  pub name: String,
  pub outcome: Outcome,
  /// Set when the name resolves in more than one root; carries the root that
  /// won. v2 prints this to stderr and continues, and so should any caller:
  /// shadowing is legitimate (it is what an extension is FOR) and silent
  /// shadowing is not.
  pub shadowed: Option<Provenance>,
}

#[derive(Debug, Clone)]
pub struct Report {
  pub steps: Vec<Step>,
  pub scope: Scope,
}

/// The skills this binary can install, and where they go.
///
/// Every path is supplied. See the module note on why that is a contract rather
/// than a convenience.
#[derive(Debug, Clone)]
pub struct Payload {
  kind: Kind,
  canon: PathBuf,
  /// The extension base (`~/.intent/ext`), when extensions are in play.
  ///
  /// **`Option`, exactly as `rules::Library` has it, and for the identical
  /// held reason.** Wiring the real resolution needs `$INTENT_EXT_DIR` /
  /// `$INTENT_EXT_DISABLE` / `$HOME`, which is the same AC-11.3 seam this whole
  /// module is parked behind. The consequence is named rather than swallowed:
  /// until it is wired, an operator with skills under `~/.intent/ext` sees them
  /// from v2 and not from v3.
  ext: Option<PathBuf>,
  target: PathBuf,
  manifest: PathBuf,
}

impl Payload {
  pub fn new(
    kind: Kind,
    install: &Path,
    ext: Option<PathBuf>,
    target: PathBuf,
    manifest: PathBuf,
  ) -> Self {
    Self {
      kind,
      canon: install
        .join("intent/plugins/claude")
        .join(kind.canon_subdir()),
      ext,
      target,
      manifest,
    }
  }

  /// Which payload this instance manages.
  pub fn kind(&self) -> Kind {
    self.kind
  }

  /// The source roots, in precedence order: extensions first, then canon.
  ///
  /// **EXT WINS, WHICH IS THE OPPOSITE ORDER FROM `rules::Library` AND IS NOT
  /// AN INCONSISTENCY.** Rules ENUMERATE -- every rule from every root appears,
  /// tagged with where it came from, so order is presentation. Skills RESOLVE
  /// -- one name yields one directory to copy -- so order is precedence, and an
  /// operator's own skill must override the shipped one or extensions cannot
  /// customise anything.
  fn roots(&self) -> Result<Vec<(Provenance, PathBuf)>, PayloadError> {
    let mut roots = Vec::new();
    for (name, dir) in self.ext_packs()? {
      roots.push((Provenance::Ext(name), dir));
    }
    roots.push((Provenance::Canon, self.canon.clone()));
    Ok(roots)
  }

  /// The extension packs carrying a `skills/` directory, by name.
  ///
  /// Dotfiles and `_`-prefixed directories are skipped, matching both v2 and
  /// `rules::Library`: the first is the operator's own hidden state, the second
  /// is the reservation the canon roots use for non-content directories.
  fn ext_packs(&self) -> Result<Vec<(String, PathBuf)>, PayloadError> {
    let Some(base) = &self.ext else {
      return Ok(Vec::new());
    };
    if !base.is_dir() {
      return Ok(Vec::new());
    }
    let mut packs = Vec::new();
    for entry in read_dir(base)? {
      let name = entry.file_name().to_string_lossy().to_string();
      if name.starts_with('.') || name.starts_with('_') {
        continue;
      }
      let dir = entry.path().join(self.kind.canon_subdir());
      if dir.is_dir() {
        packs.push((name, dir));
      }
    }
    packs.sort();
    Ok(packs)
  }

  /// Every root in which `name` resolves, in precedence order.
  ///
  /// A skill is a directory containing `SKILL.md`; a directory without one is
  /// not a skill, which is v2's test too.
  pub fn origins(&self, name: &str) -> Result<Vec<Origin>, PayloadError> {
    check_name(name)?;
    let mut found = Vec::new();
    for (provenance, root) in self.roots()? {
      let dir = root.join(name);
      if dir.join(self.kind.marker()).is_file() {
        found.push(Origin {
          name: name.to_string(),
          dir,
          provenance,
        });
      }
    }
    Ok(found)
  }

  /// The winning source for `name`, if any.
  pub fn resolve(&self, name: &str) -> Result<Option<Origin>, PayloadError> {
    Ok(self.origins(name)?.into_iter().next())
  }

  /// Every skill name this install can offer, deduplicated, in name order.
  ///
  /// **SORTED, because `read_dir` is not.** A command whose output depends on
  /// filesystem iteration order produces a different answer on every machine,
  /// which is the class `corpus_machine_independence` exists to catch.
  pub fn available(&self) -> Result<Vec<Origin>, PayloadError> {
    let mut names = BTreeSet::new();
    for (_, root) in self.roots()? {
      if !root.is_dir() {
        continue;
      }
      for entry in read_dir(&root)? {
        let name = entry.file_name().to_string_lossy().to_string();
        if check_name(&name).is_err() {
          continue;
        }
        if entry.path().join(self.kind.marker()).is_file() {
          names.insert(name);
        }
      }
    }
    let mut out = Vec::new();
    for name in names {
      if let Some(origin) = self.resolve(&name)? {
        out.push(origin);
      }
    }
    Ok(out)
  }

  /// Where a named unit is installed: a directory for a tree, a file otherwise.
  fn installed_dir(&self, name: &str) -> PathBuf {
    match self.kind.shape() {
      Shape::Tree => self.target.join(name),
      Shape::SingleFile => self.target.join(format!("{name}.md")),
    }
  }

  /// The installed file whose mtime IS this unit's `installed_at`.
  ///
  /// **A PATH, NOT A CLOCK.** The module note says why: `installed_at` is a
  /// property the filesystem recorded when this command wrote the file, which
  /// is what keeps it inside D42. This function only says WHICH file, and the
  /// two kinds answer differently because a tree's marker is inside it and a
  /// single file is its own.
  fn installed_marker(&self, name: &str) -> PathBuf {
    match self.kind.shape() {
      Shape::Tree => self.installed_dir(name).join(self.kind.marker()),
      Shape::SingleFile => self.installed_dir(name),
    }
  }

  pub fn is_installed(&self, name: &str) -> bool {
    self.installed_marker(name).is_file()
  }

  /// The checksum of what a canon unit would INSTALL, in this kind's scope.
  ///
  /// See [`SCOPE_FILE`]: for a single-file payload this is the marker alone and
  /// deliberately not the canon directory, because the directory carries files
  /// that never reach the operator and would report updates that change nothing.
  fn unit_checksum(&self, dir: &Path) -> Result<String, PayloadError> {
    match self.kind.shape() {
      Shape::Tree => tree_checksum(dir),
      Shape::SingleFile => file_checksum(&dir.join(self.kind.marker())),
    }
  }

  /// The checksum of an INSTALLED unit, in this kind's scope.
  ///
  /// Separate from [`Payload::unit_checksum`] because the two shapes diverge on
  /// exactly this axis: a tree installs at the same shape it is stored, and a
  /// single file does not, so "the installed bytes" is a different path
  /// expression from "the canon bytes" for one kind and the same for the other.
  /// The paths this unit occupies under the target, as the manifest records
  /// them: relative to the target root, `/`-separated.
  ///
  /// **THE MANIFEST DESCRIBES WHAT AN OPERATOR WOULD FIND, NOT WHAT CANON
  /// HOLDS**, which is only a distinction for the shape that renames on the way
  /// in. Recording a subagent's `agent.md` would name a file that is not there
  /// under a name no other verb uses.
  fn installed_files(&self, name: &str) -> Result<Vec<String>, PayloadError> {
    match self.kind.shape() {
      Shape::Tree => Ok(
        relative_files(&self.installed_dir(name))?
          .iter()
          .map(|p| display(p))
          .collect(),
      ),
      Shape::SingleFile => Ok(vec![format!("{name}.md")]),
    }
  }

  fn installed_checksum(&self, name: &str) -> Result<String, PayloadError> {
    match self.kind.shape() {
      Shape::Tree => tree_checksum(&self.installed_dir(name)),
      Shape::SingleFile => file_checksum(&self.installed_dir(name)),
    }
  }

  /// Every skill installed on disk, whatever this build's manifest knows.
  ///
  /// **DISK, NOT THE MANIFEST, AND THAT IS THE POINT.** The manifest records
  /// what this tool installed; the target directory records what is actually
  /// there. Where they disagree is precisely where the interesting cases live
  /// -- a skill installed by v2, or by hand -- so a reader that trusted the
  /// manifest for this question could never see one.
  pub fn installed(&self) -> Result<Vec<String>, PayloadError> {
    let mut out = Vec::new();
    if !self.target.is_dir() {
      return Ok(out);
    }
    for entry in read_dir(&self.target)? {
      let raw = entry.file_name().to_string_lossy().to_string();
      // **THE NAME IS RECOVERED FROM THE PATH, AND ONLY ONE KIND HAS TO UNDO A
      // RENAME TO GET IT.** A skill's directory is named for the skill; a
      // subagent's file is `<name>.md`, so the `.md` this tool appended on the
      // way in comes off on the way out. A scan that skipped that step would
      // report every agent under a name no verb accepts.
      let name = match self.kind.shape() {
        Shape::Tree => raw,
        Shape::SingleFile => match raw.strip_suffix(".md") {
          Some(stem) => stem.to_string(),
          None => continue,
        },
      };
      if check_name(&name).is_err() {
        continue;
      }
      if self.is_installed(&name) {
        out.push(name);
      }
    }
    out.sort();
    Ok(out)
  }

  /// Read the manifest, or an empty one in this build's schema.
  ///
  /// **A MISSING MANIFEST IS EMPTY; AN UNREADABLE ONE IS AN ERROR**
  /// (IN-AG-NO-SILENT-001). Nothing installed is an ordinary state. A manifest
  /// that exists and cannot be parsed is a broken install, and treating it as
  /// empty would silently re-install every skill and overwrite whatever the
  /// operator had.
  pub fn manifest(&self) -> Result<Manifest, PayloadError> {
    if !self.manifest.is_file() {
      return Ok(Manifest::empty(self.kind));
    }
    let text = std::fs::read_to_string(&self.manifest).map_err(|source| PayloadError::Io {
      path: self.manifest.clone(),
      source,
    })?;
    serde_json::from_str(&text).map_err(|source| PayloadError::Manifest {
      path: self.manifest.clone(),
      source,
    })
  }

  fn write_manifest(&self, manifest: &Manifest) -> Result<(), PayloadError> {
    if let Some(parent) = self.manifest.parent() {
      std::fs::create_dir_all(parent).map_err(|source| PayloadError::Io {
        path: parent.to_path_buf(),
        source,
      })?;
    }
    let mut text =
      serde_json::to_string_pretty(manifest).map_err(|source| PayloadError::Manifest {
        path: self.manifest.clone(),
        source,
      })?;
    text.push('\n');
    std::fs::write(&self.manifest, text).map_err(|source| PayloadError::Io {
      path: self.manifest.clone(),
      source,
    })
  }

  /// Install one or more named skills.
  ///
  /// `force` governs only the already-installed case; it is not a licence to
  /// ignore anything else.
  pub fn install(&self, names: &[String], force: bool) -> Result<Report, PayloadError> {
    let mut manifest = self.manifest()?;
    let mut steps = Vec::new();
    for name in names {
      let origins = match self.origins(name) {
        Ok(o) => o,
        Err(e @ PayloadError::BadName { .. }) => return Err(e),
        Err(e) => return Err(e),
      };
      let shadowed = shadow(&origins);
      let Some(origin) = origins.into_iter().next() else {
        steps.push(Step {
          name: name.clone(),
          outcome: Outcome::SourceMissing,
          shadowed,
        });
        continue;
      };
      let already = self.is_installed(name);
      if already && !force {
        steps.push(Step {
          name: name.clone(),
          outcome: Outcome::AlreadyInstalled,
          shadowed,
        });
        continue;
      }
      // **MEASURED BEFORE THE COPY, BECAUSE AFTER IT THERE IS NOTHING LEFT TO
      // MEASURE.** `install --force` over an existing tree is the other door
      // to the destruction `sync --force` reaches, and it had no report at all.
      let discarded = if already {
        let sum = self.installed_checksum(name)?;
        // Identical to source: the copy destroys nothing, so naming a discard
        // would teach the operator to ignore the line that matters.
        (sum != self.unit_checksum(&origin.dir)?).then_some(sum)
      } else {
        None
      };
      let prior = manifest.find(name).cloned();
      let (entry, removed) = self.materialise(&origin, prior.as_ref())?;
      let files = entry.files.len();
      manifest.upsert(entry);
      steps.push(Step {
        name: name.clone(),
        outcome: match (discarded, prior.is_some()) {
          (Some(discarded), _) => Outcome::Forced {
            written: files,
            removed,
            discarded,
            baseline: Baseline::Recorded,
          },
          (None, true) => Outcome::Updated {
            written: files,
            removed,
          },
          (None, false) => Outcome::Installed { files },
        },
        shadowed,
      });
    }
    self.write_manifest(&manifest)?;
    Ok(Report {
      scope: manifest.scope(),
      steps,
    })
  }

  /// Bring every installed skill up to date with its source.
  ///
  /// The comparison is v2's three-way -- source against the manifest's record,
  /// and the installed tree against the same record -- with the two corrections
  /// named on [`Outcome::Conflicted`] and [`Outcome::Undecidable`].
  pub fn sync(&self, force: bool) -> Result<Report, PayloadError> {
    let mut manifest = self.manifest()?;
    let scope = manifest.scope();
    // **THE UNION, NOT THE MANIFEST.** A skill present on disk that this build
    // has no entry for is exactly the baseline-less case ruling 4 governs, and
    // iterating the manifest alone would walk straight past it -- reporting a
    // clean sync over a skill it cannot account for.
    let mut names: BTreeSet<String> = manifest.installed.iter().map(|e| e.name.clone()).collect();
    names.extend(self.installed()?);
    let mut steps = Vec::new();

    for name in names {
      let origins = self.origins(&name)?;
      let shadowed = shadow(&origins);
      let Some(origin) = origins.into_iter().next() else {
        steps.push(Step {
          name,
          outcome: Outcome::SourceMissing,
          shadowed,
        });
        continue;
      };

      let source_sum = self.unit_checksum(&origin.dir)?;
      // **`is_installed`, NOT `is_dir`.** The old test asked whether the
      // target path was a DIRECTORY, which is true of an installed skill and
      // false of every installed subagent -- so a single-file payload would
      // have measured `None` for every row and reported an install as due,
      // forever, over files that were already correct.
      let target_sum = if self.is_installed(&name) {
        Some(self.installed_checksum(&name)?)
      } else {
        None
      };

      // **`old` IS AN `Option` AND AN UNDECLARED SCOPE MAKES IT `None`** --
      // ruling 3. The recorded string still exists; it just does not answer
      // this question, and carrying it as a value would invite exactly the
      // comparison that is meaningless.
      let old = match scope {
        // **BOTH DECLARED SCOPES ANSWER, AND THE ARM IS NOT A WILDCARD.** A
        // manifest whose scope matches THIS payload's kind has a usable
        // baseline; `Undeclared` is the only value that does not. Written as
        // two named arms rather than `_ =>` so that a third scope added later
        // has to be taught rather than inherited silently -- the failure this
        // module's own ruling 3 is about.
        Scope::Tree | Scope::File => manifest.find(&name).map(|e| e.checksum.clone()),
        Scope::Undeclared => None,
      };

      let outcome = match (&old, &target_sum) {
        // Nothing installed: this is an install, whatever the manifest says.
        (_, None) => {
          let prior = manifest.find(&name).cloned();
          let (entry, removed) = self.materialise(&origin, prior.as_ref())?;
          let written = entry.files.len();
          manifest.upsert(entry);
          Outcome::Updated { written, removed }
        }
        (Some(old), Some(target)) => {
          let source_moved = *old != source_sum;
          let target_moved = old != target;
          match (source_moved, target_moved) {
            (false, false) => Outcome::UpToDate,
            (false, true) if !force => Outcome::ModifiedLocally,
            (true, true) if !force => Outcome::Conflicted,
            _ => {
              // **`target_moved` IS THE DISCRIMINATOR, NOT `force`.** A forced
              // run over a tree that nobody touched is an ordinary update and
              // says so; only a tree that had MOVED has something to lose.
              // Keyed on the state rather than on the flag because the flag
              // says what the operator ASKED FOR and the state says what
              // actually happened, and only the second is worth reporting.
              let discarded = target_moved.then(|| target.clone());
              let prior = manifest.find(&name).cloned();
              let (entry, removed) = self.materialise(&origin, prior.as_ref())?;
              let written = entry.files.len();
              manifest.upsert(entry);
              match discarded {
                Some(discarded) => Outcome::Forced {
                  written,
                  removed,
                  discarded,
                  baseline: Baseline::Recorded,
                },
                None => Outcome::Updated { written, removed },
              }
            }
          }
        }
        // Ruling 4. With no usable baseline, an installed tree that differs
        // from source is EITHER an upstream change OR the operator's own edit,
        // and v2 recorded nothing that can tell them apart. Rebaselining does
        // not recover it: recording the installed tree as the new baseline
        // makes a locally-modified skill `source != old, target == old`, which
        // is an update, so the edit dies one command later instead of now.
        (None, Some(target)) => {
          if *target == source_sum {
            // **ADOPTING A BYTE-IDENTICAL TREE IS NOT REBASELINING.** The
            // objection to rebaselining is that it discards the distinction
            // between an upstream change and a local edit. Here there is no
            // distinction to discard: the installed tree and the source agree
            // exactly, so recording it loses nothing and gives the next sync a
            // baseline to work from instead of refusing forever.
            let installed_at = mtime_rfc3339(&self.installed_marker(&name))?;
            manifest.upsert(Entry {
              name: name.clone(),
              source_path: origin.dir.display().to_string(),
              installed_at,
              checksum: source_sum.clone(),
              files: self.installed_files(&name)?,
            });
            Outcome::UpToDate
          } else if force {
            // **FORCE REACHES THIS STATE TOO (vc, 2026-08-23, under hv's pen),
            // AND THE GRANT TURNS ON TWO CLAUSES OF AC-07.3 THAT DO NOT
            // CONFLICT.** (d) says that with `old` absent v3 REPORTS AND REFUSES
            // TO CHOOSE, because what distinguishes an upstream change from an
            // operator edit was never recorded and is not recoverable. (e) says
            // `--force` must exist and must report the discarded checksum,
            // because **without it a held skill has no CLI remedy at all and
            // the honest refusal is a dead end.**
            //
            // **(d) FORBIDS CHOOSING; (e) LICENSES DESTROYING WITH A RECORD.**
            // Force does not adjudicate whether the local bytes were an edit --
            // it DECLINES TO KNOW, discards forward at the operator's explicit
            // instruction, and records what it destroyed. And (d) creates the
            // HOLD that (e) was minted to give a remedy to: if force does not
            // reach here, (d)'s hold IS the dead end (e) exists to close.
            //
            // **MY OWN ARGUMENT FOR THIS WAS THE WRONG HALF, AND THE
            // CORRECTION IS WORTH MORE THAN THE GRANT.** I argued that v3 has
            // no prompt to override -- which answers the FIRST clause of the
            // old test's name and leaves the second standing. *Not about
            // inventing information that was never recorded* is not retired by
            // anything; it is (d), ratified. **Taking my reasoning would have
            // retired a live constraint along with a dead one.**
            //
            // **SO: NO BASELINE IS INVENTED HERE.** `materialise` writes the
            // source, and the manifest entry records THE NEW STATE -- never the
            // discarded tree, which would launder unknown bytes into a
            // baseline and make the next sync report an update where nobody
            // knows one happened.
            let prior = manifest.find(&name).cloned();
            let (entry, removed) = self.materialise(&origin, prior.as_ref())?;
            let written = entry.files.len();
            manifest.upsert(entry);
            Outcome::Forced {
              written,
              removed,
              discarded: target.clone(),
              baseline: Baseline::Absent,
            }
          } else {
            Outcome::Undecidable
          }
        }
      };

      steps.push(Step {
        name,
        outcome,
        shadowed,
      });
    }

    self.write_manifest(&manifest)?;
    Ok(Report { steps, scope })
  }

  /// Remove named skills.
  ///
  /// **REMOVES WHAT IT INSTALLED AND LEAVES WHAT IT FOUND** -- ruling 5's
  /// discriminator, applied here as well as in `sync`. v2 does `rm -rf` on the
  /// whole directory, which destroys an operator's own file dropped inside it.
  /// A rule that holds in one verb and not its sibling is not a rule, and this
  /// direction can never lose data.
  pub fn uninstall(&self, names: &[String]) -> Result<Report, PayloadError> {
    let mut manifest = self.manifest()?;
    let mut steps = Vec::new();
    for name in names {
      check_name(name)?;
      let dir = self.installed_dir(name);
      if !dir.exists() {
        manifest.remove(name);
        steps.push(Step {
          name: name.clone(),
          outcome: Outcome::NotInstalled,
          shadowed: None,
        });
        continue;
      }
      let recorded: BTreeSet<String> = manifest
        .find(name)
        .map(|e| e.files.iter().cloned().collect())
        .unwrap_or_default();

      // **THE SINGLE-FILE SHAPE REMOVES A FILE AND NEVER WALKS A DIRECTORY**,
      // and it keeps the remove-only-what-we-recorded rule rather than being
      // excused from it. That rule matters MORE here, not less: every kind's
      // units share one directory, so an unrecorded `<name>.md` is either the
      // operator's own agent or one v2 installed -- and this machine's own
      // `~/.claude/agents/` held eight of the latter and no v3 manifest at all
      // when this was written. Removing it because the name matched would
      // delete a file this build never wrote.
      if self.kind.shape() == Shape::SingleFile {
        let shown = format!("{name}.md");
        let (removed, left) = if recorded.contains(&shown) {
          remove_file(&dir)?;
          (vec![shown], Vec::new())
        } else {
          (Vec::new(), vec![shown])
        };
        manifest.remove(name);
        steps.push(Step {
          name: name.clone(),
          outcome: Outcome::Removed { removed, left },
          shadowed: None,
        });
        continue;
      }

      let present = relative_files(&dir)?;
      let mut removed = Vec::new();
      let mut left = Vec::new();
      for rel in &present {
        let shown = display(rel);
        if recorded.contains(&shown) {
          remove_file(&dir.join(rel))?;
          removed.push(shown);
        } else {
          left.push(shown);
        }
      }
      // **A v2 ENTRY HAS NO FILE LIST, SO NOTHING IS RECORDED AND NOTHING IS
      // REMOVED.** That is the honest outcome rather than a failure: the
      // operator is told the directory was left, and `--force`-installing over
      // it first gives this build a file list to work from.
      prune_empty_dirs(&dir)?;
      manifest.remove(name);
      steps.push(Step {
        name: name.clone(),
        outcome: Outcome::Removed { removed, left },
        shadowed: None,
      });
    }
    self.write_manifest(&manifest)?;
    Ok(Report {
      scope: manifest.scope(),
      steps,
    })
  }

  /// Copy a skill's tree into place and build its manifest entry.
  ///
  /// **ONE IMPLEMENTATION BEHIND `install` AND `sync`.** Two copies of "write
  /// the tree, prune what we wrote and source no longer has, record what we
  /// wrote" would agree on the day they were written and drift the first time
  /// either verb changed (IN-AG-HIGHLANDER-001).
  fn materialise(
    &self,
    origin: &Origin,
    prior: Option<&Entry>,
  ) -> Result<(Entry, Vec<String>), PayloadError> {
    let dest = self.installed_dir(&origin.name);

    // **THE SINGLE-FILE SHAPE RETURNS EARLY RATHER THAN THREADING BRANCHES
    // THROUGH THE TREE WALK.** Everything below this point -- the prune of
    // recorded-but-no-longer-sourced files, the empty-directory sweep -- is
    // about a unit that OWNS a directory. A subagent owns one file inside a
    // directory full of other people's agents, so a prune there would be a
    // tool reaching outside what it installed. There is nothing to prune: the
    // written set is always exactly one path, so nothing recorded can fail to
    // be re-written.
    if self.kind.shape() == Shape::SingleFile {
      let from = origin.dir.join(self.kind.marker());
      if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|source| PayloadError::Io {
          path: parent.to_path_buf(),
          source,
        })?;
      }
      std::fs::copy(&from, &dest).map_err(|source| PayloadError::Io {
        path: dest.clone(),
        source,
      })?;
      let checksum = self.unit_checksum(&origin.dir)?;
      let installed_at = mtime_rfc3339(&dest)?;
      return Ok((
        Entry {
          name: origin.name.clone(),
          source_path: origin.dir.display().to_string(),
          installed_at,
          checksum,
          files: self.installed_files(&origin.name)?,
        },
        Vec::new(),
      ));
    }

    let sources = relative_files(&origin.dir)?;
    let written: BTreeSet<String> = sources.iter().map(|p| display(p)).collect();

    for rel in &sources {
      let from = origin.dir.join(rel);
      let to = dest.join(rel);
      if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent).map_err(|source| PayloadError::Io {
          path: parent.to_path_buf(),
          source,
        })?;
      }
      std::fs::copy(&from, &to).map_err(|source| PayloadError::Io {
        path: to.clone(),
        source,
      })?;
    }

    // Ruling 5: remove only what a previous run of THIS tool recorded writing
    // and that source no longer carries. An unrecorded file is the operator's.
    let mut removed = Vec::new();
    if let Some(prior) = prior {
      for stale in prior.files.iter().filter(|f| !written.contains(*f)) {
        let path = dest.join(stale);
        if path.is_file() {
          remove_file(&path)?;
          removed.push(stale.clone());
        }
      }
      removed.sort();
    }
    prune_empty_dirs(&dest)?;

    let checksum = self.unit_checksum(&origin.dir)?;
    let installed_at = mtime_rfc3339(&dest.join(self.kind.marker()))?;
    Ok((
      Entry {
        name: origin.name.clone(),
        source_path: origin.dir.display().to_string(),
        installed_at,
        checksum,
        files: written.into_iter().collect(),
      },
      removed,
    ))
  }
}

/// The root that won, when a name resolves in more than one.
fn shadow(origins: &[Origin]) -> Option<Provenance> {
  if origins.len() > 1 {
    origins.first().map(|o| o.provenance.clone())
  } else {
    None
  }
}

/// **A SKILL NAME IS A SINGLE DIRECTORY NAME AND NOTHING ELSE.** See
/// [`PayloadError::BadName`] for why this is a boundary rather than a nicety.
fn check_name(name: &str) -> Result<(), PayloadError> {
  let ok = !name.is_empty()
    && name
      .chars()
      .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
  if ok {
    Ok(())
  } else {
    Err(PayloadError::BadName {
      name: name.to_string(),
    })
  }
}

/// Every file at or below `dir`, relative to it, in a deterministic order.
///
/// **DIRECTORIES ARE NOT ENTRIES AND SYMLINKS ARE NOT FOLLOWED.** A skill is
/// content; following a link out of the tree would copy something the source
/// root does not own, and it is the shape that turns an install into an
/// arbitrary read of the operator's disk.
fn relative_files(dir: &Path) -> Result<Vec<PathBuf>, PayloadError> {
  fn walk(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), PayloadError> {
    for entry in read_dir(dir)? {
      let path = entry.path();
      let meta = std::fs::symlink_metadata(&path).map_err(|source| PayloadError::Io {
        path: path.clone(),
        source,
      })?;
      if meta.file_type().is_symlink() {
        continue;
      }
      if meta.is_dir() {
        walk(root, &path, out)?;
      } else if meta.is_file()
        && let Ok(rel) = path.strip_prefix(root)
      {
        out.push(rel.to_path_buf());
      }
    }
    Ok(())
  }
  let mut out = Vec::new();
  if dir.is_dir() {
    walk(dir, dir, &mut out)?;
  }
  out.sort();
  Ok(out)
}

/// The SHA-256 of a whole skill directory: every file's path AND its content.
///
/// **PATHS ARE HASHED, NOT ONLY CONTENTS**, so renaming a file inside a skill
/// changes the checksum. A content-only digest reports a rename as no change,
/// which is the same blind spot one axis over.
/// The SHA-256 of ONE file's contents.
///
/// **NO PATH IS HASHED HERE, AND THAT IS THE DIFFERENCE FROM
/// [`tree_checksum`] RATHER THAN AN OMISSION.** A tree hashes paths because a
/// rename inside it is a real change to the unit. A single-file unit has one
/// path, and this tool chose it -- `<name>.md` -- so hashing it would add a
/// constant and answer the same question more slowly.
fn file_checksum(path: &Path) -> Result<String, PayloadError> {
  let bytes = std::fs::read(path).map_err(|source| PayloadError::Io {
    path: path.to_path_buf(),
    source,
  })?;
  Ok(format!("{:x}", Sha256::digest(&bytes)))
}

fn tree_checksum(dir: &Path) -> Result<String, PayloadError> {
  let mut hasher = Sha256::new();
  for rel in relative_files(dir)? {
    let bytes = std::fs::read(dir.join(&rel)).map_err(|source| PayloadError::Io {
      path: dir.join(&rel),
      source,
    })?;
    hasher.update(display(&rel).as_bytes());
    hasher.update(b"\0");
    hasher.update(format!("{:x}", Sha256::digest(&bytes)).as_bytes());
    hasher.update(b"\n");
  }
  Ok(format!("{:x}", hasher.finalize()))
}

/// A relative path as the manifest records it: `/`-separated, on every platform.
///
/// The manifest is data about an install and is read by people; a backslash
/// form on one platform and a slash form on another would make the same skill
/// record two different file lists.
fn display(rel: &Path) -> String {
  rel
    .components()
    .map(|c| c.as_os_str().to_string_lossy().to_string())
    .collect::<Vec<_>>()
    .join("/")
}

/// The written file's own mtime, RFC 3339.
///
/// See the module note: this is not a clock reading. `sync.rs:391` does the
/// same conversion for the same reason.
fn mtime_rfc3339(path: &Path) -> Result<String, PayloadError> {
  let meta = std::fs::metadata(path).map_err(|source| PayloadError::Mtime {
    path: path.to_path_buf(),
    source,
  })?;
  let modified = meta.modified().map_err(|source| PayloadError::Mtime {
    path: path.to_path_buf(),
    source,
  })?;
  OffsetDateTime::from(modified)
    .format(&Rfc3339)
    .map_err(|source| PayloadError::Time {
      path: path.to_path_buf(),
      source,
    })
}

fn read_dir(dir: &Path) -> Result<Vec<std::fs::DirEntry>, PayloadError> {
  let entries = std::fs::read_dir(dir).map_err(|source| PayloadError::Io {
    path: dir.to_path_buf(),
    source,
  })?;
  let mut out = Vec::new();
  for entry in entries {
    out.push(entry.map_err(|source| PayloadError::Io {
      path: dir.to_path_buf(),
      source,
    })?);
  }
  Ok(out)
}

fn remove_file(path: &Path) -> Result<(), PayloadError> {
  std::fs::remove_file(path).map_err(|source| PayloadError::Io {
    path: path.to_path_buf(),
    source,
  })
}

/// Drop directories a prune emptied, deepest first, stopping at `root`.
///
/// A directory left behind by removing the last file in it is a visible
/// artefact of a file that no longer exists. `remove_dir` is used rather than
/// `remove_dir_all` deliberately: it refuses a non-empty directory, so this can
/// never reach a file it was not entitled to.
fn prune_empty_dirs(root: &Path) -> Result<(), PayloadError> {
  fn walk(root: &Path, dir: &Path) -> Result<(), PayloadError> {
    if !dir.is_dir() {
      return Ok(());
    }
    for entry in read_dir(dir)? {
      let path = entry.path();
      if path.is_dir() {
        walk(root, &path)?;
      }
    }
    if dir != root && read_dir(dir)?.is_empty() {
      let _ = std::fs::remove_dir(dir);
    }
    Ok(())
  }
  if root.is_dir() {
    walk(root, root)
  } else {
    Ok(())
  }
}
