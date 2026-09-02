//! Operator settings: the DECLARED allow-list, and the one reader/writer of
//! `~/.intent/config.json`'s `explorer` section. `AC-17.14`.
//!
//! # The exposed set is declared, never derived
//!
//! **`AC-17.14`: a settings surface exposes only state whose modification is a
//! legitimate operator act, and the set is declared rather than derived from
//! what the file happens to contain.** The config document carries
//! `intent_version` -- a MIGRATION MARKER recording what the tree has been
//! migrated to -- and `intent_dir`, which is structural. Both are writable in
//! the sense that a text editor can change them; **writability is not
//! permission**. An operator editing `intent_version` tells the migrator a lie
//! it has no way to detect, which is `IN-AG-NO-SILENT-001` at a surface where
//! the silent failure is a SUCCESSFUL WRITE.
//!
//! So [`DECLARED`] is a table, and a key that is not in it is not a setting.
//! **The direction matters more than the instance.** A deny-list -- everything
//! except these two -- inverts the failure: a key added to the config next year
//! becomes editable the moment somebody writes it, by nobody's decision, which
//! is exactly how the two fields above would have arrived.
//!
//! And the refusal is **structural rather than advisory**: [`read_all`] does
//! not return the excluded keys marked read-only for a face to grey out, it
//! does not return them at all. A row an operator can see is a row an operator
//! will eventually try.
//!
//! # One section, and paths resolve inside it
//!
//! hv ruled the scope (2026-09-02): `/settings` is bound to [`SECTION`] and
//! never to the whole document. A path is therefore written the way the
//! operator says it -- `editing.mode`, not `explorer.editing.mode` -- because
//! **one resolution rule beats two**, and a surface that accepts both spellings
//! has to answer what the second one means when the section changes.
//!
//! # One renderer for one file
//!
//! [`render_doc`] is THE way this file is written, and [`crate::bootstrap`]
//! calls it rather than rendering its own. That is not tidiness: `bootstrap`'s
//! own note explains that `serde_json` writes a map in whatever order it is
//! given and offers no trailing newline, and this is a file an operator opens
//! in an editor. **Two writers of one JSON document are two spellings of its
//! shape, and they agree until they do not** -- at which point the file an
//! operator diffs has changed for a reason nobody can name.
//!
//! **Every key survives a write, including ones this module has never heard
//! of.** The known keys come first in a fixed order and the rest follow in the
//! order `serde_json`'s map yields them, so the file is stable across writes
//! and a hand-added key is not silently eaten by a settings change.

use std::path::Path;

use serde_json::{Map, Value};

/// The section `/settings` is bound to. Nothing outside it is a setting.
pub const SECTION: &str = "explorer";

/// One setting an operator may change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setting {
  /// How the operator names it, RELATIVE to [`SECTION`]: `editing.mode`.
  pub path: &'static str,
  /// What it is called on screen.
  pub label: &'static str,
  /// What it does, in the operator's words.
  pub blurb: &'static str,
  /// Every value it may take, the first being the default.
  ///
  /// **NON-EMPTY BY CONTRACT, and a test holds it.** A setting with no declared
  /// values would be free text, and there is no collector for free text on this
  /// surface yet -- so declaring one would offer an edit that cannot happen,
  /// which is `AC-17.13` one row over. The vocabulary grows when the act lands.
  pub values: &'static [&'static str],
}

impl Setting {
  /// The value in force when the file says nothing.
  pub fn default(&self) -> &'static str {
    self.values[0]
  }

  /// The value after this one, wrapping. **A DECLARED SET IS PICKED, NOT
  /// TYPED**: `emcas` is a spelling error a surface with the list in hand can
  /// make impossible, and refusing it after the fact is the worse of the two.
  pub fn next_after(&self, current: &str) -> &'static str {
    let at = self.values.iter().position(|v| *v == current);
    match at {
      Some(i) => self.values[(i + 1) % self.values.len()],
      // A value the declaration does not carry -- hand-edited into the file.
      // Cycling lands on the default, which is the one value always legal.
      None => self.default(),
    }
  }
}

/// Every setting, in the order a face shows them.
///
/// **ONE ENTRY, AND THAT IS THE HONEST SET.** hv ruled the editing mode a
/// setting because `set -o vi` **is not detectable from a child process** --
/// measured, not assumed: `SHELLOPTS` is bash-only and absent under zsh,
/// nothing else in the environment carries it, and `~/.inputrc` is readline's
/// file which zsh never reads. Declared-not-detected is `ST0037`'s ruling one
/// surface over.
pub const DECLARED: &[Setting] = &[Setting {
  path: "editing.mode",
  label: "editing mode",
  blurb: "which keymap the composer uses",
  values: &["emacs", "vi"],
}];

/// The setting `path` names, or `None`.
///
/// **THE ALLOW-LIST IS THIS FUNCTION**, so there is exactly one place that
/// decides whether a spelling is a setting.
pub fn find(path: &str) -> Option<&'static Setting> {
  DECLARED.iter().find(|s| s.path == path)
}

/// Why a settings operation could not happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsError {
  /// **REFUSED AS A SPELLING** (`tui-design.md` section 8): what was tried, and
  /// what governs. Never resolved to something near it.
  NoSuchSetting(String),
  /// A value outside the declared set.
  NoSuchValue { path: String, value: String },
  /// The file is there and is not a JSON object.
  Unreadable { path: String, why: String },
  /// The write did not land.
  Unwritable { path: String, why: String },
}

impl std::fmt::Display for SettingsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SettingsError::NoSuchSetting(s) => write!(
        f,
        "`{s}` is not a setting -- `/settings` governs the `{SECTION}` section, which carries {}",
        names()
      ),
      SettingsError::NoSuchValue { path, value } => {
        let allowed = find(path).map(|s| s.values.join(", ")).unwrap_or_default();
        write!(
          f,
          "`{value}` is not a value for `{path}` -- it takes {allowed}"
        )
      }
      SettingsError::Unreadable { path, why } => write!(f, "cannot read {path}: {why}"),
      SettingsError::Unwritable { path, why } => write!(f, "cannot write {path}: {why}"),
    }
  }
}

impl std::error::Error for SettingsError {}

/// The declared paths, for a refusal that teaches the scope instead of reading
/// as broken.
fn names() -> String {
  DECLARED
    .iter()
    .map(|s| s.path)
    .collect::<Vec<_>>()
    .join(", ")
}

/// Every declared setting with the value in force, in declaration order.
///
/// **AN ABSENT FILE IS NOT AN ERROR AND AN ABSENT KEY IS NOT EITHER**: the
/// question asked is *what is in force*, and the answer on a machine that has
/// never written a setting is the defaults. There is nothing half-done to hide
/// -- no write has happened -- so distinguishing the cases would produce a
/// branch every caller takes identically.
pub fn read_all(config: &Path) -> Vec<(&'static Setting, String)> {
  let doc = document(config).unwrap_or_default();
  DECLARED
    .iter()
    .map(|s| {
      let value = section_value(&doc, s.path)
        .filter(|v| s.values.contains(&v.as_str()))
        .unwrap_or_else(|| s.default().to_string());
      (s, value)
    })
    .collect()
}

/// One setting's value in force, or a refusal naming the spelling.
pub fn read_one(config: &Path, path: &str) -> Result<String, SettingsError> {
  let setting = find(path).ok_or_else(|| SettingsError::NoSuchSetting(path.to_string()))?;
  Ok(
    read_all(config)
      .into_iter()
      .find(|(s, _)| s.path == setting.path)
      .map(|(_, v)| v)
      .unwrap_or_else(|| setting.default().to_string()),
  )
}

/// Set one setting, creating [`SECTION`] if the file has never carried it.
///
/// **BOTH HALVES ARE CHECKED BEFORE ANYTHING IS WRITTEN**: an undeclared path
/// and an undeclared value both refuse having touched nothing, so there is no
/// state in which the file has moved and the caller was told it did not.
pub fn write_one(config: &Path, path: &str, value: &str) -> Result<(), SettingsError> {
  let setting = find(path).ok_or_else(|| SettingsError::NoSuchSetting(path.to_string()))?;
  if !setting.values.contains(&value) {
    return Err(SettingsError::NoSuchValue {
      path: path.to_string(),
      value: value.to_string(),
    });
  }
  let mut doc = document(config)?;
  put(&mut doc, setting.path, value);
  if let Some(dir) = config.parent() {
    std::fs::create_dir_all(dir).map_err(|e| SettingsError::Unwritable {
      path: dir.display().to_string(),
      why: e.to_string(),
    })?;
  }
  std::fs::write(config, render_doc(&doc)).map_err(|e| SettingsError::Unwritable {
    path: config.display().to_string(),
    why: e.to_string(),
  })
}

/// The config document, or an empty one when the file is not there.
///
/// **A FILE THAT IS THERE AND UNPARSEABLE IS AN ERROR, AND AN ABSENT ONE IS
/// NOT.** They are different questions: nothing has been configured yet, versus
/// something is configured and this cannot tell what. Treating the second as
/// the first would let a write flatten a config the operator broke by hand into
/// one this module invented.
fn document(config: &Path) -> Result<Map<String, Value>, SettingsError> {
  let Ok(text) = std::fs::read_to_string(config) else {
    return Ok(Map::new());
  };
  if text.trim().is_empty() {
    return Ok(Map::new());
  }
  let parsed: Value = serde_json::from_str(&text).map_err(|e| SettingsError::Unreadable {
    path: config.display().to_string(),
    why: e.to_string(),
  })?;
  match parsed {
    Value::Object(map) => Ok(map),
    other => Err(SettingsError::Unreadable {
      path: config.display().to_string(),
      why: format!("expected an object, found {}", kind_of(&other)),
    }),
  }
}

fn kind_of(v: &Value) -> &'static str {
  match v {
    Value::Null => "null",
    Value::Bool(_) => "a boolean",
    Value::Number(_) => "a number",
    Value::String(_) => "a string",
    Value::Array(_) => "an array",
    Value::Object(_) => "an object",
  }
}

/// Read `SECTION.<dotted>` out of a document.
fn section_value(doc: &Map<String, Value>, dotted: &str) -> Option<String> {
  let mut at = doc.get(SECTION)?;
  for segment in dotted.split('.') {
    at = at.get(segment)?;
  }
  at.as_str().map(str::to_string)
}

/// Write `SECTION.<dotted>`, creating every level that is missing.
///
/// **A NON-OBJECT IN THE WAY IS REPLACED, and that is the only destructive
/// thing here.** `explorer` holding a string means the path the operator asked
/// for cannot exist; keeping it would mean refusing every write to the section
/// forever with no way to clear it from this surface.
fn put(doc: &mut Map<String, Value>, dotted: &str, value: &str) {
  let mut at = doc
    .entry(SECTION.to_string())
    .or_insert_with(|| Value::Object(Map::new()));
  let segments: Vec<&str> = dotted.split('.').collect();
  for (i, segment) in segments.iter().enumerate() {
    if !at.is_object() {
      *at = Value::Object(Map::new());
    }
    let map = at.as_object_mut().expect("just made it an object");
    if i + 1 == segments.len() {
      map.insert((*segment).to_string(), Value::String(value.to_string()));
      return;
    }
    at = map
      .entry((*segment).to_string())
      .or_insert_with(|| Value::Object(Map::new()));
  }
}

/// The order the known keys are written in. **A FIXED ORDER SO THE FILE LOOKS
/// THE SAME EVERY TIME IT IS WRITTEN** -- `bootstrap`'s own requirement, kept
/// here because this is now the one renderer.
const ORDER: &[&str] = &["intent_version", "author", "intent_dir", SECTION];

/// The config document's bytes: THE one rendering of this file.
///
/// **KEYS THIS MODULE HAS NEVER HEARD OF SURVIVE**, after the known ones. A
/// renderer that emitted only what it knew would make every settings write a
/// silent deletion of anything an operator had added by hand -- the same class
/// as the deny-list above, arriving through the writer instead of the reader.
pub fn render_doc(doc: &Map<String, Value>) -> String {
  let mut keys: Vec<&String> = Vec::new();
  for known in ORDER {
    if let Some((k, _)) = doc.get_key_value(*known) {
      keys.push(k);
    }
  }
  for k in doc.keys() {
    if !ORDER.contains(&k.as_str()) {
      keys.push(k);
    }
  }
  let mut out = String::from("{\n");
  for (i, k) in keys.iter().enumerate() {
    let rendered = serde_json::to_string_pretty(&doc[*k]).unwrap_or_else(|_| "null".to_string());
    let indented = rendered.replace('\n', "\n  ");
    out.push_str(&format!("  {}: {}", Value::String((*k).clone()), indented));
    if i + 1 < keys.len() {
      out.push(',');
    }
    out.push('\n');
  }
  out.push_str("}\n");
  out
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  fn tmp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("intent-settings-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture dir");
    dir.join("config.json")
  }

  /// **THE CRITERION'S OWN SENTENCE, AS A TEST**: the surface exposes only
  /// state whose modification is a legitimate operator act, and the two fields
  /// that are not are ABSENT rather than read-only.
  #[test]
  fn the_migration_marker_and_the_structural_key_are_not_settings_at_all() {
    for forbidden in ["intent_version", "intent_dir", "author"] {
      assert!(
        find(forbidden).is_none(),
        "`{forbidden}` resolved as a setting, so a surface could offer it"
      );
    }
    assert!(
      !DECLARED.is_empty(),
      "an empty allow-list would pass the check above by having nothing in it"
    );
  }

  /// The allow-list holds in the direction that matters: a key that appears in
  /// the FILE is still not a setting. **This is the deny-list failure, driven**
  /// -- under a deny-list this write would land.
  #[test]
  fn a_key_the_file_carries_is_still_not_a_setting() {
    let path = tmp("declared-not-derived");
    std::fs::write(
      &path,
      "{\n  \"intent_version\": \"3.0.0\",\n  \"intent_dir\": \"intent\"\n}\n",
    )
    .expect("fixture");
    let refused = write_one(&path, "intent_version", "9.9.9");
    assert!(
      matches!(refused, Err(SettingsError::NoSuchSetting(ref s)) if s == "intent_version"),
      "writing the migration marker was not refused: {refused:?}"
    );
    let after = std::fs::read_to_string(&path).expect("still readable");
    assert!(
      after.contains("3.0.0"),
      "the refusal still moved the file: {after}"
    );
  }

  /// A refusal says what was tried and what governs -- section 8's rule, so the
  /// message teaches the scope rather than reading as broken.
  #[test]
  fn an_unknown_spelling_is_refused_as_a_spelling_and_names_the_section() {
    let said = SettingsError::NoSuchSetting("author".into()).to_string();
    assert!(
      said.contains("author"),
      "the refusal does not say what was tried: {said}"
    );
    assert!(
      said.contains(SECTION),
      "the refusal does not name the section that governs: {said}"
    );
    assert!(
      said.contains("editing.mode"),
      "the refusal does not say what IS a setting: {said}"
    );
  }

  /// Defaults are in force before anything has been written, and the file need
  /// not exist for the question to have an answer.
  #[test]
  fn a_machine_that_has_never_written_a_setting_still_has_them_all_in_force() {
    let path = tmp("never-written");
    let all = read_all(&path);
    assert_eq!(all.len(), DECLARED.len(), "a declared setting went missing");
    for (setting, value) in &all {
      assert_eq!(
        value,
        setting.default(),
        "`{}` is not at its default on a fresh machine",
        setting.path
      );
    }
    assert_eq!(read_one(&path, "editing.mode").expect("declared"), "emacs");
  }

  /// **THE SECTION IS CREATED ON FIRST WRITE AND EVERY OTHER KEY SURVIVES IT**
  /// -- including one this module has never heard of, which is the half a
  /// renderer that emitted only what it knew would eat.
  #[test]
  fn the_first_write_creates_the_section_and_disturbs_nothing_else() {
    let path = tmp("first-write");
    std::fs::write(
      &path,
      "{\n  \"intent_version\": \"3.0.0\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"something_nobody_declared\": 41\n}\n",
    )
    .expect("fixture");
    write_one(&path, "editing.mode", "vi").expect("a declared setting with a declared value");
    let after = std::fs::read_to_string(&path).expect("readable");
    let parsed: Value = serde_json::from_str(&after).expect("still valid JSON:\n{after}");
    assert_eq!(parsed["intent_version"], "3.0.0");
    assert_eq!(parsed["author"], "matts");
    assert_eq!(parsed["intent_dir"], "intent");
    assert_eq!(
      parsed["something_nobody_declared"], 41,
      "a settings write ate a key it did not declare:\n{after}"
    );
    assert_eq!(parsed[SECTION]["editing"]["mode"], "vi");
    assert_eq!(read_one(&path, "editing.mode").expect("declared"), "vi");
  }

  /// A value outside the declared set refuses, and refuses before writing.
  #[test]
  fn a_value_the_setting_does_not_declare_is_refused_before_anything_moves() {
    let path = tmp("bad-value");
    write_one(&path, "editing.mode", "vi").expect("a legal write first");
    let before = std::fs::read_to_string(&path).expect("readable");
    let refused = write_one(&path, "editing.mode", "acme");
    assert!(
      matches!(refused, Err(SettingsError::NoSuchValue { .. })),
      "an undeclared value was accepted: {refused:?}"
    );
    assert_eq!(
      std::fs::read_to_string(&path).expect("readable"),
      before,
      "a refused write still moved the file"
    );
  }

  /// **A HAND-EDITED VALUE OUTSIDE THE DECLARED SET READS AS THE DEFAULT**, so
  /// a broken file cannot put the composer into a keymap that does not exist.
  #[test]
  fn a_value_the_declaration_does_not_carry_falls_back_rather_than_taking_effect() {
    let path = tmp("hand-broken");
    std::fs::write(
      &path,
      "{\n  \"explorer\": { \"editing\": { \"mode\": \"dvorak\" } }\n}\n",
    )
    .expect("fixture");
    assert_eq!(
      read_one(&path, "editing.mode").expect("declared"),
      "emacs",
      "an undeclared value in the file took effect"
    );
  }

  /// The renderer is stable: writing the same value twice produces the same
  /// bytes, so a settings change is a one-line diff rather than a reshuffle.
  #[test]
  fn writing_the_same_value_twice_leaves_the_same_bytes() {
    let path = tmp("stable");
    write_one(&path, "editing.mode", "vi").expect("write");
    let once = std::fs::read_to_string(&path).expect("readable");
    write_one(&path, "editing.mode", "vi").expect("write");
    assert_eq!(once, std::fs::read_to_string(&path).expect("readable"));
    assert!(once.ends_with("}\n"), "the file has no trailing newline");
  }

  /// A file that is there and unparseable is an ERROR, not an empty config --
  /// otherwise a write flattens a config the operator broke by hand.
  #[test]
  fn a_broken_file_refuses_rather_than_being_treated_as_a_fresh_machine() {
    let path = tmp("broken");
    std::fs::write(&path, "{ this is not json").expect("fixture");
    let refused = write_one(&path, "editing.mode", "vi");
    assert!(
      matches!(refused, Err(SettingsError::Unreadable { .. })),
      "a broken config was overwritten rather than refused: {refused:?}"
    );
    assert_eq!(
      std::fs::read_to_string(&path).expect("readable"),
      "{ this is not json",
      "the operator's broken file was replaced"
    );
  }

  /// **EVERY DECLARED SETTING IS PICKABLE**, which is what lets the surface
  /// offer a cycle rather than a text collector. A free-text setting would need
  /// one built first.
  #[test]
  fn every_declared_setting_offers_at_least_two_values_to_pick_between() {
    for s in DECLARED {
      assert!(
        s.values.len() >= 2,
        "`{}` declares {} value(s), so there is nothing to cycle to -- a free-text \
         setting needs a collector this surface does not have",
        s.path,
        s.values.len()
      );
      assert!(!s.label.trim().is_empty(), "`{}` has no label", s.path);
      assert!(!s.blurb.trim().is_empty(), "`{}` explains nothing", s.path);
    }
  }

  /// Cycling visits every value and comes back, so no value is unreachable from
  /// the keyboard.
  #[test]
  fn cycling_reaches_every_value_and_returns_to_where_it_started() {
    for s in DECLARED {
      let mut at = s.default();
      let mut seen = vec![at];
      for _ in 0..s.values.len() {
        at = s.next_after(at);
        if !seen.contains(&at) {
          seen.push(at);
        }
      }
      assert_eq!(
        seen.len(),
        s.values.len(),
        "`{}` cannot reach all of {:?} by cycling -- got {seen:?}",
        s.path,
        s.values
      );
      assert_eq!(at, s.default(), "`{}` does not wrap", s.path);
    }
  }
}
