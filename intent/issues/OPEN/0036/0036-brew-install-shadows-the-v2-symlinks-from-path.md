---
id: "0036"
title: brew install shadows the v2 symlinks from PATH position 1, and the v3 refusal it exposes names an upgrade verb the v3 binary does not have
date: 2026-08-15
reporter: matts
status: OPEN
severity: medium
---

# 0036: brew install shadows the v2 symlinks from PATH position 1, and the v3 refusal it exposes names an upgrade verb the v3 binary does not have

## Tags

distribution, migration, WP-11, WP-10

## Summary

WP-11 makes `brew install` the install story. v2 installs by clone-and-symlink into `~/.local/bin` and `~/bin`; brew installs into `/opt/homebrew/bin`, which its own `shellenv` puts at the FRONT of PATH. So installing v3 does not replace the v2 install and does not prompt for one -- **it shadows it.** The user runs one `brew install` and every `intent` in every one of their v2 projects starts answering from a different binary, with no upgrade having been requested and the old install still sitting there, correct and unreachable.

The v3 binary handles that first contact well: it detects the unmigrated project, refuses, exits 1, and gives a remedy. **The remedy is `run 'intent upgrade'`, and the v3 binary has no `upgrade` subcommand** -- so a user who follows it verbatim gets `error: unrecognized subcommand 'upgrade'`. It is unreachable precisely for the user it was written for, because the binary printing it is the one that now answers to `intent`.

**This is a sequencing hazard, not a design defect, and the distinction is the whole point of filing it rather than fixing it.** `migration.md:3` is explicit that the migrator IS the v3 binary's `intent upgrade` detecting a v2 project, so the verb is planned and the remedy text is correct about the end state. It is inert today because nothing is published. It goes live at the first `int macos publish`, which is a WP-11 act, and nobody at cut time will think to check whether a remedy string resolves.

## Reproduction

Measured on this machine, 2026-08-15, against `native/rust/target/release/intent` at `11602d1d`.

```
$ echo "$PATH" | tr ':' '\n' | grep -n -E 'homebrew/bin|\.local/bin|matts/bin'
1:/opt/homebrew/bin
17:/Users/matts/.local/bin
19:/Users/matts/bin

$ readlink ~/.local/bin/intent ~/bin/intent
/Users/matts/Devel/prj/Intent/bin/intent
/Users/matts/Devel/prj/Intent/bin/intent

$ brew --prefix
/opt/homebrew
```

A formula installs to `$(brew --prefix)/bin`, which is sixteen entries ahead of the first v2 symlink. Then, in an unmigrated v2 project:

```
$ native/rust/target/release/intent st list ; echo "exit=$?"
error: this project has not been migrated to Intent v3 -- it declares Intent 2.19.0,
       and 56 steel threads carry v2 canon this binary cannot read (...)
  remedy: run `intent upgrade` to migrate this project to Intent v3
exit=1

$ native/rust/target/release/intent upgrade
error: unrecognized subcommand 'upgrade'

$ native/rust/target/release/intent --help | grep -iE 'upgrade|migrate'
(no output -- neither verb is on the v3 surface)
```

Note the exit code is genuinely 1. An earlier reading of this same refusal recorded `exit=0`; that was an artefact of piping through `head` and reading the pipeline's status rather than the binary's, not a property of the binary.

## Root Cause

Two install models that were never designed against each other, meeting for the first time at WP-11.

- **v2** is a clone the user owns: `~/.local/bin/intent` and `~/bin/intent` both symlink into `bin/intent` in a working tree. Its `intent upgrade` exists and works.
- **v3** is a brew-managed binary in a prefix Homebrew deliberately places first on PATH.

Nothing arbitrates between them. The first-past-the-PATH rule does, silently, and it favours the newer one -- which is the right outcome for someone who wanted to upgrade and an invisible one for someone who did not know they were choosing.

The remedy string is at `project.rs:111` and is written against the designed end state, where `intent upgrade` is the v3 migrator (`migration.md:3`). Nothing is wrong with the string; what is missing is a constraint tying its publication to the verb existing.

## Impact

Bounded today, sharp later.

- **Today: none.** No release carries an asset, the tap is deliberately empty, and `int macos publish` refuses a dev version. The path to this cannot be walked.
- **At the first publish:** any existing v2 user who runs `brew install` gets a working-but-unmigrated estate where the tool refuses in every project and the printed remedy fails with `unrecognized subcommand`. The refusal is at least informative and correctly exit-1, so this degrades to confusion rather than data loss -- and it lands on exactly the users who liked Intent enough to already have it.

## Proposed Fix

Not ruled here; the sequencing is hv's and the layers belong to other nodes.

1. **WP-10 lands `intent upgrade` before anything is published.** Makes the remedy simply true, and is the outcome `migration.md` already describes. Preferred if the ordering suits.
2. **`int macos publish` gains a precondition that every verb its own remedies name is reachable in the binary being published.** Buildable now and it generalises past this one string. **Deliberately NOT built:** it couples WP-11's publish gate to another work package's progress, and forcing that sequencing is a ruling rather than a mechanism.
3. **The remedy is made version-aware** -- tell a user whose `intent` is v3 something a v3 binary can do. Weakest of the three: it treats the symptom and leaves the shadowing itself undescribed.

Whatever is chosen, the install/upgrade docs owed by WP-11 should say plainly that `brew install` shadows rather than replaces a v2 clone install, and what to do about the old symlinks.

## Related

- ST0056 -- WP-11 (distribution; AC-11.1 is the clean-machine install) and WP-10 (migration; owns the `upgrade` verb)
- `intent/st/ST0056/migration.md:3` -- the migrator IS the v3 binary's `intent upgrade`
- `native/rust/crates/intentsvcs/src/project.rs:111` -- the remedy string

## Resolutions

{{TBC}}
