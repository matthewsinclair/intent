//! **AN ESTATE HAS NO WAY TO LEARN THAT ITS COMMIT GATE IS NOT RUNNING** (vc's
//! finding, 2026-08-27; built under vc's pen).
//!
//! Found on Baize: config `3.0.0`, canon present, fully ported, whiteboard
//! nodes at work, and a `pre-commit.intent` carrying no guard block whatsoever.
//! `doctor --verbose` there printed a long report with NO mention of `hook`,
//! `gate`, `guard`, `INTENT_HOME` or `pre-commit`. **Nobody noticed, and nobody
//! could have**: an unwired guard does not fail, it reports nothing, and
//! reporting nothing is indistinguishable from passing.
//!
//! # The severity is split by PROPERTY, and the split is the design
//!
//! One severity for every property is what would have made this useless.
//!
//! | property                                   | severity                | reds today |
//! | ------------------------------------------- | ------------------------ | ----------- |
//! | installed and cannot execute               | `GateNotRunning`, GATES | a few      |
//! | behind the template                        | `Advisory`, not counted | **all**    |
//!
//! **THE SECOND ROW IS WHY IT IS AN ADVISORY.** dc proved by `cmp` that the
//! current template is installed in NO estate, Intent's own included, so gating
//! it would make every estate in the fleet permanently red -- and a check that
//! is always red is one operators learn to skip, at which point it is not there
//! for the ones that are genuinely broken either.
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
//! These arms are unreachable through the IO path on a healthy developer machine:
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
//! Row three reds both, which the forecast did not expect: a state that never
//! reports "behind" fails every input that should report it, same-length or
//! not. Row four is the pair's discriminator and reds alone -- **a length
//! comparison agrees with a byte comparison on every realistic carrier**,
//! because templates grow, and disagrees only on an edit that changes what the
//! guards do without changing how long the file is.

use intentsvcs::doctor::{self, CarrierShape, GateState, GateTemplates, carrier_shape, gate_state};
use intentsvcs::finding::FindingClass;

/// A carrier that would actually run guards.
const WIRED: &str =
  "#!/usr/bin/env bash\nGUARD_RUNNER=\"$GUARD_HOME/lib/templates/hooks/pre-commit-guards.sh\"\n";

/// The Baize carrier: every appearance of a hook, running nothing.
const UNWIRED: &str = "#!/usr/bin/env bash\n# Intent critic gate\necho 'pre-commit'\n";

/// The SHIM carrier: a locator. It names no guard runner, by design -- it
/// resolves `~/.intent/home` and execs the gate, and the gate reaches the runner.
const SHIM: &str = "#!/usr/bin/env bash\n# resolve the install and exec its gate\nif [ \"$1\" = --where ]; then\n  cat ~/.intent/home\nfi\n";

/// Most arms are about a MONOLITHIC carrier, so they name one template and this
/// says which. Written as a helper rather than repeated inline so that adding a
/// third carrier shape breaks the arms that must be revisited.
fn gate_only(t: &str) -> GateTemplates<'_> {
  GateTemplates {
    gate: Some(t),
    shim: None,
  }
}

#[test]
fn no_carrier_and_nothing_calling_one_is_a_choice_rather_than_a_fault() {
  assert_eq!(
    gate_state(None, Some("#!/bin/sh\nexec prettier\n"), gate_only(WIRED)),
    GateState::NotInstalled,
    "a project that never installed the gate must not be reported for it, or every \
     non-adopting estate is faulted for a decision it made"
  );
  assert_eq!(
    gate_state(None, None, gate_only(WIRED)),
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
      gate_only(WIRED)
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
    gate_state(Some(UNWIRED), None, gate_only(WIRED)),
    GateState::CarrierRunsNoGuards,
    "a carrier can carry every comment the template has and still execute nothing; \
     what makes guards run is the runner path, so that is what is looked for"
  );
}

#[test]
fn guards_read_from_an_install_that_cannot_be_resolved_do_not_run() {
  assert_eq!(
    gate_state(Some(WIRED), None, GateTemplates::default()),
    GateState::NoResolvableInstall,
    "the roster is read LIVE out of the install, so a carrier that cannot find one \
     runs and finds no guards -- which reports nothing, exactly like passing"
  );
}

#[test]
fn a_carrier_older_than_its_template_is_reported_and_not_counted() {
  let older = format!("{WIRED}# and one more line the template has since grown\n");
  assert_eq!(
    gate_state(Some(WIRED), None, gate_only(&older)),
    GateState::BehindTheTemplate {
      carrier: WIRED.len(),
      template: older.len()
    },
    "the carrier is a copy taken at install time and nothing re-copies it"
  );
  assert_eq!(
    gate_state(Some(WIRED), None, gate_only(WIRED)),
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
    gate_state(Some(&carrier), None, gate_only(&template)),
    GateState::BehindTheTemplate {
      carrier: carrier.len(),
      template: template.len()
    },
    "same size, different guards: a length comparison calls this current"
  );
}

// ---------------------------------------------------------------------------
// THE SECOND CARRIER SHAPE (issue 0105)
//
// `canon::install_carrier` writes a SHIM, and both halves of this check assumed
// the monolithic carrier: the runner test failed a shim as the Baize state, and
// the comparison then measured it against a template it is not a copy of.
// FIXING EITHER ALONE LEAVES THE ESTATE RED, which is why both are driven here.
// ---------------------------------------------------------------------------

#[test]
fn the_two_carrier_shapes_are_told_apart_in_both_directions() {
  assert_eq!(
    carrier_shape(SHIM),
    CarrierShape::Shim,
    "a carrier that answers --where is a locator"
  );
  assert_eq!(
    carrier_shape(WIRED),
    CarrierShape::Monolithic,
    "and one that names the runner carries the guards itself"
  );
  // **THE DISCRIMINATOR MUST BE TWO-SIDED OR IT IS A HEURISTIC.** Measured on
  // the shipped templates 2026-08-28: `--where` 2 in the shim and 0 in the gate
  // body; `pre-commit-guards.sh` 0 in the shim and 6 in the gate body. The
  // fixtures above are disjoint the same way, so neither assertion could pass
  // by accident of a marker present in both.
  assert!(!SHIM.contains("pre-commit-guards.sh"));
  assert!(!WIRED.contains("--where"));
}

#[test]
fn a_shim_carrier_is_not_the_baize_state() {
  assert_eq!(
    gate_state(
      Some(SHIM),
      None,
      GateTemplates {
        gate: Some(WIRED),
        shim: Some(SHIM),
      }
    ),
    GateState::Current,
    "a shim names no guard runner BY CONSTRUCTION, so testing it for one reports \
     every correctly-installed v3 estate as running nothing -- an ACTIONABLE red \
     on the estates that just did what they were asked"
  );
}

#[test]
fn a_shim_is_compared_against_the_shim_template_and_not_the_gate_body() {
  // The state before the fix, and the reason fixing only the runner test was
  // not enough: the comparand was the GATE BODY, which a shim can never equal,
  // so `BehindTheTemplate` was permanently true for every shim estate.
  assert_eq!(
    gate_state(
      Some(SHIM),
      None,
      GateTemplates {
        gate: Some(WIRED),
        shim: Some(SHIM),
      }
    ),
    GateState::Current,
    "the carrier IS the current shim; comparing it to the gate body reports it \
     behind a template it is not a copy of, forever"
  );
  let newer = format!("{SHIM}# and one line the shim template has since grown\n");
  assert_eq!(
    gate_state(
      Some(SHIM),
      None,
      GateTemplates {
        gate: Some(SHIM),
        shim: Some(&newer),
      }
    ),
    GateState::BehindTheTemplate {
      carrier: SHIM.len(),
      template: newer.len()
    },
    "and a genuinely stale shim is still reported -- without this arm the one \
     above passes under a check that has simply stopped looking at shims. NOTE \
     the gate template here is deliberately set to bytes that WOULD match the \
     carrier: if the comparand were still the gate body this arm would read \
     Current, so it fails on the wrong-comparand bug specifically"
  );
}

#[test]
fn a_shim_with_no_resolvable_shim_template_is_not_silently_current() {
  assert_eq!(
    gate_state(
      Some(SHIM),
      None,
      GateTemplates {
        gate: Some(WIRED),
        shim: None,
      }
    ),
    GateState::NoResolvableInstall,
    "an install that resolves the gate body but not the shim template cannot say \
     whether the shim is current, and must not answer Current by default -- the \
     gate template being present is not evidence about the shim"
  );
}

#[test]
fn a_monolithic_carrier_still_has_to_name_its_runner() {
  assert_eq!(
    gate_state(
      Some(UNWIRED),
      None,
      GateTemplates {
        gate: Some(WIRED),
        shim: Some(SHIM),
      }
    ),
    GateState::CarrierRunsNoGuards,
    "THE BAIZE ARM MUST SURVIVE THE FIX. Relaxing the runner test for shims is \
     only correct if it is still applied to carriers that are not shims; \
     otherwise this change trades a false red for the false green the check was \
     written to prevent"
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
  let fx = crate::common::Fixture::new();
  let facade = fx.facade_on_disk();
  let report = doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    Some(facade.store()),
    intentsvcs::doctor::Scope::All,
  );
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
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  std::fs::write(hooks.join("pre-commit.intent"), UNWIRED).expect("plant the carrier");

  let facade = fx.facade_on_disk();
  let report = doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    Some(facade.store()),
    intentsvcs::doctor::Scope::All,
  );
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
  let fx = crate::common::Fixture::new();
  // NOT a git repo, and carrying the two names at its root.
  std::fs::write(
    fx.root().join("pre-commit"),
    "#!/bin/sh\n. \"$(git rev-parse --git-path hooks)/pre-commit.intent\"\n",
  )
  .expect("write a root-level pre-commit");

  let facade = fx.facade_on_disk();
  let report = doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    Some(facade.store()),
    intentsvcs::doctor::Scope::All,
  );
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
  let fx = crate::common::Fixture::new();
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
  let report = doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    None,
    intentsvcs::doctor::Scope::All,
  );

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

// ---------------------------------------------------------------------------
// A project's OWN wiring (issue 0426). Advisories: the commit in this checkout
// still runs what it wired, and what they report is what a fresh clone loses.
// ---------------------------------------------------------------------------

/// The advisories `doctor` reports on this fixture, as `(file, detail)`.
fn advisories(fx: &crate::common::Fixture) -> Vec<(String, String)> {
  let facade = fx.facade_on_disk();
  doctor::diagnose(
    &fx.project(),
    &crate::common::ctx(),
    Some(facade.store()),
    intentsvcs::doctor::Scope::All,
  )
  .findings
  .into_iter()
  .filter(|f| f.class == FindingClass::Advisory)
  .map(|f| (f.file, f.detail))
  .collect()
}

/// Add a `guards` array to the fixture's config, keeping every other key.
fn declare_guards(fx: &crate::common::Fixture, guards: serde_json::Value) {
  let path = fx.root().join("intent/.config/config.json");
  let mut config: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&path).expect("read config")).expect("parse");
  config["guards"] = guards;
  std::fs::write(&path, config.to_string()).expect("declare guards");
}

/// The Lamplight chain: Intent's block, and a project guard wired by hand below it.
const HAND_WIRED: &str = "#!/usr/bin/env bash\nset -euo pipefail\n\n# intent-chain-block:start (generated by intent claude upgrade)\n\"$(git rev-parse --git-path hooks)/pre-commit.intent\" \"$@\" || exit $?\n# intent-chain-block:end\n\n# the project's own\nif ! bin/hooks/inbox-guard; then\n  exit 1\nfi\n";

#[test]
fn a_hand_wired_guard_in_an_untracked_chain_is_reported_and_its_structure_is_not() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  std::fs::write(fx.root().join(".git/hooks/pre-commit"), HAND_WIRED).expect("plant the chain");

  let found = advisories(&fx);
  let wiring: Vec<&String> = found
    .iter()
    .map(|(_, d)| d)
    .filter(|d| d.contains("no guard declares"))
    .collect();
  assert_eq!(wiring.len(), 1, "one advisory for the chain: {found:?}");
  assert!(
    wiring[0].contains("line 9 `if ! bin/hooks/inbox-guard; then`"),
    "it names the hand-wired line by number: {}",
    wiring[0]
  );
  for noise in ["set -euo", "`fi`", "exit 1", "pre-commit.intent"] {
    assert!(
      !wiring[0].contains(noise),
      "{noise:?} is shell structure or Intent's own block, not a project guard: {}",
      wiring[0]
    );
  }
}

#[test]
fn a_guard_both_declared_and_hand_wired_is_reported_as_doubled_and_not_as_undeclared() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  std::fs::write(fx.root().join(".git/hooks/pre-commit"), HAND_WIRED).expect("plant the chain");
  declare_guards(
    &fx,
    serde_json::json!([{ "run": ["bin/hooks/inbox-guard"] }]),
  );

  let found = advisories(&fx);
  assert!(
    found
      .iter()
      .any(|(_, d)| d.contains("runs twice") && d.contains("line 9")),
    "the doubled guard is reported: {found:?}"
  );
  assert!(
    !found.iter().any(|(_, d)| d.contains("no guard declares")),
    "and a declared guard is not also called undeclared: {found:?}"
  );
}

#[test]
fn a_tracked_hooks_directory_with_no_hooks_path_is_reported_and_setting_it_clears_it() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let dir = fx.root().join(".githooks");
  std::fs::create_dir_all(&dir).expect("mkdir .githooks");
  std::fs::write(
    dir.join("pre-commit"),
    "#!/usr/bin/env bash\nbin/hooks/inbox-guard\n",
  )
  .expect("tracked hook");
  let git = |args: &[&str]| {
    let ok = std::process::Command::new("git")
      .arg("-C")
      .arg(fx.root())
      .args(args)
      .status()
      .expect("git")
      .success();
    assert!(ok, "git {args:?}");
  };
  git(&["add", ".githooks/pre-commit"]);

  let found = advisories(&fx);
  assert!(
    found
      .iter()
      .any(|(f, d)| f == ".githooks/pre-commit" && d.contains("core.hooksPath")),
    "a fresh clone of this tree runs none of .githooks: {found:?}"
  );

  git(&["config", "core.hooksPath", ".githooks"]);
  let found = advisories(&fx);
  assert!(
    !found.iter().any(|(_, d)| d.contains("core.hooksPath")),
    "the control: with the path set, git runs it and there is nothing to say: {found:?}"
  );
}

// # Each state says what happens to a commit, and names the verb that repairs it
//
// Issue 0570, driven on the pair at 4c687eaad before these arms were written:
// with the carrier removed a commit is REFUSED ("GATE ABSENT") and `intent
// claude upgrade --apply` writes the carrier back; a carrier naming no runner
// lets every commit through and the same verb replaces it; and a shim whose
// pointer is absent refuses every commit until `intent bootstrap` records one.
// The class remedy said commits go through ungated and that no verb repairs
// any of it, and both were false for two states of three.

use intentsvcs::doctor::{GateAt, gate_not_running_detail};
use intentsvcs::install::PointerState;

const CARRIER: &str = ".git/hooks/pre-commit.intent";

fn at<'a>(shape: Option<CarrierShape>, pointer: &'a PointerState) -> GateAt<'a> {
  GateAt {
    carrier: CARRIER,
    shape,
    pointer,
    gate_body: true,
    own_install: None,
  }
}

fn resolves() -> PointerState {
  PointerState::Resolves {
    root: "/opt/intent".into(),
  }
}

fn detail(state: GateState, at: GateAt<'_>) -> String {
  gate_not_running_detail(&state, at).expect("a gate-not-running state has a detail")
}

#[test]
fn a_missing_carrier_says_the_commit_is_refused_and_names_upgrade() {
  let p = resolves();
  let d = detail(GateState::ChainCallsAMissingCarrier, at(None, &p));
  assert!(
    d.contains("REFUSED"),
    "the chain refuses the commit (GATE ABSENT): {d}"
  );
  assert!(
    d.contains("intent claude upgrade --apply"),
    "that verb writes the carrier back: {d}"
  );
  assert!(
    !d.contains("skipped"),
    "no guard is skipped: the commit does not happen: {d}"
  );
}

#[test]
fn a_carrier_that_runs_no_guards_lets_commits_through_and_names_upgrade() {
  let p = resolves();
  let d = detail(
    GateState::CarrierRunsNoGuards,
    at(Some(CarrierShape::Monolithic), &p),
  );
  assert!(
    d.contains("goes through"),
    "this is the one ungated state: {d}"
  );
  assert!(
    d.contains("intent claude upgrade --apply"),
    "it replaces the carrier with the shim: {d}"
  );
  assert!(
    !d.contains("intent bootstrap"),
    "the pointer resolves, so nothing more is owed: {d}"
  );
}

#[test]
fn replacing_a_runnerless_carrier_with_no_pointer_also_owes_bootstrap() {
  let p = PointerState::Absent;
  let d = detail(
    GateState::CarrierRunsNoGuards,
    at(Some(CarrierShape::Monolithic), &p),
  );
  assert!(d.contains("intent claude upgrade --apply"), "{d}");
  assert!(
    d.contains("intent bootstrap"),
    "the shim that verb installs refuses every commit until the pointer names an install: {d}"
  );
}

#[test]
fn a_shim_with_no_pointer_refuses_every_commit_until_bootstrap() {
  let p = PointerState::Absent;
  let d = detail(
    GateState::NoResolvableInstall,
    at(Some(CarrierShape::Shim), &p),
  );
  assert!(
    d.contains("REFUSED"),
    "the shim refuses rather than skipping: {d}"
  );
  assert!(d.contains("intent bootstrap"), "{d}");
}

#[test]
fn a_shim_whose_pointer_names_no_install_says_where_it_points() {
  let p = PointerState::Unusable {
    root: "/gone/intent".into(),
  };
  let d = detail(
    GateState::NoResolvableInstall,
    at(Some(CarrierShape::Shim), &p),
  );
  assert!(
    d.contains("/gone/intent"),
    "the stale line is the fault, so it is quoted: {d}"
  );
  assert!(
    d.contains("REFUSED") && d.contains("intent bootstrap"),
    "{d}"
  );
}

#[test]
fn a_pointer_naming_an_install_without_its_gate_owes_a_reinstall() {
  let p = resolves();
  let mut a = at(Some(CarrierShape::Shim), &p);
  a.gate_body = false;
  let d = detail(GateState::NoResolvableInstall, a);
  assert!(
    d.contains("REFUSED"),
    "the shim refuses an install with no gate body: {d}"
  );
  assert!(d.contains("/opt/intent") && d.contains("reinstall"), "{d}");
}

#[test]
fn a_monolithic_carrier_on_a_binary_outside_any_install_carries_that_error() {
  let p = resolves();
  let mut a = at(Some(CarrierShape::Monolithic), &p);
  a.own_install = Some("cannot locate the install -- reinstall Intent");
  let d = detail(GateState::NoResolvableInstall, a);
  assert!(
    d.contains("cannot locate the install -- reinstall Intent"),
    "the install error is carried, not dropped: {d}"
  );
  assert!(
    d.contains("goes through"),
    "its roster is empty, so nothing is enforced: {d}"
  );
}

#[test]
fn states_that_are_not_a_gate_not_running_have_no_detail() {
  let p = resolves();
  for state in [
    GateState::NotInstalled,
    GateState::Current,
    GateState::BehindTheTemplate {
      carrier: 1,
      template: 2,
    },
  ] {
    assert_eq!(
      gate_not_running_detail(&state, at(None, &p)),
      None,
      "{state:?}"
    );
  }
}

#[test]
fn the_class_remedy_no_longer_says_that_nothing_repairs_the_gate() {
  let r = FindingClass::GateNotRunning.remedy();
  assert!(
    !r.contains("NO VERB"),
    "two of the three states have a verb: {r}"
  );
  assert!(
    !r.contains("going through ungated"),
    "most states refuse the commit: {r}"
  );
}

#[test]
fn a_shim_with_no_pointer_read_by_a_binary_outside_any_install_carries_its_error() {
  let p = PointerState::Absent;
  let mut a = at(Some(CarrierShape::Shim), &p);
  a.own_install = Some("cannot locate the install -- reinstall Intent");
  let d = detail(GateState::NoResolvableInstall, a);
  assert!(
    d.contains("intent bootstrap"),
    "bootstrap from a complete install repairs it: {d}"
  );
  assert!(
    d.contains("cannot locate the install -- reinstall Intent"),
    "and bootstrap from THIS binary fails, so its own error and remedy are carried: {d}"
  );
}
