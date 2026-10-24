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
# AN ESTATE THAT DECLARES NOTHING ANSWERS 3, NOT 0 (vc, 2026-09-22; issue
# 0506). The first spelling exited 0 without a word when the key was absent,
# which is the all-clear a reader believes: the roster counted the guard as
# having RUN. Exit 3 puts it in the runner's `skipped (not applicable)` tally
# instead. The guard itself prints nothing on any of its three exit-3 paths: no
# config, no `formatters` key, an empty list.
#
# A declaration that cannot be READ is a third fact and it blocks, because a
# tracked declaration the gate cannot parse is a broken tree, and a commit
# passing over it is the silent loss this guard exists to end.
#
# A missing CONFIG means the guard was dispatched by something other than the
# roster, whose applies-when names that file.
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
  local f rc err bad="" unmeasured=""
  for f in "${STAGED[@]}"; do
    CHECKED=$((CHECKED + 1))
    # prettier's exit code says which: 1 is "not formatted", 2 is
    # an error -- a config it cannot read, a file it cannot parse -- and an
    # error is not a verdict on the file's formatting.
    err="$(git show ":$f" 2>/dev/null | prettier --stdin-filepath "$f" --check 2>&1 >/dev/null)" && rc=0 || rc=$?
    if [ "$rc" -eq 1 ]; then
      bad="$bad
      $f"
    elif [ "$rc" -ne 0 ]; then
      CHECKED=$((CHECKED - 1))
      unmeasured="$unmeasured
      $f: $(printf '%s' "$err" | head -1)"
    fi
  done
  [ -n "$bad" ] && refuse_files "markdown" "$bad" "prettier --write <the files listed above>"
  [ -n "$unmeasured" ] && unmeasured_files "markdown" "$unmeasured"
  return 0
}

# ---- elixir, via mix format ----
#
# WHICH STAGED FILES ARE ELIXIR'S IS THE PROJECT'S OWN ANSWER, READ FROM ITS
# .formatter.exs, AND NEVER A LIST HERE. A Phoenix project's `inputs` name
# `*.{heex,ex,exs}` and its HTMLFormatter plugin formats the templates, so a
# check on `.ex` and `.exs` alone passed every unformatted template, and the
# project's own `mix format --check-formatted` refused it afterwards. An
# umbrella's root names few inputs and hands each app its own .formatter.exs
# through `subdirectories`, so those are read as well. A project with no
# .formatter.exs is checked on `.ex` and `.exs`, mix's own default.
#
# THE READ IS OF THE TEXT, NOT AN EVALUATION, because evaluating the file would
# boot Elixir on every commit, whether or not it stages any. It takes the
# extension each quoted glob names in its last path segment, so it cannot see a
# brace group holding a `/`, or inputs computed from a variable. A file it reads
# no extension from REFUSES, naming the fix: a formatter config the gate cannot
# read is a broken tree, and a guess would pass the very files it formats.

# The quoted strings inside the list that follows `$2:` in the .formatter.exs at
# $1, one per line. The list may span lines, and it ends at its first `]`
# outside a string, so a character class inside a glob does not end it.
formatter_list() {
  awk -v key="$2:" '
    {
      if (!on) {
        i = index($0, key)
        if (i == 0) next
        s = substr($0, i + length(key))
        on = 1
      } else {
        s = $0
      }
      inq = 0
      for (k = 1; k <= length(s); k++) {
        c = substr(s, k, 1)
        if (inq) {
          if (c == "\"") { print cur; inq = 0 } else { cur = cur c }
        } else if (c == "\"") {
          inq = 1
          cur = ""
        } else if (c == "]") {
          exit
        }
      }
    }' "$1" 2>/dev/null
}

# The extensions one glob names, from its last path segment: the brace group
# after its last dot, or the text after that dot. Anything but a plain name is
# dropped, so a `*.*` cannot widen the selection to every staged file.
glob_exts() {
  local seg="${1##*/}" tail e
  case "$seg" in
    *.*) tail="${seg##*.}" ;;
    *) return 0 ;;
  esac
  tail="${tail//\{/}"
  tail="${tail//\}/}"
  local IFS=,
  # shellcheck disable=SC2086 # split on purpose, at the commas IFS names
  for e in $tail; do
    case "$e" in
      '' | *[!A-Za-z0-9_]*) ;;
      *) printf '%s\n' "$e" ;;
    esac
  done
}

# The extensions the .formatter.exs in directory $1 names in its inputs, with
# those of each subdirectory it hands a .formatter.exs of its own, to $2 levels
# below it.
elixir_exts() {
  local dir="$1" depth="$2" g sub
  [ -f "$dir/.formatter.exs" ] || return 0
  while IFS= read -r g; do
    glob_exts "$g"
  done < <(formatter_list "$dir/.formatter.exs" inputs)
  [ "$depth" -gt 0 ] || return 0
  while IFS= read -r g; do
    # shellcheck disable=SC2086 # unquoted on purpose: the glob is mix's own
    for sub in $dir/$g; do
      [ -f "$sub/.formatter.exs" ] && elixir_exts "$sub" $((depth - 1))
    done
  done < <(formatter_list "$dir/.formatter.exs" subdirectories)
}

# ONE `mix format --check-formatted` OVER EVERY STAGED PROBE, because each call
# boots the VM, and a commit staging many templates paid one boot per file. Each
# probe sits beside its original, so `.formatter.exs` resolution is the file's
# own, including a plugin an umbrella app declares, and its leading dot keeps
# mix's compiler from reading it as a source file.
#
# mix names each unformatted file by its ABSOLUTE path, on a line of its own
# inside its colour codes, so a probe is matched by its whole path and never by
# its name, which two directories can share. An error in any one file ends the
# call before mix lists anything, so a call that ends in anything but that
# listing is asked again a file at a time: the error then belongs to the file
# that caused it, and every other file still gets a verdict of its own. mix
# exits 1 for an unformatted file AND for one it cannot parse, so only its own
# `--check-formatted` line names the unformatted case; anything else is an
# error, reported as not checked, with its first line.
check_elixir() {
  local exts e
  local -a patterns=()
  if [ -f .formatter.exs ]; then
    exts="$(elixir_exts . 3 | LC_ALL=C sort -u)"
    if [ -z "$exts" ]; then
      note "REFUSED -- .formatter.exs names no file extension this guard can read in its inputs, so which staged files mix formats is unknown."
      note "  fix:   write its inputs as literal globs, eg \"*.{heex,ex,exs}\" and \"{config,lib,test}/**/*.{heex,ex,exs}\""
      exit 1
    fi
  else
    exts="$(printf 'ex\nexs\n')"
  fi
  while IFS= read -r e; do
    patterns+=("*.$e")
  done <<EOF
$exts
EOF
  staged_files "${patterns[@]}"
  [ "${#STAGED[@]}" -gt 0 ] || return 0
  if ! command -v mix >/dev/null 2>&1; then
    UNENFORCED="$UNENFORCED elixir(mix)"
    return 0
  fi
  local f probe out err i top esc nl='
' bad="" unmeasured="" listed=0
  local -a originals=() probes=()
  for f in "${STAGED[@]}"; do
    case "$f" in
      */*) probe="$(dirname "$f")/.staged-check-$$.$(basename "$f")" ;;
      *) probe=".staged-check-$$.$f" ;;
    esac
    PROBES="$PROBES
$probe"
    if git show ":$f" > "$probe" 2>/dev/null; then
      originals+=("$f")
      probes+=("$probe")
    else
      rm -f "$probe"
      unmeasured="$unmeasured
      $f: its staged blob could not be read"
    fi
  done
  if [ "${#probes[@]}" -gt 0 ]; then
    if out="$(mix format --check-formatted "${probes[@]}" 2>&1 >/dev/null)"; then
      CHECKED=$((CHECKED + ${#probes[@]}))
    else
      case "$out" in
        *--check-formatted*)
          top="$(pwd -P)"
          esc="$(printf '\033')"
          out="$(printf '%s\n' "$out" | sed "s/${esc}\[[0-9;]*m//g")"
          for i in "${!probes[@]}"; do
            case "$nl$out$nl" in
              *"$nl$top/${probes[$i]}$nl"*)
                bad="$bad
      ${originals[$i]}"
                listed=1
                ;;
            esac
          done
          ;;
      esac
      if [ "$listed" = 1 ]; then
        CHECKED=$((CHECKED + ${#probes[@]}))
      else
        for i in "${!probes[@]}"; do
          if err="$(mix format --check-formatted "${probes[$i]}" 2>&1 >/dev/null)"; then
            CHECKED=$((CHECKED + 1))
          else
            case "$err" in
              *--check-formatted*)
                CHECKED=$((CHECKED + 1))
                bad="$bad
      ${originals[$i]}"
                ;;
              *)
                err="${err//$(basename "${probes[$i]}")/$(basename "${originals[$i]}")}"
                unmeasured="$unmeasured
      ${originals[$i]}: $(printf '%s' "$err" | grep -v '^mix format failed for file' | head -1)"
                ;;
            esac
          fi
        done
      fi
    fi
    for i in "${!probes[@]}"; do
      rm -f "${probes[$i]}"
    done
  fi
  [ -n "$bad" ] && refuse_files "elixir" "$bad" "mix format <the files listed above>"
  [ -n "$unmeasured" ] && unmeasured_files "elixir" "$unmeasured"
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
      # Not checked, so not counted in the `ok --` line's checked files.
      CHECKED=$((CHECKED - 1))
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
