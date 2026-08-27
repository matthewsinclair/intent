#!/usr/bin/env bash
# quiet.sh -- is the tree quiet for a SHARED build? TWO reads at the moment; exit 0 only when both are empty.
#   read 1: dirt under BOTH scopes the build guard declares -- native/rust AND surface (sharedtarget.lib's
#           SHARED_TARGET_DIRT_SCOPES). dc, 2026-08-26 20:02Z: this file read native/rust alone, LOOSER than the gate
#           it fronts; a dirty surface/dispatch-table.json would have read QUIET here, the build would have been
#           REDIRECTED to target/private (never blocked), and the shared target/release -- the only dir any consumer
#           reads -- would have kept the previous pair with three green stamps, all true about the wrong build.
#   read 2: a live cargo/rustc EXECUTABLE **whose cwd is inside THIS tree**. Two axes, both paid for by a real failure:
#           * `pgrep -x` on the process NAME, never `pgrep -f` on arguments: the peer Claude sessions' argv carries the
#             restart prompt, which mentions cargo, so an argument match reports them forever (measured 2026-08-26:
#             "91 cargo processes" were three claude sessions and my own shell's echo lines).
#           * CWD-SCOPED, not machine-wide (lamplight-vc, 2026-08-27). The read was machine-wide while the hazard it
#             guards -- writing a target dir a build is using -- is PER-TARGET-DIR. It blocked Lamplight's build three
#             times on two Intent worktree builds and one Conflab build, none of which could touch Lamplight's target
#             dir. **In a fleet all converting at once that is red for everyone, permanently, on work that is none of
#             their business -- and an always-red guard is one the operator learns to wave through. lamplight-vc
#             nearly did.** A same-tree peer's hold stays meaningful; another estate's is not our business.
#   OUT-OF-SCOPE PROCESSES ARE PRINTED, NEVER SILENTLY DROPPED: a narrowing that hides what it excluded cannot be
#   distinguished from one that found nothing, and this narrowing is exactly where a wrong answer would hide.
#   CWD IS A PROXY FOR THE TARGET DIR, NOT THE TARGET DIR. A build launched from elsewhere with CARGO_TARGET_DIR
#   pointing INTO this tree reads as out of scope. Nothing cheap closes that; the shared-artefact guard is what
#   actually refuses a bad write, and this is a courtesy read in front of it.
#   a process list is a statement about the past the instant it returns; this narrows the race, it cannot close it.
#   This copy of the scope is hand-held (nothing rosters quiet.sh); widen it whenever sharedtarget.lib does.
TREE=~/Devel/prj/Intent
cd "$TREE" || exit 2
dirt=$(git status --porcelain -- native/rust surface)
mine=""; theirs=""; unknown=""
for p in $(pgrep -x cargo rustc 2>/dev/null); do
  cwd=$(lsof -a -p "$p" -d cwd -Fn 2>/dev/null | grep '^n' | cut -c2-)
  # A pid that has EXITED between the pgrep and the lsof is gone and cannot touch
  # anything -- drop it silently. A pid that still EXISTS whose cwd will not read
  # is an unknown, and an unknown counts against quiet: resolving ambiguity toward
  # "not our business" is the direction that produces a false clean (vc, against
  # its own first fix, 2026-08-27 -- five such pids were being filed as elsewhere).
  kill -0 "$p" 2>/dev/null || continue
  line="pid $p $(ps -o comm= -p "$p" 2>/dev/null) cwd=${cwd:-<unreadable>}"
  case "$cwd" in
    "")                unknown="${unknown}${line}"$'\n' ;;
    "$TREE"|"$TREE"/*) mine="${mine}${line}"$'\n' ;;
    *)                 theirs="${theirs}${line}"$'\n' ;;
  esac
done
printf 'dirt under native/rust + surface: %s path(s)\n' "$(printf '%s' "$dirt" | grep -c .)"
[ -n "$dirt" ] && printf '%s\n' "$dirt" | sed 's/^/   /'
printf 'live cargo/rustc IN THIS TREE: %s\n' "$(printf '%s' "$mine" | grep -c .)"
[ -n "$mine" ] && printf '%s' "$mine" | sed 's/^/   /'
printf 'live cargo/rustc ELSEWHERE (not our business, listed so the narrowing is visible): %s\n' "$(printf '%s' "$theirs" | grep -c .)"
[ -n "$theirs" ] && printf '%s' "$theirs" | sed 's/^/   /'
printf 'live cargo/rustc of UNKNOWN cwd (counts AGAINST quiet -- an unknown is not an elsewhere): %s\n' "$(printf '%s' "$unknown" | grep -c .)"
[ -n "$unknown" ] && printf '%s' "$unknown" | sed 's/^/   /'
if [ -z "$dirt" ] && [ -z "$mine" ] && [ -z "$unknown" ]; then echo "QUIET at $(date -u +'%H:%M:%SZ')"; exit 0
else echo "NOT quiet at $(date -u +'%H:%M:%SZ')"; exit 1; fi
