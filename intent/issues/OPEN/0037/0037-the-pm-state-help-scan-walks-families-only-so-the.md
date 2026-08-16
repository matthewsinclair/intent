---
id: "0037"
title: the PM-state help scan walks families only, so the eight new_surface commands are never checked -- and it scans five retired paths that do not ship, so the same enumerator is both too narrow and too wide
date: 2026-08-16
reporter: matts
status: OPEN
severity: medium
---

# 0037: the PM-state help scan walks families only, so the eight new_surface commands are never checked -- and it scans five retired paths that do not ship, so the same enumerator is both too narrow and too wide

## Tags

acceptance, surface, parity, no-silent-errors, coverage, measured

## Summary

`no_pm_state_in_output.rs::declared_paths()` builds the command list for AC-00.9's help-text surface by walking `table["families"][].entries[]` and nothing else. **The dispatch table holds rows in two places**: 104 in `families`, and 8 more in a top-level `new_surface` array. The eight are `search`, `sync`, `schema`, `export`, `ingest`, `backup`, `daemon` and `mcp` -- **every one of them ships, every one carries help text, and none of them appears in `families`**, so none is ever scanned.

The same function includes five paths that do NOT ship (`st organize`, `organize`, `treeindex`, `help`, `st_zero`). **So one enumerator is simultaneously too narrow and too wide**, which is the live instance of ic's generalisation filed the same morning: the dispatch table is a parity register before it is a command list, a row means the question was ASKED rather than answered yes, and anything enumerating "all commands" wants `is_shipped()`.

Found by vc sweeping the contract for declared-set-where-shipped-set-is-meant, at ic's request, hours after using this same test to satisfy AC-00.9.

## Reproduction

Measured 2026-08-16 at `7b4096be`.

**The enumerator**, `native/rust/crates/intent-cli/tests/no_pm_state_in_output.rs:196-214`:

```rust
fn declared_paths() -> Vec<String> {
  ...
  for family in table["families"].as_array().into_iter().flatten() {
    for entry in family["entries"].as_array().into_iter().flatten() {
      if let Some(p) = entry["path"].as_str() {
        paths.push(p.to_string());
      }
    }
  }
  assert!(
    paths.len() > 20,
    "precondition: the dispatch table declares the command surface, got {} paths",
    paths.len()
  );
  paths
}
```

**The two row homes:**

```
$ jq -r '"families entries: \([.families[].entries[]]|length)"' surface/dispatch-table.json
families entries: 104
$ jq -r '"new_surface array: \((.new_surface // [])|length)"' surface/dispatch-table.json
new_surface array: 8
```

**And they do not overlap** -- the eight are reachable only through the array:

```
$ jq -r '[(.new_surface // [])[].path] as $ns | [.families[].entries[].path] as $fam
         | ($ns - ($ns - $fam)) | if length==0 then "NO OVERLAP" else "overlap: \(.)" end' \
    surface/dispatch-table.json
NO OVERLAP -- the 8 are reachable ONLY via new_surface
```

**The `> 20` precondition cannot notice.** It passes comfortably at 104, so the guard against an empty list is green while a twelfth of the surface is missing. A precondition tuned to catch "the file did not parse" is not a coverage assertion, and reads like one.

## Root Cause

**The table grew a second row home and the reader did not.** `new_surface` exists because those eight commands have no v2 antecedent, so they have no place in a families structure organised by the v2 surface. That is a correct modelling decision for a parity register. It becomes a hazard the moment a consumer treats the register as a command list, because the second home is invisible to anyone who has only ever seen the first.

The too-wide half has a different cause and is much milder: five retired paths ARE in `families`, correctly, because a retired command is a row whose question was asked and answered no. Including them costs a few wasted invocations of commands the v3 binary does not have.

**The two halves share one root: `path` is being read as "a command" when the row means "a command we made a decision about".**

## Impact

**Not realised. Measured, not assumed.** All eight commands' help text was scanned by hand at `7b4096be` for Intent thread ids, WP numbers, AC/AT ids and decision numbers: **all eight clean.** The gap exists and nothing has walked through it.

**The reason it is still worth filing is what it does to the criterion rather than what it has done:**

- **AC-00.9 is currently SATISFIED and rests partly on this check.** The criterion says no PM state reaches a user-facing surface; the instrument establishes that for 104 of 112 commands. The eight it misses are the newest commands in the tree -- the ones whose help strings are being written this week -- so the uncovered set is exactly the set most likely to acquire a leak.
- **A false green here is silent by construction.** The test passes, the AC reads satisfied, the WP counts it, and nothing anywhere reports that the list was short.
- **The too-wide half is self-limiting** and is noted for completeness rather than urgency: invoking a retired path yields an error whose text is then scanned, which is harmless and arguably useful.

**Deliberately not claimed: that this makes AC-00.9's satisfaction wrong.** The other surfaces in that file -- the string-literal scan across all three shipped crates, the schema faces, the renderer-reachability guard -- are not enumerated from `families` and do not share this gap. The help surface alone is short.

## Proposed Fix

1. **`declared_paths()` reads both homes and filters to the shipped set.** Walk `families[].entries[]` and `new_surface[]`, then drop any row where `disposition == "retire"` or `target.state == "retire"` -- the same predicate `Entry::is_shipped()` already applies in `dispatch.rs`, so this is reusing a decision rather than making a second one.
2. **Replace the `> 20` precondition with one that can actually fail.** Assert the scanned count equals the shipped-row count computed from the table, so the list going short is an error rather than a smaller number. A precondition that passes at 104 and at 112 alike is not measuring the thing its message claims.
3. **The general form, which is ic's ask and larger than this issue**: any consumer that enumerates "all commands" states which set it means and gets it from one place. Two row homes plus a shipped/declared distinction is four ways to be wrong, and each consumer is currently deciding independently.

Canary it: add a ninth row to `new_surface` carrying a WP id in its help, and watch the test go red. **A coverage fix that cannot be shown to cover is the same class of defect as the one it repairs.**

## Related

- ST0056 / AC-00.9 -- the criterion this instrument serves; satisfied 2026-08-16 on this test, with this gap open
- ic's finding the same morning -- `agent-guide.spec.md` mandated completeness over "every declared row", which required a guide containing `intent st_zero`, a command hv killed. Same root, opposite direction, and the reason this sweep was run
- `guide_refs_check.sh` (fixed at `be5d4b83`) -- the check whose error message asserted a capability it lacked; the measurement rule that came out of it applies here too, since `> 20` reads as a coverage assertion and is not one
- 0035 -- same Impact shape: a real hole, nobody through it, filed on what it does to the artefact rather than on realised harm
- `IN-AG-NO-SILENT-001` -- a short list that reports success is a failure that does not surface

## Resolutions

{{TBC}}
