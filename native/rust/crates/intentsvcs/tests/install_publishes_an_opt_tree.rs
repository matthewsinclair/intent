//! ST0087 WP-02: `intent install` publishes a clean dev tree's runtime set and
//! pair into a prefix with T3's manifest (devbin's `docs/dev-opt-brew.md`); the
//! publish round-trips through `shasum -a 256 -c`; each refusal is by name and
//! writes nothing; doctor names drift and an old manifest.
//!
//! **THE PAIR IS TWO FILES CARRYING THE MARKER THE BUILD EMBEDS**, so the split
//! pair refusal can be driven without building a second binary.

use std::path::{Path, PathBuf};

use intentsvcs::finding::FindingClass;
use intentsvcs::install::{self, Build, InstallError, MANIFEST, ManagedBy, Tree};

use crate::common::{git_at, git_commit_all_at, git_init_at};

/// The build-input scope the fixture's pair was built under, in the shape
/// `INTENT_DIRT_SCOPE` carries, and the exclude pathspecs currency hands git
/// beside it, in the shape `INTENT_UNSHIPPED` carries.
const SCOPE: &str = ":(top)native/rust :(top)surface";
const UNSHIPPED: &str = ":(top,exclude,glob)native/rust/crates/*/tests/** :(top,exclude,glob)native/rust/crates/*/benches/**";

struct Dev {
  _dir: tempfile::TempDir,
  root: PathBuf,
  pair: PathBuf,
  sha: String,
}

impl Dev {
  fn build(&self) -> Build<'_> {
    Build {
      name: "intent",
      version: "9.9.9",
      commit: &self.sha,
      kind: "dev",
      scope: SCOPE,
      unshipped: UNSHIPPED,
    }
  }

  fn publish(
    &self,
    prefix: &Path,
    force: bool,
    by: ManagedBy,
  ) -> Result<install::Publication, InstallError> {
    install::publish_at(
      &self.root,
      &self.pair,
      &self.build(),
      Some(prefix),
      force,
      by,
    )
  }
}

/// A committed dev checkout holding every support path, and a pair built at
/// its HEAD.
fn dev_tree() -> Dev {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path().join("dev");
  for (path, body) in [
    ("lib/templates/README.md", "templates\n"),
    ("lib/templates/hooks/guard.sh", "#!/bin/sh\n"),
    ("intent/plugins/claude/README.md", "claude\n"),
    ("intent/plugins/agents/plugin.json", "{}\n"),
    ("intent/plugins/agents/not-shipped.md", "outside the set\n"),
  ] {
    let file = root.join(path);
    std::fs::create_dir_all(file.parent().expect("parent")).expect("dirs");
    std::fs::write(&file, body).expect("write");
  }
  let root = root.canonicalize().expect("canonical");
  git_init_at(&root);
  git_commit_all_at(&root);
  let sha = head(&root);
  let pair = dir.path().join("pair");
  std::fs::create_dir_all(&pair).expect("pair");
  for bin in ["intent", "intentd"] {
    std::fs::write(
      pair.join(bin),
      format!("\0[intent-source-commit:{sha}]\0{bin}"),
    )
    .expect("bin");
  }
  Dev {
    _dir: dir,
    root,
    pair,
    sha,
  }
}

fn head(root: &Path) -> String {
  let out = std::process::Command::new("git")
    .args(["rev-parse", "HEAD"])
    .current_dir(root)
    .output()
    .expect("git rev-parse");
  String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn refused(result: Result<install::Publication, InstallError>, needle: &str) {
  match result {
    Ok(p) => panic!(
      "published to {} where a refusal naming `{needle}` was due",
      p.prefix.display()
    ),
    Err(e) => assert!(
      e.to_string().contains(needle),
      "the refusal did not name `{needle}`: {e}"
    ),
  }
}

/// AT-02.2: **the publish lays out the keg's layout with `.intent-install` in
/// T3's form -- tool, version, source_commit and managed_by, then one sum line
/// per runtime file sorted by path -- and `shasum -a 256 -c` accepts it.**
#[test]
fn a_publish_lays_out_the_keg_and_round_trips_through_shasum() {
  let dev = dev_tree();
  let out = tempfile::tempdir().expect("tempdir");
  let prefix = out.path().join("opt/intent");

  let published = dev
    .publish(&prefix, false, ManagedBy::Intent)
    .expect("publish");

  assert!(!published.replaced);
  let manifest = std::fs::read_to_string(prefix.join(MANIFEST)).expect("manifest");
  let header: Vec<&str> = manifest
    .lines()
    .take_while(|l| l.starts_with("# "))
    .collect();
  assert_eq!(
    header,
    [
      "# tool: intent".to_string(),
      "# version: 9.9.9".to_string(),
      format!("# source_commit: {}", dev.sha),
      "# managed_by: intent".to_string(),
    ]
  );
  let body: Vec<&str> = manifest
    .lines()
    .filter(|l| !l.starts_with('#'))
    .map(|l| l.split_once("  ").expect("a sum line").1)
    .collect();
  let mut sorted = body.clone();
  sorted.sort_unstable();
  assert_eq!(body, sorted, "the body is not sorted by path");
  assert!(body.contains(&"bin/intent") && body.contains(&"bin/intentd"));
  assert!(body.contains(&"intent/plugins/agents/plugin.json"));
  assert!(
    !body.contains(&"intent/plugins/agents/not-shipped.md"),
    "a file outside the runtime set was published"
  );

  let check = std::process::Command::new("shasum")
    .args(["-a", "256", "-c", "--quiet", "-"])
    .current_dir(&prefix)
    .stdin(std::process::Stdio::piped())
    .spawn()
    .and_then(|mut child| {
      use std::io::Write;
      let sums: String = manifest
        .lines()
        .filter(|l| !l.starts_with('#'))
        .map(|l| format!("{l}\n"))
        .collect();
      child
        .stdin
        .take()
        .expect("stdin")
        .write_all(sums.as_bytes())?;
      child.wait()
    })
    .expect("run shasum");
  assert!(
    check.success(),
    "shasum -a 256 -c refused the published tree"
  );

  let id = install::tree_at(&prefix.canonicalize().expect("canonical"), &dev.sha);
  assert_eq!(id.tree, Tree::Opt);
  assert!(
    install::tree_findings(&id).is_empty(),
    "a fresh publish raised findings: {:?}",
    install::tree_findings(&id)
  );
}

/// AT-02.4: **a tree published `--managed-by brew` answers brew and refuses an
/// `intent install` over it, `--force` included, naming brew; an opt tree is
/// replaced only with `--force`.**
#[test]
fn an_existing_opt_tree_is_replaced_only_with_force_and_brew_only_by_brew() {
  let dev = dev_tree();
  let out = tempfile::tempdir().expect("tempdir");
  let prefix = out.path().join("opt");
  dev
    .publish(&prefix, false, ManagedBy::Intent)
    .expect("first publish");

  refused(
    dev.publish(&prefix, false, ManagedBy::Intent),
    "pass --force",
  );
  let again = dev
    .publish(&prefix, true, ManagedBy::Intent)
    .expect("forced publish");
  assert!(again.replaced);
  let stale: Vec<_> = std::fs::read_dir(out.path())
    .expect("read")
    .filter_map(Result::ok)
    .map(|e| e.file_name())
    .filter(|n| n != "opt")
    .collect();
  assert!(
    stale.is_empty(),
    "the publish left staging behind: {stale:?}"
  );

  let brew = out.path().join("keg");
  dev
    .publish(&brew, false, ManagedBy::Brew)
    .expect("brew publish");
  assert_eq!(
    install::tree_at(&brew.canonicalize().expect("canonical"), &dev.sha).tree,
    Tree::Brew
  );
  refused(
    dev.publish(&brew, true, ManagedBy::Intent),
    "brew upgrade intent",
  );
}

/// AT-02.1: **`intent install` runs from a dev tree only and refuses by name,
/// before any write, a dirty runtime set, a pair behind HEAD or split, no
/// prefix, a prefix overlapping the tree, and a prefix it may not replace.**
#[test]
fn each_refusal_names_its_cause_and_writes_nothing() {
  let dev = dev_tree();
  let out = tempfile::tempdir().expect("tempdir");

  refused(
    install::publish_at(
      &dev.root,
      &dev.pair,
      &dev.build(),
      None,
      false,
      ManagedBy::Intent,
    ),
    "needs a prefix",
  );
  refused(
    dev.publish(Path::new("relative/opt"), false, ManagedBy::Intent),
    "absolute",
  );
  refused(
    dev.publish(&dev.root.join("opt"), false, ManagedBy::Intent),
    "overlaps the dev tree",
  );

  let foreign = out.path().join("foreign");
  std::fs::create_dir_all(&foreign).expect("dir");
  std::fs::write(foreign.join("keep.txt"), "mine\n").expect("write");
  refused(
    dev.publish(&foreign, true, ManagedBy::Intent),
    "did not write",
  );
  assert!(
    foreign.join("keep.txt").exists(),
    "a refused prefix lost its file"
  );

  let other = out.path().join("other");
  std::fs::create_dir_all(&other).expect("dir");
  std::fs::write(other.join(MANIFEST), "# tool: molt\n# managed_by: molt\n").expect("write");
  refused(dev.publish(&other, true, ManagedBy::Intent), "tool: molt");

  std::fs::write(
    dev.pair.join("intentd"),
    "\0[intent-source-commit:0000000000000000000000000000000000000000]\0",
  )
  .expect("write");
  refused(
    dev.publish(&out.path().join("split"), false, ManagedBy::Intent),
    "is not one build",
  );
  std::fs::write(
    dev.pair.join("intentd"),
    format!("\0[intent-source-commit:{}]\0", dev.sha),
  )
  .expect("write");

  std::fs::write(dev.root.join("lib/templates/README.md"), "edited\n").expect("write");
  refused(
    dev.publish(&out.path().join("dirty"), false, ManagedBy::Intent),
    "lib/templates/README.md, a support path",
  );
  std::fs::write(dev.root.join("lib/templates/README.md"), "templates\n").expect("write");

  // HEAD moves on a commit to one of the build's inputs, outside the support
  // set, so the inputs alone can refuse it (issue 0657). A commit that changes
  // none of them installs, in `a_head_past_the_build_is_judged_by_what_moved`.
  let input = dev.root.join("native/rust/crates/c/src/lib.rs");
  std::fs::create_dir_all(input.parent().expect("parent")).expect("dirs");
  std::fs::write(&input, "pub fn f() {}\n").expect("write");
  git_commit_all_at(&dev.root);
  refused(
    dev.publish(&out.path().join("behind"), false, ManagedBy::Intent),
    "native/rust/crates/c/src/lib.rs, one of the build's inputs",
  );

  for fresh in ["split", "dirty", "behind"] {
    assert!(
      !out.path().join(fresh).exists(),
      "a refused publish wrote {fresh}"
    );
  }

  let source = dev_tree();
  let opt = out.path().join("opt-source");
  source
    .publish(&opt, false, ManagedBy::Intent)
    .expect("publish");
  refused(
    install::publish_at(
      &opt.canonicalize().expect("canonical"),
      &source.pair,
      &source.build(),
      Some(&out.path().join("from-opt")),
      false,
      ManagedBy::Intent,
    ),
    "only from a dev tree",
  );
}

/// Issue 0657: **a HEAD past the build is judged by what moved.** Commits that
/// change none of the build's inputs, a board file and a crate's own test,
/// install and say so, the manifest still naming the built commit; a `tests/`
/// outside the cargo workspace, a support-path commit, a build that is not an
/// ancestor of HEAD and a dev tree whose git cannot be read each refuse, naming
/// why. A commit to a crate's source refuses in
/// `each_refusal_names_its_cause_and_writes_nothing`.
#[test]
fn a_head_past_the_build_is_judged_by_what_moved() {
  let out = tempfile::tempdir().expect("tempdir");

  let quiet = dev_tree();
  for (path, body) in [
    ("intent/whiteboard/cc/wip.md", "a board write\n"),
    (
      "native/rust/crates/c/tests/arm.rs",
      "// a test, built into no shipped binary\n",
    ),
  ] {
    let file = quiet.root.join(path);
    std::fs::create_dir_all(file.parent().expect("parent")).expect("dirs");
    std::fs::write(&file, body).expect("write");
  }
  git_commit_all_at(&quiet.root);
  let at = head(&quiet.root);
  let published = quiet
    .publish(&out.path().join("quiet"), false, ManagedBy::Intent)
    .expect("a HEAD moved only outside the build's inputs installs");
  assert_eq!(published.head.as_deref(), Some(at.as_str()));
  let manifest =
    std::fs::read_to_string(out.path().join("quiet").join(MANIFEST)).expect("manifest");
  assert!(
    manifest.contains(&format!("# source_commit: {}\n", quiet.sha)),
    "the manifest stopped naming the built commit:\n{manifest}"
  );
  assert_eq!(
    install::head_note(&quiet.build(), &published).as_deref(),
    Some(
      format!(
        "HEAD ({}) is past the commit the pair was built from ({}), and no commit between them changes the build's inputs (native/rust, surface, less native/rust/crates/*/tests/** and native/rust/crates/*/benches/**)",
        &at[..12],
        &quiet.sha[..12]
      )
      .as_str()
    )
  );

  // A tests/ outside the cargo workspace is an input like any other, so only
  // a crate's own tests/ and benches/ are dropped (vc's narrowing of v1).
  let tested = dev_tree();
  let file = tested.root.join("surface/tests/x.json");
  std::fs::create_dir_all(file.parent().expect("parent")).expect("dirs");
  std::fs::write(&file, "{}\n").expect("write");
  git_commit_all_at(&tested.root);
  refused(
    tested.publish(&out.path().join("tested"), false, ManagedBy::Intent),
    "surface/tests/x.json, one of the build's inputs",
  );

  let support = dev_tree();
  std::fs::write(
    support.root.join("intent/plugins/claude/README.md"),
    "edited\n",
  )
  .expect("write");
  git_commit_all_at(&support.root);
  refused(
    support.publish(&out.path().join("support"), false, ManagedBy::Intent),
    "intent/plugins/claude/README.md, a support path",
  );

  // The built commit is rewritten into a sibling, so HEAD no longer descends
  // from the commit the pair names.
  let sideways = dev_tree();
  std::fs::write(sideways.root.join("NOTES.md"), "a sibling\n").expect("write");
  git_at(&sideways.root, &["add", "-A"]);
  git_at(
    &sideways.root,
    &["commit", "-q", "--amend", "-m", "a sibling"],
  );
  refused(
    sideways.publish(&out.path().join("sideways"), false, ManagedBy::Intent),
    "not an ancestor of HEAD",
  );

  let blind = dev_tree();
  std::fs::remove_dir_all(blind.root.join(".git")).expect("rm .git");
  std::fs::write(
    blind.root.join(".git"),
    "gitdir: /nonexistent/.git/worktrees/gone\n",
  )
  .expect("write");
  refused(
    blind.publish(&out.path().join("blind"), false, ManagedBy::Intent),
    "clean runtime set",
  );

  for fresh in ["tested", "support", "sideways", "blind"] {
    assert!(
      !out.path().join(fresh).exists(),
      "a refused publish wrote {fresh}"
    );
  }
}

/// AT-02.3: **doctor names every file that drifted from an opt tree's
/// manifest and an opt manifest in an older form; a keg with no manifest is a
/// note naming `brew upgrade intent` (the next arm).**
#[test]
fn doctor_names_every_drifted_file_and_an_old_manifest() {
  let dev = dev_tree();
  let out = tempfile::tempdir().expect("tempdir");
  let prefix = out.path().join("opt");
  dev
    .publish(&prefix, false, ManagedBy::Intent)
    .expect("publish");
  let prefix = prefix.canonicalize().expect("canonical");

  std::fs::write(prefix.join("lib/templates/README.md"), "edited in place\n").expect("write");
  std::fs::remove_file(prefix.join("lib/templates/hooks/guard.sh")).expect("remove");
  std::fs::write(prefix.join("intent/plugins/claude/stray.md"), "stray\n").expect("write");
  let findings = install::tree_findings(&install::tree_at(&prefix, &dev.sha));
  let details: Vec<&str> = findings.iter().map(|f| f.detail.as_str()).collect();
  assert!(
    findings
      .iter()
      .all(|f| f.class == FindingClass::InstallManifestDrift),
    "{details:?}"
  );
  for want in [
    "lib/templates/README.md has changed",
    "lib/templates/hooks/guard.sh is missing",
    "intent/plugins/claude/stray.md is not in the manifest",
  ] {
    assert!(
      details.iter().any(|d| d.contains(want)),
      "doctor did not say `{want}`: {details:?}"
    );
  }

  std::fs::write(
    prefix.join(MANIFEST),
    "# tool: intent\n# managed_by: intent\n",
  )
  .expect("write");
  let old = install::tree_findings(&install::tree_at(&prefix, &dev.sha));
  assert_eq!(old.len(), 1, "{old:?}");
  assert_eq!(old[0].class, FindingClass::InstallManifestOld);
  assert!(old[0].detail.contains("version") && old[0].detail.contains("source_commit"));
}

#[test]
fn a_keg_with_no_manifest_is_a_note_naming_brew_upgrade() {
  let dir = tempfile::tempdir().expect("tempdir");
  let prefix = dir.path().canonicalize().expect("canonical");
  let root = prefix.join("Cellar/intent/9.9.9/libexec");
  std::fs::create_dir_all(root.join(install::MARKER)).expect("keg");
  std::fs::create_dir_all(prefix.join("opt")).expect("opt");
  std::os::unix::fs::symlink("../Cellar/intent/9.9.9", prefix.join("opt/intent")).expect("link");

  let findings = install::tree_findings(&install::tree_at(&root, "unknown"));

  assert_eq!(findings.len(), 1, "{findings:?}");
  assert_eq!(findings[0].class, FindingClass::Advisory);
  assert!(findings[0].detail.contains("brew upgrade intent"));
}
