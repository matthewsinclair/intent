#!/usr/bin/env bash
#
# pre-commit-shim.sh -- locate the real Intent gate and hand the process to it.
# Copyright (c) 2026 Matthew Sinclair
# Licensed under the MIT License (see LICENSE file)
#
# This file is what a project gets as `.git/hooks/pre-commit.intent`. It is
# deliberately the ONLY Intent-owned bytes in a consumer repository: it finds
# the gate and execs it, and every decision about what a gate does lives in the
# one body it execs.
#
# ---- WHY A SHIM AND NOT A COPY (hv ruling 1, 2026-08-27, `1d0ce157`) ----
#
# The gate used to be COPIED into each project. A copy is a fork with a
# distribution mechanism: seventeen estates ran three different generations of
# this file simultaneously, and nothing anywhere could say so. A fix landed in
# the source reached a project only when somebody re-ran an upgrade there, and
# "landed" and "in effect" drifted apart with no symptom.
#
# **THE ARGUMENT THAT ACTUALLY SETTLED IT WAS NOT DRIFT.** It was that the
# ROUTE moved. On 2026-08-27 the answer to "where does a fix land to reach the
# fleet" changed three times in one working day -- a frozen v2 checkout, then
# the development tree, then a Homebrew Cellar -- and none of the three
# announced itself. Copies drift slowly and visibly; a moving route re-points
# every consumer at once, silently, while people are delivering along it.
#
# ---- HOW THE ROOT IS FOUND, AND WHAT IS DELIBERATELY NOT TRIED ----
#
# One line in `~/.intent/home`, written by the installer from `install::home()`.
# The source of the answer publishes its own cache; nothing else computes it.
#
# **`$INTENT_HOME` IS NOT READ FROM THE ENVIRONMENT, AND THAT IS NOT AN
# OVERSIGHT.** `install.rs` refuses the environment deliberately: a stale
# `INTENT_HOME` exported in somebody's shell would make a v3 binary exec v2's
# scripts. A shim that reintroduced the variable would hand back exactly what
# the binary refuses, one layer down, where nothing tests it.
#
# **THERE IS NO FALLBACK CHAIN.** Not "try the file, then ask the binary", not
# "try the file, then guess from $0". A fallback is how a wrong-but-plausible
# root gets used in silence, which is the failure this file exists to remove --
# and both propagation classes that produced this design arrived that way. When
# the pointer cannot be trusted, the answer is a refusal naming what was found,
# never a second guess.
#
# **AND A STALE POINTER IS NEVER AUTO-REPAIRED.** Rewriting it here would mean
# this shim deciding what the install root is, which is the one thing it must
# not do -- the installer owns that, and a self-healing pointer hides the
# incomplete install that the operator needs to know about.
#
# ---- REFUSING, NOT SKIPPING (hv ruling 4, same record) ----
#
# Both failure modes exit non-zero and name themselves. A gate that cannot
# locate what it needs REFUSES; it does not skip. A skip is indistinguishable
# from a pass to everything downstream, which is how an unenforced fleet looks
# healthy in every log anyone reads.

set -u

_home_file="${HOME}/.intent/home"
_self="pre-commit (intent shim)"

# `--where` answers what this shim resolved and exits, without running a gate.
# **vc's ask, 2026-08-27, and it is a diagnosis tool rather than a nicety:**
# every routing question this week turned on WHICH COPY IS ACTUALLY RUNNING,
# and answering it took three nodes and a census of seventeen estates. A gate
# whose provenance is one flag away is a gate nobody has to census.
_where() {
  echo "pointer:  ${_home_file}"
  if [ ! -f "$_home_file" ]; then
    echo "state:    ABSENT"
    return 1
  fi
  _r="$(head -n 1 "$_home_file" 2>/dev/null || true)"
  echo "root:     ${_r:-<empty>}"
  if [ -n "$_r" ] && [ -d "$_r/lib/templates" ]; then
    echo "state:    OK"
    echo "gate:     ${_r}/lib/templates/hooks/pre-commit.sh"
    return 0
  fi
  echo "state:    UNUSABLE (no lib/templates under that root)"
  return 1
}

if [ "${1:-}" = "--where" ]; then
  _where
  exit $?
fi

# ---- FAILURE 1: the pointer is absent or empty ----
#
# Absent and empty are ONE state on purpose. Both mean the installer did not
# finish, both have the same remedy, and splitting them would offer the reader
# a distinction with no consequence attached to it.
if [ ! -f "$_home_file" ] || [ ! -s "$_home_file" ]; then
  echo "${_self}: cannot locate the Intent install." >&2
  echo "  ${_home_file} is $( [ -f "$_home_file" ] && echo 'empty' || echo 'absent' )." >&2
  echo "  that file is written by the installer -- its absence means the install" >&2
  echo "  never completed, not that Intent is missing." >&2
  echo "  refusing rather than skipping: a declared gate that cannot run is a failure." >&2
  echo "  remedy: reinstall Intent, then re-commit." >&2
  echo "  to bypass this one commit (use sparingly): git commit --no-verify" >&2
  exit 1
fi

_root="$(head -n 1 "$_home_file" 2>/dev/null || true)"

# ---- FAILURE 2: the pointer resolves somewhere that is not an install ----
#
# `lib/templates` is the marker `install.rs` itself uses, so the shim and the
# binary agree on what an install IS by construction rather than by two
# definitions kept in step by hand.
#
# **THE POINTED-AT PATH IS QUOTED BACK.** The whole afternoon that produced
# this file was made of a pointer that resolved to a plausible WRONG tree, and
# "cannot find the install" without saying where it looked sends the reader to
# reinstall when the fault is a stale line in a file.
if [ -z "$_root" ] || [ ! -d "$_root/lib/templates" ]; then
  echo "${_self}: the recorded Intent install root is not an install." >&2
  echo "  ${_home_file} points at: ${_root:-<empty>}" >&2
  echo "  no lib/templates/ there -- the install was moved, renamed or deleted." >&2
  echo "  NOT repairing it here: the installer owns that pointer, and a shim that" >&2
  echo "  rewrote it would be choosing an install root on your behalf." >&2
  echo "  refusing rather than skipping: a declared gate that cannot run is a failure." >&2
  echo "  remedy: reinstall Intent so the pointer is rewritten, then re-commit." >&2
  echo "  to bypass this one commit (use sparingly): git commit --no-verify" >&2
  exit 1
fi

_gate="${_root}/lib/templates/hooks/pre-commit.sh"

# ---- FAILURE 3: the root is an install and the gate is not in it ----
#
# Distinct from failure 2 and worth its own message: the root IS an install, so
# "reinstall" is the wrong first move -- what is broken is that install's
# contents, and naming the missing file is what lets someone see that.
if [ ! -f "$_gate" ]; then
  echo "${_self}: the Intent install has no pre-commit gate." >&2
  echo "  root:    ${_root}" >&2
  echo "  missing: ${_gate}" >&2
  echo "  the root is an install (it has lib/templates/), so this is an incomplete" >&2
  echo "  or partially-removed one rather than a wrong pointer." >&2
  echo "  refusing rather than skipping: a declared gate that cannot run is a failure." >&2
  echo "  remedy: reinstall Intent, then re-commit." >&2
  echo "  to bypass this one commit (use sparingly): git commit --no-verify" >&2
  exit 1
fi

# `exec` rather than a call: one process, and the gate's exit status is this
# hook's exit status with nothing in between to reinterpret it. A wrapper that
# forwarded `$?` by hand is one `set -e` interaction away from turning a
# refusal into a pass.
exec bash "$_gate" "$@"
