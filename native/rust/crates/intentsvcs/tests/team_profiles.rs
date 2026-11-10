//! ST0092 WP-02: the shipped Team Profiles, read through the library.
//!
//! The shipped set is read from the repository's own canon. Every refusal is
//! driven on a fixture directory built in a tempdir, so no shipped file is ever
//! mutated, and each fixture starts from one that validates clean, so a refusal
//! arm cannot pass on a fixture that was broken some other way.

use std::path::{Path, PathBuf};

use intentsvcs::teams::{
  CANON_TEAMS, Driver, Finding, ProfileKind, SEATS_FILE, SECTIONS, SeatRegistry, TERMS_FILE, Teams,
};
use testkit::repo_root;

fn shipped() -> Teams {
  Teams::new(&repo_root())
}

// ---------------------------------------------------------------------------
// AT-02.4 (ST0092): what each shipped profile declares
// ---------------------------------------------------------------------------

/// ST0092 AT-02.4: team declares vc, cc, ic, ac, dc and hv with vc the
/// mediator; solo declares gc and hv with gc the mediator; supervisor adds sc
/// and makes it the mediator; supervised adds sv and leaves the mediator alone.
#[test]
fn at_02_4_each_shipped_profile_declares_its_seats_and_mediator() {
  let teams = shipped();
  let expect: [(&str, ProfileKind, Option<&str>, &[&str]); 4] = [
    (
      "team",
      ProfileKind::Base,
      Some("vc"),
      &["vc", "cc", "ic", "ac", "dc", "hv"],
    ),
    ("solo", ProfileKind::Base, Some("gc"), &["gc", "hv"]),
    ("supervisor", ProfileKind::Layer, Some("sc"), &["sc"]),
    ("supervised", ProfileKind::Layer, None, &["sv"]),
  ];
  for (name, kind, mediator, seats) in expect {
    let p = teams
      .profile(name)
      .unwrap_or_else(|e| panic!("{name}: {e}"));
    assert_eq!(p.front.kind, kind, "{name}'s kind");
    assert_eq!(p.front.mediator.as_deref(), mediator, "{name}'s mediator");
    assert_eq!(p.front.seats, seats, "{name}'s seats");
  }
  let mut names = teams.names().expect("the shipped names");
  names.sort();
  let mut expected: Vec<&str> = expect.iter().map(|(n, ..)| *n).collect();
  expected.sort_unstable();
  assert_eq!(
    names, expected,
    "the install ships these profiles and no other"
  );
}

// ---------------------------------------------------------------------------
// AT-02.5 (ST0092): the terms document's seats table against the registry
// ---------------------------------------------------------------------------

/// One row of the terms document's seats table, as its five columns.
fn terms_rows(terms: &str) -> Vec<[String; 5]> {
  terms
    .lines()
    .filter(|l| l.starts_with("| `"))
    .map(|l| {
      let cells: Vec<String> = l
        .trim_matches('|')
        .split('|')
        .map(|c| c.trim().trim_matches('`').to_string())
        .collect();
      assert_eq!(
        cells.len(),
        5,
        "a seats-table row has moniker, name, role, driver and remit: {l}"
      );
      [
        cells[0].clone(),
        cells[1].clone(),
        cells[2].clone(),
        cells[3].clone(),
        cells[4].clone(),
      ]
    })
    .collect()
}

/// ST0092 AT-02.5: the seats table in `TERMS.md` and `seats.json` agree seat
/// for seat in moniker, name, role, driver and default remit.
#[test]
fn at_02_5_the_terms_seats_table_and_the_registry_agree() {
  let teams = shipped();
  let table = terms_rows(&teams.terms().expect("the terms document"));
  let registry = teams.registry().expect("the seat registry");
  assert!(
    !table.is_empty(),
    "the terms document's seats table was not found"
  );
  let from_registry: Vec<[String; 5]> = registry
    .seats
    .iter()
    .map(|s| {
      [
        s.moniker.clone(),
        s.name.clone(),
        s.role.clone(),
        s.driver.phrase().to_string(),
        s.remit.clone(),
      ]
    })
    .collect();
  assert_eq!(
    table, from_registry,
    "{TERMS_FILE}'s seats table and {SEATS_FILE} differ"
  );
}

/// The parity arm's own control: a registry differing in one field is caught.
#[test]
fn at_02_5_a_registry_differing_in_one_field_is_caught() {
  let teams = shipped();
  let table = terms_rows(&teams.terms().expect("the terms document"));
  let mut registry = teams.registry().expect("the seat registry");
  registry.seats[0].driver = Driver::Human;
  let changed: Vec<String> = registry
    .seats
    .iter()
    .map(|s| s.driver.phrase().to_string())
    .collect();
  let terms: Vec<String> = table.iter().map(|r| r[3].clone()).collect();
  assert_ne!(changed, terms, "a changed driver must read as a difference");
}

// ---------------------------------------------------------------------------
// AT-02.2 (ST0092): every refusal names the file and the field
// ---------------------------------------------------------------------------

const BASE: &str = "base.md";
const LAYER: &str = "layer.md";

/// A fixture profiles directory: the shipped registry, one base and one layer
/// that validate clean, each overridable.
struct Fixture {
  dir: tempfile::TempDir,
}

impl Fixture {
  fn new() -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let shipped = repo_root().join(CANON_TEAMS);
    std::fs::copy(shipped.join(SEATS_FILE), dir.path().join(SEATS_FILE)).expect("registry");
    std::fs::copy(shipped.join(TERMS_FILE), dir.path().join(TERMS_FILE)).expect("terms");
    let f = Self { dir };
    f.write(BASE, &base("mediator: vc\n", "[vc, cc, hv]", ""));
    f.write(
      LAYER,
      "---\nkind: layer\nmediator: sc\nseats: [sc]\n---\n\n# A layer\n\nIts own prose.\n",
    );
    f
  }

  fn write(&self, file: &str, text: &str) {
    std::fs::write(self.dir.path().join(file), text).expect("write fixture");
  }

  fn path(&self, file: &str) -> PathBuf {
    self.dir.path().join(file)
  }

  fn registry(&self, edit: impl FnOnce(&mut SeatRegistry)) {
    let path = self.path(SEATS_FILE);
    let mut reg: SeatRegistry =
      serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("parse");
    edit(&mut reg);
    std::fs::write(&path, serde_json::to_string_pretty(&reg).expect("json")).expect("write");
  }

  fn findings(&self) -> Vec<Finding> {
    Teams::at(self.dir.path())
      .validate()
      .expect("the fixture directory is readable")
      .0
  }
}

/// A base profile with the given mediator line, seats list and an extra line
/// appended to its first section.
fn base(mediator: &str, seats: &str, in_section: &str) -> String {
  let mut text = format!("---\nkind: base\n{mediator}seats: {seats}\n---\n\n# A base\n");
  for (i, s) in SECTIONS.iter().enumerate() {
    text.push_str(&format!(
      "\n{s}\n\nThe prose.{}\n",
      if i == 0 { in_section } else { "" }
    ));
  }
  text
}

/// The one finding a refusal produces, named by its file and field.
fn assert_one(found: &[Finding], file: &Path, field: &str) {
  assert!(
    found.len() == 1 && found[0].path == file && found[0].field == field,
    "expected one finding on {} field `{field}`, got: {found:#?}",
    file.display()
  );
  let line = found[0].to_string();
  let name = file.file_name().expect("a file").to_string_lossy();
  assert!(
    line.contains(name.as_ref()) && line.contains(field),
    "the message names the file and the field: {line}"
  );
}

/// The fixture every refusal arm starts from validates clean.
#[test]
fn at_02_2_the_fixture_starts_clean() {
  let f = Fixture::new();
  assert!(f.findings().is_empty(), "{:#?}", f.findings());
}

/// ST0092 AT-02.2: a base with no mediator.
#[test]
fn at_02_2_a_base_with_no_mediator() {
  let f = Fixture::new();
  f.write(BASE, &base("", "[vc, cc, hv]", ""));
  assert_one(&f.findings(), &f.path(BASE), "mediator");
}

/// ST0092 AT-02.2: a mediator that is not one of the profile's seats.
#[test]
fn at_02_2_a_mediator_outside_the_seats() {
  let f = Fixture::new();
  f.write(BASE, &base("mediator: ic\n", "[vc, cc, hv]", ""));
  assert_one(&f.findings(), &f.path(BASE), "mediator");
}

/// ST0092 AT-02.2: a duplicate moniker, in a profile and in the registry.
#[test]
fn at_02_2_a_duplicate_moniker() {
  let f = Fixture::new();
  f.write(BASE, &base("mediator: vc\n", "[vc, cc, cc, hv]", ""));
  assert_one(&f.findings(), &f.path(BASE), "seats");

  let f = Fixture::new();
  f.registry(|r| {
    let again = r.seats[1].clone();
    r.seats.push(again);
  });
  assert_one(&f.findings(), &f.path(SEATS_FILE), "moniker");
}

/// ST0092 AT-02.2: a role equal to its moniker.
#[test]
fn at_02_2_a_role_equal_to_its_moniker() {
  let f = Fixture::new();
  f.registry(|r| r.seats[1].role = r.seats[1].moniker.clone());
  assert_one(&f.findings(), &f.path(SEATS_FILE), "role");
}

/// ST0092 AT-02.2: a name written as a session name.
#[test]
fn at_02_2_a_name_written_as_a_session_name() {
  let f = Fixture::new();
  f.registry(|r| r.seats[1].name = format!("intent-{}", r.seats[1].moniker));
  assert_one(&f.findings(), &f.path(SEATS_FILE), "name");
}

/// ST0092 AT-02.2: a second seat driven by the human.
#[test]
fn at_02_2_a_second_seat_driven_by_the_human() {
  let f = Fixture::new();
  f.registry(|r| r.seats[1].driver = Driver::Human);
  let second = Teams::at(f.dir.path()).registry().expect("registry").seats[1]
    .moniker
    .clone();
  f.write(
    BASE,
    &base("mediator: vc\n", &format!("[vc, {second}, hv]"), ""),
  );
  assert_one(&f.findings(), &f.path(BASE), "seats");
}

/// ST0092 AT-02.2: a layer that declares a human seat.
#[test]
fn at_02_2_a_layer_that_declares_a_human_seat() {
  let f = Fixture::new();
  f.write(
    LAYER,
    "---\nkind: layer\nmediator: sc\nseats: [sc, hv]\n---\n\n# A layer\n",
  );
  assert_one(&f.findings(), &f.path(LAYER), "seats");
}

/// ST0092 AT-02.2: a moniker the seat registry does not hold.
#[test]
fn at_02_2_a_moniker_the_registry_does_not_hold() {
  let f = Fixture::new();
  f.write(BASE, &base("mediator: vc\n", "[vc, cc, zz, hv]", ""));
  assert_one(&f.findings(), &f.path(BASE), "seats");
}

/// ST0092 AT-02.2: a base that lacks one of its four sections, each in turn.
#[test]
fn at_02_2_a_base_that_lacks_a_section() {
  for missing in SECTIONS {
    let f = Fixture::new();
    let text = base("mediator: vc\n", "[vc, cc, hv]", "").replace(missing, "## Something else");
    f.write(BASE, &text);
    assert_one(&f.findings(), &f.path(BASE), "brief");
  }
}

/// ST0092 AT-02.2: a layer that carries one of the four sections.
#[test]
fn at_02_2_a_layer_that_carries_a_section() {
  for carried in SECTIONS {
    let f = Fixture::new();
    f.write(
      LAYER,
      &format!(
        "---\nkind: layer\nmediator: sc\nseats: [sc]\n---\n\n# A layer\n\n{carried}\n\nProse.\n"
      ),
    );
    assert_one(&f.findings(), &f.path(LAYER), "brief");
  }
}

/// ST0092 AT-02.2: a base section that writes its mediator as a moniker, which
/// a layer moving the mediator would leave stale.
#[test]
fn at_02_2_a_section_that_names_the_mediator_by_moniker() {
  let f = Fixture::new();
  f.write(BASE, &base("mediator: vc\n", "[vc, cc, hv]", " Ask vc."));
  assert_one(&f.findings(), &f.path(BASE), "brief");
}

/// ST0092 AT-02.2: a frontmatter key the profile schema does not hold, and a
/// kind that is neither base nor layer, each named by its field.
#[test]
fn at_02_2_an_unknown_key_or_kind() {
  let f = Fixture::new();
  f.write(
    BASE,
    &base("mediator: vc\n", "[vc, cc, hv]", "").replace("kind: base\n", "kind: base\nowner: cc\n"),
  );
  assert_one(&f.findings(), &f.path(BASE), "owner");

  let f = Fixture::new();
  f.write(
    BASE,
    &base("mediator: vc\n", "[vc, cc, hv]", "").replace("kind: base", "kind: crew"),
  );
  assert_one(&f.findings(), &f.path(BASE), "kind");
}

/// The shipped set itself validates clean, through the library.
#[test]
fn at_02_2_the_shipped_set_has_no_finding() {
  let (findings, examined) = shipped().validate().expect("the shipped set is readable");
  assert!(findings.is_empty(), "{findings:#?}");
  assert_eq!(
    examined,
    shipped().names().expect("names").len(),
    "every shipped profile was examined"
  );
}
