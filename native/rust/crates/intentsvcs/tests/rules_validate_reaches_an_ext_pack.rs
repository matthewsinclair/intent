//! **THE EXT HALF OF `intent claude rules validate`, PROVEN AT THE LEVEL THAT
//! IS REACHABLE TODAY.**
//!
//! The CLI cannot validate extension packs: `render.rs`'s `library()` passes
//! `None`, because `userstate::ext_base()` returns `None` and that is a HELD
//! RULING with a substantive reason -- *"an operator who set
//! `INTENT_EXT_DISABLE=1` would have their extensions silently switched back
//! on"* -- not merely an environment-variable allowlist question.
//!
//! **BUT `Library::new`'s `ext` IS A PARAMETER, NOT AN ENVIRONMENT READ**, so
//! everything downstream of that decision can be driven now. What is blocked is
//! WHERE the path comes from; what is not blocked is what happens once it has
//! one. Proving the second half here means that when the first half is ruled,
//! the wiring is one line over behaviour that already has coverage -- rather
//! than a ruling landing on an unexercised path.
//!
//! **THIS IS NOT A SUBSTITUTE FOR THE TWO BATS ARMS AND DOES NOT CLAIM TO BE.**
//! `rules validate passes the ext valid-ext fixture rule` and `rules validate
//! detects duplicate ids across files` drive the BINARY, and the binary still
//! cannot reach a pack. They stay red until the ruling lands.

use intentsvcs::rules::Library;
use std::path::{Path, PathBuf};

fn install_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

/// A pack at `<base>/<name>/rules/<language>/<category>/<slug>/RULE.md`, which
/// is the layout `ext_packs` walks and `files` descends.
fn plant(base: &Path, pack: &str, slug: &str, body: &str) -> PathBuf {
  let dir = base.join(pack).join("rules/agnostic/fixt").join(slug);
  std::fs::create_dir_all(&dir).expect("create the pack");
  let path = dir.join("RULE.md");
  std::fs::write(&path, body).expect("write the rule");
  path
}

const GOOD: &str = "---\nid: IN-AG-EXTOK-001\ntitle: An ext rule\nlanguage: agnostic\ncategory: fixt\nseverity: recommendation\nsummary: Fine.\nprinciples: []\napplies_when: []\n---\n\n# An ext rule\n";

const BAD_ID: &str = "---\nid: NOT-AN-INTENT-ID\ntitle: An ext rule\nlanguage: agnostic\ncategory: fixt\nseverity: recommendation\nsummary: Fine.\nprinciples: []\napplies_when: []\n---\n\n# An ext rule\n";

#[test]
fn an_ext_packs_rules_are_validated_and_a_defective_one_is_named() {
  let dir = tempfile::tempdir().expect("tempdir");
  let bad = plant(dir.path(), "somepack", "broken", BAD_ID);

  let lib = Library::new(&install_root(), Some(dir.path().to_path_buf()));
  let (findings, _) = lib.validate(None).expect("validate");

  assert!(
    findings.iter().any(|f| f.path == bad),
    "the ext pack's defective rule was not reported: {:?}",
    findings.iter().map(|f| &f.path).collect::<Vec<_>>()
  );
}

#[test]
fn the_same_library_without_an_ext_base_cannot_see_it() {
  // **THE CONTROL THAT MAKES THE ARM ABOVE MEAN ANYTHING.** Without it, a
  // finding could have come from the canon corpus and the ext walk could be
  // doing nothing at all -- which is exactly the state the CLI is in.
  let dir = tempfile::tempdir().expect("tempdir");
  let bad = plant(dir.path(), "somepack", "broken", BAD_ID);

  let lib = Library::new(&install_root(), None);
  let (findings, _) = lib.validate(None).expect("validate");

  assert!(
    !findings.iter().any(|f| f.path == bad),
    "an ext-less library reported an ext rule, so the ext base is not what reaches them"
  );
}

#[test]
fn a_clean_ext_pack_adds_no_findings_and_is_genuinely_non_empty() {
  let dir = tempfile::tempdir().expect("tempdir");
  let good = plant(dir.path(), "somepack", "ok", GOOD);

  let lib = Library::new(&install_root(), Some(dir.path().to_path_buf()));
  let (findings, examined) = lib.validate(None).expect("validate");

  // **THE FIXTURE IS ASSERTED BEFORE THE ABSENCE IS BELIEVED.** A pack the walk
  // never descended would also produce no findings, and would look identical.
  let (bare, bare_examined) = Library::new(&install_root(), None)
    .validate(None)
    .expect("validate");
  assert!(
    examined > bare_examined,
    "the ext pack contributed no rules to the corpus, so a clean result proves nothing"
  );

  assert!(
    !findings.iter().any(|f| f.path == good),
    "a well-formed ext rule was reported: {:?}",
    findings
      .iter()
      .filter(|f| f.path == good)
      .collect::<Vec<_>>()
  );
  assert_eq!(
    findings.len(),
    bare.len(),
    "the clean pack changed the finding count, so something about it is being judged differently"
  );
}

#[test]
fn a_duplicate_id_across_canon_and_ext_is_caught() {
  // **THE PROPERTY THE BLOCKED BATS ARM IS ABOUT**, driven where it can be.
  // An ext pack that shadows a canon id is the case a per-file check cannot
  // see and the whole-corpus check can -- and it is the one that matters,
  // because two rules answering to one id makes `rules show` ambiguous.
  let dir = tempfile::tempdir().expect("tempdir");
  let a = plant(dir.path(), "packone", "dupe", GOOD);
  let b = plant(dir.path(), "packtwo", "dupe", GOOD);

  let lib = Library::new(&install_root(), Some(dir.path().to_path_buf()));
  let (findings, _) = lib.validate(None).expect("validate");

  for path in [&a, &b] {
    assert!(
      findings
        .iter()
        .any(|f| &f.path == path && f.message.contains("is declared by")),
      "both sides of a duplicate id must be told, and {} was not: {:?}",
      path.display(),
      findings
        .iter()
        .map(|f| f.message.clone())
        .collect::<Vec<_>>()
    );
  }
}
