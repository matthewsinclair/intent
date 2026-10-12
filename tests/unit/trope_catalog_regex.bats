#!/usr/bin/env bats
# The trope catalogue's regexes, run the way their consumer runs them.
# IN-PR-STYLE-004 and critic-prose apply each `**Regex**:` line with
# `LC_ALL=C.UTF-8 grep -iE`, so every arm here does exactly that, on text whose
# curly characters are built from octal bytes so that this file stays ASCII.
#
# Issue 0589: FMT-03's class held straight quotes where curly ones were meant,
# so it flagged every ASCII quote and missed every curly one, and every
# apostrophe or quote class in a phrase rule held only the ASCII form, so the
# phrase typed with U+2019 or U+201D went unflagged. A phrase rule must match
# both forms, so its straight form is a second positive; the ASCII negative
# belongs to FMT-03 alone. The characters are literal in the catalogue because
# no grep honours a backslash-u escape: one in a bracket is read as the
# characters it is spelled with.

load "../lib/test_helper.bash"

CATALOG="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-detrope/data/trope-catalog.md"

LDQUO=$(printf '\342\200\234') # U+201C
RDQUO=$(printf '\342\200\235') # U+201D
LSQUO=$(printf '\342\200\230') # U+2018
RSQUO=$(printf '\342\200\231') # U+2019
APOS="'"
DQUOTE='"'

utf8_grep() {
  LC_ALL=C.UTF-8 grep "$@"
}

# Without a UTF-8 locale grep reads a multibyte character byte by byte, and
# every arm below would measure the locale instead of the catalogue. So the
# arm stops here rather than blame a regex.
require_utf8_grep() {
  printf 'x%sy\n' "$RSQUO" | utf8_grep -qE "x[${RSQUO}]y" && return 0
  echo "grep under LC_ALL=C.UTF-8 does not read U+2019 as one character, so"
  echo "the catalogue's quote classes cannot be measured on this machine."
  return 1
}

# The `**Regex**:` pattern of the trope whose frontmatter says `id: $1`, with
# the PCRE `(?i)` prefix dropped as IN-PR-STYLE-004 says (`-i` covers it).
catalog_regex() {
  local pattern
  pattern=$(awk -v id="$1" '
    $0 == "id: " id { found = 1; next }
    found && /^- \*\*Regex\*\*: `/ {
      sub(/^- \*\*Regex\*\*: `/, ""); sub(/`$/, ""); print; exit
    }' "$CATALOG")
  printf '%s' "${pattern#'(?i)'}"
}

# 0 when trope $1's regex matches the line $2 as the consumer runs it, 1 when
# it does not, and 2 when the catalogue has no pattern for $1, so a missing
# pattern fails a negative arm as well as a positive one.
trope_matches() {
  local pattern
  pattern=$(catalog_regex "$1")
  if [ -z "$pattern" ]; then
    echo "no Regex line under id: $1 in $CATALOG"
    return 2
  fi
  printf '%s\n' "$2" | utf8_grep -qiE -e "$pattern"
}

# id|kind|a line carrying the trope. For kind apos, @ marks its apostrophe;
# for kind dquote, ^ and @ mark its opening and closing double quotes.
quote_samples() {
  cat <<'EOF'
TN-01|apos|Here@s the thing about caching.
TN-02|apos|It@s like a library for your data.
TN-04|apos|I@ll admit the first draft was rough.
TN-04|apos|If I@m being honest, it took a week.
TN-05|apos|It@s clear that the cache is the bottleneck.
TN-07|apos|Let@s unpack this one step at a time.
ASR-01|apos|I@m an AI, so I cannot taste coffee.
ASR-02|apos|You@re playing a rogue in this campaign.
ASR-03|apos|I@ve been instructed to keep it short.
CMP-01|apos|As we@ve seen, the cache is the bottleneck.
NAR-01|apos|I@ve been expecting you, Mr Bond.
NAR-02|apos|If I@m being honest, the tests were thin.
SS-01|apos|It@s not the cache, it@s the network.
SS-04|apos|It@s worth noting that the API changed.
NAR-04|dquote|^I know,@ she breathed.
EOF
}

# The sample $2 of kind $1 typed with typeset quotes.
curly_form() {
  local s="$2"
  case "$1" in
    apos) s=${s//@/$RSQUO} ;;
    dquote) s=${s//^/$LDQUO} && s=${s//@/$RDQUO} ;;
  esac
  printf '%s' "$s"
}

# The sample $2 of kind $1 typed with ASCII quotes.
straight_form() {
  local s="$2"
  case "$1" in
    apos) s=${s//@/$APOS} ;;
    dquote) s=${s//^/$DQUOTE} && s=${s//@/$DQUOTE} ;;
  esac
  printf '%s' "$s"
}

@test "trope catalog: FMT-03 flags each curly quote (0589)" {
  require_utf8_grep
  local q
  for q in "$LDQUO" "$RDQUO" "$LSQUO" "$RSQUO"; do
    run trope_matches FMT-03 "a typeset ${q}quote${q} in a README"
    [ "$status" -eq 0 ] || fail "FMT-03 missed the curly quote in: a typeset ${q}quote${q} in a README (status $status) $output"
  done
}

@test "trope catalog: FMT-03 leaves ASCII quotes alone (0589)" {
  require_utf8_grep
  run trope_matches FMT-03 "don't say \"yes\" or 'no'"
  [ "$status" -eq 1 ] || fail "FMT-03 flagged plain ASCII quotes (status $status) $output"
}

@test "trope catalog: every phrase rule matches its phrase typed with curly quotes (0589)" {
  require_utf8_grep
  local id kind sample line
  while IFS='|' read -r id kind sample; do
    line=$(curly_form "$kind" "$sample")
    run trope_matches "$id" "$line"
    [ "$status" -eq 0 ] || fail "$id missed its phrase typed with curly quotes (status $status): $line $output"
  done < <(quote_samples)
}

@test "trope catalog: every phrase rule still matches its phrase typed with ASCII quotes (0589)" {
  require_utf8_grep
  local id kind sample line
  while IFS='|' read -r id kind sample; do
    line=$(straight_form "$kind" "$sample")
    run trope_matches "$id" "$line"
    [ "$status" -eq 0 ] || fail "$id missed its phrase typed with ASCII quotes (status $status): $line $output"
  done < <(quote_samples)
}
