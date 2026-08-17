//! **D44's DONE window: the terminal is a moment, the committed file is a
//! record.**
//!
//! hv retired `todo --flush` and `--prune` and replaced two destructive verbs
//! with one non-destructive display parameter: under D01 the db is truth and
//! `todo.md` is a generated view, **so there is nothing left to prune -- there
//! is only a question of how much of the record to show.**
//!
//! vc then ruled the surface (2026-08-16, under hv's standing "go with your
//! recs", raised because hv ruled the window and not which surface it applies
//! to): **the window applies to the TERMINAL render and the committed file
//! carries everything.** A window resolved against a clock makes a file's
//! content depend on when it was generated rather than on what happened, and
//! this repository commits `todo.md` -- so a windowed file would diff with no
//! cause in the estate.
//!
//! **Every assertion is differential.** A window test that only checks "the
//! recent one is present" passes on an implementation that windows nothing at
//! all, which is exactly the state this replaces -- so each case carries a row
//! that must survive AND a row that must not, and the file is asserted to keep
//! the row the terminal drops.
//!
//! **No test here reads a clock, and the dates are boundary values rather than
//! realistic ones on purpose.** The property under test is a `>=` against a
//! cutoff SQLite resolves; a fixture that computed its own "today" in Rust
//! would be asserting the implementation's arithmetic against a second copy of
//! it, and would flip at a midnight boundary between the two clocks. `9999` is
//! inside every window and `2001` is outside every plausible one, whenever
//! this runs -- which is the same reasoning D42 applies to the shipped code,
//! turned on the test.

mod common;

use common::Fixture;
use intentsvcs::model::{Thread, ThreadStatus};
use intentsvcs::remedy::Remedy;
use intentsvcs::views::TodoWindow;

/// Inside any window, at any time this test is ever run.
const INSIDE: &str = "9999-12-31";
/// Outside any window a person would configure, likewise.
const OUTSIDE: &str = "2001-01-01";

fn done_thread(id: &str, completed: &str) -> Thread {
  let mut t = common::sample_thread(id);
  t.id = id.to_string();
  t.status = ThreadStatus::Completed;
  t.completed = Some(completed.to_string());
  t.wps.clear();
  t
}

/// Rewrite the fixture's config with a `todo.window_hours`.
fn set_window_hours(fixture: &Fixture, hours: u32) {
  fixture.write_file(
    "intent/.config/config.json",
    &format!(
      "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"],\n  \"todo\": {{ \"window_hours\": {hours} }}\n}}\n"
    ),
  );
}

/// **The window discriminates, and the FILE keeps what the terminal drops.**
///
/// Driven through the facade rather than through the pure filter, because the
/// claim is about two surfaces disagreeing -- and a unit test of the filter
/// cannot see whether the file was wired to it by mistake.
#[test]
fn the_terminal_trims_done_and_the_committed_file_does_not() {
  let fixture = Fixture::new();
  fixture.write_thread(&done_thread("ST0001", OUTSIDE));
  fixture.write_thread(&done_thread("ST0002", INSIDE));

  let facade = fixture.facade();
  let terminal = facade.todo_view_windowed().expect("windowed view");
  let file = facade.todo_view().expect("file view");

  assert!(
    terminal.contains("ST0002"),
    "a completion inside the window must be shown:\n{terminal}"
  );
  assert!(
    !terminal.contains("ST0001"),
    "one outside it must not -- if it appears, the window is filtering nothing, which is the state D44 replaced:\n{terminal}"
  );
  assert!(
    file.contains("ST0001") && file.contains("ST0002"),
    "the COMMITTED file carries every completion. Dropping rows from it would make a generated artefact a function of WHEN it was generated rather than of the \
     model, which is committed churn with no cause in the estate:\n{file}"
  );
}

/// **A longer window reaches further back**, which is the point of the setting
/// being configurable and the half a fixed default cannot demonstrate.
///
/// The same fixture and the same date, twice, with only the configuration
/// changing -- so a pass cannot come from the two cases differing in some
/// other way.
#[test]
fn widening_the_window_admits_an_older_completion() {
  let fixture = Fixture::new();
  fixture.write_thread(&done_thread("ST0001", OUTSIDE));

  let narrow = fixture
    .facade()
    .todo_view_windowed()
    .expect("default window");
  assert!(
    !narrow.contains("ST0001"),
    "outside the 24-hour default:\n{narrow}"
  );

  // A century, in hours. Wide enough to admit `OUTSIDE` whenever this runs.
  set_window_hours(&fixture, 24 * 365 * 100);
  let wide = fixture.facade().todo_view_windowed().expect("wide window");
  assert!(
    wide.contains("ST0001"),
    "and inside a hundred-year one. If this fails, the configured value is not reaching the query -- the default would still have passed the assertion above:\n{wide}"
  );
}

/// **The window reads the DOMAIN date, never the record stamp.**
///
/// Both rows are written to this store in the same instant, so their
/// `created_at` / `updated_at` are indistinguishable; only `completed`
/// differs. **An implementation windowing on record time passes every other
/// test in this file and fails this one** -- and it would be a window onto
/// when someone last ran a command, reported as a window onto when work was
/// done. The store is rebuildable by design (D36), so that reading would reset
/// on every rebuild and show the whole estate as just-finished.
#[test]
fn the_window_reads_when_work_was_completed_not_when_the_row_was_written() {
  let fixture = Fixture::new();
  fixture.write_thread(&done_thread("ST0001", OUTSIDE));
  fixture.write_thread(&done_thread("ST0002", INSIDE));

  let terminal = fixture
    .facade()
    .todo_view_windowed()
    .expect("windowed view");
  assert!(
    terminal.contains("ST0002") && !terminal.contains("ST0001"),
    "both rows were written to this store together and only their completion dates differ, so anything windowing on the WRITE stamp shows both or neither:\n{terminal}"
  );
}

/// A completion the estate never recorded is not silently recent.
///
/// v2 estates carry completed threads with an empty `completed:` line, so this
/// is the shape the migration actually produces rather than a hypothetical.
/// The row is outside every window because there is no date to place it in
/// one -- **absent is not "now"**, which is the direction the mistake would go.
#[test]
fn a_completion_with_no_date_is_not_treated_as_recent() {
  let fixture = Fixture::new();
  let mut undated = done_thread("ST0001", "");
  undated.completed = None;
  fixture.write_thread(&undated);
  fixture.write_thread(&done_thread("ST0002", INSIDE));

  let terminal = fixture
    .facade()
    .todo_view_windowed()
    .expect("windowed view");
  assert!(
    !terminal.contains("ST0001"),
    "a thread with no completion date has nothing to compare, so it cannot be inside a window:\n{terminal}"
  );
  assert!(
    fixture
      .facade()
      .todo_view()
      .expect("file")
      .contains("ST0001"),
    "and it is still in the committed file, because the file is the record"
  );
}

/// **The PROJECTION writes an unwindowed file too, and this test exists
/// because a mutation proved nothing was checking it.**
///
/// `todo.md` has two writers: `Facade::todo_update`, which goes through
/// `todo_view`, and `views::render_all`, which is what a sync writes. The
/// first was covered by the case above; windowing the second passed every test
/// in this file. **A ruling enforced on one of two writers is enforced on
/// neither**, because the uncovered one silently wins whenever it runs last.
#[test]
fn the_projection_that_a_sync_writes_carries_every_completion() {
  let fixture = Fixture::new();
  fixture.write_thread(&done_thread("ST0001", OUTSIDE));
  fixture.write_thread(&done_thread("ST0002", INSIDE));

  let project = fixture.project();
  let facade = fixture.facade();
  let views = intentsvcs::views::render_all(
    &project,
    facade.canon(),
    &intentsvcs::views::RenderContext { version: "3.0.0" },
  );
  let todo = views
    .iter()
    .find(|v| v.path.ends_with("todo.md"))
    .expect("the projection includes todo.md");

  assert!(
    todo.content.contains("ST0001") && todo.content.contains("ST0002"),
    "the projected file must carry every completion, exactly like the one `todo update` writes -- otherwise which writer ran last decides what the repository \
     commits:\n{}",
    todo.content
  );
}

/// **A window the DATA cannot honour is refused by name, not rounded** -- vc's
/// ruling, 2026-08-17, which rejected both spellings offered to them.
///
/// **The failure it prevents is worse than the rounding it was first described
/// as.** The cutoff is `date('now', '-Nh')`, truncated to a date, so at 02:00 a
/// 6-hour window reaches back into yesterday and at 12:00 it does not: the same
/// configuration produces a different DONE bucket depending on the hour it is
/// read at, with nothing on screen to say why. **A value that means one thing
/// in the morning and another in the afternoon is not a rounded value; it is a
/// setting that cannot be reasoned about at all.**
#[test]
fn a_window_the_data_cannot_honour_is_refused_by_name() {
  let fixture = Fixture::new();
  fixture.write_thread(&done_thread("ST0001", INSIDE));
  set_window_hours(&fixture, 6);

  let refused = fixture
    .facade()
    .todo_view_windowed()
    .expect_err("a 6-hour window is not a whole number of days and must be refused");
  let rendered = refused.render();

  assert!(
    rendered.contains('6'),
    "the refusal must name the value that was configured -- an operator cannot act on a rule they have to look up:\n{rendered}"
  );
  assert!(
    rendered.contains("24") && rendered.contains("date"),
    "and it must name the REASON and the two honourable values either side. Refusing without saying why is how a setting becomes folklore:\n{rendered}"
  );
}

/// **The COMMITTED FILE is not held hostage by a display setting**, and this is
/// the assertion that stops the refusal being a worse defect than the rounding.
///
/// `todo.md` is a generated view of the model; the window applies to the
/// terminal alone. A refusal that reached the file would make an unwritable
/// artefact out of a preference about how much scrollback to show.
#[test]
fn a_bad_window_does_not_stop_the_committed_file_being_written() {
  let fixture = Fixture::new();
  fixture.write_thread(&done_thread("ST0001", INSIDE));
  set_window_hours(&fixture, 6);

  let file = fixture
    .facade()
    .todo_view()
    .expect("the unwindowed view never consults the window");
  assert!(
    file.contains("ST0001"),
    "the file is the record and carries every completion regardless of what the terminal is configured to show:\n{file}"
  );
}

/// **Zero is honourable and stays honourable.** It is a whole number of days,
/// it means what it says -- back to the start of today -- and the existing
/// ruling that it is not special-cased into "show everything" is unchanged by
/// adding a refusal beside it.
#[test]
fn a_zero_window_is_a_whole_number_of_days_and_is_not_refused() {
  let fixture = Fixture::new();
  fixture.write_thread(&done_thread("ST0001", OUTSIDE));
  set_window_hours(&fixture, 0);

  let terminal = fixture
    .facade()
    .todo_view_windowed()
    .expect("0 is a whole multiple of 24 and must not be refused");
  assert!(
    !terminal.contains("ST0001"),
    "and it still means what it says rather than being reinterpreted as its opposite:\n{terminal}"
  );
}

/// **THE SELF-RETIREMENT, MEASURED RATHER THAN PROMISED.**
///
/// The reason this guard exists is that `completed` has no time component. vc's
/// ruling turns on the guard becoming unreachable when that changes, rather
/// than becoming something a future reader has to notice and delete -- so the
/// claim is worth more than a comment saying it.
///
/// `check` takes the resolution as a parameter for exactly this: hand it `1`,
/// which is what `COMPLETED_RESOLUTION_HOURS` becomes the day the field gains a
/// time, and observe that the refusal has no reachable input at all.
#[test]
fn the_refusal_retires_itself_when_completed_gains_a_time_component() {
  use intentsvcs::model::COMPLETED_RESOLUTION_HOURS;
  use intentsvcs::project::UnhonourableWindow;

  // Today, at the resolution the data actually has.
  assert!(
    (1..COMPLETED_RESOLUTION_HOURS).any(|h| UnhonourableWindow::check(
      h,
      COMPLETED_RESOLUTION_HOURS
    )
    .is_err()),
    "at today's resolution SOMETHING must be refusable -- if nothing is, the rule below is retiring a guard that never fired and this whole file is decorative"
  );

  // The day `completed` becomes a datetime.
  for hours in 0..=72 {
    assert!(
      UnhonourableWindow::check(hours, 1).is_ok(),
      "a window of {hours}h was refused at a resolution of 1. The guard must become unreachable the moment the data can honour any window, rather than surviving \
       as a rule whose reason has expired"
    );
  }

  // And the `%` stays total across the edit that reaches zero.
  assert!(
    UnhonourableWindow::check(6, 0).is_ok(),
    "a resolution of 0 must refuse nothing rather than panic -- the one edit this code anticipates is an edit to that constant"
  );
}

/// The pure filter, over the cases the id shape makes reachable.
#[test]
fn a_work_package_is_windowed_by_its_parent_thread() {
  let only: TodoWindow = TodoWindow::Only(["ST0002".to_string()].into_iter().collect());

  assert!(only.shows("ST0002"), "the thread itself");
  assert!(
    only.shows("ST0002/03"),
    "and its work packages -- `completed` is a thread-level fact and a WP has no date of its own to window on"
  );
  assert!(!only.shows("ST0001"), "an excluded thread");
  assert!(!only.shows("ST0001/03"), "and its work packages with it");
  assert!(
    !only.shows("ST00021"),
    "a prefix match is not a parent match: ST00021 is a different thread, and a `starts_with` would admit it"
  );

  assert!(
    TodoWindow::All.shows("ST9999"),
    "`All` is not an empty allowlist -- it is every row, and that is what the committed file gets"
  );
}
