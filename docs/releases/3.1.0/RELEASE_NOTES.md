# Intent v3.1.0

**v3.1.0 teaches Intent's index what a symbol IS, and then teaches it to resolve one.** v3.0.2 gave the index a search surface that could find a name; this release gives it the shape around the name -- what kind of thing it is in its own language's words, what type or module it is written in, how many arguments it takes -- and a third level that asks each language's own toolchain which definition a reference actually points at. **It is also the first release built for a project more than one person works on**: history travels in the repository, one command brings a clone's store up to date after a pull, and an id two clones both minted has a verb that repairs it. **If you are running v3.0.3, the new questions are the reason to upgrade and the store migration is the thing to read about first**: it is one-way, and the Upgrading section says how to keep a way back.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** Every claim below was verified against the tree the release is being cut from: by reading the code that produces the behaviour, by naming the test arm that holds it, or by reading the closed issue's own landing. A commit message is never the source. Anything ruled but not yet landed is deliberately absent rather than described in advance: a release note that documents a verb the release does not contain is the same defect as a manual page that does, and it is harder to withdraw.

**One open defect is named here rather than left for a reader to find, and this release detects it.** The store's FTS5 index for the source half can go malformed, so a search on an affected term fails; before this release `intent doctor` and `intent index status` both read clean over it. `intent index rebuild` repairs it. The cause is not reproduced: the damage is characterised exactly, and eight deliberate attempts across two corpora -- including one on a fixture built to the affected index's own fragmentation profile -- did not produce it. What 3.1.0 adds is detection: `intent doctor` now looks for a document the index holds with no row in the content table, and runs SQLite's own FTS5 integrity check beside it. **Both read the index's segments, so they share a blind spot, and doctor says so in its output** rather than presenting them as two independent witnesses. It is recorded in [Known defects](../../known-defects.md) with the repair, and it is the one thing in this release we can describe and cannot yet explain.

**The bundled SQLite moves from 3.46.0 to 3.53.2** (`rusqlite` 0.32 to 0.40). The detector needs it for its second witness: SQLite 3.46.1 fixed false-positive integrity-check reports on FTS5 indexes in secure-delete mode, which is the mode Intent's search tables run in. That is the smaller reason. 3.46.0 carries SQLite's WAL-reset database corruption defect, fixed in 3.51.3, whose preconditions are a WAL-mode database written or checkpointed by more than one process at the same instant. That is how `intentd` and the CLI share a store, so every v3 install before this one ran on it.

## Added

**A symbol row says what it is, what it belongs to and its arity.** The index stored a definition or a reference, a name and a span, so nobody could ask for the structs named `Config` or the methods of a type. Every symbol now carries its `subkind` in its own language's vocabulary -- `function`, `method`, `struct`, `enum`, `variant`, `field`, `trait`, `macro` for Rust; `module`, `protocol`, `impl`, and the clause as it is actually spelled, `def`, `defp`, `defmacro`, `defguard`, `defdelegate`, for Elixir -- along with the container it is written in and that container's kind, the trait name on rows inside an `impl Trait for Type`, and its arity. **One row stands for one name node**, fixed where the duplicate was made rather than hidden by a `DISTINCT` at read, so a definition written once answers once. Rust and Elixir are read through Intent's own queries rather than those grammars' tags queries, which carry no kind, container or arity; Swift and Lua keep theirs and gain the typed kind and the de-duplication.

**`intent search --subkind <subkind>` and `--in <container>` narrow by what a symbol is and what it sits in, and with no query they are the whole question.** `intent search --subkind method --in AddressError` lists that type's methods with no text to search for. A container can be named whole or by its last segment. **`--subkind` is its own flag and not more words for `--kind`**: one word carrying two vocabularies meant `--kind def` read as every definition to one caller and as an Elixir public clause to another. An unknown subkind is refused with the roster of the languages in scope, and that roster is read off the index's own compiled queries rather than a list written beside them, so it cannot promise a word the extractor does not write.

**`intent index resolve` resolves references to the definitions they name, using each language's own toolchain.** Until now a reference was a name match, and nothing in Intent could say which `new` a line calls. Rust is read from rust-analyzer's SCIP export; Elixir by compiling the Mix project with the compiler's own tracer. **A resolved row always joins a written reference**, on path, line and name, so a reference only the toolchain sees is counted as unmatched and never stored. A run replaces exactly the files it joined and purges rows whose path has left the index, each file carrying the hash it was read at. **The toolchain builds under `intent/.cache/resolve/` and never the project's own build directory.**

**It runs on that verb and nothing else.** rust-analyzer's export runs the workspace's build scripts and proc macros, and no switch it has stops that; an Elixir compile runs the project's macros. So nothing starts it unasked -- not `intentd`, not a reconcile, not a hook -- and it is not offered on the MCP tool tier in this release.

**A resolved reference answers with the definition it points at, and `--target` asks for the references that point at one definition.** A hit whose key joins exactly one current resolved row carries the target with its path and line, and the row ends `-> <target>  <path>:<line>`; one that joins several lists them as candidates rather than picking, because the store keeps no tie-break and neither does the answer. A hit in a file that has changed since it was resolved keeps its syntax level. **Every answer carries the index's resolution state**, naming each language whose third level is not current and why -- its toolchain is not where Intent can run it, its last run failed, its files have changed since they were resolved, or nothing has been resolved here yet -- so a caller list is never a confident subset of a tree the toolchain has not read.

**Every search answer says when the index it read was last reconciled.** A reconcile stamps the store in the statement that performs it, by the database clock, and that reading rides in every envelope: the CLI's, the daemon's, the MCP tool's and the explorer's pane alike. This matters most for a daemon-served search, which does not reconcile before answering, so the envelope is the only place its age could come from.

**`intent index status` reports what the index costs, measured.** Each index family's bytes are read out of SQLite's own `dbstat` rather than estimated, and the whole store's from its page count. It also reports what each language's resolution holds, and **lists the stale paths rather than counting them** -- a reader deciding whether a caller list can be trusted needs to know whether the file in front of them is one of them.

**Intent's MCP server tells a client what its index answers, on the handshake.** A client that lists tools by name only, deferring their schemas, showed the model nothing about what they do -- so a session with the server loaded still reached for grep for a question the index answers better. The server now sends its instructions on initialize, from one line in the command register: how to route a code lookup by name, what those answers include, what they never resolve, and the selector a client needs to load the tool's schema before calling it.

**`intent daemon logs` prints the daemon's recent lines, and `--follow` keeps printing them.** The daemon writes two logs and no verb named them, so an operator asking what it had been doing had to find both files first. They print as one list merged by each line's UTC stamp, so an error older than the newest start line no longer sits below it and reads as though it came after. A log not yet written is named as absent rather than skipped in silence. Without `--follow` it exits, because a script or an agent that runs it must not hang.

**Intent.app has a Console, on Cmd-L, and its one-off verbs write into it.** Run Doctor used to discard its output, so a clean pass was indistinguishable from a click that did nothing. The Console tails the daemon's logs while its window is open and colours lines by the CLI's own tokens, and Run Doctor and the new Rebuild Search Index stream into it and bring it forward. A second one-off while one is running is refused with an alert naming the running command rather than queued.

**A project declares its own pre-commit guards, and Intent's hook runs them.** `.git/hooks/` is untracked in every repository, and so is the line that points git at a tracked hooks directory -- so a guard wired by hand belonged to one checkout, a fresh clone was gated by fewer guards than the original, and nothing anywhere said so. A `"guards"` array in `intent/.config/config.json` names each one as an argv, resolved from the repository root and never handed to a shell, with an optional path that makes a guard not applicable while nothing exists there. **The declaration is tracked, so a clone runs the set the checkout it came from ran.**

**`intent st detach <ID> <path>` removes an attachment from a thread, and says what happens to the file.** `intent st attach` had no inverse, so a document could leave a thread only by hand-editing canon. **The file on disk is yours**: when it is still there the output names it and says that a running `intentd`, or the next sync, carries an authored file under a thread back in as an attachment unless it is deleted.

**`intent wp show` lists the criteria scoped to a work package, and `intent wp gate` prints its close-gate verdict.** Neither question had an answer at the package: `wp show` printed its header alone and `wp gate` was not a command. Both read the package through one door, so the listing and the verdict cannot disagree about what is in scope.

**A standing directive is an item kind of its own, and only the hypervisor's board carries one.** The whiteboard protocol has always kept durable instructions under their own heading, read at pickup the way decisions are, and the model had no kind for them -- so a carry of that board could only report every directive as a line it would not carry, and carrying them as decisions would have lost the distinction the protocol draws: a decision records a call that was made, a directive is an instruction still in force.

**`intent wb register --correct` changes a registered node's name and role, and keeps its board.** A node registered with the wrong values could previously be repaired only by hand-editing a generated file or by deleting the row. It writes name and role and nothing else, so the node's board, its items and its messages stay attached, and it refuses a moniker that is not registered so it never creates a node.

**Every whiteboard verb records one event.** Board writes reached the event log nowhere, so the store could say what a row held and nothing about who put it there. Each verb now writes one event naming the acting node and its arguments, in the same transaction as the rows it writes.

**`intent sync` prints what this clone needs after a pull, and `intent sync --apply` does it.** After a pull, every verb answered from the store as it stood before the pull: a thread the pull brought answered `no steel thread` until someone ran the whole-store restore, which is the destructive direction. Bare `intent sync` now reads the store, the tree and git's status and prints the steps, writing nothing. In order, they are a branch behind its upstream, an id both sides minted, a canon file both sides changed, the ingest of the committed canon and history, the views that need regenerating, the index, and `doctor` last as the exit code. **Each step says what kind of step it is**: a quiet one runs, a reversible one asks and `--yes` answers it, and a non-reversible one, taking a side in a conflict, always asks a person. **Intent never pulls, commits or pushes**; its one write to git is staging the files it wrote to resolve a conflict.

**`intent claude upgrade --apply` wires three git hooks that run it for you.** `post-merge`, `post-checkout` and `post-rewrite` run `intent sync --apply` after a pull, a branch switch or a rebase. A hook has no terminal, so it runs only the quiet steps and names what it left, and it always exits 0, because a hook must never fail a checkout.

**`intent st renumber` and `intent issues renumber` repair an id two clones both minted.** Ids are minted highest-plus-one over the local canon, so two people mint the same `ST0001`, and git refuses the merge. The only repair was a hand-renamed file, a hand-edited id and a store restore. Each verb moves the record and everything that names it structurally, records its own event, and **lists the prose that names the old id without rewriting it**, because prose is authored. Mid-merge, `intent sync --apply` does the renumber itself.

**Every project act travels as its own committed file, so history crosses a clone.** The event log lived in each machine's store and nowhere else, so a clone got the present and none of how it was reached. Each act now writes one file under `intent/.canon/events/`, in the same write as the rows it records, naming its author. Acts that describe one machine, such as a heartbeat or a sync, stay on that machine. **`intent upgrade` writes, once, the files for the history a project's store already holds**, so what happened before this release travels too.

**[Working in a team](../../concepts/working-in-a-team.md) says how a project works across several clones**, written from a driven run of two clones of one origin. It covers what travels and what does not, what a reviewer reads in a pull request, the one command after a pull, a twice-minted id, and a CI job that runs `intent doctor` on the merge result, because a merge made on the forge passes through nobody's commit gate.

## Changed

**`intent wb migrate` refuses to carry a board that holds lines the model cannot hold, and names every one of them before it writes anything.** A sub-heading became an item whose text was a literal `###`, a markdown table became one item whose rows rendered as continuation lines, and a board's lead paragraph was reported and dropped -- and the run exited cleanly either way, so a script could not tell a complete carry from a lossy one, and the re-run that would have fixed it was refused as a second carry. Each of those is now one named unit, the exit status tells the two apart, and `--drop-uncarried` says in its closing line how many units it dropped. **Whichever way it goes the board's own markdown is kept byte for byte** under `.history/pre-migration/`, so a dropped line leaves the model and not the record.

**The search tool's description leads with the code lookups.** It opened on whole-project prose search and sent code lookups to grep, with definition, symbol, outline and callers appearing only in the parameter text -- so a model searching its own tools for where a function is defined did not find this one. `intent search --help` and the MCP tool's description are both rendered from that one row, so they cannot disagree.

**The generated `CLAUDE.md` tells a session to ask Intent's index before grep, and how to reach it.** The MCP server already told a client on initialize what the index answers, and a session that had read that text still reached for grep first. It is in `CLAUDE.md` alone and not in `AGENTS.md`, because the tool's name and the way to load it are Claude Code's and `AGENTS.md` is tool-agnostic. It offers no shell alternative on purpose, because a CLI line beside it invites Bash. **Which tool a session reaches for is the model's choice and varies between runs, so this changes the odds rather than guaranteeing an outcome.**

**The shipped symbol-context hook answers a Bash grep, and its answer reaches the model.** It matched only Claude Code's `Grep` tool while a session's searches run through Bash -- and it printed its answer as plain stdout, which that hook's output does not carry to the model at all, so it had been invisible even for the one tool it did match. A regex, several patterns or a command built by substitution append nothing, because the question is no longer one name. The shipped settings still do not wire it: turning it on stays the project's decision.

**Every line the daemon writes to its logs opens with an RFC 3339 UTC time, and every notice carrying a remedy opens with `warning:`.** There was no time on any line, so the only clock was the file's mtime and a deliberate restart could not be told from a crash and a respawn by anything but the wording of one line. `intentd --version` and `--help` stay unstamped, because they answer the person who typed them.

**`/projects` is a list inside the explorer, searched and driven like the others.** It was a screen of its own, with its own layout and its own keys and nothing to type into -- the one place where progressive search did not work.

**A thread's or an issue's view in the explorer splits in half, and the selected field renders as markdown below it.** Every other row used to be one clipped line, so a field longer than the line could be read only by opening an editor on it.

**A thread's `acceptance: exempt` is fixed when the thread is authored, and the close gate's refusal names the routes that exist.** The field was declared immutable while `intent set` wrote it happily -- two answers to one question -- and the gate's refusal for a thread whose contract had been emptied told the operator to declare it. The refusals now name what a verb can actually do.

**`intent doctor` shows a store that lags the committed canon on a default run.** After a pull it was reported only under `--verbose`, so a default run printed `0 finding(s)` over a store that could not find a thread the pull had brought. It is shown and not counted, so the exit code is unchanged, and its remedy names `intent sync --apply`.

## Fixed

**`intent search` counts the rows the answer carries.** Both denominators counted hits the body did not hold, so the answer said more had come back than it carried: the code corpus was searched whatever `--tier` said, counted, and then dropped along with the group that held it.

**A search tier that cannot answer says so, and counts nothing.** Asking for a tier a project cannot serve counted hits it could not return and printed a note about withheld rows above an empty answer. The envelope now carries one entry per tier asked for by name that this project cannot serve, with the reason.

**An outline lists what a file defines, and a definition answers once.** An outline listed every call site in the file beside the definitions, and every definition twice -- so the prior-art check this project's own canon prescribes answered twice for a name defined once, and read as a Highlander violation that was not there.

**A text hit prints the line that matched.** The prose form printed the hit's name while the matching line was carried only in `--json`, so a search showed a reader where a hit was and hid what it had matched.

**A word inside a CamelCase or snake_case name finds the file that defines it.** Searching for a part of a compound identifier missed the definition that contains it.

**A reference inside a macro invocation is a reference**, and a reference says the path or the module the source wrote. A qualifier is stored as written and never resolved, and every surface says so, including what each language's references still leave out.

**A commit no longer costs the daemon a walk of the project's tree per event**, and a large indexed directory vanishing costs a refresh one statement per table rather than one per file.

**`intent upgrade` reports no whiteboard file as owed by a build.** The whiteboard stayed a member of the set of things the model claims and no build carries, after the build that carries it had landed -- so every project with a board got the not-yet-carried line, and the list grew with each migration because the views a carry writes were counted too.

**A URL handed to a verb that takes a steel-thread id names the whole address you typed.** The id reader split its argument at the first `/`, so the refusal quoted a fragment nobody had typed.

**`intent critic`'s own usage errors exit with the code its gate fails open on**, and the shipped rules' greppable proxies no longer fire on the form the rule prescribes. The pre-commit gate no longer lints a rule library's own bad examples -- which is a pair of files that could not otherwise be committed at all.

**A verdict has to fit the row it is recorded on, and a green acceptance test is reachable only from red.** The red-before-green edge was enforced by the verdict verbs and not by the create.

**`intent doctor` reports a registered-but-unmigrated board as an advisory naming the verb, and never as view skew.** Skew is what the gate refuses, and no sync could clear it, because nothing writes over that markdown.

**A view-skew finding whose difference is one a formatter makes says so**, and `intent claude upgrade --apply` gives a project the formatter exclusion it is missing.

**The canon-ignore guard judges the ignore rules the commit carries, not the checkout's.** A rule staged and then edited again was compared across two documents, and a staged rule reaching canon that the worktree lacked was never matched at all. **A scratch tree it cannot build refuses rather than passing**: nothing was checked, so nothing is allowed.

**The whiteboard header guard refuses a header value detached from its key.** The header block is one line per key, so a value a formatter has broken onto its own line reads as empty -- a node claiming work packages reads as claiming none, and nothing about the board looks wrong. The guard names the limit it read from the repository's own formatter configuration, and says when rejoining is not the fix because the next commit would break the line again.

**Intent.app no longer hangs on a command that fills its stderr.**

**A store that will not open is a finding.** `doctor` passed over a store file that exists and will not open, while every other verb refused on it and sent the operator to `doctor`.

**A write no longer creates `intent/.canon/project.json` when there is nothing to record in it.** On a branch that never committed it, the file was created untracked and the next merge refused to overwrite it, and the new `post-checkout` hook made that happen on every branch switch.

**`intent upgrade` writes no `intent/events.jsonl`**, and its list of files naming a v2 path no longer lists the records Intent itself writes.

The full list is in the [CHANGELOG](../../../CHANGELOG.md).

## Removed

**`intent claude rules index` is retired, and the family is a read.** v3 embeds the rule library in the binary, so the verb had no installation to regenerate an index in -- and the index it wrote, with its generator and its template, was wrong as well as unread. The spelling is refused as an unknown subcommand rather than as a declared verb that is not built.

**`intent at new` takes no `--status`: a new acceptance test starts at its kind's entry.** A create that can name a status can name green, and green is reachable only from red, so the flag minted a passing row nobody had seen fail.

**Artefacts nothing in v3 reads are deleted**: two hook templates that were in no guard roster, dead templates that `init` carried embedded while declaring it writes none of them, and a subagent manifest no v3 code read. No command behaves differently; what goes is weight in the install and rows that had to be explained. **A project that wired one of those hook templates by hand out of the install path will find it gone.**

## Upgrading

```
  $ intent backup                    # with v3.0.3, BEFORE upgrading -- see below
  $ brew upgrade matthewsinclair/intent/intent
  $ intent --version
  $ intent index status
```

**Your runtime store is migrated on first open, and the upgrade is a ONE-WAY DOOR.** v3.0.3 wrote schema version 26 and this release speaks 29. The first command to touch a project migrates its store in place, through every step between, without asking -- and **nothing migrates it back.** A store written by a newer `intent` than the one you are running is refused outright at open, with the remedy stated as _upgrade intent rather than migrating the store down_. Forward is implemented; backward is not, and no restore verb ships.

**So take the snapshot with v3.0.3 first, before you upgrade.** `intent backup` writes the store at whatever schema it holds at that moment, so once the new build has opened the project it can no longer produce a snapshot of the old schema. Copying that file back over `intent/.cache/intent.db` by hand is the only way back that always works, and anything written after the snapshot is lost with it. A snapshot is same-schema rollback and never the recovery path for a store an upgraded binary refuses.

**Every step between the two versions is the search index's own tables, and the ladder is purely additive.** Two of the three create tables and nothing else; the third rebuilds two derived index tables and carries their rows forward. **None of them reads or rewrites a single entity row** -- no thread, work package, criterion, test, issue, attachment, board row or event. So the migration's risk is confined to a cache the tool can rebuild from your tree. **Your project's files are not touched by this.**

**The first command that reconciles will re-extract every code file the index had already read.** The typed symbol rows carried forward by the migration arrive without their new fields, by design, and the reconcile re-extracts the file rather than mixing two shapes in one answer. Nothing is lost by it -- the rows are rewritten from the files on disk -- but that first reconcile after upgrading is the expensive one, and on a large tree it is noticeably so.

**The third index level starts empty, and it stays empty until you ask for it.** `intent index resolve` is the only thing that populates it, it runs your project's build to do so, and until it has run every answer will say that language's resolution is unresolved and name the verb. That is the intended resting state for a project that does not want its build run by a search: nothing is broken and nothing needs clearing.

**If you run more than one v3 install, the first v3.1.0 command to touch a shared project ends the older install's access to it** -- an older machine, a colleague who has not upgraded, a pinned CI image. The refusal names the store and both version numbers, which is enough to diagnose, and does not say that an upgrade elsewhere caused it.

**A v3.0.3 `intentd` still running after the upgrade will be refused by the migrated store**, because the daemon opens through the same door. Restart it with the new build: `intent daemon restart`.

**Upgrading changes nothing about a hand-authored whiteboard.** Nothing migrates a board on its own and nothing renders over one. What is new is that `intent doctor` now says, per node, that the board is registered and not migrated and names `intent wb migrate <node>` -- and **that finding blocks no commit**, where the skew report it replaces did. A board write on an unmigrated node is still refused, with the reason: a write renders the board from the store, so writing before the carry would replace the markdown with a render of a board that holds none of it.

**If you do carry a board, the carry is now refusable and the refusal is the useful outcome.** It names every line the model cannot hold, with its file and line, before writing anything. Read that list before reaching for `--drop-uncarried`; the markdown is kept either way, but a line that leaves the model leaves the board's live surface.

**Run `intent upgrade` in each project, and commit what it writes.** It writes an event file for every project act the store holds, so the project's history travels with it, and it says how many. On a project with a long history that is a large commit, made once. It also removes the `intent/events.jsonl` ignore line earlier v3 versions wrote.

**Reinstall each project's pre-commit gate to pick up the new guards:** `intent claude upgrade --apply --skip-settings`. The carrier at `.git/hooks/pre-commit.intent` is copied into the project, so its text arrives only at that run, while the gate body and Intent's own guards are read live out of the resolved install and are in effect as soon as the install moves. `--skip-settings` leaves `.claude/settings.json` and `.mcp.json` as they are. The same run wires the three git hooks that run `intent sync --apply` after a pull. `.git/hooks` is not cloned, so every clone runs it once.

**Intent's own guard roster is unchanged.** What is new is that a project may declare its own guards in `intent/.config/config.json`, and that the declaration is strict on purpose: a guard whose body is missing, not executable or untracked blocks the commit, as does a `guards` array that cannot be read. That is deliberately stricter than a missing roster guard, which is an install behind its roster, where this is a broken tree. `intent doctor` reports what a clone will not receive as advisories it does not count.

**If your gate's refusals ever told you to reinstall Intent, they now name `intent bootstrap`.** Reinstalling writes no home pointer, so the old remedy could not be followed.

**Nothing about `.claude/settings.json` changed, and no Claude Code hook name was added or removed.** The two shipped-and-off hooks remain shipped and off; a project opts in from its own `.claude/settings.local.json`.

**The bundled SQLite is 3.53.2, and an existing store's search index opens as it stands.** FTS5 has written two on-disk index versions, 4 and 5, and both 3.46.0 and 3.53.2 accept both, so nothing about the index format moves with the engine. If you build Intent from source: `rusqlite` is built with its `fallible_uint` feature, because 0.40 carries the `u64` conversion the store relies on only behind that feature. With it on, a `u64` converts through `i64::try_from` and fails loudly past `i64::MAX`, exactly as 0.32 did by default.

**One defect to know about, and `intent doctor` now tells you when you have it.** If a search fails on a term you can see in your tree, with an error about the virtual table's content being corrupt, run `intent index rebuild` -- it repairs the index in one pass and the entities were never affected. `intent doctor` reports this state as an advisory: it is shown, it is not counted as a finding, and it does not change doctor's exit code, so a script that gates on doctor behaves as it did. It is in [Known defects](../../known-defects.md).
