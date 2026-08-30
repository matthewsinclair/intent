//! Focus order over a declared form: `AC-17.5`.
//!
//! **THIS MODULE CARRIES NO ORDERING OF ITS OWN, AND THAT IS THE WHOLE
//! DESIGN.** `form::Form::fields` already says of itself that it IS the tab
//! order and that no second field carries it, so a `Vec` here holding "the tab
//! order" would be exactly the second home the form module refuses one level
//! down. Focus is therefore an INDEX INTO the declaration -- it can drift out
//! of range, which is checkable, but it cannot disagree about what comes after
//! what, because there is nothing here to disagree with.
//!
//! **IT CARRIES NO `ratatui` AND NO `crossterm`**, for the reason [`super::mode`]
//! and [`super::terminal`] do not: the realiser is what these invariants CHECK,
//! so proving them must not require the thing being checked. Tab order is a
//! property of a declaration, not of a terminal, and needs no tty to establish.
//!
//! # What `AC-17.5` actually demands, and why a walk rather than a sample
//!
//! *Every field is reachable going forward and going backward, focus wraps at
//! both ends, and no field can be entered that cannot be left* -- the
//! `no_state_can_be_entered_and_not_left` property the entity machines already
//! carry, applied to focus. The criterion then names its own method and rules
//! out the easy one: **asserted by walking the whole field set in both
//! directions and reconciling against the declaration, never by driving a few
//! tabs and observing that it looked right.** So the tests below walk the REAL
//! declared forms out of `surface/forms.json`, exhaustively, from every
//! starting position -- a sample that happened to miss the one trapped row
//! would be green and wrong.
//!
//! # Every row is reachable, including the ones you cannot type into
//!
//! `editable` is deliberately NOT consulted here. A `button` row -- `fiat` on
//! both the thread and wp forms -- declares `editable: false` because there is
//! no text to edit, and it is precisely a row the operator has to be able to
//! REACH in order to press. Skipping non-editable rows in the tab order would
//! make the form's only action unreachable while every invariant below still
//! passed, because they would be walking the filtered set and agreeing with
//! themselves about it.

/// A position within a form of `n` rows.
///
/// **`n` IS CARRIED SO THE TYPE CANNOT NAME A ROW THAT DOES NOT EXIST.** The
/// alternative -- a bare `usize` plus whatever length the caller had to hand --
/// is the shape where the wrap arithmetic is written at each call site and one
/// of them gets the `- 1` wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Focus {
  at: usize,
  n: usize,
}

impl Focus {
  /// Focus on the first row, or `None` for a form with no rows.
  ///
  /// **AN EMPTY FORM HAS NO FOCUS RATHER THAN A FOCUS ON NOTHING.** Returning
  /// a `Focus { at: 0, n: 0 }` would make `index()` name row zero of a form
  /// that has none, and every wrap below would divide the intent out of a
  /// modulo by zero. The absence is the honest value and the type says so.
  pub fn first(n: usize) -> Option<Self> {
    (n > 0).then_some(Self { at: 0, n })
  }

  /// The row this focus names. Always a valid index into the declaration.
  pub fn index(self) -> usize {
    self.at
  }

  /// How many rows the form has.
  pub fn len(self) -> usize {
    self.n
  }

  /// The next row, wrapping past the end to the first.
  ///
  /// **WRAPPING IS WHAT MAKES THE ORDER TOTAL RATHER THAN MERELY LONG.**
  /// Stopping at the last row means the last row can be entered and not left
  /// in the forward direction, so an operator who only presses Tab is stuck
  /// there -- the trap state, arrived at by omission rather than by design.
  pub fn forward(self) -> Self {
    Self {
      at: (self.at + 1) % self.n,
      n: self.n,
    }
  }

  /// The previous row, wrapping past the start to the last.
  ///
  /// **Written as `+ n - 1` rather than a checked subtraction**, because
  /// `at - 1` underflows at row zero and the panic would be the wrap case --
  /// the one place this function exists to handle.
  pub fn back(self) -> Self {
    Self {
      at: (self.at + self.n - 1) % self.n,
      n: self.n,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use intentsvcs::form::Loaded;
  use std::collections::BTreeSet;

  /// The real declared forms, because **`AC-17.5` is a claim about the
  /// declaration and a synthetic `n` cannot falsify it.** A test over
  /// `Focus::first(5)` would pass identically if `forms.json` were empty.
  fn declared() -> Loaded {
    Loaded::load().expect("surface/forms.json loads")
  }

  /// Every form the file declares, as `(entity, row count)`.
  fn shapes() -> Vec<(String, usize)> {
    declared()
      .forms()
      .iter()
      .map(|f| (f.entity.clone(), f.fields.len()))
      .collect()
  }

  /// **A WALK OVER NO FORMS PASSES EVERY INVARIANT BELOW.** If the loader ever
  /// returns an empty set -- a moved file, a renamed key, a refactor that
  /// silently changes what `forms()` means -- each test would iterate zero
  /// times and report success. This is the positive control for the corpus
  /// itself, and it is the failure the rest of this module cannot see.
  #[test]
  fn the_declaration_is_not_empty_and_neither_are_its_forms() {
    let shapes = shapes();
    assert!(
      !shapes.is_empty(),
      "`forms()` returned no forms, so every walk below iterates zero times and passes for free"
    );
    let empty: Vec<&String> = shapes
      .iter()
      .filter(|(_, n)| *n == 0)
      .map(|(e, _)| e)
      .collect();
    assert!(
      empty.is_empty(),
      "these forms declare no rows, so a walk over them proves nothing: {empty:?}"
    );
  }

  /// **TOTALITY FORWARD.** From every starting row, `n` steps of `forward`
  /// visit every row exactly once and return to where they began.
  ///
  /// Driven from EVERY start rather than from row zero, because a cycle that
  /// is total from the first row is total from all of them only if it really
  /// is a single cycle -- and "starts at zero" is the assumption a broken wrap
  /// would satisfy while leaving a second orbit nobody visits.
  #[test]
  fn walking_forward_from_any_row_reaches_every_row_exactly_once() {
    for (entity, n) in shapes() {
      for start in 0..n {
        let mut f = Focus { at: start, n };
        let mut seen = BTreeSet::new();
        for _ in 0..n {
          assert!(
            seen.insert(f.index()),
            "`{entity}` revisits row {} within one lap forward from {start}, so the order is not a single cycle and some row is unreachable",
            f.index()
          );
          f = f.forward();
        }
        assert_eq!(
          seen.len(),
          n,
          "`{entity}` forward from {start} reached {} of {n} rows -- a row the operator cannot tab to is a row that does not exist to them",
          seen.len()
        );
        assert_eq!(
          f.index(),
          start,
          "`{entity}` forward from {start} did not close its lap, so focus does not wrap at the end"
        );
      }
    }
  }

  /// **TOTALITY BACKWARD**, asserted separately rather than inferred from the
  /// forward walk. *Reversible* in `AC-17.5` is its own clause: a Shift-Tab
  /// implemented as "forward n-1 times" satisfies the forward test and can
  /// still be wrong, and the operator meets the difference immediately.
  #[test]
  fn walking_backward_from_any_row_reaches_every_row_exactly_once() {
    for (entity, n) in shapes() {
      for start in 0..n {
        let mut f = Focus { at: start, n };
        let mut seen = BTreeSet::new();
        for _ in 0..n {
          assert!(
            seen.insert(f.index()),
            "`{entity}` revisits row {} within one lap backward from {start}",
            f.index()
          );
          f = f.back();
        }
        assert_eq!(
          seen.len(),
          n,
          "`{entity}` backward from {start} reached {} of {n} rows",
          seen.len()
        );
        assert_eq!(
          f.index(),
          start,
          "`{entity}` backward from {start} did not close its lap, so focus does not wrap at the start"
        );
      }
    }
  }

  /// **REVERSIBILITY, as the inverse property rather than as a second walk.**
  /// `back` undoes `forward` and `forward` undoes `back`, from every row of
  /// every declared form -- including across both wrap points, which is where
  /// an off-by-one lives.
  #[test]
  fn back_undoes_forward_and_forward_undoes_back_from_every_row() {
    for (entity, n) in shapes() {
      for start in 0..n {
        let f = Focus { at: start, n };
        assert_eq!(
          f.forward().back(),
          f,
          "`{entity}`: Tab then Shift-Tab from row {start} did not return, so the two keys disagree about the order"
        );
        assert_eq!(
          f.back().forward(),
          f,
          "`{entity}`: Shift-Tab then Tab from row {start} did not return"
        );
      }
    }
  }

  /// **THE TRAP PROPERTY, stated in its own words** rather than left implied
  /// by totality: no row can be entered and not left. The mode machine refuses
  /// this for modes; `AC-17.5` applies the same law to focus.
  ///
  /// A single-row form is the interesting case and it PASSES deliberately:
  /// both keys are self-loops, so the row is left in the only sense available,
  /// and the operator is not stuck because there is nowhere else to be.
  #[test]
  fn no_row_can_be_entered_and_not_left() {
    for (entity, n) in shapes() {
      for start in 0..n {
        let f = Focus { at: start, n };
        let moved = f.forward() != f || f.back() != f;
        assert!(
          moved || n == 1,
          "`{entity}` row {start} goes nowhere on either key in a form of {n} rows, which is a trap with a cursor in it"
        );
      }
    }
  }

  /// **THE ORDER IS THE DECLARATION, RECONCILED AGAINST IT** -- the clause
  /// `AC-17.5` names last and the one a focus module is most likely to break
  /// by helpfully sorting, grouping or skipping.
  ///
  /// Asserted over the FIELD NAMES rather than over indices, because index
  /// order agreeing with itself is not evidence: it is the same list compared
  /// to the same list. Walking `n` steps and collecting names is the only form
  /// of this check that could come out wrong.
  ///
  /// **AND IT IS NOT REDUNDANT WITH THE TOTALITY WALKS -- MEASURED, NOT
  /// ASSUMED.** Mutating `forward` to step by TWO leaves both totality tests
  /// GREEN, because all three declared forms have an odd row count today
  /// (thread 15, wp 9, issue 9) and a stride of two through an odd cycle still
  /// visits every row exactly once. Totality would have reported a form whose
  /// tab key skipped every other field as fully reachable. **This test is what
  /// caught that mutation**, which is `AC-17.5` naming *reconciling against the
  /// declaration* as a separate clause rather than a restatement -- and a
  /// reminder that the corpus being odd is a coincidence the checks must not
  /// lean on.
  #[test]
  fn tab_order_is_declaration_order_including_the_rows_you_cannot_type_into() {
    let loaded = declared();
    for form in loaded.forms() {
      let declaration: Vec<&str> = form.fields.iter().map(|f| f.name.as_str()).collect();
      let mut focus = Focus::first(form.fields.len()).expect("checked non-empty above");
      let mut walked: Vec<&str> = Vec::new();
      for _ in 0..form.fields.len() {
        walked.push(form.fields[focus.index()].name.as_str());
        focus = focus.forward();
      }
      assert_eq!(
        walked, declaration,
        "`{}`: tabbing through the form does not visit its rows in declared order",
        form.entity
      );

      // The clause with teeth: a focus model that skipped `editable: false`
      // would still pass the walk above IF the walk were built from the same
      // filtered set. Naming the non-editable rows and requiring them present
      // is what makes the check independent of that mistake.
      let unwritable: Vec<&str> = form
        .fields
        .iter()
        .filter(|f| !f.editable)
        .map(|f| f.name.as_str())
        .collect();
      for name in &unwritable {
        assert!(
          walked.contains(name),
          "`{}` row `{name}` declares `editable: false` and is not in the tab order. A `button` is exactly a row with nothing to type and everything to reach -- skipping it makes the form's action unreachable",
          form.entity
        );
      }
    }
  }

  /// An empty form has no focus at all, rather than a focus naming row zero of
  /// nothing. Checked directly because no declared form is empty, so the real
  /// corpus can never exercise it.
  #[test]
  fn an_empty_form_has_no_focus_rather_than_a_focus_on_nothing() {
    assert_eq!(
      Focus::first(0),
      None,
      "a form with no rows has nowhere to put the cursor"
    );
    assert!(
      Focus::first(1).is_some(),
      "a one-row form does have a focus"
    );
  }
}
