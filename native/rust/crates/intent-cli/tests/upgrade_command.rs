//! `intent upgrade` -- the upgrade door, driven through the real binary.
//!
//! The facade-level tests prove `Facade::upgrade` does the right thing when
//! called. They cannot prove the verb is REACHABLE, and reachability was the
//! whole of the gap: the dispatch table advertised `upgrade` in `--help` while
//! the binary answered exit 2, so every arm of the cutover measurement ran a
//! command that wrote nothing and came back byte-identical -- a green from a
//! door that never opened.
//!
//! So these tests drive the process. What they pin is narrow and each clause
//! earns its place:
//!
//!   1. the door opens and leaves the store's directory gitignored;
//!   2. running it again is byte-identical;
//!   3. a refused project is left exactly as it was found, stamp included.
//!
//! (3) is the one that would be expensive to get wrong, and it is the one a
//! facade test cannot make on its own: "nothing was written" is a claim about
//! the filesystem after a process exited, not about a return value.
//!
//! # It was AT-10.16, AC-10.3's covering row at the binary
//!
//! AC-10.3 asked a v2 estate to convert in one commit. ST1c2db93 U4 removed
//! the v2 conversion, so AC-10.3 is withdrawn and AT-10.16 is n/a; the arms
//! here drive `intent upgrade` on a v3 project through the shipped binary.

use std::collections::BTreeMap;

fn run(args: &[&str], cwd: &std::path::Path) -> (String, String, i32) {
  run_under_home(args, cwd, testkit::fixture_home())
}

/// `run` with the `HOME` handed in.
///
/// **THE DEFAULT IS A FIXTURE `HOME` AND IT WAS THE REAL ONE UNTIL ISSUE
/// `0492`.** These arms drive `claude upgrade`, which resolves the machine's
/// install pointer -- so under the developer's own `HOME` they read, and in
/// the neighbouring `bootstrap` case write, per-user state belonging to
/// whoever ran the suite. `testkit::fixture_home` exists for exactly the case
/// where a test *cannot know whether it touches per-user state*, which is this
/// one: the verb's subject is the project, and one of the facts it reports is
/// the machine's.
///
/// **THE ARM BELOW HANDS IN ITS OWN RATHER THAN TAKING THE FIXTURE**, because
/// `fixture_home` is per-PROCESS and shared by every arm in this binary: an
/// arm that writes a pointer into it is writing into the other arms' `HOME`.
fn run_under_home(
  args: &[&str],
  cwd: &std::path::Path,
  home: &std::path::Path,
) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", home)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// A project whose `config.json` declares `version` and nothing else.
///
/// **The version is a parameter because the refusal is a behaviour**: a
/// project declaring a v2 version is refused, and one declaring a v3 version
/// is the ordinary project the other arms drive.
fn project_declaring(dir: &std::path::Path, version: &str) {
  std::fs::create_dir_all(dir.join("intent/.config")).expect("mkdir");
  std::fs::write(
    dir.join("intent/.config/config.json"),
    format!(
      "{{\"intent_version\":\"{version}\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}}\n"
    ),
  )
  .expect("write config");
}

/// Put `root` under git with its estate COMMITTED, so the upgrade starts from
/// a clean tree and what it changed is what `git status` shows.
///
/// **CALLED AFTER THE ESTATE IS WRITTEN, NEVER BEFORE.** Committing first would
/// leave every file that follows untracked, which is dirty.
fn git_ready(root: &std::path::Path) {
  for args in [
    ["init", "-q"].as_slice(),
    &["config", "user.email", "t@example.com"],
    &["config", "user.name", "t"],
    &["config", "core.excludesFile", "/dev/null"],
    &["config", "commit.gpgSign", "false"],
    &["add", "-A"],
    &["commit", "-q", "-m", "fixture estate"],
  ] {
    let ok = std::process::Command::new("git")
      .args(args)
      .current_dir(root)
      .stdin(testkit::lifeline_for(args))
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git {args:?} failed at {}", root.display());
  }
}

fn declared_version(dir: &std::path::Path) -> String {
  let text =
    std::fs::read_to_string(dir.join("intent/.config/config.json")).expect("read config.json");
  let value: serde_json::Value = serde_json::from_str(&text).expect("config.json is JSON");
  value["intent_version"]
    .as_str()
    .expect("intent_version is a string")
    .to_string()
}

/// Every file the migration is responsible for, keyed by relative path.
///
/// **`intent/.cache/` is excluded on D34**: the DB is per-machine truth and is
/// never committed, so it is not among the artefacts this estate hands to
/// anyone else. What is compared is what the migration promises to reproduce --
/// canon and the generated views.
///
/// **NOT because there is nothing there worth checking, and the first version
/// of this comment said so wrongly.** It called the store "rebuilt", cited D36
/// for it, and dismissed the difference as "bytes nobody has promised anything
/// about". **D36 is the ruling that the DB is NOT disposable** -- `rm
/// intent.db` is not an operation, "not as a unit of account in canon" -- so it
/// was cited as the ground for exactly what it forbids. D34 alone carries the
/// exclusion and always did; `rebuilt` was the word doing the damage.
///
/// **The difference is one knowable thing, measured by ic rather than assumed
/// by me**: `created_at` and `updated_at` default to `strftime`, so ~705 rows
/// record when the migration ran, and **two runs of a PERFECT migrator can
/// never produce the same database**. Normalise those stamps and the content is
/// identical to the byte.
///
/// So the store is a subject with a DIFFERENT PREDICATE -- content modulo
/// run-timestamps -- and that check belongs to `intentsvcs`, which owns the
/// store and may use rusqlite. **This crate may not (D06, held by
/// `dep_graph_guard.rs`)**, and shelling out to `sqlite3` would re-introduce
/// the binary dependency Intent bundles SQLite precisely to avoid.
fn tree(root: &std::path::Path) -> BTreeMap<String, Vec<u8>> {
  fn walk(root: &std::path::Path, at: &std::path::Path, out: &mut BTreeMap<String, Vec<u8>>) {
    for entry in std::fs::read_dir(at).expect("read_dir") {
      let path = entry.expect("entry").path();
      let rel = path
        .strip_prefix(root)
        .expect("under root")
        .to_string_lossy()
        .into_owned();
      if rel.starts_with("intent/.cache") {
        continue;
      }
      // **`.git` IS NOT PART OF THE ESTATE.** `0271` makes every migrating
      // fixture a repository, and loose objects, refs and the index move for
      // reasons no assertion here is about.
      if rel == ".git" {
        continue;
      }
      if path.is_dir() {
        walk(root, &path, out);
      } else {
        out.insert(rel, std::fs::read(&path).expect("read file"));
      }
    }
  }
  let mut out = BTreeMap::new();
  walk(root, root, &mut out);
  out
}

/// Report the FIRST path whose bytes differ, rather than dumping two trees.
fn first_difference(
  a: &BTreeMap<String, Vec<u8>>,
  b: &BTreeMap<String, Vec<u8>>,
) -> Option<String> {
  for (path, bytes) in a {
    match b.get(path) {
      None => return Some(format!("{path} disappeared on the second run")),
      Some(other) if other != bytes => {
        return Some(format!(
          "{path} changed on the second run ({} bytes -> {} bytes)",
          bytes.len(),
          other.len()
        ));
      }
      Some(_) => {}
    }
  }
  b.keys()
    .find(|p| !a.contains_key(*p))
    .map(|p| format!("{p} appeared on the second run"))
}

/// **Running it twice is byte-identical, and the store's directory is
/// ignored.**
///
/// Under the fix-forward ruling an interrupted upgrade is recovered by running
/// it again, so a re-run that changed anything would make the documented
/// remedy destructive. The plain re-run is the cheap half and the one that
/// regresses silently.
#[test]
fn running_it_twice_leaves_the_tree_byte_identical() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "rerun-fixture"], root);
  assert_eq!(code, 0, "init: {err}");
  for title in ["A thread", "Another thread"] {
    let (_, err, code) = run(&["st", "new", title], root);
    assert_eq!(code, 0, "st new: {err}");
  }

  git_ready(root);
  let (_, err1, code1) = run(&["upgrade"], root);
  assert_eq!(code1, 0, "first run: {err1}");
  let after_first = tree(root);

  // **Anti-vacuity.** Two empty trees compare equal, so the comparison below
  // is only worth running if the first one left canon to compare.
  assert!(
    after_first.keys().any(|p| p.starts_with("intent/.canon/")),
    "the first run left no canon, so comparing the trees would prove nothing"
  );

  // The store's directory is ignored by the same operation, so an upgraded
  // project cannot commit a database by accident.
  let ignored = std::fs::read_to_string(root.join(".gitignore")).unwrap_or_default();
  assert!(
    ignored.lines().any(|l| l.trim() == "intent/.cache/"),
    "the runtime store is not gitignored after an upgrade: {ignored:?}"
  );
  assert!(
    !ignored.contains("*.db"),
    "the ignore rule is a PATH rule -- `*.db` would swallow a database the \
     user wants tracked: {ignored:?}"
  );

  let (_, err2, code2) = run(&["upgrade"], root);
  assert_eq!(code2, 0, "second run: {err2}");
  let after_second = tree(root);

  assert_eq!(
    after_first.len(),
    after_second.len(),
    "the second run changed how many files exist"
  );
  assert!(
    first_difference(&after_first, &after_second).is_none(),
    "a re-run is not idempotent: {}",
    first_difference(&after_first, &after_second).unwrap_or_default()
  );
}

/// **A project declaring a version before 3 is refused, never converted, and
/// is left exactly as it was found, stamp included.**
///
/// This build does not read such a config, so the upgrade refuses at discovery,
/// before it reads or writes anything. "Nothing was written"
/// is a claim about the filesystem after the process exited, so it is
/// asserted where it can be observed.
#[test]
fn a_v2_project_is_refused_and_left_as_it_was() {
  let dir = tempfile::tempdir().expect("tempdir");
  let declared = format!("{}.19.0", intentsvcs::project::OLDEST_READ_MAJOR - 1);
  let declared = declared.as_str();
  project_declaring(dir.path(), declared);
  let before = tree(dir.path());

  let (out, err, code) = run(&["upgrade"], dir.path());
  assert_eq!(
    code, 1,
    "a refusal is a verdict (1), never an unavailable tool (2): {err}{out}"
  );
  assert!(
    err.contains(declared),
    "the refusal names the version declared: {err}"
  );
  assert_eq!(
    declared_version(dir.path()),
    declared,
    "a refused upgrade stamped the version anyway"
  );
  assert!(
    !dir.path().join("intent/.cache/intent.db").exists(),
    "a refused upgrade left a store behind"
  );
  assert!(
    first_difference(&before, &tree(dir.path())).is_none(),
    "a refused upgrade changed the project: {}",
    first_difference(&before, &tree(dir.path())).unwrap_or_default()
  );
}

/// **THE READ-ONLY MODE WAS THE ONLY ONE THAT COULD NOT REPORT A REDIRECTED
/// GATE** (issue `0492`).
///
/// `claude upgrade` compares two roots: the binary's own install, and whatever
/// `~/.local/share/intent/home` names -- the tree whose guards every commit in
/// the project will actually run. They are allowed to differ, so this is a
/// note rather than a warning and the exit code does not move. But the dry run
/// returned before reaching it, on the reasoning that the pointer warnings
/// describe a gate that was just installed. That reasoning holds for the two
/// `CANNOT RUN` warnings and not for this one: **which install this machine
/// points at is true before the command runs and unchanged by it**, so
/// withholding it from the read-only mode withheld it from precisely the run
/// an operator makes when they are checking rather than changing.
///
/// Driven on 2026-09-19, when a scratch worktree held that pointer for a day
/// and a dry `claude upgrade` in every estate had nothing to say about it.
#[test]
fn a_dry_run_says_the_gate_will_come_from_a_different_install() {
  let home = tempfile::tempdir().expect("tempdir");
  // A second tree that IS an install -- `pointer_state` resolves it only if it
  // carries the marker, and a note about an unusable pointer is a different
  // message with a different subject.
  let other = home.path().join("another-install");
  std::fs::create_dir_all(other.join("lib/templates/.claude/scripts")).expect("mkdir");
  let pointer = home.path().join(".local/share/intent/home");
  std::fs::create_dir_all(pointer.parent().unwrap()).expect("mkdir");
  std::fs::write(&pointer, format!("{}\n", other.display())).expect("write pointer");

  let project = tempfile::tempdir().expect("tempdir");
  project_declaring(project.path(), "3.0.0");
  git_ready(project.path());

  let (out, err, code) = run_under_home(&["claude", "upgrade"], project.path(), home.path());
  assert_eq!(code, 0, "a dry run reports and does not fail: {err}");
  assert!(
    out.contains("dry run:"),
    "this must be the read-only mode, or the arm is testing --apply: {out}"
  );
  assert!(
    out.contains("note: the gate will run from a DIFFERENT install than this binary."),
    "the dry run must name the divergence: {out}"
  );
  assert!(
    out.contains(&other.display().to_string()),
    "the note must quote the install the pointer names, as the apply path does: {out}"
  );
  // **THE DRY RUN IS STILL DRY, AND THIS IS THE HALF THAT WOULD HURT.** The
  // note is a read of machine state; a read that republished the pointer would
  // be the defect this issue is about, committed by its own fix.
  assert_eq!(
    std::fs::read_to_string(&pointer).unwrap(),
    format!("{}\n", other.display()),
    "a dry run must not have written the install pointer"
  );
}

/// **A HOOK WHOSE CHAIN BLOCK CANON CANNOT REWRITE IS NAMED IN THE RUN'S
/// OUTPUT** (issue `0538`, vc's check): which hook, why, and that it kept its
/// block, since that block may still pass a commit in silence. A count in the
/// summary alone would say none of that.
#[test]
fn a_hook_canon_cannot_rewrite_is_named_with_its_reason() {
  let home = tempfile::tempdir().expect("tempdir");
  let project = tempfile::tempdir().expect("tempdir");
  project_declaring(project.path(), "3.0.0");
  git_ready(project.path());
  let hook = project.path().join(".git/hooks/pre-commit");
  let block = "# intent-chain-block:start (generated by intent claude upgrade)\n_intent_chain=x\n# intent-chain-block:end\n";
  let doubled = format!("#!/usr/bin/env bash\n{block}{block}");
  std::fs::write(&hook, &doubled).expect("plant a doubled hook");

  let (out, err, code) = run_under_home(
    &["claude", "upgrade", "--apply"],
    project.path(),
    home.path(),
  );
  assert_eq!(code, 0, "held is reported, not failed: {err}");
  assert!(
    out.contains("pre-commit -- it carries two chain blocks"),
    "the hook and the reason: {out}"
  );
  assert!(
    out.contains("its chain block was not rewritten"),
    "what it kept: {out}"
  );
  assert_eq!(
    std::fs::read_to_string(&hook).expect("the hook"),
    doubled,
    "a held hook is left byte for byte"
  );
}

/// `git` in `root`, asserted to succeed.
fn git(root: &std::path::Path, args: &[&str]) {
  git_out(root, args);
}

/// Issue 0551: **A STORE GIT TRACKS IS NAMED, BY THE UPGRADE AND BY DOCTOR.**
/// Intent 3.0's `init` wrote no `.gitignore`, so its projects committed the
/// store; the rule a later run writes leaves it in the index, and until now
/// nothing said so. The fixture commits the store by force, which is the index
/// state such a project has.
#[test]
fn a_store_git_tracks_is_named_until_it_is_untracked() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "tracked-store-fixture"], root);
  assert_eq!(code, 0, "init: {err}");
  let (_, err, code) = run(&["st", "new", "A thread"], root);
  assert_eq!(code, 0, "st new: {err}");
  git_ready(root);
  git(root, &["add", "-f", "intent/.cache/intent.db"]);
  git(
    root,
    &["commit", "-q", "-m", "the store, as 3.0 committed it"],
  );

  let (_, err, code) = run(&["upgrade"], root);
  assert_eq!(code, 0, "upgrade: {err}");
  assert!(
    err.contains("git rm --cached intent/.cache/intent.db"),
    "the upgrade names the command that untracks the store: {err}"
  );
  let (out, err, _) = run(&["doctor"], root);
  let said = format!("{out}{err}");
  assert!(
    said.contains("store-tracked") && said.contains("git rm --cached intent/.cache/intent.db"),
    "a default doctor shows it, with the command: {said}"
  );

  git(root, &["rm", "-q", "--cached", "intent/.cache/intent.db"]);
  git(root, &["add", "-A"]);
  git(
    root,
    &["commit", "-q", "-m", "the upgrade, the store untracked"],
  );
  let (out, err, _) = run(&["doctor"], root);
  assert!(
    !format!("{out}{err}").contains("store-tracked"),
    "an untracked store is not named: {out}{err}"
  );
}

/// `git` in `root`, asserted to succeed, answering what it printed.
fn git_out(root: &std::path::Path, args: &[&str]) -> Vec<u8> {
  let out = std::process::Command::new("git")
    .args(args)
    .current_dir(root)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run git");
  assert!(
    out.status.success(),
    "git {args:?} failed at {}: {}",
    root.display(),
    String::from_utf8_lossy(&out.stderr)
  );
  out.stdout
}

/// NUL-separated paths, as a set.
fn nul_paths(bytes: &[u8]) -> std::collections::BTreeSet<String> {
  bytes
    .split(|b| *b == 0)
    .filter(|path| !path.is_empty())
    .map(|path| String::from_utf8_lossy(path).into_owned())
    .collect()
}

/// Issue 0745. **THE UPGRADE NAMES EVERY PATH IT CHANGED IN A MANIFEST, AND
/// ITS CLOSING LINE COMMITS EXACTLY THOSE.** A 3.4.0 project whose aggregate
/// views git tracks, as 3.4.0 committed them, is upgraded through the binary.
/// What `git status` shows changed, less the views the run untracked (their
/// deletion is staged and they stay on disk, ignored), is the manifest's set.
/// The closing line's two steps, `git add` from the manifest and a plain
/// commit, leave nothing behind, the staged deletions included. A second run
/// changes nothing, writes an empty manifest, and says so.
#[test]
fn the_upgrade_names_every_path_it_changed_and_its_two_steps_commit_them() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "manifest-fixture"], root);
  assert_eq!(code, 0, "init: {err}");
  crate::common::born_ordinal(root);
  crate::common::seed_init_manifest(root);
  let st = intentsvcs::id::ThreadId::from_ordinal(1)
    .expect("an ordinal thread")
    .stored();
  crate::common::seed_ordinal_thread(root, &st, "An ordinal thread", &[(1, "A package")]);
  crate::common::seed_ordinal_issue(root, 1, "An ordinal issue");
  let (_, err, code) = run(&["st", "list"], root);
  assert_eq!(code, 0, "the 3.4.0 project renders its views: {err}");
  git_ready(root);
  // 3.4.0 committed its aggregate views: every file git ignores but the store.
  let ignored: Vec<String> = nul_paths(&git_out(
    root,
    &[
      "ls-files",
      "-z",
      "--others",
      "--ignored",
      "--exclude-standard",
    ],
  ))
  .into_iter()
  .filter(|path| !path.starts_with("intent/.cache/"))
  .collect();
  assert!(
    !ignored.is_empty(),
    "the fixture has no aggregate view on disk to track"
  );
  let mut add = vec!["add", "-f", "--"];
  add.extend(ignored.iter().map(String::as_str));
  git(root, &add);
  git(
    root,
    &["commit", "-q", "-m", "the views, as 3.4.0 committed them"],
  );

  let (_, err, code) = run(&["upgrade"], root);
  assert_eq!(code, 0, "upgrade: {err}");
  let paths_file = "intent/.cache/upgrade.paths";
  let manifest_path = root.join(paths_file);
  let manifest = nul_paths(&std::fs::read(&manifest_path).expect("the manifest is written"));
  assert!(
    manifest.contains("intent/.intentfiles"),
    "the first upgrade does not name the manifest it folded and removed: {err}"
  );
  let untracked: std::collections::BTreeSet<String> = err
    .lines()
    .filter_map(|line| line.strip_prefix("untracked: "))
    .filter_map(|rest| rest.split(" -- ").next())
    .map(str::to_string)
    .collect();
  assert!(
    !untracked.is_empty(),
    "the upgrade untracked no aggregate view: {err}"
  );
  let changed: std::collections::BTreeSet<String> = crate::common::git_changed(root)
    .difference(&untracked)
    .cloned()
    .collect();
  assert_eq!(
    changed, manifest,
    "what git shows changed, less the untracked views, is not the manifest: {err}"
  );
  assert!(
    err.contains(&format!(
      "from the project root, stage exactly what this run changed with `git add --pathspec-from-file={paths_file} --pathspec-file-nul`, then commit with a plain `git commit`"
    )),
    "the closing line does not give the two steps: {err}"
  );
  assert!(
    err.contains(&format!("paths: {paths_file} -- "))
      && !err.contains(&format!("wrote: {paths_file}")),
    "the paths file is named on a paths: line, never a wrote: line, which is a commit line: {err}"
  );

  git(
    root,
    &[
      "add",
      &format!("--pathspec-from-file={paths_file}"),
      "--pathspec-file-nul",
    ],
  );
  git(root, &["commit", "-q", "-m", "the upgrade"]);
  assert_eq!(
    crate::common::git_changed(root),
    std::collections::BTreeSet::new(),
    "the two steps left a change behind: {err}"
  );

  let (_, err, code) = run(&["upgrade"], root);
  assert_eq!(code, 0, "second run: {err}");
  assert_eq!(
    nul_paths(&std::fs::read(&manifest_path).expect("the second run writes its manifest")),
    std::collections::BTreeSet::new(),
    "a second run names a path: {err}"
  );
  assert!(
    err.contains("this run left nothing for git to commit"),
    "the second run does not say it left nothing to commit: {err}"
  );
}

/// Issue 0746 (0707's residue, hv decision 82). **A TRACKED VIEW OF A RECORD
/// THE PROJECT NO LONGER REALISES IS REMOVED BY THE UPGRADE, AND ITS COMMIT
/// PASSES DOCTOR.** A 3.4.0 project realised a thread with a package, so its
/// cover was rendered in 3.4.0's shape and committed; then the thread left the
/// manifest and its views stayed tracked. The release's cover differs from
/// that one because the renderer changed, which the gate against today's
/// render cannot tell from a hand edit, so before this the upgrade left the
/// cover and doctor refused its commit. The cover is exactly what 3.4.0's
/// format renders, so the upgrade removes it, names it, stages it through the
/// closing line's two steps, and the commit leaves doctor nothing to refuse.
/// A view carrying a person's edit is still kept: that arm is issue 0316's, in
/// `upgrade_dehydrates_undeclared_thread_views`.
#[test]
fn the_upgrade_removes_a_tracked_view_of_an_unrealised_record_and_its_commit_passes_doctor() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (_, err, code) = run(&["init", "residue-fixture"], root);
  assert_eq!(code, 0, "init: {err}");
  crate::common::born_ordinal(root);
  let st = intentsvcs::id::ThreadId::from_ordinal(1)
    .expect("an ordinal thread")
    .stored();
  let package = "A package";
  crate::common::seed_ordinal_thread(root, &st, "A thread once realised", &[(1, package)]);
  let project = intentsvcs::project::Project::open(root).expect("the fixture project");
  // Realised as 3.4.0 realised it, through the verb that hydrates.
  let (_, err, code) = run(&["st", "hydrate", &st], root);
  assert_eq!(code, 0, "the 3.4.0 project realises its thread: {err}");
  let cover = project.st_dir().join(&st).join("info.md");
  // **THE CONTROL**: 3.4.0's cover tables each package by title, and the
  // release's cover replaces the table with a pointer to `intent wp list`. So
  // this cover differs from its release render in text a person could have
  // written, the gate against today's render keeps it, and only the gate
  // against 3.4.0's render clears it.
  assert!(
    std::fs::read_to_string(&cover).is_ok_and(|text| text.contains(&format!("| {package}"))),
    "the fixture's cover does not table its package in 3.4.0's shape: {err}"
  );
  git_ready(root);
  // 0707's residue: the thread leaves the manifest and its views stay tracked.
  crate::common::seed_init_manifest(root);
  git(root, &["add", "--", "intent/.intentfiles"]);
  git(
    root,
    &[
      "commit",
      "-q",
      "-m",
      "the thread unrealised, its views still tracked",
    ],
  );

  let (_, err, code) = run(&["upgrade"], root);
  assert_eq!(code, 0, "upgrade: {err}");
  let cover_rel = project.relative(&cover);
  assert!(
    !cover.exists()
      && err.contains(&format!(
        "removed: {cover_rel} -- a view of a record this project does not realise"
      )),
    "the upgrade did not remove and name the cover 3.4.0's format renders: {err}"
  );
  let paths_file = "intent/.cache/upgrade.paths";
  assert!(
    nul_paths(&std::fs::read(root.join(paths_file)).expect("the paths file is written"))
      .contains(&cover_rel),
    "the paths file does not carry the cover's deletion: {err}"
  );

  git(
    root,
    &[
      "add",
      &format!("--pathspec-from-file={paths_file}"),
      "--pathspec-file-nul",
    ],
  );
  git(root, &["commit", "-q", "-m", "the upgrade"]);
  let (out, derr, code) = run(&["doctor"], root);
  assert_eq!(
    code, 0,
    "doctor refuses the upgrade's commit:\n{out}{derr}\nupgrade said:\n{err}"
  );
}
