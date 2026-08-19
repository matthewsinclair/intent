//! **Issue 0044: a command this build RETIRED is refused by name, and does not
//! answer in the same code as a command that ran and said no.**
//!
//! 0044's structural finding is that the exit code was decided by WHERE the
//! failure happened in the parse tree rather than by WHAT went wrong.
//! Retirement removes a name from the clap surface, so a retired command never
//! reached dispatch at all -- it got clap's generic `1`, the same number as a
//! genuine runtime refusal and as a critic run that found real problems. **The
//! careful work that gave unimplemented commands a deliberate code was
//! structurally unreachable for exactly the class a migration hits most.**
//!
//! **The roster is READ FROM THE TABLE, never listed here.** A hardcoded list
//! would cover the five retirements that exist today and silently stop covering
//! the sixth -- the same shape as the comment beside `EXIT_UNAVAILABLE` that
//! named the one consumer its author had in view. Retire a command tomorrow and
//! it is covered on the next run, by nobody's decision.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use intent_cli::dispatch;
use testkit::workspace_root;

/// An unmigrated v2 project, BUILT rather than borrowed.
///
/// **This used to be `install_root()`, on the grounds that this repository was
/// itself an unmigrated v2 project. The hoist made that false** (2026-08-18):
/// Intent now declares `3.0.0-dev` and carries its own canon, so `st list` here
/// SUCCEEDS at 0 and the differential below lost the genuine refusal it is
/// built on. The test caught it -- its precondition assertion names exactly
/// this -- but it caught a property it had borrowed from its environment, and
/// the environment was always free to move.
///
/// So the condition is created here instead. Nothing about the property under
/// test needs this repository specifically; it needs *a live command that ran
/// and said no*, which an unmigrated project produces by construction
/// (AC-10.7). The install is still resolved from the binary's own path, not
/// from the working directory, so pointing the runs at a temporary project
/// costs nothing.
fn unmigrated_project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir .config");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"2.19.0\",\n  \"project_name\": \"Retired\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"shell\"]\n}\n",
  )
  .expect("write config");
  // A thread in v2 shape -- `info.md` and no `thread.json` -- so the project is
  // unmigrated by EVIDENCE as well as by declaration, which is the pair
  // AC-10.7 detects on.
  let td = dir.path().join("intent").join("st").join("ST0001");
  std::fs::create_dir_all(&td).expect("mkdir thread");
  std::fs::write(
    td.join("info.md"),
    "---\nstatus: In Progress\n---\n\n# ST0001: a real thread\n",
  )
  .expect("write v2 info.md");
  dir
}

/// The Intent install root.
fn install_root() -> PathBuf {
  workspace_root()
    .parent()
    .and_then(Path::parent)
    .expect("the rust workspace sits two levels under the Intent install")
    .to_path_buf()
}

fn run(args: &[&str], cwd: &Path) -> (Option<i32>, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(cwd)
    .stdin(Stdio::null())
    .output()
    .expect("run the v3 binary");
  (
    out.status.code(),
    String::from_utf8_lossy(&out.stderr).to_string(),
  )
}

/// Every spelling the table retires, as argv, with the row it came from.
///
/// **A RECLAIMED SPELLING IS EXCLUDED, AND IT IS NOT THE SAME AS A RETIRED ONE**
/// (hv, 2026-08-19). `path` stopped being unique across this table when hv
/// reclaimed `organize` for v3: the v2 face is still a retired row, and a
/// separate shipped row now carries the same word for a different program. Typing
/// it reaches the shipped verb, correctly, and answers `is a known command that
/// is not implemented yet` -- which is a true statement about v3 and is not the
/// retirement message this file asserts.
///
/// **THE COST OF THE EXCLUSION IS STATED RATHER THAN HIDDEN:** for a reclaimed
/// spelling, nothing here checks that the v2 face is refused, because there is no
/// longer any way to type it. That is a real loss of coverage and it is the
/// unavoidable half of reclaiming a name -- the alternative is asserting a
/// refusal the surface cannot produce.
fn retired_spellings() -> Vec<(Vec<String>, String)> {
  let table = dispatch::table();
  let shipped: std::collections::BTreeSet<&str> = dispatch::shipped_entries(&table)
    .iter()
    .map(|e| e.path.as_str())
    .collect();
  table
    .retired()
    .iter()
    .filter(|e| !shipped.contains(e.path.as_str()))
    .flat_map(|e| {
      e.spellings()
        .into_iter()
        .map(|s| {
          (
            s.iter().map(|seg| seg.to_string()).collect::<Vec<String>>(),
            e.target.spelling.clone(),
          )
        })
        .collect::<Vec<_>>()
    })
    .collect()
}

/// **The fixture proves itself.** Every assertion below iterates the retired
/// set; an empty set agrees with all of them, silently, and a table parse that
/// dropped `disposition` would produce exactly that.
#[test]
fn the_table_declares_retirements_for_this_file_to_measure() {
  let spellings = retired_spellings();
  assert!(
    spellings.len() >= 2,
    "the dispatch table yielded {} retired spellings. Every case in this file iterates that set, so an empty or near-empty one makes the whole file vacuous",
    spellings.len()
  );
  assert!(
    spellings
      .iter()
      .any(|(_, replacement)| !replacement.is_empty()),
    "no retired row carries a replacement spelling, so the branch that names one is never exercised -- and that branch is what issue 0044 asks for"
  );
}

/// **The refusal names the command and says what happened to it.**
///
/// `unrecognized subcommand 'treeindex'` is true and tells a v2 user nothing:
/// it cannot distinguish a command that was removed from one they mistyped.
#[test]
fn every_retired_spelling_is_refused_by_name() {
  let root = install_root();
  for (argv, replacement) in retired_spellings() {
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let (code, stderr) = run(&args, &root);
    let typed = argv.join(" ");

    assert!(
      stderr.contains(&format!("`intent {typed}`")),
      "`intent {typed}` was not named in its own refusal. **The spelling reported must be the one TYPED**, not the row's canonical path -- `st organise` is an \
       alias on a retired row, and answering it by naming `st organize` asks someone mid-migration to reconcile two spellings differing by one letter before \
       they can read the one fact they need. exit {code:?}, stderr: {stderr}"
    );
    assert!(
      stderr.contains("retired"),
      "`intent {typed}` failed without saying it was retired, which is the whole information a migrating caller needs: {stderr}"
    );
    if replacement.is_empty() {
      assert!(
        stderr.contains("no v3 replacement"),
        "`intent {typed}` has no replacement in the table and must say so -- silence there reads as an omission rather than as an answer: {stderr}"
      );
    } else {
      assert!(
        stderr.contains(&replacement),
        "`intent {typed}` is replaced by `{replacement}` in the table and the refusal did not name it. The register already holds the mapping; not reading it \
         is the whole of 0044's first proposed fix: {stderr}"
      );
    }
  }
}

/// **vc's canary, and it is the one assertion this issue reduces to: a retired
/// command must not answer in the same code as a command that RAN and said no.**
///
/// The differential is measured rather than assumed. The fixture is an
/// unmigrated v2 project, so a live command there produces a genuine runtime
/// refusal -- the exact condition 0044 records as sharing `1` with retirement.
/// `critic` would be the sharper partner and is not built yet; the property is
/// the same one, driven through the refusal that ships today.
#[test]
fn a_retired_command_and_a_genuine_refusal_do_not_share_a_code() {
  let project = unmigrated_project();
  let root = project.path().to_path_buf();

  let (refusal_code, refusal) = run(&["st", "list"], &root);
  assert_eq!(
    refusal_code,
    Some(1),
    "precondition: a live command in an unmigrated project refuses at 1. If this changed, the comparison below is measuring something else: {refusal}"
  );
  assert!(
    !refusal.contains("retired"),
    "precondition: `st list` is a LIVE command and must not be answered by the retired path -- a shipped name matching a retired row would mean the surface is \
     no longer the authority on what works: {refusal}"
  );

  for (argv, _) in retired_spellings() {
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let (code, stderr) = run(&args, &root);
    assert_ne!(
      code,
      refusal_code,
      "`intent {}` answers in the same code as a command that ran and legitimately refused. **A caller cannot then tell 'that command no longer exists' from \
       'your code has findings'**, and the measured consequence was a devbin gate reporting success over two directories it had failed to index. stderr: {stderr}",
      argv.join(" ")
    );
  }
}

/// **D37: our own ids never reach a user's terminal.**
///
/// The table's `target.ratification` notes are the natural place to source an
/// explanation from and they are full of hv rulings, dates, and our design and
/// criterion ids. Printing them would be the most informative wrong thing to
/// do, so the refusal is built from structural fields only -- and this is what
/// keeps it that way.
#[test]
fn the_refusal_carries_none_of_our_own_bookkeeping() {
  let root = install_root();
  for (argv, _) in retired_spellings() {
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let (_, stderr) = run(&args, &root);
    for marker in ["AC-", "AT-", "WP-", "ST00", "D21", "hv,", "ratification"] {
      assert!(
        !stderr.contains(marker),
        "the refusal for `intent {}` leaked `{marker}` -- that is our project's bookkeeping reaching a user's terminal (D37), and the likely route is a \
         ratification note being printed rather than the structural fields: {stderr}",
        argv.join(" ")
      );
    }
  }
}

/// **Longest path first, which is a contract rather than an implementation
/// detail.**
///
/// A caller matching a command line against these prefixes must try
/// `st organize` before `st`. No retired path is a prefix of another TODAY, so
/// getting this wrong would be invisible -- and a sort that is only correct
/// because the data has not yet reached the case it guards is worth pinning
/// before it does.
#[test]
fn retired_rows_are_ordered_longest_path_first() {
  let lengths: Vec<usize> = dispatch::table()
    .retired()
    .iter()
    .map(|e| e.path.split(' ').count())
    .collect();
  assert!(
    lengths.windows(2).all(|w| w[0] >= w[1]),
    "the retired rows are not ordered longest-first: {lengths:?}. A shorter path earlier in the list can absorb a command line meant for a longer one, and the \
     wrong row's replacement is then reported as the answer"
  );
}

/// **An unknown command is still an unknown command.** The retired path must
/// not become a catch-all that reports every typo as a retirement.
/// **A NAME THIS BUILD RECLAIMED IS A LIVE COMMAND, AND A TYPO ON IT IS A TYPO.**
///
/// `retired_refusal` is consulted after clap fails, and clap fails for two very
/// different reasons: a name that does not exist, and a USAGE ERROR on a name
/// that does. It matched argv by prefix against the retired roster, so for the
/// one name v3 reclaimed the second case was answered as the first.
///
/// **Measured before the fix: `intent organize --zzz-not-a-flag` answered _`intent
/// organize` was retired in Intent v3 and is not a command in this build_,
/// remedy _there is no v3 replacement -- remove it from any script that calls
/// it_, at exit 2.** A working verb told an operator who mistyped a flag to
/// delete it from their automation. Exit 2 is additionally the code the
/// pre-commit gate fails open on.
///
/// **DRIVEN FROM THE TABLE, never from the name `organize`.** The reclaimed set
/// is one row today and the defect is a property of reclamation, not of that
/// verb; a test naming it would stop covering the second reclamation on the day
/// it lands. `retired_spellings()` above already excludes shipped paths, so this
/// hole is invisible to every other assertion in this file by construction --
/// which is exactly why it needs its own arm rather than a wider filter.
#[test]
fn a_live_command_that_reclaimed_a_retired_name_is_not_answered_as_retired() {
  let root = install_root();
  let table = dispatch::table();
  let shipped: std::collections::BTreeSet<&str> = dispatch::shipped_entries(&table)
    .iter()
    .map(|e| e.path.as_str())
    .collect();

  let reclaimed: Vec<String> = table
    .retired()
    .iter()
    .map(|e| e.path.clone())
    .filter(|p| shipped.contains(p.as_str()))
    .collect();

  // Not an assertion that the set is non-empty: reclamation is a thing the
  // estate may legitimately have none of. It IS reported, so a run where the
  // set silently emptied does not read as a pass over a population.
  println!("reclaimed names measured: {reclaimed:?}");

  for path in &reclaimed {
    let mut args: Vec<&str> = path.split(' ').collect();
    args.push("--zzz-not-a-flag");
    let (code, stderr) = run(&args, &root);
    assert!(
      !stderr.contains("retired"),
      "`intent {}` SHIPS in this build; a usage error on it must not answer as a retirement -- that tells an operator to delete a working command from their scripts: {stderr}",
      path
    );
    assert_eq!(
      code,
      Some(1),
      "`intent {}` -- a usage error on a live command is exit 1, not the unavailable code the gate fails open on: {stderr}",
      path
    );
  }
}

/// **AND THE RECLAMATION MUST NOT HAVE UN-RETIRED THE FACE BENEATH IT.** The
/// guard walks the whole spelling, so `st organize` stays retired while
/// `organize` is live -- `st` is reachable and `organize` UNDER it is not. A
/// guard that checked only the first token would have answered `st organize` as
/// a usage error and quietly resurrected a v2 face.
#[test]
fn a_retired_face_under_a_live_parent_is_still_refused_by_name() {
  let root = install_root();
  let nested: Vec<Vec<String>> = retired_spellings()
    .into_iter()
    .map(|(s, _)| s)
    .filter(|s| s.len() > 1)
    .collect();
  assert!(
    !nested.is_empty(),
    "the table must declare at least one retired SUBcommand, or this arm measures nothing"
  );
  for spelling in &nested {
    let args: Vec<&str> = spelling.iter().map(String::as_str).collect();
    let (code, stderr) = run(&args, &root);
    assert!(
      stderr.contains("retired"),
      "`intent {}` is a retired face under a live parent and must still be refused by name: {stderr}",
      spelling.join(" ")
    );
    assert_eq!(code, Some(2), "and at the unavailable code: {stderr}");
  }
}

#[test]
fn a_command_that_never_existed_is_not_reported_as_retired() {
  let root = install_root();
  for args in [vec!["nonsense"], vec!["st", "nonsense"]] {
    let (_, stderr) = run(&args, &root);
    assert!(
      !stderr.contains("retired"),
      "`intent {}` was never a command and must not be answered as though it were removed -- telling someone their typo used to work sends them looking for a \
       migration note that does not exist: {stderr}",
      args.join(" ")
    );
  }
}
