---
id: "0040"
title: config.json declares st_prefix and v2 honours it in six places, but v3 reads the field nowhere -- the id allocator and the legacy scanner both hardcode ST
date: 2026-08-16
reporter: matts
status: OPEN
severity: medium
---

# 0040: config.json declares st_prefix and v2 honours it in six places, but v3 reads the field nowhere -- the id allocator and the legacy scanner both hardcode ST

## Tags

parity, config, highlander, declared-but-unread, migration, measured

## Summary

`Config.st_prefix` is a real deserialized field with a serde default of `"ST"`. **It occurs three times in the entire Rust workspace and all three are its own declaration.** No code reads it, no test asserts it.

Meanwhile the two places that would need it hardcode the value instead: `facade.rs:1891-1895` allocates the next thread id with `format!("ST{:04}", ...)` after `strip_prefix("ST")`, and `legacy.rs:198` recognises a v2 thread directory with `starts_with("ST")`.

**v2 honours the field properly**, which is what makes this a parity break rather than dead config. `bin/intent_st:75` reads it (`ST_PREFIX=$(get_config_field "st_prefix" "ST")`) and it reaches the directory glob (`:196`), the id parse (`:199`, `:213`), the file glob (`:211`) and the allocator (`:228`, `printf "%s%04d"`). `bin/intent_init:120` writes `"st_prefix": "ST"` into every project v2 creates, so **every project in the estate carries this field**.

Found by vc during the hv-assigned Highlander review of the v3 Rust tree, 2026-08-16.

## Reproduction

Measured at `ff094157` against a pristine `git archive HEAD` extract, so no peer's uncommitted edit is in the reading.

**Every occurrence in the workspace:**

```
$ grep -rn 'st_prefix' --include='*.rs' --include='*.json' --include='*.toml' native/rust/
crates/intentsvcs/src/project.rs:34:  #[serde(default = "default_st_prefix")]
crates/intentsvcs/src/project.rs:35:  pub st_prefix: String,
crates/intentsvcs/src/project.rs:48:fn default_st_prefix() -> String {
```

Three hits: the attribute, the field, the default function. **Zero reads. Zero tests.**

**What hardcodes it instead:**

```
crates/intentsvcs/src/facade.rs:1891:      .filter_map(|t| t.id.strip_prefix("ST"))
crates/intentsvcs/src/facade.rs:1895:    format!("ST{:04}", highest + 1)
crates/intentsvcs/src/legacy.rs:198:  name.len() == 6 && name.starts_with("ST") && name[2..].bytes().all(|b| b.is_ascii_digit())
```

Note `legacy.rs:198` also hardcodes the **length** (`name.len() == 6`), so it encodes "two-character prefix plus four digits" in a second, independent way.

**What v2 does with the same field:**

```
$ grep -rn 'st_prefix' bin/
bin/intent_st:75:  ST_PREFIX=$(get_config_field "st_prefix" "ST")
bin/intent_st:189:  local st_prefix="${ST_PREFIX:-ST}"
bin/intent_st:196:    for dir in $(find "$base_dir" -type d -name "$st_prefix[0-9][0-9][0-9][0-9]" ...)
bin/intent_st:199:      id_str=${id_str#$st_prefix}
bin/intent_st:211:    for file in $(find "$base_dir" -type f -name "$st_prefix[0-9][0-9][0-9][0-9].md" ...)
bin/intent_st:213:      id_str=${id_str#$st_prefix}
bin/intent_st:228:  printf "%s%04d" "$st_prefix" $next_id
bin/intent_init:120:  "st_prefix": "ST",
```

**Confirmed by running, not by reading.** Four fixtures from one generator, differing only in the thread directory's name and the `st_prefix` value, each a committed git repo at `intent_version: 2.19.0` with a single WIP thread. Run against the debug binary (`legacy.rs` is clean at HEAD, so this path is HEAD's):

| fixture | directory | `st_prefix` | threads read | residue | exit | output                   |
| ------- | --------- | ----------- | ------------ | ------- | ---- | ------------------------ |
| st      | `ST0001`  | `ST`        | **1**        | 0       | 0    | `ok: this estate parses` |
| th      | `TH0001`  | `TH`        | **0**        | 0       | 0    | `ok: this estate parses` |
| mix1    | `ST0001`  | `TH`        | **1**        | 0       | 0    | `ok: this estate parses` |
| mix2    | `TH0001`  | `ST`        | **0**        | 0       | 0    | `ok: this estate parses` |

Two things the crossed arms establish that the straight pair could not. **The config field has no effect in either direction**: setting `st_prefix: TH` does not stop an `ST` directory being read (mix1), and setting `st_prefix: ST` does not rescue a `TH` directory (mix2). The directory name alone decides, exactly as `legacy.rs:198` says.

**And the invisible cases do not fail -- they SUCCEED.** `read: 0 thread(s)`, `residue: 0 blocking, 0 carried`, `ok: this estate parses`, **exit 0**. The operator is told their estate is clean by a tool that could not see any of it.

## Root Cause

**The same declared-but-unhonoured class as 0035 / 0039 / EXP-07, arriving through the opposite mechanism, which is why no instrument built for those would find it.**

In 0039 the field is absent from the Rust type, so serde drops it and the declaration is structurally invisible. Here the field **is** declared, **is** deserialized, and round-trips correctly -- it simply has no consumer. dc's proposed `#[serde(flatten)] rest: BTreeMap` check closes the first mechanism and **cannot see this one**, because `st_prefix` never lands in `rest`.

Rust's own `dead_code` lint does not fire either: the field is `pub` on a `pub struct` in a library crate, so it is reachable by definition and the compiler is right not to warn.

**The discriminator that separates a defect from a decision here is not mechanical, it is "does a consumer exist and hardcode the value instead".** Three of `Config`'s seven fields have zero read sites and only this one is a defect:

| field       | read sites | verdict                                                                                              |
| ----------- | ---------- | ---------------------------------------------------------------------------------------------------- |
| `st_prefix` | 0          | **DEFECT** -- the consumers exist (`facade.rs`, `legacy.rs`) and hardcode the value                  |
| `author`    | 0          | **carried deliberately** -- D02 removed the verblock, so the consumer is gone by ruling; round-trips |
| `languages` | 0          | **pending** -- its families (`lang`, `critic`, `agents`) are not wired yet                           |

Recording the table because the classification is the reusable part: a declared-and-unread field is a defect when the code that should read it exists and encodes the value another way; it is pending when the consumer is unbuilt, and correct when the consumer was removed by a ruling.

## Impact

**A project configured with any prefix other than `ST` is read as an empty estate, and told so with an `ok:` and exit 0.**

**The failure is not that the migration breaks -- it is that the migration SUCCEEDS on nothing and says so.** That is the worst available shape and it is the one measured above. A refusal would be safe: the operator fixes the estate and re-runs. A green `ok: this estate parses` over an unread estate is an instruction to proceed, and exit 0 means an automated pipeline does.

- **AC-00.2 / AC-10.5 are breached silently.** Artefact conservation says every v2 artefact is converted or named in the residue by class. An unrecognised thread directory is **neither** -- it is not converted, and `residue: 0 blocking, 0 carried` positively asserts there is nothing to name.
- **`facade.rs:1895` then allocates `ST0001`** into a project whose every existing thread carries a different prefix, so the first post-migration `st new` collides the two vocabularies.
- **The field survives into the v3 config carrying a value nothing honours**, so the file keeps claiming a setting that stopped working.
- **`IN-AG-NO-SILENT-001`** in its purest form: the one code path that exists to report what could not be read reports that everything was read.

**Filed at `high`, lowered to `medium` by the survey below, and the reasoning is recorded rather than quietly adjusted.**

I named the fleet survey as the thing that would settle the severity, then ran it. **Every project surveyed uses `ST`:**

```
Anvil ST · Baize ST · Cdsync ST · Conflab ST · Courses ST · Devbin ST · Intent ST
Laksa (absent -- defaults to ST) · Lamplight ST · MicroGPTEx ST · Molt ST
Molt-flynn ST · Molt-matts ST · Prolix ST · Riffle ST · Utilz ST
```

**So the entire migration corpus named in `migration.md` -- Intent, Lamplight, Utilz, Baize -- is unaffected, and no migration we are actually about to run can hit this.** Intent's own dogfood migration is safe.

**Why it stays open at `medium` rather than closing.** The breach is of the migrator's contract, not of a project: AC-00.2 / AC-10.5 promise conservation-or-residue and this path delivers neither, with no instrument able to see it. v2 documented and honoured the setting, so a user outside this fleet may hold one, and v3 ships publicly. **What the survey removes is urgency, not the defect** -- which is the distinction between severity and correctness, and the reason to run the survey before ranking rather than after.

**Ranking it `high` beside 0038 would have been the real harm.** 0038 blocks every commit in a migrated project; this affects zero known projects. Two `high` rows where one is unreachable is how the reachable one loses its place in the queue.

**Also not claimed: that the fix is to honour it.** Retiring the field is a legitimate answer -- v3 may reasonably decide the prefix is fixed. That is a ratification with a `disposition: retire` row and a migration note, which is a decision; what exists today is neither.

## Resolution ruled

**hv ruled 2026-08-16: RETIRE. The prefix is fixed at `ST` and the knob is deleted.** (Under hv's standing "go with your recs"; vc's recommendation was retire, on the grounds below.)

**And retiring turns out not to be a change of direction, which is the finding that should have been in this issue from the start: `st_prefix` appears in NO ST0056 spec.** `data-model.md`'s project-config table lists six fields and never included it. **The design had already dropped the knob and nobody propagated that to the type**, so the field was residue of a decision already taken rather than a feature awaiting wiring. That reframes the whole issue: the code was not ahead of the design, it was behind it, and nothing compared them.

**What retirement obliges, both routed to cc:**

1. **`Config.st_prefix` and `default_st_prefix()` are deleted** from `project.rs`. Ratified in `data-model.md` under "`st_prefix` -- RETIRED".
2. **The migrator NAMES the field when a project carries a non-`ST` value.** This is the load-bearing half: retiring a knob nobody uses is fine, retiring it under someone who does use it without telling them is the silent data change this whole thread exists to prevent. **No fleet member is affected -- all 16 use `ST` -- so this costs nothing today and is the only thing that makes the decision safe for anyone outside the fleet.**
3. **`legacy.rs:198` loses its hardcoded `name.len() == 6`** in the same change. With the prefix fixed, the length is derivable rather than asserted, and leaving it is how the next reader finds two independent encodings of one fact.

**No parity-register row is owed.** The register is file- and command-level; this is a config field with no command surface, and nothing in the dispatch table references it.

## Proposed Fix (superseded by the ruling above, kept for the reasoning)

Two routes, and the choice belongs to hv because it is a scope call, not a code call.

1. **Honour it.** `facade.rs`'s allocator and `legacy.rs`'s recogniser both read `project.config().st_prefix`. `legacy.rs:198` loses its hardcoded `len() == 6` in favour of `prefix.len() + 4`. One test with a non-default prefix, which is also the canary.
2. **Retire it.** A parity-register row with `disposition: retire` and a ratification, `default_st_prefix` deleted, and the migrator names the field when it carries a non-`ST` value forward so an affected project is told rather than silently changed.

**Whichever route: the canary is a project whose config sets a non-default prefix.** There is no such fixture today, which is why nothing caught this.

## Related

- 0039 -- the same class through the other mechanism (declared, no Rust field, serde drops it); dc's `rest: BTreeMap` fix closes that one and is blind to this one
- 0035 / EXP-07 -- earlier instances of declared-but-unhonoured in the dispatch register
- 0041 -- the other Highlander finding from the same review; a vocabulary spelled twice rather than a setting read never
- AC-00.2 / AC-10.5 -- artefact conservation, which an unrecognised thread directory breaks silently
- `IN-AG-HIGHLANDER-001` -- the prefix is one fact and the tree holds three encodings of it: the config default, the allocator's literal, and the scanner's literal-plus-length

## Resolutions

{{TBC}}
