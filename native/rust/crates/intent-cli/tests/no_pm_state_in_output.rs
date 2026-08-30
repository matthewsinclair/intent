//! AT-00.8 / AC-00.9: **Intent's own project-management state never reaches
//! Intent's output.**
//!
//! hv, directly: _"NEVER EVER put Intent project management state like ST or WP
//! numbers or ACs etc into output from Intent. Some other project doesn't care
//! about an AC or a WP or even a test that is in the Intent project itself."_
//!
//! **The hard part is REFERENT, not regex**, and the whole design follows from
//! it. `ST0000` in help text names the reader's own STZero retrofit and is
//! correct; `ST0056` in a remedy names a thread in a repository the reader
//! cannot see. The two are indistinguishable by shape, so this file keys on
//! **what an identifier refers to** and blesses exactly one: an id that names
//! something present in EVERY Intent project. Everything else names Intent's
//! own development, because at build time there is nothing else it could name.
//!
//! **Presence in a binary is not emission, and a check that conflates them is
//! worthless.** dc measured `strings <binary> | grep` on the adjacent
//! `INTENT_HOME` question and it was 100% false-positive: the embedded parity
//! prose is in the binary and no code path reads it. So each surface below
//! either DRIVES the shipped binary and reads what came out, or establishes the
//! reachability precondition it depends on -- never "is the string in there".
//!
//! Four surfaces, because the leak has four carriers and three of them were
//! invisible to the first specification of this test:
//!
//! 1. **The schema faces**, driven through `intent schema`. **This is the
//!    largest carrier and it was not in the original method at all.** The faces
//!    are generated from `///` doc comments -- schemars lifts them into the
//!    JSON Schemas, async-graphql into the SDL -- and `render.rs`'s `schema`
//!    arm `print!`s the result verbatim into the caller's terminal. A comment
//!    is exempt from this criterion because a consumer never sees it; a comment
//!    that a generator publishes into a face the CLI prints is seen, so the
//!    exemption does not reach it.
//! 2. **Help text**, driven for every command the dispatch table declares.
//! 3. **Inline string literals** in shipped source -- error messages, remedies,
//!    refusals, worked examples. Comments and the trailing `#[cfg(test)]`
//!    module are excluded, and the exclusion is proven against the real corpus
//!    below rather than against a fixture.
//! 4. **The roadmap-carrying asset field stays UNREAD.** `Entry.owner_wp`
//!    records which of Intent's work packages owes each ported command, and it
//!    does not reach a renderer. Its VALUES are legitimately Intent's port
//!    ledger and `gen_dispatch_table.sh` consumes them; what would make it a
//!    leak is a renderer, so that is what is guarded. See
//!    [`the_roadmap_field_is_not_reachable_from_any_renderer`] for why this is
//!    stronger than checking the values, and for the deviation from AC-00.9's
//!    letter it implies.
//!
//! The model's own `Disposition::Unbuilt` carried the same thing as an
//! `owed_by: "WP-06"` and **was removed rather than guarded**: `intentsvcs` is a
//! library another project can link, so a field naming our work packages had no
//! business in it, and keeping it would have forced surface 3 to grow a
//! per-field exemption. A guard with no exemptions is worth more than a field
//! whose content was never consumer-facing.

use std::path::{Path, PathBuf};
use std::process::Command;

use testkit::repo_root;

mod common;
use common::{declared_paths, shipped_sources, string_literals};

// ---------------------------------------------------------------------------
// The referent rule
// ---------------------------------------------------------------------------

/// **The one identifier that names something in the READER's project.**
///
/// `ST0000` is the STZero retrofit convention, present in every Intent project,
/// so "Retrofit ST0000 deliverables into a brownfield project" is about the
/// reader's own estate and is correct. It is also therefore the right id for a
/// worked example -- the only ST id guaranteed to resolve to something the
/// reader can actually look at.
const READERS_OWN: &str = "ST0000";

/// Whether a `D42`-shaped identifier can be decided on this surface.
///
/// **It cannot be decided in prose, and that was MEASURED rather than
/// reasoned** (cc, 2026-08-15). The first cut of this file counted design
/// decisions everywhere, and the shipped surface immediately produced a false
/// positive that no amount of regex fixes: `intent st bootstrap --help` says
/// _"Target a single deliverable (D2-D11)"_, where **`D11` is STZero
/// deliverable eleven -- a thing in the READER's own project** -- and it is
/// indistinguishable in shape from `D15`, a design decision in ours. Both are
/// `D` plus two digits. This is the `ST0000`-versus-`ST0056` trap in a second
/// class, and unlike that one it has no blessable single value to carve out.
///
/// **The consequence is that the D class is enforced only where the ambiguity
/// cannot arise**, rather than being enforced badly everywhere or dropped
/// everywhere. A guard that fires on correct help text gets switched off, and
/// then it is protecting nothing at all.
#[derive(Clone, Copy, PartialEq)]
enum Decisions {
  /// **A schema face, where the D class IS decidable.** A face describes the
  /// model -- threads, work packages, criteria, events -- and STZero
  /// deliverables are not in it, so no `D`-shaped identifier there can be
  /// naming the reader's project. Anything that looks like a design decision in
  /// a published contract is one.
  Counted,
  /// **Prose the CLI emits, where it is not.** Help text and remedies talk
  /// about the reader's estate, which has deliverables `D1`..`D11` in it.
  /// Design provenance still does not belong in these strings; it is simply not
  /// mechanically separable from correct content, so it is left to review
  /// rather than asserted falsely.
  Ambiguous,
}

/// Find every identifier that names Intent's own project-management state.
///
/// Hand-rolled rather than a regex dependency, for the same reason
/// `store_schema_version.rs` writes out FNV-1a: the property wanted is "spot
/// these shapes", which needs no engine, and a shipped crate should not grow a
/// dependency to get it.
fn pm_identifiers(text: &str, decisions: Decisions) -> Vec<String> {
  let b = text.as_bytes();
  let mut found = Vec::new();
  let mut i = 0;
  while i < b.len() {
    match identifier_at(b, i, decisions) {
      Some(end) => {
        let id = &text[i..end];
        if id != READERS_OWN {
          found.push(id.to_string());
        }
        i = end;
      }
      None => i += 1,
    }
  }
  found
}

/// The end offset of a project-management identifier starting at `i`, if there
/// is one.
fn identifier_at(b: &[u8], i: usize, decisions: Decisions) -> Option<usize> {
  // A left boundary, so `ID42` is not read as `D42` and `XST0056` is not read
  // as `ST0056`.
  if i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
    return None;
  }
  let digits = |from: usize, n: usize| -> bool {
    from + n <= b.len() && b[from..from + n].iter().all(u8::is_ascii_digit)
  };
  // A right boundary that only excludes MORE digits and letters: `AC-02.8's`
  // and `WP-06,` are both the identifier plus punctuation, and rejecting them
  // would let a leak hide behind an apostrophe.
  let ends = |at: usize| -> bool { at >= b.len() || !b[at].is_ascii_alphanumeric() };

  // `ST0056`, and NOT `ST00567`.
  if b[i..].starts_with(b"ST") && digits(i + 2, 4) && ends(i + 6) {
    return Some(i + 6);
  }
  // `WP-06`.
  if b[i..].starts_with(b"WP-") && digits(i + 3, 2) && ends(i + 5) {
    return Some(i + 5);
  }
  // `AC-02.8`, `AT-00.11` -- group of two, then a dot, then a sequence.
  for prefix in [b"AC-", b"AT-"] {
    if b[i..].starts_with(prefix) && digits(i + 3, 2) && i + 5 < b.len() && b[i + 5] == b'.' {
      let mut end = i + 6;
      while end < b.len() && b[end].is_ascii_digit() {
        end += 1;
      }
      if end > i + 6 && ends(end) {
        return Some(end);
      }
    }
  }
  // `D42` -- a design-decision number, counted only where a `D`-shaped id
  // cannot be one of the reader's own STZero deliverables. See [`Decisions`].
  if decisions == Decisions::Counted && b[i] == b'D' && digits(i + 1, 2) && ends(i + 3) {
    return Some(i + 3);
  }
  None
}

// ---------------------------------------------------------------------------
// Driving the shipped binary
// ---------------------------------------------------------------------------

/// Run the shipped binary somewhere that is NOT an Intent project.
///
/// Deliberately a tempdir. Run inside this repository and a command could pick
/// up Intent's own estate and print Intent's own thread ids legitimately -- the
/// test would then be measuring the fixture rather than the binary, and the
/// leaks it exists to find would be indistinguishable from correct output about
/// the project it was standing in.
fn run(dir: &Path, args: &[&str]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .current_dir(dir)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  let mut both = String::from_utf8_lossy(&out.stdout).into_owned();
  both.push_str(&String::from_utf8_lossy(&out.stderr));
  both
}

// Surface 1 -- the schema faces, which the CLI prints verbatim
// ---------------------------------------------------------------------------

/// **The largest carrier, and the one a source-literal scan cannot see.**
///
/// `intent schema <face>` does `print!("{content}")` on a face generated from
/// the model's `///` doc comments. So a doc comment on a modelled type is not a
/// comment as far as this criterion is concerned -- it is a paragraph the CLI
/// prints into a stranger's terminal. Every design-provenance sentence that
/// belongs in the source belongs in a `//`, which no generator lifts.
#[test]
fn the_schema_faces_the_cli_prints_carry_no_pm_state() {
  let dir = tempfile::tempdir().expect("tempdir");
  let mut offenders = Vec::new();

  let names: Vec<String> = std::fs::read_dir(repo_root().join("schema"))
    .expect("schema/ exists")
    .filter_map(Result::ok)
    .filter(|e| e.path().is_file())
    .map(|e| e.file_name().to_string_lossy().into_owned())
    .collect();
  assert!(!names.is_empty(), "precondition: schema/ carries the faces");

  // The bare verb prints every face; the named ones prove the per-face path is
  // not a different renderer with a different exposure.
  let mut invocations: Vec<Vec<&str>> = vec![vec!["schema"]];
  for name in &names {
    invocations.push(vec!["schema", name.as_str()]);
  }

  for args in invocations {
    let printed = run(dir.path(), &args);
    for id in pm_identifiers(&printed, Decisions::Counted) {
      offenders.push(format!("`intent {}` printed `{id}`", args.join(" ")));
    }
  }

  assert!(
    offenders.is_empty(),
    "the schema faces the CLI prints carry Intent's own project-management state.\n\
     A `///` on a modelled type is PUBLISHED -- schemars lifts it into the JSON Schema and \
     async-graphql into the SDL, and `intent schema` prints the result.\n\
     Move the reasoning to a plain `//` and leave the `///` saying what a consumer needs, then \
     re-bless with `INTENT_BLESS=1 cargo test -p intentsvcs --test schema_faces_drift`:\n  {}",
    offenders.join("\n  ")
  );
}

// ---------------------------------------------------------------------------
// Surface 2 -- help text
// ---------------------------------------------------------------------------

/// Every declared command's usage block, driven rather than read.
///
/// This is where `Entry.help` and `Flag.help` from the compiled-in dispatch
/// table actually surface, so driving it covers the asset fields that reach a
/// renderer without asserting anything about the 100-odd the narrow serde
/// structs silently drop. **Asserting on the whole asset would be red on
/// correct data**, which is how a check gets disabled.
#[test]
fn every_declared_commands_help_carries_no_pm_state() {
  let dir = tempfile::tempdir().expect("tempdir");
  let mut offenders = Vec::new();
  let mut blessed_seen = 0;

  let mut invocations: Vec<Vec<String>> = vec![vec!["--help".to_string()]];
  for path in declared_paths() {
    let mut args: Vec<String> = path.split_whitespace().map(str::to_string).collect();
    args.push("--help".to_string());
    invocations.push(args);
  }

  for args in &invocations {
    let borrowed: Vec<&str> = args.iter().map(String::as_str).collect();
    let printed = run(dir.path(), &borrowed);
    if printed.contains(READERS_OWN) {
      blessed_seen += 1;
    }
    for id in pm_identifiers(&printed, Decisions::Ambiguous) {
      offenders.push(format!("`intent {}` printed `{id}`", args.join(" ")));
    }
  }

  assert!(
    offenders.is_empty(),
    "help text carries Intent's own project-management state -- say what is unavailable and what \
     to do instead, never who owes it or under which work package:\n  {}",
    offenders.join("\n  ")
  );

  // **The carve-out, proven on the real surface rather than a fixture.**
  // `ST0000` genuinely appears in shipped help text, describing the reader's
  // own STZero retrofit. If this drops to zero the corpus stopped exercising
  // the distinction and the test above is no longer discriminating -- it would
  // be passing because there is nothing left to get wrong.
  assert!(
    blessed_seen > 0,
    "no shipped help text mentions {READERS_OWN} any more, so this test no longer proves it can \
     tell the reader's own ids from Intent's -- which is the only hard part of the check"
  );
}

// ---------------------------------------------------------------------------
// Surface 3 -- inline string literals in shipped source
// ---------------------------------------------------------------------------

/// No error, remedy, refusal or worked example names Intent's own state.
#[test]
fn no_shipped_string_literal_carries_pm_state() {
  let mut offenders = Vec::new();
  for file in shipped_sources() {
    let code = std::fs::read_to_string(&file).expect("read shipped source");
    let shown = file
      .strip_prefix(repo_root())
      .unwrap_or(&file)
      .display()
      .to_string();
    for literal in string_literals(&code) {
      for id in pm_identifiers(&literal, Decisions::Ambiguous) {
        let excerpt: String = literal.chars().take(90).collect();
        offenders.push(format!("{shown}: `{id}` in \"{excerpt}\""));
      }
    }
  }
  assert!(
    offenders.is_empty(),
    "shipped strings carry Intent's own project-management state. A consumer reading \"owed by \
     WP-06\" learns nothing they can act on; say what is unavailable and what to do instead. \
     Worked examples use {READERS_OWN}, which names something in the reader's own project:\n  {}",
    offenders.join("\n  ")
  );
}

/// **The exemption is proven against the real corpus, not against a fixture.**
///
/// AC-00.9 exempts comments, and the test has to demonstrate it tolerates them
/// or it gets disabled the first time it fires on a doc comment. A fixture
/// would prove only that the scanner handles the fixture; the shipped source
/// carries hundreds of these identifiers in comments right now, and the test
/// above is green, which is the claim itself.
///
/// The floor is asserted so the demonstration cannot quietly become vacuous: if
/// the comments were all rewritten away, this test would keep passing while
/// proving nothing.
#[test]
fn the_scan_tolerates_pm_state_in_comments_at_the_scale_it_actually_occurs() {
  let mut in_comments = 0;
  for file in shipped_sources() {
    let code = std::fs::read_to_string(&file).expect("read shipped source");
    let literals = string_literals(&code);
    let in_literals: usize = literals
      .iter()
      .map(|l| pm_identifiers(l, Decisions::Ambiguous).len())
      .sum();
    let everywhere = pm_identifiers(&code, Decisions::Ambiguous).len();
    in_comments += everywhere.saturating_sub(in_literals);
  }
  assert!(
    in_comments > 100,
    "shipped source carries only {in_comments} project-management identifiers in comments, so \
     this file no longer demonstrates that the exemption holds under load"
  );
}

// ---------------------------------------------------------------------------
// Surface 4 -- the roadmap fields stay unreachable
// ---------------------------------------------------------------------------

/// **The structural half, and it is guarded as REACHABILITY rather than as
/// content.**
///
/// AC-00.9's finding is that the leak was designed in: a model field whose job
/// is to carry Intent's roadmap, wired to a renderer. Two such fields exist --
/// `Entry.owner_wp` from the dispatch table and `Disposition::owed_by` in
/// `transitions.rs` -- and **as measured on 2026-08-15 neither has a read site
/// anywhere in shipped source.** The `owner()` function and the `render.rs`
/// remedy that AC-00.9 traces are both gone.
///
/// **So their values are checked by nothing here, deliberately, and that is a
/// deviation from AC-00.9's letter which reads "an Intent WP id in `owner_wp`"
/// as a red-first case** (cc, with vc). Three reasons, and the third is why the
/// deviation makes the guard stronger rather than weaker:
///
/// 1. **The values are legitimate.** `owner_wp` is Intent's port ledger, and
///    ic's `gen_dispatch_table.sh` has three consumers for it. It is not
///    output. Requiring it clean would delete a working instrument to satisfy a
///    criterion about output.
/// 2. **Checking content here would be a presence check**, which this whole
///    file exists to avoid -- and which the AT's own method forbids, having
///    measured that instrument at 100% false-positive.
/// 3. **A content check passes on the real defect.** Launder every value today
///    and the field is still one `println!` away from a consumer's terminal;
///    the next value added is not laundered, and nothing notices. Guarding the
///    RENDERER catches the wiring, which is the act that turns a ledger into a
///    leak -- and the surfaces above then cover the content, because once it is
///    rendered it is output. **Mutation-tested in that direction**: wiring
///    `owner_wp` to a reader with a perfectly clean value fails this test, which
///    is precisely what a value check cannot do.
///
/// Only `owner_wp` is watched, because it is the only such field left. The
/// model's `Disposition::owed_by` is gone rather than guarded, so an arm for it
/// here would match nothing and read like coverage -- and a reintroduction would
/// be a `"WP-06"` literal, which surface 3 catches without help.
#[test]
fn the_roadmap_field_is_not_reachable_from_any_renderer() {
  const FIELD: &str = "owner_wp";
  let mut reads = Vec::new();
  let mut declarations = 0;
  for file in shipped_sources() {
    let code = std::fs::read_to_string(&file).expect("read shipped source");
    let shown = file
      .strip_prefix(repo_root())
      .unwrap_or(&file)
      .display()
      .to_string();
    for (n, line) in code.lines().enumerate() {
      let t = line.trim();
      if t.starts_with("//") {
        continue;
      }
      // A declaration (`pub owner_wp: String`) and an initialiser
      // (`owner_wp: String::new()`) are both `<field>:`. Anything else -- a
      // field access, a destructure, a format argument -- is a READ, and a read
      // is one step from a renderer.
      let mut from = 0;
      while let Some(at) = t[from..].find(FIELD) {
        let at = from + at;
        if t[at + FIELD.len()..].starts_with(':') {
          declarations += 1;
        } else {
          reads.push(format!("{shown}:{}: {t}", n + 1));
          break;
        }
        from = at + FIELD.len();
      }
    }
  }
  assert!(
    reads.is_empty(),
    "`{FIELD}` carries Intent's roadmap and is now READ in shipped code, which is the act that \
     turns a debt ledger into a leak into someone else's terminal.\n\
     If wiring it is deliberate, its values must first stop naming Intent's work packages -- say \
     what is unavailable and what to do instead:\n  {}",
    reads.join("\n  ")
  );
  // **The check must not pass because the field vanished.** A guard whose
  // subject is gone reports "no reads" forever and reads exactly like a guard
  // that is working.
  assert!(
    declarations > 0,
    "`{FIELD}` is no longer declared in shipped source, so this test is asserting the absence of \
     reads of a field that does not exist -- delete it or point it at what replaced it"
  );
}

// ---------------------------------------------------------------------------
// The referent rule itself
// ---------------------------------------------------------------------------

/// The two red-first cases AC-00.9 names, plus the boundaries that make the
/// scan honest.
///
/// **(b) is the one a naive implementation fails**: a regex over `ST0\d{3}`
/// cannot tell the reader's own STZero from Intent's thread, so it either
/// misses the leaks or deletes correct help -- and both end with the check
/// switched off.
#[test]
fn the_rule_keys_on_referent_and_not_on_shape() {
  let anywhere = Decisions::Ambiguous;
  let face = Decisions::Counted;

  // (a) Intent's own state, in every class that is decidable anywhere.
  assert_eq!(pm_identifiers("owed by WP-06", anywhere), vec!["WP-06"]);
  assert_eq!(pm_identifiers("see ST0056", anywhere), vec!["ST0056"]);
  assert_eq!(pm_identifiers("covers AC-02.8", anywhere), vec!["AC-02.8"]);
  assert_eq!(pm_identifiers("covers AT-00.8", anywhere), vec!["AT-00.8"]);

  // (b) the reader's own project, which must stay green. This is the case a
  // regex over the identifier's SHAPE cannot get right, and getting it wrong in
  // either direction ends with the check disabled.
  assert!(pm_identifiers("Retrofit ST0000 deliverables", anywhere).is_empty());
  assert!(pm_identifiers("name it as `<ST id>/<NN>`, eg ST0000/03", anywhere).is_empty());

  // **The same trap in the D class, which has no blessable value** -- so the
  // surface decides, not the string. Shipped help text really does say the
  // first of these, and a face really does say the second.
  assert!(pm_identifiers("Target a single deliverable (D2-D11)", anywhere).is_empty());
  assert_eq!(
    pm_identifiers("carried over unchanged (D17)", face),
    vec!["D17"]
  );

  // Boundaries. `ends` deliberately admits punctuation, because a leak must not
  // be able to hide behind an apostrophe or a comma.
  assert_eq!(
    pm_identifiers("AC-02.8's remedy", anywhere),
    vec!["AC-02.8"]
  );
  assert_eq!(
    pm_identifiers("WP-06, WP-07", anywhere),
    vec!["WP-06", "WP-07"]
  );
  // ...and refuses to read an identifier out of the middle of a word.
  assert!(pm_identifiers("XST0056", anywhere).is_empty());
  assert!(pm_identifiers("ID42", face).is_empty());
  assert!(pm_identifiers("ST00567", anywhere).is_empty());
  assert!(pm_identifiers("D420", face).is_empty());
}

/// The literal scanner sees literals and not comments -- including a literal
/// whose interior lines look exactly like comments, which is the DDL's shape
/// and the case a line-based scan gets backwards.
#[test]
fn the_literal_scan_separates_published_text_from_source_commentary() {
  let code = "\
// a line comment naming ST0056
/// a doc comment naming WP-06
let e = \"error: nothing here\";
const DDL: &str = \"\\
-- a SQL comment INSIDE a literal, naming D42
CREATE TABLE t (id TEXT);\";
let r = r#\"a raw string naming AC-02.8\"#;
let q = s.trim_matches('\"');
let n = t.contains('\\n');
let s: &'static str = \"WP-99 is inside a literal after a lifetime\";
";
  let literals = string_literals(code);
  let found: Vec<String> = literals
    .iter()
    .flat_map(|l| pm_identifiers(l, Decisions::Counted))
    .collect();
  assert_eq!(
    found,
    vec!["D42", "AC-02.8", "WP-99"],
    "the identifiers inside literals are found and the two inside comments are not -- and the \
     `'\"'` char literal did not throw the scan off, which is what a naive quote-counter does; \
     got literals: {literals:?}"
  );
}

/// The trailing test module is excluded, and its exclusion is bounded.
#[test]
fn intents_own_test_fixtures_are_out_of_scope() {
  let code = "\
let shipped = \"remedy: run `intent doctor`\";
#[cfg(test)]
mod tests {
  fn f() { let id = \"ST0056\"; }
}
";
  assert!(
    string_literals(code)
      .iter()
      .all(|l| pm_identifiers(l, Decisions::Ambiguous).is_empty()),
    "a fixture inside `#[cfg(test)]` is never compiled into a shipped binary"
  );
  assert!(
    string_literals(code).iter().any(|l| l.contains("doctor")),
    "the truncation must not swallow the shipped code above the test module"
  );
}

// ---------------------------------------------------------------------------
// AT-00.17 -- the third surface: files Intent WRITES INTO other repositories
// ---------------------------------------------------------------------------
//
// `AC-00.9` was EXTENDED on 2026-08-20 by hv, direct, on seeing thread ids in
// code again, and the extension is what this module covers. Everything above
// this line tests OUTPUT -- what the shipped binaries emit. **That is the
// narrower of the two readings and the payload fell through the difference.**
//
// A template copied by `st new`, a hook installed into `.git/hooks/`, a skill
// installed into `.claude/`, and the rule library served to a consumer's agent
// are none of them binary output, and none of them are comments a consumer
// never sees. They are the opposite: artefacts whose whole purpose is to be
// read inside somebody else's project.
//
// **THE DISCRIMINATOR IS REFERENT, NOT SHAPE**, which is the same method the
// `ST0000`-versus-`ST0056` trap is closed on above rather than a second
// invention. `intent st show ST0042` teaches syntax and costs a consumer
// nothing -- `ST0042` names no thread in this estate, so it is a placeholder
// that happens to be four digits. `ST0035` names a real one, and a reader who
// follows it arrives at a tracker they cannot open.
//
// Thread-relative ids are stricter and the reason is that they are strictly
// less resolvable: a consumer meeting `AC-07.4` in an installed hook has no
// thread to resolve it against, so there is no referent test to apply and the
// id cannot be anything but a citation.

use std::collections::BTreeSet;

/// Every steel-thread id that names a REAL thread in this estate.
///
/// Read from the canon extract's filenames rather than from a list written
/// here, because a list would be a second home for the thread roster and would
/// go stale in exactly the direction that matters: a NEW thread cited in
/// payload would stop being detected on the day it was created.
fn real_thread_ids() -> BTreeSet<String> {
  let dir = repo_root().join("intent/.canon/st");
  let mut ids = BTreeSet::new();
  for entry in std::fs::read_dir(&dir).expect("intent/.canon/st is readable") {
    let path = entry.expect("a readable dir entry").path();
    if path.extension().and_then(|e| e.to_str()) == Some("json") {
      if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        ids.insert(stem.to_string());
      }
    }
  }
  ids
}

/// Every file Intent installs into somebody else's repository.
///
/// `lib/templates/` is what `intent init` and the canon engine lay down;
/// `intent/plugins/claude/` is the skills, subagents and rule library served to
/// a consumer's agent. Both are payload by definition -- nothing here is read
/// only inside this repo.
fn installed_payload() -> Vec<(PathBuf, String)> {
  let root = repo_root();
  let mut out = Vec::new();
  for sub in ["lib/templates", "intent/plugins/claude"] {
    collect_text_files(&root.join(sub), &mut out);
  }
  out
}

fn collect_text_files(dir: &Path, out: &mut Vec<(PathBuf, String)>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      collect_text_files(&path, out);
    } else if let Ok(text) = std::fs::read_to_string(&path) {
      out.push((path, text));
    }
  }
}

/// Files whose whole subject is the SHAPE of an acceptance record, where a
/// `WP-01` or an `AC-01.1` is the format being taught rather than a pointer.
///
/// **THE EXEMPTION IS PROVEN, NOT ASSERTED** -- see
/// [`the_template_exemption_covers_only_real_templates`]. It is also NARROW: it
/// reaches thread-relative ids only, so a resolving `STnnnn` in one of these
/// files still fires. That matters because the template DID carry one
/// (`Exemption (ST0048)`) and an exemption keyed to the file rather than to the
/// id class would have swallowed it.
const FORMAT_TEMPLATES: [&str; 2] = [
  "lib/templates/prj/st/ST####/acceptance.md",
  "intent/plugins/claude/skills/in-tca-synthesize/SKILL.md",
];

/// Every AT id that CITES a given payload file, read from the register.
///
/// **THE REGISTER REQUIRES A CITED FILE TO NAME THE ROW THAT CITES IT** -- the
/// close-gate refuses a citation the file does not carry, because a one-way
/// pointer drifts silently. So an `AT-nn.n` inside the file that row cites is a
/// STRUCTURAL id, not a pointer into a tracker a consumer cannot open.
///
/// **THIS WAS FOUND BY BREAKING IT.** Stripping `AT-10.13` out of
/// `append-only-guard.sh` as a citation immediately blocked the close-gate:
/// `does not carry the literal id AT-10.13`. Two obligations in genuine
/// tension, and the resolution is to compute the exemption from the register
/// rather than to weaken either one -- so it covers exactly the ids the
/// register demands, in exactly the files it demands them in, and moves on its
/// own when a citation moves.
fn register_ids_for(rel: &str) -> BTreeSet<String> {
  // EVERY thread's canon, not this one's. The first cut read `ST0056.json`
  // alone and immediately lost `AT-01.5`, which is ST0057's citation of a
  // shipped guard -- so the narrow version exempted nothing in that file and
  // the sweep stripped a load-bearing marker. A citation can come from any
  // thread; the file being cited has no idea which.
  let dir = repo_root().join("intent/.canon/st");
  let mut text = String::new();
  for entry in std::fs::read_dir(&dir).expect("intent/.canon/st is readable") {
    let path = entry.expect("a readable dir entry").path();
    if path.extension().and_then(|e| e.to_str()) == Some("json") {
      text.push_str(&std::fs::read_to_string(&path).expect("a readable canon file"));
    }
  }
  let mut out = BTreeSet::new();
  // The extract is JSON; the citation and the id sit on the same test record,
  // so a line-oriented scan over the rendered rows is enough and needs no
  // parser dependency.
  for chunk in text.split("\"id\":") {
    if let Some(id_end) = chunk.find(',') {
      let id: String = chunk[..id_end].trim().trim_matches('"').to_string();
      if id.starts_with("AT-") && chunk.contains(rel) {
        out.insert(id);
      }
    }
  }
  out
}

/// A marker that a file is teaching a shape: a `{placeholder}`, a run of `X`
/// standing for a count, or a `####` path segment that no thread id can match.
fn looks_like_a_template(path: &str, text: &str) -> bool {
  path.contains("####") || text.contains("| {") || text.contains("{name}")
}

/// **THE EXEMPTION CAN ONLY EVER COVER A TEMPLATE, AND ONLY EVER A
/// THREAD-RELATIVE ID.** Both halves are checked here rather than trusted at
/// the call site, so adding a path to `FORMAT_TEMPLATES` that is not a template,
/// or one that later grows a real citation, fails loudly instead of quietly
/// widening the hole.
#[test]
fn the_template_exemption_covers_only_real_templates() {
  let real = real_thread_ids();
  let root = repo_root();
  for rel in FORMAT_TEMPLATES {
    let path = root.join(rel);
    let text = std::fs::read_to_string(&path)
      .unwrap_or_else(|_| panic!("`{rel}` is exempted and must exist"));
    assert!(
      looks_like_a_template(rel, &text),
      "`{rel}` is exempted as a format template and carries no template marker"
    );
    let resolving: Vec<String> = pm_identifiers(&text, Decisions::Ambiguous)
      .into_iter()
      .filter(|id| id.starts_with("ST") && real.contains(id))
      .collect();
    assert!(
      resolving.is_empty(),
      "`{rel}` is exempted for its PLACEHOLDERS and carries a real citation: {resolving:?}"
    );
  }
}

/// The citations in one payload file: identifiers that point into Intent's own
/// tracker rather than teaching a consumer a shape.
fn citations_in_at(path: &str, text: &str, real: &BTreeSet<String>) -> Vec<String> {
  let exempt = FORMAT_TEMPLATES.contains(&path);
  let structural = if path.is_empty() {
    BTreeSet::new()
  } else {
    register_ids_for(path)
  };
  pm_identifiers(text, Decisions::Ambiguous)
    .into_iter()
    .filter(|id| {
      if id.starts_with("ST") {
        // Referent: a four-digit thread id is a citation only if it resolves.
        real.contains(id)
      } else {
        // `WP-nn`, `AC-n.n`, `AT-n.n` -- no thread context, so nothing to
        // resolve against and nothing it can be but a citation, EXCEPT in a
        // file whose subject is the shape of an acceptance record, or where the
        // register itself demands the file name the row that cites it.
        !exempt && !structural.contains(id)
      }
    })
    .collect()
}

/// **THE POSITIVE CONTROL FOR THE CORPUS.** Every assertion below iterates the
/// payload, so an empty payload passes all of them for free -- a moved
/// directory, a renamed root, a `read_dir` that silently returns nothing.
#[test]
fn the_installed_payload_is_a_real_corpus_and_the_thread_roster_resolves() {
  let payload = installed_payload();
  assert!(
    payload.len() > 100,
    "installed payload came back as {} file(s); every citation assertion below \
     would then iterate almost nothing and pass for free",
    payload.len()
  );
  let real = real_thread_ids();
  assert!(
    real.len() > 20,
    "the thread roster came back as {} id(s), so the referent filter would \
     classify real citations as placeholders and report a clean payload",
    real.len()
  );
  assert!(
    real.contains("ST0056"),
    "the roster must contain this thread, or the referent test is reading the \
     wrong directory"
  );
}

/// **THE DISCRIMINATOR IS DRIVEN IN BOTH DIRECTIONS**, because a filter that
/// flagged everything would satisfy the leak assertion trivially and report a
/// defect it had not measured.
///
/// Driven against a SYNTHETIC roster rather than the live one, so this tests
/// the filter and not the estate. The first cut used the live roster and
/// asserted `ST0042` was a placeholder -- **which failed, because `ST0042` is a
/// real thread here.** That was the right failure and it is why the two
/// concerns are now separate tests: a control whose verdict moves when somebody
/// creates a thread is measuring the wrong thing.
#[test]
fn the_referent_filter_separates_a_placeholder_from_a_citation() {
  let roster: BTreeSet<String> = ["ST0056".to_string()].into_iter().collect();

  assert!(
    citations_in_at("", "intent st show ST4242", &roster).is_empty(),
    "an id absent from the roster names no thread, so it teaches syntax"
  );
  assert!(
    citations_in_at("", "\"${CANON_DIR}/st/ST9999.json\"", &roster).is_empty(),
    "`ST9999` is a deliberate probe for a path that does not exist"
  );
  assert!(
    citations_in_at("", "see ST0000 for the retrofit", &roster).is_empty(),
    "`ST0000` is the READER's own thread and is blessed above"
  );
  assert_eq!(
    citations_in_at("", "# COVERS ST0056 AC-10.13 / AT-10.13.", &roster),
    vec!["ST0056", "AC-10.13", "AT-10.13"],
    "a rostered thread id and two thread-relative ids are all citations"
  );
}

/// **THE ESTATE'S OWN WORKED-EXAMPLE IDS RESOLVE TO REAL THREADS, AND THAT IS
/// THE FINDING RATHER THAN A FIXTURE DETAIL.**
///
/// `AC-00.9`'s text argues that `intent st show ST0042` "teaches syntax and
/// costs a consumer nothing -- the id is a placeholder that happens to be four
/// digits". **Measured 2026-08-30: it is not a placeholder.** `ST0042`,
/// `ST0005` and `ST0001` are all real threads in this estate, so every shipped
/// worked example built on them points a consumer at a tracker they cannot
/// open, by the criterion's own definition of a citation.
///
/// `READERS_OWN` is the resolution and the file already says so: `ST0000` is
/// the only id guaranteed to resolve to something the reader can actually look
/// at. This test pins the collision so a worked example cannot quietly go back
/// to a four-digit id that happens to be free today and is taken next month.
#[test]
fn the_worked_example_ids_in_shipped_payload_are_not_placeholders() {
  let real = real_thread_ids();
  for id in ["ST0042", "ST0005", "ST0001"] {
    assert!(
      real.contains(id),
      "`{id}` was expected to resolve here -- if it no longer does, the payload \
       finding this test records has changed and wants re-measuring, not deleting"
    );
  }
  assert!(
    !real.contains(READERS_OWN),
    "`{READERS_OWN}` must NOT be a thread in this estate, or the one id blessed \
     as the reader's own would itself be a citation into our tracker"
  );
}

/// **NO FILE INTENT INSTALLS INTO ANOTHER REPOSITORY CITES INTENT'S OWN
/// TRACKER.** This is `AC-00.9`'s third surface and it is the one hv extended
/// the row to reach.
///
/// **THIS ROW IS RED AND THE REDNESS IS THE POINT.** The payload carries live
/// citations today; the criterion is unsatisfied and this test says so in the
/// one place a reader will look, instead of the measurement living in the
/// criterion's prose where it decays unwatched. It was 80 references across 25
/// files when hv extended the row on 2026-08-20 and it is larger now, which is
/// what an unwatched surface does.
///
/// **THE POPULATION WAS PARKED WITH A NAMED EXPIRY AND THE EXPIRY WAS
/// HONOURED, WHICH IS THE ONLY ACCEPTABLE END FOR A PARK.** Measured
/// 2026-08-30 by vc when this was written: **93 citations across 27 of 228
/// installed payload files.** Cleared the same day -- vc took 26 files, dc took
/// the fiat-close rule -- and the `#[ignore]` came off rather than the
/// assertion being relaxed. Relaxing a gate at the moment it stops covering
/// anything converts a refusal into a silent pass, and a ratchet pinned at 93
/// would have pinned the defect.
///
/// **WHAT IS PARKED IS THE CORPUS, NOT THE CHECK.** The two controls above run
/// and gate now: the corpus positive control, and the both-directions
/// discrimination. `#[ignore]` rather than a relaxed assertion is deliberate --
/// relaxing a gate at the moment it stops covering anything converts a refusal
/// into a silent pass, and a ratchet pinned at 93 would pin the defect.
/// `#[ignore]` says NOT RUN, in every test run, where a reader sees it.
///
/// **TWO OF THE CITATIONS WERE LOAD-BEARING AND ONLY THE REGISTER KNEW.**
/// `AT-10.13` and `AT-01.5` are cited BY rows whose files must name them, so
/// stripping them blocked two close-gates. Neither this instrument nor the
/// criterion could have predicted that; the gate refused, which is why the
/// exemption above is computed from every thread's canon rather than listed.
#[test]
fn no_installed_payload_file_cites_intents_own_tracker() {
  let real = real_thread_ids();
  let payload = installed_payload();
  let root = repo_root();

  let mut offenders: Vec<(String, Vec<String>)> = Vec::new();
  let mut total = 0usize;
  for (path, text) in &payload {
    let rel_for_scan = path
      .strip_prefix(&root)
      .unwrap_or(path)
      .display()
      .to_string();
    let cites = citations_in_at(&rel_for_scan, text, &real);
    if !cites.is_empty() {
      total += cites.len();
      let rel = path
        .strip_prefix(&root)
        .unwrap_or(path)
        .display()
        .to_string();
      offenders.push((rel, cites));
    }
  }
  offenders.sort();

  let report: String = offenders
    .iter()
    .map(|(p, c)| format!("\n  {p} -- {}", c.join(", ")))
    .collect();

  assert!(
    offenders.is_empty(),
    "AT-00.17: {} citation(s) into Intent's own tracker across {} of {} \
     installed payload file(s). A consumer reading these arrives at a tracker \
     they cannot open.{report}",
    total,
    offenders.len(),
    payload.len()
  );
}

// ---------------------------------------------------------------------------
// No absolute home path in a file that functions as config
// ---------------------------------------------------------------------------
//
// **CARRIED FROM `tests/unit/no_absolute_home_paths.bats` (issue 0016), WHOSE
// VEHICLE THE v2 CUT REMOVES AND WHOSE PROPERTY v3 NEEDS MORE.** The original
// defect: hook resolution is a RUNTIME question that was answered at WRITE
// time, so a resolved `INTENT_HOME` froze into a tracked `.claude/settings.json`
// -- the hooks worked on exactly one machine, and a public repository published
// one person's home directory path. **The harm is PUBLISHING**, which is why
// every arm below is scoped to tracked or shipped bytes.
//
// **THE PROPERTY LANDS HERE RATHER THAN ANYWHERE ELSE** because this file is
// the v3 family for *what must not appear in generated output*, and vc measured
// v3's coverage of this particular property at ZERO before ruling the
// migration. Deleting the bats file without carrying this would have been
// prune-as-loss.
//
// **SCOPE, MEASURED RATHER THAN ASSUMED, AND THE WIDER READING IS THE TRAP.**
// The tempting population is "every generated artefact", and it is wrong: 14
// tracked generated files carry a home path today -- 8 canon thread extracts, 6
// issue extracts and `intent/st/ST0056/acceptance.md` -- and every one is
// AUTHORED PROSE quoting a path as historical record, carried verbatim into
// canon and projected into a view. A guard reporting those would fire on
// content that is correct as it stands, which is worse than no guard. So the
// population is what the original guard actually had: **files that FUNCTION as
// config** -- what Intent ships to consumers, and this project's own live
// `.claude/` stack.
//
// **THE LaunchAgent PLIST IS DELIBERATELY OUT OF SCOPE AND IT LOOKS LIKE IT
// SHOULD BE IN.** It is the newest generated artefact and it is *made of*
// absolute paths -- a plist must name the binary's absolute path or launchd
// cannot start it. It is also written per-machine into the operator's own
// `~/Library/LaunchAgents/` and is never tracked, so it cannot publish anything.
// Including it would red a correct generator.
//
// **THREE OF THE ORIGINAL SEVEN ARMS ARE NOT HERE, AND NEITHER ABSENCE IS AN
// OMISSION.** The hook-runner arms (stdin and exit code passed through, an
// unknown hook refused by name) are already v3-covered in `hook_compat.rs`,
// which asserts `error: unknown hook:` and drives `require-in-session` to both
// exit 2 and exit 0 -- measured before dropping them, not assumed. And the
// canon-engine substitution arm EXPIRED WITH ITS SUBJECT: it guarded an
// `INTENT_HOME]]` substitution inside `intent/plugins/claude/bin/
// intent_claude_upgrade`, which the cut deleted. A guard whose subject is gone
// is the vacuous-pass shape this thread has been paying for all day, so it is
// retired by name here rather than left to pass over nothing.

/// An absolute path into a user home directory, on either platform layout.
///
/// A plain scan rather than a regex, and the lower-case test is what makes it
/// one: `/Users/` alone matches the macOS root itself, which is not a leak.
fn home_paths_in(text: &str) -> Vec<String> {
  let mut out = Vec::new();
  for prefix in ["/Users/", "/home/"] {
    let mut from = 0;
    while let Some(at) = text[from..].find(prefix) {
      let start = from + at;
      let rest = &text[start + prefix.len()..];
      if rest.chars().next().is_some_and(|c| c.is_ascii_lowercase()) {
        let end = rest
          .find(|c: char| c.is_whitespace() || c == '"' || c == '\'')
          .map_or(rest.len(), |i| i);
        out.push(format!("{prefix}{}", &rest[..end]));
      }
      from = start + prefix.len();
    }
  }
  out
}

/// The template Intent lays down in somebody else's repository.
#[test]
fn the_shipped_claude_template_carries_no_absolute_home_path() {
  let dir = repo_root().join("lib/templates/.claude");
  let mut payload = Vec::new();
  collect_text_files(&dir, &mut payload);
  assert!(
    !payload.is_empty(),
    "no shipped .claude/ template was read at {} -- the population is empty and \
     a green here would mean nothing",
    dir.display()
  );

  let offenders: Vec<String> = payload
    .iter()
    .flat_map(|(p, text)| {
      home_paths_in(text)
        .into_iter()
        .map(move |hit| format!("{}: {hit}", p.display()))
    })
    .collect();
  assert!(
    offenders.is_empty(),
    "the shipped .claude/ template carries an absolute home path, so every \
     consumer scaffolded from it inherits one machine's layout:\n  {}",
    offenders.join("\n  ")
  );
}

/// This project's own tracked `.claude/` stack -- the live instance issue 0016
/// actually reported.
///
/// **TRACKED, not present.** `.claude/settings.local.json` is the per-machine
/// permission allowlist, gitignored by design, and absolute paths in it are
/// correct rather than a leak. Publishing is the harm, so `git ls-files` is the
/// population and the working directory is not.
#[test]
fn this_projects_tracked_claude_stack_carries_no_absolute_home_path() {
  let root = repo_root();
  let listed = Command::new("git")
    .args(["ls-files", ".claude/"])
    .current_dir(&root)
    .output()
    .expect("git ls-files runs in the repository");
  let files: Vec<&str> = std::str::from_utf8(&listed.stdout)
    .expect("git prints utf-8 paths here")
    .lines()
    .filter(|l| !l.is_empty())
    .collect();
  assert!(
    !files.is_empty(),
    "git listed no tracked .claude/ files -- the population is empty and a \
     green here would mean nothing"
  );

  let mut offenders = Vec::new();
  for rel in &files {
    let Ok(text) = std::fs::read_to_string(root.join(rel)) else {
      continue;
    };
    for hit in home_paths_in(&text) {
      offenders.push(format!("{rel}: {hit}"));
    }
  }
  assert!(
    offenders.is_empty(),
    "a tracked .claude/ file carries an absolute home path, and this repository \
     is public:\n  {}",
    offenders.join("\n  ")
  );
}

/// **THE MECHANISM, NOT THE SYMPTOM.** The two arms above would both pass on a
/// settings.json that carried a `[[INTENT_HOME]]` placeholder waiting to be
/// substituted at write time -- which is precisely the design that produced
/// issue 0016. Nothing in this file is per-machine, so there is nothing to
/// substitute; the template and the live copy are byte-identical, and that is
/// what makes the property hold by construction rather than by vigilance.
#[test]
fn settings_json_needs_no_substitution_and_matches_its_template() {
  let root = repo_root();
  let template = root.join("lib/templates/.claude/settings.json");
  let live = root.join(".claude/settings.json");

  let template_text = std::fs::read_to_string(&template).expect("the shipped settings template");
  assert!(
    !template_text.contains("[["),
    "the settings template carries a substitution placeholder, which is the \
     write-time resolution issue 0016 was about: {}",
    template.display()
  );

  let live_text = std::fs::read_to_string(&live).expect("this project's live settings.json");
  assert_eq!(
    live_text, template_text,
    "this project's .claude/settings.json has drifted from the template it is \
     supposed to be a byte-for-byte copy of, so one of them is per-machine"
  );
}

/// **The scan is driven to a POSITIVE before any of its zeroes are believed.**
///
/// Three arms above assert emptiness, and an emptiness assertion is worth
/// exactly what its instrument is worth -- the failure this estate keeps
/// paying for is a check that cannot fail returning the reassuring answer. So
/// the detector is shown finding what it is for, and shown NOT firing on the
/// two neighbours that would make it useless: the bare macOS `/Users` root,
/// which is not a leak, and a `$HOME`-relative path, which is the fix rather
/// than the defect.
#[test]
fn the_home_path_scan_fires_on_a_leak_and_not_on_its_neighbours() {
  let planted = r#"{"command": "/Users/someone/.intent/bin/intent"}"#;
  let hits = home_paths_in(planted);
  assert_eq!(
    hits,
    vec!["/Users/someone/.intent/bin/intent".to_string()],
    "the scan did not find a planted macOS home path, so every green above is \
     a statement about the instrument"
  );

  let linux = "exec /home/runner/work/intent/bin/intent";
  assert_eq!(
    home_paths_in(linux),
    vec!["/home/runner/work/intent/bin/intent".to_string()],
    "the scan missed the Linux layout, which is the platform CI runs on"
  );

  for innocent in [
    "ls /Users",
    "cd /Users/",
    "$HOME/.claude/settings.json",
    "~/.intent/ext",
    "${INTENT_HOME}/lib/templates",
  ] {
    assert!(
      home_paths_in(innocent).is_empty(),
      "the scan fired on `{innocent}`, which is portable and correct -- a guard \
       that reports the fix as the defect gets turned off"
    );
  }
}
