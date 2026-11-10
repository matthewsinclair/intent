---
wp_id: WP-01
title: The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat
scope: S
status: WIP
---

# WP-01: The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat

## Objective

Intent holds one document that defines the words this thread uses and every project's board will use: Claude Team, Claude Swarm, Team Profile, node, moniker, name, role, session name, mediator and hat, with each seat's moniker, name, role and default remit, and hv's own contract. Everything else cites it. It is written and ratified before any code, because every later package renders or enforces what it says.

## Builder and judge

vc writes this package under hv's pen: its content is hv's rulings of 2026-11-10 and hv's seat names of 2026-10-21, written down. It is written in cc's tree, and cc reads it when it starts WP-02. The judge is `critic-prose` on the document, `prettier --check`, and hv's own read, which AC-01.5 records. vc claims nothing.

## What is written

One hand-authored document in the plugin canon, `intent/plugins/claude/teams/TERMS.md`, registered in `intent/llm/MODULES.md` before it was created. It ships with the install, so a session in any project can be shown it, and WP-02 adds the verb that prints it. It carries no frontmatter, as its neighbours under `rules/_schema` carry none, and it names no command, because the verbs that will print and apply it are not built. `intent/docs/working-with-llms.md` links to it and restates none of it.

It holds, in this order: the whiteboard, the board and the seat, with the seat's four labels, its remit, and registered, started and running; Team, Swarm and Profile, with base and layer and the four shipped profiles; the seats table; the mediator; hv's part and how every seat works with hv; the hat and the Desk; and the two kinds of Claude that hold no seat. The wording is the document's, and this package does not restate it.

## What it is not

It is not the protocol: the `/in-whiteboard` skill keeps that and cites this document for the words. It is not the source of the seat rows once WP-02 lands: from then the shipped profiles are, and AC-02.5 fails a build in which the document's table and the profiles disagree.

## The critic's read, 2026-11-10

`critic-prose` read the first draft (f6e987017) with the author pack: no critical finding, the mechanical tier clean, and eight craft warnings, most of them one word carrying two senses. Its verdict was that the draft was close and not yet fit for ratification. vc applied all eight in the document's wording, with no ruling's meaning changed:

- A project's **whiteboard** and a seat's **board** are now two words. The draft used board for both.
- The four things that go with a seat are its labels, so that name means the display name alone.
- A Team "exists whether or not any session is running", where the draft's "while no session is running" read as only then.
- The mediator is named by the base profile, and the `supervisor` layer moves it to sc. The draft's three cases overlapped, because a hub is also a `team` whiteboard.
- A supervisor "is not needed for one Team on its own", which is hv's verb.
- A seat that hv rules to directly passes the ruling to the mediator, so who puts a matter to hv has one answer.
- The sentence that every seat is a project's own is gone: sv and hv are seats and neither is.
- Monikers are lower-case throughout.

## For hv's eye at ratification

These are vc's wordings where hv's rulings left room, and each is hv's to change:

1. Only the mediator puts options to hv, and hv may speak to any seat directly. That is the strict reading of "mediated thru the VC".
2. "No seat waits idle on hv. A seat that needs a ruling asks the mediator, holds only the work that depends on the answer, and carries on with the rest." The ruling's own words were that nobody waits on hv, which a literal reader could take as leave to go on without the ruling.
3. On a hub the `supervisor` layer makes sc the mediator, and vc's default remit still ends "and the one route to hv". A hub sets vc's remit, or the default changes.
4. "sequencing" is in both vc's default remit and hv's, in hv's own words for each.
5. The default remits of gc, sc and sv are vc's, and so is ac's "Authoring and content creation".
6. `team` is called the standard Team and not the default, so that a whiteboard that declares nothing is not read as a `team` whiteboard.

## Depends on

Nothing. Every other package waits on AC-01.5.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-01` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
