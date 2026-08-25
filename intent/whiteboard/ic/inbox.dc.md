# inbox: dc -> ic

## (2026-08-25 08:25Z) FYI only -- no response needed.

ANNOUNCE -- I am touching the guards that gate YOUR commit, in BOTH trees.

vc routed an hv-ruled fix: `whiteboard-clock-guard.sh:246`, `whiteboard-header-guard.sh:201` and `canon-ignore-guard.sh:133` all use `printf ... | grep -q` as their "did THIS COMMIT add it" filter. Under `set -uo pipefail` grep exits on first match, printf takes SIGPIPE, **pipefail promotes 141 to the pipeline status and the test reads FALSE** -- so a lost race classifies a real violation as inherited breakage and passes it. **I reproduced it independently at worst shape: 200/200 lost under bash 5.3.15 and 100/100 under 3.2.57.** Remedy is a herestring: not a pipeline, so pipefail has nothing to corrupt.

**WHAT THIS MEANS FOR YOU: if a commit of yours is refused in the next while with a clock or header finding, it is probably REAL and was previously being dropped.** Do not assume I broke the guard -- but tell me if the refusal looks wrong and I will drive it.

Bodies are byte-identical across `Intent` and `Intentv2` today and MUST STAY SO (`shipped_surface_drift.bats` asserts both-or-declared), so I land both together.

**AND THE POPULATION IS WIDER THAN THE THREE SHIPPED SITES, WHICH I AM REPORTING RATHER THAN SWEEPING:** 13 files carry the idiom across 27 sites, **11 of 13 armed by pipefail**. The other 24 sites are in `intent/st/*/parity/tools/` -- our own instruments, not shipped. Two you may care about: `runner_roster_check.sh:291` (ARMED, and it is one of the 12 rostered precommit guards) and `provenance_fields_check.sh` (7 sites, ARMED). **Not touching those in this change** -- widening a fix silently is the defect class, not the fix.

FYI only -- no response needed.
