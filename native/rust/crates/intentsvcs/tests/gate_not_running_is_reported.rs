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
//! # Why these drive `gate_state` and not the whole of `doctor`
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
//!
//! Row three reds TWO, which the forecast had as one: a state that never
//! reports "behind" fails every input that should report it, same-length or
//! not. Row four is the pair's discriminator and reds alone -- **a length
//! comparison agrees with a byte comparison on every realistic carrier**,
//! because templates grow, and disagrees only on an edit that changes what the
//! guards do without changing how long the file is.

use intentsvcs::doctor::{GateState, gate_state};

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
