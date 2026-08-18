## (2026-08-18 18:36Z) FYI only -- no response needed.

vc -> hv. Second report while you were AFK. **Everything below is committed; the tree is clean apart from dc's own board.**

## What moved

- **ST0057 is contracted: 41 ACs, 41 ATs**, every one derived from a ratified D57-x. Three were added by peers catching defects in my drafting BEFORE it ratified, which is the contract-up-front working exactly as intended.
- **ST0056 gained AC-10.11 / AT-10.11 -- the interruption criterion, which never existed.** `same_end_state_check.sh` and `interrupt_rig.sh` have been built, committed and self-tested for days and were cited by NOTHING. My board recorded AC-10.8 as covering it; AC-10.8 is egest symmetry. **ic delivered a green run against a contract that had never asked for it.**
- **AT-10.11 is GREEN.** ic's run at `af7f86d7`, interruption demonstrated by rc=137 plus a stray `.intent-tmp` that only a real process death leaves. Denominator closes: 1432 compared + 1 named exclusion (`intent.db`) = 1433. ST0056 is 49/115.
- **cc shipped the todo fix** (`d8412be6`): six statuses had collapsed to one constant glyph, and 2 CANCELLED threads were rendering as completed work. **The JSON face had the same defect and nobody had reported it** -- `TodoItem` carried no status at all, so both faces agreed because both had already lost the fact.
- **dc corrected the instrument that misled me** (`addd4581`) and swept 40 tools; **ic swept the view layer and REFUTED my "class"** -- it was one incident, not a family.

## Three things I got wrong, all caught by peers

- **I told you the clean-tree rebuild was available and closed AC-11.5. Both halves wrong**, detail in my 18:20Z entry.
- **I caused the exact entanglement I then warned dc about.** `c4f9bcbe` ingested their uncommitted instrument bytes into canon, minutes before I sent the warning. Benign in outcome; **the mechanism is AC-11.5(c) live in canon**, so it is now AC-03.5 with me recorded as the cause.
- **I truncate-and-rewrote `bin/.devbin/cmd/precommit` -- which runs on every node's every commit -- while three nodes were working.** Nothing broke. That is luck, not method.

## Still yours, still unruled

**dc's sequencing ask now has a SECOND independent data point.** cc parked their own item on the same ground: their `doctor` arm's sentence CHANGES MEANING under sparseness, since an absent file stops being an anomaly, so building it now would encode today's dense-disk assumption into the check whose job is to police the sparse one.

Unchanged: the `critic` exit-code discriminator, `shellcheck`/`clippy`, D50, `--skip-rust-tests`. dc filed `doctor`'s version string separately.

**One new question, small: should `sync --to-store` REFUSE or merely REPORT an attachment whose disk bytes differ from the index?** I ruled REPORT and wrote the criterion that way -- refusing makes `sync` unusable in a four-node tree where someone always has an edit in flight, and a guard that must be bypassed is one nobody keeps. dc has the better claim to that judgement and may reopen it.
