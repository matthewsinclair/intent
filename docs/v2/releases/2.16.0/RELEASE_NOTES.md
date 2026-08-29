# Intent v2.16.0 Release Notes

**Release Date**: in progress

## Overview

Intent v2.16.0 adds the **`content` (web-content) project-type pack** (ST0053) and the **`IN-PR-*` shared prose base** that both prose disciplines now stand on. `content` is the second non-code discipline after `author` (v2.15.0); a project that declares `languages: [content]` gets a web-content rule pack, the shared prose critic, canon templates, and an essentials skill, activated exactly the way a code language pack is.

This is a minor, not a patch: it adds a new project-type surface plus a shared base pack. It is strictly opt-in -- a project that does not declare `content` sees no behaviour change -- and needs no migration.

## The Highlander decision: reuse, not copy

The obvious way to build `content` would have been to copy the `author` pack and swap in web rules. That copies banned-filler, the trope pass, and heading hygiene into a second home -- two divergent copies of the same mechanical surface, drifting apart. Instead, ST0053 lifts the shared surface into an `IN-PR-*` **prose base** that every prose discipline depends on, and lets each pack own only its genuinely distinct rules.

## What's new

| Asset     | What it is                                                                         |
| --------- | ---------------------------------------------------------------------------------- |
| Base pack | Four shared `IN-PR-*` mechanical rules -- `intent claude rules list --lang prose`  |
| Rule pack | Six web-distinct `IN-CO-*` rules -- `intent claude rules list --lang content`      |
| Critic    | `critic-prose` (renamed from `critic-author`) -- serves every prose discipline     |
| Canon     | `intent lang init content` installs `RULES-content.md` + `ARCHITECTURE-content.md` |
| Skill     | `/in-content-essentials` -- loaded by `/in-session` when `content` is declared     |

### The prose base (`IN-PR-*`)

The mechanical prose-hygiene rules that any prose discipline shares:

- `IN-PR-STYLE-001` banned filler and house style (`eg` not `e.g.`)
- `IN-PR-STYLE-002` no vanity metrics
- `IN-PR-STYLE-003` one H1, no skipped heading levels
- `IN-PR-STYLE-004` the mechanical trope pass

The `author` pack was refactored onto this base: its four mechanical rules moved into `IN-PR-*` (with migration aliases), leaving `author` its discipline-specific rules only. No behaviour change for an author project.

### The content pack (`IN-CO-*`)

Six web-distinct rules in two tiers:

- **`style` (mechanical)** -- `IN-CO-STYLE-001` page meta present (title / description / canonical), `-002` image alt-text, `-003` descriptive link text.
- **`craft` (judgment)** -- `IN-CO-CRAFT-001` scannability and web voice, `-002` one clear primary call to action, `-003` reading level matched to the audience.

### One critic for every prose discipline

`critic-author` is renamed to `critic-prose`. It loads the `IN-PR-*` base plus whichever of `author` / `content` the project declares (both, if both), resolved from config `languages`. Two modes: `review` (the mechanical `style` tier, default) and `craft-check` (the judgment `craft` tier, on instruction). The two-form detrope is preserved, re-anchored to `IN-PR-STYLE-004`.

**Migration note:** a v2.15.0 project that dispatched `critic-author` should dispatch `critic-prose`.

## Activating the pack

```
intent lang init content
```

Installs the content canon, appends the Language Packs entry, and adds `content` to config `languages`. `/in-session` then loads `/in-content-essentials`, and `/in-review` dispatches `content -> critic-prose`.

## Deferred

The headless prose gate stays deferred (D4). The self-dogfood confirmed the reason: the pack's own prose is clean, but the mechanical grep false-positives on rule files (which quote antipatterns deliberately) and fenced examples, so a headless gate still needs a confirmation / house-style suppression layer. `critic-prose` is on-demand (`Task`) only.
