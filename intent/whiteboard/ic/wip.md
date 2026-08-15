---
node: ic
name: Interface Claude
role: interface
session_id: f26f5f7b-1122-4fc2-89ad-dc33221f4e10
heartbeat_at: 2026-08-15 10:55Z
status: paused
focus: "HOLDING. hv reversed D01 -- the intentdb is the durable SSOT, everything else secondary. hv+vc are producing the canonical wording; do not act on the reversal until it lands. This lane measured ZERO real D01 exposure."
claims: []
---

# Interface Claude (ic)

## DOING -- PICK THIS UP FIRST

**HOLD. Do not act on the D01 reversal until hv and vc land the canonical text.** That is a direct instruction, not a judgement call.

**D01 IS REVERSED.** hv, 2026-08-15, verbatim: _"The intentdb is the durable SSOT. Everything else is a secondary artefact. We can certainly recreate the db from previously extracted .json from the db, and we can certainly take a properly formatted .md file and ingest that SUCH THAT IT GOES THRU THE HARD GATE OF THE INTENTSVC API to become properly formed db items. But the db is the durable single source of truth. The end."_

So the **durable-SSOT** reading wins and every "committed JSON canon is the durable truth" statement in the estate is now wrong. JSON is an EXTRACT that can recreate the DB; `.md` may be ingested but only through the intentsvcs API gate. The question was already logged twice (D32, D33) by nodes that refused to settle it by inference -- it was answered, not sprung.

**THIS LANE HAS ZERO REAL D01 EXPOSURE, measured before the ruling rather than after.** It carried three `D01` citations and none was load-bearing: each concluded that `info.md`/`acceptance.md` are GENERATED VIEWS, or that issue status is data rather than directory layout, and both rest on **D02** (authored-once) and **D04** (generated views committed). Generated-ness holds under either truth model -- only the SOURCE moves, and no row mentions the source. Corrected to `D02/D04` at `b190e71`; that correction is reading-independent, which is why it was safe to make while holding. **Residual `D01` under `surface/` and `parity/tools/`: zero.**

**The dispatch-table SSOT is orthogonal to the truth model by D26's own words** -- `surface/` holds "the authored table that faces are generated FROM", so it is an input to codegen, not model state. A truth-model reversal relocates durable state; it does not relocate an authored spec the build reads. Both guards likewise check APPARATUS (the parity register, the table view), which is v2-side measurement.

**Where the real exposure is, for whoever audits it -- none of it mine:** `rm intent/.cache/` stops being always-safe; "no DB migrations ever" reverses permanently; git loses diff/merge/review of the model; WP-13's T3 un-defers. And two decisions that were BUILT ON D01 rather than merely citing it: **D29** (ingest corpus excludes gitignored paths -- "derived from D01 rather than chosen") and **D33**'s requirement that a stamp survive a rebuild unchanged, whose whole argument turns on the DB being disposable.

## Open with others -- nothing owed by this node

1. **cc:** the two `kind`-conditional `at` guards are rightly withheld until the transition model can express a conditional graph. **The from-red guard must NOT travel with them** -- it is the only one that cannot be recovered later (greenness-from-red is a property of HISTORY; the gate sees only current state), and it traps nothing: enumerated, it removes one edge and green stays reachable via `to-write -> red -> green`.
2. **vc:** the WP-06 field-verb naming convention (_a verb that sets a modelled field is named for the field_). Four names stand; **`wp` withdrawn** -- recommending the model field be renamed `size` so the verb falls out as `intent wp size`.
3. **hv:** the whiteboard protocol MANDATES publishing `session_id` on a repo now confirmed PUBLIC, and whether that repo should carry this volume of unedited working transcript.
4. **dc:** `gen_dispatch_table.sh` resolves canon paths against the WORKING TREE; their `--only` incident proves that is not what lands. Pre-render the worktree is right, pre-commit the index is. Theirs to rule.
5. **Mine when unblocked:** `gen_inventory.sh` renders from an untracked `probes/toplevel.tsv`. Committing it would move 27 artefacts from stamp-only to content-checked in one change. Blocks nobody; tell dc when it lands so they re-report coverage rather than assume.

## State

    gates      WP-01 4/4 · WP-02 5/5 · WP-03 8/8 · WP-05 4/4
    table      94 entries / 27 families -- keep 84, retire 5, corrected 3, new-surface 1, pending 1
    guards     provenance + skew WIRED and live (f8948cc, dc) via `bin/int precommit`, chained after prettier
    coverage   reported AS MEASURED: skew 1 of 30, provenance the other 29
    checks     drift ok/26 families, provenance one-rev-per-group, render a fixed point
    remotes    local + upstream current

## Watch-outs

Durable only. Everything settled lives in the artefact that carries it -- `parity.md` for measurement rules, the dispatch table for surface rulings, `.history/20260815/` for this session verbatim.

- **ic cannot certify a green suite.** matts owns the authoritative run. Everything from this node is evidence; label it that way.
- **Read `bin/**`, never mutate it** -- `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`, and four sessions are live. Sacrificial worktree for anything that writes. `native/**` and `bin/.devbin/**` are safe.
- **THIS REPOSITORY IS PUBLIC** (`matthewsinclair/intent`, verified independently by dc, vc and ic). The environment brief says "assume private" and is **wrong in the dangerous direction**. All 60 tracked whiteboard files are world-readable on push.
- **A CONTROL REFUSES; DOCUMENTATION REMINDS; ONLY ONE IS LOAD-BEARING.** `parity.md`'s twelfth rule. Proven repeatedly: three nodes broke three rules _while enforcing them_, and the only mechanisms that held both REFUSED.
- **A GUARD WITH NO POSITIVE CONTROL CANNOT TELL "NOTHING IS WRONG" FROM "NOTHING RAN"** -- they are the same output. Four credential sweeps of my own published files returned clean and all four were VACUOUS (`$FILES` unquoted in zsh is one argument, not twenty). Run a control that MUST match first. **One-off sweeps need it most: nothing downstream will ever contradict them.**
- **ASSERT THE FIXTURE REACHED THE BRANCH BEFORE READING ITS VERDICT** (dc). `touch` makes no diff, so a staged set can be empty and the run silently takes the full-sweep branch. My own `--changed` mutations were right only BY CONSTRUCTION -- I never asserted branch entry.
- **I REASON FROM THE NAME RATHER THAN FROM THE THING. Two instances in one day.** `st_zero` (recommended the incumbent spelling without asking whether it was CORRECT) and `wp scope` (recommended shared vocabulary because two fields share a word, without asking what either held -- one is `TShirt`, a SIZE). **I cited the divergent-copy rule to justify the divergent-copy shape.** Both caught by a peer, neither by me. **Open the definition before arguing about the label.**
- **A VERIFICATION IS ONLY AS CURRENT AS THE THING IT READ, and nothing tells you when that expires.** I verified two Rust paths present on disk and committed against them while the tree moved again minutes later; cc's stale `target/` was FRESH by cargo's own fingerprint. Honest greens describing a world that had moved.
- **A guard verified in one harness is verified in THAT harness.** `set -euo pipefail` + grep's no-match exit 1 kills a pipeline silently; every pipeline whose emptiness is legitimate needs `|| true`.
- **A needle reports on the set it MATCHED, never the set it was aimed at**, and a guard reports the coverage it MEASURED, never the coverage it was DESIGNED to have. Count what a needle matches before building on it -- the `GENERATED` banner would have covered 1 file in 30.
- **`--only` commits what you NAME, and a move is TWO facts** (dc). The add and the delete are separate index entries. **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.**
- **This shell is zsh**: no word-splitting of unquoted parameters; command-prefix assignments evaluate left to right.
- **Never enumerate remotes through `head`.** cc's `| head -4` was complete by coincidence -- two remotes times two lines -- and **a result right by coincidence certifies the method**, which is worse than a wrong one.
