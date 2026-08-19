//! **The `observed` column is v2's bytes, and until now nothing ever read it back.**
//!
//! An `as-observed` row is a claim that v3's stdout equals v2's. 60 rows make
//! that claim and **the claim is delegated to prose** -- `observed.stdout`, a
//! sentence written by whoever read v2 at the time. Nothing anywhere compares
//! that sentence to v2, and nothing compares it to v3.
//!
//! **On 2026-08-17 the only subset anyone checked came back 4 wrong out of 8.**
//! Three had dropped a suffix -- `ok: <AC> back in scope` where v2 prints `ok:
//! <AC> back in scope (unsatisfied)`, and the dropped word is the whole question
//! an undo raises. **The fourth was `at na`, carrying `n-a`: v3's own token, in
//! the column that records v2.** That row is `as-observed`, so a check comparing
//! v3 against that column would have found agreement and **certified issue 0056
//! as correct parity.** The defect reached the register built to catch it,
//! before anyone knew there was a defect.
//!
//! **So the point of this file is not that one requirement should have one
//! statement. It is that a field a test EXECUTES gets measured, and measuring is
//! what found the four.** Prose drifts because nothing reads it back.
//!
//! # Two columns, two different kinds of red
//!
//! A row is held to exactly one template, chosen by its `target.state`:
//!
//! - **`as-observed`** -> held to `observed.stdout_exact.template`, v2's MEASURED
//!   bytes. A difference is a **parity break**: fix the binary, or ratify the
//!   deviation and move the row to `corrected`.
//! - **`corrected`** -> held to `target.stdout_exact.template`, v3's RULED bytes.
//!   A difference is an **unimplemented ratification**: build the ruled voice.
//!
//! **The two reds have different remedies, which is why they are two tests.**
//! Reporting "5 rows disagree" without saying which column each disagreed with
//! would hand the reader one number covering two unrelated obligations -- the
//! same collapse as a row pretending to one stdout when it has two.
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
//! # What is EXPECTED to be red, and why a green would be the alarm
//!
//! - **`at na`** -- template is v2's `n/a`; issue 0056 is open with the fix
//!   designed. **The token has already moved once without reaching v2's**: 0056
//!   records `n-a`, and at `34c6a3ae` it prints `na`. A row can be wrong in more
//!   than one spelling over its life, so an issue naming the wrong value is not
//!   evidence the row is right now.
//! - **`ac satisfy`, `at red`** -- the other two ruled parity breaks.
//! - **`ac rescope`, `ac reinstate`** -- ruled voice not yet built.
//!
//! **Measured against a `git archive` extract of `34c6a3ae`, not the worktree.**
//! The worktree run of this same file came back GREEN on all three as-observed
//! breaks -- because a peer had uncommitted fixes in `render.rs`, `facade.rs`,
//! `model.rs` and `views.rs` at the time. Reporting that would have declared the
//! canary green while the thing it guards was still broken at HEAD, and credited
//! a peer with work they had not landed.
//!
//! **A green on any of these would mean the template had been written from the
//! binary**, which is how `at na` came to carry `n-a` in the column that records
//! v2 in the first place.
//!
//! # What this file does NOT cover, stated so its green is not over-read
//!
//! Only nine rows carry a template -- the `ac`/`at`/`issues` literals. The other
//! ~51 `as-observed` rows are prose (`the table`, `the info.md contents`) and
//! cannot be asserted this way; their coverage question is the differential,
//! AT-00.1, and it is red. **Nine of 60 is not parity coverage and this file
//! must never be cited as though it were.**
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
  /// v2's measured bytes. A difference is a parity break.
  V2Record,
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
      (Held::V2Record, "as-observed") => match obs {
        Some(x) => x["template"].as_str().expect("template is a string"),
        None => continue,
      },
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

/// **v3's stdout must equal the bytes v2 printed, for every row that claims it does.**
#[test]
fn every_literal_as_observed_row_matches_the_v2_bytes_it_declares() {
  let raw = table();
  let decls = declarations(&raw, Held::V2Record);
  assert!(
    !decls.is_empty(),
    "no `as-observed` row carries `observed.stdout_exact`, so this test would pass \
     by having nothing to compare -- which is the exact failure it exists to prevent"
  );

  let wrong = mismatches(&decls);
  assert!(
    wrong.is_empty(),
    "{} of {} literal `as-observed` row(s) do NOT reproduce v2's stdout:\n\n  {}\n\n\
     Each of these rows declares `target.state: as-observed`, which IS the claim \
     that v3's output equals v2's. A difference here is a PARITY BREAK, not a \
     failing test -- fix the binary, or ratify the deviation and move the row to \
     `corrected` (which re-asserts it against the ruling, it does not drop it). \
     **Do not edit the template to match v3**: the template is v2's measured bytes, \
     and rewriting it to match the binary is how `at na` came to carry `n-a` in the \
     column that records v2.",
    wrong.len(),
    decls.len(),
    wrong.join("\n  ")
  );
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
