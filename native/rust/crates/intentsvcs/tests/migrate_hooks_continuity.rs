//! **AT-10.4: `upgrade` leaves the hook estate alone -- asserted over the
//! PLANNED WRITES, because the bytes on their own cannot fail.**
//!
//! # The criterion as worded is true by construction, and that is the finding
//!
//! AC-10.4 asks for `.claude/settings.json` + `.claude/scripts/**`
//! byte-identical across a migration, *asserted not assumed*. Driven at HEAD
//! `8d20dc49`, and re-driven there rather than carried forward from the earlier
//! pin, because a measurement of a four-node tree is true of one rebuild:
//!
//! - `migrate.rs` names `.claude` **nowhere**.
//! - `Facade::upgrade` is `legacy::scan` -> `migrate::plan` ->
//!   `WriteSet::commit` -> `Store::rebuild` -> `converge_gitignore` ->
//!   `stamp_version`, and nothing else.
//! - `intent-cli`'s `upgrade` arm is `Facade::upgrade` plus printing.
//! - The only module in `intentsvcs` naming `.claude` in code is `install.rs`,
//!   whose whole public surface is `home`, `resolve` and `hook_script` --
//!   three path resolvers, no writer -- and whose only callers are the CLI arms
//!   for `hooks` and `claude`.
//!
//! So the bytes are equal because **no code path exists that could make them
//! unequal**, and a test of that alone is the vacuous green this thread exists
//! to refuse: it passes on a build where `upgrade` does nothing at all, and it
//! keeps passing right through the way this property is most likely to break.
//!
//! # Which is not hypothetical: v2's own `upgrade` DID propagate canon
//!
//! `propagate_canon_skills` sits on v2.x's upgrade path, and the shape of a
//! propagation is *rewrite these files from the template, usually to the bytes
//! already there*. **A propagation that is a no-op on content passes a
//! byte-identity assertion** -- and from that day `upgrade` OWNS those files,
//! so the next template change rewrites an operator's hooks with nothing
//! asserting otherwise.
//!
//! # So the falsifiable form is over the WRITE SET
//!
//! `migrate::plan` returns `Plan { writes: WriteSet }`, and `WriteSet::writes`
//! yields `(&Path, &str)` before a byte is committed. Asserting **no planned
//! path carries a `.claude` component** fails on exactly the identical-bytes
//! rewrite the first arm cannot see. It is asserted over a set separately
//! proven to be a real migration, or it is the same vacuity one level up.
//!
//! # Neither arm subsumes the other, which is why there are two
//!
//! | mutation                                                | bytes        | write set |
//! | ------------------------------------------------------- | ------------ | --------- |
//! | plan writes `.claude/settings.json`, changed            | dies         | dies      |
//! | plan writes `.claude/settings.json`, **identical bytes** | lives        | **dies**  |
//! | a write to `.claude` from **outside** the plan           | **dies**     | lives     |
//! | `upgrade` becomes a no-op returning `Ok`                 | control dies | lives     |
//!
//! The write set is blind to anything that does not travel through it; the
//! bytes are blind to a write that changes nothing. **And the control lives in
//! the same test as the bytes rather than beside it**, because *the hook estate
//! did not change* is equally what a migration that never ran produces.

mod common;

use std::path::PathBuf;

use common::{Fixture, changed, facade_ctx, tree, v2_estate, v2_thread};
use intentsvcs::facade::Facade;
use intentsvcs::{legacy, migrate};

/// A v2 estate carrying a hook install of the shape v2.10.0 lays down, plus two
/// threads that convert clean.
///
/// **The content is arbitrary and the structure is not.** AC-10.4 names
/// `.claude/settings.json` AND `.claude/scripts/**`, so the fixture holds a
/// file at the named path and two files *under* the named tree -- a fixture
/// with only the former leaves the `scripts/**` half of the criterion
/// unexercised while reading as though it covered it.
fn v2_estate_with_hooks() -> Fixture {
  let fx = v2_estate();
  fx.write_file(
    ".claude/settings.json",
    "{\n  \"hooks\": {\n    \"UserPromptSubmit\": [\n      { \"command\": \"intent claude hook require-in-session\" }\n    ]\n  }\n}\n",
  );
  fx.write_file(
    ".claude/scripts/require-in-session.sh",
    "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
  );
  fx.write_file(
    ".claude/scripts/post-tool-advisory.sh",
    "#!/usr/bin/env bash\nset -euo pipefail\nintent critic \"$1\" || true\n",
  );
  v2_thread(&fx, "ST0001", "WIP");
  v2_thread(&fx, "ST0002", "Completed");
  fx
}

/// **ARM ONE, AND ITS CONTROL, WHICH MUST BE THE SAME RUN.**
///
/// The literal criterion: every byte under `.claude/` survives a migration
/// unchanged. On its own that is satisfied by an `upgrade` that refused, or
/// crashed early, or was never reached -- so the same run must be shown to have
/// converted something.
///
/// The two assertions compose: the second says *some file moved*, the first
/// says *none of them was under `.claude/`*, so together they say the verb did
/// its job somewhere else. Neither wording depends on knowing what a migration
/// writes, which is what keeps this arm from having to be rewritten every time
/// one does.
#[test]
fn upgrade_leaves_the_hook_estate_byte_identical_and_the_same_run_did_convert() {
  let fx = v2_estate_with_hooks();
  let hooks = fx.path(".claude");

  let hooks_before = tree(&hooks);
  assert_eq!(
    hooks_before.len(),
    3,
    "the fixture must hold settings.json and both scripts, or the equality below is vacuous: {:?}",
    hooks_before.keys().collect::<Vec<_>>()
  );
  let estate_before = tree(fx.root());

  Facade::upgrade(&fx.project(), &facade_ctx()).expect("a clean v2 estate converts");

  let touched = changed(&hooks_before, &tree(&hooks));
  assert!(
    touched.is_empty(),
    "`upgrade` wrote to the hook estate: {touched:?}"
  );

  let moved = changed(&estate_before, &tree(fx.root()));
  assert!(
    !moved.is_empty(),
    "`upgrade` returned Ok having changed no file at all, so the byte-identity asserted above \
     is about a verb that never ran"
  );
}

/// **ARM TWO: THE ONE THAT CAN FAIL BEFORE THE BYTES DO.**
///
/// A plan naming `.claude/settings.json` with the bytes already there is
/// invisible to arm one and is the whole risk -- it is what a canon
/// propagation looks like on the day it lands, and v2's `upgrade` had one.
#[test]
fn the_migration_plan_names_no_path_under_dot_claude() {
  let fx = v2_estate_with_hooks();
  let project = fx.project();

  let scan = legacy::scan(&project).expect("a v2 estate scans");
  let plan = migrate::plan(&project, &facade_ctx(), scan).expect("a clean v2 estate plans");
  let planned: Vec<PathBuf> = plan.writes.writes().map(|(p, _)| p.to_path_buf()).collect();

  // **NON-VACUITY, and it is deliberately not `!planned.is_empty()`**: a plan
  // holding one unrelated file satisfies that and says nothing about a
  // migration having been planned. Both threads' canon must be in the set --
  // and the expected path is READ FROM `Project` rather than typed here, so the
  // predicate cannot be quietly fitted to whatever the planner happened to do.
  for id in ["ST0001", "ST0002"] {
    let canon = project.thread_json(id);
    assert!(
      planned.contains(&canon),
      "the plan must be a real migration -- {} is not among its {} writes",
      project.relative(&canon),
      planned.len()
    );
  }

  // **Matched on a path COMPONENT of the estate-relative path, not on a
  // substring of the absolute one.** A substring test over the absolute path is
  // a claim about the temp directory as much as about the plan; stripping the
  // root first makes this a fact about the project and nothing else.
  let hooks: Vec<String> = planned
    .iter()
    .filter(|p| {
      p.strip_prefix(fx.root())
        .is_ok_and(|rel| rel.components().any(|c| c.as_os_str() == ".claude"))
    })
    .map(|p| project.relative(p))
    .collect();

  assert!(
    hooks.is_empty(),
    "`migrate::plan` names the hook estate, so `upgrade` now owns files it must not: {hooks:?}"
  );
}
