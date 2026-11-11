//! ST0092 WP-04 bank (i) at the process surface: `intent wb retire`, `wb
//! status --all`, `wb show` of a retired node, a re-apply after a retirement,
//! and `wb init <base> --change`, driven through the binary and read as a
//! caller reads them. The typed arms in `intentsvcs/tests/wb_retire.rs` are
//! the precise half; this file compiles on a base with no retirement, where
//! each arm is red at runtime.

use std::path::Path;

use intentsvcs::teams::Teams;

const REASON: &str = "the hub's work moved to its successor";

fn run(cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn ok(cwd: &Path, args: &[&str]) -> String {
  let (out, err, code) = run(cwd, args);
  assert_eq!(code, 0, "`intent {}`: {out}{err}", args.join(" "));
  out
}

/// A project with `profile` applied.
fn applied(name: &str, profile: &str) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  ok(dir.path(), &["init", name]);
  ok(dir.path(), &["wb", "init", profile, "--apply"]);
  dir
}

fn resolved(profile: &str) -> intentsvcs::teams::Resolved {
  Teams::new(&testkit::repo_root())
    .resolve(&[profile.to_string()], None, false)
    .expect("the profile resolves")
}

/// A declared seat that is neither the mediator nor the human seat.
fn retirable(profile: &str) -> String {
  let r = resolved(profile);
  r.seats
    .iter()
    .map(|s| s.moniker.clone())
    .find(|m| *m != r.mediator && *m != r.human)
    .expect("a seat a retirement may take")
}

/// The monikers `wb status --json` lists, with or without `--all`.
fn listed(cwd: &Path, all: bool) -> Vec<String> {
  let mut args = vec!["wb", "status", "--json"];
  if all {
    args.push("--all");
  }
  let rows: serde_json::Value = serde_json::from_str(&ok(cwd, &args)).expect("status --json");
  rows
    .as_array()
    .expect("an array of rows")
    .iter()
    .filter_map(|r| r["moniker"].as_str().map(str::to_string))
    .collect()
}

/// ST0092 AT-04.1 at the surface: `wb retire` retires a node, `wb status`
/// lists it only with `--all`, and `wb show` reads it without `--all` and
/// says it is retired.
#[test]
fn at_04_1_a_retired_node_is_listed_only_with_all_and_shown_as_retired() {
  let dir = applied("wbretiredoor", "team");
  let seat = retirable("team");
  let out = ok(dir.path(), &["wb", "retire", &seat, "--reason", REASON]);
  assert!(out.contains(&format!("ok: {seat} retired")), "{out}");

  assert!(
    !listed(dir.path(), false).contains(&seat),
    "listed without --all"
  );
  assert!(
    listed(dir.path(), true).contains(&seat),
    "not listed with --all"
  );
  let shown = ok(dir.path(), &["wb", "show", &seat]);
  assert!(
    shown.contains("retired") && shown.contains(REASON),
    "{shown}"
  );

  let again = ok(dir.path(), &["wb", "retire", &seat, "--reason", REASON]);
  assert!(
    again.contains(&format!("ok: {seat} was already retired")),
    "{again}"
  );
}

/// ST0092 AT-04.2 at the surface: a retirement with no reason, of the
/// mediator or of the human seat refuses at exit 1 with a remedy, and writes
/// nothing.
#[test]
fn at_04_2_a_retirement_without_a_reason_or_of_the_mediator_or_human_refuses() {
  let dir = applied("wbretirerefuses", "team");
  let r = resolved("team");
  for (moniker, reason) in [
    (retirable("team"), " ".to_string()),
    (r.mediator.clone(), REASON.to_string()),
    (r.human.clone(), REASON.to_string()),
  ] {
    let (out, err, code) = run(dir.path(), &["wb", "retire", &moniker, "--reason", &reason]);
    assert_eq!(code, 1, "`wb retire {moniker}`: {out}{err}");
    assert!(err.contains("error:") && err.contains("remedy:"), "{err}");
  }
  assert!(
    listed(dir.path(), false).contains(&r.mediator),
    "a refusal retired the mediator"
  );
}

/// ST0092 AT-04.5 at the surface: `wb register` of a retired moniker with its
/// profile's name and role reinstates it.
#[test]
fn at_04_5_register_reinstates_a_retired_node() {
  let dir = applied("wbretirereinstate", "team");
  let seat = retirable("team");
  let declared = resolved("team")
    .seats
    .into_iter()
    .find(|s| s.moniker == seat)
    .expect("the seat");
  ok(dir.path(), &["wb", "retire", &seat, "--reason", REASON]);
  ok(
    dir.path(),
    &[
      "wb",
      "register",
      &seat,
      "--name",
      &declared.name,
      "--role",
      &declared.role,
    ],
  );
  assert!(
    listed(dir.path(), false).contains(&seat),
    "the reinstated node is not listed"
  );
}

/// ST0092 AT-04.6 at the surface: `wb init <base> --change` lists the seats
/// it retires in its dry run, and retires them on `--apply`.
#[test]
fn at_04_6_a_change_of_base_lists_and_retires_the_old_bases_seats() {
  let dir = applied("wbretirechange", "solo");
  let team = resolved("team");
  let leaving: Vec<String> = resolved("solo")
    .seats
    .into_iter()
    .map(|s| s.moniker)
    .filter(|m| !team.seats.iter().any(|s| &s.moniker == m))
    .collect();
  assert!(
    !leaving.is_empty(),
    "the fixture needs a seat solo declares and team does not"
  );

  let plan = ok(dir.path(), &["wb", "init", "team", "--change"]);
  for m in &leaving {
    assert!(
      plan.contains(&format!("retire     {m} ")),
      "the dry run does not list {m}: {plan}"
    );
  }
  ok(dir.path(), &["wb", "init", "team", "--change", "--apply"]);
  let active = listed(dir.path(), false);
  let everyone = listed(dir.path(), true);
  for m in &leaving {
    assert!(
      !active.contains(m) && everyone.contains(m),
      "{m} was not retired"
    );
  }
}

/// vc's ruling (2) at the surface: a re-apply after a retirement leaves the
/// seat retired, writes nothing, and its dry run prints the retired line.
#[test]
fn a_reapply_after_a_retirement_prints_it_and_writes_nothing() {
  let dir = applied("wbretirereapply", "team");
  let seat = retirable("team");
  ok(dir.path(), &["wb", "retire", &seat, "--reason", REASON]);
  let plan = ok(dir.path(), &["wb", "init", "team"]);
  assert!(
    plan.contains(&format!("retired    {seat} ({REASON})")),
    "the dry run does not print the retired line: {plan}"
  );
  let out = ok(dir.path(), &["wb", "init", "team", "--apply"]);
  assert!(!out.contains("\nwrote: "), "the re-apply wrote: {out}");
  assert!(
    !listed(dir.path(), false).contains(&seat),
    "the re-apply reinstated {seat}"
  );
}

/// ST0092 AT-04.7 and AT-04.8 at the surface (bank (ii)): an ask to a retired
/// node refuses at exit 1 naming the retirement, and the sender's pickup names
/// the message the leaver never handled with the archive that takes it, which
/// then runs as printed.
#[test]
fn at_04_7_and_8_an_ask_to_a_retired_node_refuses_and_pickup_names_the_stranded_message() {
  let dir = applied("wbretirerecipient", "team");
  let seat = retirable("team");
  let mediator = resolved("team").mediator;
  ok(
    dir.path(),
    &["wb", "ask", &seat, "one it never read", "--node", &mediator],
  );
  ok(dir.path(), &["wb", "retire", &seat, "--reason", REASON]);

  let (out, err, code) = run(
    dir.path(),
    &["wb", "ask", &seat, "are you there?", "--node", &mediator],
  );
  assert_eq!(code, 1, "an ask to a retired node: {out}{err}");
  assert!(
    err.contains("retired") && err.contains(REASON) && err.contains("remedy:"),
    "{err}"
  );

  let up = ok(dir.path(), &["wb", "pickup", "--node", &mediator]);
  let line = up
    .lines()
    .find(|l| l.starts_with("stranded: ") && l.contains(&seat))
    .unwrap_or_else(|| panic!("the pickup names no stranded message: {up}"));
  let archive = line
    .split('`')
    .nth(1)
    .expect("the line carries the archive command");
  let args: Vec<String> = shell_words(archive);
  assert_eq!(
    args.first().map(String::as_str),
    Some("intent"),
    "{archive}"
  );
  let argv: Vec<&str> = args[1..].iter().map(String::as_str).collect();
  ok(dir.path(), &argv);
  let again = ok(dir.path(), &["wb", "pickup", "--node", &mediator]);
  assert!(!again.contains("stranded: "), "{again}");
}

/// Split a printed command as a shell would, for the one quoting form the
/// pickup line uses: double quotes around the anchor.
fn shell_words(cmd: &str) -> Vec<String> {
  let mut words = Vec::new();
  let mut word = String::new();
  let mut quoted = false;
  for c in cmd.chars() {
    match c {
      '"' => quoted = !quoted,
      ' ' if !quoted => {
        if !word.is_empty() {
          words.push(std::mem::take(&mut word));
        }
      }
      _ => word.push(c),
    }
  }
  if !word.is_empty() {
    words.push(word);
  }
  words
}
