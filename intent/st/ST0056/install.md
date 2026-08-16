# Install and upgrade - ST0056: v3 (WP-11 deliverable)

**NOTHING BELOW WORKS YET, AND THAT IS DELIBERATE.** The tap `matthewsinclair/homebrew-intent` is live and **carries no formula**, because a formula pointing at a release that does not exist reads as "the tap is broken" rather than "the release is not out yet". These are the instructions the first `int macos publish` makes true; they become user-facing documentation at the WP-12 cutover, not before. Read the tense as future.

This document is about **getting the binary onto a machine and what that does to an install already there**. It is not the migration spec. The v2 -> v3 data migration -- preconditions, the flow, the carry policy, what the migrator refuses -- is `migration.md`, and is deliberately not restated here.

## Install

```
brew install matthewsinclair/intent/intent
```

That is the whole install story for v3, and it replaces v2's clone-and-symlink model. Two binaries land: `intent` (the CLI) and `intentd` (one daemon per machine).

**The fully-qualified form is deliberate and must not be shortened.** Measured 2026-08-15: neither `intent` nor `intentd` exists in homebrew-core, so `brew install intent` would resolve to our tap today. It works only while core stays empty of that name -- and the day core gains one, **every published instruction becomes ambiguous at best and installs somebody else's software at worst**, in an instruction users have already copied into their own documentation. The long form cannot be shadowed and costs one line of typing.

## THE ONE THING TO READ BEFORE INSTALLING: brew SHADOWS a v2 install, it does not replace it

**This is the sentence this document exists for.** v2 installs by cloning the repository and symlinking `bin/intent` into `~/.local/bin` and `~/bin`. Homebrew installs into its own prefix, which `brew shellenv` puts at the **front** of `PATH`. Measured on the primary dev machine, 2026-08-16:

| PATH position | what is there                                     |
| ------------- | ------------------------------------------------- |
| 1             | the Homebrew prefix -- where `brew install` lands |
| 17            | `~/.local/bin/intent` -> the v2 clone             |
| 19            | `~/bin/intent` -> the v2 clone                    |

So one `brew install` does not upgrade anything and does not ask. **It puts a new binary in front of the old one, and every `intent` command in every one of your v2 projects starts answering from v3** -- with the v2 install still sitting there, still correct, and now unreachable. No upgrade was requested and nothing was removed.

**What you will see first.** v3 detects an unmigrated v2 project, refuses, and exits non-zero. That is correct behaviour and the point at which to run the migration below -- but it will happen in projects you were not thinking about, the first time you type `intent` in one of them.

**Known sharp edge at cutover (issue 0036):** the refusal's remedy names `intent upgrade`, and until WP-10 lands, the v3 binary has no `upgrade` subcommand -- so following it verbatim gives `error: unrecognized subcommand 'upgrade'`. The remedy is right about the end state and unreachable today. **Do not publish before that resolves**, because the string is unreachable precisely for the user it was written for: the binary printing it is the one that now answers to `intent`.

### The good consequence of shadowing, which is why this is a hazard and not a defect

Because nothing was removed, **nothing was lost**. The v2 install is intact two positions down the `PATH`. That makes the recovery below a `PATH` edit rather than a reinstall, and it is the reason shadowing is the safer of the two behaviours even though it is the more surprising one.

## Upgrading a project from v2

The migrator is the v3 binary detecting a v2 project:

```
intent upgrade
```

**Floored at v2.19.0.** Below that floor it refuses by name and prints a two-hop instruction (`install intent@2 && intent upgrade`, then retry). The v2 ledger is never reimplemented in Rust.

It refuses over a dirty git tree. The migration is one visible commit, and a half-done abort over dirt is worse than an early refusal.

Everything else about what converts, what is carried, what is refused, and what the residue report contains: **`migration.md`**. That is the single source; this section exists only to say which verb to type and what will stop you.

## Running v2 and v3 side by side, or going back

Because the v2 install was shadowed rather than replaced, all three of these work without reinstalling anything:

- **Reach v2 explicitly** -- call it by path: `~/.local/bin/intent`. It is unchanged.
- **Put v2 back in front** -- move `~/.local/bin` ahead of the Homebrew prefix in your `PATH`. The shadowing reverses the moment the order does.
- **Remove v3** -- `brew uninstall intent`. The v2 symlinks resume answering immediately, because they were never touched.

**A migrated project does not un-migrate**, and that asymmetry is the thing to be careful about. Reverting the binary does not revert a project the migrator has already converted; `migration.md` owns what that conversion did and whether it is reversible. Reordering `PATH` is safe. Reordering `PATH` _after_ migrating a project is a different question, and the answer is in the migration spec.

## Uninstall

```
brew uninstall intent
brew untap matthewsinclair/intent
```

Neither touches a v2 clone, a v2 symlink, or any project's `intent/` directory.

## What is checked, and what is not

**Checked at publish, per AC-11.4:** the published checksum matches the bytes a user downloads. `int macos publish` uploads, then **re-downloads from the public URL the formula sends brew to and hashes THAT**, and only ships the formula if they agree. On a mismatch the release stays and no formula ships -- a release nothing points at is inert, whereas a formula naming unconfirmed bytes is an installer. The failure this closes never fails for us and fails for every `brew install`, which is where we have the least visibility.

**Checked at build:** the binaries are signed with the Geodica Developer ID and notarised. `codesign --verify --strict` returns 0 on an ad-hoc signature, so it is not the test; the check that means anything is `spctl -a -t open --context context:primary-signature` on a quarantined copy.

**Not checked, and stated rather than implied:** no release has ever carried an asset, so the upload path cannot be rehearsed without publishing. `gh release create` is the one call in `int macos publish` that has never run. Everything downstream of it -- the re-download, the hash, the formula render, the tap clone, the commit -- has been exercised against the real live tap with nothing pushed.
