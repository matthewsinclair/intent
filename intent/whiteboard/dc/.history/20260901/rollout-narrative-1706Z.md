# The devbin rollout afternoon -- what happened and what it taught

Archived 2026-09-01 17:06Z, before the localfold. Pre-fold board: `wip-prefold-1706Z.md` (sha `58a13929`).

## The shape of the afternoon

hv put every node on hold to roll devbin out, and then the work happened almost entirely in the channel between four nodes rather than in any tree. Five TN001 revisions landed (v0.5 -> v0.9), every one of them driven by a peer finding something wrong with the previous one, and the document ended up being about the failure mode that produced it.

## TN001, v0.5 through v0.9

- **v0.6** -- the per-estate status column removed, rows kept as DATED EVENTS. devbin-vc's ask, vc's adjudication, hv's blessing. Applying vc's tense test found what none of the three of us had named: **the Intent row read "NO -- not one crate" from v0.1 to v0.5.** It stopped being true at `71a96213` and stayed on the page for the rest of the day, thirteen lines below the note's own status line saying the opposite. Not rotting -- FALSE, in a document circulated as fleet canon, contradicted by its own header.
- **v0.7** -- the crate table said 257 -> 5; cargo says 6. vc predicted it from the manifests, declared their reading a proxy they could not adjudicate under the hold, and named the one command that would settle it. Adjudicated here: `intentd` took a second declared target back at `33ac4348`, 1h37m after the 5 was recorded, and three revisions passed over the table including `73857a72` whose own subject line names `33ac4348`.
- **v0.8** -- the amendment that corrects a rule we had all been prescribing.
- **v0.9** -- the citation lands against devbin-vc's `4afef84`, and a boundary on what hv's cold cycle verified.

## The finding of the day: A PRINTED NUMBER GETS READ INSTEAD OF RUN

Three of us converged on "state the figure with the command that regenerates it beside it". I prescribed it to devbin-vc. vc put it in three board entries and let hv see it as the estate's standing advice.

**The stale table carried its regenerating command directly underneath it.** The command was right there and nobody ran it, including the two people who wrote the rule.

So the cure buys AUDITABILITY and buys nothing for CURRENCY. Amended, in place rather than beside:

- **LIVE figure -> print the COMMAND ALONE, delete the number.**
- **HISTORICAL figure -> DATE it.** A dated event is not a live claim.

**And this vindicates devbin-vc's original ask more than any of our refinements did.** They asked for the verdicts to be dropped so a reader would run the command. Three of us improved that into a scheme for printing better figures, and the scheme is what failed.

## The classes, from four nodes, all one family

- **A control chosen from the cases you can drive selects for cases that pass** (devbin-vc, refining mine). My two `time` controls were arms A and D -- exactly the two configurations that cannot exhibit the failure -- so the untested-mechanism error was DOWNSTREAM of controls that had already told me there was nothing to explain.
- **A message correcting someone's state is itself a state claim** (cc). I refused to relay vc's reading and re-drove it, then applied that discipline to the SUBJECT and not to the claims about the RECIPIENT. Both of mine were one command away, and one was already recorded on my own board.
- **A message cited as a document** (devbin-vc). A cross-session message has no path, no revision, no verblock and no guard. Moving a claim from the channel into the document upgraded its authority to something checkable while removing everything that would let anyone check it. No control catches this -- the instrument is fine and the ATTRIBUTION is wrong.
- **A document applies its own rule to the claims about other people and not to the claims about itself** (vc). Three instances in TN001, every one a figure about the authoring estate. Nobody audits the author's claims about the author.
- **A dangling symlink is invisible to a PATH resolver** (conflab-vc). Once a cycle darkens a binary, every later run is CORRECTLY silent about repeating the damage -- the detector can only warn about binaries that still work. Worse than a control that cannot go red, because it used to.

## My own errors, kept as two kinds rather than one count

**Design faults** -- an instrument that cannot exhibit the failure:

- The `time`/`dvb` autoload diagnosis, controlled with two arms that could not fail.
- A fleet-wide grep without `-i` against a SHOUTING-CAPS corpus, which returned clean. **Caught only because the control I ran next -- a phrase I knew was present -- also returned nothing.** Had I stopped at the first result I would have told a peer their quotation was invented.
- `DIRT_SCOPE` read twice from the wrong corpus (`lib/templates/hooks`, `bin/.devbin/lib`; it lives in `bin/.devbin/cmd/shared`, which is project-owned). Both returned clean, on the one fact I had flagged as a resume condition BECAUSE it could move.

**Attention faults under load** -- the disproof already in hand (cc's distinction, kept separate deliberately; one command answers both, which is the only sense in which the cure is shared):

- Told cc their LockHeld commit was blocked when `3fc1d152` was an ancestor of HEAD **and recorded on my own board**.
- Told cc their board carried a claim it never carried.
- Withdrew the `fullcycle` objection on devbin-vc's ordering -- correct about the sequence -- and never asked whether the build phase produced release artefacts. It did not. Three phases went green on Conflab with hv's CLI dangling.

## What I got right and want to keep doing

- Refused three times to write another estate's unverified measurement into fleet canon, including when a peer asked and when it would have been convenient.
- Refused to supply a census figure to hv after my last two censuses were both wrong in the direction that overstates.
- Put the unverified marker ON THE FACE of the escalation (cc's ask) rather than in anyone's memory, naming three holders and the required re-driver.
- Honoured the resume condition before doing any work on the lift: re-read `DIRT_SCOPE` from the guard rather than off a board.
- Measured devbin-vc's general claim about consumer git history before it became canon -- it was overstated, and the vendored runtime IS committed here.
