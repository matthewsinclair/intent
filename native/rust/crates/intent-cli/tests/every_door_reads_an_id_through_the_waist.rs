//! Issue `0713` (ST0095 WP-10): **a door that reads an id reads it through the
//! waist, so a short spelling names the same record everywhere.** `todo done`
//! handed its specifier to `scope_of` raw, so `todo done 56/02` named "no steel
//! thread 56" where `wp done 56/02` works; `ac descope --to` compared its
//! target raw, so `--to 59` was "not a steel thread" where every other door
//! reads 59 as ST0059; and `events --subject` matched the subject as typed, so
//! `--subject 94` matched nothing. A filter over history reads an untagged
//! number as every record it could name (cc's meaning, 2026-11-20).
//!
//! The short spellings are derived from the minted ids: the tagged one from the
//! waist (`ThreadId::tagged`, `IssueId::tagged`), and the bare number and the
//! unpadded package by trimming the waist's own outputs, never from a literal.
//!
//! `intent id` (AC-10.6) is the read door a script asks in place of a pattern
//! of its own, so it answers through the same resolver as every verb's door.

use crate::common::{created, ok, run};
use intentsvcs::id::{IssueId, ThreadId, WpId};
use serde_json::Value;

/// A minted thread's bare number, eg `1` for `ST0001`: its tagged spelling
/// with the tag trimmed off.
fn bare(stored: &str) -> String {
  let thread = ThreadId::parse(stored).expect("a minted thread id");
  thread
    .tagged()
    .trim_start_matches(char::is_alphabetic)
    .to_string()
}

#[test]
fn todo_done_reaches_the_package_a_short_spelling_names() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let st = created(&ok(root, &["st", "new", "A thread the doors name"]));
  let wp = created(&ok(
    root,
    &["wp", "new", st.as_str(), "A package the doors name"],
  ));
  let (_, package) = intentsvcs::id::split_wp_address(&wp).expect("a package address");
  let package = WpId::parse(package).expect("a minted package id");
  let short_wp = format!("{}/{}", bare(&st), package.shown().trim_start_matches('0'));

  // The package is not started, so `wp.done` is refused by the transition
  // machine, and the refusal names the package it read: before the fix it
  // named "no steel thread" by the short spelling's head.
  let (out, err, code) = run(root, &["todo", "done", short_wp.as_str()]);
  assert_eq!(code, 1, "`todo done {short_wp}`:\n{out}{err}");
  assert!(
    err.contains(&format!(
      "`wp.done` is not a legal transition for {wp}, which is `not-started`"
    )),
    "`todo done {short_wp}` reaches the package `{wp}`:\n{out}{err}"
  );
}

#[test]
fn ac_descope_takes_a_bare_thread_number() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let from = created(&ok(root, &["st", "new", "The thread a requirement leaves"]));
  let to = created(&ok(root, &["st", "new", "The thread that will hold it"]));
  ok(
    root,
    &[
      "ac",
      "new",
      from.as_str(),
      "AC-01.1",
      "--text",
      "a requirement that moves",
    ],
  );

  let out = ok(
    root,
    &[
      "ac",
      "descope",
      from.as_str(),
      "AC-01.1",
      "--to",
      bare(&to).as_str(),
    ],
  );
  assert!(
    out.contains(&format!("descoped to {to}")),
    "`--to {}` names the thread `{to}`, as at every other door:\n{out}",
    bare(&to)
  );
}

#[test]
fn events_shows_every_record_an_untagged_number_names() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let st = created(&ok(root, &["st", "new", "A thread the log names"]));
  let issue = created(&ok(root, &["issues", "add", "An issue the log names"]));
  let thread_tag = ThreadId::parse(&st).expect("a minted thread id").tagged();
  let issue_tag = IssueId::parse(&issue).expect("a minted issue id").tagged();
  let number = bare(&st);
  assert_eq!(
    issue_tag.trim_start_matches(char::is_alphabetic),
    number,
    "the first thread and the first issue share a number"
  );
  let row = |id: &str| format!("  {id}  by ");

  let both = ok(root, &["events", "--subject", number.as_str()]);
  assert!(
    both.contains(&row(&st)) && both.contains(&row(&issue)),
    "`{number}` shows the events of {st} and of issue {issue}:\n{both}"
  );
  let thread_only = ok(root, &["events", "--subject", thread_tag.as_str()]);
  assert!(
    thread_only.contains(&row(&st)) && !thread_only.contains(&row(&issue)),
    "`{thread_tag}` narrows to the thread:\n{thread_only}"
  );
  let issue_only = ok(root, &["events", "--subject", issue_tag.as_str()]);
  assert!(
    issue_only.contains(&row(&issue)) && !issue_only.contains(&row(&st)),
    "`{issue_tag}` narrows to the issue:\n{issue_only}"
  );

  // A package under an issue's spelling begins as an id and is not one, so it
  // is refused rather than matched against nothing.
  let under_an_issue = format!("{issue_tag}/{number}");
  let (out, err, code) = run(root, &["events", "--subject", under_an_issue.as_str()]);
  assert_eq!(code, 1, "`{under_an_issue}`:\n{out}{err}");
  assert!(
    err.contains(&format!("`{thread_tag}` names the steel thread")),
    "the refusal offers the spelling that works:\n{err}"
  );
}

/// The WP-10 critic's red control: **A CRITERION'S EVENTS ARE FOUND BY THE
/// SUBJECT THE LOG WRITES**, `<thread>/<AC id>`, its thread in any spelling.
/// The subject door reads `<thread>/<tail>` through the waist's one reader, and
/// a tail that spells no package id is a subject under the thread, never a
/// refused package.
#[test]
fn events_finds_a_criterions_subject_as_the_log_writes_it() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let st = created(&ok(root, &["st", "new", "A thread with a criterion"]));
  let ac = "AC-01.1";
  ok(
    root,
    &[
      "ac",
      "new",
      st.as_str(),
      ac,
      "--text",
      "a requirement the log names",
    ],
  );
  let subject = format!("{st}/{ac}");
  for spelled in [subject.clone(), format!("{}/{ac}", bare(&st))] {
    let found = ok(root, &["events", "--subject", spelled.as_str()]);
    assert!(
      found.contains(&format!("  {subject}  by ")),
      "`--subject {spelled}` finds the criterion's events:\n{found}"
    );
  }
}

#[test]
fn set_takes_a_package_as_wp_new_prints_it() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let st = created(&ok(root, &["st", "new", "A thread with a package"]));
  let wp = created(&ok(
    root,
    &["wp", "new", st.as_str(), "A package set renames"],
  ));

  // It was refused as "neither an address nor an artefact id".
  ok(
    root,
    &["set", wp.as_str(), "title", "A package set has renamed"],
  );
  let shown = ok(root, &["wp", "show", wp.as_str()]);
  assert!(
    shown.contains("A package set has renamed"),
    "`set {wp}` reached the package:\n{shown}"
  );
}

/// One answer of `intent id --format json`, by its place.
fn answer(doc: &Value, at: usize) -> &Value {
  &doc["answers"][at]
}

#[test]
fn id_names_the_record_each_spelling_names() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "doors"]);
  let st = created(&ok(root, &["st", "new", "A thread the id door names"]));
  let wp = created(&ok(root, &["wp", "new", st.as_str(), "A package it names"]));
  let first = created(&ok(root, &["issues", "add", "An issue sharing the number"]));
  let second = created(&ok(
    root,
    &["issues", "add", "An issue alone at its number"],
  ));
  let thread_tag = ThreadId::parse(&st).expect("a minted thread id").tagged();
  let number = bare(&st);
  let second_number = IssueId::parse(&second)
    .expect("a minted issue id")
    .tagged()
    .trim_start_matches(char::is_alphabetic)
    .to_string();
  // The second issue's number under the thread tag names no thread here.
  let no_thread = format!(
    "{}{second_number}",
    thread_tag.trim_end_matches(|c: char| c.is_ascii_digit())
  );

  // Each spelling of one record, and a package's address: every one an id.
  let doc: Value = serde_json::from_str(&ok(
    root,
    &[
      "id",
      st.as_str(),
      thread_tag.as_str(),
      wp.as_str(),
      second_number.as_str(),
      "--format",
      "json",
    ],
  ))
  .expect("`intent id --format json` prints one JSON document");
  for (at, (kind, id)) in [
    ("thread", st.as_str()),
    ("thread", st.as_str()),
    ("wp", wp.as_str()),
    ("issue", second.as_str()),
  ]
  .into_iter()
  .enumerate()
  {
    let a = answer(&doc, at);
    assert_eq!(a["answer"], "id", "answer {at} names one record:\n{doc:#}");
    assert_eq!(a["kind"], kind, "answer {at}'s kind:\n{doc:#}");
    assert_eq!(a["id"], id, "answer {at}'s stored id:\n{doc:#}");
    let canon = a["canon"].as_str().expect("a canon path");
    assert!(
      root.join(canon).is_file(),
      "answer {at}'s canon file `{canon}` is the record's own:\n{doc:#}"
    );
  }
  let thread = answer(&doc, 0);
  let package = answer(&doc, 2);
  assert_eq!(
    package["canon"], thread["canon"],
    "a package's canon is its thread's, which carries it:\n{doc:#}"
  );
  let thread_dir = thread["path"].as_str().expect("a thread path");
  assert!(
    package["path"]
      .as_str()
      .is_some_and(|p| p.starts_with(&format!("{thread_dir}/"))),
    "a package is realised under its thread:\n{doc:#}"
  );

  // An untagged number naming a thread and an issue, a well-formed id naming
  // nothing here, a word that is no id, and another project's address of a
  // thread this project also has: every token answered, exit 1.
  let elsewhere = format!("intent://elsewhere/threads/{st}");
  let (out, err, code) = run(
    root,
    &[
      "id",
      number.as_str(),
      no_thread.as_str(),
      "banana",
      elsewhere.as_str(),
    ],
  );
  assert_eq!(code, 1, "not every token is an id here:\n{out}{err}");
  let lines: Vec<&str> = out.lines().collect();
  assert_eq!(
    lines[..3],
    [
      format!("ambiguous token={number} kind=thread,issue candidates={st},{first}").as_str(),
      format!("unknown token={no_thread} kind=thread").as_str(),
      "not-an-id token=banana -- `banana` is neither an address nor an artefact id -- remedy: name a thread (`ST0000`) or a full address (`intent:///threads/ST0000`)",
    ],
    "one line per token, its first word the answer:\n{out}{err}"
  );
  // Another project's address names nothing here, whatever this project holds
  // under the same id (issue 0338).
  assert!(
    lines.len() == 4
      && lines[3].starts_with(&format!("not-an-id token={elsewhere} -- "))
      && lines[3].contains("names the project `elsewhere`"),
    "another project's address is no id of a record here:\n{out}{err}"
  );
}
