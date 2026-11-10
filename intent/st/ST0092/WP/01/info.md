---
wp_id: WP-01
title: The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat
scope: S
status: Not Started
---

# WP-01: The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat

## Objective

Intent holds one document that defines the words this thread uses and every project's board will use: Claude Team, Claude Swarm, Team Profile, node, moniker, name, role, session name, mediator and hat, with each seat's moniker, name, role and default remit, and hv's own contract. Everything else cites it. It is written and ratified before any code, because every later package renders or enforces what it says.

## Builder and judge

vc writes this package under hv's pen: its content is hv's rulings of 2026-11-10 and hv's seat names of 2026-10-21, written down. It is written in cc's tree, and cc reads it when it starts WP-02. The judge is `critic-prose` on the document, `prettier --check`, and hv's own read, which AC-01.5 records. vc claims nothing.

## What is written

One hand-authored document in the plugin canon, beside where the profiles will ship (vc's proposal: `intent/plugins/claude/teams/TERMS.md`), registered in `intent/llm/MODULES.md` before it is created. It ships with the install, so a session in any project can be shown it, and WP-02 adds the verb that prints it. `intent/docs/working-with-llms.md` links to it and restates none of it. It holds:

- **The terms.** A Claude Team is the standing roster a profile registers on one project's board, every member registered whether started or not. A Claude Swarm is whichever of a Team's sessions are running at one time. A Team Profile is the Intent resource that declares a Team. A node is one seat on a board. Its moniker is its two-letter handle, routing key and directory; its name is its display name; its role is one word; and its session is named `<project>-<moniker>`. A moniker is never written as a name or as a role.
- **The seats.** hv's names of 2026-10-21. The default remits of vc, cc, ic, ac, dc and hv are hv's own words of 2026-11-10, as the first swarm's roster records them; those of gc, sc and sv are vc's, for hv to edit at ratification:

| Moniker | Name              | Role       | Driven by                | Default remit                                                                                                  |
| ------- | ----------------- | ---------- | ------------------------ | -------------------------------------------------------------------------------------------------------------- |
| `vc`    | Validation Claude | validation | a session                | Boss of the Claude team: sequencing, briefs, validation of landed and claimed work, and the one route to hv    |
| `cc`    | Control Claude    | control    | a session                | Back-end services and the non-user-facing implementation of services                                           |
| `ic`    | Interface Claude  | interface  | a session                | User-facing interfaces (UI and UX) and machine-facing interfaces (REST, GraphQL and the rest)                  |
| `ac`    | Authoring Claude  | authoring  | a session                | Authoring and content creation                                                                                 |
| `dc`    | DevX Claude       | devx       | a session                | The dev-x pipeline, end to end                                                                                 |
| `hv`    | Hypervisor        | hypervisor | the human                | Scope, sequencing, release policy, and every ratified decision                                                 |
| `gc`    | Generalist Claude | generalist | a session                | The whole project, alone: it builds and checks its own work and talks to hv directly                           |
| `sc`    | Supervisor Claude | supervisor | a session                | Coordination across several projects' VCs: audits, briefs to each VC, and the route to hv on its own hub       |
| `sv`    | Supervisor Claude | supervisor | the supervisor's session | The supervisor's seat on a supervised project's board: findings and briefs to that project's vc, and no builds |

- **The mediator.** The one node through which hv is reached on a board: vc on a team, sc on a supervisor's hub, gc on a solo board.
- **hv's contract.** hv rules, in a live session or in the TUI. The mediator records each ruling on hv's board with its date, the channel it came by and whose wording it is, and presents matters for hv's review without deciding them. hv reads no board unprompted and writes to no inbox. No node waits on hv: it asks the mediator and carries on with what does not depend on the answer. A directive leaves hv's board only on hv's word.
- **The hat.** A duty a node carries on top of its role, worn by one node at a time within its scope, and taken and handed over by an explicit act. The Desk is the hat this thread defines; its duties are WP-08's.
- **Other kinds.** A sleeve agent works on one machine and is Molt's (`molt agent`, roles sysadm, ops and docs). An estate agent works in one Gtools estate and is Gtools'. Neither is a node, neither has a profile, and this document defines neither's contract.

## What it is not

It is not the protocol: the `/in-whiteboard` skill keeps that and cites this document for the words. It is not the source of the seat rows once WP-02 lands: from then the shipped profiles are, and AC-02.5 fails a build in which this table and the profiles disagree.

## Depends on

Nothing. Every other package waits on AC-01.5.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-01` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
