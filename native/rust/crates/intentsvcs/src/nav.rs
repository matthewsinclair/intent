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

/// The path segments that are not entity kinds. See [`View::Settings`] and
/// [`View::Help`].
pub const SETTINGS_SEGMENT: &str = "settings";
pub const HELP_SEGMENT: &str = "help";

/// Every segment the entity namespace may not use.
///
/// **ONE LIST, so the reservation test cannot fall behind the reservations.**
/// A third reserved view added without a row here would be a collision nothing
/// checks -- which is the whole failure the first reservation was written to
/// make impossible.
pub const RESERVED: &[&str] = &[SETTINGS_SEGMENT, HELP_SEGMENT];

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
  /// One item INSIDE a child collection: the work package, not the list of them.
  ///
  /// **A WORK PACKAGE IS AN ITEM AND [`View::Item`] CANNOT HOLD ONE, WHICH IS
  /// WHY THIS IS A VARIANT RATHER THAN A LONGER `id`.** `Item` carries ONE id;
  /// a work package is named by two components -- the thread and the sequence
  /// -- and packing them into one string (`Item { kind: "wp", id: "ST0056/17" }`)
  /// puts a `/` inside a segment. [`View::path`] then renders `/wp/ST0056/17`
  /// and [`View::parse`] reads it back as `Children { kind: "wp", id: "ST0056",
  /// field: "17" }`.
  ///
  /// **THAT ROUND TRIP WAS THE WORST OF THE THREE AVAILABLE OUTCOMES: it
  /// neither round-tripped NOR refused.** A refusal would have been defensible
  /// -- `parse`'s own contract two doors down says a spelling that names
  /// nothing is refused as a spelling. Silently returning a DIFFERENT address
  /// is what `IN-AG-NO-SILENT-001` forbids outright, and it is what decided
  /// this shape over rewording `AC-17.6` (vc, 2026-09-09, under hv's pen).
  ///
  /// **THE SEGMENT REUSES A DECLARED DESCENT RATHER THAN INVENTING ONE.**
  /// `/thread/ST0056/wps/17` descends `wps` -- the same field [`Descent`]
  /// already declares and [`View::Children`] already addresses -- so the
  /// ladder reads collection, item, child-collection, child-item with nothing
  /// new to remember. `Collection -> Item` existed; `Children -> nothing` was
  /// the gap.
  Child {
    kind: String,
    id: String,
    field: String,
    item: String,
  },
  /// The whole key/command reference. Derived, never written out.
  ///
  /// **A SECOND RESERVED SEGMENT, AND THE COST IS THE SAME ONE
  /// [`View::Settings`] PAYS** -- `no_declared_entity_kind_is_reserved` holds
  /// both against the real declaration, so a form named `help` fails the suite
  /// rather than disappearing from the browser.
  Help {
    /// The command whose usage to show, or `None` for the whole surface.
    of: Option<String>,
  },
  /// The operator's own settings: [`crate::settings::DECLARED`], not the model.
  ///
  /// **THE ONE VIEW THAT IS NOT DERIVED FROM THE DECLARATION, AND IT IS HERE
  /// FOR THE REASON THE REST ARE: `View` IS WHAT A STACK HOLDS.** A settings
  /// screen that was not a `View` would need a second place for the face to
  /// remember it was there -- a parallel navigation model, which is the second
  /// home this module exists to refuse. `AC-17.7`'s no-trap property then
  /// covers it for free: it can be entered, and it can be left.
  ///
  /// **ITS SEGMENT IS RESERVED, WHICH IS A REAL COST AND IS PAID DELIBERATELY.**
  /// `/settings` would otherwise parse as `Collection { kind: "settings" }`, so
  /// an entity kind of that name becomes unaddressable. [`reserved_is_free`]
  /// holds the reservation against the REAL declaration rather than against an
  /// assumption -- a kind named `settings` fails the suite rather than
  /// disappearing from the browser.
  Settings,
}

impl View {
  /// The path for this view, in the browser and in the trail.
  pub fn path(&self) -> String {
    match self {
      View::Entities => "/".to_string(),
      View::Collection { kind } => format!("/{kind}"),
      View::Item { kind, id } => format!("/{kind}/{id}"),
      View::Children { kind, id, field } => format!("/{kind}/{id}/{field}"),
      View::Child {
        kind,
        id,
        field,
        item,
      } => format!("/{kind}/{id}/{field}/{item}"),
      View::Settings => format!("/{SETTINGS_SEGMENT}"),
      View::Help { of: None } => format!("/{HELP_SEGMENT}"),
      View::Help { of: Some(name) } => format!("/{HELP_SEGMENT}/{name}"),
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
      // **THE RESERVATION IS ANSWERED BEFORE THE ENTITY NAMESPACE, WHICH IS
      // WHAT MAKES IT A RESERVATION.** Checking it after would make the winner
      // depend on whether a form happened to be declared with this name.
      [seg] if *seg == SETTINGS_SEGMENT => Some(View::Settings),
      [seg] if *seg == HELP_SEGMENT => Some(View::Help { of: None }),
      // **THE RESERVATION REACHES THE SECOND SEGMENT TOO**, or `/help/st`
      // would parse as the ITEM `st` of an entity kind called `help` -- the
      // same collision one level down, and the one that would arrive the day
      // `/help <command>` was added without anyone re-reading this arm.
      [seg, name] if *seg == HELP_SEGMENT => Some(View::Help {
        of: Some((*name).to_string()),
      }),
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
      // **THE FOURTH SEGMENT IS THE LADDER'S LAST RUNG, NOT AN OPEN TAIL.**
      // `_ => None` still catches five and beyond, so this stays a closed
      // grammar: the arm below is the only path that ends inside a child
      // collection, and anything deeper is refused as a spelling exactly as
      // this function's contract promises.
      [kind, id, field, item] => Some(View::Child {
        kind: (*kind).to_string(),
        id: (*id).to_string(),
        field: (*field).to_string(),
        item: (*item).to_string(),
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
  /// An untagged number that names MORE THAN ONE thing which actually exists.
  ///
  /// **THE CANDIDATES ARE A LIST RATHER THAN A SENTENCE, WHICH IS THE WHOLE
  /// REASON THIS IS A VARIANT AND NOT A MESSAGE** (vc, 2026-08-31). The
  /// explorer renders them as rows to pick from, the MCP tier needs them
  /// structured, and a CLI verb prints a refusal -- three renderings of one
  /// fact, and a resolver that returned prose would force two of the three to
  /// parse English back.
  #[error("`{input}` names {} things in this project", candidates.len())]
  Ambiguous {
    input: String,
    candidates: Vec<crate::address::Address>,
  },
  /// A well-formed untagged number that names NOTHING here.
  ///
  /// **NOT THE SAME FACT AS [`Unlanded::Absent`], AND COLLAPSING THEM WOULD
  /// MISREPORT ONE.** `Absent` knows which collection was meant, because the
  /// spelling said so; this arrives from a spelling that named two and found
  /// neither, so there is no single `kind` to put in that sentence. Saying
  /// *this project has no such thread* about `0999` would be true and would
  /// answer about one of the two things the caller might have meant.
  #[error("`{input}` is well formed and names nothing in this project")]
  Unresolvable {
    input: String,
    searched: Vec<crate::address::Address>,
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
pub fn view_for(entity: &Entity) -> Option<View> {
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
    // **A WORK PACKAGE REACHES ITS OWN VIEW THROUGH THE DESCENT, NOT THROUGH
    // `View::Item`.** This arm answered `None` until 2026-09-09, and the reason
    // it gave was true at the time: *`wp` is a declared kind whose item view
    // nothing reaches* -- no navigation push produced `View::Item { kind: "wp" }`
    // and the realiser answered `None` for it, so landing there would have
    // painted a form whose every value is blank, for EVERY work package rather
    // than only a missing one.
    //
    // **`View::Child` retired that, and retiring it is what closes `AC-17.6`'s
    // second arm.** `intent edit wp ST0056/17` and `intent browse wp ST0056/17`
    // have to reach ONE model; while this answered `None` the browse arm could
    // not reach what the edit arm could, which is the disagreement the criterion
    // names. The view is the one the `wps` descent already renders and the one
    // its rows already door into, so nothing new is reachable that was not
    // reachable by hand -- what changes is that the ADDRESS now lands on it.
    Entity::Wp { thread, wp } => Some(View::Child {
      kind: "thread".to_string(),
      id: thread.to_string(),
      field: "wps".to_string(),
      item: wp.to_string(),
    }),
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

/// The inverse of [`view_for`] FOR ITEM VIEWS, which is the only direction a
/// face holding a kind and an id can travel.
///
/// **THIS IS NOT A RESOLVER, AND THE DIFFERENCE IS WHY IT IS ALLOWED TO
/// EXIST.** [`crate::resolve`] answers *what does this token NAME*, which needs
/// the store because a bare number names two things. This answers *what is the
/// address of the thing I am already looking at*: the kind is KNOWN, carried by
/// the view the operator navigated to, so nothing is inferred from a spelling
/// and no second ladder is created.
///
/// **IT EXISTS BECAUSE THE WIDTH ARM CAME OUT** (2026-08-31). The TUI's editor
/// reached `address::promote(&handoff.id)` and let the SPELLING decide, which
/// worked only while an untagged four-digit token silently meant `Issue`. With
/// that arm gone `promote("0164")` is ambiguous, so the TUI would have refused
/// to save an edit to any issue field -- **and no test would have caught it**,
/// because every TUI test drives `kind: "thread", id: "ST0056"`, which is
/// tagged and resolves either way. The kind the editor was already holding, and
/// underscoring in one of the two signatures, is now read.
///
/// `None` for a view that is not an item, and for any kind outside the two that
/// have item views at all. [`view_for`] is the roster --
/// `an_item_view_round_trips_to_the_entity_it_came_from` drives the pair
/// together so neither can grow an arm the other lacks.
pub fn entity_for_item(view: &View) -> Option<Entity> {
  match view {
    View::Item { kind, id } => match kind.as_str() {
      "thread" => Some(Entity::Thread { id: id.clone() }),
      "issue" => Some(Entity::Issue { id: id.clone() }),
      _ => None,
    },
    // **THE CHILD RUNG IS AN ITEM VIEW, WHICH IS WHY IT BELONGS IN THIS
    // FUNCTION AND NOT BESIDE IT.** A face standing on `/thread/ST0056/wps/17`
    // holds an item as surely as one standing on `/thread/ST0056`; the only
    // difference is that its address takes two components. This answered `None`
    // for every work package until 2026-09-09 -- the SECOND of the two
    // refusals `browser_url` used to cite as the reason a work package had no
    // browser address -- and `View::Child` is what retires both.
    //
    // The descent is matched by NAME rather than assumed: `wps` is the field
    // [`view_for`] authored on the way out, and reading it back anywhere else
    // would be a second spelling of one mapping.
    View::Child {
      kind,
      id,
      field,
      item,
    } if kind == "thread" && field == "wps" => Some(Entity::Wp {
      thread: id.clone(),
      wp: item.clone(),
    }),
    _ => None,
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
  // **ONE PROBE, DERIVED, RATHER THAN A SECOND CLOSURE INJECTED BESIDE
  // `present`.** The ruled shape was a second injected closure; this asks the
  // one that is already here. `exists` and `present` would be two probes of
  // nearly the same question -- *is this thing in the store* against *does this
  // view have content* -- and for an ITEM view they are the same question, so
  // two injections would be two homes that agree until a face wires one of them
  // differently. Deriving it also means a caller cannot supply a presence rule
  // for landing and a different one for resolving.
  //
  // **THE SINGLE DOOR WAS CHOSEN, NOT DEFAULTED, AND THE CONDITION FOR SPLITTING
  // IT IS RECORDED HERE ON PURPOSE** (vc's condition on accepting this over
  // their own ruled shape, 2026-08-31). If a face ever genuinely needs presence
  // and content to DIFFER -- an entity that exists whose view legitimately
  // renders empty -- that is the moment to separate them, and the separation is
  // a RULING rather than a quiet second injection. A reader who meets two
  // closures here later should be able to tell that somebody decided, rather
  // than that somebody added one.
  let exists =
    |a: &crate::address::Address| view_for(&a.entity).map(|v| present(&v)).unwrap_or(false);
  let address = match crate::resolve::resolve(input, exists) {
    Ok(crate::resolve::Resolution::Resolved(a)) => a,
    // **THE LADDER FOUND TWO AND REFUSES TO PICK.** Precedence here would be
    // `0189` one layer up: answering confidently about the entity the caller
    // did not name. 48 of 69 thread numbers on this estate are also issue
    // numbers, so it would be the common case rather than a corner.
    Ok(crate::resolve::Resolution::Ambiguous(candidates)) => {
      return Landing::Root(Unlanded::Ambiguous {
        input: input.to_string(),
        candidates,
      });
    }
    Ok(crate::resolve::Resolution::Unresolvable { searched }) => {
      return Landing::Root(Unlanded::Unresolvable {
        input: input.to_string(),
        searched,
      });
    }
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
      // **THE BROWSER ROUTE IS NAMED BY ITS WIRED SPELLING, NOT ITS UNWIRED
      // TWIN** (AC-06.11, found 2026-09-01 by the source-side corpus in
      // `remedies_are_reachable.rs`, which reaches this arm because no fixture
      // can provoke it). Driven: `intent browse <kind> <id>` answers `browse is
      // a known command that is not implemented yet`, while `intent edit <kind>
      // <id> --browser` is wired and refuses honestly, naming `intent daemon
      // start`. **THE DECLARATION IS NOT THE DEFECT AND IS NOT TOUCHED HERE**:
      // `browse` is declared ahead of its arm deliberately, because INV-09 /
      // `ST0058 AC-00.6` refuses a capability present by one spelling and absent
      // by the other, and hv ruled it ships on exactly that ground. **A REMEDY
      // IS NOT A CAPABILITY DECLARATION** -- it is advice an operator types
      // next, so it names the spelling that works TODAY and moves to the verb
      // the day the realiser lands.
      Unlanded::NoView { form, .. } => format!(
        "the explorer opens threads and issues; a {form} is addressable but has no view yet, so \
         reach it with `intent edit` or `intent edit --browser`"
      ),
      Unlanded::Absent { kind, .. } => match kind.as_str() {
        "issue" => "list what is there with `intent issue list`".into(),
        _ => "list what is there with `intent st list`".into(),
      },
      // **THE REMEDY IS THE ADDRESSES THEMSELVES, NOT ADVICE ABOUT THEM.** The
      // operator's next move is to name one, so naming both is the shortest
      // path to it -- and each is rendered by `to_url` rather than spelled
      // here, so a worked example cannot teach a form the tool refuses.
      Unlanded::Ambiguous { candidates, .. } => format!(
        "name the one you meant: {}",
        candidates
          .iter()
          .map(|a| a.to_url())
          .collect::<Vec<_>>()
          .join(" or ")
      ),
      Unlanded::Unresolvable { searched, .. } => format!(
        "nothing was found at {} -- list what is there with `intent st list` or `intent issue list`",
        searched
          .iter()
          .map(|a| a.to_url())
          .collect::<Vec<_>>()
          .join(" or ")
      ),
    }
  }
}
