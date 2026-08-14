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
use intentsvcs::model::TShirt;
use intentsvcs::project::Project;

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

/// A verb the dispatch table carries and the facade does not yet implement.
///
/// It must NOT say "a command is required" -- one WAS given, and reporting a
/// missing command for a present-but-unwired one is the same
/// same-text-for-different-causes collapse AC-04.4 forbids. The operator needs
/// to know the difference between "you typed nothing" and "we have not built
/// that yet", because only one of them is their problem.
/// It names the work package that OWES the verb, read from the table, rather
/// than a hardcoded WP-06. `intent daemon` is WP-08's and `intent mcp` is
/// WP-09's; a message telling the operator WP-06 owed them would be wrong the
/// first time anyone read it, and wrong in the confident voice of a fact.
fn unwired(family: &str, verb: &str) -> Result<(), String> {
  let path = if verb.is_empty() {
    family.to_string()
  } else {
    format!("{family} {verb}")
  };
  let owner = crate::dispatch::owner_of(&crate::dispatch::table(), &path);
  Err(format!(
    "error: `{path}` is in the dispatch table but not yet wired to the facade (ST0056 {owner})\n  remedy: run `intent {family} --help` for the verbs that are"
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
      open()?.st_cancel(&id).map_err(fail)?;
      println!("ok: {id} cancelled");
      Ok(())
    }
    Some(("list", _)) => {
      let f = open()?;
      for t in f.st_list() {
        println!("{}  {}  {}", t.id, status(t.status), t.title);
      }
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
      let seq = f.wp_new(&st, &title, TShirt::M).map_err(fail)?;
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
      for w in f.wp_list(&st).map_err(fail)? {
        println!("WP-{:02}  {:?}  {}", w.seq, w.status, w.title);
      }
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
      "error: `{target}` is not a work package\n  remedy: name it as `<ST id>/<NN>`, eg ST0056/03"
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

fn status(s: intentsvcs::model::ThreadStatus) -> &'static str {
  use intentsvcs::model::ThreadStatus as S;
  match s {
    S::NotStarted => "Not Started",
    S::Wip => "WIP",
    S::Tbc => "TBC",
    S::Hold => "On Hold",
    S::Completed => "Completed",
    S::Cancelled => "Cancelled",
  }
}
