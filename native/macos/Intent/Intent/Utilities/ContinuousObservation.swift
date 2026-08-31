import Foundation
import Observation

/// Observation that survives its first change. Ported from Geodica/Conflab; the
/// lesson it carries applies unchanged.
///
/// `withObservationTracking(_:onChange:)` is one-shot: it fires `onChange`
/// exactly once, before the first write to anything the tracked closure read,
/// and then stops observing. Keeping a view in sync therefore means re-arming
/// after every change. This type owns that, so no call site has to.
///
/// Two things live here so that no call site has to get them right:
///
/// - **Re-arming.** Re-arming by hand gives every call site a path where an
///   early return skips it, and the view then stops updating for the life of
///   the process -- with no crash and no log. A silent dead UI is a worse
///   failure than a wasteful one.
/// - **Ordering, and re-entrancy.** `onChange` fires *before* the new value is
///   written, so reading the observed property inside it yields the value being
///   replaced. Worse, re-arming from inside the notification re-enters the
///   Observation registrar while it is still delivering -- a variant that did
///   the work synchronously via `MainActor.assumeIsolated` instead of hopping
///   did not merely read stale, it aborted the process on the first delivery.
///   Deferring to the next main-actor turn fixes both, here, once.
///
/// The observation ends when this object is released, so a view that holds one
/// in a property gets teardown for free. `stop()` is for ending it earlier.
@MainActor
final class ContinuousObservation {
  private let track: @MainActor () -> Void
  private let onChange: @MainActor () -> Void
  private var stopped = false

  /// Invokes `onChange` immediately to render current state, then again after
  /// every change to any `@Observable` property that `track` reads.
  ///
  /// `track` is re-run on each re-arm, so a closure whose reads depend on
  /// current state observes whatever it reads *this* time -- conditional reads
  /// are tracked correctly rather than pinned to the first pass.
  init(
    track: @escaping @MainActor () -> Void,
    onChange: @escaping @MainActor () -> Void
  ) {
    self.track = track
    self.onChange = onChange
    onChange()
    arm()
  }

  /// Ends the observation. Idempotent; a change already in flight will not
  /// deliver after this returns.
  func stop() {
    stopped = true
  }

  private func arm() {
    guard !stopped else { return }
    withObservationTracking(track) { [weak self] in
      // Runs before the write lands, and on whichever actor performed it.
      // Hopping to the main actor both satisfies isolation and defers the read
      // until the new value is actually there.
      Task { @MainActor in
        guard let self, !self.stopped else { return }
        self.onChange()
        self.arm()
      }
    }
  }
}
