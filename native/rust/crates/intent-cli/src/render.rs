//! Parse -> facade -> render. The whole of the CLI's logic is routing and
//! formatting; every decision belongs to intentsvcs.
//!
//! **The voice is v2's** (INV-01, issue 0023): lowercase `ok:` / `error:`
//! prefixes, no banners, results on stdout and failures on stderr. INV-06
//! records that about a fifth of v2's failure paths write to the wrong stream;
//! that is a defect being corrected, not a contract being reproduced.

use clap::ArgMatches;

use crate::dispatch;
use intentsvcs::contract::Scope;
use intentsvcs::facade::{Facade, FacadeContext, FacadeError};
use intentsvcs::model::{AtStatus, TShirt, ThreadStatus};
use intentsvcs::project::Project;
use intentsvcs::views;

/// Everything a rendered failure says. The facade's own rendering already
/// carries the message, the full cause chain and the remedy (AC-04.4), so this
/// adds nothing and hides nothing.
fn fail(e: FacadeError) -> String {
  e.render()
}

/// Dispatch one parsed invocation.
pub fn run(matches: &ArgMatches) -> Result<(), String> {
  match matches.subcommand() {
    Some(("st", m)) => st(m),
    Some(("wp", m)) => wp(m),
    Some(("ac", m)) => ac(m),
    Some(("at", m)) => at(m),
    Some(("search", m)) => search(m),
    Some(("schema", m)) => schema(m),
    Some(("doctor", _)) => doctor(),
    Some(("ingest", m)) => ingest(m),
    Some(("sync", m)) => sync(m),
    Some(("backup", m)) => backup(m),
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
fn open() -> Result<Facade, String> {
  let (project, ctx) = context()?;
  Facade::open(project, ctx).map_err(fail)
}

/// Locate the project and assemble the ambient context, WITHOUT loading canon
/// into the store.
///
/// Split out from [`open`] because `doctor` needs exactly this much and no
/// more: it has to run on a project that cannot be opened, since that is when
/// someone reaches for it. Every other verb goes on to [`Facade::open`].
fn context() -> Result<(Project, FacadeContext), String> {
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

/// The work-package status vocabulary, in v2's spelling.
fn wp_status(s: intentsvcs::model::WpStatus) -> &'static str {
  use intentsvcs::model::WpStatus as W;
  match s {
    W::NotStarted => "Not Started",
    W::Wip => "WIP",
    W::Done => "Done",
  }
}

/// T-shirt scope, in the canonical short form.
///
/// **A `corrected` divergence, and one v2 could not have avoided.** v2 reads
/// `scope:` as free text, so it renders whatever the file happens to say --
/// and this repository's 129 work packages carry **eleven** spellings:
/// `Small` 56, `Medium` 34, `Large` 8, `L` 8, `XL` 5, `M` 5, `S` 4,
/// `ExtraSmall` 4, `Extra Small` 3, `XS` 1, and `Medium-Large` 1. "As
/// observed" cannot mean reproducing that, because it is not a behaviour --
/// it is the absence of one, which is exactly what modelling the field fixes.
///
/// The short form because it is what the canon says (the enum's own wire
/// spelling) and what the project's sizing convention states, so the column
/// and the file agree. Same shape as the `TBC` / `Not Started` collapse that
/// `views.rs` records.
///
/// **This function is not finished, and the eleventh spelling is why.** I
/// first measured TEN, because I piped the count through `head`, which
/// defaults to ten lines -- and the row it cut off is the one that decides the
/// rule. `Medium-Large` maps to nothing in `XS · S · M · L · XL · XXL`; it
/// sits between two of them, in a CLOSED thread, where hv's carry policy says
/// lossless-by-carrying and never lossy. Normalising it is a guess, blocking
/// violates the policy, dropping is loss. vc's ruling (data-model.md): `scope`
/// carries a MARKED-LEGACY form for a value outside the enum, on this model's
/// own `acceptance_test` precedent -- D05's posture one level down, where an
/// unknown enum VALUE is marked by name exactly as an unknown FIELD is.
///
/// Until that lands in the model, this match is exhaustive over an enum that
/// cannot yet represent every value the corpus holds.
fn scope(s: TShirt) -> &'static str {
  match s {
    TShirt::XS => "XS",
    TShirt::S => "S",
    TShirt::M => "M",
    TShirt::L => "L",
    TShirt::XL => "XL",
    TShirt::XXL => "XXL",
  }
}

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
fn st_table(f: &Facade, a: &ArgMatches) -> Result<String, String> {
  let wanted = match opt(a, "status") {
    Some(spec) => status_filter(&spec)?,
    // v2's default: WIP only. NOT the same as `--status all`.
    None => Some(vec![ThreadStatus::Wip]),
  };
  st_rows(f, a, wanted)
}

/// The index scope: every thread, whatever `--status` would have said.
/// `st sync` has no status filter in v2 -- the index is the whole estate.
fn st_table_all(f: &Facade, a: &ArgMatches) -> Result<String, String> {
  st_rows(f, a, None)
}

fn st_rows(
  f: &Facade,
  a: &ArgMatches,
  wanted: Option<Vec<ThreadStatus>>,
) -> Result<String, String> {
  let markdown = flag(a, "markdown");
  let width = match opt(a, "width").map(|w| w.parse::<usize>()) {
    Some(Ok(n)) if n > 0 => n,
    Some(Err(_)) => return Err("error: --width takes a number of columns".to_string()),
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
        status(t.status).to_string(),
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
fn sync(m: &ArgMatches) -> Result<(), String> {
  match (flag(m, "to-disk"), flag(m, "to-store")) {
    // Both is not "do both": they are opposite directions over the same two
    // endpoints, so running them in either order makes one of them pointless
    // and the other authoritative by accident of ordering.
    (true, true) => Err(
      "error: `--to-disk` and `--to-store` are opposite directions, so naming both chooses neither\n  remedy: run the one whose DESTINATION you mean -- `--to-disk` writes the extract from the store, `--to-store` replaces the store from the extract"
        .to_string(),
    ),
    (true, false) => {
      let mut f = open()?;
      let count = f.sync_to_disk().map_err(fail)?;
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
      let overwrite = f.sync_overwrite().map_err(fail)?;
      if overwrite.is_empty() {
        eprintln!("note: the store and the extract agree; this restore overwrites nothing");
      } else {
        eprintln!("warning: replacing the store from the extract OVERWRITES:");
        for line in &overwrite {
          eprintln!("  {line}");
        }
      }
      let count = f.sync_from_disk().map_err(fail)?;
      println!("ok: store replaced from the extract, {count} thread(s)");
      Ok(())
    }
    (false, false) => {
      let f = open()?;
      let overwrite = f.sync_overwrite().map_err(fail)?;
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
      Err(String::new())
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
/// category that is empty** (ic, measured 2026-08-15). Seventeen commands are
/// unimplemented and **nine of them are leaves with zero verbs** -- `info`,
/// `init`, `bootstrap`, `learn`, `fileindex`, `version`, `export`, `ingest`,
/// `mcp`. On every one, "run `intent <x> --help` for the verbs that are" sends
/// the reader to a help block that lists no verbs at all, so the remedy costs
/// them a command and returns nothing.
///
/// A remedy that cannot be acted on is worse than no remedy: it reads as a lead
/// and spends the reader's next move. The leaf form points at the root surface,
/// which is never empty, and says plainly that nothing here provides it.
///
/// The family/leaf question is asked of the TABLE rather than of a list kept
/// here, so a family that gains or loses its verbs moves between the two forms
/// on its own -- ic's nine is a measurement of today, not a roster to maintain.
fn unwired(family: &str, verb: &str) -> Result<(), String> {
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
  Err(format!(
    "error: `{path}` is a known command that is not implemented yet\n  remedy: {remedy}"
  ))
}

fn st(m: &ArgMatches) -> Result<(), String> {
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
      open()?.st_start(&id).map_err(fail)?;
      println!("ok: {id} started");
      Ok(())
    }
    Some(("done", a)) => {
      let id = arg(a, "id")?;
      open()?.st_done(&id).map_err(fail)?;
      println!("ok: {id} done");
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
      open()?.st_cancel(&id, &reason).map_err(fail)?;
      println!("ok: {id} cancelled");
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
      open()?.st_triage(&id).map_err(fail)?;
      println!("ok: {id} accepted out of triage");
      Ok(())
    }
    Some(("hold", a)) => {
      let id = arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      open()?.st_hold(&id, &reason).map_err(fail)?;
      println!("ok: {id} on hold");
      Ok(())
    }
    Some(("resume", a)) => {
      let id = arg(a, "id")?;
      open()?.st_resume(&id).map_err(fail)?;
      println!("ok: {id} resumed");
      Ok(())
    }
    Some(("reopen", a)) => {
      let id = arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      open()?.st_reopen(&id, &reason).map_err(fail)?;
      println!("ok: {id} reopened");
      Ok(())
    }
    Some(("reinstate", a)) => {
      let id = arg(a, "id")?;
      let reason = opt(a, "reason").unwrap_or_default();
      open()?.st_reinstate(&id, &reason).map_err(fail)?;
      println!("ok: {id} reinstated to the backlog");
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
      println!("status: {}", status(t.status));
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
        let count = f.sync_to_disk().map_err(fail)?;
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
    Some((verb, _)) => unwired("st", verb),
    None => Err("error: a steel thread command is required".to_string()),
  }
}

fn wp(m: &ArgMatches) -> Result<(), String> {
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
      let seq = f.wp_new(&st, &title, TShirt::S).map_err(fail)?;
      println!("created: {st}/{seq:02}");
      Ok(())
    }
    Some(("start", a)) => {
      let (st, seq) = wp_target(a)?;
      open()?.wp_start(&st, seq).map_err(fail)?;
      println!("ok: {st}/{seq:02} started");
      Ok(())
    }
    Some(("done", a)) => {
      let (st, seq) = wp_target(a)?;
      open()?.wp_done(&st, seq).map_err(fail)?;
      println!("ok: {st}/{seq:02} done");
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
      open()?.wp_reopen(&st, seq, &reason).map_err(fail)?;
      println!("ok: {st}/{seq:02} reopened");
      Ok(())
    }
    Some(("unstart", a)) => {
      let (st, seq) = wp_target(a)?;
      open()?.wp_unstart(&st, seq).map_err(fail)?;
      println!("ok: {st}/{seq:02} back to not started");
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
            scope(w.scope).to_string(),
            wp_status(w.status).to_string(),
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
      println!("status: {}", intentsvcs::model::enum_str(&wp.status));
      println!("scope: {}", intentsvcs::model::enum_str(&wp.scope));
      Ok(())
    }
    Some((verb, _)) => unwired("wp", verb),
    None => Err("error: a work package command is required".to_string()),
  }
}

fn ac(m: &ArgMatches) -> Result<(), String> {
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
        Err(String::new())
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
      open()?.ac_satisfy(&st, &id, &evidence).map_err(fail)?;
      println!("ok: {id} satisfied");
      Ok(())
    }
    Some(("unsatisfy", a)) => {
      let st = arg(a, "stid")?;
      let id = arg(a, "acid")?;
      open()?.ac_unsatisfy(&st, &id).map_err(fail)?;
      // The evidence goes with the satisfaction, so the line says so -- a
      // reader who is told only "unsatisfied" has to go and look to find out
      // whether the citation survived (AC-04.6, D32).
      println!("ok: {id} unsatisfied (evidence cleared)");
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
      println!("{}", verdict.line(&target));
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
      open()?
        .ac_descope(&st, &id, &to, by.as_deref(), reason.as_deref())
        .map_err(fail)?;
      println!("ok: {id} descoped to {to}");
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
      open()?
        .ac_withdraw(&st, &id, &reason, by.as_deref())
        .map_err(fail)?;
      println!("ok: {id} withdrawn");
      Ok(())
    }
    Some(("rescope", a)) => {
      let st = arg(a, "stid")?;
      let id = arg(a, "acid")?;
      open()?.ac_rescope(&st, &id).map_err(fail)?;
      println!("ok: {id} back in scope");
      Ok(())
    }
    Some(("reinstate", a)) => {
      let st = arg(a, "stid")?;
      let id = arg(a, "acid")?;
      open()?.ac_reinstate(&st, &id).map_err(fail)?;
      println!("ok: {id} reinstated");
      Ok(())
    }
    Some((verb, _)) => unwired("ac", verb),
    None => Err("error: an acceptance criterion command is required".to_string()),
  }
}

fn at(m: &ArgMatches) -> Result<(), String> {
  match m.subcommand() {
    Some(("list", a)) => {
      let st = arg(a, "stid")?;
      let f = open()?;
      for t in f.at_list(&st).map_err(fail)? {
        println!(
          "{}  {}  covers {}",
          t.id,
          intentsvcs::model::enum_str(&t.status),
          t.covers.join(", ")
        );
      }
      Ok(())
    }
    Some((state @ ("green" | "red" | "na"), a)) => {
      let st = arg(a, "stid")?;
      let id = arg(a, "atid")?;
      let status = match state {
        "green" => AtStatus::Green,
        "red" => AtStatus::Red,
        _ => AtStatus::Na,
      };
      open()?.at_set(&st, &id, status).map_err(fail)?;
      println!("ok: {id} {state}");
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
          "error: `at lint --fix` is not implemented in v3\n  remedy: fix the rows `intent at lint` names -- v2's --fix rewrote what it could parse and left the rest, which is why it is not being carried over".to_string(),
        );
      }
      let f = open()?;
      let findings = f.at_lint(&st).map_err(fail)?;
      for finding in &findings {
        println!("{finding}");
      }
      if findings.is_empty() {
        Ok(())
      } else {
        Err(String::new())
      }
    }
    Some((verb, _)) => unwired("at", verb),
    None => Err("error: an acceptance test command is required".to_string()),
  }
}

/// AC-06.4: full-text search across ST prose, issue bodies and WP text.
///
/// A miss is exit 0 with no output, not an error. "Nothing matched" is a
/// successful search, and v2's own read verbs answer an empty set the same way
/// -- making it a failure would mean every `grep`-shaped use in a script had to
/// special-case the common answer.
fn search(m: &ArgMatches) -> Result<(), String> {
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
fn doctor() -> Result<(), String> {
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
    Err(String::new())
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
fn ingest(a: &ArgMatches) -> Result<(), String> {
  let project = match opt(a, "path") {
    Some(path) => Project::open(std::path::Path::new(&path)).map_err(|e| {
      format!(
        "error: {e}\n  remedy: give `intent ingest` the root of an Intent project -- the directory holding `intent/`, not the markdown itself"
      )
    })?,
    None => context()?.0,
  };
  Facade::ingest_from_md(&project).map_err(fail)?;
  // Unreachable until WP-10 lands the parser, and written anyway: an arm whose
  // success path is a `todo!()` is one refactor away from being a silent
  // success, and the message a migrator will want is the count it moved.
  println!("ok: ingested {}", project.relative(project.root()));
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
fn schema(m: &ArgMatches) -> Result<(), String> {
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
      None => Err(format!(
        "error: no schema face named `{name}`\n  remedy: one of: {}",
        intentsvcs::faces::face_names().join(", ")
      )),
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
    )),
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

fn wp_target(a: &ArgMatches) -> Result<(String, u32), String> {
  let target = arg(a, "specifier")?;
  match scope_of(&target) {
    (st, Scope::WorkPackage(seq)) => Ok((st, seq)),
    _ => Err(format!(
      // The worked example is `ST0000` deliberately (D37): it is the STZero
      // retrofit id, so it names something in the READER's own project rather
      // than a thread in ours.
      "error: `{target}` is not a work package\n  remedy: name it as `<ST id>/<NN>`, eg ST0000/03"
    )),
  }
}

/// Read a declared positional by the name the DISPATCH TABLE gives it.
///
/// `try_get_one` rather than `get_one`, and the distinction is load-bearing:
/// `get_one` PANICS when the id was never declared, so a renderer that asked
/// for a name the table does not carry would crash with a clap internal
/// message and exit 101 -- neither a v2 exit code nor an Intent error. This
/// turns a table/renderer mismatch into a named failure, which is what
/// No Silent Errors asks for at a seam between two things that must agree.
fn arg(m: &ArgMatches, name: &str) -> Result<String, String> {
  match m.try_get_one::<String>(name) {
    Ok(Some(value)) => Ok(value.clone()),
    Ok(None) => Err(format!("error: {name} is required")),
    Err(e) => Err(format!(
      "error: the CLI asked for an argument `{name}` that the dispatch table does not declare\n  caused by: {e}\n  remedy: this is a build defect -- the renderer and surface/dispatch-table.json disagree"
    )),
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

fn status(s: intentsvcs::model::ThreadStatus) -> &'static str {
  use intentsvcs::model::ThreadStatus as S;
  match s {
    S::NotStarted => "Not Started",
    S::Wip => "WIP",
    S::Triage => "Triage",
    S::Hold => "On Hold",
    S::Completed => "Completed",
    S::Cancelled => "Cancelled",
  }
}

/// `intent backup` takes a snapshot; `--list` reports what exists.
///
/// **`--list` is deliberately NOT the health report.** It answers what exists,
/// and one place reports health -- `doctor`. Two commands answering "is my
/// backup all right" is how they come to disagree, and the one a user reaches
/// for first would be the one that never says no.
fn backup(m: &ArgMatches) -> Result<(), String> {
  let facade = open()?;
  let project = facade.project().clone();

  if flag(m, "list") {
    let snapshots = facade
      .store()
      .snapshots()
      .map_err(|e| format!("error: {e}\n  remedy: {}", e.remedy()))?;
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

  let written = intentsvcs::backup::take(&project, facade.store())
    .map_err(|e| format!("error: {e}\n  remedy: {}", e.remedy()))?;
  println!("created: {}", project.relative(&written));

  let retention = intentsvcs::backup::Retention::from_project(&project);
  let removed = intentsvcs::backup::prune(&project, facade.store(), retention)
    .map_err(|e| format!("error: {e}\n  remedy: {}", e.remedy()))?;
  for path in &removed {
    println!("removed: {}", project.relative(path));
  }
  Ok(())
}
