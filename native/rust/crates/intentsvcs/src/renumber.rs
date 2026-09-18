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
/// Nothing structured refers to an issue -- `related` and claims both name
/// threads -- so the only references it can have are prose.
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
  Some(Renumbered {
    canon: next,
    rewritten: Vec::new(),
    claims: Vec::new(),
  })
}

/// The claim lists that change, a claim being a thread or one of its packages.
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
