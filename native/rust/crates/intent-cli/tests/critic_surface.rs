//! **`intent critic` -- the surface `tests/unit/intent_critic.bats` guarded,
//! carried onto the binary that now provides it (ST0056, the `bin/` prune).**
//!
//! The v2 file's arms ran against `bin/intent_critic`, a population-A script
//! the prune deletes. The standard applied is vc's, from
//! `no_absolute_home_paths`: **does v3 cover the PROPERTY** -- not does the v2
//! test still pass, because a passing test whose subject is being deleted is
//! evidence about the past. Most of its properties are covered and are asserted
//! here against the shipped binary. The rest are not carried, and each is named
//! below rather than dropped in silence.
//!
//! **THE v2 FILE IS STILL PRESENT AND STILL GREEN, AND THAT IS DELIBERATE.**
//! The `bin/` delete is held on an hv ruling, so for as long as it is held both
//! files run. This is a migration in flight, not a second home to be resolved
//! by deleting whichever copy a reader meets first: `tests/unit/intent_critic.bats`
//! goes WITH its subject, in the one commit that removes population A, or it
//! does not go at all.
//!
//! # What v3 does BETTER, so the migration is not read as a downgrade
//!
//! v2's JSON was a bare array of findings, so `[]` was its whole answer for a
//! clean run and **a run that examined nothing was byte-identical to a run that
//! examined everything and found nothing.** v3 emits an object carrying
//! `armed`, `asked`, `census`, `refused` and `total` beside `findings`, and its
//! text mode says so in words: _a clean result covers what was ASKED and says
//! nothing about the rest_. That is the denominator discipline this estate has
//! been adding everywhere, already present here.
//!
//! # THE PROPERTIES THAT DID NOT COME ACROSS
//!
//! **Some are retirements with their subject**, needing no decision: v2's arm 1
//! (`bin/intent_critic` exists and is executable) and arm 21 (`intent critic`
//! dispatches to `bin/intent_critic`) are about a script that will not exist.
//! v2's arm 2 asserts the v2 usage STRING (`intent critic <lang>`), which is
//! v2's rendering of its own help; clap owns that now.
//!
//! **TWO WERE LIVE DIVERGENCES WHERE v2 REFUSED AND v3 ANSWERED SUCCESS. ONE
//! IS CLOSED AND ONE IS STILL OPEN.** They were pinned below as assertions on
//! TODAY's behaviour, not endorsed -- the technique `plugin_surface.rs` uses:
//! couple the record to the behaviour, so that fixing either one REDS THE ARM
//! and sends the next reader back to this header instead of letting the
//! divergence be absorbed by a quiet edit. **That coupling worked as designed:
//! the second divergence was fixed on 2026-09-12 and the arm reddened**, which
//! is why this header changed in the same commit as the code.
//!
//! **(1) A BARE `intent critic <lang>` REPORTS CLEAN OVER ZERO FILES AT EXIT
//! 0.** v2 exited 2 with _no files specified_ rather than guess a population.
//! Measured in this repository, which tracks many `.rs`, `.sh` and Elixir
//! files: `critic rust`, `critic shell` and `critic elixir` each print `ok: no
//! <lang> findings ... across 0 file(s)` and exit 0. **The population is not
//! empty; the run examined none of it.** The stdout line does carry the `0
//! file(s)` denominator, so this is not a silent zero to a human reading it --
//! but the EXIT CODE is what a caller branches on, and it says clean. The
//! shipped gate is unaffected: `lib/templates/hooks/pre-commit.sh` invokes
//! `--staged`, where an empty population genuinely means nothing to check.
//!
//! **(2) AN UNKNOWN `--format` WAS ACCEPTED AT EXIT 0 AND SILENTLY RENDERED
//! TEXT. CLOSED 2026-09-12, hv's v3.0.2 ruling, `intent/wip.md` item 8.** v2
//! exited 2 with _invalid --format_; `--help` declares the flag as
//! `--format <text|json>` -- a closed set -- and the binary took any
//! string, so a script whose `--format json` was typoed to `--format jsonl` got
//! text at exit 0 and parsed garbage. **Its sibling on the same command already
//! validated**: `--severity-min bogus` is refused at exit 2, so the two flags
//! disagreed about whether a declared value set is enforced. That was vc's own
//! class from 2026-08-31 -- the declaration promising what the binary does not
//! do, raised there against `intent daemon status` declaring
//! `--format terminal|json` and projecting neither.
//!
//! It now refuses at exit 2 with a remedy naming the set, which is this
//! command's usage code by INV-04's named exception rather than a slip. The arm
//! below asserts the REFUSAL; the record of what it used to do is this
//! paragraph.
//!
//! Both were `IN-AG-NO-SILENT-001`'s subject. (1) is still open: it is a
//! question about what a bare invocation should do, which nobody has ruled.

use std::path::PathBuf;
use std::process::{Command, Output};

use testkit::repo_root;

/// The bad/good pair the v2 file used: the `strong-assertions` rule's own
/// fixtures, which ship in the rule library and are not population A.
fn fixture(name: &str) -> PathBuf {
  repo_root().join(format!(
    "intent/plugins/claude/rules/elixir/test/strong-assertions/{name}"
  ))
}

/// Drive the shipped binary from the repository root, which is where its canon
/// discovery finds the real rule library.
fn critic(args: &[&str]) -> Output {
  crate::common::intent()
    .arg("critic")
    .args(args)
    .current_dir(repo_root())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary")
}

fn out(o: &Output) -> String {
  String::from_utf8_lossy(&o.stdout).into_owned()
}

fn err(o: &Output) -> String {
  String::from_utf8_lossy(&o.stderr).into_owned()
}

/// v2 arm: `--languages` lists the code-critic languages and NOT the prose
/// disciplines, which have no headless critic.
///
/// The refutation is the half that carries the meaning -- listing `author`
/// would advertise a runner that does not exist.
#[test]
fn the_registry_lists_code_critics_and_withholds_the_prose_disciplines() {
  let o = critic(&["--languages"]);
  assert_eq!(o.status.code(), Some(0), "`--languages` must succeed");
  let printed = out(&o);
  let listed: Vec<&str> = printed
    .lines()
    .map(str::trim)
    .filter(|l| !l.is_empty())
    .collect();

  for lang in ["elixir", "rust", "swift", "lua", "shell"] {
    assert!(
      listed.contains(&lang),
      "the registry dropped `{lang}`: {listed:?}"
    );
  }
  for prose in ["author", "content"] {
    assert!(
      !listed.contains(&prose),
      "`{prose}` has no headless critic, so listing it advertises a runner that \
       does not exist: {listed:?}"
    );
  }
}

/// v2 arms: an unknown language and an unknown severity are both refused at
/// exit 2, and each refusal names what it would have accepted.
#[test]
fn an_unknown_language_or_severity_is_refused_and_the_refusal_names_the_set() {
  let lang = critic(&["cobol", "--files", "/tmp/x"]);
  assert_eq!(
    lang.status.code(),
    Some(2),
    "an unknown language must be refused"
  );
  assert!(
    err(&lang).contains("must be a language"),
    "the refusal must say what the argument had to be: {}",
    err(&lang)
  );

  let good = fixture("good_test.exs");
  let sev = critic(&[
    "elixir",
    "--files",
    good.to_str().unwrap(),
    "--severity-min",
    "bogus",
  ]);
  assert_eq!(
    sev.status.code(),
    Some(2),
    "an unknown severity must be refused"
  );
  assert!(
    err(&sev).contains("not a severity"),
    "the refusal must name the offending value: {}",
    err(&sev)
  );
}

/// v2 arms: a prose discipline is a clean no-op, NOT an argument error.
///
/// The distinction is the whole point -- `author` is a real Intent discipline
/// with no headless runner, so refusing it as an unknown language would tell an
/// operator their configuration is wrong when it is right.
#[test]
fn a_prose_discipline_is_a_clean_no_op_and_not_an_argument_error() {
  for discipline in ["author", "content"] {
    let o = critic(&[discipline]);
    assert_eq!(
      o.status.code(),
      Some(0),
      "`critic {discipline}` must be a clean no-op, not a refusal.\nstderr: {}",
      err(&o)
    );
  }

  // The control: the same shape with a genuinely unknown word IS refused, or
  // the arm above would pass for a binary that accepted anything at all.
  let control = critic(&["notadiscipline"]);
  assert_eq!(
    control.status.code(),
    Some(2),
    "anti-vacuity: an unknown word must still be refused, or the no-op above \
     says nothing"
  );
}

/// v2 arms: the bad fixture produces a CRITICAL finding naming its rule and
/// exits 1; the good fixture at the same threshold is clean at exit 0.
///
/// **Asserted as a PAIR, because neither half means anything alone.** A runner
/// that reported everything would pass the first; one that reported nothing
/// would pass the second.
#[test]
fn the_bad_fixture_fires_its_rule_and_the_good_one_is_clean_at_the_same_threshold() {
  let bad = fixture("bad_test.exs");
  let o = critic(&[
    "elixir",
    "--files",
    bad.to_str().unwrap(),
    "--severity-min",
    "critical",
  ]);
  assert_eq!(o.status.code(), Some(1), "a critical finding must exit 1");
  let printed = out(&o);
  assert!(
    printed.contains("CRITICAL"),
    "the severity must be named: {printed}"
  );
  assert!(
    printed.contains("IN-EX-TEST-001"),
    "the finding must name the rule that produced it: {printed}"
  );
  // v2's severity-filter arm, folded in: at `critical`, warnings are excluded.
  assert!(
    !printed.contains("WARNING"),
    "`--severity-min critical` let a warning through: {printed}"
  );

  let good = fixture("good_test.exs");
  let clean = critic(&[
    "elixir",
    "--files",
    good.to_str().unwrap(),
    "--severity-min",
    "critical",
  ]);
  assert_eq!(
    clean.status.code(),
    Some(0),
    "the good fixture must be clean"
  );
  assert!(
    out(&clean).contains("ok:"),
    "a clean run must say so: {}",
    out(&clean)
  );
}

/// v2 arms: `--format json` is parseable, carries findings when there are any,
/// and is empty when there are none.
///
/// **Widened deliberately.** v2 asserted a bare array, so its clean answer was
/// `[]` -- indistinguishable from a run over nothing. v3's object is asserted
/// to carry the denominator too, because that is the property worth guarding
/// and the reason the shape changed.
#[test]
fn json_carries_both_the_findings_and_the_denominator_the_bare_array_could_not() {
  let bad = fixture("bad_test.exs");
  let o = critic(&[
    "elixir",
    "--files",
    bad.to_str().unwrap(),
    "--severity-min",
    "critical",
    "--format",
    "json",
  ]);
  assert_eq!(
    o.status.code(),
    Some(1),
    "findings still exit 1 under --format json"
  );

  let v: serde_json::Value =
    serde_json::from_str(&out(&o)).expect("`--format json` must emit parseable JSON on stdout");
  let findings = v["findings"].as_array().expect("a findings array");
  assert!(
    !findings.is_empty(),
    "the bad fixture must produce findings: {v}"
  );

  for key in ["armed", "asked", "total"] {
    assert!(
      v.get(key).is_some(),
      "the JSON dropped `{key}` -- without the denominator a clean answer cannot \
       be told from a run that examined nothing, which is what v2's bare array \
       could not express: {v}"
    );
  }

  let good = fixture("good_test.exs");
  let clean = critic(&[
    "elixir",
    "--files",
    good.to_str().unwrap(),
    "--severity-min",
    "critical",
    "--format",
    "json",
  ]);
  assert_eq!(clean.status.code(), Some(0));
  let cv: serde_json::Value = serde_json::from_str(&out(&clean)).expect("parseable JSON");
  assert_eq!(
    cv["findings"].as_array().map(Vec::len),
    Some(0),
    "a clean run must report an empty findings list: {cv}"
  );
  // **THE ASSERTION THAT MAKES THIS MIGRATION WORTH MORE THAN THE ARM IT
  // REPLACES.** `total` is the RULE denominator, not a findings count, so a
  // clean run reports `findings: []` beside a NON-ZERO population -- which is
  // exactly the sentence v2's bare `[]` could not say. A clean answer over an
  // empty rule library would be a very different fact, and under v2 the two
  // were the same three bytes.
  let total = cv["total"].as_u64().expect("a rule denominator");
  assert!(
    total > 0,
    "a clean run reported an empty findings list over a ZERO-rule population, \
     which is the reassuring answer from an instrument that asked nothing: {cv}"
  );
  assert_eq!(
    cv["census"].as_array().map(Vec::len),
    Some(total as usize),
    "the census must account for every rule the denominator claims: {cv}"
  );
}

/// v2 arms: `--staged` outside a git repository is refused; inside one with
/// nothing staged it is clean.
///
/// **The pair is the point.** An empty population under `--staged` genuinely
/// means nothing to check, which is why exit 0 is right there and wrong for the
/// bare form recorded in this file's header.
#[test]
fn staged_is_refused_outside_a_repository_and_clean_inside_one_with_nothing_staged() {
  let dir = tempfile::tempdir().expect("tempdir");

  let outside = crate::common::intent()
    .args(["critic", "elixir", "--staged"])
    .current_dir(dir.path())
    .output()
    .expect("run the v3 binary");
  assert_eq!(
    outside.status.code(),
    Some(2),
    "`--staged` outside a repository must refuse, not report clean.\nstdout: {}",
    String::from_utf8_lossy(&outside.stdout)
  );
  assert!(
    err(&outside).contains("git repositor"),
    "the refusal must name the reason: {}",
    err(&outside)
  );

  for args in [
    vec!["init", "-q", "."],
    vec![
      "-c",
      "user.email=t@t.com",
      "-c",
      "user.name=T",
      "commit",
      "--allow-empty",
      "-q",
      "-m",
      "init",
    ],
  ] {
    let st = Command::new("git")
      .args(&args)
      .current_dir(dir.path())
      .status()
      .expect("git");
    assert!(st.success(), "fixture setup failed: git {args:?}");
  }
  // A project, because critic refuses outside one like every other verb.
  std::fs::create_dir_all(dir.path().join("intent/.config")).expect("config dir");
  std::fs::write(
    dir.path().join("intent/.config/config.json"),
    r#"{"intent_version":"3.0.0","project_name":"CriticStaged","author":"t","created_date":"2026-04-24T00:00:00Z"}"#,
  )
  .expect("project marker");

  let inside = crate::common::intent()
    .args(["critic", "elixir", "--staged"])
    .current_dir(dir.path())
    .output()
    .expect("run the v3 binary");
  assert_eq!(
    inside.status.code(),
    Some(0),
    "`--staged` with nothing staged is genuinely nothing to check.\nstderr: {}",
    String::from_utf8_lossy(&inside.stderr)
  );
}

/// v2 arms: a rule disabled in `.intent_critic.yml` is suppressed, and
/// disabling a DIFFERENT rule leaves it firing.
///
/// **Both halves or neither.** The suppression arm alone passes for a runner
/// that stopped finding anything, which is the failure mode that matters most
/// in a gate: the config silently disabling everything looks exactly like a
/// clean estate.
#[test]
fn a_disabled_rule_is_suppressed_and_disabling_another_leaves_it_firing() {
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::create_dir_all(dir.path().join("intent/.config")).expect("config dir");
  std::fs::write(
    dir.path().join("intent/.config/config.json"),
    r#"{"intent_version":"3.0.0","project_name":"CriticConfig","author":"t","created_date":"2026-04-24T00:00:00Z"}"#,
  )
  .expect("project marker");

  let bad = fixture("bad_test.exs")
    .canonicalize()
    .expect("the bad fixture");
  let drive = |disabled: &str| {
    std::fs::write(
      dir.path().join(".intent_critic.yml"),
      format!("disabled:\n  - {disabled}\nseverity_min: critical\n"),
    )
    .expect("critic config");
    crate::common::intent()
      .args([
        "critic",
        "elixir",
        "--files",
        bad.to_str().unwrap(),
        "--severity-min",
        "critical",
      ])
      .current_dir(dir.path())
      .output()
      .expect("run the v3 binary")
  };

  let other = drive("IN-EX-TEST-999");
  assert_eq!(
    other.status.code(),
    Some(1),
    "disabling an unrelated rule must leave this one firing, or the suppression \
     below is a statement about a runner that found nothing.\nstdout: {}",
    out(&other)
  );
  assert!(
    out(&other).contains("IN-EX-TEST-001"),
    "the rule must still be named: {}",
    out(&other)
  );

  let suppressed = drive("IN-EX-TEST-001");
  assert_eq!(
    suppressed.status.code(),
    Some(0),
    "the disabled rule was still counted.\nstdout: {}",
    out(&suppressed)
  );
  assert!(
    !out(&suppressed).contains("IN-EX-TEST-001"),
    "a disabled rule must not be reported: {}",
    out(&suppressed)
  );
}

/// **A FILE THE TOOL DECLINED IS NOT A FILE THAT PASSED** (`intent/wip.md`
/// item 8, ruled by hv 2026-09-12).
///
/// shellcheck refuses zsh outright (`SC1071`) and exits 1 with one `error`
/// line carrying a code no rule claims, so the finding filter dropped it, the
/// rule reported nothing, **and the census said `ran`.** A `.zsh` file passed
/// every shellcheck-armed rule at exit 0, examined by nothing, with the census
/// -- whose whole job is to say what was NOT asked -- asserting it had been.
///
/// **THE BASH CONTROL IS THE ARM, NOT DECORATION.** Identical content in a
/// `.sh` file must still be ASKED, or a fix that simply stopped running
/// shellcheck everywhere would pass this test.
///
/// **AND IT ASSERTS ON BOTH KINDS OF MACHINE.** Where shellcheck is installed
/// the subject is the decline; where it is not, the same rules are `ToolAbsent`
/// and the run REFUSES at 3 -- so this never degrades into a skip that reads as
/// a pass on a machine without the tool.
#[test]
fn a_file_shellcheck_declines_is_reported_not_counted_as_asked() {
  let dir = tempfile::tempdir().expect("tempdir");
  let body = "for f in $(ls *.txt); do\n  echo \"$f\"\ndone\n";
  let zsh = dir.path().join("probe.zsh");
  let sh = dir.path().join("control.sh");
  std::fs::write(&zsh, format!("#!/usr/bin/env zsh\n{body}")).expect("write zsh");
  std::fs::write(&sh, format!("#!/usr/bin/env bash\n{body}")).expect("write sh");

  let have_shellcheck = Command::new("shellcheck")
    .arg("--version")
    .output()
    .is_ok_and(|o| o.status.success());

  let declined = critic(&["shell", "--files", zsh.to_str().unwrap()]);
  let control = critic(&["shell", "--files", sh.to_str().unwrap()]);

  if !have_shellcheck {
    assert_eq!(
      declined.status.code(),
      Some(3),
      "with no shellcheck on this machine both runs are ARMED-but-absent and \
       must REFUSE, which is the other half of the same contract: {}{}",
      out(&declined),
      err(&declined)
    );
    assert_eq!(control.status.code(), Some(3));
    return;
  }

  assert!(
    out(&declined).contains("DECLINED to read") && out(&declined).contains("probe.zsh"),
    "the census must name the rules that did not run and the file they did not \
     run on: {}",
    out(&declined)
  );
  assert!(
    out(&declined).contains("0 of "),
    "a run whose every armed rule declined the file asked NOTHING, and the \
     headline is where a reader sees it: {}",
    out(&declined)
  );
  assert!(
    !out(&control).contains("DECLINED") && !out(&control).contains("0 of "),
    "THE CONTROL: identical content in a .sh file is still asked, or a fix that \
     stopped running shellcheck at all would pass the arm above: {}",
    out(&control)
  );
}

/// **THE DIVERGENCE STILL OPEN, PINNED TO TODAY'S BEHAVIOUR AND NOT/// **THE DIVERGENCE STILL OPEN, PINNED TO TODAY'S BEHAVIOUR AND NOT
/// ENDORSED, AND THE ONE THAT CLOSED.**
///
/// See this file's header for both in full. The first assertion exists so the
/// divergence cannot be absorbed silently: **if it is fixed, this arm goes red
/// and sends the reader to the header rather than to a green suite that forgot
/// the question was open.** That is the coupling `plugin_surface.rs` uses for
/// the same purpose, and it did its job -- the `--format` half reddened here
/// when the fix landed, and the header and this arm moved in that commit.
#[test]
fn a_bare_language_answers_clean_and_an_undeclared_format_is_refused() {
  // (1) STILL OPEN. A bare language answers CLEAN over a population it did not
  // read. The control is in the assertion: this repository tracks hundreds of
  // Rust files, so `0 file(s)` is a statement about the scan and not about the
  // corpus.
  let bare = critic(&["rust"]);
  assert_eq!(
    bare.status.code(),
    Some(0),
    "RECORDED, NOT ENDORSED: v2 exited 2 (`no files specified`) rather than \
     guess a population. If this now refuses, the divergence in this file's \
     header is CLOSED -- update the header and delete this half."
  );
  assert!(
    out(&bare).contains("across 0 file(s)"),
    "the run examined nothing and the line that says so is the only thing \
     distinguishing it from a real clean run: {}",
    out(&bare)
  );

  // (2) CLOSED. An undeclared `--format` value is refused, and the refusal
  // names the set the row declares rather than leaving the operator to guess.
  let good = fixture("good_test.exs");
  let bogus = critic(&[
    "elixir",
    "--files",
    good.to_str().unwrap(),
    "--format",
    "xml",
  ]);
  assert_eq!(
    bogus.status.code(),
    Some(2),
    "an undeclared `--format` is a usage error, and 2 is this command's usage \
     code (INV-04's named exception): {}{}",
    out(&bogus),
    err(&bogus)
  );
  assert!(
    err(&bogus).contains("not a format this command serves") && err(&bogus).contains("text, json"),
    "the refusal must name the set, or the operator is told no and not what to type: {}",
    err(&bogus)
  );
  assert!(
    out(&bogus).is_empty(),
    "a refused invocation renders nothing: a caller that reads stdout must not \
     find a report there: {}",
    out(&bogus)
  );

  // **THE DECLARED VALUES BOTH STILL WORK**, or the arm above would pass over a
  // flag that refuses everything.
  for fmt in ["text", "json"] {
    let o = critic(&["elixir", "--files", good.to_str().unwrap(), "--format", fmt]);
    assert_eq!(
      o.status.code(),
      Some(0),
      "`--format {fmt}` is declared on this verb's row and must be served: {}{}",
      out(&o),
      err(&o)
    );
  }

  // The sibling flag on the same command validates the same way. Asserted here
  // so the two are read as one command's single rule rather than as two flags
  // that happen to agree today.
  let sev = critic(&[
    "elixir",
    "--files",
    good.to_str().unwrap(),
    "--severity-min",
    "xml",
  ]);
  assert_eq!(
    sev.status.code(),
    Some(2),
    "`--severity-min` and `--format` now agree: a declared value set is enforced"
  );
}

/// **The gate does not lint a rule library's own examples; a named file is still
/// read.** A staged file under the library the run loaded is one of its rules'
/// fixtures, and critiquing it refused the library's own commits. It is skipped
/// and named in `skipped_library`. The same file handed over with `--files` is an
/// ask, and fires.
#[test]
fn a_staged_rule_example_is_skipped_and_named_while_a_named_one_is_read() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  std::fs::create_dir_all(root.join("intent/.config")).expect("config dir");
  std::fs::write(
    root.join("intent/.config/config.json"),
    r#"{"intent_version":"3.0.0","project_name":"CriticLibrary","author":"t","created_date":"2026-04-24T00:00:00Z"}"#,
  )
  .expect("project marker");
  let rule_dir = root.join("rules/elixir/test/strong-assertions");
  std::fs::create_dir_all(&rule_dir).expect("rule dir");
  for name in ["RULE.md", "bad_test.exs"] {
    std::fs::copy(fixture(name), rule_dir.join(name)).expect("copy the shipped rule");
  }
  for args in [
    vec!["init", "-q", "."],
    vec!["add", "rules/elixir/test/strong-assertions/bad_test.exs"],
  ] {
    let st = Command::new("git")
      .args(&args)
      .current_dir(root)
      .status()
      .expect("git");
    assert!(st.success(), "fixture setup failed: git {args:?}");
  }
  let rules = root.join("rules");
  let drive = |extra: &[&str]| {
    let mut args = vec!["critic", "elixir", "--rules", rules.to_str().unwrap()];
    args.extend_from_slice(extra);
    args.extend_from_slice(&["--severity-min", "critical", "--format", "json"]);
    crate::common::intent()
      .args(&args)
      .current_dir(root)
      .output()
      .expect("run the v3 binary")
  };

  let staged = drive(&["--staged"]);
  assert_eq!(
    staged.status.code(),
    Some(0),
    "a staged rule example is not the project's code.\nstderr: {}",
    err(&staged)
  );
  let v: serde_json::Value = serde_json::from_str(&out(&staged)).expect("json");
  assert_eq!(
    v["skipped_library"],
    serde_json::json!(["rules/elixir/test/strong-assertions/bad_test.exs"]),
    "and the skip is named, not silent: {v}"
  );
  assert!(v["findings"].as_array().is_some_and(Vec::is_empty), "{v}");

  let bad = rule_dir.join("bad_test.exs");
  let named = drive(&["--files", bad.to_str().unwrap()]);
  assert_eq!(
    named.status.code(),
    Some(1),
    "the same file named with --files is an ask, and fires.\nstdout: {}",
    out(&named)
  );
}

/// Issue 0437: **IN-RS-CODE-004's proxy reads the `Result`'s own error type,
/// never the last parameter of a generic inside it.** The gate refused a
/// `BTreeMap<String, String>` collected into a `Result` whose error is
/// `rusqlite::Error`, on the map's own `, String>`. Driven through the binary
/// against the real library, which is how the gate runs it.
#[test]
fn a_string_closing_a_generic_inside_a_result_is_not_its_error_type() {
  let dir = tempfile::tempdir().expect("tempdir");
  let file = dir.path().join("src/lib.rs");
  std::fs::create_dir_all(file.parent().expect("src")).expect("mkdir src");
  std::fs::write(
    &file,
    [
      "  .collect::<Result<std::collections::BTreeMap<String, String>, rusqlite::Error>>()?;",
      "pub fn load(path: &str) -> Result<Config, String> {",
      "pub fn check() -> Result<(), String> {",
      "pub fn open() -> Result<Config, Box<dyn std::error::Error>> {",
      "pub fn bytes() -> Result<Vec<u8>, String> {",
    ]
    .join("\n"),
  )
  .expect("write the fixture");

  let o = critic(&[
    "rust",
    "--files",
    file.to_str().unwrap(),
    "--format",
    "json",
  ]);
  let v: serde_json::Value = serde_json::from_str(&out(&o)).expect("parseable JSON");
  let struck: Vec<u64> = v["findings"]
    .as_array()
    .expect("a findings array")
    .iter()
    .filter(|f| f["rule"] == "IN-RS-CODE-004")
    .filter_map(|f| f["line"].as_u64())
    .collect();
  assert_eq!(
    struck,
    vec![2, 3, 4],
    "the map's `String>` is not the error type, so line 1 reads clean; a `String` or \
     `Box<dyn Error>` error still fires; and a first parameter carrying a generic (line 5) \
     is the stated false negative, pinned so that widening the pattern is a decision: {v}"
  );
}
