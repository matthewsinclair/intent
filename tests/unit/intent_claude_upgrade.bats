#!/usr/bin/env bats
# Tests for `intent claude upgrade --apply` (ST0035/WP-11).
#
# Five spec scenarios from intent/st/ST0035/WP/11/info.md (lines 76-82):
#   1. Fresh project --apply -> all canon artefacts installed
#   2. Re-run --apply -> zero changes (idempotence)
#   3. User-edited CLAUDE.md user-section -> preserved on refresh
#   4. Pre-existing non-Intent pre-commit hook -> chained, not overwritten
#   5. --dry-run -> no file modifications

load "../lib/test_helper.bash"

# Override setup() to additionally isolate HOME so installed subagents and
# skills on the host machine do not bleed into the upgrade probes (which
# would enqueue UPDATE_SUBAGENT / UPDATE_SKILL actions and bias snapshots).
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  setup_fake_home
  cd "$TEST_TEMP_DIR" || exit 1
  # NORMALIZE_GITIGNORE scenarios call `git commit` in scratch repos.
  # The HOME override above isolates from the host's gitconfig, so git
  # has no user.email / user.name and commits fail with exit 128. Set
  # them in the temp HOME so git commits succeed.
  git config --global user.email "test@intent.local"
  git config --global user.name "Intent Test"
}

teardown() {
  teardown_fake_home
  if [ -d "$TEST_TEMP_DIR" ]; then
    cd "$INTENT_PROJECT_ROOT" || exit 1
    rm -rf "$TEST_TEMP_DIR"
  fi
}

# Create + initialise a fresh Intent project at $TEST_TEMP_DIR/<name>.
# Sets PROJ_DIR and cds there. `intent init` creates .git automatically.
init_scratch() {
  local name="${1:-scratch}"
  PROJ_DIR="$TEST_TEMP_DIR/$name"
  mkdir -p "$PROJ_DIR"
  cd "$PROJ_DIR"
  run_intent init >/dev/null 2>&1 || fail "intent init failed in $PROJ_DIR"
}

# Snapshot tree state (paths + sha1) for idempotence assertions. Excludes
# .git internals (mtime-noisy and irrelevant to canon state).
tree_snapshot() {
  ( cd "$1" && find . -type f \
      -not -path './.git/*' \
      -not -name '*.swp' \
      | sort \
      | xargs shasum 2>/dev/null )
}

@test "fresh project --apply installs all canon artefacts" {
  init_scratch fresh

  run run_intent claude upgrade --apply
  assert_success

  assert_file_exists "$PROJ_DIR/.claude/settings.json"
  # The three hook scripts are NO LONGER installed into a project: the door
  # (`intent claude hook <name>`) execs the body from $INTENT_HOME, so a project
  # copy is read by nothing. They were residue from issue 0016 moving dispatch
  # to the CLI without removing the install step.
  [ ! -f "$PROJ_DIR/.claude/scripts/session-context.sh" ] || fail "session-context.sh installed; it is inert and should be pruned"
  [ ! -f "$PROJ_DIR/.claude/scripts/require-in-session.sh" ] || fail "require-in-session.sh installed; it is inert and should be pruned"
  [ ! -f "$PROJ_DIR/.claude/scripts/post-tool-advisory.sh" ] || fail "post-tool-advisory.sh installed; it is inert and should be pruned"
  assert_file_exists "$PROJ_DIR/.git/hooks/pre-commit"
  assert_file_exists "$PROJ_DIR/.intent_critic.yml"
  assert_file_exists "$PROJ_DIR/CLAUDE.md"
  assert_file_exists "$PROJ_DIR/usage-rules.md"
  assert_file_exists "$PROJ_DIR/intent/llm/MODULES.md"
  assert_file_exists "$PROJ_DIR/intent/llm/DECISION_TREE.md"

  [ -x "$PROJ_DIR/.git/hooks/pre-commit" ] || fail "pre-commit hook not executable"

  # Chained architecture: canon body lives at pre-commit.intent; pre-commit
  # is a chain stub that delegates via the marker block.
  assert_file_exists "$PROJ_DIR/.git/hooks/pre-commit.intent"
  [ -x "$PROJ_DIR/.git/hooks/pre-commit.intent" ] || fail "pre-commit.intent not executable"
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit.intent" "intent critic gate"
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "intent-chain-block:start"
  # CLAUDE.md carries the Intent footer marker (so refresh path activates).
  # **THE INVARIANT, NOT THE PROSE.** This asserted "Generated from" and went
  # red on a pure footer rewording -- the same coupling that broke
  # CANON_INTENT_FOOTER_MARK. The template PATH is what survives a rewording;
  # the English around it never did.
  assert_file_contains "$PROJ_DIR/CLAUDE.md" "lib/templates/llm/_CLAUDE.md"
}

@test "re-running --apply produces no file changes (idempotence)" {
  init_scratch idem
  run_intent claude upgrade --apply >/dev/null 2>&1

  before="$(tree_snapshot "$PROJ_DIR")"

  run run_intent claude upgrade --apply
  assert_success

  after="$(tree_snapshot "$PROJ_DIR")"
  if [ "$before" != "$after" ]; then
    diff <(echo "$before") <(echo "$after")
    fail "tree changed on second --apply (not idempotent)"
  fi
}

@test "user-edited CLAUDE.md user-section is preserved on refresh" {
  init_scratch preserve
  run_intent claude upgrade --apply >/dev/null 2>&1

  # Inject custom directives inside the user section AND drift a non-user
  # heading so REFRESH_CLAUDE_MD is enqueued (otherwise nothing to do).
  awk '
    /<!-- user:end -->/ && !done {
      print "MY CUSTOM DIRECTIVE"
      print "Line two of custom content."
      done = 1
    }
    { print }
  ' "$PROJ_DIR/CLAUDE.md" > "$PROJ_DIR/CLAUDE.md.new"
  mv "$PROJ_DIR/CLAUDE.md.new" "$PROJ_DIR/CLAUDE.md"
  # Use perl -i for portability (sed -i syntax differs between BSD and GNU).
  perl -i -pe 's/^## Project-specific$/## Project-specific (DRIFTED)/' "$PROJ_DIR/CLAUDE.md"

  run run_intent claude upgrade --apply
  assert_success

  assert_file_contains "$PROJ_DIR/CLAUDE.md" "MY CUSTOM DIRECTIVE"
  assert_file_contains "$PROJ_DIR/CLAUDE.md" "Line two of custom content."

  # Drifted heading reverted to canonical.
  if grep -qF "## Project-specific (DRIFTED)" "$PROJ_DIR/CLAUDE.md"; then
    fail "drifted heading should have been reverted by refresh"
  fi
}

@test "pre-existing non-Intent pre-commit hook is chained, body preserved" {
  init_scratch chain

  rm -f "$PROJ_DIR/.git/hooks/pre-commit"
  printf '%s\n' '#!/bin/sh' 'set -e' '' '# user hook' 'echo running' 'exit 0' \
    > "$PROJ_DIR/.git/hooks/pre-commit"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"

  run run_intent claude upgrade --apply
  assert_success

  # pre-commit.intent installed and executable.
  assert_file_exists "$PROJ_DIR/.git/hooks/pre-commit.intent"
  [ -x "$PROJ_DIR/.git/hooks/pre-commit.intent" ] || fail "chained hook not executable"

  # Chain block inserted into the user's pre-commit.
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "intent-chain-block:start"
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "intent-chain-block:end"
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "pre-commit.intent"

  # User's original body is still present (preserved, not overwritten).
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "echo running"
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "exit 0"

  # Preamble preserved: shebang + set -e remain at the top, before the block.
  local first_block_line
  first_block_line=$(grep -n "intent-chain-block:start" "$PROJ_DIR/.git/hooks/pre-commit" | head -1 | cut -d: -f1)
  [ "$first_block_line" -gt 2 ] || fail "chain block landed too early (line $first_block_line; should be after shebang + set)"
}

@test "chain block insertion is idempotent across re-applies" {
  init_scratch chain_idem

  rm -f "$PROJ_DIR/.git/hooks/pre-commit"
  printf '%s\n' '#!/bin/sh' 'set -e' '' 'echo first run' \
    > "$PROJ_DIR/.git/hooks/pre-commit"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"

  # First apply -> inserts chain block.
  run_intent claude upgrade --apply >/dev/null 2>&1
  first_sha="$(shasum "$PROJ_DIR/.git/hooks/pre-commit" | awk '{print $1}')"

  # Second apply -> chain block already present; must not modify the file.
  run_intent claude upgrade --apply >/dev/null 2>&1
  second_sha="$(shasum "$PROJ_DIR/.git/hooks/pre-commit" | awk '{print $1}')"

  [ "$first_sha" = "$second_sha" ] || fail "chain block re-inserted (not idempotent: $first_sha -> $second_sha)"

  # Exactly one block of markers.
  local start_count end_count
  start_count=$(grep -c "intent-chain-block:start" "$PROJ_DIR/.git/hooks/pre-commit")
  end_count=$(grep -c "intent-chain-block:end" "$PROJ_DIR/.git/hooks/pre-commit")
  [ "$start_count" -eq 1 ] || fail "expected 1 start marker, got $start_count"
  [ "$end_count" -eq 1 ] || fail "expected 1 end marker, got $end_count"
}

@test "dry-run reports CHAIN_REQUIRED when chain block missing" {
  init_scratch chain_required

  # Install pre-commit.intent without the chain block (simulates a project
  # that ran a pre-v2.10.0 canon-installer that left the manual-paste step).
  rm -f "$PROJ_DIR/.git/hooks/pre-commit"
  printf '%s\n' '#!/bin/sh' 'echo nothing' \
    > "$PROJ_DIR/.git/hooks/pre-commit"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"
  cp "$INTENT_PROJECT_ROOT/lib/templates/hooks/pre-commit.sh" "$PROJ_DIR/.git/hooks/pre-commit.intent"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit.intent"

  run run_intent claude upgrade
  assert_success
  assert_output_contains "CHAIN_REQUIRED"
  assert_output_contains "Insert intent-chain-block"
}

@test "dry-run reports CHAINED when chain block already present" {
  init_scratch chained_ok

  rm -f "$PROJ_DIR/.git/hooks/pre-commit"
  cat > "$PROJ_DIR/.git/hooks/pre-commit" <<'PCH'
#!/bin/sh
set -e
# intent-chain-block:start (generated by intent claude upgrade)
_intent_chain="$(git rev-parse --git-path hooks 2>/dev/null)/pre-commit.intent"
if [ -x "$_intent_chain" ]; then
  "$_intent_chain" "$@" || exit $?
fi
# intent-chain-block:end
echo done
PCH
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"
  cp "$INTENT_PROJECT_ROOT/lib/templates/hooks/pre-commit.sh" "$PROJ_DIR/.git/hooks/pre-commit.intent"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit.intent"

  run run_intent claude upgrade
  assert_success
  assert_output_contains "CHAINED"
  # No CHAIN_REQUIRED action should appear in the plan.
  if echo "$output" | grep -q "Insert intent-chain-block"; then
    fail "plan still contains chain-block action when CHAINED"
  fi
}

@test "legacy single-file pre-commit (canon body verbatim) is migrated to chained" {
  init_scratch legacy_migrate

  # Simulate legacy install pattern (pre-chaining era): canon body sits at
  # pre-commit, no pre-commit.intent. Wipe what init_scratch produced and
  # plant the canon body directly at pre-commit.
  rm -f "$PROJ_DIR/.git/hooks/pre-commit" "$PROJ_DIR/.git/hooks/pre-commit.intent"
  cp "$INTENT_PROJECT_ROOT/lib/templates/hooks/pre-commit.sh" "$PROJ_DIR/.git/hooks/pre-commit"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"

  run run_intent claude upgrade --apply
  assert_success

  # Canon body relocated to pre-commit.intent.
  assert_file_exists "$PROJ_DIR/.git/hooks/pre-commit.intent"
  [ -x "$PROJ_DIR/.git/hooks/pre-commit.intent" ] || fail "pre-commit.intent not executable"
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit.intent" "intent critic gate"

  # pre-commit reduced to a chain stub.
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "intent-chain-block:start"
  assert_file_contains "$PROJ_DIR/.git/hooks/pre-commit" "pre-commit.intent"
  if grep -qF "intent critic gate" "$PROJ_DIR/.git/hooks/pre-commit"; then
    fail "pre-commit still contains canon body after migration"
  fi
}

@test "dry-run reports LEGACY when canon body sits at pre-commit (no pre-commit.intent)" {
  init_scratch legacy_detect

  rm -f "$PROJ_DIR/.git/hooks/pre-commit" "$PROJ_DIR/.git/hooks/pre-commit.intent"
  cp "$INTENT_PROJECT_ROOT/lib/templates/hooks/pre-commit.sh" "$PROJ_DIR/.git/hooks/pre-commit"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"

  run run_intent claude upgrade
  assert_success
  assert_output_contains "LEGACY"
  assert_output_contains "single-file"
}

@test "legacy migration is idempotent (re-apply after migration is a no-op)" {
  init_scratch legacy_idem

  rm -f "$PROJ_DIR/.git/hooks/pre-commit" "$PROJ_DIR/.git/hooks/pre-commit.intent"
  cp "$INTENT_PROJECT_ROOT/lib/templates/hooks/pre-commit.sh" "$PROJ_DIR/.git/hooks/pre-commit"
  chmod +x "$PROJ_DIR/.git/hooks/pre-commit"

  run_intent claude upgrade --apply >/dev/null 2>&1
  first_pc="$(shasum "$PROJ_DIR/.git/hooks/pre-commit" | awk '{print $1}')"
  first_pci="$(shasum "$PROJ_DIR/.git/hooks/pre-commit.intent" | awk '{print $1}')"

  run_intent claude upgrade --apply >/dev/null 2>&1
  second_pc="$(shasum "$PROJ_DIR/.git/hooks/pre-commit" | awk '{print $1}')"
  second_pci="$(shasum "$PROJ_DIR/.git/hooks/pre-commit.intent" | awk '{print $1}')"

  [ "$first_pc" = "$second_pc" ] || fail "pre-commit re-modified on idempotent re-apply ($first_pc -> $second_pc)"
  [ "$first_pci" = "$second_pci" ] || fail "pre-commit.intent re-modified on idempotent re-apply ($first_pci -> $second_pci)"
}

@test "NORMALIZE_GITIGNORE replaces broad .claude with .claude/settings.local.json" {
  init_scratch gi_broad

  printf '%s\n' '/_build/' '.claude' '/cover/' > "$PROJ_DIR/.gitignore"
  git -C "$PROJ_DIR" add .gitignore && git -C "$PROJ_DIR" commit -m base >/dev/null 2>&1

  run run_intent claude upgrade --apply
  assert_success

  # Broad .claude line replaced with the canonical form.
  if grep -qE '^\.claude/?$' "$PROJ_DIR/.gitignore"; then
    fail "broad .claude/ line still present after normalize"
  fi
  assert_file_contains "$PROJ_DIR/.gitignore" ".claude/settings.local.json"
  # /AGENTS.md.bak appended too.
  assert_file_contains "$PROJ_DIR/.gitignore" "/AGENTS.md.bak"
  # Surrounding lines preserved.
  assert_file_contains "$PROJ_DIR/.gitignore" "/_build/"
  assert_file_contains "$PROJ_DIR/.gitignore" "/cover/"
}

@test "NORMALIZE_GITIGNORE appends missing .claude/settings.local.json + AGENTS.md.bak" {
  init_scratch gi_missing

  printf '%s\n' '/_build/' '/deps/' > "$PROJ_DIR/.gitignore"
  git -C "$PROJ_DIR" add .gitignore && git -C "$PROJ_DIR" commit -m base >/dev/null 2>&1

  run run_intent claude upgrade --apply
  assert_success

  assert_file_contains "$PROJ_DIR/.gitignore" ".claude/settings.local.json"
  assert_file_contains "$PROJ_DIR/.gitignore" "/AGENTS.md.bak"
  # Original lines preserved.
  assert_file_contains "$PROJ_DIR/.gitignore" "/_build/"
  assert_file_contains "$PROJ_DIR/.gitignore" "/deps/"
}

@test "NORMALIZE_GITIGNORE is idempotent (canonical .gitignore not re-touched)" {
  init_scratch gi_idem

  # "Canonical" means every canon-managed entry present, so this fixture grows
  # by one each time the seam adopts another (issue 0018 added the treeindex
  # rule). The assertion below is the real contract and is unchanged: a fully
  # canonical file must come out byte-identical.
  printf '%s\n' '/_build/' '.claude/settings.local.json' '/AGENTS.md.bak' 'intent/.treeindex/' > "$PROJ_DIR/.gitignore"
  git -C "$PROJ_DIR" add .gitignore && git -C "$PROJ_DIR" commit -m base >/dev/null 2>&1

  before_sha="$(shasum "$PROJ_DIR/.gitignore" | awk '{print $1}')"
  run run_intent claude upgrade --apply
  assert_success
  after_sha="$(shasum "$PROJ_DIR/.gitignore" | awk '{print $1}')"

  [ "$before_sha" = "$after_sha" ] || fail "canonical .gitignore re-modified ($before_sha -> $after_sha)"

  # Dry-run reports OK, not NORMALIZE.
  run run_intent claude upgrade
  assert_success
  if echo "$output" | grep -q "NORMALIZE"; then
    fail "NORMALIZE action enqueued for an already-canonical .gitignore"
  fi
}

@test "REVIEW warning fires only when RULES/ARCH match _default verbatim" {
  init_scratch review_default

  run_intent claude upgrade --apply >/dev/null 2>&1

  # Verbatim _default -> warning fires.
  run run_intent claude upgrade
  assert_success
  assert_output_contains "REVIEW intent/llm/RULES.md"
  assert_output_contains "still verbatim _default template"

  # Customise RULES.md -> warning suppressed.
  echo "## Project rule: never use sleep in tests" >> "$PROJ_DIR/intent/llm/RULES.md"
  run run_intent claude upgrade
  assert_success
  if echo "$output" | grep -q "REVIEW intent/llm/RULES.md"; then
    fail "REVIEW fired even though RULES.md was customised"
  fi
}

@test "PROJECT_NAME resolves from config.json (not basename of relative path)" {
  init_scratch myproj

  # Force the relative-path codepath: cd into the project, invoke with --project-dir "."
  cd "$PROJ_DIR" || fail "cannot cd"
  run run_intent claude upgrade --apply --project-dir .
  assert_success

  # CLAUDE.md title must be the canonical project name from config.json,
  # not "." (which is what basename "." returns).
  local first_line
  first_line=$(head -1 "$PROJ_DIR/CLAUDE.md")
  if [ "$first_line" = "# ." ]; then
    fail "CLAUDE.md title is '# .' (PROJECT_NAME basename bug returned)"
  fi
  [ "$first_line" = "# myproj" ] || fail "expected '# myproj', got '$first_line'"
}

@test "canon-installer always installs _default RULES/ARCHITECTURE (not language-specific)" {
  init_scratch any

  run run_intent claude upgrade --apply
  assert_success

  # _default template markers present.
  assert_file_contains "$PROJ_DIR/intent/llm/RULES.md" "intent claude rules list --lang <lang>"
  assert_file_contains "$PROJ_DIR/intent/llm/ARCHITECTURE.md" "System architecture and design decisions"

  # Elixir-template markers absent (would indicate accidental fallback to elixir/).
  if grep -qF "Core Elixir Rules" "$PROJ_DIR/intent/llm/RULES.md"; then
    fail "intent/llm/RULES.md got Elixir template (canon-installer must use _default)"
  fi
  if grep -qF "Phoenix/Ash web application" "$PROJ_DIR/intent/llm/ARCHITECTURE.md"; then
    fail "intent/llm/ARCHITECTURE.md got Elixir template (canon-installer must use _default)"
  fi
}

@test "canon-installer ignores language markers (multi-language reality)" {
  init_scratch polyglot
  # Stage markers for multiple languages -- canon-installer must still use
  # _default and not pick any single one as "primary".
  touch "$PROJ_DIR/mix.exs" "$PROJ_DIR/Cargo.toml" "$PROJ_DIR/Package.swift"

  run run_intent claude upgrade --apply
  assert_success

  # Still _default -- no Elixir/Rust/Swift template selection.
  if grep -qF "Core Elixir Rules" "$PROJ_DIR/intent/llm/RULES.md"; then
    fail "polyglot project picked Elixir template (canon-installer must always use _default)"
  fi
  assert_file_contains "$PROJ_DIR/intent/llm/RULES.md" "intent claude rules list --lang <lang>"
}

@test "--dry-run does not modify the filesystem" {
  init_scratch dry

  before="$(tree_snapshot "$PROJ_DIR")"

  run run_intent claude upgrade
  assert_success

  after="$(tree_snapshot "$PROJ_DIR")"
  if [ "$before" != "$after" ]; then
    diff <(echo "$before") <(echo "$after")
    fail "tree changed on dry-run"
  fi

  assert_file_not_exists "$PROJ_DIR/.claude/settings.json"
  assert_file_not_exists "$PROJ_DIR/.intent_critic.yml"
  assert_file_not_exists "$PROJ_DIR/usage-rules.md"
}

# ST0043 AT-01.8 (red-first): T11/T12 portability. The canon engine must not use
# BSD-only `sed -i ''` (it breaks GNU/Linux upgrades). The unanchored CLAUDE.md
# version-sed dies with the VERSION_BUMP path (AT-01.6), so "anchored" reduces to
# "no version-sed at all". RED until the in-place edit is made portable.
@test "sed edits are portable and the version handling is anchored" {
  local canon="${INTENT_PROJECT_ROOT}/intent/plugins/claude/bin/intent_claude_upgrade"
  if grep -nE "sed -i ''" "$canon"; then
    fail "BSD-only 'sed -i \\'\\'' is non-portable; use a portable in-place edit"
  fi
}

# --- FENCES (2026-08-24 config sweep) -------------------------------------

# The canon engine had NO version check of any kind while its orchestrator did.
# The AGENTS.md probe tested `[ "$local" = "$TARGET" ]`, and EQUALITY HAS NO
# DIRECTION, so a project AHEAD of the installed tool was regenerated BACKWARDS
# and reported as a routine refresh. `version_gt` was already exported from
# intent_helpers and this file already sourced it -- the helper existed and this
# path simply never called it, which is why "extract a shared helper" would have
# prevented nothing. The fence has to be a CALL-SITE test.
@test "canon engine REFUSES a project whose canon is ahead of this tool" {
  init_scratch guard
  local ahead; ahead="99.0.0"
  jq --arg v "$ahead" '.intent_version = $v' intent/.config/config.json > c.tmp && mv c.tmp intent/.config/config.json

  run run_intent claude upgrade --project-dir .
  [ "$status" -ne 0 ] || fail "engine planned work against a newer project instead of refusing"
  [[ "$output" == *"refusing downgrade"* ]] || fail "refusal did not name the reason: $output"
  [[ "$output" == *"$ahead"* ]] || fail "refusal did not name the project version: $output"
}

# The refusal must name WHICH install is stale. Every project on this machine
# resolves `intent` through $INTENT_HOME, so "upgrade the tool" is useless
# advice unless the operator can see which tool answered.
@test "the downgrade refusal names the resolved install so the operator can act" {
  init_scratch guard
  jq '.intent_version = "99.0.0"' intent/.config/config.json > c.tmp && mv c.tmp intent/.config/config.json

  run run_intent claude upgrade --project-dir .
  [[ "$output" == *"resolved install:"* ]] || fail "refusal did not name the install: $output"
}

# POSITIVE CONTROL: the guard must not refuse everything. A project at or below
# the tool's version still upgrades, or the fence above passes trivially.
@test "positive control: a project at the tool version is NOT refused" {
  init_scratch guard
  local target; target="$(cat "${INTENT_PROJECT_ROOT}/VERSION")"
  jq --arg v "$target" '.intent_version = $v' intent/.config/config.json > c.tmp && mv c.tmp intent/.config/config.json

  run run_intent claude upgrade --project-dir .
  assert_success
  [[ "$output" != *"refusing downgrade"* ]] || fail "guard refused a same-version project: $output"
}

# Skills are SKILL.md PLUS scripts/ and data/, and the scripts are where the
# behaviour lives. Checksumming SKILL.md alone reported UP TO DATE while an
# installed script differed from canon; the only way out was --force or touching
# SKILL.md. Measured 2026-08-24 when in-autopsy's script fix hit exactly that.
@test "skill drift is detected from a SCRIPT-only change, not just SKILL.md" {
  init_scratch guard
  local installed="$HOME/.claude/skills/in-autopsy"
  local canon="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-autopsy"
  [ -d "$canon" ] || skip "in-autopsy canon not present"

  mkdir -p "$installed/scripts"
  cp "$canon/SKILL.md" "$installed/SKILL.md"
  cp -R "$canon/scripts/." "$installed/scripts/" 2>/dev/null || true

  run run_intent claude upgrade --project-dir .
  [[ "$output" != *"in-autopsy:"*"OUTDATED"* ]] || fail "reported drift on an identical copy: $output"

  # Perturb a SCRIPT ONLY. SKILL.md stays byte-identical.
  echo "# drift introduced by a script-only edit" >> "$installed/scripts/autopsy.exs"
  cmp -s "$canon/SKILL.md" "$installed/SKILL.md" || fail "test bug: SKILL.md must stay identical"

  run run_intent claude upgrade --project-dir .
  [[ "$output" == *"in-autopsy"* ]] || fail "in-autopsy absent from the probe: $output"
  [[ "$output" == *"OUTDATED"* ]] || fail "script-only drift not detected -- checksum is SKILL.md-only again: $output"
}

# Every action the canon engine can enqueue must carry a DECLARED DISPOSITION.
#
# The orphaned-scripts class ran for four months in silence: issue 0016 moved
# hook dispatch to the CLI, the install step stayed, and nothing anywhere said
# whether a written artefact was meant to persist or to be cleaned up. The rule
# is NOT "every write needs a prune" -- most canon artefacts should persist.
# The rule is that the ANSWER must be written down, so the next artefact whose
# reason for existing disappears reddens here instead of lingering.
#
# Adding an action without classifying it fails this test. That is the point:
# the roster is deliberately manual, because "should this persist?" is a
# judgement no grep can make.
@test "every canon action has a declared disposition (persist or prune)" {
  local engine="${INTENT_PROJECT_ROOT}/intent/plugins/claude/bin/intent_claude_upgrade"

  # PERSIST: the artefact is canon the project is meant to keep.
  local persist="CHAIN_PRE_COMMIT CHAIN_PRE_COMMIT_BLOCK CREATE INSTALL_CLAUDE_MD \
INSTALL_CRITIC_CONFIG INSTALL_PRE_COMMIT INSTALL_SETTINGS INSTALL_TREEINDEXIGNORE \
INSTALL_USAGE_RULES MERGE MIGRATE_LEGACY_PRE_COMMIT NORMALIZE_GITIGNORE \
PLANT_DECISION_TREE PLANT_MODULES REFRESH_CLAUDE_MD REGENERATE RENAME_SKILL \
UPDATE_SKILL UPDATE_SUBAGENT"

  # REMOVES: the action's whole job is taking something away.
  local removes="DELETE DELETE_LEGACY_AGENTS PRUNE_HOOK_SCRIPT"

  local declared=" $persist $removes "
  local undeclared=""
  local a
  for a in $(grep -ohE 'add_action "[A-Z_]+' "$engine" | sed 's/add_action "//' | sort -u); do
    case "$declared" in
      *" $a "*) ;;
      *) undeclared="$undeclared $a" ;;
    esac
  done

  [ -z "$undeclared" ] || fail "canon actions with no declared disposition:$undeclared
Add each to the persist or removes roster in this test. If an artefact is
written but nothing will ever remove it, say so explicitly -- that is the
declaration. Silence is how the orphaned hook scripts survived four months."
}

# The pruner must name its targets explicitly, never sweep a directory.
# A project may keep its own hooks in .claude/scripts/ (Lamplight carries an
# unwired fmt-md-on-write.sh), and a blanket sweep would take them too.
@test "the hook-script pruner names its three targets and does not sweep" {
  local engine="${INTENT_PROJECT_ROOT}/intent/plugins/claude/bin/intent_claude_upgrade"
  grep -q 'for _script in session-context.sh require-in-session.sh post-tool-advisory.sh' "$engine" \
    || fail "pruner no longer enumerates its three targets explicitly"
  grep -qE 'rm -rf .*\.claude/scripts' "$engine" \
    && fail "pruner sweeps the directory; a project's own scripts would be destroyed"
  return 0
}

# **THE TOOL MUST RECOGNISE ITS OWN OUTPUT, AND NOTHING ASSERTED THAT UNTIL NOW.**
# `CANON_INTENT_FOOTER_MARK` is how `intent claude upgrade` tells a generated
# CLAUDE.md from a user-authored one. It was a CONTIGUOUS substring of the
# footer's PROSE, so a pure wording edit to `_CLAUDE.md` silently broke it:
# every project upgrading afterwards had its CLAUDE.md permanently reclassified
# as "user-authored (left alone)" and no canon change ever reached it again.
#
# **IT FAILED AS A LEGITIMATE MESSAGE.** "user-authored (left alone)" is a real
# and correct state -- quoted approvingly in two v2.10.0 canary reports -- so
# nothing in the output distinguished the defect from the design. And the only
# remedy the tool offers is `--force`, which destroys the user:start block, so a
# consumer following its advice loses their own directives.
#
# The mark is READ FROM THE ENGINE rather than pinned as a literal here, so this
# tests the RELATIONSHIP and no rename can make it pass while being false.
@test "the footer mark matches the shipped template -- the tool recognises its own output" {
  local engine="${INTENT_PROJECT_ROOT}/intent/plugins/claude/bin/intent_claude_upgrade"
  local tmpl="${INTENT_PROJECT_ROOT}/lib/templates/llm/_CLAUDE.md"
  local line
  line="$(grep -E '^CANON_INTENT_FOOTER_MARK=' "$engine")" \
    || fail "no CANON_INTENT_FOOTER_MARK assignment in the engine"
  # Evaluate the assignment so the value tested is the one the tool actually
  # uses, escapes and all -- extracting it textually would compare a string the
  # running shell never sees.
  eval "$line"
  [ -n "${CANON_INTENT_FOOTER_MARK:-}" ] || fail "CANON_INTENT_FOOTER_MARK is empty"

  grep -qF -- "$CANON_INTENT_FOOTER_MARK" "$tmpl" \
    || fail "the shipped _CLAUDE.md template does not contain the marker the tool
uses to recognise its own output. Every project that upgrades will have its
CLAUDE.md declassified as user-authored and will never receive canon again.
  mark:     $CANON_INTENT_FOOTER_MARK
  template: $tmpl
Make the mark an invariant of the footer (the template PATH survives any
rewording), never a substring of its prose."

  # A project that has NOT yet upgraded still carries the historical footer.
  # If the mark stops matching that, this fix declassifies exactly the projects
  # it was meant to rescue.
  local historic='_Generated from `lib/templates/llm/_CLAUDE.md` on 2026-08-14 for Intent v2.19.0._'
  printf '%s\n' "$historic" | grep -qF -- "$CANON_INTENT_FOOTER_MARK" \
    || fail "the mark no longer matches the pre-existing footer wording; projects
that have not upgraded yet would be declassified by this change."
}

# **THE SECOND READER OF THE SAME FOOTER FAMILY (vc, flagged rather than filed).**
# `:751` extracts a version from AGENTS.md with its own regex over the same
# "_Generated by Intent v..." prose. It is a different consumer of the same
# string family, and the CLAUDE.md mark broke precisely because nobody
# enumerated the readers of a string everyone was treating as prose. So it is
# asserted here rather than discovered next time.
#
# **KNOWN AND DELIBERATELY NOT ASSERTED: this regex TRUNCATES a pre-release
# suffix.** `v3.0.0-dev` yields `3.0.0`, so a pre-release AGENTS.md reads as the
# release. That is a live property of the v2 tool's version comparison, not of
# this fix, and pinning it here would freeze a behaviour nobody has ruled on.
@test "the AGENTS.md version reader still matches the shipped _AGENTS.md footer form" {
  local engine="${INTENT_PROJECT_ROOT}/intent/plugins/claude/bin/intent_claude_upgrade"
  local pattern
  pattern="$(grep -oE "_Generated by Intent v\\[0-9\\]\\+[^']*" "$engine" | head -1)"
  [ -n "$pattern" ] || fail "the AGENTS.md version-reader regex is no longer at its site"

  # Both shipped renderings of the _AGENTS.md footer must be readable by it.
  printf '%s\n' '_Generated by Intent v2.19.0 on 2026-08-25_' \
    | grep -qE "$pattern" || fail "the v2 AGENTS.md footer form is no longer matched"
  printf '%s\n' '_Generated by Intent v3.0.0-dev from `lib/templates/llm/_AGENTS.md`._' \
    | grep -qE "$pattern" || fail "the v3 AGENTS.md footer form is no longer matched"
}
