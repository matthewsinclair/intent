# Intent v2 — historical record

Everything under this directory documents **Intent v2**, the Bash implementation whose final release was **v2.19.0** (2026-08-14). It is frozen. Nothing here describes the tool you get from `brew install intent` today.

Intent v3 is a full Rust rewrite. Its documentation is the rest of `docs/`, starting at [`docs/index.md`](../index.md). If you are migrating, [`docs/migrating-from-v2.md`](../migrating-from-v2.md) is written for you specifically.

## What is here

| Path                     | What it holds                                                          |
| ------------------------ | ---------------------------------------------------------------------- |
| [`blog/`](blog/)         | Seven posts on the thinking behind Intent, plus two unpublished drafts |
| [`releases/`](releases/) | Release notes for eighteen v2 versions, 1.2.1 through 2.19.0           |

## Why the canonical URLs point somewhere else

Every post in `blog/` carries a `canonical:` field naming its **old** location under `docs/blog/`, not its location here. That is deliberate and it should not be repaired.

These posts have been linked to and indexed for a year under those URLs. Rewriting the canonicals to `docs/v2/blog/` would make the archive authoritative for its own content — tidier, and it would discard every inbound link and whatever search ranking the posts have earned. That is not recoverable once reindexed.

So the trade was made explicitly: the archive is **not** the authoritative location for what it holds, and old links keep resolving. If you are tempted to make the canonicals match the paths, this paragraph is the reason not to.

## What is deliberately absent

There is no v3 blog. The v2 posts stay as a record of what was argued at the time; they have not been updated to describe v3, and several of them describe behaviour v3 does not have. Where a post has already been overtaken by events it carries an editor's note saying so, added when it happened.

---

_Frozen 2026-08-29. Superseded by the v3 documentation set at [`docs/`](../)._
