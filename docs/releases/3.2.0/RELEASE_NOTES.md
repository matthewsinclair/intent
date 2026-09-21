# Intent v3.2.0

**v3.2.0 makes a thread's related links writable, and makes `intent doctor` say what it used to stay silent about.** A thread's `related` list could be read and written by nothing, so a link naming a thread adopted under a new id could only be repaired by hand-editing canon; `intent st relate` and `intent st unrelate` are its doors now. Around that, this is a release of repairs found by running v3.1.0 across real estates: `intent doctor` reports a whiteboard row its board file lacks and a root file behind the installed Intent's templates, `intent index rebuild` repairs the index table it used to refuse behind, a fresh `git worktree add` no longer builds a whole store, and a search says whether it reconciled before it answered. **If you are running v3.1.0 there is no store migration to read about first**: the schema is unchanged, and the Upgrading section is short.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** Every claim below was verified against the tree the release is being cut from, by reading the code that produces the behaviour, by naming the test arm that holds it, or by reading the closed issue's own landing. A commit message is never the source. Anything ruled but not yet landed is deliberately absent rather than described in advance.

**The section headings follow the CHANGELOG's 3.2.0 section, which is what the release publishes**; this page carries the same entries with the reasoning a reader upgrading needs.

## Added

**`intent st relate <ID> <TARGET> [--note <text>]` and `intent st unrelate <ID> <TARGET>` write a thread's `related` links** (issue 0460). `related` was readable as a descent and writable by nothing: `intent set` hands every field a string and the list wants a sequence, so every value was refused, and the refusal sent the operator to a lifecycle verb or a member address, neither of which exists for this field. `relate` refuses an unknown thread, a target the project does not carry, and a thread named as its own target. **A link is a value, the target and its note together**: relating a target already linked replaces the note, a missing `--note` included, and relating it with the note it already has writes nothing and records nothing. `unrelate` drops a link whether or not its target still exists, which is the case the verb is for. Repointing a link is the drop and then the link, two acts and two events. Each act writes the store, canon and the realised `info.md` under one `st.relate` or `st.unrelate` event, and both verbs are offered on MCP.

**`intent doctor` reports a root file behind the installed Intent's templates** (issue 0496). `intent claude upgrade --apply` writes `AGENTS.md`, `CLAUDE.md`, `.claude/settings.json` and the git hooks' chain blocks and carriers from templates, and nothing reported one that had fallen behind: a `CLAUDE.md` a template change had left stale read `0 finding(s)`. Each such file that is present and would be rewritten is now named in a `root-file-behind` advisory, with the `claude upgrade` spelling that rewrites it. **It is shown and not counted**, because a newer Intent reads every project behind until the upgrade runs there. A file never installed is not reported, nor is one that is the project's own, and the pre-commit carrier stays with the gate check that already reports it.

## Changed

**`intent set` refuses `related` by name and names the two verbs.** This also closes the daemon's field-write door to the list, which took a whole array; a whole-thread write still carries it.

**`intent claude rules index` is new surface since the 3.1.0 tag, as a retired spelling.** The 3.1.0 register had no row for it; it now has a `retire` row, so it refuses by name at exit 2 rather than as an unknown subcommand. The generated reference's "what changed since" list compares shipped commands only and does not carry a retirement, so it is recorded here; the behaviour change itself is under Fixed.

**The `CLAUDE.md` that `intent claude upgrade --apply` writes describes the verb as 3.1.0 built it.** It now says that `--skip-settings` also leaves `.mcp.json` as it found it, that an existing `.claude/settings.json` not naming `intent claude hook` is held rather than overwritten unless `--force` is given, and that the standards skill reaches a machine's `~/.claude/skills/` through `intent claude skills sync`. A project's rendered `CLAUDE.md` picks this up at its next `claude upgrade --apply`.

## Fixed

**`intent doctor` reports a whiteboard row the store holds and its `board.json` does not** (issue 0495). Its store-versus-canon check rebuilt threads and issues and compared nothing on a board, so a board write whose render was refused left the row in the store and out of the file, and `doctor` said nothing where the same failure on a thread was reported. Each migrated node's board is now compared by the keyed comparison a restore already applies, and a node that differs is named in the same `store-stale` advisory with the door that lands a board's view. **It stays an advisory and is not counted in the verdict**, because on a shared tree it is also the normal state during another node's board write, so read doctor's whole output rather than its last line.

**`intent index rebuild` repairs an unreadable search-index table in place** (issue 0453). Every door opened the store through the damaged table, so the verb refused behind the damage it exists to repair. It now reads every other table first, refusing by name before any write if one of those is unreadable, then drops and recreates the two index tables from the store's own schema in one transaction. A store that cannot be opened at all refuses as before.

**A search answer says whether it reconciled the index before it answered** (issue 0484). A search skips its reconcile beside a watching daemon, under `--no-reconcile` and through `--daemon`, and its `complete: true` could not be told from one over a tree just walked. The envelope carries `reconciled`, and the terminal prints one line on stderr when the answer did not reconcile first.

**`git worktree add` no longer builds a whole store in the new tree** (issue 0483). The post-checkout carrier ran `intent sync --apply` on a fresh checkout, which on a large estate took minutes and wrote a store the size of the project's history. It now skips a fresh checkout and prints one line saying the first `intent` verb builds the store. A branch switch inside a checkout still syncs.

**`intent upgrade` keeps a project's flushed DONE list flushed** (issue 0485). A re-run on a v3 project rendered `todo.md` with every finished thread back in DONE, and `doctor` then reported the upgrade's own output as a hand edit. The upgrade now carries the watermark from the project's canon into the store it rebuilds, and an unreadable `project.json` refuses rather than rendering without it.

**`intent claude rules index` is refused as retired, at exit 2, like every other retired spelling** (issue 0475). 3.1.0 answered it with clap's `unrecognized subcommand` at exit 1, which the 3.0.3 entry that retired it describes, while `claude ws`, withdrawn the same way, got the retired refusal.

**The append-only guard protects `intent/.canon/events/`, not `intent/events.jsonl`** (issue 0458). No verb has written `events.jsonl` since 3.1.0 moved the log to one committed file per event. An edited or deleted event file is now refused at commit, with `git restore --staged --worktree --source=HEAD -- <path>` as the remedy.

**An authored `info.md` below a view's depth is an attachment that can be named** (issue 0461). A view name is now refused only at a view's own depth, the thread root and directly under `WP/<nn>/`; a v2 file whose name still cannot be carried is withheld with the rename-and-`intent st attach` remedy.

**`intent st attach` refuses a name no address can reach** (issue 0490), because it now asks the same naming gate `intent set` does.

**`intent daemon start` refuses a socket path the platform cannot bind, before it spawns `intentd`** (issue 0479), naming the path, its length, the platform's limit, and the shorter `XDG_RUNTIME_DIR` or `XDG_STATE_HOME` to set.

**`intent wb migrate` carries a numbered list one item per line, and marks a unit that reads as state for a person to read** (issues 0488, 0489). The report prints what each section yielded, zeros included, and puts a `READ THIS:` line on any uncarried or coerced unit that looks state-bearing. The mark decides nothing.

**A board write whose render failed names the door that lands a board** (issue 0487): `intent wb touch --node <you>`, and it says that neither `intent organize` nor `intent st sync` does it.

**A test run in a scratch tree cannot silently take over the machine's install pointer** (issue 0492), and the dry `claude upgrade` says when the pre-commit gate comes from another install. `intent bootstrap` still repairs a pointer that was taken over.

**Smaller corrections**: `intent at na`'s refusal on a test row names a flag that exists (issue 0480); the advisory critic hook hands its findings to the model as `additionalContext` (issue 0478), still opt-in and wired by neither shipped `settings.json`; `intent init` lists its config file relative to the project like every other path in its report (issue 0477).

**`intent wb migrate` names the restamp it makes, with both values: `heartbeat: <authored> as authored, <carry> at carry`** (issue 0497). The carry writes the node's heartbeat at its own instant and keeps the header's claim beside it, as ruled, so a board untouched for weeks rendered as live on the morning it was carried, and nothing in the report said so. Both values are read back from the row the carry wrote; a header that claimed no heartbeat reads `none as authored`.

## Upgrading

```
  $ brew upgrade matthewsinclair/intent/intent
  $ intent --version
  $ intent daemon restart
  $ intent claude upgrade --apply --skip-settings    # in each project
```

**There is no store migration in this release.** v3.1.0 and v3.2.0 both speak schema version 29, so the first command to touch a project opens its store as it is and migrates nothing.

**Restart `intentd` with the new build**: `intent daemon restart`. A running daemon keeps serving the build it was started from, and `intent daemon status` names both when they differ.

**Refresh each project's canon with `intent claude upgrade --apply --skip-settings`, and commit what it writes.** It re-renders `CLAUDE.md` with the corrected hooks paragraph above and rewrites the git hook carriers, which carry the post-checkout change that stops a fresh worktree building a store. `--skip-settings` leaves `.claude/settings.json` and `.mcp.json` as they are. Intent's own guards and the gate body are read live from the resolved install, so the append-only guard's new scope is in effect as soon as the install moves.

**A thread whose `related` list names a thread that no longer exists can now be repaired without touching canon**: `intent st unrelate <ID> <old>`, then `intent st relate <ID> <new> --note "<why>"`. `intent doctor` already names every such link.
