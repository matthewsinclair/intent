//! THE BRIEF A SEAT IS TOLD (ST0092 WP-06), and its one renderer.
//!
//! **ONE BRIEF, FOUR FACES.** `wb pickup` prints it ahead of the board, its
//! `--json` and MCP's pickup carry it, and `wb brief` prints it alone, each
//! from [`crate::facade::Facade::seat_brief`] and this module. `claude start`
//! hands the launched session [`SeatBrief::launch`], built by the same call.
//!
//! **THE SEATS ARE THE ROSTER README'S.** A brief is built on
//! [`crate::roster_readme::Roster::of`], so the README and the brief cannot
//! disagree about a seat, its name, role or remit. What a brief adds is the
//! seat's own place: whether it is the mediator, its route to hv and its launch
//! steps, each cut from the base profile by [`crate::teams::section`] for the
//! seat's kind, and whether each other seat has ever run, which
//! [`crate::seat::never_started`] decides at the read.
//!
//! **A BOARD THAT DECLARES NO PROFILE KEEPS ITS LAUNCH.** Its brief names the
//! seat and nothing else, and its launch is the text `intent claude start`
//! composed before this thread, byte for byte, so nothing changes for a project
//! until its own `wb init`.
//!
//! **PURE.** Every function here builds or renders; the facade reads the store
//! and the install.

use serde::Serialize;

use crate::roster_readme::{Roster, RosterSeat};
use crate::teams::{
  Profile, SECTION_LAUNCH_MEDIATOR, SECTION_LAUNCH_SEATS, SECTION_ROUTE_MEDIATOR,
  SECTION_ROUTE_SEATS,
};

/// One seat as a brief lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BriefSeat {
  pub moniker: String,
  pub name: String,
  pub role: String,
  /// The remit the project set, or the profile's default; `None` for a node no
  /// profile names that has no remit of its own, and on an undeclared board.
  pub remit: Option<String>,
  /// Whether `remit` is the profile's default rather than the project's own.
  pub remit_is_default: bool,
  /// Whether no session has ever run in the seat: [`crate::seat::never_started`].
  pub never_started: bool,
}

/// A seat named by its moniker and display name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SeatRef {
  pub moniker: String,
  pub name: String,
}

/// The team a declared board's seat sits on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Team {
  pub base: String,
  pub layers: Vec<String>,
  /// The seat hv is reached through.
  pub mediator: SeatRef,
  /// Whether the seat this brief is for is the mediator.
  pub is_mediator: bool,
  /// The seat the human drives.
  pub human: String,
  /// Every other seat, in the roster's order.
  pub seats: Vec<BriefSeat>,
  /// The base profile's prose for this seat: its opening paragraphs, then its
  /// route to hv for the seat's kind.
  pub prose: String,
}

/// What a seat is told.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SeatBrief {
  pub seat: BriefSeat,
  /// `None` on a board that declares no profile.
  pub team: Option<Team>,
  /// The launch context, when it was asked for: the opening line, the steps
  /// and, on a declared board, the brief and the profile's launch steps.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub launch: Option<String>,
}

/// Who a seat is, from its row: moniker, name, role, remit and whether a
/// session has ever run in it.
pub type SeatRow<'a> = (&'a str, &'a str, &'a str, Option<&'a str>, bool);

impl SeatBrief {
  /// The brief of a seat on a board that declares no profile: its name and
  /// role, and, when `project` is given, the launch it had before this thread.
  pub fn unprofiled(me: SeatRow<'_>, project: Option<&str>) -> Self {
    let (moniker, name, role, remit, never_started) = me;
    let launch = project.map(|p| unprofiled_launch(moniker, name, p));
    Self {
      seat: BriefSeat {
        moniker: moniker.to_string(),
        name: name.to_string(),
        role: role.to_string(),
        remit: remit.map(str::to_string),
        remit_is_default: false,
        never_started,
      },
      team: None,
      launch,
    }
  }

  /// The brief of `me` on a declared board, from the roster an apply leaves,
  /// the base profile and whether each seat has ever run (`never_started`, by
  /// moniker). With `project`, the launch context too.
  pub fn of(
    me: &str,
    roster: &Roster,
    base: &Profile,
    human: &str,
    never_started: impl Fn(&str) -> bool,
    project: Option<&str>,
  ) -> Option<Self> {
    let own = roster.seats.iter().find(|s| s.moniker == me)?;
    let line = |s: &RosterSeat| BriefSeat {
      moniker: s.moniker.clone(),
      name: s.name.clone(),
      role: s.role.clone(),
      remit: s.remit.clone(),
      remit_is_default: s.default_remit,
      never_started: never_started(&s.moniker),
    };
    let is_mediator = roster.mediator == me;
    let route = if is_mediator {
      SECTION_ROUTE_MEDIATOR
    } else {
      SECTION_ROUTE_SEATS
    };
    let prose = [
      preamble(&base.brief),
      base.section(route).unwrap_or_default(),
    ]
    .into_iter()
    .filter(|p| !p.is_empty())
    .collect::<Vec<_>>()
    .join("\n\n");
    let mut brief = Self {
      seat: line(own),
      team: Some(Team {
        base: roster.base.clone(),
        layers: roster.layers.clone(),
        mediator: SeatRef {
          moniker: roster.mediator.clone(),
          name: roster.mediator_name.clone(),
        },
        is_mediator,
        human: human.to_string(),
        seats: roster
          .seats
          .iter()
          .filter(|s| s.moniker != me)
          .map(line)
          .collect(),
        prose,
      }),
      launch: None,
    };
    if let Some(project) = project {
      let steps = if is_mediator {
        SECTION_LAUNCH_MEDIATOR
      } else {
        SECTION_LAUNCH_SEATS
      };
      brief.launch = Some(profiled_launch(
        &brief,
        project,
        &base.section(steps).unwrap_or_default(),
      ));
    }
    Some(brief)
  }

  /// The brief as `wb pickup` prints it ahead of the board and `wb brief`
  /// prints it alone.
  pub fn text(&self) -> String {
    let s = &self.seat;
    let Some(team) = &self.team else {
      return format!(
        "seat: {} ({}), role {}; this board declares no Team Profile",
        s.moniker, s.name, s.role
      );
    };
    let layers = if team.layers.is_empty() {
      String::new()
    } else {
      format!(" + {}", team.layers.join(" + "))
    };
    let mut out = vec![
      format!(
        "seat: {} ({}), role {}, on a {}{layers} team",
        s.moniker, s.name, s.role, team.base
      ),
      format!("  remit: {}", remit_words(s)),
    ];
    if team.is_mediator {
      out.push(
        "  mediator: you are the mediator; you reach hv directly, and the other seats' requests for hv come to you"
          .to_string(),
      );
    } else {
      out.push(format!(
        "  mediator: {} ({}); you reach hv through it",
        team.mediator.moniker, team.mediator.name
      ));
    }
    out.push(format!("  seats ({}):", team.seats.len()));
    for o in &team.seats {
      let mark = if o.never_started {
        ", never started"
      } else {
        ""
      };
      out.push(format!(
        "    {} ({}), {}{mark} -- {}",
        o.moniker,
        o.name,
        o.role,
        remit_words(o)
      ));
    }
    if !team.prose.is_empty() {
      out.push(String::new());
      out.push(team.prose.clone());
    }
    out.join("\n")
  }
}

/// A remit as a brief prints it.
fn remit_words(s: &BriefSeat) -> String {
  match (&s.remit, s.remit_is_default) {
    (Some(r), true) => format!("{r} {}", crate::roster_readme::DEFAULT_MARK),
    (Some(r), false) => r.clone(),
    (None, _) => "none set".to_string(),
  }
}

/// A profile's opening paragraphs: the text before its first `## ` heading,
/// without its `# ` title.
fn preamble(brief: &str) -> String {
  brief
    .lines()
    .take_while(|l| !l.starts_with("## "))
    .filter(|l| !l.starts_with("# "))
    .collect::<Vec<_>>()
    .join("\n")
    .trim()
    .to_string()
}

/// The first step every launch takes: the session loads its skills, which
/// picks it up.
fn first_step(moniker: &str) -> String {
  format!(
    "  1. Run /in-session (loads coding skills, releases the prompt gate, chains /in-whiteboard pickup).\n     If pickup did not identify you, run: /in-whiteboard pickup {moniker}"
  )
}

/// The launch context `intent claude start` composed before this thread, for a
/// board that declares no profile: byte for byte what the launcher printed
/// ahead of the restart context, the session told to show hv its plan and wait.
pub fn unprofiled_launch(moniker: &str, name: &str, project: &str) -> String {
  format!(
    "YOU ARE: {moniker} ({name}) -- a workstream node on the {project} whiteboard (intent/whiteboard/{moniker}/).\n\
     \n\
     On wake, do ALL of the following now, in this one turn, without waiting for further input:\n\
     {}\n  \
     2. Read your inbox(es) under intent/whiteboard/{moniker}/ and act on any opening assignment.\n  \
     3. Claim your steel threads if your assignment names any.\n  \
     4. ULTRATHINK, then show me a detailed plan for today's work for this workstream ({moniker}).\n  \
     5. Then STOP and wait for my instructions.",
    first_step(moniker)
  )
}

/// The launch context of a seat on a declared board: the seat named, the first
/// step, then its brief and the profile's launch steps for its kind.
fn profiled_launch(brief: &SeatBrief, project: &str, steps: &str) -> String {
  let s = &brief.seat;
  format!(
    "YOU ARE: {} ({}) -- the {} seat on the {project} whiteboard (intent/whiteboard/{}/).\n\
     \n\
     On wake, do ALL of the following now, in this one turn, without waiting for further input:\n\
     {}\n  \
     2. Then take your launch steps below.\n\
     \n\
     Your brief:\n\
     {}\n\
     \n\
     Your launch steps:\n\
     {steps}",
    s.moniker,
    s.name,
    s.role,
    s.moniker,
    first_step(&s.moniker),
    brief.text()
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_preamble_drops_the_title_and_stops_at_the_first_section() {
    let brief = "# Title\n\nOne.\n\nTwo.\n\n## Route\n\nNot this.";
    assert_eq!(preamble(brief), "One.\n\nTwo.");
  }

  #[test]
  fn an_unprofiled_brief_names_the_seat_and_launches_as_before() {
    let b = SeatBrief::unprofiled(("cc", "Control Claude", "control", None, false), Some("p"));
    assert!(b.team.is_none());
    assert!(b.text().contains("cc (Control Claude), role control"));
    let launch = b.launch.expect("asked for");
    assert!(
      launch.starts_with("YOU ARE: cc (Control Claude) -- a workstream node on the p whiteboard")
    );
    assert!(launch.ends_with("  5. Then STOP and wait for my instructions."));
  }
}
