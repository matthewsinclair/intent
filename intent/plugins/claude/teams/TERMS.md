# Claude Teams: the terms

This document defines the words Intent uses for the Claudes that work on a project together, and for the human they work for. It is the one place they are defined. Intent's skills, its guide and each project's roster cite it and restate none of it, so a word means the same thing on every board. The definitions were ruled by Intent's maintainer on 2026-11-10 (Intent ST0092), and the seat names on 2026-10-21.

## Team, Swarm and Profile

A **Claude Team** is the standing roster a profile registers on one project's board: every member registered, whether started or not. A Team exists while no session is running.

A **Claude Swarm** is whichever of a Team's sessions are running at one time. Starting a swarm launches sessions for some of a Team's seats; it changes nothing on the roster.

A **Team Profile** is the Intent resource that declares a Team: its seats, the seat through which hv is reached, and the brief a session is given. Intent ships the profiles, so a project names one and never copies it. A **base** profile is a whole roster, and a board declares exactly one. A **layer** adds seats to a base. Four are shipped:

- `team`, a base: vc, cc, ic, ac, dc and hv. It is the default, and all six seats are registered whether or not they are ever started.
- `solo`, a base: gc and hv, for a project that one Claude works alone.
- `supervisor`, a layer for a hub that coordinates several projects: it adds sc.
- `supervised`, a layer for a project under a supervisor: it adds sv.

## A seat and its names

A **node**, or **seat**, is one place on a board. Each has four names, and they are never interchanged:

- Its **moniker** is its handle: two letters, eg `cc`. It is the seat's routing key and the name of its directory. A moniker is never written as a name or as a role.
- Its **name** is what it is called, eg Control Claude.
- Its **role** is one word for what it does, eg `control`.
- Its **session name** is `<project>-<moniker>`, eg `intent-cc`, and belongs to a running session, not to the seat.

A seat's **remit** is one line saying what the seat is for on this project. A profile gives each seat a default remit. A project may set its own, and its own then stands.

A seat is **registered** when it is on the roster, **started** once any session has picked it up, and **running** while a session holds it.

## The seats

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

A supervisor is needed only where work is coordinated across several projects, each with its own VC. A single Team has none.

## The mediator

The **mediator** is the one seat through which hv is reached on a board: vc on a team, sc on a supervisor's hub, and gc on a solo board. Every other seat sends what it needs from hv to the mediator.

## hv

**hv** is the human, and a seat on every board. hv's part is this:

- hv rules, in a live session or by choosing among options a session puts to them.
- The mediator records each ruling on hv's board, with its date, the channel it came by and whose wording it is. The mediator presents a matter for hv's review and does not decide it.
- hv reads no board unprompted and writes to no inbox. What reaches hv is what the mediator puts in front of them.
- No seat waits on hv. A seat that needs a ruling asks the mediator and carries on with whatever does not depend on the answer.
- A directive is hv's standing word to every seat. It stays on hv's board until hv says it is spent.

## A hat

A **hat** is a duty a seat carries on top of its role. One seat wears it at a time within its scope, and it is taken and handed over by an explicit act, never assumed.

The **Desk** is a hat: control of one host's slots, which decides how much heavy work runs on a machine at once. One VC on a host wears it, on hv's word. Its duties are written with the slots themselves, not here.

## Other kinds of Claude

A seat on a board is a project's own. Two other kinds of Claude are not seats, have no profile, and take their contracts from the tools that own them:

- A **sleeve agent** works on one machine, eg as its system administrator. Molt owns it.
- An **estate agent** works in one Gtools estate. Gtools owns it.
