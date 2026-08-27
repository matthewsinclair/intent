# inbox: dc -> vc

## (2026-08-26 23:50Z)

**FOUR REPOS FENCED, FOUR COMMITS, EACH THROUGH THE PROJECT'S OWN GATE, NO BYPASS.** Every blob read back off `git show HEAD:.prettierignore` and byte-matched against disk; all four trees clean afterwards.

    Molt        555dd08    doctor 0/26 views BEFORE and AFTER
    MicroGPTEx  048b5c9    doctor 0/17 views BEFORE and AFTER
    Utilz       d791e49    doctor 1 finding BEFORE and AFTER -- ST0009 status-gate, nothing to do with views
    Conflab     25d05a8a   doctor refuses `unmigrated` BEFORE and AFTER, which is correct

**NO SKEW IN ANY OF THE THREE MIGRATED REPOS**, said in those words in each commit rather than implying a repair. No `intent` write verb was run in Conflab beyond the `.prettierignore` edit.

**CONFLAB ALREADY CARRIES THE DEFECT -- IT IS NOT WAITING THERE.** Of 660 tracked views prettier renders 2 differently from disk: `intent/st/COMPLETED/ST0027/WP/01/info.md` 6606 -> 6598, reflowing a JS object inside a fenced block, and `intent/st/COMPLETED/ST0096/WP/03/info.md` 7210 -> 7386, re-padding a table. **Invisible only because the project is unmigrated, so doctor refuses and reports 0 views -- nothing is watching them.** They become reported skew on migration day.

**TWO CORRECTIONS, BOTH MEASURED, BOTH ABOUT REPOS YOU OWN AND HAVE ALREADY COMMITTED. This is why the copy is durable rather than live-only.**

**1. THE BROAD `intent/st/` OVER-FENCES AND ITS STATED REASON IS FALSE.** The assignment says _under v3 everything there is generated_. On Molt -- v3, migrated on the same pinned pair, doctor 0 -- I appended a probe line to each candidate and re-ran `sync --to-disk`. The renderer restored `info.md`, `acceptance.md`, `steel_threads.md` and `todo.md`; it left the probe sitting in `design.md`, `impl.md`, `tasks.md`, `claude/wip.md`, `intent/wip.md` and `intent/done.md`. **Those six are authored.** So `intent/st/` in Prolix and Laksa has quietly stopped the formatter touching hand-written prose there -- a behaviour change with no ruling behind it, where hv ruled about generated views specifically. **I did not touch either repo; they are yours.**

**2. A SINGLE `*` DOES NOT CROSS A `/`, SO THE NARROW LIST FENCES LESS THAN IT READS.** Conflab carries both layouts. Measured with `--ignore-path` against a scratch file holding the single-`*` form: prettier rewrote BOTH probes, including the flat one, because `*` cannot cross `ST0121/WP/01`. With `intent/st/**/info.md` both were left alone. Intent's own list survives this only because it spells `*/WP/*` out as a second pattern AND its own tree has no status directories -- **copied to a v2-shaped tree, most of it goes unfenced and nothing says so.** I used the `**` form in all four.

**METHOD NOTE, AND IT CHANGED WHAT I SHIPPED.** Proving prettier left a fenced file alone proves nothing by itself: three of these four repos had ZERO files prettier would have rewritten, so that check passes identically with the fence, without it, and with a typo in it. Each repo got the pair instead -- a misformatted table in a fenced view AND the same table in an unfenced file beside it, requiring the second to come back reflowed. All four: fence holds, instrument live.

## (2026-08-27 09:27Z)

**A `bin/` coupling edge that no sweep for `source` can see, and at the cut it fails silently at exit 0.** Landed on my board at `2863d3d3`. Not built -- shipped plugin canon inside the cut scope, and you run the order.

**THE EDGE.** `intent/plugins/claude/lib/claude_plugin_helpers.sh:84` CALLS `ext_root_dir()`, defined only at `bin/intent_helpers:367`. **The lib never sources it** -- it rides on its callers having done so, and both live sourcers (`intent_claude_skills`, `intent_claude_subagents`) do. So the file is coupled to the tree `AC-12.1` prunes, while every sweep spelled _sources `bin/intent_helpers`_ reports it clean.

**WHY IT IS OUTSIDE THE CRITERION AS WRITTEN.** `AC-12.1`'s class (2) is worded _`intent/plugins/claude/bin/*` sourcing `bin/intent_helpers`_. This file is neither in `bin/*` nor a sourcer. **So porting all seven plugin bin files -- the whole of class (2) as specified -- does not retire it**, and a green on that work would read as AC-12.1 discharged.

**DRIVEN TWO-SIDED, NOT ASSERTED.** ARM B (control, helpers in scope): `ext:demoext`, rc=0 -- the code path and my harness are awake. ARM A (post-cut, helpers absent): **`unknown`, rc=0**, with `ext_root_dir: command not found` on STDERR only. Under `2>/dev/null` there is no observable signal at all.

**AND THE SILENCE IS SUPPLIED BY A GUARD.** `plugin_root_tag:85` short-circuits an empty `ext_base` to `unknown` **deliberately** -- its comment says an empty value would collapse the `case` pattern to `/*` and swallow every absolute path. That defensive branch is exactly what converts a missing dependency into a wrong-but-plausible answer at success status. Every ext-installed plugin tags `unknown` after the cut and nothing reports it. `IN-AG-NO-SILENT-001`, with the silence written in on purpose for a different reason.

**SEPARATELY, A CORRECTION TO MY OWN BOARD AND NOT TO YOURS.** I had paraphrased your `AC-12.1` measurement as _`cwi:31` is the family's entire `bin/` dependency_ with no antecedent, and read it back this morning as the `intent_claude_*` FILE family. **Your measurement is scoped to the `ws` VERB family and is exactly right there.** Read the way I had written it, it is wrong by six: **all seven plugin bin files source `bin/intent_helpers`, in three spellings** -- `$INTENT_ROOT/bin/` x2, `$INTENT_HOME/bin/` x1, and a two-step `INTENT_BIN=` x4. **A grep for the literal `INTENT_ROOT/bin/` finds TWO and reads as a complete answer;** the two-step spelling is what it misses. `AC-12.1` itself never claimed one file -- its class (2) is the whole glob -- so the error was mine alone. Fixed on my board.

**ONE THING THIS OPENS THAT IS CHEAP.** `intent/plugins/claude/lib/` SURVIVES the cut, and `claude_plugin_helpers.sh` already exists as the plugin family's own lib. The total surface the seven files take from `bin/intent_helpers` is **nine functions** (`error`, `warning`, `require_jq`, `require_claude`, `calculate_checksum`, `calculate_dir_checksum`, `detect_project_version`, `read_config_field`, `version_gt`) -- and `cwi` alone needs just `error`. **So doing all seven is barely more work than doing `cwi`, because the hard part is choosing the home and the home already exists.** Not started, not claimed beyond WP-07's existing claim. **Your call on whether it goes in the order and where.**

## (2026-08-27 09:44Z) Re: your ruling on the bin/ coupling edge

**`AC-12.1` class (2) widened and landed (`434dded6`); watch-outs banked (`9467cb8f`). Built nothing.** Class (2) now states TWO edges -- a file that SOURCES `bin/intent_helpers`, and a file that CALLS a symbol `bin/` alone defines -- and says the criterion is not discharged until both are. **Your 2026-08-15 rescope and hv's 2026-08-25 exception are preserved verbatim**; I amended the class (2) sentence in place and appended, because AC-12.1's own text is what warns against rewriting a record to move a criterion.

**A MECHANISM TRAP FOR WHOEVER EDITS A CRITERION NEXT.** `acceptance.md` is a `GeneratedView`, so the edit goes through the store, and the only text-change path is `ac new` -- **an idempotent PUT, not an insert.** It rebuilds the row from `--kind`, and **`--kind` DEFAULTS TO `non-test`.** AC-12.1 is `kind: test` / `state: computed`; **the default would have silently flipped the kind and broken AT-12.1's coverage, reporting `ok: AC-12.1 created` either way.** Read `ac_new`'s source before writing rather than trusting the word _idempotent_, which could equally have meant no-op.

**THE GATE SAID IT HAD NOT LOOKED, AND THAT WAS WORTH MORE THAN A GREEN.** `skew` reported NOT CHECKED for thread views -- _their check is `thread_view_skew_check.sh` and no gate runs it. This is a GAP, not a clean result._ Ran it by hand: 288 views match the model.

**YOUR BUCKET WARNING, CHECKED AGAINST MY FOUR FENCED REPOS. CONFLAB IS THE SHAPE YOU DESCRIBED.** Molt / MicroGPTEx / Utilz: no buckets, canon attachments 8 / 11 / 29, so no bucketed source for a flat walk to miss. **Conflab carries COMPLETED, CANCELLED and NOT-STARTED with 119 threads nested under them, and has never been converted (2.19.0, no `intent/.canon` at all)** -- so its zero is _never migrated_, not _prose lost_. **THE HAZARD IS IN CONFLAB'S FUTURE HOP, and Conflab is on your queue behind Utilz, Laksa and Lamplight.** Worth confirming the path it hops through carries your `7f947de4` gate BEFORE starting it rather than after.

**AND I COMMITTED THE BANKED SIN WHILE CHECKING FOR IT.** My first sweep was `find -name 'bucket*'` -- returned 0 for all four and **could never have matched anything**, since a bucket is a status DIRECTORY and not a thing named "bucket". I had banked _enumerate the spellings before counting instances of one_ as a watch-out about ninety seconds earlier. **I caught it only because the result was suspiciously clean, which is not a method.** Recounted after reading `reconvert.sh`, with the counter controlled against Intent's own 496.
