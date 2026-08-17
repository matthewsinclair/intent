---
id: "0035"
title: "ac satisfy accepts an empty --evidence at every layer: the declaration is structurally invisible, the renderer defaults it, and the facade stores it, so a non-test AC can be satisfied with no citation"
date: 2026-08-15
reporter: matts
status: CLOSED
severity: high
---

# 0035: ac satisfy accepts an empty --evidence at every layer: the declaration is structurally invisible, the renderer defaults it, and the facade stores it, so a non-test AC can be satisfied with no citation

## Tags

acceptance, contract, surface, clap, no-silent-errors, measured

## Summary

**The whole discriminator between a test AC and a non-test AC is how it is proven.** A test AC is satisfied by its covering acceptance test going green; **a non-test AC is satisfied by NAMED EVIDENCE, and nothing else.** That is the contract `intent ac` exists to enforce.

`--evidence` is declared `"required": true` in the canon. **It is not required anywhere that runs.** Three layers each independently decline to enforce it, in both the v2 shell and the v3 Rust path, so a non-test criterion can be moved to Satisfied with an empty citation and the command reports `ok:`.

Found by ic while classifying flags for EXP-05 -- by reading `dispatch.rs` against the renderer, not by reasoning about either.

## Reproduction

Measured 2026-08-15 at `7d4eb0f1`.

**The declaration, `surface/dispatch-table.json` (`/families/2/entries/3/flags/0`):**

```json
{
  "spellings": ["--evidence"],
  "type": "string",
  "help": "The named evidence reference",
  "value": "<ref>",
  "required": true,
  "disposition": "keep"
}
```

**Layer 1 -- the declaration cannot reach the binary.** `pub struct Flag` has THREE fields (`spellings`, `kind`, `help`). The canon authors eight. `accepts`, `default`, `required` and `value` **do not deserialize at all** -- they are not unread, they are structurally invisible. `required: true` is a string in a file that no type can represent.

**Layer 2 -- the v3 renderer defaults it away**, `native/rust/crates/intent-cli/src/render.rs:669-671`:

```rust
Some(("satisfy", a)) => {
  let st = arg(a, "stid")?;
  let id = arg(a, "acid")?;
  let evidence = arg(a, "evidence").unwrap_or_default();
```

**Both required positionals use `arg(a, ..)?`, which propagates. `evidence` alone uses `unwrap_or_default()`, which turns absent into `""`.** The two spellings sit three lines apart.

**Layer 3 -- the facade stores it**, `native/rust/crates/intentsvcs/src/facade.rs:1127-1147`. `ac_satisfy` checks that the criterion is non-test and that it is on-scope, then writes `AcState::Satisfied { evidence: evidence.to_string() }`. **There is no non-empty check.** An empty string is a valid satisfaction.

**And v2 -- the path actually maintaining ST0056 today -- has the same hole**, `bin/intent_acceptance:1056-1067`:

```sh
local ref=""
while [ $# -gt 0 ]; do
  case "$1" in
    --evidence)   ref="$2"; shift 2 ;;
    --evidence=*) ref="${1#--evidence=}"; shift ;;
    *)            shift ;;
  done
[ -n "$stid" ] && [ -n "$acid" ] || usage
```

`ref` is initialised empty, the flag is optional in the loop, and **the guard on the next line checks `stid` and `acid` and never checks `ref`.** The `*) shift` arm also swallows a mistyped `--evidance` in silence, so a typo satisfies the criterion with no citation and no warning.

## Root Cause

**Three layers, each of which would be sufficient on its own, and each of which assumes one of the others is doing it.**

The declaration is the interesting one, because it is not a bug in any single place: someone wrote `required: true` in the canon and it is TRUE as a statement of intent. It simply describes a field the type layer does not have, so it documents a constraint rather than imposing one. **A declaration that cannot be deserialized is indistinguishable, in the file, from one that is enforced** -- which is the same shape as an advertised flag no renderer reads, one level further out.

`unwrap_or_default()` on a `String` is the specific hazard: it does not read as a decision. Three lines of near-identical code where two propagate and one defaults is not a thing a reviewer's eye catches, and there is no compiler complaint because both spellings type-check.

## Impact

**Not realised. Measured, not assumed.** All 22 satisfied non-test ACs in ST0056 carry an evidence field -- swept at `7d4eb0f1`, zero evidence-free rows. The hole exists and nobody has walked through it, because every satisfaction so far passed `--evidence` by habit.

**The reason it is still HIGH is what it does to the artefact rather than what it has done:**

- **It defeats the one distinction the AC/AT machine is built on.** With empty evidence permitted, a non-test AC is satisfiable by assertion, which makes it a test AC with extra steps and no test. **The gate then counts it toward a green.**
- **It fails silently and prints success.** `ok: <id> satisfied`, exit 0, and the row still lints clean.
- **It is invisible in review afterwards.** A row satisfied with no citation looks like a row whose author had nothing to cite. **The absence of evidence and the absence of a REQUIREMENT for evidence are indistinguishable in the finished artefact.**
- **The typo arm makes it reachable by accident, not just by intent.** `--evidance` is swallowed by `*) shift`, so a careful author who meant to cite gets the same outcome as one who did not.

## Proposed Fix

**Refuse at the layer that cannot be bypassed, and refuse in all three anyway** -- they are cheap and they fail differently.

1. **The facade refuses.** `ac_satisfy` returns an error on empty-or-whitespace evidence. This is the load-bearing one: it is the typed API, it is the only door under D01, and it covers every caller including the daemon and any future surface. **A non-test criterion satisfied without a citation is not a thing the model should be able to represent.**
2. **The renderer stops defaulting.** `arg(a, "evidence")?` -- the same spelling as the two lines above it. Nothing else in that match arm needs to change.
3. **The declaration becomes enforceable, or stops claiming.** Either `Flag` grows the four missing fields so `required: true` reaches clap (EXP-07's scope, cc's half), or the canon's `required` is removed from rows it cannot enforce. **What must not persist is a file asserting a constraint that no layer applies** -- that is worse than not declaring it, because it reads as covered.
4. **v2: check `ref` beside `stid` and `acid`**, and replace the `*) shift` arm with a refusal on an unrecognised argument so a typo is an error rather than a silent empty.

Add a guard asserting `ac satisfy` on a non-test AC with no `--evidence` exits non-zero and leaves the row unsatisfied -- and canary it by removing the check and watching the guard go red, because a guard against a silent success is exactly the kind that passes vacuously.

**Not proposed: back-filling or auditing existing rows.** The sweep above found none to fix.

## Related

- ST0056 -- found during EXP-05 flag classification (ic); the evidence contract is AC-00.x's and vc stewards it
- EXP-07 (ic's register) -- the wider finding this is the live instance of: `Flag` deserializes 3 of 8 authored fields, `accepts` domains are unvalidated (`--severity-min banana` parses), and `spine.rs:152-159` silently drops any flag with no long spelling (four instances of `IN-AG-NO-SILENT-001`)
- 0033 -- same file in v2, same family: the AT row's note is read and then discarded by a write path that had it in hand
- `IN-AG-NO-SILENT-001` -- every failure surfaces; this is a missing-input failure that reports success

## Resolutions

### v3 FIXED AND VERIFIED BY vc, 2026-08-16, WITH THE CANARY THIS ISSUE ASKED FOR. v2 STILL OPEN AND STILL THE PATH IN USE.

Verified at a clean tree (`bf27b804`, worktree identical to HEAD), layer by layer, against the fix rather than against the account of it.

**Layer 1 -- FIXED.** `pub struct Flag` now deserializes `value`, `required` and `default` beside the original three (cc, `7e051f3f`). `accepts` is deliberately still absent and cc recorded why on the type: its four rows are prose in four different shapes, and a `value_parser` built from the two that look like enums would refuse input the other two describe as valid. **That is a declaration REMOVED from the canon's claim rather than left asserting something no layer applies**, which is what this issue asked for -- either enforce it or stop claiming it.

**Layer 2 -- DELIBERATELY NOT CHANGED, and cc's reasoning is better than the fix I proposed.** `render.rs:704` still reads `opt(a, "evidence").unwrap_or_default()`, with the comment that the facade guard _"refuses `--evidence \"\"` as well as an absent flag -- which re-checking the flag here could not do."_ **That is correct and my proposed fix #2 was strictly weaker.** A clap-level `required` catches an ABSENT flag only; `--evidence ""` is a supplied-but-empty value that sails past it. Adding the check would give two error paths for one user fault, with the weaker one firing first. **This issue said "refuse in all three anyway -- they are cheap and they fail differently". They do not fail differently here: the renderer's version can only ever catch a subset of what the facade already catches.**

**Layer 3 -- FIXED, and this is the load-bearing one.** `Guard::EvidenceRecorded` is declared on the `ac.satisfy` edge (`transitions.rs:377`) and enforced in `check_ac_guards` (`facade.rs:1005-1031`), which **trims** the justification before testing it, so whitespace-only evidence refuses too. `ac_satisfy` reaches it through `set_ac_state`, so no verb can bypass it.

**THE CANARY, run because this issue demanded it: _"canary it by removing the check and watching the guard go red, because a guard against a silent success is exactly the kind that passes vacuously."_** In a sacrificial worktree, `Guard::EvidenceRecorded` was stripped from the `ac.satisfy` edge and the suite re-run:

```
test satisfying_a_criterion_with_no_evidence_is_refused ... FAILED
  panicked at facade_acceptance.rs:360: evidence "" was accepted as evidence
test result: FAILED. 13 passed; 1 failed
```

**The guard is falsifiable.** Both doors are covered and both are tested: `facade_acceptance.rs:350` (the API) and `ingest_refusal.rs:297` (the schema face, via `minLength`), 24 tests green at HEAD.

**Layer 4 -- v2 IS UNCHANGED AND THE HOLE IS LIVE.** `bin/intent_acceptance:1059-1067` still initialises `ref=""`, still guards only `stid` and `acid`, and still swallows a mistyped `--evidance` in the `*) shift` arm. **This is not an oversight -- v2 is under hv's DEFAULT-DEFER and WP-04 replaces it -- but it must be said plainly: `intent ac satisfy` is the v2 path, it is what this project is using to maintain ST0056 today, and every AC satisfied this session went through it.** The reason no evidence-free row exists is still habit, exactly as the Impact section said when this was filed.

**So the issue stays OPEN on the v2 layer alone.** The realised risk has not changed and the artefact-level risk is now closed on the side that ships.
