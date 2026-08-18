#!/bin/bash
# realise_plan.sh -- what would `intent organize` do, and what would it refuse?
#
# An executable reading of `intent/st/ST0056/realisation.md`. It computes the
# manifest a project SHOULD have and the reconciliation plan against what is on
# disk. It writes nothing. It exists so the design can be argued against a real
# estate rather than a sketch, and so whoever builds `organize` has a second
# derivation to disagree with.
#
# WHAT THIS TOOL CANNOT DO, STATED BESIDE ITS OUTPUT RATHER THAN IN A README.
# The dehydration gate is "re-render the view from the store and compare bytes;
# refuse on any difference". Rendering is the Rust binary's job and this is a
# shell script, so **every DEHYDRATE line below is UNGATED** -- it names a
# candidate, never an authorisation. A plan that printed a dehydrate list
# without saying so would read as permission to delete, which is the one
# reading this whole programme exists to prevent.
#
# CANON IS THE AUTHORITY FOR WHAT EXISTS, NOT THE FILESYSTEM. Threads are
# enumerated from `intent/st/*/thread.json` and issues from
# `intent/issues/NNNN.json`, because a directory listing answers "what has a
# folder" and the question is "what artefacts are there". Those differ by
# exactly the set this design is about.
set -uo pipefail

die() { echo "realise: $*" >&2; exit 2; }

ROOT="${1:-.}"
[ -d "$ROOT/intent" ] || die "no intent/ under $ROOT (argument is a project root)"
MANIFEST="$ROOT/intent/.intentfiles"
command -v jq >/dev/null || die "jq is required"

# ---------------------------------------------------------------------------
# 1. Canon -- what artefacts exist, and which are terminal
# ---------------------------------------------------------------------------
# Terminal is the whole default rule, so it is spelled ONCE. cc is adding
# `is_terminal()` to the model for the doctor's completion arm; when `organize`
# is built it must call that, not re-spell this list. Two spellings of one
# vocabulary is how the facade/doctor contradiction got in.
is_terminal_thread() { case "$1" in completed|cancelled) return 0 ;; *) return 1 ;; esac; }
is_terminal_issue()  { case "$1" in closed) return 0 ;; *) return 1 ;; esac; }

AUTO="$(mktemp)"; ALL="$(mktemp)"; trap 'rm -f "$AUTO" "$ALL"' EXIT
n_thread=0; n_issue=0
# ST0057 WP-01: canon is FLAT -- `.canon/st/<ID>.json`, one file per thread, id
# INSIDE the JSON. The `<ID>/thread.json` nesting is gone entirely, so this is not
# a prefix substitution: `s|intent/st/|intent/.canon/|` yields a glob matching
# nothing while looking correct. Shape confirmed by cc against the applied patch.
#
# The id still comes from `jq -r '.id'` below and never from the path, which is
# why this relocation costs one line here and three silent breakages elsewhere.
for f in "$ROOT"/intent/.canon/st/*.json; do
  [ -f "$f" ] || continue
  id="$(jq -r '.id // empty' "$f")"; st="$(jq -r '.status // empty' "$f")"
  [ -n "$id" ] || die "thread canon with no id: $f"
  n_thread=$((n_thread + 1))
  echo "STEELTHREAD:$id" >> "$ALL"
  is_terminal_thread "$st" || echo "STEELTHREAD:$id" >> "$AUTO"
done
for f in "$ROOT"/intent/issues/[0-9][0-9][0-9][0-9].json; do
  [ -f "$f" ] || continue
  num="$(jq -r '.number // empty' "$f")"; st="$(jq -r '.status // empty' "$f")"
  [ -n "$num" ] || die "issue canon with no number: $f"
  id="$(printf '%04d' "$num")"
  n_issue=$((n_issue + 1))
  echo "ISSUE:$id" >> "$ALL"
  is_terminal_issue "$st" || echo "ISSUE:$id" >> "$AUTO"
done
# AN EMPTY ESTATE AND CANON-SOMEWHERE-ELSE ARE DIFFERENT DIAGNOSES AND THE OLD
# MESSAGE GAVE ONLY THE FIRST. During the WP-01 window canon EXISTS and has simply
# not moved yet, and "refusing to plan over an empty estate" sends the reader
# hunting a lost estate instead of reading the migration state. Not a
# compatibility shim -- this reads the old location to REPORT, never to process.
if [ "$n_thread" -le 0 ]; then
  legacy_n="$(find "$ROOT/intent/st" -maxdepth 2 -name 'thread.json' 2>/dev/null | grep -c . || true)"
  if [ "${legacy_n:-0}" -gt 0 ]; then
    die "no canon at \`intent/.canon/st/*.json\`, but ${legacy_n} thread(s) are still at the PRE-WP-01 location \`intent/st/<ID>/thread.json\`.
  This is a MIGRATION-STATE report, not an empty estate: the relocation has not run in this tree yet. Run it, or check out a revision whose canon matches this tool."
  fi
  die "no thread canon found at \`intent/.canon/st/*.json\` and none at the pre-WP-01 location either -- refusing to plan over an empty estate"
fi

echo "realise: canon -- $n_thread thread(s), $n_issue issue(s); $(wc -l < "$AUTO" | tr -d ' ') non-terminal"

# ---------------------------------------------------------------------------
# 2. The manifest -- parsed with a grammar that REFUSES rather than skips
# ---------------------------------------------------------------------------
# A line the parser cannot read aborts the run naming the line number. Skipping
# it drops an artefact from realisation and leaves an estate indistinguishable
# from one that never listed it -- the silent-drop shape v2.19.0 already paid
# for twice (`ac gate` F1, and the AT row grammar's `at lint`).
DECL="$(mktemp)"; PINNED="$(mktemp)"; trap 'rm -f "$AUTO" "$ALL" "$DECL" "$PINNED"' EXIT
if [ -f "$MANIFEST" ]; then
  region=pinned; ln=0; bad=0
  while IFS= read -r line || [ -n "$line" ]; do
    ln=$((ln + 1))
    case "$line" in
      '<!-- '*': BEGIN -->') region=generated; continue ;;
      '<!-- '*': END -->')   region=pinned;    continue ;;
      '#'*|'') continue ;;
    esac
    entry="${line%%#*}"; entry="$(printf '%s' "$entry" | sed 's/[[:space:]]*$//')"
    [ -n "$entry" ] || continue
    case "$entry" in
      STEELTHREAD:ST[0-9][0-9][0-9][0-9]|ISSUE:[0-9][0-9][0-9][0-9]) ;;
      *) echo "realise: MALFORMED $MANIFEST:$ln -- '$line'" >&2; bad=$((bad + 1)); continue ;;
    esac
    echo "$entry" >> "$DECL"
    [ "$region" = pinned ] && echo "$entry" >> "$PINNED"
  done < "$MANIFEST"
  [ "$bad" -eq 0 ] || die "$bad malformed line(s) -- REFUSING; a skipped line silently drops an artefact"
  echo "realise: manifest -- $(sort -u "$DECL" | wc -l | tr -d ' ') declared, $(sort -u "$PINNED" | wc -l | tr -d ' ') pinned by hand"
else
  echo "realise: manifest -- ABSENT at $MANIFEST; planning against the default rule alone"
fi

# ---------------------------------------------------------------------------
# 3. The plan
# ---------------------------------------------------------------------------
# Declared = the generated set (non-terminal) UNION the hand pins. A pin
# outranks status: that is the entire point of the pinned region, and without
# it the first status change deletes a human's working copy with no line in the
# output naming the decision.
WANT="$(mktemp)"; trap 'rm -f "$AUTO" "$ALL" "$DECL" "$PINNED" "$WANT"' EXIT
cat "$AUTO" > "$WANT"; [ -s "$PINNED" ] && cat "$PINNED" >> "$WANT"
sort -u "$WANT" -o "$WANT"

hyd=0; ver=0; deh=0; non=0; v2seen=0
DEHLIST="$(mktemp)"; trap 'rm -f "$AUTO" "$ALL" "$DECL" "$PINNED" "$WANT" "$DEHLIST"' EXIT
while read -r e; do
  kind="${e%%:*}"; id="${e#*:}"
  case "$kind" in
    STEELTHREAD) dir="$ROOT/intent/st/$id"; probe="$dir/info.md" ;;
    # THE ISSUE VIEW HAS NO v3 PATH, AND THE TOOL FOUND THAT RATHER THAN THE
    # DESIGN. `intent/issues/NNNN.json` is canon; the only rendered issue
    # markdown in the estate is v2's `issues/<BUCKET>/NNNN/NNNN-slug.md`,
    # which is exactly the residue this design is meant to retire. So probe
    # BOTH and report which layout answered -- a probe that looked only at
    # the v3 path would report all 40 absent and read as "nothing to do".
    ISSUE)
      probe="$ROOT/intent/issues/$id/$id.md"
      if [ ! -f "$probe" ]; then
        v2probe="$(find "$ROOT/intent/issues" -path "*/$id/*.md" -type f 2>/dev/null | head -1)"
        [ -n "$v2probe" ] && { probe="$v2probe"; v2seen=$((v2seen + 1)); }
      fi
      ;;
  esac
  want=no; grep -qxF "$e" "$WANT" && want=yes
  have=no; [ -f "$probe" ] && have=yes
  if   [ "$want" = yes ] && [ "$have" = no  ]; then hyd=$((hyd + 1)); echo "  HYDRATE   $e"
  elif [ "$want" = yes ] && [ "$have" = yes ]; then ver=$((ver + 1))
  elif [ "$want" = no  ] && [ "$have" = yes ]; then deh=$((deh + 1)); echo "$e" >> "$DEHLIST"
  else non=$((non + 1))
  fi
done < "$ALL"

echo "realise: plan -- HYDRATE $hyd, VERIFY $ver, DEHYDRATE-CANDIDATE $deh, already-absent $non (of $((n_thread + n_issue)) artefact(s))"
if [ "$v2seen" -gt 0 ]; then
  echo "realise: NOTE -- $v2seen issue view(s) were found ONLY at v2's bucket path"
  echo "         (\`issues/<BUCKET>/NNNN/NNNN-slug.md\`). v3 declares no rendered issue"
  echo "         markdown path at all, so \`intent edit <ISSUE>\` currently has nowhere to"
  echo "         write. That is a hole in the design, not in the estate."
fi
if [ "$deh" -gt 0 ]; then
  echo "realise: DEHYDRATE candidates are UNGATED and are NOT an authorisation to delete."
  echo "         The gate is: re-render each view from the store, compare bytes, refuse on any"
  echo "         difference. That is the Rust renderer's job and this script cannot run it, so"
  echo "         nothing here has been checked for reproducibility. First $( [ "$deh" -gt 6 ] && echo 6 || echo "$deh" ):"
  head -6 "$DEHLIST" | sed 's/^/           /'
fi

# ---------------------------------------------------------------------------
# 4. The manifest this estate should have
# ---------------------------------------------------------------------------
echo
echo "--- proposed $MANIFEST (generated regions only; pins are never emitted by a tool) ---"
{
  echo "# .intentfiles -- which intent artefacts are realised as on-disk files"
  echo "#"
  echo "# Lines between BEGIN and END are GENERATED by \`intent organize\` from status."
  echo "# Lines outside the markers are PINS and are never rewritten or removed."
  echo
  echo "<!-- ISSUES: BEGIN -->"
  grep '^ISSUE:' "$AUTO" 2>/dev/null | sort -r
  echo "<!-- ISSUES: END -->"
  echo
  echo "<!-- STEEL THREADS: BEGIN -->"
  grep '^STEELTHREAD:' "$AUTO" 2>/dev/null | sort -r
  echo "<!-- STEEL THREADS: END -->"
}
