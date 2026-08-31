//! **Issue 0043's canary: no command the session hooks invoke may answer "not
//! implemented".**
//!
//! `.claude/settings.json` wires `UserPromptSubmit` to `intent claude hook
//! require-in-session` on EVERY prompt, and Claude Code reads exit 2 as BLOCK.
//! When 0038 moved unimplemented commands from 1 to 2 -- correctly, for the
//! pre-commit gate it was measured against -- an unimplemented `claude hook`
//! began answering in the code that means "refuse this prompt". **A migrated
//! project would have refused every prompt, and the refusal could not be
//! cleared from inside the session**: the documented escapes are to run
//! `/in-session` (which needs a prompt) or to touch the sentinel path the gate
//! prints (which it no longer printed).
//!
//! **The roster is READ FROM THE SHIPPED CANON, never listed here.** A
//! hardcoded pair would have covered exactly the two hooks that existed when
//! this was written, which is the same shape as the comment beside
//! `EXIT_UNAVAILABLE` naming exactly the one consumer its author had in view.
//! Wire a third hook into `settings.json` and it is covered on the next run,
//! by nobody's decision.
//!
//! **The assertion is on the MESSAGE, not on the exit code, and that is not a
//! weakening.** Exit 2 out of this path is AMBIGUOUS BY DESIGN: it is what
//! `require-in-session.sh` returns when it deliberately blocks a prompt whose
//! sentinel is absent, which is the gate working. A test asserting "not 2"
//! would have to break the gate to pass. The unimplemented marker distinguishes
//! the two exactly -- one is Intent refusing on the operator's behalf, the
//! other is Intent unable to answer at all -- and `the_marker_is_the_message_an
//! _unimplemented_command_actually_prints` keeps the needle honest against a
//! command that really is unwired.

use std::path::Path;
use std::process::{Command, Stdio};

use testkit::workspace_root;

/// What `render::unwired` prints. Pinned by a live probe below rather than
/// trusted, because a needle that no longer matches turns this whole file
/// green and silent -- the exact failure mode it exists to catch.
const UNIMPLEMENTED: &str = "is a known command that is not implemented yet";

/// The Intent install root: the repository, which is what the shipped
/// `settings.json` and the shipped hook scripts both live under.
fn install_root() -> std::path::PathBuf {
  // `native/rust` -> the repository root.
  workspace_root()
    .parent()
    .and_then(Path::parent)
    .expect("the rust workspace sits two levels under the Intent install")
    .to_path_buf()
}

/// Every `intent ...` invocation the shipped session hooks make, as argv.
///
/// Read out of `lib/templates/.claude/settings.json` -- the file a consumer
/// project receives verbatim -- so this measures the canon rather than a
/// description of it. Non-`intent` commands (the `Stop` reminder is a bare
/// `echo`) are not this file's business and are skipped.
fn hook_invocations(root: &Path) -> Vec<Vec<String>> {
  let path = root.join("lib/templates/.claude/settings.json");
  let text = std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("read the shipped settings at {}: {e}", path.display()));
  let json: serde_json::Value =
    serde_json::from_str(&text).expect("the shipped settings.json is valid JSON");

  let mut out = Vec::new();
  let events = json
    .get("hooks")
    .and_then(|h| h.as_object())
    .expect("settings.json declares a `hooks` object");
  for (_event, matchers) in events {
    for matcher in matchers.as_array().into_iter().flatten() {
      for hook in matcher
        .get("hooks")
        .and_then(|h| h.as_array())
        .into_iter()
        .flatten()
      {
        let Some(command) = hook.get("command").and_then(|c| c.as_str()) else {
          continue;
        };
        let argv: Vec<String> = command.split_whitespace().map(str::to_string).collect();
        if argv.first().map(String::as_str) == Some("intent") {
          out.push(argv[1..].to_vec());
        }
      }
    }
  }
  out
}

/// Run the v3 binary with `args` and return (code, stderr).
///
/// stdin is an empty pipe that is closed immediately: these hooks read Claude
/// Code's event JSON from it, and a hook inheriting the test harness's stdin
/// would hang rather than fail.
fn run(args: &[String], cwd: &Path) -> (Option<i32>, String) {
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

#[test]
fn every_command_the_session_hooks_invoke_is_implemented() {
  let root = install_root();
  let invocations = hook_invocations(&root);

  // The fixture proves itself. A parse that found nothing would agree with
  // every claim this file makes, silently -- and the roster being derived is
  // precisely what makes that failure invisible without this line.
  assert!(
    invocations.len() >= 2,
    "parsed {} intent invocations out of the shipped settings.json, expected at least the SessionStart and UserPromptSubmit hooks -- the parse is broken, and a \
     broken parse passes this test vacuously",
    invocations.len()
  );

  let mut locked_out = Vec::new();
  for argv in &invocations {
    let (code, stderr) = run(argv, &root);
    if stderr.contains(UNIMPLEMENTED) {
      locked_out.push(format!(
        "`intent {}` -> exit {:?}: {}",
        argv.join(" "),
        code,
        stderr.trim()
      ));
    }
  }

  assert!(
    locked_out.is_empty(),
    "a command wired into the shipped session hooks is not implemented in this binary:\n  {}\n\nISSUE 0043: `UserPromptSubmit` reads exit 2 as BLOCK, so an \
     unimplemented hook command refuses every prompt in a migrated project -- and the refusal cannot be cleared from inside the session, because clearing it \
     means submitting a prompt. Implement the command; do not change the exit code, which is correct for the pre-commit gate that reads the same number as \
     fail-open.",
    locked_out.join("\n  ")
  );
}

/// **The needle is real.** Everything above rests on one string matching what
/// `unwired` prints; if that message is reworded, the test above passes for
/// every command whether implemented or not.
///
/// The specimen is any command that is declared in the dispatch table and
/// unwired -- the same condition `claude hook` was in when 0043 was filed.
/// It was `mcp` until WP-09 built the stdio server (2026-08-30), at which
/// point this test went red exactly as its message instructs and the specimen
/// moved to `st bootstrap`. When THAT wires, the red lands on whoever wired
/// it, and the message below says what to do; if the estate ever runs out of
/// unwired declared commands entirely, the marker's realness needs a
/// different proof than driving one -- read `retirement_is_enumerable`'s
/// emptied COMING-SOON arm for the shape that took.
#[test]
fn the_marker_is_the_message_an_unimplemented_command_actually_prints() {
  let root = install_root();
  let (code, stderr) = run(&["st".to_string(), "bootstrap".to_string()], &root);
  assert!(
    stderr.contains(UNIMPLEMENTED),
    "`intent st bootstrap` is a declared, unwired command and did not print the marker this file matches on. Either it has been implemented -- pick another \
     unwired command -- or the message was reworded and every assertion in this file is now vacuous. exit {code:?}, stderr: {stderr}"
  );
}

/// **The hook actually RUNS, rather than merely parsing** -- and its own exit
/// code reaches the caller.
///
/// The distinction 0043 turns on is that exit 2 from the SCRIPT is correct and
/// exit 2 from the unimplemented COMMAND is a lockout. This drives the script
/// to its blocking branch on purpose, with a session id that has no sentinel,
/// and requires both that the code is 2 and that the message is the gate's.
/// A wrapper that swallowed the child's status would show up here as a 0.
#[test]
fn the_gates_own_block_still_reaches_the_caller_through_the_v3_binary() {
  let root = install_root();
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["claude", "hook", "require-in-session"])
    .current_dir(&root)
    // A session whose sentinel cannot exist: the gate must choose to block.
    .env("CLAUDE_CODE_SESSION_ID", "0043-canary-no-such-session")
    .stdin(Stdio::null())
    .output()
    .expect("run the hook through the v3 binary");
  let stderr = String::from_utf8_lossy(&out.stderr).to_string();

  assert_eq!(
    out.status.code(),
    Some(2),
    "the gate's deliberate block must reach the caller as 2 -- Claude Code's BLOCK code -- rather than being translated by the wrapper. stderr: {stderr}"
  );
  assert!(
    !stderr.contains(UNIMPLEMENTED),
    "the 2 came from the unimplemented command rather than from the gate, which is the lockout itself: {stderr}"
  );
  assert!(
    stderr.contains("Expected sentinel:"),
    "the gate must still print the sentinel path -- it is the only escape from a blocked session, and 0043's lockout was self-sealing precisely because this \
     line was replaced by the not-implemented message: {stderr}"
  );
}

/// **The hook door DELEGATES: every `2` a Claude Code hook sees came from the
/// script, never from the CLI.**
///
/// This is the per-caller answer to the four-contracts problem, stated as a
/// mechanism rather than as a choice of constant. `2` means four different
/// things to four consumers (fail-open, block, advisory, refuse-to-stop), so no
/// global value is right; what makes the Claude Code side safe is that
/// `claude hook` is the single door those consumers reach the binary through
/// and it hands the decision to the script.
///
/// Driven over the failure modes the CLI can reach BEFORE any script runs --
/// an unknown hook name, and a missing name at all. Both must answer 1, which
/// vc's ARM1 measured as non-blocking; a `2` here would refuse the prompt on
/// the operator's behalf over a typo in their settings file.
#[test]
fn the_hook_door_never_answers_in_the_callers_refusal_code() {
  let root = install_root();
  for argv in [
    vec!["claude".to_string(), "hook".to_string(), "nope".to_string()],
    vec!["claude".to_string(), "hook".to_string()],
  ] {
    let (code, stderr) = run(&argv, &root);
    assert_ne!(
      code,
      Some(2),
      "`intent {}` answered in the code UserPromptSubmit reads as BLOCK. Every failure the CLI itself can reach on this path must be 1 -- measured as \
       non-blocking -- so that a 2 reaching a hook consumer is always the script's own deliberate refusal. stderr: {stderr}",
      argv.join(" ")
    );
    assert!(
      !stderr.contains(UNIMPLEMENTED),
      "`intent {}` reported itself unimplemented: {stderr}",
      argv.join(" ")
    );
  }
}

/// **Issue 0042: the pre-commit gate's own resolution expression, run against
/// this binary.**
///
/// The gate does not read `intent info`'s exit code at all -- it parses
/// `INTENT_HOME:` out of stdout with `sed` and builds the whiteboard guards'
/// paths from the result. With `info` unimplemented the value was empty, both
/// guards took the not-found branch, and timestamp and header enforcement
/// stopped. The `sed` expression here is copied from `pre-commit.sh` rather
/// than paraphrased, because a paraphrase of the consumer proves nothing about
/// the consumer.
#[test]
fn info_resolves_the_path_the_pre_commit_gate_builds_its_guards_from() {
  let root = install_root();
  let out = Command::new("sh")
    .arg("-c")
    .arg(format!(
      "{} info 2>/dev/null | sed -n 's/^ *INTENT_HOME: *//p' | head -1",
      env!("CARGO_BIN_EXE_intent")
    ))
    .current_dir(&root)
    .output()
    .expect("run the gate's resolution expression");
  let resolved = String::from_utf8_lossy(&out.stdout).trim().to_string();

  assert!(
    !resolved.is_empty(),
    "`intent info` produced no INTENT_HOME line for the gate to parse, so both whiteboard guards fail open (issue 0042)"
  );

  // The guards the gate actually builds, named as it names them.
  for guard in ["whiteboard-clock-guard.sh", "whiteboard-header-guard.sh"] {
    let path = Path::new(&resolved).join("lib/templates/hooks").join(guard);
    assert!(
      path.is_file(),
      "the gate would look for {guard} at {} and not find it, so it would announce a fail-open and commit unchecked",
      path.display()
    );
  }
}
