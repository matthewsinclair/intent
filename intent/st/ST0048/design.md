# Design - ST0048: Acceptance close-gate fails empty or missing contract

## Problem

`intent ac gate` is the single authority that `intent st done` / `intent wp done` consult before closing a unit (`bin/intent_acceptance:199`, `cmd_ac_gate`). "Done" is computed as "every in-scope AC is satisfied." When a unit has **zero in-scope acceptance criteria**, that predicate is **vacuously true**, so the gate opens and the unit closes with no verifiable definition of done.

Two code paths produce the vacuous pass:

- `bin/intent_acceptance:226` -- `[ "$total" -eq 0 ] && exit 0` (a present `acceptance.md` with no real `- AC-` lines in scope).
- `bin/intent_acceptance:207` -- `[ -f "$acc" ] || exit 0` (no `acceptance.md` at all).

A missing or empty acceptance contract is silently treated as a satisfied one -- the exact inverse of the contract's purpose. Reported from a downstream Intent project as a real, repeated hole: "work marked done with nothing to verify it against."

## Current behaviour (as-built)

`cmd_ac_gate` has these exits today (`bin/intent_acceptance:199-230`):

| Condition                                | Line    | Today         |
| ---------------------------------------- | ------- | ------------- |
| No thread dir                            | 205     | open (exit 0) |
| No `acceptance.md`                       | 207     | open (exit 0) |
| Present file, malformed AC/AT lines      | 208-211 | BLOCK (F1)    |
| Present file, **zero in-scope ACs**      | 226     | open (exit 0) |
| Present file, all in-scope ACs satisfied | 227     | open (exit 0) |
| Present file, an in-scope AC unsatisfied | 228-229 | BLOCK         |

The catch unique to Intent: **`intent st new` stamps `acceptance.md` unconditionally** -- it is the 5th default doc (`bin/intent_st:1207`), and the template ships only indented, inert examples (zero column-0 `- AC-` lines). So **every steel thread is born with an `acceptance.md` that has zero ACs**, which means file-presence is _not_ the opt-in boundary -- the real, current opt-in boundary is line 226's `total -eq 0` open. The behaviour is deliberate and documented:

- Gate header comment, `bin/intent_acceptance:195-198`: "zero in-scope ACs ... leave the gate open ... Old STs are never forced into a contract -- matts, 2026-06-16."
- Test `AT-04.4` (`tests/unit/acceptance_close_gate.bats:103-110`): a freshly-stamped ST with no real ACs **must close**.
- The no-file open is guarded by the test at `tests/unit/acceptance_close_gate.bats:145-151`.

So this is not a bug _inside_ the opt-in model -- it is a request to **replace opt-in-by-presence with mandatory-by-default + explicit exemption**. Ratified by hv (2026-06-29) as a shipped-as-broken fix.

## Approach

Make a unit with no verifiable contract **fail** the close-gate, with one explicit, visible escape hatch:

1. **Zero in-scope ACs (present file) -> BLOCK** (was line 226 open).
2. **No `acceptance.md` -> BLOCK** (was line 207 open). The escape is to create the file with real ACs or the exemption marker.
3. **Exemption** -- `acceptance: exempt` in the `acceptance.md` YAML frontmatter opens the gate deliberately and **visibly** (the gate prints an `EXEMPT` line; No-Silent-Errors -- an exemption is announced, never silent). Default template ships _without_ the marker, so **default = enforced**.
4. **Unchanged:** satisfied ACs pass; unsatisfied ACs block; malformed lines block (F1).

## Design Decisions

- **D1 -- Exemption marker = `acceptance: exempt` frontmatter (not a body marker).** Frontmatter is linter-stable (not reflowed), visible at the head of the file, and trivially greppable; a simple `sed`/`grep` over the `--- ... ---` block reads it, consistent with how `info.md` status is read. A body marker (`(no-ac)`) sits in linter-managed prose and is easier to miss. An optional reason may follow (`acceptance: exempt -- pure authorial content`); the gate echoes whatever is there.
- **D2 -- Missing `acceptance.md` fails (hv ruling, 2026-06-29).** Failing line 226 but not 207 is incoherent -- you could delete the file to evade the gate. Failing both, with `acceptance: exempt` as the sole escape, is the consistent No-Silent stance.
- **D3 -- WP-level granularity is the one OPEN decision (needs hv ratification).** The gate is one thread-level file, WP-filtered by AC id group (`AC-00.x` = ST-level, `AC-NN.x` = WP-NN). When `intent wp done ST/NN` runs and WP-NN has no own `AC-NN.x` lines:
  - **WP-lenient (recommended):** the empty-check is _thread-level_. A WP closes as long as the **thread** carries >=1 real AC (or is `exempt`); a WP with no own ACs rolls its proof up to the ST boundary (`st done` still enforces "the thread has ACs"). One exemption field; fixes the actual repeated failure (whole threads closing with no contract); matches the report's _rationale_ ("no verifiable definition of done" -> the thread must have a boundary).
  - **WP-strict (faithful to the report's literal wording):** `wp done ST/NN` blocks whenever WP-NN has zero own ACs, unless WP-NN is _individually_ exempt. Catches the "forgot ACs on this WP" silent pass too, but needs a per-WP exemption field (`acceptance_exempt_wps: [NN]`) and forces an AC (or exempt entry) onto every WP.
  - The two differ by ~one line in `cmd_ac_gate` plus (for strict) one extra frontmatter field, so switching later is cheap. **Recommending WP-lenient** for minimal mechanism + migration; hv to ratify. `acceptance.md` AC-01.5 is written to the lenient rule and is the only AC that moves if hv picks strict.
- **D4 -- Ships as PATCH 2.13.1 (hv ruling).** Shipped-as-broken framing. The semver number does not soften the behaviour change: the 2.13.1 release note and the upgrade path MUST lead with the migration (what now fails + the `acceptance: exempt` recipe).
- **D5 -- The canon story changes in lockstep (Highlander).** Several places assert "opt-in / legacy-safe / closes as before"; they become wrong the moment this lands and are rewritten together (WP-02): `bin/intent_acceptance:192-198` (gate header), `bin/intent_st:548-550` + `bin/intent_wp:198-199` (consumer comments), `intent/docs/working-with-llms.md:213` (D11 close-gate prose), and the `acceptance.md` template guidance.

## Architecture

The change is localised to `cmd_ac_gate` in `bin/intent_acceptance`; the consumers (`intent_st`, `intent_wp`) call it unchanged -- they already treat non-zero as "blocked."

New helper (PFIC, single responsibility):

```bash
# True if acceptance.md frontmatter declares an explicit exemption.
acceptance_exempt() {
  local v
  v="$(sed -n '/^---$/,/^---$/p' "$1" | grep -m1 -E '^acceptance:' \
        | sed -E 's/^acceptance:[[:space:]]*//')"
  case "$v" in exempt*) return 0 ;; *) return 1 ;; esac
}
```

Revised `cmd_ac_gate` control flow (No-Silent -- every refusal prints a typed reason, every exit code is meaningful):

```
resolve st dir            -> none:        exit 0   (not a real unit; unchanged)
acceptance.md missing     -> BLOCK exit 1          "no acceptance.md (no contract)"   [D2]
acceptance: exempt        -> EXEMPT exit 0         prints the exemption + reason       [D1]
malformed AC/AT lines     -> BLOCK exit 1          unchanged (F1)
thread has zero real ACs  -> BLOCK exit 1          "zero acceptance criteria"          [closes 226]
in-scope total == 0       -> (WP-lenient) exit 0   rolls up to ST boundary             [D3]
sat == total              -> exit 0                unchanged
otherwise                 -> BLOCK exit 1          "sat/total satisfied; unsatisfied:" unchanged
```

## Migration / fleet impact

This is a behaviour change for **every** consumer project, served centrally from `$INTENT_HOME` (it lands the moment the tool updates -- no per-project rollout). On upgrade:

- Any in-flight ST/WP that never authored ACs stops closing until it authors ACs or adds `acceptance: exempt`. (Intent itself is safe: ST0046 has 10 ACs; ST0048 authors its own.)
- Any thread predating ST0044 (no `acceptance.md`) stops closing until the file is added (real ACs or exemption).

Mitigation = the explicit escape hatch + a migration-led 2.13.1 release note (WP-03). No automatic migration mutates user contracts: we do not silently author exemptions on anyone's behalf -- that would re-hide the hole.

## Test plan

ATs extend `tests/unit/acceptance_close_gate.bats` (the existing close-gate suite), red-first:

- Invert `AT-04.4` -- a present, zero-AC contract now BLOCKS (was "must close").
- Revise the no-`acceptance.md` test (`:145-151`) -- now BLOCKS (was "stays open").
- Add the report's three dogfood self-tests: zero ACs + no exemption -> FAIL; zero ACs + `acceptance: exempt` -> PASS; real ACs present -> unaffected.
- Add a WP-granularity AT per the D3 ruling.
- Mechanical doc guard (WP-02): grep that no "closes exactly as before" / "opt-in" close-gate claim survives in the canon + comments.

## Alternatives Considered

- **Keep opt-in, fix a narrower hole.** Rejected: under opt-in there is no crisp narrower hole -- the hole _is_ the opt-in default (acceptance.md is stamped on every `st new`, so "present but empty" is the norm, not an edge case).
- **Infer WP exemption from "the ST has other ACs."** This is the WP-lenient roll-up and is acceptable because the thread still carries a ratified boundary; it is explicitly **not** inferring exemption from total emptiness (a fully-empty thread still fails). Inferring exemption from emptiness -- the thing the report forbids -- is never done.
- **Body marker `(no-ac)` instead of frontmatter.** Rejected (D1): lives in linter-managed prose, easier to miss, not cleanly greppable.
