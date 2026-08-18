//! AT-03.15 / AC-03.14: a write moves mtime on exactly the files whose bytes
//! changed.
//!
//! **A write moves mtime on EXACTLY the files whose bytes changed, and no
//! others** -- measured over the FILE ESTATE before and after, at every
//! user-visible verb that writes it.
//!
//! The criterion reached this form after four corrections from three nodes,
//! and each one is a constraint on this file:
//!
//! - **The SUBJECT is the verb, never a type.** An earlier guard sat on
//!   `views::write_all` -- correct, and reached by nothing, because every
//!   caller was a test. `view_determinism.rs` drove that function directly and
//!   was green throughout while the estate churned. **An internal subject lets
//!   the test reach PAST the thing being tested**, so nothing here calls
//!   `WriteSet` or `write_all`; it drives what a user runs.
//! - **The DENOMINATOR is the file estate, not the write set.** `WriteSet` is
//!   internal too, and a denominator taken from it would be blind to any write
//!   that never joins one. Walking the tree needs no internal type and catches
//!   a bypassing write for free.
//! - **The PROPERTY is `moved == changed`, not "the second run writes zero".**
//!   Run-twice-writes-zero is inapplicable to the 27 mutating verbs, where
//!   some files SHOULD change -- so it would exempt the criterion exactly
//!   where the churn is worst. It is a corollary here, not the test.
//! - **The EXTENT is every verb that writes.** One verb under-covers: six
//!   commit sites serve 31 user-visible verbs, and a row naming only `sync`
//!   would leave `todo update`, `upgrade` and the whole `apply` family green
//!   while churning.
//!
//! **No clock (D42).** Every file is aged to a fixed synthetic stamp between
//! the two observations, so a rewritten file carries whatever the filesystem
//! gave it and a skipped file still carries the constant exactly. A
//! sleep-based version passes vacuously on a coarse-resolution filesystem --
//! which is the very failure this criterion detects, so the instrument must
//! not be able to fail that way itself.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use common::{Fixture, sample_thread};
use intentsvcs::model::{AtStatus, TShirt};

/// 2001-09-09, and obviously synthetic on sight. An absolute constant makes
/// the assertion "the mtime is still exactly this" rather than "it is still
/// roughly where I put it".
fn aged() -> SystemTime {
  UNIX_EPOCH + Duration::from_secs(1_000_000_000)
}

/// Every file under the project root, with its bytes. The store lives in
/// `intent/.cache/` and is not a projected artefact, so it is excluded --
/// including it would report the DB's own writes as estate churn.
fn walk(root: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(root) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      if path.file_name().is_some_and(|n| n == ".cache") {
        continue;
      }
      walk(&path, out);
    } else if path.is_file() {
      out.push(path);
    }
  }
}

fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
  let mut paths = Vec::new();
  walk(root, &mut paths);
  paths
    .into_iter()
    .filter_map(|p| std::fs::read(&p).ok().map(|bytes| (p, bytes)))
    .collect()
}

/// Stamp every file in the estate with the synthetic constant.
fn age_everything(before: &BTreeMap<PathBuf, Vec<u8>>) {
  for path in before.keys() {
    std::fs::File::options()
      .write(true)
      .open(path)
      .expect("open to age")
      .set_modified(aged())
      .expect("age");
  }
}

/// The two sets the criterion compares, as project-relative paths.
struct Verdict {
  moved: Vec<String>,
  changed: Vec<String>,
  /// Files the verb CREATED. Not part of `moved == changed` -- a new file has
  /// no prior mtime to move and no prior bytes to differ from -- but it is
  /// evidence the verb FIRED, which is what the positive control needs.
  /// `issues add` writes exactly one new file and touches nothing else, so
  /// without this it reports "changed 0" and reads as a refused verb.
  created: Vec<String>,
}

fn verdict(root: &Path, before: &BTreeMap<PathBuf, Vec<u8>>) -> Verdict {
  let after = snapshot(root);
  let rel = |p: &Path| {
    p.strip_prefix(root)
      .unwrap_or(p)
      .to_string_lossy()
      .into_owned()
  };

  // A file the verb CREATED is neither moved nor changed: it has no prior
  // mtime to move and no prior bytes to differ from. Counting a creation as
  // churn would red every verb that legitimately adds a file, so both sets are
  // taken over paths present in BOTH observations.
  let mut moved = Vec::new();
  let mut changed = Vec::new();
  let created: Vec<String> = after
    .keys()
    .filter(|p| !before.contains_key(*p))
    .map(|p| rel(p))
    .collect();
  for (path, before_bytes) in before {
    let Some(after_bytes) = after.get(path) else {
      continue;
    };
    let still_aged = std::fs::metadata(path)
      .and_then(|m| m.modified())
      .map(|m| m == aged())
      .unwrap_or(false);
    if !still_aged {
      moved.push(rel(path));
    }
    if before_bytes != after_bytes {
      changed.push(rel(path));
    }
  }
  Verdict {
    moved,
    changed,
    created,
  }
}

/// Run `verb` against an aged estate and require `moved == changed`.
///
/// **`expect_change` is a POSITIVE CONTROL and it is not optional.** The first
/// run of this file had `st start` PASSING against a fixture whose threads
/// were already `Wip`: the gate refused the transition, the verb wrote
/// nothing, and `moved == changed == {}` held perfectly. **Emptiness from a
/// verb that never fired is indistinguishable from a verb that fired
/// cleanly** (ic's formulation), so a mutating verb must PROVE it mutated
/// before its own quietness counts for anything.
/// **`setup` runs BEFORE the snapshot and `verb` runs after, and the split is
/// load-bearing.** The first version ran both inside the measured window, and
/// three round-trip rows -- `hold` then `resume`, `unstart` then `start`,
/// `descope` then `rescope` -- came back RED with mtime moved and bytes
/// unchanged. **That was the harness measuring two verbs as one**: the pair
/// restores the original content, so `changed` is empty while `moved` is not.
/// The fix was correct throughout; the instrument was wrong, and it said so.
fn check_moves_only_what_changed(
  fx: &Fixture,
  label: &str,
  expect_change: bool,
  setup: impl FnOnce(&Fixture),
  verb: impl FnOnce(&Fixture),
) -> Result<(), String> {
  setup(fx);
  let before = snapshot(fx.root());
  if before.is_empty() {
    return Err(format!("{label}: precondition -- the estate is empty"));
  }
  age_everything(&before);

  verb(fx);

  let Verdict {
    moved,
    changed,
    created,
  } = verdict(fx.root(), &before);

  // **The control is "did the verb FIRE", so a creation counts.** Requiring a
  // CHANGED file would have failed `issues add`, whose whole effect is one new
  // file -- and the honest reading of that is not "the verb was refused".
  let fired = !changed.is_empty() || !created.is_empty();
  if fired != expect_change {
    return Err(format!(
      "  {label}: POSITIVE CONTROL failed -- expected the verb to write {}, and it changed {} file(s) and created {}. A quiet verb that was REFUSED proves nothing about a writer.",
      if expect_change {
        "something"
      } else {
        "nothing"
      },
      changed.len(),
      created.len()
    ));
  }

  if moved != changed {
    let extra: Vec<&String> = moved.iter().filter(|p| !changed.contains(p)).collect();
    return Err(format!(
      "  {label}: {} moved mtime, {} changed bytes, estate of {} -- moved-but-unchanged: {:?}",
      moved.len(),
      changed.len(),
      before.len(),
      extra
    ));
  }
  Ok(())
}

/// A fixture carrying enough estate for the denominator to be interesting:
/// several threads, so a one-thread mutation leaves most views untouched.
fn seeded() -> Fixture {
  let fx = Fixture::new();
  for id in ["ST0001", "ST0002", "ST0003"] {
    fx.write_thread(&sample_thread(id));
  }
  let mut facade = fx.facade();
  facade.sync_from_disk().expect("ingest the seeded canon");
  facade.sync_to_disk().expect("project it back");
  fx
}

// ---------------------------------------------------------------------------
// THE VERB ROSTER.
//
// **The criterion says EVERY user-visible verb that writes the estate, and it
// says the set must be PRINTED** -- so a verb added later and not driven is
// visible rather than silently absent. `CASES` below IS the driven set: the
// coverage test compares it against `DECLARED`, so the roster cannot claim
// coverage the file does not have.
//
// **Mechanical coverage is NOT sufficient and this file is the proof.** One
// skip at one writer does serve all six commit sites -- and on the first run
// `st start` still passed while writing nothing, because the fixture threads
// are seeded `Wip`, the gate refused the transition, and `moved == changed ==
// {}` held perfectly. **Each verb has its own preconditions, and a verb that
// cannot fire hands you a free green.** So every row sets up the state its
// verb needs, and every row carries a positive control.

/// **THE ROSTER IS DERIVED, NEVER AUTHORED, AND THAT IS THE POINT.**
///
/// An earlier version of this file carried a hand-written `const DECLARED:
/// &[&str]` of 31 verbs. vc held AT-03.15 red on it and was right: `surface/
/// dispatch-table.json` is that population's DECLARED HOME, and its own
/// `populations.why` records that the set *"was hand-written five times in one
/// week"* and that *"This block is the one home."* **The hand-written const was
/// the sixth copy, authored after that block.**
///
/// The coverage guard protected the roster's MEMBERS while nothing protected
/// the ROSTER -- a verb never added to it was invisible to the test, and
/// "30 of 31" read as the surface when it was only the list. **A probe whose
/// population cannot contain the failure it tests for, in the denominator.**
///
/// **The filter is `read_or_mutate`, and the choice is forced.** It is present
/// on all 113 entries; `side_effects` exists on 10, and the table itself rules
/// that reading its absence as "no side effects" is *"absence-as-meaning in the
/// one place it decides whether an agent may close a steel thread"*. A field
/// that cannot answer for 103 of 113 members cannot define a denominator.
///
/// The population is `families[].entries[]` UNION `new_surface[]`, INTERSECTED
/// with `populations.shipped` -- the union because `entries` alone omits the 8
/// top-level rows that all ship, the intersection because it includes rows
/// dispositioned `retire` that the binary does not contain.
fn shipped_mutators() -> Vec<String> {
  let path = testkit::repo_root().join("surface/dispatch-table.json");
  let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
    // **A missing table FAILS rather than skips.** The roster's whole value is
    // being derived; a skip here would restore the authored denominator with
    // no one seeing it happen.
    panic!(
      "the declared surface is unreadable at {}: {e}",
      path.display()
    )
  });
  let table: serde_json::Value = serde_json::from_str(&text).expect("the dispatch table is JSON");

  let shipped: Vec<&str> = table["populations"]["shipped"]
    .as_array()
    .expect("populations.shipped is an array")
    .iter()
    .filter_map(|v| v.as_str())
    .collect();

  let families = table["families"]
    .as_array()
    .expect("families is an array")
    .iter()
    .filter_map(|f| f["entries"].as_array())
    .flatten();
  let new_surface = table["new_surface"]
    .as_array()
    .expect("new_surface is an array")
    .iter();

  let mut out: Vec<String> = families
    .chain(new_surface)
    .filter(|e| e["read_or_mutate"].as_str() == Some("mutate"))
    .filter_map(|e| e["path"].as_str())
    .filter(|p| shipped.contains(p))
    .map(str::to_string)
    .collect();
  out.sort();
  out.dedup();
  out
}

/// Driven somewhere else, with the file named.
///
/// **Its own key rather than a member of `UNPROVEN`.** vc's ruling on buckets
/// 2 and 3 -- *"a key named for one reason cannot hold members admitted for
/// another"* -- applies here too: "covered elsewhere" is a claim with evidence
/// behind it, and "we have not established where this writes" is the absence of
/// one. Merging them would let a covered verb lend its credibility to an
/// unproven one.
const COVERED_ELSEWHERE: &[(&str, &str)] = &[(
  "upgrade",
  "unmigrated_project.rs -- it needs a pre-migration v2 project, not a v3 fixture",
)];

/// Writes files OUTSIDE the thread estate, **with the path it writes named**.
///
/// Named from the table's own `observed.side_effects`, never from the verb's
/// name. `agents sync` is the cautionary one: its `AGENTS.md.bak` was
/// **UNDECLARED until 2026-08-17**, which is the exact shape AC-03.14 exists
/// for -- a verb writing a file nobody had written down.
const OUT_OF_ESTATE: &[(&str, &str)] = &[
  (
    "agents sync",
    "AGENTS.md at the project root, plus AGENTS.md.bak beside it",
  ),
  (
    "lang init",
    "intent/llm/RULES-<lang>.md, ARCHITECTURE-<lang>.md, and config.json's languages array",
  ),
];

/// **THE DEBT, CARRIED BY NAME. Every one of these is a shipped mutator whose
/// writes are NOT ESTABLISHED IN EITHER DIRECTION.**
///
/// They are not excused and they are not counted as covered. The table cannot
/// answer for them -- `side_effects` is ABSENT rather than empty on 83 of 113
/// entries, and absence is not "no". **This list IS the minted form of "we have
/// not established this", and its membership is the exact work-list for adding
/// the field to the table.**
///
/// The test PRINTS its size and REFUSES TO GROW IT, so the debt is visible and
/// shrinking rather than silent.
const UNPROVEN: &[&str] = &[
  "agents generate",
  "agents init",
  "at lint",
  "backup",
  "bootstrap",
  "claude hook",
  "claude prime",
  "claude rules",
  "claude skills",
  "claude start",
  "claude subagents",
  "claude upgrade",
  "claude ws",
  "config set",
  "daemon",
  "ext new",
  "fileindex",
  "ingest",
  "init",
  "lang remove",
  "lang sync",
  "learn",
  "llm usage_rules",
  "mcp",
  "st bootstrap",
  "st repair",
  "st sync",
  "todo",
  "todo done",
  "todo list",
  "todo notdone",
  "todo toggle",
];

/// A driven case's label maps to the table path it exercises. `sync` is one
/// table entry driven from both directions.
fn table_path(label: &str) -> &str {
  match label {
    "sync --to-disk" | "sync --to-store" => "sync",
    other => other,
  }
}

type Case = (&'static str, fn(&Fixture), fn(&Fixture), bool);

const NOOP: fn(&Fixture) = |_| {};

fn cases() -> Vec<Case> {
  vec![
    // -- the whole-estate direction. Nothing changes: these re-project canon
    //    that is already on disk, which is the no-op shape.
    (
      "sync --to-disk",
      NOOP,
      |fx| {
        fx.facade().sync_to_disk().expect("sync to disk");
      },
      false,
    ),
    (
      "sync --to-store",
      NOOP,
      |fx| {
        fx.facade().sync_from_disk().expect("sync from store");
      },
      false,
    ),
    (
      "todo update",
      NOOP,
      |fx| {
        fx.facade().todo_update().expect("todo update");
      },
      false,
    ),
    // -- st. `seeded()` creates ST0001..ST0003, all `wip`, so a thread minted
    //    in setup is always ST0004.
    (
      "st new",
      NOOP,
      |fx| {
        fx.facade().st_new("a new thread").expect("st new");
      },
      true,
    ),
    (
      "st hold",
      NOOP,
      |fx| {
        fx.facade().st_hold("ST0001", "parked").expect("st hold");
      },
      true,
    ),
    (
      "st cancel",
      NOOP,
      |fx| {
        fx.facade()
          .st_cancel("ST0001", "superseded")
          .expect("st cancel");
      },
      true,
    ),
    (
      "st done",
      NOOP,
      |fx| {
        fx.facade().st_done("ST0001").expect("st done");
      },
      true,
    ),
    // `st.triage` and `st.start` are legal only from `triage`/`not-started`,
    // and `st new` lands a thread in `triage`.
    (
      "st triage",
      |fx| {
        fx.facade().st_new("fresh").expect("mint");
      },
      |fx| {
        fx.facade().st_triage("ST0004").expect("st triage");
      },
      true,
    ),
    (
      "st start",
      |fx| {
        fx.facade().st_new("fresh").expect("mint");
      },
      |fx| {
        fx.facade().st_start("ST0004").expect("st start");
      },
      true,
    ),
    (
      "st resume",
      |fx| {
        fx.facade().st_hold("ST0001", "parked").expect("to hold");
      },
      |fx| {
        fx.facade().st_resume("ST0001").expect("st resume");
      },
      true,
    ),
    (
      "st reopen",
      |fx| {
        fx.facade().st_done("ST0001").expect("to done");
      },
      |fx| {
        fx.facade()
          .st_reopen("ST0001", "AC added after close")
          .expect("st reopen");
      },
      true,
    ),
    (
      "st reinstate",
      |fx| {
        fx.facade()
          .st_cancel("ST0001", "superseded")
          .expect("to cancelled");
      },
      |fx| {
        fx.facade()
          .st_reinstate("ST0001", "back in scope")
          .expect("st reinstate");
      },
      true,
    ),
    // -- wp. The fixture carries WP 2 (done) and WP 3 (wip).
    (
      "wp new",
      NOOP,
      |fx| {
        fx.facade()
          .wp_new("ST0002", "a work package", TShirt::S)
          .expect("wp new");
      },
      true,
    ),
    (
      "wp rescope",
      NOOP,
      |fx| {
        fx.facade()
          .wp_rescope("ST0001", 3, TShirt::XL)
          .expect("wp rescope");
      },
      true,
    ),
    (
      "wp unstart",
      NOOP,
      |fx| {
        fx.facade().wp_unstart("ST0001", 3).expect("wp unstart");
      },
      true,
    ),
    (
      "wp done",
      NOOP,
      |fx| {
        fx.facade().wp_done("ST0001", 3).expect("wp done");
      },
      true,
    ),
    (
      "wp reopen",
      NOOP,
      |fx| {
        fx.facade()
          .wp_reopen("ST0001", 2, "more to do")
          .expect("wp reopen");
      },
      true,
    ),
    (
      "wp start",
      |fx| {
        fx.facade().wp_unstart("ST0001", 3).expect("to not-started");
      },
      |fx| {
        fx.facade().wp_start("ST0001", 3).expect("wp start");
      },
      true,
    ),
    // -- ac. AC-03.2 is the NON-TEST criterion and is seeded SATISFIED: a
    //    test-backed criterion's satisfaction is COMPUTED and `ac satisfy` is
    //    refused on it by design.
    (
      "ac unsatisfy",
      NOOP,
      |fx| {
        fx.facade()
          .ac_unsatisfy("ST0001", "AC-03.2")
          .expect("ac unsatisfy");
      },
      true,
    ),
    (
      "ac descope",
      NOOP,
      |fx| {
        fx.facade()
          .ac_descope("ST0001", "AC-03.1", "ST0002", None, None)
          .expect("ac descope");
      },
      true,
    ),
    (
      "ac withdraw",
      NOOP,
      |fx| {
        fx.facade()
          .ac_withdraw("ST0001", "AC-03.1", "never real", None)
          .expect("ac withdraw");
      },
      true,
    ),
    (
      "ac satisfy",
      |fx| {
        fx.facade()
          .ac_unsatisfy("ST0001", "AC-03.2")
          .expect("to unsatisfied");
      },
      |fx| {
        fx.facade()
          .ac_satisfy("ST0001", "AC-03.2", "measured")
          .expect("ac satisfy");
      },
      true,
    ),
    (
      "ac rescope",
      |fx| {
        fx.facade()
          .ac_descope("ST0001", "AC-03.1", "ST0002", None, None)
          .expect("to descoped");
      },
      |fx| {
        fx.facade()
          .ac_rescope("ST0001", "AC-03.1")
          .expect("ac rescope");
      },
      true,
    ),
    (
      "ac reinstate",
      |fx| {
        fx.facade()
          .ac_withdraw("ST0001", "AC-03.1", "never real", None)
          .expect("to withdrawn");
      },
      |fx| {
        fx.facade()
          .ac_reinstate("ST0001", "AC-03.1")
          .expect("ac reinstate");
      },
      true,
    ),
    // -- at. AT-03.1 is seeded GREEN, so `at green` on it would be the free
    //    green this file exists to refuse; every row drives a real transition.
    (
      "at red",
      NOOP,
      |fx| {
        fx.facade()
          .at_set("ST0001", "AT-03.1", AtStatus::Red)
          .expect("at red");
      },
      true,
    ),
    (
      "at na",
      NOOP,
      |fx| {
        fx.facade()
          .at_set("ST0001", "AT-03.1", AtStatus::Na)
          .expect("at na");
      },
      true,
    ),
    (
      "at green",
      |fx| {
        fx.facade()
          .at_set("ST0001", "AT-03.1", AtStatus::Red)
          .expect("to red");
      },
      |fx| {
        fx.facade()
          .at_set("ST0001", "AT-03.1", AtStatus::Green)
          .expect("at green");
      },
      true,
    ),
    // -- issues. The first issue in an empty project is 1.
    (
      "issues add",
      NOOP,
      |fx| {
        fx.facade()
          .issue_add("a defect", None, None)
          .expect("issues add");
      },
      true,
    ),
    (
      "issues close",
      |fx| {
        fx.facade()
          .issue_add("a defect", None, None)
          .expect("to exist");
      },
      |fx| {
        fx.facade().issue_close(1).expect("issues close");
      },
      true,
    ),
    (
      "issues open",
      |fx| {
        let mut f = fx.facade();
        f.issue_add("a defect", None, None).expect("to exist");
        f.issue_close(1).expect("to closed");
      },
      |fx| {
        fx.facade().issue_open(1).expect("issues open");
      },
      true,
    ),
  ]
}

/// **The criterion measured at every verb, with every failure reported at
/// once.** A first-failure abort would hide the shape of a regression across a
/// 30-verb roster: one writer serves them all, so if the skip breaks they ALL
/// break, and seeing that is the diagnosis.
#[test]
fn every_verb_moves_only_what_changed() {
  let mut failures: Vec<String> = Vec::new();
  for (label, setup, verb, expect_change) in cases() {
    let fx = seeded();
    if let Err(why) = check_moves_only_what_changed(&fx, label, expect_change, setup, verb) {
      failures.push(why);
    }
  }
  assert!(
    failures.is_empty(),
    "{} of {} driven verb(s) failed:\n{}",
    failures.len(),
    cases().len(),
    failures.join("\n")
  );
}

/// **THE ENUMERATION CLAUSE: every shipped mutator is in exactly one bucket,
/// and a new one is red by construction.**
///
/// This is what AT-03.15 was actually holding out for. The roster is derived
/// from the declared surface, so a verb added there without a case here cannot
/// pass -- which is the property a hand-authored list can never have.
#[test]
fn every_shipped_mutator_is_accounted_for() {
  let shipped = shipped_mutators();
  let driven: Vec<&str> = cases().iter().map(|(l, _, _, _)| table_path(l)).collect();

  let elsewhere: Vec<&str> = COVERED_ELSEWHERE.iter().map(|(v, _)| *v).collect();
  let out_of_estate: Vec<&str> = OUT_OF_ESTATE.iter().map(|(v, _)| *v).collect();

  println!(
    "verb coverage, derived from surface/dispatch-table.json:\n  \
     {} shipped mutator(s): {} driven here, {} covered elsewhere, {} out of estate, {} UNPROVEN",
    shipped.len(),
    shipped
      .iter()
      .filter(|v| driven.contains(&v.as_str()))
      .count(),
    elsewhere.len(),
    out_of_estate.len(),
    UNPROVEN.len()
  );

  // Nothing may claim a bucket it has no business in: a stale entry here is
  // the same rot as a missing one, pointing the other way.
  for list in [&driven, &elsewhere, &out_of_estate] {
    let stale: Vec<&&str> = list
      .iter()
      .filter(|v| !shipped.contains(&v.to_string()))
      .collect();
    assert!(
      stale.is_empty(),
      "bucketed but not a shipped mutator: {stale:?}"
    );
  }
  let stale: Vec<&&str> = UNPROVEN
    .iter()
    .filter(|v| !shipped.contains(&v.to_string()))
    .collect();
  assert!(
    stale.is_empty(),
    "listed UNPROVEN but not a shipped mutator: {stale:?}"
  );

  // In EXACTLY one: a verb in two buckets is a claim and its own contradiction.
  for verb in &shipped {
    let hits = [
      driven.contains(&verb.as_str()),
      elsewhere.contains(&verb.as_str()),
      out_of_estate.contains(&verb.as_str()),
      UNPROVEN.contains(&verb.as_str()),
    ]
    .iter()
    .filter(|b| **b)
    .count();
    assert_eq!(
      hits, 1,
      "{verb} is in {hits} buckets; every shipped mutator belongs to exactly one"
    );
  }

  // **The debt may shrink and must not grow.** A new shipped mutator silently
  // appended to UNPROVEN would be the authored roster all over again.
  assert!(
    UNPROVEN.len() <= 32,
    "UNPROVEN grew to {} -- a new shipped mutator needs a case, a named path, or a stated reason, not a longer debt list",
    UNPROVEN.len()
  );
}

/// The corollary, kept because it is the cheapest form to read and because it
/// is what the estate was first measured with -- 20 of 20 views moving on a
/// second sync. **It is NOT the criterion**: it grades only the no-op case, so
/// a row built on it would exempt the 27 `apply` verbs, which is exactly where
/// the churn was worst.
#[test]
fn a_second_sync_writes_nothing_at_all() {
  let fx = seeded();
  let before = snapshot(fx.root());
  age_everything(&before);
  fx.facade().sync_to_disk().expect("second sync");
  let Verdict {
    moved,
    changed,
    created,
  } = verdict(fx.root(), &before);
  assert!(
    moved.is_empty() && changed.is_empty() && created.is_empty(),
    "a no-op sync wrote {} file(s) and changed {}: {:?}",
    moved.len(),
    changed.len(),
    moved
  );
}
