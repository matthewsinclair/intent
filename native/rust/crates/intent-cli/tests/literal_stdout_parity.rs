//! **The `observed` column is v2's bytes, and until now nothing ever read it back.**
//!
//! An `as-observed` row is a claim that v3's stdout equals v2's. 62 rows make
//! that claim and **the claim is delegated to prose** -- `observed.stdout`, a
//! sentence written by whoever read v2 at the time. Nothing anywhere compares
//! that sentence to v2, and nothing compares it to v3.
//!
//! **On 2026-08-17 the only subset anyone checked came back 4 wrong out of 8.**
//! Three had dropped a suffix -- `ok: <AC> back in scope` where v2 prints `ok:
//! <AC> back in scope (unsatisfied)`, and the dropped word is the whole question
//! a reinstate raises. **The fourth was `at na`, carrying `n-a`: v3's own token,
//! in the column that records v2.** That row is `as-observed`, so a check
//! comparing v3 against that column would have found agreement and **certified
//! issue 0056 as correct parity.** The defect reached the register built to
//! catch it, before anyone knew there was a defect.
//!
//! **So the point of this file is not that one requirement should have one
//! statement. It is that a field a test EXECUTES gets measured, and measuring is
//! what found the four.** Prose drifts because nothing reads it back.
//!
//! **`at na` IS EXPECTED TO FAIL HERE AND THAT IS THE FILE WORKING.** Its
//! template is v2's `n/a`; HEAD prints `n-a`; issue 0056 is open with the fix
//! designed. A green suite here would mean the template had been written from
//! v3 again.
//!
//! **What this file does NOT cover, stated so its green is not over-read.** Only
//! nine rows carry `stdout_exact` -- the `ac`/`at`/`issues` literals. The other
//! ~53 `as-observed` rows are prose (`the table`, `the info.md contents`) and
//! cannot be asserted this way; their coverage question is the differential,
//! AT-00.1, and it is red. **Nine of 62 is not parity coverage and this file
//! must never be cited as though it were.**
//!
//! A `corrected` row is deliberately ineligible: v2's bytes are not its target.
//! `at green` was in the first draft of the declaration and the generator arm
//! refused it on exactly that ground -- v3 drops v2's green-only-from-red
//! ladder, so a template carrying v2's bytes would have asserted the ratified
//! deviation away. Same shape as `at na`, running the other direction.

use std::path::Path;
use std::process::Command;

use intent_cli::dispatch;

/// The fixture every declaration is written against: one thread with one
/// authored criterion, one test-backed criterion, a test AT and a non-test AT,
/// a sink thread to descope into, and one open issue.
///
/// **Entity IDs match the ones the v2 templates were measured with**, which is
/// the only reason a template can be a literal rather than a pattern.
///
/// **AT-01.2 is seeded `to-write`, not `n-a`, and the first draft had it wrong.**
/// Seeded at its target the `at na` declaration measures a SELF-LOOP -- v3
/// answered `ok: AT-01.2 already n-a` and the run reported a parity break whose
/// real cause was the fixture. A template describes a MOVEMENT, so the fixture
/// must leave the verb somewhere to go. Same trap as cc's witness reading state
/// before the verb ran, one field over.
fn seed(root: &Path) {
  let cfg = root.join("intent/.config");
  std::fs::create_dir_all(&cfg).expect("mkdir config");
  std::fs::write(
    cfg.join("config.json"),
    r#"{ "intent_version": "3.0.0", "project_name": "Parity", "author": "ic",
  "intent_dir": "intent", "languages": ["rust"] }
"#,
  )
  .expect("write config");

  std::fs::create_dir_all(root.join("intent/st/ST0001")).expect("mkdir st1");
  std::fs::write(
    root.join("intent/st/ST0001/thread.json"),
    r#"{
  "schema": "intent/thread@3.0", "id": "ST0001", "slug": "probe",
  "title": "Probe", "status": "wip", "created": "2026-08-17",
  "objective": "", "context": "",
  "wps": [ { "seq": 1, "title": "W", "scope": "S", "status": "wip" } ],
  "criteria": [
    { "id": "AC-01.1", "text": "Authored", "kind": "non-test",
      "state": { "is": "unsatisfied" } },
    { "id": "AC-01.2", "text": "Test-backed", "kind": "test",
      "state": { "is": "computed" } }
  ],
  "tests": [
    { "id": "AT-01.1", "covers": ["AC-01.2"], "kind": "test", "status": "to-write" },
    { "id": "AT-01.2", "covers": ["AC-01.1"], "kind": "non-test", "status": "to-write" }
  ]
}
"#,
  )
  .expect("write thread");

  std::fs::create_dir_all(root.join("intent/st/ST0002")).expect("mkdir st2");
  std::fs::write(
    root.join("intent/st/ST0002/thread.json"),
    r#"{ "schema": "intent/thread@3.0", "id": "ST0002", "slug": "sink",
  "title": "Sink", "status": "wip", "created": "2026-08-17",
  "objective": "", "context": "", "wps": [], "criteria": [], "tests": [] }
"#,
  )
  .expect("write sink");

  std::fs::create_dir_all(root.join("intent/issues")).expect("mkdir issues");
  std::fs::write(
    root.join("intent/issues/0001.json"),
    r#"{ "schema": "intent/issue@3.0", "number": 1, "slug": "probe",
  "title": "Probe issue", "status": "open", "created": "2026-08-17",
  "severity": "low" }
"#,
  )
  .expect("write issue");
}

fn bin() -> &'static str {
  env!("CARGO_BIN_EXE_intent")
}

fn run(root: &Path, argv: &[String]) -> (String, bool) {
  let out = Command::new(bin())
    .args(argv)
    .current_dir(root)
    .output()
    .expect("spawn intent");
  let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
  s.push_str(&String::from_utf8_lossy(&out.stderr));
  (s.trim_end().to_string(), out.status.success())
}

struct Decl {
  path: String,
  setup: Vec<Vec<String>>,
  argv: Vec<String>,
  template: String,
}

fn declarations() -> Vec<Decl> {
  let raw: serde_json::Value = serde_json::from_str(dispatch::TABLE).expect("the table parses");
  let strings = |v: &serde_json::Value| -> Vec<String> {
    v.as_array()
      .expect("argv is an array")
      .iter()
      .map(|s| s.as_str().expect("argv member is a string").to_string())
      .collect()
  };

  let mut out = Vec::new();
  for fam in raw["families"].as_array().expect("families") {
    for e in fam["entries"].as_array().expect("entries") {
      let Some(x) = e.get("observed").and_then(|o| o.get("stdout_exact")) else {
        continue;
      };
      out.push(Decl {
        path: e["path"].as_str().expect("path").to_string(),
        setup: x["setup"]
          .as_array()
          .expect("setup is an array")
          .iter()
          .map(strings)
          .collect(),
        argv: strings(&x["argv"]),
        template: x["template"]
          .as_str()
          .expect("template is a string")
          .to_string(),
      });
    }
  }
  out
}

/// **v3's stdout must equal the bytes v2 printed, for every row that claims it does.**
#[test]
fn every_literal_as_observed_row_matches_the_v2_bytes_it_declares() {
  let decls = declarations();
  assert!(
    !decls.is_empty(),
    "no row carries `observed.stdout_exact`, so this test would pass by having \
     nothing to compare -- which is the exact failure it exists to prevent"
  );

  let mut wrong = Vec::new();
  for d in &decls {
    let dir = tempfile::tempdir().expect("tempdir");
    seed(dir.path());
    for s in &d.setup {
      let (o, ok) = run(dir.path(), s);
      assert!(
        ok,
        "`{}`: setup `{}` failed, so the row was never driven to the state its \
         template describes and the comparison below would be meaningless:\n{o}",
        d.path,
        s.join(" ")
      );
    }
    let (got, _) = run(dir.path(), &d.argv);
    if got != d.template {
      wrong.push(format!(
        "`{}`\n     invoked: intent {}\n     v2 printed: {}\n     v3 printed: {}",
        d.path,
        d.argv.join(" "),
        d.template,
        got
      ));
    }
  }

  assert!(
    wrong.is_empty(),
    "{} of {} literal `as-observed` row(s) do NOT reproduce v2's stdout:\n\n  {}\n\n\
     Each of these rows declares `target.state: as-observed`, which IS the claim \
     that v3's output equals v2's. A difference here is a parity break, not a \
     failing test -- fix the binary, or ratify the deviation and move the row to \
     `corrected`. **Do not edit the template to match v3**: the template is v2's \
     measured bytes, and rewriting it to match the binary is how `at na` came to \
     carry `n-a` in the column that records v2.",
    wrong.len(),
    decls.len(),
    wrong.join("\n  ")
  );
}

/// The declaration must not be able to describe a row whose target is not v2.
///
/// **`corrected` means the deviation is ratified**, so v2's bytes are the thing
/// v3 is deliberately NOT doing. A template on such a row would assert the
/// ratification away, and it would look like the strictest possible check while
/// doing it. `at green` was in the first draft on exactly this footing.
#[test]
fn no_corrected_or_new_surface_row_carries_a_v2_template() {
  let raw: serde_json::Value = serde_json::from_str(dispatch::TABLE).expect("the table parses");
  let mut bad = Vec::new();
  for fam in raw["families"].as_array().expect("families") {
    for e in fam["entries"].as_array().expect("entries") {
      if e
        .get("observed")
        .and_then(|o| o.get("stdout_exact"))
        .is_none()
      {
        continue;
      }
      let state = e["target"]["state"].as_str().unwrap_or("<absent>");
      if state != "as-observed" {
        bad.push(format!(
          "`{}` is `{state}`",
          e["path"].as_str().unwrap_or("?")
        ));
      }
    }
  }
  assert!(
    bad.is_empty(),
    "a v2 stdout template sits on {} row(s) whose target is not v2:\n  {}\n\n\
     On a `corrected` row the deviation is ratified and v2's bytes are what v3 \
     deliberately does not print, so this template would assert the ratification \
     away while reading as a strict check.",
    bad.len(),
    bad.join("\n  ")
  );
}
