//! **The upgrade's plan** -- the complete set of files an upgraded project
//! holds, structured canon and regenerated views, read from the project's own
//! committed canon, and **written by nobody here**.
//!
//! **The uncommitted return is the atomicity, not a step towards it.** A
//! blocked upgrade must leave the project untouched, and a function that
//! CANNOT write cannot half-write: the guarantee is a property of the signature
//! rather than of anyone getting an early return in the right order. The
//! caller commits the batch, and [`crate::write_set::Applied`] can still undo
//! it if a later step fails.
//!
//! **The order beyond this point is the facade's and it is load-bearing**:
//! plan -> commit the writes -> rebuild the DB -> converge gitignore -> move
//! the canon format -> **stamp the version LAST**, so an upgrade stopped part
//! way still declares the version it had and a re-run plans it again.
//!
//! **It writes no events** (issue 0457): `created` and `completed` are carried
//! in each record itself, and the event log is one committed file per event,
//! which the facade backfills from the store.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::export::{self, Bundle};
use crate::facade::FacadeContext;
use crate::finding::{Finding, FindingClass, Refusal};
use crate::ingest::Canon;
use crate::model::{Issue, Thread};
use crate::project::Project;
use crate::views::{self, RenderContext};
use crate::write_set::WriteSet;

/// Everything the upgrade would do, with nothing done yet.
#[derive(Debug)]
pub struct Plan {
  /// Canon and views, **uncommitted**. The caller commits it.
  pub writes: WriteSet,
  /// The model, for the store rebuild the facade runs after the writes land.
  pub threads: Vec<Thread>,
  pub issues: Vec<Issue>,
}

/// Why no plan exists.
#[derive(Debug, thiserror::Error)]
pub enum Blocked {
  /// A record in the committed canon that cannot be read, so the upgrade would
  /// rebuild the store and the views without it.
  ///
  /// **Carried as a [`Refusal`] rather than a bare `Vec<Finding>`** so the
  /// per-line classed report arrives with the error instead of being
  /// re-implemented at whichever door renders it. Not `#[source]`, for
  /// the reason [`crate::ingest::IngestError::Refused`] records: a source here
  /// renders the whole list twice and every residue count reads double.
  #[error("{0}")]
  Residue(Refusal),
  /// `intent/.canon/project.json` is present and unreadable, so the DONE
  /// watermark it carries cannot be honoured (issue 0485).
  ///
  /// **Refused rather than rendered without it**, because rendering without it
  /// is the defect: on a project that has flushed, `todo.md` comes out with
  /// every finished thread back in DONE and `doctor` then reports the
  /// upgrade's own output as a hand edit. `doctor` reads a damaged file as
  /// absent because it must still diagnose; a writer must not.
  #[error("{0}")]
  ProjectState(Refusal),
  /// The model would not serialise.
  ///
  /// **A defect in Intent, not in the estate, and it says so** -- everything
  /// here was built in memory from data that already parsed, so there is no
  /// artefact for an operator to go and repair. Reported rather than unwrapped
  /// because a panic in the middle of a migration is the one failure mode with
  /// no message and no remedy.
  #[error("the migration's own canon could not be encoded")]
  Canon {
    #[source]
    source: crate::canon_format::CanonFormatError,
  },
  /// Two artefacts resolve to one file.
  ///
  /// **Nothing below this guards it today.** [`WriteSet`] applies its entries
  /// in order, so a duplicated path writes both and keeps the last -- one
  /// artefact silently overwriting another, inside the operation whose whole
  /// promise is that nothing disappears. It would land as a clean migration
  /// with a file missing and every count reconciling.
  ///
  /// **The detection belongs in [`WriteSet::add`] and cc is taking it there**
  /// (2026-08-17): every caller needs uniqueness, so a set that permits
  /// duplicates is wrong on its own terms rather than wrong for this one use.
  /// The variant stays here regardless -- `WriteSet` can say the batch is
  /// malformed and only the join can say WHICH artefacts collided -- and the
  /// scan below is deleted the day `add` becomes fallible, so the two do not
  /// settle into one check nobody can reach.
  #[error("two artefacts would be written to {path}")]
  Collision { path: String },
  /// Two artefacts whose paths differ only in case, which a case-insensitive
  /// disk (macOS's default) stores as one file (issue 0711, cc's ruling of
  /// 2026-11-19).
  ///
  /// **The same guard as [`Blocked::Collision`], asking the one-file key**,
  /// and refused on every file system, because canon travels by git and the
  /// pair collides on the next clone made on a Mac. Its pairs are attachments,
  /// text or opaque, each keyed at the path realising it writes, or an
  /// attachment beside a view. `path` is the first written and `other` the one
  /// that would overwrite it, each as written.
  #[error(
    "two artefacts would be written to one file on a case-insensitive disk: {path} and {other}"
  )]
  CaseCollision { path: String, other: String },
  /// The canon is short of one file per artefact.
  ///
  /// **Counted from the model rather than from the exporter's output**, which
  /// is the only way the check can disagree with the thing it is checking: one
  /// `thread.json` per thread, one `<n>.json` per issue. An
  /// equality derived from [`export::canon_parts`] would agree with
  /// `canon_parts` by construction and report a silent drop as correct.
  #[error("the canon carries {actual} file(s) for {expected} artefact(s)")]
  Conservation { expected: usize, actual: usize },
  /// `.intentfiles` is present and will not parse, so its rows cannot be
  /// folded into the records (ST0096 WP-01). **Refused, never read fail-open**:
  /// the fold removes the file, so a row it could not read would be lost.
  #[error("{path} does not parse, so the upgrade cannot fold it into the records: {cause}")]
  Manifest {
    path: String,
    cause: crate::intentfiles::IntentfilesError,
  },
  /// `.intentfiles` is present and cannot be read, so the fold cannot say
  /// what it declares (ST0096 WP-01).
  #[error("{path} could not be read, so the upgrade cannot fold it into the records")]
  ManifestUnreadable {
    path: String,
    #[source]
    source: std::io::Error,
  },
  /// An attachment of a thread this upgrade leaves unrealised holds bytes on
  /// disk its canon does not record, so removing the thread's views would
  /// lose an edit never synced (ST0096 WP-05, AC-05.3). A realised thread's
  /// attachment stays on disk untouched, so only these are asked.
  #[error(
    "{path} differs from the attachment its thread's canon records, and the upgrade would remove it with the thread's other files"
  )]
  DirtyAttachment { path: String },
  /// An attachment of a thread this upgrade leaves unrealised could not be read,
  /// so whether it holds an edit never synced is unanswered, and unanswered is
  /// refused, as organize refuses it.
  #[error(
    "{path} could not be read, so whether it holds an edit its thread's canon does not record is unknown"
  )]
  AttachmentUnreadable {
    path: String,
    #[source]
    source: std::io::Error,
  },
  /// Organize's view of the tree, which the unsynced-attachment check asks
  /// (issue Idf2dd7c), could not be read: the walk, or whether the disk folds
  /// case. Unanswered is refused, as organize refuses it.
  #[error("the tree could not be read to check the unrealised threads' attachments: {source}")]
  TreeUnread {
    #[source]
    source: crate::organize::OrganizeError,
  },
}

impl crate::remedy::Remedy for Blocked {
  fn remedy(&self) -> String {
    match self {
      // The findings above have each named their own artefact and action;
      // `Refusal` already says so in the words the rest of the tool uses.
      Self::Residue(refusal) | Self::ProjectState(refusal) => {
        crate::remedy::Remedy::remedy(refusal)
      }
      // The encoder's own remedy, which tells a record that would not serialise
      // (a defect to report) from two of a thread's files that would be one
      // (a rename the operator makes).
      Self::Canon { source } => crate::remedy::Remedy::remedy(source),
      Self::Collision { path } => format!(
        "two artefacts in this project's canon resolve to {path}; nothing was written. Rename or remove one of them, then run `intent upgrade` again"
      ),
      Self::CaseCollision { path, other } => format!(
        "{path} and {other} differ only in case, so a case-insensitive disk (macOS's default) stores them as one file and one would overwrite the other; nothing was written. Give the authored file among them another name (either one, when both are authored), then run `intent upgrade` again"
      ),
      Self::Manifest { cause, .. } => crate::remedy::Remedy::remedy(cause),
      Self::ManifestUnreadable { path, .. } => format!(
        "make {path} readable (the cause line names why it is not), then run `intent upgrade` again; nothing was written"
      ),
      Self::DirtyAttachment { path } => format!(
        "nothing was written. Keep the edit with `intent sync --to-store`, which takes {path} into the store, or restore the file from git if the canon is right, then run `intent upgrade` again"
      ),
      Self::AttachmentUnreadable { path, .. } => format!(
        "nothing was written. Make {path} readable (the cause line names why it is not), then run `intent upgrade` again"
      ),
      Self::TreeUnread { source } => format!(
        "nothing was written. {}, then run `intent upgrade` again",
        crate::remedy::Remedy::remedy(source)
      ),
      Self::Conservation { expected, actual } => format!(
        "{} artefact(s) produced no canon; nothing in the project caused this and a re-run will reproduce it -- report it",
        expected.saturating_sub(*actual)
      ),
    }
  }
}

/// **Every thread in the committed canon**, read through the one canon reader.
///
/// Under ST0057 the disk is a SPARSE PROJECTION: a dehydrated thread has canon
/// and no directory, so the population is the canon's, never the disk's
/// (measured 2026-08-20: an upgrade that read only the realised threads cut a
/// 57-thread store to 3, at rc=0, under an `ok:`).
///
/// # An unreadable canon REFUSES rather than being skipped
///
/// A skipped file would leave the population silently incomplete, and the
/// rebuild would run off it.
fn canon_threads(project: &Project) -> Result<Vec<Thread>, Blocked> {
  let Ok(ids) = project.thread_ids() else {
    return Ok(Vec::new());
  };
  let mut threads = Vec::new();
  let mut residue = Vec::new();
  for id in ids {
    let path = project.thread_json(&id);
    let rel = project.relative(&path);
    match std::fs::read_to_string(&path) {
      Ok(text) => match crate::ingest::read_thread(project, &id, &text) {
        Ok(thread) => threads.push(thread),
        Err(mut found) => residue.append(&mut found),
      },
      Err(source) => residue.push(Finding::new(
        &rel,
        FindingClass::BrokenReference,
        format!(
          "canon names {} and the file is not readable: {source} -- the upgrade would rebuild \
           the store and the views without it",
          id.as_str()
        ),
      )),
    }
  }
  if !residue.is_empty() {
    return Err(Blocked::Residue(Refusal::new(residue)));
  }
  Ok(threads)
}

/// **Every issue in the committed canon**, [`canon_threads`]'s counterpart.
///
/// Its absence was intent#0070: an upgrade that topped up threads from canon
/// and not issues replaced a populated issue table with nothing, at rc=0,
/// under an `ok:`. An unreadable or invalid canon REFUSES rather than being
/// skipped, for [`canon_threads`]'s reason.
fn canon_issues(project: &Project) -> Result<Vec<Issue>, Blocked> {
  let Ok(numbers) = project.issue_numbers() else {
    return Ok(Vec::new());
  };
  let mut issues = Vec::new();
  let mut residue = Vec::new();
  for number in numbers {
    let path = project.issue_json(number);
    let rel = project.relative(&path);
    match std::fs::read_to_string(&path) {
      Ok(text) => match crate::ingest::read_issue(project, number, &text) {
        Ok(issue) => issues.push(issue),
        Err(mut found) => residue.append(&mut found),
      },
      Err(source) => residue.push(Finding::new(
        &rel,
        FindingClass::BrokenReference,
        format!(
          "canon names issue {} and the file is not readable: {source} -- the upgrade \
           would rebuild the store and the views without it",
          number.shown()
        ),
      )),
    }
  }
  if !residue.is_empty() {
    return Err(Blocked::Residue(Refusal::new(residue)));
  }
  Ok(issues)
}

/// The batch an upgrade would write, read from the project's committed canon.
///
/// Everything assembles: canon from the release's encoder, views from
/// [`views::render_all`], both returned UNCOMMITTED. A record that does not
/// read blocks.
pub fn plan(project: &Project, ctx: &FacadeContext) -> Result<Plan, Blocked> {
  let mut threads = canon_threads(project)?;
  let mut issues = canon_issues(project)?;
  // The canon may carry hashed records beside ordinals, so both populations
  // are held in the one order (ST0095 WP-08).
  threads.sort_by(crate::id::record_order);
  issues.sort_by(crate::id::record_order);
  assemble(project, ctx, threads, issues)
}

/// The whole of the plan's work, over a model.
///
/// Apart from [`plan`] because reading the canon and assembling from a model
/// are different jobs, and the tests here reach the assembly with a model of
/// their own rather than by writing canon first.
fn assemble(
  project: &Project,
  ctx: &FacadeContext,
  threads: Vec<Thread>,
  issues: Vec<Issue>,
) -> Result<Plan, Blocked> {
  // **THE WATERMARK COMES FROM CANON'S PROJECT STATE, THE AUTHORITY
  // `Facade::doctor` READS** (issue 0485). This was `None`, on the reasoning
  // that a conversion has never been flushed -- false of `intent upgrade`
  // re-run on a project that HAS flushed, and a project with no
  // `project.json` still reads `None` here. With `None` it wrote
  // every finished thread back into DONE and doctor refused the upgrade's own
  // `todo.md`: measured on a clone of Lamplight at 884f2e3ae, 122958 bytes
  // written against 24247 rendered.
  let todo_watermark = crate::ingest::read_project_state(project)
    .map_err(|findings| Blocked::ProjectState(Refusal::new(findings)))?
    .and_then(|state| state.todo_watermark);
  // The format the upgrade WRITES, one binding for the views and the records'
  // files below, so a cover renders in the format its records are written in.
  // Named apart from `format` below, the project's own on-disk format, which
  // decides how the manifest is read and nothing that is written.
  let target = crate::canon_format::CanonFormat::CURRENT;
  let ctx_render = RenderContext {
    version: &ctx.version,
    todo_watermark,
    devbin: crate::rootfiles::has_devbin(project.root()),
    canon_format: target,
  };

  // `render_all` reads only `threads`, and `sections` is authored prose that
  // stays on disk as files (export.rs: "authored prose is OUT").
  let canon = Canon {
    threads,
    issues,
    sections: Vec::new(),
    boards: Vec::new(),
    declaration: None,
  };
  // **TWO POPULATIONS, AND CONFLATING THEM IS HOW THE FIRST CUT OF THE UNION
  // TRADED ONE DEFECT FOR ANOTHER.** The STORE and the project-level index must
  // cover every thread canon holds; the per-thread files on disk must cover
  // only what `.intentfiles` declares. `render_all` renders both kinds, so the
  // union above -- which is right for the model -- hydrated all 57 threads onto
  // a disk whose manifest declares 3. Measured: 326 files written against 38,
  // and 54 untracked directories appearing under an `ok:`.
  //
  // **NOTHING WAS CONSULTING `.intentfiles` HERE BEFORE, AND IT LOOKED
  // CORRECT.** The scan's disk-walk population happened to EQUAL the declared
  // population -- both were "the threads with directories" -- so the migrator
  // respected the manifest by coincidence. Breaking the coincidence is what
  // showed there was no mechanism.
  //
  // **`Realised::declares` answers TRUE for an absent or unreadable manifest**
  // (ABSENT IS NOT EMPTY, hv). That rule is untouched and stays right for every
  // other reader -- but THIS caller is about to WRITE the manifest, so for it
  // "nobody has said" is not the end of the question. `declare_default_if_absent`
  // runs minutes later in `Facade::upgrade` and declares the WIP set; realising
  // everything here and declaring a subset there is what left six of ten
  // realised thread directories in Intent's own estate non-WIP.
  //
  // **SO THE MIGRATION FILTERS AGAINST THE BYTES IT IS ABOUT TO WRITE, NOT
  // AGAINST A SECOND PREDICATE.** `default_declaration` is the ONE definition
  // of what the default declares (AC-11.3); spelling `status == Wip` again here
  // would be a second home for it, which is the exact defect that criterion
  // exists to forbid -- and it would arrive through the migrator, where nobody
  // is looking. Reading the declaration back through `realised_from` means the
  // declared set and the realised set cannot disagree, because they are the
  // same text.
  //
  // **`Unreadable` IS LEFT FAIL-OPEN DELIBERATELY.** A manifest that exists and
  // will not parse is somebody having said something we cannot read, and
  // `declare_default_if_absent` will not overwrite it either -- so realising
  // everything keeps this door and that one agreeing.
  //
  // # What this leaves behind, measured rather than assumed
  //
  // `WriteSet` has no remove, so an undeclared thread's directory stays on
  // disk carrying the views an earlier run wrote: undeclared, and no longer
  // refreshed until the facade dehydrates them through organize's own gate.
  // **That state is AC-11.4's, not a hole** -- `organize` (preview) reports it
  // as to-remove-and-blocked with every pre-existing file intact, driven on a
  // fixture 2026-08-27: `0 to remove (2 blocked)`. And a STALE undeclared view
  // is still caught: `views::skew_against` walks `render_all` over all canon and
  // consults `Realised` only in its file-ABSENT arm, so an undeclared file that
  // differs from the model reports `view-skew` -- driven on the same fixture,
  // against a control where the identical undeclared thread with a CURRENT file
  // reported nothing. vc made that measurement the condition of lifting the
  // hold this clause sat under, on the ground that a check narrower than the
  // defect it is aimed at is the class that has cost this estate most.
  //
  // # The fold (ST0096 WP-01, R1)
  //
  // The realised set is then written onto the records, which the plan writes
  // in the release's format: a listed record carries `realised: true` and any
  // other record carries no field, so the fold maps the manifest exactly and
  // two branches upgraded apart write the same bytes. `Facade::upgrade` then
  // removes the manifest. A project an earlier upgrade moved already carries
  // the field and has no manifest, so a re-run keeps what its records say.
  // An unreadable manifest is refused here, not read fail-open: the fold
  // removes the file, so a row it could not read would be lost.
  let format = crate::ingest::canon_format(project)
    .map_err(|findings| Blocked::ProjectState(Refusal::new(findings)))?;
  let realised = match format {
    crate::canon_format::CanonFormat::V4 => {
      crate::intentfiles::declared_by_records(&canon.threads, &canon.issues)
    }
    crate::canon_format::CanonFormat::V3 => {
      match std::fs::read_to_string(project.intentfiles_path()) {
        Ok(text) => {
          crate::intentfiles::realised_for_action(&text).map_err(|cause| Blocked::Manifest {
            path: project.relative(&project.intentfiles_path()),
            cause,
          })?
        }
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
          crate::intentfiles::default_for(&canon.threads, &canon.issues)
        }
        Err(source) => {
          return Err(Blocked::ManifestUnreadable {
            path: project.relative(&project.intentfiles_path()),
            source,
          });
        }
      }
    }
  };
  let mut canon = canon;
  crate::intentfiles::fold_onto(&realised, &mut canon.threads, &mut canon.issues);

  // **AN UNSYNCED ATTACHMENT EDIT IS REFUSED BEFORE ANY WRITE** (ST0096 WP-05,
  // AC-05.3; cc's meaning). An unrealised thread's attachment that holds an
  // edit never synced would be lost when its views go, so it is compared with
  // the canon before anything moves; a file the check cannot read is
  // unanswered, and unanswered is refused, as organize refuses it. A case pair
  // among a thread's attachments is issue 0711's guard's, below.
  let unrealised_threads: Vec<&Thread> = canon
    .threads
    .iter()
    .filter(|t| !realised.declares(&t.id))
    .collect();
  //
  // **ASKED OF ORGANIZE, ON ORGANIZE'S TREE** (issue Idf2dd7c). This was a
  // second copy of organize's gate, comparing shas by itself and reading only
  // the row's spelling. Which file is the row's is `organize::row_file`'s
  // answer over `organize::observe`'s tree, the tree and the resolver the
  // removal that follows the landing uses (issue 0722: on a disk that folds
  // case a file spelled in another case is the row's working copy, gated and
  // named by its own spelling; on one that does not it is a case pair nothing
  // removes). Whether it may go is `organize::gate`'s. A file that is not there
  // has nothing to lose. An opaque attachment's bytes are its sidecar's where
  // the row does not already carry them.
  if unrealised_threads.iter().any(|t| !t.attachments.is_empty()) {
    let (tree, _) =
      crate::organize::observe(project, &[]).map_err(|source| Blocked::TreeUnread { source })?;
    let keyed = crate::project::DiskKeys::new(&tree.present, project.root());
    for thread in &unrealised_threads {
      let mut attachments = (*thread).clone();
      let unloaded = crate::ingest::load_blobs(project, &mut attachments);
      if !unloaded.is_empty() {
        return Err(Blocked::ProjectState(Refusal::new(unloaded)));
      }
      let dir = project.thread_dir(&thread.id);
      for attachment in &attachments.attachments {
        let Some(path) = crate::organize::row_file(
          dir.join(&attachment.path),
          &tree.present,
          &keyed,
          tree.folds,
        )
        .filter(|path| tree.present.contains(path)) else {
          continue;
        };
        let step = crate::organize::Step {
          path,
          action: crate::organize::Action::Dehydrate,
          content: attachment.as_bytes().map(<[u8]>::to_vec),
        };
        crate::organize::gate(&step, &[]).map_err(|refused| match refused {
          crate::organize::OrganizeError::HandEdited { .. } => Blocked::DirtyAttachment {
            path: project.relative(&step.path),
          },
          crate::organize::OrganizeError::Io { source, .. } => Blocked::AttachmentUnreadable {
            path: project.relative(&step.path),
            source,
          },
          other => Blocked::TreeUnread { source: other },
        })?;
      }
    }
  }

  let unrealised: Vec<PathBuf> = unrealised_threads
    .iter()
    .map(|t| project.thread_dir(&t.id))
    .collect();
  // **AN UNDECLARED ISSUE IS NOT REALISED EITHER** (issue 0320). The filter
  // above answers for threads only, so every converted issue's view was
  // written -- a CLOSED one included, which the default declaration leaves out
  // -- and the next `organize` unclaimed and removed it. Asked through the one
  // predicate the projection and the skew check use, so the three cannot
  // disagree about which issue is realised.
  let views: Vec<views::View> = views::render_all(project, &canon, &ctx_render)
    .into_iter()
    .filter(|v| !unrealised.iter().any(|dir| v.path.starts_with(dir)))
    .filter(|v| {
      !matches!(
        views::undeclared_owner(project, &v.path, &canon, &realised),
        Some(views::Undeclared::Issue { .. })
      )
    })
    .collect();
  let Canon {
    threads,
    issues,
    sections: _,
    boards: _,
    declaration: _,
  } = canon;

  // One canon file per record (ST0096 WP-05, cc's meaning): each thread's own,
  // one per work package, criterion, test and attachment, and one per issue --
  // stated from the model, before the encoder is asked anything, so a record
  // the encoder dropped is counted missing. No event log (issue 0457).
  let expected = threads
    .iter()
    .map(|t| 1 + t.wps.len() + t.criteria.len() + t.tests.len() + t.attachments.len())
    .sum::<usize>()
    + issues.len();

  // `project_id` is empty on a pre-migration project and `Bundle` records that
  // honestly rather than inventing one.
  //
  // **THIS COMMENT USED TO END `the facade mints and stamps it, last`, AND
  // THAT WAS NEVER TRUE** -- not stale, never true. `stamp_version` inserted
  // `intent_version` and nothing else, so the sentence promised a fix, in the
  // one place a reader tracing the empty id would arrive, and sent them away
  // satisfied. It is true as of this commit; it is written here as history
  // because the failure was the PROMISE, and a corrected promise reads exactly
  // like the one that was wrong.
  //
  // **The empty id is inert HERE regardless, which is the part worth knowing
  // and is why the stamp belongs at the facade rather than earlier**:
  // `canon_parts` reads `bundle.threads` and `bundle.issues`, and never
  // `bundle.project_id`, so nothing this bundle emits carries the id at all.
  // Migration passes `Vec::new()` for events, which `canon_parts` does not read.
  // **EVERY RECORD IN THE RELEASE'S CANON FORMAT** (ST0095 WP-06): this is
  // the rewrite `intent upgrade` makes once, deterministic, so two clones
  // upgraded apart write the same bytes. It changes no id. The one file whose
  // schema says the format, `project.json`, is written by the facade last.
  //
  // **EACH THREAD IS SPLIT INTO ITS RECORDS' FILES** (ST0096 WP-05, AC-05.3)
  // through the one encoder of a thread's layout, so two branches upgraded
  // apart write the same files and a second run writes nothing. Each issue is
  // its one file, through the one encoder of an issue.
  //
  // **A THREAD'S RECORDS ARE WRITTEN BEFORE ITS OWN FILE** (cc, 2026-11-20).
  // `ingest` reads a thread's children by the format of its own file, and the
  // write set commits in the order it is given, so an upgrade cut short leaves
  // each thread either with its 3.4.0 file whole or with a release-format own
  // file whose records are all on disk; a re-run rewrites any stray record.
  // The facade writes `project.json` after the whole set, as it always has.
  let bundle = Bundle::new(&ctx.project_id, threads, issues, Vec::new());
  // **EVERY ATTACHMENT, NOT ONLY THE BYTES THIS BATCH CARRIES** (issue 0711).
  // A text attachment rides inline in its thread's canon, and a thread read
  // back from canon carries no sidecar bytes, so neither reaches the batch as a
  // file of its own. Realising the thread writes each of them to its own path
  // in the thread's directory, so each is keyed there.
  let attachment_paths: Vec<PathBuf> = bundle
    .threads
    .iter()
    .flat_map(|thread| {
      let dir = project.thread_dir(&thread.id);
      thread
        .attachments
        .iter()
        .map(move |attachment| dir.join(&attachment.path))
    })
    .collect();
  // **THE ATTACHMENTS ARE KEYED BEFORE ANY RECORD IS ENCODED** (ST0096 WP-05,
  // cc's ruling (a)). In the release's layout an attachment's record file and
  // its bytes are both built from its path, so a pair the key equates is a pair
  // on each of them; asked here first, the refusal names the attachments
  // themselves, and the encoder's own check never meets a pair in an upgrade.
  one_file_each(project, &attachment_paths)?;
  let mut parts: Vec<(String, String)> = Vec::with_capacity(expected);
  for thread in &bundle.threads {
    let files = crate::canon_format::encode_thread_files(thread, target)
      .map_err(|source| Blocked::Canon { source })?;
    let own = crate::project::canon_thread_rel(&thread.id.stored());
    let (own, records): (Vec<_>, Vec<_>) = files.into_iter().partition(|(rel, _)| *rel == own);
    parts.extend(records);
    parts.extend(own);
  }
  for issue in &bundle.issues {
    parts.push((
      crate::project::canon_issue_rel(issue.number),
      crate::canon_format::encode_issue(issue, target)
        .map_err(|source| Blocked::Canon { source })?,
    ));
  }
  if parts.len() != expected {
    return Err(Blocked::Conservation {
      expected,
      actual: parts.len(),
    });
  }

  // `canon_parts` names its files relative to the intent directory so a
  // refusal can quote a path an operator recognises; the views already carry
  // absolute paths from `Project`.
  let intent_dir = project.intent_dir();
  let mut batch: Vec<(PathBuf, String)> = Vec::with_capacity(parts.len() + views.len());
  batch.extend(
    parts
      .into_iter()
      .map(|(rel, content)| (intent_dir.join(rel), content)),
  );
  batch.extend(views.into_iter().map(|view| (view.path, view.content)));
  // **THE BYTES THE CANON NAMES (0084).** An opaque attachment's canon entry
  // is its hash and size; its bytes are a sidecar, and a migration that wrote
  // the entry and not the sidecar left canon naming a file that did not exist.
  let blobs: Vec<(PathBuf, Vec<u8>)> = export::canon_blobs(&bundle)
    .into_iter()
    .map(|(rel, raw)| (intent_dir.join(rel), raw))
    .collect();

  // Every path the batch writes, and every attachment's realised path again,
  // through the same guard: a record file or a view can meet another only here.
  one_file_each(
    project,
    batch
      .iter()
      .map(|(p, _)| p)
      .chain(blobs.iter().map(|(p, _)| p))
      .chain(attachment_paths.iter()),
  )?;

  // The opaque bytes first, so no canon file is on disk before the bytes it
  // names, and then the batch in its order: each thread's records before its
  // own file.
  let mut writes = WriteSet::new();
  for (path, raw) in blobs {
    writes.add_bytes(path, raw);
  }
  for (path, content) in batch {
    writes.add(path, content);
  }

  Ok(Plan {
    writes,
    threads: bundle.threads,
    issues: bundle.issues,
  })
}

/// ONE FILE ON SOME DISK IS ONE FILE (issue 0711, cc's ruling): the first two
/// of `paths` that the one-file key equates are refused, before anything is
/// written, on every file system. What the key equates, and what it does not,
/// is its own doc's to say.
fn one_file_each<'a>(
  project: &Project,
  paths: impl IntoIterator<Item = &'a PathBuf>,
) -> Result<(), Blocked> {
  let mut seen: BTreeMap<String, &PathBuf> = BTreeMap::new();
  for path in paths {
    if let Some(first) = seen.insert(crate::project::one_file_key(path), path) {
      // `relative` spells a path by its components, so two spellings of one
      // file on every disk (`a/./b.png` and `a/b.png`) read alike here, and
      // what is left to tell two keyed-alike paths apart is case.
      let (first, path) = (project.relative(first), project.relative(path));
      return Err(if first == path {
        Blocked::Collision { path }
      } else {
        Blocked::CaseCollision {
          path: first,
          other: path,
        }
      });
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::THREAD_SCHEMA;

  const VERSION: &str = "3.0.0-test";

  fn project(dir: &std::path::Path) -> Project {
    let config = dir.join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir");
    std::fs::write(
      config.join("config.json"),
      r#"{"intent_version":"3.0.0","intent_dir":"intent"}
"#,
    )
    .expect("config");
    Project::discover(dir).expect("discover")
  }

  fn ctx() -> FacadeContext {
    FacadeContext::for_tests("ic".to_string(), String::new(), VERSION.to_string())
  }

  /// A model to assemble, so a test names only the records it cares about.
  #[derive(Default)]
  struct Model {
    threads: Vec<Thread>,
    issues: Vec<Issue>,
  }

  fn assemble_model(project: &Project, ctx: &FacadeContext, model: Model) -> Result<Plan, Blocked> {
    assemble(project, ctx, model.threads, model.issues)
  }

  fn thread(id: &str) -> Thread {
    serde_json::from_value(serde_json::json!({
      "schema": THREAD_SCHEMA,
      "id": id,
      "title": format!("{id} title"),
      "status": "wip",
      "created": "2026-08-17",
    }))
    .expect("thread fixture")
  }

  fn wp(seq: u32) -> crate::model::WorkPackage {
    serde_json::from_value(serde_json::json!({
      "id": crate::id::WpId::from_ordinal(seq).expect("an ordinal that fits").stored(),
      "title": crate::id::WpId::from_ordinal(seq).expect("an ordinal that fits").heading(),
      "status": "not-started",
    }))
    .expect("wp fixture")
  }

  /// **Built through serde rather than as a struct literal, deliberately.**
  /// `Issue` is gaining `reporter` (cc, 2026-08-17); a literal here would red
  /// their commit from this file, and a fixture that breaks on a field it does
  /// not care about is a fixture that gets weakened to stop breaking.
  fn issue(number: u32) -> Issue {
    serde_json::from_value(serde_json::json!({
      "schema": crate::model::ISSUE_SCHEMA,
      "id": crate::id::IssueId::from_ordinal(number).expect("an ordinal that fits").stored(),
      "slug": format!("issue-{number}"),
      "title": format!("issue {number}"),
      "status": "closed",
      "created": "2026-08-17",
    }))
    .expect("issue fixture")
  }

  /// A record in the committed canon that does not read blocks the plan, and
  /// the refusal names its file.
  #[test]
  fn an_unreadable_canon_record_blocks_and_the_report_names_it() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let canon = project.thread_json("ST0001");
    std::fs::create_dir_all(canon.parent().expect("a parent")).expect("mkdir");
    std::fs::write(&canon, "not canon").expect("write");

    let Err(Blocked::Residue(refusal)) = plan(&project, &ctx()) else {
      panic!("a record that does not read must block");
    };
    let report = refusal.to_string();
    assert!(report.contains("ST0001"), "report: {report}");
  }

  #[test]
  fn a_clean_model_plans_canon_and_views_and_still_writes_nothing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let before = tree(dir.path());

    let planned = assemble_model(
      &project,
      &ctx(),
      Model {
        threads: vec![thread("ST0001"), thread("ST0002")],
        issues: vec![issue(46)],
      },
    )
    .expect("a clean estate plans");

    // Canon: two threads and one issue, and no event log (issue 0457). Views: info + acceptance
    // per thread, plus the index and the todo view. The fixture's issue is
    // CLOSED, and since 0320 a closed issue realises no view, so it plans none.
    assert_eq!(planned.threads.len(), 2);
    assert_eq!(planned.issues.len(), 1);
    assert_eq!(planned.writes.len(), 3 + 6);
    // **The equality below is worthless if `tree` sees nothing**, and a
    // before/after comparison of two empty vectors passes for any behaviour.
    // This is what makes the next line an assertion rather than a shape.
    assert_eq!(
      before,
      vec!["intent/.config/config.json".to_string()],
      "the walker must be able to see a file that IS there"
    );
    assert_eq!(
      before,
      tree(dir.path()),
      "planning is read-only and this is the assertion that says so"
    );
  }

  #[test]
  fn two_artefacts_claiming_one_path_are_refused_rather_than_overwritten() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let Err(Blocked::Collision { path }) = assemble_model(
      &project,
      &ctx(),
      Model {
        threads: vec![thread("ST0001"), thread("ST0001")],
        ..Default::default()
      },
    ) else {
      panic!("a duplicated id must not silently overwrite");
    };
    assert!(
      path.contains("ST0001"),
      "the refusal names the file: {path}"
    );
  }

  #[test]
  fn a_duplicated_issue_number_is_caught_by_the_same_guard() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let Err(Blocked::Collision { path }) = assemble_model(
      &project,
      &ctx(),
      Model {
        issues: vec![issue(46), issue(46)],
        ..Default::default()
      },
    ) else {
      panic!("two issues on one number must not silently overwrite");
    };
    assert!(path.contains("46"), "the refusal names the file: {path}");
  }

  /// ST0001 carrying `attachments`, with the spelling their author gave them.
  fn carrying(attachments: Vec<crate::model::Attachment>) -> Model {
    let mut carrier = thread("ST0001");
    carrier.attachments = attachments;
    Model {
      threads: vec![carrier],
      ..Default::default()
    }
  }

  /// Two opaque attachments, which reach the guard as sidecar files.
  fn opaque_pair(first: &str, second: &str) -> Model {
    carrying(vec![
      crate::model::Attachment::opaque(first, b"first".to_vec()),
      crate::model::Attachment::opaque(second, b"second".to_vec()),
    ])
  }

  /// Issue 0711: two paths that differ only in case are one file on a
  /// case-insensitive disk, which is macOS's default, so there the second write
  /// would overwrite the first. The refusal names the first written, then the
  /// one that would overwrite it, each as written. The pair is given out of
  /// byte order, so a guard that sorted it would answer the other way round.
  #[test]
  fn two_paths_that_differ_only_in_case_are_refused_as_one_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let Err(refusal) = assemble_model(&project, &ctx(), opaque_pair("design.png", "Design.png"))
    else {
      panic!(
        "design.png and Design.png are one file on a case-insensitive disk, so they must not both be written"
      );
    };
    let Blocked::CaseCollision { path, other } = &refusal else {
      panic!("a pair that differs only in case is refused as a case collision, not as: {refusal}");
    };
    assert!(
      path.ends_with("ST0001/design.png") && other.ends_with("ST0001/Design.png"),
      "the first written, then the one that would overwrite it: {path}, {other}"
    );
    let remedy = crate::remedy::Remedy::remedy(&refusal);
    assert!(
      remedy.contains("differ only in case"),
      "the remedy says why two names are one file: {remedy}"
    );
  }

  /// The key is built from components, as the exact compare it replaced was:
  /// `a/./b.png` and `a/b.png` are one file on every disk, so the refusal is the
  /// plain collision, naming the one file, with its own remedy.
  #[test]
  fn two_spellings_of_one_path_are_refused_as_one_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let Err(refusal) = assemble_model(&project, &ctx(), opaque_pair("a/./b.png", "a/b.png")) else {
      panic!("a/./b.png and a/b.png are one file on every disk, so they must not both be written");
    };
    let Blocked::Collision { path } = &refusal else {
      panic!("two spellings of one file are refused as one path, not as: {refusal}");
    };
    assert!(
      path.ends_with("ST0001/a/b.png"),
      "the refusal names the one file: {path}"
    );
    let remedy = crate::remedy::Remedy::remedy(&refusal);
    assert!(
      remedy.contains("resolve to"),
      "the remedy says the two artefacts resolve to one file: {remedy}"
    );
  }

  /// A text attachment rides inline in its thread's canon, so no sidecar of
  /// its own reaches the batch; realising the thread writes it, so a text pair
  /// that differs only in case is refused too.
  #[test]
  fn two_text_attachments_that_differ_only_in_case_are_refused_as_one_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let scan = carrying(vec![
      crate::model::Attachment::new("Notes.md", "first"),
      crate::model::Attachment::new("notes.md", "second"),
    ]);

    let Err(refusal) = assemble_model(&project, &ctx(), scan) else {
      panic!("Notes.md and notes.md are one file once realised on a case-insensitive disk");
    };
    let Blocked::CaseCollision { path, other } = &refusal else {
      panic!(
        "a text pair that differs only in case is refused as a case collision, not as: {refusal}"
      );
    };
    assert!(
      path.ends_with("st/ST0001/Notes.md") && other.ends_with("st/ST0001/notes.md"),
      "the refusal names both realised paths, in the order written: {path}, {other}"
    );
  }

  #[test]
  fn every_planned_write_lands_under_the_project() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let planned = assemble_model(
      &project,
      &ctx(),
      Model {
        threads: vec![thread("ST0001")],
        issues: vec![issue(1)],
      },
    )
    .expect("plan");

    // The batch is private to `WriteSet`, so this proves the property the only
    // way available from outside: commit it into a throwaway tree and look at
    // what appeared. It is also the one test that exercises the whole path.
    let count = planned.writes.len();
    planned.writes.commit().expect("commit").keep();
    let after = tree(dir.path());

    // **Named, not counted.** A count assertion passes for any set of the
    // right size, and the property at stake is that the canon landed at the
    // path every later reader resolves -- `Project::thread_json` and
    // `Project::issue_json`, which is where the exporter's unpadded issue
    // spelling would have shown up as a file nothing can find.
    for expected in [
      project.thread_json("ST0001"),
      project.info_view("ST0001"),
      project.acceptance_view("ST0001"),
      project.issue_json(crate::id::IssueId::from_ordinal(1).expect("an issue id")),
      project.steel_threads_view(),
      project.todo_view(),
    ] {
      let rel = project.relative(&expected);
      assert!(after.contains(&rel), "missing {rel} in {after:?}");
    }
    assert_eq!(
      after.len(),
      count + 1,
      "everything planned landed, and nothing else did (+1 is the config that was already there)"
    );
    for path in &after {
      assert!(
        path.starts_with("intent/"),
        "a migration wrote outside intent/: {path}"
      );
    }
  }

  /// **A thread WITH work packages, because `st/<ID>/WP/<NN>/info.md` is the
  /// other zero-padded path in this estate and the issue canon has just shown
  /// what a padding disagreement costs.**
  ///
  /// Every other fixture here has no WPs, so the WP view path was reached only
  /// by the canary run -- which counts files and never checks where they went.
  /// `Project::wp_info_view` is the single builder for it, so there is no
  /// second spelling to disagree with today; this pins that there is not one
  /// tomorrow either, and it pins the padding rather than the count.
  #[test]
  fn a_work_package_view_lands_at_the_zero_padded_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    let mut thread = thread("ST0001");
    thread.wps = vec![wp(1), wp(12)];
    let packages = thread.wps.len();

    let planned = assemble_model(
      &project,
      &ctx(),
      Model {
        threads: vec![thread],
        ..Default::default()
      },
    )
    .expect("plan");

    // canon: the thread's own file and one per work package in the release's
    // layout (ST0096 WP-05), and no event log (issue 0457). views: info and
    // acceptance, a cover per work package, and the index and the todo view.
    let canon = 1 + packages;
    let views = 2 + packages + 2;
    assert_eq!(planned.writes.len(), canon + views);
    planned.writes.commit().expect("commit").keep();

    let after = tree(dir.path());
    for seq in [1u32, 12] {
      let seq = crate::id::WpId::from_ordinal(seq).expect("a package id");
      let rel = project.relative(&project.wp_info_view("ST0001", seq));
      assert!(after.contains(&rel), "missing {rel} in {after:?}");
    }
    // Named literally as well as derived: the derived assertion above passes
    // if BOTH sides drop the padding together, which is exactly how the issue
    // defect stayed invisible -- each side self-consistent, neither crossed.
    assert!(
      after.contains(&"intent/st/ST0001/WP/01/info.md".to_string()),
      "WP 1 must be `01`, not `1`: {after:?}"
    );
  }

  /// **The invariant [`Blocked::Conservation`] guards, asserted from the other
  /// side so that it is not a tripwire nobody can reach.**
  ///
  /// The guard itself cannot be provoked -- `canon_parts` returns exactly one
  /// part per artefact, for every input -- so a test that tried to
  /// construct the refusal would have to break the exporter to do it. This
  /// pins the equality instead: change the shape of `canon_parts` and this reds
  /// in the same commit that arms the guard. An untested error path and an
  /// untested invariant are the same liability read from two ends.
  #[test]
  fn the_canon_carries_one_file_per_artefact_and_no_event_log() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());

    for (threads, issues) in [(0, 0), (1, 0), (0, 1), (3, 2)] {
      let planned = assemble_model(
        &project,
        &ctx(),
        Model {
          threads: (1..=threads)
            .map(|n| {
              thread(
                &crate::id::ThreadId::from_ordinal(n)
                  .expect("an ordinal that fits")
                  .stored(),
              )
            })
            .collect(),
          issues: (1..=issues).map(issue).collect(),
        },
      )
      .expect("plan");

      // Views: info + acceptance per thread (no WPs in these fixtures), plus
      // the index and the todo view. ST0069 WP-01 gave an issue a realised
      // form, and 0320 then limited it to OPEN issues: these fixtures' issues
      // are closed, so they contribute canon and no view. Stated here rather
      // than derived, so a change in what the estate renders has to be
      // acknowledged -- and this is that acknowledgement rather than a number
      // nudged until it passed.
      let views = 2 * threads as usize + 2;
      let canon = threads as usize + issues as usize;
      assert_eq!(
        planned.writes.len(),
        canon + views,
        "{threads} thread(s) + {issues} issue(s)"
      );
    }
  }

  /// cc, 2026-11-20: `ingest` reads a thread's children by the format of its own
  /// file, so an upgrade cut short is re-runnable only while each thread's
  /// records reach disk before its own file. The write set commits in the order
  /// it is given, so the order planned is the order written.
  #[test]
  fn each_threads_records_are_planned_before_its_own_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let project = project(dir.path());
    let mut carrier = thread("ST0001");
    carrier.wps = vec![wp(1), wp(2)];

    let planned = assemble_model(
      &project,
      &ctx(),
      Model {
        threads: vec![carrier.clone()],
        ..Default::default()
      },
    )
    .expect("plan");
    let order: Vec<PathBuf> = planned
      .writes
      .writes()
      .map(|(path, _)| path.to_path_buf())
      .collect();

    let intent = project.intent_dir();
    let own = intent.join(crate::project::canon_thread_rel(&carrier.id.stored()));
    let own_at = order
      .iter()
      .position(|p| *p == own)
      .expect("the thread's own file is planned");
    let records: Vec<PathBuf> =
      crate::canon_format::encode_thread_files(&carrier, crate::canon_format::CanonFormat::CURRENT)
        .expect("the layout")
        .into_iter()
        .map(|(rel, _)| intent.join(rel))
        .filter(|p| *p != own)
        .collect();
    assert!(
      !records.is_empty(),
      "the carrier must have records, or the order is not tested"
    );
    for record in &records {
      let at = order
        .iter()
        .position(|p| p == record)
        .unwrap_or_else(|| panic!("{} is not planned", record.display()));
      assert!(
        at < own_at,
        "{} is planned after its thread's own file",
        record.display()
      );
    }
  }

  /// Every file under `root`, project-relative and sorted.
  fn tree(root: &std::path::Path) -> Vec<String> {
    fn walk(dir: &std::path::Path, root: &std::path::Path, out: &mut Vec<String>) {
      let Ok(entries) = std::fs::read_dir(dir) else {
        return;
      };
      for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
          walk(&path, root, out);
        } else {
          out.push(crate::project::relative(root, &path));
        }
      }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
  }
}
