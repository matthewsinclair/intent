//! **AT-07.2, covering ST0056 AC-07.2 -- `intent claude hook <name>` is
//! byte-compatible with v2 for every shipped hook, and the consumer's
//! `settings.json` is untouched.**
//!
//! **THE POPULATION IS READ FROM THE SHIPPED CANON AND NEVER LISTED HERE, AND
//! THE ONE PLACE IT IS NOT READ FROM IS v3's OWN ROSTER.** v3 keeps a closed
//! list in `install::HOOKS`, deliberately (a name that reaches the filesystem
//! is a name that can contain `../`). That closure is sound and it makes the
//! constant a THIRD independent encoding of a fact already written twice in
//! canon -- so a test that read it would be asking the suspect to describe
//! itself, and would have agreed with the defect this file was written to find.
//! The two sources that DEFINE a shipped hook are the scripts
//! `lib/templates/.claude/scripts/` ships and the commands
//! `lib/templates/.claude/settings.json` wires. A name in either is in the
//! population, and every failure names which source put it there.
//!
//! **WHY A LIVE DIFFERENTIAL AND NOT A GOLDEN FIXTURE.** Three of the four
//! hooks print values read from the environment they run in -- the git branch
//! and short SHA (`session-context.sh:55`), the project directory name
//! (`:53`), the uncommitted-path count (`session-finish.sh:63`). A recorded
//! expectation is wrong the moment HEAD moves, and pinning it would test the
//! fixture rather than the port. **Those fields vary between RUNS and not
//! between SUBJECTS**: drive both binaries against one fixture at one HEAD and
//! whatever they print, they must print the same bytes. That is the property
//! the criterion actually names, and it is measurable without a baseline.
//!
//! **THE FIXTURE IS A DISPOSABLE TREE, NOT THIS CHECKOUT, AND THAT IS A
//! CORRECTNESS REQUIREMENT RATHER THAN TIDINESS.** `session-finish` counts
//! uncommitted paths; run against this repository, two adjacent invocations
//! straddling a peer's commit disagree, and the differential would report a
//! port defect for a board edit. Five nodes share this checkout and ~100
//! commits a day land in it.
//!
//! **`INTENT_HOME` IS UNSET FOR BOTH, AND THE DIFFERENTIAL IS INVALID
//! WITHOUT IT.** v2's door resolves its scripts through `${INTENT_HOME:=<own
//! location>}` (`intent_claude_hook:27`) while v3 resolves from
//! `current_exe()`, ignoring the variable entirely. On a developer machine
//! that variable points at the FROZEN v2 install -- so left alone, v2 reads one
//! tree's scripts and v3 reads another's, and the two agree only because the
//! bodies happen to be byte-identical across the trees today. That is agreement
//! by luck wearing the shape of a measurement. Unset, v2 falls back to its own
//! location and both doors read `<repo>/lib/templates/.claude/scripts/`.
//!
//! The `GIT_*` variables are removed for the same reason one step down:
//! `git -C <dir>` changes the working directory and does NOT override `GIT_DIR`,
//! `GIT_INDEX_FILE` or `GIT_WORK_TREE`, so an inherited one would point both
//! hooks' git calls at a tree that is not the fixture.
//!
//! **A KNOWN DIVERGENCE THAT IS DELIBERATELY NOT ASSERTED HERE.** v2 forwards
//! trailing arguments to the script (`shift; exec bash "$script" "$@"`); v3
//! runs `Command::new("bash").arg(&script)` and drops them. No shipped hook
//! reads a positional argument -- measured, all four -- so the divergence
//! cannot change any shipped hook's output, which puts it outside this
//! criterion's population. It is recorded here rather than tested because the
//! first hook that takes an argument makes it live, and this is where whoever
//! adds one will be reading.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use testkit::repo_root;

/// A fixed session id, so `require-in-session` prints a sentinel path that is a
/// function of this test and not of whoever is running it. The gate only ever
/// READS the sentinel, so a value that matches nothing on disk is the stable
/// choice as well as the safe one.
const SESSION_ID: &str = "hook-compat-fixture-0000";

/// What v3's door prints when it does not recognise a name. Pinned as a live
/// probe by `a_name_the_canon_does_not_ship_is_refused` rather than trusted:
/// a needle that stops matching turns the reachability arm green for every
/// name whether the door serves it or not.
const UNKNOWN: &str = "error: unknown hook:";

struct Run {
  code: Option<i32>,
  stdout: Vec<u8>,
  stderr: Vec<u8>,
}

/// Every hook name the canon ships, mapped to the source that says so.
///
/// Two sources, deliberately unioned rather than intersected. A script in
/// `scripts/` that nothing wires is still a shipped hook -- `post-tool-advisory`
/// is exactly that, documented as "wire by hand; off by default" -- and a
/// command wired in `settings.json` is one a consumer's Claude Code will invoke
/// on a real event whether or not we shipped a script for it. Either alone
/// would have a blind spot the other covers.
fn population(root: &Path) -> BTreeMap<String, Vec<&'static str>> {
  let mut found: BTreeMap<String, Vec<&'static str>> = BTreeMap::new();

  let scripts = root.join("lib/templates/.claude/scripts");
  for entry in std::fs::read_dir(&scripts).expect("read the shipped scripts directory") {
    let path = entry.expect("read a scripts/ entry").path();
    if path.extension().and_then(|e| e.to_str()) == Some("sh") {
      let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("a .sh file has a stem")
        .to_string();
      found
        .entry(name)
        .or_default()
        .push("lib/templates/.claude/scripts/");
    }
  }

  let settings = root.join("lib/templates/.claude/settings.json");
  let text = std::fs::read_to_string(&settings).expect("read the shipped settings.json");
  for name in wired_hook_names(&text) {
    found
      .entry(name)
      .or_default()
      .push("lib/templates/.claude/settings.json");
  }

  // **The fixture proves itself.** A read that found nothing would agree with
  // every claim below, silently, and deriving the population is precisely what
  // makes that failure invisible without this line.
  assert!(
    found.len() >= 3,
    "found {} shipped hook names across scripts/ and settings.json, expected at least 3 -- the canon read is broken, and a broken read passes every \
     assertion in this file vacuously",
    found.len()
  );

  found
}

/// The `<name>` of every `intent claude hook <name>` in a settings.json.
///
/// Textual rather than a JSON parse, and the assertion above is what keeps that
/// honest: the shape being matched is a command line inside a string value, so
/// a parser would buy structure over a field this file does not otherwise read.
fn wired_hook_names(text: &str) -> Vec<String> {
  const NEEDLE: &str = "intent claude hook ";
  let mut names = Vec::new();
  for (idx, _) in text.match_indices(NEEDLE) {
    let rest = &text[idx + NEEDLE.len()..];
    let name: String = rest
      .chars()
      .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
      .collect();
    if !name.is_empty() {
      names.push(name);
    }
  }
  names
}

/// A disposable Intent-shaped project: a git tree with one commit, one
/// uncommitted path, and the canon `settings.json` a consumer receives.
///
/// Every value the hooks print is a function of this tree, so the pair of runs
/// is reproducible and cannot be moved by anything happening in the checkout.
fn fixture() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();

  std::fs::create_dir_all(root.join("intent")).expect("mkdir intent/");
  std::fs::create_dir_all(root.join(".claude")).expect("mkdir .claude/");

  // Matched by `session-context.sh`'s `grep -m1 -E '^\*\*ST[0-9]+'`, so the
  // WIP line is exercised rather than skipped.
  std::fs::write(
    root.join("intent/wip.md"),
    "# wip\n\n**ST0001 -- the fixture thread.**\n",
  )
  .expect("write wip.md");

  let settings = repo_root().join("lib/templates/.claude/settings.json");
  std::fs::copy(&settings, root.join(".claude/settings.json"))
    .expect("seed the consumer settings.json");

  git(root, &["init", "--quiet"]);
  git(root, &["add", "intent/wip.md"]);
  git(root, &["commit", "--quiet", "-m", "fixture"]);
  // One uncommitted path, so `session-finish` takes its counted branch rather
  // than the silent-on-clean one and the count is a fixed 1.
  std::fs::write(root.join("dirty.txt"), "uncommitted\n").expect("write the dirty path");

  dir
}

/// git, with every ambient input the fixture must not inherit turned off.
///
/// `-c` rather than a config write: the branch name, the identity and the
/// signing setting are the operator's on this machine and all three would
/// otherwise reach the fixture.
fn git(root: &Path, args: &[&str]) {
  let out = Command::new("git")
    .args(["-c", "init.defaultBranch=main"])
    .args(["-c", "user.name=hook-compat"])
    .args(["-c", "user.email=hook-compat@invalid"])
    .args(["-c", "commit.gpgsign=false"])
    .args(args)
    .current_dir(root)
    .env_remove("GIT_DIR")
    .env_remove("GIT_INDEX_FILE")
    .env_remove("GIT_WORK_TREE")
    .env_remove("GIT_OBJECT_DIRECTORY")
    .env_remove("GIT_COMMON_DIR")
    .output()
    .expect("run git for the fixture");
  assert!(
    out.status.success(),
    "fixture git {args:?} failed: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// Run one door against the fixture, with every ambient input pinned.
fn run_door(bin: &Path, name: &str, fx: &Path, stdin: &[u8]) -> Run {
  let mut child = Command::new(bin)
    .args(["claude", "hook", name])
    .current_dir(fx)
    .env("CLAUDE_PROJECT_DIR", fx)
    .env("CLAUDE_CODE_SESSION_ID", SESSION_ID)
    .env_remove("INTENT_HOME")
    .env_remove("INTENT_SKIP_IN_SESSION_GATE")
    .env_remove("GIT_DIR")
    .env_remove("GIT_INDEX_FILE")
    .env_remove("GIT_WORK_TREE")
    .env_remove("GIT_OBJECT_DIRECTORY")
    .env_remove("GIT_COMMON_DIR")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap_or_else(|e| panic!("spawn {}: {e}", bin.display()));

  child
    .stdin
    .as_mut()
    .expect("the child's stdin is piped")
    .write_all(stdin)
    .expect("write the event payload");
  // Dropped so the child sees EOF: every hook reads stdin to completion and
  // one holding an open pipe would hang instead of failing.
  drop(child.stdin.take());

  let out = child.wait_with_output().expect("wait for the door");
  Run {
    code: out.status.code(),
    stdout: out.stdout,
    stderr: out.stderr,
  }
}

fn v3() -> PathBuf {
  PathBuf::from(env!("CARGO_BIN_EXE_intent"))
}

/// v2's shell CLI, tracked in THIS repository (not the frozen install).
fn v2() -> Option<PathBuf> {
  let p = repo_root().join("bin/intent");
  p.is_file().then_some(p)
}

/// **Every hook the canon ships must be reachable through v3's door.**
///
/// The assertion is that the door did not refuse the NAME. What the script then
/// does is the differential's business; this arm is about whether v3 serves the
/// hook at all, which is the failure a consumer meets first and the one that
/// cannot be worked around from inside a session.
#[test]
fn every_hook_the_canon_ships_is_reachable_through_the_door() {
  let root = repo_root();
  let fx = fixture();

  let mut refused = Vec::new();
  for (name, sources) in population(&root) {
    let run = run_door(&v3(), &name, fx.path(), b"");
    let stderr = String::from_utf8_lossy(&run.stderr).to_string();
    if stderr.contains(UNKNOWN) {
      refused.push(format!(
        "`intent claude hook {name}` -> exit {:?}: {}\n      shipped by: {}",
        run.code,
        stderr.trim(),
        sources.join(", ")
      ));
    }
  }

  assert!(
    refused.is_empty(),
    "v3's door refuses the name of a hook this canon ships:\n  {}\n\nAC-07.2 says byte-compatible \"for every shipped hook\", and a name the door \
     does not serve has no output to compare. A hook wired in settings.json is invoked by a consumer's Claude Code on a real event: v2 runs it, v3 answers \
     `unknown hook`, and the consumer sees the refusal on every occurrence of that event. This is issue 0043's shape one hook over -- there the command was \
     unimplemented, here the command is implemented and its roster is short -- and it is invisible to `session_hook_lockout.rs`, whose needle is the \
     unimplemented marker, which a wrong-name refusal does not print.",
    refused.join("\n  ")
  );
}

/// **The refusal is real, so the arm above can fail.**
///
/// Without this, `every_hook_the_canon_ships_is_reachable_through_the_door`
/// asserts an ABSENCE over a needle nothing proves is still spoken -- 0 of 0
/// wearing the shape of a green. A name the canon does not ship must be refused,
/// and refused in the marker that arm reads.
#[test]
fn a_name_the_canon_does_not_ship_is_refused() {
  let root = repo_root();
  let fx = fixture();
  let bogus = "not-a-shipped-hook";
  assert!(
    !population(&root).contains_key(bogus),
    "the control name `{bogus}` has become a real shipped hook; pick another"
  );

  let run = run_door(&v3(), bogus, fx.path(), b"");
  let stderr = String::from_utf8_lossy(&run.stderr).to_string();
  assert!(
    stderr.contains(UNKNOWN),
    "the door served an unshipped name, or reworded its refusal. Either way the reachability arm above is now vacuous. exit {:?}, stderr: {stderr}",
    run.code
  );
  // Not 2. A typo in an operator's settings file must not be answered in the
  // code `UserPromptSubmit` reads as BLOCK -- the same boundary
  // `session_hook_lockout.rs` holds, asserted here because this file drives the
  // door over a wider set of names.
  assert_ne!(
    run.code,
    Some(2),
    "the door answered an unknown name in the caller's refusal code; a settings typo would block every prompt. stderr: {stderr}"
  );
}

/// **The live differential: both binaries, one fixture, one HEAD, byte for
/// byte.**
///
/// This is AC-07.2's own sentence, and it is the only arm that can catch a
/// misremembered format, because it never states what the output should be.
#[test]
fn the_two_binaries_agree_byte_for_byte_on_every_shipped_hook() {
  let Some(v2) = v2() else {
    eprintln!("SKIPPED the live differential: bin/intent is absent (post-cutover tree?)");
    return;
  };
  let root = repo_root();
  let fx = fixture();

  let mut diverged = Vec::new();
  for (name, _) in population(&root) {
    let a = run_door(&v2, &name, fx.path(), b"");
    let b = run_door(&v3(), &name, fx.path(), b"");
    if a.code != b.code || a.stdout != b.stdout || a.stderr != b.stderr {
      diverged.push(format!(
        "{name}:\n      v2 exit {:?} stdout {:?} stderr {:?}\n      v3 exit {:?} stdout {:?} stderr {:?}",
        a.code,
        String::from_utf8_lossy(&a.stdout),
        String::from_utf8_lossy(&a.stderr),
        b.code,
        String::from_utf8_lossy(&b.stdout),
        String::from_utf8_lossy(&b.stderr),
      ));
    }
  }

  assert!(
    diverged.is_empty(),
    "the two doors disagree on a shipped hook:\n  {}\n\nBoth are thin: each resolves a name and `exec`s `bash <script>`, and with INTENT_HOME unset both \
     resolve to <repo>/lib/templates/.claude/scripts/. So a divergence here is the DOOR's, never the hook's -- the same script produced both columns, or one \
     door never reached it.",
    diverged.join("\n  ")
  );
}

/// **The gate's block code is the script's own and reaches the caller
/// untranslated.**
///
/// Issue 0043's core property, asserted positively rather than as "not 2".
/// `require-in-session` exits 2 when the sentinel is absent, which is the gate
/// working; a door that wrapped the script instead of replacing it would have
/// to reproduce that number deliberately, and the shape that produced 0043 was
/// a wrapper that merely intended to pass it through.
#[test]
fn the_gates_block_code_is_the_scripts_own_and_reaches_the_caller() {
  let fx = fixture();
  let run = run_door(&v3(), "require-in-session", fx.path(), b"");
  assert_eq!(
    run.code,
    Some(2),
    "the gate did not block with 2 on an absent sentinel. Either the door translated the script's exit code, or the sentinel for `{SESSION_ID}` exists on \
     this machine. stderr: {}",
    String::from_utf8_lossy(&run.stderr)
  );
  assert!(
    String::from_utf8_lossy(&run.stderr).contains("Expected sentinel:"),
    "the gate blocked without naming the sentinel path; the documented escape from a blocked session is to touch the path it prints. stderr: {}",
    String::from_utf8_lossy(&run.stderr)
  );
}

/// **stdin reaches the script unread, proven by the exit code moving with the
/// bytes.**
///
/// `require-in-session` passes a prompt beginning with `/` (a slash command,
/// which is how `/in-session` itself gets through the gate it releases). So the
/// same hook, same environment, same sentinel state answers 2 for an empty
/// payload and 0 for that one. A door that consumed, buffered or re-encoded
/// stdin cannot produce both.
#[test]
fn stdin_reaches_the_script_unread() {
  if Command::new("jq").arg("--version").output().is_err() {
    eprintln!(
      "SKIPPED the stdin fidelity probe: jq is absent, and the gate parses the payload with it"
    );
    return;
  }
  let fx = fixture();

  let empty = run_door(&v3(), "require-in-session", fx.path(), b"");
  let slash = run_door(
    &v3(),
    "require-in-session",
    fx.path(),
    br#"{"prompt":"/in-session"}"#,
  );

  assert_eq!(
    empty.code,
    Some(2),
    "the control half of the probe did not block; with no payload and no sentinel the gate must refuse. stderr: {}",
    String::from_utf8_lossy(&empty.stderr)
  );
  assert_eq!(
    slash.code,
    Some(0),
    "a slash-command payload did not reach the script: the gate blocked a prompt it must pass. Either the door did not forward stdin, or it forwarded \
     something other than the bytes it was given. stderr: {}",
    String::from_utf8_lossy(&slash.stderr)
  );
}

/// **AC-07.2's second limb: the consumer's `settings.json` is untouched.**
///
/// Running a hook must never rewrite the file that named it. Asserted over the
/// byte content rather than the mtime, because a rewrite that happened to
/// reproduce the bytes is not a defect and a touch that changed nothing is not
/// the harm the criterion names.
///
/// **THE LIVENESS CHECK IS THE POINT OF THIS ARM, NOT DECORATION.** An
/// untouched-file assertion is satisfied perfectly by a loop in which nothing
/// ran -- and that is not hypothetical here, because a door refusing every name
/// is the exact defect this file was written to find. Without the check below,
/// the very failure the first arm catches would make THIS arm green.
///
/// **A RESIDUAL, STATED RATHER THAN LEFT TO BE INFERRED FROM A GREEN.** This
/// proves the hooks do not rewrite the file; it does not prove the assertion
/// could catch one that did. The positive control for that would mean making a
/// shipped hook destructive, and the shipped hooks are the ones this
/// repository's own Claude Code sessions execute on every event -- so the
/// control is more dangerous than the class it would police. What IS proven is
/// that the loop did real work.
#[test]
fn running_every_hook_leaves_the_consumers_settings_untouched() {
  let root = repo_root();
  let fx = fixture();
  let settings = fx.path().join(".claude/settings.json");
  let before = std::fs::read(&settings).expect("read the seeded settings.json");

  let mut ran = 0;
  for (name, _) in population(&root) {
    let run = run_door(&v3(), &name, fx.path(), b"");
    // A script actually executed: the door reached it and it returned its own
    // status. A refused NAME exits 1 with no script ever running.
    if !String::from_utf8_lossy(&run.stderr).contains(UNKNOWN) {
      ran += 1;
    }
  }
  // `> 0`, not `== population.len()`, deliberately. The hole being closed is
  // "nothing ran"; WHICH hooks are reachable is the first arm's contract, and
  // asserting it twice would make one defect fail two arms without either
  // failure saying anything the other did not. A magic number here would also
  // track the roster's size in silence, which is the drift this file exists to
  // catch.
  assert!(
    ran > 0,
    "not one shipped hook reached a script, so `untouched` is a claim about a loop that did nothing. See `every_hook_the_canon_ships_is_reachable_through_the_door`, which is the arm that says why."
  );

  let after = std::fs::read(&settings).expect("read the settings.json after the hooks ran");
  assert_eq!(
    before, after,
    "running the shipped hooks rewrote the consumer's .claude/settings.json. 0016 forbids the tool rewiring a consumer's hooks, and a hook that edits the \
     file naming it can change what runs next session without anyone asking."
  );
}
