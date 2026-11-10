# shellcheck shell=bash
#
# lib_citation.sh -- the shell guards' one reading of an AT row's citation (issue
# 0670). SOURCED, never run.
#
# A row cites `<file>` or `<file>:<line>`, the line of its own arm. The Rust
# side reads that through `intentsvcs::contract::citation`; the guards here read
# canon and views with jq and awk and cannot call it, so they share this one
# pattern instead of each spelling its own: a `:` followed by a positive decimal
# with no leading zero, at the end, which `citation` reads the same way.
# Taking it off leaves the file a guard checks for existence or reads.
#
# Readers: absent_at_check.sh, declared_kind_check.sh, instrument_reach_census.sh
# (jq, `sub($line_re; "")` with `--arg line_re`) and stale_at_check.sh (awk,
# `-v line_re=`).
# shellcheck disable=SC2034 # read by the guards that source this file
CITATION_LINE_RE=':[1-9][0-9]*$'
