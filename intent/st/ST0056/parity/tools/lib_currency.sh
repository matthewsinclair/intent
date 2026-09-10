#!/bin/bash
# lib_currency.sh -- can a given artefact see a recent change, by CONTENT?
#
# SOURCED, NOT EXECUTED. It defines functions and forms no verdict, so it ships
# 644 like `lib_staged.sh`, `lib_corpus.sh`, `lib_mdfmt.sh`, `lib_surface.sh`
# and `lib_classify.sh`.
#
# ==========================================================================
# WHY THIS EXISTS
# ==========================================================================
#
# `AC-00.14`: an instrument that cannot see a change under test emits a
# confident, well-formed, internally consistent verdict about a world that has
# moved, and blames the estate. **27 instruments in this directory drive a
# binary. ONE of them demonstrates it can see a change.** The other 26 have the
# exposure and no way to ask about it that does not cost them a census.
#
# **`instrument_currency_check.sh` IS A WHOLE-ESTATE CENSUS AND THAT IS THE
# WRONG SHAPE FOR A CONSUMER.** An instrument that called it would enumerate
# every binary in the tree to learn about the ONE it is about to drive. So the
# probe logic lives here, the census sources it, and a consumer asks a single
# question about a single artefact.
#
# **ONE HOME, WHICH IS THE WHOLE POINT.** Twenty-six instruments each rolling
# their own currency demonstration would be twenty-six homes for one concern --
# the thing `IN-AG-HIGHLANDER-001` forbids, arriving through diligence rather
# than through carelessness.
#
# ==========================================================================
# THREE STATES, AND THE THIRD IS NOT A REFINEMENT
# ==========================================================================
#
#   CURRENT      carries a probe literal.
#   BLIND        links the probe's source file and carries no probe literal.
#   UNREACHABLE  does not link the probe's source at all.
#
# **UNREACHABLE EXISTS BECAUSE `intentd` DOES NOT DEPEND ON `intent-cli`.** A
# probe derived from a literal in `intent-cli` cannot be carried by `intentd`
# however current it is, and reporting that as BLIND is a confident verdict
# about a question the probe cannot ask of that artefact -- `AC-00.14`'s own
# class, committed by the instrument that measures it. Measured 2026-09-09:
# both `intentd` binaries reported BLIND while freshly built from the same
# commit as the `intent` binaries that reported CURRENT.
#
# The discriminator is DERIVED, not declared: rustc embeds each source file's
# path for panic locations, so an artefact carries `crates/<crate>/src/<f>.rs`
# iff that file is linked into it. A hand-listed exclusion would rot exactly
# like a hardcoded probe.
#
# ==========================================================================
# WHAT THIS DOES NOT ANSWER
# ==========================================================================
#
# **WEAKER THAN IT LOOKS.** The probe shows an artefact can see ONE commit, not
# the commit a given instrument's own claim is about. A binary current for this
# probe can still be blind to the specific change being reported on.
#
# It says nothing about whether an instrument's verdict is CORRECT. Correctness
# and validity are two questions and this is the second one only.
#
# It covers COMPILED artefacts. A generated table, a canon extract, a fixture
# tree or a vendored manifest can be exactly as stale and `strings` cannot be
# pointed at any of them.

# The probe: the newest commit yielding a literal that still ships. Sets
# CURRENCY_PROBE_COMMIT, CURRENCY_PROBE_LITERALS and CURRENCY_PROBE_SRCS.
#
# A literal sitting after its file's first `#[cfg(test)]` is REJECTED -- it
# would be a probe for code that does not ship, which tests nothing.
currency_derive_probe() {
  local root="$1" crates="$2" depth="${3:-40}"
  CURRENCY_PROBE_COMMIT=""
  CURRENCY_PROBE_LITERALS=()
  CURRENCY_PROBE_SRCS=()
  local commit lit hit file rest line tmod
  while IFS= read -r commit; do
    [ -n "$commit" ] || continue
    local cands=() srcs=()
    while IFS= read -r lit; do
      [ -n "$lit" ] || continue
      hit="$(grep -rn -F -- "$lit" "$crates"/*/src/*.rs 2>/dev/null | head -1)"
      [ -n "$hit" ] || continue
      file="${hit%%:*}"; rest="${hit#*:}"; line="${rest%%:*}"
      tmod="$(grep -n '#\[cfg(test)\]' "$file" 2>/dev/null | head -1 | cut -d: -f1)"
      if [ -n "$tmod" ] && [ "$line" -ge "$tmod" ]; then continue; fi
      cands+=("$lit")
      srcs+=("crates/${file#"$crates"/}")
    done < <(git -C "$root" show "$commit" -- 'native/rust/crates/*/src/*.rs' 2>/dev/null \
      | grep -E '^\+' | grep -oE '"[a-z][a-z0-9 :;,._-]{24,70}"' | tr -d '"' | sort -u)
    if [ "${#cands[@]}" -gt 0 ]; then
      CURRENCY_PROBE_COMMIT="$commit"
      CURRENCY_PROBE_LITERALS=("${cands[@]}")
      CURRENCY_PROBE_SRCS=("${srcs[@]}")
      return 0
    fi
  done < <(git -C "$root" log -n "$depth" --format=%h -- 'native/rust/crates/*/src/*.rs' 2>/dev/null)
  return 1
}

# One artefact, one word: CURRENT | BLIND | UNREACHABLE.
currency_state() {
  local a="$1" p f hits=0 linked=0 c
  for p in "${CURRENCY_PROBE_LITERALS[@]}"; do
    c="$(strings "$a" 2>/dev/null | grep -cF -- "$p" || true)"
    hits=$((hits + ${c:-0}))
  done
  [ "$hits" -gt 0 ] && { echo CURRENT; return 0; }
  for f in "${CURRENCY_PROBE_SRCS[@]}"; do
    c="$(strings "$a" 2>/dev/null | grep -cF -- "$f" || true)"
    [ "${c:-0}" -gt 0 ] && linked=1
  done
  [ "$linked" -eq 1 ] && { echo BLIND; return 0; }
  echo UNREACHABLE
}

# THE CONSUMER'S ENTRY POINT: assert the artefact I am about to drive is not
# blind. Prints one line and returns 0 unless the artefact is demonstrably
# stale.
#
# **UNREACHABLE RETURNS 0 DELIBERATELY.** It is not a currency finding -- the
# probe cannot speak about that artefact either way -- and turning "I cannot
# tell" into a refusal would block instruments over a question nobody asked.
# It prints, because a limit not in the output is not a limit the reader has.
#
# **AND A DERIVATION FAILURE IS rc=2, NOT rc=0.** If the probe cannot be
# derived, this establishes nothing, and a silent pass would be exactly the
# defect `AC-00.14` names.
currency_require() {
  local a="$1" root="${2:-}" crates="${3:-}" state
  [ -f "$a" ] || { echo "currency: no artefact at $a -- nothing to ask about" >&2; return 2; }
  if [ -z "${CURRENCY_PROBE_COMMIT:-}" ]; then
    currency_derive_probe "$root" "$crates" || {
      echo "currency: no shipping literal in the last commits -- the probe could not be derived, so this run establishes nothing about $a" >&2
      return 2
    }
  fi
  state="$(currency_state "$a")"
  case "$state" in
    CURRENT) echo "currency: ok -- $(basename "$a") carries $CURRENCY_PROBE_COMMIT"; return 0 ;;
    UNREACHABLE)
      echo "currency: NOT ASKED of $(basename "$a") -- it does not link ${CURRENCY_PROBE_SRCS[0]-the probe source}, so no probe derived from that file can speak about it either way"
      return 0 ;;
    BLIND)
      echo "currency: BLIND -- $(basename "$a") cannot see $CURRENCY_PROBE_COMMIT, so any verdict below describes a world that has moved" >&2
      return 1 ;;
  esac
}
