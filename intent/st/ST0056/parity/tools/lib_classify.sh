#!/bin/bash
# lib_classify.sh -- WHY does this thing never reach the CLI?
#
# SOURCED, NOT EXECUTED. Ships 644, like lib_corpus.sh and lib_mdfmt.sh.
#
# A burn of zero is a measurement: nothing changed when the binary was
# redirected. It does not say why, and the why is what decides the class -- a
# file that drives `bin/intent_<sub>` directly needs a semantic rewrite for v3,
# a file that sources a shell library dies with the shell, and a file that
# pins this repository's own content is not a conformance test at all. Three
# different futures behind one number.
#
# THIS LIVES HERE BECAUSE THE SAME JUDGEMENT IS NEEDED AT TWO GRANULARITIES.
# gen_register.sh applies it per FILE. gen_pertest.sh applies it per TEST, to
# the non-burning half of a `pending` file. The rules must be identical or the
# per-test rows would contradict the file row they are meant to resolve -- and
# two copies of a rule set that must agree is the drift lib_corpus.sh was
# written to catch, discovered the hard way in this very directory.
#
# EVERY RULE HERE IS A GREP, AND A GREP CANNOT TELL CODE FROM DATA. A file
# holding a call-site pattern as a test FIXTURE matches the rule for making
# that call. `intent_bin_retarget_guard.bats` is the live case: it greps the
# estate for `bin/intent_<sub>` spellings and holds them as literal needles, so
# the sub-script rule fires on a file that invokes nothing. That is why callers
# carry an OVERRIDES table and consult it FIRST -- a classification the machine
# cannot make is stated by a human, never guessed. Loosening the rule until it
# stops noticing would blind it to the real sites, which is the worse failure:
# a wrong `retire` is coverage that vanishes at the cut with nobody watching.

# classify_no_burn <path>
#
# Emits `<class>|<basis>|<note>` for a file whose content never reaches the CLI.
# Takes a PATH rather than text so it works unchanged for a whole `.bats` file
# and for a single extracted test body written to a temp file -- the caller
# chooses the granularity, the rules do not change.
classify_no_burn() {
  local f="$1"
  [ -f "$f" ] || { echo "UNCLASSIFIED|unreadable|classify_no_burn could not read $f, so no rule was applied. A missing input is not a clean classification."; return 0; }

  # THIS NEEDLE IS NARROWER THAN THE ESTATE AND STAYS THAT WAY DELIBERATELY.
  # It matches one spelling, `${INTENT_BIN_DIR}/intent_<sub>`, and finds 5 files.
  # A needle covering the other variable forms (`${INTENT_PROJECT_ROOT}/bin/`,
  # `$INTENT_HOME/bin/`) finds 18. Measured 2026-08-15, so the gap is known
  # rather than assumed.
  #
  # It causes no misclassification today, and the reason is worth writing down
  # because it is luck with a structure. Thirteen of the eighteen have NON-ZERO
  # burn, so this ladder never runs on them at all -- they are keep or pending
  # on the burn measurement alone. Of the five that reach here, every one lands
  # in the right class by a different rule: two `intent_migrations_*` and
  # `helpers.bats` source a library (retire), `intent_bin_retarget_guard` is a
  # named override, and `treeindex_commands` matches this needle directly.
  #
  # AND WIDENING IT WOULD BREAK A ROW THAT IS CURRENTLY RIGHT. `release_sidecars`
  # matches the broad form only via `run grep -F 'stamp_project_version'
  # "${INTENT_HOME}/bin/intent_upgrade"` -- it reads the script as DATA and
  # invokes nothing. A wider needle classifies it `deviate` (semantic rewrite
  # needed) when `out-of-scope` is correct. Same trap as the guard's own
  # allowlist, one directory over: a grep cannot tell a call site from a file
  # being read, and this rule runs FIRST, so a false positive here steals the
  # row from every rule that would have got it right.
  #
  # So the limit is: a NEW zero-burn file that invokes a sub-script through one
  # of the unmatched spellings would fall through to `out-of-scope` and read as
  # "not a conformance test". Nothing currently does. `gen_pertest.sh --verify`
  # is what notices if that changes at test granularity; at file granularity the
  # register is regenerated often enough that the row would be seen.
  if grep -qE '\$\{INTENT_BIN_DIR\}/intent_[a-z_]+' "$f"; then
    echo "deviate|sub-script entry point|Invokes bin/intent_<sub> directly, bypassing the dispatcher (PROJECT_ROOT resolution, INTENT_ORIG_CWD, cd to root -- bin/intent:198-218). No equivalent under one binary; needs a semantic rewrite, not a path swap."
  # THE QUOTE IS NOT ALWAYS A DOUBLE QUOTE, and hardcoding it cost a real
  # misclassification. This needle read `source "$VAR/bin/intent...` with a
  # literal `"`, which misses every site that sources from inside a
  # `bash -c "..."` -- where the inner quote must be a SINGLE quote or the
  # string ends. That is not an exotic spelling; it is the ordinary way to run a
  # shell function in a clean subshell, and it is how `helpers.bats` writes all
  # 11 of its call sites.
  #
  # The consequence was worse than a wrong label. `helpers.bats` fell through to
  # the final rule and was classified `out-of-scope` -- "never invokes the CLI,
  # pins this repository's own content, survives a binary swap untouched." Every
  # clause of that is false: it sources a bash library and calls its functions,
  # so it dies with the shell. 17 tests were sitting in the class that means
  # "not part of the parity contract" when they belong in the class that means
  # "there is no binary to retarget".
  #
  # Widened to either quote or none. Verified against the whole estate before
  # landing: four files newly match, all four are genuine
  # `source <lib>; call_function` sites, and nothing that matched before stopped
  # matching.
  elif grep -qE $'source [\'"]?\\$\\{?(INTENT_PROJECT_ROOT|INTENT_HOME)\\}?/bin/intent|source .*(rules_lib|critic_runner)\\.sh' "$f"; then
    echo "retire|shell-function unit test|Sources a shell file and calls its functions directly. Dies with bash; there is no binary to retarget."
  elif ! grep -qE 'run_intent|\$INTENT_BIN\b' "$f"; then
    # Never invokes the CLI in any form. Not a conformance test at all: it pins
    # this repository's own content (skills, rules, docs, attribution) and
    # survives a binary swap untouched. This is the WIDENED rule -- the first
    # version keyed on `git ls-files`/`grep -r` and left seven such files
    # UNCLASSIFIED, certifying only the shapes it already knew about.
    echo "out-of-scope|no CLI invocation|Never invokes the CLI. Asserts this repository's own content, not the command surface; survives a binary swap untouched and is not a conformance test."
  else
    # Invokes the CLI and yet nothing changes when the binary is redirected.
    # That combination is genuinely odd and is flagged rather than guessed.
    echo "UNCLASSIFIED|invokes CLI, zero burn|Calls the CLI but no test changes result when the binary is redirected. Either the invocation is inert or the assertions do not depend on it. Needs adjudication."
  fi
}

# extract_test_body <bats-file> <test-name> <out-path>
#
# Writes one `@test` block's body so classify_no_burn can be applied to it.
#
# The block runs from `@test "<name>" {` to the next line that is exactly `}`.
# That is a heuristic, and here is its limit stated rather than discovered: a
# body containing a closing brace in column 0 would be truncated. No file in
# this estate does that, and bats style puts nested braces indented -- but a
# caller seeing a suspiciously short body should suspect this before suspecting
# the classifier.
#
# Returns 1 when the named test is not found, so a caller can refuse rather
# than classify an empty file (which would match "no CLI invocation" and
# silently report out-of-scope -- absence reading as evidence, again).
extract_test_body() {
  local file="$1" name="$2" out="$3"
  awk -v want="$name" '
    index($0, "@test") == 1 {
      # Test names are quoted; take what is between the first and last quote.
      line = $0
      first = index(line, "\"")
      if (first > 0) {
        rest = substr(line, first + 1)
        last = 0
        for (i = length(rest); i > 0; i--) if (substr(rest, i, 1) == "\"") { last = i; break }
        nm = substr(rest, 1, last - 1)
        # BATS UNESCAPES THE NAME BEFORE PRINTING IT TO TAP, so the source and
        # the TAP disagree for any name containing an escape. Live case:
        # ext_seed_validity.bats writes `\$INTENT_HOME` in the @test line and
        # bats reports `$INTENT_HOME`. Comparing the two literally missed the
        # block and the caller reported "body not found" -- which is how this
        # was found, the refusal naming the extraction heuristic as the suspect
        # rather than silently classifying an empty body as out-of-scope.
        gsub(/\\\$/, "$", nm)
        gsub(/\\"/, "\"", nm)
        if (nm == want) { grabbing = 1; next }
        else if (grabbing) { grabbing = 0 }
      }
    }
    grabbing && $0 == "}" { grabbing = 0; exit }
    grabbing { print }
  ' "$file" > "$out"
  [ -s "$out" ]
}

# ---------------------------------------------------------------------------
# THE OVERRIDES TABLE lives here for the same reason the rules do: gen_pertest.sh
# must not emit per-test rows for a file whose class was DECIDED rather than
# measured, or the per-test rows would quietly contradict the ruling the file
# row carries. Moved out of gen_register.sh when the second consumer appeared.
# ---------------------------------------------------------------------------

# DECIDED, not inferred. Every rule below is a grep, and a grep cannot tell code
# from data: a file that carries a call-site pattern as a test FIXTURE matches
# the rule for making that call. `intent_bin_retarget_guard.bats` is the live
# case -- it greps the estate for `bin/intent_<sub>` spellings and holds them as
# literal strings, so the sub-script rule fires on a file that invokes nothing.
#
# Rather than loosen the rule until it stops noticing (which would blind it to
# the real sites), such files are named here with the reason. Same discipline as
# the guard's own allowlist: a classification the machine cannot make is stated
# by a human, never guessed, because a wrong `retire` is coverage that vanishes
# at the cut with nobody watching.
#
# Format: <basename>|<class>|<basis>|<note>
OVERRIDES="
intent_bin_retarget_guard.bats|out-of-scope|harness invariant, decided|Guards the \$INTENT_BIN invariant across the estate by reading test SOURCE; it invokes no CLI at all. It holds \`bin/intent_<sub>\` spellings as literal needles, which the sub-script rule cannot tell from a call site. Carries into v3 unchanged in purpose -- whatever the binary is, the estate must reach it through one name.
whiteboard_clock_guard.bats|out-of-scope|hook behaviour, decided|Exercises a pre-commit hook in a throwaway git repo, not the Intent CLI. Unaffected by the binary swap.
organize_commands.bats|retire|hv ruling 2026-08-14|Retires with the command. hv ruled \`organize\` vestigial by construction -- a strictly structured model cannot hold data in the wrong spot or format -- so both implementations are planned retires (parity.md, 2026-08-14; via vc). Classified by ruling, not by burn.
"

lookup_override() {
  printf '%s\n' "$OVERRIDES" | grep -F "$(basename "$1")|" | head -1 | cut -d'|' -f2-
}

