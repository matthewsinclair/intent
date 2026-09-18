# Known defects in v3.1.0

**Every defect on this page has been run against the build v3.1.0 is cut from.** Not inferred from our issue register: driven against that build before its version stamp moved, when `intent --version` printed `intent 3.0.3 (493705d980f4a61aef6c8e169ee699559411d03f)`, each in a fresh scratch project under an isolated `HOME`. Where a claim could not be driven it is not on the page, and the last sections say what that leaves out.

**A defect is on this page if you can hit it by following the documentation correctly.** Something that only bites a maintainer editing the register, or a team sharing one checkout, is recorded against the issue rather than here.

**An issue being closed in our register does not mean the defect is gone from your build.** Every issue this page cites is closed in the register, and each entry below still reproduces on this build.

**The page is re-driven whole at every release, and an entry that no longer reproduces leaves it.** A page titled for one release lists what a reader of that release has. What each release fixed is in `CHANGELOG.md`, and an entry that left names the release that fixed it in the commit that removed it.

## The search index

**The store's FTS5 index for the source half can go malformed, so a search on an affected term fails** (`intent#0442`, closed on a repair and a detector with its cause unreproduced). The damage is a document the index still holds with no row in its content table. Its cause is not reproduced: eight deliberate attempts across two corpora did not produce it. What this release adds is that `intent doctor` sees it. Driven on a copy of the damaged store:

```
  $ intent doctor
  search-index: src_sections -- both index probes are dirty: the index-side probe and fts5's own check both object, not counted in the verdict
    remedy: `intent index rebuild` re-derives the index from its content table
    orphaned: 1 docid(s) the index holds with no content row: 2599
    fts5 check: fts5: checksum mismatch for table "src_sections"
  doctor: 2 finding(s) across 0 thread(s), 0 issue(s), 2 view(s), 7 file(s) -- 1 advisory(ies), not counted -- search index DAMAGED in src_sections, not counted; `intent index rebuild` repairs it
```

The two findings and the exit code of 1 belong to the scratch project; the search index is shown and not counted, so it never moves `doctor`'s exit code, and the summary line carries it under `--quiet` too. **Run `intent index rebuild`**: it rewrites the index in one pass, the entities were never affected, and `doctor` then reads `search index: no orphaned document and fts5's check clean, from two probes that share one blind spot (both read the index's segments)`. That last clause is deliberate: both readings go through the index's own segments, so a clean pair is not two independent witnesses.

## A stray directory disables the whole project

**An `STnnnn` directory anywhere under `intent/st/` that holds a thread's files is picked up as a thread, and one it cannot read stops every command** (`intent#0011`). A staging copy at `intent/st/staging/ST0099/` holding `info.md` and `acceptance.md`, copied from a thread this build rendered, is enough. What you get is not a duplicate row in a listing, it is:

```
  $ intent st list
  error: this project has not been migrated to Intent v3 -- it declares Intent 3.0.3, and 1 steel thread carries v2 canon this binary cannot read (ST0099)
    remedy: run `intent upgrade` to migrate this project to Intent v3
```

at exit 1 on every verb, including ones that have nothing to do with the stray thread; `intent doctor` reports it as `residue: unmigrated` and exits 4, the code for an estate it could not judge. The remedy is misleading: the project is fine and one directory is not. Keep working copies of threads outside `intent/st/`.

**A second route reaches the same wall.** A thread placed under an `_inbox/` status directory produces the identical stop (`intent#0066`). Two different stray-directory shapes, one symptom, and in both cases the message names a thread the operator never created.

## Threads

**`intent wp show` prints a short header, not the work package's `info.md`** (`intent#0245`). After writing a body with `intent set intent:///threads/ST0001/wp/01 body <text>` and hydrating the thread, `intent/st/ST0001/WP/01/info.md` is on disk carrying that body, and:

```
  $ intent wp show ST0001/01
  ST0001/WP-01: first package
  status: Not Started
  scope: S
  criteria: none scoped to ST0001/01
```

None of the body is shown. `intent wp --help` describes the verb as `Show work package info.md`, so the description and the behaviour disagree. Read the file directly if you want its body.

## Criteria and tests

**A work package or thread whose criteria are all descoped or withdrawn cannot be marked done, and the refusal's `remedy:` line does not say how to close it** (`intent#0063`). Give a work package one criterion, withdraw it, and `intent wp done` refuses at exit 1 with:

```
  error: ST0001/01 is not ready to close -- gate: ST0001/01 BLOCKED -- all 1 in-scope AC(s) are descoped or withdrawn; nothing is left to verify. Add one with `intent ac new`, bring one back with `intent ac rescope` or `intent ac reinstate`, or cancel the unit with `intent st cancel` or `intent wp cancel`.
    remedy: satisfy or formally descope the remaining criteria, then close again
```

The diagnosis names the routes that work, and the `remedy:` line under it still speaks of remaining criteria, of which there are none. **Follow the diagnosis**: add a criterion with `intent ac new`, bring one back with `intent ac rescope` or `intent ac reinstate`, or run `intent wp cancel <ST>/<NN> --reason <text>`. A thread in the same state gets the same refusal from `intent st done`, and there `intent st cancel` is the route. The thread-level exemption is not one: `acceptance: exempt` is fixed when a thread is authored, and `intent set intent:///threads/ST0001 acceptance exempt` refuses with `` `acceptance` cannot be set on `intent:///threads/ST0001`: the close-gate exemption, fixed when the thread is authored and moved by nothing afterwards ``.

**A test-backed criterion cannot carry a note, and the refusal sends you round a loop** (`intent#0211`). On a test-backed criterion that is not yet satisfied, `intent ac edit <ST> <AC> --note <text>` refuses at exit 1:

```
  error: `note` cannot be set on `intent:///threads/ST0001/ac/AC-01.3`: AC-01.3 is computed, and only an unsatisfied criterion carries a note -- a computed row keeps its own record, so move it with `intent ac unsatisfy|rescope|reinstate` first
    remedy: go to the door the refusal names: a lifecycle verb for a field a state machine owns, and the member's own address for a collection
```

The criterion is unsatisfied (`ac show` prints `satisfied: no`), and none of the three verbs moves it: `ac unsatisfy` refuses with `AC-01.3 is test-backed, so its satisfaction is computed from covering green acceptance tests and cannot be set directly`, and `ac rescope` and `ac reinstate` each answer `ok: AC-01.3 already computed` and change nothing. Put the note on the covering test row instead: `intent at edit <ST> <AT> --note <text>` writes it.

**The close gate passes a test row that cites no file** (`intent#0213`, and `intent#0229`, a second row for the same code site filed from an independent report). Create an AT with the default `--kind test` and no `--file`, take it red and then green, and both of these exit 0:

```
  $ intent at lint ST0001
  lint: ST0001 ok -- 0 of 1 AT row(s) examined and conforming; 1 not examined (1 with a verdict and NO READABLE CITATION)
  $ intent ac gate ST0001
  gate: ST0001 PASS -- 1/1 satisfied
```

`at lint` names the row it did not examine and still answers `ok`; `ac gate` passes over it and says nothing.

**A wrong file is caught and no file is not.** On a row citing a file that does not exist, `intent at red` and `intent at green` each refuse at exit 1 (``AT-01.2 cites `tests/missing.rs`, which does not exist, so `green` would make it a live absent_at finding``); on a row citing nothing, `intent at green` answers `ok: AT-01.1 -> green`. So a thread whose test rows lack readable citations can pass its own close gate over evidence nothing read. Read the `not examined` clause of `at lint` before trusting a gate PASS, and give every test row a file with `intent at edit <ST> <AT> --file <path>`.

**The close gate reads your working tree, so a thread can pass on evidence nobody else has** (`intent#0265`). A test-backed criterion covered by an AT citing `tests/probe.rs`, with that file present on disk and **not committed** (`git status` lists it untracked), returns `gate: ST0001 PASS -- 1/1 satisfied` at exit 0 -- and `intent st done` then closes the thread on it. A clone of the same repository does not contain the cited file at all.

Two controls make it sharp. Remove the file from the worktree and the gate flips to `gate: ST0001 BLOCKED -- 1 acceptance test contract finding(s) over 1 row(s): AT-01.1 cites a file that does not exist: tests/probe.rs`, so the gate is genuinely reading the tree and the PASS was not indifference. And **the verdict names no tree**, so two people running the identical command in the same repository can get different answers with nothing in either output to explain the difference. Commit the cited files before reading a gate result as a claim about the project; on a shared checkout a PASS is a statement about one person's disk.

**The citation check stops at close, with nothing saying so** (`intent#0267`). Close a thread on an honest citation, then remove the id from the cited file: `at lint` answers `lint: ST0001 ok -- 1 of 1 AT row(s) examined and conforming`, `ac gate` still answers `PASS`, and `doctor` does not mention it. The exemption is deliberate -- retrofitting id labels into a finished thread is archaeology -- and the defect is that nothing distinguishes _checked and true_ from _true at close, unchecked since_: the lint line calls the row `examined`. **The file-existence arm is not exempt**: delete the cited file and the same closed thread reports `AT-01.1 cites a file that does not exist: tests/a.rs` at exit 1. So a closed thread's coverage is checked for presence and not for content, and reads identically either way.

## Editing

**Addressing an issue by its URL in `intent edit` refuses with a bare `a` glued onto a vowel-initial noun** (`intent#0081`). The kind form, `intent edit issue 0001`, now refuses before reaching it and names the verb that corrects an issue. The address form still reaches it, exit 1:

```
  $ intent edit intent:///issues/0001
  error: `issue` is not something that can be realised to disk: issue 0001's only file is its generated view, which is rendered from the store rather than authored -- `intent issues edit 0001` corrects the record it is rendered from
    remedy: address an ARTEFACT instead -- a steel thread. A `issue` has no files of its own, so there is nothing for realisation to create; if you meant the thread that carries it, address the thread.
```

The refusal is telling you the right thing, and `intent issues edit 0001` is the verb; `` A `issue` `` is the article bug, because this message builds the article by hand rather than asking the noun for it.

## Syncing

**Text appended to a generated view after its `_Generated by Intent v..._` banner is discarded by `sync --to-store`, which reports that it overwrote nothing** (`intent#0192`). Exit 0:

```
  $ printf '\n## Hand Added\n\nTEXT\n' >> intent/st/ST0001/info.md
  $ intent sync --to-store
  note: no thread the store already holds differs on disk, so this restore overwrites nothing (a thread the extract has and the store does not is an ADD and is not examined here)
  ok: store rewritten from the canon extract; nothing the store already held was overwritten
  $ grep -c TEXT intent/st/ST0001/info.md
  0
```

`intent doctor` run before the sync reports the appended text as `view-skew` at exit 1; run after it, it reports nothing, because the view has been regenerated. **Run `intent doctor` before `sync --to-store`, and put prose in `## Objective` or `## Context`, the two sections that round-trip, or write it with `intent set`.**

## The rule critics

**`severity_min` in `.intent_critic.yml` is honoured by the pre-commit gate and ignored by the runner you would test with** (`intent#0288`). You will meet this file: `intent claude upgrade --apply` seeds it into a project when it is absent, and the critic's own refusal routes you to it. With `severity_min: critical` in the file and a shell file carrying a warning-level finding, `intent critic shell --files src/w.sh` still reports `[WARNING] IN-SH-CODE-002` at exit 1, while committing the same file through the installed gate passes at exit 0 (`intent critic gate: 1 of 1 declared language(s) enforced (shell).`). The runner takes its severity from the command line alone and falls back to the `warning` default, so raising the floor in the file changes what the gate reports and nothing about the run you made to check it. Pass `--severity-min <lvl>` explicitly when you want the runner to answer the same question as the gate; with `--severity-min critical` the same run answers `ok: no shell findings at severity >= critical across 1 file(s)`.

## Declared and not implemented

Listed in `--help` and refusing when called, exit codes as shown.

**`intent config` and `intent learn`.** Both are in `intent --help`, and each refuses with ``error: `config` is a known command that is not implemented yet`` (`learn` likewise), exit 2.

**`intent ext` and every verb under it** (`intent#0177`, which records the missing `remove`). `intent ext --help` lists `list`, `show`, `validate` and `new`, and each refuses with ``error: `ext` is a known command that is not implemented yet``, exit 2. `intent ext remove` is not listed and refuses with `error: unrecognized subcommand 'remove'`, exit 1.

## Uninstalling a skill

**A second `uninstall` of a skill already removed reports a change that did not happen.** After `intent claude skills uninstall in-debug --force` has removed an edited skill, running `uninstall` again prints `in-debug removed nothing` over `ok: 1 changed, 0 already settled, 0 need a decision`, at exit 0. Read the per-skill line, not the total. **The skill's directory is left behind empty** in `~/.claude/skills/` after the removal. For a skill this build did not write, the file stays on disk and the tool says so -- `removed nothing; left 1 this build did not install, so it is still loadable` -- which is it being careful rather than a defect.

## What this page does not cover

**This is the driven set, not the whole register.** Every entry was driven in a fresh scratch project under an isolated `HOME`. Conditions that setup cannot create are not covered: several sessions sharing one checkout, and surfaces not driven for this page -- `intent fc`, `intent mcp`, `intent graphql`, `intent explore`, `intent browse` and the menubar app. Defects that need those conditions are not described here, because an undriven defect is a guess.

If you hit something not listed, that is the gap rather than a surprise. The register is the fuller record, and `intent doctor` reports on your own project.

## Reading this against your own build

`intent --version` names the build you are on, and the sha it prints is the commit the binary was built at; a release carries its tag's commit.

**The register itself cannot tell you which build a row describes** (`intent#0191`). An issue carries no field naming the version it was broken or fixed in -- `intent issues show <id> --json` has `body`, `created`, `number`, `reporter`, `schema`, `severity`, `slug`, `status` and `title` and nothing about a build -- so `intent issues list` cannot separate rows about a published build from rows about `main`. This page is that partition for v3.1.0, drawn by driving each row. **If you find an issue that seems to describe your version, check here before believing it.**
