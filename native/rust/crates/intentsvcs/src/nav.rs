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

use crate::address::Entity;
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
/// The parsed schema face for `kind`, resolved through its fragment.
///
/// **THE POINTER COMES FROM [`crate::form::face_for`], WHICH IS NOW ITS ONLY
/// HOME.** This module carried its own `face_pointer` with byte-identical
/// arms until 2026-08-30, and the comment on it named the other copy without
/// removing it -- *`crate::form` says the same thing about the same pointer.*
///
/// **`wp` IS A DEFINITION, NOT A ROOT**, and the fragment is load-bearing:
/// naming the bare file would look at `thread`'s properties while claiming to
/// look at a work package's, and it would go UNNOTICED, because today the wp
/// form's only `button` row (`fiat`) misses at both levels, so the right answer
/// comes out of the wrong lookup. The first descent added to the wp form would
/// read the wrong schema in silence. That reasoning is why the mapping is worth
/// one home rather than two agreeing ones.
pub fn face_json(kind: &str) -> Option<serde_json::Value> {
  let pointer = crate::form::face_for(kind)?;
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

/// Where `intent explore [address]` opens, and WHY it opened there.
///
/// **hv, 2026-08-30, overriding vc's no-address ruling of the same day:** *I
/// see no reason why `intent expl[ore] ...` couldn't take an `intent://...`
/// URL scheme URI or even an ID that it tries to match to something if it can,
/// and if it can't it just opens at the root.*
///
/// # It lives here because the WEB FACE needs the same answer
///
/// A URL bar and a jump-to box resolve exactly this, and `intentd` cannot
/// reach `intent-cli`. Putting it in the face would mean the two faces resolve
/// an operator's spelling by two derivations -- which is the same defect
/// `AC-17.12` forbids one level down, where the two faces must not derive
/// their own path segments.
///
/// # The fallback ANNOUNCES itself, and that is not a widening of the ruling
///
/// hv said *it just opens at the root*, which contrasts with REFUSING rather
/// than with TELLING (vc, ruled). A browser that silently opens somewhere
/// other than you asked is the answer-confidently-from-partial-evidence class
/// this rewrite exists to remove, so [`Landing::Root`] carries the reason and
/// the face puts it on the info row.
///
/// **This is also why `explore` and `edit` diverge on a miss and neither is
/// wrong**: `intent edit ST9999` must REFUSE, because it was asked to act on a
/// specific thing; `intent explore ST9999` opens at the root, because it was
/// asked to open the explorer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Landing {
  /// The address named a view this surface can show, and the entity is there.
  At(View),
  /// The root, plus the reason it is not where the operator asked for.
  Root(Unlanded),
}

/// Why an address did not become a view.
///
/// # Three shapes over five input cases, and the split is deliberate
///
/// The five cases an operator can produce are: it resolves and is present
/// ([`Landing::At`]); it resolves and is ABSENT; it is a real address whose
/// form has no view; it names TWO things; it names nothing. **The last two
/// share a variant because [`crate::address`] already tells them apart IN
/// WORDS**, and its author wrote those words carefully -- re-deriving the
/// distinction here would be a second, worse copy of a message that already
/// exists, exactly as re-wording the editor launcher's error would be.
///
/// **`Absent` is the one that is easy to miss and it is the reason this type
/// takes a presence test at all.** [`crate::address::promote`] is purely
/// SYNTACTIC: it never reads the store, so `ST9999` resolves perfectly. A
/// landing computed from the grammar alone would open the thread form with
/// every value blank -- and `render` deliberately renders a form that cannot
/// load *with its field names intact*, because an empty screen would falsely
/// claim the entity has no fields. The consequence is that a thread which does
/// not exist and a thread which exists and is empty PAINT THE SAME SCREEN.
/// That is reachable-and-blank reading as data, and no test over well-formed
/// present ids can see it.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Unlanded {
  /// The address parser refused, IN ITS OWN WORDS.
  ///
  /// **It carries the parser's REMEDY as well as its message, because the
  /// parser wrote both** and passing on only the diagnosis would drop the half
  /// that tells the operator what to type instead.
  #[error("{why}")]
  Unreadable {
    input: String,
    why: String,
    remedy: String,
  },
  /// A real, well-formed address for a form this surface has no view for.
  ///
  /// **Not the same fact as naming nothing, and telling an operator that
  /// `intent:///nodes/vc` named nothing would be FALSE** -- it sends them
  /// hunting for a thing they already have, which is the shape of a gate that
  /// starts a search instead of ending one.
  #[error("`{input}` is a real address, and this surface has no view for a {form}")]
  NoView { input: String, form: String },
  /// A real address, a view this surface can show, and nothing there.
  #[error("`{input}` is well formed, and this project has no such {kind}")]
  Absent {
    input: String,
    kind: String,
    view: View,
  },
}

/// The view an entity address opens, or `None` where this surface has none.
///
/// # A total match, not a lookup table
///
/// Six of thirteen [`Entity`] forms have views. The seven that do not are
/// listed by name rather than falling through a wildcard, so **a fourteenth
/// form does not compile until someone decides which it is** -- the same
/// discipline [`Entity::form`] states for its own arm set, applied to a second
/// question about the same enum.
///
/// # This function is a TRANSLATION, and it is the only place the two
/// vocabularies meet
///
/// **The address grammar and the view ladder disagree, both deliberately.** An
/// address says `/threads/ST0056/wp` and `/threads/ST0056/ac`, because D57-8
/// gives collections REST-shaped plural addresses. A view path says
/// `/thread/ST0056/wps` and `/thread/ST0056/criteria`, because `AC-17.12` ruled
/// that every view segment is a name the FORM DECLARATION already carries and
/// nothing is pluralised. Neither is wrong and neither derives the other --
/// nothing declares that `ac` and `criteria` are one concept.
///
/// So the reconciliation has to be authored, and the danger is that a second
/// one gets authored somewhere else. **This is the one home**, and
/// `every_child_view_this_maps_to_is_a_descent_the_declaration_carries` holds
/// its output against the declaration so the two cannot drift in silence.
fn view_for(entity: &Entity) -> Option<View> {
  let item = |kind: &str, id: &str| View::Item {
    kind: kind.to_string(),
    id: id.to_string(),
  };
  let children = |thread: &str, field: &str| View::Children {
    kind: "thread".to_string(),
    id: thread.to_string(),
    field: field.to_string(),
  };
  match entity {
    Entity::Threads => Some(View::Collection {
      kind: "thread".to_string(),
    }),
    Entity::Issues => Some(View::Collection {
      kind: "issue".to_string(),
    }),
    Entity::Thread { id } => Some(item("thread", id)),
    Entity::Issue { id } => Some(item("issue", id)),
    Entity::WpCollection { thread } => Some(children(thread, "wps")),
    Entity::AcCollection { thread } => Some(children(thread, "criteria")),
    // **`wp` IS A DECLARED KIND WHOSE ITEM VIEW NOTHING REACHES.** No
    // navigation push produces `View::Item { kind: "wp" }` and the realiser
    // answers `None` for it, so landing there would paint a form whose every
    // value is blank -- for every work package, not just a missing one. Its
    // COLLECTION renders, which is why the arm above is `Some`.
    Entity::Wp { .. }
    // A criterion, a test and an attachment are ROWS INSIDE a collection this
    // surface renders, not items with views of their own.
    | Entity::Ac { .. }
    | Entity::At { .. }
    | Entity::Attachment { .. }
    // The whiteboard and the event log are addressable and are not in the form
    // declaration at all.
    | Entity::Node { .. }
    | Entity::NodeInbox { .. }
    | Entity::Event { .. } => None,
  }
}

/// Resolve an operator's spelling to a [`Landing`].
///
/// # One call, and nothing new resolves anything
///
/// [`crate::address::promote`] tests for the `intent://` scheme itself and
/// delegates to [`crate::address::parse`], else asks
/// [`crate::model::normalise_id`]. So both spellings hv named reach the
/// estate's ONE existing door and this function dispatches between nothing.
///
/// **A `/thread/ST0056` path spelling was considered and refused**:
/// [`View::parse`] validates nothing -- `/banana` parses as a collection -- so
/// accepting it would need fresh validation against [`kinds`], and that fresh
/// validation would be the second resolver the no-address ruling was right to
/// fear.
///
/// # Presence is INJECTED
///
/// `present` keeps this module free of the facade, the same way the TUI's
/// editor launcher is passed in as a closure rather than resolved where it is
/// used. Both faces then share one presence rule instead of writing two.
pub fn land(input: &str, present: impl Fn(&View) -> bool) -> Landing {
  let address = match crate::address::promote(input) {
    Ok(a) => a,
    Err(why) => {
      return Landing::Root(Unlanded::Unreadable {
        input: input.to_string(),
        why: why.to_string(),
        remedy: crate::remedy::Remedy::remedy(&why),
      });
    }
  };
  let Some(view) = view_for(&address.entity) else {
    return Landing::Root(Unlanded::NoView {
      input: input.to_string(),
      form: address.entity.form().to_string(),
    });
  };
  if present(&view) {
    Landing::At(view)
  } else {
    Landing::Root(Unlanded::Absent {
      input: input.to_string(),
      kind: address.entity.form().to_string(),
      view,
    })
  }
}

/// **THE REMEDY SAYS WHAT TO TYPE INSTEAD, AND FOR A REFUSED SPELLING IT IS THE
/// PARSER'S OWN.** `explore` is the forgiving door -- it opens at the root
/// rather than refusing -- so the remedy is the only part of the exchange that
/// moves the operator forward, and re-writing one that already exists would be
/// a second, worse copy.
impl crate::remedy::Remedy for Unlanded {
  fn remedy(&self) -> String {
    match self {
      Unlanded::Unreadable { remedy, .. } => remedy.clone(),
      Unlanded::NoView { form, .. } => format!(
        "the explorer opens threads and issues; a {form} is addressable but has no view yet, so \
         reach it with `intent edit` or `intent browse`"
      ),
      Unlanded::Absent { kind, .. } => match kind.as_str() {
        "issue" => "list what is there with `intent issue list`".into(),
        _ => "list what is there with `intent st list`".into(),
      },
    }
  }
}
