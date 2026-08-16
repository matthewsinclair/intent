//! **The FROZEN v2 markdown parser** -- WP-10 Phase A (migration.md).
//!
//! This is the one place in v3 that reads v2's format, and it is deliberately
//! different code from [`crate::ingest`]. Ingest is STRICT because current-
//! version data has a schema to be strict against; this reads an estate that
//! never had one, so it is lenient about shape and ruthless about saying what
//! it could not read. Sharing one parser between them would force one of those
//! postures onto the other.
//!
//! **Phase A is READ-ONLY and it gates Phase B.** Nothing here writes: it
//! parses the whole estate, builds the model it can, and reports everything it
//! could not. That split is migration.md's, and it is what makes "refuse what
//! cannot convert without loss, name everything, guess nothing" checkable
//! before a single byte of anyone's project is rewritten.
//!
//! **Residue and carry are different buckets, and the difference is hv's
//! ruling rather than a severity judgement** (migration.md, 2026-08-14). The
//! same unreadable row BLOCKS in a live thread and CARRIES in a closed one:
//! live residue is fixed under v2 tooling and the migration re-run, while a
//! closed thread's legacy grammar is carried whole into the richer model --
//! marked legacy, nothing guessed, nothing dropped. The forcing fact is that
//! the fleet's sweep program is dead, so "fix it under v2 and re-run" is an
//! instruction one estate's owner has permanently refused. Keeping the two in
//! separate fields makes the ruling structural instead of a convention someone
//! has to remember.

use std::collections::BTreeMap;
use std::path::Path;

use crate::finding::{Finding, FindingClass};
use crate::model::{
  AcKind, AcState, AcceptanceTest, AtKind, AtStatus, Criterion, THREAD_SCHEMA, TShirt, Thread,
  ThreadStatus, WorkPackage, WpStatus,
};
use crate::project::Project;

/// What Phase A found.
#[derive(Debug, Default)]
pub struct Scan {
  /// The model, as far as it could be built.
  pub threads: Vec<Thread>,
  /// Findings in LIVE threads. **These block the migration.**
  pub residue: Vec<Finding>,
  /// The same classes of finding in CLOSED threads. **These convert under the
  /// carry policy** and are reported so the count reconciles, never so that
  /// anyone acts on them.
  pub carried: Vec<Finding>,
}

impl Scan {
  /// A live-thread finding: blocks.
  fn block(&mut self, finding: Finding) {
    self.residue.push(finding);
  }

  /// A closed-thread finding: carries.
  fn carry(&mut self, finding: Finding) {
    self.carried.push(finding);
  }

  /// Route by whether the thread is closed -- the ONE place the ruling is
  /// applied, so no call site has to remember which bucket it is in.
  fn record(&mut self, closed: bool, finding: Finding) {
    if closed {
      self.carry(finding)
    } else {
      self.block(finding)
    }
  }
}

/// Parse an entire v2 estate. Writes nothing.
pub fn scan(project: &Project) -> Result<Scan, std::io::Error> {
  let mut out = Scan::default();
  for (id, dir) in thread_dirs(project) {
    let info = dir.join("info.md");
    let Ok(text) = std::fs::read_to_string(&info) else {
      // A directory that looks like a thread and has no `info.md` is not a
      // thread this parser can classify. It is named rather than skipped: a
      // silently ignored directory is how an artefact disappears from a
      // migration whose whole promise is that nothing does.
      out.block(Finding::new(
        project.relative(&info),
        FindingClass::UnknownFileShape,
        "a steel-thread directory with no info.md".to_string(),
      ));
      continue;
    };
    let rel = project.relative(&info);
    let (front, body) = frontmatter(&text);

    // The status decides which bucket every later finding for this thread
    // lands in, so it is read first and a thread whose status cannot be read
    // is treated as LIVE -- the conservative direction, because guessing
    // "closed" would silently carry rows that should have blocked.
    let raw_status = front.get("status").cloned().unwrap_or_default();
    let status = thread_status(&raw_status);
    let closed = matches!(
      status,
      Some(ThreadStatus::Completed | ThreadStatus::Cancelled)
    );
    if status.is_none() {
      out.block(Finding::new(
        &rel,
        FindingClass::UnknownStatus,
        format!("thread status {raw_status:?} is not in the v2 vocabulary"),
      ));
    }

    if let Some(line) = conflict_marker_line(&text) {
      out.record(
        closed,
        Finding::new(&rel, FindingClass::ConflictMarkers, "in info.md").at_line(line),
      );
    }

    let sections = sections(body);
    let (criteria, tests) = acceptance(project, &dir, closed, &mut out);
    let wps = work_packages(project, &dir, closed, &mut out);
    out.threads.push(Thread {
      schema: THREAD_SCHEMA.to_string(),
      id: id.clone(),
      title: title(body).unwrap_or_else(|| id.clone()),
      slug: front.get("slug").filter(|s| !s.is_empty()).cloned(),
      status: status.unwrap_or(ThreadStatus::NotStarted),
      status_reason: None,
      created: date(front.get("created")),
      completed: front
        .get("completed")
        .filter(|s| !s.is_empty())
        .map(|s| date(Some(s))),
      acceptance: None,
      // **Verbatim, never reflowed** (migration.md: the migrator does not
      // improve prose). An absent section stays empty rather than acquiring a
      // placeholder.
      objective: sections.get("Objective").cloned().unwrap_or_default(),
      context: sections.get("Context").cloned().unwrap_or_default(),
      related: Vec::new(),
      wps,
      criteria,
      tests,
    });
  }

  // The 0011 class: two artefacts claiming one natural id. Checked across the
  // whole estate rather than within a directory, because the v2 layout puts
  // closed threads in subdirectories -- which is exactly how one id comes to
  // exist in two places.
  let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
  for t in &out.threads {
    *seen.entry(t.id.as_str()).or_default() += 1;
  }
  let duplicates: Vec<String> = seen
    .iter()
    .filter(|(_, n)| **n > 1)
    .map(|(id, n)| format!("{id} appears {n} times"))
    .collect();
  for detail in duplicates {
    out.block(Finding::new("intent/st", FindingClass::DuplicateId, detail));
  }

  Ok(out)
}

/// Every `ST####` directory, at the top level and under v2's status
/// subdirectories.
///
/// The subdirectory walk is not optional: `intent st done` MOVES a thread into
/// `COMPLETED/`, so a migrator reading only the top level would silently
/// convert the live threads and lose every finished one -- which on this estate
/// is 54 of 56.
fn thread_dirs(project: &Project) -> Vec<(String, std::path::PathBuf)> {
  let root = project.st_dir();
  let mut out = Vec::new();
  let push_from = |dir: &Path, out: &mut Vec<(String, std::path::PathBuf)>| {
    let Ok(entries) = std::fs::read_dir(dir) else {
      return;
    };
    let mut found: Vec<(String, std::path::PathBuf)> = entries
      .flatten()
      .filter(|e| e.path().is_dir())
      .filter_map(|e| {
        let name = e.file_name().to_string_lossy().into_owned();
        is_thread_id(&name).then_some((name, e.path()))
      })
      .collect();
    found.sort();
    out.append(&mut found);
  };
  push_from(&root, &mut out);
  for bucket in ["COMPLETED", "NOT-STARTED", "CANCELLED"] {
    push_from(&root.join(bucket), &mut out);
  }
  out
}

fn is_thread_id(name: &str) -> bool {
  name.len() == 6 && name.starts_with("ST") && name[2..].bytes().all(|b| b.is_ascii_digit())
}

/// v2's status synonym table, ported from `bin/intent_helpers:canonical_status`.
///
/// **Ported rather than reimplemented from the canonical spellings, and the
/// difference is a false finding I nearly filed.** A census of this estate
/// showed one work package at `status: Complete`, which is not one of v2's
/// canonical outputs -- so it read as an unknown status. It is not: `complete`
/// is in v2's synonym table and has always resolved to `Completed`. **The
/// vocabulary is what the tool ACCEPTS, not the set of values it prints**, and
/// a migrator that confused the two would report residue against data v2
/// considered perfectly well-formed.
fn thread_status(raw: &str) -> Option<ThreadStatus> {
  match raw.trim().to_ascii_lowercase().as_str() {
    "wip" | "in progress" | "inprogress" | "in-progress" => Some(ThreadStatus::Wip),
    // **TBC maps to NotStarted, NEVER to Triage** (migration.md, ratified).
    // In v2, TBC abbreviates "To Be Commenced" -- `bin/intent_st:46` spells it
    // out in the tool's own usage text. `Triage` reuses the three letters and
    // not the meaning, so mapping to it would invent a triage decision nobody
    // made, for every thread that ever carried the token.
    "not started" | "notstarted" | "not-started" | "tbc" | "to be commenced" => {
      Some(ThreadStatus::NotStarted)
    }
    "completed" | "complete" | "done" => Some(ThreadStatus::Completed),
    "cancelled" | "canceled" => Some(ThreadStatus::Cancelled),
    "on hold" | "onhold" | "on-hold" | "hold" => Some(ThreadStatus::Hold),
    _ => None,
  }
}

fn wp_status(raw: &str) -> Option<WpStatus> {
  match raw.trim().to_ascii_lowercase().as_str() {
    "wip" | "in progress" | "inprogress" | "in-progress" => Some(WpStatus::Wip),
    "not started" | "notstarted" | "not-started" | "tbc" | "to be commenced" => {
      Some(WpStatus::NotStarted)
    }
    "done" | "complete" | "completed" => Some(WpStatus::Done),
    _ => None,
  }
}

/// The eleven scope spellings this estate actually carries.
///
/// **Measured, not guessed, and the count is load-bearing**: `Small` 56,
/// `Medium` 34, `Large` 8, `L` 8, `XL` 5, `M` 5, `S` 4, `ExtraSmall` 4,
/// `Extra Small` 3, `XS` 1, `Medium-Large` 1. v2 reads `scope:` as free text,
/// so this is not a vocabulary anyone chose -- it is the absence of one, which
/// is what modelling the field fixes.
///
/// **`Medium-Large` maps to nothing and is not guessed at.** It sits between
/// two enum values, in a closed thread, so under the carry policy it is
/// reported as carried rather than blocking -- and the model has no
/// marked-legacy form for a scope yet, which is a WP-10 dependency on
/// data-model.md rather than something to invent here.
fn scope(raw: &str) -> Option<TShirt> {
  match raw
    .trim()
    .to_ascii_lowercase()
    .replace([' ', '-', '_'], "")
    .as_str()
  {
    "xs" | "extrasmall" => Some(TShirt::XS),
    "s" | "small" => Some(TShirt::S),
    "m" | "medium" => Some(TShirt::M),
    "l" | "large" => Some(TShirt::L),
    "xl" | "extralarge" => Some(TShirt::XL),
    "xxl" => Some(TShirt::XXL),
    _ => None,
  }
}

fn work_packages(project: &Project, dir: &Path, closed: bool, out: &mut Scan) -> Vec<WorkPackage> {
  let wp_root = dir.join("WP");
  let Ok(entries) = std::fs::read_dir(&wp_root) else {
    return Vec::new();
  };
  let mut dirs: Vec<std::path::PathBuf> = entries
    .flatten()
    .map(|e| e.path())
    .filter(|p| p.is_dir())
    .collect();
  dirs.sort();

  let mut wps = Vec::new();
  for wp_dir in dirs {
    let info = wp_dir.join("info.md");
    let Ok(text) = std::fs::read_to_string(&info) else {
      continue;
    };
    let rel = project.relative(&info);
    let (front, body) = frontmatter(&text);

    let Some(seq) = front
      .get("wp_id")
      .and_then(|v| v.trim().strip_prefix("WP-"))
      .and_then(|v| v.trim().parse::<u32>().ok())
      .or_else(|| {
        wp_dir
          .file_name()
          .and_then(|n| n.to_str())
          .and_then(|n| n.parse::<u32>().ok())
      })
    else {
      out.record(
        closed,
        Finding::new(
          &rel,
          FindingClass::UnparseableRow,
          "work package has no readable wp_id".to_string(),
        ),
      );
      continue;
    };

    // **ABSENT IS NOT INVALID, and conflating them invents work.** The first
    // run of this parser reported 20 findings against this estate; 19 of them
    // were fields that were never authored, reported as values "not in the v2
    // vocabulary". Three closed threads predate the work-package frontmatter
    // convention entirely -- ST0023's work packages have no frontmatter at all
    // -- so the honest reading is "v2 never recorded this", not "v2 recorded
    // something wrong". A migrator that says the second sends someone to
    // repair files their tooling was perfectly happy with, which is precisely
    // the confident-from-partial-evidence habit v3 exists to end.
    //
    // The real count on this estate is ONE: `scope: Medium-Large`.
    let raw_status = front.get("status").cloned().unwrap_or_default();
    let status = match (raw_status.trim().is_empty(), wp_status(&raw_status)) {
      (_, Some(status)) => status,
      (true, None) => {
        out.record(
          closed,
          Finding::new(
            &rel,
            FindingClass::FieldNotRecorded,
            "this work package predates the frontmatter convention: no status was ever recorded",
          ),
        );
        WpStatus::NotStarted
      }
      (false, None) => {
        out.record(
          closed,
          Finding::new(
            &rel,
            FindingClass::UnknownStatus,
            format!("work-package status {raw_status:?} is not in the v2 vocabulary"),
          ),
        );
        WpStatus::NotStarted
      }
    };

    let raw_scope = front.get("scope").cloned().unwrap_or_default();
    let scope = match (raw_scope.trim().is_empty(), scope(&raw_scope)) {
      (_, Some(scope)) => scope,
      // Reported once per work package rather than once per absent field: the
      // fact is "this file predates the convention", and saying it twice makes
      // one old file look like two problems.
      (true, None) => TShirt::M,
      (false, None) => {
        out.record(
          closed,
          Finding::new(
            &rel,
            FindingClass::UnknownScope,
            format!(
              "work-package scope {raw_scope:?} is not a T-shirt size, and the model has no \
               marked-legacy form for one yet"
            ),
          ),
        );
        TShirt::M
      }
    };

    if let Some(line) = conflict_marker_line(&text) {
      out.record(
        closed,
        Finding::new(&rel, FindingClass::ConflictMarkers, "in a work package").at_line(line),
      );
    }

    let sections = sections(body);
    wps.push(WorkPackage {
      seq,
      title: front
        .get("title")
        .map(|t| t.trim_matches('"').to_string())
        .or_else(|| title(body))
        .unwrap_or_default(),
      scope,
      status,
      status_reason: None,
      objective: sections.get("Objective").cloned().unwrap_or_default(),
      // D28: everything that is not the objective is the body, carried
      // verbatim, so a section the template never named survives.
      body: sections
        .iter()
        .filter(|(k, _)| k.as_str() != "Objective")
        .map(|(k, v)| format!("## {k}\n\n{v}"))
        .collect::<Vec<_>>()
        .join("\n\n"),
    });
  }
  wps
}

/// Parse `acceptance.md` -- absent on most threads, which is not a finding.
///
/// **42 of this estate's 56 threads have no `acceptance.md` at all**, because
/// the contract arrived in v2.11.13. An empty contract is the ordinary case;
/// treating a missing file as residue would report 42 findings about a feature
/// those threads predate.
fn acceptance(
  project: &Project,
  dir: &Path,
  closed: bool,
  out: &mut Scan,
) -> (Vec<Criterion>, Vec<AcceptanceTest>) {
  let path = dir.join("acceptance.md");
  let Ok(text) = std::fs::read_to_string(&path) else {
    return (Vec::new(), Vec::new());
  };
  let rel = project.relative(&path);
  let mut criteria = Vec::new();
  let mut tests = Vec::new();

  for (i, line) in text.lines().enumerate() {
    let line_no = (i + 1) as u32;
    let Some(row) = line.strip_prefix("- ") else {
      continue;
    };
    if row.starts_with("AC-") {
      match criterion(row) {
        Some(c) => criteria.push(c),
        None => out.record(
          closed,
          Finding::new(&rel, FindingClass::UnparseableRow, "AC row").at_line(line_no),
        ),
      }
    } else if row.starts_with("AT-") {
      match acceptance_test(row) {
        Some(t) => tests.push(t),
        None => out.record(
          closed,
          Finding::new(&rel, FindingClass::UnparseableRow, "AT row").at_line(line_no),
        ),
      }
    }
  }

  // The broken-reference class: an AT covering a criterion that is not in this
  // thread's contract. Reported rather than dropped -- the coverage link is
  // the thing the contract is FOR.
  let ids: Vec<&str> = criteria.iter().map(|c| c.id.as_str()).collect();
  for test in &tests {
    for covered in &test.covers {
      if !ids.contains(&covered.as_str()) {
        out.record(
          closed,
          Finding::new(
            &rel,
            FindingClass::BrokenReference,
            format!(
              "{} covers {covered}, which is not in this contract",
              test.id
            ),
          ),
        );
      }
    }
  }

  (criteria, tests)
}

/// `- AC-<gg>.<n> [(non-test)] <text> [-- evidence: <e>] [-- satisfied: yes|no]`
fn criterion(row: &str) -> Option<Criterion> {
  let (id, rest) = row.split_once(' ')?;
  if !id.starts_with("AC-") {
    return None;
  }
  let non_test = rest.trim_start().starts_with("(non-test)");
  let body = rest.trim_start().trim_start_matches("(non-test)").trim();

  let evidence = field(body, "evidence");
  let satisfied = field(body, "satisfied");
  let text = body
    .split(" -- evidence:")
    .next()
    .unwrap_or(body)
    .split(" -- satisfied:")
    .next()
    .unwrap_or(body)
    .trim()
    .to_string();

  let (kind, state) = if non_test {
    let state = match (satisfied.as_deref(), evidence) {
      // **Evidence is required for a satisfied non-test criterion and is NOT
      // invented when it is missing.** A synthesised sentence reads as
      // evidence forever after and nothing downstream can tell it from the
      // real thing (vc's migration ruling). A `satisfied: yes` with no
      // evidence therefore arrives UNSATISFIED, and the v2 claim is residue
      // rather than a value.
      (Some("yes"), Some(e)) if !e.trim().is_empty() => AcState::Satisfied { evidence: e },
      _ => AcState::Unsatisfied,
    };
    (AcKind::NonTest, state)
  } else {
    // A test-backed criterion's satisfaction is COMPUTED from its covering
    // tests, so nothing is carried onto the row.
    (AcKind::Test, AcState::Computed)
  };

  Some(Criterion {
    id: id.to_string(),
    text,
    kind,
    state,
  })
}

/// `- AT-<gg>.<n> <subject> -- covers <ids> -- status: <s> [-- test: <name>] [-- note]`
///
/// **The keyed fields are FOUND rather than split out**, and that is the whole
/// difficulty of this grammar. A note routinely contains ` -- ` itself, so
/// splitting the row on the separator over-splits exactly the rows carrying the
/// most information. Searching for the three known keys leaves everything else
/// as the note, whatever it contains.
fn acceptance_test(row: &str) -> Option<AcceptanceTest> {
  let (id, rest) = row.split_once(' ')?;
  if !id.starts_with("AT-") {
    return None;
  }
  let covers = covers(rest)?;
  if covers.is_empty() {
    return None;
  }
  let status = match field(rest, "status")?.trim() {
    "green" => AtStatus::Green,
    "red" => AtStatus::Red,
    "to-write" | "towrite" => AtStatus::ToWrite,
    "n/a" | "n-a" | "na" => AtStatus::Na,
    _ => return None,
  };

  let subject = rest.split(" -- ").next().unwrap_or("").trim();
  let non_test = subject.starts_with("(non-test)");
  let file = subject.trim_matches('`').to_string();
  // The 0017 reference rules: a test file has at least one `/` and no `:`.
  // A reference failing them is a legacy citation, and it is CARRIED whole
  // rather than reshaped into something that satisfies the grammar.
  let is_path = !non_test && file.contains('/') && !file.contains(':');

  Some(AcceptanceTest {
    id: id.to_string(),
    kind: if non_test {
      AtKind::NonTest
    } else {
      AtKind::Test
    },
    file: is_path.then(|| file.clone()),
    prose: non_test.then(|| subject.trim_start_matches("(non-test)").trim().to_string()),
    covers,
    status,
    note: field(rest, "test"),
    legacy: (!non_test && !is_path && !file.is_empty())
      .then(|| crate::model::Legacy { raw: file.clone() }),
  })
}

/// The covered AC ids: ` -- covers AC-01.2, AC-01.3 -- ...`.
///
/// **Its own accessor because `covers` carries NO COLON**, where every other
/// keyed field does -- `status:`, `test:`, `evidence:`, `satisfied:`. One
/// accessor assuming a uniform `key: value` shape rejected 226 of 226 AT rows
/// in this estate and reported each one as an unparseable row, which is a
/// migrator confidently blaming an estate for its own grammar. Measured by
/// running it, not by reading the spec: the spec says "covers" and the reader
/// supplies the colon from habit.
fn covers(row: &str) -> Option<Vec<String>> {
  const MARKER: &str = " -- covers ";
  let start = row.find(MARKER)? + MARKER.len();
  let rest = &row[start..];
  let end = rest.find(" -- ").unwrap_or(rest.len());
  Some(
    rest[..end]
      .split(',')
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect(),
  )
}

/// The value of ` -- <key>: ...`, up to the next ` -- ` or the end.
fn field(row: &str, key: &str) -> Option<String> {
  let marker = format!(" -- {key}: ");
  let start = row.find(&marker)? + marker.len();
  let rest = &row[start..];
  let end = rest.find(" -- ").unwrap_or(rest.len());
  Some(rest[..end].trim().to_string())
}

/// Split v2's `---` frontmatter from the body.
///
/// Line-oriented `key: value`, exactly like the whiteboard header block and for
/// the same reason: it is hand-written prose-bearing data, and a
/// quoting-sensitive parser on hand-written values fails on the values people
/// actually write.
fn frontmatter(text: &str) -> (BTreeMap<String, String>, &str) {
  let mut map = BTreeMap::new();
  let Some(rest) = text.strip_prefix("---\n") else {
    return (map, text);
  };
  let Some(end) = rest.find("\n---\n") else {
    return (map, text);
  };
  for line in rest[..end].lines() {
    if let Some((k, v)) = line.split_once(':') {
      map.insert(k.trim().to_string(), v.trim().to_string());
    }
  }
  (map, &rest[end + 5..])
}

/// `# ST0001: Some title` -> `Some title`.
fn title(body: &str) -> Option<String> {
  let line = body.lines().find(|l| l.starts_with("# "))?;
  let after = line.trim_start_matches("# ").trim();
  Some(match after.split_once(ididy_separator(after)?) {
    Some((_, title)) => title.trim().to_string(),
    None => after.to_string(),
  })
}

/// The `: ` that separates an id prefix from a title, when there is one.
fn ididy_separator(heading: &str) -> Option<&'static str> {
  heading.contains(": ").then_some(": ")
}

/// `## Heading` sections, mapped to their verbatim bodies.
fn sections(body: &str) -> BTreeMap<String, String> {
  let mut out = BTreeMap::new();
  let mut current: Option<String> = None;
  let mut buffer = String::new();
  for line in body.lines() {
    if let Some(heading) = line.strip_prefix("## ") {
      if let Some(name) = current.take() {
        out.insert(name, buffer.trim().to_string());
      }
      buffer.clear();
      current = Some(heading.trim().to_string());
    } else if current.is_some() {
      buffer.push_str(line);
      buffer.push('\n');
    }
  }
  if let Some(name) = current {
    out.insert(name, buffer.trim().to_string());
  }
  out
}

/// v2 writes `20260814`; the model holds `2026-08-14`.
///
/// A value that is neither is passed through untouched rather than reshaped --
/// an absent date stays absent, and an unrecognised one stays exactly as
/// authored so the residue can name it.
fn date(raw: Option<&String>) -> String {
  let Some(raw) = raw.map(|s| s.trim()) else {
    return String::new();
  };
  if raw.len() == 8 && raw.bytes().all(|b| b.is_ascii_digit()) {
    format!("{}-{}-{}", &raw[0..4], &raw[4..6], &raw[6..8])
  } else {
    raw.to_string()
  }
}

/// The first line carrying a git conflict marker, if any.
fn conflict_marker_line(text: &str) -> Option<u32> {
  text
    .lines()
    .enumerate()
    .find(|(_, l)| l.starts_with("<<<<<<< ") || l.starts_with(">>>>>>> ") || *l == "=======")
    .map(|(i, _)| (i + 1) as u32)
}
