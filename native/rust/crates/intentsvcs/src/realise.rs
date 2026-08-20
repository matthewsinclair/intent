//! The text realisation -- the human fallback (ST0057 WP-06, AC-06.1 / AC-06.2).
//!
//! **D57 MAKES THIS A PRECONDITION OF SPARSENESS RATHER THAN A CONSEQUENCE OF
//! IT.** No tree may hold less than everything until a human can get all of it
//! back without the tool. So this is not a convenience export beside the model;
//! it is the thing that makes removing files from a working tree a defensible
//! act at all, and it is one of the nineteen declared preconditions the
//! dehydration gate refuses on.
//!
//! **THE DENOMINATOR IS THE CRITERION, NOT A NICETY OF THE OUTPUT (AC-06.1).**
//! A partial realisation that reads as complete is worse than no realisation:
//! it is the artefact a human reaches for precisely when the tool is gone or
//! wrong, so a quiet shortfall is discovered at the moment nothing else can
//! help. [`Realisation::complete`] compares what was written against canon's
//! OWN totals -- asked of the owner, never reconstructed from the work done,
//! which is the defect `Facade::hydrate` shipped and its own test caught.
//!
//! **WRITE-ONLY BY CONSTRUCTION (AC-06.2).** There is no reader in this module
//! and there is no route back: `Project::classify` never sees a path under
//! `.backup/`, so nothing here can become authoritative by being read back.
//! That is asserted by the absence of a read path rather than intended, because
//! an import route added later would silently promote a regenerable artefact
//! into a second source of truth.
//!
//! **NO TIME CROSSES INTO THIS MODULE (D42).** It takes a DESTINATION, not a
//! stamp. The caller derives the directory from a stamp the DATABASE returned
//! -- `append_event` emits `RETURNING ts` -- exactly as [`crate::backup`] takes
//! its filename from `begin_snapshot`. A `stamp: &str` parameter here would be
//! a time-typed input wearing a string's clothes, and D42 is about signatures.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::ingest::Canon;
use crate::project::Project;
use crate::views::{self, RenderContext};
use crate::write_set::WriteSet;

/// What the estate holds, and what a realisation of it wrote.
///
/// **Five populations rather than one total, because a shortfall has to name
/// WHICH KIND went missing.** "1841 of 1847" sends a human to diff two trees;
/// "attachments 4 of 10" sends them to the four threads carrying attachments.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
  pub threads: usize,
  pub wps: usize,
  pub issues: usize,
  pub attachments: usize,
  pub views: usize,
}

impl Counts {
  /// Canon's own totals. **THE DENOMINATOR, ASKED OF THE OWNER.**
  ///
  /// Derived here from the canon rather than accumulated while writing, so the
  /// two numbers this criterion compares come from genuinely different places.
  /// Counting as we write and comparing that to itself is a tautology that
  /// reports complete for every input.
  pub fn of(canon: &Canon) -> Self {
    Self {
      threads: canon.threads.len(),
      wps: canon.threads.iter().map(|t| t.wps.len()).sum(),
      issues: canon.issues.len(),
      attachments: canon.threads.iter().map(|t| t.attachments.len()).sum(),
      // A view per thread info + per thread acceptance + per WP, which is what
      // `views::render_all` produces. Asked of the renderer below rather than
      // recomputed from that sentence -- see `realise`.
      views: 0,
    }
  }

  pub fn total(&self) -> usize {
    self.threads + self.wps + self.issues + self.attachments + self.views
  }
}

/// One completed realisation.
#[derive(Debug, Clone)]
pub struct Realisation {
  /// The directory everything was written under.
  pub root: PathBuf,
  /// Every file written, in path order.
  pub written: Vec<PathBuf>,
  /// What this run realised.
  pub counts: Counts,
  /// What canon holds. **AC-06.1's denominator.**
  pub totals: Counts,
}

impl Realisation {
  /// Whether every artefact canon holds reached the realisation.
  ///
  /// **The whole criterion, in one comparison, and it is the reason `Counts`
  /// derives `PartialEq` rather than exposing a per-field check at every call
  /// site.** A caller writing its own five comparisons is a caller that can
  /// forget the fifth, and the fifth is attachments -- the population most
  /// likely to be partially realisable and least likely to be noticed.
  pub fn complete(&self) -> bool {
    self.counts == self.totals
  }

  /// The populations that fell short, named. Empty when [`Self::complete`].
  pub fn shortfall(&self) -> Vec<String> {
    let mut out = Vec::new();
    let mut note = |name: &str, got: usize, want: usize| {
      if got != want {
        out.push(format!("{name} {got} of {want}"));
      }
    };
    note("threads", self.counts.threads, self.totals.threads);
    note("work packages", self.counts.wps, self.totals.wps);
    note("issues", self.counts.issues, self.totals.issues);
    note(
      "attachments",
      self.counts.attachments,
      self.totals.attachments,
    );
    note("views", self.counts.views, self.totals.views);
    out
  }
}

/// Why a realisation did not happen.
#[derive(Debug, Error)]
pub enum RealiseError {
  /// An opaque attachment whose sidecar was never loaded.
  ///
  /// **REFUSED RATHER THAN SKIPPED, and the difference is the whole point of
  /// the denominator.** Canon names bytes; if they are not here, writing a
  /// zero-byte file or omitting the entry would produce a realisation that
  /// counts short -- or worse, one that counts complete over a file the human
  /// cannot use. `model.rs` already makes a missing sidecar a refusal at
  /// ingest; this is the same rule at the other end.
  #[error(
    "refusing to realise {thread}/{path}: canon records it as an opaque attachment but carries no bytes for it, so the realisation would name a file it does not contain"
  )]
  MissingBytes { thread: String, path: String },
  #[error("cannot write the realisation at {path}: {source}")]
  Io {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
}

/// Realise the whole estate as readable files under `root`.
///
/// `root` is a DESTINATION the caller has already named -- see the module note
/// on D42. Nothing here asks what time it is.
pub fn realise(
  project: &Project,
  canon: &Canon,
  ctx: &RenderContext<'_>,
  root: &Path,
) -> Result<Realisation, RealiseError> {
  let mut set = WriteSet::new();
  let mut counts = Counts::default();
  let mut totals = Counts::of(canon);

  // **THE VIEWS COME FROM `views::render_all`, WHICH IS THEIR OWNER.** A second
  // renderer here would be a divergent copy of every view in the estate, and it
  // would diverge in the artefact a human consults when the tool is untrusted.
  // Both the numerator and the denominator for views are asked of it, because
  // a sentence in this file describing what it produces is a fourth list.
  let views = views::render_all(project, canon, ctx);
  totals.views = views.len();
  for view in &views {
    let rel = project.relative(&view.path);
    set.add(root.join("views").join(rel), view.content.clone());
    counts.views += 1;
  }

  // Threads and their work packages, as the authored canon rather than as a
  // rendering of it -- the views above are the readable form, and this is the
  // form that carries every field a view drops.
  for thread in &canon.threads {
    let json = serde_json::to_string_pretty(thread)
      .unwrap_or_else(|e| format!("{{\"unserialisable\": \"{e}\"}}"));
    set.add(root.join("canon").join(format!("{}.json", thread.id)), json);
    counts.threads += 1;
    counts.wps += thread.wps.len();

    for att in &thread.attachments {
      let dest = root
        .join("attachments")
        .join(&thread.id)
        .join(att.path.trim_start_matches('/'));
      // **`as_bytes` IS THE ONE PLACE THE TWO FORMS REJOIN, so this arm never
      // has to know whether it got text or a blob** -- and it cannot get it
      // wrong in one of two branches. An OPAQUE attachment is written as its
      // BYTES: the realisation is a DIRECTORY, so "text" names the intent (a
      // human recovers the estate unaided) rather than a UTF-8 restriction. A
      // stub naming bytes it did not carry would be the lossy-export defect
      // wearing a fallback's clothes.
      let Some(bytes) = att.as_bytes() else {
        return Err(RealiseError::MissingBytes {
          thread: thread.id.clone(),
          path: att.path.clone(),
        });
      };
      write_bytes(&dest, bytes)?;
      counts.attachments += 1;
    }
  }

  for issue in &canon.issues {
    let json = serde_json::to_string_pretty(issue)
      .unwrap_or_else(|e| format!("{{\"unserialisable\": \"{e}\"}}"));
    set.add(
      root
        .join("issues")
        .join(format!("{:04}.json", issue.number)),
      json,
    );
    counts.issues += 1;
  }

  if !set.is_empty() {
    set
      .commit()
      .map_err(|e| RealiseError::Io {
        path: root.to_path_buf(),
        source: std::io::Error::other(e),
      })?
      .keep();
  }

  // Asked of the filesystem rather than accumulated, for the same reason the
  // denominator is asked of canon: a list of what we MEANT to write answers a
  // different question from what is now there, and `Facade::hydrate` shipped
  // exactly that confusion until its own test caught it.
  let written: Vec<PathBuf> = walk(root).into_iter().collect();

  Ok(Realisation {
    root: root.to_path_buf(),
    written,
    counts,
    totals,
  })
}

fn write_bytes(dest: &Path, bytes: &[u8]) -> Result<(), RealiseError> {
  if let Some(parent) = dest.parent() {
    std::fs::create_dir_all(parent).map_err(|source| RealiseError::Io {
      path: parent.to_path_buf(),
      source,
    })?;
  }
  std::fs::write(dest, bytes).map_err(|source| RealiseError::Io {
    path: dest.to_path_buf(),
    source,
  })
}

/// Every file under `root`, in path order.
fn walk(root: &Path) -> BTreeSet<PathBuf> {
  let mut out = BTreeSet::new();
  let mut stack = vec![root.to_path_buf()];
  while let Some(dir) = stack.pop() {
    let Ok(entries) = std::fs::read_dir(&dir) else {
      continue;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        stack.push(path);
      } else {
        out.insert(path);
      }
    }
  }
  out
}

impl crate::remedy::Remedy for RealiseError {
  /// **TWO CAUSES WITH OPPOSITE SUBJECTS, SO TWO REMEDIES.** A missing sidecar
  /// is a repair to the ESTATE -- canon names bytes nobody carries, and the
  /// person who knows where they went is whoever wrote the thread. A write
  /// failure is a repair to the MACHINE. One sentence covering both would send
  /// the operator to the wrong half every other time.
  fn remedy(&self) -> String {
    match self {
      Self::MissingBytes { thread, path } => format!(
        "restore the bytes for `{path}` under `{thread}`, or remove the attachment from canon -- `intent sync --to-store {thread}` takes the disk copy if the file is present. The realisation is the fallback a human reads when the tool cannot help, so it refuses to claim a file it has no bytes for rather than write an empty one."
      ),
      Self::Io { path, .. } => format!(
        "check that `{}` is writable and has space -- the realisation writes a fresh directory per run and never overwrites an earlier one, so a previous realisation is still intact and nothing has been lost.",
        path.display()
      ),
    }
  }
}
