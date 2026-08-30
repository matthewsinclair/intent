//! **`intent init` MINTS A `project_id`, AND TWO PROJECTS DO NOT SHARE ONE.**
//!
//! D15 ratifies `(project_id, natural_id)` as the global identity and D20
//! spells both daemon events with the project half. **Minting existed for only
//! one of the two ways a project comes to exist**: `stamp_version` stamps
//! during migration, and `init` wrote a config with no such field.
//!
//! **THE GAP WAS INVISIBLE EXACTLY WHERE IT WOULD HAVE BEEN NOTICED.** Every
//! tree any of us develops in arrived by migration -- Intent's own included --
//! so every tree carried an id and looked correct. The population that lacked
//! one was every project `init` creates, which is all of them from here. A
//! defect absent from the developer's tree and present in every user's is the
//! worst distribution there is.
//!
//! **THE SECOND TEST IS THE ONE THAT DISCRIMINATES.** A mint that returned a
//! constant would satisfy the first test completely, and the failure it hides
//! is the one that matters: two projects with equal ids are indistinguishable
//! to a daemon subscriber, which is a correctness failure in the feature the
//! identity exists to serve.

use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path) -> (String, i32) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// The `project_id` a project's config carries, or `None` when it has none.
///
/// **AN EMPTY STRING IS REPORTED AS ABSENT RATHER THAN AS A VALUE**, which is
/// the same rule `Config::identity` applies in the library. A reader that
/// returned `Some("")` here would let a test assert an identity was present
/// while the project had none -- the exact shape that made three subscription
/// arms compare `""` to `""` and pass.
fn identity(root: &Path) -> Option<String> {
  let text =
    std::fs::read_to_string(root.join("intent/.config/config.json")).expect("read the config");
  let value: serde_json::Value = serde_json::from_str(&text).expect("config is JSON");
  value
    .get("project_id")
    .and_then(serde_json::Value::as_str)
    .filter(|id| !id.is_empty())
    .map(str::to_string)
}

fn initialised(name: &str) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let (err, code) = run(&["init", name], dir.path());
  assert_eq!(code, 0, "init failed: {err}");
  dir
}

#[test]
fn init_mints_an_identity_rather_than_leaving_the_seam_unbuilt() {
  let dir = initialised("fixture-project");
  let id = identity(dir.path()).expect("init wrote no usable project_id");

  // **SHAPE, NOT RFC CONFORMANCE.** Parsing it with the `uuid` crate would be
  // testing that crate and would put a dev-dependency on this package to do
  // it. The claim under test is that `init` mints a distinct identity, and the
  // canonical 36-character dashed form is enough to show it is an id rather
  // than a name, a counter, or a leftover placeholder.
  assert_eq!(
    id.len(),
    36,
    "the minted id is not a canonical UUID: {id:?}"
  );
  assert_eq!(
    id.matches('-').count(),
    4,
    "the minted id is not a canonical UUID: {id:?}"
  );
}

#[test]
fn two_projects_never_share_an_identity() {
  let alpha = initialised("alpha");
  let beta = initialised("beta");

  let a = identity(alpha.path()).expect("alpha has no project_id");
  let b = identity(beta.path()).expect("beta has no project_id");

  assert_ne!(
    a, b,
    "two freshly created projects share an identity, so nothing downstream can tell them apart"
  );
}

/// **THE COMPOSITION THAT DID NOT EXIST BEFORE `init` MINTED.**
///
/// `running_it_twice_leaves_the_tree_byte_identical` pins re-running `upgrade`
/// on a MIGRATED tree, where the first run is what mints. It cannot see this
/// case: a project that arrives at `upgrade` already carrying an identity.
/// Mint-if-absent is what makes that safe, and **a project's identity changing
/// under a routine upgrade would be silent** -- the config stays valid, every
/// command keeps working, and only a subscriber that had remembered the old id
/// would ever know.
#[test]
fn upgrading_an_initialised_project_keeps_the_identity_it_was_minted() {
  let dir = initialised("fixture-project");
  let before = identity(dir.path()).expect("init wrote no usable project_id");

  let (err, code) = run(&["upgrade"], dir.path());
  assert_eq!(code, 0, "upgrade failed: {err}");

  let after = identity(dir.path()).expect("upgrade removed the project_id");
  assert_eq!(
    before, after,
    "upgrade re-minted the identity of a project that already had one"
  );
}
