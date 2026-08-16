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

**Both halves have landed (cc, 2026-08-16): the instance, and the class the issue said was worth more than its two commands. One clause is outstanding and it is ic's.**

### The instance

`Entry` grows `aliases`, and the spine registers each as a **visible** clap alias -- v2 lists them (`done|notdone <stid> <atid>  Aliases for green | red`), and a hidden alias that works but cannot be discovered is a different way of not shipping it. Measured after the change:

```
at green     -> error: this project has not been migrated to Intent v3 ...
at done      -> error: this project has not been migrated to Intent v3 ...
at red       -> error: this project has not been migrated to Intent v3 ...
at notdone   -> error: this project has not been migrated to Intent v3 ...
st organise  -> error: unrecognized subcommand 'organise'
```

All four spellings now reach the same place, and the retired alias stays retired.

**Every `Entry`-backed `Command` is now built in one function.** The property had to hold at three construction sites -- the family, the leaf verb, and the new-surface top level -- and held at none of them, because there was no single expression of "a Command made from an Entry". `.about(entry.help)` was already repeated at all three; the aliases would have been the fourth thing to remember three times.

An alias is authored as a full path, so its prefix must equal the entry's own or it names a different command; `check_vocabularies` refuses a mismatch at load, beside the value checks it already made.

**Two tests, discovered from the table rather than listed, both canaried.** The first asserts EQUALITY OF BEHAVIOUR between the two spellings rather than presence in help text -- searching help for `done` passes on any command whose help contains the word. It needs no arguments to do it: with none, the canonical spelling reports its missing positionals and a spelling that does not exist reports `unrecognized subcommand`, so the two answers differ exactly when the alias is missing. The second pins the retired half, which a naive fix gets wrong. Both count what they checked, so an empty loop cannot pass.

### The class

`crates/intent-cli/tests/canon_keys_are_read.rs` asserts, in both directions, that **a key the canon classifies as driving behaviour is a key some Rust type reads** -- and that nothing is read which the canon has not classified at all.

**`deny_unknown_fields` is the obvious answer and it is the wrong one**, which is worth recording because the next person will reach for it. `dispatch.rs` carries an explicit ruling against it and the ruling is right: the table is a REGISTER, not canon the tool writes. `target` alone carries 44 authored keys against one field, and a strict type would stop the binary loading its own surface the first time someone documented a decision in it. **But that exemption is also exactly how five fields were lost**, because it makes an unread contract key indistinguishable from an unread note -- and nothing mechanical separates them, as dc measured.

So the split is authored, and ic authored it: `key_classes` at `fd961437`, scoped to `Entry`/`Flag`/`Arg` with `Table` and `Target` keeping their exemption. This test is the half that binds the declaration to the code. **Neither side is restated in the test** -- the canon says which keys must drive behaviour, and the types are asked what they read by SERIALIZING them, because a hand-kept list of field names would be a roster of the same kind that failed, and wrong in precisely the place the type was wrong.

**It found a sixth instance on its first run: `Arg.default`**, declared on 8 rows and absent from the struct. ic had flagged the trap in it the same day, and the measurement is why the field is deserialized and validated but **not rendered as a clap `default_value`**: seven of the eight are literals and `init` reads `"the current directory name"`, which is a description of a computation. Wiring it straight through would make `intent init` name a project `the current directory name` -- the confidently-wrong behaviour that having the field was supposed to prevent.

The discriminator is the arg's own `type`, not a list of exempt names: `enum` and `subcommand` have a CLOSED domain, so a default must name a member of it and `check_vocabularies` now checks that it does -- taking a subcommand slot's domain from its SIBLING VERBS where it declares no `values`, which is how the four rows spelling `default: "list"` with no values are legal. `string` has an open domain, so nothing can tell a value from a description of one, and it is left alone. The only row that is not a literal is the only `string` row.

`exposed_on_mcp` and `read_or_mutate` (AC-09.1's two gating fields, both on all 112 rows) are now deserialized too, and deliberately without `#[serde(default)]`: the two plausible defaults are both wrong to pick silently. For `read_or_mutate` especially -- it is the field an agent tier gates safety on, and an absent value defaulting to `read` would present an unclassified command as safe to call unattended.

### Outstanding

- ~~**Clause 2, `surface_check.sh` (ic).**~~ **RULED, 2026-08-16, and it closes with no work.** ic: `surface_check.sh` does NOT want its own copy, because **a shell script comparing JSON to a binary cannot ask serde what it deserialized** -- it can only search text, and a text search must know its needle, which is exactly why it was blind to `aliases` in the first place. **That is a limit of the mechanism, not an oversight in the script**, so the Rust guard is not merely the stronger form here, it is the only form that can exist. The general prescription in Proposed Fix item 2 -- "the check should refuse a canon key it does not know about" -- survives and lives in `canon_keys_are_read.rs`, where it can.
- **Not done, and named so it is not mistaken for done:** `default` is validated, not rendered. A row that declares one still gets no clap default. That is a deliberate stop, not an oversight, and it wants either a canon spelling that distinguishes a computed default from a literal one, or a decision that the four `subcommand` rows are wired and `init` is not.

  **STILL OPEN, re-measured 2026-08-16 -- and this issue does NOT close on the clause-2 ruling above.** Recorded because cc's own board had "0039 CAN BE CLOSED, ic ruled clause 2 with no work" queued as an action, and clause 2 was never the only outstanding item; the note above it has been here since the resolution was written. Measured rather than re-read: `Flag.default` DOES reach clap (`spine.rs:444`, guarded so it applies only where a default makes sense), `Arg.default` does not, and eight rows declare one. **A queued action whose premise was true when it was queued is not a queued action whose premise is true when it runs**, which is the whole of why it was checked before being taken.
