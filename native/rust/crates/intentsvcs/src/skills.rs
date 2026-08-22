//! `intent claude skills` -- the skills Intent installs into Claude Code.
//!
//! AC-07.3. v2's implementation is `intent/plugins/claude/bin/intent_claude_skills`
//! over the shared `claude_plugin_helpers.sh`; this is the port, and it is a
//! port with three ruled corrections rather than a transcription. Each one is
//! named at the code that carries it.
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

/// What a manifest's recorded `checksum` values cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
  /// The whole skill directory: every file's path and content.
  Tree,
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
  pub fn empty() -> Self {
    Self {
      version: MANIFEST_VERSION.to_string(),
      checksum_scope: Some(SCOPE_TREE.to_string()),
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
pub enum SkillsError {
  #[error("cannot read or write {path}: {source}")]
  Io {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
  #[error("the installed-skills manifest at {path} is not readable as JSON: {source}")]
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

impl crate::remedy::Remedy for SkillsError {
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
pub struct Skills {
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

impl Skills {
  pub fn new(install: &Path, ext: Option<PathBuf>, target: PathBuf, manifest: PathBuf) -> Self {
    Self {
      canon: install.join("intent/plugins/claude/skills"),
      ext,
      target,
      manifest,
    }
  }

  /// The source roots, in precedence order: extensions first, then canon.
  ///
  /// **EXT WINS, WHICH IS THE OPPOSITE ORDER FROM `rules::Library` AND IS NOT
  /// AN INCONSISTENCY.** Rules ENUMERATE -- every rule from every root appears,
  /// tagged with where it came from, so order is presentation. Skills RESOLVE
  /// -- one name yields one directory to copy -- so order is precedence, and an
  /// operator's own skill must override the shipped one or extensions cannot
  /// customise anything.
  fn roots(&self) -> Result<Vec<(Provenance, PathBuf)>, SkillsError> {
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
  fn ext_packs(&self) -> Result<Vec<(String, PathBuf)>, SkillsError> {
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
      let dir = entry.path().join("skills");
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
  pub fn origins(&self, name: &str) -> Result<Vec<Origin>, SkillsError> {
    check_name(name)?;
    let mut found = Vec::new();
    for (provenance, root) in self.roots()? {
      let dir = root.join(name);
      if dir.join("SKILL.md").is_file() {
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
  pub fn resolve(&self, name: &str) -> Result<Option<Origin>, SkillsError> {
    Ok(self.origins(name)?.into_iter().next())
  }

  /// Every skill name this install can offer, deduplicated, in name order.
  ///
  /// **SORTED, because `read_dir` is not.** A command whose output depends on
  /// filesystem iteration order produces a different answer on every machine,
  /// which is the class `corpus_machine_independence` exists to catch.
  pub fn available(&self) -> Result<Vec<Origin>, SkillsError> {
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
        if entry.path().join("SKILL.md").is_file() {
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

  /// Where a named skill is installed.
  fn installed_dir(&self, name: &str) -> PathBuf {
    self.target.join(name)
  }

  pub fn is_installed(&self, name: &str) -> bool {
    self.installed_dir(name).join("SKILL.md").is_file()
  }

  /// Every skill installed on disk, whatever this build's manifest knows.
  ///
  /// **DISK, NOT THE MANIFEST, AND THAT IS THE POINT.** The manifest records
  /// what this tool installed; the target directory records what is actually
  /// there. Where they disagree is precisely where the interesting cases live
  /// -- a skill installed by v2, or by hand -- so a reader that trusted the
  /// manifest for this question could never see one.
  pub fn installed(&self) -> Result<Vec<String>, SkillsError> {
    let mut out = Vec::new();
    if !self.target.is_dir() {
      return Ok(out);
    }
    for entry in read_dir(&self.target)? {
      let name = entry.file_name().to_string_lossy().to_string();
      if check_name(&name).is_err() {
        continue;
      }
      if entry.path().join("SKILL.md").is_file() {
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
  pub fn manifest(&self) -> Result<Manifest, SkillsError> {
    if !self.manifest.is_file() {
      return Ok(Manifest::empty());
    }
    let text = std::fs::read_to_string(&self.manifest).map_err(|source| SkillsError::Io {
      path: self.manifest.clone(),
      source,
    })?;
    serde_json::from_str(&text).map_err(|source| SkillsError::Manifest {
      path: self.manifest.clone(),
      source,
    })
  }

  fn write_manifest(&self, manifest: &Manifest) -> Result<(), SkillsError> {
    if let Some(parent) = self.manifest.parent() {
      std::fs::create_dir_all(parent).map_err(|source| SkillsError::Io {
        path: parent.to_path_buf(),
        source,
      })?;
    }
    let mut text =
      serde_json::to_string_pretty(manifest).map_err(|source| SkillsError::Manifest {
        path: self.manifest.clone(),
        source,
      })?;
    text.push('\n');
    std::fs::write(&self.manifest, text).map_err(|source| SkillsError::Io {
      path: self.manifest.clone(),
      source,
    })
  }

  /// Install one or more named skills.
  ///
  /// `force` governs only the already-installed case; it is not a licence to
  /// ignore anything else.
  pub fn install(&self, names: &[String], force: bool) -> Result<Report, SkillsError> {
    let mut manifest = self.manifest()?;
    let mut steps = Vec::new();
    for name in names {
      let origins = match self.origins(name) {
        Ok(o) => o,
        Err(e @ SkillsError::BadName { .. }) => return Err(e),
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
      if self.is_installed(name) && !force {
        steps.push(Step {
          name: name.clone(),
          outcome: Outcome::AlreadyInstalled,
          shadowed,
        });
        continue;
      }
      let prior = manifest.find(name).cloned();
      let (entry, removed) = self.materialise(&origin, prior.as_ref())?;
      let files = entry.files.len();
      manifest.upsert(entry);
      steps.push(Step {
        name: name.clone(),
        outcome: if prior.is_some() {
          Outcome::Updated {
            written: files,
            removed,
          }
        } else {
          Outcome::Installed { files }
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
  pub fn sync(&self, force: bool) -> Result<Report, SkillsError> {
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

      let source_sum = tree_checksum(&origin.dir)?;
      let installed = self.installed_dir(&name);
      let target_sum = if installed.is_dir() {
        Some(tree_checksum(&installed)?)
      } else {
        None
      };

      // **`old` IS AN `Option` AND AN UNDECLARED SCOPE MAKES IT `None`** --
      // ruling 3. The recorded string still exists; it just does not answer
      // this question, and carrying it as a value would invite exactly the
      // comparison that is meaningless.
      let old = match scope {
        Scope::Tree => manifest.find(&name).map(|e| e.checksum.clone()),
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
              let prior = manifest.find(&name).cloned();
              let (entry, removed) = self.materialise(&origin, prior.as_ref())?;
              let written = entry.files.len();
              manifest.upsert(entry);
              Outcome::Updated { written, removed }
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
            let installed_at = mtime_rfc3339(&installed.join("SKILL.md"))?;
            manifest.upsert(Entry {
              name: name.clone(),
              source_path: origin.dir.display().to_string(),
              installed_at,
              checksum: source_sum.clone(),
              files: relative_files(&installed)?
                .iter()
                .map(|p| display(p))
                .collect(),
            });
            Outcome::UpToDate
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
  pub fn uninstall(&self, names: &[String]) -> Result<Report, SkillsError> {
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
  ) -> Result<(Entry, Vec<String>), SkillsError> {
    let dest = self.installed_dir(&origin.name);
    let sources = relative_files(&origin.dir)?;
    let written: BTreeSet<String> = sources.iter().map(|p| display(p)).collect();

    for rel in &sources {
      let from = origin.dir.join(rel);
      let to = dest.join(rel);
      if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent).map_err(|source| SkillsError::Io {
          path: parent.to_path_buf(),
          source,
        })?;
      }
      std::fs::copy(&from, &to).map_err(|source| SkillsError::Io {
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

    let checksum = tree_checksum(&origin.dir)?;
    let installed_at = mtime_rfc3339(&dest.join("SKILL.md"))?;
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
/// [`SkillsError::BadName`] for why this is a boundary rather than a nicety.
fn check_name(name: &str) -> Result<(), SkillsError> {
  let ok = !name.is_empty()
    && name
      .chars()
      .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
  if ok {
    Ok(())
  } else {
    Err(SkillsError::BadName {
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
fn relative_files(dir: &Path) -> Result<Vec<PathBuf>, SkillsError> {
  fn walk(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), SkillsError> {
    for entry in read_dir(dir)? {
      let path = entry.path();
      let meta = std::fs::symlink_metadata(&path).map_err(|source| SkillsError::Io {
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
fn tree_checksum(dir: &Path) -> Result<String, SkillsError> {
  let mut hasher = Sha256::new();
  for rel in relative_files(dir)? {
    let bytes = std::fs::read(dir.join(&rel)).map_err(|source| SkillsError::Io {
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
fn mtime_rfc3339(path: &Path) -> Result<String, SkillsError> {
  let meta = std::fs::metadata(path).map_err(|source| SkillsError::Mtime {
    path: path.to_path_buf(),
    source,
  })?;
  let modified = meta.modified().map_err(|source| SkillsError::Mtime {
    path: path.to_path_buf(),
    source,
  })?;
  OffsetDateTime::from(modified)
    .format(&Rfc3339)
    .map_err(|source| SkillsError::Time {
      path: path.to_path_buf(),
      source,
    })
}

fn read_dir(dir: &Path) -> Result<Vec<std::fs::DirEntry>, SkillsError> {
  let entries = std::fs::read_dir(dir).map_err(|source| SkillsError::Io {
    path: dir.to_path_buf(),
    source,
  })?;
  let mut out = Vec::new();
  for entry in entries {
    out.push(entry.map_err(|source| SkillsError::Io {
      path: dir.to_path_buf(),
      source,
    })?);
  }
  Ok(out)
}

fn remove_file(path: &Path) -> Result<(), SkillsError> {
  std::fs::remove_file(path).map_err(|source| SkillsError::Io {
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
fn prune_empty_dirs(root: &Path) -> Result<(), SkillsError> {
  fn walk(root: &Path, dir: &Path) -> Result<(), SkillsError> {
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
