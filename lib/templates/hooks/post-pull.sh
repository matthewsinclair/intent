#!/usr/bin/env bash
#
# post-pull.sh -- bring this clone's store up to the canon a pull, a branch
# checkout or a rewrite just put on disk.
# Copyright (c) 2026 Matthew Sinclair
# Licensed under the MIT License (see LICENSE file)
#
# This file is what a project gets as `.git/hooks/post-merge.intent`,
# `.git/hooks/post-checkout.intent` and `.git/hooks/post-rewrite.intent`, each
# reached from its hook's chain block. `intent claude upgrade --apply` writes
# all three from this one template.
#
# ---- WHY IT EXISTS ----
#
# The store is this machine's truth and git never carries it. With two clones
# and no daemon, a thread that `git pull` brought was unknown to every verb:
# `intent st show` refused it at rc=1, and every verb went on answering from
# the pre-pull model until someone ran a restore by hand. A daemon watching the tree ingests a pull by itself; a clone
# with no daemon had nothing that did.
#
# ---- WHAT IT RUNS, AND WHY THAT IS SAFE ON EVERY PULL ----
#
# `intent sync --apply`: this clone's plan, applied, the same verb a person
# runs. A hook has no terminal, so only the plan's QUIET steps run and nothing
# asks; every step that would ask is left and named on one line. The step that
# brings the store up to a pull is the daemon's own pass. It takes the files
# only where they say something the store did not write, and it lands only
# under the store's hold-unless-moved lock. So it is safe beside a running
# daemon (the second pass finds nothing to take) and beside a peer's write (it
# renders again rather than reverting it). It is NOT `--to-store`, the restore.
#
# The verb runs `doctor` last and exits with its verdict, so a non-zero exit
# with a `doctor:` line means the store WAS brought up to date and the estate
# has findings; only a non-zero exit without one means the verb did not run.
#
# ---- A CARRIER, NOT A SHIM, AND THAT IS DELIBERATE ----
#
# The pre-commit carrier is a shim that finds the gate's body in the install
# and execs it, because that body is hundreds of lines of policy that must
# not drift between estates. This file holds no policy of its own. Every rule
# is in the `intent` binary it calls, so the thing that could drift is the
# binary, and that is the same binary every other verb runs.
#
# ---- IT ALWAYS EXITS 0 ----
#
# A post-merge, post-checkout or post-rewrite hook cannot stop what already
# happened, and a non-zero exit only makes git print a failure over a pull that
# succeeded. So nothing here fails the operation. What it will NOT do is hide
# that it did not do its job: when `intent` is missing or the pass refuses, it
# prints ONE line saying the store was not brought up to date, and the command
# to run by hand.
#
# ---- ONE LINE PER THING A PERSON NEEDS TO KNOW, NOTHING OTHERWISE ----
#
# A pull that changed no thread or issue and left nothing is the common case
# and prints nothing. The verb's confirmation begins `ok: took` exactly when
# the store changed (`intentsvcs::sync::ingested`), it prints `left: ` naming
# the steps a person must run, and a test holds this file to those words.

set -u

_hook="$(basename "$0" .intent)"

# git passes post-checkout <previous HEAD> <new HEAD> <flag>, and the flag is 1
# for a branch checkout and 0 for `git checkout -- <file>`. A file checkout
# moves no canon a branch did not already hold.
if [ "$_hook" = "post-checkout" ] && [ "${3:-}" != "1" ]; then
  exit 0
fi

# Not an Intent project, or not one at this commit: nothing to ingest.
if [ ! -f intent/.config/config.json ]; then
  exit 0
fi

# A fresh checkout -- `git worktree add`, or a clone that carries hooks --
# passes the null object id as the previous HEAD (issue 0483). There is no
# store here yet, so the pass would BUILD one, from scratch, in what is usually
# a throwaway tree, and on a large estate that costs minutes and a store the
# size of the project's history. The first `intent` verb builds it when it is
# wanted. A branch switch inside an
# existing checkout passes a real previous HEAD and still syncs below.
case "${1:-}" in
  *[!0]* | "") ;;
  *)
    if [ "$_hook" = "post-checkout" ]; then
      echo "intent (${_hook}): a fresh checkout -- the store is built by the first \`intent\` verb, or now by \`intent sync --apply\`"
      exit 0
    fi
    ;;
esac

if ! command -v intent >/dev/null 2>&1; then
  echo "intent (${_hook}): the store was NOT brought up to date -- no \`intent\` on PATH; run \`intent sync --apply\` once it is" >&2
  exit 0
fi

_out="$(intent sync --apply 2>&1)"
_rc=$?
printf '%s\n' "$_out" | while IFS= read -r _line; do
  case "$_line" in
    "ok: took "*) echo "intent (${_hook}): ${_line#ok: }" ;;
    "left: "*) echo "intent (${_hook}): ${_line}" ;;
  esac
done
_doctor="$(printf '%s\n' "$_out" | grep '^doctor: ' | head -n 1)"
if [ "$_rc" -ne 0 ]; then
  if [ -n "$_doctor" ]; then
    echo "intent (${_hook}): ${_doctor} -- run \`intent doctor\` to read them" >&2
  else
    echo "intent (${_hook}): the store was NOT brought up to date -- \`intent sync --apply\` exited ${_rc}: $(printf '%s\n' "$_out" | head -n 1); run it by hand to read the remedy" >&2
  fi
fi
exit 0
