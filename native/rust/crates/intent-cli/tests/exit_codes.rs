//! INV-02 / AC-05.2: usage errors exit **1**, not clap's default 2.
//!
//! **This file was written BEFORE the clap spine existed**, because ic's
//! dispatch table asks for exactly that and the reason is worth restating: v2
//! exits 1 on every usage error (`error()` at `bin/intent_helpers:7-11` is
//! `echo >&2; exit 1`, and it is the only failure exit in the shipped surface
//! bar `intent critic`), while clap exits 2 for both
//! `ErrorKind::MissingRequiredArgument` and `ErrorKind::UnknownArgument`.
//!
//! D17 carries v2's codes over, so the override is a framework-layer decision
//! made once at the start. Pinned here so that a clap major bump, or someone
//! removing the override, reds ONE named invariant -- instead of a hundred BATS
//! conformance tests failing for a reason nobody traces back to this decision.
//!
//! The exception is `intent critic`, which genuinely uses 2 (INV-04); asserting
//! that here too is what stops a blanket "always exit 1" from looking correct.
//!
//! **That last sentence was true and the test under it was vacuous, which is
//! how issue 0038 shipped.** v3 gave every failure `EXIT_ERROR`, so a command
//! this build has not wired reported itself in the code that means "your code
//! is bad" -- and the shipped pre-commit gate, whose `2+` branch exists for
//! exactly this, never reached it and blocked every commit in a migrated
//! project. The three tests at the foot of this file are the contract v2
//! actually has, measured rather than inferred, and the last one drives the
//! consumer instead of the number.
//!
//! # AT-10.9
//!
//! **The id lives here because the citation is checkable from both ends** -- the
//! contract row names this FILE, and a file that does not name the row back
//! leaves the link provable in one direction only. vc recorded the second end
//! as owed; this is it.
//!
//! Both arms of AC-10.9 ("a project can still COMMIT with v3 installed --
//! MIGRATED OR NOT") are driven END TO END through the shipped
//! `lib/templates/hooks/pre-commit.sh`, never against a stub:
//!
//! - `a_migrated_project_can_still_commit_while_a_hook_invoked_command_is_unbuilt`
//! - `an_unmigrated_project_can_still_commit`
//!
//! **The second arm is the one the criterion gained when issue 0045 widened it,
//! and it did not exist until 2026-08-17.** The original wording was written
//! against 0038, whose fixture is migrated -- so the criterion, the test built
//! from it, and every instrument pointed at it inherited that scope, and the
//! UNMIGRATED project, which is the state every project is in until WP-10 runs
//! on it, was covered by nothing.
//!
//! **Neither arm asserts the exit code of `intent critic` directly**, and that
//! is deliberate: such a test passes the moment someone changes 1 to 2 and
//! proves nothing about the gate. The hook is the consumer whose behaviour
//! changed, so the hook is what is driven.
//!
//! Status on the row is vc's to set, from a clean tree, and not from here.

use std::process::{Command, Output};

/// Run the built binary. `CARGO_BIN_EXE_<name>` is set by cargo for
//. integration tests, so this needs no dev-dependency and cannot pick up a
/// stale `intent` from PATH -- which would silently measure v2.
fn run(args: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .output()
    .expect("run the v3 binary")
}

#[test]
fn a_missing_required_argument_exits_1() {
  let out = run(&["st", "show"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "clap's default is 2; D17 carries v2's 1 over.\nstdout: {}\nstderr: {}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
}

#[test]
fn an_unknown_flag_exits_1() {
  let out = run(&["st", "list", "--no-such-flag"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "clap's default is 2 for UnknownArgument too.\nstderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

#[test]
fn an_unknown_subcommand_exits_1() {
  let out = run(&["st", "bogusverb"]);
  assert_eq!(out.status.code(), Some(1));
}

#[test]
fn an_unknown_family_exits_1() {
  let out = run(&["nosuchfamily"]);
  assert_eq!(out.status.code(), Some(1));
}

/// INV-01: the voice is lowercase `error:` on stderr, no banners.
///
/// clap writes its own `error: ...` in a different shape and to a stream that
/// varies by error kind, so this asserts BOTH the prefix and the stream.
#[test]
fn usage_errors_speak_v2s_voice_on_stderr() {
  let out = run(&["st", "show"]);
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(
    stderr.starts_with("error: "),
    "the lowercase voice (0023, INV-01), got: {stderr}"
  );
  assert!(
    String::from_utf8_lossy(&out.stdout).is_empty(),
    "a failure writes nothing to stdout -- INV-06 is the v2 defect being corrected, not reproduced"
  );
  assert!(
    !stderr.contains("Usage:") || stderr.lines().count() < 20,
    "no banner dump on a usage error"
  );
}

/// A successful invocation exits 0 and says so in v2's voice.
#[test]
fn success_exits_0() {
  let out = run(&["--version"]);
  assert_eq!(
    out.status.code(),
    Some(0),
    "stderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// **This test replaces one that could not fail, and the vacuity is the story.**
///
/// The original ran `intent critic --help` and asserted
/// `code != 2 || !stderr.contains("unexpected argument")`. `--help` exits 0
/// and writes nothing to stderr, so the first disjunct was always true and the
/// assertion held for every possible behaviour of the binary. Its doc comment
/// said it existed "so a blanket always-exit-1 cannot pass" -- **and a blanket
/// always-exit-1 is exactly what shipped**, as issue 0038. The guard was
/// written, named for the right property, and never evaluated it.
///
/// So the real form asks for the code on an invocation that FAILS, and the
/// disjunction is gone: there is one number and it is asserted.
#[test]
fn the_unavailable_exception_is_not_flattened_by_the_override() {
  let out = run(&["critic", "shell", "--staged"]);
  assert_eq!(
    out.status.code(),
    Some(2),
    "INV-04's 2 survives INV-02's blanket override of clap's usage codes.\nstderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );
}

/// **Issue 0038, the whole contract in one test.**
///
/// Asserted TOGETHER rather than as three cases, because the defect was not
/// any single wrong number -- it was three different events sharing one. A
/// per-case test passes if every code is changed to the same new value; this
/// one only passes if they stay distinct in the way v2 distinguishes them.
///
/// Measured against v2 in this repository (`bin/intent`, 2026-08-16) rather
/// than inferred from its source, and the measurement narrowed the fix: two of
/// the three cases issue 0038 proposed separating **already matched v2 and had
/// to stay 1**. Only "this build cannot answer" was wrong.
#[test]
fn an_unbuilt_command_is_not_the_same_event_as_a_bad_invocation() {
  let unbuilt = run(&["critic", "shell", "--staged"]).status.code();
  let unknown = run(&["nosuchfamily"]).status.code();
  let usage = run(&["st", "show"]).status.code();

  assert_eq!(
    unbuilt,
    Some(2),
    "a known command this build has not wired is an UNAVAILABLE TOOL, which is v2's 2 -- \
     the consumer that reads it (the shipped pre-commit gate) treats 2 as fail-open and 1 as \
     `your code has findings`"
  );
  assert_eq!(unknown, Some(1), "v2 exits 1 for an unknown command");
  assert_eq!(usage, Some(1), "v2 exits 1 for a usage error (INV-02, D17)");
  assert_ne!(
    unbuilt, usage,
    "the tool being absent and the caller being wrong are different events and only one of them \
     is the caller's fault"
  );
}

/// **The consumer, driven end to end -- because the number is only worth what
/// the thing reading it does with it.**
///
/// Every assertion above is about a number in isolation. The defect was never
/// about a number: it was that a project migrating to v3 while any
/// hook-invoked command was still unwired **could not commit at all**, and the
/// remedy it printed named findings that did not exist, leaving `--no-verify`
/// as the only way through -- which trains a habit that outlives the cause.
///
/// So this drives the SHIPPED hook against the real binary, the way the defect
/// was found. The hook is not modified by the fix and is not modified by this
/// test; its `2+` fail-open branch was correct all along and simply never
/// reached. If someone reverts the exit code, a unit assertion above reds AND
/// this reds with the user-visible symptom, which is the one worth reading.
/// Build a project fixture at `intent_version` and run the SHIPPED pre-commit
/// hook inside it, returning (exit code, stderr).
///
/// **Extracted rather than copied.** The unmigrated case below differs from the
/// migrated one by a single field, and a fifty-line fixture pasted twice is two
/// fixtures that agree until somebody edits one -- which is the drift these
/// tests exist to catch one layer up.
fn shipped_hook_in(intent_version: &str) -> (Option<i32>, String) {
  let hook = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../../../lib/templates/hooks/pre-commit.sh")
    .canonicalize()
    .expect("locate the shipped pre-commit hook");

  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  std::fs::create_dir_all(root.join("intent/.config")).expect("mkdir");
  std::fs::write(
    root.join("intent/.config/config.json"),
    format!(
      "{{\"intent_version\":\"{intent_version}\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"shell\"]}}\n"
    ),
  )
  .expect("write config");
  std::fs::write(root.join("script.sh"), "#!/bin/bash\necho hi\n").expect("write a shell file");

  for args in [
    vec!["init", "-q", "."],
    vec!["config", "user.email", "t@t"],
    vec!["config", "user.name", "t"],
    vec!["add", "-A"],
  ] {
    let ok = Command::new("git")
      .args(&args)
      .current_dir(root)
      .output()
      .expect("run git")
      .status
      .success();
    assert!(ok, "git {args:?} failed while building the fixture");
  }

  // The hook calls `intent` by name, so v3 has to BE `intent` on PATH -- which
  // is also how a consumer meets this: issue 0036 records that `brew install`
  // shadows a v2 install rather than replacing it.
  let shim = root.join("shim");
  std::fs::create_dir_all(&shim).expect("mkdir shim");
  #[cfg(unix)]
  std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_intent"), shim.join("intent"))
    .expect("put v3 first on PATH as `intent`");

  let path = format!(
    "{}:{}",
    shim.display(),
    std::env::var("PATH").unwrap_or_default()
  );
  let out = Command::new("bash")
    .arg(&hook)
    .current_dir(root)
    .env("PATH", path)
    .output()
    .expect("run the shipped hook");
  (
    out.status.code(),
    String::from_utf8_lossy(&out.stderr).to_string(),
  )
}

#[test]
fn a_migrated_project_can_still_commit_while_a_hook_invoked_command_is_unbuilt() {
  let (code, stderr) = shipped_hook_in("3.0.0");

  assert_eq!(
    code,
    Some(0),
    "the gate must fail OPEN when the critic is unavailable, not block the commit.\n{stderr}"
  );
  assert!(
    stderr.contains("fail-open"),
    "and it must say WHY it let the commit through, rather than passing silently:\n{stderr}"
  );
  assert!(
    !stderr.contains("commit blocked by findings"),
    "the remedy naming findings that do not exist is the half of 0038 a user actually meets:\n{stderr}"
  );
}

/// **ISSUE 0045 (vc): an UNMIGRATED project must still be able to commit, and
/// this test exists to fail on a day that has not arrived.**
///
/// The shipped gate reads `1` from `intent critic` as FINDINGS and blocks. But
/// `Facade::open` calls `readable()` before anything else, so **every
/// facade-opening command in an unmigrated project returns `Unmigrated ->
/// Failure::Error -> 1`.** Build `critic` on `Facade::open` -- which is the
/// obvious right thing to reach for -- and the gate blocks every commit in
/// every unmigrated project, printing a remedy about findings that do not
/// exist while the true remedy sits on screen above it, overridden.
///
/// **It passes today only because `critic` is unbuilt and answers 2**, into the
/// fail-open branch issue 0038's fix created. That is a reprieve nobody chose
/// and it ends when WP-07 does.
///
/// **The control is what stops this being decorative.** A guard that merely
/// asserts "the gate returned 0" would pass on a fixture that was never
/// unmigrated at all, which is the vacuous shape this file's own header records
/// shipping as 0038. So the fixture is proven unmigrated first, through the
/// same binary, and the refusal it produces is the exact one that would reach
/// the gate.
#[test]
fn an_unmigrated_project_can_still_commit() {
  // The fixture is genuinely in the state under test, and the refusal that
  // would reach the gate is genuinely reachable -- at the code the gate reads
  // as findings.
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::create_dir_all(dir.path().join("intent/.config")).expect("mkdir");
  std::fs::write(
    dir.path().join("intent/.config/config.json"),
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"shell\"]}\n",
  )
  .expect("write config");
  let control = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "list"])
    .current_dir(dir.path())
    .output()
    .expect("run a facade-opening command");
  assert_eq!(
    control.status.code(),
    Some(1),
    "precondition: a facade-opening command in an unmigrated project refuses at 1 -- the code the gate reads as findings. If this is no longer true the test \
     below is guarding nothing"
  );
  assert!(
    String::from_utf8_lossy(&control.stderr).contains("not been migrated"),
    "precondition: and it refuses for the MIGRATION reason, not some other one"
  );

  let (code, stderr) = shipped_hook_in("2.19.0");

  assert_eq!(
    code,
    Some(0),
    "the shipped gate blocked a commit in an unmigrated project. **A command the gate invokes is answering in the code it reads as findings** -- almost \
     certainly `critic` built on `Facade::open`, whose `readable()` refuses before anything else runs. `doctor` and the migrator are already exempt because \
     their job IS this state; `critic` needs exempting on a different ground -- its consumer fails CLOSED on the refusal code.\n{stderr}"
  );
  assert!(
    !stderr.contains("commit blocked by findings"),
    "and it must never claim findings it does not have. The true remedy -- run `intent upgrade` -- is printed by the refusal and then overridden by one that \
     cannot be followed:\n{stderr}"
  );
}

/// **THE AGENT GUIDE'S EXIT-CODE PARAGRAPH IS EXECUTED, NOT READ.**
///
/// It told every agent that `intent critic` exits **2** when it has findings,
/// "which is a verdict about your code and not a broken run", and that `1` is
/// "every failure". **Both halves are backwards**, and this file's own opening
/// paragraph had the correct contract written down two hundred lines above the
/// false one.
///
/// Measured at v2's source: `bin/intent_critic:254` exits **1** with findings,
/// `:89` and `:95` exit **2** when it cannot run -- and the shipped gate writes
/// the contract out in its own comment at
/// `lib/templates/hooks/pre-commit.sh:262`: *0 = clean, 1 = findings at or
/// above threshold, 2 = invocation error (fail-open for that language)*.
///
/// **So `2` is the code the gate FAILS OPEN on**, and an agent following the
/// guide would read a critic that never ran as a verdict on its code, and a
/// real findings result -- the one that BLOCKS the commit -- as a broken run.
///
/// **The guide is generated, and that is exactly why this is needed.**
/// `guide.rs` closes its own module doc with the reason: *completeness of the
/// ROW SET comes for free; the truth of each rendered field does not, and no
/// generator will ever check it.* The surface-wide facts are hand-written prose
/// inside a generated document, so they are the part with no mechanism behind
/// them -- and one of them was false.
#[test]
fn the_guides_exit_code_claims_are_what_the_binary_does() {
  let out = run(&["llm", "guide"]);
  assert_eq!(out.status.code(), Some(0), "the guide must render");
  let guide = String::from_utf8_lossy(&out.stdout).into_owned();

  // The claim about `2`, driven: a declared-but-unimplemented command, carrying
  // the stderr line the guide tells an agent to recognise it by.
  let unbuilt = run(&["critic", "shell"]);
  assert_eq!(
    unbuilt.status.code(),
    Some(2),
    "the guide's stated cause of a 2 does not produce one"
  );
  const PHRASE: &str = "is a known command that is not implemented yet";
  assert!(
    String::from_utf8_lossy(&unbuilt.stderr).contains(PHRASE),
    "stderr: {}",
    String::from_utf8_lossy(&unbuilt.stderr)
  );
  assert!(
    guide.contains(PHRASE),
    "the guide must name the stderr line an agent recognises a 2 by, or the \
     code is unattributable"
  );

  // The claim about `1`, driven on a command that ran and refused.
  assert_eq!(
    run(&["st", "show"]).status.code(),
    Some(1),
    "the guide says 1 is the command running and answering no"
  );

  // **And no command may be given its own exit-code rule in prose while this
  // build does not give it one.** `critic` is not implemented here, so every
  // sentence the guide spends on its codes describes software that is not in
  // the binary the reader is holding -- which is how the swap survived. Scoped
  // to the surface-wide section, because the per-row reference names commands
  // constantly and must.
  let facts = guide
    .split("## Facts about the whole surface")
    .nth(1)
    .unwrap_or_else(|| panic!("the guide has no surface-wide section:\n{guide}"))
    .split("\n## ")
    .next()
    .expect("a section body");
  assert!(
    !facts.contains("critic"),
    "the surface-wide facts state an exit-code rule for `critic`, which this \
     build does not implement -- so the claim cannot be checked against \
     anything the reader can run:\n{facts}"
  );
}
