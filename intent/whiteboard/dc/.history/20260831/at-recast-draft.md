# DRAFT for vc -- the `at.recast` documentation half. NOT LANDED, NOT CANON.

Proposed home: `docs/concepts/criteria-and-tests.md`, immediately after the status table and the `red`-before-`green` paragraph at `:78-80`. It belongs there because that paragraph already tells the reader to move a row through `red`, and this is the cost of taking that advice.

Everything below was driven against `intent 3.0.0 (a854d7c3)` in a tree that binary created. **Two claims on my own board did not survive the drive and the draft states what the tool does instead** -- see the note at the end.

---

## The proposed text

**`to-write` is a state you can leave and cannot return to.** The three status verbs are `at green`, `at red` and `at na`, and none of them spells `to-write`. Nor does anything else: `at` ships `list`, `lint`, `green`, `red`, `na`, `new` and `edit`, and the return transition is declared in the model without a command that reaches it.

That matters because the natural correction is the one that does not work:

```
  $ intent at new ST0001 AT-03.1 --covers AC-03.1 --file tests/eviction.rs
  error: ST0001 already has acceptance test AT-03.1, and a create must not replace it
    remedy: nothing was written and AT-03.1 still holds its file, coverage, status and note.
```

**The refusal is right and its remedy is the route.** A create would have eaten the note; `at edit` keeps it.

**The way out is to write the artefact the row was owed.** A row is not stuck because its status is wrong -- it is stuck because it is now claiming something that is not yet true, and the repair is to make it true rather than to relabel it:

```
  $ printf '// AT-03.1 -- evicts oldest entries under memory pressure\n' > tests/eviction.rs
  $ intent at edit  ST0001 AT-03.1 --file tests/eviction.rs
  $ intent at green ST0001 AT-03.1
  $ intent ac gate  ST0001
  gate: ST0001 PASS -- 4/4 satisfied
```

**Re-citing to a file that already exists does not shortcut this, and the tool will not stop you at the time.** `at edit` accepts any path and answers `ok: AT-03.1 re-cited` whether or not that file carries the row's own id. The check lives at the gate, so a re-cite that looks like it worked surfaces later as a criterion that will not satisfy:

```
  gate: ST0001 BLOCKED -- 3/4 satisfied; unsatisfied: AC-03.1
```

**`at lint` will not warn you either** -- it reports `ok -- N AT row(s) conform` for a row citing a file with no id in it, because lint reads the row's grammar and the citation rule is about the file.

**One thing you cannot work out from the tool, and it decides which file you write.** A red row's cited artefact sits in whatever the project's default test run is. So citing a file inside that run does not merely record your own red -- it turns the whole run red for everyone who works on the project, including people with no interest in this criterion. A row that must sit red for a while should cite an instrument that runs on demand rather than one the default run collects.

---

## Notes for you, not for the page

**Two claims of mine were wrong and the draft follows the drive.** I had recorded that re-citing to an existing instrument is _correctly refused because the cited file must carry the literal row id_. It is not refused: `at edit` returns `ok:` and the gate catches it afterwards. What was refused in the `AT-10.5` instance must have been the gate, and my board compressed _the gate refused_ into _re-citing was refused_ -- a materially different operator experience, since one stops you and the other lets you believe you are done.

**A candidate defect I am not filing, because you took the last one on the same grounds.** `at lint` passes a row whose citation the gate refuses. Lint is advertised as checking rows against the grammar, so this may be correct scope -- but an operator who reads `lint: ST0001 ok -- 4 AT row(s) conform` has been told their rows are fine by the verb whose name means _check my rows_. If you want it filed I will; if you would rather it went in as a sentence on the page, say which.

**Deliberately not included:** any figure. The board carries _15 of 77 to-write rows carry no citation_ from your own earlier correction, and I have not re-derived it today, so it is not in the draft.
