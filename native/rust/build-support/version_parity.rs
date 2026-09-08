// The build-time version-parity assertion, shared by every binary crate's
// `build.rs`.
//
// ONE FACT, TWO FILES, AND NOTHING HELD THEM EQUAL. The project version lives
// at `VERSION` in the repo root, where the v2 shell CLI reads it
// (`bin/intent_helpers`), and again at `[workspace.package] version` in
// `native/rust/Cargo.toml`, from which every crate inherits it with
// `version.workspace = true` and both binaries compile their version string via
// `CARGO_PKG_VERSION`. **The mirror is what ships and the source is what
// nothing in this build reads** -- so a bump applied to one and not the other
// produced a tag saying one number and a binary saying another, with no
// symptom until someone read them side by side.
//
// `int version check` now checks the same pair (`project.version_sidecars` in
// `bin/.devbin/config.yaml`), and this file exists because that check is a
// thing a person RUNS. This one cannot be skipped: a drifted tree does not
// compile.
//
// A SEPARATE FILE FROM `source_commit.rs`, DELIBERATELY, THOUGH THE TWO ARE
// INCLUDED BY THE SAME WRAPPERS. That file's subject is PROVENANCE -- which
// commit an artefact came from. This one's is VERSION PARITY. They are one
// wrapper's two calls and not one concern, and folding this into that file
// would leave its name covering a check it does not describe. The estate has
// already ruled this shape once, on the two whiteboard guards: one concern, one
// home, because a guard's name must not come to cover checks it does not name.
//
// THERE IS DELIBERATELY NO `cargo:rerun-if-changed` HERE, AND THE REASON IS THE
// ONE `source_commit.rs` RECORDS AT LENGTH: emitting NO line is not "no
// trigger", it is cargo's default of re-running the script when any file in the
// PACKAGE changes. **Emitting ANY `rerun-if-changed` REPLACES that default**,
// so naming `VERSION` here would swap a trigger that follows the code for one
// that does not follow it at all -- and it would break the source-commit embed
// that shares these wrappers, which is a defect in someone else's contract
// caused by a line in mine.
//
// WHAT THAT COSTS, STATED RATHER THAN DISCOVERED. `VERSION` sits outside the
// package, so an edit to `VERSION` ALONE does not re-trigger this script: the
// build reuses the cached result and the drift is not reported until the next
// change to anything in the package. **The path that matters is covered
// regardless** -- `Cargo.toml` IS in the package, so any bump of the mirror
// fires this immediately, and a release bump writes both. The uncovered case is
// a hand-edit of `VERSION` alone, caught at the next source change rather than
// at once. That is a delay, not a hole, and `int version check` reports it in
// the meantime.
//
// A MISSING `VERSION` IS FATAL BY CHOICE, not by oversight. The parity claim is
// unverifiable without it, and an unverifiable claim that passes silently is
// exactly the defect one layer up: `int version check` answered green over an
// empty sidecar list for as long as it was unconfigured. If a build that has no
// repo root ever becomes legitimate here -- `cargo package`, a vendored crate --
// that is a decision to take deliberately at this line, not to discover as a
// check that quietly stopped checking.
//
// `intentsvcs` does not include this, and that is not an omission: it inherits
// the same `version.workspace = true` value, so asserting there would assert the
// same fact a third time, and the artefacts that carry a version to a user are
// the two binaries.
fn assert_version_parity() {
  // The repo root, from the package this script is being compiled for:
  // native/rust/crates/<crate>/ -> crates -> rust -> native -> root.
  let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
    .expect("cargo always sets CARGO_MANIFEST_DIR for a build script");
  let version_file = std::path::Path::new(&manifest_dir).join("../../../../VERSION");

  let declared = match std::fs::read_to_string(&version_file) {
    Ok(text) => text.trim().to_string(),
    Err(err) => panic!(
      "cannot read the project version file at {} ({err}).\n\
       This build asserts that VERSION and [workspace.package] version agree, and it cannot do that \
       without VERSION. Nothing has been built. If a build without the repo root is legitimate here, \
       change native/rust/build-support/version_parity.rs deliberately -- do not let the check \
       silently stop checking.",
      version_file.display()
    ),
  };

  // The manifest's own number, which is what the compiled binary will report.
  let compiled = std::env::var("CARGO_PKG_VERSION").expect("cargo always sets CARGO_PKG_VERSION");

  assert!(
    declared == compiled,
    "version drift: VERSION says {declared}, native/rust/Cargo.toml says {compiled}.\n\
     The Cargo.toml is what the binary reports, so a build from here would ship {compiled} under a \
     tree that calls itself {declared}. Heal with `bin/devbin version sync` (writes the sidecars from \
     VERSION), or `bin/devbin version set <v>` to move both."
  );
}
