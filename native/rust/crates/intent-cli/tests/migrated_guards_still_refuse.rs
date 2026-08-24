//! AT-10.10 / AC-10.10: a migrated project's whiteboard guards still REFUSE
//! what they refused before the swap.
//!
//! **Asserted by driving the shipped hook and watching it BLOCK, never by
//! checking that the guard files are present.** Issue 0042 is precisely a case
//! where the files existed, were correct, and were never reached: the hook
//! resolved `INTENT_HOME` through `intent info`, v3 answered with nothing, and
//! both guards took the fail-open branch. Every file was in place throughout.
//!
//! **THE REFUSAL IS THE ASSERTION.** A test that stages a good artefact and
//! watches the commit succeed passes with both guards switched off, which is
//! AC-10.9's blind spot and the entire reason this row exists. The good-artefact
//! case is still here, but as a CONTROL rather than as the evidence: without it
//! a guard that refuses everything would look identical to one that works.
//!
//! **SECOND ARM: the resolver, not just the outcome.** The hook's fail-open
//! branch is correct behaviour; what moved in 0042 was its TRIGGER. A test that
//! only checks the commit was blocked will one day pass because of a third
//! thing nobody has thought of -- a malformed board, a missing git binary, a
//! guard that errors for an unrelated reason. So the resolution is driven
//! directly, using the extraction lifted OUT OF THE SHIPPED HOOK at test time
//! rather than retyped here, because a copy of a `sed` expression is a second
//! implementation that drifts from the one that runs.
//!
//! **Why this test lives in `intent-cli` and not `intentsvcs`** (vc, 2026-08-17,
//! on dc's measurement): `CARGO_BIN_EXE_intent` is set only for integration
//! tests of the package that DECLARES the binary. `intentsvcs` declares no
//! `[[bin]]` target and does not dev-depend on `intent-cli` -- and it must not,
//! because `dep_graph_guard.rs` enforces that direction. A test that cannot
//! invoke the real binary could satisfy the resolver arm only with a stub, and a
//! stub tests the hook's PARSER while calling it the resolver.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The Intent install this test drives the hook against -- the repository this
/// crate is compiled from. Derived from `CARGO_MANIFEST_DIR` rather than from
/// the current directory, which `cargo test` does not promise.
fn intent_home() -> PathBuf {
  let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  for _ in 0..4 {
    p.pop();
  }
  assert!(
    p.join("lib/templates/hooks/pre-commit.sh").is_file(),
    "could not locate the Intent install from CARGO_MANIFEST_DIR; looked at {}",
    p.display()
  );
  p
}

/// A fake INSTALL, because that is the only shape the resolver answers to.
///
/// **`intent info` IGNORES `INTENT_HOME` in the environment. Measured, not
/// assumed**: the same binary reports `<not set>` with the variable set and with
/// it absent, and reports a real path only when it sits inside an install. It
/// self-locates by walking up from its own symlink-resolved executable path, so
/// the value the hook reads is a fact about WHERE THE BINARY LIVES.
///
/// The first version of this test set `INTENT_HOME` in the child environment and
/// passed. It passed because `cargo test` puts the binary under `target/` INSIDE
/// the repository, so walking up found a tree with `lib/templates/` in it -- and
/// it failed the moment `CARGO_TARGET_DIR` moved that build elsewhere. **The
/// environment variable was doing nothing the entire time; the layout was doing
/// all of it.** A fixture that passes for a reason unrelated to what it tests.
///
/// So the rig builds the layout instead: a real copy of the binary at
/// `<install>/bin/intent` -- a copy and not a symlink, since the resolver walks
/// up from the RESOLVED path and a link would land back in the target dir -- and
/// `<install>/lib` pointing at the shipped tree, so the guards this drives are
/// the shipped guards rather than fixtures of them.
fn fake_install(base: &Path, repo: &Path) -> PathBuf {
  let install = base.join("install");
  fs::create_dir_all(install.join("bin")).expect("install bin");
  fs::copy(env!("CARGO_BIN_EXE_intent"), install.join("bin/intent"))
    .expect("copy the binary under test");
  std::os::unix::fs::symlink(repo.join("lib"), install.join("lib"))
    .expect("point at the shipped lib");
  install
}

/// Run a JUST-COPIED binary, retrying while Linux reports it busy.
///
/// **ETXTBSY IS A PROPERTY OF THIS HARNESS, NOT OF THE THING UNDER TEST.**
/// `fs::copy` closes its own destination handle, but this test binary is
/// multi-threaded and several of its tests fork (`Command::output`). A child
/// forked between another thread's open and close inherits that write fd, and
/// between `fork` and `execve` it still holds it -- Linux refuses to `execve`
/// a file any process has open for writing. macOS does not enforce that, which
/// is exactly why the macOS leg stayed green while ubuntu reddened on
/// `Os { code: 26, kind: ExecutableFileBusy }` (CI run 32718512776).
///
/// Bounded retry rather than a mutex: the window is microseconds, it belongs to
/// the harness, and serialising these tests would slow them to buy determinism
/// this assertion does not need. A retry that never succeeds still fails, and
/// says why.
///
/// Matched on `raw_os_error() == 26` rather than `ErrorKind::ExecutableFileBusy`
/// so this does not depend on that variant's stabilisation.
fn output_retrying_busy(mut build: impl FnMut() -> Command, what: &str) -> Output {
  const ETXTBSY: i32 = 26;
  let mut last = String::new();
  for _ in 0..100 {
    match build().output() {
      Ok(out) => return out,
      Err(e) if e.raw_os_error() == Some(ETXTBSY) => {
        last = e.to_string();
        std::thread::sleep(std::time::Duration::from_millis(20));
      }
      Err(e) => panic!("{what}: {e}"),
    }
  }
  panic!(
    "{what}: still busy after 100 attempts over ~2s ({last}) -- that is no longer a fork race"
  );
}

fn git(root: &Path, args: &[&str]) -> Output {
  Command::new("git")
    .args(args)
    .current_dir(root)
    .output()
    .expect("run git")
}

/// Commit through the SHIPPED hook, with the real binary reachable as `intent`
/// from an install the resolver can actually answer about.
fn commit_through_hook(root: &Path, install: &Path, msg: &str) -> Output {
  let path = format!(
    "{}:{}",
    install.join("bin").display(),
    std::env::var("PATH").unwrap_or_default()
  );
  Command::new("git")
    .args(["commit", "-m", msg])
    .current_dir(root)
    .env("PATH", path)
    .output()
    .expect("run git commit")
}

fn combined(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// A project that has been migrated: it has a whiteboard, a git repository, and
/// the shipped pre-commit hook installed the way the installer installs it.
fn migrated_project_without_config(home: &Path) -> tempfile::TempDir {
  project(home, false)
}

fn migrated_project(home: &Path) -> tempfile::TempDir {
  project(home, true)
}

fn project(home: &Path, with_config: bool) -> tempfile::TempDir {
  let td = tempfile::tempdir().expect("tempdir");
  let root = &td.path().join("repo");
  fs::create_dir_all(root).expect("repo dir");
  let install = fake_install(td.path(), home);

  assert!(git(root, &["init", "-q", "."]).status.success(), "git init");
  assert!(git(root, &["config", "user.email", "t@t"]).status.success());
  assert!(git(root, &["config", "user.name", "t"]).status.success());
  assert!(
    git(root, &["config", "commit.gpgsign", "false"])
      .status
      .success()
  );

  // The whiteboard is what makes the guards apply at all: they are opt-in by the
  // presence of this directory, so a project without one must see no change.
  fs::create_dir_all(root.join("intent/whiteboard/dc")).expect("whiteboard dir");
  fs::write(root.join("README.md"), "base\n").expect("seed");

  // AND A PROJECT CONFIG, WHICH IS NOT SCAFFOLDING -- IT IS LOAD-BEARING, AND
  // THIS TEST FOUND OUT BY OMITTING IT. The shipped hook returns `exit 0` when
  // `intent/.config/config.json` is absent, forty lines ABOVE the whiteboard
  // block, and the comment introducing that block says the guards "Run BEFORE
  // the critic". They do not. A board with no config gets no guards at all and
  // no message saying so -- an exit written when there was one arm is a claim
  // that the run is over. Reported rather than fixed here: the hook ships to
  // every consumer. A migrated project keeps its config (v3 reads it in
  // project.rs, migrate.rs and backup.rs), so this row's own population is not
  // the one exposed -- but a fixture without it tests the exit, not the guard.
  if with_config {
    fs::create_dir_all(root.join("intent/.config")).expect("config dir");
    fs::write(
      root.join("intent/.config/config.json"),
      "{\n  \"languages\": []\n}\n",
    )
    .expect("config");
  }

  let hook_src = home.join("lib/templates/hooks/pre-commit.sh");
  let hook_dst = root.join(".git/hooks/pre-commit");
  fs::create_dir_all(root.join(".git/hooks")).expect("hooks dir");
  fs::copy(&hook_src, &hook_dst).expect("install the shipped hook");
  let mut perms = fs::metadata(&hook_dst).expect("hook meta").permissions();
  std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
  fs::set_permissions(&hook_dst, perms).expect("chmod the hook");

  assert!(git(root, &["add", "-A"]).status.success(), "stage the seed");
  let seed = commit_through_hook(root, &install, "seed");
  assert!(
    seed.status.success(),
    "the seed commit must pass -- if it does not, every refusal below proves nothing:\n{}",
    combined(&seed)
  );
  td
}

fn board(stamp: &str) -> String {
  format!(
    "---\nnode: dc\nname: DevX Claude\nrole: worker\nsession_id: none\nheartbeat_at: {stamp}\nstatus: active\nfocus: \"driving the guard\"\nclaims: []\n---\n\n# DevX Claude (dc)\n\n## DOING\n\n- nothing\n"
  )
}

#[test]
fn a_bad_board_stamp_is_refused_by_the_shipped_hook() {
  let home = intent_home();
  let td = migrated_project(&home);
  let root = &td.path().join("repo");

  // The BAD artefact: a heartbeat with no trailing `Z`. Check B is syntactic and
  // exact -- it consults no clock and has no tolerance -- so this refusal cannot
  // become flaky with the passage of time, which matters for a test that will
  // outlive everyone who read it.
  fs::write(
    root.join("intent/whiteboard/dc/wip.md"),
    board("2026-08-17 12:00"),
  )
  .expect("write board");
  assert!(git(root, &["add", "-A"]).status.success());

  let out = commit_through_hook(
    root,
    &td.path().join("install"),
    "wb(dc): a stamp with no Z",
  );
  let text = combined(&out);

  assert!(
    !out.status.success(),
    "THE COMMIT WAS NOT REFUSED. A guard that stopped stopping is exactly issue 0042, \
     and the files being present says nothing about it. The hook said:\n{text}"
  );
  assert!(
    text.contains("Z"),
    "refused, but not visibly by the clock guard -- a refusal for a third reason \
     would satisfy the outcome and miss the point:\n{text}"
  );
}

#[test]
fn a_board_with_no_project_config_is_still_guarded() {
  // THE ORDERING HAS A CONTROL NOW INSTEAD OF A COMMENT. The shipped hook used
  // to reach two fail-open exits -- no `intent` on PATH, no
  // `intent/.config/config.json` -- BEFORE the whiteboard block, so a board in a
  // repo that had not been `intent init`-ed was silently unguarded. The block's
  // own introducing comment said the guards run first the whole time it was
  // false, which is why this is a test and not a third comment.
  //
  // The population is a state the design PERMITS: the whiteboard is opt-in by
  // directory presence, so a board can legitimately exist before a config does.
  let home = intent_home();
  let td = migrated_project_without_config(&home);
  let root = &td.path().join("repo");

  fs::write(
    root.join("intent/whiteboard/dc/wip.md"),
    board("2026-08-17 12:00"),
  )
  .expect("write board");
  assert!(git(root, &["add", "-A"]).status.success());

  let out = commit_through_hook(
    root,
    &td.path().join("install"),
    "wb(dc): a stamp with no Z, and no project config",
  );
  assert!(
    !out.status.success(),
    "a board with no project config was NOT guarded -- the hook reached a \
     fail-open exit before the whiteboard block, which is the ordering defect \
     this arm exists to pin:\n{}",
    combined(&out)
  );
}

#[test]
fn the_control_a_good_stamp_still_commits() {
  // WITHOUT THIS THE REFUSAL ABOVE PROVES NOTHING. A hook that refuses every
  // commit is indistinguishable from one that works, if the only case you run is
  // the one that must fail.
  let home = intent_home();
  let td = migrated_project(&home);
  let root = &td.path().join("repo");

  fs::write(
    root.join("intent/whiteboard/dc/wip.md"),
    board("2026-08-17 12:00Z"),
  )
  .expect("write board");
  assert!(git(root, &["add", "-A"]).status.success());

  let out = commit_through_hook(
    root,
    &td.path().join("install"),
    "wb(dc): a stamp that carries its Z",
  );
  assert!(
    out.status.success(),
    "a well-formed board was refused, so the guard refuses regardless of its input \
     and the refusal test above is vacuous:\n{}",
    combined(&out)
  );
}

#[test]
fn the_resolver_answers_and_the_hook_does_not_fail_open() {
  // THE SECOND ARM, AND IT IS NOT A DUPLICATE OF THE FIRST. The hook's fail-open
  // branch is correct; what moved in 0042 was its trigger. A test asserting only
  // the outcome passes the day the block comes from somewhere else.
  //
  // The extraction is lifted OUT OF THE SHIPPED HOOK at test time. Retyping the
  // `sed` would make this a second implementation of the thing under test, and
  // the two would agree until the day they mattered.
  let home = intent_home();
  let hook =
    fs::read_to_string(home.join("lib/templates/hooks/pre-commit.sh")).expect("read the hook");
  let expr = hook
    .lines()
    .find(|l| l.contains("INTENT_HOME_RESOLVED=") && l.contains("sed -n"))
    // The span after `sed -n '`, NOT simply the first quoted run: the same line
    // opens with `printf '%s\n'`, so taking quote #1 lifts printf's format
    // string and asserts on a resolution this test invented. Measured -- that is
    // what the first version of this did.
    .and_then(|l| l.split_once("sed -n '").map(|(_, rest)| rest))
    .and_then(|rest| rest.split_once('\'').map(|(expr, _)| expr))
    .expect(
      "could not lift the resolver expression out of the shipped hook -- refusing to \
       assert on a resolution this test invented",
    )
    .to_string();

  // Driven through the INSTALL, not through the build output, because that is
  // the only thing the resolver answers about -- and because a test that sets
  // INTENT_HOME here would assert on an override the binary does not read.
  let td = tempfile::tempdir().expect("tempdir");
  let install = fake_install(td.path(), &home);
  let info = output_retrying_busy(
    || {
      let mut c = Command::new(install.join("bin/intent"));
      c.arg("info").current_dir(td.path());
      c
    },
    "run intent info",
  );

  let mut sed = Command::new("sed")
    .args(["-n", &expr])
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .spawn()
    .expect("spawn sed");
  {
    use std::io::Write;
    let stdin = sed.stdin.as_mut().expect("sed stdin");
    stdin.write_all(&info.stdout).expect("feed sed");
    stdin.write_all(&info.stderr).expect("feed sed");
  }
  let resolved = sed.wait_with_output().expect("sed output");
  let value = String::from_utf8_lossy(&resolved.stdout)
    .lines()
    .next()
    .unwrap_or("")
    .to_string();

  assert!(
    !value.is_empty(),
    "the hook's own extraction resolved INTENT_HOME to NOTHING, which is the \
     fail-open trigger from issue 0042. `intent info` said:\n{}{}",
    String::from_utf8_lossy(&info.stdout),
    String::from_utf8_lossy(&info.stderr)
  );
  assert!(
    Path::new(&value).is_dir(),
    "INTENT_HOME resolved to {value:?}, which is not a directory -- the hook would \
     take the fail-open branch and both guards would be skipped"
  );
}
