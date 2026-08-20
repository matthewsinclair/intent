//! Parse -> facade -> render. The whole of the CLI's logic is routing and
//! formatting; every decision belongs to intentsvcs.
//!
//! **The voice is v2's** (INV-01, issue 0023): lowercase `ok:` / `error:`
//! prefixes, no banners, results on stdout and failures on stderr. INV-06
//! records that about a fifth of v2's failure paths write to the wrong stream;
//! that is a defect being corrected, not a contract being reproduced.

use clap::ArgMatches;

use crate::dispatch;
use crate::spine::Failure;
use intentsvcs::address;
use intentsvcs::contract::Scope;
use intentsvcs::facade::{EventFilter, Facade, FacadeContext, FacadeError, Outcome};
use intentsvcs::model::{AtStatus, IssueStatus, TShirt, ThreadStatus, enum_str};
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
    Some(("doctor", _)) => doctor(),
    Some(("organize", m)) => organize(m),
    Some(("upgrade", _)) => upgrade(),
    Some(("ingest", m)) => ingest(m),
    Some(("export", m)) => export(m),
    Some(("todo", m)) => todo(m),
    Some(("sync", m)) => sync(m),
    Some(("backup", m)) => backup(m),
    Some(("info", _)) => info(),
    Some(("claude", m)) => claude(m),
    Some(("llm", m)) => llm(m),
    Some(("issues", m)) => issues(m),
    Some(("agents", m)) => agents(m),
    Some(("critic", m)) => critic(m),
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
  let markdown = flag(a, "markdown");
  let width = match opt(a, "width").map(|w| w.parse::<usize>()) {
    Some(Ok(n)) if n > 0 => n,
    Some(Err(_)) => {
      return Err(Failure::Error(
        "error: --width takes a number of columns".to_string(),
      ));
    }
    _ => terminal_width(),
  };

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

  let mode = if markdown {
    views::TableMode::Markdown
  } else {
    views::TableMode::Terminal { fill: width }
  };
  Ok(views::table(ST_COLUMNS, &rows, mode))
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
fn sync_scope(m: &ArgMatches) -> intentsvcs::sync::Scope {
  let ids: Vec<String> = m
    .try_get_many::<String>("id")
    .ok()
    .flatten()
    .map(|vals| vals.cloned().collect())
    .unwrap_or_default();
  if ids.is_empty() {
    intentsvcs::sync::Scope::All
  } else {
    intentsvcs::sync::Scope::Threads(ids)
  }
}

fn sync(m: &ArgMatches) -> Result<(), Failure> {
  let scope = sync_scope(m);
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
      eprintln!(
        "  --to-disk   rewrites the files from the store. Safe: the files are re-creatable"
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
      eprintln!("  remedy: `intent sync --to-disk` is the routine direction");
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

fn st(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("new", a)) => {
      let title = arg(a, "title")?;
      let mut f = open()?;
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
      let id = arg(a, "id")?;
      reported(&open()?.st_start(&id).map_err(fail)?, &id, "started");
      Ok(())
    }
    Some(("done", a)) => {
      let id = arg(a, "id")?;
      reported(&open()?.st_done(&id).map_err(fail)?, &id, "done");
      Ok(())
    }
    Some(("cancel", a)) => {
      let id = arg(a, "id")?;
      // The ratified machine guards `st cancel` with "reason recorded", so the
      // facade refuses without one. `--reason` is read through `opt` rather
      // than `arg` on purpose: the flag is a dispatch-table row and the table
      // is ic's lane, so an absent one must not crash the renderer. When it is
      // absent the facade's `ReasonRequired` says exactly what is missing,
      // instead of cancelling a thread with no record of why.
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.st_cancel(&id, &reason).map_err(fail)?,
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
      let id = arg(a, "id")?;
      reported(
        &open()?.st_triage(&id).map_err(fail)?,
        &id,
        "accepted out of triage",
      );
      Ok(())
    }
    Some(("hold", a)) => {
      let id = arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.st_hold(&id, &reason).map_err(fail)?,
        &id,
        "on hold",
      );
      Ok(())
    }
    Some(("resume", a)) => {
      let id = arg(a, "id")?;
      reported(&open()?.st_resume(&id).map_err(fail)?, &id, "resumed");
      Ok(())
    }
    Some(("reopen", a)) => {
      let id = arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      reported(
        &open()?.st_reopen(&id, &reason).map_err(fail)?,
        &id,
        "reopened",
      );
      Ok(())
    }
    Some(("reinstate", a)) => {
      let id = arg(a, "id")?;
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
    Some(("show", a)) => {
      let id = arg(a, "id")?;
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
    Some(("hydrate", a)) => hydrated(&arg(a, "id")?),
    Some((verb, _)) => unwired("st", verb),
    None => Err("error: a steel thread command is required".into()),
  }
}

fn wp(m: &ArgMatches) -> Result<(), Failure> {
  match m.subcommand() {
    Some(("new", a)) => {
      let st = arg(a, "stid")?;
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
      let seq = f.wp_new(&st, &title, TShirt::S).map_err(fail)?;
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
      let st = arg(a, "stid")?;
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
      print!(
        "{}",
        views::table(
          WP_COLUMNS,
          &rows,
          views::TableMode::Terminal {
            fill: terminal_width()
          }
        )
      );
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
      let target = arg(a, "stid")?;
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
    Some(("satisfy", a)) => {
      let st = arg(a, "stid")?;
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
      let st = arg(a, "stid")?;
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
      let st = arg(a, "stid")?;
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
      let target = arg(a, "stid")?;
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
      let st = arg(a, "stid")?;
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
      let st = arg(a, "stid")?;
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
      let st = arg(a, "stid")?;
      let id = arg(a, "acid")?;
      let mut f = open()?;
      let outcome = f.ac_rescope(&st, &id).map_err(fail)?;
      reported(&outcome, &id, &back_in_scope(&f, &st, &id)?);
      Ok(())
    }
    Some(("reinstate", a)) => {
      let st = arg(a, "stid")?;
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
    Some(("list", a)) => {
      let st = arg(a, "stid")?;
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
      let st = arg(a, "stid")?;
      let id = arg(a, "atid")?;
      let status = match state {
        "green" => AtStatus::Green,
        "red" => AtStatus::Red,
        _ => AtStatus::Na,
      };
      reported(
        &open()?.at_set(&st, &id, status).map_err(fail)?,
        &id,
        &format!("-> {}", status.display()),
      );
      Ok(())
    }
    Some(("lint", a)) => {
      let st = arg(a, "stid")?;
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
fn organize(m: &ArgMatches) -> Result<(), Failure> {
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

fn doctor() -> Result<(), Failure> {
  let (project, ctx) = context()?;
  // **Opened opportunistically, and a failure to open is not reported here.**
  // `doctor` exists to run on a project that cannot be opened, so the store is
  // a bonus rather than a requirement: with one, the backup half of the report
  // is answerable; without one, every other check still runs and the backup
  // question is simply not asked. Reporting "no backup" because the store
  // could not be read would be a confident wrong answer at the moment a user
  // is least able to check it.
  let opened = Facade::open(project.clone(), ctx.clone()).ok();
  let report = Facade::doctor(&project, &ctx, opened.as_ref().map(|f| f.store()));
  for finding in &report.findings {
    println!("{finding}");
  }
  for withheld in withheld_flags() {
    println!("{withheld}");
  }
  // **Named, every one, and NOT as findings.** These files are outside the
  // carried extensions by design, so they are inventory rather than faults and
  // they do not move the exit code. They are printed because the alternative
  // is silence, and silence is what lets a file vanish when the disk stops
  // being the place things live.
  if !report.unattached.is_empty() {
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
    "doctor: {} finding(s) across {} thread(s), {} issue(s), {} view(s), {} file(s)",
    report.findings.len(),
    report.threads_checked,
    report.issues_checked,
    report.views_checked,
    report.files_checked
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
      let json = flag(m, "json") || m.subcommand().is_some_and(|(_, a)| flag(a, "json"));
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
        print!("{}", f.todo_view_windowed().map_err(fail)?);
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
  let Some(spec) = opt(a, "specifier") else {
    return Err(
      "error: `todo done` needs something to do\n  remedy: name a thread or work package (`intent todo done ST0000`, `ST0000/02`)"
        .into(),
    );
  };
  let mut f = open()?;
  // `scope_of` already owns "is this a thread or a work package": `ac gate`
  // and `wp_target` both parse specifiers through it, and a second reading
  // of `ST0001/02` here is a second place for the answer to differ.
  let outcome = match scope_of(&spec) {
    (st, Scope::Thread) => f.st_done(&st).map_err(fail)?,
    (st, Scope::WorkPackage(seq)) => f.wp_done(&st, seq).map_err(fail)?,
  };
  // **This was the FIRST arm to read the outcome and it shipped the wrong
  // spelling** -- `ok: {spec} was already done`, which named the verb rather than
  // the state. It went through `reported` with the other eighteen when issue 0050
  // settled the house form, and the correction direction is worth recording: the
  // NEWER spelling lost to the one with a v2 antecedent, which is the right way
  // round for a Highlander tie-break.
  reported(&outcome, &spec, "done");
  Ok(())
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
fn export(a: &ArgMatches) -> Result<(), Failure> {
  let f = open()?;
  // `None` when the flag is absent, which the facade reads as the roster's
  // declared default. Not defaulted here: the default is a fact about the
  // format roster, and a copy of it in the renderer is a second place for it
  // to be wrong.
  let text = f.export(opt(a, "format").as_deref()).map_err(fail)?;
  print!("{text}");
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
fn withheld_flags() -> Vec<String> {
  let table = dispatch::table();
  let mut out = Vec::new();
  for entry in dispatch::shipped_entries(&table) {
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
  let target = arg(a, "specifier")?;
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
      TShirt::ALL
        .iter()
        .map(enum_str)
        .collect::<Vec<String>>()
        .join(", ")
    ))
  })
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
      print!(
        "{}",
        views::table(
          ISSUE_COLUMNS,
          &rows,
          views::TableMode::Terminal {
            fill: terminal_width()
          }
        )
      );
      Ok(())
    }
    Some(("show", a)) => {
      let raw = arg(a, "id")?;
      let number = issue_number(&raw)?;
      let f = open()?;
      let issue = f.issue_show(number).map_err(fail)?;
      if flag(a, "json") {
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
      let reporter = reporter();
      let mut f = open()?;
      let number = f
        .issue_add(&title, severity.as_deref(), reporter.as_deref())
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
      let number = issue_number(&arg(a, "id")?)?;
      reported(
        &open()?.issue_close(number).map_err(fail)?,
        &format!("issue {number:04}"),
        "-> CLOSED",
      );
      Ok(())
    }
    Some(("open", a)) => {
      let number = issue_number(&arg(a, "id")?)?;
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
    // `declared_but_unwired.rs` drives whatever is in that state. What changed
    // is that no `issues` verb is in it today.
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
  match outcome.already() {
    None => println!("ok: {subject} {moved}"),
    Some(state) => println!("ok: {subject} already {state}"),
  }
}

/// An operator's spelling of an issue id, as a number.
///
/// **`21`, `0021` and `0021.json` are one issue** -- v2 normalises the same way
/// (`bin/intent_issues:normalize_id`), and an operator who copied a padded id
/// out of a filename must not be told it does not exist.
fn issue_number(raw: &str) -> Result<u32, Failure> {
  let trimmed = raw.trim().trim_end_matches(".json").trim_end_matches(".md");
  trimmed.trim_start_matches('0').parse::<u32>().or_else(|_| {
    // All zeros trims to nothing, and `0000` is a legal id shape even if no
    // project uses it -- answering it with a PARSE error would blame the
    // operator's spelling for an issue that is merely absent.
    if !trimmed.is_empty() && trimmed.chars().all(|c| c == '0') {
      Ok(0)
    } else {
      Err(Failure::Error(format!(
        "error: `{raw}` is not an issue id\n  remedy: name it as a number, eg `21` or `0021`"
      )))
    }
  })
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
    Some((verb, _)) => unwired("claude", verb),
    None => unwired("claude", ""),
  }
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

  println!(
    "{:<22} {:<14} {:<10} {:<14} {:<14} {}",
    "id", "severity", "language", "category", "prov", "title"
  );
  println!(
    "{:<22} {:<14} {:<10} {:<14} {:<14} {}",
    "--", "--------", "--------", "--------", "----", "-----"
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
  use std::os::unix::process::CommandExt;

  let name = arg(a, "name")?;
  if !intentsvcs::install::HOOKS.contains(&name.as_str()) {
    return Err(Failure::Error(format!(
      "error: unknown hook: {name}\n  remedy: one of {}",
      intentsvcs::install::HOOKS.join(", ")
    )));
  }
  let home = intentsvcs::install::home().map_err(|e| Failure::Error(format!("error: {e}")))?;
  let script = intentsvcs::install::hook_script(&home, &name);
  // Named rather than left to the shell. Claude Code surfaces a hook's stderr,
  // so this is the difference between "Intent's install is incomplete" and an
  // opaque 127 the operator cannot act on.
  if !script.is_file() {
    return Err(Failure::Error(format!(
      "error: hook script not found: {}\n  remedy: the Intent install is incomplete -- reinstall it",
      script.display()
    )));
  }
  let e = std::process::Command::new("bash").arg(&script).exec();
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

/// An optional value, ABSENT rather than fatal when this subcommand does not
/// declare it.
///
/// `get_one` panics on an undeclared id -- exit 101, neither a v2 code nor an
/// Intent error -- so a helper shared by two subcommands cannot use it. That
/// is not hypothetical: `st list` and `st sync` share a renderer, `st sync`
/// declares no `--markdown`, and the shared code panicked the moment it asked.
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

  let json = m.get_one::<String>("format").map(|f| f == "json").unwrap_or(false);
  if json {
    render_critic_json(&report);
  } else {
    render_critic_text(&report, files.len(), severity_min.as_str());
  }

  // **REFUSAL OUTRANKS FINDINGS, AND BOTH BLOCK.** See `Report::exit_code`.
  match report.exit_code() {
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

  let declared = report.census.iter().filter(|r| r.arming == Arming::Declared).count();
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
      println!("[{}] {} at {}:{}", upper, f.rule_id, f.path.display(), f.line_no);
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
