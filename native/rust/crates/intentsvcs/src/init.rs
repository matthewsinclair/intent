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
  /// GENERATED from project state to this root-file name -- not seeded.
  ///
  /// **THE THIRD CASE THE FIRST TWO COULD NOT EXPRESS.** `At` copies the
  /// template with `init`'s four tokens filled in; `NotByInit` writes nothing.
  /// A file that `init` must produce but must NOT seed fits neither, and hv's
  /// 2026-09-09 ruling that `init` lays `AGENTS.md` down created exactly that
  /// case: the file has to exist from the first moment, and seeding it would
  /// teach the operator it is theirs to edit when the next `agents sync`
  /// overwrites it.
  ///
  /// **THE DISTINCTION IS THE POINT, NOT THE PLUMBING.** Flipping `_AGENTS.md`
  /// to `At` would have laid the file down too, and would have written the
  /// template's UNRESOLVED blocks -- `init`'s `fill` replaces four tokens and
  /// resolves no `[[#lang ...]]` block, so a seeded `AGENTS.md` would carry
  /// markup instead of the project's languages. The failure would have looked
  /// like a template bug rather than a wrong disposition.
  Generated(&'static str),
  /// Deliberately not written by `init`, with the reason.
  ///
  /// **THIS VARIANT CARRIES REASONS THAT POINT OPPOSITE WAYS AND THAT IS
  /// DELIBERATE**: `usage-rules.md` is USER-OWNED (init must not decide it),
  /// while `ARCHETYPES.md` and `DEPENDENCY_GRAPH.md` belong to another verb.
  /// What it must never again carry is *generated* -- that reading is now
  /// `Generated`, because it is the one reason that implies `init` SHOULD
  /// write the file.
  NotByInit(&'static str),
}

use Destination::{At, Generated, NotByInit};

/// Every embedded template's disposition.
const DESTINATIONS: &[(&str, Destination)] = &[
  ("prj/_wip.md", At("intent/wip.md")),
  ("llm/_CLAUDE.md", At("CLAUDE.md")),
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
  // **`AGENTS.md` IS GENERATED, NOT SEEDED, AND `init` NOW GENERATES IT.**
  // `intent agents sync` derives it from project state and `in-essentials`
  // rule 2 forbids editing it by hand, so seeding a file whose next
  // regeneration overwrites it would teach the operator it is theirs to edit.
  // That objection is unchanged and is why this is `Generated` rather than
  // `At`.
  //
  // **WHAT CHANGED IS THAT NOT-SEEDED WAS BEING READ AS NOT-WRITTEN.** A
  // project born on v3 had no `AGENTS.md` at all, so `intent agents validate`
  // returned rc=1 -- `error: AGENTS.md not found at project root` -- against a
  // project the tool had just created. hv ruled 2026-09-09 that `init` lays it
  // down. `intent agents init` already did exactly this and nothing called it.
  ("llm/_AGENTS.md", Generated("AGENTS.md")),
  // User-owned by convention: v2's canon installer writes it only when absent
  // and never overwrites. `init` creating one would decide a project's terse
  // rule contract before anyone has written a rule.
  (
    "llm/_usage-rules.md",
    NotByInit("user-owned; the canon installer seeds it"),
  ),
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
  /// Each language asked for, in the order asked, with `true` where this run
  /// declared it and `false` where an earlier name in the same list already
  /// had. Empty when none was asked for.
  pub languages: Vec<(String, bool)>,
}

/// Why `init` did not initialise.
#[derive(Debug)]
pub enum InitError {
  /// There is already a project here. **Refused rather than merged**: `init`
  /// over a live project would overwrite a config someone has tuned, and the
  /// operator almost certainly meant a different directory.
  AlreadyAProject(PathBuf),
  /// This directory is not a project, but it already holds files `init`
  /// writes. **The absence of a config makes a directory NOT-A-PROJECT; it
  /// never made it EMPTY**, and collapsing those two claims is what let
  /// `init` destroy a `CLAUDE.md` or an `AGENTS.md` somebody wrote, at exit 0,
  /// under a line reporting the file as created.
  ///
  /// Carries EVERY collision rather than the first: an operator told about one
  /// moves it, re-runs, and loses the next.
  WouldOverwrite(Vec<PathBuf>),
  /// An embedded template has no entry in `DESTINATIONS`. Loud, because the
  /// alternative is writing fewer files than were embedded and saying nothing.
  NoDisposition(&'static str),
  Io(PathBuf, std::io::Error),
  Store(StoreError),
  /// A `Generated` destination's embedded template would not expand. **Named
  /// separately from `Io` because the disk is not what failed** -- the bytes
  /// are compiled into this binary, so a fault here is a defect in the build
  /// and not something an operator can fix in their project.
  Render(&'static str, crate::rootfiles::Fault),
  /// A language this build cannot serve was asked for. Refused before
  /// anything is written, as `intent init --lang` checks it too.
  Undeclarable(Vec<String>),
}

impl std::fmt::Display for InitError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::AlreadyAProject(p) => write!(
        f,
        "already an Intent project: {} exists\n  remedy: `init` refuses rather than merging -- to start elsewhere, run it in an empty directory",
        p.display()
      ),
      Self::WouldOverwrite(paths) => {
        writeln!(
          f,
          "this directory is not an Intent project, but it already holds files `init` writes:"
        )?;
        for path in paths {
          writeln!(f, "    {}", path.display())?;
        }
        write!(
          f,
          "  remedy: `init` refuses rather than overwriting a file it did not write -- move or delete the files above, or run it somewhere else. Nothing has been written."
        )
      }
      Self::NoDisposition(t) => write!(
        f,
        "the embedded template `{t}` has no declared destination\n  remedy: this is a defect in the build, not in your project -- every embedded template must declare where it lands or why it does not"
      ),
      Self::Io(p, e) => write!(f, "could not write {}: {e}", p.display()),
      Self::Store(e) => write!(f, "could not create the project store: {e}"),
      // The remedy is the error's own -- `RootFileError` distinguishes a
      // missing template from a malformed one from an unresolvable install,
      // and a sentence here could not tell which happened.
      Self::Render(name, fault) => write!(
        f,
        "the embedded template for {name} would not expand: {fault}\n  remedy: this is a defect in the build, not in your project -- the template is compiled into this binary"
      ),
      Self::Undeclarable(langs) => write!(
        f,
        "not a language this build can serve: {}\n  remedy: run `intent lang list` for the languages you can declare. Nothing has been written.",
        langs.join(", ")
      ),
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
  init_with_languages(root, project_name, author, intent_version, &[])
}

/// [`init`], declaring `languages` in the config it writes.
///
/// **THE LANGUAGES GO INTO THE FIRST WRITE OF THE CONFIG, BECAUSE THE ROOT
/// FILES ARE RENDERED FROM IT.** `intent init --lang` used to create the project
/// and declare the languages afterwards, so the `AGENTS.md` this function
/// renders said "None declared" beside a config that declared them (issue
/// 0557). Declaring them here puts them in the config before the generated root
/// files read it.
pub fn init_with_languages(
  root: &Path,
  project_name: &str,
  author: &str,
  intent_version: &str,
  languages: &[String],
) -> Result<Initialised, InitError> {
  let config = root.join("intent/.config/config.json");
  let undeclarable: Vec<String> = languages
    .iter()
    .filter(|l| !crate::rules::is_declarable(l))
    .cloned()
    .collect();
  if !undeclarable.is_empty() {
    return Err(InitError::Undeclarable(undeclarable));
  }
  // In the order asked, each name once: the same rule `Config::declare_language`
  // applies to `intent lang init`, so a repeated name is reported as already
  // declared rather than written twice.
  let mut declared: Vec<String> = Vec::new();
  let mut outcomes: Vec<(String, bool)> = Vec::new();
  for lang in languages {
    let first = !declared.contains(lang);
    if first {
      declared.push(lang.clone());
    }
    outcomes.push((lang.clone(), first));
  }
  let languages_json = serde_json::to_string(&declared)
    .map_err(|e| InitError::Io(config.clone(), std::io::Error::other(e)))?;
  if config.exists() {
    return Err(InitError::AlreadyAProject(config));
  }

  // Every template's disposition is resolved BEFORE anything is written, so a
  // missing one refuses on an untouched directory rather than half way through.
  let mut plan: Vec<(&'static str, &'static str, &'static str)> = Vec::new();
  let mut skipped: Vec<(&'static str, &'static str)> = Vec::new();
  let mut generated: Vec<(&'static str, &'static str)> = Vec::new();
  for (name, body) in EMBEDDED_TEMPLATES {
    match DESTINATIONS.iter().find(|(n, _)| n == name) {
      Some((_, At(dest))) => plan.push((name, dest, body)),
      // **THE BODY IS CARRIED, AND AC-07.1 IS WHY.** The first version of this
      // reached `rootfiles::generate`, which resolves the INSTALL root -- and
      // `embedded_init::init_works_from_the_binary_alone` went red, correctly:
      // the criterion is that a fresh `init` works offline from the binary
      // alone, so `init` renders from the EMBEDDED template and never from a
      // copy on disk. A generated file differs from an `At` file in how it is
      // EXPANDED, not in where its template comes from.
      Some((_, Generated(dest))) => generated.push((*dest, *body)),
      Some((_, NotByInit(why))) => skipped.push((name, why)),
      None => return Err(InitError::NoDisposition(name)),
    }
  }

  // **EVERY DESTINATION IS TESTED BEFORE THE FIRST BYTE MOVES, AND THE REFUSAL
  // NAMES ALL OF THEM.** The only thing `init` used to check was the config,
  // and everything after it was a bare `fs::write` -- so running it in a
  // directory that already held a `CLAUDE.md`, an `AGENTS.md` or an
  // `intent/wip.md` destroyed the operator's file and then REPORTED IT AS
  // CREATED. Not a silent failure: a silent success, which is worse, because
  // the output is indistinguishable from the run that was meant.
  //
  // **THIS SITS ABOVE `Store::open` DELIBERATELY.** Opening the store creates
  // `intent/.cache/intent.db` and migrates one that is already there, so a
  // check placed after it has already written to the directory it is deciding
  // whether to write to. That is also why the store's own path is in the set:
  // a stray db with no config beside it is somebody's data, and `init` has no
  // more business migrating it unasked than it has overwriting their
  // `CLAUDE.md`.
  //
  // **`.prettierignore` IS DELIBERATELY ABSENT FROM THE SET, and its writer is
  // the reason.** `facade::converge_formatter_exclusion` appends the patterns
  // that are missing and leaves everything already present untouched, so an
  // operator's file is not at risk from it. Refusing over a file most
  // repositories already carry would make `init` unusable in exactly the
  // populated directory this check exists to protect -- a check that fires on
  // the safe case as readily as the dangerous one teaches people to work
  // around it.
  let mut occupied: Vec<PathBuf> = plan
    .iter()
    .map(|(_, dest, _)| root.join(dest))
    .chain(generated.iter().map(|(dest, _)| root.join(dest)))
    .chain([
      root.join("intent/.intentfiles"),
      root.join("intent/.cache/intent.db"),
      // The two aggregate views `init` writes below (issue 0448). Named by
      // path here because the check runs before a `Project` exists to derive
      // them, under the `intent_dir` this function writes into the config.
      root.join("intent/st/steel_threads.md"),
      root.join("intent/todo.md"),
    ])
    .filter(|path| path.exists())
    .collect();
  if !occupied.is_empty() {
    occupied.sort();
    return Err(InitError::WouldOverwrite(occupied));
  }

  let store = Store::open(&root.join("intent/.cache/intent.db")).map_err(InitError::Store)?;
  // The event names who created the project by the rule every later event
  // uses (ST0078 P1), so an `init` with no recorded author names git's identity.
  let principal = crate::facade::author_at(root, author);
  let stamp = store
    .append_event(&Envelope::minted(
      &principal,
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

  // The schedule's default has ONE home: [`crate::project::BackupConfig`]. A
  // second literal here is how a default drifts from the thing that reads it.
  let backup_schedule = crate::project::BackupConfig::default().schedule;

  // **`project_id` IS MINTED HERE BECAUSE `init` IS ONE OF THE TWO WAYS A
  // PROJECT COMES TO EXIST, AND IT WAS THE ONE THAT DID NOT MINT.** D15
  // ratifies `(project_id, natural_id)` as the global identity and D20 spells
  // both daemon events with the project half, so a project without one is a
  // ratified seam with an unbuilt half rather than a project that opted out.
  //
  // **THE MISSING HALF WAS INVISIBLE PRECISELY WHERE IT WOULD HAVE BEEN
  // NOTICED.** `stamp_version` mints during migration, so every tree anyone
  // develops in -- including Intent's own, which is self-hosted through a v2
  // migration -- carries an id and looks correct. The population that lacked
  // one was every project `init` creates, which is all of them from here.
  //
  // Minted BESIDE THE CONFIG WRITE rather than lazily at first use: a lazily
  // minted id differs between two readers that race, which would make the
  // identity depend on who asked first.
  let project_id = crate::project::mint_project_id();
  write(
    &config,
    &format!(
      "{{\n  \"intent_version\": {intent_version:?},\n  \"project_name\": {project_name:?},\n  \"author\": {author:?},\n  \"project_id\": {project_id:?},\n  \"created\": {stamp:?},\n  \"intent_dir\": \"intent\",\n  \"languages\": {languages_json},\n  \"backup\": {{\n    \"schedule\": {backup_schedule:?}\n  }}\n}}\n"
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
      // The token feeds only the footer, so it takes the footer's form from
      // the one home `rootfiles::render` also reads.
      .replace(
        "[[INTENT_VERSION]]",
        crate::views::banner_version(intent_version),
      )
  };

  let mut written = Vec::new();
  for (_, dest, body) in plan {
    let path = root.join(dest);
    write(&path, &fill(body))?;
    written.push(path);
  }

  // **AC-11.3: `init` LEAVES `.intentfiles` PRESENT, CARRYING THE STANDARD
  // HEADER AND NO `STEELTHREAD:` LINE -- through the same function the
  // migration and `upgrade` reach.**
  //
  // A fresh project has no threads, so `default_declaration(&[])` is the header
  // alone. **The empty argument is the point rather than a degenerate case**:
  // it makes init's manifest the same function's OUTPUT as everybody else's, so
  // a change to the header moves all three callers at once. Three call sites
  // each deriving "the open set" for themselves is three chances to disagree
  // about what open means, and the one that drifts is the one nobody runs.
  //
  // **NOT A TEMPLATE, AND THE PROJECT RULE SAYS TEMPLATES.** `lib/templates/`
  // is the one home for GENERATED CONTENT. This file is durable STATE derived
  // from status, and a template carrying the header would be a second home for
  // the one thing AC-11.3 requires to have exactly one -- so honouring the
  // template rule here would break the criterion it looks like it serves.
  //
  // **PRESENT-AND-DECLARING-NOTHING IS A REAL STATEMENT, NOT AN EMPTY FILE.**
  // Absent means nobody has said, so everything is realised; this says nothing
  // is. On a project with no threads those agree, and they are still not the
  // same claim -- `st new` adds the entry as each thread is created, so the
  // manifest tracks the estate from the first commit and its history reads as a
  // diff rather than as a sudden appearance the day somebody runs `--default`.
  let manifest = root.join("intent/.intentfiles");
  write(
    &manifest,
    &crate::intentfiles::default_declaration(&[], &[]),
  )?;
  written.push(manifest);

  // **THE SECOND OF AC-07.6'S TWO DOORS.** The migration converges this for a
  // v2 estate coming across; without it here, a project BORN on v3 is the one
  // shape that never gets it. The two doors call one function on purpose --
  // and `.gitignore` had exactly this asymmetry (converged on migrate, absent
  // on init) until issue 0323 put it through the same door below.
  //
  // Runs after every write above, so the roster it converges describes a
  // project that fully exists. A `Project` that will not open one line after
  // `init` wrote its config is a real failure and is surfaced, never skipped.
  let project = crate::project::Project::open(root)
    .map_err(|cause| InitError::Io(root.to_path_buf(), std::io::Error::other(cause)))?;

  // **THE GENERATED ROOT FILES, AND THEY CANNOT BE WRITTEN WITH THE REST.**
  // Every `At` above is a template copy that needs nothing but the four tokens;
  // these are rendered from PROJECT STATE, so they cannot be written until the
  // config exists and `Project::open` has read it back. That is why this sits
  // below the loop rather than inside it, and why the order in this function is
  // load-bearing in one more place than its opening comment said.
  //
  // **THROUGH `rootfiles::substitute`, THE PURE EXPANDER, ON THE EMBEDDED
  // BODY.** Not `rootfiles::generate`, which resolves the install root: AC-07.1
  // requires a fresh `init` to work offline from the binary alone, and reading
  // a template off disk here would make the 0022 broken-install class
  // constructible again. `embedded_init` is the test that says so and it caught
  // this on the first attempt.
  //
  // **THE FULL EXPANDER, NOT `fill`.** The four-token `fill` above resolves no
  // `[[#lang ...]]` block, so seeding this file would write markup where the
  // project's languages belong -- which is the whole reason the disposition is
  // `Generated` and not `At`.
  let ctx = crate::views::RenderContext {
    version: intent_version,
    // A project one line old has never flushed, so there is no cutoff to
    // carry: this is what the facade's own render context reads from a fresh
    // store, so the aggregate views below match what `sync --to-disk` writes.
    todo_watermark: None,
  };
  for (dest, body) in &generated {
    let content = crate::rootfiles::substitute(body, project.config(), &ctx)
      .map_err(|fault| InitError::Render(dest, fault))?;
    let path = root.join(dest);
    write(&path, &content)?;
    written.push(path);
  }

  // **THE TWO AGGREGATE VIEWS, SO THE FIRST COMMIT IS NOT REFUSED** (issue
  // 0448). `doctor` counts an absent `steel_threads.md` or `todo.md` as skew
  // even on an estate with no thread, so a project `init` left without them
  // was refused at its first commit by the gate `claude upgrade --apply`
  // installs, with `sync --to-disk` as a remedy the new user had to find.
  // Rendered by `views::aggregate_views`, the one home `render_all` also uses,
  // so what `init` writes and what a sync would write are the same bytes.
  for view in crate::views::aggregate_views(&project, &[], &ctx) {
    write(&view.path, &view.content)?;
    written.push(view.path);
  }

  // **THE `init` EVENT'S COMMITTED FILE** (ST0078 P1). Every event travels as
  // its own file under `.canon/events/`, and `init` is the first one a project
  // has, so a clone knows who created it. Written from what the store actually
  // recorded, stamp included.
  for event in store
    .take_landed_events()
    .into_iter()
    .filter(|e| crate::event::travels(&e.op))
  {
    let path = project
      .event_file(&event)
      .map_err(|e| InitError::Io(project.events_dir(), std::io::Error::other(e)))?;
    let body = crate::event::to_file(&event)
      .map_err(|e| InitError::Io(path.clone(), std::io::Error::other(e)))?;
    write(&path, &body)?;
    written.push(path);
  }

  crate::facade::converge_formatter_exclusion(&project)
    .map_err(|cause| InitError::Io(root.join(".prettierignore"), cause))?;
  written.push(root.join(".prettierignore"));

  // **THE STORE STAYS OUT OF GIT FROM THE FIRST COMMIT** (issue 0323). A fresh
  // project had no rule for `intent/.cache/`, so `git add .` staged
  // `intent.db`, which D34 says never enters history. The one converger the
  // migration calls, so both doors write the same rules in the same words.
  crate::facade::converge_gitignore(&project)
    .map_err(|cause| InitError::Io(root.join(".gitignore"), cause))?;
  written.push(root.join(".gitignore"));

  written.sort();

  Ok(Initialised {
    root: root.to_path_buf(),
    project_name: project_name.to_string(),
    config,
    written,
    skipped,
    languages: outcomes,
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
  /// tests can see it. **The floor is the table, never a number**: one embedded
  /// template per declared destination, so a template deleted with its row
  /// moves both sides at once (a fixed floor went red when issue 0331 deleted
  /// the dead templates), and the second assertion keeps the table non-empty.
  #[test]
  fn the_embed_is_not_empty() {
    assert_eq!(
      EMBEDDED_TEMPLATES.len(),
      DESTINATIONS.len(),
      "the walk embedded a different number of templates than DESTINATIONS declares"
    );
    assert!(
      DESTINATIONS.iter().any(|(_, d)| matches!(d, At(_))),
      "every template is NotByInit, so `init` writes no starter content at all"
    );
  }
}
