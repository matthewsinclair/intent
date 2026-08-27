//! ST0057 **AT-11.1, AT-11.4, AT-11.5** -- covering **AC-11.1, AC-11.4,
//! AC-11.5**, and standing beside AT-11.2 which covers AC-11.2 from
//! `exit_codes.rs`. `intent organize --default` writes `.intentfiles` from
//! thread status, and **never removes a file**.
//!
//! **DRIVEN THROUGH THE BINARY, NOT THE FACADE, BECAUSE THE CRITERIA ARE ABOUT
//! A COMMAND.** Two of the four -- the tty refusal and the alias -- exist only
//! at the surface: a facade-level test cannot tell `organize` from `organise`
//! and has no terminal to be absent from.
//!
//! # Mutations, measured -- every assertion below has been SEEN to fail by name
//!
//! | mutation                                                | reds                                                     |
//! | ------------------------------------------------------- | -------------------------------------------------------- |
//! | `--default` also calls `organize(Mode::Apply)`           | `default_creates_no_file_...` ONLY                       |
//! | the summary drops its ` (N blocked)` parenthetical       | `default_removes_no_file_...` ONLY                       |
//! | `default_declaration` declares everything not Completed  | the absent-manifest, creation and removal tests          |
//!
//! **THE FIRST ROW IS THE ONE WORTH READING.** An applying `--default` is the
//! exact defect AC-11.4 exists to prevent, and the REMOVAL test does not catch
//! it -- the dehydration gate refuses the removal on a project that declares no
//! preconditions, so nothing is removed and the survival assertion passes. Only
//! the CREATION arm catches it, because hydration is not gated. Before that arm
//! existed this mutation left every test in the file green.
//!
//! **AND THE HARNESS THAT MEASURED THIS HAD TO BE FIXED FIRST.** A restore that
//! puts the bytes back with a metadata-preserving copy leaves cargo's staleness
//! check unmoved, so the next run links the MUTATED artefact while `git diff`
//! reports the tree clean. The first attempt at this table was contaminated
//! exactly that way and read as three tests redding under every mutation. The
//! restore has to bump the mtime, and the baseline has to be re-run to green
//! after each revert.
//!
//! # The one arm that is NOT driven here, named rather than left to be noticed
//!
//! **AC-11.2's `--force` answered `y` ON A TTY is not covered.** It needs a
//! pseudo-terminal, which this estate has no harness for. What IS covered is
//! the half that can fail silently: `--force` WITHOUT a tty writing nothing and
//! exiting non-zero. **The uncovered half fails loudly** -- an operator typing
//! it sees the prompt or does not -- while the covered half is the one that
//! would let a script regenerate a declaration with nobody watching.

use std::process::{Command, Output};

fn intent(dir: &std::path::Path, args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    // **stdin CLOSED, DELIBERATELY: that is what makes these runs non-tty.**
    // Inheriting the harness's stdin would make the result depend on whether
    // cargo was run from a terminal -- so the same assertion would pass locally
    // and mean something different in CI.
    .stdin(std::process::Stdio::null())
    .output()
    .expect("run the v3 binary")
}

/// Two threads, **both at `triage`**, and neither of them realised.
///
/// **THE NAME OF THE SECOND THREAD IS NOT ITS STATUS.** This helper used to say
/// *one open thread and one closed one*; it creates `A thread to close` and
/// never closes it, so both sit at `triage` and each test moves what it needs.
/// A fixture doc that describes the intent of the titles rather than the state
/// they produce is the same defect as a stale comment, arriving one step
/// earlier -- and here it mattered, because `triage` is the population that
/// tells the current declaration rule from the one it replaced.
///
/// **AND NEITHER THREAD HAS A VIEW FILE ON DISK.** `st new` writes canon
/// (`intent/.canon/st/ST0001.json`) and does not realise; only `st hydrate` or
/// `organize --apply` puts `intent/st/ST0001/*.md` there. Any test that needs a
/// REALISED project has to say so.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  assert!(
    intent(root, &["init", "Fixture"]).status.success(),
    "the fixture must initialise"
  );
  for title in ["An open thread", "A thread to close"] {
    assert!(
      intent(root, &["st", "new", title]).status.success(),
      "st new must succeed"
    );
  }
  dir
}

fn manifest(root: &std::path::Path) -> String {
  std::fs::read_to_string(root.join("intent/.intentfiles")).expect("manifest")
}

fn declared(text: &str) -> Vec<String> {
  text
    .lines()
    .filter(|l| l.starts_with("STEELTHREAD:"))
    .map(|l| l.trim().to_string())
    .collect()
}

/// The one-line verdict `organize` prints, which is where the counts live.
///
/// Taken by prefix rather than by line number: the rows above it vary with the
/// estate, so an index would be a fixture detail the assertion did not mean to
/// depend on.
fn summary(out: &Output) -> String {
  let said = String::from_utf8_lossy(&out.stdout).into_owned();
  said
    .lines()
    .find(|l| l.starts_with("organize (preview):"))
    .unwrap_or_else(|| panic!("no preview summary line in:\n{said}"))
    .to_string()
}

/// Every file under the project, with its bytes -- the only way to assert
/// **AC-11.4** without trusting a report about what was written.
fn tree(root: &std::path::Path) -> std::collections::BTreeMap<String, Vec<u8>> {
  fn walk(
    dir: &std::path::Path,
    root: &std::path::Path,
    out: &mut std::collections::BTreeMap<String, Vec<u8>>,
  ) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for e in entries.flatten() {
      let p = e.path();
      if p.is_dir() {
        walk(&p, root, out);
      } else if let Ok(bytes) = std::fs::read(&p) {
        out.insert(
          p.strip_prefix(root).unwrap_or(&p).display().to_string(),
          bytes,
        );
      }
    }
  }
  let mut out = std::collections::BTreeMap::new();
  walk(root, root, &mut out);
  out
}

// ---------------------------------------------------------------------------
// AC-11.1 -- ABSENT: WRITE IT FROM STATUS
// ---------------------------------------------------------------------------

#[test]
fn an_absent_manifest_is_written_from_status_and_declares_only_the_open() {
  let dir = project();
  let root = dir.path();
  std::fs::remove_file(root.join("intent/.intentfiles")).expect("start from absent");

  // **THREE POPULATIONS, NOT TWO, BECAUSE THE PREDICATE CHANGED UNDER THIS
  // TEST.** `--default` declares WIP and nothing else (hv, 2026-08-26), so the
  // fixture needs a thread that is neither WIP nor closed: `ST0002` stays in
  // `triage`. A fixture of one open and one closed thread CANNOT TELL THE TWO
  // RULES APART -- the old `!is_closed()` predicate declared Triage too, and
  // would pass an assertion written against it.
  assert!(
    intent(root, &["st", "start", "ST0001"]).status.success(),
    "moving the first thread to WIP must succeed"
  );

  let out = intent(root, &["organize", "--default"]);
  assert!(out.status.success(), "exit 0: {out:?}");

  assert_eq!(
    declared(&manifest(root)),
    vec!["STEELTHREAD:ST0001"],
    "the WIP thread and nothing else -- ST0002 is `triage`, which the old \
     `!is_closed()` rule declared and this one does not"
  );

  // **AC-11.1's SECOND CLAUSE, AND IT IS THE HALF THAT SAYS THE WRITE WAS
  // COHERENT.** The lines above prove the file says what it should. They cannot
  // tell you the estate AGREES with it -- a declaration naming an id `organize`
  // then reports as `diverged` or `refused` is a well-formed file describing a
  // state the tool cannot reach, and every assertion above passes over it.
  let preview = intent(root, &["organize"]);
  let said = summary(&preview);
  assert!(
    said.contains("0 diverged"),
    "a declaration can be well formed and still name a state the tool cannot reach; \
     `diverged` is where that shows: {said}"
  );
  // **THE CRITERION ALLOWS EITHER, SO THE ASSERTION IS ON THE PAIR.** Both mean
  // the preview accepts the declaration; which of the two a given estate lands
  // on is a fact about whether the files happen to be there, not about the
  // declaration.
  assert!(
    said.contains("2 unchanged") || said.contains("2 to hydrate"),
    "ST0001's two files must be accounted for as unchanged or to-hydrate: {said}"
  );

  // **AND `refused` IS DELIBERATELY NOT ASSERTED HERE, WHICH NEEDS SAYING
  // BECAUSE THIS FIXTURE PRODUCES ONE.** The refusal is the dehydration gate
  // declining to remove the UNDECLARED thread's files -- AC-11.4's subject, and
  // driven there. Folding it into this test would make an AC-11.1 red mean
  // either of two unrelated things.
  //
  // **THE FIXTURE IS FULLY REALISED, AND IT IS REALISED BY AN ORDERING THAT
  // READS LIKE IT COULD NOT MATTER.** The manifest is removed and `st start`
  // runs after it; with the manifest ABSENT nobody has said what is realised, so
  // everything is, and the next write realises the whole estate -- both threads,
  // not just the one being started. Measured, because `st start` BEFORE the
  // removal leaves the same manifest and NOTHING on disk. Two orderings, same
  // file, opposite estates.
}

// ---------------------------------------------------------------------------
// AC-11.2 -- PRESENT: CHANGE NOT ONE BYTE
// ---------------------------------------------------------------------------

#[test]
fn a_present_manifest_is_left_byte_identical_and_force_is_named() {
  let dir = project();
  let root = dir.path();
  let before = manifest(root);

  let out = intent(root, &["organize", "--default"]);
  assert!(out.status.success(), "exit 0 on a present file: {out:?}");
  assert_eq!(manifest(root), before, "not one byte changed");

  let said = String::from_utf8_lossy(&out.stdout).into_owned();
  assert!(
    said.contains("--force"),
    "the report must name the spelling that WOULD regenerate it, or the operator \
     is told no and not told how: {said}"
  );
  assert!(
    said.contains("declares"),
    "and must say how many entries the present file declares: {said}"
  );
}

#[test]
fn force_without_a_tty_writes_nothing_and_exits_non_zero() {
  let dir = project();
  let root = dir.path();
  let before = manifest(root);

  let out = intent(root, &["organize", "--default", "--force"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "no tty means no confirmation, and no confirmation means no write: {out:?}"
  );

  // **THE REASON, NOT JUST THE OUTCOME -- and this assertion is the whole test.**
  // vc mutated the tty guard to `if false` and this test STAYED GREEN: with the
  // guard gone, the confirmation read hits EOF on a null stdin, the answer is
  // not `y`, and the run refuses anyway. **Two different refusals sharing one
  // exit code, and the version of this test that read only the code could not
  // tell them apart** -- IN-AG-RED-CONTROL-001 in the file written to prove a
  // criterion. Only the tty guard can produce this wording.
  let said = String::from_utf8_lossy(&out.stderr).into_owned();
  assert!(
    said.contains("terminal"),
    "the refusal must name the TERMINAL as what is missing, or it is \
     indistinguishable from the empty-answer refusal one line further on: {said}"
  );
  assert!(
    said.contains("no flag or environment variable that answers for you"),
    "and must say that the absence of a human IS the refusal, which is the \
     property AC-11.2 measures: {said}"
  );
  assert_eq!(
    manifest(root),
    before,
    "the refusal must have written nothing"
  );

  // **THE CONTROL, AND IT IS WHAT STOPS THIS PASSING FOR THE WRONG REASON.**
  // A build where `--default` was broken outright would also refuse here. The
  // same fixture under bare `--default` must still succeed.
  assert!(
    intent(root, &["organize", "--default"]).status.success(),
    "bare --default on the same fixture must still work, or the refusal above \
     proves only that the verb is broken"
  );
}

#[test]
fn force_on_its_own_is_a_usage_error_rather_than_a_silent_no_op() {
  let dir = project();
  let out = intent(dir.path(), &["organize", "--force"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "`--force` without `--default` modifies nothing and must not read as a \
     forced reconciliation: {out:?}"
  );
}

// ---------------------------------------------------------------------------
// AC-11.4 -- NEVER REMOVES A FILE
// ---------------------------------------------------------------------------

/// **AC-11.4 -- `--default` NEVER REMOVES A FILE, AND THE FOLLOW-UP PREVIEW SAYS
/// SO IN THE ONE PLACE AN OPERATOR READS.**
///
/// Two clauses, and they fail differently. The survival clause is about the
/// disk and is checked byte for byte; the report clause is about what the
/// operator is told, and a run that removed nothing while printing `0 to
/// remove` with no parenthetical would satisfy the first and betray the second.
///
/// **THE FIXTURE IS FULLY REALISED, WHICH IT USED TO ONLY LOOK LIKE.** The
/// earlier version guarded itself with *the UNDECLARED thread must have files
/// on disk, or this test cannot fail* -- and that guard passed on
/// `intent/.canon/st/ST0002.json`, a canon file this verb would never remove
/// under any implementation. **A positive control satisfied by something the
/// mutation could not touch is decoration**, so the guard now names the
/// realised view path, which is the population actually at risk.
///
/// Realisation comes from an ordering that reads like it could not matter: the
/// manifest is removed FIRST, and with it absent nobody has said what is
/// realised, so everything is -- and the next write puts the whole estate on
/// disk. `st start` before the removal leaves the same manifest and nothing on
/// disk at all.
#[test]
fn default_removes_no_file_belonging_to_an_undeclared_thread() {
  let dir = project();
  let root = dir.path();

  // **THE MANIFEST MUST BE ABSENT OR THIS VERB DOES NOTHING AT ALL.** `init`
  // writes `.intentfiles` and `st new` adds each id to it, so by this point the
  // file is PRESENT -- and a present manifest is the no-op arm. An earlier
  // version asserted that no file was removed by a run that never executed
  // anything downstream of the write, which is why vc's mutation
  // (`declare_default` calling `organize(Mode::Apply)`) could not redden it.
  // **A survival assertion needs a run that could have killed something.**
  std::fs::remove_file(root.join("intent/.intentfiles")).expect("make --default act");
  assert!(
    intent(root, &["st", "start", "ST0001"]).status.success(),
    "one thread WIP, so the declaration is non-empty and ST0002 is left out of it"
  );

  let before = tree(root);
  assert!(
    before.keys().any(|k| k.starts_with("intent/st/ST0002/")),
    "the UNDECLARED thread must have REALISED files -- not merely canon -- or the \
     survival assertion below is about a population `--default` could never have \
     touched:\n{:?}",
    before.keys().collect::<Vec<_>>()
  );

  assert!(
    intent(root, &["organize", "--default"]).status.success(),
    "exit 0"
  );

  let after = tree(root);
  for (path, bytes) in &before {
    // The manifest is the one file this verb writes, and the store is not part
    // of the estate's authored surface.
    if path.ends_with(".intentfiles") || path.contains(".cache") {
      continue;
    }
    assert_eq!(
      after.get(path),
      Some(bytes),
      "{path} changed or vanished -- `--default` never removes a file"
    );
  }

  // **THE REPORT CLAUSE.** The undeclared thread's files are what a later
  // `organize --apply` would remove, and the preview has to say so while saying
  // it did not. `0 to remove` alone would be true and misleading -- the
  // parenthetical is the whole point, and it is the shape `render.rs` records
  // as having been measured wrong on a real estate.
  let said = summary(&intent(root, &["organize"]));
  assert!(
    said.contains("0 to remove (2 blocked)"),
    "the preview must report the undeclared thread's two files as blocked removals \
     rather than as nothing at all: {said}"
  );
  assert!(
    said.contains("0 diverged"),
    "nothing diverged -- the files are exactly what canon holds: {said}"
  );

  // **WHAT THIS TEST DOES NOT ESTABLISH, RECORDED HERE BECAUSE IT IS NOT
  // RECOVERABLE FROM THE ASSERTIONS.** The blocked count is a COUNT OF FILES,
  // and no line of the report names ST0002. An operator reading this output
  // learns that two files are blocked, not which thread they belong to. The
  // criterion says *reports the non-WIP threads as to remove and blocked*, and
  // a count satisfies the second half of that only by arithmetic that happens
  // to be unambiguous when there is exactly one undeclared thread.
  //
  // **AND THE SURVIVAL CLAUSE ABOVE IS HELD UP BY THE DEHYDRATION GATE, NOT BY
  // THIS VERB DECLINING TO ACT. MEASURED, NOT SUSPECTED:** making
  // `--default` call `facade.organize(Mode::Apply)` -- an applying `--default`,
  // which is the exact defect the clause exists to catch -- leaves THIS TEST
  // green, because the gate refuses the removal on a project that declares no
  // dehydration preconditions. The removal arm cannot distinguish *never
  // removes* from *tried and was stopped*.
  //
  // **THE ARM THAT CAN IS CREATION, AND IT IS NEXT DOOR.** Hydration is not
  // gated, so an applying `--default` betrays itself by writing files rather
  // than by removing them:
  // [`default_creates_no_file_for_a_declared_thread_it_has_not_realised`].
}

/// **THE ARM THAT ACTUALLY CATCHES AN APPLYING `--default`.**
///
/// AC-11.4's own words are about removal, and removal is gated -- so on a
/// project with no declared dehydration preconditions the gate refuses, nothing
/// is removed, and a `--default` that had called `organize --apply` passes the
/// survival assertion next door untouched. **Measured as a mutation, not
/// reasoned: before this test existed, that mutation left every test in the
/// file green. With it, that mutation reds this one and nothing else.**
///
/// Hydration is not gated. So the fixture is inverted -- a thread that IS
/// declared and is NOT on disk -- and the assertion is that `--default` writes
/// no file it did not have to. The verb's own report claims exactly this in the
/// sentence *no file was created or removed*, and until now nothing drove the
/// first half of it.
///
/// **THE ORDERING IS THE FIXTURE AND IT IS EASY TO GET BACKWARDS.** `st start`
/// runs while the manifest is still PRESENT, so it edits the list and realises
/// nothing; the manifest is removed AFTER. Reverse those two and the write
/// lands with the manifest absent, which means nobody has said what is realised,
/// so everything is -- and the estate arrives fully on disk with nothing left
/// for this test to catch.
#[test]
fn default_creates_no_file_for_a_declared_thread_it_has_not_realised() {
  let dir = project();
  let root = dir.path();
  assert!(
    intent(root, &["st", "start", "ST0001"]).status.success(),
    "WIP while the manifest is present, so the id is declared and nothing is realised"
  );
  std::fs::remove_file(root.join("intent/.intentfiles")).expect("make --default act");

  let before = tree(root);
  assert!(
    !before.keys().any(|k| k.starts_with("intent/st/ST0001/")),
    "the DECLARED thread must be absent from disk, or there is nothing for an applying \
     `--default` to create and this test cannot fail:\n{:?}",
    before.keys().collect::<Vec<_>>()
  );

  assert!(
    intent(root, &["organize", "--default"]).status.success(),
    "exit 0"
  );

  let after = tree(root);
  let created: Vec<_> = after
    .keys()
    .filter(|k| !before.contains_key(*k))
    .filter(|k| !k.ends_with(".intentfiles") && !k.contains(".cache"))
    .collect();
  assert!(
    created.is_empty(),
    "`--default` writes the declaration and nothing else -- its own report says `no file \
     was created or removed`, and these appeared: {created:?}"
  );

  // And the follow-up preview agrees the thread is declared-but-absent, which is
  // the state that made the assertion above meaningful.
  let said = summary(&intent(root, &["organize"]));
  assert!(
    said.contains("2 to hydrate"),
    "the declared thread's two files are still waiting to be realised: {said}"
  );
}

// ---------------------------------------------------------------------------
// AC-11.5 -- ONE CODE PATH
// ---------------------------------------------------------------------------

#[test]
fn organise_and_organize_are_one_code_path() {
  let dir_z = project();
  let dir_s = project();
  std::fs::remove_file(dir_z.path().join("intent/.intentfiles")).expect("absent");
  std::fs::remove_file(dir_s.path().join("intent/.intentfiles")).expect("absent");

  let z = intent(dir_z.path(), &["organize", "--default"]);
  let s = intent(dir_s.path(), &["organise", "--default"]);

  assert_eq!(z.status.code(), s.status.code(), "same exit code");
  assert_eq!(
    String::from_utf8_lossy(&z.stdout),
    String::from_utf8_lossy(&s.stdout),
    "byte-identical output -- if the alias reached a second handler this is \
     where it would show"
  );
  assert_eq!(
    manifest(dir_z.path()),
    manifest(dir_s.path()),
    "byte-identical files"
  );
}
