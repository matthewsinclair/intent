#!/usr/bin/env bash
#
# pre-commit-guards.sh -- THE GUARD ROSTER, and the loop that dispatches it.
#
# READ LIVE OUT OF `INTENT_HOME`. NEVER COPIED INTO A PROJECT. That sentence is
# the entire reason this file exists as a file, so it goes first.
#
# WHAT WENT WRONG, MEASURED ON THIS REPOSITORY 2026-08-20 AT `5dbac6fb`.
# `pre-commit.sh` was then copied verbatim into `.git/hooks/pre-commit.intent`
# by `intent claude upgrade --apply` (today that path holds the shim
# `pre-commit-shim.sh`, which execs `pre-commit.sh` live), and it used to carry
# the roster INSIDE it.
# The guard bodies were read live from `INTENT_HOME`, and its own comment said
# in these words that this "makes a new guard propagate without touching a
# consumer's .git/hooks/". **The bodies propagated. The roster did not.** So
# adding a guard to canon reached nobody until they reinstalled the hook. The
# shipped roster named more guards than this repo's installed pre-commit.intent
# ran (its copy was hardcoded), `bin/int precommit` compensated for only part of
# the gap, and canon-ignore-guard.sh and append-only-guard.sh ran NOWHERE in
# this repository.
#
# `append-only-guard.sh` was written because `.history/` lines were destroyed
# on 2026-08-17 and events on 2026-08-19. It protected nothing here
# from the day it was written. **A guard nothing dispatches is indistinguishable
# from a guard that passes** -- this file's own subject matter, happening to the
# mechanism, for days, in silence.
#
# So the rule is now structural rather than remembered: the copied file names no
# guard and holds no roster, and there is exactly one place a guard is declared.
# Adding one is a line in the array below and it reaches every consumer with no
# reinstall, no version bump and nothing to remember.
#
# THE ABSENCES MUST STAY APART, one level up from the hook that first collapsed them.
# The hook already distinguished "the resolver did not answer" (ALL guards
# missing) from "one guard file is missing" (one hole), because collapsing them
# printed one benign "not found" per guard while the gate was in fact not
# running. Delegating the roster adds another: **the runner itself missing**,
# which is again all-guards-missing but has a different remedy -- nothing is
# wrong with the guards and nothing is wrong with the resolver; the install is
# older than this mechanism. The hook owns absences 1 and 2 because they are
# about locating THIS file. This file owns absence 3.

set -u

# THE ROSTER. One guard per concern, declared once, here.
#
# Format is `applies-when|basename|what goes unchecked if it is missing`.
#
# ONE GUARD PER CONCERN, NOT ONE GUARD THAT GREW. The clock guard checks
# TIMESTAMPS (every check is about clocks); the header guard checks the
# HEADER BLOCK's format contract. They were kept apart by ruling (vc,
# 2026-08-16): folding a second concern into a file named for the first makes
# its name lie to the next reader, and it couples two controls that should be
# independently canaried and independently disabled.
#
# ALL OF THEM RUN, THEN THE GATE DECIDES. Stopping at the first refusal costs a
# node one commit attempt per defect, and a board with a bad stamp AND an
# escaped value is one editing session, not two. Each guard prints its own
# report; this loop only aggregates the verdict.
#
# THE APPLICABILITY TEST IS PER-GUARD AND NOT PER-ROSTER (dc 2026-08-19, hv
# approved). The roster was whiteboard-only and so was its gate -- one
# `[ -d intent/whiteboard ]` around the lot -- which meant a guard about
# anything else had nowhere to be declared. `applies-when` is a PATH whose
# presence makes the guard relevant, so a project without a board runs no
# whiteboard guard and a project without canon runs no canon guard.
GUARDS=(
  'intent/whiteboard|whiteboard-clock-guard.sh|timestamps are UNCHECKED'
  'intent/whiteboard|whiteboard-header-guard.sh|header values are UNCHECKED'
  'intent/.canon|canon-ignore-guard.sh|an ignore rule reaching canon is UNCHECKED'
  # `intent` RATHER THAN EITHER SUBJECT, AND THE WIDTH IS DELIBERATE (cc's
  # proposal, taken). This guard's subjects are `intent/whiteboard/*/.history/**`
  # and `intent/.canon/events/**`, so neither path alone is right, and an entry per subject
  # would dispatch one guard twice. `intent` is the smallest path containing both.
  #
  # IT DOES WEAKEN THE PROPERTY ARGUED FOR ABOVE, and saying so is cheaper than
  # discovering it: this guard opts in by BEING AN INTENT PROJECT rather than by
  # its subjects existing, so an Intent project with neither an events log nor a
  # board still pays for it. The cost is one `git diff --cached --numstat` that
  # returns empty and exits 0, and **the guard re-tests both subjects itself
  # regardless of why it was dispatched** -- so `applies-when` is a cheap
  # pre-filter here and not the real gate.
  #
  # The event-log subject is the per-event layout (Intent 3.1.0): one committed
  # file per event under `intent/.canon/events/`, written once and never
  # rewritten or deleted. It replaced `intent/events.jsonl`, a
  # file no verb writes since 0457. `.history/**` is untouched, every node
  # folds into it daily, and it is where the first loss happened.
  'intent|append-only-guard.sh|a write where an append was meant is UNCHECKED'
  # THE APPLICABILITY IS THE CONFIG FILE AND THE REAL TEST IS INSIDE THE GUARD
  # for the reason the append-only row above states about itself:
  # `applies-when` is a PATH test, and what makes this guard relevant is a
  # DECLARATION -- `"formatters"` in that same config. Every Intent project has
  # the file, so the row dispatches the guard everywhere and the guard reports
  # NOT APPLICABLE, in its own verdict, wherever nothing is declared.
  #
  # NAMING THE CONFIG AND NOT `intent` IS WHAT MAKES THE ROW READ TRUE. A wider
  # path would dispatch it in a project whose config had been removed, where it
  # cannot answer at all, and `--list-guards` would report it applicable.
  'intent/.config/config.json|staged-format-guard.sh|staged bytes are UNCHECKED against the declared formatters|self'
)

# ---- THE ROSTER ROW'S SHAPE HAS ONE HOME, AND THESE TWO FUNCTIONS ARE IT ----
#
# A row is `applies-when|guard|unchecked-prose` with an OPTIONAL fourth field,
# `self`, declaring that the guard settles its own applicability.
# Both loops below need to read it and neither may spell the split itself: the
# prose field used to be `${g_rest#*|}`, which takes the REST OF THE LINE, so a
# fourth field would have landed silently inside the prose in one loop and been
# invisible in the other. **That is the same defect as `int hooks` reading five
# tab columns where a sixth would land inside the fifth** -- the shape below
# records it once so a fifth roster field cannot reintroduce it.
#
# THE PROSE FIELD MUST NOT CONTAIN A PIPE. It does not today, measured across
# the whole roster before this split was written, and the cost of one appearing
# is that its tail would read as a flag. A row is authored here and nowhere
# else, so this is a rule for whoever adds the next one rather than a parse to
# harden -- stated because an unstated invariant is how the next reader breaks it.

# The unchecked-prose field: everything after the guard name, minus any flags.
roster_unchecked() {  # roster_unchecked "<guard>|<prose>[|<flag>]"
  local tail="${1#*|}"
  printf '%s' "${tail%%|*}"
}

# The self-classification flag, empty when the row does not declare one.
roster_self() {  # roster_self "<guard>|<prose>[|<flag>]"
  local tail="${1#*|}"
  case "$tail" in
    *\|*) printf '%s' "${tail#*|}" ;;
    *) printf '' ;;
  esac
}

# THE GUARDS ARE THIS FILE'S SIBLINGS, BY CONSTRUCTION.
#
# Derived from our own path rather than taken as an argument or re-resolved from
# `intent info`. The caller already resolved an install in order to find us, and
# asking a second time invites the two answers to differ -- a half-resolved
# `INTENT_HOME` could then pair this roster with another install's guard bodies,
# or with none, and the failure would look like "guard missing" rather than like
# the resolution bug it is. Siblings cannot disagree about which install they
# are in.
GUARD_HOME="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ---- THE PROJECT'S OWN GUARDS ----
#
# `.git/hooks/` is untracked in every repository, and so is the `core.hooksPath`
# line in `.git/config` that points git at a tracked hooks directory. A guard a
# project wires by hand is therefore a property of ONE CHECKOUT: a fresh clone
# runs Intent's roster above and not the project's own guards, and nothing says
# so. Lamplight lost four that way.
#
# So a project DECLARES its guards in its tracked `intent/.config/config.json`,
# and they run here, after Intent's roster, through the same dispatch and the
# same reporting -- a clone runs the set the original checkout ran:
#
#   "guards": [
#     {"run": ["bin/hooks/guards/whiteboard-inbox-guard.sh"], "when": "intent/whiteboard"},
#     {"run": ["bin/int", "precommit"]}
#   ]
#
# `run` is an argv resolved from the repository root, and its first element is
# a TRACKED, EXECUTABLE file. `when` is optional and means what the roster's
# `applies-when` means: a path whose absence makes the guard not applicable.
#
# **A DECLARED GUARD THAT CANNOT RUN BLOCKS THE COMMIT** (vc, 2026-09-16), which
# is stricter than a missing roster guard above. That one is an install older
# than its roster; this one is a tracked declaration naming a body the tree does
# not carry, runnable, tracked -- a broken tree, and a commit that passes over
# it would be the silent loss this block exists to end. An absent `when` path is
# not applicable and never blocks.
PROJECT_CONFIG="intent/.config/config.json"

# One line per declared guard: `<index> US <when> US <run as JSON>`, or
# `! US <reason>` for a declaration that cannot be read. Nothing when the
# project declares no guards.
#
# **US (0x1f), NOT TAB, AND A TAB WAS THE FIRST SPELLING.** Tab is whitespace
# to `read`, so the empty `when` of a guard that always applies collapsed into
# the next field: `run` was read as `when`, found absent, and the guard was
# skipped as not applicable -- silently, which is this block's own defect. A
# non-whitespace separator keeps an empty field a field.
P_SEP="$(printf '\037')"
project_guard_rows() {
  [ -f "$PROJECT_CONFIG" ] || return 0
  grep -q '"guards"' "$PROJECT_CONFIG" 2>/dev/null || return 0
  if ! command -v jq >/dev/null 2>&1; then
    printf '!%s%s\n' "$P_SEP" "${PROJECT_CONFIG} declares guards and jq is not installed, so they cannot be read"
    return 0
  fi
  jq -r '
    if .guards == null then empty
    elif (.guards | type) != "array" then "!\u001fguards in intent/.config/config.json is \(.guards | type), not an array"
    else .guards | to_entries[] | .key as $i | .value as $g
      | if ($g | type) != "object"
          or ($g.run | type) != "array"
          or ($g.run | length) == 0
          or ([$g.run[] | type] | any(. != "string"))
          or (($g.when // "") | type) != "string"
        then "!\u001fguards[\($i)] in intent/.config/config.json needs `run`, a non-empty array of strings, and an optional string `when`"
        else "\($i)\u001f\($g.when // "")\u001f\($g.run | @json)"
        end
    end' "$PROJECT_CONFIG" 2>/dev/null \
    || printf '!%s%s\n' "$P_SEP" "${PROJECT_CONFIG} is not valid JSON, so its guards cannot be read"
}

# Why a declared guard's first element cannot run, or nothing when it can.
project_guard_fault() {
  if [ ! -f "$1" ]; then
    echo "MISSING"
  elif [ ! -x "$1" ]; then
    echo "not-executable"
  elif ! git ls-files --error-unmatch -- "$1" >/dev/null 2>&1; then
    echo "untracked"
  fi
}

# --list-guards -- ANSWER WHAT IS ENFORCED, WITHOUT ENFORCING ANYTHING.
#
# **`int hooks` REPORTED THIS GATE AS ONE LINE AND NAMED NONE OF ITS GUARDS**,
# so the tool an operator consults to find out what the gate enforces
# under-reported it by the whole of this roster. It printed the
# repo-local guards and, for the shipped ones, `pre-commit.intent (critic + the
# shipped guard roster)` -- which is presence, not membership. **A check that
# under-reports what a gate enforces is worse than no check** is that command's
# own sentence, in its own header, about the defect it was written to close.
#
# THIS ARM IS FIRST AND RETURNS BEFORE ANY DISPATCH, WHICH IS THE WHOLE
# CONTRACT. `int hooks` is a read-only report, and it has already been burned
# once by a probe with a side effect: it grepped `cmd/prepush` for the string
# `--list-guards`, matched a COMMENT, invoked it, and prepush -- which ignores
# unknown flags and falls through -- cloned the repository and cold-built it,
# every time anyone asked what the hooks were wired to. **Never ask a question
# by running the thing.**
#
# THE RUNNER IS THE AUTHORITY ON ITS OWN ROSTER, and that is why this is a flag
# rather than something `int hooks` derives. The previous arrangement grepped a
# runner's source for a path shape and under-reported a gate's guards
# WITHIN THE DAY, because a guard had been implemented inline and matched no
# path. A roster restated anywhere but here goes stale the moment a guard is
# added -- and this file exists because exactly that happened to the copied
# hook it replaced.
#
# IT REPORTS PRESENCE ITSELF RATHER THAN LETTING THE CALLER STAT THE PATH.
# These guards live beside THIS file, under the resolved install, and a caller
# in a consumer repository has no reason to be able to resolve that -- it would
# stat `<repo>/<name>`, find nothing, and report every shipped guard MISSING.
# The one process that knows `GUARD_HOME` is the one that computed it.
if [ "${1:-}" = "--list-guards" ]; then
  for g_entry in "${GUARDS[@]}"; do
    g_when="${g_entry%%|*}"
    g_rest="${g_entry#*|}"
    g_name="${g_rest%%|*}"
    g_self="$(roster_self "$g_rest")"
    if [ ! -f "$GUARD_HOME/$g_name" ]; then
      g_state="MISSING"
    elif [ ! -e "$g_when" ]; then
      # NOT APPLICABLE IS NOT A FAULT, and it is reported as its own word for
      # the same reason the dispatch loop keeps them apart: a project with a
      # board and no canon has no hole where the canon guard would be.
      #
      # **AND THE OTHER KIND OF NOT-APPLICABLE NOW HAS A WORD HERE TOO, ONE
      # ARM DOWN**. A guard whose
      # applicability is a DECLARATION rather than a path cannot be asked
      # without being RUN, and this arm returns before any dispatch on purpose:
      # the header above records what it cost the last time a read-only report
      # answered a question by running the thing. So the roster DECLARES it and
      # nothing is executed to find out.
      g_state="not-applicable"
    elif [ -n "$g_self" ]; then
      # **THIS SAYS THE GUARD DECIDES, NOT WHAT IT DECIDED**, and the
      # distinction is the whole honesty of the column. The roster's path test
      # says yes; whether the guard then finds itself applicable is a fact
      # about THIS COMMIT, which only the run knows. A reader learns WHICH
      # guards settle their own applicability and must read the run's tally to
      # learn what any of them answered.
      g_state="self-classifying"
    else
      g_state="present"
    fi
    printf '%s\t%s\t%s\t%s\t%s\n' "$g_name" "$GUARD_HOME/$g_name" "$g_when" "$g_state" "intent"
  done
  # THE 5TH COLUMN SAYS WHOSE GUARD IT IS. Appended rather than
  # inserted, so a reader that SPLITS the row keeps its first four fields -- but a
  # bash `read` of four names hands the fourth the rest of the line, so every
  # such reader must name the fifth (`int hooks` does).
  while IFS="$P_SEP" read -r p_index p_when p_run; do
    if [ "$p_index" = "!" ]; then
      printf '%s\t%s\t%s\t%s\t%s\n' "-" "-" "-" "unreadable: ${p_when}" "project"
      continue
    fi
    p_first="$(jq -r '.[0]' <<<"$p_run")"
    p_state="$(project_guard_fault "$p_first")"
    if [ -z "$p_state" ]; then
      if [ -n "$p_when" ] && [ ! -e "$p_when" ]; then
        p_state="not-applicable"
      else
        p_state="present"
      fi
    fi
    printf '%s\t%s\t%s\t%s\t%s\n' "$(jq -r 'join(" ")' <<<"$p_run")" "$PWD/$p_first" "${p_when:--}" "$p_state" "project"
  done < <(project_guard_rows)
  exit 0
fi

BLOCKED=0
RAN=0
SKIPPED=0
MISSING=0

for g_entry in "${GUARDS[@]}"; do
  g_when="${g_entry%%|*}"
  g_rest="${g_entry#*|}"
  g_name="${g_rest%%|*}"
  g_unchecked="$(roster_unchecked "$g_rest")"

  # NOT APPLICABLE IS SILENT, AND ABSENT IS LOUD. A project with a board and no
  # canon must not be told a canon guard did not run -- it has nothing to guard,
  # so there is no hole. The two are different facts and only one is a finding.
  if [ ! -e "$g_when" ]; then
    SKIPPED=$((SKIPPED + 1))
    continue
  fi

  g_path="${GUARD_HOME}/${g_name}"
  if [ -f "$g_path" ]; then
    # **THE THIRD ANSWER, AND THE COUNTER IT FEEDS ALREADY EXISTED** (issue
    # 0506). Applicability is settled two ways, not one: by the PATH test above
    # for a guard whose subject is a file, and by the GUARD ITSELF for a guard
    # whose subject is a DECLARATION -- every Intent project carries
    # `intent/.config/config.json`, so a path test on it can only ever say yes.
    # Until now the runner read two answers from a guard it dispatched, 0 and
    # non-zero, so the second kind had nowhere to put its verdict and printed
    # prose instead. Prose reaches no summary, no `--list-guards` and no tally.
    #
    # 3 IS FREE AND THE OTHERS ARE NOT: 1 is BLOCKED, 2 is the shell's own error
    # (a `bash` that cannot run the file), and 0 already means ran-and-passed.
    # Censused before it was claimed: no shipped guard can reach 3 today --
    # every `exit` in the roster is a literal 0 or 1 except
    # `staged-format-guard.sh`'s `exit "$REFUSE"`, and `REFUSE` is only ever
    # assigned 0 or 1. So nothing silently changes meaning under this.
    #
    # **THE ONE PLACE IT CAN BITE IS A PROJECT-DECLARED GUARD, AND IT IS NAMED
    # RATHER THAN HIDDEN.** The loop below runs guards this runner did not ship
    # and cannot census. A project guard already exiting 3 to mean something of
    # its own stops blocking and starts reading as not-applicable. That is a
    # real behaviour change for a consumer, it is the cost of giving the class a
    # code at all, and a reader meeting it deserves to find it written down
    # here rather than to derive it from a tally that went quiet.
    bash "$g_path"
    g_rc=$?
    case "$g_rc" in
      3) SKIPPED=$((SKIPPED + 1)) ;;
      0) RAN=$((RAN + 1)) ;;
      *)
        # STILL COUNTED AS RAN, DELIBERATELY. A guard that blocked did run, and
        # it is the BLOCKED flag that carries the verdict; moving it out of RAN
        # would make the tally disagree with what happened.
        RAN=$((RAN + 1))
        BLOCKED=1
        ;;
    esac
  else
    MISSING=$((MISSING + 1))
    # Reached only with the runner located, so this really is one hole and the
    # other guards really did run. Named, not silent: a subject present with no
    # guard behind it is exactly the invisible non-enforcement this whole
    # mechanism exists to end.
    echo "intent gate: ${g_when} present but ${g_name} was not found;" >&2
    echo "  ${g_unchecked} this commit. (looked in: ${g_path})" >&2
  fi
done

# **ONE LINE ON STDOUT, AND IT EXISTS BECAUSE SILENCE ON SUCCESS IS
# INDISTINGUISHABLE FROM NOT RUNNING** (cc, 2026-08-20, measured on the first
# commit after the `core.hooksPath` redirect). Every other message in this file
# goes to stderr and only on a problem, so a passing run printed NOTHING -- and
# a runner that was never dispatched prints nothing too. That is the sentence
# this whole mechanism was built to delete, arriving on the success path.
#
# A COUNT, NOT A LIST, and the distinction is the one refused a hundred lines
# up. Listing which guards were skipped means reciting a roster the reader
# cannot check; saying HOW MANY ran answers the only question silence leaves
# open -- did anything happen. `skipped` is the not-applicable population and is
# a normal, healthy number: a project with no canon skips the canon guard and
# owes nothing.
#
# **THAT SENTENCE WAS FALSE BETWEEN 2026-09-22 AND THIS COMMIT, AND THE REPAIR
# IS WHAT MAKES IT TRUE AGAIN RATHER THAN A REWORDING OF IT**.
# When `staged-format-guard.sh` landed, the not-applicable population split
# across two counters: guards this runner settled BEFORE dispatch, counted in
# SKIPPED, and guards that settled it THEMSELVES after dispatch, counted in RAN.
# The printed gloss `(not applicable)` then admitted two readings at once --
# "this count IS the not-applicable population", which the line above asserts
# and which had stopped being true, and "skipped means not-applicable AND not
# dispatched", which was true. NEITHER PRINTED LINE EVER LIED and no count ever
# excluded anything; three sharper framings of that evidence were written and
# all three were withdrawn. The defect was that the file asserted the reading
# that had become false, which is why the fix is a CLASSIFICATION -- exit 3
# above -- and not a correction to any printed number.
#
# THE CLASS HAS NOW GIVEN WAY TWICE FOR UNRELATED REASONS, which is what makes
# it a soft spot rather than an accident: once above, where a TAB collapsed an
# empty `when` field and a guard was skipped as not-applicable silently, and
# once here. A third instance should be read as structural.
#
# `missing` is reported separately and never folded into `skipped`, because
# they are opposite facts: skipped means there was nothing to guard, missing
# means there was and no guard was there. The per-guard report above already
# names each one; this is the total, so a reader who saw no detail lines can
# still tell the difference between three and zero.
P_DECLARED=0
P_RAN=0
P_SKIPPED=0
P_REFUSED=0
while IFS="$P_SEP" read -r p_index p_when p_run; do
  P_DECLARED=$((P_DECLARED + 1))
  if [ "$p_index" = "!" ]; then
    P_REFUSED=$((P_REFUSED + 1))
    BLOCKED=1
    echo "intent gate: the project's declared guards cannot be read -- this commit is REFUSED." >&2
    echo "  ${p_when}." >&2
    echo "  a guard the project declared and nothing ran is a hole, not a pass." >&2
    echo "  remedy: correct the \`guards\` declaration (or install jq), then commit again." >&2
    continue
  fi
  if [ -n "$p_when" ] && [ ! -e "$p_when" ]; then
    P_SKIPPED=$((P_SKIPPED + 1))
    continue
  fi
  # bash 3.2 has no mapfile, and an empty-array expansion under `set -u` is an
  # error there, so the argv is read line by line and is never empty.
  p_argv=()
  while IFS= read -r p_arg; do
    p_argv+=("$p_arg")
  done < <(jq -r '.[]' <<<"$p_run")
  p_first="${p_argv[0]}"
  p_fault="$(project_guard_fault "$p_first")"
  if [ -n "$p_fault" ]; then
    P_REFUSED=$((P_REFUSED + 1))
    BLOCKED=1
    echo "intent gate: declared guard \`${p_first}\` is ${p_fault} -- this commit is REFUSED." >&2
    case "$p_fault" in
      MISSING) echo "  guards[${p_index}] in ${PROJECT_CONFIG} names a file this tree does not carry." >&2 ;;
      not-executable) echo "  the file is present and carries no execute bit, so it cannot run." >&2 ;;
      untracked) echo "  it runs in this checkout and a clone never receives it, which is the loss the declaration exists to end." >&2 ;;
    esac
    echo "  remedy: restore the guard (tracked, with its execute bit), or remove guards[${p_index}] from ${PROJECT_CONFIG}; then commit again." >&2
    continue
  fi
  # The same three answers as the shipped loop, for the same reason: a project
  # guard whose subject is a declaration rather than a path has the same claim
  # on the class. See the note above for the one case this changes for an
  # existing consumer.
  p_argv[0]="$PWD/$p_first"
  "${p_argv[@]}"
  p_rc=$?
  case "$p_rc" in
    3) P_SKIPPED=$((P_SKIPPED + 1)) ;;
    0) P_RAN=$((P_RAN + 1)) ;;
    *)
      P_RAN=$((P_RAN + 1))
      BLOCKED=1
      ;;
  esac
done < <(project_guard_rows)

printf 'guards: %d ran, %d skipped (not applicable)' "$RAN" "$SKIPPED"
[ "$MISSING" -gt 0 ] && printf ', %d MISSING' "$MISSING"
if [ "$P_DECLARED" -gt 0 ]; then
  printf '; project: %d ran, %d skipped (not applicable)' "$P_RAN" "$P_SKIPPED"
  [ "$P_REFUSED" -gt 0 ] && printf ', %d REFUSED' "$P_REFUSED"
fi
printf '\n'

exit "$BLOCKED"
