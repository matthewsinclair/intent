# ST0045 Design -- Whiteboard Protocol 3.0 (per-node dirs + single-writer inboxes)

## 1. Problem (why 2.0 hurt)

Protocol 2.0 coordinated N concurrent Claude sessions through SHARED files:

- `asks.md` -- every stream appends messages AND removes resolved ones. N writers.
- `<stream>.md` -- per-stream, but touched by peers during cross-stream `archive`.
- `lamplight.md` -- the shared platform-edit broadcast channel. N writers.

Consequences, compounding with stream count:

- **Contention.** Concurrent edits collided (`modified-since-read`); the skill carried a large "do NOT archive a live peer's file" hazard.
- **Cleanse-difficulty.** Removing a resolved `asks.md` thread needed judgment over other streams' content AND coordination to avoid collision.
- **Unbounded growth + context burn.** Shared files grew to tens of KB and were reloaded on every `pickup`. Because cleanse was hard, it was deferred, so they ballooned.

## 2. Model (3.0)

Every file has exactly ONE writer. Each participant is a **node** with a directory:

```
intent/whiteboard/
  README.md                 # protocol reference + the project's node roster
  <node>/
    wip.md                  # the node's live board (frontmatter + DOING + TODO + watch-outs + decisions)
    inbox.<sender>.md       # one per OTHER node: messages FROM that sender
    history/YYYYMMDD/        # the node's archived DONE work + handled inbox entries
```

- `<node>/wip.md` -- written only by the node.
- `<node>/inbox.<sender>.md` -- appended only by `<sender>`; read + cleansed only by the owner.

Why per-SENDER inboxes (not a single `inbox.md`): one inbox would have N-1 writers -- back to 2.0. Per-sender = each file has exactly one appender.

## 3. Messaging

- **Point-to-point:** append to `<recipient>/inbox.<me>.md`. Reply flips direction (`<sender>/inbox.<me>.md`), threaded by `Re: <timestamp>`.
- **Broadcast:** `announce` writes one line to every peer's `inbox.<me>.md`. This SUBSUMES `lamplight.md` -- "touching the shared platform layer" is just an announce.
- **Escalation:** append to `hv/inbox.<me>.md`. The human is `hv`.
- **Recipient owns lifecycle:** read -> action -> `clear`/`archive` handled entries into own history. Single-owner cleanse, no cross-node coordination.

## 4. The hypervisor (hv)

The human is a first-class node, conventionally `hv` (the hypervisor) -- GENERIC in Intent, never the human's name. Symmetric: `hv/` has `wip.md` (standing directives + decisions + what hv is waiting on) + `inbox.<node>.md` for each Claude node. This gives escalations a real, persistent address instead of a `cc: <name>` annotation on a shared ask. Agents refer to `hv` / the hypervisor in all protocol language.

## 5. Skill changes (in-whiteboard SKILL.md)

Rewritten for the node model. Subcommand delta vs 2.0:

- `ask <node>` now routes to `<node>/inbox.<me>.md` (was: append to shared `asks.md`).
- NEW `announce <text>` -- broadcast to all peer inboxes (replaces the `lamplight` subcommand + file).
- NEW `clear <sender>` -- archive handled entries out of one own inbox.
- `archive` -- now archives ONLY your own `<you>/` dir; the 2.0 cross-file, cross-stream, collision-prone archive (with its "don't touch a live peer" hazard) is gone.
- `decide` -- appends to own `wip.md` `## Decisions` (was: own stream file's "Recent decisions" section).
- `pickup` -- reads own `wip.md` + own four inboxes + peers' `wip.md` frontmatter (was: read every `*.md` + filter `asks.md` on `to:`).
- `claim` / `unclaim` -- now in `wip.md` `claims` (renamed from `claimed_steel_threads`).
- Removed: the `lamplight` subcommand; the `to:` / `from:` ask header (the path encodes it); the weekly Monday-anchored history buckets (now per-node `history/YYYYMMDD/`).
- Identity: the 2-letter moniker is dir + routing key + handle (collapses 2.0's separate `stream_id` + `handle`).

## 6. Migration playbook (the Lamplight reference implementation)

Executed live on Lamplight 2026-06-15, peers quiescent:

1. Scaffold `whiteboard/{<nodes>}/` + the per-sender inbox files (one per other node).
2. Migrate each old `<stream>.md` -> `<node>/wip.md` (frontmatter -> moniker; focus + DOING/TODO + watch-outs + decisions; archive DONE into the node's history).
3. Route live `asks.md` entries into recipient inboxes; cross-node decisions into the sender's `wip.md` `## Decisions`.
4. Seed `hv/wip.md` (standing directives + the protocol-change decision record); route human-facing escalations to `hv/inbox.<sender>.md`.
5. Rewrite `whiteboard/README.md` (the project's roster + the protocol) and `.claude/restart.md` (the orientation pointers).
6. `announce` / welcome each Claude node in its inbox (protocol changed; where your state is; how to send/receive; reload skills).
7. Hard-delete the retired shared files (`asks.md`, `lamplight.md`, `cookies.md`, the flat stream files). Git preserves them.
8. Commit scoped (`--only`), no push.

## 7. Rollout (the Intent release)

1. Land this skill rewrite + ST0045 (drafted in the working copy at `../Intent` from Lamplight).
2. Version-bump Intent; changelog the Protocol 3.0 change.
3. `intent claude skills sync` to install the new skill (GLOBAL -- see blast-radius below).
4. For each project that adopts the whiteboard: run the migration playbook (section 6) once, peers quiescent.
5. Check the `in-session` (pickup) / `in-finish` (release) chaining: they invoke the subcommands, so the chaining is unchanged; only confirm no doc in them hard-codes the old file model.

## 8. Open considerations

- **Skill blast-radius.** The skill installs globally (`~/.claude/skills/`). Syncing 3.0 affects EVERY whiteboard-using project at once -- a project still on the 2.0 flat layout breaks under the 3.0 skill (it lists `<node>/` dirs and finds flat files). Mitigation: migrate each adopting project's layout in the same release window, or scope the new skill project-local during a per-project trial. Lamplight (the reference) was migrated first.
- **Inbox append/cleanse race.** A sender appending to `X/inbox.me.md` while X cleanses it is a possible overlap (rare: cleanse runs at pickup, appends during active work). Recoverable via optimistic concurrency (re-read + retry on `modified-since-read`). The bulletproof hardening, if ever needed, is maildir-style (one file per message); deferred, not built.
- **Naming.** `<node>/wip.md` (live board) vs `intent/wip.md` (post-session snapshot) share a basename but are distinct paths/roles; documented in README.
- **Daily archival discipline.** The model keeps files lean only if `archive` runs daily-or-more. Because it is now single-owner and collision-free, it is cheap -- so it actually happens.
