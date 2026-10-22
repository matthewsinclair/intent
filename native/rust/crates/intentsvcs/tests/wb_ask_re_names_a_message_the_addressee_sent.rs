//! Issue 0610: `wb ask --re` names a message the addressee sent the asker, and
//! stores its address in canonical form.
//!
//! The anchor used to be stored as typed, so `--re "2026-10-18 18:0"` went out
//! as `Re: 2026-10-18 18:0`, threading against nothing. It now resolves through
//! the one resolver `wb edit message` uses, with the direction reversed, so
//! both verbs read one grammar and refuse in one voice.

use crate::common::Fixture;
use intentsvcs::facade::{Facade, FacadeError};

/// The stamp an inbox heading shows for a message sent at `recorded_at`.
fn heading_stamp(recorded_at: &str) -> String {
  format!("{} {}Z", &recorded_at[..10], &recorded_at[11..16])
}

/// A git repository holding a project with `cc` and `vc` registered.
fn two_nodes(fx: &Fixture) -> Facade {
  fx.git_init();
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f
}

/// `cc`'s board carrying three messages from `vc`, by `wb migrate`: a carry
/// stamps every row it writes with its one instant, so the three share a
/// heading minute without depending on the clock.
fn three_carried_from_vc(fx: &Fixture) -> Facade {
  fx.git_init();
  for (node, name, role) in [
    ("cc", "Control Claude", "control"),
    ("vc", "Validation Claude", "validation"),
  ] {
    let dir = fx.root().join("intent/whiteboard").join(node);
    std::fs::create_dir_all(&dir).expect("a board directory");
    std::fs::write(
      dir.join("wip.md"),
      format!(
        "---\nnode: {node}\nname: {name}\nrole: {role}\nstatus: active\n---\n\n# {name} ({node})\n"
      ),
    )
    .expect("a board header");
  }
  std::fs::write(
    fx.root().join("intent/whiteboard/cc/inbox.vc.md"),
    "# inbox: vc -> cc\n\n## (2026-09-12 09:00Z)\n\nthe first question\n\n\
     ## (2026-09-12 09:05Z)\n\nthe second question\n\n\
     ## (2026-09-12 09:09Z)\n\nthe third question\n",
  )
  .expect("an inbox from vc");
  let mut f = fx.facade_on_disk();
  f.register_roster().expect("register the roster");
  f.wb_migrate("cc", false).expect("carry cc's board");
  f.wb_migrate("vc", false).expect("carry vc's board");
  f
}

/// What `cc` has sent `vc`, as `cc`'s board holds it: a message lives in its
/// sender's board.
fn sent_by_cc(f: &Facade) -> Vec<(String, Option<String>)> {
  f.board("cc")
    .expect("cc's board")
    .sent
    .into_iter()
    .filter(|m| m.recipient == "vc")
    .map(|m| (m.body, m.re))
    .collect()
}

/// The heading stamp of the first message `node` has received.
fn first_received(f: &Facade, node: &str) -> String {
  heading_stamp(&f.board_read(node, true).expect("the board").messages[0].recorded_at)
}

/// The incident's shape: a stamp cut short is refused, the refusal lists the
/// messages it could have meant with each address ready to copy, and nothing
/// is sent.
#[test]
fn a_partial_anchor_is_refused_with_the_messages_it_could_mean() {
  let fx = Fixture::new();
  let mut f = three_carried_from_vc(&fx);
  let minute = first_received(&f, "cc");
  let partial = &minute[..minute.len() - 2];

  let refused = f.wb_ask("cc", "vc", "a reply", Some(partial), false);

  assert!(
    matches!(&refused, Err(FacadeError::WbNoSuchMessage { .. })),
    "{refused:?}"
  );
  let said = refused.expect_err("refused").to_string();
  for listed in [
    format!("{minute}#1  \"the first question\""),
    format!("{minute}#3  \"the third question\""),
  ] {
    assert!(said.contains(&listed), "`{listed}` is not offered: {said}");
  }
  assert_eq!(sent_by_cc(&f), Vec::new(), "a refused reply was sent");
}

/// A whole address is stored in canonical form: `#<n>` names one of several
/// in a minute, and a minute holding several without it is refused with the
/// listing, as `wb edit message` refuses it.
#[test]
fn a_named_message_is_stored_as_its_canonical_address() {
  let fx = Fixture::new();
  let mut f = three_carried_from_vc(&fx);
  let minute = first_received(&f, "cc");

  f.wb_ask(
    "cc",
    "vc",
    "the answer",
    Some(format!("{minute}#2").as_str()),
    false,
  )
  .expect("#2 names one");
  let ambiguous = f.wb_ask("cc", "vc", "a second answer", Some(minute.as_str()), false);

  assert_eq!(
    sent_by_cc(&f),
    vec![("the answer".to_string(), Some(format!("{minute}#2")))]
  );
  assert!(
    matches!(
      &ambiguous,
      Err(FacadeError::WbMessageAmbiguous { count: 3, .. })
    ),
    "{ambiguous:?}"
  );
}

/// The direction: the anchor names a message the ADDRESSEE sent, so a message
/// the asker sent is no anchor for its own reply, and a lone message's anchor
/// is stored as its stamp alone.
#[test]
fn only_a_message_the_addressee_sent_is_answered() {
  let fx = Fixture::new();
  let mut f = two_nodes(&fx);
  f.wb_ask("cc", "vc", "cc's own question", None, false)
    .expect("cc asks");
  let own = first_received(&f, "vc");

  let refused = f.wb_ask("cc", "vc", "a reply", Some(own.as_str()), false);

  assert!(
    matches!(&refused, Err(FacadeError::WbNoSuchMessage { .. })),
    "{refused:?}"
  );
  assert!(
    refused
      .expect_err("refused")
      .to_string()
      .contains("`vc` has sent `cc` no message"),
    "the refusal does not say there is nothing to answer"
  );

  f.wb_ask("vc", "cc", "vc's question", None, false)
    .expect("vc asks");
  let theirs = first_received(&f, "cc");
  f.wb_ask("cc", "vc", "the answer", Some(theirs.as_str()), false)
    .expect("a message vc sent is answered");
  assert_eq!(
    sent_by_cc(&f).last(),
    Some(&("the answer".to_string(), Some(theirs)))
  );
}
