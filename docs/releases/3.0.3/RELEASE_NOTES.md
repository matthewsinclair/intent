# Intent v3.0.3

**v3.0.3 is v3.0.2, published.** The v3.0.2 tag was cut and its GitHub release created, but no artefacts were ever published under it: the release pipeline refused to stage the 3.0.2 support tree at the tag, and the publish step refuses anything not staged from a clean checkout at the tag. v3.0.3 carries the fix to that pipeline and nothing that changes what `intent` or `intentd` do.

**If you are upgrading from v3.0.1, the [v3.0.2 release notes](../3.0.2/RELEASE_NOTES.md) are the notes for this upgrade.** Everything they describe ships in v3.0.3, and their Upgrading section applies in full: your user state moves to the XDG layout on the first run, and each project's pre-commit carrier needs `intent claude upgrade --apply --skip-settings` to follow it.

## Provenance

Every claim below was verified against the tree the release is cut from, by reading the code that produces the behaviour. The only change between the v3.0.2 tag and this release that touches code is the one under Fixed.

## Fixed

- **`int macos prepare` stages the support tree again.** Its support-tree guard scanned the source for every `home.join(...)` chain and treated each as a path under the install root. v3.0.2 added chains that are not install-root paths -- the XDG data directory, the legacy `~/.intent`, `Library/LaunchAgents`, and the whiteboard's board home -- so the guard refused the tree at the tag. It now reads only install-root chains, and the two parameters that name the install root are called `install` rather than `home`. This is release tooling and two parameter names; the shipped binaries behave exactly as v3.0.2's would have.

## Upgrading

- **From v3.0.1:** follow the Upgrading section of the [v3.0.2 notes](../3.0.2/RELEASE_NOTES.md).
- **From a v3.0.2 build of this repository:** nothing to do. The store schema, the user-state layout and the carrier are unchanged from v3.0.2.
