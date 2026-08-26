//! AT-03.19 / AC-03.14: **the CLI-level half of "a write moves mtime on
//! exactly the files whose bytes changed, and no others".**
//!
//! `intentsvcs`' `write_moves_only_what_changed.rs` (AT-03.15) is the other
//! half. It drives `Facade` methods, and AT-03.15's own note records why that
//! is a boundary rather than a backlog: of the verbs it could not reach,
//! **31 of 32 appear nowhere in the facade surface at all** -- `st repair`,
//! `st bootstrap`, `st sync` reach no `intentsvcs` path, and the `todo` family
//! resolves only into `intent-cli`'s `render.rs`/`spine.rs`. No number of extra
//! cases in that file can reach them. **This is the second instrument it named:
//! a CLI-level driver that runs the binary and snapshots the estate around it.**
//!
//! # The observable, named
//!
//! AT-03.15's note requires this file to name the observable it uses, because
//! two obvious ones are both wrong and vc measured why:
//!
//! - **"did any file appear"** trips on pure reads -- `st list --status all` is
//!   classified `read` and materialises `intent/.cache/intent.db` from a clean
//!   start, reproduced here.
//! - **"the tracked tree"** is blind to that same write, because `.cache` is
//!   gitignored (D29).
//!
//! The observable here is the **projected estate**: every file under the
//! project root EXCLUDING `intent/.cache/`, keyed `(path -> bytes, mtime)`.
//! Same definition as AT-03.15 and for the same reason -- the store is not a
//! projected artefact, so counting the DB's own writes as estate churn would
//! red every verb that reads. **And `.cache` is not merely dropped: each case
//! records whether the store appeared, so the write neither observable can see
//! is still observed.**
//!
//! # Why every case declares an exit code
//!
//! **A verb that does not run writes nothing, and "wrote nothing" is what
//! passing looks like.** A driver that only compares two snapshots goes green
//! against a binary that refuses, a typo'd subcommand, or a fixture the verb
//! cannot act on -- the vacuous green this estate keeps re-finding in new
//! clothes. So `Expect` carries the exit code AND whether the verb is required
//! to have written, and both are asserted. That is this file's form of ic's
//! rule: **a run that did not measure must read UNMEASURED, never green.**
//!
//! # What driving them actually found
//!
//! Ten verbs came here as "unproven". **Only three of them write.** The rest
//! are classified `mutate` on a surface that never drove them:
//!
//! - `st bootstrap`, `st repair` -- `rc=2`, *is a known command that is not
//!   implemented yet*. Declared, unwired.
//! - `at lint --fix` -- `rc=1`, *`at lint --fix` is not implemented in v3*. The
//!   whole reason `at lint` was classified `mutate` is a flag that does not
//!   exist; the bare form leaves the estate byte-identical.
//! - `todo` and `todo list` -- `rc=0`, and **with `intent/todo.md` ABSENT they
//!   print the view and do not create it**, which is exactly the write their
//!   own `--help` promises (*"generates it if absent"*). The verb that
//!   generates it is `todo update`.
//! - `todo notdone`, `todo toggle` -- `rc=1` in **every** state driven: a
//!   `triage` thread, a `wip` thread, a `not-started` WP, and a genuinely
//!   `Completed` thread. The message is *"reopens finished work, and a reopen
//!   must record why it happened"* and the verbs have no way to carry a reason,
//!   so they route to `st reopen` and never write. Unreachable by design,
//!   including in the one state they nominally serve.
//! - `ingest` -- `rc=0`, reads, and creates no store even from a storeless
//!   start.
//!
//! **That is a population defect in the dispatch table, not a debt in this
//! file, and it is reported rather than quietly re-bucketed**: seven shipped
//! `mutate` entries write nothing at all. Each is driven here and each
//! assertion goes RED the day it starts writing, which forces the re-bucket
//! instead of leaving an excuse behind.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

// ---------------------------------------------------------------------------
// the estate
// ---------------------------------------------------------------------------

/// Every file under the project root, with its bytes and its mtime.
///
/// `intent/.cache/` is excluded: the store is the durable SSOT, not a projected
/// artefact, and a verb that reads it writes it. Its presence is observed
/// separately by [`store_present`] so the exclusion is a scoping decision
/// rather than a blind spot.
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

fn snapshot(root: &Path) -> BTreeMap<PathBuf, (Vec<u8>, SystemTime)> {
  let mut paths = Vec::new();
  walk(root, &mut paths);
  paths
    .into_iter()
    .filter_map(|p| {
      let bytes = std::fs::read(&p).ok()?;
      let mtime = std::fs::metadata(&p).ok()?.modified().ok()?;
      Some((p, (bytes, mtime)))
    })
    .collect()
}

fn store_present(root: &Path) -> bool {
  root.join("intent/.cache/intent.db").is_file()
}

/// The two sets the criterion compares, plus the two it deliberately keeps out
/// of the comparison and still reports.
struct Verdict {
  moved: Vec<String>,
  changed: Vec<String>,
  /// Neither moved nor changed -- a new file has no prior mtime to move and no
  /// prior bytes to differ from. Kept because it is evidence the verb FIRED,
  /// which is what a "this verb must write" case needs.
  created: Vec<String>,
  deleted: Vec<String>,
  store_appeared: bool,
}

fn verdict(
  root: &Path,
  before: &BTreeMap<PathBuf, (Vec<u8>, SystemTime)>,
  store_before: bool,
) -> Verdict {
  let after = snapshot(root);
  let rel = |p: &Path| {
    p.strip_prefix(root)
      .unwrap_or(p)
      .to_string_lossy()
      .into_owned()
  };

  let mut moved = Vec::new();
  let mut changed = Vec::new();
  for (path, (bytes, mtime)) in before {
    let Some((now_bytes, now_mtime)) = after.get(path) else {
      continue;
    };
    if now_mtime != mtime {
      moved.push(rel(path));
    }
    if now_bytes != bytes {
      changed.push(rel(path));
    }
  }
  moved.sort();
  changed.sort();

  let mut created: Vec<String> = after
    .keys()
    .filter(|p| !before.contains_key(*p))
    .map(|p| rel(p))
    .collect();
  let mut deleted: Vec<String> = before
    .keys()
    .filter(|p| !after.contains_key(*p))
    .map(|p| rel(p))
    .collect();
  created.sort();
  deleted.sort();

  Verdict {
    moved,
    changed,
    created,
    deleted,
    store_appeared: !store_before && store_present(root),
  }
}

// ---------------------------------------------------------------------------
// the fixture
// ---------------------------------------------------------------------------

fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

fn copy_tree(from: &Path, to: &Path) {
  std::fs::create_dir_all(to).expect("mkdir");
  for entry in std::fs::read_dir(from).expect("read_dir").flatten() {
    let src = entry.path();
    let dst = to.join(entry.file_name());
    if src.is_dir() {
      copy_tree(&src, &dst);
    } else {
      std::fs::copy(&src, &dst).expect("copy");
    }
  }
}

/// **Built by the binary, never by hand.** A hand-written estate says nothing
/// about what the tool produces, and the verbs driven here re-project exactly
/// the artefacts `init` and `st new` laid down.
///
/// Six threads rather than one, because the criterion's failure mode is *the
/// verb rewrote everything to change two* -- and on a one-thread estate
/// "everything" and "what changed" are the same set, so the assertion would
/// hold against the very defect it exists to catch.
fn pristine() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "cli-estate-fixture"], root);
  assert_eq!(code, 0, "fixture init failed: {err}");
  for n in 1..=6 {
    let title = format!("Thread number {n}");
    let (_, err, code) = run(&["st", "new", &title], root);
    assert_eq!(code, 0, "fixture st new failed: {err}");
  }
  // ST0002 is the closeable one: `todo done` runs the acceptance gate, and a
  // fresh thread has an empty contract, so without this the verb refuses and
  // the case would prove nothing about its write.
  exempt(root, "ST0002");
  dir
}

/// Declare `acceptance: exempt` on one thread through canon + the store, which
/// is the estate's own mechanism for a deliberately contract-free unit.
fn exempt(root: &Path, id: &str) {
  let canon = root.join("intent/.canon/st").join(format!("{id}.json"));
  let text = std::fs::read_to_string(&canon).expect("read canon");
  let patched = text.replacen(
    "\"status\":",
    "\"acceptance\": \"exempt\",\n  \"status\":",
    1,
  );
  assert_ne!(
    text, patched,
    "canon for {id} had no status field to anchor on"
  );
  std::fs::write(&canon, patched).expect("write canon");
  let (_, err, code) = run(&["sync", "--to-store", id], root);
  assert_eq!(code, 0, "fixture sync --to-store {id} failed: {err}");
}

// ---------------------------------------------------------------------------
// the cases
// ---------------------------------------------------------------------------

/// What the verb is required to do. **Both limbs are asserted**, because a
/// snapshot comparison alone cannot tell "wrote nothing" from "never ran".
enum Expect {
  /// Exits 0 and leaves at least one changed or created file behind.
  Writes,
  /// Exits with this code and leaves the projected estate byte-identical.
  /// A non-zero code here is a REFUSAL that is part of the verb's contract,
  /// not a broken fixture -- each one names which in `why`.
  WritesNothing(i32),
}

/// The phrase `render::unwired` emits.
///
/// **Duplicated as a literal, deliberately, and the duplication is the point.**
/// `flag_reachability.rs` carries the same string with the same reasoning: if
/// the wording changes, every file asserting it must notice, and a shared
/// constant in one of them would make every other copy look derived. **No copy
/// is authoritative and none may be promoted to it** -- which is why this file
/// carries its own rather than importing one, and why retiring any single
/// holder of the string must leave at least two behind.
const UNWIRED_PHRASE: &str = "is a known command that is not implemented yet";

struct Case {
  /// The dispatch-table path this exercises, and the label the roster prints.
  verb: &'static str,
  args: &'static [&'static str],
  /// Fixture surgery this case needs, run on its own copy of the estate.
  prep: fn(&Path),
  expect: Expect,
  /// Text the verb must put on stderr.
  ///
  /// **An exit code alone does not identify a refusal.** `rc=2` is what the
  /// unimplemented arm returns AND what clap returns for a missing required
  /// argument, so a case that drove the wrong invocation would pass on the
  /// parser's refusal while claiming the verb's. This pins which one happened.
  must_say: Option<&'static str>,
  why: &'static str,
}

const NOOP: fn(&Path) = |_| {};

/// Remove `intent/todo.md`, so a verb whose help says *"generates it if
/// absent"* is driven in the state that claim is about. **Driving it with the
/// file present proves nothing: the generator and the reader look identical.**
fn without_todo(root: &Path) {
  let p = root.join("intent/todo.md");
  if p.exists() {
    std::fs::remove_file(&p).expect("remove todo.md");
  }
}

/// Remove the store, so a verb claimed to read can be checked for
/// materialising one.
fn without_store(root: &Path) {
  let p = root.join("intent/.cache");
  if p.exists() {
    std::fs::remove_dir_all(&p).expect("remove store");
  }
}

/// Neither the view nor the store, so `todo list` is driven in the state BOTH
/// of the wrong observables are wrong about at once.
fn without_todo_or_store(root: &Path) {
  without_todo(root);
  without_store(root);
}

/// `st.done` is not a legal transition from `triage`, so a `todo done` case on
/// a fresh thread measures the state machine refusing and nothing else.
///
/// **This prep exists because the case failed without it and the failure was
/// the point**: `todo done` was declared a writer, exited 1, and left the
/// estate untouched -- which a snapshot-only driver reports as a clean pass.
fn started(root: &Path) {
  let (_, err, code) = run(&["st", "start", "ST0002"], root);
  assert_eq!(code, 0, "prep st start failed: {err}");
}

/// Make the thread index stale so `st sync --write` has something to do.
/// **Scribbled rather than deleted**: a deleted file comes back as `created`
/// and never enters the moved/changed comparison, so the case would assert the
/// criterion on an empty set.
fn stale_index(root: &Path) {
  std::fs::write(
    root.join("intent/st/steel_threads.md"),
    "# Steel Threads\n\nstale content this file did not have\n",
  )
  .expect("scribble the index");
}

fn cases() -> Vec<Case> {
  vec![
    // -- the three that actually write -------------------------------------
    Case {
      verb: "st sync",
      args: &["st", "sync", "--write"],
      prep: stale_index,
      expect: Expect::Writes,
      must_say: None,
      why: "rewrites intent/st/steel_threads.md from the threads; the mutate is behind --write and the bare form is a dry run, so driving the bare form would prove nothing about the writer",
    },
    Case {
      verb: "todo update",
      args: &["todo", "update"],
      prep: without_todo,
      expect: Expect::Writes,
      must_say: None,
      why: "the todo family's ACTUAL mutator -- it creates intent/todo.md when absent, which is the write `todo` and `todo list` are documented to perform and do not",
    },
    Case {
      verb: "todo done",
      args: &["todo", "done", "ST0002"],
      prep: started,
      expect: Expect::Writes,
      must_say: None,
      why: "closes the thread through st done and regenerates; needs a closeable unit, which is why the fixture declares ST0002 acceptance-exempt",
    },
    // -- classified `mutate`, driven, writes nothing ------------------------
    Case {
      verb: "todo",
      args: &["todo"],
      prep: without_todo,
      expect: Expect::WritesNothing(0),
      must_say: None,
      why: "help says `Show intent/todo.md (generates it if absent)`; driven with it ABSENT it prints the view at rc=0 and does not create the file",
    },
    Case {
      verb: "todo list",
      args: &["todo", "list"],
      prep: without_todo_or_store,
      expect: Expect::WritesNothing(0),
      must_say: None,
      why: "same claim, same result -- and driven from a storeless start it MATERIALISES intent/.cache/intent.db, which is why this file observes the store separately rather than keying on `did any file appear`. That observation is REPORTED and deliberately not asserted: a build that stopped creating one here would be an improvement, and a pin would red this file for it",
    },
    Case {
      verb: "at lint",
      args: &["at", "lint", "ST0001"],
      prep: NOOP,
      expect: Expect::WritesNothing(0),
      must_say: None,
      why: "the bare form reports and leaves the estate byte-identical",
    },
    Case {
      verb: "at lint --fix",
      args: &["at", "lint", "ST0001", "--fix"],
      prep: NOOP,
      expect: Expect::WritesNothing(1),
      must_say: None,
      why: "`at lint --fix` is not implemented in v3 -- the flag that earns this verb its `mutate` classification does not exist, so the classification is unearned in BOTH arms",
    },
    Case {
      verb: "ingest",
      args: &["ingest"],
      prep: without_store,
      expect: Expect::WritesNothing(0),
      must_say: None,
      why: "Phase A reads and writes nothing INCLUDING no store; driven from a storeless start so the claim is measured rather than inherited",
    },
    Case {
      verb: "st bootstrap",
      args: &["st", "bootstrap"],
      prep: NOOP,
      expect: Expect::WritesNothing(2),
      must_say: Some(UNWIRED_PHRASE),
      why: "rc=2, `is a known command that is not implemented yet` -- declared on the surface, implemented by nothing",
    },
    Case {
      verb: "st repair",
      args: &["st", "repair"],
      prep: NOOP,
      expect: Expect::WritesNothing(2),
      must_say: Some(UNWIRED_PHRASE),
      why: "rc=2, same refusal, same reason",
    },
    Case {
      verb: "todo notdone",
      args: &["todo", "notdone", "ST0002"],
      prep: NOOP,
      expect: Expect::WritesNothing(1),
      must_say: None,
      why: "refuses in EVERY state driven -- triage thread, wip thread, not-started WP, and a genuinely Completed thread -- because a reopen must record a reason and this verb cannot carry one. Unreachable including in the state it nominally serves",
    },
    Case {
      verb: "todo toggle",
      args: &["todo", "toggle", "ST0002"],
      prep: NOOP,
      expect: Expect::WritesNothing(1),
      must_say: None,
      why: "same unconditional refusal, routed to st reopen",
    },
  ]
}

// ---------------------------------------------------------------------------
// the assertions
// ---------------------------------------------------------------------------

#[test]
fn every_cli_verb_moves_only_what_changed() {
  let base = pristine();
  let scratch = tempfile::tempdir().expect("tempdir");
  let mut failures: Vec<String> = Vec::new();
  let mut report: Vec<String> = Vec::new();

  for (i, case) in cases().iter().enumerate() {
    let root = scratch.path().join(format!("case{i:02}"));
    copy_tree(base.path(), &root);
    (case.prep)(&root);

    let store_before = store_present(&root);
    let before = snapshot(&root);
    assert!(
      !before.is_empty(),
      "{}: the fixture estate is empty, so nothing this case asserts can fail",
      case.verb
    );

    let (out, err, code) = run(case.args, &root);
    let v = verdict(&root, &before, store_before);

    let mut note = |msg: String| failures.push(format!("  {}: {msg}", case.verb));

    // The criterion itself.
    if v.moved != v.changed {
      note(format!(
        "moved != changed\n    moved-not-changed: {:?}\n    changed-not-moved: {:?}",
        v.moved
          .iter()
          .filter(|p| !v.changed.contains(p))
          .collect::<Vec<_>>(),
        v.changed
          .iter()
          .filter(|p| !v.moved.contains(p))
          .collect::<Vec<_>>(),
      ));
    }

    // The half a snapshot comparison cannot see.
    match case.expect {
      Expect::Writes => {
        if code != 0 {
          note(format!(
            "declared a writer, exited {code} -- UNMEASURED, not green\n    stderr: {}",
            err.lines().next().unwrap_or("")
          ));
        }
        if v.changed.is_empty() && v.created.is_empty() {
          note(
            "declared a writer and wrote nothing -- the case is vacuous and would pass against a verb that had been removed"
              .to_string(),
          );
        }
      }
      Expect::WritesNothing(expected) => {
        if let Some(phrase) = case.must_say
          && !err.contains(phrase)
        {
          note(format!(
            "exited {code} without saying {phrase:?} -- the rc is right and the reason is not the recorded one\n    stderr: {}",
            err.lines().next().unwrap_or("")
          ));
        }
        if code != expected {
          note(format!(
            "expected rc={expected}, got {code} -- the recorded reason no longer describes the verb\n    stdout: {}\n    stderr: {}",
            out.lines().next().unwrap_or(""),
            err.lines().next().unwrap_or("")
          ));
        }
        if !v.changed.is_empty() || !v.created.is_empty() || !v.deleted.is_empty() {
          note(format!(
            "is classified `mutate`, was measured writing NOTHING, and now writes: changed {:?} created {:?} deleted {:?} -- re-bucket it, do not relax this",
            v.changed, v.created, v.deleted
          ));
        }
      }
    }

    report.push(format!(
      "  {:<16} rc={code:<2} moved={:<3} changed={:<3} created={:<3} deleted={:<3} store_appeared={}",
      case.verb,
      v.moved.len(),
      v.changed.len(),
      v.created.len(),
      v.deleted.len(),
      v.store_appeared
    ));
  }

  // **The verb set is ENUMERATED AND PRINTED**, which the criterion requires in
  // as many words, so a verb added to the surface and not driven here is
  // visible rather than silently absent.
  println!(
    "AT-03.19 -- {} CLI verb(s) driven against the real binary over a {}-file projected estate:\n{}",
    cases().len(),
    snapshot(base.path()).len(),
    report.join("\n")
  );

  assert!(
    failures.is_empty(),
    "{} of {} driven CLI verb(s) failed:\n{}",
    failures.len(),
    cases().len(),
    failures.join("\n")
  );
}

/// **The join to AT-03.15**, so the two halves cannot drift apart in silence.
///
/// AT-03.15 carries the derived roster of shipped mutators and buckets every
/// one of them; the verbs driven HERE are cited there as covered. If a label in
/// that citation stops matching a verb driven here, the two files disagree
/// about what this file proves -- and the citation is the only thing linking
/// them, since neither imports the other.
#[test]
fn the_driven_set_is_stable_and_named() {
  let verbs: Vec<&str> = cases().iter().map(|c| c.verb).collect();
  let mut sorted = verbs.clone();
  sorted.sort_unstable();
  sorted.dedup();
  assert_eq!(
    sorted.len(),
    verbs.len(),
    "a verb is driven twice -- one case would be silently shadowing the other's reason"
  );
  for case in cases() {
    assert!(
      !case.why.is_empty(),
      "{} is driven with no stated reason -- a bucket entry without one is an excuse",
      case.verb
    );
  }
}
