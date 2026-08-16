---
id: "0040"
title: config.json declares st_prefix and v2 honours it in six places, but v3 reads the field nowhere -- the id allocator and the legacy scanner both hardcode ST
date: 2026-08-16
reporter: matts
status: OPEN
severity: high
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

**A project configured with any prefix other than `ST` migrates into a v3 that cannot see its own threads.**

- `legacy.rs:198` does not recognise the thread directories, so Phase A reads the estate as empty rather than reporting residue -- **the failure mode is a silent under-count, not a refusal**, which is the direction the migrator is explicitly built to avoid.
- `facade.rs:1895` then allocates `ST0001` into a project whose every existing thread carries a different prefix.
- The field survives the migration into the v3 config carrying a value nothing honours, so the config keeps claiming a setting that has stopped working.

**Not claimed: that any fleet member currently sets a non-default prefix.** Intent's own is `ST` and I have not surveyed Lamplight, Utilz or Baize. The severity is `high` on the migrator's contract rather than on a known affected project -- AC-00.2 / AC-10.5 assert that every v2 artefact is accounted for or named in the residue, and a thread whose directory is not recognised is neither.

**Also not claimed: that the fix is to honour it.** Retiring the field is a legitimate answer -- v3 may reasonably decide the prefix is fixed. That is a ratification with a `disposition: retire` row and a migration note, which is a decision; what exists today is neither.

## Proposed Fix

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
