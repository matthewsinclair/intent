//! AC-07.3 -- `intent claude skills` install / sync / uninstall.
//!
//! **THIS FILE IS THE AT-07.3 SUBJECT AND IT IS NOT AT THE PATH THE ROW CITES.**
//! The row names `intent-cli/tests/skills_sync.rs`; the behaviour lives in
//! `intentsvcs::payload`, and the CLI arm is held behind the AC-11.3 `$HOME`
//! ruling so there is nothing in `intent-cli` to drive. Writing a CLI test
//! against an unwired command would be **building the citation rather than the
//! check** -- dc's refusal on AT-07.4, adjudicated and upheld by vc, in this
//! same WP. The row gets repointed; the test goes where its subject is.
//!
//! **EVERY ARM HERE IS ABOUT A CASE v2 GETS WRONG, EXCEPT THE CONTROLS.** The
//! controls are load-bearing: a sync that refused everything, or propagated
//! everything unconditionally, would pass the interesting arms and be useless.

use std::fs;
use std::path::{Path, PathBuf};

use intentsvcs::payload::{Baseline, Kind, MANIFEST_RELATIVE, Outcome, Payload, Scope};

/// A disposable estate: an install tree, a Claude target, a manifest path.
struct Fixture {
  _tmp: tempfile::TempDir,
  install: PathBuf,
  target: PathBuf,
  manifest: PathBuf,
}

impl Fixture {
  fn new() -> Self {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().to_path_buf();
    let install = root.join("install");
    fs::create_dir_all(install.join("intent/plugins/claude/skills")).unwrap();
    // `install::MARKER` -- what makes a tree an install.
    fs::create_dir_all(install.join("lib/templates")).unwrap();
    Self {
      install,
      target: root.join("home/.claude/skills"),
      manifest: root.join("home/.intent").join(MANIFEST_RELATIVE),
      _tmp: tmp,
    }
  }

  fn skills(&self) -> Payload {
    Payload::new(
      Kind::Skills,
      &self.install,
      None,
      self.target.clone(),
      self.manifest.clone(),
    )
  }

  fn with_ext(&self, ext: &Path) -> Payload {
    Payload::new(
      Kind::Skills,
      &self.install,
      Some(ext.to_path_buf()),
      self.target.clone(),
      self.manifest.clone(),
    )
  }

  fn canon(&self) -> PathBuf {
    self.install.join("intent/plugins/claude/skills")
  }

  /// The checksum the manifest RECORDS for a skill.
  ///
  /// **Read through the library rather than off the JSON**, so a test asserting
  /// what was recorded cannot pass by agreeing with a second parser.
  fn manifest_checksum(&self, name: &str) -> String {
    self
      .skills()
      .manifest()
      .expect("the manifest reads back")
      .installed
      .iter()
      .find(|e| e.name == name)
      .unwrap_or_else(|| panic!("{name} is recorded"))
      .checksum
      .clone()
  }

  /// Write a source skill with a `SKILL.md` and whatever else is named.
  fn source(&self, name: &str, files: &[(&str, &str)]) {
    let dir = self.canon().join(name);
    write_tree(&dir, files);
  }
}

fn write_tree(dir: &Path, files: &[(&str, &str)]) {
  fs::create_dir_all(dir).unwrap();
  for (rel, body) in files {
    let path = dir.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, body).unwrap();
  }
}

fn read(path: &Path) -> String {
  fs::read_to_string(path).unwrap()
}

fn outcome(steps: &[intentsvcs::payload::Step], name: &str) -> Outcome {
  steps
    .iter()
    .find(|s| s.name == name)
    .unwrap_or_else(|| panic!("no step for `{name}` in {steps:?}"))
    .outcome
    .clone()
}

fn one(name: &str) -> Vec<String> {
  vec![name.to_string()]
}

// ---------------------------------------------------------------------------
// The defect this whole change exists for.
// ---------------------------------------------------------------------------

/// **THE BLIND SPOT, INVERTED. This is the arm that fails under v2's rule.**
///
/// Driven against v2 at HEAD before any of this was written: source
/// `scripts/run.sh` changed, `SKILL.md` untouched, `sync --force` prints
/// `up to date`, exits 0, and the installed script is stale. `--force` never
/// reaches it -- v2's force is read only inside the local-modification branch,
/// which is guarded by `source == old` and so cannot run when the source moved.
///
/// Field provenance: surfaced 2026-05-21 during the v2.11.8 gate-deadlock fix,
/// on a script-only edit to `release-gate.sh` -- the in-session gate's own
/// releaser, which every session in every project runs.
#[test]
fn a_change_confined_to_a_script_propagates() {
  let f = Fixture::new();
  f.source(
    "in-probe",
    &[("SKILL.md", "# probe\n"), ("scripts/run.sh", "ORIGINAL\n")],
  );
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();
  assert_eq!(
    read(&f.target.join("in-probe/scripts/run.sh")),
    "ORIGINAL\n"
  );

  // Only the script moves. SKILL.md is byte-identical.
  fs::write(f.canon().join("in-probe/scripts/run.sh"), "CHANGED\n").unwrap();

  let report = s.sync(false).unwrap();
  assert!(
    matches!(outcome(&report.steps, "in-probe"), Outcome::Updated { .. }),
    "a scripts-only change must be seen: {:?}",
    report.steps
  );
  assert_eq!(read(&f.target.join("in-probe/scripts/run.sh")), "CHANGED\n");
}

/// The control that makes the arm above mean something: an unchanged tree is
/// left alone. A sync that copied unconditionally would pass the test above and
/// be worthless.
#[test]
fn an_unchanged_tree_is_up_to_date() {
  let f = Fixture::new();
  f.source(
    "in-probe",
    &[("SKILL.md", "# probe\n"), ("scripts/run.sh", "ORIGINAL\n")],
  );
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();
  let report = s.sync(false).unwrap();
  assert_eq!(outcome(&report.steps, "in-probe"), Outcome::UpToDate);
}

/// A rename inside a skill is a change, even though no file's CONTENT differs.
/// A content-only digest reports it as no change -- the same blind spot one
/// axis over.
#[test]
fn renaming_a_file_inside_a_skill_is_a_change() {
  let f = Fixture::new();
  f.source(
    "in-probe",
    &[("SKILL.md", "# probe\n"), ("scripts/old.sh", "BODY\n")],
  );
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();

  fs::remove_file(f.canon().join("in-probe/scripts/old.sh")).unwrap();
  fs::write(f.canon().join("in-probe/scripts/new.sh"), "BODY\n").unwrap();

  let report = s.sync(false).unwrap();
  assert!(
    matches!(outcome(&report.steps, "in-probe"), Outcome::Updated { .. }),
    "{:?}",
    report.steps
  );
  assert!(f.target.join("in-probe/scripts/new.sh").is_file());
  assert!(!f.target.join("in-probe/scripts/old.sh").exists());
}

// ---------------------------------------------------------------------------
// Ruling 5 -- prune what we installed, never what we found.
// ---------------------------------------------------------------------------

/// v2 is `cp -r source/* target/` and nothing clears the target, so a retired
/// script stays live in every consumer forever while sync reports success.
#[test]
fn a_script_retired_upstream_is_removed() {
  let f = Fixture::new();
  f.source(
    "in-probe",
    &[
      ("SKILL.md", "# probe\n"),
      ("scripts/keep.sh", "KEEP\n"),
      ("scripts/gone.sh", "GONE\n"),
    ],
  );
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();
  assert!(f.target.join("in-probe/scripts/gone.sh").is_file());

  fs::remove_file(f.canon().join("in-probe/scripts/gone.sh")).unwrap();
  let report = s.sync(false).unwrap();

  match outcome(&report.steps, "in-probe") {
    Outcome::Updated { removed, .. } => {
      assert_eq!(removed, vec!["scripts/gone.sh".to_string()])
    }
    other => panic!("expected an update that pruned, got {other:?}"),
  }
  assert!(!f.target.join("in-probe/scripts/gone.sh").exists());
  assert!(f.target.join("in-probe/scripts/keep.sh").is_file());
}

/// **THE BOUNDARY, AND IT IS WHY THE MANIFEST RECORDS A FILE LIST.** A prune
/// keyed on "what is in the target but not the source" is correct and
/// destructive: it deletes the operator's own file. A sync may remove what it
/// INSTALLED; it may not remove what it FOUND.
#[test]
fn a_file_the_operator_added_is_never_pruned() {
  let f = Fixture::new();
  f.source(
    "in-probe",
    &[("SKILL.md", "# probe\n"), ("scripts/keep.sh", "KEEP\n")],
  );
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();

  // The operator drops their own note inside the installed skill.
  fs::write(f.target.join("in-probe/MY-NOTES.md"), "mine\n").unwrap();

  // **`force`, AND IT IS LOAD-BEARING RATHER THAN CONVENIENT.** Adding a file
  // moves the installed tree, so with upstream also moving this is
  // `Conflicted` and an unforced sync REFUSES -- correctly. Two earlier
  // versions of this test therefore never reached the prune at all: they were
  // green because nothing ran, under a name promising the prune had been
  // measured. Forcing is what puts the copy on the path being asserted about.
  //
  // **TWO ROUNDS, AND THE SECOND IS THE ONE THAT BITES.** A mutation recording
  // every file PRESENT in the target -- rather than only the ones this tool
  // wrote -- is invisible on the first pass, because the prune reads the PRIOR
  // entry and the operator's file is not in it yet. It only removes the file on
  // the run AFTER the one that mis-recorded it. A boundary that holds once and
  // fails on the next pass is not a boundary.
  for body in ["# probe v2\n", "# probe v3\n"] {
    fs::write(f.canon().join("in-probe/SKILL.md"), body).unwrap();
    let report = s.sync(true).unwrap();
    // `Forced`, because the operator's added file MOVED the installed tree and
    // this run is forced over it. The assertion's job is unchanged -- prove the
    // copy ran, or the prune below measures nothing.
    assert!(
      matches!(outcome(&report.steps, "in-probe"), Outcome::Forced { .. }),
      "the copy must actually have run, or this test measures nothing: {:?}",
      report.steps
    );
    assert_eq!(
      read(&f.target.join("in-probe/MY-NOTES.md")),
      "mine\n",
      "a sync removed a file it never installed"
    );
    let m = s.manifest().unwrap();
    let entry = m.installed.iter().find(|e| e.name == "in-probe").unwrap();
    assert!(
      !entry.files.contains(&"MY-NOTES.md".to_string()),
      "the manifest claimed a file this tool never wrote: {:?}",
      entry.files
    );
  }
}

/// The same rule in the sibling verb. v2 does `rm -rf` on uninstall, which
/// destroys the operator's file; a rule that holds in one verb and not its
/// sibling is not a rule.
#[test]
fn uninstall_removes_what_it_installed_and_leaves_what_it_found() {
  let f = Fixture::new();
  f.source(
    "in-probe",
    &[("SKILL.md", "# probe\n"), ("scripts/run.sh", "BODY\n")],
  );
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();
  fs::write(f.target.join("in-probe/MY-NOTES.md"), "mine\n").unwrap();

  let report = s.uninstall(&one("in-probe")).unwrap();
  match outcome(&report.steps, "in-probe") {
    Outcome::Removed { removed, left } => {
      assert!(removed.contains(&"SKILL.md".to_string()), "{removed:?}");
      assert!(
        removed.contains(&"scripts/run.sh".to_string()),
        "{removed:?}"
      );
      assert_eq!(left, vec!["MY-NOTES.md".to_string()]);
    }
    other => panic!("expected a removal that reported what it left, got {other:?}"),
  }
  assert!(f.target.join("in-probe/MY-NOTES.md").is_file());
  assert!(!f.target.join("in-probe/SKILL.md").exists());
  assert!(!f.target.join("in-probe/scripts").exists());
}

// ---------------------------------------------------------------------------
// Ruling 4 -- no baseline means report and refuse, never pick.
// ---------------------------------------------------------------------------

/// **THE REFUSAL PATH, WHICH vc FLAGGED WOULD OTHERWISE SHIP UNTESTED.**
///
/// A skill is installed and this build has no entry for it -- v2 put it there,
/// or a human did. Its tree differs from source, and NOTHING RECORDED tells an
/// upstream change from an operator's edit. So the sync reports and takes
/// neither remedy.
#[test]
fn an_installed_skill_with_no_baseline_is_refused_not_guessed() {
  let f = Fixture::new();
  f.source(
    "in-probe",
    &[
      ("SKILL.md", "# upstream\n"),
      ("scripts/run.sh", "UPSTREAM\n"),
    ],
  );
  // Installed by something that is not this build: no manifest entry at all.
  write_tree(
    &f.target.join("in-probe"),
    &[("SKILL.md", "# theirs\n"), ("scripts/run.sh", "THEIRS\n")],
  );

  let s = f.skills();
  let report = s.sync(false).unwrap();

  assert_eq!(outcome(&report.steps, "in-probe"), Outcome::Undecidable);
  assert_eq!(
    read(&f.target.join("in-probe/scripts/run.sh")),
    "THEIRS\n",
    "the refusal must not have written anything"
  );
}

/// **`--force` RESOLVES IT BY DISCARDING FORWARD, AND NEVER BY INVENTING A
/// BASELINE.**
///
/// **THIS TEST WAS AMENDED RATHER THAN DELETED, AND RENAMED OFF A PREMISE THAT
/// IS HALF DEAD** (vc, 2026-08-23, condition 1 of the grant). It read
/// `force_does_not_resolve_a_missing_baseline` and argued: *force is about
/// overriding a prompt, not about inventing information that was never
/// recorded.*
///
/// **THE FIRST CLAUSE DIED AND THE SECOND DID NOT, AND I HAD THE SPLIT WRONG.**
/// I argued the whole name was retired because v3 has no prompt to override.
/// That answers the first clause only. *Not inventing information that was
/// never recorded* is AC-07.3(d), ratified: with `old` absent, what
/// distinguishes an upstream change from an operator edit was never written
/// down and is not recoverable. **Taking my reasoning would have retired a live
/// constraint along with a dead one** -- which is why an argued test goes to
/// someone who did not author both sides.
///
/// **SO THE SURVIVING CLAUSE IS WHAT THIS NOW PINS.** (d) forbids CHOOSING;
/// AC-07.3(e) licenses DESTROYING WITH A RECORD. Force declines to know,
/// discards forward on the operator's instruction, and **the manifest records
/// THE NEW STATE -- never the discarded tree.** Laundering unknown bytes into a
/// baseline would make the very next sync report an ordinary update, on
/// evidence nobody ever had.
#[test]
fn force_never_invents_a_baseline_it_discards_forward_and_records_the_new_state() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# upstream\n")]);
  write_tree(&f.target.join("in-probe"), &[("SKILL.md", "# theirs\n")]);
  let s = f.skills();

  // Held without force -- (d)'s refusal, unchanged.
  assert_eq!(
    outcome(&s.sync(false).unwrap().steps, "in-probe"),
    Outcome::Undecidable
  );
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# theirs\n");

  // Forced: adopts the source, and SAYS the baseline was absent.
  let discarded = match outcome(&s.sync(true).unwrap().steps, "in-probe") {
    Outcome::Forced {
      discarded,
      baseline,
      ..
    } => {
      assert_eq!(
        baseline,
        Baseline::Absent,
        "a discard with no baseline must not be reported as the operator's edit: (d) says that is not knowable"
      );
      discarded
    }
    other => panic!("force must resolve a missing baseline: {other:?}"),
  };
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# upstream\n");

  // **THE CLAUSE THAT SURVIVED: the recorded baseline is the NEW state.** If
  // the discarded tree were recorded instead, the next sync would see
  // `source != old` and report a routine update -- inventing, one command
  // later, exactly the history (d) says was never available.
  let recorded = f.manifest_checksum("in-probe");
  assert_ne!(
    recorded, discarded,
    "the discarded tree was laundered into the baseline"
  );
  assert_eq!(
    outcome(&s.sync(false).unwrap().steps, "in-probe"),
    Outcome::UpToDate,
    "after a forced adopt the tree IS the source, so the next sync has nothing to do"
  );
}

/// The other half of ruling 4, and the reason it is not simply "refuse when
/// unknown": when the installed tree and the source are byte-identical there is
/// no distinction to lose, so adopting it costs nothing and stops the refusal
/// repeating forever.
#[test]
fn a_byte_identical_unrecorded_skill_is_adopted_rather_than_refused() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# same\n")]);
  write_tree(&f.target.join("in-probe"), &[("SKILL.md", "# same\n")]);

  let s = f.skills();
  assert_eq!(
    outcome(&s.sync(false).unwrap().steps, "in-probe"),
    Outcome::UpToDate
  );

  // Adopted: the NEXT sync has a baseline, so a real change now propagates.
  fs::write(f.canon().join("in-probe/SKILL.md"), "# moved\n").unwrap();
  assert!(matches!(
    outcome(&s.sync(false).unwrap().steps, "in-probe"),
    Outcome::Updated { .. }
  ));
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# moved\n");
}

// ---------------------------------------------------------------------------
// Ruling 2 -- the conflict v2's prompt cannot see.
// ---------------------------------------------------------------------------

/// **BOTH SIDES MOVED. v2 OVERWRITES AND CALLS IT `update available`.**
/// Its write is unconditional and downstream of every branch
/// (`claude_plugin_helpers.sh:430`); the `if`/`elif` picks the wording, not
/// the outcome. A genuine conflict misses the local-modification prompt --
/// guarded by `source == old` -- and is then reported exactly as a routine
/// upstream bump, so nothing the operator sees separates the run that
/// destroyed their edit from the run that did not.
#[test]
fn both_sides_changed_is_a_conflict_and_is_not_silently_overwritten() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# base\n")]);
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();

  fs::write(f.canon().join("in-probe/SKILL.md"), "# upstream\n").unwrap();
  fs::write(f.target.join("in-probe/SKILL.md"), "# mine\n").unwrap();

  let report = s.sync(false).unwrap();
  assert_eq!(outcome(&report.steps, "in-probe"), Outcome::Conflicted);
  assert_eq!(
    read(&f.target.join("in-probe/SKILL.md")),
    "# mine\n",
    "the operator's edit was overwritten without being reported"
  );
}

/// Only the installed tree moved. This is the case v2's prompt was built for,
/// and it is the one case v2 handles correctly.
#[test]
fn a_local_edit_alone_is_reported_and_held() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# base\n")]);
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();
  fs::write(f.target.join("in-probe/SKILL.md"), "# mine\n").unwrap();

  assert_eq!(
    outcome(&s.sync(false).unwrap().steps, "in-probe"),
    Outcome::ModifiedLocally
  );
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# mine\n");

  // ...and force is what overrides it, which is v2's contract for this case.
  //
  // **`Forced` RATHER THAN `Updated`, AND THE NARROWING IS THE POINT.** The
  // operator's tree had MOVED, so this run destroys their work -- and the whole
  // of vc's ruling is that such a run must not be reportable as the routine
  // update it otherwise resembles. Asserting `Updated { .. }` here would pass
  // for exactly the reporting v2 gives, which is the defect.
  assert!(matches!(
    outcome(&s.sync(true).unwrap().steps, "in-probe"),
    Outcome::Forced { .. }
  ));
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# base\n");
}

// ---------------------------------------------------------------------------
// The manifest's own contract.
// ---------------------------------------------------------------------------

/// Ruling 3: a stored value whose meaning changed must not keep its old
/// version, and the scope is declared as a field rather than implied.
#[test]
fn the_manifest_declares_what_its_checksums_cover() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# probe\n")]);
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();

  let m = s.manifest().unwrap();
  assert_eq!(m.scope(), Scope::Tree);
  assert_ne!(m.version, "1.0.0", "v2's version must not be reused");

  let entry = m.installed.iter().find(|e| e.name == "in-probe").unwrap();
  assert_eq!(entry.files, vec!["SKILL.md".to_string()]);
  assert!(entry.installed_at.ends_with('Z') || entry.installed_at.contains('+'));
}

/// **v3 MUST NOT WRITE v2's FILE.** Same path plus a differently-computed
/// `checksum` is a perpetual mutual clobber: v2 finds `source != old`
/// unconditionally, overwrites; v3 sees v2's value, overwrites back; both print
/// `updated` forever. The cutover state -- both binaries on `PATH` -- is the
/// collision state, so this is not hypothetical.
#[test]
fn the_v3_manifest_is_not_v2s_file() {
  assert_ne!(MANIFEST_RELATIVE, "skills/installed-skills.json");
}

/// A manifest that declares a scope this build cannot interpret has no usable
/// baseline, so every skill under it takes the refusal path rather than being
/// compared against values that answer a different question.
#[test]
fn an_uninterpretable_scope_yields_no_baseline() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# upstream\n")]);
  write_tree(&f.target.join("in-probe"), &[("SKILL.md", "# theirs\n")]);
  fs::create_dir_all(f.manifest.parent().unwrap()).unwrap();
  fs::write(
    &f.manifest,
    r#"{"version":"9.0.0","checksum_scope":"something-later","installed":[
      {"name":"in-probe","source_path":"/x","installed_at":"2026-01-01T00:00:00Z","checksum":"deadbeef","files":["SKILL.md"]}]}"#,
  )
  .unwrap();

  let s = f.skills();
  assert_eq!(s.manifest().unwrap().scope(), Scope::Undeclared);
  assert_eq!(
    outcome(&s.sync(false).unwrap().steps, "in-probe"),
    Outcome::Undecidable
  );
}

/// A manifest that exists and cannot be parsed is a broken install, not an
/// empty one. Treating it as empty would silently re-install everything and
/// overwrite whatever the operator had.
#[test]
fn an_unreadable_manifest_is_an_error_not_an_empty_one() {
  let f = Fixture::new();
  fs::create_dir_all(f.manifest.parent().unwrap()).unwrap();
  fs::write(&f.manifest, "{ this is not json").unwrap();
  assert!(f.skills().manifest().is_err());
}

// ---------------------------------------------------------------------------
// Resolution: roots, precedence, and the names that reach the filesystem.
// ---------------------------------------------------------------------------

/// An extension skill overrides the shipped one of the same name -- which is
/// what an extension is FOR -- and the shadowing is REPORTED rather than
/// silent.
#[test]
fn an_extension_shadows_canon_and_says_so() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# canon\n")]);
  let ext = f.install.parent().unwrap().join("ext");
  write_tree(
    &ext.join("mine/skills/in-probe"),
    &[("SKILL.md", "# ext\n")],
  );

  let s = f.with_ext(&ext);
  let report = s.install(&one("in-probe"), false).unwrap();
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# ext\n");

  let step = report
    .steps
    .iter()
    .find(|s| s.name == "in-probe")
    .expect("a step for the installed skill");
  assert!(
    step.shadowed.is_some(),
    "shadowing must be reported, not silent"
  );
}

/// **A NAME THAT REACHES THE FILESYSTEM IS A NAME THAT CAN CONTAIN `../`.**
#[test]
fn a_name_that_could_escape_the_root_is_refused() {
  let f = Fixture::new();
  let s = f.skills();
  for bad in ["../etc", "a/b", "", ".", "..", "with space", "dot.dot"] {
    assert!(
      s.origins(bad).is_err(),
      "`{bad}` was accepted as a skill name"
    );
  }
  assert!(s.origins("in-probe_2").is_ok());
}

/// A directory without a `SKILL.md` is not a skill, which is v2's test too.
#[test]
fn a_directory_without_a_skill_md_is_not_a_skill() {
  let f = Fixture::new();
  fs::create_dir_all(f.canon().join("not-a-skill")).unwrap();
  fs::write(f.canon().join("not-a-skill/README.md"), "x").unwrap();
  f.source("in-real", &[("SKILL.md", "# real\n")]);

  let names: Vec<String> = f
    .skills()
    .available()
    .unwrap()
    .into_iter()
    .map(|o| o.name)
    .collect();
  assert_eq!(names, vec!["in-real".to_string()]);
}

/// Installing something the install does not carry is reported, not a crash,
/// and it does not stop the rest of the batch.
#[test]
fn a_missing_source_is_reported_and_does_not_stop_the_batch() {
  let f = Fixture::new();
  f.source("in-real", &[("SKILL.md", "# real\n")]);
  let report = f
    .skills()
    .install(&[String::from("in-absent"), String::from("in-real")], false)
    .unwrap();
  assert_eq!(outcome(&report.steps, "in-absent"), Outcome::SourceMissing);
  assert!(matches!(
    outcome(&report.steps, "in-real"),
    Outcome::Installed { .. }
  ));
}

/// Re-installing without `--force` reports rather than overwriting. Prompting
/// belongs to the terminal skin, not to the operation.
#[test]
fn installing_over_an_existing_skill_needs_force() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# probe\n")]);
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();
  fs::write(f.canon().join("in-probe/SKILL.md"), "# newer\n").unwrap();

  assert_eq!(
    outcome(
      &s.install(&one("in-probe"), false).unwrap().steps,
      "in-probe"
    ),
    Outcome::AlreadyInstalled
  );
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# probe\n");

  assert!(matches!(
    outcome(
      &s.install(&one("in-probe"), true).unwrap().steps,
      "in-probe"
    ),
    Outcome::Forced { .. }
  ));
  assert_eq!(read(&f.target.join("in-probe/SKILL.md")), "# newer\n");
}

/// Symlinks are not followed. Copying through one reaches content the source
/// root does not own, which turns an install into an arbitrary read of the
/// operator's disk.
#[cfg(unix)]
#[test]
fn a_symlink_in_a_source_skill_is_not_followed() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# probe\n")]);
  let secret = f.install.parent().unwrap().join("secret.txt");
  fs::write(&secret, "SECRET\n").unwrap();
  std::os::unix::fs::symlink(&secret, f.canon().join("in-probe/leak.txt")).unwrap();

  f.skills().install(&one("in-probe"), false).unwrap();
  assert!(
    !f.target.join("in-probe/leak.txt").exists(),
    "a symlink was followed out of the source root"
  );
}

/// **THE DISCARDED CHECKSUM DESCRIBES WHAT WAS DISCARDED, AND THAT IS DRIVEN
/// RATHER THAN ASSUMED.**
///
/// vc's ruling makes this number the whole remedy: once the copy has run it is
/// the only artefact that can identify the operator's tree. **A number that is
/// merely PRESENT satisfies the shape of the ruling and none of its purpose**,
/// so this asserts the property that makes it useful -- same content, same
/// checksum; different content, different checksum.
#[test]
fn the_discarded_checksum_is_a_function_of_what_was_discarded() {
  let discard = |edit: &str| {
    let f = Fixture::new();
    f.source("in-probe", &[("SKILL.md", "# upstream\n")]);
    let s = f.skills();
    s.install(&one("in-probe"), false).unwrap();
    fs::write(f.target.join("in-probe/SKILL.md"), edit).unwrap();
    match outcome(&s.sync(true).unwrap().steps, "in-probe") {
      Outcome::Forced { discarded, .. } => discarded,
      other => panic!("a forced run over a moved tree must report a discard: {other:?}"),
    }
  };

  assert_eq!(
    discard("# mine\n"),
    discard("# mine\n"),
    "the same discarded content must yield the same checksum, or it identifies nothing"
  );
  assert_ne!(
    discard("# mine\n"),
    discard("# something else entirely\n"),
    "a checksum that does not move with the content is a constant wearing a remedy's name"
  );
}

/// **FORCING OVER A TREE NOBODY TOUCHED IS AN ORDINARY UPDATE AND SAYS SO.**
///
/// The discriminator is `target_moved`, not the flag: the flag says what the
/// operator ASKED FOR and the state says what actually HAPPENED. **Reporting a
/// discard here would be false** -- nothing was lost -- and it would teach an
/// operator to skim past the line on the runs where something was.
#[test]
fn forcing_over_an_unmoved_tree_claims_no_discard() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# v1\n")]);
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();

  // Upstream moves; the installed tree does not.
  fs::write(f.canon().join("in-probe/SKILL.md"), "# v2\n").unwrap();

  assert!(
    matches!(
      outcome(&s.sync(true).unwrap().steps, "in-probe"),
      Outcome::Updated { .. }
    ),
    "a forced run that destroyed nothing must not report a discard"
  );
}

/// **`--force` RESOLVES A CONFLICT, WHICH IS THE OTHER STATE IT REACHES.**
///
/// Held without it (`both_sides_changed_is_a_conflict_and_is_not_silently_overwritten`
/// above pins that); adopted with it, and the operator's tree is named on the
/// way out. The third held state, `Undecidable`, is deliberately NOT resolved --
/// see the hold recorded at its arm in `skills.rs`.
#[test]
fn force_resolves_a_conflict_and_names_what_it_discarded() {
  let f = Fixture::new();
  f.source("in-probe", &[("SKILL.md", "# v1\n")]);
  let s = f.skills();
  s.install(&one("in-probe"), false).unwrap();

  // Both sides move.
  fs::write(f.canon().join("in-probe/SKILL.md"), "# upstream v2\n").unwrap();
  fs::write(f.target.join("in-probe/SKILL.md"), "# mine\n").unwrap();

  assert_eq!(
    outcome(&s.sync(false).unwrap().steps, "in-probe"),
    Outcome::Conflicted
  );
  assert!(matches!(
    outcome(&s.sync(true).unwrap().steps, "in-probe"),
    Outcome::Forced { .. }
  ));
  assert_eq!(
    read(&f.target.join("in-probe/SKILL.md")),
    "# upstream v2\n",
    "force adopts the upstream copy"
  );
}

/// **THE DISCARDED CHECKSUM IS A FUNCTION OF THE DISCARDED CONTENT IN THE
/// MISSING-BASELINE STATE TOO** (vc, 2026-08-23, condition 3 of the grant).
///
/// `the_discarded_checksum_is_a_function_of_what_was_discarded` proves this
/// where a baseline exists, and **it would stay green if this state reported a
/// constant** -- different arm, different construction site. The remedy is
/// worth MORE here than there, not less: with a baseline the operator at least
/// knows the bytes were theirs, and here the checksum is the only handle on
/// content nobody can otherwise characterise.
#[test]
fn the_discarded_checksum_moves_with_the_content_when_no_baseline_exists() {
  let discard = |theirs: &str| {
    let f = Fixture::new();
    f.source("in-probe", &[("SKILL.md", "# upstream\n")]);
    // Installed by something that is not this build: no manifest entry at all.
    write_tree(&f.target.join("in-probe"), &[("SKILL.md", theirs)]);
    match outcome(&f.skills().sync(true).unwrap().steps, "in-probe") {
      // **THE BASELINE IS DELIBERATELY NOT ASSERTED HERE.** It is the other
      // test's property, and pinning it in both makes one mutation fail two
      // names -- which is exactly what a mutation proof exists to tell apart.
      Outcome::Forced { discarded, .. } => discarded,
      other => panic!("force must resolve a missing baseline: {other:?}"),
    }
  };

  assert_eq!(
    discard("# theirs\n"),
    discard("# theirs\n"),
    "the same discarded content must yield the same checksum, or it identifies nothing"
  );
  assert_ne!(
    discard("# theirs\n"),
    discard("# something else entirely\n"),
    "a checksum that does not move with the content is a constant wearing a remedy's name"
  );
}
