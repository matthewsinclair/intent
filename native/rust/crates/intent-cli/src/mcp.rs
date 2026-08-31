//! The MCP tool tier: every tool, generated from the dispatch table -- and
//! [`serve`], the hand-written match that answers them.
//!
//! **THE DATA HALF EMITS DESCRIPTIONS; [`serve`] MAKES THE CALLS** -- vc's
//! (a)-now ruling (2026-08-30): hand-written arms, gated two-sided against
//! this population, with (b) -- routing through the daemon's one
//! `dispatch(op)` door -- recorded as the 3.x destination rather than
//! refused. The split matters for the tag window: neither half needs a new
//! dependency, so nobody's `Cargo.lock` moves until the server arm lands.
//!
//! # 3.0.0 LIMIT: the MCP face serves IN-PROCESS ONLY (vc, ruled 2026-08-30)
//!
//! AC-08.2's dual-path identity governs the CLI face and does NOT extend to
//! this one: [`serve`] holds a `Facade` opened in-process and never consults
//! a daemon. **The limit is discharged when this tier routes through
//! `dispatch(op)`, the 3.x destination** -- until then, an MCP caller on a
//! machine with a live daemon reads and writes the same store through the
//! store's own serialisation, which is what makes in-process honest rather
//! than stale.
//!
//! # Open-per-call
//!
//! The facade is opened by the CALLER, per call, through `render.rs`'s one
//! door -- `cli_routing::the_in_process_engine_has_exactly_one_door` counts
//! construction sites, and this module must never add one. Every exposed row
//! opens SHARED: the plan's draft said `open_exclusive()` for `st sync`, and
//! the tree says otherwise -- the CLI's own `st sync` arm opens shared, and
//! the exclusive family (top-level `sync`, `ingest`) exposes no tool, so the
//! claim was corrected to the measurement rather than the measurement to the
//! claim. [`serve`] takes the opened facade and returns; nothing here caches,
//! pools, or re-opens.
//!
//! # The population is the TABLE's, by declaration
//!
//! A row becomes a tool iff it declares `exposed_on_mcp: true` AND names its
//! serving `facade` method -- the AC-09.6 contract, *exposed implies
//! servable*, with the read's report classifying every row that is not here
//! yet (facade gaps, namespaces, narrows, unwired). No skip list, no
//! hand-kept roster: the day a row gains its door it becomes a tool by
//! regeneration, which is `AC-09.4`'s rule applied to the surface it was
//! written about.
//!
//! # Parameter schemas come from the table, NOT from `schemars` over types
//!
//! Deliberate, and D37's qualification is the reason: *a doc comment on a
//! derived type is an unreviewed publication channel -- the author is writing
//! a comment and the consumer is reading a contract.* The table's `args` and
//! `flags` are authored, reviewed canon; deriving the schemas from them keeps
//! every published word one somebody chose to publish. The D37 sweep below
//! asserts the consequence: no Intent-internal tracker id reaches any tool's
//! name, description or schema.
//!
//! # Descriptions lead with what calling this DOES to the estate
//!
//! `AC-09.1`'s extension, in D45's projection order and through
//! [`crate::guide`]'s own renderers rather than a second spelling of them:
//! `read_or_mutate` first, then the row's help, and for a mutation the
//! recoverability sentence -- because the MCP agent is the surface with LESS
//! context, and it needs the safety fact more, not less.

use crate::dispatch::{Arg, Entry, Flag, Table};
use intentsvcs::contract::{Scope, Verdict};
use intentsvcs::facade::{
  EventFilter, Exported, Facade, FacadeContext, FacadeError, ListEdit, Note, Outcome,
};
use intentsvcs::model::{AcKind, AtStatus, IssueStatus, ThreadStatus};
use intentsvcs::remedy::Remedy;
use serde_json::{Map, Value, json};

/// One generated tool: what an MCP client is told, plus the routing facts the
/// serving match needs (`path` for errors, `facade` for the call).
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
  pub name: String,
  pub path: String,
  pub facade: String,
  pub read_or_mutate: String,
  pub description: String,
  pub input_schema: Value,
}

/// Why a row that asked to be a tool could not become one.
///
/// **REFUSED, NEVER SMOOTHED** -- a declared enum whose values the generator
/// cannot find is a row to fix, and stringifying it would publish a schema
/// that accepts what the command refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Undeclarable {
  pub path: String,
  pub why: String,
}

/// Every tool the committed table declares, or the first row that refuses.
pub fn tools(table: &Table) -> Result<Vec<Tool>, Undeclarable> {
  let mut out = Vec::new();
  for entry in crate::dispatch::shipped_entries(table) {
    let Some(facade) = entry.facade.as_deref() else {
      continue;
    };
    if !entry.exposed_on_mcp {
      continue;
    }
    out.push(Tool {
      name: tool_name(&entry.path),
      path: entry.path.clone(),
      facade: facade.to_string(),
      read_or_mutate: entry.read_or_mutate.clone(),
      description: description(entry),
      input_schema: schema(entry)?,
    });
  }
  Ok(out)
}

/// `st new` -> `intent_st_new`. The prefix is the namespace `intent_graphql`
/// (AC-09.2) already established; the body is the path with its one legal
/// separator swapped.
fn tool_name(path: &str) -> String {
  format!("intent_{}", path.replace([' ', '-'], "_"))
}

/// The D45 projection, through the guide's own renderer -- one home for the
/// safety sentence, asserted by the guide's tests and reused here verbatim.
fn description(entry: &Entry) -> String {
  format!(
    "{} {}",
    entry.help.trim_end_matches('.'),
    crate::guide::safety(entry)
  )
}

/// Does this arg reach the published schema? A `subcommand` slot does not:
/// it is how the TABLE spells a third level (`todo list` fills `todo`'s verb
/// slot), and each filling is its own tool -- publishing the slot would offer
/// a parameter whose every value is already a different tool's name.
fn published_arg(arg: &Arg) -> bool {
  arg.kind != "subcommand"
}

/// Does this flag reach the published schema? [`Flag::ships`] is the one home
/// for the keep rule, and `exposed_on_mcp` is the flag-level narrow.
///
/// **THIS REPLACED A FILTER THAT HAD MATCHED ZERO FLAGS SINCE THE TIER
/// LANDED.** The first cut skipped `disposition == "drop"` -- and the
/// vocabulary is keep/retire/intrinsic, with no `drop` in it, so the skip
/// never fired and every retired and intrinsic flag was published
/// (`intent_doctor` advertised `fix` AND `help`). A predicate that cannot
/// match its subject returns the number that means success; the vocabulary is
/// now refused at `dispatch::table()` load, so this comparison provably ranges
/// over values the corpus can hold, and the positive control below drives it.
fn published_flag(flag: &Flag) -> bool {
  flag.ships() && flag.exposed_on_mcp
}

/// The parameter names a tool publishes -- the SAME predicates `schema()`
/// filters by, so [`serve`]'s unknown-parameter refusal and the published
/// schema cannot drift.
fn published_params(entry: &Entry) -> Vec<String> {
  entry
    .args
    .iter()
    .filter(|a| published_arg(a))
    .map(|a| a.name.clone())
    .chain(
      entry
        .flags
        .iter()
        .filter(|f| published_flag(f))
        .filter_map(flag_name),
    )
    .collect()
}

/// One JSON-Schema object from the row's declared args and flags.
fn schema(entry: &Entry) -> Result<Value, Undeclarable> {
  let refuse = |why: String| Undeclarable {
    path: entry.path.clone(),
    why,
  };
  let mut properties = Map::new();
  let mut required: Vec<String> = Vec::new();

  for arg in entry.args.iter().filter(|a| published_arg(a)) {
    let mut prop = Map::new();
    match arg.kind.as_str() {
      "enum" => {
        if arg.values.is_empty() {
          return Err(refuse(format!(
            "arg `{}` is an enum with no `values` list",
            arg.name
          )));
        }
        prop.insert("type".into(), json!("string"));
        prop.insert("enum".into(), json!(arg.values));
      }
      // The domain id types all travel as strings; the description carries
      // the domain so an agent types `ST0056`, not a guess.
      "st-id" => {
        prop.insert("type".into(), json!("string"));
        prop.insert("description".into(), json!("a steel thread id, eg ST0000"));
      }
      "st-id/NN" => {
        prop.insert("type".into(), json!("string"));
        prop.insert(
          "description".into(),
          json!("a work package id, eg ST0000/01"),
        );
      }
      "st-id[/NN]" => {
        prop.insert("type".into(), json!("string"));
        prop.insert(
          "description".into(),
          json!("a steel thread id, optionally scoped to a work package, eg ST0000 or ST0000/01"),
        );
      }
      "ac-id" => {
        prop.insert("type".into(), json!("string"));
        prop.insert(
          "description".into(),
          json!("an acceptance criterion id, eg AC-0.0"),
        );
      }
      "at-id" => {
        prop.insert("type".into(), json!("string"));
        prop.insert(
          "description".into(),
          json!("an acceptance test id, eg AT-0.0"),
        );
      }
      "issue-id" => {
        prop.insert("type".into(), json!("string"));
        prop.insert("description".into(), json!("an issue number, eg 0000"));
      }
      "string" | "positional" => {
        prop.insert("type".into(), json!("string"));
      }
      other => {
        return Err(refuse(format!(
          "arg `{}` has undeclared type `{other}`",
          arg.name
        )));
      }
    }
    if let Some(default) = &arg.default {
      prop.insert("default".into(), json!(default));
    }
    properties.insert(arg.name.clone(), Value::Object(prop));
    match arg.arity.as_str() {
      "1" => required.push(arg.name.clone()),
      "0..1" => {}
      other => {
        return Err(refuse(format!(
          "arg `{}` has undeclared arity `{other}`",
          arg.name
        )));
      }
    }
  }

  for flag in entry.flags.iter().filter(|f| published_flag(f)) {
    let name = flag_name(flag)
      .ok_or_else(|| refuse(format!("a flag on `{}` has no long spelling", entry.path)))?;
    let mut prop = Map::new();
    match flag.kind.as_str() {
      "bool" => {
        prop.insert("type".into(), json!("boolean"));
      }
      "string" => {
        prop.insert("type".into(), json!("string"));
      }
      "integer" => {
        prop.insert("type".into(), json!("integer"));
      }
      "enum" => {
        let values = enum_values(flag).ok_or_else(|| {
          refuse(format!(
            "flag `--{name}` on `{}` is an enum with no values",
            entry.path
          ))
        })?;
        prop.insert("type".into(), json!("string"));
        prop.insert("enum".into(), json!(values));
      }
      other => {
        return Err(refuse(format!(
          "flag `--{name}` has undeclared type `{other}`"
        )));
      }
    }
    if !flag.help.is_empty() {
      prop.insert("description".into(), json!(flag.help));
    }
    if let Some(default) = &flag.default {
      prop.insert("default".into(), json!(default));
    }
    properties.insert(name, Value::Object(prop));
  }

  Ok(json!({
    "type": "object",
    "properties": properties,
    "required": required,
  }))
}

/// The property name a flag publishes under: its long spelling, bare.
fn flag_name(flag: &Flag) -> Option<String> {
  flag
    .spellings
    .iter()
    .find(|s| s.starts_with("--"))
    .map(|s| s.trim_start_matches('-').replace('-', "_"))
}

/// A flag enum's values come from `value` ("a|b", the `--format` precedent)
/// and from nowhere else. `accepts` is deliberately not deserialized --
/// dispatch.rs records why: four rows of PROSE in four shapes with no common
/// parse -- so the two rows that carried real value sets only there had their
/// `value` placeholders rewritten into lists by this module's author, which
/// is the direction the record prescribes: fix the row, never smooth the
/// reader. A `<placeholder>` that survives is a row to fix and refuses here.
fn enum_values(flag: &Flag) -> Option<Vec<String>> {
  match &flag.value {
    Some(v) if !v.starts_with('<') => Some(v.split('|').map(|s| s.trim().to_string()).collect()),
    _ => None,
  }
}

// ---------------------------------------------------------------------------
// The serving match
// ---------------------------------------------------------------------------

/// Why a tool call was not answered.
///
/// Three variants and no more, because the caller's next move differs three
/// ways: `UnknownTool` means the NAME is wrong (or narrowed off this surface);
/// `Args` means the name was right and the arguments were not; `Refused` means
/// the call was well-formed and the facade said no -- carrying the full
/// [`FacadeError`], whose own rendering (message, cause chain, remedy) is the
/// one rendering (AC-04.4).
#[derive(Debug, thiserror::Error)]
pub enum ServeError {
  #[error("no MCP tool serves `{path}`")]
  UnknownTool { path: String },
  #[error("`{path}`: {why}")]
  Args { path: String, why: String },
  #[error(transparent)]
  Refused(#[from] FacadeError),
  /// The escape hatch could not reach, or was refused by, the daemon that
  /// executes it. Rendered by `hatch` with its own remedies -- the one tool on
  /// this surface whose refusal can name `intent daemon start`.
  #[error(transparent)]
  Bridge(#[from] crate::hatch::HatchError),
}

impl ServeError {
  /// The operator-facing text: the facade's own rendering for a refusal, the
  /// message itself for the two argument-shaped variants.
  pub fn render(&self) -> String {
    match self {
      Self::Refused(e) => e.render(),
      Self::Bridge(e) => e.render(),
      other => other.to_string(),
    }
  }
}

/// Every path [`serve`] answers -- ONE roster, asserted equal to the
/// generated tool population in BOTH directions by
/// `tests::the_roster_and_the_tool_population_agree_both_ways`, and driven
/// end-to-end by `tests::every_roster_path_reaches_an_arm`. A row gaining its
/// door joins `tools()` by regeneration and this list by hand -- the gate is
/// what makes forgetting either half a red test rather than a silent gap.
pub const SERVED: [&str; 60] = [
  "st new",
  "st start",
  "st done",
  "st cancel",
  "st triage",
  "st hold",
  "st resume",
  "st reopen",
  "st reinstate",
  "st hydrate",
  "st dehydrate",
  "st list",
  "st show",
  "st edit",
  "st sync",
  "wp new",
  "wp start",
  "wp done",
  "wp reopen",
  "wp cancel",
  "wp reinstate",
  "wp unstart",
  "wp rescope",
  "wp list",
  "wp show",
  "ac list",
  "ac status",
  "ac satisfy",
  "ac unsatisfy",
  "ac gate",
  "ac descope",
  "ac rescope",
  "ac withdraw",
  "ac reinstate",
  "ac new",
  "ac edit",
  "at list",
  "at lint",
  "at green",
  "at red",
  "at na",
  "at new",
  "at edit",
  "issues list",
  "issues add",
  "issues show",
  "issues close",
  "issues open",
  "todo",
  "todo list",
  "todo update",
  "doctor",
  "agents generate",
  "agents validate",
  "search",
  "export",
  "organize",
  "events",
  "graphql",
  "schema",
];

/// Answer one tool call against an already-open facade.
///
/// `path` is the TABLE path (`st new`), not the tool name (`intent_st_new`) --
/// the caller that published the tools holds both on [`Tool`]. `args` is the
/// call's argument object; `Null` reads as empty. The population is the
/// table's: a path outside it, or narrowed off this surface, is
/// [`ServeError::UnknownTool`] whether or not an arm exists below.
pub fn serve(
  f: &mut Facade,
  ctx: &FacadeContext,
  path: &str,
  args: &Value,
) -> Result<Value, ServeError> {
  let table = crate::dispatch::table();
  let entry = match crate::dispatch::entry(&table, path) {
    Some(e) if e.exposed_on_mcp && e.facade.is_some() => e,
    _ => {
      return Err(ServeError::UnknownTool {
        path: path.to_string(),
      });
    }
  };

  let empty = Map::new();
  let map: &Map<String, Value> = match args {
    Value::Null => &empty,
    Value::Object(m) => m,
    other => {
      return Err(args_err(
        path,
        format!("arguments must be an object, not {other}"),
      ));
    }
  };
  // **UNKNOWN PARAMETERS ARE REFUSED, NOT IGNORED.** An arm that reads known
  // keys and skips the rest would ACCEPT a narrowed flag in silence -- the
  // dishonest half of the hide-classify discipline. One check here closes the
  // class for every tool at once, against the same predicates `schema()`
  // publishes by.
  let known = published_params(entry);
  for key in map.keys() {
    if !known.iter().any(|k| k == key) {
      return Err(args_err(
        path,
        format!(
          "no parameter named `{key}` on this tool -- it takes: {}",
          if known.is_empty() {
            "no parameters".to_string()
          } else {
            known.join(", ")
          }
        ),
      ));
    }
  }

  match path {
    // ----- steel threads -----
    "st new" => {
      let title = need_s(path, map, "title")?;
      // **`start` REFUSES rather than composing** (vc's compose ruling): the
      // CLI's `-s` spans Triage -> NotStarted -> Wip as two declared
      // transitions, and an MCP agent composes calls natively -- offering the
      // shortcut here would put a second home on a two-step act.
      if opt_b(path, map, "start")? {
        return Err(args_err(
          path,
          "`start` is not served here -- create the thread, then call `intent_st_start` with the new id",
        ));
      }
      let id = f.st_new(title)?;
      Ok(json!({ "created": id }))
    }
    "st start" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      Ok(outcome_json(&f.st_start(&id)?, &id))
    }
    "st triage" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      Ok(outcome_json(&f.st_triage(&id)?, &id))
    }
    "st resume" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      Ok(outcome_json(&f.st_resume(&id)?, &id))
    }
    "st hold" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default();
      Ok(outcome_json(&f.st_hold(&id, reason)?, &id))
    }
    "st reopen" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default();
      Ok(outcome_json(&f.st_reopen(&id, reason)?, &id))
    }
    "st reinstate" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default();
      Ok(outcome_json(&f.st_reinstate(&id, reason)?, &id))
    }
    "st done" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let list = if opt_b(path, map, "keep")? {
        ListEdit::Suppressed
      } else {
        ListEdit::AsDeclared
      };
      let on = opt_s(path, map, "date")?;
      Ok(outcome_json(&f.st_done_listing(&id, list, on)?, &id))
    }
    "st cancel" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default();
      let list = if opt_b(path, map, "keep")? {
        ListEdit::Suppressed
      } else {
        ListEdit::AsDeclared
      };
      let on = opt_s(path, map, "date")?;
      Ok(outcome_json(
        &f.st_cancel_listing(&id, reason, list, on)?,
        &id,
      ))
    }
    "st hydrate" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let address = promote(path, &id)?;
      let paths = f.hydrate(&address)?;
      let rel: Vec<String> = paths.iter().map(|p| f.project().relative(p)).collect();
      // `exists`, not `wrote` -- the facade's own distinction: hydrate is
      // idempotent in both steps and returns the paths that NOW exist.
      Ok(json!({ "address": address.to_url(), "exists": rel }))
    }
    "st dehydrate" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let address = promote(path, &id)?;
      let done = f.dehydrate(&address)?;
      let rel = |paths: &[std::path::PathBuf]| -> Vec<String> {
        paths.iter().map(|p| f.project().relative(p)).collect()
      };
      Ok(json!({
        "address": address.to_url(),
        "unlisted": done.unlisted,
        "removed": rel(&done.removed),
        "pruned": rel(&done.pruned),
      }))
    }
    "st list" => {
      // Absent means the CLI's bare default -- WIP only, which is NOT
      // `status: "all"` (issue 0019's distinction, kept on this face).
      let wanted = match opt_s(path, map, "status")? {
        Some(raw) => crate::render::status_filter(raw).map_err(|why| args_err(path, why))?,
        None => Some(vec![ThreadStatus::Wip]),
      };
      let rows: Vec<&intentsvcs::model::Thread> = f
        .st_list()
        .into_iter()
        .filter(|t| wanted.as_ref().is_none_or(|w| w.contains(&t.status)))
        .collect();
      Ok(json!({ "threads": val(path, &rows)? }))
    }
    "st show" => {
      // `file` is a published arg this tool deliberately does not serve: it
      // selects a file view for the edit surfaces, and this tool returns the
      // WHOLE thread. Accepting it in silence would be the accept-and-ignore
      // shape; refusing names the working alternative.
      if map.contains_key("file") {
        return Err(args_err(
          path,
          "`file` is not served here -- the tool returns the whole thread; `intent_st_edit` resolves a file view to its path",
        ));
      }
      let id = spec(path, need_s(path, map, "id")?)?;
      let t = f.st_show(&id)?;
      val(path, t)
    }
    "st edit" => {
      let id = spec(path, need_s(path, map, "id")?)?;
      let file = opt_s(path, map, "file")?.unwrap_or("info").to_string();
      // The declared vocabulary, from the table -- the same home the CLI arm
      // reads, so the two doors cannot drift about what a file view is.
      let permitted = crate::dispatch::arg_values(&table, "edit", "file");
      if !permitted.is_empty() && !permitted.iter().any(|v| v == &file) {
        return Err(args_err(
          path,
          format!(
            "`{file}` is not a file this verb can open -- name one of {}",
            permitted.join(", ")
          ),
        ));
      }
      let address = promote(path, &id)?;
      // AC-05.3: the ONE door from (entity, file) to a path. Calling it keeps
      // the `facade.edit` construction-site count at one.
      let resolved = crate::render::artefact_path(f, &address, &file)?;
      // `{path}` ALWAYS -- there is no terminal here to open an editor on.
      Ok(json!({ "path": resolved.display().to_string() }))
    }
    "st sync" => {
      // The CLI's bare form renders a preview TABLE -- terminal furniture with
      // read tools already covering the question. Serving only the write keeps
      // this tool's one meaning; the refusal names the readers.
      if !opt_b(path, map, "write")? {
        return Err(args_err(
          path,
          "without `write: true` this verb only renders a terminal preview table -- for reads use `intent_st_list` / `intent_st_show`; with `write: true` it persists the index and views",
        ));
      }
      let count = f.sync_to_disk(&intentsvcs::sync::Scope::All)?;
      let index = f.project().relative(&f.project().steel_threads_view());
      Ok(json!({ "updated": index, "threads": count }))
    }

    // ----- work packages -----
    "wp new" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let title = need_s(path, map, "title")?;
      // S by default, same as the CLI: sizing happens after the package is
      // written (hv, issue 0052), and `wp rescope` is the exit.
      let seq = f.wp_new(&st, title, intentsvcs::model::DEFAULT_WP_SCOPE)?;
      Ok(json!({ "created": format!("{st}/{seq:02}") }))
    }
    "wp start" => {
      let (st, seq) = wp_scope(path, map)?;
      Ok(outcome_json(
        &f.wp_start(&st, seq)?,
        &format!("{st}/{seq:02}"),
      ))
    }
    "wp done" => {
      let (st, seq) = wp_scope(path, map)?;
      Ok(outcome_json(
        &f.wp_done(&st, seq)?,
        &format!("{st}/{seq:02}"),
      ))
    }
    "wp unstart" => {
      let (st, seq) = wp_scope(path, map)?;
      Ok(outcome_json(
        &f.wp_unstart(&st, seq)?,
        &format!("{st}/{seq:02}"),
      ))
    }
    "wp reopen" => {
      let (st, seq) = wp_scope(path, map)?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default().to_string();
      Ok(outcome_json(
        &f.wp_reopen(&st, seq, &reason)?,
        &format!("{st}/{seq:02}"),
      ))
    }
    "wp cancel" => {
      let (st, seq) = wp_scope(path, map)?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default().to_string();
      Ok(outcome_json(
        &f.wp_cancel(&st, seq, &reason)?,
        &format!("{st}/{seq:02}"),
      ))
    }
    "wp reinstate" => {
      let (st, seq) = wp_scope(path, map)?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default().to_string();
      Ok(outcome_json(
        &f.wp_reinstate(&st, seq, &reason)?,
        &format!("{st}/{seq:02}"),
      ))
    }
    "wp rescope" => {
      let (st, seq) = wp_scope(path, map)?;
      let raw = need_s(path, map, "size")?;
      let size = crate::render::t_shirt(raw).map_err(|e| failure_args(path, e))?;
      Ok(outcome_json(
        &f.wp_rescope(&st, seq, size)?,
        &format!("{st}/{seq:02}"),
      ))
    }
    "wp list" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      val(path, f.wp_list(&st)?)
    }
    "wp show" => {
      let (st, seq) = wp_scope(path, map)?;
      val(path, f.wp_show(&st, seq)?)
    }

    // ----- acceptance criteria -----
    "ac list" => {
      let target = spec(path, need_s(path, map, "stid")?)?;
      let (st, _) = crate::render::scope_of(&target);
      let rows: Vec<Value> = f
        .ac_list(&st)?
        .iter()
        .map(|r| {
          json!({
            "id": r.id, "text": r.text, "state": r.state, "covered_by": r.covered_by,
          })
        })
        .collect();
      Ok(json!({ "criteria": rows }))
    }
    "ac gate" => {
      let target = spec(path, need_s(path, map, "stid")?)?;
      let (st, scope) = crate::render::scope_of(&target);
      let verdict = f.gate(&st, scope)?;
      Ok(verdict_json(&verdict, verdict.line(&target)))
    }
    "ac status" => {
      let target = spec(path, need_s(path, map, "stid")?)?;
      let (st, scope) = crate::render::scope_of(&target);
      let verdict = f.gate(&st, scope)?;
      // `status_line`, like the CLI: the count-first shape, exit-code-free.
      Ok(verdict_json(&verdict, verdict.status_line()))
    }
    "ac new" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      let text = need_s(path, map, "text")?;
      let kind = match opt_s(path, map, "kind")? {
        Some("test") => AcKind::Test,
        None | Some("non-test") => AcKind::NonTest,
        Some(other) => {
          return Err(args_err(
            path,
            format!("`{other}` is not a criterion kind -- expected `test` or `non-test`"),
          ));
        }
      };
      Ok(outcome_json(&f.ac_new(&st, id, text, kind)?, id))
    }
    "ac edit" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      let text = need_s(path, map, "text")?;
      Ok(outcome_json(&f.ac_edit(&st, id, text)?, id))
    }
    "ac satisfy" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      let evidence = opt_s(path, map, "evidence")?.unwrap_or_default();
      Ok(outcome_json(&f.ac_satisfy(&st, id, evidence)?, id))
    }
    "ac unsatisfy" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      Ok(outcome_json(&f.ac_unsatisfy(&st, id)?, id))
    }
    "ac descope" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      let to = opt_s(path, map, "to")?.unwrap_or_default();
      let by = opt_s(path, map, "by")?;
      let reason = opt_s(path, map, "reason")?;
      Ok(outcome_json(&f.ac_descope(&st, id, to, by, reason)?, id))
    }
    "ac withdraw" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      let reason = opt_s(path, map, "reason")?.unwrap_or_default();
      let by = opt_s(path, map, "by")?;
      Ok(outcome_json(&f.ac_withdraw(&st, id, reason, by)?, id))
    }
    "ac rescope" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      Ok(outcome_json(&f.ac_rescope(&st, id)?, id))
    }
    "ac reinstate" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "acid")?;
      Ok(outcome_json(&f.ac_reinstate(&st, id)?, id))
    }

    // ----- acceptance tests -----
    "at list" => {
      let target = spec(path, need_s(path, map, "stid")?)?;
      let (st, _) = crate::render::scope_of(&target);
      Ok(json!({ "tests": val(path, f.at_list(&st)?)? }))
    }
    "at lint" => {
      let target = spec(path, need_s(path, map, "stid")?)?;
      let (st, _) = crate::render::scope_of(&target);
      let report = f.at_lint(&st)?;
      Ok(json!({ "findings": report.findings, "rows": report.rows }))
    }
    "at green" | "at red" | "at na" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "atid")?;
      let status = match path {
        "at green" => AtStatus::Green,
        "at red" => AtStatus::Red,
        _ => AtStatus::Na,
      };
      let note = opt_s(path, map, "note")?.map(str::to_string);
      Ok(outcome_json(&f.at_set(&st, id, status, note)?, id))
    }
    "at new" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "atid")?;
      let covers = strings(path, map, "covers")?.unwrap_or_default();
      let kind = match opt_s(path, map, "kind")? {
        None | Some("test") => intentsvcs::model::AtKind::Test,
        Some("non-test") => intentsvcs::model::AtKind::NonTest,
        Some(other) => {
          return Err(args_err(
            path,
            format!("`{other}` is not an acceptance-test kind -- expected `test` or `non-test`"),
          ));
        }
      };
      let status = match opt_s(path, map, "status")? {
        None | Some("to-write") => AtStatus::ToWrite,
        Some("red") => AtStatus::Red,
        Some("green") => AtStatus::Green,
        Some("n-a") | Some("n/a") => AtStatus::Na,
        Some(other) => {
          return Err(args_err(
            path,
            format!(
              "`{other}` is not an acceptance-test status -- expected `to-write`, `red`, `green` or `n/a`"
            ),
          ));
        }
      };
      let file = opt_s(path, map, "file")?.map(str::to_string);
      let prose = opt_s(path, map, "prose")?.map(str::to_string);
      let note = opt_s(path, map, "note")?.map(str::to_string);
      Ok(outcome_json(
        &f.at_new(&st, id, kind, file, prose, covers, status, note)?,
        id,
      ))
    }
    "at edit" => {
      let st = spec(path, need_s(path, map, "stid")?)?;
      let id = need_s(path, map, "atid")?;
      let file = opt_s(path, map, "file")?.map(str::to_string);
      let prose = opt_s(path, map, "prose")?.map(str::to_string);
      // `None` when absent -- the facade reads absence as "not saying", and an
      // empty vec would silently clear the row's coverage (the arm's contract,
      // kept on this face).
      let covers = strings(path, map, "covers")?;
      Ok(outcome_json(&f.at_edit(&st, id, file, prose, covers)?, id))
    }

    // ----- issues -----
    "issues list" => {
      let kind = opt_s(path, map, "kind")?.unwrap_or("open");
      let wanted = match kind.to_ascii_lowercase().as_str() {
        "open" => Some(IssueStatus::Open),
        "closed" => Some(IssueStatus::Closed),
        "all" => None,
        other => {
          return Err(args_err(
            path,
            format!("`{other}` is not an issue bucket -- use one of open, closed, all"),
          ));
        }
      };
      let rows: Vec<&intentsvcs::model::Issue> = f
        .issue_list()
        .into_iter()
        .filter(|i| wanted.is_none_or(|w| i.status == w))
        .collect();
      Ok(json!({ "issues": val(path, &rows)? }))
    }
    "issues show" => {
      let number = issue_number(path, need_s(path, map, "id")?)?;
      val(path, f.issue_show(number)?)
    }
    "issues add" => {
      let title = need_s(path, map, "title")?;
      let severity = opt_s(path, map, "severity")?;
      if let Some(bad) = severity
        && intentsvcs::model::IssueSeverity::parse(bad).is_none()
      {
        return Err(args_err(
          path,
          format!(
            "`{bad}` is not an issue severity -- name one of {}",
            intentsvcs::model::IssueSeverity::SPELLINGS.join(", ")
          ),
        ));
      }
      let body = opt_s(path, map, "body")?.unwrap_or_default();
      // Reporter is deliberately `None`: the CLI guesses one from the host's
      // environment, and a guess made on the server about a remote agent would
      // be a fabricated attribution. Absent renders as "nobody said".
      let number = f.issue_add(title, severity, None, body)?;
      Ok(json!({ "created": format!("{number:04}") }))
    }
    "issues close" => {
      let number = issue_number(path, need_s(path, map, "id")?)?;
      Ok(outcome_json(
        &f.issue_close(number)?,
        &format!("{number:04}"),
      ))
    }
    "issues open" => {
      let number = issue_number(path, need_s(path, map, "id")?)?;
      Ok(outcome_json(
        &f.issue_open(number)?,
        &format!("{number:04}"),
      ))
    }

    // ----- todo -----
    "todo" | "todo list" => val(path, &f.todo_buckets()?),
    "todo update" => {
      f.todo_update()?;
      Ok(json!({ "ok": true, "regenerated": "todo.md" }))
    }

    // ----- doctor / agents -----
    // **THE ESCAPE HATCH: SHIPPED TO intentd, NEVER EXECUTED HERE.** The open
    // facade is touched for its root and nothing else -- a tool that ran the
    // document in this process would be the async runtime arriving through the
    // back door, which is the one thing the zero-dependency ruling forbids
    // (`hatch.rs` carries the whole argument). The arguments are refused
    // BEFORE any daemon is looked for, which is what keeps the roster drive
    // deterministic on a machine with or without one running.
    "graphql" => {
      let query = need_s(path, map, "query")?;
      let variables = crate::hatch::variables(opt_s(path, map, "variables")?)
        .map_err(|e| args_err(path, e.to_string()))?;
      Ok(crate::hatch::graphql(f.project().root(), query, variables)?)
    }
    // The shape of the entities an agent is about to manipulate -- vc's need
    // test says yes to this row by name. Store-free: the faces are generated
    // from the types, and `Facade::schema` is the door onto `faces::schema`,
    // the same home the global terminal verb answers from.
    "schema" => {
      let face = opt_s(path, map, "face")?;
      let faces = f.schema(face)?;
      if opt_b(path, map, "versions")? {
        val(
          path,
          &faces
            .iter()
            .map(|x| {
              json!({"face": x.name, "intent_ver": x.intent_ver, "key": x.key, "contract_ver": x.contract_ver})
            })
            .collect::<Vec<_>>(),
        )
      } else {
        val(
          path,
          &faces
            .iter()
            .map(|x| json!({"name": x.name, "content": x.content}))
            .collect::<Vec<_>>(),
        )
      }
    }
    "doctor" => {
      let report = Facade::doctor(f.project(), ctx, Some(f.store()));
      Ok(crate::render::doctor_json(&report))
    }
    "agents generate" => Ok(json!({ "content": f.agents_generate()? })),
    "agents validate" => {
      use intentsvcs::rootfiles::AgentsFileState as S;
      let report = f.agents_validate();
      let sections: Vec<Value> = report
        .sections
        .iter()
        .map(|(name, present)| json!({ "name": name, "present": present }))
        .collect();
      Ok(json!({
        "state": match report.state {
          S::Missing => "missing",
          S::Symlink => "symlink",
          S::NotRegular => "not-regular",
          S::Regular => "regular",
        },
        "sections": sections,
        "errors": report.errors(),
        "warnings": report.warnings(),
        "valid": report.errors() == 0,
      }))
    }

    // ----- the new-surface four -----
    "search" => {
      let query = need_s(path, map, "query")?;
      let hits = f.search(query)?;
      // The AC-06.4 distinction travels: an empty result over an unpopulated
      // index is NOT a miss, and only this side knows which happened.
      let note = if hits.is_empty() && f.prose_sections_indexed()? == 0 {
        Some(
          "nothing is indexed, so this search could not have matched -- an empty result here does NOT mean the phrase is absent",
        )
      } else {
        None
      };
      Ok(json!({ "hits": val(path, &hits)?, "note": note }))
    }
    "export" => {
      let format = opt_s(path, map, "format")?;
      match f.export(format)? {
        Exported::Document(text) => Ok(json!({ "document": text })),
        Exported::Realised(r) => {
          let counts = |c: &intentsvcs::realise::Counts| {
            json!({
              "threads": c.threads, "wps": c.wps, "issues": c.issues,
              "attachments": c.attachments, "views": c.views,
            })
          };
          Ok(json!({
            "realised": {
              "root": r.root.display().to_string(),
              "written": r.written.len(),
              "counts": counts(&r.counts),
              "totals": counts(&r.totals),
              "complete": r.complete(),
            }
          }))
        }
      }
    }
    "organize" => {
      // `--default` / `--force` are narrowed off this surface (the declaration
      // op ends in a terminal confirmation); the unknown-parameter check above
      // is what refuses them by name.
      let mode = if opt_b(path, map, "apply")? {
        intentsvcs::organize::Mode::Apply
      } else {
        intentsvcs::organize::Mode::Preview
      };
      let applied = mode == intentsvcs::organize::Mode::Apply;
      let report = f.organize(mode)?;
      let rel = |paths: &[std::path::PathBuf]| -> Vec<String> {
        paths.iter().map(|p| p.display().to_string()).collect()
      };
      Ok(json!({
        "applied": applied,
        "hydrated": rel(&report.hydrated),
        "rewritten": rel(&report.rewritten),
        "unchanged": rel(&report.unchanged),
        "dehydrated": rel(&report.dehydrated),
        "unclaimed": rel(&report.unclaimed),
        "diverged": rel(&report.diverged),
        "pruned": rel(&report.pruned),
        // Refusals travel WITH the act (never silent) -- each rendered through
        // its own remedy, the one rendering.
        "refused": report.refused.iter().map(|e| e.render()).collect::<Vec<_>>(),
      }))
    }
    "events" => {
      let limit = match map.get("limit") {
        None => None,
        Some(Value::String(n)) => Some(n.parse::<usize>().map_err(|_| {
          args_err(
            path,
            format!("`{n}` is not a count -- limit takes a whole number"),
          )
        })?),
        Some(Value::Number(n)) => match n.as_u64() {
          Some(v) => Some(v as usize),
          None => {
            return Err(args_err(
              path,
              format!("`{n}` is not a count -- limit takes a whole number"),
            ));
          }
        },
        Some(other) => {
          return Err(args_err(
            path,
            format!("`{other}` is not a count -- limit takes a whole number"),
          ));
        }
      };
      let filter = EventFilter {
        op: opt_s(path, map, "op")?.map(str::to_string),
        subject: opt_s(path, map, "subject")?.map(str::to_string),
        limit,
      };
      let page = f.events(&filter)?;
      Ok(crate::render::events_json(&page))
    }

    // The population check above admitted this path, so a fall-through here is
    // a roster/arm drift -- reported as the same UnknownTool the gate test
    // asserts against, never swallowed.
    _ => Err(ServeError::UnknownTool {
      path: path.to_string(),
    }),
  }
}

// ---------------------------------------------------------------------------
// serve()'s small vocabulary
// ---------------------------------------------------------------------------

fn args_err(path: &str, why: impl Into<String>) -> ServeError {
  ServeError::Args {
    path: path.to_string(),
    why: why.into(),
  }
}

/// A renderer-side refusal (`Failure`) rephrased as this surface's `Args`.
fn failure_args(path: &str, e: crate::spine::Failure) -> ServeError {
  args_err(path, e.message().unwrap_or("refused").to_string())
}

/// Required string parameter.
fn need_s<'a>(path: &str, map: &'a Map<String, Value>, key: &str) -> Result<&'a str, ServeError> {
  match map.get(key) {
    Some(Value::String(s)) => Ok(s),
    Some(other) => Err(args_err(
      path,
      format!("`{key}` must be a string, not {other}"),
    )),
    None => Err(args_err(path, format!("`{key}` is required"))),
  }
}

/// Optional string parameter -- absent is `None`, a non-string is refused.
fn opt_s<'a>(
  path: &str,
  map: &'a Map<String, Value>,
  key: &str,
) -> Result<Option<&'a str>, ServeError> {
  match map.get(key) {
    None | Some(Value::Null) => Ok(None),
    Some(Value::String(s)) => Ok(Some(s)),
    Some(other) => Err(args_err(
      path,
      format!("`{key}` must be a string, not {other}"),
    )),
  }
}

/// Optional boolean parameter -- absent is `false`, a non-bool is refused.
fn opt_b(path: &str, map: &Map<String, Value>, key: &str) -> Result<bool, ServeError> {
  match map.get(key) {
    None | Some(Value::Null) => Ok(false),
    Some(Value::Bool(b)) => Ok(*b),
    Some(other) => Err(args_err(
      path,
      format!("`{key}` must be true or false, not {other}"),
    )),
  }
}

/// Optional list-of-strings parameter. The schema publishes repeated flags as
/// `string`, so a single string is accepted as a one-element list.
fn strings(
  path: &str,
  map: &Map<String, Value>,
  key: &str,
) -> Result<Option<Vec<String>>, ServeError> {
  match map.get(key) {
    None | Some(Value::Null) => Ok(None),
    Some(Value::String(s)) => Ok(Some(vec![s.clone()])),
    Some(Value::Array(items)) => {
      let mut out = Vec::with_capacity(items.len());
      for item in items {
        match item {
          Value::String(s) => out.push(s.clone()),
          other => {
            return Err(args_err(
              path,
              format!("`{key}` must be a list of strings; `{other}` is not a string"),
            ));
          }
        }
      }
      Ok(Some(out))
    }
    Some(other) => Err(args_err(
      path,
      format!("`{key}` must be a string or a list of strings, not {other}"),
    )),
  }
}

/// A thread spelling, canonicalised through the ONE thread door.
fn spec(path: &str, raw: &str) -> Result<String, ServeError> {
  crate::render::thread_spec(raw).map_err(|e| failure_args(path, e))
}

/// A `<ST>/<NN>` spelling resolved to its work-package scope, refusing a bare
/// thread the way `wp_target` does -- same rule, this face's rendering.
fn wp_scope(path: &str, map: &Map<String, Value>) -> Result<(String, u32), ServeError> {
  let target = spec(path, need_s(path, map, "specifier")?)?;
  match crate::render::scope_of(&target) {
    (st, Scope::WorkPackage(seq)) => Ok((st, seq)),
    _ => Err(args_err(
      path,
      format!("`{target}` is not a work package -- name it as `<ST id>/<NN>`, eg ST0000/03"),
    )),
  }
}

/// An issue number, through the model's own normaliser.
fn issue_number(path: &str, raw: &str) -> Result<u32, ServeError> {
  intentsvcs::model::normalise_issue_id(raw).map_err(|_| {
    args_err(
      path,
      format!("`{raw}` is not an issue number -- name it as digits, eg 0000"),
    )
  })
}

/// An address promoted from a bare id -- the caller's spelling, so a refusal
/// is an `Args` error carrying the promoter's own rendering.
fn promote(path: &str, raw: &str) -> Result<intentsvcs::address::Address, ServeError> {
  intentsvcs::address::promote(raw).map_err(|e| args_err(path, e.render()))
}

/// `serde_json::to_value` with the failure named. It cannot fail on these
/// derive-serialised types in practice; if it ever does, the message says
/// build defect rather than blaming the caller's arguments.
fn val<T: serde::Serialize + ?Sized>(path: &str, value: &T) -> Result<Value, ServeError> {
  serde_json::to_value(value).map_err(|e| {
    args_err(
      path,
      format!("build defect -- the result could not be serialised: {e}"),
    )
  })
}

/// What a mutating verb DID, in the projection the board ruled:
/// `moved` / `already` / `notes[]`, each note structural rather than prose.
/// The notes TRAVEL -- `Outcome::MovedWith` exists precisely so a non-CLI
/// caller cannot skip them in silence.
fn outcome_json(outcome: &Outcome, subject: &str) -> Value {
  let notes: Vec<Value> = outcome
    .notes()
    .iter()
    .map(|note| match note {
      Note::UnsyncedAttachments(paths) => json!({
        "kind": "unsynced-attachments", "paths": paths,
      }),
      Note::FiatClosedSoleCover(acs) => json!({
        "kind": "fiat-closed-sole-cover", "criteria": acs,
      }),
      Note::UnsyncedUnknown => json!({ "kind": "unsynced-unknown" }),
    })
    .collect();
  json!({
    "subject": subject,
    "moved": outcome.moved(),
    "already": outcome.already(),
    "notes": notes,
  })
}

/// The gate's verdict, projected: the word, the fiat count, the waiting ids,
/// and the face's own line (`line` for `ac gate`, `status_line` for
/// `ac status` -- the caller picks, this shape carries it).
fn verdict_json(verdict: &Verdict, line: String) -> Value {
  let word = match verdict {
    Verdict::Pass { .. } => "pass",
    Verdict::Exempt { .. } => "exempt",
    Verdict::Blocked { .. } => "blocked",
  };
  json!({
    "verdict": word,
    "fiat": match verdict {
      Verdict::Pass { fiat, .. } => *fiat,
      _ => 0,
    },
    "unsatisfied": match verdict {
      Verdict::Blocked { unsatisfied, .. } => unsatisfied.clone(),
      _ => Vec::new(),
    },
    "line": line,
  })
}

// ---------------------------------------------------------------------------
// MCP resources (AC-09.5)
//
// A resource is a model entity read through its EXISTING facade door and
// rendered by `crate::show` -- so its contents ARE the CLI read, byte for byte,
// rather than a second rendering that agrees today. The URI kind is the
// nav/address grammar's own (`thread`, `wp`, `issue`), never a fourth spelling:
// a resource URI is one more face onto the one path grammar. wip.md and the
// whiteboard boards are deliberately NOT resources -- they have no facade door
// and no CLI read to match, so serving them would assert agreement with
// nothing (the reworded AC-09.5, and the design at
// `intent/st/ST0056/parity/ac-09_5-resources-design.md`).
// ---------------------------------------------------------------------------

/// One resource the server serves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
  pub uri: String,
  pub name: String,
}

/// Why a `resources/read` could not be answered. `NotFound` is the facade's own
/// refusal; the two `BadUri` shapes are the URI never naming a resource.
#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
  #[error("`{uri}`: {why}")]
  BadUri { uri: String, why: String },
  #[error(transparent)]
  NotFound(#[from] FacadeError),
}

impl ResourceError {
  /// The operator-facing text, mirroring [`ServeError::render`].
  pub fn render(&self) -> String {
    match self {
      Self::NotFound(e) => e.render(),
      other => other.to_string(),
    }
  }
}

/// Every concrete resource, enumerated from the facade: one per thread, work
/// package and issue. The name is the first line the CLI read prints, so a
/// listing reads the way `st list` does.
pub fn resource_list(f: &Facade) -> Vec<Resource> {
  let mut out = Vec::new();
  for t in f.st_list() {
    out.push(Resource {
      uri: format!("intent:///thread/{}", t.id),
      name: format!("{}: {}", t.id, t.title),
    });
    for wp in &t.wps {
      out.push(Resource {
        uri: format!("intent:///wp/{}/{}", t.id, wp.seq),
        name: format!("{}/WP-{:02}: {}", t.id, wp.seq, wp.title),
      });
    }
  }
  for i in f.issue_list() {
    out.push(Resource {
      uri: format!("intent:///issue/{:04}", i.number),
      name: format!("{:04}: {}", i.number, i.title),
    });
  }
  out
}

/// Read one resource by URI: parse the nav-grammar path, take the facade door,
/// render with `crate::show`. The returned text is byte-identical to the
/// equivalent `intent … show` -- the same function renders both faces.
pub fn resource_read(f: &Facade, uri: &str) -> Result<String, ResourceError> {
  let bad = |why: &str| ResourceError::BadUri {
    uri: uri.to_string(),
    why: why.to_string(),
  };
  let rest = uri
    .strip_prefix("intent:///")
    .ok_or_else(|| bad("a resource uri begins `intent:///`"))?;
  let mut parts = rest.split('/').filter(|s| !s.is_empty());
  let kind = parts.next().unwrap_or("");
  match kind {
    "thread" => {
      let id = parts
        .next()
        .ok_or_else(|| bad("a thread uri is `intent:///thread/<id>`"))?;
      Ok(crate::show::thread(f.st_show(id)?))
    }
    "wp" => {
      let st = parts
        .next()
        .ok_or_else(|| bad("a work-package uri is `intent:///wp/<st>/<seq>`"))?;
      let seq = parts
        .next()
        .ok_or_else(|| bad("a work-package uri is `intent:///wp/<st>/<seq>`"))?
        .parse::<u32>()
        .map_err(|_| bad("the work-package sequence must be a whole number"))?;
      Ok(crate::show::work_package(st, f.wp_show(st, seq)?))
    }
    "issue" => {
      let number = parts
        .next()
        .ok_or_else(|| bad("an issue uri is `intent:///issue/<number>`"))?
        .parse::<u32>()
        .map_err(|_| bad("the issue number must be a whole number"))?;
      Ok(crate::show::issue(f.issue_show(number)?))
    }
    other => Err(bad(&format!(
      "`{other}` is not a resource kind -- resources are `thread`, `wp` and `issue`"
    ))),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::dispatch;

  fn all() -> Vec<Tool> {
    tools(&dispatch::table()).expect("the committed table generates every declared tool")
  }

  /// **THE POPULATION IS DERIVED, NEVER PINNED**: exactly the rows declaring
  /// both `exposed_on_mcp` and `facade` become tools -- so a row gaining its
  /// door joins by regeneration, and a narrow leaves the same way.
  #[test]
  fn every_exposed_row_with_a_door_is_a_tool_and_nothing_else_is() {
    let table = dispatch::table();
    let expected: Vec<&str> = dispatch::shipped_entries(&table)
      .into_iter()
      .filter(|e| e.exposed_on_mcp && e.facade.is_some())
      .map(|e| e.path.as_str())
      .collect();
    let got: Vec<String> = all().iter().map(|t| t.path.clone()).collect();
    assert_eq!(
      got,
      expected.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
      "the tool population and the table's declaration disagree"
    );
    assert!(
      got.len() > 40,
      "only {} tools generated, which is too few to be the read's 59 -- the population query \
       has stopped reading the fields",
      got.len()
    );
  }

  #[test]
  fn tool_names_are_unique_and_legal() {
    let tools = all();
    let mut names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    for n in &names {
      assert!(
        n.starts_with("intent_")
          && n
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
        "tool name `{n}` is outside the MCP-safe charset"
      );
    }
    names.sort_unstable();
    let before = names.len();
    names.dedup();
    assert_eq!(before, names.len(), "two rows generated the same tool name");
  }

  /// **THE SAFETY FACT LEADS OR RIDES EVERY DESCRIPTION** -- `AC-09.1`'s
  /// extension: the identical destructive operation must not warn a CLI agent
  /// and say nothing to an MCP agent. Driven on the sharpest row the table
  /// carries: `at green` is `one-way` because issue 0033 destroys the row's
  /// authored note.
  #[test]
  fn descriptions_carry_the_d45_projection_and_one_way_says_so() {
    let tools = all();
    for t in &tools {
      assert!(
        t.description.contains("`read`") || t.description.contains("`mutate`"),
        "`{}`'s description carries no safety fact: {:?}",
        t.name,
        t.description
      );
      if t.read_or_mutate == "mutate" {
        assert!(
          t.description.contains("reversible")
            || t.description.contains("idempotent")
            || t.description.contains("ONE-WAY"),
          "mutation `{}` says nothing about recoverability: {:?}",
          t.name,
          t.description
        );
      }
    }
    let green = tools
      .iter()
      .find(|t| t.path == "at green")
      .expect("at green is exposed with a door");
    assert!(
      green.description.contains("ONE-WAY"),
      "the one-way mutation must SAY one-way to the surface with less context: {:?}",
      green.description
    );
  }

  /// Every declared arg and every kept flag reaches the schema; required is
  /// exactly the arity-1 args; enums carry their values whichever of the
  /// three corpus spellings declared them.
  #[test]
  fn schemas_carry_every_declared_parameter() {
    let table = dispatch::table();
    let by_path: std::collections::BTreeMap<String, &crate::dispatch::Entry> =
      dispatch::shipped_entries(&table)
        .into_iter()
        .map(|e| (e.path.clone(), e))
        .collect();
    let mut enums = 0usize;
    for t in all() {
      let entry = by_path[&t.path];
      let props = t.input_schema["properties"]
        .as_object()
        .expect("schema has properties");
      let required: Vec<&str> = t.input_schema["required"]
        .as_array()
        .expect("schema has required")
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
      for a in entry.args.iter().filter(|a| super::published_arg(a)) {
        assert!(
          props.contains_key(&a.name),
          "`{}`: arg `{}` missing from the schema",
          t.path,
          a.name
        );
        assert_eq!(
          a.arity == "1",
          required.contains(&a.name.as_str()),
          "`{}`: arg `{}` required-ness disagrees with its arity",
          t.path,
          a.name
        );
        if a.kind == "enum" {
          enums += 1;
          assert!(
            props[&a.name]["enum"].is_array(),
            "`{}`: enum arg `{}` published without its values",
            t.path,
            a.name
          );
        }
      }
      for f in entry.flags.iter().filter(|f| super::published_flag(f)) {
        let name = super::flag_name(f).expect("kept flags have long spellings");
        assert!(
          props.contains_key(&name),
          "`{}`: flag `--{name}` missing from the schema",
          t.path
        );
        if f.kind == "enum" {
          enums += 1;
          assert!(
            props[&name]["enum"].is_array(),
            "`{}`: enum flag `--{name}` published without its values",
            t.path
          );
        }
      }
    }
    assert!(
      enums > 5,
      "only {enums} enums swept; the corpus declares more, so the sweep is not reaching them"
    );
  }

  /// **D37, ON THE NEW SURFACE, AS A SWEEP**: no Intent-internal tracker id
  /// in anything a tool publishes. The discrimination follows the guide's own
  /// shipped precedent (`guide.rs` emits `ST0000` in its example error): a
  /// ZERO id is a format illustration -- information about the OPERATOR's
  /// project -- while any specific id is information about us, which is the
  /// leak D37 names ("(ST0056 WP-06)" was the real instance). The sweep
  /// therefore bans specific-looking ids and admits exactly the zero family.
  #[test]
  fn no_tracker_id_reaches_a_published_tool() {
    let specific = |text: &str| {
      let bytes: Vec<char> = text.chars().collect();
      let digits_at = |i: usize, n: usize| {
        (0..n).all(|k| bytes.get(i + k).map(char::is_ascii_digit) == Some(true))
      };
      // ST followed by four digits that are not 0000; WP-/AC-/AT- followed by
      // any digit sequence containing a nonzero digit before the next
      // non-id character.
      for i in 0..bytes.len() {
        if bytes[i] == 'S' && bytes.get(i + 1) == Some(&'T') && digits_at(i + 2, 4) {
          let id: String = bytes[i + 2..i + 6].iter().collect();
          if id != "0000" {
            return Some(format!("ST{id}"));
          }
        }
        for tag in ["WP-", "AC-", "AT-"] {
          let t: Vec<char> = tag.chars().collect();
          if bytes[i..].starts_with(&t) {
            let rest: String = bytes[i + t.len()..]
              .iter()
              .take_while(|c| c.is_ascii_digit() || **c == '.')
              .collect();
            if rest.chars().any(|c| c.is_ascii_digit() && c != '0') {
              return Some(format!("{tag}{rest}"));
            }
          }
        }
      }
      None
    };
    // The detector's own controls: it must SEE a leak and PASS the zero forms.
    assert_eq!(specific("gate ST0056 blocked"), Some("ST0056".into()));
    assert_eq!(specific("per AC-1.2"), Some("AC-1.2".into()));
    assert_eq!(specific("eg ST0000 or AC-0.0 or AT-0.0"), None);
    for t in all() {
      let published = format!("{} {} {}", t.name, t.description, t.input_schema);
      if let Some(leak) = specific(&published) {
        panic!(
          "tool `{}` publishes an Intent tracker id (`{leak}`): D37 -- a zero id is a format            example, a specific one is information about us",
          t.name
        );
      }
    }
  }

  /// **THE TWO-SIDED GATE.** The SERVED roster and the generated tool
  /// population are asserted equal in BOTH directions -- a row gaining its
  /// door without an arm, or an arm outliving its row, is a named diff here
  /// rather than a silent gap. `assert_eq` on sorted vecs IS both directions.
  #[test]
  fn the_roster_and_the_tool_population_agree_both_ways() {
    let mut tools: Vec<String> = all().iter().map(|t| t.path.clone()).collect();
    let mut served: Vec<String> = SERVED.iter().map(|s| s.to_string()).collect();
    tools.sort_unstable();
    served.sort_unstable();
    assert_eq!(
      served, tools,
      "the serving roster and the generated tool population disagree -- left is SERVED, right is tools()"
    );
  }

  /// A project the drive below can open: the same config shape
  /// `intentsvcs/tests/common` lays down, discovered from its root.
  fn fixture() -> (tempfile::TempDir, Facade) {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = dir.path().join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir .config");
    std::fs::write(
      config.join("config.json"),
      "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"ic\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
    )
    .expect("write config");
    let project = intentsvcs::project::Project::open(dir.path()).expect("open the fixture project");
    let ctx = FacadeContext {
      principal: "ic".to_string(),
      project_id: "00000000-0000-0000-0000-000000000000".to_string(),
      version: env!("CARGO_PKG_VERSION").to_string(),
    };
    let facade = Facade::open_in_memory(project, ctx).expect("open the in-memory facade");
    (dir, facade)
  }

  fn drive_ctx() -> FacadeContext {
    FacadeContext {
      principal: "ic".to_string(),
      project_id: "00000000-0000-0000-0000-000000000000".to_string(),
      version: env!("CARGO_PKG_VERSION").to_string(),
    }
  }

  /// **EVERY ROSTER PATH REACHES AN ARM.** Driven through a real in-memory
  /// facade with empty arguments: most calls refuse on a missing parameter or
  /// an empty estate, and every one of those answers is fine -- the one
  /// answer that fails this test is `UnknownTool`, which would mean the
  /// roster names a path the match does not serve.
  #[test]
  fn every_roster_path_reaches_an_arm() {
    let (_dir, mut facade) = fixture();
    let ctx = drive_ctx();
    for path in SERVED {
      let got = serve(&mut facade, &ctx, path, &json!({}));
      assert!(
        !matches!(got, Err(ServeError::UnknownTool { .. })),
        "`{path}` is on the SERVED roster and the match answered UnknownTool"
      );
    }
  }

  /// The population is the TABLE's: a path outside it is unknown, and so is a
  /// real command narrowed off this surface (`info`) -- the negative control
  /// proving the gate above can fail.
  #[test]
  fn a_path_off_the_roster_is_unknown() {
    let (_dir, mut facade) = fixture();
    let ctx = drive_ctx();
    for path in ["no such tool", "info", "st attach"] {
      assert!(
        matches!(
          serve(&mut facade, &ctx, path, &json!({})),
          Err(ServeError::UnknownTool { .. })
        ),
        "`{path}` must be UnknownTool on this surface"
      );
    }
  }

  /// `intent_schema` answers from the faces the types generate, per face or
  /// all, refuses an unknown face by name, and carries the two version markers
  /// when asked for versions instead of bodies.
  #[test]
  fn the_schema_tool_answers_from_the_faces_and_refuses_an_unknown_one() {
    let (_dir, mut facade) = fixture();
    let ctx = drive_ctx();

    let one =
      serve(&mut facade, &ctx, "schema", &json!({"face": "ddl.sql"})).expect("a known face");
    assert_eq!(one.as_array().map(Vec::len), Some(1), "{one}");
    assert_eq!(one[0]["name"], "ddl.sql");
    assert_eq!(
      one[0]["content"].as_str(),
      intentsvcs::faces::face("ddl.sql").as_deref(),
      "the tool's content is the face's, byte for byte"
    );

    let all = serve(&mut facade, &ctx, "schema", &json!({})).expect("every face");
    assert_eq!(
      all.as_array().map(Vec::len),
      Some(intentsvcs::faces::face_names().len()),
      "{all}"
    );

    let versions = serve(&mut facade, &ctx, "schema", &json!({"versions": true})).expect("markers");
    for row in versions.as_array().expect("a list") {
      assert_eq!(row["intent_ver"], intentsvcs::faces::INTENT_VER, "{row}");
      assert!(
        row["contract_ver"].as_str().is_some_and(|v| !v.is_empty()),
        "{row}"
      );
    }

    let unknown = serve(&mut facade, &ctx, "schema", &json!({"face": "not-a-face"}));
    assert!(
      matches!(
        unknown,
        Err(ServeError::Refused(
          intentsvcs::facade::FacadeError::NoSuchFace { .. }
        ))
      ),
      "{unknown:?}"
    );
  }

  /// The hatch's arguments are refused BEFORE any daemon is looked for, so
  /// this drive is deterministic on a machine with or without one running --
  /// and neither refusal is `UnknownTool`, which is what the roster gate needs.
  #[test]
  fn the_hatch_refuses_its_arguments_before_reaching_for_a_daemon() {
    let (_dir, mut facade) = fixture();
    let ctx = drive_ctx();
    let missing = serve(&mut facade, &ctx, "graphql", &json!({}));
    assert!(
      matches!(&missing, Err(ServeError::Args { why, .. }) if why.contains("query")),
      "{missing:?}"
    );
    let bad = serve(
      &mut facade,
      &ctx,
      "graphql",
      &json!({"query": "{ threads { id } }", "variables": "[1]"}),
    );
    assert!(
      matches!(&bad, Err(ServeError::Args { why, .. }) if why.contains("--variables")),
      "{bad:?}"
    );
  }

  /// An unknown parameter is refused by name, never accepted-and-ignored --
  /// which is also what keeps a narrowed flag (`st list --width`,
  /// `organize --force`) OFF this surface rather than silently swallowed.
  #[test]
  fn an_unknown_parameter_is_refused_by_name() {
    let (_dir, mut facade) = fixture();
    let ctx = drive_ctx();
    for (path, args) in [
      ("st list", json!({ "width": "80" })),
      ("organize", json!({ "force": true })),
      ("doctor", json!({ "fix": true })),
    ] {
      match serve(&mut facade, &ctx, path, &args) {
        Err(ServeError::Args { why, .. }) => {
          assert!(
            why.contains("no parameter named"),
            "`{path}` refused for another reason: {why}"
          );
        }
        other => panic!("`{path}` with an unknown parameter answered {other:?}"),
      }
    }
  }

  /// **THE POSITIVE CONTROL ON THE DISPOSITION FILTER.** Its predecessor
  /// skipped `disposition == "drop"` against a vocabulary with no `drop` --
  /// it matched zero flags forever and published retired and intrinsic flags
  /// (`intent_doctor` advertised `fix` and `help`). This drives the filter at
  /// both poles: the corpus must CONTAIN non-keep flags on exposed rows, and
  /// none of them may reach a schema.
  #[test]
  fn the_keep_filter_is_live_and_non_keep_flags_stay_unpublished() {
    let table = dispatch::table();
    let mut non_keep_on_exposed = 0usize;
    for entry in dispatch::shipped_entries(&table)
      .into_iter()
      .filter(|e| e.exposed_on_mcp && e.facade.is_some())
    {
      for flag in entry.flags.iter().filter(|f| !f.ships()) {
        non_keep_on_exposed += 1;
        let name = super::flag_name(flag).expect("non-keep flags still spell a long name");
        let tool = all()
          .into_iter()
          .find(|t| t.path == entry.path)
          .expect("exposed row generates a tool");
        assert!(
          tool.input_schema["properties"].get(&name).is_none(),
          "`{}` publishes non-keep flag `--{name}`",
          entry.path
        );
      }
    }
    assert!(
      non_keep_on_exposed > 0,
      "no exposed row carries a non-keep flag, so the filter above was asserted against nothing -- the control is dead"
    );
    // The doctor row is the measured instance: `--fix` is `retire` and must
    // not be a parameter.
    let doctor = all()
      .into_iter()
      .find(|t| t.path == "doctor")
      .expect("doctor is exposed");
    assert!(
      doctor.input_schema["properties"].get("fix").is_none(),
      "doctor publishes `fix`, the exact flag the zero-match filter shipped"
    );
  }

  /// The narrowed flag key trims parameters off published schemas -- the
  /// measured instances from the 2026-08-30 narrow: terminal-channel flags on
  /// read tools.
  #[test]
  fn a_narrowed_flag_is_not_published() {
    for (path, param) in [
      ("st list", "width"),
      ("st list", "format"),
      ("at lint", "fix"),
      ("events", "format"),
      ("organize", "default"),
      ("organize", "force"),
    ] {
      let tool = all()
        .into_iter()
        .find(|t| t.path == path)
        .unwrap_or_else(|| panic!("{path} is exposed"));
      assert!(
        tool.input_schema["properties"].get(param).is_none(),
        "`{path}` publishes narrowed flag `{param}`"
      );
    }
  }

  /// A `subcommand` slot is not a parameter: each filling is its own tool.
  #[test]
  fn a_subcommand_slot_is_not_published() {
    let todo = all()
      .into_iter()
      .find(|t| t.path == "todo")
      .expect("todo is exposed");
    assert!(
      todo.input_schema["properties"].get("command").is_none(),
      "todo publishes its verb slot as a parameter"
    );
  }
}
