# The store

**The database is the source of truth. The files on disk are a projection of it.** That sentence is the whole design, and almost every surprising behaviour in Intent follows from it.

## Why it works this way

Intent v2 kept everything in Markdown files and derived what it needed by parsing them. That is the obvious design and it fails in a specific way: **a file is a document and a document has no schema**, so every reader reconstructs the model slightly differently, every writer formats slightly differently, and the two drift without anything being able to see it.

v3 inverts it. Objects live in a store with a real schema, and the Markdown you read is generated from them.

| Layer              | What it is                                        | Who writes it           |
| ------------------ | ------------------------------------------------- | ----------------------- |
| **The store**      | `intent/.cache/intent.db` — the source of truth   | The tool, via its verbs |
| **Canon extracts** | `intent/.canon/st/ST####.json` — tracked in git   | `intent sync`           |
| **Views**          | `intent/st/<ID>/*.md` — generated, human-readable | `intent sync`           |

**The store is not in git; the canon extracts are.** That is the split that makes the design workable in a team: the database is a local cache that can always be rebuilt, and what your colleagues review in a pull request is the JSON extract, which has a schema and diffs sensibly.

## The generated views carry a banner and it means what it says

Files under `intent/st/<ID>/` are **generated**. Editing one is not an error the tool will report — it is an edit that the next sync overwrites.

If you want to change a thread, use a verb. If you want to change something no verb reaches, edit the canon extract and sync it back:

```
  $ $EDITOR intent/.canon/st/ST0001.json
  $ intent sync --to-store ST0001
  $ intent sync --to-disk  ST0001
```

**Always scope a sync to a thread id.** A bare `intent sync` is a whole-project operation and it is almost never what you meant.

### Sync reads the worktree, so sync before you commit

**This is the ordering rule that catches everyone once.**

`intent sync --to-store` reads files **as they are in your working tree**, not as they are in HEAD. So if you edit a file and commit it first, then sync, the canon is built from the post-commit worktree — which is fine — but if you commit the file and the canon separately, **the first commit is permanently divergent**: it holds file bytes that the canon it shipped with does not describe.

The compliant order is:

1. Edit the file.
2. `intent sync --to-store <ID>`, then `intent sync --to-disk <ID>`.
3. Commit the file **and** the canon together, in one commit.

The pre-commit gate enforces this by refusing a commit whose canon names bytes the commit does not carry. It is a backstop on the commit path rather than a detector of the underlying divergence — canon on disk can be wrong between a sync and a commit, and nothing says so.

## What lives only in the store

Some things have no file projection at all, deliberately:

- **The event log** — append-only, every state transition with who and when. This is what makes a fiat close permanent as a record even though the state it produced is reversible.
- **The file index** — a git-style index the sync engine uses to know what changed.
- **Document sections** — the result of prose ingest, used by `intent search`.

## Disk is sparse and that is not a bug

**A thread's files are realised on demand.** A thread can exist in the store with no directory on disk at all, and most attachments a store knows about name a path that does not exist. That is the design working: the projection is created when someone asks for it.

The practical consequence is worth stating because it looks alarming from the outside: **counting store records whose file is missing measures nothing useful.** It cannot separate "stale after a rename" from "never realised", and those are different facts. If you write a tool against the store, say what your denominator is before you report a number.

## Backups

```
  $ intent backup
```

Backups land in `intent/.backup/`. **That directory must be in `.gitignore` and never tracked** — a backup in git is a second copy of everything, diverging from the first, forever.

---

Back to [Concepts](index.md), or on to the [command reference](../reference/).
