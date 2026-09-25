//! Issue 0574: **the register's user-facing text carries no maintainers' notes.**
//!
//! `surface/dispatch-table.json` renders into three surfaces a user reads: the
//! reference pages under `docs/reference` (each verb's and family's `help`, each
//! argument's `note` and `default`, each flag's `help`), `--help` (the help texts
//! and `value_help`), and the MCP tool descriptions (`help` and `when_to_use`).
//! The 3.2.1 doc audit rewrote 25 rows that carried Rust source paths, rulings,
//! node names, dates and criterion ids into those surfaces. They survived because
//! nothing scanned what renders into `docs/reference`: `no_pm_state_in_output.rs`
//! drives `--help` only, and only for thread, work-package, criterion and test ids
//! and issue numbers.
//!
//! So this reads the register itself, every row in both homes -- a retired row's
//! `help` still renders, in a page's Retired table -- and refuses, in any field
//! that renders:
//!
//! - a Rust source path (`render.rs`, `spine.rs:193`);
//! - a date (`2026-09-24`): a text a user reads describes the tool, not when a
//!   maintainer changed it;
//! - a development id (`AC-09.6`, `AT-00.8`, `INV-04`, `WP-06`, `TN001`);
//! - an issue citation (`issue 0142`);
//! - a node moniker (`cc`, `dc`, `ic`, `vc`) outside a code span. `hv` is not in
//!   the list: it is the whiteboard's own name for the human's board, and the
//!   `wb` rows teach it.
//!
//! **WHAT IT DOES NOT REFUSE, MEASURED RATHER THAN ASSUMED.** A run of capitals
//! and the words "ruling" or "verbatim" were measured over the 508 rendered texts
//! on 2026-09-25 and every hit was legitimate: `when_to_use` writes its guidance
//! as `USE IT` / `DO NOT USE IT` for the agent reading it, and "a ruling" is what
//! a whiteboard broadcast carries. A detector that fires on correct text gets
//! switched off, so those shapes are left to review.

use serde_json::Value;
use testkit::repo_root;

/// Every text the register renders, with where it sits.
fn rendered_texts(table: &Value) -> Vec<(String, String)> {
  let mut out = Vec::new();
  let mut push = |at: String, v: &Value| {
    if let Some(s) = v.as_str() {
      out.push((at, s.to_string()));
    }
  };
  let entry = |e: &Value, out_push: &mut dyn FnMut(String, &Value)| {
    let path = e["path"].as_str().unwrap_or("?").to_string();
    out_push(format!("{path}.help"), &e["help"]);
    out_push(format!("{path}.when_to_use"), &e["when_to_use"]);
    for a in e["args"].as_array().into_iter().flatten() {
      let name = a["name"].as_str().unwrap_or("?");
      for k in ["note", "default", "help"] {
        out_push(format!("{path}.args[{name}].{k}"), &a[k]);
      }
      for (vk, vv) in a["value_help"].as_object().into_iter().flatten() {
        out_push(format!("{path}.args[{name}].value_help.{vk}"), vv);
      }
    }
    for f in e["flags"].as_array().into_iter().flatten() {
      let name = f["spellings"][0].as_str().unwrap_or("?");
      out_push(format!("{path}.flags[{name}].help"), &f["help"]);
    }
  };
  let families = table["families"]
    .as_array()
    .expect("the register's `families`");
  let new_surface = table["new_surface"]
    .as_array()
    .expect("the register's `new_surface`");
  assert!(
    !families.is_empty() && !new_surface.is_empty(),
    "precondition: both homes of the command surface are read"
  );
  for fam in families {
    let name = fam["name"].as_str().unwrap_or("?");
    push(format!("family {name}.help"), &fam["help"]);
    for e in fam["entries"].as_array().into_iter().flatten() {
      entry(e, &mut push);
    }
  }
  for e in new_surface {
    entry(e, &mut push);
  }
  push("root_help".to_string(), &table["root_help"]);
  push("mcp_instructions".to_string(), &table["mcp_instructions"]);
  out
}

/// The text outside backtick code spans: an even-indexed segment of a split on
/// the backtick is prose, which is how `gen_reference.sh` reads it too.
fn prose(text: &str) -> String {
  text
    .split('`')
    .enumerate()
    .filter(|(i, _)| i % 2 == 0)
    .map(|(_, s)| s)
    .collect::<Vec<_>>()
    .join(" ")
}

fn words(text: &str) -> impl Iterator<Item = &str> {
  text
    .split(|c: char| {
      !(c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '/' || c == ':')
    })
    .filter(|w| !w.is_empty())
}

fn is_date(w: &str) -> bool {
  let b = w.as_bytes();
  (0..b.len().saturating_sub(9)).any(|i| {
    let d = |j: usize| b[i + j].is_ascii_digit();
    b[i] == b'2'
      && b[i + 1] == b'0'
      && d(2)
      && d(3)
      && b[i + 4] == b'-'
      && d(5)
      && d(6)
      && b[i + 7] == b'-'
      && d(8)
      && d(9)
  })
}

fn is_dev_id(w: &str) -> bool {
  let w = w.trim_matches(|c: char| !c.is_ascii_alphanumeric());
  ["AC-", "AT-", "INV-", "WP-"].iter().any(|p| {
    w.strip_prefix(p)
      .is_some_and(|r| r.starts_with(|c: char| c.is_ascii_digit()))
  }) || (w.len() == 5 && w.starts_with("TN") && w[2..].chars().all(|c| c.is_ascii_digit()))
}

fn is_rust_path(w: &str) -> bool {
  let w = w.trim_end_matches(['.', ':']);
  w.ends_with(".rs") && w.len() > 3 || w.contains(".rs:")
}

/// What a user-facing text must not carry, one name per finding.
fn maintainer_notes(text: &str) -> Vec<String> {
  let mut found = Vec::new();
  for w in words(text) {
    if is_rust_path(w) {
      found.push(format!("a Rust source path `{w}`"));
    }
    if is_date(w) {
      found.push(format!("a date `{w}`"));
    }
    if is_dev_id(w) {
      found.push(format!("a development id `{w}`"));
    }
  }
  let lower = text.to_ascii_lowercase();
  for (i, _) in lower.match_indices("issue ") {
    let rest = &lower[i + 6..];
    let digits: String = rest
      .trim_start_matches('#')
      .chars()
      .take_while(|c| c.is_ascii_digit())
      .collect();
    if digits.len() >= 3 {
      found.push(format!("an issue citation `issue {digits}`"));
    }
  }
  for w in prose(text).split(|c: char| !c.is_ascii_alphanumeric()) {
    if ["cc", "dc", "ic", "vc"].contains(&w) {
      found.push(format!("a node moniker `{w}`"));
    }
  }
  found
}

#[test]
fn no_rendered_register_text_carries_a_maintainers_note() {
  let raw = std::fs::read_to_string(repo_root().join("surface/dispatch-table.json"))
    .expect("read the register");
  let table: Value = serde_json::from_str(&raw).expect("the register parses");
  let texts = rendered_texts(&table);
  let bad: Vec<String> = texts
    .iter()
    .flat_map(|(at, s)| {
      maintainer_notes(s)
        .into_iter()
        .map(move |f| format!("{at}: {f}"))
    })
    .collect();
  assert!(
    bad.is_empty(),
    "{} rendered register text(s) carry a maintainers' note -- a user reads these in \
     docs/reference, --help or an MCP tool description; move the note to a field that does \
     not render:\n{}",
    bad.len(),
    bad.join("\n")
  );
}

/// **EACH DETECTOR IS SEEN TO FIRE, AND A CLEAN TEXT IS SEEN NOT TO.** A guard
/// that has only ever passed is indistinguishable from one that cannot fail.
#[test]
fn each_detector_fires_on_its_shape_and_a_clean_text_passes() {
  for (text, kind) in [
    ("Refuses here (render.rs:1215 says why)", "Rust source path"),
    ("Ruled on 2026-09-24 by the release", "date"),
    ("Exposed implies servable (AC-09.6)", "development id"),
    ("See INV-04 for the exit contract", "development id"),
    ("The detector from issue 0142", "issue citation"),
    ("vc landed this after the build", "node moniker"),
  ] {
    let f = maintainer_notes(text);
    assert!(
      f.iter().any(|x| x.contains(kind)),
      "`{text}` must be named as a {kind}: {f:?}"
    );
  }
  for clean in [
    "Show the DOING / TODO / DONE view of threads and work packages",
    "on `hv`'s board alone, a standing `directive`",
    "`intent sync --to-store` carries each board.json into the store",
    "USE IT when every node needs the same line, or to broadcast a ruling",
    "a `vc` in a code span is the reader's own name, not ours",
  ] {
    assert!(
      maintainer_notes(clean).is_empty(),
      "`{clean}` is a user's text: {:?}",
      maintainer_notes(clean)
    );
  }
}
