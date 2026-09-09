//! `AT-15.3` (ST0056) / `AC-15.3`: **a RETIRE is carried out through the
//! payload API and the removal REACHES A CONSUMER -- asserted by driving a
//! consumer install, never by observing that the source directory is clean.**
//!
//! # THE ROW'S OWN SENTENCE IS THE TEST, AND ITS WARNING IS THE CONTROL
//!
//! `AC-15.3` says the removal must be asserted **"never by observing that the
//! source directory is clean"**. That is not a style preference -- it names a
//! check that PASSES WHILE THE DEFECT IS PRESENT, and this file makes that
//! executable rather than trusting anyone to avoid it. The first arm performs
//! the forbidden check on purpose and asserts it gives the WRONG answer: a
//! skill deleted by hand from the source leaves the consumer's copy in place,
//! so "the source is clean" is true and "the skill is gone" is false at the
//! same moment.
//!
//! **A check that cannot be shown to fail is not evidence.** The second arm
//! then does it properly, through `uninstall`, and asserts the CONSUMER is
//! what changed.
//!
//! # WHY A PLANTED FIXTURE RATHER THAN A REAL VERDICT, STATED NOT HIDDEN
//!
//! **The real population is EMPTY: the skills triage returned ZERO RETIRE
//! verdicts**, with its four grounds recorded in `intent/docs/skills-triage.md`.
//! A test over that population would pass without executing anything, which is
//! the vacuous satisfaction this thread keeps finding in its own instruments --
//! and `AC-15.3` is a requirement about a MECHANISM, so the mechanism is what
//! is driven. **If a RETIRE is ever ruled, this is the check it must pass; the
//! fixture is here because there is nothing real to point it at yet.**
//!
//! # WHAT THIS DOES NOT CLAIM
//!
//! It does not assert that any particular skill SHOULD be retired -- that is a
//! judgement and no test holds it. It does not exercise the CLI arm: the
//! behaviour lives in `intentsvcs::payload` and is driven there, which is the
//! same placement `skills_sync.rs` records and for the same reason.

use std::fs;
use std::path::{Path, PathBuf};

use intentsvcs::payload::{Kind, MANIFEST_RELATIVE, Payload};

const PLUGIN_REL: &str = "intent/plugins/claude/skills";

/// A disposable estate: an install tree (the SOURCE) and a Claude target (the
/// CONSUMER). The two being separate directories is the whole point of the row.
struct Estate {
  _tmp: tempfile::TempDir,
  install: PathBuf,
  target: PathBuf,
  manifest: PathBuf,
}

impl Estate {
  fn new() -> Self {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path().to_path_buf();
    let install = root.join("install");
    crate::common::fake_install(&install, PLUGIN_REL);
    Self {
      install,
      target: root.join("home/.claude/skills"),
      manifest: root.join("home/.intent").join(MANIFEST_RELATIVE),
      _tmp: tmp,
    }
  }

  fn payload(&self) -> Payload {
    Payload::new(
      Kind::Skills,
      &self.install,
      None,
      self.target.clone(),
      self.manifest.clone(),
    )
  }

  fn source_dir(&self, name: &str) -> PathBuf {
    self.install.join(PLUGIN_REL).join(name)
  }

  fn consumer_dir(&self, name: &str) -> PathBuf {
    self.target.join(name)
  }

  /// Plant a skill in the SOURCE. A directory is a skill iff it carries a
  /// `SKILL.md`, which is the same marker `payload.rs` uses.
  fn plant(&self, name: &str) {
    let d = self.source_dir(name);
    fs::create_dir_all(d.join("scripts")).expect("skill dir");
    fs::write(
      d.join("SKILL.md"),
      format!("# {name}\n\nA planted fixture.\n"),
    )
    .expect("SKILL.md");
    fs::write(d.join("scripts/run.sh"), "#!/bin/bash\necho planted\n").expect("script");
  }
}

fn present(p: &Path) -> bool {
  p.join("SKILL.md").is_file()
}

#[test]
fn invariant_a_retire_is_asserted_at_the_consumer_and_not_at_the_source() {
  let e = Estate::new();
  let name = "in-fixture-retire-me".to_string();
  e.plant(&name);

  // ---- INSTALL: the consumer must actually receive it, or nothing below means
  // anything. This is the positive control on the whole estate: a removal
  // "reaching" a consumer that never received the skill is not a measurement.
  let report = e.payload().install(std::slice::from_ref(&name), false);
  assert!(
    report.is_ok(),
    "install failed, so the estate is not wired: {report:?}"
  );
  assert!(
    present(&e.consumer_dir(&name)),
    "the CONSUMER did not receive the planted skill, so every assertion below \
     would pass against an absence that was never a presence. Estate broken; \
     fix it before reading any verdict here."
  );
  assert!(
    present(&e.source_dir(&name)),
    "the SOURCE lost the skill during install -- install must not be a move."
  );

  // ---- ARM 1: THE FORBIDDEN CHECK, PERFORMED ON PURPOSE.
  // `AC-15.3` forbids asserting a removal "by observing that the source
  // directory is clean". Here is why: delete it from the SOURCE by hand, the
  // way a hand-carried RETIRE would, and the source IS clean -- while the
  // consumer still has it. The forbidden check returns the wrong answer, and
  // this arm fails if it ever stops doing so.
  fs::remove_dir_all(e.source_dir(&name)).expect("hand-delete from source");
  assert!(
    !present(&e.source_dir(&name)),
    "the hand-delete did not take, so arm 1 is not exercising its subject."
  );
  assert!(
    present(&e.consumer_dir(&name)),
    "THE FORBIDDEN CHECK NO LONGER FAILS, which means this arm has stopped \
     controlling anything. `AC-15.3` forbids asserting a removal by observing \
     a clean source PRECISELY BECAUSE a hand-delete leaves the consumer's copy \
     in place. If the consumer's copy now disappears when the source is deleted \
     by hand, the mechanism changed and this row's wording needs re-reading."
  );

  // ---- ARM 2: THE REQUIRED CHECK, THROUGH THE API, ASSERTED AT THE CONSUMER.
  // Restore the source so `uninstall` acts on a coherent estate, then drive the
  // real verb and assert the thing the row actually asks for.
  e.plant(&name);
  let report = e.payload().install(std::slice::from_ref(&name), true);
  assert!(report.is_ok(), "re-install failed: {report:?}");
  assert!(
    present(&e.consumer_dir(&name)),
    "re-install did not reach the consumer"
  );

  let report = e.payload().uninstall(std::slice::from_ref(&name));
  assert!(report.is_ok(), "uninstall failed: {report:?}");
  assert!(
    !present(&e.consumer_dir(&name)),
    "THE REMOVAL DID NOT REACH THE CONSUMER. `uninstall` reported success and \
     the consumer still carries the skill -- which is the exact defect \
     `AC-15.3` exists to catch, and the reason it refuses source-side evidence."
  );

  // The source is untouched by uninstall, which is what makes the forbidden
  // check forbidden: the two directories answer different questions.
  assert!(
    present(&e.source_dir(&name)),
    "uninstall removed the SOURCE copy. That would make the forbidden check \
     accidentally correct, and it is not what the verb is specified to do."
  );

  eprintln!(
    "AT-15.3 reach: mechanism driven on a PLANTED fixture, because the real \
     RETIRE population is EMPTY -- the triage returned zero RETIRE verdicts. \
     Arm 1 drives the forbidden source-side check and requires it to give the \
     WRONG answer; arm 2 drives `uninstall` and asserts at the CONSUMER. NOT \
     covered: whether any real skill should be retired, and the CLI arm, which \
     lives with its subject in `intentsvcs::payload`."
  );
}
