# Design: where Intent's per-user files live (WP-05)

**RULED by hv, 2026-09-13.** _"XDG\_\* looks very much like the right way to do. Use that or conform to that. That's what we want."_ _"What ever is the most compliant to standards, use that. Use you judgement."_ _"We can ignore v2 now as there's nothing left using it. Just push on as if it doesn't exist."_ The standard is the XDG Base Directory Specification, for `intent` and `intentd` alike. `XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_STATE_HOME` and `XDG_RUNTIME_DIR` join `no_intent_home.rs`'s `ALLOWED`, read in `userstate.rs` only, each taking the specification's default when unset or empty. Lands after the 3.0.2 tag, for 3.1.0 (vc).

## The layout

| Kind                                            | Path                                                          | What lives there                                           |
| ----------------------------------------------- | ------------------------------------------------------------- | ---------------------------------------------------------- |
| Configuration, `$XDG_CONFIG_HOME` (`~/.config`) | `~/.config/intent/config.json`                                | The operator's settings: `author`, the explorer's settings |
|                                                 | `~/.config/intent/projects.json`                              | The project registry (WP-03), hand-editable                |
| Data, `$XDG_DATA_HOME` (`~/.local/share`)       | `~/.local/share/intent/home`                                  | The install pointer                                        |
|                                                 | `~/.local/share/intent/{skills,subagents,agents}/`            | The installed-payload manifests                            |
|                                                 | `~/.local/share/intent/ext/`                                  | User extensions                                            |
| State, `$XDG_STATE_HOME` (`~/.local/state`)     | `~/.local/state/intent/`                                      | intentd's logs, the macOS app's build output               |
| Runtime, `$XDG_RUNTIME_DIR`                     | `$XDG_RUNTIME_DIR/intent/`, else `~/.local/state/intent/run/` | intentd's socket, address, token and lock                  |
| Apple's, not XDG                                | `~/Library/LaunchAgents/com.matthewsinclair.intentd.plist`    | launchd reads only that directory                          |
| Claude Code's, not Intent's                     | `~/.claude/`                                                  | Unchanged                                                  |

- **The runtime fallback is silent.** macOS never sets `XDG_RUNTIME_DIR`, so the specification's warning would print on every command there.
- **No `intent.d/`.** A `.d` directory holds drop-in fragments a tool composes; nothing here composes fragments.

## The migration

- **What moves.** The first 3.1.0 command to resolve a per-user path, finding `~/.intent/` and no `~/.config/intent/config.json` in v3's shape, moves `~/.intent/config.json`, `home`, `skills/`, `subagents/`, `agents/` and `ext/` into the layout, removes `~/.intent/`, and prints one line saying so. Hooks run through the installed binary, so a commit on any estate performs the move rather than refusing on it.
- **Read once, never again.** `~/.intent/` is read by that move and by nothing else in 3.1.0. A `~/.config/intent/config.json` not in v3's shape is replaced.
- **What a running daemon meets.** Runtime files are not moved: a 3.0.x intentd keeps its socket under `~/.local/share/intent/` until it stops. A 3.1.0 client looks under the new runtime path, finds no daemon, and runs in-process as it does with none running. `intent daemon restart` starts the 3.1.0 daemon at the new path and removes the old runtime files. The rebuild's daemon restart does this on this machine.
- **What an old binary meets.** A 3.0.x `intent` finds no `~/.intent/home`, and its existing refusal names `intent bootstrap`; running that recreates a `~/.intent/` the 3.1.0 binary never reads. The remedy is the 3.1.0 binary.
- **Every literal reader moves in the same change**: `userstate.rs`, `lib/templates/hooks/pre-commit-shim.sh` and `pre-commit.sh`, `bin/.devbin/cmd/{hooks,macos}`, the skills naming the manifest path, and the menubar app.

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
