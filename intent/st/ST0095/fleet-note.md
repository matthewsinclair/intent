# Intent 3.5.0: the fleet's upgrade note

Intent 3.5.0 gives new threads, work packages and issues git-style ids, and moves each project's canon to a new format. Intent 3.4.x cannot read a project once it is upgraded, so the order below is the whole of this note. The release notes' Upgrading section (docs/releases/3.5.0/RELEASE_NOTES.md) is the full account.

## 1. Every machine upgrades Intent before any project on it is upgraded

**No hashed record is committed in a project until every machine that opens that project runs 3.5.0.** A 3.4.x machine cannot read an upgraded project: it reports each canon file as schema-invalid and refuses. It also cannot read a store 3.5.0 has migrated, because the store moves one way on first open.

The Homebrew tap carries 3.5.0, so each machine takes it from brew:

```
intent daemon stop
brew update
brew upgrade matthewsinclair/intent/intent matthewsinclair/intent/intent-app
intent --version                       # names 3.5.0 (brew, ...)
intent daemon start
```

Stop `intentd` before the upgrade and start it after, because the upgrade replaces the binary a running `intentd` was started from (issue 0468). A machine that runs no daemon skips both lines.

**A machine keeps one route for starting `intentd`.** On macOS there are two: `intent daemon start --at-login` enrols a LaunchAgent, and `brew services start matthewsinclair/intent/intent` runs it under Homebrew's service. A machine enrolled both ways starts two, and one of them can be a stale build (gyges showed a 3.3.3 `intentd` beside the current one). Keep one: `intent daemon stop --at-login` removes the LaunchAgent, or `brew services stop matthewsinclair/intent/intent` removes the service.

A machine that took the release from the opt tree while the tap was held runs `intent use brew` first, so brew's intent is the one on PATH again. Restart Claude sessions afterwards: the PATH change reaches login shells only.

## 2. Then each estate upgrades each of its projects, once

In each project, on a clean tree, with open branches that add work packages landed first:

```
intent upgrade
git add --pathspec-from-file=intent/.cache/upgrade.paths --pathspec-file-nul
git commit
```

The closing line of `intent upgrade` gives the two git steps. Use a plain `git commit`, not `git commit --only`. The upgrade changes no id, and a re-run that has nothing to do says so. Until a project is upgraded it keeps working on its ordinals; only `st new`, `wp new` and `issues add` refuse there, with `intent upgrade` as the remedy.

Then, in the same project, re-render its hooks and session context from 3.5.0's canon, and commit exactly the paths it prints under `wrote:` and `removed:`:

```
intent claude upgrade --apply
git add <each path it printed>
git commit --only <each path it printed>
```

## 3. What each estate meets afterwards

**A criterion in a one-digit group is now judged by its package's gate** (issue 0712, closed at 96542a105). A criterion such as `AC-1.1` used to fall outside every package's gate. It now counts toward the package it names, so `intent wp done` can block on a criterion it never saw before. Report A counted 182 such rows across Devbin, Lamplight and Prolix. Before closing a package in those estates, run `intent ac gate <thread>` and satisfy, descope or withdraw what it names.

**New ids look different.** A new thread is `ST` and 16 hex digits, shown and typed by its short form, and a new work package or issue follows the same pattern. Existing ids do not change. Anything that matched ids by a four-digit pattern (a script, a hook, a template) should ask `intent id` instead.
