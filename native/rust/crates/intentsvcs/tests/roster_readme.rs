//! ST0092 WP-07: `intent/whiteboard/README.md` is rendered from the declared
//! Team Profiles and the store by `wb init --apply`, and everything inside its
//! preserved block is the project's own and survives every render.
//!
//! Every arm runs in a fresh fixture project and reads the shipped profiles
//! from the install this test binary resolves, which is this repository, so
//! the seats, remits and route come from the profile files and are never
//! written down here.

use intentsvcs::facade::Facade;
use intentsvcs::teams::Teams;

use crate::common::{Fixture, ctx};

const TEAM: &str = "team";
const README: &str = "intent/whiteboard/README.md";
const START: &str = "<!-- user:start -->";
const END: &str = "<!-- user:end -->";

fn names(list: &[&str]) -> Vec<String> {
  list.iter().map(|s| (*s).to_string()).collect()
}

/// An apply of the team profile that runs.
fn apply(f: &mut Facade) -> intentsvcs::facade::WbInit {
  f.wb_init(&names(&[TEAM]), true, false)
    .expect("the apply runs")
}

/// An apply of the team profile that refuses, and why.
fn refused(f: &mut Facade) -> intentsvcs::facade::FacadeError {
  f.wb_init(&names(&[TEAM]), true, false)
    .expect_err("the apply refuses")
}

/// The `team` profile resolved from the shipped files.
fn team() -> intentsvcs::teams::Resolved {
  Teams::new(&testkit::repo_root())
    .resolve(&names(&[TEAM]), None, false)
    .expect("team resolves")
}

/// The text inside the README's block.
fn block(readme: &str) -> &str {
  let start = readme.find(START).expect("a start marker") + START.len();
  let end = readme[start..].find(END).expect("an end marker") + start;
  &readme[start..end]
}

/// The roster table's lines: every line from the header row to the last row.
fn table(readme: &str) -> Vec<&str> {
  readme
    .lines()
    .skip_while(|l| !l.starts_with("| "))
    .take_while(|l| l.starts_with('|'))
    .collect()
}

/// Doctor's findings of `class`, as its id prints, with whether each is counted
/// and whether it is shown by default.
fn doctor_class(fx: &Fixture, class: &str) -> Vec<(String, bool, bool)> {
  let report =
    intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::Live);
  report
    .findings
    .iter()
    .filter(|f| f.class.as_str() == class)
    .map(|f| {
      (
        f.file.clone(),
        f.class.is_actionable(),
        f.class.is_shown_by_default(),
      )
    })
    .collect()
}

/// ST0092 AT-07.1: an apply renders the README from the one template: a title,
/// the declared profile, a roster row for each seat with its moniker, name,
/// role, driver and remit, the table padded as the project's formatter pads
/// one, the route to hv naming the mediator as the one reader of hv's inbox,
/// and the verb that prints the terms.
#[test]
fn at_07_1_an_apply_renders_the_roster_readme_from_the_profile_and_the_store() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f);
  let readme = fx.read(README);
  let resolved = team();

  assert!(readme.starts_with("# Fixture "), "no title: {readme}");
  assert!(
    readme.contains(&format!("`{TEAM}`")),
    "no profile: {readme}"
  );
  let rows = table(&readme);
  for seat in &resolved.seats {
    let row = rows
      .iter()
      .find(|r| r.starts_with(&format!("| {} ", seat.moniker)))
      .unwrap_or_else(|| panic!("no row for {}: {readme}", seat.moniker));
    for cell in [&seat.name, &seat.role, &seat.remit] {
      assert!(
        row.contains(cell.as_str()),
        "{} lacks {cell}: {row}",
        seat.moniker
      );
    }
    assert!(
      row.contains(seat.driver.phrase()),
      "{} lacks its driver: {row}",
      seat.moniker
    );
  }
  let width = rows[0].chars().count();
  assert!(
    rows.iter().all(|r| r.chars().count() == width),
    "the table's columns are not padded to one width: {rows:#?}"
  );
  assert!(
    rows[1]
      .trim_matches('|')
      .split('|')
      .all(|c| c.trim().chars().all(|ch| ch == '-')),
    "the second row is not the separator: {}",
    rows[1]
  );

  let route = readme
    .split("## Route to hv")
    .nth(1)
    .expect("a route to hv section");
  assert!(
    route.contains(&format!("`{}`", resolved.mediator))
      && route.contains("the one reader of hv's inbox"),
    "the route does not name the mediator as hv's inbox's one reader: {route}"
  );
  assert!(
    readme.contains("`intent wb profile terms`"),
    "no terms verb: {readme}"
  );
  assert!(
    readme.trim_end().ends_with(END),
    "the template does not end on its block: {readme}"
  );
}

/// ST0092 AT-07.1, on a board whose mediator is not `vc` (vc's condition on
/// the route): the route names the declaration's mediator, with the name the
/// store holds for it, and no other seat.
#[test]
fn at_07_1_the_route_names_the_declared_mediator_whichever_seat_it_is() {
  const SOLO: &str = "solo";
  let resolved = Teams::new(&testkit::repo_root())
    .resolve(&names(&[SOLO]), None, false)
    .expect("solo resolves");
  assert_ne!(
    resolved.mediator,
    team().mediator,
    "the fixture needs another mediator"
  );
  let mediator = resolved
    .seats
    .iter()
    .find(|s| s.moniker == resolved.mediator)
    .expect("the mediator is a seat");

  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_init(&names(&[SOLO]), true, false)
    .expect("the apply runs");
  let readme = fx.read(README);
  let route = readme
    .split("## Route to hv")
    .nth(1)
    .expect("a route to hv section");
  assert!(
    route.contains(&format!("`{}` ({})", mediator.moniker, mediator.name)),
    "the route does not name {}: {route}",
    mediator.moniker
  );
  assert!(
    !route.contains(&format!("`{}`", team().mediator)),
    "the route names the team profile's mediator: {route}"
  );
}

/// ST0092 AT-07.2: what the project writes inside the block is carried byte
/// for byte across a re-render, by the function that carries CLAUDE.md's
/// block and by no copy of it.
#[test]
fn at_07_2_the_block_is_carried_byte_for_byte_by_canons_one_function() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f);
  let ours = "\n## Our notes\n\nKeep  two spaces, a trailing one \nand *stars*.\n\n";
  let readme = fx.read(README);
  fx.write_file(
    README,
    &readme.replacen(&format!("{START}\n"), &format!("{START}{ours}"), 1),
  );

  let seat = team().seats[0].moniker.clone();
  f.wb_remit(&seat, Some("a remit this project set"))
    .expect("a remit");
  apply(&mut f);
  let after = fx.read(README);
  assert!(
    after.contains("a remit this project set"),
    "the re-render did not run: {after}"
  );
  assert_eq!(
    block(&after),
    ours,
    "the block was not carried byte for byte"
  );

  let source = std::fs::read_to_string(
    testkit::repo_root().join("native/rust/crates/intentsvcs/src/roster_readme.rs"),
  )
  .expect("the roster README module");
  assert!(
    source.contains("canon::carry_user_block("),
    "the README is not carried by canon's function"
  );
  assert!(
    !source.contains("fn carry_user_block"),
    "the module carries its own copy of the block function"
  );
}

/// ST0092 AT-07.3: a first render over a README Intent never rendered carries
/// the whole file inside the block, except a frontmatter block that opens it,
/// which is dropped.
#[test]
fn at_07_3_a_first_render_carries_the_whole_old_readme_and_drops_its_verblock() {
  let fx = Fixture::new();
  let prose =
    "# Our roster\n\nWritten by hand, with its own history.\n\n| a | b |\n|---|---|\n| 1 | 2 |\n";
  fx.write_file(
    README,
    &format!("---\nverblock: \"11 Nov 2026:v0.1: dc - hand-written\"\n---\n\n{prose}"),
  );
  let mut f = fx.facade_on_disk();
  apply(&mut f);
  let readme = fx.read(README);
  assert!(
    block(&readme).contains(prose),
    "the old README is not inside the block: {readme}"
  );
  assert!(
    !readme.contains("verblock:"),
    "the verblock was carried: {readme}"
  );
  assert!(
    readme.starts_with("# Fixture "),
    "the generated part does not open the file: {readme}"
  );
}

/// ST0092 AT-07.4: the generated part states nothing the profiles and the
/// store do not declare (issue 0672): two projects with the same declaration
/// and remits differ only in the project's name.
#[test]
fn at_07_4_two_projects_with_one_declaration_differ_only_in_their_name() {
  let render = |name: &str| {
    let fx = Fixture::new();
    fx.write_file(
      "intent/.config/config.json",
      &format!(
        "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"{name}\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}}\n"
      ),
    );
    let mut f = fx.facade_on_disk();
    apply(&mut f);
    fx.read(README)
  };
  let one = render("Alpha");
  let two = render("Bravo");
  assert_ne!(one, two, "the project's name is not in the README");
  assert_eq!(
    one.replace("Alpha", "Bravo"),
    two,
    "the README states something other than its declaration"
  );
}

/// ST0092 AT-07.5: a hand edit outside the block is `roster-readme-edited`,
/// shown and never counted, and an apply refuses it before writing anything,
/// naming the line; a README only behind this build's render is
/// `roster-readme-behind`, shown and never counted, and an apply re-renders
/// it; and a re-render with nothing changed writes nothing.
#[test]
fn at_07_5_an_edit_outside_the_block_refuses_and_a_behind_readme_re_renders() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f);

  let before = crate::common::tree(fx.root());
  apply(&mut f);
  let mut after = crate::common::tree(fx.root());
  after.retain(|p, _| !p.starts_with("intent/.cache/"));
  let mut held = before.clone();
  held.retain(|p, _| !p.starts_with("intent/.cache/"));
  assert_eq!(held, after, "a re-render with nothing changed wrote");

  let seat = team().seats[0].moniker.clone();
  f.wb_remit(&seat, Some("set after the render"))
    .expect("a remit");
  let behind = doctor_class(&fx, "roster-readme-behind");
  assert!(
    behind.iter().any(|(file, ..)| file == README),
    "not reported behind: {behind:?}"
  );
  assert!(behind.iter().all(|(_, counted, shown)| !counted && *shown));
  apply(&mut f);
  assert!(fx.read(README).contains("set after the render"));

  let edited = fx
    .read(README)
    .replacen("## Roster", "## Roster, as we call it", 1);
  fx.write_file(README, &edited);
  let found = doctor_class(&fx, "roster-readme-edited");
  assert!(
    found.iter().any(|(file, ..)| file == README),
    "not reported edited: {found:?}"
  );
  assert!(found.iter().all(|(_, counted, shown)| !counted && *shown));
  let line = edited
    .lines()
    .position(|l| l == "## Roster, as we call it")
    .expect("the edited line")
    + 1;
  let refusal = refused(&mut f);
  let said = format!("{refusal} {}", intentsvcs::remedy::Remedy::remedy(&refusal));
  assert!(
    said.contains(&format!("line {line}")),
    "the refusal names no line: {said}"
  );
  assert!(
    said.contains("block"),
    "the remedy does not say to move it into the block: {said}"
  );
  assert_eq!(
    fx.read(README),
    edited,
    "the refused apply wrote the README"
  );
}

/// ST0092 AT-07.6: a seat whose remit was never set shows its profile's
/// default, marked as the default; a seat whose remit the project set shows
/// the project's words.
#[test]
fn at_07_6_an_unset_remit_shows_the_default_marked_and_a_set_one_the_projects_words() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f);
  let resolved = team();
  let (set, unset) = (&resolved.seats[0], &resolved.seats[1]);
  f.wb_remit(&set.moniker, Some("our own words"))
    .expect("a remit");
  apply(&mut f);
  let readme = fx.read(README);
  let row = |m: &str| {
    table(&readme)
      .into_iter()
      .find(|r| r.starts_with(&format!("| {m} ")))
      .unwrap_or_else(|| panic!("no row for {m}"))
      .to_string()
  };
  let unset_row = row(&unset.moniker);
  assert!(
    unset_row.contains(&unset.remit) && unset_row.contains("(the profile's default)"),
    "{unset_row}"
  );
  let set_row = row(&set.moniker);
  assert!(
    set_row.contains("our own words") && !set_row.contains("(the profile's default)"),
    "{set_row}"
  );
}

/// ST0092 AT-07.7: a render refuses, before writing and naming the line, a
/// README whose markers are anything but one start followed by one end, and a
/// first render refuses one that holds either marker.
#[test]
fn at_07_7_stray_markers_refuse_naming_the_line() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f);
  let doubled = format!("{}\n{START}\n", fx.read(README));
  fx.write_file(README, &doubled);
  let line = doubled.lines().count();
  let refusal = refused(&mut f);
  assert!(
    refusal.to_string().contains(&format!("line {line}")),
    "{refusal}"
  );
  assert_eq!(fx.read(README), doubled, "the refused render wrote");

  let first = Fixture::new();
  let mention = format!("# Ours\n\nWe write notes between {START} and {END}.\n");
  first.write_file(README, &mention);
  let mut f = first.facade_on_disk();
  let refusal = refused(&mut f);
  assert!(refusal.to_string().contains("line 3"), "{refusal}");
  assert_eq!(
    first.read(README),
    mention,
    "the refused first render wrote"
  );
}

/// ST0092 AT-07.5, vc's property on ic's note: a README write that fails
/// records nothing, and the next apply writes the README with no refusal. The
/// write fails honestly, on a whiteboard directory that refuses it.
#[test]
fn at_07_5_a_failed_readme_write_records_nothing_and_the_next_apply_heals() {
  use std::os::unix::fs::PermissionsExt;
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  apply(&mut f);
  let declaration = fx.read("intent/whiteboard/declaration.json");
  let seat = team().seats[0].moniker.clone();
  f.wb_remit(&seat, Some("set before the failed write"))
    .expect("a remit");

  // **THE MODE IS PUT BACK BY A GUARD**, which runs when an assert below
  // panics too, so a red arm leaves no tree its own cleanup cannot delete.
  // The write fails for every account the legs run as: none runs as root
  // (GitHub's runners and kovacs run as a user), and a root run would apply
  // and fail `expect_err` below, never pass by writing through.
  struct Restore(std::path::PathBuf, std::fs::Permissions);
  impl Drop for Restore {
    fn drop(&mut self) {
      let _ = std::fs::set_permissions(&self.0, self.1.clone());
    }
  }
  let dir = fx.root().join("intent/whiteboard");
  let guard = Restore(
    dir.clone(),
    std::fs::metadata(&dir)
      .expect("the whiteboard")
      .permissions(),
  );
  std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o555)).expect("refuse writes");
  let failed = f.wb_init(&names(&[TEAM]), true, false);
  drop(guard);
  let refusal = failed.expect_err("a README write into a read-only directory fails");
  assert!(
    refusal.to_string().contains("README.md"),
    "the failure does not name the README: {refusal}"
  );
  assert_eq!(
    fx.read("intent/whiteboard/declaration.json"),
    declaration,
    "a failed README write recorded a baseline"
  );

  apply(&mut f);
  assert!(
    fx.read(README).contains("set before the failed write"),
    "the next apply did not write the README"
  );
}
