//! ST0056 -- `intent organize -v/-q`: the unclaimed inventory moves behind
//! `--verbose`, the record of what the run DOES stays at the default, and the
//! refusal prints at every verbosity.
//!
//! **THE DEFECT, MEASURED BY vc ACROSS THE FLEET 2026-09-07.** `organize`
//! printed dozens of times as many lines as `doctor`. Lamplight alone was most
//! of them, nearly all `unclaimed:` -- one per DIRECTORY, already grouped,
//! under a summary line that already carried the file count AND a digest of the
//! membership. Driven on Lamplight with this change: **the output fell to a
//! small fraction of its length**, and every `to-remove:` line survived, which
//! is the half that must not move.
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
//! with the change REVERTED, because Intent has no unclaimed files: "no
//! `unclaimed:` lines at the default" is satisfied by an estate that has none
//! to print. That is the same shape as vc's `[n/a` fixture passing with its fix
//! reverted, and cc's class change passing the suite -- twice in one afternoon,
//! on different nodes. The rule both of us settled on is cc's phrasing: **state
//! what the test would have to SEE in order to fail, then check the fixture can
//! produce it.** Here that is >1 unclaimed directory AND >0 action lines, and
//! it is asserted rather than assumed.
//!
//! # Mutations, measured -- every assertion below has been SEEN to fail by name
//!
//! The mutations below were each applied to a `cp` snapshot of `render.rs`,
//! each reverted with `cp` and verified byte-identical with `cmp`, and the
//! baseline re-run to green after every one. **`git checkout` is NOT the revert
//! here**: a peer's uncommitted work shares this file, so `git diff` is
//! expected to be non-empty and answers a different question than the one being
//! asked.
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

use std::process::Output;

fn intent(dir: &std::path::Path, args: &[&str]) -> Output {
  crate::common::intent()
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
  // `st start` realises the thread it declares since issue 0079; the two
  // action rows this fixture needs are hydrations, so its views are removed.
  std::fs::remove_dir_all(root.join("intent/st/ST0001")).expect("unrealise ST0001");
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

/// The same fixture after a run that realised it, with the declaration taken
/// away -- so the next reconciliation REMOVES the views it just wrote.
fn project_with_a_removal() -> tempfile::TempDir {
  let dir = project();
  let root = dir.path();
  assert!(
    intent(root, &["organize", "--apply"]).status.success(),
    "the views must exist before a run can remove them"
  );
  // `.intentfiles` is hand-editable by design and nothing regenerates it;
  // declaring nothing is what makes the realised views undeclared.
  std::fs::write(root.join("intent/.intentfiles"), "# declares nothing\n")
    .expect("undeclare everything");
  dir
}

/// **`--quiet` MAY WITHHOLD WHAT A RUN WROTE. IT MAY NOT WITHHOLD WHAT A RUN
/// REMOVED** (hv, 2026-09-12: silent deletion).
///
/// `shows_body` carried the reasoning from the day it was written -- removals
/// are this verb's SUBJECT and must not go behind a flag -- and then `--quiet`
/// arrived and put them behind one anyway, because they shared a predicate with
/// the hydration lines. Driven before the fix: `organize --apply --quiet`
/// printed one summary line, removed two files and pruned a directory, and
/// named none of the three.
#[test]
fn quiet_still_names_every_path_a_run_removes() {
  let dir = project_with_a_removal();
  let said = stdout(&intent(dir.path(), &["organize", "--apply", "--quiet"]));

  let removals: Vec<&str> = said
    .lines()
    .map(str::trim_start)
    .filter(|l| l.starts_with("removed: ") || l.starts_with("pruned: "))
    .collect();
  assert!(
    !removals.is_empty(),
    "a quiet run that removed files must still name them, or the estate loses \
     bytes under a summary line. Got:\n{said}"
  );
  assert!(
    said.contains("to-remove: "),
    "and the plan is still printed BEFORE the act, quiet or not. Got:\n{said}"
  );

  // **THE CONTROL, AND IT IS WHAT KEEPS `--quiet` A FLAG AT ALL**: the lines
  // that are not removals are still withheld.
  assert!(
    !said
      .lines()
      .any(|l| l.trim_start().starts_with("unclaimed: ")),
    "`--quiet` still withholds the inventory. Got:\n{said}"
  );
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

/// A bare project whose v2 `COMPLETED/` bucket holds two files for a thread the
/// store has never heard of, so the v2 prune withholds both and refuses.
///
/// **TWO FILES AND ONE CLASS LINE, SO THE TWO QUANTITIES DIFFER.** A renderer
/// that printed one line per file would print two, and one that printed the
/// class without its count would name no `2`.
fn project_with_unheld_v2_files() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  assert!(
    intent(root, &["init", "Fixture"]).status.success(),
    "the fixture must initialise"
  );
  let bucket = root.join("intent/st/COMPLETED/ST0099");
  std::fs::create_dir_all(&bucket).expect("the v2 bucket");
  for name in ["design.md", "notes.md"] {
    std::fs::write(bucket.join(name), format!("# {name}\n\nNever ingested.\n"))
      .expect("an unheld v2 file");
  }
  dir
}

fn stderr_of(root: &std::path::Path, args: &[&str]) -> String {
  String::from_utf8_lossy(&intent(root, args).stderr).into_owned()
}

/// **0318: THE v2 PRUNE REFUSAL IS ONE CLASS PER RUN, NOT ONE ERROR PER FILE.**
///
/// hv ran `organize` on Laksa and a PREVIEW printed one `error: refusing to
/// prune <file>` line per unheld v2 file, each carrying the same remedy -- an
/// `error:` from a run that changed nothing, repeated until the lines that
/// mattered scrolled away. Ruled: one line carrying the count, the reason and
/// the remedy, at every verbosity; the paths with their reasons only under
/// `--verbose`, with the default announcing that it narrowed; the summary's
/// `refused` count kept.
///
/// **THE CLASS STAYS ON STDERR AT EVERY VERBOSITY AND THE PATHS GO TO STDOUT**,
/// which is what keeps `stderr_does_not_vary_with_verbosity` and
/// `quiet_is_the_summary_alone` true on an estate that has this refusal: the
/// per-path list is inventory, the same kind as the unclaimed directories.
#[test]
fn the_v2_prune_refusal_is_one_class_line_and_verbose_names_each_file() {
  let dir = project_with_unheld_v2_files();
  let root = dir.path();
  let args = |flags: &[&'static str]| {
    let mut a = vec!["organize"];
    a.extend_from_slice(flags);
    a
  };

  let default_out = run(root, &[]);
  let default_err = stderr_of(root, &args(&[]));
  assert!(
    summary(&default_out).ends_with(", 2 refused"),
    "the positive control: the fixture produces the refusal, and the summary still counts each file. Got:\n{default_out}"
  );
  let class: Vec<&str> = default_err
    .lines()
    .filter(|l| l.contains("prune the v2 tree"))
    .collect();
  assert_eq!(
    class.len(),
    1,
    "one class line for the whole run. stderr:\n{default_err}"
  );
  assert!(
    class[0].contains("would refuse")
      && class[0].contains("2 file(s)")
      && !class[0].starts_with("error:"),
    "a preview says what the apply WOULD refuse, names the count, and is not an `error:`. Got: {}",
    class[0]
  );
  assert!(
    !default_err.contains("design.md") && !default_err.contains("notes.md"),
    "no per-file refusal on stderr. stderr:\n{default_err}"
  );
  assert_eq!(
    default_err
      .lines()
      .filter(|l| l.trim_start().starts_with("remedy:"))
      .count(),
    1,
    "one remedy, not one per file. stderr:\n{default_err}"
  );
  assert!(
    !default_out.contains("design.md") && !default_out.contains("notes.md"),
    "the default does not list the files. stdout:\n{default_out}"
  );
  assert!(
    default_out
      .lines()
      .any(|l| l.contains("2 file(s) not listed") && l.contains("--verbose")),
    "and says that it narrowed, with the figure. stdout:\n{default_out}"
  );

  let verbose_out = run(root, &["--verbose"]);
  for name in ["design.md", "notes.md"] {
    assert!(
      verbose_out
        .lines()
        .any(|l| l.contains(&format!("intent/st/COMPLETED/ST0099/{name} -- "))),
      "`--verbose` names {name} with its reason. stdout:\n{verbose_out}"
    );
  }

  assert_eq!(
    stderr_of(root, &args(&["--verbose"])),
    default_err,
    "the class line and its remedy do not vary with `--verbose`"
  );
  assert_eq!(
    stderr_of(root, &args(&["--quiet"])),
    default_err,
    "nor with `--quiet`"
  );
}

/// **THE APPLY SAYS WHAT IT REFUSED, IN THE PAST TENSE, AS AN ERROR, AND EXITS
/// NON-ZERO.** An apply renders twice -- the plan immediately before the act,
/// in the future tense, and then the act -- so its stderr legitimately carries
/// one `would refuse` from the plan. What this pins is the act's own line:
/// exactly one `error: refused`, with the count, and no per-file refusal from
/// either render.
#[test]
fn an_apply_reports_the_v2_prune_refusal_once_as_an_error() {
  let dir = project_with_unheld_v2_files();
  let out = intent(dir.path(), &["organize", "--apply"]);
  let err = String::from_utf8_lossy(&out.stderr).into_owned();
  let refused: Vec<&str> = err
    .lines()
    .filter(|l| l.starts_with("error: refused to prune the v2 tree"))
    .collect();
  assert_eq!(
    refused.len(),
    1,
    "one past-tense class line for the act. stderr:\n{err}"
  );
  assert!(
    refused[0].contains("2 file(s)"),
    "with the count. Got: {}",
    refused[0]
  );
  assert!(
    !err.contains("refusing to prune"),
    "and no per-file refusal, from the plan printed before the act or from the act. stderr:\n{err}"
  );
  assert!(
    !out.status.success(),
    "a refused removal moves the exit code on the apply path"
  );
  assert!(
    dir
      .path()
      .join("intent/st/COMPLETED/ST0099/design.md")
      .is_file(),
    "and nothing was removed"
  );
}
