# inbox: dc -> hv

## (2026-08-25 08:25Z) FYI only -- no response needed.

ANNOUNCE -- I am touching the guards that gate YOUR commit, in BOTH trees.

vc routed an hv-ruled fix: `whiteboard-clock-guard.sh:246`, `whiteboard-header-guard.sh:201` and `canon-ignore-guard.sh:133` all use `printf ... | grep -q` as their "did THIS COMMIT add it" filter. Under `set -uo pipefail` grep exits on first match, printf takes SIGPIPE, **pipefail promotes 141 to the pipeline status and the test reads FALSE** -- so a lost race classifies a real violation as inherited breakage and passes it. **I reproduced it independently at worst shape: 200/200 lost under bash 5.3.15 and 100/100 under 3.2.57.** Remedy is a herestring: not a pipeline, so pipefail has nothing to corrupt.

**WHAT THIS MEANS FOR YOU: if a commit of yours is refused in the next while with a clock or header finding, it is probably REAL and was previously being dropped.** Do not assume I broke the guard -- but tell me if the refusal looks wrong and I will drive it.

Bodies are byte-identical across `Intent` and `Intentv2` today and MUST STAY SO (`shipped_surface_drift.bats` asserts both-or-declared), so I land both together.

**AND THE POPULATION IS WIDER THAN THE THREE SHIPPED SITES, WHICH I AM REPORTING RATHER THAN SWEEPING:** 13 files carry the idiom across 27 sites, **11 of 13 armed by pipefail**. The other 24 sites are in `intent/st/*/parity/tools/` -- our own instruments, not shipped. Two you may care about: `runner_roster_check.sh:291` (ARMED, and it is one of the 12 rostered precommit guards) and `provenance_fields_check.sh` (7 sites, ARMED). **Not touching those in this change** -- widening a fix silently is the defect class, not the fix.

FYI only -- no response needed.

## (2026-08-21 14:30Z) FYI only -- no response needed.

**hv HAS RULED `bin/` IS dc's LANE, AND I AM TOUCHING `bin/.devbin/cmd/` TODAY.** Attributing, not asserting: hv ruled it in the live channel just now, answering a question I put with options. Announcing before I touch it because `bin/` is the one genuine cc/dc collision on the roster.

**Two sites:** `cmd/precommit:141` (the one-word `intentdb` -> `the SQLite db` noun fix vc routed me) and `cmd/hooks` (a cwd-resolution hazard, below). Per-file pathspec on commit, never a directory one -- your 13:00Z note.

**hv ALSO RELEASED ALL THREE HELD ITEMS:** `tests/lib/test_helper.bash:93`, and the two roster admissions (`canon_commit_check.sh`, `thread_view_skew_check.sh`). **The first of those moves ST0057 AC-03.6**, so two of the five outstanding gate rows are reachable by me today.

**cc: YOUR FLAG ON ARM C IS ANSWERED, AND THE ANSWER IS THAT IT SURVIVED THE SPLIT.** Driven end to end in a throwaway clone at `510d4b10`: wired properly, a stamp with **no trailing Z** -- clock-guard check B, syntactic, no tolerance -- **committed at rc=0**. The dispatcher is gitignored at `.gitignore:158`, so **no clone can ever receive it**, and the chain block's `[ -x ]` has no `else`. The four shipped guards and the critic ran nowhere while **ten parity guards printed ~60 lines of confident output**. **The hazard is not silence -- it is reassuring noise from a different roster.**

**AND A CORRECTION TO THREE CLAIMS I NEARLY ESCALATED, ALL THREE MINE.** `int hooks` resolves its target repo from **the binary's location, not the cwd**. The PATH `int` is `Intent/bin/int`, so standing in any other checkout it silently answers about **Intent**. That produced three false findings before I caught it. Driven both ways:

```
cd Intentv2 && int hooks        -> hooks in .../Intent/.githooks      (wrong tree)
cd Intentv2 && ./bin/int hooks  -> hooks in .../Intentv2/.githooks
                                   gate ABSENT -> pre-commit.intent is not in ...
```

**`int hooks` is CORRECT and already says the right thing** -- it reports `gate ABSENT`, names the missing file, and says the chain skips silently. **So vc's remedy form 2 is already built and AC-01.5 is form 1 only.** It is also a live trap for restart step 0: running `int hooks` in a fresh clone gets you Intent's answer about a different tree.

## (2026-09-02 16:32Z)

**FIVE ITEMS FOR YOUR REVIEW, ROUTED DIRECT AT YOUR INSTRUCTION.** My board carries these too; this is the durable copy, because a live channel is not a durable one and a write with no named reader is a queue rather than a delivery.

**1. `ST0057 AT-07.5` NEEDS A STOP-THE-DAEMON WINDOW. It is the only open item that is mine to want and yours to grant.** The thread qualifier is load-bearing: two threads carry a row with that id, and ST0056's is a green `agents_sync_parity` row about heading structure that flatly contradicts the item. The real row is `intent/st/ST0057/parity/tools/no_daemon_required.sh`. Arm B (structural) passes; arm A (behavioural) REFUSES rather than fails while an intentd is up, so **the green rests on a moment nobody can reproduce on demand.** Stopping the daemon costs every live session, which is why no node may take it.

**2. THE v3.0.0 TO v3.0.1 STORE MIGRATION IS 13 TO 17 AND IRREVERSIBLE, with no path back to the tag.** Endpoints measured, not inherited: `80d8b2ca` IS `v3.0.0` and reads `SCHEMA_VERSION: i32 = 13`; the tree reads 17. **The rung count between them is NOT measured and is NOT claimed.** Local exposure is nil and that is deliberately not the finding -- it is an obligation to people who are not on this box. Release note, refusal message, or backup step.

**3. `0207` (at green/red/na --note replaces wholesale), plus the remedies for `0203` and `0208`** -- all yours, unchanged. AC-04.6 sits behind `0207`; AC-07.7 needs a published artefact.

**4. THE `overhead` METER REPORTS A FIGURE ABOUT THE WRONG SUBJECT** (W46), with devbin-vc. Filed rather than escalated, and you have not weighed in; it is the kind of number that gets quoted at somebody.

**5. FLIP-THEN-BURN, and the ORDER is the whole point.** vc decomposed it at `821e7268` after I relayed the compound to you unexamined. `flip` = rebind the default `INTENT_BIN`, which today points at the v2 shell script while two other bindings are live; `burn` = re-run `burn.sh`, which has hung for 3.5 hours once. **`burn.sh` classifies RELATIVE TO the binding, so a burn taken before the flip is wall time paid twice.** Yours on wall time alone.

**CLOSED, RECORDED SO NOBODY RE-OPENS THEM.** The rebuild window is discharged: the pair was rebuilt at `4be902e1`, `currency ok`, zero non-test files under `native/rust` changed since. **I reported it to you as outstanding from a stale line on my own board**, and vc was carrying the same claim. The ruled pipefail sweep landed at `7f1bfb53` and is superseded by the gated guard at `70e2e48c`, which found 13 sites the sweep could not see.

Items 1 to 5 are the review. The closed pair is FYI only -- no response needed.

## (2026-09-02 16:39Z) Re: (2026-09-02 16:32Z) FYI only -- no response needed.

**AMENDING ITEM 5 (flip-then-burn) BECAUSE vc DROVE IT AFTER I ROUTED IT** (`7d37a063`, their 8ab). **NEITHER THE FLIP NOR THE BURN NEEDS A BUILD**: `burn.sh` drives bats against `INTENT_BIN`, the flip points that at the v3 release binary, and that binary is current at `4be902e1`. **11 paths under `native/rust` are dirty (ic building `/settings`) and that forbids a REBUILD while a node is live -- it does not stand in the way of flip or burn.** So the item is still yours on wall time, and it is NOT waiting on a clean tree. The order remains load-bearing.

## (2026-09-04 10:06Z)

**`AC-00.3`: FOUR CANON-NAMED VERBS ANSWER rc=2. WIRE THEM OR DE-CANON THEM?** It blocks `ST0058`, `ST0058` binds the cut, and the cut is now everything. **Routed through vc, who surfaces it; the recommendation is mine.**

**THE MENU YOU NARROWED TO TWO IS RIGHT FOR ONE OF THE FOUR AND WRONG FOR THE OTHER THREE, AND THAT IS THE ONLY REASON THIS IS LONGER THAN A YES/NO.** I drove the implementations this morning rather than the surface, and the four split into artefacts whose economics are opposite. One answer applied to all four either builds a writer for a file nothing reads, or deletes the documentation of a capability that is running in my session as I write this.

**DRIVEN, NOT TRANSCRIBED** -- delivered binary `3.0.0 (c5db8b8a)`, `currency ok`, so the pair describes this tree.

- `claude rules index`, `claude rules validate`, `claude subagents list`, `claude subagents status` -- all **rc=2**, all print the unwired phrase.
- **CONTROLS (vc's, and they discriminate):** `intent claude nosuchverb` is **rc=1 unrecognized subcommand**; `intent claude rules list --lang agnostic` is **rc=0**. So rc=2 separates DECLARED-BUT-UNIMPLEMENTED from NONEXISTENT -- the finding is not a bare non-zero.
- **Canon names nine spellings**: `usage-rules.md:111-117` (seven `subagents`), `:140,141` (`rules validate`, `rules index`), `:212`, `:313`; `AGENTS.md:113,144`.
- `fn claude` (`render.rs:6945`) arms are `hook rules skills upgrade start ws` -- **no `subagents` arm at all**. `fn rules` (`render.rs:7498`) arms are `list show` -- no `index`, no `validate`.
- The dispatch table declares `claude subagents` as a **FAMILY WITH NO LEAVES**, which is exactly why the unwired message names the family and not the leaf.

**WHAT CHANGES THE ANSWER, AND IT IS THE PART I HAD WRONG YESTERDAY.**

**`subagents`: THE v3 IMPLEMENTATION ALREADY EXISTS.** `intentsvcs::payload::Kind` carries an `Agents` arm in *every* method -- `canon_subdir` -> `subagents`, `marker` -> `agent.md`, `target_subdir` -> `agents`, `manifest_relative` -> `subagents/installed-subagents.v3.json`, `shape` -> `SingleFile`, `scope_token` -- including the one structural asymmetry (a skill is a directory landing as a directory; a subagent is a directory landing as one renamed file). `intentsvcs/tests/subagents_payload.rs` drives it. And `fn skills` is already written as `payload_lib(Kind::Skills)`. **So wiring is parameterising an arm that exists, not building a capability.** The capability is also in daily use: eight subagents are installed in `~/.claude/agents/` and this session is dispatching `critic-*` through them.

**`rules index`: THE FILE IT WOULD REGENERATE HAS NO v3 READER.** `intent/plugins/claude/rules/index.json` is checked in, and **nothing in the Rust tree reads it** -- zero hits for `index.json`, positive-controlled at thirty hits for `dispatch-table.json` on the same instrument, so the zero is the corpus and not the grep. The v3 `Library` reads the rule files directly (`rules.rs:225,276`).

**`rules validate`: no v3 implementation at all.** **`subagents status`: no counterpart anywhere** -- `skills status` does not exist either.

**OPTIONS, PER ARTEFACT.**

1. **`subagents` (six spellings)** -- **(A1) wire it**, mirroring `fn skills` with `Kind::Agents`; **(A2) de-canon it**, strike the lines.
2. **`subagents status` (the seventh)** -- **(B1) drop it from canon** while wiring the other six; **(B2) build it for both kinds.**
3. **`rules index`** -- **(C1) de-canon it**; **(C2) wire it.**
4. **`rules validate`** -- **(D1) wire it**; **(D2) de-canon it.**

**MY RECOMMENDATION: A1, B1, C1, D1.**

- **Wire `subagents`.** The expensive half is built and tested; the missing half is a render arm. **De-canoning is the opposite of the `ext` case rather than an application of it** -- you de-canoned a verb with no capability behind it, and this is a working capability whose documentation would be the thing deleted.
- **Drop `status` from canon.** It has no counterpart for skills, and building it for one kind and not the other manufactures the asymmetry.
- **De-canon `rules index`. THIS is the true `ext` member of the set** -- wiring it means building a writer for a file that has no reader.
- **Wire `rules validate`** -- and this is the weakest of my four; the capability is real (`intent/docs/rules.md` specifies the schema) but it is the only one of the four with nothing already built, so if you want to cut one, cut this.

**SIZES, AND TWO ARE GUESSES.** `subagents` wiring **S** (guess -- the payload side is read, no code written). `status` drop **XS**. `rules index` de-canon **XS**. `rules validate` **M and a guess**.

**NOT MEASURED, AND I AM NOT CLAIMING IT:** whether the eight installed subagents got there via v2, via `claude upgrade`, or by hand. The capability's existence is measured; the installation route is not.

## (2026-09-05 10:50Z) FYI only -- no response needed.

**ANNOUNCE -- A BOUNDED RELEASE BUILD IS RUNNING ON YOUR BOX AND YOU CAN VETO IT.**

`cargo build --release -j 4` at tag `v3.0.0`, in a DETACHED WORKTREE with a PRIVATE `CARGO_TARGET_DIR` under my scratchpad. **It does not touch the shared pair, `native/rust/target/release`, the delivered symlinks or the daemon.** vc authorised it and said explicitly that this is theirs to authorise because `0196` is about the SHARED pair and a private target dir is not the shared pair -- no rebuild window is taken. You are being told because it is your machine and it is a visible resource event, not because vc needed your word. **A veto from you supersedes vc and I will stop.**

**THE BOUND CAME FROM A MEASUREMENT.** vc read load average 14.45 on 16 cores; I re-read it at 12.66 with no cargo or rustc running, so four live sessions are already at roughly four fifths of the core count. An UNBOUNDED build adds 16 rustc jobs on top of that, which is the thing that would make it a bad neighbour rather than the build itself. Parallelism does not change the artefact, so `-j 4` is the same binary later rather than a different binary. Disk is not a constraint.

**WHAT IT IS FOR.** `docs/known-defects.md` opens with its own rule -- every defect on it was run against v3.0.0 ITSELF, not against `main`. **No v3.0.0 binary exists on this machine**: both PATH symlinks point at the shared dev release, which names `92e4d914`, not the tag `80d8b2ca`. 27 `ST0068/AC-02.3` dispositions need a drive against the published build before a `stated` row can honestly be written. The 14 that needed no binary landed at `bbb9f8f9`.

**AND THE ITEM THAT IS STILL YOURS AND HAS NOT MOVED: this branch is many commits ahead of both `local` and `upstream` with nothing pushed.** vc is relaying it as the named reader; I am recording it here so it survives the session rather than living in a message.

## (2026-09-10 09:33Z)

**THREE DOORS THAT SHIP IN v3 ANSWER `not implemented yet`, AND I AM NOT WIRING ANY OF THEM WITHOUT A WORD FROM YOU, BECAUSE ALL THREE POINT AT SUBJECTS THIS ESTATE HAS SINCE RETIRED.** cc found them in WPs I claim and I verified each against `target/release` at `ec55b3ba` with both controls -- a known-wired door (`st list`, renders) and a known-unwired one (`config`, refuses) -- so the probe can tell the two apart.

  st bootstrap      WP-04   keep / corrected
  agents template   WP-07   keep / corrected
  claude prime      WP-07   keep / as-observed

**THE PATTERN IS THE FILING. EACH ROW'S DISPOSITION OR REMEDY WAS SETTLED BEFORE THE THING IT OPERATES ON WAS RETIRED, AND BUILDING TO THE ROW WOULD SHIP, IN A NEW DOOR, THE DEFECT THE NEIGHBOURING CODE WAS REWRITTEN TO AVOID.**

**(1) `agents template` -- your remedy is nine days older than the retirement of its subject.** You ruled it 2026-08-17 at `632d9d861`: `list` reports what each directory actually offers, in separate labelled groups, and `show` resolves either kind, so no name `list` prints is a name `show` refuses. The principle is untouched. The SUBJECT is not: the directory it enumerates, `intent/plugins/agents/templates/`, **has no reader anywhere in v3.** The only two mentions in the Rust tree are doc-comments explaining why v3 does not read it. `b60f9ebb5` (2026-08-26, nine days after your ruling) made `intent lang init` install nothing, and `rules.rs:67` states the reason in the words this filing would otherwise have to invent -- *enumerating the template directory would be a correct value about a subject that is no longer the question.* **Building your remedy today puts that exact sentence inside a door, one file from the comment warning against it.** Measured: eight directories, `elixir` alone carries an `AGENTS.md`, and `AGENTS.md` is now GENERATED rather than seeded (`init.rs`, `Generated` not `At`) -- so both halves of the door point at retired ground, not just the canon-pack half your remedy widened `show` to cover.

**(2) `st bootstrap` -- its deliverable list is part live and part ruled-against, and only measurement separates them.** Eight deliverables. **D2 (CLAUDE.md) is live** -- `init.rs:84`, `At("CLAUDE.md")` -- and that is the negative control that keeps this finding specific rather than a blanket claim. **D3 installs `intent/llm/MODULES.md`, which v3's `init` declares `NotByInit("a hand-maintained index of a tree the store already indexes")`** on your 2026-08-24 ruling. So D3 retrofits a file the estate decided not to create. **D8 is *MEMORY.md via `intent claude prime`*** -- it depends on the second unwired door in this filing. **D10 installs `intent/.config/learnings.md`, written by `intent learn`, which is itself unwired** (driven just now: `error: 'learn' is a known command that is not implemented yet`).

**(3) `claude prime` -- cc's finding and I am carrying it rather than restating it as mine.** `prime` consumes `learnings.md`; `learn` writes it by stamping `- $(date +%Y-%m-%d): <desc>`; your 2026-08-15 ruling leaves NO clock in the workspace, and `one_clock.rs` bans every route across `src/` AND `tests/` with an empty exempt list. **prime is learn's stated consumer, so the pair carries one design decision between them and it lands across two work packages.**

**WHAT I AM ASKING FOR, AND IT IS ONE RULING RATHER THAN THREE.** For each door: is it still a `keep`? A `keep` whose subject is retired is a disposition that outlived its premise, and the honest outcomes are to retire the door, to re-point it at whatever replaced the subject, or to say the subject is not as retired as I have measured. **I can build any of the three once you say which.** I am not choosing between those myself -- picking one would be minting a disposition, and dispositions are yours.

**AND THE POPULATION IS WIDER THAN MY THREE.** vc is already bringing you the `as-observed` rows measured before 2026-08-15. **This is the same defect one field over: `keep` and `corrected` rows whose remedy or deliverable list predates a retirement.** Worth ruling as a class, because the three I hit are the three cc's probe could reach -- **57 of 127 paths were NOT PROBED and 7 are inconclusive**, so my three are a floor and not a count. I am deliberately not proposing a sweep sized to that probe.

---

**SEPARATELY, AND NOT BLOCKED ON ANY OF THE ABOVE -- A DEVX GAP I WANT TO BUILD, ROUTED HERE ON vc's INSTRUCTION.**

**NO TOOL ON THIS MACHINE REPORTS WHAT IS RUNNING OR WHAT THE STORE WEIGHS, AND THAT COST A DAY.** Sixty-five CPU-hours, a 559MB WAL against a 21MB database, load average 417, and four nodes' writes going missing -- and the way anybody found out was a node noticing the machine felt slow. `int local status` already names `target/debug/` in its own scope block as *the one the test harness spawns*, and then measures nothing about it. **A door that names the artefact and does not measure it is the gate-figure defect in a different costume.**

It re-ran itself while I was writing this: vc watched the daemon population go 0 -> 2 -> 13 in twenty minutes with four holding the live store, and ic caught `0216` eating a real write at four holders. **Not one tool said anything, again, today.**

Two requirements go INTO the mechanism rather than into a note, both bought with real errors this morning:

- **COUNT BY EXECUTABLE, NEVER BY COMMAND-LINE TEXT.** `ps -axo command= | grep -c 'target/debug/intentd'` returns 2 with ZERO daemons running, because this harness wraps commands in an eval string and the pattern appears in a process's command line by construction. The `[i]ntentd` bracket trick stops the grep matching itself and does nothing about the wrapper. **I gave you the opposite advice yesterday -- prefer `ps` over `pgrep` -- and it is withdrawn; the reason I gave was false.** The discriminator is `ps -axo pid=,command= | awk '$2 ~ /(^|\/)intentd$/'`, matching argv[0], or asking the socket.
- **REPORT STORE HOLDERS SEPARATELY FROM PROCESS COUNT.** Those are the ones that cost writes, and the two numbers came apart today: 13 processes, 4 holders.

Size **M**. Falsifier is clean and does not need your machine: spawn N in a scratch project, the door says N; kill them, it says 0. vc has agreed the shape and added the executable requirement. **Your word needed only on whether it belongs in this cut or after it** -- it is devx, it is unblocked, and it is the only new work my column has that nobody is waiting on.

-- dc

## (2026-09-10 09:51Z)

**ADDENDUM TO MY FILING ABOVE. THE POPULATION IS FOUR SUITES WIDER THAN THREE DOORS, AND IT IS ONE SHAPE RATHER THAN TWO -- vc's instruction, and I agree with it.**

Fixing AC-12.1 this morning took me one command away from deleting four bats suites, and checking what referenced them is the only thing that stopped me. **All four are dispositioned `keep` in `intent/st/ST0056/parity/register.md`, with the full burn ALREADY RECORDED:**

    agent_commands.bats    50 tests   50/50   keep   full burn
    skills_commands.bats   39 tests   39/39   keep   full burn
    rule_index.bats         8 tests    8/8    keep   full burn
    claude_prime.bats       2 tests    2/2    keep   full burn

They are cited by `burn-baseline.tsv`, `lib_classify.sh` and four TAP baselines. **Deleting them is a DISPOSITION CHANGE and a re-base of the parity population, and it lands on `AC-06.1` -- the run only you can make.** I would have re-based your baseline while fixing a red workflow, in the same commit, with nothing saying so. **That is the worst shape a correct-looking change can take**, and the only reason it is not in the tree is that I looked before deleting.

**AND IT IS THE SAME CLASS AS THE THREE DOORS ABOVE, ONE LAYER DOWN: a disposition that outlived its subject.** Four suites dispositioned `keep`, testing five doors pruned at `125f601d8` eleven days ago. So the ask above is unchanged in kind and wider in scope -- **one ruling over three doors AND four suites**, rather than answering the doors now and the suites in a fortnight. `d8a8c070` fixed the half that is unambiguously mine: v2's dispatcher no longer advertises what v2 cannot serve. **It does NOT turn CI green and does not claim to** -- the suites now meet `Unknown claude subcommand` instead of `Plugin command not found`.

**SHARPENING THE `int local status` PROPOSAL, ON A CORRECTION vc MADE AGAINST THEMSELVES.** vc told three nodes there is NO spawn site for `intentd` in shipped code. **There is one**: `render.rs:6636`, where `intent daemon start` detaches a daemon into its own process group via `process_group(0)`. That is CORRECT for a user verb -- your daemon should outlive your shell -- and it means **the estate has TWO daemon-creation mechanisms with different lifetimes, one deliberately detached, and no tool distinguishes them or counts either.** It was invisible to a `Command::new` grep because `resolve_intentd()` computes the binary name rather than spelling it. **Nobody could have caught this by reading; it took the machine falling over.** That is the argument for the tool, and it is stronger than the one I filed with.

**AND A MACHINE-COST QUESTION I AM FILING AS OPEN RATHER THAN AS A FINDING, BECAUSE SETTLING IT NEEDS THE ONE THING NOBODY MAY DO RIGHT NOW.** A tool reporting what a test run costs this machine probably has to count more than processes. Measured, statically, with zero daemons and nothing building:

- **247,053 files under `native/rust/target`**, and **no `.metadata_never_index`** at the target, crate or repo root, with Spotlight indexing enabled on the volume.
- **A load contributor nobody in this estate has named: Dropbox.** `fileproviderd` at 59.7% and `Dropbox` at 18.1% -- and the `local` git remote is `~/Dropbox/Repositories/Devel/Intent`, where 255 commits landed this morning. The repo itself is NOT inside Dropbox; the remote is.
- **I CANNOT GIVE YOU A SPOTLIGHT ITEM COUNT AND WILL NOT PICK ONE.** Three query forms returned three answers -- `mdfind '*'` gave 92, `kMDItemFSName == '*'` gave 0, `find` on disk gave 247,053. **An instrument disagreeing with itself three ways adjudicates nothing**, so vc's 43,898 is neither confirmed nor refuted by me.
- **vc read `syspolicyd` at 69%; I read it at 2.8%.** Different samples minutes apart, and Gatekeeper assessment is bursty by nature. **Not a refutation and I am not offering it as one.**

**THE CAUSAL LINK IS THE PART THAT IS MISSING, AND IT IS MISSING BY CONSTRUCTION: confirming that a workspace run drives this cost means RUNNING A SUITE, which is exactly what is forbidden while the machine recovers.** So this is an open question with its own blocker stated, not a finding dressed as one.

-- dc
