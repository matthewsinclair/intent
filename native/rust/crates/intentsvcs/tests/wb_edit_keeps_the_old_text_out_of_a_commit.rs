//! Issue 0523: `wb edit` changes one of the acting node's items, and WHERE THE
//! OLD TEXT IS AFTERWARDS is the point of it. Gtools' identifier gate refused
//! a board whose item text and originating event file both carried client
//! identifiers, and no verb could change either.
//!
//! **"COMMITTED" IS ASKED OF GIT, SO EVERY CASE HERE RUNS IN A REAL
//! REPOSITORY.** An event no commit holds is a draft and is amended in place;
//! one a commit holds is history, and a `wb.edit` event records the change
//! beside it. A fixture without a repository could drive only the first arm.
//!
//! **"NO FILE UNDER `intent/`" LEAVES OUT THE STORE, AND SAYS SO.**
//! `intent/.cache/` is this machine's database, which no commit carries (D34),
//! and SQLite keeps freed pages until they are reused, so a byte search of it
//! would measure the storage engine rather than the verb.

use std::path::Path;

use crate::common::Fixture;
use intentsvcs::facade::{Facade, FacadeError, NextCommit, Note, WbEdit};
use intentsvcs::model::WbItemKind;
use intentsvcs::remedy::Remedy;

const OLD: &str = "client ACME-4471 owes the renewal";
const NEW: &str = "a client owes the renewal";

/// A git repository holding a project whose `cc` board is registered.
fn board(fx: &Fixture) -> Facade {
  fx.git_init();
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f
}

/// Every file under `intent/` a commit could carry that holds `needle`,
/// project-relative. The store is left out, for the reason the header gives.
fn files_holding(fx: &Fixture, needle: &str) -> Vec<String> {
  fn walk(dir: &Path, root: &Path, needle: &str, out: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).expect("read dir") {
      let path = entry.expect("entry").path();
      if path.is_dir() {
        if path.file_name().is_some_and(|n| n == ".cache") {
          continue;
        }
        walk(&path, root, needle, out);
      } else if std::fs::read(&path)
        .expect("read file")
        .windows(needle.len())
        .any(|w| w == needle.as_bytes())
      {
        out.push(
          path
            .strip_prefix(root)
            .expect("under the root")
            .display()
            .to_string(),
        );
      }
    }
  }
  let mut out = Vec::new();
  walk(&fx.path("intent"), fx.root(), needle, &mut out);
  out.sort();
  out
}

/// The committed-or-not event files of one op, as their text.
fn event_files(fx: &Fixture, op: &str) -> Vec<String> {
  let spelled = format!("\"op\": \"{op}\"");
  files_holding(fx, &spelled)
    .into_iter()
    .filter(|p| p.starts_with("intent/.canon/events/"))
    .map(|p| fx.read(&p))
    .collect()
}

#[test]
fn an_uncommitted_origin_is_amended_and_the_old_text_is_in_no_file() {
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("add");
  assert!(
    !files_holding(&fx, OLD).is_empty(),
    "the rig holds the old text before the edit"
  );

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  assert!(
    matches!(&edited, WbEdit::Amended { next_commit: Some(next), .. } if next.is_empty()),
    "the scan ran and found nothing for the next commit to carry: {edited:?}"
  );
  assert_eq!(files_holding(&fx, OLD), Vec::<String>::new());
  assert!(
    event_files(&fx, "wb.edit").is_empty(),
    "a draft is amended, so no correction event carries anything into the commit"
  );
  assert!(
    event_files(&fx, "wb.add").iter().any(|e| e.contains(NEW)),
    "the originating event now carries the new text"
  );
}

#[test]
fn a_committed_origin_is_kept_and_a_wb_edit_records_the_new_text() {
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("add");
  fx.git_commit_all();
  let origin = event_files(&fx, "wb.add");

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  let WbEdit::Recorded { still_at_head, .. } = &edited else {
    panic!("a committed origin is recorded, not amended: {edited:?}");
  };
  assert!(
    still_at_head
      .iter()
      .any(|p| p.starts_with("intent/.canon/events/"))
      && still_at_head.contains(&"intent/whiteboard/cc/board.json".to_string()),
    "the answer names the committed event and the committed board: {still_at_head:?}"
  );
  assert_eq!(
    event_files(&fx, "wb.add"),
    origin,
    "the committed event is history and is left byte for byte"
  );
  assert!(
    event_files(&fx, "wb.edit")
      .iter()
      .any(|e| e.contains(NEW) && !e.contains(OLD)),
    "the correction event carries the new text and never the old"
  );
  let item = f
    .board("cc")
    .expect("the board")
    .items
    .into_iter()
    .find(|i| i.seq == seq)
    .expect("the item");
  assert_eq!(item.text, NEW);
  assert!(
    fx.read("intent/whiteboard/cc/board.json").contains(NEW),
    "the render honours the correction"
  );
}

#[test]
fn a_second_edit_before_the_commit_amends_the_first() {
  // The intermediate text must reach no commit either: the pending correction
  // event is the draft now, so it is the one amended.
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("add");
  fx.git_commit_all();
  let WbEdit::Recorded { event: first, .. } = f
    .wb_edit("cc", WbItemKind::Todo, seq, "an intermediate wording")
    .expect("first edit")
  else {
    panic!("a committed origin is recorded");
  };

  let second = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  assert_eq!(
    second,
    WbEdit::Amended {
      event: first,
      still_at_head: Vec::new(),
      next_commit: Some(NextCommit::default()),
      new_holds_old: false,
    },
    "no commit ever held the intermediate wording"
  );
  assert_eq!(
    files_holding(&fx, "an intermediate wording"),
    Vec::<String>::new()
  );
}

#[test]
fn a_board_committed_without_its_event_is_named_and_the_draft_is_still_amended() {
  // ic's review: a board commit that leaves its event file behind puts the
  // text at HEAD while the event is still a draft. The draft is amended, so
  // the next commit carries none of it, and the answer names what HEAD holds
  // instead of claiming nothing does. The text carries a quote, a backslash,
  // a non-ASCII character and a newline, so the search has to find it as the
  // `.json` file spells it, escaped, across what would be two lines raw.
  let tricky = "client \"ACME\" owes C:\\renewals by f\u{e9}vrier\nand a second line";
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, tricky).expect("add");
  fx.git(&["add", "--", "intent/whiteboard/cc/board.json"]);
  fx.git(&["commit", "-q", "-m", "the board without its event"]);

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  let WbEdit::Amended { still_at_head, .. } = &edited else {
    panic!("an uncommitted event is amended even when HEAD holds the text: {edited:?}");
  };
  assert_eq!(
    still_at_head,
    &vec!["intent/whiteboard/cc/board.json".to_string()]
  );
  assert!(
    matches!(&edited, WbEdit::Amended { next_commit: Some(next), .. } if next.is_empty()),
    "HEAD keeps what it holds, and the next commit carries none of it: {edited:?}"
  );
  let quoted = serde_json::to_string(tricky).expect("a str serialises");
  let escaped = &quoted[1..quoted.len() - 1];
  assert_eq!(
    (files_holding(&fx, tricky), files_holding(&fx, escaped)),
    (Vec::new(), Vec::new()),
    "the working tree holds the old text in neither spelling, so the next commit carries none"
  );
}

#[test]
fn an_archived_item_is_edited_like_a_live_one() {
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Watchout, OLD).expect("add");
  f.wb_archive("cc", WbItemKind::Watchout, seq)
    .expect("archive");

  f.wb_edit("cc", WbItemKind::Watchout, seq, NEW)
    .expect("edit");

  assert_eq!(files_holding(&fx, OLD), Vec::<String>::new());
}

#[test]
fn the_same_text_moves_nothing_and_records_nothing() {
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("add");
  let before = fx.read("intent/whiteboard/cc/board.json");

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, OLD).expect("edit");

  assert_eq!(edited, WbEdit::Unchanged);
  assert_eq!(fx.read("intent/whiteboard/cc/board.json"), before);
  assert!(event_files(&fx, "wb.edit").is_empty());
}

#[test]
fn another_nodes_item_is_not_addressable_and_the_refusal_names_the_board() {
  // The address is the acting node's own: `--node` names whose board the kind
  // and number are read on, so vc's todo is not an item of cc's.
  let fx = Fixture::new();
  let mut f = board(&fx);
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  let seq = f.wb_add("vc", WbItemKind::Todo, OLD).expect("add");

  let refused = f.wb_edit("cc", WbItemKind::Todo, seq, NEW);

  assert!(
    matches!(&refused, Err(FacadeError::WbNoSuchItem { node, .. }) if node == "cc"),
    "{refused:?}"
  );
  let remedy = refused.expect_err("refused").remedy();
  assert!(remedy.contains("intent wb show cc"), "{remedy}");
  assert!(
    fx.read("intent/whiteboard/vc/board.json").contains(OLD),
    "vc's item is untouched"
  );
}

#[test]
fn a_directive_is_refused_off_hv_s_board_as_adding_one_is() {
  let fx = Fixture::new();
  let mut f = board(&fx);

  assert!(matches!(
    f.wb_edit("cc", WbItemKind::Directive, 1, NEW),
    Err(FacadeError::WbDirectiveOffHv { .. })
  ));
}

#[test]
fn a_staged_draft_is_named_because_the_index_still_holds_the_old_text() {
  // A commit a gate refused leaves its paths staged: the case the verb is for.
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("add");
  fx.git(&["add", "-A", "--", "intent/.canon/events"]);
  f.take_notes();

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  let WbEdit::Amended {
    next_commit: Some(next),
    ..
  } = &edited
  else {
    panic!("a staged draft is still a draft, and the scan ran: {edited:?}");
  };
  assert!(
    next
      .would
      .iter()
      .any(|p| p.starts_with("intent/.canon/events/")),
    "the index still holds the old text, so the next commit would carry it: {next:?}"
  );
  let notes = f.take_notes();
  assert!(
    notes.iter().any(|n| matches!(
      n,
      Note::StagedBeforeTheEdit(paths) if paths.iter().any(|p| p.starts_with("intent/.canon/events/"))
    )),
    "{notes:?}"
  );
}

#[test]
fn a_carrier_less_item_never_borrows_a_later_items_creator() {
  // vc's hold on v2: text alone matched a LATER item that said the same thing,
  // and amending that item's draft rewrote ITS creation event. A migrated item
  // has no creator of its own, and the later item's creator lies at or after
  // that later item, outside the migrated one's window -- so the item added
  // after it in the same words keeps its record byte for byte, and the edit
  // is recorded.
  let fx = Fixture::new();
  let home = fx.root().join("intent/whiteboard/cc");
  std::fs::create_dir_all(&home).expect("cc's directory");
  std::fs::write(
    home.join("wip.md"),
    "---\nnode: cc\nname: Control Claude\nrole: control\nstatus: active\n---\n\n# Control Claude (cc)\n\n## TODO\n\n- the shared wording\n",
  )
  .expect("a hand-authored board");
  let mut f = fx.facade_on_disk();
  f.register_roster().expect("register the roster");
  f.wb_migrate("cc", false).expect("carry cc's board");
  let later = f
    .wb_add("cc", WbItemKind::Todo, "the shared wording")
    .expect("a later item in the same words");
  let before = event_files(&fx, "wb.add");

  let edited = f
    .wb_edit("cc", WbItemKind::Todo, 1, NEW)
    .expect("edit the carried item");

  assert!(matches!(edited, WbEdit::Recorded { .. }), "{edited:?}");
  assert_eq!(
    event_files(&fx, "wb.add"),
    before,
    "the later item's creation event is untouched"
  );
  let texts: Vec<(u32, String)> = f
    .board("cc")
    .expect("cc's board")
    .items
    .into_iter()
    .map(|i| (i.seq, i.text))
    .collect();
  assert_eq!(
    texts,
    vec![
      (1, NEW.to_string()),
      (later, "the shared wording".to_string())
    ]
  );
}

#[test]
fn an_earlier_event_in_the_items_millisecond_does_not_hide_its_creator() {
  // v3's miss, reproduced by a probe on 2026-09-23: the previous
  // transaction's event can share the item's millisecond while the item's own
  // `wb.add` lands in the next. v3 looked only in that first instant, missed
  // the creator, recorded the edit and left the old text in the draft. The
  // stamps are set here to that state, which the clock produces only
  // sometimes on its own.
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("add");
  let db = rusqlite::Connection::open(fx.project().db_path()).expect("a second connection");
  let stamp: String = db
    .query_row(
      "SELECT recorded_at FROM wb_item WHERE node = 'cc' AND kind = 'todo' AND seq = ?1",
      [seq],
      |r| r.get(0),
    )
    .expect("the item's stamp");
  let creator: String = db
    .query_row("SELECT id FROM event_log WHERE op = 'wb.add'", [], |r| {
      r.get(0)
    })
    .expect("the creator");
  let earlier: String = db
    .query_row(
      "SELECT id FROM event_log WHERE id <> ?1 ORDER BY ts DESC, id DESC LIMIT 1",
      [&creator],
      |r| r.get(0),
    )
    .expect("the event before it");
  let second = &stamp[..20];
  let (shared, next) = (format!("{second}500Z"), format!("{second}501Z"));
  db.execute(
    "UPDATE wb_item SET recorded_at = ?1 WHERE node = 'cc' AND kind = 'todo' AND seq = ?2",
    rusqlite::params![shared, seq],
  )
  .expect("the item");
  db.execute(
    "UPDATE event_log SET ts = ?1 WHERE id = ?2",
    rusqlite::params![shared, earlier],
  )
  .expect("the earlier event");
  db.execute(
    "UPDATE event_log SET ts = ?1 WHERE id = ?2",
    rusqlite::params![next, creator],
  )
  .expect("the creator");

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  assert!(
    matches!(&edited, WbEdit::Amended { next_commit: Some(next), .. } if next.is_empty()),
    "the creator is found and amended: {edited:?}"
  );
  assert_eq!(files_holding(&fx, OLD), Vec::<String>::new());
}

#[test]
fn a_carrier_the_rules_cannot_see_is_named_as_what_the_next_commit_would_carry() {
  // vc's arm for v4: whatever the carrier rule, a miss leaves the old text in
  // a draft, and the answer names it rather than claiming nothing holds it.
  // The miss is planted by changing the text the store records for the
  // creator, so no rule matching on text can take it, while its file on disk
  // still holds the old text.
  let fx = Fixture::new();
  let mut f = board(&fx);
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("add");
  let db = rusqlite::Connection::open(fx.project().db_path()).expect("a second connection");
  db.execute(
    "UPDATE event_log SET payload = json_set(payload, '$.text', 'planted') WHERE op = 'wb.add'",
    [],
  )
  .expect("plant the miss");

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  let WbEdit::Recorded {
    next_commit: Some(next),
    ..
  } = &edited
  else {
    panic!("a carrier the rules cannot see is recorded, and the scan ran: {edited:?}");
  };
  let holding = files_holding(&fx, OLD);
  assert!(
    !holding.is_empty()
      && holding
        .iter()
        .all(|p| p.starts_with("intent/.canon/events/")),
    "the rig leaves the old text in the creator's draft and nowhere else: {holding:?}"
  );
  assert_eq!(
    (&next.would, &next.could),
    (&Vec::new(), &holding),
    "the untracked draft could reach the next commit, and every file still holding it is named"
  );
}

#[test]
fn a_peers_committed_copy_of_the_text_is_named_at_head() {
  // ic's review of v4: HEAD was read only under the editor's own board, so a
  // peer's committed item saying the same thing went unseen and the answer
  // said the old text reached no commit. A commit's tree carries every
  // unchanged file, so the peer's copy is in the next commit too, and HEAD's
  // search now covers all of `intent/`.
  let fx = Fixture::new();
  let mut f = board(&fx);
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f.wb_add("vc", WbItemKind::Todo, OLD).expect("vc's copy");
  fx.git_commit_all();
  let seq = f.wb_add("cc", WbItemKind::Todo, OLD).expect("cc's draft");

  let edited = f.wb_edit("cc", WbItemKind::Todo, seq, NEW).expect("edit");

  let WbEdit::Amended { still_at_head, .. } = &edited else {
    panic!("cc's own draft is still amended: {edited:?}");
  };
  assert!(
    still_at_head.contains(&"intent/whiteboard/vc/board.json".to_string())
      && still_at_head.contains(&"intent/whiteboard/vc/wip.md".to_string())
      && still_at_head
        .iter()
        .any(|p| p.starts_with("intent/.canon/events/")),
    "the peer's committed board and event are named: {still_at_head:?}"
  );
  assert!(
    still_at_head
      .iter()
      .all(|p| !p.starts_with("intent/.cache/")),
    "the store is never named, even where this fixture committed it: {still_at_head:?}"
  );
}
