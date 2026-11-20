# shellcheck shell=bash
#
# lib_at_canon.sh -- the shell guards' one reading of the thread canon's AT rows
# (issue 0733). SOURCED, never run, by a tool that then calls at_canon_root.
#
# BASH 3.2, NOT 5: the commit gate runs these tools through `#!/usr/bin/env
# bash`, which on a Mac can be /bin/bash 3.2.57, so no associative array,
# mapfile, `local -n` or case-changing expansion here or in either tool. A
# report-only tool that refuses on a missing feature blocks every commit.
#
# Readers: absent_at_check.sh and stale_at_check.sh. Each keeps only its own
# judgement, which rows it examines and what a finding is; how the canon is
# found, read and refused lives here once. Before this file each tool read the
# canon its own way, and they disagreed on what an unreadable row was:
# absent_at let a status of null, a number or a misspelt word fall quietly out
# of its population, and stale_at kept a thread with no status in scope as
# `unknown`.
#
# THE RULE IS NO SILENT ERRORS. Anything this reader cannot read refuses with
# exit status 2 and a remedy, and never reads as clean or as in scope:
#   - no canon directory, or no thread file in it;
#   - a thread file jq cannot parse (each one named, with jq's own message),
#     or one holding no JSON value or more than one;
#   - a thread that is not an object, a thread status outside ThreadStatus, a
#     `tests` that is present and is not an array;
#   - a row that is not an object, an id that is missing, empty, not a string
#     or holds a control character, a kind outside AtKind, a status outside
#     AtStatus, a file that is neither null nor a string without a control
#     character, or that names a line and no file;
#   - a line of the canon pass this reader cannot read.
# A thread with no `tests` key holds no rows and is read as such: 60 of 96
# threads carried none on 2026-11-20, every one a thread with no AT row.
#
# THE VOCABULARY IS THE MODEL'S, READ FROM schema/thread.schema.json, the face
# faces.rs generates from the model and commits. This file spells no copy of
# AtStatus, AtKind or ThreadStatus, so a status the model adds is legal here on
# the day the schema carries it. An enum the schema read cannot find, or finds
# empty, refuses and names the path it looked for: an empty set would let
# every value through, or refuse every row, and either is silent. Each tool
# checks the words it judges by (to-write, green, cancelled) with
# at_canon_legal, so a word the model renames refuses rather than matching
# nothing.
#
# THE API, every function returning 2 on a refusal it has already printed:
#   at_canon_refuse <e> [<r>] prints error and remedy, returns 2
#   at_canon_root <dir>       enters the project root above <dir>
#   at_canon_ready            jq, the vocabulary, the canon directory, and its
#                             thread files into AT_CANON_FILES (top level only:
#                             the canon is written flat, as <ID>.json)
#   at_canon_legal <def> <w>... each word is a value of <def>
#   at_canon_read <file>...   one jq pass; refuses before any callback if
#                             anything is unreadable, then calls the tool's
#                             at_thread <tid> <status> for each file and
#                             at_row <tid> <id> <kind> <status> <file> for each
#                             row, in file order, with a citation's line suffix
#                             taken off the file (lib_citation.sh); sets
#                             AT_CANON_ROWS to the rows read

AT_CANON_DIR="intent/.canon/st"
AT_CANON_SCHEMA="schema/thread.schema.json"
AT_CANON_DEFS="ThreadStatus AtStatus AtKind"

# The one reading of a citation, `<file>` or `<file>:<line>` (issue 0670).
# shellcheck source=lib_citation.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib_citation.sh" || {
  echo "error: cannot read lib_citation.sh beside lib_at_canon.sh" >&2
  return 2
}

at_canon_refuse() {
  echo "error: $1" >&2
  [ -n "${2:-}" ] && echo "remedy: $2" >&2
  return 2
}

# at_canon_root <dir> -- walks up from <dir> to the project root by its marker
# rather than counting `..` levels, and enters it. Counting is what broke
# stale_at_check.sh on its first run: the tools sit five directories down, not
# four, and a wrong count fails as a missing thread rather than a missing root.
at_canon_root() {
  local root
  # Absolute first: `dirname .` is `.`, so a relative walk would never reach /.
  root="$(cd "$1" && pwd)" || {
    at_canon_refuse "cannot enter $1 to look for the project root above it"
    return
  }
  while [ "$root" != "/" ] && [ ! -f "$root/intent/.config/config.json" ]; do
    root="$(dirname "$root")"
  done
  [ -f "$root/intent/.config/config.json" ] || {
    at_canon_refuse "no Intent project root above $1" "run this inside an Intent project"
    return
  }
  cd "$root" || at_canon_refuse "cannot enter the project root $root"
}

# A definition's values: its own enum, or each oneOf or anyOf branch's enum or
# const. AtStatus is written as branches (n-a and fiat carry their own docs),
# ThreadStatus and AtKind as one enum.
# shellcheck disable=SC2016 # jq's $-names, never the shell's
AT_CANON_VOCAB_JQ='
  def vocab:
    if type == "object" then
      [ (., ((.oneOf // .anyOf // [])[]))
        | if type == "object" then (.enum // (if has("const") then [.const] else [] end))[] else empty end
        | strings ]
    else [] end;
  (.["$defs"] // .definitions // {}) as $d
  | reduce ($names | split(" ")[]) as $n ({}; .[$n] = ($d[$n] | vocab))
'

at_canon_vocab() {
  [ -f "$AT_CANON_SCHEMA" ] || {
    at_canon_refuse "no $AT_CANON_SCHEMA to read the canon's vocabulary from" \
      "run this from the root of Intent's own repository, where the model's schema face is committed"
    return
  }
  AT_CANON_VOCAB="$(jq -c --arg names "$AT_CANON_DEFS" "$AT_CANON_VOCAB_JQ" "$AT_CANON_SCHEMA")" || {
    at_canon_refuse "$AT_CANON_SCHEMA could not be read (jq's message above says why)" \
      "check \`jq . $AT_CANON_SCHEMA\`; the vocabulary must be read, never assumed"
    return
  }
  # One object holding every definition, or nothing is trusted: an empty
  # schema file gives jq no input, so it prints nothing and succeeds.
  jq -e --arg n "$AT_CANON_DEFS" 'type == "object" and ((($n | split(" ")) - keys) | length == 0)' \
    <<<"$AT_CANON_VOCAB" >/dev/null 2>&1 || {
    at_canon_refuse "$AT_CANON_SCHEMA gave no vocabulary for ${AT_CANON_DEFS// /, }" \
      "check \`jq . $AT_CANON_SCHEMA\`; an empty or truncated schema must refuse, never read as an empty set"
    return
  }
  local empty name
  empty="$(jq -r 'to_entries[] | select(.value | length == 0) | .key' <<<"$AT_CANON_VOCAB")" || {
    at_canon_refuse "the vocabulary read from $AT_CANON_SCHEMA could not be checked" "check \`jq . $AT_CANON_SCHEMA\`"
    return
  }
  if [ -n "$empty" ]; then
    while IFS= read -r name; do
      echo "error: no values at .[\"\$defs\"].${name} (or .definitions.${name}) in $AT_CANON_SCHEMA, read as its enum, or each oneOf or anyOf branch's enum or const" >&2
    done <<<"$empty"
    at_canon_refuse "the vocabulary in $AT_CANON_SCHEMA is incomplete, as named above" \
      "the schema moved; point lib_at_canon.sh at where the model's vocabulary now lives, never at an empty set"
    return
  fi
}

# at_canon_legal <def> <word>... -- a tool's own judgement words are the
# model's: a word the model renamed would otherwise match no row, and a
# population that silently shrinks reads as clean.
at_canon_legal() {
  local def="$1" word missing=0
  shift
  for word in "$@"; do
    jq -e --arg d "$def" --arg w "$word" '.[$d] | index([$w]) | values' <<<"$AT_CANON_VOCAB" >/dev/null || {
      echo "error: this tool judges by the word \"$word\", which is not a value of $def in $AT_CANON_SCHEMA" >&2
      missing=$((missing + 1))
    }
  done
  [ "$missing" -eq 0 ] || at_canon_refuse "${missing} of this tool's word(s) are not the model's" \
    "the model renamed a $def; align the tool with $AT_CANON_SCHEMA"
}

at_canon_ready() {
  command -v jq >/dev/null 2>&1 || {
    at_canon_refuse "jq is required to read the thread canon" "install jq"
    return
  }
  at_canon_vocab || return
  [ -d "$AT_CANON_DIR" ] || {
    at_canon_refuse "no thread canon at $AT_CANON_DIR" \
      "run this from the root of an Intent project whose canon is committed; \`intent sync\` writes it"
    return
  }
  AT_CANON_FILES=()
  local f
  for f in "$AT_CANON_DIR"/*.json; do
    [ -f "$f" ] && AT_CANON_FILES+=("$f")
  done
  # A canon with no thread is a real state and NOT a clean run: "nothing
  # violated" and "nothing was examined" must never print the same line.
  [ "${#AT_CANON_FILES[@]}" -gt 0 ] || {
    at_canon_refuse "no thread canon file under $AT_CANON_DIR" \
      "run this from the root of an Intent project that has threads; \`intent sync\` writes their canon"
    return
  }
}

# One line per JSON value read (FILE, its file name as JSON), per thread
# (THREAD), per row (ROW), per thread's row count (COUNT), and per unreadable
# thing (BAD, its label as JSON whenever it could hold a control character). A
# thread id is the file's name, so it is checked like an id. A control
# character, a NUL among them, is refused wherever a value reaches the shell:
# bash drops a NUL from a substitution with no more than a warning, so
# "tests/a.bats\u0000zz" would read as tests/a.batszz.
# shellcheck disable=SC2016 # jq's $-names, never the shell's
AT_CANON_SCAN_JQ='
  def plain: test("[\\x00-\\x1f]") | not;
  def clean: type == "string" and . != "" and plain;
  def shown: if clean then . else tojson end;
  def bad($label; $why): "BAD\t" + $label + "\t" + $why;
  def legal($def; $x): $v[$def] | any(. == $x);
  def row($tid):
    . as $r
    | if type != "object" then bad($tid + "/?"; "an AT row is " + type + ", not an object")
      else
        ($tid + "/" + ($r.id | shown)) as $at
        | (if ($r.file | type) == "string" then $r.file | sub($line_re; "") else "" end) as $cited
        | [ (if ($r.id | clean) then empty else "id " + ($r.id | tojson) + " is missing, empty, not a string or holds a control character" end),
            (if legal("AtKind"; $r.kind) then empty else "kind " + ($r.kind | tojson) + " is not an AtKind" end),
            (if legal("AtStatus"; $r.status) then empty else "status " + ($r.status | tojson) + " is not an AtStatus" end),
            (if $r.file == null or (($r.file | type) == "string" and ($r.file | plain)) then empty
             else "file " + ($r.file | tojson) + " is neither null nor a string without a control character" end),
            (if ($r.file | type) == "string" and $r.file != "" and $cited == "" then "file " + ($r.file | tojson) + " names a line and no file" else empty end) ] as $why
        | if ($why | length) > 0 then bad($at; $why | join("; "))
          else "ROW\t" + $tid + "\t" + $r.id + "\t" + $r.kind + "\t" + $r.status + "\t" + $cited
          end
      end;
  inputs
  | ("FILE\t" + (input_filename | tojson)),
    ( (input_filename | split("/") | last | sub("\\.json$"; "")) as $name
      | ($name | shown) as $tid
      | if ($name | clean | not) then bad($tid; "the thread file name holds a control character")
        elif type != "object" then bad($tid; "the thread canon is " + type + ", not an object")
        else
          (if legal("ThreadStatus"; .status) then "THREAD\t" + $tid + "\t" + .status
           else bad($tid; "thread status " + (.status | tojson) + " is not a ThreadStatus") end),
          (if has("tests") and (.tests | type) != "array" then bad($tid; "tests is " + (.tests | type) + ", not an array")
           else (.tests // []) as $rows
             | ("COUNT\t" + ($rows | length | tostring)), ($rows[] | row($tid))
           end)
        end )
'

at_canon_read() {
  local scan
  # JQ READS ITS FILES AS ONE STREAM, so a file cut off mid-value runs on into
  # the next and jq's message names that next file. On a failure each file is
  # parsed alone, and every one jq cannot parse is named with jq's own words;
  # the stream's message is printed only when no single file explains it. A
  # cut-off file the next one completes parses as a stream, and the count of
  # values per file below refuses it.
  scan="$(jq -n -r --argjson v "$AT_CANON_VOCAB" --arg line_re "$CITATION_LINE_RE" "$AT_CANON_SCAN_JQ" "$@" 2>/dev/null)" || {
    local f why named=0
    for f in "$@"; do
      why="$(jq empty "$f" 2>&1)" || {
        echo "error: ${f}: jq cannot parse it: ${why}" >&2
        named=$((named + 1))
      }
    done
    [ "$named" -gt 0 ] ||
      jq -n -r --argjson v "$AT_CANON_VOCAB" --arg line_re "$CITATION_LINE_RE" "$AT_CANON_SCAN_JQ" "$@" 2>&1 >/dev/null | sed 's/^/error: /' >&2
    at_canon_refuse "the thread canon could not be read as JSON (each file jq cannot parse is named above)" \
      "check \`jq . $AT_CANON_DIR/<ID>.json\`; an unreadable canon must refuse, never report clean"
    return
  }
  [ -n "$scan" ] || {
    at_canon_refuse "read $# thread canon file(s) and the canon pass printed nothing" \
      "an empty thread file holds no JSON value; this reader reads .status and .tests[] in $AT_CANON_DIR/<ID>.json"
    return
  }

  # EVERYTHING IS CHECKED BEFORE ANY CALLBACK RUNS, so a tool never prints a
  # finding from a canon it then refuses.
  local kind a b c d e bad=0 seen=""
  AT_CANON_ROWS=0
  while IFS="$(printf '\t')" read -r kind a b c d e; do
    case "$kind" in
      FILE) seen="${seen}${a}"$'\n' ;;
      THREAD | ROW) ;;
      COUNT) AT_CANON_ROWS=$((AT_CANON_ROWS + a)) ;;
      BAD)
        bad=$((bad + 1))
        echo "error: ${a}: ${b}" >&2
        ;;
      *)
        bad=$((bad + 1))
        echo "error: the canon pass printed a line this reader cannot read: $(printf '%q' "$kind")" >&2
        ;;
    esac
  done <<<"$scan"

  # EACH FILE HOLDS ONE JSON VALUE, counted here because jq never sees an empty
  # file: it yields nothing for it, so an empty thread file would read as no
  # thread at all, and a file holding two values would read every row twice.
  # Each name is a JSON string, one per line, matched whole: no associative
  # array, because the gate may run this under macOS's bash 3.2.
  # The list of names is read into a variable first, so a jq that cannot make
  # it refuses rather than leaving a loop that checks nothing; grep's own
  # status is read too, since 0 and 1 are answers and 2 is a failure.
  local names named n grc
  names="$(jq -n -r '$ARGS.positional[] | tojson' --args "$@")" && [ -n "$names" ] || {
    at_canon_refuse "could not list the $# thread canon file(s) to count their JSON values" \
      "this reader needs jq 1.6 or later, for --args"
    return
  }
  while IFS= read -r named; do
    grc=0
    n="$(grep -cxF -- "$named" <<<"$seen")" || grc=$?
    if [ "$grc" -gt 1 ]; then
      bad=$((bad + 1))
      echo "error: ${named}: its JSON values could not be counted (grep exited ${grc})" >&2
    elif [ "$n" != 1 ]; then
      bad=$((bad + 1))
      echo "error: ${named}: holds ${n} JSON value(s), not one" >&2
    fi
  done <<<"$names"

  if [ "$bad" -gt 0 ]; then
    at_canon_refuse "${bad} thing(s) in the thread canon could not be read, each named above" \
      "this reader takes the model's vocabulary from $AT_CANON_SCHEMA; fix the canon, or align the reader with the fields the canon carries"
    return
  fi
  if [ "$AT_CANON_ROWS" -eq 0 ]; then
    at_canon_refuse "no AT rows in $# thread canon file(s)" \
      "check the canon carries .tests[]; a read with no rows is not a clean run"
    return
  fi

  while IFS="$(printf '\t')" read -r kind a b c d e; do
    case "$kind" in
      THREAD) at_thread "$a" "$b" ;;
      ROW) at_row "$a" "$b" "$c" "$d" "$e" ;;
    esac
  done <<<"$scan"
  return 0
}
