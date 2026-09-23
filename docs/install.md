# Installation

Intent is a binary plus a support tree. The support tree is not optional: several commands resolve files out of it at runtime, and an install missing it breaks those commands. **Not all of them fail when it does.** Some refuse with an error, and `intent claude rules list`, `intent claude skills list` and `intent claude subagents list` answer at exit 0 however little they find, which is why the check below is read by its output rather than by its exit code.

## Homebrew (macOS on Apple silicon)

```
  $ brew install matthewsinclair/intent/intent
  $ intent --version
```

This is the supported path. The formula installs the CLI (`intent`), the daemon (`intentd`) and the support tree into the keg, and sets no environment variable: the binary finds the tree from its own location, as a source build does (below).

**Run `intent bootstrap` once on each machine, after installing.** It writes the install pointer, `~/.local/share/intent/home` (under `$XDG_DATA_HOME` when that is set), and your per-user config at `~/.config/intent/config.json`. A project's pre-commit gate finds Intent through that pointer, so until it exists every commit in a project with the gate installed is refused, and the refusal names `intent bootstrap`. Homebrew cannot run it for you, because a formula's `post_install` cannot write to your home directory, so the formula prints a caveat saying this. A source build needs it too. Re-running it repoints the pointer at the install you run it from and leaves an existing config alone; `--force` replaces the config. `intent bootstrap --check` writes nothing: it prints where the pointer sends the gate and exits 1 when the gate cannot run.

**The formula is macOS on Apple silicon only.** It declares `depends_on arch: :arm64` and `depends_on :macos`, and no Intel or Linux binary is built. Anywhere else, build [from source](#from-source).

**The CLI does not need the daemon.** Every `intent` command except `intent graphql` does its work in-process unless you pass `--daemon`; `intent graphql` is answered only by a running `intentd`. If you want `intentd` running, `intent daemon start` starts it, and `intent daemon status` and `intent daemon stop` do what they say. The formula also declares a Homebrew service, so `brew services start intent` keeps `intentd` running under launchd instead; its stdout and stderr go to `intentd.log` under Homebrew's `var/log`.

## The menubar app

The Intent menubar app ships as `Intent.app.zip` on each [GitHub release](https://github.com/matthewsinclair/intent/releases). **The Homebrew formula does not install it.** It needs macOS 14 or later, and it runs the `intent` it finds on your login shell's `PATH`, so install the CLI first. To install the app, download `Intent.app.zip` from the release, unzip it, and move `Intent.app` to `/Applications`.

On its first run the app registers itself as a login item, so the menubar comes back after you log in. `intent app status` reports whether it is running and where it found it; `intent app start`, `stop` and `restart` look for it in `/Applications`.

## From source

Requires stable Rust 1.90 or later: the workspace is edition 2024, and 1.90 is the highest `rust-version` a locked dependency declares. CI builds with the current stable toolchain.

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

## Per-user files

Intent keeps each user's files in the XDG Base Directory layout. Configuration lives under `$XDG_CONFIG_HOME/intent/` (default `~/.config/intent/`). Data lives under `$XDG_DATA_HOME/intent/` (default `~/.local/share/intent/`): the install pointer, the skill and subagent manifests, and `ext/`. `intentd`'s logs live under `$XDG_STATE_HOME/intent/` (default `~/.local/state/intent/`). Its runtime files live under `$XDG_RUNTIME_DIR/intent/`, or under `~/.local/state/intent/run/` when that variable is unset, as it always is on macOS. A variable that is unset, empty or not an absolute path takes its default.

The first command of v3.0.2 or later moves what an earlier build kept in `~/.intent/` into this layout, and removes `~/.intent/` when that leaves it empty. Anything else in it is left alone.

## Verifying an install

```
  $ intent --version
  $ intent claude rules list
```

**`intent claude rules list` is the install check, and you read its output, not its exit code.** It reads the rule library out of the install root. A healthy install lists rules and ends with a `total:` line. An install missing the library ends instead with `total: 0 rule(s) -- and there is NO RULE LIBRARY at <path>, so this install is incomplete rather than empty`, and **both exit 0**. `intent claude skills list` answers `no skills in this install` and `intent claude subagents list` answers `no subagents in this install` when their trees are missing, also at exit 0.

**`intent doctor` is a different tool and it will not tell you this.** It reports on the _project_ you are standing in: backup staleness, a thread whose recorded status disagrees with its own gate, a generated view that differs from the store. Useful, and it does not check the support tree: the only part of the install it reads is the hook templates it compares the project's pre-commit gate against. Run it once you have a project; it is not an installation check.

## What the install has to contain

Intent resolves these paths against its install root. **A binary on its own is not an installation**, and the failure mode is per-command rather than global, which is why it is worth knowing what is meant to be there.

| Path under the install root                   | Resolver                          | What stops working without it                                                      |
| --------------------------------------------- | --------------------------------- | ---------------------------------------------------------------------------------- |
| `lib/templates/`                              | `intentsvcs::canon`               | `intent init`, `intent upgrade`, every generated hook body, both whiteboard guards |
| `intent/plugins/claude/rules/`                | `intentsvcs::rules::Library::new` | `intent claude rules list` / `show`, `intent critic <lang>`                        |
| `intent/plugins/claude/skills/`               | `intentsvcs::payload`             | `intent claude skills list` / `install` / `sync` / `uninstall`                     |
| `intent/plugins/claude/subagents/`            | `intentsvcs::payload`             | `intent claude subagents list` / `install` / `sync` / `uninstall`                  |
| `intent/plugins/<name>/plugin.json`           | `intentsvcs::plugins::root`       | `intent plugin list` / `show` (each plugin is a directory holding a `plugin.json`) |
| `intent/plugins/claude/bin/intent_claude_cwi` | `intentsvcs::install::cwi_script` | `intent claude start`                                                              |

On a Homebrew install these root at `$KEG/libexec`, which is what `intent info` reports on its `INTENT_HOME` line. The support archive is rooted at the install root, so the tree lands directly under `libexec`:

```
  $KEG/bin/intent, intentd                          symlinks into libexec
  $KEG/libexec/bin/intent, intentd
  $KEG/libexec/lib/templates/
  $KEG/libexec/intent/plugins/claude/rules/
  $KEG/libexec/intent/plugins/claude/skills/
  $KEG/libexec/intent/plugins/claude/subagents/
  $KEG/libexec/intent/plugins/claude/bin/intent_claude_cwi
```

If a command fails with an error naming a path rather than an argument, this table is where to look first. **Nothing in the tool audits this table for you**; the closest thing is running one command per row and seeing whether it works. For the rules, skills and subagents rows, "works" means it lists something, because their empty answer comes back at exit 0.

## Upgrading

```
  $ brew upgrade intent
```

**The first command of a newer build to open a project's store migrates it in place, and nothing migrates it back.** An older build then refuses that store and says it was written by a newer Intent. If you might need to go back, take `intent backup` with the older build first.

**Views an older build rendered report as `stale-render` in `intent doctor`.** This is advisory and does not block a commit, and `intent sync --to-disk` brings the views up to date.

**From v3.2.0 or earlier, run `intent bootstrap` once after the upgrade.** Those builds recorded the versioned keg, `<prefix>/Cellar/intent/<version>/libexec`, as the install pointer, and `brew upgrade` deletes that keg, so until you run it every commit in a project with the gate installed is refused, and the refusal names the command. From v3.2.1, `intent bootstrap` records `<prefix>/opt/intent/libexec`, Homebrew's link to whichever version is current, so later upgrades need nothing. `intent bootstrap --check` says whether the pointer still names a versioned keg.

**From v3.0.1 or earlier,** the first command of v3.0.2 or later moves your per-user files out of `~/.intent/` (see [Per-user files](#per-user-files)). A pre-commit gate installed by the older build still reads the old pointer, so run `intent claude upgrade --apply --skip-settings` in each project, and run `intent bootstrap` if `~/.local/share/intent/home` does not exist.

**From v3.0.0,** note that the v3.0.0 keg shipped without the rule library, the skills and `intent_claude_cwi`. The [v3.0.1 release notes](releases/3.0.1/RELEASE_NOTES.md) cover that migration.

## Known defects

[Known defects](known-defects.md) lists every defect in the current release that a reader can reach by following the documentation correctly.

---

Next: [Getting started](getting-started.md).
