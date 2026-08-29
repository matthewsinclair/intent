//! AT-08.2 / AC-08.2: two independent routes to one implementation, driven
//! over the whole shipped surface and required to agree.
//!
//! **THIS IS THE FIRST THING WP-08 BUILDS, AND THE ORDER IS THE POINT.**
//! `intentsvcs` owns the surface; the CLI and the daemon are both CLIENTS of
//! it (design.md D06, D32). The risk that ruling exists to prevent is a daemon
//! that grows its OWN implementation, and the moment to catch that is before
//! there is a daemon -- not after, when the divergence has a caller. So this
//! file lands while `intentd` is still 83 lines of placeholder, and every
//! later WP-08 criterion lands underneath it.
//!
//! **THE PATHS ARE A LIST, AND THAT IS THE WHOLE MECHANISM.** Adding the
//! daemon is adding a [`Route`] variant; the body below then drives it without
//! being edited, and a daemon answering differently from the library fails
//! here. A test that hard-coded two routes would have to be REMEMBERED at the
//! moment the third arrived, which is exactly when nobody is looking.
//!
//! **WHAT IT COMPARES TODAY, STATED SO NOBODY READS MORE INTO A GREEN.** The
//! two live routes are the real `intent` binary in its own process, and
//! `spine::parse` + `render::run` called in this one. They are genuinely
//! different -- separate compilation unit, separate process, separate
//! environment and working directory, and only the binary goes through
//! `main`'s failure-to-exit-code mapping. What they are NOT is a client and a
//! server, so this file does not yet prove anything about a socket. It proves
//! that the library and the shipped binary answer identically, which is the
//! invariant the daemon will be held to when it exists.
//!
//! **TEMP FIXTURES, NEVER THE LIVE ROOT, AND THAT IS NOT TIDINESS.** Opening a
//! store runs the migration ladder, so a test that points at the real project
//! MUTATES SHARED DURABLE STATE. Sixteen files in `intentsvcs` do exactly that
//! and one of them migrated this machine's store to a schema no committed
//! binary could read, taking the delivered CLI down for everyone whose build
//! predated it (dc's finding, 2026-08-29). A harness driving the ENTIRE
//! shipped surface is the worst possible candidate to repeat that, so every
//! route gets its own freshly built project and touches nothing else.
//!
//! **EACH ROUTE GETS ITS OWN FIXTURE, WHICH IS LOAD-BEARING RATHER THAN
//! CAUTIOUS.** Most of these verbs mutate. Run `st new` down route A and then
//! down route B against ONE directory and route B is answering a different
//! question -- it meets a project that already has the thread. The routes
//! would then disagree for a reason that has nothing to do with conformance,
//! and the failure would read as a real one.
//!
//! **THE FILE LIVES HERE AND `AT-08.2` SAYS `intentd/tests/`.** That is a
//! deliberate, declared deviation, not drift: `env!("CARGO_BIN_EXE_intent")`
//! is only defined for tests in the crate that DECLARES that binary, and
//! locating a sibling binary by walking the target directory is the fragile
//! spelling this estate avoids everywhere else. Both of today's routes are
//! CLI-side, so this is where they can both be reached. The row is canon and
//! re-pointing it is vc's pen; raised with vc rather than silently left
//! mismatched.

use std::path::Path;
use std::process::Command;

use intent_cli::{dispatch, render, spine};

/// One way of reaching the implementation.
///
/// **A DAEMON VARIANT GOES HERE AND NOWHERE ELSE.** The list is walked; the
/// comparison is written once. That is what makes this a test D32 cannot be
/// reversed underneath rather than a pair of assertions someone has to
/// remember to extend.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Route {
  /// The shipped `intent` binary, in its own process.
  Binary,
  /// `spine::parse` + `render::run`, in this process.
  InProcess,
}

impl Route {
  const ALL: &'static [Route] = &[Route::Binary, Route::InProcess];

  fn name(self) -> &'static str {
    match self {
      Route::Binary => "binary",
      Route::InProcess => "in-process",
    }
  }
}

/// What a route answered: the exit code, and the message it put on stderr.
///
/// **THE CODE IS ALWAYS COMPARABLE AND THE MESSAGE IS NOT**, so they are
/// separate fields rather than one blob. A clap parse failure writes its own
/// usage text straight to the process's stderr, which the in-process route
/// cannot capture without reopening file descriptors -- so for those rows the
/// message is empty HERE and non-empty from the binary, and comparing them
/// would fail for a reason that is about capture rather than about conformance.
/// The split is counted and reported below instead of being papered over.
#[derive(Debug)]
struct Answer {
  code: i32,
  message: String,
}

/// A project with the minimum a v3 verb needs to resolve its root.
///
/// Deliberately the same shape `cli_end_to_end.rs` uses -- one fixture idiom
/// in this crate, not two.
fn project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"DualPath\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
  )
  .expect("write config");
  dir
}

fn via_binary(root: &Path, argv: &[String]) -> Answer {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  Answer {
    code: out.status.code().unwrap_or(-1),
    message: String::from_utf8_lossy(&out.stderr).trim_end().to_string(),
  }
}

/// The same call `main.rs` makes, with its exit-code mapping reproduced.
///
/// **THE MAPPING IS COPIED FROM `main.rs` ON PURPOSE AND IT IS THE ONE THING
/// HERE THAT COULD GO STALE.** If `main` grows a case this does not have, the
/// two routes diverge and this file reports it as a conformance failure --
/// which is the correct outcome, because a binary whose exit codes are not a
/// function of the library IS the defect this file exists to find.
fn via_library(root: &Path, argv: &[String]) -> Answer {
  std::env::set_current_dir(root).expect("cd into the fixture");
  let mut full = vec!["intent".to_string()];
  full.extend(argv.iter().cloned());
  match spine::parse(full) {
    Err(code) => Answer {
      code: code as i32,
      message: String::new(),
    },
    Ok(matches) => match render::run(&matches) {
      Ok(()) => Answer {
        code: spine::EXIT_OK as i32,
        message: String::new(),
      },
      Err(failure) => Answer {
        code: failure.code() as i32,
        message: failure.message().unwrap_or_default().trim_end().to_string(),
      },
    },
  }
}

/// Replace this route's own fixture path with a stable token.
///
/// **THE HARNESS FOUND THIS ON ITS FIRST RUN, AGAINST ITSELF.** 119 of the 120
/// rows agreed and `init` did not -- because it names the offending path in its
/// refusal, and the two routes are deliberately looking at two different
/// directories. The messages were identical in every byte that carries meaning
/// and differed only in the tempdir the harness had just minted.
///
/// **THE FIX IS NORMALISATION, NOT EXCLUSION.** Dropping `init` from the
/// population would have turned a self-inflicted mismatch into a permanent
/// hole in the corpus, and a corpus that cannot exhibit a defect passes for
/// free. Sharing one fixture between the routes was the other tempting repair
/// and it is worse: it would silently reintroduce the ordering dependence that
/// `drive` exists to prevent.
///
/// Both spellings are replaced because macOS hands out `/var/...` and reports
/// `/private/var/...` for the same directory, so a single substitution would
/// miss whichever form the verb happened to print.
fn without_fixture_path(message: String, root: &Path) -> String {
  let mut out = message;
  let canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
  for form in [canonical.as_path(), root] {
    if let Some(text) = form.to_str() {
      out = out.replace(text, "<PROJECT>");
    }
  }
  out
}

fn drive(route: Route, argv: &[String]) -> Answer {
  let fixture = project();
  let root = fixture.path();
  let answer = match route {
    Route::Binary => via_binary(root, argv),
    Route::InProcess => via_library(root, argv),
  };
  // The fixture is dropped after this, having answered, so nothing outlives
  // the row it was built for -- which is why the path is scrubbed HERE, while
  // the directory that produced it is still known.
  Answer {
    code: answer.code,
    message: without_fixture_path(answer.message, root),
  }
}

/// **ONE TEST FUNCTION, DELIBERATELY.**
///
/// The in-process route sets the process working directory, which is global
/// state. Cargo runs the tests within one binary on several threads, so a
/// second `#[test]` in this file would race this one for the cwd and both
/// would report nonsense -- intermittently, which is the worst kind. One test
/// means the sequencing is structural rather than a convention someone has to
/// know. Separate test BINARIES run in separate processes and are unaffected.
#[test]
fn invariant_every_shipped_verb_answers_identically_down_every_route() {
  let table = dispatch::table();

  // **DEDUPED, AND KEYED ON THE ROW RATHER THAN THE PATH.** `organize` is two
  // rows and sits in `populations.shipped` AND `populations.retired`; driving
  // a path lets whichever row is reached first decide what was tested. The
  // duplicate is a live defect in the register (`declared` double-counts it,
  // so the conservation sum balances through two cancelling errors) and is
  // being closed separately -- this harness must not inherit it, and must not
  // paper over it either, so the count below is reported.
  let mut argvs: Vec<Vec<String>> = Vec::new();
  let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
  // `shipped_entries` is the ONE traversal -- families plus `new_surface`,
  // filtered by `is_shipped`. Rolling a second one here would be a divergent
  // copy of the definition of "shipped" in the file whose whole subject is
  // that there is one description of the surface.
  for entry in dispatch::shipped_entries(&table) {
    if seen.insert(entry.path.clone()) {
      argvs.push(entry.path.split(' ').map(str::to_string).collect());
    }
  }
  assert!(
    !argvs.is_empty(),
    "the shipped population is empty, so this harness proved nothing -- \
     the table is the corpus and an empty corpus passes for free"
  );

  let mut compared_on_code_only = 0usize;
  let mut compared_on_code_and_message = 0usize;
  let mut divergences: Vec<String> = Vec::new();

  for argv in &argvs {
    let answers: Vec<(Route, Answer)> = Route::ALL
      .iter()
      .map(|route| (*route, drive(*route, argv)))
      .collect();

    let (first_route, first) = &answers[0];
    for (route, answer) in &answers[1..] {
      if answer.code != first.code {
        divergences.push(format!(
          "`intent {}`: {} exited {}, {} exited {}",
          argv.join(" "),
          first_route.name(),
          first.code,
          route.name(),
          answer.code
        ));
        continue;
      }
      // Compare the text only where every route actually produced some. See
      // `Answer`: a clap parse failure writes straight to the process stderr,
      // which the in-process route never sees, and an empty-vs-present
      // comparison there would be measuring capture rather than conformance.
      if answers.iter().all(|(_, a)| !a.message.is_empty()) {
        if answer.message != first.message {
          divergences.push(format!(
            "`intent {}`: {} said {:?}, {} said {:?}",
            argv.join(" "),
            first_route.name(),
            first.message,
            route.name(),
            answer.message
          ));
        }
      }
    }

    if answers.iter().all(|(_, a)| !a.message.is_empty()) {
      compared_on_code_and_message += 1;
    } else {
      compared_on_code_only += 1;
    }
  }

  // **THE SPLIT IS ASSERTED, NOT JUST PRINTED.** A green whose message
  // comparison silently covered zero rows would be a vacuity wearing a pass,
  // and the cheapest way to notice is to require the weaker bucket not to be
  // everything. Predict the split, never the total.
  assert!(
    compared_on_code_and_message > 0,
    "every one of the {} shipped row(s) was compared on its exit code alone, \
     so the message half of this harness proved nothing at all",
    argvs.len()
  );

  assert!(
    divergences.is_empty(),
    "{} of {} shipped row(s) answered differently down different routes, \
     which is D32 being reversed:\n  {}",
    divergences.len(),
    argvs.len(),
    divergences.join("\n  ")
  );

  eprintln!(
    "dual-path conformance: {} shipped row(s) x {} route(s) agreed \
     ({} on exit code and message, {} on exit code alone)",
    argvs.len(),
    Route::ALL.len(),
    compared_on_code_and_message,
    compared_on_code_only
  );
}
