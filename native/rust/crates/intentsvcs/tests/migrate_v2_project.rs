//! **AT-10.3 / AT-00.6: a clean v2 estate converts, and each of AC-10.3's five
//! limbs is asserted BY ITS OWN TEST.**
//!
//! # Why five tests and not one test with five assertions
//!
//! The criterion is a conjunction -- *canon emitted, views regenerated, config
//! stamped 3.0.0 + `project_id`, DB built, gitignore converged* -- and **one
//! `it converted` passes on four of five**. Five assertions in one body is
//! better than one, and still not enough: the first to fail masks the rest, so
//! a run reports one broken limb when three are broken and the operator fixes
//! one and re-runs. Five tests report five verdicts.
//!
//! # What was already covered, measured before a line of this was written
//!
//! `intent-cli/tests/upgrade_command.rs ::
//! a_v2_estate_migrates_through_the_binary_and_the_stamp_lands` already drives
//! **two** of the five end to end through the shipped binary -- canon emitted,
//! and gitignore converged including the negative that the rule is a PATH rule
//! rather than `*.db`. It carries `AT-10.1` and nothing links any of it here.
//!
//! **That is the fifth instance of this class today and the first where the
//! uncited coverage is PARTIAL, which is worse to find**: the practice that
//! caught the other four is subject-grep-then-drive-the-verb, and a subject
//! grep that hits the file tells you nothing about WHICH limbs it covers. The
//! file gets its own covering row rather than being folded into this one, per
//! the `AT-10.15` precedent -- the schema gives one `file` per row, and two
//! rows keep two assertions separately falsifiable.
//!
//! # The stamp limb was not a test gap
//!
//! `AC-10.3` says *config stamped 3.0.0 **+ `project_id`***, and `project_id`
//! was never written by anything. `stamp_version` inserted `intent_version`
//! and stopped; Intent's own self-hosted, migrated `config.json` carried no
//! such field; `intent export --format json` returned `project_id = ""` at
//! rc=0. **Three sites knew about it -- `project.rs` ruled it out as the
//! migration marker by reasoning that migrated projects have one, `migrate.rs`
//! commented on it being empty and promised the facade would mint it, and
//! `design.md` mandated it -- and none of them wrote it.** vc ruled the value a
//! UUID, 2026-08-20. The stamp landed with this file.
//!
//! And the only test that drove the stamp asserted
//! `assert_ne!(declared_version, "2.19.0")` -- that the version CHANGED, not
//! that it is right, **passing on a stamp of `banana`**. That is the
//! five-limbs argument arriving inside a single limb.

mod common;

use common::{Fixture, facade_ctx, v2_estate, v2_thread};
use intentsvcs::facade::Facade;

/// A clean v2 estate, converted. Panics rather than returning a Result: every
/// test below is about the *state after* a successful conversion, so a failure
/// here is a broken fixture and not a finding.
fn converted() -> Fixture {
  let fx = v2_estate();
  v2_thread(&fx, "ST0001", "WIP");
  v2_thread(&fx, "ST0002", "Completed");
  Facade::upgrade(&fx.project(), &facade_ctx()).expect("a clean v2 estate converts");
  fx
}

/// The `intent_version` this project declares, read back out of the config.
fn declared(fx: &Fixture) -> String {
  let text = fx.read("intent/.config/config.json");
  let v: serde_json::Value = serde_json::from_str(&text).expect("config is JSON");
  v["intent_version"].as_str().unwrap_or_default().to_string()
}

/// The `project_id` this project declares, or the empty string.
///
/// **Empty and absent are deliberately the same answer here**, because they are
/// the same answer everywhere else: `Config::project_id` is `Option<String>`
/// and every read site in the estate does `.unwrap_or_default()`, so an
/// unstamped project already presents `""`.
fn project_id(fx: &Fixture) -> String {
  let text = fx.read("intent/.config/config.json");
  let v: serde_json::Value = serde_json::from_str(&text).expect("config is JSON");
  v["project_id"].as_str().unwrap_or_default().to_string()
}

/// Whether `s` has the shape of a v4 UUID.
///
/// **Hand-written on purpose, and this is the provenance rule rather than
/// nostalgia.** The value is minted by `uuid::Uuid::new_v4`; validating it with
/// `uuid::Uuid::parse_str` would put the same crate on both sides of the
/// assertion, which passes by construction for any self-consistent library --
/// including one that minted a constant. The predicate must not be drawn from
/// the thing under test, so this reads the shape RFC 4122 specifies and nothing
/// else: 8-4-4-4-12 lowercase hex, and the version nibble is `4`.
fn looks_like_a_v4_uuid(s: &str) -> bool {
  let groups: Vec<&str> = s.split('-').collect();
  groups.len() == 5
    && [8, 4, 4, 4, 12] == groups.iter().map(|g| g.len()).collect::<Vec<_>>()[..]
    && groups.iter().all(|g| {
      g.chars()
        .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    })
    && groups[2].starts_with('4')
}

/// **LIMB 1: structured canon is emitted.**
#[test]
fn a_clean_estate_emits_structured_canon_for_every_thread() {
  let fx = converted();
  let project = fx.project();
  for id in ["ST0001", "ST0002"] {
    let canon = project.thread_json(id);
    assert!(
      canon.is_file(),
      "no committed canon at {}",
      project.relative(&canon)
    );
    let text = std::fs::read_to_string(&canon).expect("canon reads");
    let v: serde_json::Value = serde_json::from_str(&text).expect("canon is JSON");
    assert_eq!(
      v["id"].as_str(),
      Some(id),
      "canon exists at the right path and is not this thread's -- a path check alone \
       passes on a migrator that writes every thread's canon into one file"
    );
  }
}

/// **LIMB 2: the views are regenerated.**
///
/// Both kinds, because they fail independently: the per-thread `info.md` comes
/// from the thread's own model, and the project-level index is assembled from
/// the whole population. `migrator_population_is_canon.rs` already proves the
/// index names the RIGHT population; this proves it is written at all.
#[test]
fn a_clean_estate_regenerates_its_views() {
  let fx = converted();
  let project = fx.project();
  for id in ["ST0001", "ST0002"] {
    assert!(
      project.info_view(id).exists(),
      "no regenerated view for {id} -- {}",
      project.relative(&project.info_view(id))
    );
  }
  let index = project.steel_threads_view();
  assert!(
    index.exists(),
    "no project-level index at {}",
    project.relative(&index)
  );
  let text = std::fs::read_to_string(&index).expect("the index reads");
  assert!(
    text.contains("ST0001") && text.contains("ST0002"),
    "the index was written and names neither thread: {text:?}"
  );
}

/// **LIMB 3a: the config is stamped, and stamped to a value rather than merely
/// changed.**
///
/// Two assertions and they are not the same one. **Equality against the
/// binary's own `INTENT_VER` is the identity claim** -- it is what makes a v2
/// binary refuse this tree afterwards. **Major-3 is the CRITERION's claim** and
/// is independent of that constant, so it still fails if `INTENT_VER` is ever
/// wrong. The test this replaces asserted only `!= "2.19.0"`, which passes on
/// a stamp of `banana`.
#[test]
fn a_clean_estate_is_stamped_with_this_binarys_version() {
  let fx = converted();
  let stamped = declared(&fx);
  assert_eq!(
    stamped,
    intentsvcs::faces::INTENT_VER,
    "the migrated project does not declare the version of the binary that migrated it"
  );
  assert!(
    stamped.starts_with("3."),
    "the criterion says 3.0.0 -- a stamp that merely CHANGED satisfies `!= 2.19.0` and \
     not this: {stamped:?}"
  );
}

/// **LIMB 3b: `project_id` is minted, and it is a UUID.**
///
/// Non-emptiness is the first assertion because `""` is what every read site
/// already produces via `.unwrap_or_default()` -- so a stamp that wrote the
/// key with an empty value would look present and behave absent, forever.
#[test]
fn a_clean_estate_is_stamped_with_a_project_id() {
  let fx = converted();
  let id = project_id(&fx);
  assert!(
    !id.is_empty(),
    "no project_id after migration -- design.md D15 stamps one HERE, and every read site \
     defaults to the empty string, so an unstamped project is indistinguishable from a \
     stamped-empty one at every point downstream"
  );
  assert!(
    looks_like_a_v4_uuid(&id),
    "project_id is present and is not a v4 UUID: {id:?}"
  );
}

/// **LIMB 3c: the id is MINTED ONCE and survives a re-run.**
///
/// `upgrade` is re-runnable under the fix-forward ruling, so a fresh UUID per
/// run would mean a project's identity is whatever the last migration happened
/// to generate. `upgrade_command.rs` asserts the whole tree is byte-identical
/// on a second run, which covers this incidentally; asserted here by name and
/// by VALUE, because a whole-tree equality reports "something moved" and this
/// reports which thing and what it was.
#[test]
fn the_project_id_is_minted_once_and_a_re_run_preserves_it() {
  let fx = converted();
  let first = project_id(&fx);
  assert!(
    !first.is_empty(),
    "nothing to preserve, so this proves nothing"
  );

  Facade::upgrade(&fx.project(), &facade_ctx()).expect("a migrated estate re-runs");

  assert_eq!(
    project_id(&fx),
    first,
    "the second migration re-minted the project_id, so this project's global identity is \
     whatever the last `upgrade` generated"
  );
}

/// **LIMB 4: the DB is built, and it holds the estate.**
///
/// Existence alone is not the limb: `converge_gitignore` and `Store::open`
/// between them create the file, so a `.is_file()` check passes on a migration
/// that built an EMPTY store. The population is the claim.
#[test]
fn a_clean_estate_builds_a_store_that_holds_it() {
  let fx = converted();
  let project = fx.project();
  let db = project.db_path();
  assert!(db.is_file(), "no store at {}", project.relative(&db));

  let store = intentsvcs::store::Store::open(&db).expect("the store opens");
  let (threads, _issues) = store.load_canon().expect("the store loads");
  let mut ids: Vec<String> = threads.into_iter().map(|t| t.id).collect();
  ids.sort();
  assert_eq!(
    ids,
    vec!["ST0001".to_string(), "ST0002".to_string()],
    "the store exists and does not hold the estate that was just migrated into it"
  );
}

/// **LIMB 5: the gitignore is converged.**
///
/// The negative is half the limb and is the half that regresses: `*.db` would
/// also stop the store being committed, and would swallow a database the
/// operator wants tracked. D29 -- a gitignored path is never canon -- makes the
/// breadth of this rule a correctness question rather than a tidiness one.
#[test]
fn a_clean_estate_converges_its_gitignore_by_path_and_not_by_glob() {
  let fx = converted();
  let ignored = fx.read(".gitignore");
  assert!(
    ignored.lines().any(|l| l.trim() == "intent/.cache/"),
    "the runtime store is not gitignored after a migration: {ignored:?}"
  );
  assert!(
    !ignored.contains("*.db"),
    "the ignore rule is a PATH rule -- `*.db` would swallow a database the operator \
     wants tracked: {ignored:?}"
  );
}
