---
id: "0058"
title: agents template: show is undeclared in the register so v3 drops it, list advertises seven templates show cannot deliver, and every error goes to stdout
date: 2026-08-17
reporter: matts
status: OPEN
severity: high
---

# 0058: agents template: show is undeclared in the register so v3 drops it, list advertises seven templates show cannot deliver, and every error goes to stdout

## Tags

surface, dispatch-table, agents, voice, measured, INV-01, subject-of-measurement, arm-scoped

## Summary

Three defects on one row, found together because the arm was executed for the first time. `surface/dispatch-table.json`'s `agents template` entry declares `args[0].values: ["list"]` -- **so `show` is not in the register at all, and v3's clap surface renders from the register.** v2's own usage line names it: `Usage: intent agents template [list|show <name>]`. An undeclared subcommand is a dropped subcommand.

**And the arm `show` exists to serve is broken for seven of the eight things `list` advertises.** `list` enumerates directories under `$TEMPLATES_DIR`; `show <name>` requires `$TEMPLATES_DIR/<name>/AGENTS.md`. Only `elixir/` has one. The other seven carry `ARCHITECTURE.md` + `RULES.md` -- they are per-language canon packs, not AGENTS.md templates -- so `intent agents template show rust` answers `error: template 'rust' not found` about a name `intent agents template list` printed one command earlier.

**`list`'s subject is "directories under templates/"; `show`'s subject is "directories containing AGENTS.md". The two subjects differ and the output cannot tell them apart** -- vc's consolidated class in `parity.md`, arriving in a command PAIR rather than in a measurement, and with the list side reading as an inventory of what the other side can do.

**Third: every error on this row goes to STDOUT, and one of them has no `error:` prefix**, against INV-01 (`Every failure writes error: <message> to STDERR`). The row nevertheless records `stderr: "error: ... on stderr (INV-01)"`. That claim was never checked because the row is `evidence_class: read` -- taken from the source without executing the arm -- which is the same shape as the four wrong `observed` values found on 2026-08-17.

Found by ic, 2026-08-17, driving the row while writing arm-scoped stdout declarations (board TODO-3).

## Reproduction

v2.19.0 on PATH, in a throwaway `intent init` project.

**The two arms the register knows about are byte-identical**, so the "list arm vs default arm" distinction the row implies does not exist:

```
$ intent agents template
Available AGENTS.md templates:
  - _default
  - author
  - content
  - elixir
  - lua
  - rust
  - shell
  - swift
                                          # exit 0

$ intent agents template list             # byte-identical to the above
```

**The undeclared arm, and the 7-of-8 failure:**

```
$ intent agents template show rust
error: template 'rust' not found          # exit 1 -- and `rust` is in the list above

$ intent agents template show elixir
<the elixir AGENTS.md>                    # exit 0 -- the only one that works
```

Enumerated against the tree, `list=yes` for all eight and `show` succeeds for one:

| template   | listed | showable | contents                     |
| ---------- | ------ | -------- | ---------------------------- |
| `_default` | yes    | **no**   | `ARCHITECTURE.md` `RULES.md` |
| `author`   | yes    | **no**   | `ARCHITECTURE.md` `RULES.md` |
| `content`  | yes    | **no**   | `ARCHITECTURE.md` `RULES.md` |
| `elixir`   | yes    | yes      | `AGENTS.md`                  |
| `lua`      | yes    | **no**   | `ARCHITECTURE.md` `RULES.md` |
| `rust`     | yes    | **no**   | `ARCHITECTURE.md` `RULES.md` |
| `shell`    | yes    | **no**   | `ARCHITECTURE.md` `RULES.md` |
| `swift`    | yes    | **no**   | `ARCHITECTURE.md` `RULES.md` |

**The stream, split rather than merged** -- `2>&1` would have hidden this and the first reading of it did:

```
$ intent agents template rust 2>/dev/null
Unknown template subcommand: rust
Usage: intent agents template [list|show <name>]
                                          # BOTH lines on STDOUT, exit 1

$ intent agents template rust 2>&1 1>/dev/null
                                          # stderr is EMPTY
```

## Root Cause

`intent/plugins/agents/bin/intent_agents:768-801`. Every branch uses bare `echo`, so all output -- success and failure alike -- goes to stdout, and the catch-all writes no `error:` prefix:

```bash
    list)
      echo "Available AGENTS.md templates:"
      for tdir in "$TEMPLATES_DIR"/*/; do
        [ -d "$tdir" ] && echo "  - $(basename "$tdir")"      # <- any directory
      done
      ;;
    show)
      local template_file="$TEMPLATES_DIR/$template_name/AGENTS.md"
      if [ -f "$template_file" ]; then                        # <- must hold AGENTS.md
        cat "$template_file"
      else
        echo "error: template '$template_name' not found"     # <- stdout
        return 1
      fi
      ;;
    *)
      echo "Unknown template subcommand: $subcommand"         # <- stdout, no `error:`
      echo "Usage: intent agents template [list|show <name>]"
      return 1
      ;;
```

The `list`/`show` disagreement is the `-d` test against the `-f` test. Neither is wrong on its own; **they are answering different questions and the user is shown only the first answer.** The directory became polymorphic when the per-language canon packs (`ARCHITECTURE.md` + `RULES.md`, ST0035) moved in beside the one real AGENTS.md template, and `list` was never narrowed to match. **`list` did not break -- its meaning was changed underneath it by an unrelated feature, and nothing re-read it.**

The register's omission of `show` has the same origin as the wrong `stderr` claim: `evidence_class: read`, so the entry was written from the dispatch site and the `values` list from the one subcommand the caller happened to name.

## Impact

**`intent agents template show` disappears from v3 unless the register gains it**, because the clap surface, the help text and the MCP tool list all render from `surface/dispatch-table.json` (AC-05.1). This is a silent drop: nothing fails, the subcommand is simply not there, and the row that should have caught it is the row that omits it.

**In v2 today, seven of eight advertised templates are unreachable through the command that advertises them.** The failure mode is worse than a plain error because `list` is the discovery path -- a user reads the list, picks a name off it, and is told the name does not exist. The natural next inference is that they typed it wrong.

**And the row asserts INV-01 compliance the arm does not have.** Anything auditing INV-01 from the register -- rather than from the binary -- passes this row. That is the register being confidently wrong in v3's favour, which is the failure vc named on the `observed` column: an uncovered row is unknown, a wrongly-recorded row is worse.

## Proposed Fix

**Declare `show` in the register** -- `args[0].values: ["list", "show"]` -- with the trailing `<name>` argument it takes. Without this, the rest is moot: v3 has no arm to fix.

**Rule `list` CORRECTED, not as-observed, and narrow it to what `show` can serve.** Reproducing a list that advertises seven names its sibling cannot deliver is laundering a v2 defect into a v3 requirement, which `parity.md` forbids in as many words. Either `list` enumerates only directories containing `AGENTS.md`, or -- better, since the canon packs are real and worth discovering -- it reports what each directory actually offers, so the two subjects are visible instead of conflated. **A one-word failure is the wrong fix here: the defect is that one output silently stands for two different sets.**

**Fix the stream and the prefix under INV-01**: both error paths to stderr, and `Unknown template subcommand:` becomes `error: unknown template subcommand: <x>`. The `Usage:` line is a legitimate second line and belongs on stderr with it.

**Correct the row's `observed.stderr`** -- it currently claims a compliance the arm does not have -- and move `evidence_class` off `read` now that all three arms have been executed.

**The row needs ARM-SCOPED stdout declarations, which the schema does not yet have.** `list`, `show <valid>`, `show <invalid>` and the catch-all have four different outputs and one row. This is the same collapse as a row pretending to one exit code when it has several, and it is the general form of the `issues add` two-line problem: **the row is not always the unit of the claim.**

## Related

- ST0056 -- Intent v3.0.0
- 0056 -- `AtStatus` has no `display()`; the same `evidence_class: read` origin
- `parity.md` -- the subject-of-measurement class; this is its arrival in a command pair
- `intent/plugins/agents/bin/intent_agents:768-801` -- all three defects
- `surface/dispatch-table.json` -- the `agents template` row, `values: ["list"]`
- INV-01 -- the voice invariant this arm violates while the row records compliance
- ST0035 -- moved the per-language canon packs into `templates/`, which is what changed `list`'s meaning

## Resolutions

{{TBC}}
