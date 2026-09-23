#!/usr/bin/env bash
#
# staged-format-guard.sh -- THE STAGED BYTES ARE FORMATTED. IT NEVER WRITES.
#
# WHAT IT IS FOR. Ten scripts across the fleet carried one
# contract with no home: run the formatter over the WORKTREE, `git add` the
# result, and commit bytes nobody staged. They were
# rewritten by hand on 2026-09-21 to check the staged bytes and refuse, which
# left ten copies of one contract, each in an untracked hook. An untracked hook
# is not cloned and neither is `core.hooksPath`, so a fresh clone of any of them
# runs none of it, and nothing reports the absence: a check that does not run is
# indistinguishable from a check that passes. This is that contract's one home.
#
# IT JUDGES AND REFUSES AND NEVER WRITES, which is what makes it a guard rather
# than a formatter wearing a guard's name (vc's decision 32, 2026-09-17; hv's
# ruling of 2026-09-21). There is no `git add` here, no `--write`, and every
# probe it creates is removed on the passing, refusing and interrupted paths.
# The formatter's own command is the REMEDY the refusal prints, and a human runs
# it.
#
# IT READS THE STAGED BLOB, NEVER THE WORKTREE, AND THAT IS THE POINT RATHER
# THAN A REFINEMENT (vc, 2026-08-30). Once no hook writes, index and worktree
# stop converging, so a verdict computed from the worktree would be computed
# from bytes the commit may not contain. The case where they differ is the
# HUNK-SCOPED COMMIT, which is the technique the no-writing ruling exists to
# protect: a node staging a hand-crafted blob would otherwise be refused for a
# worktree it is not committing, or passed on a staged blob nobody checked.
#
# THE DECLARATION IS PER PROJECT AND THE BODY IS THIS FILE. `intent/.config/
# config.json` carries `"formatters": ["markdown", "elixir", "rust"]`, and a
# project that declares none is NOT APPLICABLE and ANSWERS 3 to say so. The
# roster's own `applies-when` is a PATH test, and every Intent project has a
# config.json, so the declaration test lives here rather than in the roster row
# -- which is why this guard classifies itself instead of being settled by the
# loop before dispatch.
#
# THE INTERIM IS OVER AND THE PROSE LINE IS GONE. This
# guard used to PRINT `not applicable -- <config> declares no formatters.` and
# exit 0, because the runner read only two answers from a guard it dispatched
# and its SKIPPED class was unreachable from inside one. The printed line
# therefore reached no summary, no `--list-guards` and no tally: it was prose
# for a human reading the hook's output, and it was recorded as an interim at
# the time rather than discovered to be one later. The runner now reads 3 as
# not-applicable and counts it in SKIPPED, so the verdict travels as a number
# a script can read and the line has nothing left to do. A line kept past its
# reason is how a file comes to describe a system it no longer matches.
#
# THE VOCABULARY IS CLOSED AND A FOURTH FORMAT IS A CHANGE TO THIS FILE, never
# an estate-supplied command (vc, 2026-09-22). An estate-supplied command would
# reopen decision 32 in a new place: nothing would stop `prettier --write` being
# declared behind a door whose name promises a check.
#
# A MISSING FORMATTER IS UNENFORCED, NOT UNFORMATTED. The two are different
# facts with different remedies, and the hook this was extracted from conflated
# them: with prettier absent, `git show | prettier --check` failed for every
# file and the refusal named them all as unformatted. A wrong remedy sends the
# reader somewhere wrong, so an absent tool is named in the verdict and blocks
# nothing, and the same distinction is kept for a formatter that ERRORS on a
# file it cannot parse.
#
# THE THREE MECHANICS BELOW ARE INHERITED, EACH FROM A REAL DEFECT, and none of
# them is decoration:
#   * `rustfmt --check` RETURNS 0 ON UNFORMATTED INPUT READ FROM STDIN, so the
#     staged bytes go to a FILE. An arm built on the pipe could never refuse
#     anything and its green would look like a working one.
#   * A PROBE SITS BESIDE THE ORIGINAL, so module and config resolution are the
#     file's own. A lone `.rs` in a scratch directory cannot resolve `mod
#     common;` and rustfmt fails -- which an arm reading only the exit code
#     reports as "not formatted", refusing correctly-formatted files with a
#     remedy that runs clean and changes nothing.
#   * A PROBE IS THE LAST RESORT, NOT THE DEFAULT. No naming scheme fixes a
#     module root whose children live in a subdirectory (`src/tui.rs` resolves
#     `mod focus;` from its STEM, so `src/tui/`), so when the staged blob equals
#     the working tree -- the normal path -- the real file IS the staged content
#     and is checked in place.
#
set -u

NAME="staged-format-guard"
CONFIG="intent/.config/config.json"

note() { printf '%s: %s\n' "$NAME" "$*" >&2; }

# ---- THE DECLARATION ----
#
# AN ESTATE THAT DECLARES NOTHING IS NAMED, NOT SILENT (vc, 2026-09-22). The
# first spelling exited 0 without a word when the key was absent, which is the
# all-clear a reader believes: the roster counts the guard as having RUN, and
# nothing on the screen distinguishes "checked everything and found it clean"
# from "checked nothing at all". An instrument may fail toward a warning a
# reader discards; it may never fail toward an all-clear.
#
# A declaration that cannot be READ is a third fact and it blocks, because a
# tracked declaration the gate cannot parse is a broken tree, and a commit
# passing over it is the silent loss this guard exists to end.
#
# The missing CONFIG is the one silent exit, and it stays silent because the
# roster's own applies-when names that file: reaching here without it means the
# guard was dispatched by something other than the roster.
[ -f "$CONFIG" ] || exit 3
if ! grep -q '"formatters"' "$CONFIG" 2>/dev/null; then
  exit 3
fi

if ! command -v jq >/dev/null 2>&1; then
  note "REFUSED -- $CONFIG declares formatters and jq is not installed, so they cannot be read."
  note "  fix:   install jq, or remove the formatters declaration"
  exit 1
fi

DECLARED="$(jq -r '
  if .formatters == null then empty
  elif (.formatters | type) != "array" then "!not-an-array"
  elif ([.formatters[] | type] | any(. != "string")) then "!not-strings"
  else .formatters[]
  end' "$CONFIG" 2>/dev/null)" || DECLARED="!unreadable"

case "$DECLARED" in
  '!not-an-array'|'!not-strings')
    note "REFUSED -- formatters in $CONFIG must be an array of strings."
    note "  fix:   \"formatters\": [\"markdown\", \"elixir\", \"rust\"]"
    exit 1 ;;
  '!unreadable')
    note "REFUSED -- $CONFIG is not valid JSON, so its formatters cannot be read."
    exit 1 ;;
esac

if [ -z "$DECLARED" ]; then
  exit 3
fi

# ---- WHAT IS STAGED ----
#
# ACM: a DELETION is not a staged blob, and `git show :path` on one fails. The
# filter carries that, and an arm drives it, because a later refactor dropping
# the filter would turn every commit that deletes a Markdown file into a
# refusal naming a file that is not there.
#
# A BINARY BLOB IS SKIPPED rather than handed to a text formatter, AND THE
# QUESTION IS ASKED PER FILE, OF GIT RATHER THAN OF THE BYTES. `--numstat` prints `-`
# for both counts on a binary path, which is git's own answer to the question
# and needs no heuristic here. Parsing the whole listing once and matching paths
# against it was the first spelling and it is worse: the fields are tab
# separated, a path may contain almost anything, and a prefix match would skip a
# text file whose name extends a binary one's.
is_binary() {
  [ "$(git diff --cached --numstat --diff-filter=ACM -- "$1" 2>/dev/null | cut -f1)" = "-" ]
}

# Staged paths for one set of globs, NUL-separated into an array: bash strips
# NUL in command substitution, so the paths cannot travel through a variable,
# and a path containing a space must survive.
STAGED=()
staged_files() {
  STAGED=()
  local f
  while IFS= read -r -d '' f; do
    [ -n "$f" ] || continue
    is_binary "$f" && continue
    STAGED+=("$f")
  done < <(git diff --cached --name-only --diff-filter=ACM -z -- "$@" 2>/dev/null || true)
}

# ---- REPORTING ----
#
# Three states, deliberately apart: BAD is unformatted and refuses; UNMEASURED
# is "could not check" and is named without refusing, because a gate that cannot
# say "I could not check" will eventually say something false instead; and
# UNENFORCED is a declared formatter whose tool is absent.
BAD_REPORT=""
UNMEASURED_REPORT=""
UNENFORCED=""
CHECKED=0
REFUSE=0

refuse_files() {
  REFUSE=1
  BAD_REPORT="$BAD_REPORT
  $1 is not formatted:$2
    fix:   $3
    then:  git add on those paths, and commit again"
}

unmeasured_files() {
  UNMEASURED_REPORT="$UNMEASURED_REPORT
  $1 could not be checked (NOT a formatting verdict):$2"
}

# ---- markdown, via prettier ----
#
# `--stdin-filepath` resolves .prettierrc and .prettierignore as though the
# bytes were at that path, so the staged blob is judged under the same config as
# the real file, with no temporary anywhere.
check_markdown() {
  staged_files '*.md'
  [ "${#STAGED[@]}" -gt 0 ] || return 0
  if ! command -v prettier >/dev/null 2>&1; then
    UNENFORCED="$UNENFORCED markdown(prettier)"
    return 0
  fi
  local f bad=""
  for f in "${STAGED[@]}"; do
    CHECKED=$((CHECKED + 1))
    git show ":$f" 2>/dev/null | prettier --stdin-filepath "$f" --check >/dev/null 2>&1 \
      || bad="$bad
      $f"
  done
  [ -n "$bad" ] && refuse_files "markdown" "$bad" "prettier --write <the files listed above>"
  return 0
}

# ---- elixir, via mix format ----
#
# The probe sits beside the original so `.formatter.exs` resolution is the
# file's own, including a plugin an umbrella app declares.
check_elixir() {
  staged_files '*.ex' '*.exs'
  [ "${#STAGED[@]}" -gt 0 ] || return 0
  if ! command -v mix >/dev/null 2>&1; then
    UNENFORCED="$UNENFORCED elixir(mix)"
    return 0
  fi
  local f probe bad=""
  for f in "${STAGED[@]}"; do
    CHECKED=$((CHECKED + 1))
    probe="$(dirname "$f")/.staged-check-$$.$(basename "$f")"
    PROBES="$PROBES
$probe"
    if git show ":$f" > "$probe" 2>/dev/null; then
      mix format --check-formatted "$probe" >/dev/null 2>&1 || bad="$bad
      $f"
    fi
    rm -f "$probe"
  done
  [ -n "$bad" ] && refuse_files "elixir" "$bad" "mix format <the files listed above>"
  return 0
}

# ---- rust, via rustfmt ----
#
# THE EDITION IS RESOLVED, NEVER HARDCODED, and that is a canon obligation
# rather than a nicety: 2021 and 2024 disagree about import order, so a guard
# shipped with one estate's edition baked in would refuse another estate's
# correct code. rustfmt finds a `rustfmt.toml` by searching upward from the
# file, and the probe sits beside the original, so a config that declares an
# edition needs no flag from us. Only when no config in the file's ancestry
# declares one is the crate's own edition read out of the nearest Cargo.toml and
# passed; when neither exists the verdict says rustfmt's default was used, so a
# reader is never left to assume it matched the crate.
#
# IT ANSWERS ON STDOUT AND SETS NOTHING, because the caller needs the answer in
# ITS OWN shell. The first spelling set a global from inside the function and
# read it afterwards, and the call site is a command substitution -- a subshell
# -- so the assignment could never have reached the verdict, and the verdict
# would have quietly stopped mentioning the default edition it was there to
# disclose.
#
# Three answers: `config` (a rustfmt.toml above the file declares an edition, so
# rustfmt's own upward search applies it and we pass no flag), `edition <N>`
# (none does, and the nearest Cargo.toml declares the crate's), and `default`
# (neither, so rustfmt's default applies and the verdict says so).
rust_edition_answer() {
  local dir="$1" e=""
  while :; do
    if [ -f "$dir/rustfmt.toml" ]; then
      e="$(sed -n 's/^[[:space:]]*edition[[:space:]]*=[[:space:]]*"\{0,1\}\([0-9]\{4\}\).*/\1/p' "$dir/rustfmt.toml" | head -1)"
      if [ -n "$e" ]; then
        printf 'config\n'
        return 0
      fi
    fi
    if [ -f "$dir/Cargo.toml" ]; then
      e="$(sed -n 's/^[[:space:]]*edition[[:space:]]*=[[:space:]]*"\{0,1\}\([0-9]\{4\}\).*/\1/p' "$dir/Cargo.toml" | head -1)"
      if [ -n "$e" ]; then
        printf 'edition %s\n' "$e"
        return 0
      fi
    fi
    if [ "$dir" = "." ] || [ "$dir" = "/" ]; then
      break
    fi
    dir="$(dirname "$dir")"
  done
  printf 'default\n'
}

check_rust() {
  staged_files '*.rs'
  [ "${#STAGED[@]}" -gt 0 ] || return 0
  if ! command -v rustfmt >/dev/null 2>&1; then
    UNENFORCED="$UNENFORCED rust(rustfmt)"
    return 0
  fi
  local f target probe errfile err rc edition_answer bad="" unmeasured=""
  for f in "${STAGED[@]}"; do
    CHECKED=$((CHECKED + 1))
    target=""; probe=""
    if [ -f "$f" ] && git show ":$f" 2>/dev/null | cmp -s - "$f"; then
      target="$f"
    else
      probe="$(dirname "$f")/.staged-check-$$.rs"
      PROBES="$PROBES
$probe"
      git show ":$f" > "$probe" 2>/dev/null || probe=""
      target="$probe"
    fi
    if [ -z "$target" ]; then
      unmeasured="$unmeasured
      $f: its staged blob could not be read"
      continue
    fi
    EDITION_FLAGS=()
    edition_answer="$(rust_edition_answer "$(dirname "$f")")"
    case "$edition_answer" in
      'edition '*) EDITION_FLAGS=(--edition "${edition_answer#edition }") ;;
      default)     DEFAULT_EDITION=1 ;;
    esac
    errfile="${probe:-$f}.staged-check-$$.err"
    PROBES="$PROBES
$errfile"
    # `if`, NOT `err=$(...)`: under `set -e` a failing command substitution
    # aborts before the next line, which once produced a gate that blocked a
    # commit and printed nothing at all.
    if [ "${#EDITION_FLAGS[@]}" -gt 0 ]; then
      rustfmt --check "${EDITION_FLAGS[@]}" "$target" >/dev/null 2>"$errfile" && rc=0 || rc=1
    else
      rustfmt --check "$target" >/dev/null 2>"$errfile" && rc=0 || rc=1
    fi
    err="$(cat "$errfile" 2>/dev/null || true)"
    rm -f "$errfile"
    [ -n "$probe" ] && rm -f "$probe"
    # STDERR MEANS "COULD NOT MEASURE", and the error must belong to the file
    # named beside it: a single variable printed after the loop showed the
    # operator the LAST file's stderr, which was empty whenever that file passed.
    if [ -n "$err" ]; then
      unmeasured="$unmeasured
      $f: $(printf '%s' "$err" | head -1)"
    elif [ "$rc" -ne 0 ]; then
      bad="$bad
      $f"
    fi
  done
  [ -n "$bad" ] && refuse_files "rust" "$bad" "rustfmt <the files listed above>"
  [ -n "$unmeasured" ] && unmeasured_files "rust" "$unmeasured"
  return 0
}

# A LEAKED PROBE IS NOT A STRAY TEMP FILE: beside a `tests/*.rs` original cargo
# compiles and RUNS it as a test target of its own, and the release build's
# shared-artefact guard refuses on any untracked file under a crate. The
# per-file `rm` covers the passing and refusing paths; this covers the
# interrupted one.
PROBES=""
DEFAULT_EDITION=0
#
# NEWLINE SEPARATED AND READ A LINE AT A TIME, because a repository is allowed a
# path with a space in it and the hook this came from would have split one into
# two names that remove nothing.
# shellcheck disable=SC2329 # invoked by the trap below, which shellcheck does not follow
cleanup_probes() {
  local p
  while IFS= read -r p; do
    [ -n "$p" ] && rm -f "$p"
  done <<EOF
$PROBES
EOF
}
trap cleanup_probes EXIT INT TERM

UNKNOWN=""
for fmt in $DECLARED; do
  case "$fmt" in
    markdown) check_markdown ;;
    elixir)   check_elixir ;;
    rust)     check_rust ;;
    *)        UNKNOWN="$UNKNOWN $fmt" ;;
  esac
done

# AN UNRECOGNISED NAME REFUSES rather than being ignored. A typo in the
# declaration means the check its author intended is not running, and a guard
# that passes over its own misconfiguration is the all-clear a reader believes.
if [ -n "$UNKNOWN" ]; then
  note "REFUSED -- $CONFIG declares a formatter this guard does not know:$UNKNOWN"
  note "  known: markdown, elixir, rust"
  note "  a fourth format is a change to this guard in canon, never an estate's own command"
  exit 1
fi

# ---- THE VERDICT, WHICH IS ALWAYS PRINTED ----
#
# An instrument may fail toward a warning a reader discards; it may never fail
# toward an all-clear a reader believes, and a line nobody sees is an all-clear.
# So the unenforced formats and the unmeasured files are in the verdict itself,
# not in a debug line.
if [ "$REFUSE" = 1 ]; then
  note "REFUSED, and NOTHING WAS REWRITTEN -- your staged bytes are exactly as you staged them.$BAD_REPORT"
fi
[ -n "$UNMEASURED_REPORT" ] && note "could not check some staged files, which is NOT a verdict on their formatting:$UNMEASURED_REPORT"
if [ -n "$UNENFORCED" ]; then
  note "UNENFORCED --$UNENFORCED declared and not on PATH; those staged bytes went unchecked."
fi
if [ "$REFUSE" = 0 ]; then
  if [ "$DEFAULT_EDITION" = 1 ]; then
    note "ok -- $CHECKED staged file(s) checked against $(printf '%s' "$DECLARED" | tr '\n' ' '); some Rust files were checked under rustfmt's default edition, because no rustfmt.toml or Cargo.toml above them declares one."
  else
    note "ok -- $CHECKED staged file(s) checked against $(printf '%s' "$DECLARED" | tr '\n' ' ')."
  fi
fi
exit "$REFUSE"
