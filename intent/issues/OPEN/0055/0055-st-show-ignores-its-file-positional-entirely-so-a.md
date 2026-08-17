---
id: "0055"
title: st show ignores its file positional entirely, so a keep row answers a different question at exit 0
date: 2026-08-17
reporter: cc
status: OPEN
severity: high
---

# 0055: st show ignores its file positional entirely, so a keep row answers a different question at exit 0

## Tags

parity, silent-success, dispatch-table, WP-05

## Summary

`intent st show <ID> <file>` accepts the `file` positional, discards it, and prints a four-line synthesised summary instead of the file. The row is `disposition: keep` / `target.state: as-observed` and declares four exit codes; v3 can produce one of them. So `intent st show ST0001 design` prints the info summary at exit 0, where v2 prints `design.md` -- the operator is told the command succeeded and is shown something else.

Found while probing every dispatch-table slot that declares `values`, prompted by `arg_values_note`'s statement that such a slot "owes an exit-1 arm for an unreadable value". `st show` is the one wired command in that population, and it does not merely fail to enforce the six values -- it never reads the slot.

## Reproduction

`native/rust/target/debug/intent` at `b7e60fc5`, in a project with one thread:

```
$ intent st show ST0001 design
ST0001: A thread
status: WIP
created: 2026-08-15
$ echo $?
0
$ intent st show ST0001 nonsense
ST0001: A thread
status: WIP
created: 2026-08-15
$ echo $?
0
```

Measured against v2 (`bin/intent_st:1044-1100`), which is what the row's `observed` block describes:

| invocation                     | v2                                      | v3                       |
| ------------------------------ | --------------------------------------- | ------------------------ |
| `st show ST0001`               | `cat info.md`                           | four-line summary        |
| `st show ST0001 design`        | `cat design.md`                         | the SAME summary, exit 0 |
| `st show ST0001 all`           | every file, `-- <file>` separators      | the same summary, exit 0 |
| `st show ST0001 nonsense`      | exit 1, `error: Unknown file type: ...` | exit 0                   |
| `st show ST0001 <absent file>` | exit 1, `error: File not found: ...`    | exit 0                   |

Three of the row's four declared exit-code cases are unreachable in v3, and the two error cases are the ones that carried information.

## Root Cause

`render.rs`'s `st show` arm reads `arg(a, "id")` and never reads `file`. There is no `Unknown file type` path because there is no read to fail. The declared `values` array is inert for the reason `arg_values_note` gives -- nothing in `spine.rs` builds a `value_parser` from a positional's `values` -- but this is the stronger case: enforcement is absent because the slot itself is unimplemented, so a correct value and an incorrect one produce identical output.

## Impact

`st show` reports success for a request it discarded. An operator who asks for `design` and reads the info summary has no signal that the file they asked for was never opened -- and `all`, whose whole purpose is to concatenate the thread's documents, prints three lines. Scripts written against v2's exit 1 on a bad file type see exit 0.

Secondary: the row's `target.note` says "`show` reads the view, so its output is unchanged in kind". That sentence is currently false -- the binary reads no view. It is the fourth artefact in vc's class (`parity.md`, `bee0a0dd`) of a record that states the requirement while the code does something else, and the third on this one command family after `wp show`'s `status: wip`.

## Proposed Fix

Not decided here, because the choice belongs to the WP-05 surface cut and touches ic's row and vc's parity reading:

1. **As-observed**: `st show <ID> [file]` cats the generated view for `info` / `acceptance` and the authored file for `design` / `impl` / `tasks`, `all` concatenates with v2's `-- <file>` separators, an unreadable value exits 1 naming the permitted six, and an absent file exits 1. This is what `keep` / `as-observed` already says, and it makes the row's own note true.
2. **Deliberate divergence**: keep the summary, declare it in the row as a target change with a reason, and give the `file` slot an exit-1 arm so a discarded request is never reported as a success. The summary would then need a name that is not `show`, or `show` needs both behaviours.

Either way the silent-success path closes first: a slot the command does not honour must refuse, not succeed.

## Related

- ST0056 -- WP-05 (surface cut) and WP-03
- 0052 -- `wp rescope`: the same class one entity over (a ratified field the surface could not reach). The difference is instructive: 0052's field was unreachable and nothing happened, while this one reports that something did.
- 0050 -- the no-op voice; `reported()` exists because a command must not confirm a movement it did not make. This is the same rule applied to a READ.
- 0047 -- one vocabulary, one home.

## Resolutions

{{TBC}}
