//! `AC-08.2`: **the CLI answers in this process unless the caller says
//! `--daemon`, and a running daemon does not change that.**
//!
//! **hv REVERSED THE DEFAULT ON 2026-08-30 AND THIS FILE IS WHAT THAT MEANS.**
//! `design.md:22` says *if the intentd socket exists and answers, the CLI MUST
//! route to it*; hv ruled the other way: *it should just use intentsvcs when
//! run locally ... intentd is for cross-project work, like looking at a ST in
//! another project on the same machine. So, whilst it is possible for the
//! intent cli to use the daemon, that shouldn't be the default.*
//!
//! **THE PROPERTY THAT BUYS IS BIGGER THAN THE LATENCY, AND THE SAME DAY PAID
//! FOR IT.** An `intentd --help` typed while diagnosing something unrelated
//! started a real daemon under the developer's `$HOME`, and for three minutes
//! every session on the machine had its store verbs refused at rc=2 by a daemon
//! nobody meant to start. **Under this ruling an accidental daemon is a process
//! nobody is talking to.** That is the arm this file leads with.
//!
//! **THE FILE THIS REPLACED WAS CALLED `daemon_fallback.rs` AND ITS SUBJECT NO
//! LONGER EXISTS.** Under the old default the interesting question was what a
//! verb did when the daemon could not serve it -- fall through, or refuse. With
//! routing opt-in there is no fallthrough to have an opinion about: the default
//! is local, and `--daemon` is a request that either succeeds or is refused.
//! **A file whose name describes a retired concept is a reader's wrong turn**,
//! so it was renamed rather than edited in place.
//!
//! **EVERY PROPERTY THE OLD FILE ASSERTED STILL HOLDS SOMEWHERE HERE**, which
//! is the rule about not deleting the coverage that disagrees with your own
//! change. The `sync` carve-out and its no-daemon control are kept verbatim in
//! substance; the fallthrough arms became the default-is-local arms; and the
//! routed-verb-meets-a-daemon-that-cannot-answer arm became the `--daemon`
//! refusal, which is the same observation with the trigger moved.
//!
//! **THE FIXTURE IS A LISTENER THAT ANSWERS THE PROBE, NOT AN `intentd`.** The
//! CLI's behaviour here is a property of *something is answering*, and the
//! shipped recogniser and reply are what it answers with, so this is a faithful
//! stand-in rather than a convenient one. It also keeps the test inside one
//! crate: cargo builds a package's own binaries for its tests and not another's.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use intentsvcs::daemon;

/// A short, unique directory: a unix socket address is a fixed-size field, so
/// `$TMPDIR` on macOS leaves too little room for the daemon's own suffix.
///
/// **THE `intent-fixture-` PREFIX IS SO A SWEEP CANNOT MISS A FAMILY** (vc,
/// 2026-08-30): a census keyed on one crate's naming convention counts a subset
/// while reading like a total.
fn short_dir(tag: &str) -> PathBuf {
  static NEXT: AtomicU32 = AtomicU32::new(0);
  let dir = PathBuf::from("/tmp").join(format!(
    "intent-fixture-{tag}-{}-{}",
    std::process::id(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  ));
  std::fs::create_dir_all(&dir).expect("create an isolated directory");
  dir
}

/// A listener that answers the liveness probe, stopped when dropped.
///
/// **IT ANSWERS WITH THE SHIPPED RECOGNISER AND THE SHIPPED REPLY.** A fixture
/// that wrote its own bytes would be testing that the CLI accepts whatever this
/// file happens to send, which is a different claim from the one being made.
struct AnsweringDaemon {
  home: PathBuf,
  stop: Arc<AtomicBool>,
}

impl AnsweringDaemon {
  fn start() -> AnsweringDaemon {
    let home = short_dir("optin-home");
    let socket = intentsvcs::userstate::daemon_socket_under(&home);
    std::fs::create_dir_all(socket.parent().expect("a parent")).expect("state dir");
    let listener = UnixListener::bind(&socket).expect("bind the fixture listener");
    listener
      .set_nonblocking(true)
      .expect("so the accept loop can notice the stop flag");

    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    std::thread::spawn(move || {
      while !flag.load(Ordering::Relaxed) {
        match listener.accept() {
          Ok((stream, _)) => {
            let _ = stream.set_nonblocking(false);
            let mut reader = BufReader::new(&stream);
            let mut line = Vec::new();
            if reader.read_until(b'\n', &mut line).is_ok() && daemon::is_probe_frame(&line) {
              let mut out = &stream;
              let _ = out.write_all(daemon::PROBE_REPLY);
              let _ = out.flush();
            }
          }
          Err(_) => std::thread::sleep(std::time::Duration::from_millis(10)),
        }
      }
    });

    let answering = AnsweringDaemon { home, stop };
    answering.wait_until_seen();
    answering
  }

  /// **THE FIXTURE IS CONFIRMED LIVE BY THE SHIPPED PREDICATE BEFORE ANY CLAIM
  /// RESTS ON IT, AND UNDER THIS RULING THAT MATTERS MORE THAN IT USED TO.**
  /// Every arm below now expects a *normal* result while a daemon is up, so a
  /// fixture that failed to answer would make all of them pass while never
  /// having exercised the daemon-present path at all -- the whole file green
  /// and measuring nothing. **The good fixture and the absent one must be shown
  /// to differ**, which is what this and `with_no_daemon_the_answers_are_the_same`
  /// do between them.
  fn wait_until_seen(&self) {
    for _ in 0..500 {
      let candidates = daemon::candidates_under(&self.home).expect("readable");
      if matches!(daemon::route(&candidates), daemon::Route::Daemon(_)) {
        return;
      }
      std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!(
      "the fixture listener never answered the shipped probe, so nothing below would be testing the daemon-present path"
    );
  }

  fn home(&self) -> &Path {
    &self.home
  }
}

impl Drop for AnsweringDaemon {
  fn drop(&mut self) {
    self.stop.store(true, Ordering::Relaxed);
    let _ = std::fs::remove_dir_all(&self.home);
  }
}

/// The title minted into every fixture project.
///
/// **THE DISCRIMINATOR FOR *IT REALLY ANSWERED FROM THE LOCAL STORE*.** An
/// empty listing exits 0 too, so rc alone cannot separate a real in-process
/// answer from a verb that printed nothing -- and two empty projects have
/// identical listings, which is the vacuous-control shape this estate has
/// already been bitten by once.
const MINTED: &str = "a thread only the local store knows about";

/// An Intent project at a fresh short path, built by the shipped initialiser,
/// carrying one thread.
fn project() -> PathBuf {
  let root = short_dir("optin-proj");
  intentsvcs::init::init(&root, "OptIn", "test", env!("CARGO_PKG_VERSION"))
    .expect("the shipped initialiser creates a project");

  let opened = intentsvcs::project::Project::open(&root).expect("the project just created");
  let ctx = intentsvcs::facade::FacadeContext {
    principal: "test".to_string(),
    project_id: opened.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  let mut facade = intentsvcs::facade::Facade::open(opened, ctx).expect("open the new project");
  let id = facade.st_new(MINTED).expect("mint one thread");
  // **STARTED, BECAUSE A FRESH THREAD IS NOT WHAT `st list` SHOWS BY DEFAULT.**
  // The first build of this fixture minted a thread and asserted it appeared;
  // `st list` filters to WIP, so the listing was empty and the arm reported the
  // in-process answer as not having reached the store. **The fixture, not the
  // subject** -- and it is worth the two lines because the alternative was
  // per-verb argv knowledge in a loop that is supposed to be derived.
  facade.st_triage(&id).expect("triage the fixture thread");
  facade
    .st_start(&id)
    .expect("start it, so a default listing names it");
  root
}

/// Run the shipped `intent` binary in a project, with a chosen `HOME`.
fn run(home: &Path, root: &Path, argv: &[&str]) -> Output {
  Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", home)
    .output()
    .expect("the intent binary runs")
}

fn text(out: &Output) -> String {
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// The declared set, asserted non-empty so nothing below can go vacuous.
fn servable() -> Vec<&'static str> {
  let paths = intent_cli::render::daemon_servable_paths();
  assert!(
    !paths.is_empty(),
    "this build declares no daemon-servable verb, so every arm keyed on the declaration passes for free"
  );
  paths
}

#[test]
fn a_verb_a_daemon_could_serve_still_runs_in_this_process_by_default() {
  // **THE ARM hv's REVERSAL IS ABOUT, AND IT DRIVES THE EXACT PATH THAT USED TO
  // ROUTE.** `st list` is declared servable, so under the old default this
  // invocation went over the wire the moment anything answered the probe -- and
  // against a probe-only fixture that is rc=2. **Exit 0 here is not a weak
  // observation: it is only reachable if the daemon was ignored**, because this
  // fixture cannot answer a thread listing at all.
  let daemon = AnsweringDaemon::start();
  let root = project();
  let servable = servable();

  let mut wrong = Vec::new();
  for path in &servable {
    let argv: Vec<&str> = path.split(' ').collect();
    let out = run(daemon.home(), &root, &argv);
    let seen = text(&out);
    if out.status.code() != Some(0) {
      wrong.push(format!(
        "`intent {path}` exited {:?} while a daemon was answering. Routing is OPT-IN: a daemon \
         being up must not change what a local command does. {seen}",
        out.status.code()
      ));
      continue;
    }
    if !seen.contains(MINTED) {
      wrong.push(format!(
        "`intent {path}` exited 0 while a daemon was answering and did not name the thread this \
         project actually holds, so it did not answer from the local store: {seen}"
      ));
    }
  }

  assert!(
    wrong.is_empty(),
    "{} of {} servable verb(s) did not run in-process by default:\n  {}",
    wrong.len(),
    servable.len(),
    wrong.join("\n  ")
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn every_servable_path_asked_with_daemon_actually_leaves_this_process() {
  // **THE GUARD AGAINST A PATH DECLARED SERVABLE WHOSE ARM NEVER CALLS
  // `served()`, AND IT IS COMPLETE OVER THE DECLARED SET.** Such a path runs
  // in-process, prints a perfectly good answer and exits 0 -- indistinguishable
  // from working, and invisible to every other test in this crate. Against a
  // fixture that answers only the probe, an invocation that genuinely left the
  // process CANNOT succeed, so rc=2 is the witness that it left.
  //
  // **THIS IS THE HALF `ROUTED` USED TO GET FOR FREE AND NO LONGER DOES.** While
  // routing was the default, a declared-but-unwired path showed up as a zero
  // delta on the daemon's dispatch counter in the conformance harness. With the
  // default local, that harness never asks the daemon anything, so the
  // declaration needs its own witness -- this one.
  let daemon = AnsweringDaemon::start();
  let root = project();
  let servable = servable();

  let mut wrong = Vec::new();
  for path in &servable {
    let mut argv = vec!["--daemon"];
    argv.extend(path.split(' '));
    let out = run(daemon.home(), &root, &argv);
    let seen = text(&out);
    match out.status.code() {
      Some(2) => {}
      Some(0) => wrong.push(format!(
        "`intent --daemon {path}` SUCCEEDED against a fixture that can answer only the liveness \
         probe. It cannot have gone over the wire, so this path is declared servable and its \
         renderer arm never calls `served()`: {seen}"
      )),
      other => wrong.push(format!(
        "`intent --daemon {path}` exited {other:?}. A daemon that answered the probe and then \
         could not answer the request is exit 2 -- the build cannot answer -- rather than a \
         verdict about the operator's project: {seen}"
      )),
    }
    if !seen.contains("remedy") {
      wrong.push(format!(
        "`intent --daemon {path}` refused without a remedy: {seen}"
      ));
    }
  }

  assert!(
    wrong.is_empty(),
    "{} problem(s) across {} servable verb(s):\n  {}",
    wrong.len(),
    servable.len(),
    wrong.join("\n  ")
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn asking_for_a_daemon_that_is_not_there_refuses_rather_than_answering_locally() {
  // **THE ONE PLACE THE EARLIER RULING INVERTS, AND THE REASON IS THE CALLER.**
  // While routing was the default, falling through to in-process was right: the
  // caller had not asked for the daemon, so a local answer answered their actual
  // question. **Now they have asked**, and a silent local answer would be a
  // different question answered with the same exit code -- the accepted-and-
  // ignored flag, which is worse than a refused one because nothing disagrees.
  let home = short_dir("optin-nodaemon-home");
  let root = project();

  for path in &servable() {
    let mut argv = vec!["--daemon"];
    argv.extend(path.split(' '));
    let out = run(&home, &root, &argv);
    let seen = text(&out);

    assert_eq!(
      out.status.code(),
      Some(2),
      "`intent --daemon {path}` with no daemon running must refuse, not quietly answer here: {seen}"
    );
    // **THE REMEDY MUST NAME BOTH WAYS OUT.** Telling the operator only to start
    // a daemon is a remedy for the wrong problem when what they wanted was the
    // answer, and the local spelling is one word shorter than the one they typed.
    assert!(
      seen.contains("intent daemon start"),
      "the refusal must say how to get a daemon: {seen}"
    );
    assert!(
      seen.contains(&format!("intent {path}")),
      "the refusal must say how to get the answer WITHOUT one, which is the thing they were \
       actually after: {seen}"
    );
  }

  let _ = std::fs::remove_dir_all(&home);
  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_daemon_flag_is_refused_on_a_verb_no_daemon_can_answer() {
  // **THE FLAG IS `global`, SO IT PARSES ON EVERY COMMAND ON THE SURFACE WHILE
  // A HANDFUL HAVE ANYWHERE TO SEND IT.** Without a refusal, `intent --daemon
  // doctor` runs locally, prints a normal answer and exits 0 -- having silently
  // done the opposite of what was asked, with the exit code agreeing. **A flag
  // accepted and ignored is worse than one refused.**
  //
  // **THE UNSERVABLE VERB IS FOUND, NOT CHOSEN.** Hard-coding one would leave
  // this arm asserting nothing the day that verb becomes servable, and it would
  // still pass, because a servable verb also refuses here (for the other
  // reason) whenever no daemon is running.
  let daemon = AnsweringDaemon::start();
  let root = project();
  let servable = servable();

  // **THE CANDIDATE IS FILTERED BEHAVIOURALLY, BECAUSE PARSING FAILS BEFORE THE
  // GUARD RUNS AND LOOKS EXACTLY LIKE THE GUARD NOT WORKING.** The first build
  // of this arm took the first shipped path that was not servable and got `st`
  // -- a family root, which clap refuses at rc=1 for wanting a subcommand,
  // before `run` and therefore before `--daemon` is ever considered. **A filter
  // that admits a candidate the subject cannot reach reports a false red**, and
  // the message it printed accused the guard of being ignored.
  let table = intent_cli::dispatch::table();
  let candidates: Vec<String> = intent_cli::dispatch::shipped_entries(&table)
    .iter()
    .map(|e| e.path.clone())
    .filter(|p| !servable.contains(&p.as_str()) && !p.starts_with("daemon"))
    .collect();
  let unservable = candidates
    .iter()
    .find(|p| {
      let argv: Vec<&str> = p.split(' ').collect();
      // Parses and runs with no arguments: rc 0 (worked) or 2 (a refusal from
      // the renderer). rc 1 is clap declining to parse, which never reaches the
      // guard under test.
      matches!(
        run(daemon.home(), &root, &argv).status.code(),
        Some(0) | Some(2)
      )
    })
    .unwrap_or_else(|| {
      panic!(
        "none of the {} unservable shipped path(s) parses with no arguments, so this arm cannot reach the guard it is about",
        candidates.len()
      )
    })
    .clone();

  let mut argv = vec!["--daemon"];
  argv.extend(unservable.split(' '));
  let out = run(daemon.home(), &root, &argv);
  let seen = text(&out);

  assert_eq!(
    out.status.code(),
    Some(2),
    "`intent --daemon {unservable}` ran anyway. The flag was accepted and ignored, which the exit \
     code cannot distinguish from it having worked: {seen}"
  );
  // **THE REFUSAL NAMES THE SET, DERIVED FROM THE DECLARATION.** A refusal that
  // says only *no* leaves the operator to guess what `--daemon` is for at the
  // moment they have just been told they cannot use it.
  for path in &servable {
    assert!(
      seen.contains(path),
      "the refusal must name `{path}` as something a daemon CAN answer: {seen}"
    );
  }

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_sync_family_still_refuses_because_its_prohibition_is_literally_true() {
  // **THE ONE PLACE A DAEMON'S PRESENCE STILL CHANGES WHAT A LOCAL COMMAND
  // DOES, AND IT IS KEPT DELIBERATELY RATHER THAN OVERLOOKED.** hv's reversal is
  // about ROUTING; this is mutual exclusion, which is a different question.
  // `design.md:22`'s parenthetical -- *never two sync engines live at once* --
  // is literally true of this family: a daemon watching a project (`AC-08.5`)
  // while `intent sync` runs in it really would watch and ingest twice.
  //
  // **AND OPT-IN ROUTING MAKES THAT MORE IMPORTANT, NOT LESS.** Under the old
  // default, `sync` refusing was one refusal among many on a daemon machine.
  // Now it is the only one, so it is also the only thing standing between a
  // watching daemon and a second engine. **The predicate is wider than the
  // hazard** -- it fires on any answering daemon, not on one watching THIS
  // project -- and narrowing it is a ruling for vc rather than an inversion,
  // because refusing preserves and running does not.
  let daemon = AnsweringDaemon::start();
  let root = project();

  let refused = run(daemon.home(), &root, &["sync", "--to-disk"]);
  let seen = text(&refused);
  assert_eq!(
    refused.status.code(),
    Some(2),
    "sync ran alongside a daemon: {seen}"
  );
  assert!(
    seen.contains("sync") && seen.contains("ingest"),
    "the refusal must name WHY these two are different from every other verb, or it reads as the blanket refusal it replaced: {seen}"
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn with_no_daemon_the_answers_are_the_same() {
  // **THE CONTROL THAT ATTRIBUTES EVERY RESULT ABOVE TO THE DAEMON, AND IT IS
  // THE HALF THAT MAKES THE FIXTURE FALSIFIABLE.** Without it, `st list`
  // succeeding and `sync` refusing could be facts about those two verbs rather
  // than about a daemon -- and the second is especially convincing, because a
  // `sync` refusing for its own unrelated reason looks exactly like the
  // carve-out working.
  let home = short_dir("optin-nodaemon-control");
  let root = project();

  for path in &servable() {
    let argv: Vec<&str> = path.split(' ').collect();
    let listed = run(&home, &root, &argv);
    assert_eq!(
      listed.status.code(),
      Some(0),
      "`intent {path}` with no daemon: {}",
      text(&listed)
    );
    assert!(
      text(&listed).contains(MINTED),
      "`intent {path}` with no daemon did not name the project's own thread"
    );
  }

  let synced = run(&home, &root, &["sync", "--to-disk"]);
  assert_eq!(
    synced.status.code(),
    Some(0),
    "sync REFUSED with no daemon running, so the refusal above was not about the daemon at all: {}",
    text(&synced)
  );

  let _ = std::fs::remove_dir_all(&home);
  let _ = std::fs::remove_dir_all(&root);
}
