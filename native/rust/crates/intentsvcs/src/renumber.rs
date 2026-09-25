//! Renumbering a steel thread or an issue: the model half (ST0078 WP-02).
//!
//! Ids are minted highest-plus-one over the local canon, so two clones can
//! mint the same one. Git refuses the merge with an add/add conflict, and the
//! repair is to move one side to a free id before merging again. This module
//! moves the id through the model and says what else it moved.
//!
//! **PURE, AND THE SPLIT IS PFIC's.** It reads a model and returns one. The
//! facade owns everything with a side: the realised directory, the canon
//! sidecars, the `.intentfiles` row, the write through the one mutation path,
//! the board writes, and the index read for the prose left alone.
//!
//! **STRUCTURED REFERENCES MOVE; PROSE DOES NOT.** A `related` entry and a
//! board claim are fields whose whole value is the id, so leaving one behind
//! is a dangling reference. A sentence naming the id is authored, and a
//! rewrite could change what somebody wrote about a different thread that
//! happened to share the number. The facade reports each one instead.

use crate::ingest::Canon;
use crate::model::Board;

/// A model with one id moved.
#[derive(Debug, Clone)]
pub struct Renumbered {
  /// The model after the move. Its boards are NOT edited: a board is written
  /// through its own door, so its new claims travel in [`Self::claims`].
  pub canon: Canon,
  /// Each structured reference outside the record itself that now names the
  /// new id, in model order, as `<where>: <old> -> <new>`.
  pub rewritten: Vec<String>,
  /// Every node whose claims name the old id, with its whole new claim list.
  pub claims: Vec<(String, Vec<String>)>,
}

/// Move steel thread `old` to `new`, or `None` when the model holds no `old`.
///
/// Whether `new` is free is the caller's question, because the answer depends
/// on the disk as well as the model.
pub fn thread(canon: &Canon, old: &str, new: &str) -> Option<Renumbered> {
  if !canon.threads.iter().any(|t| t.id == old) {
    return None;
  }
  let mut next = canon.clone();
  let mut rewritten = Vec::new();
  for thread in &mut next.threads {
    if thread.id == old {
      thread.id = new.to_string();
    }
  }
  for thread in &mut next.threads {
    for related in &mut thread.related {
      if related.id == old {
        related.id = new.to_string();
        rewritten.push(format!("{} related: {old} -> {new}", thread.id));
      }
    }
  }
  // Every other door hands the renderers the model in id order, so this does.
  next.threads.sort_by(|a, b| a.id.cmp(&b.id));
  let claims = moved_claims(&canon.boards, old, new, &mut rewritten);
  Some(Renumbered {
    canon: next,
    rewritten,
    claims,
  })
}

/// Move issue `old` to `new`, or `None` when the model holds no `old`.
///
/// **THIS SAID "NOTHING STRUCTURED REFERS TO AN ISSUE -- `related` AND CLAIMS
/// BOTH NAME THREADS -- SO THE ONLY REFERENCES IT CAN HAVE ARE PROSE", AND
/// THAT SENTENCE IS NOW FALSE** (hv, decision 29, 2026-09-22). A BOARD CAN
/// CLAIM AN ISSUE, as `ISSUE:0512`. The sentence is rewritten rather than
/// deleted because it was the stated REASON this function rewrote nothing, and
/// a reason that has stopped holding is the thing a later reader most needs to
/// see move: deleting it quietly would leave the behaviour looking like a
/// choice nobody had revisited.
///
/// `related` still names only threads, so an issue has no `related` edge to
/// move; the claim half is now real and uses the same `moved_claims` the
/// thread side does. **A widening that had left this behind would not have
/// failed** -- it would have silently left a board claiming a number that had
/// moved, which is a dangling reference with nothing to report it.
pub fn issue(canon: &Canon, old: u32, new: u32) -> Option<Renumbered> {
  if !canon.issues.iter().any(|i| i.number == old) {
    return None;
  }
  let mut next = canon.clone();
  for issue in &mut next.issues {
    if issue.number == old {
      issue.number = new;
    }
  }
  next.issues.sort_by_key(|i| i.number);
  // **THROUGH THE MODEL'S OWN FORMATTER, NEVER `format!("ISSUE:{old:04}")`
  // HERE.** The width and the prefix are one fact each, owned by `model`; a
  // second spelling of either in this file agrees until one of them moves.
  let mut rewritten = Vec::new();
  let claims = moved_claims(
    &canon.boards,
    &crate::model::issue_claim(old),
    &crate::model::issue_claim(new),
    &mut rewritten,
  );
  Some(Renumbered {
    canon: next,
    rewritten,
    claims,
  })
}

/// The boards' claims with work package `seq`s moved as `moves` says, and each
/// node whose claims moved with its whole new list (issue 0555). A claim names
/// a package as `<thread>/<NN>`, which [`moved_claims`] matches whole.
pub fn work_package_claims(
  boards: &[Board],
  thread: &str,
  moves: &[(u32, u32)],
) -> (Vec<(String, Vec<String>)>, Vec<String>) {
  let mut boards = boards.to_vec();
  let mut rewritten = Vec::new();
  let mut changed: Vec<String> = Vec::new();
  for (from, to) in moves {
    for (node, claims) in moved_claims(
      &boards,
      &format!("{thread}/{from:02}"),
      &format!("{thread}/{to:02}"),
      &mut rewritten,
    ) {
      if let Some(board) = boards.iter_mut().find(|b| b.node.moniker == node) {
        board.node.claims = claims;
      }
      if !changed.contains(&node) {
        changed.push(node);
      }
    }
  }
  let out = changed
    .into_iter()
    .filter_map(|node| {
      boards
        .iter()
        .find(|b| b.node.moniker == node)
        .map(|b| (node.clone(), b.node.claims.clone()))
    })
    .collect();
  (out, rewritten)
}

/// The claim lists that change, a claim being a thread, one of its packages,
/// or an issue.
///
/// **THE MATCH IS BY WHOLE ADDRESS OR BY ADDRESS-PLUS-`/`, WHICH IS WHY IT
/// TOOK THE ISSUE FORM WITHOUT A LINE OF CHANGE.** `moved_claim` refuses a
/// merely-longer id -- `ST00031` is not `ST0003` -- by requiring a `/` after
/// the prefix, and that guard holds identically for `ISSUE:0051` against
/// `ISSUE:0005`. Issue ids are fixed width besides, so the case cannot arise;
/// the guard is what makes that a belt rather than the only reason.
fn moved_claims(
  boards: &[Board],
  old: &str,
  new: &str,
  rewritten: &mut Vec<String>,
) -> Vec<(String, Vec<String>)> {
  let mut out = Vec::new();
  for board in boards {
    let node = &board.node.moniker;
    let mut moved = false;
    let claims: Vec<String> = board
      .node
      .claims
      .iter()
      .map(|claim| match moved_claim(claim, old, new) {
        Some(to) => {
          rewritten.push(format!("{node}'s claims: {claim} -> {to}"));
          moved = true;
          to
        }
        None => claim.clone(),
      })
      .collect();
    if moved {
      out.push((node.clone(), claims));
    }
  }
  out
}

fn moved_claim(claim: &str, old: &str, new: &str) -> Option<String> {
  if claim == old {
    return Some(new.to_string());
  }
  claim
    .strip_prefix(old)
    .and_then(|rest| rest.strip_prefix('/'))
    .map(|seq| format!("{new}/{seq}"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_package_claim_moves_with_its_thread_and_a_longer_id_does_not() {
    assert_eq!(
      moved_claim("ST0003/02", "ST0003", "ST0004").as_deref(),
      Some("ST0004/02")
    );
    assert_eq!(
      moved_claim("ST0003", "ST0003", "ST0004").as_deref(),
      Some("ST0004")
    );
    assert_eq!(moved_claim("ST00031", "ST0003", "ST0004"), None);
    assert_eq!(moved_claim("ST0005/01", "ST0003", "ST0004"), None);
  }
}
