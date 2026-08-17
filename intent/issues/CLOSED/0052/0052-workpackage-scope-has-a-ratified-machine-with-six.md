---
id: "0052"
title: WorkPackage.scope has a ratified machine with six edges and no way in or out from the command surface -- wp_rescope is implemented but unwired, and wp new takes no scope
date: 2026-08-17
reporter: matts
status: CLOSED
severity: medium
---

# 0052: WorkPackage.scope has a ratified machine with six edges and no way in or out from the command surface -- wp_rescope is implemented but unwired, and wp new takes no scope

## Tags

surface, facade, unwired, machine, scope, work-package, measured, truth-model

## Summary

`WorkPackage.scope` is a ratified machine. It declares six edges, an `absent` initial state, and a facade verb with its own carefully-reasoned self-loop arm. **No user can reach any of it.**

- `Facade::wp_rescope` (`facade.rs:1428`) is `pub`, implemented, and its only callers in the whole workspace are two tests -- `mutation_completeness.rs:785` and `legacy_scope_carry.rs:197`. There is no `render.rs` arm and no `spine.rs` entry.
- `intent wp --help` lists seven subcommands: `new`, `start`, `done`, `reopen`, `unstart`, `list`, `show`. No `rescope`.
- `intent wp new --help` lists **no options at all**, only `-h, --help`. So the scope cannot be supplied at creation either.

The field can therefore be neither chosen at creation nor changed afterwards from the command surface.

**CORRECTION, and it changes what the entrance half of this issue says (vc, 2026-08-17, ruling at `e571f6bf`).** The first version of this summary said the scope cannot be entered at all and that every v3 work package "sits at `absent` permanently". **That is wrong.** `Facade::wp_new` takes a scope -- `pub fn wp_new(&mut self, st, title, scope: TShirt)` (`facade.rs:1353`) -- and the renderer supplies one: `f.wp_new(&st, &title, TShirt::S)` (`render.rs:574`), hardcoded, with its own comment explaining that `lib/templates/prj/st/WP/info.md` seeds `scope: Small` and a different default would write different canon for the same command. **So the value IS entered, the default is deliberate, and it is parity.** Verified from the commit with `git show` because both files were dirty at the time of reading.

**The correction makes the finding sharper rather than smaller, and moves it off the entrance and onto the exit.** Every work package v3 creates is `S`, chosen by nobody, with no way to change it -- so `intent wp list` will show a column of `S` rather than a column of blanks, which is worse: a blank is visibly missing and an `S` is a claim. **And it sits directly against the migration's own stated principle.** `scope_legacy` exists precisely so that a v2 value nobody has adjudicated is carried rather than coerced -- `transitions.rs` says the migration "carries that as `nobody recorded one` instead of substituting a medium". The migration refuses to invent a size for a package that has none; `wp new` invents one for every package it creates.

**The layer distinction is why this read as a conflict and neither statement was wrong.** The FACADE can enter all seven initial values and drive all six edges. The SURFACE can enter exactly one and drive none. `State` stays and the model is right; this is a surface gap, and nothing in `transitions.rs` changes.

**The commit that built the verb names this exact hazard and catches only half of it.** From `transitions.rs`, on the `wp.rescope` edges:

> closure check: a value the caller supplies at creation is ENTERED, so having no exit makes every one of the six sizes a trap.

The closure check asked whether the six sizes have an exit, found they did not, and added one. Nothing asked whether they have an ENTRANCE. **The premise of the sentence -- "a value the caller supplies at creation" -- is false at HEAD, and it is stated in the same breath as the fix it justifies.**

Found by ic, 2026-08-17, enumerating the rows that can self-loop in order to give each one a `no_op` declaration (issue 0050). `wp rescope` came out of the facade as a self-loop-capable verb and had no dispatch-table row to attach the declaration to, which is what exposed it.

## Reproduction

Measured at HEAD `a395dcdb`, against the release binary built from it.

```
$ grep -rn "wp_rescope" native/rust/crates/
native/rust/crates/intentsvcs/src/facade.rs:1428:  pub fn wp_rescope(...)      <- the definition
native/rust/crates/intentsvcs/tests/mutation_completeness.rs:785               <- test
native/rust/crates/intentsvcs/tests/legacy_scope_carry.rs:197                  <- test

$ intent wp --help          # v3 binary
Commands:
  new  start  done  reopen  unstart  list  show

$ intent wp new --help
Options:
  -h, --help  Print help
```

The machine, `transitions.rs:366-374`:

```rust
initial: &["XS", "S", "M", "L", "XL", "XXL", "absent"],
edges: &[
  Edge::direct("wp.rescope", &[], "XS"),
  ... six in total ...
],
```

And the dispatch table has no `wp rescope` row, which is consistent -- v2 had no such command -- and is why no parity instrument flagged it. `surface_check` compares the table to the binary and passes: both agree the command does not exist. **The register cannot see a verb that neither side declares**, which is the same enumerator blind spot as issue 0037, one layer along.

## Root Cause

Two independent gaps that happen to close over the same field, which is why neither looked like a problem on its own.

**The exit was built for the machine, not for the surface.** `wp_rescope` was added to satisfy a closure property of the ratified machine -- every state needs an outbound edge -- and satisfying it in the facade satisfies the machine. Wiring it to the CLI is a separate act that nothing required.

**The entrance was never v3's to lose, and that is what hides it.** v2 has no scope setter either: `intent wp new` takes `<STID> "Title"` and nothing else, and `bin/intent_wp:243` only READS `scope:` back out of `info.md` to render the list column. **In v2 the scope was set by hand-editing the work package's frontmatter, and v3 hardcodes the template's `Small` at `render.rs:574` to reproduce exactly that starting value.** So a parity reading of `wp new` correctly concludes there is nothing to port -- the default is faithful and the deviation is not in the command, it is in the truth model. Under D01 the DB is the SSOT and `info.md` is a generated view, so **what v3 removes is not a command, it is the hand-edit** -- the only mechanism v2 ever had for changing the value. That is a workflow the rewrite retires by design, and the replacement verb exists but was never connected. **Each half is defensible on its own and the pair is the defect**: reproduce v2's default faithfully, retire v2's only way to change it, and do not wire the verb that replaces it.

`scope_legacy` shows migration was thought about carefully -- a v2 string nobody has adjudicated is carried rather than coerced, and `wp_rescope` resolves it. **That path is also unreachable, so a carried legacy value can never be resolved by the operator who owns it.**

## Impact

**Every v3-created work package is `S`, chosen by nobody, and nothing can change it.** T-shirt sizing is house practice on this project (XS/XL, never clock time), `intent wp list` renders a Scope column, and `intent todo` reads scope. On this repository, `wp list ST0056` today shows real sizes for all twelve packages -- L, L, L, XL and so on -- **and every one of them was migrated from v2, so every one was sized by a human editing a file.** Packages created natively in v3 will all read `S`.

**A wrong value is worse here than a missing one, which is why the correction above raises the severity of this rather than lowering it.** A blank Scope column is visibly unanswered and invites someone to fill it. A column of `S` is a claim: it reads as twelve deliberate sizing decisions, and on a project where sizing is house practice it will be believed. **The migration refuses to invent a size for a package that has none, on the explicit grounds that substituting a medium is worse than recording an absence; `wp new` invents one for every package it creates.** Those two positions are in the same codebase and only one of them was argued.

**And a migrated `scope_legacy` is permanent.** The value exists to be adjudicated later by a human who looks at it; the verb that adjudicates it cannot be called.

Severity is medium rather than high only because v3 is not on `PATH` and nobody is creating work packages with it yet. It becomes a day-one bug the moment somebody does, and it is invisible until then because every package that exists was migrated.

## Proposed Fix

Wire the verb, and decide the entrance separately -- they are two decisions and only the first is mechanical.

- **`intent wp rescope <STID/NN> <size>`** -- a render arm over the existing `wp_rescope`, which needs nothing else built. Its self-loop behaviour is already correct and already reasoned: a rescope to the same size is a no-op ONLY when there is no carried `scope_legacy`, because resolving the carry is a real movement with the same from and to (`facade.rs:1443-1456`). That is a genuinely good distinction and it currently has no user.
- **`intent wp new --scope <size>`, or a deliberate decision not to have it -- for hv, not for a renderer.** The correction above narrows this question usefully: the default already exists, is faithful to v2's template, and is argued for at `render.rs:570-573`, so the question is not "does scope get entered" but **"can the operator override a default chosen for parity reasons rather than for their package"**. It is a workflow call -- does sizing happen while you name a package, or after you have written it -- and both answers are defensible. **If the answer is `no flag`, `wp rescope` becomes the entrance as well as the exit, and the hardcoded `S` becomes an honest starting value rather than a silent verdict**, which is a cleaner model than a flag. The current state is the only one that is not defensible, because it is the answer nobody chose.

Both need dispatch-table rows before they ship. `wp rescope` has one as of `ba513915` -- added so the arm can be wired at all, since the spine builds clap FROM the table -- and it carries the `no_op` declaration from 0050 already: its no-op condition is the only one in the population not decidable from the two states alone.

**And one thing NOT to do: do not make `wp new` write `absent`.** It would remove the invented `S` and look like the principled fix, but it breaks parity with v2's template for no gain, and it trades a wrong value for a missing one at the exact moment the exit verb is being wired to fill it.

## Related

- ST0056 -- Intent v3.0.0
- 0050 -- the `no_op` declaration this row would need; enumerating for it is how this was found
- 0037 -- the enumerator blind spot; a register cannot see a verb neither side declares
- `transitions.rs:366-374` -- the machine
- `facade.rs:1428-1470` -- the implemented, unwired verb

## Resolution -- CLOSED 2026-08-17, on hv's ruling: NO FLAG, `wp rescope` IS THE ENTRANCE

**hv, 2026-08-17: _"0052 no flag, wp rescope is the entrance"._** That is the second of the two options this issue's Proposed Fix set out, and it closes with no code to write.

**Both halves are now settled and neither needed the other's answer to land.**

- **The mechanical half shipped** (cc, `4a0c905c`). `intent wp rescope <SPECIFIER> <SIZE>` is wired -- verified at HEAD, not taken on report: the render arm is at `render.rs:639`, bound through `reported(...)` so it speaks the no-op grammar 0050 ruled, and `intent wp --help` lists `rescope` among the eight verbs. The T-shirt vocabulary has one home in `TShirt::parse`, derived from the enum's own serialisation.
- **The entrance half is ruled rather than built.** `wp new` takes `<STID> <TITLE>` and no flags -- confirmed from the register at HEAD (`args=stid,title`, `flags=` empty). It stays that way.

**What the ruling buys, in the issue's own terms.** The hardcoded `S` was the part that could not be defended: not because `S` is the wrong value -- it is right, and `render.rs:580-585` argues it correctly on parity grounds, since v2's `lib/templates/prj/st/WP/info.md` seeds `scope: Small` and a different default would be a parity break hiding in a value rather than in an output -- but because **a default nobody can override is a verdict, and a default with an exit is a starting value.** `wp rescope` is that exit. The same `S` now means something different without changing.

**The `absent` trap was avoided and should stay avoided.** This issue's Proposed Fix named it explicitly: making `wp new` write `absent` would look like the principled fix, breaks parity with v2's template for no gain, and trades a wrong value for a missing one at the exact moment the exit verb is being wired to fill it. The ruling does not reopen that.

**The self-loop distinction this issue flagged as having no user now has one.** `wp_rescope` treats a rescope to the same size as a no-op ONLY where there is no carried `scope_legacy`, because resolving the carry is a real movement with the same from and to (`facade.rs:1443-1456`). With `rescope` as the sole entrance, that is the mechanism by which a migrated `scope_legacy` gets adjudicated by a human instead of having a size coerced onto it -- which was the capability the unwired verb was denying.

### OWED, and it is the reason this closure is not silent

**`render.rs:633-634` still says _"The field's only exit, and until hv rules on `wp new --scope` its only entrance too"_.** hv has ruled, and the ruling makes it the entrance permanently rather than provisionally. **That comment is now stale in the worst direction: it tells the next reader the question is open when it is closed**, at the exact site where someone would go looking to answer it.

**Not edited here.** `render.rs` is dirty under cc as this is written, and a peer's in-flight file is not vc's to touch. Handed to cc as a one-sentence change.

**Recorded as an instance rather than a chore, because it is AC-05.5's subject exactly**: a decision taken in one contract artefact and absent from another, with both individually correct when read alone. The issue carries the ruling; the code comment carries the question; nothing joins them. The register was checked and carries no `open_question` on either row, so the comment is the only stale artefact -- which is only knowable because it was looked for.
