//! AT-05.1 / AC-05.1 (ST0081 WP-05): **off macOS, `intent daemon start
//! --at-login` and `intent daemon stop --at-login` refuse before writing
//! anything**, naming login enrolment as macOS's and `intent daemon start`
//! without the flag as the way to run the daemon.
//!
//! **RUN OFF macOS ONLY, BECAUSE ON macOS THE FLAG IS SUPPORTED** and these
//! arms would enrol a real job. They are ignored there rather than compiled
//! out, so every host still compiles and lints them, and `cargo test` says on
//! macOS that they did not run. Linux CI and hv's Ubuntu VM run them. The
//! decision itself is driven on every host by `launchagent`'s own unit tests,
//! because `enrolment_supported` is pure in the operating system it is handed.

use std::path::Path;
use std::process::Output;

fn at_login(verb: &str, home: &Path) -> Output {
  let argv = ["daemon", verb, "--at-login"];
  crate::common::intent()
    .args(argv)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(&argv))
    .output()
    .expect("the intent binary runs")
}

fn said(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

#[test]
#[cfg_attr(
  target_os = "macos",
  ignore = "macOS supports --at-login, and running this would enrol a real job"
)]
fn start_at_login_refuses_off_macos_and_writes_no_plist() {
  let home = tempfile::tempdir().expect("tempdir");
  let out = at_login("start", home.path());
  let text = said(&out);

  assert!(
    !out.status.success(),
    "start --at-login did not refuse: {text}"
  );
  assert!(text.contains("macOS"), "the refusal names no macOS: {text}");
  assert!(
    text.contains("`intent daemon start` without the enrolment flag"),
    "the refusal names no unenrolled start: {text}"
  );
  assert!(
    !home.path().join("Library").exists(),
    "a LaunchAgents tree was written: {text}"
  );
}

#[test]
#[cfg_attr(
  target_os = "macos",
  ignore = "macOS supports --at-login, and running this would unenrol a real job"
)]
fn stop_at_login_refuses_off_macos_the_same_way() {
  let home = tempfile::tempdir().expect("tempdir");
  let out = at_login("stop", home.path());
  let text = said(&out);

  assert!(
    !out.status.success(),
    "stop --at-login did not refuse: {text}"
  );
  assert!(text.contains("macOS"), "the refusal names no macOS: {text}");
  assert!(
    text.contains("`intent daemon start` without the enrolment flag"),
    "the refusal names no unenrolled start: {text}"
  );
  assert!(
    !home.path().join("Library").exists(),
    "a LaunchAgents tree was written: {text}"
  );
}
