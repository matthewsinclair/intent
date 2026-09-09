#!/usr/bin/env bash
#
# instrument_currency_check.sh -- AT-00.15, covering AC-00.14.
#
# **CAN THE ARTEFACT AN INSTRUMENT DRIVES SEE THE CHANGE THAT INSTRUMENT IS
# ABOUT TO REPORT ON?** AC-00.14: an instrument that cannot see a change under
# test emits a confident, well-formed, internally consistent verdict about a
# world that has moved, and blames the estate. Four instances, four nodes, one
# day. **None was corrected by the instrument noticing; each by another node
# asking.**
#
# **A CONTENT TEST, NEVER A CHRONOLOGICAL ONE, AND THAT IS THE CRITERION'S OWN
# RULING RATHER THAN A PREFERENCE HERE.** A build from a dirty tree carries code
# YOUNGER than any commit stamp, so build time fails in both directions; a pin
# fixes the revision you measured and carries nothing about HEAD. So this file
# does not read `intent-source-commit`, deliberately -- that marker names a
# COMMIT, exactly and only, and `AT-11.5` has already measured two binaries with
# identical markers where one wiped the estate and one worked. Provenance is
# `self_provenance_check.sh`'s question and it is a different question.
#
# ==========================================================================
# THE DERIVATION IS THE WHOLE CARE, AND ITS FIRST FORM WAS WRONG
# ==========================================================================
#
# A hardcoded probe rots: it tests currency against the day it was written, so
# the currency check goes stale exactly like its subjects. The probe is
# therefore DERIVED from the newest commit that contributes one.
#
# **THE FIRST DERIVATION RETURNED 0 ON EVERY ARTEFACT AND THE BINARY WAS FINE.**
# Measured 2026-08-31: eleven candidate literals from three recent commits, all
# 0 hits against a current `intent` -- and the cause was that
# `git show -- 'crates/*/src/*.rs'` happily yields literals from colocated
# `#[cfg(test)]` modules (`mcp.rs:1260` encloses `mcp.rs:1266` and `:1310`).
# **A `cfg(test)` literal ships in no binary ever built, so an instrument using
# one reports EVERY artefact stale** -- a confident, well-formed verdict about a
# world that never existed, which is this row's own class committed by the
# instrument built to catch it. It was caught by driving the derivation before
# trusting it, and by nothing else.
#
# **SO A CANDIDATE IS REJECTED WHEN IT SITS AFTER ITS FILE'S FIRST
# `#[cfg(test)]`.** That is a convention test, not a parse: this codebase puts
# test modules at the end of the file. It is stated as a limitation rather than
# sold as exact.
#
# ==========================================================================
# WHY THERE IS NO DESIGNATED REFERENCE BINARY -- THE CIRCULARITY
# ==========================================================================
#
# The obvious control is "the probe must match a known-current artefact". It is
# circular and it fails in the one direction that matters: if the designated
# reference is ITSELF stale, every probe surviving that filter is one the stale
# binary already carries, and the instrument certifies staleness as currency.
#
# **SO THE CONTROL IS TWO-SIDED AND NAMES NO PRIVILEGED ARTEFACT: the probe must
# be shown able to match SOMETHING.** If it matches nothing in the estate this
# tool CANNOT DISTINGUISH "every artefact is stale" from "this literal never
# ships", and it says so at exit 2 rather than reporting a clean sweep of
# findings. A finding from a probe that has not demonstrated itself is not a
# finding.
#
# ==========================================================================
# REACH -- PRINTED, NOT ONLY WRITTEN HERE
# ==========================================================================
#
# Emitted in the output because `output-contracts.md` is the estate's own
# receipt that co-location in the SOURCE does not co-locate in the EMISSION: a
# caveat must travel with the output, because the output is what gets quoted.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
CRATES="$ROOT/native/rust/crates"
DEPTH="${DEPTH:-40}"

die() { echo "instrument-currency: $*" >&2; exit 2; }
[ -d "$CRATES" ] || die "no crates dir at $CRATES"

# --------------------------------------------------------------------------
# THE ARTEFACT POPULATION: every Intent binary an instrument could be pointed
# at. Derived from the delivery paths and the build tree, never hand-listed.
# --------------------------------------------------------------------------
artefacts=()
for cand in \
  "$ROOT/native/rust/target/release/intent" \
  "$ROOT/native/rust/target/release/intentd" \
  "$ROOT/native/rust/target/debug/intent" \
  "$ROOT/native/rust/target/debug/intentd" \
  "$(command -v intent 2>/dev/null || true)" \
  "$(command -v intentd 2>/dev/null || true)"; do
  [ -n "$cand" ] && [ -f "$cand" ] || continue
  real="$(cd "$(dirname "$cand")" && pwd)/$(basename "$cand")"
  [ -L "$cand" ] && real="$(readlink "$cand")"
  case " ${artefacts[*]-} " in *" $real "*) continue ;; esac
  artefacts+=("$real")
done

[ "${#artefacts[@]}" -gt 0 ] || die "no Intent binary found -- nothing to ask about. This is exit 2 and not a finding: an estate mid-build has no artefact, and reporting one as stale would be the class this file exists for."

# --------------------------------------------------------------------------
# DERIVE THE PROBE. Walk back from HEAD; the first commit yielding a candidate
# that survives the cfg(test) rejection defines it.
# --------------------------------------------------------------------------
probe_commit=""
probes=()
srcs=()

while IFS= read -r commit; do
  [ -n "$commit" ] || continue
  cands=(); srcs=()
  while IFS= read -r lit; do
    [ -n "$lit" ] || continue
    # The literal must still be in the tree -- a probe for deleted code tests nothing.
    hit="$(grep -rn -F -- "$lit" "$CRATES"/*/src/*.rs 2>/dev/null | head -1)"
    [ -n "$hit" ] || continue
    file="${hit%%:*}"; rest="${hit#*:}"; line="${rest%%:*}"
    # REJECT a literal sitting after its file's first `#[cfg(test)]`.
    tmod="$(grep -n '#\[cfg(test)\]' "$file" 2>/dev/null | head -1 | cut -d: -f1)"
    if [ -n "$tmod" ] && [ "$line" -ge "$tmod" ]; then continue; fi
    cands+=("$lit")
    # The literal's SOURCE FILE, in the form rustc embeds in panic locations
    # (`crates/<crate>/src/<file>.rs`). Kept because an artefact that does not
    # link this file CANNOT carry the probe, and reporting that as BLIND is the
    # very defect this row is about -- see the UNREACHABLE partition below.
    srcs+=("crates/${file#"$CRATES"/}")
  done < <(git -C "$ROOT" show "$commit" -- 'native/rust/crates/*/src/*.rs' 2>/dev/null \
    | grep -E '^\+' | grep -oE '"[a-z][a-z0-9 :;,._-]{24,70}"' | tr -d '"' | sort -u)

  if [ "${#cands[@]}" -gt 0 ]; then
    probe_commit="$commit"
    probes=("${cands[@]}")
    break
  fi
done < <(git -C "$ROOT" log -n "$DEPTH" --format=%h -- 'native/rust/crates/*/src/*.rs' 2>/dev/null)

[ -n "$probe_commit" ] || die "no shipping literal found in the last $DEPTH source commits -- the probe could not be derived, so nothing here is measurable. Raise DEPTH or accept that this run establishes nothing."

head_sha="$(git -C "$ROOT" log -1 --format=%h)"
behind="$(git -C "$ROOT" rev-list --count "$probe_commit..HEAD" 2>/dev/null || echo '?')"

echo "AT-00.15 -- instrument currency, by CONTENT"
echo ""
echo "PROBE derived from $probe_commit ($behind commit(s) behind HEAD $head_sha), ${#probes[@]} literal(s):"
for p in "${probes[@]}"; do echo "    \"$p\""; done
echo ""

# --------------------------------------------------------------------------
# CONTROL A -- `strings` produced output at all. A zero from an unreadable
# artefact is an instrument failure wearing a finding's clothes.
# --------------------------------------------------------------------------
echo "CONTROL A -- the reader works (total strings per artefact):"
readable=0
for a in "${artefacts[@]}"; do
  n="$(strings "$a" 2>/dev/null | wc -l | tr -d ' ')"
  printf '    %-8s %s\n' "$n" "${a#"$ROOT"/}"
  [ "${n:-0}" -gt 0 ] && readable=$((readable + 1))
done
[ "$readable" -gt 0 ] || die "strings produced nothing on any artefact -- every result below would be an artefact of the reader"
echo ""

# --------------------------------------------------------------------------
# CONTROL B -- the probe is shown able to MATCH. No designated reference; the
# demonstration is that something in the population carries it.
# --------------------------------------------------------------------------
# THE THIRD STATE, AND IT IS NOT A REFINEMENT -- IT IS THIS ROW'S OWN DEFECT
# FOUND IN THIS FILE. `intentd` does not depend on `intent-cli` (its Cargo.toml
# says so and `dep_graph_guard.rs` enforces it), so a probe derived from a
# literal in `intent-cli` CANNOT be carried by `intentd` however current it is.
# Reporting that as BLIND is a confident, well-formed verdict about a question
# the probe cannot ask of that artefact -- `AC-00.14` exactly, committed by the
# instrument that measures `AC-00.14`. Measured 2026-09-09: both `intentd`
# binaries reported BLIND while both were freshly built from the same commit as
# the `intent` binaries that reported CURRENT.
#
# The discriminator is DERIVED, not declared: rustc embeds each source file's
# path for panic locations, so an artefact carries `crates/<crate>/src/<f>.rs`
# iff that file is linked into it. A hand-listed exclusion would rot exactly
# like the hardcoded probe this file already refuses to have.
#
# DRIVEN TO ALL THREE STATES ON PLANTED ARTEFACTS, 2026-09-09, BECAUSE A NEW
# STATE CAN SWALLOW THE FINDINGS IT WAS ADDED BESIDE. If everything became
# UNREACHABLE this instrument would be retired without anyone deciding to, so
# BLIND was shown to be still reachable rather than assumed to be. Recipe --
# `ROOT` is overridable and `.git` and `crates` are symlinked in, so the probe
# still derives from the real history:
#
#   FAKE=$(mktemp -d); ln -s "$PWD/.git" "$FAKE/.git"
#   mkdir -p "$FAKE/native/rust/target/release" "$FAKE/native/rust/target/debug"
#   ln -s "$PWD/native/rust/crates" "$FAKE/native/rust/crates"
#   release/intent : a probe literal + `crates/intent-cli/src/render.rs`  -> CURRENT
#   debug/intent   : `crates/intent-cli/src/render.rs`, NO probe literal  -> BLIND
#   {release,debug}/intentd : `crates/intentsvcs/src/...` only            -> UNREACHABLE
#   PATH=/usr/bin:/bin ROOT="$FAKE" bash instrument_currency_check.sh
#
# Result: `1 current + 1 blind + 2 unreachable = 4 of 4`. A FOURTH fixture,
# carrying no probe literal on ANY artefact, is refused by CONTROL B at exit 2
# rather than reported -- that arm was driven too and is not new.
srcs_seen=0
for a in "${artefacts[@]}"; do
  n="$(strings "$a" 2>/dev/null | grep -cE 'crates/[a-z-]+/src/' || true)"
  [ "${n:-0}" -gt 0 ] && srcs_seen=$((srcs_seen + 1))
done
[ "$srcs_seen" -eq "${#artefacts[@]}" ] || die "at least one artefact exposes no \`crates/*/src/\` path at all, so UNREACHABLE below could not be distinguished from a reader failure. $srcs_seen of ${#artefacts[@]} exposed one. This is exit 2 and not a finding."

current=(); stale=(); unreachable=()
for a in "${artefacts[@]}"; do
  hits=0
  for p in "${probes[@]}"; do
    c="$(strings "$a" 2>/dev/null | grep -cF -- "$p" || true)"
    hits=$((hits + ${c:-0}))
  done
  linked=0
  for f in "${srcs[@]}"; do
    c="$(strings "$a" 2>/dev/null | grep -cF -- "$f" || true)"
    [ "${c:-0}" -gt 0 ] && linked=1
  done
  if [ "$hits" -gt 0 ]; then current+=("$a")
  elif [ "$linked" -eq 1 ]; then stale+=("$a")
  else unreachable+=("$a"); fi
done

echo "CONTROL B -- the probe can match: ${#current[@]} of ${#artefacts[@]} artefact(s) carry it"
if [ "${#current[@]}" -eq 0 ]; then
  echo ""
  echo "  CANNOT ESTABLISH. The probe matched nothing, and from inside this tool"
  echo "  \"every artefact is stale\" and \"this literal never ships\" are the same"
  echo "  observation. Reporting ${#stale[@]} findings from an undemonstrated probe is"
  echo "  the defect this row is about, so it reports none."
  exit 2
fi
echo ""

echo "VERDICT -- can each artefact see $probe_commit?"
for a in "${current[@]}"; do printf '    CURRENT      %s\n' "${a#"$ROOT"/}"; done
for a in "${stale[@]}"; do printf '    BLIND        %s\n' "${a#"$ROOT"/}"; done
for a in "${unreachable[@]-}"; do [ -n "$a" ] && printf '    UNREACHABLE  %s  -- does not link %s\n' "${a#"$ROOT"/}" "${srcs[0]-the probe source}"; done
echo "    partition: ${#current[@]} current + ${#stale[@]} blind + ${#unreachable[@]} unreachable = $((${#current[@]} + ${#stale[@]} + ${#unreachable[@]})) of ${#artefacts[@]}"
[ $((${#current[@]} + ${#stale[@]} + ${#unreachable[@]})) -eq "${#artefacts[@]}" ] || die "partition does not close"
[ "${#unreachable[@]}" -eq 0 ] || echo "    UNREACHABLE is NOT a finding about currency: the probe's source is not linked into those artefacts, so no probe derived from it can speak about them either way." 
echo ""

# --------------------------------------------------------------------------
# THE CENSUS -- which instruments ASK the question before reporting. Three
# states, because "reads a pin" is the answer the criterion rules INSUFFICIENT
# and it must not be counted with either the ones that ask or the ones that do
# not.
# --------------------------------------------------------------------------
echo "CENSUS -- of the instruments that DRIVE an artefact, which demonstrate it can see the change?"
c_content=0; c_pin=0; c_neither=0; pin_list=(); neither_list=()
for f in "$HERE"/*.sh; do
  base="$(basename "$f")"
  [ "$base" = "instrument_currency_check.sh" ] && continue
  grep -qE 'target/(release|debug)/intent|command -v intent|which intent|INTENT_BIN|\$BIN\b' "$f" 2>/dev/null || continue
  if grep -q 'instrument_currency_check.sh' "$f" 2>/dev/null; then
    c_content=$((c_content + 1))
  elif grep -q 'intent-source-commit' "$f" 2>/dev/null; then
    c_pin=$((c_pin + 1)); pin_list+=("$base")
  else
    c_neither=$((c_neither + 1)); neither_list+=("$base")
  fi
done
total=$((c_content + c_pin + c_neither))
echo "    $c_content demonstrate currency by CONTENT"
echo "    $c_pin read the PROVENANCE PIN only -- the answer AC-00.14 rules insufficient"
echo "    $c_neither drive an artefact and ask it nothing"
echo "    partition: $c_content + $c_pin + $c_neither = $total instrument(s) that drive an artefact"
[ "${#pin_list[@]}" -gt 0 ] && { echo "  PIN ONLY:"; for x in "${pin_list[@]}"; do echo "    $x"; done; }
[ "${#neither_list[@]}" -gt 0 ] && { echo "  ASK NOTHING:"; for x in "${neither_list[@]}"; do echo "    $x"; done; }
echo ""

cat <<'REACH'
REACH OF THIS INSTRUMENT, stated because a limit not in the output is not a
limit the reader has:
  COVERS      Intent's own compiled binaries, against one derived probe.
  DOES NOT    every non-binary artefact an instrument drives -- a generated
              table, a canon extract, a fixture tree, a vendored manifest --
              each of which can be exactly as stale and none of which `strings`
              can be pointed at.
  DOES NOT    whether an instrument's verdict is CORRECT. Correctness and
              validity are two questions and this is the second one only.
  WEAKER THAN IT LOOKS  the probe demonstrates an artefact can see ONE commit,
              not the commit each instrument's own claim is about. A binary
              current for this probe can still be blind to the specific change
              a given instrument is reporting on.
  CONVENTION  the cfg(test) rejection is "after the file's first
              `#[cfg(test)]`", which is this codebase's layout and not a parse.
REACH
echo ""

if [ "${#stale[@]}" -gt 0 ] || [ "$c_content" -eq 0 ]; then
  echo "RED. AC-00.14 asks that every instrument emitting a verdict about the estate"
  echo "DEMONSTRATE it can see the change under test. This measures one necessary"
  echo "condition over one artefact kind and censuses who asks at all; requiring it"
  echo "is a mechanism nothing yet calls, and $c_content instrument(s) call this one."
  exit 1
fi
echo "PASS."
exit 0
