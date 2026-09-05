# Known defects in v3.0.0

**Every defect on this page has been run against v3.0.0 itself.** Not against `main`, not inferred from our issue register: driven against the published binary, which reports `intent 3.0.0 (80d8b2ca)`. Where a claim could not be driven it is not on the page, and the last section says what that leaves out.

That distinction is not pedantry. The first draft of this page was written from our issue titles and it was wrong in both directions: it described defects that arrived after the tag and are not in this build, and it described one that is real with the wrong symptom. Driving it against the shipped binary was the only thing that found either.

**A defect is on this page if you can hit it by following the documentation correctly.** Something that only bites a maintainer editing the register, or a team sharing one checkout, is recorded against the issue rather than here.

Two things worth knowing before the list. **An issue being closed in our register does not mean the fix is in your build** — the register tracks `main`, and several fixes landed after the tag. And **an issue being open does not always mean it is still broken.**

## A stray directory disables the whole project

**Any `STnnnn` directory anywhere under `intent/st/` is picked up as a thread, and one it cannot read stops every command** (`intent#0011`). A staging or scratch copy at `intent/st/staging/ST0099/` is enough. What you get is not a duplicate row in a listing, it is:

```
  error: this project has not been migrated to Intent v3 -- it declares Intent 3.0.0,
         and 1 steel thread carries v2 canon this binary cannot read (ST0099)
  remedy: run `intent upgrade` to migrate this project to Intent v3
```

on every verb, including ones that have nothing to do with the stray thread. The remedy is misleading: the project is fine and one directory is not. Keep working copies of threads outside `intent/st/`.

**A second route reaches the same wall.** A thread placed under an `_inbox/` status directory produces the identical stop (`intent#0066`), rather than the invisibility that issue records. Two different stray-directory shapes, one symptom, and in both cases the message names a thread the operator never created.

## Starting a project

**`intent init` writes an Elixir/Phoenix decision tree into every project, whatever language you work in** (`intent#0224`). Driven on v3.0.0 in an empty directory: `intent init` leaves `intent/llm/DECISION_TREE.md` at 3 KB while the project's declared languages are `[]`. The file routes code placement through Phoenix contexts, LiveViews and Ash resources, so a Rust or Swift project gets guidance for a stack it does not use. Nothing declines it at init time and no flag suppresses it. Delete the file if it is not yours; nothing regenerates it unless you re-run `init`.

**The `AGENTS.md` a shell project is given documents a test command that finds no tests** (`intent#0220`). Driven on v3.0.0: with `shell` declared, `intent agents sync` writes an `AGENTS.md` whose line 29 reads `bats tests/`. `bats` is not recursive, so a project whose suites live under `tests/unit/` gets `ERROR: Found no tests. (Try \`--allow-empty-suite\`?)`from the documented command, where`bats -r tests/`runs them.`AGENTS.md`is the first file the project tells an agent to read, and it is generated, so a correction written into it is overwritten at the next`intent agents sync`. No per-project override exists for a template value. Run `bats -r tests/`; the flag is correct for a flat layout too.

Declaring the language is its own obstacle on this build: `intent init --lang shell` refuses (`intent#0187`), so the array has to be edited into `intent/.config/config.json` by hand before the generated file says anything about shell at all.

## Threads

**`intent st list` shows only in-progress threads and discloses the filter nowhere** (`intent#0121`). Driven: two threads, one `WIP` and one `Triage`; the default listing shows one row and `--status all` shows both, with nothing in the default output indicating that anything was filtered. A short list reads as a short project.

**`intent st hydrate` reports `exists` for a file it has just created** (`intent#0083`). Driven by deleting `info.md` and hydrating: the file is restored and reported with the same `exists:` prefix as the one that was already there. The output cannot distinguish a restore from a no-op, so you cannot tell what it did.

**A migrated v2 project loses some thread slugs** (`intent#0080`). Driven against a real 2.19.0 estate captured from this repository's history and migrated by v3.0.0: 21 of 56 threads come through with no slug. The register records this as affecting every migrated thread and blanking the whole column; it does not. The `Slug` column renders normally and most threads carry one, so the ones that do not are easy to miss.

**A mistyped subcommand becomes a thread or an issue, silently** (`intent#0223`). Driven on v3.0.0: `intent st new help` returns `created: ST0001` and `intent issues add help` returns `created: intent/.canon/issues/0001.json`, both at exit 0. The word after the verb is a title positional, so any bare subcommand or flag name typed there -- `help`, `start`, `severity` -- is accepted as the title and written to your project. The two arms that should catch it both work: `--help` prints help, and omitting the argument entirely refuses at exit 1. It is the bare word that lands.

The row cannot be repaired afterwards: an issue title and body are write-once (`intent#0151`, `intent#0090`), so a thread or issue created this way can never be renamed. This project's own register carries several, issues and threads together, all titled after intent subcommands.

**The write-once title is the part that outlasts the mistake, and one of ours proves it.** A thread created this way as `dehydrate` was later adopted and filled in -- it carries a real objective and real context and is genuinely in progress -- and **its title is still the bare subcommand name, because nothing in the tool can change it.** Closing the debris is the answer only while it is debris; once something is adopted, the junk title is permanent. If you hit this, decide early whether the row is worth keeping, because renaming will not be available later.

**`intent wp show` prints a short header, not the work package's `info.md`** (`intent#0245`). Driven on v3.0.0: `intent wp show ST0001/01` returns three lines -- the id and title, `status:`, and `scope:` -- while `intent/st/ST0001/WP/01/info.md` exists on disk and is not shown. The dispatch table describes the verb as showing the work package's `info.md`, so the description and the behaviour disagree. Read the file directly if you want its body. **Note on the register row**: `intent#0245` says four lines; it is three at v3.0.0.

**`intent st attach` accepts a repo-relative path and mints a second attachment row for a file that already has one** (`intent#0262`). Driven on v3.0.0: `intent st attach ST0001 parity/probe.txt --from <file>` records `parity/probe.txt`, and running it again with `intent/st/ST0001/parity/probe.txt` -- the same file, addressed from the repository root -- returns `ok:` at exit 0 and leaves canon holding both paths. The verb wants the path relative to the THREAD and says so nowhere. `intent doctor` does not report the duplicate, and there is no `detach`: `intent st` ships no verb that removes an attachment row, so repair means editing `intent/.canon/st/<ID>.json` by hand.

The form that breaks it is the form the tooling invites. A repo-relative path is what `git status` prints at you, and it is what a reader has in hand at the moment something asks for the file.

## Criteria and tests

**`intent doctor` reports criteria you deliberately withdrew, and `intent ac gate` does not** (`intent#0256`). Driven on v3.0.0 with both controls: a criterion `AC-02.1` on a thread with no `WP-02` is reported as `model-inconsistent -- AC-02.1 belongs to WP-02, which ST0001 does not have`, and **the finding is byte-identical before and after `intent ac withdraw` runs on it**. The same thread's `ac gate` changes its answer correctly, to `all in-scope AC(s) are descoped or withdrawn`. So the two verbs ship opposite verdicts on one thread, and the one that ignores the withdrawal is the one that prints a finding count.

**This scales badly on a real estate, because the count is what people read.** Withdrawing a group of criteria -- the normal way to record that a work package will never be built -- adds one `doctor` finding each, permanently, and they crowd out the findings that need attention. Read `intent ac gate <thread>` for whether a thread is actually in order; treat `doctor`'s model-inconsistency findings as needing a check against each criterion's withdrawn state before you act on any of them.

**A work package whose criteria are all descoped cannot be closed, and the refusal's remedy cannot be followed** (`intent#0063`). Driven on v3.0.0: give a work package one criterion, descope it, and `intent wp done` refuses at exit 1 with `all 1 in-scope AC(s) are descoped or withdrawn; nothing is left to verify`. The remedy printed underneath reads `satisfy or formally descope the remaining criteria, then close again` — but there are no remaining criteria, which is the whole reason it refused. Following it exactly leaves you where you started.

The refusal also names an escape, `declare 'acceptance: exempt'`, and **there is no way to declare it on a work package**: the WP cover carries `wp_id`, `title`, `scope` and `status`, and nothing else. The exemption exists at thread scope only. A work package emptied one descope at a time has no route to `Done`. Leave one criterion in scope and satisfy it, or leave the package open.

**And there is no way to declare `acceptance: exempt` at thread scope either, though the gate tells you to** (`intent#0227`). Driven on v3.0.0 in a fresh project: `intent ac gate ST0001` on a thread with no criteria exits 1 with `BLOCKED -- the thread has zero acceptance criteria (empty contract). Define ACs, or declare 'acceptance: exempt'.` No verb writes that state: `intent ac --help`, `intent at --help` and `intent st --help` mention `exempt` nowhere between them. The state has a complete read path and no writer, so the second half of the remedy cannot be followed at any scope. Define a criterion and satisfy it; the exemption is not reachable from the command line.

**Note on the register row**: `intent#0063`'s title also asserts that `WpStatus` has no `Cancelled` variant. That part is false — it carries one at v3.0.0 and at `HEAD` — but the behaviour above is real and was driven separately.

**`intent ac new` on an id that already exists replaces the row instead of refusing** (`intent#0119`). Driven: creating `AC-01.1` twice returns `ok: AC-01.1 created` both times. The replacement is a full write, so a field you do not supply is written empty rather than preserved, and there is no verb that edits a criterion in place. Treat `ac new` as create-only and read `ac list` before re-running it on an id you are unsure of. **v3.0.1 closes this**; on v3.0.0 the verb that repairs is the verb that destroys.

**`intent ac list` never prints the criterion text** (`intent#0168`). Driven: a criterion whose text is a distinctive sentence does not have that sentence anywhere in the listing. It prints ids, coverage and satisfaction, so planning from its output means planning from ids. There is no `ac show`.

**A criterion cannot record what would discharge it until it is discharged** (`intent#0211`). `intent ac satisfy` is the only verb that takes evidence and `--evidence <ref>` is required on it, so there is no way to write down what a criterion is waiting for while it is still open. Driven on v3.0.0: `ac satisfy --help` reads `Usage: intent ac satisfy --evidence <ref> <STID> <ACID>`. Planning notes for an open criterion have to live outside the tool.

**`at lint` reports a test row as conforming when the row cites no test at all** (`intent#0213`). The register carries a second row for the same code site (`intent#0229`), filed from an independent report and closed as the duplicate it is; both describe this behaviour. Driven on v3.0.0: create an AT with `--kind test` and no `--file`, drive it to green, and `intent at lint` answers `ok -- 1 AT row(s) conform`. The row asserts a passing test and names nothing that could have passed.

**The refusal that does exist makes this worse, not better.** `at new --file tests/does_not_exist.rs` is refused outright at exit 1 -- `cites a file that does not exist` -- so the tool checks the path when you give one and checks nothing when you do not. **Citing a wrong file is caught; citing no file is blessed.**

**The close gate has the identical blindness, and that is the half that costs you something.** `intent ac gate` is not a safer alternative to the linter: driven on v3.0.0, a test-backed criterion covered by a single fileless test row driven green returns `gate: ST0001 PASS -- 1/1 satisfied` at exit 0, and says nothing about the row it did not examine. The same `contract_report` backs both, by design -- its own source says so, on the ground that two rule sets would drift. **So a thread whose test rows all lack files can pass its own close gate over evidence nothing read.** Treat a `conform` or `satisfied` count that matches your row count as unverified until you have checked the rows cite files.

**`intent at lint --fix` is advertised and refuses** (`intent#0139`). `at lint --help` documents it as _Migrate the mechanical part of a legacy row_; calling it exits non-zero without doing so.

**The close gate reads your working tree, so a thread can pass on evidence nobody else has** (`intent#0265`). Driven on v3.0.0: a test-backed criterion covered by an AT citing `tests/probe.rs`, with that file present on disk and **not committed**, returns `gate: ST0001 PASS -- 1/1 satisfied` at exit 0 -- and `intent st done` then closes the thread on it. A clone of the same repository does not contain the cited file at all.

Two controls make it sharp. Remove the file from the worktree and the gate flips to `BLOCKED -- AT-01.1 cites a file that does not exist`, so the gate is genuinely reading the tree and the PASS was not indifference. And **the verdict names no tree**, so two people running the identical command in the same repository can get different answers with nothing in either output to explain the difference. Commit the cited files before reading a gate result as a claim about the project; on a shared checkout a PASS is a statement about one person's disk.

**A test file satisfies a citation whenever it contains the id anywhere, including under another thread's name** (`intent#0267`). AT ids are only locally unique -- `AT-01.1` exists in as many threads as you have -- and the check takes a path and a bare id. Driven on v3.0.0: a file whose sole mention is the comment `// ST0002 AT-01.1 -- this test witnesses thread B and nothing else`, cited from **ST0001**, gives `lint: ST0001 ok -- 1 AT row(s) conform` and `gate: ST0001 PASS -- 1/1 satisfied`. Strip the id and the lint goes red naming the row, so the check is running; it is matching a literal that does not belong to the thread asking. Qualify the id with its thread in the test's own text and the collision at least becomes visible to a reader, though not to the tool.

`intent at new` does not check the cited file's contents on this build at all -- a file carrying no id was accepted at exit 0 -- so the first thing that looks at a citation is `at lint`.

**And the citation check stops at close, with nothing saying so** (`intent#0267` again). Driven: close a thread on an honest citation, then remove the id from the cited file. `at lint` answers `ok -- 1 AT row(s) conform`, `at list` still renders the row `green`, `ac gate` still answers `PASS`, and `doctor` does not mention it. The exemption is deliberate -- retrofitting id labels into a finished thread is archaeology -- and the defect is that nothing distinguishes _checked and true_ from _true at close, unchecked since_. **The file-existence arm is not exempt**: delete the cited file and the same closed thread reports `cites a file that does not exist`. So a closed thread's coverage is checked for presence and not for content, and reads identically either way.

## Editing

**An address is answered even when it names something that does not exist** (`intent#0238`). Driven on v3.0.0: `intent edit intent:///threads/ST0001/attachments/nope.md` -- an attachment that was never created -- refuses with `intent/st/ST0001/info.md is generated from the model`. **The trailing segment is dropped rather than checked**, so the answer is about the thread, an entity you did not name, and the error you read discusses a file you did not ask about. Nothing tells you the attachment is absent.

The same grammar refuses criteria and tests outright: `intent:///threads/ST0001/acs/AC-01.1` comes back as `has trailing segments after a complete address` **for a criterion `intent ac list` shows as existing**, with the remedy `an address ends at the entity it names`. It is the same message for a real id and an invented one, so the refusal is about the shape of the address and not about what is in the project. **Address threads and issues; for anything below them, use the family verbs (`ac list`, `at list`, `st show`) rather than an address.**

**Two Intent commands writing to the same thread at once lose one of the writes, and both report success** (`intent#0206`). Driven on v3.0.0: three `intent ac new` calls launched concurrently against one thread each print `ok: AC-01.n created` at exit 0, and afterwards the thread holds two of the three. Run the same three sequentially and all three survive, so this is contention and not a broken verb. The canon left behind is valid, so nothing downstream reports a problem either.

This bites hardest where it is least visible: a script, a `Makefile` target, or two terminals working the same thread. Serialise anything that writes canon, and if a row you created is missing, re-run the command rather than assuming you mistyped it.

**No verb writes a thread's title, objective, context, body or preamble** (`intent#0185`). Driven: `intent st edit ST0001 info` refuses with `is generated from the model, so an edit here is lost at the next render`, and its remedy says to author it with `intent st` — which has no verb that writes those fields. Edit `intent/.canon/st/<ID>.json`, then `intent sync --to-store`, then `intent sync --to-disk`.

**`intent issues add` creates an issue whose body no verb can write** (`intent#0090`). Driven: the created issue has a body of length zero and nothing can fill it. Issue titles and work package bodies have the same write-once door (`intent#0154`).

**`intent edit` sends you to a route that gives the same refusal** (`intent#0153`). Driven: `intent edit ST0001` refuses with `is generated from the model, so an edit here is lost at the next render` and a remedy saying to author it with `intent st`. Following that remedy -- `intent st edit ST0001 info` -- produces the identical refusal. The remedy names the door you just came through.

**Addressing an issue with `intent edit` refuses with a remedy that offers the thing it just refused, and glues a bare `a` onto a vowel-initial noun** (`intent#0081`). Driven on v3.0.0, using the address form its own `--help` prescribes (`Usage: intent edit <ADDRESS> [FILE]`):

```
  $ intent edit intent:///issues/0001
  error: `issue` is not something that can be realised to disk ...
    remedy: address an ARTEFACT instead -- a thread or an issue. A `issue` has no files
    of its own, so there is nothing for realisation to create ...
```

Two things are wrong in one message. **The remedy says `a thread or an issue`** and an issue is exactly what was refused, so following it returns you to the refusal — that half is corrected after `v3.0.0`, which now says `a steel thread`. **`A \`issue\`` is the article bug** and it is not corrected: seven of the fourteen entity names are vowel-initial, and this site builds the article by hand rather than asking the noun for it. The refusal is still telling you the right thing; only its grammar and its remedy are wrong.

## Syncing

**`intent sync --to-store` reports two contradictory things in one breath** (`intent#0069`, `intent#0111`). Driven on a thread-scoped sync:

```
  note: the store and the extract agree; this restore overwrites nothing
  ok: store replaced from the extract, 1 thread(s)
```

The first line says nothing changed and the second says the store was replaced. A thread-scoped call also describes itself as acting on the whole store.

**A file authored in canon alone never reaches disk, and the sync says it worked** (`intent#0082`). Driven: an attachment added to `intent/.canon/st/ST0001.json` and then synced produces `ok: extract written for 1 thread(s)` and no file. The count of files under `intent/st/` does not move.

This bounds the canon-editing route that [Getting started](getting-started.md) uses for thread fields: editing canon and syncing works for a thread's `objective` and `context`, and does **not** work for adding a file. `intent st attach` is the writer for attachments, and no direction of `sync` is.

**Text appended to a generated view AFTER its `_Generated by Intent v...` banner is discarded, and `sync --to-store` reports success** (`intent#0192`). Driven:

```
  $ printf '\n## Hand Added\n\nTEXT\n' >> intent/st/ST0001/info.md
  $ intent sync --to-store
  ok: store rewritten from the canon extract, 2 thread(s)
  $ intent sync --to-disk && grep -c TEXT intent/st/ST0001/info.md
  0
```

The same section inserted **before** the banner is refused, by name, with the text left intact — so this is one hole in a working guard rather than a missing one. `intent doctor` does report the drift as `view-skew`, so the loss is detectable after the fact; what does not report it is the verb you ran to make the edit land. **Append above the banner, or put the text in `## Objective` or `## Context`, which are the two sections that round-trip.**

**`intent sync --to-store` reverts a committed correction on disk, reports success, and leaves the wrong value in the store** (`intent#0260`). Driven on v3.0.0 in a fresh project: correct a thread's objective in `intent/st/ST0001/info.md`, commit it, then run the verb whose documented job is reading the extract into the store.

```
  note: the store and the extract agree; this restore overwrites nothing
  ok: store replaced from the extract, 1 thread(s)
```

Exit 0, and **both lines are false**. The store and the extract did not agree, and the store was not replaced. Afterwards the correction is gone from the working file, the pre-edit text is back in its place, and canon still holds the old value. The safety-sounding first line is the reason the operation did nothing, printed as though it were a guarantee, and the write to your file is not mentioned at all.

**Commit before you sync** -- that is the only thing that makes the loss recoverable -- and change a thread's fields in `intent/.canon/st/<ID>.json` rather than in the extract (`intent#0185`).

## Searching

**A hyphen in a search query is read as SQL and leaks the error** (`intent#0194`). Driven on both builds: `intent search canon-ignore` exits 1 with `sqlite: no such column: ignore`, while `intent search canon` returns hits normally. The query goes to FTS5 unescaped, so the hyphen is parsed as an operator and the term after it as a column name. Any query containing `-` fails the same way, which includes most of this project's own vocabulary -- `read-back`, `at-lint`, `to-write`. Quote nothing and search a single word; there is no escaping syntax that helps, because the escaping is missing on the tool's side of the call.

**The remedy printed with that refusal names the wrong characters** (`intent#0247`). Driven on v3.0.0: `intent search no-backup` exits 1 and the remedy reads `search takes an FTS5 expression -- quote a phrase, and escape or drop bare punctuation like ':' and '*'`. Neither character it names is the one that failed, and the one that failed is not named. A reader who follows it exactly -- removing colons and asterisks from a query containing neither -- changes nothing and gets the identical error.

**A search hit names the file but not the place in it** (`intent#0195`). Driven on v3.0.0: a file containing a term on lines 1 and 3 comes back as a single row ending `:0`, and so does the register's own `design.md` for a phrase it carries at line 91. **The column that looks like a line number is 0 for every prose hit**, so a result set tells you which files matched and gives you nothing to navigate to -- on a long document that is the difference between an answer and a place to start reading. Structured canon hits do carry a non-zero value in that column, but it is a section ordinal rather than a line, so the two kinds of row are not comparable even though they are printed identically.

The same issue also reports one file repeating once per hit -- four identical rows for a phrase occurring once. **That half did not reproduce on v3.0.0 and is deliberately not stated here as either present or fixed.** In a scratch project the issue's own file and query return exactly one row, but a scratch project holds that file once where this repository holds it on disk, in canon, and across views; that is a difference in state rather than in build, and it was not isolated. What is certain is the `:0`.

**What search gets right, so this is not read as worse than it is:** an unindexed project says so rather than returning an empty list, in the tool's own words -- `nothing is indexed, so this search could not have matched -- an empty result here does NOT mean <term> is absent`. That is the failure mode that would actually mislead a reader, and it is closed.

## The store grows with every write

**Every mutation appends a full copy of the thread's prose to the search index, and the index it replaces is never truncated** (`intent#0234`). Driven on v3.0.0 in a fresh project holding one thread with two sections of prose: 30 `intent ac new` calls take `doc_sections` from 2 rows to **62**, against **2** distinct `(file, owner_id, seq)` throughout -- two duplicate rows per mutation, of content that never changed. Over the same 30 calls `doc_sections_data` goes from 2,710 bytes to 74,776, and the database file from 114 KB to 250 KB.

The growth tracks how often a project has been written to rather than what it holds, and nothing reports it: the row count is right for the model, searches keep answering correctly, and `VACUUM` reclaims none of the index because the tombstones are live data rather than free pages. What you notice is start-up latency on a heavily-edited project. **Both mechanisms are fixed after the tag**, and on a fixed binary a single `intent sync --to-store` repairs an existing store, so the route out is a newer build rather than a maintenance verb.

## The daemon

**`intentd --help` starts a daemon instead of printing help** (`intent#0162`). On v3.0.0 the binary inspects argv for `--version` and then serves regardless of what else is there, so any argument it does not recognise -- `--help` and `-h` included -- falls through to starting a real daemon under your real `$HOME`. It binds, it publishes, and it does not return. While it is up, every other Intent session on the machine has its store verbs refused at `rc=2` by a daemon nobody meant to start.

Do not type it. If you already have, find the process and stop it: `pgrep -fl intentd`, then `intent daemon stop` or kill the pid. **v3.0.1 closes this** -- the fixed binary prints usage for `--help` and refuses any other argument with a remedy, on the stated ground that starting a daemon by accident takes every session on the machine down together.

**How this entry was established, because the obvious check is the defect.** Running `intentd --help` on the published build to confirm the behaviour would reproduce the outage on the machine doing the checking. So the two binaries were compared statically instead: the v3.0.1 help text is absent from the v3.0.0 binary and present in the current one, with a control string both carry, so an unreadable binary cannot masquerade as an unfixed one. The behaviour itself was driven first-hand on 2026-08-30, once, before it was understood -- which is how it was found.

## The pre-commit gate

**`intent claude upgrade --apply` installs a gate that enforces nothing, and reports that it wrote it** (`intent#0266`). Driven on v3.0.0 from a faithful install tree: the verb prints `written: .git/hooks/pre-commit`, and the hook it writes is a chain block that execs `.git/hooks/pre-commit.intent` **if that file is executable**. Nothing in v3.0.0 ever writes that file -- six candidate verbs were driven and none creates it, and the build ships no template for it. The test is false forever, so every commit passes ungated and in silence.

Three controls separate this from a gate that is merely quiet. The listing sees the file the moment one is planted by hand, so the six negatives are real. Copying the shipped gate body to that path makes the very next commit print `guards: 1 ran, 3 skipped (not applicable)`, so the gate speaks as soon as anything runs it. And `intent doctor` on the same project emits three findings while mentioning the gate, the hook, or `pre-commit` **zero** times -- the absence of a gate finding is the check declining to look, not a healthy project. The function that installs the carrier arrived after the tag. **If you want the gate on this build, put the gate body at `.git/hooks/pre-commit.intent` yourself and make it executable.**

**Behind that sits a second defect you cannot reach until you do** (`intent#0242`). The gate reports which declared languages went unenforced only when that count is non-zero, and `intent init` leaves `languages` at `[]`, so a gate with nothing to enforce is silent and looks identical to one enforcing everything. Driven with the carrier installed by hand: the commit prints its guard line and says nothing about critics or languages at all.

## The rule critics

**The shell critic claims any file in `bin/`, whatever language it is** (`intent#0228`). Five canon shell rules -- two of them `critical` -- carry `applies_to: ["**/*.sh", "**/*.bash", "bin/*"]`, and the third pattern is not extension-constrained. Driven on v3.0.0: a Lua script at `bin/luatool` whose second line is the comment `-- shells out for the legacy path: cat $1 | grep x` returns `[CRITICAL] IN-SH-CODE-001 at bin/luatool:2`. The identical bytes at `src/luatool.lua` return no findings, so the discriminator is the path and not the content.

**And `bin/*` overrides an extension rather than merely lacking one.** The same bytes at `bin/luatool.lua` -- a file that names its own language -- are claimed too, and draw the same critical. The pattern stands in for _an extensionless shell script in `bin/`_, which is a real convention it cannot see directly. Keep non-shell files out of `bin/`, or expect critical findings against a language the rules do not describe.

## Declared and not implemented

Each of these is listed in `--help` and refuses when called. Driven against v3.0.0, exit codes as shown.

**`intent daemon` and its subcommands** (`intent#0163`) — `error: daemon is a known command that is not implemented yet`, exit 2.

**`intent claude rules validate`** (`intent#0156`) — the same refusal at exit 2. The command works in the v2 shell estate.

**`intent ext remove`** (`intent#0177`) — `unrecognized subcommand`. `ext` creates and has no way to undo.

**`intent agents` on its own** (`intent#0175`) — exit 2. The bare family verb is an unwired dispatcher, not a broken feature. **Two of its five verbs are wired**: `intent agents generate` and `intent agents sync` both exit 0. **`intent agents init`, `intent agents validate` and `intent agents template` are not** — each exits 2 with the same `known command that is not implemented yet` the bare verb gives. This entry previously named `validate` as one of the working verbs, which sent a reader from one unwired verb to another; corrected 2026-09-05 by driving all five against v3.0.0. `intent agents --help` lists all five without distinguishing them, so the help is not a guide to what is wired.

**`intent claude skills install --all` is named by the project's own canon and refused by the binary** (`intent#0236`). Driven on v3.0.0: `intent claude skills install --all` exits 1 with `error: unexpected argument '--all' found`, which reads as a typo. The flag is not a typo -- the root canon names it, and v2 shipped it. Install the skills you want by name; `intent claude skills --help` lists the verbs that work.

## Declared, accepted, and ignored

These flags are documented in `--help`, accepted without complaint, and read by nothing. Passing one changes no behaviour and reports no error.

**`intent agents init --template`** (`intent#0180`) and **`intent llm usage_rules --symlink`** (`intent#0181`). Both confirmed present in v3.0.0's own help output.

**`intent claude upgrade` has no `--skip-settings`** (`intent#0143`). v2 had a flag to decline the Claude Code settings file. v3.0.0's `claude upgrade --help` offers only `--apply`, `--force` and `--help`, so there is no way to ask it to leave your settings alone.

**`intent claude skills uninstall --force`** (`intent#0078`). The flag is in `--help` and the call behind it takes no force argument at all, so passing it changes nothing. Driven on v3.0.0 against a skill this build did not write:

```
  $ intent claude skills uninstall hand-made --force
    hand-made                    removed (0 file(s)); left 1 this build did not install: SKILL.md
  ok: 1 changed, 0 already settled, 0 need a decision
```

The file is still on disk afterwards, and **that is the tool being careful rather than the tool failing** — it will not delete what it has no record of writing, and the per-skill line says so, gives the count, and names the file. **What is wrong is one line lower: the summary says `1 changed` when no file changed.** Read the per-skill line, not the total.

## Recorded against v3.0.0 and NOT present in it

These are in our register and you will not hit them on this build. They are listed because finding an open issue that describes your version is otherwise alarming.

**`intent st new` writes no files** (`intent#0079`) — not true of v3.0.0, where `st new` writes `info.md` and `acceptance.md`. This arrived after the tag.

**`intent edit` ignores the kind it was given** (`intent#0149`, `intent#0189`) — v3.0.0's `edit` does not accept `--path` at all, refusing with `unexpected argument`. Post-tag.

**`intent modules find` is unimplemented** (`intent#0067`) and **`modules check` routes you to `intent upgrade`** (`intent#0122`) -- neither is true of v3.0.0. Driven with a `MODULES.md` present, `modules find` returns the row, and `modules check` routes to `intent lang init`.

**`intent init` bakes an absolute path into `.claude/settings.json`** (`intent#0016`), **`st edit`'s refusal names an empty list** (`intent#0145`), and **`at edit` is kind-blind** (`intent#0146`) -- none reproduce on v3.0.0. The settings file carries no absolute path, the refusal names `acceptance.md, info.md`, and `at edit` does not exist there at all.

**`intent fc` dispatches on kind by hand** (`intent#0171`) and **`ac list` renders a fiat-closed criterion wrongly** (`intent#0137`) — neither `intent fc` nor `intent at fc` exists in v3.0.0; both refuse with `unrecognized subcommand`. Post-tag.

## Migrating from v2

The v2 ingest has its own defect set and its own recovery routes, covered where you meet them in [Migrating from v2](migrating-from-v2.md): evidence discarded silently from criteria authored unsatisfied (`intent#0133`), and a measurement that cannot tell "nothing was lost" from "nothing was measured" (`intent#0098`). `intent st repair` is declared retired and was never built in v3.0.0 either (`intent#0118`).

**Two acceptance tests sharing an id stop the migration with a raw SQLite error that names neither** (`intent#0268`). Driven on v3.0.0 against a real 2.19.0 estate captured from this repository's history: the estate migrates cleanly as captured (`migrated: 56 thread(s), 25 issue(s), 334 file(s) written`), and duplicating a single `AT-01.1` row inside one thread's `acceptance.md` turns the same command into:

```
  error: could not update the runtime store
    caused by: sqlite: UNIQUE constraint failed: tests.thread_id, tests.id
    caused by: Error code 1555: A PRIMARY KEY constraint failed
    remedy: the change was not made. Do NOT delete the store -- it is the source of
    truth, not a cache, and the committed extract may be older than it. Run
    `intent doctor` to inspect the estate
```

**The thread and the id appear nowhere in it**, and the remedy closes a loop: `intent doctor` on an unmigrated estate can only report `this project has not been migrated ... run \`intent upgrade\`` -- the command that just failed and sent you there. It reads the model, and the model is empty because migrating is what would fill it.

The store warning is also written for a project this is not: `intent/.cache/intent.db` here is a schema-only file created seconds earlier by the failure, with zero rows and no committed extract to be newer than. **The rollback itself is sound** -- nothing lands, the tree is unchanged, and a re-run fails identically -- so the refusal is safe and it is the reporting that strands you. Find the offenders yourself before re-running:

```
  grep -rho 'AT-[0-9]*\.[0-9]*' <thread>/acceptance.md | sort | uniq -d
```

and remember that v2 keeps status in the path, so a scan over `intent/st/*/acceptance.md` misses everything under `COMPLETED/` and `CANCELLED/`.

## What this page does not cover

**This is the driven set, not the whole register.** Intent's open issues include defects that need a condition this page's checks cannot create from a fresh project — a migrated v2 estate, a bucketed thread, an installed skill, a running daemon. Those are real and they are not described here, because describing an undriven defect is how the first version of this page came to be wrong.

If you hit something not listed, that is the gap rather than a surprise. The register is the fuller record, and `intent doctor` reports on your own project.

## Reading this against your own build

`intent --version` names the build you are on. Everything here was driven against `intent 3.0.0 (80d8b2ca)`, the published tag. A source install from `main` behaves differently, and several entries above are already fixed there.

**The register itself cannot tell you which build a row describes** (`intent#0191`). An issue says what is broken and carries no field naming the version it was broken in, so `intent issues list` mixes rows about the published tag with rows about `main`. This page is the partition, drawn by hand and by driving each row: what is above is what a v3.0.0 reader can hit, and the section before last names the rows that read alarmingly and are not present. **If you find an open issue that seems to describe your version, check here before believing it.**
