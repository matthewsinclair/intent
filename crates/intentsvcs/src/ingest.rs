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
  #[error("{0}")]
  Refused(#[from] Refusal),
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

/// Ingest a v2 estate by parsing its markdown -- the explicit recovery path,
/// and the seam the WP-10 migrator plugs its frozen legacy parser into
/// (design.md; acceptance lives at AC-10.2/10.3, not in WP-03).
///
/// It refuses rather than half-working, because a migrator that does half of a
/// two-ended migration is the failure this project has already paid for once.
pub fn from_md(project: &Project) -> Result<Canon, IngestError> {
  Err(
    Refusal::new(vec![Finding::new(
      project.relative(&project.st_dir()),
      FindingClass::UnknownFileShape,
      "ingest --from-md is scaffolding: the frozen legacy markdown parser lands in WP-10 (ST0056). Nothing has been read and nothing has been written.",
    )])
    .into(),
  )
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
