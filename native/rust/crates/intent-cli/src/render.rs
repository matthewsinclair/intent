//! Parse -> facade -> render. The whole of the CLI's logic is routing and
//! formatting; every decision belongs to intentsvcs.
//!
//! **The voice is v2's** (INV-01, issue 0023): lowercase `ok:` / `error:`
//! prefixes, no banners, results on stdout and failures on stderr. INV-06
//! records that about a fifth of v2's failure paths write to the wrong stream;
//! that is a defect being corrected, not a contract being reproduced.

use clap::ArgMatches;
use std::path::Path;

use crate::dispatch;
use crate::spine::Failure;
use intentsvcs::address;
use intentsvcs::contract::Scope;
use intentsvcs::facade::{
  EventFilter, Exported, Facade, FacadeContext, FacadeError, ListEdit, Note, Outcome,
};
use intentsvcs::model::{
  self, AcKind, AtKind, AtStatus, IssueStatus, TShirt, ThreadStatus, enum_str,
};
use intentsvcs::output::{Format, Output};
use intentsvcs::project::Project;
use intentsvcs::remedy::Remedy;
use intentsvcs::views;

/// Everything a rendered failure says. The facade's own rendering already
/// carries the message, the full cause chain and the remedy (AC-04.4), so this
/// adds nothing and hides nothing.
fn fail(e: FacadeError) -> Failure {
  Failure::Error(e.render())
}

/// Dispatch one parsed invocation.
pub fn run(matches: &ArgMatches) -> Result<(), Failure> {
  match matches.subcommand() {
    Some(("st", m)) => st(m),
    Some(("wp", m)) => wp(m),
    Some(("ac", m)) => ac(m),
    Some(("at", m)) => at(m),
    Some(("search", m)) => search(m),
    Some(("schema", m)) => schema(m),
    Some(("doctor", m)) => doctor(m),
    Some(("organize", m)) => organize(m),
    Some(("upgrade", _)) => upgrade(),
    Some(("bootstrap", m)) => bootstrap(m),
    Some(("init", m)) => init(m),
    Some(("ingest", m)) => ingest(m),
    Some(("export", m)) => export(m),
    Some(("todo", m)) => todo(m),
    Some(("sync", m)) => sync(m),
    Some(("backup", m)) => backup(m),
    Some(("info", _)) => info(),
    Some(("version", _)) => version(),
    Some(("plugin", m)) => plugin(m),
    Some(("lang", m)) => lang(m),
    Some(("modules", m)) => modules(m),
    Some(("claude", m)) => claude(m),
    Some(("llm", m)) => llm(m),
    Some(("issues", m)) => issues(m),
    Some(("agents", m)) => agents(m),
    Some(("critic", m)) => critic(m),
    Some(("edit", m)) => edited(m),
    Some(("events", m)) => events(m),
    Some((family, _)) => unwired(family, ""),
    None => {
      println!(
        "intent {} -- run `intent --help`",
        env!("CARGO_PKG_VERSION")
      );
      Ok(())
    }
  }
}

/// Open the project the caller is standing in.
///
/// INV-03, the project-context gate: a command that needs a project says so
/// when there is not one, rather than half-working. The marker is the config
/// file's presence, never an environment variable (issue 0025).
fn open() -> Result<Facade, Failure> {
  let (project, ctx) = context()?;
  Facade::open(project, ctx).map_err(fail)
}

/// Locate the project and assemble the ambient context, WITHOUT loading canon
/// into the store.
///
/// Split out from [`open`] because `doctor` needs exactly this much and no
/// more: it has to run on a project that cannot be opened, since that is when
/// someone reaches for it. Every other verb goes on to [`Facade::open`].
fn context() -> Result<(Project, FacadeContext), Failure> {
  let cwd = std::env::current_dir()
    .map_err(|e| format!("error: cannot read the working directory: {e}"))?;
  let project = Project::discover(&cwd).map_err(|e| {
    format!("error: {e}\n  remedy: run `intent init` here, or change to a directory inside an Intent project")
  })?;
  let ctx = FacadeContext {
    principal: "local".to_string(),
    project_id: project.config().project_id.clone().unwrap_or_default(),
    version: env!("CARGO_PKG_VERSION").to_string(),
  };
  Ok((project, ctx))
}

// **The renderer's clock is GONE, and its absence is the point** (hv,
// 2026-08-15: time comes from the DB). `fn today()` read the process clock and
// handed the result to the facade as `FacadeContext.today`, which made the
// date a value a caller supplied -- so the CLI, the daemon and any test could
// each supply a different one and all look correct. `Store::now` / `today` are
// the one clock now, and there is no seam here to disagree through.

/// The columns `intent st list` and the generated index share.
const ST_COLUMNS: &[&str] = &["ID", "Slug", "Status", "Created", "Completed"];

const WP_COLUMNS: &[&str] = &["WP", "Title", "Scope", "Status"];

/// v2's `intent issues list` columns (`bin/intent_issues:240`).
const ISSUE_COLUMNS: &[&str] = &["ID", "Status", "Sev", "Title"];

/// How wide to render, in v2's order of preference
/// (`bin/intent_helpers:get_terminal_width`).
///
/// `COLUMNS` first because that is what the BATS estate sets to make width
/// testable; a real terminal query second; a fixed 100 last, so output is
/// deterministic when there is no terminal at all -- which is every CI run and
/// every pipe.
fn terminal_width() -> usize {
  if let Ok(cols) = std::env::var("COLUMNS")
    && let Ok(n) = cols.trim().parse::<usize>()
    && n > 0
  {
    return n;
  }
  if let Some((w, _)) = terminal_size::terminal_size() {
    return w.0 as usize;
  }
  100
}

/// Who is raising this issue: `git config user.name`, or nobody.
///
/// **v2 reads four sources and this reads ONE, and the three that are missing
/// were refused by a ratified guard rather than dropped.** v2's chain
/// (`bin/intent_issues:169`) is `INTENT_AUTHOR`, `AUTHOR`, `git config
/// user.name`, `$USER` -- and AC-11.3 permits the shipped surface exactly one
/// environment variable, `COLUMNS`. Adding three needs an hv ruling and a row
/// in `ALLOWED`, which is a question in front of hv rather than something to
/// take while they are away. **The guard caught this on the first run and its
/// own message says why it has to: every machine here has those variables set,
/// so a quiet addition would have failed nothing.**
///
/// So the divergence is stated rather than silent: a user who sets
/// `INTENT_AUTHOR` is ignored until that ruling lands. The common case is
/// unaffected -- any machine that has ever made a commit has `user.name` -- and
/// `git config` is a subprocess rather than an environment read, so the leg that
/// does the real work is the one that survives the constraint.
///
/// **`None` rather than a placeholder when there is no identity**, which is
/// exactly AC-11.3's own scenario: a brew-installed binary on a machine with no
/// clone and no git config. Inventing `unknown` would sign an issue nobody
/// signed.
fn reporter() -> Option<String> {
  let out = std::process::Command::new("git")
    .args(["config", "user.name"])
    .output()
    .ok()?;
  out
    .status
    .success()
    .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
    .filter(|v| !v.is_empty())
}

/// v2's status synonyms, normalised (`bin/intent_helpers:canonical_status`).
///
/// Case-insensitive, and both spellings of every state are accepted because
/// v2 accepts both -- `wip` and `in progress` are the same filter, and an
/// operator who learned one of them must not get an empty table for using it.
fn status_filter(spec: &str) -> Result<Option<Vec<ThreadStatus>>, String> {
  use ThreadStatus as S;
  if spec.eq_ignore_ascii_case("all") {
    return Ok(None);
  }
  let mut wanted = Vec::new();
  for raw in spec.split(',') {
    let name = raw.trim().to_ascii_lowercase();
    let status = match name.as_str() {
      "wip" | "in progress" => S::Wip,
      "tbc" | "not started" => S::NotStarted,
      "completed" | "done" => S::Completed,
      "cancelled" | "canceled" => S::Cancelled,
      "hold" | "on hold" => S::Hold,
      "" => continue,
      other => {
        return Err(format!(
          "error: `{other}` is not a steel thread status\n  remedy: use one of wip, tbc, completed, cancelled, hold -- or `all`"
        ));
      }
    };
    wanted.push(status);
  }
  Ok(Some(wanted))
}

/// Render the steel-thread table.
///
/// Shared by `st list` and `sync` because v2 shares them
/// (`bin/intent_st:717-1043` composes one from the other) and
/// `tests/unit/output_width.bats` asserts they are byte-identical over the
/// same scope. The default scope differs on purpose, though: bare `st list`
/// shows WIP only, while the index covers everything -- issue 0019, because
/// `steel_threads.md` says it indexes ALL threads and was being built from the
/// WIP-only view, so it decayed to empty at every release close.
fn st_table(f: &Facade, a: &ArgMatches) -> Result<String, Failure> {
  let wanted = match opt(a, "status") {
    Some(spec) => status_filter(&spec)?,
    // v2's default: WIP only. NOT the same as `--status all`.
    None => Some(vec![ThreadStatus::Wip]),
  };
  st_rows(f, a, wanted)
}

/// The index scope: every thread, whatever `--status` would have said.
/// `st sync` has no status filter in v2 -- the index is the whole estate.
fn st_table_all(f: &Facade, a: &ArgMatches) -> Result<String, Failure> {
  st_rows(f, a, None)
}

fn st_rows(
  f: &Facade,
  a: &ArgMatches,
  wanted: Option<Vec<ThreadStatus>>,
) -> Result<String, Failure> {
  let out = output_of(a)?;

  let rows: Vec<Vec<String>> = f
    .st_list()
    .into_iter()
    .filter(|t| wanted.as_ref().is_none_or(|w| w.contains(&t.status)))
    .map(|t| {
      vec![
        t.id.clone(),
        t.slug.clone().unwrap_or_default(),
        t.status.display().to_string(),
        t.created.clone(),
        t.completed.clone().unwrap_or_default(),
      ]
    })
    .collect();

  table_out(&out, ST_COLUMNS, &rows)
}

/// Reconcile the runtime store with the committed canon on disk.
///
/// The expensive, infrequent half of the daily-driver split: ordinary commands
/// answer from the store and never scan the tree, so this is what makes the
/// store agree with the files again after a `git pull` or a hand edit.
///
/// **Two spellings, ONE implementation, and both are load-bearing.** `intent
/// sync` is the name hv gave it; `intent st sync` is v2's own command
/// (`bin/intent_st:1145`), whose job -- regenerating the thread index from the
/// ST files -- is a strict subset of this reconciliation now that the index is
/// generated from the model. Dropping either would break something real, so
/// they share this function rather than sharing a copy of it.
///
/// It was `st sync` only until vc reached for `intent sync`, the spelling the
/// dispatch table advertises and hv actually named, and got "not yet wired".
/// The documented spelling being the broken one is the worst way round.
/// **The bare verb REFUSES (AC-03.9).**
///
/// `sync` has two directions and they differ in destructiveness: db -> disk
/// rewrites re-creatable files from the source of truth, and disk -> db
/// replaces the source of truth from the files. Under D01 as reversed the
/// second is a RESTORE, and it destroys any change that is in the store and
/// not yet projected.
///
/// A verb whose two directions differ in destructiveness must not have a
/// silent default, so this one has none. It used to default to the
/// destructive direction, which is how the defect existed.
///
/// **The selector is `--to-disk` / `--to-store`, and the flags name the
/// DESTINATION rather than the source** -- ic's row, and the reasoning is
/// theirs: the destination is the side that gets overwritten, so it is the side
/// the operator needs to be sure about.
///
/// This function used to take no `ArgMatches` at all -- `Some(("sync", _))`
/// discarded them -- so the two rows the table had been advertising for a day
/// could not be read even in principle, and the bare verb's remedy told the
/// operator the selector was not built while `sync --help` listed it. **That is
/// worse than the gap it was describing**: a surface that advertises a flag and
/// an implementation that denies it exists disagree in the one place a user
/// checks. It is also the mechanical shape of a whole class -- a discarded
/// `ArgMatches` drops every flag on the command silently, because clap accepts
/// what the table declares whether or not anything reads it.
/// The threads named on the command line, or the whole estate when none are.
///
/// **The table declares `id` at `0..n` and this is what reads it.** The comment
/// on `sync` below already records the class: a discarded `ArgMatches` drops
/// every argument the table declares, silently, because clap accepts the
/// declaration whether or not anything consumes it. Declaring the scope and
/// not reading it would have advertised a narrowing that never happened --
/// worse than not shipping it, because the operator would believe their peers'
/// files were safe.
///
/// **No ids means the whole estate**, so every invocation that worked
/// yesterday means what it meant.
fn sync_scope(m: &ArgMatches) -> Result<intentsvcs::sync::Scope, Failure> {
  let ids: Vec<String> = m
    .try_get_many::<String>("id")
    .ok()
    .flatten()
    .map(|vals| vals.cloned().collect())
    .unwrap_or_default();
  if ids.is_empty() {
    return Ok(intentsvcs::sync::Scope::All);
  }
  // **NORMALISED ONE BY ONE, AND A BAD ONE REFUSES THE WHOLE RUN.** `sync` is
  // the verb that writes canon or disk wholesale; syncing the three ids that
  // parsed and silently dropping the fourth would be a partial write reported
  // as a complete one.
  ids
    .iter()
    .map(|raw| thread_spec(raw))
    .collect::<Result<Vec<_>, _>>()
    .map(intentsvcs::sync::Scope::Threads)
}

fn sync(m: &ArgMatches) -> Result<(), Failure> {
  let scope = sync_scope(m)?;
  match (flag(m, "to-disk"), flag(m, "to-store")) {
    // Both is not "do both": they are opposite directions over the same two
    // endpoints, so running them in either order makes one of them pointless
    // and the other authoritative by accident of ordering.
    (true, true) => Err(
      "error: `--to-disk` and `--to-store` are opposite directions, so naming both chooses neither\n  remedy: run the one whose DESTINATION you mean -- `--to-disk` writes the extract from the store, `--to-store` replaces the store from the extract"
        .into(),
    ),
    (true, false) => {
      let mut f = open()?;
      let count = f.sync_to_disk(&scope).map_err(fail)?;
      println!("ok: extract written for {count} thread(s)");
      Ok(())
    }
    (false, true) => {
      let mut f = open()?;
      // **Stated BEFORE, never reported after.** The facade computes this
      // against the store rather than trusting a timestamp, and the whole
      // reason it is a separate call is that a summary afterwards is a receipt
      // for a loss the operator needed one moment earlier.
      //
      // It states and then proceeds rather than refusing: naming `--to-store`
      // IS the choice AC-03.9 asks the operator to make, and a second gate
      // would need a force flag the table does not declare. The limit is real
      // and is vc's to price -- in a non-interactive invocation "one moment
      // earlier" is one line earlier -- so it is recorded here rather than
      // quietly resolved by inventing surface.
      let overwrite = f.sync_overwrite(&scope).map_err(fail)?;
      if overwrite.is_empty() {
        eprintln!("note: the store and the extract agree; this restore overwrites nothing");
      } else {
        eprintln!("warning: replacing the store from the extract OVERWRITES:");
        for line in &overwrite {
          eprintln!("  {line}");
        }
      }
      // **ST0057 AC-03.5, printed BEFORE the write and not after it.** This
      // run reads the WORKTREE, so any attachment edited and not staged is
      // about to enter canon carrying bytes no commit contains -- and canon
      // recording those is indistinguishable on inspection from canon
      // recording correct ones. A report after the store was replaced would be
      // a receipt for something already done.
      //
      // **`None` is printed as not-knowing rather than as silence.** No
      // repository, or git did not run, and the difference between "nothing is
      // uncommitted" and "I could not ask" is the whole reason the facade
      // returns an Option here.
      match f.sync_uncommitted(&scope).map_err(fail)? {
        None => eprintln!(
          "note: the index could not be read, so whether any attachment carries uncommitted bytes is UNKNOWN"
        ),
        Some(found) if found.is_empty() => {}
        Some(found) => {
          eprintln!(
            "warning: {} attachment(s) carry bytes no commit contains, and this run takes them into canon:",
            found.len()
          );
          for line in &found {
            eprintln!("  {line}");
          }
          eprintln!(
            "  commit them first if canon should name bytes a reader can obtain -- this run does not refuse, and the commit gate will"
          );
        }
      }
      let count = f.sync_from_disk(&scope).map_err(fail)?;
      println!("ok: store replaced from the extract, {count} thread(s)");
      Ok(())
    }
    (false, false) => {
      let f = open()?;
      let overwrite = f.sync_overwrite(&scope).map_err(fail)?;
      eprintln!("error: `sync` has two directions and will not guess which one you mean");
      // **`Safe` WAS AN UNCONDITIONAL CLAIM AND THE CONDITION IS REAL.** The
      // files are re-derivable FROM THE STORE, so the word is only true while
      // the store is faithful -- and the case that breaks it is not
      // hypothetical. A porter defect (`e935734d`) put truncated AT citations
      // into the store while the authored `acceptance.md` still carried the
      // full line, which makes the FILES the only intact copy and this
      // direction the one that destroys them. `finding.rs:319` already reasons
      // exactly this way about a derivable artefact and says what it costs
      // anyway; this line asserted the opposite of what it costs.
      eprintln!(
        "  --to-disk   rewrites the files from the store. Routine WHILE THE STORE IS FAITHFUL: the files are re-derived from it, so anything the store did not capture is gone"
      );
      eprintln!(
        "  --to-store  replaces the store from the files. DESTRUCTIVE: any change not yet written to disk is lost"
      );
      if overwrite.is_empty() {
        eprintln!("  (nothing would be overwritten by `--to-store` right now)");
      } else {
        eprintln!("  `--to-store` would currently overwrite:");
        for line in &overwrite {
          eprintln!("    {line}");
        }
      }
      // **The remedy names the SAFE direction only.** AC-03.9 is explicit that
      // a remedy sending an operator to the destructive direction to recover is
      // itself the defect, and the list above is the only place `--to-store`
      // is named -- as a cost, not as a suggestion.
      //
      // **AND IT NOW CARRIES THE COPY-ASIDE STEP, WHICH IS THE HOUSE FORM AND
      // WAS MISSING HERE ALONE.** `finding.rs:319` tells the operator the
      // regeneration DISCARDS the hand edit and to copy anything they meant to
      // keep out first; `:345` makes copying the file outside the project the
      // FIRST instruction, on the grounds that it costs nothing and removes
      // the irreversibility before any question of which version was meant.
      // This site named neither, so the one remedy an operator reaches by
      // typing a command wrong was the one that did not say what it costs.
      eprintln!(
        "  remedy: `intent sync --to-disk` is the routine direction. If this store came from a port or a migration, copy the files you care about outside the project FIRST -- that step costs nothing and is the only one that cannot lose anything"
      );
      Err(Failure::Verdict)
    }
  }
}

/// A verb the dispatch table carries and the facade does not yet implement.
///
/// It must NOT say "a command is required" -- one WAS given, and reporting a
/// missing command for a present-but-unwired one is the same
/// same-text-for-different-causes collapse AC-04.4 forbids. The operator needs
/// to know the difference between "you typed nothing" and "we have not built
/// that yet", because only one of them is their problem.
/// **It used to name the work package that owes the verb** -- read from the
/// dispatch table rather than hardcoded, which was the right fix for the
/// problem it was solving and the wrong thing to be printing at all. D37: our
/// own thread and work-package numbers are not output. A user reading
/// "(ST0056 WP-08)" learns nothing they can act on; they learn that this tool
/// leaks its authors' backlog.
///
/// The distinction the message exists to draw is preserved in full -- "you
/// typed nothing" versus "we have not built that yet" -- because only one of
/// those is the operator's problem. What is dropped is the internal citation,
/// not the meaning.
/// **A LEAF GETS A DIFFERENT REMEDY, because the generic one promises a
/// category that is empty** (ic, measured 2026-08-15). At that measurement
/// seventeen commands were unimplemented and **nine of them were leaves with
/// zero verbs** -- `info`, `init`, `bootstrap`, `learn`, `fileindex`,
/// `version`, `export`, `ingest`, `mcp`. On every one, "run `intent <x> --help`
/// for the verbs that are" sends the reader to a help block that lists no verbs
/// at all, so the remedy costs them a command and returns nothing.
///
/// **`ingest` and `export` have since been wired and are no longer in that
/// set** (cc, `c8d90298` and the export commit). The figure is left as ic
/// measured it, dated, rather than decremented: a count restated on every
/// change is a count nobody can check, and the shape it demonstrates is what
/// this comment is for.
///
/// A remedy that cannot be acted on is worse than no remedy: it reads as a lead
/// and spends the reader's next move. The leaf form points at the root surface,
/// which is never empty, and says plainly that nothing here provides it.
///
/// The family/leaf question is asked of the TABLE rather than of a list kept
/// here, so a family that gains or loses its verbs moves between the two forms
/// on its own -- ic's nine is a measurement of today, not a roster to maintain.
fn unwired(family: &str, verb: &str) -> Result<(), Failure> {
  let path = if verb.is_empty() {
    family.to_string()
  } else {
    format!("{family} {verb}")
  };
  let has_verbs = dispatch::table()
    .families
    .iter()
    .filter(|f| f.name == family)
    .flat_map(|f| f.entries.iter())
    .any(|e| e.verb().is_some() && e.is_shipped());
  let remedy = if has_verbs {
    format!("run `intent {family} --help` for the verbs that are")
  } else {
    "nothing in this build provides it -- `intent --help` lists what does".to_string()
  };
  // **`Unavailable`, not `Error` -- this is issue 0038 and it is the exit code
  // that matters, not the wording.** The message was already correct and said
  // plainly that the command is unbuilt; it exited 1, which every consumer
  // written against v2 reads as "the tool ran and returned a negative verdict
  // about your work". The shipped pre-commit gate reads exactly that, so a
  // project migrating to v3 while any hook-invoked command was still unwired
  // could not commit at all -- and the remedy it printed named findings that
  // did not exist, leaving `--no-verify` as the only way through.
  //
  // The gate's fail-open branch for `2+` was correct all along and simply
  // never reached. Nothing in the hook changes; the number does.
  Err(Failure::Unavailable(format!(
    "error: `{path}` is a known command that is not implemented yet\n  remedy: {remedy}"
  )))
}

/// `st hydrate` and `issues hydrate`: ONE implementation behind two doors.
///
/// **Two family verbs, not two behaviours.** `Facade::hydrate` dispatches on
/// the address's ENTITY, so a thread and an issue reach the same code by
/// construction -- and a second copy here would be the Highlander defect in the
/// one place the estate can least afford it, since the two would agree on the
/// day they were written and drift the first time a view kind lands.
///
/// **THE WHOLE ARGUMENT GOES TO `promote`, NEVER AN ID LIFTED OUT OF IT.** The
/// verb takes an ADDRESS because the SERVICE refuses in address terms: two of
/// `Facade::hydrate`'s three refusal arms -- a foreign authority, and an entity
/// that is not an artefact -- are unreachable from a bare id. Extracting the id
/// and rebuilding `intent:///threads/<id>` is the spelling that reads fine and
/// silently converts a cross-project reference into a local one, and the
/// authority refusal never fires because the authority is gone before it is
/// called.
///
/// A malformed argument is a USAGE error naming both accepted forms, never a
/// not-found: `AddressError::NotAddressable` exists for exactly that, so an
/// operator who typed `ST57` is not sent into the estate hunting a thread that
/// was never addressed.
/// `intent edit <address> [file]` and `intent st edit <id> [file]`, which are
/// one function because AC-05.3 says path-printing has one home.
///
/// **IT PRINTS THE PATH AND NOTHING ELSE ON STDOUT**, carried from v2
/// (`bin/intent_st:1101-1144`) because the invocation that matters is
/// `$EDITOR "$(intent edit ST0001 design)"` and anything else on that stream
/// ends up as a filename. **The name is a historical misnomer** -- it never
/// launches an editor -- and v2's own docs already work around it.
///
/// The realisation it does first is reported on STDERR for the same reason: it
/// is news the operator wants and a substitution does not.
fn edited(m: &ArgMatches) -> Result<(), Failure> {
  // **TWO DOORS, AND THEY WERE READ AS ONE.** `intent edit <address>` may name
  // either collection and has to go through the collection-agnostic door;
  // `intent st edit <id>` is thread-scoped and learns its collection from its
  // own verb. Folding them together gave `st edit 59` the agnostic door's
  // refusal for an argument that was never ambiguous -- and let `st edit 0042`
  // accept an ISSUE, which then failed three layers down in the realiser.
  let argument = match arg(m, "address") {
    Ok(a) => a,
    Err(_) => thread_arg(m, "id")?,
  };
  let file = opt(m, "file").unwrap_or_else(|| "info".to_string());

  // **THE DECLARED VOCABULARY IS ENFORCED HERE, FROM THE TABLE, AND THE LAYER
  // IS THE POINT.** `surface/dispatch-table.json` declares
  // `info | design | impl | tasks | acceptance` for this argument and the spine
  // reads `arg.default` without ever reading `arg.values` -- so until now the
  // set was declared and nothing honoured it, which is the
  // declaration-versus-implementation gap AC-04.6 exists to find.
  //
  // **NOT A CLAP `value_parser`, AND THAT IS NOT A STYLE CHOICE**: clap rejects
  // at exit 2, and 2 is INV-04's USAGE code that the pre-commit gate FAILS OPEN
  // on. A bad enum value is a refusal the operator must see, not a gate bypass.
  // Refusing here keeps it at 1 and lets the message name the set.
  //
  // Read from the table rather than written out, so the vocabulary has one home
  // and this cannot drift from the row it enforces.
  let permitted = dispatch::arg_values(&dispatch::table(), "edit", "file");
  if !permitted.is_empty() && !permitted.iter().any(|v| v == &file) {
    return Err(Failure::Error(format!(
      "error: `{file}` is not a file this verb can open\n  remedy: name one of {}",
      permitted.join(", ")
    )));
  }

  let address = address::promote(&argument).map_err(|e| Failure::Error(e.render()))?;
  let mut facade = open()?;
  let path = facade.edit(&address, &file).map_err(fail)?;
  println!("{}", path.display());
  Ok(())
}

fn hydrated(argument: &str) -> Result<(), Failure> {
  let address = address::promote(argument).map_err(|e| Failure::Error(e.render()))?;
  let mut facade = open()?;
  let paths = facade.hydrate(&address).map_err(fail)?;

  // **`exists`, NOT `wrote`, AND THE DISTINCTION IS THE FACADE'S OWN.**
  // `hydrate` documents its return as *paths that NOW EXIST, not paths this run
  // had a step for* -- it is idempotent in both of its steps, so the ordinary
  // second call writes nothing and returns the same set. Labelling these
  // `wrote:` would be a count of one thing standing for a count of another,
  // which is the class that let `1 refused` speak for 423 files.
  // **THE URL RATHER THAN THE ARGUMENT, SO THE PROMOTION IS VISIBLE.** An
  // operator who typed `ST0056` is told what it was promoted to, which is the
  // one place the bare-id shorthand can be seen doing its work; echoing their
  // own argument back would confirm only that it was received.
  let project = facade.project();
  println!(
    "ok: {} hydrated -- listed in {}, {} file(s) on disk",
    address.to_url(),
    project.relative(&project.intentfiles_path()),
    paths.len()
  );
  for path in &paths {
    println!("  exists: {}", project.relative(path));
  }
  Ok(())
}

/// The inverse of `hydrated`, and the report is where the two differ most.
///
/// **`removed:` RATHER THAN `hydrated`s `exists:`, AND THE ASYMMETRY IS HONEST.**
/// `hydrate` labels its lines `exists` because it is idempotent in both steps
/// and the ordinary second call writes nothing while returning the same set --
/// so `wrote` would be a count of one thing standing for a count of another.
/// Dehydration has no such gap: a file already absent was never in the plan, so
/// every path here names a removal this run actually performed.
fn dehydrated(argument: &str) -> Result<(), Failure> {
  let address = address::promote(argument).map_err(|e| Failure::Error(e.render()))?;
  let mut facade = open()?;
  let done = facade.dehydrate(&address).map_err(fail)?;

  let project = facade.project();
  let manifest = project.relative(&project.intentfiles_path());

  // **AC-00.6: `NOTHING TO DO` AND `DID SOMETHING` MUST NOT READ THE SAME**, and
  // the two axes are reported SEPARATELY because they move independently.
  // Three of their four combinations are real states: listed with files on
  // disk; listed with nothing realised; and -- the one worth being able to see
  // -- NOT listed while files are present, where the manifest already said
  // unrealised and the disk disagreed. A single blended sentence would make
  // that disagreement unreportable. This is the class that let `1 refused`
  // speak for 423 files.
  if !done.unlisted && done.removed.is_empty() && done.pruned.is_empty() {
    println!(
      "ok: {} was already dehydrated -- not listed in {manifest}, nothing realised to remove",
      address.to_url()
    );
    return Ok(());
  }

  println!("ok: {} dehydrated", address.to_url());
  if done.unlisted {
    println!("  delisted: {manifest}");
  } else {
    // Reached when the files were present and the id was never listed. Saying
    // nothing here would let the run read as an ordinary delisting.
    println!("  unchanged: {manifest} -- the id was not listed, and the files were present anyway");
  }
  for path in &done.removed {
    println!("  removed: {}", project.relative(path));
  }
  for path in &done.pruned {
    println!(
      "  pruned: {} (emptied by the removal)",
      project.relative(path)
    );
  }
  // **NEVER A BARE `dehydrated` WHILE A DIRECTORY TREE REMAINS** (vc,
  // 2026-08-26). `prune_emptied` skips a directory it cannot delete through an
  // `is_ok()`, which is the right floor and a silent report. Content outside
  // the corpus -- gitignored review aids, say -- legitimately survives and must
  // not refuse the run, but the manifest now says this thread is dehydrated and
  // the disk still has files under it. **git leaves ignored files behind too
  // and says nothing, because git keeps no manifest to contradict; we do.**
  for path in &done.left_in_place {
    println!("  left in place: {} (not empty)", project.relative(path));
  }
  Ok(())
}

fn st(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("new", a)) => {
      let title = arg(a, "title")?;
      let mut f = open()?;
      // **`--dehydrate` WAS RETIRED HERE (hv, 2026-08-27).** It suppressed the
      // list entry, which stopped meaning anything the moment hv's 16:30Z
      // ruling took `st.new` out of the list-editing set: `declared_list_edit`
      // answers `None` for `st.new` whichever `ListEdit` the verb hands it, so
      // the flag produced byte-identical outcomes with and without. Measured,
      // not reasoned -- and then retired rather than documented, because
      // Intent prunes and **a flag that does nothing is one somebody
      // eventually tries to use.**
      //
      // `st_new_listing` STAYS, and is not now a test-only door: `st_new`
      // delegates to it, so it is the one body. `--keep` on `st done` and
      // `st cancel` still reach `ListEdit` and still mean what they say.
      let id = f.st_new(&title).map_err(fail)?;
      // **`-s|--start` COMPOSES two declared transitions and never constructs
      // the end state** (vc, ruled 2026-08-15). The flag is v2 parity and it
      // never changed; the machine grew a state underneath it. v2's `st new`
      // landed at `not-started`, so `-s` was ONE step; v3 enters at `Triage`,
      // so it now spans `Triage -> NotStarted -> Wip`.
      //
      // Building the end state directly is the obvious implementation and
      // produces two defects at once: an audit trail showing a thread that was
      // never triaged, and an effective `Triage -> Wip` edge that is not in the
      // ratified machine -- which either forces AC-04.6 to accept an undeclared
      // edge or routes construction around `transitions.rs` entirely.
      //
      // The triage decision is not skipped by composing. A user typing
      // `--start` has decided the thread is real work, which IS the triage
      // decision, made explicitly by the same act.
      //
      // **v2 reproduced the OUTCOME, not the mechanism.** Its `-s` sed-edits
      // `status:` straight to WIP (`bin/intent_st:381`) -- the construct-the-
      // end-state shape, in the incumbent. Parity is owed on what the operator
      // observes, never on how the file got that way.
      //
      // A failure part-way leaves the thread at the state it reached and says
      // so, rather than being rolled back: each step is a real transition that
      // really happened, and the log is the record of what happened.
      if flag(a, "start") {
        f.st_triage(&id).map_err(fail)?;
        f.st_start(&id).map_err(fail)?;
      }
      // v2 prints nothing extra for `-s` (`bin/intent_st:377-381`), so neither
      // does this. The new status is one `st list` away and a second line here
      // would be a deviation owed to nobody.
      println!("created: {id}");
      Ok(())
    }
    Some(("start", a)) => {
      let id = thread_arg(a, "id")?;
      reported(&open()?.st_start(&id).map_err(fail)?, &id, "started");
      Ok(())
    }
    Some(("done", a)) => {
      let id = thread_arg(a, "id")?;
      // `--keep` closes the thread and LEAVES its `.intentfiles` entry, so its
      // files stay. It also suppresses the closing note, because the note is
      // about an impending dehydration that `--keep` has just cancelled.
      let list = if flag(a, "keep") {
        ListEdit::Suppressed
      } else {
        ListEdit::AsDeclared
      };
      reported(
        &open()?.st_done_listing(&id, list).map_err(fail)?,
        &id,
        "done",
      );
      Ok(())
    }
    Some(("cancel", a)) => {
      let id = thread_arg(a, "id")?;
      // The ratified machine guards `st cancel` with "reason recorded", so the
      // facade refuses without one. `--reason` is read through `opt` rather
      // than `arg` on purpose: the flag is a dispatch-table row and the table
      // is ic's lane, so an absent one must not crash the renderer. When it is
      // absent the facade's `ReasonRequired` says exactly what is missing,
      // instead of cancelling a thread with no record of why.
      let reason = opt(a, "reason").unwrap_or_default();
      // **`--keep` IS ON BOTH CLOSING VERBS SINCE hv's 2026-08-20 RULING.** It
      // was on `st done` alone because AC-05.2 named only that one; two
      // identical acts with the override on one of them is a surface that has
      // to be memorised rather than understood.
      let list = if flag(a, "keep") {
        ListEdit::Suppressed
      } else {
        ListEdit::AsDeclared
      };
      reported(
        &open()?
          .st_cancel_listing(&id, &reason, list)
          .map_err(fail)?,
        &id,
        "cancelled",
      );
      Ok(())
    }
    // The five lifecycle verbs below have NO v2 antecedent -- every one is a
    // `new-surface` row -- so there is no parity text to reproduce and the
    // wording is authored here. The arms are in dispatch-table row order, so
    // a verb the table carries and this match does not is visible by reading
    // down the two side by side rather than by trusting a count.
    //
    // **Where the verb does not name its own landing state, the message
    // does.** `triage` and `reinstate` are both readable in the wrong
    // direction -- "triage" sounds like it puts a thread INTO triage, and a
    // reinstated thread lands in the backlog rather than back at `wip` -- and
    // an operator who has to run `st show` to find out where a verb put their
    // thread has been told less than the verb knew.
    Some(("triage", a)) => {
      let id = thread_arg(a, "id")?;
      reported(
        &open()?.st_triage(&id).map_err(fail)?,
        &id,
        "accepted out of triage",
      );
      Ok(())
    }
    Some(("hold", a)) => {
      let id = thread_arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.st_hold(&id, &reason).map_err(fail)?,
        &id,
        "on hold",
      );
      Ok(())
    }
    Some(("resume", a)) => {
      let id = thread_arg(a, "id")?;
      reported(&open()?.st_resume(&id).map_err(fail)?, &id, "resumed");
      Ok(())
    }
    Some(("reopen", a)) => {
      let id = thread_arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.st_reopen(&id, &reason).map_err(fail)?,
        &id,
        "reopened",
      );
      Ok(())
    }
    Some(("reinstate", a)) => {
      let id = thread_arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.st_reinstate(&id, &reason).map_err(fail)?,
        &id,
        "reinstated to the backlog",
      );
      Ok(())
    }
    Some(("list", a)) => {
      let f = open()?;
      print!("{}", st_table(&f, a)?);
      Ok(())
    }
    // **THE LAST GAP AC-08.5 NAMES, AND THE VERB IS DELIBERATELY NARROW.**
    // `Attachment.text` and `blob` are writable -- `put` writes them -- and had
    // no route on the mutation surface, so the criterion's first clause failed on
    // two fields while the refusal correctly said *there is no CLI verb for this
    // today*. This is that verb.
    //
    // **THE SPELLING IS PROVISIONAL AND ROUTED TO hv** (vc authorised the
    // capability and declined the name, 2026-08-25). AC-08.5 asks whether the
    // field is settable through the mutation surface and has no opinion on what
    // the verb is called, so the row can close on capability while the name is
    // decided separately -- and a rename before anything ships costs nothing.
    // **vc refused to pick it under the pen because a name chosen inside a fix
    // becomes the ruling by default**, which is the failure this criterion
    // produced twice today.
    //
    // **FORM FOLLOWS CONTENT, AND THE DECISION IS NOT MADE HERE.**
    // `Facade::put_attachment` decides it by DECODING -- valid UTF-8 inline,
    // anything else as bytes -- which is `project.rs`'s rule (ST0057 AC-03.2)
    // reached rather than restated. **This verb refused non-UTF-8 for one build**
    // and the refusal was honest at the time: `put` takes a string, so there was
    // genuinely no route. **The route now exists and the refusal is gone rather
    // than reworded**, because a remedy that describes a limit the code no
    // longer has is the false-remedy class arriving through disuse.
    Some(("attach", a)) => {
      let id = thread_arg(a, "id")?;
      let path = arg(a, "path")?;
      let from = arg(a, "from")?;

      let bytes = std::fs::read(&from).map_err(|e| {
        Failure::Error(format!(
          "error: cannot read `{from}`: {e}\n  remedy: name a file that exists and is readable"
        ))
      })?;

      // **THE ADDRESS IS CONSTRUCTED, NOT SPELLED.** The first draft built
      // `format!("intent:///threads/{id}/attachments/{path}")` and
      // `address_resolution_single_home` refused it by name: *resolution has ONE
      // home and these spell the scheme themselves.* Building the `Entity` says
      // the same thing in the type system and cannot drift.
      let address = intentsvcs::address::Address {
        authority: None,
        entity: intentsvcs::address::Entity::Attachment {
          thread: id.clone(),
          path: path.clone(),
        },
        format: None,
      };
      let mut facade = open()?;
      facade.put_attachment(&address, &bytes).map_err(fail)?;
      println!("ok: {path} written to {id}");
      Ok(())
    }
    Some(("show", a)) => {
      let id = thread_arg(a, "id")?;
      let f = open()?;
      let t = f.st_show(&id).map_err(fail)?;
      println!("{}: {}", t.id, t.title);
      println!("status: {}", t.status.display());
      // **Directly under the status, because it is the status's reason.** Four
      // verbs REQUIRE one and refuse without it, and until now no human face
      // showed it -- the field reached `thread.json` and the GraphQL SDL and
      // nothing a person reads. It carries the CURRENT status's reason only;
      // any transition without a reason clears it.
      if let Some(reason) = &t.status_reason {
        println!("reason: {reason}");
      }
      println!("created: {}", t.created);
      if let Some(done) = &t.completed {
        println!("completed: {done}");
      }
      Ok(())
    }
    // `intent st sync` is v2's INDEX sync, and it is NOT the top-level
    // `intent sync`. I had wired it as an alias for the store reconciliation
    // and the dispatch table carries my note saying "both spellings run it".
    // That note was wrong, and `tests/unit/output_width.bats` is what proved
    // it: v2's `st sync` prints the thread table -- byte-identical to `st list
    // --status all` over the same scope -- and `--write` persists the index,
    // printing `updated: <path>`. Neither is "reconcile the store".
    //
    // The scope difference is deliberate and is issue 0019: bare `st list`
    // shows WIP only, while the index covers ALL threads, because
    // `steel_threads.md` says it indexes every thread and was being built from
    // the WIP-only view -- so it decayed to empty at every release close.
    Some(("sync", a)) => {
      if flag(a, "write") {
        let mut f = open()?;
        // v2's `st sync` regenerates `steel_threads.md` from the threads --
        // a projection, which under the reversed D01 is the db -> disk
        // direction. It maps onto the SAFE half, never the restore.
        let count = f
          .sync_to_disk(&intentsvcs::sync::Scope::All)
          .map_err(fail)?;
        // v2 prints the index path, and that is kept because it is the file
        // this verb is named for and what a script greps. The COUNT is added
        // because it would otherwise be the narrower-than-the-act message
        // class: the projection rewrites every view, not just the index, and a
        // line naming one file while writing many is how an operator learns to
        // trust a report that is not describing what happened.
        println!(
          "updated: {} (and the projection for {count} thread(s))",
          f.project().relative(&f.project().steel_threads_view())
        );
      } else {
        let f = open()?;
        print!("{}", st_table_all(&f, a)?);
      }
      Ok(())
    }
    Some(("hydrate", a)) => hydrated(&thread_arg(a, "id")?),
    Some(("dehydrate", a)) => dehydrated(&thread_arg(a, "id")?),
    // **AC-05.3: PATH-PRINTING HAS ONE HOME.** `st edit` is the same call with
    // an `st-id` argument instead of an address -- and since `address::promote`
    // takes a bare id, there is nothing left for this arm to do but pass it on.
    // A second implementation here is what the criterion exists to forbid.
    Some(("edit", a)) => edited(a),
    Some((verb, _)) => unwired("st", verb),
    None => Err("error: a steel thread command is required".into()),
  }
}

fn wp(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("new", a)) => {
      let st = thread_arg(a, "stid")?;
      let title = arg(a, "title")?;
      let mut f = open()?;
      // S, not M, and it is not a taste call. v2's `wp new` takes no scope
      // flag at all, so every work package it creates carries whatever
      // `lib/templates/prj/st/WP/info.md` seeds -- and that template says
      // `scope: Small`. A different default writes different canon for the
      // same command, which is a parity break hiding in a value rather than
      // in an output.
      //
      // **NO `--scope` FLAG, and hv ruled that permanently** (issue 0052, closed
      // `8e5ef648`): sizing happens after you have written the package, not while
      // you are naming it. So the value here stops being a verdict and becomes a
      // starting value, because `wp rescope` is now the exit -- same `S`, and the
      // only thing that changed is that it can be moved.
      //
      // **Not `absent`**: that would trade a wrong value for a missing one at the
      // moment the exit verb arrived to fill it, and it would break the template
      // parity above for nothing.
      let seq = f
        .wp_new(&st, &title, intentsvcs::model::DEFAULT_WP_SCOPE)
        .map_err(fail)?;
      println!("created: {st}/{seq:02}");
      Ok(())
    }
    Some(("start", a)) => {
      let (st, seq) = wp_target(a)?;
      reported(
        &open()?.wp_start(&st, seq).map_err(fail)?,
        &format!("{st}/{seq:02}"),
        "started",
      );
      Ok(())
    }
    Some(("done", a)) => {
      let (st, seq) = wp_target(a)?;
      reported(
        &open()?.wp_done(&st, seq).map_err(fail)?,
        &format!("{st}/{seq:02}"),
        "done",
      );
      Ok(())
    }
    // `wp reopen` is the inverse `wp done` never had, and its absence was
    // doing live damage rather than being a tidiness gap: `wp done` consults
    // the gate on the way in and nothing re-checks afterwards, so a work
    // package legitimately closed becomes a false green the moment an AC is
    // added to it. Until this arm existed the only repair was hand-editing the
    // file the CLI exists to own.
    Some(("reopen", a)) => {
      let (st, seq) = wp_target(a)?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.wp_reopen(&st, seq, &reason).map_err(fail)?,
        &format!("{st}/{seq:02}"),
        "reopened",
      );
      Ok(())
    }
    // **The state the model could not express until 2026-08-21.** `wp done`
    // consults the gate; a work package whose scope was removed has an emptied
    // contract, and the gate correctly refuses to infer an exemption from
    // emptiness (ST0048). With no `Cancelled` at WP level the only announced
    // exemption was thread-scoped, so closing one unit meant discarding the
    // standing of every AC in the thread. This verb is that announcement, as
    // data in a field the gate reads.
    Some(("cancel", a)) => {
      let (st, seq) = wp_target(a)?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.wp_cancel(&st, seq, &reason).map_err(fail)?,
        &format!("{st}/{seq:02}"),
        "cancelled",
      );
      Ok(())
    }
    Some(("reinstate", a)) => {
      let (st, seq) = wp_target(a)?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.wp_reinstate(&st, seq, &reason).map_err(fail)?,
        &format!("{st}/{seq:02}"),
        "reinstated",
      );
      Ok(())
    }
    Some(("unstart", a)) => {
      let (st, seq) = wp_target(a)?;
      reported(
        &open()?.wp_unstart(&st, seq).map_err(fail)?,
        &format!("{st}/{seq:02}"),
        "back to not started",
      );
      Ok(())
    }
    // **THE FIELD'S ONLY ENTRANCE AND ITS ONLY EXIT. hv RULED IT: no `wp new
    // --scope`, permanently** (issue 0052, closed `8e5ef648`). This comment said
    // "until hv rules", which was true when it was written and told the next
    // reader the question was open at the exact site they would come to answer it.
    //
    // `Facade::wp_rescope` was `pub`, implemented and reachable from nothing but
    // two tests, so `WorkPackage.scope` was a ratified machine with six edges the
    // surface could drive none of.
    //
    // **What the ruling buys is a change of meaning with no change of code**
    // (vc's framing): `wp new`'s hardcoded `S` was never the wrong VALUE -- see
    // its own arm -- but a default nobody can override is a verdict, and a default
    // with an exit is a starting value. Same `S`, different thing.
    //
    // **And this is where a migrated carry gets adjudicated.** `wp_rescope`
    // treats a same-size rescope as a no-op only when no `scope_legacy` is
    // carried, so rescoping a package whose v2 scope was outside the vocabulary
    // resolves the carry rather than reporting nothing done -- a human deciding,
    // which is exactly what the unwired verb was denying and the reason the
    // migration refuses to coerce a size in the first place.
    Some(("rescope", a)) => {
      let (st, seq) = wp_target(a)?;
      let size = t_shirt(&arg(a, "size")?)?;
      reported(
        &open()?.wp_rescope(&st, seq, size).map_err(fail)?,
        &format!("{st}/{seq:02}"),
        &format!("rescoped to {}", enum_str(&size)),
      );
      Ok(())
    }
    Some(("list", a)) => {
      let st = thread_arg(a, "stid")?;
      let f = open()?;
      let wps = f.wp_list(&st).map_err(fail)?;
      // v2's empty case is a SENTENCE, not an empty table -- unlike `st list`,
      // which prints its header. Both exit 0. Reproduced rather than
      // regularised: the row is target: as-observed, and a caller parsing this
      // output today is parsing that sentence.
      if wps.is_empty() {
        println!("no work packages for {st}");
        return Ok(());
      }
      let rows: Vec<Vec<String>> = wps
        .iter()
        .map(|w| {
          vec![
            format!("{:02}", w.seq),
            w.title.clone(),
            w.scope_display(),
            w.status.display().to_string(),
          ]
        })
        .collect();
      // The SAME renderer `st list` uses, which is what the v2 row asks for in
      // as many words: "so `wp list` and `st list` column layout cannot drift
      // apart".
      print!("{}", table_out(&output_of(a)?, WP_COLUMNS, &rows)?);
      Ok(())
    }
    Some(("show", a)) => {
      let (st, seq) = wp_target(a)?;
      let f = open()?;
      let wp = f.wp_show(&st, seq).map_err(fail)?;
      println!("{st}/WP-{:02}: {}", wp.seq, wp.title);
      // **`display()`, not `enum_str` -- this printed `wip` where every other
      // surface prints `WIP`, and the row makes it a parity break rather than a
      // preference.** `wp show` is `keep` / `as-observed`, v2 implements it by
      // catting `info.md` (`bin/intent_wp:263`), and `views.rs` writes that file's
      // status line with `display()`. So v2 printed `WIP` and the row's own note
      // says "the command reads the view, so its output is unchanged in kind".
      //
      // Found by issue 0050's witness, which reads a state back from the tool
      // rather than asserting a literal: `st show` and `issues show` both said
      // `WIP`-style and this one said `wip`, in the same tool, on the same field,
      // with no reason recorded anywhere. **Three `show` commands and two
      // vocabularies is 0047's shape**, and a test that pinned the kebab was
      // pinning the divergence.
      println!("status: {}", wp.status.display());
      // Same rule as `st show`: `wp reopen` is the WP transition the machine
      // guards with a required reason, and nothing rendered it.
      if let Some(reason) = &wp.status_reason {
        println!("reason: {reason}");
      }
      println!("scope: {}", wp.scope_display());
      Ok(())
    }
    Some((verb, _)) => unwired("wp", verb),
    None => Err("error: a work package command is required".into()),
  }
}

fn ac(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("gate", a)) => {
      let target = thread_arg(a, "stid")?;
      let (st, scope) = scope_of(&target);
      let f = open()?;
      let verdict = f.gate(&st, scope).map_err(fail)?;
      println!("{}", verdict.line(&target));
      if verdict.is_pass() {
        Ok(())
      } else {
        // The gate's own line IS the message; it went to stdout because the
        // gate is read by machines via the exit code (v2 does the same).
        Err(Failure::Verdict)
      }
    }
    // **CREATE, AC-08.6.** The nine arms beside this one are all TRANSITIONS on
    // a row that already exists; until this landed, the only route to a new
    // criterion was hand-editing `.canon/st/<ID>.json` and running
    // `sync --to-store` -- which is how AC-08.6 itself reached canon.
    //
    // Parse, call, render: the state a new criterion starts in is DERIVED FROM
    // ITS KIND and that derivation lives in the facade, not here, because it is
    // a fact about the model rather than about this door.
    Some(("new", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "acid")?;
      let text = arg(a, "text")?;
      let kind = match opt(a, "kind").as_deref() {
        Some("test") => AcKind::Test,
        // The table declares the default, so an absent flag and an explicit
        // `non-test` are the same answer here rather than two paths.
        None | Some("non-test") => AcKind::NonTest,
        Some(other) => {
          return Err(Failure::Error(format!(
            "`{other}` is not a criterion kind -- expected `test` or `non-test`"
          )));
        }
      };
      reported(
        &open()?.ac_new(&st, &id, &text, kind).map_err(fail)?,
        &id,
        "created",
      );
      Ok(())
    }
    Some(("satisfy", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "acid")?;
      // Read through `opt` and passed through, like the five thread and work
      // package verbs that owe a reason: the facade's `EvidenceRecorded` guard
      // is what refuses it, and it refuses `--evidence ""` as well as an absent
      // flag -- which re-checking the flag here could not do.
      let evidence = opt(a, "evidence").unwrap_or_default();
      // **`by evidence`, restored -- issue 0056, and it is the one of the five
      // that cannot be argued as tidying.** v2 prints `ok: <AC> satisfied by
      // evidence`, and the phrase is MORE load-bearing in v3 than it was in v2:
      // `AcState::Satisfied` carries evidence that cannot be empty, while a
      // test-backed criterion is `Computed` with nothing stored. So v2's wording
      // names a distinction this model made structural, and dropping it made the
      // line say less about v3 than it said about v2.
      //
      // The no-op stays `already satisfied` -- `reported` composes it from the
      // state, so the phrase is structurally unable to leak into a line where no
      // evidence was recorded.
      reported(
        &open()?.ac_satisfy(&st, &id, &evidence).map_err(fail)?,
        &id,
        "satisfied by evidence",
      );
      Ok(())
    }
    Some(("unsatisfy", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "acid")?;
      // The evidence goes with the satisfaction, so the line says so -- a
      // reader who is told only "unsatisfied" has to go and look to find out
      // whether the citation survived (AC-04.6, D32).
      //
      // **The parenthetical belongs to the MOVEMENT and must not survive into
      // the no-op**, which is why this arm names its phrase rather than sharing
      // one: nothing was cleared if nothing was satisfied, and `already
      // unsatisfied (evidence cleared)` would report a clearing that did not
      // happen. `reported` takes the movement phrase and composes the no-op from
      // the state, so this falls out rather than needing care.
      reported(
        &open()?.ac_unsatisfy(&st, &id).map_err(fail)?,
        &id,
        "unsatisfied (evidence cleared)",
      );
      Ok(())
    }
    Some(("list", a)) => {
      let st = thread_arg(a, "stid")?;
      let f = open()?;
      // v2's shape verbatim (`bin/intent_acceptance:909`), including the
      // absent space after `covered-by:` -- the ids arrive space-prefixed, so
      // an uncovered criterion renders `covered-by:` with nothing after it.
      for row in f.ac_list(&st).map_err(fail)? {
        let covering = row
          .covered_by
          .iter()
          .map(|id| format!(" {id}"))
          .collect::<String>();
        println!("ac: {}  covered-by:{covering}  {}", row.id, row.state);
      }
      Ok(())
    }
    Some(("status", a)) => {
      let target = thread_arg(a, "stid")?;
      let (st, scope) = scope_of(&target);
      let f = open()?;
      let verdict = f.gate(&st, scope).map_err(fail)?;
      // v2 reports N/M plus the verdict, and exits 0 either way: `status` is a
      // read. Only `gate` carries the verdict in its exit code.
      //
      // **`status_line`, not `line` -- the exit code was always right and the
      // PREFIX was wrong, and that combination is the harm.** This printed the
      // gate's own line (`gate: ST0056 BLOCKED -- ...`) beside exit 0, so a
      // consumer reading either channel alone gets a different answer than one
      // reading the other; the pre-commit gate is such a consumer. It also
      // dumped the full unsatisfied enumeration, which is `ac list`'s job --
      // `status` is the count.
      println!("{}", verdict.status_line());
      Ok(())
    }
    Some(("descope", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "acid")?;
      // **`to` passes through like the rest now that the facade can tell an
      // absent target from a missing one.** It stayed on `arg(..)?` for one
      // commit because the facade's only answer to a blank target was
      // "cannot descope AC-03.2 to , which is not a steel thread in this
      // project" -- a message with the same hole in it twice. That was a reason
      // to fix the refusal, not to keep re-checking the flag here:
      // `DescopeTargetRequired` now says a thread was not named, and clap
      // refuses an absent `--to` from the declared `required` before either.
      let to = opt(a, "to").unwrap_or_default();
      let by = arg(a, "by").ok();
      let reason = arg(a, "reason").ok();
      // **ONE OF THE TWO ARMS ISSUE 0050's ENUMERATION MISSED, and the reason is
      // its shape.** 0050 counted nineteen dropped sites by scanning for
      // `open()?.<verb>(..)` on one line; this arm and `withdraw` below break the
      // call across lines, so a line-oriented scan cannot see them. **The real
      // count was twenty-one.** Found by driving every self-loop-capable verb
      // twice through the real binary -- `ac descope` printed `ok: AC-01.1
      // descoped to ST0001` on both calls, which is the defect the issue is about,
      // in an arm the issue does not list.
      reported(
        &open()?
          .ac_descope(&st, &id, &to, by.as_deref(), reason.as_deref())
          .map_err(fail)?,
        &id,
        &format!("descoped to {to}"),
      );
      Ok(())
    }
    Some(("withdraw", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "acid")?;
      // Same passthrough, and this arm is why the rule is worth stating twice.
      // It USED to re-check requiredness here with `arg(..)?`, which looked
      // like the careful choice and was the reason the identical hole in
      // `satisfy` read as a simple oversight rather than as a missing guard:
      // two arms hand-implementing a rule that belonged in one place, and
      // nothing able to say which of them was right.
      let reason = opt(a, "reason").unwrap_or_default();
      let by = arg(a, "by").ok();
      // The second arm 0050's line-oriented count could not see -- see `descope`
      // above.
      reported(
        &open()?
          .ac_withdraw(&st, &id, &reason, by.as_deref())
          .map_err(fail)?,
        &id,
        "withdrawn",
      );
      Ok(())
    }
    // **BOTH UNDO VERBS PRINT THE SAME STRING, AND THE STATE IN IT IS COMPUTED**
    // -- ic's ratification, issue 0056. `ok: <AC> back in scope (<landing
    // state>)`, where the landing state is `AcState::entry(kind)`: `unsatisfied`
    // for an authored criterion, `computed` for a test-backed one.
    //
    // **`corrected` rather than `as-observed`, and the deviation is forced by the
    // model rather than chosen.** v2 prints one string across both verbs and both
    // kinds -- `back in scope (unsatisfied)` -- because v2 had no `computed` to
    // name. Restoring that literal would be WRONG for a test-backed criterion,
    // which is the only reason this row is not held to v2's bytes.
    //
    // The two verbs differ in which off-scope state they UNDO, not in where they
    // land, so `reinstated` was naming the verb the caller had just typed and
    // dropping the part they did not already know.
    //
    // **The state is READ BACK from the facade, never spelled here.** It cannot be
    // a literal -- it depends on the criterion's kind -- and reading it back means
    // the movement line and the no-op line take their state from the same place,
    // so they are structurally unable to disagree.
    Some(("rescope", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "acid")?;
      let mut f = open()?;
      let outcome = f.ac_rescope(&st, &id).map_err(fail)?;
      reported(&outcome, &id, &back_in_scope(&f, &st, &id)?);
      Ok(())
    }
    Some(("reinstate", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "acid")?;
      let mut f = open()?;
      let outcome = f.ac_reinstate(&st, &id).map_err(fail)?;
      reported(&outcome, &id, &back_in_scope(&f, &st, &id)?);
      Ok(())
    }
    Some((verb, _)) => unwired("ac", verb),
    None => Err("error: an acceptance criterion command is required".into()),
  }
}

/// The movement phrase the two undo verbs share: `back in scope (<state>)`.
///
/// Read from the criterion AFTER the verb, because the landing state is
/// `AcState::entry(kind)` and no literal here could name it -- `unsatisfied` for
/// an authored criterion, `computed` for a test-backed one. **One helper for both
/// verbs, because the ratification is that they print the same string**; two
/// call sites composing it separately is how they would come to differ.
fn back_in_scope(f: &Facade, st: &str, ac: &str) -> Result<String, Failure> {
  let thread = f.st_show(st).map_err(fail)?;
  let state = thread
    .criteria
    .iter()
    .find(|c| c.id == ac)
    .ok_or_else(|| Failure::Error(format!("error: no acceptance criterion {ac} in {st}")))?
    .state
    .name();
  Ok(format!("back in scope ({state})"))
}

fn at(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    // **CREATE, AC-08.7.** Like `ac new`, the five arms beside this one are all
    // transitions. Unlike `ac new`, the created row has a VALIDITY question:
    // it carries `file`, `covers` and `status`, so the facade holds it to the
    // same L2-L5 grammar `at lint` enforces on every other row, before the
    // write rather than after it.
    Some(("new", a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "atid")?;
      let covers: Vec<String> = a
        .get_many::<String>("covers")
        .map(|v| v.cloned().collect())
        .unwrap_or_default();
      let kind = match opt(a, "kind").as_deref() {
        None | Some("test") => AtKind::Test,
        Some("non-test") => AtKind::NonTest,
        Some(other) => {
          return Err(Failure::Error(format!(
            "`{other}` is not an acceptance-test kind -- expected `test` or `non-test`"
          )));
        }
      };
      let status = match opt(a, "status").as_deref() {
        None | Some("to-write") => AtStatus::ToWrite,
        Some("red") => AtStatus::Red,
        Some("green") => AtStatus::Green,
        // `n-a` is the wire spelling and `n/a` is what every authored row in
        // every estate says, so both are accepted -- refusing the spelling the
        // corpus uses would be a door nobody could find.
        Some("n-a") | Some("n/a") => AtStatus::Na,
        Some(other) => {
          return Err(Failure::Error(format!(
            "`{other}` is not an acceptance-test status -- expected `to-write`, `red`, `green` or `n/a`"
          )));
        }
      };
      reported(
        &open()?
          .at_new(
            &st,
            &id,
            kind,
            opt(a, "file"),
            opt(a, "prose"),
            covers,
            status,
            opt(a, "note"),
          )
          .map_err(fail)?,
        &id,
        "created",
      );
      Ok(())
    }
    Some(("list", a)) => {
      let st = thread_arg(a, "stid")?;
      let f = open()?;
      for t in f.at_list(&st).map_err(fail)? {
        // `display()`, not `enum_str` -- the wire form spells `Na` as `n-a` and
        // every authored row in every estate says `n/a`.
        println!(
          "{}  {}  covers {}",
          t.id,
          t.status.display(),
          t.covers.join(", ")
        );
      }
      Ok(())
    }
    // **THE MOVEMENT PHRASE COMES FROM THE STATUS, NOT FROM THE SUBCOMMAND NAME**
    // -- issue 0056, ic's ruling. This passed `state` as the movement phrase, ie
    // the verb the caller typed, so one command printed three spellings of one
    // value: `na` on a move (the subcommand), `n-a` on a self-loop (`enum_str`,
    // through `AlreadyThere`), against v2's `n/a`. **Echoing the verb is correct
    // for `green` and `red` because those tokens happen to match v2's, which is
    // the coincidence that hid it** -- the same two-of-three shape as the serde
    // vocabulary, one layer down and on the same family.
    //
    // The `-> ` arrow is v2's and was dropped across the whole AT family. It is
    // drift rather than a ruling, and this binary is the proof: `issues close`
    // and `issues open` keep it and reproduce v2 exactly, so the AT family was
    // the only place it went missing.
    Some((state @ ("green" | "red" | "na"), a)) => {
      let st = thread_arg(a, "stid")?;
      let id = arg(a, "atid")?;
      let status = match state {
        "green" => AtStatus::Green,
        "red" => AtStatus::Red,
        _ => AtStatus::Na,
      };
      reported(
        &open()?
          .at_set(&st, &id, status, opt(a, "note"))
          .map_err(fail)?,
        &id,
        &format!("-> {}", status.display()),
      );
      Ok(())
    }
    Some(("lint", a)) => {
      let st = thread_arg(a, "stid")?;
      if a.try_get_one::<bool>("fix").ok().flatten() == Some(&true) {
        // The 0017 `--fix` half-migrated rows: it rewrote what it could parse
        // and silently left the rest, which is worse than refusing, because a
        // lossy fixer damages what it touches and a lossy SUGGESTION damages
        // everything touched after it. v3 will not ship one that cannot finish
        // the job.
        return Err(
          "error: `at lint --fix` is not implemented in v3\n  remedy: fix the rows `intent at lint` names -- v2's --fix rewrote what it could parse and left the rest, which is why it is not being carried over".into(),
        );
      }
      let f = open()?;
      let report = f.at_lint(&st).map_err(fail)?;
      for finding in &report.findings {
        println!("{finding}");
      }
      // **THE VERDICT LINE, WITH ITS DENOMINATOR. Without it a clean lint was
      // zero bytes at exit 0** -- byte-identical to a lint that never ran, on
      // the surface a reader trusts the AT contract on. v2's positive control
      // (`bin/intent_acceptance:1278`) is the shape being restored, and the row
      // count is what makes it a control rather than a reassurance: `ok` alone
      // is equally true of a thread with no rows at all.
      //
      // `Failure::Verdict` was already the declared contract for this arm --
      // spine.rs names `at lint` as one of the four sites whose verdict is on
      // stdout and whose stderr is therefore silent. The failing path returned
      // it while printing no verdict; the enum's doc comment was describing an
      // intent the code did not carry out.
      let rows = report.rows;
      if report.findings.is_empty() {
        println!("lint: {st} ok -- {rows} AT row(s) conform");
        Ok(())
      } else {
        println!(
          "lint: {st} FAILED -- {} finding(s) over {rows} AT row(s)",
          report.findings.len()
        );
        Err(Failure::Verdict)
      }
    }
    Some((verb, _)) => unwired("at", verb),
    None => Err("error: an acceptance test command is required".into()),
  }
}

/// AC-06.4: full-text search across ST prose, issue bodies and WP text.
///
/// A miss is exit 0 with no output, not an error. "Nothing matched" is a
/// successful search, and v2's own read verbs answer an empty set the same way
/// -- making it a failure would mean every `grep`-shaped use in a script had to
/// special-case the common answer.
fn search(m: &ArgMatches) -> Result<(), Failure> {
  let query = arg(m, "query")?;
  let f = open()?;
  let hits = f.search(&query).map_err(fail)?;
  // **An unpopulated index answers every query exactly the way a genuine miss
  // does**: exit 0, zero bytes, byte-identical. So a user whose prose has never
  // been indexed is told, in the tool's own voice, that their phrase is not
  // there -- when the truth is that the question was never asked. AC-06.4 was
  // written for this shape rather than for the hits, and it is the AC-10.7
  // silent-empty class in another command.
  //
  // Asked ONLY on the empty path, and as a COUNT: a search that found
  // something has already answered the question, and paying for the check on
  // the common path would tax every successful search to inform the empty one.
  //
  // stdout stays empty in both cases on purpose -- a grep-shaped caller keeps
  // its contract and a miss is still exit 0 -- so the distinction is drawn on
  // stderr, where a diagnosis belongs and where it cannot corrupt a pipe.
  if hits.is_empty() && f.prose_sections_indexed().map_err(fail)? == 0 {
    eprintln!(
      "note: nothing is indexed, so this search could not have matched -- an empty result here does NOT mean `{query}` is absent"
    );
    // **It NAMES THE FACT AND NOT A COMMAND, on vc's ruling** (2026-08-15).
    //
    // This used to say `intent sync --to-store`. That reading was right about
    // the direction -- authored prose is disk-native under D02, so disk -> db
    // is not a recovery path for prose but the only path it has -- and wrong
    // about the command, on a ground the direction argument cannot reach:
    // **a remedy must not propose an operation whose blast radius exceeds the
    // fault it repairs.** The fault is an unpopulated prose index.
    // `--to-store` replaces the ENTIRE store, and `event_log` is the one table
    // that is durable truth and not reconstructible from the files -- so an
    // operator following that remedy to fix a search result could lose history
    // that exists nowhere else. "The direction is routine for this data" is
    // not the same claim as "this command is routine for this data".
    //
    // **There is no command whose blast radius matches this fault**, which is
    // why this states a condition rather than an action: a narrow re-index of
    // prose alone does not exist yet. Naming the too-large command anyway
    // would be trading a search result for a history nobody could get back.
    eprintln!(
      "  note: authored prose reaches the index only when the working tree is read into the store, and that has not happened in this project yet"
    );
    return Ok(());
  }
  for hit in hits {
    let heading = hit.heading.as_deref().unwrap_or("(preamble)");
    println!("{}:{}  {}  {}", hit.file, hit.seq, hit.owner_id, heading);
  }
  Ok(())
}

/// AC-06.2: the health report.
///
/// Findings go to STDOUT, not stderr, and the exit code carries the verdict.
/// A report IS the output of a successful doctor run -- the command did its
/// job when it found things -- so writing findings to stderr would put the
/// answer on the error channel. The nonzero exit is what a script reads, in
/// the same shape `ac gate` uses.
/// `intent upgrade` -- the migration door, and the only verb whose subject is
/// the state every other verb refuses.
///
/// **It goes through [`context`] and NOT [`open`]**, for the reason `doctor`
/// does: `open` loads canon through a gate that turns away an unmigrated
/// project, and an unmigrated project is the only estate this verb exists for.
/// Routing it through the usual door would make the remedy refuse the thing it
/// is the remedy for -- and the binary already prints `run intent upgrade` when
/// it refuses, so that spelling has to work.
///
/// **The exit codes are 0 and 1, and deliberately never 2.** [`Failure::Error`]
/// is "the command ran and the answer is no", which is exactly what a blocked
/// migration is; `Unavailable` (2) says this build cannot answer the question
/// at all, and consumers written against v2 read it as fail-open. A refusal to
/// convert an estate that would lose data is a verdict, not an absence, and
/// reporting it as 2 would invite exactly the wrong response.
///
/// **Nothing is written when it refuses** -- structural, not careful: the
/// facade plans into an uncommitted `WriteSet`.
///
/// The flags v2 carried here (`--backup-dir`, `--no-backup`) are NOT wired,
/// because the table holds them at `disposition: pending` and pending does not
/// ship. That is a live question -- v3 rolls back through git, so v2's backup
/// flags may not apply at all -- and wiring them to satisfy the shape would
/// answer it by accident, in a renderer, which is not where it gets answered.
fn upgrade() -> Result<(), Failure> {
  let (project, ctx) = context()?;
  // The whole operation, including the refusal. `fail` renders the message,
  // the cause chain and the remedy, and `MigrationBlocked` delegates its
  // remedy to `Blocked` -- which knows whether the estate needs repairing
  // under v2 or whether the migrator itself failed. Those are different
  // actions for different people, so this arm must not flatten them into one
  // hand-written sentence the way `ingest` does for its own single case.
  let done = Facade::upgrade(&project, &ctx).map_err(fail)?;

  // Carried findings get `ingest`'s treatment for `ingest`'s reason: the
  // section header prints ONCE, and each line goes through `carried_line`
  // rather than `Display`, which would lead with `residue:` and append a
  // remedy telling the operator to fix a row the ruling says converts as it is.
  if !done.carried.is_empty() {
    println!("carried (converts as-is, no action):");
    for finding in &done.carried {
      println!("{}", finding.carried_line());
    }
  }

  // **Per section, on stdout, beside `carried` -- never a count.** A total
  // reconciles arithmetically and tells nobody which section went, and a
  // decision the migration does not name cannot be told from one it never
  // made. Two verdicts: `dropped` removes a section from canon, `deferred`
  // keeps it and stands the renderer's own copy down.
  //
  // **The deferral row exists because vc could not read a zero** -- their
  // `DOUBLED-SECTION 20 -> 0` is produced identically by a migrator that
  // deferred to the author and by one that stopped generating the section, and
  // they separated the two only by going and reading which pointer survived.
  if !done.dispositions.is_empty() {
    println!("sections not carried as-is:");
    for d in &done.dispositions {
      println!(
        "  {} -- ## {} -- {} -- {}",
        d.owner,
        d.heading,
        d.verdict.as_str(),
        d.reason
      );
    }
  }

  eprintln!(
    "migrated: {} thread(s), {} issue(s), {} file(s) written",
    done.threads, done.issues, done.files
  );
  // **Printed only when it happened, because on a first run it is noise and on
  // a re-run it is the only thing the operator actually wants to know.** It is
  // a count of threads whose SOURCE differed, not of work skipped: they are in
  // the plan like any other and re-emit byte-identical canon.
  //
  // **It reports the observation and NOT a cause, and the first version of
  // this line got that wrong.** It said "a previous run of this command was
  // interrupted", which is ONE of at least three ways to arrive here -- the
  // others being a run that completed normally, and, until `426416e1`, a
  // project stuck in the loop where every re-run reported success and changed
  // nothing. ic caught it on a tree that had never been interrupted; their
  // gate's arm B prints this same line where interruption genuinely IS the
  // cause, and nothing in the output told the two apart.
  //
  // **The migrator was not present when that canon was written and must not
  // narrate it.** Same class as a fabricated timestamp: an invented cause is
  // indistinguishable by inspection from a recorded one, and it is worse than
  // silence because it sends the reader looking for an interruption that never
  // happened.
  if !done.already_migrated.is_empty() {
    eprintln!(
      "already migrated: {} thread(s) had committed canon and were re-emitted from it rather than converted -- their content is unchanged",
      done.already_migrated.len()
    );
  }
  // **A SEPARATE LINE RATHER THAN A SECOND CLAUSE ON THE ONE ABOVE**, because
  // the two populations reach it by different routes and can differ: a v2
  // estate mid-conversion has already-migrated threads and no canon issues at
  // all. Folding them into one sentence would make a zero on either side read
  // as a statement about both.
  if !done.already_migrated_issues.is_empty() {
    eprintln!(
      "already migrated: {} issue(s) had committed canon and were re-emitted from it rather than converted -- their content is unchanged",
      done.already_migrated_issues.len()
    );
  }
  eprintln!(
    "ok: this project is now Intent v{} -- commit the canon and the generated views",
    intentsvcs::faces::INTENT_VER
  );
  Ok(())
}

/// `intent organize` -- reconcile the tree with `.intentfiles` (D57-3).
///
/// **PARSE, CALL, RENDER. Every decision is `intentsvcs::organize`'s** -- which
/// of D57-3's five rows a path falls in, whether a removal is safe, whether the
/// estate may dehydrate at all. A reconciliation rule expressed in a renderer
/// would be a second answer beside the one the acceptance tests drive.
///
/// **NO FLAGS, AND THAT IS THE TABLE'S CALL RATHER THAN MINE.** The dispatch row
/// declares `"flags": []` and records the polarity question as OPEN and ic's
/// (AC-05.1): v2's two `organize` faces took OPPOSITE polarity for the same
/// operation, and a polarity chosen at wiring time by whoever happened to build
/// the entry is exactly how that pair came to disagree. So this ships what the
/// table declares, and a `--dry-run` arrives by adding the flag to the table
/// first. Worth stating plainly: that means the bare spelling ACTS.
///
/// **WHAT MAKES THAT SAFE TODAY IS A GATE, NOT A HABIT.** Every removal is
/// refused while any declared dehydration precondition is unmet, and separately
/// each removal must re-render byte for byte before it happens.
/// `intent organize --default` -- write `.intentfiles` from status.
///
/// **AC-11.1, AC-11.2, AC-11.4, AC-11.5.** Parse, confirm, call, render: the
/// decision about what the declaration CONTAINS is
/// `intentfiles::default_declaration`'s and the decision about whether to write
/// is `Facade::declare_default`'s. What belongs here and nowhere else is the
/// HUMAN -- whether one is present, and what they said.
fn declared_default(m: &ArgMatches) -> Result<(), Failure> {
  let force = flag(m, "force");
  let (project, ctx) = context()?;
  let manifest = project.relative(&project.intentfiles_path());

  // **THE TTY IS CHECKED BEFORE THE FACADE IS EVEN OPENED, SO A REFUSAL CANNOT
  // HAVE WRITTEN ANYTHING (AC-11.2).** Not an ordering convention: nothing
  // between here and the write can fail in a way that leaves a partial
  // declaration, because there is nothing between here and the write.
  //
  // **AND THE REFUSAL IS UNCONDITIONAL ON `--force`, NOT CONDITIONAL ON THE
  // FILE BEING PRESENT.** The criterion says `--force` without a tty writes
  // nothing and exits non-zero, and taking that literally is both the stricter
  // reading and the measurable one: a rule that fires only when a file happens
  // to exist is a rule whose test passes on the wrong fixture. **The absence of
  // a human IS the refusal** -- there is no `--yes` and no environment
  // override, because a flag-driven force would make this criterion
  // unmeasurable rather than merely weaker.
  if force && !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
    return Err(Failure::Error(format!(
      "error: `--default --force` regenerates `{manifest}` from status and needs a person to confirm it\n  remedy: run it in a terminal. There is deliberately no flag or environment variable that answers for you -- bare `intent organize --default` writes the declaration when there is none and changes nothing when there is"
    )));
  }

  let mut facade = Facade::open(project.clone(), ctx).map_err(fail)?;

  if force {
    // Asked BEFORE the write and answered by a human, so the report below is
    // always about something that has already happened.
    // **THE OLD SECOND LINE READ `it removes no files`, AND AC-11.6 MAKES THAT
    // FALSE.** It was true of every arm this verb had when it was written --
    // `--default` wrote a declaration and stopped -- and it stayed on the
    // screen while the arm underneath it became the destructive one. A confirm
    // prompt is the one piece of prose in this estate whose whole job is to be
    // accurate at the moment a human decides, so it names the act, names what
    // survives it, and names the spelling that does not do it.
    println!("about to regenerate {manifest} from thread status, and then APPLY it.");
    println!(
      "this is the destructive arm: it realises every thread the regenerated file declares, and it REMOVES the files of every realised thread it does not."
    );
    println!(
      "a thread whose preconditions are unmet keeps every one of its files, and the refusal names the thread and the precondition."
    );
    println!(
      "`intent organize --default` without --force writes the declaration and removes nothing."
    );
    print!("proceed? [y/N] ");
    use std::io::Write as _;
    let _ = std::io::stdout().flush();
    let mut answer = String::new();
    std::io::stdin()
      .read_line(&mut answer)
      .map_err(|e| Failure::Error(format!("error: could not read the confirmation: {e}")))?;
    // **ONLY `y` PROCEEDS, AND EVERY OTHER ANSWER IS A NO** -- including an
    // empty line, which is what an operator who hit return without reading
    // produces. A prompt whose default is yes is a prompt that did not ask.
    if answer.trim() != "y" {
      return Err(Failure::Error(format!(
        "error: not confirmed, so `{manifest}` is unchanged"
      )));
    }
  }

  let done = facade.declare_default(force).map_err(fail)?;

  // **THREE OUTCOMES, THREE SENTENCES.** Written-where-there-was-none,
  // left-alone, and regenerated are different events; a report that reads the
  // same for the first and third would make the safe run indistinguishable from
  // the destructive one.
  if !done.wrote {
    println!(
      "ok: {manifest} is already present and declares {} entr{}",
      done.declares,
      if done.declares == 1 { "y" } else { "ies" }
    );
    println!(
      "    nothing was written. `intent organize --default --force` regenerates it from status, after confirming on a terminal."
    );
  } else if done.was_present {
    println!(
      "ok: {manifest} regenerated from status -- now declares {} entr{}",
      done.declares,
      if done.declares == 1 { "y" } else { "ies" }
    );
  } else {
    println!(
      "ok: {manifest} written -- declares {} entr{} (every WIP thread, and nothing else)",
      done.declares,
      if done.declares == 1 { "y" } else { "ies" }
    );
  }
  // **THE TWO ARMS FORK HERE, AND THE SENTENCE BELOW IS WHY THE FORK EXISTS.**
  // It used to be unconditional and said `no file was created or removed, IN
  // ANY ARM (AC-11.4)`. That is still exactly true of `--default` on its own,
  // which is what AC-11.4 is about -- and AC-11.6 makes it false of
  // `--default --force`, which is the arm hv described as the destructive one
  // and the only one. An unconditional claim would now be a report that says
  // nothing was removed on the run that removed things.
  if !force {
    println!(
      "    no file was created or removed. `intent organize` previews what this declaration implies."
    );
    return Ok(());
  }

  // **AND THE APPLY IS THE SAME APPLY, NOT A SECOND ONE (AC-11.6).** The
  // criterion's list -- realise every declared thread, dehydrate the undeclared
  // realised ones only where every precondition holds, refuse per-thread by
  // name otherwise, then WP-13's ingest and prune -- is the definition of
  // `Mode::Apply`, so this reaches it rather than restating it. Composing the
  // two operations is exactly what `fn organize` says it will not do inline,
  // and that reservation is about the BARE verb: a run that decides what should
  // be realised and acts on it cannot be previewed, so it is available only
  // behind `--force` and a confirmed human, which is the shape hv ruled.
  println!("    applying the regenerated declaration in the same run.");
  let report = facade
    .organize(intentsvcs::organize::Mode::Apply)
    .map_err(fail)?;
  // Performed tense, and a refusal moves the exit code exactly as it does under
  // `--apply` -- the same act reported by the same code, including the part
  // where something asked to be removed and was not.
  render_organize_report(&project, &report, false)
}

fn organize(m: &ArgMatches) -> Result<(), Failure> {
  // **`--default` IS A DIFFERENT OPERATION, NOT A MODE OF THIS ONE.** It writes
  // the declaration; reconciliation reads it. Folding them into one pass would
  // make a run that both decides what SHOULD be realised and then acts on that
  // decision in the same breath -- which is precisely the shape an operator
  // cannot preview, because the input to the preview would be produced by the
  // run being previewed.
  if flag(m, "default") {
    return declared_default(m);
  }
  // **`--force` ALONE IS A USAGE ERROR AND SAYS SO.** It qualifies `--default`
  // and modifies nothing here. Accepting it silently would make
  // `organize --force` read as a forced reconciliation, which is a command this
  // build does not have and would be the most dangerous one to imply.
  if flag(m, "force") {
    return Err(Failure::Error(
      "error: `--force` qualifies `--default` and means nothing on its own\n  remedy: `intent organize --default --force` regenerates the declaration from status, after confirming on a terminal".to_string(),
    ));
  }
  // **PREVIEW UNLESS ASKED, RULED BY ic ON AC-05.1 (2026-08-19).** The polarity
  // is the surface's to decide and it is decided in the dispatch table; this
  // reads the answer rather than holding one. See the `--apply` flag's
  // `disposition_basis` for the three grounds -- the short version is that v2
  // shipped BOTH polarities for this one operation, and resolving toward
  // preview resolves it in the direction that cannot lose data.
  let mode = if flag(m, "apply") {
    intentsvcs::organize::Mode::Apply
  } else {
    intentsvcs::organize::Mode::Preview
  };
  let previewing = mode == intentsvcs::organize::Mode::Preview;
  let (project, ctx) = context()?;
  let mut facade = Facade::open(project.clone(), ctx).map_err(fail)?;
  let report = facade.organize(mode).map_err(fail)?;
  render_organize_report(&project, &report, previewing)
}

/// Render an organize report, in either tense.
///
/// **EXTRACTED BECAUSE AC-11.6 GAVE THIS BLOCK A SECOND CALLER, AND A SECOND
/// COPY WOULD HAVE BEEN THE WORST KIND** (Highlander, `IN-AG-HIGHLANDER-001`).
/// Every safety property this verb has lives in the WORDING here -- the tense
/// on every line, the `(N blocked)` parenthetical beside a `0 to remove`, the
/// no-ellipsis rule, the refusals on stderr in both modes. A copy would start
/// identical and drift, and the drift would be invisible: both copies would go
/// on printing plausible reports, and the one that lost a clause would lose it
/// in the arm that REMOVES FILES.
///
/// `previewing` is passed rather than derived from a `Mode`, because the
/// destructive arm of `--default --force` renders in the performed tense
/// without ever holding a `Mode` of its own.
fn render_organize_report(
  project: &Project,
  report: &intentsvcs::organize::Report,
  previewing: bool,
) -> Result<(), Failure> {
  // **PROJECT-RELATIVE, THROUGH THE PROJECT'S OWN ANSWER.** Measured on a real
  // estate before this was added: 199 unclaimed paths printed absolute, each
  // carrying 90 characters of temp-directory prefix before the part that
  // identifies the file. A report whose every line starts with the same
  // irrelevant 90 characters is one nobody reads to the end of, and the end is
  // where the removals are.
  let show = |path: &std::path::Path| project.relative(path);

  // **THE COUNTS LEAD AND THE PATHS FOLLOW, WITH NO ELLIPSIS ANYWHERE.** A
  // summary carrying a whole denominator is not a truncation; a `head -20` with
  // a trailing dot-dot-dot is. This verb's whole subject is files appearing and
  // disappearing, so a path must never be able to vanish from its own report --
  // being inside a counted group is fine, being dropped is not.
  //
  // **THE TENSE IS IN EVERY LINE, NOT ONLY IN A FOOTER, AND THAT IS THE WHOLE
  // SAFETY PROPERTY OF THE PREVIEW.** A trailing "nothing was written" is
  // correct and useless: these lines get grepped, pasted into a message, and
  // read one at a time, and a single line reading `removed: intent/st/...` is
  // indistinguishable from a run that removed it. Carrying the tense per line
  // costs six words and makes a line that has been separated from its footer
  // still tell the truth.
  let (head, hyd, rew, rem, prn) = if previewing {
    (
      "organize (preview):",
      "to-hydrate",
      "to-rewrite",
      "to-remove",
      "to-prune",
    )
  } else {
    ("organize:", "hydrated", "rewritten", "removed", "pruned")
  };
  // **THE REFUSAL COUNT IS ON STDOUT WITH THE OTHERS, AND IT IS NOT DECORATION.**
  // Measured on a real estate: the summary read `0 to remove` while the refusal
  // on STDERR read `would remove 544 file(s)`. Both are true and they answer
  // different questions -- this run removes nothing, and 544 are blocked behind
  // the ship gate -- but a caller capturing only stdout saw `0 to remove, 0
  // diverged` at exit 0 and had every reason to conclude the estate was
  // reconciled. It is not; it is 544 removals behind a shut gate. INV-01 puts
  // the refusal TEXT on stderr and that is right, so the COUNT has to appear
  // here or stdout is quietly complete-looking at precisely the moment it is
  // least complete.
  // **AND THE REFUSAL COUNT WAS STILL NOT ENOUGH, BECAUSE IT COUNTS REFUSALS
  // AND THE READER IS ASKING ABOUT FILES.** `PreconditionsUnmet` is deliberately
  // ONE refusal for the whole run (`organize.rs`, at the push), so a gate
  // holding back four hundred removals renders as `1 refused` -- which is
  // accurate, three orders of magnitude too small, and sits in the same line as
  // `0 to remove`. Measured on this estate by cc: stdout `0 to remove, 1
  // refused` beside a stderr refusal reading `would remove 423 file(s)`, same
  // run. Both true; only one of them gets grepped.
  //
  // **THE FIGURE GOES IN THE SUMMARY LINE ITSELF RATHER THAN A NOTE UNDER IT**,
  // for the reason the tense is in every line above: these lines get separated
  // from their context, and a line that has been separated must still tell the
  // truth. `0 to remove (423 blocked)` does; `0 to remove` with the real number
  // one line down does not.
  //
  // **AND `0 to remove` IS NOT A BUG THAT THIS PAPERS OVER -- IT IS CORRECT AND
  // THAT IS THE HAZARD.** In a preview, `to remove` answers *what would
  // `--apply` remove*, and today the answer is genuinely none, because the gate
  // refuses. It becomes 423 on the day the last precondition goes green, with
  // no edit to this verb and nothing in the output having changed shape. The
  // parenthetical is what makes the pending state visible while it is still
  // pending.
  // **EMPTY MEANS EMPTY, and a digest of nothing would be a constant that looks
  // like a measurement.** A reconciled estate prints `0 unclaimed` with no
  // parenthetical rather than `0 unclaimed (e3b0c44298fc)`, which is sha256 of
  // the empty input and says nothing at all while looking authoritative.
  let unclaimed_digest = if report.unclaimed.is_empty() {
    String::new()
  } else {
    format!(" ({})", report.unclaimed_digest())
  };
  let blocked = report.blocked();
  let blocked = if blocked == 0 {
    String::new()
  } else {
    format!(" ({blocked} blocked)")
  };
  println!(
    "{head} {} {}, {} {}, {} unchanged, {} {}{}, {} {}, {} unclaimed{}, {} diverged, {} refused",
    report.hydrated.len(),
    if previewing { "to hydrate" } else { "hydrated" },
    report.rewritten.len(),
    if previewing {
      "to rewrite"
    } else {
      "rewritten"
    },
    report.unchanged.len(),
    report.dehydrated.len(),
    if previewing { "to remove" } else { "removed" },
    blocked,
    // **PRUNED SITS BESIDE REMOVED BECAUSE IT IS THE SAME KIND OF ACT.** ic
    // added the field with the reason on it -- removing a directory is a
    // SMALLER act than removing a file and not a smaller KIND -- and then could
    // not render it, because this file was mid-edit in my tree. A destructive
    // act recorded on the report and absent from the report's own summary is
    // the one line of this verb an operator cannot review, which is precisely
    // what `0 to remove` beside a 423-file refusal already was tonight. Twice
    // in one evening, same shape, different field.
    report.pruned.len(),
    if previewing { "to prune" } else { "pruned" },
    report.unclaimed.len(),
    // **THE DIGEST RIDES ON THE SUMMARY LINE, WHICH IS THE ONLY LINE THAT MUST
    // ANSWER `DID ANYTHING CHANGE` WITHOUT A FLAG.** Grouping the 199 paths by
    // directory took the report from 199 lines to 4, and vc measured that it
    // still fails a same-directory SWAP -- byte-identical output across a
    // membership change, the changed entry at position 2 of 199. Count and
    // directory set are exactly the two quantities such a swap preserves.
    // Cardinality moves the count; membership moves this.
    unclaimed_digest,
    report.diverged.len(),
    report.refused.len()
  );
  for (label, paths) in [
    (hyd, &report.hydrated),
    (rew, &report.rewritten),
    (rem, &report.dehydrated),
    (prn, &report.pruned),
  ] {
    for path in paths {
      println!("  {label}: {}", show(path));
    }
  }
  // **Reported, never acted on, and named rather than counted.** An unclaimed
  // path means a human put it there and the renderer cannot produce it; a
  // divergence means the STORE is stale and the remedy is the operator's
  // choice. Both are inventory the operator has to see, and neither moves the
  // exit code -- they are not failures of this run.
  // **GROUPED BY DIRECTORY, AND THIS IS NOT A TRUNCATION.** hv met the
  // unbounded form directly: 199 unclaimed paths, printed in full on every run,
  // wrapping the eight lines that carry the decision. Measured by cc -- 196
  // `.tap`, 2 `.tsv`, 1 `.gitkeep`, across exactly TWO directories -- and
  // **nothing will ever move them out of that bucket**: they are not views (the
  // renderer cannot produce a `.tap`), not attachments (`ATTACHMENT_EXTENSIONS`
  // is md/txt/sh), and not declarable, because `.intentfiles` names ARTEFACTS
  // and never FILES. Permanently unclaimed by construction.
  //
  // **A REPORT WHOSE FIRST TWO HUNDRED LINES ARE IDENTICAL ON EVERY RUN TRAINS
  // ITS READER TO STOP LOOKING**, and the run where one of those lines changes
  // is then the run nobody sees. That is the third instance of one class today
  // -- `view_skew_check.sh` printing a clean-sounding summary while the drift it
  // existed for grew, and `runner_roster_check.sh` green at 11 gated over a
  // wiring that judged the wrong commit. Both were PRESENT and technically
  // correct. This one arrives through volume rather than wording.
  //
  // **THE COUNT-AND-NO-ELLIPSIS RULE ABOVE IS HONOURED, NOT WAIVED.** Its own
  // words are *being inside a counted group is fine, being dropped is not*, and
  // a directory line carrying its own count is exactly a counted group -- the
  // total still reconciles against the summary. The rule's justification is
  // that this verb's subject is files appearing and DISAPPEARING; an unclaimed
  // file is doing neither. It is inventory, and inventory groups.
  let mut by_dir: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
  for path in &report.unclaimed {
    let shown = std::path::PathBuf::from(show(path).to_string());
    let dir = shown
      .parent()
      .map(|d| d.display().to_string())
      .filter(|d| !d.is_empty())
      .unwrap_or_else(|| ".".to_string());
    *by_dir.entry(dir).or_default() += 1;
  }
  for (dir, count) in &by_dir {
    println!("  unclaimed: {dir}/ ({count} file(s))");
  }
  for path in &report.diverged {
    println!("  diverged: {}", show(path));
  }

  // **THE FOOTER IS BELT-AND-BRACES, NOT THE MECHANISM.** The per-line tense
  // above is what makes a preview unmistakable; this says it once more in plain
  // words and names the spelling that performs it, so the operator never has to
  // go and look the flag up.
  if previewing {
    println!(
      "organize: preview only -- nothing was written or removed. `intent organize --apply` performs it."
    );
  }

  if report.refused.is_empty() {
    return Ok(());
  }
  // **EVERY REFUSAL IS SHOWN, IN BOTH MODES.** "27 removals refused" with one
  // example is the truncation this verb can least afford, and in a preview the
  // refusals are the most valuable thing on the screen: they are the answer to
  // "what will happen if I now type `--apply`", which is the only question the
  // preview exists to answer.
  for refusal in &report.refused {
    eprintln!("{}", refusal.render());
  }

  // **A REFUSAL MOVES THE EXIT CODE ON THE APPLY PATH AND NOT ON THE PREVIEW,
  // AND THIS IS THE ONE PLACE THE TWO MODES DELIBERATELY DISAGREE.**
  //
  // On `--apply` the code must move: something asked to be removed and was not,
  // so a script treating the run as success carries on believing the estate is
  // reconciled when it is not.
  //
  // A preview did exactly what it was asked to do -- it previewed -- and
  // nothing about the estate is now mis-believed, because nothing changed. The
  // deciding argument is what the alternative costs TODAY: nineteen
  // preconditions are unmet, so every preview on this estate carries a
  // `PreconditionsUnmet` refusal, and a preview that exited non-zero would make
  // the BARE, DEFAULT spelling of a routine verb fail permanently on a healthy
  // project. That is the always-on alarm that `Plan::run` already refuses to
  // build on the digest guard, arriving one layer up at the exit code -- and an
  // always-failing default is how an operator learns to stop reading this
  // command's output at all, which costs exactly the refusals above.
  if previewing {
    return Ok(());
  }
  Err(Failure::Verdict)
}

/// Read the history out of the one place it lives.
///
/// **The other half of the ruling that deleted the tracked extract.** Removing
/// the projection without this would have traded an unread file for an
/// unreadable table -- no `events` verb existed in either binary, so the file
/// was the only reader-facing surface the log had.
fn events(m: &ArgMatches) -> Result<(), Failure> {
  let (project, ctx) = context()?;
  let facade = Facade::open(project, ctx).map_err(fail)?;
  let filter = EventFilter {
    op: m.get_one::<String>("op").cloned(),
    subject: m.get_one::<String>("subject").cloned(),
    limit: m
      .get_one::<String>("limit")
      .map(|n| {
        n.parse::<usize>().map_err(|_| {
          Failure::Unavailable(format!(
            "error: `{n}` is not a count\n  remedy: --limit takes a whole number, eg --limit 20"
          ))
        })
      })
      .transpose()?,
  };
  let page = facade.events(&filter).map_err(fail)?;

  if m.get_one::<String>("format").is_some_and(|f| f == "json") {
    let rows: Vec<serde_json::Value> = page
      .rows
      .iter()
      .map(|e| {
        serde_json::json!({
          "id": e.id, "ts": e.ts, "op": e.op,
          "subject": { "kind": e.subject.kind, "id": e.subject.id },
          "payload": e.payload,
        })
      })
      .collect();
    println!(
      "{}",
      serde_json::to_string_pretty(&serde_json::json!({
        "shown": page.rows.len(), "matched": page.matched, "total": page.total, "events": rows,
      }))
      .unwrap_or_default()
    );
    return Ok(());
  }

  // **AN EMPTY STORE SAYS SO, AND IT IS NOT A CLEAN BILL OF HEALTH.** A fresh
  // clone has no history at all -- that is the accepted cost of the log living
  // only in the store -- and silence at exit 0 cannot tell "nothing happened
  // here" from "everything is fine", which is the distinction the prose critics
  // lost by emitting nothing.
  if page.total == 0 {
    println!("events: no history in this store -- nothing has been recorded here yet.");
    return Ok(());
  }
  if page.matched == 0 {
    println!(
      "events: no event matches this filter, of {} in this store.",
      page.total
    );
    return Ok(());
  }

  for e in &page.rows {
    // The disk verbs name a PATH SET rather than an artefact, so their subject
    // id is empty by design; printing an empty column for them would read as a
    // missing value rather than an inapplicable one.
    match e.subject.id.as_str() {
      "" => println!("{}  {}  {}", e.ts, e.id, e.op),
      id => println!("{}  {}  {}  {}", e.ts, e.id, e.op, id),
    }
  }
  // **THE DENOMINATOR IS ROWS, NEVER VERBS, AND ALL THREE NUMBERS ARE SAID
  // WHEN THEY DIFFER.** A count of what was printed reported as a count of
  // what exists is this estate's most-repeated defect; a limit and a filter
  // move different numbers, so the line names which moved.
  if page.rows.len() == page.total {
    println!("events: {} event(s).", page.total);
  } else {
    println!(
      "events: showing {} of {} matched, {} in this store.",
      page.rows.len(),
      page.matched,
      page.total
    );
  }
  Ok(())
}

/// Advisories are printed above the summary and never counted in it; the suffix
/// says how many were set aside so a reader cannot mistake "0 finding(s)" for
/// "nothing printed" (hv, 2026-08-26).
fn advisory_suffix(report: &intentsvcs::doctor::Report) -> String {
  match report.advisories() {
    0 => String::new(),
    n => format!(" -- {n} advisory(ies), not counted"),
  }
}

fn doctor(a: &ArgMatches) -> Result<(), Failure> {
  // **QUIET WINS OVER VERBOSE, and that is v2's rule rather than a tie-break
  // invented here** -- `bin/intent_doctor:134` reads
  // `if [ "$VERBOSE" = true ] && [ "$QUIET" != true ]`, so the two together
  // resolve to quiet in v2 and must resolve the same way in v3. A parity flag
  // whose interaction with its sibling differs from v2's is a flag that has
  // been re-designed under the name of being carried across.
  let quiet = a.get_flag("quiet");
  let verbose = a.get_flag("verbose") && !quiet;
  // **REFUSED BEFORE THE WORK, AND BY THE RENDERER RATHER THAN BY CLAP.**
  // A `value_parser` would reject at exit 2, which is INV-04's USAGE code and
  // the one the pre-commit gate FAILS OPEN on -- so a typo in `--format` would
  // read to every v2-era consumer as *the checker is broken, carry on* instead
  // of as a refusal. Exit 1 is the honest answer: the command ran and the
  // answer is no. This is the same ruling already recorded against `critic`'s
  // vocabulary, applied to the flag rather than to the command.
  //
  // It runs BEFORE `context()` because refusing costs nothing and diagnosing a
  // whole estate to then reject the request is work done for an answer we have
  // already decided not to give.
  let format = enum_flag(a, "doctor", "--format")?;
  let (project, ctx) = context()?;
  // **Opened opportunistically, and a failure to open is not reported here.**
  // `doctor` exists to run on a project that cannot be opened, so the store is
  // a bonus rather than a requirement: with one, the backup half of the report
  // is answerable; without one, every other check still runs and the backup
  // question is simply not asked. Reporting "no backup" because the store
  // could not be read would be a confident wrong answer at the moment a user
  // is least able to check it.
  let opened = Facade::open(project.clone(), ctx.clone()).ok();

  // **WHAT THIS RUN RESOLVED, WHICH IS v2's `--verbose` AND NOT A NEW IDEA.**
  // v2 emits `INTENT_HOME=...` and `Found at ...` under the flag
  // (`bin/intent_doctor:204,217`): the value is naming WHERE the answers came
  // from, on the one command a user reaches for when the others have stopped
  // working. The v3 equivalents are the paths this run read.
  //
  // **THE STORE LINE IS THE ONE THAT EARNS THE FLAG.** The comment above says
  // the backup question is not asked when the store will not open -- correct,
  // and until now NOTHING SAID WHICH RUN HAPPENED. Two reports differing by a
  // whole check looked identical, so a reader could not tell a clean bill of
  // health from a check that was skipped. That is this estate's own recurring
  // sentence arriving inside `doctor` itself.
  if verbose {
    println!("doctor: root      {}", project.root().display());
    println!("doctor: intent    {}", project.intent_dir().display());
    println!("doctor: canon     {}", project.canon_dir().display());
    println!(
      "doctor: store     {}",
      match opened {
        Some(_) => "opened -- the backup check was asked".to_string(),
        None =>
          "NOT opened -- the backup check was NOT asked, and its silence is not a pass".to_string(),
      }
    );
  }

  let report = Facade::doctor(&project, &ctx, opened.as_ref().map(|f| f.store()));
  // **THE MACHINE FACE THIS FILE ASKED FOR IN WORDS.** The `unattached` block
  // below carries the note *it stays inline until `doctor` has a machine face
  // to carry it, which needs a surface row and is not mine to add*. This is
  // that face, and the row is now declared on `doctor` in the table.
  //
  // **IT CARRIES THE WHOLE REPORT, INCLUDING THE COUNTS.** `Report`'s own doc
  // comment says the counts exist so a clean result can say what it covered --
  // *no problems found* over an estate the checker never read is the same
  // sentence as *no problems found* over one it read completely. A JSON face
  // that emitted only `findings` would drop exactly the half that tells those
  // apart, and a machine reader has no summary line to fall back on.
  //
  // `--quiet` and `--verbose` are not consulted here, deliberately: both shape
  // a HUMAN reading, and a machine face that changed its keys under a verbosity
  // flag would make every consumer's parse conditional on how it was invoked.
  if format == "json" {
    render_doctor_json(&report);
    return if report.is_healthy() {
      Ok(())
    } else {
      Err(Failure::Verdict)
    };
  }
  // **AN ADVISORY IS COUNTED AND POINTED AT, NOT PRINTED** (hv, 2026-08-26,
  // reading Baize: "How is this an improvement?"). Reclassifying them fixed the
  // exit code and left the terminal exactly as buried -- 66 blocks, four lines
  // each, every one of them saying nothing is owed. A report whose actionable
  // half is invisible under notes nobody has to act on is the defect the class
  // was introduced to cure, so the bodies move behind `--verbose`, which is
  // already the flag for "what this run resolved". `--quiet` drops them
  // entirely, like every other line that does not move the exit code.
  for finding in &report.findings {
    if finding.class == intentsvcs::finding::FindingClass::Advisory && !verbose {
      continue;
    }
    println!("{finding}");
  }
  if !quiet && !verbose && report.advisories() > 0 {
    println!(
      "advisory: {} note(s) not shown and not counted -- `intent doctor --verbose` reads them",
      report.advisories()
    );
  }
  // **`--quiet` DROPS WHAT IS NOT A FINDING, AND THESE ARE NOT FINDINGS** --
  // the doc comment on `withheld_flags` says so in those words, and
  // `Report::unattached` says it again. v2's `--quiet` is "only show errors and
  // warnings", so the line between kept and dropped is already drawn by the
  // estate: anything that does not move the exit code goes.
  if !quiet {
    for withheld in withheld_flags(&dispatch::table()) {
      println!("{withheld}");
    }
  }
  // **Named, every one, and NOT as findings.** These files are outside the
  // carried extensions by design, so they are inventory rather than faults and
  // they do not move the exit code. They are printed because the alternative
  // is silence, and silence is what lets a file vanish when the disk stops
  // being the place things live.
  if !quiet && !report.unattached.is_empty() {
    // **The count LEADS and it is complete.** A summary carrying a whole
    // denominator is not a truncation; a `head -20` with an ellipsis is (vc).
    // The shape is what a reader needs first -- on this project 196 of these
    // are one thread's TAP baselines, which is an outlier rather than the
    // normal case, and a bare list of 237 paths says none of that.
    let mut by_ext: Vec<(String, usize)> = Vec::new();
    for path in &report.unattached {
      let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("(none)")
        .to_string();
      match by_ext.iter_mut().find(|(e, _)| *e == ext) {
        Some((_, n)) => *n += 1,
        None => by_ext.push((ext, 1)),
      }
    }
    by_ext.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let shape: Vec<String> = by_ext.iter().map(|(e, n)| format!("{n} *.{e}")).collect();
    println!(
      "doctor: {} file(s) under a thread are not carried by the store",
      report.unattached.len()
    );
    println!("        {}", shape.join(", "));
    // **Every path, and no ellipsis.** The rule this satisfies is that a file
    // must never disappear from the report -- appearing inside a counted group
    // is fine, vanishing is not. It stays inline until `doctor` has a machine
    // face to carry it, which needs a surface row and is not mine to add.
    for path in &report.unattached {
      println!("  {path}");
    }
  }
  println!(
    // **THE SUMMARY SURVIVES `--quiet`, DELIBERATELY, AND IT IS THE ONE
    // INFORMATIONAL LINE THAT DOES.** Dropping it would make a clean run under
    // `--quiet` print NOTHING AT ALL at rc=0 -- and silence on success is
    // indistinguishable from the command never having run, which is the exact
    // defect this estate spent 2026-08-20 finding in its own commit gate. The
    // counts are also the coverage denominator: "no problems found" over an
    // estate the checker never read is the same sentence as "no problems
    // found" over one it read completely, and `Report`'s own doc comment says
    // the counts exist to tell those apart. `--quiet` is for less noise, not
    // for a verdict you cannot check.
    "doctor: {} finding(s) across {} thread(s), {} issue(s), {} view(s), {} file(s){}",
    report.actionable(),
    report.threads_checked,
    report.issues_checked,
    report.views_checked,
    report.files_checked,
    advisory_suffix(&report)
  );
  if report.is_healthy() {
    Ok(())
  } else {
    // The report above IS the message; the exit code is the machine's copy.
    Err(Failure::Verdict)
  }
}

/// Read an estate's MARKDOWN into the store through the API gate -- the
/// recovery path and the v2 migrator, behind one verb.
///
/// **`[PATH]` is optional, and the asymmetry is the argument for it.** Given, we
/// ingest that tree: the migrator's case, where the estate belongs to another
/// project. Omitted, this project's own markdown: the recovery case, where you
/// are already standing in it. A migrator invoked on another tree names it; a
/// user recovering their own estate does not have to say where they are.
///
/// **The project is opened WITHOUT a facade, deliberately.** [`Facade::open`]
/// loads canon, and an estate that has to be read from markdown is one whose
/// canon is missing or was never written -- so opening a facade first would
/// require the project to be migrated before it could be migrated.
///
/// **On `--from-md`, which this arm reads and cannot act on differently.** ic
/// objected that it is a mode flag with one mode; vc ruled it kept because
/// withdrawing it would put the table in contradiction with ratified rows, and
/// sent the objection to AC-10.2/10.3 where its acceptance lands. Wiring the
/// verb turned that judgement into a measurement, and it comes out ic's way:
/// the OTHER thing `ingest` could have meant -- rebuilding the store from the
/// committed JSON canon -- is already `intent sync --from-disk`
/// (`Facade::sync_from_disk`), so giving bare `ingest` that meaning would be
/// two commands for one operation. **Markdown is what is left, and it is the
/// whole of what is left.** Recorded here rather than acted on: the flag is
/// ratified, the ruling names where the objection belongs, and a renderer is
/// not the place to overturn either.
fn ingest(a: &ArgMatches) -> Result<(), Failure> {
  let project = match opt(a, "path") {
    Some(path) => Project::open(std::path::Path::new(&path)).map_err(|e| {
      format!(
        "error: {e}\n  remedy: give `intent ingest` the root of an Intent project -- the directory holding `intent/`, not the markdown itself"
      )
    })?,
    None => context()?.0,
  };
  let scan = Facade::ingest_from_md(&project).map_err(fail)?;

  // **The residue report, in migration.md's format: one line per finding,
  // machine-parseable, human-actionable, and NEVER truncated.** A capped list
  // reads as complete when it is not, which on a migration means an operator
  // fixing what they were shown and hitting the rest one command later.
  for finding in &scan.residue {
    println!("{finding}");
  }
  // Carried findings are printed too, and marked, because the counts have to
  // reconcile: hv's ruling is that a closed thread's legacy grammar CONVERTS
  // rather than blocking, and a report that showed only blockers would leave
  // the operator unable to tell "nothing wrong" from "not looked at".
  // **The header is a SECTION header, so it prints once** (ic, measured on the
  // canary: nine carried findings, nine copies of it). And each line goes
  // through `carried_line` rather than `Display`, because `Display` leads with
  // `residue:` -- the word this report reserves for the blocking bucket -- and
  // appends a remedy telling the operator to fix a row the ruling says converts
  // as it is. The counts said `0 blocking, 9 carried` while every line above
  // them said otherwise.
  if !scan.carried.is_empty() {
    println!("carried (converts as-is, no action):");
    for finding in &scan.carried {
      println!("{}", finding.carried_line());
    }
  }

  let wps: usize = scan.threads.iter().map(|t| t.wps.len()).sum();
  let criteria: usize = scan.threads.iter().map(|t| t.criteria.len()).sum();
  let tests: usize = scan.threads.iter().map(|t| t.tests.len()).sum();
  eprintln!(
    "read: {} thread(s), {wps} work package(s), {criteria} criteria, {tests} acceptance test(s)",
    scan.threads.len()
  );
  eprintln!(
    "residue: {} blocking, {} carried",
    scan.residue.len(),
    scan.carried.len()
  );

  if scan.residue.is_empty() {
    // **It says what it did NOT do.** This is the read-only half of the
    // migration; someone who runs it and sees "ok" will otherwise assume their
    // project has been converted, and then be surprised by either answer to
    // "did it work".
    eprintln!("ok: this estate parses -- nothing was read into a store and nothing was written");
    Ok(())
  } else {
    // Live-thread residue BLOCKS. The remedy names the fixing environment,
    // which is v2: this binary refuses what it cannot convert without loss,
    // and the tool that can repair a v2 artefact is the last v2 release.
    Err(
      "error: this estate has residue in live steel threads, so migrating it now would lose data\n  remedy: fix the rows named above under v2 tooling, then run this again -- the `carried:` lines are not yours to fix, they convert as they are"
        .into(),
    )
  }
}

/// `intent todo` -- the flat DOING / TODO / DONE view.
///
/// The bare command lists, because the table declares `arity: "0..1"` with
/// `default: "list"` on its verb slot.
///
/// **The view is rendered from the store, not read off `intent/todo.md`.** v2's
/// help says "show intent/todo.md (generates it if absent)", which under the
/// reversed D01 would mean showing an extract that may be stale while truth
/// sits one query away. The bytes are the same bytes -- `todo update` writes
/// exactly what this prints -- so nothing observable changes except that a
/// stale file can no longer be shown as current.
fn todo(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    None | Some(("list", _)) => {
      let f = open()?;
      // `--json` is declared on BOTH the family and the `list` verb, so it is
      // read from whichever level carried it -- `intent todo --json` and
      // `intent todo list --json` are the same request.
      // The flag may sit on the family or the subcommand, so both are asked and
      // either answering json is json.
      let json = output_of(m)?.format() == Format::Json
        || match m.subcommand() {
          Some((_, a)) => output_of(a)?.format() == Format::Json,
          None => false,
        };
      if json {
        let buckets = f.todo_buckets().map_err(fail)?;
        println!(
          "{}",
          serde_json::to_string_pretty(&buckets)
            .map_err(|e| format!("error: the view could not be rendered as JSON: {e}"))?
        );
      } else {
        // **The WINDOWED view, and this is the one place the two differ**
        // (D44, vc's surface ruling). `intent/todo.md` carries every
        // completion because a committed artefact is a record; what a person
        // reads at a terminal is a moment, and DONE is trimmed to
        // `todo.window_hours`. Same generator, one parameter apart.
        print!("{}", f.todo_view().map_err(fail)?);
      }
      Ok(())
    }
    Some(("update", _)) => {
      let mut f = open()?;
      f.todo_update().map_err(fail)?;
      println!("ok: todo.md regenerated");
      Ok(())
    }
    Some(("done", a)) => todo_done(a),
    // **`notdone` and `toggle` REFUSE, and the refusal is the honest
    // implementation rather than a gap.**
    //
    // Both reopen finished work, and every route from a finished state in the
    // ratified transition machine -- `st reopen`, `wp reopen` -- is guarded
    // `ReasonRecorded`. The declared surface for these two carries a specifier
    // and nothing else, so there is no argument for a reason to arrive
    // through: the spine is BUILT from the table, so a positional the table
    // does not declare cannot be added here even if it should be.
    //
    // That leaves exactly three options, and two of them are worse than
    // refusing. Synthesising a reason ("reopened via todo") puts a sentence
    // nobody wrote into the permanent record, where nothing downstream can
    // tell it from one someone meant -- the confected-evidence class, one
    // field over. Bypassing the guard for this one caller makes the machine
    // advisory, and a guard with a documented way round it is not a guard.
    //
    // So it refuses and names the route that records a reason. The surface
    // question -- whether these rows should grow a reason argument or be
    // withdrawn -- belongs to the table's owner, and a refusal that says so is
    // what makes it visible rather than a silently missing verb.
    Some((verb @ ("notdone" | "toggle"), a)) => {
      let spec = opt(a, "specifier").unwrap_or_default();
      let target = if spec.is_empty() {
        "the thread".to_string()
      } else {
        spec
      };
      Err(format!(
        "error: `todo {verb}` reopens finished work, and a reopen must record why it happened\n  remedy: run `intent st reopen {target} \"<reason>\"` (or `intent wp reopen`), which records the reason on the thread and in the event log"
      )
      .into())
    }
    Some((verb, _)) => unwired("todo", verb),
  }
}

/// `intent todo done <specifier>` -- mark one thread or work package done.
///
/// **It was three operations behind one verb and D44 removed two of them.**
/// `--flush` advanced a DONE watermark and `--prune` was `--flush` with its
/// output kept; hv withdrew both, and the sentence that did it removes more
/// than the two flags: *"all of the data is in the db so we can (re)generate
/// whatever we need"*. There was never any durable state behind the watermark
/// to advance -- the DONE bucket is computed at render time -- so the verb
/// that maintained it had no referent rather than merely being unnecessary.
///
/// The table retired the rows first (ic, `0855eb4e`), which is the only order
/// that works: the spine builds its flags FROM the table, so the flags were
/// already gone from the surface by the time these arms were reached. Removing
/// the arms first would have left declared flags with no implementation.
fn todo_done(a: &ArgMatches) -> Result<(), Failure> {
  let flush = flag(a, "flush");
  let prune = flag(a, "prune");
  let spec = opt(a, "specifier");

  match (spec, flush || prune) {
    (Some(spec), false) => {
      let mut f = open()?;
      // `scope_of` already owns "is this a thread or a work package": `ac gate`
      // and `wp_target` both parse specifiers through it, and a second reading
      // of `ST0001/02` here is a second place for the answer to differ.
      let outcome = match scope_of(&spec) {
        (st, Scope::Thread) => f.st_done(&st).map_err(fail)?,
        (st, Scope::WorkPackage(seq)) => f.wp_done(&st, seq).map_err(fail)?,
      };
      reported(&outcome, &spec, "done");
      Ok(())
    }
    (None, true) => {
      let mut f = open()?;
      let flushed = f.todo_flush().map_err(fail)?;
      if prune {
        // **The archiving payload FIRST, then the effect.** A caller
        // redirecting this -- `intent todo done --prune >> intent/done.md`, the
        // use v2 documented -- wants the items, and printing a status line into
        // the middle of their archive is what a summary-first order would do.
        // The advisory goes to stderr for the same reason.
        for item in &flushed.cleared {
          println!("{item}");
        }
      }
      let mark = flushed.watermark.as_deref().unwrap_or("(none)");
      eprintln!(
        "ok: DONE watermark advanced to {mark}, {} item(s) cleared",
        flushed.cleared.len()
      );
      // **A DONE view that did not empty is explained rather than left to look
      // like a failure.** This used to say today's work was unflushable, and
      // that stopped being true when the watermark became an instant and
      // `completed` started being widened to midnight: work finished this
      // morning now flushes. What can still survive is a completion date at or
      // after the cutoff, which means a future-dated `completed:` -- a
      // hand-edited thread, or two machines whose clocks disagree under D34.
      if !flushed.remaining.is_empty() {
        eprintln!(
          "note: {} item(s) carry a completion date at or after {mark} and stay in DONE",
          flushed.remaining.len()
        );
      }
      Ok(())
    }
    // Both a target and a flush: two different operations in one invocation,
    // and the order between them changes the result, so it refuses rather than
    // picking one.
    (Some(_), true) => Err(
      "error: `todo done <specifier>` marks one item done and `--flush` clears the whole DONE view; naming both asks for two different operations at once\n  remedy: run them separately -- mark the item done first, then `intent todo done --flush`"
        .into(),
    ),
    (None, false) => Err(
      "error: `todo done` needs something to do\n  remedy: name a thread or work package (`intent todo done ST0000`, `ST0000/02`), or pass `--flush` to advance the DONE watermark"
        .into(),
    ),
  }
}

/// `intent export --format <fmt>` -- the estate as one portable document
/// (AC-06.6, AC-02.6, D34).
///
/// **It writes to STDOUT, and the declared surface leaves no other option.**
/// The dispatch row carries exactly one flag, `--format`; there is no path
/// argument, so there is nowhere for the command to put a file that the
/// operator chose. Inventing one here would be inventing surface. Stdout is
/// also the better answer on its own merits -- `intent export > estate.json`
/// composes, never clobbers anything the operator did not name, and matches
/// `intent schema`, which prints a face the same way.
///
/// **That made the row's `read_or_mutate: mutate` describe a command that
/// cannot exist as declared**, and it has since been corrected to `read` (ic,
/// `f394ca9c`). The old value was reasoned from "export writes files into the
/// working tree and can clobber them", which is true only of a version of this
/// command that has an output path -- sound reasoning about the wrong subject.
/// Two routes agreed on the correction: the definition quantifies over every
/// flag and the only flag picks a projection rather than a destination, and
/// `schema` is the same shape (one flag, a face printed to stdout) and was
/// already `read`, so counting `intent export > estate.json` as mutation would
/// have left the table disagreeing with itself about one command shape.
///
/// The refusals are the facade's, in full, and none of them is composed here --
/// an unknown format, a format that will not carry the canon back, and the
/// exporter failing its own round-trip are three different answers with three
/// different remedies, and the layer that knows which one happened is the one
/// that says so.
/// `intent init` -- the ONE command that must work with no project present.
///
/// **It never calls [`open`], and that is structural rather than an
/// optimisation.** Every other verb resolves a project first; this one exists
/// because there is not one yet, so reaching for the facade would refuse the
/// command on the exact condition it is meant to remove.
///
/// **`--with-st0000` AND `--lang` ARE DECLARED `keep` IN THE TABLE AND ARE
/// REFUSED HERE BY NAME.** Their subsystems answer 2 in this build -- measured
/// 2026-08-20, `intent lang init rust` returns `not implemented yet` -- so
/// accepting either would report a project set up in a way it is not. **A flag
/// that is silently ignored is worse than one that refuses**: the operator gets
/// what they asked for in the exit code and not in the tree, and nothing
/// downstream ever says so. Refusing names the flag, the reason and the state.
fn init(a: &ArgMatches) -> Result<(), Failure> {
  // **THE ID IS THE LONG SPELLING WITH `--` STRIPPED AND THE HYPHENS KEPT.**
  // `DispatchFlag::arg_id` returns `self.long()`, so the table's
  // `--with-st0000` is the id `with-st0000`. The first version of this loop
  // spelled it `with_st0000`, snake-cased out of habit, and **the refusal
  // never fired** -- `init --with-st0000` created the project and ignored the
  // flag. cc drove it and found it inside the hour.
  //
  // **THE ONE-CHARACTER TYPO IS NOT THE DEFECT. `.ok()` WAS.** The original
  // read `try_get_one::<bool>(flag).ok().flatten()`, which turns clap's
  // `UnknownArgument` -- an id that does not exist, which is only ever a bug --
  // into `None`, indistinguishable from a flag the operator did not pass. So
  // the guard written to stop a flag being silently ignored was itself
  // silently ignored, three lines under a comment saying why that is the worse
  // failure. An unknown id now panics: it cannot happen in a shipped build,
  // and if it does, the renderer and the table have drifted and nothing else
  // would say so.
  for (flag, needs) in [
    ("with-st0000", "the ST0000 bootstrap"),
    ("lang", "`intent lang init`"),
  ] {
    let asked = match a.try_get_one::<bool>(flag) {
      Ok(v) => v.copied().unwrap_or(false),
      // Not a bool: the table declares this one as a string with a value.
      Err(clap::parser::MatchesError::Downcast { .. }) => a
        .try_get_one::<String>(flag)
        .unwrap_or_else(|e| {
          panic!("`init` reads a flag id the surface does not build: {flag} ({e})")
        })
        .is_some(),
      Err(e) => panic!("`init` reads a flag id the surface does not build: {flag} ({e})"),
    };
    if asked {
      return Err(Failure::Unavailable(format!(
        "error: `--{}` cannot be honoured in this build -- {needs} is not implemented yet\n  \
         remedy: run `intent init` without it; the project is created either way, and nothing \
         about it forecloses running that step once the command lands",
        flag.replace('_', "-")
      )));
    }
  }

  let cwd = std::env::current_dir().map_err(|e| Failure::Unavailable(format!("error: {e}")))?;
  // The directory name is the table's declared default for `project_name`, and
  // it is read here rather than defaulted in the surface so the fallback and
  // the declaration cannot disagree.
  let name = opt(a, "project_name").unwrap_or_else(|| {
    cwd
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_else(|| "project".into())
  });
  // **NO ENVIRONMENT READ HERE, AND IT WAS `std::env::var("USER")` UNTIL
  // AC-11.3 CAUGHT IT.** The shipped surface reads exactly one environment
  // variable, and the guard is protecting precisely this command: AC-07.1's
  // case is *a brew-installed binary meeting a machine with no clone and no
  // developer environment*, so reaching for a developer's `$USER` inside the
  // one verb that must work there is the wrong read in the wrong place.
  // Nothing would ever have failed here -- every machine in this estate has
  // `USER` set, which is what makes the guard worth more than the test run.
  //
  // **THE ENV READ BELONGS IN `bootstrap`, AND THAT IS NOW WHERE IT IS.**
  // `bootstrap` writes `~/.intent/config.json` once, at developer-environment
  // setup, reading `$USER` through `userstate::author()` under hv's grant of
  // 2026-08-27. This reads that file rather than the environment, so the
  // identity has exactly one home and `init` stays inside AC-11.3's invariant
  // -- AC-07.1's case is a brew-installed binary on a machine with no clone and
  // no developer environment, and reaching for `$USER` in the one verb that
  // must work there was the wrong read in the wrong place.
  //
  // **THE PARAGRAPH THIS REPLACES SAID `bootstrap` ANSWERS 2, WHICH WAS TRUE
  // WHEN WRITTEN AND STOPPED BEING TRUE THE MOMENT THE ARM ABOVE LANDED.** A
  // comment describing a sibling's state is a fact with an owner somewhere
  // else; it is edited here in the same commit that falsified it, because the
  // next reader has no way to tell a stale claim from a current one.
  let recorded = intentsvcs::bootstrap::recorded_author();
  let author = recorded.as_deref().unwrap_or("unknown");

  // `author` and not `&author`: it became a `&str` when the `$USER` read came
  // out for AC-11.3, and the borrow that was right for the `String` before it
  // survived the type change. **The compiler accepts both, so only clippy sees
  // it** -- and no local dispatcher runs clippy, which is how it reached HEAD.
  let made = intentsvcs::init::init(&cwd, &name, author, env!("CARGO_PKG_VERSION"))
    .map_err(|e| Failure::Unavailable(format!("error: {e}")))?;

  println!("created: {} at {}", made.project_name, made.root.display());
  // **ONLY SAID WHEN IT IS TRUE.** An unconditional "author is unset" line
  // survived into every run that DID resolve one, which is the same
  // stale-by-construction shape as the comment above.
  if recorded.is_none() {
    println!(
      "  author is unset -- run `intent bootstrap` to record it once for this machine, or set it in {}",
      made
        .config
        .strip_prefix(&made.root)
        .unwrap_or(&made.config)
        .display()
    );
  }
  println!("  {}", made.config.display());
  for p in &made.written {
    println!("  {}", p.strip_prefix(&made.root).unwrap_or(p).display());
  }
  // **THE SKIPPED SET IS PRINTED, because a short file list is otherwise
  // indistinguishable from a truncated one.** Each line is a decision with a
  // reason, which is the difference between "init wrote four files" and "init
  // wrote four of fourteen and you cannot tell which ten are missing".
  if !made.skipped.is_empty() {
    println!(
      "  ({} embedded template(s) deliberately not written -- run with --help for the family notes)",
      made.skipped.len()
    );
  }
  Ok(())
}

/// `intent bootstrap`: set THIS MACHINE up.
///
/// **THE CALLER hv RULED FOR `install::publish_home()`** (2026-08-27,
/// `164d5bce`), chosen over `doctor` -- which would make a diagnostic a writer
/// -- and over a new `install` verb, which is new surface with a whole parity
/// apparatus behind a one-line job. `bootstrap` is the only command whose NAME
/// already means *set this machine up*, and the pointer is a machine-level
/// fact.
///
/// **BOTH FLAGS ARE READ HERE AND THEY HAD TO BE.** `flag_reachability`
/// exempts a family with no renderer arm and decides that BEHAVIOURALLY, by
/// running the command and matching the unwired phrase. So wiring this arm
/// lifted the exemption for `--force` and `--quiet` in the same commit that
/// created it -- which is why the pointer half could not ship on its own:
/// `--force`'s declared help is *"Force recreation of config even if it
/// exists"*, and it names the CONFIG.
fn bootstrap(m: &ArgMatches) -> Result<(), Failure> {
  let force = flag(m, "force");
  let quiet = flag(m, "quiet");

  let report = intentsvcs::bootstrap::run(force).map_err(|e| Failure::Error(Remedy::render(&e)))?;

  // **`--quiet` SUPPRESSES THE REPORT, NEVER THE WORK, AND NEVER A FAILURE.**
  // The `?` above has already returned any error to the spine, which writes it
  // to stderr regardless -- a quiet flag that could hide a failed setup would
  // be the silent-error class in the one command an operator runs when nothing
  // works yet.
  if quiet {
    return Ok(());
  }

  match &report.pointer {
    intentsvcs::install::Published::Unchanged { root } => {
      println!("ok: install root already recorded -- {}", root.display());
    }
    intentsvcs::install::Published::Written { root } => {
      println!("created: install root recorded -- {}", root.display());
    }
    // **A MOVE IS REPORTED WITH BOTH VALUES.** The pointer changing is the
    // event behind every routing question this estate had in August, and
    // "recorded X" alone leaves the reader unable to tell a first run from a
    // relocation.
    intentsvcs::install::Published::Changed { root, from } => {
      println!("changed: install root was {from}");
      println!("         install root is now {}", root.display());
    }
  }

  match &report.config {
    intentsvcs::bootstrap::Config::Kept { path } => {
      println!("ok: configuration already exists -- {}", path.display());
      println!("  use --force to recreate it");
    }
    intentsvcs::bootstrap::Config::Created { path, author } => {
      println!("created: {}", path.display());
      report_author(author.as_deref());
    }
    intentsvcs::bootstrap::Config::Replaced { path, author } => {
      println!("created: {} (replaced)", path.display());
      report_author(author.as_deref());
    }
  }

  // **NO `export INTENT_HOME` / PATH BLOCK, WHICH v2 PRINTS AND v3 MUST NOT.**
  // `install.rs` reads no environment at all; that advice would teach a model
  // of the tool that is wrong, and the pointer just written is what replaced
  // it. **AND NO `intent doctor` RUN**, for the reason hv rejected `doctor` as
  // this caller, read from the other end.
  println!("done: this machine is set up");
  Ok(())
}

/// **An unset author is REPORTED, not passed over in silence.**
///
/// A setup command that records no identity and says nothing leaves the
/// operator to discover it when `init` writes `unknown` into a project weeks
/// later.
fn report_author(author: Option<&str>) {
  match author {
    Some(a) => println!("  author: {a}"),
    None => println!("  author is unset -- $USER names nobody in this environment"),
  }
}

fn export(a: &ArgMatches) -> Result<(), Failure> {
  // `mut` because `md` REALISES rather than emits (AC-06.3): its artefact is a
  // directory tree, and writing one mints a database stamp. The binding says
  // out loud that `export` stopped being a pure read for every format.
  let mut f = open()?;
  // `None` when the flag is absent, which the facade reads as the roster's
  // declared default. Not defaulted here: the default is a fact about the
  // format roster, and a copy of it in the renderer is a second place for it
  // to be wrong.
  match f.export(opt(a, "format").as_deref()).map_err(fail)? {
    Exported::Document(text) => print!("{text}"),
    // **THE DENOMINATOR IS PRINTED, NOT THE COUNT (AC-06.1).** A partial
    // realisation that reads as complete is worse than no realisation, and a
    // bare "wrote 41 files" cannot be wrong out loud. Both numbers come from
    // genuinely different places -- one accumulated while writing, one derived
    // from canon -- so they can disagree, which is the whole point.
    Exported::Realised(r) => {
      let (c, t) = (&r.counts, &r.totals);
      println!(
        "realised: {} file(s) under {}",
        r.written.len(),
        r.root.display()
      );
      println!(
        "  threads {}/{}  wps {}/{}  issues {}/{}  attachments {}/{}  views {}/{}",
        c.threads,
        t.threads,
        c.wps,
        t.wps,
        c.issues,
        t.issues,
        c.attachments,
        t.attachments,
        c.views,
        t.views
      );
      // SAID OUT LOUD IN BOTH DIRECTIONS. A silent success and a silent
      // shortfall are indistinguishable to a reader, which is the failure this
      // estate spent 2026-08-20 removing from four separate instruments.
      if r.complete() {
        println!("  complete: every artefact canon holds reached the realisation");
      } else {
        println!("  INCOMPLETE: canon holds artefacts this realisation did not write");
      }
      // NOT AN ARTEFACT ON STDOUT, and the operator is told why rather than
      // left to notice. `intent export` means "the artefact on stdout" for
      // every format but this one; a tree cannot be one, and saying so here
      // costs a line and answers the question the absence would raise.
      println!(
        "  markdown is the generated VIEW and is never read back -- for data a program will read, use `--format json`"
      );
    }
  }
  Ok(())
}

/// **Flags the table declares and the surface deliberately withholds** --
/// AC-06.8's "absence must not be silent" half.
///
/// The ruling is that a `pending` flag does not refuse the build and does not
/// ship, and that `doctor` reports the count so the quiet-absence hazard is
/// answered somewhere. **The first half worked and the second did not exist**,
/// and ic's diagnosis of why is the reason this lives here rather than beside
/// the rest of `doctor`: the mitigation was ruled into `intentsvcs::doctor`,
/// which **structurally cannot perform it**. `intentsvcs` does not depend on
/// `intent-cli`, and the table is `include_str!`'d into this crate, so the
/// facade cannot see the data the finding is about. Making it able to would
/// invert the layering to satisfy a report.
///
/// So the surface half of the report is composed in the surface's own crate and
/// printed beside the facade's findings, which is the shape that needs no new
/// dependency in either direction.
///
/// **It NAMES them rather than counting them.** A count tells a user that
/// something is missing and not which thing they just failed to run, and the
/// hazard being answered is specifically that three of these four WORK IN v2 --
/// a user who read v2's help and typed `doctor --verbose` gets `unexpected
/// argument` with no hint that the flag is deliberately withheld pending a
/// decision (ic, measured). Naming costs one line each and answers the actual
/// question.
///
/// These are NOT findings: they do not count toward the total and they do not
/// make `doctor` exit nonzero. A deliberate, ratified withholding is not a
/// defect in the estate, and reporting it as one would train a reader to
/// ignore the number that matters.
/// **THE TABLE IS A PARAMETER SO THE DISCRIMINATION CAN BE TESTED WITHOUT A
/// LIVE DEFECT** (vc ruling, 2026-08-20). It read `dispatch::table()` itself,
/// which made the compiled-in table the only possible input -- so the only way
/// to prove this function NAMES a withheld flag was for the estate to be
/// carrying one. D55 resolved the last of them, the withheld population reached
/// zero, and `dispatch_ssot.rs`'s discrimination check panicked at `0 withheld
/// and 75 shipped`: **an instrument that borrows a live instance of the defect
/// has made the defect a fixture, and the estate is then not free to fix it.**
///
/// The ruling that generalises: **an instrument's discrimination is a property
/// of the INSTRUMENT, never of the estate's current defect count.** Taking the
/// table by reference costs one argument and makes a synthetic instance
/// constructible, which is the whole of the fix.
pub fn withheld_flags(table: &dispatch::Table) -> Vec<String> {
  let mut out = Vec::new();
  for entry in dispatch::shipped_entries(table) {
    for flag in &entry.flags {
      if flag.disposition == "pending" {
        out.push(format!(
          "surface: `{}` withholds {} pending a decision on whether it ships -- it is declared and deliberately not built",
          entry.path,
          flag.spellings.join(" / ")
        ));
      }
    }
  }
  out
}

/// AC-06.5: print the generated schema faces.
///
/// It does NOT call `open()`. The faces are rendered from types compiled into
/// this binary, so they are the same everywhere and asking for a project would
/// make the command fail in the one place it is most useful -- outside a
/// project, when you are deciding what a project should contain.
fn schema(m: &ArgMatches) -> Result<(), Failure> {
  // **`--versions` selects the OUTPUT MODE and `face` selects WHICH faces**
  // (ic, declared with the row rather than left to be inferred). They compose:
  // neither arm special-cases the other, so `intent schema ddl.sql --versions`
  // is one face's versions and `intent schema --versions` is all of them. An
  // undeclared composition is how two authors arrive at two answers.
  let versions = flag(m, "versions");
  match m.try_get_one::<String>("face") {
    Ok(Some(name)) => match intentsvcs::faces::face(name) {
      Some(content) => {
        if versions {
          print_versions(Some(name));
        } else {
          print!("{content}");
        }
        Ok(())
      }
      None => Err(
        format!(
          "error: no schema face named `{name}`\n  remedy: one of: {}",
          intentsvcs::faces::face_names().join(", ")
        )
        .into(),
      ),
    },
    Ok(None) => {
      if versions {
        print_versions(None);
      } else {
        print!("{}", intentsvcs::faces::all_faces_banner());
      }
      Ok(())
    }
    Err(e) => Err(format!(
      "error: the CLI asked for an argument `face` that the dispatch table does not declare\n  caused by: {e}\n  remedy: this is a build defect -- the renderer and surface/dispatch-table.json disagree"
    )
    .into()),
  }
}

/// One line per face: which build produced it, and which contract it is.
///
/// **Both parts, always, because they answer different questions** -- the tool
/// version says which build wrote the artefact and moves on every release; the
/// contract version says whether a consumer needs to regenerate their client
/// and must not move on a patch. Printing only one of them would leave the
/// reader unable to tell a rebuild from a change, which is the distinction the
/// two-part scheme exists for.
///
/// The keys are the faces' own spellings, so a line here and a line inside the
/// artefact can be correlated without a translation table.
fn print_versions(only: Option<&str>) {
  let rows: Vec<_> = intentsvcs::faces::versions()
    .into_iter()
    .filter(|(name, ..)| only.is_none_or(|want| *name == want))
    .collect();
  let width = rows.iter().map(|(n, ..)| n.len()).max().unwrap_or(0);
  for (name, tool, key, contract) in rows {
    println!("{name:<width$}  INTENT_VER={tool}  {key}={contract}");
  }
}

/// `ST0000` or `ST0000/03`.
fn scope_of(target: &str) -> (String, Scope) {
  match target.split_once('/') {
    Some((st, wp)) => match wp.parse::<u32>() {
      Ok(seq) => (st.to_string(), Scope::WorkPackage(seq)),
      Err(_) => (target.to_string(), Scope::Thread),
    },
    None => (target.to_string(), Scope::Thread),
  }
}

fn wp_target(a: &ArgMatches) -> Result<(String, u32), Failure> {
  let target = thread_arg(a, "specifier")?;
  match scope_of(&target) {
    (st, Scope::WorkPackage(seq)) => Ok((st, seq)),
    _ => Err(Failure::Error(format!(
      // The worked example is `ST0000` deliberately (D37): it is the STZero
      // retrofit id, so it names something in the READER's own project rather
      // than a thread in ours.
      "error: `{target}` is not a work package\n  remedy: name it as `<ST id>/<NN>`, eg ST0000/03"
    ))),
  }
}

/// An operator's spelling of a T-shirt size.
///
/// **`values` on a positional in the dispatch table is a DECLARATION, not a
/// constraint, so THIS is the enforcement the row is declaring and there is no
/// other** (`arg_values_note`). `spine.rs` reads `values` in exactly two places
/// -- expanding a `kind: subcommand` slot, and resolving a default -- and builds
/// no `value_parser` from a positional's, so the six sizes on the `wp rescope`
/// row reach clap as documentation. The way that becomes a defect is not a
/// decision anyone takes: an author writes the array assuming clap has it, an
/// implementer reads the row assuming the same, and nobody enforces it. That
/// nearly happened here.
///
/// **The refusal names the permitted set, and generates it from the enum.** A
/// bare parse error blames the spelling without saying what the spellings are,
/// which spends the reader's next move on guessing; and a hand-written list of
/// six would be a seventh copy of the vocabulary going stale in a string.
fn t_shirt(raw: &str) -> Result<TShirt, Failure> {
  TShirt::parse(raw).ok_or_else(|| {
    Failure::Error(format!(
      "error: `{raw}` is not a T-shirt size\n  remedy: one of: {}",
      TShirt::spellings()
    ))
  })
}

/// `intent plugin` -- what this INSTALL ships, and what each plugin declares.
///
/// Bare `plugin` IS `plugin list`, which is v2's observed behaviour rather than
/// a convenience added here. It answers OUTSIDE a project too, and that is the
/// contract rather than an oversight: a plugin is a property of the install, so
/// there is no project for the question to be about.
fn plugin(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    None | Some(("list", _)) => plugin_list(),
    Some(("show", sm)) => plugin_show(sm),
    Some((verb, _)) => unwired("plugin", verb),
  }
}

/// The install root every plugin path is resolved against.
///
/// Never `$INTENT_HOME`. A v2 value left in the environment would point this
/// binary at v2's manifests and the output would look completely ordinary --
/// the same rule `rules.rs` and `skills.rs` are built on (AC-11.3).
fn plugin_install() -> Result<std::path::PathBuf, Failure> {
  intentsvcs::install::home().map_err(|e| Failure::Error(e.render()))
}

fn plugin_error(e: intentsvcs::plugins::PluginError) -> Failure {
  Failure::Error(e.render())
}

fn plugin_list() -> Result<(), Failure> {
  let install = plugin_install()?;
  let plugins = intentsvcs::plugins::discover(&install).map_err(plugin_error)?;

  println!("Intent Plugins");
  println!();
  for p in &plugins {
    println!("  {}", p.name);
    println!("    {}", p.description);
    println!();
    for c in &p.commands {
      // `{:<40}` is v2's `printf "    %-40s %s\n"`, read off `bin/intent_plugin`
      // rather than inferred from its rendered output -- a column measured by
      // counting spaces in a terminal is a guess that happens to be right.
      println!("    {:<40} {}", c.syntax, c.description);
    }
    println!();
  }
  if plugins.is_empty() {
    println!("  No plugins found.");
    println!();
  }
  println!("Run 'intent plugin show <name>' for detailed plugin information.");
  Ok(())
}

fn plugin_show(m: &ArgMatches) -> Result<(), Failure> {
  let name = match opt(m, "name") {
    Some(name) => name,
    None => {
      return Err(Failure::Error(
        "error: plugin show: missing plugin name".to_string(),
      ));
    }
  };
  let install = plugin_install()?;
  let Some(p) = intentsvcs::plugins::find(&install, &name).map_err(plugin_error)? else {
    return Err(Failure::Error(format!(
      "error: unknown plugin '{name}'\n  remedy: run `intent plugin list` to see the plugins this build ships"
    )));
  };

  println!("Plugin: {}", p.name);
  println!("  Version:     {}", p.version);
  println!("  Description: {}", p.description);
  println!("  Location:    {}", p.root.display());
  println!();
  println!("Commands ({}):", p.commands.len());
  println!();
  for c in &p.commands {
    println!("  {}", c.syntax);
    println!("    {}", c.description);
    println!();
  }
  // **v2's closing line is NOT ported, and dropping it is the point.** It reads
  // `Run 'intent help <name>' for full command documentation.` -- and `intent
  // help` is RETIRED in v3, refusing at exit 2 with "there is no v3
  // replacement". Porting it faithfully would ship a remedy naming a verb this
  // binary answers by refusing, which is AC-06.11's class arriving through
  // `as-observed` fidelity rather than through carelessness. There is no v3
  // verb that renders a plugin's full documentation, so nothing replaces it and
  // no line is printed: an absent pointer beats a pointer to a refusal.
  Ok(())
}

/// `intent lang` -- the declared-languages family.
///
/// **THE FAMILY IS MIXED, WHICH IS WHY IT TAKES NO FAMILY-LEVEL EXEMPTION FROM
/// THE PROJECT GATE THE WAY `plugin` DOES.** `list` and `show` answer from the
/// tool's own registry and are correct outside a project; `init` and `remove`
/// write `intent/.config/config.json` and must refuse outside one (INV-03).
/// The exemption `plugin` legitimately takes is family-wide, and taking it here
/// would exempt precisely the two verbs that mutate.
fn lang(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    None => lang_usage(),
    Some(("list", _)) => lang_list(),
    Some(("show", sm)) => lang_show(sm),
    Some(("init", sm)) => lang_declare(sm),
    Some(("remove", sm)) => lang_undeclare(sm),
    Some((verb, _)) => unwired("lang", verb),
  }
}

/// Bare `intent lang` prints usage and exits 0, as v2 does.
///
/// It asks the built `Command` for the text rather than composing a block, for
/// the reason `version` does: a usage string written here is a second home for
/// the help the table already owns, and the two would be equal exactly until
/// somebody added a verb.
fn lang_usage() -> Result<(), Failure> {
  let mut root = crate::spine::build(&dispatch::table());
  // **`build()` FIRST, AND IT IS NOT A TIDY-UP.** An unbuilt subcommand has not
  // inherited the root's settings or its display name, so rendering it produces
  // `Usage: lang` and advertises a `help` subcommand the root disabled -- a verb
  // that exits 1 when a reader takes the advice. Measured: without this call,
  // bare `intent lang` and `intent lang --help` differ by two lines.
  //
  // That is the `version` defect exactly -- one capability, two spellings,
  // disagreeing bytes -- and it was introduced HERE, in the commit after the one
  // that fixed it, by rendering from the authority rather than asking it.
  root.build();
  match root.find_subcommand_mut("lang") {
    Some(cmd) => {
      print!("{}", cmd.render_help());
      Ok(())
    }
    // Unreachable while `lang` ships, and NOT an `unwrap`: the table is data,
    // and the one thing a renderer must never do with absent data is panic in
    // front of a user who can fix it.
    None => Err(Failure::Error(
      "error: `lang` is missing from the dispatch table".to_string(),
    )),
  }
}

/// `intent lang list` -- the languages a project may declare.
///
/// **BYTE-IDENTICAL TO v2 TODAY AND DERIVED FROM A DIFFERENT PLACE.** v2 answers
/// by listing directories under `intent/plugins/agents/templates/`; v3 answers
/// from [`intentsvcs::rules::declarable`]. The seven names agree, and they agree
/// for a reason rather than by luck -- the template set was the declarable set
/// minus the two packs nobody declares.
///
/// The change is that `intent lang init` no longer installs a template, so the
/// template directory has stopped being the thing the question is about.
/// Enumerating it would still print the right seven names, which is the failure
/// this estate has spent a day naming: a correct value about the wrong subject,
/// carrying nothing that says so.
///
/// v2's "no language templates available at <dir>" branch is deliberately not
/// ported. It fires when the template directory is missing; the derived list is
/// a compile-time constant and cannot be empty, so porting the branch would ship
/// a message no input can produce.
fn lang_list() -> Result<(), Failure> {
  println!("Available language packs:");
  for lang in intentsvcs::rules::declarable() {
    println!("  {lang}");
  }
  Ok(())
}

/// `intent lang show <lang>` -- what declaring a language does.
///
/// **`corrected`, and the correction is the whole of it.** v2 prints "Files
/// installed by 'intent lang init <lang>'" over a list of two paths. v3's `init`
/// installs nothing, so a faithful port would print a durable claim that two
/// files will appear, and they will not -- the same shape as `plugin show`
/// pointing at a retired verb, arriving through fidelity rather than neglect.
fn lang_show(m: &ArgMatches) -> Result<(), Failure> {
  let lang = lang_one(m)?;
  if !intentsvcs::rules::is_declarable(&lang) {
    return Err(Failure::Error(unknown_language(&lang)));
  }
  println!("Language: {lang}");
  println!();
  println!("`intent lang init {lang}` declares the language in");
  println!("intent/.config/config.json and installs nothing into the project.");
  println!();
  println!("Its rules are served by the installed Intent tool:");
  println!("  intent claude rules list --lang {lang}");
  println!("  intent claude rules show <id>");
  Ok(())
}

/// The project a `lang` write may touch -- **discovered AND past the migration
/// gate**, which are two questions and were answered as one.
///
/// **`context()` IS NOT THE GATE, AND THE FIRST VERSION OF THIS FAMILY USED IT.**
/// It discovers a project and stops; the migration check lives in
/// `Facade::open`, and `lang init` / `lang remove` do not need a facade, so they
/// reached for the lighter call and skipped the gate with it. Measured on a
/// fixture: `intent lang init rust` in an UNMIGRATED v2 project exited 0 and
/// rewrote its `config.json` into v3 shape -- adding `author`, `intent_dir` and
/// the `todo` block while leaving `intent_version: 2.19.0` in place. A v3 binary
/// half-migrating a v2 project, silently, from a command about languages.
///
/// It refuses through `FacadeError::Unmigrated` rather than composing a message,
/// so the operator meets the same wording and the same remedy here as from every
/// other refusal -- one home for what "this project has not been migrated" says.
/// A project, refused unless it has been migrated.
///
/// **`context()` DISCOVERS A PROJECT AND STOPS; THE MIGRATION GATE LIVES IN
/// `Facade::open`, WHICH A COMMAND THAT NEEDS NO STORE NEVER CALLS.** Wired
/// without this, `intent lang init rust` in an unmigrated v2 project exited 0
/// and rewrote its `config.json` into v3 shape with `intent_version: 2.19.0`
/// left in place. `unmigrated_surface.rs` caught it.
///
/// **RENAMED FROM `lang_project` WHEN `modules` BECAME THE SECOND CALLER.** The
/// two have nothing to do with each other except this, and a helper named for
/// its first caller is how the second one gets a copy instead
/// (IN-AG-HIGHLANDER-001).
fn migrated_project() -> Result<Project, Failure> {
  let (project, _) = context()?;
  match project.migration() {
    intentsvcs::project::Migration::Done => Ok(project),
    intentsvcs::project::Migration::Pending(pending) => {
      Err(fail(intentsvcs::facade::FacadeError::Unmigrated(pending)))
    }
  }
}

/// `intent lang init <lang>...` -- declare one or more languages.
///
/// Named `lang_declare` rather than `lang_init` because that is what it now
/// does, and because `init` as a Rust identifier next to the crate's other
/// `init` reads as project initialisation.
fn lang_declare(m: &ArgMatches) -> Result<(), Failure> {
  let langs = lang_many(m)?;
  let project = migrated_project()?;
  let mut config = project.config().clone();

  let mut failed = 0usize;
  let mut declared = 0usize;
  for lang in &langs {
    if !intentsvcs::rules::is_declarable(lang) {
      eprintln!("{}", unknown_language(lang));
      failed += 1;
      continue;
    }
    if config.declare_language(lang) {
      println!("declared: {lang}");
    } else {
      println!("ok: {lang} already declared (no change)");
    }
    declared += 1;
  }

  // **WRITTEN ONCE, AFTER THE LOOP, AND ONLY IF SOMETHING CHANGED.** A write per
  // language would rewrite the file N times for one command and leave a partial
  // declaration behind if the third name were rejected mid-loop.
  intentsvcs::project::write_config(project.root(), &config)
    .map_err(|e| Failure::Error(e.render()))?;

  println!();
  println!("Summary: {declared} language(s) declared; {failed} error(s).");
  if failed > 0 {
    return Err(Failure::Verdict);
  }
  Ok(())
}

/// `intent lang remove <lang>...` -- undeclare one or more languages.
///
/// **IT DOES NOT REFUSE AN UNDECLARABLE NAME, AND `init` DOES.** Removing a name
/// that could never have been declared is already the no-op it prints, whereas
/// declaring one would write a value the rest of the tool cannot serve. The
/// asymmetry is deliberate: a remove that refuses is a remove that cannot clean
/// up after a rename.
fn lang_undeclare(m: &ArgMatches) -> Result<(), Failure> {
  let langs = lang_many(m)?;
  let project = migrated_project()?;
  let mut config = project.config().clone();

  for lang in &langs {
    if config.undeclare_language(lang) {
      println!("removed: '{lang}' from intent/.config/config.json languages");
    } else {
      println!("noop: '{lang}' not declared");
    }
  }

  intentsvcs::project::write_config(project.root(), &config)
    .map_err(|e| Failure::Error(e.render()))?;

  println!();
  println!("Summary: {} language(s) processed.", langs.len());
  Ok(())
}

/// The one wording for an unknown language, so `init` and `show` cannot describe
/// the same rejection two ways.
fn unknown_language(lang: &str) -> String {
  format!(
    "error: '{lang}' is not a language this build can serve\n  remedy: run `intent lang list` for the languages you can declare ({})",
    intentsvcs::rules::declarable().join(" ")
  )
}

/// The single-value positional, by the name the table gives it.
fn lang_one(m: &ArgMatches) -> Result<String, Failure> {
  m.get_one::<String>("lang")
    .cloned()
    .ok_or_else(|| Failure::Error("error: intent lang show: missing language argument".to_string()))
}

/// The repeated positional. clap enforces the `1..n` minimum, so an empty list
/// here means the table and the parser disagree rather than that the user typed
/// nothing -- which is worth saying differently from a usage error.
fn lang_many(m: &ArgMatches) -> Result<Vec<String>, Failure> {
  let langs: Vec<String> = m
    .get_many::<String>("lang")
    .map(|v| v.cloned().collect())
    .unwrap_or_default();
  if langs.is_empty() {
    return Err(Failure::Error(
      "error: intent lang: missing language argument(s)".to_string(),
    ));
  }
  Ok(langs)
}

/// `intent modules` -- the module-registry family.
fn modules(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    None => modules_usage(),
    Some(("find", sm)) => modules_find(sm),
    Some(("check", _)) => modules_check(),
    Some((verb, _)) => unwired("modules", verb),
  }
}

/// Bare `intent modules` prints usage and exits 0, as v2 does.
///
/// `build()` before `find_subcommand_mut` for the reason `lang_usage` records:
/// an unbuilt subcommand renders `Usage: modules` and advertises a `help` verb
/// the root disabled. One capability, two spellings, disagreeing bytes.
fn modules_usage() -> Result<(), Failure> {
  let mut root = crate::spine::build(&dispatch::table());
  root.build();
  match root.find_subcommand_mut("modules") {
    Some(cmd) => {
      print!("{}", cmd.render_help());
      Ok(())
    }
    None => Err(Failure::Error(
      "error: `modules` is missing from the dispatch table".to_string(),
    )),
  }
}

/// `intent modules find <term>` -- search the registry.
///
/// **NO MATCH EXITS 1, AND THE TABLE SAID 0.** Driven against the frozen v2
/// install on a fixture: a matching term is rc=0, a non-matching term prints
/// `no matches for '<term>'` on stdout at **rc=1**. `bin/intent_modules`'
/// `cmd_find` returns 1 on the empty branch and always has. The table's
/// `observed` block carried "found, or no match -> 0" with an evidence class of
/// `read` -- taken from the source without executing that arm, and read wrong.
/// Issue 0067 records the same rc=0 as measured. **Two documents agreed with
/// each other and neither agreed with the program**, which is why this is
/// as-observed rather than a correction: grep's convention is what v2 already
/// had.
///
/// The message goes to STDOUT and the failure is silent, so the rc carries the
/// verdict without a second line on stderr contradicting the first on stdout.
fn modules_find(m: &ArgMatches) -> Result<(), Failure> {
  let term = m.get_one::<String>("term").cloned().unwrap_or_default();
  let project = migrated_project()?;
  let text =
    intentsvcs::modules::read_registry(project.root()).map_err(|e| Failure::Error(e.render()))?;
  let rows = intentsvcs::modules::find_rows(&text, &term);
  if rows.is_empty() {
    println!("no matches for '{term}'");
    return Err(Failure::Verdict);
  }
  for row in rows {
    println!("{row}");
  }
  Ok(())
}

/// `intent modules check` -- compare the registry against the filesystem.
///
/// **THE POPULATION IS DERIVED FROM THE DECLARED `languages`, WHICH IS THE ONE
/// DELIBERATE DEVIATION.** See [`intentsvcs::modules`] for why v2's
/// `bin/intent_*` scan was never general.
///
/// **AND THE SCANNED LINE IS PRINTED BEFORE THE VERDICT, NOT AFTER IT.** `ok:
/// registry matches filesystem` over an empty population is the estate's
/// recurring defect in one line -- a check that could not fire, reading exactly
/// like one that passed. A reader needs the denominator before the count, so it
/// leads.
fn modules_check() -> Result<(), Failure> {
  let project = migrated_project()?;
  let languages = project.config().languages.clone();
  let report = intentsvcs::modules::check(project.root(), &languages)
    .map_err(|e| Failure::Error(e.render()))?;

  if report.scanned_anything() {
    let parts: Vec<String> = report
      .scanned
      .iter()
      .filter(|(_, n)| *n > 0)
      .map(|(lang, n)| format!("{lang} ({n})"))
      .collect();
    println!("note: scanned {}", parts.join(", "));
  } else {
    println!(
      "warning: no declared language contributes a source population, so nothing was compared against the registry"
    );
    println!("  remedy: declare what this project is written in with `intent lang init <lang>`");
  }
  println!();

  if !report.unregistered.is_empty() {
    println!("warning: unregistered files");
    for f in &report.unregistered {
      println!("  + {f}");
    }
    println!();
  }
  if !report.stale.is_empty() {
    println!("warning: stale registry entries");
    for f in &report.stale {
      println!("  - {f}");
    }
    println!();
  }

  if report.clean() {
    println!("ok: registry matches filesystem");
    return Ok(());
  }
  let issues = report.unregistered.len() + report.stale.len();
  println!("error: {issues} issue(s) found");
  Err(Failure::Verdict)
}

/// `intent version` -- the subcommand twin of `--version`.
///
/// **IT ASKS CLAP FOR THE STRING RATHER THAN COMPOSING ONE, AND THAT IS THE
/// WHOLE DESIGN.** The criterion this closes is not "the subcommand exists"
/// but "one capability does not disagree with itself about whether it
/// exists": `--version` answered rc=0 with a version while `version` refused
/// rc=2 as unimplemented, in one binary. Composing a second line here would
/// close the rc gap and leave a BYTES gap behind it, which is the same defect
/// one level down and harder to see -- both spellings would answer, and
/// nothing would say they answered differently.
///
/// `render_version()` is the exact string `spine::parse` prints for the
/// `DisplayVersion` arm, so the two are one value with two call sites rather
/// than two values that a test hopes are equal. The test is still written,
/// because a shared SOURCE is not a shared OUTPUT -- a future `print!` that
/// trimmed or decorated this would pass the type checker.
///
/// Rebuilding the Command costs a table parse on a command that prints one
/// line. That is deliberate: the alternative is threading the built Command
/// through `run`, which widens a signature every arm shares for the benefit of
/// one, and the honest cost of asking the authority is smaller than the cost
/// of keeping a copy near the caller.
fn version() -> Result<(), Failure> {
  // `print!`, not `println!`: `render_version()` already ends in a newline, and
  // the byte-identity property is asserted against `--version`, so an added
  // newline here would break the criterion rather than tidy the output.
  print!(
    "{}",
    crate::spine::build(&dispatch::table()).render_version()
  );
  Ok(())
}

/// `intent info` -- the installation and project overview.
///
/// **THIS COMMAND NEVER GATES ON PROJECT STATE, and that is the whole of issue
/// 0042. It is not the same claim as "it never gates", which is what this
/// comment used to say and what shipped as a defect the same day.** The
/// shipped pre-commit gate resolves the Intent install by parsing this
/// command's `INTENT_HOME:` line back out of its stdout, and it does so in
/// projects that are not migrated, half-migrated, or not projects at all. A
/// version that refused outside v3 canon would resolve to the empty string in
/// exactly the situations the guards exist for -- which is what the
/// unimplemented command was already doing, silently, by exiting 2 with no
/// stdout at all. Both whiteboard guards stopped enforcing and neither said so.
///
/// **The over-general form of that rule cost an exit code.** Written as "never
/// gates", it licensed returning `Ok(())` after failing to resolve the install
/// -- so the command printed `<not set>`, named the reason on stderr, and told
/// its caller everything was fine. dc found it by running a published-layout
/// build, and the framing is theirs: **0044 is `1` meaning five things; this
/// was `0` meaning "I could not do the thing you asked"** -- the worse half,
/// because a wrong non-zero code stops a caller for the wrong reason and a zero
/// on failure stops nothing at all. vc records that their own 0044 sweep was
/// structurally blind to it: a table that classifies failures BY exit code puts
/// a failure returning `0` in the success row by construction.
///
/// **The split this function now holds: an unmigrated project is not a failure
/// of `info`; an unresolvable install is.** Project state degrades to what can
/// honestly be printed. Install resolution is the tool's own footing -- if it
/// is gone, the guards the gate builds from this output genuinely cannot run,
/// and saying so is the only useful answer.
///
/// The code is `Failure::Error` (1) rather than `Unavailable` (2) deliberately:
/// consumers read 2 as fail-open -- which is exactly wrong here -- and 0044 may
/// reclassify the whole surface, so this takes the code that makes a caller
/// stop rather than inventing a fourth meaning ahead of that decision.
///
/// So the Installation block is unconditional, it is printed FIRST, and the
/// project half degrades to what can honestly be said rather than taking the
/// whole command down with it. v2's own probe row records the same posture as
/// observed behaviour: exit 0 outside a project.
///
/// **A hook resolving a path by parsing a display command is still wrong**, and
/// this does not make it right -- 0016 forbids rewiring the consumer's hooks
/// for the v3 swap, so the display command is what exists to be parsed today.
/// The line's shape is therefore load-bearing, not cosmetic, and
/// `info_line_is_parseable_by_the_pre_commit_gate` pins it against the gate's
/// own `sed` expression rather than against a description of it.
fn info() -> Result<(), Failure> {
  println!("Intent: The Steel Thread Process");
  println!();
  println!("Installation:");

  // **Both facts are gathered BEFORE anything is printed and neither is acted
  // on until the bottom**, so that the exit decision cannot be made by whichever
  // branch happens to reach a `return` first. The previous shape returned
  // `Ok(())` from the middle of the project block, which meant a bottom-of-
  // function check would have been correct for a project and unreachable
  // outside one -- the same "enforced on one of two writers, so enforced on
  // neither reliably" shape that D44's window hit in `views::render_all`.
  let install = intentsvcs::install::home();
  let cwd = std::env::current_dir();

  // A broken install is NAMED on stderr and still prints v2's `<not set>` on
  // stdout. The token is v2's, so a consumer parsing this line sees a value it
  // already handles rather than a missing line it has no branch for -- and the
  // gate's fail-open message then quotes `<not set>` back at the operator
  // inside the path it tried, which says what went wrong at the point they can
  // act on it. Silence on both streams is what issue 0042 was. **Printing it is
  // not the same as succeeding at it**, which is what this function got wrong.
  println!(
    "  INTENT_HOME:     {}",
    match &install {
      Ok(home) => home.display().to_string(),
      Err(_) => "<not set>".to_string(),
    }
  );
  println!("  Version:         {}", env!("CARGO_PKG_VERSION"));
  // v2 printed `which intent`, which answers "what would run" rather than
  // "what IS running" -- and those differ precisely during a v3 rollout, where
  // a brew binary shadows a v2 symlink (issue 0036). The running executable is
  // the one fact this process can state without guessing.
  match std::env::current_exe() {
    Ok(exe) => println!("  Executable:      {}", exe.display()),
    Err(e) => println!("  Executable:      <unknown: {e}>"),
  }
  println!();

  info_project(cwd.as_deref().ok());

  // **THE ONE EXIT DECISION, and it is the last thing this function does.**
  // Nothing above it returns, so every printable line is printed on every path
  // -- including the `INTENT_HOME:` line the pre-commit gate parses, which has
  // a stdout contract and no exit-code contract at all.
  match (install, cwd) {
    (Err(e), _) => Err(Failure::Error(format!("error: {e}"))),
    (_, Err(e)) => Err(Failure::Error(format!(
      "error: cannot read the working directory: {e}"
    ))),
    _ => Ok(()),
  }
}

/// The project half of `intent info`. **Returns nothing, on purpose: no project
/// state may reach the exit code** (issue 0042), so this half has no way to
/// report a failure and no need of one.
///
/// `None` is a working directory that could not be read -- an environment
/// failure rather than a statement about the project, so it is not reported as
/// "not in a project", which would be a confident answer from no evidence.
fn info_project(cwd: Option<&std::path::Path>) {
  println!("Project:");

  let Some(cwd) = cwd else {
    println!("  cannot be determined -- the working directory is unreadable");
    println!();
    return;
  };

  let project = match Project::discover(cwd) {
    Ok(project) => project,
    Err(_) => {
      println!("  Not in an Intent project directory");
      println!();
      println!("To create a new project:  intent init");
      println!("To see available commands: intent help");
      println!();
      return;
    }
  };

  let config = project.config().clone();
  println!("  Location:        {}", project.root().display());
  println!("  Name:            {}", or_unknown(&config.project_name));
  println!("  Author:          {}", or_unknown(&config.author));
  println!("  Intent version:  {}", or_unknown(&config.intent_version));
  println!();

  println!("Steel Threads:");
  match project.migration() {
    // **An unmigrated project gets the reason and the remedy, never a zero.**
    // Counting v2 canon here would mean a second thread reader living outside
    // `legacy.rs`, and reporting `Total: 0` for an estate of 56 is the
    // confident-answer-from-partial-evidence bug this thread exists to end
    // (AC-10.7: "no threads found" and "threads this binary cannot see" are
    // the same empty vector).
    intentsvcs::project::Migration::Pending(pending) => {
      println!("  {pending}");
      println!("  remedy: {}", pending.remedy());
    }
    intentsvcs::project::Migration::Done => {
      let ctx = FacadeContext {
        principal: "local".to_string(),
        project_id: config.project_id.clone().unwrap_or_default(),
        version: env!("CARGO_PKG_VERSION").to_string(),
      };
      match Facade::open(project, ctx) {
        Ok(facade) => {
          let threads = facade.st_list();
          let count = |want: ThreadStatus| threads.iter().filter(|t| t.status == want).count();
          // **v2's five buckets, including the one the model can now tell
          // apart and this view still cannot.** v2 bucketed by DIRECTORY, so
          // everything left at `intent/st/` read as In Progress -- `Triage`
          // and `Hold` included. The dispatch table pins this entry
          // `as-observed` with `corrected` ratified for the argument handling
          // ONLY, so the buckets stay v2's and the rollup is recorded here
          // rather than quietly widened. Raised with vc as a view question,
          // not decided in a render function.
          let in_progress =
            count(ThreadStatus::Wip) + count(ThreadStatus::Hold) + count(ThreadStatus::Triage);
          println!("  Total:           {}", threads.len());
          println!("  In Progress:     {in_progress}");
          println!("  Completed:       {}", count(ThreadStatus::Completed));
          println!("  Not Started:     {}", count(ThreadStatus::NotStarted));
          println!("  Cancelled:       {}", count(ThreadStatus::Cancelled));
        }
        Err(e) => {
          // `fail` yields a `Failure` now, and this site wants the TEXT --
          // taken from the same renderer `fail` uses rather than from a second
          // formatting of the error.
          println!("  unavailable: {}", e.render());
        }
      }
    }
  }
  println!();
}

/// `intent issues` -- all six verbs, since hv ratified Machine 4.
///
/// **`add`, `close` and `open` were blocked on a ratification, not on effort,
/// and reported themselves unbuilt for two days.** `transitions.rs` had
/// `Issue.status` as `Disposition::Unbuilt` while `data-model.md` ratified three
/// machines and no issue machine; AC-04.6 requires the implemented graph to
/// match the ratified machines EXACTLY, so wiring `close` and `open` meant
/// declaring `open <-> closed` on my own authority. The edges looked obvious,
/// which is exactly when the discipline is worth keeping. hv ruled Machine 4 on
/// 2026-08-17 and the three are wired here.
///
/// **Every string below is v2's, and that is the `keep` disposition being
/// honoured rather than a coincidence.** The family is `keep` with
/// `target.state: as-observed`, so `bin/intent_issues` is the specification --
/// including its two-line `add` output and its `already CLOSED` self-loop, which
/// is where hv's self-loop ruling took its citation from.
fn issues(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    // The bare form runs `list`, which is the table's declared default verb
    // for this family.
    None | Some(("list", _)) => {
      let a = m.subcommand().map(|(_, a)| a).unwrap_or(m);
      let kind = opt(a, "kind").unwrap_or_else(|| "open".to_string());
      let wanted = match kind.to_ascii_lowercase().as_str() {
        "open" => Some(IssueStatus::Open),
        "closed" => Some(IssueStatus::Closed),
        "all" => None,
        other => {
          return Err(Failure::Error(format!(
            "error: `{other}` is not an issue bucket\n  remedy: use one of open, closed, all"
          )));
        }
      };

      let f = open()?;
      let rows: Vec<Vec<String>> = f
        .issue_list()
        .into_iter()
        .filter(|i| wanted.is_none_or(|w| i.status == w))
        .map(|i| {
          vec![
            format!("{:04}", i.number),
            i.status.display().to_string(),
            // v2 prints `?` for an issue whose severity was never recorded,
            // and the token is kept: a blank cell reads as a rendering fault,
            // where `?` reads as "nobody said".
            i.severity.clone().unwrap_or_else(|| "?".to_string()),
            i.title.clone(),
          ]
        })
        .collect();

      if rows.is_empty() {
        println!("no {kind} issues");
        return Ok(());
      }
      print!("{}", table_out(&output_of(a)?, ISSUE_COLUMNS, &rows)?);
      Ok(())
    }
    Some(("show", a)) => {
      let number = issue_arg(a, "id")?;
      let f = open()?;
      let issue = f.issue_show(number).map_err(fail)?;
      if output_of(a)?.format() == Format::Json {
        println!(
          "{}",
          serde_json::to_string_pretty(issue)
            .map_err(|e| format!("error: the issue could not be rendered as JSON: {e}"))?
        );
      } else {
        println!("{:04}: {}", issue.number, issue.title);
        println!("status: {}", issue.status.display());
        if let Some(sev) = &issue.severity {
          println!("severity: {sev}");
        }
        println!("created: {}", issue.created);
        if let Some(closed) = &issue.closed {
          println!("closed: {closed}");
        }
        // **v2's `cmd_show` cats the whole file** (`bin/intent_issues:270`), so
        // until the body was modelled this command showed strictly LESS than
        // the tool it replaces -- the prose, and `reporter`, both carried and
        // both unreachable. A field nothing can read is a field that is not
        // there, and an issue is mostly its prose.
        if let Some(reporter) = &issue.reporter {
          println!("reporter: {reporter}");
        }
        if !issue.body.is_empty() {
          println!();
          println!("{}", issue.body);
        }
      }
      Ok(())
    }
    Some(("add" | "new", a)) => {
      let title = arg(a, "title")?;
      // **`--severity` has a DEFAULT and it lives in the dispatch table, not
      // here.** v2's flag parsing defaults it to `medium` and the row carries
      // that; the facade takes `None` to mean nobody said, which is the state
      // `issues list` renders as `?`. Reading it through `opt` keeps the default
      // where the surface declares it -- if the table's default is ever removed,
      // the facade records the absence rather than this arm inventing one.
      let severity = opt(a, "severity");
      // **THE ROSTER IS ENFORCED HERE, ON THE WAY IN, AND IT WAS ENFORCED
      // NOWHERE BEFORE.** The table has declared `critical|high|medium|low`
      // all along and `--help` printed it, but nothing parsed it: `--severity
      // bogus` exited 0 and wrote `bogus` into canon, where every later reader
      // had to cope with it.
      //
      // **AT THE DOOR RATHER THAN IN THE TYPE**, so the store keeps reading
      // whatever is already in it -- see [`intentsvcs::model::Issue::severity`]
      // for why a typed field would turn this into an unreadable store on a
      // machine whose canon was written before the check existed.
      //
      // Exit 1 rather than clap's 2: an unknown value is refused in the
      // renderer by the same ruling that governs `--format`, because the
      // pre-commit gate fails open on USAGE.
      if let Some(bad) = severity.as_deref()
        && intentsvcs::model::IssueSeverity::parse(bad).is_none()
      {
        return Err(Failure::Error(format!(
          "error: `{bad}` is not an issue severity\n  remedy: name one of {}",
          intentsvcs::model::IssueSeverity::SPELLINGS.join(", ")
        )));
      }
      let reporter = reporter();
      let body = issue_body(a)?;
      let mut f = open()?;
      let number = f
        .issue_add(&title, severity.as_deref(), reporter.as_deref(), &body)
        .map_err(fail)?;
      // v2 prints TWO lines (`bin/intent_issues:187-188`): the path it wrote,
      // then `<id>:<title>`.
      //
      // **ONE LINE CARRIED TWO DEVIATIONS OF OPPOSITE LEGITIMACY, WHICH MADE IT
      // UNREADABLE AS EITHER** -- issue 0060, ic's finding. The LAYOUT is
      // ratified: v3's issue canon is flat (`intent/issues/<NNNN>.json`,
      // data-model.md), so v3 cannot print v2's bucketed path because v3 does not
      // create it. **The ABSOLUTENESS was forced by nothing.** This printed the
      // fully-qualified path, so the line a user is meant to copy embedded `$HOME`
      // and the working directory -- and on this project that means agent-session
      // tmpdirs landing in issue text.
      //
      // Neither implementation chose a convention: v2 builds its path from
      // `$INTENT_DIR` and is relative by construction, v3 held a resolved root and
      // printed it. **That is what makes this `as-observed` rather than a
      // deviation to ratify -- there is nothing here that was decided and could be
      // defended.**
      //
      // **And the defect made itself unmeasurable**, which is why it was filed
      // rather than noted: `literal_stdout_parity.rs` asserts a row against a
      // literal template, and a template cannot contain a machine's tmpdir. Every
      // other row's coverage question is whether anyone has written the
      // declaration; this row's was whether one could be written at all. Relative
      // now, so it can be.
      println!(
        "created: {}",
        f.project().relative(&f.project().issue_json(number))
      );
      println!("{number:04}:{title}");
      Ok(())
    }
    Some(("close", a)) => {
      let number = issue_arg(a, "id")?;
      reported(
        &open()?.issue_close(number).map_err(fail)?,
        &format!("issue {number:04}"),
        "-> CLOSED",
      );
      Ok(())
    }
    Some(("open", a)) => {
      let number = issue_arg(a, "id")?;
      reported(
        &open()?.issue_open(number).map_err(fail)?,
        &format!("issue {number:04}"),
        "-> OPEN",
      );
      Ok(())
    }
    // **`issues hydrate` AND `issues dehydrate` ARE GONE FROM THE SURFACE, AND
    // THE ANSWER WAS NOT "WIRE THEM LATER"** (hv, 2026-08-20).
    //
    // They were declared and unwired here while the question went up: does an
    // issue have a realised form at all? It does not. **Issues are
    // canon-and-store only**, so both rows are retired in the dispatch table,
    // `ISSUE:` is out of the `.intentfiles` grammar, and `Address::artefact`
    // answers `None` for an issue -- which is what turns the case into a
    // refusal at `Facade::hydrate`'s door instead of a walk into the wrong
    // layer. The full record is on the two retired rows.
    //
    // **The arm below stays and is not vestigial.** It is the family's
    // catch-all for a verb the table declares and this file has not wired, and
    // `cli_write_moves_only_what_changed.rs` drives whatever is in that state.
    // What changed is that no `issues` verb is in it today.
    Some((verb, _)) => unwired("issues", verb),
  }
}

/// **THE ONE PLACE A MUTATING VERB REPORTS ITSELF** -- issue 0050.
///
/// Nineteen arms called the facade as a statement, `open()?.st_done(&id)?;`, which
/// propagates the error and DISCARDS the `Ok` value, then printed the movement
/// message unconditionally. So `intent st done` on a completed thread printed
/// `ok: ST0001 done`, having done nothing, at exit 0 -- while `intent todo done`,
/// which delegates to that same `st_done`, reported the no-op. **The wrapper was
/// honest and the thing it wrapped was not**, which is the wrong way round.
///
/// **The no-op line names the STATE the entity is in, not the verb the caller
/// failed to perform.** v2's only `already` arm is `ok: issue 0050 already
/// CLOSED` (`bin/intent_issues:283`), which is also the arm hv's self-loop ruling
/// cites. `was already done` -- the spelling `todo done` shipped -- coincides with
/// the state only when the verb and the state share a word, and it stops meaning
/// anything on `st hold` (state: `On Hold`) or `st triage` (state: `Not Started`).
/// Naming the state tells a caller what the entity IS; naming the verb tells them
/// what they failed to do, which they already know. ic and vc reached this
/// independently.
///
/// **No third prefix.** INV-01 names `ok:` and `error:`; v2's `skipped: <ID>
/// already in progress` on `st start` is carried as a deviation on that row.
/// Reviving it across nineteen arms to match one v2 verb, against the invariant,
/// is a larger surface change than matching the one v2 form that already complies.
///
/// **The state comes from the FACADE, not from a literal here**, and that is the
/// half worth defending. `ac rescope` and `ac reinstate` land on
/// `AcState::entry(kind)`, so this renderer cannot know their target -- but more
/// than that, seventeen hard-coded state words would be seventeen spellings a
/// rename could not reach, which is issue 0047 rebuilt in a new file.
///
/// `moved` is the movement phrase. `issues` passes `-> CLOSED`, which composes
/// into v2's two lines exactly: `ok: issue 0021 -> CLOSED` and `ok: issue 0021
/// already CLOSED`.
fn reported(outcome: &Outcome, subject: &str, moved: &str) {
  // **THE NOTES ARE PRINTED HERE, AND THAT PLACEMENT IS WHAT MAKES THEM
  // UNDROPPABLE.** Adding an `Outcome` variant would not have forced the
  // nineteen arms to handle it -- they all ask `Outcome::already`, a method,
  // which absorbs a new variant in silence. What actually prevents a dropped
  // note is that every arm reports through this one function, so a verb that
  // grows something to say gets it printed without its arm being touched.
  //
  // **STDERR, AND THE SHAPE IS THE ONE `sync` ALREADY USES** -- `warning:` with
  // the paths indented for a real finding, `note:` for a question that could not
  // be asked. Two prefixes rather than one because "nothing is uncommitted" and
  // "I could not look" are what an operator most needs to tell apart, and INV-01
  // governs the `ok:`/`error:` result line on stdout rather than this.
  for note in outcome.notes() {
    match note {
      Note::UnsyncedAttachments(paths) => {
        eprintln!(
          "warning: {} attachment(s) carry bytes no commit contains, and closing this leaves them for the next `organize` to remove:",
          paths.len()
        );
        for path in paths {
          eprintln!("  {path}");
        }
        eprintln!(
          "  remedy: `intent sync --to-store {subject}` takes the disk copy into the store -- an attachment is authored ON DISK, so a divergence means the store is stale"
        );
      }
      Note::UnsyncedUnknown => eprintln!(
        "note: the index could not be read, so whether this thread's attachments carry uncommitted bytes is UNKNOWN"
      ),
    }
  }
  match outcome.already() {
    None => println!("ok: {subject} {moved}"),
    Some(state) => println!("ok: {subject} already {state}"),
  }
}

/// v2's placeholder for a config field it could not read. Kept because a blank
/// after a padded label reads as a rendering fault rather than as missing data.
fn or_unknown(value: &str) -> &str {
  if value.trim().is_empty() {
    "Unknown"
  } else {
    value
  }
}

/// `intent llm` -- and `guide` is the verb that exists.
///
/// **The renderer is ic's and landed tested (`2a654db3`); this is the arm that
/// makes it reachable.** They had the line in this file and took it back out
/// within the minute rather than leave a verb of theirs inside someone else's
/// uncommitted change, which is the right call and is why it is here instead.
///
/// `guide::render` returns the whole document as a `String` and takes the
/// table: the command reference is generated at render time from
/// `dispatch::shipped_entries`, so there is no committed guide file to go
/// stale. `print!` rather than `println!` -- the document ends with its own
/// newline, and adding a second is a diff in anything that captures it.
fn llm(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("guide", _)) => {
      print!("{}", crate::guide::render(&dispatch::table())?);
      Ok(())
    }
    Some((verb, _)) => unwired("llm", verb),
    None => unwired("llm", ""),
  }
}

fn claude(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("hook", a)) => hook(a),
    Some(("rules", a)) => rules(a),
    Some(("skills", a)) => skills(a),
    Some(("upgrade", a)) => claude_upgrade(a),
    Some(("start", a)) => claude_cwi(a, CwiVerb::Start),
    Some(("ws", a)) => claude_cwi(a, CwiVerb::Ws),
    Some((verb, _)) => unwired("claude", verb),
    None => unwired("claude", ""),
  }
}

/// The two verbs [`claude_cwi`] fronts. An enum rather than a `&str` so the
/// argv assembly below is exhaustive: the two verbs read their positionals
/// from DIFFERENT shapes, and a string would let a third verb be added here
/// with `start`'s shape by accident.
#[derive(Clone, Copy)]
enum CwiVerb {
  Start,
  Ws,
}

/// `intent claude start <ws>` and `intent claude ws <verb> [wsid]` -- the MAAC
/// whiteboard launcher and provisioner.
///
/// **THE SURFACE WAS ALREADY PARSED AND ONLY THE RENDERER WAS MISSING**, the
/// same shape [`skills`] records. `dispatch-table.json` declares both verbs,
/// lists `claude start` under `shipped`, and the spine builds their positionals
/// -- so `intent claude --help` has listed them, `intent claude start` with no
/// argument has correctly said `<WS>` is required, and the verb has answered
/// `2` the whole time. **Wiring this makes the table TRUE rather than newly
/// false**: the entry claiming `shipped` was the thing that was wrong.
///
/// **IT DISPATCHES RATHER THAN PORTS, DELIBERATELY.** See
/// [`intentsvcs::install::cwi_script`] for hv's ruling that this script
/// survives the cut. Measured before wiring: `CWI_DRY_RUN=1 ... start cc` and
/// `... ws list` both exit 0 under a v3 binary, in this project and from
/// another estate's working directory, because the script does its own
/// `find_project_root`. One binary therefore serves every estate.
///
/// **THE VERB IS PASSED THROUGH, NOT CONSUMED.** v2's `bin/intent` carried the
/// same instruction as a comment -- *"Do NOT shift: intent_claude_cwi's own
/// dispatch consumes `start`/`ws`"* -- and it is the sort of thing a reader
/// tidies away, so it is stated here as well as obeyed.
///
/// **KNOWN, AND NOT FIXED BY THIS CHANGE** (vc, 2026-08-27): once these are
/// wired, every estate's `intent claude start` reads its launcher out of ONE
/// project's checkout, because that is where [`intentsvcs::install::home`]
/// resolves for a binary living there. That is a second consumer of the
/// unowned machine-level fact that one project's build tree is on eleven
/// projects' `PATH` -- wiring does not create it, but it does add weight to it,
/// and hv rules on it with the weight visible rather than meeting it later.
fn claude_cwi(m: &ArgMatches, verb: CwiVerb) -> Result<(), Failure> {
  let home = intentsvcs::install::home().map_err(|e| Failure::Error(format!("error: {e}")))?;
  let script = intentsvcs::install::cwi_script(&home);

  let args = match verb {
    CwiVerb::Start => vec!["start".to_string(), arg(m, "ws")?],
    CwiVerb::Ws => match m.subcommand() {
      Some((sub, a)) => {
        let mut v = vec!["ws".to_string(), sub.to_string()];
        // `wsid` is `0..1` in the table -- `ws list` takes none, `ws hygiene`
        // takes an optional one. `opt` rather than `arg`, or the optional verbs
        // refuse on a positional the table says they do not need.
        if let Some(wsid) = opt(a, "wsid") {
          v.push(wsid);
        }
        v
      }
      // **UNREACHABLE FROM THE CLI, and the first comment here said the
      // opposite.** I wrote that `intent claude ws` bare would hand through to
      // the script's own usage; it does not. The table gives `verb` arity `1`,
      // the spine turns that into clap's `subcommand_required`, and clap
      // refuses at `error: 'intent claude ws' requires a subcommand` before
      // this function is entered. Driven, not reasoned about.
      //
      // The arm stays because `subcommand()` is an `Option` and the match must
      // be total; it is written as the passthrough it would be if the arity
      // ever relaxed, rather than an `unreachable!()` that would turn a table
      // edit into a panic.
      None => vec!["ws".to_string()],
    },
  };

  exec_shipped_script(&script, &args, "whiteboard launcher")
}

/// `intent claude upgrade` -- apply v3 canon to an existing project (issue 0077).
///
/// **ITS OWN DESCRIPTION IS THE CUTOVER INSTRUCTION.** The dispatch table calls
/// this verb *"Apply Claude canon to the project"*; hv's 2026-08-26 instruction
/// is *"we just go into each project and REWRITE their Intent usage/config/setup
/// to be INTENTv3 CANONICAL"*. The verb has been listed by `--help` and refusing
/// at rc=2 the whole time, which is the table-says-it-ships / binary-says-no
/// shape `skills` above records.
///
/// **WHY A TOOL RATHER THAN FIFTEEN HAND-EDITS** (vc, 2026-08-26): a hand
/// rewrite delivers one canonical estate today and nothing holds it tomorrow,
/// and fifteen hand-written repositories produce fifteen artefacts nobody can
/// re-derive. A tool produces a state that can be RE-ASSERTED AND COMPARED --
/// run it twice, change nothing.
///
/// **`--apply` IS REQUIRED AND THE DEFAULT IS A DRY RUN**, matching v2. This
/// writes into a project's `.claude/`, its root canon and its pre-commit hook;
/// a verb that does that on a bare invocation is one nobody can explore safely.
fn claude_upgrade(m: &ArgMatches) -> Result<(), Failure> {
  let f = open()?;
  let home = intentsvcs::install::home().map_err(|e| Failure::Error(e.render()))?;
  let ctx = views::RenderContext {
    version: env!("CARGO_PKG_VERSION"),
    // Root files only; `todo.md` is not rendered here.
    todo_watermark: None,
  };
  let root = f.project().root();
  let hooks = intentsvcs::canon::hooks_dir(root);

  if !m.get_flag("apply") {
    println!(
      "canon (dry run): would apply v3 canon to {}",
      root.display()
    );
    for name in [
      ".claude/settings.json",
      "CLAUDE.md",
      "AGENTS.md",
      "usage-rules.md (only if absent)",
      ".intent_critic.yml (only if absent)",
    ] {
      println!("  {name}");
    }
    match &hooks {
      Some(h) => {
        // **BOTH HALVES OF THE GATE ARE NAMED, CARRIER FIRST.** A dry run that
        // listed only the chain block would describe the exact state that block
        // produces on its own: a reference with no referent, silently skipped.
        println!("  {}/pre-commit.intent (the gate shim)", h.display());
        println!("  {}/pre-commit (chain block, region-edited)", h.display());
      }
      // NOT SILENCE. A dry run that omits a step it cannot do reads as a plan
      // that never included it.
      None => println!("  (no git repository -- no gate: neither carrier nor chain block)"),
    }
    println!("re-run with --apply to write.");
    return Ok(());
  }

  let applied = intentsvcs::canon::apply(
    root,
    &home,
    f.project().config(),
    &ctx,
    hooks.as_deref(),
    m.get_flag("force"),
  )
  .map_err(|e| Failure::Error(e.to_string()))?;

  for p in &applied.written {
    println!("written: {}", rel(root, p));
  }
  // **REPORTED, NOT SILENT.** A run that says nothing about a file it examined
  // cannot be told from one that skipped it -- and "already canonical" is the
  // answer the second run of a converger is supposed to give.
  for p in &applied.unchanged {
    println!("unchanged: {}", rel(root, p));
  }
  for p in &applied.preserved {
    println!("preserved: {} (yours, not canon's)", rel(root, p));
  }
  // **HELD IS NOT PRESERVED AND MUST NOT READ AS IT.** Canon owns the template
  // for these and declined to write over a hand-authored copy. Naming the flag
  // is the point: a run that silently skipped the project's most-read file
  // would look identical to one that had nothing to do.
  for p in &applied.held {
    println!(
      "held: {} -- hand-authored, no generated marker; --force overwrites",
      rel(root, p)
    );
  }
  println!(
    "ok: {} written, {} already canonical, {} preserved, {} held.",
    applied.written.len(),
    applied.unchanged.len(),
    applied.preserved.len(),
    applied.held.len()
  );
  Ok(())
}

/// A path relative to the project root, for reporting. Absolute paths in a
/// per-file list make the list unreadable and say nothing the header did not.
fn rel(root: &std::path::Path, p: &std::path::Path) -> String {
  p.strip_prefix(root).unwrap_or(p).display().to_string()
}

/// `intent claude skills` -- AC-07.3.
///
/// **THE SURFACE WAS ALREADY PARSED AND ONLY THE RENDERER WAS MISSING.** The
/// spine builds all five verbs and the `-v` flag from the dispatch table, so
/// `claude skills --help` has been listing commands that answered `2` -- which
/// is the table-says-it-ships / binary-says-no shape the spine's own comment
/// records for three short-only flags.
fn skills(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("list", a)) => skills_list(a),
    Some(("install", a)) => skills_change(a, SkillVerb::Install),
    Some(("sync", a)) => skills_change(a, SkillVerb::Sync),
    Some(("uninstall", a)) => skills_change(a, SkillVerb::Uninstall),
    Some(("show", a)) => skills_show(a),
    Some((verb, _)) => unwired("claude", &format!("skills {verb}")),
    None => unwired("claude", "skills"),
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SkillVerb {
  Install,
  Sync,
  Uninstall,
}

/// The skill library this binary serves, rooted at its own install.
///
/// **NO PROJECT IS OPENED.** Skills belong to the INSTALL and land in the
/// OPERATOR's home; neither is a property of the directory you happen to be
/// standing in, and requiring a project would make the command unusable in the
/// one place people first reach for it -- a fresh machine.
///
/// `$HOME` is reached through [`intentsvcs::userstate`], which is the single
/// file permitted to read it (hv, 2026-08-22). Extensions stay `None` on the
/// held ruling; see that module for the named consequence.
fn skills_lib() -> Result<intentsvcs::skills::Skills, Failure> {
  use intentsvcs::userstate;
  let install = intentsvcs::install::home()
    .map_err(|e| Failure::Error(format!("error: {e}\n  remedy: {}", e.remedy())))?;
  let target = userstate::skills_target()
    .map_err(|e| Failure::Error(format!("error: {e}\n  remedy: {}", e.remedy())))?;
  let manifest = userstate::skills_manifest()
    .map_err(|e| Failure::Error(format!("error: {e}\n  remedy: {}", e.remedy())))?;
  Ok(intentsvcs::skills::Skills::new(
    &install,
    userstate::ext_base(),
    target,
    manifest,
  ))
}

fn skill_names(a: &ArgMatches) -> Vec<String> {
  a.get_many::<String>("name")
    .map(|v| v.cloned().collect())
    .unwrap_or_default()
}

fn skills_fail(e: intentsvcs::skills::SkillsError) -> Failure {
  Failure::Error(format!("error: {e}\n  remedy: {}", e.remedy()))
}

/// install / sync / uninstall -- one renderer, because they report the same
/// thing about the same objects (IN-AG-HIGHLANDER-001). Three copies of the
/// tally would agree the day they were written.
fn skills_change(a: &ArgMatches, verb: SkillVerb) -> Result<(), Failure> {
  use intentsvcs::skills::Outcome;
  let lib = skills_lib()?;
  let names = skill_names(a);
  // **`--force` NOW REACHES THIS SURFACE**, declared on the `claude skills` row
  // and read here. It was wired as a hard `false` until 2026-08-23, which left
  // every held skill with no CLI remedy at all -- so the messages below could
  // only name what an operator could do by hand.
  //
  // **`try_get_one` RATHER THAN `get_one`, BECAUSE THE VERB DECIDES WHETHER THE
  // FLAG EXISTS.** `uninstall` does not take it: v2's `--force` there skips an
  // interactive confirmation and v3 does not prompt at all, so there is no
  // prompt to skip. Asking clap for an argument a subcommand never declared
  // PANICS, and a panic here would turn a correct absence into a crash.
  let force = a.try_get_one::<bool>("force").ok().flatten().copied() == Some(true);

  if names.is_empty() && verb != SkillVerb::Sync {
    return Err(Failure::Error(format!(
      "error: name at least one skill to {}\n  remedy: `intent claude skills list` prints what this install carries",
      if verb == SkillVerb::Install {
        "install"
      } else {
        "uninstall"
      }
    )));
  }

  let report = match verb {
    SkillVerb::Install => lib.install(&names, force),
    SkillVerb::Sync => lib.sync(force),
    SkillVerb::Uninstall => lib.uninstall(&names),
  }
  .map_err(skills_fail)?;

  if report.steps.is_empty() {
    println!("ok: nothing installed by this build yet");
    return Ok(());
  }

  // **A STEP THAT NEEDS A DECISION IS COUNTED SEPARATELY FROM ONE THAT FAILED,
  // AND BOTH ARE COUNTED SEPARATELY FROM ONE THAT WORKED.** v2 collapses held
  // and succeeded into one exit 0, so an operator whose skill was held back
  // learns it only by reading the scrollback.
  let mut moved = 0;
  let mut settled = 0;
  let mut needs_decision = 0;

  for step in &report.steps {
    let line = match &step.outcome {
      Outcome::Installed { files } => {
        moved += 1;
        format!("installed ({files} file(s))")
      }
      Outcome::Updated { written, removed } if removed.is_empty() => {
        moved += 1;
        format!("updated ({written} file(s))")
      }
      Outcome::Updated { written, removed } => {
        moved += 1;
        format!(
          "updated ({written} file(s), {} retired: {})",
          removed.len(),
          removed.join(", ")
        )
      }
      // **THE DISCARDED CHECKSUM IS THE WHOLE REMEDY AND IS PRINTED FIRST**
      // (vc's ruling). Once the copy has run it is the only artefact that can
      // identify what was there, so it leads the line rather than trailing it.
      // **`--force` is named as the cause** so a run that destroyed work cannot
      // be mistaken, in a scrollback, for the routine update above it -- which
      // is v2's defect exactly: it prints `update available` either way.
      Outcome::Forced {
        written,
        removed,
        discarded,
        baseline,
      } => {
        moved += 1;
        let retired = if removed.is_empty() {
          String::new()
        } else {
          format!(", {} retired: {}", removed.len(), removed.join(", "))
        };
        // **THE LINE NAMES WHICH STATE WAS RESOLVED, BECAUSE THE DISCARD MEANS
        // DIFFERENT THINGS IN EACH** (vc's condition 2). An ordinary discard
        // line says *this was your edit*. **With no baseline nobody can know
        // that** -- AC-07.3(d) says so explicitly -- so saying it would assert
        // exactly what (d) rules unknowable, and an operator would go looking
        // for an edit they may never have made.
        let provenance = match baseline {
          intentsvcs::skills::Baseline::Recorded => "your local changes",
          intentsvcs::skills::Baseline::Absent => {
            "content this build had no record of writing, so whether it was your edit or an upstream change is NOT KNOWN"
          }
        };
        format!(
          "OVERWRITTEN by --force; discarded {provenance}; discarded tree checksum {discarded} ({written} file(s) written{retired})"
        )
      }
      Outcome::Removed { removed, left } if left.is_empty() => {
        moved += 1;
        format!("removed ({} file(s))", removed.len())
      }
      Outcome::Removed { removed, left } => {
        moved += 1;
        format!(
          "removed ({} file(s)); left {} this build did not install: {}",
          removed.len(),
          left.len(),
          left.join(", ")
        )
      }
      Outcome::UpToDate => {
        settled += 1;
        "up to date".to_string()
      }
      Outcome::NotInstalled => {
        settled += 1;
        "not installed".to_string()
      }
      Outcome::AlreadyInstalled => {
        needs_decision += 1;
        "already installed -- `--force` overwrites it and reports the checksum of what it discarded"
          .to_string()
      }
      Outcome::ModifiedLocally => {
        needs_decision += 1;
        "modified here since it was installed -- HELD. `--force` takes the source copy and reports the checksum of what it discarded; copy your edits out first if you want them".to_string()
      }
      Outcome::Conflicted => {
        needs_decision += 1;
        "changed upstream AND here -- HELD. `--force` takes the source copy and reports the checksum of what it discarded; copy your edits out first if you want them".to_string()
      }
      Outcome::Undecidable => {
        needs_decision += 1;
        "installed here, but this build has no record of writing it, so it cannot tell an upstream change from your own edit -- and no further evidence exists to settle it. `--force` decides it in the source's favour and reports the checksum of what it discarded".to_string()
      }
      Outcome::SourceMissing => {
        needs_decision += 1;
        "no source for this name in this install".to_string()
      }
    };
    println!("  {:<28} {line}", step.name);
    if let Some(prov) = &step.shadowed {
      eprintln!("warning: `{}` is shadowed by {prov}", step.name);
    }
  }

  println!("ok: {moved} changed, {settled} already settled, {needs_decision} need a decision");

  // **EXIT 1 WHEN SOMETHING NEEDS A DECISION, AND THIS IS A CHOICE RATHER THAN
  // AN INHERITANCE.** v2 exits 0 unless EVERYTHING failed, so a held skill is
  // reported in a line nobody reads and the command says it succeeded. A
  // decision the operator has not made is not a success, and `Verdict` is the
  // right shape: the detail is already on stdout where it can be read or
  // parsed, so a second copy on stderr would be noise.
  if needs_decision > 0 {
    return Err(Failure::Verdict);
  }
  Ok(())
}

fn skills_list(a: &ArgMatches) -> Result<(), Failure> {
  let lib = skills_lib()?;
  let available = lib.available().map_err(skills_fail)?;
  let verbose = a.try_get_one::<bool>("v").ok().flatten().copied() == Some(true);

  if available.is_empty() {
    println!("no skills in this install");
    return Ok(());
  }
  for origin in &available {
    let state = if lib.is_installed(&origin.name) {
      "installed"
    } else {
      "-"
    };
    if verbose {
      println!(
        "{:<28} {:<10} {:<8} {}",
        origin.name,
        state,
        origin.provenance,
        origin.dir.display()
      );
    } else {
      println!("{:<28} {:<10} {}", origin.name, state, origin.provenance);
    }
  }
  Ok(())
}

fn skills_show(a: &ArgMatches) -> Result<(), Failure> {
  let lib = skills_lib()?;
  let Some(name) = skill_names(a).into_iter().next() else {
    return Err(Failure::Error(
      "error: name a skill to show\n  remedy: `intent claude skills list` prints what this install carries".to_string(),
    ));
  };
  let Some(origin) = lib.resolve(&name).map_err(skills_fail)? else {
    return Err(Failure::Error(format!(
      "error: no skill named `{name}` in this install\n  remedy: `intent claude skills list` prints what there is"
    )));
  };
  let body = std::fs::read_to_string(origin.dir.join("SKILL.md")).map_err(|e| {
    Failure::Error(format!(
      "error: cannot read {}: {e}\n  remedy: the skill resolved but its SKILL.md could not be read -- check the install tree",
      origin.dir.display()
    ))
  })?;
  print!("{body}");
  Ok(())
}

/// `intent claude rules` -- the highest-traffic verb in the tool.
///
/// **125 CALL SITES IN THIS REPO'S OWN MACHINERY**, measured across `.claude/`,
/// `lib/templates/`, `intent/plugins/`, `AGENTS.md`, `CLAUDE.md` and
/// `usage-rules.md` -- against 230 for the whole `claude` family. The four rules
/// of the road are not vendored into a consuming project; every agent reads them
/// through this command, so while it answered `2` the agentic contract named a
/// command that could not be run.
///
/// **THE VERB DEFAULTS TO `list`, per the table's `default: "list"` on the
/// slot.** Bare `intent claude rules` lists, which is what every consumer that
/// omits the verb already expects.
///
/// **`validate` AND `index` ARE DELIBERATELY LEFT ANSWERING `2`, AND THAT IS NOT
/// AN OVERSIGHT.** The row's `target.state` is `pending-hv` with a real
/// question on it -- *in v3 rules are embedded in the binary (WP-07), so `index`
/// has no installation to mutate and arguably retires with the on-disk rules
/// root*. `list` and `show` carry no part of that question: they are pure reads
/// and they are what the 125 call sites use. **Implementing the unquestioned
/// half does not resolve the questioned half**, and shipping `index` to make the
/// family look complete would settle a pending ruling by writing code.
fn rules(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("list", a)) => rules_list(a),
    Some(("show", a)) => rules_show(a),
    Some((verb, _)) => unwired("claude", &format!("rules {verb}")),
    None => rules_list(m),
  }
}

/// The library this binary serves.
///
/// **NO PROJECT IS OPENED, AND THAT IS THE POINT.** The rule library belongs to
/// the INSTALL, so `intent claude rules` must answer from anywhere -- including
/// the directory an operator is about to run `intent init` in. Reaching for
/// `context()` here would make the command that explains the rules require a
/// project that does not exist yet.
fn library() -> Result<intentsvcs::rules::Library, Failure> {
  let home = intentsvcs::install::home()
    .map_err(|e| Failure::Error(format!("error: {e}\n  remedy: {}", e.remedy())))?;
  // **CANON ONLY TONIGHT, AND THE `None` IS A HELD RULING RATHER THAN A GAP.**
  // v2 also serves rule packs from `~/.intent/ext`, resolved through
  // `$INTENT_EXT_DIR` / `$INTENT_EXT_DISABLE` / `$HOME`. Wiring that here fails
  // `no_intent_home::the_shipped_surface_reads_exactly_one_environment_variable`
  // -- `ALLOWED` is `["COLUMNS"]`, exactly one -- and that test says in its own
  // failure message that a further read "needs an hv ruling and a row in
  // ALLOWED, not a quiet addition", because every machine here has the variable
  // set so nothing else would fail.
  //
  // **The extension case is genuinely different from `$INTENT_HOME` and that is
  // why it is a question rather than a refusal**: the assets are unversioned and
  // operator-authored, so there is no v2/v3 skew to serve wrongly. But the
  // invariant is one variable, the ruling is hv's, and the seam is a parameter
  // rather than a rewrite -- ext support is this argument and nothing else.
  //
  // **THE CONSEQUENCE IS NAMED, NOT SWALLOWED:** an operator with rules under
  // `~/.intent/ext` sees them from v2 and not from v3, and `Provenance::Ext`
  // exists and is currently unreachable.
  Ok(intentsvcs::rules::Library::new(&home, None))
}

/// `intent claude rules list [--lang <lang>]`.
///
/// **THE COLUMN WIDTHS ARE v2's, TO THE CHARACTER, AND THEY ARE NOT DECORATION.**
/// This output is read by people and pasted into messages; a v3 that reflows it
/// makes every existing screenshot, doc and habit subtly wrong for no gain. v2
/// picked them so the widest real value fits with slack -- `id` is dominated by
/// `IN-AG-THIN-COORD-001` at 20, `severity` by the `recommendation` enum at 14,
/// `category` by `architecture` at 12 -- and the `prov` column is wide because
/// it carries `ext:<name>` rather than just `canon`.
///
/// **AN EMPTY FILTER RESULT PRINTS THE HEADER AND `total: 0`, not silence.** A
/// command that prints nothing is indistinguishable from one that failed to run,
/// and `--lang` is exactly where a typo lands.
fn rules_list(m: &ArgMatches) -> Result<(), Failure> {
  let lang = m.get_one::<String>("lang").cloned();
  let all = library()?.rules().map_err(|e| Failure::Error(e.render()))?;
  let shown: Vec<_> = all
    .iter()
    .filter(|r| lang.as_ref().is_none_or(|l| &r.language == l))
    .collect();

  // The trailing column is written into the format string rather than passed
  // as an argument: clippy denies `{}` fed a literal, and CI runs it with
  // `-D warnings`. The other five stay arguments because they carry width
  // specifiers, which is what the format string is for.
  println!(
    "{:<22} {:<14} {:<10} {:<14} {:<14} title",
    "id", "severity", "language", "category", "prov"
  );
  println!(
    "{:<22} {:<14} {:<10} {:<14} {:<14} -----",
    "--", "--------", "--------", "--------", "----"
  );
  for r in &shown {
    println!(
      "{:<22} {:<14} {:<10} {:<14} {:<14} {}",
      r.id,
      dash(&r.severity),
      dash(&r.language),
      dash(&r.category),
      r.provenance.to_string(),
      dash(&r.title)
    );
  }
  println!();
  println!("total: {} rule(s)", shown.len());
  Ok(())
}

/// `intent claude rules show <id>`.
///
/// **AN UNKNOWN ID EXITS 1, NOT 2** -- the table's `observed` says so, and the
/// distinction is the one the whole exit-code contract turns on. `1` means the
/// command RAN and the answer is no; `2` means this build cannot answer at all.
/// A mistyped rule id is emphatically the first.
///
/// **THE REMEDY NAMES THE COMMAND THAT LISTS THEM, and does not guess at a near
/// match.** A "did you mean" that is wrong sends the reader to the wrong rule
/// with confidence; `rules list` is one command and always correct.
fn rules_show(m: &ArgMatches) -> Result<(), Failure> {
  let id = arg(m, "id")?;
  let found = library()?
    .show(&id)
    .map_err(|e| Failure::Error(e.render()))?;
  match found {
    Some((rule, body)) => {
      // **v2's HEADER, KEPT VERBATIM, and the provenance line is why.** Without
      // it the reader cannot tell a rule the tool ships from one this machine
      // adds -- which is the difference between "file a bug against Intent" and
      // "look at your own extension", and it is invisible in the body. The
      // source path is absolute on purpose: an agent told to go and edit the
      // rule needs somewhere to go.
      println!("# Rule: {}", rule.id);
      println!("# Provenance: {}", rule.provenance);
      println!("# Source: {}", rule.path.display());
      println!();
      print!("{body}");
      if !body.ends_with('\n') {
        println!();
      }
      Ok(())
    }
    // `Error`, not `Verdict`: nothing has been written to stdout, so the
    // operator needs the message on stderr. Both are exit 1.
    None => Err(Failure::Error(format!(
      "error: no rule with id `{id}`\n  remedy: `intent claude rules list` names every rule this build serves, canon and extension alike"
    ))),
  }
}

/// A field the frontmatter did not carry, shown rather than left blank.
///
/// v2 prints `?` for an absent value. Blank would make a malformed rule look
/// like a narrow column, which is the failure mode where nobody ever fixes it.
fn dash(value: &str) -> &str {
  if value.is_empty() { "?" } else { value }
}

/// `intent claude hook <name>` -- run a shipped Claude Code lifecycle hook.
///
/// **The single most parity-critical entry in the family** (the dispatch table
/// says so on the row) and the direct cause of issue 0043. A consumer's
/// `.claude/settings.json` names `intent claude hook require-in-session` on
/// EVERY prompt; with the command unimplemented it answered `2`, which is
/// Claude Code's BLOCK code, so a migrated project refused every prompt and
/// the refusal could not be cleared from inside the session.
///
/// **`exec`, not spawn-and-wait, for the same three reasons v2 gave.** The
/// hooks read Claude Code's event JSON on stdin, so stdin must flow through
/// untouched; the gate signals BLOCK with exit 2 specifically, so the exit code
/// must be the script's own and not a wrapper's translation of it; and
/// replacing the process makes it impossible for a later edit here to swallow
/// either by accident. A wrapper that merely intended to pass the code through
/// is the shape that produced this issue.
fn hook(a: &ArgMatches) -> Result<(), Failure> {
  let name = arg(a, "name")?;
  if !intentsvcs::install::HOOKS.contains(&name.as_str()) {
    return Err(Failure::Error(format!(
      "error: unknown hook: {name}\n  remedy: one of {}",
      intentsvcs::install::HOOKS.join(", ")
    )));
  }
  let home = intentsvcs::install::home().map_err(|e| Failure::Error(format!("error: {e}")))?;
  let script = intentsvcs::install::hook_script(&home, &name);
  exec_shipped_script(&script, &[], "hook script")
}

/// Replace this process with `bash <script> <args...>`, or fail BY NAME.
///
/// **Extracted because there are now two doors, not because it is tidier.**
/// `claude hook` and `claude start`/`claude ws` both resolve a shipped shell
/// asset out of [`intentsvcs::install::home`] and hand the process to it, and
/// the load-bearing part of that dance is the `is_file` check: without it an
/// absent asset reaches the shell and comes back as an opaque `127`, which
/// tells an operator nothing about which of the two things is missing. One
/// copy, so the next door cannot ship without the check.
///
/// `kind` names what was not found, because "script not found" over a path is
/// a fact the operator already has -- what they lack is which subsystem is
/// incomplete.
fn exec_shipped_script(script: &Path, args: &[String], kind: &str) -> Result<(), Failure> {
  use std::os::unix::process::CommandExt;

  // Named rather than left to the shell. Claude Code surfaces a hook's stderr,
  // so this is the difference between "Intent's install is incomplete" and an
  // opaque 127 the operator cannot act on.
  if !script.is_file() {
    return Err(Failure::Error(format!(
      "error: {kind} not found: {}\n  remedy: the Intent install is incomplete -- reinstall it",
      script.display()
    )));
  }
  let e = std::process::Command::new("bash")
    .arg(script)
    .args(args)
    .exec();
  // `exec` returns ONLY on failure to replace the process.
  Err(Failure::Error(format!(
    "error: cannot run {}: {e}",
    script.display()
  )))
}

/// Read a declared positional by the name the DISPATCH TABLE gives it.
///
/// `try_get_one` rather than `get_one`, and the distinction is load-bearing:
/// `get_one` PANICS when the id was never declared, so a renderer that asked
/// for a name the table does not carry would crash with a clap internal
/// message and exit 101 -- neither a v2 exit code nor an Intent error. This
/// turns a table/renderer mismatch into a named failure, which is what
/// No Silent Errors asks for at a seam between two things that must agree.
fn arg(m: &ArgMatches, name: &str) -> Result<String, Failure> {
  match m.try_get_one::<String>(name) {
    Ok(Some(value)) => Ok(value.clone()),
    Ok(None) => Err(Failure::Error(format!("error: {name} is required"))),
    Err(e) => Err(Failure::Error(format!(
      "error: the CLI asked for an argument `{name}` that the dispatch table does not declare\n  caused by: {e}\n  remedy: this is a build defect -- the renderer and surface/dispatch-table.json disagree"
    ))),
  }
}

/// An operator's spelling of a THREAD id, canonicalised, or a refusal that says
/// which kind of wrong it is.
///
/// **THIS IS THE ONE THREAD DOOR AND THERE USED TO BE NONE.** Every st, ac, at,
/// wp and sync verb read its id with `arg` and handed the raw bytes to a lookup
/// that compared them to `t.id`, so `st show 59` answered *no steel thread 59 in
/// this project* -- **A NOT-FOUND FOR SOMETHING THAT WAS NEVER A NAME.** That is
/// the one wrong answer `address::promote`'s own doc names, honoured there and
/// routed around by nine verbs.
fn thread_arg(m: &ArgMatches, name: &str) -> Result<String, Failure> {
  let raw = arg(m, name)?;
  thread_spec(&raw)
}

/// `ST0056`, `56`, `s56` -- and the `<thread>/<NN>` composite the scoped verbs
/// take, whose thread half is normalised and whose tail is passed through
/// UNTOUCHED because a work-package number is not this function's to interpret.
fn thread_spec(raw: &str) -> Result<String, Failure> {
  let (head, tail) = match raw.split_once('/') {
    Some((h, t)) => (h, Some(t)),
    None => (raw, None),
  };
  let id =
    model::normalise_thread_id(head).map_err(|e| id_refusal(head, e, model::IdKind::Thread))?;
  Ok(match tail {
    Some(t) => format!("{id}/{t}"),
    None => id,
  })
}

/// An operator's spelling of an ISSUE id, as a number.
fn issue_arg(m: &ArgMatches, name: &str) -> Result<u32, Failure> {
  let raw = arg(m, name)?;
  model::normalise_issue_id(&raw).map_err(|e| id_refusal(&raw, e, model::IdKind::Issue))
}

/// **THE ONE PLACE A BAD ID IS EXPLAINED, AND THE WANTED COLLECTION IS THE
/// CALLER'S TO SUPPLY.** `model::IdError` is a fact about a spelling and is
/// deliberately collection-independent -- `i59` is a well-formed id, and whether
/// that is an error depends entirely on which verb was asked. Putting the
/// expectation in the error would make the model hold the CLI's context; putting
/// the message here keeps the fact in one place and the wording in another.
fn id_refusal(raw: &str, e: model::IdError, wanted: model::IdKind) -> Failure {
  let what = wanted.as_str();
  Failure::Error(match e {
    model::IdError::NotAnId => format!(
      "error: `{raw}` is not {} id\n  remedy: name it as a number, eg `{}`",
      wanted.with_article(),
      example(wanted)
    ),
    model::IdError::OutOfRange => format!(
      "error: `{raw}` is out of range -- {} id is {} digits\n  remedy: name it as a number, eg `{}`",
      wanted.with_article(),
      model::THREAD_DIGITS,
      example(wanted)
    ),
    // **THE REFUSAL THIS CHANGE EXISTS FOR.** Naming the OTHER collection and
    // offering the tag that would have worked, because a refusal whose remedy is
    // a spelling the operator can type is a different object from one that says
    // no. Reporting this as a not-found sends them into the estate looking for
    // something that was never addressed.
    model::IdError::WrongCollection { named } => format!(
      "error: `{raw}` names {}, and this verb takes {}\n  remedy: `{}{}` names the {what}",
      named.with_article(),
      wanted.with_article(),
      wanted.tag(),
      raw.trim_start_matches(|c: char| !c.is_ascii_digit())
    ),
    // Only the collection-agnostic door can raise this: every other caller
    // learns the collection from its own verb.
    model::IdError::Ambiguous { seq } => format!(
      "error: `{raw}` names both a steel thread and an issue\n  remedy: `s{seq}` names the steel thread, `i{seq}` the issue"
    ),
  })
}

/// A worked example in the reader's own vocabulary. `ST0000` is the STZero
/// retrofit id (D37), so it names something in THEIR project rather than ours.
fn example(kind: model::IdKind) -> &'static str {
  match kind {
    // `0` and `ST0000` are the SAME id in its two spellings, which is the
    // equivalence the example is teaching -- and `ST0000` is the STZero
    // retrofit id, so it names something in the READER's project rather than a
    // thread in ours. `no_pm_state_in_output` enforces exactly that, and caught
    // this line carrying `ST0046` on its first run.
    model::IdKind::Thread => "0` or `ST0000",
    model::IdKind::Issue => "21` or `0021",
  }
}

/// **THE ONE PLACE A COMMAND'S OUTPUT SHAPE IS DECIDED.**
///
/// Reads every spelling and hands back a resolved [`Output`]. It is safe on a
/// verb declaring none of them, because [`opt`] and [`flag`] are absent-not-fatal
/// for an undeclared name -- so a verb opts in by declaring the flag in the
/// dispatch table and needs no arm here.
///
/// **THE TERMINAL WIDTH IS DISCOVERED HERE AND NOWHERE DEEPER.** AC-11.3 permits
/// the shipped surface exactly one environment variable, `COLUMNS`; a services
/// module reaching for it would put that permission somewhere nothing checks.
fn output_of(m: &ArgMatches) -> Result<Output, Failure> {
  Output::resolve(
    opt_explicit(m, "format").as_deref(),
    opt(m, "width").as_deref(),
    flag(m, "json"),
    flag(m, "markdown"),
    terminal_width(),
  )
  .map_err(|e| Failure::Error(e.render()))
}

/// A value the CALLER actually typed, distinguished from one clap supplied.
///
/// **A DEFAULT IS NOT A CHOICE, AND CONFLATING THEM BREAKS EVERY ALIAS.** The
/// `--format` rows declare `default: terminal`, which is right -- it documents
/// the vocabulary and drives `--help`. But [`opt`] cannot tell a default from a
/// typed value, so `st list --markdown` read as *`--format terminal` AND
/// `--markdown`*, two formats, and refused a command nobody had made ambiguous.
/// **The refusal was correct about its inputs and wrong about the world.**
///
/// Measured: it fired on every alias use the moment the default was declared,
/// which is also why the earlier hand-driven pass came back clean -- that binary
/// predated the default. `ValueSource` is the only thing that can separate the
/// two, so the question is asked of clap rather than inferred from the value.
fn opt_explicit(m: &ArgMatches, name: &str) -> Option<String> {
  // **`value_source` PANICS ON AN UNDECLARED ID, WHERE `try_get_one` RETURNS
  // `Err`.** That asymmetry is the whole reason [`opt`] and [`flag`] exist -- a
  // verb that does not declare a flag must not crash the renderer -- and
  // reaching for a sibling API that does not share the property put a panic on
  // every verb without `--format`. Measured: `intent issues` exited 101 with
  // *`"format"` is not an id of an argument or a group*.
  //
  // Asking [`opt`] first makes the second call safe by construction rather than
  // by a list of which verbs declare what: a `Some` here means the id exists.
  let value = opt(m, name)?;
  match m.value_source(name) {
    Some(clap::parser::ValueSource::CommandLine) => Some(value),
    _ => None,
  }
}

/// Render a table through the resolved output, or refuse JSON by name.
///
/// **A VERB THAT CANNOT EMIT JSON SAYS SO INSTEAD OF EMITTING SOMETHING ELSE.**
/// `Output::table` returns `None` for JSON because a list-of-lists is not the
/// object anyone means; a verb with a real JSON projection branches before this.
fn table_out(out: &Output, headers: &[&str], rows: &[Vec<String>]) -> Result<String, Failure> {
  out.table(headers, rows).ok_or_else(|| {
    Failure::Error(
      "error: this verb has no json projection\n  remedy: use `--format terminal` or `--format md`"
        .to_string(),
    )
  })
}

/// An optional value, ABSENT rather than fatal when this subcommand does not
/// declare it.
///
/// `get_one` panics on an undeclared id -- exit 101, neither a v2 code nor an
/// Intent error -- so a helper shared by two subcommands cannot use it. That
/// is not hypothetical: `st list` and `st sync` share a renderer, `st sync`
/// declares no `--markdown`, and the shared code panicked the moment it asked.
/// The body an `issues add` invocation gives the new issue.
///
/// **`--body` AND `--from` TOGETHER REFUSE RATHER THAN ONE WINNING** (hv,
/// 2026-08-27, with the door itself). Both name the issue's prose, so passing
/// both leaves nothing on the command line to say which the author meant --
/// and a precedence rule would resolve it SILENTLY, in favour of whichever the
/// implementer happened to test first. An author who passes both has made a
/// mistake they can see; an author whose file was quietly discarded in favour
/// of an inline string has one they cannot.
///
/// **AN UNREADABLE `--from` REFUSES AND IS NEVER AN EMPTY BODY.** The whole
/// point of this door is that the field had no writer, so a read failure
/// silently creating an empty-bodied issue would be the defect it exists to
/// close -- and it would land in the record as an issue somebody wrote nothing
/// in. Absence and unreadability are different states here as everywhere.
///
/// Neither flag is the ordinary case and gives an empty body: nobody wrote one,
/// which is a state.
fn issue_body(m: &ArgMatches) -> Result<String, Failure> {
  match (opt(m, "body"), opt(m, "from")) {
    (Some(_), Some(_)) => Err(Failure::Error(
      "error: --body and --from both give the issue its prose, so passing both says nothing about which one you meant\n  remedy: pass one of them"
        .to_string(),
    )),
    (Some(text), None) => Ok(text),
    (None, Some(path)) => std::fs::read_to_string(&path).map_err(|e| {
      Failure::Error(format!(
        "error: --from could not read {path}\n  caused by: {e}\n  remedy: name a readable file, or pass the prose inline with --body"
      ))
    }),
    (None, None) => Ok(String::new()),
  }
}

fn opt(m: &ArgMatches, name: &str) -> Option<String> {
  m.try_get_one::<String>(name).ok().flatten().cloned()
}

/// A boolean flag, FALSE when this subcommand does not declare it. Same
/// reasoning as [`opt`].
fn flag(m: &ArgMatches, name: &str) -> bool {
  m.try_get_one::<bool>(name).ok().flatten().copied() == Some(true)
}

/// `intent backup` takes a snapshot; `--list` reports what exists.
///
/// **`--list` is deliberately NOT the health report.** It answers what exists,
/// and one place reports health -- `doctor`. Two commands answering "is my
/// backup all right" is how they come to disagree, and the one a user reaches
/// for first would be the one that never says no.
fn backup(m: &ArgMatches) -> Result<(), Failure> {
  let facade = open()?;
  let project = facade.project().clone();

  if flag(m, "list") {
    let snapshots = facade.store().snapshots().map_err(|e| e.render())?;
    if snapshots.is_empty() {
      // Not silence: an empty list and a broken backup look identical on an
      // empty stream, and only one of them is fine.
      println!("no snapshots have been taken of this store");
      return Ok(());
    }
    for snapshot in &snapshots {
      match (&snapshot.path, snapshot.bytes) {
        (Some(path), Some(bytes)) => {
          println!("{}  {path}  {bytes} bytes", snapshot.taken_at)
        }
        // An attempt with no file is the record of a failure, and it prints
        // rather than being filtered out -- filtering it is how a list of
        // successes comes to read as a history.
        _ => println!(
          "{}  {}  {}",
          snapshot.taken_at,
          snapshot.outcome,
          snapshot.detail.as_deref().unwrap_or("no detail recorded")
        ),
      }
    }
    return Ok(());
  }

  let written = intentsvcs::backup::take(&project, facade.store()).map_err(|e| e.render())?;
  println!("created: {}", project.relative(&written));

  let retention = intentsvcs::backup::Retention::from_project(&project);
  let removed =
    intentsvcs::backup::prune(&project, facade.store(), retention).map_err(|e| e.render())?;
  for path in &removed {
    println!("removed: {}", project.relative(path));
  }
  Ok(())
}

/// The `agents` family -- the `ROOT_FILES` generator (ST0057 AC-00.4).
///
/// **`generate` is wired and its four siblings are not, and that is a chosen
/// boundary rather than partial delivery.** The dispatch table records `agents
/// sync` as writing `AGENTS.md.bak` beside the file it rewrites; v3 already has
/// [`intentsvcs::backup`] carrying D35's rolling snapshots, so wiring the write
/// half tonight would pick between two backup mechanisms by accident, at the
/// hour when the person who owns that call is asleep. `generate` is the half
/// the table itself calls "pure emit-path -- writes nothing", and it is the
/// whole of what AC-00.4 asks for: a generator, so that the derivability of
/// these three files stops being an assumption.
///
/// Errors render through [`Remedy::render`] rather than a second format string
/// here -- that trait exists precisely so a caller cannot invent a rival
/// rendering, and `fail` is not reusable because it takes a `FacadeError`.
fn agents(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("generate", _)) => {
      // **`open`, not `context`, and the difference is a defect this shipped
      // with for thirteen minutes.** `context` only DISCOVERS the project, so
      // the first cut emitted a v3 `AGENTS.md` over an unmigrated v2 estate at
      // exit 0 -- a generator answering about a project it cannot read. Only
      // `doctor` is entitled to `context`, because it is the verb you reach for
      // precisely when the project will not open. Found by dc's
      // `unmigrated_surface` sweep, which is the row that exists for this.
      let f = open()?;
      let home = intentsvcs::install::home().map_err(|e| Failure::Error(e.render()))?;
      let ctx = views::RenderContext {
        version: env!("CARGO_PKG_VERSION"),
        // Nothing on this path renders `todo.md`, so there is no watermark to
        // carry and asking the store for one would be a read with no reader.
        todo_watermark: None,
      };
      let content = intentsvcs::rootfiles::render(&home, "AGENTS.md", f.project().config(), &ctx)
        .map_err(|e| Failure::Error(e.render()))?;
      // `print!`, not `println!` -- the template ends with its own newline and
      // a second one would put the generated file one byte away from what the
      // generator produced, which is exactly the comparison AC-00.4 exists for.
      print!("{content}");
      Ok(())
    }
    Some(("sync", _)) => {
      let f = open()?;
      let home = intentsvcs::install::home().map_err(|e| Failure::Error(e.render()))?;
      let ctx = views::RenderContext {
        version: env!("CARGO_PKG_VERSION"),
        // Nothing on this path renders `todo.md`, so there is no watermark to
        // carry and asking the store for one would be a read with no reader.
        todo_watermark: None,
      };
      // **THIS VOICE IS v2's AND IT IS NOT TIDIED.** A bare capitalised progress
      // line with a trailing ellipsis, then a line carrying a full stop -- both
      // against the house style issue 0023 spent a release enforcing, both
      // recorded `as-observed` on the dispatch-table row, and reproducing them
      // IS the contract. The third line v2 printed announced `AGENTS.md.bak`;
      // it is gone because the backup is, and that deviation was RATIFIED
      // rather than cleaned up -- see `rootfiles::sync`.
      println!("Syncing AGENTS.md with latest project state...");
      intentsvcs::rootfiles::sync(
        f.project().root(),
        &home,
        "AGENTS.md",
        f.project().config(),
        &ctx,
      )
      .map_err(|e| Failure::Error(e.render()))?;
      println!("ok: AGENTS.md updated at project root.");
      Ok(())
    }
    Some((verb, _)) => unwired("agents", verb),
    None => unwired("agents", ""),
  }
}

/// `intent critic <lang>` -- the headless rule critic, and the pre-commit gate.
///
/// **THE EXIT CODES ARE THE CONTRACT AND THEY ARE NOT clap's.** 0 clean, 1
/// findings, 2 usage, 3 refused. `Failure::Error` would give findings the right
/// code by accident; it is used deliberately here so the mapping is stated once
/// in `spine.rs` and read once here.
///
/// **A USAGE ERROR IN THIS COMMAND EXITS 2, NOT 1, AND IT IS THE ONE PLACE IN
/// THE SURFACE THAT DOES.** INV-02 puts usage errors at 1 everywhere else. ic
/// ruled the exception real (2026-08-20): a critic that cannot parse its own
/// invocation IS the gate's own breakage, and `pre-commit.sh` fails open on 2
/// precisely so a broken checker cannot stop anyone committing. Giving a usage
/// error the blocking code would make a typo in the hook's own command line
/// block every commit in the repository.
fn critic(m: &ArgMatches) -> Result<(), Failure> {
  // `--languages` is a pure read of the roster and answers before anything else,
  // including before a language is required.
  if m.get_flag("languages") {
    for l in intentsvcs::critic::HEADLESS_LANGUAGES {
      println!("{l}");
    }
    return Ok(());
  }

  let lang = m
    .get_one::<String>("lang")
    .ok_or_else(|| Failure::Unavailable("error: a language is required".into()))?;

  // **`author` AND `content` ARE A CLEAN NO-OP, NOT AN ERROR.** Prose critique
  // is on-demand via the `critic-prose` subagent; the gate dispatches every
  // declared language and must not print a spurious refusal for the two that
  // legitimately have no headless runner (issue 0003 -- the gate carries no
  // language knowledge of its own and cannot drift from this registry).
  if lang == "author" || lang == "content" {
    return Ok(());
  }

  // **AN UNKNOWN LANGUAGE IS A USAGE ERROR, NOT AN EMPTY RUN, AND THIS WAS A
  // REAL DEFECT CAUGHT BY DRIVING IT RATHER THAN READING IT.** Without this
  // check the run proceeds, finds no rules whose `language` matches, and
  // reports a clean census at exit 0. **In the gate that means a typo in a
  // project's declared language list silently disables checking for it** --
  // the failure reads as a pass, which is the one shape this command exists to
  // prevent. v2 refuses with exit 2 and the same roster.
  if !intentsvcs::critic::HEADLESS_LANGUAGES.contains(&lang.as_str()) {
    return Err(Failure::Unavailable(format!(
      "error: first argument must be a language ({}) or a prose discipline (author content)",
      intentsvcs::critic::HEADLESS_LANGUAGES.join(" ")
    )));
  }

  let severity_min = m
    .get_one::<String>("severity-min")
    .map(|s| s.as_str())
    .unwrap_or("warning");
  let severity_min = intentsvcs::critic::Severity::parse(severity_min).ok_or_else(|| {
    Failure::Unavailable(format!(
      "error: `{severity_min}` is not a severity\n  remedy: one of critical, warning, recommendation, style"
    ))
  })?;

  let mut files: Vec<std::path::PathBuf> = m
    .get_many::<String>("files")
    .map(|v| v.map(std::path::PathBuf::from).collect())
    .unwrap_or_default();

  if m.get_flag("staged") {
    files.extend(staged_files()?);
  }

  // **THE PROJECT IS OPTIONAL AND THAT IS DELIBERATE.** v2 runs this command
  // outside a project (`PROJECT_ROOT` may be empty) and only consults
  // `.intent_critic.yml` when there is one. Requiring a project would make the
  // critic unusable in exactly the place a fresh checkout needs it.
  let disabled = std::env::current_dir()
    .ok()
    .and_then(|cwd| intentsvcs::project::Project::discover(&cwd).ok())
    .and_then(|p| std::fs::read_to_string(p.root().join(".intent_critic.yml")).ok())
    .map(|t| intentsvcs::critic::parse_disabled(&t))
    .unwrap_or_default();

  let lib = library()?;
  let report = intentsvcs::critic::run(&lib, lang, &files, severity_min, &disabled)
    .map_err(|e| Failure::Unavailable(format!("error: {e}")))?;

  let json = m
    .get_one::<String>("format")
    .map(|f| f == "json")
    .unwrap_or(false);
  if json {
    render_critic_json(&report);
  } else {
    render_critic_text(&report, files.len(), severity_min.as_str());
  }

  // **REFUSAL OUTRANKS FINDINGS, AND BOTH BLOCK.** See `Report::exit_code`.
  //
  // **THE `2` ARM IS LOAD-BEARING AND ITS ABSENCE WAS INVISIBLE.** `exit_code`
  // is not the process's code -- this match translates it -- so an empty-library
  // refusal returning 2 from the report would have fallen into `_ => Ok(())` and
  // exited 0, leaving a unit test on `exit_code()` green over a surface that
  // never changed. The code and its arm are one change, not two.
  match report.exit_code() {
    // **THE LIBRARY LOADED NOTHING, SO THE RUN ASKED NOTHING.** `Unavailable` is
    // the variant whose documented meaning is *this build cannot answer the
    // question at all*, and whose code v2 consumers already treat as fail-open
    // rather than as a verdict about their own work -- which is what an absent
    // rule library deserves. It is OUR breakage: blocking every commit in every
    // vendored estate because the keg shipped without its rules is issue 0043
    // rebuilt on the git side.
    //
    // The hook has no arm for 2, and that is the point: it lands in `*)`, which
    // records the language UNENFORCED and prints the `N of M` digest. Five
    // declared languages read `5 of 5 declared language(s) went UNENFORCED`.
    2 => Err(Failure::Unavailable(format!(
      "critic: {} -- REFUSED: the rule library is EMPTY, so this run examined {} file(s) against NO rules.\n  a gate that armed nothing has not passed, it has abstained; refusing rather than sealing `ok` over an empty denominator.\n  remedy: this build cannot find its rule library -- reinstall or upgrade Intent so `intent/plugins/claude/rules` ships with it, then check `intent claude rules list --lang {}`",
      report.lang,
      files.len(),
      report.lang
    ))),
    3 => Err(Failure::Refused(format!(
      "critic: {} -- REFUSED: {} rule(s) armed by this project could not be enforced here: {}.\n  remedy: install the missing tool, or disarm that rule in .intent_critic.yml",
      report.lang,
      report.unenforced().len(),
      report.unenforced().join(" ")
    ))),
    1 => Err(Failure::Verdict),
    _ => Ok(()),
  }
}

/// The staged set, as the gate sees it.
///
/// `--diff-filter=ACM` deliberately: a DELETED file cannot be critiqued and a
/// runner that tried would report "cannot read" for an ordinary, correct commit.
fn staged_files() -> Result<Vec<std::path::PathBuf>, Failure> {
  let out = std::process::Command::new("git")
    .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
    .output()
    .map_err(|e| Failure::Unavailable(format!("error: cannot run git: {e}")))?;
  if !out.status.success() {
    return Err(Failure::Unavailable(
      "error: `git diff --cached` failed\n  remedy: run this inside a git repository".into(),
    ));
  }
  Ok(
    String::from_utf8_lossy(&out.stdout)
      .lines()
      .filter(|l| !l.trim().is_empty())
      .map(std::path::PathBuf::from)
      .collect(),
  )
}

/// The human face: findings first, then the census.
///
/// **THE CENSUS IS PRINTED ON EVERY RUN INCLUDING A CLEAN ONE, AND THAT IS THE
/// WHOLE POINT OF IT.** A runner that prints nothing when it finds nothing
/// reports a green over questions it never put -- which is this command's
/// founding defect. The headline leads with what was ASKED rather than what was
/// ARMED, because arming says something COULD answer and only asking says this
/// invocation DID.
fn render_critic_text(report: &intentsvcs::critic::Report, files: usize, severity_min: &str) {
  use intentsvcs::critic::{Arming, Disposition, Severity};

  // **THE CENSUS COMES FIRST AND THE FINDINGS FOLLOW IT.** That is v2's order
  // and it is the right way round: what could be asked frames what was found,
  // and a reader who sees findings first has already formed a verdict before
  // learning the run only covered two rules of six.
  println!(
    "critic: {} -- {} of {} rule(s) ASKED of this run; {} armed in total. A clean result covers what was ASKED and says nothing about the rest.",
    report.lang,
    report.ran(),
    report.total(),
    report.armed()
  );

  let declared = report
    .census
    .iter()
    .filter(|r| r.arming == Arming::Declared)
    .count();
  // **THE ID LISTS ARE SORTED, AND THIS IS A DELIBERATE DEVIATION FROM v2
  // RATHER THAN AN ACCIDENT OF PORTING.** v2 emits them in filesystem-walk
  // order. Measured 2026-08-20: both binaries are stable run-to-run and the
  // SETS are identical, so nothing is lost -- but walk order is meaningless to
  // a reader, undiffable between two runs on different machines, and it makes
  // "did this list change" a question nobody can answer by eye. Sorted, the
  // line is a set the reader can scan and a diff can compare.
  let mut undeclared: Vec<&str> = report
    .census
    .iter()
    .filter(|r| r.arming == Arming::Undeclared)
    .map(|r| r.rule_id.as_str())
    .collect();
  undeclared.sort_unstable();
  let mut unrunnable: Vec<&str> = report
    .census
    .iter()
    .filter(|r| r.arming == Arming::Unrunnable)
    .map(|r| r.rule_id.as_str())
    .collect();
  unrunnable.sort_unstable();
  let quiet = declared + undeclared.len() + unrunnable.len();

  if quiet > 0 {
    println!(
      "critic: {} -- {} rule(s) could not be armed at all: {} declared unanswerable, {} undeclared, {} with a proxy this runner must refuse.",
      report.lang,
      quiet,
      declared,
      undeclared.len(),
      unrunnable.len()
    );
  }
  if !undeclared.is_empty() {
    println!(
      "critic: {} -- UNDECLARED, nobody has recorded whether these are mechanically checkable: {}",
      report.lang,
      undeclared.join(" ")
    );
  }
  if !unrunnable.is_empty() {
    println!(
      "critic: {} -- UNRUNNABLE proxy, present but outside the runner contract: {}",
      report.lang,
      unrunnable.join(" ")
    );
  }

  let mut ooc: Vec<String> = report
    .census
    .iter()
    .filter_map(|r| match &r.disposition {
      Disposition::OutOfContext(tool) => Some(format!("{}({})", r.rule_id, tool)),
      _ => None,
    })
    .collect();
  ooc.sort_unstable();
  if !ooc.is_empty() {
    println!(
      "critic: {} -- ARMED but NOT RUN HERE, the tool does not belong in this context (a whole-workspace analyser is not a per-file gate): {}",
      report.lang,
      ooc.join(" ")
    );
  }

  let mut absent: Vec<String> = report
    .census
    .iter()
    .filter_map(|r| match &r.disposition {
      Disposition::ToolAbsent(tool) => Some(format!("{}({})", r.rule_id, tool)),
      _ => None,
    })
    .collect();
  absent.sort_unstable();
  if !absent.is_empty() {
    println!(
      "critic: {} -- ARMED but NOT RUN HERE, the tool is not on this machine: {}",
      report.lang,
      absent.join(" ")
    );
  }

  // **A CLEAN RUN SAYS WHAT IT COVERED, INCLUDING THE FILE COUNT.** `ok: no
  // findings` on its own is the sentence the census exists to deny -- it reads
  // as "your code is fine" when it may mean "nothing was examined".
  if report.findings.is_empty() {
    println!(
      "ok: no {} findings at severity >= {} across {} file(s)",
      report.lang, severity_min, files
    );
    return;
  }

  for sev in [
    Severity::Critical,
    Severity::Warning,
    Severity::Recommendation,
    Severity::Style,
  ] {
    let group: Vec<_> = report
      .findings
      .iter()
      .filter(|f| f.severity == sev)
      .collect();
    if group.is_empty() {
      continue;
    }
    let upper = sev.as_str().to_uppercase();
    println!();
    println!("== {} ({}) ==", upper, group.len());
    for f in group {
      println!(
        "[{}] {} at {}:{}",
        upper,
        f.rule_id,
        f.path.display(),
        f.line_no
      );
      println!("  > {}", f.line);
    }
  }
}

/// The machine face.
///
/// **STDOUT STAYS ONE PARSEABLE DOCUMENT AND THE CENSUS GOES WITH IT, NOT TO
/// STDERR.** v2 pushes the census to stderr under `--format json` to keep
/// stdout clean; that made the honesty half unreadable to exactly the consumer
/// most likely to act on it automatically. Carrying `census` as a field costs
/// the document nothing and keeps "what was asked" attached to "what was found"
/// -- which is the invariant the text face already holds.
/// The value of an enum-typed flag, checked against the vocabulary THE TABLE
/// declares for it.
///
/// **THE ROSTER IS READ, NEVER COPIED, AND THAT IS THE WHOLE POINT OF THE
/// HELPER.** A hand-written `&["text", "json"]` beside a table that declares
/// `text|json` is two literals compared to nothing -- they can drift, and the
/// drift is silent in the direction that matters: the table gains a format,
/// the renderer keeps refusing it, and `--help` advertises a value the command
/// rejects. `export` already takes this shape one layer down by handing the
/// string to the facade rather than defaulting it in the renderer, and its
/// comment gives the reason -- *the default is a fact about the format roster,
/// and a copy of it in the renderer is a second place for it to be wrong*.
///
/// **AN EMPTY ROSTER REFUSES AT 2 RATHER THAN ACCEPTING ANYTHING.** If the
/// table declares no values, this build cannot check what was asked, which is
/// exactly `Unavailable`'s meaning -- the checker is broken and that is ours,
/// not the caller's. Falling through to "accept it then" would turn a missing
/// declaration into a permanently permissive flag -- the empty-roster hazard,
/// where a check whose population went to zero reports a green over nothing.
///
/// **THAT CITATION NAMED `declared_but_unwired.rs:59` FOR ONE HOUR AND I WROTE
/// IT** (ic, 2026-08-21). I cited that file at `11c2037d` and retired it in the
/// next commit, having already spent the afternoon reporting exactly this class
/// in two peers' files. **A doc comment is the one kind of citation nothing
/// here checks** -- `at lint` reads AT rows and sees none of this -- so the
/// dangling reference would have survived every gate in the tree. The rule the
/// instance earns: **cite the PROPERTY, not the file that happens to hold it**,
/// because the property outlives the file and the reader needs the property.
fn enum_flag(a: &ArgMatches, path: &str, spelling: &str) -> Result<String, Failure> {
  let id = spelling.trim_start_matches('-');
  let table = dispatch::table();
  // **BOTH POPULATIONS, CHAINED.** `families` carries the v2-ported commands
  // and `new_surface` the v3-only ones; a lookup over either alone answers
  // confidently about half the surface. Reading only `families` is what made a
  // census of mine report `export` and `events` as undeclared when both are
  // declared -- an absence measured inside the wrong scope.
  let declared: Vec<String> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .find(|e| e.path == path)
    .and_then(|e| {
      e.flags
        .iter()
        .find(|fl| fl.spellings.iter().any(|s| s == spelling))
    })
    .and_then(|fl| fl.value.as_ref())
    .map(|v| v.split('|').map(|s| s.trim().to_string()).collect())
    .unwrap_or_default();
  if declared.is_empty() {
    return Err(Failure::Unavailable(format!(
      "error: the dispatch table declares no values for `{spelling}` on `{path}`, so this build cannot check the one you asked for"
    )));
  }
  // The table's `default` reaches clap, so the flag is normally present. The
  // fallback is the roster's FIRST value rather than a literal, for the same
  // no-second-copy reason as everything above.
  let chosen = a
    .get_one::<String>(id)
    .cloned()
    .unwrap_or_else(|| declared[0].clone());
  if declared.contains(&chosen) {
    Ok(chosen)
  } else {
    Err(Failure::Error(format!(
      "error: `{spelling}={chosen}` is not a value `{path}` declares -- it takes {}",
      declared.join(" or ")
    )))
  }
}

/// `doctor`'s machine face.
///
/// **`findings` is serialised through [`intentsvcs::finding::Finding`]'s own
/// `Serialize`, not hand-mapped here.** `render_critic_json` below hand-maps
/// because `critic::Finding` carries no derive; this one does, so building a
/// second wire shape beside it would be a second home for the same fact and
/// they would drift the first time a field is added.
fn render_doctor_json(report: &intentsvcs::doctor::Report) {
  let doc = serde_json::json!({
    "healthy": report.is_healthy(),
    "findings": report.findings,
    // **THE COVERAGE DENOMINATOR TRAVELS WITH THE VERDICT.** A machine reader
    // has no summary line to fall back on, so dropping these would leave
    // `"findings": []` meaning both *nothing is wrong* and *nothing was read*.
    "checked": {
      "threads": report.threads_checked,
      "issues": report.issues_checked,
      "views": report.views_checked,
      "files": report.files_checked,
    },
    // Inventory rather than faults -- they do not move the exit code. Carried
    // in full and never truncated, which is the rule the text face states in
    // those words: appearing inside a counted group is fine, vanishing is not.
    "unattached": report.unattached,
  });
  // `{:#}` is `Value`'s own pretty Display, so there is no `Result` to unwrap
  // and no branch that could print nothing on a serialisation error.
  println!("{doc:#}");
}

fn render_critic_json(report: &intentsvcs::critic::Report) {
  let findings: Vec<serde_json::Value> = report
    .findings
    .iter()
    .map(|f| {
      serde_json::json!({
        "rule": f.rule_id,
        "severity": f.severity.as_str(),
        "file": f.path.display().to_string(),
        "line": f.line_no,
        "text": f.line,
      })
    })
    .collect();
  let census: Vec<serde_json::Value> = report
    .census
    .iter()
    .map(|r| {
      serde_json::json!({
        "rule": r.rule_id,
        "arming": r.arming.as_str(),
        "disposition": r.disposition.as_str(),
        "by": r.by,
      })
    })
    .collect();
  let doc = serde_json::json!({
    "language": report.lang,
    "asked": report.ran(),
    "armed": report.armed(),
    "total": report.total(),
    "findings": findings,
    "census": census,
    "refused": report.refused,
  });
  println!("{}", serde_json::to_string_pretty(&doc).unwrap_or_default());
}

#[cfg(test)]
mod tests {
  use super::*;

  /// **The MECHANISM half of the unwired claim, split from the ROSTER half.**
  ///
  /// `cli_write_moves_only_what_changed.rs` drives the refusal through the real
  /// binary for the verbs the `DECLARED_BUT_UNWIRED` bucket names. **The roster
  /// question -- WHICH verbs are unwired -- and the mechanism question -- what
  /// `unwired` PRODUCES -- were once answered by a single loop, and only the
  /// first of them needs a real process.** This module takes the second, at the
  /// function, where it can be driven for families that do not exist at all.
  ///
  /// **THE SPLIT IS WHAT LET `declared_but_unwired.rs` RETIRE (2026-08-21).**
  /// Its own header had recorded the loss it could no longer avoid: at ONE
  /// roster member there was nothing left to distinguish a shared-path
  /// regression from a single implementation. Neither half needed the loop --
  /// the roster half moved to explicit named cases, which cannot pass
  /// vacuously the way an iteration over an empty roster can, and the mechanism
  /// half came here, where a synthetic family removes the need to borrow a live
  /// defect as a fixture.
  ///
  /// **A same-module test reaches a private fn, so nothing here widens
  /// visibility.** Making `unwired` `pub(crate)` to test it would have changed
  /// the thing under test to test it.
  ///
  /// # The two arms are a PAIR and neither can pass on the other's text
  ///
  /// `unwired` branches its remedy on whether the family has any shipped verb,
  /// and **both branches end in the same `Unavailable` with the same phrase**,
  /// so an arm asserting only the code and the phrase passes against either.
  /// Each arm therefore asserts its own branch AND the ABSENCE of the other's.
  /// Without that, the branch is untested and the test reads as if it were not.
  #[test]
  fn a_family_that_does_not_exist_refuses_at_two_and_names_the_path() {
    let failure = unwired("not-a-family", "").expect_err("unwired always fails");

    // **The literal 2, deliberately, not `EXIT_UNAVAILABLE`.** The contract is
    // with consumers written against v2, which know the NUMBER; asserting the
    // constant would keep passing if the constant were changed, which is the
    // one regression this pins. Issue 0038 shipped because this path exited 1
    // and every v2-era gate read that as a negative verdict about the user's
    // own work.
    assert_eq!(failure.code(), 2, "an unwired verb must exit 2, never 1");

    let message = failure.message().expect("Unavailable carries a message");
    assert!(
      message.contains("`not-a-family`"),
      "the message must name the path asked for: {message}"
    );
    assert!(
      message.contains("nothing in this build provides it"),
      "a family with no shipped verbs gets the nothing-provides-it remedy: {message}"
    );
    assert!(
      !message.contains("for the verbs that are"),
      "and must NOT get the other branch's remedy, which would send an operator to a --help that does not exist: {message}"
    );
  }

  /// **AT-00.7 / AC-00.7: DISPATCHED RATHER THAN HAND-CALLED, AND THE FIXTURE
  /// HAD TO MOVE.**
  ///
  /// This test used to call `unwired("st", "dehydrate")` BY HAND. `unwired` is
  /// a pure function that interpolates the verb into a message and consults
  /// only whether the FAMILY ships verbs -- **the verb's own wired-ness is
  /// never read.** So the fixture was decorative: the test passed identically
  /// for `st list`, and it kept passing on the day `dehydrate` was wired, which
  /// is the exact moment a test with this name should have gone red.
  ///
  /// **`IN-AG-RED-CONTROL-001` in its quietest form** -- not a control that
  /// broke, but one that never could break, wearing a name that says it does.
  ///
  /// The fixture is now `bootstrap`. `repair` is the other genuinely-unwired
  /// verb in this family and was passed over on purpose: its surface row is
  /// `pending-hv`, so it could move under this test without anybody touching
  /// the test. `bootstrap` is `keep` and hv-ratified.
  #[test]
  fn an_unwired_verb_in_a_wired_family_is_sent_to_that_family() {
    let matches = crate::spine::build(&dispatch::table())
      .try_get_matches_from(["intent", "st", "bootstrap"])
      .expect("`st bootstrap` is DECLARED, so it parses -- only its dispatch arm is missing");
    let failure = run(&matches).expect_err("an unwired verb fails");
    assert_eq!(failure.code(), 2, "an unwired verb must exit 2, never 1");

    let message = failure.message().expect("Unavailable carries a message");
    assert!(
      message.contains("`st bootstrap`"),
      "family and verb are joined with a space when both are present: {message}"
    );
    assert!(
      message.contains("run `intent st --help` for the verbs that are"),
      "a family that DOES ship verbs sends the operator to its help: {message}"
    );
    assert!(
      !message.contains("nothing in this build provides it"),
      "and must NOT claim the family is absent when it is not: {message}"
    );
  }

  /// **AT-00.7's control, and it is the half the old test could not have had.**
  ///
  /// A hand-called `unwired` produces the message above no matter what dispatch
  /// does, so the test above alone still cannot tell "reached `unwired`" from
  /// "`unwired` formats a string". This asserts the inverse through the same
  /// door: a WIRED verb must NOT reach it. **The pair can only both pass if
  /// `st()` is genuinely routing on the verb.**
  ///
  /// `st show` with something that is not a spelling of an id, because
  /// `thread_arg` runs BEFORE `open()` -- so this needs no project, opens no
  /// store, and writes nothing, while still proving the arm was entered.
  #[test]
  fn a_wired_verb_dispatched_the_same_way_does_not_reach_unwired() {
    let matches = crate::spine::build(&dispatch::table())
      .try_get_matches_from(["intent", "st", "show", "not-an-id"])
      .expect("`st show` parses; the argument is refused later, by us");
    let failure = run(&matches).expect_err("`not-an-id` is not a thread spelling");

    let message = failure.message().expect("an id refusal carries a message");
    assert!(
      !message.contains("is a known command that is not implemented yet"),
      "a wired verb must reach its own arm, not the unwired one -- if this fires, \
       dispatch is falling through and the test above is passing for the wrong reason: {message}"
    );
    assert_ne!(
      failure.code(),
      2,
      "exit 2 is `this build cannot answer at all`; a wired verb refusing a bad argument \
       answered, and the answer was no: {message}"
    );
  }
}
