# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.2.1] - in progress

### Added

- **`intent outstanding`, alias `intent outs`, shows everything open and in progress in one table** (ST0079). A snapshot of what was open took three commands -- `intent st list`, `intent issues`, and `intent wp list` once per thread -- and now takes one: a row for each thread bare `intent st list` shows, each followed directly by its work packages in progress, then a row for each issue bare `intent issues` shows, with the kind (ST, WP or Issue) in the leftmost column, then ID, status and title. A work package in progress under a thread `st list` does not show brings that thread in as its parent, with the thread's own status, so a package is never listed apart from its thread. `--show` takes a comma-separated list of `st`, `wp`, `is` (or `issue`, `issues`) and `all`, the default. The table renders through the list verbs' shared output layer (`--format terminal|md`, `--width`, `--markdown`), and a closing line counts each kind shown against how many exist, so an empty result reads as none of N rather than as missing data; a thread shown only as a package's parent is counted apart from the outstanding threads. It is offered on MCP as a read. **What counts as open is one definition**: `st list` and `issues` now read their bare defaults from the same place, on the CLI and on MCP, so the three verbs cannot disagree about it.
- **The explorer's `/outstanding` shows the table `intent outs` prints, as a view** (ST0079 WP-01). `/outs` reaches it too, because it is how the name starts. The rows are the verb's own, read by the same facade call and laid out in the order it returns them: the kind leftmost, then ID, status and title. Enter on a row opens that thread, work package or issue, and Backspace returns to the table. The verb's counts line sits at the foot, and when nothing is outstanding it is the whole view. An argument after the command is refused on the info row. The palette's resting list grows by one entry, and the omnibox's dropdown with it, so every one of the explorer's own commands is still offered at rest. A work package's row now carries its thread and sequence as fields (`wp`), in the MCP tool's rows too, so no face has to read them back out of the rendered ID.
- **`intent wb edit <kind> <id> <text> --node <moniker>` changes the text of one of your own board items, live or archived, and keeps the old text out of any commit that does not already hold it** (issue 0523). No verb could change an item's text, so text that must not be committed, such as a client's name, stayed on the board and in the event file that recorded it. Where no commit holds that event yet, the edit amends it in place, keeping its id and stamp. Where a commit holds it, a `wb.edit` event records the new text, and git history keeps the old one; rewriting that history is the repository's business, never the verb's. The answer names every file under `intent/` that HEAD already carries the old text in, found by reading HEAD rather than assumed, so a board committed without its event file, or a peer's committed item quoting the text, is reported rather than missed. It also names every file the next commit would carry the old text in (staged) or could carry it in (unstaged or untracked), and it says no file under `intent/` holds the old text, at HEAD or in the next commit, only when both searches found nothing: a copy `wb migrate` kept, or another item saying the same thing, is named rather than assumed away. When the new text contains the old text, the answer says so, since every file carrying the new text then carries the old text too. A board or event file staged before the edit is named too, because a plain `git commit` carries what the index holds rather than what is on disk. `intent wb edit message <anchor> <text> --to <recipient> --node <moniker>` does the same for a message you sent, handled or live, found by its recipient and the stamp its inbox heading shows, with `<anchor>#<n>` picking one of several sent in one minute; an announce's copies change together, and the answer names each recipient. Like every `wb` verb that writes text, it is one-way and is not offered on MCP.
- **An edited board item or message renders `(edited)`** (issue 0525): at the end of an item's first line; on a message, after `(handled)` in its inbox heading, and after the route in `wb show` and `wb pickup`. It says the text changed and never what it said: an edit of an uncommitted draft leaves no trace in the event log by design, so the mark is how a peer who acted on the text can tell. `board.json` carries the stamp as `edited_at`, omitted when there is none, and schema rung 30 adds the column to both tables. A restore also updates an edited message in place, keeping its id and so its `#<n>` in its minute, where it used to remove and re-add it (issue 0529).
- **A whiteboard board can claim an issue, as `ISSUE:<NNNN>`** (issues 0511 and 0519). `intent wb claim` admitted only a thread (`ST0079`) or a work package (`ST0079/01`), so work on an issue could not be claimed, and two sessions working one issue could not see each other on their boards. The form is the one `intent organize` already writes into `intent/.intentfiles`; a bare `0512` and free text are refused. `intent issues renumber` now rewrites a board's claim to the issue's new number, as `intent st renumber` does for a thread, so a renumbered issue leaves no board claiming a number that moved. The sentence naming the claim forms has one home, and `wb claim`'s refusal and `wb migrate`'s reason for a claim it cannot carry both print it from there.
- **The pre-commit gate checks staged bytes against the formatters a project declares, and writes nothing** (issue 0505). A project that lists its formatters in `intent/.config/config.json` (any of `markdown`, `elixir` and `rust`, eg `"formatters": ["markdown", "rust"]`) gets a shipped guard that runs each formatter's check over the STAGED bytes of every matching file. It refuses the commit by listing the files and the command that formats them, and it rewrites nothing. A formatter name it does not know is refused too, rather than passed over. Estates had each hand-wired a formatter into their own hook, and a hook that formats the worktree and re-stages it commits bytes nobody staged. `intent/docs/pre-commit-hook.md` describes the declaration. The whiteboard header guard's refusal stops saying that a formatter runs before it.
- **`intent bootstrap --check` says where this machine's pre-commit gate resolves and whether it can run, and writes nothing** (issue 0533). Nothing in Intent answered where the gate's install root resolves, so a tool that needed to know copied the project shim's own resolution to check it. `--check` prints the lines the shim's `--where` prints, `pointer:`, `root:`, `state:` and `gate:`, with the same states, and exits as it does: 0 when the gate can run and 1 when it cannot. It needs no project and no installed carrier. When the gate runs from a different install from the binary asking, it names both roots. When the pointer names a versioned Homebrew keg, which the next `brew upgrade` deletes (issue 0527), it says so and still exits 0. `intent info` prints the same answer on a new `Gate root:` line, and its `INTENT_HOME:` line and its exit code are unchanged. `intent claude upgrade` takes its pointer warnings and its divergence note from the same answer, and they read as before.

### Fixed

- **A `brew upgrade` no longer leaves every gated commit refused** (issue 0527). `intent bootstrap` recorded the install root it resolved, which on a Homebrew install is the versioned keg, `<prefix>/Cellar/intent/<version>/libexec`. `brew upgrade` deletes that keg in its cleanup, so the pointer named nothing and the project shim refused every commit in a gated project until `intent bootstrap` was run again. It now records `<prefix>/opt/intent/libexec`, Homebrew's link to whichever version is current, whenever that link resolves to the same keg, so the pointer survives every upgrade; any other root, a source checkout included, is recorded as before. A machine upgrading from 3.2.0 or earlier still holds the old pointer, and the formula's caveat now tells it to run `intent bootstrap` once. `intent claude upgrade` compares the pointer with the running install by what the two paths resolve to, so the `opt` path and its keg no longer read as two installs. intentd's LaunchAgent was driven on a brew-shaped layout rather than assumed: it records the `bin` link it was started through, which an upgrade does not delete.
- **`intent/todo.md` opens with its generator marker again, so a tool that keeps its own todo.md can tell Intent's view apart** (issue 0528). v2's `intent todo` opened the view with frontmatter naming it, `generator: intent todo`, and v3 dropped it at the port. Utilz's `todo` refuses to overwrite a todo.md whose frontmatter names another generator and takes one with none for its own, so, run beside Intent's view, it rewrote the view. The view opens with v2's frontmatter again, byte for byte, and `intent todo` prints it too, because the terminal view and the file are one generator. A todo view an older v3 wrote reads in `intent doctor` as a stale render, which is advisory and blocks nothing, until `intent todo update` rewrites that one view, as the next command that writes it also does; a marker naming any other generator is still skew.
- **A multi-line board item stays one item: in its board view, and under `intent wb show` and `intent wb pickup`** (issue 0532). A board view printed an item's lines after its first at column 0, so they left the list item: a sub-bullet read as a sibling item, and an item holding a fenced block could turn every section below it into code. They are now set in under the item's `- `, and `wb show` and `wb pickup` print them set in under the item's own line; a one-line item prints as it did. A board an older Intent wrote in the old shape is reported by `intent doctor` as a stale render, which blocks no commit, until the node's next board write or `intent sync --to-disk` re-renders it.
- **The explorer (`intent explore`) shows what changed under it** (issue 0520). It read every view from the model as it stood when it opened, so a write from another terminal, from another node or from intentd's ingest of a hand edit never reached it, not even on leaving a view and coming back. Neither did its own `/` commands, whose `ok` sat over a list that had not changed. An edit to a record somebody else had changed was refused -- `changed while this command was running -- nothing was written` -- every time until the explorer was restarted. Nothing was lost: that refusal is the store's own check, and it still judges an edit against the record the edit started from. The explorer now catches up with the store before it reads anything, and while nothing is being edited it re-reads the view on screen within half a second of a change, keeping the cursor on the row it was on. The search pane re-runs when it is entered, as before, because its freshness line says how current its answer is. It follows the store: a file edited by hand under `intent/` shows once intentd's ingest takes it, or, with no daemon running, once `intent sync --to-store` brings it in. That is where every other verb sees such an edit too.
- **`intent st done --date` and `intent st cancel --date` answer a stated date on a thread that is already closed** (issue 0503). The self-loop returned `ok:` before the date was so much as read, so a stated date, and a malformed one with it, was dropped at rc 0 by the verb whose flag exists to record it. A malformed date is now refused exactly as it is on the close. The date already on record is still `ok: already Completed`, because a self-loop means nothing to do. A date that differs from the record, or one stated on a thread that records none, is refused naming what is on record, with `intent set <ST> completed <date>` as the remedy. The self-loop still never re-runs the gate, which is what it was written for.
- **`intent set <ST> completed` refuses what the closing verbs refuse** (issue 0504). The setter re-parsed the field as the string the model declares, so `not-a-date` reached canon at rc 0, and so did a completion date on a thread in Triage. A thread's completion date now passes one rule wherever it is written: ISO 8601 `YYYY-MM-DD`, and only on a thread that is Completed or Cancelled. Clearing the field still repairs a stray date on a thread that closed nothing, and is refused on one that closed, which is the NULL-completed row this door was built to repair.
- **The explorer's hint line no longer offers `⏎ edit` on a row where Enter does nothing** (issue 0502). Enter on a `label` row is a declared no-op. That covers every row of `/help`, and the counts line that ends `/outstanding`. The hint still called Enter an edit on these rows, because the label kind had no arm of its own. It now names no Enter verb there, as it already did for a button with nowhere to go.
- **The shell critic reaches a script by its shebang, and a clean verdict counts only the files a rule was asked of** (issue 0536). Each shell rule selects files by its `applies_to` globs, so a script with no extension was asked nothing unless it sat directly inside a directory named `bin`, which left out the `.githooks/` hooks and every script nested under `bin/.devbin/`. The run still counted such a file and printed `ok ... across 1 file(s)`. A file whose shebang names sh, bash or zsh is now tested against each rule's own globs as though it carried that shell's extension, so every rule keeps the dialect it declares. The verdict counts only the files some rule was asked of and names the rest, and where nothing was asked it says `no shell rule was asked of any of the N file(s) given` instead of `ok`. The three commands in `bin/.devbin/cmd/macos` that split its fixed `SUPPORT_PATHS` list on purpose now say so with a `shellcheck disable`.
- **`intent critic --staged` judges the bytes being committed** (issue 0537). It listed the staged paths from the index and then read each file from the work tree, so after `git add -p`, or an edit made after staging, the gate judged bytes other than the ones in the commit: a staged violation under a clean work tree passed, a clean stage under a dirty work tree was refused, and a staged file missing from the work tree stopped the run. Each staged file is now judged by the blob the index holds for it, in the index `GIT_INDEX_FILE` names, and shellcheck reads a copy of that blob under the file's own name. A file renamed and edited in the commit is judged at its new path, where rename detection used to drop it unexamined. `--files` still reads the named files from disk.
- **`intent wb migrate` labels its `.history` count by what it counts, and names the copies that hold what it dropped** (issue 0499). The closing line said `N snapshot(s)` over every `.history` document the carry took in, fold archives included, while the refusal's remedy calls the pre-migration copies snapshots. So a node with one copy could read `29 snapshot(s)`. The line now reads `N .history document(s) carried`, and each pre-migration copy the carry keeps (the board always, and each inbox holding a unit the model cannot carry) is named on its own `kept:` line. A dropped unit is in one of those files.
- **`intent sync --to-disk` runs beside an intentd that is watching the project** (issue 0500). It was refused with the reason that `sync` would run a second engine against the daemon's, and the remedy was to stop the daemon. That reason holds for `sync --to-store`, which replaces the store from the extract and would race the daemon's ingest, and it still refuses. `--to-disk` only writes the extract from the store, as every other writing verb does beside the daemon, and it costs the daemon no ingest.
- **`intent wb register`'s refusals print the whole command that works** (issue 0522). A moniker without both `--name` and `--role`, the flags without a moniker, and `--correct` without all three were each refused with a sentence about the flags and no remedy, so the form had to be found somewhere else. Each refusal now prints `intent wb register <moniker> --name "<display name>" --role <role>` with the moniker filled in where one was typed, and names `intent wb status` for the nodes that exist. The flags without a moniker also say that the moniker comes first, as a positional. Every other remedy that sends someone to register a node prints the same form, and `intent wb register --help` now says what the moniker, `--name` and `--role` are, each with an example.
- **`intent wb show --all` and `intent wb pickup --all` say which messages are handled** (issue 0531). They list every message, handled ones included, and printed a handled message's line exactly like a live one's, so the one listing meant to show handled messages could not say which they were. A handled message's line now carries `(handled)`, after any `(fyi)` and before `(edited)`, the order its inbox heading uses.
- **`intent --version` and `intentd --version` say whether the build is a release or a dev build** (issue 0534). A brew-installed release and a dev-tree build printed the same shape, `intent 3.2.0 (<sha>)`, so telling them apart meant knowing which commit a tag names. The line now ends in `release` for a clean build of a commit tagged `v<version>`, and in `dev` for every other build, `dirty-` and `unknown` included: `intent 3.2.1 (<sha>) release`. The commit in the parentheses and the `[intent-source-commit:]` marker are unchanged, so nothing that reads them moves. The app's menu row keeps the word, and a binary on disk carries it as `[intent-source-kind:]`.
- **`intent st done` and `intent wp done` name every step out of an empty contract, and a refused close's `remedy:` line is the refusal's own** (issue 0526). A unit whose thread has no acceptance criteria, or whose criteria are all descoped or withdrawn, is refused, and the refusal named `intent ac new`, or the verbs that bring a criterion back, and stopped there. A criterion is not satisfied by being added or brought back, so following it led to a second refusal. It now names the step after: satisfy the criterion, a non-test one with `intent ac satisfy --evidence <ref> <STID> <ACID>` and a test-backed one by covering it with `intent at new` and taking that test to red, then green, with `intent at red` and `intent at green`, or cancel the unit. `intent ac gate` and `intent ac status` print the same line. The `remedy:` line under every refused close read "satisfy or formally descope the remaining criteria, then close again", which under an empty contract named criteria that did not exist. It now comes from the reason the gate refused, and keeps that text only where criteria remain unsatisfied.
- **A guard that does not apply to a commit is counted as skipped, not as run** (issue 0506). The guard runner read two answers from a guard it had dispatched, pass or block, so a guard that can only tell whether it applies once it runs, such as the staged-format guard in a project that declares no formatters, printed a sentence and was counted among the guards that ran. A guard now answers `3` for not applicable: it is counted under `skipped` and never blocks, and a project's own guards can give the same answer.
- **`intent organize`'s digest of unclaimed paths no longer changes when the project is moved or cloned elsewhere** (issue 0509). It printed the unclaimed paths relative to the project, but the digest beside their count was taken over their absolute paths, so the same set of files gave a different digest in a different checkout. The digest is taken over the project-relative paths.
- **`intent wb claim --help` and `intent wb unclaim --help` say what an `<id>` can be** (issue 0530). Since 0511 a board can claim an issue, and the forms were named only by the refusal a wrong one got: `--help` printed a bare `<ID>`. It now names a thread as `ST0000`, a work package as `ST0000/01`, or an issue as `ISSUE:0000`, the sentence the refusal and the MCP tool's description print, and the whiteboard skill, which still said a claim takes a thread or a work package, names all three.
- **The pre-commit gate says how many critic rules the project has disabled** (issue 0510). The critic prints that count so that a project which has switched rules off cannot read as a clean pass, and the gate discarded the critic's output whenever it passed, so the count never reached the one check every commit goes through. The gate's summary line now ends `<N> rule(s) disabled by this project.`
- **The pre-commit chain block refuses a commit it cannot gate, rather than letting it through in silence** (issue 0538). The block `intent claude upgrade` writes into each hook it wires ran that hook's `<hook>.intent` carrier when the carrier was executable, and otherwise did nothing. So with the carrier absent, not executable, or unreachable because `git rev-parse` failed, the commit went ahead with no guard, no critic and no doctor, and nothing said so. The carrier is gitignored, so every clone has the block without it, and so does every worktree of a project whose hooks are reached through `core.hooksPath`. The pre-commit block now refuses there and names the path and the remedy. The post-merge, post-checkout and post-rewrite blocks say so on stderr and still exit 0: git has already changed the tree, and a failing post-checkout makes `git checkout` and `git worktree add` report failure. `intent claude upgrade --apply` now brings a block already in a hook to this form in place, keeping every line outside it. Before, it left any existing block as it was, so no project already wired would have received the change. Intent's own hooks carry the same block.
- **The MCP `organize` tool reports project-relative paths** (issue 0513). Its JSON fields are named for paths relative to the project, and it filled them with absolute ones, unlike the CLI and the MCP `st hydrate` and `st dehydrate` tools.
- **`intent edit` on an issue's address names the verbs that read and change an issue** (issue 0514). Asked for `intent edit intent:///issues/<NNNN> --path`, the refusal said an issue's only file is its generated view, and its remedy then said an issue has no files of its own and to address a steel thread instead. The remedy now names `intent issues show <NNNN>` to read the issue and `intent set intent:///issues/<NNNN> body --from <file>` to change its text.
- **`--list-guards` lists a guard that decides at run time whether it applies** (issue 0515). The guard runner's listing judged a guard applicable by its `when` path alone, which cannot answer for a guard like the staged-format guard: every project has the config file it reads. The runner's roster now marks such a guard, and `--list-guards` reports it as `self-classifying` without running it.
- **intentd ingests an edit made outside Intent even when the file events for it are dropped** (issue 0516). Under heavy file activity elsewhere on the volume, the file events for a watched project could be lost before intentd saw them, and a hand edit under `intent/` was then never ingested. Each watched project now has a backstop that compares the canon files with what intentd last read every 30 seconds and ingests only what changed, so an edit the events missed is ingested within about 30 seconds.

## [3.2.0] - 2026-09-21

**v3.2.0 makes a thread's related links writable, and makes `intent doctor` say what it used to stay silent about.** `intent st relate` and `intent st unrelate` are the first doors to a thread's `related` list, which until now could be repaired only by hand-editing canon; `doctor` gains two advisories, one for a whiteboard row the store holds and its board file does not, and one for a root file that has fallen behind the installed Intent's templates. Beside that is a sweep of the fixes that landed after the 3.1.0 tag, from `intent index rebuild` repairing the table it used to refuse behind to a fresh `git worktree add` no longer building a whole store. **There is no store migration: the schema is unchanged since 3.1.0 -- read the [release notes](docs/releases/3.2.0/RELEASE_NOTES.md) for the per-project step that brings the new hook carriers in.**

### Added

- **`intent st relate <ID> <TARGET> [--note <text>]` and `intent st unrelate <ID> <TARGET>` write a thread's `related` links, and `intent set` names them.** The list was readable, and writable by nothing: `set` hands every field a string and the list wants a sequence, so every value was refused, and the refusal sent the operator to a lifecycle verb or a member address, neither of which exists for this field. The only repair for a link naming a thread that had been adopted under a new id was a hand edit of the thread's canon file. `relate` refuses a target the project does not carry and a thread named as its own target; a link is a value, so relating a linked target replaces its note, a missing `--note` included, and the same note again writes nothing. `unrelate` drops a link whether or not its target still exists. Repointing a link is the drop and then the link. Each act writes the store, canon and the realised `info.md` under one `st.relate` or `st.unrelate` event, and both are offered on MCP. `set` now refuses `related` by name and names the two verbs, which also closes the daemon's field-write door to the list.

- **`intent doctor` reports a root file behind the installed Intent's templates** (issue 0496). `intent claude upgrade --apply` writes `AGENTS.md`, `CLAUDE.md`, `.claude/settings.json` and the git hooks' chain blocks and carriers from templates, and nothing reported one that had fallen behind: a `CLAUDE.md` a template change had left stale read `0 finding(s)`. Each such file that is present and would be rewritten is now named in a `root-file-behind` advisory, with the `claude upgrade` spelling that rewrites it. **It is shown and not counted**, because a newer Intent reads every project behind until the upgrade runs there. A file never installed is not reported, nor is one that is the project's own, and the pre-commit carrier stays with the gate check that already reports it.

### Changed

- **The `CLAUDE.md` that `intent claude upgrade --apply` writes describes the verb as 3.1.0 built it** (the doc sweep, e12e071d1). It now says that `--skip-settings` also skips `.mcp.json`, that a `settings.json` with no `intent claude hook` in it is held rather than overwritten, and that `intent claude skills sync` reaches `~/.claude/skills`. A project's rendered `CLAUDE.md` picks this up at its next `claude upgrade --apply`.

### Fixed

- **`intent doctor` reports a whiteboard row the store holds and its `board.json` does not** (issue 0495). Its store-versus-canon check rebuilt threads and issues and compared nothing on a board, so a board write whose render was refused left the row in the store and out of the file, and `doctor` said nothing while the same failure on a thread was reported. Each migrated node's board is now compared between the store and `board.json` by the keyed comparison a restore already applies, and a node that differs is named in the same `store-stale` advisory, with the door that lands a board's view. **It stays an advisory and is not counted in the verdict**, as the thread arm is, because on a shared tree it is also the normal state during another node's board write. A node not yet migrated is left out, since its markdown is still the board.

- **`intent index rebuild` repairs an unreadable search-index table in place** (issue 0453). Every door opened the store through the damaged table, so the verb refused behind the damage it exists to repair. It now reads every other table first, refusing by name before any write if one of those is unreadable, then drops and recreates the two index tables from the store's own schema in one transaction. A store that cannot be opened at all refuses as before.

- **The append-only guard protects `intent/.canon/events/`, not `intent/events.jsonl`** (issue 0458). No verb has written `events.jsonl` since 3.1.0 moved the log to one committed file per event. An edited or deleted event file is now refused at commit, with `git restore --staged --worktree --source=HEAD -- <path>` as the remedy.

- **An authored `info.md` below a view's depth is an attachment that can be named** (issue 0461). An address refused a view's file name at any depth, so an author's `info.md` deeper than `WP/<nn>/` could not be reached. A view name is now refused only at a view's own depth: the thread root and directly under `WP/<nn>/`. A v2 file whose name still cannot be carried is withheld with the rename-and-`intent st attach` remedy.

- **`intent claude rules index` is refused as retired, at exit 2, like every other retired spelling.** 3.1.0 answered it with clap's `unrecognized subcommand` at exit 1 -- which the 3.0.3 entry that retired it describes -- while `claude ws`, withdrawn the same way, got the retired refusal. It now has a row of its own in the register and prints "`intent claude rules index` was retired in Intent v3 and is not a command in this build".

- **`intent init` lists its config file relative to the project, like every other path in its report.** It printed `intent/.config/config.json` as the one absolute path in the list.

- **The advisory critic hook hands its findings to the model** (issue 0478). `post-tool-advisory.sh` printed them to plain stdout, which a PostToolUse hook's model never receives. It now emits them as `additionalContext`, as the symbol-context hook does. The hook is opt-in and wired by neither shipped `settings.json`.

- **`intent daemon start` refuses a socket path the platform cannot bind, before it spawns `intentd`** (issue 0479). Under a long state directory it said only that `intentd` "was started and is not answering", and the cause was in the daemon's log. The refusal names the path, its length, the platform's limit, and the shorter `XDG_RUNTIME_DIR` or `XDG_STATE_HOME` to set.

- **`intent at na`'s refusal on a test row names a flag that exists** (issue 0480). Its remedy said to recreate the row with `intent at new` "with no `--status`", a flag `at new` no longer accepts. It now says the row starts at its kind's entry, `n/a` with `--kind non-test`.

- **`git worktree add` no longer builds a whole store in the new tree** (issue 0483). The post-checkout carrier ran `intent sync --apply` on a fresh checkout, which took 322s and wrote a 75 MB store on a large estate. It now skips a fresh checkout and prints one line saying the first `intent` verb builds the store. A branch switch inside a checkout still syncs.

- **A search answer says whether it reconciled the index before it answered** (issue 0484). A search skips its reconcile beside a watching daemon, under `--no-reconcile` and through `--daemon`, and its `complete: true` could not be told from one over a tree just walked. The envelope carries `reconciled`, and the terminal prints one line on stderr when the answer did not reconcile first.

- **`intent upgrade` keeps a project's flushed DONE list flushed** (issue 0485). A re-run on a v3 project rendered `todo.md` with every finished thread back in DONE, and `doctor` then reported the upgrade's own output as a hand edit. The upgrade now reads the watermark from the project's canon and carries it into the store it rebuilds, and an unreadable `project.json` refuses rather than rendering without it.

- **A board write whose render failed names the door that lands a board** (issue 0487). The warning said to run `intent st sync`, which rewrites a thread's views and cannot land a board's. It now names `intent wb touch --node <you>`, and says that neither `intent organize` nor `intent st sync` does it.

- **`intent wb migrate` carries a numbered list one item per line, and marks a unit that reads as state for a person to read** (issues 0488, 0489). A `1.` list was carried as one item, so a four-item TODO became a single row. The report now prints what each section yielded, zeros included, and puts a `READ THIS:` line on any uncarried or coerced unit that looks state-bearing. The mark decides nothing.

- **`intent st attach` refuses a name no address can reach** (issue 0490). It skipped the naming gate that `intent set` runs, so `st attach ST0001 todo.md` answered ok while the address for the same file refused it as a view. Both doors now ask the same gate.

- **A test run in a scratch tree cannot silently take over the machine's install pointer, and the dry `claude upgrade` says when the pre-commit gate comes from another install** (issue 0492). A suite run in a worktree published that worktree as the install root, and every project's gate then ran its guards from it. A linked worktree or temp-dir root no longer replaces a pointer that names a real install, and `intent bootstrap` still repairs one that was taken over.

- **`intent wb migrate` names the restamp it makes, with both values: `heartbeat: <authored> as authored, <carry> at carry`** (issue 0497). The carry writes the node's heartbeat at its own instant and keeps the header's claim beside it, as ruled, so a board untouched for weeks rendered as live on the morning it was carried, and nothing in the report said so. Both values are read back from the row the carry wrote; a header that claimed no heartbeat reads `none as authored`.

## [3.1.0] - 2026-09-19

**v3.1.0 teaches the index what a symbol IS, and then teaches it to resolve one.** Every symbol now carries its kind in its own language's words, the container it is written in and its arity, so a search can ask for the structs named `Config` or the methods of a type; and `intent index resolve` asks each language's own toolchain which definition a reference actually points at, which makes a caller list answerable for the first time. **It is also the first release built for a project more than one person works on**: every project act travels as its own committed file, `intent sync` plans and applies what a clone needs after a pull, and an id two clones both minted has a verb that repairs it. Beside that: the whiteboard's carry refuses a lossy migration instead of exiting clean, a project declares its own pre-commit guards in tracked configuration, the daemon's logs are readable through a verb and through Intent.app's new Console, and every search answer says when the index it read was last reconciled. **The store's schema moves 26 to 29 and the upgrade is one-way -- read the [release notes](docs/releases/3.1.0/RELEASE_NOTES.md) before upgrading if you might want a way back.**

### Added

- **A symbol row says what it is, what it belongs to and its arity.** The index stored `def` or `ref`, a name and a span, so nobody could ask for the structs named `Config` or the methods of a type, and a method Rust's tags query matched twice was two rows. Every symbol now carries `subkind` in its language's own words (`function`, `method`, `assoc_fn`, `struct`, `enum`, `variant`, `field`, `trait`, `type`, `const`, `static`, `module`, `macro` for Rust; `module`, `protocol`, `impl`, `struct`, `exception` and the clause as spelled -- `def`, `defp`, `defmacro`, `defmacrop`, `defguard`, `defguardp`, `defdelegate` -- for Elixir), `container` and `container_kind` for the impl, trait, module or type it is written in, `trait_name` on the rows inside an `impl Trait for Type`, `arity` and `arity_min`, and `level`. **One row stands for one name node**, fixed at extraction and never by a `DISTINCT` at read, and a definition's own name is never also a reference to it. Rust and Elixir are read through Intent's own queries rather than those grammars' tags queries, which carry no kind, container or arity; Swift and Lua keep theirs and gain the typed kind and the de-duplication and nothing else. The extractor carries a version, recorded per file, so a file whose rows an older extractor wrote is re-extracted by the next reconcile instead of mixing two shapes in one answer.

- **A reference says the path or the module the source wrote, and a name inside a macro invocation is a reference.** `AddressError::new(..)` was not a reference at all, a type used in a signature was not one, and `Repo.get(..)` and `Map.get(..)` were both a reference to `get`. Rust references now include scoped calls, every type name (an impl's trait and self type included), a scoped path that is not called, a name a `use` imports, and the names inside a macro invocation's token tree, each on its own name node, with the path as written in `qualifier` and `level` 2. Elixir remote calls, remote captures (`&Repo.get/2`) and remote pipe targets carry their module as the qualifier with the arity written or piped; local calls, local captures and local pipe targets stay unqualified at level 1; and `use`, `import`, `require` and `alias` are references named by the whole module, one row per brace entry. A module is expanded by the file's own `alias` forms -- `as:`, the brace form, `__MODULE__` and an Erlang module alias -- within the enclosing do-block, and nothing `import` or `use` brings in is applied. **A qualifier is stored as written and never resolved**, and every surface says so, including what each language's references still leave out.

- **`intent search --subkind <subkind>` and `--in <container>` narrow the index by what a symbol is and what it sits in, and with no query they are the whole question.** `--subkind` takes the language's own word (`struct`, `method`, `defp`, `call`) and repeats; `--in` takes a container named whole or by its last segment, so `--in AddressError` finds a method whose container is `errors::AddressError`. `intent search --subkind method --in AddressError` lists that type's methods with no text to search for, and a search with nothing to search for is refused with a remedy naming that form. **`--subkind` is its own flag and not more words for `--kind`**: one word carrying two vocabularies meant `--kind def` read as every definition to one caller and as an Elixir public clause to another. An unknown subkind is refused with the roster of the languages in scope, and that roster is read off the index's own compiled queries rather than a list written beside them, so it cannot promise a word the extractor does not write. `--context` and `--outline` now honour every filter and `--limit`, which they ignored, and a `--tier` filter that leaves the structural tier out is refused rather than answered from tiers that hold no symbols. Every symbol hit carries its `subkind`, `container`, `container_kind`, `trait_name`, `arity`, `arity_min`, `qualifier` and `level` under the same names in the terminal row, in `--json`, through the MCP tool, and in the SQL door and the published DDL face -- and the answer says which level answered and what that language's references still leave out.

- **`intent index resolve` resolves references to the definitions they name, using each language's own toolchain, and reports each language's run.** Until now a reference was a name match and nothing in Intent could say which `new` a line calls. Rust is read from rust-analyzer's SCIP export, one export per root-most `Cargo.toml` the index holds; Elixir by compiling the Mix project at the project root with the compiler's own tracer. `--lang <lang>` runs one language instead of every language the project declares, `--full` rebuilds everything the toolchain keeps incremental state for, and `--json` returns the record. **A resolved row always joins a written reference**, on path, line and name, against the bytes the tool read, so a reference only the toolchain sees -- a field, a variant, a bracket access, a macro's expansion -- is counted as unmatched and never stored; `matched`, `unmatched` and `dropped` partition everything the tool emitted, `dropped` is counted by reason from a closed roster, and a reason a reader did not declare fails the run rather than being printed. A run replaces exactly the files it joined, in one transaction, and purges rows whose path has left the index; a failure writes only that language's record, so what an earlier run stored still answers, each file carrying the hash it was read at. A run is full whatever was asked until one has stored under this build's extractor, because an incremental run over a build cache that already exists can trace nothing and report an empty tier as a success. **The toolchain builds under `intent/.cache/resolve/<lang>` and never the project's own build directory**: Rust's export runs with `CARGO_TARGET_DIR` inside it, and Elixir compiles the `dev` environment into `intent/.cache/resolve/elixir/dev`, seeded from the project's `_build/dev/lib` where there is one and made again when `mix.lock` moves, with `MIX_ENV` given to the compile rather than read from the shell. A tool that is not where Intent can run it is recorded as `missing`, naming the program, and named by `intent index resolve` at exit 1 and by `intent index status` -- never stored as a current run that resolved nothing -- and a declared language whose project holds no `Cargo.toml`, or no `mix.exs` at its root, is named not applicable and its record left alone. **It runs on this verb alone**: rust-analyzer's export runs the workspace's build scripts and proc macros and no switch it has stops that, and a compile runs a project's macros, so nothing starts it unasked -- not intentd, not a reconcile, not a hook, and it is not offered on the MCP tool tier in this release.

- **`intent index status` says what each language's level 3 holds, and names the files that have gone stale.** Beside the per-corpus counts and each declared language's grammar readiness, it now prints a `resolution:` block per language a run has recorded: the last run's state and tool, the failure it named with the file and line where the tool named one, then the stored run's `matched`, `unmatched`, `dropped` and `ambiguous`, its drops by reason, and a line saying so when that run joined under an extractor this build no longer writes, which makes every one of its files stale. **The stale paths are listed rather than counted**, for the reason the skipped paths are: a reader deciding whether a caller list can be trusted needs to know whether the file in front of them is one of them. `size: resolved` joins the size lines. `--json` carries the same under `resolution`, and so does the `index_status` MCP tool, because an agent weighing a resolved caller list needs the stale paths as much as the counts.

- **A reference a resolution run named answers at `level` 3 with the definition it points at, and `intent search --target <target>` asks for the references that point at one definition.** A hit whose key joins exactly one current resolved row carries `target`, `target_path` and `target_line` flat on it, and the terminal and the explorer end its row with `-> <target>  <path>:<line>`; one that joins several keeps its syntax level and lists them as `candidates`, reading `one of N`, because the store keeps no tie-break and neither does an answer. A hit in a file that has changed since it was resolved, or that moved on disk, keeps its syntax level. **Every answer carries `index.resolution`**, index-wide, naming each language whose level 3 is not current: `missing` (its toolchain is not where Intent can run it), `failed` (its last run failed), `stale` (the files it lists changed since they were resolved) or `unresolved` (no run has stored its references here), each with the sentence that says what that means and, for `unresolved`, that a person resolves it by running `intent index resolve`, which runs the project's build. A language whose project holds nothing for its tool is not named, so an estate that declares Elixir and holds no `mix.exs` does not carry a permanent alarm nobody can clear -- `intent index status` still shows the record. `--target` narrows every other door and every filter narrows it, with no query it lists the references alone, it keeps no definition, and it takes no hit from a stale file, which the answer names. A target no resolved row names is refused with the resolved targets that end the same way, compared whole, without a trailing arity and without a trailing `()`; otherwise the empty answer says that a target nothing references and a misspelt one read the same. An index where nothing has been resolved is refused with the remedy `intent index resolve` rather than answered as an empty list, and `--daemon` with a target is refused by name, because a daemon of another build would drop the field and answer the wrong question at exit 0. `--json`, the MCP tool's `target` and the SQL door's `resolved.target` ask the same question of the same rows.

- **Every search answer says when the index it read was last reconciled.** The envelope's freshness block declared `reconciled_at` and no store ever wrote it, so the field was omitted from every answer -- and a daemon-served search does not reconcile before answering, which makes the envelope the only place its age could come from. A reconcile now stamps the store in the statement that performs it, by the database clock, and `reconciled_at` rides in every envelope: the CLI's, the daemon's, the MCP tool's and the explorer's pane alike. A whole-scope reconcile that finds nothing still ran and still stamps; a scoped refresh that finds nothing writes nothing, because a stamp inside `intent/` would be the next event the daemon's watcher sees.

- **`intent index status` and `intent index rebuild` report what the index costs, measured.** Neither said anything about size, which is the figure that decides whether indexing a large tree's source is affordable at all. Each index table's bytes are now read out of SQLite's `dbstat` -- an FTS5 table's shadow tables summed under its own name, a table's indexes counted with it -- and the whole store's from its page count, printed as `size: <family>  <bytes> bytes` and carried in `--json` under `sizes`. Read, never estimated.

- **Intent's MCP server tells a client what its index answers, on the handshake.** A client that lists a server's tools by name only -- deferring their schemas until something asks for them -- showed the model nothing about what they do, and Intent's server sent no `instructions`, the one field such a client puts in the model's context. So a session with the server loaded still reached for grep for a question the index answers better. The server now sends `instructions` on `initialize`, from one line in the command register: it routes a code lookup by name to the search tool (`kind` def, `outline`, `context`), says what those answers include and what they never resolve, and names the selector a client needs in order to load the tool's schema before calling it.

- **A standing directive is an item kind of its own, `directive`, and only `hv`'s board carries one.** The protocol has always kept the hypervisor's durable instructions under `## Standing directives` -- read at pickup the way decisions are -- and the model had no kind for them, so `intent wb migrate hv` could only report every directive as a line it would not carry, and carrying them as decisions would have lost the distinction the protocol draws: a decision records a call that was made, a directive is an instruction still in force. `intent wb add directive "<text>" --node hv` writes one, the same verb on any other node is refused by name, the board view renders the kind under `## Standing directives` on `hv`'s board alone, and `intent wb migrate hv` carries that section's lines. A board that is not `hv`'s and carries the section refuses the whole carry, naming the section, before anything is written. The store keeps the kind as unconstrained text, so no store schema moves; the published faces and the command register carry the new value. Whether a fold may retire a directive is a rule in the `in-whiteboard` skill and not a refusal, because the verb cannot tell `hv` retiring a spent directive from a fold tidying the board.

- **`intent wb register <moniker> --name <display> --role <role> --correct` changes a registered node's name and role, and keeps its board.** A node registered with the wrong values could be repaired only outside the verbs: `register` refuses a re-register with different values and offered no update or force, so what was left was hand-editing `intent/whiteboard/<node>/board.json`, a generated file, and running `intent sync --to-store`, or a `DELETE` on `wb_node`. `--correct` writes name and role and nothing else, so the node's board, its items and its messages stay attached; it refuses a moniker that is not registered and names plain `register`, so it never creates a node; and values already held answer `unchanged` at exit 0 and record nothing. The refusal an ordinary re-register takes now names the flag beside the held and the offered values.

- **Every whiteboard verb records one event, so the log says which node wrote a board row, when, and with which arguments.** Board writes reached the event log nowhere: the store could say what a row held and nothing about who put it there, which is why a registration that wrote a foreign name over a live node could not be attributed to anything. Each verb -- `add`, `announce`, `archive`, `ask`, `claim`, `clear`, `correct`, `decide`, `migrate`, `pickup`, `register`, `release`, `touch` and `unclaim` -- now writes one `wb.*` event naming the acting node (the sender for `ask` and `announce`) and the verb's arguments, in the same transaction as the rows it writes, so `intent events --op wb.add` reads them like any other op and `--subject <node>` narrows to one node's writes. A verb that moves no row commits nothing and records nothing, and a restore records no board event, because its act is the sync and the sync records it. Every statement that changes a board row also moves that row's `updated_at`, which used to be stamped at insert and moved by no whiteboard writer -- so a pickup, a touch, a release, an archive, a clear, a claim and a migration's header carry each leave a trace of the edit, while an unchanged row keeps the stamp it had.

- **`intent wp show <ST>/<NN>` lists the criteria scoped to that work package, and `intent wp gate <ST>/<NN>` prints its close-gate verdict.** Asked which criteria a package holds, which are satisfied and by what, no verb answered at the package: `wp show` printed its header alone, `wp gate` was not a command, and `ac list` scopes to a thread. `wp show` now prints one `ac list` line per criterion in the package's scope, or says outright that none are scoped, and the MCP resource read prints the same bytes. `wp gate` prints the verdict through the renderer `ac gate` already uses and exits non-zero on BLOCKED, so a pass over an empty scope reads differently from a pass over satisfied criteria. Both read the package through one door, so the listing and the verdict cannot disagree about what is in scope. No new MCP tool: `ac gate <ST>/<NN>` already serves that verdict there.

- **`intent st detach <ID> <path>` removes an attachment from a thread, and says what happens to the file.** `intent st attach` had no inverse, so a document could leave a thread only by hand-editing `intent/.canon/st/<ID>.json` and running `intent sync --to-store` -- the hand edit of canon the CLI exists to spare an operator. The record now leaves the store and canon through the same write `st attach` uses, under its own event. **The file on disk is yours**: when it is still there the output names it and says that a running `intentd`, or the next `intent sync --to-store`, carries an authored file under a thread back in as an attachment unless it is deleted. A detach naming an attachment the thread does not carry is refused at exit 1, and the remedy points at the paths the thread's canon does carry.

- **`intent daemon logs` prints `intentd`'s recent log lines, and `--follow` keeps printing them as the daemon writes.** The daemon writes two logs, its stdout and its stderr, and no verb named them or read them: an operator asking what the daemon had been doing had to find both files first. It prints one header naming both, then their last lines as ONE list merged by each line's UTC stamp (`--lines <n>`, 40 by default), so an error older than the newest start line no longer sits below it and reads as though it came after; a line with no stamp stays under the line above it in its own file, so a multi-line notice never splits. A log the daemon has not written yet is named as absent rather than skipped in silence. Without `--follow` it exits 0, because a script or an agent that runs it must not hang; with `--follow` it resumes each log where the backlog stopped, so no line prints twice and none written in between is lost, and the tail dies with the verb however the verb dies.

- **Intent.app has a Console, on Cmd-L, and its one-off verbs write into it.** Run Doctor ran `intent doctor` and discarded the output, so a clean pass was indistinguishable from a click that did nothing. The Console tails `intent daemon logs --follow` while its window is open and colours lines by the CLI's own tokens -- `error:` and `caused by:` as errors, `warning:` and `remedy:` as warnings, `»` lines as markers -- reading past the timestamp every daemon line now carries, and a line with no token is left as a log line rather than guessed at. Run Doctor and the new Rebuild Search Index stream into it between `» intent doctor` and `» exit 0 · 1.2s` and bring it forward, so the result is seen; Start, Stop and Restart intentd write their command and result there without bringing it forward; a second one-off while one is running is refused with an alert naming the running command rather than queued. The footer names the files being tailed, File -> Close (Cmd-W) and View -> Clear Console (Cmd-K) reach the window, and the app's own lines are kept up to the backlog's bound, so a command that ran while the window was shut is there when it opens. Closing the window -- or the app dying however it dies -- ends the verb and its tail, because the app holds the child's stdin.

- **A thread's or an issue's view in the explorer splits in half, and the selected field's contents render as markdown below it.** Only the rows of a thread's work-package, criteria and test lists ever opened a pane, and every other row was one clipped line, so a field longer than the line could be read only by opening an editor on it. Every field row of an item view now carries its whole value, a collection row carries its members as its own list shows them, and a thread's document rows carry the attachment; the pane renders headings, emphasis, inline code, lists, quotes and tables in the explorer's own colours, wrapping prose at the width it is given and cutting code and tables at the edge rather than reflowing what they say. Tab crosses between the halves, the arrows, Page Up/Down and Home/End scroll the pane, Enter edits the selected field from either half exactly as Enter on the field does, and `/help` names those keys. A split body divides in half rather than taking what the detail needs, so the pane no longer jumps as the cursor crosses from a one-line field to a long one. Code blocks render in one colour: no syntax highlighter ships with this.

- **A project declares its own pre-commit guards in `intent/.config/config.json`, and Intent's hook runs them.** `.git/hooks/` is untracked in every repository, and so is the `core.hooksPath` line that points git at a tracked hooks directory, so a guard wired by hand belonged to one checkout: a fresh clone ran Intent's roster and not the project's own guards, its commits were gated by fewer guards than the original, and nothing anywhere said so. A `"guards"` array names each one as an argv -- `run`, resolved from the repository root and never handed to a shell, with an optional `when` path that makes the guard not applicable while nothing exists there -- and the runner dispatches them after Intent's own roster, through the same reporting. The declaration is tracked, so a clone runs the set the checkout it came from ran. A declared guard whose body is missing, not executable or untracked, and a `guards` array that cannot be read, block the commit with a remedy; that is deliberately stricter than a missing roster guard, which is an install behind its roster, where this is a broken tree. `--list-guards` says whose each guard is, and `intent doctor` reports what a clone will not receive as advisories it does not count: an untracked chain line that no guard declares, a guard both declared and hand-wired so it runs twice, and a tracked `.githooks` or `bin/hooks` pre-commit while `core.hooksPath` is unset.

- **`intent doctor` reads the search index and says when it is damaged.** A malformed FTS5 source index could fail a search on an affected term while `doctor` and `intent index status` both read clean over it. `doctor` now reads both search tables two ways: every document the index holds, checked against its content table, and SQLite's own per-table FTS5 check. A damaged table gets one line saying what the two readings mean together, then what each read, with the orphaned document ids named and `intent index rebuild` as the remedy. It is shown on a default run and in `--format json`, it is not counted as a finding, and it leaves the exit code alone. Both readings read the index's own segments, so they share a blind spot, and a clean result says so on the summary line, under `--quiet` too.

- **`intent sync` prints this clone's plan for bringing its store up to the committed canon, and `intent sync --apply` carries it out.** Bare `intent sync` used to refuse and ask which direction you meant. After a pull, every verb answered from the store as it stood before the pull -- `intent st show` on a thread the pull brought answered `no steel thread` -- until someone ran the whole-store restore, which is the destructive direction. Bare `sync` now reads the store, the tree and git's status and prints the steps this clone needs, in order, writing nothing:
  - a branch behind its upstream, said and never pulled;
  - unmerged paths Intent does not own, said and left;
  - an id both sides minted, renumbered;
  - a canon file both sides changed, a side to take;
  - the ingest of the committed canon and event files the store does not hold;
  - unmerged and stale generated views, regenerated;
  - the index, refreshed;
  - then `doctor`, whose verdict is the exit code.

  `--apply` runs them. **Each step declares its recoverability.** A quiet step runs. A reversible one, eg a renumber or staging the views it regenerated, asks `y/N` on a terminal, and `--yes` answers it. A non-reversible one, taking a side, always asks a person, and no flag answers for it. With no terminal the quiet steps run and one `left:` line names the rest. `--plan <digest>` refuses an apply when the tree has moved since that plan was printed. The ingest takes the disk only where it says something the store did not write, so `--apply` runs beside a watching `intentd`. **Intent never pulls, commits or pushes**: its one write to git is staging the files it wrote to resolve a conflict. `--to-disk` and `--to-store` keep their meanings, and `--apply` with either is refused.

- **`intent claude upgrade --apply` wires `post-merge`, `post-checkout` and `post-rewrite` hooks that run `intent sync --apply` after a pull.** They go through the same region-edited chain as the pre-commit gate, from one carrier template, and `post-checkout` runs only on a branch checkout. A hook has no terminal, so it runs the quiet steps and names what it left. It prints one line when the store changed and one naming the failure when it could not run, and it always exits 0, because a hook must never fail a checkout. `.git/hooks` is not cloned, so each clone runs `claude upgrade --apply` once.

- **`intent st renumber <old> <new>` and `intent issues renumber <old> <new>` move a thread or an issue to a free id.** Ids are minted highest-plus-one over the local canon, so two clones mint the same `ST0001` or the same issue `0001`. Git refuses the merge with an add/add conflict, and the only repair was a hand-renamed canon file, a hand-edited id and a store restore. Each verb refuses an id the store holds, and an id a pull put in the tree that the store has not loaded yet.
  - A thread's renumber moves its canon file, its realised directory, its views and its `.intentfiles` line, rewrites other threads' `related` references, and moves the whiteboard claims on the thread and its work packages.
  - An issue's renumber moves its canon file and its view.
  - Each records its own event, and a filesystem failure puts every move back.
  - **Prose that names the old id is found through the index and listed, never rewritten**, because prose is authored.

  Mid-merge, `intent sync --apply` renumbers this clone's side of a twice-minted id itself. Neither verb is offered on the MCP tool tier.

- **Every project act travels as its own committed file, so a project's history crosses a clone.** The event log lived in each machine's store and nowhere else. A clone got the present and none of how it was reached, and `intent events` on two clones of one project told two stories.
  - Each act now writes `intent/.canon/events/<YYYY>/<MM>/<DD>/<ULID>.json` in the same write as the rows it records. The file is named by its id, never rewritten, and names its author: git's `user.name <email>`, then the project config's author, then `local`.
  - Acts that describe one machine stay in that machine's store: a heartbeat and a pickup's stamp, an organize, a sync in either direction, and a text realisation.
  - A clone's ingest adds every event file whose id its store does not hold, and deletes nothing.
  - `intent doctor` reads the files, and names by path one that is not an event or whose name is not its id.

  **`intent upgrade` writes, once, an event file for every project event the store holds and the tree lacks**, so a project's history from before this release travels too, and it says how many it wrote. It also removes the `intent/events.jsonl` ignore rule earlier v3 versions wrote, with its comment.

- **`docs/concepts/working-in-a-team.md` says how a project works across several clones.** It is written from a driven run of two clones of one bare origin, and it covers:
  - what travels and what stays on each machine;
  - the setup, once per machine and once per clone;
  - what a reviewer reads in a pull request;
  - the one command after a pull, and what the hooks do not see;
  - an id both clones minted, and one thread edited on both sides;
  - history travelling, and the schema refusal between Intent versions;
  - a CI job that runs `intent doctor` on the merge result, because a merge made on the forge passes through nobody's commit gate.

### Changed

- **The generated `CLAUDE.md` tells a session to ask Intent's index before grep, and how to reach it.** The MCP server already told a client on initialize what the index answers, and a session that had read that text still reached for grep first. The template now carries a `## Finding code` section naming the tool and the call: load `mcp__intent__intent_search` through ToolSearch, then ask for `kind` `def` with the name as `query`, or `context` with the name, and fall back to grep only when the answer says the index is not complete for the paths involved. It is in `CLAUDE.md` alone and not in `AGENTS.md`: ToolSearch and the tool's name are Claude Code's, and `AGENTS.md` is tool-agnostic. It offers no shell alternative on purpose, because a CLI line beside it invites Bash. Which tool a session reaches for is the model's choice and varies between runs, so this is a section that changes the odds and not a guarantee.

- **The search tool's description leads with the code lookups.** It opened on whole-project prose search and sent code lookups to grep, with definition, symbol, outline and callers appearing only in the parameter text -- so a model searching its own tools for where a function is defined did not find this one. The row now leads with where a name is defined, what a file holds and where a name is called, keeps whole-project text search as the second use, and says what `complete: false` means now that a binary or a symlink no longer sets it. `intent search --help` and the MCP tool's description are both rendered from that row, so they cannot disagree.

- **The shipped symbol-context hook answers a Bash grep, and its answer reaches the model.** It matched Claude Code's `Grep` tool only, while a session's searches run through Bash -- and it printed its answer as plain stdout, which a PostToolUse hook's output does not carry to the model at all, so it had been invisible even for the one tool it did match. It now prints its answer as `hookSpecificOutput.additionalContext`, and it answers a Bash call whose command runs exactly one `grep`, `egrep`, `fgrep`, `rg` or `git grep` with exactly one symbol-shaped pattern. A regex, several patterns, several searches in one command, a pattern file or a command built by substitution append nothing, because the question is no longer one name; a command with no grep-family word, or longer than 512 bytes, is declined before it is read, because the recogniser's walk is quadratic and this runs after every Bash call. The freshness rule that silences it when the index cannot answer for the paths in its own answer, and exit 0 whatever happens, are unchanged, and the shipped settings still do not wire it -- turning it on stays the project's decision, and the script says how.

- **`intent wb migrate` refuses to carry a board that holds lines the model cannot hold, names every one of them with its file and line before it writes anything, and carries the rest only when `--drop-uncarried` says to.** A `###` sub-heading became an item whose text was a literal `###`, a markdown table became one item whose rows rendered as a bullet's continuation lines, the words after a heading's kind word were dropped, and a board's lead paragraph was reported and dropped -- and the run exited 0 either way, so a script could not tell a complete carry from a lossy one, and the re-run that would have fixed it was refused as a second carry. Each of those is now one named unit, the exit status tells the two apart, and the flag that proceeds says in its closing line how many units it dropped. Whichever way it goes the board's `wip.md` is kept byte for byte at `.history/pre-migration/wip.md` and carried as a snapshot, so a dropped line leaves the model and not the record; a different file already in a copy's place refuses the carry rather than being overwritten, and no copy is written until every copy's path has been checked.

- **The shipped whiteboard header guard refuses a header value that has been detached from its key.** The header block is one line per key and the reader takes a value as everything after the first `: ` on the key's line, so a value a formatter has broken onto its own line reads as EMPTY -- a node claiming work packages reads as claiming none, and nothing about the board looks wrong. The usual author is a `prettier --write`, either from a gate that writes before its guards or from the run made to clear a `--check` refusal. The guard now reports each detached key once, with the rejoined line and its length; past the formatter's `printWidth`, read from the consumer's own `.prettierrc.json` or `.prettierrc` and otherwise prettier's default of 80, it says that rejoining is not the fix and to shorten the value, because a rejoined line over the width is broken again by the next commit. Same corpus as the escape check beside it: live boards, the header block, and only what the commit adds.

- **The agent guide states no count of its own surface.** The guide that `intent llm` prints said how many rows declare `--help`, a figure that goes stale the moment one more does, while the sentence's whole point is that `--help` works everywhere and a per-row rendering would under-report it. It says that only some rows declare it. Nothing else in the guide changed.

- **A generated view's footer names the model it was rendered from, in prose.** A thread's `info.md` and `acceptance.md` said they were generated from `thread.json`, which is not a file this build has: a thread's canon is `intent/.canon/st/<ID>.json`. The footer names the model now -- the thread canon, the issue canon, the whiteboard model -- and prints it without backticks, because a phrase is not a file name. The root files, whose footer names a real template, keep theirs.

- **A thread's and a work package's generated `## Acceptance` paragraph names the verbs that write a contract row.** It sent the reader to mint or reword a row in the thread's canon file and then run `intent sync --to-store` -- a hand edit of canon, which is the route those verbs retired. Both covers now name `intent ac new` and `intent at new`, `intent ac edit` and `intent at edit`, and the state verbs, from one source, so the two cannot drift. A cover that authors its own `## Acceptance` section is untouched.

- **`intent/.intentfiles` is one flat list, and a line that is exactly `# BEGIN INTENT` or `# END INTENT` is refused with its line number.** The manifest once had two regions: a block between those markers, rewritten from status, and pins outside it that survived the rewrite. That design was replaced -- commands change the list and nothing recomputes it, so a hand-added line has no rewrite to survive -- and the marker grammar outlived it in the parser alone. A balanced pair parsed as regions nothing acted on, and an unbalanced one refused by naming a construct that no longer means anything. Both markers begin with `#`, so the comment rule would otherwise admit them in silence. Every entry now declares its artefact wherever it sits in the file, `intent edit` appends its line rather than placing it above a marker, and the refusal says to delete the marker line and keep the entries around it.

- **A thread's `acceptance: exempt` is fixed when the thread is authored, and the close gate's refusal names the routes that exist.** The field was declared immutable -- an attribute of a thread rather than a lifecycle, so it gets no verb -- while `intent set` wrote it happily, and the gate's refusal for a thread whose contract had been emptied told the operator to declare it. The declaration and the setter were two answers to one question. `intent set intent:///threads/<ID> acceptance ...` now refuses, naming the field and what the value is for, and the explorer's thread form locks that row rather than refusing after someone has typed. The gate's refusals for an absent or emptied contract name what a verb can actually do: add a criterion with `intent ac new`, bring one back with `intent ac rescope` or `intent ac reinstate`, or cancel the unit with `intent st cancel` or `intent wp cancel`. A thread that already declares the exemption still gates as exempt.

- **Every line `intentd` writes to its logs opens with an RFC 3339 UTC time, and every notice that carries a remedy opens with `warning:`.** The logs alternated `intentd listening on ...` and `intentd stopping: asked over the wire` with no time on any line, so the only clock was the file's mtime and a deliberate restart could not be told from a crash and a respawn by anything but the wording of one line. One writer now stamps both logs -- `YYYY-MM-DDTHH:MM:SS.mmmZ`, one space, then the line byte for byte, with every line of a multi-line notice carrying that same reading -- and `intentd --version` and `--help` stay unstamped, because they answer the person who typed them. The notices that printed a `remedy:` line under an opening with no severity token now carry `warning:`, the token the daemon's accept and LaunchAgent notices already carried, so a reader of the log or of Intent.app's Console tells a failure from a record by the line's own words instead of guessing from the words after `intentd:`; and an ingest failure prints its remedy once, where it printed it twice. A stop asked over the wire still names no asker: a request is a project root and an op, and carries no identity.

- **`/projects` is a list inside the explorer, searched and driven like `/threads` and `/issues`.** It was a screen of its own, with its own layout, its own keys and nothing to type into -- the one place in the explorer where progressive search did not work. The registered projects are rows on the view stack now, filtered by the omnibox's own matcher, and every project is in the omnibox, so typing its name anywhere offers it. The cursor starts on the project that is open, or on the registered project nearest the working directory when none is, the earliest on a tie; a directory that shares only the home prefix with every project leaves it on the first row. Enter switches to the project and reopens at its threads list, and a root that is no longer a project stays listed as missing and is refused with the reason, leaving you on the list. `intent explore` outside an Intent project opens the explorer on that list, where help and settings work, every other view says no project is open and names `/projects`, and `/quit` returns to the shell.

- **`intent claude upgrade --apply` now says, after its summary, that it writes no generated view and that `intent organize --apply` is the one door for a stale one.** The command rewrites canon and the gate, and a reader whose views were stale had no line telling them which verb re-renders one -- so the note sits directly after the `ok:` line, where the report is still being read.

- **The bundled SQLite is 3.53.2, up from 3.46.0.** 3.46.0 carries SQLite's WAL-reset database corruption defect, fixed in 3.51.3, whose preconditions are a WAL-mode database written or checkpointed by more than one process at once -- which is how `intentd` and the CLI share a store. It also carried a false-positive FTS5 integrity-check report in secure-delete mode, the mode Intent's search tables run in, which 3.46.1 fixed. `rusqlite` moves from 0.32 to 0.40, built with its `fallible_uint` feature, so a `u64` still converts through `i64::try_from` and fails loudly past `i64::MAX`, as 0.32 did by default. An existing store's search index opens as it stands.

- **A search does not reconcile the index itself where a daemon is watching the project.** Routing is opt-in, so a plain `intent search`, and the explorer's search pane, answer in this process even while `intentd` watches the tree, and each one reconciled the index first: a second writer beside the one already keeping that index current. Both now ask the same question the sync carve-out asks, and skip the reconcile when the daemon is watching. Where the daemon cannot answer, a search reconciles, as every search did before, because a daemon that hiccuped must not become a search that cannot run.

- **The release preflight refuses a stale `docs/reference`.** v3.0.3's tag shipped the previous cut's reference pages, because the regeneration landed after the tag and nothing reported it. The preflight now regenerates both halves into scratch against the previous release tag and refuses on any difference, masking only the generation stamp and the revision row. `--allow-stale-reference` lets a cut proceed with a warning and the diff.

- **`intent doctor` shows store-stale on a default run.** After a pull, a store that did not hold what the committed canon held was reported only under `--verbose`, so a default run printed `0 finding(s)` over a store that answered `no steel thread` for a thread the pull brought. It is now shown and not counted: the exit code is unchanged, the line prints, it counts the committed event files the store does not hold, and its remedy names `intent sync --apply`.

### Fixed

- **The explorer's search pane draws its freshness and level note.** The pane was meant to put a note on its info row when it opens -- that the answer is partial, or which files moved since they were indexed, or, with nothing partial to say, which level answered and what that language's references leave out -- and no frame had ever shown it: the event loop set the note, arrived at the view, and then cleared the notice, so every note was wiped before the next frame drew it. The arrival is now the one place a view's notice is set or cleared, and the pane's row is asserted on the screen rather than on the function that produces the words.

- **`intent search` counts the rows the answer carries.** `returned` and the `N of M shown` note counted hits the body did not hold, so the answer said more had come back than it carried: the code corpus was searched whatever `--tier` said, counted into both denominators, and then dropped along with the lexical group that held it, which `--tier structural` does. The corpus is read only when the lexical tier is asked, both numbers are summed from the groups the answer carries, so `returned` is the length of the body, and `--limit` caps each group rather than a blend -- tiers are ranked within themselves and never mixed.

- **A search tier that cannot answer says so, and counts nothing.** `--tier semantic` in a project with no embedder counted hits it could not return and printed a note about withheld rows above an empty answer. The envelope now carries `unanswered`, one entry per tier asked for by name that this project cannot serve, with the reason -- no embedder configured, no vector for the query, or nothing embedded for this model yet -- and the terminal says `note: the semantic tier did not answer -- no embedder is configured for this project`. An absent tier nobody asked for is still simply absent.

- **`intent search --outline <path>` lists what a file defines, and a definition answers once.** An outline listed every call site in the file as a `ref` beside the definitions, and every definition twice, because two patterns of a language's tags query match one node and a query cursor reports both -- so the prior-art check this project's own canon prescribes, `intent search --kind def <name>`, answered twice for a name defined once and read as a Highlander violation that was not there. An outline is now the file's definitions, with its references listed only when `--kind ref` asks for them, on the CLI and the MCP tool alike, and one definition is one row, held where the duplicate is made rather than hidden by a `DISTINCT` at read.

- **A text hit prints the line that matched.** The prose form printed the hit's name -- a document's section label, or a whiteboard entry's whole heading -- while the matching line was carried only in `--json`, so a search showed a reader where a hit was and hid what it had matched. The row now carries the snippet after the name when the two differ.

- **A file the store came to carry answers a search once.** A path indexed before the store carried it -- a whiteboard board's `.history` snapshot, after boards became rows -- kept its `file` rows beside the store's own sections, so one line came back twice in the same answer, once owned by the path and once by the node's moniker, identical in every other field. A refresh removed only paths that had left the disk; it now also removes a path the store has since taken over.

- **`intent index status` says `no-grammar` for a language that has none.** The prose kinds `author` and `content` were reported as `grammar: author  unknown`, which is the word for a state the reporter could not determine, beside the `no-grammar` the row already had for a declared language with no grammar. Both now read `no-grammar`, so the report no longer looks like a lookup that failed.

- **A search answer names every path the index would not take, and only text it could not read makes the answer incomplete.** The envelope's `skipped` list was never filled, so `complete: true` sat beside an index holding unread files and a search inside one was a confident miss with no note to say so. The answer now fills `skipped` from the index's own rows, scoped by `--path` and `--lang` exactly as staleness is, and the terminal prints a note per path. A skip by policy -- a binary file, a symlink -- is listed and leaves the answer complete, because neither holds text a query could match; a file over the size cap or one the index could not read leaves a gap, as does a reason this build cannot parse. So `complete: false` now means text the index has not read, which is the one condition the tool's description tells a caller to grep for.

- **A word inside a CamelCase or snake_case name finds the file that defines it.** The index carries a column whose whole purpose is to make the parts of a name searchable, and nothing ever filled it: `Removal` did not find `SkipRemoval`, and only a prefix reached a camel-cased name, from its start. A reconcile now fills a source file's name parts from the names it defines, split at CamelCase and snake_case boundaries, for every language the project declares.

- **The MCP search tool and the explorer's `/search` pane reconcile before they answer.** Only the CLI did. Without a daemon these two answered from an index as current as the last CLI search or `intent index rebuild`, so a file added, or edited so that it now matched, produced no hit, no stale path and `complete: true` -- a confident subset, on the two surfaces a model and a reader actually use. Both now refresh the index first, as the CLI has since the daemonless path was built.

- **The MCP search tool honours a filter sent as a string, and refuses one of any other type.** `kind`, `tier`, `lang` and the outline `ref` switch were read only as arrays while the tool's schema publishes them as strings, so the string a model sends was dropped without a word and the unfiltered envelope came back -- asking for a definition by name answered everything the text tier had, where the CLI's `--kind def` answers one line. Each now takes one string or a list of strings, the same reader every other parameter uses, and any other type is refused naming the parameter and the types it takes.

- **A commit no longer costs the daemon a walk of the project's `intent/` tree per event.** The canon watcher kept a file-ID cache that walked whatever each creation event named, with a stat per entry and before any ignore rule applied -- and a commit's gate and ingest produce exactly that burst of events, so a two-line board commit cost the daemon seconds of system time on one watcher thread, and a run of commits added up. That registration now keeps no cache, as the index registration has since the idle-daemon fix. What the cache bought is a rename arriving as one event; without it a rename arrives as a removal and a creation, which the batch handler already reconciles as a vanished path and a leaf. The watched scope is unchanged.

- **A large indexed directory vanishing costs a refresh one statement per table, not one per path.** Every removed path took its own `DELETE` in each index table, and neither section table can look a row up by path, so each of those statements scanned a whole table: a build directory of many indexed paths leaving the tree ran a refresh at a core for minutes without finishing, where a wholesale rebuild of the same tree took about a minute. The paths are now staged into a temporary table and each table takes one delete against it.

- **`intent wb register <node> --name <name> --role <role>` beside a hand-authored board writes no file, and `intent wb migrate <node>` lands the carried board itself.** The argument form of `register` stamped the node migrated and rendered its empty row at once, so the `wip.md` the migration exists to read was replaced by a board of `_(none)_` sections before the migration could open it, and the documented order -- register, then migrate -- carried an empty board unless the file was restored from git in between. The roster form has held to the opposite rule since it was written; the single-node form now holds to it too: the row is written, the index is refreshed, no file is touched, and every board write refuses by name until the carry has run. The carry then lands its own board and inboxes, which it could not do while it was still reading them: before this, a migrated board stayed hand-authored on disk, `intent doctor` reported it as view skew, and the pre-commit doctor gate refused every commit in the repository -- other sessions' included -- until some later board write happened to re-render it. The projection writes only views whose node has a migrated row, so an unmigrated peer's markdown is still left alone.
  - **Corrected 2026-09-21 (issue 0486): "writes no file" is false.** Both forms of `register` record one `wb.register` event file under `intent/.canon/events/`, as the entry above that says every whiteboard verb records one event states. What holds is narrower: beside a hand-authored board, `register` writes no board or inbox view, so `intent wb migrate <node>` still reads the file. A node with no hand-authored board has its board rendered at once, and an unchanged re-register writes nothing.

- **`intent wb migrate` refuses a board holding an inbox from a sender the roster does not carry, names every such file and the registration that admits its sender, and writes nothing.** It used to skip that inbox, carry the rest, and report `Register the sender and re-run` -- a remedy that could not be followed, because the verb refuses a second carry of a board that already holds rows, so those entries could then reach the model by no command at all. The refusal now comes before the board is even read, so the re-run it asks for stays open.

- **A carried inbox entry does not count against the message bound, so a migrated inbox never refuses its sender.** The carry writes every hand-authored entry as a live message, so an inbox with a long history was born over the bound and from that moment the SENDER's next `intent wb ask` was refused until the RECIPIENT cleared -- which a node that is offline cannot be asked to do. The bound is backpressure on a live conversation and a migration is not one: a carried row stays out of the count until the recipient's first `intent wb clear` marks it handled with the rest, and a message sent through the verb counts as it always did.

- **A carried inbox entry's heading shows the stamp it was written with, as claimed text beside the carry's own.** A migration stamps every entry it carries with one `recorded_at`, so entries written hours apart all rendered as the moment of the carry and a migrated inbox lost the ordering its headings exist to give. The written stamp is now rendered verbatim after the heading's own, marked `claimed`, where nothing reads it as a time: a board's markdown is the one place in the model where a stamp is a claim rather than a reading, and it is kept as a claim rather than laundered into a form indistinguishable from the real thing.

- **A board's `claims:` header carries only addresses `intent wb claim` would accept, and a spelling it refuses is named with the form the verb takes.** The carry copied the bracketed list verbatim, so a board claiming `ST0112/WP-07` reached the model with a claim no command could have written and nothing could resolve -- the migration was a second door into that column, admitting what the first door refuses. The check now has one home that both doors ask, a refused claim is reported as a unit the carry will not carry, and a `/WP-NN` spelling gets its `ST0000/NN` address as a hint rather than a silent rewrite.

- **An inbox entry written in its own heading carries that text as its body.** The reader took the `Re:` field and the FYI marker out of a `## (...)` heading and discarded whatever else was there, so an entry whose whole message sat on the heading line was carried with an empty body and the words were reported nowhere -- a drop with no record, which is indistinguishable from a line that was never written. Heading prose now leads the body, above anything below the heading.

- **The carry's report says which word applies to what: a sentence carried as an item is `coerced:`, a file it left on disk is `left in place:`, and `uncarried:` now means only what it refuses or drops.** Prose sitting inside a board section is carried as an item of that section's kind, because that is how every live board writes its DOING -- but a paragraph reading `Nothing.` then counts as work in flight, and nothing said so. Non-markdown files under a node's `.history/` were reported as uncarried while still being on disk and tracked, so a run that lost nothing read as a list of losses. Each is now its own line with its own word, and the reconciliation counts them apart.

- **`intent wb migrate --drop-uncarried` keeps every hand-authored inbox holding a line it drops, byte for byte, beside the board's own copy.** The drop was ruled safe on the ground that nothing is lost, because the pre-migration board is kept verbatim -- and that held for `wip.md` and for nothing else: a dropped inbox line, such as a note that an inbox had been cleared into `.history/`, was afterwards in no file under `intent/whiteboard/` and in no row, because the carry re-renders every inbox from the rows it wrote. Each such inbox is now kept at `.history/pre-migration/inbox.<sender>.md`, on disk and in the store's prose, named on its own `carried: [snapshot]` line and named in the refusal's remedy before the drop is taken.

- **A node whose board and inboxes are views Intent rendered carries nothing, says which files were views, and exits 0.** A project that registered a node without carrying its board has generated views under it, and the carry read them as somebody's markdown: on an inbox it refused the node over the two footer lines of every file, naming lines nobody wrote, and on a board it carried that footer into the model as the node's own decisions. The renderer now owns one predicate for whether a text is a view it wrote, and both readers ask it, so a view's banner is set aside the way the empty-section and empty-inbox sentinels already were. A view that offers nothing once its banner is gone is reported on a `rendered:` line and no copy of it is kept, because every byte of it was written from rows; a view that still offers a unit is carried as any board is, since on a copied or freshly cloned estate those lines can be the only record of rows the store does not hold.

- **`intent upgrade`'s and `intent organize`'s legacy-pointer worklists leave out lines under a whiteboard `.history/` archive.** The worklist tells the reader to read and reword each legacy pointer it found, and on an estate whose only hits were archived board folds the remedy could not be followed at all: the append-only guard refuses any commit that changes an archive there. An archive is a record rather than a pointer to reword, so those hits are off the worklist; a live file naming the same v2 path stays on it.

- **`intent wb pickup` returns `hv`'s directives, watch-outs and decisions in full, and every peer's header says what it leaves out.** A pickup rendered the acting node's own items completely and each peer as one header line, so no node saw another board's standing content at boot -- and because the read showed a node its own watch-outs in full, it did not look short, it looked complete. One node booted past a ruling that a test flake was known and spent a morning diagnosing it. `hv`'s live items of those kinds now come back in board section order, and each peer's header carries a `not shown:` line counting its live items by kind and naming `intent wb show <peer>`. Archived items are neither shown nor counted, and the standing list is absent when the acting node is `hv`, whose own board already carries them, or when the project has no `hv` board -- so an empty list only ever means `hv` holds none.

- **`intent wb pickup` records the session it runs in when no `--session` is given.** It wrote a node's `session_id` only when the flag was passed, and the `/in-whiteboard` skill's pickup line passes none -- so a node that booted by the skill kept whatever id an earlier session had written, and nothing reported it. A read that finds a node's transcript from its header therefore reached a dead session with nothing distinguishing the wrong transcript from the right one. The id now comes from `CLAUDE_CODE_SESSION_ID` when the flag is absent, read at the CLI edge; an explicit `--session` still wins, and with neither the row keeps the value it holds. The MCP pickup tool takes the same default, because the server runs as the session's child and carries the variable.

- **`intent wb register` reads a board's header before it writes, and refuses rather than writing past it.** Naming a node on the command line where `intent/whiteboard/<node>/wip.md` already existed used that file only to decide whether to render, and never read the header in it -- so arguments contradicting the name or role the board itself carried were written without a word, and a foreign identity reached a live node's row that way. The header is now read the way `wb migrate` reads it; a disagreement names both values and the file, and nothing is written. The roster form -- `wb register` with no moniker, which reads every node's own header -- used to skip a board whose header lacked `node`, `name` or `role`, report nothing and exit 0, leaving the roster quietly one node short and indistinguishable from a node nobody had created. It now reads every header before the first write and refuses the whole roster, naming each such board's `wip.md` and the fields it lacks, so no partial roster is left behind.

- **A write that lands and then fails a later step exits 0 and says the write landed.** A whiteboard verb could commit its row, meet a locked store while landing the board's views, and return that failure as the write's own -- with a remedy that said to retry, which doubles the row for `wb add`, `wb ask` and `wb decide`, each of which appends one. The verb now exits 0 and warns that the write landed and which step did not run -- rendering the views, writing them, recording them in the file index, landing a board's views, or recording an `organize` run in the event log -- with the cause's whole chain and a remedy that says not to retry and names `intent st sync`, never the disk -> store direction, which would overwrite the change with the stale copy. An MCP reply carries the same under `notes`. A rollback that left the files torn is still the verb's error and now names each file it could not restore. And a write the store refused BEFORE committing leaves `.intentfiles` as it found it: `intent issues add`, the issue status verbs and a thread transition that lists its thread each left a register row behind with no store row under it.

- **A view an older Intent rendered is rewritten without the lost-edit warning.** The first write after an upgrade warned `overwrote bytes that were not the store's render -- an edit to a generated file is gone` for every view whose only difference was the footer's version stamp, and nothing had been edited. A warning that fires on the renderer's own stamp trains its reader to ignore it, which hides the day a real edit is lost. The overwrite check now asks the same predicate `doctor` asks for that exact difference, so a footer-only difference is rewritten quietly and every other overwritten file is still named.

- **A registered board that has not been migrated is reported as that, with `intent wb migrate` as the route in, and never blocks a commit.** `doctor` called such a board a generated view that differs from the model, named `intent sync --to-disk` as the remedy and warned that the remedy would discard a hand edit. It does neither: the remedy exits 0, the file is byte-identical afterwards and the finding comes back, because nothing renders over a hand-authored board until the carry. With the gate refusing on `doctor`'s verdict, an estate mid-cutover was wedged with no supported way out. The board is now an advisory -- uncounted, non-blocking, and saying that it and its inboxes are still hand-authored markdown that nothing renders over or compares against the model -- and its inboxes are skipped. In the same change, a differing issue view that `.intentfiles` does not declare gets a remedy that clears it: delete the view if nobody edited it, or add `ISSUE:<nnnn>` to `.intentfiles` and run `intent organize --apply`, which rewrites it.

- **`intent claude start <ws>`'s refusal for a node that is not on the board shows what a display name and a role look like.** The remedy named `intent wb register <ws> --name <display name> --role <role>` and showed neither value, and the launcher names the session `<project>-<ws>` -- so the session name was the natural fill-in, and a node was registered with the session name as its display name and its own moniker as its role, where every other node on that board read like `Control Claude` / `control`. The refusal now shows `--name "Control Claude" --role control`, says a display name and a one-word role rather than the session name, names the verb that lists the nodes that exist, and names `intent wb register ... --correct` for a node already registered wrong.

- **`intent upgrade` reports no whiteboard file as owed by a build that has not landed.** The whiteboard was the one member of the set the model claims and no build carries, and it stayed a member after `wb register` and `wb migrate` carried it -- so an upgrade printed `not yet carried -- the model claims these and they are still on disk: the whiteboard (intent/whiteboard/)` in every project, with or without a board, and enumerated every board file under a per-artefact heading, a list that grew with each migration's views and snapshots. The set is empty by delivery and every reader of it says nothing for an empty one, so neither line is emitted. The mechanism stays: the next class the model covers before a build carries it becomes a member again.

- **`intent sync --to-store` keeps a board row's identity, and says `nothing the store already held was overwritten` only when that is true.** The restore deleted every whiteboard row and re-inserted the boards from the extract, so a whole-project sync renumbered items and messages and restamped rows nothing had changed -- and changed a node's name and role -- while printing that it had overwritten nothing. The difference is now applied by natural key: a node by its moniker, an item by its node, kind and sequence, a message by its sender, recipient, stamp and body as a multiset. An unchanged row keeps its id and its stamp; a changed or inserted one takes `updated_at` from the clock while its other stamps are carried from the board. The same difference feeds the write and the preview, so the overwrite list names every board row a restore would change, orphan rows whose node is not on the roster included, beside the threads and issues it already named.

- **`intent st done` refuses while a work package is still not started or in progress, and names each one.** The close gate reads criteria, so a thread whose criteria all passed closed at exit 0 with a package nobody had started. The packages are read after the gate, so a thread that is BLOCKED is told that first, and the refusal names each open package with the two verbs that settle one. `intent fc` stays the human's override and does not read them; `todo done` inherits the refusal through the same setter.

- **A verdict has to fit the row it is recorded on, and `at green` is reachable only from `red`.** `at green` went straight from `to-write`, so a row could be recorded passing without anything having been seen to fail; `at na` landed on a test row and `at red` or `at green` on a non-test one, minting exactly the pair `intent doctor` reports as model-inconsistent. Green now has `red` as its only origin, a verdict that does not fit the row's kind is refused by name, and `intent at edit --kind` re-enters the status at the new kind's entry -- `to-write` for a test row, `n/a` for a non-test one -- rather than refusing, so a re-kind is still one call. A row created without a status starts at its kind's entry for the same reason.

- **`intent st done` and `intent wp done` warn when the unit's objective is still unwritten.** The condition was computed and never said, so a thread or a package closed in silence with nothing on the record saying what it was for -- while the `in-finish` skill promised the warning and told a reader to go and check by hand. The close still happens, because the objective is not a gate, and the warning names the address `intent set` takes to write it. The skill says the same now.

- **The pre-commit gate honours `.intent_critic.yml`'s `show_all: true` as `severity_min: style`.** Only the subagent prompts read the key, so a project that turned it on saw no change in what the gate reported and nothing said why. An explicit `severity_min` still wins over it, and the critic contract in the shipped rule library says the gate reads it.

- **`intent at new --covers` naming a criterion the thread does not have refuses with a remedy you can act on at a terminal.** The refusal borrowed the JSON door's variant, so a caller at a prompt was told to `PUT` json to a caller-assigned id or `POST` to a collection. The contract refusal has its own variant now, and its remedy names the verbs: `intent at lint` to see every finding, `intent ac list` to see the criteria that exist, `intent ac new` to create the missing one, `intent at edit --covers` to point the row at one that does. `intent at edit` shares the refusal, so its remedy changed with it; the error text is unchanged.

- **A seeded `usage-rules.md` names the project rather than carrying `[[PROJECT_NAME]]`.** `intent claude upgrade --apply` copied that template raw, so the first lines of a project's own DO / NEVER contract read as an unexpanded token. Every seeded file now goes through the one token expander; the token-free ones come back unchanged. A file already carrying the literal token is yours once seeded and is not rewritten.

- **`intent critic` says when a rule's proxy ran only in part.** In a mixed proxy block -- some lines the runner can execute faithfully, some outside its contract -- the refusal was dropped the moment one line survived, so the rule reported as if the whole proxy had run and the lines nobody asked were invisible. Such a rule is now recorded as refused, named as `PARTLY RUN` in the text report beside the `UNRUNNABLE` line, and carried in the JSON. The exit code is unchanged: a refused proxy is reported and never blocks.

- **`intent critic`'s own usage errors exit 2, the code its gate fails open on.** An unknown flag exited 1, which the shipped pre-commit gate reads as findings, so a flag skew between an installed hook and the binary blocked a commit under a report of findings that did not exist. A critic that cannot parse its own invocation is the gate's breakage rather than the developer's, so it now takes the fail-open code, exactly as its unknown-language refusal already did. Every other command keeps the exit-1 rule for a usage error.

- **The shipped rules' greppable proxies no longer fire on the form the rule prescribes.** `test-highlander-shared-setup` grepped for the fixture definition its own Good form writes, and `real-code-over-mocks` matched the Good and missed the Bad, so both reported findings against projects following the rule; the first keeps only the line that matches the Bad, and the second declares that no greppable proxy is authoritative for it, because a line match cannot tell an internal stub from an external mock. `no-silent-failures` matches the catch-all clause on its own line, which is how formatted Elixir writes the Bad form. `no-parse-ls` claims SC2045, SC2011 and SC2010 beside SC2012, so the headless run reports each signal its Detection lists. Every swift and lua rule declares its proxy rather than leaving the question undecided, so those languages' runs no longer report their rules as undeclared, and the `critic-<lang>` subagent is named as what applies them.

- **The pre-commit gate no longer lints a rule library's own Bad examples.** A rule's example files are fixtures for that rule, so committing a change to a library linted them against itself and `--no-verify` was the only way through. A staged file under the library the run loaded -- canon or an extension root -- is now skipped and named, in the text report and in the JSON, rather than skipped in silence. A file named with `--files` is still read, because that is an ask.

- **`intent claude skills`, `intent claude subagents` and `intent claude rules` say what each of their verbs does in `--help`.** Their verbs come from a values slot that built each one with no description, so the help listed bare names against empty text and a reader had to guess or run one. The register carries a help line per value and the spine sets it, in v2's own usage wording; `intent surface retired`, the one other blank leaf, says what it lists. A test walks the built surface at every depth and names any subcommand that would ship with no description.

- **`intent st show` prints the thread's objective, or says it is not yet written.** The one sentence saying what a thread is for lived in the thread's `info.md` and in nothing the command printed, so the quickest read of a thread was the one read that left it out. The MCP resource read shares the renderer and prints the same bytes.

- **The cost-analysis skill counts Elixir comments, and the TCA report prints the dedup rate it computes.** `cost-metrics.sh` took `--` as Elixir's comment prefix, so an Elixir estate's comments counted as nothing; it counts `#`. `tca-report.sh` computed a dedup rate and then rendered the row as `?`; the row carries the value.

- **A project too old for this build is named as what it is, and every remedy names a release that exists.** A pre-v2.10 estate -- its config at `.intent/config.json` -- was answered `no Intent project found`, so the obvious next move was `intent init` over a project that was already there. It is now named as a pre-v2.10 project, and the remedy is the two hops: bring it to v2.19.0 with Intent v2.19.0, then run `intent upgrade` with v3. A project stamped below the v2.19.0 migration floor was told to `install intent@2`, which no tap carries; that remedy names the v2.19.0 release and its URL. The commands print the error's own remedy rather than one sentence for every shape of absence, and the user's own `~/.intent` is user state that is never read as a project.

- **`intent sync --to-store` reports an unstaged deletion of a carried attachment as a deletion, and says when HEAD holds its bytes.** It asked git which tracked files differ from the index and labelled every hit as edited and unstaged, so deleting a carried attachment -- the ordinary way to let a closed thread dehydrate -- produced a warning that bytes HEAD held were in no commit. A deletion now reports as one, naming whether HEAD holds the bytes; an edit still reports as modified. The warning's closing clause also said the commit gate would refuse: it does not, and now says so, because `doctor` reports attachment drift as advisory. A promised consequence that does not follow teaches an operator to read past the next warning.

- **`intent upgrade` removes the stale views of every thread it leaves undeclared, and names each path.** Two correct rules left those files with no owner: an undeclared thread's views are not rendered, and `organize` will not remove a view it cannot re-render byte for byte. A view an earlier Intent rendered for a thread the upgrade had just undeclared was therefore re-rendered by nothing and removed by nothing, and the operator was handed a pile of files to delete by hand on an estate the tool had just called upgraded. The dehydration gate now accepts exactly one difference, the footer's version, through the predicate `doctor` and the overwrite check already ask, and it accepts it organize-wide rather than as an upgrade-only exception; `upgrade` then runs organize's own plan narrowed to those views, reporting each removed path and each refusal in organize's own words -- a hand edit, an unmet precondition, a withheld thread -- and bypassing none. A view whose body differs by a single byte is still refused and named, and the narrowed plan carries no v2 prune, so an ingest and a removal stay two runs.

- **`intent upgrade` realises the view of a converted open issue and not of a closed one.** It wrote a file under `intent/issues/` for every issue it converted while declaring only the open ones, so the next `intent organize` listed the closed ones as unclaimed and removed them: correct at every step, and an estate that reported files to remove the moment it reported being upgraded. The realisation asks the one predicate the projection and the skew check use, so a closed issue's view is no longer written for the next run to take away. Its canon stands and `intent issues show` reads it.

- **`intent init` writes the `.gitignore` rules that keep the store out of git.** Only the migration converged them, so a project born on v3 was the one shape where `git add .` staged `intent/.cache/intent.db`, which never enters history. A fresh project now ignores `intent/.cache/`, `intent/events.jsonl` and `intent/.backup/` by path from its first commit, through the one converger both doors call, and never as `*.db`, which would swallow a database the project wants tracked. The rules are appended, so an existing `.gitignore` keeps everything it had.

- **`intent edit` on a criterion, a test row, a work package's cover or an issue answers about the thing you named.** An `ac` or `at` address printed the thread's `info.md` at exit 0 -- a file the row is not in -- so `--path` sent an editor at bytes the next render would replace; it resolves to the thread's generated `acceptance.md` now and takes the refusal every generated view takes, naming `intent ac` and `intent at`. A work package's cover refused with a remedy naming `intent wp`, which has no writer for a package's prose; it names `intent set` with the package's address. And `issue` has left this verb's kind roster, because an issue's one file is a view rendered from the store: `intent edit issue <id>` refuses and names `intent issues edit <id>`, while `intent browse` keeps the kind.

- **`intent st show <ID> <file>` prints the file it names, read from the store.** The argument is declared on the register with the values `info`, `design`, `impl`, `tasks`, `acceptance` and `all`, and the command reference renders that declaration as the verb's contract -- but the dispatcher read only the id, so every value printed the same three-line cover at exit 0, a request discarded and reported as served. `info` is still the cover; `design`, `impl` and `tasks` print that document as the store holds it, so a completed, dehydrated thread answers where there is nothing on disk to read; `acceptance` prints the rendered contract; and `all` prints the cover then each file the thread carries under a `-- <file>` separator, in the declared order, leaving out the ones it does not carry. **A named file the thread does not carry refuses at exit 1** and names `intent st attach`, a value outside the declared set refuses at exit 1 and names the set, and an attachment carried as an opaque file whose bytes this read did not load refuses at exit 2 rather than printing nothing as success. The MCP `intent_st_show` tool is unchanged: it returns the whole thread and still refuses a `file` parameter by name.

- **An address naming another project is refused by name, and the refusal spells this project's own address.** `intent://<other>/threads/ST0001` parses, and until now each door did something different with it: the MCP `resources/read` returned THIS project's ST0001 byte for byte, the silent form of the defect; `intent browse` and `intent edit --browser` resolved the id here and went looking for a daemon; `intent edit` answered with this project's not-found, a true sentence about the wrong project; and `intent set` promised a project registry that does not exist. One check now runs first in every facade door that takes an address, and in the MCP resource read and the browser resolve, so they refuse in one voice -- naming the url, the project it names, and the same address with no authority, which is this project's spelling of it. Another project's artefacts are reached from inside that project.

- **`intent organize --apply` and `intent st hydrate` write an attachment that is not UTF-8, byte for byte.** A file under a thread that is not text is carried into the store as bytes, and both verbs handed their write loop the attachment's inline TEXT, which such an attachment has none of -- so the step was passed over and then reported as `hydrated:`, a path named as realised that was never written. The dehydration gate then failed the working copy as a file it could not read, when the question was whether the copy matched. A step now carries the store's bytes in whichever form the store holds them: hydration writes them unchanged, the gate compares bytes so an identical binary copy passes and a changed one is refused as an edit, and **a step that must write and holds no bytes is refused by path** -- naming `intent sync --to-store`, which loads an opaque attachment's bytes from its canon sidecar -- rather than counted as hydrated. Writing an empty file would have been worse: present, and wrong.

- **`intent organize --apply` removes a thread's files together, or none of them.** Each removal went through the hand-edit gate on its own, so a thread whose cover the gate refused kept its views and lost its attachments: a tree left half-dehydrated, with the authored files the store is the only copy of gone and the generated ones it can rebuild kept. Every removal is now gated before any of them runs, and one refused file withholds the rest of its thread, reported as one refusal naming the thread and the files it held back beside the refusal that stopped it. An issue's view stays per file, because an issue has one.

- **`intent st reopen`, `intent st start` and `intent st resume` write a thread's attachments with its views.** Reopening a completed, dehydrated thread re-declared it and realised its cover, its contract and each work package's cover, and left the documents attached to it -- a `design.md` and its kin -- to `intent organize`. The next commit was refused by the canon gate, which names bytes the staged index does not contain, and the operator had to run `intent organize --apply` to learn what was missing. A verb that re-declares a thread now realises it through the same path `intent st hydrate` uses, so the same files land and the next commit's gate has them. The status and the list edit land first, so a realisation that fails is a warning naming `intent organize --apply` rather than the verb's own refusal.

- **Re-kinding a criterion leaves a state its new kind can hold.** `intent set intent:///threads/<ID>/ac/<AC> kind non-test` wrote the kind and kept the state, so a computed criterion became a non-test row in a state only a test-backed one can be in: `intent doctor` refused the canon, `intent ac satisfy` refused the row, and only withdraw-then-reinstate moved it. A state the new kind cannot hold now re-enters at the state a criterion of that kind is created in -- computed flipped to non-test lands unsatisfied, a noteless unsatisfied row flipped to test lands computed -- and a flip that would lose something is refused with the verb that clears it first: a recorded satisfaction names `intent ac unsatisfy`, and a note names `intent ac edit`. `intent ac new` takes its entry state from the same place.

- **A v2 criterion with an evidence clause and no acceptance test migrates as the authored criterion it is.** A row reading `-- evidence: <ref> -- satisfied: no`, with no `(non-test)` marker, was migrated as test-backed and computed, and its clause was dropped: `intent ac edit --note` refuses a test-backed row, `intent ac satisfy` refuses it by design, and no verb re-kinded it, so the only repair was a hand edit of canon. The migration now reads such a row as non-test when no acceptance test covers it and its own text names none, so it arrives unsatisfied with the clause as its note, and the run's finding says the row was read that way, with its file and line. A row that names an acceptance test, or that one covers, keeps the test-backed reading. The migration guide says how to repair a project converted by an earlier build.

- **`intent critic` outside a project refuses, as every other verb does.** The rule library resolves from the install rather than the project, so a run in an uninitialised directory produced a report a reader could not tell from a project's -- findings, a census and exit 0, with nothing saying that no project had been read and no `.intent_critic.yml` consulted. It now answers the refusal every other verb gives, the project error's own rendering with its remedy, at exit 1. **Critic's own usage refusals come first and still work anywhere**: an unknown language, an unparseable severity or a `--format` this verb does not serve exits 2 inside a project or out of one, and `intent critic --languages` still answers the roster anywhere. A project whose config will not read is warned about rather than refused, because the pre-commit gate runs this verb in every estate.

- **`intent claude upgrade --apply`, `intent agents sync` and `intent agents init` record the root files they write in the act that writes them.** All three wrote `AGENTS.md` and `CLAUDE.md` straight to disk, so the file index kept its record of the previous bytes and the writer left its own index behind. Under a running `intentd` the canon watch then read the rewrite as an edit from outside Intent and ingested it, which is how the gap was found. Each door now writes and records together, keeping the paths the index's own corpus covers -- canon also writes hooks and settings it holds no row for -- and a dry run writes nothing and records nothing. A canon step that fails is carried as a typed refusal with a remedy for its kind rather than as a message.

- **`intent claude upgrade --apply` gives a project the formatter exclusion it is missing, and names each pattern it adds.** Intent keeps a formatter off its generated views by appending their patterns to `.prettierignore`, and only two things ever wrote them: `intent init`, and the last act of the v2 migration. A project that took canon before that pattern list grew to cover issue and whiteboard views never received the rest, no later verb converged it, and nothing reported the gap -- so a project whose own pre-commit block runs a formatter over staged markdown had a second writer of its generated views, and the doctor gate refused every commit from the one that landed the rewrite onwards, whatever the commit touched. The verb a project takes canon by now converges the exclusion and prints each pattern it appends; a dry run names what it would add.

- **A view-skew finding whose difference is one a formatter makes says so.** `intent doctor` reported a generated view a formatter had rewritten exactly as it reports a hand edit, and told the operator that regenerating would discard work -- for a difference nobody had authored. When the view and its render compare equal once blank-line runs are collapsed and single-asterisk emphasis is rewritten with underscores, the finding now says a formatter is the likely second writer and names the command that adds the generated views to `.prettierignore`. It is a hint on the finding and decides nothing about whether the view is skewed, because a hand edit can take the same shape.

- **The v2 prune reads a v2 issue's id, so an upgrade removes the v2 tree it has carried.** v2 quotes an issue's id in its frontmatter, as `id: "0015"`, and its own template writes it that way. The converter stripped the quotes; the prune's verdict did not, so on every real migration it withheld every v2 issue file with the reason that the file carried no readable id and named no issue this store could hold -- false, since the store had just carried each of them. One withheld file stops the whole prune, so a first migration pruned none of its v2 tree, thread buckets included, and `intent organize` then listed the same files as unclaimed. Both now read the field through one reader, so the two cannot answer differently about the same bytes.

- **A Homebrew install lists the `agents` plugin.** `intent plugin list` reads each plugin's own manifest from the install root, and the support archive carried the `claude` plugin's tree while leaving the `agents` manifest behind -- so a keg answered about one plugin where a source tree answered about both, and `intent plugin show agents` had nothing to show. The archive now carries it.

- **The pre-commit gate's two broken-install refusals name the command that repairs them.** The shim finds Intent through a pointer file, and when the pointer resolved to something that is not an install, or to an install whose gate script is missing, both refusals said to reinstall Intent -- which writes no pointer, so following the remedy left the commit refused exactly as before. They now name `intent bootstrap`, run from the install the pointer should name, and the missing-gate case also offers restoring that install's hook templates. This is the class the v3.0.2 note fixed for a first install, in the two remedies it did not reach.

- **A thread's objective and context, and a work package's objective, have one stored form whichever door wrote them.** `intent set <address> objective --from <file>` kept the file's trailing newline while the `info.md` read-back trimmed it, so one field held two values depending on how the store had been filled: anything that fills a store from the realised tree -- `intent sync --to-store`, or a fresh store -- rewrote those canon files by a byte per field with no change of content, and a reviewer checking that canon moved only by the change in hand had that to rule out first. The stored form is now the text with the newlines at either end removed, applied wherever text enters the model, and the read-back keeps a first line's indentation and a trailing hard break's spaces, which the old trim took away with the framing.

- **A store write waits for the writer lock rather than being refused the moment another process holds it.** An edit of an existing record could fail with `database is locked` in milliseconds, with nothing left holding the lock by the time anyone looked, while a create beside it waited and landed: rusqlite's default transaction is DEFERRED, the compare-and-swap reads before it writes, and SQLite refuses a read transaction's upgrade to write at once instead of running the busy handler. Every transaction now opens IMMEDIATE through one door, so a contended write waits out the store's five-second wait. The long holder is gone with it: the prose index's canon half was deleted row by row under FTS5 secure-delete before a rebuild re-derived the whole index anyway, inside every mutation and every ingest, and secure-delete is now off across that delete and rebuild and on again inside the same transaction. The index ends identical and the writer lock is held for a fraction of the time.

- **A store busy past its wait says that nothing was written and nothing is damaged.** A lock held longer than the five-second wait reached the daemon's ingest as "could not read the committed canon", whose remedy was to fix the artefacts named above and run `intent doctor` -- which had nothing to list, because nothing was wrong with any file. The classification has one home in the store now, and the remedy says another process held the write lock for longer than this command's wait and to re-run once that write has finished. A store cause an artefact CAN produce, such as a constraint a malformed canon breaks, keeps the artefacts remedy. The store carried no busy remedy at all before, so a busy store met through a direct store error was told not to delete the store; it gets this one.

- **`intent app status`, `intent app start` and `intent app restart` see the running Intent.app on macOS 27, and a launch is given a deadline rather than a count of polls.** On macOS 27, `lsappinfo info -only pid <bundle id>` prints the whole record template with `[ NULL ]` for every other field and the pid in the record's own spelling (`pid = 96907`, unquoted), and the parse split on the first `=` -- which was `bundleID=[ NULL ]` -- so `app status` reported an app that was up as not running, and every reading a launch waited on could never succeed. Each field is now read by its name in either spelling, quoted or bare, and an empty value or `[ NULL ]` reads as absent rather than as a value. The launch wait is a deadline of 15 s, polled at the same interval and returning the moment the app registers with LaunchServices: a relaunch straight after a quit took longer than the old 2 s, and `app start` and `app restart` then reported a launch that had succeeded as a failure and sent the operator after it. When the deadline does pass, the error says the app did not register within it and the remedy is to check `intent app status` before looking for a failure. Every app lifecycle failure now prints its remedy.

- **`intent app start` tells an installed Intent where to get Intent.app.** With no bundle to launch it named `bin/devbin macos app-build`, a tool only a source tree carries, so the single remedy on offer could not be run by anyone who had installed Intent rather than cloned it. The remedy is chosen by the install root the binary resolved: a tree carrying `bin/devbin` is sent to it, and any other install is sent to `Intent.app.zip` from its own release, with the release URL. `intent app restart` shares the choice.

- **Intent.app no longer hangs on a command that fills its stderr, and Restart intentd runs the shipped restart.** The app read a command's stdout to end-of-file before touching stderr, so a command that filled the stderr pipe before closing stdout deadlocked the app against its own child. Both pipes are read at the same time now. Restarting the daemon from the menu also ran `daemon stop` and then `daemon start` as two commands the app sequenced itself; it calls `intent daemon restart`, which sequences them and waits for the new daemon to answer.

- **The TCA skills write to the store.** `in-tca-init` made `WP/NN` directories and hand-wrote an `info.md` in each, a shape v3 reads nowhere: the directories existed and no work package did. Each work package is now created with `intent wp new`, its id is checked against the one that came back, and its body is written with `intent set`. `in-tca-finish`'s pre-flight counted unchecked `- [ ]` boxes in a view that is rendered from the model and carries none, so it could never refuse; it asks `intent ac gate` for the thread and names every criterion that is not satisfied.

- **With the detail pane open, the explorer's list scrolls to the rows below its half.** The scroll was taken against the whole body while the list was painted in the half a split leaves it, so the cursor walked into rows nobody had drawn: the selected row, and everything after it, could not be brought on screen. The list's height has one home now, and the scroll, the painter and the page keys all read it.

- **The canon-ignore guard judges the ignore rules the commit carries, not the checkout's.** It read its rules from the worktree and attributed the added lines from the index, so whenever a staged `.gitignore` differed from the checked-out one -- routine on a shared tree, where a re-staging formatter is normal -- it compared line numbers across two different documents, and a staged rule that would un-commit the canon estate could pass unmatched. Every ignore file the index holds is now checked out into a scratch tree and git's own matcher is pointed at it, so both the rules and the lines they are attributed by are the commit's. A scratch tree that cannot be built refuses the commit, because checking the wrong rules is worse than saying so.

- **The Rust critic's `Result<T, String>` rule reads a `Result`'s own error type, not the last parameter of a generic inside it.** `collect::<Result<BTreeMap<String, String>, rusqlite::Error>>()` was refused at the pre-commit gate for the `String>` that closes the map's type parameters, so code whose error type was a real error type had to be recomposed to get past a rule it had not broken. A `Result`'s first parameter may now hold no angle bracket in the pattern, so a matched `, String>` or `, Box<dyn Error>>` always closes the `Result` itself. The narrower pattern can strike no line the old one did not, here or in any consumer, and the rule's Detection section states what it gives up.

- **An in-place edit in the explorer takes the whole line editor, a locked field opens no edit at all, and a rows pane follows its cursor.** Editing a field, only typing and Backspace worked -- the arrows, Home, End and Delete did nothing, which is what hv reported editing a title. The field now holds the composer's own line editor through the one key dispatcher: the arrows, Home and End, Delete, the emacs chords and vi's normal mode all act on the value, a chord nothing binds types nothing, and the caret is drawn where it actually is; under vi the first Esc enters normal mode and the second discards the edit. Enter on a row its form marks not editable no longer opens an edit that could not be saved -- it says why on the info row and the hint offers no edit verb there. A rows pane draws its cursor and scrolls to keep it on screen, as the list does. An item whose lookup fails renders the refusal on an error row instead of a screen of empty values.

- **A verb that takes a steel thread id names the whole address you typed.** Handing one a URL -- `intent:///threads/ST0001` -- split it at the first `/` and refused a fragment nobody had typed: `` `intent:` is not a steel thread id ``. The address is now recognised before the split and refused whole, with what the verb takes and a worked example in the reader's own vocabulary; the example is never an id read out of the URL, which can name another project's thread. Every door through that reader moves with it -- the CLI's thread verbs, `st hydrate` and `st dehydrate` among them, `sync`'s ids, and the MCP thread tools.

- **`intentd`'s background sync no longer writes a render taken before a command-line write over that write.** When a command-line write landed between the sync's snapshot and its file commit, the sync wrote its older render over the write's canon file and view and recorded those bytes as the store's own, silently: disk held the subject as it was before the write while the store held it as it is after, and the next write to that subject warned that an edit to a generated file was gone. The sync now reads the store's change counter before its snapshot and writes only under the writer lock, only if no other write has landed since. If one has, it renders again, and after three tries it gives up and logs the files it would have changed rather than writing any of them.

- **A search that meets a store fault says the search could not be answered, instead of blaming the query.** Every SQLite error on the search path was reported as a malformed query, whose remedy sends the reader after an unbalanced `(`, so a damaged index, a busy database or an I/O fault each told the reader their correct query was wrong, and every retry seemed to confirm it. Only an expression FTS5 itself refuses is now the reader's; anything else is a store fault, reported as such with `intent index rebuild` named where a rebuild was driven to cure it. `hello NEAR` is answered rather than refused.

- **`intent wb migrate`'s help says what the live bounds do after a carry.** It said a migrated inbox over its bound refuses new sends until its owner clears it, which stopped being true when carried messages left the live count. It now says the two bounds differ, and why: a migrated inbox refuses no sender, because the message bound protects a sender from someone else's backlog, while a migrated board over its item bound still refuses its owner's next `add` until something is archived, because the board is the owner's own.

- **A view an older Intent rendered is no longer read as a hand edit when only the renderer's own text differs.** The checks that guard a generated view compared whole files, so a view whose footer or Acceptance cover paragraph an earlier release worded differently read as a possible hand edit: `intent doctor` reported it as blocking `view-skew`, which the pre-commit gate refuses, `intent organize` refused to dehydrate it, and `st sync --write` read a two-byte footer change on an empty estate as the store being behind. A project upgraded from v3.0.3 met the first on its two project-level views, whose footer's wording changed, with nothing edited. All of them now mask the banner line and the cover paragraph the renderer writes, so that difference reads as a stale render, while an edit to authored text, or text appended after the banner, still refuses.

- **A new project's first commit is no longer refused by the gate it just installed.** `intent init` wrote neither `intent/st/steel_threads.md` nor `intent/todo.md`, and `doctor` counts a missing aggregate view as skew even on a project with no thread, so after `intent claude upgrade --apply` wired the pre-commit gate, the first `git commit` of every new project was refused with `generated view is missing` and a remedy the new user had to work out. `intent init` now writes both views, and a fresh project commits clean.

- **A store that will not open is a finding.** A store file that exists and will not open, or opens and cannot be read back, was passed over by `doctor`. Every other verb refused on it with a remedy telling the operator to run `intent doctor`, which printed `0 finding(s)` at exit 0. It is now the counted finding `store-unreadable`. A store that is only busy stays an advisory.

- **A write no longer creates `intent/.canon/project.json` when there is nothing to record in it.** Every write added the file, cutoff or not, and `intent init` writes none. So a first write on a branch created it and it was committed there; switching back removed it; the next write recreated it untracked; and the next merge of the branch refused to overwrite it. The `post-checkout` hook's sync made that happen on every branch switch. The file is now created only when there is a todo cutoff to record, and a file that exists is still rewritten.

- **`intent upgrade` writes no `intent/events.jsonl`, and its list of files naming a v2 path no longer lists records Intent writes.** Every v3 upgrade re-emitted canon through a list that ended with an empty single-file event log. An ignore rule hid that file until this release retired the rule, and it would then have appeared as an untracked file. No verb writes it now, and `intent upgrade` removes the empty one an earlier upgrade left, saying so, once it has retired the rule; one that git tracks, or that holds bytes, is left where it is and named with why. The list of authored files naming a v2 bucket path now skips everything under `intent/.canon/`, event files included, and the whiteboard's records and rendered views. Those are records Intent re-derives, where a reword is overwritten or rewrites history. A node's own files beside its board stay on the list.

### Removed

- **`intent claude rules index` is retired, and the family is a read.** v3 embeds the rule library in the binary, so the verb had no installation to regenerate an index in -- and the index it wrote, with its generator and its template, was wrong as well as unread. All three are deleted. The spelling is refused as an unknown subcommand rather than as a declared verb that is not built, `intent claude rules --help` lists `list`, `show` and `validate`, and because those three write nothing the register calls the family a read: the agent guide's safety line for it says `read` where it said `mutate`.

- **Artefacts nothing in v3 reads are deleted**: the module-check and critic-guard hook templates, which were in no guard roster; the dead `llm` and steel-thread templates, which `init` carried embedded while declaring that it writes none of them; and the subagent manifest `global-agents.json`, which no v3 code read and whose checksums were empty. No command behaves differently; what goes is weight in the install and rows that had to be explained.

- **`intent at new` takes no `--status`: a new acceptance test starts at its kind's entry.** A create that can name a status can name `green`, and green is reachable only from red, so the flag minted a passing row nobody had seen fail -- around the red-before-green edge the verdict verbs enforce. Leaving it off already gave the kind's entry, and that is now the only status a create can produce: a test-backed row starts `to-write` and a non-test row `n-a`, with the verdict recorded afterwards by `intent at red`, `intent at green` or `intent at na`. The flag is gone from the parser and from `--help`, and the MCP tool refuses a `status` parameter by name rather than accepting and ignoring it.

## [3.0.3] - 2026-09-14

**v3.0.3 is the v3.0.2 product, published.** The v3.0.2 tag was cut and its GitHub release created, but its artefacts were never published: the release pipeline's support-tree guard refused the 3.0.2 tree at the tag, and a stage taken anywhere but a clean checkout at the tag is refused by `int macos publish`, correctly. v3.0.3 carries the guard fix and nothing else that changes behaviour. **If you are on v3.0.1, read the [v3.0.2 notes](docs/releases/3.0.2/RELEASE_NOTES.md) -- everything in them ships in this release, including the Upgrading section.**

### Fixed

- **`int macos prepare` stages the support tree again.** Its guard scanned the source for every `home.join(...)` chain and took each one as a path under the install root. 3.0.2 added chains that are not: the XDG data directory, the legacy `~/.intent`, `Library/LaunchAgents`, and the whiteboard's board home. The guard now reads only install-root chains, and the two install-root parameters that were named `home` are named `install`. Release tooling and two parameter names; no behaviour change in `intent` or `intentd`.

## [3.0.2] - 2026-09-14

Every document in the repository was checked against v3.0.1 as built and corrected where it disagreed, and the defects that audit found are being fixed. What is fixed is below; [Known defects](docs/known-defects.md) carries what is not, and says which release fixed the rest.

### Added

- **`intent search` answers one envelope, and it says how fresh it is.** A search returns its hits grouped by tier -- lexical text, and structural symbols -- ranked within a tier and never blended, with both denominators (what matched, what came back) and the index's own freshness beside them. When the index is behind or a path was skipped, the answer says so and names the paths, so a result is never a confident subset. `--json` and the MCP tool return that same value from the same call.
- **`intent search --sql <statement>` runs ONE read-only statement over this store.** It answers a class of question no verb does -- joins across the model and the index -- on a read-only connection that cannot write whatever the statement says, bounded by a row cap and a work budget, with both denominators reported. A second statement, a write or a state-changing pragma is refused by name.
- **`intent search --kind def <name>` answers whether a thing with that name already exists**, from the tree rather than from a registry someone remembered to update. Symbols come from each language's own tree-sitter tags query, so a language is a grammar and nothing else; references are name-matched occurrences and every surface says so, because nothing here resolves a name to what it points at. **Four grammars ship in the binary** -- rust, elixir, swift and lua, each one's byte cost measured against a baseline before any of them shipped; `shell` is declared and off, because `tree-sitter-bash` carries no tags query and would name no symbols for its bytes.
- **`intent search --outline <path>` lists a file's symbols with their spans, and `--context <name>` gives a definition with the places that call it.** These are the answers grep cannot give: they replace reading a whole file with reading one span, which is what an agent asking _what is in here_ actually needs.
- **`intent index status` says what the index holds and every path it will not hold, with the reason -- and names the paths rather than counting them.** `intent index rebuild` walks the scope and rewrites what it holds. A status reads the rows and never walks, so an operator comparing the two can see when the index is behind.
- **The index holds the repository, not just the model.** Prose on disk -- the README, the docs tree, the boards -- and source in every language the project declares are indexed alongside the store's own entities, with each exclusion a named row carrying its reason rather than a silence.
- **`/search` in the explorer is a resident pane.** It used to lend the terminal to the CLI, so results printed to the screen and were gone on the next repaint. The hits are rows now: Enter on an entity opens its view, Enter on a file opens the file, and the freshness line sits where the reader meets it before trusting the list.
- **The MCP tool descriptions say when NOT to use the tool.** A model choosing between the index and grep needs the half a help line never carries, so the register row that describes each tool now carries it, and the tool a session sees says it.
- **`intent doctor` runs on the pre-commit gate, and its classes say which ones are worth a refusal.** Nothing between closing a work package and the release preflight ever asked whether an estate still agreed with its store, so findings accumulated and every commit went in over them. The gate now runs `doctor` after the guards and refuses on its EXIT CODE alone -- it never parses doctor's prose -- and `doctor` splits its own classes to carry that verdict: a generated view that is missing or hand-edited blocks, and so does canon that will not parse, a schema that will not validate, conflict markers and duplicate ids. A status decision the human owns, an attachment the store has not seen, a backup that is behind, a view rendered by an older Intent and a project that has not migrated do not: those are estates that are early or mid-decision rather than broken, and refusing them would enforce things the gate is not there for. **`doctor` now answers three codes rather than two**: 0 clean or advisory only, 1 blocking findings in an estate it read, and 4 an estate it could not judge at all -- no project here, a config that will not parse, or a project that has not migrated to v3. The gate fails OPEN on anything that is not 0 or 1 and says the estate went unenforced, so installing it does not stop a v2 project from committing; the release preflight aborts on any non-zero, which stays right from the other direction, because a project the tool cannot judge does not cut. There is no opt-out flag; `--no-verify` remains and leaves a trace.

- **The canon routes a prior-art check through the index.** Every skill, template and rule that told a model how to check whether something already exists now names `intent search --kind def <name>` first and grep as the fallback for when the answer says the index is not complete -- with a project's own module registry searched as well, where it keeps one.

- **`intent search` answers from a running daemon, and answers the same thing either way.** `intent --daemon search <text>` asks the daemon, which calls the same query the local path calls and returns the same envelope, so the two answers are identical for the same tree; only where the index last caught up can differ, and the answer says that itself. Without a daemon the query now reconciles the index against the tree before it answers, because nothing else is keeping that index current -- with a daemon the watcher does it continuously, so no query pays for it. `--no-reconcile` asks the other question: what the index HOLDS, answered as it stands, with every path that has moved underneath it named rather than left to be discovered. `--daemon` on `--outline`, `--context` or `--sql` is refused by name, because no daemon answers those and a flag accepted and ignored is worse than one refused.

- **A Claude Code session's own grep becomes a door into the index.** `intent claude hook post-tool-symbol-context` runs after a Grep whose pattern is one symbol and appends what the index knows about that name -- where it is defined and where it occurs, as source spans -- so the model gets the answer grep cannot give without asking for it. It only ever ADDS: the grep has already run and its result stands, and the hook exits 0 whatever happens. **It appends nothing when the index cannot answer for the paths its own answer names**, which is the whole safety of it, and that rule is one shared shell function rather than a reading each hook grows for itself -- `index.complete` is the whole index's claim, so one unreadable file anywhere would silence an answer about a path nowhere near it. A pattern that is not one identifier is never answered, because grep's job is text and the index answers about names. Shipped and off by default, like the critic advisory: turning it on changes what every session in a project sees after every Grep, so it is the project's decision and the script says how to make it.

- **A whiteboard node is a row in the store, and its board and inboxes are views rendered from those rows.** The board protocol was markdown each node hand-wrote, held in shape by nothing but the node's own discipline. `intent wb register` puts a node into the model, named from its arguments (`wb register <moniker> --name <display> --role <role>`) or read as a roster from each node's existing board header. From there every change is a verb, and each kind of item has one door: `wb add` writes `doing`, `todo`, `hold` and `watchout` items and refuses `decision`, which `wb decide` writes; `wb archive` retires an item; `wb ask` sends to one node and `wb announce` to every other; `wb clear` marks one sender's messages handled; `wb claim` and `wb unclaim` keep a node's claims; `wb pickup`, `wb touch` and `wb release` mark a node active, alive and paused; `wb status` lists the nodes and `wb show` reads one whole board, listing the messages still live and counting the handled ones, which `--all` lists. **Every write lands its rendered views on disk as it commits**, so `intent/whiteboard/<id>/` is generated the way a thread's views are: a hand edit to a board or an inbox is skew to `intent doctor`, and the next write replaces it. **The bounds refuse rather than truncate.** An entry body over the size bound, a message into an inbox already holding its live bound (an inbox being one sender and one recipient), and an item of a kind the node already holds its live bound of are each refused by name, with the bound in the refusal and nothing written; `wb announce` checks every recipient before it writes to any, so a broadcast never half-lands. The human's node is unbounded by default. **Every stamp is read from the service's clock at the write**, and no caller supplies a time. **A node registered from its header refuses every board write until `intent wb migrate` has carried its markdown**, so no render lands over a hand-authored board the model has not read; a node named from its arguments has no such board and writes at once.
- **`intent wb migrate <node>` carries a hand-authored whiteboard board into the model, and names every line it does not carry.** The board protocol has been markdown on disk since it was written: a header block, item sections, one inbox file per sender, and a `.history/` directory of folds. This reads one node's whole directory and writes it as rows -- items of all five kinds, holds keeping the condition that releases them, inbox entries in source order, and each `.history/` file as a verbatim snapshot document rather than as items, because a fold's archive is a picture of a board at a moment and splitting it per item would manufacture a second history of the same node. **What it cannot carry it names, per item, with the file and line it was found at**: a section no kind maps, prose above the first section, a stray line above an inbox's first entry, an inbox whose sender is not on the roster, a `.history/` file that is not a document. A total that reconciles arithmetically would tell nobody which line stayed behind. **The stamps a board CLAIMS are carried verbatim into their own untrusted field** while the service stamps the ingest instant beside them, so a migration neither rewrites history nor puts a hole in the rule that no caller supplies a time. It applies no live bound and marks nothing handled: the bounds are a refusal on the next write, not a state the carry declares on a node's behalf. A board that already holds rows is refused by name rather than carried twice.
- **`intent claude upgrade --apply` declares Intent's MCP server to Claude Code.** A `.mcp.json` naming `intent mcp` is seeded when the project has none, so a session reaches the tools without anyone configuring it. A project that already has one keeps it untouched, including under `--force`, and `--skip-settings` now declines this file as well as `.claude/settings.json` -- one flag for the wiring Claude Code reads, because deleting a seeded file is not a way to decline it when the next run seeds it again.

- **`intent discover [fromdir]` registers the Intent projects under a directory, and `intent explore` registers the one it opens**, in one per-user file, `~/.config/intent/projects.json`. `discover` walks `--depth` levels (4 by default), honours `.gitignore`, does not descend into a project it found, and names each project it did not register with the reason. A running `intentd` watches the file and lists every project in it.
- **The explorer moves between projects.** `/projects` opens a picker of the registered projects, and `intent explore` outside an Intent project starts there. Enter opens a project, Esc goes back, and a project no longer on disk is shown as missing rather than opened.
- **`/threads` and `/issues` open their lists inside the explorer**, rather than lending the terminal to the CLI; `/issues` with arguments still runs `intent issues`. `/issues` lists open issues newest first, a rule, then closed ones, as `/threads` does.
- **The menubar app shows the daemon's port, state and thread count on one line**, where it used to take two rows, and shows no port or count beside a daemon that is not live.

### Changed

- **The documentation says what v3.0.1 does.** The README, the install, migration and known-defects pages, the command reference, the concept pages, the guides under `intent/docs/`, the skills, the subagents, the rule library, and the comments in the release scripts, the workflows and the menubar app were each measured against the build and rewritten where they were wrong. Hardcoded counts are gone from all of them: where the tool reports a figure about itself, the page names the command that reports it.
- **The templates `intent claude upgrade --apply` writes into a project say what v3 does.** The generated `AGENTS.md` and `CLAUDE.md`, and the seeded `usage-rules.md` and `.intent_critic.yml`, no longer point at documentation "at the Intent install", which a Homebrew install does not carry, and no longer describe `$INTENT_HOME` or v2's leading-zeros rule, or say that `intent critic` reads the file's severity threshold (only the pre-commit gate does). A project picks up the generated files on its next `intent claude upgrade --apply`; `usage-rules.md` and `.intent_critic.yml` are yours once seeded and are not rewritten.
- **The shipped hook scripts' comments** describe how v3 installs and runs them. Only comments changed; every hook behaves as it did.
- **The command register's prose** (`surface/dispatch-table.json`), which the command reference is generated from, is corrected: the exit codes stated for `INV-04`, argument notes that claimed behaviour the build does not have, and the `intent claude` verb list, which still named the retired `prime`. None of the changed text reaches a command's help or output, so no command behaves or reads differently.
- **The commit `intent --version` names is the one the binary was built at.** In 3.0.0 and 3.0.1 it was the newest commit touching the build's inputs, an ancestor of the commit you built at, so comparing it with your own `HEAD` told you nothing. `intent`, `intentd` and Intent.app now carry the checkout's `HEAD`, and a release carries its tag's commit, so checking a build is one comparison. A binary is still marked `dirty-` only when its own inputs had uncommitted changes. A commit that lands while a build runs leaves that build naming the commit before it.
- **The published schema faces under `schema/` carry 3.0.1.** At the v3.0.1 tag they still said 3.0.0. Their generator re-stamped them and only the version line changed; the release step that should have stamped them is unchanged.
- **Intent's per-user files follow the XDG Base Directory Specification**: configuration under `~/.config/intent/`, data (the install pointer, the skill and subagent manifests, `ext/`) under `~/.local/share/intent/`, `intentd`'s logs under `~/.local/state/intent/`, and its runtime files under `$XDG_RUNTIME_DIR/intent/` or `~/.local/state/intent/run/`. The first v3.0.2 command moves what Intent owns out of `~/.intent/` and removes it when that leaves it empty. A project's pre-commit gate installed by an earlier build reads the old pointer, so run `intent claude upgrade --apply --skip-settings` in each project after upgrading.

### Removed

- **`intent claude ws` is retired: a whiteboard node is a row in the store, not a directory on disk.** The family managed the board as FILES -- `new` scaffolded a node's directory and its inbox pairs, `list` read each `wip.md` header, `archive` moved a directory aside, and `hygiene` linted the header block a node hand-wrote. A node is now a row and `intent/whiteboard/<id>/` is the rendered view of it, so `intent wb register` creates one, `intent wb status` lists them, `intent wb show` reads one whole board, `intent wb archive` retires an item, and the two whiteboard guards plus `intent doctor` are what `hygiene` was reaching for. The spelling refuses at exit 2 naming its replacement rather than answering clap's generic unknown-command, so a script that calls it is told where the capability went. `intent claude start <ws>` is unaffected and still launches a session bound to a node -- it refuses a node that is not on the board instead of offering to scaffold one, and names the command that registers it.
- **The worker-bee extension seed** (`lib/templates/ext-seeds/`). Nothing in v3 read it, and `intent ext` is declared and not implemented.
- **Template trees nothing in v3 reads**: `lib/templates/archetypes/`, `lib/templates/issues/`, `lib/templates/prime/` and `lib/templates/_treeindexignore`, and the per-language templates under `intent/plugins/agents/templates/`, which the Homebrew install never carried.
- **`lib/help/`**, v2's help files. No v3 binary reads them, and the Homebrew install never carried them.

### Fixed

- **An idle daemon no longer holds a core.** A daemon with no client burned a core for seconds whenever anything under the repository moved, `target/` and `.git/` included, and for minutes as it opened a project. Every index refresh read every stored prose body and sorted them all, which SQLite does by spilling to temporary files, only to find the whiteboard's file names, and the watcher asked for one refresh per path in a batch. A refresh now reads only those file names, and a batch is one refresh. The index's file watcher also keeps no file-ID cache and passes on no change under an ignored directory or `.git`, so a burst of deletions under a build directory no longer holds the watcher thread or walks the repository. A refresh that names a file or a directory reads only what it names, and no longer rebuilds the search index. The store thread also waits for work on the standard library's thread parker rather than tokio's.

- **A daemon indexes the project it opens, not only the files that change after it starts.** Its index grew only as its watcher named paths, so a project opened over a quiet tree -- a fresh clone, or any checkout nobody was editing -- was indexed at the root and nowhere below it, and `intent --daemon search` answered from that index as if the rest of the tree held nothing. On opening a project the daemon now surveys the index once and refreshes each stale directory in turn, letting any waiting request go first; on an index that is already current the survey writes nothing.

- **A patch release no longer makes every generated view look hand-edited.** Every view's footer names the Intent that rendered it, so the moment an estate moved one release every view on disk differed from what the binary rendered -- and `doctor` reported each one as skew, saying it was either a hand edit or a moved store and warning that regenerating would discard work. Neither was true and there was nothing to lose. A difference confined to the footer's version is now its own advisory class, `stale-render`, cleared by `intent sync --to-disk`; the decision is taken before the skew check, so a view that was only ever re-rendered never reaches the blocking class. Every remaining skew finding prints the offset of the first differing byte, because the two lengths it carried are equal whenever an edit substitutes rather than adds, and an operator could not otherwise tell a hand edit from a renderer change without cloning the tree.

- **A repeatable filter written before the query no longer swallows it.** `intent search --kind def <name>` -- the spelling this project's own canon prescribes as the prior-art check -- refused with `nothing to search for`. A flag declaring an unbounded arity was built as a greedy list of values accepted once, which inverts both halves of what the row declares: the list had no terminator, so the POSITIONAL after it was read as another value, and a second occurrence of the flag was refused outright while the ellipsis in its own help line promised exactly that repetition. The same declaration governs `--tier` and `--lang`, and away from `search` it ate required positionals -- `at new --covers <acid> <STID> <ATID>` and `critic --files <path> <LANG>` both refused for arguments the caller had given. An unbounded flag now takes one value per occurrence and repeats, so `--kind def --kind ref` asks for both, and the surface renders `--kind <kind>` without the ellipsis, because the ellipsis was the greed. The undocumented space-separated form `--kind def ref` is refused by name rather than silently narrowing the query.
- **A document the store carries and the disk holds answers a search once, not once per corpus.** A thread's `design.md` is realised on disk AND carried in the store as an attachment, so one phrase on one line came back twice -- `file` from the disk prose corpus and `thread` from the store's doc sections, the same path and the same line, differing only in kind and owner. The disk corpus already excluded the store's PROJECTIONS by rule, which was right and not wide enough: a rendered view is produced by the renderer, while `design.md`, `impl.md` and `tasks.md` are authored, so they are not views -- and the store carries them anyway, which is what decides the question. The exclusion is now every path the store carries prose for, obtained by asking the renderer and the store's own attachment rows, so a document attached later is excluded on the day it is attached. Prose the store does not carry is untouched and is still the disk corpus's to answer. Reachable before this release through `intent index rebuild`; the daemonless search path now reconciles before it answers, which made it the default answer rather than a rare one.
- **`intent init` refuses a directory that already holds files it writes, and names every one.** It tested for `intent/.config/config.json` and nothing else, then wrote its starter content with no further check -- so running it where a `CLAUDE.md`, an `AGENTS.md`, an `intent/wip.md`, an `intent/llm/RULES.md` or an `intent/llm/ARCHITECTURE.md` already existed destroyed that file and listed it as created. The absence of a config makes a directory not a project; it never made it empty. Every destination is now checked before anything is written, a collision names all of them at once, and `.prettierignore` is not among them because its writer only ever appends.
- **`intent claude upgrade --apply` holds a `.claude/settings.json` it did not write.** It overwrote the file unconditionally, in the same function that already held a hand-authored `CLAUDE.md`, so a project with its own Claude Code settings -- permissions, a model pin, hooks of its own -- lost them to a command run to refresh its documentation. A settings file that has never carried Intent's hook door is now held and named; `--force` overwrites it and says so in its help; `--skip-settings` still skips. A file Intent did write is still updated, so a hook fix reaches projects as it always did.
- **`intent claude subagents sync` takes the `--dry-run` that `intent claude skills sync` takes.** Both families share one preview in one function, but only the skills row declared the flag, so on subagents it was refused at the parser as an unexpected argument. Since a `sync` now holds a subagent you have edited, the preview is how you decide whether to type `--force`, and it was available for one payload kind and not the other. `--force`'s help on that family also said it overwrites an agent manifest; it overwrites the subagent's own file.
- **`intent claude skills uninstall` and `intent claude subagents uninstall` name the files they deleted.** They reported the removed files as a count while naming the files they left behind, so the half of the line an operator could act on was the half that needed no action, and the paths that were gone were withheld. Both halves are now named.
- **`.intent_critic.yml`'s `disabled:` list is read when its key line carries a trailing comment.** The shape the documentation showed put the comment on the key line, and in that shape nothing was disabled and every rule stayed armed -- with no line anywhere saying so, because an empty opt-out list is a legitimate state. A project that opted a rule out by following the docs was being linted against it. On 3.0.1 the working shape is the comment on its own line under `disabled:`.
- **`intent critic` refuses a `--format` it does not serve.** An unknown format rendered text at exit 0, so a consumer asking for a machine-readable report got prose and a success code and had no way to tell. It now refuses at exit 2, which is what a usage error takes.
- **`intent critic` prints nothing before a refusal.** A run that was about to refuse printed an `ok:` summary line first, so the output said both that the run succeeded and that it did not. At exit 2 the refusal is now the whole output.
- **`intent critic` does not count a file it could not read as a file it checked.** Where shellcheck declined a file, the run folded it into the total it reported as examined, so coverage nothing had read was counted as coverage. The file is now reported as not run.
- **`intent claude skills uninstall` and `intent claude subagents uninstall` no longer destroy an edit you made.** They removed every file they had recorded writing without asking whether those bytes were still their own, so a skill or subagent you installed and then edited was deleted at exit 0, under a line reading `removed (N file(s))` -- the same line, from the same code, that a run destroying nothing prints. A unit whose recorded files no longer match their recorded checksum is now held, nothing is removed, and the exit is the one every other undecided state takes; `--force` removes it and names the checksum of what it discarded, which is what its help always claimed it did. A file you keep beside a skill that Intent did not install is not a modification of it and does not hold the removal.
- **A release cannot be tagged with schema faces stamped for another version.** The v3.0.1 tag carried `schema/*` reading `INTENT_VER: 3.0.0`, because the published faces are generated from the crate's own version at compile time and the release stamped the version without regenerating them -- so five schema tests fail at that tag. The release now regenerates the faces through their own generator, in the release commit, and refuses to tag unless every published face carries the version being cut. It also refuses at pre-flight if a tree's faces disagree with its own `VERSION` before anything moves.
- **A Homebrew install carries the subagents.** The v3.0.1 keg shipped without `intent/plugins/claude/subagents`, so `intent claude subagents list` answered `no subagents in this install` at exit 0 and there was nothing for `intent claude subagents install` to install -- the `critic-<lang>` family included. The support archive now carries that tree, and the release refuses to build one that omits a directory the binary resolves by name at run time.
- **A fresh install says how to finish it.** The pre-commit gate a project installs finds Intent through a pointer file (`~/.local/share/intent/home` from this release), which only `intent bootstrap` writes, so every commit was refused after a first install -- and the refusal said to reinstall, which writes no pointer. It now names `intent bootstrap`, and the formula says the same in a caveat. Homebrew cannot do it for you: its `post_install` runs with a throwaway HOME and cannot write yours.
- **A new project's `intent/wip.md` carries its own date and author.** The template stamped every project's `wip.md` with a fixed 2025 date and one person's name; `intent init` now fills in the date and the project's author.

- **`intent organize --apply` names every path it will remove before it removes one, and performs the plan it printed.** It computed and performed in one call and rendered afterwards, so the first time a path reached the screen it was already gone; `--apply --quiet` named nothing at all, because removals shared a predicate with the lines a quiet run is entitled to withhold. The plan is now printed first and the act is pinned to it, refusing if the tree moved in between -- a promise about which files go, made about a different run, is worse than no promise. A removing plan asks on a terminal and says it is proceeding off one, so a captured log carries the sentence standing between the plan and the removals; it asks only when something goes. The preview also predicts the directories a removal empties, which ran only under `--apply` before, so a preview said nothing would be pruned and the apply removed directories the plan had not named. `--quiet` may withhold what a run wrote and may not withhold what it removed.
- **`intent st dehydrate` names every path it will remove before it removes one.** Its `removed:` and `pruned:` lines printed from the report, which is after the act, so the first mention of a path was in the past tense. The future-tense lines now print from the plan, above the past-tense lines from the report, so the two can be compared by eye. The announcement is made at the last moment before anything is irreversible, after every refusal has been taken: announcing earlier would name files a refusal then spares, which is its own false report.
- **`intent st hydrate` refuses to write over a view whose bytes differ, and `--overwrite` names what it discards.** Every verify step went into the write set unconditionally, so a hand edit to a realised view -- or any work an unregistered writer had left there -- was replaced by the render and reported afterwards as written, at exit 0. An overwrite is a removal of the bytes that were there, and `dehydrate` has refused this exact signature since it was written, because the difference may be a hand edit and nothing on disk says which; the two verbs now answer alike. A view this process cannot read is not treated as diverged, so a permissions problem does not refuse a realisation under the wrong message. `intent doctor`'s remedy for a skewed view names `--overwrite`, because the bare verb now refuses.
- **A realisation verb removes nothing.** `intent st hydrate`, `intent edit --path` and `intent st edit` build the estate's plan and narrow it to one artefact's directory, because classification needs the estate as its denominator -- and the narrowed plan could carry removals, which the run performed. A verb whose whole job is to make files exist could delete one on its way past, printing a single path and nothing else. All three now refuse, name every file the plan would have removed, and point at `intent organize`, which is the verb that reconciles an estate. The loss was latent rather than realised: the estate-wide ship gate held every removal until the last precondition went green, and a defect whose only guard is a gate designed to open is one that arrives on the day nobody is looking.
- **The MCP `organize` tool removes nothing its caller was not shown.** One call with `apply: true` removed files, and the first and only account of which files was the response that came back afterwards. A machine caller has no moment of looking, so the moment is made into a protocol: call once without applying to see the plan, then echo the `plan` digest that answer carries. Applying without one is refused and the refusal says what to do. The echo is checked rather than merely required -- a digest that no longer matches this tree refuses, because the estate moved between the two calls and the removals about to run are not the ones that were shown.
- **`intent upgrade` ingests the v2 bucket files of a thread that already has canon.** An estate converted before the prune existed still carries v2's `COMPLETED/`, `NOT-STARTED/` and `CANCELLED/` directories beside its migrated threads, and re-running the upgrade re-emitted every thread from canon without reading them -- so the attachments in those buckets were carried nowhere, `intent organize` refused to prune a single file of the v2 tree, and its remedy said to re-run the upgrade that had just done nothing. The upgrade now carries each bucket file into its thread and names it (`ingested:`), names each file it declines with the reason (`not ingested:` -- a name the attachment naming gate refuses, a file over the size cap, or a copy that differs from the attachment canon already holds, where canon wins), and never stops the run over one. A run that ingested anything removes nothing: it names what it left (`prune deferred:`), and `intent organize` names every path before `--apply` removes it.
- **`intent organize` reports the v2 prune refusal once per run, not once per file.** The prune removes nothing while any v2 file is unheld, so a preview over an estate with many unheld files printed an `error:` line for every one of them, each carrying the same remedy -- a run that changed nothing read as a run that failed, over and over, until the lines that said otherwise had scrolled away. The refusal is now one line with the count and one remedy, printed at every verbosity: a preview says what `--apply` would refuse and is not an error, and an apply says what it refused. The remedy names both ways out: `intent upgrade` ingests what it can, and a file it reports as not ingested stays until it is renamed or moved out of the v2 tree by hand. `--verbose` lists the files with the reason each is unheld, the default says how many it left out, and the summary's `refused` count is unchanged.

## [3.0.1] - 2026-09-11

Full detail, including what an upgrade does and does not recover: `docs/releases/3.0.1/RELEASE_NOTES.md`.

### Fixed

- **A Homebrew install of v3.0.0 has no rule library and no skills, and the verbs that read them succeed empty rather than failing.** The formula's copy list did not match what the binary resolves at runtime, so the keg was missing `intent/plugins/claude/rules/` and `intent/plugins/claude/skills/`. On that keg `intent claude rules list` and `intent claude skills list` exit 0 and list nothing, `intent claude rules show <id>` says no such rule exists, and `intent critic <lang>` reports `ok` at exit 0 over zero rules. **The critic is the dangerous one: it reports a clean result over rules it never had.** That is also why the fault shipped: nothing failed. The copy list is now checked against its consumer rather than maintained beside it.
- **`intent claude ws` and `intent claude start` did not run in v3.0.0.** Both were declared and unimplemented, answering `is a known command that is not implemented yet` at exit 2. v3.0.1 implements them and ships `intent/plugins/claude/bin/intent_claude_cwi`, the launcher they run, which the v3.0.0 keg did not carry either.

- **`intent ac new` on an id that already existed destroyed the row and reported success**, and there was no edit verb to reach for instead. It now refuses, names the id, and points at `intent ac edit`. The ordinary way to hit it was to retype a criterion you meant to reword, so the row destroyed was the one being handled carefully. `intent at new` carried the same shape and is refused the same way.

- **A criterion authored unsatisfied lost its evidence clause on migration, by construction, while exiting 0.** `AcState::Unsatisfied` was a unit variant with nowhere to put the text, so a wildcard arm routed the case to a state that could not represent it. **No parsing fix could have reached it.** The state now carries the note and the match is three explicit arms rather than two and a catch-all, so a fourth case cannot be added without someone deciding what happens to the prose. Recorded first as ingest damage, which is the wrong class: a migration artefact is surveyed and repaired once, where an unrepresentable state destroys the same data on every hop.

- **`IN-AG-PFIC-001` stated a different rule in six canon homes than the one the rule library owns.** The library titles it _Pure Function, Impure Coordination_ -- keep the domain core deterministic, push I/O to the boundary -- while `AGENTS.md`, `intent/llm/RULES.md`, the `in-standards` and `in-review` skills and two rule-pack references glossed it as an idiom list. Code can be fully idiomatic and still bury I/O deep in a domain core, so `in-review`'s checklist recorded the rule as examined while the violation passed. Each home now points at the owning rule instead of restating it. **A project already on v3.0.0 does not pick this up from a tool upgrade** -- see the release notes for the check and the two verbs that rewrite it.

- **`intent claude skills sync` reported a conflict for skills whose canon and installed trees are byte-identical, and told the operator that forcing would discard their work.** The comparison tested each side against the recorded baseline and never against the other, so a stale baseline made both limbs true. It was also a latch: no held outcome records a baseline, correctly, since nothing was written -- so the stale value survived the run that reported it and every later sync recomputed the same verdict. Measured on a live install: four skills held, three of them byte-identical to canon. **The hold's wording changed with it.** It said _changed upstream AND here -- copy your edits out first_, asserting an edit the baseline cannot establish; it now states only what the build knows, including that `--force` is safe if you know you have no local changes. The old message talked a careful operator out of the correct action, which is how the skills stayed stale.
- **Text hand-written after a generated file's trailing banner vanished on a round-trip instead of refusing.** The renderer owns everything up to the banner; anything after it was neither preserved nor reported, so an operator's addition disappeared with the command exiting 0.
- **The v2-to-v3 conversion ran with no git and over a dirty tree.** It now refuses both, scoped to the conversion path so the convergent re-run is untouched. A conversion with nothing to fall back to, or one folding uncommitted work into its result, cannot be undone by the operator afterwards.
- **`intent critic --rules <dir>` was accepted and never read.** A run given a rules tree ran the installed rules instead and could report clean. The run is now rooted where the flag says, and a `--rules` path that is not a directory is refused by name.
- **A project config that would not read was ignored in silence by `intent critic`**, so every rule `.intent_critic.yml` disables was back on, with nothing saying why. The critic now warns on stderr, naming the config and the parse error, and still runs: the pre-commit gate calls it in every project, so a config quirk must not wedge a commit.

### Added

- **The Intent menubar app ships, as `Intent.app.zip` on the GitHub release.** A universal bundle for macOS 14 or later: Developer ID signed with the hardened runtime, notarised by Apple, and with its notarisation ticket stapled to the bundle, so Gatekeeper can check it offline. It lives in the menubar with no dock icon and runs the `intent` CLI it finds on your login shell's `PATH`, so install the CLI first. **The Homebrew formula installs the CLI pair (`intent` and `intentd`) only, not the app.** To install the app, download `Intent.app.zip` from the release, unzip it, and move `Intent.app` to `/Applications`.
- **`intent ac edit`** — change a criterion's text without touching its satisfaction. Its absence is what made the destructive create reachable.
- **`intent at edit`** — re-cite a test's file or coverage while keeping the status and note a re-create would have reset.
- **`intent daemon restart`** -- `intent app` had it and `daemon` did not, while a doc comment on the `app` family asserted the two carried the same verbs. Driving it exposed a pre-existing race that also affected the menubar's own Restart button: `stop` waited on the socket while `start` gated on the advisory lock the daemon holds until it exits, so a composition of the two could leave the daemon down with both halves reporting `ok:`. Fixed in `stop`, so every composition inherits it.
- **The menubar app's status line opens `intentd`'s web face.** The line that reports health IS the affordance -- clicking `intentd is active` opens the daemon's page in a browser. There is no second menu item and the URL is not printed: an earlier cut showed both, which said the same thing twice and put a bare `http://127.0.0.1:<port>` in a menu. The address is captured when the menu is painted rather than re-asked on click, because a daemon that restarts between the two answers on a different port -- `intentd` binds `127.0.0.1:0` and the kernel assigns.
- **`intent daemon status --format json` reports a `url` for a live daemon.** Present if and only if a loopback endpoint completes a round trip, so it is an answer rather than a read of `intentd.addr`, which outlives a daemon that was killed. This is what lets the app render the address without deriving it: the app holds no product logic, so the CLI owes it a field rather than a string to parse. The page it points at is served unauthenticated -- the bearer token gates `/op` and the entity views -- and a test asserts that with a raw unauthenticated `GET`, so the day `/` starts demanding the secret is a day a test fails rather than a day an operator meets a login wall.

### Changed

- **The runtime store migrates from schema 13 to 18 on first open, and neither step (13 -> 17, 17 -> 18) can be undone.** v3.0.0 refuses a migrated store. Take a snapshot with v3.0.0's `intent backup` before upgrading: copied back over `intent/.cache/intent.db` by hand, it is the only way back that always works. Deleting `intent/.cache/` lets v3.0.0 rebuild from committed canon only while that canon carries nothing v3.0.1 alone writes, and gives up the event log. See the release notes.
- **`intent st edit` opens an editor on a terminal and prints the path into a pipe.** In v3.0.0 it printed the path in both cases. `--editor` and `--path` force either branch. They exist for a stated cost rather than for symmetry: a bare terminal test makes behaviour depend on an invisible property of the environment, so a wrapper, a CI job or an editor plugin gets a different result with nothing in the command saying why. A script expecting a path on stdout is already in the branch that still prints one.
- **`intent organize` no longer prints the full inventory by default.** On a large estate the default ran to thousands of lines, which is not a report anybody reads. `-v` restores the inventory and `-q` drops to the action rows alone. **The `to-remove:` lines stay in the default output** -- they name files the estate is about to lose, and shortening a report by hiding its consequences is the wrong saving.
- **`intent st list` heads its first column with the project's directory name** rather than a fixed label, so the output says which estate it describes. Clipped at 16 characters, a bound forced by measurement: a 35-character directory pushed a 34-character title off the screen.
- **`intent critic` refuses at exit 2 when its rule library is empty.** v3.0.0 reported `ok` at exit 0 over zero rules, which is what a Homebrew install of v3.0.0, missing its library, still does. A project whose `.intent_critic.yml` disables every rule for a language is not that case: it exits 0, and the report says how many rules were disabled.

### Removed

- **Nothing that worked in an installed v3.0.0 is removed.** The removals below are commands that never ran, and a source file the Homebrew install never carried.
- **The v2 shell implementation is gone from the repository** (`bin/intent` and the 25 `bin/intent_*` scripts), and the v2 shell critic and hook launcher under `intent/plugins/claude/` (`lib/critic_runner.sh`, `lib/rules_lib.sh`, `bin/intent_claude_hook`) go with it; v3's critic and its `intent claude hook` launcher are built into the binary. This only affects someone running `bin/intent` from a checkout of this repository; the Homebrew install never shipped any of it, and the installed `intent` is the v3 binary. `bin/int`, `bin/devbin` and `bin/.devbin/` stay. The repository's shell test suite now drives the v3 binary, and the v2-only tests went with the v2 shell.
- **`intent st bootstrap`, `intent agents template` and `intent claude prime` are now declared retired, and none of them ever ran.** In v3.0.0 each answered `is a known command that is not implemented yet` at exit 2; each now answers `was retired in Intent v3` at exit 2. **`intent init --with-st0000` is gone with them**: v3.0.0 refused it at exit 2 because the ST0000 bootstrap was not implemented, and v3.0.1 no longer recognises the flag, so it fails as an unexpected argument at exit 1. A script using any of the four failed before and fails after; only the flag's exit code moves.
- **`intent st repair` is now declared retired, and it never ran.** In v3.0.0 it answered `is a known command that is not implemented yet`; it now answers `was retired in Intent v3`. The message changed and the capability did not, because there was none. A script calling it failed before and fails after. **A previous entry claimed it shipped and worked in v3.0.0** -- that was read off the register's `shipped` population, which counts declared-and-not-retired rows and cannot see whether a command was built.

## [3.0.0] - 2026-08-26

Intent is rewritten as a native binary, and the model underneath it changes: **a project's data lives in a database, the committed files are a projection of it, and the markdown you read is generated.** Everything below follows from that one change. Migration is one command, it refuses rather than guesses, and a v2 tool and a v3 tool each refuse the other's projects so that a half-migrated estate cannot be quietly corrupted.

### Changed

- **Intent is a native binary rather than a shell program.** Three pieces, and the split is the point: a library that owns every operation and the store, a thin CLI that parses, calls and renders, and a per-machine daemon that serves several projects at once. Nothing that decides anything lives in the command layer, so the CLI, the daemon and an agent all reach the same code by construction instead of by discipline. The practical consequences are the ones you notice first -- commands answer in milliseconds on estates where the shell implementation took seconds, and a data model that is validated on the way in rather than parsed out of prose on the way out.

- **The database is the source of truth, and the files are a projection of it.** In v2 the markdown under `intent/` _was_ the project: every command read it, wrote it, and trusted it. In v3 the project lives in a local SQLite store, a canonical extract of it is committed to git, and the readable markdown is generated from it. **This is the change that everything else in this release depends on, and it is worth being concrete about what it means for you.** `intent sync --to-disk` writes the store out; `intent sync --to-store` reads the committed extract back in; either direction takes a thread id, so you can move one thread without touching the estate. The store is per-machine and gitignored -- it never travels -- and the extract is what your collaborators actually receive.

- **The generated views are generated, and editing them does not do what it used to.** A thread's `info.md` and `acceptance.md` are rendered from the model. A criterion typed into `acceptance.md` by hand is discarded at the next render, silently, because that file is an output. `intent doctor` reports a view that has drifted from the model that produced it, so the divergence is visible rather than surprising.

- **A v2 tool refuses a v3 project, and a v3 tool refuses a v2 project.** Both directions are deliberate and both exit non-zero with the remedy named. Before this, a v2 binary in a migrated tree operated and wrote -- creating files, updating indexes, reporting success -- against a model it could not read. **A hard stop is strictly better than quiet corruption**, and the cost is real: whichever tool your PATH resolves to is the one that has to match the project, so migrating a project and repointing the tool are one operation rather than two.

### Added

- **`intent organize`, and `.intentfiles` -- disk becomes optional without anything becoming unrecoverable.** A project accumulates hundreds of files for threads finished years ago, and all of them are reproducible from the store. `.intentfiles` is a plain list of the artefacts you want realised on disk; `intent organize` reconciles the tree with it, realising what is declared and removing what is not, and it previews unless you pass `--apply`. **Nothing is removed that the store cannot reproduce byte for byte** -- the check is per-file and the removal is refused otherwise. **An absent `.intentfiles` keeps everything**; a manifest declaring nothing keeps nothing, and the difference between those two is deliberate rather than an accident of an empty file.

- **`intent://` addresses, so a piece of data has a name.** Any thread, work package, issue or field can be named by address and read by it, which is what lets an agent ask for one value instead of a rendered document.

- **`intent search` -- full-text across every piece of authored prose in the project**, which the shell implementation could only approximate with `grep` over whichever files happened to be on disk.

- **`intent export` and `intent ingest` -- the way data leaves and the way it comes back.** `export` extracts the store into a portable form usable without Intent at all, including a readable text realisation of the whole estate for a human with no tool. `ingest` is the door in: it is the recovery path, and it is also the v2 migrator, so the same gate that validates a migration validates anything else being read in.

- **`intent events` -- the history the store holds.** Every operation that changes the model is recorded with what it touched and when, filterable by operation and by subject. This is the one thing in the project that cannot be reconstructed from anything else, which is why it is queryable rather than merely present.

- **`intent schema` -- the generated schema faces.** The JSON Schema, the SQL DDL and the GraphQL SDL are printed from the same model definition rather than maintained beside it, so a tool integrating with Intent reads a description that cannot drift from the thing it describes. Every table declares how its data leaves the store, and a table that declares nothing is refused.

- **`intent backup` -- a snapshot of this machine's store, with expiry.** Deliberately one-way: there is no restore verb, because the store is rebuildable from the committed extract and a restore verb invites someone to overwrite good data with an old snapshot.

- **`intent doctor` gained real diagnostics for the new model** -- views that have drifted from the model, files under a thread that the store does not carry, and commands whose declared surface disagrees with the built one.

### Migration Guide

**Check before you convert. `intent ingest` reads your estate and writes nothing** -- not a file, not a database -- and reports what parses, what carries a value it will convert as-is, and what it cannot read at all. Run it and read the residue before anything touches the project. **Nothing blocking means the estate is convertible.**

**There is a floor at v2.19.0, and a project below it must cross that first.** The v3 tool says so by name and gives the two-step remedy rather than refusing flatly.

**Migrate the project and repoint the tool in one operation.** Both tools refuse the other's projects, so the moment a project declares v3 every command refuses until whatever `intent` resolves to on your PATH is the v3 binary. That is not a hazard to work around -- it is the hard stop that replaced quietly writing v2 structures into a v3 tree -- but it means the two halves are one change and not two.

**Nothing needs fixing under v2 first.** A value the v2 vocabulary never had is carried across as it stands rather than blocked or guessed at, so an estate with years of hand-authored variation converts without a clean-up pass.

### Removed

- **`intent treeindex` retires whole -- the command, the `intent/.treeindex/` cache, and the rules that told an agent to consult it.** The store carries a source tree index, so a directory summary maintained by hand in a parallel cache is a second copy of something the model already holds. Anything reading `intent/.treeindex/` should read the store instead.

- **`intent help <command>` is gone; `--help` is the one surface.** Most of the v2 surface had no help file at all, so `intent help` fell through silently for the majority of commands it appeared to serve.

- **`intent organize` in its v2 sense is retired and the name is reclaimed.** The v3 command with that name does something else entirely: v2's organised files on disk, v3's reconciles disk against the model, and `intent st organize` goes with it. **A script calling the old one will not fail -- it will do something different**, which is the one migration hazard in this release that a refusal cannot catch for you.

- **Issues are stored and committed but never realised as files.** `intent issues hydrate` and `intent issues dehydrate` are withdrawn rather than left declared, because there was nothing for an issue to be realised into, and a verb reporting success over a file it did not write is worse than an absent verb.

### Renamed

- **`intent st_zero` becomes `intent st bootstrap`.** The underscore spelling was the only one of its kind on the surface and it is gone rather than aliased -- a rename facility built for a population of one reads as foresight and ships as unused surface.

## [2.19.1] - never released; this work shipped in 3.0.0

**There is no `v2.19.1` tag and there will not be one.** This work was staged against the v2 shell line and was overtaken by the rewrite. Every item below was checked present in the `v3.0.0` tag (`80d8b2ca`) by its introducing commit rather than assumed from one of them. It is kept under the version it was written for, because the detail describes the v2 line and folding it into the 3.0.0 entry would misattribute it as rewrite work.

### Added

- **`lib/templates/hooks/whiteboard-header-guard.sh` -- the whiteboard header block is not YAML, and writing valid YAML there is now refused at commit time.** The protocol has ruled since v2.19.0 that the block is a line-oriented `key: value` where quotes inside a value are literal and never escaped, and it shipped that ruling with a measurement of only one of its two failure directions. The measured direction is a node writing **invalid** YAML: it self-repairs, because the next node to read the board sees something broken. The other direction has no such corrective and cannot acquire one -- a node that knows YAML, meeting a `"` inside a double-quoted value, escapes it, which is correct YAML, produced by care, and looks entirely fine. The reader (`fm_get`) strips the delimiters _without_ unescaping, deliberately, so the only symptom is `ws list` rendering a backslash or a doubled apostrophe mid-prose at a moment nobody is looking. One instance reached HEAD and stayed there until a peer happened to run the command.

  It ships on that single observation because the diagnosis, not the count, decides. Under the first explanation -- a pre-commit formatter quirk -- one instance is evidence of rarity and deferring is right; that explanation does not survive measurement, since `prettier --write` at the binary the hook resolves, with the hook's own invocation, leaves the reconstructed input byte-identical and there is no other writer. Under the explanation that does survive, the author is any competent node, and every consumer of this protocol runs nodes.

  It is a **separate guard from the clock guard**, not a fold into it: that file's name and contract are timestamps, and a name that comes to cover checks it does not describe is the defect this estate keeps finding in other artefacts. The shipped `pre-commit.sh` now declares both in one roster and **runs every guard before deciding**, so a board carrying a bad stamp and an escaped value costs one editing session rather than two commit attempts. Adding a third guard is a line in that array.

  Its scope is narrow and each boundary was measured rather than reasoned. **Live boards only** -- a git pathspec wildcard is matched against the whole path, so `intent/whiteboard/*/wip.md` crosses slashes and reached `.history/`; on this repository that pathspec matched twenty-one files, sixteen of them archives, which an archive commit would then have been refused over. **Header blocks only** -- the extractor requires the opening fence and stops at the closing one, because an unanchored range scans the entire file when there is no header. **Only lines the commit adds** -- otherwise one pre-existing escaped value would wedge every future heartbeat commit on that board, and a guard that must be bypassed to work is a guard nobody keeps. **Never prose** -- nodes report this class to each other by quoting it, and five tracked board files do exactly that today, one of them a live board; scanning prose would make reporting the defect an offence. Like the clock guard it never auto-corrects, and prints the repaired line instead.

  Nothing changes for a project without `intent/whiteboard/`. Only `pre-commit.sh` is copied into a project; guard bodies are resolved at runtime from `INTENT_HOME`, so this reaches every board-running project on the next `intent upgrade` with no `.git/hooks/` surgery and no new installer wiring.

### Changed

- **Intent adopts devbin, the estate's dev launcher, and `bin/release` becomes `intent build release`.** Every other project here (Lamplight, Conflab, Baize, Laksa) already runs its build, test and check gates through devbin; Intent stayed off-piste, and the v3 native binary needs the Rust tooling devbin already carries. The launcher installs as **`bin/int`**, not `bin/intent`: `bin/intent` is the v2 shell CLI and the product itself, so devbin's own `link_alias` refuses to replace it and left it untouched. The estate's usual two-letter alias would have been `in`, which is impossible -- `in` is a bash reserved word, so it parses as a syntax error in bash while working in zsh, which is the worst combination available. `bin/int test all`, `bin/int build cli` and `bin/int build release` are the surface that matters. The release orchestrator moved verbatim to `bin/.devbin/cmd/build.d/release` with one behavioural change: `PROJECT_ROOT` now comes from the dispatcher rather than being derived from the script's own path, so it refuses to run standalone and cannot be fooled about which tree it is cutting. `--help` output stops printing its own comment markers, a BSD-sed bug (`\?` is not a basic-regex quantifier there) that had made every `--help` since the script was written render as raw comments on macOS.

### Fixed

- **An inherited `PROJECT_ROOT` decided project-vs-global without resolving anything, so Intent wrote into whatever tree a parent process named (issue 0025).** `PROJECT_ROOT` is a generic name -- a Makefile, direnv, CI, or devbin, which exports it on every invocation -- and Intent read it as an answer. `intent claude subagents install` chose its manifest path on the variable's mere presence, so run from any directory with that variable set it wrote into the tree it named, silently; `intent_agents` had the inverse shape, where an inherited value SUPPRESSED the config load entirely so no amount of standing in the right directory corrected it. Resolution is now the only authority: `resolve_project_root` assigns from the filesystem and overwrites anything inherited, `require_project_root` resolves before it refuses rather than merely testing the variable, the three plugin bins that never resolved now do so at load (which matters because the dispatcher execs plugin commands before it loads config), and `bin/intent` clears an inherited value at entry so a future reader that forgets to resolve fails safe -- empty, and therefore an honest refusal -- instead of naming a stranger's tree. `INTENT_HOME` remains the supported way to point at a different tool tree.

- **The agents plugin's five suppressed config loads (issue 0025, first pass).** `intent/plugins/agents/bin/intent_agents` guarded its config load with `[ -z "${PROJECT_ROOT:-}" ]`, so when a parent process exported that variable — a Makefile, direnv, CI, or devbin, which exports it on every invocation — `load_intent_config` never ran and every path below operated on whatever tree the ambient value named, from any working directory, silently. Resolution is now unconditional; `load_intent_config` assigns from `find_project_root` and is idempotent, so it cannot be fooled from outside. The test runner also scrubs `PROJECT_ROOT`, `INTENT_ROOT` and `BIN_DIR` before computing its own, because a suite that inherits ambient project state measures the machine it happens to run on rather than the code. The wider surface — `intent_claude_subagents` choosing its manifest path on the variable's mere presence, and `require_project_root` returning success for any non-empty value — is recorded in the issue with file and line, and deliberately left for its own pass rather than rushed alongside a tooling change.

- **`intent at lint` and `intent ac gate` accepted a work-package scope and silently dropped it (issue 0024).** `at lint <ID>/NN` documents the scope in its own usage line; both commands then linted, counted and reported every AT row in the thread under a question naming one work package. The reporting half made a finished work package read as blocked by rows it does not own -- on the reporting estate, four different work packages of one thread returned the byte-identical verdict, the whole thread's numbers each time, and a work package with four satisfied criteria and four green tests could not be closed. The mutating half was worse: `--fix` under a scope rewrote rows the scope excluded, so narrowing a fix to one work package _in order to be careful_ rewrote the whole thread. An instrument that accepts a narrowing argument and answers the wider question reads exactly like a correct answer, which is how this survived a 1639-row estate unnoticed. Both AT loops now apply the same `in_wp_filter` the gate's criteria loop always did -- before the row count, so the denominator narrows with the findings -- and out-of-scope rows are copied verbatim rather than skipped, since the fixer rewrites the whole file. The lint lines and the gate's remedy now print the scope that was resolved rather than the bare thread id, so the subject of a count can never be wider than the count.

## [2.19.0] - 2026-08-14

### Added

- **The AT row has a grammar, and `intent at lint` enforces it (issue 0017, with 0014 + 0015).** An acceptance test row asserts _this named test, in this named file, proves these named criteria, and here is its state_ -- and until now three of those four were recovered from free-form markdown by independent single-shot regexes. The reference was the weakest: it was defined as _whatever sits inside the first pair of backticks_, so it was not required to be a path, to contain a directory, to exist, or to be present at all. No row could ever be malformed, only partially recovered, silently, one field at a time. On the reporting estate that produced five mutually incompatible reference forms across 314 rows and two live green ATs citing Tailwind utility classes as their test files, with no diagnostic anywhere.

  The row is now one anchored pattern with two arms -- a test arm requiring a backticked repo-relative path, and a non-test arm (`(non-test)` + prose + `status: n/a`) for the doc / eyeball / gate rows the status vocabulary has always declared. Every field reader is one line over that pattern, so a non-conforming row yields NO field rather than a plausible wrong one. `intent at lint <ID>` reports five checks: **L1** the row matches the grammar, **L2** a `green`/`red` row's cited file exists (`to-write` exempt -- a missing file is the correct state for a test not yet written), **L3** the cited file contains the literal AT id, **L4** every covered id is a real AC row, **L5** a non-test AT is not the sole cover for a test-backed AC (it can never satisfy one, because `n/a` is never green). `intent at lint <ID> --fix` migrates the mechanical half.

  **`--fix` refuses any row it cannot migrate without losing something, and the diagnostic names everything the row cites.** Two shapes are left alone: a `path::"name"` citation, because the migration is two-ended -- cite the file, put the AT id inside the test -- and stripping the name before the id lands does not half-migrate the row, it breaks the only link it had, converting an honest "retired form" finding into a misleading "the file does not carry the id" one on a row that now reads complete and points at nothing; and a `pathA + pathB` citation, because the grammar admits one file and choosing which survives is the author's call. The lint line was the sharper defect: it reported everything before the first `::`, so on a two-file row it named one file and silently discarded the other, and anyone hand-following that advice lost exactly what the fixer did. It now lists every cited file and says where the rest belong. Measured against a consumer estate of 1642 AT rows: 268 rows still migrate mechanically, and none loses a cited file or a test name.

  `--fix` resolves a bare filename to its one real path, and that search is now bounded to references that could actually BE a filename. It previously ran a full-tree walk per non-conforming row: on a 65GB estate whose rows carried a `[to-write]` placeholder in the reference slot, the tool scanned the whole repository twelve times over -- four seconds a row on a warm cache, and it reads as a hang. The correctness half was worse than the cost, because `find -name` takes a glob: `[to-write]` is a character class, so a lone single-character filename in a subdirectory was a unique match, and the row would have been rewritten to cite a file with nothing to do with it. The excluded trees are also pruned rather than filtered, which is what the exclusion list always looked like it was doing.

  **The one substantive change is that the row links by id rather than by test name.** A cited name is unverifiable -- paraphrase defeats every string match, and it is why the reference grew three competing shapes -- while an id is checkable from both ends: the row names the file, the file names the row, and `rg AT-03.2` finds both. Name the test by putting the AT id inside it.

  The status verbs answer to the same grammar: each arm has its own vocabulary, so `intent at na` refuses a test-backed row and `intent at red|green` refuses a `(non-test)` one, before writing rather than after. Substituting a status an arm does not admit pushes the row out of the grammar, at which point the strict reader correctly sees nothing -- and the write verifier would report that the file had not been updated, about a file it had just broken.

- **`intent ac descope` / `rescope`, and `intent ac withdraw` / `reinstate` (issue 0013).** An AC's state was modelled as a boolean when practice has four. A requirement that moves to another thread, or is withdrawn outright, is a real and routine decision, and the only two representations available were both wrong: `satisfy` is a lie, because the work was not done, and leaving it unsatisfied is honest but permanent -- the AC counts against the thread forever, the gate reports BLOCKED, and `wp done` refuses to close a thread that is genuinely finished. The measured instance had been sitting BLOCKED for days with no outstanding work.

  Both states are non-blocking, and both are reported separately rather than folded into the failure count (`29/29 satisfied, 1 descoped -- PASS`), because a thread that descoped half its contract has to look like one. Each verb carries the audit payload that justifies it existing at all -- `descope` requires `--to` and validates it against a real thread (a descope to a thread that does not exist is a strike with extra steps), `withdraw` requires `--reason` (a withdrawal with no reason is a deleted line with extra steps, and deleting the line is the practice it replaces). Who ruled, when, and which thread now owns it all land on the AC line, greppable and diffable, where `intent ac list` can report them. `intent ac satisfy` refuses an AC in either state and names the undo, because recording satisfaction of a requirement nobody is doing any more is the dishonest bookkeeping these verbs exist to replace.

  The state is detected by marker and never by the `satisfied:` field, and it is checked BEFORE satisfaction: a descoped test-backed AC whose covering AT went with it would otherwise find no cover and report unsatisfied, which is the false BLOCKED the whole issue is about. A contract emptied entirely by off-scope moves is refused rather than passed on an empty set, and the refusal names the existing `acceptance: exempt` escape -- ST0048's rule is that an exemption is announced, never inferred from emptiness, and a contract emptied one descope at a time is still emptiness.

- **`intent st done` and `intent wp done` say something when the objective was never written (issue 0010).** A steel thread could be closed with `## Objective` still holding the words the template shipped -- marked complete without anyone having stated what it was for -- and nothing said a word. The document whose job is to carry the intent instead carried a prompt to supply one, and the close is the last moment anyone looks at it. It warns and deliberately does not block: the acceptance contract is the gate, and an unstated objective is a reason to write one, not grounds to refuse a close that is otherwise finished.

  The scope is one section of one file, and the narrowness is the feature -- a sweep for any bracketed placeholder across the doc set fires on most threads in a real estate, and a warning that fires on nearly everything is switched off within a day. The placeholder strings live once, in `bin/intent_helpers`, and a guard asserts each still matches every generator that writes it -- both templates and both no-template fallback heredocs -- so a reword that forgets the constant fails loudly instead of silently switching the warning off.

### Fixed

- **`intent ac gate` no longer counts a green AT whose cited test file does not exist (issue 0015).** The citation was parsed and used in exactly one place: printing it. Nothing resolved it, so a test that was renamed, moved or deleted left its AT green forever and the gate kept counting it as coverage -- reporting a thread closer to done than it was, on the strength of a test that cannot be run. It is a false-green, so it survived by making the gate _more_ permissive as citations rotted. Now L2 blocks it, and `intent at red|green` refuse a dangling citation at the moment it goes load-bearing rather than at the next gate. Found in this repo's own estate on the first sweep: ST0052 AT-03.1 was `green` citing `tests/unit/critic_author.bats`, a deck renamed to `critic_prose.bats` in ST0053 with the citation left behind.
- **`AGENTS.md` prerequisites come from the declared languages, not filesystem probes (issue 0009).** The generator decided what a project was built in by probing for `mix.exs`, `Cargo.toml` and friends, while everything else in Intent reads the `languages` array that ST0037 made authoritative precisely because filesystem presence is unreliable evidence. Two mechanisms answered one question and could disagree with nothing to notice: a stray `mix.exs` told every reader the project needs an Elixir toolchain, and a polyglot repo whose markers sit in subdirectories declared `elixir` and was told it needs nothing. AGENTS.md is, by its own preamble, the contract every agentic CLI reads and trusts without cross-checking, so the wrong answer was stated authoritatively. Prerequisites now read `has_project_language`; **build and test commands keep their probes**, which is the issue's own split -- `mix test` genuinely depends on a `mix.exs` being at that path. `lua` and `swift` gain prerequisite lines they never had. No back-fill migration: declaration is authoritative, and an undeclared project gets the existing declare-hint instead (the 0008 precedent).

  Two exceptions, stated rather than left as apparent oversights. **Node stays on its probe** because Intent's declared-language vocabulary has no name for it, so gating it on a declaration a project cannot make would delete the line forever -- a silent loss dressed up as consistency; completing that means adding a `javascript` pack, which is its own decision. **Bats stays on its probe** for a better reason: it is a test runner, not a language, and the need for it is evidenced by `.bats` files existing rather than by declaring `shell`.

- **`intent upgrade` now converges `AGENTS.md`.** It synced subagents and skills and left the one file the CLIs actually read untouched, so a generator correction reached a consumer only if they knew to run `intent agents sync` by hand -- the v2.18.0 lesson, one file over. New `intent agents sync --check` reports staleness without writing (ignoring the generated-by date, which would otherwise report drift daily and make the step fire on every run forever). The convergence runs **after** the canon apply, not as a ledger step: canon creates files that AGENTS.md's own file map lists, so regenerating first left it stale exactly when canon had changed something.

- **`.claude/settings.json` no longer bakes an absolute Intent home path into every scaffolded project (issue 0016).** The template invoked its hooks through `[[INTENT_HOME]]/lib/templates/.claude/scripts/<name>.sh`, substituted with the installing machine's absolute home at install time -- so hook resolution, a runtime question, was answered at write time and the answer froze into a tracked file. The hooks therefore worked on exactly one machine and broke for every other contributor, and a public repository published one user's home directory path. This repository was itself an instance. Hooks are now named as `intent claude hook <name>`, a thin runner (`intent/plugins/claude/bin/intent_claude_hook`) that execs the shipped script with stdin and the exit code passed through untouched -- the UserPromptSubmit gate signals "block" with exit 2 specifically, so both had to survive the indirection. `settings.json` needs no substitution, is byte-identical on every machine, and the canon engine's `[[INTENT_HOME]]` arm is gone. Consumers converge on their next `intent upgrade` (verified end to end against a project carrying another machine's baked path). A guard asserts no tracked file under `lib/templates/.claude/` or `.claude/` carries an absolute home path.
- **`steel_threads.md` is an index of ALL steel threads again -- which it had never actually been (issue 0019).** `st sync --write` composed the DEFAULT (WIP-only) `st list` into a document whose own preamble says "an index of all steel threads", so the committed canonical index only ever carried the 1-2 threads in flight and went empty the moment a release closed the last one -- this repo's own index was an empty table over 55 threads. Born this way; unnoticed because the file decays to empty exactly when nobody is looking. The composition now passes `--status all`, the delegation's stderr flows instead of being swallowed, and `update_steel_threads_index` loses the five arguments no code path ever read -- two call sites even grepped a Created date from a path the thread had already moved away from, and the wrong answer was discarded with everything else. The terminal `st list` default (WIP) is untouched: view and index are different documents with different contracts.
- **`intent st list --status all` shows all threads, so `steel_threads.md` holds them (issue 0020).** The `all` branch walked a hardcoded array of ten status _literals_ and collected rows by exact string match, which made a presentation ordering double as a membership test: a thread whose status was not one of those ten was dropped with no diagnostic, no count and no exit-code change, from a view that names itself `all`. Two gaps compounded it. `canonical_status` -- the single synonym table -- was bypassed on this path, so `COMPLETE` was never folded into `Completed`; and a genuinely unknown status such as `SUPERSEDED` had no group at all, so even correct normalisation would have left it unplaced. Membership is now decided by `normalise_status`, the same comparison the multi-status branch immediately alongside it already used, and the ten literals collapse to the five canonical tokens they were only ever spelling out. Rows the vocabulary cannot place are emitted after the ordered groups and named on stderr with their ids -- issue 0007's precedent, report the row _and_ the anomaly, because showing it silently hides a data problem and warning without showing it is the bug itself. The vocabulary is deliberately not widened to admit whatever a project has written: whether `SUPERSEDED` is a real status is a question for the project that wrote it, and the tool's job is to stop discarding rows it does not recognise.

  It reached committed state through issue 0019. `st sync --write` composes this exact view into `intent/st/steel_threads.md`, whose own preamble says it indexes every steel thread -- so the omission was not a transient display fault, it was written into tracked project state while the regeneration reported success. Measured on the reporting estate: 96 thread documents on disk, 94 rows emitted, exit 0 throughout. This repository is unaffected -- all 55 of its threads carry vocabulary statuses -- which is exactly why the defect survived the whole release.

- **`intent st` no longer treats any `STnnnn` directory at any depth as a live steel thread (issue 0011).** `list` / `sync` / `repair` / `organize` enumerated with an unbounded `find` rooted at `intent/st`, bounded only by the name pattern. The recursion is necessary -- it is how threads in `COMPLETED/`, `NOT-STARTED/` and `CANCELLED/` are found -- but it meant any staging, triage or archive area placed under `intent/st/` became live threads, and since such areas characteristically hold _copies_, the result was duplicate ids in a namespace whose one guarantee is that an id names one thread. `st sync --write` then persisted the duplicate row into the project's committed index. Underscore-prefixing the directory reads as the obvious way to mark it not-live and had no effect. The rule now lives once, in `bin/intent_helpers`, taken from the `intent todo` view which already had it right: a thread is at `<base>/STnnnn` or `<base>/<BUCKET>/STnnnn`, one level, `info.md` required. An explicit bucket allowlist rather than a `_`-prefix blacklist, so it fails closed for any future non-thread directory however named.

  Every consumer reads that one rule, including the two counters that a review found still hand-rolling it: `intent info` treated any directory one level into a bucket as a steel thread, so it could report a different total from `intent st list` for the same project with nothing to notice, and `intent organize`'s summary used the same unbounded `find` the issue is about. A mechanical guard now asserts no command outside the helper enumerates thread directories -- grepping for the rule is what found those two, after both the fix and the audit had read past them.

- **`intent st organize --write` names a collision, finishes the sweep, and exits non-zero (issue 0011).** The filed report inferred from reading that organize printed a false `Moved` after a failed `mv`. Running it showed something different and worse: `bin/intent_st` runs under `set -e`, so the failed `mv` aborted the whole command -- raw `mv` stderr, exit 1, no intent-level message naming the collision, and every thread after it left unprocessed. Nothing is lost (`mv` refuses to merge non-empty directories); what was missing was a voice and a finished sweep. The move is now checked before it is claimed, the collision is reported with the id and both paths, the sweep continues, and the command exits non-zero at the end. The cause is probed rather than assumed: a permissions or cross-device failure is reported as what it is, with `mv`'s own message, instead of being announced as two directories claiming one id -- which would send the reader hunting a duplicate that does not exist.
- **The treeindex cache is no longer tracked, and the ignore rule reaches consumers (issue 0018).** `intent treeindex` writes its directory summaries into a shadow tree at `intent/.treeindex/`, and that tree was committed -- 87 files in this repository, ignored by nothing at any level. It is derived state (every file carries a `fingerprint:` / `generated:` stamp and the tool regenerates it on demand), it is machine-flavoured (the summaries embed the generating machine's absolute paths -- the class issue 0016 removed from `settings.json`, at scale, in a public repository), and it went stale in silence: the committed `bin/.treeindex` still described a command retired in v2.11.12, so the "project memory" it appeared to be was simply wrong, which is worse than absent. The feature shipped with an _indexing_ exclusion mechanism (`.treeindexignore`) and no _git_ exclusion anywhere, in any template, so the cache was committed by default and every consumer inherited that default.

  `intent/.treeindex/` joins the canon-managed `.gitignore` entries, so a consumer converges on its next `intent upgrade`. Where the upgrade finds the cache **already tracked** it prints the one-line `git rm -r --cached` and does not run it: ignoring a path does not untrack what is already tracked, so the rule on its own would be a fix that silently does nothing -- but staging deletions across someone else's tree, during an upgrade they invoked for other reasons, is not a decision this tool gets to make for them. With the cache untracked here, the issue-0016 absolute-path guard drops its treeindex carve-out; what is left is historical prose in completed steel threads, deliberately not rewritten because it is the record of what was true at the time.

- **`intent doctor` reports duplicate steel-thread ids.** Bounding the enumerator stops a staging area manufacturing duplicates, but the same id can still occupy two _canonical_ buckets at once -- exactly what an interrupted or collided `organize` leaves behind. That state was previously rendered as an extra table row and nothing else.
- **AT coverage ids no longer drop silently (issue 0014).** Punctuation fused to an id (`AC-09.1's`, `AC-04.3:`) matched nothing and the link vanished, rendering identically to never having written the AT -- so the contract reported work uncovered that was done, tested and green. It surfaced late, as `wp done` refusing to close, which reads as "the work is not finished" rather than "the line is phrased wrongly". Both are now grammar failures, and the diagnostic quotes the ids that _did_ resolve beside the text that did not -- because a silent parse failure does not merely lose data, it teaches the reader a false rule.

### Removed

- **`intent st zero` no longer installs `credo_checks/`, and the templates it copied are gone (issue 0021).** Deliverable D5a put six custom Credo checks into every Elixir project it retrofitted, then _tried_ to register them in `.credo.exs` -- skipping that step entirely when `elixir` was off PATH, and reducing it to a printed warning when it failed, while the copy itself was unconditional. So the usual outcome was a directory of checks that no runner ever loaded, in a tree that reported success.

  They were a second implementation of concerns the rule library and the `critic-<lang>` pre-commit gate already enforce -- Highlander, thin coordinator, debug artifacts, `@impl`, assertive access -- which is `IN-AG-HIGHLANDER-001` with the duplicate half left to rot. And rot it did: the reporting estate carried them in `elixirc_paths` for five months, compiling them into every build and executing them zero times, and when they were finally wired up experimentally one of them crashed Credo 1.7.19 outright, calling `ExecutionIssues.append/2` with an argument shape from an older API. They had stopped being _wirable_ during routine dependency bumps, with no signal, because nothing ran them. ST0032 had already retired two of the eight as false-positive generators, which was the first evidence the mechanism was not earning its keep.

  **The sharpest statement of the defect is that a check absent from the runner's config is invisible, and its presence in the repository reads as "enforced" to every human who finds it -- which is worse than absence.** That is this release's theme one more time: a thing that looks like coverage and is not.

  `lib/templates/credo_checks/` and `lib/scripts/configure_credo.exs` are deleted rather than deprecated, and `D5a` is now rejected as an unknown deliverable rather than silently accepted. Porting the checks to the current Credo API was considered and refused: it would restore the duplication on purpose, at maintenance cost, to shadow a gate that already works.

- **The no-template fallback heredocs are gone from `intent st new` and `intent wp new` (issue 0022).** Each carried a heredoc that wrote a "minimal" `info.md` when the templates could not be found -- a second copy of generated content, which project rule 6 forbids for precisely the reason both copies then demonstrated. The work-package copy wrote a `## Acceptance Criteria` section with checkboxes, a form the template retired when ST0044 made `acceptance.md` the single home, so the shadow was actively instructing users to do the thing the real template forbids. The steel-thread copy wrote `info.md` and nothing else, so a thread born from it silently had no acceptance contract at all, and every gate that reads one found nothing to read. Neither drift was noticed, because nothing keeps a shadow copy honest -- templates are edited by people working on templates, and a heredoc that only fires on a broken install is edited by nobody.

  Both now `error`, naming the template path and the resolved `INTENT_HOME`, and each removes the directory it had already created so the refusal genuinely leaves nothing behind. A missing template means the install is incomplete, which is the one thing the user needs told; quietly substituting divergent content hides it and hands back a document that looks right and is not. They are deleted rather than corrected, because correcting them restores two copies and buys another year of drift. The two 0010 drift guards over these heredocs invert accordingly -- from "the constant still matches the second generator" to "there is no second generator" -- their own comment having made this argument first.

- **`intent doctor` reports a leftover `credo_checks/`, and names both ends.** Three states, deliberately not conflated, because they are different stories with the same remedy: present but unreferenced (**never ran** -- inert, with the `git rm` printed), present and wired (**they do run** -- verify against your installed Credo first, since a pre-1.7-API check takes the whole `--strict` run down, and the removal command is deliberately _not_ offered for a directory that is load-bearing), and referenced by a `.credo.exs` whose files are gone (**stale registration**). Where `mix.exs` also names `credo_checks` in `elixirc_paths`, those lines are quoted with their line numbers -- removing the directory without them breaks the build, and a report that names one end of a two-ended migration damages everything that follows it. It warns and never errors, so a consumer carrying the residue can still cut a release; and it reports without acting, on issue 0018's rule that staging deletions across someone else's tree is not this tool's decision.

### Changed

- **The whiteboard board's header block is declared NOT YAML, and `ws hygiene` enforces the rule that was actually implemented (issue 0012).** The block is documented as YAML frontmatter and consumed as line-oriented text by every reader in the tool, and where the two disagreed the tooling rewarded the file that was wrong: `ws list` stripped the surrounding quotes without unescaping, so a board with unescaped quotes inside a `focus:` scalar -- invalid YAML -- displayed correctly, while a board corrected to valid YAML displayed `\"` mid-prose. Both passed hygiene, which never checked that the block parsed at all: the one channel the protocol specifies as machine-read was the one nothing machine-checked.

  The fork was the decision, and it goes to line-oriented `key: value`: one line per key, a single pair of surrounding quotes as a display delimiter, quotes inside a value literal and never escaped. The block is hand-written by LLM nodes in prose-heavy fields, which is close to the worst case for a quoting-sensitive format -- on the reporting board two of five nodes were unparseable at a point in time, and a sweep of one node's last 25 revisions found four invalid across two separate episodes, every one of which repaired itself at the next fold before anyone noticed. Hygiene now rejects any line in the block that is not a single-line `key: value` (the shape that genuinely breaks a line-oriented reader), warns on a missing recommended key rather than failing boards that predate the rule, and says nothing about YAML validity because validity is not the contract. The display-delimiter strip moved into `fm_get`, so `ws list` and `ws hygiene` read one value.

- **The close-gate honours the AT grammar from the day it ships.** An estate written against the old free-form convention will gate BLOCKED until it is swept, and that is the fix working: every row named was already contributing no coverage, silently. A new `at_grammar` ledger step runs the mechanical migration during `intent upgrade`, so a consumer is swept by upgrading rather than by knowing the command; rows needing a human are reported by name and never guessed at. This repo's own 116 rows were migrated the same way -- 103 mechanically, 13 by hand.
- **`lib/templates/prj/st/ST####/acceptance.md` states the grammar and no longer teaches retired forms.** The template taught `[test path::name]` and a bare parenthetical note after the status; both are now rejected, which is precisely how they entered the estate. Two holes found by running the proposed grammar against real rows rather than reading it.
- **`warning()` and `error()` speak the documented lowercase voice (issue 0023).** The CLI's prefix family is lowercase (`ok:`, `created:`, `done:`) and most hand-rolled sites already matched it; the two shared emitters did not. `warning()` went first, then `error()` -- which was the worse of the pair, because the one function whose entire job is to give failures a single voice was the function setting the wrong example, and all 25 hand-rolled error sites were copying it. 26 sites swept across `bin/` and the plugin bins; the twelve test assertions pinning the old string were found by sweeping for them before the change rather than by watching them fail. Guarded with the stream (stderr) and the exit (1) asserted, plus a mechanical grep so neither form can come back. Deliberately left, and named rather than omitted: every `Error:` echo in the plugin bins goes to **stdout**, which is a real defect and a different decision -- it changes what callers capture, not merely what they read.

### Internal

- `list_st_dirs` (THE steel-thread enumerator) added to `bin/intent_helpers` and registered in `intent/llm/MODULES.md`.
- `warn_unedited_objective` and the two `*_OBJECTIVE_PLACEHOLDER` constants likewise: the string the warning looks for and the string the generators write are one decision, in one place.
- The `extract_field` seam takes an explicit capture-group argument and uses `@` as its `s///` delimiter: the AT grammar carries a literal `/` and `n/a`, either of which closes an `s/.../.../` early. A guard asserts no grammar pattern contains an `@`.

## [2.18.0] - 2026-07-30

Minor release completing the reach of the v2.17.4 fixes. `intent upgrade` now converges the tool-managed Language Packs block, so a consumer picks up a generator correction by upgrading rather than by knowing which command to re-run by hand. It is a minor, not a patch, because it adds a new subcommand and because `intent upgrade` now writes a file it previously left alone on every consumer.

### Added

- **`intent lang sync [--check]`.** Converges the Language Packs block in `intent/llm/RULES.md` to canon for every language declared in `intent/.config/config.json`. `--check` reports without writing, exiting non-zero when any entry is missing or stale. Deliberately narrow: it touches that managed block and nothing else. `intent lang init` copies `RULES-<lang>.md` and `ARCHITECTURE-<lang>.md` over whatever is on disk, and projects hand-edit those files, so `init` is the wrong thing to run unattended; `sync` is the part that is safe to converge without asking.

### Fixed

- **`intent upgrade` now converges the Language Packs block (completes issue 0005).** v2.17.4 corrected the entry text -- it named a rule-pack path that only resolves inside the Intent installation -- and made the writer an upsert, but nothing in the upgrade path called it, so the correction reached only a project that happened to re-run `intent lang init` by hand. Every consumer would have kept the dangling pointer indefinitely while its upgrade reported success. `intent upgrade` is the convergent orchestrator, and a stale tool-managed block is exactly what it exists to drive to target, so a new `lang_packs` ledger step delegates to `intent lang sync`. Consumers upgrading from v2.17.x will see the entries rewritten in place, once. Hand edits to `RULES-<lang>.md` are untouched.
- **`intent upgrade` no longer exits non-zero in silence when the version stamp is already at target.** The orchestrator runs under `set -e` and called the version stamper as a bare command before reading `$?`, so the already-at-target return aborted the run after every step had completed -- an upgrade that did all its work, reported none of it, and failed. Introduced in v2.17.4 alongside the shared stamper; caught before it reached a release.
- **`bin/release` stamps every sidecar before the tag (maintainer tooling).** `intent/.config/config.json` and `CLAUDE.md` were corrected by a manual wrap commit landing _after_ the tag, so every published tag was internally inconsistent -- checking out `v2.17.3` gave `VERSION=2.17.3` beside `intent_version=2.17.2`. All five sidecars are now stamped in the release commit, from one list the detect step and the stage step both read, and the script refuses to tag if anything outside that list is left dirty. It also pins `INTENT_HOME` to the checkout being released: `bin/intent` only derives it when unset, so an `INTENT_HOME` exported in a maintainer's shell silently won and every sub-command read the wrong tree's `VERSION`.

### Internal

- `stamp_project_version` (THE `intent_version` stamper) and `has_project_language` (THE declared-language predicate) added to `bin/intent_helpers`; `intent upgrade` and `bin/release` share the former, and the AGENTS.md generator uses the latter. Both registered in `intent/llm/MODULES.md`.

## [2.17.4] - 2026-07-30

Patch release fixing four issues in two pairs. The acceptance parser could report a write it never performed and a green test as unsatisfied -- both because a `sed` non-match is invisible. The generators could assert into a consumer project two things that are true only where the tool lives.

### Fixed

- **`intent ac satisfy` no longer reports `ok:` having written nothing (issue 0006).** The write was a substitution anchored on a literal ` -- evidence:` segment, which a non-test acceptance criterion need not carry; `sed` exits 0 on zero substitutions, so a no-op write was indistinguishable from a real one, and the success message was printed unconditionally either way. The tool that records verified state was reporting success having written nothing, and the only symptom was a close-gate that looked like it had regressed. Writes are now verified rather than assumed: `assert_written` re-reads the row after every mutation and confirms the field reports the intended value, erroring with the id, the file and what the row still reads. It verifies the result rather than the mechanism, so it survives any future drift in the substitution pattern and also catches a substitution that fired but wrote the wrong value. `ac satisfy` is additionally now total over both row shapes -- it rewrites the evidence tail rather than substituting into one that may not exist -- so an author no longer has to hand-craft a tail to make the tool work. `replace_line`'s copy-back is checked too: an unwritable contract used to emit a raw shell "Permission denied" and carry on to print `ok:`.
- **`intent ac gate` no longer reads a green acceptance test as unsatisfied when its status carries markdown emphasis (issue 0007).** The field extractors were bare `sed -E 's/.../\1/'`, and `sed` prints the input unchanged when a substitution does not fire -- so each was total by accident, and a status the pattern could not parse (`status: **green`) came back as the entire test line, compared false against `green`, and left the criterion unsatisfied with nothing said. Eleven rows on one live thread understated it by half, and the understated gate is what routed the thread for triage. Extraction now runs through one seam (`extract_field`) that matches before it substitutes and returns empty on a non-match, so a caller sees "no status" instead of a line masquerading as a token. Markdown emphasis is deliberately NOT tolerated on read -- the vocabulary is documented in every contract's own preamble, and leniency without a diagnostic would only move the silence.
- **The acceptance readers now name the fields they cannot parse.** An acceptance-test status outside `to-write | red | green | n/a`, and an acceptance criterion whose non-test marker is unclosed (`(non-test,` rather than `(non-test)`), are both reported on stderr by `ac list` / `ac status` / `at list` / `ac gate`, quoting the required form. Neither blocks: unlike a malformed id -- which vanishes from the count and lets the gate go vacuous -- these leave their criterion uncredited, so the gate already failed closed. It failed closed _silently_, which is what cost the diagnostic cycles. `intent at green` and `intent ac satisfy` diagnose the row directly rather than reporting the conclusion the parser reached ("is test-backed") in place of the cause the author can act on.
- **`intent lang init` no longer writes a rule-pack path that dangles in the consumer project (issue 0005).** The Language Packs entry named `intent/plugins/claude/rules/<lang>/` -- real inside the Intent installation, written verbatim into a consumer's `intent/llm/RULES.md`, where `intent lang init` never vendors it and nothing resolves. It now names the command instead: `intent claude rules list --lang <lang>`, which is resolution the reader can run and is accurate in any repository. The entry-writer became an upsert -- it used to skip whenever any entry existed, so every already-initialised project would have kept the stale text for good inside a block no project can correct by hand -- so a single `intent lang init <lang>` heals it. The needle identifying an entry existed in three hard-coded copies; it is now one function that matches any revision of the wording, which is what lets the upsert and `lang remove` recognise a pre-fix entry.
- **`intent agents sync` no longer asserts a Bash 4.0+ prerequisite into every generated `AGENTS.md` (issue 0008).** It was the only ungated line in a Prerequisites block whose four other entries are detection-gated, so projects with no shell in them were told they need bash 4.0+ -- and the floor was wrong regardless, since macOS has shipped 3.2.57 as `/bin/bash` since 2007. `AGENTS.md` is, by the generator's own preamble, the contract every agentic CLI reads and trusts without cross-checking, so a false prerequisite there invites an agent to write code the target platform cannot run. The line is now gated on the declared `languages` array and states no version, pointing at the project's own documentation instead. `has_project_language` is new in `bin/intent_helpers` as the single declared-language predicate. A Prerequisites block that would now come out empty says so rather than leaving a bare heading.

### Note for consumer projects

Projects carrying acceptance contracts with out-of-vocabulary test statuses will start seeing warnings naming those rows. The rows were already failing to satisfy their criteria; the warning is the fix reporting what was previously silent, not a new restriction. Projects generated by an earlier Intent should re-run `intent agents sync`, and `intent lang init <lang>` for each declared language, to pick up the corrected `AGENTS.md` prerequisite and Language Packs entries.

## [2.17.3] - 2026-07-24

Patch release closing the last vacuous-pass hole in the acceptance close-gate: a scope naming no real steel thread or work package is now refused instead of reporting a silent pass.

### Fixed

- **`intent ac gate` no longer exits 0 for a target it could not resolve (issue 0004).** An unresolvable target degraded to an empty acceptance-criterion set, and each command in the family then reported its own flavour of vacuous success over that emptiness: the gate found nothing unsatisfied and exited 0 in silence, `ac status` printed `0/0 satisfied`, `ac list` printed no rows. "This target does not exist" and "this target has nothing unsatisfied" were the same internal state, and only the second was ever reported -- so `intent ac gate ST9999`, `intent ac gate ST0055/99` and a genuinely satisfied `intent ac gate ST0055` were indistinguishable to the CI steps and pre-commit hooks the gate exists to serve. Target resolution is now a distinct, failable step that runs before evaluation, in one resolver (`resolve_target`) that the whole `ac` / `at` family shares: it validates the `/NN` work-package segment, which nothing validated anywhere before, and reports a bad target as `BLOCKED` + exit 1 from the machine-facing gate and `Error:` + exit 1 from the human-facing readers. `resolve_wp_dir` is new in `bin/intent_helpers` as the WP analogue of `resolve_st_dir`, and the three `bin/intent_wp` sites that resolve an existing WP now share it.
- **The close-gate announces every verdict, pass included.** Silence used to be the gate's success signal, which is what kept the defect above invisible through three releases of daily dogfooding. `PASS` lines now join the existing `EXEMPT` and `BLOCKED` lines. The ST0044 WP-lenient rollup (a work package with no acceptance criteria of its own rolls up to the thread contract) is preserved, but is now granted only to a work package that exists and is announced when taken, rather than inferred from a zero count.
- **A non-numeric work-package number is a clean error, not raw bash arithmetic.** `parse_wp_specifier` fed the `/NN` segment to a bare `10#` expansion, so `intent wp show|start|done <st>/abc` -- and the `ac` / `at` family through the same helper -- aborted with `10#abc: value too great for base` on stderr instead of an Intent error. Guarded once in the shared helper. Zero-padding tolerance is unchanged: `ST0055/3` and `ST0055/03` both resolve.

## [2.17.2] - 2026-07-13

Patch release fixing two issues dogfooded in Intent's own tooling: `intent todo` mis-rendering a non-canonical status, and the pre-commit critic gate erroring on declared prose languages.

### Fixed

- **`intent todo` no longer renders `[?]` for a non-canonical status string (issue 0002).** The flat view's glyph mapping keyed on the raw frontmatter `status:` value, so a synonym `intent st` tolerates via `canonical_status` -- eg the directory-name form `NOT-STARTED` in place of `Not Started` -- fell through to the unknown-status `[?]` instead of bucketing as TODO with `[ ]`, and `intent todo` and `intent st` disagreed about the same thread. `canonical_status` (the single status synonym table) moved from `bin/intent_st` into the shared `bin/intent_helpers` -- the library both `intent st` and `intent todo` source, so it is the only home both can reach -- and `intent todo`'s `status_box` now canonicalises the value before mapping it to a glyph. Regression test added.
- **The pre-commit critic gate no longer errors or fail-opens on declared prose languages (issue 0003).** A project that declares `author` / `content` in its `languages` array made the gate invoke `intent critic author`, which accepted only the five code languages (`elixir | rust | swift | lua | shell`) and rejected prose with exit 2; the gate caught the non-zero exit and printed an `invocation error ... fail-open` pair on every commit, while reporting a pass for a check it never ran. `intent critic` now derives its accepted set from a single language registry in `critic_runner.sh`, treats `author` / `content` as a clean exit-0 no-op (prose critique is on-demand via the `critic-prose` subagent, which fires only on `.md` / `.mdx` / `.html`, never on code), and exposes `intent critic --languages`. The gate is unchanged -- it defers to the exit code, so a declared prose language is now silent. Regression tests added.

## [2.17.1] - 2026-07-10

Patch release hardening `intent issues` for adopting legacy issue trees, and completing the fleet normalisation begun in 2.17.0 (ST0055 / WP-05).

### Fixed

- **`intent issues` now picks the right primary file in a multi-`.md` issue directory.** An issue directory adopted from an ad-hoc tree may hold satellites alongside its primary (`NNNN-resolved.md`, `NNNN-session.md`) that carry no frontmatter. `issue_file` used to take the first `NNNN-*.md` alphabetically, so `show` / `list` could read an empty satellite (eg Lamplight's `0003-resolved.md` sorted before `0003-runtime-interaction-gaps.md`). It now prefers the `NNNN-*.md` carrying a `status:` frontmatter line, falling back to first-match for single-file issues. Regression test added.

### Changed

- **Fleet issue trees normalised to the directory-per-issue canon (ST0055 / WP-05).** Utilz, Conflab, and Lamplight had ad-hoc `intent/issues/` trees; all are now directory-per-issue with `status: CLOSED` on closed issues and the vendored `_templ/` removed (Intent owns the template). Cross-repo, maintainer-side; no change to Intent's own behaviour.

## [2.17.0] - 2026-07-10

Minor release adding **`intent issues`** (ST0055) -- a first-class, lightweight issue tracker built into the CLI, formalising the ad-hoc `intent/issues/` convention into a supported command. It is a minor, not a patch, because it adds a new command surface; it is additive, with zero behaviour change for projects that never run it, and the issues tree is created lazily on first use.

### Added

- **`intent issues` -- lightweight issue tracker (ST0055).** Five verbs: `list [--kind open|closed|all]`, `add [--severity SEV] TITLE` (prints `ID:TITLE`; alias `new`), `show ID [--json]`, `close ID` (OPEN -> CLOSED), `open ID` (CLOSED -> OPEN). Issues are the sub-steel-thread unit -- a bug or follow-up too small for a steel thread. On disk they are directory-per-issue under `intent/issues/{OPEN,CLOSED}/NNNN/NNNN-slug.md` (so an issue can carry attachments / sources / sub-work), the bucket directory is the authoritative status with the frontmatter `status:` mirroring it, and a legacy `RESOLVED` status is normalised to CLOSED on read. IDs are the next zero-padded 4-digit integer across both buckets. The issue template is Intent-owned (`lib/templates/issues/_ISSUE.md`), stamped on `add` -- not vendored per project. The tree scaffolds lazily on the first `add`. Registered in `MODULES.md` and `intent help`.

### Fixed

- **A `|` in a title no longer corrupts markdown tables.** The shared table renderer splits rows on `|`, so a pipe inside a steel-thread / work-package / issue title shifted every column of `steel_threads.md`, `intent wp list`, and `intent todo`. `sanitize_title` (new, in `bin/intent_helpers`) now replaces `|` with `/` at the input boundary of `intent st new`, `intent wp new`, and `intent issues add`, so a pipe can never enter a stored title. `slugify` was promoted from `bin/intent_st` into `bin/intent_helpers` at the same time so `st` / `wp` / `issues` share one slugifier.

### Changed

- **The release orchestrator moved from `scripts/release` to `bin/release`**, folding the single-file `scripts/` directory into `bin/` to match the fleet's layout. Maintainer-only; cut a release with `bin/release --minor` (etc.). The `set_e_increment_guard` test now scans `bin/` alone (which now covers `release`).

## [2.16.1] - 2026-07-09

Patch release aligning Intent's `usage_rules` / `usage-rules.md` guidance with the library's v1.x model (ST0054), plus a set of workstream-hygiene fixes bundled with it. Everything here is documentation, skills, or self-contained CLI hygiene -- no change to the generator or the rule library.

### Fixed

- **`usage_rules` interop guidance brought up to the library's v1.x model (ST0054).** `working-with-llms.md`, `/in-standards` (plus the Elixir and Ash peer skills), and the `_usage-rules.md` template described the pre-v1.0, argument-driven `usage_rules`. They now describe the config-driven model (the `:usage_rules` mix.exs key; `mix usage_rules.sync` takes no task arguments; the inline / link / skills delivery modes), name the two distinct `usage-rules.md` artifacts (Intent's hand-authored project contract vs the library's per-dependency files), reference the topical `deps/*/usage-rules/*.md` sub-rule folders, and state Intent's `.claude/skills` coexistence policy (Intent projects stay Intent-native; leave the library's skill generation off). The `_usage-rules.md` template no longer carries a hard-coded Intent version that drifts between upgrades.
- **`intent st new` and `st sync --write` now write canonical, deterministic markdown into `steel_threads.md`.** The persisted index was rendered through the terminal table renderer (pipeless, width-filled), so its content depended on the ambient terminal width and relied on a markdown linter to normalise it on save. `st sync --write` now emits content-fit GitHub-flavoured markdown (`| --- |`) that is identical regardless of terminal width; the on-screen `st list` / `wp list` display is unchanged.

### Added

- **`localfold` / `globalfold` vocabulary is now Intent canon.** `/in-finish` defines the two fold scopes -- localfold (per-workstream tidy before a compact) and globalfold (project-wide tidy before end of day, when all workstreams close out) -- and `/in-whiteboard` cross-references them to the per-node archive / release operations. Saying "localfold" or "globalfold" in an Intent project now has a defined meaning.
- **`intent todo` surfaced in the session skills.** `/in-essentials`, `/in-start`, and `/in-next` now point at `intent todo` (the flat DOING / TODO / DONE projection of steel-thread status) so it is reached for during orientation and next-step selection.
- **`intent todo` mutual guard with `utilz todo`.** `intent todo` stamps a `generator: intent todo` marker into the generated `intent/todo.md` frontmatter and refuses to overwrite a `todo.md` owned by another tool -- eg `utilz todo`, a fork sharing the file format, which stamps `generator: utilz todo`. Legacy `todo.md` files with no frontmatter are regenerated and gain the marker.

## [2.16.0] - 2026-07-08

Minor release adding the **`content` (web-content) project-type pack** (ST0053) and the **`IN-PR-*` shared prose base** it is built on. `content` is the second prose discipline after `author`; both now stand on one shared mechanical prose surface rather than duplicating it. It is a minor, not a patch, because it adds a new project-type surface (and a shared base pack); it is strictly opt-in, with zero behaviour change for projects that do not declare `content`.

### Added

- **The `IN-PR-*` prose base (ST0053).** The mechanical prose-hygiene rules that every prose discipline shares -- banned filler and `eg`-not-`e.g.`, no vanity metrics, heading hygiene, and the mechanical trope pass -- now live once in a `prose` base pack (`intent claude rules list --lang prose`), not copied per discipline. Introduces the `PR` language code.
- **The `content` rule pack (ST0053).** Six web-distinct `IN-CO-*` rules in two tiers: `style` (mechanical) -- page meta present (title / description / canonical), image alt-text, descriptive link text; and `craft` (judgment) -- scannability and web voice, one clear primary call to action, reading level matched to the audience. Enumerate with `intent claude rules list --lang content`. Introduces the `CO` language code.
- **`intent lang init content` (ST0053).** Installs `intent/llm/RULES-content.md` + `ARCHITECTURE-content.md` (web-content information architecture -- pages / posts layout, page front-matter, the content review pipeline), appends the Language Packs entry, and adds `content` to config `languages`. `intent lang list` now enumerates `content`.
- **`/in-content-essentials` skill (ST0053).** The content pipeline -- draft, mechanical detrope, revise for craft, structural check, CTA / reading-level pass -- loaded by `/in-session` when `content` is declared. `/in-review` dispatches `content -> critic-prose`.

### Changed

- **`critic-author` is renamed to `critic-prose` (ST0053).** One prose critic now serves every prose discipline, parameterised by the declared language: it loads the `IN-PR-*` base plus whichever of `author` / `content` the project declares. The two-form detrope is preserved, re-anchored to the base's `IN-PR-STYLE-004` mechanical pass. Projects on 2.15.0 that dispatched `critic-author` should use `critic-prose`.
- **The `author` pack refactored onto the prose base (ST0053).** The four shared mechanical rules moved from `IN-AU-STYLE-*` into `IN-PR-STYLE-*` (with migration aliases); the `author` pack now owns only its discipline-specific rules -- front-matter / objectives plus the four craft rules. No behaviour change for an author project: the same rules apply, sourced from the base plus the discipline pack.
- **`IN-PR-STYLE-001` no longer bans `overall` (ST0053).** The `docs/blog` dogfood confirmed that `overall` hits are the legitimate adjective sense ("overall progress"), matching a 2026-04 autopsy of Intent's own corpus. The banned filler is now `absolutely` only; `overall` is explicitly documented as not-a-tell. Rule content version 1 -> 2.

## [2.15.1] - 2026-07-07

### Fixed

- **`intent st list`, `intent st sync`, and `intent wp list` now share one table renderer.** Each sized its table by a rule of its own: `wp list` hard-capped `Title` at 30 columns (truncating every title, eg `Interpretation schema: verb...`), and `st sync` re-invoked `list` at a fixed `dft_width` while `st list` used the terminal, so the two rendered differently for identical data. They now render through a single `render_table` (`bin/intent_helpers`) that fills the terminal width (or an explicit `--width`), with content-fit as the floor so nothing is ever truncated. `st sync` composes `st list`, so their output is byte-identical.

## [2.15.0] - 2026-07-03

Minor release adding the **`author` project-type pack** (ST0052) -- the first non-code discipline on Intent's `languages` axis. A project declaring `languages: [author]` gets an authoring rule pack, a prose critic, canon templates, and an essentials skill, all activated the same way a code language is. It is a minor, not a patch, because it adds a new project-type surface; it is strictly opt-in, with zero behaviour change for projects that do not declare `author`.

### Added

- **The `author` rule pack (ST0052).** Nine `IN-AU-*` rules in two tiers: `style` (mechanical, greppable -- banned filler and `eg`-not-`e.g.`, no vanity metrics, front-matter + learning objectives, heading hygiene, and a mechanical trope pass) and `craft` (judgment / critic-as-reader -- voice and register consistency, cross-chapter continuity, full `/in-detrope` diagnosis, citation and attribution). Enumerate with `intent claude rules list --lang author`. The pack introduces the `AU` language code across the rule-id validator and enumeration.
- **`critic-author` subagent (ST0052).** The first non-code rule-library critic (prose + courseware). Read-only; two modes -- `review` (the mechanical `style` tier, default) and `craft-check` (the judgment `craft` tier, on instruction). It wires detrope in two forms without forking the trope catalogue: the mechanical trope pass runs by default off `in-detrope`'s `detection: automated` regexes, and the full `/in-detrope` diagnosis is emitted as a handoff recommendation, never invoked by the critic.
- **`intent lang init author` (ST0052).** Installs `intent/llm/RULES-author.md` + `ARCHITECTURE-author.md` (book / course information architecture -- parts, chapters or modules, learning objectives, the authoring pipeline), appends the Language Packs entry, and adds `author` to config `languages` -- exactly like a code language. `intent lang list` now enumerates `author`.
- **`/in-author-essentials` skill (ST0052).** The authoring pipeline -- outline, draft, mechanical detrope, revise, structural check -- loaded by `/in-session` when `author` is declared. `/in-review` dispatches `author -> critic-author`; in an author-only project no code critic runs, and a mixed project (eg `[elixir, author]`) runs each critic on its own subtree.

### Notes

- The headless pre-commit prose gate is deliberately deferred: `.md` extension alone cannot route a file to the author pack, and Intent's own `--` house style trips the trope catalogue's dash-overuse regex, so a headless gate needs a confirmation / suppression layer first. `critic-author` is on-demand (`Task`) only in this release.

## [2.14.0] - 2026-07-02

Minor release adding **`intent todo`** — a flat DOING / TODO / DONE view of every steel thread and work package, projected from real `status:` so it cannot drift (ST0050) — plus a generated-file width fix (ST0051). It is a minor, not a patch, because it adds a new command surface.

### Added

- **`intent todo` — a projected DOING / TODO / DONE board (ST0050).** `intent/todo.md` is a nested GFM checklist bucketed by real status: DOING (`WIP` threads + their work packages), TODO (`Not Started`), DONE (recent completions). Every checkbox is derived from the unit's `status:` and its status-directory placement — there is no separately-stored state, so the file cannot drift from `intent/st/**`. `intent todo` / `todo list` prints it (generating on first use); `todo update` regenerates it. Mutation verbs change _real_ status by wrapping `intent st` / `intent wp` and regenerating — `todo done` / `notdone` / `toggle` — so `todo done` inherits the ST0048 acceptance close-gate (a BLOCKED contract is refused, never bypassed). `intent todo --json` emits the board as keyed-by-bucket JSON (each thread carrying its work packages) for export to other systems.
- **DONE flush / prune + ISO completion timestamps (ST0050).** The DONE bucket is watermarked — `## DONE:<T>`, where `<T>` is the last-flush instant — and lists completions at or after it. `intent todo done --flush` advances `<T>` (clearing the view without touching the record in `COMPLETED/`); `intent todo done --prune` emits the pruned items to stdout (for archiving, eg `>> intent/done.md`) and then flushes. `intent st done` now stamps `completed:` as an ISO 8601 UTC timestamp for exact flush ordering; a legacy `%Y%m%d` stamp is still tolerated everywhere `completed:` is read.
- **`dft_width` config field (ST0051).** A new `intent/.config/config.json` field (default `120`) sets the width for generated files; `intent init` seeds it.

### Fixed

- **Generated `steel_threads.md` no longer truncates at 80 columns (ST0051).** `intent st sync --write` hard-coded an 80-column width, clipping the slug column of the generated index. Generated files now size to `dft_width` (config, default 120); interactive stdout stays at the terminal width; an explicit `--width N` overrides both.

## [2.13.1] - 2026-06-29

Patch release hardening the acceptance close-gate (ST0048). The gate behind `intent st done` / `intent wp done` previously treated a unit with **zero acceptance criteria** -- or no `acceptance.md` at all -- as vacuously done, so work closed with nothing to verify it against. That is now a hard failure: an empty or missing contract is refused, with an explicit `acceptance: exempt` marker as the sole escape. No-Silent-Errors applied to the acceptance layer.

**Behaviour change (read before upgrading):** any in-flight ST/WP that never authored acceptance criteria, or that has no `acceptance.md`, will stop closing until it authors criteria or is marked `acceptance: exempt`. Migration: `docs/releases/2.13.1/RELEASE_NOTES.md`.

### Fixed

- **The close-gate no longer passes an empty or missing contract (ST0048).** `intent ac gate` -- the authority behind `st done` / `wp done` -- now exits non-zero with a BLOCKED report when a present `acceptance.md` has zero in-scope ACs, or when `acceptance.md` is absent. This closes the vacuous-green hole where "every in-scope AC satisfied" was trivially true of zero ACs. Unsatisfied-AC and malformed-line blocking are unchanged.

### Added

- **`acceptance: exempt` frontmatter marker (ST0048).** The sole, explicit, visible escape from the hardened gate: a deliberately AC-free unit (eg a pure content / authorial task) declares `acceptance: exempt` in its `acceptance.md` frontmatter and the gate passes, announcing the exemption (never inferred from emptiness). The default -- no marker -- is enforced. WP scope is WP-lenient: a WP with no own ACs rolls up to the ST boundary as long as the thread carries a contract.

### Changed

- **Canon and consumer comments describe fail-by-default.** `intent/docs/working-with-llms.md` D11, the `bin/intent_st` / `bin/intent_wp` close-gate comments, and the stamped `acceptance.md` template now state that the gate fails an empty or missing contract; the retired "opt-in / legacy-safe / closes exactly as before" framing is removed and pinned out by a grep guard in `tests/unit/acceptance_close_gate.bats`.

## [2.13.0] - 2026-06-25

Minor release adding **`intent claude start` and `intent claude ws`** -- the MAAC (multi-agent agentic coding) whiteboard launcher and workstream lifecycle (ST0047). One command provisions whiteboard workstreams (the Protocol 3.0 nodes), launches a Claude Code session bound to one with the verified effort / permission / context, and manages the node lifecycle. The capability was pioneered by convention in Lamplight (the operational reference), productised as the MVP in Baize, and is now first-class in Intent, served centrally to every project from `$INTENT_HOME`. It is a minor, not a patch, because it adds a new command surface across the fleet.

### Added

- **`intent claude start <ws>` (ST0047).** Launches an interactive Claude Code session bound to a whiteboard workstream: composes the node identity + the project `.claude/restart.md` + a standing "show a daily plan, then wait" instruction, seeds `/in-session` (admitted by the in-session gate's slash-exemption, which chains `/in-whiteboard pickup`), and execs `claude --effort max --permission-mode auto --append-system-prompt ...`. Provisions the workstream first if absent (prompted). `CWI_DRY_RUN` prints the assembled argv instead of launching.
- **`intent claude ws new|list|archive|hygiene` (ST0047).** The deterministic workstream lifecycle that complements the Claude-driven `/in-whiteboard` skill: `ws new` scaffolds a Protocol 3.0 node (`wip.md` frontmatter, `.history/.gitkeep`, bidirectional `_(empty)_` inboxes with every existing peer; `hv` is Workstream Zero, working nodes are made to order); `ws list` reads the roster from frontmatter (read-only); `ws archive` retires a node into `.archived/` keeping its `.history/`; `ws hygiene` runs a mechanical structural lint (warns on oversized boards + stale heartbeats; never archives DOING content -- the semantic archive stays the Claude-driven `/in-whiteboard archive`). `CWI_WB` overrides the whiteboard root.
- **`intent/plugins/claude/bin/intent_claude_cwi`** -- the command's home in the `intent claude` plugin family. It resolves the current project via `find_project_root` (not the tool home), so it is served centrally and available in every project with no per-project install. Behavioural ATs in `tests/unit/claude_with_intent.bats` drive it through the real dispatch via the `CWI_WB` / `CWI_DRY_RUN` seams.
- **A live `intent/whiteboard/` for Intent itself** -- `hv` + `cc` + `vc` (no interface node: Intent is CLI plus data, not UX) plus a roster README, so Intent now dogfoods MAAC on its own development.

### Changed

- **The `/in-whiteboard` skill defers to the script for scaffolding (Highlander SSOT).** Its "Scaffolding a node" prose now points at `intent claude ws new`, and the skill's lazy-inbox wording ("never pre-seeded") was reconciled to the script's eager bidirectional pre-seed (ratified by the WP-01 acceptance + the Baize golden board); `ask` / `announce` keep on-demand inbox creation as a self-healing fallback for hand-added nodes. The Baize `bin/claude_with_intent` prototype is retired in favour of the central command (Highlander -- no divergent second copy).

## [2.12.0] - 2026-06-15

Minor release landing two steel threads. **ST0043** rewrites `intent upgrade` from a 524-line version-case ladder into a ~150-line convergent orchestrator and removes every migration path below the v2.9.0 fleet floor (fail-forward). **ST0045** formalises the Whiteboard Protocol 3.0 rewrite (per-node directories + single-writer inboxes + the `hv` hypervisor node) with an AC/AT contract and closes the reference-vs-skill drift. It is a minor, not a patch, because ST0043 changes upgrade behaviour for every project. Two close-gate hardening fixes ride along.

### Changed

- **`intent upgrade` is now a convergent orchestrator (ST0043).** `bin/intent_upgrade` rewritten to: detect state -> semver sanity before any mutation (error on missing/unparseable version; refuse a downgrade; refuse below the v2.9.0 floor; a future/unknown version no longer hard-fails with "Unknown version") -> verified backup -> walk a state-probed ledger (`LEDGER="relocate_config languages_field"`, dispatched by the `step_<id>_needs/_run/_verify` naming convention) -> one delegation to `intent claude upgrade --apply` -> stamp the target once, last. Applicability is decided by each step probing on-disk state, so an interrupted upgrade re-run does exactly the remaining work.
- **Single version stamper.** `intent_upgrade` is the sole writer of `intent_version` (jq, once, last). `intent/plugins/claude/bin/intent_claude_upgrade` is the sole canon engine; its `VERSION_BUMP` action and both version `sed` stamps are removed, and `canon_substitute_placeholders` is rewritten off BSD `sed -i ''` to a portable `sed > tmp && mv` so canon substitution works on Linux.
- **Whiteboard Protocol 3.0 is the documented model (ST0045).** The `in-whiteboard` skill, the `in-session` / `in-finish` chaining steps, and the "Multi-session coordination" section of `intent/docs/working-with-llms.md` now describe per-node directories with single-writer `wip.md` + per-sender `inbox.<sender>.md`, the `hv` hypervisor node, and `announce`-based shared-platform coordination, replacing the retired 2.0 flat-file model (shared `asks.md`, per-stream files, a shared platform file).

### Added

- **`bin/intent_migrations` (ST0043)** -- a new upgrade-only file (sourced only by `intent_upgrade`) holding the two ledger-step trios plus `intent_relocate_dotintent` (moved verbatim from helpers). No step writes the version.
- **Whiteboard 3.0 skill completeness (ST0045)** -- the `in-whiteboard` skill now specifies inbox-file init (`# inbox: <sender> -> <recipient>` header + single-writer note + `_(empty)_` sentinel + creation on first `ask`), `.history/.gitkeep` scaffolding, the `hv` node variant (human-driven, `session_id` optional/`none`, advisory heartbeat, `## Standing directives`), and the message-entry format (required vs recommended fields).
- **Mechanical guards** -- `tests/unit/intent_upgrade_orchestrator.bats` + `intent_migrations_{relocate,languages}.bats` (ST0043); `tests/unit/whiteboard_protocol_3_guard.bats` (ST0045, pins out any live 2.0 reference); AT-01.8 in `intent_claude_upgrade.bats` (canon-engine portability + no `VERSION_BUMP`).

### Removed

- **All migration code below the v2.9.0 fleet floor (ST0043, fail-forward).** `bin/intent_helpers` shrank 2026 -> 369 lines: every `migrate_v*_to_*` (v0 through v2.8.2->v2.9.0), every `needs_v2_*` predicate, the pre-v2 YAML/JSON/structure converters, and the migration ceremony helpers (`show_migration_summary`, `count_migration_files`, `create_project_backup`, `needs_migration`, ...). `detect_project_version` (+ `detect_stp_version`) stays shared. The only upgrade-time `~/.intent/ext/` bootstrap went with `migrate_v2_8_2_to_v2_9_0` (verified safe: `intent ext` creates the dir on demand; the fleet is already v2.9.0+). Deleted tests: `ext_migration.bats`, `migrate_v2_9_0_to_v2_10_0.bats`, `migrate_v2_10_x_to_v2_11_0.bats`, and the `create_project_backup` test.

### Fixed

- **The acceptance close-gate no longer drops malformed contract lines (F1).** An AC/AT line that failed the strict numeric grammar (eg a letter-group id like `AC-U.1`) was silently dropped, which could make a gate vacuously pass. `bin/intent_acceptance` now detects malformed lines on every read path and blocks the gate loudly. (F6, a proposal to block `st done` when `acceptance.md` is absent, was deliberately declined: a thread with no contract has not opted into the AC regime, so the gate stays open -- opt-in by presence, unchanged for legacy threads.)

## [2.11.14] - 2026-06-14

Patch fixing `intent organize` on Linux. The command tallied moves with `((counter++))` under `set -e`; in bash, `((x++))` returns exit status 1 when `x` is 0 (post-increment yields the old value), and bash 5.x's `set -e` acts on that, aborting the script after the first thread. macOS bash 3.2 is lenient in that loop/case context, so the break stayed invisible behind macOS-green CI from v2.11.12 (when `intent organize` was resurrected) through v2.11.13 -- Ubuntu CI had been red the whole time. The defect class is converted to `x=$((x + 1))` (an assignment always returns 0) and pinned shut by a guard test.

### Fixed

- **`intent organize` no longer exits 1 on Linux** (or any modern-bash host). It had been moving only the first thread and then dying. The three `((moved_count++))` / `((kept_count++))` increments in `bin/intent_organize`, plus three latent same-class sites in `bin/intent_helpers` (the frontmatter parser's `((line_num++))`, which starts at 0 every call, and two `[ -f ] && ((count++))` legacy-config tallies), now use `x=$((x + 1))`. A new guard test (`tests/unit/set_e_increment_guard.bats`) greps `bin/` and `scripts/` for naked `((x++))` / `((x--))` and fails on any hit -- closing the class that macOS-only CI green had masked for four releases.

## [2.11.13] - 2026-06-14

Patch shipping ST0044: `acceptance.md` becomes the fifth default steel-thread document, and with it an Acceptance-Criteria / Acceptance-Test process that makes "done" an externally-verified event rather than a self-asserted checkbox. It ships as a patch on opt-in-by-presence grounds -- the close-gate and the `acceptance.md` contract are inert for any thread that does not adopt them, so behaviour is unchanged for non-adopting projects (the basis on which ST0040's whiteboard also shipped as a patch). The one non-additive change is `intent st edit`, which now prints a path rather than launching an editor. The thread was dogfooded on itself: ST0044's own build ran through the five-step with an independent verifier, and the thread closed through the very gate it introduced.

### Added

- **`acceptance.md` as a default steel-thread document.** `intent st new` stamps it alongside info / design / impl / tasks, via the existing `lib/templates/prj/st/ST####/*.md` glob -- no seam, default on. The template carries the acceptance contract: the AC section (ST-level + per-WP) and the AT section, with example lines indented under guidance so a freshly stamped thread carries no live column-0 entries and cannot self-gate-block. `info.md` and `WP/info.md` reference it and restate no ACs (one home, Highlander); `intent st show` learns the `acceptance` type (named and in `all`).

- **`intent ac` and `intent at` -- the acceptance contract CLI.** An **AC** (Acceptance Criterion) is a ratified completeness boundary; an **AT** (Acceptance Test) is a small red-to-green test that proves a slice of it. `ac list` / `ac status` / `ac satisfy` / `ac gate`; `at list` / `at red` / `at green` / `at na`, with `done` / `notdone` aliasing green / red. The AT state machine is `to-write -> red -> green` (+ `n/a`), and green is reachable only from red -- a test cannot claim proof without first having failed. A test-backed AC is satisfied by computation (iff a covering AT is green), never by hand; a non-test AC carries inline evidence and is signed off by the verifier. The grammar is column-0 and bash-3.2-greppable; all reads and writes target `acceptance.md` alone.

- **The acceptance close-gate.** `intent ac gate <stid>[/NN]` is the single authority on whether a thread or work package may close. `intent st done` and `intent wp done` consult it and refuse on BLOCKED, with the verdict computed from the coverage map rather than read from a hand-ticked box. It is opt-in and legacy-safe: a thread with no directory, no `acceptance.md`, or zero in-scope ACs exits 0, so existing threads close exactly as before.

- **The five-step process, documented.** `intent/docs/working-with-llms.md` D11 is the canon home: the AC / AT axes, the five-step (verifier ratifies ACs -> builder writes red-first ATs -> verifier witnesses RED -> builder builds to green -> repeat), the open-gate and close-gate, and the lifecycle mapping. Light pointers reference it from `/in-plan` (open-gate), `/in-verify` (red-first + witness RED), and `/in-finish` (close-gate), each referencing D11 and none restating it.

### Changed

- **`intent st edit <id> [type]` prints a path instead of launching an editor.** It now validates the file type and echoes the file's absolute path for every doc type (info / design / impl / tasks / acceptance), replacing the macOS `open` / `$EDITOR` launch. This is the one non-additive change in the release: scripting `intent st edit` to open a file must now pipe the path to an editor explicitly.

## [2.11.12] - 2026-06-11

Patch shipping the full ST0042 arc: a Fable 5 architectural review of the Intent codebase (run as the first deliberate MFIC exercise, ST0041) followed by execution of all nine work packages it produced. Theme: architectural integrity -- Highlander consolidation, no-silent-errors enforcement, canon-vs-reality reconciliation, dead-surface pruning, and a test suite that can actually refute the product. Includes one small addition (`intent st cancel`) and a set of removals (dead dispatchers, the retired `intent audit`).

### Fixed

- **`intent organize` dispatches and `intent llm usage_rules` displays again** (found adding ST0042 T10 coverage). The organize script was named `intent_organise` while every referencing surface (help text, `intent st organize`) uses the "organize" spelling, so the top-level command had errored with "Unknown command" since the dispatcher's `intent_$COMMAND` default arm was introduced -- the script is renamed to match. `intent llm usage_rules` still read the retired `intent/llm/usage-rules.md` location and errored on every invocation since the v2.10.0 root-canon move -- it now reads the root `usage-rules.md`.

- **Critic test files assert on real product behaviour** (ST0042 T10). `critic_report_format.bats` asserted on a heredoc fixture defined inside itself, `critic_dispatch.bats` tested a test-local reimplementation of the dispatch logic, and `critic_config.bats` only verified the host YAML parser works -- all three were green regardless of what the product did. They now drive the real surfaces: the headless runner's text/JSON reports and exit codes (via `--rules` with synthetic rules), the shipped pre-commit hook's per-language dispatch / fail-open / blocking behaviour (stub `intent` on PATH recording invocations), and the runner's `disabled:` + the hook's `severity_min:` consumption of `.intent_critic.yml`. The permanently-skipped cross-FS migration test (which skipped unconditionally on every platform) is deleted; behavioural coverage added for the previously untested `intent llm`, `intent organize`, and `intent claude prime` -- which is what surfaced the two dead commands above.

### Removed

- **Dead and legacy surfaces pruned** (ST0042 T6, fail-forward). `bin/intent_main` (dead second dispatcher, diverged from the real one, zero callers), `bin/intent_minimal` (alpha-versioned Phase-1 stub), and the `bin/stp` symlink (the retired STP name, a year post-rebrand) are deleted. `intent audit` is retired: its custom-Credo checks ran in parallel with the rule-library critics as a second Elixir enforcement engine (Highlander violation); `intent critic <lang>` is the canonical engine. The credo-check templates themselves survive -- `intent st zero` (brownfield retrofit) still installs them. Orphan template sets `lib/templates/eng/tpd/` and `lib/templates/usr/_user_guide.md` (no generator reads either) are deleted, and `intent help`'s footer no longer points at `docs/user_guide.md` / `docs/reference_guide.md`, which never shipped. MODULES.md, help text, README, and tests updated; `intent modules check` reports a clean registry.

### Added

- **`intent st cancel <ID>`.** The docs have mandated it as the compliant cancellation path since the status discipline landed, but the command never existed -- cancelling a thread meant manually editing `status:` frontmatter, which the same docs forbid. The new dispatch case mirrors `done`: stamps `status: Cancelled` (frontmatter + body), relocates the thread directory to `intent/st/CANCELLED/`, and updates the index (ST0042 T9 / F-DOCS-2).

### Fixed

- **Canon docs describe the as-built system again** (ST0042 T9). `working-with-llms.md`'s session-hook section showed a settings shape (`matchers` arrays, `$INTENT_HOME/lib/hooks/*.sh` scripts, a soft/strict script pair) that never shipped -- it now shows the real template (`matcher` string, `lib/templates/.claude/scripts/`, echo-based Stop hook) and an executable softening path instead of the phantom `user_prompt_submit_soft.sh`. `critics.md`'s `/in-review` dispatch table described filesystem-marker probing removed by ST0037; it now documents the `languages`-array dispatch the skill actually performs. The phantom `intent claude skills status` reference, the pre-v2.10 `.intent/config.json` path in `usage-rules.md`, stale `intent/st/ST0035|ST0040/` paths (now under `COMPLETED/`), `rules.md`'s "nine required sections" (the validator requires seven), README's v2.6.0 claim and nonexistent `intent/usr/` entry, `writing-extensions.md`'s "v2.10 will enforce" promises, and CLAUDE.md's v2.11.0 stamp are all corrected; `/in-whiteboard` joins the `usage-rules.md` skills table. Guard tests in `docs_completeness.bats` pin the phantom command, hook-key, hook-script, and legacy-path classes.

- **`intent modules check` honours `file::function` registry rows and the registry matches the filesystem again** (ST0042 T7, Highlander gate). Three live advisory subagents (`intent`, `socrates`, `diogenes`) were absent from MODULES.md; they now have rows. The `needs_v2_9_0_upgrade` row described config-file-reading behaviour the function (which takes a version-string argument) never had, and cited the pre-v2.10 `.intent/` path -- corrected. A dangling row pointed at a credo check (`dependency_graph.ex`) that does not exist, and the credo-check row listed stale `R2/R6/...` identifiers instead of the actual filenames -- both fixed; the `_default/AGENTS.md` row left over after that template's deletion is removed. Finally, `intent modules check` itself tested each `file::function` row as a literal path, so every function-qualified entry reported as permanently stale and eroded trust in the gate; the check now splits on `::` and verifies the function is defined in the file.

- **Rules-path drift finished off, with a guard so it cannot return** (ST0042 T2). v2.11.11 fixed the generated AGENTS.md/CLAUDE.md and the critic subagents but missed nine canon skills (`in-session` -- auto-loaded every session in every fleet project -- plus `in-standards`, `in-review`, `in-ash-ecto-essentials`, `in-elixir-essentials`, `in-elixir-testing`, and the three `in-tca-*` skills), all still steering agents to the dead local `intent/plugins/claude/rules/` path; all nine now route through `intent claude rules list` / `show`. The `[[LANG]]` placeholder in `_usage-rules.md`, which no installer substitutes and which shipped verbatim into consumers, is replaced with language-agnostic wording (the pre-commit line now also names the real `intent critic <lang>` entry point instead of an install-local `bin/intent_critic` path). Install-resident doc references (`working-with-llms.md`, `critics.md`, `rules.md`, etc.) in generated AGENTS.md/CLAUDE.md and the elixir template are now qualified "at the Intent install", matching `_usage-rules.md`'s already-correct form; the installed `.intent_critic.yml` no longer points at a sample file consumers don't have. A mechanical guard (`rules_path_guard.bats`) greps every propagated/generated surface for the dead path and for unsubstitutable `[[...]]` placeholders.

- **One definition per shared concern across the CLI** (ST0042 T5, Highlander). The `get_intent_version` fallback literal was repeated at ~20 call sites with drift (`2.2.1` / `2.3.x` / `2.6.0` / `2.8.x` / `2.9.0` / `2.11.0`), so a broken install reported a different stale version depending on which script was asked -- the fallback now lives in `get_intent_version` alone (with a warning to stderr, since a missing VERSION file means a broken install). Config-field parsing (three divergent implementations: jq field-read, grep/cut, grep|sed) consolidates on `read_config_field`; `find_project_root` (three copies, one with a latent walk-up bug in the pre-commit hook template) on the `bin/intent_helpers` original; the steel-thread directory resolver (three copies) on a shared `resolve_st_dir`; and the `~/.intent/ext` root walk (five inline expansions with divergent `INTENT_EXT_DISABLE` handling) on `ext_root_dir` / `ext_enumerate_names`. All shared primitives live in `bin/intent_helpers` and are registered in MODULES.md; mechanical guard tests pin the version-fallback and ext-root patterns against reintroduction.

- **Generated AGENTS.md lists the skills and subagents that are actually installed.** The Installed Skills / Installed Subagents section renderers read only `$PROJECT_ROOT/.claude/`, a location the installers never write (they install user-globally to `$HOME/.claude/`), so the sections said "No skills installed" on a fully provisioned machine. Both renderers now read the project directory and `$HOME/.claude/`, project first, deduped by name (ST0042 T4). The static `_default` AGENTS.md template -- which lacked three of the four validator-required sections and self-described as auto-generated while diverging from the generator -- is deleted; `intent agents init --template _default` now uses the generated content, so a fresh `_default` install passes `intent agents validate` (one content source, per Highlander).

- **Commands no longer report success after silently doing nothing** (ST0042 T3, No-Silent-Errors). Four paths fixed: `intent st new` on a legacy file-structure project errors honestly instead of printing `created:` after a guarded copy from a nonexistent template; `intent agents init --template {rust,lua,shell,swift}` now actually creates the root `AGENTS.md` (generated content) instead of printing "Created" while copying nothing; `intent init`'s interactive agent install invokes the real `intent claude subagents install intent` dispatcher instead of a script path that does not exist; and `intent upgrade`'s backup verifies every copy and aborts before any migration on failure, instead of `|| true` followed by an unconditional "Backup created successfully".

- **`intent st repair` / `organize` normalise status to the stored canon.** Both commands carried inline status tables that mapped `wip` to `In Progress`, while the canon written by `st new`/`st start` and matched by the path resolver is `WIP` -- so a repair could rewrite a thread's frontmatter to a value the rest of the tool does not recognise. One synonym table (`canonical_status`) now feeds repair, organize, and the list-filter normaliser (ST0042 T5); regression test pins `repair` writing `status: WIP`.

- **Config values are loaded verbatim, never evaled.** `load_intent_config` built `key="value"` shell assignments from raw JSON values with jq and `eval`ed them, so a config value containing `$(...)`, backticks, or `$VAR` in `intent/.config/config.json` (or `~/.config/intent/config.json`) executed arbitrary shell on the next project-scoped `intent` command. Config fields are now read individually with `jq -r` and assigned directly (ST0042 T1); a regression test proves shell metacharacters in config values are inert.

- **Test suite no longer writes to the real `~/.claude`.** Two `intent_upgrade_dispatcher.bats` tests ran `intent upgrade` without HOME isolation, so the upgrade tail-call (skills + subagents sync) overwrote the developer's real `~/.claude` mirrors on every suite run. The fake-HOME pattern, previously copy-pasted across six test files with drift, is promoted to a single `setup_fake_home` / `teardown_fake_home` pair in `tests/lib/test_helper.bash` (ST0042 F-TEST-1/F-TEST-9); all seven files now use it. Verified: a full suite run leaves the real `~/.claude` untouched.
- **`intent st new` stamps the current Intent version.** New steel threads were created with a hardcoded `intent_version: 2.4.0` in their frontmatter (and `2.0.0` on the no-template fallback path) -- the values frozen into the template and heredoc when they were last hand-edited. Both creation paths now substitute the live version from `get_intent_version` (single source: `$INTENT_HOME/VERSION`), per Highlander. Regression test proves the stamp matches `VERSION` and that no unsubstituted placeholder survives.

## [2.11.11] - 2026-06-03

Patch fixing rules-path drift in the LLM guidance Intent generates for consuming projects. The generated `AGENTS.md` (via `intent agents sync`) and `CLAUDE.md` (from `lib/templates/llm/_CLAUDE.md`), plus the `critic-<lang>` subagents, told agents the coding-rule library lives at a local `intent/plugins/claude/rules/` path. That directory exists only inside the Intent tool itself; in a consuming project the rules are reachable solely through the CLI (`intent claude rules list` / `show`). A field `critic-elixir` run looked for the local directory, failed to find it, and fell back with a confusing "rule library not installed at the expected path" diagnostic, reviewing at reduced fidelity.

### Fixed

- **Generated guidance points at the CLI, not a local directory.** The `intent agents sync` generator, the `_CLAUDE.md` / `_usage-rules.md` templates, and the `_default` / per-language `AGENTS.md` + `RULES.md` agent templates now describe rule access as served by the installed Intent tool via `intent claude rules list` / `show <id>`, with no reference to a local rules directory that does not exist in consumers.
- **`critic-<lang>` subagents resolve rules through the CLI.** All five critics (`elixir`, `rust`, `swift`, `lua`, `shell`) now enumerate via `intent claude rules list` and read each rule with `intent claude rules show <id>`, partitioning code-vs-test mode by the `category` column. The CLI already merges canon and `~/.intent/ext/` rules with provenance and id-shadowing, so the per-critic extension-merge step was removed (one enumeration path, per Highlander). The `elixir-test-critic` upstream probe, which sits outside the CLI's reach, is retained.

### Changed

- **`intent upgrade` re-syncs installed subagents.** The upgrade path now runs `intent claude subagents sync` alongside the existing skills sync (failure-tolerant, no `--force`), so the corrected critics reach every machine's `~/.claude/agents/` mirror on the next upgrade rather than requiring a manual sync.

## [2.11.10] - 2026-05-28

Additive patch extending the `/in-whiteboard` skill with a stream-role vocabulary and an optional handle. Both are field-proven in Lamplight and generalised here so every Intent project inherits them. Opt-in like the rest of the protocol -- a project that declares no Verifier and uses no handles sees zero behaviour change.

### Added

- **Verifier stream role.** A new `## Stream roles` section documents an optional, advisory-only Verifier stream: the independent check that another stream's claimed/landed work is correct, complete, consistent, and faithful to what the user asked. It triangulates the _ask_ (the peer's Claude Code session transcript), the _plan_ (`~/.claude/plans/*.md`), and _reality_ (the whiteboard + `intent/st/**` + the code + the tests); fires on a "done" claim rather than continuously; reads the as-built with `file:line` evidence; classifies findings expected-vs-real; self-refutes high-severity findings before posting; and outputs to `asks.md` with direct escalation for a compounding false-"done". The Verifier never mutates another stream's code and never blocks its progress -- the user adjudicates, the owning stream fixes. It is a role a stream adopts, designated in the project's `whiteboard/README.md`, not a subcommand.
- **Recommended baseline operating model.** Streams and handles are per-project configuration declared in the project's own `whiteboard/README.md` -- any number of streams, any handles. The skill now recommends a baseline shape of one Control stream (heavy lifting) plus one Verifier stream (the independent check), with additional streams project-specific. The baseline is a recommendation, not a requirement; peer-only and other rosters remain valid.
- **Optional `handle:` stream-frontmatter field.** Short shorthand for terse asks-routing (eg `CC`, `VC`, `IC`). Additive: `stream_id` remains the routing key, so adding handles never breaks `pickup` or `asks`.

## [2.11.9] - 2026-05-23

Additive patch extending the `/in-whiteboard` skill with an `archive` subcommand. Field-tested by hand in Lamplight first (whose per-stream files had grown into append-only logs spanning ~a week and were costing real tokens on every `pickup`), then encoded into canon so the procedure is repeatable across all Intent projects. Opt-in by directory presence like the rest of the whiteboard protocol — projects without `intent/whiteboard/` see zero behaviour change.

### Added

- **`/in-whiteboard archive [as-of <YYYY-MM-DD>]`.** Rolls DONE/superseded content older than 2 days out of the live whiteboard files (`<stream>.md` + `asks.md` + the shared `<platform>.md`; never `README.md` or live ledgers) into weekly, Monday-anchored `history/<YYYYMMDD>.<file>` buckets keyed by the ISO week of the archived **content**, not the run date — so one run can append to several weekly buckets. It is judgment-guided, not a blind date filter: frontmatter, the current RESUME/STATUS block, standing reference, and any still-open item stay regardless of age; resolved asks, superseded status blocks, and absorbed decisions move. A one-line `> **[archived]** ...` pointer is left where content was removed. Concurrency-safe by construction: archive only your own stream file, or sweep all streams when peers are paused, and always commit via an explicit pathspec. History buckets are append-only and never reloaded on `pickup`; git history remains the ultimate trace.

## [2.11.8] - 2026-05-21

Patch fixing a multi-session deadlock in the `/in-session` UserPromptSubmit gate. With two or more Claude Code sessions open against the **same** Intent project, the gate blocked every prompt and `/in-session` never released it — the user was forced to manually `touch` the expected sentinel on every turn. Lamplight, which runs concurrent streams in one project, hit this constantly.

The cause was an asymmetric source of truth for "my session id". The gate (`require-in-session.sh`) read the real `session_id` from its hook payload and checked `/tmp/intent/in-session-<session_id>.sentinel`. The releaser (`release-gate.sh`, run by `/in-session`) had no payload, so it read the id from a shared per-project state file written by `SessionStart`. Concurrent sessions all wrote that one file, it held some other session's id, the releaser touched the wrong sentinel, and the gate deadlocked.

### Fixed

- **Single source of session identity.** Both the gate and the releaser now resolve identity from `$CLAUDE_CODE_SESSION_ID`, the env var Claude Code exports into every hook and Bash tool invocation. The two sides agree by construction, with no shared mutable file between them. When the env var is absent (an older Claude Code build) both sides degrade to the same `unknown` sentinel, which the releaser always touches, so they still agree and the gate self-heals. Concurrent sessions in one project are now fully supported.

### Removed

- **Shared per-project session-id state file.** `session-context.sh` (the `SessionStart` hook) no longer persists the session id to `/tmp/intent-claude-session-current-id-<key>`; that file was the concurrent-session corruption source and is no longer load-bearing. `release-gate.sh` drops its state-file and legacy-file reads. Stale copies left in `/tmp` are inert (never read) and need no cleanup.

## [2.11.7] - 2026-05-18

Additive patch shipping the multi-session coordination protocol designed in a parallel Lamplight session and live-tested in `/Users/matts/Devel/prj/Lamplight/intent/whiteboard/`. Two concurrent Claude Code sessions on the same Intent project now have a real live-channel between them instead of "wait for the other session's `wip.md` to update at next session-end". ST0040 captures the design, alternatives considered, and the deliberate deferrals; this release rolls it into formal canon.

The protocol is opt-in by presence: a project gains coordination only after it creates `intent/whiteboard/`. Projects without the directory see zero behaviour change — the chained `/in-whiteboard` step from `/in-session` and `/in-finish` skips silently.

### Added

- **`/in-whiteboard` skill** at `intent/plugins/claude/skills/in-whiteboard/`. Subcommand surface: `pickup` / `claim` / `unclaim` / `touch` / `ask` / `decide` / `lamplight` / `release` / `status`. Each session belongs to a durable **stream** (eg `control`, `ia-ux`) that owns one file under `intent/whiteboard/`; shared `asks.md` carries cross-stream handoffs; a per-project shared `<platform>.md` file (eg `lamplight.md`, `core.md`) carries shared-platform-layer edit notices. Claims are by steel-thread ID only — no glob paths. Heartbeat-older-than-7-days marks a claim reclaimable; reclaim requires explicit user acknowledgement.

- **Chain integration**: `/in-session` step 5 auto-fires `/in-whiteboard pickup` (after gate release); `/in-finish` step 1 auto-fires `/in-whiteboard release` (before any `wip.md` / `restart.md` / `done.md` updates). Both opt-in by directory presence.

- **`asks.md` header conventions** layered on top of the required `to:` / `from:` line: optional `Re: <prior-ask-anchor>` for reply threads, optional `FYI only -- no response needed.` to mark info-dump asks the recipient stream should not queue a reply to. Borrowed from the cross-project LLMsend protocol (in-whiteboard is the intra-project sibling).

### Changed

- **`intent upgrade` auto-installs `in-whiteboard` and re-syncs the canon skill mirror.** Two calls inserted after the migration dispatcher completes: `intent claude skills install in-whiteboard` (idempotent for users without the skill) and `intent claude skills sync` (propagates the `in-session` / `in-finish` chain edits into any already-installed mirror). Both calls failure-tolerant — a missing `~/.claude/` mirror or a user "N" at an overwrite prompt should not break the upgrade. No `--force` is used, so user customisations are never silently lost.

- **Upgrade "Next steps:" output** gains a one-line pointer to the new "Multi-session coordination" section of `intent/docs/working-with-llms.md` so users know how to opt in.

### Caveat

A Claude Code session already running at upgrade time has the old `in-session` / `in-finish` prose loaded in context — the new chain only auto-fires from the **next** `/in-session` (after `/compact` or session restart). Manual `/in-whiteboard pickup` still works in the current session. New sessions started after upgrade get the chain.

### Tests

- `tests/unit/skills_commands.bats` enumerates `in-whiteboard` in the canonical roster covered by the `claude skills list shows available skills` invariant.
- `tests/unit/intent_upgrade_dispatcher.bats` gains a regression case asserting that a v2.10.x → current-target upgrade lands `in-whiteboard` at `~/.claude/skills/in-whiteboard/SKILL.md`. The test fakes `$HOME` so the install writes into a sandbox.

### Documentation

- `intent/docs/working-with-llms.md` gains a "Multi-session coordination" section after "Skills and /in-session auto-load". Covers the live-channel-vs-snapshot tense/reader/cadence distinction, the file layout, stream identity discovery, ST-only claims, shared platform layer pattern, chain integration, heartbeat semantics, and a pointer to the Lamplight live reference and ST0040 design rationale.

## [2.11.6] - 2026-05-15

Additive patch shipping one new Lua coding rule surfaced during Lamplight ST0163 WP-04 (Murder mechanic hook authoring). The rule formalises an idiom matts called canon-worthy after seeing it applied to `worlds/v4/murder/experiences/murder_on_the_weekend/{phase,night_kill,facts}.lua`: "way more readable than loads of imperative if/then blocks."

### Added

- **IN-LU-CODE-006 — Dispatch table over if-chain for value dispatch**. Lua has no pattern matching and no multi-head function definitions; the idiomatic substitute is a table-of-functions keyed by the discriminating value, with a single lookup + invoke at the call site. The rule forbids `if/elseif` chains dispatching on a value to different downstream function calls and prescribes the `HANDLERS` table idiom instead. Guard clauses on derived booleans (alive checks, nil checks, invariant violations) stay as `if`. Concretises IN-AG-PFIC-001; sister rule IN-EX-CODE-001 (Elixir multi-head dispatch). Enforcement is via the `critic-lua` subagent (prose Detection); no Greppable proxy block, in line with the existing Lua-pack convention.

### Tests

- `tests/unit/rule_pack_lua.bats` registers the new rule in its canonical-id enumeration; the existing presence + count + validator + list invariants now cover IN-LU-CODE-006.
- `tests/fixtures/critics/lua/code/would-catch/sample.lua` gains a `perturbation.tag` dispatch chain so the would-catch fixture exercises the new rule; `manifest.txt` lists IN-LU-CODE-006.

## [2.11.5] - 2026-05-05

Behavioural patch fixing three latent bugs surfaced by a Conflab session 2026-05-05. All three were shipped-as-broken; the first two silently produced output that looked plausible while dropping load-bearing content; the third silently regressed a project's recorded version stamp.

### Fixed

- **`intent treeindex` reported "empty response from Claude" for every directory** when run inside any v2.10.0+ Intent project. Root cause: the spawned `claude -p` session inherits the project's `UserPromptSubmit` hooks; the strict gate (`require-in-session.sh`) fires on the first prompt, sees no `/in-session` sentinel for the ephemeral session_id, and exits 2; the non-bare `claude -p` swallows the hook's stderr and exits 0 with empty stdout. Treeindex saw empty stdout and reported per-directory failures. The treeindex tool was fine; the gate was the silent killer.

- **`intent agents generate` produced a stripped AGENTS.md** (empty project name, no language scaffolding, no installed-skill enumeration, no conditional resource links) when invoked directly via the dispatcher. Root cause: the `generate` dispatch path did not call `load_intent_config`, leaving `PROJECT_ROOT` empty so every per-project detection (`mix.exs`, `Cargo.toml`, `.claude/skills/`, `CLAUDE.md`, `usage-rules.md`) silently failed. `intent agents sync` was unaffected because it pre-loads config. Latent since the dispatcher was first added 2025-08-20; surfaced today when `generate` was invoked standalone for a diff repro.

- **`migrate_v2_10_x_to_v2_11_0` hard-coded the target stamp to `"2.11.0"`** instead of the live Intent target. A project walked up from v2.10.x through the migration path would land with `intent_version = "2.11.0"` regardless of which v2.11.x patch was current. Field impact was muted because `needs_v2_11_0_upgrade` short-circuits projects already carrying the `languages` field, but the bug existed and would have stamped v2.11.5 projects as "2.11.0". Fix stamps `get_intent_version`.

### Changed

- **`require-in-session.sh` accepts `INTENT_SKIP_IN_SESSION_GATE=1` as an explicit bypass.** Non-interactive automation has no chat surface for `/in-session` to run in, so the gate has no UX affordance for those sessions. The env var is the opt-out wrappers set; the gate short-circuits to exit 0 before any other check. Interactive sessions and untagged automation continue through the normal sentinel-based gate.

- **`bin/intent_treeindex` sets `INTENT_SKIP_IN_SESSION_GATE=1` on its `claude -p` invocation.** Treeindex is automation by definition; the bypass is unconditional.

- **`intent_agents_generate_content` self-loads project context.** The fix moves the `load_intent_config` + `require_project_root` guard from the dispatcher branches into the function itself, so any caller (dispatcher, `init`, `sync`, future automation) gets a consistent project context without duplicating the load preamble. Highlander applied: one source of truth for "this function needs `PROJECT_ROOT`."

### Documentation

- `intent/docs/working-with-llms.md` D7 documents the `INTENT_SKIP_IN_SESSION_GATE` bypass and adds it to the FAQ fix list.
- `intent help treeindex` lists the env var under a new `ENVIRONMENT` section so future `claude -p` wrapper authors can replicate the convention.

### Tests

- `tests/unit/require_in_session_gate.bats` covers the bypass branch and the existing slash-command / sentinel pass-throughs as regression smokes.
- `tests/unit/intent_agents.bats` gains a regression case asserting `intent agents generate` populates project name, language detection, and installed-skill enumeration when invoked with `PROJECT_ROOT` unset.
- `tests/unit/intent_upgrade_dispatcher.bats` gains a regression case asserting that a v2.10.x project upgraded via the migration path lands with the live target stamp (read from `VERSION`), not a hard-coded literal.

## [2.11.4] - 2026-04-30

Docs-only patch following v2.11.3's field verification.

### Documentation

- **Critic runner code locality** — `intent/docs/critics.md` "Headless runner" section gains a note clarifying that the runner (`bin/intent_critic` + `critic_runner.sh`) and the canon rule library load from `$INTENT_HOME` (the Intent install on `$PATH`), not from each project's plugin tree. A fix to the runner or a canon rule applies to every Intent project the moment Intent itself updates. Per-project canon refresh (`intent claude upgrade`) keeps `intent/llm/RULES*.md` and `.claude/skills/` copies in sync with the Intent version, but is not a prerequisite for gate behaviour to change. The behaviour itself is unchanged from v2.11.3 — only the docs are clarified.

### Verification

- v2.11.3's strict-proxy fix smoke-tested in Conflab (the canonical field witness) on 2026-04-30. Previously-misfiring patterns clear: `IN-EX-CODE-004` no longer flags single-step `case ... do`; `IN-EX-TEST-003` no longer flags compliant `use ExUnit.Case, async: true`. Gate still produces signal on real violations (e.g. `IN-EX-TEST-001` weak assertions, `IN-EX-TEST-005` control flow in tests, `IN-EX-TEST-007`). No stderr `note: skipping` diagnostics — expected, since the stripped rules no longer carry proxy blocks for the runner to refuse.

## [2.11.3] - 2026-04-29

ST0039 ships: pre-commit critic gate stops emitting findings derived from `Greppable proxy` regexes the headless runner cannot honour. Defect fix to behaviour shipped as broken in v2.11.0; no new feature surface, no schema change.

### Fixed

- **Pre-commit gate false positives on `IN-EX-CODE-004` (with-for-railway)**. Field report from a Conflab session post-upgrade-to-v2.11.0: the gate flagged every `case ... do` line in two LiveView files (22 false positives in a 3-file diff), forcing back-to-back `--no-verify` commits. Root cause: `bin/intent_critic`'s parser extracted only the first quoted regex from a multi-line proxy block and ran it as `grep -nE`, silently dropping the `| wc -l` qualifier that made the line a counter heuristic rather than a detector. The rule's _actual_ detection — "two or more nested fallible calls without `with`" — is body-confirmation territory and not expressible as a single-file regex.

- **Pre-commit gate false positive on `IN-EX-TEST-003` (async-by-default)**. Same Conflab session: the gate flagged the _compliant_ `use ExUnit.Case, async: true` line. Root cause: the documented proxy uses `grep -rnL ... | xargs grep -l ...` (find files lacking `, async: true`); the runner extracted the first quoted argument and ran it forward as `grep -nE`, matching the compliant form.

- **Runner silently degraded complex grep proxies** (pipes, `xargs`, `grep -L`, `grep -v`, awk, multi-line continuations). `critic_pattern_from_grep_command` consumed the first single-quoted argument from any proxy bash block and emitted findings as if the runner had executed the full pipeline. Replaced with a strict-proxy contract.

### Changed

- **`bin/intent_critic` runner contract** is now strict. `critic_runner.sh` accepts only proxy lines of the form `grep [-r|-n|-E|-rn|-rE|-nE|-rnE|--include=GLOB ...] '<pattern>' [<path>...]`: single grep invocation, no pipes, no `xargs`, no `-L` / `-v` / `-B` / `-A` / `-l` / `-c` / `-o` / `-w` / `-x`. Multi-line proxy blocks are accepted as a union of simple lines (findings deduped on `(line, content)`). Lines the runner cannot honour are refused with a once-per-rule stderr diagnostic `note: skipping <rule_id> (proxy not headless-runnable)`. Loud > silent.

- **`intent/docs/critics.md`** "Mechanical subset only" paragraph rewritten to document the strict-proxy contract and the stderr diagnostic.

### Removed

- **Greppable proxy blocks** stripped from 8 Elixir rules whose detection cannot be expressed as a simple single-file regex. The rules themselves are unchanged in prose and still apply via `/in-review` (LLM `critic-elixir` subagent does the body confirmation):
  - `IN-EX-TEST-003` (async-by-default) — inverse semantics.
  - `IN-EX-CODE-003` (impl-true-on-callbacks) — line continuation + negative filter.
  - `IN-EX-LV-001` (two-phase-mount) — `-B5` context + negative filter.
  - `IN-EX-LV-003` (thin-liveviews) — awk state machine for line counting.
  - `IN-EX-PHX-001` (thin-controllers) — awk state machine for line counting.
  - `IN-EX-ASH-001` (code-interfaces-only) — callsite scope cannot be inferred per-file.
  - `IN-EX-ASH-002` (actor-on-query) — proxy was inverted (fired on compliant code).
  - `IN-EX-TEST-004` (start-supervised) — required `grep -v start_supervised` filter.

- **`IN-EX-CODE-004` (with-for-railway)** counter line — the `case.*do$ | wc -l` heuristic is gone; the rule now ships only the legitimate `error -> error` forwarder detector. Single-step `case` blocks are no longer mechanically flagged.

- **`critic_pattern_from_grep_command`** function in `critic_runner.sh`. Replaced by `critic_proxy_is_simple` predicate + `critic_patterns_from_grep_block` walker. No back-compat shim (fail-forward).

## [2.11.2] - 2026-04-28

Second hotfix following v2.11.0/v2.11.1.

### Fixed

- **`intent upgrade` failed with `Error: Unknown version: 2.11.0`** when applied to a project already at v2.11.x. The dispatcher in `bin/intent_upgrade` had cases for every released source version up to v2.10.1 but none for v2.11.0; an in-flight project at v2.11.0 attempting to upgrade to v2.11.1 fell into the `*) error "Unknown version: $VERSION"` arm. Added a `"2.11.*"` case that runs the idempotent v2.11.x migration (no-op on field-already-present and stamp-already-current); a glob match rather than a literal so future patches in the v2.11 line don't need a fresh case each time.

## [2.11.1] - 2026-04-28

CI hotfix following v2.11.0.

### Fixed

- **Pre-commit hook errors on `set -u` + empty `LANGS` array** under some bash versions (CI macOS runner). v2.11.0 introduced the empty-array path (a project with `languages: []` declares zero critics), but the existing `for lang in "${LANGS[@]}"; do` loop expansion erred as "unbound variable" before the loop body ran. Length-guarded the loop with an explicit `if [ "${#LANGS[@]}" -gt 0 ]; then` check. Local installs need to re-run `intent claude upgrade --apply` to pick up the corrected hook template; new installs from v2.11.1 onward get the fix automatically.

## [2.11.0] - 2026-04-28

ST0037 ships: languages-in-use becomes an explicit per-project configuration field, replacing four sites of filesystem-marker probing. The probe-based detection was a regression against design intent (filesystem presence is unreliable evidence; a vendored example or a one-off script can flip the wrong switch). Schema change is automatic via migration; existing fleet projects need no user action beyond `intent upgrade`.

### Added

- **`languages: []`** field in `intent/.config/config.json`. Array of canonical language names (`elixir`, `rust`, `swift`, `lua`, `shell`). Array order is the explicit declaration; the first entry is the primary where a primary is needed. Empty array is a valid state.
- **`intent lang remove <lang> [<lang> ...]`** -- new verb. Reverses `intent lang init`: removes the entry from the agnostic `RULES.md` Language Packs marker block, deletes `intent/llm/RULES-<lang>.md` and `intent/llm/ARCHITECTURE-<lang>.md`, removes the language from `intent/.config/config.json`. Idempotent: a never-installed language emits `noop:` and returns 0.
- **`get_project_languages()`** helper in `bin/intent_helpers`. Reads the field via `jq`, returns one language per line, returns 0 lines when the array is empty or the field is absent. Used by the pre-commit critic gate.
- **`add_project_language()`** / **`remove_project_language()`** helpers in `bin/intent_helpers`. Atomic config-field mutation via tempfile + `mv`.
- **`migrate_v2_10_x_to_v2_11_0`** in `bin/intent_helpers`. Adds the `languages` field with back-fill from existing `intent/llm/RULES-<lang>.md` presence (alphabetical for determinism). When the back-fill set is empty AND a pre-commit hook is installed, falls back to `["shell"]` to preserve current "shell-always" gate behaviour. Idempotent: if the field is already present, only stamps the version.
- **BATS coverage** for the migration (`tests/unit/migrate_v2_10_x_to_v2_11_0.bats`), the new `intent lang init` config writes and the new `intent lang remove` verb (`tests/unit/intent_lang.bats`), the config-driven critic dispatch (`tests/unit/critic_dispatch.bats`), the config-driven pre-commit gate (`tests/unit/pre_commit_hook.bats`), and the regression guards on `/in-session` SKILL.md (no filesystem probes, no phantom skill refs).

### Changed

- **`/in-session` SKILL.md** -- detection table replaced with config-driven flow. The skill reads `(.languages // []) | .[]` from `intent/.config/config.json` and invokes any matching essentials skill. Currently only `/in-elixir-essentials` (and `/in-elixir-testing`) are real per-language essentials skills; rust/swift/lua/shell coding rules ship via the rule library at `intent/plugins/claude/rules/<lang>/` plus the `critic-<lang>` subagent applied on demand.
- **`/in-review` SKILL.md** -- stage-2 dispatch table replaced with config-driven flow. Reads the `languages` array, dispatches one critic per language listed.
- **`/in-tca-audit` SKILL.md** -- critic-selection table replaced with the same config-driven dispatch.
- **`lib/templates/hooks/pre-commit.sh`** -- the `LANGS+=(elixir)` etc. probe block is gone. The hook reads `languages` from config; an empty array means no language critics run, mirroring the explicit-config contract.
- **`bin/intent_init`** -- fresh projects get `"languages": []` in their initial config. The existing `--lang <lang>` flag still seeds the array via `intent lang init`.
- **`intent/docs/working-with-llms.md`** -- "Skills and /in-session auto-load" section rewritten to describe the config-driven flow; the four phantom skill references (`/in-rust-essentials`, `/in-swift-essentials`, `/in-lua-essentials`, `/in-shell-essentials`) are gone.

### Removed

- **Filesystem-probe-based language detection** at four canon sites (`in-session/SKILL.md`, `in-review/SKILL.md`, `in-tca-audit/SKILL.md`, `lib/templates/hooks/pre-commit.sh`). File presence is no longer treated as evidence of language-in-use.
- **Phantom skill references** to `/in-rust-essentials`, `/in-swift-essentials`, `/in-lua-essentials`, `/in-shell-essentials`. Those skills were promised in WP06/WP12 ("ships in WPNN") in the v2.10.x SKILL.md but never authored. The rule-pack + critic-subagent path is the working mechanism for those four languages and the prose now reflects that.

### Fixed

- **`create_v2_directory_structure()` in `bin/intent_helpers`** -- previously created an empty top-level `.intent/` unconditionally during `intent upgrade`, even on projects already on the v2.10 layout (`intent/.config/` present). The next phase (`migrate_v2_9_0_to_v2_10_0`) then refused to proceed because both `.intent/` and `intent/.config/` existed. Skips the `.intent/` creation when `intent/.config/` is in place. Pre-existing latent bug, surfaced when chaining the v2.9.0 → v2.10.0 → v2.11.0 migration sequence.
- **critic-elixir false positives on canonical OTP/Mix idioms (ST0038)**. Three rules misfired in the headless pre-commit gate against correct code (Lamplight ST0163/WP-01 commit attempt):
  - **`IN-EX-TEST-002` (no-process-sleep)** fired on `Process.sleep(:infinity)` in a `Mix.Task.run/1` body in `lib/`. The rule's frontmatter declared `applies_to: ["test/**/*_test.exs"]` but the runner ignored the field. Fixed by adding `applies_to` honoring in `critic_apply_rule` (`intent/plugins/claude/lib/critic_runner.sh`); globs are matched with suffix anchoring so umbrella layouts (`apps/<app>/lib/...`, `apps/<app>/test/...`) resolve correctly.
  - **`IN-EX-CODE-002` (tagged-tuple-returns)** fired on every public `def name(args) do` because the greppable proxy was too coarse to express "fallible function returns bare nil/false" as a per-file regex. Greppable proxy stripped; the rule remains active for the LLM-driven `critic-elixir` subagent via `/in-review`, where the body and call sites can be read.
  - **`IN-EX-CODE-006` (module-highlander)** fired on every public `def name(...)` for the same reason -- and the rule's actual concern (cross-module duplication) is fundamentally not a per-file scan. Greppable proxy stripped; subagent-applied via `/in-review`.
  - New BATS coverage in `tests/unit/critic_runner_applies_to.bats` (15 tests) verifies glob-to-regex translation, umbrella-layout matching, the absence of greppable proxies on the two stripped rules, and the presence of the proxy on `IN-EX-TEST-002`. `tests/unit/pre_commit_hook.bats` updated to stage fixtures under `test/<name>_test.exs` so they match `IN-EX-TEST-001`'s `applies_to`.

### Migration notes

- `intent upgrade` from any v2.10.x project runs `migrate_v2_10_x_to_v2_11_0` automatically. The migration is atomic, idempotent, and cannot lose user data.
- Polyglot projects: declare languages in primacy order with `intent lang init <primary> <secondary> ...`. The first entry is the primary; later entries follow. To change primacy, `intent lang remove <lang>` and re-init in the desired order.

## [2.10.1] - 2026-04-28

v2.10.x polish line. Two new pieces of maintainer infrastructure (a release script and a `intent doctor` migration-leftover warning), the gate-firing fix that surfaced post-v2.10.0 dogfood, and three pre-existing v2.10.0 dogfood-journal follow-ups that needed closing.

### Added

- **`scripts/release`** -- maintainer release orchestrator. Single-invocation cut: pre-flight (clean tree, doctor, tests, gh auth), version bump (`--patch / --minor / --major / vX.Y.Z`), CHANGELOG date finalisation, sidecar sync (VERSION + AGENTS.md), commit, idempotent tag, push to both remotes (local + upstream), GitHub release publication. Modelled on Conflab's release pattern, pared back to Intent's surface (single repo, two remotes, no native binary, no Homebrew tap). `--dry-run` previews every step with no side effects.
- **`intent doctor` check 4d** -- warning (not error) when a stale top-level `.intent/` directory remains after a v2.9 -> v2.10 migration. Auto-staging is intentionally NOT done: the user runs `git rm -rf .intent/` themselves so the cleanup is visible in the commit.

### Fixed

- **`/in-session` UserPromptSubmit gate-firing loop**. The SKILL.md inlined an awk pipeline whose positional-field expansion was silently emptied by Claude Code's skill renderer, producing a malformed project_key that prevented the per-session sentinel from being written. The waterfall is now in `intent/plugins/claude/skills/in-session/scripts/release-gate.sh`, invoked by the SKILL by path; the renderer never sees the pipeline.
- **`intent claude upgrade --dry-run` UX** -- distinguish three states for the `config.json` pre-flight (canonical, legacy `.intent/`, absent) so a pre-relocation project no longer reports its expected-missing config as a hard problem. The legacy-location case now points the user at `intent upgrade` for the relocation step.
- **`IN-RS-CODE-005` (lifetime-elision-first) carve-out** -- explicit "Does Not Apply" entry for teaching examples in `intent/plugins/claude/rules/**` and `tests/fixtures/critics/rust/**`. Closes the false-positive that fired on a clean fixture during ST0034 WP07 verification.

### Changed

- **Diogenes test-spec handoff** -- the four critic agent.md files (`critic-{elixir,rust,swift,lua}`) now uniformly suppress the Diogenes RECOMMENDATION for targets under `tests/fixtures/critics/`. Critic self-test fixtures are not real test code; the handoff was firing inconsistently across critics post-WP07.

## [2.10.0] - 2026-04-27

Retargeted from v2.9.1 mid-development to bundle ST0036 (directory relocation, breaking change) into the same release. Version bump reflects the semver-breaking directory move; LLM canon work (originally scoped as v2.9.1) ships alongside.

Two steel threads landed in this release:

- **ST0035** -- Canonical LLM Config + Fleet Rollout. Three-file root canon (`AGENTS.md` + `CLAUDE.md` + `usage-rules.md`), session hooks (SessionStart + UserPromptSubmit strict gate + Stop), pre-commit critic gate via `bin/intent_critic`, `.intent_critic.yml` per-project config, the `working-with-llms.md` canon narrative, and per-language canon (`intent lang init`).
- **ST0036** -- Directory relocation `.intent/` -> `intent/.config/`. Breaking change. Intent's per-project metadata directory moves from a separate top-level `.intent/` to a nested `intent/.config/`, eliminating the "two top-level dirs" smell. Migration handled atomically by `migrate_v2_9_0_to_v2_10_0` on `intent upgrade`.

Fleet rollout: 14 in-scope projects (Intent self + 8 canary + 5 user-manual; Pplr OOS) all on v2.10.0 canon. Canary discipline surfaced and resolved three canon-installer rough edges (`MIGRATE_LEGACY_PRE_COMMIT`, `CHAIN_PRE_COMMIT` auto-insert, `NORMALIZE_GITIGNORE`) before fleet sweep. See `intent/st/ST0035/WP/15/canary-summary.md`, `WP/16/fleet-summary.md`, `WP/17/feedback-report.md`, and `WP/17/dogfood-journal.md`.

### Added

- **ST0035** (Canonical LLM Config + Fleet Rollout).
- **ST0036** (Directory relocation: `.intent/` -> `intent/.config/`). Breaking change. Intent's per-project metadata directory moves from top-level `.intent/` to nested `intent/.config/`, eliminating the "two top-level dirs" smell. Migration handled atomically by `migrate_v2_9_0_to_v2_10_0` on `intent upgrade`.
- **`intent lang` command** (ST0035/WP-19) for per-language canon installation. Subcommands: `list`, `show`, `init`. `intent lang init <lang> [<lang> ...]` is idempotent and multi-language; copies `intent/plugins/agents/templates/<lang>/{RULES,ARCHITECTURE}.md` into `intent/llm/{RULES,ARCHITECTURE}-<lang>.md` and appends a marker-bracketed entry to the agnostic `intent/llm/RULES.md` Language Packs section. Replaces the rejected auto-language-detection approach (real projects are polyglot; explicit user choice via `--lang` is more honest). Available canon languages: `elixir`, `rust`, `swift`, `lua`, `shell`. New stub templates ship for the four newer languages.
- **`intent init --lang <list>`** flag invokes `intent lang init` for each named language post-init. Comma- or space-separated list. Equals form (`--lang=elixir`) also accepted.
- **Agnostic `_default` canon now includes RULES.md + ARCHITECTURE.md** in fresh `intent init`. Previously only `MODULES.md` + `DECISION_TREE.md` were laid down; canon-installer's `_default` templates were only seen via `intent claude upgrade --apply`. Now `intent init` produces a v2.10.0-complete baseline including the Language Packs anchor that `intent lang init` writes into.

### Changed

- `bin/intent_helpers`: `migrate_v2_9_0_to_v2_10_0()` replaces the earlier `migrate_v2_9_0_to_v2_9_1()` stub. Bundles version stamp + ST0036 directory relocation. Canon-apply logic still lands in ST0035/WP11 via `intent claude upgrade --apply` (separate step).
- `bin/intent_upgrade`: chain extended to v2.10.0 (new gate `needs_v2_10_0_upgrade`, new case, new chain tail).
- Root `VERSION` bumped to `2.10.0`.
- **Treeindex ignore canonicalised** (ST0036/WP-06): new `lib/templates/_treeindexignore` template is the single source of truth. `bin/intent_treeindex::ensure_treeindexignore` reads from the template instead of an inline heredoc (Highlander cleanup per CLAUDE.md project rule #6). `intent claude upgrade --apply` installs the file when absent (new `INSTALL_TREEINDEXIGNORE` action; existing files left alone). Granularity flipped from blanket `.intent/` to `intent/.config/cache/` + `intent/.config/backup/` so `config.json` stays indexed.
- **Pre-commit hook template** (ST0036/WP-04): `lib/templates/hooks/pre-commit.sh` now probes `intent/.config/config.json` instead of `.intent/config.json` when deciding whether to skip the critic gate (fail-open in non-Intent repos). Newly-installed gates and any project that re-runs `intent claude upgrade --apply` after v2.10.0 pick up the corrected probe.

### Breaking

- **Per-project metadata directory relocated**: `.intent/config.json` → `intent/.config/config.json`. Same for `.intent/backup/` → `intent/.config/backup/`. Anything scripting against `.intent/` (CI, editor plugins, ad-hoc `jq`) breaks on upgrade; update to `intent/.config/`. Migration is fail-forward: old location is pruned, no backwards-compat symlink. Full migration guide including recovery from interrupted upgrades: [`intent/docs/migration-v2.10.0.md`](./intent/docs/migration-v2.10.0.md).

### Removed

- **`intent/usr/*.md`** retired (ST0035/WP-18). The three hand-authored user docs (`user_guide.md`, `reference_guide.md`, `deployment_guide.md`) were stamped at `intent_version: 2.6.0` (2026-03-05), seven minor versions behind the v2.10.0 canon, and substantially duplicated by `README.md`, `intent/docs/working-with-llms.md`, `intent help <cmd>`, and `AGENTS.md`. Per fail-forward (no preservation, prune actively): all three deleted; the `intent/usr/` directory is gone. Cross-references updated: `README.md` Documentation and Getting Help sections now point at `intent/docs/working-with-llms.md` (canon narrative) + `intent help` (commands) + `intent/docs/migration-v2.10.0.md` (upgrade path); `docs/blog/0005-getting-started-with-intent.md` Intent Documentation section refreshed; `intent/docs/migration-v2.10.0.md` "unchanged subdirectories" list trimmed.
- **ST0010** (Anthropic MCP Integration, v2.0.0-era) cancelled — superseded by v2.9.0 skills / subagents / extensions. Moved to `intent/st/CANCELLED/` with deprecation annotation.
- **ST0015** (Enhanced Steel Thread Templates, v2.0.0-era) cancelled — superseded by v2.9.0 tooling. Moved to `intent/st/CANCELLED/` with deprecation annotation.

## [2.9.0] - 2026-04-23

### Added

- **ST0034: Agentic Software Engineering Suite.** Rules become first-class citizens of Intent. Each rule is an atomic Markdown file with structured frontmatter, a Detection heuristic, and bad/good examples. Skills cite rules by stable `IN-*` IDs; Critic subagents enforce them.
- **Rule library** at `intent/plugins/claude/rules/` with packs for `agnostic`, `elixir`, `rust`, `swift`, `lua`, and `shell`. Schema reference at `intent/plugins/claude/rules/_schema/rule-schema.md`. Schema is intentionally compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, 2026 Manuel Zubieta) so upstream rules drop into Intent's discovery unchanged.
- **`intent claude rules`** command surface: `list`, `show`, `validate`, `index`. The `validate` subcommand is the canonical authoring gate; `index` regenerates a deterministic, sorted `index.json`.
- **Critic subagent family**: `critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`. Thin orchestrators that read the rule library at invocation time, apply each rule's Detection heuristic to target source files, and emit a stable severity-grouped report. Modes: `code` and `test` (`critic-shell` is `code` only).
- **`.intent_critic.yml`** per-project config for disabling rules and adjusting severity thresholds. Sample at `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`.
- **User extension system** at `~/.intent/ext/<name>/` with the `intent ext` command surface (`list`, `show`, `validate`, `new`). Extensions contribute subagents, skills, or rule packs without modifying canon. Discovery is layered: canon is the default; user extensions override by name with a visible shadow warning. Manifest schema at `intent/plugins/claude/ext-schema/extension.schema.json`.
- **Reference extension `worker-bee`** at `~/.intent/ext/worker-bee/`. The migration seeds it from `lib/templates/ext-seeds/worker-bee/` on first run; further development happens at the user-local path, not in canon.
- **Authoritative documentation**: `intent/docs/rules.md` (rule library guide), `intent/docs/critics.md` (critic contract and report format), `intent/docs/writing-extensions.md` (extension authoring guide with worker-bee worked example).
- **Migration `migrate_v2_8_2_to_v2_9_0`** in `bin/intent_helpers`: stamps version, bootstraps `~/.intent/ext/`, seeds worker-bee, prunes installed copies of the deleted `elixir` subagent and the relocated `worker-bee` from `~/.claude/agents/` and `~/.intent/agents/installed-agents.json`. Idempotent — running the upgrade twice is safe and never overwrites user state.
- **`/in-session` bootstrap skill** for post-`/compact` skill loading.
- **`tests/unit/docs_completeness.bats`** verifies the new docs are present, cross-referenced, and that `intent agents sync` is idempotent.

### Removed

- **`elixir` subagent** (replaced by `critic-elixir` plus the Elixir rule pack). The migration aggressively prunes installed copies on upgrade.
- **`worker-bee` from Intent canon** (relocated to the reference extension at `~/.intent/ext/worker-bee/`). Re-install via `intent claude subagents install worker-bee` after the v2.9.0 upgrade.

### Changed

- **`in-standards` skill** loads agnostic rules by ID (no longer a "re-read CLAUDE.md" reminder).
- **`in-review` skill** stage-2 dispatches to `critic-<lang>` based on project language detection.
- **`in-elixir-essentials` and `in-elixir-testing` skills** declare machine-readable `rules:` frontmatter listing the IN-\* IDs they cite. Bodies remain rule-reference tables — content lives in the rule files.
- **TCA suite refactored for the rule library**: `in-tca-init` selects rule packs by ecosystem instead of inventing per-audit R-numbering; `in-tca-audit` dispatches `critic-<lang>` per WP and captures the verbatim critic report; `in-tca-synthesize` consumes the stable critic schema (CRITICAL/WARNING/RECOMMENDATION/STYLE + IN-_ IDs); `in-tca-remediate` and `in-tca-finish` cite IN-_ IDs throughout. The 1195-line `intent/docs/total-codebase-audit.md` is updated for v2.9.0; pre-v2.9.0 lessons-learned appendices are preserved with a historical-context note.
- **CLAUDE.md, MODULES.md, DECISION_TREE.md** updated for the v2.9.0 surfaces. DECISION_TREE.md gains three new branches: rule placement, skill placement, and rule-vs-skill-vs-subagent.
- **Help files** updated: `lib/help/ext.help.md`, `lib/help/rules.help.md`, `lib/help/claude.help.md` (now lists the `rules` subcommand and the `critic-*` family).
- **`creating-custom-agents.md`** distinguishes canon subagents from extension subagents; cross-links `writing-extensions.md`.

### Attribution

- Rule schema and selected rule principles inspired by [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, copyright 2026 Manuel Zubieta), pinned at commit `1d9aa40700dab7370b4abd338ce11b922e914b14`. See `intent/plugins/claude/rules/_attribution/elixir-test-critic.md`.

## [2.8.2] - 2026-04-15

### Fixed

- **ST0033: cwd-resilient dispatch.** `intent` subcommands now work from any directory inside an Intent project, not only from the project root. The dispatcher (`bin/intent`) exports `INTENT_ORIG_CWD` and `cd`s to `$PROJECT_ROOT` before `exec`'ing the subcommand, so every subcommand runs with a known-correct cwd. Outside any project, commands fail cleanly with "not in an Intent project" and no longer create stray `.intent/` or `intent/` directories at the invoker's cwd. `intent treeindex` and `intent fileindex` consult `INTENT_ORIG_CWD` when resolving relative path arguments.
- **Upgrade chain completed through 2.8.2.** `bin/intent_upgrade`'s case statement previously halted at 2.6.0 for any starting version <= 2.5.0 and had no entry for 2.6.0/2.7.0 at all, leaving projects stuck mid-chain. Every starting-version case now chains through `migrate_v2_6_0_to_v2_8_0` (new, pure version stamp), `migrate_v2_8_0_to_v2_8_1`, and `migrate_v2_8_1_to_v2_8_2`. The pre-v2 fallback chain is extended to match. `needs_v2_8_2_upgrade` accepts 2.6.0 and 2.7.0 as starting points.
- **ST0032: Credo custom checks wired into `.credo.exs`.** `intent st zero` (D5a) and `intent audit` now use a standalone `lib/scripts/configure_credo.exs` to programmatically patch `.credo.exs`, replacing the earlier wrong hint about `elixirc_paths` in `mix.exs` and the `intent audit --checks-dir` workaround. Removed 2 broken check templates (`boolean_operators`, `dependency_graph`), fixed 4 buggy ones (`map_get_on_struct`, `missing_impl_annotation`, `debug_artifacts`, `thick_coordinator`), and added `bracket_access_on_struct`. Existing projects that went through `st zero` can re-run D5a to pick up the wiring.

## [2.8.1] - 2026-04-09

### Added

- **TCA pre-flight guard** (`tca-report.sh --check-only`) with 4 checks: shape (WP/ dir, design.md with rule set), feedback-report.md exists, no unfilled `[Fill in:` placeholders, zero unchecked `- [ ]` acceptance criteria in info.md
- **Provisioning Invariants** (§ 0.0 in `intent/docs/total-codebase-audit.md`): four load-bearing rules -- TCA is its own dedicated steel thread, WPs are flat, last WP is synthesis, rank components by later-pain impact not raw violation count
- **`tca-init.sh` provisioning guards**: refuse to provision inside an existing `intent/st/ST*/WP/*` path, refuse to overwrite an audit with populated `socrates.md` files
- **False Positive Guidance as REQUIRED** in the `in-tca-init` design.md template, with an R8/R9 example. Lamplight benchmark: R8 false-positive rate dropped from ~82% to 0% with pre-classification.
- **Audit metadata line** in `in-tca-audit` Post-WP section: `**Agent**: {type}; **Turns**: N; **Raw hits**: N; **FPs**: N` at the top of each component audit's `socrates.md`
- **`chains_to:` frontmatter** on all 5 TCA skills: `in-tca-init` -> `in-tca-audit` -> `in-tca-synthesize` -> `in-tca-remediate` -> `in-tca-finish` -> `in-finish`

### Changed

- **BREAKING (internal TCA scripts)**: `--st-dir` renamed to `--tca-dir` across `tca-init.sh`, `tca-progress.sh`, `tca-report.sh`, and their SKILL.md invocations. 33 occurrences across 6 files. Shell variable `ST_DIR` renamed to `TCA_DIR`. Direct callers of these scripts must update their invocations.
- **`in-tca-finish` skill restructured**: feedback report is now a top-level artifact at `$TCA_DIR/feedback-report.md` rather than a "Feedback WP" `socrates.md`. The `/in-finish` wrap-up is gated on the pre-flight guard passing.
- **Dedup-rate KPI framing** in the TCA reference doc: low dedup rate on newly-authored code is now framed as a positive signal about rule-aware authorship.

### Fixed

- **Premature TCA close-out failure mode**: prevents the "lying session docs" window that occurred during Lamplight ST0121 (commits 75706c18 -> 98616a0c, 2026-04-08). The pre-flight guard makes this mechanically impossible.
- **Silent guard failures** in `tca-report.sh`: `grep -c` and `grep | wc -l` pipelines interacted badly with `set -euo pipefail` (grep returning 1 on zero matches killed the script silently on assignment). Replaced with pure-shell while-loop counters.

### Motivation

Integrates feedback from the Lamplight ST0121 TCA run (2026-04-08/09). The audit worked -- 17 raw violations found, 10 fixed -- but exposed 8 corrections in provisioning and close-out discipline. Documentation was not enough: an eager operator skipped past written guidance. This release replaces guidance with mechanical guards wherever the failure modes allow it. See ST0031 (5 commits, `58143ae..5b4435f`) for implementation detail.

## [2.8.0] - 2026-03-28

### Added

- **Detrope skill** -- `/in-detrope` for LLM trope detection and stylometric analysis
  - Trope catalog vendored from [llm-tropes](https://github.com/matthewsinclair/llm-tropes) (44 tropes, 8 categories)
  - Context-aware severity assessment (reads project CLAUDE.md for audience/purpose)
  - Two modes: `quick` (diagnosis) and `full` (diagnosis + concrete rewrites)
  - Stylometric profile with AI signal strength rating
  - Integrates with Utilz `cleanz --detrope` for automated pre-scanning

### Changed

- **Blog series detroped** -- all 8 blog posts revised to remove LLM writing tropes
  - Removed magic adverbs, landscape metaphors, negative parallelism, stakes inflation
  - Rewrote to sound human: varied rhythm, concrete detail, reduced AI cadence

## [2.7.0] - 2026-03-19

### Added

- **TCA v3.0** -- Total Codebase Audit process document updated from v2.0 to v3.0 (ST0028)
  - Validated Rust and Swift rules replacing hypothetical ones (from real polyglot audit)
  - Ash Framework supplemental rules (A1-A5) as first-class audit rules
  - Rule precision boundaries (R5 matchable-values-only, R7 defstruct-only)
  - Effective file count model for WP sizing (weight table: Ash DSL 0.25x, Rust 1.5x, etc.)
  - Phase 0.5 pre-filtering of mechanical rules via grep
  - Confidence field (HIGH/MEDIUM/LOW) on audit findings
  - 5-tier priority scheme (P0/P1/P2a/P2b/P3) replacing 4-tier
  - Deduplication by root cause, not rule number
  - Main conversation remediation model (not sub-agents)
  - Test optimization with `mix test --failed`
  - Example C (polyglot: 256 files, 59% dedup rate)
  - New lessons: anti-hallucination, R5 over-reporting, remediation agent failures, R7 false positives
- **TCA skill suite** -- 5 operational skills with 3 automation scripts
  - `/in-tca-init` -- provisioning (SKILL.md + tca-init.sh)
  - `/in-tca-audit` -- component audit execution (SKILL.md + tca-progress.sh)
  - `/in-tca-synthesize` -- cross-component synthesis
  - `/in-tca-remediate` -- batched remediation in main conversation
  - `/in-tca-finish` -- wrap-up and feedback report (SKILL.md + tca-report.sh)

## [2.6.0] - 2026-03-05

### Added

- **Plugin discovery** -- `intent plugin` command for discovering plugins and their commands
  - `intent plugin` / `intent plugin list` -- lists all plugins with command syntax
  - `intent plugin show <name>` -- detailed view of a single plugin
  - `plugin.json` metadata files in each plugin directory for structured discovery
- `intent help claude` -- help file for the claude command namespace
- `intent help plugin` -- help file for the plugin command
- **ST0026 Phase 1** -- Steel Thread Zero code quality enforcement
  - Skills renamed from `intent-*` to `in-*` prefix
  - `intent claude prime` command for memory injection
  - LLM templates: `_CLAUDE.md`, `_MODULES.md`, `_DECISION_TREE.md`, `_ARCHETYPES.md`
  - 9 Elixir archetype templates in `lib/templates/archetypes/elixir/`
  - 5 workflow skills: `in-start`, `in-plan`, `in-next`, `in-standards`, `in-finish`
  - TN004 total codebase audit tech note
- **ST0026 Phase 2** -- Automated enforcement and guardrails
  - `intent audit quick` command with 7 custom Credo check templates (R2, R6, R7, R8, R11, R15, D11)
  - `intent audit health` command with 4 health checks, `--report` and `--diff` flags
  - `intent learn` command for capturing project learnings (footgun/worked/failed)
  - `intent modules check` command for module registry guardrails
  - `intent modules find` command for searching the registry
  - Dependency graph Credo check template (`dependency_graph.ex`, rule D11)
  - Dependency graph template (`_DEPENDENCY_GRAPH.md`) for umbrella apps
  - Claude Code advisory hook template for unregistered module warnings
  - `intent st zero install` command for brownfield project retrofit (D12)
    - 4-phase process: Audit, Gap Analysis, Proposals, Apply
    - 9 ST0000 deliverables checked (D2-D11): CLAUDE.md, MODULES.md, ARCHETYPES.md, Credo checks, DECISION_TREE.md, MEMORY.md, module hook, learnings.md, DEPENDENCY_GRAPH.md
    - Auto-discovers modules from `.ex` files in `lib/` (or `apps/*/lib/` for umbrellas)
    - Flags: `--audit-only`, `--dry-run`, `--deliverable <ID>`
    - Elixir-specific deliverables (D4, D5a, D11) only installed when `mix.exs` present
  - `intent init --with-st0000` flag for greenfield projects (D1)
    - Runs full ST0000 bootstrap after standard project initialization

### Changed

- `intent audit health` now umbrella-aware -- scans `apps/*/lib/` in umbrella projects
- `intent audit health` Highlander suspects reformatted to multi-line output (function name + indented files)
- `intent audit quick --checks-only` now force-copies templates (ensures updates applied on re-run)
- Rationalized CLI output across all commands to Rust-style conventions
  - Lowercase status prefixes: `ok:`, `error:`, `warning:`, `hint:`
  - Action prefixes: `created:`, `updated:`, `removed:`, `started:`, `done:`
  - No separator bars, banners, or unicode decorations
- `intent st` now supports `--help`/`-h` flags

### Fixed

- Credo template `thick_coordinator.ex`: `@default_params` interpolation before definition
- Credo template `highlander_suspect.ex`: unused variable warning on `_arity`
- Credo template `debug_artifacts.ex`: unused `@debug_calls` module attribute removed
- `intent help` now shows `claude` and `plugin` commands in Core section
- `intent help` agents description corrected from "Manage Claude Code sub-agents" to "Manage AGENTS.md for projects"
- `lib/help/agents.help.md` rewritten to document actual AGENTS.md commands (was documenting subagent operations)
- `intent help` now shows Plugins section pointing to `intent plugin`

## [2.5.0] - 2026-02-24

### Added

- **Work package management** -- `intent wp` as a top-level command (ST0024)
  - `intent wp new <STID> "Title"` -- create next WP in STID/WP/NN/info.md
  - `intent wp done <STID/NN>` -- mark WP as Done, hint when all WPs complete
  - `intent wp start <STID/NN>` -- mark WP as WIP
  - `intent wp list <STID>` -- table with WP, Title, Scope, Status columns
  - `intent wp show <STID/NN>` -- display WP info.md
  - `intent wp help` -- show usage
  - Specifier syntax: `ST0011/01` or shorthand `11/01`
  - WP info.md template at `lib/templates/prj/st/WP/info.md`
  - 29 new BATS tests in `tests/unit/wp_commands.bats`
- Shared helpers extracted to `bin/intent_helpers`:
  - `normalise_st_id()` -- normalizes bare numbers and partial IDs to ST#### format
  - `escape_sed_replacement()` -- escapes special characters for sed substitutions
- `intent-essentials` skill Rule 8: "Use `intent wp` commands for work package management"
- **Shared plugin helper library** -- Highlander audit refactoring
  - `intent/plugins/claude/lib/claude_plugin_helpers.sh` -- shared install/sync/uninstall via callbacks
  - `intent_claude_skills` reduced from 654 to 299 lines
  - `intent_claude_subagents` reduced from 1015 to 613 lines
  - `get_config_field()` in `bin/intent_helpers` replaces inline `grep -oE` config extraction

### Removed

- **Backlog.md integration** -- all backlog commands and configuration removed (ST0023)
  - Removed `intent bl` / `intent backlog` wrapper command
  - Removed `intent task` command (create, list, sync)
  - Removed `intent status` command (show, sync, report)
  - Removed `intent migrate` command (embedded task migration)
  - Removed `backlog_dir` and `backlog_list_status` configuration keys
  - Removed backlog directory creation from `intent init`
  - Removed backlog optional tool check from `intent doctor`
  - Removed backlog references from all subagent definitions
  - Removed Node.js setup from CI pipeline (was only needed for Backlog.md)
  - Deleted 3 test files (bl_commands.bats, task_commands.bats, migration.bats)
  - Test suite reduced from 17 to 14 files

### Changed

- Documentation updated with WP bare number syntax, special character support, and directory structure
- CI pipeline simplified: no longer requires Node.js installation
- `intent help` no longer lists backlog-related commands
- `intent info` no longer shows Backlog section
- Consolidated duplicate `version`/`intent_version` config fields to just `intent_version`
- TPD files annotated with "[Removed in v2.5.0]" for historical backlog sections
- Blog posts annotated with editor's notes about removal

### Fixed

- Test side-effect: `agent_commands.bats` no longer modifies real source files during test runs
  - Added `create_source_sandbox()` for tests that simulate source changes
  - Removed `git checkout` from teardown that was reverting uncommitted edits

## [2.4.0] - 2026-02-17

### Added

- **Skills system** -- new always-on enforcement layer for Claude Code (ST0020)
  - `intent claude skills list` -- show available and installed skills
  - `intent claude skills install <name>` -- install skill(s) to `.claude/skills/`
  - `intent claude skills sync` -- update installed skills with latest versions
  - `intent claude skills uninstall <name>` -- remove Intent-managed skills
  - `intent claude skills show <name>` -- display skill content and status
  - SHA256 checksum-based manifest tracking at `~/.intent/skills/installed-skills.json`
- Six skills for proactive code enforcement:
  - `intent-essentials` -- 7 Intent workflow rules (CLI usage, treeindex, steel thread conventions)
  - `intent-elixir-essentials` -- 8 core rules (pattern matching, tagged tuples, pipes, naming)
  - `intent-ash-ecto-essentials` -- 7 Ash/Ecto rules (code interfaces, migrations, actor placement)
  - `intent-phoenix-liveview` -- 7 LiveView rules (two-phase mount, streams, components)
  - `intent-elixir-testing` -- 8 mandatory test quality rules (no control flow in tests, strong assertions, spec-driven)
  - `intent-autopsy` -- session forensics and memory meta-learning (ST0021)
- **Diogenes subagent** -- Elixir Test Architect using Socratic dialog (ST0020 WP-11)
  - Two personas: Aristotle (Empiricist) and Diogenes (Skeptic)
  - Specify mode: 5-phase dialog producing `*.spec.md` test specifications
  - Validate mode: gap analysis comparing specs to test files
- **intent-autopsy skill** -- session forensics and memory meta-learning (ST0021)
  - Inspired by [@chickensintrees](https://github.com/chickensintrees) and adapted from his work with STEF
  - Elixir script (`autopsy.exs`) pre-processes JSONL session files
  - Detects correction pairs, frustration signals, capability regressions, banned patterns
  - Memory-aware analysis: compares findings against MEMORY.md and CLAUDE.md rules
  - Identifies memory gaps, enforcement failures, undocumented conventions, stale memory
  - Ships default `banned-words.txt` with common AI-isms (delve, unfortunately, etc.)
- `intent claude upgrade` command for diagnosing and upgrading LLM guidance layer
  - Dry-run by default (use `--apply` to execute)
  - `--project-dir DIR` to target external projects
  - Diagnoses files, subagents, and skills; generates upgrade plan; applies changes
- Elixir subagent reference documents:
  - `ash-ecto.md` -- Ash/Ecto database patterns (Ash-first, never raw Ecto)
  - `liveview.md` -- LiveView operational patterns (two-phase rendering, streams, uploads)
  - `testing.md` -- Testing reference (DataCase, ConnCase, LiveView, Mox, Ash testing)
  - `project-structure.md` -- Standard Phoenix/Ash project layout
- Elixir project templates for `intent agents init --template elixir`:
  - `AGENTS.md` template with Elixir project overview and commands
  - `RULES.md` template with 9 core rules + framework rules + NEVER DO list
  - `ARCHITECTURE.md` template with domain map and directory structure skeleton
- `usage-rules.md` -- Intent's own LLM-optimized usage reference (~310 lines)
- `docs/upgrade-guide-2.4.0.md` -- human-readable upgrade guide for Intent projects
- **Special character handling** in `st new` -- titles with `/`, `&`, `\` no longer break creation (ST0022)
- **Slug generation** -- `st new` auto-generates a URL-safe `slug:` field in frontmatter, max 50 chars (ST0022)
- **`-s|--start` flag** for `st new` -- create and immediately start a steel thread in one command (ST0022)
- `intent doctor` now checks for Elixir installation (optional, needed for autopsy)
- BATS tests across 17 test files

### Changed

- Refactored Elixir subagent rules from 23 overlapping to 12 non-overlapping rules
  - Organized into 5 categories: Data Access, Control Flow, Composition, Error Handling, Code Hygiene
  - Each rule is distinct with no overlap between categories
- `intent claude skills install` now copies entire skill directory (not just SKILL.md)
  - Scripts and supporting files installed alongside SKILL.md
  - `intent claude skills sync` also copies full directory on update
- Updated `intent agents init` to support `--template <name>` flag
  - Template copies AGENTS.md, RULES.md, ARCHITECTURE.md from template directory
  - RULES.md and ARCHITECTURE.md are human-curated (not overwritten without `--force`)
- Added NEVER DO rule: never put `require` inside a function body (module level only)
- Added YAML frontmatter with `description` field to all SKILL.md files for Claude Code discovery
- `st list` and `st sync` now show "Slug" column instead of "Title" (falls back to title for older threads)
- Updated copyright to 2026 across all source files

## [2.3.4] - 2026-02-04

### Added

- `intent treeindex DIR` command for LLM-optimized directory summaries (ST0019 WP01)
  - Bottom-up directory indexing with Claude Haiku 4.5 for summarization
  - Centralized shadow directory at `intent/.treeindex/` keeps source tree clean
  - `.treeindexignore` configuration for excluding files/dirs from indexing
  - Auto-generated `README.md` in `.treeindex/` shadow directory for LLM orientation
  - Fingerprint-based staleness detection (filenames + sizes, no mtime dependency)
  - `--check` mode for CI/reporting without regeneration
  - `--dry-run` mode to preview without writing
  - `--force` to regenerate regardless of staleness
  - `--depth N` to control directory traversal depth (default 2)
  - Platform-compatible (macOS/Linux stat differences handled)
  - Bash 3.2 compatible (works with macOS default `/bin/bash`)
- 41 bats tests for treeindex command in `tests/unit/treeindex_commands.bats`
- CLAUDE.md convention: check `intent/.treeindex/<dir>/.treeindex` before exploring unfamiliar directories
- Release notes documentation in `docs/releases/2.3.4/RELEASE_NOTES.md`

### Fixed

- `intent init` now displays correct version from VERSION file instead of hardcoded 2.0.0
- `--sync` flag bug in steel thread management

### Changed

- Expanded Elixir subagent with architectural principles, Ash/Phoenix patterns, and testing guidance
- Replaced 'eg' abbreviation throughout documentation (was 'e.g.,')
- Updated all documentation to match as-built codebase (was frozen at v2.1.0)
  - `.github/workflows/README.md`: Full rewrite from STP to Intent
  - `tests/README.md`: Updated to v2.3.4 with all 14 test files
  - `README.md`: Fixed project structure, added treeindex/AGENTS.md/subagent commands
  - `intent/usr/user_guide.md`: Added treeindex, AGENTS.md, Claude subagent sections
  - `intent/usr/reference_guide.md`: Added treeindex, fileindex, agents, subagent command references
  - `intent/usr/deployment_guide.md`: Added plugin/subagent deployment and treeindex integration
  - `examples/hello-world/README.md`: Updated to v2.3.4 with current structure

### Migration

- Added `migrate_v2_3_3_to_v2_3_4()` function in `bin/intent_helpers`
- Added `needs_v2_3_4_upgrade()` function in `bin/intent_helpers`
- Updated `bin/intent_upgrade` to handle v2.3.3 -> v2.3.4 upgrade path
- All version upgrade paths updated to include v2.3.4 migration

### Technical Improvements

- Treeindex uses headless `claude -p` with `--tools ""` for text-in/text-out summarization
- Shadow directory design avoids polluting source tree with index files
- Fingerprint design is git-clone-stable (no mtime dependency)
- Full test suite now at 265 tests

## [2.3.3] - 2025-10-02

### Added

- Comprehensive Elixir style guide for the Elixir Claude subagent
  - Module organization (imports, aliases, whitespace)
  - Function definitions and multiline preferences
  - Testing patterns and fixture design
  - Code composition and pipeline usage
  - Naming conventions and ubiquitous language
  - Documentation standards
  - Type specifications
  - Dependency management
  - Database design precision
  - Version control conventions
- Full style documentation in `intent/plugins/claude/subagents/elixir/style.md`
- Release notes documentation in `docs/releases/2.3.3/RELEASE_NOTES.md`

### Changed

- Updated `intent/plugins/claude/subagents/elixir/agent.md` to reference style guide alongside antipatterns
- Enhanced Elixir subagent now provides both antipattern detection (v2.3.2) and style guidance (v2.3.3)

### Migration

- Added `migrate_v2_3_2_to_v2_3_3()` function in `bin/intent_helpers`
- Added `needs_v2_3_3_upgrade()` function in `bin/intent_helpers`
- Updated `bin/intent_upgrade` to handle v2.3.2 → v2.3.3 upgrade path
- All version upgrade paths updated to include v2.3.3 migration

### Technical Improvements

- Elixir subagent now provides holistic code quality guidance combining antipatterns and style
- Style guide complements antipattern detection for comprehensive code reviews
- Improved upgrade mechanism with full test coverage (212 tests passing)

## [2.3.2] - 2025-09-04

### Added

- Comprehensive antipattern detection to Elixir subagent
  - Detects and remediates 24 common Elixir antipatterns
  - Antipatterns categorized into Code (9), Design (6), Process (4), and Meta-programming (5)
  - Full documentation in `intent/plugins/claude/subagents/elixir/antipatterns.md`
  - Antipatterns sourced from official Elixir documentation
- Antipattern review workflow integrated into Elixir Doctor
- Example usage commands and report formats for antipattern detection
- Key principles for antipattern prevention

### Changed

- Enhanced Elixir subagent with antipattern detection capabilities
- Updated systematic review template to include antipattern analysis
- Elixir Doctor now automatically checks for antipatterns during code reviews

### Technical Improvements

- Better code quality guidance through antipattern detection
- More comprehensive code review process
- Proactive detection of common Elixir mistakes

## [2.3.1] - 2025-08-29

### Added

- Worker-bee agent for Worker-Bee Driven Design (WDD) in Elixir applications
- Resources directory structure for agents with templates and Mix tasks
- Worker-bee agent includes comprehensive WDD validation and scaffolding tools

### Changed

- Enhanced agent system to support resource directories
- Improved subagent installation and management

## [2.3.0] - 2025-08-20

### Added

- Plugin architecture for Intent
- Claude subagents system (renamed from agents)
- AGENTS.md universal AI agent instructions
- Support for multiple AI platforms through AGENTS.md
- New `intent agents` commands for AGENTS.md management
- New `intent claude subagents` commands (replacing old `intent agents`)

### Changed

- Renamed `intent agents` commands to `intent claude subagents`
- Moved subagents to `intent/plugins/claude/subagents/`
- Updated project structure to support plugins

### Technical Improvements

- More flexible agent system architecture
- Better separation of concerns with plugin system
- Universal agent instructions format

## [2.2.1] - 2025-08-11

### Added

- Centralized version management through VERSION file
- `get_intent_version()` function in intent_helpers for consistent version retrieval
- Comprehensive tool dependency checking in `intent doctor`
- Platform-specific installation instructions for all required tools
- Better error handling for missing jq dependency across all commands

### Changed

- Steel threads now start with 'WIP' status instead of 'In Progress' when using `intent st start`
- Tool dependencies categorized as required, core, and optional in doctor command
- Enhanced jq error messages with clear installation instructions
- All scripts now read version from centralized VERSION file

### Fixed

- `intent upgrade` now preserves existing CLAUDE.md files instead of overwriting them
- Silent failures when jq is missing during agent operations
- Missing error messages for required tool dependencies
- Inadequate installation guidance for different platforms
- Version number inconsistencies across different scripts

### Technical Improvements

- Single source of truth for version management
- Reduced maintenance overhead for version updates
- Improved fallback behavior when tools are missing
- Better user experience with actionable error messages

## [2.2.0] - 2025-08-05

### Added

- `intent fileindex` command for systematic file tracking and progress management
- Check functionality (`-C` flag) to explicitly mark files as checked [x] in the index
- Uncheck functionality (`-U` flag) to explicitly mark files as unchecked [ ] in the index
- Toggle functionality (`-X` flag) to switch files between checked/unchecked states
- Flexible operation modes - works both within Intent projects and standalone
- Enhanced Elixir agent with systematic code review workflow using fileindex
- Support for both Elixir module names and filesystem paths in the Elixir agent
- Comprehensive test suite for fileindex command (47 tests including check/uncheck)
- Demo mode (`--demo`) to showcase fileindex functionality

### Changed

- Updated all version references from 2.1.0 to 2.2.0
- Enhanced `intent upgrade` to support 2.1.0 → 2.2.0 migrations
- Improved upgrade path handling for incremental version upgrades
- Updated Elixir agent documentation with systematic review workflow
- Added fileindex to global commands list

### Fixed

- Bash compatibility issues on macOS (associative arrays, readarray command)
- Local variable declarations at global scope in shell scripts
- Missing `assert_output` function in test framework
- Test expectations for error messages

### Technical Improvements

- Replaced bash associative arrays with parallel arrays for macOS compatibility
- Replaced `readarray` with portable while loops
- Added proper error handling for edge cases in file operations
- Enhanced test helper with assert_output function

## [2.1.0] - 2025-07-27

### Added

- `intent agents init` command to initialize agent configuration
- Support for upgrading from Intent v2.0.0 to v2.1.0
- Enhanced agent manifest management with proper initialization
- Improved agent setup workflow with explicit initialization step

### Changed

- Updated all version references from 2.0.0 to 2.1.0
- Enhanced `intent upgrade` to support 2.0.0 → 2.1.0 migrations
- Improved agent installation workflow to require initialization first
- Updated documentation to reflect v2.1.0 features

### Fixed

- Agent directories not being properly created during upgrade
- Missing agent initialization when upgrading from older versions
- Agent manifest not being created in fresh installations
- Incorrect creation of `agents/` directory at project root instead of `intent/agents/`
- Upgrade process incorrectly preserving root-level agent directories

## [2.0.0] - 2025-07-17

### Added

- New `intent` command as the primary CLI (replacing `stp`)
- `intent bootstrap` command for easy global setup
- `intent doctor` command for comprehensive diagnostics
- `intent st repair` command to fix malformed steel thread metadata
- JSON-based configuration system (local and global)
- Full backwards compatibility with STP v1.x projects
- Comprehensive test suite with GitHub Actions CI/CD
- Example projects demonstrating migration paths
- Support for `jq` dependency in workflows
- **Claude Code Sub-Agent Integration**: Complete agent management system
  - `intent agents` command suite (list, install, sync, uninstall, show, status)
  - Intent agent with steel thread methodology knowledge
  - Elixir agent with Usage Rules and Ash/Phoenix patterns
  - Global and project-specific agent support
  - Manifest-based tracking with checksum integrity
  - Seamless integration with intent init, doctor, and upgrade commands

### Changed

- **BREAKING**: Renamed from STP to Intent
- **BREAKING**: Flattened directory structure (intent/ instead of stp/prj/)
- **BREAKING**: Executables moved to top-level bin/ directory
- **BREAKING**: Configuration format changed from YAML to JSON
- Improved error messages and user feedback
- Enhanced migration tools with fail-forward approach
- Streamlined command structure and naming
- Updated all documentation to reflect Intent branding

### Fixed

- GitHub Actions workflow issues with bats libraries
- Symlink issues with stp compatibility command
- Test suite reliability and coverage
- Configuration loading hierarchy
- Path resolution in various environments
- Malformed YAML frontmatter in steel threads after migration
- Legacy field names (stp_version) in steel thread metadata
- Conflicting status values between frontmatter and body content

### Deprecated

- `stp` command (now aliases to `intent` for compatibility)
- Old directory structure (stp/prj/st/ → intent/st/)
- YAML configuration format
- Nested project directory structure

### Migration Guide

#### From STP v1.x to Intent v2.0.0

1. **Automatic Migration**: Run `intent upgrade` to automatically migrate your project
2. **Manual Installation**:

   ```bash
   # Clone Intent repository
   git clone https://github.com/matthewsinclair/intent.git
   cd intent

   # Add to PATH
   export PATH="$PATH:$(pwd)/bin"

   # Bootstrap global configuration
   intent bootstrap
   ```

3. **Project Structure Changes**:
   - `stp/prj/st/` → `intent/st/`
   - `stp/prj/wip.md` → `intent/wip.md`
   - `stp/eng/` → `intent/eng/`
   - `stp/usr/` → `intent/usr/`

4. **Command Changes**:
   - All `stp` commands now use `intent`
   - Same subcommands and options supported
   - `stp` symlink provided for compatibility

See [Release Notes](./docs/releases/2.0.0/RELEASE_NOTES.md) for complete details.

## [1.2.1] - 2025-07-09

### Added

- Directory-based structure for steel threads (replacing single files)
- New steel thread file types: `info.md`, `design.md`, `impl.md`, `tasks.md`
- Migration script `migrate_st_to_dirs` for upgrading from v1.2.0 to v1.2.1
- Support for editing/viewing specific steel thread files with `stp st show/edit <id> <file>`
- `stp st show <id> all` command to view all steel thread files at once
- Automatic file creation when editing non-existent steel thread files
- Version tracking in `stp/.config/version` file

### Changed

- **BREAKING**: Steel threads are now directories containing multiple files instead of single `.md` files
- Updated `stp_st` script to handle both legacy (file) and new (directory) structures
- Enhanced `stp st new` to create directory structure with all template files
- Modified `stp st done` to move entire directories when completing steel threads
- Updated `stp st list` to read from `info.md` files in directories
- Enhanced `stp st organize` to handle directory-based steel threads
- Improved `stp upgrade` to automatically detect and migrate steel threads to directory structure
- Updated all documentation to reflect new steel thread structure

### Fixed

- Version detection in `stp_st` now properly checks for directory vs file structure
- Steel thread organization now correctly moves directories instead of files

### Migration Guide

#### Upgrading from v1.2.0 to v1.2.1

1. Run `stp upgrade` - it will detect old-format steel threads and offer to migrate them
2. The migration will:
   - Create a backup in `.backup/1.2.1/`
   - Create directories for each steel thread (eg `ST0001/`)
   - Split content into separate files based on sections
   - Preserve all existing content and metadata
3. After migration, use `stp st organize --write` to organize by status if desired

#### New Steel Thread Commands

- `stp st show ST0001 design` - Show only the design.md file
- `stp st edit ST0001 impl` - Edit the implementation file
- `stp st show ST0001 all` - View all files for a steel thread

## [1.2.0] - 2025-07-09

### Added

- New `stp llm usage_rules` command for displaying STP usage patterns to LLMs
- `--symlink` option for `stp llm usage_rules` to create usage-rules.md symlinks in projects
- Comprehensive test suite for the llm command (`stp/tests/llm/llm_test.bats`)
- DEPRECATIONS.md file to track deprecated features
- Help documentation for llm command (`stp/bin/.help/llm.help.md`)
- Archive directory structure for deprecated content (`stp/prj/archive/`)

### Changed

- Renamed `usage_rules.md` to `usage-rules.md` to follow Elixir Hex package conventions
- Updated `stp_upgrade` to handle file renaming during upgrades
- Updated all documentation to reference Backlog for historical tracking instead of journal.md
- Simplified `stp_init` to only create `wip.md` and `steel_threads.md` in the prj directory

### Fixed

- Fixed `stp upgrade` version mismatch (was using 1.0.0 instead of 1.2.0)
- Made file organization in `stp upgrade` optional with new `--organize` flag to prevent unexpected file moves

### Deprecated

- `journal.md` file - users should migrate to Backlog task tracking for historical project narrative

### Removed

- `journal.md` creation from `stp_init` script
- Journal template from `stp/_templ/prj/_journal.md`
- All references to `journal.md` from documentation (18 files updated across user guides, reference guides, blog posts, and templates)

### Migration Guide

#### For users with existing journal.md files

1. Your existing `journal.md` has been automatically moved to `stp/prj/archive/journal-deprecated.md`
2. Use `stp bl list` to view task history moving forward
3. Track detailed progress in Backlog task descriptions
4. Use steel thread documents for high-level context and decisions

#### For LLM integration

1. Use `stp llm usage_rules` to display usage patterns
2. Create symlinks with `stp llm usage_rules --symlink` for projects expecting usage-rules.md
3. Reference the usage rules documentation at `intent/llm/usage-rules.md`

## [1.0.0] - 2025-06-03

### Added

- Initial release of Steel Thread Process (STP)
- Core script framework for managing steel threads
- Template system for project documentation
- Integration with Backlog.md for task management
- Comprehensive test suite using BATS
- User and reference documentation
- Blog series explaining STP concepts and methodology

### Features

- `stp init` - Initialize STP in a project
- `stp st` - Manage steel threads (new, list, show, edit, done, sync)
- `stp bl` - Backlog.md wrapper for task management
- `stp task` - Create and list tasks linked to steel threads
- `stp status` - Synchronize steel thread status with task completion
- `stp migrate` - Migrate embedded tasks to Backlog
- `stp upgrade` - Upgrade STP files to latest format
- `stp help` - Comprehensive help system

[2.6.0]: https://github.com/matthewsinclair/intent/compare/v2.5.0...v2.6.0
[2.5.0]: https://github.com/matthewsinclair/intent/compare/v2.4.0...v2.5.0
[2.4.0]: https://github.com/matthewsinclair/intent/compare/v2.3.4...v2.4.0
[2.3.4]: https://github.com/matthewsinclair/intent/compare/v2.3.3...v2.3.4
[2.3.3]: https://github.com/matthewsinclair/intent/compare/v2.3.2...v2.3.3
[2.3.2]: https://github.com/matthewsinclair/intent/compare/v2.3.1...v2.3.2
[2.3.1]: https://github.com/matthewsinclair/intent/compare/v2.3.0...v2.3.1
[2.3.0]: https://github.com/matthewsinclair/intent/compare/v2.2.1...v2.3.0
[2.2.1]: https://github.com/matthewsinclair/intent/compare/v2.2.0...v2.2.1
[2.2.0]: https://github.com/matthewsinclair/intent/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/matthewsinclair/intent/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/matthewsinclair/intent/compare/v1.2.1...v2.0.0
[1.2.1]: https://github.com/matthewsinclair/intent/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/matthewsinclair/intent/compare/v1.0.0...v1.2.0
[1.0.0]: https://github.com/matthewsinclair/intent/releases/tag/v1.0.0
