#!/usr/bin/env bash
# skills-declobber.sh [--repair] -- ~/.claude/skills is SHARED by every project, and hop 1 (the frozen
# v2 tool's `upgrade`, run on any project below 2.19.0) reaches propagate_canon_skills unconditionally
# and writes v2's canon over it (devbin/vc measured three skills clobbered at 13:33Z on 2026-08-26).
# A clobber is decided EXACTLY: installed SKILL.md bytes == the v2 source's AND != v3 canon's.
# A skill matching NEITHER tree is a local edit or an orphan: HELD, named, never forced.
# Report always (rc 0 clean / 1 clobbered-not-repaired / 2 refused / 3 repair did not land);
# --repair restores only when the held set is EMPTY, so a forced sync can discard nothing real.
set -uo pipefail
SK=${VC_SKILLS_DIR:-$HOME/.claude/skills}
V2=${VC_V2_SKILLS:-$HOME/Devel/prj/Intentv2/intent/plugins/claude/skills}
V3=${VC_V3_SKILLS:-$HOME/Devel/prj/Intent/intent/plugins/claude/skills}
# REFUSE RATHER THAN DEFAULT. This used to fall back to `bin/intent3`, so a
# caller that forgot to pass one silently got whatever that wrapper resolved to
# -- which is how a retired path stays in service. `lamplight-triage.sh` and
# `sweep-default.sh` already use this shape; cc found this one still defaulting.
I=${VC_INTENT:?set VC_INTENT to the intent binary to use -- this script refuses to pick one for you, because a default is how a retired path stays in service (cc, 2026-08-27)}
L=${VC_SCRATCH:-/tmp/vc-scratch}; mkdir -p "$L"
REPAIR=0; [ "${1:-}" = --repair ] && REPAIR=1
sha() { shasum -a 256 "$1" | cut -c1-64; }
clob=(); hold=(); ok=0
for d in "$SK"/*/; do s=$(basename "$d"); f="$d/SKILL.md"; [ -f "$f" ] || continue
  h=$(sha "$f"); h2=""; h3=""
  [ -f "$V2/$s/SKILL.md" ] && h2=$(sha "$V2/$s/SKILL.md")
  [ -f "$V3/$s/SKILL.md" ] && h3=$(sha "$V3/$s/SKILL.md")
  if [ -n "$h3" ] && [ "$h" = "$h3" ]; then ok=$((ok+1))
  elif [ -n "$h2" ] && [ "$h" = "$h2" ]; then clob+=("$s")
  else hold+=("$s"); fi
done
echo "skills: $((ok+${#clob[@]}+${#hold[@]})) installed; $ok == v3 canon; ${#clob[@]} CLOBBERED (== v2 source): ${clob[*]:-none}; ${#hold[@]} matching NEITHER tree (HELD): ${hold[*]:-none}"
[ ${#clob[@]} -eq 0 ] && exit 0
[ $REPAIR -eq 1 ] || { echo "clobbered and not repairing (no --repair)"; exit 1; }
[ ${#hold[@]} -eq 0 ] || { echo "REFUSING --repair: ${#hold[@]} skill(s) match neither tree and a forced sync would discard them"; exit 2; }
for s in "${clob[@]}"; do "$I" claude skills sync --force "$s" > "$L/declobber.$s.out" 2>&1; rc=$?
  h=$(sha "$SK/$s/SKILL.md"); h3=$(sha "$V3/$s/SKILL.md")
  if [ "$h" = "$h3" ]; then echo "repaired: $s (sync rc=$rc; installed == v3 canon)"; else echo "REPAIR DID NOT LAND: $s (sync rc=$rc; installed != v3 canon)"; tail -3 "$L/declobber.$s.out"; exit 3; fi
done
exit 0
