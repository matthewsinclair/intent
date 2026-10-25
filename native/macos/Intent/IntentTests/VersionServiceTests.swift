import Foundation
import Testing

@testable import Intent

/// The masthead row's `{version}`, `{commit}` and `{kind}`, read from `intent
/// version` (issue 0607), in Gtools' `CLIVersion` shape. What these pin is the
/// Swift side's reading of the shapes it knows, written as literals copied from
/// spine.rs's `"{version} ({SOURCE_COMMIT}) {SOURCE_KIND}"`, and from the
/// kind-less line an `intent` before 3.2.1 prints, which the app still meets.
/// They cannot see a change to that line on the Rust side: a line in another
/// shape is all version, by design, which is why the CLI's own tests pin the
/// line itself.
@Suite("Version row")
struct VersionServiceTests {
  /// **THE KIND SURVIVES** (issue 0534). It is the word that says whether the
  /// `intent` the app drives is the release or a dev build.
  @Test("a release build keeps its kind, its sha cut to its first eight")
  func releaseKeepsItsKind() {
    #expect(
      VersionService.parse("intent 3.2.1 (8a48430ee9b8dceeb5ecebb83a678941bc44848a) release\n")
        == CLIVersion(version: "3.2.1", commit: "8a48430e", kind: "release"))
  }

  @Test("a dirty dev build keeps its marker and its kind")
  func dirtyDevKeepsBoth() {
    #expect(
      VersionService.parse("intent 3.2.1 (dirty-8a48430ee9b8dceeb5ecebb83a678941bc44848a) dev\n")
        == CLIVersion(version: "3.2.1", commit: "dirty-8a48430e", kind: "dev"))
  }

  @Test("a line with no kind, as an intent before 3.2.1 prints, gives none")
  func noKind() {
    #expect(
      VersionService.parse("intent 3.0.1 (8a48430ee9b8dceeb5ecebb83a678941bc44848a)\n")
        == CLIVersion(version: "3.0.1", commit: "8a48430e", kind: nil))
  }

  /// The dirt marker survives the shortening. A commit that dropped it would
  /// report a dirty build as the clean commit it was built beside.
  @Test("a dirty build keeps its marker")
  func dirtyKeepsMarker() {
    #expect(
      VersionService.parse("intent 3.0.2 (dirty-8a48430ee9b8dceeb5ecebb83a678941bc44848a)\n")
        == CLIVersion(version: "3.0.2", commit: "dirty-8a48430e", kind: nil))
  }

  @Test("a SHA-256 commit is cut the same way")
  func sha256() {
    let sha256 = "8a48430ee9b8dceeb5ecebb83a678941bc44848a8a48430ee9b8dceeb5ecebb8"
    #expect(
      VersionService.parse("intent 3.0.1 (\(sha256))") == CLIVersion(version: "3.0.1", commit: "8a48430e", kind: nil))
  }

  /// `unknown` is the CLI's own word for a commit git could not answer.
  @Test("an unknown commit is kept as given")
  func unknownCommit() {
    #expect(
      VersionService.parse("intent 3.0.1 (unknown)\n") == CLIVersion(version: "3.0.1", commit: "unknown", kind: nil))
    #expect(
      VersionService.parse("intent 3.2.1 (unknown) dev\n") == CLIVersion(version: "3.2.1", commit: "unknown", kind: "dev"))
  }

  @Test("a line with no commit gives the version alone")
  func noCommit() {
    #expect(VersionService.parse("intent 4.0.0-rc1\n") == CLIVersion(version: "4.0.0-rc1", commit: nil, kind: nil))
  }

  @Test("a line in any other shape is all version, never parsed into something the CLI did not say")
  func unrecognisedShape() {
    #expect(
      VersionService.parse("intent version 4.0.0 (beta) tonight\n")
        == CLIVersion(version: "intent version 4.0.0 (beta) tonight", commit: nil, kind: nil))
    #expect(VersionService.parse("warning: stale\n") == CLIVersion(version: "warning: stale", commit: nil, kind: nil))
  }

  @Test("empty output is no answer")
  func emptyOutput() {
    #expect(VersionService.parse("") == nil)
    #expect(VersionService.parse("  \n") == nil)
  }

  /// **A FAILED READ IS NOT A PENDING ONE.** This pins the states only: that
  /// `refresh()` sets `failed` on a thrown or empty read is not reached here,
  /// because the CLI call is not injectable.
  @Test("a failed read keeps its cause, and a pending one has none")
  func failedIsNotPending() {
    let failed = VersionState.failed("intent was not found on the login shell's PATH")
    #expect(failed.failure == "intent was not found on the login shell's PATH")
    #expect(failed.answer == nil)
    #expect(VersionState.pending.failure == nil)
    #expect(VersionState.pending.answer == nil)
  }

  /// AT-03.4: **THE 3.2.7 LINE** (ST0087 WP-01, the contract's T4) names its
  /// tree, a twelve-character sha cut to eight, the root and the kind.
  @Test("the tree line gives its tree, sha, root and kind")
  func treeLineGivesEveryField() {
    #expect(
      VersionService.parse("intent 3.2.7 (dev, 8a48430ee9b8) at /Users/m/Devel/prj/Intent dev\n")
        == CLIVersion(
          version: "3.2.7", commit: "8a48430e", kind: "dev", tree: "dev",
          root: "/Users/m/Devel/prj/Intent"))
  }

  /// AT-03.4: a dev tree whose support set differs from its build keeps its
  /// `+dirty`, so it never reads as a clean build.
  @Test("the tree line keeps +dirty")
  func treeLineKeepsDirty() {
    #expect(
      VersionService.parse("intent 3.2.7 (dev, 8a48430ee9b8+dirty) at /src/Intent dev\n")
        == CLIVersion(
          version: "3.2.7", commit: "8a48430e+dirty", kind: "dev", tree: "dev",
          root: "/src/Intent"))
  }

  /// AT-03.4: the brew form names the opt link as its root, and a root may
  /// hold a space.
  @Test("the brew line names its opt link, spaces and all")
  func brewLineNamesItsRoot() {
    #expect(
      VersionService.parse(
        "intent 3.2.7 (brew, 8a48430ee9b8) at /opt/homebrew/opt/intent/libexec release\n")
        == CLIVersion(
          version: "3.2.7", commit: "8a48430e", kind: "release", tree: "brew",
          root: "/opt/homebrew/opt/intent/libexec"))
    #expect(
      VersionService.parse(
        "intent 3.2.7 (opt, 8a48430ee9b8) at /Users/m/Application Support/intent release")
        == CLIVersion(
          version: "3.2.7", commit: "8a48430e", kind: "release", tree: "opt",
          root: "/Users/m/Application Support/intent"))
  }

  /// Issue 0646: hv ruled on 2026-10-25 that the line drops ` at <root>`, so
  /// the tree line without it reads the same fields, its root nil.
  @Test("the tree line without a root gives its tree, sha and kind")
  func treeLineWithoutRoot() {
    #expect(
      VersionService.parse("intent 3.2.7 (dev, 397d7568e1d8) dev\n")
        == CLIVersion(version: "3.2.7", commit: "397d7568", kind: "dev", tree: "dev"))
    #expect(
      VersionService.parse("intent 3.2.7 (brew, 397d7568e1d8) release")
        == CLIVersion(version: "3.2.7", commit: "397d7568", kind: "release", tree: "brew"))
  }

  /// AT-03.4: `(dev, unknown)` is a dev tree whose git could not be read.
  @Test("the (dev, unknown) line keeps unknown as its commit")
  func devUnknownLine() {
    #expect(
      VersionService.parse("intent 3.2.7 (dev, unknown) at /src/Intent dev\n")
        == CLIVersion(
          version: "3.2.7", commit: "unknown", kind: "dev", tree: "dev", root: "/src/Intent"))
  }
}
