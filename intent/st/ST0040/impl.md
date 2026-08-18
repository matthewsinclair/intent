# Implementation - ST0040: Whiteboard protocol for multi-Claude sessions in the one repo

## Shipped in v2.11.7 (2026-05-18)

Skill, chain integrations, auto-install in the upgrade dispatcher, regression tests, and the `working-with-llms.md` "Multi-session coordination" section all land in v2.11.7. The release is a patch (overriding the "new skill = minor" precedent) on the grounds that the protocol is opt-in by directory presence — projects without `intent/whiteboard/` see zero behaviour change. Two small additions came from a late-stage review of the cross-project LLMsend protocol (<https://github.com/pmarreck/llmsend/blob/yolo/skills/LLMsend/SKILL.md>): optional `Re:` cross-reference and `FYI only` headers on `asks.md` entries. The tmux/kitty live-ping mechanism from LLMsend was considered and deliberately not adopted for the intra-project case.

## Implementation

The `in-whiteboard` skill was landed out-of-cycle into Intent's canonical plugin pack on 2026-05-18 to unblock the Lamplight project (which had two concurrent Claude Code sessions hitting coordination friction). This `impl.md` records what's already in tree; remaining work to roll into a formal Intent release lives in `tasks.md`.

### Canonical source files added

1. **New skill**: `intent/plugins/claude/skills/in-whiteboard/SKILL.md`
   - YAML frontmatter (`description:` + `chains_to: []`).
   - Subcommand surface documented inline (pickup / claim / unclaim / touch / ask / decide / lamplight / release / status).
   - Stream-identity discovery prose: arg > inference > ask user.
   - Per-subcommand procedure with explicit numbered steps.
   - Protocol invariants section.
   - Red Flags table covering the predictable rationalisations.

### Canonical source files edited

2. **`intent/plugins/claude/skills/in-session/SKILL.md`**
   - `chains_to:` extended with `"in-whiteboard"`.
   - New procedural step 5: "Pickup the whiteboard" -- invokes `/in-whiteboard pickup` if `intent/whiteboard/` exists; skips silently otherwise.
   - Existing "Confirm and proceed" renumbered to step 6.

3. **`intent/plugins/claude/skills/in-finish/SKILL.md`**
   - `chains_to:` extended with `"in-whiteboard"` (placed before `"in-verify"`).
   - New procedural step 1: "Release the whiteboard" -- invokes `/in-whiteboard release` if `intent/whiteboard/` exists; skips silently otherwise.
   - Same opt-in-by-presence guard as the in-session chain.
   - Existing steps 1-5 renumbered 2-6.
   - "Skill Chain" section at the bottom updated to mention `/in-whiteboard release` ahead of `/in-verify`.

### Opt-in-by-presence guard

Both chain integrations honour the same guard: if `intent/whiteboard/` does not exist in the project root, the chained `/in-whiteboard` step skips silently. Existing Intent projects without a whiteboard see zero behaviour change. New projects gain coordination once they create the directory (manually or via a future `intent whiteboard init` -- not implemented in v0).

## Code Examples

### Stream file (real example from Lamplight, 2026-05-18)

```yaml
---
stream_id: control
current_session_id: 47e67e0b-1f7f-4c47-a11a-a105e448ec52
session_started_at: 2026-05-18T15:00:00Z
heartbeat_at: 2026-05-18T16:32:00Z
status: active
focus: "ST0177 WP-05 -- experience-state lifecycle"
claimed_steel_threads: [ST0163, ST0177]
recent_memory_writes:
  - feedback_typed_pctx_slot_for_builtins
  - feedback_framework_vs_experience_catalog
  - feedback_no_silent_failures_anywhere
---
# Control Stream

## Recent decisions affecting other streams

- (2026-05-18) Combat promoted to first-class Pctx direct slot.
  `pctx.combat` is now the canonical accessor; `pctx.mechanics[:combat]` is retired.

## Watch-outs

- `Pctx` struct is in flux -- direct slots being added per WP. Access by direct
  field (`pctx.scenes`, `pctx.phase`, `pctx.combat`), not via the `mechanics` bucket.
- `Run.Narration.SubstitutionResolver` is the Highlander for `{{X}}` Mustache slot
  resolution. Do not re-implement slot resolution.
```

### Skill invocation pattern

```
/in-whiteboard pickup            # session start (auto-chained from /in-session)
/in-whiteboard claim ST0177      # entering a steel-thread
/in-whiteboard ask ia-ux "Pctx field rename in flight; pause LV-side reads of pctx.combat for 2h"
/in-whiteboard decide "Combat promoted to first-class Pctx slot"
/in-whiteboard release           # session end (auto-chained from /in-finish)
```

## Technical Details

- **No external state.** All protocol state lives in `intent/whiteboard/` files. No daemon, no inter-process locking, no `intent/.config/whiteboard.json` (v0).
- **Append-only on shared files.** `asks.md` and `lamplight.md` are append-only -- new entries never modify or move existing entries. Single-writer-per-file for stream files.
- **Heartbeat semantics.** `/compact` does NOT trigger session-end. The next `/in-session` (auto-firing in the new context) calls `pickup` which touches `heartbeat_at`. A stream stays `active` across `/compact`.
- **Stale threshold.** `heartbeat_at > 7 days old` AND `status: active` => the stream is **stale**; another stream may reclaim with explicit user acknowledgement. The skill never silently overwrites another stream's claim.
- **Stream-identity discovery.** Inference order: (1) arg to `/in-whiteboard pickup <slug>`; (2) cues (cwd, branch, recent commits, wip.md In-flight labels, user framing); (3) ask the user. Once a stream file exists, subsequent sessions of that stream inherit identity from the file.

## Challenges & Solutions

### Challenge: where to put cross-stream "I just decided X" signals

Three plausible homes: the deciding stream's own file, a shared `decisions.md`, or `asks.md`. Chose **the deciding stream's own file** as the default and `asks.md` for true point-to-point handoffs that need the other stream to act. Rejected `decisions.md` outright -- it would duplicate `intent/history/YYYYMM-done.md` past-tense badly, and dilute `done.md`'s authority.

### Challenge: glob-path claims drift from actual edits

The claimer types `apps/control/**` but actually edits `apps/lamplight/lib/lamplight/core/runloop.ex`. The other stream's view of "what's claimed" diverges from the truth. Resolved by claiming by **steel-thread ID only**; paths are inferred from `intent/st/<ID>/info.md` cross-references. STs are the user's mental model; globs are not.

### Challenge: shared platform layer (`apps/lamplight/**`) no ST claims cover

Both streams may legitimately edit the shared platform layer. ST claims cannot cover this without overlap by construction. Resolved with a dedicated `lamplight.md` shared file: claim-on-touch with a one-line "I'm about to edit X for Y reason" pattern.

### Challenge: ST-ID allocation race in `intent st new`

`intent st new` scans the disk for max ST number and increments. Two concurrent sessions both read max=N, both create ST(N+1), one wins, one silently overwrites. This bit the Lamplight project on 2026-05 (ST0178 was claimed by Bounded Improvisation before the MCP Player surface ST could grab it). **Not solved here** -- the fix belongs in `intent st new` (flock + atomic write of a stub `info.md`). Documented in `tasks.md` as a known-bug pointer.
