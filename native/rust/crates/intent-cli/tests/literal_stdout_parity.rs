//! **A `corrected` row is held to the v3 bytes its ratification declares.**
//!
//! A row whose v3 behaviour is ruled is `corrected` and carries
//! `target.stdout_exact`: the invocation (`setup`, then `argv`), the bytes v3 is
//! REQUIRED to print (`template`), and the ruling they come from (`basis`). It is
//! held to those bytes here. That is the one claim this file asserts.
//!
//! # Why a `corrected` row is asserted at all
//!
//! **Ratifying a row must not be a way to stop checking it.** When a ratified
//! row was left unasserted, **the cheapest way to make a red row stop failing
//! was to ratify it.** That incentive sat inside the instrument built to make
//! deviation expensive, so a corrected row states its ruled bytes and is driven
//! against them.
//!
//! # What this file does NOT cover
//!
//! An `as-observed` or `new-surface` row carries no ruled template and is
//! deliberately NOT handled here. Stated rather than half-generalised.

use std::path::Path;

use intent_cli::dispatch;

/// Rows that MUST keep a declaration, checked by name.
///
/// **This is a ratchet against the one escape the schema guards cannot see.**
/// Every guard below reasons about a row that HAS a template. Deleting
/// `target.stdout_exact` outright removes the row from every population at
/// once and reds nothing -- and deletion is exactly what happened when these two
/// `ac` rows were first ratified. Naming them here converts a silent deletion
/// into an edit to this list, which somebody has to argue for in a diff.
///
/// Adding a row is free. Removing one should not be.
const ROWS_THAT_MUST_STAY_DECLARED: &[&str] = &["ac rescope", "ac reinstate"];

/// The fixture every declaration is written against: one thread with one
/// authored criterion, one test-backed criterion, a test AT and a non-test AT,
/// a sink thread to descope into, and one open issue.
///
/// **Entity IDs match the ones the templates name**, which is the only reason a
/// template can be a literal rather than a pattern.
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

  crate::common::write_thread_canon(
    root,
    "ST0001",
    serde_json::json!({
      "slug": "probe", "title": "Probe", "status": "wip", "created": "2026-08-17",
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
    }),
  );

  crate::common::write_thread_canon(
    root,
    "ST0002",
    serde_json::json!({
      "slug": "sink", "title": "Sink", "status": "wip", "created": "2026-08-17",
      "wps": [], "criteria": [], "tests": []
    }),
  );

  crate::common::write_issue_canon(
    root,
    1,
    serde_json::json!({
      "slug": "probe",
      "title": "Probe issue",
      "status": "open",
      "created": "2026-08-17",
      "severity": "low"
    }),
  );
}

/// Returns the two channels SEPARATELY, and that separation is the point.
///
/// **This used to concatenate stderr onto stdout and compare the merged string
/// against a field named `stdout_exact`, in a file named `literal_stdout_parity`**
/// (cc found it, 2026-08-17, and measured it before reporting it). A row would
/// then have PASSED with the expected bytes on STDERR and stdout empty --
/// which is precisely the defect class this register exists to catch, so the
/// instrument was blind to its own subject.
///
/// **Not live when found, and it is worth recording that it was a hazard rather
/// than a defect**: comparing stdout alone leaves every declared row green, so
/// no row was passing on stderr. cc measured that in a worktree and I
/// reproduced it here before changing anything. **A merge that nothing
/// currently exercises stays correct right up until the day one command moves
/// its answer to stderr -- and on that day the instrument reports parity.**
///
/// vc's mechanism for this is CHANNEL SHARING, from their own liveness defect:
/// their probe ran `st list 2>&1`, so a refusal naming thread ids arrived on
/// the same stream as the answer and **the failure was well-formed input**.
/// Same shape, one layer down.
fn run(root: &Path, argv: &[String]) -> (String, String, bool) {
  let out = crate::common::intent()
    .args(argv)
    .current_dir(root)
    .stdin(testkit::lifeline_for(argv))
    .output()
    .expect("spawn intent");
  (
    String::from_utf8_lossy(&out.stdout).trim_end().to_string(),
    String::from_utf8_lossy(&out.stderr).trim_end().to_string(),
    out.status.success(),
  )
}

struct Decl {
  path: String,
  setup: Vec<Vec<String>>,
  argv: Vec<String>,
  template: String,
}

fn table() -> serde_json::Value {
  serde_json::from_str(dispatch::TABLE).expect("the table parses")
}

fn strings(v: &serde_json::Value) -> Vec<String> {
  v.as_array()
    .expect("argv is an array")
    .iter()
    .map(|s| s.as_str().expect("argv member is a string").to_string())
    .collect()
}

/// Walk every entry with its `target.state`, so callers never have to infer the
/// state from which fields exist.
fn entries(raw: &serde_json::Value) -> Vec<(&serde_json::Value, String)> {
  let mut out = Vec::new();
  for fam in raw["families"].as_array().expect("families") {
    for e in fam["entries"].as_array().expect("entries") {
      let state = e
        .get("target")
        .and_then(|t| t.get("state"))
        .and_then(|s| s.as_str())
        .unwrap_or("<absent>")
        .to_string();
      out.push((e, state));
    }
  }
  out
}

/// Collect the `corrected` rows, each with its ruled invocation and bytes.
///
/// Chosen by `target.state`, never by which fields happen to be present: a
/// template on any other row is refused by the guard below rather than driven.
fn declarations(raw: &serde_json::Value) -> Vec<Decl> {
  let mut out = Vec::new();
  for (e, state) in entries(raw) {
    if state != "corrected" {
      continue;
    }
    let Some(tgt) = e.get("target").and_then(|t| t.get("stdout_exact")) else {
      continue;
    };
    out.push(Decl {
      path: e["path"].as_str().expect("path").to_string(),
      setup: tgt["setup"]
        .as_array()
        .expect("setup is an array")
        .iter()
        .map(strings)
        .collect(),
      argv: strings(&tgt["argv"]),
      template: tgt["template"]
        .as_str()
        .expect("template is a string")
        .to_string(),
    });
  }
  out
}

/// Drive each declaration and return the ones whose stdout differs.
fn mismatches(decls: &[Decl]) -> Vec<String> {
  let mut wrong = Vec::new();
  for d in decls {
    let dir = tempfile::tempdir().expect("tempdir");
    seed(dir.path());
    for s in &d.setup {
      let (o, e, ok) = run(dir.path(), s);
      // Both channels here on purpose: a FAILING setup puts its reason on
      // stderr, and this message exists to explain the failure rather than to
      // assert parity. The comparison below is the one that must not merge.
      assert!(
        ok,
        "`{}`: setup `{}` failed, so the row was never driven to the state its \
         template describes and the comparison below would be meaningless:\n{o}{e}",
        d.path,
        s.join(" ")
      );
    }
    let (got, err, _) = run(dir.path(), &d.argv);
    // **THE RESULT LINES, NOT THE FILES THE WRITE NAMES** (issue 0624). Every
    // write prints a `wrote:` line per file it landed, for the commit rule, and
    // one of them is an event file whose name is minted per run, which no
    // literal can hold. A template's claim is the verb's result.
    let got = crate::common::result_lines(&got);
    if got != d.template {
      // **NAME THE CHANNEL WHEN THE CHANNEL IS THE DEFECT.** Without this the
      // day the merge would have mattered reads as "v3 printed nothing", and
      // the reader goes looking for a command that produced no output -- when
      // what actually happened is that it produced exactly the right bytes on
      // the wrong stream. That is the failure this whole file is about, so it
      // must not arrive disguised as silence.
      let note = if err == d.template {
        format!(
          "\n     WRONG CHANNEL: stdout was {}, and STDERR carried the required bytes exactly. \
           This row's claim is about STDOUT.",
          if got.is_empty() { "EMPTY" } else { "different" }
        )
      } else if got.is_empty() && !err.is_empty() {
        format!("\n     stdout was EMPTY; stderr said: {err}")
      } else {
        String::new()
      };
      wrong.push(format!(
        "`{}`\n     invoked:  intent {}\n     required: {}\n     v3 printed: {}{}",
        d.path,
        d.argv.join(" "),
        d.template,
        got,
        note
      ));
    }
  }
  wrong
}

/// **A ratified row is held to its RULING, which is the obligation ratifying buys
/// -- not a release from being checked.**
#[test]
fn every_corrected_row_matches_the_v3_bytes_its_ratification_declares() {
  let raw = table();
  let decls = declarations(&raw);
  assert!(
    !decls.is_empty(),
    "no `corrected` row carries `target.stdout_exact`, so this test would pass by \
     having nothing to compare -- and an empty corrected population is precisely \
     the state this file was in when ratifying a row silently removed its assertion"
  );

  let wrong = mismatches(&decls);
  assert!(
    wrong.is_empty(),
    "{} of {} `corrected` row(s) do NOT print the bytes their ratification requires:\n\n  {}\n\n\
     This is an UNIMPLEMENTED RATIFICATION. The remedy is to \
     build the ruled voice, which the row's `target.ratification` states in full. \
     **Do not edit the template to match v3**: these bytes come from the ruling, and \
     rewriting them from the binary makes the assertion say v3 == v3.",
    wrong.len(),
    decls.len(),
    wrong.join("\n  ")
  );
}

/// The declaration must not be able to describe a row whose target it is not.
#[test]
fn a_template_sits_only_where_its_column_is_the_rows_target() {
  let raw = table();
  let mut bad = Vec::new();

  for (e, state) in entries(&raw) {
    let path = e["path"].as_str().unwrap_or("?");
    let Some(tgt) = e.get("target").and_then(|t| t.get("stdout_exact")) else {
      continue;
    };

    // A ruled template is meaningless on a row with no ruling to hold it to.
    if state != "corrected" {
      bad.push(format!(
        "`{path}` is `{state}` and carries `target.stdout_exact` -- a ruled template \
         belongs only on a ratified deviation"
      ));
      continue;
    }

    // The template names the bytes of ONE invocation, so it must say which.
    if !tgt["setup"].is_array() || !tgt["argv"].is_array() {
      bad.push(format!(
        "`{path}`'s `target.stdout_exact` does not declare its invocation as `setup` \
         and `argv` arrays, so its bytes describe no command"
      ));
    }

    // The bytes must be traceable to a ruling, or they are just bytes.
    if tgt
      .get("basis")
      .and_then(|b| b.as_str())
      .unwrap_or("")
      .is_empty()
    {
      bad.push(format!("`{path}`'s `target.stdout_exact` has no `basis`"));
    }
    if e["target"]
      .get("ratification")
      .and_then(|r| r.as_str())
      .unwrap_or("")
      .is_empty()
    {
      bad.push(format!(
        "`{path}` is held to a ruled template but carries no `target.ratification` \
         stating the ruling"
      ));
    }
  }

  assert!(
    bad.is_empty(),
    "{} declaration(s) are attached to the wrong column:\n  {}",
    bad.len(),
    bad.join("\n  ")
  );
}

/// **Ratifying a row must not be a way to stop being checked.**
///
/// Deleting `target.stdout_exact` removes a row from every population at once,
/// silently -- so the floor is checked by name.
#[test]
fn ratifying_a_row_cannot_remove_its_assertion() {
  let raw = table();
  let mut bad = Vec::new();

  for want in ROWS_THAT_MUST_STAY_DECLARED {
    let rows = entries(&raw);
    let row = rows.iter().find(|(e, _)| e["path"].as_str() == Some(*want));
    match row {
      None => bad.push(format!(
        "`{want}` is named in the floor but is not a row at all"
      )),
      Some((e, _))
        if e
          .get("target")
          .and_then(|t| t.get("stdout_exact"))
          .is_none() =>
      {
        bad.push(format!(
          "`{want}` is named in the floor and has lost its `target.stdout_exact` -- \
           deleting a declaration removes the row from every population at once and \
           reds nothing else, which is why the floor is checked by name"
        ))
      }
      Some(_) => {}
    }
  }

  assert!(
    bad.is_empty(),
    "{} row(s) have lost an assertion they are required to keep:\n  {}",
    bad.len(),
    bad.join("\n  ")
  );
}
