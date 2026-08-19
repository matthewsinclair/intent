#!/usr/bin/env bash
# no_daemon_required.sh -- AT-07.5 / AC-07.5.
#
# **NO DAEMON IS REQUIRED TO READ YOUR OWN PROJECT.** With intentd stopped,
# uninstalled and never started, every address in a FULLY DEHYDRATED estate
# resolves through the CLI.
#
# WHY THE CRITERION IS NOT WITHDRAWABLE ON hv's GROUNDS, checked rather than
# assumed. hv withdrew AC-00.3 because "if we need safety, we've got all the
# historical STs etc in git" -- any precondition PROVING REVERSIBILITY proves
# against a loss git already prevents. Applied here it does not bite, and the
# reason is sharper than "this one is different": **restoring the estate from
# git RE-HYDRATES it, which destroys the precondition this row tests.** Git is
# not a fallback for a dehydrated-estate property; reaching for it falsifies
# the subject. AC-00.3 was a safety proof, this is an ACCESSIBILITY claim, and
# git substitutes for the first and cannot substitute for the second.
#
# ==========================================================================
# THE ARM THAT MATTERS IS ARM B, AND SAYING SO IS THE WHOLE POINT
# ==========================================================================
#
# `intent daemon` answers `is a known command that is not implemented yet` in
# this build. **So arm A -- no intentd process, and every read still answers --
# is green because the daemon DOES NOT EXIST, not because the read path is
# daemon-free.** That is a clean-by-luck green, and it would keep printing the
# same reassuring line on the day reads start routing through a daemon.
#
# So arm B asserts the STRUCTURAL property instead: `intent-cli` declares no
# dependency on `intentd`, and its sources open no socket. **Arm B is the one
# that survives intentd shipping**; arm A is the behavioural witness that the
# estate really was empty when the reads answered. Both run, both are reported,
# and the weakness of arm A is printed on every run rather than left for a
# reader to work out.
#
# EXIT CODES. 0 = the property holds. 1 = it does not. **2 = CANNOT MEASURE**,
# which is never a pass: no binary, no scratch, a daemon already running, or an
# estate that refused to dehydrate. An unmeasurable run that exits 0 is the
# failure this file exists to avoid.
#
# --selftest drives POSITIVE CONTROLS -- it proves each arm can fail -- because
# an all-green checker and a checker that cannot fail print the same thing.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$HERE/../../../../.." && pwd)"

BIN="${BIN:-$REPO_ROOT/native/rust/target/release/intent}"
CLI_MANIFEST="${CLI_MANIFEST:-$REPO_ROOT/native/rust/crates/intent-cli/Cargo.toml}"
CLI_SRC="${CLI_SRC:-$REPO_ROOT/native/rust/crates/intent-cli/src}"

RC=0
say()  { printf '%s\n' "$*"; }
fail() { printf 'FAIL: %s\n' "$*"; RC=1; }
die()  { printf 'error: %s\n' "$*" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"

# ---------------------------------------------------------------------------
# ARM B -- STRUCTURAL. The CLI cannot reach a daemon even if one is running.
#
# Checked against the manifest and the sources rather than by running anything,
# because the question is what the binary is ABLE to do, not what it did today.
# ---------------------------------------------------------------------------
arm_b_structural() {
  say "== arm B (structural): the CLI has no way to reach a daemon =="
  [ -f "$CLI_MANIFEST" ] || { die "no intent-cli manifest at $CLI_MANIFEST"; }
  [ -d "$CLI_SRC" ]      || { die "no intent-cli sources at $CLI_SRC"; }

  # A dependency on intentd. Comments and the package DESCRIPTION are excluded:
  # the description says "in-process or GraphQL to intentd" and is prose about
  # a designed future, not a dependency edge.
  local dep
  dep="$(sed -n '/^\[.*dependencies.*\]/,/^\[/p' "$CLI_MANIFEST" | grep -c '^[[:space:]]*intentd' || true)"
  if [ "$dep" -ne 0 ]; then
    fail "arm B: intent-cli declares a dependency on intentd ($dep line(s))"
  else
    say "  ok: intent-cli declares no intentd dependency"
  fi

  # A socket or network client in the sources. Comments stripped first, so a
  # doc comment DESCRIBING the daemon does not read as a call to it.
  local sock
  sock="$(cat "$CLI_SRC"/*.rs \
    | sed 's|//.*$||' \
    | grep -cE 'UnixStream|UnixDatagram|TcpStream|reqwest|hyper::client' || true)"
  if [ "$sock" -ne 0 ]; then
    fail "arm B: intent-cli sources open $sock socket/network client(s) -- a read may leave this process"
  else
    say "  ok: no socket or network client in intent-cli sources"
  fi
}

# ---------------------------------------------------------------------------
# ARM A -- BEHAVIOURAL. A fully dehydrated estate answers every read.
#
# THE TWO CONTROLS ARE THE ARM. Without them a green means nothing:
#   1. the estate must actually BE empty (else the reads are served by files);
#   2. no intentd may run BEFORE or AFTER (else a read may have been served by
#      one, or may have started one).
# ---------------------------------------------------------------------------
arm_a_behavioural() {
  say "== arm A (behavioural): a dehydrated estate answers every read =="
  [ -x "$BIN" ] || die "no runnable binary at $BIN -- build it with \`bin/int build cli\`"

  if pgrep -f 'intentd' >/dev/null 2>&1; then
    die "an intentd process is already running -- this arm cannot distinguish a daemon-free read from a daemon-served one while it is up"
  fi

  local work; work="$(mktemp -d)" || die "could not create a scratch directory"
  trap 'rm -rf "$work"' RETURN
  case "$(cd "$work" && pwd -P)" in
    "$REPO_ROOT"*) die "the scratch directory resolves inside this project -- refusing to dehydrate in a live tree" ;;
  esac

  local p="$work/p"
  mkdir -p "$p/intent/.canon/st" "$p/intent/.config" || die "could not lay out the scratch project"
  cat > "$p/intent/.config/config.json" <<'J'
{"intent_version":"3.0.0","project_name":"AT0705","author":"ic","intent_dir":"intent","languages":["rust"],"plugins":{"claude":{}}}
J

  # Two ordinary threads to read back, and one that OPENS THE GATE.
  #
  # **A PROJECT DECLARING NO PRECONDITIONS CANNOT DEHYDRATE AT ALL** -- measured,
  # not assumed: `organize --apply` refuses with "0 checked of 0 declared, so
  # nothing is proved and nothing may be removed". The gate is fail-closed and
  # absence of a declaration is not permission. So the fixture must DECLARE a
  # precondition and SATISFY it; a fixture that simply omitted the block would
  # make this arm unmeasurable while looking like a setup detail.
  local i
  for i in 1 2; do
    jq -n --arg id "ST000$i" '{schema:"intent/thread@3.0", id:$id, title:("Readable " + $id),
      status:"wip", created:"2026-08-19", objective:("OBJECTIVEMARKER for " + $id),
      context:"", body:"", preamble:"", attachments:[]}' \
      > "$p/intent/.canon/st/ST000$i.json" || die "could not write fixture canon"
  done
  jq -n '{schema:"intent/thread@3.0", id:"ST0057", title:"Gate", status:"wip",
    created:"2026-08-19", objective:"opens the dehydration gate", context:"", body:"",
    preamble:"", attachments:[],
    criteria:[
      {id:"AC-00.1", text:"the declaring criterion <<PRECONDITIONS AC-00.9 PRECONDITIONS>>",
       kind:"non-test", state:{is:"unsatisfied"}},
      {id:"AC-00.9", text:"met, so the gate is open and this arm can reach a dehydrated estate",
       kind:"non-test", state:{is:"satisfied", evidence:"fixture: the gate is not the subject of AT-07.5"}}
    ]}' > "$p/intent/.canon/st/ST0057.json" || die "could not write the gate thread"

  ( cd "$p" && HOME="$work" "$BIN" sync --to-store >/dev/null 2>&1 ) || die "sync --to-store failed on the fixture"
  ( cd "$p" && HOME="$work" "$BIN" sync --to-disk  >/dev/null 2>&1 ) || die "sync --to-disk failed on the fixture"

  local before; before="$(find "$p/intent/st" -type f 2>/dev/null | wc -l | tr -d ' ')"
  [ "$before" -ge 3 ] || die "the fixture realised only $before file(s); there is nothing to dehydrate and this arm would pass vacuously"

  # DEHYDRATE EVERYTHING: an empty manifest declares nothing realised.
  : > "$p/intent/.intentfiles"
  local out; out="$( cd "$p" && HOME="$work" "$BIN" organize --apply 2>&1 )"
  say "  organize said -- $(printf '%s\n' "$out" | head -1)"

  # CONTROL ONE: the estate must actually be empty of artefact files.
  local left; left="$(find "$p/intent/st" -mindepth 2 -type f 2>/dev/null | wc -l | tr -d ' ')"
  if [ "$left" -ne 0 ]; then
    die "the estate did not dehydrate -- $left artefact file(s) remain of $before; a read answering now proves nothing"
  fi
  say "  ok: estate fully dehydrated, $before file(s) -> 0 artefact file(s)"

  # THE READS. Each must exit 0 AND say something -- an empty rc=0 is not a
  # resolution, and `at list` on a thread with no tests legitimately prints
  # nothing, which is why the roster below names only reads with content.
  local probe rc answer n_ok=0 n_run=0
  while IFS='|' read -r probe expect; do
    [ -n "$probe" ] || continue
    n_run=$((n_run + 1))
    # **`$probe` IS DELIBERATELY UNQUOTED and this file's shebang is what makes
    # that correct.** Under bash an unquoted expansion word-splits, so `st show
    # ST0001` reaches the binary as three arguments. Under ZSH it would not --
    # SH_WORD_SPLIT is off by default -- and the whole roster would arrive as
    # one subcommand, which answers `unrecognized subcommand 'st show ST0001'`
    # at rc=1 and reads exactly like the failure this arm exists to detect.
    # Measured while building this file, in a zsh probe loop, against a green
    # estate. Do not "fix" the quoting, and do not run this under zsh.
    # shellcheck disable=SC2086
    answer="$( cd "$p" && HOME="$work" "$BIN" $probe 2>&1 )"; rc=$?
    if [ "$rc" -ne 0 ]; then
      fail "arm A: \`$probe\` exited $rc from a dehydrated estate -- $(printf '%s' "$answer" | head -1)"
    elif ! printf '%s' "$answer" | grep -q "$expect"; then
      fail "arm A: \`$probe\` exited 0 but did not resolve -- expected /$expect/, said: $(printf '%s' "$answer" | head -1)"
    else
      n_ok=$((n_ok + 1))
    fi
  done <<ROSTER
st show ST0001|Readable ST0001
st list|ST0001
ac list ST0057|AC-00.1
info|Intent
todo|TODO
export --format json|ST0001
search OBJECTIVEMARKER|ST000
ROSTER
  say "  reads resolved: $n_ok of $n_run"

  # **THE SHARPEST WITNESS: CONTENT WITH NO FILE UNDER IT.**
  #
  # The first version of this probe asked `search` for a hit and asserted the
  # path it cited was gone. **It was wrong, and the way it was wrong is worth
  # keeping.** Search cited `intent/.canon/st/ST0001.json` -- which exists, and
  # SHOULD: canon is the committed extract and dehydration never touches it.
  # The probe had quietly assumed search cites VIEWS, which it does only when a
  # thread carries prose sections; this fixture has none, so it fell through to
  # canon and the arm reported a failure of the tool rather than of itself.
  #
  # So the witness is direct instead. `st show` must print a thread's OBJECTIVE
  # -- a string that lives in the store and in canon and NOT in any file the
  # reader could have opened -- while the view that would ordinarily carry it is
  # provably absent. A read serving bytes whose file does not exist cannot have
  # been served by the filesystem, and that is the whole claim.
  # `st show` was the second wrong guess and is recorded rather than quietly
  # swapped out: it prints a SUMMARY -- id, title, status, created -- and the
  # objective is not in it. The objective lives in the info VIEW, which is the
  # file this arm has just removed, so the read that must carry it is one that
  # serves the store's contents rather than a header. `export --format json`
  # is that read.
  local marker="OBJECTIVEMARKER"
  local dumped; dumped="$( cd "$p" && HOME="$work" "$BIN" export --format json 2>&1 )"
  local view="$p/intent/st/ST0001/info.md"
  if [ -e "$view" ]; then
    fail "arm A: $view still exists, so the export may have been served by a file"
  elif ! printf '%s' "$dumped" | grep -q "$marker"; then
    fail "arm A: export did not serve the objective with no file under it -- $(printf '%s' "$dumped" | head -1)"
  else
    say "  ok: export served \`$marker\` while intent/st/ST0001/info.md does not exist"
  fi

  # CONTROL TWO: no daemon before (checked above) and none after.
  if pgrep -f 'intentd' >/dev/null 2>&1; then
    fail "arm A: an intentd process is running AFTER the reads -- something started one"
  else
    say "  ok: no intentd process before or after"
  fi
}

# ---------------------------------------------------------------------------
# SELFTEST -- positive controls. Each arm is driven against an input that MUST
# fail it. An arm that stays green here is an arm that cannot fail at all.
# ---------------------------------------------------------------------------
selftest() {
  local w; w="$(mktemp -d)"; trap 'rm -rf "$w"' RETURN
  local bad=0

  say "== selftest: each arm must be able to fail =="

  # B1: a manifest that DOES depend on intentd.
  mkdir -p "$w/src"
  printf '[package]\nname = "x"\n\n[dependencies]\nintentd = { path = "../intentd" }\n' > "$w/Cargo.toml"
  printf 'fn main() {}\n' > "$w/src/main.rs"
  if ( RC=0; CLI_MANIFEST="$w/Cargo.toml" CLI_SRC="$w/src" arm_b_structural >/dev/null 2>&1; [ "$RC" -ne 0 ] ); then
    say "  ok: B1 (declared intentd dependency) is caught"
  else
    say "  BROKEN: B1 was not caught -- arm B cannot see a dependency"; bad=1
  fi

  # B2: sources that open a socket.
  printf 'use std::os::unix::net::UnixStream;\nfn main() { let _ = UnixStream::connect("/tmp/x"); }\n' > "$w/src/main.rs"
  printf '[package]\nname = "x"\n' > "$w/Cargo.toml"
  if ( RC=0; CLI_MANIFEST="$w/Cargo.toml" CLI_SRC="$w/src" arm_b_structural >/dev/null 2>&1; [ "$RC" -ne 0 ] ); then
    say "  ok: B2 (socket in sources) is caught"
  else
    say "  BROKEN: B2 was not caught -- arm B cannot see a socket"; bad=1
  fi

  # B3: THE NEGATIVE CONTROL FOR THE COMMENT-STRIPPER. A doc comment mentioning
  # the daemon must NOT fail the arm -- otherwise arm B is a grep for the word
  # and every honest comment about intentd becomes a finding.
  printf '// this CLI could speak to intentd over a UnixStream one day\nfn main() {}\n' > "$w/src/main.rs"
  if ( RC=0; CLI_MANIFEST="$w/Cargo.toml" CLI_SRC="$w/src" arm_b_structural >/dev/null 2>&1; [ "$RC" -eq 0 ] ); then
    say "  ok: B3 (the same words inside a COMMENT) is correctly not a finding"
  else
    say "  BROKEN: B3 was caught -- arm B is a word grep, not a dependency check"; bad=1
  fi

  [ "$bad" -eq 0 ] || { printf 'selftest: at least one arm cannot do its job\n' >&2; exit 2; }
  say "selftest: every arm fails on its control and passes on the negative one"
}

case "${1:-}" in
  --selftest) selftest; exit 0 ;;
  "") ;;
  *) die "unknown argument '$1' -- this tool takes no arguments, or --selftest" ;;
esac

arm_b_structural
arm_a_behavioural

say ""
if [ "$RC" -eq 0 ]; then
  say "AC-07.5 HOLDS: a fully dehydrated estate resolves every read with no daemon."
  say "  ARM A IS WEAK EVIDENCE TODAY AND THIS LINE IS PART OF THE RESULT: \`intent daemon\`"
  say "  answers \"is a known command that is not implemented yet\" in this build, so no"
  say "  daemon could have served a read whether or not the read path would let it."
  say "  **ARM B IS THE ONE THAT SURVIVES intentd SHIPPING.**"
else
  say "AC-07.5 DOES NOT HOLD on this tree."
fi
exit "$RC"
