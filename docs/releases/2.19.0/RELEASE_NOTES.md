# Intent v2.19.0 Release Notes

**Release Date**: 2026-08-14

## Overview

Intent v2.19.0 gives the **acceptance-test (AT) row a real grammar** and makes `intent at lint` enforce it, alongside thirteen other fixes that turn out to share one shape: a reader that recovered part of what it was given, discarded the rest, and said nothing about it.

This is a minor, not a patch. It is behaviour-changing for any project that keeps acceptance contracts: **the close-gate honours the new grammar from the day it ships**, so an estate written against the old free-form convention will gate `BLOCKED` until it is swept. That is the fix working -- every row it names was already contributing no coverage, silently, and the gate was reporting threads closer to done than they were. `intent upgrade` runs the mechanical migration for you.

Fourteen issues closed: 0009 through 0022.

**Elixir projects: one thing to check after upgrading.** If your project has a root-level `credo_checks/` directory, Intent put it there and has now stopped shipping it. Run `intent doctor` — it will tell you whether those checks were ever actually running, and what to remove. Details under [Removed](#removed-credo_checks) below.

## The headline: the AT row has a grammar

An AT row asserts _this named test, in this named file, proves these named criteria, and here is its state_. Three of those four were previously recovered from free-form markdown by independent single-shot regexes, and the reference was the weakest link: it was defined as **whatever sat inside the first pair of backticks**. It was not required to be a path, to contain a directory, to exist, or to be present at all.

The consequence is that **no row could ever be malformed -- only partially recovered, silently, one field at a time**. On one reporting estate that produced five mutually incompatible reference forms across 314 rows, and two live green ATs citing CSS utility classes as their test files, with no diagnostic anywhere.

### The new shape

One anchored pattern with two arms:

| Arm          | Shape                                                     | Statuses                     |
| ------------ | --------------------------------------------------------- | ---------------------------- |
| **Test**     | backticked repo-relative path to the test file            | `green` / `red` / `to-write` |
| **Non-test** | `(non-test)` + prose, for doc / eyeball / gate assertions | `n/a`                        |

Every field reader is one line over that pattern, so a non-conforming row now yields **no field** rather than a plausible wrong one.

### `intent at lint <ID>`

| Check  | Enforces                                                                                |
| ------ | --------------------------------------------------------------------------------------- |
| **L1** | the row matches the grammar                                                             |
| **L2** | a `green`/`red` row's cited file exists (`to-write` exempt -- absence is correct there) |
| **L3** | the cited file contains the literal AT id                                               |
| **L4** | every covered id is a real AC row                                                       |
| **L5** | a non-test AT is not the sole cover for a test-backed AC (`n/a` is never green)         |

### Link by id, not by test name

This is the one substantive change to how you write a row. A cited test _name_ is unverifiable -- paraphrase defeats every string match, which is exactly why the reference grew three competing shapes. An id is checkable from both ends: the row names the file, the file names the row, and `rg AT-03.2` finds both.

**Name the test by putting the AT id inside it.**

### `--fix` refuses what it cannot migrate without loss

`intent at lint <ID> --fix` migrates the mechanical half and deliberately leaves two shapes alone:

- **`path::"name"` citations**, because the migration is two-ended -- cite the file, _and_ put the AT id inside the test. Stripping the name before the id lands does not half-migrate the row; it breaks the only link the row had.
- **`pathA + pathB` citations**, because the grammar admits one file and choosing which survives is the author's call.

Both are reported by name, with every cited file listed and a note on where the rest belong. Measured against a consumer estate of 1642 AT rows: 268 rows still migrate mechanically, and none loses a cited file or a test name.

## Also in this release

### Acceptance criteria have four states, not two

`intent ac descope --to <thread>` / `rescope`, and `intent ac withdraw --reason <why>` / `reinstate`.

A requirement that moves to another thread, or is withdrawn outright, is routine -- and previously both representations available were wrong. `satisfy` is a lie (the work was not done); leaving it unsatisfied is honest but permanent, so the AC counts against the thread forever and `wp done` refuses to close work that is genuinely finished.

Both new states are non-blocking and **reported separately rather than folded into the totals** (`29/29 satisfied, 1 descoped -- PASS`), because a thread that descoped half its contract has to look like one. Each verb carries its audit payload: `descope` validates `--to` against a real thread, `withdraw` requires `--reason`. A contract emptied entirely by off-scoping is refused, and the refusal names the existing `acceptance: exempt` escape -- an exemption is announced, never inferred from emptiness.

### Fixes

| Issue    | Fix                                                                                                                                   |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| **0019** | `steel_threads.md` indexes all threads again -- it was composed from the WIP-only view, so it decayed to empty at every release close |
| **0020** | `intent st list --status all` means all -- it walked ten status literals and dropped anything else, silently, exit 0                  |
| **0011** | `intent st` stops treating any `STnnnn` directory at any depth as live, so staging areas no longer manufacture duplicate ids          |
| **0016** | `.claude/settings.json` no longer bakes an absolute home path -- hooks resolve at runtime via `intent claude hook <name>`             |
| **0018** | the treeindex cache is no longer tracked; it is derived, machine-flavoured, and went stale in silence                                 |
| **0015** | the gate no longer counts a green AT whose cited test file does not exist                                                             |
| **0014** | coverage ids with fused punctuation (`AC-09.1's`) no longer drop silently                                                             |
| **0009** | `AGENTS.md` prerequisites come from the declared `languages`, not filesystem probes                                                   |
| **0010** | `st done` / `wp done` warn when the Objective still holds the template placeholder (warn, never block)                                |
| **0012** | the whiteboard header block is ruled line-oriented `key: value`, not YAML, and `ws hygiene` enforces what it actually implements      |
| **0022** | `st new` / `wp new` no longer substitute a hand-written copy when a template is missing -- a broken install now says so               |

<a id="removed-credo_checks"></a>

## Removed: `credo_checks/` (Elixir)

`intent st zero install` used to copy six custom Credo checks into a root-level `credo_checks/` directory in any Elixir project it retrofitted, then try to register them in `.credo.exs`. **The copy was unconditional; the registration was not** — it was skipped silently when `elixir` was not on PATH, and reduced to a printed warning when it failed. The command reported success either way.

So the usual result was a directory of checks that no runner ever loaded. One project carried them in `elixirc_paths` for five months — compiled into every `dev` and `test` build, executed zero times. By the time they were wired up experimentally, one of them crashed Credo 1.7.19 outright: it calls `ExecutionIssues.append/2` with an argument shape from a pre-1.7 API. They had stopped being _wirable_ during routine dependency bumps, and nothing signalled it, because nothing ran them.

They were also redundant. Their concerns — Highlander, thin coordinator, debug artifacts, `@impl`, assertive access — are the same rules the Intent rule library states and the `critic-<lang>` pre-commit gate enforces. Two mechanisms for one concern, and the second one dead.

**`lib/templates/credo_checks/` and the `.credo.exs` configuration script are deleted**, and `D5a` is now rejected as an unknown deliverable. They are not ported to the current Credo API: that would restore the duplication deliberately, at maintenance cost, to shadow a gate that already works.

### What to do if you have one

Run `intent doctor`. It reports three states, and they are genuinely different situations:

| What doctor says      | What it means                                  | What to do                                                                                              |
| --------------------- | ---------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `have never run`      | `.credo.exs` does not reference the directory  | Inert. Delete it — doctor prints the `git rm`                                                           |
| `these checks DO run` | `.credo.exs` references it                     | **Verify against your installed Credo first.** A pre-1.7-API check takes your whole `--strict` run down |
| `stale registration`  | `.credo.exs` names checks whose files are gone | Remove the `requires:` entry and the check modules from `.credo.exs`                                    |

**If `mix.exs` also names `credo_checks` in `elixirc_paths`, doctor quotes those lines with their numbers.** Remove both ends in one commit — deleting the directory on its own breaks the build.

Doctor warns and never errors here, so this will not block a release or a gate.

## Upgrading

```
intent upgrade
```

One pass does all of it:

- runs the **`at_grammar` migration** over your acceptance contracts, so you are swept by upgrading rather than by knowing the command;
- converges **`AGENTS.md`** (after the canon apply, so it sees canon's own new files);
- rewrites **`.claude/settings.json`** to the portable hook form -- byte-identical on every machine;
- adds **`intent/.treeindex/`** to the canon-managed `.gitignore`.

### Two things to expect

**Your close-gate may report BLOCKED afterwards.** That is the intended outcome on an unswept estate, not a regression. Rows `--fix` will not touch are reported by name -- work through those by hand, citing the file and putting the AT id inside the test.

**If your treeindex cache is already tracked**, the upgrade prints the exact `git rm -r --cached` and does **not** run it. Ignoring a path does not untrack what is already tracked, so the ignore rule alone would silently do nothing -- but staging deletions across your tree, during an upgrade you invoked for other reasons, is not a decision this tool gets to make for you.

## Compatibility

- No config migration; no on-disk relocation.
- Projects with no `acceptance.md` are unaffected by the grammar work -- the gate remains opt-in by presence.
- `intent at na` / `red` / `green` now refuse a status the row's arm does not admit, **before** writing rather than after.

Internal narrative: `intent/history/v2.19.0.md`. Per-issue record with full verification: `intent/issues/CLOSED/0009` .. `0020`.
