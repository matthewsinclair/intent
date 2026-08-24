//! `intent init` -- create a working project from an empty directory.
//!
//! Covers **ST0057 AC-06.4** (a working project from an empty directory, with
//! the text realisation exercised end-to-end from one) and **ST0056 AC-07.1**
//! (offline from the binary alone; the 0022 broken-install class
//! unconstructible).
//!
//! **THE BROKEN-INSTALL CLASS IS REMOVED BY CONSTRUCTION, NOT REPORTED WELL.**
//! v2 reads its templates from `$INTENT_HOME/lib/templates` and its only
//! recourse when they are absent is `bin/intent_init:225`, `error "Template not
//! found"`. Here they are compiled in (see
//! `build-support/embed_templates.rs`), so there is no install tree for a
//! binary to be separated from.
//!
//! **WHAT A WORKING PROJECT ACTUALLY NEEDS IS ONE FILE, AND THAT IS MEASURED
//! RATHER THAN ASSUMED.** A directory containing only
//! `intent/.config/config.json` accepts `st new` and realises end-to-end --
//! driven 2026-08-20: `created: ST0001`, then `export --format md` reporting
//! `threads 1/1  views 4/4  complete`. Everything else -- `.cache/intent.db`,
//! `.canon/`, `intent/st/`, `todo.md`, `steel_threads.md` -- is created lazily
//! by the tool at the moment it is first needed. **So the templates below are
//! starter CONTENT, and none of them is load-bearing for the project to
//! work.** Saying which is which matters: a failure to write `wip.md` must not
//! read like a failure to create a project.
//!
//! **NO TIME ENTERS THIS MODULE (D42).** Nothing here takes a timestamp and
//! nothing here asks for one -- not the OS, not the filesystem, and not the
//! database. The stamp comes back OUT of the store's own write:
//! `Store::append_event` returns what the INSERT actually wrote, and that value
//! fills both `created` in the config and `[[DATE]]` in every template. IN is
//! forbidden, OUT is fine.

use crate::event::{Envelope, Subject};
use crate::store::{Store, StoreError};
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/embedded_templates.rs"));

/// Where an embedded template goes, or why it does not go anywhere.
///
/// **THIS IS A DISPOSITION AND NOT A LIST, AND THE DIFFERENCE IS WHAT KEEPS IT
/// FROM ROTTING.** The template POPULATION is walked at build time, so it
/// cannot go stale by omission. Where each one LANDS is genuinely per-file
/// knowledge that has to be written down somewhere -- but a template with no
/// entry here is a **loud refusal**, never a silent skip, and
/// `every_embedded_template_has_a_disposition` fails the build's tests rather
/// than letting `init` quietly write fewer files than it embedded.
enum Destination {
  /// Written to this path, relative to the project root.
  At(&'static str),
  /// Deliberately not written by `init`, with the reason.
  NotByInit(&'static str),
}

use Destination::{At, NotByInit};

/// Every embedded template's disposition.
const DESTINATIONS: &[(&str, Destination)] = &[
  ("prj/_wip.md", At("intent/wip.md")),
  ("llm/_CLAUDE.md", At("CLAUDE.md")),
  // **NOT SEEDED. A HAND-MAINTAINED INDEX OF THE SOURCE TREE IS THE ONE THING
  // THIS PROJECT'S OWN HIGHLANDER RULE FORBIDS.**
  //
  // MODULES.md exists to answer "does something already do this?" That job is
  // real. The artefact is not: it is a manually-kept index of a tree the store
  // already indexes, so the registry that exists to enforce Highlander is
  // itself the duplicate. Retiring it now rather than after the search work
  // lands is deliberate (hv, 2026-08-24) -- prune first, then build only what
  // is needed, rather than carrying dross across the rewrite.
  //
  // Measured 2026-08-24 across the estate: Intent's own copy had grown to
  // ~354KB over ~367 rows while CLAUDE.md instructed the reader to check it
  // before creating any module -- an instruction nobody can follow, and the
  // verb that should answer it (`intent modules find`) is unimplemented here.
  // Lamplight had already retired its copy to a 790-byte placeholder by hv
  // ruling in June, for drift. Two estates reached the same verdict
  // independently, from opposite ends: too big to read, and too stale to trust.
  //
  // The template stays EMBEDDED rather than deleted: a project that wants a
  // registry can still be given one. What ends is `init` deciding that every
  // project has one before anybody has written a module.
  (
    "llm/_MODULES.md",
    NotByInit("a hand-maintained index of a tree the store already indexes"),
  ),
  ("llm/_DECISION_TREE.md", At("intent/llm/DECISION_TREE.md")),
  // **SEEDED, AND THE OPPOSITE CALL TO THE TEN PER-LANGUAGE FILES ABOVE.**
  //
  // Measured 2026-08-24 across four estates: every one had AUTHORED its
  // agnostic pair -- Intent, Lamplight, Baize (52 added lines of real
  // architecture), Laksa. Not one left them verbatim, while 10 of 10
  // per-language files were untouched in the repo that wrote the templates.
  // Same directory, adjacent names, opposite evidence. The seed works here
  // precisely because these are the files somebody has a reason to fill.
  //
  // v3 had DROPPED both, and nobody noticed because the v2 path still seeded
  // them -- the frozen-checkout hazard again, this time hiding a regression
  // rather than a fix. Restoring them is the repair.
  //
  // The template bodies were REWRITTEN rather than carried over. The v2
  // `_default` pair is the "incomplete template that reads like a spec" that
  // prompted this whole review: four `<!-- Replace with ... -->` prompts and a
  // placeholder directory tree in ARCHITECTURE.md, and a RULES.md paragraph
  // advertising the per-language fan-out that issue 0068 retires. Seeding
  // those verbatim would have shipped the defect the ruling exists to remove.
  // Both now say what belongs in them, say that they are empty ON PURPOSE, and
  // stop.
  ("llm/_RULES.md", At("intent/llm/RULES.md")),
  ("llm/_ARCHITECTURE.md", At("intent/llm/ARCHITECTURE.md")),
  // **`AGENTS.md` IS GENERATED, NOT SEEDED.** `intent agents sync` derives it
  // from project state, and `in-essentials` rule 2 forbids editing it by hand.
  // Seeding a file whose next regeneration overwrites it teaches the operator
  // that it is theirs to edit.
  (
    "llm/_AGENTS.md",
    NotByInit("generated by `intent agents sync`"),
  ),
  // User-owned by convention: v2's canon installer writes it only when absent
  // and never overwrites. `init` creating one would decide a project's terse
  // rule contract before anyone has written a rule.
  (
    "llm/_usage-rules.md",
    NotByInit("user-owned; the canon installer seeds it"),
  ),
  (
    "llm/_ARCHETYPES.md",
    NotByInit("archetypes are a language pack's, laid down by `intent lang init`"),
  ),
  (
    "llm/_DEPENDENCY_GRAPH.md",
    NotByInit("written when a project first declares dependencies, not at init"),
  ),
  // **THE STEEL-THREAD TEMPLATES ARE v2's VIEW RENDERER AND v3 HAS ANOTHER.**
  // Under D02 these files are GENERATED VIEWS of canon; the generator reads
  // canon and owns their shape. Laying down a template copy at init would be a
  // second source for a view, which is the defect this thread exists to remove.
  ("prj/st/ST####/info.md", NotByInit("a generated view (D02)")),
  (
    "prj/st/ST####/design.md",
    NotByInit("a generated view (D02)"),
  ),
  ("prj/st/ST####/impl.md", NotByInit("a generated view (D02)")),
  (
    "prj/st/ST####/tasks.md",
    NotByInit("a generated view (D02)"),
  ),
  (
    "prj/st/ST####/acceptance.md",
    NotByInit("a generated view (D02)"),
  ),
  ("prj/st/WP/info.md", NotByInit("a generated view (D02)")),
];

/// What `init` created.
#[derive(Debug, Clone)]
pub struct Initialised {
  pub root: PathBuf,
  pub project_name: String,
  /// The config, named separately because it is the ONLY file that makes the
  /// directory a project. Everything in `written` is starter content.
  pub config: PathBuf,
  /// Starter content actually written, in path order.
  pub written: Vec<PathBuf>,
  /// Embedded templates deliberately not written, with reasons. Reported so a
  /// short file count reads as a decision rather than as a shortfall.
  pub skipped: Vec<(&'static str, &'static str)>,
}

/// Why `init` did not initialise.
#[derive(Debug)]
pub enum InitError {
  /// There is already a project here. **Refused rather than merged**: `init`
  /// over a live project would overwrite a config someone has tuned, and the
  /// operator almost certainly meant a different directory.
  AlreadyAProject(PathBuf),
  /// An embedded template has no entry in [`DESTINATIONS`]. Loud, because the
  /// alternative is writing fewer files than were embedded and saying nothing.
  NoDisposition(&'static str),
  Io(PathBuf, std::io::Error),
  Store(StoreError),
}

impl std::fmt::Display for InitError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::AlreadyAProject(p) => write!(
        f,
        "already an Intent project: {} exists\n  remedy: `init` refuses rather than merging -- to start elsewhere, run it in an empty directory",
        p.display()
      ),
      Self::NoDisposition(t) => write!(
        f,
        "the embedded template `{t}` has no declared destination\n  remedy: this is a defect in the build, not in your project -- every embedded template must declare where it lands or why it does not"
      ),
      Self::Io(p, e) => write!(f, "could not write {}: {e}", p.display()),
      Self::Store(e) => write!(f, "could not create the project store: {e}"),
    }
  }
}

/// Create a project at `root`.
///
/// **ORDER IS LOAD-BEARING AND IT IS NOT THE OBVIOUS ONE.** The store is
/// opened and written FIRST, before the config it belongs to exists, because
/// the store is the only thing here entitled to produce a timestamp (D42) and
/// both the config's `created` and every template's `[[DATE]]` need one. The
/// alternative -- write the config, then fill its date afterwards -- writes the
/// same file twice and leaves a window in which a project exists with no
/// creation time.
pub fn init(
  root: &Path,
  project_name: &str,
  author: &str,
  intent_version: &str,
) -> Result<Initialised, InitError> {
  let config = root.join("intent/.config/config.json");
  if config.exists() {
    return Err(InitError::AlreadyAProject(config));
  }

  // Every template's disposition is resolved BEFORE anything is written, so a
  // missing one refuses on an untouched directory rather than half way through.
  let mut plan: Vec<(&'static str, &'static str, &'static str)> = Vec::new();
  let mut skipped: Vec<(&'static str, &'static str)> = Vec::new();
  for (name, body) in EMBEDDED_TEMPLATES {
    match DESTINATIONS.iter().find(|(n, _)| n == name) {
      Some((_, At(dest))) => plan.push((name, dest, body)),
      Some((_, NotByInit(why))) => skipped.push((name, why)),
      None => return Err(InitError::NoDisposition(name)),
    }
  }

  let store = Store::open(&root.join("intent/.cache/intent.db")).map_err(InitError::Store)?;
  let stamp = store
    .append_event(&Envelope::minted(
      author,
      project_name,
      "init",
      // The subject of an `init` event is the PROJECT, which has no natural
      // id of its own the way a thread does -- its name is the id anyone would
      // use to refer to it, and the project_id field beside it carries the
      // same value. Written out rather than reached for through a constructor
      // that does not exist.
      Subject {
        kind: "project".to_string(),
        id: project_name.to_string(),
      },
      serde_json::json!({ "intent_version": intent_version }),
    ))
    .map_err(InitError::Store)?;

  write(
    &config,
    &format!(
      "{{\n  \"intent_version\": {intent_version:?},\n  \"project_name\": {project_name:?},\n  \"author\": {author:?},\n  \"created\": {stamp:?},\n  \"intent_dir\": \"intent\",\n  \"languages\": []\n}}\n"
    ),
  )?;

  // **THE SUBSTITUTION IS A CLOSURE AND NOT A FUNCTION, BECAUSE D42 IS ABOUT
  // SIGNATURES.** It was `fn substitute(.., date: &str, ..)`, which is *a
  // time-typed input parameter -- a defect by inspection*, and it sat directly
  // under a module comment claiming no time enters here. A named function
  // taking a date is a place a caller can inject one; a closure over `stamp`
  // has no such parameter, and `stamp` can only have come from the INSERT
  // above. v2's placeholder vocabulary is otherwise unchanged -- these are the
  // same template files.
  let fill = |body: &str| {
    body
      .replace("[[PROJECT_NAME]]", project_name)
      .replace("[[AUTHOR]]", author)
      .replace("[[DATE]]", &stamp)
      .replace("[[INTENT_VERSION]]", intent_version)
  };

  let mut written = Vec::new();
  for (_, dest, body) in plan {
    let path = root.join(dest);
    write(&path, &fill(body))?;
    written.push(path);
  }
  written.sort();

  Ok(Initialised {
    root: root.to_path_buf(),
    project_name: project_name.to_string(),
    config,
    written,
    skipped,
  })
}

fn write(path: &Path, body: &str) -> Result<(), InitError> {
  if let Some(parent) = path.parent() {
    std::fs::create_dir_all(parent).map_err(|e| InitError::Io(parent.to_path_buf(), e))?;
  }
  std::fs::write(path, body).map_err(|e| InitError::Io(path.to_path_buf(), e))
}

#[cfg(test)]
mod tests {
  use super::*;

  /// **THE POPULATION IS WALKED, SO THIS IS THE ONLY PLACE IT CAN ROT.**
  /// Adding a template to `lib/templates/{llm,prj}` embeds it automatically;
  /// this is what forces somebody to say where it goes. Without it a new
  /// template would reach `init` as a runtime `NoDisposition` refusal in a
  /// user's empty directory rather than a red test on the machine that added
  /// it.
  #[test]
  fn every_embedded_template_has_a_disposition() {
    let missing: Vec<_> = EMBEDDED_TEMPLATES
      .iter()
      .map(|(n, _)| *n)
      .filter(|n| !DESTINATIONS.iter().any(|(d, _)| d == n))
      .collect();
    assert!(
      missing.is_empty(),
      "embedded templates with no declared destination: {missing:?}"
    );
  }

  /// The other direction, and it is not the same check. A destination naming a
  /// template that no longer exists is a roster entry for nothing -- harmless
  /// at runtime, and exactly how a list stops describing its subject.
  #[test]
  fn every_disposition_names_a_template_that_exists() {
    let orphans: Vec<_> = DESTINATIONS
      .iter()
      .map(|(n, _)| *n)
      .filter(|n| !EMBEDDED_TEMPLATES.iter().any(|(e, _)| e == n))
      .collect();
    assert!(
      orphans.is_empty(),
      "destinations naming templates that are not embedded: {orphans:?}"
    );
  }

  /// **THE EMBED IS NON-EMPTY, ASSERTED SEPARATELY FROM THE TWO ABOVE.** Both
  /// of those pass vacuously on an empty table -- no template can be missing a
  /// disposition if there are no templates. The build script already refuses a
  /// zero-length walk; this is the same property asserted where a reader of the
  /// tests can see it.
  #[test]
  fn the_embed_is_not_empty() {
    assert!(
      EMBEDDED_TEMPLATES.len() >= 10,
      "only {} template(s) embedded -- the walk found almost nothing",
      EMBEDDED_TEMPLATES.len()
    );
    assert!(
      DESTINATIONS.iter().any(|(_, d)| matches!(d, At(_))),
      "every template is NotByInit, so `init` writes no starter content at all"
    );
  }
}
