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
  facade
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("ingest the seeded canon");
  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("project it back");
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
const COVERED_ELSEWHERE: &[(&str, &str)] = &[
  (
    "st attach",
    "cli_end_to_end.rs `st_attach_writes_an_attachments_content_and_refuses_what_it_cannot_carry` \
     -- the verb that closed AC-08.5's last field-axis gap. `Attachment.text` was writable through \
     `Facade::put` with no route on the mutation surface, so the criterion's first clause failed on \
     a field whose own refusal correctly said *there is no CLI verb for this today*. It is driven \
     at the CLI rather than here because what it writes is an attachment's CONTENT rather than a \
     field of a document, so the whole-row diff this file is built around does not describe it. \
     **Its spelling is PROVISIONAL and routed to hv** (vc authorised the capability and declined \
     the name): the criterion asks whether the field is settable through the mutation surface and \
     has no opinion on what the verb is called, so a rename does not move this row",
  ),
  (
    "st edit",
    "edit_prints_a_path_that_exists.rs -- and it ARRIVED in this census by being reclassified \
     rather than by being written. It was declared `read_or_mutate: read` while `Facade::edit` \
     calls `hydrate`, so it realised a thread's views and appended `STEELTHREAD:<id>` to the \
     TRACKED `.intentfiles` -- and on its rc=1 refusal path it did that while reporting that \
     nothing had happened. (ic reported it; vc corrected the row 2026-08-22.) That file drives \
     both halves the reclassification implies: `editing_pins_the_artefact_so_the_next_organize_\
     keeps_it` proves the write on the ALLOWED path, and `the_filename_refusal_writes_nothing_at_\
     all` proves its absence on the refused one. **A census keyed on a DECLARED field cannot see \
     a verb whose declaration is wrong**, which is why this row was invisible here for as long as \
     the field said `read` -- the bucket that was missing was not a bucket, it was the entry",
  ),
  (
    "wp cancel",
    "mutation_completeness.rs -- both the success walk and the UNMET guard walk drive it: \
     `wp.cancel` from every declared from-state, and its ReasonRecorded guard refused with a \
     blank justification. The write set is the same `set_wp_status` path `wp done` and \
     `wp reopen` already prove here, so a fourth copy would test the setter a third time \
     rather than test this verb",
  ),
  (
    "wp reinstate",
    "mutation_completeness.rs -- same pair of walks, from `cancelled`, its only declared \
     from-state",
  ),
  (
    "upgrade",
    "unmigrated_project.rs -- it needs a pre-migration v2 project, not a v3 fixture",
  ),
  (
    "organize",
    "organize_idempotent_mtime.rs -- ST0057 AC-04.4 IS this file's property for this verb, measured as MTIMES MOVED rather than as a content diff, and it carries the positive control that a zero-movement result needs. Driving it here as well would be a second expression of one claim, and the two would answer to different fixtures.",
  ),
  (
    "edit",
    "edit_prints_a_path_that_exists.rs -- the writes are `Facade::hydrate`'s and nothing else's, and that is the criterion rather than a shortcut: AC-05.3 says path-printing has ONE home, so `edit` realising anything itself WOULD BE the defect. `an_absent_artefact_is_realised_and_the_printed_path_exists` and `editing_pins_the_artefact_so_the_next_organize_keeps_it` drive both halves of what it writes -- the files and the manifest entry -- from a fixture that starts DEHYDRATED, so neither can pass against a verb that wrote nothing. NOT covered by `facade_hydrate.rs`: that proves hydrate's writes, and what is owed here is that `edit` performs them before it answers.",
  ),
  (
    "st sync",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- it has no facade method at all, so no case in THIS file can reach it. Driven at the CLI over a scribbled index, the only state in which it writes: moved == changed == {steel_threads.md}. NOT driven in the bare form, which is a dry run -- the mutate is behind --write, and a driver running the bare form proves nothing about the writer, which is the trap `at lint --fix` was already recorded as.",
  ),
  (
    "todo done",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- resolves only into intent-cli's render.rs/spine.rs. Driven on an acceptance-exempt thread, because on a fresh one `st.done` is not a legal transition from `triage` and the verb refuses: the case FAILED that way on its first run and both guards fired, which is the evidence that a snapshot-only driver would have reported it as a clean pass.",
  ),
  (
    "st hydrate",
    "facade_hydrate.rs -- `the_call_is_idempotent_in_what_it_returns`, `the_pin_is_idempotent` and `hydrating_something_already_on_disk_still_pins_it` are this criterion for this verb, driven at the primitive the CLI arm is two lines over. NOT organize_idempotent_mtime.rs, which measures the ESTATE-WIDE verb: hydrate runs a plan FILTERED to one artefact, so the whole-estate file covers the mechanism and not the scoping, and citing it would name a file that cannot fail when this verb regresses.",
  ),
];

/// **DECLARED ON THE SURFACE, IMPLEMENTED BY NOTHING -- so the writes are not
/// unproven, they are provably EMPTY.**
///
/// Its own key rather than a member of [`UNPROVEN`], by the same ruling that
/// split `COVERED_ELSEWHERE` off: *a key named for one reason cannot hold
/// members admitted for another.* `UNPROVEN` means **we have not established
/// what this writes**; these verbs write nothing and the binary says so, at
/// `rc=2`, with `is a known command that is not implemented yet` on stderr.
/// Filing a known zero as a debt would inflate the debt and hide the fact.
///
/// **AND THE MEMBERSHIP IS DRIVEN, NOT ASSERTED HERE.** The named file runs
/// each of these against the real binary and requires the refusal; the day one
/// is implemented it stops exiting 2 and that file goes RED, which forces the
/// re-bucket rather than leaving a live mutator sitting in an excuse list.
/// **It measures the CAPABILITY (does the binary refuse?) and never a NAME** --
/// the distinction ST0057 AC-08.5 paid for, where a pin grepping `facade.rs`
/// for `fn at_new` passed while `put` created both rows thirty lines away.
const DECLARED_BUT_UNWIRED: &[(&str, &str)] = &[
  (
    "st dehydrate",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- no `Facade::dehydrate` exists at all; only `hydrate` is built. **This row had TWO drivers for one window on 2026-08-21, on purpose: `declared_but_unwired.rs` held the claim, cc's driver landed BESIDE it, and only then did ic retire the original.** A gap in which nothing drives the claim is silent and every suite in it is green; an overlap is visible, self-healing, and fails loud if either driver is wrong. The overlap is spent and the row is single-driver again -- **recorded because the sequence is the reusable part, not this row's history: the adopter lands first, the retiree second, always.**",
  ),
  (
    "st bootstrap",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- `rc=2`, `is a known command that is not implemented yet`, and the projected estate byte-identical. Driven at the CLI because it reaches no `intentsvcs` path to drive.",
  ),
  (
    "st repair",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- same refusal, same measurement, same reason it cannot be driven here.",
  ),
  // **`issues hydrate` AND `issues dehydrate` LEFT THIS BUCKET BY LEAVING THE
  // SURFACE** (hv, 2026-08-20): issues are canon-and-store only, both rows are
  // retired in the dispatch table, so they are no longer shipped mutators and
  // there is nothing here to excuse.
  //
  // **This is the bucket working, and it is worth saying which half worked.**
  // The membership check fired on its own -- `bucketed but not a shipped
  // mutator: ["issues hydrate", "issues dehydrate"]` -- so the stale rows
  // announced themselves rather than sitting here as two excuses for commands
  // that no longer exist. An authored list with no derived denominator has no
  // way to notice that.
];

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
    "intent/.config/config.json's languages array, and NOTHING under intent/llm/ -- \
CORRECTED 2026-08-25 with the wiring (issue 0068). This string named RULES-<lang>.md and \
ARCHITECTURE-<lang>.md until v3's `lang init` shipped without the fan-out; it was read off v2's \
`observed.side_effects`, which is a correct record of v2 and was never a claim about this binary. \
A written-down file that no verb writes is the AC-03.14 class from the other direction, which this \
bucket's own doc names -- and it was sitting in the bucket that exists to name paths.",
  ),
  (
    "lang remove",
    "intent/.config/config.json's languages array only. MOVED FROM `UNPROVEN` 2026-08-25: it was \
admitted there because v2's remove DELETED intent/llm/RULES-<lang>.md and ARCHITECTURE-<lang>.md, \
so what it wrote was genuinely unestablished. v3's removes nothing but the array -- there is \
nothing installed to delete -- so it is now `lang init`'s exact inverse and belongs in `lang \
init`'s bucket. `UNPROVEN` means we have not established what this writes; leaving it there after \
establishing it would file a known answer as a debt.",
  ),
];

/// **SHIPPED, WIRED, CLASSIFIED `mutate` ON THE SURFACE -- AND DRIVEN, THEY
/// WRITE NOTHING.**
///
/// Its own key by the same ruling that split the others: *a key named for one
/// reason cannot hold members admitted for another.* These are not covered
/// elsewhere (nothing proves their writes, because there are none), not
/// declared-but-unwired (they run, and several exit 0), and emphatically not
/// UNPROVEN -- **the measurement exists and it is negative.** Filing a measured
/// zero as a debt would inflate the debt and hide the finding.
///
/// **THE FINDING IS A POPULATION DEFECT IN THE DISPATCH TABLE, NOT A DEBT IN
/// THIS FILE.** Six of the 64 entries the derived roster calls shipped mutators
/// write nothing at all, and two of them are documented in their own `--help`
/// as performing a write that driving them shows does not happen. That is the
/// AC-03.14 class -- *a verb writing a file nobody had written down* -- arriving
/// from the other direction: **a file written down that no verb writes.**
///
/// **MEMBERSHIP IS DRIVEN, NEVER ASSERTED HERE.** The named file runs each of
/// these against the real binary and requires both the exit code AND the
/// unchanged estate. The day one starts writing, that file goes RED and forces
/// the re-bucket, rather than leaving an excuse sitting here.
const MUTATE_BUT_WRITES_NOTHING: &[(&str, &str)] = &[
  (
    "at lint",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- BOTH arms driven. Bare: rc=0, estate byte-identical. `--fix`, the flag that earns the `mutate` classification: `rc=1`, `at lint --fix is not implemented in v3`. The classification rests on a flag that does not exist.",
  ),
  (
    "ingest",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- Phase A reads and writes nothing INCLUDING no store, driven from a storeless start rather than inherited. Its paired control is `todo list` in the same run and the same condition, which DOES materialise one -- so the store observation is not stuck-false.",
  ),
  (
    "todo",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- `--help` says `Show intent/todo.md (generates it if absent)`; driven with the file ABSENT it prints the view at rc=0 and does not create it. The verb that generates it is `todo update`, which IS driven in this file.",
  ),
  (
    "todo list",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- same documented write, same absence of it. It does materialise intent/.cache/intent.db, which is a write neither obvious observable can see: `did any file appear` trips on pure reads that do the same thing, and the tracked tree is blind to it because .cache is gitignored (D29). Observed and reported there, deliberately not pinned.",
  ),
  (
    "todo notdone",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- rc=1 in EVERY state driven: a triage thread, a wip thread, a not-started WP, and a genuinely Completed thread. A reopen must record why it happened and this verb cannot carry a reason, so it routes to `st reopen` and never writes. Unreachable including in the one state it nominally serves -- which is why the refusal is not a fixture problem.",
  ),
  (
    "todo toggle",
    "intent-cli/tests/cli_write_moves_only_what_changed.rs -- the same unconditional refusal, driven on a NOT-STARTED WP as well, where `reopens finished work` cannot be true. State-independent by construction.",
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
  "init",
  "learn",
  "llm usage_rules",
  "mcp",
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
        fx.facade()
          .sync_to_disk(&intentsvcs::sync::Scope::All)
          .expect("sync to disk");
      },
      false,
    ),
    (
      "sync --to-store",
      NOOP,
      |fx| {
        fx.facade()
          .sync_from_disk(&intentsvcs::sync::Scope::All)
          .expect("sync from store");
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
  let unwired: Vec<&str> = DECLARED_BUT_UNWIRED.iter().map(|(v, _)| *v).collect();
  let writes_nothing: Vec<&str> = MUTATE_BUT_WRITES_NOTHING.iter().map(|(v, _)| *v).collect();

  // **EVERY FIGURE IS AN INTERSECTION WITH `shipped`, AND THE LINE STATES ITS
  // OWN ARITHMETIC** (vc, 2026-08-20, who caught the old form contradicting the
  // assertion two lines below it).
  //
  // The old line mixed one intersected count with three raw list lengths under
  // a single denominator and never said whether they summed. On the run that
  // found this it printed `69 shipped mutator(s): 29 driven here, 2 covered
  // elsewhere, 2 out of estate, 32 UNPROVEN` -- **65 presented as 69** -- while
  // the assertion below reported ONE unbucketed verb. Four were unbucketed.
  // Neither line was false; nothing read both.
  //
  // **A SUMMARY THAT CANNOT DISAGREE WITH ITSELF IS THE FIX, NOT A BIGGER
  // NUMBER.** So the tally is printed with its total and the shortfall is named
  // rather than left to subtraction -- this is the estate's own recurring class,
  // a count of containers standing in for a count of contents, and it had got
  // inside the instrument that exists to measure coverage.
  let tally = |bucket: &[&str]| {
    shipped
      .iter()
      .filter(|v| bucket.contains(&v.as_str()))
      .count()
  };
  let (d, e, o, u, w, n) = (
    tally(&driven),
    tally(&elsewhere),
    tally(&out_of_estate),
    tally(&unwired),
    tally(&writes_nothing),
    tally(UNPROVEN),
  );
  let unbucketed: Vec<&str> = shipped
    .iter()
    .filter(|v| {
      ![
        &driven,
        &elsewhere,
        &out_of_estate,
        &unwired,
        &writes_nothing,
        &UNPROVEN.to_vec(),
      ]
      .iter()
      .any(|b| b.contains(&v.as_str()))
    })
    .map(String::as_str)
    .collect();

  println!(
    "verb coverage, derived from surface/dispatch-table.json:\n  \
     {} shipped mutator(s) = {d} driven here + {e} covered elsewhere + {o} out of estate \
     + {u} declared-but-unwired + {w} classified-mutate-but-writes-nothing + {n} UNPROVEN = {}{}",
    shipped.len(),
    d + e + o + u + w + n,
    if unbucketed.is_empty() {
      String::new()
    } else {
      format!(
        "\n  {} in NO bucket, named rather than counted: {unbucketed:?}",
        unbucketed.len()
      )
    }
  );

  // Nothing may claim a bucket it has no business in: a stale entry here is
  // the same rot as a missing one, pointing the other way.
  for list in [
    &driven,
    &elsewhere,
    &out_of_estate,
    &unwired,
    &writes_nothing,
  ] {
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
  //
  // **EVERY OFFENDER IS REPORTED, NOT THE FIRST** (vc, 2026-08-20). The old form
  // asserted inside the loop, so it aborted on the first verb it found and named
  // ONE -- while the summary line above it proved FOUR were unbucketed. A reader
  // fixing the named verb would have re-run and met the second, then the third:
  // four rounds to learn a fact the instrument already held in full. Same
  // reasoning as `every_verb_moves_only_what_changed`'s own comment thirty lines
  // up -- **one writer serves them all, so the SHAPE of the failure is the
  // diagnosis** -- which this loop was the last one in the file not to honour.
  let misfiled: Vec<String> = shipped
    .iter()
    .filter_map(|verb| {
      let hits: Vec<&str> = [
        ("driven here", driven.contains(&verb.as_str())),
        ("covered elsewhere", elsewhere.contains(&verb.as_str())),
        ("out of estate", out_of_estate.contains(&verb.as_str())),
        ("declared-but-unwired", unwired.contains(&verb.as_str())),
        (
          "classified-mutate-but-writes-nothing",
          writes_nothing.contains(&verb.as_str()),
        ),
        ("UNPROVEN", UNPROVEN.contains(&verb.as_str())),
      ]
      .iter()
      .filter(|(_, hit)| *hit)
      .map(|(name, _)| *name)
      .collect();
      (hits.len() != 1).then(|| format!("  {verb}: in {} bucket(s) {hits:?}", hits.len()))
    })
    .collect();
  assert!(
    misfiled.is_empty(),
    "{} of {} shipped mutator(s) are not in exactly one bucket:\n{}",
    misfiled.len(),
    shipped.len(),
    misfiled.join("\n")
  );

  // **The debt may shrink and must not grow -- AND THE BOUND MOVES DOWN WITH
  // IT, or the shrink undoes itself in silence.**
  //
  // It was 32, and 32 stopped being a bound the moment ten verbs left: a
  // ratchet left at its old value permits the exact regrowth it was installed
  // to prevent, while still reading as a ratchet. **Tightening is part of
  // discharging, not a separate tidy-up.**
  //
  // The ten that left were the whole of AT-03.15's stated hold -- the
  // thread-estate verbs by family -- and they left DRIVEN, at the CLI, in
  // `intent-cli/tests/cli_write_moves_only_what_changed.rs`. The 22 that remain
  // are the ones that note already ruled out of this criterion's subject:
  // they write `~/.claude`, `~/.intent/ext/`, config, a fresh project, or they
  // serve. **That is a table-completeness debt, and it is not this row.**
  assert!(
    UNPROVEN.len() <= 22,
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
  fx.facade()
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("second sync");
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
