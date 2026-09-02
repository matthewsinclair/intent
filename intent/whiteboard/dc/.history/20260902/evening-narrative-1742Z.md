# dc -- evening narrative, 2026-09-02 1742Z

Pre-fold board beside this file: `wip-prefold-1742Z.md`, sha `39be64e6`, 59,327 bytes, `cmp` IDENTICAL against the live file at archive time. The board keeps the rules; this keeps the reasoning.

## What the evening was

hv routed their open set through vc, vc sequenced it, and **seven items came to me as vc rulings under hv's pen -- not hv rulings, and the distinction is structural rather than polite.** Four landed. The afternoon had been one long lesson about populations; the evening was the same lesson arriving four more times, twice against me, and twice from peers who could have absorbed it and did not.

## The through-line: a ruling named after the instance it was found in

**ITEM 1.** vc ruled _exempt `hv/inbox.*` from prettier_ because cc had been forced to rewrite a byte inside a delivered entry. I ran the positive control before counting anything -- the protocol's own heading form through the formatter the gate resolves -- then counted the corpus. **39 headings carry a `Re:` or `FYI` field and ZERO retain the three-space separator the skill specifies.** Every node wrote the documented format; every one was rewritten on the way in. **24 in `hv/inbox.*`, 15 in peer-to-peer inboxes the narrow ruling did not reach.**

**I did not widen it myself.** W49 was four hours old and the convenient direction is still the wrong one to take unilaterally. vc ruled wide and named their own narrowing as W50 arriving from the other side: _the reason I gave for the narrow ruling was already the wide one, and I did not notice._

Two defects, one commit. The append-only surface stops being rewritten, and **the spec moved to one-or-more spaces rather than the 39 files being mass-edited** -- nothing parses the separator, so rewriting them would be a bulk byte-change across append-only surfaces to satisfy a cosmetic field. `.prettierignore`'s header said NOTHING ELSE IS EXCLUDED and named the whiteboard as formatted; it now carries a CRITERION rather than a list -- **single writer AND existing content is evidence, both halves required.** That criterion is what keeps a BOARD out of the exemption, and verifying a board is still formatted was the check that mattered.

## The one I broke, and the two people who caught it

**I left `facade.rs` half-applied and walked away.** Variant and raise in, const and exhaustive arm not. `intentsvcs` is a dependency of `intent-cli` and `intentd`, so it was red for every node in the workspace, not for me. cc found it, told me, and named the reason it is not carelessness: **a Rust enum makes exhaustiveness a distributed obligation and the compiler reports it from DOWNSTREAM, so the node who breaks it is structurally the least likely to see it.**

Then cc came back with the sharper version. **`--workspace` is not a weaker `--workspace --all-targets`; it is a different SUBJECT** -- the exhaustive matches live in test targets a plain check does not build. And `error_remedies.rs` already carries a comment naming the exact command, written about ic hitting the same class at `db3f947a`. **Three of us have read that file and all three ran the weaker form.** Four instances, and the fourth was a defect in the cure.

**A doc that names its own instrument does not make anyone run it.** Same shape as this morning's `36`: the artefact was correct and nothing downstream was obliged to act on it.

## `0207`, and the word I dropped

vc ruled (c) -- refuse on the status verbs, give `at edit` the `--note` it should always have had -- with rider 1 I would have got wrong alone: **`at edit --note` gets NO refusal, because a door that refuses is not a door.**

Then cc found the guard firing on a test whose own name says a note-only write is legal. **The cause was one word: `0207` says refuse when the existing note is MATERIALLY longer, and I implemented `shorter`.** I built the spelling and dropped the word carrying the judgement, on the day I had been handing that class to other people.

**So I went to find the threshold and the corpus refused to give me one.** 313 notes with content: median 178, p75 1184, max 17485, and **every size from 150 to 1200 populated near-continuously, largest gap about 30 bytes.** No valley. That retired the threshold option on evidence rather than taste, and its failure mode was the silent one -- a 400-byte record destroyed below the line with the guard green.

**vc then ruled out my recommendation too, and was right.** Containment, not length: **a length check FAILS OPEN on a same-length rewrite**, which is the shape cc had found in `threads.revision` that morning. Containment dominates -- anything appended contains the original -- and needs no threshold, so my corpus result stands as the reason one was never available.

And vc took my diagnosis one level further than I had: **`materially longer` was ITSELF instance-shaped**, written looking at a 7803 -> 683 destruction where length was the visible symptom. **The mechanism was never length; it is content loss.** W50 sitting inside `0207`'s own remedy sketch.

## The test change, and the objection I could not answer myself

Changing a failing test to match new behaviour is what the `AC-06.1` rider exists to stop, and cc raised it before I could. My discriminator -- _the test still tests its own name_ -- was my reading of my own change, which is the least trustworthy kind.

**vc settled it structurally: a test updated to follow a RULED contract change is auditable as one; a test updated to follow an UNRULED implementation is the defect.** Condition: the test carries the ruling, its date, and what its expectation used to be. Without that the next reader cannot tell the change from the thing it resembles.

## The instruction that paid out

vc's last line was _do not assume the fix moves exactly one test._ **It moved three, all mine, all in `error_remedies`** -- my own provocation setup landed a long note that did not contain the existing one. Correct under `shorter`, refused under containment.

**It failed loudly because I had written it as an `expect` with a comment saying it was an assertion rather than a convenience -- an hour before it fired, for exactly the reason it fired.** That is the only thing today I would repeat unchanged.

## What I am carrying into the bounce

Three items open of the seven, plus the two hv already holds. Everything routed through vc this time, at hv's instruction.
