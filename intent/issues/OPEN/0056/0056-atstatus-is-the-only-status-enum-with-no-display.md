---
id: "0056"
title: AtStatus is the only status enum with no display(), so v3 writes 'status: n-a' into the generated acceptance.md and v2's own linter rejects it L1 -- three of four variants coincide, which is what hid it
date: 2026-08-17
reporter: matts
status: OPEN
severity: high
---

# 0056: AtStatus is the only status enum with no display(), so v3 writes 'status: n-a' into the generated acceptance.md and v2's own linter rejects it L1 -- three of four variants coincide, which is what hid it

## Tags

model, views, canon, parity, at, display, measured, migration-hazard

## Summary

`AtStatus` (`model.rs:740-749`) has **no `impl` block and no `display()`**. `ThreadStatus` (`:238`), `WpStatus` (`:432`) and `IssueStatus` (`:789`) each have one. So every path that renders an AT status falls through to `enum_str`, which is the serde spelling -- and serde spells the non-test status `n-a` while **v2 spells it `n/a` everywhere: in the row grammar, in the linter's vocabulary, and in 23 rows of this repository's own canon.**

**TWO LIVE SITES, and the one that bites soonest needs no migration at all.** `render.rs:878` prints the token straight to stdout on `intent at list` -- a bare read of a `keep`-classified command, which is the centre of the parity contract:

```
$ intent at list ST0001
AT-01.1  n-a  covers AC-01.1
AT-01.2  green  covers AC-01.1
```

**And `views.rs:459-461` writes the AT row into the generated `acceptance.md` with the same call**, so `intent sync --to-disk` emits a row that v2's linter refuses:

```
lint: L1 AT-01.2 -- unreadable AT status: 'n-a' -- vocabulary is to-write | red | green | n/a
lint: ST0001 FAILED -- 1 finding(s) over 2 AT row(s)
```

**Three of the four variants coincide between the two spellings, and that is the whole reason this survived.** `to-write`, `red` and `green` are byte-identical in serde and in v2. Only `Na` diverges. A reviewer checking that the AT vocabulary round-trips would sample any of the other three and find agreement.

**It is the same defect cc fixed on `wp show` hours earlier, a third entity along, and the codebase says so about itself.** The doc comment on `IssueStatus::display` reads:

> **On the type for the reason [`ThreadStatus::display`] is**, and it was the same defect one entity over: `render.rs` spelled this as `enum_str(&status).to_ascii_uppercase()` at two sites, so the uppercase convention lived in the CLI crate while every other status vocabulary lived here.

Two entities were fixed by moving the human spelling onto the type. `AtStatus` is the third and was not.

Found by ic, 2026-08-17, measuring `target.no_op` for `at na` and noticing the printed token appears in no canon file in the estate.

**CONFIRMED INDEPENDENTLY BY vc the same day, with two details that make it worse than first written.**

**`n-a` is an EXPLICIT rename, not a derivation.** `model.rs:747` carries `#[serde(rename = "n-a")]`, and the container's `rename_all = "kebab-case"` would produce `na` for `Na` on its own. So the token is not kebab-case fallout that nobody noticed -- **somebody chose `n-a`, deliberately, against a canon vocabulary that spells it `n/a`.** Whatever that choice was for, it was not for the generated view, and nothing recorded it as a wire-only spelling.

**And the doc comment two lines above states the wrong token as though it were the vocabulary.** `model.rs:745` reads _"`n-a` is not green"_ -- prose about the human-facing meaning, written in the machine spelling. **That is the sibling class again, inside the enum that causes it**: a defect written down as the specification, so checking the code against its own documentation finds agreement.

**The 23 rows split 20 / 3, and the 3 are the ones that matter for the argument.** Twenty are in ST0056, the live thread; **three are in `COMPLETED/ST0054`.** So this is not confined to work in progress -- regenerating a COMPLETED thread's view produces a file v2's linter rejects, on a thread nobody is editing and nobody would think to re-lint.

## Reproduction

Measured against a release binary built from a `git archive HEAD` extract at `ae3e308f`, whose `native/` is bit-identical to `b7e60fc5`.

**The no-op line**, which is how it was noticed:

```
$ intent at na ST0001 AT-01.1     # second call, the self-loop
ok: AT-01.1 already n-a           # exit 0
```

**The generated view, which is the damaging one.** A thread whose canon carries one non-test AT:

```
$ intent sync --to-disk
ok: extract written for 1 thread(s)

$ grep AT-01.1 intent/st/ST0001/acceptance.md
- AT-01.1 (non-test)  -- covers AC-01.1 -- status: n-a
```

**v2's verdict on that exact generated line**, dropped into a v2 project and linted by v2:

```
$ intent at lint ST0001
lint: L1 AT-01.2 -- unreadable AT status: 'n-a' -- vocabulary is to-write | red | green | n/a (markdown emphasis around the value is NOT parsed)
lint: ST0001 FAILED -- 1 finding(s) over 2 AT row(s)
```

v2's grammar admits exactly one non-test token (`bin/intent_acceptance:181`):

```
AT_GRAMMAR_NONTEST="^- ($AT_G_ID) \(non-test\) (.*) -- covers ($AT_G_COVERS) -- status: (n/a)$AT_G_NOTE\$"
```

And the estate today:

```
$ grep -rho 'status: n/a' intent/st/ | wc -l
23
```

## Root Cause

One missing method, and a hiding mechanism that is worth more than the method.

**The missing method.** Three status enums carry a `display()` that states the human spelling on the type, deliberately, so it cannot drift into the CLI crate. `AtStatus` has none, so `enum_str` -- a serde-derived spelling intended for the wire, not for a person -- reaches both stdout and the generated canon.

**The hiding mechanism: a four-variant vocabulary in which three variants are fixed points.** Any check that samples the enum, any test that round-trips a status, any reviewer reading the variant list sees agreement three times out of four. **The one divergent variant is also the rarest in practice** -- `n-a` is non-test rows only -- so the sampling bias runs the same direction as the coincidence. This is why the defect is not visible from the type: the type looks like the other three, and three quarters of its values behave like the other three.

**The layer distinction that makes it high rather than medium.** The `wp show` instance and the `at na` no-op line are both READ paths: wrong word, right state, recoverable by reading again. `views.rs` is a WRITE path. Under D01 the DB is the SSOT and `acceptance.md` is a generated view, so `sync --to-disk` is how the human-readable canon comes into existence -- and it is currently the mechanism by which a v2-unreadable token enters files that are committed.

## Impact

**Running `intent sync --to-disk` on this repository would make every non-test AT row unlintable by the tool that is still installed.** 23 rows carry `status: n/a` today. v2 remains the `intent` on PATH for every session on this machine and is the only tool that can currently close a thread.

**And L1 findings BLOCK, which is what turns a spelling into an outage.** v2.19.0 shipped `intent at lint` with L1-L5, and v2.12.0's close-gate work made malformed AC/AT lines block rather than be silently dropped -- deliberately, because a silent drop produced a vacuous green. So the failure mode is: v3 regenerates the views, v2's gate refuses every affected thread, and `intent st done` / `wp done` stop working on them. **The two tools disagree about a four-character token and the consequence is that work cannot be closed.**

**`--fix` cannot rescue it and should not be relied on to.** `intent at lint --fix` REFUSES what it cannot migrate without loss -- that refusal is the centrepiece of v2.19.0 -- and even where it could rewrite the token, the next `sync --to-disk` puts it back. **A repair the generator overwrites is not a repair.**

Severity is high rather than critical only because v3 is not on `PATH` and nobody has run `sync --to-disk` against the live estate. It is a day-one migration bug and it is invisible until the first sync.

## Proposed Fix

**Give `AtStatus` a `display()` and route both call sites through it** -- the shape already used twice in the same file:

```rust
impl AtStatus {
  /// v2's spelling, for a human and for the generated view: the non-test
  /// status is `n/a`, which is the only token v2's row grammar admits.
  pub fn display(self) -> &'static str {
    match self {
      Self::ToWrite => "to-write",
      Self::Red => "red",
      Self::Green => "green",
      Self::Na => "n/a",
    }
  }
}
```

**Three call sites take `display()`: `views.rs:461` (the generated view), `render.rs:878` (`at list`, the second live site below) and the no-op line.** The serde spelling must NOT change -- `n-a` is the JSON canon's token, already committed, and the schema publishes it. The two spellings are correct and separate, which is precisely what `display()` exists to keep apart.

**THE GENERAL RULE FIRST WRITTEN HERE WAS WRONG AND IS REPLACED RATHER THAN ANNOTATED, because leaving both in one document is the defect this issue is an instance of.** The original said: _a status enum reaching a human or a generated view goes through `display()`, never through `enum_str`._ **That would red three correct sites and, worse, teach the fix that breaks it.** `TShirt` (`model.rs:349-357`) carries **no `rename_all` attribute at all**, so serde emits the variant names verbatim -- `XS`, `S`, `M`, `L`, `XL`, `XXL` -- which is exactly v2's spelling. `render.rs:645` and `:1426` print `TShirt` through `enum_str` and are correct; `intent wp list` renders `XL` and `intent wp rescope Huge` offers `one of: XS, S, M, L, XL, XXL`, both measured. **A check demanding `display()` there would have the next author write one that is a verbatim copy of the serde derive -- a divergent second spelling, introduced by the check written to prevent divergent second spellings, landing as a green improvement.**

**The property, stated over the ENUM SET rather than over call sites** (vc, 2026-08-17):

> For every enum that reaches human output, the serde spelling and the displayed spelling must be the same string -- either because they **coincide** (`TShirt`) or because a **`display()` maps one to the other** (`ThreadStatus`, `WpStatus`, `IssueStatus`). A `display()` is required exactly when they differ.

**This passes `TShirt` for the right reason rather than by exemption, and that distinction is the whole value.** An exemption list was the natural next move and would have been wrong: it would record "TShirt may skip `display()`", when the truth is that TShirt already satisfies the property. **An exemption states that a rule does not apply; this states that the rule is met.** Only the second survives someone later adding a `rename_all` to `TShirt`.

**And the method generalises past this enum: count where a defect MANIFESTS, not where it originates.** `enum_str` has ~50 call sites and only four can reach a human -- the rest are `store.rs`, `facade.rs`, `model.rs`, `doctor.rs`, where the JSON canon spelling is exactly what is wanted. **Four is small enough to read, so the property can be stated instead of proxied. A proxy is only worth having when the population is too large to state**, and reaching for one over a population of four is how the false positives got in.

## THERE IS A SECOND LIVE SITE, AND IT IS A DIRECT STDOUT PARITY BREAK RATHER THAN A GENERATED VIEW (vc, 2026-08-17)

**`intent at list` prints the same token, straight to stdout.** `render.rs:878`, read at HEAD:

```rust
for t in f.at_list(&st).map_err(fail)? {
  println!("{}  {}  covers {}", t.id, enum_str(&t.status), t.covers.join(", "));
}
```

`facade.rs:1936` -- `pub fn at_list(&self, st: &str) -> Result<&[AcceptanceTest], FacadeError>` -- so `t.status` is `AtStatus` and every non-test row prints `n-a`.

**This is the more directly in-scope half of the two.** The `views.rs` case runs through the generated-view deviation classes and needs a `sync --to-disk` to bite; **this one is the stdout of a `keep`-classified command, which is the centre of the parity contract, and it bites on a bare read.** So the fix has two call sites, not one, and the second needs no migration to expose it.

### AND THE GENERAL RULE ABOVE IS WRONG AS STATED -- IT WOULD MANUFACTURE THREE FALSE POSITIVES

Swept at HEAD: `enum_str` has **34 call sites over six files**, of which **FOUR can reach a human** -- `render.rs` (3: `:645`, `:878`, `:1426`) and `views.rs` (1: `:461`). The other 30 are `store.rs`, `facade.rs`, `model.rs` and `doctor.rs`, where `enum_str` is CORRECT because the JSON canon spelling is what is wanted. **But of the four, two are `TShirt` (`:645`, `:1426`) and they are correct too** -- leaving `views.rs:461` and `render.rs:878`, which are exactly the two defective sites named above.

**Those numbers are corrected from 42/6, which was mine and wrong** (ic caught it; the defective set never moved, and the correction is recorded because counts are load-bearing here). `grep -c 'enum_str'` counts **the `use` import at `render.rs:15` and the explanatory comment at `:692`** as call sites. That is the second time in one day a comment has been counted as data on this thread -- the first being `gen_register.sh:34`, a header comment narrating the very incident it was then reported as an instance of. **A needle whose subject is "lines containing the string" reported as "call sites" is the day's class in a `grep -c`**, and the fix is the boring one: exclude imports and comment lines, or read the four.

`TShirt` (`model.rs:349-357`) carries **no `rename_all` attribute at all**, so serde emits the variant names verbatim -- `XS`, `S`, `M`, `L`, `XL`, `XXL` -- which is exactly v2's spelling. **It has no `display()` and does not need one, because for this enum the two spellings coincide by construction.**

**So "no status enum reaches human output via `enum_str`" would red three correct sites and teach the next author to add a `display()` that is a verbatim copy of the serde derive** -- a divergent second spelling introduced by the check meant to prevent divergent second spellings. The rule has to target the DIVERGENCE, not the call:

> For every enum that reaches human output, the serde spelling and the displayed spelling must be the same string -- **either because they coincide (`TShirt`) or because a `display()` maps one to the other (`ThreadStatus`, `WpStatus`, `IssueStatus`).** A `display()` is required exactly when they differ.

That is a test over the ENUM SET rather than over call sites, it is stated as the property rather than as a proxy for it, and it passes `TShirt` for the right reason instead of by exemption.

## THE ROUND TRIP IS ASYMMETRIC, AND IT IS IDEMPOTENT FROM THE SECOND PASS -- WHICH DECIDES WHAT KIND OF TEST CAN CATCH IT (vc, 2026-08-17)

**The reader and the writer do not share a vocabulary, and only the writer is narrow.** `legacy.rs:608` ingests liberally:

```rust
"n/a" | "n-a" | "na" => AtStatus::Na,
```

while `views.rs:461` emits through `enum_str` and so has exactly one spelling. **So v3 reads `n/a` and writes back `n-a`: a round trip that does not return.**

**And the liberal reader is what hides it, in the precise sense that it removes the only signal.** Ingest of the live estate is genuinely clean -- `ingest --from-md` over a copy of this repository reads 56 threads / 140 WPs / 281 criteria / 228 ATs at **0 blocking, 9 carried, exit 0** -- and that measurement is CORRECT. It says nothing whatever about emit, because emit is a different layer, and I reported it to hv as part of a self-hosting assessment without measuring the other half. **The reader's tolerance means the corruption cannot announce itself on the way in; it is created on the way out and discovered by a third tool.**

**THE PART THAT MATTERS FOR TESTING: the corruption happens exactly ONCE and every subsequent check confirms stability.** `n-a` re-ingests to `Na` and re-emits to `n-a`, so from pass 2 the round trip is a fixed point:

```
v2 canon  n/a  ->  Na  ->  n-a      <- the only lossy step, and it is the first
          n-a  ->  Na  ->  n-a      <- stable, forever
```

**Therefore a round-trip stability test seeded from v3's own output can never see this defect** -- it would report a clean fixed point and be right. **The seed has to be the v2 canon**, ie a corpus v3 did not produce. This is the general shape and it is worth more than this instance: _a stability property measured from the system's own output confirms that the system is consistent with itself, which is exactly what a one-time normalisation preserves._ Same distinction as read-back being a consistency check rather than a correspondence one (`parity.md`).

**Consequence for the fix**: `display()` closes it, and a regression test must assert **byte-equality against a fixture written in v2's spelling** -- not that ingest-then-emit is stable, which it already is.

## Related

- ST0056 -- Intent v3.0.0
- 0050 -- measuring `target.no_op` for `at na` is how the token was noticed
- 0033 -- `intent at red/green/na` destroying the AT row note; same family, same rows
- `model.rs:740-749` -- `AtStatus`, no impl block
- `model.rs:238` / `:432` / `:789` -- the three enums that DO carry `display()`
- `model.rs:783-789` -- `IssueStatus::display`'s doc comment naming the class
- `views.rs:459-461` -- the write path
- `bin/intent_acceptance:181` -- v2's non-test row grammar, admitting `n/a` alone

## Resolutions

{{TBC}}

## THERE ARE THREE SPELLINGS OF THIS ONE STATE, TWO OF THEM INSIDE ONE COMMAND (ic, 2026-08-17)

**`intent at na` prints a different token depending on which arm it takes**, and neither is v2's:

| path              | prints                    | source of the word                                      |
| ----------------- | ------------------------- | ------------------------------------------------------- |
| v2 `intent at na` | `ok: AT-01.2 -> n/a`      | v2's vocabulary, and its row grammar's only legal token |
| v3 movement       | `ok: AT-01.2 na`          | **the CLI SUBCOMMAND NAME** (`render.rs:884-892`)       |
| v3 self-loop      | `ok: AT-01.2 already n-a` | `enum_str` via `Outcome::AlreadyThere`                  |

The movement arm never touches `AtStatus` for display at all:

```rust
Some((state @ ("green" | "red" | "na"), a)) => {
  let status = match state { "green" => Green, "red" => Red, _ => Na };
  reported(&open()?.at_set(&st, &id, status).map_err(fail)?, &id, state);
                                                                  // ^^^^^ the SUBCOMMAND NAME
}
```

**So `display()` alone does not close this.** It fixes `views.rs`, `at list` and the self-loop; the movement message would still print `na`, because it is echoing the verb the user typed rather than the state the entity reached. The third call site needs the enum, not the string.

**AND THE HIDING MECHANISM REPEATS AT A SECOND LAYER, WHICH IS WHY NOBODY SAW IT.** The subcommand names are `green`, `red`, `na`. Two of the three are byte-identical to v2's tokens, so echoing the verb name is correct for `at green` and `at red` and wrong only for `at na` -- **the same two-of-three coincidence as the serde vocabulary, arriving independently, in a different mechanism, on the same command family.** A reviewer checking that the movement message names the right state would sample `green` or `red` and find agreement.

## FIVE `as-observed` ROWS DO NOT REPRODUCE v2's STDOUT, FOUND BY EXECUTING THE REGISTER (ic, 2026-08-17)

Measured by `literal_stdout_parity.rs`, which drives each row's declared invocation against a v3 binary and compares it to v2's measured bytes:

| row            | v2                                        | v3                          |
| -------------- | ----------------------------------------- | --------------------------- |
| `ac satisfy`   | `ok: AC-01.1 satisfied by evidence`       | `ok: AC-01.1 satisfied`     |
| `ac rescope`   | `ok: AC-01.1 back in scope (unsatisfied)` | `ok: AC-01.1 back in scope` |
| `ac reinstate` | `ok: AC-01.1 back in scope (unsatisfied)` | `ok: AC-01.1 reinstated`    |
| `at red`       | `ok: AT-01.1 -> red`                      | `ok: AT-01.1 red`           |
| `at na`        | `ok: AT-01.2 -> n/a`                      | `ok: AT-01.2 na`            |

**Only the last is this issue.** The other four are separate, previously unrecorded parity breaks on rows whose `target.state` is `as-observed` -- the arrow dropped across the AT family, the resulting-state suffix dropped from the AC undo verbs, and `ac reinstate` rewritten entirely. **`at red`'s `observed` column was CORRECT the whole time**; nothing compared it to the binary, so the divergence sat unrecorded next to an accurate record of what it should have been.

They are listed here rather than filed separately because the fix for this issue touches the same arms, and whoever wires `display()` should decide all five voices at once. **Each is either a bug to fix or a deviation to ratify onto `corrected`; none is a template to edit.**
