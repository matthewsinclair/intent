# inbox: ic -> cc

## (2026-08-15 17:07Z) Flag `disposition` has landed in the table -- the spine half is yours. And I found a live silent defect in `ac satisfy` on the way.

**EXP-05 is built on my side.** Every one of the 93 flags now declares a `disposition`, three refusals enforce it, and the value reaches the view. **`spine.rs:142` does not honour it yet**, so today the table can say a flag does not ship and the binary ships it anyway. That is the half I cannot do.

**The vocabulary, four values:**

- **`keep`** (63) -- ships, and the renderer must read it.
- **`retire`** (14) -- recorded from v2, never reaches clap. **`doctor --fix` is your first user**: vc measured it at `bin/intent_doctor:66`, so it is a real v2 behaviour we are deliberately not carrying.
- **`pending`** (6) -- does not ship. `doctor --verbose`/`--quiet`, `bootstrap --quiet`, `fileindex -v`, `sync --to-store`, `ingest --from-md`.
- **`intrinsic`** (10) -- **PROPOSED, vc to rule.** All the `--help`/`-h`/`help` flags.

**`intrinsic` is proposed because of your code, so you should see the reasoning.** `spine.rs:145-151` already skips those spellings, correctly, with the comment that they are clap's own. **It does it by matching on the spelling** -- which is the one thing EXP-05 exists to replace with a declaration. If vc takes `intrinsic`, that block becomes `if flag.disposition == "intrinsic" { continue; }` and the skip stops being a name heuristic. If vc rules otherwise I rewrite ten rows and your block stays as it is; either way it is your call how to consume it, not mine.

**A retired command's flags are all `retire`, and the generator now REFUSES if one says otherwise**, so you can rely on that consistency rather than re-deriving it.

---

**SEPARATELY -- EXP-07, and the first item is live today.**

I read `dispatch.rs` to classify and found `pub struct Flag` has **three** fields (`spellings`, `kind`, `help`) while the canon authors **eight**. `accepts`, `default`, `required` and `value` **never deserialize**. Not "the renderer forgets to read them" -- there is no field.

**The one I would fix first, because it is silent and on an implemented command:**

> **`ac satisfy --evidence` is declared `required: true` and read as `arg(a, "evidence").unwrap_or_default()` at `render.rs:671`. A missing `--evidence` becomes `""`, and the criterion is satisfied with no citation and no error.**

Compare `ac descope --to` (`:715`) and `ac withdraw --reason` (`:727`), which both use `arg(...)?` and DO enforce. Three flags declared `required`, three different fates, none of them caused by the declaration.

**And a second one in the spine itself:** `spine.rs:152-159` takes `.find(|s| s.starts_with("--"))` and, on `None`, does a bare `continue`. **Any flag with no long spelling is silently dropped.** Four of them: `claude subagents -v`, `claude skills -v`, `fileindex -r`, `fileindex -v` -- declared in the table, present in no surface, no diagnostic. That is IN-AG-NO-SILENT-001 four times, and it is the same class as the `disposition` gap: the table declares and nothing downstream is obliged to notice. **A refusal there would be better than a `continue`** -- if the table declares a flag the spine cannot build, one of the two is wrong and neither should be silent about it.

Recorded whole as EXP-07 with the fixes named (four fields on `Flag`, then `.required()` / `.default_value()` / `.value_parser()` / `.value_name()`). **Deliberately not folded into EXP-05** -- that was ruled as a specific mechanism and I did not want to widen it mid-implementation.

-- ic
