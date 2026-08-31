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

use common::{
  DATA_MODEL, Fixture, OUT_OF_MODEL_HEADING, changed, data_model_text, out_of_model_enumeration,
  out_of_model_section, sample_issue, sample_thread, tree,
};
use intentsvcs::sync::Scope;

/// The out-of-model members this fixture can plant, because they are the only
/// ones named as PATHS rather than as categories.
const NAMED_PAIR: &[&str] = &["wip.md", "restart.md"];

/// The two directories the egest reads FROM and therefore may not be deleted:
/// the store it projects, and the config that says where anything lives.
const NOT_THE_FILE_ESTATE: &[&str] = &[".cache", ".config"];

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
  // **THE ENUMERATION HALF, NOT THE WHOLE SECTION, AND THAT WAS A LIVE HOLE IN
  // THIS ARM.** It used to ask whether the SECTION contained the justifying
  // phrase. The section states the whiteboard's DEPARTURE inside itself -- *The
  // whiteboard left this set at D30 ... modelled above as `wb_node`* -- so a
  // member justified by any phrase from that paragraph passed here while the
  // document was saying the exact opposite. **A mention is not an instance, in
  // the instrument written to stop precisely that.** Found 2026-08-31 when the
  // mirror check in `the_migrator_says_what_it_did_not_carry.rs` fired on its
  // own author's first draft; latent here rather than live, because no current
  // member is justified from the departure prose.
  let (enumeration, _exceptions) = out_of_model_enumeration(section);

  // **THE PRODUCT'S OWN DECLARATION IS THE SUBJECT, NOT THIS TEST'S IDEA OF
  // IT.** `NOT_CARRIED` is what the shipped qualifier is built from, so pinning
  // anything else would leave the emitted sentence free to name a member no
  // document authorises -- the denominator problem moved one file over.
  for member in intentsvcs::sync::NOT_CARRIED {
    assert!(
      enumeration.contains(member.justified_by),
      "the shipped qualifier names `{}`, justified by the phrase `{}`, and the ENUMERATION \
       under `## What is deliberately not modelled` no longer carries that phrase. If it is now \
       only in the section's departure prose, the document says that thing LEFT the excluded \
       set and the declaration has it backwards. Re-derive from the section as it now reads -- \
       do not adjust the phrase to match, which is the claim certifying itself",
      member.shown,
      member.justified_by
    );
  }

  // **AND THE FIXTURE'S PLANTED PAIR MUST BE SOMETHING THE PRODUCT ACTUALLY
  // DECLARES.** Without this the arms below could plant files the shipped set
  // never mentions, and the two would drift apart while both stayed green.
  for planted in NAMED_PAIR {
    assert!(
      intentsvcs::sync::NOT_CARRIED
        .iter()
        .any(|m| m.justified_by.contains(planted)),
      "this fixture plants `{planted}` as out-of-model, but no member of the shipped \
       `NOT_CARRIED` set is justified by a phrase naming it, so the test and the product \
       disagree about what the egest cannot carry"
    );
  }

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

/// **AC-10.8's SECOND HALF: the output NAMES what it could not reproduce.**
///
/// The criterion requires the out-of-model set *"named in the output rather
/// than silently absent"*. Until 2026-08-31 `sync --to-disk` printed
/// `ok: extract written for {n} thread(s)` -- **a count, naming nothing it
/// could not reproduce** -- so an operator who deleted their estate and egested
/// was told how many threads were written and never told their `wip.md` was
/// not coming back.
///
/// **vc RULED THE FORM, AND IT WAS NONE OF THE THREE OPTIONS PUT UP.** All
/// three asked *where do we print the list*; the ruling is that the defect is
/// the claim itself -- the sentence asserts a completeness it does not have --
/// so the correction is **a qualifier on that sentence, never an enumeration
/// beside it**. Printing the set on a second line, or on `doctor`, leaves
/// `ok: extract written for 301 thread(s)` saying exactly what it said before.
///
/// **THIS IS vc's OWN AC-10.5 FINDING ONE VERB OVER:** *"the check can
/// enumerate the loss" is a different claim from "the operation accounts for
/// it"*, and reading the criterion the other way makes it unfalsifiable, since
/// any amount of silence would pass so long as some test listed the set.
#[test]
fn the_egest_names_what_it_could_not_reproduce() {
  let fx = Fixture::new();
  estate_with_the_out_of_model_pair(&fx);
  delete_the_file_estate(&fx);

  let count = fx
    .facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("egest");
  let claim = intentsvcs::sync::extract_written(count);

  for member in intentsvcs::sync::NOT_CARRIED {
    assert!(
      claim.contains(member.shown),
      "the egest's claim does not name `{}`, which it cannot reproduce, so the operator is \
       told a count and not told what is missing: {claim}",
      member.shown
    );
  }

  // **THE COUNT IS STILL THERE, because the qualifier is an addition to the
  // claim and not a replacement for it.** A sentence that named the residue
  // and dropped the number would trade one silence for another.
  assert!(
    claim.contains(&count.to_string()),
    "the claim stopped carrying the thread count it is the report OF: {claim}"
  );
}

/// **EVERY DECLARED MEMBER REALLY FAILS TO RETURN, MEASURED RATHER THAN
/// ASSERTED.**
///
/// **THE DENOMINATOR ATTACK HAS AN INVERSE AND ONLY THIS ARM SEES IT.** The
/// arm above and [`the_denominator_is_declared_where_the_criterion_says_it_is`]
/// together stop the set from SHRINKING without the document's leave. Neither
/// stops it GROWING: a member added to `NOT_CARRIED` that the egest reproduces
/// perfectly well would sail through both, and would quietly excuse the
/// operation from reproducing something it is perfectly capable of
/// reproducing. That is how a residue list becomes a licence.
///
/// So each declared member gets a specimen planted, ingested, deleted and
/// egested, and **each one must still be missing afterwards.**
///
/// **THE MAPPING IS ASSERTED TOTAL BEFORE IT IS USED**, because a specimen
/// table that silently skipped an unmapped member would go green on exactly
/// the addition it exists to catch.
#[test]
fn every_declared_member_really_fails_to_return() {
  /// One plantable instance per declared member. The path is this fixture's
  /// choice; that the member belongs in the set at all is the document's.
  const SPECIMENS: &[(&str, &str, &str)] = &[
    (
      "prose",
      "intent/docs/freeform.md",
      "# Freeform\n\nauthored, in no thread\n",
    ),
    (
      "shipped content",
      "intent/.claude/skills/in-specimen/SKILL.md",
      "# shipped, embedded in the binary\n",
    ),
    (
      "wip/restart",
      "intent/wip.md",
      "# WIP\n\nauthored, in no table\n",
    ),
  ];

  // **LENGTH FIRST, AND IT IS THE ANTI-SHRINK HALF.** The per-member loop
  // below only walks what `NOT_CARRIED` still declares, so DROPPING a member
  // makes it loop less and pass -- and a dropped member is the qualifier
  // quietly telling the operator less than it used to. Comparing the counts is
  // what turns a removal into a red.
  assert_eq!(
    SPECIMENS.len(),
    intentsvcs::sync::NOT_CARRIED.len(),
    "the shipped `NOT_CARRIED` set and this arm's specimen table have drifted apart. If a \
     member was REMOVED, the qualifier now names less than it did and nothing else here would \
     have said so -- re-derive from `data-model.md` rather than deleting the specimen to match"
  );

  for member in intentsvcs::sync::NOT_CARRIED {
    assert!(
      SPECIMENS.iter().any(|(shown, ..)| *shown == member.shown),
      "`NOT_CARRIED` declares `{}` and this arm has no specimen for it, so the declaration \
       grew and nothing measured whether the egest can actually reproduce it",
      member.shown
    );
  }

  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  for (_, path, body) in SPECIMENS {
    fx.write_file(path, body);
  }
  // **INGESTED FIRST, so a file that fails to return failed to be REPRODUCED
  // rather than merely never having been offered.** Without this the arm
  // proves only that the fixture wrote the files after the store last looked,
  // which is true of any file at all and would pass for every possible set.
  fx.facade_on_disk()
    .sync_from_disk(&Scope::All)
    .expect("the estate ingests");
  delete_the_file_estate(&fx);
  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("the store still holds the estate, so it egests");
  let after = tree(fx.root());

  let returned: Vec<&str> = SPECIMENS
    .iter()
    .filter(|(_, path, _)| after.contains_key(*path))
    .map(|(shown, ..)| *shown)
    .collect();

  assert!(
    returned.is_empty(),
    "the qualifier tells the operator these are not modelled, and the egest reproduced them \
     anyway: {returned:?}. A member the extract CAN carry does not belong in the declared set \
     -- naming it there is a licence to lose it later"
  );
}

/// **AND `are unchanged` IS THE HALF AN OPERATOR ACTS ON.**
///
/// The qualifier makes two claims and the arms above test one. *Not modelled*
/// tells the reader the extract is incomplete; ***unchanged* tells them their
/// authored files survived the run**, which is the half that decides whether
/// they reach for a backup. A sentence that kept saying it after the egest
/// started overwriting `wip.md` would be worse than the bare count it
/// replaced: silence sends you to check, and a false reassurance does not.
#[test]
fn what_the_claim_calls_unchanged_is_left_byte_identical() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let authored = "# WIP\n\nauthored, and the egest must not touch this\n";
  fx.write_file("intent/wip.md", authored);
  fx.write_file("intent/restart.md", authored);
  fx.write_file("intent/docs/freeform.md", authored);

  fx.facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("a populated estate egests");

  for path in [
    "intent/wip.md",
    "intent/restart.md",
    "intent/docs/freeform.md",
  ] {
    let now = std::fs::read_to_string(fx.path(path)).unwrap_or_else(|e| {
      panic!("the claim says `{path}` is unchanged and the egest removed it: {e}")
    });
    assert_eq!(
      now, authored,
      "the claim says `{path}` is unchanged and the egest rewrote it"
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
