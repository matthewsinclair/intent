//! `AT-08.6` / `AC-08.6`: **`projectChanged` and `fileChanged` deliver over the
//! socket, and nothing more ships in 3.0.0** (`D20`).
//!
//! **THE CRITERION HAS A CEILING IN IT, WHICH IS UNUSUAL AND IS THE HALF MOST
//! LIKELY TO ERODE.** D20: *3.0.0 subscriptions are exactly two ...  Nothing
//! more ships until a consumer (TUI/bus) exists to need it.* A third event is a
//! decision somebody must make, so this file makes adding one a COMPILE ERROR
//! rather than a review question -- an exhaustive match over `Event` that has
//! to be edited before a new variant builds.
//!
//! **THE TWO EVENTS ARE NOT REDUNDANT AND THE TEST TURNS ON WHICH LAYER MOVED.**
//! `fileChanged` says the DISK moved and comes from the watcher, which is the
//! only thing that knows the paths. `projectChanged` says the MODEL moved and
//! comes from the store thread after the re-read completes, which is the only
//! thing that knows it finished. **Collapsing them would deliver `the model
//! changed` before it had**, and a subscriber redrawing on that reads the state
//! it already held -- correct-looking, one edit stale, forever.
//!
//! **AND THE TWO-SUBSCRIBER ARM IS THE ONE THAT CANNOT BE SKIPPED.** A fan-out
//! built on an mpsc queue gives each event to exactly ONE listener. With a
//! single subscriber that is indistinguishable from a broadcast; with two it is
//! catastrophic and silent. The defect is invisible at the size everybody tests
//! at.

mod common;

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use common::RunningDaemon;
use intentsvcs::wire::{self, Event, Op, Request, Response};

/// How long a read waits before the test gives up rather than hanging.
///
/// **BOUNDED BECAUSE THE FAILURE MODE OF A SUBSCRIPTION IS *NOTHING ARRIVES*.**
/// An unbounded read turns a missing event into a hung suite, whose only
/// symptom is that the suite is slow -- a class this estate has met twice in
/// two days, once from an exec that never returns and once from an unbounded
/// loop in a unit test. **The bound is on the READ and not on the assertion**:
/// what is asserted is that the event arrived, not that it arrived quickly.
const READ_TIMEOUT: Duration = Duration::from_secs(20);

/// A subscription: the connection, held open, and its acknowledged project id.
///
/// **ONE CONNECTION FOR THE WHOLE FEED, WHICH IS WHAT MAKES THIS TESTABLE AT
/// ALL.** `wire::ask` is one request per connection, which is what every other
/// caller does and cannot express a stream. This is the one shape in the estate
/// that needs a connection with a history.
struct Feed {
  reader: BufReader<UnixStream>,
  project_id: String,
}

impl Feed {
  fn open(daemon: &RunningDaemon, root: &Path) -> Feed {
    let stream = UnixStream::connect(daemon.socket()).expect("connect to the daemon");
    stream
      .set_read_timeout(Some(READ_TIMEOUT))
      .expect("bounded read");
    let mut writer = &stream;
    let framed = wire::frame(&Request {
      root: root.to_path_buf(),
      op: Op::Subscribe,
    })
    .expect("serialisable");
    writer.write_all(&framed).expect("send the subscribe");
    writer.flush().expect("flush");

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader
      .read_line(&mut line)
      .expect("read the acknowledgement");
    let response = wire::parse_response(line.as_bytes()).expect("a readable response");

    // **THE ACKNOWLEDGEMENT IS REQUIRED BEFORE ANY EVENT, AND ITS ABSENCE IS
    // WHY.** Without it, *subscribed to a quiet project* and *the daemon did
    // not understand the request and is about to close* are the same
    // observation: silence.
    let Response::Subscribed { project_id } = response else {
      panic!("subscribing answered {response:?} rather than acknowledging the subscription");
    };
    Feed { reader, project_id }
  }

  /// Read the next event, or fail saying nothing arrived.
  fn next(&mut self) -> Event {
    let mut line = String::new();
    match self.reader.read_line(&mut line) {
      Ok(0) => panic!("the daemon closed the subscription instead of sending an event"),
      Ok(_) => {}
      Err(e) => panic!("no event arrived within {READ_TIMEOUT:?}: {e}"),
    }
    // **THE SHIPPED READER, NOT `serde_json` DIRECTLY.** A test that parsed the
    // line itself would be a second opinion about what an event line is, and
    // would keep passing while the real client could not read the feed at all.
    wire::parse_event(line.as_bytes())
      .unwrap_or_else(|e| panic!("unreadable event line `{line}`: {e}"))
  }

  /// Drain whatever the feed already holds, until it goes quiet.
  ///
  /// **A FRESH SUBSCRIPTION IS NOT A QUIET ONE, AND THE FIRST BUILD OF THIS
  /// FILE ASSUMED IT WAS.** Subscribing OPENS the project, which runs the
  /// migration ladder and can write inside the watched tree -- so the daemon's
  /// own startup can produce a `fileChanged`/`projectChanged` pair that arrives
  /// before anything the test did. Three arms then read the daemon's events as
  /// their own.
  ///
  /// **IT PASSED SIX TIMES ALONE AND FAILED THREE OF FIVE UNDER THE FULL
  /// SUITE**, which is the whole lesson: a green bounds the ring it ran on. On
  /// a quiet box the startup writes settle before the subscribe lands and the
  /// feed really is empty; under load they overlap. **The isolated run was not
  /// evidence that the test was right, only that the race had not fired.**
  ///
  /// The deadline here is short and is on the READ rather than on the claim:
  /// what is required is that the drain TERMINATES.
  fn settle(&mut self) {
    self
      .reader
      .get_ref()
      .set_read_timeout(Some(Duration::from_millis(1200)))
      .expect("a short deadline for draining");
    for _ in 0..256 {
      let mut line = String::new();
      match self.reader.read_line(&mut line) {
        Ok(0) => break,
        Ok(_) if line.trim().is_empty() => break,
        Ok(_) => continue,
        Err(_) => break,
      }
    }
    self
      .reader
      .get_ref()
      .set_read_timeout(Some(READ_TIMEOUT))
      .expect("restore the deadline");
  }

  /// Read events until one satisfies `wanted`, or give up naming what came.
  fn wait_for(&mut self, what: &str, wanted: impl Fn(&Event) -> bool) -> Event {
    let mut seen = Vec::new();
    for _ in 0..64 {
      let event = self.next();
      if wanted(&event) {
        return event;
      }
      seen.push(event);
    }
    panic!("no {what} arrived in 64 events; got {seen:?}");
  }
}

/// Write a canon thread file straight onto disk, as an external editor would.
fn write_thread(root: &Path, id: &str) {
  let path = root.join("intent/.canon/st").join(format!("{id}.json"));
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
  std::fs::write(
    &path,
    format!(
      "{{\n  \"schema\": \"intent/thread@3.0\",\n  \"id\": \"{id}\",\n  \"title\": \"Written by hand\",\n  \"status\": \"wip\",\n  \"created\": \"2026-08-30\",\n  \"objective\": \"\",\n  \"context\": \"\"\n}}\n"
    ),
  )
  .expect("write the thread file");
}

#[test]
fn a_subscriber_learns_the_project_id_before_any_event_arrives() {
  let daemon = RunningDaemon::start();
  let root = common::project("Subscribed");

  let feed = Feed::open(&daemon, &root);

  // **THE ID IS COMPARED AGAINST THE PROJECT'S OWN CONFIG, NOT AGAINST
  // ITSELF.** Asserting only that some id came back would pass on an empty
  // string, and `D20`'s events are unusable without an id a subscriber can
  // attribute.
  let expected = intentsvcs::project::Project::open(&root)
    .expect("the fixture project opens")
    .config()
    .project_id
    .clone()
    .unwrap_or_default();
  assert!(
    !expected.is_empty(),
    "the fixture project has no project_id, so this arm cannot discriminate"
  );
  assert_eq!(
    feed.project_id, expected,
    "the subscription acknowledged a different project than the one it was opened on"
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn an_external_edit_delivers_both_d20_events_in_layer_order() {
  let daemon = RunningDaemon::start();
  let root = common::project("Delivered");

  let mut feed = Feed::open(&daemon, &root);
  feed.settle();
  write_thread(&root, "ST0077");

  // **THE ORDER CLAIM IS ABOUT CAUSATION, NOT ARRIVAL POSITION, AND THE FIRST
  // BUILD CONFUSED THE TWO.** It asserted that the FIRST event on the
  // connection was a `fileChanged` -- true on a quiet box and an accident of
  // quiet, because opening the project can emit events of the daemon's own. The
  // real claim is that the `fileChanged` naming THIS file precedes the
  // `projectChanged` it causes: the disk moves before the model does, so a
  // subscriber redrawing from the store on `fileChanged` would read the state
  // it already had.
  let mine = feed.wait_for(
    "a fileChanged naming ST0077",
    |e| matches!(e, Event::FileChanged { path, .. } if path.ends_with("ST0077.json")),
  );
  let Event::FileChanged { project_id, .. } = &mine else {
    unreachable!("wait_for matched on the variant");
  };
  assert_eq!(project_id, &feed.project_id);

  let changed = feed.wait_for("projectChanged", |e| {
    matches!(e, Event::ProjectChanged { .. })
  });
  let Event::ProjectChanged { project_id } = &changed else {
    unreachable!("wait_for matched on the variant");
  };
  assert_eq!(project_id, &feed.project_id);

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn every_subscriber_receives_every_event() {
  // **THE ARM THAT IS INVISIBLE AT THE SIZE EVERYBODY TESTS AT.** An mpsc
  // fan-out gives each event to exactly one listener, which is
  // indistinguishable from a broadcast with one subscriber and catastrophic
  // with two -- and the failure is silent: each subscriber sees a plausible
  // subset and neither can tell.
  let daemon = RunningDaemon::start();
  let root = common::project("Fanout");

  let mut first = Feed::open(&daemon, &root);
  let mut second = Feed::open(&daemon, &root);
  first.settle();
  second.settle();
  assert_eq!(
    first.project_id, second.project_id,
    "two subscriptions to one project acknowledged different ids"
  );

  write_thread(&root, "ST0078");

  for (which, feed) in [("first", &mut first), ("second", &mut second)] {
    let event = feed.wait_for("fileChanged", |e| matches!(e, Event::FileChanged { .. }));
    let Event::FileChanged { path, .. } = &event else {
      unreachable!("wait_for matched on the variant");
    };
    assert!(
      path.ends_with("ST0078.json"),
      "the {which} subscriber was told about `{}` rather than the file that changed",
      path.display()
    );
  }

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_change_outside_the_sync_scope_delivers_nothing() {
  // The scope rule reaching the feed: a subscriber must not be woken by the
  // daemon's own store writes, which land in `intent/.cache/` inside the
  // watched tree.
  let daemon = RunningDaemon::start();
  let root = common::project("Scoped");

  let mut feed = Feed::open(&daemon, &root);
  // **THE ABSENCE CLAIM BELOW IS ONLY MEANINGFUL FROM A QUIET FEED.** An event
  // the daemon emitted while opening the project would arrive inside the
  // observation window and be read as the write having woken the subscriber.
  feed.settle();

  std::fs::create_dir_all(root.join("intent/.cache")).expect("mkdir");
  std::fs::write(root.join("intent/.cache/scratch"), b"not canon").expect("write");

  // **THE ABSENCE IS OBSERVED WITH A SHORT READ DEADLINE AND THEN THE POSITIVE
  // CONTROL FOLLOWS.** An absence claim proves nothing without evidence the
  // channel was working -- a subscriber whose connection had died delivers
  // exactly this silence.
  feed
    .reader
    .get_ref()
    .set_read_timeout(Some(Duration::from_millis(1500)))
    .expect("a short deadline for the absence");
  let mut line = String::new();
  let quiet = feed.reader.read_line(&mut line);
  assert!(
    quiet.is_err() || line.is_empty(),
    "a write inside `intent/.cache/` delivered `{line}`. That directory holds the daemon's own store, so a subscriber woken by it is being told about the daemon's reaction to itself"
  );

  feed
    .reader
    .get_ref()
    .set_read_timeout(Some(READ_TIMEOUT))
    .expect("restore the deadline");
  write_thread(&root, "ST0079");
  let event = feed.wait_for("fileChanged", |e| matches!(e, Event::FileChanged { .. }));
  let Event::FileChanged { path, .. } = &event else {
    unreachable!("wait_for matched on the variant");
  };
  assert!(
    path.ends_with("ST0079.json"),
    "the feed was still alive but named `{}`",
    path.display()
  );

  let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_subscriptions_are_exactly_d20s_two() {
  // **D20's CEILING, HELD BY THE COMPILER.** The match below is exhaustive, so
  // a third `Event` variant does not fail this assertion -- it fails to
  // COMPILE, which turns *nothing more ships in 3.0.0* from a sentence in a
  // design document into something somebody has to edit a test to get past.
  fn tag(event: &Event) -> &'static str {
    match event {
      Event::ProjectChanged { .. } => "project_changed",
      Event::FileChanged { .. } => "file_changed",
    }
  }

  // **AND THE WIRE FORM IS TYPED OUT BY HAND HERE, IN EXACTLY ONE PLACE.** Both
  // ends of every other test use serde, so a round trip agrees with itself
  // about a line no client could send -- this estate shipped a doubly-tagged
  // request format through two green tests for exactly that reason. These
  // literals are the control.
  let project = Event::ProjectChanged {
    project_id: "p-1".to_string(),
  };
  let file = Event::FileChanged {
    project_id: "p-1".to_string(),
    path: PathBuf::from("/p/intent/.canon/st/ST0001.json"),
  };
  assert_eq!(
    String::from_utf8(wire::frame(&project).expect("serialisable")).expect("utf8"),
    "{\"event\":\"project_changed\",\"project_id\":\"p-1\"}\n"
  );
  assert_eq!(
    String::from_utf8(wire::frame(&file).expect("serialisable")).expect("utf8"),
    "{\"event\":\"file_changed\",\"project_id\":\"p-1\",\"path\":\"/p/intent/.canon/st/ST0001.json\"}\n"
  );

  assert_eq!(tag(&project), "project_changed");
  assert_eq!(tag(&file), "file_changed");
}
