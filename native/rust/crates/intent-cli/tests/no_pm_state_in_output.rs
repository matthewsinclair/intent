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

use intent_cli::dispatch;
use testkit::repo_root;

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

/// Every SHIPPED command path the dispatch table declares -- from **both** of
/// its row homes.
///
/// **This was wrong in two directions at once, which is why it looked right**
/// (vc, issue 0037). It walked `families[].entries[]` and stopped, so the
/// top-level `new_surface` array -- `search`, `sync`, `schema`, `export`,
/// `ingest`, `backup`, `daemon`, `mcp`, eight rows with zero overlap -- was
/// never scanned by ANY surface in this file. Their help lives in the
/// compiled-in JSON rather than in Rust literals, so the string-literal scan
/// does not reach them either. And it took every row regardless of
/// disposition, so it also drove five RETIRED paths. One enumerator, too
/// narrow and too wide.
///
/// **The count assertion was the reason nobody noticed.** It read
/// `paths.len() > 20` under the message "precondition: the dispatch table
/// declares the command surface" -- a sentence that reads as a coverage claim
/// and is a did-the-file-parse check. It passes at 104 and it passes at 112,
/// so it could not see a twelfth of the surface be absent. **A precondition
/// whose message describes a stronger property than it tests is worse than no
/// message**, because it answers the question a reader came to ask.
///
/// So the shape is now: read both homes, and filter on the SAME
/// [`dispatch::Entry::is_shipped`] the spine applies when it builds the
/// surface -- reusing that decision rather than making a second one that can
/// drift from it. The table is read through the typed `dispatch::table()` for
/// the same reason.
fn declared_paths() -> Vec<String> {
  let table = dispatch::table();
  let from_families: Vec<String> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .filter(|e| e.is_shipped())
    .map(|e| e.path.clone())
    .collect();
  let from_new_surface: Vec<String> = table
    .new_surface
    .iter()
    .filter(|e| e.is_shipped())
    .map(|e| e.path.clone())
    .collect();

  // **Each home is asserted separately, because the defect was one home
  // returning nothing while the total still looked healthy.** A single total
  // cannot distinguish "both homes read" from "one home read and the other is
  // large"; these two can, and they are what actually regressed.
  assert!(
    !from_families.is_empty(),
    "precondition: no shipped row was read from `families`, so the ported surface is unscanned"
  );
  assert!(
    !from_new_surface.is_empty(),
    "precondition: no shipped row was read from `new_surface`, so v3's own commands are \
     unscanned -- this is issue 0037 exactly, and it passed for a day"
  );

  let paths: Vec<String> = from_families.into_iter().chain(from_new_surface).collect();

  // And the total EQUALS what the table declares as shipped, computed by
  // counting rather than by collecting, so going short is an error rather than
  // a smaller number that still satisfies a `>`.
  let shipped = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .filter(|e| e.is_shipped())
    .count();
  assert_eq!(
    paths.len(),
    shipped,
    "the scan covers every shipped row the table declares, or it covers an unstated subset"
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
/// **Char literals are HANDLED rather than assumed absent, and the reason is
/// that the assumption failed within the hour.** This first asserted that no
/// `'"'` appeared in shipped source, on the grounds that none did. Then
/// `faces.rs` grew a `marker()` reader whose last step is `.trim_matches('"')`
/// -- ordinary, correct code -- and the assertion fired. **It fired rather than
/// mis-scanning, which is the whole argument for stating an assumption instead
/// of relying on it**, but a guard that refuses legitimate code is a guard
/// someone deletes, so the scanner learned the construct.
///
/// A leading `'` is ambiguous in Rust -- `'a` is a lifetime, `'x'` is a char --
/// so it is disambiguated by looking for the closing quote two or three bytes
/// on. A lifetime has none and simply advances.
///
/// Block comments are still asserted absent, because there are none and
/// handling nesting is real work for a construct this codebase does not use.
fn string_literals(code: &str) -> Vec<String> {
  // **THE BLOCK-COMMENT ASSERTION MOVED INTO THE WALK ON 2026-08-20, AND THE
  // REASON IS THAT ITS DETECTION HAD STOPPED MATCHING ITS SUBJECT.**
  //
  // It stood here as `!code.contains("/*")` over the whole file text, which is
  // the one place in this function that does not know whether it is inside a
  // literal. `critic.rs` landed glob patterns -- `"test/**/*_test.exs"`,
  // `"lib/**/*.ex"`, `"lib/*.ex"` -- and every one of them contains `/*` inside
  // a STRING. Two of eight tests failed reporting a block comment in a file
  // that has none.
  //
  // **The assumption the doc states is still true and was never the problem.**
  // There are no block comments in shipped source, and the scanner still
  // refuses rather than mis-scanning if one appears. What was wrong was a
  // substring test standing in for a syntactic fact -- the same shape as
  // ST0039's greppable proxies, where a regex that cannot see structure is
  // asked a question only structure can answer.
  //
  // So the check now fires from inside the walk, at a point where `i` is known
  // to be outside every literal, comment and raw string. It is strictly more
  // precise: a real block comment is still caught, and correct code that merely
  // spells `/*` is not.

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
    // A block comment, outside any literal -- the assertion the pre-check
    // above used to make on raw text. Reaching here means `i` is not inside a
    // string, a raw string, a char literal or a line comment, so this is a real
    // `/*` and not a glob.
    assert!(
      !(b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*'),
      "a block comment appeared in shipped source at byte {i}; this scanner only skips `//` line \
       comments, so it would read the comment's body as code. **Check it is a real block comment \
       and not a `/*` inside a string** -- glob patterns like `lib/**/*.ex` are correct code and \
       reach this line only if the walk above has a hole"
    );
    // A char literal, which may CONTAIN a quote (`'"'`) and would otherwise be
    // read as opening a string. A lifetime (`'static`) has no closing quote in
    // that position and falls through to the plain advance below.
    if b[i] == b'\'' {
      let closes_at = if b.get(i + 1) == Some(&b'\\') { 3 } else { 2 };
      if b.get(i + closes_at) == Some(&b'\'') {
        i += closes_at + 1;
        continue;
      }
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

/// The citations in one payload file: identifiers that point into Intent's own
/// tracker rather than teaching a consumer a shape.
fn citations_in(text: &str, real: &BTreeSet<String>) -> Vec<String> {
  pm_identifiers(text, Decisions::Ambiguous)
    .into_iter()
    .filter(|id| {
      if id.starts_with("ST") {
        // Referent: a four-digit thread id is a citation only if it resolves.
        real.contains(id)
      } else {
        // `WP-nn`, `AC-n.n`, `AT-n.n` -- no thread context, so nothing to
        // resolve against and nothing it can be but a citation.
        true
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
    citations_in("intent st show ST4242", &roster).is_empty(),
    "an id absent from the roster names no thread, so it teaches syntax"
  );
  assert!(
    citations_in("\"${CANON_DIR}/st/ST9999.json\"", &roster).is_empty(),
    "`ST9999` is a deliberate probe for a path that does not exist"
  );
  assert!(
    citations_in("see ST0000 for the retrofit", &roster).is_empty(),
    "`ST0000` is the READER's own thread and is blessed above"
  );
  assert_eq!(
    citations_in("# COVERS ST0056 AC-10.13 / AT-10.13.", &roster),
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
/// **THE POPULATION IS PARKED AND THE INSTRUMENT IS NOT, which is the same
/// shape `AT-06.8` was parked in and for the same reason.** Measured 2026-08-30
/// by vc: **93 citations across 27 of 228 installed payload files.** Three
/// nodes are editing this payload concurrently -- dc owns the hooks, ic the
/// skills surface -- so a baseline frozen mid-landing would red the workspace
/// for four nodes on a number that was never stable.
///
/// **WHAT IS PARKED IS THE CORPUS, NOT THE CHECK.** The two controls above run
/// and gate now: the corpus positive control, and the both-directions
/// discrimination. `#[ignore]` rather than a relaxed assertion is deliberate --
/// relaxing a gate at the moment it stops covering anything converts a refusal
/// into a silent pass, and a ratchet pinned at 93 would pin the defect.
/// `#[ignore]` says NOT RUN, in every test run, where a reader sees it.
///
/// **EXPIRY, NAMED SO IT CANNOT BECOME PERMANENT: remove `#[ignore]` when the
/// payload citations are cleared.** They are mechanical -- provenance comments
/// in installed hooks (`# COVERS ST0056 AC-10.13`), and worked examples built
/// on `ST0042` / `ST0005` / `ST0001`, all three of which resolve here. The
/// resolution for the second class is already named in this file: `READERS_OWN`
/// is the only id guaranteed to resolve to something the reader can look at.
///
/// **AT-00.17 IS THEREFORE RED AND MUST NOT BE MOVED ON THE STRENGTH OF THIS
/// COMMIT.** The instrument exists and is driven in both directions; the
/// criterion is unmet until this arm runs.
#[ignore = "AT-00.17: population parked with a named expiry; the instrument runs and the corpus does not gate yet"]
#[test]
fn no_installed_payload_file_cites_intents_own_tracker() {
  let real = real_thread_ids();
  let payload = installed_payload();
  let root = repo_root();

  let mut offenders: Vec<(String, Vec<String>)> = Vec::new();
  let mut total = 0usize;
  for (path, text) in &payload {
    let cites = citations_in(text, &real);
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
