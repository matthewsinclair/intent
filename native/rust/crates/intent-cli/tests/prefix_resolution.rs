//! Least-chars-necessary-to-be-unique, at every level of the command tree.
//!
//! **THE FEATURE IS TWO HALVES AND THE SECOND IS THE ONE THAT CAN LIE.**
//! Resolving a unique prefix is `infer_subcommands`, one line. Refusing an
//! ambiguous one is the half with a message in it, and clap's own message for
//! that case says `unrecognized subcommand` -- **which is false in the direction
//! that costs**, sending somebody to look for a verb they have already found.
//!
//! **EVERY EXPECTATION HERE IS DERIVED FROM THE SHIPPED TABLE, NEVER TYPED IN.**
//! A test asserting `intent explo` resolves would keep passing after `explode`
//! was added beside it -- reporting as working the exact case the feature has to
//! get right. So the corpus is the real command list, uniqueness is computed
//! from it, and the assertions follow.

use std::process::Command;

use intent_cli::dispatch;

/// Every top-level command name the SHIPPED surface carries.
///
/// **`shipped_entries`, NOT EVERY ROW IN THE TABLE, AND THE FIRST BUILD GOT
/// THIS WRONG IN THE INSTRUCTIVE DIRECTION.** The table also carries RETIRED
/// commands -- `help`, `st_zero`, `treeindex` -- so that a person typing one
/// gets told what replaced it. They are deliberately absent from the clap
/// surface, so a corpus built from every row asserted that `intent tr` should
/// reach a command this build does not have, and reported three failures that
/// were entirely the test's.
///
/// **THE POPULATION YOU MEASURED IS NOT THE ONE YOU CLAIMED**, arrived at from
/// the honest end: the assertions were right and the corpus was wrong. It is the
/// same traversal `dual_path_conformance` uses, for the same reason -- a second
/// definition of "shipped" in a file about the shipped surface is the drift it
/// exists to catch.
fn top_level() -> Vec<String> {
  let table = dispatch::table();
  let mut names: Vec<String> = dispatch::shipped_entries(&table)
    .iter()
    .filter_map(|entry| entry.path.split(' ').next().map(str::to_string))
    .collect();
  names.sort();
  names.dedup();
  names
}

/// Run the shipped binary under a FIXTURE `HOME`, never the operator's.
///
/// **THIS FILE PICKS ITS VERBS FROM THE DISPATCH TABLE AT RUN TIME, SO ITS
/// REACH IS WHATEVER THE TABLE HOLDS THE DAY IT RUNS.** Today every invocation
/// here carries `--help` or is an unparseable prefix, so nothing touches user
/// state -- and that is a fact about today's table, not a property of this
/// file. **A verb implemented next week is a verb this file will drive**, which
/// is how `dispatch_ssot` came to publish this machine's install pointer to a
/// scratch worktree that was then deleted (2026-08-27, `9c2ba9ed`).
///
/// **IT WAS MISSING FROM THE FIRST BUILD AND `table_driven_tests_fixture_their_home`
/// CAUGHT IT -- ON THE FIRST FULL-SUITE RUN AFTER THIS FILE LANDED, WHICH WAS A
/// DAY LATER.** The guard worked from the moment the file existed; what did not
/// happen was anybody running it. **A guard that works and a loop nobody runs
/// are indistinguishable from a green** (vc, 2026-08-30).
fn run(argv: &[&str]) -> (i32, String, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("the intent binary runs");
  (
    out.status.code().unwrap_or(-1),
    String::from_utf8_lossy(&out.stdout).to_string(),
    String::from_utf8_lossy(&out.stderr).to_string(),
  )
}

/// The shortest prefix of `name` that no other name in `corpus` shares.
fn shortest_unique(name: &str, corpus: &[String]) -> Option<String> {
  (1..=name.len())
    .map(|k| &name[..k])
    .find(|prefix| {
      corpus
        .iter()
        .filter(|other| other.starts_with(*prefix))
        .count()
        == 1
    })
    .map(str::to_string)
}

#[test]
fn the_shortest_unique_prefix_of_every_command_reaches_that_command() {
  let names = top_level();
  assert!(
    names.len() > 20,
    "the command corpus is {} -- too small to be the real surface, so this test measured nothing",
    names.len()
  );

  let mut checked = 0usize;
  let mut failures: Vec<String> = Vec::new();

  for name in &names {
    // A command that is a strict prefix of another (`st` inside `st_zero`) has
    // no unique prefix shorter than itself, and reaches itself by EXACT match
    // rather than by inference. Skipped here and covered by its own test below,
    // because folding the two would hide which rule was doing the work.
    let Some(prefix) = shortest_unique(name, &names) else {
      continue;
    };
    if prefix == *name {
      continue;
    }
    checked += 1;

    let (_, out, err) = run(&[&prefix, "--help"]);
    let seen = format!("{out}{err}");
    // `--help` on the resolved command prints that command's own usage, which
    // names it. Comparing against the FULL name is what makes this a test of
    // where the prefix landed rather than of whether anything happened.
    if !seen.contains(name.as_str()) {
      failures.push(format!(
        "`intent {prefix}` should reach `{name}` and its help does not mention it: {}",
        seen.lines().next().unwrap_or("<no output>")
      ));
    }
  }

  assert!(
    checked >= 10,
    "only {checked} command(s) had a proper unique prefix, which is too few for this to be \
     evidence about the surface"
  );
  assert!(
    failures.is_empty(),
    "{} of {checked} unique prefixes did not reach their command:\n  {}",
    failures.len(),
    failures.join("\n  ")
  );
}

#[test]
fn an_ambiguous_prefix_is_refused_and_says_what_it_was_between() {
  let names = top_level();

  // **THE AMBIGUOUS TOKEN IS FOUND IN THE CORPUS, NOT CHOSEN.** Hard-coding
  // `exp` would leave this test asserting nothing the day `export` is renamed --
  // and it would still pass, because a missing command also fails.
  let ambiguous = (1..6)
    .flat_map(|k| {
      names
        .iter()
        .filter(move |n| n.len() > k)
        .map(move |n| n[..k].to_string())
    })
    .find(|token| names.iter().filter(|n| n.starts_with(token)).count() >= 2)
    .expect("some prefix of some command is shared by two commands");

  let sharers: Vec<&String> = names.iter().filter(|n| n.starts_with(&ambiguous)).collect();

  let (code, _, err) = run(&[&ambiguous]);
  assert_ne!(code, 0, "an ambiguous prefix must not succeed: {err}");
  assert!(
    err.contains("ambiguous"),
    "the refusal for `{ambiguous}` must say it is AMBIGUOUS. clap's own message is `unrecognized \
     subcommand`, which is false -- these commands exist, and it sends the reader looking for one \
     they have already found: {err}"
  );
  for sharer in &sharers {
    assert!(
      err.contains(sharer.as_str()),
      "the refusal must NAME `{sharer}` as one of the things `{ambiguous}` matched, or the reader \
       has to go and work out the alternatives themselves: {err}"
    );
  }
}

#[test]
fn every_spelling_the_refusal_recommends_actually_resolves() {
  // **THE ARM THAT MAKES THE MESSAGE A CONTRACT RATHER THAN PROSE.** The first
  // build of this remedy offered `token + 1 character`, which produced
  // "`exp` or `exp` or `ext`" for `intent ex` -- a recommendation that is still
  // ambiguous, printed twice. **A remedy that does not work is worse than no
  // remedy**: it is confidently wrong at the moment somebody is already stuck.
  let (_, _, err) = run(&["ex"]);
  assert!(
    err.contains("ambiguous"),
    "expected an ambiguity refusal for `ex`: {err}"
  );

  let offered: Vec<String> = err
    .split('`')
    .skip(1)
    .step_by(2)
    .filter(|token| !token.is_empty() && token.chars().all(|c| c.is_ascii_lowercase()))
    .map(str::to_string)
    .collect();
  let recommended: Vec<&String> = offered.iter().filter(|t| t.starts_with("ex")).collect();
  assert!(
    !recommended.is_empty(),
    "the refusal recommended no spelling at all: {err}"
  );

  for spelling in &recommended {
    if spelling.as_str() == "ex" {
      continue;
    }
    let (code, out, e) = run(&[spelling, "--help"]);
    assert_eq!(
      code, 0,
      "the refusal recommended `{spelling}` and it does not resolve: {e}"
    );
    assert!(
      !format!("{out}{e}").contains("ambiguous"),
      "the refusal recommended `{spelling}`, which is STILL ambiguous. The remedy has to name a \
       spelling that works, or it is the tool disagreeing with itself in front of somebody who is \
       already stuck"
    );
  }
}

#[test]
fn every_exact_command_name_still_resolves_to_itself() {
  // **EXACT-MATCH-WINS IS THE RULE THAT MAKES PREFIXES SAFE, AND IT IS TESTED
  // OVER THE WHOLE CORPUS RATHER THAN AGAINST A SHADOWED PAIR.**
  //
  // The first version of this test looked for a command that is a strict prefix
  // of another -- `st` inside `st_zero` -- and asserted it still reached itself.
  // **Measured across every level of the shipped tree: there are ZERO such
  // pairs**, because `st_zero` is retired. So that test was driving a shape the
  // surface does not have, and its own guard said to replace it rather than
  // leave it passing.
  //
  // **THE PROPERTY IS REAL EVEN THOUGH TODAY'S INSTANCE IS NOT**: the day
  // somebody adds a verb beginning with an existing verb's whole name, inference
  // must not take the shorter one off the surface. What is testable NOW, over
  // the real corpus, is the rule that would protect it -- an exactly-typed
  // command name always resolves to itself and is never reported as ambiguous.
  // That fails the moment exact-match-wins is lost, which is the regression the
  // shadowed-pair test existed to catch.
  let names = top_level();
  let mut failures: Vec<String> = Vec::new();

  for name in &names {
    let (code, out, err) = run(&[name, "--help"]);
    let seen = format!("{out}{err}");
    if code != 0 || seen.contains("ambiguous") {
      failures.push(format!(
        "`intent {name}` is an exact command name and did not resolve to itself (rc={code}): {}",
        seen.lines().next().unwrap_or("<no output>")
      ));
    }
  }

  assert!(
    failures.is_empty(),
    "{} of {} exact command names failed to resolve:\n  {}",
    failures.len(),
    names.len(),
    failures.join("\n  ")
  );
}

#[test]
fn a_token_that_is_no_prefix_at_all_keeps_clap_s_own_message() {
  // The ambiguity path must not swallow the genuinely-unknown case: replacing a
  // true message with a vaguer one is not an improvement.
  let (code, _, err) = run(&["zzzznotacommand"]);
  assert_ne!(code, 0);
  assert!(
    !err.contains("ambiguous"),
    "a token matching nothing was reported as ambiguous: {err}"
  );
  assert!(
    err.contains("unrecognized") || err.contains("unknown"),
    "an unknown command should still say so: {err}"
  );
}
