//! **AN ESTATE HAS NO WAY TO LEARN THAT ITS COMMIT GATE IS NOT RUNNING** (vc's
//! finding, 2026-08-27; built under vc's pen).
//!
//! Found on Baize: config `3.0.0`, canon present, fully ported, four whiteboard
//! nodes, and a `pre-commit.intent` carrying no guard block whatsoever. `doctor
//! --verbose` there printed 139 lines with ZERO mentions of `hook`, `gate`,
//! `guard`, `INTENT_HOME` or `pre-commit`. **Nobody noticed, and nobody could
//! have**: an unwired guard does not fail, it reports nothing, and reporting
//! nothing is indistinguishable from passing.
//!
//! # The severity is split by PROPERTY, and the split is the design
//!
//! One severity for all four properties is what would have made this useless.
//!
//! | property                                   | severity                | reds today |
//! | ------------------------------------------- | ------------------------ | ----------- |
//! | installed and cannot execute               | `GateNotRunning`, GATES | 2 of 17    |
//! | behind the template                        | `Advisory`, not counted | **17 of 17** |
//!
//! **THE SECOND ROW IS WHY IT IS AN ADVISORY.** dc proved by `cmp` that the
//! current template is installed in ZERO estates, Intent's own included, so
//! gating it would make every estate in the fleet permanently red -- and a check
//! that is always red is one operators learn to skip, at which point it is not
//! there for the two that are genuinely broken either.
//!
//! # An ABSENT carrier is deliberately not a finding
//!
//! Nothing here demands that a project HAVE a gate; faulting an absent one
//! would fault every non-adopting estate for a choice it made. **The
//! discriminator is whether anything REFERENCES the carrier.** Two absent files
//! are an opt-out; a chain calling a file that is not there is an estate that
//! believes it is protected and is not. Both arms are driven below, because
//! they differ only in a file the naive check does not read.
//!
//! # The IO edge is driven too, because I nearly shipped it on inspection
//!
//! The verdict below is pure and every arm of it is driven. **The EDGE that
//! feeds it was not**, and it carries a real decision: a tree that is not a git
//! repository reports NOTHING rather than a missing gate, because `doctor` runs
//! on trees that are neither repositories nor projects and a missing hook is
//! not a defect of a directory that cannot have hooks. That arm was verified by
//! reading it -- **in a file whose own argument is that a check about silently
//! unprotected estates must not ship arms verified by inspection.** Both edge
//! arms are now driven: a non-repository says nothing, and a real repository
//! carrying an unwired carrier reports through the whole path.
//!
//! # Why the rest drive `gate_state` and not the whole of `doctor`
//!
//! Two arms are unreachable through the IO path on a healthy developer machine:
//! `NoResolvableInstall` needs a machine with no install, and `Current` needs a
//! carrier byte-identical to whatever template that machine holds. **A check
//! about estates being silently unprotected must not itself ship arms verified
//! only by inspection**, so the verdict is a pure function of three texts and
//! every arm is driven here. The IO edge -- resolving the hooks path through
//! `git rev-parse --git-path hooks`, so a `core.hooksPath` redirect and a linked
//! worktree both resolve the way the shipped chain resolves them -- is exercised
//! by `doctor` on the real estate rather than faked here.
//!
//! # Mutations, measured -- each revert re-run to a green baseline
//!
//! | mutation                                              | reds                                          |
//! | ------------------------------------------------------ | ---------------------------------------------- |
//! | the missing-carrier arm stops reading the chain        | `an_absent_carrier_...` ONLY                  |
//! | the guard-runner marker check is removed              | `a_carrier_that_names_no_guard_runner_...`    |
//! | `BehindTheTemplate` becomes `Current`                 | the older-template test AND the same-size one |
//! | the byte comparison becomes a length comparison       | `a_carrier_the_same_size_...` ONLY            |
//! | the IO edge reports nothing, ever                     | `a_repository_carrying_an_unwired_carrier_...`|
//! | the edge stops checking `git rev-parse`'s exit code   | `a_root_level_file_named_pre_commit_...` ONLY |
//! | the check is hoisted above the migration early-return | the unmigrated-reach test AND the IO test     |
//!
//! The last row is the REACH LIMIT arm. Hoisting makes the check run on
//! unmigrated estates, which is precisely the change the limit records -- and
//! it reds the IO test too, because the finding is then produced twice.
//!
//! **THE LAST ROW REDDED NOTHING WHEN FIRST MEASURED, AND THAT IS WHY THE TEST
//! BESIDE IT EXISTS.** Dropping the exit-code check leaves git's empty stdout,
//! and `root.join("")` is the project root -- so the whole edge behaved
//! identically, on every fixture, with a guard removed. The input that
//! separates them is a tree keeping hook sources at its root, which is an
//! ordinary layout. **A defensive branch that no input can distinguish is
//! indistinguishable from dead code, and the honest repair is to find the input
//! rather than to write the row down as covered.**
//!
//! Row three reds TWO, which the forecast had as one: a state that never
//! reports "behind" fails every input that should report it, same-length or
//! not. Row four is the pair's discriminator and reds alone -- **a length
//! comparison agrees with a byte comparison on every realistic carrier**,
//! because templates grow, and disagrees only on an edit that changes what the
//! guards do without changing how long the file is.

mod common;

use intentsvcs::doctor::{self, GateState, gate_state};
use intentsvcs::finding::FindingClass;

/// A carrier that would actually run guards.
const WIRED: &str =
  "#!/usr/bin/env bash\nGUARD_RUNNER=\"$GUARD_HOME/lib/templates/hooks/pre-commit-guards.sh\"\n";

/// The Baize carrier: every appearance of a hook, running nothing.
const UNWIRED: &str = "#!/usr/bin/env bash\n# Intent critic gate\necho 'pre-commit'\n";

#[test]
fn no_carrier_and_nothing_calling_one_is_a_choice_rather_than_a_fault() {
  assert_eq!(
    gate_state(None, Some("#!/bin/sh\nexec prettier\n"), Some(WIRED)),
    GateState::NotInstalled,
    "a project that never installed the gate must not be reported for it, or every \
     non-adopting estate is faulted for a decision it made"
  );
  assert_eq!(
    gate_state(None, None, Some(WIRED)),
    GateState::NotInstalled,
    "and neither is a project with no chain at all"
  );
}

#[test]
fn an_absent_carrier_that_something_calls_is_broken() {
  assert_eq!(
    gate_state(
      None,
      Some("#!/bin/sh\n. \"$(git rev-parse --git-path hooks)/pre-commit.intent\"\n"),
      Some(WIRED)
    ),
    GateState::ChainCallsAMissingCarrier,
    "a chain calling a carrier that is not there is an estate that believes it is \
     protected and is not -- and it differs from the opt-out case ONLY in the chain, \
     which is the file a naive existence check never reads"
  );
}

#[test]
fn a_carrier_that_names_no_guard_runner_is_the_baize_state() {
  assert_eq!(
    gate_state(Some(UNWIRED), None, Some(WIRED)),
    GateState::CarrierRunsNoGuards,
    "a carrier can carry every comment the template has and still execute nothing; \
     what makes guards run is the runner path, so that is what is looked for"
  );
}

#[test]
fn guards_read_from_an_install_that_cannot_be_resolved_do_not_run() {
  assert_eq!(
    gate_state(Some(WIRED), None, None),
    GateState::NoResolvableInstall,
    "the roster is read LIVE out of the install, so a carrier that cannot find one \
     runs and finds no guards -- which reports nothing, exactly like passing"
  );
}

#[test]
fn a_carrier_older_than_its_template_is_reported_and_not_counted() {
  let older = format!("{WIRED}# and one more line the template has since grown\n");
  assert_eq!(
    gate_state(Some(WIRED), None, Some(&older)),
    GateState::BehindTheTemplate {
      carrier: WIRED.len(),
      template: older.len()
    },
    "the carrier is a copy taken at install time and nothing re-copies it"
  );
  assert_eq!(
    gate_state(Some(WIRED), None, Some(WIRED)),
    GateState::Current,
    "and a carrier that matches its template is not reported at all -- without this \
     arm the one above passes under a check that reports every estate always"
  );
}

/// **THE COMPARISON IS BYTES, AND THIS IS THE ONLY INPUT THAT PROVES IT.**
///
/// A length comparison is the obvious cheap implementation and it agrees with a
/// byte comparison on every realistic pair of a carrier and its successor --
/// templates grow. It disagrees here, on the one case that matters: an edit
/// that changes what the guards DO without changing how long the file is.
#[test]
fn a_carrier_the_same_size_as_the_template_but_not_the_same_bytes_is_behind_it() {
  let template = format!("{WIRED}# aaaa\n");
  let carrier = format!("{WIRED}# bbbb\n");
  assert_eq!(
    template.len(),
    carrier.len(),
    "the fixture must be same-length, or this test is the ordinary case"
  );
  assert_eq!(
    gate_state(Some(&carrier), None, Some(&template)),
    GateState::BehindTheTemplate {
      carrier: carrier.len(),
      template: template.len()
    },
    "same size, different guards: a length comparison calls this current"
  );
}

// ---------------------------------------------------------------------------
// THE IO EDGE
// ---------------------------------------------------------------------------

/// **A TREE THAT IS NOT A REPOSITORY REPORTS NOTHING, WHICH IS A DECISION.**
///
/// `doctor` is what you run when everything else has stopped working, so it
/// runs on trees that are not projects and not repositories. A directory that
/// cannot have hooks does not have a broken gate, and saying it does would put
/// a permanent finding on every such tree -- the same permanent-red failure the
/// severity split exists to avoid, arriving through a different door.
#[test]
fn a_tree_that_is_not_a_repository_says_nothing_about_hooks() {
  let fx = common::Fixture::new();
  let facade = fx.facade_on_disk();
  let report = doctor::diagnose(&fx.project(), &common::ctx(), Some(facade.store()));
  let hooks: Vec<&str> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::GateNotRunning)
    .map(|f| f.detail.as_str())
    .collect();
  assert!(
    hooks.is_empty(),
    "a tempdir is not a repository and cannot have a hook, so it cannot have a broken one: {hooks:?}"
  );
}

/// **AND A REAL REPOSITORY WITH AN UNWIRED CARRIER REPORTS THROUGH THE WHOLE
/// PATH** -- git resolution, both file reads, the verdict, and the class.
///
/// The pair matters more than either half: without this arm the test above
/// passes under an edge that reports nothing ever, which is indistinguishable
/// from an edge that correctly says nothing about a tempdir.
#[test]
fn a_repository_carrying_an_unwired_carrier_is_reported_through_the_io() {
  let fx = common::Fixture::new();
  fx.git_init();
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  std::fs::write(hooks.join("pre-commit.intent"), UNWIRED).expect("plant the carrier");

  let facade = fx.facade_on_disk();
  let report = doctor::diagnose(&fx.project(), &common::ctx(), Some(facade.store()));
  let found: Vec<&str> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::GateNotRunning)
    .map(|f| f.detail.as_str())
    .collect();
  assert_eq!(
    found.len(),
    1,
    "the unwired carrier must be reported through the real IO path: {:?}",
    report
      .findings
      .iter()
      .map(|f| &f.detail)
      .collect::<Vec<_>>()
  );
  assert!(
    found[0].contains("names no guard runner"),
    "and it must be the Baize arm rather than some other gate state: {found:?}"
  );
  assert!(
    report.actionable() > 0,
    "a dead gate is actionable -- it is the half of this check that moves the exit code"
  );
}

/// **THE EXIT-CODE CHECK ON `git rev-parse` IS LOAD-BEARING, AND THIS IS THE
/// ONLY INPUT THAT SHOWS IT.**
///
/// When git fails it writes nothing to stdout, so dropping the check leaves an
/// empty path and `root.join("")` is the PROJECT ROOT. Everything then proceeds
/// against `<root>/pre-commit` and `<root>/pre-commit.intent` as though they
/// were hooks. On almost every tree those files do not exist and the mutation
/// is invisible -- it was, when first measured: the whole edge behaved
/// identically with the check removed.
///
/// A repository that keeps hook SOURCES at its root is the case where it stops
/// being invisible, and it is an ordinary layout rather than a contrivance.
/// There the tool would read a file that is not a hook, in a tree that is not a
/// repository, and report a broken gate that does not exist.
#[test]
fn a_root_level_file_named_pre_commit_is_not_mistaken_for_a_hook() {
  let fx = common::Fixture::new();
  // NOT a git repo, and carrying the two names at its root.
  std::fs::write(
    fx.root().join("pre-commit"),
    "#!/bin/sh\n. \"$(git rev-parse --git-path hooks)/pre-commit.intent\"\n",
  )
  .expect("write a root-level pre-commit");

  let facade = fx.facade_on_disk();
  let report = doctor::diagnose(&fx.project(), &common::ctx(), Some(facade.store()));
  let found: Vec<&str> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::GateNotRunning)
    .map(|f| f.detail.as_str())
    .collect();
  assert!(
    found.is_empty(),
    "a file at the project root is not a git hook, and a tree that is not a repository \
     has no gate to be broken: {found:?}"
  );
}

/// **THE CHECK CANNOT FIRE ON AN UNMIGRATED ESTATE, AND THAT IS A REACH LIMIT
/// RATHER THAN A CLEAN RESULT.**
///
/// `diagnose` returns at the migration arm before anything file-shaped is
/// reached -- correctly, because on an unmigrated project the model is empty
/// and every later check would describe a consequence rather than the cause.
/// The consequence for THIS check is that the estates least likely to have a
/// working gate are the ones it never looks at.
///
/// Measured live by conflab-vc the same evening: `intent doctor` on Conflab
/// reports `1 finding across 0 thread(s), 0 issue(s), 0 view(s), **0 file(s)**`.
/// Zero files scanned. **A new binary does not fix it and neither does this
/// check** -- only the port does, and the port is the same event that installs
/// the guards, which makes the check moot exactly where it was most wanted.
///
/// **IT IS DRIVEN RATHER THAN WRITTEN DOWN, because a limit recorded only in
/// prose is a limit that stops being true without anyone noticing.** If the
/// migration arm is ever moved below this check, this test reds and whoever
/// moved it learns that a check they had not considered now runs on estates it
/// was never measured against.
#[test]
fn an_unmigrated_estate_is_never_reached_and_that_is_the_limit_not_a_pass() {
  let fx = common::Fixture::new();
  fx.git_init();
  // Declare v2, which is what an unported estate carries.
  let config = fx.root().join("intent/.config/config.json");
  let text = std::fs::read_to_string(&config).expect("read config");
  std::fs::write(&config, text.replace("\"3.0.0\"", "\"2.11.0\"")).expect("declare v2");

  // A carrier that would certainly be reported, if anything looked at it.
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  std::fs::write(hooks.join("pre-commit.intent"), UNWIRED).expect("plant the carrier");

  // **`None` FOR THE STORE, AND THAT IS THE REALISTIC CASE RATHER THAN A
  // CONVENIENCE.** The first draft of this test called `facade_on_disk()` and
  // panicked: the facade REFUSES to open an unmigrated project, correctly. An
  // unmigrated estate is exactly the one whose store a caller cannot open, which
  // is why `diagnose` takes an `Option` at all -- so passing `None` is what a
  // real `intent doctor` does on Conflab tonight, not a fixture shortcut.
  let report = doctor::diagnose(&fx.project(), &common::ctx(), None);

  assert!(
    report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::Unmigrated),
    "the fixture must actually read as unmigrated, or this test proves nothing about \
     the arm it is named for: {:?}",
    report.findings.iter().map(|f| &f.class).collect::<Vec<_>>()
  );
  assert!(
    !report
      .findings
      .iter()
      .any(|f| f.class == FindingClass::GateNotRunning),
    "the gate check must NOT have run -- and the silence here is the limit being \
     recorded, not the estate being healthy. Its carrier runs no guards and nothing \
     said so"
  );
}
