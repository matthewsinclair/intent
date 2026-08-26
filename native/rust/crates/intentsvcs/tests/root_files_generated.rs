//! AT-00.4 / ST0057 AC-00.4: **`ROOT_FILES` have a v3 generator, so that their
//! derivability is a mechanism rather than an assumption.**
//!
//! # What this deliberately does NOT assert
//!
//! **Byte-equality against the three files on disk.** Those were written by v2
//! v2.19.0 and say so in their own footers; a v3 generator that reproduced them
//! byte for byte would be reproducing a frozen release's output rather than
//! generating from this project's state. The criterion asks whether v3 CAN
//! produce each of the three -- and separately whether the failure that
//! prompted it, a root file emptied on 2026-08-18, is now recoverable.
//!
//! # The control does the work
//!
//! "Every root file renders to something" is satisfied by a generator that
//! emits the template unchanged, so the arms below are built around what must
//! be ABSENT: an undeclared language's section, and every unsubstituted token.
//! A generator that kept everything fails those, and one that kept nothing
//! fails the positive arms -- neither can pass by being lazy in one direction.

use intentsvcs::project::Config;
use intentsvcs::rootfiles::{self, Fault};
use intentsvcs::sync::ROOT_FILES;
use intentsvcs::views::RenderContext;
use testkit::repo_root;

/// A config carrying exactly the declared languages, built through the real
/// deserialiser so a field this test forgets is a field `serde` defaults rather
/// than one this test invents.
fn config(languages: &[&str]) -> Config {
  serde_json::from_value(serde_json::json!({
    "intent_version": "3.0.0-probe",
    "project_name": "ProbeProject",
    "author": "Probe Author",
    "languages": languages,
  }))
  .expect("the probe config deserialises")
}

fn ctx() -> RenderContext<'static> {
  RenderContext {
    version: "3.0.0-probe",
    // Root files only; nothing here renders todo.md.
    todo_watermark: None,
  }
}

#[test]
fn every_root_file_has_a_template_this_binary_can_render() {
  let home = repo_root();
  let cfg = config(&["rust"]);

  let views = rootfiles::render_all(&home, &cfg, &ctx()).expect("every root file renders");

  assert_eq!(
    views.len(),
    ROOT_FILES.len(),
    "render_all produced {} file(s) for {} declared in ROOT_FILES -- the generator and the roster \
     must agree, and a generator covering a subset is the shape this criterion exists to prevent",
    views.len(),
    ROOT_FILES.len()
  );

  for (view, name) in views.iter().zip(ROOT_FILES) {
    assert_eq!(
      view.path.to_string_lossy(),
      *name,
      "render_all emitted files out of ROOT_FILES order, so the pairing above is not what it \
       appears to be"
    );
    // Non-empty is the floor, and it is the floor BECAUSE of the incident: the
    // live instance this criterion cites is a root file that became empty.
    assert!(
      !view.content.trim().is_empty(),
      "`{name}` rendered to nothing -- the generator exists and cannot put the file back, which is \
       the state this criterion was written to end"
    );
    assert!(
      !view.content.contains("[["),
      "`{name}` rendered with an unsubstituted token still in it -- the literal `[[...]]` would \
       ship to whoever reads the generated file, and the person running the generator is the one \
       who never sees it"
    );
  }
}

#[test]
fn an_emptied_root_file_is_recoverable_which_is_the_live_instance() {
  let home = repo_root();
  let cfg = config(&["rust"]);

  // The incident, reproduced: the file on disk holds nothing.
  let emptied = "";
  let generated = rootfiles::render(&home, "AGENTS.md", &cfg, &ctx()).expect("AGENTS.md renders");

  assert_ne!(
    generated.trim(),
    emptied,
    "the generator's output is indistinguishable from an emptied file, so nothing detects the \
     emptying and nothing restores it"
  );
  assert!(
    generated.len() > 1000,
    "AGENTS.md rendered to {} byte(s), which is short enough that this arm would pass on a stub -- \
     the recovery this asserts has to put back a real document",
    generated.len()
  );
}

#[test]
fn a_declared_language_is_carried_and_an_undeclared_one_is_absent() {
  let home = repo_root();

  let rust_only = rootfiles::render(&home, "AGENTS.md", &config(&["rust"]), &ctx())
    .expect("the rust-only project renders");
  let elixir_only = rootfiles::render(&home, "AGENTS.md", &config(&["elixir"]), &ctx())
    .expect("the elixir-only project renders");

  assert!(
    rust_only.contains("cargo test"),
    "a project declaring `rust` did not get the rust section"
  );
  assert!(
    elixir_only.contains("mix test"),
    "a project declaring `elixir` did not get the elixir section"
  );

  // **THE CONTROL, and without it the two arms above pass on a generator that
  // emits every section to everyone.**
  assert!(
    !rust_only.contains("mix test"),
    "a project declaring only `rust` was told to run `mix test` -- the blocks are not being \
     resolved at all, and the positive arms above prove nothing"
  );
  assert!(
    !elixir_only.contains("cargo test"),
    "a project declaring only `elixir` was told to run `cargo test`, same defect in the other \
     direction"
  );

  // And the two renders must actually DIFFER, which is the property the four
  // assertions above only imply.
  assert_ne!(
    rust_only, elixir_only,
    "two projects with different declared languages rendered identically"
  );
}

#[test]
fn a_project_declaring_no_language_is_told_how_to_declare_one() {
  let home = repo_root();
  let none = rootfiles::render(&home, "AGENTS.md", &config(&[]), &ctx())
    .expect("a project with no declared language still renders");

  assert!(
    none.contains("intent lang init"),
    "a project with no declared languages got no prerequisites and no way to fix that -- the empty \
     case is the one a fresh `intent init` lands in"
  );
  assert!(
    !none.contains("cargo test") && !none.contains("mix test"),
    "a project declaring nothing was given language sections anyway"
  );
}

#[test]
fn the_date_token_refuses_rather_than_passing_through() {
  let cfg = config(&["rust"]);
  let fault = rootfiles::substitute("stamped [[DATE]] here", &cfg, &ctx())
    .expect_err("`[[DATE]]` is not a token this generator substitutes");

  assert_eq!(fault, Fault::UnknownToken("DATE".to_string()));

  // **The point is not that DATE specifically is refused -- it is that NOTHING
  // unknown passes through.** A generator that special-cased this one token
  // would ship the next typo as literal text.
  let typo = rootfiles::substitute("[[PROJECT_NAM]]", &cfg, &ctx())
    .expect_err("a misspelt token refuses too");
  assert_eq!(typo, Fault::UnknownToken("PROJECT_NAM".to_string()));

  // The positive control: a token that IS known expands, so the two refusals
  // above are about the token and not about substitution being broken.
  assert_eq!(
    rootfiles::substitute("[[PROJECT_NAME]]", &cfg, &ctx()).expect("a known token expands"),
    "ProbeProject"
  );
}

#[test]
fn a_malformed_block_refuses_and_names_which_one() {
  let cfg = config(&["rust"]);

  assert_eq!(
    rootfiles::substitute("[[#lang rust]]\nkept\n", &cfg, &ctx())
      .expect_err("an unclosed block refuses"),
    Fault::Unclosed("[[#lang rust]]".to_string()),
    "an unclosed block silently swallowed the rest of the template"
  );

  assert_eq!(
    rootfiles::substitute("body\n[[/lang]]\n", &cfg, &ctx())
      .expect_err("an unopened close refuses"),
    Fault::Unopened("[[/lang]]".to_string())
  );

  assert_eq!(
    rootfiles::substitute(
      "[[#lang rust]]\n[[#lang shell]]\nx\n[[/lang]]\n[[/lang]]\n",
      &cfg,
      &ctx()
    )
    .expect_err("nested blocks refuse"),
    Fault::Nested("[[#lang shell]]".to_string(), "[[#lang rust]]".to_string()),
    "blocks nested silently, so a template can express a condition this generator does not \
     actually evaluate"
  );

  // The positive control: the same shapes, well formed, expand.
  assert_eq!(
    rootfiles::substitute("[[#lang rust]]\nkept\n[[/lang]]\n", &cfg, &ctx()).expect("well formed"),
    "kept\n"
  );
  assert_eq!(
    rootfiles::substitute("[[#lang swift]]\ndropped\n[[/lang]]\n", &cfg, &ctx())
      .expect("well formed"),
    "",
    "an undeclared language's block was kept"
  );
}

#[test]
fn the_generator_is_not_a_view_and_must_not_become_one() {
  // **A behavioural guard on the hazard the module doc names.** `views::View`
  // is the type `organize` dehydrates; root files escape only because nothing
  // classifies them as belonging to a thread. If someone later pushes these
  // three through `views::render_all`, this catches it -- the root files would
  // appear among the views, one classifier change from removal, and `AGENTS.md`
  // is the file this criterion exists because something already emptied it.
  let root = repo_root();
  let project = intentsvcs::project::Project::open(&root).expect("the real project opens");
  let canon = intentsvcs::ingest::read(&project).expect("canon reads");
  let views = intentsvcs::views::render_all(&project, &canon, &ctx());

  for name in ROOT_FILES {
    assert!(
      !views
        .iter()
        .any(|v| v.path.file_name().and_then(|n| n.to_str()) == Some(*name)),
      "`{name}` is being rendered as a generated VIEW. Views are disk-discarded and dehydrate with \
       their thread; root files are not claimed by any thread and survive only because of that. \
       Putting them in this list makes three root files one classifier change away from removal"
    );
  }
  assert!(
    !views.is_empty(),
    "render_all produced no views at all, so the absence asserted above is vacuous"
  );
}

// ---------------------------------------------------------------------------
// The write half (`intent agents sync`)
// ---------------------------------------------------------------------------

#[test]
fn sync_puts_down_exactly_what_generate_emits() {
  let home = repo_root();
  let cfg = config(&["rust"]);
  let dir = tempfile::tempdir().expect("a temp project root");

  let written = rootfiles::sync(dir.path(), &home, "AGENTS.md", &cfg, &ctx()).expect("sync writes");
  let on_disk = std::fs::read_to_string(&written).expect("the written file reads back");
  let emitted = rootfiles::render(&home, "AGENTS.md", &cfg, &ctx()).expect("generate emits");

  assert_eq!(
    on_disk, emitted,
    "`sync` put down something other than what `generate` emits, so the two halves are separate \
     answers to one question and only one of them is ever looked at"
  );
  assert_eq!(written, dir.path().join("AGENTS.md"));
}

#[test]
fn sync_leaves_no_bak_sibling_and_that_is_the_ratified_deviation() {
  let home = repo_root();
  let cfg = config(&["rust"]);
  let dir = tempfile::tempdir().expect("a temp project root");

  std::fs::write(dir.path().join("AGENTS.md"), "the previous version").expect("a prior file");
  rootfiles::sync(dir.path(), &home, "AGENTS.md", &cfg, &ctx()).expect("sync overwrites");

  // v2 wrote `AGENTS.md.bak` here. hv ratified dropping it (2026-08-19): it was
  // gitignored, so it never reached git and guarded a loss git already
  // prevents -- and v3 already carries D35's snapshots in `backup`. **Pinned so
  // it cannot quietly come back as a second backup mechanism.**
  assert!(
    !dir.path().join("AGENTS.md.bak").exists(),
    "a `.bak` sibling reappeared -- that is a second backup mechanism beside `backup.rs`, for one \
     file, guarding a loss git already prevents"
  );
  let siblings: Vec<String> = std::fs::read_dir(dir.path())
    .expect("the temp root lists")
    .flatten()
    .map(|e| e.file_name().to_string_lossy().to_string())
    .collect();
  assert_eq!(
    siblings,
    vec!["AGENTS.md".to_string()],
    "sync left more than the file it was asked to write: {siblings:?}"
  );
}

#[test]
fn a_second_sync_with_the_same_bytes_does_not_move_the_mtime() {
  let home = repo_root();
  let cfg = config(&["rust"]);
  let dir = tempfile::tempdir().expect("a temp project root");

  let p = rootfiles::sync(dir.path(), &home, "AGENTS.md", &cfg, &ctx()).expect("first sync");
  let first = std::fs::metadata(&p)
    .expect("stat")
    .modified()
    .expect("mtime");

  rootfiles::sync(dir.path(), &home, "AGENTS.md", &cfg, &ctx()).expect("second sync");
  let second = std::fs::metadata(&p)
    .expect("stat")
    .modified()
    .expect("mtime");

  assert_eq!(
    first, second,
    "an unchanged root file was rewritten, moving its mtime -- that is the churn loop, and on a \
     file at the project root it wakes every watcher in the tree for nothing"
  );

  // **The control: a run that SHOULD write must move it**, or the equality
  // above is satisfied by a `sync` that never writes at all.
  rootfiles::sync(dir.path(), &home, "AGENTS.md", &config(&["elixir"]), &ctx())
    .expect("third sync, different content");
  let third = std::fs::metadata(&p)
    .expect("stat")
    .modified()
    .expect("mtime");
  assert_ne!(
    second, third,
    "changed content did not move the mtime either, so `sync` is not writing and the skip asserted \
     above proves nothing"
  );
}
