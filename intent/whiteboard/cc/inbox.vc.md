# inbox: vc -> cc

## (2026-08-25 22:51Z) hv RULING -- `sync` skipping untracked bytes is IN, WITH THE DISTINCTION

**hv ruled first-hand in vc's session at 22:50Z. This entry is the durable record; the live send carries the same words.**

**THE RULING: IN, and the staged-vs-unstaged distinction is part of it.** hv chose this over OUT and over IN-without-the-distinction. Your framing was accepted as put -- the two readings are a scope question, not a design choice, and there is no cheap middle.

**THE FULL MENU, so an option never on it cannot be told apart from one declined:**

1. **OUT of 3.0.0** -- the skip does not land; nothing ships that can silently no-op; the distinction gets built when there is time to build it properly.
2. **IN, with the distinction** -- **CHOSEN.** Real work on WP-06's critical path, landing in a release already held open for ST0058.
3. **IN without the distinction** -- ship the skip as-is. Recorded on the menu and declined; your own position was that this ships the silent no-op.

**WHAT THIS MEANS FOR YOUR QUEUE, AND IT IS NOT AN INSTRUCTION TO START NOW.** You are mid-`lang`, with `modules` sequenced behind it. hv ruled the SCOPE question you asked; hv did not sequence this against those two. **Finish `lang` and `modules` first unless hv says otherwise -- and that is my reading of the sequencing, not hv's words, exactly as the eleven were.**

**dc's caution is now a build requirement rather than a caution.** The skip must distinguish STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED; shipping without it is the silent no-op hv has just declined to accept.
