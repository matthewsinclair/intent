//! **AT-19.1 / AC-19.1: hits come back in ONE envelope, grouped by tier,
//! ranked within a tier, with both denominators.**
//!
//! The envelope is built before the corpus it will carry (WP-18 is cc's, in
//! flight), and that order is deliberate: the design's structural claim is that
//! **a tier is a GROUP and a corpus is an ENTRY**, so the CLI contract and the
//! MCP schema must not move when WP-20's structural tier or cc's source corpus
//! lands. Nothing tests that claim except building the second one, so this file
//! pins the shape the first one settles.
//!
//! **WHY THE ASSERTIONS ARE RELATIONS RATHER THAN COUNTS.** `matched == 2` pins
//! a fixture; `returned <= matched`, `matched` unmoved by a cap, and scores
//! ordered within a group are the properties the surface promises. They survive
//! a fixture gaining a section and they catch the defects the AC names.

use crate::common::{Fixture, sample_thread};
use intentsvcs::model::Attachment;
use intentsvcs::search::{SearchQuery, Tier};

/// A thread whose attachments carry one word twice, so a cap has something to
/// cap: on a one-hit estate `matched` and `returned` are the same number and
/// the denominator case cannot be exhibited at all.
fn estate() -> Fixture {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.attachments = vec![
    Attachment::new("one.md", "# One\n\nA quokka lives in this section.\n"),
    Attachment::new("two.md", "# Two\n\nA quokka, and a second quokka.\n"),
  ];
  fx.write_thread(&thread);
  fx
}

#[test]
fn one_tier_is_still_a_group_and_carries_its_own_order() {
  let fx = estate();
  let f = fx.facade();
  let answer = f
    .search_all("quokka", &SearchQuery::default())
    .expect("the search answered");

  // **TWO TIERS ARE BUILT NOW, SO THERE ARE TWO GROUPS** -- the structural tier
  // landed with WP-20's integration, and the claim this arm makes is that a
  // TIER is a group, not that there is one of them. The group is present
  // whether or not it has hits, because an absent group reads as a tier that is
  // not built.
  assert_eq!(
    answer.groups.len(),
    2,
    "the lexical and structural tiers are built, so there are two groups"
  );
  assert_eq!(answer.groups[0].tier, Tier::Lexical);
  assert_eq!(answer.groups[1].tier, Tier::Structural);
  assert!(
    !answer.groups[0].hits.is_empty(),
    "the fixture's word is indexed, so the group is not empty: {answer:?}"
  );
  assert_eq!(answer.query, "quokka", "the answer carries the words asked");

  // **RANKED WITHIN THE TIER.** FTS5 ranks best-first and lower is better, so
  // the scores the envelope publishes must be non-decreasing down the group --
  // otherwise the surface is showing an order it did not compute.
  let scores: Vec<f64> = answer.groups[0].hits.iter().map(|hit| hit.score).collect();
  assert!(
    scores.windows(2).all(|pair| pair[0] <= pair[1]),
    "hits are not in the order their scores claim: {scores:?}"
  );
}

#[test]
fn both_denominators_travel_and_a_cap_moves_only_one_of_them() {
  let fx = estate();
  let f = fx.facade();
  let all = f
    .search_all("quokka", &SearchQuery::default())
    .expect("the unrestricted search answered");
  assert_eq!(
    all.returned, all.matched,
    "nothing was capped, so the two denominators agree"
  );
  assert_eq!(all.returned, all.groups[0].hits.len());

  let capped = f
    .search_all(
      "quokka",
      &SearchQuery {
        limit: Some(1),
        ..SearchQuery::default()
      },
    )
    .expect("the capped search answered");
  assert_eq!(
    capped.matched, all.matched,
    "a cap is a bound on the answer, not on the question: `matched` must not move"
  );
  assert_eq!(capped.returned, 1, "the cap was honoured");
  assert!(
    capped.returned < capped.matched,
    "the fixture must be able to exhibit a capped answer, or this test proves nothing"
  );
}

#[test]
fn a_filter_is_part_of_the_question_so_the_denominator_counts_what_survives_it() {
  let fx = estate();
  let f = fx.facade();
  // `issue` is a real kind this index can hold and the fixture has none of, so
  // the filter empties the group without emptying the corpus.
  let filtered = f
    .search_all(
      "quokka",
      &SearchQuery {
        kinds: vec![intentsvcs::search::HitKind::Issue],
        ..SearchQuery::default()
      },
    )
    .expect("the filtered search answered");
  assert_eq!(
    filtered.matched, 0,
    "`matched` reports what matched the question that was asked, filter included"
  );
  assert_eq!(
    filtered.groups.len(),
    2,
    "every built tier is still a group, even the ones the filter emptied"
  );
  assert!(filtered.groups.iter().all(|g| g.hits.is_empty()));
  assert!(
    !filtered.index.is_empty(),
    "an empty ANSWER over a populated index must not read as an empty index"
  );
}
