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

### A FOURTH INSTANCE, 2026-08-17, AND IT IS MINE -- the author of this issue walked into it the next day

**I filed this on 2026-08-16 and reproduced it on 2026-08-17 in a different tool**, hand-writing `jq -r '.families[].entries[].path'` for the 0044 exit-code sweep and describing the result as "the whole declared surface". Found by ic, measured rather than read: **104 probed, 107 shipped, 112 declared -- too narrow by the 8 `new_surface[]` rows and too wide by the 5 retired ones, three apart with opposite signs.**

**That the person carrying this issue in working memory rewrote its exact defect the next day settles what kind of problem it is.** It is not a knowledge problem and it is not an attention problem: **the wrong enumerator is the SHORT one, the natural one, and the one that returns a plausible number.** Nothing about `.families[].entries[]` looks partial, and 104 looks exactly as much like a command surface as 107 does.

**The unprobed rows were disproportionately the ones the sweep was about** -- four of the eight are `one-way` mutations, and `daemon` / `mcp` are the likeliest rows in the surface to spend an exit code on something structural. **A too-narrow enumerator does not remove a random sample; it removes whatever was added last, which is whatever is newest and least tested.**

### The shell-side mechanism now exists: `parity/tools/lib_surface.sh`

ic's suggestion, taken. A sourced-only library with one home for all four populations -- **declared 112, shipped 107, retired 5, probeable 104** -- registered in `MODULES.md` before it was written. `surface_shipped` is the default for any question about what the tool does; `surface_retired` exists on its own because a retired command is not absent from the WORLD (v2 users still type it), so it is the population for "what happens when someone runs the old command". It refuses rather than defaulting when the table cannot be located, on the same grounds as `probe.sh`: a population computed from a table that is not there is a complete, uniform, entirely fictional surface.

**It also carries the exclusion list `implemented_check.sh` had been re-deriving** -- `daemon`, `mcp`, `claude start`, the three that do not return -- **newline-delimited, and that is not a style choice: `claude start` contains a space, so a space-separated list word-splits into two commands that are not in it.** A path with a space is the normal case in this table.

**This does NOT close the issue.** The instance filed here is in Rust (`no_pm_state_in_output.rs`), and the fix there is Proposed Fix 1 -- `Entry::is_shipped()`, which `dispatch.rs` already applies, so it is reusing a decision rather than making a second one. **A shell library and a Rust predicate encoding the same four populations is itself the Highlander question this issue is about**, and whoever does the Rust side should decide whether the table grows a generated manifest both read, rather than adding a third hand-rolled walk.
