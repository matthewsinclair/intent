//! **`intent critic` -- the surface `tests/unit/intent_critic.bats` guarded,
//! carried onto the binary that now provides it (ST0056, the `bin/` prune).**
//!
//! The v2 file has 21 arms against `bin/intent_critic`, a 12,415-line
//! population-A script the prune deletes. The standard applied is vc's, from
//! `no_absolute_home_paths`: **does v3 cover the PROPERTY** -- not does the v2
//! test still pass, because a passing test whose subject is being deleted is
//! evidence about the past. Seventeen of the 21 properties are covered and are
//! asserted here against the shipped binary. Four are not carried, and each is
//! named below rather than dropped in silence.
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
//! # THE FOUR THAT DID NOT COME ACROSS
//!
//! **Two are retirements with their subject**, and need no decision: v2's arm 1
//! (`bin/intent_critic` exists and is executable) and arm 21 (`intent critic`
//! dispatches to `bin/intent_critic`) are about a script that will not exist.
//! v2's arm 2 asserts the v2 usage STRING (`intent critic <lang>`), which is
//! v2's rendering of its own help; clap owns that now.
//!
//! **TWO ARE LIVE DIVERGENCES WHERE v2 REFUSED AND v3 ANSWERS SUCCESS.** They
//! are pinned below as assertions on TODAY's behaviour, not endorsed -- the
//! technique `plugin_surface.rs` uses: couple the record to the behaviour, so
//! that fixing either one REDS THE ARM and sends the next reader back to this
//! header instead of letting the divergence be absorbed by a quiet edit.
//!
//! **(1) A BARE `intent critic <lang>` REPORTS CLEAN OVER ZERO FILES AT EXIT
//! 0.** v2 exited 2 with _no files specified_ rather than guess a population.
//! Measured in this repository, which tracks 332 `.rs`, 112 `.sh` and 41 Elixir
//! files: `critic rust`, `critic shell` and `critic elixir` each print `ok: no
//! <lang> findings ... across 0 file(s)` and exit 0. **The population is not
//! empty; the run examined none of it.** The stdout line does carry the `0
//! file(s)` denominator, so this is not a silent zero to a human reading it --
//! but the EXIT CODE is what a caller branches on, and it says clean. The
//! shipped gate is unaffected: `lib/templates/hooks/pre-commit.sh` invokes
//! `--staged`, where an empty population genuinely means nothing to check.
//!
//! **(2) AN UNKNOWN `--format` IS ACCEPTED AT EXIT 0 AND SILENTLY RENDERS
//! TEXT.** v2 exited 2 with _invalid --format_. `--help` declares the flag as
//! `--format <text|json>` -- a closed set of two -- and the binary takes any
//! string. A script whose `--format json` is typoed to `--format jsonl` gets
//! text at exit 0 and parses garbage. **Its sibling on the same command
//! validates**: `--severity-min bogus` is refused at exit 2 with ``bogus` is
//! not a severity``, so the two flags disagree about whether a declared value
//! set is enforced. This is vc's own class from 2026-08-31 -- the declaration
//! promising what the binary does not do, raised there against `intent daemon
//! status` declaring `--format terminal|json` and projecting neither -- and
//! this is a second instance in a different verb.
//!
//! Both are `IN-AG-NO-SILENT-001`'s subject and neither is fixed here: they are
//! surface behaviour, and a test migration is not the place to change what the
//! binary does during a tag window.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

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
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .arg("critic")
    .args(args)
    .current_dir(repo_root())
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

  let outside = Command::new(env!("CARGO_BIN_EXE_intent"))
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

  let inside = Command::new(env!("CARGO_BIN_EXE_intent"))
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
    Command::new(env!("CARGO_BIN_EXE_intent"))
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

/// **THE TWO DIVERGENCES, PINNED TO TODAY'S BEHAVIOUR AND NOT ENDORSED.**
///
/// See this file's header for both in full. These assertions exist so the
/// divergence cannot be absorbed silently: **if either is fixed, this arm goes
/// red and sends the reader to the header rather than to a green suite that
/// forgot the question was open.** That is the coupling `plugin_surface.rs`
/// uses for the same purpose. Delete this arm when the record moves; do not
/// edit the expectations to match a fix.
#[test]
fn the_two_places_where_v2_refused_and_this_build_answers_success() {
  // (1) A bare language answers CLEAN over a population it did not read. The
  // control is in the assertion: this repository tracks hundreds of Rust files,
  // so `0 file(s)` is a statement about the scan and not about the corpus.
  let bare = critic(&["rust"]);
  assert_eq!(
    bare.status.code(),
    Some(0),
    "RECORDED, NOT ENDORSED: v2 exited 2 (`no files specified`) rather than \
     guess a population. If this now refuses, the divergence in this file's \
     header is CLOSED -- update the header and delete this arm."
  );
  assert!(
    out(&bare).contains("across 0 file(s)"),
    "the run examined nothing and the line that says so is the only thing \
     distinguishing it from a real clean run: {}",
    out(&bare)
  );

  // (2) An undeclared `--format` value is accepted and text is rendered.
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
    Some(0),
    "RECORDED, NOT ENDORSED: `--help` declares `--format <text|json>` and v2 \
     exited 2 on anything else. If this now refuses, the divergence is CLOSED \
     -- update the header and delete this arm."
  );
  assert!(
    !out(&bogus).starts_with('{'),
    "the fallback renders text rather than the JSON a caller may have meant: {}",
    out(&bogus)
  );

  // The sibling flag on the same command DOES validate. Asserted here so the
  // two arms above are read as a disagreement inside one command rather than
  // as a binary that validates nothing.
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
    "`--severity-min` validates while `--format` does not; if this changed, the \
     header's framing of the second divergence needs revisiting"
  );
}
