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
