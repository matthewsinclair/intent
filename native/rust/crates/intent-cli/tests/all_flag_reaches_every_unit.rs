//! `--all` on `claude skills` and `claude subagents` (issue 0236).
//!
//! # This file is the flag's ONLY protection, and that is a measured claim
//!
//! `flag_reachability::every_declared_flag_on_a_wired_family_is_read_by_the_renderer`
//! looks like it covers this. **It cannot.** A violation there requires BOTH no
//! accessor site AND no mention of the id anywhere in the renderer -- a
//! deliberately conservative gate whose own comment names the price: *a flag
//! whose id happens to appear as an unrelated string passes*. `render.rs`
//! spells `"all"` twice for reasons that have nothing to do with this flag (a
//! status filter at `:703`, an issue-status arm at `:6594`), so `--all` is in
//! that check's SHIELDED bucket by construction and **no removal of its read
//! can ever red it**. Driven 2026-09-04: with `--all` declared on both rows and
//! the renderer reading it nowhere, that check passed.
//!
//! So the flag could be declared and inert, and only these arms would say so.
//!
//! # Why the flag exists at all
//!
//! **A v2-to-v3 REGRESSION, not a v3 feature.** v2 shipped it working;
//! `usage-rules.md` instructs `install --all` at four sites. v3 declared it on
//! no row, so clap refused it at **rc=1 `unexpected argument`** -- which tells
//! an operator they mistyped the thing root canon told them to type. rc=2
//! *known command, not implemented yet* is a coherent shipping state and is
//! what `config`/`ext`/`learn` ship as; rc=1 is not.
//!
//! # The two verbs resolve `--all` against DIFFERENT populations
//!
//! `install --all` means everything this install CARRIES (`available()`);
//! `uninstall --all` means everything it has INSTALLED (`installed()`). The
//! asymmetry is load-bearing and is asserted below rather than assumed: against
//! one population `uninstall --all` would try to remove units never installed,
//! and against the other `install --all` would be a no-op on a fresh machine --
//! the one machine the flag exists for.

use std::path::Path;
use std::process::Command;

fn run(home: &Path, args: &[&str]) -> (i32, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .env("HOME", home)
    .output()
    .expect("spawn intent");
  let mut text = String::from_utf8_lossy(&out.stdout).to_string();
  text.push_str(&String::from_utf8_lossy(&out.stderr));
  (out.status.code().unwrap_or(-1), text)
}

/// `list` prints `<name> <state> <provenance>`; a unit that is not installed
/// carries `-`. Counting the two states separately rather than one total,
/// because a single number cannot say which way an arm failed.
fn states(listing: &str) -> (usize, usize) {
  let mut installed = 0;
  let mut absent = 0;
  for line in listing.lines() {
    let mut cols = line.split_whitespace();
    let (Some(_name), Some(state)) = (cols.next(), cols.next()) else {
      continue;
    };
    match state {
      "installed" => installed += 1,
      "-" => absent += 1,
      _ => {}
    }
  }
  (installed, absent)
}

#[test]
fn install_all_reaches_every_unit_the_install_carries() {
  for family in ["skills", "subagents"] {
    let home = tempfile::tempdir().expect("temp HOME");
    let h = home.path();

    let (rc, before) = run(h, &["claude", family, "list"]);
    assert_eq!(rc, 0, "`claude {family} list` failed: {before}");
    let (installed_before, absent_before) = states(&before);
    // The fixture must be able to exhibit the change. A HOME with nothing to
    // install would pass this test whatever `--all` did.
    assert!(
      absent_before > 0,
      "fixture cannot exhibit the change: no uninstalled {family} in a fresh HOME\n{before}"
    );
    assert_eq!(
      installed_before, 0,
      "a fresh HOME already had {family} installed\n{before}"
    );

    let (rc, out) = run(h, &["claude", family, "install", "--all"]);
    assert_eq!(rc, 0, "`claude {family} install --all` failed:\n{out}");

    let (rc, after) = run(h, &["claude", family, "list"]);
    assert_eq!(rc, 0, "list after install failed: {after}");
    let (installed_after, absent_after) = states(&after);
    assert_eq!(
      absent_after, 0,
      "`--all` left {absent_after} {family} uninstalled, so it did not reach every unit\n{after}"
    );
    assert_eq!(
      installed_after, absent_before,
      "`--all` installed {installed_after} of the {absent_before} {family} this install carries\n{after}"
    );
  }
}

#[test]
fn uninstall_all_removes_every_unit_that_is_installed() {
  for family in ["skills", "subagents"] {
    let home = tempfile::tempdir().expect("temp HOME");
    let h = home.path();

    let (rc, out) = run(h, &["claude", family, "install", "--all"]);
    assert_eq!(rc, 0, "setup install failed:\n{out}");
    let (_, listed) = run(h, &["claude", family, "list"]);
    let (installed, _) = states(&listed);
    assert!(installed > 0, "setup did not install anything\n{listed}");

    let (rc, out) = run(h, &["claude", family, "uninstall", "--all"]);
    assert_eq!(rc, 0, "`claude {family} uninstall --all` failed:\n{out}");

    let (_, after) = run(h, &["claude", family, "list"]);
    let (still, _) = states(&after);
    assert_eq!(
      still, 0,
      "`uninstall --all` left {still} {family} installed\n{after}"
    );
  }
}

#[test]
fn all_and_a_named_unit_are_refused_rather_than_silently_resolved() {
  let home = tempfile::tempdir().expect("temp HOME");
  let (rc, out) = run(
    home.path(),
    &["claude", "skills", "install", "--all", "in-session"],
  );
  assert_ne!(
    rc, 0,
    "naming a unit AND `--all` succeeded, so one of them was silently dropped:\n{out}"
  );
  assert!(
    out.contains("mutually exclusive"),
    "the refusal did not say the two are mutually exclusive:\n{out}"
  );
}

#[test]
fn all_is_refused_on_sync_where_it_would_name_the_same_set_twice() {
  let home = tempfile::tempdir().expect("temp HOME");
  let (rc, out) = run(home.path(), &["claude", "skills", "sync", "--all"]);
  assert_ne!(
    rc, 0,
    "`sync --all` succeeded; the flag should be refused there:\n{out}"
  );
  assert!(
    out.contains("not available on `sync`"),
    "the refusal did not name `sync` as the reason:\n{out}"
  );
}
