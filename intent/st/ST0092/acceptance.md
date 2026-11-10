---
st_id: ST0092
title: A Claude Team is declared by a profile: wb init sets up its board, hv is reached through one node, and the fleet converges
---

# ST0092: A Claude Team is declared by a profile: wb init sets up its board, hv is reached through one node, and the fleet converges -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat (status: Not Started)

- AC-01.1 (non-test) One document in the plugin canon, shipped with the install, defines Claude Team, Claude Swarm, Team Profile, node, moniker, name, role, session name, mediator and hat in the words hv ruled on 2026-11-10. -- satisfied: no
- AC-01.2 (non-test) It gives the moniker, name, role, driver and default remit of each seat, vc, cc, ic, ac, dc, hv, gc, sc and sv, with the role words validation, control, interface, authoring, devx, hypervisor, generalist and supervisor, and it says a moniker is never written as a name or as a role. -- satisfied: no
- AC-01.3 (non-test) It states hv's contract: hv rules live or in the TUI; the mediator records each ruling on hv's board with its date, its channel and whose wording it is, and presents for review without deciding; hv reads no board unprompted and writes to no inbox; no node waits on hv; and a directive leaves hv's board only on hv's word. -- satisfied: no
- AC-01.4 (non-test) It defines the hat and names the Desk as one, and it names sleeve agents and estate agents as kinds owned by Molt and Gtools without defining the contract of either. -- satisfied: no
- AC-01.5 (non-test) hv has read the document and ratified it, and vc has recorded the ratification on hv's board with its date and channel, before any other package of this thread starts. -- satisfied: no

### WP-02 -- A Team Profile is a shipped resource: team, solo, supervisor and supervised (status: Not Started)

- AC-02.1 The plugin canon ships the profiles team, solo, supervisor and supervised under one schema, and `intent wb profile validate` passes on the shipped set. -- satisfied: no (computed)
- AC-02.2 Validation fails, naming the file and the field, on a base with no mediator, a mediator that is not one of the profile's seats, a duplicate moniker, a role equal to its moniker, a name written as a session name, a second seat driven by the human, and a layer that declares a human seat. -- satisfied: no (computed)
- AC-02.3 `intent wb profile list` names each shipped profile and whether it is a base or a layer, `intent wb profile show <name>` prints its seats, its mediator and its brief, and `intent wb profile terms` prints the terms document; all three answer in a directory that is no Intent project. -- satisfied: no (computed)
- AC-02.4 team declares vc, cc, ic, ac, dc and hv with vc the mediator; solo declares gc and hv with gc the mediator; supervisor adds sc and makes it the mediator; and supervised adds sv and leaves the mediator as it was. -- satisfied: no (computed)
- AC-02.5 The seats table in the terms document and the shipped profiles agree seat for seat in moniker, name, role and default remit, and a test fails on any difference. -- satisfied: no (computed)
- AC-02.6 An unknown profile name is refused with the shipped names in its remedy, and nothing is read from `~/.local/share/intent/ext/`. -- satisfied: no (computed)

### WP-03 -- wb init reconciles a board to its profile, and is a dry run until told to apply (status: Not Started)

- AC-03.1 In a project with no whiteboard, `intent wb init team --apply` registers every seat the profile declares with the profile's name, role and default remit, records the declaration in the store and as canon under `intent/whiteboard/`, and prints every file it wrote on a `wrote:` line. -- satisfied: no (computed)
- AC-03.2 Without `--apply` it writes nothing and prints, for each seat, register, correct with the old and the new value, unchanged, or outside the profile, and then every view it would write or remove. -- satisfied: no (computed)
- AC-03.3 On a board whose nodes hold items, messages, handled marks, claims, archives, heartbeats, status and focus, an apply changes names, roles, absent remits and the roster set and nothing else: those fields of every `board.json` are byte for byte what they were. -- satisfied: no (computed)
- AC-03.4 A second apply writes nothing and says so, and `intent wb init --apply` with no profile named re-applies the board's declaration. -- satisfied: no (computed)
- AC-03.5 A base other than the declared one is refused and names `--change`; a layer on a board with no base is refused; and a hand-authored node that has not been migrated is refused before anything is written, naming `intent wb migrate`. -- satisfied: no (computed)
- AC-03.6 `intent wb register` of a moniker the declared profiles name is taken only with the profile's name and role, and its refusal prints them; a moniker they do not name registers as before, and the dry run reports it as outside the profile. -- satisfied: no (computed)
- AC-03.7 A remit set with `intent wb remit <moniker> "<text>"` survives every later apply, and the profile's default fills only an absent one. -- satisfied: no (computed)
- AC-03.8 A clone of a project whose board was applied ingests the same declaration, seats and remits from the canon on disk. -- satisfied: no (computed)
- AC-03.9 A build of this thread reads a board at schema `intent/board@4.0` unchanged. -- satisfied: no (computed)

### WP-04 -- A seat can be retired, and a board can change its profile (status: Not Started)

- AC-04.1 `intent wb retire <moniker> --reason "<why>"` retires a node: `intent wb status` lists it only with `--all`, an ask to it is refused and names the retirement, an announce skips it, and its board and every message it sent or received stay readable. -- satisfied: no (computed)
- AC-04.2 It refuses without `--reason`, refuses a node that holds a claim and names each claim, and refuses the board's mediator. -- satisfied: no (computed)
- AC-04.3 A live message does not refuse a retirement: one the leaver sent stays in its recipient's inbox until that recipient handles it, and one addressed to the leaver is named in the retirement's report and at its sender's next pickup, where `intent wb archive message` then accepts it. -- satisfied: no (computed)
- AC-04.4 A pair's inbox views are removed, on `removed:` lines, only when they hold no live message. -- satisfied: no (computed)
- AC-04.5 Registering a retired moniker again reinstates the same node with its board and never makes a second. -- satisfied: no (computed)
- AC-04.6 `intent wb init <base> --change --apply` registers the new base's seats and retires those of the old base that the new one does not declare, its dry run lists both, and a re-apply without `--change` retires nothing. -- satisfied: no (computed)

### WP-05 -- hv is reached through one node, and the tool enforces it (status: Not Started)

- AC-05.1 On a board that declares a profile, the human seat's only inbox pair is with the mediator: no `inbox.hv.md` is rendered in any other node's directory and no `hv/inbox.<node>.md` for any other node. -- satisfied: no (computed)
- AC-05.2 `intent wb ask hv` from any other node is refused with a remedy that names the mediator and can be run as written, and a message from the human seat to any node but the mediator is refused the same way. -- satisfied: no (computed)
- AC-05.3 `intent wb announce` sends no copy to the human seat. -- satisfied: no (computed)
- AC-05.4 A write as the human seat from a Claude session bound to any node but the mediator is refused, and the same write from the mediator's session, or from a shell with no Claude session, is taken. -- satisfied: no (computed)
- AC-05.5 hv's directives, decisions and watch-outs print in full at every node's pickup. -- satisfied: no (computed)
- AC-05.6 An apply refuses to drop an hv inbox view that holds a live message and names each one with its sender, proceeds once the mediator has cleared them, and leaves every one readable with `--all`. -- satisfied: no (computed)
- AC-05.7 A board that declares no profile renders every pair and takes every ask, as it did before this thread. -- satisfied: no (computed)
- AC-05.8 (non-test) The `/in-whiteboard` skill says what the tool does about hv, `ask` and `announce` in the same landing as this package, cites the terms document for its words, and no longer requires the README to name hv's reader. -- satisfied: no

### WP-06 -- A session is told its seat at pickup and at launch (status: Not Started)

- AC-06.1 `intent wb pickup` on a board that declares a profile prints, ahead of the board, the declared profiles, this node's name, role and remit, the node hv is reached through, and one line for each other seat with its name, role and remit; `--json` and the MCP tool carry the same fields. -- satisfied: no (computed)
- AC-06.2 A seat that is registered and has never run is listed like any other and marked as never started. -- satisfied: no (computed)
- AC-06.3 `intent claude start <node>` hands the launched session the same brief, and its refusal of an unregistered moniker names `intent wb init`. -- satisfied: no (computed)
- AC-06.4 (non-test) A session started on a converged board that hv did not brief states its own seat, its remit and its route to hv from the pickup alone, and vc records that boot by its session id. -- satisfied: no

### WP-07 -- The roster README is rendered from the profile and keeps the project's own block (status: Not Started)

- AC-07.1 An apply renders `intent/whiteboard/README.md` from one template under `lib/templates/`: the declared profiles, a roster table with each seat's moniker, name, role, driver and remit, the route to hv, and the verb that prints the terms. -- satisfied: no (computed)
- AC-07.2 Everything below the template's marker is carried byte for byte across every re-render, by the function that carries CLAUDE.md's block and by no copy of it. -- satisfied: no (computed)
- AC-07.3 The first render over a README that has no marker carries the whole existing file below the marker and says so. -- satisfied: no (computed)
- AC-07.4 The generated part states nothing that the profiles and the store do not declare, which is issue 0672's lesson: rendered for two projects with the same declaration and remits, it differs only in the project's name. -- satisfied: no (computed)
- AC-07.5 A hand edit above the marker is skew that `intent doctor` reports and the next render restores, and a re-render with nothing changed writes nothing. -- satisfied: no (computed)

### WP-08 -- The Desk is a hat with a written contract (status: Not Started)

- AC-08.1 The controller's duties have one template source: the coordinator's `AGENTS.md` that `intent estate init` renders and the brief a hat-wearing node is given carry the same duties text from it. -- satisfied: no (computed)
- AC-08.2 `intent estate control take` from a Claude session bound to a node that is not its board's vc is refused with a remedy naming who may take it; with `--hv "<words>"` it is taken and the words are recorded; and a session in no project takes it as before. -- satisfied: no (computed)
- AC-08.3 While a node's session holds control, its pickup prints that it wears the desk, with the duties and the asks waiting, and its board header and `intent wb status` show the hat, read from the control record and stored nowhere else. -- satisfied: no (computed)
- AC-08.4 When the recorded controller's session is gone, `intent estate status` says the desk is orphaned and prints the step that takes it, and no node's pickup or board shows the hat. -- satisfied: no (computed)
- AC-08.5 (non-test) The duties say how the hat sits with a VC's own work: a waiting ask is answered before the VC's own next step, the wearer's own project books through the same queue on the record, the hat is released at session end, and it passes to another VC on hv's quoted word. -- satisfied: no

### WP-09 -- Doctor reports a board that has left its profile (status: Not Started)

- AC-09.1 `intent doctor` reports, each as its own class with `intent wb init` in its remedy: a board that declares no profile, a seat whose name or role differs from its profile's, a seat its profile declares that is not registered, and an hv inbox view outside the mediator's pair. -- satisfied: no (computed)
- AC-09.2 None of the four classes refuses a commit: the pre-commit gate passes a tree that carries each. -- satisfied: no (computed)
- AC-09.3 (non-test) vc has run the banked census against a build that carries this package and recorded the count of each class for each project, and whether any class gates after the sweep is ruled by hv with that table in front of him. -- satisfied: no

### WP-10 -- The new surface reads as one (status: Not Started)

- AC-10.1 Every verb this thread adds has a dispatch-table row that states its recoverability, its `when_to_use` and its MCP disposition, and `intent llm guide` prints it. -- satisfied: no (computed)
- AC-10.2 (non-test) ic has read every new help line, refusal and exit code against the tool's voice and its exit-code contract, corrected what differed, and recorded the read on its board. -- satisfied: no
- AC-10.3 (non-test) No shipped file defines one of the terms a second time: the `/in-whiteboard` skill, the guide and the roster template cite the terms document, and ic has recorded the search that shows it. -- satisfied: no

### WP-11 -- Every live board converges, Intent's own first (status: Not Started)

- AC-11.1 (non-test) vc's read-only census names every live board on this machine with the profiles it will declare, each correction, registration and retirement, and each live message addressed to hv from a node that does not mediate; hv has said which one-node boards are solo and has said go. -- satisfied: no
- AC-11.2 (non-test) vc has measured what the released Intent before this thread does with a board at the new schema, and no project is converged while a machine that reads its board runs an Intent that cannot read it. -- satisfied: no
- AC-11.3 (non-test) Intent's own board is converged first: ac is registered, dc reads devx, the README is rendered with its previous text below the marker, and the commit carries exactly what `intent wb init --apply` printed. -- satisfied: no
- AC-11.4 (non-test) Every board with no running session is converged from vc's session, one commit for each project of exactly what the verb printed; every board whose vc or sc is running is converged by that node on vc's exact step; and vc has read each commit. -- satisfied: no
- AC-11.5 (non-test) Pipeliner's sc is retired and its board is a team with vc the mediator, once hv has confirmed it and Pipeliner's own nodes have been told; until then its board is converged with sc as it stands. -- satisfied: no
- AC-11.6 (non-test) Molt's guide and its roster README cite Intent's terms for the project suffixes and record them nowhere, by molt-vc's hand. -- satisfied: no
- AC-11.7 (non-test) The census re-run after the sweep reads: every board declares a profile, no seat's role is its moniker, no seat's name is a session name, every board has hv and a mediator, and no hv inbox view exists outside a mediator's pair; a board hv exempts is named with his words. -- satisfied: no
- AC-11.8 (non-test) The sweep installed Intent in no project, made a board in no project that had none, pushed nothing, and wrote nothing in any project but what `intent wb init --apply` printed. -- satisfied: no

## Acceptance Tests

### WP-01 -- The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat (status: Not Started)

_(no tests in this group)_

### WP-02 -- A Team Profile is a shipped resource: team, solo, supervisor and supervised (status: Not Started)

_(no tests in this group)_

### WP-03 -- wb init reconciles a board to its profile, and is a dry run until told to apply (status: Not Started)

_(no tests in this group)_

### WP-04 -- A seat can be retired, and a board can change its profile (status: Not Started)

_(no tests in this group)_

### WP-05 -- hv is reached through one node, and the tool enforces it (status: Not Started)

_(no tests in this group)_

### WP-06 -- A session is told its seat at pickup and at launch (status: Not Started)

_(no tests in this group)_

### WP-07 -- The roster README is rendered from the profile and keeps the project's own block (status: Not Started)

_(no tests in this group)_

### WP-08 -- The Desk is a hat with a written contract (status: Not Started)

_(no tests in this group)_

### WP-09 -- Doctor reports a board that has left its profile (status: Not Started)

_(no tests in this group)_

### WP-10 -- The new surface reads as one (status: Not Started)

_(no tests in this group)_

### WP-11 -- Every live board converges, Intent's own first (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
