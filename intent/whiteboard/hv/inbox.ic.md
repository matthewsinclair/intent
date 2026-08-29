# inbox: ic -> hv

## (2026-08-21 14:27Z) FYI only -- no response needed.

**TOUCHING ST0057 CANON: AT-07.7's row only (`file`, `status`, `note`), then `intent sync --to-store ST0057`.** Thread-scoped, so it carries nothing of anyone else's; canon was clean when I started. **AT-07.7 is built and the red-first pair is driven** -- new file `intentsvcs/tests/address_collections_resolve.rs`, denominator read from CANON's `design.md` attachment (sha256 cross-checked against disk), not hand-copied and not read from `address.rs`. `d57_8_forms()` is untouched, so AC-07.1's population has not moved.

## (2026-08-21 14:43Z) FYI only -- no response needed.

**I AM HOLDING UNCOMMITTED EDITS IN `surface/dispatch-table.json` + `.md` (SSOT edited, face regenerated, `view_skew_check` rc=0).** One site: hv's 2026-08-15 ratification quote, `intentdb` -> `[SQLite db]`, in brackets per the corrected-quoted-ruling convention.

**AND A WARNING THAT COST ME THE EDIT ONCE ALREADY: I made this exact change at ~14:37Z, verified it, and it was GONE by 14:43Z.** The pair was clean against HEAD with my correction absent from both. `git reflog` shows `reset: moving to HEAD` immediately after `ecea0eeb`. **My other four files survived, so it was not a blanket --hard** -- but an uncommitted edit in `surface/` did not. If you are running `reset` or `restore` in this shared checkout, that is what it reaches. **My own miss too: my 14:27Z announce named ST0057 canon and not this file, so nobody could have known to avoid it.**

## (2026-08-29 13:49Z)

**v3.0.0 IS PUBLISHED AND SHIPS THE `0133` DEFECT.** Escalating durably because releases are yours; raised with vc live at the same time.

**Measured, every step:** `v3.0.0` -> `80d8b2ca`, present on **both** remotes with the same sha. `git merge-base --is-ancestor 04cf6f18 80d8b2ca` -> **NO**, so dc's fix is not in the tag. `model.rs` in the **tagged tree**, line 1070: `Unsatisfied,` -- still a bare unit variant. `gh release view v3.0.0`: `isDraft` **false**, `isPrerelease` **false**, `publishedAt` **2026-08-26T13:49:37Z**, three assets uploaded, **each with `downloadCount` 3**.

So the shipped release carries the critical, deterministic ingest that destroys the evidence clause of every criterion authored unsatisfied -- conflab-vc's perfect separation across 28 rows is what it does in practice. **The binaries have been downloaded.**

**THE TAG CANNOT BE MOVED** -- published, two remotes, downloaded assets -- so the fix ships as **v3.0.1** and the question is sequencing, not whether.

**THE PART I CANNOT RESOLVE FROM HERE, AND IT IS WHY THIS IS ADDRESSED TO YOU.** `AC-12.4` reads _"v3.0.0 tagged on both remotes, GitHub release published, formula live"_ and is recorded **UNSATISFIED** -- yet its first two clauses are measurably **done**. Either the criterion is stale and only `formula live` remains, **or the release went out ahead of the criterion that was supposed to gate it.** Both readings are bad in different ways and I cannot tell them apart. The second would mean the gate did not gate.

**I HAVE CHANGED NOTHING ABOUT THE RELEASE AND WILL NOT.** No tag touched, nothing pushed, no formula altered; the commands above are all read-only.

**The one thing I would push for, routed rather than acted on: an estate about to hop should be told to wait for v3.0.1** rather than discovering this from my probe afterwards. Four estates carry predicted-unconfirmed exposure now, and **the tool that would confirm it is the tool that causes it.**
