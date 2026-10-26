---
verblock: "23 Oct 2026:v0.1: ic - first inventory for WP-01"
---

# Every surface that feeds prompt text to an agent

This is the report ST0085 WP-01 asks for: each place where text that Intent stores or ships reaches an agent as something to act on, how the agent comes to read it, whether it can put a stale copy in front of the agent, and what marks a non-live copy today. It proposes no fix. The fixes are new surface behind the line drawn on 2026-10-23, and hv reads this before any fix lane starts.

**Prompt text** here means text written for an agent to act on: skills, agent files, rules, the root contracts, hook output, remedies, and rulings or messages addressed to a node. It also means prose an agent reads while working that carries instructions, such as an issue's spec or a thread's plan.

**A stale copy** is a copy of prompt text that is not the live version (superseded, retired, archived, never landed, or installed from an older Intent) put in front of an agent with nothing in reach saying so.

**How it was measured.** On 2026-10-23, on the development machine, by ic:

- canon locations and their installers were read;
- installed skills and subagents were compared with canon;
- the hook wiring was read in each of this repository's settings files;
- the search index's reach over archive paths was probed with path filters;
- the head of each kind of archived file was read for a liveness marker;
- git history was searched for skills and subagents canon deleted.

What was not measured is listed at the end.

## Loaded or served as live instruction

### Skills

- **Where:** canon is `intent/plugins/claude/skills/<name>/SKILL.md`, with `scripts/` and `data/` beside some.
- **How an agent reads it:** `intent claude skills install|sync` copies a skill into `${CLAUDE_CONFIG_DIR:-~/.claude}/skills/<name>/` and records a checksum in `~/.local/share/intent/skills/installed-skills.v3.json`. Claude Code lists every installed skill at session start and loads one in full when it is invoked, by a person, by `/in-session`'s chain or by the model.
- **Stale routes:**
  - `sync` holds a skill edited locally, so the held copy keeps its old text until someone resolves the hold.
  - Canon has deleted skills: `in-next` and `in-start` at ae6a83ceb (2026-08-29), `in-handoff` at 861fa66c9 (2026-08-25). A machine that installed one before then keeps a directory that `intent claude skills list` reports as `unlisted`, and Claude Code loads it while it holds a `SKILL.md`.
  - The install is per user, not per project, so every project on a machine sees one installed set, whatever Intent its own files were last written by.
  - A live skill can quote its own superseded instruction to explain a reversal: `in-finish/SKILL.md:43` quotes its old first bullet, _What was accomplished this session_, which it says it carried until 2026-09-08.
- **Measured on this machine:** every canon skill reports `installed canon`. An extra directory, `synced`, reports `inert` (it holds empty `.bucket-*` files and no `SKILL.md`), so Claude Code cannot load it.
- **Marker today:** none in an installed copy. Nothing an agent reads names the canon revision it came from; the manifest's checksum is not shown to the agent.

### Agent files (subagents)

- **Where:** canon is `intent/plugins/claude/subagents/<name>/agent.md`. `intent claude subagents install|sync` copies each to `<config dir>/agents/<name>.md`.
- **How an agent reads it:** Claude Code offers every installed agent to its Agent tool, and the agent's text becomes that subagent's instructions. The critic agents read the rule library when invoked.
- **Stale routes:**
  - The skill routes above, for the same reasons. Canon deleted `worker-bee` at 65f3ceac1 and the multi-file `elixir` agent at 3b2da98a7 (both 2026-04-22).
  - A project can carry its own agent files under `.claude/agents/`, which Claude Code loads for every session in that project and no Intent installer reads or compares. This repository carries `.claude/agents/elixir.md`: it is tracked, it was last edited at adf7d1096 (2026-09-24), and it is offered to this session's Agent tool, while canon's `elixir` agent is the one deleted on 2026-04-22. It is maintained by hand rather than left behind, but nothing would report it if it went stale.
- **Measured on this machine:** every canon agent reports `installed canon`.

### The rule library

- **Where:** `intent/plugins/claude/rules/`, one directory per pack (`agnostic`, `elixir`, `rust`, `swift`, `lua`, `shell`, `prose`, `author`, `content`) plus `_schema` and `_attribution`.
- **How an agent reads it:** the installed Intent tool serves it at run time (`intent claude rules list|show`). The `critic-<lang>` agents and the headless `intent critic` read it when they run, and skills cite rule IDs and tell the agent to read them (`in-review/SKILL.md:54`).
- **Stale routes:**
  - The install root decides which copy is served. On this machine the root is this working tree, so an edit is served the moment it is saved, before anyone has judged it (decision 30). A brew install serves the copy packaged with its version, so an older install serves older rules.
  - Every rule has a `## Bad` example, which is instruction-shaped by design, and `_attribution` carries upstream text.
  - A search of the rules' frontmatter for a retired, withdrawn, superseded or deprecated status found none. A rule leaves by deletion, so its last text survives only in git history and in older installs.

### The root contracts

- **Where:** `lib/templates/llm/_AGENTS.md` and `_CLAUDE.md`, and the seeds for `usage-rules.md`, `RULES.md` and `ARCHITECTURE.md`. `intent claude upgrade --apply` and `intent agents sync` render them into each project, at the root or under `.claude/` as its `contracts` key says. `intent init` renders from copies embedded when the binary was built.
- **How an agent reads it:** Claude Code loads `CLAUDE.md` at session start, and through its `@AGENTS.md` line, `AGENTS.md`. Other agents read `AGENTS.md`.
- **Stale routes:**
  - A project not upgraded since keeps the contract text of the Intent that last wrote it.
  - `usage-rules.md` is seeded once and never overwritten, so it drifts from canon by design.
  - Older projects may still carry a retired `intent/llm/AGENTS.md`, or the Elixir and Phoenix `DECISION_TREE.md` that `init` once wrote into every project.
  - Ignored backup directories keep earlier contracts: this repository's `.backup/backup-20260724-104904/` holds a `CLAUDE.md` from July.

### Text the running build generates

- Hooks:
  - `.claude/settings.json` wires SessionStart (`session-context`), UserPromptSubmit (`require-in-session`) and Stop (`session-finish`).
  - The PostToolUse hooks, `post-tool-symbol-context` and `post-tool-advisory`, are off by default and wired by hand. This repository's `.claude/settings.local.json` wires `post-tool-symbol-context` on Grep and Bash, which appends index hits to the agent's context after a search. `post-tool-advisory`, run as `intent claude hook post-tool-advisory` where a project wires it and sets `post_tool_use_advisory` in `.intent_critic.yml`, appends critic findings after an edit.
  - Every hook body is served from the running install's `lib/templates/.claude/scripts/`, so its text is as current as that install.
- The MCP server: `intent mcp` serves instructions and tool descriptions built from `surface/dispatch-table.json`, from its `mcp_instructions` and each row. A server process carries the text of the binary that started it. A rebuild does not reach a server already running, which keeps the old code until it is reopened (`intent/restart.md`).
- Verb output: `--help`, `intent llm guide`, `intent claude rules show`, and every `remedy:` line on a refusal, which an agent is meant to follow. All of it comes from the binary that answers, so it is stale only when an older binary answers.

## Records an agent reads while working

### Steel threads

- **Where:** `intent/st/<ID>/`: generated covers and acceptance views, and attachments such as `design.md`, `impl.md` and `tasks.md`. The text of every attachment is also in `intent/.canon/st/<ID>.json` and in the store.
- **How an agent reads it:** on demand: `intent st show`, a read of the tree, `intent search`, the MCP search.
- **Stale routes:**
  - A completed thread's plans and instructions stay readable. While it is hydrated they sit in the tree beside live work: ST0056 and ST0057 stay hydrated because the gate, the release script and acceptance rows cite files under them, and ST0056 holds design, implementation, task and contract documents side by side.
  - The cover carries the thread's status. An attachment read alone does not: `ST0056/critic-gate.md` carries a verblock with dates and no status. So a reader of the file alone cannot tell a live contract from a finished plan; ST0056's `data-model.md` is still read as the data-model contract by the test suite (`intentsvcs/tests/common/mod.rs:804`).

### Issues

- **Where:** `intent/issues/<n>.md` (a generated view) and `intent/.canon/issues/<n>.json`. A body can carry a spec addressed to a node; 0622's reads "LANE: ic, S. SHAPE: ...".
- **Stale routes:** a closed issue's view leaves the tree at the next organize (0625's has), but its record, body and spec stay in canon and in the store, and `intent issues show` prints them under status CLOSED. The status is the marker. A body quoted away from its record has none.

### The whiteboard

- **Where:** `intent/whiteboard/<node>/board.json` holds every item the node wrote, archived ones included, and every message it sent. `wip.md` and the `inbox.<sender>.md` views are rendered from it.
- **How an agent reads it:** `intent wb pickup` prints the node's board and live messages, and prints hv's directives, watch-outs and decisions in full, to every node at every session start. `intent wb show <node>` reads any board. The files are greppable.
- **Stale routes:**
  - hv's decisions are printed at every pickup whether spent or standing. This morning's pickup printed decision 24's "THE VERSION IS 3.1.0" and decision 25's instruction for "after the bounce" beside rulings that still bind. Decision 10's own "advisory until hv confirms" and decision 17's "corrects decision 8 forward" are the only liveness markers in the set, and both were written by hand into the text.
  - An inbox view renders every message its sender ever sent that recipient, handled ones unmarked (issue 0627, now specced).
  - Archived is a state in `board.json`. `intent wb show --all` and the JSON list archived items; the rendered `wip.md` does not.
  - Every write's event under `intent/.canon/events/` keeps the item's or message's text (`payload.text` on a `wb.add`), readable by grep.
- **`.history/` per node:** the hand-authored era's folds and the pre-migration copies, kept append-only (the commit gate's `append-only-guard.sh` refuses any removed line). A fold file opens with the same header a live board carries (`node`, `session_id`, `heartbeat_at`, `status`), so only its path says it is an archive. Messages addressed to "you" and instructions for sessions long ended live there.

### `intent/history/` and the project snapshots

- `intent/history/` mixes a live record with snapshots. `20260915-hv-rulings.md` is the record `intent/restart.md` names for hv's rulings. The `*-wip-prefold.md`, `*-restart-prefold.md` and `*-done.md` files and the v2 release notes are snapshots.
- A prefold snapshot carries the same verblock frontmatter as the live `intent/wip.md`; its filename is the only marker.
- `intent/wip.md` and `intent/restart.md` are live by construction, and `.claude/restart.md` holds no state.

### Docs and release text

- `intent/docs/` holds the authoring guides, which quote skills, rules and hooks. `docs/` holds the user docs, `docs/reference/` (generated at each cut) and `docs/releases/`. Beside them sit `CHANGELOG.md` and `docs/v2/`, the v2 README, blog and releases.
- Release notes and the CHANGELOG describe behaviour as it stood at each release, including verbs since retired, and `docs/v2/` describes the v2 Bash CLI, which has been deleted.
- The reference pages carry the revision they describe; the rest carry no liveness marker.

### The store, the search index and the backups

- `intent/.cache/intent.db` holds every record, every archived board item, every message, and the search index over the code and prose corpora.
- `intent search`, and the MCP `intent_search` whose instructions tell an agent to use it first for "where is this written across the whole project", return hits from the whiteboard `.history/` trees, `intent/history/`, `docs/v2/` and the hydrated ST0056 tree; each was probed today with a path filter. A hit carries its path and tier and no liveness field. The event log is not indexed: its probe returned no hit.
- `.backup/` and `intent/.backup/` (ignored) hold store snapshots and damage evidence as database files, each carrying everything the store held when it was taken.

## Git history and other copies of the tree

- History: `git log -p`, `git show <rev>:<path>`, `git diff` and `git blame` give every superseded skill, rule, template, board and document in the same shape as the live one. Readers that take history as input include code review of a change (its diff), the commit gate (`intent critic --staged` reads the staged diff), the reference generators (`git show <rev>:<path>`), the validation role, and any agent asked about a change.
- Banks: `refs/bank/<node>/...` hold patch blobs that are kept for good, since no spent bank ref is deleted (vc's ruling of 2026-09-22). Banks that never landed are among them, so a bank of a skill or template change can be prompt text that was never live.
- Worktrees: the linked worktrees under `tmp/wt-*` are full checkouts at older commits, skills and templates included. Several were open at the time of measuring.
- The v2 checkout: the frozen checkout at `~/Devel/prj/_Archive/Intentv2` (named in `intent/restart.md`, outside this repository) holds v2's skills, templates and docs, which parity work reads.
- Scratch copies: session scratch directories hold copies agents make; this session copied a generator and its libraries into its scratchpad today.

## Outside Intent's ownership, inside the same loop

- Session transcripts: `~/.claude/projects/<project-key>/*.jsonl` hold every instruction, skill text and tool output of past sessions. `/in-autopsy` reads them (`in-autopsy/SKILL.md:146`), and the validation role reads a node's transcript.
- Claude Code project memory: `<config dir>/projects/<project-key>/memory/`. Its `MEMORY.md` index is loaded at session start in this estate, and the files it names are read on recall. One directory serves every node of a project, and each entry reflects when it was written. `/in-autopsy` compares sessions against it (`in-autopsy/SKILL.md:44`).
- Other estates and machines: each carries its own installed skills, contracts, boards and history at its own Intent version. On this machine one user-level install and this tree's templates serve every estate at once.

## What marks a non-live copy today

- An ownership marker, not a liveness one: generated views say "Generated by Intent v3 ... Do not edit".
- Status fields: thread and work-package covers, issue records (CLOSED), and board items (archived, a state the rendered view does not show).
- Hand-written markers: a few decisions on hv's board, and the "used to" and "until <date>" notes inside skills.
- Path or filename alone: `.history/`, `intent/history/*-prefold.md`, `docs/v2/`, `tmp/wt-*`, `.backup/`.
- Nothing at all: git history and banks; transcripts; a fold file or a completed thread's attachment read alone; a search hit; text from an older running MCP server, hook body or binary.

## Not measured

- Other machines and estates, and the payloads of brew-installed kegs.
- Whether `intent claude skills sync` removes an installed skill that canon no longer carries.
- Whether this repository's `usage-rules.md` differs from the seed canon would write today.
- How Claude Code's memory loader treats memory files beyond the `MEMORY.md` index.
- Whether any agent has acted on a stale copy beyond the case issue 0611 was raised on.
