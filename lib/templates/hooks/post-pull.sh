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
# asks. The step that brings the store up to a pull is the daemon's own pass.
# It takes the files only where they say something the store did not write,
# and it lands only under the store's hold-unless-moved lock. So it is safe beside a running daemon (the
# second pass finds nothing to take) and beside a peer's write (it renders
# again rather than reverting it). It is NOT `--to-store`, the restore.
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
# ---- ONE LINE WHEN IT TOOK SOMETHING, NOTHING OTHERWISE ----
#
# A pull that changed no thread or issue is the common case and prints nothing.
# The verb's confirmation begins `ok: took` exactly when the store changed
# (`intentsvcs::sync::ingested`), and a test holds this file to that word.

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

if ! command -v intent >/dev/null 2>&1; then
  echo "intent (${_hook}): the store was NOT brought up to date -- no \`intent\` on PATH; run \`intent sync --apply\` once it is" >&2
  exit 0
fi

_out="$(intent sync --apply 2>&1)"
_rc=$?
case "${_rc}:${_out}" in
  "0:ok: took "*)
    echo "intent (${_hook}): ${_out#ok: }"
    ;;
  0:*) ;;
  *)
    echo "intent (${_hook}): the store was NOT brought up to date -- \`intent sync --apply\` exited ${_rc}: $(printf '%s\n' "$_out" | head -n 1); run it by hand to read the remedy" >&2
    ;;
esac
exit 0
