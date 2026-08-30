//! AT-10.8 / AC-10.8: **deleting the file estate and egesting from the DB
//! reproduces it, and the only difference is the out-of-model set.**
//!
//! **THE TWO DIRECTIONS MUST BE INVERSES, WHICH IS D34's PRECONDITION.** The
//! extract is the interchange, so a field that egests wrong is data loss at the
//! clone boundary where nobody typed anything. That is why this is measured
//! rather than argued: an untested egest guarantee only fails on the day
//! someone has already lost the files.
//!
//! **WHY THE COMPARISON IS AGAINST THE PRIOR ESTATE AND NEVER AGAINST WHAT
//! EGEST EMITTED.** The criterion's own note says a test that only compares
//! what egest emitted proves nothing, and it is right: asking the egest which
//! files it should have written and then checking it wrote them is a
//! tautology that passes on every possible defect. **The estate is snapshotted
//! BEFORE it is deleted**, so the thing being reproduced was authored by
//! something other than the operation under test.
//!
//! # The denominator is READ, never hardcoded
//!
//! **A CHECK THAT SUPPLIES ITS OWN OUT-OF-MODEL SET CERTIFIES ITS OWN
//! DENOMINATOR**, which is the attack ic drove through `--out-of-model` on
//! 2026-08-18: the migrator zeroes a counter by naming everything, and every
//! loss becomes licensed by being listed. So the set is taken from
//! `data-model.md`'s `## What is deliberately not modelled`, and
//! [`the_denominator_is_declared_where_the_criterion_says_it_is`] fails if that
//! section stops existing or stops naming the pair this test plants.
//!
//! **THE SECTION IS PROSE AND CANNOT BE PARSED INTO GLOBS, SO THE MAPPING IS
//! PINNED TO ITS SOURCE PHRASE INSTEAD.** Two of the three entries are
//! categories rather than paths -- *prose*, and *rules/skills/templates*
//! (shipped content embedded in the binary) -- and neither is a file set a test
//! can enumerate in a fixture. The third IS named: `wip.md` / `restart.md`, the
//! project-level pair. **This test plants that pair and requires the document
//! to still name it**, so the mapping cannot outlive the sentence it was
//! derived from. That is the whole mechanism: not a parse, and not a hardcode.
//!
//! **AND MY FIRST SEARCH FOR THIS SECTION FOUND NOTHING, WHICH IS RECORDED
//! BECAUSE IT NEARLY BECAME A FINDING.** AC-10.8 cites *"the out-of-model set
//! enumerated at `data-model.md`"* and the string `out-of-model` appears ZERO
//! times in that file. The set is there under a different heading. **A citation
//! is not wrong because your grep term is** -- positive-controlling the search
//! against a term that had to be present is what separated the two.
//!
//! # What this does NOT cover, stated rather than discovered later
//!
//! **AC-10.8's second half is UNBUILT and this file does not pretend
//! otherwise.** The criterion requires the out-of-model set to be *"named in
//! the output rather than silently absent"*, and `sync --to-disk` prints
//! `ok: extract written for {n} thread(s)` and nothing else. The arm for it is
//! present and `#[ignore]`d with its expiry, on vc's standing ruling that
//! relaxing a gate at the moment it stops covering anything converts a refusal
//! into a silent pass -- **so it says NOT RUN, in every run, where a reader
//! sees it.** The row is RED until that arm runs.

mod common;

use std::collections::BTreeMap;

use common::{Fixture, changed, sample_issue, sample_thread, tree};
use intentsvcs::sync::Scope;

/// The document AC-10.8 names as carrying the enumeration.
const DATA_MODEL: &str = "intent/st/ST0056/data-model.md";

/// The heading the enumeration actually sits under.
const OUT_OF_MODEL_HEADING: &str = "## What is deliberately not modelled";

/// The out-of-model members this fixture can plant, because they are the only
/// ones named as PATHS rather than as categories.
const NAMED_PAIR: &[&str] = &["wip.md", "restart.md"];

/// The two directories the egest reads FROM and therefore may not be deleted:
/// the store it projects, and the config that says where anything lives.
const NOT_THE_FILE_ESTATE: &[&str] = &[".cache", ".config"];

fn data_model_text() -> String {
  let path = testkit::repo_root().join(DATA_MODEL);
  std::fs::read_to_string(&path)
    .unwrap_or_else(|e| panic!("AC-10.8 names {DATA_MODEL} as the enumeration's home: {e}"))
}

/// The enumeration's own section, from its heading to the next one.
fn out_of_model_section(text: &str) -> &str {
  let start = text
    .find(OUT_OF_MODEL_HEADING)
    .unwrap_or_else(|| panic!("{DATA_MODEL} no longer carries `{OUT_OF_MODEL_HEADING}`"));
  let rest = &text[start + OUT_OF_MODEL_HEADING.len()..];
  match rest.find("\n## ") {
    Some(end) => &rest[..end],
    None => rest,
  }
}

/// **THE DENOMINATOR IS DECLARED WHERE THE CRITERION SAYS, AND STILL NAMES WHAT
/// THIS TEST PLANTS.**
///
/// Without this the arms below plant two files on my say-so and call the result
/// an out-of-model set -- which is the same move as hardcoding it, one step
/// further from the reader. **If the section is renamed, dropped, or stops
/// naming the pair, this reds and the mapping must be re-derived** rather than
/// silently continuing to describe a document that has moved on.
#[test]
fn the_denominator_is_declared_where_the_criterion_says_it_is() {
  let text = data_model_text();
  let section = out_of_model_section(&text);

  for member in NAMED_PAIR {
    assert!(
      section.contains(member),
      "`{OUT_OF_MODEL_HEADING}` in {DATA_MODEL} no longer names `{member}`, so this test's \
       planted set has stopped being justified by the document AC-10.8 cites. Re-derive the \
       mapping from the section as it now reads -- do not adjust `NAMED_PAIR` to match the \
       test's expectations, which is the denominator certifying itself"
    );
  }

  // The section must still be about what the DB cannot reproduce, since that
  // is the only reason its contents are the expected difference here.
  assert!(
    text.contains("cannot reproduce from the DB alone"),
    "{DATA_MODEL} no longer states that this set is what an export cannot reproduce from the \
     DB, which is the sentence that makes it AC-10.8's expected difference rather than merely \
     a list of unmodelled things"
  );
}

/// A populated estate, egested once, with the out-of-model pair planted beside
/// it. Returns the snapshot taken BEFORE anything is deleted.
fn estate_with_the_out_of_model_pair(fx: &Fixture) -> BTreeMap<String, Vec<u8>> {
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&sample_thread("ST0002"));
  fx.write_issue(&sample_issue(7));
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("a populated estate egests");

  // **AUTHORED, NEVER GENERATED.** These are the project-level tracking pair,
  // written by a human and held in no table, which is exactly why the egest
  // cannot put them back.
  fx.write_file("intent/wip.md", "# WIP\n\nauthored, in no table\n");
  fx.write_file("intent/restart.md", "# Restart\n\nauthored, in no table\n");

  tree(fx.root())
}

/// Delete everything the egest is supposed to be able to rebuild, plus the
/// out-of-model files, leaving only what it reads FROM.
fn delete_the_file_estate(fx: &Fixture) {
  let intent = fx.path("intent");
  for entry in std::fs::read_dir(&intent)
    .expect("read the intent dir")
    .flatten()
  {
    let name = entry.file_name().to_string_lossy().into_owned();
    if NOT_THE_FILE_ESTATE.contains(&name.as_str()) {
      continue;
    }
    let path = entry.path();
    if path.is_dir() {
      std::fs::remove_dir_all(&path).expect("remove a directory of the estate");
    } else {
      std::fs::remove_file(&path).expect("remove a file of the estate");
    }
  }
}

/// **THE PROPERTY: everything modelled comes back, byte for byte.**
#[test]
fn deleting_the_file_estate_and_egesting_reproduces_every_modelled_file() {
  let fx = Fixture::new();
  let before = estate_with_the_out_of_model_pair(&fx);

  delete_the_file_estate(&fx);
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("the store still holds the estate, so it egests");
  let after = tree(fx.root());

  let missing: Vec<&String> = before
    .keys()
    .filter(|path| !after.contains_key(*path))
    .filter(|path| !is_out_of_model(path) && !is_the_store(path))
    .collect();

  assert!(
    missing.is_empty(),
    "the estate was deleted and egested back, and these MODELLED files did not return -- each \
     one is data loss at the clone boundary, which is what D34 prices this criterion in:\n  {}",
    missing
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<_>>()
      .join("\n  ")
  );

  // **THE PATH IS NAMED AND THE BYTES ARE NOT PRINTED.** The first cut used
  // `assert_eq!` on the byte vectors and dumped 757KB of sqlite into the
  // failure output on its first real red -- a diagnostic nobody can read,
  // about a file that was never in this criterion's scope.
  let differing: Vec<&String> = before
    .iter()
    .filter(|(path, _)| !is_out_of_model(path) && !is_the_store(path))
    .filter(|(path, bytes)| after.get(*path).is_some_and(|back| back != *bytes))
    .map(|(path, _)| path)
    .collect();

  assert!(
    differing.is_empty(),
    "these files came back with different bytes, so the two directions are not inverses -- a \
     plausible carrier of the DB rather than a faithful one:\n  {}",
    differing
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<_>>()
      .join("\n  ")
  );
}

/// **AND THE DIFFERENCE IS EXACTLY THE OUT-OF-MODEL SET, NOT MERELY WITHIN
/// IT.**
///
/// The arm above would pass on an egest that also wrote fifty files nobody
/// asked for, and on one that reproduced everything while leaving the
/// out-of-model pair mysteriously present. **Both halves are asserted: nothing
/// modelled is missing, and nothing outside the declared set differs.**
#[test]
fn the_only_difference_is_the_declared_out_of_model_set() {
  let fx = Fixture::new();
  let before = estate_with_the_out_of_model_pair(&fx);

  // **ANTI-VACUITY, AND IT COMES BEFORE THE PROPERTY.** If the pair was never
  // written, the difference is empty and this arm passes having measured an
  // estate with no out-of-model content in it at all.
  for member in NAMED_PAIR {
    let rel = format!("intent/{member}");
    assert!(
      before.contains_key(&rel),
      "{rel} was not in the estate before deletion, so this arm would compare an estate that \
       has no out-of-model content and report success"
    );
  }
  assert!(
    before.len() > 10,
    "the fixture estate is {} file(s), which is too few to be the populated estate this arm \
     claims to be reproducing",
    before.len()
  );

  delete_the_file_estate(&fx);
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("egest");
  let after = tree(fx.root());

  let unexplained: Vec<String> = changed(&before, &after)
    .into_iter()
    .filter(|path| !is_out_of_model(path))
    // The store is not the file estate: it is the thing being projected FROM,
    // and it legitimately advances -- `sync --to-disk` records its own act, so
    // the log is one event ahead of the file by construction.
    .filter(|path| !is_the_store(path))
    .collect();

  assert!(
    unexplained.is_empty(),
    "these paths differ after a delete-and-egest and are NOT in the out-of-model set declared \
     at {DATA_MODEL}. Either the egest is not an inverse of the ingest, or the declared set has \
     grown and the document has not been updated to say so:\n  {}",
    unexplained.join("\n  ")
  );

  // And the out-of-model pair really is gone -- if it came back, the set is
  // wrong rather than the egest, and this test would be certifying a stale
  // enumeration.
  for member in NAMED_PAIR {
    let rel = format!("intent/{member}");
    assert!(
      !after.contains_key(&rel),
      "{rel} came back from an egest, so it IS reproducible from the DB and {DATA_MODEL} is \
       wrong to list it as out of model -- the enumeration has outlived the model"
    );
  }
}

/// **DRIVEN TO BOTH VERDICTS, because a comparison that never reports a missing
/// file is not a comparison.**
///
/// The arms above pass on a healthy estate, which is indistinguishable from a
/// diff that cannot see a loss. Here a modelled file is removed from the store
/// side by egesting a NARROWER scope, so one thread's views are legitimately
/// not rewritten -- and the same comparison is required to notice.
#[test]
fn the_comparison_notices_a_modelled_file_that_did_not_come_back() {
  let fx = Fixture::new();
  let before = estate_with_the_out_of_model_pair(&fx);

  delete_the_file_estate(&fx);
  // ONE thread, not All -- so ST0002's files are not written back.
  fx.facade_on_disk()
    .sync_to_disk(&Scope::Threads(vec!["ST0001".into()]))
    .expect("a scoped egest");
  let after = tree(fx.root());

  let missing: Vec<&String> = before
    .keys()
    .filter(|path| !after.contains_key(*path))
    .filter(|path| !is_out_of_model(path) && !is_the_store(path))
    .collect();

  assert!(
    missing.iter().any(|p| p.contains("ST0002")),
    "a scoped egest left ST0002's modelled files unwritten and the comparison did not report \
     them, so the arms above are green against a diff that cannot see a loss. Reported: {missing:?}"
  );
}

/// **AC-10.8's SECOND HALF, AND IT IS UNBUILT.**
///
/// The criterion requires the out-of-model set *"named in the output rather
/// than silently absent"*. `Facade::sync_to_disk` returns `Result<usize>` and
/// the renderer prints `ok: extract written for {n} thread(s)` -- **a count,
/// naming nothing it could not reproduce.** So an operator who deletes their
/// estate and egests is told how many threads were written and is never told
/// that their `wip.md` is not coming back.
///
/// **THIS IS vc's OWN AC-10.5 FINDING ONE VERB OVER:** *"the check can
/// enumerate the loss" is a different claim from "the operation accounts for
/// it"*, and reading the criterion the other way makes it unfalsifiable, since
/// any amount of silence would pass so long as some test listed the set.
///
/// **EXPIRY, NAMED SO IT CANNOT BECOME PERMANENT: remove `#[ignore]` once the
/// egest names its residue.** Which output carries it is a specification
/// question routed to vc and deliberately not answered here -- on the egest
/// every run, only when non-empty, or on `doctor`/`export` with the row
/// reworded. **`#[ignore]` rather than a relaxed assertion, on vc's standing
/// ruling: relaxing a gate at the moment it stops covering anything converts a
/// refusal into a silent pass.** It says NOT RUN, in every run, where a reader
/// sees it.
#[test]
#[ignore = "AC-10.8's naming half is unbuilt: sync --to-disk reports a count and names no residue"]
fn the_egest_names_what_it_could_not_reproduce() {
  let fx = Fixture::new();
  estate_with_the_out_of_model_pair(&fx);
  delete_the_file_estate(&fx);

  let report = fx
    .facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("egest")
    .to_string();

  for member in NAMED_PAIR {
    assert!(
      report.contains(member),
      "the egest's output does not name `{member}`, which it cannot reproduce, so the operator \
       is told a count and not told what is missing: {report}"
    );
  }
}

/// The store: the thing being projected FROM, not part of the file estate.
///
/// **IT LEGITIMATELY DIFFERS AFTER AN EGEST AND THAT IS A FIXED POINT.**
/// `sync_to_disk` records its own act, so the log is exactly one event ahead of
/// the files it just wrote -- `facade.rs` argues this at the site rather than
/// buying a clean number by moving an unrecorded write elsewhere. Comparing it
/// would red every run for a reason that is not this criterion's subject.
fn is_the_store(path: &str) -> bool {
  path.starts_with("intent/.cache/")
}

/// Whether a path is in the declared out-of-model set.
///
/// Keyed on the pair the document NAMES, and deliberately not on a category
/// like "prose" -- a predicate that matched a category would decide for itself
/// what counts as unmodelled, which is the denominator problem again.
fn is_out_of_model(path: &str) -> bool {
  NAMED_PAIR
    .iter()
    .any(|member| path == format!("intent/{member}"))
}
