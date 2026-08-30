//! Navigation: `AC-17.7`, and the stack `intent explore` roots one level above
//! `intent edit`.
//!
//! **`explore` IS NOT A SECOND SUBSYSTEM. IT IS THIS STACK WITH A DIFFERENT
//! BOTTOM.** `intent edit <kind> <id>` enters at [`View::Item`];
//! `intent explore` enters at [`View::Entities`]. Same views, same keys, same
//! menu. `AC-17.7` already described the ladder in its own words -- *entity-kind
//! -> collection -> item -> child (entities -> steel threads -> ST0068 -> its
//! work packages)* -- so hv's requirement asked for an entry point onto a model
//! the register had already specified, not for a new model.
//!
//! # Nothing here is hand-written, which is the criterion
//!
//! `AC-17.7`: *nav is DERIVED from the model, not hand-built ... a hand-written
//! navigation tree is the same second home as a hand-written field list and
//! goes stale the same way.* So:
//!
//! - **The entity kinds come from the form declaration** ([`kinds`]), which
//!   `AC-17.2` already holds against the schema faces. Declare a form, get a
//!   row at the root.
//! - **The descents come from the schema** ([`descents`]). `surface/forms.json`
//!   glosses `button` as *a DESCENT or an ACTION*, so the widget alone cannot
//!   tell `wps` from `fiat`. The schema can: a descent's property is
//!   `"type": "array"` with an `items.$ref`, and that `$ref` names the child
//!   kind. `fiat` is an object with neither. **The discriminator is the model's
//!   own shape rather than a list somebody maintains**, and
//!   `the_discriminator_separates_a_real_descent_from_a_real_action` drives it
//!   on exactly that pair.
//!
//! # Every segment is a name the declaration already carries
//!
//! A view's path is `/thread/ST0056/wps`, not `/threads/ST0056/work-packages`.
//! **No segment is invented, pluralised or prettified**, because a spelling
//! rule is a second home for naming and it breaks on the first kind that does
//! not take `-s`. `thread` is the entity kind as declared and `wps` is the
//! field as declared.
//!
//! **THAT PATH IS THE WEB SURFACE'S PATH TOO.** vc minted the property: the
//! TUI's view stack and the browser's URL are the same sequence from the same
//! declarations, so neither renderer invents a level and `--browser` has an
//! exact meaning at any depth. [`View::path`] and [`View::parse`] are held to a
//! round trip over every view the real declaration can produce, which is what
//! makes the two surfaces checkably the same rather than similar.

use intentsvcs::form::Loaded;

/// One level of the ladder `AC-17.7` names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
  /// The root `intent explore` opens at: every declared entity kind.
  Entities,
  /// Every item of one kind.
  Collection { kind: String },
  /// One item -- the form. Where `intent edit <kind> <id>` starts.
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

/// The view stack. `⏎` pushes, `Backspace`/`ESC` pops, and **popping the root
/// is what quits** -- `tui-design.md` §3: *ESC always walks toward NORMAL, and
/// at the root it QUITS*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack {
  views: Vec<View>,
}

impl Stack {
  /// A stack rooted at `bottom`. **The root is the argument, and that is the
  /// entire difference between `explore` and `edit`.**
  pub fn rooted_at(bottom: View) -> Self {
    Self {
      views: vec![bottom],
    }
  }

  /// `intent explore`.
  pub fn explore() -> Self {
    Self::rooted_at(View::Entities)
  }

  /// `intent edit <kind> <id>`.
  pub fn at_item(kind: impl Into<String>, id: impl Into<String>) -> Self {
    Self::rooted_at(View::Item {
      kind: kind.into(),
      id: id.into(),
    })
  }

  pub fn current(&self) -> &View {
    // **THE STACK IS NEVER EMPTY BY CONSTRUCTION**: it is built with a bottom
    // and `pop` refuses to remove it. That invariant is what lets this return a
    // reference rather than an `Option` every caller would have to unwrap.
    self.views.last().expect("a Stack always carries its root")
  }

  pub fn depth(&self) -> usize {
    self.views.len()
  }

  pub fn at_root(&self) -> bool {
    self.views.len() == 1
  }

  pub fn push(&mut self, view: View) {
    self.views.push(view);
  }

  /// Pop one level. **`false` means the root was reached, which the realiser
  /// reads as QUIT** -- it does not mean the pop failed.
  pub fn pop(&mut self) -> bool {
    if self.at_root() {
      return false;
    }
    self.views.pop();
    true
  }

  /// The trail for the APP row. *A way back that is wired and unlabelled is a
  /// way back nobody finds* -- a real strawman defect, so the trail is part of
  /// the model rather than something the realiser remembers to draw.
  pub fn trail(&self) -> String {
    self
      .views
      .iter()
      .map(|v| v.path())
      .collect::<Vec<_>>()
      .join("  <  ")
  }
}

/// A descent declared by one entity kind: the field that opens it and the kind
/// it opens onto.
#[derive(Debug, Clone, PartialEq, Eq)]
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
/// `intentsvcs::form` says the same thing about the same pointer. Naming the
/// bare file here would look at `thread`'s properties while claiming to look at
/// a work package's -- and it would go UNNOTICED, because today the wp form's
/// only `button` row (`fiat`) misses at both levels, so the right answer comes
/// out of the wrong lookup. The first descent added to the wp form would read
/// the wrong schema silently.
fn face_pointer(kind: &str) -> Option<&'static str> {
  match kind {
    "thread" => Some("thread.schema.json"),
    "wp" => Some("thread.schema.json#/$defs/WorkPackage"),
    "issue" => Some("issue.schema.json"),
    _ => None,
  }
}

/// The parsed schema face for `kind`, resolved through its fragment.
fn face_json(kind: &str) -> Option<serde_json::Value> {
  let pointer = face_pointer(kind)?;
  let (file, fragment) = match pointer.split_once('#') {
    Some((f, frag)) => (f, Some(frag)),
    None => (pointer, None),
  };
  let root: serde_json::Value = serde_json::from_str(&intentsvcs::faces::face(file)?).ok()?;
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

#[cfg(test)]
mod tests {
  use super::*;

  fn loaded() -> Loaded {
    Loaded::load().expect("the shipped form declaration must load")
  }

  /// Every view the real declaration can produce, so the round trip below is
  /// held over the corpus rather than over three hand-picked examples.
  fn every_view(l: &Loaded) -> Vec<View> {
    let mut out = vec![View::Entities];
    for kind in kinds(l) {
      out.push(View::Collection { kind: kind.clone() });
      out.push(View::Item {
        kind: kind.clone(),
        id: "ST0056".into(),
      });
      for d in descents(l, &kind) {
        out.push(View::Children {
          kind: kind.clone(),
          id: "ST0056".into(),
          field: d.field,
        });
      }
    }
    out
  }

  #[test]
  fn the_declaration_is_not_empty_and_neither_is_the_root() {
    let l = loaded();
    assert!(
      !kinds(&l).is_empty(),
      "no entity kinds, so every walk below is over nothing"
    );
    assert!(
      every_view(&l).len() > kinds(&l).len(),
      "no view beyond the root and its collections, so the round trip asserts almost nothing"
    );
  }

  /// **THE PROPERTY vc MINTED**: the TUI's stack and the browser's URL are the
  /// same sequence, so neither renderer can invent a level. Held as a round
  /// trip over every constructible view.
  #[test]
  fn every_view_round_trips_through_its_path() {
    let l = loaded();
    let views = every_view(&l);
    assert!(!views.is_empty());
    for v in &views {
      let p = v.path();
      assert_eq!(
        View::parse(&p).as_ref(),
        Some(v),
        "{v:?} rendered to {p:?} and did not parse back to itself"
      );
    }
  }

  /// A path that names nothing is refused AS A SPELLING rather than resolved to
  /// something near it.
  #[test]
  fn a_path_that_names_nothing_is_refused_rather_than_guessed() {
    for bad in ["", "thread", "/thread//ST0056", "/a/b/c/d", "//"] {
      assert_eq!(
        View::parse(bad),
        None,
        "{bad:?} parsed to a view and should not have"
      );
    }
  }

  /// The no-trap property `AC-17.7` states, which is
  /// `no_state_can_be_entered_and_not_left` applied to navigation: *no level
  /// can be entered that cannot be left*.
  #[test]
  fn no_level_can_be_entered_and_not_left() {
    let l = loaded();
    for v in every_view(&l) {
      let mut s = Stack::explore();
      let before = s.clone();
      s.push(v.clone());
      assert_eq!(s.depth(), 2, "pushing {v:?} did not deepen the stack");
      assert!(s.pop(), "pushed {v:?} and could not pop back out of it");
      assert_eq!(
        s, before,
        "popping {v:?} did not restore the stack it was pushed onto"
      );
    }
  }

  /// Deep nesting leaves the same way it arrived, one level at a time, from
  /// every depth -- not only from one.
  #[test]
  fn a_stack_of_any_depth_unwinds_to_its_root_and_stops_there() {
    let l = loaded();
    let views = every_view(&l);
    for depth in 1..=views.len().min(8) {
      let mut s = Stack::explore();
      for v in views.iter().take(depth) {
        s.push(v.clone());
      }
      assert_eq!(s.depth(), depth + 1);
      for _ in 0..depth {
        assert!(s.pop(), "unwinding stopped early at depth {}", s.depth());
      }
      assert!(
        s.at_root(),
        "unwound to depth {} rather than to the root",
        s.depth()
      );
      assert!(
        !s.pop(),
        "popping the root must report QUIT rather than emptying the stack"
      );
      assert!(s.at_root(), "a refused pop must leave the root in place");
    }
  }

  /// `explore` and `edit` differ in ONE thing, and this is it.
  #[test]
  fn explore_and_edit_are_the_same_stack_with_different_bottoms() {
    let e = Stack::explore();
    let i = Stack::at_item("thread", "ST0056");
    assert_eq!(e.current(), &View::Entities);
    assert_eq!(
      i.current(),
      &View::Item {
        kind: "thread".into(),
        id: "ST0056".into()
      }
    );
    assert!(
      e.at_root() && i.at_root(),
      "both are rooted; neither is nested inside the other"
    );
    assert_eq!(
      e.depth(),
      i.depth(),
      "the difference is the root, never the depth"
    );
  }

  /// **THE DISCRIMINATOR, DRIVEN ON THE PAIR THAT MOTIVATES IT.** `wps` and
  /// `fiat` are both `button` rows on the thread form; one is a collection and
  /// one is a verb. A check that only asserted `wps` is present would pass for
  /// a rule that admitted everything.
  #[test]
  fn the_discriminator_separates_a_real_descent_from_a_real_action() {
    let l = loaded();
    let form = l.form("thread").expect("the thread form must be declared");
    let buttons: Vec<&str> = form
      .fields
      .iter()
      .filter(|f| f.widget == "button")
      .map(|f| f.name.as_str())
      .collect();
    assert!(
      buttons.contains(&"wps"),
      "the fixture assumes `wps` is a button row"
    );
    assert!(
      buttons.contains(&"fiat"),
      "the fixture assumes `fiat` is a button row -- without it this test cannot show the \
       discriminator refusing anything"
    );

    let declared = descents(&l, "thread");
    let found: Vec<&str> = declared.iter().map(|d| d.field.as_str()).collect();
    assert!(
      found.contains(&"wps"),
      "`wps` is an array of a $ref and must be a descent"
    );
    assert!(
      !found.contains(&"fiat"),
      "`fiat` is an action, not a collection; descending into it would descend into a verb"
    );
  }

  /// **THE FRAGMENT IS RESOLVED, AND THIS IS THE ONLY THING THAT SAYS SO.**
  /// `wp` resolves to `thread.schema.json#/$defs/WorkPackage`. Ignoring the
  /// fragment yields the THREAD schema, and `descents(l, "wp")` still comes
  /// back empty -- the right answer from the wrong lookup -- because the wp
  /// form's only button row misses at both levels. Asserted on the properties
  /// themselves so the resolution is checked rather than its consequence.
  #[test]
  fn a_face_that_is_a_definition_resolves_through_its_fragment() {
    let wp = face_json("wp").expect("wp must resolve to a face");
    let props = wp
      .get("properties")
      .expect("the WorkPackage definition must carry properties");
    assert!(
      props.get("seq").is_some(),
      "the resolved face is missing WorkPackage's own `seq`"
    );
    assert!(
      props.get("wps").is_none(),
      "the resolved face carries `wps`, which belongs to THREAD -- the fragment was ignored and \
       every wp descent would be read off the wrong schema"
    );
    let thread = face_json("thread").expect("thread must resolve to a face");
    assert!(
      thread
        .get("properties")
        .and_then(|p| p.get("wps"))
        .is_some(),
      "the thread face is missing `wps`, so the contrast above proves nothing"
    );
  }

  /// A descent must name the kind it opens onto, or the level below it cannot
  /// be built.
  #[test]
  fn every_descent_names_the_kind_it_opens_onto() {
    let l = loaded();
    let mut total = 0usize;
    for kind in kinds(&l) {
      for d in descents(&l, &kind) {
        assert!(
          !d.child.is_empty(),
          "{kind}.{} declares no child kind",
          d.field
        );
        assert!(!d.label.is_empty(), "{kind}.{} declares no label", d.field);
        total += 1;
      }
    }
    assert!(
      total > 0,
      "no descent was found anywhere, so this test asserted nothing"
    );
  }
}
