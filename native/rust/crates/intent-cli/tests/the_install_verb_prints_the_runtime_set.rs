//! ST0087 WP-01: `intent install --print-runtime-set` prints the support paths
//! of the runtime set from the one list in the binary, which the packaging
//! reads from the pair it names; bare `install` refuses as not built until the
//! publish modes land (WP-02).

use std::process::Output;

fn run(args: &[&str]) -> Output {
  crate::common::intent()
    .args(args)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary")
}

/// AT-01.2: **`intent install --print-runtime-set` prints the runtime set's
/// support paths from the one list in the binary, and the install holds each.**
#[test]
fn the_runtime_set_is_printed_one_path_per_line_from_the_one_list() {
  let out = run(&["install", "--print-runtime-set"]);

  assert_eq!(
    out.status.code(),
    Some(0),
    "stderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  let printed: Vec<String> = String::from_utf8_lossy(&out.stdout)
    .lines()
    .map(str::to_string)
    .collect();
  assert_eq!(printed, intentsvcs::install::RUNTIME_SET);
  let home = intentsvcs::install::resolve(crate::common::intent_path()).expect("the install");
  for path in &printed {
    assert!(
      home.join(path).exists(),
      "the runtime set names {path}, which this install does not hold"
    );
  }
}

#[test]
fn bare_install_refuses_as_not_built_at_two() {
  let out = run(&["install"]);

  assert_eq!(out.status.code(), Some(2));
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(
    stderr.contains("is a known command that is not implemented yet")
      && stderr.contains("--print-runtime-set"),
    "the refusal did not say what is built: {stderr}"
  );
  assert!(out.stdout.is_empty(), "a refusal printed paths");
}
