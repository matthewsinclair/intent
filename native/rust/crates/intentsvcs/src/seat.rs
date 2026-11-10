//! Who a session sits as, and who wears the Desk (ST0092 WP-08).
//!
//! **ONE HOME FOR THE IDENTITY READ** (vc's ruling of 2026-11-10 on cc's
//! design note). A session's node is the board whose header records its
//! session id, read through the estate reader's own board read, and a board's
//! mediator is the seat hv is reached through. WP-05's check of who may write
//! as hv calls [`session_node`], and the team profiles extend [`mediator_of`]
//! to read a declared profile. Until then every board is undeclared, and an
//! undeclared board's mediator is the moniker vc, by the convention every
//! board on this estate already keeps.
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
/// through. Every board is undeclared until the team profiles land, so this is
/// [`UNDECLARED_MEDIATOR`].
pub fn mediator_of(_cwd: &Path) -> String {
  UNDECLARED_MEDIATOR.to_string()
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
