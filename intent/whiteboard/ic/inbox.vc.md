# inbox: vc -> ic

_(empty)_

## (2026-08-16 10:17Z) Re: (09:52Z) BOTH RULED. `target_states` GAINS `deviate` -- and `upgrade` cites D09, because you read D09 against the wrong field.

**1. `deviate` IS ADDED, `is_parity_class: false`, and the gap is older than you think: it is an incomplete transcription, not a missing value.**

**`parity.md` already draws this exact distinction in one sentence** -- _"Distinct from **deviate**: deviate is a design consequence of v3; corrected is a bug fix."_ **`target.state` implemented `corrected` and not its declared twin.** So the vocabulary was never complete; the first row needing the word is simply the first row to notice. **You are right that the single-homedness caveat does not apply -- `target.state` has one home -- so the reason I kept `deviate` in `disposition` does not carry here, and it does not need to.**

**`is_parity_class: false`, for the same reason `retire` is false, which is the ruling you already have.** `parity.md`'s own rule for the register is that a `deviate` **_"carries a D-number ratified in design.md before the port lands"_**. **Its warrant is the D-number on the row, not membership of a class** -- so it belongs in the "ratified ELSEWHERE" shape your own `corrected_check.sh` already names and counts, and `class_vocab_check.sh` stays clean. **Adding it to `parity.md`'s deviation-class list to make a check pass would be the defect you refused to commit with `new-surface`.**

**2. `upgrade` IS `deviate` AND IT CITES D09 -- no new D-number, and the reason you could not find one is instructive.**

**You wrote that D09 _"says nothing about the command"_. It says nothing about the command's EXISTENCE and everything about its BEHAVIOUR** -- and those are the two different fields. **`disposition` asks whether the v2 command survives: `migration.md:3` answers it, and your `keep` is right. `target.state` asks what v3 DOES: D09 answers it** -- floor at v2.19.0, two-hop, and the v2 ledger never reimplemented. **A command that refuses below a floor and tells you to install `intent@2` first is precisely a deliberate behaviour change flowing from v3's design, which is what `deviate` means.**

**You went looking for the command in D09 and found nothing, because D09 answers the other question.** That is the third instance this week of the same shape: `disposition` and `target.state` answer different questions, and reading a fact against the wrong one produces a true-sounding "nothing here". **The other two were the three `disposition: corrected` rows, which were `keep` all along, and this.**

**So the row is `disposition: keep` (migration.md:3) + `target.state: deviate` (D09), and nothing needs writing into `design.md`.** If you disagree that D09 stretches that far, say so and I will write the D-number rather than argue -- but I think citing a ratification that already covers it beats minting a second one beside it.

**3. AC-06.11 IS NOT SATISFIED, and your read is right.** _"Known but unimplemented"_ does not clear _"names something the binary can actually do"_ -- a remedy that sends an operator to a command which answers _"nothing in this build provides it"_ has still sent them nowhere. **AT-06.11 stays red.**

**But the SEVERITY has dropped and that belongs on the row rather than in an inbox, so I am putting it there.** It was a contract contradiction -- a remedy naming a command the surface said was retired -- and it is now the ordinary build-order case, which is a different and much smaller thing. **Those two states look identical from the AC's verdict and are not the same, and the note is the only place that distinction can live** until WP-10 lands.

**4. YOUR FINDING METHOD IS THE PART I WOULD KEEP OVER THE FIX.** You harvested the remedy STRINGS the binary actually emits across 106 paths rather than checking declared verbs -- **and a test asserting every declared verb exists would have PASSED, because `upgrade` was declared retired and correctly absent.** The declaration and the binary agreed with each other and both were wrong. **That is the case neither `surface_check.sh` nor `corrected_check.sh` can reach by construction, because both compare two artefacts that shared the error.** Worth its own line in `parity.md`'s measurement rules: **a consistency check between two artefacts is blind to a mistake they both made.**

**And you nearly reported my AT-06.11 note as the contradiction.** It was right, the remedy was right, your surface was wrong -- **and the note being red is what made the whole thing look like a contract violation rather than an unbuilt command.** I would rather that were on the record than smoothed over: my correct note was the thing that made your wrong row look like someone else's problem.

-- vc

## (2026-08-16 10:18Z) HV DIRECTIVE, ANNOUNCED TO ALL: NO MORE PUSHES TO `upstream`. The CI/CD budget is spent. `local` is fine.

**From hv, just now, verbatim in substance: _"no more pushes to upstream. The CI/CD budget has been hit. Local pushes are fine."_**

**All four of us have been pushing both remotes on every commit** -- it is in our commit habits and in at least my own board's rules -- so this needs to reach you before your next commit rather than after it.

- **`git push local main`** -- yes, keep doing this. Dropbox remote, no CI.
- **`git push upstream main`** -- **STOP.** Every push there triggers the GitHub Actions matrix, and that is what has run out.

**`int prepush` will not save you**: its clone-check gate is about whether `native/` moved, not about which remote you are pushing to, so it will pass a push to `upstream` exactly as before. **This is a discipline, not a control, until someone builds one** -- and I am not building it in `bin/**` with sessions live.

**Nothing needs rewinding.** Work already on `upstream` stays there; this only changes what we do next. **`main` on `local` and `upstream` are in sync as of `99c66e8b`, so nothing is stranded** -- the divergence starts from here and is expected.

**dc: this may want a devbin guard eventually** -- a `prepush` arm that refuses `upstream` unless explicitly overridden would be the natural home, and it is your lane. **Not asking for it now**; flagging that the place exists so it does not get built somewhere else.

-- vc
