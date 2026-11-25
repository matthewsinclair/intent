//! **SHIPPED PROSE THAT RESTATES AN ID CONSTANT IS HELD TO IT** (ST1c2db93
//! P28a). The root-file template every project's `AGENTS.md` renders from and
//! the skills an agent loads each tell a reader how an id is spelled: how many
//! hex a git-style id shows, how few a typed prefix may carry, what an
//! ordinal looks like. Those numbers live in `intentsvcs::id`, and the prose
//! is written by hand, so a change to a constant would leave the prose
//! teaching the old shape with nothing failing. This file builds each phrase
//! from the constants and asserts the prose carries it, so either side moving
//! alone fails here.
//!
//! The same holds for every other shipped copy of an id form (ST1c2db93
//! WP-ce27): the `intent/wip.md` template, the canon-ignore hook's probes of a
//! future canon file, and the tech note's sentence on a claim's address, which
//! does not ship but teaches the same forms. Each pin reads its phrase, never
//! a line number, so an edit elsewhere in its file moves nothing here.

use intentsvcs::event::EVENTS_DIR;
use intentsvcs::id::{
  CLAIM_ADDRESS_FORMS, EXAMPLE_HEX, EXAMPLE_WP_HEX, ISSUE_DIGITS, ISSUE_MARKER, IssueId, LabelKind,
  MIN_PREFIX_HEX, SHOWN_HEX, THREAD_DIGITS, THREAD_PREFIX, WP_DIGITS, WP_SHOWN_HEX, prefix_floor,
};
use intentsvcs::project::{
  CANON_DIR, canon_ac_rel, canon_at_rel, canon_attachment_rel, canon_blob_rel, canon_issue_rel,
  canon_project_rel, canon_thread_rel, canon_wp_rel,
};
use testkit::repo_root;

/// The root-file template `AGENTS.md` renders from.
const AGENTS_TEMPLATE: &str = "lib/templates/llm/_AGENTS.md";
/// The essentials skill every session loads.
const IN_ESSENTIALS: &str = "intent/plugins/claude/skills/in-essentials/SKILL.md";
/// The whiteboard skill, which teaches a claim's address.
const IN_WHITEBOARD: &str = "intent/plugins/claude/skills/in-whiteboard/SKILL.md";
/// The template a project's `intent/wip.md` starts from.
const WIP_TEMPLATE: &str = "lib/templates/prj/_wip.md";
/// The shipped commit hook that refuses an ignore rule reaching the canon.
const CANON_IGNORE_GUARD: &str = "lib/templates/hooks/canon-ignore-guard.sh";
/// The tech note on working with LLMs, which teaches a claim's address too.
const LLM_NOTE: &str = "intent/docs/working-with-llms.md";
/// The published schema faces whose descriptions quote an id (ST1c2db93
/// close sweep, A 1.7): each is generated from intentsvcs' types, and
/// `schema_faces_drift` holds the face to its type.
const THREAD_SCHEMA: &str = "schema/thread.schema.json";
const ISSUE_SCHEMA: &str = "schema/issue.schema.json";
const EVENT_SCHEMA: &str = "schema/event.schema.json";
const DDL_FACE: &str = "schema/ddl.sql";

/// A count as the prose spells it. Panics on a count the prose has no word
/// for, which is a prompt to extend this table, not to change the prose.
fn word(n: usize) -> &'static str {
  const WORDS: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
  ];
  WORDS
    .get(n)
    .copied()
    .unwrap_or_else(|| panic!("no word for {n}: extend this table"))
}

fn read(rel: &str) -> String {
  let path = repo_root().join(rel);
  std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn assert_states(rel: &str, text: &str, phrase: &str) {
  assert!(
    text.contains(phrase),
    "{rel} no longer states `{phrase}`, which the constants in `intentsvcs` spell: \
     the prose and the constants have parted"
  );
}

/// The prefix rule: the fewest hex a typed prefix carries, and the more it
/// needs when it is all digits, read from `prefix_floor` itself. `of` follows
/// "hex" where the prose says whose hex they are.
fn prefix_rule(of: &str) -> String {
  let all_digits = prefix_floor(&"0".repeat(MIN_PREFIX_HEX), THREAD_DIGITS);
  format!(
    "any unique prefix of at least {} hex{of}, or {} if they are all digits",
    word(MIN_PREFIX_HEX),
    word(all_digits)
  )
}

/// The prefix rule as the template and the essentials skill state it.
fn prefix_phrase() -> String {
  prefix_rule(" of a git-style id")
}

/// A thread's, a package's and an issue's id as a face shows them, and as an
/// ordinal one is spelled, each built from the constants.
fn shown_thread() -> String {
  format!("{THREAD_PREFIX}{}", &EXAMPLE_HEX[..SHOWN_HEX])
}
fn shown_package() -> String {
  format!("{}/{}", shown_thread(), &EXAMPLE_WP_HEX[..WP_SHOWN_HEX])
}
fn shown_issue() -> String {
  format!("{ISSUE_MARKER}{}", &EXAMPLE_HEX[..SHOWN_HEX])
}
fn ordinal_thread() -> String {
  format!("{THREAD_PREFIX}{}", "0".repeat(THREAD_DIGITS))
}
fn ordinal_package() -> String {
  format!("{}/{}", ordinal_thread(), "0".repeat(WP_DIGITS - 1) + "1")
}
fn ordinal_issue() -> String {
  "0".repeat(ISSUE_DIGITS)
}

#[test]
fn the_agents_template_states_the_id_constants() {
  let text = read(AGENTS_TEMPLATE);
  for phrase in [
    prefix_phrase(),
    format!("`{}` for a thread", shown_thread()),
    format!("`{}` for one of its work packages", shown_package()),
    format!("`{}` for an issue", shown_issue()),
    format!(
      "`{}`, `{}`, issue `{}`",
      ordinal_thread(),
      ordinal_package(),
      ordinal_issue()
    ),
  ] {
    assert_states(AGENTS_TEMPLATE, &text, &phrase);
  }
}

#[test]
fn the_essentials_skill_states_the_id_constants() {
  let text = read(IN_ESSENTIALS);
  for phrase in [
    prefix_phrase(),
    format!(
      "a thread shows as `{THREAD_PREFIX}` and {} hex, eg `{}`",
      word(SHOWN_HEX),
      shown_thread()
    ),
    format!(
      "an issue as `{ISSUE_MARKER}` and {} hex, eg `{}`",
      word(SHOWN_HEX),
      shown_issue()
    ),
    format!(
      "a slash and {} hex, eg `{}`",
      word(WP_SHOWN_HEX),
      shown_package()
    ),
    format!(
      "`{}`, issue `{}`, work package `{}`",
      ordinal_thread(),
      ordinal_issue(),
      ordinal_package()
    ),
  ] {
    assert_states(IN_ESSENTIALS, &text, &phrase);
  }
}

/// The whiteboard skill teaches a claim's address in both families, each id
/// whole as a board stores it, by quoting `id::CLAIM_ADDRESS_FORMS`, the one
/// sentence every claim refusal and MCP's label descriptions quote too.
#[test]
fn the_whiteboard_skill_states_the_id_constants() {
  assert_states(IN_WHITEBOARD, &read(IN_WHITEBOARD), CLAIM_ADDRESS_FORMS);
}

/// The wip template tells a reader to lead a focus item with its thread's id,
/// with one example of each family.
#[test]
fn the_wip_template_states_the_id_constants() {
  let phrase = format!("eg `{}` or `{}`", shown_thread(), ordinal_thread());
  assert_states(WIP_TEMPLATE, &read(WIP_TEMPLATE), &phrase);
}

/// The event file the canon-ignore hook names under `events/`: a day and the
/// ULID specification's example id. Any dated name serves, because the probe is
/// about the directory the layout gives (`intentsvcs::event::EVENTS_DIR`).
const EVENT_FILE: &str = "2026/01/01/01ARZ3NDEKTSV4RRFFQ69G5FAV.json";
/// The attachment the canon-ignore hook names as opaque bytes under its future
/// thread's directory in `st/`, for the same reason as [`EVENT_FILE`].
const OPAQUE_ATTACHMENT: &str = "diagram.png";
/// The attachment whose record the hook names under `records/`, for the same
/// reason as [`OPAQUE_ATTACHMENT`].
const RECORDED_ATTACHMENT: &str = "design.md";

/// A canon path as the canon-ignore hook spells a probe: its own name for the
/// canon directory, then the path the project's layout gives after
/// `project::CANON_DIR`, in the double quotes the probe's line holds. The
/// quotes keep a comment that mentions the path from standing in for the probe.
fn probe_line(rel: &str) -> String {
  let after = rel
    .strip_prefix(CANON_DIR)
    .unwrap_or_else(|| panic!("{rel} is not under {CANON_DIR}"));
  format!("\"${{CANON_DIR}}{after}\"")
}

/// The canon-ignore hook probes, by name, a future canon file in each place the
/// project's layout keeps a record: an event's file under `events/`, the
/// thread's file and an opaque attachment's bytes under `st/`, a package, a
/// criterion, a test, and an attachment under `records/`, and the issue's file.
/// Each is named with the worked example's hex and labels, which name no
/// record. It probes the project's state file by its name as well. Every path
/// is the one `intentsvcs::project` and `intentsvcs::event` give.
#[test]
fn the_canon_ignore_guard_probes_the_worked_example() {
  let text = read(CANON_IGNORE_GUARD);
  let thread = format!("{THREAD_PREFIX}{EXAMPLE_HEX}");
  // The worked package's first label of each kind. This restates the second
  // form `LabelKind::examples` gives, because no function gives one label:
  // change the two together.
  let label = |kind: LabelKind| format!("{}-{EXAMPLE_WP_HEX}.1", kind.head());
  let issue = IssueId::parse(&format!("{ISSUE_MARKER}{EXAMPLE_HEX}"))
    .expect("the worked example's hex makes an issue's stored id");
  for rel in [
    format!("{CANON_DIR}/{EVENTS_DIR}/{EVENT_FILE}"),
    canon_thread_rel(&thread),
    canon_blob_rel(&thread, OPAQUE_ATTACHMENT),
    canon_wp_rel(&thread, EXAMPLE_WP_HEX),
    canon_ac_rel(&thread, &label(LabelKind::Criterion)),
    canon_at_rel(&thread, &label(LabelKind::Test)),
    canon_attachment_rel(&thread, RECORDED_ATTACHMENT),
    canon_issue_rel(issue),
    canon_project_rel(),
  ] {
    assert_states(CANON_IGNORE_GUARD, &text, &probe_line(&rel));
  }
}

/// The register. `gen_reference.sh` publishes its argument notes into
/// `docs/reference`, and `--help` does not print them.
const REGISTER: &str = "surface/dispatch-table.json";

/// The note the register gives one argument of one command, read from the
/// table the binary carries.
fn register_note(path: &str, arg: &str) -> String {
  let table: serde_json::Value = serde_json::from_str(intent_cli::dispatch::TABLE)
    .unwrap_or_else(|e| panic!("{REGISTER} does not parse: {e}"));
  table["families"]
    .as_array()
    .into_iter()
    .flatten()
    .flat_map(|family| family["entries"].as_array().into_iter().flatten())
    .chain(table["new_surface"].as_array().into_iter().flatten())
    .filter(|entry| entry["path"] == path)
    .flat_map(|entry| entry["args"].as_array().into_iter().flatten())
    .find(|a| a["name"] == arg)
    .and_then(|a| a["note"].as_str())
    .unwrap_or_else(|| panic!("{REGISTER} has no note on `{path}`'s `{arg}`"))
    .to_string()
}

/// The register's notes on an id argument quote each family's form sentence
/// whole, so the reference teaches both families as every refusal does. The
/// `id` command's note gives a git-style example of each kind, and an ordinal
/// thread and package.
#[test]
fn the_register_notes_state_the_id_forms() {
  use intentsvcs::id::{ISSUE_ID_FORM, THREAD_ID_FORM, WP_ADDRESS_FORM};
  for (path, forms) in [
    ("edit", &[THREAD_ID_FORM, WP_ADDRESS_FORM][..]),
    (
      "browse",
      &[THREAD_ID_FORM, WP_ADDRESS_FORM, ISSUE_ID_FORM][..],
    ),
  ] {
    let note = register_note(path, "id");
    for form in forms {
      assert_states(REGISTER, &note, form);
    }
  }
  let token = register_note("id", "token");
  for example in [
    shown_thread(),
    shown_issue(),
    shown_package(),
    ordinal_thread(),
    ordinal_package(),
  ] {
    assert_states(REGISTER, &token, &format!("`{example}`"));
  }
}

/// The user docs a reader learns ids from.
const README: &str = "README.md";
const GETTING_STARTED: &str = "docs/getting-started.md";
const TEAM_DOC: &str = "docs/concepts/working-in-a-team.md";
const CRITERIA_DOC: &str = "docs/concepts/criteria-and-tests.md";

/// The user docs and the tech note each restate some of the prefix rule, the
/// shown widths, a label's group and a claim's issue form, in their own words.
/// Each phrase is built from the constants.
#[test]
fn the_user_docs_state_the_id_constants() {
  use intentsvcs::id::{CLAIM_ISSUE_PREFIX, THREAD_GROUP};
  let states = |rel: &str, phrases: &[String]| {
    let text = read(rel);
    for phrase in phrases {
      assert_states(rel, &text, phrase);
    }
  };
  let prefix = prefix_rule("");
  let ordinal_group = format!("and {} digits for one made before", word(WP_DIGITS));

  states(README, std::slice::from_ref(&prefix));
  states(
    GETTING_STARTED,
    &[
      prefix.clone(),
      ordinal_group.clone(),
      format!("The {} hex the tables print", word(WP_SHOWN_HEX)),
      format!("`AC-{THREAD_GROUP}.*` belong to the thread"),
    ],
  );
  states(
    TEAM_DOC,
    &[
      prefix,
      format!(
        "{} hex for a thread or an issue, {} for a work package",
        word(SHOWN_HEX),
        word(WP_SHOWN_HEX)
      ),
      format!("A thread's or an issue's {} hex", word(SHOWN_HEX)),
      format!("or an issue as `{CLAIM_ISSUE_PREFIX}` and its id"),
    ],
  );
  states(
    CRITERIA_DOC,
    &[
      ordinal_group,
      format!("the package's {} shown hex", word(WP_SHOWN_HEX)),
      format!("`AC-{THREAD_GROUP}.*` are thread-level"),
    ],
  );
  states(
    LLM_NOTE,
    &[
      format!("`{THREAD_GROUP}` for the thread as a whole"),
      format!("{} digits for a package made before", word(WP_DIGITS)),
      format!(
        "the package's {} shown hex, `AC-{}.1`",
        word(WP_SHOWN_HEX),
        &EXAMPLE_WP_HEX[..WP_SHOWN_HEX]
      ),
      format!("`AC-{EXAMPLE_WP_HEX}.1`"),
    ],
  );
}

/// The tech note teaches a claim's address in both families by quoting
/// `id::CLAIM_ADDRESS_FORMS`, as the whiteboard skill does.
#[test]
fn the_llm_note_states_the_claim_forms() {
  assert_states(LLM_NOTE, &read(LLM_NOTE), CLAIM_ADDRESS_FORMS);
}

/// The schema faces quote the worked example whole, as a record stores it,
/// and an ordinal of each family at its width (ST1c2db93 close sweep, A 1.7).
/// The ordinals' numbers are the descriptions' own, so the pin binds their
/// width and family, which the constants fix, and not which number they are.
#[test]
fn the_schema_faces_state_the_id_constants() {
  let thread = format!("{THREAD_PREFIX}{EXAMPLE_HEX}");
  let package = format!("{thread}/{EXAMPLE_WP_HEX}");
  let issue = format!("{ISSUE_MARKER}{EXAMPLE_HEX}");
  let ordinal = |n: usize, width: usize| format!("{n:0width$}");
  let faces = [
    (
      THREAD_SCHEMA,
      vec![
        format!("`{thread}`"),
        format!(
          "{} digits for an ordinal, eg `{}`",
          word(WP_DIGITS),
          ordinal(1, WP_DIGITS)
        ),
      ],
    ),
    (
      ISSUE_SCHEMA,
      vec![format!(
        "{} digits for an ordinal, eg `{}`",
        word(ISSUE_DIGITS),
        ordinal(21, ISSUE_DIGITS)
      )],
    ),
    (
      EVENT_SCHEMA,
      vec![
        format!(
          "`{}/{}` or `{}`",
          ordinal_thread(),
          ordinal(2, WP_DIGITS),
          ordinal(21, ISSUE_DIGITS)
        ),
        format!("`{thread}`, `{package}` or `{issue}`"),
      ],
    ),
    (
      DDL_FACE,
      vec![
        format!("`{}` or `{EXAMPLE_WP_HEX}`", ordinal(4, WP_DIGITS)),
        format!("`{}` or `{issue}`", ordinal(703, ISSUE_DIGITS)),
      ],
    ),
  ];
  for (rel, phrases) in faces {
    let text = read(rel);
    for phrase in phrases {
      assert_states(rel, &text, &phrase);
    }
  }
}
