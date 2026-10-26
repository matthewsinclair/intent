//! The census tick (ST0086 WP-02): once a minute it samples the host into
//! the ring the estate page draws its sparklines from and reaps Intent's WIP
//! records for gone sessions, and it reads the account's usage endpoint at
//! most once (vc decision 73).
//!
//! **THE ENDPOINT IS READ ONLY FOR A DAEMON SOMEBODY ASKS.** A tick reads it
//! when the estate was asked for since the last read, and the first ask of
//! a daemon holding no reading wakes the tick at once. So a daemon whose
//! estate nobody opens -- every test daemon -- never touches the Keychain or
//! the network, and a page polling every few seconds costs one read a tick.
//!
//! **NOT A RECORD.** The ring and the endpoint reading live in memory, and a
//! restart starts them empty (design.md). They are one per process by
//! definition, so they are process-wide statics rather than state threaded
//! through every connection that might ask.

use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use intentsvcs::estate::census::{self, Sample};
use intentsvcs::estate::{self, Held, usage};
use intentsvcs::remedy::Remedy;

use crate::daemon_log::elogln;

/// A minute apart, as the page's sparklines are drawn.
const TICK: Duration = Duration::from_secs(60);

/// An hour of ticks.
const KEPT: usize = 60;

static RING: Mutex<VecDeque<Sample>> = Mutex::new(VecDeque::new());

static ENDPOINT: Mutex<Option<usage::Endpoint>> = Mutex::new(None);

/// The estate was asked for since the endpoint was last read.
static ASKED: AtomicBool = AtomicBool::new(false);

/// Wakes the tick for the first ask of a daemon holding no reading.
static FIRST_ASK: std::sync::LazyLock<tokio::sync::Notify> =
  std::sync::LazyLock::new(tokio::sync::Notify::new);

/// What the tick holds, for the estate reader; asking is what arms the
/// endpoint read.
pub fn held() -> Held {
  ASKED.store(true, Ordering::Relaxed);
  let endpoint = ENDPOINT
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner)
    .clone();
  if endpoint.is_none() {
    FIRST_ASK.notify_one();
  }
  Held {
    history: RING
      .lock()
      .unwrap_or_else(std::sync::PoisonError::into_inner)
      .iter()
      .cloned()
      .collect(),
    endpoint,
  }
}

/// Adds a sample, dropping the oldest past [`KEPT`].
fn push(ring: &mut VecDeque<Sample>, sample: Sample) {
  ring.push_back(sample);
  while ring.len() > KEPT {
    ring.pop_front();
  }
}

/// One job's last failure, so a failure is logged when it starts or changes
/// and not every minute it persists.
#[derive(Default)]
struct Noted(Option<String>);

impl Noted {
  fn ok(&mut self) {
    self.0 = None;
  }

  fn failed(&mut self, what: &str, why: String, remedy: &str) {
    if self.0.as_deref() != Some(why.as_str()) {
      elogln!("warning: intentd could not {what}: {why}\n  remedy: {remedy}");
      self.0 = Some(why);
    }
  }
}

/// What one round found, gathered off the runtime; `None` for a job the
/// round did not run.
struct Found {
  sample: Option<Result<Sample, intentsvcs::estate::Unreadable>>,
  pruned: Option<Result<estate::Pruned, estate::EstateError>>,
  endpoint: Option<Result<usage::Endpoint, estate::EstateError>>,
}

fn gather(whole: bool, read_endpoint: bool) -> Found {
  run(
    whole,
    read_endpoint,
    // The ring records how many slot requests are running (WP-04); a slot
    // read that fails is the sample's failure, never a silent zero.
    || {
      estate::host_slots_running()
        .map_err(|e| intentsvcs::estate::Unreadable {
          path: "slots".to_string(),
          why: e.to_string(),
          pid: None,
        })
        .and_then(census::sample)
    },
    estate::host_prune,
    estate::host_endpoint,
  )
}

/// One round over the jobs it is handed: a whole tick samples and reaps, and
/// the endpoint is read only when somebody asked. Separate from [`gather`] so
/// a test can hand it jobs that count their calls rather than the host's.
fn run(
  whole: bool,
  read_endpoint: bool,
  sample: impl FnOnce() -> Result<Sample, intentsvcs::estate::Unreadable>,
  prune: impl FnOnce() -> Result<estate::Pruned, estate::EstateError>,
  endpoint: impl FnOnce() -> Result<usage::Endpoint, estate::EstateError>,
) -> Found {
  Found {
    sample: whole.then(sample),
    pruned: whole.then(prune),
    endpoint: read_endpoint.then(endpoint),
  }
}

/// Runs the tick for as long as the process runs.
///
/// **A FAILED SAMPLE LEAVES A GAP IN THE RING** rather than a made-up point,
/// and the page reads the gap. A failed endpoint reading is kept like a good
/// one: it carries its failure, and the page names it in `usage.error`.
pub async fn tick() {
  let mut every = tokio::time::interval(TICK);
  every.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
  let (mut sampling, mut pruning, mut reading) =
    (Noted::default(), Noted::default(), Noted::default());
  loop {
    let whole = tokio::select! {
      _ = every.tick() => true,
      () = FIRST_ASK.notified() => false,
    };
    let read_endpoint = ASKED.swap(false, Ordering::Relaxed);
    if !whole && !read_endpoint {
      continue;
    }
    let found = match tokio::task::spawn_blocking(move || gather(whole, read_endpoint)).await {
      Ok(found) => found,
      Err(e) => {
        elogln!("warning: the estate census tick did not finish: {e}");
        continue;
      }
    };
    match found.sample {
      Some(Ok(sample)) => {
        sampling.ok();
        push(
          &mut RING
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
          sample,
        );
      }
      Some(Err(u)) => sampling.failed(
        "sample the host's load for the estate census",
        u.why,
        "the estate page's load history has a gap until it can; the rest of the estate is unaffected",
      ),
      None => {}
    }
    match found.pruned {
      Some(Ok(_)) => pruning.ok(),
      Some(Err(e)) => pruning.failed(
        "reap the WIP records of gone sessions",
        e.to_string(),
        &e.remedy(),
      ),
      None => {}
    }
    match found.endpoint {
      Some(Ok(endpoint)) => {
        reading.ok();
        *ENDPOINT
          .lock()
          .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(endpoint);
      }
      Some(Err(e)) => reading.failed("read the account's usage", e.to_string(), &e.remedy()),
      None => {}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::Cell;

  /// ST0086 AT-02.2
  ///
  /// Which jobs one round runs, by what the round is: a whole tick samples
  /// and reaps the WIP records of gone sessions, a wake for a first ask does
  /// neither, and the endpoint is read only when the estate was asked for.
  #[test]
  fn a_round_reaps_on_the_tick_and_reads_the_endpoint_only_when_asked() {
    for whole in [false, true] {
      for asked in [false, true] {
        let (sampled, pruned, read) = (Cell::new(0), Cell::new(0), Cell::new(0));
        let found = run(
          whole,
          asked,
          || {
            sampled.set(sampled.get() + 1);
            Err(intentsvcs::estate::Unreadable {
              path: "fixture".to_string(),
              why: "not measured in a test".to_string(),
              pid: None,
            })
          },
          || {
            pruned.set(pruned.get() + 1);
            Ok(estate::Pruned {
              removed: Vec::new(),
            })
          },
          || {
            read.set(read.get() + 1);
            Ok(Err(usage::EndpointFailure::Status { status: 401 }))
          },
        );
        assert_eq!(
          sampled.get(),
          u32::from(whole),
          "whole {whole}, asked {asked}: sampled"
        );
        assert_eq!(
          pruned.get(),
          u32::from(whole),
          "whole {whole}, asked {asked}: reaped"
        );
        assert_eq!(
          read.get(),
          u32::from(asked),
          "whole {whole}, asked {asked}: read"
        );
        assert_eq!(found.pruned.is_some(), whole);
        assert_eq!(found.endpoint.is_some(), asked);
      }
    }
  }

  /// ST0086 AT-02.6
  ///
  /// Asking for the estate is what arms the next endpoint read.
  #[test]
  fn asking_for_the_estate_arms_the_endpoint_read() {
    ASKED.store(false, Ordering::Relaxed);
    let _ = held();
    assert!(
      ASKED.swap(false, Ordering::Relaxed),
      "an ask did not arm the endpoint read"
    );
  }

  /// ST0086 AT-02.2
  #[test]
  fn the_ring_keeps_the_last_hour_oldest_first() {
    let mut ring = VecDeque::new();
    for n in 0..(KEPT + 3) {
      push(
        &mut ring,
        Sample {
          at: format!("{n}"),
          load1: 0.0,
          slots_running: 0,
        },
      );
    }
    assert_eq!(ring.len(), KEPT);
    assert_eq!(ring.front().map(|s| s.at.as_str()), Some("3"));
    assert_eq!(
      ring.back().map(|s| s.at.clone()),
      Some(format!("{}", KEPT + 2))
    );
  }
}
