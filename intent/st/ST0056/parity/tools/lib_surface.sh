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

# ==========================================================================
# READING THE REGISTER AT A REVISION
# ==========================================================================
#
# **THE REGISTER IN YOUR WORKING TREE DESCRIBES THE TIP, AND DOCS ARE WRITTEN
# FOR A RELEASE.** Measured 2026-08-29: `ac edit` and `at edit` sit in
# `populations.shipped` at HEAD and in no published tag, so an artefact built
# from the working copy describes verbs the shipped tool does not have.
# `DISPATCH_TABLE` is the seam that fixes it -- every function above reads
# whatever it names -- and these two functions are how a caller aims it at a
# commit instead of at the tip.
#
# THEY LIVE HERE BECAUSE THE SECOND CALLER ARRIVED. `gen_cut_surface.sh` grew
# them inline; `gen_reference.sh` needs the same two, and both carry a trap that
# was paid for once and must not be re-learned. A copy in each generator would
# be two homes for a mechanism whose whole value is that it refuses correctly.

SURFACE_TABLE_PATH="${SURFACE_TABLE_PATH:-surface/dispatch-table.json}"

# Resolve a revision to a full sha, REFUSING rather than defaulting.
#
# A rev that does not resolve must never fall through to the working tree: that
# emits a confident artefact about a revision nobody named, which is the shape
# of every fabricated zero on this board.
surface_resolve_rev() {
  local rev="$1" root sha
  root="$(git rev-parse --show-toplevel 2>/dev/null)" || root="."
  if ! sha="$(git -C "$root" rev-parse --verify --quiet "${rev}^{commit}")"; then
    echo "error: \`${rev}\` does not resolve to a commit in $root" >&2
    return 1
  fi
  printf '%s\n' "$sha"
}

# Extract the register AT a revision, into a destination file.
#
# **BRACES AROUND THE VARIABLE ARE NOT STYLE.** Unbraced, zsh reads `$sha:s...`
# as a history modifier, `git show` receives a mangled ref, and the result is an
# EMPTY stream that every downstream `grep -c` and `wc -l` reports as 0 -- a
# fabricated zero indistinguishable from a real absence. Measured on this board.
#
# The empty check below is the second half of the same guard: `git show` can
# succeed on a path that exists and is empty, and an empty table produces a
# complete, uniform, entirely fictional surface rather than an error.
surface_table_at() {
  local sha="$1" dest="$2" root
  root="$(git rev-parse --show-toplevel 2>/dev/null)" || root="."
  if ! git -C "$root" show "${sha}:${SURFACE_TABLE_PATH}" > "$dest" 2>/dev/null; then
    echo "error: ${SURFACE_TABLE_PATH} does not exist at ${sha}" >&2
    return 1
  fi
  if [ ! -s "$dest" ]; then
    echo "error: ${SURFACE_TABLE_PATH} at ${sha} is empty" >&2
    return 1
  fi
}

# ==========================================================================
# BELOW THE COMMAND: ITS VERBS AND ITS FLAGS
# ==========================================================================
#
# **A COMMAND THAT SHIPS AT TWO REVISIONS CAN STILL HAVE CHANGED BETWEEN THEM.**
# The shipped population is paths, so a diff of it is silent on a verb added to a
# command's verb slot and on a flag added to a command. Issue 0702: the 3.4.0
# reference called its surface identical to v3.3.3 while issue 0700 had added
# `claude rules contract`. These two readers are the one answer both generators
# take to "what sits below each shipped command": tab-separated (command, verb)
# and (command, flag) pairs, sorted, restricted to the shipped population of
# whatever table DISPATCH_TABLE names.

# (command, verb) pairs: each value of a shipped row's verb slot, the
# `subcommand` arg that lists `values` (eg `claude rules`: `list`, `show`,
# `validate`, `contract`). A family's valueless slot names no verb here: its
# verbs are rows of their own, already in the shipped population.
surface_verbs() {
  local t shipped out
  t="$(_surface_table)" || return 1
  shipped="$(surface_shipped)" || return 1
  out="$(jq -r '[.. | objects | select(has("args") and has("path")) | .path as $p
    | (.args[]? | select(.type == "subcommand") | .values[]?)
    | "\($p)\t\(.)"] | unique | .[]' "$t")" || return 1
  printf '%s\n' "$out" | awk -F'\t' 'NR==FNR{s[$0];next} $1 in s' <(printf '%s\n' "$shipped") - | LC_ALL=C sort
}

# (command, flag) pairs, moved here from gen_cut_surface.sh so the reference
# index reads the same pairs the cut surface does.
#
# **THE FLAG DELTA IS A SEPARATE QUESTION FROM THE COMMAND DELTA AND WAS THE
# ONE ACTUALLY BEING ASKED.** The command diff answers "what commands are new";
# it is silent on a flag added to a command that already shipped, which is
# exactly the shape of the gap that sent `--note` to hv as an open scope item.
# Measured 2026-08-31: 13 new commands and 28 new flags, so reporting only the
# first understates the delivery by more than half.
#
# TWO NARROWINGS, BOTH LOAD-BEARING, BOTH MEASURED RATHER THAN ASSUMED:
#
#   RESTRICTED TO `shipped`  A row dispositioned `retire` STILL CARRIES ITS
#                            FLAGS in the register. Unrestricted, `fileindex`'s
#                            8 flags and `st repair --write` read as present at
#                            HEAD, and the removal count came out as 1 when the
#                            true figure is 10. The restriction changed the
#                            REMOVED set and left the NEW set byte-identical --
#                            checked, not assumed, because a narrowing that
#                            moves a number you are not watching is the
#                            denominator attack.
#
#   INTRINSIC EXCLUDED       clap supplies `--help`/`-h` on every command, so
#                            they appear and disappear with the command and
#                            carry no delta signal of their own. The register
#                            already dispositions them `intrinsic`, so this
#                            reads a DECLARED field rather than matching on the
#                            spelling -- the inference-from-name that EXP-05
#                            exists to replace.
#
# KNOWN IMPRECISION, STATED BECAUSE IT CANNOT BE FIXED HERE: pairs are keyed on
# (path, spelling) and `organize` is two different commands sharing one path,
# so their flags union. It is the same duplicate-spelling trap the cut
# surface's closing section names for commands, and it is the reason this keys
# on the pair rather than reporting a bare per-command count.
surface_flags() {
  local t shipped out
  t="$(_surface_table)" || return 1
  shipped="$(surface_shipped)" || return 1
  out="$(jq -r '[.. | objects | select(has("flags") and has("path")) | .path as $p
    | (.flags[]? | select((.disposition // "") != "intrinsic") | .spellings[]? | select(startswith("--")))
    | "\($p)\t\(.)"] | unique | .[]' "$t")" || return 1
  printf '%s\n' "$out" | awk -F'\t' 'NR==FNR{s[$0];next} $1 in s' <(printf '%s\n' "$shipped") - | LC_ALL=C sort
}
