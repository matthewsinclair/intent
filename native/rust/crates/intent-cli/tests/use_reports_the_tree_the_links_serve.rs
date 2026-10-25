//! ST0087 WP-03: bare `intent use` reports which tree the link set in
//! `~/.local/bin` serves, changing nothing, and says brew is the resting state
//! while it serves dev and says nothing while it serves brew (hv's ruling of
//! 2026-10-25, decision 80). Driven through the real binary with a planted
//! HOME and PATH; the switch's own arms are intentsvcs' `the_switch_points`.

use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::Path;
use std::process::Output;

fn bare_use(home: &Path, path: &str) -> Output {
  crate::common::intent()
    .arg("use")
    .env("HOME", home)
    .env("PATH", path)
    .stdin(testkit::lifeline_for(&["use"]))
    .output()
    .expect("run the v3 binary")
}

/// AT-03.1: **no link is brew, and the report adds no resting note.**
#[test]
fn with_no_link_bare_use_reports_brew_and_adds_nothing() {
  let home = tempfile::tempdir().expect("home");
  let out = bare_use(home.path(), "/usr/bin:/bin");
  let stdout = String::from_utf8_lossy(&out.stdout);
  assert_eq!(
    out.status.code(),
    Some(0),
    "stdout: {stdout}\nstderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(stdout.contains("ok: no link, so brew answers"), "{stdout}");
  assert!(
    !stdout.contains("note:"),
    "brew carries no resting note: {stdout}"
  );
  assert!(
    !home.path().join(".local").exists(),
    "a bare report created the link directory"
  );
}

/// AT-03.1: **while the set serves dev, the report says brew is the resting
/// state and `intent use brew` puts it back.**
#[test]
fn with_the_set_on_dev_bare_use_adds_the_resting_line() {
  let home = tempfile::tempdir().expect("home");
  let home = std::fs::canonicalize(home.path()).expect("canonical");
  let dev = home.join("checkout");
  std::fs::create_dir_all(dev.join(intentsvcs::install::MARKER)).expect("templates");
  std::fs::create_dir_all(dev.join(".git")).expect("git");
  let pair = dev.join(intentsvcs::switch::DEV_PAIR);
  std::fs::create_dir_all(&pair).expect("pair");
  let bin = home.join(".local/bin");
  std::fs::create_dir_all(&bin).expect("bin");
  for name in intentsvcs::switch::LINKS {
    std::fs::write(pair.join(name), "#!/bin/sh\nexit 0\n").expect("binary");
    std::fs::set_permissions(pair.join(name), std::fs::Permissions::from_mode(0o755))
      .expect("chmod");
    symlink(pair.join(name), bin.join(name)).expect("link");
  }
  let out = bare_use(&home, &format!("{}:/usr/bin:/bin", bin.display()));
  let stdout = String::from_utf8_lossy(&out.stdout);
  assert_eq!(
    out.status.code(),
    Some(0),
    "stdout: {stdout}\nstderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert!(
    stdout.contains(&format!(
      "ok: the link set serves the dev tree at {}",
      dev.display()
    )),
    "{stdout}"
  );
  assert!(
    stdout.contains(&format!("note: {}", intentsvcs::switch::RESTING)),
    "{stdout}"
  );
}
