//! **The gate is two files, and canon used to write only one of them.**
//!
//! `canon::apply` has always written the chain block into `pre-commit`. That
//! block is:
//!
//! ```sh
//! _intent_chain="$(git rev-parse --git-path hooks 2>/dev/null)/pre-commit.intent"
//! if [ -x "$_intent_chain" ]; then
//!   "$_intent_chain" "$@" || exit $?
//! fi
//! ```
//!
//! **`if` with no `else`.** So until `install_carrier` existed, the one verb
//! whose job is wiring the gate produced a reference to a file that nothing in
//! either tree ever wrote -- and the result was not an error. It was a commit
//! at rc=0, in a project whose every report said the gate was wired.
//!
//! **AND THE BLOCK STAYED SILENT UNTIL ISSUE `0538`.** Installing the carrier
//! made the absent case rarer and left it passing: the carrier is gitignored, so
//! a clone or a worktree reached through `core.hooksPath` has the block and not
//! the carrier. The block now refuses there, and the arms at the end of this
//! file drive every way it can find no carrier to call.
//!
//! Baize is the measured instance: `intent_version` 3.0.0, canon present, fully
//! ported, whiteboard nodes at work, and a gate running nothing.
//!
//! # Why these arms and not "does apply write the file"
//!
//! A presence test passes on an implementation that writes the wrong bytes from
//! the wrong tree with the wrong mode. Each arm below pins a property that a
//! plausible implementation gets wrong, and every one of them was reachable:
//!
//! - **Source.** dc measured many estates carrying the FROZEN v2 tree's gate
//!   byte for byte, nearly all stamped the same day. The last fleet-wide
//!   install read from a tree nobody develops in. **The defect was what the
//!   installer read**, so an installer that reads from anywhere but the
//!   resolved install root reproduces today's state exactly, and at rc=0.
//! - **Mode.** `[ -x ]` is the test the block applies. Right bytes plus wrong
//!   mode is the silent skip with every byte in place, and it is precisely the
//!   state a `write_if_changed` short-circuit leaves untouched forever.
//! - **The hook's own mode**, for the same reason one file up: a `pre-commit`
//!   git will not execute is a gate that never runs, and the "block already
//!   present" report is what made that invisible.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use intentsvcs::canon;

fn home() -> std::path::PathBuf {
  testkit::repo_root()
}

/// The shim as it exists in this install -- the bytes any correct implementation
/// must produce.
fn shim_template() -> String {
  let p = home().join("lib/templates/hooks/pre-commit-shim.sh");
  std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("no shim template at {}: {e}", p.display()))
}

fn mode(path: &Path) -> u32 {
  std::fs::metadata(path)
    .unwrap_or_else(|e| panic!("stat {}: {e}", path.display()))
    .permissions()
    .mode()
}

/// One `apply` against a fixture project, with the hooks directory handed in.
fn apply(fx: &crate::common::Fixture, hooks: &Path) -> canon::Applied {
  let project = fx.project();
  canon::apply(
    fx.root(),
    &home(),
    project.config(),
    &crate::common::ctx(),
    Some(hooks),
    canon::Options::default(),
  )
  .expect("canon apply")
}

fn hooks_dir(fx: &crate::common::Fixture) -> std::path::PathBuf {
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  hooks
}

/// Write `hook`'s chain block under `preamble` into `hooks`, executable, and
/// return its path.
fn write_hook(hooks: &Path, hook: &str, preamble: &str) -> std::path::PathBuf {
  let path = hooks.join(hook);
  std::fs::write(
    &path,
    canon::insert_chain_block(hook, preamble).expect("a hook with no block gets one"),
  )
  .expect("write hook");
  std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
  path
}

/// Run a hook under bash in `cwd`: its exit code and its stderr.
fn run_hook(hook: &Path, cwd: &Path) -> (Option<i32>, String) {
  let out = std::process::Command::new("bash")
    .arg(hook)
    .current_dir(cwd)
    .output()
    .expect("run the hook");
  (
    out.status.code(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
  )
}

/// **THE ARM THAT MAKES EVERY OTHER ARM MEAN SOMETHING.**
///
/// It drives the chain block as a shell program with no carrier beside it. It
/// was the negative control for the fail-open, and asserted the commit PASSED
/// in silence, until issue `0538` gave the block its refusal: the premise
/// changed, as this arm said it would. The commit is now refused, and the
/// refusal names what is missing and the remedy.
///
/// **Deliberately independent of `apply`.** It executes the block canon emits,
/// so it stays true about the block even if `apply` is rewritten.
#[test]
fn the_block_alone_refuses_the_commit_and_says_why() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let hook = write_hook(&hooks, "pre-commit", "");
  assert!(
    !hooks.join("pre-commit.intent").exists(),
    "the arm requires the carrier to be absent"
  );

  let (code, stderr) = run_hook(&hook, fx.root());
  assert_eq!(code, Some(1), "no carrier must refuse the commit: {stderr}");
  assert!(stderr.contains("pre-commit: GATE ABSENT"), "{stderr}");
  assert!(
    stderr.contains("pre-commit.intent"),
    "the missing path is named: {stderr}"
  );
  assert!(
    stderr.contains("remedy: intent claude upgrade --apply"),
    "{stderr}"
  );
}

/// **A CARRIER GIT CANNOT EXECUTE IS REFUSED BY NAME** (issue `0538`): present,
/// so not absent, and no more able to run the gate.
#[test]
fn a_carrier_without_its_execute_bit_is_refused_by_name() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let hook = write_hook(&hooks, "pre-commit", "");
  let carrier = hooks.join("pre-commit.intent");
  std::fs::write(&carrier, "#!/bin/sh\nexit 0\n").expect("plant the carrier");
  std::fs::set_permissions(&carrier, std::fs::Permissions::from_mode(0o644)).expect("chmod 644");

  let (code, stderr) = run_hook(&hook, fx.root());
  assert_eq!(code, Some(1), "{stderr}");
  assert!(
    stderr.contains("pre-commit: GATE NOT EXECUTABLE"),
    "{stderr}"
  );
}

/// **A FAILED HOOKS LOOKUP IS REFUSED TOO, UNDER `set -e`** (issue `0538`).
/// Run outside any repository, `git rev-parse` fails. The block's assignment
/// used to abort a `set -e` hook right there, at rc 128 with nothing printed;
/// it now falls through to the refusal, which speaks.
#[test]
fn a_failed_hooks_lookup_is_refused_under_set_e() {
  let hooks = tempfile::tempdir().expect("hooks");
  let outside = tempfile::tempdir().expect("a directory in no repository");
  let hook = write_hook(
    hooks.path(),
    "pre-commit",
    "#!/usr/bin/env bash\nset -euo pipefail\n",
  );

  let (code, stderr) = run_hook(&hook, outside.path());
  assert_eq!(code, Some(1), "{stderr}");
  assert!(stderr.contains("pre-commit: GATE ABSENT"), "{stderr}");
}

/// **A CARRIER THAT RUNS DECIDES THE COMMIT**, its exit code passed through
/// untouched: the control for the refusals above.
#[test]
fn a_carrier_that_runs_passes_its_code_through() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let hook = write_hook(&hooks, "pre-commit", "");
  let carrier = hooks.join("pre-commit.intent");
  for (body, expected) in [("exit 0", Some(0)), ("exit 3", Some(3))] {
    std::fs::write(&carrier, format!("#!/bin/sh\n{body}\n")).expect("plant the carrier");
    std::fs::set_permissions(&carrier, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let (code, stderr) = run_hook(&hook, fx.root());
    assert_eq!(code, expected, "{body}: {stderr}");
    assert!(!stderr.contains("GATE"), "{body}: {stderr}");
  }
}

/// **A POST-PULL HOOK WITHOUT ITS CARRIER WARNS WHERE THERE IS A STORE, IS
/// SILENT WHERE THERE IS NONE, AND EXITS 0 EITHER WAY** (issue `0538`, vc's
/// ruling of 2026-09-23). Git has already changed the tree when it runs, and a
/// `post-checkout` that exits non-zero becomes the exit code of `git checkout`
/// and of `git worktree add`, so nothing is refused. With no store there is
/// nothing to bring up to date, which is every fresh judging worktree. The
/// third pass is the control: a carrier that is there runs, and nothing warns.
#[test]
fn a_post_pull_hook_warns_only_where_there_is_a_store() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let store = fx.root().join("intent/.cache/intent.db");
  assert!(!store.exists(), "the first pass requires no store");
  for name in canon::POST_PULL_HOOKS {
    let hook = write_hook(&hooks, name, "#!/usr/bin/env bash\nset -euo pipefail\n");
    let (code, stderr) = run_hook(&hook, fx.root());
    assert_eq!(
      (code, stderr.as_str()),
      (Some(0), ""),
      "{name}: no store, so nothing to say"
    );
  }

  std::fs::create_dir_all(store.parent().expect("a parent")).expect("mkdir .cache");
  std::fs::write(&store, b"").expect("a store");
  for name in canon::POST_PULL_HOOKS {
    let (code, stderr) = run_hook(&hooks.join(name), fx.root());
    assert_eq!(code, Some(0), "{name}: {stderr}");
    assert!(
      stderr.contains(&format!(
        "{name}: Intent's store was NOT brought up to date"
      )),
      "{name}: {stderr}"
    );
  }

  for name in canon::POST_PULL_HOOKS {
    let carrier = hooks.join(format!("{name}.intent"));
    std::fs::write(&carrier, "#!/bin/sh\necho carrier-ran >&2\n").expect("plant the carrier");
    std::fs::set_permissions(&carrier, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    let (code, stderr) = run_hook(&hooks.join(name), fx.root());
    assert_eq!(code, Some(0), "{name}: {stderr}");
    assert!(
      stderr.contains("carrier-ran") && !stderr.contains("NOT brought up to date"),
      "{name}: {stderr}"
    );
  }
}

/// The lines of `text` from its chain block's opener through its end marker,
/// each found at the start of a line: prose may name a marker mid-sentence, as
/// `intent/docs/pre-commit-hook.md` does above its manual-install block.
fn block_of(text: &str) -> String {
  let lines: Vec<&str> = text.split_inclusive('\n').collect();
  let start = lines
    .iter()
    .position(|l| l.starts_with("# intent-chain-block:start"))
    .expect("a chain block opener at the start of a line");
  let end = start
    + lines[start..]
      .iter()
      .position(|l| l.trim_end() == "# intent-chain-block:end")
      .expect("a chain block end marker");
  lines[start..=end].concat()
}

/// **INTENT'S OWN HOOKS CARRY CANON'S BLOCK, BYTE FOR BYTE** (issue `0538`).
///
/// `.githooks/pre-commit` carried a refusing form canon never wrote, so the
/// estate that ships the gate protected itself and shipped every other estate
/// the silent block. The copies are legitimate only while this holds them to the
/// one home, and the manual install in `intent/docs/pre-commit-hook.md` is held
/// the same way.
#[test]
fn intents_own_hooks_carry_canons_block() {
  let repo = testkit::repo_root();
  let canon_block = |hook: &str| block_of(&canon::insert_chain_block(hook, "").expect("a block"));
  let mut copies = vec![("pre-commit", repo.join(".githooks/pre-commit"))];
  for name in canon::POST_PULL_HOOKS {
    copies.push((name, repo.join(".githooks").join(name)));
  }
  copies.push(("pre-commit", repo.join("intent/docs/pre-commit-hook.md")));
  for (hook, path) in copies {
    let text =
      std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert_eq!(
      block_of(&text),
      canon_block(hook),
      "{} does not carry canon's {hook} block",
      path.display()
    );
  }
}

/// The carrier lands, executable, and its bytes are the install root's own.
#[test]
fn apply_installs_the_carrier_from_the_resolved_install_root() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);

  apply(&fx, &hooks);

  let carrier = hooks.join("pre-commit.intent");
  assert!(
    carrier.is_file(),
    "the block that sources this file is written by the same function"
  );
  assert_eq!(
    std::fs::read_to_string(&carrier).expect("read carrier"),
    shim_template(),
    "the carrier must be the template from the install root the binary resolved. \
     Eleven estates already carry a gate copied from the frozen v2 tree -- an \
     installer reading from anywhere else reproduces that, correctly and quietly"
  );
  assert!(
    mode(&carrier) & 0o111 != 0,
    "`[ -x ]` is what the chain block tests; mode {:o} makes the gate a no-op",
    mode(&carrier)
  );
}

/// **THE MODE IS REPAIRED EVEN WHEN THE BYTES NEED NO WRITE.**
///
/// This is the arm that fails the moment `make_executable` is moved back inside
/// a `write_if_changed` short-circuit. The state it describes -- correct bytes,
/// mode 644 -- is self-perpetuating under that implementation: every future run
/// reports the carrier already canonical and never touches the bit that makes it
/// run.
#[test]
fn a_correct_carrier_that_is_not_executable_is_repaired() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let carrier = hooks.join("pre-commit.intent");

  std::fs::write(&carrier, shim_template()).expect("plant correct bytes");
  std::fs::set_permissions(&carrier, std::fs::Permissions::from_mode(0o644)).expect("chmod 644");

  let applied = apply(&fx, &hooks);

  assert!(
    applied.unchanged.iter().any(|p| p == &carrier),
    "the bytes matched, so this run must report it already canonical rather than \
     rewriting it: {applied:?}"
  );
  assert!(
    mode(&carrier) & 0o111 != 0,
    "and must still have fixed the mode -- bytes and mode are two properties and \
     only one of them was correct. got {:o}",
    mode(&carrier)
  );
}

/// The same defect one file up: a `pre-commit` already carrying the block, which
/// git will not execute.
///
/// `insert_chain_block` returns `None` here -- "already correct" -- and that
/// report is exactly what kept this state invisible.
#[test]
fn a_hook_already_carrying_the_block_is_made_executable() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let hook = hooks.join("pre-commit");

  std::fs::write(
    &hook,
    canon::insert_chain_block("pre-commit", "").expect("a hook with the block"),
  )
  .expect("write hook");
  std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o644)).expect("chmod 644");

  apply(&fx, &hooks);

  assert!(
    mode(&hook) & 0o111 != 0,
    "a pre-commit git will not execute is a gate that never runs, whatever it \
     contains. got {:o}",
    mode(&hook)
  );
}

/// Run it twice, change nothing -- the property `claude upgrade`'s own doc
/// claims for the whole verb.
#[test]
fn a_second_run_reports_the_carrier_already_canonical() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let carrier = hooks.join("pre-commit.intent");

  let first = apply(&fx, &hooks);
  assert!(
    first.written.iter().any(|p| p == &carrier),
    "the first run writes it: {first:?}"
  );

  let second = apply(&fx, &hooks);
  assert!(
    second.unchanged.iter().any(|p| p == &carrier),
    "the second must report it unchanged, not write it again: {second:?}"
  );
  assert!(
    !second.written.iter().any(|p| p == &carrier),
    "and must not appear as written: {second:?}"
  );
}

/// A project with no repository gets no gate, and that is not a failure.
///
/// The caller passes `None`; canon must not invent a hooks directory, and must
/// not fail on the way past.
#[test]
fn no_repository_means_no_carrier_and_no_error() {
  let fx = crate::common::Fixture::new();
  let project = fx.project();
  let applied = canon::apply(
    fx.root(),
    &home(),
    project.config(),
    &crate::common::ctx(),
    None,
    canon::Options::default(),
  )
  .expect("a project without git is a supported shape, not an error");

  assert!(
    !applied
      .written
      .iter()
      .chain(applied.unchanged.iter())
      .any(|p| p.ends_with("pre-commit.intent")),
    "no repository, no carrier: {applied:?}"
  );
}

/// `--skip-settings` (issue `0143`): a project's own `.claude/settings.json` is
/// left byte-for-byte as it was, reported as skipped, and the rest of canon --
/// the gate included -- is still applied.
///
/// **THE FILE IS PLANTED WITH NON-CANON BYTES, SO THE ARM CAN FAIL.** Against
/// an absent or already-canonical file, "left alone" and "rewritten" would look
/// the same; the control run without the flag shows the same fixture DOES get
/// rewritten.
///
/// **AND THE PLANT CARRIES INTENT'S HOOK DOOR, WHICH IT DID NOT UNTIL v3.0.2.**
/// Canon now HOLDS a `settings.json` that never carried the door -- that is
/// batch 4's fix, and it is the right answer for a file nobody here wrote. But
/// it means a plant WITHOUT the door makes the control run hold rather than
/// rewrite, and the control's only job is to show this fixture does get
/// rewritten when the flag is absent. So the plant is now an Intent-written
/// file the project has edited: the door is present, `mine` is not canon, and
/// "left alone" and "rewritten" stay as distinguishable as they ever were.
#[test]
fn skip_settings_leaves_the_settings_file_alone_and_says_so() {
  let project_settings = r#"{ "hooks": { "Stop": [ { "matcher": "", "hooks": [ { "type": "command", "command": "intent claude hook session-finish" } ] } ] }, "mine": true }"#;
  let run = |skip: bool| {
    let fx = crate::common::Fixture::new();
    fx.git_init();
    let hooks = hooks_dir(&fx);
    let settings = fx.root().join(".claude/settings.json");
    std::fs::create_dir_all(settings.parent().expect("parent")).expect("mkdir .claude");
    std::fs::write(&settings, project_settings).expect("plant settings");
    let project = fx.project();
    let applied = canon::apply(
      fx.root(),
      &home(),
      project.config(),
      &crate::common::ctx(),
      Some(&hooks),
      canon::Options {
        skip_settings: skip,
        ..Default::default()
      },
    )
    .expect("canon apply");
    let after = std::fs::read_to_string(&settings).expect("settings still readable");
    (applied, after, settings, hooks)
  };

  let (applied, after, settings, hooks) = run(true);
  assert_eq!(
    after, project_settings,
    "--skip-settings rewrote the project's settings.json"
  );
  // **MEMBERSHIP, NOT THE WHOLE LIST, BECAUSE THE CLAIM IS ABOUT THIS FILE.**
  // This asserted `skipped == vec![settings]` until 2026-09-12, when
  // `--skip-settings` widened to the harness-wiring CLASS and `.mcp.json`
  // joined the list (AC-24.1, vc's ruling). The test went red on the list's
  // LENGTH, which it was never about -- the same shape as a test pinning the
  // sentence around the claim rather than the claim. `.mcp.json`'s own
  // disposition is asserted in `canon_seeds_the_mcp_declaration_once`, and a
  // second home for it here would be the duplication this estate refuses.
  assert!(
    applied.skipped.contains(&settings),
    "the skip must be reported: {applied:?}"
  );
  assert!(
    !applied
      .written
      .iter()
      .chain(applied.unchanged.iter())
      .any(|p| p == &settings),
    "a skipped file must not also report as written or unchanged: {applied:?}"
  );
  assert!(
    applied
      .written
      .iter()
      .any(|p| p == &hooks.join("pre-commit.intent")),
    "skipping the settings must not skip the rest of canon: {applied:?}"
  );

  let (control, after, settings, _) = run(false);
  assert_ne!(
    after, project_settings,
    "the control run must rewrite the planted file, or the arm above proves nothing"
  );
  assert!(
    control.skipped.is_empty(),
    "nothing is skipped without the flag: {control:?}"
  );
  assert!(
    control.written.iter().any(|p| p == &settings),
    "{control:?}"
  );
}

/// **REPORT MODE ANSWERS WHAT `--apply` WOULD DO, AND WRITES NOTHING** (issue
/// `0115`).
///
/// The dry run used to print canon's roster, identical on a stale estate and a
/// clean one. Three runs pin the replacement: a report on a fresh fixture names
/// what would be written and leaves the tree untouched; `--apply` then writes
/// EXACTLY that set; and a report on the now-canonical tree says nothing would
/// be written -- the answer the roster could never give.
#[test]
fn report_mode_answers_what_apply_would_do_and_writes_nothing() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let project = fx.project();
  let run = |report: bool| {
    canon::apply(
      fx.root(),
      &home(),
      project.config(),
      &crate::common::ctx(),
      Some(&hooks),
      canon::Options {
        report,
        ..Default::default()
      },
    )
    .expect("canon apply")
  };

  let report = run(true);
  assert!(
    !report.written.is_empty(),
    "a fresh fixture has canon to write, so the report must name it: {report:?}"
  );
  for path in &report.written {
    assert!(
      !path.exists(),
      "report mode wrote {} -- a dry run that writes is not a dry run",
      path.display()
    );
  }

  let applied = run(false);
  assert_eq!(
    applied.written, report.written,
    "the report and the write disagree about what changes"
  );

  let clean = run(true);
  assert!(
    clean.written.is_empty(),
    "on a canonical tree the report must say nothing would be written: {clean:?}"
  );
  // The count that matters is that the clean report EXAMINED the tree rather than
  // returning an empty verdict: an empty `written` from a report that looked
  // at nothing would pass the assertion above.
  assert!(
    !clean.unchanged.is_empty(),
    "a clean report must name what it found canonical, not return nothing: {clean:?}"
  );
}

/// **A HOOK WHOSE BLOCK CANON CANNOT REWRITE IS HELD WITH ITS REASON, NEVER
/// REPORTED `unchanged`** (issue `0538`, vc's check). `insert_chain_block`
/// answers `None` for it and for canon's own block alike; only one of them is
/// canonical, and the other keeps a block that may still pass in silence.
#[test]
fn a_hook_canon_cannot_rewrite_is_held_with_its_reason() {
  let fx = crate::common::Fixture::new();
  fx.git_init();
  let hooks = hooks_dir(&fx);
  let hook = hooks.join("pre-commit");
  let block = "# intent-chain-block:start (generated by intent claude upgrade)\n_intent_chain=x\n# intent-chain-block:end\n";
  let doubled = format!("#!/usr/bin/env bash\n{block}{block}");
  std::fs::write(&hook, &doubled).expect("plant a doubled hook");

  let applied = apply(&fx, &hooks);
  assert_eq!(
    applied.blocks_held,
    vec![(hook.clone(), canon::BlockHeld::Doubled)],
    "{applied:?}"
  );
  assert!(
    !applied.unchanged.contains(&hook),
    "a held hook is not canonical: {applied:?}"
  );
  assert_eq!(
    std::fs::read_to_string(&hook).expect("the hook"),
    doubled,
    "its bytes are left as they were"
  );
}
