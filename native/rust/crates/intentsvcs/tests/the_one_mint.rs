//! ST0095 WP-06: the one mint. A new id is derived from its record's creation
//! event, which is minted first, so the record's `origin` and that event's id
//! are one value. Two records that have met never show one short form, the
//! mint's draws are bounded, and a project `intent upgrade` has not moved
//! refuses every mint, in the wording hv took (hv decisions 70 and 71).

use crate::common::{FIXTURE_ORIGIN, Fixture, sample_thread};
use intentsvcs::facade::FacadeError;
use intentsvcs::id::{
  ISSUE_MARKER, IssueId, MINT_DRAWS, MintError, Mintable, SHOWN_HEX, THREAD_PREFIX, ThreadId,
  WP_SHOWN_HEX, WpId, mint,
};
use intentsvcs::model::TShirt;
use intentsvcs::remedy::Remedy;

/// A well-formed creation event's id that is not [`FIXTURE_ORIGIN`].
const ANOTHER_ORIGIN: &str = "01M7VXR0UNDTR1PF1XTVRE0R1H";

/// Hex digits that keep `hex`'s first `shown` and replace every later one with
/// its complement, so the id they spell shows the same short form and is not
/// the same id.
fn beside(hex: &str, shown: usize) -> String {
  let (front, rest) = hex.split_at(shown);
  let flipped: String = rest
    .chars()
    .map(|c| {
      let digit = c.to_digit(16).expect("a hashed id is hex");
      std::char::from_digit(15 - digit, 16).expect("a hex digit's complement is one")
    })
    .collect();
  format!("{front}{flipped}")
}

fn thread_beside(id: &ThreadId) -> ThreadId {
  let stored = id.stored();
  let hex = stored.strip_prefix(THREAD_PREFIX).expect("a thread id");
  ThreadId::parse(&format!("{THREAD_PREFIX}{}", beside(hex, SHOWN_HEX))).expect("a thread id")
}

fn wp_beside(id: &WpId) -> WpId {
  WpId::parse(&beside(&id.stored(), WP_SHOWN_HEX)).expect("a work package id")
}

fn issue_beside(id: &IssueId) -> IssueId {
  let stored = id.stored();
  let hex = stored
    .strip_prefix(ISSUE_MARKER)
    .expect("a hashed issue id");
  IssueId::parse(&format!("{ISSUE_MARKER}{}", beside(hex, SHOWN_HEX))).expect("an issue id")
}

/// The draws a scripted mint is handed, in order.
fn scripted<'a>(origins: &'a [&'a str]) -> impl FnMut() -> String + 'a {
  let mut next = origins.iter();
  move || {
    next
      .next()
      .expect("the mint drew more than the script holds")
      .to_string()
  }
}

#[test]
fn the_id_is_derived_from_the_creation_event_it_draws() {
  let (origin, id) = mint::<ThreadId>(&[], scripted(&[FIXTURE_ORIGIN])).expect("a mint");
  assert_eq!(origin, FIXTURE_ORIGIN, "the origin is the drawn event's id");
  assert!(
    id.derives_from(&origin),
    "the id {} is not the one its origin mints",
    id.stored()
  );
}

/// The re-draw, for each kind, against a visible record built to show the
/// first draw's short form: the second draw is taken, and its short form is
/// apart from every visible one.
#[test]
fn a_short_form_a_visible_record_shows_is_drawn_again() {
  let first = ThreadId::from_origin(FIXTURE_ORIGIN);
  let visible = [thread_beside(&first)];
  assert!(
    visible[0].short() == first.short() && visible[0] != first,
    "the fixture must share the first draw's short form without being its id"
  );
  let (origin, id) =
    mint(&visible, scripted(&[FIXTURE_ORIGIN, ANOTHER_ORIGIN])).expect("the second draw");
  assert_eq!(
    origin, ANOTHER_ORIGIN,
    "the shared short form was not drawn again"
  );
  assert_ne!(id.short(), visible[0].short());

  let first = WpId::from_origin(FIXTURE_ORIGIN);
  let visible = [wp_beside(&first)];
  assert!(visible[0].short() == first.short() && visible[0] != first);
  let (origin, _) =
    mint(&visible, scripted(&[FIXTURE_ORIGIN, ANOTHER_ORIGIN])).expect("the second draw");
  assert_eq!(
    origin, ANOTHER_ORIGIN,
    "a work package's short form was not kept apart"
  );

  let first = IssueId::from_origin(FIXTURE_ORIGIN);
  let visible = [issue_beside(&first)];
  assert!(visible[0].short() == first.short() && visible[0] != first);
  let (origin, _) =
    mint(&visible, scripted(&[FIXTURE_ORIGIN, ANOTHER_ORIGIN])).expect("the second draw");
  assert_eq!(
    origin, ANOTHER_ORIGIN,
    "an issue's short form was not kept apart"
  );
}

#[test]
fn an_id_a_visible_record_holds_is_refused() {
  let visible = [ThreadId::from_origin(FIXTURE_ORIGIN)];
  let refused = mint(&visible, scripted(&[FIXTURE_ORIGIN])).expect_err("the id is held");
  assert_eq!(
    refused,
    MintError::Taken {
      what: ThreadId::WHAT,
      id: visible[0].short(),
    }
  );
}

#[test]
fn the_mint_refuses_after_its_bounded_draws() {
  let visible = [thread_beside(&ThreadId::from_origin(FIXTURE_ORIGIN))];
  let mut drawn = 0;
  let refused = mint(&visible, || {
    drawn += 1;
    FIXTURE_ORIGIN.to_string()
  })
  .expect_err("every draw shows a visible short form");
  assert_eq!(
    refused,
    MintError::ShortFormsShared {
      what: ThreadId::WHAT,
      draws: MINT_DRAWS,
    }
  );
  assert_eq!(
    drawn, MINT_DRAWS,
    "the mint drew a number of times other than its bound"
  );
}

/// Through the facade, for each creation verb: the record carries an origin,
/// its id derives from it, and the creation event the log holds has that id
/// and names the record.
#[test]
fn a_new_record_carries_its_creation_events_id() {
  let fx = Fixture::new();
  let mut f = fx.facade();

  let st = f.st_new("a thread the mint makes").expect("st new");
  let thread = f.st_show(&st).expect("there").clone();
  let origin = thread
    .origin
    .clone()
    .expect("a minted thread carries its origin");
  assert!(thread.id.derives_from(&origin));

  let seq = f
    .wp_new(&st, "a package the mint makes", TShirt::S)
    .expect("wp new");
  let package = f
    .st_show(&st)
    .expect("there")
    .wps
    .iter()
    .find(|w| w.seq == seq)
    .cloned()
    .expect("the new package");
  let package_origin = package
    .origin
    .clone()
    .expect("a minted package carries its origin");
  assert!(seq.derives_from(&package_origin));

  let number = f
    .issue_add("an issue the mint makes", None, None, "")
    .expect("issues add");
  let issue_origin = f
    .issue_show(number)
    .expect("there")
    .origin
    .clone()
    .expect("a minted issue carries its origin");
  assert!(number.derives_from(&issue_origin));

  let events = f.store().events().expect("the log");
  for (origin, op, subject) in [
    (&origin, "st.new", thread.id.stored()),
    (
      &package_origin,
      "wp.new",
      intentsvcs::id::wp_address(&st, seq),
    ),
    (&issue_origin, "issues.add", number.stored()),
  ] {
    let created = events
      .iter()
      .find(|e| &e.id == origin)
      .unwrap_or_else(|| panic!("no event has the {op} record's origin {origin} as its id"));
    assert_eq!(
      created.op, op,
      "the origin names an event that is not the creation"
    );
    assert_eq!(
      created.subject.id, subject,
      "the creation event names another record"
    );
  }
}

/// hv decisions 70 and 71: every mint in a project in the ordinal format
/// refuses, in ic's wording, with `intent upgrade` as its one remedy, and
/// writes nothing.
#[test]
fn every_mint_refuses_before_the_upgrade_and_writes_nothing() {
  let fx = Fixture::ordinal();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = fx.facade();
  let events = f.store().events().expect("the log").len();
  let packages = f.st_show("ST0001").expect("there").wps.len();

  let refusals = [
    ("st new", f.st_new("a thread").expect_err("st new refuses")),
    (
      "wp new",
      f.wp_new("ST0001", "a package", TShirt::S)
        .expect_err("wp new refuses"),
    ),
    (
      "issues add",
      f.issue_add("an issue", None, None, "")
        .expect_err("issues add refuses"),
    ),
  ];
  for (verb, refused) in &refusals {
    assert!(
      matches!(refused, FacadeError::NotYetUpgraded { verb: named } if named.as_str() == *verb),
      "`{verb}` was refused for another reason: {refused}"
    );
    assert_eq!(
      refused.to_string(),
      format!(
        "`{verb}` refused, and nothing was written: this project's canon is in the ordinal \
         format, which an older Intent still opens without reading a git-style id"
      )
    );
    assert_eq!(
      refused.remedy(),
      "run `intent upgrade` once and commit what it prints. It changes no id, and an older \
       Intent then refuses the project by name instead of misreading it, so upgrade Intent \
       first on every machine that opens it"
    );
  }
  assert_eq!(
    f.store().events().expect("the log").len(),
    events,
    "a refused mint wrote an event"
  );
  assert_eq!(
    f.st_show("ST0001").expect("there").wps.len(),
    packages,
    "a refused mint wrote a package"
  );
}

/// **THE RE-MINT OF ONE ORDINAL** (ST0095 WP-06): a thread and an issue the
/// project holds as ordinals move to git-style ids derived from an origin each
/// record then carries, and that origin names an event in the log. The door is
/// ic's `st remint` and `issues remint`; this is the facade beneath it.
#[test]
fn an_ordinal_is_re_minted_to_an_id_its_origin_derives() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_issue(&crate::common::sample_issue(7));
  let mut f = fx.facade();

  let thread = f
    .st_remint(&ThreadId::parse("ST0001").expect("an ordinal thread"))
    .expect("an ordinal thread is re-minted");
  let issue = f
    .issue_remint(IssueId::from_ordinal(7).expect("an ordinal issue"))
    .expect("an ordinal issue is re-minted");

  let events = f.store().events().expect("the log");
  let moved = f
    .st_show(&thread.to)
    .expect("the thread at its new id")
    .clone();
  let origin = moved
    .origin
    .clone()
    .expect("a re-minted thread carries its origin");
  assert!(
    moved.id.derives_from(&origin),
    "the new id is not the one its origin mints"
  );
  assert!(
    events.iter().any(|e| e.id == origin),
    "the origin names no event in the log"
  );

  let number = IssueId::parse(&issue.to).expect("the issue's new id");
  let origin = f
    .issue_show(number)
    .expect("the issue at its new id")
    .origin
    .clone()
    .expect("a re-minted issue carries its origin");
  assert!(
    number.derives_from(&origin),
    "the new id is not the one its origin mints"
  );
  assert!(
    events.iter().any(|e| e.id == origin),
    "the origin names no event in the log"
  );
}

/// A re-mint refuses before the upgrade, as every mint does (hv decision 70).
#[test]
fn a_re_mint_before_the_upgrade_refuses() {
  let fx = Fixture::ordinal();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = fx.facade();
  let refused = f
    .st_remint(&ThreadId::parse("ST0001").expect("an ordinal thread"))
    .expect_err("a re-mint is a mint");
  assert!(
    matches!(&refused, FacadeError::NotYetUpgraded { verb } if verb == "st remint"),
    "{refused}"
  );
}
