//! ST0078 WP-05 through the real binary and real git: **after a pull, one
//! command brings a clone and its store back into step** (AT-05.1 to AT-05.4).
//!
//! hv, 2026-09-18: nobody should type ten commands to bring a clone and its
//! store back into step. Bare `intent sync` prints the plan and writes
//! nothing; `intent sync --apply` applies it. Alice and Bob clone one origin
//! under one isolated `HOME`, with no daemon and no hooks, so every change
//! here is the verb's. The origin, both clones and every commit are common's
//! [`Forge`]'s.
//!
//! **THE MERGE ARMS DRIVE THE CASE P2 COULD NOT**: P2's renumber repairs a
//! collision before the merge. Here the merge has already stopped on it, the
//! old id's paths hold the pulled side, and the plain verb would move them.

use std::path::{Path, PathBuf};

use crate::common::{Forge, Said};

/// The creation events the two clones' ordinal records were minted by, as
/// the event files a branch cut before the upgrade carries.
const ALICES_CREATION: &str = "01JA0000000000000000000001";
const BOBS_CREATION: &str = "01JA0000000000000000000002";

/// The forge's origin, Alice's clone with an initialised project pushed to
/// it, and Bob's clone of that.
struct Team {
  forge: Forge,
  alice: PathBuf,
  bob: PathBuf,
}

impl Team {
  fn new() -> Self {
    let forge = Forge::new();
    let alice = forge.clone_as("alice");
    forge.intent_ok(&alice, &["init", "Team"]);
    forge.commit_all(&alice, "intent init");
    forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
    let bob = forge.clone_as("bob");
    forge.intent_ok(&bob, &["sync", "--apply"]);
    Self { forge, alice, bob }
  }

  /// Both clones mint their next thread, and both get the same id. Bob starts
  /// his, so it is realised and declared; the merge then stops on the canon
  /// add/add. Returns what the merge said and the id Bob's mint printed.
  /// **AN ORDINAL IN FLIGHT, SEEDED ON BOTH SIDES** (ST0095 WP-06). A mint
  /// makes a git-style id now, which never collides, so the collision under
  /// repair is the one a branch cut before the upgrade carries: one ordinal,
  /// written by each clone with its own creation event. Bob's event is
  /// [`BOBS_CREATION`], which his record's new id derives from.
  fn both_mint_the_same_id_and_bob_pulls(&self) -> (String, String) {
    let id = intentsvcs::id::ThreadId::from_ordinal(1)
      .expect("an ordinal thread")
      .stored();
    crate::common::seed_ordinal_thread(&self.alice, &id, "Alice's next", &[]);
    crate::common::seed_creation_event(&self.alice, ALICES_CREATION, "st.new", "thread", &id);
    // The views her mint rendered, as a branch cut before the upgrade carries
    // them, so the merge conflicts in the generated views as well as in canon.
    self.forge.intent_ok(&self.alice, &["sync", "--to-disk"]);
    self.forge.commit_all(&self.alice, &format!("alice: {id}"));
    self
      .forge
      .git(&self.alice, &["push", "-q", "origin", "HEAD:main"]);
    crate::common::seed_ordinal_thread(&self.bob, &id, "Bob's next", &[]);
    crate::common::seed_creation_event(&self.bob, BOBS_CREATION, "st.new", "thread", &id);
    self
      .forge
      .intent_ok(&self.bob, &["st", "start", id.as_str()]);
    self.forge.commit_all(&self.bob, &format!("bob: {id}"));
    let merge = Said::from(
      self
        .forge
        .git_try(&self.bob, &["pull", "-q", "--no-rebase", "origin", "main"]),
    );
    assert_ne!(
      merge.code, 0,
      "the fixture needs the merge to stop:\n{}",
      merge.text
    );
    (merge.text, id)
  }

  /// The paths git holds unmerged in `clone`.
  fn unmerged(&self, clone: &Path) -> String {
    self
      .forge
      .git(clone, &["diff", "--name-only", "--diff-filter=U"])
  }
}

/// The digest a printed plan names, `(plan <digest>)`.
fn digest_of(plan: &str) -> String {
  plan
    .split("(plan ")
    .nth(1)
    .and_then(|rest| rest.split(')').next())
    .unwrap_or_default()
    .to_string()
}

/// AT-05.1: **the plan is ordered steps with their recoverability, under a
/// digest, and writes nothing; `--apply --plan <digest>` refuses a tree that
/// has moved.** Bob fetches Alice's commit without pulling, so the first step
/// is the branch behind its upstream -- reported, and nothing is run.
#[test]
fn the_plan_writes_nothing_and_an_apply_after_the_tree_moved_is_refused() {
  let team = Team::new();
  let forge = &team.forge;
  let st =
    crate::common::created(&forge.intent_ok(&team.alice, &["st", "new", "Release checklist"]));
  forge.commit_all(&team.alice, &format!("alice: {st}"));
  forge.git(&team.alice, &["push", "-q", "origin", "HEAD:main"]);
  forge.git(&team.bob, &["fetch", "-q"]);

  let status_before = forge.git(&team.bob, &["status", "--porcelain"]);
  let head_before = forge.git(&team.bob, &["rev-parse", "HEAD"]);
  let plan = forge.intent_ok(&team.bob, &["sync"]);
  assert!(plan.starts_with("plan: "), "{plan}");
  assert!(
    plan.contains(". behind (quiet): origin/main holds 1 commit(s) this branch does not"),
    "the branch behind its upstream is the first thing said: {plan}"
  );
  assert!(
    plan
      .lines()
      .last()
      .is_some_and(|l| l.contains("  then: doctor (quiet): ")),
    "and doctor is last: {plan}"
  );
  let counted: usize = plan
    .split("plan: ")
    .nth(1)
    .and_then(|rest| rest.split(' ').next())
    .and_then(|n| n.parse().ok())
    .unwrap_or_default();
  let numbered = plan
    .lines()
    .filter(|l| {
      l.trim_start()
        .split_once(". ")
        .is_some_and(|(n, _)| n.parse::<usize>().is_ok())
    })
    .count();
  assert_eq!(
    counted, numbered,
    "the summary counts exactly the numbered steps: {plan}"
  );
  let digest = digest_of(&plan);
  assert_eq!(digest.len(), 64, "the plan names its digest: {plan}");
  assert_eq!(
    forge.git(&team.bob, &["status", "--porcelain"]),
    status_before,
    "the plan wrote nothing git can see"
  );

  // The tree moves between the plan and the apply.
  std::fs::write(
    team.bob.join("intent/moved.md"),
    "a file the plan never read\n",
  )
  .expect("move the tree");
  let refused = forge.intent_said(&team.bob, &["sync", "--apply", "--plan", &digest]);
  assert_eq!(refused.code, 1, "{}", refused.text);
  assert!(
    refused
      .text
      .contains("the plan you were shown read the tree as"),
    "{}",
    refused.text
  );
  assert!(refused.text.contains("remedy:"), "{}", refused.text);

  // Against the tree as it now stands, the apply runs, says the branch is
  // behind, and pulls nothing.
  let replanned = forge.intent_ok(&team.bob, &["sync"]);
  let applied = forge.intent_said(
    &team.bob,
    &["sync", "--apply", "--plan", &digest_of(&replanned)],
  );
  assert!(
    applied
      .text
      .contains("note: behind: origin/main holds 1 commit(s)"),
    "{}",
    applied.text
  );
  assert_eq!(
    forge.git(&team.bob, &["rev-parse", "HEAD"]),
    head_before,
    "intent never pulls"
  );
}

/// AT-05.3: **a merge stopped on an id both clones minted is
/// repaired by `--apply --yes`**: this clone's thread moves to the next id free
/// in the store and the tree, the pulled one keeps its id, the store takes the
/// merged canon, and doctor's verdict is the exit code. No view is left
/// unmerged to regenerate, because the lists both clones rewrote travel in no
/// commit (ST0096). **Only what it regenerated is staged,
/// and it never commits, pulls or pushes.**
// ST0095 AT-05.3: the arm below.
#[test]
fn a_merge_stopped_on_a_twice_minted_id_is_repaired_and_only_what_was_regenerated_is_staged() {
  let team = Team::new();
  let forge = &team.forge;
  let (merge, minted) = team.both_mint_the_same_id_and_bob_pulls();
  // Where sync's repair moves this clone's thread: the git-style id its own
  // creation event mints (ST0095 WP-06), which this clone's log holds.
  let moved = intentsvcs::id::ThreadId::from_origin(BOBS_CREATION).stored();
  let bob = &team.bob;
  assert!(
    team
      .unmerged(bob)
      .contains(&format!("intent/.canon/st/{minted}.json")),
    "the fixture stops on the canon add/add:\n{merge}"
  );
  std::fs::write(bob.join("notes.txt"), "Bob's own file\n").expect("an untracked file");
  let head = forge.git(bob, &["rev-parse", "HEAD"]);
  let origin_main = forge.git(&forge.bare(), &["rev-parse", "main"]);

  let plan = forge.intent_ok(bob, &["sync"]);
  assert!(
    plan.contains(&format!(
      ". renumber (reversible): both sides minted steel thread {minted}: this clone's moves to {moved}"
    )),
    "{plan}"
  );
  assert!(
    !plan.contains(". behind ("),
    "mid-pull, the upstream's commits are the ones being merged, so nobody is told to pull: {plan}"
  );
  assert!(
    plan.contains(". ingest (quiet): take the merged canon into the store; what it takes can be read once the conflicts are resolved"),
    "the ingest waits on the canon conflict: {plan}"
  );

  // No terminal and no `--yes`: the renumber is left, and what reads the
  // merged canon waits on it.
  let left = forge.intent_said(bob, &["sync", "--apply"]);
  let line = left
    .text
    .lines()
    .find(|l| l.starts_with("left: "))
    .unwrap_or_default()
    .to_string();
  assert!(
    line.contains("renumber (reversible)") && line.contains("ingest (waits on the conflicts)"),
    "{}",
    left.text
  );
  assert!(
    team
      .unmerged(bob)
      .contains(&format!("intent/.canon/st/{minted}.json")),
    "nothing was resolved without an answer"
  );
  assert!(
    left
      .text
      .lines()
      .any(|l| l.starts_with("doctor: ") && l.contains("the estate could not be read")),
    "mid-merge the closing doctor says it could not read the estate, not that it is empty: {}",
    left.text
  );

  // `--yes` answers the reversible steps.
  let repaired = forge.intent_said(bob, &["sync", "--apply", "--yes"]);
  assert!(
    repaired.text.contains(&format!(
      "ok: renumbered this clone's {minted} to {moved}, kept the pulled {minted}"
    )),
    "{}",
    repaired.text
  );
  // No view was unmerged to regenerate: the two lists both clones rewrote are
  // each clone's own render and travel in no commit (ST0096), so the plan
  // names no views to resolve.
  assert!(
    !plan.contains(". resolve views (") && !repaired.text.contains("ok: regenerated and staged"),
    "no view is unmerged once the lists leave git: {plan}\n{}",
    repaired.text
  );
  assert_eq!(
    team.unmerged(bob).trim(),
    "",
    "nothing is left unmerged: {}",
    repaired.text
  );
  let last = repaired.text.lines().last().unwrap_or_default();
  assert!(
    last.starts_with("doctor: 0 finding(s)") && repaired.code == 0,
    "doctor runs last and its verdict is the exit code (rc {}): {}",
    repaired.code,
    repaired.text
  );

  // AT-05.4: staged is intent's own, the untracked file is untouched, and no
  // commit, pull or push happened.
  let staged = forge.git(bob, &["diff", "--cached", "--name-only"]);
  for path in staged.lines() {
    assert!(
      path.starts_with("intent/"),
      "sync staged a path it did not regenerate: {path}"
    );
  }
  assert!(
    forge
      .git(bob, &["status", "--porcelain"])
      .contains("?? notes.txt"),
    "an untracked file stays untracked"
  );
  assert_eq!(
    forge.git(bob, &["rev-parse", "HEAD"]),
    head,
    "sync never commits"
  );
  let (_, _, merging) = forge.git_try(bob, &["rev-parse", "-q", "--verify", "MERGE_HEAD"]);
  assert_eq!(merging, 0, "the merge is still the person's to commit");
  assert_eq!(
    forge.git(&forge.bare(), &["rev-parse", "main"]),
    origin_main,
    "sync never pushes"
  );

  // The person commits the merge, and both threads are there.
  forge.commit(bob, &["--no-edit"]);
  let list = forge.intent_ok(bob, &["st", "list", "--status", "all"]);
  assert!(
    list.contains(&minted) && list.contains("Alice's next"),
    "{list}"
  );
  assert!(
    list.contains(&crate::common::shown(&moved)) && list.contains("Bob's next"),
    "{list}"
  );
}

/// **A canon content conflict always asks a person, and `--yes`
/// does not answer it.** With no terminal it is left and named, and the steps
/// that read the merged canon wait. Answered -- here by the library's `ask`,
/// the one door the terminal prompt also goes through -- it takes the side
/// chosen and stages it.
#[test]
fn a_canon_content_conflict_needs_a_person_and_yes_does_not_answer_it() {
  let team = Team::new();
  let forge = &team.forge;
  let st = crate::common::created(&forge.intent_ok(&team.alice, &["st", "new", "Shared"]));
  let canon = format!("intent/.canon/st/{st}.json");
  forge.commit_all(&team.alice, &format!("alice: {st}"));
  forge.git(&team.alice, &["push", "-q", "origin", "HEAD:main"]);
  forge.git(&team.bob, &["pull", "-q", "--no-rebase", "origin", "main"]);
  forge.intent_ok(&team.bob, &["sync", "--apply"]);

  forge.intent_ok(&team.alice, &["set", st.as_str(), "title", "Alice's title"]);
  forge.commit_all(&team.alice, "alice: retitle");
  forge.git(&team.alice, &["push", "-q", "origin", "HEAD:main"]);
  forge.intent_ok(&team.bob, &["set", st.as_str(), "title", "Bob's title"]);
  forge.commit_all(&team.bob, "bob: retitle");
  let merge =
    Said::from(forge.git_try(&team.bob, &["pull", "-q", "--no-rebase", "origin", "main"]));
  assert_ne!(
    merge.code, 0,
    "the fixture needs the merge to stop:\n{}",
    merge.text
  );
  assert!(team.unmerged(&team.bob).contains(&canon));

  let left = forge.intent_said(&team.bob, &["sync", "--apply", "--yes"]);
  let line = left
    .text
    .lines()
    .find(|l| l.starts_with("left: "))
    .unwrap_or_default()
    .to_string();
  assert!(
    line.contains("take a side (non-reversible)")
      && line.contains("ingest (waits on the conflicts)"),
    "`--yes` does not answer a non-reversible step: {}",
    left.text
  );
  assert!(
    team.unmerged(&team.bob).contains(&canon),
    "and the conflict stands"
  );

  let project = intentsvcs::project::Project::open(&team.bob).expect("Bob's project");
  let ctx = intentsvcs::facade::FacadeContext::for_tests(
    "test".to_string(),
    project.config().project_id.clone().unwrap_or_default(),
    env!("CARGO_PKG_VERSION").to_string(),
  );
  let mut facade = intentsvcs::facade::Facade::open(project, ctx).expect("the facade opens");
  let mut asked = Vec::new();
  let applied = facade
    .sync_apply(
      &intentsvcs::sync::Scope::All,
      &intentsvcs::plan::Asking {
        yes: false,
        terminal: true,
        shown: None,
      },
      &mut |step| {
        asked.push(step.name());
        match step.action {
          intentsvcs::plan::Action::TakeSide { .. } => {
            intentsvcs::plan::Decision::Take(intentsvcs::plan::Side::Theirs)
          }
          _ => intentsvcs::plan::Decision::Run,
        }
      },
    )
    .expect("the apply runs");
  assert!(
    asked.contains(&"take a side"),
    "the person was asked: {asked:?}"
  );
  assert!(
    applied
      .done
      .iter()
      .any(|l| l.starts_with(&format!("took theirs for {canon}"))),
    "{:?}",
    applied.done
  );
  assert!(
    !team.unmerged(&team.bob).contains(&canon),
    "the side taken is staged"
  );
  let shown = forge.intent_ok(&team.bob, &["st", "show", st.as_str()]);
  assert!(shown.contains("Alice's title"), "{shown}");
}

/// ST0096 WP-05, on intent-vc's ruling that retired the step which kept both
/// (issue 0709): **IN 3.4.0'S FORMAT A THREAD BOTH SIDES ADDED A WORK PACKAGE
/// TO IS REFUSED, NOT OFFERED AS A SIDE TO TAKE.** Its packages are inside its
/// own file, so taking either side would drop the other side's package while
/// reading as an ordinary choice (issue 0555). The plan and the apply both
/// refuse, name the file and the fix 3.4.0 had, and write nothing. **AND THAT
/// FIX HOLDS**: each side's mint committed its `wp.new` event, and with the
/// file resolved by hand, one package renumbered, `sync --apply --yes` and
/// `doctor` both pass.
#[test]
fn a_thread_both_sides_added_packages_to_in_the_ordinal_format_is_refused() {
  let forge = Forge::new();
  let alice = forge.clone_as("alice");
  forge.intent_ok(&alice, &["init", "Team"]);
  crate::common::born_ordinal(&alice);
  let id = intentsvcs::id::ThreadId::from_ordinal(7)
    .expect("an ordinal thread")
    .stored();
  let first = (1, "First");
  crate::common::seed_ordinal_thread(&alice, &id, "Onboarding", &[first]);
  forge.commit_all(&alice, "a thread in the ordinal format");
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let bob = forge.clone_as("bob");
  forge.intent_ok(&bob, &["sync", "--apply"]);

  let package = intentsvcs::id::wp_address(
    &id,
    intentsvcs::id::WpId::from_ordinal(2).expect("an ordinal package"),
  );
  crate::common::seed_ordinal_thread(&alice, &id, "Onboarding", &[first, (2, "Alice's")]);
  crate::common::seed_creation_event(&alice, ALICES_CREATION, "wp.new", "wp", &package);
  forge.commit_all(&alice, "alice: a package");
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  crate::common::seed_ordinal_thread(&bob, &id, "Onboarding", &[first, (2, "Bob's")]);
  crate::common::seed_creation_event(&bob, BOBS_CREATION, "wp.new", "wp", &package);
  forge.commit_all(&bob, "bob: a package");
  let merge = Said::from(forge.git_try(&bob, &["pull", "-q", "--no-rebase", "origin", "main"]));
  assert_ne!(
    merge.code, 0,
    "the fixture needs the merge to stop on the thread's file:\n{}",
    merge.text
  );
  let project = intentsvcs::project::Project::open(&bob).expect("Bob's project");
  let canon = project.relative(&project.thread_json(&id));
  let before = forge.git(&bob, &["status", "--porcelain"]);

  for args in [&["sync"][..], &["sync", "--apply", "--yes"][..]] {
    let refused = forge.intent_said(&bob, args);
    assert_eq!(
      refused.code, 1,
      "`intent {args:?}` refuses: {}",
      refused.text
    );
    assert!(
      refused.text.contains(&format!(
        "both sides of this merge added work packages to {id}, which this project's ordinal \
         format keeps inside {canon}"
      )),
      "`intent {args:?}` names the thread and its file: {}",
      refused.text
    );
    assert!(
      refused
        .text
        .contains("give one of them the next number no package in the thread uses"),
      "`intent {args:?}` names the fix a merge in this format always had: {}",
      refused.text
    );
    assert!(
      !refused.text.contains("take a side ("),
      "`intent {args:?}` offers no side to take: {}",
      refused.text
    );
    assert_eq!(
      forge.git(&bob, &["status", "--porcelain"]),
      before,
      "`intent {args:?}` wrote nothing"
    );
  }

  // The remedy, as a person runs it: both packages kept in the thread's file,
  // the pulled one moved to the next number the thread does not use.
  crate::common::seed_ordinal_thread(
    &bob,
    &id,
    "Onboarding",
    &[first, (2, "Bob's"), (3, "Alice's")],
  );
  forge.git(&bob, &["add", canon.as_str()]);
  forge.intent_ok(&bob, &["sync", "--apply", "--yes"]);
  let packages = forge.intent_ok(&bob, &["wp", "list", id.as_str()]);
  for title in ["Bob's", "Alice's"] {
    assert!(
      packages.contains(title),
      "{title} package is kept: {packages}"
    );
  }
  forge.intent_ok(&bob, &["doctor"]);
}

/// Issue Ic70f05f: **A MERGE VERSION OF A THREAD'S FILE THAT WILL NOT READ IS
/// REFUSED, NOT A SIDE TO TAKE.** Bob adds a work package to a thread in the
/// ordinal format; the side he pulls carries the thread's file in bytes that
/// are not a thread. Whether both sides added packages was read from the three
/// versions, and an unreadable one answered "no", so the path was offered as a
/// side to take, and taking the pulled side drops Bob's package (issue 0555).
/// Now both `sync` and `sync --apply --yes` refuse, naming the path and the
/// version, and write nothing.
#[test]
fn a_merge_version_of_a_threads_file_that_will_not_read_is_refused_not_a_side_to_take() {
  let forge = Forge::new();
  let alice = forge.clone_as("alice");
  forge.intent_ok(&alice, &["init", "Team"]);
  crate::common::born_ordinal(&alice);
  let id = intentsvcs::id::ThreadId::from_ordinal(7)
    .expect("an ordinal thread")
    .stored();
  let first = (1, "First");
  crate::common::seed_ordinal_thread(&alice, &id, "Onboarding", &[first]);
  forge.commit_all(&alice, "a thread in the ordinal format");
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let bob = forge.clone_as("bob");
  forge.intent_ok(&bob, &["sync", "--apply"]);

  let project = intentsvcs::project::Project::open(&alice).expect("Alice's project");
  let canon = project.relative(&project.thread_json(&id));
  std::fs::write(alice.join(&canon), "{ \"schema\": \"not a thread\n").expect("write the bytes");
  forge.commit_all(&alice, "alice: the thread's file, unreadable");
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  crate::common::seed_ordinal_thread(&bob, &id, "Onboarding", &[first, (2, "Bob's")]);
  forge.commit_all(&bob, "bob: a package");
  let merge = Said::from(forge.git_try(&bob, &["pull", "-q", "--no-rebase", "origin", "main"]));
  assert_ne!(
    merge.code, 0,
    "the fixture needs the merge to stop on the thread's file:\n{}",
    merge.text
  );
  let before = forge.git(&bob, &["status", "--porcelain"]);

  for args in [&["sync"][..], &["sync", "--apply", "--yes"][..]] {
    let refused = forge.intent_said(&bob, args);
    assert!(
      refused.code == 1
        && refused.text.contains(&format!(
          "the :3 version of {canon} in this merge is not a readable thread file"
        )),
      "`intent {args:?}` refuses, naming the path and the version, where it offered the path \
       as a side to take, and taking the pulled side drops Bob's package: {}",
      refused.text
    );
    assert!(
      !refused.text.contains("take a side ("),
      "`intent {args:?}` offers no side to take: {}",
      refused.text
    );
    assert_eq!(
      forge.git(&bob, &["status", "--porcelain"]),
      before,
      "`intent {args:?}` wrote nothing"
    );
  }
}

/// Issue I33fa93f: **THE FACADE'S `PackageAddedTwice` REFUSAL, END TO END.**
/// Two branches cut before the upgrade each make the thread's next ordinal
/// package, then each runs `intent upgrade`, so each holds that package as its
/// own record file under one id. Their merge stops on that file as an add/add,
/// and `sync` refuses rather than take a side, naming the package and its file,
/// and writes nothing.
#[test]
fn a_package_two_branches_made_before_the_upgrade_is_refused_end_to_end() {
  let forge = Forge::new();
  let alice = forge.clone_as("alice");
  forge.intent_ok(&alice, &["init", "Team"]);
  crate::common::born_ordinal(&alice);
  let id = intentsvcs::id::ThreadId::from_ordinal(7)
    .expect("an ordinal thread")
    .stored();
  let first = (1, "First");
  crate::common::seed_ordinal_thread(&alice, &id, "Onboarding", &[first]);
  forge.commit_all(&alice, "a thread in the ordinal format");
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let bob = forge.clone_as("bob");
  forge.intent_ok(&bob, &["sync", "--apply"]);

  let next = 2;
  let package = intentsvcs::id::WpId::from_ordinal(next).expect("an ordinal package");
  for (copy, title) in [(&alice, "Alice's"), (&bob, "Bob's")] {
    crate::common::seed_ordinal_thread(copy, &id, "Onboarding", &[first, (next, title)]);
    forge.commit_all(copy, &format!("{title} package, before the upgrade"));
    forge.intent_ok(copy, &["upgrade"]);
    forge.commit_all(copy, "intent upgrade");
  }
  forge.git(&alice, &["push", "-q", "origin", "HEAD:main"]);
  let merge = Said::from(forge.git_try(&bob, &["pull", "-q", "--no-rebase", "origin", "main"]));
  assert_ne!(
    merge.code, 0,
    "the fixture needs the merge to stop on the package's file:\n{}",
    merge.text
  );
  let before = forge.git(&bob, &["status", "--porcelain"]);

  for args in [&["sync"][..], &["sync", "--apply", "--yes"][..]] {
    let refused = forge.intent_said(&bob, args);
    assert!(
      refused.code == 1
        && refused.text.contains(&format!(
          "both sides of this merge added a work package as {id}/{}",
          package.stored()
        )),
      "`intent {args:?}` refuses, naming the package: {}",
      refused.text
    );
    assert!(
      refused.text.contains("git merge --abort"),
      "`intent {args:?}` gives the remedy: {}",
      refused.text
    );
    assert_eq!(
      forge.git(&bob, &["status", "--porcelain"]),
      before,
      "`intent {args:?}` wrote nothing"
    );
  }
}
