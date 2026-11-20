//! Issue 0738: a doctor finding that carries its own remedy gives that remedy
//! alone, never beside its class's. The event-file findings are driven through
//! the binary in `the_event_log_travels.rs`; this is the config-side finding
//! whose remedy is the instance's arithmetic.

use crate::common::{Fixture, ctx};
use intentsvcs::finding::{CANNOT_HONOUR, FindingClass};
use intentsvcs::model::COMPLETED_RESOLUTION_HOURS;

/// A window that is not a whole number of days is refused with one remedy:
/// the class's sentence that stays true of it, then the two honourable values
/// either side of the one configured.
#[test]
fn an_unhonourable_todo_window_gives_one_remedy_naming_the_values_either_side() {
  let fx = Fixture::new();
  let configured = COMPLETED_RESOLUTION_HOURS + COMPLETED_RESOLUTION_HOURS / 2;
  fx.write_file(
    "intent/.config/config.json",
    &format!(
      "{{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Fixture\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"],\n  \"todo\": {{ \"window_hours\": {configured} }}\n}}\n"
    ),
  );
  let report =
    intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All);
  let found: Vec<_> = report
    .findings
    .iter()
    .filter(|f| f.class == FindingClass::UnhonourableSetting)
    .collect();
  let [finding] = found.as_slice() else {
    panic!("one unhonourable setting: {found:?}");
  };
  let down = configured - configured % COMPLETED_RESOLUTION_HOURS;
  let up = down + COMPLETED_RESOLUTION_HOURS;
  assert!(
    finding.remedy().starts_with(CANNOT_HONOUR)
      && finding.remedy().contains(&format!("{down} or {up}")),
    "the remedy is the class's true clause and this window's values: {finding}"
  );
  let shown = finding.to_string();
  assert_eq!(shown.matches("remedy: ").count(), 1, "{shown}");
  assert!(
    !shown.contains(finding.class.remedy()),
    "the class remedy is not printed beside the finding's own: {shown}"
  );
}
