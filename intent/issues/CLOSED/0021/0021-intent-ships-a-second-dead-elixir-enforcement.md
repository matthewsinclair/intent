---
id: "0021"
title: Intent ships a second, dead Elixir enforcement mechanism: st zero installs credo_checks/ that nothing wires, duplicating the rule library it already enforces
date: 2026-08-14
reporter: matts
status: CLOSED
severity: high
---

# 0021: Intent ships a second, dead Elixir enforcement mechanism: st zero installs credo_checks/ that nothing wires, duplicating the rule library it already enforces

## Tags

elixir, credo, highlander, silent-failure, st-zero, prune

## Summary

`intent st zero install` deliverable **D5a** copies six custom Credo check templates into a root-level `credo_checks/` directory in any Elixir project, then attempts to register them in `.credo.exs`. The registration step is best-effort and its failure is a printed warning, not an error, so the common outcome is a directory of checks that **no runner ever loads**.

The checks duplicate concerns the Intent rule library and the `critic-<lang>` pre-commit gate already enforce -- Highlander, thin coordinator, debug artifacts, `@impl`, assertive access. Two mechanisms for one concern, and the second one is dead: a straight `IN-AG-HIGHLANDER-001` violation whose dead half has been rotting unobserved.

Reported by Laksa, which carried the directory for five months.

## Reproduction

Measured on Laksa, 2026-08-14. Seven checks (`Mix.Checks.{HighlanderSuspect, ThickCoordinator, DebugArtifacts, MapGetOnStruct, MissingImplAnnotation, BooleanOperators, DependencyGraph}`) sat in `credo_checks/`, added to `elixirc_paths` for `dev` and `test`, so they **compiled into every build**.

A Credo check only runs if `.credo.exs` names it -- `requires:` plus an `enabled:` entry. No revision of Laksa's `.credo.exs` ever did. The checks were born under `lib/mix/checks/`, moved to `credo_checks/` in a March "Fix Intent credo mixup" commit, and the wiring step never happened at any point along that path.

**Compiled for five months, executed zero times.**

Worse: when wired experimentally, `MissingImplAnnotation` **crashed Credo 1.7.19 outright** -- it calls `ExecutionIssues.append/2` with an argument shape from an older Credo API. So the checks had stopped being _wirable_ at some point during routine dependency bumps, with no signal at all, because nothing ran them.

In this repository, the shipping side:

```
$ grep -rn 'mkdir.*credo_checks' bin/ lib/
bin/intent_st_zero:528:        mkdir -p "credo_checks"

$ intent st zero --help          # not in `intent help`, but live via the *) auto-dispatch
Usage: intent st zero install [options]
```

## Root Cause

Two independent faults that compound.

**1. A duplicate enforcement mechanism (`IN-AG-HIGHLANDER-001`).** The custom checks encode rules the agnostic + Elixir rule packs already state, and which `critic-elixir` and the headless pre-commit gate (`bin/intent_critic`) already enforce. That pathway demonstrably works -- it blocked a Laksa commit on the morning this was found. Intent shipped a second implementation of the same concern into consumer trees and then stopped maintaining it.

**2. A silent-success install (`IN-AG-NO-SILENT-001`).** `bin/intent_st_zero:516-552` does the copy unconditionally and the wiring conditionally:

```bash
mkdir -p "credo_checks"
... cp each template ...
echo "  created: $count Credo check templates in credo_checks/"
if command -v elixir >/dev/null 2>&1; then
  config_result=$(elixir "$CONFIGURE_CREDO_SCRIPT" --remove-stale 2>&1) && \
    echo "  $config_result" || \
    echo "  warning: could not auto-configure .credo.exs"
fi
```

Two paths leave the tree looking configured and not being:

- `elixir` is not on PATH -- the whole wiring step is skipped, silently, with no message at all;
- `configure_credo.exs` fails (eg it bails with `error: .credo.exs has unexpected structure`) -- a warning is printed and the command **carries on and reports success**.

Either way the copy has already happened, `created:` has already been printed, and nothing downstream re-checks. The deliverable's own audit (`check_d5a_credo`, `bin/intent_st_zero:147-178`) does distinguish `.credo.exs configured` from `not configured` -- but it is only consulted by `st zero` itself, which nobody re-runs.

**The generalizable failure (Laksa's framing, and it is the sharper statement):** a check that exists but is not in the runner's config is invisible, and **presence in the repo reads as "enforced" to every human who finds it, which is worse than absence.** Same class as this release's `steel_threads.md` and `st list --status all` -- a thing that looks like coverage and is not.

## Impact

- **Every Elixir project retrofitted with `intent st zero install` may carry a directory of dead checks**, believed by its maintainers to be enforcing rules that nothing enforces.
- **They compile.** Added to `elixirc_paths`, they are built on every `mix compile` in dev and test -- cost with no benefit, and a build-breaking coupling if the directory is deleted without also removing those entries.
- **They rot invisibly.** At least one is incompatible with Credo 1.7.x. A project that wires them today takes its entire `--strict` run down.
- **The concern is already covered**, so the residue is pure liability: two mechanisms, one dead, and the dead one is the more visible.

## Proposed Fix

**Prune the shipping side entirely; report the consumer residue and never touch it.**

1. **Remove deliverable D5a from `intent st zero`** -- the install arm, the audit function, the label, the id from `DELIVERABLE_IDS` and the validation case, and the help table row. Intent stops creating `credo_checks/` in anyone's tree.
2. **Delete `lib/templates/credo_checks/` and `lib/scripts/configure_credo.exs`.** Fail-forward: no deprecation stub, no preserved-but-unused template tree. The rule library is the surviving mechanism.
3. **Remove the MODULES.md row** for the templates.
4. **`intent doctor` reports a consumer's residue**, modelled on Check 4d (leftover `.intent/`): warn, name the exact commands, run nothing. It must diagnose all three states, because they are different stories with the same fix:
   - `credo_checks/` present, `.credo.exs` does not reference it -- **never ran**, inert;
   - `credo_checks/` present and referenced -- they **do** run; verify against the installed Credo version before trusting them, because a pre-1.7-API check takes the whole `--strict` run down;
   - `.credo.exs` references checks whose files are gone -- **stale registration**.
5. **Name both ends.** Deleting the directory without removing its `elixirc_paths` entries breaks the build. This is a two-ended migration and the report must say so, listing what it found on each end -- the 0017 `--fix` rule: a tool that cannot finish a job must not start it, and a report that names one end damages everything that follows it.
6. **Guard**: a fixture carrying `credo_checks/` is reported by doctor in each of the three states, with the `elixirc_paths` end named; and no code path in `bin/` creates `credo_checks/`.

Deliberately NOT proposed: porting the checks to the current Credo API. That would restore the Highlander violation on purpose, at maintenance cost, to duplicate a gate that already works.

## Related

- 0018 -- the treeindex cache stops being project state. Same shape (Intent-created residue in consumer trees) and the source of the reporting rule: print the exact command, never run it across someone else's tree.
- 0020 / 0019 -- the same "looks like coverage, is not" class, one layer up.
- ST0032 -- built `configure_credo.exs` and the check templates; already retired two of them (`boolean_operators`, `dependency_graph`) as false-positive generators, which is the first evidence this mechanism was not earning its keep.
- ST0034 / ST0035 -- the rule library and the `critic-<lang>` pre-commit gate that supersede these checks.

## Resolutions

**Fixed in v2.19.0 (before the cut, on hv's instruction). Filed on Laksa's report; executed by vc.**

All six proposed items taken as filed, and the "deliberately NOT proposed" exclusion honoured -- the checks are not ported to the current Credo API, because doing so would restore the Highlander violation on purpose.

### The shipping side is gone

`intent st zero` no longer has a D5a. Removed: the install arm, `check_d5a_credo`, the label, the id from `DELIVERABLE_IDS` and from the `--deliverable` validation case, and the help-table row in `lib/help/stzero.help.md`. `lib/templates/credo_checks/` (six templates) and `lib/scripts/configure_credo.exs` are **deleted**, not deprecated -- `lib/scripts/` is now empty and gone with them. The two MODULES.md rows are removed. Nothing under `bin/` or `lib/` creates `credo_checks/` any more, asserted mechanically rather than by reading.

`D5a` now fails as an unknown deliverable rather than being silently accepted, because silent acceptance of a retired id lets a script keep asking for it and report success while installing nothing -- which is a small copy of the bug being fixed.

### The consumer side is reported, never touched

`intent doctor` check **4e**, modelled on check 4d (leftover `.intent/`). It distinguishes the three states Laksa's checklist names, and the distinction is the point -- the remedy is the same but the risk is not:

| State                                  | Reported as             | Removal command offered? |
| -------------------------------------- | ----------------------- | ------------------------ |
| present, `.credo.exs` does not name it | **has never run**       | yes (`git rm -r`)        |
| present and named in `.credo.exs`      | **these checks DO run** | **no** -- it is live     |
| absent, `.credo.exs` still names it    | **stale registration**  | n/a (edit `.credo.exs`)  |

The wired state additionally carries the version caveat, because that is the trap Laksa hit: a check written against a pre-1.7 Credo API crashes the entire `--strict` run, so "wired" is not the same as "safe".

**Both ends are named.** Where `mix.exs` also mentions `credo_checks` (the `elixirc_paths` entries), the offending lines are quoted with their line numbers. Deleting the directory without them breaks the build; this is a two-ended migration and 0017's rule applies -- a report that names one end damages everything that follows it, exactly as the lossy `at lint` suggestion did.

It **warns and never errors**. `bin/release` gates on `intent doctor` and aborts non-zero, so a consumer carrying this residue must still be able to cut a release. Verified by assertion, not assumption.

### Verification

Behavioural, in a scratch project, all four states exercised end to end: unwired (reported never-run, `git rm` printed, both `mix.exs` lines quoted with line numbers), wired (reported live, version caveat, removal command correctly withheld), stale (directory removed, `.credo.exs` retained -- reported as stale registration), and clean (`ok`). Doctor exits 0 in every residue state and reports `0 errors`.

**Mutation battery, sacrificial worktree, exact-string substitutions that hard-fail if unapplied, restored between each:**

| Mutation                                   | Kills                | Reading                                          |
| ------------------------------------------ | -------------------- | ------------------------------------------------ |
| N1 whole check 4e deleted                  | 1, 2, 3, 4, 5        | the check's existence is pinned                  |
| N2 wired/unwired distinction collapsed     | 3 only               | the live-vs-inert split is real, not cosmetic    |
| N3 the `mix.exs` end dropped               | 5 only               | the both-ends requirement is pinned              |
| N4 residue escalated to `show_error`       | 2, 5, 6              | severity is pinned -- it cannot become a blocker |
| N5 the `git rm` line dropped               | 2 only               | the removal command is pinned                    |
| N6 `st zero` creates `credo_checks/` again | the 2 removal guards | including the mechanical grep over all of `bin/` |

**One honest gap, recorded rather than papered over:** test 6 (warns-never-errors) **survives N1**. Deleting the check entirely leaves exit 0 and `0 errors` true, so that test does not pin the check's existence -- N1 is caught by tests 1-5, and N4 is what proves test 6 is not vacuous. Each test pins its own property; no single test pins all of them, which is the intended division.

N6 is the load-bearing one for the removal: it kills the mechanical grep guard, which watches all of `bin/` and `lib/` rather than the one arm that was mutated. Grep for the rule, do not read for it -- 0011's lesson.

**Collateral:** `st_zero_commands` (33), `credo_checks_residue` (6), `doctor_leftover_intent` (5), `modules_commands` (20), `help_commands` (12), `docs_completeness` (16), `bootstrap`, `global_commands`, `config`, `no_absolute_home_paths`, `release_script`, `release_sidecars`, and both integration decks -- all green. `intent critic shell` clean on both changed files. `intent doctor` still passes on this repository.

### Note on scope

The `st zero` D5a install was the only creator of `credo_checks/` in the codebase; `intent init`, `intent upgrade`, the rule library, `critic-elixir`, and the Elixir `AGENTS.md` template never referenced it. So this was the brownfield retrofit command, not "the Elixir language capability" -- worth stating because that was the open question when the report came in.
