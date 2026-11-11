---
st_id: ST0092
title: A Claude Team is declared by a profile: wb init sets up its board, hv is reached through one node, and the fleet converges
status: WIP
created: 2026-11-10
completed:
---

# ST0092: A Claude Team is declared by a profile: wb init sets up its board, hv is reached through one node, and the fleet converges

## Objective

A project's Claude Team is declared once, by naming a profile, and the tool does the rest: `intent wb init <profile>` registers every seat with its canonical name, role and remit, the board says who reaches hv, and a session that boots on that board is told its own seat without hv typing it. This is hv's ask of 2026-11-10, relayed by the vc of a swarm hv had just started and ruled in vc's session the same day. hv was writing the same roster brief by hand at the start of every swarm, and the fleet's boards show what that has cost.

Concretely:

1. The terms are defined once, in Intent: Claude Team, Claude Swarm, Team Profile, the seats with their monikers, names and roles, the mediator, and the hat (WP-01).
2. A Team Profile is a shipped Intent resource of config and prose. `team` declares vc, cc, ic, ac, dc and hv; `solo` declares gc and hv; `supervisor` and `supervised` add the cross-project layer (WP-02).
3. `intent wb init <profile>` reconciles a board to its profile. It is a dry run by default; it registers what is missing, corrects what drifted and leaves everything else on the board alone; and running it again is the reinit (WP-03). A seat can be retired, and a board can change its profile (WP-04).
4. hv is reached through one node, and the tool enforces it: hv's only inbox pair is with the mediator, an ask to hv from any other node is refused with the mediator named, and an announce no longer copies hv (WP-05).
5. A session reads its brief at pickup: its seat, its remit, the route to hv and the hats it wears (WP-06). The roster README is rendered from the profile and keeps a block that is the project's own (WP-07).
6. The Desk is a hat with a written contract, worn by one VC on a host (WP-08).
7. Doctor reports a board that has left its profile (WP-09), the surface reads as one (WP-10), and every live board on this machine is converged, Intent's own first (WP-11).

## Context

**Why.** hv, 2026-11-10, to the vc of a swarm hv had just started on a client project: "I seem to have to write this same message every time I start a new Claude Swarm, which is a bit frustrating." The message is the roster and its mandates: who the nodes are, what each is for, and how hv is reached. Nothing in Intent holds it. `intent wb register` takes one node at a time, with a name and a role typed by hand; the roster README is hand-authored and has no single writer; and a pickup reads neither. hv's shape for the fix, in the same message: "A profile should be an Intent resource that is just some simple config plus possibly some text that we can give to the LLM on init to set up the whiteboard", and "This needs to work with the existing "intent wb register..." workflow, too."

**What was measured, read-only, on 2026-11-10.** vc read every live whiteboard on rhadamanth: 32 boards (18 under `~/Devel/prj`, 3 under `prj/Arca`, 9 in a client tree and 2 under `~/Devel/writing`), holding 130 node rows.

- 38 rows carry their moniker as their role and 26 carry a session name as their display name. `intent wb register --help` warns against both.
- One moniker has several role words. `dc` is `dc` on 13 boards, `devx` on 7 and `worker` on 3, Intent's own among them. `ac` is `ac` on 5, `authoring` on 2 and `author` on 1. `hv` is `hypervisor` on all 24 boards that have it.
- The 32 boards hold 17 different rosters. 3 hold all five of vc, cc, ic, ac and dc. 8 have no `hv` node, so no escalation surface, and 6 have no roster README, so no named reader of hv's inbox.
- 10 boards carry a Supervisor Claude: `sc` on a hub's own board in 3, and `sv`, the supervisor's seat on a supervised project's board, in 7.
- 884 board messages have been addressed to hv, and 741 of them are copies of an `announce` (883 and 740 at vc's first reading half an hour earlier, the figures hv's question quoted). hv has sent 5. 112 inbox views pair hv with a node that does not mediate, and 100 of those are empty.
- `intent claude start` registers nothing: it reads the node and refuses an absent one. `intent wb register <moniker>` refuses without `--name` and `--role`. So the drifted rows were typed or migrated, and no launcher wrote them.
- The Desk's duties are written in one place, `lib/templates/estate/_AGENTS.md`, for a dedicated coordinator that stands in no project. No skill names the desk, so each VC that has worn it learned it from hv's prose.

The instrument and its output are banked as refs/bank/vc/st0092/census-boards.zsh (a651ef150413) and refs/bank/vc/st0092/census-20261110.txt (cf5d0de46fd7), read at 08:29Z by `date -u`.

**hv's rulings, 2026-11-10.** Seven by selection in the TUI hv asked for, each of them vc's recommendation; the first four were answered between 08:07Z and 08:11Z and the last three by 08:13Z, on vc's `date -u`. The wording is vc's and each choice is hv's.

1. A Claude Team is the standing roster a profile registers on a project's board, every member registered whether started or not. A Claude Swarm is whichever of a Team's sessions are running at one time.
2. The Supervisor has its own profile, shipped in this thread. hv, in prose the same hour: "SC is only needed on larger projects where we need multi-project coordination that goes across multiple VCs. Its not needed for a single swarm/tean."
3. The single Claude is `gc`, Generalist Claude, role `generalist`.
4. Sleeve agents and estate agents stay out of the profile mechanism. The terms name them, with Molt and Gtools as their owners.
5. hv's mediation is enforced by the tool.
6. The roster README is generated, with a preserved block.
7. The sweep writes directly where a project has no running session and through its vc or sc where one is running: all 32 boards, Intent's own first.

And hv's prose in vc's session, the same morning:

- The default Team: "a Claude Swarm (or Team) has, by default: VC, CC, IC, AC, and DC. The fact that not all of the Team is started at any one time is just a detail of use, they all should be registered and ready to be used if needed. If there is a case where just need 1 Claude and not a swarm, then that should be a different profile with a differnt role, abbreviation, and remit."
- The terms: "we canonically define the terms: Claude Swarm vs Claude Team; the correct abbreviations CC vs Control, etc; and the same definitions in each case for what the roles do."
- hv's own role: "We need to tidy up the HV / Hypervisor role and actions. It's ad-hoc right now, and needs to be made explicit. If you look at how the projects work, HV interactions are (or shouild be!) mediated thru the VC. Having HV detritous in the other Claude's inbox/whiteboards is a distraction at best, and intellectual pollution at worst."
- The sweep: "When done, we do an estate sweep and reach in thru each project's spinal cord and just do some brain surgery so that they are in line with this profile based approach."
- Pipeliner's `sc`: "I think that was a mistake and probably needs to be revoked and just back to a normal VC/CC/IC/DC/AC mix."
- The Desk: "The way I have been using this is as an extra "hat" that one VC wears. That _seems_ to be working, but we need to institutionalise the way that role works in addition to the other VC roles."

The seat names are hv's of 2026-10-21, recorded until now in Molt's roster README and nowhere in Intent.

**What this thread does not do.**

- It ships no user-authored profile. `intent ext` is declared and not built, and nothing reads `~/.local/share/intent/ext/`. A profile is Intent's canon, the same on every machine that runs the same Intent.
- It gives no sleeve agent or estate agent a profile or a board, and it installs Intent in no project that lacks it (hv, 2026-09-23).
- It does not fix issue 0672. It carries that issue's lesson as AC-07.4: a generated contract states what was declared, and never a default guessed for the project.
- It renames no node. A seat that must change its moniker is retired and its successor registered.

**Order and release.** WP-01 lands first, and no code is written before hv has ratified its document (AC-01.5). WP-02 and WP-03 follow in that order, and WP-04 to WP-09 stand on them. WP-11 runs only once the brew-installed Intent on this machine carries WP-02 to WP-10. The thread adds verbs. Which release carries it, and where it sits against the held 3.2.9 train, is hv's to say.

**Builders and judge.** cc builds WP-02 to WP-09 and ic builds WP-10, each claiming its package in the chain that starts it. vc writes WP-01 under hv's pen and directs WP-11. vc judges every bank and claims nothing, and each builder mints its package's AT rows with its bank. These assignments are vc's proposal, for hv's review of this elaboration.

## Work Packages

| WP    | Title                                                                                 | Size | Status      |
| ----- | ------------------------------------------------------------------------------------- | ---- | ----------- |
| WP-01 | The terms are defined once: Team, Swarm, profile, the seats, the mediator and the hat | S    | Done        |
| WP-02 | A Team Profile is a shipped resource: team, solo, supervisor and supervised           | M    | Done        |
| WP-03 | wb init reconciles a board to its profile, and is a dry run until told to apply       | M    | Done        |
| WP-04 | A seat can be retired, and a board can change its profile                             | S    | WIP         |
| WP-05 | hv is reached through one node, and the tool enforces it                              | M    | Done        |
| WP-06 | A session is told its seat at pickup and at launch                                    | S    | WIP         |
| WP-07 | The roster README is rendered from the profile and keeps the project's own block      | M    | Done        |
| WP-08 | The Desk is a hat with a written contract                                             | M    | Done        |
| WP-09 | Doctor reports a board that has left its profile                                      | S    | WIP         |
| WP-10 | The new surface reads as one                                                          | S    | Not Started |
| WP-11 | Every live board converges, Intent's own first                                        | L    | Not Started |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
