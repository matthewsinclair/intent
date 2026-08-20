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
use intentsvcs::store::{DDL, Store};
use serde_json::Value;
use std::path::{Path, PathBuf};
use testkit::repo_root;

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
    } else if let Some(rest) = decl.strip_prefix("ON DEMAND") {
      // **THE THIRD FORM (D53), AND IT IS NOT A SOFTER `DERIVED`.** `DERIVED`
      // says the data is reconstructible from something else already on disk;
      // `event_log` is the one table reconstructible from nothing, so it could
      // never take that exemption however convenient it looked. ON DEMAND says
      // the file form is real, lossless and standard, and simply is not kept
      // projected in the working tree -- so it owes a NAME for the form and a
      // reason it is not projected, and both are checked here.
      let rest = rest.trim_start_matches(['-', ' ']).trim();
      let (name, why) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
      if name.is_empty() {
        gaps.push(format!(
          "{table} claims ON DEMAND without naming the file form: {decl:?}"
        ));
      } else if why.trim_start_matches(['-', ' ']).trim().len() < 20 {
        gaps.push(format!(
          "{table} claims ON DEMAND without saying why it is not projected: {decl:?}"
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
// Referents -- ST0057 AC-01.7 / AT-01.7: a declaration must RESOLVE
// ---------------------------------------------------------------------------

/// Where a declared path actually points, or `None` when nothing is there.
///
/// **Resolved against the repository the face ships with, never a fixture.** A
/// declaration is a promise made to a consumer who has the extract and not the
/// DB, so the only tree that can answer it is the one that travels.
///
/// `<ID>` and `<NNNN>` each stand for one path component, and the literal text
/// around a placeholder still binds: `<NNNN>.json` matches `0033.json` and not
/// `steel_threads.md`. Walked segment by segment rather than handed to a glob,
/// because the interesting failure is WHICH segment stopped resolving, and an
/// empty glob result cannot say whether a directory was missing or merely
/// empty.
fn resolves_under(root: &Path, declared: &str) -> Option<PathBuf> {
  let mut candidates = vec![root.to_path_buf()];
  for segment in declared.split('/').filter(|s| !s.is_empty()) {
    let mut next = Vec::new();
    match segment.split_once('<') {
      Some((prefix, rest)) => {
        let suffix = rest.split_once('>')?.1;
        for base in &candidates {
          let Ok(entries) = std::fs::read_dir(base) else {
            continue;
          };
          for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.len() >= prefix.len() + suffix.len()
              && name.starts_with(prefix)
              && name.ends_with(suffix)
            {
              next.push(entry.path());
            }
          }
        }
      }
      None => {
        for base in &candidates {
          let joined = base.join(segment);
          if joined.exists() {
            next.push(joined);
          }
        }
      }
    }
    if next.is_empty() {
      return None;
    }
    // Sorted so a failure names the same file on every machine; `read_dir`
    // order is the filesystem's business, not this test's.
    next.sort();
    candidates = next;
  }
  candidates.into_iter().next()
}

/// The `carried by` declarations as (table, path), with the path ALONE.
///
/// **The first whitespace token only, and that is load-bearing rather than
/// tidy.** [`declarations_in`] deliberately absorbs the comment lines following
/// a declaration -- adjacency is how a declaration and its table are bound --
/// so two of the real ones arrive with a paragraph of prose attached. Taking
/// the whole remainder as the path yields a referent that cannot resolve **for
/// a reason that has nothing to do with where canon lives**: red before the
/// relocation, red after it, and reading exactly like a failed move.
/// The file forms a table declares as PRODUCED rather than projected (D53).
///
/// Deliberately a sibling of [`carried_paths`] rather than folded into it: the
/// two forms owe DIFFERENT evidence, and one function returning both would let
/// a caller check the cheaper proof and believe it had checked the other.
fn on_demand_forms(ddl: &str) -> Vec<(String, String)> {
  declarations_in(ddl)
    .into_iter()
    .filter_map(|(table, decl)| {
      let name = decl?
        .strip_prefix("ON DEMAND")?
        .trim_start_matches(['-', ' '])
        .split_whitespace()
        .next()?
        .to_string();
      Some((table, name))
    })
    .collect()
}

fn carried_paths(ddl: &str) -> Vec<(String, String)> {
  declarations_in(ddl)
    .into_iter()
    .filter_map(|(table, decl)| {
      let path = decl?
        .strip_prefix("carried by ")?
        .split_whitespace()
        .next()?
        .to_string();
      Some((table, path))
    })
    .collect()
}

/// AC-01.7: every `carried by` path names something that is really there.
///
/// **[`declaration_gaps`] cannot do this, and that is a limit rather than an
/// oversight.** It verifies the FORM of a claim -- that a table declares
/// something, and that the something starts with `carried by ` or is
/// `DERIVED -- <why>` -- and it never looks at the referent. A declaration
/// naming a path that has never existed passes it clean. **A check that
/// validates the form of a claim and never its referent is the shape this
/// estate keeps meeting.**
///
/// Under D34 the face travels while the DB never does, so a consumer following
/// the declaration to get their data out follows it to nothing, and **a false
/// statement about how to recover the data is strictly worse than no
/// statement** -- which is precisely what the openness block exists to prevent.
///
/// `DERIVED` declarations are out of scope BY CONSTRUCTION and not because the
/// filter happens to drop them: they name no path, and this criterion is about
/// referents. The partition is asserted so that the exclusion stays visible
/// instead of being inferred from a filter.
///
/// **Both figures are printed and the partition is asserted to close**, because
/// `resolved == declared` passes vacuously when a declaration is quietly
/// dropped from the DDL -- the shrunken-roster failure this estate has already
/// paid for. That the roster covers every table is held by
/// [`the_enumeration_reads_the_ddl_and_finds_every_table`] and
/// [`every_table_declares_a_file_form_or_a_reasoned_exemption`]; this test does
/// not restate it.
///
/// **RED-FIRST, DRIVEN BY THE REAL EVENT RATHER THAN A BOGUS PATH.** At
/// `f41d6760` the DDL was repointed to `intent/.canon/` while the 57 + 40 files
/// were still under `intent/st/` and `intent/issues/`, so at that revision this
/// goes red on 7 of 8 against the live tree -- six `intent/.canon/st/<ID>.json`,
/// one `intent/.canon/issues/<NNNN>.json`, with only `intent/events.jsonl`
/// resolving. The WP-01 file move is what turns it green. A hand-edited bogus
/// path would only have proved the checker parses.
#[test]
fn every_carried_by_declaration_resolves_to_something_on_disk() {
  let root = repo_root();
  let face = root.join("schema").join("ddl.sql");
  let ddl = std::fs::read_to_string(&face)
    .unwrap_or_else(|e| panic!("the shipped face {} is unreadable: {e}", face.display()));

  let carried = carried_paths(&ddl);
  let declarations = declarations_in(&ddl);
  let derived = declarations
    .iter()
    .filter(|(_, d)| d.as_deref().is_some_and(|d| d.starts_with("DERIVED")))
    .count();

  let on_demand = on_demand_forms(&ddl);
  assert_eq!(
    carried.len() + derived + on_demand.len(),
    declarations.len(),
    "the partition does not close: {} carried by + {derived} DERIVED + {} ON DEMAND over {} \
     declarations -- something declares in a FOURTH form and this test is silently not \
     looking at it",
    carried.len(),
    on_demand.len(),
    declarations.len()
  );

  let declared = carried.len();
  let mut dangling = Vec::new();
  for (table, path) in &carried {
    if resolves_under(&root, path).is_none() {
      dangling.push(format!("{table}: `carried by {path}` resolves to nothing"));
    }
  }
  let resolved = declared - dangling.len();

  assert!(
    dangling.is_empty(),
    "{resolved} of {declared} carried-by declarations resolve under {}; {} name a path \
     that is not there:\n  {}",
    root.display(),
    dangling.len(),
    dangling.join("\n  ")
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
    facade
      .sync_to_disk(&intentsvcs::sync::Scope::All)
      .expect("db -> disk");
  }

  // **A real clone, not a deleted store.** The extract is copied to a machine
  // that has never held this project's database, which is the case D34 is about
  // -- and reaching it by deleting a database would be the fixture idiom D36
  // rules out, in a test about how data travels.
  let elsewhere = fx.clone_extract();
  let mut restored = elsewhere.facade_on_disk();
  restored
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("disk -> db");

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

  // **THE EVENT LOG'S ROUND TRIP MOVED WITH D53; IT DID NOT DISAPPEAR, AND
  // THIS ARM PINS BOTH HALVES.** `intent/events.jsonl` is no longer projected
  // into the working tree, so the disk trip above cannot carry history --
  // asserting that it does would be asserting the design hv replaced. The
  // accepted COST is asserted here rather than left implicit, because a cost
  // nobody wrote down is one the next reader repairs by accident.
  let back: Vec<String> = restored
    .store()
    .events()
    .expect("events")
    .iter()
    .map(|e| e.id.clone())
    .collect();
  for id in &minted {
    assert!(
      !back.contains(id),
      "the disk round trip carried an event ({id}) -- under D53 the log is not projected, so \
       something has re-added a working-tree extract and the deletion is being undone by \
       accident rather than by a ruling"
    );
  }

  // **AND THE GUARANTEE THAT REMAINS IS DRIVEN, NOT ASSUMED.** `event_log`
  // declares its file form ON DEMAND, so AC-02.6's losslessness for this table
  // is proved through the exporter. **Deleting the assertion along with the
  // mechanism it outlived would leave hv's standing requirement held by
  // nobody** -- which is the failure mode the requirement exists to prevent,
  // reached by tidying rather than by neglect.
  let events = restored.store().events().expect("events for the bundle");
  let bundle = intentsvcs::export::Bundle::new("openness", Vec::new(), Vec::new(), events.clone());
  let parts = intentsvcs::export::canon_parts(&bundle).expect("canon parts");
  let (_, jsonl) = parts
    .iter()
    .find(|(rel, _)| rel == intentsvcs::event::JSONL)
    .expect("the exporter emits the ON DEMAND file form for event_log");
  for envelope in &events {
    assert!(
      jsonl.contains(&envelope.id),
      "an event did not survive the EXPORT -- nothing recomputes history, so this is the \
       one loss that cannot be repaired: {}",
      envelope.id
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
  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("first emit");

  let paths = [
    "intent/.canon/st/ST0056.json",
    "intent/.canon/issues/0021.json",
    // **NOT `intent/events.jsonl` (D53).** It is no longer projected, so a
    // second machine writes no bytes for it and there is nothing here for the
    // two repositories to fight over -- which was this arm's whole subject for
    // that path.
  ];
  let first: Vec<String> = paths.iter().map(|p| fx.read(p)).collect();

  // Round the extract through a clone and emit again from there: the bytes a
  // second machine writes must be the bytes the first one did, or the two
  // repositories fight over every file forever.
  let elsewhere = fx.clone_extract();
  let mut second_pass = elsewhere.facade_on_disk();
  second_pass
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("read it all back");
  second_pass
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("second emit");

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
  fx.facade_on_disk()
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("emit");

  let thread: Value = serde_json::from_str(&fx.read_canon("ST0056")).expect("plain JSON");
  assert_eq!(thread["id"], "ST0056");
  assert!(
    thread["criteria"][0]["state"]["is"].is_string(),
    "a criterion's state is legible without knowing the enum: {}",
    thread["criteria"][0]
  );

  let issue: Value = serde_json::from_str(&fx.read(&fx.issue_canon_rel(21))).expect("plain JSON");
  assert_eq!(issue["number"], 21);

  // JSONL: one self-describing object per line, no framing to reimplement.
  // **Read from the EXPORT rather than the tree (D53)**: the form is unchanged
  // and still standard, and this arm's claim is about the FORM, so it follows
  // the form to wherever it is produced rather than lapsing with the projection.
  let events = {
    let held = fx.facade_on_disk().store().events().expect("events");
    let bundle = intentsvcs::export::Bundle::new("openness", Vec::new(), Vec::new(), held);
    intentsvcs::export::canon_parts(&bundle)
      .expect("canon parts")
      .into_iter()
      .find(|(rel, _)| rel == intentsvcs::event::JSONL)
      .expect("the exporter emits the history form")
      .1
  };
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
