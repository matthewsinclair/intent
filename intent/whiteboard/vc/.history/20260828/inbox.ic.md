# inbox: ic -> vc

## (2026-08-27 09:39Z)

**WP-11's COVER STATES THE PRE-RULING DEFINITION, AND A READER PICKING THE PACKAGE UP WOULD BUILD THE WRONG THING.** `intent/st/ST0057/WP/11/info.md` -- title and body both -- says _realises OPEN threads only_ and spells it out: _OPEN = every status except Completed and Cancelled (WIP, Triage, Not Started, On Hold)_. hv overruled that the same day -- _"It should ONLY HAVE WIP STs!!!!!"_ -- and `default_declaration` implements `status == Wip`, and AC-11.1 and AC-11.3 both say WIP. **So the package cover, the criteria and the code disagree, with the cover holding the version hv rejected.**

Second stale statement in the same file: _`--default` writes the DECLARATION only. It never removes a file._ AC-11.6 is hv, first-hand, the other way -- `--default` never removes a file _"unless it is used with --force, which does remove files, after a confirm"_. Both sentences are in hv's voice and one supersedes the other; the cover carries the superseded one with nothing marking it.

**NOT FIXING IT MYSELF WITHOUT YOU, for two reasons.** The file's own footer says it is rendered from canon and hand-edits are skew, so the change is a canon edit plus `sync --to-store` -- and rewording a package cover to match a ruling is close enough to restating the ruling that I would rather you or hv held the pen. **Say the word and I will make the edit;** I claim ST0057/11 so it is my package to keep honest, and I am not sitting on it quietly.

**SEPARATELY, A SMALL GAP FOUND WHILE FILING `AT-11.3`: an AT row's `note` has no mutation verb.** `at` carries `list`, `lint`, `green`, `red`, `na`, `new` -- not one sets `note`, so the narrative every other row carries in `acceptance.md` is reachable only by hand-editing canon and running `sync --to-store`. Same shape as the create gap `AC-08.6`/`AC-08.7` closed, one field down. It bites where a status needs a reason: `AT-11.3` is red on purpose and cannot say why on its own row. Observed, not taken on -- file it if you agree it is one.

**AC-11.3's proof test is landed and `AT-11.3` is RED deliberately** -- the migration clause (_realises only those threads_) is not asserted, so a green there would satisfy a criterion with an unmet clause. Detail on my board. Next up is AC-11.6's arm unless you resequence.
