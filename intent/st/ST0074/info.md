---
st_id: ST0074
title: Machine-wide projects: intentd's config home, the project registry and discover, the explorer's project picker, and the menubar status line
status: WIP
created: 2026-09-13
completed:
---

# ST0074: Machine-wide projects: intentd's config home, the project registry and discover, the explorer's project picker, and the menubar status line

## Objective

Make Intent's own front doors reach every Intent project on the machine rather than only the one the terminal is standing in, and present them consistently.

- The explorer handles `/threads` and `/issues` itself, and can pick a project from the ones this machine knows (`/projects`, and `intent explore` outside a project).
- A single, standards-compliant home for intentd's durable configuration, human- and machine-editable, that lists every known Intent project.
- `intent explore` keeps the project it runs in registered there, and `intent discover [fromdir]` finds Intent projects under a directory, checks each is config-compatible with the running intentd, and registers it.
- intentd watches that file and refreshes what it knows when it changes.
- The menubar app presents its menu in the same shape as Gtools': an identity row, one status line (place, state, details), then the actions.

## Context

Raised by hv on 2026-09-13, while the cut waited on 0354, and ruled into one bundle: "all of this should really go into the one work bundle. If we need a new ST to hold it all, then make one and make the various bits WPs inside it."

What exists today, measured rather than recalled:

- intentd's project registry (`intentd/src/registry.rs`) is in memory only. A project is registered as a side effect of first contact, so a freshly started daemon knows nothing until each project has been used again.
- intentd has no durable configuration of its own. Its runtime state lives under `~/.local/share/intent/` (socket, loopback address, token, lock, log). The operator's v3 configuration is `~/.intent/config.json` (`intentsvcs::userstate::global_config`).
- `~/.config/intent/config.json` is v2's file (it reads `intent_version: 2.0.0`, `backlog_dir`, `editor`) and nothing in v3 reads it: every v3 per-user store has its own path and never reads or writes v2's (hv adopted, 2026-08-22).
- The shipped surface may read only the environment variables in `no_intent_home.rs`'s `ALLOWED` (`COLUMNS`, `EDITOR`, `HOME`, `USER`, `VISUAL`). A standards-compliant location that honours `XDG_CONFIG_HOME` or similar needs a grant from hv, recorded as a row there.
- `intent explore` outside a project refuses through `context()`: it needs a project directory to open.
- The explorer's palette sent `/issues` to `intent issues` and offered no `/threads`; landed as the explorer's own acts at 793984a50.

hv's design for the registry, in their words: "intentd has some global config that is easily human AND machine editable. Any time 'intent explore' is run, it should ensure that if it is running in an intent project, that that project is up to date in the global config that intentd can find it. AND: we need an 'intent discover [fromdir]' command that looks for all intent projects from the current dir (by default) or a dir that is passed in as a param. Whenever it finds one, it confirms that it is 'config compatible' with the current intentd, and then ensures that it is registered in the global config for intentd to find. intentd keeps an eye on that global config file and refreshes its knowledge of host-wide intent projects whenever it changes."

And on the location: "We also need to resolve where intentd's config resides ... that answer needs to be standards compliant." The location is ruled before the registry is built, because the registry is the first thing to live there.

What `intent explore` does outside a project when no daemon answers depends on the registry: hv, "the answer here changes depending on the outcome of 1".

## Work Packages

| WP    | Title                                                                                                                    | Size | Status |
| ----- | ------------------------------------------------------------------------------------------------------------------------ | ---- | ------ |
| WP-01 | The explorer handles /threads and /issues itself                                                                         | S    | Done   |
| WP-02 | The menubar's one status line, in Gtools' shape                                                                          | S    | Done   |
| WP-03 | The project registry: explore registers its project, intent discover registers compatible ones, intentd watches the file | L    | Done   |
| WP-04 | The explorer's project picker: /projects, and intent explore outside a project                                           | M    | Done   |
| WP-05 | Where intentd's durable configuration lives: a standards-compliant home, ruled by hv                                     | S    | Done   |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0074.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.2 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
