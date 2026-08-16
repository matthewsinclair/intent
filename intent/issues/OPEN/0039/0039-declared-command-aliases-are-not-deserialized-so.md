---
id: "0039"
title: declared command aliases are not deserialized, so four keep-classified aliases do not exist in the binary -- at done and at notdone are live parity breaks and no instrument covers the field
date: 2026-08-16
reporter: matts
status: OPEN
severity: medium
---

# 0039: declared command aliases are not deserialized, so four keep-classified aliases do not exist in the binary -- at done and at notdone are live parity breaks and no instrument covers the field

## Tags

surface, parity, clap, no-silent-errors, deserialization, measured

## Summary

`surface/dispatch-table.json` declares `aliases` on five entries. **`pub struct Entry` does not have the field**, so it is not unread -- it is structurally invisible, exactly as `required` was on `Flag` in issue 0035. Four of the five are `disposition: keep`, meaning the v2 spelling is supposed to survive into v3. **It does not.**

Two of the four are on families that are already wired, and both are confirmed absent from the binary: **`at done` and `at notdone`**, which v2 documents in its own help as _"Aliases for green | red"_. The other two (`issues new`, `lang rm`) sit on families that are still dark and will inherit the same gap the moment they are wired.

**Neither instrument covers the field.** `parity/tools/surface_check.sh` -- the tool whose whole job is checking the binary against the table -- contains zero occurrences of `aliases`, and so does `crates/intent-cli/tests/dispatch_ssot.rs`. So the table declares four commands that do not exist, and every check reports agreement.

Found by vc sweeping for the declared-but-not-deserialized class after meeting it a fourth time in three files.

## Reproduction

Measured 2026-08-16 at `20a09731`, against a debug binary built from the current tree.

**What the table declares:**

```
$ jq -r '.families[].entries[] | select(.aliases != null and (.aliases|length)>0)
         | "  \(.path)  aliases=\(.aliases|join(","))  disposition=\(.disposition)"' \
    surface/dispatch-table.json
  st organize  aliases=st organise  disposition=retire
  at green     aliases=at done      disposition=keep
  at red       aliases=at notdone   disposition=keep
  issues add   aliases=issues new   disposition=keep
  lang remove  aliases=lang rm      disposition=keep
```

**What the type reads.** `pub struct Entry` (`crates/intent-cli/src/dispatch.rs:131-164`) deserializes eight fields: `path`, `help`, `args`, `flags`, `v2`, `target`, `disposition`, `owner_wp`. **`aliases` is not among them, and nothing anywhere in `crates/*/src/` mentions it.**

**What the binary does.** Run against a nonexistent thread so nothing can mutate:

```
at green     -> error: this project has not been migrated to Intent v3 ...
at done      -> error: unrecognized subcommand 'done'
at red       -> error: this project has not been migrated to Intent v3 ...
at notdone   -> error: unrecognized subcommand 'notdone'
```

The canonical spellings reach the migration check, so the family is wired. **The declared aliases do not exist.**

**And v2 has them:**

```
$ intent at
  done|notdone <stid> <atid>                 Aliases for green | red
```

**What the instruments say:**

```
$ grep -c 'aliases' intent/st/ST0056/parity/tools/surface_check.sh
0
$ grep -c 'aliases' native/rust/crates/intent-cli/tests/dispatch_ssot.rs
0
```

## Root Cause

**The same shape as 0035 Layer 1 and EXP-07, for the fourth time in three files.** A field is authored in the canon, no Rust type declares it, serde drops it silently, and the declaration reads as covered because a JSON file cannot say whether anyone is listening.

The instrument gap has a separate cause worth naming on its own. `surface_check.sh` compares the binary against the table **field by field, over the fields it knows about** -- paths, flags, help. **An unknown field is not a mismatch, it is invisible**, so adding a field to the canon silently adds an unchecked field rather than a failing one. The check cannot report on a key it was never told exists.

## Impact

**Realised, small, and user-visible -- which distinguishes it from 0035, where the hole existed and nobody had walked through it.** `at done` and `at notdone` are gone today. They are not obscure: they are the spellings this thread's own author reaches for, and the reason they exist in v2 is that `green`/`red` describe the row's state while `done`/`notdone` describe what the user did.

- **It is a silent parity break in a `keep` row**, which is the one classification that promises the v2 spelling survives. A `keep` row that does not ship is worse than a `retire` row, because `retire` is a decision with a ratification and this is an accident with neither.
- **The instrument that exists to catch exactly this reports agreement**, so nothing escalates and nothing will.
- **It scales with the port rather than staying at two.** `issues new` and `lang rm` are correct in the table today and will be absent the moment `issues` and `lang` are wired -- so the defect count grows as the surface is built, and each new instance arrives already reported green.

**Not claimed: that any AC currently rests on aliases.** No criterion mentions them; that is part of the finding rather than a mitigation.

## Proposed Fix

1. **`Entry` grows `aliases: Vec<String>`** and the spine registers each as a clap alias on the command it belongs to -- `Command::visible_alias`, so it shows in help the way v2's does. **Register only for rows that ship**, using the same `is_shipped()` the spine already applies, or `st organise` comes back with `st organize`.
2. **`surface_check.sh` checks the field**, and the general form matters more than this instance: **the check should refuse a canon key it does not know about**, rather than ignoring it. An unknown key is either a field someone forgot to wire or a field someone forgot to check, and both want a human. That single change closes this whole class prospectively instead of one field at a time.
3. **A test in `dispatch_ssot.rs`**: every declared alias on a shipped row resolves to the same command as its canonical spelling. Canary it by removing one alias registration and watching it go red.

**The wider recommendation, which is the reason this issue is worth more than its two commands:** this is the fourth declared-but-not-deserialized field found in three files (`Flag.required`/`accepts`/`default`/`value`, `Entry.exposed_on_mcp`, `Entry.read_or_mutate`, and now `Entry.aliases`). **Four separate fixes have been proposed and none of them closes the class.** A single check comparing the canon's authored key set against the types' deserialized key set, refusing on any key no type reads, would have caught all four before any of them shipped.

## Related

- 0035 -- same shape, one level in: `required: true` declared on `Flag` and structurally invisible. Its Layer 1 was fixed by adding the fields; the class was not
- EXP-07 (ic) -- `Flag` deserialized 3 of 8 authored fields; the wider finding this is another instance of
- AC-09.1 -- declares `exposed_on_mcp` and `read_or_mutate`, both undeserialized today (ic, 2026-08-16), so the agent guide's two gating fields are in the same state
- AC-06.8 -- a declared value nothing renders is the defect that AC exists to prevent; this is its input-side twin
- `IN-AG-NO-SILENT-001` -- a canon that declares four commands the binary does not have, while every check reports agreement

## Resolutions

{{TBC}}
