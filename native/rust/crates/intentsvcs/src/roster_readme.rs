//! THE ROSTER README, `intent/whiteboard/README.md` (ST0092 WP-07).
//!
//! **A GENERATED VIEW WITH ONE WRITER, `wb init --apply`, THAT KEEPS THE
//! PROJECT'S OWN BLOCK.** Everything above the block is rendered from one
//! template, `lib/templates/whiteboard/_README.md`, filled from the declared
//! profiles and the store. Everything inside the block is the project's and is
//! carried by [`crate::canon::carry_user_block`], the function that carries
//! `CLAUDE.md`'s block, and by no copy of it.
//!
//! **A RECORDED HASH TELLS AN EDIT FROM A NEWER RENDER.** Each render records,
//! on the board's declaration (`readme_baseline`), the sha256 of the text
//! outside the block as it rendered it. Text outside the block that no longer
//! hashes to that is an edit, which an apply refuses and `intent doctor`
//! reports as `roster-readme-edited`. Text that does but is not this build's
//! render is only out of date, `roster-readme-behind`, and an apply
//! re-renders it. A README with no recorded hash was never rendered, and its
//! first render puts the whole file inside the block.
//!
//! **PURE.** Every function here decides; `wb init --apply` writes what was
//! decided, and doctor asks the same question of the same inputs.

use crate::canon::{USER_END, USER_START};
use crate::project::Config;
use crate::teams::{Driver, Resolved};
use crate::views::RenderContext;

/// The template, under the install's templates.
pub const TEMPLATE: &str = "whiteboard/_README.md";

/// The file, under the whiteboard directory.
pub const FILE: &str = "README.md";

/// How a remit taken from the profile is marked in the roster table.
pub const DEFAULT_MARK: &str = "(the profile's default)";

const PROFILES_TOKEN: &str = "[[ROSTER_PROFILES]]";
const TABLE_TOKEN: &str = "[[ROSTER_TABLE]]";
const ROUTE_TOKEN: &str = "[[ROSTER_ROUTE]]";

/// What a seat's driver column says for a registered node no declared profile
/// names: the profiles declare no driver for it, and nothing is guessed.
const UNDECLARED: &str = "not in the profiles";

/// One row of the roster table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RosterSeat {
  pub moniker: String,
  pub name: String,
  pub role: String,
  pub driver: String,
  pub remit: String,
}

/// Everything the generated part says, each from the profiles or the store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roster {
  pub base: String,
  pub layers: Vec<String>,
  pub mediator: String,
  pub mediator_name: String,
  pub seats: Vec<RosterSeat>,
  /// The base profile's route to hv for every seat but the mediator, whole.
  pub route: String,
}

/// One registered node as the roster reads it: moniker, name, role, and the
/// remit the project set.
pub type Registered<'a> = (&'a str, &'a str, &'a str, Option<&'a str>);

impl Roster {
  /// The roster an apply leaves: each declared seat with the profile's name and
  /// role, its driver from the declared human seat or the registry, and the
  /// remit the project set or the profile's default, marked; then each
  /// registered node no profile names, as the store holds it.
  pub fn of(resolved: &Resolved, registered: &[Registered<'_>], base_brief: &str) -> Self {
    let mut seats: Vec<RosterSeat> = resolved
      .seats
      .iter()
      .map(|seat| {
        let own = registered
          .iter()
          .find(|(m, ..)| *m == seat.moniker)
          .and_then(|(.., remit)| *remit);
        let driver = if seat.moniker == resolved.human {
          Driver::Human
        } else {
          seat.driver
        };
        RosterSeat {
          moniker: seat.moniker.clone(),
          name: seat.name.clone(),
          role: seat.role.clone(),
          driver: driver.phrase().to_string(),
          remit: own.map_or_else(|| format!("{} {DEFAULT_MARK}", seat.remit), str::to_string),
        }
      })
      .collect();
    for (moniker, name, role, remit) in registered {
      if !resolved.seats.iter().any(|s| s.moniker == *moniker) {
        seats.push(RosterSeat {
          moniker: (*moniker).to_string(),
          name: (*name).to_string(),
          role: (*role).to_string(),
          driver: UNDECLARED.to_string(),
          remit: remit.unwrap_or(UNDECLARED).to_string(),
        });
      }
    }
    // The mediator as the declaration names it, with the name the store holds
    // for it, or the profile's where it is not registered yet.
    let mediator_name = registered
      .iter()
      .find(|(m, ..)| *m == resolved.mediator)
      .map(|(_, name, ..)| (*name).to_string())
      .or_else(|| {
        resolved
          .seats
          .iter()
          .find(|s| s.moniker == resolved.mediator)
          .map(|s| s.name.clone())
      })
      .unwrap_or_default();
    Self {
      base: resolved.base.clone(),
      layers: resolved.layers.clone(),
      mediator: resolved.mediator.clone(),
      mediator_name,
      seats,
      route: section(base_brief, crate::teams::SECTION_ROUTE_SEATS).unwrap_or_default(),
    }
  }
}

/// The body of the `## ` section `heading` opens in `brief`, trimmed: every
/// line after the heading up to the next `## ` heading.
fn section(brief: &str, heading: &str) -> Option<String> {
  let mut lines = brief.lines().skip_while(|l| l.trim_end() != heading);
  lines.next()?;
  let body: Vec<&str> = lines.take_while(|l| !l.starts_with("## ")).collect();
  Some(body.join("\n").trim().to_string())
}

/// The roster table, its columns padded to their widest cell as the project's
/// formatter pads a Markdown table, so a formatter run changes nothing in it.
pub fn table(seats: &[RosterSeat]) -> String {
  let cell = |s: &str| s.replace('|', "\\|");
  let header = ["Moniker", "Name", "Role", "Driven by", "Remit"].map(str::to_string);
  let rows: Vec<[String; 5]> = seats
    .iter()
    .map(|s| {
      [
        cell(&s.moniker),
        cell(&s.name),
        cell(&s.role),
        cell(&s.driver),
        cell(&s.remit),
      ]
    })
    .collect();
  let mut widths = [3usize; 5];
  for row in std::iter::once(&header).chain(rows.iter()) {
    for (w, c) in widths.iter_mut().zip(row) {
      *w = (*w).max(c.chars().count());
    }
  }
  let line = |row: &[String; 5]| {
    let cells: Vec<String> = row
      .iter()
      .zip(widths)
      .map(|(c, w)| format!("{c}{}", " ".repeat(w - c.chars().count())))
      .collect();
    format!("| {} |", cells.join(" | "))
  };
  let rule = format!(
    "| {} |",
    widths
      .iter()
      .map(|w| "-".repeat(*w))
      .collect::<Vec<_>>()
      .join(" | ")
  );
  let mut out = vec![line(&header), rule];
  out.extend(rows.iter().map(line));
  out.join("\n")
}

/// The profiles line: the base, then each layer.
fn profiles(roster: &Roster) -> String {
  let layers = match roster.layers.as_slice() {
    [] => String::new(),
    layers => format!(
      ", with the layers {}",
      layers
        .iter()
        .map(|l| format!("`{l}`"))
        .collect::<Vec<_>>()
        .join(", ")
    ),
  };
  format!(
    "This whiteboard declares the base profile `{}`{layers}. `intent wb profile show <name>` prints a profile.",
    roster.base
  )
}

/// The route to hv: the mediator named as the one reader of hv's inbox, then
/// the base profile's route for every other seat, printed whole.
fn route(roster: &Roster) -> String {
  format!(
    "hv is reached through the mediator, `{}` ({}), the one reader of hv's inbox. What the profile asks of every other seat:\n\n{}",
    roster.mediator, roster.mediator_name, roster.route
  )
}

/// The README as this build renders it, with an empty block.
pub fn render(
  template: &str,
  roster: &Roster,
  cfg: &Config,
  ctx: &RenderContext<'_>,
) -> Result<String, crate::rootfiles::Fault> {
  let text = template
    .replace(PROFILES_TOKEN, &profiles(roster))
    .replace(TABLE_TOKEN, &table(&roster.seats))
    .replace(ROUTE_TOKEN, &route(roster));
  crate::rootfiles::substitute(&text, cfg, ctx)
}

/// What is wrong with a README's block markers, naming the line.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MarkerFault {
  #[error("line {line} holds a second `{marker}`, where the README has one")]
  Second { marker: &'static str, line: usize },
  #[error("line {line} holds `{USER_END}` before any `{USER_START}`")]
  EndFirst { line: usize },
  #[error("it holds no `{marker}`, where the README's block opens and closes")]
  Missing { marker: &'static str },
  #[error(
    "line {line} holds `{marker}`, and a README Intent never rendered may not: its first render puts the whole file inside the block"
  )]
  BeforeFirstRender { marker: &'static str, line: usize },
}

/// Each line, numbered from 1, that holds `marker`, once for each time it does.
fn marker_lines(text: &str, marker: &str) -> Vec<usize> {
  text
    .lines()
    .enumerate()
    .flat_map(|(i, l)| std::iter::repeat_n(i + 1, l.matches(marker).count()))
    .collect()
}

/// The block's span, strictly between its markers, as byte offsets, once the
/// markers are one start followed by one end.
fn span(text: &str) -> Result<(usize, usize), MarkerFault> {
  let starts = marker_lines(text, USER_START);
  let ends = marker_lines(text, USER_END);
  if let Some(line) = starts.get(1) {
    return Err(MarkerFault::Second {
      marker: USER_START,
      line: *line,
    });
  }
  if let Some(line) = ends.get(1) {
    return Err(MarkerFault::Second {
      marker: USER_END,
      line: *line,
    });
  }
  let start = text
    .find(USER_START)
    .ok_or(MarkerFault::Missing { marker: USER_START })?;
  let end = text
    .find(USER_END)
    .ok_or(MarkerFault::Missing { marker: USER_END })?;
  if end < start {
    return Err(MarkerFault::EndFirst {
      line: ends.first().copied().unwrap_or_default(),
    });
  }
  Ok((start + USER_START.len(), end))
}

/// The text outside the block, markers kept.
fn outside(text: &str, (start, end): (usize, usize)) -> String {
  format!("{}{}", &text[..start], &text[end..])
}

/// The hash a render records: the sha256 of its text outside the block.
pub fn baseline(text: &str) -> Result<String, MarkerFault> {
  Ok(crate::sync::sha256_of(
    outside(text, span(text)?).as_bytes(),
  ))
}

/// What a render does to the README.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
  /// There is no README, and the render writes one.
  Create,
  /// The README was never rendered, and the whole of it goes inside the
  /// block. `dropped` is the frontmatter block that opened it, which is not
  /// carried.
  First { dropped: Option<String> },
  /// The README is this build's render behind, and is rendered again.
  Rerender,
  /// The README is already this build's render.
  Unchanged,
}

/// A render decided: what it does, the text it writes, and the hash it records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
  pub plan: Plan,
  pub text: String,
  pub baseline: String,
}

/// Why a render refuses. Nothing is written.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ReadmeError {
  #[error("intent/whiteboard/README.md: {0}")]
  Markers(MarkerFault),
  #[error(
    "intent/whiteboard/README.md was edited outside its block since it was rendered: {}",
    edited_lines(.lines)
  )]
  Edited { lines: Vec<(usize, String)> },
}

fn edited_lines(lines: &[(usize, String)]) -> String {
  if lines.is_empty() {
    return "lines the render wrote were removed".to_string();
  }
  lines
    .iter()
    .map(|(n, text)| format!("line {n} (`{text}`)"))
    .collect::<Vec<_>>()
    .join(", ")
}

impl crate::remedy::Remedy for MarkerFault {
  fn remedy(&self) -> String {
    match self {
      Self::BeforeFirstRender { .. } => format!(
        "take the marker out of the README, or write it another way, then run `intent wb init --apply` again: a first render puts the whole file between `{USER_START}` and `{USER_END}`, and a marker already in it would end the block early. Nothing was written."
      ),
      Self::Second { .. } | Self::EndFirst { .. } | Self::Missing { .. } => format!(
        "leave one `{USER_START}` followed by one `{USER_END}` in the README, with the project's own notes between them, then run `intent wb init --apply` again. Nothing was written."
      ),
    }
  }
}

impl crate::remedy::Remedy for ReadmeError {
  fn remedy(&self) -> String {
    match self {
      Self::Markers(fault) => crate::remedy::Remedy::remedy(fault),
      Self::Edited { .. } => format!(
        "move those lines inside the block, between `{USER_START}` and `{USER_END}`, where every render keeps them, and put the generated text back as `git diff` shows it, then run `intent wb init --apply` again. Nothing was written."
      ),
    }
  }
}

/// Split a frontmatter block that opens `text` from the rest: the block, if
/// there is one, and the text after it with its leading blank lines.
fn frontmatter(text: &str) -> (Option<String>, &str) {
  let Some(rest) = text.strip_prefix("---\n") else {
    return (None, text);
  };
  let Some(close) = rest
    .find("\n---\n")
    .map(|at| at + 1)
    .or_else(|| rest.starts_with("---\n").then_some(0))
  else {
    return (None, text);
  };
  let after = &rest[close + 4..];
  let block = format!("---\n{}---", &rest[..close]);
  (Some(block), after.trim_start_matches('\n'))
}

/// Decide a render of the README over what is on disk.
///
/// `existing` is the file, `recorded` the hash the last render recorded, and
/// `rendered` this build's render with an empty block.
pub fn decide(
  existing: Option<&str>,
  recorded: Option<&str>,
  rendered: &str,
) -> Result<Decision, ReadmeError> {
  let decided = |plan, text: String| -> Result<Decision, ReadmeError> {
    let baseline = baseline(&text).map_err(ReadmeError::Markers)?;
    Ok(Decision {
      plan,
      text,
      baseline,
    })
  };
  let Some(existing) = existing else {
    return decided(Plan::Create, rendered.to_string());
  };
  let Some(recorded) = recorded else {
    for marker in [USER_START, USER_END] {
      if let Some(line) = marker_lines(existing, marker).first() {
        return Err(ReadmeError::Markers(MarkerFault::BeforeFirstRender {
          marker,
          line: *line,
        }));
      }
    }
    let (dropped, rest) = frontmatter(existing);
    let rest = if rest.is_empty() || rest.ends_with('\n') {
      rest.to_string()
    } else {
      format!("{rest}\n")
    };
    let wrapped = format!("{USER_START}\n{rest}{USER_END}\n");
    return decided(
      Plan::First { dropped },
      crate::canon::carry_user_block(&wrapped, rendered),
    );
  };
  let held = span(existing).map_err(ReadmeError::Markers)?;
  let theirs = outside(existing, held);
  let rendered_outside = span(rendered)
    .map(|at| outside(rendered, at))
    .unwrap_or_default();
  // **AN EDIT IS TEXT OUTSIDE THE BLOCK THAT MATCHES NEITHER THE RECORDED
  // RENDER NOR THIS BUILD'S.** Text that already is this build's render was
  // written by a render, whatever the baseline says: an apply whose README
  // landed and whose store did not leaves exactly that, and the next apply
  // heals it by recording it (vc's ruling on ic's read of WP-07).
  if crate::sync::sha256_of(theirs.as_bytes()) != recorded && theirs != rendered_outside {
    // The lines outside the block this build would not write, by number: each
    // line's byte offset is the running sum of the lines before it.
    let ours: std::collections::BTreeSet<&str> = rendered_outside.lines().collect();
    let mut offset = 0usize;
    let mut lines = Vec::new();
    for (i, line) in existing.lines().enumerate() {
      let inside = offset >= held.0 && offset < held.1;
      if !inside && !ours.contains(line) {
        lines.push((i + 1, line.to_string()));
      }
      offset += line.len() + 1;
    }
    return Err(ReadmeError::Edited { lines });
  }
  let text = crate::canon::carry_user_block(existing, rendered);
  let plan = if text == existing {
    Plan::Unchanged
  } else {
    Plan::Rerender
  };
  decided(plan, text)
}

/// Where a README stands against this build's render, for `intent doctor`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Standing {
  /// It is this build's render.
  Current,
  /// It is behind this build's render, and an apply re-renders it.
  Behind,
  /// It was edited outside its block, or its markers are wrong.
  Edited(ReadmeError),
  /// It was never rendered: nothing to judge.
  NeverRendered,
}

/// Doctor's question: the same decision an apply would make, read as a
/// standing.
pub fn standing(existing: Option<&str>, recorded: Option<&str>, rendered: &str) -> Standing {
  if recorded.is_none() {
    return Standing::NeverRendered;
  }
  match decide(existing, recorded, rendered) {
    Ok(Decision {
      plan: Plan::Unchanged,
      ..
    }) => Standing::Current,
    Ok(_) => Standing::Behind,
    Err(e) => Standing::Edited(e),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_frontmatter_block_that_opens_the_file_is_split_from_the_rest() {
    let (dropped, rest) = frontmatter("---\nverblock: x\n---\n\n# Ours\n");
    assert_eq!(dropped.as_deref(), Some("---\nverblock: x\n---"));
    assert_eq!(rest, "# Ours\n");
    assert_eq!(frontmatter("# Ours\n---\n"), (None, "# Ours\n---\n"));
  }

  #[test]
  fn the_markers_must_be_one_start_then_one_end() {
    let ok = format!("a\n{USER_START}\nb\n{USER_END}\n");
    assert!(span(&ok).is_ok());
    let twice = format!("{ok}{USER_START}\n");
    assert_eq!(
      span(&twice),
      Err(MarkerFault::Second {
        marker: USER_START,
        line: 5
      })
    );
    let reversed = format!("{USER_END}\n{USER_START}\n");
    assert_eq!(span(&reversed), Err(MarkerFault::EndFirst { line: 1 }));
  }

  #[test]
  fn a_readme_that_is_this_builds_render_is_rendered_whatever_the_baseline_says() {
    let rendered = format!("# Roster\n\n{USER_START}\n{USER_END}\n");
    let ours = format!("# Roster\n\n{USER_START}\nour notes\n{USER_END}\n");
    let stale =
      baseline("# Older\n\n<!-- user:start -->\n<!-- user:end -->\n").expect("a baseline");
    let decided = decide(Some(&ours), Some(&stale), &rendered).expect("not an edit");
    assert_eq!(decided.plan, Plan::Unchanged);
    assert_eq!(decided.text, ours);
    assert_ne!(
      decided.baseline, stale,
      "the heal records this render's hash"
    );

    let edited = format!("# Ours now\n\n{USER_START}\n{USER_END}\n");
    assert!(matches!(
      decide(Some(&edited), Some(&stale), &rendered),
      Err(ReadmeError::Edited { .. })
    ));
  }

  #[test]
  fn the_table_pads_every_column_to_its_widest_cell() {
    let seats = [RosterSeat {
      moniker: "vc".into(),
      name: "Validation Claude".into(),
      role: "validation".into(),
      driver: "a session".into(),
      remit: "a | b".into(),
    }];
    let t = table(&seats);
    let widths: std::collections::BTreeSet<usize> = t.lines().map(|l| l.chars().count()).collect();
    assert_eq!(widths.len(), 1, "{t}");
    assert!(t.contains("a \\| b"), "{t}");
  }
}
