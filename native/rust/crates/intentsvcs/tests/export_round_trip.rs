//! **AC-06.6 -- `intent export --format <fmt>` round-trips, or refuses by
//! name.**
//!
//! The criterion is a disjunction, so a guard that only checks one arm passes
//! on an estate where the other is broken. Both are checked here, and both are
//! checked by WALKING THE ROSTER rather than by naming formats: a format added
//! to `export::FORMATS` is picked up by these tests on the next run, whichever
//! arm it declares, because there is no second list for it to be missing from.
//!
//! **The third test is the one that makes the first two mean anything.** Every
//! format currently in the roster passes, which is indistinguishable from a
//! verifier that checks nothing -- so a projection that deliberately drops data
//! is driven through the same code and required to be refused. Without it,
//! `every_format_that_claims_to_round_trip_...` is a coverage claim asserted by
//! its own name.

mod common;

use common::{PROJECT_ID, sample_issue, sample_thread};
use intentsvcs::event::Envelope;
use intentsvcs::export::{
  self, Bundle, DEFAULT_FORMAT, ExportRefusal, FORMATS, Format, Projection,
};
use intentsvcs::model::{AcKind, AcState, Criterion, Issue, Thread};

/// Scalars a YAML reader turns into something that is not a string.
///
/// **This list is what refused the YAML row**, and it is what turned
/// design.md:57's "trivial via serde" from a claim into a measurement. Issue
/// 0012's quoting hazard -- the scar that put JSON in the canon rather than
/// YAML -- is one row of it.
///
/// It stays here after that decision because it is not really a YAML list: it
/// is a list of values that break ENCODINGS, and it is now the fixture every
/// format in the roster is round-tripped against. The next format added will be
/// measured on the same values that caught this one.
///
/// They are not exotic. `no` is an answer someone types into a note; `007` is
/// an issue number; `12:30` is a time in a title; `- item` is the first line of
/// a pasted list. Every one of them is what a title, a body or a piece of
/// evidence actually contains.
const HAZARDS: &[&str] = &[
  // YAML 1.1 booleans -- the Norway problem, and the one everybody has heard of
  "no",
  "yes",
  "on",
  "off",
  "true",
  "false",
  "y",
  "n",
  // nulls
  "null",
  "Null",
  "NULL",
  "~",
  // numbers, including the ones that lose a leading zero or a trailing zero
  "007",
  "1.0",
  "0x1F",
  "0o17",
  "1e3",
  "+1",
  "-0",
  // sexagesimal under YAML 1.1: a time in a title becomes an integer
  "12:30",
  "1:2:3",
  // floats with no numeric spelling
  ".inf",
  "-.inf",
  ".nan",
  // a date, which a YAML reader may hand back as a timestamp rather than text
  "2026-08-14",
  // indicators: characters that mean something structural in the wrong column
  "=",
  "-",
  "?",
  ":",
  "a: b",
  "- item",
  "# not a comment",
  "*anchor",
  "&ref",
  "!tag",
  "|block",
  ">folded",
  "%directive",
  "@reserved",
  "`backtick",
  // whitespace, which a block scalar cannot preserve at an edge
  " leading",
  "trailing ",
  "  both  ",
  // control and multi-line
  "\ttab",
  "line\nbreak",
  "trailing \nspace before newline",
  "\n",
  "",
  // quoting and escaping
  "\"quoted\"",
  "'single'",
  "back\\slash",
  "\\n not a newline",
  // unicode
  "emoji 🦘 and ünïcödé",
  // long enough that an emitter may want to wrap it, which would insert a
  // newline into a value that has none
  "a very long single line that runs well past any sensible column limit and must not acquire a line break on the way through, because a break inserted here is a byte the canon did not have",
];

/// A thread carrying every hazard in a modelled string field.
///
/// Built ON TOP of the shared `sample_thread`, so it keeps that fixture's
/// coverage -- both AC kinds, all the off-scope states, a legacy-carried AT,
/// markup-bearing prose -- and adds the scalar hazards to it. A fresh minimal
/// thread would have traded one kind of coverage for the other.
fn adversarial_thread(id: &str) -> Thread {
  let mut thread = sample_thread(id);
  thread.title = format!("no · 12:30 · {}", thread.title);
  thread.objective = format!("{}\n\n{}", thread.objective, HAZARDS.join(" | "));
  if let Some(wp) = thread.wps.last_mut() {
    wp.body = format!("{}\n\n{}", wp.body, HAZARDS.join("\n"));
  }
  // One criterion per hazard, so each hazard is a value in its own right
  // rather than a fragment inside a longer string. A hazard concatenated into
  // a sentence is quoted for reasons that have nothing to do with the hazard,
  // which is how a fixture comes to prove nothing.
  for (i, hazard) in HAZARDS.iter().enumerate() {
    thread.criteria.push(Criterion {
      id: format!("AC-99.{}", i + 1),
      text: (*hazard).to_string(),
      kind: AcKind::NonTest,
      state: AcState::Unsatisfied,
    });
  }
  thread
}

fn adversarial_issue(number: u32) -> Issue {
  let mut issue = sample_issue(number);
  issue.title = "no".to_string();
  issue.severity = Some("007".to_string());
  issue
}

/// A bundle with DB-STAMPED events in it.
///
/// The events come from a real facade mutation rather than from struct
/// literals, so their `ts` values were set by SQLite at INSERT (D42) and this
/// fixture confects no time at all. It also means the event rows in the bundle
/// are the shape the product produces, not the shape a test author assumed.
fn bundle() -> Bundle {
  let fixture = common::Fixture::new();
  let mut facade = fixture.facade();
  facade
    .st_new("a thread, so the log has a real record in it")
    .expect("st_new");
  let events: Vec<Envelope> = facade.store().events().expect("events");
  assert!(
    !events.is_empty(),
    "the fixture must carry events, or the events half of the canon round-trips vacuously"
  );
  Bundle::new(
    PROJECT_ID,
    vec![adversarial_thread("ST0001"), adversarial_thread("ST0002")],
    vec![adversarial_issue(9), adversarial_issue(21)],
    events,
  )
}

/// **The anti-vacuity check, and it comes first because the two tests after it
/// are conditional on it.**
///
/// Each of those walks the roster and asserts something about the rows of one
/// kind. An empty roster, or one with no rows of a given kind, makes the
/// corresponding test pass having examined nothing -- and it would keep its
/// name, which is the form of green this project keeps catching.
#[test]
fn the_roster_carries_both_kinds_so_neither_arm_is_asserted_over_an_empty_set() {
  let round_trips = FORMATS
    .iter()
    .filter(|f| matches!(f.projection, Projection::RoundTrips { .. }))
    .count();
  let lossy = FORMATS
    .iter()
    .filter(|f| matches!(f.projection, Projection::Lossy { .. }))
    .count();
  assert!(
    round_trips > 0,
    "AC-06.6's first arm needs at least one format that round-trips"
  );
  assert!(
    lossy > 0,
    "AC-06.6's second arm needs at least one format that is refused by name; with none, \
     `a_format_that_cannot_carry_the_canon_back_...` examines nothing and still passes"
  );
  // Names are the operator's handle on a format and the refusal lists them, so
  // two rows answering to one name would make a refusal ambiguous and a lookup
  // arbitrary.
  let mut names = export::names();
  names.sort_unstable();
  let before = names.len();
  names.dedup();
  assert_eq!(before, names.len(), "two formats share a name: {names:?}");
}

/// The first arm: **what it emits re-ingests to a byte-identical canon.**
///
/// Asserted directly here -- emit, read back, compare canon -- rather than by
/// trusting `project_with`'s internal verification, so the guard and the
/// mechanism are two things. If the production check were deleted, this still
/// fails.
#[test]
fn every_format_that_claims_to_round_trip_re_derives_the_canon_byte_for_byte() {
  let bundle = bundle();
  let source = export::canon_parts(&bundle).expect("canon of the fixture");
  assert!(
    source.len() >= 3,
    "the fixture must carry threads, issues and a log, or 'the canon' is one file"
  );

  for format in FORMATS {
    let Projection::RoundTrips { emit, read } = &format.projection else {
      continue;
    };
    let name = format.name;
    let text = emit(&bundle).unwrap_or_else(|e| panic!("{name}: emitting failed: {e}"));
    let back = read(&text).unwrap_or_else(|e| panic!("{name}: reading back failed: {e}"));
    let after = export::canon_parts(&back).expect("canon of the round-trip");

    assert_eq!(
      source.len(),
      after.len(),
      "{name}: the round-trip changed how many canon files the estate has"
    );
    for (before, after) in source.iter().zip(after.iter()) {
      assert_eq!(
        before.0, after.0,
        "{name}: the round-trip reordered the canon"
      );
      assert_eq!(
        before.1, after.1,
        "{name}: {} did not survive the round-trip",
        before.0
      );
    }

    // And the command-level behaviour agrees with the property: a format that
    // round-trips is one the exporter actually emits.
    assert!(
      export::project_with(&bundle, format).is_ok(),
      "{name}: the property holds but the exporter refused it"
    );
  }
}

/// **The fixture is in the right world**, checked rather than assumed.
///
/// The round-trip test above passes if the hazards never reached the bundle: it
/// would be comparing two copies of tame data and reporting success, under a
/// name claiming otherwise. So every hazard is required to be findable in the
/// canon that is actually being round-tripped.
#[test]
fn the_hazards_this_fixture_claims_to_carry_are_in_the_canon_it_round_trips() {
  let bundle = bundle();
  let canon = export::canon_parts(&bundle)
    .expect("canon")
    .iter()
    .map(|(_, text)| text.clone())
    .collect::<String>();
  for hazard in HAZARDS {
    if hazard.trim().is_empty() {
      continue;
    }
    // Compared in the canon's own escaping rather than raw, so a hazard
    // containing a newline or a backslash is looked for in the form the file
    // actually holds -- otherwise those rows would silently never match and
    // the check would be weakest on the nastiest values.
    let escaped = serde_json::to_string(hazard).expect("json string");
    let inner = escaped.trim_matches('"');
    assert!(
      canon.contains(inner),
      "the fixture claims to carry {hazard:?} and the canon does not"
    );
  }
}

/// **YAML is refused on a measurement, and this pins the measurement to the
/// row.**
///
/// The row is the one place in the estate where a format our own reader
/// round-trips perfectly is nonetheless refused, so it is the one most likely
/// to be "fixed" by someone who runs the round trip, sees green, and concludes
/// the refusal is stale. It is not stale: the round trip was green when the
/// decision was made. What fails is the third-party read, which no test in this
/// language can perform.
///
/// So this asserts what the alternative cannot -- that the reason travels with
/// the refusal. Re-enabling YAML means deleting this test, which means reading
/// the row, which is the entire object.
#[test]
fn the_yaml_row_is_refused_and_carries_the_measurement_that_refused_it() {
  let yaml = export::find("yaml").expect(
    "the yaml row stays in the roster as a REFUSAL rather than being deleted: a format that is \
     simply absent is refused as 'no such format', which tells a YAML user to check their \
     spelling instead of telling them what to do",
  );
  let Projection::Lossy { because, instead } = &yaml.projection else {
    panic!(
      "yaml is emitted again. Its round trip was ALREADY green when it was refused -- \
       serde_norway 0.9.42 survived 24 of 24 hazards. What failed was PyYAML 6.0.3 reading the \
       same bytes: 6 of 24 corrupted, `no` to False and `2026-08-14` to a date object. Re-check \
       that before trusting a green round trip."
    );
  };
  // The numbers, not just a sentiment: a reason that says "YAML is risky" is an
  // opinion someone will overrule, and one that says what was measured is not.
  for cited in ["PyYAML", "6 of 24", "2026-08-14", "12:30"] {
    assert!(
      because.contains(cited),
      "the refusal drops {cited:?}, so a reader cannot tell a measurement from a preference: {because}"
    );
  }
  // And the route has to be one that actually serves the refused user, rather
  // than a shrug. It is only defensible because YAML 1.2 is a superset of JSON
  // -- measured: PyYAML reads the JSON export with 0 of 26 values corrupted.
  assert!(
    instead.contains("json"),
    "a YAML user refused with no YAML-shaped answer has simply been refused: {instead}"
  );
}

/// The second arm: **refused by name rather than emitted lossily.**
///
/// The refusal has to carry three things, and the third is the one that gets
/// dropped: WHICH format (so the operator knows the spelling was understood),
/// WHY (so they do not try again), and WHAT INSTEAD (so the refusal is a
/// junction rather than a wall).
#[test]
fn a_format_that_cannot_carry_the_canon_back_is_refused_by_name_with_a_reason_and_a_route() {
  let bundle = bundle();
  for format in FORMATS {
    let Projection::Lossy { .. } = &format.projection else {
      continue;
    };
    let refusal = export::project_with(&bundle, format)
      .expect_err("a format that cannot carry the canon back must not be emitted");
    let ExportRefusal::Lossy {
      name,
      because,
      instead,
    } = refusal
    else {
      panic!("{}: refused for the wrong reason: {refusal:?}", format.name);
    };
    assert_eq!(name, format.name, "the refusal names the format asked for");
    assert!(
      !because.trim().is_empty(),
      "{name}: refused with no reason -- the operator's next move is to try again"
    );
    assert!(
      !instead.trim().is_empty(),
      "{name}: refused with no route, which is a wall"
    );
  }
}

/// **The verifier has teeth**, proved by a projection that lies.
///
/// Two lies, because they fail in different places and only one of them is the
/// obvious one. Dropping a whole collection changes a file wholesale; changing
/// one character inside a title changes a file that is otherwise perfect, and
/// that is the shape a real encoding bug takes -- the export looks entirely
/// correct and one value has quietly become something else.
///
/// **This is the mutation test performed rather than described.** Every format
/// in the roster passes the round-trip, so nothing else here can tell a working
/// verifier from an absent one.
#[test]
fn a_projection_that_drops_or_alters_data_is_refused_rather_than_emitted() {
  let bundle = bundle();

  fn read_json(text: &str) -> Result<Bundle, String> {
    serde_json::from_str(text).map_err(|e| e.to_string())
  }

  // A projection that writes everything except the history.
  fn emit_without_events(bundle: &Bundle) -> Result<String, String> {
    let mut copy = bundle.clone();
    copy.events.clear();
    serde_json::to_string(&copy).map_err(|e| e.to_string())
  }

  // A projection that writes everything and gets ONE character wrong -- the
  // encoding bug, rather than the missing-field bug.
  fn emit_with_a_mangled_title(bundle: &Bundle) -> Result<String, String> {
    let mut copy = bundle.clone();
    if let Some(thread) = copy.threads.first_mut() {
      thread.title = thread.title.replace("no", "false");
    }
    serde_json::to_string(&copy).map_err(|e| e.to_string())
  }

  let dropped = Format {
    name: "drops-the-log",
    help: "a projection that silently omits the event log",
    projection: Projection::RoundTrips {
      emit: emit_without_events,
      read: read_json,
    },
  };
  let refusal = export::project_with(&bundle, &dropped)
    .expect_err("a projection that drops the log must be refused, not written");
  let ExportRefusal::RoundTripFailed { name, detail } = refusal else {
    panic!("refused for the wrong reason: {refusal:?}");
  };
  assert_eq!(name, "drops-the-log");
  assert!(
    detail.contains("events.jsonl"),
    "the refusal names the canon file that did not survive: {detail}"
  );

  let mangled = Format {
    name: "reads-no-as-false",
    help: "a projection standing in for a scalar that changes meaning",
    projection: Projection::RoundTrips {
      emit: emit_with_a_mangled_title,
      read: read_json,
    },
  };
  let refusal = export::project_with(&bundle, &mangled)
    .expect_err("a projection that alters one value must be refused, not written");
  let ExportRefusal::RoundTripFailed { name, detail } = refusal else {
    panic!("refused for the wrong reason: {refusal:?}");
  };
  assert_eq!(name, "reads-no-as-false");
  assert!(
    detail.contains("ST0001/thread.json"),
    "the refusal names the file the difference is in: {detail}"
  );
  assert!(
    detail.contains("first difference at byte"),
    "and carries the evidence, which the operator cannot reproduce -- the emitted text was never written: {detail}"
  );
}

/// **An unknown format offers only what it can actually deliver.**
///
/// Found by running the command rather than by reading it: `--format xml`
/// answered "one of: json, yaml, md", and two of those three refuse. A remedy
/// whose suggestions are themselves refusals costs the operator another command
/// and returns them to the same place, which is the defect this estate already
/// records under borrowed remedies -- here in the one message whose entire job
/// is to say what to type next.
///
/// The declined names are still CARRIED, because someone who guessed `xml` may
/// guess `yaml` next; they are reported as declined rather than offered.
#[test]
fn an_unknown_format_offers_only_formats_that_emit_and_never_a_refused_one() {
  let refusal = export::project(&bundle(), "xml").expect_err("there is no xml projection");
  let ExportRefusal::Unknown {
    name,
    emits,
    refused,
  } = refusal
  else {
    panic!("refused for the wrong reason: {refusal:?}");
  };
  assert_eq!(name, "xml");
  assert_eq!(
    emits,
    export::emitting_names()
      .iter()
      .map(|s| s.to_string())
      .collect::<Vec<_>>(),
    "the operator is offered the formats that produce an artefact"
  );
  assert!(!emits.is_empty(), "and there is at least one to offer");
  for declined in &refused {
    assert!(
      !emits.contains(declined),
      "{declined:?} refuses and is being offered as the remedy for a refusal"
    );
  }
  // The two lists together are the roster: nothing is silently dropped from
  // the operator's view, it is only sorted into can-have and cannot.
  let mut all: Vec<String> = emits.iter().chain(refused.iter()).cloned().collect();
  all.sort();
  let mut roster: Vec<String> = export::names().iter().map(|s| s.to_string()).collect();
  roster.sort();
  assert_eq!(all, roster, "a format in the roster is in neither list");
}

/// **The default is a format that exists and round-trips.**
///
/// A default naming a row that is not in the roster turns the bare command --
/// the one most people run -- into a refusal, and it would be invisible to
/// every test above, all of which name their format explicitly.
#[test]
fn the_default_format_is_in_the_roster_and_is_one_that_round_trips() {
  let format = export::find(DEFAULT_FORMAT)
    .unwrap_or_else(|| panic!("the default `{DEFAULT_FORMAT}` is not in the roster"));
  assert!(
    matches!(format.projection, Projection::RoundTrips { .. }),
    "the bare `intent export` must not default to a format that refuses"
  );
}
