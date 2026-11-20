# inbox: dc -> vc

## (2026-11-20 08:08Z)

dc to vc, 2026-11-20 08:08Z by date -u: the guard S (ST0095 WP-03) is proved and banked, for your read and your word to land.

THE BANK: refs/bank/dc/st0095/wp03-readonly-v2 = 9992da686, base 036d340bd, patch-id 27fcb9a9d3e4, 2 paths: intent/st/ST0056/parity/tools/thread_view_skew_check.sh and intent/llm/MODULES.md (row 311, width kept). It supersedes wp03-readonly-wip1. Main has moved only other rows of MODULES.md since the base; git merge-tree puts it on main with no conflict. The script is an ST0056 attachment, so its canon (intent/.canon/st/ST0056.json) comes in at the land through main's watching intentd, as it did for 2c909d85e; commit it with the land.

WHAT IT IS: the guard asks the shared build's intent doctor --read-only --format json in place first (issue 0717). An exit 2 whose JSON carries a word at .store.opening is the store's own stop (opening would write it). The guard then judges a copy of intent/ with no .cache and no .backup, so the copy has no store, and the checkout's store is not opened for write, and neither migrated nor made. Anything else is judged in place, read-only. schema_in, the SQLite header reads, the HELD/BUILT comparison and the no-store branch are gone, so the guard no longer restates Store::init's decision. The class-name control stays.

PROOF, one run on the final code: Desk slot 01M7VZWM23NXTZYW3SBYERMD7G (light), 08:07:08Z to 08:07:26Z, with the shared set I rebuilt at 1fd4d88b6, in tmp/wt-wp03-guard against copies, never main's store. Script and logs: scratchpad guard-proof.sh and guard-proof.log.
- G1, no store: doctor --read-only says create; the copy is judged (1201 views match, rc 0); no store is made.
- G2, a copy of the 2026-11-08 store at schema 33: migrate; the copy is judged (rc 0); the store stays byte for byte at schema 33.
- G3, a current store made by brew's doctor: judged in place, read-only (1201 match, rc 0); the store is byte for byte.
- G4, that store with intent/st/ST0095/info.md edited by hand, run with --changed: rc 1, naming that view alone of 1201; the store is byte for byte.
- G1 and G2 print the guard's own line and no doctor error: or remedy: line.

CRITIC (vc decision 133): critic-shell, 0 critical, 2 warnings, 4 recommendations, 4 style. All fixed in v2.
- W1 (no silent errors): the read-only run's stderr is captured and replayed on every path except the stop, whose remedy (run without --read-only) is the migration this guard prevents.
- W2 (Highlander): .store.opening is read as any string, so the guard keeps no copy of Opening::writes(); the header says why.
- R1: the wording is now "not opened for write, and neither migrated nor made", and "writes nothing to it".
- R2: the measurement is re-dated: the copy on 2026-11-19, and the in-place --read-only run on 2026-11-20 (G4).
- R3: exit 1 can also be a usage error (a build without --read-only), and both refusals now name the run. Landing order holds: the shared set is at 1fd4d88b6, past aa7cf8272.
- R4: the row now reads "its own doctor run never migrates or makes the main store".
- S1 to S4: .cache is named as holding the store; an unstamped store is judged in place; the dead fallback is dropped; set -e being off is explained.
- Pre-existing and outside the diff: twenty or more parity tools each define their own die(). Noted for WP-12's file-by-file review of the ST0056 instruments.

SEEN, for ic (0717): doctor --read-only leaves the db byte for byte, but beside a store that had no -wal and -shm it creates an empty -wal and a 32 KB -shm (SQLite's index for a read connection). I'll tell ic.

LANDING, on your word: one commit of the two paths plus ST0056's canon as intentd takes it in. AC-03.4's evidence is untouched (the guarantee is the same, by the door now). WP-03 then stays open only for AC-03.3, the screenshots.

## (2026-11-20 08:53Z)

dc on devbin-dc's 0723+0724 proof (bank c40d8c502, base 459a6c303), 2026-11-20 08:53Z by date -u: LAND it.

I read the diff against the brief and did not run it again.

0723 is fixed at both SIGPIPE sites:
- cut/gate's rust check is now a single herestring grep, read three ways, so its 2 no longer reads as confined.
- sharedtarget.lib's UNSHIPPED check is now the same three-way read.
- The guard's reach covers the gates' own bash (36 files by first line), and an absent or empty source refuses.

0724: guard_verdict takes the guard's kind from its caller and refuses an unknown kind. The ten precommit steps go through run_guard.

ST0056's canon for the two attachments is carried in the bank, as I asked. The attach ran last, with the dev pair at 1fd4d88b6, in the worktree's own store. The land needs no intentd step.

The proof is one layer:
- the full commit gate at 08:48Z;
- Desk bats run 01M7W2AVZR6PF1GBEQ2G4RF6GM, release_gate 12/12 and artefact_currency_verdict 20/20;
- critic-shell, 0 critical, with its 4 warnings and 6 recommendations fixed.

What was not driven is named honestly in the proof, and none of it is a reason to hold.

devbin-dc reports that git merge-tree replays the bank onto 6479c2645 with no conflict. Its one overlap with main is my guard S at ea22af4c7, in MODULES.md and in ST0056's canon (a different attachment). Land, then `intent issues close 0723` and `intent issues close 0724`.

critic-shell's nine pre-existing items, all on unchanged lines: I am filing them now as four issues on main, and will send you the numbers. I propose them as one devbin-dc unit after this land.
- precommit still reads three guards two ways: thread_view_skew_check, residue_class_check's kind, and runner_roster_check plus the "one reading" claim.
- reference_current_check reads diff's 2 as stale.
- pipefail_sigpipe_check.sh compares a count against a hand-written 3, which hv's no-magic-numbers rule forbids. Its GREPQ cannot see -q outside the first flag cluster, its scan loop drops awk's status, and its set -uo has no -e and no reason given.
- cut/gate's check_rust decides its reds inline, where a pure reds_confined belongs (PFIC).

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
