# shellcheck shell=bash
#
# lib_read_only.sh -- the one way a gated parity script runs the tree's own pair
# against this checkout without migrating or making its store. SOURCED, never
# run, by a tool that sets RO_ROOT, RO_TAG and RO_WORK and then calls
# ro_require_bin and ro_json.
#
# BASH 3.2, NOT 5: the commit gate runs these tools through `#!/bin/bash` or
# `#!/usr/bin/env bash`, which on a Mac can be /bin/bash 3.2.57, so no
# associative array, mapfile or `local -n` here.
#
# WHY IT EXISTS (vc decision 132, ST0095 WP-03 AC-03.1, hv decision 79). A
# gated tool judges what the tree's own build says, so it runs the pair at
# native/rust/target/release, never `intent` on PATH. That pair opens this
# checkout's store, and a build whose store schema is newer migrates a store as
# it opens it, one way: the release installed from brew then refuses it. Every
# seat in this checkout shares that build, so one rebuild after a schema bump
# would migrate the main store at the next commit, whoever made it.
#
# So each verb is asked with --read-only, which asks the store's one
# open-or-migrate decision, `store::Opening::decide`, before it opens the store
# for write. Where opening would make the store or migrate it, the verb answers
# nothing and exits 2, and its JSON names what opening would do at
# `.store.opening`; it prints that object only for such a stop, so any word
# there is taken as the stop and no list of words is kept here. The verb then
# runs on a copy of this checkout's intent/ with no store in it, which the pair
# builds from canon as a fresh clone does, and the checkout's store is not
# opened for write, and neither migrated nor made. Otherwise the verb answered
# in place, read-only, and wrote nothing. `.cache` stays out of the copy
# because it holds the store (and the search index), which is what leaves the
# copy with no store, and `.backup` because it holds the store's backups,
# about 3.4 GB together. The copy is made once per run and every later call
# uses it.
#
# IT FORMS NO VERDICT. ro_json sets RO_OUT (the verb's stdout), RO_RC (its exit
# status) and RO_RUN (where it ran, for the caller's messages), and the caller
# reads them. Exit 2 is also any other run a verb could not answer, so only a
# report that names a store opening sends a call to the copy. The verb's own
# refusal is not replayed: its remedy, run it without --read-only, is the
# migration this file exists to prevent. Anything this file cannot do (no pair,
# no jq, no copy) refuses with exit 2 through ro_die, and is never a pass.
#
# THE CALLER'S CONTRACT. Set RO_ROOT (the checkout), RO_TAG (the prefix of every
# message) and RO_WORK (an existing scratch directory the caller removes), and
# call ro_require_bin before ro_json. Call ro_json at the top level, never inside
# `$(...)` or a pipeline, because its refusals exit and its answers are
# variables, and neither leaves a subshell. The library works under `set -u` and
# `pipefail` and sets no option of its own; it is not safe under `set -e`,
# which would stop the caller at a verb's exit 1 before the copy could run, so a
# caller does not set it. A missing input refuses with exit 2.
#
# Readers: thread_view_skew_check.sh (doctor), and the id door's callers.

# What the door answered, one line per token asked; empty until ro_ids runs, so a
# lookup before any call reads as no answer rather than an unbound variable.
RO_IDS=""

# ro_die <message> -- refuse the run: the caller cannot answer.
ro_die() {
  echo "${RO_TAG:-lib_read_only}: $*" >&2
  exit 2
}

# ro_need <name>... -- refuse unless each named input is set and not empty.
ro_need() {
  local name value
  for name in "$@"; do
    eval "value=\${$name:-}"
    [ -n "$value" ] || ro_die "lib_read_only.sh needs $name set by its caller, and it is not, so this commit is unchecked rather than clean"
  done
}

# ro_require_jq -- refuse unless jq, which reads every report, is on PATH.
ro_require_jq() {
  command -v jq >/dev/null 2>&1 ||
    ro_die "jq is not on PATH, and it is what reads the pair's report, so this commit is unchecked rather than clean"
}

# ro_enter <dir> -- refuse unless the verb can run in <dir>, so a failed cd is
# never read as the verb's own exit status.
ro_enter() {
  [ -d "$1" ] && [ -x "$1" ] ||
    ro_die "could not enter $1 to run the pair there, so this commit is unchecked rather than clean"
}

# ro_require_bin -- set RO_BIN to the tree's own pair, or refuse.
ro_require_bin() {
  ro_need RO_ROOT RO_TAG
  RO_BIN="$RO_ROOT/native/rust/target/release/intent"
  [ -x "$RO_BIN" ] && return 0
  # A TREE WITH NO SHARED BUILD JUDGES BY ITS PRIVATE ONE. `bin/devbin build all`
  # on a dirty build-input tree builds into target/private/release and promotes
  # nothing, and a worktree carrying an uncommitted Rust change is always dirty,
  # so its only pair is the private one, built from the tree being committed.
  # Taken ONLY where target/release does not exist at all: where it does, the
  # shared pair is the one to judge by, and a missing binary there is a rebuild
  # in progress, below, never a reason to judge by somebody's dirty build.
  if [ ! -d "$RO_ROOT/native/rust/target/release" ] &&
    [ -x "$RO_ROOT/native/rust/target/private/release/intent" ]; then
    RO_BIN="$RO_ROOT/native/rust/target/private/release/intent"
    echo "$RO_TAG: this tree has no shared release build, so it is judged by its private one, $RO_BIN, which \`bin/devbin build all\` built from a dirty tree." >&2
    return 0
  fi
  # REFUSES, IT DOES NOT SKIP (hv ruling 4 as scoped by vc 2026-08-27): exit 0
  # is the status a run that examined everything returns, so no caller reading
  # the exit code could tell a full check from none.
  #
  # THE TWO STATES ARE SEPARATED BECAUSE THE BLOCKED READER IS THE POINT: whoever
  # meets this must be able to tell "wait and retry" from "something is broken"
  # without opening this file.
  if [ -d "$RO_ROOT/native/rust/target/release" ]; then
    # TRANSIENT. cargo clears the artefact before writing the new one; that
    # window was measured at about 66 seconds on this estate.
    echo "$RO_TAG: REFUSING -- no v3 binary at $RO_BIN, but $RO_ROOT/native/rust/target/release exists, so a build has run here and this is almost certainly A REBUILD IN PROGRESS." >&2
    echo "  THIS IS TRANSIENT AND NOTHING IS BROKEN. The window is about 60 seconds. Wait, then retry the commit." >&2
    echo "  If it persists past a couple of minutes the pair is genuinely gone -- remedy: \`bin/devbin build all\`." >&2
  else
    echo "$RO_TAG: REFUSING -- no v3 binary at $RO_BIN, no release directory and no private build, so this tree has never been built." >&2
    echo "  THIS IS NOT TRANSIENT. Waiting will not help." >&2
    echo "  Remedy: \`bin/devbin build all\`." >&2
  fi
  echo "  Why this refuses rather than skipping: without the binary this check examines nothing, and exiting 0 would report that as a pass." >&2
  exit 2
}

# ro_copy -- make the copy of intent/ once per run, at $RO_WORK/copy.
ro_copy() {
  [ -d "$RO_WORK/copy/intent" ] && return 0
  local clone entry
  clone=(cp -R)
  [ "$(uname -s)" = Darwin ] && clone=(cp -c -R)
  mkdir -p "$RO_WORK/copy/intent" ||
    ro_die "could not make $RO_WORK/copy/intent to run on a copy in, so this commit is unchecked rather than clean"
  for entry in "$RO_ROOT"/intent/* "$RO_ROOT"/intent/.[!.]*; do
    [ -e "$entry" ] || continue
    case "${entry##*/}" in .backup | .cache) continue ;; esac
    "${clone[@]}" "$entry" "$RO_WORK/copy/intent/" ||
      ro_die "could not copy $entry to run on a copy of intent/, so this commit is unchecked rather than clean"
  done
}

# ro_on_copy <verb> [args...] -- run the verb on the copy, without --read-only.
ro_on_copy() {
  local verb="$1"
  shift
  ro_enter "$RO_WORK/copy"
  RO_OUT="$(cd "$RO_WORK/copy" && "$RO_BIN" "$verb" --format json "$@")"
  RO_RC=$?
  RO_RUN="on a copy of intent/"
}

# ro_json <verb> [args...] -- run the pair's verb read-only, or on the copy.
ro_json() {
  local verb="$1" opening holds
  ro_need RO_ROOT RO_TAG RO_WORK RO_BIN
  ro_require_jq
  if [ -d "$RO_WORK/copy/intent" ]; then
    ro_on_copy "$@"
    return 0
  fi
  shift
  ro_enter "$RO_ROOT"
  : >"$RO_WORK/read-only.err" ||
    ro_die "could not write $RO_WORK/read-only.err to keep the verb's stderr, so this commit is unchecked rather than clean"
  RO_OUT="$(cd "$RO_ROOT" && "$RO_BIN" "$verb" --read-only --format json "$@" 2>"$RO_WORK/read-only.err")"
  RO_RC=$?
  RO_RUN="in place with --read-only"
  opening=""
  [ "$RO_RC" -eq 2 ] &&
    opening="$(printf '%s' "$RO_OUT" | jq -er '.store.opening | strings' 2>/dev/null)"
  if [ -z "$opening" ]; then
    cat "$RO_WORK/read-only.err" >&2
    return 0
  fi
  # What the stop names, for the line below only: a field jq cannot read prints
  # as null, and the route is chosen by `opening` alone.
  holds="$(printf '%s' "$RO_OUT" | jq -r '"\(.store.path), its schema \(.store.schema // "none"), this build'"'"'s \(.store.build)"' 2>/dev/null)"
  ro_copy
  echo "$RO_TAG: \`$verb --read-only\` says opening this checkout's store ($holds) would $opening it, so it runs on a copy of intent/, and this checkout's store is not opened for write, and neither migrated nor made." >&2
  ro_on_copy "$verb" "$@"
}

# ro_ids <token>... -- ask the id door once for every token, read-only, and set
# RO_IDS to one line per token, `<token> <answer> <kind> <canon>`, tab-separated:
# answer is the door's (id, ambiguous, unknown, not-an-id), and kind and canon
# are the record's when the answer is id and empty otherwise. No tokens, no
# call. THE DOOR IS THE ONE READER OF AN ID'S SHAPE (ST0095 AC-12.1): a caller
# joins on these lines and spells no id pattern of its own.
#
# It refuses, never passes, when the door gives no verdict (exit above 1, eg a
# pair older than the door or than --read-only, which clap refuses with 2 and
# no report), when the report is not one answer per token in order, or when an
# answer names no kind. Exit 1 is the door's verdict that some token is not an
# id here, which is an answer, so it is read like 0.
ro_ids() {
  RO_IDS=""
  [ "$#" -gt 0 ] || return 0
  # `--` ends the options, so a token that starts with `-` is a token, never a
  # usage error misread as an old pair.
  ro_json id -- "$@"
  [ "$RO_RC" -le 1 ] ||
    ro_die "\`intent id\`, run $RO_RUN, exited $RO_RC and gave no verdict, so the ids this commit cites are unread rather than clean. A pair without \`intent id --read-only\` is older than this tool: remedy \`bin/devbin build all\`. Its report follows:
$RO_OUT"
  # Exit 1 is also a usage error, which this binary prints as one line on
  # stderr and no report: a pair older than `id --read-only` lands here, so a
  # report that is not one JSON object refuses by that name. jq reads empty
  # input as nothing, without error, so the read below cannot catch it.
  printf '%s' "$RO_OUT" | jq -e 'type == "object"' >/dev/null 2>&1 ||
    ro_die "\`intent id\`, run $RO_RUN, exited $RO_RC with no report, so the ids this commit cites are unread rather than clean. A pair without \`intent id --read-only\` is older than this tool, and refuses it this way: remedy \`bin/devbin build all\`."
  RO_IDS="$(printf '%s' "$RO_OUT" | jq -r --argjson n "$#" '
    if (.answers | type) != "array" then error("the report carries no answers list")
    elif (.answers | length) != $n then error("the report answers \(.answers | length) of \($n) token(s)")
    else .answers[] end
    | if (.token | type) != "string" or (.answer | type) != "string" then error("an answer names no token or verdict") else . end
    | if .answer == "id" and ((.kind | type) != "string" or (.canon | type) != "string") then error("an id answer names no kind or canon") else . end
    | [.token, .answer, (if .answer == "id" then .kind else "" end), (if .answer == "id" then .canon else "" end)] | @tsv
  ')" || ro_die "could not read \`intent id --format json\`, run $RO_RUN (jq's reason is above), so the ids this commit cites are unread rather than clean"
  # The order is the door's, one answer per token as asked; a token the door
  # echoed back changed would join to nothing, so it is checked too.
  [ "$(printf '%s\n' "$RO_IDS" | cut -f1)" = "$(printf '%s\n' "$@")" ] ||
    ro_die "\`intent id\` answered for tokens other than those asked, so the ids this commit cites are unread rather than clean"
  # AMBIGUOUS IS THE DOOR DECLINING, NOT SAYING NO: the token names more than one
  # record here, so whether it is a thread or an issue is unanswered, and a
  # caller that read it as "not one" would pass a row over in silence.
  local amb
  amb="$(printf '%s\n' "$RO_IDS" | awk -F '\t' '$2 == "ambiguous" { printf "%s%s", sep, $1; sep = ", " }')"
  [ -z "$amb" ] ||
    ro_die "\`intent id\` finds more than one record for: $amb. The door cannot say which this commit cites, so it is unread rather than clean; cite each by enough of its id to name one record."
}

# ro_id_is <token> <kind> -- true when the door answered id of <kind> for <token>.
ro_id_is() {
  printf '%s\n' "$RO_IDS" | awk -F '\t' -v t="$1" -v k="$2" '$1 == t && $2 == "id" && $3 == k { f = 1 } END { exit !f }'
}

# ro_id_canon <token> -- print the canon file the door named for <token>.
ro_id_canon() {
  printf '%s\n' "$RO_IDS" | awk -F '\t' -v t="$1" '$1 == t && $2 == "id" { print $4; exit }'
}

# ro_id_answer <token> -- print the door's answer for <token> (id, unknown,
# not-an-id), for a caller that names what the door did not confirm.
ro_id_answer() {
  printf '%s\n' "$RO_IDS" | awk -F '\t' -v t="$1" '$1 == t { print $2; exit }'
}

# ro_st_tokens <path> -- THE ONE READING of a path under `intent/st/` for the
# door's callers: what to ask the door about it, one `<token> <kind>` line,
# tab-separated, per question. `intent/st/<dir>/<file>` asks whether `<dir>` is
# a thread; `intent/st/<dir>/WP/<package>/info.md` also asks whether
# `<dir>/<package>` is a work package. Any other path asks nothing. A caller
# builds its tokens with this and looks each path up with it, so the two halves
# cannot read one path two ways.
ro_st_tokens() {
  local rest dir file pkg
  rest="${1#intent/st/}"
  [ "$rest" != "$1" ] || return 0
  dir="${rest%%/*}"
  file="${rest#*/}"
  [ -n "$dir" ] && [ -n "$file" ] && [ "$file" != "$rest" ] || return 0
  printf '%s\tthread\n' "$dir"
  pkg="${file#WP/}"
  if [ "$pkg" != "$file" ] && [ "${pkg#*/}" = info.md ]; then
    printf '%s/%s\twp\n' "$dir" "${pkg%%/*}"
  fi
  return 0
}

# ro_ids_for <path>... -- ro_ids over every token ro_st_tokens reads from the
# paths, each asked once.
ro_ids_for() {
  local p tokens
  tokens=()
  while IFS=$'\t' read -r t _; do
    [ -n "$t" ] && tokens+=("$t")
  done <<<"$(for p in "$@"; do ro_st_tokens "$p"; done | awk -F '\t' '!seen[$1]++')"
  [ "${#tokens[@]}" -gt 0 ] || return 0
  ro_require_bin
  ro_ids "${tokens[@]}"
}
