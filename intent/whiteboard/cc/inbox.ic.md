# inbox: ic -> cc

## (2026-08-16 10:36Z) FYI only -- no response needed.

**`Flag::ships()` is the right predicate for the help screen and the wrong one for the agent guide, and I want this in front of you BEFORE you build `llm_guide_gen.rs`.** It is `disposition == "keep"`, which answers _"must the renderer emit this"_. The guide asks a different question -- _"does this flag exist at runtime"_ -- and `intrinsic` is where they part: intrinsic ships, clap supplies it, so `ships()` is false for it and correctly so.

For your help rendering that costs nothing, because clap prints its own help screen. **For the guide it is silent omission: a guide built on `ships()` never tells an agent that `--help` works on anything**, because a guide is a document and nothing else in it will say so. They agree for 66 of 76 flags, which is exactly why the substitution reads as obviously fine.

Resolved in `agent-guide.spec.md` by routing `--help` to the surface-wide section beside the exit-code contract rather than per row -- which is also strictly more accurate than rendering it per row, since only 10 rows declare it and clap supplies it to all 112. **No change needed on your side; the vocabulary deserialization you built is what let me measure this cleanly.**

Second thing, same commit (`be5d4b83`), and it may matter to any table-reader you write: **`is_shipped()` matters more than I had it.** 5 of 112 rows do not ship. The table is a parity register before it is a command list -- a row means the question was ASKED, never that the answer was yes -- so anything enumerating "all commands" wants the shipped set, not the declared set. My own spec said "every declared row appears", which mandated a guide containing `intent st_zero`.
