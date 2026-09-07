//! ST0056 -- `intent organize -v/-q`: the unclaimed inventory moves behind
//! `--verbose`, the record of what the run DOES stays at the default, and the
//! refusal prints at every verbosity.
//!
//! **THE DEFECT, MEASURED BY vc ACROSS THE FLEET 2026-09-07.** `organize`
//! printed ~3155 lines against `doctor`'s ~95. Lamplight alone was 2100, of
//! which 2072 were `unclaimed:` -- one per DIRECTORY, already grouped, under a
//! summary line that already carried the file count AND a digest of the
//! membership. Driven on Lamplight with this change: **2098 -> 28**, and the
//! 25 `to-remove:` lines survived, which is the half that must not move.
//!
//! **THIS IS A CLI TEST BECAUSE THE SUBJECT IS A RENDERING.** The report is
//! unchanged -- `organize::Report` carries exactly what it carried -- so a
//! facade-level test cannot see this change at all. What moved is which of the
//! report's rows reach a terminal, and only the binary decides that.
//!
//! # The fixture has to be able to fail, and one test asserts that it can
//!
//! **`the_fixture_can_exhibit_what_the_others_measure` IS NOT CEREMONY.** Run
//! these assertions against Intent's own estate and every one of them passes
//! with the change REVERTED, because Intent has 0 unclaimed files: "no
//! `unclaimed:` lines at the default" is satisfied by an estate that has none
//! to print. That is the same shape as vc's `[n/a` fixture passing with its fix
//! reverted, and cc's class change passing 1210 tests -- twice in one afternoon,
//! two nodes. The rule both of us settled on is cc's phrasing: **state what the
//! test would have to SEE in order to fail, then check the fixture can produce
//! it.** Here that is >1 unclaimed directory AND >0 action lines, and it is
//! asserted rather than assumed.
//!
//! # Mutations, measured -- every assertion below has been SEEN to fail by name
//!
//! Seven mutations, each applied to a `cp` snapshot of `render.rs`, each
//! reverted with `cp` and verified byte-identical with `cmp`, and the baseline
//! re-run to green after every one. **`git checkout` is NOT the revert here**:
//! a peer's uncommitted work shares this file, so `git diff` is expected to be
//! non-empty and answers a different question than the one being asked.
//!
//! | mutation                                              | reds                                                        |
//! | ----------------------------------------------------- | ----------------------------------------------------------- |
//! | `shows_inventory` returns `true` always               | `quiet_is_the_summary_alone`, `the_default_...`, `verbose_...` |
//! | `shows_inventory` returns `false` always              | `the_fixture_can_exhibit_...`, `verbose_lists_...`          |
//! | `shows_body` returns `true` always                    | `quiet_is_the_summary_alone`                                |
//! | `shows_body` returns `false` always                   | `the_default_...`, `the_fixture_...`, `verbose_lists_...`   |
//! | `Verbosity::of` resolves `(true, true)` to `Verbose`  | `quiet_wins_over_verbose`                                   |
//! | the withheld line prints `by_dir.len() + 1`           | `verbose_lists_what_the_default_withheld`                   |
//! | the withheld line prints the ACTION-row count         | `verbose_lists_what_the_default_withheld`                   |
//!
//! **THE LAST ROW IS THE ONE WORTH READING, AND IT IS vc's CLASS RATHER THAN A
//! SEVENTH IDEA.** This fixture first had 2 unclaimed directories and 2 action
//! rows, so a withheld line printing the wrong quantity read `2 == 2` and
//! stayed green. vc hit the identical shape an hour earlier on their own
//! withheld-count arm -- findings, threads and admit-calls all equalled 2 on
//! their fixture, and a counter broken from `+= found.len()` to `+= 1` still
//! reached the expected number. The fixture is now 3 / 5 / 2, pairwise
//! distinct, and that mutation reds. **A control is worth exactly the fixture's
//! ability to tell the mutated quantity from its neighbours**, which is a
//! sharper rule than "positive-control the instrument" and is the one that
//! caught this.

use std::process::{Command, Output};

fn intent(dir: &std::path::Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .stdin(std::process::Stdio::null())
    .output()
    .expect("run the v3 binary")
}

/// One WIP thread that is DECLARED and not realised (so the preview carries
/// `to-hydrate:` action lines), plus unclaimed files in two directories.
///
/// **`.tap` IS THE EXTENSION BECAUSE IT CANNOT BECOME ANYTHING ELSE.** An
/// unclaimed path is one the renderer cannot produce and the store does not
/// carry: `.tap` is not a view and not an attachment (`ATTACHMENT_EXTENSIONS`
/// is md/txt/sh), so these files stay unclaimed no matter what else the fixture
/// does. A `.md` here would be an attachment and the fixture would silently
/// stop testing the thing.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  assert!(
    intent(root, &["init", "Fixture"]).status.success(),
    "the fixture must initialise"
  );
  assert!(
    intent(root, &["st", "new", "An open thread"])
      .status
      .success(),
    "st new must succeed"
  );
  assert!(
    intent(root, &["st", "start", "ST0001"]).status.success(),
    "the thread must reach WIP -- only WIP threads are declared, so a triage \
     thread would leave the manifest empty and the preview with no action lines"
  );
  assert!(
    intent(root, &["organize", "--default"]).status.success(),
    "the declaration must be written, or there is nothing to hydrate"
  );
  // **THE THREE QUANTITIES ARE PAIRWISE DISTINCT ON PURPOSE: 3 DIRECTORIES, 5
  // FILES, 2 ACTION ROWS.** vc's correction, 2026-09-07, from an arm of their
  // own that stayed green under a real mutation: they asserted `2 withheld` on
  // a fixture where findings, threads and admit-calls all happened to equal 2,
  // so a counter broken from `+= found.len()` to `+= 1` still reached the
  // expected number. **A control is only worth the fixture's ability to tell
  // the mutated quantity from its NEIGHBOURS.** With 2 directories and 2 action
  // rows -- which is what this fixture had first -- a withheld line that
  // printed the action count instead of the directory count would have been
  // invisible. `the_fixture_can_exhibit_what_the_others_measure` asserts the
  // distinctness rather than leaving it to this comment.
  for (sub, name) in [
    ("parity", "a.tap"),
    ("parity", "b.tap"),
    ("logs", "c.tap"),
    ("logs", "d.tap"),
    ("fixtures", "e.tap"),
  ] {
    let d = root.join("intent/st/ST0001").join(sub);
    std::fs::create_dir_all(&d).expect("fixture directory");
    std::fs::write(d.join(name), "x\n").expect("fixture file");
  }
  dir
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

fn run(root: &std::path::Path, flags: &[&str]) -> String {
  let mut args = vec!["organize"];
  args.extend_from_slice(flags);
  stdout(&intent(root, &args))
}

/// The per-directory inventory rows, which carry a `/ (N file(s))` tail.
fn inventory_rows(said: &str) -> Vec<&str> {
  said
    .lines()
    .filter(|l| l.trim_start().starts_with("unclaimed: ") && l.contains(" file(s))"))
    .collect()
}

/// The rows recording what the run would do -- the half that must NOT move.
fn action_rows(said: &str) -> Vec<&str> {
  said
    .lines()
    .filter(|l| {
      let t = l.trim_start();
      t.starts_with("to-hydrate:")
        || t.starts_with("to-rewrite:")
        || t.starts_with("to-remove:")
        || t.starts_with("to-prune:")
    })
    .collect()
}

fn summary(said: &str) -> &str {
  said
    .lines()
    .find(|l| l.starts_with("organize (preview):"))
    .unwrap_or_else(|| panic!("no preview summary line in:\n{said}"))
}

/// The figure the withheld line claims, or `None` when no such line printed.
fn withheld(said: &str) -> Option<usize> {
  let line = said.lines().find(|l| l.contains("not listed --"))?;
  line
    .split_whitespace()
    .find_map(|w| w.parse::<usize>().ok())
}

/// **THE POSITIVE CONTROL. Everything else in this file is vacuous without it.**
#[test]
fn the_fixture_can_exhibit_what_the_others_measure() {
  let dir = project();
  let verbose = run(dir.path(), &["--verbose"]);
  assert!(
    inventory_rows(&verbose).len() > 1,
    "the fixture must produce unclaimed files across MORE THAN ONE directory, or \
     `the_default_withholds_only_the_inventory` passes on an estate with nothing \
     to withhold. Got:\n{verbose}"
  );
  assert!(
    !action_rows(&verbose).is_empty(),
    "the fixture must produce at least one action row, or `shows_body` is never \
     exercised and a mutation returning `false` would go unnoticed. Got:\n{verbose}"
  );
  // **AND THE THREE QUANTITIES MUST DIFFER FROM EACH OTHER**, or an assertion
  // comparing against the wrong one still reads green. This is the arm vc's
  // 2026-09-07 correction adds: their own withheld-count test survived a real
  // mutation because findings, threads and admit-calls all equalled 2 on its
  // fixture, and nothing about the green looked wrong.
  let dirs = inventory_rows(&verbose).len();
  let files: usize = inventory_rows(&verbose)
    .iter()
    .filter_map(|l| l.split_once(" (")?.1.split_once(" file").map(|(n, _)| n))
    .filter_map(|n| n.parse::<usize>().ok())
    .sum();
  let actions = action_rows(&verbose).len();
  assert!(
    dirs != files && dirs != actions && files != actions,
    "the fixture's three quantities must be pairwise distinct so an assertion \
     against the wrong one cannot pass: dirs={dirs}, files={files}, actions={actions}"
  );
}

#[test]
fn the_default_withholds_only_the_inventory() {
  let dir = project();
  let said = run(dir.path(), &[]);
  assert!(
    inventory_rows(&said).is_empty(),
    "the default must not list the unclaimed inventory. Got:\n{said}"
  );
  assert!(
    !action_rows(&said).is_empty(),
    "**THE ACTION ROWS STAY AT THE DEFAULT.** They are this verb's subject: \
     `--apply` exists because a build once advertised its destructive half in \
     its own help and did not show it. Got:\n{said}"
  );
  assert!(
    withheld(&said).is_some(),
    "a narrowing must announce itself -- `0 unclaimed` over a full listing is \
     byte-identical to `0 unclaimed` over a suppressed one. Got:\n{said}"
  );
}

#[test]
fn verbose_lists_what_the_default_withheld() {
  let dir = project();
  let default = run(dir.path(), &[]);
  let verbose = run(dir.path(), &["--verbose"]);
  assert_eq!(
    withheld(&default),
    Some(inventory_rows(&verbose).len()),
    "**THE WITHHELD FIGURE AND THE LISTING MUST RECONCILE.** A pointer naming \
     the wrong number is worse than no pointer: it reads as a denominator and \
     is not one.\ndefault:\n{default}\nverbose:\n{verbose}"
  );
  assert_eq!(
    summary(&default),
    summary(&verbose),
    "the summary is the one line that does not vary with verbosity"
  );
}

#[test]
fn quiet_is_the_summary_alone() {
  let dir = project();
  let said = run(dir.path(), &["--quiet"]);
  assert_eq!(
    said.lines().count(),
    1,
    "`--quiet` is the summary and nothing else on stdout. Got:\n{said}"
  );
  assert_eq!(
    summary(&said),
    summary(&run(dir.path(), &[])),
    "**THE SUMMARY SURVIVES `--quiet`.** A quiet clean run printing nothing at \
     all is indistinguishable from the command never having run."
  );
}

#[test]
fn quiet_wins_over_verbose() {
  let dir = project();
  assert_eq!(
    run(dir.path(), &["--quiet", "--verbose"]),
    run(dir.path(), &["--quiet"]),
    "**QUIET WINS**, which is the rule `doctor` already applies. Two sibling \
     verbs resolving one flag pair two ways is the surface Highlander problem."
  );
}

/// **A REFUSAL IS NOT A VERBOSITY-LEVEL THING.** It moves the exit code on the
/// apply path, so a flag that could hide one would hide the single line an
/// operator most needs. Driven on Lamplight: the refusal on stderr is
/// BYTE-IDENTICAL under default, `--verbose` and `--quiet`.
///
/// **WHAT THIS FIXTURE CAN AND CANNOT DO, SAID RATHER THAN LEFT TO BE FOUND.**
/// It asserts stderr is identical across the three, which on a fixture with no
/// refusal is three empty strings -- true, and weak. The real instance is
/// measured on a live estate and recorded above rather than claimed here, and
/// the assertion that would need a divergent-file fixture is NOT written: a
/// test whose green depends on a condition the fixture cannot create is the
/// thing the positive control above exists to prevent.
#[test]
fn stderr_does_not_vary_with_verbosity() {
  let dir = project();
  let err = |f: &[&str]| {
    let mut args = vec!["organize"];
    args.extend_from_slice(f);
    String::from_utf8_lossy(&intent(dir.path(), &args).stderr).into_owned()
  };
  assert_eq!(
    err(&[]),
    err(&["--quiet"]),
    "`--quiet` must not touch stderr"
  );
  assert_eq!(
    err(&[]),
    err(&["--verbose"]),
    "`--verbose` must not touch stderr"
  );
}
