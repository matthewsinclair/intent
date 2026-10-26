//! The host census (ST0086 WP-02): the load, and the running processes of
//! each class the controller declares, attributed to the project whose tree
//! holds each one's working directory. It retires vc's census script.
//!
//! **THE TOOL KNOWS NO PROCESS NAMES, AS IT KNOWS NO LANE NAMES.** A class is
//! a name and a pattern matched against the process's command line, one
//! `census/watches/<class>.json` each under the estate directory, written by
//! [`watch`] and removed by [`unwatch`]. Every declared class is a row, idle
//! ones at zero, so a declared class never drops off the page (hv,
//! 2026-10-24).
//!
//! **A PROCESS IS COUNTED UNDER EVERY CLASS IT MATCHES.** Classes are the
//! controller's words and may overlap on purpose (eg `cargo` and `rust`), and
//! choosing one for it would be the tool knowing what the names mean.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{EstateError, Unreadable, estate_dir};
use crate::userstate::Dirs;
use crate::write_set::{WriteError, write_atomically};

/// The schema of one declared class's file.
pub const WATCH_SCHEMA: &str = "intent/estate-census-watch@1";

/// `ps` by absolute path: a judging run's PATH holds only the tools it links.
const PS: &str = "/bin/ps";

#[cfg(not(target_os = "linux"))]
const LSOF: &str = "/usr/sbin/lsof";

#[cfg(not(target_os = "linux"))]
const SYSCTL: &str = "/usr/sbin/sysctl";

#[cfg(target_os = "linux")]
const LOADAVG: &str = "/proc/loadavg";

/// One declared class, as its file holds it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Watch {
  pub schema: String,
  pub class: String,
  pub pattern: String,
}

/// Running processes of one class in one project, or in none.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectCount {
  pub project: Option<String>,
  pub count: u32,
}

/// One declared class's row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcRow {
  pub class: String,
  pub count: u32,
  pub projects: Vec<ProjectCount>,
}

/// One point of intentd's in-memory ring, a minute apart.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Sample {
  pub at: String,
  pub load1: f64,
  pub slots_running: u32,
}

/// The census as `intent/estate-status@1` carries it.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct Census {
  /// The 1, 5 and 15 minute load averages; empty when they could not be read.
  pub load: Vec<f64>,
  pub procs: Vec<ProcRow>,
  /// Oldest first; the caller's, since only a daemon lives long enough to
  /// keep one.
  pub history: Vec<Sample>,
}

/// `<state>/estate/census/watches`.
pub fn watches_dir(dirs: &Dirs) -> PathBuf {
  estate_dir(dirs).join("census").join("watches")
}

/// A class name is a file name: letters, digits, `-` and `_`.
fn valid_class(class: &str) -> bool {
  !class.is_empty()
    && class
      .chars()
      .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Declares a class, or changes its pattern. Written whole by atomic rename.
pub fn watch(dirs: &Dirs, class: &str, pattern: &str) -> Result<PathBuf, EstateError> {
  if !valid_class(class) {
    return Err(EstateError::BadClass {
      class: class.to_string(),
    });
  }
  Regex::new(pattern).map_err(|e| EstateError::BadPattern {
    pattern: pattern.to_string(),
    why: e.to_string(),
  })?;
  let dir = watches_dir(dirs);
  let path = dir.join(format!("{class}.json"));
  let unwritable = |cause: WriteError| EstateError::Unwritable {
    path: path.clone(),
    cause,
  };
  let watch = Watch {
    schema: WATCH_SCHEMA.to_string(),
    class: class.to_string(),
    pattern: pattern.to_string(),
  };
  let mut text = serde_json::to_string_pretty(&watch).map_err(|e| {
    unwritable(WriteError::Io {
      path: path.display().to_string(),
      source: std::io::Error::other(e),
    })
  })?;
  text.push('\n');
  std::fs::create_dir_all(&dir)
    .map_err(|e| WriteError::Io {
      path: dir.display().to_string(),
      source: e,
    })
    .and_then(|()| write_atomically(&path, text.as_bytes()))
    .map_err(unwritable)?;
  Ok(path)
}

/// Removes a declared class. Removing one that is not declared is not an
/// error, so a repeat is the same state; the answer says which it was.
pub fn unwatch(dirs: &Dirs, class: &str) -> Result<bool, EstateError> {
  if !valid_class(class) {
    return Err(EstateError::BadClass {
      class: class.to_string(),
    });
  }
  let path = watches_dir(dirs).join(format!("{class}.json"));
  match std::fs::remove_file(&path) {
    Ok(()) => Ok(true),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
    Err(cause) => Err(EstateError::Unremovable { path, cause }),
  }
}

/// One watch file, or why it is unreadable.
pub fn parse_watch(path: &Path, text: &str) -> Result<(Watch, Regex), Unreadable> {
  let unreadable = |why: String| Unreadable {
    path: path.display().to_string(),
    why,
    pid: None,
  };
  let watch: Watch =
    serde_json::from_str(text).map_err(|e| unreadable(format!("not a census watch: {e}")))?;
  if watch.schema != WATCH_SCHEMA {
    return Err(unreadable(format!(
      "watch schema `{}` is not `{WATCH_SCHEMA}`",
      watch.schema
    )));
  }
  let named = path.file_stem().and_then(|s| s.to_str());
  if named != Some(watch.class.as_str()) {
    return Err(unreadable(format!(
      "declares class `{}` in a file named for another",
      watch.class
    )));
  }
  let pattern =
    Regex::new(&watch.pattern).map_err(|e| unreadable(format!("pattern does not compile: {e}")))?;
  Ok((watch, pattern))
}

/// Every declared class in name order, and every file that could not be read.
fn read_watches(dirs: &Dirs, unreadable: &mut Vec<Unreadable>) -> Vec<(Watch, Regex)> {
  let Ok(entries) = std::fs::read_dir(watches_dir(dirs)) else {
    return Vec::new();
  };
  let mut paths: Vec<PathBuf> = entries
    .filter_map(Result::ok)
    .map(|e| e.path())
    .filter(|p| p.extension().is_some_and(|x| x == "json"))
    .collect();
  paths.sort();
  paths
    .into_iter()
    .filter_map(|path| {
      let read = std::fs::read_to_string(&path).map_err(|e| Unreadable {
        path: path.display().to_string(),
        why: e.to_string(),
        pid: None,
      });
      match read.and_then(|text| parse_watch(&path, &text)) {
        Ok(watch) => Some(watch),
        Err(u) => {
          unreadable.push(u);
          None
        }
      }
    })
    .collect()
}

/// `ps -axo pid=,args=` as each process's command line.
pub fn parse_ps_args(out: &str) -> Vec<(u32, String)> {
  out
    .lines()
    .filter_map(|line| {
      let line = line.trim_start();
      let (pid, args) = line.split_once(char::is_whitespace)?;
      Some((pid.parse().ok()?, args.trim().to_string()))
    })
    .collect()
}

/// `lsof -a -d cwd -p <pids> -Fn` as each pid's working directory: `p` lines
/// name the pid, `n` lines the path, and the `f` lines between are ignored.
pub fn parse_lsof_cwd(out: &str) -> BTreeMap<u32, PathBuf> {
  let mut cwds = BTreeMap::new();
  let mut pid = None;
  for line in out.lines() {
    if let Some(p) = line.strip_prefix('p') {
      pid = p.parse().ok();
    } else if let (Some(n), Some(p)) = (line.strip_prefix('n'), pid) {
      cwds.insert(p, PathBuf::from(n));
    }
  }
  cwds
}

/// The three load averages from `sysctl -n vm.loadavg` (`{ 1.0 2.0 3.0 }`)
/// or `/proc/loadavg` (`1.0 2.0 3.0 2/400 1234`): the first three numbers
/// either way.
pub fn parse_load(out: &str) -> Option<Vec<f64>> {
  let load: Vec<f64> = out
    .split_whitespace()
    .filter(|w| *w != "{" && *w != "}")
    .take(3)
    .map(str::parse)
    .collect::<Result<_, _>>()
    .ok()?;
  (load.len() == 3).then_some(load)
}

/// The rows, one per declared class in the order given. Pure: `project_of`
/// answers which project a working directory stands in.
pub fn count(
  watches: &[(Watch, Regex)],
  procs: &[(u32, String)],
  cwds: &BTreeMap<u32, PathBuf>,
  project_of: &mut impl FnMut(&Path) -> Option<String>,
) -> Vec<ProcRow> {
  watches
    .iter()
    .map(|(watch, pattern)| {
      let mut by: BTreeMap<Option<String>, u32> = BTreeMap::new();
      for (pid, _) in procs.iter().filter(|(_, args)| pattern.is_match(args)) {
        let project = cwds.get(pid).and_then(|cwd| project_of(cwd));
        *by.entry(project).or_default() += 1;
      }
      let mut projects: Vec<ProjectCount> = by
        .into_iter()
        .map(|(project, count)| ProjectCount { project, count })
        .collect();
      // Named projects first, by name; the unattributed after them.
      projects
        .sort_by(|a, b| (a.project.is_none(), &a.project).cmp(&(b.project.is_none(), &b.project)));
      ProcRow {
        class: watch.class.clone(),
        count: projects.iter().map(|p| p.count).sum(),
        projects,
      }
    })
    .collect()
}

/// The census on this host now, with `history` as the caller kept it. A tool
/// that cannot answer is listed in `unreadable` and its part left empty.
pub fn read(dirs: &Dirs, history: Vec<Sample>, unreadable: &mut Vec<Unreadable>) -> Census {
  let load = read_load().unwrap_or_else(|u| {
    unreadable.push(u);
    Vec::new()
  });
  let watches = read_watches(dirs, unreadable);
  let procs = if watches.is_empty() {
    Vec::new()
  } else {
    read_procs().unwrap_or_else(|u| {
      unreadable.push(u);
      Vec::new()
    })
  };
  let matched: Vec<u32> = procs
    .iter()
    .filter(|(_, args)| watches.iter().any(|(_, p)| p.is_match(args)))
    .map(|(pid, _)| *pid)
    .collect();
  let cwds = read_cwds(&matched).unwrap_or_else(|u| {
    unreadable.push(u);
    BTreeMap::new()
  });
  let mut seen: BTreeMap<PathBuf, Option<String>> = BTreeMap::new();
  let mut project_of = |cwd: &Path| {
    seen
      .entry(cwd.to_path_buf())
      .or_insert_with(|| super::project_name_at(cwd))
      .clone()
  };
  Census {
    load,
    procs: count(&watches, &procs, &cwds, &mut project_of),
    history,
  }
}

/// One point for intentd's ring: the 1-minute load now, stamped by the one
/// estate clock (hv's ruling, 2026-10-24, vc decision 72).
pub fn sample(slots_running: u32) -> Result<Sample, Unreadable> {
  let load = read_load()?;
  Ok(Sample {
    at: super::iso(super::clock::now()).unwrap_or_default(),
    load1: load[0],
    slots_running,
  })
}

/// The 1-minute load now, for a lane's load admission (ST0086 WP-04).
pub fn load1() -> Result<f64, Unreadable> {
  read_load()?
    .first()
    .copied()
    .ok_or_else(|| unread("load", "the load read held no averages".to_string()))
}

/// A tool or file the census could not read, listed against it.
fn unread(path: &str, why: String) -> Unreadable {
  Unreadable {
    path: path.to_string(),
    why,
    pid: None,
  }
}

#[cfg(target_os = "linux")]
fn read_load() -> Result<Vec<f64>, Unreadable> {
  let text = std::fs::read_to_string(LOADAVG)
    .map_err(|e| unread(LOADAVG, format!("`{LOADAVG}` could not be read: {e}")))?;
  parse_load(&text).ok_or_else(|| unread(LOADAVG, format!("`{LOADAVG}` holds no load averages")))
}

#[cfg(not(target_os = "linux"))]
fn read_load() -> Result<Vec<f64>, Unreadable> {
  let out = Command::new(SYSCTL)
    .args(["-n", "vm.loadavg"])
    .output()
    .map_err(|e| unread(SYSCTL, format!("`{SYSCTL}` could not run: {e}")))?;
  let text = String::from_utf8_lossy(&out.stdout);
  parse_load(&text).ok_or_else(|| {
    unread(
      SYSCTL,
      format!("`{SYSCTL} -n vm.loadavg` named no load averages"),
    )
  })
}

fn read_procs() -> Result<Vec<(u32, String)>, Unreadable> {
  let out = Command::new(PS)
    .args(["-axo", "pid=,args="])
    .env("LC_ALL", "C")
    .output()
    .map_err(|e| unread(PS, format!("`{PS}` could not run: {e}")))?;
  if !out.status.success() {
    return Err(unread(
      PS,
      format!(
        "`{PS} -axo pid=,args=` failed: {}",
        String::from_utf8_lossy(&out.stderr).trim()
      ),
    ));
  }
  Ok(parse_ps_args(&String::from_utf8_lossy(&out.stdout)))
}

/// Each pid's working directory. A process gone or not ours to read has none
/// and counts as in no project.
#[cfg(target_os = "linux")]
fn read_cwds(pids: &[u32]) -> Result<BTreeMap<u32, PathBuf>, Unreadable> {
  Ok(
    pids
      .iter()
      .filter_map(|pid| {
        std::fs::read_link(format!("/proc/{pid}/cwd"))
          .ok()
          .map(|cwd| (*pid, cwd))
      })
      .collect(),
  )
}

#[cfg(not(target_os = "linux"))]
fn read_cwds(pids: &[u32]) -> Result<BTreeMap<u32, PathBuf>, Unreadable> {
  if pids.is_empty() {
    return Ok(BTreeMap::new());
  }
  let pids: Vec<String> = pids.iter().map(u32::to_string).collect();
  let out = Command::new(LSOF)
    .args(["-a", "-d", "cwd", "-p", &pids.join(","), "-Fn"])
    .output()
    .map_err(|e| unread(LSOF, format!("`{LSOF}` could not run: {e}")))?;
  // `lsof` exits 1 when a listed pid has gone, which is an answer; nothing on
  // stdout with something on stderr is not.
  let stdout = String::from_utf8_lossy(&out.stdout);
  let stderr = String::from_utf8_lossy(&out.stderr);
  if !out.status.success() && stdout.trim().is_empty() && !stderr.trim().is_empty() {
    return Err(unread(LSOF, format!("`{LSOF}` failed: {}", stderr.trim())));
  }
  Ok(parse_lsof_cwd(&stdout))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn watch(class: &str, pattern: &str) -> (Watch, Regex) {
    (
      Watch {
        schema: WATCH_SCHEMA.to_string(),
        class: class.to_string(),
        pattern: pattern.to_string(),
      },
      Regex::new(pattern).unwrap(),
    )
  }

  #[test]
  fn both_load_forms_read_as_three_numbers() {
    assert_eq!(
      parse_load("{ 12.72 18.64 18.76 }\n"),
      Some(vec![12.72, 18.64, 18.76])
    );
    assert_eq!(
      parse_load("0.52 0.58 0.59 2/412 4021\n"),
      Some(vec![0.52, 0.58, 0.59])
    );
    assert_eq!(parse_load("{ 1.0 }"), None);
  }

  #[test]
  fn lsof_field_output_maps_each_pid_to_its_cwd() {
    let out = "p60698\nfcwd\nn/Users/someone/Devel/prj/A\np61213\nfcwd\nn/tmp/b\n";
    let cwds = parse_lsof_cwd(out);
    assert_eq!(
      cwds.get(&60698),
      Some(&PathBuf::from("/Users/someone/Devel/prj/A"))
    );
    assert_eq!(cwds.get(&61213), Some(&PathBuf::from("/tmp/b")));
  }

  #[test]
  fn ps_args_keep_the_whole_command_line() {
    let out = "  101 /usr/bin/cargo test --workspace\n 2002 bats -r tests/\n";
    assert_eq!(
      parse_ps_args(out),
      vec![
        (101, "/usr/bin/cargo test --workspace".to_string()),
        (2002, "bats -r tests/".to_string())
      ]
    );
  }

  #[test]
  fn every_declared_class_is_a_row_idle_ones_at_zero() {
    let watches = [
      watch("bats", r"(^|/)bats( |$)"),
      watch("cargo", r"(^|/)cargo( |$)"),
    ];
    let procs = vec![
      (1, "/x/cargo test".to_string()),
      (2, "cargo build".to_string()),
      (3, "cargo clippy".to_string()),
      (4, "vim cargo.toml".to_string()),
    ];
    let cwds = BTreeMap::from([
      (1, PathBuf::from("/p/A/sub")),
      (2, PathBuf::from("/p/B")),
      (3, PathBuf::from("/elsewhere")),
    ]);
    let mut project_of = |cwd: &Path| {
      ["A", "B"]
        .into_iter()
        .find(|p| cwd.starts_with(format!("/p/{p}")))
        .map(str::to_string)
    };
    let rows = count(&watches, &procs, &cwds, &mut project_of);
    assert_eq!(
      rows,
      vec![
        ProcRow {
          class: "bats".to_string(),
          count: 0,
          projects: vec![],
        },
        ProcRow {
          class: "cargo".to_string(),
          count: 3,
          projects: vec![
            ProjectCount {
              project: Some("A".to_string()),
              count: 1
            },
            ProjectCount {
              project: Some("B".to_string()),
              count: 1
            },
            ProjectCount {
              project: None,
              count: 1
            },
          ],
        },
      ]
    );
  }

  #[test]
  fn a_watch_file_names_its_own_class() {
    let text = format!(r#"{{"schema":"{WATCH_SCHEMA}","class":"cargo","pattern":"cargo"}}"#);
    assert!(parse_watch(Path::new("/w/cargo.json"), &text).is_ok());
    let err = parse_watch(Path::new("/w/bats.json"), &text).unwrap_err();
    assert_eq!(
      err.why,
      "declares class `cargo` in a file named for another"
    );
  }

  #[test]
  fn watch_then_unwatch_round_trips_and_refuses_bad_input() {
    let home = std::env::temp_dir().join(format!("census-watch-{}", std::process::id()));
    let dirs = Dirs::at_home(&home);
    let path = watch_writes(&dirs);
    let mut unreadable = Vec::new();
    let read = read_watches(&dirs, &mut unreadable);
    assert_eq!(read.len(), 1, "{unreadable:?}");
    assert_eq!(read[0].0.class, "cargo");
    assert!(path.ends_with("census/watches/cargo.json"));
    assert!(matches!(
      super::watch(&dirs, "../x", "cargo"),
      Err(EstateError::BadClass { .. })
    ));
    assert!(matches!(
      super::watch(&dirs, "x", "("),
      Err(EstateError::BadPattern { .. })
    ));
    assert!(unwatch(&dirs, "cargo").unwrap());
    assert!(!unwatch(&dirs, "cargo").unwrap());
    let _ = std::fs::remove_dir_all(&home);
  }

  fn watch_writes(dirs: &Dirs) -> PathBuf {
    super::watch(dirs, "cargo", r"(^|/)(cargo|rustc)( |$)").expect("written")
  }
}
