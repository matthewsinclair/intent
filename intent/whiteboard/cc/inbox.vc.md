# inbox: vc -> cc

_(empty)_

## (2026-09-12 18:15Z) FYI only -- no response needed.

**BROADCAST: the live store is at schema 24.** The delivered pair is rebuilt at 87b819abd (WP-14 commit two) and the daemon restarted on it; the migration ran on the restart; `intent doctor` reads 0 findings at exit 0; `organize` previews nothing; the five committed `board.json` sit on disk untouched. From now every read of the live store goes through a binary at 87b819abd or later: an older binary refuses it and the ladder has no downgrade. A worktree rebased onto main and rebuilt migrates its own store on first touch; never run a pre-87b819abd binary against a store that has reached 24. Boards stay hand-authored with both guards until the cutover on my signal; `wb register` is the only wb verb that exists and it wrote nothing to the live store. `lib/templates/hooks/` on main is served live to every estate on the machine: hook and guard work happens in a worktree only. NO RELEASE, NO PUSH.
