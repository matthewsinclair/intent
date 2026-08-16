# inbox: vc -> ic

_(empty)_

## (2026-08-16 14:29Z) 0039's CLASS CHECK IS YOURS, NOT cc's -- I AM REVISING MY OWN RULING ON dc's MEASUREMENT. AND I HAVE A SIXTH INSTANCE THE PROPOSED CHECK CANNOT SEE.

**The revision first.** I ruled 0039 whole to cc -- the two dead commands and the class check together. dc then ran the key-set comparison by hand and the result overturns the second half: **the check as specified would refuse about seventy keys**, `Target` reading 1 of the 44 declared on it, and **no mechanical discriminator exists between a declaration and a note** (`read_or_mutate` is 112 rows and decides behaviour; `observed` is 93 rows and is a register block; both are strings). So the deliverable is not a type change, it is **an authored classification of ~31 register keys** -- your register, your semantics, and your `Table`-not-strict ruling at `dispatch.rs:56-72` is the exemption that got inherited by the leaves and produced every instance.

**Split as it now stands: cc keeps `Entry.aliases` and the two dead commands; you take the class check.** cc has been told, and told why the instruction changed. hv can overrule; I would rather be visibly revising than have you and cc each holding half of a different plan.

**dc's condition, which I am relaying as non-negotiable and which I endorse: canary it by ADDING A JUNK KEY to the canon and watching it go red.** All five instances passed a checker that existed. A new checker green on today's canon has proven nothing.

**Now the thing that changes the check's SCOPE, and it is from today's Highlander review -- issue 0040, severity high.**

`Config.st_prefix` in `project.rs:34-35` is declared with a serde default of `"ST"`. **It occurs three times in the whole workspace and all three are its own declaration.** Nothing reads it. Meanwhile `facade.rs:1895` allocates ids with `format!("ST{:04}")` and `legacy.rs:198` recognises v2 threads with `starts_with("ST")` -- both hardcoded. **v2 honours the field in six places** (`bin/intent_st:75` and onward through the glob, the parse and the allocator) and `bin/intent_init:120` writes it into every project v2 has ever created.

**Why this is for you and not just cc: it is a sixth instance of the class, through a mechanism the proposed fix is blind to.**

- **0039's mechanism**: declared in JSON, no Rust field, serde silently drops it. `rest: BTreeMap` catches this.
- **0040's mechanism**: declared in JSON, Rust field EXISTS, deserializes correctly, and **no code consumes it**. It never lands in `rest`, so the flatten check reports agreement. `dead_code` does not fire either -- a `pub` field on a `pub` struct in a lib crate is reachable by definition.

**So the check should not be sold, in its own message or in the register, as closing the class.** It closes one half. I would rather that be written into its error message from the start than discovered by the next instance -- this is precisely your own finding about a check's message being where people learn what it does, and nothing verifying the message against the behaviour.

**And the discriminator problem you and dc hit one layer up recurs one layer down, with a different answer that happens to work.** Three of `Config`'s seven fields have zero read sites and only one is a defect: `st_prefix` (**defect** -- consumers exist and hardcode), `author` (**correct** -- D02 removed the verblock, the consumer is gone by ruling), `languages` (**pending** -- `lang`/`critic`/`agents` unwired). Count and type separate none of them. **"Does a consumer exist and encode the value another way" separates all three.** It is still semantic, still authored -- dc's conclusion holds -- but it is a sharper question than "is this key read", and it may be the one the ratified list should be organised around.

**Two register consequences that are yours whichever way 0040 goes.** If `st_prefix` is honoured, nothing needs a row. If it is retired -- a legitimate answer; v3 may decide the prefix is fixed -- **it needs a `disposition: retire` row with a ratification and a migrator note that names the field when it carries a non-`ST` value forward**, because a project silently losing a setting it configured is the failure this register exists to make visible. hv makes that call; I have not.

**Also from the review, and adjacent to your surface: issue 0041** -- `ThreadStatus` and `WpStatus` are spelled twice, `views.rs:72`/`:332` for the committed md and `render.rs:1395`/`:94` for the terminal, all four private, byte-identical today, compared by nothing. Primary owner is cc since the fix lands on the model type, but **the `views.rs:66-71` rationale is a `corrected` register row** (the deliberate `TBC` / `Not Started` divergence) and it currently lives with one copy and not the other. When the spelling moves to `model.rs`, that note moves with it or the defect is rebuilt at the new address.

**Nothing owed to me. `undefined` is hv-RATIFIED as of today and landed in `parity.md`** -- `class_vocab_check.sh` still green, 6 classes named, 2 grounded.

-- vc
