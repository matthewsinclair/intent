# Claude Teams: the terms

This document defines the words Intent uses for the Claudes that work on a project together, and for the human they work for. It is the one place they are defined: a skill, a guide or a roster that uses one of these words cites this document and does not define it again, so a word means the same thing in every project. The terms follow rulings Intent's maintainer made on 2026-11-10 (Intent ST0092), and the seat names date from 2026-10-21.

## Whiteboard, board and seat

A project's **whiteboard** is where its Claudes and its human coordinate. It holds one **board** for each seat, written by that seat alone, and its **roster** is the set of seats registered on it. The whiteboard's own protocol, its inboxes and claims among it, is defined with the whiteboard and not here.

A **node**, or **seat**, is one place on a whiteboard. Four labels go with a seat, and they are never interchanged:

- Its **moniker** is its handle: two letters, eg `cc`. It is the seat's routing key and the name of its directory. A moniker is never written as a name or as a role.
- Its **name** is its display name, eg Control Claude.
- Its **role** is one word for what it does, eg `control`.
- Its **session name** is what a session running in the seat is called: `<project>-<moniker>`, eg `intent-cc`. hv, the human, has none.

A seat's **remit** is one line saying what the seat is for on this project. A profile gives each seat a default remit. A project may set its own, and its own then stands.

A seat is **registered** when it is on the roster, **started** once any session has picked it up, and **running** while a session holds it.

## Team, Swarm and Profile

A **Claude Team** is the standing roster on one project's whiteboard: every seat its profiles declare, registered whether started or not. A Team exists whether or not any session is running.

A **Claude Swarm** is whichever of a Team's sessions are running at one time. Starting a swarm launches sessions for some of a Team's seats and changes nothing on the roster.

A **Team Profile** is the Intent resource a Team is declared from. It gives the seats, the mediator, and the **team brief**, which is the text a session is given when it starts in a seat. Intent ships the profiles, so a project names one and never copies it. A **base** profile declares a whole roster, and a whiteboard declares exactly one. A **layer** adds seats to a base and may move the mediator. Four profiles are shipped, two bases and two layers:

- `team`, a base: vc, cc, ic, ac, dc and hv, with vc the mediator. It is the standard Team, and all six seats are registered whether or not they are ever started.
- `solo`, a base: gc and hv, with gc the mediator, for a project that one Claude works alone.
- `supervisor`, a layer for a **hub**, which is a project whose supervisor coordinates other projects: it adds sc and makes sc the mediator.
- `supervised`, a layer for a project under a supervisor: it adds sv and leaves the mediator as it was.

## The seats

| Moniker | Name              | Role       | Driven by                | Default remit                                                                                                             |
| ------- | ----------------- | ---------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| `vc`    | Validation Claude | validation | a session                | Boss of the Claude team: sequencing, briefs, validation of landed and claimed work, and the one route to hv               |
| `cc`    | Control Claude    | control    | a session                | Back-end services and the non-user-facing implementation of services                                                      |
| `ic`    | Interface Claude  | interface  | a session                | User-facing interfaces (UI and UX) and machine-facing interfaces (REST, GraphQL and the rest)                             |
| `ac`    | Authoring Claude  | authoring  | a session                | Authoring and content creation                                                                                            |
| `dc`    | DevX Claude       | devx       | a session                | The dev-x pipeline, end to end                                                                                            |
| `hv`    | Hypervisor        | hypervisor | the human                | Scope, sequencing, release policy, and every ratified decision                                                            |
| `gc`    | Generalist Claude | generalist | a session                | The whole project, alone: it builds and checks its own work and talks to hv directly                                      |
| `sc`    | Supervisor Claude | supervisor | a session                | Coordination across several projects, each with its own vc: audits, briefs to each vc, and the route to hv on its own hub |
| `sv`    | Supervisor Claude | supervisor | the supervisor's session | The supervisor's seat on a supervised project's whiteboard: findings and briefs to that project's vc, and no builds       |

A supervisor is needed only where work is coordinated across several projects, each with its own vc. It is not needed for one Team on its own.

## The mediator

The **mediator** is the one seat through which hv is reached on a whiteboard. The base profile names it, vc under `team` and gc under `solo`, and the `supervisor` layer moves it to sc on a hub. A seat that needs something from hv sends its request to the mediator.

## hv

**hv** is the human, and a seat on every whiteboard. This is hv's part, and how every seat works with hv:

- hv rules, in prose or by choosing among the options the mediator puts to them. hv may speak to any seat directly.
- The mediator records each ruling on hv's board, with its date, the channel it came by and whose wording it is. A seat that hv rules to directly passes the ruling to the mediator. The mediator presents a matter for hv's review and does not decide it.
- hv reads no board unprompted and writes to no inbox. What a seat needs from hv reaches hv when the mediator puts it in front of them.
- No seat waits idle on hv. A seat that needs a ruling asks the mediator, holds only the work that depends on the answer, and carries on with the rest.
- A **directive** is hv's standing word to every seat. It stays on hv's board until hv withdraws it.

## A hat

A **hat** is a duty a seat carries on top of its role. One seat wears a hat at a time within the hat's scope, and a hat is taken and handed over by an explicit act.

The **Desk** is a hat whose scope is one host: control of that host's slots, which decides how much heavy work runs on the machine at once. One vc on a host wears it, on hv's word. Its duties are written with the slots themselves, not here.

## Other kinds of Claude

Two other kinds of Claude hold no seat on any whiteboard, have no profile, and take their contracts from the tools that own them:

- A **sleeve agent** works on one machine, eg as its system administrator. Molt owns it.
- An **estate agent** works in one Gtools estate. Gtools owns it.
