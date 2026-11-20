//! ST0095 WP-07: what a typed spelling names (AC-07.1), and that nothing a
//! store keeps is a prefix (AC-07.2).
//!
//! Every hashed record here is written by the fixture writer with the id its
//! creation event's id mints, and two of those events were found by search to
//! mint ids that share their shown form, so an ambiguity is the real thing and
//! not a constructed one. Every length is read from the waist's constants.

use crate::common::{Fixture, sample_issue, sample_thread};
use intentsvcs::facade::{EventFilter, Facade, FacadeError, IdRecord};
use intentsvcs::id::{
  ISSUE_MARKER, IssueId, MIN_PREFIX_HEX, SHOWN_HEX, THREAD_DIGITS, THREAD_PREFIX, ThreadId,
  WP_SHOWN_HEX, WpId, prefix_floor,
};
use intentsvcs::model::{AcKind, WorkPackage};

/// Two creation events' ids whose SHA-256s share their first [`SHOWN_HEX`] hex
/// digits, found by search and kept: two records minted from them show one
/// short form, which is what two branches that have not met can mint.
const SHARED_A: &str = "01M7W4RVVVQHEPM8G2HC5CTKDH";
const SHARED_B: &str = "01M7WC0P8VR6FFPF2WGDKR2YCZ";
/// A creation event's id whose hash begins apart from both.
const APART: &str = "01M7WRV7ZXZZ47FFMJ6E15EWYN";

/// A hashed thread's hex, behind its marker.
fn hex(id: &ThreadId) -> String {
  id.stored()
    .strip_prefix(THREAD_PREFIX)
    .expect("a thread id")
    .to_string()
}

/// The fewest of `hex`'s leading digits that a door takes as a prefix.
fn shortest(hex: &str) -> &str {
  &hex[..prefix_floor(&hex[..MIN_PREFIX_HEX], THREAD_DIGITS)]
}

struct Estate {
  fx: Fixture,
  f: Facade,
  a: ThreadId,
  b: ThreadId,
  c: ThreadId,
  wp: WpId,
  issue: IssueId,
}

fn estate() -> Estate {
  let fx = Fixture::new();
  let (a, b, c) = (
    ThreadId::from_origin(SHARED_A),
    ThreadId::from_origin(SHARED_B),
    ThreadId::from_origin(APART),
  );
  assert_eq!(
    a.shown(),
    b.shown(),
    "the kept pair must show one short form, or the ambiguity below is not tested"
  );
  assert!(
    !hex(&c).starts_with(&hex(&a)[..MIN_PREFIX_HEX]),
    "the third must begin apart from the pair"
  );
  let wp = WpId::from_origin(SHARED_A);
  for (id, origin, title) in [
    (&a, SHARED_A, "the first of two"),
    (&b, SHARED_B, "the second of two"),
    (&c, APART, "a third, apart"),
  ] {
    let mut thread = sample_thread(&id.stored());
    thread.origin = Some(origin.to_string());
    thread.title = title.to_string();
    thread.related.clear();
    // Unrealised, so the hydrate below by prefix has a record to move.
    thread.realised = false;
    if *id == a {
      let mut package: WorkPackage = thread.wps[0].clone();
      package.seq = wp;
      package.origin = Some(SHARED_A.to_string());
      package.title = "a package of the first".to_string();
      thread.wps.push(package);
    }
    fx.write_thread(&thread);
  }
  let issue = IssueId::from_origin(SHARED_A);
  let mut record = sample_issue(1);
  record.number = issue;
  record.origin = Some(SHARED_A.to_string());
  record.title = "an issue beside the first".to_string();
  fx.write_issue(&record);
  let mut f = fx.facade_on_disk();
  f.wb_register("ic", "Interface", "interface")
    .expect("register ic");
  Estate {
    fx,
    f,
    a,
    b,
    c,
    wp,
    issue,
  }
}

/// The ids an ambiguity names, in the order it names them.
fn candidate_ids(refused: FacadeError) -> Vec<IdRecord> {
  match refused {
    FacadeError::AmbiguousId { candidates, .. } => candidates,
    other => panic!("expected an ambiguity, got {other:?}"),
  }
}

#[test]
fn a_unique_prefix_names_its_record_with_or_without_its_marker_and_in_either_case() {
  let e = estate();
  let c = hex(&e.c);
  for typed in [
    shortest(&c).to_string(),
    format!("{THREAD_PREFIX}{}", shortest(&c)),
    shortest(&c).to_ascii_uppercase(),
    format!("{}{}", THREAD_PREFIX.to_ascii_lowercase(), &c[..SHOWN_HEX]),
    e.c.shown(),
    e.c.stored(),
  ] {
    assert_eq!(
      e.f.resolve_thread(&typed).expect("a unique prefix"),
      e.c,
      "`{typed}` names the third thread"
    );
  }
  // The pair's first differing digit makes a prefix of the first unique.
  let (a, b) = (hex(&e.a), hex(&e.b));
  let apart = a
    .chars()
    .zip(b.chars())
    .position(|(x, y)| x != y)
    .expect("two ids differ");
  assert_eq!(
    e.f
      .resolve_thread(&a[..=apart])
      .expect("unique past the shared front"),
    e.a
  );
  let issue_hex = e
    .issue
    .stored()
    .strip_prefix(ISSUE_MARKER)
    .expect("a hashed issue")
    .to_string();
  for typed in [
    format!("{ISSUE_MARKER}{}", shortest(&issue_hex)),
    format!(
      "{}{}",
      ISSUE_MARKER.to_ascii_lowercase(),
      shortest(&issue_hex)
    ),
    shortest(&issue_hex).to_string(),
  ] {
    assert_eq!(
      e.f.resolve_issue(&typed).expect("an issue's prefix"),
      e.issue
    );
  }
}

#[test]
fn a_shared_short_form_is_refused_with_every_candidate_and_none_is_picked() {
  let e = estate();
  let refused = e
    .f
    .resolve_thread(&e.a.shown())
    .expect_err("two threads show this short form");
  let said = refused.to_string();
  let candidates = candidate_ids(refused);
  let mut ids: Vec<String> = candidates.iter().map(|c| c.id.clone()).collect();
  ids.sort();
  let mut want = vec![e.a.stored(), e.b.stored()];
  want.sort();
  assert_eq!(ids, want, "every candidate, and only the candidates");
  for candidate in &candidates {
    assert_eq!(candidate.kind, "thread");
    assert!(
      candidate.created.is_some(),
      "a thread's created day is named"
    );
    assert!(
      said.contains(&candidate.title),
      "the refusal names each candidate's title: {said}"
    );
  }
}

#[test]
fn bare_hex_at_the_door_that_does_not_know_the_collection_is_asked_of_both() {
  let e = estate();
  let candidates = candidate_ids(
    e.f
      .resolve_named(&e.a.shown()[THREAD_PREFIX.len()..])
      .expect_err("the pair and the issue all begin so"),
  );
  let kinds: std::collections::BTreeSet<&str> = candidates.iter().map(|c| c.kind).collect();
  assert_eq!(
    kinds.into_iter().collect::<Vec<_>>(),
    ["issue", "thread"],
    "one ambiguity across both kinds"
  );
}

#[test]
fn an_ordinal_shaped_spelling_is_the_ordinal_and_never_a_hash() {
  let e = estate();
  let digits: String = hex(&e.a).chars().take_while(char::is_ascii_digit).collect();
  assert!(
    !digits.is_empty(),
    "the kept pair's hash begins with a digit, which this case relies on"
  );
  let ordinal = &digits[..digits.len().min(THREAD_DIGITS)];
  assert_eq!(
    e.f.resolve_thread(ordinal).expect("an ordinal"),
    ThreadId::from_ordinal(ordinal.parse().expect("digits")).expect("fits"),
    "`{ordinal}` is an ordinal, and the hashed thread that begins with it is not reached"
  );
}

#[test]
fn a_spelling_too_short_for_a_prefix_is_refused_naming_the_minimum() {
  let e = estate();
  let typed = &hex(&e.c)[..MIN_PREFIX_HEX - 1];
  match e.f.resolve_thread(typed) {
    Err(FacadeError::NotAnId { refusal, .. }) => {
      assert!(refusal.message.contains("too short"), "{}", refusal.message)
    }
    // Digits alone that short are an ordinal, which names its thread.
    Ok(id) => assert!(!id.is_hashed(), "`{typed}` reached a hash"),
    Err(other) => panic!("`{typed}`: {other:?}"),
  }
}

#[test]
fn a_prefix_that_names_nothing_is_unknown_and_says_what_was_looked_for() {
  let e = estate();
  let nothing = (0..=u32::from(u16::MAX))
    .map(|n| format!("{n:0width$x}", width = MIN_PREFIX_HEX))
    .find(|front| {
      prefix_floor(front, THREAD_DIGITS) == MIN_PREFIX_HEX
        && ![&e.a, &e.b, &e.c]
          .iter()
          .any(|t| hex(t).starts_with(front.as_str()))
    })
    .expect("some four hex digits begin no thread here");
  match e.f.resolve_thread(&nothing) {
    Err(FacadeError::UnknownId { kinds, .. }) => assert_eq!(kinds, ["thread"]),
    other => panic!("`{nothing}`: {other:?}"),
  }
}

#[test]
fn a_package_prefix_resolves_within_its_own_thread_and_not_another() {
  let e = estate();
  let package = &e.wp.stored()[..WP_SHOWN_HEX];
  let typed = format!("{}/{package}", e.a.stored());
  assert_eq!(
    e.f.resolve_wp(&typed).expect("the first thread's package"),
    (e.a.clone(), e.wp)
  );
  match e.f.resolve_wp(&format!("{}/{package}", e.c.stored())) {
    Err(FacadeError::UnknownId { kinds, .. }) => assert_eq!(kinds, ["wp"]),
    other => panic!("another thread's package was reached: {other:?}"),
  }
}

/// AC-07.2: each path that stores an id is given a prefix and keeps the whole.
#[test]
fn every_store_path_given_a_prefix_keeps_the_whole_id() {
  let mut e = estate();
  let a_typed = format!("{THREAD_PREFIX}{}", &hex(&e.a)[..SHOWN_HEX + 1]);
  let package = &e.wp.stored()[..WP_SHOWN_HEX];
  let issue_typed = format!(
    "{}{}",
    intentsvcs::id::CLAIM_ISSUE_PREFIX,
    &e.issue.stored()[..ISSUE_MARKER.len() + SHOWN_HEX]
  );

  // Claims.
  for typed in [a_typed.clone(), format!("{a_typed}/{package}"), issue_typed] {
    assert!(e.f.wb_claim("ic", &typed).expect("a claim by prefix"));
  }
  assert_eq!(
    e.f.store().wb_claims("ic").expect("claims"),
    [
      e.a.stored(),
      intentsvcs::id::wp_address(e.a.as_str(), e.wp),
      e.issue.claim(),
    ]
  );

  // `related`, and the canon that carries it.
  e.f
    .st_relate(&e.c.stored(), &a_typed, None)
    .expect("relate by prefix");
  let related: Vec<String> = e
    .f
    .st_show(&e.c.stored())
    .expect("the third")
    .related
    .iter()
    .map(|r| r.id.stored())
    .collect();
  assert_eq!(related, [e.a.stored()]);
  assert!(
    e.fx.read_canon(&e.c.stored()).contains(&e.a.stored()),
    "the canon carries the whole id"
  );

  // A criterion's group.
  let label = format!("AC-{package}.1");
  e.f
    .ac_new(
      &e.a.stored(),
      &label,
      "a criterion of the package",
      AcKind::NonTest,
    )
    .expect("a criterion grouped by a package prefix");
  let whole = format!("AC-{}.1", e.wp.stored());
  assert!(
    e.f
      .st_show(&e.a.stored())
      .expect("the first")
      .criteria
      .iter()
      .any(|c| c.id == whole),
    "the group is stored whole, as {whole}"
  );

  // Event subjects: a filter by prefix matches the whole subject, here the
  // relate written on the third thread above.
  let c_typed = format!("{THREAD_PREFIX}{}", shortest(&hex(&e.c)));
  let page = e
    .f
    .events(&EventFilter {
      subject: Some(c_typed),
      ..EventFilter::default()
    })
    .expect("events by a prefix");
  assert!(
    !page.rows.is_empty(),
    "the relate's event names the third thread"
  );
  assert!(page.rows.iter().all(|row| row.subject.id == e.c.stored()));

  // Realisation, through an address that carries the prefix: the whole
  // thread's own record moves (ST0096 WP-01).
  let address = intentsvcs::address::promote(&a_typed).expect("an address by prefix");
  e.f.hydrate(&address).expect("hydrate by prefix");
  assert!(
    e.f
      .st_show(&e.a.stored())
      .expect("the first thread")
      .realised,
    "hydrate by prefix did not realise the thread it names"
  );
}
