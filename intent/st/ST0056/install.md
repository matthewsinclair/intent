# Install and upgrade - ST0056: v3 (WP-11 deliverable)

**Status: published.** The tap `matthewsinclair/homebrew-intent` carries the formula for the current release; `brew info matthewsinclair/intent/intent` names the version it serves. Each release is cut with `int build release`, and `int macos prepare`, `formula`, `publish` and `smoke --reinstall` stage, publish and then reinstall it cold from the tap.

This document is about **getting the binary onto a machine and what that does to an install already there**. It is not the migration spec. The v2 -> v3 data migration -- preconditions, the flow, the carry policy, what the migrator refuses -- is `migration.md`, and is deliberately not restated here.

## Install

```
brew install matthewsinclair/intent/intent
```

That is the whole install story for v3, and it replaces v2's clone-and-symlink model. The formula is macOS arm64 only (`depends_on arch: :arm64`, `depends_on :macos`). It installs `intent` (the CLI) and `intentd` (one daemon per machine) into the keg's `libexec/bin` with symlinks in `bin`, and unpacks the support tree beside them in `libexec`.

**The fully-qualified form is deliberate and must not be shortened.** homebrew-core has no `intent` formula (`https://formulae.brew.sh/api/formula/intent.json` answers 404), so `brew install intent` resolves to our tap only while core stays empty of that name -- and the day core gains one, **every published instruction becomes ambiguous at best and installs somebody else's software at worst**, in an instruction users have already copied into their own documentation. The long form cannot be shadowed and costs one line of typing.

### Then set the machine up: `intent bootstrap`

```
intent bootstrap
```

**The formula has no post-install step, so this is yours to run.** `intent bootstrap` records the install root in `~/.intent/home` and the operator's author identity in `~/.intent/config.json`. The recorded root is the keg's own versioned path -- driven, `created: install root recorded -- /opt/homebrew/Cellar/intent/3.0.1/libexec` -- so run it again after every `brew upgrade`.

**Every project's pre-commit gate depends on it.** The gate each project carries is a shim (`.git/hooks/pre-commit.intent`) that reads `~/.intent/home` and execs the gate that install ships. With the file absent, empty or naming a removed keg, the shim refuses every commit in that project. `intent claude upgrade --apply` checks the pointer after installing the shim and says so, with `remedy: intent bootstrap`.

### The daemon and the menubar app

`intentd` is started with `intent daemon start` (add `--at-login` to enrol it with launchd) and managed with `intent daemon stop|restart|status`. The formula also declares a `brew services` block that runs `intentd` directly.

**The menubar app is not installed by the formula.** Each release carries it as a separate asset, `Intent.app.zip`. `intent app start|stop|restart|status` looks for it at `/Applications/Intent.app`, then in a local build output.

## THE ONE THING TO READ BEFORE INSTALLING: brew SHADOWS a v2 install, it does not replace it

**This is the sentence this document exists for.** A v2 install is a clone of the repository with `bin/intent` symlinked into `~/.local/bin` or `~/bin`. Homebrew installs into its own prefix, and `brew shellenv` puts that prefix at the **front** of `PATH`. Whichever `intent` comes first on `PATH` answers. brew reports the ordering itself: when an earlier `PATH` entry shadows the keg, its caveats list `intent (shadowed by ...)`.

So one `brew install` does not upgrade anything and does not ask. **It puts a new binary in front of the old one, and every `intent` command in every one of your v2 projects starts answering from v3** -- with the v2 install still sitting there, still correct, and now unreachable. No upgrade was requested and nothing was removed.

**What you will see first.** v3 detects an unmigrated v2 project and refuses at exit 1:

```
error: this project has not been migrated to Intent v3 -- it declares Intent 2.19.0
  remedy: run `intent upgrade` to migrate this project to Intent v3
```

That is correct behaviour and the point at which to run the migration below -- but it will happen in projects you were not thinking about, the first time you type `intent` in one of them.

### How the binary finds its support tree

`intent claude hook <name>` does not reimplement the hooks -- it **execs `lib/templates/.claude/scripts/<name>.sh` out of the install root**, so the script's exit code is the one Claude Code sees. The binary resolves that root by walking up from its symlink-resolved `current_exe()` to the first directory containing `lib/templates/` (`install.rs:129`), and `intent info` prints it as `INTENT_HOME:`. **There is no `INTENT_HOME` fallback and that is deliberate** (AC-11.3, and stronger than the AC asks): the environment is not read at all, because a stale v2 export would otherwise make a v3 binary exec v2's hook scripts with nothing reporting the mismatch. When the root cannot be resolved, `intent info` exits 1 and `intent claude hook` fails at exit 1 naming the missing install.

**So the support tree has to ship beside the binary.** A release carries these assets: `intent-aarch64-apple-darwin`, `intentd-aarch64-apple-darwin`, `intent-support.tar.gz` and `Intent.app.zip`. The support archive is built from `SUPPORT_PATHS` in `bin/.devbin/cmd/macos` and carries `lib/templates`, the rule library (`intent/plugins/claude/rules`), the skills (`intent/plugins/claude/skills`) and `intent/plugins/claude/bin/intent_claude_cwi`. It does not carry `intent/plugins/claude/subagents`, so a brew install answers `intent claude subagents list` with `no subagents in this install`.

- **The archive is rooted at the INSTALL ROOT**, not at the templates directory, so the formula's install line is "put everything in this archive into `libexec`". A new shipped path is a content change rather than a formula change.
- **`libexec`, not `prefix/lib`, and not on taste.** Both layouts resolve. `lib` is a brew-LINKED directory, so `prefix/lib/templates` would publish a directory called `templates` into the shared prefix under about as generic a name as exists. `libexec` is not linked. The binary must still sit beside the marker, so `bin` gets a symlink; `bats-core` ships this exact shape.
- **"Staged artefact" and "must be proven signed and notarised" are different sets**, because the tarball must be hashed and cannot be notarised. `checksum` CLASSIFIES every staged file and REFUSES an unclassified one, so the next artefact gets a decision or gets refused -- it can neither ship as unproven bytes under a published hash, nor be silently omitted from `SHA256SUMS.txt`. `publish` uploads and round-trips the same derived set, so what ships and what has a published hash cannot drift.

### Exit codes belong to the caller's contract

Intent's canon `.claude/settings.json` wires Claude Code's `SessionStart`, `UserPromptSubmit` and `Stop` hooks to `intent claude hook session-context`, `require-in-session` and `session-finish` -- an unqualified `intent`, resolved from `PATH`. Before the `claude` family was implemented, the v3 binary answered it with exit 2, which is `UserPromptSubmit`'s BLOCK code: once brew put v3 in front, every prompt in every Intent project with the canon hooks was refused, and the remedy (`/in-session`) was itself a prompt. **The trigger was `brew install`, not migration** (issue 0043, closed when `info` and the `claude hook` family were implemented).

**Three things from the rig that confirmed it outlive the fix:**

- **A blocked prompt exits the `claude` process with 0.** The block is in-band, in the output stream, so any wrapper checking the process exit code sees success while the model never saw the prompt. **A test of this class must assert on OUTPUT, never on exit code.**
- **`Stop` at exit 2 means "do not stop"** -- measured as a hang with no output. `Stop` is now routed through `intent claude hook session-finish`, and `session-finish.sh` exits 0 on every path by stated contract, so the routing did not arm that failure.
- **Exit 1 passes a `UserPromptSubmit` prompt through**, so `intent claude hook` failing to find its install does not lock anyone out.

**The general shape: `2` carries four meanings across four contracts that all read the same number** -- fail-open in the pre-commit gate, BLOCK in `UserPromptSubmit`, advisory in `SessionStart`, and refuse-to-stop in `Stop`. **An exit code is a property of the CALLER's contract, not of the tool.**

### The good consequence of shadowing, which is why this is a hazard and not a defect

Because nothing was removed, **nothing was lost**. The v2 install is intact further down the `PATH`. That makes the recovery below a `PATH` edit rather than a reinstall, and it is the reason shadowing is the safer of the two behaviours even though it is the more surprising one.

## Upgrading a project from v2

The migrator is the v3 binary detecting a v2 project:

```
intent upgrade
```

**Floored at v2.19.0.** Below that floor it refuses by name at exit 1 and prints a two-hop instruction:

```
error: this project declares Intent 2.18.0 and is below the migration floor, so it cannot be converted directly
  remedy: this project is below the v2.19.0 migration floor -- run `install intent@2 && intent upgrade` first, then migrate it with v3
```

The v2 ledger is never reimplemented in Rust. The tap carries no `intent@2` formula, so the first hop is run with a v2 clone checked out at the `v2.19.0` tag.

**It refuses over a dirty git tree**, naming each uncommitted path, because a migration commit assembled over someone's work could not be reverted without taking that work too.

**It writes the migration and does not commit it.** Its closing line is `ok: this project is now Intent v3.0.1 -- commit the canon and the generated views`; making that one commit, containing only the migration, is the operator's step, and it is the commit a rollback reverts.

Everything else about what converts, what is carried, what is refused, and what the residue report contains: **`migration.md`**. That is the single source; this section exists only to say which verb to type and what will stop you.

## Running v2 and v3 side by side, or going back

Because the v2 install was shadowed rather than replaced, all three of these work without reinstalling anything:

- **Reach v2 explicitly** -- call it by path, eg `~/.local/bin/intent`.
- **Put v2 back in front** -- move the v2 symlink's directory ahead of the Homebrew prefix in your `PATH`. The shadowing reverses the moment the order does.
- **Remove v3** -- `brew uninstall intent`. The v2 symlinks resume answering immediately, because they were never touched.

**A v2 clone stays v2 only while its checkout does.** `bin/intent` is deleted on `main`, so pulling `main` into a v2 clone removes the binary its symlinks point at.

**A migrated project does not un-migrate**, and that asymmetry is the thing to be careful about. Reverting the binary does not revert a project the migrator has already converted; `migration.md` owns what that conversion did and whether it is reversible. Reordering `PATH` is safe. Reordering `PATH` _after_ migrating a project is a different question, and the answer is in the migration spec.

## Uninstall

```
intent daemon stop --at-login
brew uninstall intent
brew untap matthewsinclair/intent
```

The first line matters only if the daemon was enrolled with `intent daemon start --at-login`; it unenrols the LaunchAgent, which brew does not know about. None of the three touches a v2 clone, a v2 symlink, `~/.intent/`, or any project's `intent/` directory.

## What is checked, and what is not

**Checked at publish, per AC-11.4:** the published checksum matches the bytes a user downloads. `int macos publish` uploads, then **re-downloads from the public URL the formula sends brew to and hashes THAT**, and only ships the formula if they agree. On a mismatch the release stays and no formula ships -- a release nothing points at is inert, whereas a formula naming unconfirmed bytes is an installer. The failure this closes never fails for us and fails for every `brew install`, which is where we have the least visibility.

**Checked at build:** the binaries are signed with the Geodica Developer ID and notarised. `codesign --verify --strict` returns 0 on an ad-hoc signature, so it is not the test; the check that means anything is `spctl -a -t open --context context:primary-signature` on a quarantined copy.

**Checked after publish:** `int macos smoke --reinstall` uninstalls, untaps, clears brew's download cache, and runs a real `brew install matthewsinclair/intent/intent` from the network before exercising the installed keg -- the network hop being the one a locally preseeded install cannot see.
