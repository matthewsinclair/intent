# Intent v3.3.0

**v3.3.0 gives a whiteboard a declared team.** A board can now name the Claude Team Profile it runs under, and Intent registers its seats, tells each session its seat at pickup and at launch, routes hv through one mediator, renders the roster README, and has `intent doctor` name a board that has drifted from its team. A node can be retired without losing its history. The release also stops the estate from calling live sessions gone when it cannot see Claude Code's sessions directory, and lets a project without `bin/devbin` declare its own build and test commands. It also fixes defects across the estate, search, the daemon and the generated contracts.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** They were written from the CHANGELOG's 3.3.0 section at 4410d1563. Each entry below was read against that section's text, and claims nothing it does not.

**The section headings follow the CHANGELOG's 3.3.0 section**; this page carries every entry, grouping related ones, with what a reader upgrading needs. `intent wb profile terms` prints the terms document that defines Team, seat, remit, mediator and the rest.

## Changed

**MCP's `intent_wb_status` renames its two counts and counts only what is live** (ST0092 WP-06). `items` becomes `live_items` and `messages` becomes `live_messages`; the old ones counted archived items and handled messages too. Each row also gains `last_pickup` and `never_started` beside `hats`, and the answer is still a bare array of rows. `intent wb status --json` gains the same four fields, and its text says `live items` and `live messages`. **A reader of the old names must read the new ones.**

**The index verbs name a language's level 3 as every search answer does** (issue 0664, ST0090 WP-01). `intent index status`, `rebuild` and `resolve` said `current` for a run whose files had changed since it resolved them, where search said `stale`; they now say `stale` too. In the JSON of all three and of the `intent_index_status` MCP tool, `state` under `resolution` is that state, and the last run's own word, which `state` used to carry, is now `outcome`.

**The opt tree `intent install` publishes records the version line its binaries print** (devbin 0332), as `# line: intent <version> (opt, <sha12>)` in its `.intent-install` header, and devbin's doctor compares each linked binary with it. An opt tree published by an earlier Intent fails that check until it is published again.

**Intent.app's Rebuild Search Index row works as Gtools' Refresh Search row does** (issue 0664, ST0090 WP-02). A rebuild no longer opens the log by itself, and one that fails, or cannot run, raises a `Rebuild failed` alert carrying the CLI's `error:` and `remedy:` lines.

**The `team` profile's vc proves each change once and batches changes into as few runs as it can** (ST0092). vc's default remit now says so, and the terms document gains a section, "Proof, and what it costs": one layer of proof per change, unrelated changes batched into one run, the mediator validating by reading the builder's evidence rather than running it again, no test of the tests added during a release, and the release's own gates the one whole test of what ships. The `/in-whiteboard` skill's validation node reads landed work against the ask and its builder's evidence.

## Added

**Claude Team Profiles ship with Intent, and `intent wb profile` serves them** (ST0092 WP-02). Four profiles ship beside the terms document: `team` and `solo`, which are bases, and `supervisor` and `supervised`, which are layers. Each seat is held once in `seats.json`. `intent wb profile list`, `show`, `validate` and `terms` answer from the installed Intent in any directory, and are on MCP as one tool per verb. `validate` names the file and field of each finding. The schema faces gain `team-seats.schema.json` and `team-profile.schema.json`.

**`intent wb init <profile>...` brings a board to what its Team Profiles declare, and `intent wb remit` sets a seat's remit** (ST0092 WP-03). `wb init` registers each declared seat that is absent and corrects a name or role that differs, and touches nothing else: no item, message, claim, heartbeat, status, focus or remit. Without `--apply` it writes nothing and prints what an apply would do. The declaration is recorded in the store and in `intent/whiteboard/declaration.json`, so a clone ingests the same team. `wb register` of a declared moniker takes only the profile's name and role. A seat with no remit of its own shows its profile's default. **The board moves to `intent/board@4.1`**: this build reads a 4.0 board, `intent doctor` names it as `board-schema-behind` without counting it, and the next board write re-renders it.

**On a board that declares a team, hv is reached through one node, and the tool enforces it** (ST0092 WP-05). The human seat keeps one inbox pair, with the mediator, and `wb init --apply` removes its other views, refusing while one holds a live message. `wb ask` between the human seat and any other node is refused, with a remedy that sends the same message to the mediator, and `wb announce` sends the human seat no copy. A write as the human seat is taken only from the mediator's Claude Code session or from a shell with no session. A board that declares no team is unchanged.

**A session is told its seat at pickup and at launch** (ST0092 WP-06). On a declared board, `intent wb pickup` prints the seat's brief ahead of the board: the team, the seat's name, role and remit, the mediator and the route to hv, and every other seat that is not retired, with a seat no session has run in marked never started. `--json` and MCP's pickup carry it too. The new `intent wb brief <moniker>` prints the brief without picking up, and `--launch` prints what `intent claude start` hands the session, whose launch steps are now the profile's. `intent wb status` marks a never-started seat in place of a heartbeat. A retired seat is refused its brief and its launch. A board that declares no profile picks up and launches exactly as before.

**The roster README is rendered from the board's Team Profiles, and keeps the project's own notes** (ST0092 WP-07). `intent wb init --apply` renders `intent/whiteboard/README.md` from one template, with the roster table and the route to hv. Everything between `<!-- user:start -->` and `<!-- user:end -->` is the project's and is carried by every render, and a first render over a hand-written README puts the whole file inside that block. An edit outside the block is refused by the apply and reported by doctor as `roster-readme-edited`; a README a newer Intent renders differently is `roster-readme-behind`. Neither is counted.

**The Desk is a hat with a written contract** (ST0092 WP-08). A session bound to a whiteboard node that is not its board's mediator is refused `intent estate control take`, unless `--hv "<hv's words>"` quotes hv's yes, which is recorded. The wearer's pickup prints that it wears the desk, the controller's duties and the asks waiting, and `wb status` shows the hat, as `hats` in its JSON and on MCP. When the controller's session is gone, `intent estate status` says the desk is orphaned and prints the step that takes it. The duties have one source, `lib/templates/estate/_DUTIES.md`.

**A node can be retired, and a board can move to another base profile** (ST0092 WP-04). `intent wb retire <moniker> --reason "<why>"` takes a node off the roster as a state, never a deletion: its board, items and messages are kept, `wb show` still reads it, and `wb status` lists it only with `--all`. A message it never handled is named for its sender with the command that archives it. An ask to a retired node is refused and an announce skips it. It refuses without a reason, for a node holding a claim, and for the board's human seat or mediator. `intent wb register` of the same moniker reinstates the same node. `intent wb init <base> --change --apply` moves a board to another base, registering the new seats and retiring the old ones it does not declare.

**`intent doctor` names a board that has left the team it declares** (ST0092 WP-09): `seat-differs-from-profile`, `seat-not-registered`, `closed-pair-view` and, for a declared seat that is retired, `seat-retired` with its reason. Each is shown and never counted, so doctor exits 0 and the commit gate passes. The remedy is `intent wb init`, then `intent wb init --apply`, which leaves a retired seat retired; the `intent wb register` command the finding names reinstates it.

**`intent wb pickup` and `intent wb status` say how old a seat's last read is** (issue 0675). A pickup prints how long ago the node last picked up on this machine, `wb status` carries each node's last pickup, and each peer's line in a pickup counts the messages that peer has not handled.

**`intent estate status` shows a start with no end whose asker is idle** (issue 0680): `running_idle` in the JSON, and a `RUNNING, ASKER IDLE` line under the controller. An asker with a background shell still running is not listed.

**`intent index status` and `intent index rebuild` say when the index was last reconciled** (issue 0664, ST0090 WP-01): a `reconciled:` line, and `reconciled_at` in their JSON and the `intent_index_status` MCP tool, read from the stamp the last reconcile wrote. Intent.app's Rebuild Search Index row shows it as a second line, `reconciled <age> ago` (ST0090 WP-02).

**A project without `bin/devbin` declares its own build and test commands for `AGENTS.md`** (issue 0678). `intent config set commands.build "<cmd>"` and `intent config set commands.test "<cmd>"` write them to `intent/.config/config.json`, and an empty value removes one. At the next `intent agents sync` the contract lists them. `bin/devbin` still wins where the tree has one. `config set` takes these two keys alone, and Intent never runs either command.

## Fixed

**The estate no longer calls live sessions gone when it cannot see Claude Code's sessions directory** (issue 0680). With `CLAUDE_CONFIG_DIR` unset, as a script that strips its environment leaves it, the estate read `$HOME/.claude`, found no sessions there, and `intent estate prune`, which intentd also runs on its census tick, removed every session's WIP record. A prune with no sessions directory now removes nothing. `intent estate status` no longer draws a live controller ORPHANED there; its JSON carries `no_sessions_dir` and it says so once, as a note or a warning. A take never succeeds over a controller whose session file will not read. Each slot verb now says which of four reasons kept it from finding its session.

**The estate page says ORPHANED only where the reader does, and shows a start whose asker is idle** (ST0092 WP-10, issue 0680). It reads `controller_orphaned` rather than judging for itself, draws a held slot ORPHANED only when its holder's session is `gone`, and lists each idle asker's running request.

**An `intentd` that Intent.app or `launchd` starts looks for Claude Code's sessions where the operator's shells do** (issue 0671). Intent.app hands every `intent` it runs its login shell's `CLAUDE_CONFIG_DIR`, and the LaunchAgent `intent daemon start --at-login` writes carries it. Beside a running daemon, `--at-login` rewrites a plist that lacks it.

**`intent daemon start --at-login` no longer says `ok:` beside a running daemon that nothing enrolled** (issue 0666). With no plist it refuses, naming the pid and the remedy; with this build's plist in place it says the daemon is enrolled and running; with an older build's plist it rewrites the plist and loads nothing.

**`intent bootstrap` makes the directory the `brew services` daemon logs into** (issue 0667), under `~/.local/state/intent`. The caveat and `docs/install.md` now say where the service logs.

**`intent estate slots ask --place-of` keeps the place of a grant the controller revoked before its start** (issue 0668), as it keeps the place of one its holder released unrun.

**`intent search --outline`, `--context` and a filter-only search reconcile the index before they answer, as a text query does, on the CLI and in the `intent_search` MCP tool** (issue 0669). `--no-reconcile` skips it on each. **Beside a watching daemon, the MCP tool no longer writes the index itself**: it leaves it to the watcher and says `"reconciled": false`, as the terminal's search already did.

**A verb under `intent estate slots`, `estate control`, `estate lane`, `claude skills`, `claude subagents`, `claude rules` or `surface` lists only its own flags in `--help`, and refuses any other flag by name before it runs** (issue 0661), at exit 1. A flag given before the verb, which was parsed and never read, is refused too. `intent llm guide` names the verbs that take each flag.

**An acceptance-test row no longer passes the contract check on another thread's arm** (issue 0670). A bare id line stops counting for a row wherever another thread has a row of the same id citing the same file, and the refusal names the rival thread. A row may now cite its arm as `<file>:<line>`, and the id is matched as a whole token, so `AT-3.5` is no longer found inside `AT-3.50`.

**`intent doctor --scope closed` names a completed thread's green AT row whose cited file no longer carries the row's id** (issue 0665), as `closed-contract-drift`, shown and never counted. Nothing reopens.

**The Rust rules the critic defers to clippy are run by a clippy gate** (issue 0649). CI and `bin/devbin check` deny `needless_pass_by_value` over the workspace's library and binary code and the restriction lints over `intentd`'s binary, and every site they raised is fixed, apart from five error adapters `map_err` forces to take their error by value, which carry an `#[allow]` with that reason. `redundant_clone`, which no gate runs, is no longer one of IN-RS-CODE-002's tool codes.

**`AGENTS.md` names no language's default build or test command** (issue 0672). It said `cargo test`, `swift build` or `bats -r tests/` whatever the project built with. The contract now says a project builds and tests through `bin/devbin` where it has one, booking a slot first, and otherwise that it declares none, unless the project has declared its own (issue 0678 above).

**`intent claude upgrade --apply` keeps the commit gate's tracked dispatchers out of every release archive** (issue 0650). Where `core.hooksPath` names a directory in the tree, the upgrade keeps a marked `export-ignore` block for it in `.gitattributes`, and a line of the project's own keeps the last word. `intent doctor` names a `.gitattributes` that lacks the block.

**A session whose `/in-session` ran before the project declared its languages is told so** (issue 0676). The gate's sentinel records the languages the run loaded skills for, the gate compares them with `intent lang declared` on every prompt and says to run `/in-session` again when they differ, and `intent wb pickup` says the same on a `languages moved:` line.

**The mechanical trope pass names one pattern engine, and every automated pattern is tested under it** (issue 0677). IN-PR-STYLE-004 ran perl-dialect patterns through `grep -iE`, which engines read differently, so a tell was found on one machine and missed on another. The rule now gives one `/usr/bin/perl` program, critic-prose and in-detrope say the same, and a Bats test runs every automated pattern under it.

## Upgrading

```
  $ intent daemon stop             # or brew services stop intent, if it runs there
  $ brew upgrade matthewsinclair/intent/intent
  $ intent --version
  $ intent daemon start            # or brew services start intent, if it ran there
```

**Every store migrates, from schema 33 to 35, on its first open** under v3.3.0, and v3.2.8 then refuses it as written by a newer Intent. **Boards move to `intent/board@4.1`, and v3.2.8 refuses a board v3.3.0 has written.** Upgrade every machine that opens a project, and every machine that reads its boards, before any of them works in it again.

**Run `intent claude upgrade --apply` in each project** to take the `AGENTS.md` that names no language's default commands (issue 0672) and the `.gitattributes` block (issue 0650), and `intent agents sync` after declaring any `commands.build` or `commands.test`.

**Run `intent claude skills sync` and `intent claude subagents sync`** to take the changed `in-session`, `in-whiteboard` and `in-detrope` skills and the `critic-prose` subagent.

**MCP clients of `intent_wb_status` read `live_items` and `live_messages`**, and readers of the index JSON read the run's own word as `outcome`.

**Under `brew services`, run `intent bootstrap` once** so the directory the daemon logs into exists (issue 0667). **A LaunchAgent enrolled by an earlier build** is rewritten to carry `CLAUDE_CONFIG_DIR` by `intent daemon start --at-login` (issue 0671).

**An opt tree published by an earlier Intent** fails devbin's doctor until `intent install --force` publishes it again from the dev checkout. A Homebrew install needs nothing.

**Restart intentd, and running Claude Code sessions, to give them the new build.** A daemon left running goes on with the old binary, and each session's `intent mcp` server keeps the binary it started with.
