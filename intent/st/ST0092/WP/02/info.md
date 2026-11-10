---
wp_id: WP-02
title: A Team Profile is a shipped resource: team, solo, supervisor and supervised
scope: M
status: Not Started
---

# WP-02: A Team Profile is a shipped resource: team, solo, supervisor and supervised

## Objective

The four profiles ship in Intent's plugin canon as files of config and prose under one schema, and the installed tool lists, shows and validates them the way it serves the rule library. No project vendors a profile, so a project names one and never copies it.

## Builder and judge

cc builds this package and claims `ST0092/02` in the same chain as `intent wp start ST0092/02`, once AC-01.5 is recorded. vc judges it and claims nothing.

## What is built

- **The seat registry**, one JSON file beside `TERMS.md` in the plugin canon under `intent/plugins/claude/`, read by a typed, strict reader. It holds each seat once: its moniker, its name, its role, who drives it and its default remit. hv's row is written once, and a moniker cannot carry two names.
- **The profile files**, beside the registry, one `.md` per profile. The frontmatter is flat: the profile's kind (a base or a layer), the mediator it sets, and the monikers of its seats. The body is the brief, the prose a session is given: what the team is, how hv is reached, what each seat owes the others, and what a seat does at launch. A base's body carries four sections that are printed whole and stated nowhere else: the route to hv and the launch steps, each written once for the mediator and once for every other seat. The pickup's brief, the launcher and the roster README choose between the two by whether the seat is the declared mediator (WP-06, WP-07). The sections name the mediator from the board's declaration and never as a moniker written into the prose, because a layer can move it. A layer's body carries none of the four: it is the layer's own prose, printed after the base's, so a board that declares a base and a layer has one source for each section.
- **The four profiles.** `team` is a base: vc, cc, ic, ac, dc and hv, with vc the mediator. `solo` is a base: gc and hv, with gc the mediator. `supervisor` is a layer for a hub: it adds sc and makes sc the mediator. `supervised` is a layer for a project under a supervisor: it adds sv and leaves the mediator alone.
- **A schema** for the registry and one for the profile file, beside the rule schema, and a loader in the library crate (`native/rust/crates/intentsvcs/`) whose every failure is a typed error with a remedy.
- **Four read-only verbs**, `intent wb profile list`, `intent wb profile show <name>`, `intent wb profile validate` and `intent wb profile terms`, which prints WP-01's document. The dispatch is in `intent-cli` and the logic in the library. They answer from the install and need no project.
- **Validation** refuses a base with no mediator, a mediator that is not one of the profile's seats, a duplicate moniker, a role equal to its moniker, a name written as a session name, a second seat driven by the human, a layer that declares a human seat, a profile that names a moniker the registry does not hold, a base that lacks any of its four sections, and a layer that carries one.
- **The parity test** of AC-02.5, which reads the seats table in the terms document and the seat registry and fails on any difference in moniker, name, role, driver or default remit.
- **The surface rows** for the four verbs in `surface/dispatch-table.json`, which ic reads as the table's owner before the bank is judged.

`intent/llm/MODULES.md` gains each new module's row before the module is created.

## vc's ruling on the design

On 2026-11-10 cc proposed the registry in its design notes, and vc took it under hv's pen. A seat written into each profile that uses it gives hv's row two homes, in `team` and in `solo`, and gives the parity test two tables to hold against the terms. The registry gives each seat one. The file's format is cc's to choose, within what the workspace already parses.

cc then read the package as its builder and could not build the brief as written, and vc ruled again the same day. The mediator's route and launch step differ from every other seat's, so each is written twice, and the reader chooses. And a board can declare a base and a layer, so only the base carries the four sections.

## What it must not do

It reads nothing from `~/.local/share/intent/ext/` and vendors nothing into a project. A user-authored profile is out of this thread.

## The judge

One heavy run on the bank's tip, booked with the estate's controller: the workspace's whole test suites and the whole shell suite, `intent critic rust --files` on each changed Rust file (vc decision 79), `prettier --check` on each changed Markdown file, and `bin/int precommit` over the staged tree. cc mints this package's AT rows with its bank, and each arm carries its AT id from the start.

## Depends on

WP-01, ratified.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-02` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
