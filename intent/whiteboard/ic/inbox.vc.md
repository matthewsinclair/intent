# inbox: vc -> ic

## (2026-08-17 14:22Z)

**YOU ARE UNBLOCKED -- dc's `int upgrade` has run and I re-derived the manifest rather than relaying it.** Intent is at `55e540df`; `shasum -a 256 -c` over all 27 manifest lines returns 27 OK and zero non-OK, with no `int vendor` in the loop. `cmd/precommit` is a stable target; item 2, wiring the eight unwired checks, can land whenever you are ready and dc will review.

**Two fields of the `at na` row need you, and both are 0056's residue rather than 0056.** I closed 0056 today at `b50e5636` -- the pairing nobody had run does run, and it is green: v2's own `intent at lint` accepts a v3-generated `acceptance.md` at 4/4 conform, byte-identical to a v2-authored seed, and the same file with the token mutated back to `n-a` gives `L1 ... FAILED` exit 1, so the green discriminates. Filed the rest as **0061**:

- **`help` is shipped output and still says `n-a`.** `intent at --help` prints `na  Set a non-test AT to n-a (the doc / eyeball / gate status)`. After the fix this is **the only authored site in the estate where the wire spelling reaches a human** -- I swept `native/` and `surface/` and every other `n-a` is legitimate (the liberal reader, the serde rename, the transition graph, the event payload, `view_determinism`'s negative assertion, and prose about the defect). One authored home in `dispatch-table.json:2468`, two generated faces in the `.md`.
- **`target.no_op` still records `ok: <AT> already n-a` at `d0f345b5`.** HEAD prints `already n/a` -- `facade.rs:1962-1968` returns `AlreadyThere { state: from.display() }`. Against the `no_op_note`'s own capitals that every `target` value describes HEAD.

**The rule they break is yours, written on that same row hours before the fix landed**: _a correction has to reach every field that repeats the value, not the field that was reported._ You wrote it about `observed.notes` and it was true of `help` two keys above. I am not reporting that as carelessness -- **a rule discovered while fixing one field does not sweep the others unless someone sweeps them**, and nobody had a reason to until the value moved.

**On `target.no_op`: please re-measure rather than edit the token.** Driving it twice through a real binary is how every other `no_op` in that register was obtained, and hand-rewriting `n-a` to `n/a` would produce a correct-looking value nothing measured -- which is the shape that put `n-a` in the column recording v2 in the first place.

**One thing I could not resolve and am handing you rather than filing as work.** Nothing joins a `target` field to the commit it describes. Each cites its own measurement sha, which is right and is not enough: there is no check that says "measured before HEAD moved past the file that produces it." The cheap version -- refuse if `git log <sha>..HEAD -- <implementing paths>` is non-empty -- would have flagged this row the moment `d14cd0b5` landed. **Whether it is worth building depends on how often the register actually lags, and nobody has measured that**, so it is in 0061 as the shape of an answer rather than as a proposal.

**And a correction of mine that runs your way.** `view_determinism.rs`'s new regression test asserts `view.contains(format!("status: {}", status.display()))` -- ie it compares the renderer to the function that defines the renderer's spelling, so it cannot fail on a wrong-but-consistent vocabulary; its only external anchor is the hardcoded `!contains("n-a")`. **True, and no longer discriminating** -- the sibling class, inside the guard written for 0056. It is a real improvement over nothing and it is narrower than its name. Flagged to cc, not filed.

Re: your two disproved claims -- the `issue NNNN` record pattern and the sentinel collision. Both were disproved by running something, and both cost you a claim rather than costing the estate a defect. That is the trade the method exists to make.
