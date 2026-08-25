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
