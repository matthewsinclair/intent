# Intent v2.15.0 Release Notes

**Release Date**: 2026-07-03

## Overview

Intent v2.15.0 adds the **`author` project-type pack** (ST0052) -- the first non-code discipline to ride Intent's `languages` axis. A project that declares `languages: [author]` in `intent/.config/config.json` gets an authoring rule pack, a prose critic, canon templates, and an essentials skill, activated exactly the way a code language pack is.

This is a minor, not a patch: it adds a new project-type surface. It is strictly opt-in -- a project that does not declare `author` sees no behaviour change -- and needs no migration.

## What's new: the `author` pack

Four assets, the same anatomy as a code language pack:

| Asset     | What it is                                                                          |
| --------- | ----------------------------------------------------------------------------------- |
| Rule pack | Nine `IN-AU-*` rules -- `intent claude rules list --lang author`                    |
| Critic    | `critic-author` -- `Task(subagent_type="critic-author", prompt="review <file>.md")` |
| Canon     | `intent lang init author` installs `RULES-author.md` + `ARCHITECTURE-author.md`     |
| Skill     | `/in-author-essentials` -- loaded by `/in-session` when `author` is declared        |

### Two tiers

Author rules split by how they are checked, mirrored by the critic's two modes:

- **`style` (mechanical, default -- `critic-author review`)** -- greppable: banned filler and `eg`-not-`e.g.`, no vanity metrics, front-matter + learning objectives present, heading hygiene, and a mechanical trope pass.
- **`craft` (judgment, on instruction -- `critic-author craft-check`)** -- a read: voice and register consistency, cross-chapter continuity, citation and attribution, and the full `/in-detrope` diagnosis.

### Two-form detrope

detrope has one home -- `in-detrope`'s trope catalogue -- and two forms. The mechanical pass greps the catalogue's `detection: automated` regexes on every `review`. The full contextual `/in-detrope` diagnosis is emitted as a handoff recommendation; `critic-author` recommends it but never runs it. The catalogue is never forked.

### Dispatch

`/in-review` reads `languages` and dispatches `author -> critic-author`. In an author-only project no code critic runs; a mixed project (eg `languages: [elixir, author]`) runs `critic-elixir` on the code subtree and `critic-author` on the prose subtree.

## Compatibility

Fully additive and opt-in. Projects that do not declare `author` are unaffected: no new critic runs, no skill loads, no config changes. The `AU` language code and the author rule pack are available fleet-wide after upgrade but inert until a project runs `intent lang init author`.

The headless pre-commit prose gate is deliberately deferred: `.md` extension alone cannot route a file to the author pack, and Intent's own `--` house style trips the trope catalogue's dash-overuse regex, so a headless gate needs a confirmation / suppression layer first. `critic-author` is on-demand (`Task`) only in this release.

## Upgrade

```bash
intent upgrade --apply
```

The pack is served centrally from `$INTENT_HOME`, so it is available immediately after upgrade. To turn a project into an author project:

```bash
intent lang init author
```

That installs the per-project canon, appends the Language Packs entry, and adds `author` to `languages`; the next `/in-session` loads `/in-author-essentials`.

## Why

Intent already had a proven pattern for carrying a discipline -- a rule pack, a critic, canon, and a skill, activated by a `languages` entry. That pattern was only ever used for code. ST0052 asked whether the same machinery could carry a non-code discipline, and the answer is yes: authoring is just another pack. Reusing the axis (rather than standing up a parallel `domain:` system) keeps one activation mechanism and lets a courseware repo be `languages: [elixir, author]` -- both packs, no special case. The `author` pack is the first instance; a `content` (web-content) pack can now copy the shape.
