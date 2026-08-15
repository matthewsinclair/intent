#!/bin/bash
# guide_refs_check.sh -- every `intent <cmd>` written in prose must be a command
# that exists.
#
# AC-09.4 says the agent guide renders from the dispatch table and no
# hand-maintained command list exists. That closes the LIST. It does not close
# the residue, which is where the rot actually happens: the authored half of any
# guide -- workflows, methodology, the NEVER DO section -- is prose that NAMES
# commands, and prose naming a renamed or retired command is a hand-maintained
# command reference that no generator will ever correct.
#
# Measured on the v2 guide before this existed (2026-08-15, ic): `usage-rules.md`
# mentions 54 of the surface's 111 commands and is stale about NONE of them. So
# the failure mode of the hand-maintained half is not drift-into-falsehood, it is
# SILENT OMISSION -- and omission is the worse one for an agent, because a wrong
# command earns an error it can react to while a missing command reads as a
# capability the tool does not have, and the agent quietly works around it. Three
# whole families (`issues`, `modules`, `lang`) are not named anywhere in 389
# lines; `issues` and `modules` do not appear even as words.
#
# This check therefore does the half a generator cannot: the generated section
# guarantees completeness, and this guarantees the prose around it stays true.
# Together they are the pair. Neither alone is AC-09.4.
#
# Usage:
#   guide_refs_check.sh <prose-file> [<prose-file>...]
#   TABLE=<path> guide_refs_check.sh ...

set -uo pipefail

die() { echo "error: $1" >&2; exit 1; }

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# tools -> parity -> ST0056 -> st -> intent -> repo root. FIVE, not four: the
# first version was one short and resolved to `intent/`, so every invocation --
# including all four mutants in the first mutation run -- died on "table not
# found" and the harness scored them as kills. A mutant that dies before
# reaching the branch under test is not evidence about that branch.
REPO_ROOT="$(cd "$HERE/../../../../.." && pwd)"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"

[ -f "$TABLE" ] || die "dispatch table not found: $TABLE"
[ $# -gt 0 ]    || die "no prose file given -- this check reads the AUTHORED half of the guide and has nothing to say about the generated half"

PATHS="$(jq -r '[.families[].entries[], .new_surface[]] | .[].path' "$TABLE" | sort -u)"
[ -n "$PATHS" ] || die "the dispatch table declares no paths -- refusing, because every reference would then resolve to nothing and this check would report the prose as entirely broken"

# Aliases are spellings the surface answers to, so prose using one is correct.
ALIASES="$(jq -r '[.families[].entries[], .new_surface[]] | .[] | (.aliases // [])[]' "$TABLE" | sort -u)"
KNOWN="$(printf '%s\n%s\n' "$PATHS" "$ALIASES" | sed '/^$/d' | sort -u)"

is_known() { printf '%s\n' "$KNOWN" | grep -qxF "$1"; }

# A command that HAS declared subcommands is a family, and a second word after
# one is a subcommand claim that must resolve. A command with none is a leaf, and
# a second word after it is just prose continuing -- "intent critic on staged
# files".
#
# THE DISTINCTION IS LOAD-BEARING AND ITS ABSENCE WAS A SURVIVING MUTANT. The
# first version allowed any second word whenever the first word resolved, so
# `intent st create` -- a command that does not exist, in the family with the
# most subcommands in the surface -- passed as cheerfully as `intent critic on`.
# The check was live, ran green, and could not see the one substitution most
# likely to happen in a rename.
#
# Derived from the table, never listed: a family is anything with at least one
# declared `<name> <sub>` path, so a command that gains its first subcommand
# starts being checked strictly on the same commit.
is_family() { printf '%s\n' "$KNOWN" | grep -qE "^$1 [a-z_]+$"; }

rc=0
for f in "$@"; do
  [ -f "$f" ] || die "prose file not found: $f"

  # Extract every `intent <word>` and `intent <word> <word>`. Placeholders
  # (`<id>`, `<lang>`) and flags stop the match, which is what we want: the
  # command PATH is what has to resolve, not its arguments.
  refs="$(grep -oE '\bintent [a-z_]+([[:space:]]+[a-z_]+)?' "$f" \
          | sed -E 's/^intent[[:space:]]+//; s/[[:space:]]+/ /g' | sort -u)"

  # ENUMERATE, AND SAY HOW MANY. A needle reports on the set it matched, so a
  # regex that silently matches nothing reads exactly like a clean file. Zero
  # references in a guide means the extractor broke, not that the guide is
  # perfect -- so zero REFUSES rather than passing.
  n="$(printf '%s\n' "$refs" | sed '/^$/d' | wc -l | tr -d ' ')"
  [ "$n" -gt 0 ] || die "no \`intent <cmd>\` reference found in $f -- a guide that names no command means the extractor stopped matching, and an empty match set passes every check built on it"

  bad=""
  while IFS= read -r r; do
    [ -n "$r" ] || continue
    is_known "$r" && continue
    first="${r%% *}"
    if is_known "$first" && ! is_family "$first"; then
      # Prose running on past a LEAF command. Legitimate English, resolves.
      continue
    fi
    if is_family "$first"; then
      bad="$bad
  intent $r  -- '$first' is a family and '${r#* }' is not one of its subcommands"
    else
      bad="$bad
  intent $r  -- no such command"
    fi
  done <<EOF
$refs
EOF

  if [ -n "$bad" ]; then
    echo "error: $f references command(s) the dispatch table does not declare -- a renamed or retired command named in prose is a hand-maintained command reference, and nothing regenerates it:$bad" >&2
    rc=1
  else
    echo "ok: $f -- $n distinct command reference(s), all declared" >&2
  fi
done

exit $rc
