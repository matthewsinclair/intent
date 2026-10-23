//! **`intent guards` -- a project's declared commit guards, as the gate reads
//! them (issue 0616).**
//!
//! The gate read `guards` out of `intent/.config/config.json` with its own jq
//! program and its own rules, beside the binary's `Config.guards`. This verb
//! prints the declaration one line per guard, `<index> US <when> US <run[0]>
//! [US <run[n]> ...]`, validated once in intentsvcs, and the gate runs what it
//! prints. Each arm below is one of the spec's: the line itself, nothing
//! declared, and each refusal with its remedy.

use std::process::Output;

/// A project in a temp directory whose config declares `guards` as written
/// (a JSON value), or declares none when `guards` is `None`.
fn project(guards: Option<&str>) -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir");
  let guards = guards
    .map(|g| format!(",\n  \"guards\": {g}"))
    .unwrap_or_default();
  std::fs::write(
    config.join("config.json"),
    format!(
      "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Guards\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]{guards}\n}}\n"
    ),
  )
  .expect("write config");
  dir
}

fn guards_in(dir: &tempfile::TempDir) -> Output {
  crate::common::intent()
    .arg("guards")
    .current_dir(dir.path())
    .stdin(testkit::lifeline_for(&["guards"]))
    .output()
    .expect("run the v3 binary")
}

fn stdout(out: &Output) -> String {
  String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
  String::from_utf8_lossy(&out.stderr).into_owned()
}

/// **THE LINE IS THE GATE'S, BYTE FOR BYTE.** US separates the fields, an
/// argument with a space in it stays one field, and the empty `when` of a guard
/// that applies to every commit keeps its field -- the two separators side by
/// side are what a tab could not hold.
#[test]
fn a_declared_guard_prints_its_line_with_us_separators() {
  let dir = project(Some(
    r#"[{"run": ["bin/hooks/check", "--strict", "a b"], "when": "docs"}, {"run": ["bin/hooks/always"]}]"#,
  ));
  let out = guards_in(&dir);
  assert!(out.status.success(), "stderr: {}", stderr(&out));
  assert_eq!(
    stdout(&out),
    "0\u{1f}docs\u{1f}bin/hooks/check\u{1f}--strict\u{1f}a b\n1\u{1f}\u{1f}bin/hooks/always\n"
  );
}

/// No `guards` key and an empty array are both "none declared": nothing on
/// stdout, at 0, so the gate has nothing to run and nothing to refuse.
#[test]
fn nothing_declared_prints_nothing_at_zero() {
  for guards in [None, Some("[]")] {
    let dir = project(guards);
    let out = guards_in(&dir);
    assert!(out.status.success(), "{guards:?}: {}", stderr(&out));
    assert_eq!(stdout(&out), "", "{guards:?} declares no guard");
  }
}

/// **AN EMPTY `run` IS REFUSED, NAMED, WITH THE SPEC'S REMEDY**, where serde
/// accepted it and only the gate's jq refused it.
#[test]
fn an_empty_run_is_refused_with_its_remedy() {
  let dir = project(Some(r#"[{"run": []}]"#));
  let out = guards_in(&dir);
  assert_eq!(out.status.code(), Some(1), "stdout: {}", stdout(&out));
  let err = stderr(&out);
  assert!(
    err.contains("error: guards[0] in intent/.config/config.json has an empty run"),
    "{err}"
  );
  assert!(
    err.contains(
      "remedy: give guards[0] a run whose first element is a tracked, executable file, or remove the entry"
    ),
    "{err}"
  );
}

/// **WHAT A LINE CANNOT CARRY IS REFUSED, NAMING THE FIELD**: a control
/// character in an argument or in `when`, and an empty argument, which bash
/// 3.2's `read -a` would drop at the end of a line.
#[test]
fn a_field_a_line_cannot_carry_is_refused_naming_it() {
  for (guards, field) in [
    (r#"[{"run": ["bin/x", "a\nb"]}]"#, "guards[0].run[1]"),
    (
      r#"[{"run": ["bin/x"], "when": "do\u001fcs"}]"#,
      "guards[0].when",
    ),
    (
      r#"[{"run": ["bin/x"]}, {"run": ["bin/y", ""]}]"#,
      "guards[1].run[1]",
    ),
  ] {
    let dir = project(Some(guards));
    let out = guards_in(&dir);
    assert_eq!(out.status.code(), Some(1), "{guards}: {}", stdout(&out));
    let err = stderr(&out);
    assert!(err.contains(&format!("error: {field} ")), "{guards}: {err}");
    assert!(err.contains("remedy: "), "{guards}: {err}");
    assert_eq!(stdout(&out), "", "{guards}: a refusal prints no line");
  }
}

/// **A MISSPELT KEY IS REFUSED** (vc's ruling on 0616): `wehn` parsed as a
/// guard with no `when`, which runs on every commit. serde now refuses it,
/// at discovery, with the config error's position and remedy.
#[test]
fn an_unknown_key_in_a_guard_is_refused() {
  let dir = project(Some(r#"[{"run": ["bin/x"], "wehn": "docs"}]"#));
  let out = guards_in(&dir);
  assert_eq!(out.status.code(), Some(1), "stdout: {}", stdout(&out));
  let err = stderr(&out);
  assert!(err.contains("unknown field `wehn`"), "{err}");
  assert!(err.contains("is not valid Intent config"), "{err}");
  assert!(
    err.contains("remedy: the parse error above names the position"),
    "{err}"
  );
}
