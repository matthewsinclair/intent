//! Navigation as a SHARED contract: `AC-17.7` and `AC-17.12`.
//!
//! **THIS IS HERE AND NOT IN A FACE BECAUSE THE PATH IS THE CONTRACT** (vc,
//! 2026-08-30). `AC-17.12` says the TUI's view stack and the browser's URL are
//! the same sequence from the same declarations. If the web face derives its
//! own segments, the two faces are no longer the same sequence and the
//! criterion is unsatisfiable by construction -- so the derivation lives in the
//! crate both faces depend on, exactly like [`crate::form::triples`].
//!
//! **AND `View` TRAVELS WITH IT BECAUSE THE SHARED THING IS A ROUND TRIP.** The
//! contract is not "a path function", it is `View <-> path string`, and a
//! function cannot be shared without its argument type. [`View::path`] and
//! [`View::parse`] are held to that round trip over every view the real
//! declaration can produce.
//!
//! # What deliberately did NOT come here
//!
//! **The view STACK stays in the face.** `push`/`pop`/`depth`/`trail` are how
//! ONE face remembers where it has been, and the web's equivalent is browser
//! history -- which it already has and must not be given a second copy of. A
//! breadcrumb is a rendering, not a shared fact.
//!
//! # Nothing here is hand-written, which is the criterion
//!
//! `AC-17.7`: *nav is DERIVED from the model, not hand-built ... a hand-written
//! navigation tree is the same second home as a hand-written field list and
//! goes stale the same way.*
//!
//! - **Entity kinds come from the form declaration** ([`kinds`]), which
//!   `AC-17.2` already holds against the schema faces.
//! - **Descents come from the schema** ([`descents`]). `surface/forms.json`
//!   glosses `button` as *a DESCENT or an ACTION*, so the widget alone cannot
//!   tell `wps` from `fiat`. The schema can: a descent's property is
//!   `"type": "array"` with an `items.$ref`, and that `$ref` names the child
//!   kind. `fiat` is an object with neither.
//!
//! # Every segment is a name the declaration already carries
//!
//! `/thread/ST0056/wps`, never `/threads/ST0056/work-packages`. **No segment is
//! invented, pluralised or prettified**, because a spelling rule is a second
//! home for naming, it breaks on the first kind that does not take `-s`, and it
//! has to be inverted to route. If a face wants plurals for display that is a
//! rendering layered on one derived path, never a second derivation.

use serde::{Deserialize, Serialize};

use crate::form::Loaded;

/// One level of the ladder `AC-17.7` names: entity-kind, collection, item,
/// child.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum View {
  /// The root: every declared entity kind.
  Entities,
  /// Every item of one kind.
  Collection { kind: String },
  /// One item -- the form.
  Item { kind: String, id: String },
  /// A collection hanging off one item, named by the field that declares it.
  Children {
    kind: String,
    id: String,
    field: String,
  },
}

impl View {
  /// The path for this view, in the browser and in the trail.
  pub fn path(&self) -> String {
    match self {
      View::Entities => "/".to_string(),
      View::Collection { kind } => format!("/{kind}"),
      View::Item { kind, id } => format!("/{kind}/{id}"),
      View::Children { kind, id, field } => format!("/{kind}/{id}/{field}"),
    }
  }

  /// The inverse of [`View::path`]. `None` for anything that is not a path this
  /// module would have produced -- **a spelling that names nothing is refused
  /// as a spelling**, never resolved to something near it.
  pub fn parse(path: &str) -> Option<View> {
    let trimmed = path.strip_prefix('/')?;
    if trimmed.is_empty() {
      return Some(View::Entities);
    }
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.iter().any(|p| p.is_empty()) {
      return None;
    }
    match parts.as_slice() {
      [kind] => Some(View::Collection {
        kind: (*kind).to_string(),
      }),
      [kind, id] => Some(View::Item {
        kind: (*kind).to_string(),
        id: (*id).to_string(),
      }),
      [kind, id, field] => Some(View::Children {
        kind: (*kind).to_string(),
        id: (*id).to_string(),
        field: (*field).to_string(),
      }),
      _ => None,
    }
  }
}

/// A descent declared by one entity kind: the field that opens it and the kind
/// it opens onto.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Descent {
  pub field: String,
  pub label: String,
  pub child: String,
}

/// Every entity kind with a declared form, in declaration order.
///
/// **This is the root's row set and it is derived, so a new form is a new row
/// with nothing to remember.**
pub fn kinds(loaded: &Loaded) -> Vec<String> {
  loaded.forms().iter().map(|f| f.entity.clone()).collect()
}

/// The descents `kind` declares: `button` rows whose schema property is an
/// array of a `$ref`'d definition.
///
/// A `button` row that is not such an array is an ACTION (`fiat`) and is
/// deliberately absent -- descending into it would be descending into a verb.
pub fn descents(loaded: &Loaded, kind: &str) -> Vec<Descent> {
  let Some(form) = loaded.form(kind) else {
    return Vec::new();
  };
  let Some(schema) = face_json(kind) else {
    return Vec::new();
  };
  form
    .fields
    .iter()
    .filter(|f| f.widget == "button")
    .filter_map(|f| {
      let child = array_item_ref(&schema, &f.name)?;
      Some(Descent {
        field: f.name.clone(),
        label: f.label.clone(),
        child,
      })
    })
    .collect()
}

/// The published face one entity kind resolves against, as a POINTER.
///
/// **`wp` IS A DEFINITION, NOT A ROOT**, and the fragment is load-bearing:
/// [`crate::form`] says the same thing about the same pointer. Naming the bare
/// file here would look at `thread`'s properties while claiming to look at a
/// work package's -- and it would go UNNOTICED, because today the wp form's
/// only `button` row (`fiat`) misses at both levels, so the right answer comes
/// out of the wrong lookup. The first descent added to the wp form would read
/// the wrong schema in silence.
fn face_pointer(kind: &str) -> Option<&'static str> {
  match kind {
    "thread" => Some("thread.schema.json"),
    "wp" => Some("thread.schema.json#/$defs/WorkPackage"),
    "issue" => Some("issue.schema.json"),
    _ => None,
  }
}

/// The parsed schema face for `kind`, resolved through its fragment.
pub fn face_json(kind: &str) -> Option<serde_json::Value> {
  let pointer = face_pointer(kind)?;
  let (file, fragment) = match pointer.split_once('#') {
    Some((f, frag)) => (f, Some(frag)),
    None => (pointer, None),
  };
  let root: serde_json::Value = serde_json::from_str(&crate::faces::face(file)?).ok()?;
  match fragment {
    None => Some(root),
    Some(frag) => {
      let mut here = &root;
      for seg in frag.split('/').filter(|s| !s.is_empty()) {
        here = here.get(seg)?;
      }
      Some(here.clone())
    }
  }
}

/// The definition name `field` is an array of, or `None` when `field` is not an
/// array of a `$ref` -- which is exactly the descent/action discriminator.
fn array_item_ref(schema: &serde_json::Value, field: &str) -> Option<String> {
  let prop = schema.get("properties")?.get(field)?;
  if prop.get("type")?.as_str()? != "array" {
    return None;
  }
  let r = prop.get("items")?.get("$ref")?.as_str()?;
  Some(r.rsplit('/').next()?.to_string())
}
