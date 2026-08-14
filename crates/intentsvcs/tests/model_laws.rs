//! AT-02.4 / AC-02.4: serialise/deserialise laws under proptest, and
//! unknown-field refusal -- an unknown field in canonical JSON is refused by
//! name, never dropped (design.md D05).

use intentsvcs::model::{
  AcKind, AcScope, AcceptanceMode, AcceptanceTest, AtKind, AtStatus, Criterion, Issue, IssueStatus,
  THREAD_SCHEMA, TShirt, Thread, ThreadStatus, WorkPackage, WpStatus, to_canonical_json,
};
use proptest::prelude::*;

fn thread_status() -> impl Strategy<Value = ThreadStatus> {
  prop_oneof![
    Just(ThreadStatus::NotStarted),
    Just(ThreadStatus::Wip),
    Just(ThreadStatus::Tbc),
    Just(ThreadStatus::Hold),
    Just(ThreadStatus::Completed),
    Just(ThreadStatus::Cancelled),
  ]
}

fn tshirt() -> impl Strategy<Value = TShirt> {
  prop_oneof![
    Just(TShirt::XS),
    Just(TShirt::S),
    Just(TShirt::M),
    Just(TShirt::L),
    Just(TShirt::XL),
    Just(TShirt::XXL),
  ]
}

fn wp_status() -> impl Strategy<Value = WpStatus> {
  prop_oneof![
    Just(WpStatus::NotStarted),
    Just(WpStatus::Wip),
    Just(WpStatus::Done)
  ]
}

fn at_status() -> impl Strategy<Value = AtStatus> {
  prop_oneof![
    Just(AtStatus::ToWrite),
    Just(AtStatus::Red),
    Just(AtStatus::Green),
    Just(AtStatus::Na),
  ]
}

fn ac_scope() -> impl Strategy<Value = AcScope> {
  prop_oneof![
    Just(AcScope::InScope),
    (0u32..9999, proptest::option::of("[a-z]{2,8}")).prop_map(|(n, by)| AcScope::Descoped {
      to: format!("ST{n:04}"),
      by,
      reason: None,
    }),
    ("[a-z ]{1,30}", proptest::option::of("[a-z]{2,8}"))
      .prop_map(|(reason, by)| AcScope::Withdrawn { reason, by }),
  ]
}

prop_compose! {
  fn work_package()(seq in 1u32..99, title in "[A-Za-z ]{1,40}", scope in tshirt(), status in wp_status()) -> WorkPackage {
    WorkPackage { seq, title, scope, status }
  }
}

prop_compose! {
  fn criterion()(g in 0u32..99, n in 1u32..9, text in "[A-Za-z ]{1,60}", kind in prop_oneof![Just(AcKind::Test), Just(AcKind::NonTest)], scope in ac_scope(), evidence in proptest::option::of("[a-z/.]{3,30}"), satisfied in proptest::option::of(any::<bool>())) -> Criterion {
    Criterion { id: format!("AC-{g:02}.{n}"), text, kind, scope, evidence, satisfied }
  }
}

prop_compose! {
  fn acceptance_test()(g in 0u32..99, n in 1u32..9, kind in prop_oneof![Just(AtKind::Test), Just(AtKind::NonTest)], file in proptest::option::of("[a-z]{2,8}/[a-z_]{2,12}\\.rs"), prose in proptest::option::of("[a-z ]{3,30}"), covers in prop::collection::vec("AC-[0-9]{2}\\.[0-9]", 1..3), status in at_status(), note in proptest::option::of("[a-z ]{1,20}")) -> AcceptanceTest {
    AcceptanceTest { id: format!("AT-{g:02}.{n}"), kind, file, prose, covers, status, note }
  }
}

prop_compose! {
  fn thread()(n in 0u32..9999, title in "[A-Za-z ]{1,60}", slug in proptest::option::of("[a-z-]{3,20}"), status in thread_status(), completed in proptest::option::of(Just("2026-08-14".to_string())), exempt in any::<bool>(), wps in prop::collection::vec(work_package(), 0..3), criteria in prop::collection::vec(criterion(), 0..3), tests in prop::collection::vec(acceptance_test(), 0..3)) -> Thread {
    Thread {
      schema: THREAD_SCHEMA.to_string(),
      id: format!("ST{n:04}"),
      title,
      slug,
      status,
      created: "2026-08-14".to_string(),
      completed,
      acceptance: exempt.then_some(AcceptanceMode::Exempt),
      wps,
      criteria,
      tests,
    }
  }
}

proptest! {
  /// parse(render(m)) == m -- the model survives its canonical JSON form.
  #[test]
  fn thread_round_trips(t in thread()) {
    let rendered = to_canonical_json(&t).expect("render");
    let parsed: Thread = serde_json::from_str(&rendered).expect("parse of own render");
    prop_assert_eq!(&parsed, &t);
    // Canonical form is a fixed point: re-render is byte-identical.
    prop_assert_eq!(to_canonical_json(&parsed).expect("re-render"), rendered);
  }
}

#[test]
fn canonical_form_is_two_space_lf_with_trailing_newline() {
  let t = sample_thread();
  let rendered = to_canonical_json(&t).expect("render");
  assert!(
    rendered.ends_with('\n'),
    "canonical JSON carries a trailing newline"
  );
  assert!(!rendered.contains('\r'), "canonical JSON is LF-only");
  assert!(
    rendered.contains("\n  \"id\""),
    "canonical JSON indents with 2 spaces"
  );
}

/// Every model struct refuses an unknown field BY NAME (D05). A typo'd or
/// stale field must never be silently dropped -- that is how a mutation
/// reports success having quietly not happened.
#[test]
fn unknown_fields_are_refused_by_name() {
  type ParseErr = fn(serde_json::Value) -> Option<String>;
  let cases: Vec<(&str, serde_json::Value, ParseErr)> = vec![
    (
      "Thread",
      serde_json::to_value(sample_thread()).unwrap(),
      |v| {
        serde_json::from_value::<Thread>(v)
          .err()
          .map(|e| e.to_string())
      },
    ),
    (
      "Issue",
      serde_json::to_value(sample_issue()).unwrap(),
      |v| {
        serde_json::from_value::<Issue>(v)
          .err()
          .map(|e| e.to_string())
      },
    ),
  ];
  for (name, mut value, parse_err) in cases {
    value
      .as_object_mut()
      .expect("model roots are objects")
      .insert("bogus_field".to_string(), serde_json::json!(1));
    let err =
      parse_err(value).unwrap_or_else(|| panic!("{name}: an unknown field must refuse, not drop"));
    assert!(
      err.contains("bogus_field"),
      "{name}: the refusal names the field, got: {err}"
    );
  }
}

fn sample_thread() -> Thread {
  Thread {
    schema: THREAD_SCHEMA.to_string(),
    id: "ST0056".to_string(),
    title: "Intent v3.0.0".to_string(),
    slug: Some("intent-v3".to_string()),
    status: ThreadStatus::Wip,
    created: "2026-08-14".to_string(),
    completed: None,
    acceptance: None,
    wps: vec![WorkPackage {
      seq: 2,
      title: "Workspace and reified model".to_string(),
      scope: TShirt::L,
      status: WpStatus::Wip,
    }],
    criteria: vec![Criterion {
      id: "AC-02.4".to_string(),
      text: "laws hold".to_string(),
      kind: AcKind::Test,
      scope: AcScope::InScope,
      evidence: None,
      satisfied: None,
    }],
    tests: vec![AcceptanceTest {
      id: "AT-02.4".to_string(),
      kind: AtKind::Test,
      file: Some("crates/intentsvcs/tests/model_laws.rs".to_string()),
      prose: None,
      covers: vec!["AC-02.4".to_string()],
      status: AtStatus::Red,
      note: None,
    }],
  }
}

fn sample_issue() -> Issue {
  Issue {
    schema: intentsvcs::model::ISSUE_SCHEMA.to_string(),
    number: 21,
    slug: "credo-checks".to_string(),
    title: "prune the dead mechanism".to_string(),
    status: IssueStatus::Closed,
    severity: Some("medium".to_string()),
    created: "2026-08-14".to_string(),
    closed: Some("2026-08-14".to_string()),
  }
}
