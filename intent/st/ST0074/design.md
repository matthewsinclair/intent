# Design: where intentd's durable configuration lives

**Status: options for hv to rule. Nothing here is chosen.** hv, 2026-09-13: _"We also need to resolve where intentd's config resides. I am not sure we have a good answer for that right now. And that answer needs to be standards compliant."_ The project registry is the first thing that will live there, so this is ruled before the registry is built.

## What exists today

| Path                           | What it holds                                                                                      | Written by                        | Read by v3 |
| ------------------------------ | -------------------------------------------------------------------------------------------------- | --------------------------------- | ---------- |
| `~/.intent/config.json`        | The operator's v3 configuration: `author`, the explorer's settings                                 | `intent bootstrap`, `/settings`   | yes        |
| `~/.intent/home`               | The install pointer                                                                                | `intent bootstrap`                | yes        |
| `~/.local/share/intent/`       | intentd's runtime state: `intentd.sock`, `intentd.addr`, `intentd.token`, `intentd.lock`, the logs | `intentd`                         | yes        |
| `~/Library/LaunchAgents/`      | The LaunchAgent plist                                                                              | `intent daemon` / the menubar app | yes        |
| `~/.config/intent/config.json` | v2's configuration (`intent_version: 2.0.0`, `backlog_dir`, `editor`)                              | v2 only                           | **no**     |

intentd has no durable configuration: its project registry is in memory and fills on first contact.

Two constraints any answer inherits:

- **v3 never reads or writes a v2 per-user store** (hv adopted, 2026-08-22). `~/.config/intent/config.json` is v2's, so a v3 file there would share a path with a tool that can never be taught about it.
- **The shipped surface reads only the environment variables in `no_intent_home.rs`'s `ALLOWED`**: `COLUMNS`, `EDITOR`, `HOME`, `USER`, `VISUAL`. Honouring `XDG_CONFIG_HOME` or `XDG_STATE_HOME` is a new row there, granted by hv.

## The standards in play

- **XDG Base Directory Specification**: configuration under `$XDG_CONFIG_HOME` (default `~/.config`), data under `$XDG_DATA_HOME` (default `~/.local/share`), state under `$XDG_STATE_HOME` (default `~/.local/state`), sockets and other runtime files under `$XDG_RUNTIME_DIR`. Widely followed by command-line tools on macOS as well as Linux.
- **Apple's file-system guidelines**: per-user application data under `~/Library/Application Support/<bundle id>/`, preferences under `~/Library/Preferences/`, logs under `~/Library/Logs/`, caches under `~/Library/Caches/`. The menubar app is an Apple bundle; the CLI and intentd are not.

## Options

### A. Keep `~/.intent/`, and put the registry beside `config.json`

- The registry is `~/.intent/projects.json` (or a section of `config.json`).
- No new environment variable, no new directory, no migration.
- Not standards-compliant: a dot-directory in `$HOME` is neither XDG nor Apple's layout. It is the status quo hv has asked to replace.

### B. XDG, with a v3-specific directory name

- Configuration, the registry included, under `$XDG_CONFIG_HOME/intent3/` or similar; daemon runtime state moves to `$XDG_STATE_HOME` and the socket to `$XDG_RUNTIME_DIR` where set.
- Standards-compliant on both platforms Intent runs on.
- Needs hv's grant for the `XDG_*` rows in `ALLOWED`.
- The directory name must not be v2's `~/.config/intent/`, or the separate-paths ruling is broken. A different name is a wart an operator has to be told about.
- `~/.intent/` becomes a migration source, then retires.

### C. XDG, reclaiming `~/.config/intent/` from v2

- As B, under the natural name.
- Requires v2's file to be retired first. The ruling that forbids sharing it was made because v2 cannot be taught the branch, so this only works if v2 is gone from every machine that matters, and it is a reversal of an hv ruling rather than a choice within it.

### D. Apple's layout on macOS, XDG elsewhere

- `~/Library/Application Support/com.matthewsinclair.intent/` for configuration and the registry on macOS; XDG paths on Linux.
- The most correct per platform, and the one the menubar app would expect.
- Two layouts to document, test and support; an operator on macOS who looks in `~/.config` finds nothing.

## Questions for hv

1. Which option, or which combination (eg B for the CLI and daemon, with the menubar app reading the same path)?
2. If XDG: grant the `XDG_*` environment rows, and pick the directory name.
3. Does daemon runtime state move too, or only configuration and the registry?
4. Where the current `~/.intent/config.json` goes: migrated by `intent upgrade`, by `intent bootstrap`, or read in place until it is retired.

# Design: the project registry and `intent discover` (WP-03)

Where the file lives is WP-05's ruling; until then it is `<config-home>/projects.json`.

## Schema

```json
{
  "schema": 1,
  "projects": [
    {
      "root": "/Users/matts/Devel/prj/Intent",
      "added_by": "discover",
      "note": "free text"
    }
  ]
}
```

- `root` is the key, stored canonicalised, as intentd's in-memory registry keys projects.
- `added_by` is `explore`, `discover` or `hand`.
- Keys the tool does not know, `note` included, are carried through a rewrite untouched (AC-03.1).
- No status, counts or timestamps: the project's own store answers those.
- A `schema` newer than the tool knows is read and never rewritten.

## Writers

- `intent explore`, inside a project: adds its root if absent, and changes nothing else (AC-03.2).
- `intent discover [fromdir]`: adds each compatible project not already listed (AC-03.3).
- A human, with an editor.
- intentd never writes it.

Every write goes to a temporary file renamed over the original; a writer that finds the file changed since it read it re-reads and re-applies its one change.

## Compatible

A candidate is registered when `Project::discover` finds its config, the config parses, and v3 would not refuse it as an unmigrated project (AC-10.7). Discover names every project it does not register, with the reason: `needs migration`, `config does not parse`, or `already registered`. It migrates and repairs nothing.

## On change (AC-03.4)

intentd watches the file's directory (a rename replaces the file's inode), debounced like the tree watcher, and re-reads the file. That changes the projects it lists, never which stores are open: opening stays first contact. A file that does not parse is logged with the error and the last good list is kept; a missing file is an empty list.

## Discover's bounds

- `fromdir` defaults to the working directory, with `--depth N` (default 4).
- It does not descend into a project it has found.
- It walks with `ignore::WalkBuilder`, the gitignore-aware walker `sync.rs` and `project.rs` already use, and skips `.git/`, `target/` and `node_modules/`.
- It does not follow symlinks.
- It prints registered, already registered and not registered with reasons.
