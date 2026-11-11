//! Who a session sits as, and who wears the Desk (ST0092 WP-08).
//!
//! **ONE HOME FOR THE IDENTITY READ** (vc's ruling of 2026-11-10 on cc's
//! design note). A session's node is the board whose header records its
//! session id, read through the estate reader's own board read, and a board's
//! mediator is the seat hv is reached through. WP-05's check of who may write
//! as hv calls [`session_node`], and [`mediator_given`] reads a board's declared
//! mediator (ST0092 WP-05), which `wb init` resolved from its profiles. An
//! undeclared board's mediator is the moniker vc, by the convention every board
//! on this estate already keeps.
//!
//! **THE DESK IS READ, NEVER STORED** (AC-08.3, as vc reworded it). Who wears
//! it is the host's control record, under the user's state dir, and whether
//! that controller's session is still running is the same pid-and-start check
//! `control take` makes. Nothing here writes, so no second record can go stale.
use std::path::Path;

use crate::estate::slots::{ControllerRecord, Slots};

/// An undeclared board's mediator.
pub const UNDECLARED_MEDIATOR: &str = "vc";

/// The node a session sits as: the moniker of the board, in the project the
/// session's `cwd` stands in (folded out of a worktree), whose header records
/// `session_id`. `None` for a session in no project, or bound to no board.
pub fn session_node(cwd: &Path, session_id: &str) -> Option<String> {
  let mut ignored = Vec::new();
  crate::estate::project_at(cwd, &mut ignored)?
    .boards
    .into_iter()
    .find(|b| b.session_id.as_deref() == Some(session_id))
    .map(|b| b.moniker)
}

/// The mediator of the board in the project at `cwd`: the seat hv is reached
/// through, read from the board's team declaration (ST0092 WP-05), and
/// [`UNDECLARED_MEDIATOR`] on a board that declares none.
///
/// A declaration this build cannot read answers as undeclared HERE, and is not
/// swallowed: `intent doctor` names the file as `schema-invalid`, and every
/// write that would rely on it reads the store's row instead.
pub fn mediator_of(cwd: &Path) -> String {
  let declared = crate::project::Project::discover(cwd)
    .ok()
    .and_then(|project| crate::ingest::declaration_on_disk(&project).ok())
    .and_then(Result::ok)
    .flatten();
  mediator_given(declared.as_ref())
}

/// The mediator a declaration names, which `wb init` resolved from its
/// profiles (the last layer naming one, else the base), or
/// [`UNDECLARED_MEDIATOR`] where there is none. **THE ONE RULE**: the facade
/// asks it with the store's row, [`mediator_of`] with the file's.
pub fn mediator_given(declaration: Option<&crate::model::TeamDeclaration>) -> String {
  declaration.map_or_else(|| UNDECLARED_MEDIATOR.to_string(), |d| d.mediator.clone())
}

/// The board's human seat: the one its declaration names, or, on a board that
/// declares no profile, [`crate::model::HYPERVISOR`] by convention, as its
/// mediator is [`UNDECLARED_MEDIATOR`]. No install is read. The one home for
/// the question (vc's ruling across ST0092 WP-04 and WP-06): any other moniker
/// is session-driven.
pub fn human_seat(declaration: Option<&crate::model::TeamDeclaration>) -> String {
  declaration.map_or_else(|| crate::model::HYPERVISOR.to_string(), |d| d.human.clone())
}

/// Whether no session has ever run in `node`'s seat (ST0092 AC-06.2): its row
/// records no session and it is not the `human` seat, which no session drives.
/// **READ FROM THE ABSENCE OF A RECORDED SESSION, NEVER FROM STATUS OR
/// HEARTBEAT**: `wb register` stamps a heartbeat at registration, so a seat that
/// never ran and one that ran and paused read alike by those. Computed at each
/// read and stored nowhere, so it cannot be stale.
pub fn never_started(node: &crate::model::WbNode, human: &str) -> bool {
  node.moniker != human && node.session_id.is_none()
}

/// Where a session sits: its node and that board's mediator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seat {
  pub node: String,
  pub mediator: String,
}

impl Seat {
  /// Whether the seat is its board's mediator.
  pub fn mediates(&self) -> bool {
    self.node == self.mediator
  }
}

/// The seat of a session whose `cwd` and id are known, or `None` when it is
/// bound to no node.
pub fn seat_of(cwd: &Path, session_id: &str) -> Option<Seat> {
  let node = session_node(cwd, session_id)?;
  Some(Seat {
    mediator: mediator_of(cwd),
    node,
  })
}

/// Who wears the Desk on this host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Desk {
  /// No controller was ever recorded, or the last one released.
  Free,
  /// A controller holds it and its session is running.
  Worn(ControllerRecord),
  /// The recorded controller has not released and its session is gone.
  Orphaned(ControllerRecord),
}

/// The step that takes an orphaned desk, as `estate status` prints it.
pub const TAKE_STEP: &str = "intent estate control take --hv \"<hv's words>\"";

/// The Desk, from the control record and whether the controller's session is
/// running: `Some(false)` when it is gone, `None` when liveness could not be
/// read.
///
/// **A LIVENESS READ THAT CANNOT ANSWER LEAVES THE DESK WORN.** Calling it
/// orphaned on a failed read would print a take step over a live holder. The
/// signal is the session's pid and its recorded start time, the check `control
/// take` makes, never a heartbeat, so a session between heartbeats is never
/// read gone.
pub fn desk_given(slots: &Slots, controller_live: impl FnOnce(&str) -> Option<bool>) -> Desk {
  let Some(current) = slots.live_controller() else {
    return Desk::Free;
  };
  let record = current.record.clone();
  match controller_live(&record.session_id) {
    Some(false) => Desk::Orphaned(record),
    Some(true) | None => Desk::Worn(record),
  }
}

/// [`desk_given`], with the controller's session read from the session files
/// under `claude_dir`.
pub fn desk(slots: &Slots, claude_dir: &Path) -> Desk {
  desk_given(slots, |id| {
    crate::estate::slots::session_live(claude_dir, id).ok()
  })
}

/// Whether `session_id` wears the Desk.
pub fn wears(desk: &Desk, session_id: &str) -> bool {
  matches!(desk, Desk::Worn(r) if r.session_id == session_id)
}

/// The hats a node wears now, read from the desk: `desk` when `session`, the
/// session its board records, wears it (ST0092 AC-08.3). WP-06's row per seat
/// extends this list and replaces nothing.
pub fn hats(desk: &Desk, session: Option<&str>) -> Vec<String> {
  match session {
    Some(s) if wears(desk, s) => vec!["desk".to_string()],
    _ => Vec::new(),
  }
}

/// What a pickup says about the desk (ST0092 AC-08.3): absent when the session
/// does not wear it. One answer for the CLI's text and `--json` and for MCP,
/// joined here and rendered there.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "state", rename_all = "lowercase")]
pub enum PickupDesk {
  /// The session wears the desk: the duties from their one template, the asks
  /// waiting for its judgement oldest first, and any request file that could
  /// not be read, listed rather than dropped.
  Wears {
    duties: String,
    asks: Vec<crate::estate::slots::AskRecord>,
    unreadable: Vec<crate::estate::Unreadable>,
  },
  /// The desk could not be read, so nobody can say whether the session wears
  /// it. Said, never taken for no desk.
  Unreadable { why: String },
}

/// The estate reader's `controller_orphaned`: null, or the recorded
/// controller whose session is gone and the step that takes the desk
/// (ST0092 AC-08.4).
pub fn orphaned_value(desk: &Desk) -> serde_json::Value {
  match desk {
    Desk::Orphaned(r) => serde_json::json!({
      "session_id": r.session_id,
      "name": r.name,
      "node": r.node,
      "take": TAKE_STEP,
    }),
    Desk::Free | Desk::Worn(_) => serde_json::Value::Null,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::estate::slots::Stamped;

  fn held(released: bool) -> Slots {
    Slots {
      controller: Some(Stamped {
        record: ControllerRecord {
          schema: crate::estate::slots::SLOT_SCHEMA.to_string(),
          session_id: "s-vc".to_string(),
          name: "intent-vc".to_string(),
          pid: 7,
          node: Some("vc".to_string()),
          predecessor: None,
          hv: None,
          released,
        },
        at: None,
      }),
      ..Slots::default()
    }
  }

  /// vc's caution on AC-08.4: liveness is the session's pid and start, so a
  /// session that cannot be judged (a failed read) is never called gone, and
  /// only a session read as not running orphans the desk.
  #[test]
  fn only_a_session_read_as_gone_orphans_the_desk_and_an_unreadable_one_keeps_it_worn() {
    let slots = held(false);
    assert!(matches!(desk_given(&slots, |_| Some(true)), Desk::Worn(_)));
    assert!(matches!(desk_given(&slots, |_| None), Desk::Worn(_)));
    let orphaned = desk_given(&slots, |_| Some(false));
    assert!(matches!(orphaned, Desk::Orphaned(_)));
    assert_eq!(orphaned_value(&orphaned)["take"], TAKE_STEP);
    assert!(
      !wears(&orphaned, "s-vc"),
      "an orphaned desk is worn by nobody"
    );
  }

  #[test]
  fn a_released_or_absent_controller_is_a_free_desk_whatever_the_liveness() {
    assert_eq!(desk_given(&held(true), |_| Some(true)), Desk::Free);
    assert_eq!(desk_given(&Slots::default(), |_| Some(true)), Desk::Free);
    assert!(orphaned_value(&Desk::Free).is_null());
  }

  #[test]
  fn a_seat_mediates_only_as_its_boards_mediator() {
    let seat = |node: &str| Seat {
      node: node.to_string(),
      mediator: UNDECLARED_MEDIATOR.to_string(),
    };
    assert!(seat("vc").mediates());
    assert!(!seat("cc").mediates());
  }
}
