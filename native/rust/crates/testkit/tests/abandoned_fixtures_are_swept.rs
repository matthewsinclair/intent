//! `AT-04.1` (`ST0073`), covering `AC-04.1`: fixture homes do not accumulate, and the sweep that
//! guarantees it runs at START rather than at exit.
//!
//! **THE POPULATION THIS WAS WRITTEN AGAINST: 902 abandoned directories in
//! `/tmp`, 133.7 MB**, across twelve name families from six creation sites.
//! Every one of those directories has a `Drop` that would have removed it. That
//! is the whole finding: `Drop` is the exit path, and the exit path is the one
//! that does not run when a test binary is killed.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

/// A pid that is certainly dead: spawn a process, reap it, keep its number.
fn a_dead_pid() -> u32 {
  let mut child = Command::new("/bin/sh")
    .args(["-c", "exit 0"])
    .spawn()
    .expect("spawn");
  let pid = child.id();
  child.wait().expect("reap");
  pid
}

fn plant(name: &str) -> PathBuf {
  let dir = PathBuf::from("/tmp").join(name);
  std::fs::create_dir_all(&dir).expect("plant a fixture");
  std::fs::write(dir.join("marker"), b"planted").expect("write into it");
  dir
}

/// **THE NAMING CONTRACT, CHECKED WITHOUT TOUCHING `/tmp`.**
///
/// The two exclusions are the load-bearing arms and neither is hypothetical.
/// `/tmp/intent` is the `in-session` gate's sentinel directory, carrying one
/// file per live Claude Code session on this machine; `/tmp/intentfiles.new` is
/// a stray file. **Both begin `intent`, so a prefix-only sweep would have
/// deleted the sentinel directory out from under every running session** -- and
/// that is precisely the sweep anyone would write first.
#[test]
fn the_naming_contract_admits_fixtures_and_refuses_what_is_not_one() {
  // Real names, copied from the live population rather than invented.
  assert_eq!(
    testkit::abandoned_fixture_pid("intent-fixture-browse-estate-86483-16"),
    Some(86483)
  );
  assert_eq!(
    testkit::abandoned_fixture_pid("intentd-proj-96099-19"),
    Some(96099)
  );
  assert_eq!(
    testkit::abandoned_fixture_pid("intentd-home-3078-5"),
    Some(3078)
  );

  // THE DESTRUCTIVE ONES.
  assert_eq!(
    testkit::abandoned_fixture_pid("intent"),
    None,
    "the in-session sentinel DIRECTORY"
  );
  assert_eq!(
    testkit::abandoned_fixture_pid("intentfiles.new"),
    None,
    "a stray file"
  );

  // A family name whose own tail is a word, not a counter.
  assert_eq!(
    testkit::abandoned_fixture_pid("intent-fixture-browse-wp-absent"),
    None
  );
  // Not ours at all.
  assert_eq!(testkit::abandoned_fixture_pid("something-123-4"), None);
}

/// **THE SWEEP, WITH THE ARM THAT MAKES THE OTHER ARM MEAN ANYTHING.**
///
/// "The abandoned directory is gone" is also what you get from a sweep that
/// deletes everything it can see, and that sweep would take out the fixtures of
/// every test run in progress on the machine -- including other developers' on a
/// shared box. So the live-pid directory MUST survive, and this asserts it in
/// the same call.
#[test]
fn a_dead_fixture_is_removed_and_a_live_one_survives_the_same_sweep() {
  let dead = plant(&format!("intent-fixture-sweepprobe-{}-0", a_dead_pid()));
  let live = plant(&format!(
    "intent-fixture-sweepprobe-{}-1",
    std::process::id()
  ));

  let report = testkit::sweep_abandoned_fixtures();
  assert!(
    !report.refused,
    "the live-pid set could not be read, so nothing was swept"
  );

  // Asserted on the DIRECTORY, not on the counts: a concurrently running suite
  // may have swept the same corpse a millisecond earlier, which would make the
  // count zero while the property holds perfectly.
  assert!(
    !dead.exists(),
    "an abandoned fixture survived the sweep: {}",
    dead.display()
  );
  assert!(
    live.exists(),
    "a LIVE fixture was deleted -- the sweep cannot tell live from dead"
  );

  assert!(
    report.examined > 0,
    "the sweep matched nothing at all, so it proves nothing"
  );
  let _ = std::fs::remove_dir_all(&live);
}

/// **`AC-04.1`'s OWN CLAUSE: A RUN LEAVES THE POPULATION NO LARGER THAN IT
/// FOUND IT** -- scoped to a family this test OWNS, and the scoping is the
/// honest part rather than a dodge.
///
/// **THE FIRST VERSION COUNTED ALL OF `/tmp` AND WAS RACY, WHICH IT PROVED BY
/// FAILING WITH `the population grew across a run: 0 -> 0`.** Between the
/// assertion and its own panic message the count moved, because five other test
/// binaries were creating and sweeping fixtures in the same directory at the
/// same time. **`/tmp` is not this test's to make claims about**, and a global
/// assertion here would have landed as an intermittent CI red whose cause is a
/// concurrently running sibling.
///
/// So the property is asserted over a probe family keyed on THIS process: plant
/// corpses that nothing else can create, sweep, require none to survive. That is
/// the mechanism that makes the global claim true, tested where it is
/// deterministic.
#[test]
fn a_run_leaves_no_corpse_of_its_own_behind() {
  let tag = format!("sweepgrowth-{}", std::process::id());
  let before = population_tagged(&tag);
  assert_eq!(before, 0, "the probe family is not unique to this process");

  for n in 0..3 {
    plant(&format!("intent-fixture-{tag}-{}-{n}", a_dead_pid()));
  }
  assert_eq!(
    population_tagged(&tag),
    3,
    "the probe did not grow its own family, so it measures nothing"
  );

  // The WORKER, not `sweep_once`: the hook is `Once`-guarded, so by this point
  // another test in this binary may already have consumed it and the call would
  // be a silent no-op. A test that cannot tell a no-op from a sweep is not one.
  testkit::sweep_abandoned_fixtures();

  assert_eq!(
    population_tagged(&tag),
    0,
    "a run left its own corpses behind: {} of 3 survived the sweep",
    population_tagged(&tag)
  );
}

fn population_tagged(tag: &str) -> usize {
  std::fs::read_dir("/tmp")
    .expect("read /tmp")
    .flatten()
    .filter(|e| {
      e.file_name()
        .to_str()
        .is_some_and(|n| n.contains(tag) && testkit::abandoned_fixture_pid(n).is_some())
    })
    .count()
}

/// **THE ANTI-ROT ARM: EVERY `/tmp` CREATION SITE SWEEPS AT START.**
///
/// A sweep wired into five of six sites is a sweep that does not work, and
/// nothing at run time would say so -- the sixth site's leak looks exactly like
/// the population the sweep is failing to clear. So this reads the sources.
///
/// **IT IS A SOURCE CHECK BECAUSE A RUNTIME CHECK PASSES ON THE BROKEN TREE
/// TOO:** `sweep_once` is idempotent per process, so once ANY site has called
/// it, a site that never calls it is indistinguishable from one that does.
#[test]
fn every_tmp_fixture_creation_site_sweeps_at_start() {
  let root = testkit::workspace_root();
  let mut sites = BTreeSet::new();
  let mut missing = BTreeSet::new();
  walk(&root.join("crates"), &mut |path: &std::path::Path| {
    let Ok(text) = std::fs::read_to_string(path) else {
      return;
    };
    if !text.contains("PathBuf::from(\"/tmp\")") {
      return;
    }
    let rel = path
      .strip_prefix(&root)
      .unwrap_or(path)
      .display()
      .to_string();
    sites.insert(rel.clone());
    if !text.contains("sweep_once()") {
      missing.insert(rel);
    }
  });

  assert!(
    sites.len() >= 6,
    "the walk found {} creation site(s); it found 6 when this was written, so it has gone blind",
    sites.len()
  );
  assert!(
    missing.is_empty(),
    "these create /tmp fixtures and never sweep at start: {missing:?}"
  );
}

fn walk(dir: &std::path::Path, f: &mut impl FnMut(&std::path::Path)) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for e in entries.flatten() {
    let p = e.path();
    if p.is_dir() {
      if p.file_name().is_some_and(|n| n == "target") {
        continue;
      }
      walk(&p, f);
    } else if p.extension().is_some_and(|x| x == "rs") {
      f(&p);
    }
  }
}
