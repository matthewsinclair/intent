//! **`doctor --quiet` AND `doctor --verbose` DO WHAT THEY SAY, DRIVEN.**
//!
//! hv's D55 (2026-08-20) ruled the verbosity flags ship. The table now says
//! `keep` and the spine builds any `keep` flag from data, so **acceptance is
//! free and proves nothing**: mark any pending flag `keep` and clap will take
//! it, silently, whether or not a renderer ever reads it. That is the hazard
//! this file exists for -- a flag that parses and does nothing is worse than an
//! absent one, because `--help` now advertises it.
//!
//! So every test here asserts a DIFFERENCE IN OUTPUT rather than an exit code.
//! An assertion that `doctor -q` succeeds passes identically on a build that
//! ignores the flag.
//!
//! # Why the interaction is tested and not just the flags
//!
//! v2 resolves the pair one way -- `bin/intent_doctor:134` is
//! `if [ "$VERBOSE" = true ] && [ "$QUIET" != true ]`, so quiet wins. A parity
//! flag whose interaction with its sibling differs from v2's has been
//! re-designed under the name of being carried across, and nothing else in the
//! estate would notice: both spellings are accepted either way.

use std::path::Path;
use std::process::Command;

fn bin() -> &'static Path {
  Path::new(env!("CARGO_BIN_EXE_intent"))
}

/// Run `doctor` in THIS repository, which is a real v3 project.
///
/// **`doctor` is read-only and this is checked rather than assumed**: the arm
/// opens a facade opportunistically and calls `Facade::doctor`, which takes the
/// store by shared reference and reports. Running it in a temp dir would test a
/// different code path -- the one that has no project to examine -- and the
/// counts these tests compare would all be zero, which is the vacuous-pass
/// shape rather than a measurement.
fn doctor(args: &[&str]) -> String {
  let root = Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(4)
    .expect("the project root is four above crates/intent-cli");
  let mut argv = vec!["doctor"];
  argv.extend_from_slice(args);
  let out = Command::new(bin())
    .args(&argv)
    .current_dir(root)
    .output()
    .unwrap_or_else(|e| panic!("could not run `intent doctor {args:?}`: {e}"));
  // Both streams, because a finding and a refusal do not share a stream and a
  // test reading only stdout would go quiet on exactly the interesting run.
  String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr)
}

/// The summary is the one informational line `--quiet` keeps, so it is the
/// needle every test below uses to tell "quiet" from "produced nothing".
const VERDICT: &str = "finding(s) across";

/// The lines `--verbose` adds: what THIS RUN resolved.
const RESOLVED: [&str; 4] = [
  "doctor: root ",
  "doctor: intent ",
  "doctor: canon ",
  "doctor: store ",
];

#[test]
fn both_flags_are_advertised() {
  let help = doctor(&["--help"]);
  for spelling in ["--verbose", "-v", "--quiet", "-q"] {
    assert!(
      help.contains(spelling),
      "`doctor --help` does not advertise `{spelling}`; D55 ruled it ships, and a flag the help does not name is one a user has no way to discover\n{help}"
    );
  }
}

/// **THE VERDICT SURVIVES `--quiet`, AND THAT IS THE LOAD-BEARING HALF.**
///
/// Dropping it would make a clean run print nothing at all at rc=0 -- and
/// silence on success is indistinguishable from the command never having run,
/// which is the defect this estate found in its own commit gate on the day
/// these flags were built. The counts are also the coverage denominator: "no
/// problems found" over an estate the checker never read reads exactly like
/// "no problems found" over one it read completely.
#[test]
fn quiet_drops_what_is_not_a_finding_and_keeps_the_verdict() {
  let quiet = doctor(&["--quiet"]);
  assert!(
    quiet.contains(VERDICT),
    "`doctor --quiet` dropped the verdict line; a quiet clean run would then be totally silent, which is indistinguishable from not having run\n{quiet}"
  );
  assert!(
    !quiet.contains("surface: "),
    "`doctor --quiet` still printed the withheld-flag lines; those are explicitly NOT findings and do not move the exit code, so they are what quiet is for\n{quiet}"
  );
  assert!(
    !quiet.contains("not carried by the store"),
    "`doctor --quiet` still printed the unattached inventory; it is inventory rather than fault, by its own doc comment\n{quiet}"
  );
}

/// A flag that parses and changes nothing passes every acceptance test and is
/// still a lie in the help text. This is the arm that catches it.
///
/// **IT RUNS AGAINST A TREE THE TEST BUILDS, AND IT DID NOT USED TO.** It ran
/// `doctor` at this repository's own root, where it passed because Intent
/// happened to hold 196 uncarried `.tap` baselines for `--quiet` to suppress.
/// On 2026-08-26 the carry stopped consulting extensions, those files became
/// attachments, the inventory went to ZERO here, and quiet and default became
/// the same 17 lines. **The flag was never broken; its only exercise was
/// incidental repository content, and it expired the moment the content
/// changed.**
///
/// That is the same shape as an enforcement inherited from an unrelated
/// mechanism, seen from the test's side: **an exercise nobody set up is an
/// assumption, and only a red ever says it has gone.** So the subject is
/// constructed here -- one file over the cap, which is what `--quiet` suppresses
/// now -- and it survives whatever this repository happens to contain next.
#[test]
fn quiet_is_strictly_less_than_the_default() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let config = root.join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Q\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");

  let made = Command::new(bin())
    .args(["st", "new", "A thread"])
    .current_dir(root)
    .output()
    .expect("st new");
  assert!(
    made.status.success(),
    "the fixture must have a thread for doctor to walk: {}",
    String::from_utf8_lossy(&made.stderr)
  );

  let thread_dir = std::fs::read_dir(root.join("intent/st"))
    .expect("st dir")
    .filter_map(Result::ok)
    .map(|e| e.path())
    .find(|p| p.is_dir())
    .expect("st new made a thread directory");
  std::fs::write(
    thread_dir.join("huge.png"),
    vec![b'x'; intentsvcs::project::ATTACHMENT_CAP_BYTES as usize + 1],
  )
  .expect("write a file over the cap");

  let at = |args: &[&str]| -> String {
    let mut argv = vec!["doctor"];
    argv.extend_from_slice(args);
    let out = Command::new(bin())
      .args(&argv)
      .current_dir(root)
      .output()
      .expect("doctor");
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    )
  };
  let default = at(&[]);
  let quiet = at(&["--quiet"]);

  assert!(
    default.contains("not carried by the store"),
    "the FIXTURE must give quiet something to suppress, or this arm proves \
     nothing about the flag:\n{default}"
  );
  assert!(
    quiet.lines().count() < default.lines().count(),
    "`doctor --quiet` produced {} line(s) against the default's {} -- the flag is accepted and does nothing",
    quiet.lines().count(),
    default.lines().count()
  );
}

/// **`--verbose` NAMES WHERE THE ANSWERS CAME FROM**, which is v2's own
/// behaviour (`bin/intent_doctor:204,217` emit `INTENT_HOME=` and `Found at `)
/// rather than a new idea.
///
/// The store line is the one that earns the flag. `doctor` asks the backup
/// half of its report only when the store opens, and until D55 nothing said
/// which run had happened -- so two reports differing by a whole check looked
/// identical. **Both branches of that line are driven**: this asserts the
/// healthy one, and the `NOT opened` branch was driven by hand against a
/// project whose canon will not parse, which is the case the arm's own comment
/// cites as the reason it exists.
#[test]
fn verbose_names_what_the_run_resolved() {
  let verbose = doctor(&["--verbose"]);
  for line in RESOLVED {
    assert!(
      verbose.contains(line),
      "`doctor --verbose` did not name `{line}` -- the flag advertises detailed information and this is the detail\n{verbose}"
    );
  }
  let default = doctor(&[]);
  assert!(
    verbose.lines().count() > default.lines().count(),
    "`doctor --verbose` produced no more than the default; the flag is accepted and does nothing"
  );
}

/// v2's rule, carried rather than re-decided. See this file's header.
#[test]
fn quiet_wins_over_verbose() {
  let both = doctor(&["--quiet", "--verbose"]);
  for line in RESOLVED {
    assert!(
      !both.contains(line),
      "`doctor --quiet --verbose` printed `{line}`; v2 resolves the pair to QUIET (bin/intent_doctor:134) and v3 must resolve it the same way or the flag has been re-designed rather than carried\n{both}"
    );
  }
  // Paired, so it cannot pass by producing nothing at all.
  assert!(
    both.contains(VERDICT),
    "`doctor --quiet --verbose` printed neither the resolution lines nor the verdict -- this arm must prove quiet WON, not that the command died\n{both}"
  );
}
