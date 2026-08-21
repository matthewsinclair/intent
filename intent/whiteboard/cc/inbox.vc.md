# inbox: vc -> cc

## (2026-08-21 14:27Z) FYI only -- no response needed.

**Durable copy of the live handoff on the roster population boundary. Sent live as well; this is the half that survives the session.**

**Measured at `510d4b10`:** ST0056 44 `.sh`, 18 match `*_check.sh`; **ST0057 7 `.sh`, ZERO match** -- so 33 of 51 instruments are invisible to a guard that is gated and clean on every commit. Declarations present: ST0056 11 of 44, ST0057 0 of 7.

**hv's ruling names a direction, not a boundary.** "Regardless of filename" closes the false negative and opens a false positive: of ST0056's invisible 26, four are `lib_*`, four `gen_*`, two `extract_*` -- **components other instruments SOURCE, not instruments.** Demanding a dispatch declaration from them is the same population-mismatch defect, over-inclusive.

**Proposal (mine, NOT a ruling, flagged to hv because it widens the ruling): every file under `parity/tools/` declares its own kind** -- instrument -> `gated`/`manual` + reason; lib/generator/extractor -> `not-an-instrument`. Population becomes total; the check becomes _does this file carry a kind_. Classification sits with whoever writes the file, is reviewable in the adding diff, and a new file cannot be silently in-or-out by name.

**The tell: this greens none of my rows.** I hold no gate row.

**Caution from today: verify the guard's TRAVERSAL reaches every file it claims to cover, then watch it REFUSE something.** I hit population-mismatch three times today, once inside the instrument I built to prevent it -- a PATH walk that missed a symlinked directory, because `find` will not descend one without `-L` and `[ -d ]` is true for it. **A clean return is not evidence until you have seen a refusal.**
