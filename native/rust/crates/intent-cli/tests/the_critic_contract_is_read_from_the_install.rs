//! Issue 0700: a critic installed in a project reads its contract from the
//! installed Intent, as it reads its rules. `intent claude rules contract`
//! prints the contract the install it runs from ships, read from its one home,
//! `intent/docs/critics.md`, and never from a copy.

use std::process::Command;

/// **THE CONTRACT IS PLANTED IN A FAKE INSTALL, WITH TEXT NO REAL ONE CARRIES**,
/// so this passes only when the verb reads the file its own install holds. A
/// copy compiled into the binary, or one read from the source tree, would print
/// Intent's real contract instead.
#[test]
fn the_contract_verb_prints_the_contract_its_install_ships() {
  let dir = tempfile::tempdir().expect("tempdir");
  let exe = crate::common::fake_install(dir.path());
  let planted = "# A critic contract no real install carries\n\nRead from the install.\n";
  let contract = dir.path().join(intentsvcs::rules::CRITIC_CONTRACT);
  std::fs::create_dir_all(contract.parent().expect("the contract sits in a directory"))
    .expect("mkdir");
  std::fs::write(&contract, planted).expect("plant the contract");

  let args = ["claude", "rules", "contract"];
  let out = exe.output(
    |p| {
      let mut c = Command::new(p);
      c.args(args)
        .current_dir(dir.path())
        .stdin(testkit::lifeline_for(&args));
      c
    },
    "run the fixture binary",
  );

  assert_eq!(
    String::from_utf8_lossy(&out.stdout),
    planted,
    "the verb must print the install's own contract, byte for byte; stderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
  assert_eq!(out.status.code(), Some(0));
}
