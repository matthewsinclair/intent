---
wp_id: WP-03
title: wb init reconciles a board to its profile, and is a dry run until told to apply
scope: M
status: Not Started
---

# WP-03: wb init reconciles a board to its profile, and is a dry run until told to apply

## Objective

`intent wb init <profile>` brings a board to what its profiles declare and touches nothing else on it. Without `--apply` it writes nothing and says what it would do. It works where a project has no whiteboard yet and on a board that has been live for months, and running it again is the reinit hv asked for.

## Builder and judge

cc builds this package and claims `ST0092/03` in the same chain as `intent wp start ST0092/03`. vc judges it and claims nothing.

## What is built

- **`intent wb init <base> [<layer>...] [--apply]`**, in the facade, as a coordinator over the identity door that already exists. For each seat the profiles declare it registers an absent one (`wb_register`), corrects a name or role that differs (`wb_correct`) and leaves a matching one alone. A registered node that no profile names is reported as outside the profile and left as it is. It is the third way onto the roster beside the two `intent wb register` has: one node from its arguments, and every node from its own board header.
- **The declaration.** The board's base, its layers, its mediator and its human seat are recorded in the store and as canon on disk under `intent/whiteboard/`, so a clone ingests the same team. The declared human seat is where every reader takes a seat's driver from: that seat is the human's, and every other seat is a session's, one that no profile names included. The same file holds the roster README's baseline, the hash each render of it records (WP-07). The canon is one file, rendered from the store as a `board.json` is and never edited by hand, and `intent doctor` reports skew in it. `intent wb init --apply` with no profile named re-applies the declaration: that is the reinit.
- **The remit.** The node row gains a remit, on the seat's own row and nowhere else, and it holds only a remit the project set. An apply writes no remit. `intent wb remit <moniker> "<text>"` sets a project's own and re-renders the roster README, and `intent wb remit <moniker> --default` removes it. A seat with none shows its profile's default wherever a remit is shown, read from the installed profile at that moment and marked as the default. So a row never has to say where its remit came from, and a default that a later release rewords reaches every project that kept it, with no apply. The dry run names every seat whose remit would be the profile's default, because no board has a stored remit today and 26 state their own in README prose. The first swarm set up by hand found on 2026-11-10 that a first apply would state each remit twice and differently, and that its own wording for cc is narrower than the default in the way that separates cc from ic on its board.
- **`register` honours the declaration.** A moniker the declared profiles name registers only with the profile's name and role, and the refusal prints them. A moniker they do not name registers as it does today. This is what stops the drift where it starts: 38 of 130 rows fleet-wide carry a role typed in as the moniker.
- **The dry run.** For each seat one of `register`, `correct` with the old and the new value, `unchanged` or `outside the profile`, then every view it would write or remove.
- **The apply's report.** Every path on a `wrote:` or `removed:` line, and one line saying that this is the roster's own mechanical commit, the carve-out in the protocol's ninth invariant, made by the coordinating node or hv. The vc that relayed hv's ask measured the reach by hand on 2026-11-10: registering `hv` wrote ten files across four boards.
- **The refusals**, each a typed error with a remedy: a hand-authored node that has not been migrated (`intent wb migrate`); a base other than the one declared (`--change`, WP-04); an unknown profile, with the shipped names; a layer on a board with no base.
- **The schema.** `board.json` moves on from `intent/board@4.0` for the new fields. A build of this thread reads a board at 4.0 unchanged. The new shape is not bent to stay readable by 3.2.8, whose `Board` and `WbNode` refuse an unknown field (vc's ruling of 2026-11-10 on cc's design note).
- **The surface rows** for `wb init` and `wb remit`, read by ic.

## What it never touches

Items, messages, handled marks, claims, archives, heartbeats, status and focus. AC-03.3 compares the bytes.

## Risks

- An apply is many roster writes at once, and a running intentd ingests disk over store after a quiet period (issue 0216). The apply is one store transaction and one landing of views, and its test reads the result back after the ingest.
- A registration re-renders every other board's inbox set, so the apply's file list is long on a large team. It is printed whole.
- An Intent that predates the new schema will meet these boards on any machine that has not upgraded. WP-11 measures what the released Intent does with one before any project is converged.
- A build of this thread moves a project's store forward the first time it opens it, by a read or a dry run as much as by an apply, and every older reader of that store is then refused (cc's measurement of 2026-11-10: the migration runs in `Store::open`). So until the cut, this package and every one after it is driven on a clone of Intent's board, and never on the real one while a 3.2.8 session reads it.

## Depends on

WP-02.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-03` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
