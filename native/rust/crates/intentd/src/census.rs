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
  Found {
    // Slots are WP-04's, so none are running until then.
    sample: whole.then(|| census::sample(0)),
    pruned: whole.then(estate::host_prune),
    endpoint: read_endpoint.then(estate::host_endpoint),
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
