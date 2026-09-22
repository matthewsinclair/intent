#!/usr/bin/env bats
# ONE BEHAVIOUR FOR A MISSING ARMED TOOL, AND THE CENSUS THAT KEEPS IT AT ONE
# (issue 0512).
#
# THE DEFECT WAS NOT A BUG, IT WAS THREE ANSWERS. A suite arm whose subject is
# an armed tool that is not installed had three fates in this repository, and
# the worst was the default:
#
#   PANIC   `require_prettier()` in
#           `native/rust/crates/intent-cli/tests/view_single_writer.rs` aborts,
#           naming the tool and the waiver.
#   SKIP    `devbin_fmt_md.bats` reported `ok 160 ... # skip prettier
#           unavailable offline` -- the word `ok`, in the pass column.
#   ASSERT  the `staged_format_guard.bats` arms said only that
#           `[ "$status" -eq 1 ]' failed`, from which no reader can tell a
#           broken guard from an absent tool.
#
# The three disagreed about whether a missing tool is fatal, invisible, or a
# test failure. `require_tool` in `tests/lib/test_helper.bash` is the one
# answer; this file drives it to both verdicts and then censuses the suite so
# the other two cannot grow back.
#
# **WHY A CENSUS AND NOT JUST THE FIX.** The two arms CI named were repaired in
# `f6ad0f990`'s successor by installing prettier, which was right and stopped
# nothing: the next arm written against an armed tool inherits whichever of the
# three behaviours its author happens to copy. The class is the deliverable.
#
# WHAT THIS FILE CANNOT SEE, SAID PLAINLY BECAUSE A CENSUS THAT DOES NOT NAME
# ITS BLIND SPOT IS READ AS COMPLETE. Arm 5 is a grep, not a parser. It finds a
# skip that NAMES an armed tool and gives an availability reason. It does not
# find a skip that probes a LAUNCHER instead of the tool -- `command -v npx ||
# skip` was exactly that shape and mentions no armed tool at all -- and it does
# not find one whose reason is worded outside the idiom list. It is a floor.

load "../lib/test_helper.bash"

HELPER="${INTENT_PROJECT_ROOT}/tests/lib/test_helper.bash"

setup_file() {
  NO_PRETTIER_PATH="${BATS_FILE_TMPDIR}/no-prettier-bin"
  build_no_tool_path "$NO_PRETTIER_PATH" prettier
  export NO_PRETTIER_PATH
}

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-armed-XXXXXX)"
}

teardown() {
  [ -d "${TEST_TEMP_DIR}" ] && rm -rf "${TEST_TEMP_DIR}"
  return 0
}

# ---------------------------------------------------------------------------

@test "PREMISE: the armed set is discovered, non-empty, and holds the tools this issue is about" {
  # Without this, arm 5 passes vacuously the day discovery breaks: an empty set
  # means an empty loop means no offenders means green. A census is
  # uninterpretable until you know how many things it looked at.
  run armed_tools
  [ "$status" -eq 0 ]
  [ -n "$output" ]

  # Both declaring surfaces must be represented, because each is read by a
  # different expression and either can silently stop matching: `critic_tool:`
  # frontmatter in the rule library, and the guard's own UNENFORCED lines.
  [[ "$output" == *"shellcheck"* ]]   # rule library
  [[ "$output" == *"prettier"* ]]     # staged-format-guard
  [[ "$output" == *"rustfmt"* ]]      # staged-format-guard
}

@test "require_tool RETURNS for a tool that is present" {
  # The other half of the arm below. A refusal check that has only ever
  # answered "refuse" is indistinguishable from one that always refuses.
  require_tool git "this control, which needs git to exist"
}

@test "require_tool FAILS for a genuinely absent tool, and the failure NAMES it" {
  # A CONSTRUCTED absence of a REAL armed tool, not a made-up tool name: a name
  # that could never exist would exercise the same branch while proving nothing
  # about the case that actually happens.
  run env PATH="$NO_PRETTIER_PATH" bash -c \
    "source '$HELPER'; require_tool prettier 'the subject of this control'"
  [ "$status" -eq 1 ]
  [[ "$output" == *"prettier is not on PATH"* ]]
  [[ "$output" == *"the subject of this control"* ]]
  [[ "$output" == *"INTENT_ALLOW_MISSING_PRETTIER"* ]]
  [[ "$output" == *"Install prettier"* ]]

  # NOT `[[ "$output" != *"skip"* ]]`, which is what this line said first and
  # which FAILED: the diagnosis itself contains the word, in the sentence "this
  # arm FAILS rather than skipping". An assertion that the remedy text avoids a
  # word is a check on prose, not on behaviour. The behaviour is the exit
  # status, asserted above.

  # THE FARM DID NOT SIMPLY BREAK EVERYTHING. Under the same PATH a different
  # tool still resolves, so the failure above is about prettier and not about
  # the fixture.
  run env PATH="$NO_PRETTIER_PATH" bash -c \
    "source '$HELPER'; require_tool git 'a tool the farm keeps'"
  [ "$status" -eq 0 ]
}

@test "the waiver returns, and says NOT MEASURED rather than reporting a pass" {
  # The waiver exists so a contributor without prettier can work; it must never
  # read as a green. The env var is the SAME ONE the Rust suite honours, which
  # is the only thing binding the two implementations together.
  run env PATH="$NO_PRETTIER_PATH" INTENT_ALLOW_MISSING_PRETTIER=1 bash -c \
    "source '$HELPER'; require_tool prettier 'the waived subject'"
  [ "$status" -eq 0 ]
  [[ "$output" == *"WAIVED"* ]]
  [[ "$output" == *"NOT"* ]]
  [[ "$output" == *"waiver, not a pass"* ]]
}

@test "no arm in the suite skips because an armed tool is absent" {
  local offenders="" tool hit
  while read -r tool; do
    while IFS= read -r hit; do
      [ -n "$hit" ] && offenders="${offenders}${tool}: ${hit}"$'\n'
    done < <(grep -n 'skip' "${INTENT_PROJECT_ROOT}"/tests/unit/*.bats 2>/dev/null \
      | grep -v ':[[:space:]]*#' \
      | grep -F -- "$tool" \
      | grep -Ei -- 'unavailable|not installed|not found|is missing|not on PATH|is absent')
  done < <(armed_tools)

  if [ -n "$offenders" ]; then
    echo "an arm degrades to a skip when an armed tool is absent, which reports"
    echo "\`ok\` for a measurement that never happened. Use require_tool:"
    echo "$offenders"
    return 1
  fi
}

@test "every skip in the suite declares WHICH class it is" {
  # THE THREE CLASSES PRINTED THE SAME THING, WHICH IS HOW THE ONE THAT IS A
  # DEFECT HID BEHIND THE TWO THAT ARE NOT. `# skip` says nothing about whether
  # an arm could not reach its tool, could not find a witness in the corpus, or
  # is describing a system this machine is not. Each now carries its own tag.
  #
  # THE EXCEPTION LIST IS THE OPEN QUESTION, WRITTEN DOWN RATHER THAN SETTLED.
  # Nine arms per leg skip because elixir is absent, in three spellings, two of
  # which differ only in capitalisation -- measured in run 35743714571. Whether
  # CI should install elixir and let those nine run, or whether they are
  # legitimately environment-gated, is a fork with a real cost on each side and
  # it is not this file's to pick. Until it is ruled they are listed here BY
  # NAME, so the question stays visible and cannot quietly grow a tenth member.
  local exceptions="tests/unit/test_autopsy.bats tests/unit/rule_pack_elixir_runnable.bats"

  local untagged="" hit file
  while IFS= read -r hit; do
    [ -z "$hit" ] && continue
    file="${hit%%:*}"
    case " $exceptions " in
      *" ${file#"${INTENT_PROJECT_ROOT}/"} "*) continue ;;
    esac
    untagged="${untagged}${hit}"$'\n'
  done < <(grep -nE '(^|[[:space:]]|\|\||&&)skip[[:space:]]+"' \
    "${INTENT_PROJECT_ROOT}"/tests/unit/*.bats 2>/dev/null \
    | grep -v ':[[:space:]]*#' \
    | grep -vE '"(NO WITNESS|OTHER SYSTEM):')

  if [ -n "$untagged" ]; then
    echo "a bare \`skip\` says nothing about WHICH of the three classes it is."
    echo "Use skip_no_witness, skip_other_system, or require_tool if the arm's"
    echo "subject is an instrument that cannot be reached:"
    echo "$untagged"
    return 1
  fi
}
