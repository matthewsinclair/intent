import Foundation

/// The one polling loop the app's services share: run a tick, sleep, repeat
/// until stopped. **ONE HOME FOR THE CADENCE**: the daemon's health, the
/// project's thread count and the CLI's version each held a copy of this loop
/// with the interval as a literal, kept in step by a comment saying they
/// mirrored each other. A change to one copy's interval or cancellation would
/// not have reached the other two.
@MainActor
final class Poller {
  private static let interval: Duration = .seconds(5)

  private var task: Task<Void, Never>?

  /// Starts the loop; a second call while running is a no-op. The owner passes
  /// a tick that captures itself weakly, so the loop never keeps its service
  /// alive.
  func start(_ tick: @escaping @Sendable @MainActor () async -> Void) {
    guard task == nil else { return }
    task = Task {
      while !Task.isCancelled {
        await tick()
        try? await Task.sleep(for: Self.interval)
      }
    }
  }

  func stop() {
    task?.cancel()
    task = nil
  }
}
