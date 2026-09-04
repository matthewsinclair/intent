//! `AC-17.6`'s foundation: **a DECLARED form resolves to a real entity, or the
//! declaration and the resolver disagree in silence.**
//!
//! # The failure this exists to catch does not look like a failure
//!
//! `form::triples` answers a field the entity does not carry with an EMPTY
//! VALUE rather than a missing row, deliberately -- *an empty value is
//! visible; a missing row is not*, and tab order is declaration order, so a
//! skipped row moves every row after it. That is right for one absent field
//! and it is exactly what makes a MISSING ENTITY invisible: hand `triples` a
//! `Value::Null` and every declared row comes back present, correctly ordered,
//! correctly labelled, correctly widgeted, and blank. **A form with no entity
//! behind it renders as an entity with no data in it, and nothing anywhere
//! distinguishes the two.**
//!
//! `nav.rs` wrote the prediction down before this test existed: *`wp` is a
//! declared kind whose item view nothing reaches ... landing there would paint
//! a form whose every value is blank -- for every work package, not just a
//! missing one.* It was true, it was latent because no navigation push
//! produced that view, and `AC-17.6` requires both verbs to cover ST, WP and
//! ISSUE -- which is precisely what makes it reachable.
//!
//! # The population is the DECLARATION, never a list written here
//!
//! `surface/forms.json` declares the forms; a roster of kinds in this file
//! would be a second home for that set and would go stale the day a fourth is
//! declared -- silently, because a test that checks three of four kinds passes.
//! So the loop is over `Loaded::forms()` and an unmapped kind FAILS rather
//! than being skipped. **A denominator that can shrink without saying so is the
//! defect this thread keeps paying for.**

use crate::common::{Fixture, sample_issue, sample_thread};
use intentsvcs::address::Entity;
use intentsvcs::form::{Loaded, triples};

/// The thread the fixture writes, and the work package inside it.
const THREAD: &str = "ST0001";
const WP_SEQ: &str = "2";
const ISSUE: u32 = 7;

/// How each DECLARED form is addressed at the fixture's own instances.
///
/// **`None` IS A TEST FAILURE, NOT A SKIP.** It means the declaration grew a
/// form this test cannot address, and the whole point is that such a form
/// cannot pass unnoticed.
fn addressed(form_entity: &str) -> Option<Entity> {
  match form_entity {
    "thread" => Some(Entity::Thread {
      id: THREAD.to_string(),
    }),
    "wp" => Some(Entity::Wp {
      thread: THREAD.to_string(),
      wp: WP_SEQ.to_string(),
    }),
    "issue" => Some(Entity::Issue {
      id: ISSUE.to_string(),
    }),
    _ => None,
  }
}

fn fixture() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread(THREAD));
  fx.write_issue(&sample_issue(ISSUE));
  fx
}

/// **THE CORE ASSERTION, AND IT IS ABOUT THE PAIRING RATHER THAN ABOUT EITHER
/// SIDE.** A declared form and a resolvable entity are two independent facts,
/// and the bug is one being true while the other is false.
#[test]
fn every_declared_form_has_an_entity_behind_it() {
  let fx = fixture();
  let facade = fx.facade();
  let declaration = Loaded::load().expect("the shipped form declaration must load");

  assert!(
    !declaration.forms().is_empty(),
    "vacuity guard: a declaration with no forms would pass every assertion below \
     by having nothing to check"
  );

  for form in declaration.forms() {
    let entity = addressed(&form.entity).unwrap_or_else(|| {
      panic!(
        "`{}` is a DECLARED form this test cannot address. Add it to `addressed` \
         -- it is not skippable, because an unaddressed form is exactly the case \
         that renders blank and reads as data",
        form.entity
      )
    });

    let resolved = facade.entity_json(&entity).unwrap_or_else(|e| {
      panic!(
        "`{}` is declared as a form and its entity does not resolve: {e}. \
         This is the pairing failure -- the renderer would paint every declared \
         row, correctly ordered and entirely blank",
        form.entity
      )
    });

    // **NOT `is_object()`.** A shape assertion passes for `{}`, which is the
    // failing case wearing the passing case's type.
    let rows = triples(form, &resolved);
    assert_eq!(
      rows.len(),
      form.fields.len(),
      "`{}` must render one row per declared field",
      form.entity
    );
    assert!(
      rows.iter().any(|r| !r.value.is_empty()),
      "`{}` resolved to an entity carrying no values at all -- which is what a \
       MISSING entity looks like, and is the one thing this test exists to tell \
       apart from a present one. Rows: {:?}",
      form.entity,
      rows.iter().map(|r| &r.name).collect::<Vec<_>>()
    );
  }
}

/// **THE POSITIVE CONTROL FOR THE ASSERTION ABOVE, AND WITHOUT IT THAT TEST
/// CANNOT DISTINGUISH A WORKING RESOLVER FROM A BROKEN ONE.**
///
/// It drives `triples` against the `Value::Null` a missing entity used to
/// produce and asserts the check goes RED: every row present, every value
/// blank. If this ever passes the *some value is non-empty* bar, the bar is
/// measuring nothing.
#[test]
fn a_missing_entity_renders_every_declared_row_blank() {
  let declaration = Loaded::load().expect("the shipped form declaration must load");
  let form = declaration.form("wp").expect("`wp` is a declared form");

  let rows = triples(form, &serde_json::Value::Null);

  assert_eq!(
    rows.len(),
    form.fields.len(),
    "the point of the control: a missing entity is not a short form, it is a \
     COMPLETE one with nothing in it"
  );
  assert!(
    rows.iter().all(|r| r.value.is_empty()),
    "the control must exhibit the failure it controls for"
  );
}

/// **`wp` HAS ITS OWN TEST BECAUSE IT IS THE ARM THAT WAS MISSING**, and a
/// regression here would be invisible in the loop above the day someone adds a
/// fourth form and reaches for the same `_ => None` that caused this.
#[test]
fn a_work_package_resolves_to_its_own_fields_and_not_to_its_threads() {
  let fx = fixture();
  let facade = fx.facade();

  let wp = facade
    .entity_json(&Entity::Wp {
      thread: THREAD.to_string(),
      wp: WP_SEQ.to_string(),
    })
    .expect("a declared `wp` form needs a resolvable work package");
  let thread = facade
    .entity_json(&Entity::Thread {
      id: THREAD.to_string(),
    })
    .expect("the fixture's thread resolves");

  // **THE DISCRIMINATING ASSERTION.** `Entity::artefact()` maps a work package
  // onto its THREAD's files, deliberately -- a wp has no files of its own -- so
  // the plausible wrong fix is to resolve the thread and call it the work
  // package. The browser reaches the MODEL, which is finer than the filesystem,
  // and these two must not be the same object.
  assert_ne!(
    wp, thread,
    "a work package must resolve to itself, not to the thread whose files carry it"
  );
  assert_eq!(
    wp.get("seq").and_then(|v| v.as_u64()),
    Some(
      WP_SEQ
        .parse::<u64>()
        .expect("the fixture's seq is a number")
    ),
    "the resolved entity must be the work package that was addressed"
  );
}

/// **AN ADDRESS THE DECLARATION HAS NO FORM FOR IS REFUSED BY NAME.**
///
/// The address grammar answers thirteen form names and `surface/forms.json`
/// declares three, so this path is reachable by anyone typing a legal address.
/// Answering it with an empty field list would make *no form for this kind*
/// indistinguishable from *a form with nothing in it* at the one layer that can
/// still tell them apart.
#[test]
fn an_entity_with_no_declared_form_refuses_rather_than_resolving_empty() {
  let fx = fixture();
  let facade = fx.facade();

  let refused = facade.entity_json(&Entity::Ac {
    thread: THREAD.to_string(),
    ac: "AC-01.1".to_string(),
  });

  let err = refused.expect_err("a criterion is not an entity with a declared form");
  let said = err.to_string();
  assert!(
    said.contains("ac"),
    "the refusal must name the form it refused, and said: {said}"
  );
}
