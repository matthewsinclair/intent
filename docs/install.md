# Installation

Intent is a single binary plus a support tree. The support tree is not optional — several commands resolve files out of it at runtime, and a install missing it will fail on those commands rather than degrade.

## Homebrew (macOS)

```
  $ brew install matthewsinclair/intent/intent
  $ intent --version
```

This is the supported path. The formula installs the binary, the daemon, and the full support tree into the keg, and points `INTENT_HOME` at it.

## From source

Requires a recent stable Rust toolchain.

```
  $ git clone https://github.com/matthewsinclair/intent
  $ cd intent/native/rust
  $ cargo build --release
```

The binaries land in `native/rust/target/release/`. **Building is not the whole install** — the binary resolves its templates, rule library and skills against `INTENT_HOME`, so a source build needs that set to the repository root:

```
  $ export INTENT_HOME=/path/to/intent
  $ export PATH="$INTENT_HOME/native/rust/target/release:$PATH"
```

## Verifying an install

```
  $ intent --version
  $ intent doctor
```

`intent doctor` checks the things that are wrong most often — whether `INTENT_HOME` resolves, whether the support tree is present and complete, and whether the project you are standing in is one this binary can read.

## What the install has to contain

Intent resolves five paths against its install root. **A binary on its own is not an installation**, and the failure mode is per-command rather than global, which is why it is worth knowing what is meant to be there.

| Path under the install root                   | Resolver         | What stops working without it                                                      |
| --------------------------------------------- | ---------------- | ---------------------------------------------------------------------------------- |
| `lib/templates/`                              | `canon.rs:284`   | `intent init`, `intent upgrade`, every generated hook body, both whiteboard guards |
| `intent/plugins/claude/rules/`                | `rules.rs:149`   | `intent claude rules list` / `show`, `intent critic <lang>`                        |
| `intent/plugins/claude/skills/`               | `skills.rs:419`  | `intent claude skills install` / `sync` / `uninstall`                              |
| `intent/plugins/`                             | `plugins.rs:108` | Parent of the two above; ships via its children                                    |
| `intent/plugins/claude/bin/intent_claude_cwi` | `install.rs:361` | `intent claude start`, `intent claude ws new` / `list` / `archive` / `hygiene`     |

On a Homebrew install these root at `$KEG/libexec`, which is what `INTENT_HOME` resolves to. The staged archive strips its own wrapper directory, so the tree roots directly on the install root:

```
  $KEG/bin/intent, intentd                          symlinks into libexec
  $KEG/libexec/bin/intent, intentd
  $KEG/libexec/lib/templates/
  $KEG/libexec/intent/plugins/claude/rules/
  $KEG/libexec/intent/plugins/claude/skills/
  $KEG/libexec/intent/plugins/claude/bin/intent_claude_cwi
```

If a command fails with an error naming a path rather than an argument, this table is where to look first. `intent doctor` reports the same thing without you having to.

## Known defects in the published v3.0.0 build

**These are real and measured, and they are stated here rather than in a changelog because they affect anyone installing today.**

**The v3.0.0 keg ships no rule library and no skills.** The copy list that builds the release archive omitted both trees. On a Homebrew install of v3.0.0, `intent claude rules list` and the whole `intent claude skills` family fail. A source install with `INTENT_HOME` set to the repository is unaffected, because the trees are there. Fixed for the next release.

**`intent ac new` on an id that already exists replaces the row rather than refusing.** The replacement is a full write, so a field you do not supply is not preserved — it is written empty. In v3.0.0 there is no verb that edits a criterion in place, so the verb that repairs is the verb that destroys. The same shape applies to `at new`.

**v3.0.1 closes this and the remedy is to upgrade.** `ac new` now refuses a taken id and names `ac edit` in the refusal, and `ac edit` and `at edit` exist to do the thing you meant. **If you are on v3.0.0 and cannot upgrade yet, treat `ac new` as create-only and read `ac list` before re-running it on an id you are unsure about.**

**Criteria authored as unsatisfied-with-evidence lost their evidence when a v2 project was ingested.** In v3.0.0 the unsatisfied state carries no payload, so a v2 row reading `satisfied: no` _with_ an evidence clause had nowhere to put the clause and it was dropped without a warning. [Migrating from v2](migrating-from-v2.md) covers how to tell whether your project is affected and what recovery looks like.

---

Next: [Getting started](getting-started.md).
