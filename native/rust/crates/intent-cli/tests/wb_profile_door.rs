//! ST0092 WP-02: `intent wb profile` at the process surface, run from a
//! directory that is no Intent project.
//!
//! The profiles belong to the install, and the binary under test resolves its
//! install from its own path, which is this repository. So the expected names,
//! seats and documents are read from the repository's canon here, never written
//! down, and every arm runs in a fresh tempdir holding no project.

use std::path::{Path, PathBuf};
use std::process::Output;

use testkit::repo_root;

fn teams_dir() -> PathBuf {
  repo_root().join("intent/plugins/claude/teams")
}

/// The shipped profile names, read off the directory the way the loader reads
/// it: every `*.md` but the terms document.
fn shipped_names() -> Vec<String> {
  let mut names: Vec<String> = std::fs::read_dir(teams_dir())
    .expect("the teams directory")
    .filter_map(|e| {
      let name = e.expect("entry").file_name().to_string_lossy().into_owned();
      (name != "TERMS.md")
        .then(|| name.strip_suffix(".md").map(str::to_string))
        .flatten()
    })
    .collect();
  names.sort();
  assert!(!names.is_empty(), "the repository ships no profile");
  names
}

/// One run of `intent wb profile <args>` in `cwd`, with `home` as HOME when
/// given.
fn run(cwd: &Path, home: Option<&Path>, args: &[&str]) -> Output {
  let mut cmd = crate::common::intent();
  cmd.current_dir(cwd).arg("wb").arg("profile").args(args);
  if let Some(home) = home {
    cmd.env("HOME", home);
  }
  cmd.output().expect("spawn intent")
}

fn stdout(o: &Output) -> String {
  String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
  String::from_utf8_lossy(&o.stderr).into_owned()
}

fn nowhere() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  assert!(
    !dir.path().join("intent").exists(),
    "the tempdir must hold no project"
  );
  dir
}

/// ST0092 AT-02.1: `intent wb profile validate` passes on the shipped set and
/// names every shipped profile.
#[test]
fn at_02_1_validate_passes_on_the_shipped_set() {
  let dir = nowhere();
  let out = run(dir.path(), None, &["validate"]);
  assert!(
    out.status.success(),
    "validate refused the shipped set: {}{}",
    stdout(&out),
    stderr(&out)
  );
  let names = shipped_names();
  let said = stdout(&out);
  assert!(
    said.starts_with(&format!("ok: {} profile(s) valid: ", names.len())),
    "validate's success line announces its outcome with `ok:` (INV-01): {said:?}"
  );
  for name in &names {
    assert!(
      said.contains(name.as_str()),
      "validate does not name {name}: {said}"
    );
  }
}

/// ST0092 AT-02.3: list names each profile with its kind, show prints its
/// seats, its mediator and its brief, and terms prints the terms document, all
/// in a directory that is no Intent project.
#[test]
fn at_02_3_list_show_and_terms_answer_outside_a_project() {
  let dir = nowhere();

  let out = run(dir.path(), None, &["list"]);
  assert!(out.status.success(), "{}", stderr(&out));
  let listed = stdout(&out);
  for name in shipped_names() {
    let text = std::fs::read_to_string(teams_dir().join(format!("{name}.md"))).expect("read");
    let kind = if text.contains("\nkind: base\n") {
      "base"
    } else {
      "layer"
    };
    assert!(
      listed
        .lines()
        .any(|l| l.split_whitespace().take(2).eq([name.as_str(), kind])),
      "list does not name {name} as a {kind}: {listed}"
    );
  }

  let out = run(dir.path(), None, &["show", "team"]);
  assert!(out.status.success(), "{}", stderr(&out));
  let shown = stdout(&out);
  let team = std::fs::read_to_string(teams_dir().join("team.md")).expect("read team.md");
  let brief = team
    .split_once("\n---\n")
    .map(|(_, b)| b.trim_start_matches('\n'))
    .expect("team.md has frontmatter");
  assert!(shown.contains("mediator: vc"), "{shown}");
  for moniker in ["vc", "cc", "ic", "ac", "dc", "hv"] {
    assert!(
      shown
        .lines()
        .any(|l| l.split_whitespace().next() == Some(moniker)),
      "show does not list the seat {moniker}: {shown}"
    );
  }
  assert!(
    shown.ends_with(brief),
    "show does not print the brief whole: {shown}"
  );

  let out = run(dir.path(), None, &["terms"]);
  assert!(out.status.success(), "{}", stderr(&out));
  let terms = std::fs::read_to_string(teams_dir().join("TERMS.md")).expect("read TERMS.md");
  assert_eq!(
    stdout(&out),
    terms,
    "terms prints the document byte for byte"
  );
}

/// ST0092 AT-02.6: an unknown profile is refused on exit 1 with the shipped
/// names in its remedy, and a profile planted under the extension base is
/// never read.
#[test]
fn at_02_6_an_unknown_profile_is_refused_and_ext_is_not_read() {
  let dir = nowhere();
  let out = run(dir.path(), None, &["show", "no-such-profile"]);
  assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
  let said = stderr(&out);
  assert!(
    said.contains("no-such-profile") && said.contains("remedy:"),
    "{said}"
  );
  for name in shipped_names() {
    assert!(
      said.contains(name.as_str()),
      "the remedy does not name {name}: {said}"
    );
  }

  let home = tempfile::tempdir().expect("home");
  let planted = "---\nkind: base\nmediator: cc\nseats: [cc, hv]\n---\n\n# Rogue\n";
  for rel in [
    ".local/share/intent/ext/teams",
    ".local/share/intent/ext/plugins/claude/teams",
  ] {
    let at = home.path().join(rel);
    std::fs::create_dir_all(&at).expect("mkdir ext");
    std::fs::write(at.join("rogue.md"), planted).expect("plant");
  }
  let out = run(dir.path(), Some(home.path()), &["list"]);
  assert!(out.status.success(), "{}", stderr(&out));
  assert!(
    !stdout(&out).contains("rogue"),
    "list read the ext base: {}",
    stdout(&out)
  );
  let out = run(dir.path(), Some(home.path()), &["show", "rogue"]);
  assert_eq!(
    out.status.code(),
    Some(1),
    "show read the ext base: {}",
    stdout(&out)
  );
}

/// One run of a copied binary in a fixture install.
fn run_install(exe: &crate::common::CopiedIntent, cwd: &Path, args: &[&str]) -> Output {
  exe.output(
    |p| {
      let mut c = testkit::fixtured_command(p);
      c.current_dir(cwd).arg("wb").arg("profile").args(args);
      c
    },
    "spawn the fixture install's intent",
  )
}

/// The exit codes the row declares that no shipped install reaches, driven on
/// fixture installs: validate exits 1 on a finding naming its file and field,
/// a verb that must read every profile exits 2 when one cannot be read, and
/// every verb exits 2 where the install carries no profiles directory (INV-04,
/// ic's findings 4 and 7 on ST0092 WP-02).
#[test]
fn a_fixture_install_drives_validates_1_and_each_2() {
  let dir = nowhere();

  let broken = tempfile::tempdir().expect("an install with one bad profile");
  let exe = crate::common::fake_install(broken.path());
  let teams = broken.path().join("intent/plugins/claude/teams");
  std::fs::create_dir_all(&teams).expect("mkdir teams");
  for entry in std::fs::read_dir(teams_dir()).expect("the shipped teams") {
    let entry = entry.expect("entry");
    std::fs::copy(entry.path(), teams.join(entry.file_name())).expect("copy a shipped file");
  }
  std::fs::write(
    teams.join("crew.md"),
    "---\nkind: crew\nseats: [cc]\n---\n\n# Crew\n",
  )
  .expect("plant a profile whose kind is neither base nor layer");

  let out = run_install(&exe, dir.path(), &["validate"]);
  assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
  let said = stderr(&out);
  assert!(said.contains("crew.md") && said.contains("kind"), "{said}");

  let out = run_install(&exe, dir.path(), &["list"]);
  assert_eq!(
    out.status.code(),
    Some(2),
    "list over an unreadable profile: {}",
    stderr(&out)
  );

  let bare = tempfile::tempdir().expect("an install with no profiles directory");
  let exe = crate::common::fake_install(bare.path());
  for args in [&["list"][..], &["show", "team"], &["validate"], &["terms"]] {
    let out = run_install(&exe, dir.path(), args);
    assert_eq!(
      out.status.code(),
      Some(2),
      "`wb profile {}` with no profiles directory: {}",
      args.join(" "),
      stderr(&out)
    );
  }
}
