# The `v2` field's population -- definition, written BEFORE counting

**PURPOSE.** vc and dc disagree about how many rows in `surface/dispatch-table.json` carry a `v2` field -- vc counts 122, `0255` states 138 -- and neither can reconcile the 16. I am the third count. **This file exists so the definition cannot be retrofitted to a number.** I already know 122, 138, and that a families-level `v2_source` hypothesis is refuted at 149, so I am contaminated on the ANSWERS while clean on the METHODS. Committing the definition first is the only thing that makes the distinction checkable by anyone else.

**I HAVE NOT ASKED EITHER OF THEM FOR THEIR METHOD AND WILL NOT UNTIL THIS HAS PRODUCED A NUMBER.**

## What I inspected before writing this, and it is structure only

- `surface/dispatch-table.json` top-level keys.
- `.families[0] | keys` and the union of all family keys.
- The item shape of `.new_surface`.
- `Entry` and its `v2` field in `crates/intent-cli/src/dispatch.rs`.

**No count of any kind has been run.**

## The structural fact that makes this ambiguous, found before counting

**ENTRIES LIVE IN TWO PLACES.**

1. `.families[].entries[]`
2. `.new_surface[]` -- a TOP-LEVEL array whose items carry the same shape, `path` and `v2` included.

And a THIRD level exists that is not an entry at all: `.families[].v2_source`, a per-FAMILY field.

`Entry.v2` is `#[serde(default)]` in the Rust type, so **a row with no `v2` key and a row with `v2: ""` are different on disk and identical after loading.** Any count taken from the JSON and any count taken through the loader can therefore disagree without either being wrong.

## The primary definition

**P = every object that the shipped dispatch loader treats as an ENTRY, whose `v2` KEY IS PRESENT in the JSON, counted over BOTH entry homes.**

Two commitments inside that:

- **Presence, not value.** `0255`'s subject is the field's GRAMMAR. The population of a field is every row where the field is written; filtering by value answers a question about values, which is the 73-distinct claim and a different question.
- **Entry homes are derived from the loader, not guessed.** Whether `.new_surface[]` is part of the table is a property of `dispatch::table()`, and I will read it rather than assume it.

**Families are NEVER merged into an entry count.** `v2_source` is a different field at a different granularity; it is reported separately or not at all.

## The axes I will report, ALL of them, decided now

| axis           | variants                                                                                                                   |
| -------------- | -------------------------------------------------------------------------------------------------------------------------- |
| entry home     | **E1** `.families[].entries[]` only; **E2** E1 + `.new_surface[]`                                                          |
| field test     | **V1** key present, any value; **V2** present and non-empty; **V3** present, non-empty, and not the `new-surface` sentinel |
| separate level | **F** families carrying `v2_source` -- reported apart, never added                                                         |

Primary is **E2 x V1**. Every other cell is reported in the same table whether or not it lands on anyone's number.

## Rules I am binding myself to

1. **The full grid is published.** Reporting only the cell that matches someone is the retrofit this file exists to prevent.
2. **No nudging toward a match.** vc has said in advance that matching NEITHER of them is the most informative outcome -- it would mean the field's row granularity is genuinely undefined, which is a bigger finding than the number. I am not to treat that as a disappointment.
3. **The definition travels with the figure, always.** A third count without its population is a third opinion.
4. **I do not edit `0255`.** dc owns it. I report to dc and to vc.

Written by ic, 2026-09-05, before the first count.
