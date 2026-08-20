#!/bin/bash
# canon_ignore_dispatch_rig.sh -- the canon-ignore guard is REACHABLE along the
# path git actually takes, and it refuses.
#
# ST0057 AT-01.5, covering AC-01.5: NO COMMIT MAY ADD AN IGNORE RULE MATCHING
# ANYTHING UNDER `intent/.canon/`. The guard itself was built and mutation-proven
# on 2026-08-18; what this drives is the half nothing covered -- that a real
# `git commit` reaches it.
#
# THE PAIRING IS THE INSTRUMENT, AND A ONE-SIDED RIG WOULD HAVE PASSED ON THE
# DAY THIS WAS WRITTEN. Proving the guard refuses says nothing about whether
# anything dispatches it, and on 2026-08-20 those two facts came apart: the
# guard was correct, rostered and live in the source repo, and ABSENT FROM EVERY
# CLONE. A rig that only planted the hazard in a working install would have gone
# green while a fresh clone committed the same hazard at rc=0.
#
# WHAT WAS MEASURED, so the reason survives the finding being fixed:
# `.githooks/pre-commit.intent` is gitignored by design (`.gitignore:158`) --
# `intent claude upgrade` owns it, and tracking a file the installer writes
# would give canon a second home. The consequence was not visible from the
# decision: `.githooks/pre-commit` reaches it with a bare `[ -x "$_chain" ]` and
# NO else, so in a clone the whole guard chain is a silent no-op. A clone wired
# by `int hooks --install` printed **hooks: this clone is wired** and then
# committed `intent/.*/` into `.gitignore` at rc=0 with zero guards run.
#
# ASSERTS THE `guards: N ran` LINE AND NOT ONLY THE EXIT CODE. The gate has
# eleven other arms and rc=1 is reachable from any of them, so an exit code
# alone attributes the refusal to nothing. The counted line is what names the
# dispatcher, and it exists only because a passing run was otherwise silent --
# which is this whole file's subject arriving on the success path.
#
# TWO RULE FORMS, BECAUSE ONE WOULD PASS A PATTERN-BRITTLE GUARD. `intent/.*/`
# is the plausible tidy-up the criterion was raised about -- every other
# `intent/.<x>/` in this tree IS local, so the convention reads "a dot directory
# under intent/ does not travel" and `.canon/` is the single exception.
# `intent/.canon/` is the direct form. A guard matching only the literal path
# would pass one and fail the other.
#
# THE CONSEQUENCE IS PROVEN, NOT ASSUMED -- and the first cut of that proof WAS
# NOT A CONTROL. With the rule present a NEW canon artefact must be invisible to
# `git status`; without it, visible. The first attempt compared the planted
# clone against the clone whose commit had just been REFUSED -- and a refusal
# leaves the rule in the worktree, so both sides carried it and agreed. Restore
# the file before the negative arm, or the control is a mirror.
#
# BUILDS ITS OWN CLONE AND NEVER TOUCHES THE WORKING TREE (`canon_concurrent_diff.sh`'s
# rule, same reason and more sharply here): four nodes share this checkout, and
# a planted `.gitignore` orphaning canon is a live hazard rather than a fixture.
#
# ERE (`-E`) THROUGHOUT. Under `/usr/bin/grep` (BSD) a `$` outside a pattern's
# final position is a LITERAL, so a BRE alternation anchored in each branch
# matches nothing and says so with a zero.
#
# Usage: canon_ignore_dispatch_rig.sh [--keep]      (--keep leaves the rig for inspection)
# Exit:  0 the property holds   1 the property fails   2 the rig could not establish its own control

set -u

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
KEEP=0
[ "${1:-}" = "--keep" ] && KEEP=1

RIG="$(mktemp -d "${TMPDIR:-/tmp}/canon-ignore-rig.XXXXXX")" || exit 2
cleanup() { [ "$KEEP" -eq 1 ] || rm -rf "$RIG"; }
trap cleanup EXIT

DISPATCHER="${REPO}/.githooks/pre-commit.intent"
FAILED=0

say()    { printf '%s\n' "$*"; }
fail()   { printf 'canon-ignore-dispatch: FAIL -- %s\n' "$*" >&2; FAILED=1; }
refuse() { printf 'canon-ignore-dispatch: REFUSED -- %s\n' "$*" >&2; exit 2; }

# The rig supplies the dispatcher by copying the one this install already has.
# `intent claude upgrade --apply` is the real-world equivalent and was driven
# once to confirm it writes the same file; copying is used here because a full
# upgrade rewrites a project's LLM canon, which is not this instrument's
# business. Absent, the rig has nothing to prove a positive with.
[ -f "$DISPATCHER" ] || refuse "no dispatcher at ${DISPATCHER}; this install cannot supply a wired clone"

# ---------------------------------------------------------------------------
# make_clone <dir> <wired|bare>
#   wired -- core.hooksPath set AND the dispatcher present: what a consumer has
#            after `int hooks --install` AND `intent claude upgrade --apply`.
#   bare  -- core.hooksPath set and the dispatcher ABSENT: what a consumer has
#            after `int hooks --install` alone, which is what the docs name.
# ---------------------------------------------------------------------------
make_clone() {
  local dir="$1" mode="$2"
  git clone --local --quiet "$REPO" "$dir" 2>/dev/null || refuse "could not clone ${REPO}"
  git -C "$dir" config core.hooksPath .githooks || refuse "could not set core.hooksPath in ${dir}"
  if [ "$mode" = "wired" ]; then
    cp "$DISPATCHER" "${dir}/.githooks/pre-commit.intent" || refuse "could not place the dispatcher"
    chmod +x "${dir}/.githooks/pre-commit.intent"
  fi
  [ -e "${dir}/intent/.canon" ] || refuse "the clone carries no intent/.canon, so the guard's subject is absent"
}

# plant <dir> <rule> -- append an ignore rule and stage it. Returns the log path.
plant() {
  local dir="$1" rule="$2" log="$3"
  printf '\n# planted by canon_ignore_dispatch_rig.sh\n%s\n' "$rule" >> "${dir}/.gitignore"
  git -C "$dir" add .gitignore
  git -C "$dir" commit -m "rig: plant ${rule}" > "$log" 2>&1
  return $?
}

# ---------------------------------------------------------------------------
# ARM 1 -- THE PROPERTY. A wired clone REFUSES both rule forms, and the refusal
# is attributable to the guard rather than to any other arm of the gate.
# ---------------------------------------------------------------------------
say "==> arm 1: a wired clone refuses the hazard, in both rule forms"
for rule in 'intent/.*/' 'intent/.canon/'; do
  d="${RIG}/wired-$(printf '%s' "$rule" | tr -c 'a-z' '-')"
  make_clone "$d" wired
  log="${d}.log"
  plant "$d" "$rule" "$log"
  rc=$?

  if [ "$rc" -eq 0 ]; then
    fail "the wired clone COMMITTED '${rule}' at rc=0 -- the criterion is unmet"
    continue
  fi

  # rc is non-zero. Attribute it, or it proves nothing: the gate has eleven
  # other arms and any of them reaching exit 1 looks identical from here.
  if ! grep -qE '^guards: [0-9]+ ran' "$log"; then
    fail "'${rule}' was refused with NO counted guard line -- the refusal is unattributed, and an unattributed refusal is not evidence the guard ran"
    continue
  fi
  if ! grep -qE 'ignore rule that reaches intent/\.canon/' "$log"; then
    fail "'${rule}' was refused by something that is not the canon-ignore guard"
    continue
  fi
  ran="$(grep -oE '^guards: [0-9]+ ran' "$log" | grep -oE '[0-9]+')"
  say "  ok  '${rule}' refused at rc=${rc}, ${ran} guard(s) ran, message names the rule"
done

# ---------------------------------------------------------------------------
# ARM 2 -- THE NEGATIVE. Without a planted rule the same wired clone must PASS.
# Arm 1 alone is satisfiable by a gate that refuses everything.
# ---------------------------------------------------------------------------
say "==> arm 2: the same wired clone passes a commit carrying no ignore rule"
d="${RIG}/wired-negative"
make_clone "$d" wired
printf 'negative arm\n' > "${d}/RIG_NEGATIVE.txt"
git -C "$d" add RIG_NEGATIVE.txt
git -C "$d" commit -m "rig: negative arm, no ignore rule" > "${d}.log" 2>&1
rc=$?
if [ "$rc" -ne 0 ]; then
  fail "the wired clone refused a commit with NO ignore rule (rc=${rc}) -- arm 1's refusals cannot be attributed to the hazard"
elif ! grep -qE '^guards: [0-9]+ ran' "${d}.log"; then
  fail "the negative arm passed with no counted guard line -- silence on success is indistinguishable from not running, which is what the count exists to end"
else
  say "  ok  passed at rc=0, $(grep -oE '^guards: [0-9]+ ran' "${d}.log" | grep -oE '[0-9]+') guard(s) ran"
fi

# ---------------------------------------------------------------------------
# ARM 3 -- THE CONSEQUENCE. The hazard is real: with the rule in place a NEW
# canon artefact is invisible to git; with it removed, visible. Existing canon
# stays TRACKED either way -- git does not un-track on an ignore rule -- so a
# check that looked only at what is already committed would find nothing wrong.
# ---------------------------------------------------------------------------
say "==> arm 3: the rule actually orphans canon, and the control is a real one"
d="${RIG}/consequence"
make_clone "$d" bare                      # bare: the commit must LAND for the state to exist
printf '\nintent/.*/\n' >> "${d}/.gitignore"
printf '{}\n' > "${d}/intent/.canon/st/ST9999.json"

if git -C "$d" check-ignore -q intent/.canon/st/ST9999.json; then
  say "  ok  with the rule, a new canon artefact is ignored"
else
  fail "the planted rule does not hide a new canon artefact -- the hazard this criterion names is not reproduced, so nothing here is evidence about it"
fi

# THE CONTROL, AND IT IS THE HALF THAT WAS WRONG FIRST. Restore `.gitignore`
# before asking the negative question, or the comparison is between two trees
# that both carry the rule and they will agree.
git -C "$d" checkout -- .gitignore 2>/dev/null || git -C "$d" restore --worktree .gitignore 2>/dev/null
if grep -qE '^intent/\.\*/$' "${d}/.gitignore"; then
  refuse "could not restore .gitignore for the control arm; a comparison against a tree that still carries the rule is a mirror, not a control"
fi
if git -C "$d" check-ignore -q intent/.canon/st/ST9999.json; then
  fail "the same artefact is still ignored with the rule removed -- something other than the planted rule is hiding canon, and arm 3's positive says nothing"
else
  say "  ok  with the rule removed, the same artefact is visible again"
fi

# ---------------------------------------------------------------------------
# ARM 4 -- REACHABILITY, REPORTED AND NOT GATED. Does a clone reach the guard
# chain from `core.hooksPath` ALONE? On 2026-08-20 it did not, and the remedy
# belongs to whoever owns the hook layout. This arm therefore NAMES the state
# rather than failing on it: a rig that went red on someone else's open
# decision would be bypassed, and a rig silent about it would let the condition
# come back unobserved.
# ---------------------------------------------------------------------------
say "==> arm 4: what a clone reaches from core.hooksPath alone (REPORT, never a gate)"
d="${RIG}/bare-reach"
make_clone "$d" bare
printf '\nintent/.canon/\n' >> "${d}/.gitignore"
git -C "$d" add .gitignore
git -C "$d" commit -m "rig: reachability probe" > "${d}.log" 2>&1
rc=$?
if [ "$rc" -eq 0 ] && ! grep -qE '^guards: [0-9]+ ran' "${d}.log"; then
  say "  REPORT: a clone with core.hooksPath set and no dispatcher runs NO guards"
  say "          and commits the hazard at rc=0. The dispatcher is gitignored, so"
  say "          this is the state of every clone until \`intent claude upgrade --apply\`."
  say "          AC-01.5 holds for this install and NOT for a consumer following the docs."
elif [ "$rc" -ne 0 ]; then
  say "  REPORT: a clone reaches the guard from core.hooksPath alone and refused (rc=${rc})."
  say "          The 2026-08-20 gap is closed; arm 1 is now the whole criterion."
else
  say "  REPORT: inconclusive -- the commit landed but a guard line was printed; read ${d}.log"
fi

say ""
if [ "$FAILED" -eq 0 ]; then
  say "canon-ignore-dispatch: the guard is reachable and refuses; the negative arm passes; the hazard reproduces."
  exit 0
fi
exit 1
