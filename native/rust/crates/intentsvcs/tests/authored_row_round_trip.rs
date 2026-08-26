//! **Authored AT rows survive a round trip through the model, byte for byte.**
//!
//! The strongest form of the 0056 question, and it is a different question from
//! the one 0056 asked. That issue was about a single token: `Na` serialised as
//! `n-a` where every author writes `n/a`. The guard written for it compares the
//! renderer to `display()` -- the function defining the renderer's spelling --
//! so it is self-consistency, and a `display()` returning `n@a` satisfies it.
//!
//! **What closes it is a corpus neither side of this repository produced.** The
//! fixture is v2-authored rows captured by vc with `git show <sha>:<path>` and
//! diffed against the worktree, from three threads that all predate both 0056
//! and its fix -- so no row here can have been shaped by the defect or by the
//! repair. Provenance is argued in the fixture's own README.
//!
//! **THE ASSERTION IS ON THE WHOLE LINE, NOT ON THE STATUS TOKEN, AND THAT IS
//! WHAT MAKES THIS WORTH MORE THAN A VOCABULARY CHECK** (vc's specification). A
//! status comparison passes on a row whose covers list was reordered or whose
//! note was dropped -- and covers and note are the two fields the v2 tool
//! already destroys elsewhere (issue 0033). Requiring the rendered line to equal
//! the authored line catches a field nobody was looking at.
//!
//! It did exactly that before this file existed: reading the two ends against
//! the corpus found 12 of 14 notes dropped outright and the other 2 stripped of
//! their key, while `status` -- the only field 0056 concerned -- round-tripped
//! correctly.
//!
//! **`red` IS ABSENT AND THAT IS DECLARED RATHER THAN PAPERED OVER.** All four
//! `red` rows in the estate live in files edited the day 0056 was filed, so
//! there is no uncontaminated source for it. It is the weakest of the four to be
//! missing -- its wire tag and authored form coincide -- but a reader must not
//! infer from a green here that all four vocabularies were compared against
//! authored bytes. Three were; the fourth is covered only by the transcription
//! in `view_determinism.rs`.

mod common;

use common::{Fixture, ctx};
use intentsvcs::ingest::Canon;
use intentsvcs::{legacy, views};

/// The captured corpus.
///
/// **It REFUSES when the fixture is unreadable rather than skipping.** A parity
/// test that passes on a missing corpus is precisely the vacuous green the
/// fixture was captured to prevent, and it would pass most loudly on the machine
/// where somebody had deleted it.
fn authored_rows() -> Vec<String> {
  let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    // Four levels: `crates/intentsvcs` -> `crates` -> `rust` -> `native` -> the
    // repository root. Three was the first guess and the refusal above is what
    // said so, in one run, instead of a skip that would have passed.
    .join("../../../../intent/st/ST0056/parity/fixtures/rows.txt");
  let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
    panic!(
      "the captured corpus at {} is unreadable ({e}). This test refuses rather than skips: its whole authority is that the bytes came from v2, so with the \
       corpus absent there is nothing here to measure",
      path.display()
    )
  });
  let rows: Vec<String> = text
    .lines()
    .filter(|l| l.starts_with("- AT-"))
    .map(str::to_string)
    .collect();
  assert!(
    rows.len() >= 14,
    "the corpus carries {} AT row(s) and at least 14 were captured -- a shrunken fixture is the cheapest way to a false green here, which is why the floor is \
     asserted rather than the exact count",
    rows.len()
  );
  rows
}

/// The AC ids a row claims to cover, read from the row.
///
/// A SECOND reader of the same substring, deliberately: the synthesised contract
/// has to satisfy the real parser's reference check, so if this disagrees with
/// `legacy`'s own reader the scan reports a broken reference and the test says so
/// rather than quietly ingesting a row whose coverage went missing.
fn covered(row: &str) -> Vec<String> {
  const MARKER: &str = " -- covers ";
  let start = match row.find(MARKER) {
    Some(at) => at + MARKER.len(),
    None => return Vec::new(),
  };
  let rest = &row[start..];
  let end = rest.find(" -- ").unwrap_or(rest.len());
  rest[..end]
    .split(',')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect()
}

/// A v2 estate carrying ONE authored AT row and the contract it covers.
///
/// One row per thread because the corpus draws on three source threads with
/// overlapping AT ids -- `AT-02.1` appears three times -- so a single combined
/// contract could not hold them. One row per thread also makes the failure
/// message name the row rather than a thread containing it.
fn estate_for(fixture: &Fixture, row: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    "---\nverblock: \"17 Aug 2026:v0.1: cc - x\"\nintent_version: 2.19.0\nstatus: Not Started\nslug: a-slug\ncreated: 20260813\ncompleted:\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n",
  );
  // The criterion kind mirrors the AT's, so the synthesised contract is one that
  // could actually have been authored rather than merely one that parses.
  let marker = if row.contains("(non-test)") {
    "(non-test) "
  } else {
    ""
  };
  let contract: String = covered(row)
    .iter()
    .map(|id| format!("- {id} {marker}a criterion\n"))
    .collect();
  fixture.write_file(
    "intent/st/ST0001/acceptance.md",
    &format!("# Acceptance\n\n## Criteria\n\n{contract}\n## Tests\n\n{row}\n"),
  );
}

/// Render the thread's `acceptance.md` and return its AT rows.
fn rendered_rows(fixture: &Fixture) -> Vec<String> {
  let scan = legacy::scan(&fixture.project()).expect("scan the v2 estate");
  assert!(
    scan.residue.is_empty() && scan.carried.is_empty(),
    "the synthesised estate is not clean, so anything measured below is about the fixture rather than the round trip: residue={:?} carried={:?}",
    scan.residue,
    scan.carried
  );
  let canon = Canon {
    threads: scan.threads,
    issues: Vec::new(),
    sections: Vec::new(),
  };
  views::render_all(&fixture.project(), &canon, &ctx())
    .into_iter()
    .find(|v| v.path.to_string_lossy().ends_with("acceptance.md"))
    .expect("the acceptance view is rendered")
    .content
    .lines()
    .filter(|l| l.starts_with("- AT-"))
    .map(str::to_string)
    .collect()
}

/// Rows whose re-render is KNOWN to diverge, BY RULING, each with its reason
/// and the condition that retires it.
///
/// **A GATE KEPT GREEN BY LEAVING OUT THE ROWS IT WOULD FAIL ON IS NOT A GATE.**
/// The rows below are real, committed, and they do not round-trip; the honest
/// record is to carry them IN the corpus and declare the divergence, which is
/// the estate's existing pattern -- *naming a permanent exclusion WITH ITS
/// REASON is a stronger record than an absent row* (`mutation_every_writable_field`).
///
/// **THE MATCH IS ON A SUBSTRING OF THE AUTHORED ROW**, not on the rendered
/// one: what is ruled is *this row diverges*, and keying on the render would
/// make the exception move whenever the renderer does.
const DIVERGES_BY_RULING: &[(&str, &str)] = &[
  (
    "test/cdsync.bats (whole suite, 328 tests)",
    "vc 2026-08-26: the citation split moves the trailing annotation into the note, so the row renders with the annotation after `status:` rather than inside the backticks. RETIRES when the annotation-beside-`file` model field lands (Conflab's schema class) and the renderer can put it back. Also carries the separate pre-existing break below.",
  ),
  (
    "test/riffle/cli/sia_pipelines_command_test.exs (6)",
    "vc 2026-08-26: as above -- annotation relocated to the note by the citation split. RETIRES with the annotation field.",
  ),
  (
    "test/riffle/cli/sia_run_command_test.exs (registration + the Ctx-returning contract)",
    "vc 2026-08-26: as above -- annotation relocated to the note by the citation split. RETIRES with the annotation field.",
  ),
  (
    "`native/ios/ProlixTests` (whole target, via `bin/prolix test swift`)`",
    "vc 2026-08-26: as above. **This row round-tripped BEFORE the citation split and does not after** -- the one regression the split costs, ruled acceptable because canon's first job is to be true and the old render was faithful to a path no filesystem has. RETIRES with the annotation field.",
  ),
];

/// **THE THREE UNBACKTICKED ROWS ABOVE CARRY A SECOND, OLDER DEFECT that is not
/// the citation split's doing and is filed separately:** their authors wrote no
/// backticks, and `test_line` emits `` `{file}` `` unconditionally, so the view
/// adds punctuation the author never wrote. Measured as 3 of 20 failing on a
/// baseline taken BEFORE the split existed. Fixing that is a renderer change,
/// not a parser change.
fn ruled_divergence(row: &str) -> Option<&'static str> {
  DIVERGES_BY_RULING
    .iter()
    .find(|(marker, _)| row.contains(marker))
    .map(|(_, reason)| *reason)
}

/// **The whole line, byte for byte, for every row v2 wrote.**
#[test]
fn every_authored_at_row_renders_back_to_the_bytes_it_was_written_as() {
  let mut compared = 0;
  let mut broken: Vec<(String, String)> = Vec::new();
  let mut ruled: Vec<(String, &'static str)> = Vec::new();

  for row in authored_rows() {
    let fixture = Fixture::new();
    estate_for(&fixture, &row);
    let out = rendered_rows(&fixture);
    // An unparseable row renders NO line, and "no line" must not be reported as
    // a spelling difference -- they are different defects with different fixes.
    assert_eq!(
      out.len(),
      1,
      "the authored row rendered {} lines rather than one, so it did not survive ingest at all and there is nothing to compare:\n  authored: {row}\n  rendered: {out:?}",
      out.len()
    );
    if out[0] != row {
      match ruled_divergence(&row) {
        Some(reason) => ruled.push((row.clone(), reason)),
        None => broken.push((row.clone(), out[0].clone())),
      }
    }
    compared += 1;
  }

  assert!(
    broken.is_empty(),
    "{} of {compared} authored rows do not render back as themselves. The fixture is v2's bytes and is not the thing to change:\n{}",
    broken.len(),
    broken
      .iter()
      .map(|(a, b)| format!("  authored: {a}\n  rendered: {b}\n"))
      .collect::<Vec<_>>()
      .join("")
  );
  assert!(compared >= 14, "only {compared} rows were compared");

  // **A DECLARED EXCEPTION THAT NO LONGER FIRES IS A LIE THAT AGES WELL.**
  // Every ruling in `DIVERGES_BY_RULING` must be exercised by a row that is
  // actually in the corpus and actually diverges. So when the annotation field
  // lands and these rows begin round-tripping, THIS arm goes red and the
  // exception is retired deliberately -- rather than sitting there forever
  // excusing a divergence that stopped happening. The exception list is the
  // only part of this test that can rot, and this is what stops it.
  assert_eq!(
    ruled.len(),
    DIVERGES_BY_RULING.len(),
    "{} ruled divergence(s) declared but {} fired. A declared exception that no longer applies must be DELETED, not left standing:\n{}",
    DIVERGES_BY_RULING.len(),
    ruled.len(),
    DIVERGES_BY_RULING
      .iter()
      .filter(|(m, _)| !ruled.iter().any(|(row, _)| row.contains(m)))
      .map(|(m, _)| format!("  did not fire: {m}\n"))
      .collect::<Vec<_>>()
      .join("")
  );
}

/// **The note survives with whatever it contains, including a spaced `--` and a
/// `test:` prefix.**
///
/// Held apart from the byte-equality above because it is the property that
/// failure would be easiest to mistake for a formatting nicety. v2 declines to
/// parse the note at all -- `AT_G_NOTE='( -- .*)?'`, greedy to end of line -- so
/// the tail has no interior structure and a reader that finds structure in it is
/// inventing a distinction the canon never made (vc's ruling).
///
/// Nine of the corpus's rows carry a note that is introduced by ` -- ` and then
/// CONTAINS ` -- `. Anything splitting on the separator over-splits exactly the
/// rows carrying the most information, and does it silently, because both halves
/// still look like a note.
#[test]
fn a_note_containing_the_row_separator_survives_whole() {
  let nested: Vec<String> = authored_rows()
    .into_iter()
    .filter(|row| {
      let tail = row.split(" -- status: ").nth(1).unwrap_or("");
      tail.contains(" -- ")
    })
    .collect();
  assert!(
    !nested.is_empty(),
    "no row in the corpus carries a note containing ` -- `, so this test asserts a property of an empty set -- the fixture has lost the rows that made it \
     worth writing"
  );

  for row in &nested {
    let fixture = Fixture::new();
    estate_for(&fixture, row);
    let out = rendered_rows(&fixture);
    assert_eq!(
      out.len(),
      1,
      "a row whose note contains the separator did not survive ingest:\n  {row}"
    );
    let authored_note = row.split(" -- status: ").nth(1).unwrap_or("");
    let rendered_note = out[0].split(" -- status: ").nth(1).unwrap_or("");
    assert_eq!(
      rendered_note, authored_note,
      "the note was reshaped. Everything after the status value is one opaque tail in v2 and must be carried verbatim:\n  authored: {row}\n  rendered: {}",
      out[0]
    );
  }
}
