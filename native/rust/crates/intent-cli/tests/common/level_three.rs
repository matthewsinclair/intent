//! A level-3 estate for ST0076 WP-07's arms (AT-07.1, AT-07.2): a Rust project
//! indexed through the binary, with a resolution stored through the facade's
//! own door by a reader that hands back the trace it was built with.
//!
//! **SEEDED IN PROCESS, ASKED THROUGH THE BINARY.** Running a toolchain is a
//! reader's concern and has its own arms. What these prove is what every face
//! does with the rows a run stored, so the rows go in through
//! `Facade::index_resolve` and the questions go through the binary a person
//! runs, whose store is the same file.

use intentsvcs::facade::{Facade, FacadeContext};
use intentsvcs::index::resolved::{Manifest, Read, Reference, Resolver, Scope, Trace, Unresolved};
use intentsvcs::index::symbols::Symbol;
use intentsvcs::model::sha256_hex;
use std::collections::BTreeMap;
use std::path::Path;

/// `helper` is called on line 2 and defined on line 6; `other` is called on
/// line 3 and defined nowhere in the project.
pub const LIB: &str = "pub fn main() {\n  helper();\n  other();\n}\n\npub fn helper() {}\n";
/// `helper` is called on line 2.
pub const TWO: &str = "pub fn two() {\n  helper();\n}\n";

/// The one target `helper` resolves to, as the reader below prints it.
pub const HELPER: &str = "crate::helper()";

/// The binary in `cwd`, under the fixture HOME: stdout, stderr and the code.
pub fn run(args: &[&str], cwd: &Path) -> (String, String, i32) {
  let out = super::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// The crate's manifest, which is what Rust's reader finds a project by.
pub const CARGO_TOML: &str =
  "[package]\nname = \"level-three-fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n";

/// A Rust project declaring rust, with its `Cargo.toml`, `src/lib.rs` and
/// `src/two.rs` indexed and nothing resolved.
pub fn estate() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  for args in [
    &["init", "level-three-fixture"][..],
    &["lang", "init", "rust"][..],
  ] {
    let (_, err, code) = run(args, root);
    assert_eq!(code, 0, "fixture {args:?} failed: {err}");
  }
  std::fs::create_dir_all(root.join("src")).expect("mkdir src");
  std::fs::write(root.join("Cargo.toml"), CARGO_TOML).expect("write Cargo.toml");
  std::fs::write(root.join("src/lib.rs"), LIB).expect("write src/lib.rs");
  std::fs::write(root.join("src/two.rs"), TWO).expect("write src/two.rs");
  let (_, err, code) = run(&["index", "rebuild"], root);
  assert_eq!(code, 0, "fixture index failed: {err}");
  dir
}

/// A reader that hands back the outcome it was built with, and prints a
/// definition the way the Rust reader prints a function.
pub struct Handed(pub Result<Trace, Unresolved>);

impl Resolver for Handed {
  fn lang(&self) -> &'static str {
    "rust"
  }
  fn tool(&self) -> &'static str {
    "fixture-analyzer"
  }
  fn manifest(&self) -> Manifest {
    intentsvcs::index::rust_analyzer::MANIFEST
  }
  fn trace(&self, _: &Scope<'_>) -> Result<Trace, Unresolved> {
    self.0.clone()
  }
  fn target_of(&self, def: &Symbol) -> Option<String> {
    Some(format!("crate::{}()", def.name))
  }
}

/// The facade over the project at `root`, in this process.
pub fn facade(root: &Path) -> Facade {
  let project = intentsvcs::project::Project::open(root).expect("the project opens");
  let ctx = FacadeContext {
    principal: "test".to_string(),
    project_id: String::new(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  Facade::open(project, ctx).expect("the facade opens")
}

/// Store `outcome` as rust's run over every declared language, through the
/// facade's own door.
pub fn resolve(root: &Path, outcome: Result<Trace, Unresolved>) {
  resolve_as(root, None, outcome);
}

/// Store `outcome` as rust's run, asked for by `lang` or over every declared
/// language, through the facade's own door.
pub fn resolve_as(root: &Path, lang: Option<&str>, outcome: Result<Trace, Unresolved>) {
  let readers: Vec<Box<dyn Resolver>> = vec![Box::new(Handed(outcome))];
  facade(root)
    .index_resolve(lang, false, &readers)
    .expect("the door answers");
}

fn to(path: &str, line: u32, name: &str, target: &str) -> Reference {
  Reference {
    path: Some(path.to_string()),
    line: Some(line),
    name: name.to_string(),
    target: target.to_string(),
    target_path: None,
    target_line: None,
  }
}

/// Both files read as they were indexed: `helper` in each resolves to
/// [`HELPER`], and `other` in `src/lib.rs` to two definitions.
pub fn both() -> Trace {
  Trace {
    read: vec![
      Read {
        path: "src/lib.rs".to_string(),
        sha256: sha256_hex(LIB.as_bytes()),
      },
      Read {
        path: "src/two.rs".to_string(),
        sha256: sha256_hex(TWO.as_bytes()),
      },
    ],
    references: vec![
      to("src/lib.rs", 2, "helper", HELPER),
      to("src/lib.rs", 3, "other", "crate::a::other()"),
      to("src/lib.rs", 3, "other", "crate::b::other()"),
      to("src/two.rs", 2, "helper", HELPER),
    ],
    excluded: BTreeMap::new(),
  }
}

/// `intent search <args> --json`, refused on a non-zero exit.
pub fn envelope(root: &Path, args: &[&str]) -> serde_json::Value {
  let mut full = vec!["search"];
  full.extend_from_slice(args);
  full.push("--json");
  let (out, err, code) = run(&full, root);
  assert_eq!(code, 0, "{full:?} failed: {err}");
  serde_json::from_str(&out).expect("the envelope is JSON")
}

/// The structural hits of an envelope, as `path:line` with each hit.
pub fn structural(answer: &serde_json::Value) -> Vec<(String, serde_json::Value)> {
  answer["groups"]
    .as_array()
    .expect("groups")
    .iter()
    .filter(|group| group["tier"] == "structural")
    .flat_map(|group| group["hits"].as_array().cloned().unwrap_or_default())
    .map(|hit| {
      let place = format!(
        "{}:{}",
        hit["path"].as_str().unwrap_or_default(),
        hit["span"]["start_line"]
      );
      (place, hit)
    })
    .collect()
}

/// The text a tool call answered, from an MCP session's frames.
pub fn tool_text(frames: &[serde_json::Value], id: i64) -> (bool, String) {
  let result = &frames
    .iter()
    .find(|frame| frame["id"] == id)
    .expect("a response to the call")["result"];
  (
    result["isError"] == true,
    result["content"][0]["text"]
      .as_str()
      .unwrap_or_default()
      .to_string(),
  )
}

pub const INITIALIZE: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"drive","version":"0"}}}"#;
