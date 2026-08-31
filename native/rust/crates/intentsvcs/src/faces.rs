//! The generated schema faces (design.md, "one master, three faces").
//!
//! The model types in this crate are the single authored master; this module
//! renders the committed artefacts under `schema/` at the repo root. The
//! JSON Schema face is generated here via schemars; the DDL face is the
//! store's [`crate::store::DDL`] rendered verbatim; the GraphQL SDL face is
//! exported from [`crate::graphql`].
//!
//! `tests/schema_faces_drift.rs` regenerates these and fails on any diff
//! against the committed files. Regenerate with `INTENT_BLESS=1 cargo test
//! -p intentsvcs --test schema_faces_drift` -- the committed face changes
//! only when a type change is deliberate, and always in the same commit.

use schemars::schema_for;

use crate::event::Envelope;
use crate::model::{Issue, Thread};

/// **Which TOOL produced this artefact** (D41). Moves on every release,
/// including a patch.
///
/// Read from the crate manifest rather than declared here, because a version
/// constant beside a version field is the hand-kept companion this decision
/// exists to remove -- and it would go stale at exactly the moment nobody is
/// looking at this file, which is a release.
pub const INTENT_VER: &str = env!("CARGO_PKG_VERSION");

/// **Whether the CONTRACT a consumer compiles against has changed** (D41), one
/// per face type.
///
/// Three, not one, and not five. The three JSON Schemas share a version
/// because they are one contract in three documents -- splitting per file
/// would let `thread` and `issue` drift apart with nothing recording that they
/// had. The DDL and the SDL are separate contracts with separate consumers, so
/// they move separately: a consumer generating a GraphQL client should not be
/// told to re-check because a SQL column moved.
///
/// **These move independently of [`INTENT_VER`], and that is the point of
/// having two parts.** A patch release moves the tool version and must not
/// move these. `tests/schema_versioning.rs` fails the build when a face's
/// content changes and its version does not, which is the same forcing
/// function `SCHEMA_VERSION` has for the store -- the one that earned its
/// existence within hours of being written.
pub const SCHEMA_JSON_VER: u32 = 16;
/// See [`SCHEMA_JSON_VER`].
pub const SCHEMA_DDL_VER: u32 = 13;
/// See [`SCHEMA_JSON_VER`].
pub const SCHEMA_SDL_VER: u32 = 14;

/// The committed faces, as `(relative path under schema/, content)` pairs.
pub fn faces() -> Vec<(&'static str, String)> {
  vec![
    ("thread.schema.json", schema_json::<Thread>()),
    ("issue.schema.json", schema_json::<Issue>()),
    ("event.schema.json", schema_json::<Envelope>()),
    ("ddl.sql", versioned_sql(crate::store::DDL)),
    ("schema.graphql", versioned_sdl(&crate::graphql::sdl())),
  ]
}

/// The version marker keys, in the one place both the writer and the reader
/// can agree on.
///
/// **`tests/schema_versioning.rs` reads the PUBLISHED file and finds these**,
/// rather than asking the constants. The failure being guarded is a generator
/// that stops injecting, and a test that reads the version out of the same
/// constant the generator used passes on exactly that defect.
pub const INTENT_VER_KEY: &str = "INTENT_VER";
/// See [`INTENT_VER_KEY`].
pub const SCHEMA_VER_KEYS: &[(&str, &str)] = &[
  ("ddl.sql", "SCHEMA_DDL_VER"),
  ("schema.graphql", "SCHEMA_SDL_VER"),
  ("thread.schema.json", "SCHEMA_JSON_VER"),
  ("issue.schema.json", "SCHEMA_JSON_VER"),
  ("event.schema.json", "SCHEMA_JSON_VER"),
];

/// **Each face carries its version in ITS OWN idiom, never in a companion
/// file** (D41a). A `.sql` file says it in SQL comments, so the artefact stays
/// exactly what it claims to be: `ddl.sql` is still a file you can feed to
/// SQLite, and `schema.graphql` is still parseable SDL.
fn versioned_sql(body: &str) -> String {
  format!("-- {INTENT_VER_KEY}: {INTENT_VER}\n-- SCHEMA_DDL_VER: {SCHEMA_DDL_VER}\n{body}")
}

fn versioned_sdl(body: &str) -> String {
  format!("# {INTENT_VER_KEY}: {INTENT_VER}\n# SCHEMA_SDL_VER: {SCHEMA_SDL_VER}\n\n{body}")
}

/// What each face SAYS its versions are, as `(face, tool version, contract
/// key, contract version)`.
///
/// **Read back out of the generated artefact, never reported from the
/// constants** -- and the distinction is the whole value of the command. The
/// failure being guarded is a generator that stops injecting: a build where the
/// injection is dropped, refactored away or short-circuited still has correct
/// constants, and it is the ARTEFACT that lost its marker. A `--versions` that
/// printed [`INTENT_VER`] directly would confidently report the right number
/// while `intent schema ddl.sql` handed a consumer a face with no version in
/// it, and the two outputs of one command would disagree.
///
/// So this parses what it just generated, which makes the command a second
/// witness to the injection in the same way `intent schema` is a second witness
/// to face drift.
///
/// A face missing a marker yields an empty string rather than being skipped: a
/// missing measurement must present as a gap the caller can see, never as a row
/// that is quietly absent from a list.
/// One face as the `schema` row answers it: the committed body, and the two
/// version markers `--versions` reports instead of the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaFace {
  pub name: &'static str,
  pub content: String,
  pub intent_ver: String,
  pub key: &'static str,
  pub contract_ver: String,
}

fn describe(name: &'static str, content: String) -> SchemaFace {
  let key = SCHEMA_VER_KEYS
    .iter()
    .find(|(face, _)| *face == name)
    .map_or("SCHEMA_VER", |(_, key)| *key);
  SchemaFace {
    name,
    intent_ver: marker(&content, INTENT_VER_KEY).unwrap_or_default(),
    key,
    contract_ver: marker(&content, key).unwrap_or_default(),
    content,
  }
}

/// The `schema` row's answer: every face, or the one named -- and an unknown
/// name is refused by name rather than answered empty.
///
/// **ONE HOME FOR A ROW THAT ANSWERS IN TWO PLACES, AND WHY THE HOME IS HERE
/// RATHER THAN ON THE FACADE.** `intent schema` is a GLOBAL verb -- it answers
/// outside a project, which `schema_command.rs` pins -- so its terminal arm
/// cannot open a facade to ask. The MCP tier reaches everything through the
/// facade, by the ruleset vc recorded (one method per exposed row; add one
/// rather than compose in the tier). So the selection lives here, store-free,
/// and [`Facade::schema`] is the facade's door onto exactly this function:
/// two callers, one body, and the refusal cannot drift between them.
pub fn schema(face: Option<&str>) -> Result<Vec<SchemaFace>, crate::facade::FacadeError> {
  let all: Vec<SchemaFace> = faces()
    .into_iter()
    .map(|(name, content)| describe(name, content))
    .collect();
  match face {
    None => Ok(all),
    Some(name) => all
      .into_iter()
      .find(|f| f.name == name)
      .map(|f| vec![f])
      .ok_or_else(|| crate::facade::FacadeError::NoSuchFace {
        face: name.to_string(),
      }),
  }
}

impl crate::facade::Facade {
  /// The `schema` row on the MCP tier (`facade: "schema"` in the dispatch
  /// table). It touches no store -- the faces are generated from the types --
  /// and delegates to [`schema`], which is the one home; see there for why the
  /// home is not this method.
  pub fn schema(&self, face: Option<&str>) -> Result<Vec<SchemaFace>, crate::facade::FacadeError> {
    schema(face)
  }
}

/// The bare form's text: every face under a `== <name> ==` banner, so the
/// output stays machine-splittable (AC-06.5's per-face byte identity is
/// checked by splitting exactly this).
pub fn banner(faces: &[SchemaFace]) -> String {
  let mut out = String::new();
  for face in faces {
    out.push_str(&format!("== {} ==\n", face.name));
    out.push_str(&face.content);
    if !face.content.ends_with('\n') {
      out.push('\n');
    }
  }
  out
}

/// The value a face records for a marker, in whichever idiom that face uses.
///
/// One reader for three idioms, because it is the same fact in all three and a
/// per-idiom reader would let one of them quietly stop being checked. SQL says
/// `-- KEY: value`, SDL says `# KEY: value`, JSON says `"x-key": value`.
///
/// `tests/schema_versioning.rs` deliberately carries its OWN copy of this
/// logic. That is not drift: it reads the COMMITTED files while this reads the
/// generated ones, and a test sharing its oracle with the code under test can
/// only be as strict as that oracle.
fn marker(face: &str, key: &str) -> Option<String> {
  let json_key = format!("\"x-{}\"", key.to_lowercase().replace('_', "-"));
  face.lines().find_map(|line| {
    let line = line.trim();
    let rest = line
      .strip_prefix(&format!("-- {key}:"))
      .or_else(|| line.strip_prefix(&format!("# {key}:")))
      .or_else(|| line.strip_prefix(&format!("{json_key}:")))?;
    Some(
      rest
        .trim()
        .trim_end_matches(',')
        .trim()
        .trim_matches('"')
        .to_string(),
    )
  })
}

/// One face by name, for `intent schema <face>`. `None` if no face has that
/// name -- the caller reports it; this module does not know the error type.
pub fn face(name: &str) -> Option<String> {
  faces()
    .into_iter()
    .find(|(path, _)| *path == name)
    .map(|(_, content)| content)
}

/// The names of the committed faces, in print order.
pub fn face_names() -> Vec<&'static str> {
  faces().into_iter().map(|(path, _)| path).collect()
}

/// Every face, banner-separated, for a bare `intent schema` (AC-06.5).
///
/// **It GENERATES rather than reading `schema/`.** That is the whole property:
/// AC-06.5 asks that what the command prints be byte-identical to the
/// committed files, and a command that printed the files would satisfy that
/// vacuously -- it would be `cat` with extra steps, and would keep passing
/// after the model and the committed face had drifted apart. Printing from the
/// types makes the command a second, independent witness to the same drift
/// `schema_faces_drift.rs` guards.
/// Render one type's JSON Schema in canonical form (2-space pretty, trailing
/// newline).
fn schema_json<T: schemars::JsonSchema>() -> String {
  let schema = schema_for!(T);
  let mut value =
    serde_json::to_value(&schema).expect("a schemars schema serialises to JSON by construction");
  // JSON Schema's own extension idiom: an `x-` keyword is ignored by every
  // validator and preserved by every parser, so the version travels inside the
  // document a consumer already has rather than beside it.
  if let Some(object) = value.as_object_mut() {
    object.insert(
      format!("x-{}", INTENT_VER_KEY.to_lowercase().replace('_', "-")),
      serde_json::Value::String(INTENT_VER.to_string()),
    );
    object.insert(
      "x-schema-json-ver".to_string(),
      serde_json::Value::from(SCHEMA_JSON_VER),
    );
  }
  let mut out = serde_json::to_string_pretty(&value)
    .expect("a schemars schema serialises to JSON by construction");
  out.push('\n');
  out
}
