//! **AT-06.8 / AC-06.8: every flag the surface DECLARES is READ by the renderer.**
//!
//! A declared-and-inert flag is a lie told by `--help`, and it is worse than an
//! absent feature: help lists it, the parser accepts it, the exit code is 0.
//! **An inert flag is indistinguishable from a working one at the surface an
//! operator actually reads.**
//!
//! # Why this walks the surface instead of checking a list
//!
//! AC-06.8 was raised from a census of five live instances, and that census
//! MISSED `st new -s`. The reason is mechanical rather than diligence-shaped
//! and it governs this file's whole design: the long spelling is `start`, and
//! that string is all over the renderer as a VERB, so a search found the
//! spelling asked for while the claim had another one. **A hand-listed set is
//! the census that missed it. So the population comes from the table.**
//!
//! # What "reads it" means here, and the limit of that
//!
//! A renderer reads a flag by spelling its id into an `ArgMatches` accessor --
//! `get_flag("quiet")`, `opt(a, "path")`, `get_one::<String>("lang")`. This
//! file collects those spelled ids out of the renderer source and asks whether
//! each declared id is among them.
//!
//! **THE LIMIT, STATED RATHER THAN DISCOVERED: this is whole-file, so it
//! establishes that SOMETHING reads the id and not that the RIGHT arm does.**
//! A flag declared on `export` and read only in `import`'s arm would pass. That
//! is weaker than the criterion's ideal and it is exactly the criterion's
//! words -- *fails naming any flag no renderer arm reads* -- so the gap is
//! between this check and a stronger one, not between this check and the row.
//! Narrowing it needs the arm boundaries, which needs parsing Rust rather than
//! scanning it, and that trade is recorded here so the next reader does not
//! have to re-derive why it was not taken.
//!
//! # The gate line, and why it is not the whole population
//!
//! Flags on a family with NO renderer arm are reported and NOT gated. There is
//! nothing there to read them yet, they arrive one at a time as each command is
//! wired, and gating them would make this a permanently-red check over a
//! backlog nobody can clear in one commit -- the guard that must be bypassed,
//! which is the guard nobody keeps. **They are NAMED on every run**: a silent
//! cap reads as complete when it is not.
//!
//! Whether a family has an arm is decided BEHAVIOURALLY, by running it, not by
//! searching the source, because **a capability arriving under an unlisted name
//! is invisible to a name check.** Established in `declared_but_unwired.rs` and
//! now driven in `cli_write_moves_only_what_changed.rs` as well -- the reason
//! stands on its own and is cited here to files that hold it, so it does not
//! dangle when one of them retires.

use std::collections::BTreeSet;
use std::process::Command;

use intent_cli::dispatch;

/// The phrase `render::unwired` emits.
///
/// **Duplicated as a literal, deliberately, and the duplication is the point:
/// if the wording changes, every file asserting it must notice, and a shared
/// constant in one of them would make every other copy look derived.** No copy
/// is authoritative and none may be promoted to it.
///
/// **THE SIBLING COPIES ARE NAMED RATHER THAN THE ORIGIN, AND THAT IS A
/// CORRECTION** (cc, 2026-08-21, on ic's catch). This comment used to read
/// *"duplicated from `declared_but_unwired.rs`"*, which made that file the
/// origin -- so retiring it would not have removed a copy, it would have
/// PROMOTED the survivor to authoritative, the exact outcome the reasoning
/// exists to prevent. **A deletion that looks like it only touches a loop
/// would have destroyed a load-bearing property, and nothing we own tracks a
/// dependency recorded in a doc comment**: `at lint` sees AT-row citations and
/// sees nothing here. Live copies today are `declared_but_unwired.rs` and
/// `cli_write_moves_only_what_changed.rs` (AT-03.19); **the invariant is that
/// at least two survive any retirement**, not that any particular file does.
const UNWIRED: &str = "is a known command that is not implemented yet";

/// The renderer's source, read rather than compiled in.
///
/// `include_str!` would freeze it at this file's compile time, which is the
/// same thing but harder to explain; reading it makes the subject obvious.
fn renderer_source() -> String {
  let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/render.rs");
  std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("could not read the renderer at {}: {e}", path.display()))
}

/// Every arg id the renderer SPELLS into a `clap` accessor.
///
/// Scanned rather than parsed. Each marker below opens an accessor whose id is
/// the next quoted string; the window bounds how far to look so a marker
/// followed by no literal cannot swallow an unrelated string from three lines
/// down. `get_one::<String>(` and `opt(a, ` put different numbers of characters
/// between the marker and the quote, which is why this takes the next quote
/// rather than a fixed offset.
///
/// **SECOND LIMIT, DRIVEN RATHER THAN ASSUMED: A COMMENTED ACCESSOR COUNTS AS A
/// READ.** Measured by planting a `keep` flag on `doctor` whose only mention in
/// the renderer was the text `get_flag("comment-probe")` inside a `//` comment
/// -- the check did not report it. So a flag can be marked read by prose about
/// it, and a doc comment explaining why a flag is NOT read would be the worst
/// case: the explanation would silence the finding it explains.
///
/// Not fixed, and the reason is the same trade as the whole-file limit above:
/// distinguishing code from comment needs a Rust parser rather than a scan.
/// Recorded because the failure is silent from a green -- **which is the exact
/// property this file exists to attack, present in the file itself.**
fn ids_the_renderer_reads(src: &str) -> BTreeSet<String> {
  // **`flag(` IS THE FOURTH MARKER AND ITS ABSENCE WAS COSTING TEN OF EIGHTEEN**
  // (ic, 2026-08-20). `render.rs` reads most of its booleans through its own
  // `fn flag(m: &ArgMatches, name: &str)` helper -- ELEVEN call sites against
  // FOUR direct `get_flag("..")` -- and the helper's own body spells
  // `try_get_one::<bool>(name)` with a VARIABLE, so nothing inside it carries
  // the id. Every flag read the majority way was therefore invisible, and the
  // check reported the id as unread with no way to tell that from a real one.
  //
  // **A SUBSTRING STANDING IN FOR A SYNTACTIC FACT IS ST0039's GREPPABLE PROXY
  // ONE LEVEL UP**, and the tell was in the baseline: `st new -s / --start` led
  // the inherited list as the instance AC-06.8's census MISSED, and it is read
  // at `render.rs:587`. **The headline example was the false positive.**
  //
  // `get_flag(` ENDS with `flag(`, so the two markers overlap and find the same
  // id twice -- harmless into a set, and cheaper than a boundary rule.
  const MARKERS: [&str; 5] = ["get_flag(", "flag(", "get_one::<", "get_many::<", "opt("];
  const WINDOW: usize = 48;
  let mut out = BTreeSet::new();
  for marker in MARKERS {
    let mut from = 0;
    while let Some(hit) = src[from..].find(marker) {
      let start = from + hit + marker.len();
      let window = &src[start..src.len().min(start + WINDOW)];
      let quoted = window.find('"').and_then(|open| {
        window[open + 1..]
          .find('"')
          .map(|close| window[open + 1..open + 1 + close].to_string())
      });
      if let Some(id) = quoted {
        out.insert(id);
      }
      from = start;
    }
  }
  out
}

/// Families the binary answers with the unwired refusal.
fn unwired_families() -> BTreeSet<String> {
  let dir = tempfile::tempdir().expect("tempdir");
  let table = dispatch::table();
  let mut out = BTreeSet::new();
  for family in &table.families {
    let Some(entry) = family.entries.iter().find(|e| e.verb().is_none()) else {
      continue;
    };
    if !entry.is_shipped() {
      continue;
    }
    let output = Command::new(env!("CARGO_BIN_EXE_intent"))
      .arg(&family.name)
      .current_dir(dir.path())
      .output()
      .unwrap_or_else(|e| panic!("could not run `intent {}`: {e}", family.name));
    let said = String::from_utf8_lossy(&output.stderr).into_owned()
      + &String::from_utf8_lossy(&output.stdout);
    if said.contains(UNWIRED) {
      out.insert(family.name.clone());
    }
  }
  out
}

/// **The scanner is validated before anything is concluded from it.**
///
/// Every assertion in this file rests on `ids_the_renderer_reads` finding what
/// is there, and a scanner that found NOTHING would report the entire surface
/// as unread -- a spectacular, obviously-wrong failure. The dangerous version
/// is a scanner that finds MOST ids: it would then report a handful of real
/// reads as violations and look like a genuine finding.
///
/// So this pins ids reached through all four accessor shapes, each spelled a
/// different distance from its marker.
#[test]
fn the_scanner_finds_ids_that_are_demonstrably_read() {
  let ids = ids_the_renderer_reads(&renderer_source());
  for known in ["quiet", "verbose", "languages", "staged", "lang"] {
    assert!(
      ids.contains(known),
      "the scanner missed `{known}`, which the renderer demonstrably reads -- every verdict in this file is unsafe until this passes\n  found {} id(s): {ids:?}",
      ids.len()
    );
  }
  assert!(
    ids.len() > 20,
    "the scanner found only {} id(s); the renderer carries 36 accessor sites, so this is a scanning failure rather than a finding about the surface",
    ids.len()
  );

  // **THE ARM THAT WOULD HAVE CAUGHT ic's FINDING, AND THE TWO ABOVE COULD
  // NOT.** Both of those are drawn from the same enumeration as the scanner --
  // five ids I knew were read through shapes I had already listed as markers,
  // and a floor low enough to clear with a whole idiom missing. A control the
  // instrument's author chose cannot find a shape the author did not know
  // about.
  //
  // This one is chosen by the SOURCE instead: every line that spells an
  // accessor must yield an id. A marker set missing an idiom leaves that
  // idiom's call sites contributing nothing, and the count of silent lines
  // jumps. Before `flag(` was added it would have been TWELVE.
  //
  // **THE EXCEPTION IS A PROPERTY, NOT A BUDGET, AND THE FIRST CUT OF THIS ARM
  // WAS THE BUDGET.** I wrote `silent.len() <= 1`, drove it, and got FIVE --
  // the `opt` and `flag` helpers' signatures and bodies, plus one `match`.
  // Every one is legitimate for the same reason: **the id is spelled as the
  // variable `name`, so there is no literal to find.** Raising the number to
  // five would have been a budget with no reason attached, and a budget of five
  // hides the next four. Asserting the REASON leaves no room: a call site the
  // markers miss is silent AND carries a literal, so it still fails.
  let src = renderer_source();
  let markers = ["get_flag(", "flag(", "get_one::<", "get_many::<", "opt("];
  let unexplained: Vec<&str> = src
    .lines()
    .filter(|line| markers.iter().any(|m| line.contains(m)))
    .filter(|line| ids_the_renderer_reads(line).is_empty())
    // **EXEMPT BY THE PROPERTY, NOT BY THE PARAMETER'S NAME.** The first cut
    // filtered on `name: &str` and `(name)`, which is the two existing helpers'
    // spelling rather than their shape -- and it broke within the hour, when a
    // new `init` helper spelled the same idea with a parameter called `flag`.
    // **A predicate written around the instances in front of you is a list with
    // a filter's syntax.**
    //
    // The general form: a silent accessor line carrying NO string literal is
    // spelling its id as a variable, which nothing can extract and nothing
    // should. One carrying a literal is a marker the scanner does not know --
    // which is the ten-false-positive case, and it still fails here.
    .filter(|line| line.contains('"'))
    .collect();
  assert!(
    unexplained.is_empty(),
    "{} accessor line(s) yielded no id and do NOT spell their argument as a variable -- a marker set that misses an idiom leaves its call sites silent, which is how ten false positives reached the baseline:\n  {}",
    unexplained.len(),
    unexplained
      .iter()
      .map(|l| l.trim())
      .collect::<Vec<_>>()
      .join("\n  ")
  );
}

/// **THE INHERITED SET, AND IT IS A RATCHET RATHER THAN AN EXCUSE LIST.**
///
/// Eighteen flags on wired families are declared and unread TODAY. They are not
/// this check's to fix: AC-06.8 says in its own words that the remedy for each
/// instance is a SCOPE DECISION and not a wiring job -- `st list --markdown`
/// might be built or might be withdrawn, and a test cannot make that call.
///
/// **So the gate is on ADDITIONS, which is this estate's own ratified rule**
/// (the clock guard and `canon_commit_check.sh` both block on what a change
/// ADDS and never on inherited breakage) -- a check that refuses eighteen
/// pre-existing instances on every run is the guard that must be bypassed,
/// which is the guard nobody keeps.
///
/// **THE COMPARISON IS EQUALITY, NOT CONTAINMENT, AND THAT IS WHAT MAKES IT A
/// RATCHET.** Wire one of these and the sets stop matching, so the test goes
/// red and whoever fixed it must shrink this list. It cannot silently become a
/// list of things that were fixed years ago, which is what an excuse list
/// decays into.
///
/// **THIS IS NOT THE BORROWED-INSTANCE DEFECT** (vc's ruling, which governs the
/// row above it): the discrimination lives in
/// `the_check_separates_a_read_id_from_an_unread_one`, and BOTH its instances
/// are synthetic. This list is a baseline, not a fixture -- if it reaches
/// empty, that arm still proves the check can tell the two apart, and the
/// criterion is simply met. **A ratchet that can reach zero is the shape a
/// borrowed fixture is not.**
///
/// **`st new -s` LED THIS LIST FOR AN HOUR AS THE PROOF THE CHECK WORKED, AND
/// IT WAS THE PROOF THE CHECK WAS BROKEN.** The comment here read *it is the
/// instance AC-06.8's own census MISSED, and this check found it by walking the
/// table rather than by being told to look* -- a satisfying sentence about a
/// flag that is read at `render.rs:587`. ic caught it (2026-08-20).
///
/// **THE CONTROL PASSED AND COULD NOT HAVE FAILED.**
/// `the_scanner_finds_ids_that_are_demonstrably_read` pins five ids and a floor
/// of twenty, and every one of those five is read through a DIRECT accessor --
/// because I built the control by listing the accessor shapes I had already
/// enumerated for the scanner. **A control drawn from the same enumeration as
/// the instrument can only confirm the shapes the enumeration already has; it
/// is structurally unable to find a missing one.** The five markers and the
/// five control ids were one list wearing two hats.
///
/// **AND THE MISSING SHAPE WAS THE MAJORITY ONE.** `render.rs` reads most
/// booleans through its own `fn flag(m, name)` helper -- eleven sites against
/// four direct -- so the scanner was blind to the common idiom and saw only the
/// exception. The right control is one the instrument's author did not choose:
/// count the accessor SITES in the source and require the ids found to be of
/// the same order. Thirty-six sites yielding twenty-odd ids should have been
/// the question, and the floor of twenty was set low enough to pass it.
const INHERITED_UNREAD: &[&str] = &[
  // **MEASURED AT `a6e336a7`, THE FIRST REVISION IN HOURS THAT BUILDS**, with
  // `render.rs` and the dispatch table both clean in the working tree so the
  // measurement equals HEAD. Eighteen at the first cut, eight after ic
  // corrected the scanner's blind spot, nine as `init` wired, and FIVE once a
  // violation required both conditions.
  //
  // **THE POPULATION HALVED THREE TIMES AND NOT ONE REDUCTION WAS THE ESTATE
  // GETTING BETTER.** Every one was this instrument getting less wrong, and
  // each was found by driving it against a real tree rather than by reading it.
  "`st bootstrap` --audit-only (id `audit-only`)",
  "`st bootstrap` --dry-run (id `dry-run`)",
  "`st bootstrap` --deliverable (id `deliverable`)",
  "`claude subagents` -v (id `v`)",
  "`claude skills` -v (id `v`)",
];

/// **THE CRITERION.** Every `keep` flag on a WIRED family is read by name.
/// **PARKED, LOUDLY, WITH A NAMED EXPIRY -- AND `#[ignore]` RATHER THAN A
/// RELAXED ASSERTION IS THE WHOLE POINT.** vc ruled an hour ago that relaxing a
/// gate at the moment it stops covering anything converts a refusal into a
/// silent pass. So this does not relax: it says NOT RUN, in every test run,
/// where a reader sees it.
///
/// **THE REASON IS THE TREE, NOT THE CHECK.** dc is landing `init` and
/// `bootstrap` right now. Each family they wire moves its flags out of the
/// ungated bucket and into the gated one, so the violation set changes with
/// every landing -- measured across one afternoon: 18, then 8 after ic's
/// scanner correction, then 9 as `init` wired. **A baseline frozen mid-landing
/// would red the workspace for four nodes on a number that was never stable.**
///
/// **EXPIRY, NAMED SO IT CANNOT BECOME PERMANENT: remove `#[ignore]` and set
/// `INHERITED_UNREAD` from one run against a settled tree, once dc's `init` /
/// `bootstrap` work is committed.** The other two arms are LIVE and gate now --
/// the scanner's self-check and the synthetic discrimination -- so what is
/// parked is the population, not the instrument.
///
/// **AT-06.8 IS THEREFORE NOT SATISFIED BY THIS COMMIT** and the row must not
/// be moved on the strength of it. The instrument exists and is proven; the
/// criterion is unmet until this arm runs.
#[test]
fn every_declared_flag_on_a_wired_family_is_read_by_the_renderer() {
  let table = dispatch::table();
  let src = renderer_source();
  let read = ids_the_renderer_reads(&src);
  let unwired = unwired_families();

  let mut violations = Vec::new();
  let mut deferred = Vec::new();
  let mut checked = 0;

  for family in &table.families {
    for entry in &family.entries {
      if !entry.is_shipped() {
        continue;
      }
      for flag in &entry.flags {
        // `intrinsic` is clap's own -- `--help` and friends. The spine
        // deliberately does not declare them and no renderer answers them, so
        // asking whether one is read is asking the wrong question.
        if !flag.ships() || flag.disposition == "intrinsic" {
          continue;
        }
        let Some(id) = flag.arg_id() else {
          continue;
        };
        let line = format!(
          "`{}` {} (id `{id}`)",
          entry.path,
          flag.spellings.join(" / ")
        );
        if unwired.contains(&family.name) {
          deferred.push(line);
        } else {
          checked += 1;
          // **TWO CONDITIONS, AND THE SECOND IS THE THIRD APERTURE THIS CHECK
          // HAS HAD TO GROW.** The accessor scan is the precise signal and it
          // cannot see an id spelled anywhere but a call site -- dc's `init`
          // loops over `[("with-st0000", ..), ("lang", ..)]` and passes each
          // through a variable, so the id is spelled at `render.rs:2194` and
          // read, and the scan called it unread.
          //
          // A whole-file literal scan alone would be the OPPOSITE mistake: it
          // is what would have called `st new -s` read because `start` is all
          // over the renderer as a verb, which is the census failure AC-06.8
          // was raised from.
          //
          // **So a violation requires BOTH: no accessor site AND no mention at
          // all.** That is strictly conservative -- neither idiom can produce a
          // false positive -- and it pays for it in false NEGATIVES, which are
          // named here rather than discovered: a flag whose id happens to
          // appear as an unrelated string passes. The precise signal is still
          // computed and still reported; only the GATE takes both.
          let mentioned = src.contains(&format!("\"{id}\""));
          if !read.contains(&id) && !mentioned {
            violations.push(line);
          }
        }
      }
    }
  }

  // **NAMED, NEVER COUNTED, AND NEVER CAPPED.** These are the flags this check
  // deliberately does not gate: their family answers the unwired refusal, so
  // there is no arm that could read them. Printing the list is what stops
  // "gated on the wired ones" being read as "gated on all of them".
  if !deferred.is_empty() {
    println!(
      "flag-reachability: {} flag(s) NOT GATED -- their family is unwired, so no arm could read them yet:",
      deferred.len()
    );
    for line in &deferred {
      println!("  {line}");
    }
  }
  println!("flag-reachability: {checked} flag(s) gated across the wired families");

  assert!(
    checked > 0,
    "no flag was gated at all -- every shipped family reported as unwired, which is a fact about this test rather than about the surface"
  );
  // Named on every run whether or not the gate fires, because the failure mode
  // is that an inert flag is indistinguishable from a working one -- and a
  // list that only appears on a red is invisible for exactly as long as
  // nothing changes.
  println!(
    "flag-reachability: {} flag(s) DECLARED and unread on a wired family:",
    violations.len()
  );
  for line in &violations {
    println!("  {line}");
  }

  let inherited: BTreeSet<&str> = INHERITED_UNREAD.iter().copied().collect();
  let found: BTreeSet<&str> = violations.iter().map(String::as_str).collect();

  let added: Vec<&&str> = found.difference(&inherited).collect();
  assert!(
    added.is_empty(),
    "these flags are DECLARED and no renderer arm reads them, and they are NOT in the inherited set -- `--help` advertises each one and the parser accepts it, so an operator cannot tell them from working flags:\n  {}",
    added.iter().map(|s| **s).collect::<Vec<_>>().join("\n  ")
  );

  let fixed: Vec<&&str> = inherited.difference(&found).collect();
  assert!(
    fixed.is_empty(),
    "these flags are in the inherited set and are now READ -- good news, and the list must shrink to match or it decays into an excuse list nobody prunes:\n  {}",
    fixed.iter().map(|s| **s).collect::<Vec<_>>().join("\n  ")
  );
}

/// **THE DISCRIMINATION, AND BOTH HALVES ARE SYNTHETIC.**
///
/// vc's ruling, 2026-08-20: an instrument's discrimination is a property of the
/// INSTRUMENT, never of the estate's current defect count. **Where a red-first
/// arm needs an instance of the defect, the instance is SYNTHETIC** -- one that
/// borrows a live instance has made the defect a fixture, and the estate is
/// then not free to fix it.
///
/// That ruling governs this row directly, because the declared-and-unread
/// population is exactly the kind that can legitimately reach zero: driving the
/// criterion to zero is the entire point of the criterion. A red-first arm
/// keyed on a live unread flag would panic on the day the last one is wired,
/// which is the failure `dispatch_ssot.rs` shipped and hit.
///
/// So neither flag here is real: one id that the renderer demonstrably reads,
/// one that nothing could read. **The check must separate them, and the second
/// must be reported BY NAME** -- naming is the whole remedy, since the failure
/// mode is that an inert flag looks like a working one.
#[test]
fn the_check_separates_a_read_id_from_an_unread_one() {
  let read = ids_the_renderer_reads(&renderer_source());

  // **The fixtures are CLONED from a live flag and re-spelled, rather than
  // constructed field by field.** A hand-built `Flag` would freeze this file's
  // idea of the struct's shape, so a field added to the table tomorrow would
  // leave the fixture subtly unlike the things it stands for -- and the whole
  // point of a synthetic instance is that it behaves like a real one.
  let table = dispatch::table();
  let template = dispatch::shipped_entries(&table)
    .into_iter()
    .flat_map(|e| e.flags.iter())
    .find(|f| f.disposition == "keep")
    .expect("the live table carries at least one `keep` flag to use as a template")
    .clone();

  let mut wired = template.clone();
  wired.spellings = vec!["--quiet".to_string(), "-q".to_string()];
  let mut inert = template;
  inert.spellings = vec!["--no-renderer-reads-this".to_string()];

  let wired_id = wired
    .arg_id()
    .expect("the synthetic wired flag has a spelling");
  let inert_id = inert
    .arg_id()
    .expect("the synthetic inert flag has a spelling");

  assert!(
    read.contains(&wired_id),
    "the synthetic READ flag (`{wired_id}`) was not found among the ids the renderer reads -- the positive half of the discrimination is broken, so a green from the criterion above proves nothing"
  );
  assert!(
    !read.contains(&inert_id),
    "the synthetic INERT flag (`{inert_id}`) was found among the ids the renderer reads, which means the scanner matches things that are not accessor ids"
  );
}
