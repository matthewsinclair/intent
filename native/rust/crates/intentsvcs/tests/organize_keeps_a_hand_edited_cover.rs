//! Issue 0641: `organize --apply` wrote the store's cover over a hand-edited
//! `info.md` put back at a dehydrated thread's path, with no `diverged` or
//! `refused` verdict, and the hand-written Objective line was gone.
//!
//! **hv RULED IT PROTECTED** (decision 77): the cover's contract keeps an edit
//! to its Objective or Context until `intent sync --to-store <ID>` takes it in,
//! and it has no exception for how the file reached the path. So the preview
//! names the path `diverged`, `--apply` refuses it with the sync named, and the
//! file is left as it was.
//!
//! **THE TEST IS `views::carriable_cover`, THE ONE THE OTHER RENDER-OVER
//! REFUSALS ASK**, and it cannot tell a hand edit from a copy the store has
//! moved past. So the control is the copy unedited over an unmoved store, and
//! the third arm pins that an unedited copy over a MOVED store is kept too: a
//! later change that made it a silent rewrite again reds here.

use crate::common::{Fixture, gate_open, sample_thread};
use intentsvcs::address::{Address, Entity};
use intentsvcs::facade::Facade;
use intentsvcs::organize::{Mode, OrganizeError, Report};
use intentsvcs::remedy::Remedy;
use serde_json::json;

const COVER: &str = "intent/st/ST0001/info.md";
const HAND_LINE: &str = "A line written by hand under the Objective.";

fn thread() -> Address {
  Address {
    authority: None,
    entity: Entity::Thread {
      id: "ST0001".to_string(),
    },
    format: None,
  }
}

/// cc's measured sequence, to the point the copy goes back: realise the
/// thread, keep its cover, dehydrate it, and, when `move_store` is set, change
/// its objective in the store while it is dehydrated. `edit` decides whether
/// the copy put back carries a hand-written line under `## Objective`. The
/// thread is declared again, and the facade returned is the one `organize`
/// runs on.
fn put_back(fx: &Fixture, move_store: bool, edit: bool) -> (Facade, String) {
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_thread(&gate_open());
  fx.write_file(
    "intent/.intentfiles",
    "STEELTHREAD:ST0001\nSTEELTHREAD:ST0057\n",
  );
  let mut f = fx.facade();
  f.hydrate(&thread()).expect("a declared thread realises");
  let kept = fx.read(COVER);

  f.dehydrate(&thread()).expect("dehydrate");
  assert!(
    !fx.path(COVER).exists(),
    "the dehydrated thread's cover is still on disk"
  );
  if move_store {
    f.set(
      &thread(),
      "objective",
      json!("The store's newer objective."),
    )
    .expect("set the objective while dehydrated");
  }

  let copy = match edit {
    true => kept.replacen(
      "## Objective\n\n",
      &format!("## Objective\n\n{HAND_LINE}\n"),
      1,
    ),
    false => kept,
  };
  assert!(
    !edit || copy.contains(HAND_LINE),
    "the hand edit did not land in the copy, so this arm proves nothing"
  );
  fx.write_file(COVER, &copy);
  let manifest = fx.read("intent/.intentfiles");
  fx.write_file(
    "intent/.intentfiles",
    &format!("{manifest}STEELTHREAD:ST0001\n"),
  );
  (f, copy)
}

fn named(paths: &[std::path::PathBuf]) -> bool {
  paths.iter().any(|p| p.ends_with(COVER))
}

fn cover_refusal(report: &Report) -> Option<&OrganizeError> {
  report
    .refused
    .iter()
    .find(|e| matches!(e, OrganizeError::CoverDiverged { path, .. } if path.ends_with(COVER)))
}

/// The preview and the act over a kept copy: `diverged` in both, the refusal
/// in both naming the sync, nothing rewritten, and the file byte for byte the
/// copy put back.
fn assert_kept(fx: &Fixture, f: &mut Facade, copy: &str) {
  let preview = f.organize(Mode::Preview).expect("preview");
  assert!(
    named(&preview.diverged),
    "the preview did not name the cover diverged: {:?}",
    preview.diverged
  );
  assert!(
    !named(&preview.rewritten),
    "the preview would rewrite the cover: {:?}",
    preview.rewritten
  );

  let report = f.organize(Mode::Apply).expect("apply");
  assert!(
    !named(&report.rewritten),
    "--apply rewrote the cover: {:?}",
    report.rewritten
  );
  assert!(
    named(&report.diverged),
    "--apply did not name the cover diverged: {:?}",
    report.diverged
  );
  let refusal = cover_refusal(&report)
    .unwrap_or_else(|| panic!("--apply did not refuse the cover: {:?}", report.refused));
  assert!(
    refusal.remedy().contains("intent sync --to-store ST0001"),
    "the refusal does not name the sync that keeps the edit: {}",
    refusal.remedy()
  );
  assert_eq!(
    fx.read(COVER),
    copy,
    "the cover on disk is not the copy put back"
  );
}

#[test]
fn a_hand_edited_cover_put_back_and_re_declared_is_kept() {
  let fx = Fixture::new();
  let (mut f, copy) = put_back(&fx, true, true);
  assert_kept(&fx, &mut f, &copy);
  assert!(
    fx.read(COVER).contains(HAND_LINE),
    "the hand-written line is gone"
  );
}

#[test]
fn an_unedited_copy_over_an_unmoved_store_is_rendered_as_before() {
  let fx = Fixture::new();
  let (mut f, _) = put_back(&fx, false, false);
  let report = f.organize(Mode::Apply).expect("apply");
  assert!(
    named(&report.rewritten) || named(&report.unchanged),
    "the cover was neither rewritten nor found unchanged: {report:?}"
  );
  assert!(
    !named(&report.diverged) && cover_refusal(&report).is_none(),
    "a cover agreeing with the store was held: {report:?}"
  );
}

#[test]
fn an_unedited_copy_over_a_moved_store_is_kept_too() {
  let fx = Fixture::new();
  let (mut f, copy) = put_back(&fx, true, false);
  assert_kept(&fx, &mut f, &copy);
}
