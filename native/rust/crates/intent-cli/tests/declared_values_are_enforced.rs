//! **A dispatch-table slot that declares `values` is enforced by SOMETHING, and
//! this is what says so.**
//!
//! `values` on a non-subcommand slot is a declaration, not a constraint.
//! `spine.rs` reads it in exactly two places -- expanding a `kind: subcommand`
//! slot, and resolving a default -- and builds no `value_parser` from a
//! positional's, so the permitted set reaches clap as documentation and the
//! enforcement is one layer down. The table's own `arg_values_note` says that is
//! a fine arrangement "only while something does" enforce it, and records how it
//! stops being fine: **an author writes the array assuming clap has it, an
//! implementer reads the row assuming the same, and nobody enforces it.** That
//! is not hypothetical -- it nearly happened on `wp rescope` on 2026-08-17, and
//! what caught it was the timing of a correction rather than anything structural.
//!
//! **So this test is the structure.** Every such slot in the table must carry a
//! disposition here, and each disposition is DRIVEN against the real binary
//! rather than asserted from the row. Adding a slot to the table without a
//! disposition fails, which is the same refusal `mutation_completeness.rs` gives
//! a `State` field with no drive arm: an entry nobody has decided about is a
//! failure, not a skip.
//!
//! **`Unenforced` is a disposition and not an exemption.** It pins the defective
//! behaviour to its issue, so the exposure cannot be silence -- and the moment
//! someone fixes it this file reds and they have to move the row. A test that
//! quietly skipped the broken case would read as coverage.
//!
//! **Measured at `b7e60fc5`: FIVE. The note says three and my own first probe
//! said four.** `arg_values_note` enumerates `critic`'s `lang`, `st show`'s `file`
//! and `wp rescope`'s `size`; `st edit`'s `file` is the fourth and is missing from
//! that sentence. The fifth is `backup.schedule` under `config`, and it took a
//! third instrument to see, because my probe walked `entry.args` and `entry.flags`
//! -- two container keys, hand-enumerated -- and the fifth lives in a third
//! (`target.keys_backup.keys`). **Three counts, three enumerators, each short of
//! the next**, which is issue 0050's nineteen-that-were-twenty-one twice more. It
//! is the reason this file's population is a recursive walk over the whole table
//! and not a list of the places a slot is expected to be.
//!
//! The fifth also widens the subject usefully. `backup.schedule` is not an argv
//! slot at all -- it is a CONFIG KEY declaring its permitted values -- and the
//! note's argument does not depend on the layer: a register declaring a
//! vocabulary that nothing enforces is the same exposure wherever the vocabulary
//! is read. That key's own note argues for enumerating over a cron expression
//! *because* a mistyped cron is "SILENTLY wrong when mistyped rather than
//! refused". The enumeration only buys that if something refuses.

use std::path::Path;
use std::process::Command;

use intent_cli::dispatch;
use intentsvcs::model::{TShirt, enum_str};

/// What this build does when handed a value the row does not permit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Disposition {
  /// Exit 1, and the message names the permitted set. A bare refusal blames the
  /// operator's spelling without saying what the spellings are, so naming the
  /// set is part of the requirement rather than a nicety.
  Enforced,
  /// Exit 2 -- the command is not built, so there is nothing to enforce yet. It
  /// owes the arm when it is wired, and this reds then.
  Unwired,
  /// **The value is ACCEPTED.** The issue number is the referent; without one
  /// this row would be an exemption wearing a disposition.
  Unenforced(&'static str),
  /// Not an argv slot -- a declared vocabulary for a surface that does not exist
  /// yet, so there is nothing to hand a bad value to. The named command must
  /// still be unbuilt; when it is built this reds and someone decides who reads
  /// the vocabulary.
  Planned { via: &'static str },
}

struct Slot {
  /// The command path as the table spells it.
  path: &'static str,
  /// The slot's name in the row -- carried so a rename of it reds this.
  arg: &'static str,
  /// The positionals that come BEFORE the declared slot.
  lead: &'static [&'static str],
  /// The positionals that come AFTER it.
  ///
  /// **ADDED FOR `edit.kind`, THE FIRST SLOT WITH A REQUIRED TRAILING
  /// POSITIONAL** (ic, 2026-08-29). Until AC-17.6 reshaped `edit` to `<KIND>
  /// <ID> [FILE]`, every declared slot in this table was the LAST positional
  /// of its row, so `lead` alone built a complete argv and the harness never
  /// had to say so. **`intent edit <bad-kind>` refuses at exit 1 for a missing
  /// `<ID>` -- a refusal about arity, not about the vocabulary** -- so the
  /// probe never reaches the value it came to test and every disposition would
  /// have been measured against the wrong refusal.
  ///
  /// The instrument's shape was a silent assumption about the corpus, and it
  /// held right up until the corpus changed. Symmetric with `lead` on purpose:
  /// a slot is surrounded, not prefixed.
  trail: &'static [&'static str],
  disposition: Disposition,
}

/// **Every non-subcommand slot in the table that declares `values`.** Read from
/// the table below and cross-checked against this in both directions.
const DECLARED: &[Slot] = &[
  Slot {
    path: "st show",
    arg: "file",
    lead: &["ST0001"],
    trail: &[],
    // Issue 0055: the arm never reads the slot at all, so a correct value and an
    // incorrect one produce the same output. Three of the row's four declared
    // exit codes are unreachable, and `st show ST0001 design` prints the info
    // summary at exit 0 -- a `keep` row answering a different question.
    disposition: Disposition::Unenforced("0055"),
  },
  // **BOTH `edit` ROWS MOVED FROM `Unwired` TO `Unenforced` ON 2026-08-20**
  // (ic, ruling 3). `st edit` was unbuilt and owed the arm when it was wired;
  // it is now wired and delegates to the top-level `intent edit`, so the debt
  // came due and this is it being paid honestly rather than upgraded.
  //
  // **WHAT THE VERB DOES DO** is refuse at exit 1 naming the files the artefact
  // CARRIES -- which is close to the requirement and is not it. The declared
  // set is `info | design | impl | tasks | acceptance`; the message names
  // neither that set nor a subset of it, since a thread carries attachments
  // outside the enum and need not carry the members inside it.
  //
  // **AND CLAP IS THE WRONG LAYER, WHICH IS WHY THIS IS NOT A ONE-LINE
  // `value_parser`.** The spine reads `arg.default` for enum args and never
  // `arg.values`, so adding the parser is mechanically easy -- and a clap
  // rejection exits **2**, which is INV-04's USAGE code and the one the
  // pre-commit gate FAILS OPEN on. Satisfying the word there would have broken
  // the contract, and done the same to `wp rescope --size` and `critic <lang>`.
  //
  // **SO THE RENDERER ENFORCES IT, READING THE SET FROM THE TABLE**
  // (`dispatch::arg_values`) rather than restating the five spellings, which
  // would have been a second declaration of the same vocabulary. Exit 1, and
  // the message names the set. Issue 0062 recorded the gap and is closed by
  // this rather than carried.
  Slot {
    path: "st edit",
    arg: "file",
    lead: &["ST0001"],
    trail: &[],
    disposition: Disposition::Enforced,
  },
  Slot {
    // **THE LEAD MOVED WITH THE ROW, AND THE OLD ONE STAYED GREEN WHILE
    // MEASURING A DIFFERENT REFUSAL** (ic, 2026-08-29, on their own change).
    // `edit` became `<KIND> <ID> [FILE]` for AC-17.6, so `intent edit ST0001
    // nonsense` now reads `ST0001` as the KIND and `nonsense` as the ID, and
    // answers `nonsense is not a steel thread id` -- an exit-1 refusal that
    // names a permitted set, which is what this arm asserts, about a question
    // nobody asked. **A probe's lead is part of the probe**: repoint the
    // positionals and the same assertion silently starts checking a different
    // thing. The correct lead answers `nonsense is not a file this verb can
    // open -- name one of info, design, impl, tasks, acceptance`.
    path: "edit",
    arg: "file",
    lead: &["st", "ST0001"],
    trail: &[],
    disposition: Disposition::Enforced,
  },
  Slot {
    // **DECLARED AND DROPPED, WHICH IS WHY IT IS `Unenforced` AND NOT
    // `Enforced`** (issue 0149). `intent edit issue 148 --path` answers `no
    // steel thread ST0148 in this project`: the parser takes `issue`, the arm
    // ignores it, and the refusal is about an entity the caller never named.
    //
    // AC-17.6's whole argument for the slot is that `intent edit 1` already
    // refuses with `1 names both a steel thread and an issue` -- so the kind
    // resolves an ambiguity the tool ALREADY reports. **A caller who types
    // `issue` has supplied that answer, and being told about `ST0148` says the
    // tool discarded the one thing it asked for.** It moves to `Enforced` when
    // the resolver reads the kind, which WP-17 piece 3 owes.
    path: "edit",
    arg: "kind",
    lead: &[],
    trail: &["ST0001"],
    disposition: Disposition::Unenforced("0149"),
  },
  Slot {
    // The twin (INV-09). Unbuilt today -- `intent browse st ST0056` answers
    // rc=2 -- so there is nothing to hand a bad value to yet, and this reds
    // when the verb is wired.
    path: "browse",
    arg: "kind",
    lead: &[],
    trail: &["ST0001"],
    disposition: Disposition::Unwired,
  },
  Slot {
    path: "wp rescope",
    arg: "size",
    lead: &["ST0001/01"],
    trail: &[],
    disposition: Disposition::Enforced,
  },
  Slot {
    path: "critic",
    arg: "lang",
    lead: &[],
    trail: &[],
    disposition: Disposition::Unwired,
  },
  Slot {
    // A config KEY, not an argument -- `target.keys_backup.keys` on the `config`
    // row, whose own note argues for an enumeration over a cron expression
    // because a mistyped cron fails silently. Nothing in the workspace mentions
    // `backup.schedule` or any of its three values, and `config` is `pending` /
    // `undefined`, so the vocabulary is a design declaration awaiting a reader.
    path: "config",
    arg: "<unnamed>",
    lead: &[],
    trail: &[],
    disposition: Disposition::Planned { via: "config" },
  },
];

/// A project with one thread and one work package, so the wired commands get
/// past the project gate and reach their own argument handling.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Values\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  let st = dir.path().join("intent/.canon/st");
  std::fs::create_dir_all(&st).expect("mkdir");
  std::fs::write(
    st.join("ST0001.json"),
    r#"{
  "schema": "intent/thread@3.0",
  "id": "ST0001",
  "slug": "a-thread",
  "title": "A thread",
  "status": "wip",
  "created": "2026-08-15",
  "objective": "",
  "context": "",
  "wps": [ { "seq": 1, "title": "A package", "scope": "S", "status": "wip" } ],
  "criteria": [],
  "tests": []
}
"#,
  )
  .expect("write canon");
  dir
}

/// Every `(path, slot, values)` the TABLE declares on a non-subcommand slot.
///
/// **Read from the raw table text, not from `dispatch::table()`, and the
/// difference is the point.** `dispatch::Arg` carries a `values` field and
/// `dispatch::Flag` does NOT -- so a `values` array written on a flag row
/// deserialises into nothing, and a sweep over the typed model would report the
/// population as complete while being structurally unable to see it. **The typed
/// model is a different subject from the table**, and a slot the model drops is
/// exactly the slot this file exists to notice. Zero flag rows carry one today,
/// which is the reason to read the text rather than the reason not to.
///
/// Generic over slot kind for the same reason: a third kind of slot added beside
/// `args` and `flags` is picked up without anyone remembering to widen this.
fn declared_in_table() -> Vec<(String, String, Vec<String>)> {
  let root: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the compiled-in table parses");
  let mut out = Vec::new();
  collect(&root, None, &mut out);
  out.sort();
  out
}

/// Walk the table for any object carrying a non-empty `values`, remembering the
/// nearest enclosing `path` so a finding names the command it belongs to.
fn collect(
  node: &serde_json::Value,
  path: Option<&str>,
  out: &mut Vec<(String, String, Vec<String>)>,
) {
  match node {
    serde_json::Value::Object(map) => {
      let path = map.get("path").and_then(|p| p.as_str()).or(path);
      let kind = map.get("type").and_then(|t| t.as_str()).unwrap_or("");
      if let Some(serde_json::Value::Array(values)) = map.get("values")
        && !values.is_empty()
        && kind != "subcommand"
      {
        let name = map
          .get("name")
          .and_then(|n| n.as_str())
          // A flag has `spellings` where an arg has `name`; taking the first
          // means an undeclared flag slot still reports as something a reader
          // can find rather than as an anonymous row.
          .or_else(|| {
            map
              .get("spellings")
              .and_then(|s| s.as_array())
              .and_then(|s| s.first())
              .and_then(|s| s.as_str())
          })
          .unwrap_or("<unnamed>");
        out.push((
          path.unwrap_or("<no path>").to_string(),
          name.to_string(),
          values
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        ));
      }
      for value in map.values() {
        collect(value, path, out);
      }
    }
    serde_json::Value::Array(items) => {
      for item in items {
        collect(item, path, out);
      }
    }
    _ => {}
  }
}

/// **The population is read from the table, and a slot with no disposition is a
/// FAILURE.**
///
/// Both directions, because one alone is not the property: a list here that had
/// gone short would pass a disposition-to-table check, and a disposition for a
/// slot the table no longer declares is a rule about nothing.
#[test]
fn every_slot_declaring_values_carries_a_disposition() {
  let in_table = declared_in_table();
  let undecided: Vec<&(String, String, Vec<String>)> = in_table
    .iter()
    .filter(|(path, arg, _)| {
      !DECLARED
        .iter()
        .any(|s| s.path == path.as_str() && s.arg == arg.as_str())
    })
    .collect();
  assert!(
    undecided.is_empty(),
    "these slots declare a permitted set and nothing here says who enforces it -- decide, do not \
     skip: {undecided:?}"
  );

  let stale: Vec<&str> = DECLARED
    .iter()
    .filter(|s| {
      !in_table
        .iter()
        .any(|(path, arg, _)| path == s.path && arg == s.arg)
    })
    .map(|s| s.path)
    .collect();
  assert!(
    stale.is_empty(),
    "these dispositions name a slot the table no longer declares, so they are rules about nothing: \
     {stale:?}"
  );

  assert!(
    !in_table.is_empty(),
    "an empty population would make every assertion in this file vacuously true -- zero rows and \
     zero rows matching are the same output"
  );
}

/// **Drive every disposition against the real binary.**
///
/// The value is one the row cannot permit by construction -- built from the
/// declared set rather than picked, so it cannot accidentally BE a permitted
/// value on some future row.
#[test]
fn each_disposition_is_what_the_binary_actually_does() {
  let dir = project();
  let in_table = declared_in_table();

  for slot in DECLARED {
    let values = in_table
      .iter()
      .find(|(path, arg, _)| path == slot.path && arg == slot.arg)
      .map(|(_, _, values)| values.clone())
      .expect("the exhaustiveness test above guarantees this");
    let bad = format!("not-{}", values.join("-or-"));

    let mut argv: Vec<&str> = slot.path.split(' ').collect();
    argv.extend_from_slice(slot.lead);
    argv.push(&bad);
    argv.extend_from_slice(slot.trail);
    let (code, text) = drive(dir.path(), &argv);

    match slot.disposition {
      Disposition::Enforced => {
        assert_eq!(
          code,
          Some(1),
          "`intent {}` declares a permitted set, so an unreadable value is a refusal: {text}",
          argv.join(" ")
        );
        for value in &values {
          assert!(
            text.contains(value),
            "the refusal must name the permitted set and `{value}` is missing: {text}"
          );
        }
      }
      Disposition::Unwired => {
        assert_eq!(
          code,
          Some(2),
          "`intent {}` is recorded here as unbuilt. If it has been wired, it now owes the exit-1 \
           arm its row's `values` declares -- move this row to Enforced rather than relaxing the \
           assertion: {text}",
          argv.join(" ")
        );
      }
      Disposition::Unenforced(issue) => {
        assert_eq!(
          code,
          Some(0),
          "`intent {}` is recorded here as accepting an unpermitted value (issue {issue}). This \
           passing at any other code means the defect has been fixed or has changed shape -- move \
           the row, do not widen it: {text}",
          argv.join(" ")
        );
      }
      Disposition::Planned { via } => {
        // The declared value is not reachable from argv at all, so the assertion
        // is about the SURFACE: while it is unbuilt there is nothing that could
        // read the vocabulary, and once it is built there must be.
        let (code, text) = drive(dir.path(), &[via]);
        assert_eq!(
          code,
          Some(2),
          "`{}` declares a vocabulary for `intent {via}`, which is recorded here as unbuilt. If it \
           has been built, its declared values now need a reader -- decide who, then move this row: \
           {text}",
          slot.arg
        );
      }
    }
  }
}

/// **The one Enforced slot, asserted against the model rather than a literal.**
///
/// `wp rescope`'s six sizes are the enum's six, and the row is the only place
/// they are written down twice. If the table and `TShirt` part company, the CLI
/// refuses a size the row promises or accepts one it does not -- and neither
/// shows up in any other test, because the table's copy is documentation and the
/// enum's is behaviour.
#[test]
fn the_rescope_row_declares_exactly_the_sizes_the_model_has() {
  let declared = declared_in_table()
    .into_iter()
    .find(|(path, arg, _)| path == "wp rescope" && arg == "size")
    .map(|(_, _, values)| values)
    .expect("the row exists");
  let from_model: Vec<String> = TShirt::ALL.iter().map(enum_str).collect();
  assert_eq!(
    declared, from_model,
    "the row's permitted set and the model's vocabulary are the same six, in the same order -- the \
     row is read by an implementer and the enum is read by the parser, and a divergence is a \
     promise the tool does not keep"
  );
}

fn drive(root: &Path, argv: &[&str]) -> (Option<i32>, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  (
    out.status.code(),
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
  )
}
