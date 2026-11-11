---
wp_id: WP-05
title: hv is reached through one node, and the tool enforces it
scope: M
status: WIP
---

# WP-05: hv is reached through one node, and the tool enforces it

## Objective

On a board that declares a profile, hv has one channel: with the mediator. The inbox pairs between hv and every other node stop being rendered, an ask to hv from any other node is refused with the mediator named, and an announce no longer copies hv. hv's directives, decisions and watch-outs still reach every node at pickup, because they are the team's law and not traffic.

## Builder and judge

cc builds this package and claims `ST0092/05` in the same chain as `intent wp start ST0092/05`. vc judges it and claims nothing.

## What was measured

Across the 32 live boards on 2026-11-10: 884 messages addressed to hv, of which 741 are copies of an `announce`; 5 messages sent by hv, ever; 112 inbox views pairing hv with a node that does not mediate, 100 of them empty. The cause is in the model: every registered node gets an inbox pair with every other, hv included, and an announce goes to every node but its sender.

The first swarm set up by hand, on 2026-11-10, showed what the convention this package replaces costs. Clearing hv's copy of an announce took two verbs, a clear as hv and an archive by its sender, because a clear marks a message handled and leaves the sender's rendered view as it was. Its vc then found itself sending a session message where an announce was right, to escape the cleanup. An announce must cost the mediator nothing, which AC-05.3 gives it.

## What is built

- **The pair.** The human seat has one inbox pair, with the mediator. No `<node>/inbox.hv.md` and no `hv/inbox.<node>.md` is rendered for any other node.
- **The ask.** `intent wb ask hv` from any other node is refused with a typed error whose remedy names the mediator and gives the same message as an ask to it. A message from the human seat to a node that is not the mediator is refused the same way: hv speaks to a node live, or the mediator carries it.
- **The announce** goes to every registered session node but the sender, and not to the human seat.
- **Writing as hv.** A write as the human seat from a Claude session bound to any node but the mediator is refused. The mediator's session writes hv's board under the pen, and a shell with no Claude session, which is hv's own terminal, writes it as before. The session's node is read from `CLAUDE_CODE_SESSION_ID`, the identity the pickup already records. That read is one function, shared with WP-08's check of who may take the desk (vc's ruling of 2026-11-10 on cc's design note).
- **The standing items.** hv's directives, decisions and watch-outs print in full at every node's pickup, as they do now.
- **The crossing.** `intent wb init --apply` refuses to drop an hv inbox view that still holds a live message and names each one with its sender. The mediator surfaces them to hv and clears them as hv, and the apply then proceeds. Fleet-wide on 2026-11-10, 4 such messages come from a `dc` or a `cc`: 3 and 1. Every such message stays readable with `--all`. A message that is handled and not archived holds no view back: only a live one refuses.
- **Opt-in by declaration.** A board that declares no profile behaves exactly as it does today, so nothing changes for a project until its own `wb init`.
- **The skill.** `intent/plugins/claude/skills/in-whiteboard/SKILL.md` says what the tool now does, in the same landing: the hv section, the rule that the README must name hv's reader, which the profile's mediator replaces, and what `ask` and `announce` do.

## What a node loses, and does not

Nothing durable. An escalation that went to hv's inbox was read by nobody until the mediator ran, on every board whose README named one; it now goes to the mediator's inbox, which is read at the same moment. A node with a complaint about the mediator records it as a watch-out on its own board, which every pickup prints in full to every peer.

## Rulings since this package was written, 2026-11-10

These are vc's rulings under the pen, made on ic's read of the dispatch rows and cc's read as builder, and written here once hv ratified the terms. Where one disagrees with the text above, it stands and the text above gives way.

The hv-write check sits in the facade, the identity door, so MCP's node-taking writes meet it, and one arm drives an MCP write as hv from a non-mediator session. It is stated once, in the table's invariants. The help for `wb ask` and `wb announce` says "on a board that declares a profile".

## Rulings on the second row read, 2026-11-10

These are vc's rulings under the pen on ic's read of this package against the dispatch table at d6f56d7e6. Where one disagrees with anything above, the section before it included, it stands and the text above gives way.

The rule is the table's INV-12, carried in this package's bank with the facade check: "On a board that declares a profile, only the mediator's session or a shell with no session writes as the human seat". A write that takes `--node <human seat>` is refused unless the session it comes from is bound to the mediator, or no Claude session is present. The check sits in the facade, so the CLI and every MCP tool that takes a node meet the one check and no dispatch arm repeats it. The refusal is exit 1 with `error:`, and its remedy names the mediator. A board that declares no profile is unaffected.

The session-to-node read is one library function, `session_node`, with `mediator_of(board)` beside it. dc builds both in WP-08, and this package calls them and extends `mediator_of` to read a declared profile through `teams.rs`: the last layer that names a mediator wins, else the base's.

Twelve rows take `--node`, and seven of them are on MCP: archive, pickup, touch, release, claim, unclaim and clear. The MCP arm drives one of those seven. It also asserts that an MCP caller always carries a session id, since the server inherits `CLAUDE_CODE_SESSION_ID` from the session that started it, so "no session" is never an MCP path.

No new row and no new flag. `wb ask`, `wb announce` and `wb init --apply` change in help and behaviour as the section above says.

## Depends on

WP-03.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0092/acceptance.md`, under the `WP-05` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
