# Installation

Intent is a binary plus a support tree. The support tree is not optional: several commands resolve files out of it at runtime, and an install missing it breaks those commands. **Not all of them fail when it does.** Some refuse with an error, and `intent claude rules list`, `intent claude skills list` and `intent claude subagents list` answer at exit 0 however little they find, which is why the check below is read by its output rather than by its exit code.

## Homebrew (macOS on Apple silicon)

```
  $ brew install matthewsinclair/intent/intent
  $ intent --version
```

This is the supported path. The formula installs the CLI (`intent`), the daemon (`intentd`) and the support tree into the keg, and sets no environment variable: the binary finds the tree from its own location, as a source build does (below).

**The formula is macOS on Apple silicon only.** It declares `depends_on arch: :arm64` and `depends_on :macos`, and no Intel or Linux binary is built. Anywhere else, build [from source](#from-source).

**The CLI does not need the daemon.** Every `intent` command does its work in-process unless you pass `--daemon`. If you want `intentd` running, `intent daemon start` starts it, and `intent daemon status` and `intent daemon stop` do what they say. The formula also declares a Homebrew service, so `brew services start intent` keeps `intentd` running under launchd instead; its stdout and stderr go to `intentd.log` under Homebrew's `var/log`.

## The menubar app

The Intent menubar app ships as `Intent.app.zip` on each [GitHub release](https://github.com/matthewsinclair/intent/releases). **The Homebrew formula does not install it.** It needs macOS 14 or later, and it runs the `intent` it finds on your login shell's `PATH`, so install the CLI first. To install the app, download `Intent.app.zip` from the release, unzip it, and move `Intent.app` to `/Applications`.

On its first run the app registers itself as a login item, so the menubar comes back after you log in. `intent app status` reports whether it is running and where it found it; `intent app start`, `stop` and `restart` look for it in `/Applications`.

## From source

Requires stable Rust 1.85 or later; the workspace is edition 2024.

```
  $ git clone https://github.com/matthewsinclair/intent
  $ cd intent/native/rust
  $ cargo build --release
```

The binaries land in `native/rust/target/release/`. **The binary finds its own templates, rule library and skills, and nothing needs setting.** `intentsvcs::install::home()` reads `std::env::current_exe()`, canonicalises it, and walks up until it reaches a directory containing `lib/templates`. A source build's binary therefore resolves to the repository root by construction. Symlinks are resolved before that walk, because a packaged `intent` is reached through one (Homebrew's `bin/intent` points into the Cellar), and walking up from the link would climb the wrong tree entirely.

So a source build needs only the binary on `PATH`:

```
  $ export PATH="/path/to/intent/native/rust/target/release:$PATH"
```

**Do not set `INTENT_HOME`. v3 never reads it.** It is v2's variable. v3 resolves its install root from its own location and nothing else, so setting the variable cannot point a binary at a different tree and cannot repair an install that is missing part of its support tree. `intent info` prints the resolved root on a line labelled `INTENT_HOME`; that line is output, not a setting.

## Verifying an install

```
  $ intent --version
  $ intent claude rules list
```

**`intent claude rules list` is the install check, and you read its output, not its exit code.** It reads the rule library out of the install root. A healthy install lists rules and ends with a `total:` line. An install missing the library ends instead with `total: 0 rule(s) -- and there is NO RULE LIBRARY at <path>, so this install is incomplete rather than empty`, and **both exit 0**. `intent claude skills list` answers `no skills in this install` and `intent claude subagents list` answers `no subagents in this install` when their trees are missing, also at exit 0.

**`intent doctor` is a different tool and it will not tell you this.** It reports on the _project_ you are standing in: backup staleness, a thread whose recorded status disagrees with its own gate, a generated view that differs from the store. Useful, and it inspects none of the install. Run it once you have a project; it is not an installation check.

## What the install has to contain

Intent resolves these paths against its install root. **A binary on its own is not an installation**, and the failure mode is per-command rather than global, which is why it is worth knowing what is meant to be there.

| Path under the install root                   | Resolver                          | What stops working without it                                                      |
| --------------------------------------------- | --------------------------------- | ---------------------------------------------------------------------------------- |
| `lib/templates/`                              | `intentsvcs::canon`               | `intent init`, `intent upgrade`, every generated hook body, both whiteboard guards |
| `intent/plugins/claude/rules/`                | `intentsvcs::rules::Library::new` | `intent claude rules list` / `show`, `intent critic <lang>`                        |
| `intent/plugins/claude/skills/`               | `intentsvcs::payload`             | `intent claude skills list` / `install` / `sync` / `uninstall`                     |
| `intent/plugins/claude/subagents/`            | `intentsvcs::payload`             | `intent claude subagents list` / `install` / `sync` / `uninstall`                  |
| `intent/plugins/`                             | `intentsvcs::plugins::root`       | Parent of the trees above; ships via its children                                  |
| `intent/plugins/claude/bin/intent_claude_cwi` | `intentsvcs::install::cwi_script` | `intent claude start`, `intent claude ws new` / `list` / `archive` / `hygiene`     |

On a Homebrew install these root at `$KEG/libexec`, which is what `intent info` reports on its `INTENT_HOME` line. The support archive is rooted at the install root, so the tree lands directly under `libexec`:

```
  $KEG/bin/intent, intentd                          symlinks into libexec
  $KEG/libexec/bin/intent, intentd
  $KEG/libexec/lib/templates/
  $KEG/libexec/intent/plugins/claude/rules/
  $KEG/libexec/intent/plugins/claude/skills/
  $KEG/libexec/intent/plugins/claude/bin/intent_claude_cwi
```

**The v3.0.1 keg carries no `intent/plugins/claude/subagents/` tree**, so on a Homebrew install `intent claude subagents list` answers `no subagents in this install` and there are no subagents to install. See [Known defects](known-defects.md).

If a command fails with an error naming a path rather than an argument, this table is where to look first. **Nothing in the tool audits this table for you**; the closest thing is running one command per row and seeing whether it works. For the rules, skills and subagents rows, "works" means it lists something, because their empty answer comes back at exit 0.

## Upgrading from v3.0.0

```
  $ brew upgrade intent
```

The v3.0.0 keg shipped without the rule library, the skills and `intent_claude_cwi`; the v3.0.1 keg carries all three. **The first v3.0.1 command to touch a project migrates its store in place, and nothing migrates it back**, so take `intent backup` with v3.0.0 first if you might need to go back. The [v3.0.1 release notes](releases/3.0.1/RELEASE_NOTES.md) cover the migration and everything else that changed.

A project whose views were rendered by v3.0.0 reports every one of them as view-skew under v3.0.1's `intent doctor`. [Known defects](known-defects.md) says what clears it.

## Known defects

[Known defects](known-defects.md) lists every defect in v3.0.1 that a reader can reach by following the documentation correctly, including the ones above.

---

Next: [Getting started](getting-started.md).
