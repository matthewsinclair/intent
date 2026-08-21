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
//! - `a_migrated_project_can_still_commit`
//! - `an_unmigrated_project_can_still_commit`
//!
//! **The first was renamed on 2026-08-20 and lost a clause it could no longer
//! keep.** It read `..._while_a_hook_invoked_command_is_unbuilt`; `critic`
//! landed at `5043d0c4` and a `shell` fixture stopped meeting one. AC-10.9's
//! own words are "a project can still COMMIT with v3 installed -- MIGRATED OR
//! NOT", which never mentioned an unbuilt command, so the criterion is
//! unchanged and the test name simply stopped over-promising. The fail-open
//! behaviour that clause was really about is now its own test,
//! `the_gate_fails_open_and_names_the_language_when_the_critic_cannot_answer`,
//! which is NOT an AT-10.9 arm.
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
///
/// # The exemplar changed on 2026-08-20 because the old one got BUILT
///
/// This drove `critic shell --staged`, whose 2 was the *unwired* 2 -- so a test
/// named for INV-04 was measuring it through a stand-in for the very command
/// INV-04 is about, and would have kept passing if `critic`'s own codes were
/// wrong in every particular. `critic` landed at `5043d0c4`, the stand-in
/// started exiting 0, and the test got STRONGER by being broken: it now drives
/// `intent critic` rejecting its own invocation, which is INV-04's 2 itself.
#[test]
fn the_unavailable_exception_is_not_flattened_by_the_override() {
  let out = run(&["critic", "klingon"]);
  assert_eq!(
    out.status.code(),
    Some(2),
    "INV-04's 2 survives INV-02's blanket override of clap's usage codes.\nstderr: {}",
    String::from_utf8_lossy(&out.stderr)
  );

  // **THE CONTROL, AND IT MUST COME BACK 1.** Without it a build that had
  // simply STOPPED overriding clap would satisfy the assertion above and look
  // correct -- clap's own default for a usage error is 2, so "2 survived" and
  // "2 was never converted in the first place" are indistinguishable from one
  // measurement. This one is a usage error clap raises, and INV-02 still turns
  // it into 1.
  //
  // **AND THE PAIR RECORDS A DIVERGENCE INSIDE `critic` THAT IS dc's TO RULE,
  // NOT MINE TO ENCODE AS CORRECT.** Both invocations are usage errors and they
  // exit differently: `critic klingon` is 2 (fail-open, language named
  // UNENFORCED) while `critic --no-such-flag` is 1, which the shipped gate
  // reads as FINDINGS and blocks on, printing a remedy for findings that do not
  // exist -- issue 0038's exact symptom through a different door. It is LATENT
  // rather than live: measured on 2026-08-20, the installed
  // `.git/hooks/pre-commit.intent` and `lib/templates/hooks/pre-commit.sh` pass
  // the same four flags, so nothing reaches this arm today. It becomes
  // reachable on hook/binary flag skew, and that skew is this repo's normal
  // state -- the installed hook is an install-time COPY.
  let flag = run(&["critic", "shell", "--no-such-flag"]);
  assert_eq!(
    flag.status.code(),
    Some(1),
    "the blanket override is no longer converting clap's usage codes, so the assertion above \
     proves nothing about INV-04 surviving it.\nstderr: {}",
    String::from_utf8_lossy(&flag.stderr)
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
///
/// # The exemplar is BORROWED, and it will expire again
///
/// `st dehydrate` stands in for "a known command this build has not wired",
/// replacing `critic shell`, which stopped being one when `critic` landed at
/// `5043d0c4`. **The roster of record is `DECLARED_BUT_UNWIRED` in
/// `intentsvcs/tests/write_moves_only_what_changed.rs`, driven by
/// `intent-cli/tests/cli_write_moves_only_what_changed.rs`, not this file** --
/// those drive every member and go red when any is built, which is the
/// mechanism; this borrows a single member and says so rather than keeping a
/// second list that could drift from it silently.
///
/// **THE RECORD MOVED ON 2026-08-21 AND THE MOVE IS THE INTERESTING PART.** It
/// used to be `declared_but_unwired.rs`, which looped over a roster -- and a
/// loop over a roster passes VACUOUSLY when the roster empties, which is why
/// that file carried an explicit non-empty guard. The driver that replaced it
/// names each verb as its own case, so there is no iterate-zero shape left to
/// guard: **the vacuity was designed out rather than relocated.** A citation
/// pointing at the retired file would still have resolved through git and told
/// a reader the mechanism is a loop it no longer is.
///
/// So this WILL red again the day `st dehydrate` is implemented, and that is
/// the design working: pick another member from the roster. On the day the
/// roster empties, these three assertions have no subject left in the surface
/// and should be RETIRED rather than repaired -- there would be no
/// declared-but-unimplemented command for the contract to be about.
#[test]
fn an_unbuilt_command_is_not_the_same_event_as_a_bad_invocation() {
  let unbuilt = run(&["st", "dehydrate", "ST0056"]).status.code();
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

/// Build a project fixture at `intent_version` declaring `language`, and run
/// the SHIPPED pre-commit hook inside it, returning (exit code, stderr).
///
/// **Extracted rather than copied.** The unmigrated case below differs from the
/// migrated one by a single field, and a fifty-line fixture pasted twice is two
/// fixtures that agree until somebody edits one -- which is the drift these
/// tests exist to catch one layer up.
///
/// `language` became a parameter on 2026-08-20: the hook dispatches one critic
/// per declared language, so it is the only lever a caller has over what the
/// gate MEETS, and the fail-open arm now needs a language the critic will not
/// answer for.
fn shipped_hook_in(intent_version: &str, language: &str) -> (Option<i32>, String) {
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
      "{{\"intent_version\":\"{intent_version}\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"{language}\"]}}\n"
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
/// test.
///
/// # This was ONE test and it was carrying TWO properties, and BOTH legs moved
///
/// It read `a_migrated_project_can_still_commit_while_a_hook_invoked_command_
/// is_unbuilt`, drove a `shell` fixture against an unwired `critic`, and
/// asserted the commit survived AND that the gate said why. **Two separate
/// things then happened to it, and only one of them was visible.**
///
/// - `b2609e26` (hole 3) reworded the `*)` arm's printed line from `invocation
///   error (exit $rc); fail-open` to `did not check ... UNENFORCED`, because
///   the gate was diagnosing a cause it never measured. **The literal
///   `fail-open` moved into a COMMENT, and this test's second assertion had
///   been pinned to it, so the test went red there** -- before `critic`
///   existed, for a reason that had nothing to do with `critic`.
/// - `5043d0c4` built `critic`, which deleted the SCENARIO: a `shell` fixture
///   no longer meets an unbuilt hook-invoked command at all.
///
/// **The first cause was hidden by the second being on its way.** Repairing
/// only for `critic` would have re-pinned a Rust test to a sentence in a shell
/// script in another tree -- and the lesson of `b2609e26` is that nobody
/// rewording that arm can be expected to know this file exists. A peer's prose
/// is not an API, and asserting on it makes it one without telling them.
///
/// So the two properties are now two tests, and the assertion below anchors on
/// `UNENFORCED` -- the hook's own load-bearing token (the array name, the
/// digest word, and the concept) rather than an incidental phrase.
#[test]
fn a_migrated_project_can_still_commit() {
  let (code, stderr) = shipped_hook_in("3.0.0", "shell");

  assert_eq!(
    code,
    Some(0),
    "the shipped gate blocked a commit in a migrated project.\n{stderr}"
  );
  assert!(
    !stderr.contains("commit blocked by findings"),
    "the remedy naming findings that do not exist is the half of 0038 a user actually meets:\n{stderr}"
  );
  // **THE CONTROL, AND IT IS EXACTLY AS STRONG AS IT READS.** It separates
  // "the commit survived because the gate PASSED" from "the commit survived
  // because the gate FELL OPEN", which are the same 0 and the same silence
  // otherwise. It does NOT establish that the critic scanned anything -- the
  // hook prints `out` only on the 1 and `*)` arms, so a clean run and a no-op
  // are indistinguishable from here. That question is `critic`'s own parity
  // suite's, one layer down, and claiming it here would be a count of
  // containers reported as a count of contents.
  assert!(
    !stderr.contains("UNENFORCED"),
    "the commit went through, but the gate FELL OPEN rather than passing -- so this test would be \
     asserting that a commit survived a gate that was not running:\n{stderr}"
  );
}

/// **THE FAIL-OPEN ARM, AND IT NEEDED A SUBJECT THAT CANNOT BE BUILT.**
///
/// The property is the one issue 0038 was really about: when `intent critic`
/// answers in a code the gate does not recognise, the commit proceeds AND the
/// gate says which language went unchecked. That protects every consumer, and
/// it is worth a test that does not expire.
///
/// Every previous subject expired. An unwired `critic` stopped being unwired;
/// any declared-but-unimplemented verb gets implemented eventually. **An
/// undeclared LANGUAGE never becomes declared** -- the hook dispatches whatever
/// `languages` names, `intent critic` owns the registry, and a name outside it
/// is a usage error at 2 forever. So the fixture declares one.
///
/// This is a fixture, not a claim that anyone ships `klingon`: the hook's own
/// comment says it "needs no language knowledge of its own", and driving it
/// with a name it cannot know is the cleanest way to hold it to that.
#[test]
fn the_gate_fails_open_and_names_the_language_when_the_critic_cannot_answer() {
  let (code, stderr) = shipped_hook_in("3.0.0", "klingon");

  assert_eq!(
    code,
    Some(0),
    "the gate must fail OPEN when the critic cannot answer, not block the commit -- a gate that \
     must be bypassed is a guard nobody keeps.\n{stderr}"
  );
  assert!(
    stderr.contains("UNENFORCED"),
    "and it must say WHAT WENT UNCHECKED, rather than passing silently. **If this reds after a \
     reword of the `*)` arm or the digest in lib/templates/hooks/pre-commit.sh, the hook is \
     probably fine and this string is the stale half** -- the property is that the gate names \
     the unchecked language, not that it uses this word:\n{stderr}"
  );
  assert!(
    stderr.contains("klingon"),
    "the digest must NAME the language; `1 of 1 went unenforced` without the name leaves the \
     operator nothing to act on:\n{stderr}"
  );
  assert!(
    !stderr.contains("commit blocked by findings"),
    "and it must never claim findings it does not have:\n{stderr}"
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
/// **THE DAY ARRIVED ON 2026-08-20 AND THE TEST HELD.** This doc used to read
/// *it passes today only because `critic` is unbuilt and answers 2 -- a
/// reprieve nobody chose, and it ends when WP-07 does.* WP-07 ended at
/// `5043d0c4`, and measured in a git-initialised unmigrated fixture the built
/// `critic` exits **0** and scans normally: dc built it on the project-optional
/// path, so `Facade::open`'s `readable()` is never reached and the refusal this
/// test feared never forms.
///
/// **Recorded rather than deleted, because the two outcomes look identical from
/// the green.** A test written to fail on a named day, which does not fail on
/// that day, has either been vindicated or quietly lost its subject -- and the
/// only thing that tells them apart is someone checking WHY it passed on the
/// day. It was vindicated: the hazard it named was real, was reachable by the
/// obvious implementation, and was avoided.
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

  let (code, stderr) = shipped_hook_in("2.19.0", "shell");

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

  // The claim about `2`, FIRST cause, driven: a declared-but-unimplemented
  // command, carrying the stderr line the guide tells an agent to recognise it
  // by. Exemplar borrowed from the `DECLARED_BUT_UNWIRED` roster; see the note
  // on `an_unbuilt_command_is_not_the_same_event_as_a_bad_invocation`.
  let unbuilt = run(&["st", "dehydrate", "ST0056"]);
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

  // **THE GUARD HERE USED TO BE ITS OWN INVERSE, AND THE FLIP IS THE POINT.**
  // It read `!facts.contains("critic")`, on the ground that *`critic` is not
  // implemented here, so every sentence the guide spends on its codes describes
  // software that is not in the binary the reader is holding*. That ground
  // expired at `5043d0c4`. The RULE it served did not: **no command may be
  // given its own exit-code rule in prose unless this build gives it one** --
  // and now that it does, the obligation reverses from "do not mention it" to
  // "mention it, and be drivable". Scoped to the surface-wide section, because
  // the per-row reference names commands constantly and must.
  let facts = guide
    .split("## Facts about the whole surface")
    .nth(1)
    .unwrap_or_else(|| panic!("the guide has no surface-wide section:\n{guide}"))
    .split("\n## ")
    .next()
    .expect("a section body");
  assert!(
    facts.contains("critic"),
    "`intent critic` is the one command in this build whose 2 is not the unwired 2, and the \
     surface-wide facts no longer say so -- an agent meeting it has nothing to read:\n{facts}"
  );

  // The claim about `2`, SECOND cause, driven -- because the sentence above is
  // only worth the behaviour behind it. **This is the assertion the old guard
  // could not make**: while `critic` was unbuilt there was nothing to run, so
  // the rule could only be enforced by silence.
  //
  // **3 IS NAMED BY INV-04's TITLE AND DELIBERATELY LEFT UNEXPLAINED HERE.**
  // It is REFUSED -- a rule the project armed could not be enforced -- and the
  // gate BLOCKS on it, so it matters to a reader. This test cannot drive one:
  // dc's commit records that arm as having no live population, covered by a
  // fixture inside `critic`'s own suite. Adding prose for it would be exactly
  // the unexecutable claim this test exists to prevent, so the gap is dc's to
  // close from the side that can drive it.
  let bad_lang = run(&["critic", "klingon"]);
  assert_eq!(
    bad_lang.status.code(),
    Some(2),
    "the guide tells an agent that `intent critic` rejecting an invocation it cannot act on is a \
     2.\nstderr: {}",
    String::from_utf8_lossy(&bad_lang.stderr)
  );
}
