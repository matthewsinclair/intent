#!/usr/bin/env bash
# gen_cut_surface.sh -- what does a NAMED REVISION ship? (hv's doc task, vc 2026-08-29.)
#
# **THE REGISTER IS A STATEMENT ABOUT `main`, AND NOTHING IN IT SAYS SO.** That
# is the whole reason this tool exists. `surface/dispatch-table.json` is the
# command-surface source of truth and the clap surface is BUILT from it, so the
# table at a revision is a sound statement of what that revision's binary
# exposes -- but the copy in your working tree describes the tip, and docs are
# written for a RELEASE. Measured 2026-08-29: `ac edit` and `at edit` sit in
# `populations.shipped` at HEAD and in no published tag, so a doc set written
# from the working copy describes two verbs the shipped tool does not have.
#
# **SO EVERY FIGURE THIS TOOL EMITS IS KEYED TO A REVISION, AND THE OUTPUT SAYS
# WHICH.** A surface count with no revision beside it is the same defect one
# layer out: true when written, silently false at the next merge.
#
# WHY NOT PROBE THE BINARY. `surface_check.sh` and `implemented_check.sh` both
# answer a near-neighbour of this question by running `--help` over the built
# tool. They cannot answer THIS one. The binary in a working tree is whatever
# was last built -- in this tree, `8177b53e`, which the gate's own currency arm
# refuses as 13 non-test files behind HEAD -- so probing it measures the build,
# not the cut. `git show <rev>:surface/dispatch-table.json` measures the cut.
#
# THE POPULATION COMES FROM `lib_surface.sh`, WHICH READS `.populations`.
# It is not re-derived here and it is not parsed here. `.families[].entries[]`
# reads like the command surface and is neither -- too narrow (it omits the
# top-level `new_surface[]` rows, which ship) and too wide (it includes rows
# dispositioned `retire`, which do not exist in the binary) at the same time,
# with opposite signs, so no count-based check flinches. The library exists
# because the same hand-written jq produced the same wrong population five times
# in one week, and a sixth walk written here would be the sixth instance.
#
# **`DISPATCH_TABLE` IS THE SEAM THAT MAKES THIS REV-KEYED.** The library reads
# whatever that variable names, so extracting the table at a revision and
# pointing the library at it reuses the one home rather than working around it.
#
# ==========================================================================
# THE TRAP THIS TOOL IS CAREFUL ABOUT, AND WHICH ITS OUTPUT WARNS READERS OF
# ==========================================================================
#
# **THE POPULATIONS ARE KEYED BY SPELLING AND SPELLING IS NOT UNIQUE.**
# `organize` appears TWICE in `declared` -- the v2 command (`retire`) and an
# unrelated new-surface command that reconciles the tree with `.intentfiles`
# (`new-surface`). It is therefore in `shipped` AND in `retired`, correctly, and
# `declared` is 127 rows over 126 distinct spellings.
#
# The consequence is for the CONSUMER, not the data: the natural computation
# `comm -23 shipped retired` returns 117 and silently DROPS A SHIPPING COMMAND.
# This tool never subtracts one population from another. It diffs `shipped` at
# one revision against `shipped` at another -- and that is safe for a reason
# worth stating rather than assuming: **`shipped` is duplicate-free** (118 rows,
# 118 distinct, measured), so within it a spelling is a key. Do not generalise
# that to the other lists; it is a property of one, measured, not a rule.
#
# ==========================================================================
# WHAT THIS TOOL DOES NOT ANSWER
# ==========================================================================
#
# **REFUSALS HAVE NO DECLARED HOME IN THE REGISTER, AND THAT GAP IS OWED.** A
# row declares path, args, flags, target, disposition, recoverability, MCP
# exposure and read-or-mutate. Nothing structural about what it REFUSES. Of 118
# shipped rows, two mention a refusal anywhere and both do it as prose inside a
# `help` string. So the refusal population here comes from `ALL_VARIANTS` in
# `intentsvcs/tests/error_remedies.rs`, a roster that declares itself exhaustive
# and is checked as one (vc, 2026-08-29) -- the same category of object as
# `.populations`, which is why it qualifies. **The register still ought to carry
# it, and an artefact that silently sourced its third leg elsewhere is how the
# next reader concludes the register does.**
#
# A green here is a DECLARATION agreement and never a behaviour claim. The build
# that emptied two estate views had a perfect surface.

# inputs: surface/dispatch-table.json intent/st/ST0056/parity/tools/lib_surface.sh intent/st/ST0056/parity/tools/lib_mdfmt.sh native/rust/crates/intentsvcs/tests/error_remedies.rs
# inputs-exempt: REV -- a git revision. The whole point of this generator is that it reads its inputs AT a named commit rather than from the working tree, so the revision is the one input that cannot be a tracked file: it is the coordinate the tracked files are read at. Re-derivable by `git rev-parse`.

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(git -C "$HERE" rev-parse --show-toplevel)"
# shellcheck source=lib_surface.sh
. "$HERE/lib_surface.sh"
# shellcheck source=lib_mdfmt.sh
. "$HERE/lib_mdfmt.sh"

REV="HEAD"
BASELINE="v3.0.0"
OUT=""
ROSTER="native/rust/crates/intentsvcs/tests/error_remedies.rs"

usage() {
  cat >&2 <<'USAGE'
usage: gen_cut_surface.sh [--rev <rev>] [--baseline <rev>] [--out <path>]

  --rev       the revision the artefact is ABOUT (default: HEAD)
  --baseline  the revision to diff against (default: v3.0.0, the published tag)
  --out       write here instead of stdout

Every figure is keyed to --rev. There is no mode that emits an unkeyed count.
USAGE
  exit 2
}

while [ $# -gt 0 ]; do
  case "$1" in
    --rev)      REV="${2:?--rev needs a revision}"; shift 2 ;;
    --baseline) BASELINE="${2:?--baseline needs a revision}"; shift 2 ;;
    --out)      OUT="${2:?--out needs a path}"; shift 2 ;;
    -h|--help)  usage ;;
    *)          echo "error: unknown argument \`$1\`" >&2; usage ;;
  esac
done

# `resolve` and `table_at` USED TO LIVE HERE and are now in `lib_surface.sh`,
# because the second caller arrived: `gen_reference.sh` needs the same two, and
# both carry traps that were paid for once -- a rev that must refuse rather than
# fall through to the working tree, and the braced expansion that stops zsh
# reading `$sha:s...` as a history modifier and handing back a fabricated zero.
# Two copies would be two homes for a mechanism whose entire value is refusing
# correctly, and the second copy is where the next drift starts.
resolve() { surface_resolve_rev "$1"; }
table_at() { surface_table_at "$1" "$2"; }

shipped_at() {
  local table="$1" out
  out="$(DISPATCH_TABLE="$table" surface_shipped)" || return 1
  printf '%s\n' "$out" | LC_ALL=C sort
}

# **THE FLAG DELTA IS A SEPARATE QUESTION FROM THE COMMAND DELTA AND WAS THE
# ONE ACTUALLY BEING ASKED.** The command diff above answers "what verbs are
# new"; it is silent on a flag added to a verb that already shipped, which is
# exactly the shape of the gap that sent `--note` to hv as an open scope item.
# Measured 2026-08-31: 13 new commands and 28 new flags, so reporting only the
# first understates the delivery by more than half.
#
# TWO NARROWINGS, BOTH LOAD-BEARING, BOTH MEASURED RATHER THAN ASSUMED:
#
#   RESTRICTED TO `shipped`  A row dispositioned `retire` STILL CARRIES ITS
#                            FLAGS in the register. Unrestricted, `fileindex`'s
#                            8 flags and `st repair --write` read as present at
#                            HEAD, and the removal count came out as 1 when the
#                            true figure is 10. The restriction changed the
#                            REMOVED set and left the NEW set byte-identical --
#                            checked, not assumed, because a narrowing that
#                            moves a number you are not watching is the
#                            denominator attack.
#
#   INTRINSIC EXCLUDED       clap supplies `--help`/`-h` on every command, so
#                            they appear and disappear with the command and
#                            carry no delta signal of their own. The register
#                            already dispositions them `intrinsic`, so this
#                            reads a DECLARED field rather than matching on the
#                            spelling -- the inference-from-name that EXP-05
#                            exists to replace.
#
# **AND THE REMOVALS ARE PARTITIONED RATHER THAN TOTALLED.** A flag that left
# because its whole command was retired is already counted one section up;
# reporting it again as a flag regression double-counts one decision and
# manufactures alarm. Only a flag removed from a command that STILL SHIPS is an
# independent regression. Measured: 9 + 1 = 10, and the 1 is `st new
# --dehydrate`, retired by hv on 2026-08-27.
#
# KNOWN IMPRECISION, STATED BECAUSE IT CANNOT BE FIXED HERE: pairs are keyed on
# (path, spelling) and `organize` is two different commands sharing one path,
# so their flags union. It is the same duplicate-spelling trap the closing
# section names for commands, and it is the reason this keys on the pair rather
# than reporting a bare per-command count.
flags_at() {
  local table="$1" shipped="$2" out
  out="$(jq -r '[.. | objects | select(has("flags") and has("path")) | .path as $p
    | (.flags[]? | select((.disposition // "") != "intrinsic") | .spellings[]? | select(startswith("--")))
    | "\($p)\t\(.)"] | unique | .[]' "$table")" || return 1
  printf '%s\n' "$out" | awk -F'\t' 'NR==FNR{s[$0];next} $1 in s' "$shipped" - | LC_ALL=C sort
}

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

REV_SHA="$(resolve "$REV")"
BASE_SHA="$(resolve "$BASELINE")" || {
  echo "note: baseline \`${BASELINE}\` did not resolve; the artefact will carry no comparison" >&2
  BASE_SHA=""
}

table_at "$REV_SHA" "$TMP/rev.json"
shipped_at "$TMP/rev.json" > "$TMP/rev.shipped"
REV_N="$(wc -l < "$TMP/rev.shipped" | tr -d ' ')"
flags_at "$TMP/rev.json" "$TMP/rev.shipped" > "$TMP/rev.flags"
REV_F="$(wc -l < "$TMP/rev.flags" | tr -d ' ')"

BASE_N="--"
BASE_F="--"
if [ -n "$BASE_SHA" ]; then
  if table_at "$BASE_SHA" "$TMP/base.json" 2>/dev/null; then
    shipped_at "$TMP/base.json" > "$TMP/base.shipped"
    BASE_N="$(wc -l < "$TMP/base.shipped" | tr -d ' ')"
    flags_at "$TMP/base.json" "$TMP/base.shipped" > "$TMP/base.flags"
    BASE_F="$(wc -l < "$TMP/base.flags" | tr -d ' ')"
  else
    echo "note: no register at ${BASELINE}; comparison omitted" >&2
    BASE_SHA=""
  fi
fi

# The roster of refusals, read at the SAME revision as everything else.
git -C "$ROOT" show "${REV_SHA}:${ROSTER}" 2>/dev/null \
  | awk '/^const ALL_VARIANTS/{f=1;next} f&&/^\];/{exit} f' \
  | grep -oE '"[A-Za-z]+"' | tr -d '"' | LC_ALL=C sort > "$TMP/refusals" || true
REFUSAL_N="$(wc -l < "$TMP/refusals" | tr -d ' ')"

emit() {
  local when; when="$(date -u +'%Y-%m-%d %H:%MZ')"
  local COMPARED
  # `${x:+a}${x:-b}` is the trap here: `:-` yields the VALUE when set, not the
  # fallback, so the set branch printed the sha twice. Explicit branch instead.
  if [ -n "$BASE_SHA" ]; then
    COMPARED="\`${BASE_SHA}\` (\`${BASELINE}\`)"
  else
    COMPARED="none -- baseline did not resolve"
  fi
  cat <<HDR
# What Intent ships at \`${REV}\`

**Generated by \`intent/st/ST0056/parity/tools/gen_cut_surface.sh\` at ${when}. Do not edit -- re-run it.**

| | |
| --- | --- |
| Revision this describes | \`${REV_SHA}\` (\`${REV}\`) |
| Compared against | ${COMPARED} |
| Commands shipped at this revision | **${REV_N}** |
| Commands shipped at the baseline | ${BASE_N} |
| Flags on shipped commands at this revision | **${REV_F}** |
| Flags on shipped commands at the baseline | ${BASE_F} |
| Refusals declared at this revision | ${REFUSAL_N} |

**Every figure here is a property of one revision.** A count copied out of this file without the revision beside it is true when copied and silently false at the next merge -- which is the exact defect this artefact was built to stop, so it would be a poor place to reintroduce it.

**This is a DECLARATION, not a behaviour claim.** The command surface is built from \`surface/dispatch-table.json\`, so the register at a revision states what that revision exposes. It does not state that any of it works.
HDR

  if [ -n "$BASE_SHA" ]; then
    printf '\n## What changed since `%s`\n\n' "$BASELINE"
    local added removed
    added="$(LC_ALL=C comm -23 "$TMP/rev.shipped" "$TMP/base.shipped" || true)"
    removed="$(LC_ALL=C comm -13 "$TMP/rev.shipped" "$TMP/base.shipped" || true)"

    if [ -n "$added" ]; then
      printf 'Commands at `%s` that `%s` does NOT have -- **anything documented from these describes a tool nobody has installed**:\n\n' "$REV" "$BASELINE"
      printf '%s\n' "$added" | sed 's/^/- `/; s/$/`/'
    else
      printf 'No command exists at `%s` that is absent from `%s`.\n' "$REV" "$BASELINE"
    fi

    if [ -n "$removed" ]; then
      printf '\nCommands `%s` has that `%s` does not -- **a v2-era reader may still type these**:\n\n' "$BASELINE" "$REV"
      printf '%s\n' "$removed" | sed 's/^/- `/; s/$/`/'
    fi

    # **A FLAG ON AN ALREADY-SHIPPING COMMAND IS INVISIBLE TO THE DIFF ABOVE**,
    # and that blind spot is why this section exists: the command list says
    # `at green` shipped at both revisions, which is true and hides that it
    # gained `--note` in between.
    local fadded fgone fgone_with_cmd fgone_standalone
    fadded="$(LC_ALL=C comm -13 "$TMP/base.flags" "$TMP/rev.flags" || true)"
    fgone="$(LC_ALL=C comm -23 "$TMP/base.flags" "$TMP/rev.flags" || true)"

    printf '\n### Flags\n\n'
    printf 'Pairs of (command, flag) over the SHIPPED population at each revision, excluding flags the register dispositions `intrinsic` -- `--help` and `-h` come from clap on every command and would move with the command rather than on their own.\n\n'

    if [ -n "$fadded" ]; then
      printf 'Flags at `%s` that `%s` does NOT have:\n\n' "$REV" "$BASELINE"
      printf '%s\n' "$fadded" | awk -F'\t' '{printf "- `%s` gains `%s`\n", $1, $2}'
    else
      printf 'No flag exists at `%s` that is absent from `%s`.\n' "$REV" "$BASELINE"
    fi

    if [ -n "$fgone" ]; then
      # **PARTITIONED, NOT TOTALLED.** A flag that left because its command was
      # retired is already counted above; repeating it here would double-count
      # one decision and read as a regression nobody made.
      fgone_with_cmd="$(printf '%s\n' "$fgone" | awk -F'\t' 'NR==FNR{s[$0];next} !($1 in s)' "$TMP/rev.shipped" - || true)"
      fgone_standalone="$(printf '%s\n' "$fgone" | awk -F'\t' 'NR==FNR{s[$0];next} ($1 in s)' "$TMP/rev.shipped" - || true)"

      printf '\nFlags `%s` has that `%s` does not, **partitioned by whether the command survived**:\n\n' "$BASELINE" "$REV"

      if [ -n "$fgone_standalone" ]; then
        printf 'On a command that STILL SHIPS -- **an independent flag regression, and the only kind that is news here**:\n\n'
        printf '%s\n' "$fgone_standalone" | awk -F'\t' '{printf "- `%s` loses `%s`\n", $1, $2}'
      else
        printf 'On a command that still ships: none.\n'
      fi

      if [ -n "$fgone_with_cmd" ]; then
        printf '\nOn a command that was itself retired -- **already counted one section up; listed so the arithmetic closes, not as separate news**:\n\n'
        printf '%s\n' "$fgone_with_cmd" | awk -F'\t' '{printf "- `%s` (with `%s`)\n", $2, $1}'
      fi
    fi
  fi

  printf '\n## The full shipped surface at `%s`\n\n' "$REV"
  printf 'Taken from `populations.shipped`, which the register declares to be the one home for this question.\n\n'
  sed 's/^/- `/; s/$/`/' "$TMP/rev.shipped"

  printf '\n## Refusals\n\n'
  printf 'The %s refusal variants declared at this revision, from `ALL_VARIANTS` in `%s`.\n\n' "$REFUSAL_N" "$ROSTER"
  printf '**The register does not carry refusals and it should.** A row declares its path, args, flags, target, disposition, recoverability, MCP exposure and read-or-mutate, and nothing about what it refuses. This list therefore comes from a roster that declares itself exhaustive and is checked as one, which is the same kind of object as `.populations` -- but it lives somewhere else, and a reader who assumes the register carries refusals because this document has them would be wrong. **Filed as owed.**\n\n'
  sed 's/^/- `/; s/$/`/' "$TMP/refusals"

  cat <<'TRAP'

## Two traps for anyone computing from the register

**1. The populations are keyed by spelling, and spelling is not unique.** `organize` is in `shipped` AND in `retired`, correctly: they are two different commands sharing a name -- the retired v2 one, and an unrelated new-surface command that reconciles the tree with `.intentfiles`. So `declared` is 127 rows over 126 distinct spellings, and the natural computation `shipped - retired` returns 117 and **silently drops a shipping command**. Key on the row.

**2. `shipped` is duplicate-free, and that is a measurement rather than a rule.** 118 rows, 118 distinct, so a spelling IS a key within that one list -- which is why the diff above is sound. Do not carry that property to the other lists; it was measured for one.
TRAP
}

# **IDEMPOTENT THROUGH THE FORMATTER, NOT MERELY THROUGH THIS RENDERER.** The
# repo's markdown formatter runs on save and widens table columns. A generator
# that emits narrow ones produces a diff against its own committed output for
# ever, without the data moving a byte -- and a "regenerate and compare" check
# built on that cries wolf on its first run, which is how such a check gets
# switched off and the drift it existed to catch walks free. `lib_mdfmt.sh`
# exists because two generators hit this independently; this is the third, and
# it sources the library rather than growing a third copy of the awk.
emit > "$TMP/raw.md"
md_align "$TMP/raw.md" "$TMP/aligned.md"

if [ -n "$OUT" ]; then
  mkdir -p "$(dirname "$OUT")"
  cp "$TMP/aligned.md" "$OUT"
  echo "ok: wrote $OUT (rev ${REV_SHA:0:12}, ${REV_N} commands, ${REFUSAL_N} refusals)"
else
  cat "$TMP/aligned.md"
fi
