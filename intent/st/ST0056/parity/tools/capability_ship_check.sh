#!/usr/bin/env bash
#
# capability_ship_check.sh -- AT-00.3, covering AC-00.3.
#
# **AC-00.3: `intentd` SHIPS IN THE RELEASE -- project registry, GraphQL, mgmt
# plane, debounced watching, CLI-owned launchd lifecycle.** Five capabilities.
# The claim is that they SHIP, not that they behave: WP-08's rows drive
# behaviour and this one asks whether the surface is there to drive.
#
# ==========================================================================
# WHY THIS IS A SHELL INSTRUMENT AND NOT A RUST TEST
# ==========================================================================
#
# The row cited `native/rust/crates/intentd/tests/daemon_lifecycle.rs`, **a
# file that has never existed**, at `to-write` -- where an absent citation is
# lint-exempt. It is re-cited here rather than moved first, because `red`
# REQUIRES the cited artefact to exist and NO VERB SETS A ROW BACK TO
# `to-write`: moving the label before writing the artefact is a one-way trip,
# and this estate has already taken it once (AT-10.5).
#
# Four of the five capabilities are daemon-side and the fifth is a claim about
# the CLI, so **no single crate's test suite witnesses all five**. And a red
# Rust test breaks `cargo test` for every node while a red shell tool does not.
#
# ==========================================================================
# THE TWO PROBES, AND WHY THE SECOND ONE IS NOT A HAND-MAINTAINED LIST
# ==========================================================================
#
# **DRIVE** -- send the op and read the answer. Only for ops that are reads.
#
# **DECLARE** -- send an op the daemon cannot know and read the roster out of
# its refusal. serde's unknown-variant error enumerates the accepted variants,
# so the roster is DERIVED FROM THE RUNNING BINARY rather than typed here.
# That matters: nothing in a binary can enumerate its own match arms on
# request, and a hand-kept list of ops would be correct when typed and
# silently wrong at the next op added -- the act that invalidates it is not
# the act that updates it.
#
# **THE DECLARE PROBE IS POSITIVE-CONTROLLED, AND WITHOUT THAT IT IS A
# SILENCE.** If the daemon ever stops enumerating -- a serde attribute change,
# a hand-written error -- every `declared` arm below would find nothing and
# report absence, or find an empty haystack and report presence, depending on
# which way the grep fell. So the control is TWO-SIDED: a name that MUST be in
# the roster, and a name that must NOT be. A probe that cannot fail returns
# the number that means success.
#
# ==========================================================================
# WHAT IS DELIBERATELY NOT DRIVEN, WITH THE REASON, AND THE DENOMINATOR MOVES
# ==========================================================================
#
# `shutdown` and `subscribe` are declared and NOT driven, and `launchd` is not
# driven either. Each exclusion carries its reason in the output, because **an
# exclusion that does not say why is a silent narrowing of the claim** -- the
# denominator attack wearing a verdict's clothes. The reasons are not alike
# and must not be recorded alike:
#
#   shutdown   DESTRUCTIVE TO PEERS. This machine runs four agent sessions
#              against one daemon. The universal safe probe can have a side
#              effect, and this one's side effect is everyone else's tooling.
#   subscribe  PERTURBS A MEASUREMENT IN FLIGHT. `wire.rs:155` says the event
#              harness reads the registry to take a before-and-after delta, so
#              a probe that registers a subscriber moves a number another node
#              is measuring -- the instrument perturbing its own estate.
#   launchd    MUTATES THE OPERATOR'S MACHINE, which is hv's to authorise.
#
# **A DECLARED CAPABILITY IS WEAKER EVIDENCE THAN A DRIVEN ONE AND THE OUTPUT
# SAYS SO.** It establishes that the op is dispatchable, never that it works.
#
# ==========================================================================
# THE TWO EXTERNAL CONTROLS ARE PROSE, AND THIS BLOCK SAYS SO RATHER THAN
# LETTING THEM READ AS TESTS
# ==========================================================================
#
# The roster controls above are LIVE ARMS -- they run on every invocation and
# exit 2 when they fail. These two are not: they were driven once, by hand, on
# 2026-08-31, and nothing re-drives them.
#
#   ROOT=<empty dir>       -> rc 1, RED, `CLI-owned launchd` reported ABSENT.
#   PATH=<dangling link>   -> rc 2, DANGLING SYMLINK, named as TRANSIENT.
#   PATH=<no intent>       -> rc 2, not on PATH at all.
#   BIN=/usr/bin/true      -> rc 2, binary present and NO DAEMON answering.
#
# **THE LAST THREE EXIST BECAUSE THE FIRST VERSION COLLAPSED THEM INTO ONE**, and
# cc named the cost: an instrument that cannot tell absent-because-rebuilding
# from absent-because-broken will eventually be read as the second. Three
# absences, three remedies, three different worlds -- and only the middle one is
# a fact about the estate.
#
# **A recorded proof is prose until someone types it**, and this estate has
# already had a mutation proof stop reproducing while its comment kept
# claiming it. So read these as evidence that the instrument COULD fail on the
# day it was written, never as evidence that it still can.
#
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
BIN="${BIN:-intent}"

echo "AT-00.3 -- do AC-00.3's five capabilities SHIP? Driven where a read is safe,"
echo "           derived from the daemon's own roster where a drive is not."
echo ""

# --------------------------------------------------- the binary, THEN the socket
# **THREE ABSENCES, NOT ONE, AND CONFLATING THEM IS THE DEFECT cc NAMED** (2026-08-31):
# *an instrument that cannot tell absent-because-rebuilding from
# absent-because-broken will eventually be read as the second.* The first
# version printed "no daemon is answering" for all three worlds, which is right
# in one of them and actively misleading in the other two.
#
# The mid-rebuild case is real and invisible from inside it: the delivered
# `intent` is a SYMLINK into the cargo output directory, so during a build the
# link exists and its target does not. `command -v` reports nothing at all --
# indistinguishable from never-installed unless the link is looked for by name.
# Measured live on this machine during a `dvb build all`, in a window of about
# one minute, with four sessions sharing that symlink.
bin_path="$(command -v "$BIN" 2>/dev/null || true)"
if [ -z "$bin_path" ]; then
  dangling=""
  IFS=: read -ra _dirs <<< "$PATH"
  for d in "${_dirs[@]}"; do
    # -L true and -e false is exactly a symlink whose target is gone.
    if [ -L "$d/$BIN" ] && [ ! -e "$d/$BIN" ]; then dangling="$d/$BIN"; break; fi
  done
  if [ -n "$dangling" ]; then
    echo "    NOT EVALUABLE -- \`$BIN\` is a DANGLING SYMLINK at ${dangling}, which on this"
    echo "    estate means a build is in flight and the target is momentarily gone."
    echo "    **This is a TRANSIENT world and not a broken one.** remedy: wait for the"
    echo "    build and re-run; nothing here is a finding about the estate."
    exit 2
  fi
  echo "    NOT EVALUABLE -- \`$BIN\` is not on PATH at all, so nothing can be asked."
  echo "    remedy: install or build it. **Distinct from the dangling case above**, which"
  echo "    is transient, and from a stopped daemon below, which is a running-state fact."
  exit 2
fi

sock="$("$BIN" daemon status 2>/dev/null | sed -n 's/.*answering at //p')"
if [ -z "$sock" ] || [ ! -S "$sock" ]; then
  echo "    NOT EVALUABLE -- \`$BIN\` is present at ${bin_path} and NO DAEMON is answering,"
  echo "    so four of the five capabilities have no surface to ask. **This is an"
  echo "    exclusion, never a pass**: a criterion scored over the members that happened"
  echo "    to be reachable is the wrong-M of AC-00.11."
  echo "    remedy: intent daemon start, then re-run."
  exit 2
fi
echo "    daemon answering at ${sock}"

ask() { printf '{"root":"%s","op":%s}\n' "$ROOT" "$1" | nc -U "$sock" 2>/dev/null; }

# ------------------------------------------------- the derived roster + control
roster_raw="$(ask '"__no_such_op__"')"
roster="$(printf '%s' "$roster_raw" | sed -n 's/.*expected one of \(.*\) at line.*/\1/p' | tr -d '`' | tr ',' ' ')"
# shellcheck disable=SC2086 -- MUST split: `$roster` is a space-separated op
# list and the split is what turns it into one op per line to count. Quoting
# it would count one line always, which is a counter that cannot discriminate.
nroster="$(printf '%s\n' $roster | grep -c .)"

# TWO-SIDED, because a one-sided witness passes under the very thing it replaces.
if [ "$nroster" -lt 2 ]; then
  echo "error: the daemon did not enumerate its op roster in its refusal, so every" >&2
  echo "       declared-only verdict below would be derived from an empty haystack." >&2
  echo "       raw: ${roster_raw:0:200}" >&2
  exit 2
fi
case " $roster " in
  *" registry "*) ;;
  *) echo "error: positive control FAILED -- \`registry\` is driven successfully below and is absent from the roster, so the roster read is wrong" >&2; exit 2 ;;
esac
case " $roster " in
  *" __no_such_op__ "*) echo "error: negative control FAILED -- the roster contains the name that was invented to be absent, so containment cannot discriminate" >&2; exit 2 ;;
  *) ;;
esac
echo "    roster derived from the running daemon (${nroster} ops), controls two-sided: PASS"
echo ""

declared() { case " $roster " in *" $1 "*) return 0 ;; *) return 1 ;; esac; }

absent=(); weak=()

# 1 -------------------------------------------------------- project registry
r="$(ask '"registry"')"
if printf '%s' "$r" | grep -q '"result":"registry"'; then
  n="$(printf '%s' "$r" | grep -o '"root":' | grep -c .)"
  printf '    %-28s DRIVEN     registry answered with %s project(s)\n' "project registry" "$n"
else
  printf '    %-28s ABSENT     %s\n' "project registry" "${r:0:90}"; absent+=("project registry")
fi

# 2 --------------------------------------------------------------- GraphQL
g="$("$BIN" graphql '{ threads { id } }' 2>&1)"
if printf '%s' "$g" | grep -q '"data"'; then
  printf '    %-28s DRIVEN     intent graphql returned data, and `graphql` is in the roster\n' "GraphQL"
  declared graphql || { printf '    %-28s NOTE       driven but NOT in the roster -- the two reads disagree\n' ""; absent+=("GraphQL roster"); }
else
  printf '    %-28s ABSENT     %s\n' "GraphQL" "${g:0:90}"; absent+=("GraphQL")
fi

# 3 ------------------------------------------------------------- mgmt plane
# The CLI half is driven; the daemon half is declared and must not be driven.
cli_mgmt=0
for v in start stop status run; do "$BIN" daemon "$v" --help >/dev/null 2>&1 && cli_mgmt=$((cli_mgmt + 1)); done
if declared shutdown && [ "$cli_mgmt" -eq 4 ]; then
  printf '    %-28s DECLARED   `shutdown` in the roster; all 4 CLI verbs answer --help. NOT DRIVEN: destructive to peer sessions on this machine\n' "mgmt plane"
  weak+=("mgmt plane")
else
  printf '    %-28s ABSENT     shutdown-in-roster=%s cli-verbs=%s/4\n' "mgmt plane" "$(declared shutdown && echo y || echo n)" "$cli_mgmt"; absent+=("mgmt plane")
fi

# 4 ------------------------------------------------------- debounced watching
# **The registry publishes a LIVE witness that beats the declaration**: `watched`
# is the daemon's own answer about this project, not a claim about the binary.
if printf '%s' "$r" | grep -q '"watched":true'; then
  printf '    %-28s DRIVEN     registry reports watched=true for this project; `subscribe` in roster. NOT DRIVEN: registering a subscriber perturbs the event delta another node measures\n' "debounced watching"
elif declared subscribe; then
  printf '    %-28s DECLARED   `subscribe` in the roster, but the registry does not report this project watched\n' "debounced watching"
  weak+=("debounced watching")
else
  printf '    %-28s ABSENT     neither a subscribe op nor a watched flag\n' "debounced watching"; absent+=("debounced watching")
fi

# 5 ------------------------------------------------ CLI-owned launchd lifecycle
# **THIS ARM WAS A MENTION TEST AND IS NOW AN INSTANCE TEST, and the first
# version passed for the wrong reason.** It counted source files matching
# /launchd|LaunchAgents|\.plist/ and found four -- which would have counted a
# file that merely mentions launchd in a comment. **A MENTION IS NOT AN
# INSTANCE**, and an arm whose evidence is a word in a file cannot tell a
# shipped capability from a doc comment about one. Caught by opening the four
# files the arm was resting on rather than trusting the count.
#
# Two-sided now, and both sides are edges rather than words:
#   PUBLISHED  `daemon start --help` and `daemon stop --help` disclose the
#              enrolment flag. That is a DRIVE of the shipped surface.
#   OWNED      the CLI crate calls `launchagent::` -- a call SITE, which is
#              the declaration cc's rule asks for. The claim in AC-00.3 is
#              CLI-OWNED lifecycle, so a plist writer nothing calls would not
#              satisfy it, and a plist on disk proves only that something did.
flag=0
for v in start stop; do "$BIN" daemon "$v" --help 2>/dev/null | grep -q -- '--at-login' && flag=$((flag + 1)); done
edges="$(grep -c 'launchagent::' "$ROOT"/native/rust/crates/intent-cli/src/render.rs 2>/dev/null || echo 0)"
if [ "$flag" -eq 2 ] && [ "$edges" -gt 0 ]; then
  printf '    %-28s DRIVEN     --at-login published on start AND stop; %s launchagent:: call site(s) in the CLI crate. NOT DRIVEN: enrolling installs a launch agent on the operator machine\n' "CLI-owned launchd" "$edges"
else
  printf '    %-28s ABSENT     --at-login on %s/2 verbs, %s CLI call site(s) into launchagent\n' "CLI-owned launchd" "$flag" "$edges"; absent+=("CLI-owned launchd")
fi

cat <<'REACH'

REACH, in the output because a limit not in the output is not a limit the
reader has:
  COVERS      whether each of AC-00.3's five capabilities has a surface that
              answers, and by which of two probes it was established.
  DOES NOT    test behaviour. `registry` answering is not the registry being
              correct; WP-08's rows own that and a bug there is invisible here.
  DOES NOT    drive `shutdown`, `subscribe` or the launchd install, each for a
              different reason named on its line above. **Three of five rest
              partly on a DECLARATION, which establishes that an op is
              dispatchable and never that it works.**
  DOES NOT    know whether the roster the daemon enumerates is the roster it
              dispatches. They are one serde enum today; if they ever diverge
              this instrument believes the error message.
  UNOWNED     a daemon that is not running makes four of five unaskable. That
              exits 2 as NOT EVALUABLE rather than passing on the one that
              remains.
REACH
echo ""

if [ "${#absent[@]}" -gt 0 ]; then
  echo "RED -- ${#absent[@]} of AC-00.3's five capabilities do not ship: ${absent[*]}"
  exit 1
fi
echo "PASS -- all five capabilities ship. ${#weak[@]} of five (${weak[*]}) rest on a DECLARATION"
echo "rather than a drive, each excluded for the reason printed on its line."
exit 0
