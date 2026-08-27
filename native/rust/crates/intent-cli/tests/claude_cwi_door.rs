//! `intent claude start` and `intent claude ws` reach the MAAC launcher.
//!
//! **The defect these arms exist for was reachability, not a missing feature.**
//! `intent/plugins/claude/bin/intent_claude_cwi` implements both verbs and was
//! measured working standalone under a v3 binary; the v3 `claude()` dispatch
//! simply had no arm for either, so both answered `2` -- "a known command that
//! is not implemented yet" -- while `intent claude --help` listed them and
//! `dispatch-table.json` filed `claude start` under `shipped`. hv was blocked
//! on `claude start` for as long as that gap stood.
//!
//! # Driving these verbs is a SAFETY question, and both hazards have a seam
//!
//! `claude start` is a declared member of `populations.not_probed`, whose
//! recorded reason is that **invoked bare it launches a real Claude Code
//! session**. `flag_reachability.rs` warns in as many words that this harness
//! family was once safe by accident of scope and that widening it is a one-line
//! change which inherits the whole hazard in silence. So the exclusion is
//! honoured rather than argued with:
//!
//! - **`CWI_DRY_RUN=1`** -- the script's own documented seam. It prints the
//!   `claude` argv it WOULD exec and exits, so no session is ever launched.
//!   Every arm below that names `start` sets it.
//! - **`CWI_WB=<tempdir>`** -- redirects the whiteboard root, so the writing
//!   verbs (`ws new`) provision into a scratch directory and the repository's
//!   real board is never touched.
//!
//! **An arm here that forgets either is not a slow test, it is a live session
//! or a written board**, which is why they are named at the top rather than
//! left as an incidental env var at the call site.

use std::path::{Path, PathBuf};
use std::process::Command;

/// The launcher, as it sits in the repository this test runs from.
fn real_script() -> PathBuf {
  testkit::repo_root().join("intent/plugins/claude/bin/intent_claude_cwi")
}

/// A tree shaped like an Intent install, holding a real binary and the real
/// launcher.
///
/// **EVERY arm runs through this, and the first draft of this file did not --
/// which is how it proved the point.** [`intentsvcs::install::home`] walks up
/// from `current_exe()` to the `lib/templates` marker, so whether
/// `CARGO_BIN_EXE_intent` resolves to a real install is a fact about where
/// `CARGO_TARGET_DIR` happens to point. Under the workspace default it sits
/// inside the repository and the walk finds the real install; under a private
/// target directory -- which is the ordinary way to build in this shared tree
/// -- it does not, and three arms failed with `cannot locate the Intent
/// install`.
///
/// **A test whose subject depends on the target directory is measuring
/// something other than the code.** Copying the binary into a tree this file
/// controls makes the install a property of the fixture rather than of the
/// caller's environment.
fn fixture_install() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  std::fs::create_dir_all(root.join("lib/templates")).expect("marker");
  std::fs::create_dir_all(root.join("bin")).expect("bin");
  std::fs::create_dir_all(root.join("intent/plugins/claude/bin")).expect("plugin bin");
  std::fs::copy(env!("CARGO_BIN_EXE_intent"), root.join("bin/intent")).expect("copy binary");
  std::fs::copy(
    real_script(),
    root.join("intent/plugins/claude/bin/intent_claude_cwi"),
  )
  .expect("copy launcher");
  dir
}

/// Run a command from the repository root, with both hazards sealed.
fn run(exe: &Path, args: &[&str], wb: &Path) -> (String, i32) {
  let out = Command::new(exe)
    .args(args)
    .current_dir(testkit::repo_root())
    .env("HOME", testkit::fixture_home())
    .env("CWI_DRY_RUN", "1")
    .env("CWI_WB", wb)
    .output()
    .expect("run the binary");
  let text = format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  (text, out.status.code().unwrap_or(-1))
}

/// Provision a node in the scratch board.
///
/// `start` on a node that does not exist PROMPTS to create it, reads EOF from a
/// test's closed stdin, and aborts at 1 -- correct behaviour, and it fails an
/// arm that meant to exercise the door rather than the absent-node path.
fn seed_node(wb: &Path, node: &str) {
  std::fs::create_dir_all(wb.join(node)).expect("node dir");
  std::fs::write(
    wb.join(node).join("wip.md"),
    format!(
      "---\nnode: {node}\nname: {node}\nrole: worker\nsession_id: none\n\
       heartbeat_at: 2026-01-01 00:00Z\nstatus: active\nfocus: \"a fixture node\"\n\
       claims: []\n---\n# {node}\n"
    ),
  )
  .expect("write board");
}

const UNWIRED: &str = "is a known command that is not implemented yet";

/// The regression this file exists for, stated as the symptom hv reported.
#[test]
fn neither_verb_answers_the_unwired_refusal() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let wb = tempfile::tempdir().expect("tempdir");
  seed_node(wb.path(), "cc");
  for args in [vec!["claude", "start", "cc"], vec!["claude", "ws", "list"]] {
    let (text, code) = run(&exe, &args, wb.path());
    assert!(
      !text.contains(UNWIRED),
      "`intent {}` still answers the unwired refusal: {text}",
      args.join(" ")
    );
    // **`assert_ne!(code, 2)` ALONE PASSES FOR THE WRONG REASON, and it did.**
    // Before every arm used the fixture, these two ran against a binary whose
    // `home()` could not resolve: it exited 1 with an install error, which is
    // neither the unwired phrase nor a 2, so the arm went green while the door
    // was never reached. A success assertion is what makes it about the door.
    assert_eq!(
      code,
      0,
      "`intent {}` did not succeed: {text}",
      args.join(" ")
    );
  }
}

/// `start` reaches the launcher and the launcher gets as far as composing the
/// session it would open.
///
/// Asserting the DRY-RUN text rather than merely a zero exit: a door wired to
/// the wrong script, or one that swallowed its argument, could also exit 0.
#[test]
fn start_reaches_the_launcher_and_composes_a_session() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let wb = tempfile::tempdir().expect("tempdir");
  seed_node(wb.path(), "cc");
  let (text, code) = run(&exe, &["claude", "start", "cc"], wb.path());
  assert_eq!(
    code, 0,
    "start should succeed under the dry-run seam: {text}"
  );
  assert!(
    text.contains("DRY RUN") && text.contains("--append-system-prompt"),
    "start did not reach the launcher's dry-run path: {text}"
  );
}

/// **The verb is PASSED THROUGH, not consumed.**
///
/// v2's `bin/intent` carried this as a comment -- *"Do NOT shift:
/// intent_claude_cwi's own dispatch consumes `start`/`ws`"* -- and it is
/// exactly the sort of line a reader tidies away. If the door shifted the verb
/// off, the launcher would receive `list` as its command, fall to its `*)`
/// case, and print usage. So this asserts board content, which only a correctly
/// dispatched `ws list` can produce.
#[test]
fn the_verb_reaches_the_launcher_rather_than_being_eaten() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let wb = tempfile::tempdir().expect("tempdir");
  seed_node(wb.path(), "qq");

  let (text, code) = run(&exe, &["claude", "ws", "list"], wb.path());
  assert_eq!(code, 0, "ws list should succeed: {text}");
  assert!(
    text.contains("qq"),
    "ws list did not read the fixture board -- the verb was probably consumed: {text}"
  );
  assert!(
    !text.contains("Usage:") && !text.contains("usage:"),
    "the launcher printed usage, which is what it does when handed a verb it does not know: {text}"
  );
}

/// The OPTIONAL positional reaches the launcher.
///
/// `wsid` is arity `0..1` in the dispatch table: `ws list` takes none and
/// `ws new` takes one. A door that read it with the required-positional
/// accessor would refuse `ws list`; one that never forwarded it would turn
/// `ws new qq` into a bare `ws new`. This drives the half that carries a value.
#[test]
fn the_optional_wsid_reaches_the_launcher() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let wb = tempfile::tempdir().expect("tempdir");
  let (text, code) = run(&exe, &["claude", "ws", "new", "zz"], wb.path());
  assert_eq!(code, 0, "ws new should succeed: {text}");
  assert!(
    wb.path().join("zz/wip.md").is_file(),
    "the wsid never reached the launcher -- no node was provisioned: {text}"
  );
}

/// An install missing its launcher refuses BY NAME rather than as an opaque
/// `127` from the shell.
///
/// **This is the arm the shared `exec_shipped_script` helper exists for.** The
/// `is_file` check is the only thing between an incomplete install and a bare
/// `127`, and with two doors now depending on it, one copy is what stops the
/// next door shipping without it. The refusal must also say WHICH subsystem is
/// incomplete: `hook script` and `whiteboard launcher` are different repairs.
#[test]
fn an_absent_launcher_refuses_by_name() {
  let install = fixture_install();
  let exe = install.path().join("bin/intent");
  let launcher = install
    .path()
    .join("intent/plugins/claude/bin/intent_claude_cwi");
  let wb = tempfile::tempdir().expect("tempdir");

  // Positive control FIRST: with the launcher present this same invocation
  // succeeds. Without it, a red arm below would be consistent with the fixture
  // simply being broken.
  let (before, code) = run(&exe, &["claude", "ws", "list"], wb.path());
  assert_eq!(code, 0, "the fixture install itself is broken: {before}");

  std::fs::remove_file(&launcher).expect("remove the launcher");
  let (text, code) = run(&exe, &["claude", "ws", "list"], wb.path());
  assert_ne!(code, 0, "an absent launcher must not succeed: {text}");
  assert!(
    text.contains("whiteboard launcher not found"),
    "the refusal does not name what is missing: {text}"
  );
  assert!(
    text.contains("intent_claude_cwi"),
    "the refusal does not name the path the operator must restore: {text}"
  );
}
