# Install and upgrade - ST0056: v3 (WP-11 deliverable)

**NOTHING BELOW WORKS YET, AND THAT IS DELIBERATE.** The tap `matthewsinclair/homebrew-intent` is live and **carries no formula**, because a formula pointing at a release that does not exist reads as "the tap is broken" rather than "the release is not out yet". These are the instructions the first `int macos publish` makes true; they become user-facing documentation at the WP-12 cutover, not before. Read the tense as future.

**ONE hard hold stands between here and publication: issue 0036** -- the refusal's remedy names a subcommand v3 does not have; verified still live at HEAD `304cd104`, `intent upgrade` exits 2. Two others were live and are now RELEASED: **issue 0043** (the prompt lockout) closed by `c6aee944`, and **the packaging hold** -- the formula installed the binaries and nothing else, so a brew-installed `intent` could not find its own install tree -- closed by `7a41ff2e` and recorded below with what it did and did not prove. A further gate, the `## [3.0.0]` CHANGELOG section, belongs to the cut rather than to this document, as does hv's call on the upstream freeze.

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

### RELEASED -- the formula shipped `bin/` and nothing else, and the binary needs `lib/templates/` beside it

**CLOSED BY `7a41ff2e`. Kept in full, because the instance is closed and the class is not**, and because the measurement below is the only record of what a published build would have done.

**This is the one that would have broken on the first published build, and it would have looked like a hook bug rather than a packaging one.** Raised by cc from the implementation side, measured here from the packaging side.

`intent claude hook <name>` does not reimplement the hooks -- it **execs `lib/templates/.claude/scripts/<name>.sh` out of the install root**, and `intent info` prints that same root for the pre-commit gate to parse back. The binary resolves the root by walking up from its symlink-resolved `current_exe()` to the directory containing `lib/templates/`. **There is no `INTENT_HOME` fallback and that is deliberate** (AC-11.3, and stronger than the AC asks): the environment read was removed rather than demoted, because a stale v2 export would otherwise make a v3 binary exec v2's hook scripts with nothing reporting the mismatch.

**The formula installs two files.** `bin.install` for `intent` and `intentd`; `lib/templates/` is staged nowhere, and no release asset has ever carried it. So the walk terminates without finding a marker.

**Measured against a reproduction of exactly what the formula produces** -- the binary under `Cellar/intent/3.0.0/bin/` with a `bin/intent` symlink in front of it, invoked from a neutral working directory:

| invocation                              | exit  | result                                                                      |
| --------------------------------------- | ----- | --------------------------------------------------------------------------- |
| `intent info`                           | **0** | `INTENT_HOME: <not set>`, with `cannot locate the Intent install` on stderr |
| `intent claude hook require-in-session` | **1** | `cannot locate the Intent install this binary belongs to`                   |

**Both failed QUIETLY, and the exit codes were why.** `claude hook` returning 1 does not block a prompt -- vc's rig established that exit 1 passes through -- so all three Claude Code hooks would have silently stopped working rather than announcing themselves. And `intent info` **exited 0 while printing an error**, so the pre-commit gate's status check passed; only the empty `INTENT_HOME` parse caught it, which is the fail-open path from issue 0042.

**One half of that is now fixed at the source, by cc, in `501f5083`: `intent info` exits 1 when it cannot resolve the install.** So a packaged install that cannot find its tree is now LOUD on the gate side -- my 0042 fix captures the status and names every skipped guard -- rather than silent. The `claude hook` side is unchanged and still passes through at exit 1. Recorded because the row above is a measurement of a commit that has moved: **the table describes `304cd104`, not HEAD.**

**This was 0042 and 0043's shape reached by a third route.** Those were commands that did not exist; this was the same commands unable to find what they need.

#### The fix, and what it proves

`7a41ff2e`. A release now ships **three** artefacts: the two binaries plus `intent-support.tar.gz`.

- **The archive is rooted at the INSTALL ROOT**, not at the templates directory, so the formula's install line is "put everything in this archive into `libexec`". `intent critic` and `intent claude rules` are unimplemented today (both exit 2) and will need the rule library, which lives **outside** the marker directory at `intent/plugins/claude/rules/`. Rooting the archive at the install root makes that a content change rather than a formula change and rather than a fourth asset.
- **`libexec`, not `prefix/lib`, and not on taste.** Both layouts resolve. `lib` is a brew-LINKED directory -- measured, 858 keg symlinks in the shared prefix -- so `prefix/lib/templates` would publish a directory called `templates` into a global namespace under about as generic a name as exists. `libexec` is not linked. The binary must still sit beside the marker, so `bin` gets a symlink; `bats-core` ships this exact shape.
- **Signing, notarisation and `verify` are untouched.** `notarize` already submitted a directory zip rather than a bare binary, so a third non-Mach-O artefact never reaches them.
- **The identity that broke is the one worth naming: "staged artefact" and "must be proven signed and notarised" stopped being the same set**, because the tarball must be hashed and cannot be notarised. `checksum` now CLASSIFIES every staged file and REFUSES an unclassified one, so the next artefact gets a decision or gets refused -- it can neither ship as unproven bytes under a published hash, nor be silently omitted from `SHA256SUMS.txt` the way the old `*-$triple` glob would have omitted it. `publish` uploads and round-trips the same derived set, so what ships and what has a published hash cannot drift.

**What is proven:** the real staged artefacts, the formula's install block replayed line for line including **both** symlink hops, all three consumers working, both whiteboard guards resolving, and the clock guard executing from the installed tree. The packaged scripts are byte-identical to the repo's, so 0042's enforcement canary carries over. The formula lints clean at a tap path with **no** offences. Every guard is mutation-tested.

**What is NOT proven, and it is the standing one:** no release here has ever carried an asset, so `gh release create` and a real `brew install` remain unexercised. **Replaying the install block by hand is evidence about the layout, not about Homebrew.** That is the same unexercised surface WP-11 has carried throughout, neither widened nor narrowed by this change.

### RELEASED -- issue 0043: shadowing locked the user out of Claude Code, in every Intent project on the machine

**CLOSED BY `c6aee944` (cc), which implemented `info` and the `claude hook` family. Re-measured at HEAD `304cd104` on a binary built from that tree**, because the previous build was three sources stale and a positive result from a stale instrument is still a result about the past:

| invocation                              | exit | state                                                      |
| --------------------------------------- | ---- | ---------------------------------------------------------- |
| `intent info`                           | 0    | implemented -- prints `INTENT_HOME`                        |
| `intent claude hook session-context`    | 0    | implemented -- `SessionStart` gets its context             |
| `intent claude hook require-in-session` | 0    | implemented -- **the prompt gate passes through**          |
| `intent claude hook post-tool-advisory` | 0    | implemented                                                |
| `intent critic <lang> --staged`         | 2    | still unimplemented; the pre-commit gate fails OPEN (0038) |
| `intent upgrade`                        | 2    | still unimplemented -- **this is 0036, and it holds**      |

**The whiteboard guards were also verified live under the real v3 binary**, which is issue 0042's canary and the thing that could not be asserted until `info` existed: a fixture board carrying a stamp with no trailing `Z`, committed through the shipped hook with v3 resolving `INTENT_HOME`, is **REFUSED**. The guards resolve and enforce.

**The section below is kept rather than deleted. The instance is closed; the class is not** -- and a document that erases a hold once it lifts teaches nobody why it was there.

---

**What it was.** Not a bad first contact -- it took away the tool the user would recover with, and it did not wait for a migration.

Intent's canon `.claude/settings.json` wires Claude Code's `UserPromptSubmit` hook to `intent claude hook require-in-session` -- an unqualified `intent`, resolved from `PATH`. The gate's own contract is `exit 0` to pass the prompt through and **`exit 2` to BLOCK it**. Since `d2b8e76d`, the v3 binary answers every not-yet-implemented command with exit 2, which is correct for the consumer that fix was written for: the pre-commit gate reads 2 as fail-open. **Two shipped consumers read the same number as opposite instructions**, and `brew install` puts the binary that returns it in front of the one that does not.

The result is that the gate's pass-through path becomes unreachable. Every prompt in every Intent project with the canon hooks is refused, and the refusal is self-sealing: the remedy is `/in-session`, which is a prompt.

**Measured here, not inferred** -- binary confirmed newer than `d2b8e76d` before it was trusted:

| where                      | `intent claude hook require-in-session`                                   |
| -------------------------- | ------------------------------------------------------------------------- |
| an unmigrated v2 project   | `exit 2` -- `` `claude` is a known command that is not implemented yet `` |
| outside any project at all | `exit 2` -- same message                                                  |

**The trigger is `brew install`, NOT migration.** `claude` is unimplemented as a family, so v3 refuses before it ever looks at the project -- which means the blast radius is every Intent project on the machine carrying the canon hooks, migrated or not, plus none-of-the-above. Issue 0043 as filed says "do not migrate this repo until it is settled"; migration is not the condition. **Publication is.** This is 0036's chain with the consequence changed: shadowing is machine-wide and unrequested, so a user meets this in a project they were not thinking about, and unlike the `intent upgrade` dead end there is no message to read past.

**It was confirmed live before it was fixed** -- vc, five arms against Claude Code 2.1.233, each a throwaway directory wiring `UserPromptSubmit` exactly as the canon does, driven headless. Exit 0 ran the prompt; **exit 1 also ran it**; exit 2 blocked; the real v3 build blocked carrying its own not-implemented text; and `/in-session`, the documented remedy, blocked too. The exit-1 arm is the one that made it a causal finding rather than a symptom: it is what establishes that moving unimplemented commands from 1 to 2 is what created this, rather than merely correlating with it.

**Two things from that rig that outlive the fix, and belong here because whoever tidies the hook wiring will meet them:**

- **A blocked prompt exits the `claude` process with 0.** The block is in-band, in the output stream. So any wrapper or automation checking the process exit code sees success while the model never saw the prompt -- **a silent-failure surface sitting in exactly the layer you would use to detect the first one.** A test of this class must assert on OUTPUT, never on exit code.
- **`Stop` at exit 2 means "do not stop".** Measured at 24 seconds and zero output, against 3 seconds and a normal answer at exit 0. Intent's `Stop` hook is a bare `echo` today and therefore cannot reach it -- **it is safe by accident of its wiring, not by design.** Routing `Stop` through `intent claude hook` for consistency is the most natural tidying move there is, and it would arm a third distinct failure from the same constant. **Do not do it without deciding what `Stop` should return.**

**The general shape, which is why the class stays open:** `2` now carries four meanings across four contracts that all read the same number -- fail-open in the pre-commit gate, BLOCK in `UserPromptSubmit`, advisory in `SessionStart`, and refuse-to-stop in `Stop`. Issue 0044 is the mirror of it on the tool's side. **An exit code is a property of the CALLER's contract, not of the tool**, and every fix so far was diagnosed against whichever consumer happened to be in view.

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
