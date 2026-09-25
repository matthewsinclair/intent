#!/usr/bin/env bats
# lib/templates/hooks/staged-format-guard.sh -- the staged bytes are formatted,
# per declared formatter, and nothing is ever written (issue 0505).
#
# EVERY ARM DRIVES A REAL FORMATTER, not a stub, because the defects this guard
# inherits are all facts about the formatters themselves: `rustfmt --check`
# returns 0 on unformatted STDIN, a lone `.rs` in a scratch directory cannot
# resolve its modules, and a missing tool reports as unformatted files unless
# the two are told apart. A stubbed formatter would pass every one of those and
# prove nothing about the thing that ships.
#
# THE ABSENT-TOOL ARM SCRUBS PATH TO `/usr/bin:/bin`, which carries git, jq and
# the coreutils the guard needs and carries no prettier, mix or rustfmt. That is
# measured on the runner rather than assumed: the arm asserts the tool is absent
# under that PATH before it asserts what the guard did about it.
#
# WHICH ARM HOLDS WHICH CLAUSE (0505's `## Acceptance`, vc, 2026-09-22). The
# clauses live in the issue's body; this is the map, so that deleting an arm
# leaves a clause pointing at nothing IN THIS FILE, visible in the diff.
#
#   AC-1  unformatted staged bytes refused, file named, formatter's own
#         command as the remedy .......................... arms 2 and 9
#   AC-2  never writes: index, worktree and probes ....... arm 4, with arm 2
#         on the printed NOTHING WAS REWRITTEN
#   AC-3  staged bytes decide, not the worktree .......... arm 3, both ways
#   AC-4  no declaration is not applicable ............... arm 1
#         (as an EXIT CODE since 0506, not as prose)
#   AC-5  an absent tool is UNENFORCED by name .......... arm 7
#         ...and not-applicable reaches a human ......... NO LONGER HERE:
#         it reaches them as the runner's `N skipped` tally, which this suite
#         cannot see and `guard_not_applicable.bats` asserts. **THE CLAUSE IS
#         WEAKER THAN IT WAS AND THAT IS RECORDED RATHER THAN GLOSSED**: a
#         named line became an anonymous increment, so a human still learns
#         that something did not apply and no longer learns WHICH from this
#         guard alone. 0506 ruled the line goes; the cost is this.
#   AC-6  the three inherited mechanics, one arm each:
#           rustfmt reads a FILE, never stdin ............ arm 2
#           the probe is named so modules resolve ........ arm 10
#           prettier judges under the path's config ...... arm 12
#   AC-7  a staged deletion and a staged binary skipped .. arms 5 and 6
#
# BEYOND THE SEVEN, held here and claimed against no clause: arm 8 (an
# unrecognised formatter is refused, naming the closed vocabulary) and arm 11
# (the rust edition is resolved, and a default is disclosed in the verdict).
#
# AND THE LIMIT, SAID PLAINLY: nothing machine-checks this mapping. It is a
# reader's aid, not a gate -- an arm renamed or a clause reworded leaves it
# stale and silent, and only a reader catches that.

# WHICH ARM NEEDS WHICH TOOL PRESENT, DECLARED RATHER THAN LEFT TO THE RUNNER
# (issue 0512). EVERY arm here drives a real formatter, so an arm whose subject
# is the guard's VERDICT cannot be measured with that formatter absent -- the
# guard takes its `command -v` branch, records the language UNENFORCED and
# refuses nothing, which is correct behaviour and makes the arm meaningless.
# Nine of the twelve arms carry such a precondition: four on prettier, five on
# rustfmt. They now say so with `require_tool`, and FAIL naming the tool.
#
# **THE ASYMMETRY IS WHAT MAKES THIS WORTH DECLARING, AND IT WAS MEASURED IN
# ONE RUN RATHER THAN REASONED ABOUT.** In run 35739715195, with prettier
# absent on both legs, the four markdown arms split two ways in the same file:
#
#   not ok 624  unformatted staged Markdown is refused        <- red, unintelligibly
#   not ok 627  prettier judges the blob under the config     <- red, unintelligibly
#   ok     620  a staged deletion passes                      <- GREEN, VACUOUSLY
#   ok     621  a staged binary blob is skipped               <- GREEN, VACUOUSLY
#
# 620 and 621 assert that the verdict does NOT say "not formatted". With
# prettier absent the markdown branch never iterates a file at all, so that
# held for a reason unrelated to anything either arm is about. The two arms
# that asserted a REFUSAL went red and got reported; the two that asserted an
# ABSENCE went green and got no attention. **An arm asserting the absence of an
# output is the one a missing tool satisfies for free**, which is why the
# preconditions are declared on all nine and not only on the two CI named.
#
# AND THE ONE THAT NEEDS NO TOOL, SO THE POPULATION IS NOT JUST "ALL OF THEM":
# arm 7 asserts what the guard does when rustfmt is ABSENT, and constructs that
# absence itself; arm 1 (no formatters declared) and arm 8 (an unrecognised
# formatter name) are answered from config before any tool is consulted.

load "../lib/test_helper.bash"

GUARD="${INTENT_PROJECT_ROOT}/lib/templates/hooks/staged-format-guard.sh"

# A scratch repo whose config declares $1 verbatim inside the formatters array,
# e.g. '"rust"' or '"markdown", "rust"' or '' for no declaration at all.
scratch_repo() {
  local repo="$TEST_TEMP_DIR/repo" decl="$1"
  mkdir -p "$repo/intent/.config"
  cd "$repo" || return 1
  git init -q -b main >/dev/null
  git config user.email "test@example.com"
  git config user.name "Test"
  if [ -z "$decl" ]; then
    printf '{\n  "intent_version": "3.2.0"\n}\n' > intent/.config/config.json
  else
    printf '{\n  "intent_version": "3.2.0",\n  "formatters": [%s]\n}\n' "$decl" > intent/.config/config.json
  fi
  git add -A
  git commit -q -m "init"
}

# FOUR SPACES, WHICH IS NOT THIS PROJECT'S HOUSE STYLE AND IS THE POINT. A
# scratch repo carries no `rustfmt.toml`, so rustfmt applies its own default,
# and a fixture written in Intent's 2-space style is correctly refused. The
# first run of this file failed four arms on exactly that, which is the guard
# doing its job on a fixture that had assumed the wrong config.
unformatted_rust() { printf 'fn  main( ) {let x=1;println!("{}",x);}\n'; }
formatted_rust() { printf 'fn main() {\n    let x = 1;\n    println!("{}", x);\n}\n'; }

@test "a project that declares no formatters answers NOT-APPLICABLE and prints nothing" {
  # **THIS ARM ASSERTED A PRINTED LINE AND AN EXIT 0 UNTIL ISSUE 0506 LANDED**,
  # and both were correct for a runner that could read only two answers from a
  # guard it had dispatched. The guard therefore said "not applicable" in prose,
  # which reached a human reading the hook's output and reached no summary, no
  # `--list-guards` and no tally. The runner now reads 3 as not-applicable and
  # counts it in SKIPPED, so the verdict travels as a number a script can read
  # and the line has nothing left to do.
  #
  # THE SILENCE IS ASSERTED, NOT ASSUMED. An arm that merely stopped checking
  # for the line would pass against a guard still printing it, and the line
  # outliving its reason is the exact failure this change exists to avoid.
  #
  # That the RUNNER then counts this as skipped rather than ran is a different
  # subject with its own home: `tests/unit/guard_not_applicable.bats`.
  scratch_repo ""
  printf 'x\n' > a.md
  git add a.md
  run bash "$GUARD"
  [ "$status" -eq 3 ]
  [ -z "$output" ]
}

# THIS ARM IS ALSO THE STDIN MECHANIC'S ONLY WITNESS. `rustfmt --check` reads
# stdin and returns 0 whatever it finds, so a guard that piped the staged blob
# would pass here and its green would be indistinguishable from a working one.
@test "unformatted staged Rust is refused, naming the file, the remedy, and that nothing was rewritten" {
  require_tool rustfmt "whether the guard refuses unformatted staged Rust"
  scratch_repo '"rust"'
  unformatted_rust > main.rs
  git add main.rs
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  [[ "$output" == *"rust is not formatted"* ]]
  [[ "$output" == *"main.rs"* ]]
  [[ "$output" == *"rustfmt <the files listed above>"* ]]
  [[ "$output" == *"NOTHING WAS REWRITTEN"* ]]
}

# The verdict follows the INDEX, in both directions. This is what a staged-blob
# check is for: the hunk-scoped commit, where worktree and index deliberately
# differ, is exactly the technique the no-writing ruling exists to protect.
@test "the verdict follows the staged bytes and not the worktree, both ways" {
  require_tool rustfmt "whether the verdict follows the staged bytes rather than the worktree"
  scratch_repo '"rust"'
  formatted_rust > main.rs
  git add main.rs
  unformatted_rust > main.rs
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" == *"ok --"* ]]

  unformatted_rust > other.rs
  git add other.rs
  formatted_rust > other.rs
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  [[ "$output" == *"other.rs"* ]]
}

@test "nothing is written across a refusal: index, worktree and probes are untouched" {
  require_tool rustfmt "whether a refusal writes anything"
  scratch_repo '"rust"'
  unformatted_rust > main.rs
  git add main.rs
  local tree_before worktree_before
  tree_before="$(git write-tree)"
  worktree_before="$(git hash-object main.rs)"

  run bash "$GUARD"
  [ "$status" -eq 1 ]

  [ "$(git write-tree)" = "$tree_before" ]
  [ "$(git hash-object main.rs)" = "$worktree_before" ]
  run find . -name '.staged-check-*'
  [ -z "$output" ]
}

# `git show :path` on a deletion fails, so a guard that took every staged path
# would refuse a commit for a file that is not there. The filter carries it and
# this arm holds the filter in place.
@test "a staged deletion passes" {
  require_tool prettier "whether the staged-deletion filter carries a deletion"
  scratch_repo '"markdown"'
  printf '# Title\n' > doc.md
  git add doc.md
  git commit -q -m "a document"
  git rm -q doc.md
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" != *"not formatted"* ]]
}

@test "a staged binary blob is skipped rather than handed to a text formatter" {
  require_tool prettier "whether a staged binary blob reaches a text formatter"
  scratch_repo '"markdown"'
  printf 'PNG\000\001\002binary\000bytes\n' > logo.md
  git add logo.md
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" != *"not formatted"* ]]
}

# A missing tool and unformatted bytes are different facts with different
# remedies. The hook this was extracted from conflated them: with prettier
# absent, every staged Markdown file was named as unformatted.
@test "a declared formatter whose tool is absent is UNENFORCED in the verdict and blocks nothing" {
  scratch_repo '"rust"'
  unformatted_rust > main.rs
  git add main.rs

  # Through a shell: `command` is a builtin, so `env` could not exec it and the
  # 127 that answered said nothing about rustfmt (BW01).
  run env PATH=/usr/bin:/bin sh -c 'command -v rustfmt'
  [ "$status" -eq 1 ]

  run env PATH=/usr/bin:/bin bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" == *"UNENFORCED"* ]]
  [[ "$output" == *"rust(rustfmt)"* ]]
  [[ "$output" != *"not formatted"* ]]
}

@test "an unrecognised formatter name is refused, naming the vocabulary" {
  scratch_repo '"swift"'
  printf 'x\n' > a.md
  git add a.md
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  [[ "$output" == *"does not know"* ]]
  [[ "$output" == *"swift"* ]]
  [[ "$output" == *"markdown, elixir, rust"* ]]
}

@test "unformatted staged Markdown is refused, with prettier's own command as the remedy" {
  require_tool prettier "whether the guard refuses unformatted staged Markdown"
  scratch_repo '"markdown"'
  printf '# Title\n\n*  one\n*  two\n' > doc.md
  git add doc.md
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  [[ "$output" == *"markdown is not formatted"* ]]
  [[ "$output" == *"doc.md"* ]]
  [[ "$output" == *"prettier --write"* ]]
}

# ISSUE 0572: A FILE THE FORMATTER COULD NOT CHECK IS NOT A FILE IT FOUND
# UNFORMATTED, for every formatter and not only Rust. prettier says which by its
# exit code (1 unformatted, 2 an error), and mix exits 1 for both, so its own
# `--check-formatted` line is what names an unformatted file. Read as "not
# formatted", an unclosed `(` was refused with `mix format` as the remedy, and
# that remedy fails on the same parse error.
@test "a staged Elixir file mix cannot parse is reported as not checked, never as not formatted" {
  require_tool mix "whether an unparseable Elixir file is told apart from an unformatted one"
  scratch_repo '"elixir"'
  printf 'defmodule A do\n  def f(x) do\n    (x\n  end\nend\n' > bad.ex
  git add bad.ex
  run bash "$GUARD"
  [[ "$output" != *"elixir is not formatted"* ]]
  [[ "$output" == *"could not be checked"* ]]
  [[ "$output" == *"bad.ex"* ]]

  # THE CONTROL: an unformatted file that parses is still refused as such.
  printf 'defmodule A do\ndef f(x),   do: x\nend\n' > ugly.ex
  git rm -q --cached bad.ex
  rm bad.ex
  git add ugly.ex
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  [[ "$output" == *"elixir is not formatted"* ]]
  [[ "$output" == *"ugly.ex"* ]]
}

@test "a staged Markdown file prettier cannot check is reported as not checked, never as not formatted" {
  require_tool prettier "whether a Markdown file prettier errors on is told apart from an unformatted one"
  scratch_repo '"markdown"'
  printf '{ "semi": \n' > .prettierrc
  printf '# Title\n' > doc.md
  git add doc.md
  run bash "$GUARD"
  [[ "$output" != *"markdown is not formatted"* ]]
  [[ "$output" == *"could not be checked"* ]]
  [[ "$output" == *"doc.md"* ]]
  # A file not checked is not counted as checked in the all-clear line.
  [[ "$output" == *"ok -- 0 staged file(s) checked"* ]]
}

# The probe path, which only runs when the staged blob differs from the
# worktree. A lone `.rs` in a scratch directory cannot resolve `mod common;`,
# and rustfmt then exits non-zero -- which an arm reading the exit code alone
# reports as "not formatted", refusing correct files with a remedy that runs
# clean and changes nothing. Beside the original, resolution is the file's own.
@test "a probed Rust file resolves its modules, so a correct file is not refused" {
  require_tool rustfmt "whether a probed Rust file resolves its modules"
  scratch_repo '"rust"'
  mkdir -p tests
  printf 'pub fn helper() -> i32 {\n    1\n}\n' > tests/common.rs
  printf 'mod common;\n\n#[test]\nfn it_works() {\n    assert_eq!(common::helper(), 1);\n}\n' > tests/foo.rs
  git add tests/common.rs tests/foo.rs
  printf 'mod common;\n\n#[test]\nfn it_works() {\n    assert_eq!(common::helper(), 1);\n}\n// worktree differs, so the staged blob is probed\n' > tests/foo.rs
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" == *"ok --"* ]]
  [[ "$output" != *"could not be checked"* ]]
}

# THE EDITION IS RESOLVED AND THE VERDICT SAYS WHEN IT COULD NOT BE. 2021 and
# 2024 disagree about import order, so a guard shipped with one estate's edition
# baked in would refuse another's correct code; and a reader must never be left
# to assume the edition matched the crate when nothing declared one.
@test "the verdict discloses a default edition, and stays silent when a manifest declares one" {
  require_tool rustfmt "what edition the verdict discloses"
  scratch_repo '"rust"'
  formatted_rust > main.rs
  git add main.rs
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" == *"rustfmt's default edition"* ]]

  printf '[package]\nname = "x"\nversion = "0.1.0"\nedition = "2024"\n' > Cargo.toml
  git add Cargo.toml
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" != *"rustfmt's default edition"* ]]
}

# AC-6's THIRD MECHANIC, whose discriminator is WHICH config was resolved.
# `git show | prettier --check` with no `--stdin-filepath` exits 0 on any bytes
# at all (measured on this runner), so the flag's absence is a vacuous green of
# exactly the shape `rustfmt --check` has on stdin; arm 9 catches that much. It
# does NOT catch a filepath that is not the file's OWN -- a basename, or the
# path of a temporary -- which resolves a different .prettierrc and judges the
# blob under a config that governs nothing. Here the root declares proseWrap
# "preserve" and `docs/` declares "always" at printWidth 40, so one staged long
# line is formatted under the root's config and unformatted under its own, and
# the verdict flips with the config the path resolves.
@test "prettier judges the blob under the config the file's OWN path resolves" {
  require_tool prettier "which .prettierrc the staged blob is judged under"
  scratch_repo '"markdown"'
  printf '{"proseWrap":"preserve"}\n' > .prettierrc
  mkdir -p docs
  printf '{"proseWrap":"always","printWidth":40}\n' > docs/.prettierrc
  printf 'One paragraph written as a single long line that a printWidth of forty would certainly rewrap.\n' > docs/long.md
  git add .prettierrc docs/.prettierrc docs/long.md
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  [[ "$output" == *"docs/long.md"* ]]

  # The negative control, same bytes, same commit: with `docs/` declaring
  # nothing the root's "preserve" governs and the file is formatted.
  git rm -q --cached docs/.prettierrc
  rm docs/.prettierrc
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  [[ "$output" == *"ok --"* ]]
}
