import Foundation
import OSLog

/// THE app's logging subsystem, named once: every category's `Logger` comes
/// from here. The subsystem is the bundle identifier, whose one home is
/// project.yml's `PRODUCT_BUNDLE_IDENTIFIER`, so a rename there reaches every
/// category and a `log stream` filtered on the subsystem shows them all. Typed
/// out in each file, a rename that missed one copy would leave that category
/// logging under the old name, where the filter no longer finds it.
enum AppLog {
  static func logger(_ category: String) -> Logger {
    Logger(subsystem: Bundle.main.bundleIdentifier ?? ProcessInfo.processInfo.processName, category: category)
  }
}
