---
wp_id: WP-06
title: A session is told its seat at pickup and at launch
scope: S
status: Not Started
---

# WP-06: A session is told its seat at pickup and at launch

## Objective

This is the package that ends the retyping. A session that boots on a profiled board reads, from the pickup alone, which team it is on, its own seat, name, role and remit, the node hv is reached through, the hats it wears, and every other seat with its remit, whether that seat has ever run or not.

## Builder and judge

cc builds this package and claims `ST0092/06` in the same chain as `intent wp start ST0092/06`. vc judges it and claims nothing. Issue 0675 is built with it, because both change what a pickup prints.

## What is built

- **The brief in `intent wb pickup`.** Ahead of the board it prints the declared profiles, this node's name, role and remit, the mediator, and one line for each other seat with its name, role, remit and whether it has ever run. `--json` carries the same fields. The MCP tool answers the same.
- **The mediator's own brief.** For the mediator the route is not a node to go through. Its brief says that it is the mediator, that it reaches hv directly, and that the other seats' requests for hv come to it.
- **A seat that has never run** is listed like any other and marked never started. hv, 2026-11-10: "they all should be registered and ready to be used if needed." The human seat carries no such mark, because no session drives it: the rule reads a seat's recorded session, hv's row never holds one, and so the exemption keys on the board's declared human seat (WP-03).
- **How old a read is**, which is issue 0675. `intent wb pickup` prints how long ago the acting node last picked up on this machine, and `intent wb status`, which has no acting node, prints the same age on the row of every seat a session drives. It is read from the store's own `wb.pickup` events, and a pickup reads it before it writes its own. Those events are machine-scoped and never reach canon, so a store ingested fresh from canon holds none: there the line says that no pickup of the node is recorded on this machine, and prints no age. One arm pins what a fresh clone prints. Each peer's line gives the number of live messages addressed to that peer, labelled live.
- **`intent wb brief <moniker>`**, a read that writes nothing: it prints a seat's brief, and with `--launch` its launch steps, and `--json` carries the same. The launcher is a script and the brief is computed in the library, so the launcher needs a door to it. That door cannot be a pickup, which moves the heartbeat and records a session: the launched session's own pickup must be the one that does both, under its own session id.
- **The launcher.** `intent claude start <node>` hands the session the same brief through `intent/plugins/claude/bin/intent_claude_cwi`, which composes the session's context today and takes the brief from `intent wb brief <node> --launch`. Its refusal of an unregistered moniker stays and now names `intent wb init`.
- **The launch steps come from the profile.** Today `compose_ctx` ends every launch on two steps written for a session that hv directs: show hv a detailed plan, then stop and wait for hv's instructions. On a board with a mediator that is wrong for every seat but one. The steps are the profile's (WP-02): a seat other than the mediator sends its plan to the mediator and takes its go from it, unless hv speaks to it directly, and the mediator shows its plan to hv. The launcher's opening line calls the session a seat, the terms' word, where today it says a workstream node. A board that declares no profile launches as it does today, with the two steps unchanged, because nothing changes for a project until its own `wb init` (WP-05).
- **The profile's prose** from WP-02 is what the brief prints after the seats: how hv is reached and what each seat owes the others. It is the one source of those words, for the pickup, the launcher and the roster README (WP-07).

## What the spike showed

On 2026-11-10 the first swarm set up by hand carried its brief as an hv standing directive, which today's pickup prints in full to every node. It reached every pickup, and it printed last, below the node's own items and live messages, where a 900-word message buried it. Ahead of the board is the requirement, as AC-06.1 says.

It also showed that `intent wb register` stamps a heartbeat at registration, so `intent wb status` prints a seat that has never run exactly as one that ran and paused. The row already holds the difference: no session has ever been recorded for it, and on the 32 live boards 4 session seats read that way that day. So never started is read from the absence of any recorded session, never from status or heartbeat, and the time a seat was registered is not printed as a heartbeat.

Its cold boot, the same day, was a partial pass. A seat that was registered and had never run was launched after the brief was in place. From its pickup alone it read its role, its remit, the mediator and the route to hv, and it followed the route without stating all of it. It did not get its own moniker from the pickup. Its start prompt named the seat, and no pickup can infer one, because the verb takes the moniker as an argument. So the launcher is the only source of the seat, and of this package's two halves the launcher's is the one the proof rests on. By the seat's own account the boot was not a clean one: its start prompt told it to run a pickup and read its inboxes, and hv typed to it while it booted, though neither named the role, the remit, the mediator or the route.

The remit reached that seat only inside the brief, because its own board holds a name and a role and no remit (AC-03.7). And the brief's line for it said registered and never started, which was false by the time the seat read it. So whether a seat has run is computed at the read and is never written into the brief's text.

## What the hand form showed

On 2026-11-10 Intent's own board was put into this thread's form by hand (d09791705): the brief as hv's directive 2, the roster README, and a "Your seat" section in `.claude/restart.md`. cc read it as this package's builder, and vc took its findings under hv's pen.

- The launcher's own steps contradicted the brief, and the hand form needed two patches over them: a paragraph of the directive, and the section in `.claude/restart.md`. That is why the launch steps come from the profile.
- The hand form handled the mediator with a special case in prose, "a seat other than vc". That is why the mediator has a brief of its own.
- The board became a control for the never-started mark. `ac`, registered that day and never launched, printed as paused with a heartbeat, and `hv`, which no session drives, printed as active with a heartbeat eight weeks old. Neither row holds a session.
- The route to hv was written by hand three times, in three wordings. The generated brief replaces all three, and WP-11 says how each hand copy is retired on Intent's own board.

## Why this and not the README

A pickup reads no README and never has (hv's decision 28, ruling 5, on Conflab). The README is for a person reading the repository; the pickup is what a session reads at every boot.

## The proof that matters

AC-06.4 is not a test. A session is launched with `intent claude start <node>` on a converged board and hv types nothing to it. vc records, by session id, that it stated its own seat, which the launcher gave it, and its role, its remit, the mediator and its route to hv, which its pickup gave it.

## Rulings since this package was written, 2026-11-10

These are vc's rulings under the pen, made on ic's read of the dispatch rows and cc's read as builder, and written here once hv ratified the terms. Where one disagrees with the text above, it stands and the text above gives way.

The facade returns one row per seat, which the CLI's `wb status`, its `--json` and MCP's `wb status` all render. It carries the live item count and the live message count, each labelled live, the last pickup stamp, the never-started mark (for session-driven seats only) and the desk hat (WP-08). MCP stops counting archived items and handled messages. `wb brief <moniker>` is a read, offered on MCP, with no `--node`, and the help for `--launch` says "prints the launch steps; launches nothing". Pickup's help, its `--json` and MCP carry the brief, with one arm comparing the three. The fresh-clone line is a `note:`, exit 0. `claude start`'s remedy names `wb init`.

## Rulings on the second row read, 2026-11-10

These are vc's rulings under the pen on ic's read of this package against the dispatch table at d6f56d7e6. Where one disagrees with anything above, the section before it included, it stands and the text above gives way.

`wb brief` is a new row: read, arg `moniker` arity 1, `--launch` (on MCP, help "prints the launch steps; launches nothing") and `--json` (not on MCP, with the family's terminal-channel note), no `--node`, on MCP.

Three edges of `wb brief`, each with an arm. On a board that declares no profile it answers on exit 0 with today's two steps and the node's name and role, and never refuses, so an unprofiled project's `claude start` is unchanged. For a retired seat it refuses on exit 1, naming the retirement with `wb register` as the remedy, and the launcher passes the refusal through. For the human seat, `--launch` refuses on exit 1 saying the seat is the human's, and without `--launch` it prints the seat's line. An unknown moniker refuses on exit 1 naming the registered seats.

The INV-09 arm covers four faces from one library function: pickup's prose, its `--json`, MCP's answer and `wb brief`. The fresh-clone line is a `note:`.

The launcher half is ic's, as a sub-unit of this package: `intent/plugins/claude/bin/intent_claude_cwi` takes its context from `wb brief <node> --launch` in place of the two hard-coded steps, passes the three edges through, and `claude start`'s unregistered-moniker remedy names `wb init`. ic builds it once `wb brief` is banked. cc keeps the package's claim and builds the rest.

## Depends on

WP-03 for the declaration and the remit; WP-08 adds the hats line.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-06` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
