#!/bin/bash
# lib_mdfmt.sh -- make a generated markdown view survive the repo formatter.
#
# SOURCED, NOT EXECUTED. Ships 644, like lib_corpus.sh.
#
# THE PROBLEM, AND IT IS NOT COSMETIC. A generated view is committed, and the
# repo's markdown formatter runs on commit. If the generator emits narrow tables
# and the formatter widens them, then every regeneration produces a diff against
# the committed file -- for ever, without the underlying data having changed by
# one byte. Any drift check built on "regenerate and compare" then cries wolf on
# its first run, and the first thing anyone does with a check that cries wolf is
# switch it off. The check dies, and the drift it existed to catch walks free.
#
# So the rule is: **idempotent THROUGH the formatter, not merely through the
# renderer.** A renderer idempotent only against itself is not enough, and the
# difference is invisible until you commit and look.
#
# Found twice, independently, which is why this is a library rather than a
# second copy. gen_dispatch_table.sh hit it first and grew this awk inline;
# gen_register.sh was found to have exactly the same skew afterwards -- 232
# differing lines between the committed register and a fresh regeneration of
# the very same data. Two generators, one concern.
#
# TWO DISTINCT CAUSES, and only the first is layout:
#
#   1. LAYOUT the renderer controls -- column widths. Fixed here, by emitting
#      the formatter's own widths so there is nothing left to normalise.
#   2. MARKUP THE DATA CARRIES -- a value holding `*emphasis*` (the formatter
#      rewrites it to `_emphasis_`), or a value holding its own backticks and
#      then wrapped in more. No aligner can fix that one: the fix is to author
#      the canon in the form the formatter already agrees with, which is why
#      this file aligns tables and does NOT try to rewrite cell content.
#
# The aligner is deliberately conservative about what counts as a table row:
# a line that starts and ends with `|`. A fenced code block containing such a
# line would be reformatted, which is wrong but has not bitten -- noted here so
# the next person hits a comment rather than a mystery.

MD_ALIGNER='
  function flush(  i, j, w, out, cell, n) {
    if (rows == 0) return
    # SEPARATOR ROWS MUST NOT SET THE WIDTH. Their cells are runs of dashes
    # whose length is whatever the author happened to type, so a hand-written
    # separator wider than any real cell silently inflates the whole column --
    # and since the aligner then reproduces that width faithfully, the table is
    # stably wrong and looks deliberate.
    #
    # Found in pertest.md, on the first artefact generated after this library
    # was extracted: the repo formatter padded `class` to 41 (the widest actual
    # cell) and this padded it to 44 (a separator typed too long in the
    # preamble heredoc). Every commit would have diffed for ever, which is the
    # precise cry-wolf failure the header above says this file exists to stop.
    # The aligner had the same bug it was written to fix.
    for (i = 1; i <= rows; i++)
      if (!sep[i])
        for (j = 1; j <= cols[i]; j++)
          if (length(cellv[i, j]) > w_[j]) w_[j] = length(cellv[i, j])
    for (i = 1; i <= rows; i++) {
      out = "|"
      for (j = 1; j <= cols[i]; j++) {
        if (sep[i]) { cell = ""; while (length(cell) < w_[j]) cell = cell "-" }
        else { cell = cellv[i, j]; while (length(cell) < w_[j]) cell = cell " " }
        out = out " " cell " |"
      }
      print out
    }
    rows = 0; n = 0
    for (j in w_) delete w_[j]
  }
  /^[ \t]*\|.*\|[ \t]*$/ {
    line = $0
    sub(/^[ \t]+/, "", line); sub(/[ \t]+$/, "", line)
    sub(/^\|/, "", line); sub(/\|$/, "", line)
    # AN ESCAPED PIPE IS CONTENT, NOT A COLUMN BOUNDARY. A cell may legitimately
    # hold `\|` -- claude_with_intent.bats has a test named "invocable as intent
    # claude start|ws through the dispatch" -- and splitting on it silently adds
    # a column, shunting every later cell one place left. The row still LOOKS
    # like a table, which is what makes it nasty: the corruption is invisible
    # until something counts columns. Protect through the split, restore
    # immediately after, so widths are measured on the real text.
    gsub(/\\\|/, "\001", line)
    rows++
    n = split(line, parts, "\\|")
    cols[rows] = n
    issep = 1
    for (j = 1; j <= n; j++) {
      c = parts[j]
      gsub(/^[ \t]+|[ \t]+$/, "", c)
      gsub(/\001/, "\\|", c)
      cellv[rows, j] = c
      if (c !~ /^:?-+:?$/) issep = 0
    }
    sep[rows] = issep
    next
  }
  { flush(); print }
  END { flush() }
'

# md_align <infile> <outfile>
#
# Never in place: awk reading and writing one path truncates it before reading,
# and a generator that destroys its own output on a formatting step is a worse
# bug than the misalignment it set out to fix. Returns non-zero on failure so a
# caller can refuse rather than publish a half-rendered view.
md_align() {
  local in="$1" out="$2"
  [ -f "$in" ] || { echo "md_align: no such file: $in" >&2; return 2; }
  [ "$in" != "$out" ] || { echo "md_align: refusing to align $in onto itself -- pass a distinct output path" >&2; return 2; }
  awk "$MD_ALIGNER" "$in" > "$out"
}
