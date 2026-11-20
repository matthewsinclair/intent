//! What is outstanding, and the one list `intent outstanding` renders (ST0079).
//!
//! **ONE DEFINITION PER KIND, READ BY EVERY VERB THAT LISTS THAT KIND** (hv,
//! 2026-09-21). Outstanding means what the list verbs already show by default:
//! a thread bare `intent st list` shows, an issue bare `intent issues` shows,
//! and a work package in progress under any thread. The first two were literals
//! in intent-cli, one on each verb's CLI face and one on its MCP face; they are
//! [`THREAD_STATUSES`] and [`ISSUE_STATUS`] now, so `outstanding` cannot list a
//! thread `st list` hides, nor `st list` hide one `outstanding` lists.
//!
//! **THE MERGE IS PURE AND RENDERS NOTHING** (PFIC): [`outstanding`] takes the
//! rows the facade read, in the order their own list verbs print them, and
//! does no I/O. The table, `--show` and the counts line are intent-cli's.

use crate::id::IdKind;
use crate::model::{Issue, IssueStatus, Thread, ThreadStatus, WpStatus};
use serde::Serialize;

/// The thread statuses bare `intent st list` shows.
pub const THREAD_STATUSES: &[ThreadStatus] = &[ThreadStatus::Wip];

/// The work-package status that is outstanding, whatever its thread's status.
///
/// No list verb had a default to carry here: `wp list` shows every package of
/// one thread, and `intent todo` buckets on the thread's status alone and nests
/// every package under it. So this one is new, and defined beside the others.
pub const WP_STATUS: WpStatus = WpStatus::Wip;

/// The issue status bare `intent issues` shows.
pub const ISSUE_STATUS: IssueStatus = IssueStatus::Open;

/// One outstanding item. `status` is its type's own display spelling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Row {
  pub kind: IdKind,
  pub id: String,
  pub status: &'static str,
  pub title: String,
  /// A work package's thread and sequence, as fields; `None` on a thread or an
  /// issue, and then absent from the JSON.
  ///
  /// **CARRIED SO NO FACE READS THEM BACK OUT OF `id`** (ST0079 WP-01, vc
  /// 2026-09-22). `id` is the package RENDERED, `STxxxx/NN`, and the explorer
  /// opens a package by its thread and its sequence as the number the `wps`
  /// descent names it by. Re-parsing the rendering would be a second home for
  /// its spelling, which breaks in silence when the rendering changes.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub wp: Option<WpRef>,
}

/// Where a work package sits: its thread, and its sequence under that thread.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WpRef {
  pub thread: crate::id::ThreadId,
  pub seq: crate::id::WpId,
}

/// How many of one kind are outstanding and shown, of how many exist, and the
/// statuses that made them outstanding -- so an empty kind reads as none of N
/// (hv's narrowed-render rule, 2026-08-28, issue 0121) rather than as missing
/// data. A thread shown only as a work package's parent is not outstanding and
/// is not counted here; [`Outstanding::parents`] counts it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Count {
  pub kind: IdKind,
  pub shown: usize,
  pub total: usize,
  pub statuses: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Outstanding {
  pub rows: Vec<Row>,
  pub counts: Vec<Count>,
  /// The threads shown only as a work package's parent: thread rows that no
  /// status of their own put in the list.
  pub parents: usize,
}

/// The outstanding rows of the kinds in `show`, and a count of each kind.
///
/// **EVERY WORK PACKAGE DIRECTLY UNDER ITS OWN THREAD** (hv, 2026-09-22, ST0079
/// WP-02). `threads` arrive as [`crate::facade::Facade::st_list`] returns them,
/// which is the order `st list` prints, and each thread is followed by its WIP
/// packages in sequence order; the issues follow the last thread, as
/// [`crate::facade::Facade::issue_list`] returns them. A package always belongs
/// to a thread, so a WIP package whose thread is not outstanding brings that
/// thread in as its parent row, with the thread's own status, where `st list`
/// orders it. With threads not in `show`, the packages are listed alone, in the
/// same order.
pub fn outstanding(threads: &[&Thread], issues: &[&Issue], show: &[IdKind]) -> Outstanding {
  let with_threads = show.contains(&IdKind::Thread);
  let with_packages = show.contains(&IdKind::WorkPackage);
  let mut rows = Vec::new();
  let (mut open, mut parents, mut packages) = (0, 0, 0);
  for t in threads {
    let wip: Vec<Row> = t
      .wps
      .iter()
      .filter(|w| with_packages && w.status == WP_STATUS)
      .map(|w| Row {
        kind: IdKind::WorkPackage,
        id: crate::id::wp_address(&t.id, w.seq),
        status: w.status.display(),
        title: w.title.clone(),
        wp: Some(WpRef {
          thread: t.id.clone(),
          seq: w.seq,
        }),
      })
      .collect();
    let outstanding = THREAD_STATUSES.contains(&t.status);
    if with_threads && (outstanding || !wip.is_empty()) {
      rows.push(Row {
        kind: IdKind::Thread,
        id: t.id.stored(),
        status: t.status.display(),
        title: t.title.clone(),
        wp: None,
      });
      if outstanding {
        open += 1;
      } else {
        parents += 1;
      }
    }
    packages += wip.len();
    rows.extend(wip);
  }
  let open_issues: Vec<Row> = issues
    .iter()
    .filter(|i| show.contains(&IdKind::Issue) && i.status == ISSUE_STATUS)
    .map(|i| Row {
      kind: IdKind::Issue,
      id: i.number.stored(),
      status: i.status.display(),
      title: i.title.clone(),
      wp: None,
    })
    .collect();
  let issues_shown = open_issues.len();
  rows.extend(open_issues);
  let counts = IdKind::ALL
    .into_iter()
    .filter(|k| show.contains(k))
    .map(|kind| {
      let (shown, total, statuses) = match kind {
        IdKind::Thread => (
          open,
          threads.len(),
          THREAD_STATUSES.iter().map(|s| s.display()).collect(),
        ),
        IdKind::WorkPackage => (
          packages,
          threads.iter().map(|t| t.wps.len()).sum(),
          vec![WP_STATUS.display()],
        ),
        IdKind::Issue => (issues_shown, issues.len(), vec![ISSUE_STATUS.display()]),
      };
      Count {
        kind,
        shown,
        total,
        statuses,
      }
    })
    .collect();
  Outstanding {
    rows,
    counts,
    parents,
  }
}
