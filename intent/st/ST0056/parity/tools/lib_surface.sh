#!/bin/bash
# lib_surface.sh -- the four populations of the dispatch table.
#
# SOURCED, NOT EXECUTED. It defines functions and exits nothing; running it
# directly does nothing useful, so it ships 644 like `lib_corpus.sh` and unlike
# every other tool in this directory.
#
# WHY THE POPULATIONS HAVE A NAME. `.families[].entries[]` reads like the
# command surface and is not it, in BOTH directions at once:
#
#   too narrow   it omits the 8 top-level `new_surface[]` rows -- `search`,
#                `sync`, `schema`, `export`, `ingest`, `backup`, `daemon`, `mcp`
#                -- and all 8 SHIP
#   too wide     it includes the 5 rows dispositioned `retire` -- `st organize`,
#                `organize`, `treeindex`, `help`, `st_zero` -- which do not
#                exist in the binary, so probing them measures nothing
#
# 104 against 112 against 107 is three apart with opposite signs. **No
# count-based sanity check flinches at that**, which is why the same hand-written
# jq produced the same wrong population five times in one week.
#
# THIS FILE NO LONGER COMPUTES THEM, AND THE REASON IS THAT ITS FIRST VERSION
# DID. Built as the one home, it then sat beside `implemented_check.sh`'s own
# `EXCLUDED` list and `surface_check.sh`'s four inline walks, none of which
# sourced it -- so it did not reduce the number of homes, it added one, and
# 0037's write-up recorded a consolidation that had not happened. **A sourced
# library closes the class only for the callers that source it, and nothing
# makes them.**
#
# So the one home moved to where it cannot be bypassed by not sourcing it:
# `.populations` in `surface/dispatch-table.json`, generated and refused-on-skew
# by `gen_dispatch_table.sh`, with `Entry::is_shipped()` bound to it by a test in
# the Rust suite. **This file is now a READER.** Same four function names, same
# newline-delimited output, so a caller that sources it needs no change.

# Resolve the table once. Every function refuses rather than defaulting: a
# population computed from a table that is not there is a complete, uniform,
# entirely fictional surface, which is the failure this library exists to end.
_surface_table() {
  local t="${DISPATCH_TABLE:-}"
  if [ -z "$t" ]; then
    local root
    root="$(git rev-parse --show-toplevel 2>/dev/null)" || root=""
    [ -n "$root" ] && t="$root/surface/dispatch-table.json"
  fi
  if [ ! -f "$t" ]; then
    echo "error: cannot locate surface/dispatch-table.json -- set DISPATCH_TABLE" >&2
    return 1
  fi
  printf '%s\n' "$t"
}

# Read one named population out of the block.
#
# REFUSES ON AN ABSENT OR EMPTY LIST rather than returning nothing. An empty
# population and a missing one produce the same silence at a call site, and the
# caller's next step is almost always a loop -- which runs zero times and looks
# exactly like a clean sweep. Every one of these lists has members by
# construction, so empty means the block is broken, not that the surface is.
_surface_pop() {
  local name="$1" t out
  t="$(_surface_table)" || return 1
  out="$(jq -er --arg k "$name" '
    if (has("populations") | not) then error("no populations block")
    elif (.populations[$k] | type) != "array" then error("populations." + $k + " is not a list")
    elif (.populations[$k] | length) == 0 then error("populations." + $k + " is empty")
    else .populations[$k][] end' "$t" 2>&1)" || {
    echo "error: cannot read \`populations.$name\` from $t -- $out" >&2
    echo "  the block is generated; run gen_dispatch_table.sh, which refuses when it disagrees with the rows" >&2
    return 1
  }
  printf '%s\n' "$out"
}

# Every row the table declares, shipped or not. 112.
surface_declared() { _surface_pop declared; }

# Every row that exists in the binary. 107. THIS IS THE DEFAULT POPULATION for
# anything asking a question about what the tool does.
surface_shipped() { _surface_pop shipped; }

# The rows that were removed. 5. Worth its own accessor because a retired
# command is not absent from the WORLD -- v2 users still type it -- so the set
# is the population for "what happens when someone runs the old command".
surface_retired() { _surface_pop retired; }

# Shipped minus the four in `not_probed`. 103.
#
# **It was 104 an hour ago and the difference is `claude upgrade`** -- see
# `surface_not_probed` below for why it was missing. Do not reconcile this 103
# against the 103 in issue 0044: that one is a RETRACTED figure from a sweep run
# with a broken hash, and the corrected sweep probed 104 and found 1. Two
# different numbers that happen to be equal is worse than two that differ.
surface_probeable() { _surface_pop probeable; }

# The four that ship and are deliberately not probed. Paths only; each member's
# stated reason lives in the block beside it.
#
# **THE NAME IS `not_probed` AND NOT `nonreturning`, BECAUSE THE OLD NAME LOST A
# MEMBER.** This constant was `SURFACE_NONRETURNING` and carried three: `daemon`,
# `mcp`, `claude start`. The real list has four. `claude upgrade` returns
# perfectly well and installs into the operator's REAL `~/.claude`, so it is
# excluded for a completely different reason -- writes outside the sandbox, not
# never returns -- and it fell out of a list whose name was only ever entitled
# to define one of the two reasons. `claude start` survived only because it
# satisfies both readings, which is precisely why nothing looked wrong: the
# surviving two-word row made the list look complete. Latent rather than
# harmless -- `claude upgrade` is unimplemented today, and WP-07 makes it live
# against a real home directory.
#
# NEWLINE-delimited on the way out, and that is not a style choice: two of the
# four are TWO-WORD PATHS, so a space-separated list read word-wise excludes a
# `claude` family and an `upgrade` row that were never named. Reading the block
# through `jq -r '.[]'` gives whole lines for free, where a shell string would
# have to be quoted correctly by every consumer.
surface_not_probed() {
  local t; t="$(_surface_table)" || return 1
  jq -er '
    if (has("populations") | not) then error("no populations block")
    elif (.populations.not_probed | type) != "array" then error("populations.not_probed is not a list")
    elif (.populations.not_probed | length) == 0 then error("populations.not_probed is empty")
    else .populations.not_probed[].path end' "$t" || {
    echo "error: cannot read \`populations.not_probed\` from $t" >&2
    return 1
  }
}

# Kept as a variable for callers that want the list without a subshell. The name
# carries `NOT_PROBED` rather than `NONRETURNING` deliberately: a caller still
# referring to the old spelling gets an empty string and an obvious break, which
# is better than silently inheriting a list that is short by one.
SURFACE_NOT_PROBED="$(surface_not_probed 2>/dev/null)"
