#!/bin/bash
# lib_surface.sh -- the four populations of the dispatch table.
#
# SOURCED, NOT EXECUTED. It defines functions and exits nothing; running it
# directly does nothing useful, so it ships 644 like `lib_corpus.sh` and unlike
# every other tool in this directory.
#
# WHY THIS IS SHARED RATHER THAN INLINE. `.families[].entries[]` reads like the
# command surface and is not it, in BOTH directions at once:
#
#   too narrow   it omits the 8 top-level `new_surface[]` rows -- `search`,
#                `sync`, `schema`, `export`, `ingest`, `backup`, `daemon`, `mcp`
#                -- and all 8 SHIP
#   too wide     it includes the 5 rows dispositioned `retire` -- `st organize`,
#                `organize`, `treeindex`, `help`, `st_zero` -- which do not
#                exist in the binary, so probing them measures nothing
#
# 104 against 107 is three apart with opposite signs. **No count-based sanity
# check flinches at that**, which is why the same hand-written jq produced the
# same wrong population three times in one week: in a test, in a help scan
# (issue 0037), and in the 0044 exit-code sweep.
#
# The fix is not vigilance. It is that the population has a name and one home.

# THE PROBE EXCLUSIONS, NAMED RATHER THAN GUESSED. These three ship and are
# deliberately not probeable: `daemon` and `mcp` are long-running servers and
# `claude start` launches a real session. `implemented_check.sh` already
# excludes the same three by name for the same reason -- this is that list,
# lifted to where every consumer can see it instead of being re-derived.
#
# NEWLINE-delimited, and that is not a style choice: `claude start` CONTAINS A
# SPACE, so a space-separated list word-splits into `claude` and `start` and
# would silently exclude two commands that are not in it. A path with a space is
# the normal case in this table, not the exotic one.
SURFACE_NONRETURNING='daemon
mcp
claude start'

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

# Every row the table declares, shipped or not. 112.
surface_declared() {
  local t; t="$(_surface_table)" || return 1
  jq -r '([.families[].entries[].path] + [.new_surface[]?.path])[]' "$t"
}

# Every row that exists in the binary. 107. THIS IS THE DEFAULT POPULATION for
# anything asking a question about what the tool does.
# BOTH retire predicates, deliberately. `surface_check.sh` REFUSES a row where
# `disposition` and `target.state` disagree on `retire`, so testing one today
# gives the same 107 as testing both -- and that agreement is a property of
# ANOTHER instrument's guard, not of this one. A library that is correct because
# something else refuses is a library whose precondition is written down nowhere,
# which is exactly the class measured in issue 0042 today. Test both here and the
# dependency disappears.
surface_shipped() {
  local t; t="$(_surface_table)" || return 1
  jq -r '([.families[].entries[] | select((.disposition != "retire") and (.target.state != "retire")) | .path]
          + [.new_surface[]? | select((.disposition != "retire") and (.target.state != "retire")) | .path])[]' "$t"
}

# The rows that were removed. 5. Worth its own accessor because a retired
# command is not absent from the WORLD -- v2 users still type it -- so the set
# is the population for "what happens when someone runs the old command".
surface_retired() {
  local t; t="$(_surface_table)" || return 1
  # Both homes and both predicates, so that `shipped` + `retired` == `declared`
  # by construction rather than by today's data. A row retired in `new_surface`
  # would otherwise be counted by neither and the arithmetic would not close.
  jq -r '([.families[].entries[], .new_surface[]?]
          | .[] | select((.disposition == "retire") or (.target.state == "retire")) | .path)' "$t"
}

# Shipped minus the three that do not return. 104.
#
# `grep -vx -f -` rather than a built -e list: it reads the exclusions as whole
# lines from stdin, so an entry containing a space is matched as one pattern and
# there is no word-splitting to get wrong.
surface_probeable() {
  surface_shipped | grep -vxF -f <(printf '%s\n' "$SURFACE_NONRETURNING")
}
