//! ST0087 WP-03: `intent use [dev|opt|brew]` points the link set in a link
//! directory at one tree, or removes it for brew, and its shadow check names a
//! link that is not first on PATH and a set split across trees (devbin's
//! `docs/dev-opt-brew.md`, T5).
//!
//! **EVERY TREE IS PLANTED AND PATH IS A STUB**, so no arm reads the real
//! `~/.local/bin`, the real PATH or a real daemon. The pair is two shell
//! scripts that log how they were called, which is how the restart's arm sees
//! that it ran the NEW link by its own path.

use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};

use intentsvcs::finding::FindingClass;
use intentsvcs::install::{MANIFEST, Tree};
use intentsvcs::remedy::Remedy;
use intentsvcs::switch::{self, Fault, LinkState, RESTING, Serving, SwitchError, Target};

/// A fake binary that logs `$0 $*` to `log`, and prints a line for
/// `--version`.
fn script(at: &Path, log: &Path) {
  std::fs::create_dir_all(at.parent().expect("a parent")).expect("dir");
  std::fs::write(
    at,
    format!(
      "#!/bin/sh\nprintf '%s %s\\n' \"$0\" \"$*\" >> '{}'\n[ \"$1\" = --version ] && echo 'intent 9.9.9 (planted)'\nexit 0\n",
      log.display()
    ),
  )
  .expect("script");
  std::fs::set_permissions(at, std::fs::Permissions::from_mode(0o755)).expect("chmod");
}

struct Plant {
  _dir: tempfile::TempDir,
  base: PathBuf,
  bin: PathBuf,
  log: PathBuf,
}

impl Plant {
  fn new() -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let base = std::fs::canonicalize(dir.path()).expect("canonical");
    Plant {
      bin: base.join("home/.local/bin"),
      log: base.join("calls.log"),
      _dir: dir,
      base,
    }
  }

  /// A dev checkout: `lib/templates`, a `.git`, and its release pair.
  fn dev(&self, name: &str, with_pair: bool) -> PathBuf {
    let root = self.base.join(name);
    std::fs::create_dir_all(root.join(intentsvcs::install::MARKER)).expect("templates");
    std::fs::create_dir_all(root.join(".git")).expect("git");
    if with_pair {
      for b in switch::LINKS {
        script(&root.join(intentsvcs::switch::DEV_PAIR).join(b), &self.log);
      }
    }
    root
  }

  /// A published tree: `lib/templates`, a manifest naming `managed_by`, and
  /// `bin/{intent,intentd}`.
  fn published(&self, name: &str, managed_by: &str) -> PathBuf {
    let root = self.base.join(name);
    std::fs::create_dir_all(root.join(intentsvcs::install::MARKER)).expect("templates");
    std::fs::write(
      root.join(MANIFEST),
      format!(
        "# tool: intent\n# version: 9.9.9\n# source_commit: {}\n# managed_by: {managed_by}\n",
        "a".repeat(40)
      ),
    )
    .expect("manifest");
    for b in switch::LINKS {
      script(&root.join("bin").join(b), &self.log);
    }
    root
  }

  fn link_into(&self, name: &str, to: &Path) {
    std::fs::create_dir_all(&self.bin).expect("bin");
    symlink(to, self.bin.join(name)).expect("link");
  }
}

/// A PATH stub over `dirs`, in order: every executable `name` in them.
fn path_of(dirs: Vec<PathBuf>) -> impl Fn(&str) -> Result<Vec<PathBuf>, SwitchError> {
  move |name| {
    Ok(
      dirs
        .iter()
        .map(|d| d.join(name))
        .filter(|p| p.is_file())
        .collect(),
    )
  }
}

/// AT-03.1: bare `use` reports the set and changes nothing; no link is brew,
/// and brew carries no resting note.
#[test]
fn no_link_is_brew_and_the_report_writes_nothing() {
  let p = Plant::new();
  let report = switch::read_at(&p.bin, &path_of(vec![])).expect("read");
  assert_eq!(report.serving, Serving::Brew);
  assert!(report.faults.is_empty(), "{:?}", report.faults);
  assert_eq!(
    report.resting_note(),
    None,
    "brew is the resting state, so it says nothing"
  );
  assert!(
    report.links.iter().all(|l| l.state == LinkState::Absent),
    "{:?}",
    report.links
  );
  assert!(!p.bin.exists(), "a bare report created the link directory");
}

/// AT-03.1: `use dev` from inside the checkout points both links at its
/// release pair, and bare `use` then reports dev with the resting note.
#[test]
fn use_dev_points_both_links_at_the_checkouts_pair() {
  let p = Plant::new();
  let root = p.dev("dev", true);
  let cwd = root.join(intentsvcs::install::MARKER);
  let lookup = path_of(vec![p.bin.clone()]);
  let pointed = switch::point_at(Target::Dev, &p.bin, &cwd, None, None, &lookup).expect("use dev");
  let pair = root.join(intentsvcs::switch::DEV_PAIR);
  for name in switch::LINKS {
    assert_eq!(
      std::fs::read_link(p.bin.join(name)).expect("a link"),
      pair.join(name),
      "{name} does not point at the checkout's pair"
    );
  }
  assert_eq!(pointed.root, root);
  assert_eq!(pointed.restart_with, p.bin.join("intent"));
  let report = switch::read_at(&p.bin, &lookup).expect("read");
  assert_eq!(
    report.serving,
    Serving::Tree {
      tree: Tree::Dev,
      root: root.clone()
    }
  );
  assert_eq!(report.resting_note(), Some(RESTING));
  assert!(report.faults.is_empty(), "{:?}", report.faults);
}

/// AT-03.1: `use dev` refuses a checkout with no pair, naming the build, and a
/// place with no checkout, writing nothing either time.
#[test]
fn use_dev_refuses_a_tree_that_is_not_there() {
  let p = Plant::new();
  let root = p.dev("bare", false);
  let lookup = path_of(vec![]);
  let e = switch::point_at(Target::Dev, &p.bin, &root, None, None, &lookup).expect_err("no pair");
  assert!(matches!(e, SwitchError::NoDevPair { .. }), "{e}");
  assert!(
    e.remedy().contains("bin/devbin build all"),
    "{}",
    e.remedy()
  );
  let e =
    switch::point_at(Target::Dev, &p.bin, &p.base, None, None, &lookup).expect_err("no checkout");
  assert!(matches!(e, SwitchError::NoDevTree { .. }), "{e}");
  assert!(!p.bin.exists(), "a refusal wrote a link");
}

/// AT-03.1: `use opt` points both links at the opt tree's pair.
#[test]
fn use_opt_points_both_links_at_the_opt_tree() {
  let p = Plant::new();
  let opt = p.published("opt", "intent");
  let lookup = path_of(vec![p.bin.clone()]);
  switch::point_at(Target::Opt, &p.bin, &p.base, None, Some(&opt), &lookup).expect("use opt");
  for name in switch::LINKS {
    assert_eq!(
      std::fs::read_link(p.bin.join(name)).expect("a link"),
      opt.join("bin").join(name)
    );
  }
  let report = switch::read_at(&p.bin, &lookup).expect("read");
  assert_eq!(
    report.serving,
    Serving::Tree {
      tree: Tree::Opt,
      root: opt
    }
  );
  assert_eq!(report.resting_note(), Some(RESTING));
}

/// AT-03.5: `use opt` refuses a prefix with no manifest by name, and names
/// `intent install --prefix <dir>`; with no prefix at all it refuses too.
#[test]
fn use_opt_refuses_a_prefix_with_no_manifest() {
  let p = Plant::new();
  let bare = p.base.join("not-published");
  std::fs::create_dir_all(bare.join("bin")).expect("dir");
  let lookup = path_of(vec![]);
  let e = switch::point_at(Target::Opt, &p.bin, &p.base, None, Some(&bare), &lookup)
    .expect_err("no manifest");
  assert!(matches!(e, SwitchError::NoManifest { .. }), "{e}");
  assert!(e.to_string().contains(MANIFEST), "{e}");
  assert!(
    e.remedy().contains("intent install --prefix <dir>"),
    "{}",
    e.remedy()
  );
  let e =
    switch::point_at(Target::Opt, &p.bin, &p.base, None, None, &lookup).expect_err("no prefix");
  assert!(matches!(e, SwitchError::NoPrefix), "{e}");
  assert!(!p.bin.exists(), "a refusal wrote a link");
}

/// AT-03.6: `use opt` refuses a tree whose manifest says `managed_by: brew`,
/// naming brew, and writes nothing.
#[test]
fn use_refuses_to_point_into_a_brew_managed_tree() {
  let p = Plant::new();
  let keg = p.published("keg", "brew");
  let lookup = path_of(vec![]);
  let e = switch::point_at(Target::Opt, &p.bin, &p.base, None, Some(&keg), &lookup)
    .expect_err("brew-managed");
  assert!(matches!(e, SwitchError::BrewManaged { .. }), "{e}");
  assert!(e.to_string().contains("brew"), "{e}");
  assert!(e.remedy().contains("brew"), "{}", e.remedy());
  assert!(!p.bin.exists(), "a refusal wrote a link");
}

/// AT-03.1: `use brew` removes the whole set and nothing else, once a
/// Homebrew intent answers after it.
#[test]
fn use_brew_removes_the_set_and_nothing_else() {
  let p = Plant::new();
  let dev = p.dev("dev", true);
  let keg = p.published("keg", "brew");
  let pair = dev.join(intentsvcs::switch::DEV_PAIR);
  for name in switch::LINKS {
    p.link_into(name, &pair.join(name));
  }
  std::fs::write(p.bin.join("other"), "not intent's").expect("other");
  let lookup = path_of(vec![p.bin.clone(), keg.join("bin")]);
  let pointed =
    switch::point_at(Target::Brew, &p.bin, &p.base, None, None, &lookup).expect("use brew");
  for name in switch::LINKS {
    assert!(
      std::fs::symlink_metadata(p.bin.join(name)).is_err(),
      "{name} is still there"
    );
  }
  assert!(
    p.bin.join("other").is_file(),
    "use brew removed a file not in the set"
  );
  assert_eq!(pointed.restart_with, keg.join("bin/intent"));
  let report = switch::read_at(&p.bin, &lookup).expect("read");
  assert_eq!(report.serving, Serving::Brew);
  assert_eq!(report.resting_note(), None);
}

/// Intent's addition to T5 (vc, 2026-10-25): `use brew` refuses before
/// removing anything when no Homebrew intent would answer afterwards.
#[test]
fn use_brew_refuses_when_nothing_brew_would_answer() {
  let p = Plant::new();
  let dev = p.dev("dev", true);
  let pair = dev.join(intentsvcs::switch::DEV_PAIR);
  for name in switch::LINKS {
    p.link_into(name, &pair.join(name));
  }
  let lookup = path_of(vec![p.bin.clone()]);
  let e =
    switch::point_at(Target::Brew, &p.bin, &p.base, None, None, &lookup).expect_err("no brew");
  assert!(matches!(e, SwitchError::NoBrew { .. }), "{e}");
  assert!(e.remedy().contains("brew link intent"), "{}", e.remedy());
  for name in switch::LINKS {
    assert!(p.bin.join(name).exists(), "{name} was removed by a refusal");
  }
}

/// A file in the set that is not Intent's link is never replaced.
#[test]
fn a_link_that_is_not_ours_is_left_and_refused() {
  let p = Plant::new();
  let root = p.dev("dev", true);
  std::fs::create_dir_all(&p.bin).expect("bin");
  std::fs::write(p.bin.join("intent"), "someone else's").expect("file");
  let lookup = path_of(vec![p.bin.clone()]);
  let e = switch::point_at(Target::Dev, &p.bin, &root, None, None, &lookup).expect_err("not ours");
  assert!(matches!(e, SwitchError::NotOurs { .. }), "{e}");
  assert_eq!(
    std::fs::read_to_string(p.bin.join("intent")).expect("still there"),
    "someone else's"
  );
  assert!(std::fs::symlink_metadata(p.bin.join("intentd")).is_err());
}

/// AT-03.2: a link that is not the first hit on PATH is named with what
/// shadows it, and doctor fails on it.
#[test]
fn a_shadowed_link_is_named_with_what_shadows_it() {
  let p = Plant::new();
  let dev = p.dev("dev", true);
  let keg = p.published("keg", "brew");
  let pair = dev.join(intentsvcs::switch::DEV_PAIR);
  for name in switch::LINKS {
    p.link_into(name, &pair.join(name));
  }
  let lookup = path_of(vec![keg.join("bin"), p.bin.clone()]);
  let report = switch::read_at(&p.bin, &lookup).expect("read");
  assert!(
    report.faults.iter().any(|f| matches!(
      f,
      Fault::Shadowed { link, by: Some(by) } if link == &p.bin.join("intent") && by == &keg.join("bin/intent")
    )),
    "{:?}",
    report.faults
  );
  let findings = switch::findings(&report);
  assert!(
    findings
      .iter()
      .any(|f| f.class == FindingClass::InstallLinkShadowed && f.detail.contains("brew")),
    "the shadowing brew binary is not named as brew's: {findings:?}"
  );
  // vc's ruling (2026-10-25): reported, never counted, so the gate does not
  // refuse a commit for the PATH of the process running the check.
  assert!(!FindingClass::InstallLinkShadowed.is_actionable());
  assert!(!FindingClass::InstallLinkSplit.is_actionable());
}

/// AT-03.2: the comparison is of resolved directories, so a PATH entry that
/// is a link to the link directory does not shadow it.
#[test]
fn the_shadow_check_compares_resolved_directories() {
  let p = Plant::new();
  let dev = p.dev("dev", true);
  let pair = dev.join(intentsvcs::switch::DEV_PAIR);
  for name in switch::LINKS {
    p.link_into(name, &pair.join(name));
  }
  let alias = p.base.join("alias-bin");
  symlink(&p.bin, &alias).expect("alias");
  let report = switch::read_at(&p.bin, &path_of(vec![alias])).expect("read");
  assert!(report.faults.is_empty(), "{:?}", report.faults);
}

/// AT-03.2: a set split across trees fails, naming how each link stands.
#[test]
fn a_split_set_fails_naming_each_link() {
  let p = Plant::new();
  let dev = p.dev("dev", true);
  let opt = p.published("opt", "intent");
  p.link_into(
    "intent",
    &dev.join(intentsvcs::switch::DEV_PAIR).join("intent"),
  );
  p.link_into("intentd", &opt.join("bin/intentd"));
  let report = switch::read_at(&p.bin, &path_of(vec![p.bin.clone()])).expect("read");
  assert_eq!(report.serving, Serving::Split);
  let split = report
    .faults
    .iter()
    .find_map(|f| match f {
      Fault::Split { detail } => Some(detail.clone()),
      _ => None,
    })
    .expect("a split fault");
  assert!(
    split.contains("dev tree") && split.contains("opt tree"),
    "{split}"
  );
  assert!(
    switch::findings(&report)
      .iter()
      .any(|f| f.class == FindingClass::InstallLinkSplit)
  );
}

/// AT-03.3: the restart runs the NEW link's `intent daemon restart` by the
/// link's own path, then prints that binary's line.
#[test]
fn the_restart_runs_the_new_link_by_its_own_path() {
  let p = Plant::new();
  let root = p.dev("dev", true);
  let lookup = path_of(vec![p.bin.clone()]);
  let pointed = switch::point_at(Target::Dev, &p.bin, &root, None, None, &lookup).expect("use dev");
  let restarted = switch::restart_through(&pointed.restart_with).expect("restart");
  assert_eq!(restarted.code, Some(0));
  assert_eq!(restarted.version, "intent 9.9.9 (planted)");
  let log = std::fs::read_to_string(&p.log).expect("the pair logged its calls");
  assert!(
    log
      .lines()
      .any(|l| l == format!("{} daemon restart", p.bin.join("intent").display())),
    "the restart did not run the new link by its own path:\n{log}"
  );
}
