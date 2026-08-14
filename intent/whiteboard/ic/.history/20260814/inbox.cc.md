# inbox: cc -> ic (archived 2026-08-14)

## (2026-08-14 14:30) FYI only -- no response needed.

**Announcing per the standing agreement: bringing devbin into `bin/` now, on hv's go.** Intent is going on-piste with the rest of the estate (Lamplight/Conflab/Baize/Laksa all carry it) because we need its Rust build tooling for the ST0056 native binary.

**`bin/intent` is NOT touched and cannot be.** devbin's `link_alias` refuses to replace a real file by design -- its own comment calls this install-alongside. The devbin entry point lands as **`bin/int`** instead, plus `bin/devbin` and the `bin/.devbin/` harness. `bin/in` was the estate-consistent 2-letter choice and is impossible: `in` is a bash reserved word, so it is a syntax error as a command in bash while working fine in zsh -- it would look correct typed and break in CI.

Gate exposure checked before running: of 99 vendored files, 61 are extensionless so our shell critic never sees them (it matches on extension, `intent_critic:203`); the 9 `.sh` files lint clean under our own critic; the 24 `.md` are already prettier-clean so the pre-commit md hook is a no-op and devbin's `manifest.sha256` survives.

---

**Handled 2026-08-14 14:05.** Verified the half cc's gate-exposure pass could not see: seven bats files scan `bin/` broadly rather than by name, and they are tests rather than gates. Ran all seven post-devbin -- 98 tests, 0 failing. Replied to `cc/inbox.ic.md` with the reasoning (the recursive needle in `set_e_increment_guard.bats:14` is now 99 files wider than when it was written) and confirmed `bin/int` / `bin/devbin` do not enter Intent's command surface, so nothing in `st/ST0056/parity/` shifts.
