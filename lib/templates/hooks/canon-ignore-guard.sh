#!/usr/bin/env bash
# canon-ignore-guard.sh -- refuse a commit that adds an ignore rule reaching
# `intent/.canon/`.
#
# ST0057 AT-01.5, covering AC-01.5. **AC-01.2 checks the STATE, by cloning. This refuses the
# EDIT, and the gap between those two moments is where the class lives.**
#
# WHY THE HAZARD IS STRUCTURAL AND NOT HYPOTHETICAL. Every other `intent/.<x>/`
# in a v3 tree is gitignored -- `.treeindex/`, `.cache/`, `.backup/` -- so the
# convention reads *a dot directory under `intent/` is local and never travels*.
# `.canon/` is the single deliberate exception, and it holds the entire steel
# thread and issue estate. A future tidy-up adding `intent/.*/` to `.gitignore`
# is a natural, tidy-looking, correct-SEEMING edit that would silently
# un-commit all of it. **A convention that every sibling follows and one
# exception breaks is not a convention anyone will remember to check.**
#
# WHAT IT ASKS, AND WHY IT ASKS GIT RATHER THAN A REGEX. The question is not
# "does an added line look like an ignore pattern for canon" -- pattern syntax
# has negations, directory suffixes, anchoring and precedence, and a
# hand-rolled matcher would disagree with git exactly where it matters. It asks
# `git check-ignore -v`, which is git's own matcher, and then reads back the
# `<source>:<line>:<pattern>` it reports. **The question is what git DOES, not
# what a rule appears to say** -- the same reason AC-01.2 clones instead of
# reading `.gitignore`.
#
# ATTRIBUTION IS THE OTHER HALF, AND IT IS WHAT KEEPS THIS KEEPABLE. Only rules
# on lines THIS COMMIT ADDS can block. Inherited breakage is reported and never
# failed on, because **a guard that must be bypassed to work is a guard nobody
# keeps** -- and a repository that arrived already broken would otherwise be
# unable to commit the fix.
#
# PROBED AT BOTH REAL AND FUTURE PATHS. A rule that matches nothing on disk
# today but would swallow tomorrow's `ST0058.json` is the same defect one day
# later, and the estate that suffers it is the one that never notices a new
# thread stopped travelling. So the probe set carries synthetic paths that do
# not exist, which `check-ignore` is happy to answer for.
#
# Exit codes: 0 clean, not applicable, or inherited-only; 1 this commit adds a
# rule that reaches canon.

set -u

CANON_DIR="intent/.canon"

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
ignore_files="$(
  git diff --cached --name-only --diff-filter=ACM 2>/dev/null |
    grep -E '(^|/)\.gitignore$' || true
)"

# Nothing touched an ignore file, so no rule can have been added. This is the
# overwhelmingly common path and it costs one `git diff`.
[ -n "$ignore_files" ] || exit 0

# The probe set: what canon looks like now, plus what it will look like next.
# Real paths first so a failure names a file the operator can go and look at.
PROBES=()
while IFS= read -r p; do
  PROBES+=("$p")
done < <(git ls-files "$CANON_DIR" 2>/dev/null | head -200)
# Future artefacts. These do not exist and do not need to: the damage a rule
# does to a file that has not been created yet is invisible until someone
# notices it never travelled.
PROBES+=(
  "${CANON_DIR}/st/ST9999.json"
  "${CANON_DIR}/issues/9999.json"
  "${CANON_DIR}/.probe-a-path-that-will-exist-later"
)

# Which (file, line) pairs does THIS commit add? Read once, as `path:linenum`.
# `--unified=0` so hunk headers give exact post-image line numbers with no
# context to walk past.
added_pairs=""
while IFS= read -r f; do
  [ -n "$f" ] || continue
  pairs="$(
    git diff --cached --unified=0 -- "$f" 2>/dev/null |
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
  )"
  [ -n "$pairs" ] && added_pairs="${added_pairs}${pairs}"$'\n'
done <<< "$ignore_files"

# `check-ignore -v` on the whole probe set in one call. `--no-index` so the
# answer is about the RULES and not about what happens to be tracked: a tracked
# file is not "ignored" for add purposes, and reading it that way would report a
# clean tree while the rule that orphans every FUTURE artefact sits there.
matches="$(git check-ignore -v --no-index -- "${PROBES[@]}" 2>/dev/null || true)"
[ -n "$matches" ] || exit 0

# `check-ignore -v` emits `<source>:<linenum>:<pattern>\t<pathname>`, one line
# PER PATH. Split on the TAB, never on the colon -- a pattern may contain one,
# and a rule mis-parsed here would be attributed to the wrong line and either
# blocked when inherited or waved through when added.
#
# GROUPED BY RULE, AND THAT IS A CORRECTNESS PROPERTY OF THE MESSAGE RATHER
# THAN A COSMETIC ONE. Ungrouped, one `intent/.*/` printed 100 identical
# `source:line:pattern` prefixes and buried the single fact the operator needs
# -- which rule to delete -- under the evidence for it. The gate's own runner
# warns that a line a reader cannot act on is how a gate's output stops being
# read, and 100 of them is that failure delivered wholesale. The count IS the
# alarming figure; the examples are what makes it concrete.
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
  if grep -qxF -- "${src}:${lineno}" <<<"$added_pairs"; then
    blocking_rules="${blocking_rules}${rule}"$'\n'
    blocking_counts="${blocking_counts}${rule}	${path}"$'\n'
  else
    inherited_rules="${inherited_rules}${rule}"$'\n'
  fi
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
echo "  ${CANON_DIR}/ is the ONE dot directory under intent/ that must be committed." >&2
echo "  Its siblings -- .treeindex/, .cache/, .backup/ -- are all local, so a rule" >&2
echo "  like 'intent/.*/' looks tidy and correct and would silently un-commit the" >&2
echo "  entire steel thread and issue estate (D29: a gitignored path is never canon)." >&2
echo "" >&2
echo "  If you meant a sibling, name it literally the way the others are named." >&2
echo "  The probe set includes paths that do NOT exist yet (ST9999.json), so a rule" >&2
echo "  matching only FUTURE artefacts is caught here too -- that is the same defect" >&2
echo "  one day later, and the estate that suffers it never notices a new thread" >&2
echo "  stopped travelling." >&2
exit 1
