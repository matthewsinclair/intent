//! Issue 0670: the AT citation check, as hv ruled it on 2026-11-01 and vc shaped
//! it on 2026-11-10 (the issue's record).
//!
//! A bare id line stops counting for a row where another thread has a green or
//! red row of the same id citing the same file; a row may cite its arm as
//! `<file>:<line>`, and then that one line is read; and the id is matched as a
//! whole token. The exhibit is Devbin ST0011's AT-3.5, which passed on ST0007's
//! bare `# AT-3.5` line over another arm. Each arm here builds that shape in a
//! fixture and asks `literal_id_finding`, the one place the refusals are
//! worded, through `ProjectRefs`, which every production reader builds.
use crate::common::{Fixture, sample_thread};
use intentsvcs::contract::{
  Carries, Followed, ProjectRefs, References, citation, followed_rows, literal_id_finding,
};
use intentsvcs::id::ThreadId;
use intentsvcs::model::{AcceptanceTest, AtKind, AtStatus, Thread, ThreadStatus};

const FILE: &str = "tests/unit/releasestatus.bats";

fn row(id: &str, file: &str, status: AtStatus) -> AcceptanceTest {
  AcceptanceTest {
    fiat: None,
    id: id.to_string(),
    kind: AtKind::Test,
    file: Some(file.to_string()),
    prose: None,
    covers: vec!["AC-3.3".to_string()],
    status,
    note: None,
    legacy: None,
  }
}

fn thread_with(id: &str, status: ThreadStatus, tests: Vec<AcceptanceTest>) -> Thread {
  let mut t = sample_thread(id);
  t.status = status;
  t.tests = tests;
  t
}

/// The exhibit: ST0007 (completed) owns the file's one `# AT-3.5` line, over
/// its own arm, and ST0011 cites the same file for its own AT-3.5, whose arm
/// at line 4 carries no id.
fn exhibit(fx: &Fixture, ours: AtStatus, rival: AtStatus) -> Vec<Thread> {
  fx.write_file(
    FILE,
    "#!/usr/bin/env bats\n\n# the arm the other row means, with no id over it\n@test \"a tag whose object the repository lacks stops release status\" {\n  true\n}\n\n# AT-3.5\n@test \"every row is rendered by release_row\" {\n  true\n}\n",
  );
  vec![
    thread_with(
      "ST0007",
      ThreadStatus::Completed,
      vec![row("AT-3.5", FILE, rival)],
    ),
    thread_with("ST0011", ThreadStatus::Wip, vec![row("AT-3.5", FILE, ours)]),
  ]
}

#[test]
fn a_bare_id_line_stops_counting_where_another_thread_has_an_armed_row_of_the_same_id_citing_the_file()
 {
  let fx = Fixture::new();
  let threads = exhibit(&fx, AtStatus::Green, AtStatus::Green);
  let refs = ProjectRefs::new(fx.root(), &threads);
  assert_eq!(
    refs.carries_id(FILE, "AT-3.5", "ST0011"),
    Carries::Ambiguous(vec!["ST0007".to_string()])
  );
  let finding = literal_id_finding(FILE, "AT-3.5", "ST0011", &refs).expect("the exhibit reddens");
  assert!(
    finding.contains("ST0007 also has a green or red AT-3.5 citing it"),
    "the refusal names the rival: {finding}"
  );
  assert!(
    finding.contains("name ST0011 on the line") && finding.contains(&format!("`{FILE}:<line>`")),
    "and both remedies: {finding}"
  );
}

#[test]
fn the_rival_thread_itself_still_passes_on_its_own_bare_line_only_while_it_has_no_rival() {
  let fx = Fixture::new();
  // A to-write row has no arm yet, so it is no rival: ST0007's bare line
  // counts for ST0007 when ST0011's row is to-write.
  let threads = exhibit(&fx, AtStatus::ToWrite, AtStatus::Green);
  let refs = ProjectRefs::new(fx.root(), &threads);
  assert_eq!(refs.carries_id(FILE, "AT-3.5", "ST0007"), Carries::Yes);
  // And once ST0011's row is armed, ST0007's bare line is ambiguous for it too.
  let threads = exhibit(&fx, AtStatus::Red, AtStatus::Green);
  let refs = ProjectRefs::new(fx.root(), &threads);
  assert_eq!(
    refs.carries_id(FILE, "AT-3.5", "ST0007"),
    Carries::Ambiguous(vec!["ST0011".to_string()])
  );
}

#[test]
fn a_line_naming_the_rows_own_thread_counts_whatever_its_rivals() {
  let fx = Fixture::new();
  let threads = exhibit(&fx, AtStatus::Green, AtStatus::Green);
  fx.write_file(
    FILE,
    "# ST0011 AT-3.5\n@test \"x\" {\n  true\n}\n# AT-3.5\n",
  );
  let refs = ProjectRefs::new(fx.root(), &threads);
  assert_eq!(refs.carries_id(FILE, "AT-3.5", "ST0011"), Carries::Yes);
}

/// Issue I88f333f: a cited line names a git-style thread by its shown front,
/// and the one scanner reads it as that thread. A front that opens with four
/// digits is the hashed thread's, never the ordinal those digits spell.
#[test]
fn a_line_naming_a_git_style_thread_by_its_shown_front_names_that_thread_and_no_ordinal() {
  let fx = Fixture::new();
  let hashed = ThreadId::parse("ST1234abcdef012345").expect("a stored hashed thread id");
  let ordinal = ThreadId::parse("ST1234").expect("a stored ordinal thread id");
  fx.write_file(FILE, &format!("# {} AT-3.5\n", hashed.shown()));
  let threads = vec![thread_with(
    hashed.as_str(),
    ThreadStatus::Wip,
    vec![row("AT-3.5", FILE, AtStatus::Green)],
  )];
  let refs = ProjectRefs::new(fx.root(), &threads);
  assert_eq!(
    refs.carries_id(FILE, "AT-3.5", hashed.as_str()),
    Carries::Yes
  );
  assert_eq!(
    refs.carries_id(FILE, "AT-3.5", ordinal.as_str()),
    Carries::OnlyUnder(vec![hashed.shown()])
  );
}

#[test]
fn a_row_citing_its_line_is_read_at_that_line_and_a_bare_id_there_counts_despite_rivals() {
  let fx = Fixture::new();
  let mut threads = exhibit(&fx, AtStatus::Green, AtStatus::Green);
  // ST0011 cites line 8, the bare `# AT-3.5`: it has said that line is its own.
  let cited = format!("{FILE}:8");
  threads[1].tests = vec![row("AT-3.5", &cited, AtStatus::Green)];
  let refs = ProjectRefs::new(fx.root(), &threads);
  assert!(refs.resolves(&cited), "the citation resolves to the file");
  assert_eq!(refs.carries_id(&cited, "AT-3.5", "ST0011"), Carries::Yes);
  assert_eq!(literal_id_finding(&cited, "AT-3.5", "ST0011", &refs), None);
}

/// A3(b): the gate follows a row only to a line that is its thread's. In the
/// exhibit the one line carrying the id is bare and ST0007 arms the same id in
/// the file, so following it would choose ST0007's arm for ST0011: it refuses.
#[test]
fn a_cited_line_whose_one_carrying_line_is_bare_beside_a_rival_is_refused_and_not_followed() {
  let fx = Fixture::new();
  let mut threads = exhibit(&fx, AtStatus::Green, AtStatus::Green);
  let cited = format!("{FILE}:4");
  threads[1].tests = vec![row("AT-3.5", &cited, AtStatus::Green)];
  let refs = ProjectRefs::new(fx.root(), &threads);
  assert_eq!(
    refs.carries_id(&cited, "AT-3.5", "ST0011"),
    Carries::Ambiguous(vec!["ST0007".to_string()])
  );
  let finding =
    literal_id_finding(&cited, "AT-3.5", "ST0011", &refs).expect("line 4 carries no id");
  assert!(
    finding.contains("name ST0011 on the line") && finding.contains(&format!("`{FILE}:<line>`")),
    "the refusal says how to make the line this thread's: {finding}"
  );
  assert!(followed_rows(&threads[1], &refs).is_empty());
}

#[test]
fn the_id_is_matched_as_a_whole_token() {
  let fx = Fixture::new();
  fx.write_file(FILE, "# AT-3.50\n# AT-3.5x is not a digit after\n");
  let threads = vec![thread_with(
    "ST0011",
    ThreadStatus::Wip,
    vec![row("AT-3.5", FILE, AtStatus::Green)],
  )];
  let refs = ProjectRefs::new(fx.root(), &threads);
  // Line 2 carries AT-3.5 followed by a letter, which counts; line 1 alone
  // would not.
  assert_eq!(refs.carries_id(FILE, "AT-3.5", "ST0011"), Carries::Yes);
  fx.write_file(FILE, "# AT-3.50\n");
  assert_eq!(refs.carries_id(FILE, "AT-3.5", "ST0011"), Carries::Absent);
}

#[test]
fn the_one_parser_reads_a_line_only_from_a_positive_decimal_suffix() {
  assert_eq!(citation("tests/a.bats:161").path, "tests/a.bats");
  assert_eq!(citation("tests/a.bats:161").line, Some(161));
  assert_eq!(citation("tests/a.bats").line, None);
  assert_eq!(citation("tests/a.bats:0").line, None);
  assert_eq!(citation("tests/a.bats:0161").line, None);
  assert_eq!(citation("tests/a.bats:x").path, "tests/a.bats:x");
  assert_eq!(citation("tests/a:b.bats").path, "tests/a:b.bats");
}

/// Issue Ifdd3c89 and A3(b): a line naming another thread's AT of the same id
/// is that thread's arm, never this row's, so it is neither offered nor
/// followed. The one line that carries the id for this thread is followed: the
/// row is counted there, and the gate gives the command that records it. Two
/// such lines are refused, and the gate does not choose.
#[test]
fn a_cited_line_is_followed_to_its_threads_one_line_and_never_to_another_threads() {
  let fx = Fixture::new();
  let lines = [
    "# the arm with no id over it",
    "@test \"x\" {",
    "# ST0007 AT-3.5",
    "@test \"the other thread's arm\" {",
    "# ST0011 AT-3.5",
    "@test \"this thread's arm\" {",
  ];
  fx.write_file(FILE, &(lines.join("\n") + "\n"));
  let at = |text: &str| {
    lines
      .iter()
      .position(|l| *l == text)
      .expect("a fixture line")
      + 1
  };
  let cited = format!("{FILE}:{}", at(lines[0]));
  let threads = vec![
    thread_with(
      "ST0007",
      ThreadStatus::Completed,
      vec![row("AT-3.5", FILE, AtStatus::Green)],
    ),
    thread_with(
      "ST0011",
      ThreadStatus::Wip,
      vec![row("AT-3.5", &cited, AtStatus::Green)],
    ),
  ];
  let refs = ProjectRefs::new(fx.root(), &threads);
  let own = at("# ST0011 AT-3.5");
  assert_eq!(
    refs.carries_id(&cited, "AT-3.5", "ST0011"),
    Carries::Moved {
      line: at(lines[0]),
      to: own
    },
    "the line naming ST0007 is not followed"
  );
  assert_eq!(literal_id_finding(&cited, "AT-3.5", "ST0011", &refs), None);
  let followed = followed_rows(&threads[1], &refs);
  assert_eq!(
    followed.iter().map(Followed::command).collect::<Vec<_>>(),
    vec![format!("intent at edit ST0011 AT-3.5 --file {FILE}:{own}")]
  );

  // The file again with this thread's arm a second time below it.
  let twice = [&lines[..], &lines[4..]].concat();
  fx.write_file(FILE, &(twice.join("\n") + "\n"));
  let carrying: Vec<usize> = twice
    .iter()
    .enumerate()
    .filter(|(_, l)| **l == lines[4])
    .map(|(i, _)| i + 1)
    .collect();
  assert_eq!(
    refs.carries_id(&cited, "AT-3.5", "ST0011"),
    Carries::NotOnLine {
      line: at(lines[0]),
      carrying
    }
  );
  let finding = literal_id_finding(&cited, "AT-3.5", "ST0011", &refs).expect("two lines carry it");
  assert!(
    finding.contains("the gate does not choose between them"),
    "the refusal says the gate does not choose: {finding}"
  );
  assert!(followed_rows(&threads[1], &refs).is_empty());
}
