//! Issue 0479: **a socket path the platform cannot bind is refused before
//! anything is spawned, and the refusal names the cause and the remedy.**
//!
//! Seen 2026-09-18 under an isolated HOME 106 characters long: `intent daemon
//! start` answered *intentd was started and is not answering*, and the reason
//! -- `path must be shorter than SUN_LEN` -- was in intentd's log, one file
//! away from the person who needed it.

use std::path::{Path, PathBuf};
use std::process::Command;

use intentsvcs::daemon::{DaemonError, socket_path_fits};

/// A path of exactly `len` bytes.
fn path_of(len: usize) -> PathBuf {
  PathBuf::from(format!("/{}", "a".repeat(len - 1)))
}

/// The platform's limit, as the check itself reports it.
fn limit() -> usize {
  match socket_path_fits(&path_of(4096)) {
    Err(DaemonError::SocketPathTooLong { limit, .. }) => limit,
    other => panic!("a 4096-byte socket path was not refused as too long: {other:?}"),
  }
}

#[test]
fn the_boundary_is_the_platforms_sun_path_less_its_terminator() {
  let limit = limit();
  // macOS 104, Linux 108: the kernel's own field, never a number written here.
  assert!(
    (100..=108).contains(&limit),
    "sun_path read as {limit} bytes, which is no platform this builds for"
  );
  assert!(
    socket_path_fits(&path_of(limit - 1)).is_ok(),
    "a path one byte short of sun_path leaves room for the NUL and must bind"
  );
  assert!(
    socket_path_fits(&path_of(limit)).is_err(),
    "a path the length of sun_path has no room for the NUL and must be refused"
  );
}

/// Any `intentd.*` runtime file under `dir`.
fn runtime_files(dir: &Path) -> Vec<PathBuf> {
  let mut out = Vec::new();
  let Ok(entries) = std::fs::read_dir(dir) else {
    return out;
  };
  for e in entries.flatten() {
    let p = e.path();
    if p.is_dir() {
      out.extend(runtime_files(&p));
    } else if p
      .file_name()
      .and_then(|n| n.to_str())
      .is_some_and(|n| n.starts_with("intentd."))
    {
      out.push(p);
    }
  }
  out
}

#[test]
fn daemon_start_refuses_a_long_socket_path_before_it_spawns() {
  testkit::sweep_once();
  let home = PathBuf::from("/tmp").join(format!("intent-fixture-sunlen-{}", std::process::id()));
  let state = home.join("s".repeat(100));
  std::fs::create_dir_all(&state).expect("create the fixture");
  let socket = state.join("intent").join("run").join("intentd.sock");
  let len = socket.as_os_str().len();
  let limit = limit();
  assert!(
    len >= limit,
    "precondition: the fixture's socket path ({len}) must overrun {limit}"
  );

  let argv = ["daemon", "start"];
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(&home)
    .env("HOME", &home)
    .env("XDG_STATE_HOME", &state)
    .env_remove("XDG_RUNTIME_DIR")
    .stdin(testkit::lifeline_for(&argv))
    .output()
    .expect("the intent binary runs");
  let stderr = String::from_utf8_lossy(&out.stderr).to_string();
  let spawned = runtime_files(&home);
  let _ = std::fs::remove_dir_all(&home);

  assert!(
    !out.status.success(),
    "daemon start succeeded on an unbindable socket path: {stderr}"
  );
  for needle in [
    socket.display().to_string(),
    format!("{len} bytes"),
    format!("holds {limit}"),
    "XDG_STATE_HOME".to_string(),
  ] {
    assert!(
      stderr.contains(&needle),
      "the refusal does not name `{needle}`:\n{stderr}"
    );
  }
  assert!(
    !stderr.contains("not answering"),
    "the refusal still reports a spawned daemon that did not answer:\n{stderr}"
  );
  assert!(
    spawned.is_empty(),
    "a daemon was spawned before the refusal: {spawned:?}"
  );
}
