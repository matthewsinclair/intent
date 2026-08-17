//! AT-02.6 / AC-02.6: **platform and data-model openness.** Every DB entity has
//! a 1-1 file form, and the round trip is lossless.
//!
//! hv's standing requirement: _"I want there to be ALWAYS a 1-1 mapping between
//! the db schema entities and an equivalent .json or .md version of those
//! entities SO THAT I can get my data out of the db and use it somewhere else
//! LOSSLESSLY."_ Under D34 that stopped being about third-party tools and
//! became the durability mechanism itself: the extract is how truth travels
//! between machines, so a lossy one destroys work at the clone boundary, where
//! nobody typed anything and nothing reports a failure.
//!
//! **THE TABLE LIST IS ENUMERATED FROM THE GENERATED DDL FACE.** Not from a
//! roster here, and the difference is not stylistic -- it is the whole of what
//! makes this test hold tomorrow. A hand-kept list is a list someone has to
//! remember to add to on the day they add a table, which is the day they are
//! thinking about anything else. The contract's own prose says "8 tables in the
//! DDL: threads, wps, criteria, tests, related, issues, event_log, file_index"
//! and **the DDL has nine** -- `doc_sections` is not in that sentence. The
//! enumeration finds it; a roster copied from the sentence would not have.
//!
//! Three properties:
//!
//! 1. **Every table declares how its data leaves** -- a file form, or a DERIVED
//!    exemption that says why. Absence is never the answer, which is the D05
//!    refusal posture applied to coverage.
//! 2. **The round trip is lossless in BOTH directions**, per D01's bidirectional
//!    sync: db -> disk -> db reproduces the DB content, and re-emitting
//!    reproduces the files byte for byte.
//! 3. **The file forms are readable without Intent** -- JSON and JSONL, parsed
//!    here by a plain `serde_json` that knows nothing about the model.
//!
//! The discriminating case is a table with no file form and no exemption. It is
//! asserted rather than performed by hand: [`declaration_gaps`] is a function of
//! a DDL string, run once over the real one and once over a synthetic one
//! carrying an undeclared table. A test over the tables that already have file
//! forms passes on the defect -- which is exactly how `event_log` survived a
//! whole AC with a schema face and no artefact.

mod common;

use common::{Fixture, sample_issue, sample_thread};
use intentsvcs::remedy::Remedy;
use intentsvcs::store::{DDL, Store};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Enumeration -- from the DDL face, never from a list here
// ---------------------------------------------------------------------------

/// Every table the DDL declares, in declaration order.
fn tables_in(ddl: &str) -> Vec<String> {
  let mut out = Vec::new();
  for line in ddl.lines() {
    let t = line.trim();
    let Some(rest) = t.strip_prefix("CREATE ") else {
      continue;
    };
    // `TABLE IF NOT EXISTS x (` and `VIRTUAL TABLE IF NOT EXISTS x USING ...`
    let rest = rest.strip_prefix("VIRTUAL ").unwrap_or(rest);
    let Some(rest) = rest.strip_prefix("TABLE ") else {
      continue; // an index or a trigger, which holds no data of its own
    };
    let rest = rest.strip_prefix("IF NOT EXISTS ").unwrap_or(rest);
    let name = rest
      .split_whitespace()
      .next()
      .unwrap_or_default()
      .trim_end_matches('(');
    if !name.is_empty() {
      out.push(name.to_string());
    }
  }
  out
}

/// The `-- openness:` declaration attached to each table, by walking the DDL
/// and remembering the last one seen before a `CREATE`.
///
/// Adjacency is the binding, which is why the declaration lives in the DDL
/// rather than in a table over here: a declaration and the thing it describes
/// cannot drift apart when they are consecutive lines of one string.
fn declarations_in(ddl: &str) -> Vec<(String, Option<String>)> {
  let mut out = Vec::new();
  let mut pending: Option<String> = None;
  for line in ddl.lines() {
    let t = line.trim();
    if let Some(decl) = t.strip_prefix("-- openness:") {
      // A two-line declaration continues on the next comment line.
      match &mut pending {
        Some(existing) => existing.push_str(decl),
        None => pending = Some(decl.trim().to_string()),
      }
      continue;
    }
    if t.starts_with("--") {
      if let Some(existing) = &mut pending {
        existing.push(' ');
        existing.push_str(t.trim_start_matches('-').trim());
      }
      continue;
    }
    if t.starts_with("CREATE ")
      && let Some(name) = tables_in(t).first()
    {
      out.push((name.clone(), pending.take()));
      continue;
    }
    pending = None;
  }
  out
}

/// What is wrong with a DDL's openness declarations. Empty means nothing is.
///
/// **A function of a string**, so the discriminating case can be a real
/// assertion rather than a mutation someone runs by hand once and never again.
fn declaration_gaps(ddl: &str) -> Vec<String> {
  let mut gaps = Vec::new();
  for (table, decl) in declarations_in(ddl) {
    let Some(decl) = decl else {
      gaps.push(format!(
        "{table} declares nothing -- add `-- openness: carried by <path>` or `-- openness: DERIVED -- <why>`"
      ));
      continue;
    };
    if let Some(reason) = decl.strip_prefix("DERIVED") {
      // An exemption with no reason is not a declaration, it is the absence of
      // one with extra words. The reason is what a reader checks.
      let reason = reason.trim_start_matches(['-', ' ']).trim();
      if reason.len() < 20 {
        gaps.push(format!(
          "{table} claims DERIVED without saying what recomputes it: {decl:?}"
        ));
      }
    } else if !decl.starts_with("carried by ") {
      gaps.push(format!("{table} declares something unreadable: {decl:?}"));
    }
  }
  gaps
}

#[test]
fn the_enumeration_reads_the_ddl_and_finds_every_table() {
  let tables = tables_in(DDL);

  // The face and the database it produces must declare the same tables --
  // otherwise a `CREATE` form the parse above does not recognise would drop a
  // table out of every check in this file, silently.
  let dir = tempfile::tempdir().expect("tempdir");
  let store = Store::open(&dir.path().join("intent.db")).expect("open");
  let live = store.table_names().expect("the live tables");

  let missed: Vec<&String> = tables.iter().filter(|t| !live.contains(t)).collect();
  assert!(
    missed.is_empty(),
    "the parse found {missed:?}, which the database does not have -- the parse is wrong"
  );
  // The other direction cannot be a straight comparison: FTS5 creates shadow
  // tables (`doc_sections_data` and friends) that are implementation, not
  // schema. What CAN be checked is that every live table is either declared or
  // a shadow of one that is.
  let undeclared: Vec<&String> = live
    .iter()
    .filter(|l| !l.starts_with("sqlite_"))
    .filter(|l| {
      !tables
        .iter()
        .any(|t| *l == t || l.starts_with(&format!("{t}_")))
    })
    .collect();
  assert!(
    undeclared.is_empty(),
    "the database has tables the DDL parse did not find: {undeclared:?}"
  );

  assert!(
    tables.len() >= 9,
    "only {} tables enumerated -- the parse is not reading the DDL: {tables:?}",
    tables.len()
  );
  assert!(
    tables.iter().any(|t| t == "doc_sections"),
    "the virtual table is a table and holds data: {tables:?}"
  );
}

/// **Every table says how its data leaves.**
#[test]
fn every_table_declares_a_file_form_or_a_reasoned_exemption() {
  let gaps = declaration_gaps(DDL);
  assert!(
    gaps.is_empty(),
    "tables with no declared route out of the database:\n  {}\n\
     absence is never the answer -- a table nobody has said how to export is a \
     table whose data cannot leave",
    gaps.join("\n  ")
  );
}

/// **THE DISCRIMINATING CASE.** A table with no declaration goes red.
///
/// Run against a synthetic DDL rather than by editing the real one, so it is a
/// standing assertion instead of a one-off someone did once. Three shapes,
/// because each is a different way of declaring nothing: silence, an exemption
/// with no reason, and a form of words the reader cannot act on.
#[test]
fn a_table_that_declares_nothing_is_refused() {
  let undeclared =
    format!("{DDL}CREATE TABLE IF NOT EXISTS shadow_ledger (\n  id TEXT PRIMARY KEY\n);\n");
  let gaps = declaration_gaps(&undeclared);
  assert!(
    gaps.iter().any(|g| g.contains("shadow_ledger")),
    "a table with no openness declaration must be reported: {gaps:?}"
  );

  let unreasoned = format!(
    "{DDL}-- openness: DERIVED\nCREATE TABLE IF NOT EXISTS shadow_ledger (\n  id TEXT PRIMARY KEY\n);\n"
  );
  assert!(
    declaration_gaps(&unreasoned)
      .iter()
      .any(|g| g.contains("shadow_ledger") && g.contains("without saying")),
    "DERIVED with no reason is the absence of a declaration wearing its clothes"
  );

  let unreadable = format!(
    "{DDL}-- openness: probably fine\nCREATE TABLE IF NOT EXISTS shadow_ledger (\n  id TEXT PRIMARY KEY\n);\n"
  );
  assert!(
    declaration_gaps(&unreadable)
      .iter()
      .any(|g| g.contains("shadow_ledger")),
    "a declaration a reader cannot act on is not a declaration"
  );

  // And the real DDL is not vacuously passing because the checker never fires.
  assert!(
    declaration_gaps(DDL).is_empty(),
    "precondition: the real DDL is clean, so the failures above are the injected ones"
  );
}

// ---------------------------------------------------------------------------
// Losslessness -- both directions, over a store with every table populated
// ---------------------------------------------------------------------------

/// A fixture whose store has a row in every table that claims a file form.
///
/// **Populated deliberately, because an empty table round-trips through
/// anything.** A test over a store with no events would prove nothing about the
/// event log, and the event log is the table that had no artefact at all.
fn populated() -> (Fixture, Vec<String>) {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  // ST0057 exists because `ac descope` enforces the ratified "target thread
  // exists" guard -- a requirement moved to a thread nobody has is held by
  // nobody.
  let mut target = sample_thread("ST0057");
  target.related.clear();
  fx.write_thread(&target);
  fx.write_issue(&sample_issue(21));
  let mut facade = fx.facade_on_disk();

  // Mutations rather than hand-written envelopes: the log has to carry what the
  // tool actually writes into it.
  //
  // NOT `.ok()`. The first cut swallowed these and the four tests below all
  // failed on the row-count precondition instead of on the refusal that caused
  // it -- a silent error in a fixture, which is the same defect as one in
  // production and harder to see. Two of the three verbs were illegal from the
  // fixture's own states (WP-02 is Done, so `wp start` is refused) and the
  // third tripped a guard landed hours earlier.
  facade
    .wp_reopen("ST0056", 2, "reopened for the round trip")
    .expect("WP-02 is Done, so reopen is the legal verb");
  facade
    .ac_descope("ST0056", "AC-03.2", "ST0057", Some("hv"), Some("moved"))
    .expect("descope to a thread that exists");
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("the thread is wip, so hold is legal");

  let ids: Vec<String> = facade
    .store()
    .events()
    .expect("events")
    .iter()
    .map(|e| e.id.clone())
    .collect();
  assert!(
    ids.len() >= 3,
    "precondition: the fixture wrote history to round-trip, got {}",
    ids.len()
  );
  (fx, ids)
}

/// **db -> disk -> db reproduces the DB content, including history.**
#[test]
fn the_round_trip_carries_every_table_that_claims_a_file_form() {
  let (fx, minted) = populated();

  {
    let mut facade = fx.facade_on_disk();
    facade.sync_to_disk().expect("db -> disk");
  }

  // **A real clone, not a deleted store.** The extract is copied to a machine
  // that has never held this project's database, which is the case D34 is about
  // -- and reaching it by deleting a database would be the fixture idiom D36
  // rules out, in a test about how data travels.
  let elsewhere = fx.clone_extract();
  let mut restored = elsewhere.facade_on_disk();
  restored.sync_from_disk().expect("disk -> db");

  let (threads, issues) = restored.store().load_canon().expect("load");
  assert_eq!(threads.len(), 2, "both threads came back");
  assert_eq!(issues.len(), 1, "the issue came back");
  // BY ID, not by index: the fixture grew a second thread and an index would
  // have kept passing while asserting about the wrong one.
  let mutated = threads
    .iter()
    .find(|t| t.id == "ST0056")
    .expect("the mutated thread");
  assert_eq!(
    mutated.status,
    intentsvcs::model::ThreadStatus::Hold,
    "and it came back in the state the mutations left it in, not its authored one"
  );
  assert_eq!(
    mutated.status_reason.as_deref(),
    Some("waiting on the fleet"),
    "with the reason, which is a column of its own and the thing a lossy trip drops"
  );
  let descoped = mutated
    .criteria
    .iter()
    .find(|c| c.id == "AC-03.2")
    .expect("the descoped criterion");
  assert!(
    matches!(&descoped.state, intentsvcs::model::AcState::Descoped { to, by, reason }
      if to == "ST0057" && by.as_deref() == Some("hv") && reason.as_deref() == Some("moved")),
    "a scope decision and everyone who made it survived: {:?}",
    descoped.state
  );

  let back: Vec<String> = restored
    .store()
    .events()
    .expect("events")
    .iter()
    .map(|e| e.id.clone())
    .collect();
  for id in &minted {
    assert!(
      back.contains(id),
      "an event did not survive the extract -- nothing recomputes history, so this is the \
       one loss that cannot be repaired: {id}"
    );
  }
}

/// **And re-emitting reproduces the files byte for byte.**
///
/// The second direction of D01's bidirectional sync. Without it, a round trip
/// can be lossless in VALUES and still rewrite every file on every sync, which
/// makes the extract churn in version control and makes a real change
/// indistinguishable from noise.
#[test]
fn re_emitting_the_extract_reproduces_it_byte_for_byte() {
  let (fx, _) = populated();
  let mut facade = fx.facade_on_disk();
  facade.sync_to_disk().expect("first emit");

  let paths = [
    "intent/st/ST0056/thread.json",
    "intent/issues/0021.json",
    "intent/events.jsonl",
  ];
  let first: Vec<String> = paths.iter().map(|p| fx.read(p)).collect();

  // Round the extract through a clone and emit again from there: the bytes a
  // second machine writes must be the bytes the first one did, or the two
  // repositories fight over every file forever.
  let elsewhere = fx.clone_extract();
  let mut second_pass = elsewhere.facade_on_disk();
  second_pass.sync_from_disk().expect("read it all back");
  second_pass.sync_to_disk().expect("second emit");

  for (path, before) in paths.iter().zip(&first) {
    assert_eq!(
      &elsewhere.read(path),
      before,
      "{path} changed when nothing did -- a round trip that rewrites its own output makes \
       every real change invisible in the noise"
    );
  }
}

/// **The file forms are readable WITHOUT Intent.**
///
/// "Use it somewhere else" is the requirement, so the check is done by a parser
/// that knows nothing about the model: plain `serde_json` into `Value`, with no
/// model type in sight. A form only Intent's own types can read is not an
/// extract, it is a private encoding.
#[test]
fn the_file_forms_parse_as_plain_json_with_no_model_types() {
  let (fx, _) = populated();
  fx.facade_on_disk().sync_to_disk().expect("emit");

  let thread: Value =
    serde_json::from_str(&fx.read("intent/st/ST0056/thread.json")).expect("plain JSON");
  assert_eq!(thread["id"], "ST0056");
  assert!(
    thread["criteria"][0]["state"]["is"].is_string(),
    "a criterion's state is legible without knowing the enum: {}",
    thread["criteria"][0]
  );

  let issue: Value = serde_json::from_str(&fx.read("intent/issues/0021.json")).expect("plain JSON");
  assert_eq!(issue["number"], 21);

  // JSONL: one self-describing object per line, no framing to reimplement.
  let events = fx.read("intent/events.jsonl");
  let lines: Vec<&str> = events.lines().filter(|l| !l.trim().is_empty()).collect();
  assert!(
    lines.len() >= 3,
    "history reached the file: {}",
    lines.len()
  );
  for line in &lines {
    let envelope: Value = serde_json::from_str(line).expect("each line is an object on its own");
    assert!(
      envelope["op"].is_string() && envelope["ts"].is_string(),
      "and says what it is: {envelope}"
    );
  }
}

/// **The extract is not merely tolerated by the file scan -- it is understood
/// by it, and the difference was a live trap.**
///
/// `events.jsonl` does not end with `.json`, so the scan's whole-document parse
/// skipped it through path shape rather than through any decision. That is the
/// same passing-by-luck the DB file had before D29 named it: a later
/// `contains(".json")`, or an extension normaliser, would start calling the one
/// file that carries all history malformed -- and a corrupt-looking history file
/// blocks every ingest of the project.
///
/// Asserted from BOTH sides, because "no finding" alone is what the accident
/// also produced: a valid extract is clean, and a damaged one is reported at
/// the right LINE rather than as a broken document.
#[test]
fn the_history_extract_is_scanned_as_jsonl_not_skipped_for_its_suffix() {
  let (fx, _) = populated();
  fx.facade_on_disk().sync_to_disk().expect("emit");
  let root = fx.root();

  let clean = intentsvcs::sync::scan(root, &[]).expect("scan");
  let history = clean
    .iter()
    .find(|e| e.path.ends_with("events.jsonl"))
    .expect("the scan sees the history extract at all");
  assert!(
    history.findings.is_empty(),
    "a valid extract is clean: {:?}",
    history.findings
  );

  // Damage line 2 and rescan. A whole-document JSON parse would report line 1
  // (the first line is not the whole file), and skipping the file entirely
  // would report nothing.
  let good = fx.read("intent/events.jsonl");
  let mut lines: Vec<String> = good.lines().map(ToString::to_string).collect();
  lines[1] = "{not json at all".to_string();
  fx.write_file("intent/events.jsonl", &format!("{}\n", lines.join("\n")));

  let damaged = intentsvcs::sync::scan(root, &[]).expect("scan");
  let history = damaged
    .iter()
    .find(|e| e.path.ends_with("events.jsonl"))
    .expect("still seen");
  assert_eq!(
    history.findings.len(),
    1,
    "one damaged line, one finding: {:?}",
    history.findings
  );
  assert_eq!(
    history.findings[0].line,
    Some(2),
    "located at the damaged LINE, which is the whole reason JSONL is read as lines"
  );
}

/// A damaged history file is refused BY LINE, never skipped.
///
/// The strictness that applies to every other read of the extract (D05), and it
/// matters more here than anywhere: skipping a bad line loses the one record
/// nothing can recompute, and leaves a log that looks complete.
#[test]
fn a_damaged_event_line_is_refused_by_number() {
  let (fx, _) = populated();
  fx.facade_on_disk().sync_to_disk().expect("emit");

  let good = fx.read("intent/events.jsonl");
  let mut lines: Vec<String> = good.lines().map(ToString::to_string).collect();
  lines[1] = "{\"id\": \"not an envelope\"}".to_string();
  fx.write_file("intent/events.jsonl", &format!("{}\n", lines.join("\n")));

  let err = fx
    .facade_on_disk()
    .sync_from_disk()
    .expect_err("a damaged history file must refuse");
  let rendered = err.render();
  assert!(
    rendered.contains("line 2"),
    "the refusal locates the damage: {rendered}"
  );
  assert!(
    rendered.contains("do NOT delete"),
    "and does not send the operator to delete the only copy of history: {rendered}"
  );
}
