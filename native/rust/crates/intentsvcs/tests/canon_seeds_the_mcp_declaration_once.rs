//! **`AC-24.1`: `.mcp.json` declares `intent mcp` to Claude Code, seeded when
//! absent and never overwritten.** AT-24.1.
//!
//! **SEEDED, NOT SYNCED, AND THE DIFFERENCE IS THE WHOLE ROW.** A project that
//! has edited this file -- a second server, an env var, a different command --
//! has decided something, and curing that variation by overwriting it destroys
//! the decision. It goes through the same `seed_if_absent` door `usage-rules.md`
//! and `.intent_critic.yml` use, and `--force` does not reach it: that flag's
//! own help names `CLAUDE.md` and `.intent_critic.yml` and stops there.
//!
//! **AND SEEDED-WHEN-ABSENT IS NOT A WAY TO DECLINE IT**, which is why
//! `--skip-settings` covers it (vc's ruling, 2026-09-12). Deleting the file
//! only means the next `--apply` seeds it again, so without that flag a project
//! that wants no MCP server declared in its editor has no way to say so. One
//! flag for the harness-wiring class, because `.claude/settings.json` and this
//! file exist only to wire Claude Code, and a project told no about the
//! lifecycle hooks that then finds an MCP server declared anyway was answered
//! by a flag that only half meant it.

use std::path::Path;

use intentsvcs::canon;

fn home() -> std::path::PathBuf {
  testkit::repo_root()
}

fn hooks_dir(fx: &crate::common::Fixture) -> std::path::PathBuf {
  let hooks = fx.root().join(".git/hooks");
  std::fs::create_dir_all(&hooks).expect("mkdir hooks");
  hooks
}

fn apply(fx: &crate::common::Fixture, hooks: &Path, opts: canon::Options) -> canon::Applied {
  let project = fx.project();
  canon::apply(
    fx.root(),
    &home(),
    project.config(),
    &crate::common::ctx(),
    Some(hooks),
    opts,
  )
  .expect("canon apply")
}

/// Somebody's own `.mcp.json`: a different server, recognisable bytes, so the
/// test reads them back rather than merely asserting a disposition.
const THEIRS: &str = r#"{
  "mcpServers": {
    "not-intent": {
      "command": "something-else",
      "args": ["serve"]
    }
  }
}
"#;

#[test]
fn absent_it_is_seeded_and_names_intent_mcp() {
  let fx = crate::common::Fixture::new();
  let hooks = hooks_dir(&fx);
  let path = fx.root().join(".mcp.json");
  assert!(!path.exists(), "the fixture already had one");

  let applied = apply(&fx, &hooks, canon::Options::default());

  assert!(
    applied.written.iter().any(|p| p == &path),
    "`.mcp.json` was not reported as written: {:?}",
    applied.written
  );
  let body = std::fs::read_to_string(&path).expect("read the seeded file");
  assert!(
    body.contains("\"intent\"") && body.contains("\"mcp\""),
    "the seeded declaration does not name `intent mcp`: {body}"
  );
  // The register's `exposed_on_mcp` rows decide what is exposed. A tool list
  // here would be a second place that decides it.
  assert!(
    !body.contains("tools"),
    "the declaration carries a tool list, which is a second home for what the register decides: {body}"
  );
}

#[test]
fn present_it_is_preserved_byte_for_byte_even_under_force() {
  let fx = crate::common::Fixture::new();
  let hooks = hooks_dir(&fx);
  let path = fx.root().join(".mcp.json");
  std::fs::write(&path, THEIRS).expect("plant somebody's own");

  let applied = apply(
    &fx,
    &hooks,
    canon::Options {
      // **`--force` MUST NOT REACH IT.** Its help names two files and this is
      // not one of them; widening a documented flag past its own text is how a
      // destructive option grows in the dark.
      force: true,
      ..canon::Options::default()
    },
  );

  assert_eq!(
    std::fs::read_to_string(&path).expect("read it back"),
    THEIRS,
    "canon rewrote a `.mcp.json` it did not write"
  );
  assert!(
    applied.preserved.iter().any(|p| p == &path),
    "the preserved file was not reported as preserved: {:?}",
    applied.preserved
  );
}

#[test]
fn skip_settings_declines_it_and_reports_it_skipped() {
  let fx = crate::common::Fixture::new();
  let hooks = hooks_dir(&fx);
  let path = fx.root().join(".mcp.json");

  let applied = apply(
    &fx,
    &hooks,
    canon::Options {
      skip_settings: true,
      ..canon::Options::default()
    },
  );

  assert!(
    !path.exists(),
    "`--skip-settings` seeded the file it declined"
  );
  assert!(
    applied.skipped.iter().any(|p| p == &path),
    "`--skip-settings` did not report `.mcp.json` as skipped: {:?}",
    applied.skipped
  );
  // Skipped is not preserved and not held: nothing was read, so nothing can be
  // said about whose the file is.
  assert!(
    !applied.preserved.iter().any(|p| p == &path) && !applied.held.iter().any(|p| p == &path),
    "a skipped path was also reported as preserved or held"
  );
}
