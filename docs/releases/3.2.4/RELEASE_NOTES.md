# Intent v3.2.4

**v3.2.4 lets a project keep its agent contracts out of sight, and makes Claude Code read them.** A project can now keep `AGENTS.md` and `CLAUDE.md` in `.claude/` instead of at its root, for a tree whose root people read rather than tools. Every generated `CLAUDE.md` now imports `AGENTS.md`, which Claude Code never loaded beside a `CLAUDE.md` before. `intent deinit` strips what Intent wrote from a project being closed, and a store under a Dropbox folder is kept out of the sync. Intent also installs on Linux through Homebrew, and this release fixes the search index doubling that could grow a store by gigabytes in a day.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** Every claim below was checked against the tree the release is cut from, by reading the code that produces the behaviour and the test that holds it.

**The section headings follow the CHANGELOG's 3.2.4 section**; this page carries every entry, grouping related ones, with what a reader upgrading needs. [Working with agents](../../working-with-agents.md) is the full description of the two layouts and `deinit`.

## Added

**A project chooses where its two agent contracts live** (ST0083). `contracts: root`, the default and what an absent key means, keeps `AGENTS.md` and `CLAUDE.md` at the root. `contracts: dotclaude` keeps both in `.claude/`, which Claude Code reads natively. `intent init --contracts dotclaude` sets it at birth, and `intent agents layout <root|dotclaude>` moves an existing project and writes the key, refusing before anything moves when the destination holds a different file. No existing config changes.

**`intent deinit` strips what Intent wrote from a project being closed** (ST0083). It is a dry run unless `--apply` is given. It removes only what it can prove Intent wrote: the generated contracts, seeds still byte-identical to their seed, and Intent's blocks in the git hooks. From `.claude/settings.json` it removes Intent's hook entries and its two deny rules and keeps the rest, and from `.mcp.json` the `intent` server. Anything else is kept and named. `--all` also removes `intent/` with the store and the project's registry row, and refuses while intentd watches the tree.

**A store under a Dropbox folder is marked ignored for the Dropbox client** (ST0083). The directory `intent/.cache` is marked when the store is created in it, by `intent init` or by the first command in a fresh clone, and the command says so once. A store that already exists under Dropbox is not marked by the upgrade: `intent doctor` names it, with the command that marks it.

**`intent doctor` reports four more things about a project's agent wiring** (ST0083). It counts a generated contract that sits in both layouts. It shows, without counting them, a `.claude/settings.json` Intent does not manage whose hooks name a missing path, an empty or `Author Unknown` author, and a v2 hook script in `.claude/scripts/` that nothing runs.

**Intent installs on Linux through Homebrew** (ST0081). `brew install matthewsinclair/intent/intent` installs the release's Linux package for x86_64 or aarch64. `docs/install.md#linux` has the steps.

**`intent issues --project <project>` files and reads issues in another registered project** (issue 0590). The project is named from the machine's registry by its root, `project_id`, directory name or `project_name`, and the flag goes before the verb.

## Fixed

**Claude Code now loads a project's `AGENTS.md`** (ST0083). It reads `AGENTS.md` only where there is no `CLAUDE.md`, so in every project Intent set up it never did. A generated `CLAUDE.md` now opens with `@AGENTS.md`, and stops repeating the four-principle index that `AGENTS.md` carries (issue 0598).

**The templated `.claude/settings.json` denies editing the generated `AGENTS.md`**, in both layouts (ST0083), and nothing against `CLAUDE.md`.

**`intent claude upgrade --apply` keeps what a project added to its `.claude/settings.json`** (issue 0606). A settings file carrying Intent's hooks was overwritten whole, so a project's own permissions, model pin or hooks beside Intent's were lost at each upgrade. Intent's entries are now brought to the template's and every other key is kept; a file that does not parse is held and named.

**A scoped `intent sync --to-store <ID>` no longer doubles the search index, and a doubled store repairs itself** (issue 0605). Every v3.1 and v3.2 release before this one carries the defect. The store moves to schema 31 on its first open under v3.2.4, which keeps one row per section and rebuilds the full-text index.

**`intent claude upgrade --apply` installs the skill that releases the prompt gate** (issue 0601), so a fresh machine's first Claude Code session is no longer locked out.

**Skills install where Claude Code loads them when `CLAUDE_CONFIG_DIR` is set** (issue 0604), and the prompt gate and the session-start line look there too.

**`intent claude skills uninstall` removes the skill's emptied directory** (issue 0602), and keeps and names a file in it that Intent did not write.

**`intent --version` and other read-only invocations no longer move a legacy `~/.intent/`** (issue 0599); the first verb that does work moves it and says so.

**Two v2 conversions finish clean** (issues 0596 and 0597): a first conversion no longer prints a spurious "not examined" line, and an On Hold thread no longer leaves its v2 `info.md` behind as view skew.

**A cover whose Objective or Context holds a `## ` line of its own is no longer refused as a hand edit** (issue 0588).

**`intent daemon start --at-login` and `stop --at-login` refuse off macOS** before writing anything (ST0081).

**The skills `/in-session` loads are smaller and are not loaded twice after a compact** (issues 0591, 0592 and 0593). `in-whiteboard` fits a compact whole, and `/in-session` invokes only what the compact did not carry.

**The mechanical trope pass sees curly quotes and apostrophes** (issue 0589), and **the docs say each backup snapshot is a full copy of the store** (issue 0587).

**The release's Homebrew smoke passes on the prompt gate's refusal** (issue 0600), as it does on a fresh machine.

## Upgrading

```
  $ intent daemon stop             # or brew services stop intent, if it runs there
  $ brew upgrade matthewsinclair/intent/intent
  $ intent --version
  $ intent daemon start            # or brew services start intent, if it ran there
```

**Every store migrates, from schema 30 to 31, on its first open** under v3.2.4, and v3.2.3 then refuses it as written by a newer Intent. Upgrade every machine that opens a project before going back to work in it.

**Run `intent claude upgrade --apply` in each project.** The `CLAUDE.md` and `AGENTS.md` templates changed and the settings template carries the deny rules. The hook scripts, which also changed, are served from the install and need no project step. A `CLAUDE.md` keeps what you wrote between its `user` markers; a `.claude/settings.json` that is not Intent's is held unless `--force`, and one that is keeps every key you added (issue 0606).

**Run `intent claude skills sync`** to take the changed `in-session`, `in-essentials`, `in-standards`, `in-whiteboard` and `in-detrope` skills.

**Restart intentd, and running Claude Code sessions, to give them the new build.** A daemon left running goes on with the old binary, and each session's `intent mcp` server keeps the binary it started with.
