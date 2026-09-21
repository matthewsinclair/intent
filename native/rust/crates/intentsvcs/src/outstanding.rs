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

/// The three kinds, declared in the order the list shows them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
  Thread,
  WorkPackage,
  Issue,
}

impl Kind {
  pub const ALL: [Kind; 3] = [Kind::Thread, Kind::WorkPackage, Kind::Issue];

  /// The type column's spelling, as hv wrote it: ST, WP, Issue.
  pub fn label(self) -> &'static str {
    match self {
      Kind::Thread => "ST",
      Kind::WorkPackage => "WP",
      Kind::Issue => "Issue",
    }
  }

  /// What a count calls this kind.
  pub fn noun(self) -> &'static str {
    match self {
      Kind::Thread => "threads",
      Kind::WorkPackage => "work packages",
      Kind::Issue => "issues",
    }
  }
}

/// One outstanding item. `status` is its type's own display spelling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Row {
  pub kind: Kind,
  pub id: String,
  pub status: &'static str,
  pub title: String,
}

/// How many of one kind are shown, of how many exist, and the statuses that
/// made them outstanding -- so an empty kind reads as none of N (hv's
/// narrowed-render rule, 2026-08-28, issue 0121) rather than as missing data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Count {
  pub kind: Kind,
  pub shown: usize,
  pub total: usize,
  pub statuses: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Outstanding {
  pub rows: Vec<Row>,
  pub counts: Vec<Count>,
}

/// The outstanding rows of the kinds in `show`: threads, then work packages,
/// then issues, whatever order `show` names them in.
///
/// **EACH KIND IN ITS SIBLING'S ORDER.** `threads` and `issues` arrive as
/// [`crate::facade::Facade::st_list`] and [`crate::facade::Facade::issue_list`]
/// return them, which is the order `st list` and `issues` print; work packages
/// follow their threads in that order, each thread's in sequence order.
pub fn outstanding(threads: &[&Thread], issues: &[&Issue], show: &[Kind]) -> Outstanding {
  let mut rows = Vec::new();
  let mut counts = Vec::new();
  for kind in Kind::ALL.into_iter().filter(|k| show.contains(k)) {
    let before = rows.len();
    let (total, statuses) = match kind {
      Kind::Thread => {
        rows.extend(
          threads
            .iter()
            .filter(|t| THREAD_STATUSES.contains(&t.status))
            .map(|t| Row {
              kind,
              id: t.id.clone(),
              status: t.status.display(),
              title: t.title.clone(),
            }),
        );
        (
          threads.len(),
          THREAD_STATUSES.iter().map(|s| s.display()).collect(),
        )
      }
      Kind::WorkPackage => {
        rows.extend(threads.iter().flat_map(|t| {
          t.wps
            .iter()
            .filter(|w| w.status == WP_STATUS)
            .map(move |w| Row {
              kind,
              id: format!("{}/{:02}", t.id, w.seq),
              status: w.status.display(),
              title: w.title.clone(),
            })
        }));
        (
          threads.iter().map(|t| t.wps.len()).sum(),
          vec![WP_STATUS.display()],
        )
      }
      Kind::Issue => {
        rows.extend(
          issues
            .iter()
            .filter(|i| i.status == ISSUE_STATUS)
            .map(|i| Row {
              kind,
              id: format!("{:04}", i.number),
              status: i.status.display(),
              title: i.title.clone(),
            }),
        );
        (issues.len(), vec![ISSUE_STATUS.display()])
      }
    };
    counts.push(Count {
      kind,
      shown: rows.len() - before,
      total,
      statuses,
    });
  }
  Outstanding { rows, counts }
}
