//! `AT-03.1` (ST0073) / `AC-03.1`: **every place the test tree starts a daemon
//! that SERVES hands it a lifeline, and a new place that forgets is refused.**
//!
//! # WHY A CHECK AND NOT JUST THE FIX
//!
//! The lifeline is passed at the call site. Three call sites carry it today and
//! nothing stops a fourth being added without it -- and a fourth without it
//! leaks exactly the way the first three did, silently, on a machine nobody is
//! watching. **The fix is one commit; the property has to outlive it.**
//!
//! # WHAT IT LOOKS AT, AND WHY IT IS THE SOURCE RATHER THAN A RUN
//!
//! A runtime check would have to start a daemon to learn whether that daemon
//! got a lifeline, which is a test that leaks when it fails. **The source is
//! the only surface where the absent case is observable without creating it.**
//!
//! # THE TWO WAYS A DAEMON IS STARTED, AND BOTH ARE COVERED
//!
//! `CARGO_BIN_EXE_intentd` starts one directly. `intent daemon run` EXECS the
//! same binary, so a pipe handed to the CLI is inherited by the daemon -- and a
//! spawn of `daemon run` without one is exactly as leaky. Covering only the
//! first would be a guard on the path the danger was expected to arrive by,
//! which `intentd`'s own front-door comment already records as the mistake that
//! cost this estate a real outage.
//!
//! # WHAT IS DELIBERATELY NOT A FINDING
//!
//! A spawn wrapped in `timeout` is bounded by something other than a lifeline
//! and is left alone. `arguments_do_not_start_a_daemon.rs`'s `run()` is the
//! case: it drives arguments that must NOT serve, under a five-second ceiling,
//! and requiring a pipe there would be requiring a lifeline for a process whose
//! whole point is that it exits immediately.

use std::path::{Path, PathBuf};

/// Every `tests/` directory in the workspace, found rather than listed.
///
/// **A HAND-LISTED SET OF CRATES IS THE THING THAT ROTS.** A new crate with a
/// daemon fixture would be outside a declared list and inside this one.
fn test_roots() -> Vec<PathBuf> {
  let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("crates/")
    .to_path_buf();
  let mut out = Vec::new();
  for entry in std::fs::read_dir(&crates).expect("read crates/") {
    let dir = entry.expect("entry").path().join("tests");
    if dir.is_dir() {
      out.push(dir);
    }
  }
  out.sort();
  out
}

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
  for entry in std::fs::read_dir(dir).expect("read a tests dir") {
    let p = entry.expect("entry").path();
    if p.is_dir() {
      rs_files(&p, out);
    } else if p.extension().is_some_and(|e| e == "rs") {
      out.push(p);
    }
  }
}

/// A spawn that starts a serving daemon and the window of source around it.
struct Spawn {
  file: String,
  line: usize,
  window: String,
  /// Does this file attach a lifeline in the helper its sites go through?
  runner_armed: bool,
}

/// **THE WINDOW IS THE UNIT, NOT THE LINE.** A builder chain spreads
/// `.env`, `.stdin` and `.spawn` over a dozen lines, so a line-oriented check
/// would see a bare `Command::new` and report every site.
const WINDOW: usize = 22;

/// How far ABOVE the spawn to look, which is a different distance from
/// [`WINDOW`] and for a different reason.
///
/// **AN EXEMPTION IS EXPLAINED BEFORE THE CODE IT EXEMPTS, AND AN EXPLANATION
/// WORTH READING IS SEVERAL LINES LONG.** The first version reached back four
/// and missed this estate's only legitimate exemption -- whose reason ran to
/// four lines, above a wrapper line, above the spawn.
const BACKTRACK: usize = 12;

/// Does this line run the CLI with an argv it was HANDED rather than one it
/// spells out?
///
/// `.args(argv)` and `.args(args)` qualify; `.args(["init", "R"])` and
/// `.args(&["daemon", "status"])` do not, because a literal list is a claim
/// about which verbs run and a variable is not.
/// Does this file build its argv from the SHIPPED SURFACE rather than spelling
/// verbs out?
///
/// **THIS IS THE NARROWING THAT MAKES THE ARGV RULE ENFORCEABLE, AND THE FIRST
/// VERSION HAD NO NARROWING AND WAS WRONG.** It demanded a lifeline at every
/// variable-argv call site in the workspace -- 101 of them -- on the reasoning
/// that a helper cannot know whether an argv starts a daemon. That reasoning is
/// false twice over. The helper HAS the argv and can look at it; and a pipe
/// nobody writes to BLOCKS any verb that reads stdin, so handing one to every
/// child hung the suite. **The blanket version failed in the direction that
/// stops the tests rather than the direction that leaks**, which is the worse
/// of the two.
///
/// What is left is the case that actually bit: a file that enumerates the
/// dispatch roster and drives every entry drives `daemon start` WITHOUT THOSE
/// WORDS APPEARING ANYWHERE IN IT. `remedies_are_reachable.rs` is that file and
/// it leaked exactly four daemons per run.
fn generates_argv_from_the_roster(src: &str) -> bool {
  src.contains("shipped_entries") || src.contains("dispatch::") || src.contains("SURFACE")
}

fn is_an_argv_call(l: &str) -> bool {
  let Some(rest) = l.split_once(".args(") else {
    return false;
  };
  let first = rest.1.trim_start();
  !first.starts_with('[') && !first.starts_with("&[") && !first.is_empty()
}

fn serving_spawns() -> Vec<Spawn> {
  let mut files = Vec::new();
  for root in test_roots() {
    rs_files(&root, &mut files);
  }
  // This file quotes every marker it searches for, in prose and in its own
  // plants. **A guard that scanned itself would report itself**, which is the
  // mention-versus-use defect arriving inside the instrument against it.
  let me = Path::new(file!())
    .file_name()
    .expect("this file has a name")
    .to_owned();

  let mut found = Vec::new();
  for f in files {
    if f.file_name() == Some(&me) {
      continue;
    }
    let src = std::fs::read_to_string(&f).expect("read a test source");
    let lines: Vec<&str> = src.lines().collect();
    for (i, l) in lines.iter().enumerate() {
      // **TWO WAYS A TEST CREATES A DAEMON, AND THE SECOND IS INVISIBLE TO THE
      // FIRST ONE'S SHAPE.** `daemon start` is a SHORT-LIVED `.output()` call
      // that leaves a DETACHED daemon behind -- `render.rs`'s `daemon_start`
      // does `process_group(0)` deliberately, so an operator's daemon survives
      // a `ctrl-c` in the shell that started it. A check keyed on `.spawn()`
      // cannot see that, and the first version of this file could not: it
      // reported the tree clean while `daemon_lifecycle.rs` drove that verb
      // nine times, and those nine are most of what took the machine to load
      // 500 on 2026-09-10.
      // **THE TWO WORDS MUST BE ADJACENT ARGUMENTS, NOT MERELY BOTH PRESENT.**
      // `unmigrated_surface.rs:568` is a TABLE of subcommand names --
      // `("daemon", &["restart", "run", "start", "status", "stop"])` -- and a
      // check that asked only whether both words appeared reported it as an
      // unguarded daemon start. Mention versus use, in the guard against it,
      // for the third time in one day. An argument list has them adjacent; a
      // table of names never does.
      let starts_daemon = l.contains("CARGO_BIN_EXE_intentd") || l.contains("\"daemon\", \"run\"");
      let detaches_a_daemon = l.contains("\"daemon\", \"start\"");
      // **AND THE THIRD SHAPE, WHICH IS THE ONE THAT ACTUALLY LEAKED.** A
      // helper that runs the CLI with a CALLER'S argv cannot know whether that
      // argv starts a daemon. `remedies_are_reachable.rs` sweeps the whole
      // declared surface to collect remedy lines, so it drives `daemon start`
      // without the words ever appearing in the file -- four leaked daemons per
      // run, invisible to both branches above, and no reap written in that file
      // could have caught them because nothing there knows a daemon was made.
      // The rule is therefore about the SHAPE of the call, not about the verb.
      let runs_arbitrary_argv = is_an_argv_call(l) && generates_argv_from_the_roster(&src);
      if !starts_daemon && !detaches_a_daemon && !runs_arbitrary_argv {
        continue;
      }
      let lo = i.saturating_sub(BACKTRACK);
      let hi = (i + WINDOW).min(lines.len());
      let window = lines[lo..hi].join("\n");
      // `.output()` and `timeout` are bounded by something else and never
      // outlive the test; only a `.spawn()` can be orphaned.
      if window.contains("\"timeout\"") {
        continue;
      }
      // A `.output()` of anything BUT `daemon start` is bounded by the call
      // itself and leaves nothing behind. `daemon start` is the exception,
      // and it is the whole reason that branch exists.
      if !window.contains(".spawn()") && !detaches_a_daemon && !runs_arbitrary_argv {
        continue;
      }
      found.push(Spawn {
        runner_armed: src.contains("std::io::pipe()") || src.contains("testkit::lifeline"),
        file: f
          .strip_prefix(env!("CARGO_MANIFEST_DIR"))
          .unwrap_or(&f)
          .display()
          .to_string(),
        line: i + 1,
        window,
      });
    }
  }
  found
}

/// The marker a deliberate exception must carry, at the site, with a reason.
///
/// **AN EXEMPTION IS A DELIBERATE, REVIEWABLE ACT OR IT IS A HOLE.** One arm in
/// this estate legitimately spawns a daemon with no lifeline -- the production
/// control, which exists to prove that a daemon launchd started keeps serving.
/// Forbidding that would delete the only test of the supervised path. But a
/// blanket allowance for "tests that meant it" is indistinguishable from an
/// omission, so the exception is spelled at the site and must say why: a bare
/// marker with nothing after it is refused.
/// **NO TRAILING PUNCTUATION, DELIBERATELY.** The first version demanded
/// `LIFELINE-EXEMPT:` with a colon, and a peer writing a correct, well-reasoned
/// exemption wrote `LIFELINE-EXEMPT,` -- so the guard refused a legitimate
/// exception over one character and would have taught them to delete the check
/// rather than the comma. A marker that is hard to spell is a marker people
/// route around.
const EXEMPT: &str = "LIFELINE-EXEMPT";

/// **A `daemon start` SITE IS ARMED BY THE HELPER THAT RUNS IT, NOT AT THE
/// CALL.** `machine.run(&["daemon", "start"])` carries no stdin of its own --
/// the pipe is attached once in `Machine::run` and inherited by every call --
/// so for those the question is whether the FILE arms its runner, which is
/// where the property actually lives.
fn carries_a_lifeline(s: &Spawn) -> bool {
  // `Stdio::piped()` at the site, or a pipe end handed in from a harness field
  // -- `Stdio::from(self.lifeline_read.try_clone()...)`. The FIRST version of
  // this predicate matched only the two spellings it had written itself, and
  // reported `daemon_lifecycle.rs` as naked while that file held the pipe.
  s.window.contains("testkit::lifeline")
    || s.window.contains("Stdio::piped()")
    || (s.window.contains("Stdio::from(") && s.window.contains("lifeline"))
    || s.window.contains("Stdio::from(reader)")
    || (s.runner_armed && s.window.contains(".run("))
}

/// Is this site deliberately exempt, WITH a reason?
///
/// **THE REASON IS THE REST OF THAT LINE, NOT THE REST OF THE WINDOW.** The
/// first version measured everything after the marker and accepted a bare
/// `LIFELINE-EXEMPT:` because the SPAWN ITSELF followed it and was long enough
/// -- an escape hatch that its own control caught being wide open.
fn exempt_with_a_reason(s: &Spawn) -> bool {
  s.window
    .lines()
    .filter_map(|l| l.split_once(EXEMPT))
    .any(|(_, reason)| reason.trim_start_matches([':', ',', ' ', '-']).trim().len() > 20)
}

#[test]
fn invariant_every_serving_daemon_spawn_hands_over_a_lifeline() {
  let spawns = serving_spawns();

  // **REFUSES OVER AN EMPTY POPULATION RATHER THAN REPORTING THE REASSURING
  // ZERO.** A pattern that stopped matching -- a renamed env macro, a
  // restructured builder -- produces exactly the same "no findings" as a clean
  // tree, and this check would then pass forever over nothing.
  assert!(
    spawns.len() >= 2,
    "this check found {} serving daemon spawns in the whole test tree, which is fewer than the ones known to exist. Its pattern has stopped matching its subject and it is now passing over an empty population -- the failure mode it exists to refuse in other instruments.",
    spawns.len()
  );

  let naked: Vec<&Spawn> = spawns
    .iter()
    .filter(|s| !carries_a_lifeline(s) && !exempt_with_a_reason(s))
    .collect();
  assert!(
    naked.is_empty(),
    "a daemon is started here without a lifeline, so it outlives a killed test binary:\n{}\n\nEvery serving spawn must hand the daemon a pipe on stdin -- `.stdin(Stdio::piped())` -- and hold the write end for as long as the daemon is wanted. `intentd` treats a pipe on stdin as an owner and exits when it closes, which the kernel does when the owner dies BY ANY MEANS, SIGKILL included. Without it, an interrupted `cargo test` leaves a real daemon holding a real store: 64 of them on this machine on 2026-09-10, 64.9 CPU-hours between them.",
    naked
      .iter()
      .map(|s| format!("  {}:{}", s.file, s.line))
      .collect::<Vec<_>>()
      .join("\n")
  );

  eprintln!(
    "AT-03.1 reach: {} serving daemon spawn(s) across the workspace test tree, every one carrying a lifeline. NOT covered: spawns that reach the daemon by a name this check does not know, and anything outside `crates/*/tests/`.",
    spawns.len()
  );
}

#[test]
fn invariant_the_check_catches_a_spawn_that_forgets() {
  // **THE POSITIVE CONTROL, AND WITHOUT IT THE ARM ABOVE IS DECORATION.** A
  // predicate that answered "fine" for everything would pass the corpus check
  // and report a clean tree forever. These are the two shapes the real sites
  // take, planted here rather than trusted.
  let naked = Spawn {
    runner_armed: false,
    file: "planted".into(),
    line: 1,
    window: r#"Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdout(Stdio::null())
      .spawn()"#
      .into(),
  };
  assert!(
    !carries_a_lifeline(&naked),
    "the predicate accepted a spawn with no stdin at all, so it cannot catch the defect it exists for"
  );

  let armed = Spawn {
    runner_armed: false,
    file: "planted".into(),
    line: 1,
    window: r#"Command::new(env!("CARGO_BIN_EXE_intentd"))
      .env("HOME", &home)
      .stdin(Stdio::piped())
      .spawn()"#
      .into(),
  };
  assert!(
    carries_a_lifeline(&armed),
    "the predicate rejected a correctly armed spawn, so it is a check that refuses everything -- which catches the defect and is still useless"
  );

  // **AND THE EXEMPTION ITSELF IS CONTROLLED IN BOTH DIRECTIONS**, because an
  // escape hatch that accepts anything is worse than no check: it looks like
  // rigour and permits everything that reaches for it.
  let bare = Spawn {
    runner_armed: false,
    file: "planted".into(),
    line: 1,
    window: "// LIFELINE-EXEMPT:\n  Command::new(x).spawn()".into(),
  };
  assert!(
    !exempt_with_a_reason(&bare),
    "a bare LIFELINE-EXEMPT with no reason was accepted, so the marker is a hole rather than a decision anyone has to defend"
  );

  let reasoned = Spawn {
    runner_armed: false,
    file: "planted".into(),
    line: 1,
    window: "// LIFELINE-EXEMPT: this is the production control and arming it would delete the only test of the supervised path\n  Command::new(x).spawn()".into(),
  };
  assert!(
    exempt_with_a_reason(&reasoned),
    "a properly reasoned exemption was refused, so the only legitimate exception in this estate cannot be expressed and someone will delete the check instead"
  );
}
