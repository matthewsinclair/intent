---
verblock: "09 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-02
title: "Init hardening: provisioning guards and FP guidance requirement"
scope: Small
status: Done
---

# WP-02: Init hardening: provisioning guards and FP guidance requirement

## Objective

Harden `in-tca-init` so operators cannot deviate into the Lamplight failure modes even when eager. Documentation is necessary but insufficient -- the init path needs mechanical guards that fail-fast on the "TCA as WP inside another ST" and "overwrite in-progress audit" antipatterns.

Addresses Lamplight feedback items P0.1b (symmetric provisioning guard, caught during Plan agent review) and P1 (False Positive Guidance as a REQUIRED design.md section). Also updates the skill's step 4 to use `"TCA: <project and scope>"` title format, replacing the generic `"Total Codebase Audit"` that would collide across multiple audits.

## Deliverables

1. Top-of-file invariant callout in `in-tca-init/SKILL.md` stating that a TCA is always its own dedicated ST and referencing `total-codebase-audit.md` section 0.0.
2. Updated step 4 of the skill to use `intent st new "TCA: <project and scope>" --start` as the canonical command, with guidance about informative titles.
3. Expanded step 7 (Write design.md) adding **False Positive Guidance (REQUIRED)** as a 5th bullet plus an example subsection showing the R8/R9 format from the Lamplight feedback report. Skill must refuse to proceed to Phase 1 if this section is missing or placeholder.
4. Added flat-WP reinforcement bullet and FP-Guidance load-bearing note to the Important Notes section.
5. Two provisioning guards in `scripts/tca-init.sh`, running after arg parsing:
   - Guard 1: refuse if the target path contains `/intent/st/ST*/WP/*` (nested-WP antipattern), ordered before the existence check so it fires on non-existent paths too.
   - Guard 2: refuse if `$ST_DIR/WP` already contains populated `socrates.md` files (overwrite protection), ordered after the existence check.

## Acceptance Criteria

- [x] Top-of-file invariant callout is present in `in-tca-init/SKILL.md`
- [x] Step 4 uses `"TCA: <project and scope>"` title format
- [x] Step 7 lists False Positive Guidance as REQUIRED with an example
- [x] Important Notes section has flat-WP and FP-Guidance bullets
- [x] `tca-init.sh` guard 1 fires on `intent/st/ST0030/WP/01` (verified: exits 1 with explanatory message)
- [x] `tca-init.sh` guard 2 fires on a sandbox ST with populated socrates.md (verified: exits 1 with "already contains" message)
- [x] `tca-init.sh` happy path still works on a fresh sandbox ST (verified: creates flat WP/01..WP/NN)
- [x] Existence check still fires for legitimate-shaped but non-existent paths (verified)
- [x] `bash -n` passes on `tca-init.sh`
- [x] No em dashes in any skill file (house style for skill list display)
- [x] No Claude attribution in commit message

## Dependencies

Depends on WP-01: the invariant callout and skill-level comments reference `total-codebase-audit.md` section 0.0 which WP-01 created. WP-02 cannot land before WP-01.
