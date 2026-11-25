#!/usr/bin/env bash
# canon-ignore-guard.sh -- refuse a commit that adds an ignore rule reaching
# `intent/.canon/`.
#
# Witness for AT-01.5. The register requires a cited file to name the row
# that cites it, so this id is structural rather than a pointer into a tracker.
#
# **A COMPANION CHECK VERIFIES THE STATE, by cloning. This refuses the
# EDIT, and the gap between those two moments is where the class lives.**
#
# WHY THE HAZARD IS STRUCTURAL AND NOT HYPOTHETICAL. Every other `intent/.<x>/`
# holding per-machine state in a v3 tree is gitignored -- `.cache/`, `.backup/`
# -- so the convention reads *a dot directory under `intent/` is local and never
# travels*. `.canon/` is not the only committed exception (`.config/` is
# committed too), but it is the one holding the entire steel thread and issue
# estate. A future tidy-up adding `intent/.*/` to `.gitignore` is a natural,
# tidy-looking, correct-SEEMING edit that would silently un-commit all of it. **A convention that every sibling follows and one
# exception breaks is not a convention anyone will remember to check.**
#
# WHAT IT ASKS, AND WHY IT ASKS GIT RATHER THAN A REGEX. The question is not
# "does an added line look like an ignore pattern for canon" -- pattern syntax
# has negations, directory suffixes, anchoring and precedence, and a
# hand-rolled matcher would disagree with git exactly where it matters. It asks
# `git check-ignore -v`, which is git's own matcher, and then reads back the
# `<source>:<line>:<pattern>` it reports. **The question is what git DOES, not
# what a rule appears to say** -- the same reason the companion check clones
# instead of reading `.gitignore`.
#
# ATTRIBUTION IS THE OTHER HALF, AND IT IS WHAT KEEPS THIS KEEPABLE. Only rules
# on lines THIS COMMIT ADDS can block. Inherited breakage is reported and never
# failed on, because **a guard that must be bypassed to work is a guard nobody
# keeps** -- and a repository that arrived already broken would otherwise be
# unable to commit the fix.
#
# PROBED AT BOTH REAL AND FUTURE PATHS. A rule that matches nothing on disk
# today but would swallow tomorrow's newly created thread file is the same defect one day
# later, and the estate that suffers it is the one that never notices a new
# thread stopped travelling. So the probe set carries synthetic paths that do
# not exist, which `check-ignore` is happy to answer for.
#
# Exit codes: 0 clean, not applicable, or inherited-only; 1 this commit adds a
# rule that reaches canon, or a step the check needs failed: a git read, a
# command that reads git's answer (grep, head, or awk), or the scratch tree of
# the staged ignore rules.

set -u

CANON_DIR="intent/.canon"

# EVERY STEP BELOW IS CHECKED. A step that failed and was taken as an empty
# answer would read as "nothing to check" and exit 0, which is the silent pass
# this guard exists to prevent. So a failed step refuses the commit and says
# which step failed, below the failing command's own message.
refuse() {
  echo "BLOCKED: canon-ignore-guard could not $1." >&2
  echo "  Nothing was checked, so nothing is allowed. Re-run the commit; if it repeats, report it." >&2
  exit 1
}

# Not applicable: no canon in this project. Opt-in by presence, exactly like the
# whiteboard guards -- nothing changes for a tree that has not relocated yet.
[ -d "$CANON_DIR" ] || exit 0

# The ignore sources a commit can add a rule to. `.git/info/exclude` is
# deliberately NOT here: it is not committable, so no commit can add to it, and
# naming it would imply a check this cannot perform.
# `mapfile` is bash 4+ and macOS ships 3.2.57, so this reads as a newline-
# delimited string exactly the way the whiteboard guards do. A hook that
# silently does nothing on the primary platform is the worst shape a guard
# can take: it reports success having checked nothing.
# R as well as A, C, and M: a renamed ignore file takes its rules to a new
# place, and every line at its new path reads as added below.
staged="$(git diff --cached --name-only --diff-filter=ACMR)" ||
  refuse "read which files this commit stages"
# grep exits 1 when no staged path is an ignore file, which is an answer, and 2
# when it could not run, which is not.
ignore_files="$(grep -E '(^|/)\.gitignore$' <<<"$staged")"
[ $? -le 1 ] || refuse "find the ignore files among the staged paths"

# Nothing touched an ignore file, so no rule can have been added. This is the
# overwhelmingly common path and it costs one `git diff`.
[ -n "$ignore_files" ] || exit 0

# The probe set: what canon looks like now, plus what it will look like next.
# Real paths first so a failure names a file the operator can go and look at.
# git's list is read whole before its head is taken, so a git that failed is
# refused, not read as a canon with no files.
tracked="$(git ls-files -- "$CANON_DIR")" || refuse "list the canon's tracked paths"
sampled="$(head -200 <<<"$tracked")" || refuse "take the first of the canon's tracked paths"
PROBES=()
while IFS= read -r p; do
  [ -n "$p" ] && PROBES+=("$p")
done <<<"$sampled"
# Named probes. A probe here need not exist: the damage a rule does to a file
# that has not been created yet is invisible until someone notices it never
# travelled. Intent mints every new id git-style, so the future thread, its
# work package, the package's criterion and test, and the issue are named with
# the worked example's hex and labels, which name no record
# (`intentsvcs::id::EXAMPLE_HEX`, `EXAMPLE_WP_HEX`, and `LabelKind::examples`).
# There is one probe for each place the project's layout keeps a record, each
# at the path `intentsvcs::project` gives: the thread's file and an opaque
# attachment's bytes under `st/`, a record of each kind under `records/` (a
# package, a criterion, a test, and an attachment), the issue's file, and an
# event's file under `events/` (`intentsvcs::event::EVENTS_DIR`), dated, with
# the ULID specification's example id. An ordinal file is a real path or
# nothing: none is minted again. The project's state file is probed by its name
# as well (`canon_project_rel`). A project in 3.4.0's format may have none. The
# real paths above are the first paths git lists, and the event files sort
# first, so they can miss every other place; a canon with no event yet has no
# real event path either. A named probe that is also a real path above is not
# added again, so a rule that reaches it counts it once. The refusal below
# prints this list, so it names the probes the guard holds and no copy of them.
NAMED_PROBES=(
  "${CANON_DIR}/events/2026/01/01/01ARZ3NDEKTSV4RRFFQ69G5FAV.json"
  "${CANON_DIR}/st/ST3f9a2c1d4e5f6a7b.json"
  "${CANON_DIR}/st/ST3f9a2c1d4e5f6a7b/diagram.png"
  "${CANON_DIR}/records/ST3f9a2c1d4e5f6a7b/wp/c2e41a0b.json"
  "${CANON_DIR}/records/ST3f9a2c1d4e5f6a7b/ac/AC-c2e41a0b.1.json"
  "${CANON_DIR}/records/ST3f9a2c1d4e5f6a7b/at/AT-c2e41a0b.1.json"
  "${CANON_DIR}/records/ST3f9a2c1d4e5f6a7b/attachments/design.md.json"
  "${CANON_DIR}/issues/I3f9a2c1d4e5f6a7b.json"
  "${CANON_DIR}/project.json"
  "${CANON_DIR}/.probe-a-path-that-will-exist-later"
)
for p in "${NAMED_PROBES[@]}"; do
  case $'\n'"$sampled"$'\n' in
    *$'\n'"$p"$'\n'*) ;;
    *) PROBES+=("$p") ;;
  esac
done

# Which (file, line) pairs does THIS commit add? Read once, as `path:linenum`.
# `--unified=0` so hunk headers give exact post-image line numbers with no
# context to walk past.
added_pairs=""
while IFS= read -r f; do
  [ -n "$f" ] || continue
  hunks="$(git diff --cached --unified=0 -- "$f")" ||
    refuse "read the lines this commit adds to $f"
  pairs="$(
    printf '%s\n' "$hunks" |
      awk -v file="$f" '
        /^@@/ {
          # @@ -a,b +c,d @@ -- take c and d from the post-image side.
          match($0, /\+[0-9]+(,[0-9]+)?/)
          spec = substr($0, RSTART + 1, RLENGTH - 1)
          n = split(spec, parts, ",")
          line = parts[1] + 0
          next
        }
        /^\+/ && !/^\+\+\+/ { print file ":" line; line++ }
        /^[^+-]/ { line++ }
      '
  )" || refuse "number the lines this commit adds to $f"
  [ -n "$pairs" ] && added_pairs="${added_pairs}${pairs}"$'\n'
done <<< "$ignore_files"

# `check-ignore -v` on the whole probe set in one call. `--no-index` so the
# answer is about the RULES and not about what happens to be tracked: a tracked
# file is not "ignored" for add purposes, and reading it that way would report a
# clean tree while the rule that orphans every FUTURE artefact sits there.
#
# **BOTH HALVES READ ONE DOCUMENT: THE STAGED ONE**. The
# attribution above comes from the INDEX, and `check-ignore` reads the rules of
# whatever work tree it is pointed at -- by default the checkout's. Those differ
# whenever an ignore file is staged and then edited again, which on a shared
# tree is routine (`git commit --only`, a re-staging formatter), and the
# `source:line` comparison below then compared line numbers across two
# documents: a staged rule reaching canon that the worktree lacked was never
# matched at all, and passed. So every ignore file the INDEX holds is written
# into a scratch work tree and git's own matcher is pointed there. The rules are
# the commit's, and so are the line numbers they are attributed by.
#
# A scratch tree that cannot be built REFUSES: checking the wrong rules, or
# none, and exiting 0 is the silent pass this guard exists to prevent.
refuse_unbuilt() {
  refuse "build the staged ignore rules to check ($1)"
}
git_dir="$(git rev-parse --absolute-git-dir)" || refuse_unbuilt "no git directory"
staged_rules="$(mktemp -d "${TMPDIR:-/tmp}/canon-ignore-XXXXXX")" || refuse_unbuilt "mktemp"
trap 'rm -rf "$staged_rules"' EXIT
staged_ignores="$(git ls-files -- '.gitignore' '*/.gitignore')" || refuse_unbuilt "git ls-files"
if [ -n "$staged_ignores" ]; then
  printf '%s\n' "$staged_ignores" | git checkout-index --prefix="$staged_rules/" --stdin ||
    refuse_unbuilt "checkout-index"
fi
# check-ignore exits 0 when it ignores a probe and 1 when it ignores none, and
# both are answers. Any other code, eg 128 for a fatal error, means git did not
# answer, so the commit is refused, not read as clean.
matches="$(
  cd "$staged_rules" || exit 2
  git --git-dir="$git_dir" --work-tree="$staged_rules" \
    check-ignore -v --no-index -- "${PROBES[@]}"
)"
case $? in
  0) ;;
  1) exit 0 ;;
  *) refuse "ask git which probes the staged ignore rules reach" ;;
esac

# `check-ignore -v` emits `<source>:<linenum>:<pattern>\t<pathname>`, one line
# PER PATH. Split on the TAB, never on the colon -- a pattern may contain one,
# and a rule mis-parsed here would be attributed to the wrong line and either
# blocked when inherited or waved through when added.
#
# GROUPED BY RULE, AND THAT IS A CORRECTNESS PROPERTY OF THE MESSAGE RATHER
# THAN A COSMETIC ONE. Ungrouped, one `intent/.*/` printed an identical
# `source:line:pattern` prefix for every canon path and buried the single fact
# the operator needs -- which rule to delete -- under the evidence for it. The
# gate's own runner warns that a line a reader cannot act on is how a gate's
# output stops being read, and a wall of them is that failure delivered
# wholesale. The count IS the alarming figure; the examples are what makes it
# concrete.
blocking_rules=""
blocking_counts=""
inherited_rules=""
while IFS= read -r m; do
  [ -n "$m" ] || continue
  rule="${m%%	*}"
  path="${m#*	}"
  src="${rule%%:*}"
  rest="${rule#*:}"
  lineno="${rest%%:*}"
  # Herestring, NOT a pipeline. This file sets no `pipefail` today, and that
  # was the ONLY reason the pipeline form was safe here -- adding `pipefail`
  # for hygiene would have armed the SIGPIPE race silently, while reading as
  # a tightening. Removed as a mechanism rather than left as an accident.
  # grep exits 1 when this commit did not add the rule's line, which is an
  # answer, and 2 when it could not run, which is not: read as "not added", an
  # added rule would pass as inherited.
  grep -qxF -- "${src}:${lineno}" <<<"$added_pairs"
  case $? in
    0)
      blocking_rules="${blocking_rules}${rule}"$'\n'
      blocking_counts="${blocking_counts}${rule}	${path}"$'\n'
      ;;
    1) inherited_rules="${inherited_rules}${rule}"$'\n' ;;
    *) refuse "match the rule ${rule} against the lines this commit adds" ;;
  esac
done <<< "$matches"

# Inherited breakage is REPORTED AND NEVER FAILED ON, and it says so on the
# path that prints it -- a contract line that stops being true when a mode
# changes is how a tool comes to assert what it has not measured.
if [ -n "$inherited_rules" ]; then
  echo "intent gate: an EXISTING ignore rule already reaches ${CANON_DIR}/ -- reported, never failed on." >&2
  printf '%s' "$inherited_rules" | sort -u | sed 's/^/  /' >&2
  echo "  (this commit did not add it, and a guard that must be bypassed to fix inherited breakage is one nobody keeps)" >&2
fi

[ -n "$blocking_rules" ] || exit 0

echo "BLOCKED: this commit adds an ignore rule that reaches ${CANON_DIR}/." >&2
echo "" >&2
printf '%s' "$blocking_rules" | sort -u | while IFS= read -r rule; do
  [ -n "$rule" ] || continue
  n="$(printf '%s' "$blocking_counts" | awk -F'\t' -v r="$rule" '$1==r{c++} END{print c+0}')"
  echo "  ${rule}" >&2
  echo "    orphans ${n} canon path(s), including:" >&2
  printf '%s' "$blocking_counts" | awk -F'\t' -v r="$rule" '$1==r{print "      " $2}' | head -3 >&2
done
echo "" >&2
echo "  ${CANON_DIR}/ must be committed: it holds the entire steel thread and issue estate." >&2
echo "  Its per-machine siblings -- .cache/, .backup/ -- are gitignored, so a rule" >&2
echo "  like 'intent/.*/' looks tidy and correct and would silently un-commit the" >&2
echo "  entire steel thread and issue estate (D29: a gitignored path is never canon)." >&2
echo "" >&2
echo "  If you meant a sibling, name it literally the way the others are named." >&2
echo "  The probe set includes paths that do NOT exist yet. So a rule that matches only" >&2
echo "  FUTURE artefacts is caught here too. That is the same defect one day later, and" >&2
echo "  the estate that suffers it never notices that a new thread stopped travelling." >&2
echo "  The paths it probes by name, whether or not they exist:" >&2
printf '    %s\n' "${NAMED_PROBES[@]}" >&2
exit 1
