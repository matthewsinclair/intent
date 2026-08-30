//! `AC-08.11`'s structural witness: no blocking store call can occupy an async
//! worker, because no handler is ever given a type that blocks.
//!
//! **THE ROW REQUIRES A STRUCTURAL WITNESS AND EXPLICITLY REFUSES A BEHAVIOURAL
//! ONE, WHICH IS THE WHOLE REASON THIS FILE IS A SOURCE SCAN.** A latency test
//! measures the machine rather than the code: on an unloaded box it passes with
//! every line of the store discipline deleted, and on a loaded one it fails for
//! reasons that have nothing to do with the property. What is actually
//! enforceable is that the `Facade` never leaves `store.rs`, where it is owned
//! by a thread the runtime does not schedule.
//!
//! **THE COMPILER IS THE REAL ENFORCEMENT AND THIS IS THE WITNESS THAT IT STILL
//! IS.** Handlers hold a `ProjectHandle`, which is a channel sender; there is
//! no path from one to a blocking database call. This file exists because that
//! arrangement can be quietly undone -- not by malice, but by somebody who
//! reasonably wanted a store handle where they were working and imported one.
//! That change compiles and every test passes.
//!
//! Same shape as `dep_graph_guard.rs`, which enforces D06 by asserting that
//! `rusqlite` appears in exactly one crate manifest, and for the same reason: a
//! dependency you cannot name is a dependency you cannot misuse.

use std::fs;
use std::path::{Path, PathBuf};

/// The one module allowed to name the facade.
const THE_DOOR: &str = "store.rs";

/// Every `.rs` file under `crates/intentd/src`.
fn daemon_sources() -> Vec<PathBuf> {
  let src = crate_root().join("src");
  let mut found = Vec::new();
  collect(&src, &mut found);
  found.sort();
  found
}

fn collect(dir: &Path, into: &mut Vec<PathBuf>) {
  for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
    let path = entry.expect("dir entry").path();
    if path.is_dir() {
      collect(&path, into);
    } else if path.extension().is_some_and(|e| e == "rs") {
      into.push(path);
    }
  }
}

fn crate_root() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// File content with `//` comment lines removed.
///
/// **COMMENTS ARE STRIPPED SO THAT EXPLAINING THIS RULE IS NOT AN INSTANCE OF
/// BREAKING IT.** Every module that participates in the arrangement has a doc
/// comment naming `Facade` to say why it does not hold one, and a scan that
/// counted those would make the documentation the violation -- the same
/// property the whiteboard guards have, where quoting a bad timestamp to a peer
/// must not itself be an offence.
fn without_comments(path: &Path) -> String {
  fs::read_to_string(path)
    .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    .lines()
    .filter(|line| !line.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n")
}

#[test]
fn the_facade_is_named_in_exactly_one_module_of_this_crate() {
  let sources = daemon_sources();
  // **AN EMPTY CORPUS IS A FAILURE TO MEASURE, NOT A PASS.** If the source
  // layout moves, a scan that found nothing would report a clean sheet, and a
  // clean sheet from a blind instrument is indistinguishable from a real one.
  assert!(
    sources.len() >= 2,
    "expected several modules under crates/intentd/src, found {sources:?} -- this check measured nothing"
  );

  let holders: Vec<String> = sources
    .iter()
    .filter(|path| without_comments(path).contains("Facade"))
    .map(|path| {
      path
        .file_name()
        .expect("a file name")
        .to_string_lossy()
        .to_string()
    })
    .collect();

  assert_eq!(
    holders,
    vec![THE_DOOR.to_string()],
    "AC-08.11: the Facade must be reachable only from {THE_DOOR}, where it is owned by a thread the async runtime does not schedule. A module that names it can call it, and a blocking store call on a runtime worker starves the accept loop -- at which point the liveness probe times out on a healthy daemon and the CLI routes in-process against a store this process owns"
  );
}

#[test]
fn the_scan_would_notice_a_second_holder() {
  // **THE POSITIVE CONTROL, WITHOUT WHICH THE GREEN ABOVE IS WORTH NOTHING.**
  // A predicate that never matched would pass the test above for free, and
  // would go on passing after somebody imported a `Facade` into the connection
  // handler. So the same predicate is driven against content that must match.
  let door = crate_root().join("src").join(THE_DOOR);
  assert!(
    without_comments(&door).contains("Facade"),
    "the door itself must name the Facade -- if this fails the predicate is inert and the check above is decoration"
  );

  // And it must NOT match a module that only discusses it in prose.
  let commented = "// This module deliberately holds no Facade.\nfn nothing() {}\n";
  assert!(
    !commented
      .lines()
      .filter(|l| !l.trim_start().starts_with("//"))
      .any(|l| l.contains("Facade")),
    "a comment naming the Facade must not count as holding one"
  );
}

#[test]
fn no_module_outside_the_door_blocks_the_runtime() {
  // **THE SECOND HALF OF THE SAME PROPERTY, AND IT IS NOT IMPLIED BY THE
  // FIRST.** A module could starve the accept loop without ever naming a
  // `Facade` -- `blocking_recv`, `blocking_send` and `block_on` all park a
  // runtime worker, and the first of those is CORRECT inside the door, on the
  // dedicated thread whose entire job is to block.
  let offenders: Vec<String> = daemon_sources()
    .iter()
    .filter(|path| path.file_name().is_some_and(|n| n != THE_DOOR))
    .filter(|path| {
      let body = without_comments(path);
      body.contains("blocking_recv")
        || body.contains("blocking_send")
        || body.contains("block_on")
        || body.contains("blocking_lock")
    })
    .map(|path| path.display().to_string())
    .collect();

  assert!(
    offenders.is_empty(),
    "AC-08.11: these modules park a runtime worker, which is what the store thread exists to prevent: {offenders:?}"
  );
}
