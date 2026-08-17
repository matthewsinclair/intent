---
id: "0052"
title: WorkPackage.scope has a ratified machine with six edges and no way in or out from the command surface -- wp_rescope is implemented but unwired, and wp new takes no scope
date: 2026-08-17
reporter: matts
status: OPEN
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

The field can therefore be neither entered nor changed from the command surface. Every work package v3 creates sits at `absent` permanently, and `intent wp list` has a Scope column that can only ever be empty for anything v3 made.

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

**The entrance was never v3's to lose, and that is what hides it.** v2 has no scope setter either: `intent wp new` takes `<STID> "Title"` and nothing else, and `bin/intent_wp:243` only READS `scope:` back out of `info.md` to render the list column. **In v2 the scope was set by hand-editing the work package's frontmatter.** So a parity reading of `wp new` correctly concludes there is nothing to port -- the deviation is not in the command, it is in the truth model. Under D01 the DB is the SSOT and `info.md` is a generated view, so the v2 mechanism is not a command that went missing; it is a workflow the rewrite retires by design, and the replacement verb exists but was never connected.

`scope_legacy` shows migration was thought about carefully -- a v2 string nobody has adjudicated is carried rather than coerced, and `wp_rescope` resolves it. **That path is also unreachable, so a carried legacy value can never be resolved by the operator who owns it.**

## Impact

**Every v3-created work package has no scope and no way to acquire one.** T-shirt sizing is house practice on this project (XS/XL, never clock time), `intent wp list` renders a Scope column, and `intent todo` reads scope. On this repository, `wp list ST0056` today shows real sizes for all twelve packages -- every one of them migrated from v2. The first package created natively in v3 will show an empty cell that nothing can fill.

**And a migrated `scope_legacy` is permanent.** The value exists to be adjudicated later by a human who looks at it; the verb that adjudicates it cannot be called.

Severity is medium rather than high only because v3 is not on `PATH` and nobody is creating work packages with it yet. It becomes a day-one bug the moment somebody does, and it is invisible until then because every package that exists was migrated.

## Proposed Fix

Wire the verb, and decide the entrance separately -- they are two decisions and only the first is mechanical.

- **`intent wp rescope <STID/NN> <size>`** -- a render arm over the existing `wp_rescope`, which needs nothing else built. Its self-loop behaviour is already correct and already reasoned: a rescope to the same size is a no-op ONLY when there is no carried `scope_legacy`, because resolving the carry is a real movement with the same from and to (`facade.rs:1443-1456`). That is a genuinely good distinction and it currently has no user.
- **`intent wp new --scope <size>`, or a deliberate decision not to have it.** v2 had no such flag, so adding one is `new-surface` rather than parity, and it may be right to make scope a decision you take after writing the package rather than while naming it. Either answer is fine; the current state is the one that is not, because it is the answer nobody chose.

Both need dispatch-table rows before they ship. `wp rescope` will be `new-surface`, and it wants a `no_op` declaration from 0050 the day it lands -- its no-op condition is the most interesting one in the table.

## Related

- ST0056 -- Intent v3.0.0
- 0050 -- the `no_op` declaration this row would need; enumerating for it is how this was found
- 0037 -- the enumerator blind spot; a register cannot see a verb neither side declares
- `transitions.rs:366-374` -- the machine
- `facade.rs:1428-1470` -- the implemented, unwired verb

## Resolutions

{{TBC}}
