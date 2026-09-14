//! **A `corrected` row is held to the v3 bytes its ratification declares.**
//!
//! `observed.stdout_exact` records what v2 was measured printing, and it claims
//! nothing about v3: an `as-observed` row says v2 was measured doing this, as the
//! register's own gloss for the state now says. A row whose v3 behaviour is ruled
//! is `corrected`, carries `target.stdout_exact`, and is held to those bytes
//! here. That is the one v3 claim this file asserts.
//!
//! It used to hold `as-observed` rows to v2's bytes too, reading the state as a
//! claim that v3 reproduces v2. Sampled against 3.0.1, most of those claims were
//! false, so the reading went (vc's ruling) rather than the rows being edited
//! one by one to match the binary.
//!
//! # Why a `corrected` row is asserted at all
//!
//! **It was not, and the gap was opened by this file's own guard.** `corrected`
//! means v2's bytes are what v3 deliberately does NOT print, so asserting them
//! would assert the ratification away while reading as the strictest check here
//! -- `at green` was in the first draft on exactly that footing and the guard
//! refused it. Correct, and it left the ratified rows **unasserted**, so **the
//! cheapest way to make a red row stop failing was to ratify it.** That
//! incentive sat inside the instrument built to make deviation expensive.
//!
//! A corrected row now carries BOTH templates. `observed.stdout_exact` is the
//! RECORD of what v2 did; `target.stdout_exact` is the REQUIREMENT v3 is held
//! to. The target deliberately carries no `setup`/`argv` and **inherits the
//! observed invocation**, so both templates describe one command and their
//! difference is the ratified deviation written out in bytes instead of prose.
//!
//! **And requiring them to DIFFER is what forces the right fixture.** `ac
//! rescope` on a NON-TEST criterion is ruled to print `back in scope
//! (unsatisfied)` -- byte-identical to v2. Declared there, the row would assert
//! as-observed behaviour under a corrected label and look strict doing it. The
//! difference-check drives the declaration onto the test-backed criterion, which
//! is the only invocation where the correction is observable at all.
//!
//! # What this file does NOT cover
//!
//! A `new-surface` row has no v2 invocation to inherit and is deliberately NOT
//! handled here. Stated rather than half-generalised.

use std::path::Path;
use std::process::Command;

use intent_cli::dispatch;

/// Rows that MUST keep a declaration, checked by name.
///
/// **This is a ratchet against the one escape the schema guards cannot see.**
/// Every guard below reasons about a row that HAS a template. Deleting
/// `observed.stdout_exact` outright removes the row from every population at
/// once and reds nothing -- and deletion is exactly what happened when these two
/// `ac` rows were first ratified. Naming them here converts a silent deletion
/// into an edit to this list, which somebody has to argue for in a diff.
///
/// Adding a row is free. Removing one should not be.
const ROWS_THAT_MUST_STAY_DECLARED: &[&str] = &[
  "ac satisfy",
  "ac descope",
  "ac rescope",
  "ac withdraw",
  "ac reinstate",
  "at red",
  "at na",
  "issues close",
  "issues open",
];

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
///
/// **The v2 side of every template was measured against a fixture v2's own
/// `intent at lint` had validated first**, so a malformed fixture could not
/// accuse a correct binary -- the expensive direction.
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

  std::fs::create_dir_all(root.join("intent/.canon/st")).expect("mkdir canon st");
  std::fs::write(
    root.join("intent/.canon/st/ST0001.json"),
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

  std::fs::write(
    root.join("intent/.canon/st/ST0002.json"),
    r#"{ "schema": "intent/thread@3.0", "id": "ST0002", "slug": "sink",
  "title": "Sink", "status": "wip", "created": "2026-08-17",
  "objective": "", "context": "", "wps": [], "criteria": [], "tests": [] }
"#,
  )
  .expect("write sink");

  std::fs::create_dir_all(root.join("intent/.canon/issues")).expect("mkdir issues");
  std::fs::write(
    root.join("intent/.canon/issues/0001.json"),
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
  let out = Command::new(bin())
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

/// Which column a row is held to. Chosen by `target.state`, never by which
/// fields happen to be present.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Held {
  /// v3's ruled bytes. A difference is an unimplemented ratification.
  V3Ruling,
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

/// Collect the rows held to one column.
///
/// **A `corrected` row inherits `setup`/`argv` from `observed.stdout_exact`**,
/// which is what makes the two templates comparable: they describe one
/// invocation, so their difference is the deviation and nothing else.
fn declarations(raw: &serde_json::Value, held: Held) -> Vec<Decl> {
  let mut out = Vec::new();
  for (e, state) in entries(raw) {
    let obs = e.get("observed").and_then(|o| o.get("stdout_exact"));
    let tgt = e.get("target").and_then(|t| t.get("stdout_exact"));

    let template = match (held, state.as_str()) {
      (Held::V3Ruling, "corrected") => match tgt {
        Some(x) => x["template"].as_str().expect("template is a string"),
        None => continue,
      },
      _ => continue,
    };

    // The invocation always comes from the observed block, for both columns.
    let Some(inv) = obs else { continue };
    out.push(Decl {
      path: e["path"].as_str().expect("path").to_string(),
      setup: inv["setup"]
        .as_array()
        .expect("setup is an array")
        .iter()
        .map(strings)
        .collect(),
      argv: strings(&inv["argv"]),
      template: template.to_string(),
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
  let decls = declarations(&raw, Held::V3Ruling);
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
     This is an UNIMPLEMENTED RATIFICATION, not a parity break -- v2 is not the \
     subject here and restoring v2's bytes would be the wrong fix. The remedy is to \
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
    let obs = e.get("observed").and_then(|o| o.get("stdout_exact"));
    let tgt = e.get("target").and_then(|t| t.get("stdout_exact"));
    let Some(tgt) = tgt else { continue };

    // A ruled template is meaningless on a row with nothing to deviate FROM.
    if state != "corrected" {
      bad.push(format!(
        "`{path}` is `{state}` and carries `target.stdout_exact` -- a ruled template \
         belongs only on a ratified deviation"
      ));
      continue;
    }

    // The target inherits the observed invocation, so it cannot stand alone.
    let Some(obs) = obs else {
      bad.push(format!(
        "`{path}` carries `target.stdout_exact` with no `observed.stdout_exact` to \
         inherit an invocation from, so the two templates would describe different \
         commands and their difference would mean nothing"
      ));
      continue;
    };

    // If they match, the row is not corrected in its OUTPUT and the label earns
    // nothing -- and, worse, the declaration was written at an invocation where
    // the correction is invisible.
    if obs["template"] == tgt["template"] {
      bad.push(format!(
        "`{path}` declares identical v2 and v3 templates ({}) -- either the row is \
         not corrected in its output, or the declaration was written at an \
         invocation where the correction is not observable",
        tgt["template"]
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
/// Two escapes, and both were live. Moving a row to `corrected` used to drop its
/// assertion, so the cheapest way to make a red row green was to ratify it.
/// Deleting `observed.stdout_exact` still removes a row from every population at
/// once, silently -- so the floor is checked by name.
#[test]
fn ratifying_a_row_cannot_remove_its_assertion() {
  let raw = table();
  let mut bad = Vec::new();

  for (e, state) in entries(&raw) {
    let path = e["path"].as_str().unwrap_or("?");
    let has_obs = e
      .get("observed")
      .and_then(|o| o.get("stdout_exact"))
      .is_some();
    let has_tgt = e
      .get("target")
      .and_then(|t| t.get("stdout_exact"))
      .is_some();

    if state == "corrected" && has_obs && !has_tgt {
      bad.push(format!(
        "`{path}` was moved to `corrected` while carrying v2's measured bytes and \
         did NOT gain a `target.stdout_exact` -- so the row is now asserted by \
         nothing. Ratifying changes WHICH template a row is held to; it does not \
         release it from being held to one."
      ));
    }
  }

  for want in ROWS_THAT_MUST_STAY_DECLARED {
    let found = entries(&raw)
      .iter()
      .any(|(e, _)| e["path"].as_str() == Some(*want));
    let declared = entries(&raw).iter().any(|(e, _)| {
      e["path"].as_str() == Some(*want)
        && e
          .get("observed")
          .and_then(|o| o.get("stdout_exact"))
          .is_some()
    });
    if !found {
      bad.push(format!(
        "`{want}` is named in the floor but is not a row at all"
      ));
    } else if !declared {
      bad.push(format!(
        "`{want}` is named in the floor and has lost its `observed.stdout_exact` -- \
         deleting a declaration removes the row from every population at once and \
         reds nothing else, which is why the floor is checked by name"
      ));
    }
  }

  assert!(
    bad.is_empty(),
    "{} row(s) have lost an assertion they are required to keep:\n  {}",
    bad.len(),
    bad.join("\n  ")
  );
}
