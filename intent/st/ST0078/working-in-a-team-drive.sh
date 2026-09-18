#!/usr/bin/env bash
# ST0078 P4 (AC-04.1): every command docs/concepts/working-in-a-team.md shows,
# driven in the order the page shows it. Two people, one bare origin, NO
# daemon, an isolated HOME. Based on vc's drive at refs/bank/vc/st0078/drive.sh.
#
# Usage: working-in-a-team-drive.sh <scratch-dir> <dir-holding-intent-and-intentd> [<a pre-P1 pair>]
# The third argument is a pair built before events travelled; section 6b makes a
# project with it, then upgrades that project with the pair under test.
# The pair must carry P1 (events travel), P2 (renumber), P3 (the ingest and the
# git hooks) and P5 (`intent sync --apply [--yes]`).
set -u
S="$1"; BIN="$2"; OLD="${3:-}"
rm -rf "$S"; mkdir -p "$S/home" "$S/run"
export PATH="$BIN:$PATH"
export HOME="$S/home" XDG_CONFIG_HOME="$S/home/.config" XDG_DATA_HOME="$S/home/.local/share" XDG_STATE_HOME="$S/home/.local/state" XDG_RUNTIME_DIR="$S/run"
mkdir -p "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_STATE_HOME"; unset CLAUDE_CODE_SESSION_ID
h() { printf '\n===== %s\n' "$*"; }
run() { printf '$ %s\n' "$*"; "$@" 2>&1; printf '  rc=%s\n' "$?"; }
# `intent st new`, printed like run(), leaving the minted id in $ID.
mint() { local out rc; printf '$ intent st new %s\n' "$1"; out=$(intent st new "$1" 2>&1); rc=$?; printf '%s\n  rc=%s\n' "$out" "$rc"; ID=$(printf '%s\n' "$out" | sed -n 's/^created: //p'); }
commit() { git add -A && git commit -q -m "$1" >/dev/null 2>&1 && echo "committed: $1" || { echo "COMMIT REFUSED: $1"; git add -A; git commit -m "$1" 2>&1 | tail -12; }; }
printf 'pair: %s\n' "$(command -v intent)"; run intent --version

git init -q --bare -b main "$S/origin.git"

h "1 alice: start the project and share it"
git clone -q "$S/origin.git" "$S/alice" 2>/dev/null && cd "$S/alice" && git config user.name Alice && git config user.email alice@example.com
run intent init Team
run grep -n -E '^intent/(\.cache|\.backup|events\.jsonl)' .gitignore
run intent bootstrap
run intent claude upgrade --apply --skip-settings
commit "intent init"; run git push -q -u origin main

h "1 bob: clone it, then once per machine and once per clone"
git clone -q "$S/origin.git" "$S/bob" && cd "$S/bob" && git config user.name Bob && git config user.email bob@example.com
run intent bootstrap
run intent claude upgrade --apply --skip-settings
run ls .git/hooks
run intent st list --status all

h "2 alice: a thread on a branch, reviewed as a PR"
cd "$S/alice"; run git switch -c alice/onboarding
mint "Onboarding guide"; ONBOARD=$ID
run intent st start "$ONBOARD"; run intent wp new "$ONBOARD" "Write the guide"
run git status --short
commit "$ONBOARD: onboarding guide, WP-01"; run git push -q -u origin alice/onboarding
run git diff --stat main...alice/onboarding
run git switch -q main; run git status --short
run git merge -q --no-ff alice/onboarding -m "Merge alice/onboarding"; run git push -q origin main

h "3 bob: pull, and the store follows"
cd "$S/bob"; run git pull -q; run intent st list --status all
cd "$S/alice"; mint "Release checklist"; RELEASE=$ID; commit "$RELEASE: release checklist"; run git push -q origin main
cd "$S/bob"; run git pull -q
run intent st list --status all; run intent st show "$RELEASE"; run intent doctor

h "4 both mint one id: renumber before the merge"
cd "$S/alice"; mint "Alice's next"; commit "alice: $ID"; run git push -q origin main
cd "$S/bob"; mint "Bob's next"; MINE=$ID; commit "bob: $MINE"
run git pull --no-rebase -q; run git diff --name-only --diff-filter=U; run git merge --abort
FREE=$(printf 'ST%04d' $(( 10#${MINE#ST} + 1 )))
run intent st renumber "$MINE" "$FREE"
commit "renumber my $MINE to $FREE"
run git pull --no-rebase -q; run git diff --name-only --diff-filter=U
run intent sync --apply --yes
run git diff --name-only --diff-filter=U
commit "merge main: Alice's $MINE, mine is $FREE"; run git push -q origin main
run intent st list --status all; run intent doctor

h "4b both mint one id again: one command repairs it mid-merge"
cd "$S/alice"; run git pull -q; mint "Alice's fifth"; commit "alice: $ID"; run git push -q origin main
cd "$S/bob"; mint "Bob's fifth"; commit "bob: $ID"
run git pull --no-rebase -q; run git diff --name-only --diff-filter=U
run intent sync
run intent sync --apply
run intent sync --apply --yes
run git diff --name-only --diff-filter=U
commit "merge main: the one command renumbered mine"; run git push -q origin main
run intent st list --status all; run intent doctor

h "5 both edit one thread"
cd "$S/alice"; run git pull -q
run intent set "$RELEASE" objective "Bring a new engineer to a first merged PR in a day"; commit "$RELEASE objective"; run git push -q origin main
cd "$S/bob"; run intent set "$RELEASE" context "Requested by support after three onboarding escalations"; commit "$RELEASE context"
run git pull --no-rebase -q; run git diff --name-only --diff-filter=U
run grep -n -E '^(<<<<<<<|=======|>>>>>>>)|"objective"|"context"' "intent/.canon/st/$RELEASE.json"
run intent sync
python3 - "$RELEASE" <<'PY'
import re, sys
p="intent/.canon/st/%s.json" % sys.argv[1]; t=open(p).read()
t=re.sub(r"<<<<<<< [^\n]*\n(.*?)=======\n(.*?)>>>>>>> [^\n]*\n", lambda m: "".join(l+"\n" for l in dict.fromkeys((m.group(1)+m.group(2)).splitlines()) if not l.strip().startswith(('"objective": ""','"context": ""'))), t, flags=re.S)
open(p,"w").write(t); print("(edited %s by hand: kept both fields)" % p)
PY
run git add "intent/.canon/st/$RELEASE.json"
run intent sync --apply --yes
run git diff --name-only --diff-filter=U
commit "merge: both $RELEASE edits"; run git push -q origin main
run grep -n -E '"objective"|"context"' "intent/.canon/st/$RELEASE.json"; run intent doctor

h "6 history travels"
cd "$S/alice"; run git pull -q
run intent events --subject "$RELEASE"
cd "$S/bob"; run intent events --subject "$RELEASE"

h "6b a project whose history predates the event files: intent upgrade backfills it"
if [ -n "$OLD" ]; then
  mkdir -p "$S/carol" && cd "$S/carol" && git init -q -b main && git config user.name Carol && git config user.email carol@example.com
  printf '(with the pre-P1 pair: %s)\n' "$("$OLD/intent" --version)"
  PATH="$OLD:$PATH" intent init Legacy >/dev/null 2>&1; PATH="$OLD:$PATH" intent st new "Before events travelled" >/dev/null 2>&1; PATH="$OLD:$PATH" intent st start ST0001 >/dev/null 2>&1
  git add -A && git -c core.hooksPath=/dev/null commit -q -m "legacy project" && echo "committed: legacy project"
  run ls intent/.canon
  printf '(with the pair under test from here)\n'
  run intent upgrade
  run git status --short --untracked-files=all
  run intent upgrade
  run git status --short --untracked-files=all
else
  echo "SKIPPED: no pre-P1 pair given"
fi

h "6c a canon file written by a newer Intent"
cd "$S/bob"; run git status --short
run grep -n '"schema"' "intent/.canon/st/$RELEASE.json"
sed -i.bak 's#"schema": "intent/thread@3.0"#"schema": "intent/thread@9.0"#' "intent/.canon/st/$RELEASE.json" && rm -f "intent/.canon/st/$RELEASE.json.bak"
printf '(rewrote %s schema to intent/thread@9.0, a version this binary does not know)\n' "intent/.canon/st/$RELEASE.json"
run grep -n '"schema"' "intent/.canon/st/$RELEASE.json"
run intent sync
run intent sync --apply
run intent st show "$RELEASE"
run intent doctor
run git checkout -- "intent/.canon/st/$RELEASE.json"
run intent sync --apply
run git status --short

h "7 the CI job, run by hand on a fresh clone of the merge result"
git clone -q "$S/origin.git" "$S/ci" && cd "$S/ci"
run intent doctor

h "8 the-store.md: does the commit gate check canon against an unsynced edit"
cd "$S/bob"; run intent st start "$FREE"
run ls "intent/st/$FREE/info.md"
python3 - "$FREE" <<'PY'
import re, sys
p="intent/st/%s/info.md" % sys.argv[1]; t=open(p).read()
t=re.sub(r"(## Objective\n\n)", r"\1Edited by hand and not synced. ", t, count=1)
open(p,"w").write(t); print("(edited %s's Objective by hand, no sync)" % p)
PY
commit "unsynced objective edit"
run git log -1 --format=%s
run intent doctor

h "9 a reset the hooks do not see"
cd "$S/alice"; run git pull -q
run intent set "$RELEASE" title "Release checklist, v2"; commit "$RELEASE retitled"; run git push -q origin main
cd "$S/bob"; run git fetch -q origin; run git reset -q --hard origin/main
run intent doctor
run intent set "$RELEASE" context "Requested by support; retitled upstream"
run git diff --stat
run git checkout -- .
run intent sync --apply
run intent set "$RELEASE" context "Requested by support; retitled upstream"
run git diff --stat
