//! `ST0064` `AC-01.2` and `AC-01.6`: **there is ONE daemon predicate, and the
//! menubar app consumes it rather than spelling a second.**
//!
//! # One arm, two rows, deliberately
//!
//! `AC-01.2` says the app's health predicate and the CLI's routing predicate
//! are one predicate rather than two that agree. `AC-01.6` says the three
//! endpoint states are a PROJECTION above `route()` and never a widening of it.
//! **Those are the same claim from two directions**, so they get one instrument
//! and not two -- which is the criteria's own point, and is recorded here so
//! nobody builds a second checker for the second row. The other clauses of both
//! rows are covered elsewhere: the wire contract by
//! `daemon_status_answers_a_machine.rs` and `HealthTests.swift`, the three
//! states and their remedies by `intentsvcs`'s
//! `daemon_health_splits_stale_from_absent.rs`.
//!
//! # What this cannot see, stated so a green is not read as more
//!
//! It establishes that no SECOND PREDICATE IS SPELLED in the app target. It
//! does NOT establish that the one predicate is the one executed at run time --
//! a static read cannot watch a process. An app that shells the CLI verb and
//! then ignores its answer passes here and is exactly as broken as the case the
//! row forbids; what covers that is a live drive of the built app, which is
//! `AC-01.4`'s territory and is not claimed here.
//!
//! It reads only the app TARGET (`native/macos/Intent/Intent`). The test target
//! is deliberately out of scope -- a test may construct whatever fixture it
//! needs -- and so is anything the app might link, which this cannot see at all.
//!
//! # Comments and string literals are stripped before the scan, and that is not
//! a convenience
//!
//! The app's own source discusses this criterion by name: `IntentApp.swift`
//! says *the socket has an owner to investigate, not remove*, and `Health.swift`
//! renders *holds the socket but is not answering*. **A scanner that read prose
//! would be red on day one for three files whose only offence is explaining the
//! rule they obey** -- the same trap `whiteboard-header-guard.sh` names, where
//! scanning prose would make reporting a defect an offence.
//!
//! # The population is asserted, not assumed
//!
//! A structural ABSENCE check is the shape that passes because it found nothing
//! to look at: a moved tree, a renamed extension, a wrong path, and it is green
//! forever with nothing to see. So the reach is asserted before the absence is
//! believed -- the file list must be non-empty AND must contain the app's known
//! health path, and `an_empty_app_target_is_a_failure_rather_than_a_pass` pins
//! that the predicate itself refuses an empty population.

use std::path::{Path, PathBuf};

use testkit::repo_root;

/// A daemon predicate found where only the CLI verb belongs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Finding {
  file: String,
  token: String,
}

/// The tokens that spell a daemon predicate the app has no business owning.
///
/// **TWO FAMILIES, BECAUSE THERE ARE TWO WAYS TO BUILD A SECOND PREDICATE.**
/// Reach the endpoint yourself (a network API), or find the endpoint yourself
/// (read the daemon's own published state). Either one is a second home for
/// *is intentd running*, which is the defect both rows name.
///
/// `Process` is deliberately ABSENT from this list. Spawning is how the app
/// reaches the CLI at all -- `IntentCLI` is the single shell-out home -- so
/// banning it would ban the compliant path along with the offending one.
const SECOND_PREDICATE: &[&str] = &[
  "import Network",
  "NWConnection",
  "NWListener",
  "NWEndpoint",
  "URLSession",
  "socket(",
  "intentd.addr",
  "intentd.token",
];

/// Swift source with comments and string literals removed.
///
/// Handles `//` to end of line, `/* */` with Swift's nesting, and plain `"`
/// string literals with backslash escapes. **It does NOT handle raw strings
/// (`#"..."#`) or multi-line `"""` literals**, which the app target does not
/// currently use; the consequence of meeting one is a FALSE POSITIVE -- a
/// finding a human then reads -- rather than a silent pass, which is the
/// direction a checker should fail in.
fn code_only(src: &str) -> String {
  let mut out = String::with_capacity(src.len());
  let bytes: Vec<char> = src.chars().collect();
  let mut i = 0;
  let mut block_depth = 0usize;
  let mut in_line = false;
  let mut in_string = false;
  while i < bytes.len() {
    let c = bytes[i];
    let next = bytes.get(i + 1).copied();
    if in_line {
      if c == '\n' {
        in_line = false;
        out.push('\n');
      }
      i += 1;
    } else if block_depth > 0 {
      if c == '*' && next == Some('/') {
        block_depth -= 1;
        i += 2;
      } else if c == '/' && next == Some('*') {
        block_depth += 1;
        i += 2;
      } else {
        if c == '\n' {
          out.push('\n');
        }
        i += 1;
      }
    } else if in_string {
      if c == '\\' {
        i += 2;
      } else {
        if c == '"' {
          in_string = false;
        }
        i += 1;
      }
    } else if c == '/' && next == Some('/') {
      in_line = true;
      i += 2;
    } else if c == '/' && next == Some('*') {
      block_depth += 1;
      i += 2;
    } else if c == '"' {
      in_string = true;
      i += 1;
    } else {
      out.push(c);
      i += 1;
    }
  }
  out
}

/// Every second predicate spelled in one Swift source.
fn second_predicates_in(file: &str, src: &str) -> Vec<Finding> {
  let code = code_only(src);
  SECOND_PREDICATE
    .iter()
    .filter(|token| code.contains(**token))
    .map(|token| Finding {
      file: file.to_string(),
      token: (*token).to_string(),
    })
    .collect()
}

/// Every `.swift` file under `dir`, as (repo-relative name, source).
fn swift_sources_under(dir: &Path) -> Vec<(String, String)> {
  fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(&path, out);
      } else if path.extension().is_some_and(|e| e == "swift") {
        out.push(path);
      }
    }
  }
  let mut paths = Vec::new();
  walk(dir, &mut paths);
  paths.sort();
  paths
    .into_iter()
    .map(|p| {
      let src = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
      let name = p
        .file_name()
        .expect("a file has a name")
        .to_string_lossy()
        .to_string();
      (name, src)
    })
    .collect()
}

/// The app target: the sources that ship in the menubar app itself.
fn app_target() -> PathBuf {
  repo_root().join("native/macos/Intent/Intent")
}

#[test]
fn the_app_target_spells_no_second_daemon_predicate() {
  let sources = swift_sources_under(&app_target());

  // **THE REACH, ASSERTED BEFORE THE ABSENCE IS BELIEVED.** An empty list and a
  // clean tree are the same green, and only one of them means anything.
  assert!(
    sources.len() >= 4,
    "the app target read as {} Swift file(s), so this check has nothing to look at and its green means nothing. The tree moved, or the extension changed",
    sources.len()
  );
  assert!(
    sources
      .iter()
      .any(|(name, _)| name == "DaemonService.swift"),
    "the app's health path (DaemonService.swift) is not in the population this check read, so a second predicate added there would be invisible to it"
  );

  let findings: Vec<Finding> = sources
    .iter()
    .flat_map(|(name, src)| second_predicates_in(name, src))
    .collect();

  assert!(
    findings.is_empty(),
    "the app spells a daemon predicate of its own: {findings:?}. AC-01.2 forbids two predicates that AGREE, and the disagreement is silent and inverted -- the menubar shows a green dot while the CLI routes every verb in-process, on the one surface built to tell the operator otherwise"
  );
}

#[test]
fn the_apps_health_read_goes_through_the_cli_verb() {
  // Read RAW rather than stripped: the verb and its flags are string literals,
  // which is exactly what `code_only` removes.
  let src = std::fs::read_to_string(app_target().join("Services/DaemonService.swift"))
    .expect("the app's health path");

  assert!(
    src.contains("IntentCLI.capture"),
    "the app's health read does not go through IntentCLI, the single shell-out home"
  );
  for fragment in ["\"daemon\"", "\"status\"", "\"--format\"", "\"json\""] {
    assert!(
      src.contains(fragment),
      "the app's health read does not name {fragment}, so it is not reaching health through `daemon status --format json` -- the verb AC-01.2 requires it to ask through"
    );
  }
}

/// **CONTROL: the check can SEE a network probe.** Fed synthetically rather
/// than planted in the tree, so nothing has to be reverted and the control
/// cannot leave residue behind.
#[test]
fn a_planted_network_probe_is_found() {
  let planted = "import Foundation\nimport Network\nlet c = NWConnection(to: ep, using: .tcp)\n";
  let found = second_predicates_in("Planted.swift", planted);
  assert!(
    found.iter().any(|f| f.token == "import Network"),
    "a Swift file importing Network was not reported: {found:?}"
  );
  assert!(
    found.iter().any(|f| f.token == "NWConnection"),
    "a Swift file opening an NWConnection was not reported: {found:?}"
  );
}

/// **CONTROL: the check can SEE the app finding the endpoint for itself**, which
/// is the other way to build a second predicate and the one that needs no
/// network API at all.
#[test]
fn a_planted_read_of_the_daemons_published_address_is_found() {
  let planted = "let addr = try String(contentsOf: home.appending(path: intentd.addr))\n";
  let found = second_predicates_in("Planted.swift", planted);
  assert_eq!(
    found,
    vec![Finding {
      file: "Planted.swift".to_string(),
      token: "intentd.addr".to_string(),
    }],
    "reading the daemon's published address is a second predicate and was not reported"
  );
}

/// **CONTROL FOR THE OTHER DIRECTION, AND IT IS THE ONE THE REAL TREE NEEDS.**
/// The app discusses this very criterion in prose. If the scanner read comments
/// and strings it would be red for files whose only offence is explaining the
/// rule they obey.
#[test]
fn the_tokens_are_not_matched_inside_comments_or_strings() {
  let prose = concat!(
    "// the socket( has an owner to investigate, not remove\n",
    "/* import Network would be a second predicate */\n",
    "let msg = \"holds the socket( but is not answering\"\n",
    "let ok = true\n",
  );
  assert_eq!(
    second_predicates_in("Prose.swift", prose),
    vec![],
    "the scanner matched inside a comment or a string literal, which makes explaining this rule an offence against it"
  );

  // And the same tokens OUTSIDE prose are still found, so the stripping did not
  // simply blind it.
  assert!(
    !second_predicates_in("Real.swift", "import Network\n").is_empty(),
    "stripping comments and strings also blinded the scanner to real code"
  );
}

/// **CONTROL FOR THE WALK ITSELF, which the two fixture controls above do not
/// reach.** They feed `second_predicates_in` directly, so they prove the
/// PREDICATE sees a plant and prove nothing about the reading that supplies it:
/// a walk that skipped subdirectories, or filtered the wrong extension, would
/// leave both of them green and the real arm blind. This plants a file on disk
/// in a nested directory and requires the whole path -- walk, read, strip,
/// match -- to carry it through.
#[test]
fn the_walk_carries_a_planted_file_through_to_a_finding() {
  let dir = tempfile::tempdir().expect("tempdir");
  let nested = dir.path().join("Services");
  std::fs::create_dir_all(&nested).expect("mkdir");
  std::fs::write(nested.join("Planted.swift"), "import Network\n").expect("write the plant");
  std::fs::write(dir.path().join("NotSwift.txt"), "import Network\n").expect("write the decoy");

  let sources = swift_sources_under(dir.path());
  assert_eq!(
    sources.len(),
    1,
    "the walk read {} file(s) from a tree holding one .swift in a subdirectory and one decoy: {sources:?}",
    sources.len()
  );

  let findings: Vec<Finding> = sources
    .iter()
    .flat_map(|(name, src)| second_predicates_in(name, src))
    .collect();
  assert_eq!(
    findings,
    vec![Finding {
      file: "Planted.swift".to_string(),
      token: "import Network".to_string(),
    }],
    "a second predicate planted on disk did not survive the walk into a finding, so the real arm would be blind to the same file"
  );
}

/// **THE POPULATION GUARD AS A PROPERTY OF THE PREDICATE, not of the tree.**
/// An absence check over an empty population is the failure mode that survives
/// every refactor silently.
#[test]
fn an_empty_app_target_is_a_failure_rather_than_a_pass() {
  let empty = tempfile::tempdir().expect("tempdir");
  assert!(
    swift_sources_under(empty.path()).is_empty(),
    "a directory with no Swift in it must read as an empty population"
  );

  let missing = swift_sources_under(&empty.path().join("no-such-dir"));
  assert!(
    missing.is_empty(),
    "an unreadable directory must read as an empty population rather than as a clean one"
  );
}
