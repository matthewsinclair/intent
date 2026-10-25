//! ST0087 WP-01: the one tree function, read in the contract's order (devbin's
//! `docs/dev-opt-brew.md`, T4), on each of its four answers from scratch roots;
//! the version line's shape, with its sha the commit the binary was built from
//! and never HEAD; and the dirty marker over the support set.
//!
//! **EVERY ROOT IS BUILT HERE**, so no arm depends on the tree the suite runs
//! in: a dev answer measured on the checkout itself would pass for the checkout
//! and say nothing about the function.

use std::path::{Path, PathBuf};

use intentsvcs::finding::FindingClass;
use intentsvcs::install::{self, Build, MANIFEST, Tree};

use crate::common::{git_commit_all_at, git_init_at};

const BUILD_KIND: &str = "dev";

fn line(commit: &str, id: &install::TreeId) -> String {
  install::version_line(
    &Build {
      name: "intent",
      version: "9.9.9",
      commit,
      kind: BUILD_KIND,
    },
    id,
  )
}

/// A root holding the install marker and every support path, canonical.
fn install_root(at: &Path) -> PathBuf {
  std::fs::create_dir_all(at.join(install::MARKER)).expect("templates");
  std::fs::write(at.join("lib/templates/README.md"), "templates\n").expect("write");
  std::fs::create_dir_all(at.join("intent/plugins/claude")).expect("claude");
  std::fs::write(at.join("intent/plugins/claude/README.md"), "claude\n").expect("write");
  std::fs::create_dir_all(at.join("intent/plugins/agents")).expect("agents");
  std::fs::write(at.join("intent/plugins/agents/plugin.json"), "{}\n").expect("write");
  at.canonicalize().expect("canonical")
}

/// A committed dev checkout and its HEAD.
fn dev_checkout(at: &Path) -> (PathBuf, String) {
  let root = install_root(at);
  git_init_at(&root);
  git_commit_all_at(&root);
  let head = head(&root);
  (root, head)
}

fn head(root: &Path) -> String {
  let out = std::process::Command::new("git")
    .args(["rev-parse", "HEAD"])
    .current_dir(root)
    .output()
    .expect("git rev-parse");
  assert!(out.status.success(), "git rev-parse HEAD failed");
  String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn a_git_checkout_with_no_manifest_is_dev_and_its_line_names_the_built_commit() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (root, sha) = dev_checkout(dir.path());

  let id = install::tree_at(&root, &sha);

  assert_eq!(id.tree, Tree::Dev);
  assert!(
    !id.dirty,
    "a clean checkout at the built commit read as dirty"
  );
  assert_eq!(
    line(&sha, &id),
    format!(
      "intent 9.9.9 (dev, {}) at {} dev",
      &sha[..12],
      root.display()
    )
  );
  assert!(
    install::tree_findings(&id).is_empty(),
    "a dev tree at its built commit raised a finding: {:?}",
    install::tree_findings(&id)
  );
}

#[test]
fn a_pair_built_behind_head_names_its_own_commit_and_doctor_says_head_moved() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (root, built) = dev_checkout(dir.path());
  std::fs::write(root.join("NOTES.md"), "a later commit\n").expect("write");
  git_commit_all_at(&root);
  let now = head(&root);
  assert_ne!(built, now, "the fixture made no second commit");

  let id = install::tree_at(&root, &built);

  assert_eq!(id.sha.as_deref(), Some(built.as_str()));
  assert!(
    line(&built, &id).contains(&format!("(dev, {})", &built[..12])),
    "the line did not name the built commit: {}",
    line(&built, &id)
  );
  assert!(
    !line(&built, &id).contains(&now[..12]),
    "the line named HEAD, which the contract forbids: {}",
    line(&built, &id)
  );
  let findings = install::tree_findings(&id);
  assert_eq!(findings.len(), 1, "{findings:?}");
  assert_eq!(findings[0].class, FindingClass::Advisory);
  assert!(
    findings[0].detail.contains(&now[..12]) && findings[0].detail.contains(&built[..12]),
    "doctor did not name both HEAD and the build: {}",
    findings[0].detail
  );
}

#[test]
fn a_changed_or_untracked_support_path_or_a_dirty_build_marks_the_line_dirty() {
  let dir = tempfile::tempdir().expect("tempdir");
  let (root, sha) = dev_checkout(dir.path());

  std::fs::write(root.join("lib/templates/README.md"), "edited\n").expect("write");
  let edited = install::tree_at(&root, &sha);
  assert!(edited.dirty, "an edited support file did not read as dirty");
  assert!(line(&sha, &edited).contains(&format!("(dev, {}+dirty)", &sha[..12])));

  git_commit_all_at(&root);
  let committed = head(&root);
  std::fs::write(root.join("intent/plugins/claude/new.md"), "untracked\n").expect("write");
  assert!(
    install::tree_at(&root, &committed).dirty,
    "an untracked file under a support path did not read as dirty"
  );

  std::fs::remove_file(root.join("intent/plugins/claude/new.md")).expect("remove");
  assert!(
    !install::tree_at(&root, &committed).dirty,
    "the control: a clean support set at the built commit must not be dirty"
  );
  assert!(
    install::tree_at(&root, &format!("dirty-{committed}")).dirty,
    "a binary built from a dirty tree did not read as dirty"
  );

  std::fs::write(root.join("NOTES.md"), "outside the support set\n").expect("write");
  assert!(
    !install::tree_at(&root, &committed).dirty,
    "a change outside the support set read as dirty"
  );
}

#[test]
fn a_dev_tree_whose_git_cannot_be_read_names_no_commit_and_doctor_fails() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = install_root(dir.path());
  std::fs::write(
    root.join(".git"),
    "gitdir: /nonexistent/.git/worktrees/gone\n",
  )
  .expect("write");

  let id = install::tree_at(&root, "0123456789abcdef0123456789abcdef01234567");

  assert_eq!(id.tree, Tree::Dev);
  assert_eq!(id.sha, None);
  assert_eq!(
    line("0123456789abcdef0123456789abcdef01234567", &id),
    format!("intent 9.9.9 (dev, unknown) at {} dev", root.display())
  );
  let findings = install::tree_findings(&id);
  assert_eq!(findings.len(), 1, "{findings:?}");
  assert_eq!(findings[0].class, FindingClass::InstallTreeUnknown);
}

#[test]
fn a_manifest_naming_intent_is_opt_and_its_sha_is_the_manifests() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = install_root(dir.path());
  let manifest_sha = "abcdefabcdef0123456789abcdef0123456789ab";
  std::fs::write(
    root.join(MANIFEST),
    format!(
      "# tool: intent\n# version: 9.9.9\n# source_commit: {manifest_sha}\n# managed_by: intent\nabc  bin/intent\n"
    ),
  )
  .expect("write");

  let id = install::tree_at(&root, "0123456789abcdef0123456789abcdef01234567");

  assert_eq!(id.tree, Tree::Opt);
  assert_eq!(
    line("0123456789abcdef0123456789abcdef01234567", &id),
    format!("intent 9.9.9 (opt, abcdefabcdef) at {} dev", root.display())
  );
}

/// AT-01.1: **one function names the running tree in the contract's order --
/// brew by its manifest, opt, dev, a manifest-less keg by its opt link, unknown
/// otherwise -- and doctor fails on an unknown tree or an unreadable dev git.**
/// This arm and its siblings drive each answer from a scratch root.
#[test]
fn a_manifest_managed_by_brew_is_brew_before_its_tool_is_read() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = install_root(dir.path());
  std::fs::write(
    root.join(MANIFEST),
    "# tool: intent\n# version: 9.9.9\n# source_commit: abcdefabcdef0123456789abcdef0123456789ab\n# managed_by: brew\n",
  )
  .expect("write");

  assert_eq!(install::tree_at(&root, "unknown").tree, Tree::Brew);
}

#[test]
fn a_keg_with_no_manifest_is_brew_and_is_named_by_its_opt_link() {
  let dir = tempfile::tempdir().expect("tempdir");
  let prefix = dir.path().canonicalize().expect("canonical");
  let keg = prefix.join("Cellar/intent/9.9.9");
  let root = install_root(&keg.join("libexec"));
  std::fs::create_dir_all(prefix.join("opt")).expect("opt");
  std::os::unix::fs::symlink("../Cellar/intent/9.9.9", prefix.join("opt/intent")).expect("link");

  let sha = "0123456789abcdef0123456789abcdef01234567";
  let id = install::tree_at(&root, sha);

  assert_eq!(id.tree, Tree::Brew);
  assert_eq!(
    line(sha, &id),
    format!(
      "intent 9.9.9 (brew, 0123456789ab) at {} dev",
      prefix.join("opt/intent/libexec").display()
    )
  );
}

#[test]
fn a_root_with_no_manifest_and_no_git_is_unknown_and_doctor_fails() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = install_root(dir.path());

  let id = install::tree_at(&root, "0123456789abcdef0123456789abcdef01234567");

  assert_eq!(id.tree, Tree::Unknown);
  assert!(
    line("0123456789abcdef0123456789abcdef01234567", &id)
      .starts_with("intent 9.9.9 (unknown, 0123456789ab) at /")
  );
  let findings = install::tree_findings(&id);
  assert_eq!(findings.len(), 1, "{findings:?}");
  assert_eq!(findings[0].class, FindingClass::InstallTreeUnknown);
}

#[test]
fn a_manifest_naming_another_tool_is_unknown() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = install_root(dir.path());
  std::fs::write(root.join(MANIFEST), "# tool: molt\n# managed_by: molt\n").expect("write");

  let id = install::tree_at(&root, "unknown");

  assert_eq!(id.tree, Tree::Unknown);
  assert!(
    id.why.as_deref().is_some_and(|why| why.contains("`molt`")),
    "the reason did not name the manifest's tool: {:?}",
    id.why
  );
}
