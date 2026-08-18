//! **`Thread::related`: v2's `## Related Steel Threads` bullets, modelled.**
//!
//! The prose was never lost -- it is carried in `body` like every other
//! unmodelled section, and three of us derived that separately before this was
//! built. **What was missing is the MODELLING**: `related` was empty on all 56
//! threads of this estate, which meant `doctor`'s broken-reference check had no
//! subject at all. A check that cannot fire is not a passing check.
//!
//! Every clause of the parse below is measured against this estate's 123
//! bullets across 52 files, and each has its own arm here because each was a
//! decision that could have gone the other way.

mod common;

use common::{Fixture, ctx};
use intentsvcs::legacy;
use intentsvcs::views;

fn estate(fixture: &Fixture, related: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0001/info.md",
    &format!(
      "---\nstatus: Completed\ncreated: 20260816\n---\n\n# ST0001: A thread\n\n## Objective\n\nShip it.\n\n## Context\n\nBecause.\n\n## Related Steel Threads\n\n{related}"
    ),
  );
}

fn links(fixture: &Fixture) -> Vec<(String, Option<String>)> {
  legacy::scan(&fixture.project()).expect("scan").threads[0]
    .related
    .iter()
    .map(|r| (r.id.clone(), r.note.clone()))
    .collect()
}

/// **All four separators v2 actually uses.** 42 bullets use `: `, 27 open a
/// parenthesis, 23 use `--` and 22 an em dash -- so a reader that knew only one
/// would take a title for an id region on most of the corpus.
#[test]
fn every_separator_the_estate_uses_splits_the_id_from_its_note() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "- ST0002: Core Script Framework\n- ST0003 -- the template system\n- ST0004 — the commands\n- ST0005 (the init command)\n",
  );

  assert_eq!(
    links(&fixture),
    vec![
      (
        "ST0002".to_string(),
        Some("Core Script Framework".to_string())
      ),
      (
        "ST0003".to_string(),
        Some("the template system".to_string())
      ),
      ("ST0004".to_string(), Some("the commands".to_string())),
      ("ST0005".to_string(), Some("the init command".to_string())),
    ]
  );
}

/// **A note that mentions another thread is not a link to it.**
///
/// 7 bullets on this estate do exactly this -- "cancelled in this ST as
/// overtaken by v2.9.0 work" and similar. Taking every id in the line would
/// model relationships nobody drew, and they would be indistinguishable from
/// the real ones afterwards.
#[test]
fn an_id_mentioned_in_the_note_is_not_modelled_as_a_link() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "- ST0010 -- cancelled here, overtaken by ST0034 and ST0035\n",
  );

  assert_eq!(
    links(&fixture),
    vec![(
      "ST0010".to_string(),
      Some("cancelled here, overtaken by ST0034 and ST0035".to_string())
    )],
    "one link and the other two ids stay prose, where their author put them"
  );
}

/// **Two ids sharing one note are two links.**
///
/// `ST0034/ST0035 -- produced most of the surface under review` appears twice
/// on this estate. Taking the first would model half a fact and say nothing
/// about the half it dropped.
#[test]
fn two_ids_in_one_bullet_are_two_links_carrying_the_same_note() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "- ST0034/ST0035 -- produced the surface under review\n",
  );

  assert_eq!(
    links(&fixture),
    vec![
      (
        "ST0034".to_string(),
        Some("produced the surface under review".to_string())
      ),
      (
        "ST0035".to_string(),
        Some("produced the surface under review".to_string())
      ),
    ]
  );
}

/// **A bullet with no id contributes nothing, and loses nothing.**
///
/// There are 9 on this estate and none is a link: five say "None" or "(none)"
/// in so many words, and the rest are prose relations -- a tech note, an
/// originating pilot, a sister-project sweep. The counter-assertion is the
/// point: the prose is still in `body`.
#[test]
fn a_bullet_that_names_no_thread_is_not_a_link_and_its_prose_survives() {
  let fixture = Fixture::new();
  estate(
    &fixture,
    "- None currently -- this is a new capability\n- **TN004**: a tech note, not a thread\n",
  );

  let thread = &legacy::scan(&fixture.project()).expect("scan").threads[0];
  assert!(
    thread.related.is_empty(),
    "neither bullet names a thread: {:?}",
    thread.related
  );
  assert!(
    thread.body.contains("None currently") && thread.body.contains("TN004"),
    "and both are carried verbatim in the body, which is where they always were"
  );
}

/// **The prose stays in `body` even when the links ARE modelled**, so the view
/// keeps the author's own wording rather than a list rebuilt from ids.
#[test]
fn modelling_the_links_does_not_remove_the_authored_section() {
  let fixture = Fixture::new();
  estate(&fixture, "- ST0002: Core Script Framework\n");

  let thread = &legacy::scan(&fixture.project()).expect("scan").threads[0];
  assert_eq!(thread.related.len(), 1, "the link is modelled");
  assert!(
    thread.body.contains("## Related Steel Threads"),
    "and the authored section is still carried -- modelling is additive"
  );
}

/// **THE COLLISION THIS WORK'S PRECONDITION EXISTS FOR.**
///
/// `views::info` generates a `## Related Steel Threads` section from
/// `thread.related`. The migrator also carries the AUTHORED one in `body`. Land
/// the field without the deferral and 52 threads get the heading twice -- which
/// is the `## Work Packages` defect this estate already shipped once and had to
/// fix after the fact.
///
/// The deferral arm was built ahead of this work and was dead until now, so
/// **this is the first test that exercises it at all**.
#[test]
fn the_generated_section_stands_down_for_the_authored_one() {
  let fixture = Fixture::new();
  estate(&fixture, "- ST0002: Core Script Framework\n");

  let out = legacy::scan(&fixture.project()).expect("scan");
  let deferred: Vec<&str> = out
    .dispositions
    .iter()
    .filter(|d| d.verdict == legacy::Verdict::Deferred)
    .map(|d| d.heading.as_str())
    .collect();
  assert_eq!(
    deferred,
    vec!["Related Steel Threads"],
    "the deferral is RECORDED, not merely effected -- a silent stand-down is \
     indistinguishable from a renderer that never generated the section"
  );

  let rendered = views::info(&out.threads[0], &ctx());
  assert_eq!(
    rendered.matches("## Related Steel Threads").count(),
    1,
    "and the heading appears exactly once in the view:\n{rendered}"
  );
}
