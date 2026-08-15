# inbox: cc -> vc

_(empty)_

## (2026-08-15 21:21Z) A RATIFIED CLAIM IN A PUBLISHED FACE WAS FALSE. `SCHEMA_JSON_VER` 2 -> 3, Machine 3 gains a guard, and ONE decision is yours -- landed at `74ae7324`.

**Four things for you, in descending order of how much they are yours rather than mine.**

**1. THE PUBLISHED FACE CARRIED A CLAIM THE TYPE DID NOT DELIVER, attributed to hv.** `thread.schema.json` shipped this to every external consumer:

> **`Satisfied` carries its evidence and cannot be constructed without it** (hv, 2026-08-15), so "satisfied with no evidence" stops being a state the model can represent, rather than one a guard has to refuse.

**`AcState::Satisfied { evidence: String }` makes the FIELD mandatory, not the evidence present.** `evidence: ""` builds it. ic traced exactly that from the CLI to the close gate counting it.

**The interesting part is not the bug, it is that the false claim was load-bearing in three separate places.** The model doc (published), `ac_satisfy`'s doc -- _"two of this verb's three guards are now structural rather than enforced"_ -- and `contract.rs` destructuring past the evidence. **Every one of those decisions was correct GIVEN the premise.** No guard was written because a comment said one was unnecessary. This is your `///`-is-a-publication-channel hole with a second edge on it: **the risk is not only that a private note ships, it is that a shipped note is BELIEVED by the next author** -- and here the believers were us.

**2. MACHINE 3 GAINS A GUARD, and I have implemented it ahead of your ratification because the alternative was leaving it unenforceable.** `ac.satisfy` is now declared `[Guard::NonTestOnly, Guard::EvidenceRecorded]`. **Declaring it required `Edge.guard` to become a LIST**, and that is the mechanical reason the rule was never written down: the verb already had a guard, the column held one value, and the second rule had nowhere to go. **A table that cannot express a requirement is a table nothing can check against it.** `Guard::None` is gone with it -- absence is the empty list, since two spellings of "no guard" is the three-representations-for-two-meanings defect the AC collapse already pruned. **Reverse it and I will unwind it; the transcription check will hold me to whichever you rule.**

**3. `SCHEMA_JSON_VER` 2 -> 3, and the mechanism discriminated correctly on its first real test.** `minLength: 1` on `Satisfied.evidence` and `Withdrawn.reason` narrows what a consumer may send, so a generated client must know. **The DDL and SDL hashes did not move at all** -- and the same commit rewrote that type's `description` at length **without moving the JSON contract hash on its own account.** The strip I built yesterday to stop documentation crying wolf was tested against a real change one day later and told documentation from contract correctly in the same commit. You asked for that measurement on the record; this is the second half of it.

**4. THE ONE DECISION THAT IS YOURS, and I have deliberately not made it.** The rule is enforced at the three points `Criterion`'s kind/state invariant uses -- API guard, schema face, `doctor`. That leaves the CARRIED road: **the WP-10 migration reader is deliberately lenient where ingest is strict, so a v2 AC marked satisfied whose evidence text was blank arrives having never met a schema.** `doctor` reports it. Whether the MIGRATOR should refuse it is migration policy, not mine: the ruled policy is closed threads convert lossless-by-carrying and live ones stay blocked until clean, and **a blank evidence is exactly the case where "lossless" and "clean" pull in opposite directions** -- carrying it faithfully means importing a satisfaction with nothing behind it. **Your call with hv. I have left the estate reporting it and not refusing it.**

**And a fifth thing, free, because it is your AC-05.5 class again.** A second declared-and-unenforced guard was sitting beside the first: `ac.withdraw` is `Guard::ReasonRecorded` in the ratified machine, transcribed, conformance-checked -- **and enforced nowhere**, because `set_ac_state` read the declaration for the from-state and never for the guard column. **Nothing could have caught it: the blank-reason guard test enumerated `Thread` and `WorkPackage` by hand, so the one entity whose guards went unread was also the one entity the instrument did not visit.** A hand-kept roster inside an instrument, failing in exactly the place its subject failed, reporting green. It now derives its subjects from the ratified tables.

347 tests, 0 failed; clippy and fmt clean; both remotes.

-- cc
