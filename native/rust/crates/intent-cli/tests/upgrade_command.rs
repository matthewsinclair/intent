//! `intent upgrade` -- the migration door, driven through the real binary.
//!
//! The facade-level tests prove `Facade::upgrade` does the right thing when
//! called. They cannot prove the verb is REACHABLE, and reachability was the
//! whole of the gap: the dispatch table advertised `upgrade` in `--help` while
//! the binary answered exit 2, so every arm of the cutover measurement ran a
//! command that wrote nothing and came back byte-identical -- a green from a
//! door that never opened.
//!
//! So these tests drive the process. What they pin is narrow and each clause
//! earns its place:
//!
//!   1. the door opens, converts, and stamps the version LAST;
//!   2. running it again is byte-identical and SAYS it read from canon;
//!   3. a blocked estate is left exactly as it was found, stamp included.
//!
//! (3) is the one that would be expensive to get wrong, and it is the one a
//! facade test cannot make on its own: "nothing was written" is a claim about
//! the filesystem after a process exited, not about a return value.

use std::collections::BTreeMap;
use std::process::Command;

fn run(args: &[&str], cwd: &std::path::Path) -> (String, String, i32) {
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

/// A project declaring v2, which is what the migrator is for.
///
/// **The declared version is a v2 one and not 3.0.0**, unlike
/// `ingest_command`'s fixture: `ingest` reads v2 markdown into a project that
/// is already v3, and this verb is the transition itself. Stating it as v2 is
/// what makes the stamp observable -- against a v3 fixture the most important
/// assertion in this file would pass without the code doing anything.
///
/// **The version is a parameter because the floor is a behaviour**, and the
/// only honest way to test a floor is with the same estate either side of it.
fn v2_project(dir: &std::path::Path, version: &str) {
  std::fs::create_dir_all(dir.join("intent/.config")).expect("mkdir");
  std::fs::write(
    dir.join("intent/.config/config.json"),
    format!(
      "{{\"intent_version\":\"{version}\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}}\n"
    ),
  )
  .expect("write config");
}

/// Write a v2 steel thread the way v2.19 writes one.
fn v2_thread(dir: &std::path::Path, id: &str, status: &str) {
  let st = dir.join("intent/st").join(id);
  std::fs::create_dir_all(&st).expect("mkdir st");
  std::fs::write(
    st.join("info.md"),
    format!(
      "---\nverblock: \"14 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: {status}\nslug: a-slug\ncreated: 20260814\ncompleted:\n---\n\n# {id}: A thread\n\n## Objective\n\nShip it.\n\n## Context\n\nBecause.\n"
    ),
  )
  .expect("write info.md");
}

fn declared_version(dir: &std::path::Path) -> String {
  let text =
    std::fs::read_to_string(dir.join("intent/.config/config.json")).expect("read config.json");
  let value: serde_json::Value = serde_json::from_str(&text).expect("config.json is JSON");
  value["intent_version"]
    .as_str()
    .expect("intent_version is a string")
    .to_string()
}

/// Every file the migration is responsible for, keyed by relative path.
///
/// **`intent/.cache/` is excluded on D34**: the DB is per-machine truth and is
/// never committed, so it is not among the artefacts this estate hands to
/// anyone else. What is compared is what the migration promises to reproduce --
/// canon and the generated views.
///
/// **NOT because there is nothing there worth checking, and the first version
/// of this comment said so wrongly.** It called the store "rebuilt", cited D36
/// for it, and dismissed the difference as "bytes nobody has promised anything
/// about". **D36 is the ruling that the DB is NOT disposable** -- `rm
/// intent.db` is not an operation, "not as a unit of account in canon" -- so it
/// was cited as the ground for exactly what it forbids. D34 alone carries the
/// exclusion and always did; `rebuilt` was the word doing the damage.
///
/// **The difference is one knowable thing, measured by ic rather than assumed
/// by me**: `created_at` and `updated_at` default to `strftime`, so ~705 rows
/// record when the migration ran, and **two runs of a PERFECT migrator can
/// never produce the same database**. Normalise those stamps and the content is
/// identical to the byte.
///
/// So the store is a subject with a DIFFERENT PREDICATE -- content modulo
/// run-timestamps -- and that check belongs to `intentsvcs`, which owns the
/// store and may use rusqlite. **This crate may not (D06, held by
/// `dep_graph_guard.rs`)**, and shelling out to `sqlite3` would re-introduce
/// the binary dependency Intent bundles SQLite precisely to avoid.
fn tree(root: &std::path::Path) -> BTreeMap<String, Vec<u8>> {
  fn walk(root: &std::path::Path, at: &std::path::Path, out: &mut BTreeMap<String, Vec<u8>>) {
    for entry in std::fs::read_dir(at).expect("read_dir") {
      let path = entry.expect("entry").path();
      let rel = path
        .strip_prefix(root)
        .expect("under root")
        .to_string_lossy()
        .into_owned();
      if rel.starts_with("intent/.cache") {
        continue;
      }
      if path.is_dir() {
        walk(root, &path, out);
      } else {
        out.insert(rel, std::fs::read(&path).expect("read file"));
      }
    }
  }
  let mut out = BTreeMap::new();
  walk(root, root, &mut out);
  out
}

/// Report the FIRST path whose bytes differ, rather than dumping two trees.
fn first_difference(
  a: &BTreeMap<String, Vec<u8>>,
  b: &BTreeMap<String, Vec<u8>>,
) -> Option<String> {
  for (path, bytes) in a {
    match b.get(path) {
      None => return Some(format!("{path} disappeared on the second run")),
      Some(other) if other != bytes => {
        return Some(format!(
          "{path} changed on the second run ({} bytes -> {} bytes)",
          bytes.len(),
          other.len()
        ));
      }
      Some(_) => {}
    }
  }
  b.keys()
    .find(|p| !a.contains_key(*p))
    .map(|p| format!("{p} appeared on the second run"))
}

/// The door opens, converts the estate, and stamps the version.
#[test]
fn a_v2_estate_migrates_through_the_binary_and_the_stamp_lands() {
  let dir = tempfile::tempdir().expect("tempdir");
  v2_project(dir.path(), "2.19.0");
  v2_thread(dir.path(), "ST0001", "WIP");
  v2_thread(dir.path(), "ST0002", "Completed");

  assert_eq!(
    declared_version(dir.path()),
    "2.19.0",
    "the fixture must start as v2 or this test proves nothing"
  );

  let (_, err, code) = run(&["upgrade"], dir.path());
  assert_eq!(code, 0, "a clean v2 estate migrates: {err}");
  assert!(
    err.contains("2 thread(s)"),
    "it reports what it converted: {err}"
  );

  // The stamp. Written LAST, so its presence means every earlier step landed.
  assert_ne!(
    declared_version(dir.path()),
    "2.19.0",
    "the project still declares v2 after a successful migration, so the stamp \
     never landed -- and v2 tooling would go on treating this estate as its own"
  );

  // Canon, which is what the estate is FOR.
  for id in ["ST0001", "ST0002"] {
    assert!(
      dir
        .path()
        .join("intent/.canon/st")
        .join(format!("{id}.json"))
        .is_file(),
      "no committed canon for {id}"
    );
  }

  // The store's directory is ignored by the same operation that creates it,
  // so a migrated project cannot commit a database by accident.
  let ignored = std::fs::read_to_string(dir.path().join(".gitignore")).unwrap_or_default();
  assert!(
    ignored.lines().any(|l| l.trim() == "intent/.cache/"),
    "the runtime store is not gitignored after a migration: {ignored:?}"
  );
  assert!(
    !ignored.contains("*.db"),
    "the ignore rule is a PATH rule -- `*.db` would swallow a database the \
     user wants tracked: {ignored:?}"
  );
}

/// **Running it twice is byte-identical, and the second run says why.**
///
/// This is the property the cutover rests on. Under the fix-forward ruling an
/// interrupted migration is recovered by running it again, so a re-run that
/// changed anything would make the documented remedy destructive -- which it
/// was, until canon-wins landed: Phase A re-read what Phase B had written and
/// the estate grew by a few hundred bytes every time, monotonically, with every
/// count reconciling and nothing blocking.
///
/// The SIGKILL arm belongs to the interruption harness and is deliberately not
/// duplicated here. This is the plain re-run, which is the cheap half and the
/// one that regresses silently.
#[test]
fn running_it_twice_leaves_the_tree_byte_identical() {
  let dir = tempfile::tempdir().expect("tempdir");
  v2_project(dir.path(), "2.19.0");
  v2_thread(dir.path(), "ST0001", "WIP");
  v2_thread(dir.path(), "ST0002", "Completed");

  let (_, err1, code1) = run(&["upgrade"], dir.path());
  assert_eq!(code1, 0, "first run: {err1}");
  let after_first = tree(dir.path());

  // **Anti-vacuity.** Two empty trees compare equal, so the comparison below
  // is only worth running if the first one actually produced the canon.
  assert!(
    after_first.contains_key("intent/.canon/st/ST0001.json"),
    "the first run wrote no canon, so comparing the trees would prove nothing"
  );

  let (_, err2, code2) = run(&["upgrade"], dir.path());
  assert_eq!(code2, 0, "second run: {err2}");
  let after_second = tree(dir.path());

  assert_eq!(
    after_first.len(),
    after_second.len(),
    "the second run changed how many files exist"
  );
  assert!(
    first_difference(&after_first, &after_second).is_none(),
    "a re-run is not idempotent: {}",
    first_difference(&after_first, &after_second).unwrap_or_default()
  );

  // **It reports the re-run rather than presenting it as fresh work.** The
  // accretion was invisible precisely because every count reconciled and the
  // command looked like it had done the job each time.
  assert!(
    err2.contains("already migrated"),
    "the second run did not say it read from committed canon: {err2}"
  );
}

/// **A blocked estate is left exactly as it was found, stamp included.**
///
/// The refusal is structural -- the plan is an uncommitted `WriteSet` -- but
/// "nothing was written" is a claim about the filesystem after the process
/// exited, so it is asserted where it can actually be observed.
///
/// The stamp is the clause that matters most. A refusal that stamped anyway
/// would leave the estate declaring v3 with no canon: v2 refuses a project
/// from the future and v3 finds nothing to read, so the project would be
/// locked out of both toolchains at once with no tool left to repair it.
#[test]
fn a_blocked_migration_writes_nothing_and_does_not_stamp() {
  let dir = tempfile::tempdir().expect("tempdir");
  v2_project(dir.path(), "2.19.0");
  v2_thread(dir.path(), "ST0001", "WIP");
  // Residue in a LIVE thread, which blocks (hv's ruling: closed threads carry).
  let info = dir.path().join("intent/st/ST0001/info.md");
  let text = std::fs::read_to_string(&info).expect("read");
  std::fs::write(&info, text.replace("status: WIP", "status: Banana")).expect("write");

  let before = tree(dir.path());

  let (out, err, code) = run(&["upgrade"], dir.path());
  assert_eq!(
    code, 1,
    "a blocked migration is a verdict (1), never an unavailable tool (2) -- \
     consumers written against v2 read 2 as fail-open: {err}{out}"
  );

  assert_eq!(
    declared_version(dir.path()),
    "2.19.0",
    "a refused migration stamped the version anyway, which would lock the \
     estate out of v2 AND v3 at once"
  );
  assert!(
    !dir.path().join("intent/.canon/st/ST0001.json").exists(),
    "a refused migration wrote canon"
  );
  assert!(
    !dir.path().join("intent/.cache/intent.db").exists(),
    "a refused migration left a v3 store inside a v2 estate"
  );

  let after = tree(dir.path());
  assert!(
    first_difference(&before, &after).is_none(),
    "a refused migration changed the estate: {}",
    first_difference(&before, &after).unwrap_or_default()
  );
}

/// **A REFUSAL IS A WORK LIST, AND EVERY LINE OF IT APPEARS EXACTLY ONCE.**
///
/// `FacadeError::MigrationBlocked` carried `#[error("{0}")]` beside `#[from]`.
/// `#[from]` implies `#[source]`, so the whole classed report printed once as
/// the message and once again as its own cause -- summary line included.
/// Measured on this fixture: eleven lines for two findings, `refused 2
/// finding(s)` twice, the first occurrence mid-output where it reads as the end
/// of the list.
///
/// **The rule was already written down one level below and survived being
/// written down, because the violation used a different spelling.**
/// `Blocked::Residue` deliberately does not carry its `Refusal` as a
/// `#[source]` and says why at `migrate.rs:110` -- "a source here renders the
/// whole list twice and every residue count reads double". A reader checking
/// the outer variant against that comment looks for `#[source]` and does not
/// find one.
///
/// **Why it is worth a test rather than a fix.** The doubling is invisible in
/// the small: two findings look like a verbose error. **It scales with the
/// estate**, and the operator who most needs the list is the one with the
/// dirtiest estate -- ic's `show_tail` finding is the same hazard from the
/// display end, where a clipped work list sends someone to fix nine of ten and
/// hit the tenth on the next run.
///
/// The count is asserted, not the layout: a formatting change should not red
/// this, and a line coming back twice must.
#[test]
fn a_refusal_names_each_finding_exactly_once() {
  let dir = tempfile::tempdir().expect("tempdir");
  v2_project(dir.path(), "2.19.0");
  for (id, bad) in [("ST0001", "Banana"), ("ST0002", "Kumquat")] {
    v2_thread(dir.path(), id, "WIP");
    let info = dir.path().join("intent/st").join(id).join("info.md");
    let text = std::fs::read_to_string(&info).expect("read");
    std::fs::write(
      &info,
      text.replace("status: WIP", &format!("status: {bad}")),
    )
    .expect("write");
  }

  let (out, err, code) = run(&["upgrade"], dir.path());
  assert_eq!(code, 1, "premise: the estate must actually block");
  let report = format!("{err}{out}");

  for id in ["ST0001", "ST0002"] {
    assert_eq!(
      report.matches(&format!("intent/st/{id}/info.md")).count(),
      1,
      "{id} is named more than once, so the operator's work list is longer \
       than the work:\n{report}"
    );
  }
  assert_eq!(
    report.matches("refused 2 finding(s)").count(),
    1,
    "the summary count appears twice, and the first one reads as the end of \
     the list:\n{report}"
  );
}

/// **AT-10.1: a sub-floor estate is refused, and the SAME estate at the floor
/// is not.**
///
/// The row cites THIS file rather than a `migrate_floor.rs` (vc's ruling): the
/// citation describes where the test lives, and summoning an artefact into
/// existence because a row names it is the pointer driving the code.
///
/// The floor exists because 2.19.0 is where the acceptance-test row grammar
/// landed. An estate below it converted directly skips that migration, so its
/// rows arrive in v3 in a grammar v3 was never told about -- silently, because
/// nothing on the conversion path looks.
///
/// **Both halves, in one test, over one estate differing in one field.** The
/// refusal arm alone would pass just as happily against a migrator that
/// refused everything, and that is not a hypothetical failure mode: the defect
/// this closes was the exact mirror of it, a floor checked on the door that
/// READS and never on the door that WRITES. A test that cannot tell "refuses
/// correctly" from "refuses" is the same kind of instrument as the one that
/// missed the bug.
///
/// Found by vc driving the fleet through the committed door: Utilz declares
/// 2.18.0 and was converted clean -- 61 files, 9 threads, stamped, no
/// complaint. **Every estate the four of us built by hand declares 2.19.0,
/// because that is what our own repo declares and it is what we copy**, so
/// this arm had no possible input until a corpus nobody here authored supplied
/// one. A guard that cannot be reached and a guard that works are the same
/// green.
#[test]
fn an_estate_below_the_migration_floor_is_refused_and_one_at_the_floor_is_not() {
  let below = tempfile::tempdir().expect("tempdir");
  v2_project(below.path(), "2.18.0");
  v2_thread(below.path(), "ST0001", "Completed");
  let before = tree(below.path());

  let (out, err, code) = run(&["upgrade"], below.path());
  assert_eq!(code, 1, "a sub-floor estate is refused: {err}{out}");
  assert!(
    err.contains("2.18.0"),
    "the refusal names the version the project actually declares, because that \
     is what the operator has to go and change: {err}"
  );
  assert!(
    err.contains("intent@2"),
    "and the remedy is the TWO-HOP -- naming the v3 migrator here would send \
     the operator back to the command that just refused them: {err}"
  );

  assert_eq!(
    declared_version(below.path()),
    "2.18.0",
    "a refused migration stamped the version anyway"
  );
  assert!(
    !below.path().join("intent/.canon/st/ST0001.json").exists(),
    "a refused migration wrote canon"
  );
  assert!(
    first_difference(&before, &tree(below.path())).is_none(),
    "a refused migration changed the estate: {}",
    first_difference(&before, &tree(below.path())).unwrap_or_default()
  );

  // The control. Same estate, same threads, one field different.
  let at_floor = tempfile::tempdir().expect("tempdir");
  v2_project(at_floor.path(), "2.19.0");
  v2_thread(at_floor.path(), "ST0001", "Completed");

  let (_, err2, code2) = run(&["upgrade"], at_floor.path());
  assert_eq!(
    code2, 0,
    "the SAME estate at the floor must convert -- without this the arm above \
     passes against a migrator that refuses everything: {err2}"
  );
  assert!(
    at_floor
      .path()
      .join("intent/.canon/st/ST0001.json")
      .is_file(),
    "the control estate reported success and wrote no canon"
  );
}
