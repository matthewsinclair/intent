//! `0216`: **a canon write reports `ok`, lands in the store, and `intentd`'s
//! debounced disk ingest reverts it.**
//!
//! **THIS FILE DRIVES THE OPPOSITE DIRECTION TO `daemon_watch.rs`, AND THE
//! DISTINCTION IS THE WHOLE REASON IT IS A SEPARATE FILE.**
//! `a_burst_of_edits_is_debounced_into_far_fewer_ingests` drives DISK -> STORE
//! and asserts the debouncer coalesces without dropping. `0216` is STORE ->
//! REVERTED-BY-DISK-INGEST: the write reached the store, was logged there, and
//! a later `disk.sync_from_disk` by a different principal put the old value
//! back. Repointing the existing test at this would have left a test that still
//! passes, still reads as covering debouncing, and quietly no longer does.
//!
//! **THE STORE WRITE IS ALSO A DISK WRITE, WHICH IS WHAT ARMS THE INGEST.**
//! `facade.rs:3728` -- *views if marked, and canon regardless; the canon writes
//! above are unconditional* -- so every mutation lands rendered views and canon
//! JSON on disk. The watcher sees its own project's renders, debounces them,
//! and ingests them back. No deliberate disk touch is needed: the render is the
//! touch.
//!
//! **WHY THE WRITES ARE SPACED WIDER THAN THE DEBOUNCE, WHICH IS THE OPPOSITE
//! OF THE SHAPE THIS EXPERIMENT WAS FIRST PLANNED WITH.** The plan carried on
//! two boards was *8+ writes inside ONE quiet period* at ~25ms. Derived against
//! `0216`'s own event log, that is the configuration LEAST likely to lose
//! anything: `QUIET` is 250ms and resets on every event, so a burst tighter
//! than the debounce fires exactly one ingest, AFTER the last write, reading a
//! disk that already carries every render. Both recorded losses had the
//! opposite spacing -- 325ms and 473ms between the write and its predecessor,
//! each wider than `QUIET` -- so an ingest began BETWEEN two writes. The
//! vulnerable window is [ingest reads disk, ingest writes store], and a write
//! landing inside it is reverted.
//!
//! **REPRODUCED 2026-09-03, AND CONTENTION WAS THE VARIABLE -- NOT SPACING AND
//! NOT CORPUS SIZE.** A single-writer fixture loses nothing at any spacing or
//! size tried. Add writers competing for the same store and the reported
//! signature appears at once, at `baseline` 40:
//!
//! ```text
//! contenders=0  competing=0     ingests=10  REFUSED=0  SILENTLY_LOST=0
//! contenders=2  competing=490   ingests=85  REFUSED=5  SILENTLY_LOST=1  ["Burst row 7"]
//! contenders=4  competing=807   ingests=87  REFUSED=4  SILENTLY_LOST=1  ["Burst row 7"]
//! contenders=8  competing=1010  ingests=97  REFUSED=6  SILENTLY_LOST=0
//! ```
//!
//! **THE LOSS IS STOCHASTIC, SO IT NEEDS A DISTRIBUTION AND NOT AN ANECDOTE.**
//! Eight runs at `contenders=2`, `baseline=40`, depth 8:
//!
//! ```text
//! losses per run : 0 0 1 1 1 1 1 2
//! rows lost      : 2, 5, 6, 6, 7, 7, 7
//! ```
//!
//! **THE PREDICTION THIS FILE WAS BUILT TO TEST IS NOT REFUTED, AND THE FIRST
//! VERSION OF THIS COMMENT SAID IT WAS.** Pre-committed wording: *the
//! render-queue model predicts MORE THAN ONE lost row at depth 8; if only the
//! last goes, that model is wrong.* Two early runs each lost exactly `Burst row
//! 7`, the last row, and this header recorded the model as refuted on that
//! basis. **The very next run lost `Burst row 2` AND `Burst row 7`, and six more
//! put the loss at rows 5, 6 and 7.** So the disconfirming condition -- *only
//! the last goes* -- is FALSE, more than one row does go, and nothing is
//! refuted.
//!
//! **RECORDED AS A MISTAKE RATHER THAN QUIETLY FIXED, BECAUSE THE MISTAKE IS THE
//! REUSABLE PART: a conclusion drawn from n=2 about a stochastic process,
//! written into an artefact, and contradicted by the next sample.** The
//! discipline that produced the good result -- pre-committing the prediction --
//! is not the same discipline as knowing when you have enough samples to apply
//! it, and holding the first well says nothing about the second.
//!
//! **THE TWO DEFECTS TRADE OFF, WHICH IS WHY BOTH ARE COUNTED SEPARATELY.** As
//! contention climbs the loud refusal (`0226`) dominates and the silent loss
//! stops appearing -- at 8 contenders every collision is refused and none is
//! swallowed. **A single counter would have shown "fewer losses at higher
//! contention" and read as the defect improving under load.**
//!
//! **A NON-REPRODUCTION IS A RESULT ABOUT THE WINDOW AND NOT A CLEAN BILL OF
//! HEALTH.** `0216` logs the window at ~1s on the live corpus. The
//! window scales with how long a whole-corpus ingest takes, and a scratch
//! project with four threads has a window of approximately nothing. That is the
//! most likely reason the scratch attempt recorded in `0216` lost nothing
//! across 18 calls -- a possibility that filing left open, because it measured
//! the wrong variable and said so. `baseline()` exists to give the ingest real
//! work; if the loss still does not appear, the honest report is *not
//! reproduced at this corpus size*, naming the size.

use std::path::Path;
use std::time::Duration;

use crate::common::{RunningDaemon, project};
use intentsvcs::facade::{Facade, FacadeContext};
use intentsvcs::wire::{Op, Request, Response};

/// Writes in a burst. Eight because the pre-committed prediction is about depth
/// 8: a render-queue model loses MORE THAN ONE row here, and losing exactly the
/// last one refutes it.
const DEPTH: usize = 8;

/// Gap between writes. **Wider than `watch.rs`'s `QUIET` (250ms) on purpose** --
/// see the header. Tighter than the debounce is the safe case, not the sharp one.
const SPACING: Duration = Duration::from_millis(350);

/// Threads minted before the burst so an ingest has a corpus to walk. The
/// vulnerable window is the ingest's duration, and on an empty project there is
/// effectively none.
///
/// **SWEEPABLE, BECAUSE CORPUS SIZE IS THE OPEN VARIABLE RATHER THAN A SETTLED
/// ONE.** At 40 this reproduced nothing while the daemon demonstrably ingested
/// -- 10 ingests, 0 lost, measured 2026-09-03 -- which is consistent with the
/// window being ~0 on a small corpus and NOT with the defect being absent.
/// `INTENT_0216_BASELINE` moves it without editing this file, so a sweep is a
/// run rather than a patch.
fn baseline() -> usize {
  std::env::var("INTENT_0216_BASELINE")
    .ok()
    .and_then(|v| v.parse().ok())
    .unwrap_or(40)
}

/// Writers competing with the burst, each minting continuously against the same
/// store through its own facade.
///
/// **THE VARIABLE THE FIRST VERSION OF THIS FILE HELD FIXED WHILE THE LIVE TREE
/// DID NOT** (vc, 2026-09-03, having reproduced `0216` on the live tree twice
/// inside the hour this harness spent concluding not-reproduced). A scratch
/// project driven by one test has exactly ONE writer; the live tree had four
/// nodes committing into one canon all afternoon. **If the mechanism needs a
/// COMPETING ingest rather than merely a fast one, a single-writer fixture
/// cannot reach it at any spacing -- and the negative result would then be a
/// property of the fixture rather than of the debounce.** That is the difference
/// between *not reproduced here* and *does not reproduce*, and only the first
/// was ever established.
fn contenders() -> usize {
  std::env::var("INTENT_0216_CONTENDERS")
    .ok()
    .and_then(|v| v.parse().ok())
    .unwrap_or(2)
}

/// Long enough that the debounce has fired and any ingest it started has
/// finished: `QUIET` is 250ms and `0216` logs ingests completing ~1.05s after
/// the write they reverted.
const SETTLE: Duration = Duration::from_secs(4);

/// A facade on this root.
///
/// **A FRESH ONE PER CALL.** A facade holds a canon snapshot, so reading back
/// through the one that performed a write reports what that process believes
/// rather than what the store holds -- and what the store holds after another
/// actor wrote it is the entire question here.
fn open(root: &Path) -> Facade {
  let opened = intentsvcs::project::Project::open(root).expect("the project exists");
  let ctx = FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  Facade::open(opened, ctx).expect("open the facade")
}

/// Mint one thread, the way a CLI verb does: in-process, against the facade.
fn mint(root: &Path, title: &str) -> String {
  open(root).st_new(title).expect("the thread is mintable")
}

/// Every thread title the STORE holds.
fn stored_titles(root: &Path) -> Vec<String> {
  open(root)
    .st_list()
    .iter()
    .map(|t| t.title.clone())
    .collect()
}

/// How many ingests the daemon has run for this root, or `None` when it holds
/// no registration for it at all.
///
/// **ONE PROBE ANSWERING BOTH QUESTIONS, BECAUSE THEY ARE ASKED TOGETHER AND
/// THE SECOND IS WORTHLESS WITHOUT THE FIRST.** Registered says the daemon is
/// watching; the COUNT says a watch actually fired. A burst that loses nothing
/// against a registered project is uninterpretable on its own -- it reads
/// identically whether the ingest ran and behaved, or never ran at all -- and
/// `0216` names reading this counter either side of the burst as the thing that
/// would settle the question.
///
/// **THE PROBE DOES NOT DISTURB ITS OWN SUBJECT, WHICH IS WHY IT CAN BE READ IN
/// A LOOP AND WHY THE CONTROL ARM MEANS ANYTHING.** Registration is a side
/// effect of being used (`registry.rs:131`: *a project the daemon is answering
/// for is a project the daemon is watching*), so an instrument that bound to the
/// root would enrol the very project whose unregistered state it exists to
/// establish. `Op::Registry` is answered from a snapshot WITHOUT binding
/// (`main.rs:664`) and is in `wire::UNCOUNTED`, so asking neither registers nor
/// counts.
fn ingests(daemon: &RunningDaemon, root: &Path) -> Option<u64> {
  let response = daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::Registry,
  });
  let Response::Registry { projects } = response else {
    panic!("intentd answered Op::Registry with something else: {response:?}");
  };
  let wanted = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
  projects
    .iter()
    .find(|p| p.root.canonicalize().unwrap_or_else(|_| p.root.clone()) == wanted)
    .map(|p| p.ingested)
}

/// Mint `baseline()` threads so the ingest has something to walk.
fn lay_a_corpus(root: &Path) {
  for n in 0..baseline() {
    mint(root, &format!("Baseline {n:03}"));
  }
}

/// `DEPTH` writes at `SPACING`, returning the titles that reported success.
fn burst(root: &Path) -> (Vec<String>, usize) {
  let mut reported_ok = Vec::new();
  let mut refused = 0;
  for n in 0..DEPTH {
    let title = format!("Burst row {n}");
    // **A REFUSAL AND A SILENT LOSS ARE DIFFERENT DEFECTS AND MUST NOT BE
    // COUNTED TOGETHER.** A write that returns `Err` is `0226`: loud, visible,
    // and the operator knows. A write that returns `Ok` and is absent afterwards
    // is `0216`: the whole point is that nothing told anyone. Panicking on the
    // first refusal, which this did until contention was added, throws away the
    // run AND conflates the two.
    if open(root).st_new(&title).is_ok() {
      reported_ok.push(title);
    } else {
      refused += 1;
    }
    std::thread::sleep(SPACING);
  }
  (reported_ok, refused)
}

/// The titles that reported `ok` and are not in the store.
fn lost(root: &Path, written: &[String]) -> Vec<String> {
  let held = stored_titles(root);
  written
    .iter()
    .filter(|title| !held.contains(title))
    .cloned()
    .collect()
}

#[test]
fn an_unwatched_project_loses_nothing_in_the_same_burst() {
  // **THE CONTROL, AND IT CARRIES TWO CLAIMS.** First, the burst harness itself
  // does not drop writes -- without that, a loss in the subject arm is a
  // property of this file rather than of the daemon. Second, the discriminator
  // really is the WATCH: same daemon, same verbs, same spacing, same corpus,
  // and the only difference is that nothing ever bound this root.
  let daemon = RunningDaemon::start();
  let root = project("Unwatched");

  lay_a_corpus(&root);
  let (written, refused) = burst(&root);
  std::thread::sleep(SETTLE);

  // Measured, never assumed. This is the exact variable `0216`'s scratch-project
  // attempt left unestablished, which is why its clean result proved nothing.
  assert!(
    ingests(&daemon, &root).is_none(),
    "the control project is REGISTERED, so it is watched and it is not a control at all -- something in this test bound the root"
  );

  let lost = lost(&root, &written);
  assert_eq!(
    refused, 0,
    "an UNWATCHED project REFUSED {refused} writes outright, which is 0226 and not this arm's subject"
  );
  assert!(
    lost.is_empty(),
    "an UNWATCHED project lost {} of {DEPTH} writes ({lost:?}), so the burst harness drops writes on its own and the subject arm below cannot be read as a daemon defect",
    lost.len()
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
#[ignore = "0216 is an OPEN defect and this REPRODUCES it, so it is red by design until the ingest stops reverting store state newer than the disk it read. Stochastic: the loss appears in a contention band (1 row at 2-4 contenders, none at 0 or 8), so a single green run is not evidence of a fix. Run with --include-ignored."]
fn an_ingest_never_reverts_a_newer_store_write() {
  // **THE SUBJECT.** Identical to the control in every respect except that the
  // daemon is made to serve -- and therefore watch -- this root.
  let daemon = RunningDaemon::start();
  let root = project("Watched");

  // Registration is a side effect of being used, so asking anything about the
  // root enrols it. There is no `register` verb by design (`registry.rs:131`).
  let _ = daemon.ask(Request {
    root: root.to_path_buf(),
    op: Op::ThreadList,
  });
  let before = ingests(&daemon, &root)
    .expect("the subject project is NOT registered, so nothing is watching it and this arm would pass for the wrong reason");

  lay_a_corpus(&root);
  // **CONTENTION, RAISED BEFORE THE BURST AND DROPPED AFTER IT.** Each contender
  // writes through its own facade, which is the door the CLI uses, so this is a
  // second writer rather than a second opinion about writing.
  let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
  let mut competing = Vec::new();
  for c in 0..contenders() {
    let root = root.clone();
    let stop = std::sync::Arc::clone(&stop);
    competing.push(std::thread::spawn(move || {
      let mut n = 0u32;
      while !stop.load(std::sync::atomic::Ordering::Relaxed) {
        let _ = open(&root).st_new(&format!("Contender {c}/{n}"));
        n += 1;
      }
      n
    }));
  }

  let (written, refused) = burst(&root);
  std::thread::sleep(SETTLE);
  stop.store(true, std::sync::atomic::Ordering::Relaxed);
  let competing: u32 = competing.into_iter().map(|h| h.join().unwrap_or(0)).sum();

  let after =
    ingests(&daemon, &root).expect("the subject project stopped being registered mid-test");
  let lost = lost(&root, &written);

  // **WITHOUT THIS THE ARM IS VACUOUS AND READS EXACTLY LIKE A PASS.** A burst
  // that loses nothing because the ingest behaved, and one that loses nothing
  // because no ingest ever ran, are the same observation until the counter is
  // read. `0216` names this read as the thing that would settle the question,
  // and the first version of this file asserted registration and skipped it.
  assert!(
    after > before,
    "the daemon ran NO ingest across the whole burst ({before} -> {after}), so this arm never exercised the mechanism and a clean result here says nothing about 0216"
  );

  // **THE PROFILE IS PRINTED WHETHER OR NOT THE ASSERT FIRES**, because the
  // pre-committed question is not pass/fail -- it is HOW MANY and WHICH. A
  // render-queue model predicts more than one loss at depth 8; exactly one loss,
  // and that the last, refutes it.
  println!(
    "0216 profile: depth={DEPTH} spacing={SPACING:?} baseline={} contenders={} competing_writes={competing} ingests={} REFUSED(0226)={refused} SILENTLY_LOST(0216)={} {lost:?}",
    baseline(),
    contenders(),
    after - before,
    lost.len()
  );

  assert!(
    lost.is_empty(),
    "{} of {DEPTH} writes reported ok and are absent from the store: {lost:?}. Each one's success line was honest -- the write landed and a later disk ingest reverted it",
    lost.len()
  );

  let _ = std::fs::remove_dir_all(&root);
}
