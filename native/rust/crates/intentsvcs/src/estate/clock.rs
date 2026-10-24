//! **The estate's one clock** (hv, 2026-10-24, vc decision 72; the reason is
//! recorded at ST0056's D42, beside the daemon log's).
//!
//! The estate view is a reading of the live host, not a record: when it was
//! taken, and how long ago a session last moved, are the reading's own
//! subject, and no store write exists to stamp them. So it is read here, once
//! per reading, and nowhere else: `estate::status` reads it for `taken_at`,
//! intentd's census tick for the ring, and every age is derived from that one
//! value. `one_clock.rs` names this file in its `EXEMPT` list; nothing it
//! returns enters a store, an extract or a view.

use time::OffsetDateTime;

/// What time it is, for one estate reading.
pub fn now() -> OffsetDateTime {
  OffsetDateTime::now_utc()
}
