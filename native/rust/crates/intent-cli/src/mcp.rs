//! The MCP tool tier's DATA: every tool, generated from the dispatch table.
//!
//! **THIS MODULE EMITS DESCRIPTIONS, NEVER CALLS** -- the serving match lands
//! beside it under vc's (a)-now ruling (2026-08-30): hand-written, gated
//! two-sided against this population, with (b) -- routing through the daemon's
//! one `dispatch(op)` door -- recorded as the 3.x destination rather than
//! refused. The split matters for the tag window: this half needs no new
//! dependency, so nobody's `Cargo.lock` moves for a tool list.
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

use crate::dispatch::{Entry, Flag, Table};
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

/// One JSON-Schema object from the row's declared args and flags.
fn schema(entry: &Entry) -> Result<Value, Undeclarable> {
  let refuse = |why: String| Undeclarable {
    path: entry.path.clone(),
    why,
  };
  let mut properties = Map::new();
  let mut required: Vec<String> = Vec::new();

  for arg in &entry.args {
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
      "string" | "positional" | "subcommand" => {
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

  for flag in &entry.flags {
    // A flag the surface is retiring must not be published to a new surface.
    if flag.disposition == "drop" {
      continue;
    }
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
      for a in &entry.args {
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
      for f in &entry.flags {
        if f.disposition == "drop" {
          continue;
        }
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
}
