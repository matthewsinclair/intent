//! **AT-19.3 and AT-19.5 / AC-19.3 and AC-19.5: an index that could not answer
//! the whole question says so, and a line is a claim about the disk.**
//!
//! These two rows are one property seen from two distances. AC-19.3 is the
//! estate's dominant defect class (AC-10.7) applied to an index: **an empty or
//! partial answer is byte-identical to a genuine miss**, so the surface must
//! carry the distinction or the reader invents it. AC-19.5 is the same rule per
//! hit: a line pointing at bytes that have moved is a confident wrong answer an
//! editor acts on.
//!
//! **THE THIRD CASE IS THE CONTROL, AND IT IS THE ONE THAT ALMOST SHIPPED
//! WRONG.** A hit with no line has three causes -- a file canon has not
//! realised, a section whose body was never a byte range of any file (every
//! canon section: the body is a JSON-escaped FIELD), and a file whose bytes
//! have genuinely moved. Only the third is staleness. Folding them reports a
//! healthy project as stale on every canon hit, which is why the unrealised
//! case is asserted here beside the moved one: without it, a green on staleness
//! would also be green on marking everything stale.

use crate::common::{Fixture, sample_thread};
use intentsvcs::model::Attachment;
use intentsvcs::search::SearchQuery;

const BODY: &str = "# Notes\n\nA quokka lives in this section.\n";

fn fixture_with_an_attachment() -> Fixture {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.attachments = vec![Attachment::new("notes.md", BODY)];
  fx.write_thread(&thread);
  fx
}

/// AT-19.3: **an index holding nothing is reported as holding nothing**, in the
/// envelope, where every surface can read it -- rather than each surface asking
/// a second question it has to know to ask.
#[test]
fn an_empty_index_is_named_in_the_envelope_and_is_not_a_miss() {
  let fx = Fixture::new();
  let f = fx.facade();
  let answer = f
    .search_all("quokka", &SearchQuery::default())
    .expect("the search answered");

  assert_eq!(answer.matched, 0, "nothing is indexed, so nothing matched");
  assert!(
    answer.index.is_empty(),
    "the envelope must say the corpus is empty: {:?}",
    answer.index.corpora
  );
  let canon = answer
    .index
    .corpora
    .get(intentsvcs::search::corpus_key(
      &intentsvcs::index::corpus::Corpus::Canon,
    ))
    .expect("the canon corpus is named even when it holds nothing");
  assert_eq!(canon.files, 0);
  assert!(
    !canon.policy.is_empty(),
    "a corpus that does not say how its freshness is decided has not answered the question"
  );
}

/// AT-19.5: **a line where the bytes are, and no line where they are not.**
#[test]
fn a_line_is_reported_only_where_the_indexed_bytes_are_still_on_the_disk() {
  let fx = fixture_with_an_attachment();
  let f = fx.facade();
  let path = "intent/st/ST0001/notes.md";

  // (a) THE FILE IS NOT REALISED. `st attach` writes the store and canon and
  // never the disk file, so this is the ordinary state of a fresh estate.
  let answer = f
    .search_all("quokka", &SearchQuery::default())
    .expect("the search answered");
  let hit = answer.groups[0]
    .hits
    .iter()
    .find(|hit| hit.path == path)
    .unwrap_or_else(|| panic!("the attachment was not indexed: {answer:?}"));
  assert!(hit.span.is_none(), "there is no file to claim a line in");
  assert!(
    !hit.stale,
    "an unrealised file is not a stale file -- marking it stale turns `complete` false for a healthy project"
  );
  assert!(
    answer.index.complete(),
    "nothing is stale and nothing was skipped: {:?}",
    answer.index.stale
  );

  // (b) THE FILE IS ON DISK CARRYING THE INDEXED BYTES. The line is found
  // rather than computed from an offset recorded at index time, so it is right
  // by construction.
  std::fs::create_dir_all(fx.path("intent/st/ST0001")).expect("mkdir the thread dir");
  std::fs::write(fx.path(path), BODY).expect("realise the attachment");
  let answer = f
    .search_all("quokka", &SearchQuery::default())
    .expect("the search answered");
  let hit = answer.groups[0]
    .hits
    .iter()
    .find(|hit| hit.path == path)
    .expect("the attachment is still indexed");
  let span = hit
    .span
    .expect("the bytes are on the disk, so there is a line");
  let line = String::from_utf8(
    std::fs::read(fx.path(path))
      .expect("read back")
      .split(|byte| *byte == b'\n')
      .nth(span.start_line as usize - 1)
      .expect("the line the span names exists")
      .to_vec(),
  )
  .expect("utf8");
  assert!(
    line.contains("quokka") || line.starts_with('#'),
    "the span must name a line of the file the match is in, not an offset in the index: {line:?}"
  );
  assert!(!hit.stale);

  // (c) THE BYTES MOVE UNDER THE INDEX. The hit keeps its path -- the file is
  // still the answer -- loses its line, and says why.
  std::fs::write(fx.path(path), "# Notes\n\nA quokka moved on.\n").expect("edit under the index");
  let answer = f
    .search_all("quokka", &SearchQuery::default())
    .expect("the search answered");
  let hit = answer.groups[0]
    .hits
    .iter()
    .find(|hit| hit.path == path)
    .expect("a stale hit is still a hit");
  assert!(
    hit.span.is_none(),
    "a line into moved bytes would be believed"
  );
  assert!(hit.stale, "the reason for the missing line is stated");
  assert!(
    !answer.index.complete(),
    "an answer carrying a stale hit is not a complete answer"
  );
  assert!(
    answer.index.stale.iter().any(|stale| stale == path),
    "the freshness block names the path: {:?}",
    answer.index.stale
  );
}
