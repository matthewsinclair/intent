# inbox: vc -> hv

_(empty)_

## (2026-08-15 17:12Z) FOR YOUR RETURN -- three parked items, each framed as one decision. Nothing is blocked on you except the tap.

Pens taken, peers told, and the operating rule announced: rule what is rulable between nodes, park only what genuinely needs you, and **hv being away widens nobody's authority.**

**1. CREATE `matthewsinclair/homebrew-intent`.** The only true blocker. D40 names the tap; creating it is an account action and dc asked rather than did, correctly. **AC-11.1 and AC-11.4 are downstream of a publication that cannot exist until it exists**, so WP-11 sits honestly at 2/4. Everything dc can do without it is done.

**2. Do `todo --flush` / `--prune` semantics carry into v3?** One decision, and it decides whether a field exists. **If they retire, the DONE watermark retires with them** and DONE filtering becomes a query over the `completed` dates already in the model. If they carry, the watermark is durable state homed in `config.json` (already ruled) rather than grepped back out of the generated `todo.md` (which is what v2 does, and is why the question surfaced). **Recommendation: retire them** -- the watermark is the only durable state in the system with no record behind it, and retiring is the option that removes a concept rather than relocating one. Not urgent; nothing waits on it.

**3. The whiteboard's hand-authored `## (...)` stamp is the same defect we spent the day lancing, and Intent SHIPS this protocol.** Every consumer inherits it. The protocol already contains its own answer in the same document: _"Use commits when you need ordering you can prove."_ **The clock guard's three checks, my six watch-out bullets and your repeated rulings are all scaffolding around a value nobody should be writing.** dc found it and correctly did not propose the change. **Recommendation: a board entry carries no time and is ordered by its commit** -- but this changes a shipped protocol every consumer runs, so it is a scope call and yours. Cost of waiting: nil.

**FOR INFORMATION, no decision wanted.** Issue **0035** filed high: `ac satisfy` accepts an empty `--evidence` at all three layers, in v3 and in v2 -- declaration structurally invisible (`Flag` deserializes 3 of 8 authored fields), `render.rs:671` `unwrap_or_default()`, facade stores it, and v2 never checks `ref`. **A non-test AC can be satisfied with no citation, which collapses the one distinction the AC/AT machine rests on.** Measured blast radius: **zero** -- all 22 satisfied non-test rows in ST0056 carry evidence. Latent, not realised, no audit owed.

Contract 34/106; gates 02 7/8, 03 9/10, 06 4/10, 11 2/4; lint clean at 106 rows. **The long pole is AC-02.8's unit** -- cc holds it, the rulings it needed are made, and `one_clock.rs` will refuse it mid-way until cc reworks the guard, which they have been told.

-- vc

## (2026-08-16 10:18Z) HV DIRECTIVE, ANNOUNCED TO ALL: NO MORE PUSHES TO `upstream`. The CI/CD budget is spent. `local` is fine.

**From hv, just now, verbatim in substance: _"no more pushes to upstream. The CI/CD budget has been hit. Local pushes are fine."_**

**All four of us have been pushing both remotes on every commit** -- it is in our commit habits and in at least my own board's rules -- so this needs to reach you before your next commit rather than after it.

- **`git push local main`** -- yes, keep doing this. Dropbox remote, no CI.
- **`git push upstream main`** -- **STOP.** Every push there triggers the GitHub Actions matrix, and that is what has run out.

**`int prepush` will not save you**: its clone-check gate is about whether `native/` moved, not about which remote you are pushing to, so it will pass a push to `upstream` exactly as before. **This is a discipline, not a control, until someone builds one** -- and I am not building it in `bin/**` with sessions live.

**Nothing needs rewinding.** Work already on `upstream` stays there; this only changes what we do next. **`main` on `local` and `upstream` are in sync as of `99c66e8b`, so nothing is stranded** -- the divergence starts from here and is expected.

**dc: this may want a devbin guard eventually** -- a `prepush` arm that refuses `upstream` unless explicitly overridden would be the natural home, and it is your lane. **Not asking for it now**; flagging that the place exists so it does not get built somewhere else.

-- vc
