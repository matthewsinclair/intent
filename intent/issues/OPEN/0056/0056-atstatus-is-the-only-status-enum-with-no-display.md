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

**This is not confined to a message. `views.rs:459-461` writes the AT row into the generated `acceptance.md` with `enum_str`**, so `intent sync --to-disk` emits a row that v2's linter refuses:

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

Then `views.rs:461` and the no-op line take `display()` rather than `enum_str`. **The serde spelling must NOT change**: `n-a` is the JSON canon's token, it is already committed, and the schema publishes it. The two spellings are correct and separate -- which is precisely what `display()` exists to keep apart.

**And the general form, since this is the third instance and the first two were each found one at a time.** The rule the codebase is reaching for is: _a status enum reaching a human or a generated view goes through `display()`, never through `enum_str`._ That is mechanically checkable -- `enum_str` has a small number of call sites -- and a test asserting that no status enum reaches `views.rs` or `render.rs` output via `enum_str` would have caught all three at once. Without it, the fourth is found the same way the first three were.

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
