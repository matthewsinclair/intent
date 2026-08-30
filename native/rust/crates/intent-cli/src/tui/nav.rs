//! The TUI view stack: `AT-17.7`, covering `AC-17.7` with [`intentsvcs::nav`], where the VIEWS themselves live.
//!
//! **THE SPLIT IS CONTRACT VERSUS STATE** (vc, 2026-08-30). [`View`], its path
//! round trip, and the derivation of entity kinds and descents are the SHARED
//! contract `AC-17.12` names -- the TUI's sequence and the browser's URL are the
//! same sequence from the same declarations -- so they live in the crate both
//! faces depend on. **A stack is not that.** `push`/`pop`/`depth`/`trail` are
//! how ONE face remembers where it has been, and the web's equivalent is
//! browser history, which it already has and must not be given a second copy
//! of. A breadcrumb is a rendering, not a shared fact.
//!
//! **`explore` IS NOT A SECOND SUBSYSTEM. IT IS THIS STACK WITH A DIFFERENT
//! BOTTOM.** `intent edit <kind> <id>` enters at [`View::Item`];
//! `intent explore` enters at [`View::Entities`]. Same views, same keys, same
//! menu. `AC-17.7` already described the ladder in its own words -- *entity-kind
//! -> collection -> item -> child (entities -> steel threads -> ST0068 -> its
//! work packages)* -- so hv's requirement asked for an entry point onto a model
//! the register had already specified, not for a new model. **The DEPTHS those
//! two verbs name are shared; the stack that remembers them is not**, which is
//! why the constructors below are one line each over shared `View` values.

pub use intentsvcs::nav::View;

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

#[cfg(test)]
mod tests {
  use super::*;
  use intentsvcs::form::Loaded;
  use intentsvcs::nav::{descents, kinds};

  /// Every view the real declaration can produce, so the walks below are over
  /// the corpus rather than over three hand-picked examples.
  fn every_view() -> Vec<View> {
    let l = Loaded::load().expect("the shipped form declaration must load");
    let mut out = vec![View::Entities];
    for kind in kinds(&l) {
      out.push(View::Collection { kind: kind.clone() });
      out.push(View::Item {
        kind: kind.clone(),
        id: "ST0056".into(),
      });
      for d in descents(&l, &kind) {
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
  fn the_corpus_is_more_than_the_root() {
    assert!(
      every_view().len() > 1,
      "one view makes every walk below trivial"
    );
  }

  /// The no-trap property `AC-17.7` states, which is
  /// `no_state_can_be_entered_and_not_left` applied to navigation: *no level
  /// can be entered that cannot be left*.
  #[test]
  fn no_level_can_be_entered_and_not_left() {
    for v in every_view() {
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
    let views = every_view();
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

  /// The trail is a rendering of the stack, so it must name every level and
  /// grow with it -- otherwise a nested view offers a way back to somewhere it
  /// cannot say.
  #[test]
  fn the_trail_names_every_level_the_stack_holds() {
    let mut s = Stack::explore();
    assert_eq!(s.trail(), "/");
    s.push(View::Collection {
      kind: "thread".into(),
    });
    s.push(View::Item {
      kind: "thread".into(),
      id: "ST0056".into(),
    });
    let trail = s.trail();
    for expected in ["/", "/thread", "/thread/ST0056"] {
      assert!(
        trail.contains(expected),
        "the trail {trail:?} does not name {expected}"
      );
    }
  }
}
