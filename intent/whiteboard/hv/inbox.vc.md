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

## (2026-08-16 11:31Z) FYI only -- no response needed. **hv HAS RULED THE v3 CUTOVER VERSION: 3.0.0.**

**Direct from hv, this session:** _"The v3 cutover version is 3.0.0. We will get all of this done -- including the text search and code parsing -- and then push 3 and then fix forward on patch releases."_

**Three things follow and the third is the one that changes sequencing.**

**1. dc is UNBLOCKED. AC-11.1 and AC-11.4 sat behind a real version and nothing else** -- not the tap, which has existed since 15:19:58Z yesterday. That was the only thing standing between dc and those two rows.

**2. The scope statement is now explicit and it is WIDER than the twelve-WP ladder reads.** Text search and code parsing are named as IN for 3.0.0, not deferred to a patch. WP-13 (`index_scope` / `search_lexical` / `search_structural` / `index_staleness` / `search_degradation` / `background_index` / `mcp_search_tool`) is nine `to-write` rows today and it is not optional.

**3. The release POSTURE is fix-forward on patches.** Ship 3.0.0 when the ladder is done, then correct on 3.0.z. **That is a licence to finish, not a licence to lower a bar** -- the fix-forward half applies after the cut, and the ACs are still the gate before it.

-- vc

## (2026-08-16 11:37Z) `undefined` -- the info you asked for, and it is a smaller question than I made it sound.

**FIRST, A CORRECTION TO MY OWN FRAMING. I told you `undefined` had no place in `parity.md`. It is in `parity.md`, at line 17, and has been since 2026-08-14.** My grep searched lowercase `undefined`; the file capitalises it as **Undefined** in the class list. Same miss I made on `INTENT_VER` this morning -- searching for the literal token instead of the idiom. **The instrument agrees with the file**: `class_vocab_check.sh` reports _"2 states claim a parity class, 6 classes named, 2 grounded -- every claimed parity class is named in parity.md."_ There is no drift and nothing is broken.

**So the actual question is one word: RATIFY OR FOLD.** The class is written, used, and consistent; it is marked _"provisional pending hv"_ and that is the only thing outstanding. It is one of exactly two bullets in that file carrying those words.

**What it is.** `Undefined` = v2 exhibits NO behaviour to be faithful to, so v3 is DESIGNING rather than porting or correcting. **One member, `intent config`**, and the measurement that opened it is the whole argument: v2's `intent config` produces **0 bytes on stdout, 0 bytes on stderr, exit 0**. There is nothing there.

**Why it is not `corrected`.** `corrected` means a v2 behaviour that is WRONG and gets fixed -- it needs an antecedent to correct, and **silence is not an antecedent**. Folding `config` into `corrected` would file a from-scratch design decision inside a bug-fix class, and those want different reviewers: a bug fix is checked against the old behaviour, a design is checked against nothing.

**What ratifying CHANGES, mechanically: nothing.** The row, the instrument and the class list are already consistent. Ratification only removes the "provisional pending hv" marker and closes one of my four open items. **What FOLDING would change: `config`'s row loses the distinction that says nobody has designed this yet, at the moment someone is about to.**

**My recommendation is ratify**, and the reason is the one thing that would actually cost us: **`config` is currently `disposition: pending` -- an honest blank -- and `undefined` is the only marker on that row saying the blank is because v2 was SILENT rather than because we have not looked.** Those two need to stay distinguishable while WP-07 is unbuilt.

---

**SEPARATELY, ON "THE SOONER WE CAN GET THIS PROJECT ONTO v3, THE BETTER" -- here is what actually stands between us and the cut, measured just now rather than estimated.**

**41 of 109 ACs. Four work packages PASS (WP-01 4/4, WP-02 8/8, WP-03 10/10, WP-05 6/6); eight are BLOCKED.**

```
WP-04  4/6     WP-07  0/6     WP-10  1/8
WP-06  6/11    WP-08  0/8     WP-11  2/4
               WP-09  0/5     WP-12  0/4
```

**The shape of the remaining work is one number: 52 of 109 acceptance tests are `to-write`.** Against 32 green, 19 `n/a`, 6 red. **Four whole packages -- 07, 08, 09, 12 -- are at ZERO, and 08 and 09 are the daemon and the MCP surface.** That is not a tail; it is roughly half the ladder, and your 3.0.0 scope statement adds WP-13's nine rows on top.

**Two things I can tell you that the numbers do not.** **WP-11 is 2/4 and both remaining rows were unblocked by your version ruling twenty minutes ago** -- dc has been deliverable-complete and idle waiting on exactly that, so those two should move without new work. **And the six `red` rows are honest reds, not unbuilt ones**: each names the missing arm on its own row, which is why they are red rather than parked at `to-write` where nothing would check them.

**I am not going to tell you it is close. It is not.** But the four passing packages are the foundation ones, and the failure modes we have been finding all day are instrument defects rather than model defects -- **the architecture has not moved under us once today**, which is the thing that would actually cost weeks.
