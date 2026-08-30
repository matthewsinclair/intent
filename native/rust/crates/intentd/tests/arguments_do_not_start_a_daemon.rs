//! An argument `intentd` does not understand must NOT start a daemon.
//!
//! **THIS COST A LIVE INCIDENT ON THE DEVELOPER'S OWN MACHINE, 2026-08-30.**
//! `main` inspected argv for `--version` and then served REGARDLESS of what else
//! was there. So `intentd --help` -- what anybody types first, and what a peer
//! typed while diagnosing something unrelated -- started a real daemon under the
//! real `$HOME`. It bound, it published an address, and for three minutes every
//! session on the machine had its store verbs refused at rc=2 by a daemon nobody
//! meant to start.
//!
//! **THE FIXTURES WERE GUARDED AND THE FRONT DOOR WAS NOT.** Every daemon test
//! in this estate takes care to supply an isolated `HOME`, precisely because a
//! daemon on the real one takes four concurrent sessions down together. The
//! thing that actually did it was a person typing `--help`. **A guard on the
//! path you expected the danger to arrive by is not a guard on the danger.**
//!
//! **EVERY CASE HERE IS DRIVEN UNDER AN ISOLATED `HOME` ANYWAY**, so that a
//! regression fails this test rather than reproducing the incident it exists to
//! prevent. A test for "does not start a daemon" that started one on the real
//! home while proving it would be the guard-that-hangs-proving-it-detects-hangs
//! shape, which this estate has already met once today.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

fn isolated_home(tag: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
  let dir = PathBuf::from("/tmp").join(format!(
    "intent-fixture-{tag}-{}-{}",
    std::process::id(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  ));
  std::fs::create_dir_all(&dir).expect("create an isolated HOME");
  dir
}

/// Run `intentd` with one argument, bounded, under a `HOME` of its own.
///
/// **BOUNDED BECAUSE THE FAILURE MODE IS `IT SERVED`**, and a served daemon
/// never exits. Without the deadline a regression here does not fail the test --
/// it hangs the suite, which is exactly the defect class this crate spent the
/// morning on.
fn run(home: &PathBuf, arg: &str) -> (Option<i32>, String, String) {
  let out = Command::new("timeout")
    .arg("5")
    .arg(env!("CARGO_BIN_EXE_intentd"))
    .arg(arg)
    .env("HOME", home)
    .output()
    .expect("intentd runs");
  (
    out.status.code(),
    String::from_utf8_lossy(&out.stdout).to_string(),
    String::from_utf8_lossy(&out.stderr).to_string(),
  )
}

/// Did anything get published under this `HOME`?
///
/// **THE DECISIVE OBSERVATION, AND IT IS NOT THE EXIT CODE.** A binary that
/// served and was then killed by the deadline exits non-zero, which is
/// indistinguishable from a clean refusal by rc alone. Publishing an address is
/// something only a serving daemon does.
fn published(home: &PathBuf) -> bool {
  home.join(".local/share/intent/intentd.addr").exists()
}

#[test]
fn an_unrecognised_argument_refuses_and_serves_nothing() {
  for arg in ["--wat", "-x", "serve", "--daemonise"] {
    let home = isolated_home("argrefuse");
    let (code, _, err) = run(&home, arg);

    assert_eq!(
      code,
      Some(1),
      "`intentd {arg}` did not refuse cleanly. Exit 124 means it SERVED and the deadline killed \
       it, which is the incident this file exists for: an argument nobody recognised started a \
       daemon. stderr: {err}"
    );
    assert!(
      !published(&home),
      "`intentd {arg}` published an address, so it bound and served despite the argument"
    );
    assert!(
      err.contains("remedy"),
      "the refusal must say what to do instead: {err}"
    );

    let _ = std::fs::remove_dir_all(&home);
  }
}

#[test]
fn help_prints_usage_and_serves_nothing() {
  let home = isolated_home("arghelp");
  let (code, out, err) = run(&home, "--help");

  assert_eq!(
    code,
    Some(0),
    "`intentd --help` must succeed. This is the exact spelling that started a daemon on the real \
     HOME: {err}"
  );
  assert!(
    !published(&home),
    "`intentd --help` published an address -- it served instead of printing help, which is the \
     incident verbatim"
  );
  assert!(
    out.contains("Usage") && out.contains("intentd"),
    "help must actually describe the binary: {out}"
  );

  let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn version_still_answers_and_serves_nothing() {
  // The arm that already worked, kept so the new refusal cannot swallow it: a
  // fix that made every argument refuse would break `--version` and nothing
  // else in this crate would notice.
  let home = isolated_home("argversion");
  let (code, out, _) = run(&home, "--version");

  assert_eq!(code, Some(0));
  assert!(
    out.contains("intentd") && out.contains(env!("CARGO_PKG_VERSION")),
    "`--version` must name the binary and its version: {out}"
  );
  assert!(!published(&home), "`--version` served");

  let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn the_no_argument_case_really_does_serve() {
  // **THE POSITIVE CONTROL, AND WITHOUT IT EVERY ASSERTION ABOVE IS FREE.** A
  // binary that refused to serve under ANY circumstances -- a broken bind, a bad
  // `HOME`, a panic on startup -- passes all three tests above, because all
  // three only require that nothing was published.
  //
  // **IT WATCHES WHILE THE DAEMON RUNS, NOT AFTER IT EXITS, AND THE FIRST
  // VERSION GOT THAT WRONG.** `Published` removes the address file on drop, so a
  // daemon that served perfectly leaves nothing behind -- and a check made after
  // the process ends sees exactly what a daemon that never started leaves.
  // **The cleanup this crate is proud of makes the naive control indistinguish-
  // able from the failure**, and it reported the correct behaviour as broken.
  let home = isolated_home("argserve");
  let mut child = Command::new(env!("CARGO_BIN_EXE_intentd"))
    .env("HOME", &home)
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .expect("intentd runs");

  let mut seen = false;
  for _ in 0..300 {
    if published(&home) {
      seen = true;
      break;
    }
    std::thread::sleep(std::time::Duration::from_millis(20));
  }

  let _ = child.kill();
  let _ = child.wait();

  assert!(
    seen,
    "`intentd` with no arguments never published an address, so it never served -- and every \
     'did not serve' assertion in this file would hold for the wrong reason"
  );

  let _ = std::fs::remove_dir_all(&home);
}
