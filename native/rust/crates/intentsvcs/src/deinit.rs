//! `intent deinit` -- strip what Intent wrote from a project being closed
//! (ST0083 WP-06, AC-06.1).
//!
//! **A DRY RUN BY DEFAULT, AND EVERY ACT IS ONE CANON CAN PROVE IS ITS OWN.**
//! [`plan`] reads the tree and decides; nothing writes until [`apply`] runs the
//! plan, and it re-reads each file first and refuses one whose bytes moved
//! since the plan read them. A file the plan cannot prove Intent wrote, or
//! cannot parse, is kept and named rather than guessed at.
//!
//! **WHETHER A FILE IS INTENT'S IS ALWAYS CANON'S ANSWER, NEVER THIS MODULE'S.**
//! The contracts ask [`crate::canon::is_generated_contract`], the seeds and
//! carriers compare against [`crate::canon::canon_bytes`], the hooks cut with
//! [`crate::canon::cut_chain_block`], and the settings surgery recognises a hook
//! by [`crate::canon::carries_hook_door`]. A second spelling of any of those
//! here would be a second opinion about the same bytes.
//!
//! **THE SETTINGS FILE IS EDITED, NOT DELETED** (vc's decision 66 (a), under
//! hv's pen). The fleet's `.claude/settings.json` files carry the owner's own
//! permissions and hooks beside Intent's, so the hook entries whose command
//! carries `intent claude hook` and the template's deny rules are removed and
//! the rest is kept. Keeping a marker-carrying file whole was the alternative,
//! and it loses: a project with no Intent whose settings still wire
//! `intent claude hook require-in-session` refuses every prompt.

use crate::canon::{self, Artefact};
use crate::project::{Contracts, Project};
use crate::views::RenderContext;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Why `deinit` could not plan or finish.
#[derive(Debug, thiserror::Error)]
pub enum DeinitError {
  #[error("{} could not be read: {cause}", path.display())]
  Unreadable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("{} could not be written or removed: {cause}", path.display())]
  Unwritable {
    path: PathBuf,
    #[source]
    cause: std::io::Error,
  },
  #[error("{} changed after it was planned, so it was left as it is", path.display())]
  Changed { path: PathBuf },
  /// By message, as `FacadeError::Canon` carries it: `CanonError` has no
  /// `source` chain.
  #[error("the installed templates could not be read: {0}")]
  Canon(canon::CanonError),
  #[error("the project registry could not be updated: {0}")]
  Registry(#[from] crate::projects::ProjectsError),
}

impl crate::remedy::Remedy for DeinitError {
  fn remedy(&self) -> String {
    match self {
      Self::Unreadable { .. } | Self::Unwritable { .. } => {
        "check that you own the file and can write its directory, then run `intent deinit` again: it plans from what is there now, so what already went is not planned twice".to_string()
      }
      Self::Changed { .. } => {
        "something wrote the file while deinit ran; run `intent deinit` again to plan from what is there now".to_string()
      }
      Self::Canon(_) => {
        "deinit compares each seeded file with the installed Intent's template, so it needs an install it can read; run `intent doctor` to see where the install resolves".to_string()
      }
      Self::Registry(e) => crate::remedy::Remedy::remedy(e),
    }
  }
}

/// One thing [`apply`] will do. Every path is absolute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Act {
  /// Remove a file whose bytes are `expect`.
  Remove {
    path: PathBuf,
    why: String,
    expect: Vec<u8>,
  },
  /// Replace a file whose bytes are `expect` with `after`.
  Rewrite {
    path: PathBuf,
    why: String,
    expect: Vec<u8>,
    after: String,
  },
  /// Remove a directory and everything in it (`--all`'s `intent/`).
  RemoveTree { path: PathBuf, why: String },
  /// Remove the project's row from the registry at `registry`.
  Unregister { registry: PathBuf, root: PathBuf },
}

/// A file the plan leaves, and why it cannot prove Intent wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kept {
  pub path: PathBuf,
  pub why: String,
}

/// What `deinit` would do, in the order it does it.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Plan {
  pub acts: Vec<Act>,
  pub kept: Vec<Kept>,
}

/// Where the plan reads from: the project, the install its templates come
/// from, the git hooks directory when there is one, and with `--all` the
/// registry that lists the project.
pub struct Scope<'a> {
  pub project: &'a Project,
  pub install: &'a Path,
  pub hooks: Option<&'a Path>,
  pub ctx: &'a RenderContext<'a>,
  /// `Some` with `--all`: the registry, whose row for this project goes too.
  pub all: Option<&'a Path>,
}

/// Read the tree and decide what `deinit` removes, edits and keeps.
pub fn plan(scope: &Scope<'_>) -> Result<Plan, DeinitError> {
  let root = scope.project.root();
  let cfg = scope.project.config();
  let mut plan = Plan::default();

  for name in ["AGENTS.md", "CLAUDE.md"] {
    for layout in [Contracts::Root, Contracts::Dotclaude] {
      let path = root.join(crate::sync::root_file_path(name, layout));
      let Some(bytes) = read(&path)? else {
        continue;
      };
      let text = String::from_utf8_lossy(&bytes);
      if !canon::is_generated_contract(name, &text) {
        plan.keep(path, "it carries no Intent footer, so a person wrote it");
      } else if name == "CLAUDE.md" && !canon::user_block_is_empty(&text) {
        plan.keep(path, "its user block holds directives a person wrote");
      } else {
        plan.remove(path, "generated by Intent", bytes);
      }
    }
  }

  for (rel, artefact) in [
    ("usage-rules.md", Artefact::UsageRules),
    (".intent_critic.yml", Artefact::CriticConfig),
  ] {
    let path = root.join(rel);
    let Some(bytes) = read(&path)? else {
      continue;
    };
    if bytes
      == canon::canon_bytes(artefact, scope.install, cfg, scope.ctx)
        .map_err(DeinitError::Canon)?
        .as_bytes()
    {
      plan.remove(path, "byte-identical to the seed Intent writes", bytes);
    } else {
      plan.keep(
        path,
        "it differs from the seed Intent writes, so it was edited",
      );
    }
  }

  let deny = template_deny(scope)?;
  plan.json_surgery(&root.join(".claude/settings.json"), |doc| {
    strip_settings(doc, &deny)
  })?;
  plan.json_surgery(&root.join(".mcp.json"), strip_mcp)?;

  if let Some(hooks) = scope.hooks {
    for hook in canon::chained_hooks() {
      plan.hook(&hooks.join(hook))?;
      let carrier = hooks.join(canon::carrier_name(hook));
      let Some(bytes) = read(&carrier)? else {
        continue;
      };
      let template =
        canon::canon_bytes(canon::carrier_artefact(hook), scope.install, cfg, scope.ctx)
          .map_err(DeinitError::Canon)?;
      if bytes == template.as_bytes() {
        plan.remove(carrier, "the carrier Intent's chain block calls", bytes);
      } else {
        plan.keep(carrier, "it differs from the carrier Intent writes");
      }
    }
  }

  if let Some(registry) = scope.all {
    plan.acts.push(Act::RemoveTree {
      path: scope.project.intent_dir(),
      why: "Intent's tree, the store with it".to_string(),
    });
    let listed = crate::projects::load(registry)?.roots();
    let canonical = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    if listed.contains(&canonical) {
      plan.acts.push(Act::Unregister {
        registry: registry.to_path_buf(),
        root: canonical,
      });
    }
  }
  Ok(plan)
}

impl Plan {
  fn keep(&mut self, path: PathBuf, why: &str) {
    self.kept.push(Kept {
      path,
      why: why.to_string(),
    });
  }

  fn remove(&mut self, path: PathBuf, why: &str, expect: Vec<u8>) {
    self.acts.push(Act::Remove {
      path,
      why: why.to_string(),
      expect,
    });
  }

  /// Plan the edit `strip` makes to the JSON file at `path`: nothing when it
  /// removes nothing, the file when nothing else is left, a rewrite otherwise.
  fn json_surgery(
    &mut self,
    path: &Path,
    strip: impl Fn(&Value) -> Value,
  ) -> Result<(), DeinitError> {
    let Some(bytes) = read(path)? else {
      return Ok(());
    };
    let doc: Value = match serde_json::from_slice(&bytes) {
      Ok(doc) => doc,
      Err(cause) => {
        self.keep(
          path.to_path_buf(),
          &format!("it does not parse as JSON ({cause}), so what in it is Intent's cannot be told"),
        );
        return Ok(());
      }
    };
    let stripped = strip(&doc);
    if stripped == doc {
      return Ok(());
    }
    if stripped.as_object().is_some_and(serde_json::Map::is_empty) {
      self.remove(
        path.to_path_buf(),
        "nothing in it but what Intent added",
        bytes,
      );
    } else {
      self.acts.push(Act::Rewrite {
        path: path.to_path_buf(),
        why: "Intent's entries removed, the rest kept and re-serialised as 2-space JSON"
          .to_string(),
        expect: bytes,
        after: render_json(&stripped),
      });
    }
    Ok(())
  }

  /// Plan the cut of the chain block in the hook at `path`.
  fn hook(&mut self, path: &Path) -> Result<(), DeinitError> {
    let Some(bytes) = read(path)? else {
      return Ok(());
    };
    let text = String::from_utf8_lossy(&bytes).into_owned();
    if let Some(why) = canon::chain_block_held(&text) {
      self.keep(path.to_path_buf(), &format!("{why}"));
      return Ok(());
    }
    let Some(cut) = canon::cut_chain_block(&text) else {
      return Ok(());
    };
    if canon::is_only_preamble(&cut) {
      self.remove(
        path.to_path_buf(),
        "nothing in it but Intent's chain block",
        bytes,
      );
    } else {
      self.acts.push(Act::Rewrite {
        path: path.to_path_buf(),
        why: "Intent's chain block cut out, every other line kept".to_string(),
        expect: bytes,
        after: cut,
      });
    }
    Ok(())
  }
}

/// The deny rules the settings template carries, read from the template, the
/// one home of what Intent adds there.
fn template_deny(scope: &Scope<'_>) -> Result<Vec<Value>, DeinitError> {
  let text = canon::canon_bytes(
    Artefact::Settings,
    scope.install,
    scope.project.config(),
    scope.ctx,
  )
  .map_err(DeinitError::Canon)?;
  let template: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
  Ok(
    template
      .pointer("/permissions/deny")
      .and_then(Value::as_array)
      .cloned()
      .unwrap_or_default(),
  )
}

/// `settings` without the hook entries whose command carries Intent's hook
/// door and without the deny rules in `deny`, emptied containers dropped.
pub fn strip_settings(settings: &Value, deny: &[Value]) -> Value {
  let mut out = settings.clone();
  if let Some(events) = out.get_mut("hooks").and_then(Value::as_object_mut) {
    for matchers in events.values_mut() {
      if let Some(matchers) = matchers.as_array_mut() {
        for matcher in matchers.iter_mut() {
          if let Some(hooks) = matcher.get_mut("hooks").and_then(Value::as_array_mut) {
            hooks.retain(|hook| {
              !hook
                .get("command")
                .and_then(Value::as_str)
                .is_some_and(canon::carries_hook_door)
            });
          }
        }
        matchers.retain(|matcher| {
          !matcher
            .get("hooks")
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty)
        });
      }
    }
    events.retain(|_, matchers| !matchers.as_array().is_some_and(Vec::is_empty));
  }
  if let Some(rules) = out
    .pointer_mut("/permissions/deny")
    .and_then(Value::as_array_mut)
  {
    rules.retain(|rule| !deny.contains(rule));
  }
  if let Some(permissions) = out.get_mut("permissions").and_then(Value::as_object_mut) {
    permissions.retain(|_, v| !v.as_array().is_some_and(Vec::is_empty));
  }
  drop_empty(&mut out, &["hooks", "permissions"]);
  out
}

/// `mcp` without the `intent` server, emptied containers dropped.
pub fn strip_mcp(mcp: &Value) -> Value {
  let mut out = mcp.clone();
  if let Some(servers) = out.get_mut("mcpServers").and_then(Value::as_object_mut) {
    servers.remove("intent");
  }
  drop_empty(&mut out, &["mcpServers"]);
  out
}

/// Remove each of `keys` from `doc` whose value is an empty object.
fn drop_empty(doc: &mut Value, keys: &[&str]) {
  if let Some(map) = doc.as_object_mut() {
    for key in keys {
      if map
        .get(*key)
        .and_then(Value::as_object)
        .is_some_and(serde_json::Map::is_empty)
      {
        map.remove(*key);
      }
    }
  }
}

/// Two-space JSON with a trailing newline, as Intent writes its own.
fn render_json(doc: &Value) -> String {
  // A `Value` parsed from text has no failing case in serde_json's serialiser.
  #[allow(
    clippy::expect_used,
    reason = "INVARIANT: a Value parsed from JSON text has no failing case in serde_json's serialiser"
  )]
  let text = serde_json::to_string_pretty(doc).expect("a parsed Value serialises");
  text + "\n"
}

/// The bytes at `path`, or `None` when nothing is there.
fn read(path: &Path) -> Result<Option<Vec<u8>>, DeinitError> {
  match std::fs::read(path) {
    Ok(bytes) => Ok(Some(bytes)),
    Err(cause) if cause.kind() == std::io::ErrorKind::NotFound => Ok(None),
    Err(cause) => Err(DeinitError::Unreadable {
      path: path.to_path_buf(),
      cause,
    }),
  }
}

/// Carry out `plan`, in order, returning the acts done. A file whose bytes
/// moved since the plan read them stops the run there, with what was done
/// before it already done, and the error names it.
pub fn apply(plan: &Plan) -> Result<Vec<Act>, DeinitError> {
  let mut done = Vec::new();
  for act in &plan.acts {
    match act {
      Act::Remove { path, expect, .. } => {
        unchanged(path, expect)?;
        std::fs::remove_file(path).map_err(|cause| DeinitError::Unwritable {
          path: path.clone(),
          cause,
        })?;
      }
      Act::Rewrite {
        path,
        expect,
        after,
        ..
      } => {
        unchanged(path, expect)?;
        std::fs::write(path, after).map_err(|cause| DeinitError::Unwritable {
          path: path.clone(),
          cause,
        })?;
      }
      Act::RemoveTree { path, .. } => {
        std::fs::remove_dir_all(path).map_err(|cause| DeinitError::Unwritable {
          path: path.clone(),
          cause,
        })?;
      }
      Act::Unregister { registry, root } => {
        crate::projects::remove(registry, std::slice::from_ref(root))?;
      }
    }
    done.push(act.clone());
  }
  Ok(done)
}

/// Refuse a file whose bytes are no longer the ones the plan read.
fn unchanged(path: &Path, expect: &[u8]) -> Result<(), DeinitError> {
  match read(path)? {
    Some(bytes) if bytes == expect => Ok(()),
    _ => Err(DeinitError::Changed {
      path: path.to_path_buf(),
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn settings_lose_intents_hooks_and_deny_rules_and_keep_the_owners() {
    let settings = serde_json::json!({
      "model": "opus",
      "permissions": { "allow": ["Bash(ls)"], "deny": ["Edit(/AGENTS.md)", "Read(./secrets)"] },
      "hooks": {
        "SessionStart": [{ "matcher": "startup", "hooks": [{ "type": "command", "command": "intent claude hook session-context" }] }],
        "Stop": [{ "matcher": "", "hooks": [
          { "type": "command", "command": "intent claude hook session-finish" },
          { "type": "command", "command": "./ours.sh" }
        ] }]
      }
    });
    let deny = vec![serde_json::json!("Edit(/AGENTS.md)")];
    assert_eq!(
      strip_settings(&settings, &deny),
      serde_json::json!({
        "model": "opus",
        "permissions": { "allow": ["Bash(ls)"], "deny": ["Read(./secrets)"] },
        "hooks": { "Stop": [{ "matcher": "", "hooks": [{ "type": "command", "command": "./ours.sh" }] }] }
      })
    );
  }

  #[test]
  fn settings_that_were_only_intents_strip_to_an_empty_document() {
    let settings = serde_json::json!({
      "permissions": { "deny": ["Edit(/AGENTS.md)"] },
      "hooks": { "Stop": [{ "hooks": [{ "type": "command", "command": "intent claude hook session-finish" }] }] }
    });
    assert_eq!(
      strip_settings(&settings, &[serde_json::json!("Edit(/AGENTS.md)")]),
      serde_json::json!({})
    );
  }

  #[test]
  fn mcp_loses_only_the_intent_server() {
    let both = serde_json::json!({ "mcpServers": { "intent": { "command": "intent" }, "other": { "command": "x" } } });
    assert_eq!(
      strip_mcp(&both),
      serde_json::json!({ "mcpServers": { "other": { "command": "x" } } })
    );
    let ours =
      serde_json::json!({ "mcpServers": { "intent": { "command": "intent", "args": ["mcp"] } } });
    assert_eq!(strip_mcp(&ours), serde_json::json!({}));
  }
}
