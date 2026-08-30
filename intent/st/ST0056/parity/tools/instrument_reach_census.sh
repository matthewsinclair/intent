#!/usr/bin/env bash
#
# instrument_reach_census.sh -- AT-00.20, covering AC-00.16.
#
# **EVERY INSTRUMENT HAS A DECLARED REACH, AND NOTHING IN THE ESTATE OWNS THE
# GAPS BETWEEN THEM.** Three instances by three nodes on 2026-08-30, each an
# instrument SOUND within its own reach with a shipped surface in the gap.
#
# THIS SCRIPT IS RED AND SAYS SO. It measures ONE necessary condition -- that an
# instrument declares what it does not cover -- and that condition is not the
# criterion. The criterion asks that something own the UNION. Declaring reach is
# what makes the union computable; it does not compute it.
#
# **AND THIS INSTRUMENT IS SUBJECT TO THE CLASS IT MEASURES, WHICH IS WHY ITS
# OWN REACH IS PRINTED RATHER THAN DESCRIBED.** It reads Rust test files. It
# does NOT read the BATS suite, the shell instruments beside it in this
# directory, the pre-commit guards, or the critic rule library -- and the
# vc instance of this very class was a shipped SHELL surface sitting outside a
# Rust scanner's reach. An instrument that measured "declares its reach" while
# hiding its own would be the fourth instance.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
CRATES="${ROOT}/native/rust/crates"

[ -d "$CRATES" ] || { echo "error: no crates dir at ${CRATES}" >&2; exit 2; }

examined=0
declaring=0
silent=()

while IFS= read -r f; do
  examined=$((examined + 1))
  # A declared reach is a statement about what the file does NOT cover.
  if grep -qiE 'does not (read|cover|scan|reach|see)|out of scope|NOT in scope|says nothing about' "$f"; then
    declaring=$((declaring + 1))
  else
    silent+=("${f#"$ROOT"/}")
  fi
done < <(find "$CRATES" -path '*/tests/*.rs' -not -path '*/target/*' | sort)

# THE DENOMINATOR IS THE POPULATION EXAMINED, printed so a reader never adds up
# (AC-00.11, AC-00.12). A partition that does not close is a defect here.
echo "AT-00.20: ${declaring} of ${examined} Rust integration-test file(s) declare what they do NOT cover"
echo "          partition: ${declaring} declaring + ${#silent[@]} silent = $((declaring + ${#silent[@]})) of ${examined}"
[ $((declaring + ${#silent[@]})) -eq "$examined" ] || { echo "error: partition does not close" >&2; exit 2; }

echo ""
echo "REACH OF THIS INSTRUMENT, stated because it is subject to its own class:"
echo "  COVERS      native/rust/crates/*/tests/*.rs"
echo "  DOES NOT    the BATS suite, the shell instruments in this directory,"
echo "              lib/templates/hooks/*, the critic rule library, colocated"
echo "              #[cfg(test)] modules in src/, and every non-Rust surface."
echo "  UNOWNED     the UNION of all of the above. Nothing computes it, which"
echo "              is the criterion this row does not yet satisfy."

echo ""
echo "RED: declaring a reach is necessary and not sufficient. AC-00.16 asks that"
echo "something own the union of every instrument's reach; this measures one"
echo "precondition over one language's integration tests and no more."
exit 1
