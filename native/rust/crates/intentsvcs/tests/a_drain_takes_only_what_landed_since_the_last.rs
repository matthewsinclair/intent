//! Issue 0624: one facade, two writes, two drains. `take_landed` takes the
//! facade's whole list of what its writes landed, so the second verb's drain
//! names the second verb's files and nothing the first landed, and a drain
//! with nothing landed since it is empty. That is what keeps a facade that
//! lives past one verb, the daemon's or the explorer's, from carrying one
//! verb's files into the next one's answer.

use crate::common::Fixture;
use intentsvcs::facade::Written;

#[test]
fn a_drain_takes_what_landed_since_the_last_and_nothing_older() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();

  let id = f.st_new("a probe thread").expect("st new");
  let first = f.take_landed();
  assert!(
    first
      .wrote
      .iter()
      .any(|p| p.ends_with(&format!("{id}.json"))),
    "the create names its canon record: {first:?}"
  );
  let first_events: Vec<&String> = first
    .wrote
    .iter()
    .filter(|p| p.contains(".canon/events/"))
    .collect();
  assert!(
    !first_events.is_empty(),
    "the create names its own event file: {first:?}"
  );

  f.st_start(&id).expect("st start");
  let second = f.take_landed();
  assert!(
    !second.wrote.is_empty(),
    "the start landed files: {second:?}"
  );
  for event in first_events {
    assert!(
      !second.wrote.contains(event),
      "the second drain carried the first verb's event {event}: {second:?}"
    );
  }

  assert_eq!(
    f.take_landed(),
    Written::default(),
    "nothing landed since the last drain"
  );
}
