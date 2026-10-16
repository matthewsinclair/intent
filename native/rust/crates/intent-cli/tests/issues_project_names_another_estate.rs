//! Issue 0590: **`intent issues --project <project>` acts on a registered
//! project other than the working directory's.**
//!
//! Every arm runs from a project of its own (`here`) under a fixture `HOME`
//! whose registry lists three others and one root that no longer opens. The
//! value resolves by root, `project_id`, directory name or `project_name`; two
//! matches and none are refused with the candidates; and nothing ever falls
//! back to `here`, which every arm checks is left byte for byte.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

struct Estate {
  _dir: tempfile::TempDir,
  home: PathBuf,
  here: PathBuf,
  alpha: PathBuf,
  beta: PathBuf,
  gamma: PathBuf,
  stale: PathBuf,
}

fn run(home: &Path, cwd: &Path, args: &[&str]) -> (String, String, i32) {
  let out = crate::common::intent()
    .args(args)
    .env("HOME", home)
    .current_dir(cwd)
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run the v3 binary");
  (
    String::from_utf8_lossy(&out.stdout).into_owned(),
    String::from_utf8_lossy(&out.stderr).into_owned(),
    out.status.code().unwrap_or(-1),
  )
}

/// `alpha-one` and `beta` both declare `project_name` Shared; `gamma` declares
/// Gamma; `here` is the working directory's project and is not registered.
fn estate() -> Estate {
  let dir = tempfile::tempdir().expect("tempdir");
  let base = std::fs::canonicalize(dir.path()).expect("canonical tempdir");
  let home = base.join("home");
  std::fs::create_dir_all(&home).expect("home");
  let project = |dirname: &str, name: &str| {
    let root = base.join(dirname);
    std::fs::create_dir_all(&root).expect("project dir");
    let (_, err, code) = run(&home, &root, &["init", name]);
    assert_eq!(code, 0, "fixture init of {dirname} failed: {err}");
    root
  };
  let here = project("here", "Here");
  let alpha = project("alpha-one", "Shared");
  let beta = project("beta", "Shared");
  let gamma = project("gamma", "Gamma");
  let stale = base.join("gone");
  let config = home.join(".config/intent");
  std::fs::create_dir_all(&config).expect("config dir");
  let roots: Vec<String> = [&alpha, &beta, &gamma, &stale]
    .iter()
    .map(|r| format!("{{\"root\": {:?}}}", r.display().to_string()))
    .collect();
  std::fs::write(
    config.join("projects.json"),
    format!("{{\"schema\": 1, \"projects\": [{}]}}\n", roots.join(", ")),
  )
  .expect("write the registry");
  Estate {
    _dir: dir,
    home,
    here,
    alpha,
    beta,
    gamma,
    stale,
  }
}

/// Every file under `dir`, with its bytes.
fn tree(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
  fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
    for entry in std::fs::read_dir(dir).expect("read the dir") {
      let path = entry.expect("an entry").path();
      if path.is_dir() {
        walk(root, &path, out);
      } else {
        let rel = path
          .strip_prefix(root)
          .expect("under the root")
          .to_path_buf();
        out.insert(rel, std::fs::read(&path).expect("read the file"));
      }
    }
  }
  let mut out = BTreeMap::new();
  walk(dir, dir, &mut out);
  out
}

fn project_id(root: &Path) -> String {
  let text = std::fs::read_to_string(root.join("intent/.config/config.json")).expect("config");
  let config: serde_json::Value = serde_json::from_str(&text).expect("config JSON");
  config["project_id"]
    .as_str()
    .expect("init mints a project_id")
    .to_string()
}

/// Add one issue through `--project wanted`, run from `here`.
fn add_through(e: &Estate, wanted: &str) -> (String, String, i32) {
  run(
    &e.home,
    &e.here,
    &["issues", "--project", wanted, "add", "filed from elsewhere"],
  )
}

fn issue_count(root: &Path) -> usize {
  std::fs::read_dir(root.join("intent/.canon/issues")).map_or(0, |dir| dir.count())
}

fn holds_the_issue(root: &Path) -> bool {
  issue_count(root) > 0
}

/// A directory name, a `project_id` and a path each name one project, and the
/// write lands in that project's own canon while `here` is left byte for byte.
#[test]
fn each_exact_form_and_the_directory_name_reach_the_named_project() {
  let e = estate();
  let before = tree(&e.here);
  let beta_id = project_id(&e.beta);
  let beta_path = e.beta.display().to_string();
  for (wanted, root) in [
    ("GAMMA", &e.gamma),
    ("alpha-one", &e.alpha),
    (beta_id.as_str(), &e.beta),
    (beta_path.as_str(), &e.beta),
  ] {
    let count_before = issue_count(root);
    let (stdout, stderr, code) = add_through(&e, wanted);
    assert_eq!(
      code, 0,
      "`--project {wanted}`: stdout={stdout}\nstderr={stderr}"
    );
    let named = format!(
      "project: {} at {}",
      root.file_name().expect("a name").to_string_lossy(),
      root.display()
    );
    assert_eq!(
      stdout.lines().next(),
      Some(named.as_str()),
      "`--project {wanted}` did not name where it wrote: {stdout}"
    );
    assert_eq!(
      issue_count(root),
      count_before + 1,
      "`--project {wanted}` wrote no issue into {}",
      root.display()
    );
  }
  assert!(
    !holds_the_issue(&e.here),
    "an issue was filed in the working directory's project"
  );
  assert_eq!(
    tree(&e.here),
    before,
    "the working directory's project changed"
  );
}

/// Two projects named `Shared` are refused naming both, and nothing is written.
#[test]
fn a_name_two_projects_carry_is_refused_with_both() {
  let e = estate();
  let before = tree(&e.here);
  let (_, stderr, code) = add_through(&e, "shared");
  assert_ne!(code, 0, "an ambiguous name was accepted: {stderr}");
  assert!(stderr.contains("names more than one"), "{stderr}");
  for root in [&e.alpha, &e.beta] {
    assert!(
      stderr.contains(&root.display().to_string()),
      "the refusal does not name {}: {stderr}",
      root.display()
    );
    assert!(!holds_the_issue(root), "{} was written", root.display());
  }
  assert_eq!(
    tree(&e.here),
    before,
    "the working directory's project changed"
  );
}

/// A name nothing carries is refused listing the registry and the root that
/// no longer opens; nothing falls back to the working directory's project.
#[test]
fn an_unknown_name_is_refused_listing_the_registry_and_the_stale_root() {
  let e = estate();
  let before = tree(&e.here);
  let (_, stderr, code) = add_through(&e, "nowhere");
  assert_ne!(code, 0, "an unknown name was accepted: {stderr}");
  assert!(stderr.contains("names no registered project"), "{stderr}");
  for root in [&e.alpha, &e.beta, &e.gamma, &e.stale] {
    assert!(
      stderr.contains(&root.display().to_string()),
      "the refusal does not list {}: {stderr}",
      root.display()
    );
  }
  assert!(!holds_the_issue(&e.here), "the issue fell back to here");
  assert_eq!(
    tree(&e.here),
    before,
    "the working directory's project changed"
  );
  // A stale root is listed and never matched, even by its own directory name.
  let (_, stderr, code) = add_through(&e, "gone");
  assert_ne!(code, 0, "a stale root was matched: {stderr}");
}

/// A read names the project on stderr, so its JSON on stdout stays one
/// document, and the document carries the project and root as fields.
#[test]
fn a_read_names_the_project_on_stderr_and_in_its_json() {
  let e = estate();
  let (_, err, code) = add_through(&e, "gamma");
  assert_eq!(code, 0, "{err}");
  let (stdout, stderr, code) = run(
    &e.home,
    &e.here,
    &[
      "issues",
      "--project",
      "gamma",
      "show",
      "0001",
      "--format",
      "json",
    ],
  );
  assert_eq!(code, 0, "stdout={stdout}\nstderr={stderr}");
  assert!(
    stderr.contains(&format!("project: gamma at {}", e.gamma.display())),
    "{stderr}"
  );
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is one JSON document");
  assert_eq!(json["title"], "filed from elsewhere");
  assert_eq!(json["project"], "gamma");
  assert_eq!(json["root"], e.gamma.display().to_string());
}

/// `--daemon` with `--project` is refused: no issues verb is served by a
/// daemon, and a daemon serves one project.
#[test]
fn daemon_with_project_is_refused() {
  let e = estate();
  let (_, stderr, code) = run(
    &e.home,
    &e.here,
    &["--daemon", "issues", "--project", "gamma", "add", "routed"],
  );
  assert_ne!(code, 0, "--daemon with --project was accepted: {stderr}");
  assert!(
    !holds_the_issue(&e.gamma),
    "the daemon route wrote an issue"
  );
}
