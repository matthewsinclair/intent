#!/bin/bash
# critic_runner.sh -- Intent headless critic runner primitives (ST0035/WP-05)
#
# Responsibility:
#   Load language-specific rules from canon + extensions, extract each
#   rule's Greppable proxy from its Detection section, run the pattern
#   against target files, and emit severity-grouped findings in text or
#   JSON. Never shells out to an LLM; never autofixes.
#
# Consumers:
#   - bin/intent_critic (thin CLI dispatch)
#
# Extends (via source):
#   - intent/plugins/claude/lib/rules_lib.sh
#
# The mechanical subset: rules that publish a "Greppable proxy" fenced
# bash block inside their Detection section are runnable by this script.
# Rules without a greppable proxy are skipped silently (they remain
# available via the `critic-<lang>` LLM subagents for richer review).

if [ -z "${INTENT_HOME:-}" ]; then
  _critic_runner_self="${BASH_SOURCE[0]}"
  while [ -L "$_critic_runner_self" ]; do
    _critic_runner_self="$(readlink "$_critic_runner_self")"
  done
  INTENT_HOME="$(cd "$(dirname "$_critic_runner_self")/../../../.." && pwd)"
  export INTENT_HOME
  unset _critic_runner_self
fi

# Source the shared rule-library primitives if not already loaded.
if ! declare -f rule_frontmatter >/dev/null 2>&1; then
  source "$INTENT_HOME/intent/plugins/claude/lib/rules_lib.sh"
fi

# ---- Language registry (Highlander: one source for the whole toolchain) ----
# The languages with a mechanical, headless CODE critic. `intent critic`
# validates its positional argument against this set, and the pre-commit gate
# consults it (via `intent critic --languages`) to decide which declared
# languages to run -- so the accepted set lives in exactly one place and the
# CLI and the gate cannot drift (issue 0003).
critic_code_languages() {
  printf '%s\n' elixir rust swift lua shell
}

# Prose / on-demand disciplines: valid declared languages whose critique is the
# critic-prose SUBAGENT (LLM judgement over .md / .mdx / .html files), never the
# headless runner. `intent critic <prose-lang>` is a clean no-op and the gate
# skips them, so declaring author / content never errors the commit gate (0003).
critic_prose_languages() {
  printf '%s\n' author content
}

# Membership predicates over the two registries above -- exact whole-line match.
critic_is_code_language() {
  critic_code_languages | grep -qxF "$1"
}
critic_is_prose_language() {
  critic_prose_languages | grep -qxF "$1"
}

# Severity priority (low → high): style, recommendation, warning, critical.
# Used for --severity-min filtering and report ordering.
critic_severity_rank() {
  case "$1" in
    critical)       echo 4 ;;
    warning)        echo 3 ;;
    recommendation) echo 2 ;;
    style)          echo 1 ;;
    *)              echo 0 ;;
  esac
}

# Project-root resolution uses find_project_root from bin/intent_helpers
# (sourced via rules_lib.sh -- Highlander, ST0042/WP-05).

# Extract the first fenced bash block inside the `## Detection` section of
# a RULE.md file, filtered to the one following a `Greppable proxy` line.
# Echoes the bash command(s); empty output means "no greppable proxy".
critic_extract_greppable_block() {
  local path="$1"
  awk '
    /^## Detection[[:space:]]*$/ { in_det = 1; next }
    /^## / && in_det { exit }
    in_det && /[Gg]reppable proxy/ { after_proxy = 1; next }
    in_det && after_proxy && /^```bash[[:space:]]*$/ { in_block = 1; next }
    in_det && in_block && /^```[[:space:]]*$/ { exit }
    in_det && in_block { print }
  ' "$path"
}

# True (return 0) iff the input line is a single, simple `grep` invocation
# the headless runner can faithfully execute. The accepted shape is:
#
#   grep [-r|-n|-E|-rn|-rE|-nE|-rnE|--include=GLOB ...] '<pattern>' [<path>...]
#
# - Exactly one `grep` invocation, no pipes or chained commands.
# - Allowed flag clusters are drawn from {r, n, E} only; -L, -v, -B, -A,
#   -l, -c, -o, -w, -x and bare -B5 / -A2 forms are rejected.
# - Pattern is single-quoted; metacharacters such as | inside the pattern
#   are fine (they are part of the regex, not the surrounding shell).
# - Path args after the pattern must not contain shell metacharacters
#   (|, ;, &, <, >, $, `, '), preventing pipelines disguised as args.
# - Empty lines and `#` comments return 1 (not a grep candidate; caller
#   skips silently — distinct from "rejected").
critic_proxy_is_simple() {
  local line="$1"
  local trimmed
  trimmed="${line#"${line%%[![:space:]]*}"}"
  trimmed="${trimmed%"${trimmed##*[![:space:]]}"}"
  [ -z "$trimmed" ] && return 1
  case "$trimmed" in "#"*) return 1 ;; esac
  local flag='(-[rnE]+|--include=[^[:space:]]+)'
  local arg='[^[:space:]'\''|;&<>$`]+'
  local re="^grep([[:space:]]+$flag)*[[:space:]]+'[^']*'([[:space:]]+$arg)*[[:space:]]*$"
  if [[ "$trimmed" =~ $re ]]; then
    return 0
  fi
  return 1
}

# Walk the Greppable proxy block of a rule and emit one acceptable regex
# per line on stdout. For each line that is a grep invocation but not
# headless-runnable, emit one stderr diagnostic per rule (deduped):
#   note: skipping <rule_id> (proxy not headless-runnable)
# Empty / comment lines are silently skipped.
critic_patterns_from_grep_block() {
  local rule_path="$1"
  local rule_id="$2"
  local block
  block="$(critic_extract_greppable_block "$rule_path")"
  [ -z "$block" ] && return 0
  local line trimmed pattern refused_emitted=0
  while IFS= read -r line; do
    trimmed="${line#"${line%%[![:space:]]*}"}"
    [ -z "$trimmed" ] && continue
    case "$trimmed" in "#"*) continue ;; esac
    if critic_proxy_is_simple "$line"; then
      pattern="$(printf '%s' "$line" | sed -n "s/[^']*'\\([^']*\\)'.*/\\1/p" | head -1)"
      [ -n "$pattern" ] && printf '%s\n' "$pattern"
    else
      if [ "$refused_emitted" -eq 0 ]; then
        printf 'note: skipping %s (proxy not headless-runnable)\n' "$rule_id" >&2
        refused_emitted=1
      fi
    fi
  done <<< "$block"
  return 0
}

# Load rule paths applicable to the given language. Agnostic rules are
# intentionally excluded (they are always concretised by language rules;
# direct application would double-report). Emits RULE.md paths on stdout.
critic_load_rule_paths() {
  local lang="$1"
  local path file_lang
  while IFS= read -r path; do
    [ -z "$path" ] && continue
    file_lang="$(rule_fm_scalar "$path" language)"
    case "$file_lang" in
      "$lang") printf '%s\n' "$path" ;;
    esac
  done < <(enumerate_all_rule_files)
}

# THE ARMING CENSUS -- what the gate could not ask, reported rather than skipped.
#
# `:18` of this file said rules without a greppable proxy are "skipped
# silently", and that sentence was a NO-SILENT violation (IN-AG-NO-SILENT-001)
# sitting in the rule library itself. Measured 2026-08-18: ALL 13 shell and rust
# rules carried no proxy and no declaration, so `intent critic shell` returned
# rc=0 having asked nothing, and its output was INDISTINGUISHABLE from `intent
# critic elixir` after asking nine real questions. Both printed `ok: no <lang>
# findings`.
#
# vc's ruling, 2026-08-18: REPORTING IS UNCONDITIONAL AND IS NOT PART OF ANY
# TRADE-OFF. It costs zero fleet breakage -- reporting is not refusing -- and it
# is owed whether or not any rule is ever armed with a real tool. Refusal is a
# separate axis, scoped to rules a project has armed via `.intent_critic.yml`.
#
# TWO AXES, NOT ONE, AND THE SECOND ONE IS WHY THIS WAS REWRITTEN (vc, ruling
# dc 2026-08-19). Every row is `<rule_id> <arming> <disposition> <by>`.
#
#   ARMING -- a property of the RULE and the project's config. Whether anything
#   COULD answer this rule.
#     armed       a greppable block with a runnable line, or a named tool
#     declared    an explicit "No greppable proxy is authoritative for this rule"
#     unrunnable  a block whose every line the runner must refuse (ST0039)
#     undeclared  none of the above -- nobody has decided
#
#   DISPOSITION -- a property of THIS INVOCATION. Whether it actually asked.
#     ran                     the question was put to this file
#     not-run:tool-absent     armed on a tool this machine does not have
#     not-run:out-of-context  armed on a tool that does not belong in this run
#     n-a                     nothing to run; the arming axis already said why
#
# THE SECOND AXIS EXISTS BECAUSE THREE OF THE FIVE TOOL-ARMED RULES ARE CLIPPY
# RULES, AND `cargo clippy` IS A WHOLE-WORKSPACE COMPILE THAT DOES NOT BELONG IN
# A PER-COMMIT HOOK AT ANY ARMING MODE. Collapsing that into `declared` hides a
# real capability behind a word meaning nothing can ever answer this; collapsing
# it into a bare `armed` reports a question this run did not ask -- which is this
# census's founding defect with a friendlier name and a better reason. `armed` +
# `not-run:out-of-context` says both true things at once.
#
# A FIFTH ARMING VALUE WAS THE FIRST DESIGN AND IT WAS WRONG: it would put a
# property of the INVOCATION into a key whose other members are properties of the
# rule, and then `armed` would mean two things depending on which member you read.
critic_arming_census() {
  local lang="$1" path rule_id block pats tool ctx
  while IFS= read -r path; do
    [ -z "$path" ] && continue
    rule_id="$(rule_fm_scalar "$path" id)"
    [ -n "$rule_id" ] || rule_id="$path"
    tool="$(rule_fm_scalar "$path" critic_tool)"
    if [ -n "$tool" ]; then
      ctx="$(rule_fm_scalar "$path" critic_tool_context)"
      ctx="${ctx:-per-file}"
      # CONTEXT IS TESTED BEFORE AVAILABILITY, AND THE ORDER IS THE RULING.
      # A tool this run was never going to invoke cannot be blamed on the
      # machine: reporting `tool-absent` for a workspace analyser during a
      # per-file run states a fact about the host in answer to a question about
      # the invocation, and the reader would go install something that would
      # still not run. Absence matters in the run that WOULD have used it.
      if [ "$ctx" != "per-file" ]; then
        printf '%s armed not-run:out-of-context %s\n' "$rule_id" "$tool"
      elif ! critic_tool_available "$tool"; then
        printf '%s armed not-run:tool-absent %s\n' "$rule_id" "$tool"
      else
        printf '%s armed ran %s\n' "$rule_id" "$tool"
      fi
      continue
    fi
    block="$(critic_extract_greppable_block "$path")"
    if [ -n "$block" ]; then
      pats="$(critic_patterns_from_grep_block "$path" "$rule_id" 2>/dev/null)"
      if [ -n "$pats" ]; then printf '%s armed ran grep\n' "$rule_id"
      else printf '%s unrunnable n-a -\n' "$rule_id"; fi
    elif grep -qi 'No greppable proxy is authoritative' "$path"; then
      printf '%s declared n-a -\n' "$rule_id"
    else
      printf '%s undeclared n-a -\n' "$rule_id"
    fi
  done < <(critic_load_rule_paths "$lang")
}

# ---------------------------------------------------------------------------
# NAMED-TOOL ARMING -- the rule says WHICH tool; the runner owns HOW.
#
# hv authorised the runner to use shellcheck and clippy; vc ruled the shape, and
# the shape is an OBLIGATION rather than a widening. A rule file names a tool and
# the diagnostics of that tool which ANSWER it. **RULE FILES NEVER CONTRIBUTE
# SHELL, EVER.** The invocation lives here, once per tool, auditable, Highlander
# -- and `critic_proxy_is_simple` is NOT relaxed, because it is an injection
# boundary ("preventing pipelines disguised as args") and relaxing it is the
# correct-seeming, tidy-looking edit that opens exactly the hole it exists to
# close.
#
# NAMING A PARSER IS THE START OF THE QUESTION, NOT THE END OF IT. 7 of the 13
# shell and rust rules name a real parser in their own Detection text and only 5
# have a tool whose output actually ANSWERS them; the other two name tools that
# answer ADJACENT propositions. A proxy is not the parser, and a NAMED parser is
# not necessarily an answer either.
# ---------------------------------------------------------------------------

# IS THE NAMED TOOL DRIVEABLE ON THIS MACHINE? Per-tool, because the probe is
# not uniform and a uniform one is WRONG: `clippy` is not a command. It ships as
# the cargo subcommand `cargo-clippy`, so `command -v clippy` returns false on a
# machine with a perfectly good clippy -- which is what the first version of this
# function did, and it reported all three clippy rules as `not-run:tool-absent`
# on a machine where `cargo clippy --version` prints 0.1.97. **A probe that
# measures the wrong name reports a real capability as missing, and the output
# reads exactly like a true absence.**
critic_tool_available() {
  case "$1" in
    clippy) command -v cargo-clippy >/dev/null 2>&1 ;;
    *)      command -v "$1" >/dev/null 2>&1 ;;
  esac
}

# shellcheck: ONE invocation site. `--format=gcc` gives `path:line:col: sev:
# message [SCxxxx]`, and the rule's declared codes select which of them are
# this rule's business -- without that, IN-SH-CODE-001 and IN-SH-CODE-002 would
# each claim every shellcheck finding.
#
# NO `--enable`, NO `--severity`, NO `--shell`: a flag that changes what the
# tool SEES belongs to the runner's judgement, and IN-RS-CODE-001's case (where
# the remedy is the ABSENCE of `--all-targets`) is why a rule file must never
# supply one.
critic_shellcheck_findings() {
  local file="$1" rule_id="$2" severity="$3" codes="$4"
  local out code_re line_no content
  code_re="$(printf '%s' "$codes" | tr ' ' '|')"
  [ -n "$code_re" ] || return 0
  out="$(shellcheck --format=gcc "$file" 2>/dev/null || true)"
  [ -n "$out" ] || return 0
  # DEDUPED ON THE LINE, matching the grep path's dedup on (line, content).
  # shellcheck reports per COLUMN, so one line carrying two defects this rule
  # owns -- `for f in ${arr[@]}; do echo $f; done` is SC2068 at col 10 and
  # SC2086 at col 29 -- arrives as two findings whose rendered output is the
  # same line printed twice. Two identical lines read as two defects, and a
  # count taken off them overstates.
  printf '%s\n' "$out" | grep -E "\[($code_re)\]" | awk -F: 'NF > 1 && !seen[$2]++ { print $2 }' | while IFS= read -r line_no; do
    [ -n "$line_no" ] || continue
    content="$(sed -n "${line_no}p" "$file" 2>/dev/null | sed 's/\t/    /g' | cut -c1-200)"
    printf '%s\t%s\t%s\t%s\t%s\n' "$severity" "$rule_id" "$file" "$line_no" "$content"
  done
}

# Dispatch. An unknown tool name is a REFUSAL and never a silent skip: a rule
# naming a tool the runner cannot drive is exactly the state this whole census
# exists to make visible.
critic_tool_findings() {
  local tool="$1" file="$2" rule_id="$3" severity="$4" codes="$5"
  case "$tool" in
    shellcheck) critic_shellcheck_findings "$file" "$rule_id" "$severity" "$codes" ;;
    clippy)     return 0 ;;
    *)
      printf 'critic: %s names tool `%s`, which this runner cannot drive -- the rule is UNENFORCED\n' \
        "$rule_id" "$tool" >&2
      return 0
      ;;
  esac
}

# Check whether a rule is disabled in the given .intent_critic.yml file.
# Looks for a flat `disabled:` list with the rule ID. Returns 0 if
# disabled, 1 otherwise. Field name aligns with intent/docs/critics.md
# and the canonical sample at
# intent/plugins/claude/rules/_schema/sample-intent-critic.yml.
#
# Awk emits exit code 10 on match; bash distinguishes this from awk's
# natural exit 0 (finished without matching).
critic_rule_disabled() {
  local rule_id="$1"
  local config="$2"
  [ -z "$config" ] && return 1
  [ -f "$config" ] || return 1
  awk -v id="$rule_id" '
    BEGIN { inside = 0 }
    /^disabled:[[:space:]]*\[/ {
      line = $0
      sub(".*\\[", "", line)
      sub("\\].*", "", line)
      n = split(line, parts, ",")
      for (i = 1; i <= n; i++) {
        it = parts[i]
        gsub("[[:space:]\"'\'']", "", it)
        sub("#.*$", "", it)
        if (it == id) { exit 10 }
      }
      next
    }
    /^disabled:[[:space:]]*$/ { inside = 1; next }
    inside == 1 && /^[[:space:]]+-/ {
      it = $0
      sub("^[[:space:]]+-[[:space:]]*", "", it)
      sub("#.*$", "", it)
      gsub("[[:space:]\"'\'']", "", it)
      if (it == id) { exit 10 }
      next
    }
    inside == 1 && /^[A-Za-z]/ { inside = 0 }
  ' "$config"
  local rc=$?
  [ "$rc" -eq 10 ] && return 0
  return 1
}

# Apply a single rule to a single file. Emits one finding per grep match
# on stdout, in a tab-delimited internal format:
#   <severity>\t<rule_id>\t<file>\t<line>\t<excerpt>
# Summary is not included here; report-formatters fetch it separately via
# rule_fm_scalar to avoid re-parsing on every finding.
# Convert a glob (lib/**/*.ex, test/**/*_test.exs) to a regex anchored to
# allow umbrella prefixes. The result is wrapped in (^|/)<regex>$ so that
# the glob matches both top-level (`lib/foo.ex`) and umbrella-nested paths
# (`apps/control/lib/foo.ex`). Single * matches a single path component;
# ** matches any depth (including zero).
critic_glob_to_regex() {
  local glob="$1"
  printf '%s' "$glob" | awk '
    {
      gsub(/\./, "\\.", $0)
      gsub(/\*\*\//, "DOUBLESTARSLASH", $0)
      gsub(/\*\*/, "DOUBLESTAR", $0)
      gsub(/\*/, "[^/]*", $0)
      gsub(/DOUBLESTARSLASH/, "(.*/)?", $0)
      gsub(/DOUBLESTAR/, ".*", $0)
      print
    }
  '
}

# Return 0 if the file matches the rule's `applies_to` globs (or if no
# `applies_to` is declared, in which case the rule is universal). Return 1
# if `applies_to` is declared but no glob matches.
critic_rule_applies_to_file() {
  local rule_path="$1"
  local file="$2"

  local globs
  globs="$(rule_fm_list "$rule_path" applies_to 2>/dev/null)"
  if [ -z "$globs" ]; then
    return 0
  fi

  local glob regex
  while IFS= read -r glob; do
    [ -z "$glob" ] && continue
    regex="$(critic_glob_to_regex "$glob")"
    if [[ "$file" =~ (^|/)${regex}$ ]]; then
      return 0
    fi
  done <<< "$globs"

  return 1
}

critic_apply_rule() {
  local rule_path="$1"
  local file="$2"

  local rule_id severity rule_status
  rule_id="$(rule_fm_scalar "$rule_path" id)"
  severity="$(rule_fm_scalar "$rule_path" severity)"
  rule_status="$(rule_fm_scalar "$rule_path" status)"
  rule_status="${rule_status:-active}"
  [ "$rule_status" != "active" ] && return 0
  [ -z "$rule_id" ] && return 0
  [ -z "$severity" ] && severity=warning

  # Honour applies_to: if the rule declares one or more globs, the file
  # must match at least one for the rule to fire. Rules without applies_to
  # are universal (current behaviour preserved). Globs use suffix anchoring
  # so umbrella layouts (apps/<app>/lib/..., apps/<app>/test/...) match
  # rules declared as `lib/**/*.ex` / `test/**/*_test.exs` (ST0038).
  if ! critic_rule_applies_to_file "$rule_path" "$file"; then
    return 0
  fi

  # NAMED-TOOL ARM -- takes precedence over a greppable block, and a rule
  # carrying both is a rule that has not decided. The tool is driven only when
  # it is present AND this run is its context; both other cases are reported by
  # the census, never silently skipped here.
  local tool tool_ctx tool_codes
  tool="$(rule_fm_scalar "$rule_path" critic_tool)"
  if [ -n "$tool" ]; then
    tool_ctx="$(rule_fm_scalar "$rule_path" critic_tool_context)"
    tool_ctx="${tool_ctx:-per-file}"
    [ "$tool_ctx" = "per-file" ] || return 0
    critic_tool_available "$tool" || return 0
    tool_codes="$(rule_fm_list "$rule_path" critic_tool_codes | tr '\n' ' ')"
    tool_codes="${tool_codes% }"
    critic_tool_findings "$tool" "$file" "$rule_id" "$severity" "$tool_codes"
    return 0
  fi

  # Multi-pattern union: walk the proxy block, accept simple grep lines,
  # refuse complex ones with a stderr diagnostic. Each accepted pattern is
  # run independently; results are unioned and deduped on (line, content)
  # so two patterns hitting the same line do not double-report.
  local patterns
  patterns="$(critic_patterns_from_grep_block "$rule_path" "$rule_id")"
  [ -z "$patterns" ] && return 0

  local pattern hits results=""
  while IFS= read -r pattern; do
    [ -z "$pattern" ] && continue
    hits="$(grep -nE "$pattern" "$file" 2>/dev/null || true)"
    [ -n "$hits" ] && results+="$hits"$'\n'
  done <<< "$patterns"
  [ -z "$results" ] && return 0

  local line_no content
  printf '%s' "$results" | grep -v '^$' | sort -u | while IFS= read -r grep_line; do
    [ -z "$grep_line" ] && continue
    line_no="${grep_line%%:*}"
    content="${grep_line#*:}"
    content="$(printf '%s' "$content" | sed 's/\t/    /g' | cut -c1-200)"
    printf '%s\t%s\t%s\t%s\t%s\n' "$severity" "$rule_id" "$file" "$line_no" "$content"
  done
}

# Scan a list of files with a language's rule set. Emits tab-delimited
# findings on stdout. Honours .intent_critic.yml disabled_rules if the
# config path is provided.
critic_scan_files() {
  local lang="$1"
  local config="$2"
  shift 2
  local file rule_path rule_id

  local rule_paths
  rule_paths="$(critic_load_rule_paths "$lang")"
  [ -z "$rule_paths" ] && return 0

  for file in "$@"; do
    [ -f "$file" ] || continue
    while IFS= read -r rule_path; do
      [ -z "$rule_path" ] && continue
      rule_id="$(rule_fm_scalar "$rule_path" id)"
      if [ -n "$config" ] && critic_rule_disabled "$rule_id" "$config"; then
        continue
      fi
      critic_apply_rule "$rule_path" "$file"
    done <<< "$rule_paths"
  done
}

# Filter findings by minimum severity. Reads tab-delimited findings on
# stdin; echoes filtered findings on stdout.
critic_filter_severity() {
  local min_rank="$1"
  awk -F'\t' -v min="$min_rank" '
    {
      sev = $1
      rank = 0
      if (sev == "critical")      rank = 4
      else if (sev == "warning")  rank = 3
      else if (sev == "recommendation") rank = 2
      else if (sev == "style")    rank = 1
      if (rank >= min) print
    }
  '
}

# Format findings as human-readable severity-grouped text.
# Reads tab-delimited findings on stdin; looks up rule summary from
# canon on first sighting. Emits empty output when no findings.
critic_format_text() {
  local tmp
  tmp="$(mktemp -t intent-critic.XXXXXX)"
  cat > "$tmp"
  [ ! -s "$tmp" ] && rm -f "$tmp" && return 0

  local sev
  for sev in critical warning recommendation style; do
    local sev_upper count
    sev_upper="$(printf '%s' "$sev" | tr '[:lower:]' '[:upper:]')"
    count="$(awk -F'\t' -v s="$sev" '$1 == s' "$tmp" | wc -l | tr -d '[:space:]')"
    [ "$count" -eq 0 ] && continue
    printf '\n== %s (%d) ==\n' "$sev_upper" "$count"
    awk -F'\t' -v s="$sev" '$1 == s' "$tmp" | while IFS=$'\t' read -r _ rule_id file line excerpt; do
      printf '[%s] %s at %s:%s\n' "$sev_upper" "$rule_id" "$file" "$line"
      printf '  > %s\n' "$excerpt"
    done
  done

  rm -f "$tmp"
}

# Format findings as JSON array. Reads tab-delimited findings on stdin.
# Empty input → `[]`.
critic_format_json() {
  awk -F'\t' '
    BEGIN { first = 1; printf "[" }
    {
      sev = $1
      id = $2
      file = $3
      line = $4
      excerpt = $5
      gsub(/\\/, "\\\\", excerpt)
      gsub(/"/, "\\\"", excerpt)
      gsub(/\\/, "\\\\", file)
      gsub(/"/, "\\\"", file)
      if (!first) printf ","
      printf "\n  {\"severity\":\"%s\",\"rule_id\":\"%s\",\"file\":\"%s\",\"line\":%s,\"excerpt\":\"%s\"}", sev, id, file, line, excerpt
      first = 0
    }
    END {
      if (first) printf "]\n"
      else printf "\n]\n"
    }
  '
}
