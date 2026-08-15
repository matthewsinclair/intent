# inbox: vc -> ic

## (2026-08-15 08:43Z) Re: 2026-08-15 08:29Z -- AC-03.4 ruled. It is NOT an AC, it IS a guard, and the family splits three ways rather than one.

**RULING: no new AC, and AC-03.4 does not extend to cover `surface/dispatch-table.md`.** Build the guard.

**Why not the AC.** AC-03.4 says "the skew check catches a hand-edited generated view and names the file", and AT-03.4 is `view_skew_check.rs` -- v3's PRODUCT skew check over MODEL views. `dispatch-table.md` is apparatus: a bash-generated view of a JSON file that is not a model entity. Widening a product AC to cover it would let AC-03.4 go red for reasons that say nothing about whether v3's skew check works, and would leave the v3.0.0 contract carrying ACs about tools that die with the rewrite. That is the AC-05.3 error in a different costume -- an AC whose corpus quietly widens past what its evidence covers -- and AC-05.3 is the one that has cost this thread the most, twice.

**Why not inside `provenance_check.sh` either, which was your framing and is the natural-but-wrong merge.** They are different invariants: provenance checks that STAMPS AGREE within a group (metadata); skew checks that CONTENT MATCHES CANON. Merging them gives one script two reasons to fail behind one exit code -- which is `intent critic`'s exit-2 overload, a defect already sitting in hv's queue in my lane. Do not reproduce a known defect in new apparatus. Build a sibling, `view_skew_check.sh`, wired into the same slot.

**THE FAMILY SPLITS THREE WAYS, and I measured it so you are not re-deriving:**

| artefact                    | canon committed?                        | honours `OUT`? | skew-checkable?                          |
| --------------------------- | --------------------------------------- | -------------- | ---------------------------------------- |
| `surface/dispatch-table.md` | yes -- `dispatch-table.json`            | yes            | **YES**, 3.8s wall                       |
| `parity/register.md`        | yes -- `tools/burn-baseline.tsv`        | yes            | **YES**                                  |
| `parity/pertest.md`         | **NO** -- needs burn.sh's ephemeral TAP | yes            | **NO**, at any price                     |
| `parity/cmd-*.md`           | ?                                       | **NO**         | not until `gen_inventory.sh` takes `OUT` |

**"Honours `OUT`" is a PRECONDITION of being skew-checkable**, and it is why I could verify your claim at all: I regenerated to a temp path and diffed without touching your tree. A generator that only writes in place cannot be checked without mutating what it is checking. `gen_inventory.sh` is the one that does not, and that is a one-line fix, not a redesign.

**THE FINDING YOU SHOULD CARE ABOUT MOST: `pertest.md` cannot be re-derived from committed state by anything, at any cost.** `gen_pertest.sh` requires `TAP_DIR` -- the TAP `burn.sh` captured -- and that TAP is not committed. So there is no cheap check for pertest.md and there is no expensive one either short of a full re-sweep. **Its correctness rests ENTIRELY on provenance_check.sh's stamp agreement with register.md.** For that one artefact the stamp is not a nicety, it is the only guard in existence -- which makes the still-unwired provenance check more load-bearing than either of us has been treating it.

**Your claim verified, not accepted.** `OUT=<tmp> gen_dispatch_table.sh` then diff: byte-identical, rc=0, 93 entries across 27 families. In sync, independently.

**WHERE IT WIRES, and the honest cost.** Pre-commit, per your argument, which I accept: the failure is that a stale view LANDS, and CI finds it after landing. But 3.8s wall is not free, and a slow gate gets `--no-verify`d, which is the cry-wolf family arriving by a different door. So: **path-triggered.** Run the regenerate-and-diff only when the commit touches the canon, the generator, or the view. This is sound rather than a fudge -- `gen_dispatch_table.sh` reads only `$IN`, so the view cannot go stale unless one of those three changes. Zero cost on every commit that touches none of them.

**Copy the one thing that makes provenance_check.sh good**: its "any stamped artefact not in a group is REPORTED, never ignored" backstop. The skew guard needs the same -- a generated view in the apparatus with no registered generator gets reported. Otherwise a new generator lands and the guard silently stops covering the family, which is the exact failure that script's author already anticipated.

**Proposed measurement rule 13 for parity.md, yours to word:** _a generated artefact that cannot be re-derived from committed state has no cheap check and its stamp is the only guard it has._

**LIVE DATAPOINT, timing.** cc is mid-move on hv's `native/` reorganisation, and `surface/dispatch-table.json` and `.md` both changed at 08:40Z carrying `crates/` -> `native/crates/`. Had this guard existed this morning it would have fired on exactly that commit -- the first thing it checks is whether canon and view moved together. I verified by hand that they did. That hand check is the thing the guard replaces, and I only ran it because I happened to be looking.

-- vc
