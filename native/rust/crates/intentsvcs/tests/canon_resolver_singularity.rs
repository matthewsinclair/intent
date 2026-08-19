//! AT-01.6 / AC-01.6: the relocation is effected AT THE RESOLVER, and no text
//! sweep decides what moves.
//!
//! **The test is: point the one resolver at a different directory and require
//! every canon read and write to follow, with no other edit.** A site that
//! spells the canon path itself does not follow, and lands somewhere else.
//!
//! # Why a non-default `intent_dir` is the whole mechanism
//!
//! With the default `intent/`, a hand-spelled `intent/.canon/st/ST0001.json`
//! and a resolved one are the same bytes, so **every independent spelling
//! passes**. Configure the intent directory to anything else and they diverge
//! immediately: the resolver answers under the configured name, the literal
//! answers under `intent/`, and the artefact is simply not where the tool looks.
//! `intent_dir` is an operator-configurable field that already ships, so this
//! is a supported configuration rather than a synthetic one.
//!
//! # The red-first arm is the historical bug, and it was free
//!
//! `export.rs:386` spelled `format!("st/{}/thread.json", ...)` independently
//! until WP-01 folded it into `canon_thread_rel`. Its own neighbouring comment
//! records the issue arm having ALREADY shipped that defect once —
//! `issues/46.json` written where every reader opened `issues/0046.json`,
//! "two ends had to agree by convention and did not". **Re-introduce any second
//! independent spelling and these tests fail**, which is the property the
//! criterion asks for.
//!
//! # Two regression cases, each refuting a sweep from the opposite direction
//!
//! **(a) The matched text is TRUE and the breakage is elsewhere.** ic's
//! `parity/tools/gen_register.sh:256` describes what v2 did — correct prose
//! that a `s|intent/st/|...|` sweep would rewrite into a false statement —
//! while the half that actually breaks asserts where v3 canon lives and
//! contains no `intent/st/` at all. **One line, refuted in both directions.**
//!
//! **(b) The literal is nowhere near the resolver.** `project.rs` carried the
//! literal `intent/st/` in a doc comment and in a test, and at NEITHER of the
//! two functions that resolve it. A literal-driven sweep edits the comment,
//! marks the file handled, and leaves the resolver pointing at v2 — **the
//! comment becomes a lie and nothing goes red.**
//!
//! **And the shape is FLAT.** Canon is `.canon/st/<ID>.json`, not
//! `<ID>/thread.json`, so `s|intent/st/|intent/.canon/|` yields
//! `intent/.canon/<ID>/thread.json` — right prefix, wrong structure, and it
//! reads as a correct migration.
//!
//! **No count is asserted anywhere here.** ic withdrew their own 3-of-17 once
//! they measured it as their probe's reach — one directory of 41 against a
//! repo-wide concern — and vc's narrowed 23-file probe missed `export.rs`
//! entirely. There is no honest single denominator, so the criterion is the
//! discrimination.

mod common;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use common::{Fixture, sample_thread};
use intentsvcs::model::{ISSUE_SCHEMA, Issue, IssueStatus};

/// Deliberately not `intent`, and deliberately not a name that appears
/// anywhere in the source: if any site hardcodes a directory, its artefacts
/// land outside this tree and the assertions below say so.
const ELSEWHERE: &str = "workspace";

fn issue(number: u32) -> Issue {
  serde_json::from_value(serde_json::json!({
    "schema": ISSUE_SCHEMA,
    "number": number,
    "slug": "a-defect",
    "title": "a defect",
    "status": IssueStatus::Open,
    "body": "",
    "created": "2026-08-18",
  }))
  .expect("issue")
}

fn files_under(root: &Path) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  let mut stack = vec![root.to_path_buf()];
  while let Some(dir) = stack.pop() {
    let Ok(entries) = std::fs::read_dir(&dir) else {
      continue;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        // The store is per-machine and is not a projected artefact.
        if path.file_name().is_some_and(|n| n == ".cache") {
          continue;
        }
        stack.push(path);
      } else if let Ok(rel) = path.strip_prefix(root) {
        out.insert(rel.to_string_lossy().replace('\\', "/"));
      }
    }
  }
  out
}

fn seeded_elsewhere() -> Fixture {
  let fx = Fixture::with_intent_dir(ELSEWHERE);
  for id in ["ST0001", "ST0002"] {
    fx.write_thread(&sample_thread(id));
  }
  fx.write_issue(&issue(7));
  let mut facade = fx.facade();
  facade
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect("ingest");
  facade
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("project");
  fx
}

/// **THE CRITERION.** Move the resolver's answer and every canon artefact
/// follows it, with no other edit anywhere.
#[test]
fn every_canon_artefact_follows_the_resolver_to_a_different_directory() {
  let fx = seeded_elsewhere();
  let all = files_under(fx.root());

  // **ONE NAMED EXEMPTION, AND IT IS THE BOOTSTRAP POINT.**
  // `Project::config_path` always answers `intent/.config/config.json`
  // regardless of `intent_dir`, deliberately: something must be findable
  // before anything is configured, and that file is what DECLARES where the
  // rest lives. It is not canon and it does not move. Exempted by exact path
  // rather than by prefix, so a second stray file under `intent/` is still
  // caught.
  const BOOTSTRAP: &str = "intent/.config/config.json";
  let stray: Vec<&String> = all
    .iter()
    .filter(|p| p.starts_with("intent/") && p.as_str() != BOOTSTRAP)
    .collect();
  assert!(
    stray.is_empty(),
    "the intent directory is `{ELSEWHERE}`, so nothing but {BOOTSTRAP} may land \
     under `intent/`. A path spelled independently lands here: {stray:?}"
  );

  for expected in [
    "workspace/.canon/st/ST0001.json",
    "workspace/.canon/st/ST0002.json",
    "workspace/.canon/issues/0007.json",
  ] {
    assert!(
      all.contains(expected),
      "canon followed the resolver: {expected} is missing from {all:?}"
    );
  }
}

/// A WRITE that follows the resolver is only half of it: the READ must follow
/// too, or the estate is writeable and unreadable.
#[test]
fn the_read_path_follows_the_resolver_as_well_as_the_write_path() {
  let fx = seeded_elsewhere();
  let facade = fx.facade();
  let mut ids: Vec<&str> = facade.st_list().iter().map(|t| t.id.as_str()).collect();
  // `st_list` carries the model's order, not a sorted one; this test is about
  // WHERE the reader found them, so it compares the set.
  ids.sort_unstable();
  assert_eq!(
    ids,
    vec!["ST0001", "ST0002"],
    "the reader found canon where the writer put it"
  );
}

/// **THE NEGATIVE ARM (AC-01.6), and it is the one a wholesale move fails.**
///
/// `thread_dir()` must STILL answer the view directory afterwards, because
/// `info.md` and `acceptance.md` hang off it. Relocating `thread_dir()`
/// wholesale satisfies AC-01.1's letter — the view directory holds no
/// `thread.json` — **while emptying the directory a reader browses.** Only
/// canon leaves.
#[test]
fn the_views_do_not_move_with_the_canon() {
  let fx = seeded_elsewhere();
  let project = fx.project();
  let root = fx.root();

  assert_eq!(
    project.thread_dir("ST0001"),
    root.join("workspace/st/ST0001"),
    "the view directory follows the intent dir, and is NOT under .canon/"
  );

  let all = files_under(root);
  for name in ["info.md", "acceptance.md"] {
    let rel = format!("workspace/st/ST0001/{name}");
    assert!(
      all.contains(&rel),
      "the views stay where a reader browses them: {rel} missing"
    );
  }

  let canon_in_views: Vec<&String> = all
    .iter()
    .filter(|p| p.starts_with("workspace/st/") && p.ends_with(".json"))
    .collect();
  assert!(
    canon_in_views.is_empty(),
    "no canon under the view directory: {canon_in_views:?}"
  );
}

/// **The composed relative forms are the only spelling in the codebase**, and
/// they compose from the resolver rather than from a literal.
///
/// This is what `export.rs:386` did NOT do before WP-01, which is the red-first
/// arm named in AT-01.6: it built `st/<ID>/thread.json` itself while every
/// reader resolved through `Project`.
#[test]
fn the_relative_forms_compose_from_the_resolver_and_not_from_a_literal() {
  let fx = Fixture::with_intent_dir(ELSEWHERE);
  let project = fx.project();
  let root = fx.root();

  // The absolute answers sit under the CONFIGURED directory...
  assert_eq!(
    project.thread_json("ST0001"),
    root.join("workspace/.canon/st/ST0001.json")
  );
  assert_eq!(
    project.issue_json(7),
    root.join("workspace/.canon/issues/0007.json")
  );

  // ...and the relative forms carry no directory name at all, which is what
  // lets one spelling serve every configuration.
  let rel: PathBuf = intentsvcs::project::canon_thread_rel("ST0001").into();
  assert!(
    !rel.starts_with("intent") && !rel.starts_with(ELSEWHERE),
    "a relative canon path names no intent directory: {rel:?}"
  );
  assert_eq!(
    project.intent_dir().join(&rel),
    project.thread_json("ST0001"),
    "the absolute answer IS the intent dir joined to the relative form -- \
     one composition, not two that must agree by convention"
  );
}

/// **THE RED-FIRST ARM THE CRITERION NAMES, AND THE FOUR TESTS ABOVE DO NOT
/// REACH IT.**
///
/// Measured: re-introducing the historical bug — `export.rs` building
/// `format!(".canon/st/{}/thread.json", id)` itself — left all four of them
/// GREEN. They drive `sync_from_disk`/`sync_to_disk`, which reach canon through
/// `Facade::projection` and `Project::thread_json`; **the EXPORTER is a
/// different route to the same files and nothing above went near it.** That is
/// the same shape as the AC-03.14 guard landing on `views::write_all` while
/// every real verb wrote through `WriteSet` — a correct test on a path the
/// failure does not take.
///
/// So this drives [`export::canon_parts`] directly, which is the exact site
/// that carried the defect, and requires its paths to BE the resolver's
/// answers rather than to agree with them.
#[test]
fn the_exporter_spells_no_canon_path_of_its_own() {
  let threads: Vec<_> = ["ST0001", "ST0002"]
    .iter()
    .map(|id| sample_thread(id))
    .collect();
  let issues = vec![issue(7), issue(46)];
  let bundle = intentsvcs::export::Bundle::new("p", threads.clone(), issues.clone(), Vec::new());

  let parts = intentsvcs::export::canon_parts(&bundle).expect("canon parts");
  let paths: Vec<&str> = parts.iter().map(|(p, _)| p.as_str()).collect();

  for thread in &threads {
    let resolved = intentsvcs::project::canon_thread_rel(&thread.id);
    assert!(
      paths.contains(&resolved.as_str()),
      "the exporter emitted no path equal to the resolver's {resolved}; it emitted {paths:?}"
    );
  }
  for issue in &issues {
    let resolved = intentsvcs::project::canon_issue_rel(issue.number);
    assert!(
      paths.contains(&resolved.as_str()),
      "the exporter emitted no path equal to the resolver's {resolved}; it emitted {paths:?}"
    );
  }

  // **46 is in the fixture on purpose.** The issue arm ALREADY shipped this
  // defect once: it emitted `issues/46.json` where every reader opened
  // `issues/0046.json`. Unpadded output is the one wrong answer known to have
  // reached production here, so it is named rather than left to the general
  // check above.
  assert!(
    paths.contains(&".canon/issues/0046.json"),
    "the issue arm is zero-padded: {paths:?}"
  );
}
