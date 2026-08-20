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
use crate::store::{IngestOutcome, Store, StoreError};
use crate::sync::{self, FileState, Scope};

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

/// Validate ONE thread's committed canon: parse, then the two checks that only
/// make sense against the id the file was found under.
///
/// **Public because `legacy.rs` needs the answer and must not grow a second
/// strict reader to get it.** After a migration, `st/<ID>/thread.json` is the
/// SOURCE and the markdown beside it is a generated view, so Phase A asks this
/// rather than re-parsing the view -- which is what let a re-run absorb the
/// renderer's own sections and accrete without bound. Two readers agreeing on
/// what valid canon is, by being one reader.
///
/// It takes the text rather than reading the file, so the caller owns the IO and
/// its error type. `read` propagates an unreadable file as an `IngestError`;
/// `legacy.rs` has already established the file exists.
pub fn read_thread(project: &Project, id: &str, text: &str) -> Result<Thread, Vec<Finding>> {
  let rel = project.relative(&project.thread_json(id));
  let thread = parse::<Thread>(&rel, text)?;
  if thread.schema != THREAD_SCHEMA {
    return Err(vec![Finding::new(
      &rel,
      FindingClass::SchemaInvalid,
      format!(
        "schema is {:?}; this binary reads {THREAD_SCHEMA:?}",
        thread.schema
      ),
    )]);
  }
  if thread.id != id {
    return Err(vec![Finding::new(
      &rel,
      FindingClass::DuplicateId,
      format!(
        "thread id {:?} does not match its directory {id:?}",
        thread.id
      ),
    )]);
  }
  Ok(thread)
}

/// Fill in every OPAQUE attachment's bytes from its sidecar in canon (ST0057
/// AC-03.1).
///
/// **A missing sidecar is a FINDING, never an empty attachment.** Canon naming
/// bytes that do not exist is AC-03.6's invariant one level down, and the
/// silent form of it is the dangerous one: an attachment that reads as present
/// with zero bytes satisfies every check that looks for it, and `organize`
/// would then hydrate an empty file over the author's -- or, worse, gate a
/// removal against an empty comparison and pass.
///
/// **The hash is NOT verified here and that is deliberate.** `doctor` compares
/// them and REPORTS; doing it at read time would either refuse a project the
/// operator needs to open in order to fix it, or silently drop the attachment
/// that failed. Reading answers "are the bytes here"; judging whether they are
/// the right bytes is a different question with a different verb.
fn load_blobs(project: &Project, thread: &mut Thread) -> Vec<Finding> {
  let mut findings = Vec::new();
  let intent_dir = project.intent_dir();
  for att in &mut thread.attachments {
    if !att.is_opaque() {
      continue;
    }
    let rel = crate::project::canon_blob_rel(&thread.id, &att.path);
    let path = intent_dir.join(&rel);
    match std::fs::read(&path) {
      Ok(raw) => att.blob = Some(raw),
      Err(e) => findings.push(Finding::new(
        &rel,
        FindingClass::BrokenReference,
        format!(
          "canon records {} as an opaque attachment of {} carrying {} byte(s), and the file is not \
           readable: {e}",
          att.path, thread.id, att.bytes
        ),
      )),
    }
  }
  findings
}

/// Read and validate the entire committed canon. Refuses with EVERY finding,
/// never the first -- one fix-and-rerun cycle, not one per defect.
pub fn read(project: &Project) -> Result<Canon, IngestError> {
  let mut canon = Canon::default();
  let mut findings = Vec::new();

  for id in project.thread_ids()? {
    match read_thread(project, &id, &read_to_string(&project.thread_json(&id))?) {
      Ok(mut thread) => {
        // **The sidecars are loaded in the SAME step that parsed the JSON, so
        // no caller ever observes the half-formed state** an opaque attachment
        // has between the two (`model::Attachment::blob`). A `Canon` handed out
        // of here holds bytes for every opaque attachment or refuses.
        findings.append(&mut load_blobs(project, &mut thread));
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
        // **FROM THE FIELD, NOT FROM A SIBLING FILE, and the swap is the point
        // of the field.** This read `issues/<nnnn>.md` if one existed. None
        // ever did -- nothing wrote one -- so the branch indexed nothing here
        // while all 40 bodies sat in the v2 estate the migration reads, and a
        // search for an issue's own words returned an empty match.
        //
        // Keeping both would give one issue's prose two homes, which is the
        // Highlander violation vc named as rule 1 of the attachment spec: a
        // file is a typed doc OR carried content, never both.
        if !issue.body.is_empty() {
          canon.sections.append(&mut prose::split(
            "issue",
            &number.to_string(),
            &rel,
            &issue.body,
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

/// **Run a load-from-canon with its outcome recorded ON the store** -- the one
/// home of that recording (AC-03.13).
///
/// A write path whose input was refused must not then be used as a source of
/// truth, and the two verbs that make up that sentence live in different
/// modules. Nothing carried the failure of the first into the second, so
/// `sync --to-store` refused, rolled back correctly, and `sync --to-disk` then
/// wrote the stale store over the canon it had just declined to read -- at
/// rc=0, destroying the same authored criterion twice (vc, 2026-08-18).
///
/// The carrier is the store itself rather than a return value, because the two
/// verbs are separate invocations of separate processes. A value handed back
/// from the ingest reaches nothing; a row in the database is still there when
/// the egest opens it tomorrow.
///
/// **It wraps rather than being called in pairs.** `begin` / `finish` at the
/// call sites would need a `finish` on every `?` in a function whose whole
/// business is refusing -- and the one that got missed would be an unrecorded
/// refusal, which is the original defect with more code in front of it.
///
/// The closure takes the store as an argument instead of capturing it, which is
/// what lets the caller keep using it inside.
pub fn recording<T>(
  store: &mut Store,
  f: impl FnOnce(&mut Store) -> Result<T, IngestError>,
) -> Result<T, IngestError> {
  store.begin_ingest()?;
  let out = f(store);
  let (outcome, detail) = match &out {
    Ok(_) => (IngestOutcome::Succeeded, String::new()),
    Err(e) => (IngestOutcome::Refused, e.to_string()),
  };
  let recorded = store.finish_ingest(outcome, &detail);
  match out {
    // **THE OPERATOR'S ERROR WINS.** A book-keeping failure must never replace
    // the refusal that caused it -- the operator needs to know their canon has
    // a duplicate id, not that a log row would not update. The row is left
    // `attempted`, which reads as not-succeeded, so losing the write here fails
    // in the safe direction rather than silently clearing the block.
    Err(e) => Err(e),
    Ok(v) => {
      recorded?;
      Ok(v)
    }
  }
}

/// Read the canon and load it into the store, atomically.
///
/// The store is touched only after the whole estate has validated, so a
/// refusal leaves the previous DB contents exactly as they were.
pub fn load(project: &Project, store: &mut Store) -> Result<Canon, IngestError> {
  recording(store, |store| {
    let canon = read(project)?;
    store.rebuild(&canon.threads, &canon.issues)?;
    store.replace_doc_sections(&canon.sections)?;
    Ok(canon)
  })
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
  //
  // **`Scope::All` here is forced, not a default.** A cold store is the normal
  // state of every fresh clone, and warming it with anything less would leave
  // the tool answering questions from a store that holds part of the estate --
  // silently, because a partial store looks exactly like a small project.
  resync(project, store, &Scope::All)
}

/// Re-read the committed canon and rebuild the store from it -- the expensive,
/// infrequent path (`intent sync`, and intentd's background pass).
///
/// This is where the whole-tree scan lives now. It refuses on an unparsed file
/// (AC-03.5) rather than reading through it, and it leaves the file index
/// updated so the sync engine can answer "what changed" without re-deriving
/// it.
/// Carry what is on disk into a canon's `attachments`, for the ONE direction
/// where the files are authoritative.
///
/// **NOT part of [`read`], and the placement is the whole correctness of it.**
/// `read` is shared by every load path, so collecting there overwrote canon's
/// attachments with a disk scan on every read -- which destroyed them outright
/// when the files were not present, and, worse, made an attachment divergence
/// IMPOSSIBLE TO OBSERVE. 5.1b rules that an attachment divergence means the
/// STORE is stale; a reader that always takes disk can never report one, so
/// the check that exists to find it silently could not fail. `doctor`'s
/// hash-mismatch arm caught this within one run of putting it in the wrong
/// place.
///
/// So the disk wins HERE and nowhere else -- `sync --to-store` is the declared,
/// destructive, files-are-authoritative direction, and the operator has already
/// been shown what it overwrites (AC-03.9).
pub fn collect_attachments_into(project: &Project, canon: &mut Canon) -> Vec<Finding> {
  let mut findings = Vec::new();
  for thread in &mut canon.threads {
    let (mut carried, refused) = project.collect_attachments(&thread.id);

    // **MERGED, NOT REPLACED, and the reason is the disk model rather than
    // caution.** Disk is a SPARSE projection of the store (ST0057, in the
    // 3.0.0 gate), so a file being absent is the NORMAL state of a dehydrated
    // attachment and is not evidence that anything was deleted. Replacing
    // outright made `sync --to-store` silently drop every attachment the
    // extract carried and the tree had not materialised -- caught by the two
    // round-trip tests, which is the shape D34 exists to prevent: the extract
    // is the interchange, and a direction that quietly empties it is not a
    // round trip.
    //
    // So a file present on disk WINS for its own path -- that is the
    // files-are-authoritative direction doing its job -- and a path canon
    // knows about with no file behind it is carried through untouched.
    // Removing an attachment is therefore an explicit act, never a
    // side effect of not having hydrated it.
    // **CANON'S ORDER IS PRESERVED, and re-sorting it was a real defect rather
    // than a cosmetic one.** The collector returns path-sorted results because
    // the migrator needs a deterministic order for a thread it is seeing for
    // the first time. Applying that sort to a thread canon ALREADY holds
    // rewrites the extract's byte layout for no change in content -- and a
    // round trip that rewrites its own output makes every real change
    // invisible in the noise, which is the property `openness.rs` guards.
    //
    // So: canon's known paths keep their authored positions, a file on disk
    // replaces its own entry in place, and only genuinely NEW paths are
    // appended (in the collector's sorted order, which is deterministic).
    let mut fresh: std::collections::HashMap<String, _> =
      carried.drain(..).map(|a| (a.path.clone(), a)).collect();
    let mut merged = Vec::with_capacity(thread.attachments.len() + fresh.len());
    for existing in &thread.attachments {
      match fresh.remove(&existing.path) {
        Some(from_disk) => merged.push(from_disk),
        None => merged.push(existing.clone()),
      }
    }
    let mut added: Vec<_> = fresh.into_values().collect();
    added.sort_by(|a, b| a.path.cmp(&b.path));
    merged.append(&mut added);
    thread.attachments = merged;

    for (name, reason) in refused {
      findings.push(Finding::new(&name, FindingClass::UnknownFileShape, reason));
    }
  }
  findings
}

/// Does this section belong to `id`'s thread?
///
/// A thread's own sections carry its id; a work package's carry the thread id
/// with the sequence appended. Both forms are matched, because a scope that
/// took a thread's prose and left its work packages' prose behind would leave
/// the search index describing a thread that no longer exists in that shape --
/// AC-06.4's failure, content present and findable as something else.
fn section_of_thread(section: &DocSection, id: &str) -> bool {
  section.owner_id == id || section.owner_id.starts_with(&format!("{id}/"))
}

/// The canon a SCOPED restore installs: named threads take their value from
/// disk, everything else keeps the value the store already holds.
///
/// **This composition is the whole feature and its absence is the trap.**
/// `resync` finishes with a whole-store `rebuild`, so handing it only the
/// named threads would DELETE every thread the operator did not name -- a far
/// worse defect than the estate-wide read it exists to fix, and a silent one,
/// because the node that ran it was saving its own work.
///
/// **Issues and prose compose the same way, and the prose half is the one that
/// is easy to miss.** `doc_sections` feeds the full-text index, so a scoped
/// restore that replaced every section from disk would take a peer's
/// uncommitted prose into search while correctly keeping it out of canon --
/// the same leak, one table over, and invisible because nothing about canon
/// would look wrong.
fn compose_scoped(store: &Store, disk: Canon, named: &[String]) -> Result<Canon, IngestError> {
  let scoped = |id: &str| named.iter().any(|n| n == id);
  let (stored_threads, stored_issues) = store.load_canon()?;

  let mut threads: Vec<Thread> = stored_threads
    .into_iter()
    .filter(|t| !scoped(&t.id))
    .collect();
  threads.extend(disk.threads.into_iter().filter(|t| scoped(&t.id)));
  threads.sort_by(|a, b| a.id.cmp(&b.id));

  let mut sections: Vec<DocSection> = store
    .doc_sections()?
    .into_iter()
    .filter(|s| !named.iter().any(|id| section_of_thread(s, id)))
    .collect();
  sections.extend(
    disk
      .sections
      .into_iter()
      .filter(|s| named.iter().any(|id| section_of_thread(s, id))),
  );

  // Issues are not threads, so a thread scope does not name any of them and
  // they all keep the store's value. `sync <issue>` is a surface that does not
  // exist yet; when it does, it composes here by the same rule.
  Ok(Canon {
    threads,
    issues: stored_issues,
    sections,
  })
}

pub fn resync(project: &Project, store: &mut Store, scope: &Scope) -> Result<Canon, IngestError> {
  recording(store, |store| resync_inner(project, store, scope))
}

/// The body of [`resync`], separated only so the recording wraps every exit
/// from it -- including the `?` on the rebuild, which is the one that produced
/// the live instance.
fn resync_inner(project: &Project, store: &mut Store, scope: &Scope) -> Result<Canon, IngestError> {
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
  let canon = match scope.named() {
    None => canon,
    Some(named) => compose_scoped(store, canon, named)?,
  };
  store.rebuild(&canon.threads, &canon.issues)?;
  store.replace_doc_sections(&canon.sections)?;
  // **The file index is left alone under a scope, deliberately.** It records
  // what was last INGESTED, and a scoped run ingested only part of what the
  // scan saw -- so writing the whole scan would mark a peer's file as seen
  // when nothing read it, and the next unscoped run would find no change in a
  // file that has never entered the store. Leaving it means the next run
  // re-reads more than it strictly must, which costs a hash and cannot lose
  // anything. That is the safe direction of a check whose whole job is to
  // notice change.
  if scope.named().is_none() {
    store.replace_file_index(&entries)?;
  }

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
/// text. **ST prose is no longer among them and its absence is a ruling, not a
/// regression** (D57-6): `design.md` / `impl.md` / `tasks.md` are carried as
/// attachments now, and freeform prose under arbitrary headings has no model
/// field to parse into, so splitting it discarded structure into nothing. WP
/// text is reified INTO `thread.json`, so after the port there is no
/// `WP/<NN>/info.md` for anything to read, and a search for a work package's
/// title found nothing at all.
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

  // **AC-06.4's ST-prose source, after `THREAD_PROSE` left the classifier**
  // (D57-6, extended by vc to this consumer -- the ruling governed
  // `project.rs` and this file was the constant's second reader).
  //
  // `design.md` / `impl.md` / `tasks.md` are attachments now, carried verbatim
  // and no longer split by heading. **Deleting the split without replacing the
  // index would have reinstated the exact class AC-06.4 was written to close**:
  // a search over authored thread prose returning byte-identically to a
  // genuine miss, which `an_unpopulated_index_is_not_the_same_answer_as_a_
  // genuine_miss` exists to make impossible.
  //
  // **ONE UNSPLIT SECTION PER ATTACHMENT, and the "unsplit" is the whole
  // ruling.** Rule 2 of the attachment spec objects to CARVING freeform prose
  // into sections the model has no fields for -- that is how `## Related Steel
  // Threads` became 52 rows of LOST-PROSE. A whole-file section carves
  // nothing: no headings parsed, no structure discarded, and the text is
  // already in the model byte-for-byte, so this adds an index rather than a
  // truth.
  //
  // Every carried attachment, not a list of three names. A file the classifier
  // decided was worth carrying is a file worth finding, and re-introducing a
  // name list here would be `THREAD_PROSE` wearing a different constant.
  for att in &thread.attachments {
    // **An opaque attachment contributes NO prose section, and skipping it is
    // the whole of the reasoning.** `doc_sections` feeds the full-text index;
    // a binary has no words, so indexing its bytes would put whatever a hex
    // dump happens to spell into search results and make the index answer for
    // a file no reader can read.
    let Some(text) = att.text.as_deref() else {
      continue;
    };
    if text.trim().is_empty() {
      continue;
    }
    out.push(DocSection {
      owner_type: "thread".to_string(),
      owner_id: thread.id.clone(),
      file: project.relative(&project.thread_dir(&thread.id).join(&att.path)),
      seq: 0,
      heading: Some(att.path.clone()),
      level: 0,
      body: text.to_string(),
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

fn read_to_string(path: &std::path::Path) -> Result<String, IngestError> {
  std::fs::read_to_string(path).map_err(|source| IngestError::Io {
    path: path.display().to_string(),
    source,
  })
}
