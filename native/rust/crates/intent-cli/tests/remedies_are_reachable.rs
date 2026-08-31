//! AT-06.11 / AC-06.11: **every remedy the binary suggests names something the
//! binary can actually do.**
//!
//! # The discriminating case, which is the whole reason this file exists
//!
//! **THE WALK IS OVER THE REMEDY STRINGS THE BINARY EMITS, NEVER THE VERBS IT
//! DECLARES.** A test asserting every declared verb exists passes trivially:
//! measured 2026-08-31, 24 distinct command references harvested from source
//! all resolved in the dispatch table and none was retired. **That clean
//! result is the trivial pass the criterion warns about**, and ic's original
//! finding says why -- `upgrade` was declared retired and correctly absent, so
//! the declaration and the binary agreed with each other and both were wrong.
//!
//! **A STATIC SCAN IS ALSO THE WRONG POPULATION, AND ITS OWN POSITIVE CONTROL
//! SAID SO.** A scan of lines containing `remedy:` cannot see a DELEGATED
//! remedy: `FacadeError::Unmigrated(pending) => pending.remedy()` puts the
//! literal in a `Remedy` impl one call away, and `unmigrated_project.rs`
//! already asserts that refusal names `intent upgrade`. **The instrument could
//! not contain the one instance the estate already tests for**, which is what
//! sent this file to a dynamic harvest.
//!
//! # What the criterion actually forbids, in two shapes
//!
//! AC-06.11 was widened to ONE row deliberately: a remedy fails identically
//! whether it names a verb that does not exist or a verb-SPACE that does not.
//! So both are checked here:
//!
//! - `intent <path>` -- the path must reach a WIRED verb.
//! - `intent <family> --help` -- the family must have at least one wired verb,
//!   or the remedy sends the reader to a list of things that do not work.
//!
//! **WIRED, NOT DECLARED, AND THAT DISTINCTION IS THE LIVE DEFECT.**
//! `render.rs`'s `unwired()` picks between those two remedies on
//! `has_verbs = .any(|e| e.verb().is_some() && e.is_shipped())`, and
//! `is_shipped()` is `disposition != "retire"` -- **a property of the
//! declaration, while the sentence promises `the verbs that ARE`.** ic closed
//! the zero-DECLARED-verbs case on 2026-08-15, when nine leaves sent readers to
//! an empty help block. The zero-WIRED-verbs case has the identical symptom,
//! the identical harm, and the same predicate cannot see it.
//!
//! # Why the probe supplies arguments
//!
//! **rc=1 FROM CLAP IS NOT EVIDENCE OF A WIRED VERB** (dc, 2026-08-31).
//! Argument validation runs BEFORE the unwired dispatch, so `config get` with
//! no argument answers `required arguments were not provided`, which reads
//! exactly like a wired subverb demanding input. The real refusal is only
//! reachable by satisfying clap first. That trap is encoded in [`wiredness`]
//! rather than remembered.

mod common;

use std::collections::BTreeSet;
use std::process::Command;

/// The unwired marker, **RE-TYPED AND NEVER IMPORTED**.
///
/// `render.rs` exports it as `pub const UNWIRED_PHRASE`, and importing it here
/// would retire this file: an assertion that imports the value it asserts moves
/// with the value and passes on any wording at all, including garbage. The
/// phrase is a published contract -- `guide.rs` renders it into user
/// documentation and the shipped pre-commit gate's fail-open branch is written
/// against it -- so a wording change must be a deliberate multi-file edit.
const UNWIRED: &str = "is a known command that is not implemented yet";

/// An argument that satisfies clap and can resolve to nothing.
const SENTINEL: &str = "ST9999";

/// Paths this test WILL NOT INVOKE, each with the reason it is refused.
///
/// **DECLARED AND LOUD, NEVER A SILENT SKIP.** A sweep that is safe only
/// because its targets happen to be unbuilt gets more dangerous as the estate
/// gets more complete, so the exclusions are named and the reasons are the
/// point. Two of these are standing rules that outrank any test's convenience.
const FORBIDDEN: &[(&str, &str)] = &[
  (
    "fc",
    "IN-AG-FIAT-001: fiat close is the human's verb. Not run here, not in a script, not to \
     measure something else. Every invocation is attributable and this file will not be the \
     reason one exists",
  ),
  (
    "daemon start",
    "takes the store exclusively and refuses every live peer node's store verbs at once -- a \
     test that does this to a shared machine is a worse defect than the one it checks",
  ),
  (
    "daemon stop",
    "daemon lifecycle on a machine with live peers",
  ),
  (
    "daemon run",
    "daemon lifecycle on a machine with live peers",
  ),
  (
    "claude upgrade",
    "writes `.claude/settings.json` and reaches the Intent install, which is outside the fixture \
     this test controls",
  ),
];

/// Command references that do NOT reach something the binary can do, TODAY.
///
/// **A RATCHET AND NOT AN EXCUSE LIST**, on `flag_reachability.rs`'s ratified
/// precedent: the comparison below is EQUALITY, so fixing one of these makes
/// the sets stop matching and whoever fixed it must shrink this list. It cannot
/// decay into a record of things repaired long ago.
///
/// **THE GATE IS ON ADDITIONS, WHICH IS THIS ESTATE'S OWN RULE** -- the clock
/// guard and `canon_commit_check.sh` both block on what a change ADDS and never
/// on inherited breakage. A check that refuses the whole standing set on every
/// run is the guard that must be bypassed, and a guard that must be bypassed is
/// one nobody keeps.
///
/// **IT WAS TWO AND IT IS NOW EMPTY, CLOSED BY THE PREDICATE RATHER THAN BY THE
/// RULING** (AT-06.11, 2026-08-31). Both entries were `config --help` and
/// `ext --help`: `unwired()` decided its remedy by asking the dispatch TABLE
/// whether the family declared any shipped verb, while the sentence it prints
/// promises the verbs this build IMPLEMENTS. hv ruled the same day that
/// `config` and `ext` SHIP declared-and-unbuilt in 3.0.1, so the families stay
/// exactly as they are -- what changed is that their remedy stopped routing the
/// operator into a help block where nothing works.
///
/// **THE SPECIFICATION QUESTION THIS LIST RECORDED AS OPEN DISSOLVED RATHER
/// THAN BEING ANSWERED.** It read: narrow the help and "for the verbs that are"
/// points at an empty list, which is ic's original zero-verbs defect by another
/// road. That is true, and it presumed a sentence had to be written. The
/// else-branch already said the right thing, so a correct predicate routes
/// these two into an EXISTING branch and nothing new was specified.
///
/// **EMPTY AND KEPT, NOT DELETED.** The comparison below is EQUALITY, so an
/// empty roster is a live ratchet: the next remedy that cannot be acted on reds
/// this arm on the day it is written. Deleting the list would retire the check
/// at the moment it first had nothing to excuse.
const INHERITED_UNREACHABLE: &[&str] = &[];

fn binary() -> &'static str {
  env!("CARGO_BIN_EXE_intent")
}

/// Whether a command path reaches a verb the binary actually implements.
///
/// **ARGUMENTS ARE ADDED UNTIL CLAP STOPS COMPLAINING, WHICH IS dc's TRAP AS
/// MECHANISM.** Validation runs before the unwired dispatch, so a bare probe of
/// a verb with required arguments returns a usage error that is indistinguishable
/// by exit code from a wired verb refusing input. The loop is bounded at three
/// sentinels; a verb needing more than that is reported inconclusive rather than
/// guessed at, because a guess here is invisible inside a green.
fn wiredness(fx: &Fixture, path: &str) -> Wired {
  match drive(fx, path) {
    None => Wired::Inconclusive,
    Some(text) if unwired_refusal(&text).is_some() => Wired::No,
    Some(_) => Wired::Yes,
  }
}

/// Drive one declared path until clap stops asking for arguments, and hand back
/// what the binary said.
///
/// **IT GOES THROUGH `Fixture::run`, WHICH IS THE WHOLE REASON IT EXISTS.** The
/// sentinel loop used to spawn `Command::new(binary())` directly with only
/// `current_dir` set, so roughly a hundred and ten drives per run reached the
/// REAL `$HOME` while `Fixture` fixtured the other two spawn sites beside them.
/// `table_driven_tests_fixture_their_home.rs` was green throughout and correctly
/// so: its `fixtures_home` predicate is FILE-scoped (`src.contains(".env(\"HOME\"")`),
/// and this file does contain that call -- in `Fixture`, not here. **The guard's
/// name covers this; its predicate cannot see inside a file.** Reported to dc,
/// whose file that is. Folding every spawn into one home is what makes the fix
/// hold rather than the fix being applied twice and drifting once.
fn drive(fx: &Fixture, path: &str) -> Option<String> {
  let mut argv: Vec<String> = path.split_whitespace().map(str::to_string).collect();
  for _ in 0..4 {
    let text = fx.run(&argv);
    if unwired_refusal(&text).is_some() || !text.contains("required arguments were not provided") {
      return Some(text);
    }
    argv.push(SENTINEL.to_string());
  }
  None
}

/// The unwired refusal AS EMITTED, with its remedy -- which is not the same
/// thing as the phrase appearing somewhere in the output.
///
/// **`intent llm guide` IS THE REASON THIS IS NOT A `contains`.** It is wired,
/// and it renders the agent guide, which DOCUMENTS the unwired phrase verbatim
/// and carries `remedy:` lines of its own. A whole-output `contains` therefore
/// reported a built verb as unwired and then paired the marker with a remedy
/// from an unrelated paragraph -- `"satisfy or formally descope the remaining
/// criteria"`, measured 2026-08-31. **The estate has met this precision before**
/// (`the_binary_under_test_is_the_one_cargo_built.rs`): a mention is not an
/// instance, and a check that reds the documentation of the hazard it guards
/// against is one nobody keeps.
///
/// The refusal is two lines in a fixed order, so they are read as a PAIR. Taking
/// the first `remedy:` anywhere in the output is what let the two come from
/// different messages.
fn unwired_refusal(text: &str) -> Option<String> {
  let lines: Vec<&str> = text.lines().collect();
  let at = lines
    .iter()
    .position(|l| l.trim_start().starts_with("error: ") && l.contains(UNWIRED))?;
  Some(
    lines
      .get(at + 1)
      .and_then(|l| l.trim_start().strip_prefix("remedy: "))
      .unwrap_or("")
      .trim()
      .to_string(),
  )
}

#[derive(PartialEq, Debug)]
enum Wired {
  Yes,
  No,
  Inconclusive,
}

/// A real project, in a temp dir, with a fixtured `HOME`.
///
/// **`HOME` IS FIXTURED SO THIS TEST CANNOT REACH THE OPERATOR'S ESTATE.** Some
/// of the paths driven below write; all of them resolve a project. A sweep of
/// 130-odd verbs that inherited the ambient `HOME` would be reaching into
/// whatever the machine happens to hold.
struct Fixture {
  dir: tempfile::TempDir,
  home: tempfile::TempDir,
}

impl Fixture {
  fn new() -> Fixture {
    let dir = tempfile::tempdir().expect("tempdir");
    let home = tempfile::tempdir().expect("home");
    let out = Command::new(binary())
      .args(["init", "R"])
      .current_dir(dir.path())
      .env("HOME", home.path())
      .output()
      .expect("init runs");
    assert!(
      out.status.success(),
      "the fixture project did not initialise, so every drive below would measure `no project` \
       rather than the verb: {}",
      String::from_utf8_lossy(&out.stderr)
    );
    Fixture { dir, home }
  }

  fn run(&self, argv: &[String]) -> String {
    let out = Command::new(binary())
      .args(argv)
      .current_dir(self.dir.path())
      .env("HOME", self.home.path())
      .output()
      .expect("the binary runs");
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    )
  }
}

/// Every remedy line the binary emits across the declared surface.
fn emitted_remedies(fx: &Fixture) -> BTreeSet<String> {
  let forbidden: BTreeSet<&str> = FORBIDDEN.iter().map(|(p, _)| *p).collect();
  let mut out = BTreeSet::new();
  for path in common::declared_paths() {
    if forbidden.contains(path.as_str()) {
      continue;
    }
    let argv: Vec<String> = path.split_whitespace().map(str::to_string).collect();
    for line in fx.run(&argv).lines() {
      if let Some(rest) = line.trim_start().strip_prefix("remedy: ") {
        out.insert(rest.trim().to_string());
      }
    }
  }
  out
}

/// The `intent ...` command references inside a remedy.
///
/// **BACKTICKED SPANS ONLY, AND THE FIRST CUT WAS GREEDY IN A WAY A GREEN
/// WOULD HAVE HIDDEN.** Stripping the backticks and then consuming
/// lowercase words turns *run `intent upgrade` to migrate this project* into
/// the reference `upgrade to migrate this project to` -- which matches no
/// declared path, so it is silently dropped and **the remedy reads as carrying
/// no command reference at all.** The property arm would then pass by having
/// nothing to check. `the_harvest_reaches_a_remedy_the_estate_asserts_elsewhere`
/// is what caught it, on exactly the instance it was planted for.
///
/// **THE LIMIT, STATED: a command named OUTSIDE backticks is invisible here.**
/// That is not left to trust -- [`every_command_reference_is_backticked`]
/// requires the convention to hold across the whole emitted corpus, so the
/// extractor's population and the real one are the same set by measurement
/// rather than by assumption.
fn references(remedy: &str) -> Vec<String> {
  let mut out = Vec::new();
  for span in remedy.split('`').skip(1).step_by(2) {
    let words: Vec<&str> = span.split_whitespace().collect();
    let Some(rest) = words.strip_prefix(&["intent"]) else {
      continue;
    };
    let mut cmd: Vec<&str> = Vec::new();
    let mut help = false;
    for w in rest {
      if *w == "--help" {
        help = true;
        break;
      }
      if !w.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
        break;
      }
      cmd.push(w);
    }
    if cmd.is_empty() && !help {
      continue;
    }
    out.push(if help {
      format!("{} --help", cmd.join(" ")).trim().to_string()
    } else {
      cmd.join(" ")
    });
  }
  out
}

/// **THE PROPERTY: every emitted remedy names something the binary can do.**
#[test]
fn every_emitted_remedy_names_something_this_build_can_do() {
  let fx = Fixture::new();
  let declared: BTreeSet<String> = common::declared_paths().into_iter().collect();
  let forbidden: BTreeSet<&str> = FORBIDDEN.iter().map(|(p, _)| *p).collect();

  let mut unreachable: BTreeSet<String> = BTreeSet::new();
  for remedy in emitted_remedies(&fx) {
    for r in references(&remedy) {
      if let Some(family) = r.strip_suffix(" --help") {
        // A verb-SPACE reference: the family must have at least one WIRED verb,
        // or the remedy sends the reader to a list of things that do not work.
        let verbs: Vec<String> = declared
          .iter()
          .filter(|p| p.starts_with(&format!("{family} ")))
          .cloned()
          .collect();
        let any_wired = verbs
          .iter()
          .filter(|p| !forbidden.contains(p.as_str()))
          .any(|p| wiredness(&fx, p) == Wired::Yes);
        if !verbs.is_empty() && !any_wired {
          unreachable.insert(r.clone());
        }
      } else if declared.contains(&r) && !forbidden.contains(r.as_str()) {
        if wiredness(&fx, &r) == Wired::No {
          unreachable.insert(r.clone());
        }
      }
    }
  }

  let inherited: BTreeSet<String> = INHERITED_UNREACHABLE
    .iter()
    .map(|s| s.to_string())
    .collect();
  assert_eq!(
    unreachable,
    inherited,
    "the set of remedies that do not reach a working verb has changed.\n  \
     ADDED (a remedy now sends an operator somewhere they cannot get to): {:?}\n  \
     FIXED (shrink INHERITED_UNREACHABLE to match): {:?}",
    unreachable.difference(&inherited).collect::<Vec<_>>(),
    inherited.difference(&unreachable).collect::<Vec<_>>()
  );
}

/// **THE HARVEST IS NON-EMPTY AND CONTAINS AN INSTANCE THIS FILE DID NOT
/// CHOOSE.**
///
/// **THE CONTROL IS DRAWN FROM A DIFFERENT ENUMERATION THAN THE INSTRUMENT**,
/// which is `flag_reachability.rs`'s hardest-won lesson: a control built from
/// the same list as the scanner can only confirm the shapes that list already
/// has, and is structurally unable to find a missing one. So the instance
/// pinned here is one the ESTATE asserts elsewhere -- `unmigrated_project.rs`
/// requires the unmigrated refusal to name `intent upgrade` -- and not one this
/// file's own regex found.
///
/// **IT IS ALSO THE EXACT INSTANCE A STATIC SCAN COULD NOT SEE**, because that
/// remedy is delegated through a `Remedy` impl and never appears on a line
/// containing `remedy:`. If the harvest ever stops reaching delegated remedies,
/// this arm is what says so.
#[test]
fn the_harvest_reaches_a_remedy_the_estate_asserts_elsewhere() {
  let fx = Fixture::new();
  let remedies = emitted_remedies(&fx);
  assert!(
    !remedies.is_empty(),
    "the sweep emitted no remedies at all, so the property arm beside this one is asserting over \
     an empty set and would pass on any binary whatever"
  );

  let unmigrated = tempfile::tempdir().expect("tempdir");
  std::fs::create_dir_all(unmigrated.path().join("intent/.config")).expect("mkdir");
  std::fs::write(
    unmigrated.path().join("intent/.config/config.json"),
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"U\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  )
  .expect("write config");
  let out = Command::new(binary())
    .args(["st", "list"])
    .current_dir(unmigrated.path())
    .env("HOME", fx.home.path())
    .output()
    .expect("runs");
  let text = String::from_utf8_lossy(&out.stderr).into_owned();
  assert!(
    text.contains("remedy: "),
    "the unmigrated refusal carries no remedy, so the delegated-remedy shape this file exists to \
     reach has stopped existing: {text}"
  );
  assert!(
    references(&text).iter().any(|r| r == "upgrade"),
    "the reference extractor did not find `intent upgrade` in the unmigrated refusal, which \
     `unmigrated_project.rs` asserts is there -- so the extractor, not the estate, is what \
     changed: {text}"
  );
}

/// **THE PROBE CAN SAY NO, DRIVEN ON BOTH VERDICTS.**
///
/// The property arm passes by finding a set that MATCHES. A `wiredness` that
/// answered `Yes` unconditionally would produce an empty unreachable set, which
/// matches nothing and reds -- but one that answered `No` unconditionally, or
/// that could not tell them apart, would produce noise that happens to be
/// filtered. So both verdicts are pinned against known members of each class.
/// The two remedies, RE-TYPED, for the same reason `UNWIRED` above is.
const ROOT_FORM: &str = "nothing in this build provides it";
const FAMILY_FORM: &str = "for the verbs that are";

/// **THE CALL SITE IS A DECLARATION, AND THIS IS WHAT KEEPS IT HONEST.**
///
/// vc's ruling, 2026-08-31: wiredness is DERIVED, or declared by a test that
/// DRIVES every declared-unwired path and asserts the marker -- never a
/// hand-maintained flag. `unwired()` no longer computes whether a family has
/// wired verbs, because the binary cannot enumerate its own match arms; which
/// remedy applies is decided by WHERE the call is, and `render.rs` argues that
/// position is structural.
///
/// **That argument is sound today and no reading of `render.rs` can confirm it
/// stayed sound.** A family that keeps its handler while its last wired verb is
/// removed takes the family branch and points at nothing -- exactly what
/// `config` and `ext` did, with nothing in the source looking wrong. It is the
/// shape a static compare cannot see, so it is driven instead.
///
/// **BOTH VERDICTS ARE REQUIRED TO OCCUR**, because a census that only ever
/// reaches one branch confirms nothing about the other and reads identically to
/// one that checks both.
#[test]
fn the_remedy_a_family_emits_matches_the_verbs_it_actually_wires() {
  let fx = Fixture::new();
  let forbidden: BTreeSet<&str> = FORBIDDEN.iter().map(|(p, _)| *p).collect();

  let mut families: BTreeSet<String> = BTreeSet::new();
  for path in common::declared_paths() {
    if let Some((family, _)) = path.split_once(' ') {
      families.insert(family.to_string());
    }
  }

  let (mut root_form, mut family_form, mut wrong, mut silent) =
    (Vec::new(), Vec::new(), Vec::new(), Vec::new());

  for family in &families {
    let verbs: Vec<String> = common::declared_paths()
      .into_iter()
      .filter(|p| p.starts_with(&format!("{family} ")) && !forbidden.contains(p.as_str()))
      .collect();

    let wired = verbs.iter().any(|p| wiredness(&fx, p) == Wired::Yes);

    // The remedy this family ACTUALLY prints, taken from the first of its own
    // paths that reaches the unwired door. A family every one of whose verbs is
    // built emits nothing here and is recorded rather than skipped in silence.
    let emitted = verbs
      .iter()
      .find_map(|p| Some((p.clone(), unwired_refusal(&drive(&fx, p)?)?)));

    let Some((path, remedy)) = emitted else {
      silent.push(family.clone());
      continue;
    };

    let expected = if wired { FAMILY_FORM } else { ROOT_FORM };
    let unexpected = if wired { ROOT_FORM } else { FAMILY_FORM };
    if remedy.contains(expected) && !remedy.contains(unexpected) {
      if wired {
        family_form.push(family.clone());
      } else {
        root_form.push(family.clone());
      }
    } else {
      wrong.push(format!(
        "{family} (via `{path}`): has a wired verb = {wired}, so the remedy must carry {expected:?} and not {unexpected:?} -- it said {remedy:?}"
      ));
    }
  }

  assert!(
    wrong.is_empty(),
    "a family's unwired remedy disagrees with the verbs it actually wires, which is AT-06.11 returning:\n  {}",
    wrong.join("\n  ")
  );

  // **THE POPULATION BEFORE THE PROPERTY.** If the walk stops reaching the
  // binary, every check above passes on nothing and says the word it says when
  // the estate is clean.
  assert!(
    !root_form.is_empty() && !family_form.is_empty(),
    "this census must reach BOTH remedies or it confirms neither -- root-form: \
     {root_form:?}, family-form: {family_form:?}, emitting nothing: {silent:?}"
  );
}

#[test]
fn the_wiredness_probe_separates_a_built_verb_from_a_declared_one() {
  let fx = Fixture::new();
  assert_eq!(
    wiredness(&fx, "st list"),
    Wired::Yes,
    "`st list` is built and driven all over this estate; a probe calling it unwired cannot be \
     trusted about anything"
  );
  assert_eq!(
    wiredness(&fx, "config get"),
    Wired::No,
    "`config get` is declared and unbuilt (hv, 2026-08-31), and reaching that verdict REQUIRES \
     satisfying clap first -- a bare probe answers `required arguments were not provided`, which \
     reads exactly like a wired subverb demanding input"
  );
}

/// **EVERY REFUSED PATH CARRIES A REASON, AND THE REASONS ARE NOT DECORATION.**
///
/// A skip list with no author and no date is a judgement nobody made. This arm
/// requires each exclusion to name a real declared path and to say something,
/// so the set cannot quietly grow into a way of making the sweep green.
#[test]
fn every_refused_path_is_declared_and_carries_its_reason() {
  let declared: BTreeSet<String> = common::declared_paths().into_iter().collect();
  for (path, why) in FORBIDDEN {
    assert!(
      declared.contains(*path),
      "`{path}` is refused by this test and the dispatch table does not declare it -- an \
       exclusion that names nothing excludes nothing"
    );
    assert!(
      why.len() > 40,
      "`{path}` is refused without a reason worth reading, which is how a safety exclusion \
       becomes a convenience one"
    );
  }
}

/// **THE EXTRACTOR'S POPULATION IS THE REAL ONE, MEASURED RATHER THAN
/// ASSUMED.**
///
/// [`references`] reads backticked spans only, which is a deliberate narrowing
/// -- the greedy alternative swallows English prose and drops the reference
/// entirely. **That narrowing is only sound while every emitted remedy actually
/// backticks its commands**, and a convention nothing checks is a convention
/// until the day it is not. So this arm walks the same corpus the property arm
/// walks and refuses any remedy that names `intent ` outside a backtick span.
///
/// Without it the property arm has a silent blind spot exactly the size of
/// whatever nobody remembered to quote.
#[test]
fn every_command_reference_is_backticked() {
  let fx = Fixture::new();
  let mut bare: Vec<String> = Vec::new();
  for remedy in emitted_remedies(&fx) {
    // Outside-backtick text is the even-indexed halves of the split.
    let outside: String = remedy.split('`').step_by(2).collect::<Vec<_>>().join(" ");
    if outside.contains("intent ") {
      bare.push(remedy);
    }
  }
  assert!(
    bare.is_empty(),
    "these remedies name a command outside backticks, which `references` cannot see -- so the \
     property arm is blind to whatever they name:\n  {}",
    bare.join("\n  ")
  );
}
