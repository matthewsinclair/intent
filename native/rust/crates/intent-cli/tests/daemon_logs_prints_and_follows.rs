//! `AT-01.1`, `AT-01.2`, `AT-01.3` and `AT-01.4` (ST0075): **`intent daemon logs` answers
//! from the state directory `userstate` names**, the verb Intent.app's Console
//! tails.
//!
//! Every arm drives the shipped binary under an isolated `HOME`, and plants the
//! logs at the paths `userstate` itself computes, so the test cannot agree with
//! the verb about where the logs live by both being wrong the same way.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Output, Stdio};
use std::sync::mpsc;
use std::time::Duration;

use intentsvcs::userstate::{Dirs, daemon_error_log_under, daemon_log_under};

use crate::common::{children_of, short_dir};

fn bin() -> PathBuf {
  PathBuf::from(env!("CARGO_BIN_EXE_intent"))
}

fn intent(home: &Path) -> Command {
  let mut cmd = Command::new(bin());
  cmd
    .env("HOME", home)
    .env_remove("XDG_STATE_HOME")
    .env_remove("XDG_CONFIG_HOME")
    .env_remove("XDG_DATA_HOME")
    .env_remove("XDG_RUNTIME_DIR")
    .current_dir(home);
  cmd
}

/// The two log paths for `home`, from `userstate` rather than restated here.
fn logs(home: &Path) -> (PathBuf, PathBuf) {
  let dirs = Dirs::at_home(home);
  (daemon_log_under(&dirs), daemon_error_log_under(&dirs))
}

fn plant(path: &Path, lines: &[String]) {
  std::fs::create_dir_all(path.parent().expect("a log has a parent"))
    .expect("create the state dir");
  let mut body = lines.join("\n");
  body.push('\n');
  std::fs::write(path, body).expect("plant a log");
}

fn numbered(prefix: &str, n: usize) -> Vec<String> {
  (1..=n).map(|i| format!("{prefix} {i}")).collect()
}

fn stdout_lines(out: &Output) -> Vec<String> {
  String::from_utf8_lossy(&out.stdout)
    .lines()
    .map(str::to_string)
    .collect()
}

fn header(out: &Path, err: &Path) -> String {
  format!("tailing {} and {}", out.display(), err.display())
}

fn absent(path: &Path) -> String {
  format!(
    "absent: {} -- intentd has not written it yet",
    path.display()
  )
}

/// A line as intentd writes it: a UTC stamp at `minute`:`second` past ten, then
/// the text.
fn at(minute: usize, second: usize, text: String) -> String {
  format!("2026-09-16T10:{minute:02}:{second:02}.000Z {text}")
}

/// AT-01.1: the header names both logs, then the last lines of both as one list
/// in the order they were written, and the verb exits 0. `--lines` changes how
/// many lines of that one list (issue 0425: an error older than a newer
/// start line is printed above it, not after stdout's whole block).
#[test]
fn the_last_lines_of_both_logs_follow_a_header_naming_them() {
  let home = short_dir("logs-last");
  let (out_log, err_log) = logs(&home);
  let out_line = |i: usize| at(i - 1, 0, format!("out {i}"));
  let err_line = |i: usize, minute: usize| at(minute, 30, format!("err {i}"));
  plant(&out_log, &(1..=60).map(out_line).collect::<Vec<_>>());
  plant(
    &err_log,
    &[err_line(1, 19), err_line(2, 39), err_line(3, 58)],
  );

  let out = intent(&home)
    .args(["daemon", "logs"])
    .output()
    .expect("run intent daemon logs");
  assert!(
    out.status.success(),
    "rc {:?}: {}",
    out.status.code(),
    String::from_utf8_lossy(&out.stderr)
  );
  let mut want = vec![header(&out_log, &err_log)];
  want.extend((23..=40).map(out_line));
  want.push(err_line(2, 39));
  want.extend((41..=59).map(out_line));
  want.push(err_line(3, 58));
  want.push(out_line(60));
  assert_eq!(stdout_lines(&out), want);

  let out = intent(&home)
    .args(["daemon", "logs", "--lines", "2"])
    .output()
    .expect("run with --lines");
  assert!(out.status.success());
  assert_eq!(
    stdout_lines(&out),
    vec![header(&out_log, &err_log), err_line(3, 58), out_line(60)]
  );

  let out = intent(&home)
    .args(["daemon", "logs", "--lines", "many"])
    .output()
    .expect("run with a bad --lines");
  assert!(
    !out.status.success(),
    "a --lines that is not a number is refused"
  );
  assert!(String::from_utf8_lossy(&out.stderr).contains("`--lines many` is not a number of lines"));
}

/// AT-01.2: a log intentd has not written yet is named as absent, never
/// skipped -- with one log present, and with neither.
#[test]
fn a_log_not_yet_written_is_named_as_absent() {
  let home = short_dir("logs-absent");
  let (out_log, err_log) = logs(&home);

  let out = intent(&home)
    .args(["daemon", "logs"])
    .output()
    .expect("run with no logs");
  assert!(out.status.success(), "no logs yet is not a failure");
  assert_eq!(
    stdout_lines(&out),
    vec![
      header(&out_log, &err_log),
      absent(&out_log),
      absent(&err_log)
    ]
  );

  plant(&out_log, &numbered("out", 2));
  let out = intent(&home)
    .args(["daemon", "logs"])
    .output()
    .expect("run with one log");
  assert!(out.status.success());
  assert_eq!(
    stdout_lines(&out),
    vec![
      header(&out_log, &err_log),
      absent(&err_log),
      "out 1".into(),
      "out 2".into()
    ]
  );
}

/// Spawn `--follow` holding its stdin, as Intent.app's Console does, with
/// its stdout read line by line on a thread.
fn follow(home: &Path) -> (Child, ChildStdin, mpsc::Receiver<String>) {
  let mut child = intent(home)
    .args(["daemon", "logs", "--follow"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
    .spawn()
    .expect("spawn intent daemon logs --follow");
  let stdin = child.stdin.take().expect("piped stdin");
  let stdout = child.stdout.take().expect("piped stdout");
  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move || {
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
      if tx.send(line).is_err() {
        break;
      }
    }
  });
  (child, stdin, rx)
}

fn next(rx: &mpsc::Receiver<String>) -> String {
  rx.recv_timeout(Duration::from_secs(20))
    .expect("a line from --follow")
}

fn alive(pid: &str) -> bool {
  Command::new("kill")
    .args(["-0", pid])
    .stderr(Stdio::null())
    .status()
    .expect("kill -0 runs")
    .success()
}

/// AT-01.3: `--follow` prints a line the daemon appends after the verb started,
/// including to a log that did not exist when it started, and ends when its
/// stdin closes.
#[test]
fn follow_prints_what_is_written_after_it_started() {
  let home = short_dir("logs-follow");
  let (out_log, err_log) = logs(&home);
  plant(&out_log, &numbered("out", 1));

  let (mut child, stdin, rx) = follow(&home);
  assert_eq!(next(&rx), header(&out_log, &err_log));
  assert_eq!(next(&rx), absent(&err_log));
  assert_eq!(next(&rx), "out 1");

  let mut log = std::fs::OpenOptions::new()
    .append(true)
    .open(&out_log)
    .expect("open the log to append");
  writeln!(log, "out 2").expect("append to the log");
  drop(log);
  assert_eq!(next(&rx), "out 2");

  plant(&err_log, &["err 1".to_string()]);
  assert_eq!(next(&rx), "err 1");

  drop(stdin);
  let status = child.wait().expect("reap the follow");
  assert!(
    status.success(),
    "a closed stdin ends --follow cleanly, got {status}"
  );
}

/// AT-01.4: issue 0281's ruling, driven through the shipped verb. However
/// `--follow` ends -- SIGTERM, SIGINT, SIGKILL, or its stdin closing -- the
/// tail it started does not survive it.
///
/// **THE TAIL'S PID IS FOUND BEFORE THE VERB IS ENDED, AND THAT IS THE
/// CONTROL.** "No tail is running" is also what a probe that never found one
/// would report, so each way first proves a live tail exists under the verb's
/// shell, then ends the verb, then waits for that same pid to go.
#[test]
fn no_tail_survives_follow_however_it_ends() {
  for way in ["TERM", "INT", "KILL", "stdin"] {
    let home = short_dir("logs-orphan");
    let (out_log, _) = logs(&home);
    plant(&out_log, &numbered("out", 1));

    let (mut child, stdin, rx) = follow(&home);
    next(&rx);
    let pid = child.id().to_string();
    let mut tail = None;
    for _ in 0..200 {
      tail = children_of(child.id())
        .iter()
        .flat_map(|sh| children_of(sh.parse().expect("a pid")))
        .next();
      if tail.is_some() {
        break;
      }
      std::thread::sleep(Duration::from_millis(50));
    }
    let tail = tail.unwrap_or_else(|| {
      panic!("{way}: no tail found under the verb's shell, so the arm would prove nothing")
    });
    assert!(
      alive(&tail),
      "{way}: the tail is alive before the verb is ended"
    );

    if way == "stdin" {
      drop(stdin);
    } else {
      Command::new("kill")
        .args(["-s", way, &pid])
        .status()
        .expect("signal the verb");
      std::mem::forget(stdin);
    }
    child.wait().expect("reap the verb");

    let mut gone = false;
    for _ in 0..200 {
      if !alive(&tail) {
        gone = true;
        break;
      }
      std::thread::sleep(Duration::from_millis(50));
    }
    assert!(
      gone,
      "{way}: tail {tail} survived the verb it was started by"
    );
  }
}
