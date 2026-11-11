---
wp_id: WP-07
title: The roster README is rendered from the profile and keeps the project's own block
scope: M
status: WIP
---

# WP-07: The roster README is rendered from the profile and keeps the project's own block

## Objective

`intent/whiteboard/README.md` becomes a generated view with one writer. `intent wb init --apply` renders its upper part from the declared profiles and the store, and everything below a marker is the project's own and survives every re-render. A profile change reaches a project at its next `wb init`, and a project loses none of what it wrote.

## Builder and judge

cc builds this package and claims `ST0092/07` in the same chain as `intent wp start ST0092/07`. vc judges it and claims nothing.

## What was measured

26 of the 32 live boards have a roster README and 6 have none. 5 carry a verblock. Intent's own says, in its own text, that it has no single writer and that its rows lag. The function that keeps a preserved block across a re-render already exists, `carry_user_block` in `native/rust/crates/intentsvcs/src/canon.rs`, wired today to `CLAUDE.md` alone. It carries what lies between `<!-- user:start -->` and `<!-- user:end -->`, and it returns the regenerated file unchanged when the existing file has no such block.

## What is built

- **One template** under `lib/templates/`, rendered through the token expander (`intentsvcs::rootfiles::substitute`): a title and one opening paragraph that say what the file is, the declared profiles, the roster table with each seat's moniker, name, role, who drives it and remit, the route to hv, and the verb that prints the terms. The roster table arrives as one token filled from the store, so the expander needs no new block kind. Its columns are padded to their widest cell, as the project's formatter pads a table, so a formatter run over the tree changes nothing in the generated part and is never its second writer. Each seat's driver comes from the board's declared human seat (WP-03), and a seat with no remit of its own shows its profile's default, read at the render and marked as the default. The route to hv is the profile's own section (WP-02), printed whole, and it names the mediator as the one reader of hv's inbox.
- **The preserved block.** The template ends on a block between `<!-- user:start -->` and `<!-- user:end -->`, the pair `carry_user_block` carries, and the project's own prose lives inside it. The function is reused, not copied. The README does not take the branch that guards `CLAUDE.md`, which holds back a file that carries no generated footer: a first render meets exactly such a file and must go on.
- **The first render.** A first render is one over a README that Intent has never rendered. The render's own record says so, and the presence of a block does not: each render records, in the declaration's canon file (WP-03), the sha256 of the README's text outside the block as it rendered it, and a README with no such record has never been rendered. It first places the whole existing file inside the block and then carries it, and the verb says so. A plain reuse of the function would lose every hand-written line, since it carries nothing when the existing file has no block. No hand-written line is lost, and the project prunes what the generated part now says. One exception, found on 2026-11-10 in the first swarm set up by hand in this thread's form: 5 of the 32 live READMEs open with a verblock, and a frontmatter block carried into the block would sit in the middle of the file. A generated view carries no verblock, so the first render does not carry a frontmatter block that opens the existing file: it drops it and names it on its own line of the report.
- **A remit written by hand.** No board has a stored remit today, and 26 state their seats' remits in README prose. A first render that filled every remit from the profile would state each seat's remit twice and differently, the default in the generated part and the project's own inside the block. So a seat whose remit was never set shows the profile's default marked as the default, and the sweep sets each board's own remits in the same step as its apply (WP-11).
- **Skew.** A hand edit outside the block is skew that `intent doctor` reports. An apply refuses to render over it, before anything is written, and names the lines and the remedy, which is to move them into the block: restoring the file would delete what someone wrote, and this package's promise is that a project loses none of it. A README that is only out of date, because a newer Intent renders it differently, re-renders. A render compared with the file cannot tell the two apart, and the recorded hash can: text outside the block that no longer matches the hash is an edit, and text that matches the hash but not this build's render is only out of date. The file index cannot stand in for the record, which cc measured on 2026-11-10: it lists every file a scan saw, whoever wrote it, so a hand-written README is already in it; every scan rebuilds it, so an edit is its baseline one scan later; and it lives in one machine's store and never reaches a clone. The record travels with the declaration, so a clone answers as the machine that rendered the README does.
- **The markers.** The block runs from the first start marker to the first end marker after it. A stray marker would therefore move the block's end and drop what follows it, and a README that only mentions the two markers would read as holding a block. So a render refuses, naming the line, a README whose markers are anything but one start followed by one end, and a first render refuses one that holds either marker at all.
- **Nothing guessed.** Every generated line has a declared source, the profile or the store. This is the lesson of issue 0672, where a template wrote a language's default build lines into a contract and was wrong in most of the fleet. AC-07.4 states it as a test.
- **No run state.** The roster table says nothing about whether a seat has run. The file is committed, and a seat's first pickup does not render it, so a mark there would be stale from that pickup on. The pickup is where run state is printed (WP-06).

## What the hand form showed

On 2026-11-10 Intent's own README was written by hand in this package's form (d09791705). cc read it as this package's builder, and vc took its findings under hv's pen.

- Its title and its opening paragraph, which give the history of the threads that built the whiteboard, sit where the generated part will go. The generated part has a title and an opening of its own. The history moves inside the block with the rest of the project's prose, because nothing in the profiles or the store declares it (AC-07.4).
- It names the one reader of hv's inbox. After WP-05 that reader is the mediator, so the generated route says so.
- It has no block, so it is a control for the first render: every line of it must come through inside the block.

## What it is not

It is not where a session learns its seat: WP-06 is. It is not issue 0672's fix, which changes `lib/templates/llm/_AGENTS.md` and is ruled separately.

## Rulings since this package was written, 2026-11-10

These are vc's rulings under the pen, made on ic's read of the dispatch rows and cc's read as builder, and written here once hv ratified the terms. Where one disagrees with the text above, it stands and the text above gives way.

A hand edit outside the block is a new doctor class, `roster-readme-edited`: shown, never counted, naming the lines and the remedy. The hard stop stays at `wb init --apply`. Out-of-date is `StaleRender`, with an arm proving it is never counted. The dropped verblock is a `note:`.

## Rulings on the second row read, 2026-11-10

These are vc's rulings under the pen on ic's read of this package against the dispatch table at d6f56d7e6. Where one disagrees with anything above, the section before it included, it stands and the text above gives way.

Out-of-date is NOT `StaleRender`, which the section above said. It is a class of its own, `roster-readme-behind`, with the remedy "`intent wb init --apply` re-renders it; only text the renderer owns differs". StaleRender's remedy names `intent sync`, which is the wrong verb for a file whose only renderer is `wb init --apply`, and `RootFileBehind` is about a root file canon writes. The new class joins `is_shown_by_default`, and an arm proves it is never counted.

`roster-readme-edited` is shown and never counted, while `ViewSkew`, a hand edit to any other generated view, is counted. That is deliberate: the README's block is the project's, and the hard stop sits at `wb init --apply`. Do not fold it into ViewSkew.

Both ids live in `FindingClass::as_str`. No new row and no new flag: `wb init`'s help carries the README's refusals, and a dry run reaches them (WP-03).

## Depends on

WP-03.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-07` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
