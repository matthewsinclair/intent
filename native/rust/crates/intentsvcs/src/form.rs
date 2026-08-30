//! The form DSL: **layout is declared, the field set is not** (`AC-17.2`).
//!
//! A form declaration carries ORDER, LABEL, WIDGET and EDITABILITY for the
//! fields of one entity. It never enumerates what fields exist. Existence and
//! type come from the entity's published schema face, and editability from the
//! mutation surface, so neither is written down twice.
//!
//! # WHY THE FIELD SET IS NOT IN THE DECLARATION
//!
//! `AC-17.2` names the failure by its precedent: **a hand-authored list of
//! fields is a second home for what `schema/*.json` already declares, and it
//! goes stale exactly the way `populations.shipped` did.** A form that lists
//! its own fields keeps rendering a form that looks complete after the model
//! has moved, and nothing says so -- which is the silent-partial class this
//! thread exists to remove.
//!
//! So this module resolves a declaration against two derivations it does not
//! own:
//!
//! - **Existence** comes from [`crate::faces::faces`], the SAME rendered bytes
//!   committed under `schema/` and the same ones a consumer compiles against.
//!   Not a re-derivation that agrees today: the published face itself.
//! - **Editability** comes from [`crate::facade::unsettable_kind`], which is
//!   the estate's ruled answer to *can I change this, and if so how* --
//!   `Elsewhere` (a route exists and the remedy names it), `Never` (no route,
//!   by design) and `NotYet` (a gap rather than a decision).
//!
//! # WHY NOT `settable_fields`, WHICH IS THE OBVIOUS ONE
//!
//! **The first version of this module used it and was wrong in a way only a
//! measurement showed.** `Facade::settable_fields` answers *can the NARROW
//! SETTER write this*, which is a smaller question than *can this be edited*.
//! Driven against the thread face it put `status` outside the writable set --
//! and `status` is precisely the field `AC-17.4` builds the `select` widget
//! for, whose whole job is to offer the legal transitions the MACHINE owns.
//! So the check would have reported the form's most carefully specified row as
//! offering an edit the surface refuses.
//!
//! **The field was the claim and the note was its scope.** `unsettable_kind`
//! is the axis vc ruled for exactly this reader: a route through another door
//! is still a route, and it is `Elsewhere` rather than a refusal.
//!
//! **Neither is re-implemented here, and that is the whole design.** A form
//! naming a field the model dropped is refused at load; a writable field on no
//! form is NAMED. Both directions, because `AC-17.2` is held both ways.
//!
//! # WHY JSON AND NOT YAML
//!
//! `tui-design.md` §1 says "a YAML declaration". **The substance of every
//! stated property survives the format change and none of the criteria name a
//! format**: `AC-17.2` says *declared, not coded*; `AC-17.4` says the widget
//! vocabulary is *declared in the DSL schema the way `entry_dispositions` and
//! `target_states` are declared in the register* -- and the register is JSON.
//! JSON is what this estate's authored canon already is, in
//! `surface/dispatch-table.json` and every `intent/.canon/st/*.json`.
//!
//! **The deciding argument is a dependency, not a taste.** No YAML parser is
//! in this workspace; adding one moves `Cargo.lock`, which three other nodes
//! build against, and the obvious crate (`serde_yaml`) was archived by its
//! maintainer. Taking a shared-lockfile change and an unmaintained dependency
//! to honour one word in a design whose every stated property is
//! format-neutral is the wrong trade. **Recorded as a deviation rather than
//! taken silently.**

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::address::Entity as AddrEntity;
use crate::facade::UnsettableKind;

/// The declaration, as authored in `surface/forms.json`.
///
/// **Compiled in with `include_str!`, so there is exactly one copy** -- the
/// same reason `dispatch::TABLE` is, and the same reason `intent init` works
/// offline from embedded canon (`AC-07.1`). A daemon that read this from disk
/// could serve a form whose declaration is missing or stale.
pub const FORMS: &str = include_str!("../../../../../surface/forms.json");

/// A form declaration for one entity kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
  /// The entity kind this form lays out.
  ///
  /// **The face is DERIVED from this, never declared beside it.** An earlier
  /// shape had the declaration carry both, which is two homes for one fact and
  /// admits a form pointing at a face that is not its entity's -- the same
  /// second-home defect the whole module exists to refuse, one level up from
  /// the field set.
  pub entity: String,
  /// The rows, IN THE ORDER THEY ARE RENDERED. `AC-17.5` makes tab order
  /// declaration order, so this vector IS the tab order and there is no
  /// second field carrying it.
  pub fields: Vec<Field>,
}

/// One row of a form: layout only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
  /// The schema property this row renders. **Checked to exist; never trusted.**
  pub name: String,
  /// What the row is called on screen. Free text -- a label is presentation,
  /// which is exactly what a form declaration is for.
  pub label: String,
  /// One of the closed vocabulary declared in the file. **Checked; an unknown
  /// widget is refused by name rather than skipped** (`AC-17.4`).
  pub widget: String,
  /// Whether this realiser offers to edit the row.
  ///
  /// **A DECLARATION, NOT AN AUTHORITY.** The mutation surface decides what
  /// can actually be written; this says whether the form OFFERS it. A row
  /// declaring `true` for a field the surface refuses is a form that invites
  /// an edit the store will reject, so [`Loaded::offers_an_edit_the_surface_refuses`]
  /// reports the disagreement rather than letting the realiser discover it
  /// after the keystroke.
  pub editable: bool,
}

/// A widget in the closed vocabulary, with the reason it is in it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
  pub value: String,
  pub gloss: String,
}

/// The whole authored file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Declaration {
  pub about: String,
  /// **THE CLOSED VOCABULARY, DECLARED IN THE FILE** (`AC-17.4`), the way
  /// `entry_dispositions` and `target_states` are declared in the register
  /// rather than living in match arms nobody can enumerate.
  pub widgets: Vec<Widget>,
  pub forms: Vec<Form>,
}

/// Why a declaration was refused. **Every variant names the thing**, because
/// `AC-17.2` and `AC-17.4` both say *refused BY NAME* -- a count of problems
/// sends the author looking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormError {
  /// The file is not the shape the type says.
  Unparseable(String),
  /// A field names a property the entity's published face does not carry.
  ///
  /// **THIS IS THE REFUSAL THE WHOLE DESIGN EXISTS FOR.** It fires the day a
  /// model field is renamed, at load, naming both the form and the field --
  /// rather than rendering a form with a hole in it.
  NoSuchProperty {
    entity: String,
    field: String,
    face: String,
    /// What the face DOES carry, so the author can see the rename.
    available: Vec<String>,
  },
  /// A widget outside the declared vocabulary.
  UnknownWidget {
    entity: String,
    field: String,
    widget: String,
    declared: Vec<String>,
  },
  /// A form naming an entity kind with no published face.
  NoSuchFace { entity: String, face: String },
  /// Two rows rendering one property. **Not a layout choice**: the value would
  /// appear twice and an edit through one would leave the other stale on
  /// screen, which is the divergent-copy shape at render time.
  DuplicateProperty { entity: String, field: String },
}

impl std::fmt::Display for FormError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Unparseable(why) => write!(f, "the form declaration does not parse: {why}"),
      Self::NoSuchProperty {
        entity,
        field,
        face,
        available,
      } => write!(
        f,
        "the `{entity}` form declares a field `{field}` that `{face}` does not carry -- \
         the face declares {}",
        available.join(", ")
      ),
      Self::UnknownWidget {
        entity,
        field,
        widget,
        declared,
      } => write!(
        f,
        "the `{entity}` form gives `{field}` the widget `{widget}`, which is not in the \
         declared vocabulary -- the file declares {}",
        declared.join(", ")
      ),
      Self::NoSuchFace { entity, face } if face.is_empty() => write!(
        f,
        "there is no form for `{entity}` -- the entity kinds with a published face are \
         thread, wp and issue"
      ),
      Self::NoSuchFace { entity, face } => write!(
        f,
        "the `{entity}` form resolves against `{face}`, which is not a published face"
      ),
      Self::DuplicateProperty { entity, field } => write!(
        f,
        "the `{entity}` form renders `{field}` on two rows -- one property, one row"
      ),
    }
  }
}

/// A declaration that has been resolved against the published faces.
#[derive(Debug, Clone)]
pub struct Loaded {
  declaration: Declaration,
}

impl Loaded {
  /// Parse and resolve [`FORMS`], refusing by name on the first failure of
  /// either kind.
  pub fn load() -> Result<Self, FormError> {
    Self::from_str(FORMS)
  }

  /// The same, over supplied bytes -- so a test can drive a BROKEN declaration
  /// without a fixture file on disk.
  ///
  /// **A refusal that is never driven is a refusal nobody knows fires**, and
  /// the only honest way to drive these two is to hand the loader a
  /// declaration that trips them.
  pub fn from_str(bytes: &str) -> Result<Self, FormError> {
    let declaration: Declaration =
      serde_json::from_str(bytes).map_err(|why| FormError::Unparseable(why.to_string()))?;

    let vocabulary: BTreeSet<&str> = declaration
      .widgets
      .iter()
      .map(|w| w.value.as_str())
      .collect();

    for form in &declaration.forms {
      let pointer = face_for(&form.entity).ok_or_else(|| FormError::NoSuchFace {
        entity: form.entity.clone(),
        face: String::new(),
      })?;
      let face = face_properties(pointer).ok_or_else(|| FormError::NoSuchFace {
        entity: form.entity.clone(),
        face: pointer.to_string(),
      })?;

      let mut seen: BTreeSet<&str> = BTreeSet::new();
      for field in &form.fields {
        if !face.contains(field.name.as_str()) {
          return Err(FormError::NoSuchProperty {
            entity: form.entity.clone(),
            field: field.name.clone(),
            face: pointer.to_string(),
            available: face.iter().map(|p| (*p).to_string()).collect(),
          });
        }
        if !vocabulary.contains(field.widget.as_str()) {
          return Err(FormError::UnknownWidget {
            entity: form.entity.clone(),
            field: field.name.clone(),
            widget: field.widget.clone(),
            declared: vocabulary.iter().map(|w| (*w).to_string()).collect(),
          });
        }
        if !seen.insert(field.name.as_str()) {
          return Err(FormError::DuplicateProperty {
            entity: form.entity.clone(),
            field: field.name.clone(),
          });
        }
      }
    }

    Ok(Self { declaration })
  }

  /// The forms, in declaration order.
  pub fn forms(&self) -> &[Form] {
    &self.declaration.forms
  }

  /// The closed widget vocabulary, as declared.
  pub fn widgets(&self) -> &[Widget] {
    &self.declaration.widgets
  }

  /// One form by entity kind.
  pub fn form(&self, entity: &str) -> Option<&Form> {
    self.declaration.forms.iter().find(|f| f.entity == entity)
  }

  /// **THE CONVERSE ARM OF `AC-17.2`: a property that CAN be changed and
  /// appears on NO form, named rather than silently unreachable.**
  ///
  /// This is the half that rots. The forward refusal fires loudly at load the
  /// day a field is renamed; this one fires the day a field is ADDED, and
  /// **nothing about a form merely missing a row looks wrong**. Without it a
  /// new writable field is unreachable through every realiser and the only
  /// symptom is that nobody ever edits it.
  ///
  /// **Reachable means `None` or `Elsewhere`, not `None` alone.** A field a
  /// machine owns is still changeable -- through the machine's verb, which is
  /// what a `select` row is -- so counting it unreachable would report the
  /// form's best-specified rows as gaps.
  ///
  /// Returns `(entity, property)` pairs, sorted.
  pub fn changeable_and_not_on_any_form(&self) -> Vec<(String, String)> {
    self.audit(
      |kind| matches!(kind, Reach::NarrowSetter | Reach::AnotherDoor),
      false,
    )
  }

  /// Rows that OFFER an edit that can never land, by any door.
  ///
  /// The mirror of [`Self::changeable_and_not_on_any_form`], and it exists for
  /// the same reason one keystroke later: a form promising an edit the store
  /// rejects teaches the operator that the tool is unreliable, and the
  /// refusal arrives after they have typed.
  pub fn offers_an_edit_that_cannot_land(&self) -> Vec<(String, String)> {
    self.audit(|kind| matches!(kind, Reach::Never | Reach::NotYet), true)
  }

  /// One walk over every declared form, selecting properties by how they can
  /// be reached.
  ///
  /// **One traversal, two questions**, because two near-identical loops over
  /// one population is how the two answers drift apart.
  fn audit(&self, want: impl Fn(Reach) -> bool, on_the_form: bool) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for form in &self.declaration.forms {
      let Some(entity) = probe_entity(&form.entity) else {
        continue;
      };
      let Some(face) = face_for(&form.entity).and_then(face_properties) else {
        continue;
      };
      let declared: BTreeSet<&str> = form
        .fields
        .iter()
        .filter(|f| !on_the_form || f.editable)
        .map(|f| f.name.as_str())
        .collect();
      for property in face {
        if declared.contains(property) != on_the_form {
          continue;
        }
        if want(reach(&entity, property)) {
          found.push((form.entity.clone(), property.to_string()));
        }
      }
    }
    found.sort();
    found
  }
}

/// How a property can be changed, if it can.
///
/// **The estate's ruled axis, not a second one.** `UnsettableKind` was ruled by
/// vc on 2026-08-25 as *can I change this, and if so how*, over a taxonomy of
/// WHY the refusal exists -- because the why-axis put the majority case with
/// its opposite. That is the reader a form has, so this is a naming of that
/// answer rather than a re-derivation of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reach {
  /// The narrow setter writes it.
  NarrowSetter,
  /// A route exists through another door -- a machine's verb, a child address
  /// -- and the remedy names it. **A `select` or `button` row.**
  AnotherDoor,
  /// No route, by design, and there never will be one. Read-only, honestly.
  Never,
  /// No route, and that is a gap rather than a decision. **A form must not
  /// offer it**, and a report must be able to say the bucket is empty rather
  /// than drop the line.
  NotYet,
}

/// Every property of one entity kind's face, with how it can be changed.
///
/// **The realiser needs this, not just the checker.** A `select` row has to
/// know the door it is offering, and a read-only row has to know whether it is
/// `Never` (honestly permanent) or `NotYet` (a gap that will close). Returned
/// in face order so a caller cannot infer meaning from a sort.
pub fn face_reach(entity: &str) -> Option<Vec<(String, Reach)>> {
  let addr = probe_entity(entity)?;
  let face = face_properties(face_for(entity)?)?;
  Some(
    face
      .into_iter()
      .map(|p| (p.to_string(), reach(&addr, p)))
      .collect(),
  )
}

/// The published face one entity kind resolves against, as a POINTER.
///
/// **`wp` is a definition, not a root.** See [`face_properties`] for what
/// naming it by bare filename did and why it was invisible.
fn face_for(entity: &str) -> Option<&'static str> {
  match entity {
    "thread" => Some("thread.schema.json"),
    "wp" => Some("thread.schema.json#/$defs/WorkPackage"),
    "issue" => Some("issue.schema.json"),
    _ => None,
  }
}

fn reach(entity: &AddrEntity, field: &str) -> Reach {
  match crate::facade::unsettable_kind(entity, field) {
    None => Reach::NarrowSetter,
    Some(UnsettableKind::Elsewhere) => Reach::AnotherDoor,
    Some(UnsettableKind::Never) => Reach::Never,
    Some(UnsettableKind::NotYet) => Reach::NotYet,
  }
}

/// The properties of one published face, read from [`crate::faces::faces`].
///
/// **The face itself, not a re-derivation.** `faces()` renders the same bytes
/// that are committed under `schema/` and that `schema_faces_drift` holds to
/// the model, so a form checked against this is checked against the artefact a
/// consumer compiles against.
///
/// # NOT EVERY ENTITY IS A ROOT, AND GETTING THAT WRONG FAILS SILENTLY
///
/// `Thread` and `Issue` are schema roots. **`WorkPackage` is not** -- it lives
/// at `$defs/WorkPackage` inside `thread.schema.json`. An earlier version of
/// this function mapped `wp` to that FILE and read its ROOT properties, which
/// handed a work-package form the THREAD's 18 fields.
///
/// **It would not have looked broken.** Six of `WorkPackage`'s nine properties
/// -- `title`, `status`, `status_reason`, `objective`, `body`, `preamble` --
/// exist on `Thread` under the same names, so the forward refusal stays quiet
/// on all of them; `criteria`, `wps` and `attachments` would have RESOLVED on
/// a form for an entity that has none. That is exactly the silent-partial
/// class `AC-17.2` exists to remove, reproduced inside the loader that
/// enforces it.
///
/// So a face is named by a POINTER, in JSON Schema's own idiom: `<file>` for a
/// root, `<file>#/$defs/<Name>` for a definition. **The pointer is not
/// optional sugar** -- a bare filename for a nested entity is the defect
/// above, and there is no spelling that means "the definition, whichever one".
fn face_properties(face: &str) -> Option<BTreeSet<&'static str>> {
  let (file, pointer) = match face.split_once('#') {
    Some((file, pointer)) => (file, Some(pointer)),
    None => (face, None),
  };
  let (_, body) = crate::faces::faces()
    .into_iter()
    .find(|(name, _)| *name == file)?;
  let parsed: Value = serde_json::from_str(&body).ok()?;

  let node = match pointer {
    None => &parsed,
    Some(pointer) => {
      let mut node = &parsed;
      for segment in pointer.trim_start_matches('/').split('/') {
        node = node.get(segment)?;
      }
      node
    }
  };

  let properties = node.get("properties")?.as_object()?;
  // Leaked deliberately and once per face: the face is generated from
  // compiled-in types, so its key set is fixed for the life of the process.
  Some(
    properties
      .keys()
      .map(|k| &*Box::leak(k.clone().into_boxed_str()))
      .collect(),
  )
}

/// A synthetic address for asking the mutation surface about a KIND.
///
/// **The id is never dereferenced.** `unsettable_kind` matches on the address
/// VARIANT; no store is opened and no row is read. Using a real id here would
/// make the answer look instance-specific when it is not.
fn probe_entity(entity: &str) -> Option<AddrEntity> {
  let id = |s: &str| s.to_string();
  match entity {
    "thread" => Some(AddrEntity::Thread { id: id("ST0000") }),
    "issue" => Some(AddrEntity::Issue { id: id("0000") }),
    "wp" => Some(AddrEntity::Wp {
      thread: id("ST0000"),
      wp: id("01"),
    }),
    _ => None,
  }
}

/// One resolved form row: the generic description every renderer consumes.
///
/// **THIS IS THE SHARED DERIVATION, AND ITS PLACEMENT IS THE POINT** (vc,
/// 2026-08-30). `tui-design.md` §10a: the daemon resolves the form declaration
/// server-side and emits a generic description, and *the JS renders
/// `{label, value, widget}` triples, so does SwiftUI, so does the TUI*. It was
/// first built in `intent-cli/src/tui/views.rs` -- one crate too high, because
/// `intentd` depends on `intentsvcs` and NOT on the CLI, so the daemon emitter
/// would have had to write the same walk again. **Two homes for one derivation,
/// arriving by the door the argument was meant to shut.**
///
/// The line: DERIVATION -- entity + declaration -> triples -- is shared and
/// lives here beside [`Form`]. RENDERING -- triples to terminal lines, HTML or
/// SwiftUI views -- is per face and stays in the face. That is what makes
/// `AC-17.1`'s agreement structural rather than coincidental: one function
/// called from three places.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Triple {
  pub name: String,
  pub label: String,
  pub widget: String,
  pub value: String,
  pub editable: bool,
}

/// Resolve `form` against `entity`: one triple per declared field, in
/// declaration order.
///
/// **VALUES ARE READ OUT OF THE SERIALISED ENTITY, NOT OUT OF A MATCH.** A
/// `match field { "title" => e.title, ... }` is a second home for the field set
/// -- the thing `AC-17.2` refuses one layer up -- and it goes stale the day a
/// property is renamed, SILENTLY, because a missing arm looks exactly like an
/// empty value. Indexing by the declared name makes the declaration the only
/// list, and it is already held against the schema.
///
/// **A FIELD THE ENTITY DOES NOT CARRY YIELDS AN EMPTY VALUE, NEVER A MISSING
/// ROW.** Tab order is declaration order (`AC-17.5`), so a skipped row moves
/// every row after it and the operator's muscle memory lands on the wrong
/// field. An empty value is visible; a missing row is not.
pub fn triples(form: &Form, entity: &Value) -> Vec<Triple> {
  form
    .fields
    .iter()
    .map(|f| Triple {
      name: f.name.clone(),
      label: f.label.clone(),
      widget: f.widget.clone(),
      value: entity.get(&f.name).map(scalar).unwrap_or_default(),
      editable: f.editable,
    })
    .collect()
}

/// One JSON value as one line.
///
/// **A COLLECTION IS ITS SIZE, NEVER ITS CONTENTS.** ST0056 carries 297
/// attachments; inlining them makes the form 325 rows of which 297 are files,
/// and it breaks the TUI's alignment guarantee outright -- one aligned name
/// column cannot serve `title` and `parity/tools/conservation_check.sh` at
/// once. The same count is what the web face puts on its own row, which is why
/// this lives here rather than in either renderer.
fn scalar(v: &Value) -> String {
  match v {
    Value::Null => String::new(),
    Value::Bool(b) => b.to_string(),
    Value::Number(n) => n.to_string(),
    Value::Array(a) => a.len().to_string(),
    // Present-or-not. `fiat` is the live case: what matters on the row is that
    // there IS one, and its content is a descent away.
    Value::Object(_) => "set".to_string(),
    Value::String(s) => one_line(s),
  }
}

/// Collapse whitespace so a value cannot become two rows.
///
/// Criterion prose reaches 59,061 characters with paragraph breaks in it. Each
/// face clips to its own width, but only after this has made the value one
/// line -- collapsing at the renderer instead would put the reason somewhere
/// the next reader would not look, and would have to be done identically in
/// three places.
fn one_line(s: &str) -> String {
  s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The bytes a field HANDS TO AN EDITOR: the entity's own value, uncollapsed.
///
/// **THIS EXISTS BECAUSE [`Triple::value`] IS LOSSY BY DESIGN AND IS THE
/// OBVIOUS THING TO REACH FOR.** A row is one line, so [`one_line`] collapses
/// every run of whitespace in it -- correct on screen, and catastrophic the
/// moment that same string is what gets written to a scratch file and handed
/// to `$EDITOR`. **The operator opens their objective, changes one word, saves,
/// and every paragraph break in it is gone** -- not by the editor's doing, but
/// because the bytes it was given were already a rendering.
///
/// That is the destroys-authored-prose class `AC-17.10` names, arriving one
/// step EARLIER than the criterion warns about: 17.10 guards the RETURN
/// repainting from a stale model, and this is the DEPARTURE handing the editor
/// a lossy render of a fresh one. Same destruction, opposite end of the trip,
/// and no test of the return path can see it.
///
/// **SHARED RATHER THAN TUI-LOCAL, AND THE RADIUS IS THE POINT.** The web
/// face's textarea needs exactly these bytes for exactly this reason, and
/// `intentd` cannot reach into the CLI. A copy in each face is two answers to
/// *what is the real value of this field*, which is the second home
/// [`triples`] was moved down a crate to avoid.
///
/// `None` for a field the entity does not carry, and for one whose value is
/// not text -- **an array or an object has no bytes to edit**, and returning
/// `""` for one would offer an edit that silently replaces a collection with a
/// string.
pub fn raw(entity: &Value, field: &str) -> Option<String> {
  match entity.get(field)? {
    Value::String(s) => Some(s.clone()),
    Value::Null => Some(String::new()),
    _ => None,
  }
}
