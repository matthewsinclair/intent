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
  $ intent claude rules list
```

**`intent claude rules list` is the install check, and you must read its COUNT rather than its exit code.** It reads the rule library out of the install root, so a healthy install lists rules and an install missing that tree prints `total: 0 rule(s)` — **both at exit 0**. Driven against the published v3.0.0 keg, which is missing the tree: `rc=0`, nothing on stderr. So the check does distinguish the two states, and it does not distinguish them anywhere a script can see; `intent claude skills list` behaves the same way, answering `no skills in this install` at exit 0. If it lists rules, the largest support tree is present and reachable. If it lists none, it is not there — and nothing about the way the command exits will tell you so.

**`intent doctor` is a different tool and it will not tell you this.** It reports on the _project_ you are standing in — backup staleness, a thread whose recorded status disagrees with its own gate, a store that has drifted from committed canon. Useful, and it inspects none of the install. Run it once you have a project; it is not an installation check.

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

If a command fails with an error naming a path rather than an argument, this table is where to look first. **Nothing in the tool audits this table for you** — the closest thing is running one command per row and seeing whether it works.

## Known defects in the published v3.0.0 build

**These are real and measured, and they are stated here rather than in a changelog because they affect anyone installing today.**

**The ones below affect installing. [Known defects](known-defects.md) carries the full set** — every defect in v3.0.0 that a reader can reach by following the documentation correctly, derived from the issue register at the cut.

**The v3.0.0 keg cannot run `intent claude ws` or `intent claude start`.** The support tree omits `intent/plugins/claude/bin/intent_claude_cwi`, which both resolve against, so the whiteboard provisioner is unavailable on a Homebrew install. **The keg contradicts itself on this**: the whiteboard skill it ships says in its own words that scaffolding a node is the job of `intent claude ws new`, and the same keg cannot run that command. Fixed for v3.0.1; a source install is unaffected, because the binary sits inside the repository and walks up to the trees from its own location. **Setting `INTENT_HOME` does nothing here** -- see below.

**The v3.0.0 keg ships no rule library and no skills.** The copy list that builds the release archive omitted both trees. On a Homebrew install of v3.0.0, `intent claude rules list` returns `total: 0 rule(s)` and `intent claude skills list` returns `no skills in this install` — **both at exit 0, so neither reports a failure you can act on.** If you were expecting these to fail and they did not, you are affected rather than exempt; read the count. A source install is unaffected, because the binary sits inside the repository and walks up to the trees from its own location. **`INTENT_HOME` is v2's variable and v3 does not read it**, so setting it will not repair a keg -- driven against a keg-shaped install root, `intent claude rules list` answers `total: 0 rule(s)` with the variable set to a full repository and with it unset alike, while the same command against an install that carries the trees answers with a count. v3 reports its resolved root _as_ `INTENT_HOME` in `intent info`, which is output rather than input, and that is the likeliest source of the belief. **The mechanism that closes this is in the copy list rather than in a promise:** `bin/.devbin/cmd/macos` now names all four support paths in its copy list, and the staging step compares that list against every install-root path the binary actually resolves and **dies before building** if one is not carried. So a keg built after that guard cannot repeat the omission. That does not change what the published v3.0.0 keg contains, and it does not change the exit code above.

**`intent ac new` on an id that already exists replaces the row rather than refusing.** The replacement is a full write, so a field you do not supply is not preserved — it is written empty. In v3.0.0 there is no verb that edits a criterion in place, so the verb that repairs is the verb that destroys. The same shape applies to `at new`.

**v3.0.1 closes this and the remedy is to upgrade.** `ac new` now refuses a taken id and names `ac edit` in the refusal, and `ac edit` and `at edit` exist to do the thing you meant. **If you are on v3.0.0 and cannot upgrade yet, treat `ac new` as create-only and read `ac list` before re-running it on an id you are unsure about.**

**Criteria authored as unsatisfied-with-evidence lost their evidence when a v2 project was ingested.** In v3.0.0 the unsatisfied state carries no payload, so a v2 row reading `satisfied: no` _with_ an evidence clause had nowhere to put the clause and it was dropped without a warning. [Migrating from v2](migrating-from-v2.md) covers how to tell whether your project is affected and what recovery looks like.

---

Next: [Getting started](getting-started.md).
