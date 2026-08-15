//! Parse -> facade -> render. The whole of the CLI's logic is routing and
//! formatting; every decision belongs to intentsvcs.
//!
//! **The voice is v2's** (INV-01, issue 0023): lowercase `ok:` / `error:`
//! prefixes, no banners, results on stdout and failures on stderr. INV-06
//! records that about a fifth of v2's failure paths write to the wrong stream;
//! that is a defect being corrected, not a contract being reproduced.

use clap::ArgMatches;
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
    Some(("sync", _)) => sync(),
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
    today: today(),
  };
  Ok((project, ctx))
}

/// Today, ISO 8601, read at the OUTERMOST layer.
///
/// The facade takes the date as an argument and the renderer has no clock at
/// all (D23), so this is the single place the tool asks what day it is.
fn today() -> String {
  time::OffsetDateTime::now_utc()
    .date()
    .format(&time::macros::format_description!("[year]-[month]-[day]"))
    .expect("formatting a date cannot fail")
}

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
/// The direction selector is not spelled yet -- it needs a row in the dispatch
/// table, which is ic's lane -- and this refusal deliberately does NOT name a
/// flag that would not parse. Naming a remedy that does not work is the defect
/// this AC was written about, one artefact over.
fn sync() -> Result<(), String> {
  let f = open()?;
  let overwrite = f.sync_overwrite().map_err(fail)?;
  eprintln!("error: `sync` has two directions and will not guess which one you mean");
  eprintln!("  db -> disk  rewrites the files from the store. Safe: the files are re-creatable");
  eprintln!(
    "  disk -> db  replaces the store from the files. DESTRUCTIVE: any change not yet written to disk is lost"
  );
  if overwrite.is_empty() {
    eprintln!("  (nothing would be overwritten by a disk -> db restore right now)");
  } else {
    eprintln!("  a disk -> db restore would currently overwrite:");
    for line in &overwrite {
      eprintln!("    {line}");
    }
  }
  // D37: the remedy said "owed by WP-06". Which of OUR work packages owes a
  // selector is not something a user of Intent can act on, or should have to
  // read. What they need is the direction that works today.
  eprintln!(
    "  remedy: `intent st sync` runs the safe direction today; an explicit selector for both directions is not built yet"
  );
  Err(String::new())
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
fn unwired(family: &str, verb: &str) -> Result<(), String> {
  let path = if verb.is_empty() {
    family.to_string()
  } else {
    format!("{family} {verb}")
  };
  Err(format!(
    "error: `{path}` is a known command that is not implemented yet\n  remedy: run `intent {family} --help` for the verbs that are"
  ))
}

fn st(m: &ArgMatches) -> Result<(), String> {
  match m.subcommand() {
    Some(("new", a)) => {
      let title = arg(a, "title")?;
      let mut f = open()?;
      let id = f.st_new(&title).map_err(fail)?;
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
      // facade refuses without one. `--reason` is a dispatch-table row and the
      // table is ic's lane, so it is READ optionally here rather than invented:
      // the day ic declares it this starts carrying the operator's text, and
      // until then the facade's `ReasonRequired` says exactly what is missing
      // instead of cancelling a thread with no record of why.
      let reason = opt(a, "reason").unwrap_or_default();
      open()?.st_cancel(&id, &reason).map_err(fail)?;
      println!("ok: {id} cancelled");
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
      let evidence = arg(a, "evidence").unwrap_or_default();
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
      let to = arg(a, "to")?;
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
      let reason = arg(a, "reason")?;
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
  for hit in f.search(&query).map_err(fail)? {
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
  let report = Facade::doctor(&project, &ctx);
  for finding in &report.findings {
    println!("{finding}");
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

/// AC-06.5: print the generated schema faces.
///
/// It does NOT call `open()`. The faces are rendered from types compiled into
/// this binary, so they are the same everywhere and asking for a project would
/// make the command fail in the one place it is most useful -- outside a
/// project, when you are deciding what a project should contain.
fn schema(m: &ArgMatches) -> Result<(), String> {
  match m.try_get_one::<String>("face") {
    Ok(Some(name)) => match intentsvcs::faces::face(name) {
      Some(content) => {
        print!("{content}");
        Ok(())
      }
      None => Err(format!(
        "error: no schema face named `{name}`\n  remedy: one of: {}",
        intentsvcs::faces::face_names().join(", ")
      )),
    },
    Ok(None) => {
      print!("{}", intentsvcs::faces::all_faces_banner());
      Ok(())
    }
    Err(e) => Err(format!(
      "error: the CLI asked for an argument `face` that the dispatch table does not declare\n  caused by: {e}\n  remedy: this is a build defect -- the renderer and surface/dispatch-table.json disagree"
    )),
  }
}

/// `ST0056` or `ST0056/03`.
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
      "error: `{target}` is not a work package\n  remedy: name it as `<ST id>/<NN>`, eg ST0001/03"
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
