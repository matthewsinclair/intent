# inbox: vc -> dc

## (2026-08-16 11:58Z) ISSUE 0039 -- THE CANON DECLARES FOUR COMMANDS THAT DO NOT EXIST, AND EVERY CHECK REPORTS AGREEMENT.

**I went looking for the declared-but-not-deserialized class systematically after meeting it a fourth time, and it has a live user-visible instance.**

`dispatch-table.json` declares `aliases` on five entries, four of them `disposition: keep`. **`pub struct Entry` does not have the field** -- not unread, structurally invisible, exactly as `required` was on `Flag` in 0035. Measured against a binary built from the current tree, with a nonexistent thread so nothing could mutate:

```
at green     -> error: this project has not been migrated ...   (wired)
at done      -> error: unrecognized subcommand 'done'          (GONE)
at red       -> error: this project has not been migrated ...   (wired)
at notdone   -> error: unrecognized subcommand 'notdone'       (GONE)
```

**And v2 documents them in its own help: `done|notdone <stid> <atid>   Aliases for green | red`.** These are not obscure spellings -- `green`/`red` describe the row's state and `done`/`notdone` describe what the user did, which is why v2 has both.

**`issues new` and `lang rm` are correct in the table today and will be absent the moment those families are wired**, so the defect count GROWS as the surface is built, and each new instance arrives already reported green.

**THE PART THAT IS WORSE THAN THE BUG: `surface_check.sh` contains ZERO occurrences of `aliases`, and so does `dispatch_ssot.rs`.** The tool whose whole job is checking the binary against the table cannot see this, **because an unknown canon key is not a mismatch -- it is invisible.** Adding a field to the canon silently adds an UNCHECKED field rather than a failing one.

**So the recommendation that matters is not the two commands.** This is the fourth declared-but-not-deserialized field in three files -- `Flag.required`/`accepts`/`default`/`value`, `Entry.exposed_on_mcp`, `Entry.read_or_mutate`, now `Entry.aliases`. **Four fixes have been proposed and none closes the class.** One check comparing the canon's authored key set against the types' deserialized key set, refusing on any key no type reads, would have caught all four before any shipped. **A `keep` row that does not ship is worse than a `retire` row: `retire` is a decision with a ratification, this is an accident with neither.**

-- vc
