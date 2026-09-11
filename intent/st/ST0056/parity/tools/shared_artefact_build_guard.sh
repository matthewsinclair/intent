#!/bin/bash
# shared_artefact_build_guard.sh -- AC-11.6 / AT-11.6.
#
# A REBUILD INTO THE SHARED ARTEFACT PATH IS REFUSED WHEN THE TREE IS DIRTY, and
# the refusal happens BEFORE the artefact exists. `native/rust/target/release/`
# is shared by every node in this checkout with no per-node isolation underneath
# it; the founding episode is a build at 19:33Z on 2026-08-18 from a live tree
# carrying uncommitted WP-01 source, which produced a working-looking binary that
# emptied the store for every node until rebuilt.
#
# THE RED ARM IS THE REAL EPISODE AND IT IS FIRST. That ordering is not a style
# choice: this row spent three revisions cited at mechanisms that could not have
# fired on the episode that produced it -- `prepush` clones HEAD to a temp dir and
# never touches the shared path at all -- and every one of those citations was
# offered with a true, driven measurement of a DIFFERENT property attached. A
# green arm alone would have passed under all three. So the first thing this file
# proves is that the guard REFUSES, on a tree shaped like the one that caused it.
#
# WHY IT DRIVES A SCRATCH REPO AND NEVER THIS ONE. The subject is a build guard,
# and the only honest way to test a build guard against the live tree would be to
# run a build into the shared slot -- which is the act the guard exists to
# prevent. The predicate is therefore separable by design (`sharedtarget.lib`),
# and every arm below drives it in a disposable repo with planted state. Nothing
# here builds anything, and nothing here reads or writes
# `native/rust/target/release/`.
#
# REACH, AND IT IS THE HONEST LIMIT OF THIS FILE: THE REDIRECT HAS NEVER
# EXECUTED. Arms 1 to 6 DRIVE the predicate in disposable repos and are real
# behaviour. Arms 7 and 8 READ THE SOURCE of `cmd/local` -- they establish that
# the ordering and the redirect are PRESENT, not that running a dirty build
# actually writes the private target dir and actually leaves the shared one
# alone. Confirming that needs a real cargo build, which this file deliberately
# does not do. One claim here is taken from source rather than from behaviour and
# it is named so that nobody reads a green as covering it.
#
# ARM 4 IS THE ONE THAT WOULD HAVE CAUGHT THE ORIGINAL DEFECT, AND IT IS ABOUT
# ORDER RATHER THAN OUTCOME. `cmd/local`'s `verify_pair` ALREADY refused a
# `dirty-` marker before any of this existed -- it simply ran after `cargo build`
# had overwritten the shared binaries, so it reported the damage rather than
# preventing it. A guard whose verdict is correct and late is indistinguishable
# from no guard, to everyone downstream of the artefact. Outcome arms cannot see
# that; only an order arm can.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

LIB="$ROOT/bin/.devbin/cmd/shared/sharedtarget.lib"
LOCAL_CMD="$ROOT/bin/.devbin/cmd/local"
# THE GUARDED BUILD MOVED OUT OF `cmd/local` ON 2026-08-26 AND THIS SUBJECT
# MOVED WITH IT. Arms 7 and 8 read the body of the ONE guarded release build,
# and that body now lives in a lib because FOUR other entrances -- `build all`,
# `build cli`, `build daemon` and `cmd/cli`'s fallback -- wrote the same shared
# artefact with a bare `cargo build` while this arm truthfully reported that
# `cmd_build` consulted the verdict. **The arm was right and its subject was one
# door of five.**
RELEASEBUILD_LIB="$ROOT/bin/.devbin/cmd/shared/releasebuild.lib"
MARKER_SRC="$ROOT/native/rust/build-support/source_commit.rs"

rc=0
pass=0
fail() { printf 'shared-artefact-guard: %s\n' "$1" >&2; rc=1; }
ok()   { pass=$((pass + 1)); printf 'shared-artefact-guard: ok -- %s\n' "$1"; }

if [ ! -r "$LIB" ]; then
  fail "no predicate library at $LIB -- the guard cannot be checked because it does not exist"
  exit 1
fi
# shellcheck source=/dev/null
. "$LIB"

# scratch_repo <dir> -- a minimal repo with a committed native/rust tree.
#
# `-c` rather than `git config`: a scratch repo inherits nothing, and a machine
# without a global identity would otherwise fail the COMMIT rather than the arm,
# which reads as a guard failure and is not one.
scratch_repo() {
  local d="$1"
  # SEE `sharedtarget.lib`s isolation note. `git -C` does NOT override GIT_DIR or
  # GIT_INDEX_FILE, and git exports both into hook environments -- so under the
  # pre-commit gate these three calls ran against THIS repository`s index and
  # staged the fixture into it, naming an object the outer repo cannot resolve
  # (`error: Error building trees`). It blocked every commit in the estate.
  local G="env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE -u GIT_OBJECT_DIRECTORY -u GIT_COMMON_DIR git"
  $G -C "$d" init -q 2>/dev/null || return 1
  mkdir -p "$d/native/rust/crates/thing/src"
  printf 'fn main() {}\n' > "$d/native/rust/crates/thing/src/main.rs"
  printf 'placeholder\n' > "$d/README.md"
  $G -C "$d" add -A >/dev/null 2>&1 || return 1
  $G -C "$d" -c user.email=guard@test -c user.name=guard commit -qm base >/dev/null 2>&1 || return 1
  return 0
}

TMP="$(mktemp -d)" || { fail "cannot make a scratch directory"; exit 1; }
trap 'rm -rf "$TMP"' EXIT

# ---------------------------------------------------------------- ARM 1: RED
# A tree dirty under native/rust, building into the shared release path, must be
# REFUSED with the offending paths named. This is the 19:33Z shape.
red="$TMP/red"
mkdir -p "$red"
if ! scratch_repo "$red"; then
  fail "arm 1 could not build its fixture -- an arm that cannot run is not an arm that passed"
else
  printf 'fn main() { /* uncommitted */ }\n' > "$red/native/rust/crates/thing/src/main.rs"
  verdict="$(shared_target_verdict "$red")"
  case "$verdict" in
    refuse:*) ok "arm 1 RED -- a dirty native/rust tree is refused: ${verdict#refuse:}" ;;
    *) fail "arm 1 RED -- a dirty native/rust tree returned '$verdict'; this is the episode the criterion was minted for and the guard let it through" ;;
  esac
  # AC-11.6 requires the refusal to NAME the paths. A refusal that does not say
  # what is dirty sends the operator to `git status` to re-derive what the guard
  # already knew, and the first thing anybody does with a guard they cannot act
  # on is bypass it.
  named="$(shared_target_dirt_report "$red")"
  case "$named" in
    *"crates/thing/src/main.rs"*) ok "arm 1 RED -- the refusal names the offending path" ;;
    *) fail "arm 1 RED -- the refusal does not name the offending path; it reported: ${named:-<nothing>}" ;;
  esac
fi

# ------------------------------------------------------------- ARM 2: UNTRACKED
# An UNTRACKED file under native/rust must refuse too. It is not a nicety: the
# live tree on the day this was written carried exactly one (an untracked test
# beside a modified source file), and a guard reading only modifications would
# have called that tree clean.
unt="$TMP/untracked"
mkdir -p "$unt"
if ! scratch_repo "$unt"; then
  fail "arm 2 could not build its fixture"
else
  printf 'fn helper() {}\n' > "$unt/native/rust/crates/thing/src/extra.rs"
  verdict="$(shared_target_verdict "$unt")"
  case "$verdict" in
    refuse:*) ok "arm 2 -- an untracked native/rust file is refused" ;;
    *) fail "arm 2 -- an untracked native/rust file returned '$verdict'; modifications are not the only way bytes enter a build" ;;
  esac
fi

# --------------------------------------------------------------- ARM 3: GREEN
# A clean tree takes the shared path. Present as a CONTROL rather than as the
# evidence: an instrument that only ever refuses is a freeze, and a freeze passes
# arm 1 for the wrong reason.
green="$TMP/green"
mkdir -p "$green"
if ! scratch_repo "$green"; then
  fail "arm 3 could not build its fixture"
else
  verdict="$(shared_target_verdict "$green")"
  case "$verdict" in
    ok) ok "arm 3 GREEN -- a clean tree is allowed into the shared path" ;;
    *) fail "arm 3 GREEN -- a clean tree returned '$verdict'; a guard that refuses everything is a freeze, and a freeze gets bypassed" ;;
  esac
fi

# ------------------------------------------------------- ARM 4: DIRT IS SCOPED
# Dirt OUTSIDE native/rust must NOT refuse. This is the arm that keeps the guard
# usable: five nodes share this clone and somebody always holds an uncommitted
# whiteboard file. Measured 2026-08-19 -- with a worktree-wide scope EVERY binary
# in this estate carried `dirty-`, including a release build of exactly HEAD. A
# flag that is always set carries no information, and it is worse than absent
# because it occupies the slot where a real signal would go.
outside="$TMP/outside"
mkdir -p "$outside"
if ! scratch_repo "$outside"; then
  fail "arm 4 could not build its fixture"
else
  printf 'a dirty board, which enters no binary\n' > "$outside/README.md"
  verdict="$(shared_target_verdict "$outside")"
  case "$verdict" in
    ok) ok "arm 4 -- dirt outside native/rust does not refuse the shared path" ;;
    *) fail "arm 4 -- dirt outside native/rust returned '$verdict'; this scope makes the guard a freeze that gets bypassed" ;;
  esac
fi

# ---------------------------------------------------- ARM 5: FAILS CLOSED
# git unreachable must NOT read as clean. Empty output means clean to any reader
# that does not distinguish them, and that is the one direction a guard on the
# build path must never fail.
nogit="$TMP/nogit"
mkdir -p "$nogit/native/rust"
verdict="$(shared_target_verdict "$nogit")"
case "$verdict" in
  refuse:*) ok "arm 5 -- an unreachable git refuses rather than reading as clean" ;;
  *) fail "arm 5 -- an unreachable git returned '$verdict'; undecidable is not a pass on the path that overwrites the shared artefact" ;;
esac

# ------------------------------------------------- ARM 6: SCOPE AGREES WITH THE MARKER
# The guard's scope and the embedded provenance marker's `DIRT_SCOPE` are the
# same question asked by two parties. If they diverge, the shared slot can hold
# an artefact this guard approved and the artefact itself disowns -- approved
# clean, stamped `dirty-`. Compared here rather than coupled at runtime: a shell
# library parsing a Rust constant on every invocation fails in the dark, and a
# test that compares them fails in the light.
# THE ASSERTION IS CONTAINMENT, NOT EQUALITY, AND THE EARLIER EQUALITY WAS WRONG
# IN A WAY THAT WOULD HAVE FORBIDDEN THE FIX IT NEEDED. Being STRICTER than the
# marker is safe: this guard then refuses builds the marker would have called
# clean. Being LOOSER is the unsafe direction, because the shared slot can hold
# an artefact this guard approved and the artefact itself disowns. An equality
# arm cannot tell those apart -- it reds on both -- and it red on the widening
# that closed a live incident.
if [ ! -r "$MARKER_SRC" ]; then
  fail "arm 6 -- no marker source at $MARKER_SRC, so scope containment cannot be checked"
else
  # DIRT_SCOPE BECAME A LIST ON 2026-08-26 and this parser moved with it, in the
  # same commit, because a reader left behind reports "the constant moved or was
  # renamed" -- a true sentence about the wrong thing.
  # Read through sharedtarget.lib's parser, the one the build and the currency
  # reader use, so this arm checks the list they actually compare over.
  marker_scopes="$(shared_target_marker_scopes "$MARKER_SRC" || true)"
  if [ -z "$marker_scopes" ]; then
    fail "arm 6 -- could not read DIRT_SCOPE from $MARKER_SRC; the constant moved, was renamed or changed shape, and an unread scope is not a contained one"
  else
    uncontained=""
    while IFS= read -r ms; do
      covered=0
      for sc in "${SHARED_TARGET_DIRT_SCOPES[@]}"; do
        [ "$sc" = "$ms" ] && covered=1
      done
      [ "$covered" -eq 1 ] || uncontained="$uncontained $ms"
    done <<< "$marker_scopes"
    if [ -z "$uncontained" ]; then
      ok "arm 6 -- the guard's scope CONTAINS every marker DIRT_SCOPE entry ($(printf '%s' "$marker_scopes" | tr '\n' ' '))"
    else
      fail "arm 6 -- the guard's scope does not contain the marker's$uncontained. Looser than the marker means the shared slot can hold an artefact this guard approved and the artefact disowns."
    fi
  fi

  # ARM 6c -- THE MARKER ASKS **IDENTITY** OVER THE SCOPE TOO, NOT JUST DIRT.
  #
  # Arm 6 checks that the two scopes agree. It is structurally blind to the
  # defect that made this fix necessary: until 2026-08-26 the marker asked DIRT
  # over DIRT_SCOPE and IDENTITY over an unscoped `rev-parse HEAD`, so one
  # string answered two questions about two different subjects -- and arm 6 was
  # green throughout, because the scope it compared was the one that was already
  # right.
  #
  # THE BODY IS READ WITH COMMENTS STRIPPED, following arm 7, which failed its
  # own first draft by matching a header sentence ABOUT an invocation. That is
  # not hypothetical here: the fix's own doc comment names `rev-parse HEAD`
  # twice, explaining what it replaced. **A guard that reads prose would refuse
  # the very change it exists to enforce.**
  emit_body="$(awk '/^fn emit_source_commit\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' "$MARKER_SRC" |
    sed 's://.*::')"
  if [ -z "$emit_body" ]; then
    fail "arm 6c -- could not extract emit_source_commit from $MARKER_SRC; an unread body is not a scoped one"
  elif grep -q 'rev-parse' <<<"$emit_body"; then
    fail "arm 6c -- emit_source_commit still asks identity with rev-parse, which is UNSCOPED. The stamp would mean 'the repo's HEAD, annotated with whether the artefact was dirty' -- two subjects in one string."
  elif ! grep -q 'rev-list' <<<"$emit_body"; then
    fail "arm 6c -- emit_source_commit asks identity with neither rev-parse nor rev-list; the call changed shape and this arm cannot say what subject it names"
  elif ! grep -q 'DIRT_SCOPE' <<<"$emit_body"; then
    fail "arm 6c -- emit_source_commit's identity call does not reach DIRT_SCOPE, so identity and dirt describe different subjects again"
  else
    ok "arm 6c -- the marker asks identity AND dirt over DIRT_SCOPE"
  fi
fi

# ------------------------------ ARM 6b: THE SCOPE COVERS WHAT THE BUILD COMPILES
# **THE DECLARED SCOPE IS CHECKED RATHER THAN DERIVED, AND THIS ARM IS THE
# PROMPT THAT MAKES DECLARING SAFE.** `intent-cli` embeds
# `surface/dispatch-table.json` with `include_str!` at COMPILE time, from OUTSIDE
# the crate tree -- so it is a build input the original `native/rust` scope could
# not see. **Measured live on 2026-08-25 (vc, on themselves): an announced,
# peer-cleared, guard-approved build on a clean `native/rust` baked a peer's
# half-written dispatch row into the shared binary, and every `intent3 sync`
# then panicked on it.** The peer reverted the file within minutes, **so the
# source that explained the binary no longer existed anywhere** -- a dirty
# `native/rust` build at least leaves its cause behind to be found.
#
# **A DECLARED SCOPE THAT MUST BE HAND-UPDATED WHEN AN UNRELATED FILE GAINS AN
# `include_str!` IS A GUARD WHOSE CORRECTNESS DEPENDS ON A STEP NOBODY IS
# PROMPTED TO TAKE** (vc). Deriving the scope at runtime would mean parsing Rust
# from shell on every invocation, which is a coupling that fails in the dark.
# This arm is the third option: declare it, and red in the light the day a second
# outside-the-tree embed appears.
#
# **THE COVERAGE TEST READS THE DECLARED SCOPES; IT USED TO READ A `case` THAT
# ONLY KNEW `surface`** (cc, 2026-08-30, on the first embed that was not one).
# The arm's own failure message says *the declared scope does NOT cover them*,
# and the arm could not answer that question: a hardcoded
# `*/surface/*) covered_by=":(top)surface"` mapped exactly one directory, so
# every other outside-the-tree embed was uncovered BY CONSTRUCTION and adding
# its directory to `SHARED_TARGET_DIRT_SCOPES` changed nothing. **That is a
# third home for a rule the comment above says is declared in one place.**
#
# **THE `../`-STRIPPING RESIDUAL IS DISCHARGED HERE (dc, 2026-09-04) AND THE
# REASONING THAT WOULD HAVE PREVENTED IT REPLACES IT RATHER THAN THE NOTE BEING
# DELETED.** The previous form stripped every `../` prefix and treated the
# remainder as repo-relative, which assumed *the climb reaches the root and not
# some directory between*. It said so, called UNCOVERED the safe direction, and
# was correct about correctness. **IT CAME TRUE:** an `include_str!(
# "../../intentd/src/shell.html")` in `crates/intentsvcs/tests/` climbs to
# `crates/`, stripped to `intentd/src/shell.html`, matched no scope, and the arm
# refused. **THE FINDING WAS FALSE AND THE REFUSAL BLOCKED EVERY COMMIT IN THE
# REPOSITORY ON EVERY PATH** -- confirmed on three nodes, on markdown and canon
# paths with no relation to Rust.
#
# **SO: ANY "SAFE DIRECTION" ARGUMENT IN A GUARD MUST NAME ITS BLAST RADIUS.**
# Not *this errs toward refusing* -- **this errs toward refusing, and a refusal
# here stops every commit in the repository on every path.** Written that way,
# `realpath` gets reached for in the same sitting. Safe-for-the-claim and
# safe-for-the-people-working are different axes and only the first was on the
# page (vc's ruling, 2026-09-04).
#
# **RESOLUTION IS NOW AGAINST THE EMBEDDING FILE'S OWN DIRECTORY.** `include_str!`
# resolves relative to the file the macro appears in, so that is the only base
# that answers the question; a prefix count answers a different one. The
# resolution is LEXICAL rather than `realpath`, deliberately: a planted control
# pair must resolve without the file existing, and a target that climbs above the
# repository root is reported rather than silently clamped.
#
# **DECLARED LIMIT, AND THE COMPLEMENT IS PRINTED RATHER THAN LEFT TO A COMMENT.**
# This arm EXAMINES `native/rust/crates` only. Embeds elsewhere under
# `native/rust` are swept for, named, and NOT examined -- so the arm cannot
# overclaim, and whoever wants them examined makes that choice in daylight.
# **DECLARING IS NOT WIDENING, and they are different acts** (vc's ruling,
# 2026-09-04): an undeclared reach is the defect this arm was just repaired for,
# and leaving one inside the repair is the narrow-selector class (`AC-00.16`)
# committed by the fix for it. Moving the glob quietly is the move that was
# ruled against; printing what the glob does not cover is the alternative.
#
# **THE COMPLEMENT HAS ITS OWN BOUNDARY AND IT IS STATED TOO**, because a
# complement computed over an undeclared sweep is the same defect one level up:
# the sweep is `native/rust`. An embed outside `native/rust` entirely is outside
# both the population and the complement, and nothing here reports it.
embed_resolve() {                       # $1 repo-relative embedding file, $2 the embed path
  # **THE JOIN GOES INTO A VARIABLE FIRST AND THAT IS LOAD-BEARING.** Word
  # splitting applies only to characters that CAME FROM an expansion, so in
  # `for seg in $dir/$2` the joining `/` is a literal and is NOT a split point:
  # `tests` + `/` + `..` arrives as the single field `tests/..`, no `..` case
  # ever matches, and the function silently returns an unresolved path. The
  # controls below caught exactly that before this landed.
  local dir="${1%/*}" out="" seg joined
  joined="$dir/$2"
  local IFS=/
  for seg in $joined; do
    case "$seg" in
      ''|.) ;;
      ..)   if [ -z "$out" ]; then printf '%s' '!ABOVE-ROOT'; return 0; fi; out="${out%/*}" ;;
      *)    out="$out/$seg" ;;
    esac
  done
  printf '%s' "${out#/}"
}
embed_covered() {                       # $1 repo-relative target; rc 0 covered, 1 not
  local sc scope_path
  for sc in "${SHARED_TARGET_DIRT_SCOPES[@]}"; do
    scope_path="${sc#:(top)}"
    case "$1" in "$scope_path"/*) return 0 ;; esac
  done
  return 1
}
embed_covered_OLD() {                   # the stripping form, kept ONLY to prove the control discriminates
  local climbed="$1"
  while case "$climbed" in ../*) true ;; *) false ;; esac; do climbed="${climbed#../}"; done
  embed_covered "$climbed"
}

# CONTROLS FIRST, AND THE POSITIVE ONE IS PLANTED RATHER THAN POINTED AT.
# **ITS SUBJECT LEFT THE TREE WHILE THIS FIX WAS BEING WRITTEN** -- ic removed the
# cross-crate embed at 18:37Z. A control aimed at an absent condition passes
# because the condition is gone, not because the logic works, and would pass
# under the broken guard too (cc caught this before it was written).
ctl_between="$(embed_resolve 'native/rust/crates/intentsvcs/tests/x.rs' '../../intentd/src/shell.html')"
ctl_outside="$(embed_resolve 'native/rust/crates/x/src/y.rs' '../../../../../CHANGELOG.md')"
ctl_bad=""
[ "$ctl_between" = "native/rust/crates/intentd/src/shell.html" ] \
  || ctl_bad="$ctl_bad between-landing-resolves-to($ctl_between)"
embed_covered "$ctl_between" \
  || ctl_bad="$ctl_bad between-landing-not-covered"
embed_covered_OLD '../../intentd/src/shell.html' \
  && ctl_bad="$ctl_bad positive-control-has-no-subject(old-logic-also-passes)"
[ "$ctl_outside" = "CHANGELOG.md" ] \
  || ctl_bad="$ctl_bad outside-resolves-to($ctl_outside)"
embed_covered "$ctl_outside" \
  && ctl_bad="$ctl_bad genuinely-outside-embed-NOT-refused"

embeds="$(grep -rHo 'include_str!("[^"]*")\|include_bytes!("[^"]*")' "$ROOT/native/rust/crates" 2>/dev/null \
          | sed -E "s#^${ROOT}/##" \
          | grep '"\.\./' \
          | sed -E 's#^([^:]+):(include_str|include_bytes)!\("([^"]+)"\)$#\1|\3#' \
          | sort -u)"
n_embeds=0
uncovered=""
while IFS= read -r pair; do
  [ -n "$pair" ] || continue
  n_embeds=$((n_embeds + 1))
  ef="${pair%%|*}"; rel="${pair#*|}"
  tgt="$(embed_resolve "$ef" "$rel")"
  embed_covered "$tgt" || uncovered="$uncovered $ef:$rel(resolves to $tgt)"
done <<EOF
$embeds
EOF
# THE COMPLEMENT, EMITTED UNCONDITIONALLY -- green or red, empty or not. A
# figure that appears only on one path cannot be read as a scope on the other.
#
# **THE DENOMINATORS COUNT THE WALK, NOT THE HITS, AND THAT IS THE COST SIGNAL**
# (vc's ruling, 2026-09-04, on the near-miss below). `3 examined` says nothing
# about what was traversed to find 3. The walk figures do: this arm sweeps 370
# files, and the first draft of the complement -- which read all of
# `native/rust` including `target/` -- would have walked **74,239**, on the
# first run, in the author's own terminal, before anything landed. No new
# watcher was needed; the line was already being printed and had the wrong
# subject.
#
# **THE HONEST LIMIT, because a proxy stated as a measure is this estate's
# recurring defect: FILES-WALKED IS A SMOKE DETECTOR, NOT A BUDGET.** A cheap
# test over a million paths can be fine and an expensive one over forty can be
# ruinous. A real budget needs a latency measurement nobody has; this is the
# cheap signal that would have caught the case that actually happened.
#
# **`target/` IS EXCLUDED AND THE EXCLUSION IS DECLARED RATHER THAN ASSUMED.**
# It is 9.4G of generated output on this machine against 7.8M of `crates`, and
# this arm runs on every commit -- the first draft of this sweep read the whole
# of `native/rust` and turned the gate into a multi-minute grep over a build
# directory. **Nothing in the estate would have reported that**; it surfaced
# because the guard got slow enough to notice. An embed found under `target/`
# is a COPY of one in source, so excluding it loses no subject, which is why
# this exclusion is sound and not merely convenient.
outside_pop="$(grep -rHo --exclude-dir=target 'include_str!("[^"]*")\|include_bytes!("[^"]*")' "$ROOT/native/rust" 2>/dev/null \
               | sed -E "s#^${ROOT}/##" \
               | grep '"\.\./' \
               | grep -v '^native/rust/crates/' \
               | sed -E 's#^([^:]+):(include_str|include_bytes)!\("([^"]+)"\)$#\1 -> \3#' \
               | sort -u)"
n_outside=0
[ -n "$outside_pop" ] && n_outside=$(printf '%s\n' "$outside_pop" | grep -c .)
n_walk_pop=$(find "$ROOT/native/rust/crates" -type f 2>/dev/null | grep -c .)
n_walk_sweep=$(find "$ROOT/native/rust" -name target -prune -o -type f -print 2>/dev/null | grep -c .)
printf 'shared-artefact-guard: arm 6b NOT EXAMINED -- population is native/rust/crates (%s file(s) walked); the complement sweep reads native/rust excluding target/ (%s file(s) walked) and found %s embed(s) with a `../` path outside the population, swept for and not examined.\n' "$n_walk_pop" "$n_walk_sweep" "$n_outside"
[ "$n_outside" -gt 0 ] && printf '    %s\n' "$outside_pop"

if [ -n "$ctl_bad" ]; then
  fail "arm 6b -- CONTROLS FAILED ($ctl_bad); no verdict is offered on the $n_embeds embed(s) this arm examined"
elif [ -z "$embeds" ]; then
  fail "arm 6b -- found NO outside-the-tree embeds at all across native/rust/crates; dispatch.rs is known to carry one, so the probe is broken rather than the tree being clean"
elif [ -z "$uncovered" ]; then
  ok "arm 6b -- every embed reaching outside its crate is covered by the declared scope ($n_embeds embed(s) examined over $n_walk_pop file(s) walked; both controls fired)"
else
  fail "arm 6b -- of $n_embeds embed(s) examined, these resolve outside every declared scope:$uncovered. A build mid-edit in one of them is approved by this guard and baked into the shared binary."
fi


# --------------------------------- ARM 6d: A GENERATED EMBED IS STILL AN EMBED
# **ARM 6b's CENSUS IS A GREP FOR A LITERAL `include_str!`, AND A GENERATED
# EMBED HAS NO LITERAL TO FIND** (`0287`). `build-support/embed_templates.rs`
# WALKS a directory and WRITES the macro lines into `$OUT_DIR`, so the embed
# exists only after a build and only under `target/` -- which arm 6b's
# population (`native/rust/crates`) does not reach, and which its complement
# sweep excludes for a correct and declared reason. **Two arms, one class, two
# individually defensible blindnesses.** Measured 2026-09-08: arm 6b printed
# `ok` on a commit while `lib/templates/{llm,prj}` reached into the shipped
# binary from outside every declared scope. A control whose population cannot
# contain the subject is not a control that failed; it is one that could never
# have fired.
#
# **THIS ARM READS THE BUILD'S OWN OUTPUT RATHER THAN PARSING THE GENERATOR,
# AND THE CHOICE IS THE DESIGN.** The generator is always present and is
# INFERENCE: answering "what does this Rust build?" by reading Rust from shell
# is the coupling that fails in the dark, and the obvious parse -- take every
# `.join("literal")` -- cannot tell an INPUT it walks from the OUTPUT file it
# writes, so it would red on `embedded_templates.rs` and refuse every commit in
# the repository for a path that is not an input at all. The build output is
# MEASUREMENT: it holds the embeds the compiler was actually handed.
#
# **ITS ABSENCE IS REPORTED, NEVER SCORED AS ZERO, AND THAT IS THE HALF THE
# REPAIRED ARM WOULD OTHERWISE REPEAT.** On a fresh checkout there is no build
# output, so an arm that simply examined it would print a confident `ok` over a
# population of nothing -- the same shape as the defect this arm exists for. It
# says NOT EXAMINED instead, and quantifies what it could not read.
#
# **BLAST RADIUS, NAMED BECAUSE THE RULING REQUIRES IT AND BECAUSE THIS ARM CAN
# REFUSE:** a failure here blocks the commit on EVERY path in the repository,
# not merely a build -- the same radius that made the `../`-stripping residual
# an outage rather than conservatism. It is written to red only when a
# generated embed resolves outside every declared scope, which is a condition an
# author creates by adding a generator and never one that arrives on its own.
# ARRAYS AND NUL-DELIMITED READS THROUGHOUT, NOT STYLE: a build directory is
# machine-generated and an absolute path under it can contain anything a
# checkout path can, spaces included. The first draft joined these into a
# space-separated string and split it again, which the shell critic refused as
# `IN-SH-CODE-001` -- correctly, because the failure it names is silent: a path
# with a space becomes two files that do not exist, `grep` reads neither, and
# the arm reports a confident zero over a population it never opened. That is
# the same shape as the blindness this whole arm was written to repair.
gen_files=()
while IFS= read -r d; do
  [ -n "$d" ] || continue
  for g in "$d"/*.rs; do [ -f "$g" ] && gen_files+=("$g"); done
done < <(find "$ROOT/native/rust/target" -maxdepth 4 -type d -name out 2>/dev/null | head -40 | sort -u)
n_gen_files=${#gen_files[@]}

# The generators themselves, counted so that "no build output" is a quantity
# rather than a blank -- a generator in source with nothing built from it is
# exactly the state this arm must not read as clean.
n_generators=$(grep -rl 'include_str!\|include_bytes!' "$ROOT/native/rust/build-support" 2>/dev/null | grep -c . || true)

# **THE COVERAGE TEST TAKES THE FULL PATH AND THE MESSAGE TAKES THE ROOT, AND
# CONFLATING THE TWO IS A BUG THIS ARM SHIPPED FOR ABOUT A MINUTE.**
# `embed_covered` asks whether a path is strictly INSIDE a scope (`scope/*`), so
# handing it a path already reduced to `lib/templates` compares the scope to
# itself and can never match -- the arm reds on a root the scopes DO cover, and
# the author reads it as the gap still being open. Full paths are tested; the
# root is derived afterwards, for the human reading the failure.
gen_paths=()
if [ "$n_gen_files" -gt 0 ]; then
  while IFS= read -r pth; do
    [ -n "$pth" ] && gen_paths+=("$pth")
  done < <(grep -ho 'include_str!("[^"]*")\|include_bytes!("[^"]*")' "${gen_files[@]}" 2>/dev/null \
           | sed -E 's#^(include_str|include_bytes)!\("([^"]+)"\)$#\2#' \
           | grep "^${ROOT}/" \
           | sed -E "s#^${ROOT}/##" \
           | sort -u)
fi
n_gen_roots=0
if [ "${#gen_paths[@]}" -gt 0 ]; then
  n_gen_roots=$(printf '%s\n' "${gen_paths[@]}" | sed -E 's#^([^/]+/[^/]+)/.*#\1#' | sort -u | grep -c .)
fi

# CONTROLS, BOTH SIDES, PLANTED RATHER THAN POINTED AT -- and they exercise the
# COVERAGE predicate, which is the only judgement this arm makes.
ctl6d=""
embed_covered "native/rust/crates/whatever" || ctl6d="$ctl6d covered-root-NOT-recognised"
embed_covered "definitely-not-a-scope/x"    && ctl6d="$ctl6d uncovered-root-NOT-refused"

gen_uncovered=""
n_gen_paths=${#gen_paths[@]}
for pth in ${gen_paths[@]+"${gen_paths[@]}"}; do
  if ! embed_covered "$pth"; then
    root="$(printf '%s' "$pth" | sed -E 's#^([^/]+/[^/]+)/.*#\1#')"
    case " $gen_uncovered " in *" $root "*) ;; *) gen_uncovered="$gen_uncovered $root" ;; esac
  fi
done

if [ -n "$ctl6d" ]; then
  fail "arm 6d -- CONTROLS FAILED ($ctl6d); no verdict is offered on the $n_gen_roots generated embed root(s) this arm examined"
elif [ "$n_gen_files" -eq 0 ]; then
  printf 'shared-artefact-guard: arm 6d NOT EXAMINED -- %s generator(s) under build-support mention an embed macro and NO build output exists to read (native/rust/target holds no out/*.rs). This is not a clean result; it is an unread one, and it is the state of a fresh checkout.\n' "$n_generators"
elif [ "${#gen_paths[@]}" -eq 0 ]; then
  fail "arm 6d -- read $n_gen_files generated file(s) and found NO absolute embed at all; embed_templates.rs is known to write them, so the probe is broken rather than the tree being clean"
elif [ -z "$gen_uncovered" ]; then
  ok "arm 6d -- every generated embed resolves inside the declared scope ($n_gen_paths embed(s) under $n_gen_roots root(s), read from $n_gen_files generated file(s) written by $n_generators generator(s); both controls fired)"
else
  fail "arm 6d -- these generated embed root(s) are outside every declared scope:$gen_uncovered. They are compiled into the shared binary, so a build taken mid-edit in one of them is approved by this guard and baked in, and no arm above can see them."
fi

# ------------------------------------------- ARM 7: THE VERDICT PRECEDES THE BUILD
# THE ORDER IS THE CRITERION. `verify_pair` already refused a `dirty-` marker
# before this guard existed; it ran after `cargo build` had replaced the shared
# binaries, so it named the damage instead of preventing it. AC-11.6 weighed that
# exact ordering and ruled for the earlier refusal in its own words -- before the
# artefact exists, rather than producing a working-looking binary four nodes then
# invoke. An outcome arm cannot see this; a correct verdict arriving late passes
# every one of arms 1 to 6.
if [ ! -r "$RELEASEBUILD_LIB" ]; then
  fail "arm 7 -- no guarded build at $RELEASEBUILD_LIB"
else
  # THE BODY OF `cmd_build`, NOT THE WHOLE FILE, AND COMMENTS STRIPPED. Both
  # narrowings are corrections to this arm's own first draft, which reported a
  # FAILURE against correct code: it grepped the whole file and matched the
  # header sentence "WHY `build` IS NOT `cargo build --release`" at line 31 --
  # prose ABOUT a cargo invocation, read as one. That is this estate's oldest
  # instrument defect (a grep cannot tell a statement from a sentence about a
  # statement) arriving inside the tool written to catch a different one.
  #
  # It could equally have failed the other way and that is the worse direction:
  # a comment MENTIONING the verdict, sitting above a real cargo line, would have
  # made the arm PASS while the call site was absent or late. Scoping to the
  # function body also removes the file-order-versus-call-order gap, since both
  # statements live in the one function and lexical order is execution order
  # inside it.
  body="$(awk '/^guarded_release_build\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' "$RELEASEBUILD_LIB" \
          | grep -vE '^[[:space:]]*#')"
  v_line="$(printf '%s\n' "$body" | grep -n 'shared_target_verdict' | head -1 | cut -d: -f1)"
  c_line="$(printf '%s\n' "$body" | grep -n 'cargo \(clean\|build\)' | head -1 | cut -d: -f1)"
  if [ -z "$body" ]; then
    fail "arm 7 -- could not extract guarded_release_build from $RELEASEBUILD_LIB; the function moved or was renamed, and an unread body is not an ordered one"
  elif [ -z "$v_line" ]; then
    fail "arm 7 -- guarded_release_build never consults the guard; the predicate exists and nothing calls it, which is a guard nothing dispatches"
  elif [ -z "$c_line" ]; then
    fail "arm 7 -- no cargo invocation inside guarded_release_build, so the order cannot be established"
  elif [ "$v_line" -lt "$c_line" ]; then
    ok "arm 7 -- the verdict is taken before the first cargo invocation inside guarded_release_build"
  else
    fail "arm 7 -- the verdict is taken AFTER cargo inside guarded_release_build. A correct verdict arriving after the shared artefact is overwritten reports the damage instead of preventing it, which is the defect this row was minted against."
  fi
fi

# --------------------------------------- ARM 8: A DIRTY BUILD IS REDIRECTED, NOT BLOCKED
# The guard must not be a freeze. AC-11.6 as amended sends a dirty build to a
# private CARGO_TARGET_DIR marked `dirty-<sha>`; without that redirect this guard
# would REFUSE the remedy another rostered guard prints -- `bin/intent3`'s
# currency check hands every node `int local build` when the shared pair is
# stale, and a stale pair on a dirty tree would then have no way forward at all.
# Two guards, one instructing a node to do what the other exists to prevent.
if [ ! -r "$LOCAL_CMD" ]; then
  : # already reported by arm 7
elif grep -q 'PRIVATE_RELEASE_DIR' "$RELEASEBUILD_LIB" && grep -q 'CARGO_TARGET_DIR' "$RELEASEBUILD_LIB"; then
  ok "arm 8 -- a refused build is redirected to a private CARGO_TARGET_DIR rather than blocked"
else
  fail "arm 8 -- no private redirect in $RELEASEBUILD_LIB; a guard that stops the build outright is a freeze, and a freeze gets bypassed"
fi


# ------------------------------- ARM 9: THE HOOK ENVIRONMENT DOES NOT LEAK IN
# **THE ARM THAT DID NOT EXIST WHEN THIS FILE WENT GREEN, AND ITS ABSENCE
# BLOCKED EVERY COMMIT IN THE ESTATE.** `git -C <dir>` changes the WORKING
# DIRECTORY and does NOT override `GIT_DIR`, `GIT_INDEX_FILE` or `GIT_WORK_TREE`
# -- and git EXPORTS ALL THREE INTO HOOK ENVIRONMENTS. Rostered `gated`, this
# file first ran from inside the pre-commit hook, where its fixtures' `git add`
# staged into THE OUTER REPOSITORY'S index, naming an object that repository
# cannot resolve. `error: Error building trees`, on every commit, for every node.
#
# **THE HARNESS WAS THE ONE ENVIRONMENT WHERE THE DEFECT COULD NOT APPEAR.** Nine
# arms, mutation-proven four ways, all run standalone -- where `GIT_DIR` is unset
# and `git -C` is sufficient. **Mutation testing varies the SUBJECT and holds the
# ENVIRONMENT fixed, so no number of mutations could have found this.** The
# estate's own class, arriving one axis over: not an instrument scoped narrower
# than its criterion, but an instrument whose HARNESS excluded the failure.
#
# **AND IT WAS NOT ONLY THE FIXTURES.** The same inheritance made
# `shared_target_verdict` answer about the HOOK'S repository rather than the one
# it was handed -- measured, with `GIT_DIR` set and cwd an empty fixture, `status`
# printing the outer repository's tracked paths as deleted. **A predicate whose
# whole contract is _about the tree you named_ was silently about a different
# one.** Arms 1-6 would have passed anyway, reading the outer tree and getting
# plausible answers from it, which is why this arm asserts the SUBJECT and not
# just the absence of damage.
outer="$TMP/outer"; inner="$TMP/inner"
mkdir -p "$outer" "$inner"
if ! scratch_repo "$outer" || ! scratch_repo "$inner"; then
  fail "arm 9 could not build its two fixtures"
else
  # `outer` is DIRTY, `inner` is CLEAN. Under a leaking environment the verdict
  # about `inner` comes back as the dirty answer, which is the whole tell.
  printf 'fn main() { /* the outer tree is dirty */ }\n' > "$outer/native/rust/crates/thing/src/main.rs"
  outer_index_before="$(shasum -a 256 "$outer/.git/index" 2>/dev/null | awk '{print $1}')"
  verdict="$(GIT_DIR="$outer/.git" GIT_INDEX_FILE="$outer/.git/index" shared_target_verdict "$inner")"
  outer_index_after="$(shasum -a 256 "$outer/.git/index" 2>/dev/null | awk '{print $1}')"
  case "$verdict" in
    ok) ok "arm 9 -- with GIT_DIR set, the verdict is still about the tree it was HANDED" ;;
    *)  fail "arm 9 -- with GIT_DIR set, a CLEAN subject returned '$verdict'; the predicate answered about the hook's repository rather than the one it was given" ;;
  esac
  # The second half: a fixture must never write to the ambient index. This is the
  # arm that would have caught the estate-wide block directly.
  scratch_repo "$TMP/leak" >/dev/null 2>&1 || true
  GIT_DIR="$outer/.git" GIT_INDEX_FILE="$outer/.git/index" scratch_repo "$TMP/leak2" >/dev/null 2>&1 || true
  outer_index_final="$(shasum -a 256 "$outer/.git/index" 2>/dev/null | awk '{print $1}')"
  if [ "$outer_index_after" = "$outer_index_final" ] && [ -n "$outer_index_after" ]; then
    ok "arm 9 -- building a fixture under a set GIT_INDEX_FILE does not touch the ambient index"
  else
    fail "arm 9 -- building a fixture MUTATED the ambient index ($outer_index_after -> $outer_index_final). This is the estate-wide commit block: the fixture stages into the outer repository, naming an object it cannot resolve."
  fi
fi

# ARM 10 -- THE ENTRANCE CENSUS, AND IT IS THE ARM THAT CLOSES THE CLASS RATHER
# THAN THE INSTANCE.
#
# Arms 7 and 8 read the ONE guarded build and say nothing about how many other
# doors exist. That is exactly how this defect lived: `int build all`,
# `int build cli`, `int build daemon` and `cmd/cli`'s fallback each wrote the
# shared artefact with a bare `cargo build` while arm 7 truthfully reported that
# the guarded path consulted the verdict. **The arm was right; its subject was
# one door of five.** Nothing here would stop a sixth being added tomorrow, so
# this arm asks the question those arms structurally cannot.
#
# THE PREDICATE: a cargo invocation that CAN produce a RELEASE build must carry,
# in its own file, evidence that it cannot reach the shared tree --
# `clone_workspace` (it builds inside a clone), `CARGO_TARGET_DIR` (redirected),
# `refuse_single_package_release` (it refuses first), or `guarded_release_build`
# (it IS the guarded path). A debug-only invocation needs none of these: that is
# the stated exception, and it is stated because a debug build exists to run
# uncommitted code.
#
# THE COUNT IS THIS ARM'S ONLY EVIDENCE THAT IT LOOKED AT ANYTHING. Its first
# draft reported `(1 examined)` against a tree carrying four release-capable
# invocations, because `grep -n` on a SINGLE file emits `<line>:<text>` with no
# filename, so `cut -d: -f3-` discarded the statement and kept nothing. It passed.
# **A census that examines one of four and reports green is the false-green this
# whole file exists to refuse**, so the number is printed and is the thing to
# read first.
#
# FILE-SCOPED EVIDENCE, DELIBERATELY, AND ITS WEAKNESS IS NAMED RATHER THAN
# HIDDEN: a file could carry `clone_workspace` for one invocation and build the
# shared tree in another. That is a weaker check than per-invocation dataflow and
# a far stronger one than none, which is what exists today. It cannot be fooled
# by accident -- only by writing the token deliberately -- and this file's own
# history says the failure mode to design against is a door nobody noticed, not
# a door someone disguised.
# prose_stripped <line> -- <line> with the spans that CANNOT EXECUTE removed.
#
# THE AXIS IS "CAN THIS SPAN RUN", NOT "WHICH QUOTE WRAPS IT". That correction is
# cc's, driven on 2026-08-27, and it caught a fix of mine that would have made
# this arm WORSE than the defect it was fixing. In shell, `"` and `` ` ``
# interpolate, so two forms that read as prose actually execute:
#
#   die "run `cargo build --release`"        <- UNESCAPED backtick: RUNS
#   printf '%s' "$(cargo build --release)"   <- $( ) inside "": RUNS
#
# Both were driven, not reasoned. The queued fix stripped both as prose, which
# would have HIDDEN a live unguarded release build behind a green arm -- the
# exact inverse of the false positive it was written to close.
#
# SO ONLY THE SINGLE-QUOTE STRIP IS ADDED HERE, because single quotes are the one
# genuinely inert form in shell: nothing interpolates inside them. That alone
# closes the 2026-08-27 block, where `cmd/cli`'s `die` single-quoted the safe
# advice and stopped four nodes on a sentence recommending the guarded path.
# The `eval` bail-out stays, since `eval '...'` re-parses and would run.
#
# CLOSED 2026-08-27, AND THE FIX IS HEREDOC STATE RATHER THAN A BETTER REGEX.
# The backtick strip used to erase an UNESCAPED span, which hid the first form
# above behind a green arm. It was not fixable per line: whether a bare backtick
# executes depends on the enclosing heredoc, whose delimiter sits on a DIFFERENT
# line:
#
#   cat <<'USAGE'   `echo X`   -> printed literally   INERT
#   cat <<USAGE     `echo X`   -> X                   EXECUTES
#
# and `cmd/macos`'s help text lives in the inert case, which is where this whole
# class started, so "never strip a bare backtick" would have closed the third
# instance by reopening the first. `inert_heredoc_lines` tracks the state across
# the file, so the two are now TOLD APART rather than traded off.
#
# THE PREDICATE IS THREE-WAY, AND THE COST OF GETTING IT WRONG IS ASYMMETRIC:
#   inert heredoc body   -> the WHOLE line is inert. Nothing inside a quoted
#                           heredoc can run, so there is no span to reason about.
#   \`...\` (escaped)    -> literal inside double quotes. INERT. This is the form
#                           `cmd/cli:63` uses to recommend the guarded path, and
#                           it is why the strip is keyed on the BACKSLASH.
#   `...` (unescaped)    -> COMMAND SUBSTITUTION. It RUNS. Kept, and the census
#                           sees it.
# A wrong INERT hides a live unguarded build behind a green arm. A wrong LIVE
# fails a correct file loudly. The second is recoverable in a minute and the
# first is the defect this whole file exists to refuse, so where the three forms
# are ambiguous this errs towards LIVE.
#
# THE HOLE THAT REMAINS, NAMED RATHER THAN INHERITED IN SILENCE: the single-quote
# strip pairs quotes left to right, so an apostrophe inside ordinary double-quoted
# prose ("it's") can pair with a later real single-quoted span and delete
# everything between them, live backticks included. It is not introduced here and
# it is not closed here. Closing it needs the same lexer this arm keeps declining
# to hand-roll into a gate four nodes commit through.

# inert_heredoc_lines <file> -- the line numbers sitting inside a QUOTED heredoc
# body, one per line. A quoted delimiter (<<'X', <<"X", <<\X) makes the body
# literal text; an UNQUOTED one interpolates, so its body is live code and is
# deliberately absent from this list. Openers in comments are skipped, since a
# commented-out heredoc opens nothing.
inert_heredoc_lines() {
  awk '
    {
      if (inhd) {
        if ($0 ~ ("^[[:space:]]*" delim "[[:space:]]*$")) { inhd = 0 }
        else if (inert) { print NR }
        next
      }
      if ($0 ~ /^[[:space:]]*#/) { next }
      s = $0
      gsub(/<<</, "@@@", s)
      if (match(s, /<<-?[[:space:]]*(\\?[A-Za-z_][A-Za-z0-9_]*|\047[^\047]+\047|"[^"]+")/)) {
        tok = substr(s, RSTART, RLENGTH)
        sub(/^<<-?[[:space:]]*/, "", tok)
        inert = (tok ~ /^[\047"\\]/) ? 1 : 0
        gsub(/[\047"\\]/, "", tok)
        delim = tok
        inhd = 1
      }
    }
  ' "$1"
}

# prose_stripped <line> [inert] -- <line> with the spans that CANNOT EXECUTE
# removed. Pass `inert` when the line sits in a quoted heredoc body.
prose_stripped() {
  local s
  if [ "${2:-}" = "inert" ]; then
    printf '%s' ""
    return
  fi
  s="$(printf '%s' "$1" | sed 's/\\`[^`]*\\`//g')"
  case "$1" in
    *eval*) printf '%s' "$s"; return ;;
  esac
  # **A STRIP THAT CANNOT DECIDE MUST NOT STRIP** (vc's ruling, 2026-08-27, on a
  # measured population of ONE). An ODD apostrophe count means at least one of
  # them is not a delimiter, so the left-to-right pairing below can run from a
  # stray apostrophe to the OPENING of a real quoted span and delete live code
  # between them. Undecidable therefore reads as LIVE: it fails a correct file
  # loudly and diagnosably instead of hiding a build behind a green arm, which is
  # the same asymmetry the heredoc case above is decided on.
  #
  # MEASURED BEFORE IT WAS APPLIED, AND THE COST IS ZERO ON THIS TREE: of the 14
  # lines the census examines under `bin/.devbin`, exactly one carries the shape
  # (`cmd/macos:278`, three apostrophes) and NO verdict changes. That line is safe
  # today only by luck of ORDER -- the real pair comes first and strips correctly,
  # leaving the stray `peer's` with nothing to pair against. Reverse the order and
  # it deletes everything between. Nothing would have reported that.
  if [ $(( $(printf '%s' "$s" | tr -cd "'" | wc -c) % 2 )) -eq 1 ]; then
    printf '%s' "$s"
    return
  fi
  printf '%s' "$s" | sed "s/'[^']*'//g"
}

# guard_evidence <file> <lineno> -- the text whose guard tokens may excuse the
# cargo invocation on <lineno>: the ENCLOSING FUNCTION, plus file scope.
#
# THE ESCAPE USED TO BE FILE-SCOPED AND THIS FILE'S OWN HEADER SAID SO --
# "a file could carry `clone_workspace` for one invocation and build the shared
# tree in another" -- written down as an accepted weakness. cc pushed on it on
# 2026-08-27 and was right to. `cmd/local` carries those tokens TODAY, so a real
# unguarded release build added to it would have passed with NO ARM FIRING,
# which is the exact entrance this arm exists to close. NAMING A WEAKNESS IS NOT
# THE SAME AS RULING IT ACCEPTABLE FOR EVER: this one was filed as theoretical
# and had quietly graduated to load-bearing as the tree grew.
#
# FILE SCOPE REMAINS VALID EVIDENCE, DELIBERATELY: a `CARGO_TARGET_DIR` exported
# at the top of a file really does redirect every build below it, and refusing
# that would newly block correct files. What no longer counts is a token sitting
# in a DIFFERENT function from the build it is supposed to excuse.
guard_evidence() {
  awk -v target="$2" '
    /^[A-Za-z_][A-Za-z0-9_]*\(\)[[:space:]]*\{/ { inside=1; start=NR; buf=$0 "\n"; next }
    inside && /^\}/ {
      if (target >= start && target <= NR) printf "%s", buf
      inside=0; buf=""; next
    }
    inside { buf = buf $0 "\n"; next }
    { print }
  ' "$1"
}

step_cargo_census() {
  local f numbered lineno line stmt bad="" total=0 inert_set
  while IFS= read -r f; do
    # Heredoc state is a property of the FILE, not of the line, so it is
    # computed once here and consulted below. See prose_stripped for why no
    # per-line regex can answer it.
    inert_set=" $(inert_heredoc_lines "$f" | tr '\n' ' ')"
    # A line that can be release: an explicit `--release`, or a `$profile`
    # variable that expands to it. Comments and prose ABOUT cargo are excluded --
    # this arm's own ancestor failed a correct file by matching a header
    # sentence, which is this estate's oldest instrument defect.
    while IFS= read -r numbered; do
      lineno="${numbered%%:*}"
      line="${numbered#*:}"
      # Prose that only MENTIONS cargo is not an invocation. See
      # prose_stripped above for which spans can be removed and, more
      # importantly, which look removable and are not.
      case "$inert_set" in
        *" $lineno "*) stmt="$(prose_stripped "$line" inert)" ;;
        *) stmt="$(prose_stripped "$line")" ;;
      esac
      case "$stmt" in
        *"cargo build"*|*"cargo clean"*) ;;
        *) continue ;;
      esac
      case "$stmt" in
        *--release*|*'$profile'*) ;;
        *) continue ;;
      esac
      total=$((total + 1))
      if grep -q 'clone_workspace\|CARGO_TARGET_DIR\|refuse_single_package_release\|guarded_release_build' <<<"$(guard_evidence "$f" "$lineno")"; then
        continue
      fi
      bad="$bad       $f: ${line#"${line%%[![:space:]]*}"}
"
    done <<EOF
$(grep -n 'cargo build\|cargo clean' "$f" 2>/dev/null | grep -vE '^[0-9]+:[[:space:]]*#')
EOF
  done <<EOF
$(find "$ROOT/bin/.devbin" -type f 2>/dev/null | sort)
EOF
  if [ -n "$bad" ]; then
    fail "arm 10 -- a cargo invocation that can build RELEASE reaches the shared artefact with no guard:
$bad       Each must build in a clone, redirect CARGO_TARGET_DIR, refuse first, or BE
       the guarded path. A bare release build here is a new unguarded entrance,
       which is what split the pair on 2026-08-26."
  else
    ok "arm 10 -- every release-capable cargo invocation under bin/.devbin is guarded, redirected, cloned or refusing ($total examined)"
  fi
}
step_cargo_census

# ARM 11 -- THE CENSUS IS DRIVEN AGAINST A TREE WHOSE ANSWER IS KNOWN, IN BOTH
# DIRECTIONS, AND IT EXISTS BECAUSE ARM 10 WAS FALSELY GREEN TWICE IN ONE DAY.
#
# 2026-08-27, two separate mechanisms: once the escape-hatch token was accepted
# from a DIFFERENT function than the build it excused, and once the strip erased
# an unescaped backtick span, which in shell is command substitution and RUNS.
# **NEITHER WAS VISIBLE FROM ARM 10's OWN OUTPUT.** A census reports what it
# found, and a census that cannot SEE a door reports zero doors exactly as
# cheerfully as a clean tree does. `(5 examined)` was true on both days.
#
# THE GREEN HALF IS THE HALF THAT WOULD HAVE BEEN SKIPPED, AND IT IS NOT
# DECORATION: a suite of RED fixtures alone passes identically for a census that
# refuses every file, which is a freeze, and a freeze gets bypassed. All three
# GREEN fixtures are forms live in this repo TODAY -- `cmd/macos`'s quoted
# heredoc, `cmd/cli:63`'s escaped backticks, and a file-scope CARGO_TARGET_DIR --
# so they are regression arms rather than hypotheticals.
#
# THE FIXTURES ARE THE EVIDENCE, SO THEY ARE PLANTED RATHER THAN DESCRIBED. The
# two live entrances below were GREEN under the pre-2026-08-27 strip and are RED
# now; that difference is the entire claim this arm makes, and it is checkable by
# reverting `prose_stripped` and watching this arm fail.

# census_verdict <fixture-root> -- RED or GREEN, from THE REAL step_cargo_census.
# ROOT and the two reporters are shadowed inside a subshell, so the live tree and
# the live counters are untouched and the arm cannot test a copy by accident.
census_verdict() {
  (
    ROOT="$1"
    v=GREEN
    ok()   { :; }
    fail() { v=RED; }
    step_cargo_census
    printf '%s' "$v"
  )
}

# plant <root> <name> -- write <root>/bin/.devbin/cmd/<name> from stdin.
plant() {
  mkdir -p "$1/bin/.devbin/cmd"
  cat > "$1/bin/.devbin/cmd/$2"
}

step_census_selftest() {
  local base="$TMP/census" spec want got n bad="" checked=0
  rm -rf "$base"

  # RED 1. An UNESCAPED backtick span inside double quotes is command
  # substitution: this really does run a release build into the shared tree.
  plant "$base/live_backtick" live_backtick <<'FIXTURE'
#!/bin/bash
run_it() {
  die "building now: `cargo build --release --manifest-path native/rust/Cargo.toml`"
}
FIXTURE

  # RED 2. An UNQUOTED heredoc interpolates, so its backtick span runs too. This
  # is the case that makes heredoc state necessary rather than a nicety.
  plant "$base/live_heredoc" live_heredoc <<'FIXTURE'
#!/bin/bash
report() {
  cat <<USAGE
  building: `cargo build --release`
USAGE
}
FIXTURE

  # RED 3. The escape-hatch token is present in the FILE but in a different
  # function from the build, which is the file-scope hole closed earlier today.
  plant "$base/crossfunction_token" crossfunction_token <<'FIXTURE'
#!/bin/bash
safe_one() {
  clone_workspace "$tmp"
}
unsafe_one() {
  cargo build --release --manifest-path native/rust/Cargo.toml
}
FIXTURE

  # GREEN 1. Escaped backticks are literal inside double quotes. `cmd/cli:63`
  # uses exactly this to RECOMMEND the guarded path, and failing it stopped four
  # nodes on a sentence once already.
  plant "$base/escaped_backtick" escaped_backtick <<'FIXTURE'
#!/bin/bash
warn_it() {
  die "no release binary -- run \`int local build\` (a bare \`cargo build --release\` does not)"
}
FIXTURE

  # GREEN 2. A quoted heredoc body is literal text. `cmd/macos`'s help lives
  # here, and it is where this whole class started.
  plant "$base/inert_heredoc" inert_heredoc <<'FIXTURE'
#!/bin/bash
usage() {
  cat <<'USAGE'
  never on target/release, which four sessions share and any
  `cargo build --release` will overwrite underneath you.
USAGE
}
FIXTURE

  # RED 4. THE SHAPE THAT DOES NOT EXIST IN THIS TREE AND WOULD BE INVISIBLE IF IT
  # ARRIVED: a stray apostrophe BEFORE a real quoted span, with a live release
  # build between them. The old left-to-right pairing ran from `peer\'s` to the
  # opening of `\'done\'` and deleted the build; the odd-count refusal keeps it.
  # A fix without this fixture closes the hole for `cmd/macos:278` and reopens it
  # for the next line anyone writes.
  plant "$base/stray_apostrophe" stray_apostrophe <<'FIXTURE'
#!/bin/bash
run_it() {
  echo "a peer's tree is not mine" ; cargo build --release ; echo 'done'
}
FIXTURE

  # GREEN 3. A file-scope redirect really does cover every build below it, and
  # refusing that would newly block correct files.
  plant "$base/filescope_token" filescope_token <<'FIXTURE'
#!/bin/bash
export CARGO_TARGET_DIR="$private"
build_one() {
  cargo build --release --manifest-path native/rust/Cargo.toml
}
FIXTURE

  for spec in live_backtick:RED live_heredoc:RED crossfunction_token:RED \
              stray_apostrophe:RED escaped_backtick:GREEN inert_heredoc:GREEN \
              filescope_token:GREEN; do
    n="${spec%%:*}"
    want="${spec#*:}"
    got="$(census_verdict "$base/$n")"
    checked=$((checked + 1))
    [ "$got" = "$want" ] || bad="$bad $n(want $want, got $got)"
  done

  if [ -n "$bad" ]; then
    fail "arm 11 -- the census misreads its own fixtures:$bad. An arm that cannot be driven to BOTH verdicts is not evidence about the tree; it is a number that has never been wrong in front of anyone."
  else
    ok "arm 11 -- the census is driven to BOTH verdicts on planted fixtures ($checked examined: 4 live entrances RED, 3 inert forms GREEN)"
  fi
}
step_census_selftest

printf 'shared-artefact-guard: %d arm(s) passed\n' "$pass"
exit "$rc"
