# vc DOING, executed narrative folded 2026-09-08 15:07Z

Every block below is WORK THAT HAPPENED. It left DOING because it is done, not because it stopped mattering; the lessons that outlive it are in `## Watch-outs` on the live board. Verbatim pre-fold: `wip-prefold-1507Z.md`.

### `0280` -- THE PROPAGATION PATH `AC-15.3` IS ABOUT IS ITSELF BROKEN, FOUND BY APPLYING THAT ROW'S STANDARD TO MY OWN FIX

**`intent claude skills sync` HOLDS FOUR SKILLS AS CONFLICTED AND THREE ARE BYTE-IDENTICAL TO CANON.** Computed with `payload.rs`'s own algorithm: `in-tca-audit` canon `262b9722` == installed `262b9722`, manifest `5bc75387` (recorded 2026-04-28). Same for `in-tca-finish`, `in-tca-init`. **The conflict test rests on the manifest baseline, and when canon and installed agree there is nothing to resolve whatever the baseline says.** `in-standards` and `in-detrope` are BOTH correct in the same run, so it is a missing case and not a broken comparison. `in-essentials`'s `changed here` limb is false too -- its installed tree is byte-identical to canon at `HEAD~1`.

**THE HARM IS THE MESSAGE, NOT THE VERDICT: _copy your edits out first if you want them_.** An operator who believes it declines to force, CORRECTLY on the evidence, and the skill stays stale. **It recruits the careful reader into inaction** -- the pessimistic-direction failure, in a tool. 4 of 23, `in-essentials` among them, so today's `20841a5e8` correction reaches nobody on this machine. Routed to cc with the caveat that I read the COMPARE path and not the WRITE path, so why the baselines went stale is unmeasured.

**AND I COMMITTED THE STRUCK DEFECT WHILE STRIKING IT.** My first drift census compared `SKILL.md` ALONE and reported 2 drifted; the tool said 5. **That is precisely the v2 behaviour I had spent the morning removing from `AC-15.3` as a thing v3 does not have** -- written the correction, then built the bug into the instrument checking it, inside an hour. Skills are TREE-scoped; the live manifest's `checksum_scope` reads `tree`, which is a third witness for the strike.

### `22 ENGINEERING` AUDITED UNDER ic's RULE, AND IT SURVIVES WITH ITS BOUND STATED

ic declined `ST0064 AC-01.7` on the grounds that **blocked on an external dependency and unbuilt are different states a gate reports identically** -- Apple credentials only hv can supply. Accepted and generalised. **I then turned it on my own figure, because accepting the distinction and not checking my own number would have been the cheap move.** A prose scan for external-dependency tells, **positive-controlled against `AC-01.7` itself, where it fires on `notarised per D38`**, returns ZERO over the 22.

**TWO LIMITS, RECORDED RATHER THAN DROPPED:** it is a text scan, so a row blocked without saying so in words the pattern knows is invisible; and **`ST0068` AC-03.1/03.2 are in the 22 and are not startable BY ME** -- they need a reader who did not consolidate the document. **22 engineering, of which 2 need a reader who is not vc.**

### WP-15 STARTED 2026-09-08. `AC-15.3`'s PREMISE WAS FALSE AND IS STRUCK; `AC-15.2` IS TWO AXES GREEN, ONE NOT VIABLE AS BUILT

**`AC-15.3` CARRIED A v2 DEFECT AS ITS RATIONALE AND v3 DOES NOT HAVE IT.** The row read _`intent claude skills sync` checksums `SKILL.md` alone, so a skill whose scripts changed without its `SKILL.md` changing does not propagate_. **`payload.rs` gives skills `Shape::Tree` / `SCOPE_TREE`** -- every file's path and content -- and `SKILL.md` is only the MARKER that makes a directory a unit. **`skills_sync::a_change_confined_to_a_script_propagates` pins the fix and is green (31 arms, driven).** Struck in place via `ac edit`; the REQUIREMENT and the evidence standard are unchanged, and the reason they survive is the stronger claim: driving a consumer separates a removal that REACHED one from a source tree that merely looks clean, whatever the checksum scope is. **An instrument built to the struck sentence would have hunted a bug that is already fixed and pinned.**

### `AC-15.2`: VERB AND FLAG AXES GREEN AND BELIEVABLE; PATHS AXIS 0-REAL-OUT-OF-43 AND NOT SHIPPABLE

- **VERB -- 33 cited spellings, all resolve.** Population is **142 paths from `families` (128) AND `new_surface` (14 more)**.
- **FLAG -- 3 (verb,flag) pairs, 0 suspect**, span-scoped, `disposition: retire` counted as NOT shipped surface.
- **CONTROLS, all three firing on planted references:** a stale verb, a RETIRED flag (`doctor --fix`), an undeclared flag. **The live corpus stays clean under the plant**, so the zero is not the instrument failing to look.
- **PATHS -- BUILT, MEASURED, AND REFUSED BY ME.** 43 candidates, **0 real**. Not shippable and the reason is a taxonomy, not a bug: a cited path resolves against FOUR different roots and some are not paths at all -- Elixir arity (`mount/3`, `Map.get/2`), rule-pack ids (`elixir/code`), template placeholders (`STXXXX`, `WP/NN`, `YYYYMMDD`), consumer-tree paths (`deps/ash/usage-rules.md`, correctly absent HERE), and install-tree paths (`rules/elixir/lv/`). **The taxonomy comes before the checker.**
- **FILE LAYOUTS axis: not started.** Do not report this AC on the two green axes.

### THREE INSTRUMENT DEFECTS I BUILT AND CAUGHT IN ONE SITTING, ALL THE SAME FAMILY

1. **MY OWN POPULATION WAS SHORT BY 16 ROWS.** I walked `families` and not `new_surface`, so `intent fc` read as UNRESOLVED and **I nearly filed a false finding against `in-essentials`, a skill loaded on every session.** `fc` is in the table and in the binary. **The denominator attack, committed on the instrument built to detect it** -- caught only because I verified the finding instead of reporting it. `flag_reachability.rs` already walks both populations; the precedent existed and I did not read it first.
2. **LINE-SCOPED FLAG ATTRIBUTION RAN AT PRECISION 0 OF 4.** `--to` on a line naming `ac descope` AND `ac withdraw` was bound to the wrong verb. **A flag binds to the invocation in ITS OWN span, never to a verb elsewhere on the line.**
3. **AND FIXING (2) SILENTLY KILLED THE NEGATIVE CONTROL.** Span-scoping dropped the verb population 33 -> 27 and made `intent to deceive` invisible, so a stale reference written in PROSE would no longer be seen. **The two axes need different scoping and that is a finding, not a preference:** a verb citation is checkable anywhere, a flag binding needs an invocation boundary. **A narrowing that improves precision and removes the control is indistinguishable from a fix.**

### THE `AC-15.2` FALSE-POSITIVE CONSTRAINT, NOW WITH A SECOND WITNESS

My board already carried it from `in-tca-init:52` (`~/.intent/ext/`). **`in-essentials:29` is the second: `intent/llm/AGENTS.md` is cited by the sentence RETIRING it** -- _legacy ... which is retired and should be removed_. **The best-maintained skills read as the stalest**, because retiring a thing means naming it. `.claude/skills/` at `:41` is the third shape: an operator-home path in a PROHIBITION, correctly absent from this repo.

### `intent doctor --scope` IS BUILT, DRIVEN AND GREEN -- 2026-09-07, in intent-cc's joint commit with `organize -v/-q`

**`--scope live|all|closed`, default `live`, exactly as hv ruled.** `Scope` + `Report.scope`/`out_of_scope` in `intentsvcs`; the two MODEL checks scoped at EMISSION with the thread in hand; view-skew and unattached always-reported as DISK facts; the summary prints the withheld count whenever non-zero AND survives `--quiet`. MCP takes `Scope::All` -- the table declares the flag `exposed_on_mcp: false`, so a narrowed MCP report would name findings its caller has no way to reach, and the caller cannot narrow for itself without committing `0256`.

**DRIVEN ON REAL ESTATES WITH A DEBUG BINARY, DELIBERATELY NOT REBUILDING RELEASE**, so nothing untested reached anyone's PATH. **Lamplight 71 lines -> 25, 64 findings -> 18. Conflab 186 -> 136, 50 announced as withheld.** The arithmetic reconciles three ways and the third is the one that proves the boundary: live 17 + closed 46 + skew 1 = 64, **the skew appearing under ALL THREE scopes** rather than being asserted to.

**THE NUMBER ON THIS BOARD LAST NIGHT WAS WRONG AND THE PARKS ITEM SHRANK WITH IT.** I wrote "124 of the fleet's counted findings, 44 of Lamplight's 64 being the parks family". Measured against thread status: **96 of 331, and of Lamplight's 44 status-gate rows only 31 are on closed threads. The other 13 sit on WIP threads and are correct live signal that SHOULD report.** So the parks ruling is worth 31 rows on one estate, all of them already hidden by the default hv ruled -- **it may not need a ruling at all.**

### A5 AGAIN, ON MY OWN TEST, AND ONLY A CONTROL FOUND IT

**`the_withheld_count_is_findings_and_not_threads` ASSERTED 2 ON A FIXTURE OF ONE THREAD WITH TWO FINDINGS, AND BREAKING `+= found.len()` TO `+= 1` LEFT IT GREEN.** `Report::admit` is called TWICE per thread -- once for the model checks, once for the gate arm -- so a per-CALL counter also reaches 2. **Findings, threads and admit-calls collided at the fixture's own number and nothing about the green read wrong.** Three orphans is the smallest fixture where all three differ (3, 1, 2). **cc hit the identical class the same hour**: their fixture had 2 directories and 2 action rows, so a withheld line printing the wrong quantity would have read 2 == 2. **Both of us wrote the control BEFORE the fixture could distinguish what it was controlling for.**

**THE RULE THAT COMES OUT OF IT, SHARPER THAN "POSITIVE-CONTROL THE INSTRUMENT":** state what the test must SEE to fail, then check the fixture can produce a number that differs from every neighbouring quantity. A control that confirms a green is decoration; only one that turns a green red is evidence.

### TWO CORRECTIONS FROM lamplight-vc, 2026-09-07 21:29Z, AND BOTH ARE MINE

**1. I REPORTED A FLEET NUMBER WITHOUT SAYING WHICH TREE IT CAME FROM.** Lamplight reads **64 on the working tree and 89 on the committed one**. The difference is **21 `kind: test -> non-test` rows across seven COMPLETED threads that I WROTE during this morning's fleet surgery and never committed** -- ST0248, ST0256, ST0270, ST0275, ST0298, ST0300, ST0332. lamplight-vc has deliberately left them uncommitted because **hv's D3 rules finished threads are history**, and both committing and discarding them are decisions neither of us may take. **So "Lamplight 88 -> 64" is conditional and I stated it flat.** Same class as everything I have been policing today: a population reported without its denominator.

**2. I TOLD lamplight-vc TO CLOSE ELEVEN WORK PACKAGES AND AT LEAST FIVE OF THEM MUST NOT CLOSE.** I wrote "the gate already passes so it will not refuse" with a general caution to read one first. **The caution was not strong enough for the case where a passing gate is the CORRECT state rather than a lagging one.** Three are parked (above); `ST0315/WP-16`'s third deliverable is outside any AC set; `ST0306/WP-03`'s own file says `all three ACs satisfied, one deliverable outstanding -- STAYS WIP`. **Measured read rate across cc and ac: 4 closes out of 12 examined.** My eleven was an over-estimate by roughly three.

### A6, AND cc's HALF IS BETTER THAN MINE

**`git add` IS THE PUBLICATION, NOT `git commit`. A STAGED FILE IS A PUBLISHED FILE**, and `--only` scopes YOUR commit while protecting nothing from mine. Stage and commit in ONE UNINTERRUPTED ACT, or announce you are holding the index. cc's wording, adopted verbatim.

**AND THE SHARED-FILE TWIN, WHICH I GOT WRONG WHEN I WARNED THEM.** I told cc their revert method was the hazard. It was not -- they were already using `cp` + `cmp`. **The real hazard is that their pre-mutation snapshot FROZE MY UNCOMMITTED WORK at one instant, so a write from me inside their window would have been silently rolled back to their copy.** The announcement is what protects against that; `cmp` only protects against their own mutation surviving. **Two different failures, and I conflated them.**

**A FILE CARRYING TWO NODES' UNCOMMITTED WORK CANNOT BE SPLIT ACROSS TWO COMMITS.** `git commit -- <path>` takes the whole file and hunk staging is interactive. Four files were joint tonight (`render.rs`, `intent-cli/tests/suite.rs`, both `surface/dispatch-table.*`), so it landed as ONE act by ONE node -- the only shape with no broken intermediate, because splitting would have put `--scope` code against a table with no `--scope` row and `dispatch::table()` is compiled in.

### `organise` WAS THE BIGGER HALF OF hv's ASK AND I MEASURED IT ON rc

**hv's ask names TWO commands. I measured `doctor` on LINES and `organise` on EXIT CODE, then reported it "never broken".** That is the same proxy substitution hv corrected me for on `doctor`, committed on the other half of the ask in the same breath. **Measured: `organise` prints ~3155 fleet lines against `doctor`'s ~95** -- Lamplight 2100, Conflab 636, Laksa 342. Lamplight's 2100 is 2072 `unclaimed:` lines, one per DIRECTORY, under a summary already carrying the count and a digest.

**cc BUILT `organize -v/-q` AND THEIR OWN JUDGEMENT CHANGED THE ANSWER.** Told to decide each class on its merits rather than sweep the body, they found Lamplight's default carries **25 `to-remove:` lines -- 25 files it is about to delete.** A wholesale sweep would have hidden them and rebuilt the exact defect `--apply` was minted to prevent. Only `unclaimed:` moved. **2098 -> 28 at the default, 1 under `-q`.**

### JOB 2 WAS ALREADY DONE AND I PUT IT IN A PLAN ANYWAY

I told hv the advisory detail still repeated ~90 chars of class policy 136 times. **It does not: `9331cb11` cut both legacy details from ~270 to ~150 chars**, and what remains is the deliberate short policy clause `doctor_advisory.rs` asserts on by name. **I was reading this board instead of the code.** No churn; a tested string is not worth thirty characters.

### DELIVERED TODAY, AND ALL OF IT DROVE hv's OWN `-v` RUNS

- **`9331cb11` -- THE REPORT GROUPS BY CLASS AND THE LEAD WORD AGREES WITH THE VERDICT.** Three output defects: (1) `Display` enumerated classes while the count asked `is_actionable()`, so Conflab printed **50 lines leading `residue:` above a summary reading `0 finding(s)`**, each carrying the remedy _nothing to fix_; the lead is now `FindingClass::lead()` and asks the predicate. **The comment it replaced claimed a protection it did not have** -- _"a match is refused by the compiler when the next variant forgets to choose"_ -- and a `_` arm is never refused, which is exactly what let the class through. (2) **`remedy()` takes no argument, so it CANNOT vary within a class** -- 45 identical copies on Lamplight, 186 on Conflab. One header, one remedy, every member. (3) Details carried ~150 chars of class policy, against `finding.rs`'s own rule.
- **`16f64586` -- `issues.edit` DECLARED, AND IT WAS NOT THE RENAME IT LOOKED LIKE.** `issue.set` is singular because it comes from the ENTITY-named generic setter; the verbs are plural because they come from the CLI FAMILY. The sources agree for `ac` and diverge for `issue`/`issues`. Renaming either half breaks its source. **Devbin now prints five lines under `-v`.**
- **`f5b602ef` (joint with cc), `a312534d`, and estate repairs**: Conflab 189 -> 0, Baize 73 -> 0, Prolix 4 -> 0, Intent 6 -> 1, Lamplight 88 -> 64.

### MEASURED ON LINES, WHICH IS THE METRIC THE ASK NAMED

`-v` / default: **13 estates at 5-8 / 1-2**. Intent 11/5, Laksa 15/5, **Baize 73/2**, **Lamplight 75/71**, **Conflab 195/2**. Baize and Conflab's `-v` is now genuine content -- 66 and 236 real notes, one line each, duplication gone.

### THE CORRECTION THAT MATTERS MOST, AND IT IS MINE

**I REPORTED "15 of 18 PRISTINE" AND hv ASKED HOW HUNDREDS OF LINES COULD BE PRISTINE. THEY WERE RIGHT.** I measured `rc=0` and zero COUNTED findings -- a number I could drive to zero -- when the ask was about OUTPUT. **Substituting a measurable proxy for the thing asked for is the same error as a narrowed denominator**, committed on the acceptance criterion itself.

**AND I SCOPED hv's ASK DOWN WITHOUT SAYING SO.** Told to fix it ALL, I handed Lamplight's 88 to lamplight-vc and reported it as "handed over". **That was a third of the problem, decided by me.** On the bounce: Lamplight's remaining 64 are 44 parks-family (hv's ruling, still unmade), 12 `covers nothing`, 4 pointing at a non-existent ST0338, 1 the new `--kind` guard correctly refused because the AT covers a TEST-BACKED AC. **None is mechanically fixable from outside -- that is what they ARE, not me declining again.**

### A5 -- A CONTROL THAT CANNOT FAIL FOR THE RIGHT REASON. TWO INSTANCES, ONE AFTERNOON, TWO NODES

**cc changed the class of the largest finding class in the fleet and all 1210 tests passed**, because the sibling test asserts DETAIL TEXT and the sentence is identical on both sides. **My first fixture for the `[n/a` fix passed with the fix REVERTED**, because I wrote `(non-test)` into the row and that marker excludes it from the path rule before the bug can be reached. **The rule is NOT "positive-control the instrument" -- we both already had that.** It is cc's phrasing and it is better than mine: **state what the test would have to SEE in order to fail, then check the fixture can produce it.**
