//! Strict ingest: committed canon -> the model -> the store (design.md D05).
//!
//! Two properties, and they are the whole module:
//!
//! **Strict.** Every structured file is validated against the generated JSON
//! Schema face before it is deserialised, and an invalid file is refused with
//! the file and the finding named. There is no tolerance ladder for
//! current-version data -- lenience was only ever a coping strategy for having
//! no schema, and v2's five parse-related issues in one release are the bill
//! for it. The legacy parser (WP-10) is lenient; this is not, and they are
//! deliberately different code.
//!
//! **Atomic.** The whole estate is read and validated into memory before the
//! store is touched. A refusal leaves nothing partially loaded, so there is no
//! state in which half a project is queryable and nothing says so. That is
//! AC-03.1's "nothing partially loaded" clause, and it is why [`read`] returns
//! a [`Canon`] rather than writing as it goes.

use std::sync::OnceLock;

use serde::de::DeserializeOwned;

use crate::finding::{Finding, FindingClass, Refusal};
use crate::model::{ISSUE_SCHEMA, Issue, THREAD_SCHEMA, Thread};
use crate::project::Project;
use crate::prose::{self, DocSection};
use crate::store::{Store, StoreError};
use crate::sync::{self, FileState};

/// Everything the committed canon says, in memory, validated.
#[derive(Debug, Clone, Default)]
pub struct Canon {
  pub threads: Vec<Thread>,
  pub issues: Vec<Issue>,
  pub sections: Vec<DocSection>,
}

#[derive(Debug, thiserror::Error)]
pub enum IngestError {
  // NOT `#[from]`: that implies `#[source]`, so with `{0}` as the Display body
  // the refusal renders once as this variant's message and again as its own
  // cause, and every residue count reads DOUBLE. Measured on this repo: 12
  // findings, 24 printed lines, against a summary line correctly saying 12.
  // AC-10.2 shows a migrator its residue per line, so a migration would have
  // reported twelve problems as twenty-four.
  #[error("{0}")]
  Refused(Refusal),
  /// The event log's extract exists and is not readable as one.
  ///
  /// Its own variant rather than a finding: history is the one thing nothing
  /// else can reconstruct, so "your history file is damaged" needs an action of
  /// its own and must not be reported as though a thread were malformed.
  #[error("the event log extract at {path} could not be read")]
  EventLogUnreadable { path: String, cause: String },
  #[error(transparent)]
  Store(#[from] StoreError),
  #[error(transparent)]
  Project(#[from] crate::project::ProjectError),
  #[error("reading {path}: {source}")]
  Io {
    path: String,
    #[source]
    source: std::io::Error,
  },
}

impl crate::remedy::Remedy for IngestError {
  fn remedy(&self) -> String {
    match self {
      // The refusal has already listed every row it could not read, each with
      // its file. Repeating "fix the errors" would be a remedy that adds
      // nothing to the thing above it.
      Self::Refused(_) => {
        "fix the rows named above -- each names the file it came from, and the `carried:` lines are not yours to fix, they convert as they are".to_string()
      }
      // Restore, never delete. The event log is the one artefact in the estate
      // that nothing recomputes: the store is rebuildable from the extract
      // (D36) and history is not rebuildable from anything.
      Self::EventLogUnreadable { path, .. } => format!(
        "restore {path} from git (`git checkout -- {path}`) rather than deleting it -- history is the one artefact nothing recomputes"
      ),
      Self::Store(e) => crate::remedy::Remedy::remedy(e),
      Self::Project(e) => crate::remedy::Remedy::remedy(e),
      Self::Io { path, .. } => format!("check that {path} exists and that this user can read it"),
    }
  }
}

impl From<Refusal> for IngestError {
  fn from(refusal: Refusal) -> Self {
    Self::Refused(refusal)
  }
}

/// The authored prose files that belong to a thread. Generated views
/// (`info.md`, `acceptance.md`) are deliberately absent: a view is rendered
/// from the model, so indexing it would index the model twice and let a stale
/// view answer a search.
const THREAD_PROSE: &[&str] = &["design.md", "impl.md", "tasks.md"];

/// Read and validate the entire committed canon. Refuses with EVERY finding,
/// never the first -- one fix-and-rerun cycle, not one per defect.
pub fn read(project: &Project) -> Result<Canon, IngestError> {
  let mut canon = Canon::default();
  let mut findings = Vec::new();

  for id in project.thread_ids()? {
    let path = project.thread_json(&id);
    let rel = project.relative(&path);
    match parse::<Thread>(&rel, &read_to_string(&path)?) {
      Ok(thread) => {
        if thread.schema != THREAD_SCHEMA {
          findings.push(Finding::new(
            &rel,
            FindingClass::SchemaInvalid,
            format!(
              "schema is {:?}; this binary reads {THREAD_SCHEMA:?}",
              thread.schema
            ),
          ));
          continue;
        }
        if thread.id != id {
          findings.push(Finding::new(
            &rel,
            FindingClass::DuplicateId,
            format!(
              "thread id {:?} does not match its directory {id:?}",
              thread.id
            ),
          ));
          continue;
        }
        collect_prose(project, &mut canon.sections, "thread", &id)?;
        collect_wp_text(project, &mut canon.sections, &thread);
        canon.threads.push(thread);
      }
      Err(mut found) => findings.append(&mut found),
    }
  }

  for number in project.issue_numbers()? {
    let path = project.issue_json(number);
    let rel = project.relative(&path);
    match parse::<Issue>(&rel, &read_to_string(&path)?) {
      Ok(issue) => {
        if issue.schema != ISSUE_SCHEMA {
          findings.push(Finding::new(
            &rel,
            FindingClass::SchemaInvalid,
            format!(
              "schema is {:?}; this binary reads {ISSUE_SCHEMA:?}",
              issue.schema
            ),
          ));
          continue;
        }
        let body = project.issue_md(number);
        if body.is_file() {
          let text = read_to_string(&body)?;
          canon.sections.append(&mut prose::split(
            "issue",
            &number.to_string(),
            &project.relative(&body),
            &text,
          ));
        }
        canon.issues.push(issue);
      }
      Err(mut found) => findings.append(&mut found),
    }
  }

  if findings.is_empty() {
    Ok(canon)
  } else {
    Err(Refusal::new(findings).into())
  }
}

/// Read the canon and load it into the store, atomically.
///
/// The store is touched only after the whole estate has validated, so a
/// refusal leaves the previous DB contents exactly as they were.
pub fn load(project: &Project, store: &mut Store) -> Result<Canon, IngestError> {
  let canon = read(project)?;
  store.rebuild(&canon.threads, &canon.issues)?;
  store.replace_doc_sections(&canon.sections)?;
  Ok(canon)
}

/// Load the model for a command to answer from -- **from the STORE when the
/// working tree has not moved**, and from canon only when it has.
///
/// This is the daily-driver path (hv, 2026-08-14: "the whole point of this is
/// to get away from files for the daily driver"). Before it existed, every
/// invocation parsed every `thread.json`, validated each against the generated
/// JSON Schema, and rebuilt the whole DB before answering a question as small
/// as `st list` -- correct, and the wrong shape entirely.
///
/// What it does NOT do is weaken D01 as reversed. **The store is truth and the
/// committed canon is the extract** (D34), so re-creating the store from canon
/// is a CAPABILITY rather than a licence -- it recovers what the extract
/// carries, which is not everything the store held. A COLD store is a different
/// thing entirely and stays routine: it takes the ingest path via [`resync`] on
/// first use and is then warm. What this changes is WHEN that is paid for, not
/// which side is authoritative.
pub fn load_fresh(project: &Project, store: &mut Store) -> Result<Canon, IngestError> {
  // THE DAILY DRIVER DOES NOT LOOK AT THE FILES. hv, 2026-08-14: "A sync
  // ingest/egest is fine to be (relatively) expensive. This is infrequent and
  // can even be done periodically by intentd in the background. But CLI and
  // TOOL and MCP use for daily driver use needs to be FAST."
  //
  // The first version of this verified freshness by content-hashing the whole
  // tree before every command, which was measured at 244 files and ~13ms on an
  // 80-thread project -- and the hashing, not the parsing, was the dominant
  // cost. Hashing is the only honest freshness test (AC-03.3), so the way to
  // stop paying for it is not to make it cheaper but to take it OFF this path.
  //
  // Where freshness comes from instead:
  //   - `intent sync`, explicitly, which is [`resync`] below;
  //   - WP-08's intentd, watching the tree and keeping the store hot;
  //   - a cold store, which ingests once here and is then warm.
  //
  // And staleness stays VISIBLE rather than becoming a silent wrong answer:
  // `doctor` rebuilds from canon and compares, so "the store disagrees with
  // the files" is a reported finding with a named remedy. That is the trade
  // hv is making, made explicit rather than assumed.
  let (threads, issues) = store.load_canon()?;
  if !threads.is_empty() || !issues.is_empty() {
    return Ok(Canon {
      threads,
      issues,
      sections: store.doc_sections()?,
    });
  }

  // Cold store: ingest once, from the files, and warm it.
  resync(project, store)
}

/// Re-read the committed canon and rebuild the store from it -- the expensive,
/// infrequent path (`intent sync`, and intentd's background pass).
///
/// This is where the whole-tree scan lives now. It refuses on an unparsed file
/// (AC-03.5) rather than reading through it, and it leaves the file index
/// updated so the sync engine can answer "what changed" without re-deriving
/// it.
pub fn resync(project: &Project, store: &mut Store) -> Result<Canon, IngestError> {
  let previous = store.file_index()?;
  let entries = sync::scan(project.root(), &previous).map_err(|e| IngestError::Io {
    path: project.root().display().to_string(),
    source: std::io::Error::other(e.to_string()),
  })?;

  let findings: Vec<Finding> = entries
    .iter()
    .filter(|e| e.state == FileState::Unparsed)
    .flat_map(|e| e.findings.iter().cloned())
    .collect();
  if !findings.is_empty() {
    return Err(Refusal::new(findings).into());
  }

  let canon = read(project)?;
  store.rebuild(&canon.threads, &canon.issues)?;
  store.replace_doc_sections(&canon.sections)?;
  store.replace_file_index(&entries)?;
  // **HISTORY IS RESTORED HERE TOO, and its absence was a silent defect.**
  // This function rebuilt seven tables from the extract and skipped the ONE
  // the extract exists to carry. Every other table is derived, so a resync
  // that omitted it would have been recoverable; `event_log` is derived from
  // nothing (D34), so omitting it lost the data outright.
  //
  // The reach was the whole of the cold-store path: `load_fresh` warms an
  // empty store through here, and an empty store is the normal state of every
  // fresh clone (D21 gitignores `intent/.cache/`). So a clone answered every
  // question correctly and had no history at all until somebody happened to
  // run the explicit `intent sync --to-store` -- and nothing reported it,
  // because a missing log looks exactly like a project that has never
  // recorded anything.
  restore_event_log(project, store)?;

  // **The restore SUCCEEDING is not the same as the history being there**, and
  // the gap is reported rather than refused. `restore_event_log` returns
  // `Ok(0)` for an absent file, which is right on its own terms: a project that
  // has never recorded anything has no log, and refusing to open it would be
  // refusing `intent init`.
  //
  // An earlier version of this refused when the canon had entities and the
  // restore produced nothing, arguing that under D34 every mutation writes an
  // envelope so entities imply history. **The suite refuted it: a hand-authored
  // `thread.json` is an entity that never came from a mutation, and that is
  // exactly the shape WP-10's migration produces** -- so the refusal would have
  // refused every estate the migrator makes, on the path AC-03.11 exists to
  // protect. The check lives in `doctor` as `EventLogAbsent` instead, which is
  // what the criterion's own word REPORTED asks for.
  Ok(canon)
}

/// Take into the store whatever the committed event-log extract carries and the
/// store does not.
///
/// **The one restore, shared by the cold-store path and the explicit
/// `sync --to-store`.** It lived on the facade and had a single caller, which
/// is how `resync` came to rebuild everything except this.
///
/// Merging on the ULID makes it idempotent and makes two machines' logs a union
/// rather than a conflict, so restoring an older extract over a newer log adds
/// nothing and loses nothing.
///
/// An absent file is not an error: a project that has never synced out has no
/// extract of its history yet, and refusing here would make the first sync of
/// an old project impossible.
pub fn restore_event_log(project: &Project, store: &mut Store) -> Result<usize, IngestError> {
  let path = project.events_jsonl();
  let text = match std::fs::read_to_string(&path) {
    Ok(text) => text,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
    Err(source) => {
      return Err(IngestError::Io {
        path: project.relative(&path),
        source,
      });
    }
  };
  let incoming = crate::event::from_jsonl(&text).map_err(|e| IngestError::EventLogUnreadable {
    path: project.relative(&path),
    cause: e.to_string(),
  })?;
  let have = store.events()?;
  let missing = crate::event::merge(&have, &incoming);
  for envelope in &missing {
    // **`restore_event`, never `append_event`** (D42). These already happened;
    // the extract carries when. Recording them as happening now would rewrite
    // the whole of an older clone's history to the moment someone restored it
    // -- and every stamp would look perfectly valid.
    store.restore_event(envelope)?;
  }
  Ok(missing.len())
}

/// Refresh the file index, and refuse if any file in scope is unparsed.
///
/// This is the AC-03.5 gate: a command that needs the estate calls it and gets
/// the findings, rather than reading through a conflict-markered file the way
/// v2's greps did. The index is written either way -- knowing which files are
/// broken is exactly what `doctor` reports -- but the caller is told no.
pub fn refresh_index(project: &Project, store: &mut Store) -> Result<(), IngestError> {
  let previous = store.file_index()?;
  let entries = sync::scan(project.root(), &previous).map_err(|e| IngestError::Io {
    path: project.root().display().to_string(),
    source: std::io::Error::other(e.to_string()),
  })?;
  store.replace_file_index(&entries)?;

  let findings: Vec<Finding> = entries
    .iter()
    .filter(|e| e.state == FileState::Unparsed)
    .flat_map(|e| e.findings.iter().cloned())
    .collect();
  if findings.is_empty() {
    Ok(())
  } else {
    Err(Refusal::new(findings).into())
  }
}

/// Validate against the generated schema, then deserialise.
///
/// Validation first is what makes the messages useful: the schema names the
/// instance path and the constraint, where a serde error names only where it
/// gave up. Deserialisation after a clean validation cannot fail for schema
/// reasons, so its error is reported honestly as a schema finding too.
fn parse<T: Validated>(rel: &str, text: &str) -> Result<T, Vec<Finding>> {
  let value: serde_json::Value = match serde_json::from_str(text) {
    Ok(value) => value,
    Err(e) => {
      return Err(vec![
        Finding::new(rel, FindingClass::MalformedJson, e.to_string()).at_line(e.line() as u32),
      ]);
    }
  };

  let errors: Vec<Finding> = T::validator()
    .iter_errors(&value)
    .map(|e| {
      Finding::new(
        rel,
        FindingClass::SchemaInvalid,
        format!("at {}: {e}", instance_path(&e)),
      )
    })
    .collect();
  if !errors.is_empty() {
    return Err(errors);
  }

  serde_json::from_value(value).map_err(|e| {
    vec![Finding::new(
      rel,
      FindingClass::SchemaInvalid,
      e.to_string(),
    )]
  })
}

/// `/` for the document root, otherwise the pointer -- so a message never
/// reads "at : ...".
fn instance_path(error: &jsonschema::ValidationError<'_>) -> String {
  let path = error.instance_path().to_string();
  if path.is_empty() {
    "/".to_string()
  } else {
    path
  }
}

/// A canon type that ingest validates before deserialising.
///
/// The validator is an associated function rather than a lookup because a
/// `static` inside a generic function is NOT per-monomorphisation in Rust --
/// one cache would be shared across every type and quietly validate issues
/// against the thread schema. One impl per type makes the compiler check what
/// a naming convention would only have hoped for.
trait Validated: DeserializeOwned + schemars::JsonSchema {
  fn validator() -> &'static jsonschema::Validator;
}

/// Compile a type's schemars output -- the same generator the committed face
/// is rendered from, so the thing that validates and the thing that is
/// committed cannot be different things.
fn compile<T: schemars::JsonSchema>() -> jsonschema::Validator {
  let schema = serde_json::to_value(schemars::schema_for!(T))
    .expect("a schemars schema serialises to JSON by construction");
  jsonschema::validator_for(&schema).expect(
    "a schemars-generated schema compiles; a failure here is a version break between schemars and jsonschema, never user input",
  )
}

impl Validated for Thread {
  fn validator() -> &'static jsonschema::Validator {
    static V: OnceLock<jsonschema::Validator> = OnceLock::new();
    V.get_or_init(compile::<Thread>)
  }
}

impl Validated for Issue {
  fn validator() -> &'static jsonschema::Validator {
    static V: OnceLock<jsonschema::Validator> = OnceLock::new();
    V.get_or_init(compile::<Issue>)
  }
}

/// Put each work package's text into the prose index (AC-06.4).
///
/// AC-06.4 names three searchable sources -- ST prose, issue bodies and WP
/// text -- and the first two are authored markdown that [`collect_prose`]
/// picks up. WP text is not: v3 reifies work packages INTO `thread.json`, so
/// after the port there is no `WP/<NN>/info.md` for anything to read, and a
/// search for a work package's title found nothing at all.
///
/// This is not double truth. `work_packages` and `doc_sections` are both
/// projections rebuilt from `thread.json` on every load, so this is one truth
/// carrying two indexes, which is what an index is. The authored/generated
/// line D02 protects is untouched: nothing here is written back to a file.
pub fn collect_wp_text(project: &Project, out: &mut Vec<DocSection>, thread: &Thread) {
  let file = project.relative(&project.thread_json(&thread.id));

  // D22 reified the THREAD's own prose into `thread.json` -- `objective` and
  // `context` -- exactly as D28 later did for work packages, and the indexer
  // was only taught about the second one. So a phrase living in a thread's
  // objective matched nothing at all.
  //
  // vc measured it as AC-06.4 failing: canon carries a unique phrase, `sync`
  // exits 0, `info.md` is regenerated CONTAINING that phrase, and `search`
  // returns exit 0 with zero bytes. That info.md is a generated VIEW, which
  // `THREAD_PROSE` deliberately excludes -- indexing it would index the model
  // twice and let a stale view answer a search -- so the phrase was rendered
  // everywhere a human looks and indexed nowhere.
  //
  // The shape is the finding, and it is the AC-10.7 class in a fourth command
  // (vc): a search over an unpopulated index is byte-identical to a search
  // with no matches, on a project that IS migrated, so the migration guard
  // cannot catch it.
  let thread_body = match (thread.objective.trim(), thread.context.trim()) {
    ("", context) => context.to_string(),
    (objective, "") => objective.to_string(),
    (objective, context) => format!("{objective}\n\n{context}"),
  };
  if !thread_body.is_empty() {
    out.push(DocSection {
      owner_type: "thread".to_string(),
      owner_id: thread.id.clone(),
      file: file.clone(),
      seq: 0,
      heading: Some(thread.title.clone()),
      level: 0,
      body: thread_body,
    });
  }

  for wp in &thread.wps {
    out.push(DocSection {
      owner_type: "work-package".to_string(),
      owner_id: format!("{}/{:02}", thread.id, wp.seq),
      file: file.clone(),
      seq: wp.seq,
      heading: Some(wp.title.clone()),
      level: 0,
      // D28: the authored prose, not just the title. AC-06.7 requires `intent
      // search` to find a phrase that appears ONLY in a work package's body,
      // which is what makes AC-06.4's "WP text" mean something rather than
      // matching titles.
      body: if wp.objective.is_empty() {
        wp.body.clone()
      } else if wp.body.is_empty() {
        wp.objective.clone()
      } else {
        format!("{}\n\n{}", wp.objective, wp.body)
      },
    });
  }
}

fn collect_prose(
  project: &Project,
  out: &mut Vec<DocSection>,
  owner_type: &str,
  id: &str,
) -> Result<(), IngestError> {
  for name in THREAD_PROSE {
    let path = project.thread_dir(id).join(name);
    if path.is_file() {
      let text = read_to_string(&path)?;
      out.append(&mut prose::split(
        owner_type,
        id,
        &project.relative(&path),
        &text,
      ));
    }
  }
  Ok(())
}

fn read_to_string(path: &std::path::Path) -> Result<String, IngestError> {
  std::fs::read_to_string(path).map_err(|source| IngestError::Io {
    path: path.display().to_string(),
    source,
  })
}
