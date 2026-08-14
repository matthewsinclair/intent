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
    Some((family, _)) => Err(format!(
      "error: `{family}` is in the dispatch table but not yet wired to the facade (ST0056 WP-06)\n  remedy: run `intent --help` for the families that are"
    )),
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
  Facade::open(project, ctx).map_err(fail)
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
    _ => Err("error: a steel thread command is required".to_string()),
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
    _ => Err("error: a work package command is required".to_string()),
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
    _ => Err("error: an acceptance criterion command is required".to_string()),
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
    _ => Err("error: an acceptance test command is required".to_string()),
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
      "error: the CLI asked for an argument `{name}` that the dispatch table does not declare\n  caused by: {e}\n  remedy: this is a build defect -- the renderer and intent/st/ST0056/dispatch-table.json disagree"
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
