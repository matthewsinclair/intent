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

/// The REPOSITORY root -- where `schema/` and `surface/` live.
///
/// Searched, never counted: a depth is a claim about a layout and goes stale in
/// silence, which is how `ancestors().nth(2)` broke everywhere at once when
/// `native/rust/` appeared.
fn repo_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .find(|d| d.join("schema").is_dir() && d.join("surface").is_dir())
    .expect("a repository root carrying schema/ and surface/ above this crate")
    .to_path_buf()
}

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
    .output()
    .expect("run the v3 binary");
  let mut both = String::from_utf8_lossy(&out.stdout).into_owned();
  both.push_str(&String::from_utf8_lossy(&out.stderr));
  both
}

/// Every command path the dispatch table declares, families and verbs alike.
fn declared_paths() -> Vec<String> {
  let table = std::fs::read_to_string(repo_root().join("surface/dispatch-table.json"))
    .expect("the dispatch table is committed");
  let table: serde_json::Value = serde_json::from_str(&table).expect("the dispatch table parses");
  let mut paths = Vec::new();
  for family in table["families"].as_array().into_iter().flatten() {
    for entry in family["entries"].as_array().into_iter().flatten() {
      if let Some(p) = entry["path"].as_str() {
        paths.push(p.to_string());
      }
    }
  }
  assert!(
    paths.len() > 20,
    "precondition: the dispatch table declares the command surface, got {} paths",
    paths.len()
  );
  paths
}

// ---------------------------------------------------------------------------
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

/// Shipped source: the three crates that become binaries or are linked into
/// one.
fn shipped_sources() -> Vec<PathBuf> {
  let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("crates/ above this crate")
    .to_path_buf();
  let mut files = Vec::new();
  for name in ["intent-cli", "intentsvcs", "intentd"] {
    collect_rs(&crates.join(name).join("src"), &mut files);
  }
  assert!(
    files.len() > 10,
    "precondition: the shipped crates have source, found {}",
    files.len()
  );
  files.sort();
  files
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for e in entries.filter_map(Result::ok) {
    let p = e.path();
    if p.is_dir() {
      collect_rs(&p, out);
    } else if p.extension().is_some_and(|x| x == "rs") {
      out.push(p);
    }
  }
}

/// Every string-literal CONTENT in one Rust file, comments and the trailing
/// test module excluded.
///
/// **A line-based "is there a quote on this line" test does not work here and
/// the reason is load-bearing.** `store.rs`'s DDL is one string literal
/// spanning two hundred lines, and its interior `--` comment lines carry no
/// quote at all -- they are literal content that `intent schema ddl.sql` prints
/// verbatim. A line-based scan sees no quote and skips exactly the lines that
/// are published. So this tracks literal spans.
///
/// Two constructs it does not handle are ASSERTED ABSENT rather than assumed
/// absent, the same posture `store_schema_version.rs` takes to in-line SQL
/// comments: a future block comment or `'"'` char literal fails here loudly
/// instead of silently changing what this scan means.
fn string_literals(code: &str) -> Vec<String> {
  assert!(
    !code.contains("/*"),
    "a block comment appeared in shipped source; this scanner only skips `//` line comments, so \
     it would read the comment's body as code"
  );
  assert!(
    !code.contains("'\"'"),
    "a `'\"'` char literal appeared in shipped source; this scanner would read it as opening a \
     string and mis-attribute everything after it"
  );

  // The trailing `#[cfg(test)] mod tests` is Intent's own test fixtures, which
  // AC-00.9 exempts: they are never compiled into a shipped binary, so nothing
  // in them can be emitted. Every shipped file has at most one, at the end --
  // asserted, because truncating at the first would silently drop real code if
  // that ever stopped being true.
  assert!(
    code.matches("#[cfg(test)]").count() <= 1,
    "a shipped file grew a second `#[cfg(test)]`, so truncating at the first would drop shipped \
     code from this scan"
  );
  let code = match code.find("#[cfg(test)]") {
    Some(at) => &code[..at],
    None => code,
  };

  let b = code.as_bytes();
  let mut out = Vec::new();
  let mut i = 0;
  while i < b.len() {
    // A line comment, outside any literal.
    if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
      while i < b.len() && b[i] != b'\n' {
        i += 1;
      }
      continue;
    }
    // A raw string: `r"`, `r#"`, `r##"` ... which has no escapes, so it ends
    // only at a quote followed by the same number of hashes.
    if b[i] == b'r' && (i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_')) {
      let mut h = i + 1;
      while h < b.len() && b[h] == b'#' {
        h += 1;
      }
      if h < b.len() && b[h] == b'"' {
        let hashes = h - (i + 1);
        let start = h + 1;
        let mut j = start;
        while j < b.len() {
          if b[j] == b'"'
            && b[j + 1..]
              .iter()
              .take(hashes)
              .filter(|c| **c == b'#')
              .count()
              == hashes
          {
            break;
          }
          j += 1;
        }
        out.push(code[start..j.min(code.len())].to_string());
        i = (j + 1 + hashes).min(b.len());
        continue;
      }
    }
    if b[i] == b'"' {
      let start = i + 1;
      let mut j = start;
      while j < b.len() {
        match b[j] {
          b'\\' => j += 2,
          b'"' => break,
          _ => j += 1,
        }
      }
      out.push(code[start..j.min(code.len())].to_string());
      i = (j + 1).min(b.len());
      continue;
    }
    i += 1;
  }
  out
}

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
";
  let literals = string_literals(code);
  let found: Vec<String> = literals
    .iter()
    .flat_map(|l| pm_identifiers(l, Decisions::Counted))
    .collect();
  assert_eq!(
    found,
    vec!["D42", "AC-02.8"],
    "the two identifiers inside literals are found and the two inside comments are not; got \
     literals: {literals:?}"
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
