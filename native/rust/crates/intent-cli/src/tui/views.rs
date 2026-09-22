//! Entity to rows: the last mapping between the model and the screen.
//!
//! **THE DERIVATION ITSELF LIVES IN `intentsvcs::form::triples`, ONE CRATE
//! DOWN, AND THIS MODULE ONLY RENDERS WHAT IT RETURNS** (vc, 2026-08-30). It
//! was built here first, which was one crate too high: `intentd` depends on
//! `intentsvcs` and NOT on the CLI, so cc's daemon emitter would have had to
//! write the same walk again -- two homes for one derivation, arriving by the
//! door the argument was meant to shut. The line is DERIVATION shared,
//! RENDERING per face.
//!
//! What follows is the reason the derivation is shaped the way it is, kept
//! here because this is where a reader meets it.
//!
//! **THE FIELD VALUES ARE READ OUT OF THE SERIALISED ENTITY, NOT OUT OF A
//! MATCH.** A `match field { "title" => e.title, "status" => ... }` is a second
//! home for the field set -- exactly what `AC-17.2` refuses one layer up, where
//! the form declaration takes existence and type from the schema instead of
//! listing fields. A hand-written value map would put the list back, in the one
//! place nothing checks it, and it would go stale the day a property is renamed
//! -- silently, because a missing arm looks like an empty value.
//!
//! Serialising and indexing by the declared name has the property the match
//! cannot: **the declaration is the only list, and it is already held against
//! the schema.**
//!
//! **AND IT IS THE SAME MECHANISM THE WEB FACE USES.** `tui-design.md` §10a:
//! the daemon resolves the form declaration server-side and emits a generic
//! `{label, value, widget}` description, and *the JS renders triples, so does
//! SwiftUI, so does the TUI*. Building the TUI's rows any other way would make
//! the two faces agree only by coincidence -- which is what `AC-17.1` exists to
//! refuse.
//!
//! # A collection is a count, never its contents
//!
//! ST0056 carries 297 attachments. Inlining them makes the form 325 rows of
//! which 297 are files, and it breaks the alignment guarantee outright: one
//! aligned name column cannot serve `title` and
//! `parity/tools/conservation_check.sh` at once. **So an array renders as its
//! SIZE and opens its own pane** -- which is also why [`super::nav`] keys
//! descents on arrays.
//!
//! # A field the entity does not carry renders EMPTY, never skipped
//!
//! Skipping would silently shorten the form, and tab order is declaration order
//! (`AC-17.5`) -- so a skipped row moves every row after it and the operator's
//! muscle memory lands on the wrong field. An empty value is visible; a missing
//! row is not.

use intentsvcs::form::{self, Form, Loaded};
use serde_json::Value;

use super::layout::Row;
use super::nav::View;

/// What a view puts on screen, before layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rendered {
  /// The APP row's text.
  pub app: String,
  pub rows: Vec<Row>,
}

/// One row per declared field, in declaration order.
///
/// **A MAP, NOT A WALK.** The walk is `intentsvcs::form::triples`; this turns
/// its `{label, value, widget}` into the TUI's row type and does nothing else,
/// which is what keeps the terminal face and the web face agreeing by
/// construction rather than by two people reading the same design section.
///
/// **EVERY FIELD CARRIES ITS OWN CONTENTS AS DETAIL** (issue 0399), read through
/// [`form::raw`] -- the uncollapsed bytes the editor is handed -- so the pane
/// under the list shows the paragraph breaks the row collapses. A collection has
/// no text (`raw` answers `None`) and carries nothing here; its members are the
/// item view builder's to attach.
pub fn rows_for(form: &Form, entity: &Value) -> Vec<Row> {
  form::triples(form, entity)
    .into_iter()
    .map(|t| {
      let contents = form::raw(entity, &t.name);
      let row = Row::named(t.name, t.label, t.value, t.widget).editable(t.editable);
      match contents {
        Some(text) => row.reading(text),
        None => row,
      }
    })
    .collect()
}

/// The entity kinds, as the root's rows: one per declared form.
pub fn entity_rows(loaded: &Loaded) -> Vec<Row> {
  loaded
    .forms()
    .iter()
    .map(|f| {
      Row::new(f.entity.clone(), String::new(), "button").opening(View::Collection {
        kind: f.entity.clone(),
      })
    })
    .collect()
}

/// The operator's settings, as rows: `AC-17.14`.
///
/// **THE ROWS COME FROM THE ALLOW-LIST, NEVER FROM THE FILE'S KEYS**, which is
/// the criterion in one line. `settings::read_all` walks
/// [`intentsvcs::settings::DECLARED`] and looks each one up; nothing reaches
/// this function by being present on disk. **So `intent_version` and
/// `intent_dir` are not greyed out here, they are ABSENT** -- a row an operator
/// can see is a row an operator will eventually try, and that one succeeds.
///
/// The row's `name` is the setting's own path, which is what the realiser hands
/// back to the writer, so the operator's spelling and the stored key are the
/// same string all the way down.
pub fn settings_rows(config: &std::path::Path) -> Vec<Row> {
  intentsvcs::settings::read_all(config)
    .into_iter()
    .map(|(s, value)| Row::named(s.path.to_string(), s.label.to_string(), value, "setting"))
    .collect()
}

/// What an empty project registry says, on the projects view and nowhere else.
const NO_PROJECTS: &str = "no projects are registered -- `intent discover <dir>` registers the ones under a directory, and `intent explore` inside a project registers that one";

/// A registered root's display name and its spelling in a view.
fn project_named(root: &std::path::Path) -> (String, String) {
  let path = root.display().to_string();
  let name = root
    .file_name()
    .map(|n| n.to_string_lossy().into_owned())
    .unwrap_or_else(|| path.clone());
  (name, path)
}

/// Whether an Intent config is still at `root`. A registry entry outlives the
/// directory it names, so the list says so rather than dropping the entry.
fn is_project(root: &std::path::Path) -> bool {
  intentsvcs::project::Project::config_path(root).is_file()
}

/// The projects view's rows: one per registered root, in the registry's order
/// (issue 0418).
///
/// **A MISSING PROJECT IS LISTED AND MARKED, NEVER DROPPED**, and its row still
/// opens [`View::Project`]: choosing it is refused with the reason by
/// [`switch_to`], the one place that decides whether a root can be entered.
pub fn project_rows(roots: &[std::path::PathBuf]) -> Vec<Row> {
  if roots.is_empty() {
    return vec![Row::new("projects", NO_PROJECTS, "label")];
  }
  roots
    .iter()
    .map(|root| {
      let (name, path) = project_named(root);
      let value = if is_project(root) {
        path.clone()
      } else {
        format!("{path}  (missing)")
      };
      Row::new(name, value, "button").opening(View::Project { root: path })
    })
    .collect()
}

/// The omnibox's project entries: the projects list itself, then one entry per
/// registered root, so typing a project's name anywhere offers it.
pub fn project_entries(roots: &[std::path::PathBuf]) -> Vec<super::omnibox::Entry> {
  use super::omnibox::Entry;
  let mut out = vec![Entry {
    id: intentsvcs::nav::PROJECTS_SEGMENT.to_string(),
    title: "every project this machine knows".to_string(),
    status: String::new(),
    door: View::Projects,
  }];
  out.extend(roots.iter().map(|root| {
    let (name, path) = project_named(root);
    Entry {
      id: name,
      status: if is_project(root) {
        String::new()
      } else {
        "missing".to_string()
      },
      title: path.clone(),
      door: View::Project { root: path },
    }
  }));
  out
}

/// The project choosing `root` switches to, or why it cannot be entered.
pub fn switch_to(root: &str) -> Result<std::path::PathBuf, super::edit::Refused> {
  let path = std::path::PathBuf::from(root);
  if is_project(&path) {
    Ok(path)
  } else {
    Err(super::edit::Refused::new(format!(
      "`{root}` is no longer an Intent project -- restore it, or remove its entry from the project registry"
    )))
  }
}

/// The projects row the cursor starts on: the registered root that shares the
/// most leading path components with `here`, the earliest on a tie (issue
/// 0419). `None` when no row opens a project.
///
/// **ONE RULE FOR BOTH EXPLORERS** (hv, 2026-09-16: *it should start on the
/// open (or nearest?) project*). With a project open, `here` is its root, so
/// its own row shares every component and wins; in the lobby `here` is the
/// working directory, so the project nearest it wins. Where nothing is nearer
/// than a common prefix every root shares, the tie falls to the first row,
/// which is where the cursor started before.
pub fn nearest_project(rows: &[Row], here: &std::path::Path) -> Option<usize> {
  let mut best: Option<(usize, usize)> = None;
  for (at, row) in rows.iter().enumerate() {
    let Some(View::Project { root }) = &row.door else {
      continue;
    };
    let shared = std::path::Path::new(root)
      .components()
      .zip(here.components())
      .take_while(|(a, b)| a == b)
      .count();
    if best.is_none_or(|(_, most)| shared > most) {
      best = Some((at, shared));
    }
  }
  best.map(|(at, _)| at)
}

/// The table `intent outs` prints, as rows (ST0079 `AC-01.1`, `AC-01.2`).
///
/// **THE ROWS LEAVE IN THE ORDER THEY ARRIVE.** Nothing here sorts, groups or
/// filters: what is outstanding, and in what order, is
/// [`intentsvcs::outstanding`]'s, so a change there reaches this view with no
/// change here (hv, 2026-09-22: *the list of items should come out of the
/// intentsvcs layer from the same functions*).
///
/// **FOUR CELLS IN THE TWO COLUMNS THE ROW MODEL HAS** ([`Row`]'s note): the
/// kind and ID on the left, the status and title on the right, each padded to
/// its widest so the four read as the verb's columns. The cells arrive already
/// written, by the helper the verb prints its own table through, so the two
/// faces cannot disagree about what a row says or in which column.
///
/// **EACH ROW OPENS WHAT IT NAMES** (`AC-01.3`, hv 2026-09-22): its door is
/// [`outstanding_door`]'s, the view the thread, package or issue's own list
/// opens, so Enter goes where it would from `/threads` or `/issues`.
///
/// **THE COUNTS END THE VIEW, AND WITH NO ROWS THEY ARE THE VIEW** -- none of N
/// rather than an empty body, as the verb prints them (`AC-00.4`).
pub fn outstanding_rows(rows: &[([String; 4], Option<View>)], counts: &str) -> Vec<Row> {
  let widest = |at: usize| {
    rows
      .iter()
      .map(|(cells, _)| cells[at].chars().count())
      .max()
      .unwrap_or(0)
  };
  let (kinds, statuses) = (widest(0), widest(2));
  let mut rows: Vec<Row> = rows
    .iter()
    .map(|([kind, id, status, title], door)| {
      let mut row = Row::named(
        id.clone(),
        format!("{kind:<kinds$}  {id}"),
        format!("{status:<statuses$}  {title}"),
        "button",
      );
      row.door = door.clone();
      row
    })
    .collect();
  if !rows.is_empty() {
    rows.push(Row::rule());
  }
  rows.push(Row::new("outstanding", counts, "label"));
  rows
}

/// Where Enter on an outstanding row goes (ST0079 `AC-01.3`): the view of the
/// thread, work package or issue the row names, through
/// [`intentsvcs::nav::view_for`], the one mapping from an entity to its view.
///
/// **A WORK PACKAGE IS OPENED BY ITS FIELDS, NEVER BY ITS RENDERED ID.** The
/// row carries the package's thread and sequence
/// ([`intentsvcs::outstanding::WpRef`]), and the sequence goes in as the number
/// the `wps` descent names its rows by (`1`, never `01`), or the door would
/// open onto a package the child view cannot find.
pub fn outstanding_door(row: &intentsvcs::outstanding::Row) -> Option<View> {
  use intentsvcs::address::Entity;
  use intentsvcs::outstanding::Kind;
  let entity = match row.kind {
    Kind::Thread => Entity::Thread { id: row.id.clone() },
    Kind::Issue => Entity::Issue { id: row.id.clone() },
    Kind::WorkPackage => {
      let wp = row.wp.as_ref()?;
      Entity::Wp {
        thread: wp.thread.clone(),
        wp: wp.seq.to_string(),
      }
    }
  };
  intentsvcs::nav::view_for(&entity)
}

/// The APP row's text for a view. **The trail and the exit key belong to the
/// stack, not to this** -- see [`super::nav::Stack::trail`].
pub fn app_line(view: &View) -> String {
  match view {
    View::Entities => "intent".to_string(),
    View::Collection { kind } => kind.clone(),
    View::Item { kind, id } => format!("{kind}  {id}"),
    View::Children { kind, id, field } => format!("{kind}  {id}  {field}"),
    View::Child {
      kind,
      id,
      field,
      item,
    } => format!("{kind}  {id}  {field}  {item}"),
    View::Settings => format!("settings  {}", intentsvcs::settings::SECTION),
    View::Help { of: None } => "help".to_string(),
    View::Help { of: Some(name) } => format!("help  intent {name}"),
    View::Search { query } if query.is_empty() => "search".to_string(),
    View::Search { query } => format!("search  {query}"),
    View::Projects => "projects".to_string(),
    View::Project { root } => format!("project  {root}"),
    View::Outstanding => "outstanding".to_string(),
  }
}

/// The envelope's hits as rows, groups separated by a rule (AC-21.1).
///
/// **THE ROWS ARE THE ENVELOPE'S, NOT A SECOND SHAPE READ OUT OF THE STORE.**
/// This is the same value `--json` prints and the MCP tool answers, mapped once
/// -- so a hit the CLI shows and a hit the pane shows cannot differ, which is
/// what `AC-21.3` asks for and what a pane building its own query would break
/// on its first divergence.
///
/// **A FILE HIT GETS NO DOOR AND THAT IS NOT AN OMISSION.** `door` is where
/// Enter DESCENDS inside the model; a file is opened in the operator's editor
/// through the lent terminal, which is an ACT rather than a descent, and the
/// loop routes it by the row's kind. Giving it a door would send Enter into a
/// view of an entity that does not exist.
pub fn search_rows(answer: &intentsvcs::search::SearchAnswer) -> Vec<Row> {
  let _ = &answer.index;
  let mut rows: Vec<Row> = Vec::new();
  for group in &answer.groups {
    // **A RULE SEPARATING NOTHING FROM SOMETHING IS DECORATION** -- the rule
    // this module already states for the thread collection's open/closed seam.
    if !rows.is_empty() && !group.hits.is_empty() {
      rows.push(Row::rule());
    }
    for hit in &group.hits {
      let place = match &hit.span {
        Some(span) => format!("{}:{}", hit.path, span.start_line),
        None => hit.path.clone(),
      };
      // **THE NAME IS THE BARE PATH AND THE TITLE CARRIES THE LINE.** `name` is
      // what the row is ACTED ON under -- the loop opens it -- and `path:12` is
      // not a file anything can open; `title` is what the operator READS, and a
      // hit without its line is a hit they have to go hunting in.
      //
      // **THE ROW'S KIND IS `button`, NOT THE HIT'S KIND, AND THE DESIGN SAYS
      // `{type: kind}`.** The divergence is deliberate and was found by driving:
      // in this estate a row's `kind` is the MODE MACHINE's discriminator, a
      // widget name -- `super::mode::BY_ROW_KIND` routes Enter by it, and a kind
      // it does not know takes the unclaimed arm, which for Enter is FIELD. A
      // hit carrying `thread` as its kind therefore opened an EDITOR over a
      // search result. The taxonomy travels where a reader can use it, in the
      // value; the widget stays a widget, which is what keeps one meaning in one
      // field.
      // ST0076 WP-04: a symbol hit says what it is and where it sits.
      let what = match &hit.symbol {
        Some(facts) => match facts.container.as_deref() {
          Some(container) => format!("{} {} in {container}", hit.kind.as_str(), facts.subkind),
          None => format!("{} {}", hit.kind.as_str(), facts.subkind),
        },
        None => hit.kind.as_str().to_string(),
      };
      // ST0076 WP-07: where a reference points, in the terminal's words.
      let detail = match hit
        .symbol
        .as_ref()
        .and_then(intentsvcs::search::SymbolFacts::points_to)
      {
        Some(points) => format!("{what}  {}  {points}", hit.snippet),
        None => format!("{what}  {}", hit.snippet),
      };
      let mut row = Row::named(hit.path.clone(), place, detail, "button");
      row.door = door_for(hit);
      rows.push(row);
    }
  }
  rows
}

/// What the reader must know before trusting these rows, or `None` when there
/// is nothing to say (AC-21.1, AC-19.3).
///
/// **IT GOES TO THE INFO ROW RATHER THAN INTO THE BODY**, which is what the
/// layout's own section list asks for: INFO is *help for whatever is under the
/// cursor* and a notice takes it. A freshness row inside the body would be a
/// row the cursor can land on and Enter cannot open -- and it would shift every
/// hit down by one, so the first thing the eye lands on would not be the best
/// match.
///
/// **AN EMPTY CORPUS IS SAID DIFFERENTLY FROM AN INCOMPLETE ONE**, because they
/// are different facts and the estate's dominant defect is reading the first as
/// a miss.
pub fn freshness_note(answer: &intentsvcs::search::SearchAnswer) -> Option<String> {
  if answer.index.is_empty() {
    return Some(
      "nothing is indexed, so this search could not have matched -- an empty result does NOT mean the phrase is absent"
        .to_string(),
    );
  }
  if answer.index.complete() {
    // **WITH NOTHING PARTIAL TO SAY, THE SYMBOL HITS' LEVEL IS SAID** (ST0076
    // WP-04, AC-04.2): the same note the terminal prints.
    return answer.symbol_note();
  }
  let stale = answer.index.stale.len();
  let skipped = answer.index.gaps().count();
  Some(match (stale, skipped) {
    (0, n) => format!("{n} path(s) in scope were not indexed -- this answer is partial"),
    (n, 0) => format!("{n} file(s) changed since they were indexed -- their hits carry no line"),
    (n, m) => format!("{n} file(s) moved on and {m} were not indexed -- this answer is partial"),
  })
}

/// Where Enter on a hit lands, when it lands inside the model (AC-21.2).
///
/// **THE ADDRESS COMES FROM THE ENTITY VOCABULARY, NOT FROM THE PATH.** A hit
/// carries the FILE its prose lives in, and for a canon section that file is
/// `thread.json` -- a path no view addresses. The entity's own kind and id are
/// what `nav` addresses, so they are what this reads; anything it cannot place
/// in the model gets no door and is opened as a file instead.
fn door_for(hit: &intentsvcs::search::Hit) -> Option<View> {
  use intentsvcs::search::HitKind;
  let id = hit.owner.as_deref()?;
  let kind = match hit.kind {
    HitKind::Thread => "thread",
    HitKind::Issue => "issue",
    // **A WORK PACKAGE IS ADDRESSED BY TWO COMPONENTS AND A HIT CARRIES ONE
    // STRING**, which is the exact collision `View::Child` was built for. The
    // owner id of a wp section is not split here on a guess: until the envelope
    // carries the pair, a wp hit opens as a file, which is true rather than
    // nearly right.
    _ => return None,
  };
  Some(View::Item {
    kind: kind.to_string(),
    id: id.to_string(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  /// Rows for these roots, as the projects list builds them.
  fn projects(roots: &[&str]) -> Vec<Row> {
    project_rows(
      &roots
        .iter()
        .map(std::path::PathBuf::from)
        .collect::<Vec<_>>(),
    )
  }

  /// Issue 0419: the open project's own row, wherever it sits in the
  /// registry, and from inside it as well as at its root.
  #[test]
  fn the_cursor_starts_on_the_open_project() {
    let rows = projects(&[
      "/u/m/Devel/prj/Alpha",
      "/u/m/Devel/prj/Intent",
      "/u/m/Devel/prj/Laksa",
    ]);
    let at = |here: &str| nearest_project(&rows, std::path::Path::new(here));
    assert_eq!(
      at("/u/m/Devel/prj/Intent"),
      Some(1),
      "not on the open project"
    );
    assert_eq!(
      at("/u/m/Devel/prj/Laksa"),
      Some(2),
      "not on the open project"
    );
    assert_eq!(
      at("/u/m/Devel/prj/Intent/native/rust"),
      Some(1),
      "a working directory inside a project did not find that project"
    );
  }

  /// Issue 0419: in the lobby the nearest project wins, and a
  /// directory that shares only the home prefix with every project leaves the
  /// cursor on the first row, as it was before.
  #[test]
  fn with_no_project_open_the_cursor_starts_on_the_nearest_one() {
    let rows = projects(&[
      "/u/m/Work/Client",
      "/u/m/Devel/prj/Intent",
      "/u/m/Devel/lab/Probe",
    ]);
    let at = |here: &str| nearest_project(&rows, std::path::Path::new(here));
    assert_eq!(at("/u/m/Devel/lab"), Some(2), "not on the nearest project");
    assert_eq!(
      at("/u/m/Devel"),
      Some(1),
      "a tie between two equally near projects did not fall to the earlier"
    );
    assert_eq!(
      at("/u/m/Downloads"),
      Some(0),
      "sharing only the home prefix moved the cursor off the first row"
    );
  }

  /// Issue 0419: rows that open no project, including the empty
  /// registry's own row, choose nothing.
  #[test]
  fn a_list_with_no_projects_chooses_no_row() {
    let here = std::path::Path::new("/u/m/Devel/prj/Intent");
    assert_eq!(nearest_project(&projects(&[]), here), None);
    assert_eq!(
      nearest_project(&[Row::new("title", "Intent", "text")], here),
      None
    );
  }

  fn loaded() -> Loaded {
    Loaded::load().expect("the shipped form declaration must load")
  }

  /// A thread-shaped value carrying every kind of JSON the model produces, so
  /// the arms below are driven rather than reasoned about.
  fn a_thread() -> Value {
    json!({
      "title": "Add a Rust-based CLI",
      "status": "wip",
      "objective": "line one\nline two\n\nline four",
      "created": "2026-08-14",
      "seq": 56,
      "wps": [1, 2, 3],
      "criteria": [],
      "fiat": { "because": "hv said so" },
      "slug": null
    })
  }

  #[test]
  fn every_declared_field_gets_exactly_one_row_in_declaration_order() {
    let l = loaded();
    let mut checked = 0usize;
    for form in l.forms() {
      let rows = rows_for(form, &a_thread());
      assert_eq!(
        rows.len(),
        form.fields.len(),
        "{} produced {} rows for {} declared fields",
        form.entity,
        rows.len(),
        form.fields.len()
      );
      for (row, field) in rows.iter().zip(form.fields.iter()) {
        assert_eq!(
          row.title, field.label,
          "row order diverged from declaration order"
        );
        assert_eq!(
          row.kind, field.widget,
          "a row carries a widget the form did not declare"
        );
      }
      checked += 1;
    }
    assert!(
      checked > 0,
      "no form was examined, so this test asserted nothing"
    );
  }

  /// **ISSUE 0399: A TEXT FIELD CARRIES ITS OWN CONTENTS, UNCOLLAPSED.** The row
  /// shows the objective on one line and the pane must get its paragraph breaks
  /// back, so the detail is the raw value and never the row's rendering. A
  /// collection has no text and carries none.
  #[test]
  fn a_text_field_carries_its_raw_contents_for_the_pane() {
    use crate::tui::layout::Detail;
    let l = loaded();
    let form = l.form("thread").expect("the thread form is declared");
    let rows = rows_for(form, &a_thread());
    let objective = rows
      .iter()
      .find(|r| r.name == "objective")
      .expect("the thread form has an objective row");
    assert_eq!(
      objective.detail,
      Some(Detail::Contents(
        "line one\nline two\n\nline four".to_string()
      )),
      "the pane was handed something other than the raw objective"
    );
    assert_ne!(
      objective.value, "line one\nline two\n\nline four",
      "the ROW must stay one line"
    );
    let wps = rows
      .iter()
      .find(|r| r.name == "wps")
      .expect("the thread form has a wps row");
    assert_eq!(wps.detail, None, "a collection carried text contents");
  }

  #[test]
  fn the_root_offers_one_row_per_declared_kind_and_they_are_all_descents() {
    let l = loaded();
    let rows = entity_rows(&l);
    assert!(
      !rows.is_empty(),
      "the root has no rows, so explore opens on nothing"
    );
    assert_eq!(
      rows.len(),
      l.forms().len(),
      "the root is not one row per declared form"
    );
    assert!(
      rows.iter().all(|r| r.kind == "button"),
      "a root row is not a descent"
    );
  }

  /// `AT-17.14` (the renderer half; the writer half is in `tui/app.rs`).
  ///
  /// **`AC-17.14`, DRIVEN AT THE SURFACE: what the file contains does not
  /// decide what the screen offers.** The fixture is a config carrying the
  /// migration marker, the structural key, the author and a key nobody
  /// declared -- four rows a derived surface would render and this one must
  /// not. Held here rather than only in `intentsvcs::settings` because the
  /// criterion is about a SURFACE, and the allow-list being right is no
  /// evidence that the renderer asked it.
  #[test]
  fn the_settings_view_renders_the_declared_set_and_not_what_the_file_holds() {
    let dir = std::env::temp_dir().join("intent-settings-view-rows");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture dir");
    let path = dir.join("config.json");
    std::fs::write(
      &path,
      "{\n  \"intent_version\": \"3.0.0\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"something_nobody_declared\": 41,\n  \"explorer\": { \"editing\": { \"mode\": \"vi\" } }\n}\n",
    )
    .expect("fixture");

    let rows = settings_rows(&path);
    assert_eq!(
      rows.len(),
      intentsvcs::settings::DECLARED.len(),
      "the view is not one row per declared setting"
    );
    for absent in [
      "intent_version",
      "intent_dir",
      "author",
      "something_nobody_declared",
    ] {
      assert!(
        !rows
          .iter()
          .any(|r| r.name == absent || r.title == absent || r.value == absent),
        "`{absent}` reached the screen -- it is in the file, and that is not a reason"
      );
    }
    let mode = rows
      .iter()
      .find(|r| r.name == "editing.mode")
      .expect("the one declared setting has no row");
    assert_eq!(mode.value, "vi", "the row does not show the value in force");
    assert_eq!(
      mode.kind, "setting",
      "a settings row must carry the kind the mode machine resolves Enter with"
    );
  }

  /// The APP row says where you are, for every view -- *a way back that is
  /// wired and unlabelled is a way back nobody finds*.
  #[test]
  fn every_view_names_itself_on_the_app_row() {
    let views = [
      View::Entities,
      View::Collection {
        kind: "thread".into(),
      },
      View::Item {
        kind: "thread".into(),
        id: "ST0056".into(),
      },
      View::Children {
        kind: "thread".into(),
        id: "ST0056".into(),
        field: "wps".into(),
      },
      View::Settings,
      View::Help { of: None },
      View::Help {
        of: Some("st".into()),
      },
      View::Projects,
    ];
    let mut seen: Vec<String> = Vec::new();
    for v in &views {
      let line = app_line(v);
      assert!(!line.trim().is_empty(), "{v:?} puts nothing on the APP row");
      assert!(
        !seen.contains(&line),
        "{v:?} shares an APP row with another view: {line:?}"
      );
      seen.push(line);
    }
  }
}
