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
//! Flags on an ENTRY the binary refuses as unwired are reported and NOT gated.
//! There is nothing there to read them yet, they arrive one at a time as each
//! command is wired, and gating them would make this a permanently-red check
//! over a backlog nobody can clear in one commit -- the guard that must be
//! bypassed, which is the guard nobody keeps. **They are NAMED on every run**:
//! a silent cap reads as complete when it is not. **The unit was the FAMILY
//! until 2026-08-31, and that was the live reason `AT-06.8` stayed red** (ic,
//! on cc's measurement and vc's note): an unwired verb under a wired root was
//! graded as wired, and a wired verb under a refusing root was never graded at
//! all. The probe now asks every entry for itself.
//!
//! Whether an entry has an arm is decided BEHAVIOURALLY, by running it, not by
//! searching the source, because **a capability arriving under an unlisted name
//! is invisible to a name check.** Driven in `cli_write_moves_only_what_changed.rs`.
//! **It was established in `declared_but_unwired.rs`, which retired on
//! 2026-08-21 once that file drove the same claim** -- and the citation is to
//! the file that HOLDS the reason today rather than to the one that first
//! made it, because a citation to an origin dangles the moment the origin goes
//! and a citation to a holder does not.

use std::collections::BTreeSet;
use std::process::Command;
use std::sync::OnceLock;

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
/// sees nothing here. **The invariant is that at least two survive any
/// retirement**, not that any particular file does.
///
/// **THE COUNT WAS NEVER TWO, AND NAMING TWO IS WHAT MADE THE INVARIANT LOOK
/// FRAGILE** (ic, measured 2026-08-21, after `declared_but_unwired.rs`
/// retired). Ten literal copies survive it, across seven files:
/// `dispatch_ssot.rs`, `exit_codes.rs`, `session_hook_lockout.rs`,
/// `guide.rs`, this file, `cli_write_moves_only_what_changed.rs` (x2) and
/// `write_moves_only_what_changed.rs` (x2) -- some asserting, some quoting the
/// phrase inside a stated reason.
///
/// **AND ONE OF THEM IS NOT A PEER: `render.rs` EMITS IT.** That copy is the
/// product, so it is authoritative in the only sense that matters -- change it
/// and the behaviour changes, and every copy here SHOULD then red. The
/// no-authoritative-copy rule governs the ASSERTING copies among themselves; it
/// was never a claim about the emitter, and reading it as one would argue for
/// leaving a renderer change unasserted.
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

/// The paths the table DECLARES are safe to drive, read from its own
/// `populations.probeable` block.
///
/// # Why this is read rather than derived
///
/// **The table enumerates its own populations so no consumer has to re-derive
/// one, and it says so in the block's `why`.** A traversal that decides what to
/// drive by walking rows is wrong in both directions at once -- too narrow,
/// because it misses the top-level `new_surface[]` rows, and too wide, because
/// it takes in rows dispositioned `retire` that the binary does not contain.
/// The `why` records the cost already paid for that: *104 against 112 against
/// 107, three apart with opposite signs, so no count-based sanity check
/// flinches -- which is how the same wrong population was hand-written five
/// times in one week.* **A floor assertion does not close this, because a
/// derived population is as likely to come out too BIG as too small.**
///
/// # Why `probeable` and not `shipped`
///
/// `probeable` is `shipped` minus `populations.not_probed`, and `not_probed` is
/// a machine-readable DO-NOT-DRIVE list carrying a reason per member. Two of
/// the five never return -- `daemon run` and `mcp` serve until killed, so any
/// timeout classifies a working server as a hang -- two write outside any
/// sandbox when invoked bare: `claude upgrade` installs into the operator's
/// REAL `~/.claude`, and `claude start` launches a real Claude Code session --
/// and one, `daemon start`, returns cleanly and leaves a daemon running behind
/// the probe. The last two joined the list on 2026-08-31, when the probe below
/// widened from family roots to every entry and would otherwise have reached
/// them.
///
/// **`.expect()` rather than a default, because the exclusion must fail LOUD
/// when it cannot be found.** A missing list that degrades to "exclude nothing"
/// is the silent form of driving all four.
fn probeable() -> BTreeSet<String> {
  let raw: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  raw["populations"]["probeable"]
    .as_array()
    .expect("`populations.probeable` is a list -- the exclusion cannot be skipped silently")
    .iter()
    .map(|m| {
      m.as_str()
        .expect("a probeable member is a path string")
        .to_string()
    })
    .collect()
}

/// The DO-NOT-DRIVE list, read from the table rather than restated here.
///
/// It was parsed inline in `no_do_not_drive_path_is_vouched_for_as_probeable`
/// and is now read by two callers, so it is one function. A second copy of a
/// safety list is the failure this whole file is about: the copies agree until
/// a member is added to one of them.
fn not_probed() -> BTreeSet<String> {
  let raw: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  raw["populations"]["not_probed"]
    .as_array()
    .expect(
      "`populations.not_probed` is a list -- the DO-NOT-DRIVE list cannot be skipped silently",
    )
    .iter()
    .map(|m| {
      m["path"]
        .as_str()
        .expect("a not_probed member has a path")
        .to_string()
    })
    .collect()
}

/// What the binary answers for EVERY shipped entry, driven rather than read.
///
/// **KEYED ON THE ENTRY AND NOT ON ITS FAMILY, AND THE CHANGE IS THE LIVE
/// REASON `AT-06.8` WAS RED** (ic, 2026-08-31, on cc's measurement and vc's
/// note). This function's predecessor, `unwired_families`, drove each family's
/// BARE root, and the gate below keyed its deferral on the first token of an
/// entry's path. Both directions of that were wrong at once: an unimplemented
/// verb inside a wired family had its flags graded as if wired -- `st
/// bootstrap`'s three, under a wired `st`, which is the trio that held the row
/// red -- and a wired verb inside a family whose bare root refuses had its
/// flags never graded at all -- every `claude` and `agents` leaf, under roots
/// whose `None` arm answers the unwired phrase. **The unit that is wired or
/// not is the entry, and the probe was asking about its parent.**
///
/// # This drives commands, so its population is a SAFETY question
///
/// **This harness was safe by accident of SCOPE and not by subtraction, and the
/// distinction only shows up when someone extends it** (cc, 2026-08-26). It
/// walks `table.families` and drives FAMILY NAMES bare, so it could not reach
/// `daemon` or `mcp` -- both `new_surface[]` rows rather than families -- and
/// bare `claude` only asks for a subcommand. **It never consulted `not_probed`
/// at all.** Widening the loop to leaf paths or to `new_surface[]` is a one-line
/// change, and before this gate existed it would have inherited the whole
/// hazard in silence: installing into the operator's real `~/.claude`, or
/// hanging forever on a server that never returns.
///
/// **A sweep that is safe only because its targets are unbuilt gets MORE
/// dangerous as the project succeeds**, which is the wrong direction for a
/// harness to age against a work programme whose whole content is wiring
/// families.
///
/// **THIS IS THAT WIDENING, DONE THROUGH THE DO-NOT-DRIVE LIST RATHER THAN
/// AROUND IT, AND IT FOUND TWO ROWS THE ROOT-ONLY LOOP HAD BEEN PROTECTING BY
/// NEVER REACHING THEM.** `daemon run` never returns -- it IS the daemon, in
/// the foreground -- and `daemon start` detaches one that outlives the probe.
/// Both sat in `populations.probeable` because nothing had ever driven a
/// `daemon` leaf; both are now in `populations.not_probed` with their grounds,
/// and this function refuses to drive any entry outside `probeable` rather
/// than skipping it.
///
/// # How an entry is asked, and what its answer means
///
/// - A row whose verb slot is filled by clap SUBCOMMANDS with declared
///   `values` (`claude skills <verb>`) is driven ONCE PER VALUE -- with the
///   bare form as well when the slot is optional -- and is unwired only if
///   EVERY invocation answers the refusal. The flags sit on the row, and one
///   wired verb is enough for an arm to read them.
/// - A root row with a subcommand slot and no `values` (`st`, `daemon`) is
///   driven BARE, exactly as the families loop always did: the dispatcher's
///   own `None` arm answers for it, and a sentinel in a subcommand slot is a
///   clap usage error that reaches no arm at all.
/// - Every other row is driven with [`PROBE_SENTINEL`] in each required
///   positional and after each required flag -- the `args[0]` rule the
///   `new_surface` loop already applied, now applied to every entry.
/// - A row in `populations.not_probed` is never driven and is reported as
///   NOT PROBED, in a bucket of its own: its wiredness is unknown here, so a
///   flag on it is neither gated nor deferred but NAMED on every run.
///
/// The refusal fires BEFORE any project is resolved -- driven 2026-08-31:
/// `st bootstrap` answers the marker at rc=2 in an empty directory and in a
/// freshly initialised project alike, and `config get x` the same -- so the
/// empty sandbox directory is a fair place to ask every entry the question.
/// **Every entry is asked in ONE stream, `dispatch::shipped_entries`**, the
/// chain the binary itself dispatches through, so this probe cannot cover a
/// population the gate below does not. The two loops this replaces -- one
/// over families, one over `new_surface[]`, the second of which was measured
/// unexercised -- are one loop.
/// The value handed to every required argument of a probed row.
///
/// **It has to satisfy clap and be incapable of RESOLVING**, because the rows
/// this probe drives include writers. A thread id that cannot exist does both:
/// it is a well-formed value for the argument kinds the table declares, and it
/// names no thread, no address and no subcommand, so the command it completes
/// refuses on its own terms rather than performing the act.
const PROBE_SENTINEL: &str = "ST9999";

/// The two things the probe can say about an entry, and the one it must not
/// fake: an entry it was not allowed to ask is neither wired nor unwired here.
struct Wiredness {
  /// Entry paths every probed invocation of which answered the unwired
  /// refusal.
  unwired: BTreeSet<String>,
  /// Entry paths `populations.not_probed` forbids driving. Not a verdict.
  not_probed: BTreeSet<String>,
}

/// One probe pass per test binary, shared: every arm that needs the answer
/// reads the same measurement instead of re-spawning the whole surface.
fn wiredness() -> &'static Wiredness {
  static PROBED: OnceLock<Wiredness> = OnceLock::new();
  PROBED.get_or_init(probe_every_shipped_entry)
}

fn probe_every_shipped_entry() -> Wiredness {
  let dir = tempfile::tempdir().expect("tempdir");
  let table = dispatch::table();
  let probeable = probeable();
  let never_drive = not_probed();
  let mut unwired = BTreeSet::new();
  let mut skipped = BTreeSet::new();
  for entry in dispatch::shipped_entries(&table) {
    // **THE EXCLUSION IS NAMED, NOT SKIPPED.** A row in the DO-NOT-DRIVE list
    // is recorded as unprobed so the gate can report its flags by name; a row
    // in NEITHER list is a change to this loop that nobody costed against
    // that list, and the assertion refuses it rather than letting the harness
    // quietly measure less than it claims.
    if never_drive.contains(&entry.path) {
      skipped.insert(entry.path.clone());
      continue;
    }
    assert!(
      probeable.contains(&entry.path),
      "`{}` is a shipped row that is in neither `populations.probeable` nor \
       `populations.not_probed`. This loop will not drive an unvouched-for path and will not \
       drop one silently: put it in one list or the other -- of the members `not_probed` has, \
       two never return, two write outside the sandbox into the operator's real home, and one \
       leaves a daemon running behind the probe.",
      entry.path
    );
    // **`HOME` IS SANDBOXED BECAUSE THIS PROBE RUNS THE COMMAND, AND ONE OF
    // THEM NOW WRITES** (vc, 2026-08-27, under hv's pen; found by cc driving
    // `bootstrap` rather than reading it). `current_dir` bounds where the
    // command runs; it does NOT bound `$HOME`, and `userstate::intent_dir` is
    // `home()?.join(".intent")` -- so bare `intent bootstrap` published the
    // install-root pointer into the OPERATOR'S REAL `~/.intent`, beside their
    // `agents/` and `evidence/`, as a side effect of asking whether the command
    // was wired. A probe whose question has a side effect is not a probe.
    //
    // **THE SANDBOX IS PREFERRED OVER `not_probed` DELIBERATELY.** Adding
    // `bootstrap` to the DO-NOT-DRIVE list would have been the precedent's
    // answer -- `claude upgrade` and `claude start` are there for exactly this
    // -- and it buys safety by DELETING COVERAGE, one family at a time, for a
    // work programme whose whole content is wiring families. Setting `HOME`
    // keeps every family probed and makes the write land where it can do no
    // harm. It is also not a new idiom here: `bootstrap_door.rs:41` already
    // drives this same command under a fixture `HOME`.
    //
    // **AND IT CLOSES THE CLASS RATHER THAN THE INSTANCE.** The doc on this
    // function says the harness was safe by accident of SCOPE and would get
    // more dangerous as the project succeeds. `bootstrap` is that prediction
    // arriving: the first FAMILY whose bare form is a leaf that writes.
    // `daemon` and `mcp` escaped by being `new_surface[]` rows and `claude` by
    // having subcommands -- none of that was subtraction. The next such family
    // is now safe before anyone notices it exists.
    //
    // **DRIVEN, TWO-SIDED, AGAINST A DECOY DIRECTORY** standing in for the
    // operator's home, so proving the hazard could never perform it: without the
    // `env` the probe wrote `<decoy>/.intent/home`; with it the decoy is
    // untouched and the write lands in the tempdir. The real `~/.intent/home`
    // did not move across either arm.
    //
    // **AND THE FIRST ATTEMPT TO DRIVE IT REPRODUCED NOTHING, WHICH IS THE
    // WARNING WORTH LEAVING HERE.** The binary was built into a target dir
    // OUTSIDE the repo, so `install::home()` found no `lib/templates` above the
    // exe and `publish_home` refused before writing -- correctly, and for a
    // reason having nothing to do with this fix. **An instrument placed where
    // the defect cannot occur reports absence and reads exactly like a pass.**
    // Rebuilt in-tree under `target/vc/`, it reproduced first try.
    // A verb value can be in the DO-NOT-DRIVE list in its own right, so the
    // expanded spelling is checked as well as the row. A row every one of
    // whose invocations is forbidden is unprobed, not unwired.
    let invocations: Vec<Vec<String>> = invocations_for(entry)
      .into_iter()
      .filter(|argv| !never_drive.contains(&argv.join(" ")))
      .collect();
    if invocations.is_empty() {
      skipped.insert(entry.path.clone());
      continue;
    }
    // `all` stops at the first invocation an arm answers, so a row with one
    // wired verb is settled by that verb and its later, possibly writing,
    // verbs are never reached.
    let every_answer_refused = invocations.iter().all(|argv| {
      let output = Command::new(env!("CARGO_BIN_EXE_intent"))
        .args(argv)
        .current_dir(dir.path())
        .env("HOME", dir.path())
        .output()
        .unwrap_or_else(|e| panic!("could not run `intent {}`: {e}", argv.join(" ")));
      let said = String::from_utf8_lossy(&output.stderr).into_owned()
        + &String::from_utf8_lossy(&output.stdout);
      said.contains(UNWIRED)
    });
    if every_answer_refused {
      unwired.insert(entry.path.clone());
    }
  }
  Wiredness {
    unwired,
    not_probed: skipped,
  }
}

/// The invocations that ask one row whether an arm answers it.
fn invocations_for(entry: &dispatch::Entry) -> Vec<Vec<String>> {
  // **THE PROBE IS CONSTRUCTED FROM THE ROW'S OWN DECLARED VALUES, NOT
  // DRIVEN BARE, AND THAT IS vc's `args[0]` RULE IN ITS THIRD INSTANCE.**
  // Driving bare asks a question a row with required arguments cannot
  // answer: clap refuses for the missing argument BEFORE dispatch, with a
  // message carrying no unwired marker, **so an unwired row reads as WIRED
  // and its flags then fail the gate below.** `fc` is where it was caught
  // (dc, 2026-08-29); `daemon` was the same rule with a required subcommand
  // and `at green` the same rule with an unread guard.
  //
  // **AND THE ARGUMENTS MUST NOT LET THE COMMAND SUCCEED, WHICH IS A HARDER
  // CONSTRAINT THAN SATISFYING CLAP.** Rows here WRITE -- `fc` performs a
  // fiat close and `edit` realises artefacts -- and this file already ruled
  // on that class: *a probe whose question has a side effect is not a
  // probe*, the `bootstrap` incident, where asking whether a command was
  // wired published into the operator's real `~/.intent`. So every supplied
  // value is [`PROBE_SENTINEL`], which resolves to nothing for any argument
  // kind the table declares: not a thread, not an address, not a
  // subcommand.
  //
  // **DRIVEN TWO-SIDED BEFORE LANDING, on all four probeable rows carrying
  // required arguments.** `search`, `daemon` and `edit` return the SAME
  // verdict bare and sentinel-supplied -- so the change alters no existing
  // conclusion -- while `fc` moves from `reads-as-WIRED` (rc=1, clap) to
  // `UNWIRED` (rc=2, the marker), which is the one verdict that was wrong.
  // A probe answering "wired" for every row is indistinguishable from a
  // working one until something is genuinely unwired, and `fc` is the only
  // unwired row available to prove it against.
  let base: Vec<String> = entry.path.split_whitespace().map(str::to_string).collect();
  let required_values = |argv: &mut Vec<String>| {
    for arg in entry.args.iter().filter(|a| a.kind != "subcommand") {
      // Requiredness for a POSITIONAL is the arity, not a boolean -- the table
      // spells it `1` / `1..n` for required and `0..1` / `0..n` for optional.
      // Reading a `required` field here would be reading one that does not
      // exist on `Arg`, which is the field-shaped half of the same mistake
      // this comment is about.
      if arg.arity.starts_with('1') {
        argv.push(PROBE_SENTINEL.to_string());
      }
    }
    for flag in &entry.flags {
      if flag.required {
        if let Some(spelling) = flag.spellings.first() {
          argv.push(spelling.clone());
          argv.push(PROBE_SENTINEL.to_string());
        }
      }
    }
  };
  let Some(slot) = entry.args.iter().find(|a| a.kind == "subcommand") else {
    let mut argv = base;
    required_values(&mut argv);
    return vec![argv];
  };
  if slot.values.is_empty() {
    // A root whose slot names no verbs of its own: bare, and the dispatcher's
    // `None` arm answers for it.
    return vec![base];
  }
  let mut out = Vec::new();
  if slot.arity.starts_with('0') {
    out.push(base.clone());
  }
  for verb in &slot.values {
    let mut argv = base.clone();
    argv.push(verb.clone());
    required_values(&mut argv);
    out.push(argv);
  }
  out
}

/// **THE GATE ABOVE IS ONLY AS GOOD AS THE POPULATION IT READS, SO THE
/// POPULATION IS CHECKED TOO.**
///
/// `probeable` is `shipped` minus `not_probed`, and that subtraction happens in
/// the table rather than here. If a `not_probed` member ever appeared in
/// `probeable` -- a hand-edit, a regenerator bug, a member added to one list and
/// not subtracted from the other -- the gate would wave it straight through and
/// report nothing, because from inside the gate a vouched-for path and a
/// dangerous one look identical.
///
/// **This is the arm that would fail. The gate itself cannot fail on this,
/// which is exactly why it needs a second one.**
///
/// It also pins the list non-empty. A `not_probed` that degraded to zero
/// members would make this test pass by having nothing to check -- the vacuous
/// green that reads like coverage.
#[test]
fn no_do_not_drive_path_is_vouched_for_as_probeable() {
  let not_probed = not_probed();

  assert!(
    !not_probed.is_empty(),
    "`populations.not_probed` is empty, so this check has nothing to test and the gate in \
     `wiredness` is unguarded. Five paths belong here: two that never return, two that write \
     into the operator's real home, and one that leaves a daemon running behind the probe."
  );

  let probeable = probeable();
  for path in &not_probed {
    assert!(
      !probeable.contains(path),
      "`{path}` is in BOTH `populations.not_probed` and `populations.probeable`. The gate in \
       `wiredness` reads `probeable` and would drive it. `not_probed` members either never \
       return or write outside the sandbox."
    );
  }
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
  //
  // **THE `st bootstrap` TRIO LEFT THIS LIST ON 2026-08-31 WITHOUT BEING FIXED,
  // AND THE DISTINCTION IS THE WHOLE ENTRY** (ic, on cc's measurement and vc's
  // note on `AT-06.8`). `--audit-only`, `--dry-run` and `--deliverable` are as
  // declared and as unread as they were; what changed is that the probe now
  // asks `st bootstrap` for ITSELF and it answers the unwired refusal, so the
  // three are DEFERRED -- named on every run in that bucket -- rather than
  // graded as if an arm existed to read them. They return to the gate the day
  // dc wires `st bootstrap`, and on that day each is either read or back here.
  //
  // **SURFACED BY THE SAME CHANGE, AND THEY ARE THE MIRROR OF THE TRIO.** Both
  // sit under a root whose bare form answers the unwired refusal -- `agents`
  // (issue 0175) and `llm` -- so the family-keyed probe had deferred every flag
  // beneath them and neither had ever been graded. Both are WIRED leaves with a
  // declared, helped, accepted flag that no renderer arm reads:
  // `llm_usage_rules()` takes no `ArgMatches` at all, and `"template"` is
  // spelled nowhere in the crate. **AC-06.8 instances of the plainest kind,
  // entered here with their owners rather than absorbed**: the remedy for each
  // is a SCOPE decision -- wire it (both did something in v2, which is the
  // parity hv ruled) or withdraw it from the table -- and that is not this
  // file's to make. `agents init` is dc's (ST0058); `llm usage_rules` is
  // WP-09's. The prediction written before the run named `agents init` as a
  // candidate and did not name `llm usage_rules`; the second is recorded as
  // the unpredicted half, not folded into the predicted one.
  //
  // **`agents init` --template LEFT THIS LIST ON 2026-08-31 BY BEING DECIDED,
  // WHICH IS THE ONLY WAY THIS RATCHET IS MEANT TO SHRINK.** vc ruled it
  // WITHDRAWN within the pen -- `disposition: retire` with the basis at the
  // site in `surface/dispatch-table.json` -- on the ground that withdrawing an
  // inert flag deletes a false claim rather than removing a feature, while
  // wiring it would be new behaviour and hv's. **The evidence was variation,
  // not a source read** (dc): `agents init` driven in three FRESH projects with
  // `--template elixir`, with `--template definitely-not-a-template`, and with
  // no flag at all produced byte-identical `AGENTS.md`, one md5, all rc=0 --
  // so the flag was a false claim in two directions, advertising templating
  // that does not happen AND accepting a template name that does not exist
  // where v2 refused an unknown one by name. It now refuses at clap.
  // `llm usage_rules --symlink` (WP-09) is unchanged and stands on the same
  // reasoning, awaiting the same single question: did the capability move, or
  // was it never built.
  "`llm usage_rules` --symlink (id `symlink`)",
  // **REMOVED BECAUSE THE DETECTION SAYS SO, AND THE DETECTION IS IMPRECISE
  // HERE -- recorded rather than absorbed.** `claude subagents` is NOT wired;
  // nothing reads its `-v`. It left this list because wiring `claude skills`
  // added a read of id `v`, and the scan matches an id across the whole
  // renderer rather than per entry -- the two rows are indistinguishable to
  // it. Shrinking the list is what the failure message instructs, and leaving
  // a row the scan reports as read would wedge the suite; but the honest state
  // is that ONE of these two ids is genuinely read and this list can no longer
  // say which. Routed to ic, who owns this file. (cc, 2026-08-22, with the
  // `claude skills` wiring.)
  //
  // **SURFACED BY WIDENING THIS CHECK TO `new_surface[]` ROWS (ic,
  // 2026-08-27), AND IT IS RATIFIED RATHER THAN BROKEN.** `ingest` is a
  // top-level row, so no run of this test had ever asked about its flags.
  // `--from-md` is a mode flag with ONE mode: the handler always ingests
  // markdown, because the other thing `ingest` could have meant -- rebuilding
  // the store from committed canon -- is already `sync --from-disk`. There is
  // nothing for a renderer to read, which is why the scan cannot see a read.
  // I objected to the flag when it was declared; vc ruled it KEPT because
  // withdrawing it would put the table in contradiction with ratified rows,
  // and sent the objection to AC-10.2/10.3 where its acceptance lands. The
  // full reasoning is at `render.rs`'s `ingest` arm. **Listed here so the
  // widening lands without wedging the suite, and so this state is visible in
  // the source instead of being hidden by a population that never covered it.**
  "`ingest` --from-md (id `from-md`)",
];

/// **THE CRITERION.** Every `keep` flag on a WIRED entry is read by name.
///
/// **THE PARKING BELOW HAS EXPIRED AND THE TEXT DESCRIBING IT HAD NOT** (ic,
/// 2026-08-27). It read *PARKED, LOUDLY, WITH A NAMED EXPIRY -- AND `#[ignore]`
/// RATHER THAN A RELAXED ASSERTION IS THE WHOLE POINT ... it says NOT RUN, in
/// every test run, where a reader sees it.* The `#[ignore]` is gone, the
/// baseline was taken, and this test RUNS -- so the one sentence a reader
/// arrives at was telling them the opposite of what the file does. Quoted
/// rather than deleted, because a corrected sentence reads exactly like one
/// that was never wrong. **vc's underlying ruling stands and is why the
/// expiry was honoured rather than the assertion relaxed:** relaxing a gate at
/// the moment it stops covering anything converts a refusal into a silent
/// pass.
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
///
/// **THAT PARAGRAPH IS FROM THE PARKED ERA AND STAYS AS A QUOTE** (ic,
/// 2026-08-31): the arm runs, and whether `AT-06.8` moves is decided on what
/// it PRINTS -- the violation list against `INHERITED_UNREAD`, the deferred
/// list and the unprobed list -- by the node holding the row's note, never by
/// a sentence in this file.
#[test]
fn every_declared_flag_on_a_wired_family_is_read_by_the_renderer() {
  let table = dispatch::table();
  let src = renderer_source();
  let read = ids_the_renderer_reads(&src);
  let probed = wiredness();

  let mut violations = Vec::new();
  let mut deferred = Vec::new();
  let mut unprobed = Vec::new();
  let mut checked = 0;
  let mut shielded = 0;
  let mut fragile: Vec<String> = Vec::new();

  // **ONE STREAM, AND IT IS THE PRODUCTION ONE.** This walked `table.families`
  // by hand, so every `new_surface[]` row was invisible to it -- including
  // `organize`, which declares `--apply`, `--default` and `--force`. Those
  // three flags have never been asked whether the renderer reads them.
  // `dispatch::shipped_entries` is the chain the binary itself dispatches
  // through, so using it means this check cannot again cover a population the
  // binary does not, and the `is_shipped` filter comes with it rather than
  // being spelled a second time here.
  for entry in dispatch::shipped_entries(&table) {
    // **THE DEFERRAL KEY IS THE ENTRY'S OWN PATH** (ic, 2026-08-31). It was
    // the first token of the path -- the family -- which graded `st
    // bootstrap`'s flags as wired because `st` is, and never graded a `claude`
    // leaf's because bare `claude` refuses. `wiredness` answers per entry, so
    // one lookup is the right lookup for a family root, a family verb and a
    // `new_surface[]` row alike.
    {
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
        if probed.unwired.contains(&entry.path) {
          deferred.push(line);
        } else if probed.not_probed.contains(&entry.path) {
          unprobed.push(line);
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
            violations.push(line.clone());
          }

          // **AND HOW MANY OF THESE COULD THE GATE ACTUALLY CATCH?** A
          // violation needs BOTH conjuncts, so the moment `"<id>"` appears in
          // the renderer for ANY reason, `!mentioned` is false and no removal
          // of that flag's accessor can ever red this check. `checked` counts
          // flags the gate LOOKED at, which is not the same number and reads
          // like it is.
          //
          // The discriminator is occurrences: at one, the sole mention is the
          // accessor itself and deleting it satisfies both conjuncts; above
          // one, a mention survives the deletion and the gate is blind for
          // that flag whatever anyone does to it.
          //
          // **DRIVEN BY HAND BEFORE IT WAS COUNTED** (cc, 2026-08-27, on
          // `bootstrap`): deleting `--force`'s read left this check GREEN,
          // because `organize` also reads an id spelled `force`. vc then
          // measured the population over the same stream -- 94 of 109 shipped
          // non-intrinsic flags cannot fire -- and only HALF are the shared-id
          // case the hand instance suggested; the other half are single-family
          // ids that appear as a literal for an unrelated reason.
          let mentions = src.matches(&format!("\"{id}\"")).count();
          if mentions > 1 {
            shielded += 1;
          } else if mentions == 1 {
            // **THE THIRD CATEGORY, AND IT IS NOT A FOOTNOTE ON THE OTHER TWO**
            // (vc, 2026-08-27). A flag here is gated ONLY by the coincidence
            // that nothing else in the renderer spells its id. Nothing defends
            // that. The day any unrelated string happens to equal one of these
            // ids -- a new command's verb, a field name, a status token -- the
            // flag moves from gated to shielded, the count above moves by one,
            // and NO TEST OBSERVES THE MOVE, because nothing about the surface
            // or the renderer is wrong at that moment.
            //
            // **REPORTED, NEVER GATED, AND THAT IS THE WHOLE DESIGN.** Gating
            // it would fail on a coincidence nobody caused and could not fix
            // except by renaming an unrelated string. The population is worth
            // watching precisely because it can decay while every actor
            // involved is behaving correctly.
            fragile.push(line);
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
      // **"their family" STOPPED BEING TRUE THE DAY A LEAF JOINED THIS LIST,
      // AND "family or leaf" STOPPED BEING TRUE WHEN THE KEY BECAME THE
      // ENTRY.** The sentence names the unit the probe actually asked about,
      // because this line is the one a reader carries away.
      "flag-reachability: {} flag(s) NOT GATED -- their entry answers the unwired refusal, so no arm could read them yet:",
      deferred.len()
    );
    for line in &deferred {
      println!("  {line}");
    }
  }
  // **NAMED FOR THE SAME REASON, AND KEPT APART FROM THE DEFERRED LIST.** A
  // deferred flag has a KNOWN reason nothing reads it; an unprobed flag has an
  // UNKNOWN one -- the table forbids driving its entry, so this file cannot
  // say whether an arm reads it. Folding the two together would let "not
  // gated" read as "known unwired" for a row that may be wired and inert.
  if !unprobed.is_empty() {
    println!(
      "flag-reachability: {} flag(s) NOT GATED -- their entry is in `populations.not_probed`, so this harness may not drive it and cannot say whether an arm reads them:",
      unprobed.len()
    );
    for line in &unprobed {
      println!("  {line}");
    }
  }
  // **THE COVERAGE IS STATED, BECAUSE A GREEN IS OTHERWISE READ AS THE WHOLE
  // SURFACE.** The false-negative class is named in the comment above and a
  // comment is what a reader stops at; the number is what they carry away.
  // This REPORTS and does not gate -- the conservative conjunction was a
  // deliberate choice against a real false-positive class (`st new -s` reading
  // `start`), and unpicking it is a design question rather than a patch.
  println!(
    "flag-reachability: {checked} flag(s) examined across the wired families -- of these, the gate CAN fire for {}, and CANNOT for {shielded} whose id also appears elsewhere in the renderer",
    checked - shielded
  );

  // **NAMED RATHER THAN COUNTED, for the reason the deferred list is: a bare
  // number here would be a figure nobody can act on, and this is the one
  // population whose membership changes without anyone touching it.**
  //
  // **AND THE THREE NUMBERS ADD UP, WHICH IS WORTH STATING SO NOBODY READS
  // THEM AS INDEPENDENT.** Every examined flag has 0, 1, or >1 mentions:
  // >1 is `shielded`, 1 is `fragile`, and 0 is already a `violations` member
  // (they survive only by being grandfathered in `INHERITED_UNREAD`). So
  // `CAN fire` = `fragile` + the zero-mention set, and the fragile count is a
  // SUBSET of it rather than a separate population.
  if !fragile.is_empty() {
    println!(
      "flag-reachability: {} of those gated are held ONLY by the coincidence that nothing else spells their id -- each moves to shielded, silently, the day any unrelated string in the renderer equals it:",
      fragile.len()
    );
    for line in &fragile {
      println!("  {line}");
    }
  }

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
    "these flags are in the inherited set and no longer report as declared-and-unread -- read, deferred because their own entry is unwired, or withdrawn -- good news either way, and the list must shrink to match or it decays into an excuse list nobody prunes:\n  {}",
    fixed.iter().map(|s| **s).collect::<Vec<_>>().join("\n  ")
  );
}

/// **THE PROBE'S OWN DISCRIMINATION, PINNED ON INSTANCES THE ESTATE ASSERTS
/// ELSEWHERE RATHER THAN ON ONES THIS FILE CHOSE** (cc's rule: a control drawn
/// from the instrument's own enumeration can only confirm the shapes it
/// already has). Each pin names where the estate makes the same claim, so a
/// pin that expires is a pin somebody will notice moving.
///
/// - `st bootstrap` is UNWIRED under a WIRED `st`: the `DECLARED_BUT_UNWIRED`
///   roster carries it and `AT-06.8`'s note was about exactly its three flags.
///   **This is the instance the family-keyed probe got wrong, in the direction
///   that graded inert flags as wired.**
/// - `claude skills` is WIRED under a `claude` whose bare form refuses: driven
///   2026-08-31, `intent claude skills list` answers rc=0 with the canon
///   roster in an empty directory. **This is the other direction -- flags never
///   graded because the root refused.**
/// - `claude subagents` is UNWIRED (cc, 2026-08-22: nothing reads its `-v`),
///   which is why its flags belong in DEFERRED and not in the ratchet.
/// - `daemon run` and `mcp` are NOT PROBED, from the table's own list.
///
/// **WHEN `st bootstrap` IS WIRED THIS CONTROL EXPIRES, AND THE MESSAGE SAYS
/// WHAT TO DO:** move the pin to the next unwired verb under a wired root, and
/// if the surface has none left, the direction still needs a subject -- a
/// synthetic one, per vc's ruling on borrowed instances.
#[test]
fn the_wiredness_probe_asks_about_the_entry_and_not_its_parent() {
  let probed = wiredness();
  assert!(
    probed.unwired.contains("st bootstrap"),
    "`st bootstrap` did not answer the unwired refusal. If it has been WIRED, this control has expired: pin the next unwired verb under a wired root (the `DECLARED_BUT_UNWIRED` roster lists the candidates), or synthesise one -- the direction it guards is an unwired leaf under a wired family being graded as wired.\n  unwired: {:?}",
    probed.unwired
  );
  assert!(
    !probed.unwired.contains("st"),
    "`st` answered the unwired refusal bare, so the probe cannot tell a wired root from an unwired leaf under it"
  );
  assert!(
    !probed.unwired.contains("claude skills"),
    "`claude skills` was classed unwired -- the verb-slot expansion is not driving its declared values, so every flag under a refusing root would be deferred again"
  );
  assert!(
    probed.unwired.contains("claude subagents"),
    "`claude subagents` was classed WIRED. If it has been wired, its flags move from DEFERRED to the gate and this pin moves with them; if not, the expansion is mistaking a clap usage error for an arm"
  );
  for path in ["daemon run", "mcp"] {
    assert!(
      probed.not_probed.contains(path),
      "`{path}` was not reported as NOT PROBED, so a row the table forbids driving was either driven or silently dropped"
    );
  }
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

/// **A FLAG THE PARSER REFUSES IS UNREACHABLE, AND EVERY ARM ABOVE PASSES ON
/// IT.** The criterion this file gates asks whether the renderer READS a
/// declared id. That is necessary and it is not sufficient: `--languages` is
/// read by `render::critic` on the arm's very first line, before a language is
/// required -- and `intent critic --languages` still exits 1 without printing
/// anything, because the table declares `lang` at arity `1` and clap refuses
/// the invocation before any handler runs.
///
/// So the scan says the flag is real, `--help` lists it, and the flag does not
/// work. **That is the same lie the file's opening line is about, told through
/// the parser instead of through the renderer**, and no source-scanning arm can
/// reach it -- the defect is in the grammar the table builds, not in the code
/// the scan reads. This one drives it.
///
/// # What this actually broke, measured rather than relayed
///
/// **The claim reaching this file was that the canon pre-commit hook lists a
/// project's languages with this flag, gets the refusal, and so fails OPEN
/// fleet-wide. It does not, and the correction matters more than the fix.**
/// `lib/templates/hooks/pre-commit.sh` reads the `languages` array straight out
/// of `intent/.config/config.json` with `jq` and dispatches
/// `intent critic <lang> --staged` per language -- the positional spelling,
/// which works. `--languages` appears NOWHERE in the shipped install
/// (`grep -rn -- '--languages'` over `libexec/`, positive-controlled against a
/// string known to be there). The fleet's gates were never opened by this.
///
/// The one real consumer is `bin/.devbin/lib/cmd/check`, and it fails CLOSED:
/// it captures the exit code and dies naming the cause. So the blast radius is
/// the devbin estates, loudly, and the defect that remains is the plain one --
/// **`--help` advertises a flag that does not work.** That is worth fixing on
/// its own and does not need the bigger story to justify it.
///
/// # Two rosters, and the nearer one is the wrong one
///
/// The `lang` arg's own `values` declares SEVEN, and asserting against it
/// would be the natural mistake: it is the field three lines from the fix.
/// `HEADLESS_LANGUAGES` is FIVE. **Both are right, about different questions.**
/// The arg accepts `author` and `content` because the critic takes them as a
/// clean no-op (prose critique is the `critic-prose` subagent's, not this
/// runner's); the flag's own help says *languages with a headless code
/// critic*, and that is the five. So this asserts against the roster the
/// handler prints, not the roster the table declares beside it.
///
/// # What this deliberately does NOT assert
///
/// Nothing here says what bare `intent critic` should exit. It is a clap usage
/// error at 1 today while `critic klingon` is 2, and `exit_codes.rs` records
/// that divergence as unruled rather than encoding either answer. **Relaxing
/// `lang` to an optional arity would have fixed this flag and settled that
/// ruling as a side effect**, which is why the fix is a declared
/// `required_unless` instead: it moves exactly the one invocation named here.
#[test]
fn an_early_exit_flag_answers_without_the_positional_it_sits_beside() {
  // **THE POSITIVE CONTROL ON THE EXPECTATION ITSELF.** The assertion below
  // compares the binary's output to this roster, so a roster that degraded to
  // zero members would make it pass on empty output -- the vacuous green this
  // file already refuses once, for `not_probed`.
  assert!(
    !intentsvcs::critic::HEADLESS_LANGUAGES.is_empty(),
    "the headless roster is empty, so the comparison below would be satisfied by a binary that printed nothing"
  );
  let expected: String = intentsvcs::critic::HEADLESS_LANGUAGES
    .iter()
    .map(|l| format!("{l}\n"))
    .collect();

  let dir = tempfile::tempdir().expect("tempdir");
  let bare = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["critic", "--languages"])
    .current_dir(dir.path())
    .output()
    .expect("could not run `intent critic --languages`");

  assert_eq!(
    bare.status.code(),
    Some(0),
    "`intent critic --languages` did not answer. The flag is declared, `--help` advertises it as answering-and-exiting, and `render::critic` reads it before requiring a language -- so a non-zero here is the parser refusing the invocation before the handler it would have satisfied.\nstderr: {}",
    String::from_utf8_lossy(&bare.stderr)
  );
  assert_eq!(
    String::from_utf8_lossy(&bare.stdout),
    expected,
    "`intent critic --languages` exited 0 without printing the headless roster one per line"
  );

  // **THE CONTROL, AND IT IS THE HALF THAT COULD REGRESS.** The fix makes a
  // required positional conditionally optional, so the spelling that supplies
  // it must keep working and keep answering identically -- the flag answers
  // before the language is read, so the language cannot change the answer.
  // Without this arm, dropping `lang` from the grammar entirely would satisfy
  // everything above.
  let with_positional = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["critic", "shell", "--languages"])
    .current_dir(dir.path())
    .output()
    .expect("could not run `intent critic shell --languages`");

  assert_eq!(
    with_positional.status.code(),
    Some(0),
    "`intent critic shell --languages` stopped working, so the fix broke the spelling that was already correct.\nstderr: {}",
    String::from_utf8_lossy(&with_positional.stderr)
  );
  assert_eq!(
    String::from_utf8_lossy(&with_positional.stdout),
    expected,
    "the positional spelling no longer prints the same roster as the bare one"
  );
}
