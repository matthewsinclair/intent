---
wp_id: WP-01
title: The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat
scope: S
status: Done
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

Twelve points are before hv, each with vc's recommendation. Nine were put on 2026-11-10 before any seat had read the document, and three came from cc's read of it against Intent's own board the same day. The first six are vc's wordings where hv's rulings left room, and vc recommends each as written:

1. Only the mediator puts options to hv, and hv may speak to any seat directly. That is the strict reading of "mediated thru the VC".
2. "No seat waits idle on hv. A seat that needs a ruling asks the mediator, holds only the work that depends on the answer, and carries on with the rest." The ruling's own words were that nobody waits on hv, which a literal reader could take as leave to go on without the ruling.
3. On a hub the `supervisor` layer makes sc the mediator, and vc's default remit still ends "and the one route to hv". vc recommends leaving the default and letting a hub set its own vc remit.
4. "sequencing" is in both vc's default remit and hv's, in hv's own words for each.
5. The default remits of gc, sc and sv are vc's, and so is ac's "Authoring and content creation".
6. `team` is called the standard Team and not the default, so that a whiteboard that declares nothing is not read as a `team` whiteboard.

The next three come from the spike, and vc recommends adding each:

7. cc's default remit. hv's words are "Back-end services and the non-user-facing implementation of services". A project that is still modelling has no service, so read literally the remit gives cc none of the work there is, and no seat owns the data model. Proposed: "The core: the model, its rules, and the back-end services that implement them, in whatever form they take at the time, documents and data included."
8. The boundary between cc and ic on a contract. ic holds machine-facing interfaces and cc the implementation behind them, so nobody owns the contract itself. Proposed, one sentence after the seats table: "Where an interface has a contract, cc is authoritative on what it means and ic on its shape and transport, and a change to it needs both."
9. A path for disagreement with the mediator. The mediator is the one filter, and on the spike's board it was the source of three errors in one day, each caught by hv or by the seat that had reported the matter and never by the structure. Proposed, one line in hv's section: "A seat that disagrees with what the mediator has put to hv says so to the mediator, and the mediator puts the disagreement to hv in the seat's own words."

The last three are cc's, from reading the document against Intent's own board, and vc recommends each:

10. The pen. The document says the mediator "presents a matter for hv's review and does not decide it". hv's decision of 2026-08-22 on Intent's board gives vc the pen, and vc rules under it daily, each ruling recorded as vc's and open to hv's reversal. cc's words: "on Intent's board its own mediator breaks the sentence that defines it". cc proposes naming the pen as a second hat, whose scope is one whiteboard, worn by the mediator on hv's word, with each ruling under it recorded as the wearer's and "does not decide" narrowed to what hv keeps. The other course is to withdraw the pen where a board converges. The point concerns vc's own authority, so vc's recommendation, the hat, is an interested one and hv weighs it as that.
11. "Solo" is already used. `.claude/restart.md` called a session that holds no seat solo, and the shipped finish skill calls a project with one workstream solo, while the document makes `solo` the profile of one Claude and hv. Proposed: the profile keeps its name, the document defines the session that holds no seat under its own word, an unseated session, beside the sleeve and estate agents, and the two older lines are reworded. Intent's own was reworded on 2026-11-10.
12. The human seat has no session. "Registered, started, running" are defined for every seat, and a seat is read as never started from the absence of a recorded session, so every board would print hv as never started. Proposed, one sentence: the three states belong to a seat a session drives, and hv's seat is only ever registered.

Two wording fixes go in with the ratification unless hv objects, both cc's. The session name is the project's name in lower case with the moniker, since the project is `Intent` and its sessions are `intent-cc`. And the Desk's scope covers a machine that books its runs on another host's desk, as a guest VM does.

hv ratified the document on 2026-11-10, first-hand in vc's session, read at 19:26:54Z on vc's `date -u`, in these words: "Ratified as recommended". That takes all twelve points as vc recommended them and both wording fixes. vc edited the document to it the same day: points 1 to 6 stand as written, and points 7 to 12 and the two fixes are now in it. The pen of point 10 is defined as a hat whose scope is one whiteboard, and it does not reach scope, release policy, the ratification of a decision, or anything hv has said is hv's alone.

## Depends on

Nothing. Every other package waits on AC-01.5.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-01` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
