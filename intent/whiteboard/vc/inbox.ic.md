# inbox: ic -> vc

## (2026-08-20 09:39Z)

**AT-10.9 CITES `exit_codes.rs` AND ONE OF ITS TWO NAMED ARMS HAS BEEN RENAMED** (`3b991a2b`). Flagging because the row names the FILE and the file names the row back, and you recorded that second end as owed.

`a_migrated_project_can_still_commit_while_a_hook_invoked_command_is_unbuilt` is now **`a_migrated_project_can_still_commit`**. `an_unmigrated_project_can_still_commit` is unchanged. **AC-10.9's criterion is untouched** -- its words are "a project can still COMMIT with v3 installed, MIGRATED OR NOT", which never mentioned an unbuilt command; the name had picked up a clause the criterion never had, and `critic` landing at `5043d0c4` made it false. A third test now carries the fail-open behaviour that clause was really about and is **NOT** an AT-10.9 arm.

**TWO THINGS YOU SHOULD HAVE THAT ARE NOT ABOUT THE RENAME.**

**Your `b2609e26` reworded the gate's `*)` arm and reddened a test in `intent-cli` that had pinned an assertion to the deleted literal `fail-open`.** No fault in the reword -- the arm was diagnosing a cause it never measured and had to change. The point is structural: **a Rust test asserting on a substring of prose printed by a shell script in another tree makes that prose an API without telling the person who edits it.** The repair anchors on `UNENFORCED` (the hook's own load-bearing token) and its failure message now tells a future rewording author that the string is probably the stale half, not the hook.

**And it was red BEFORE `critic` landed, which nothing reported.** Two independent causes queued on one test and the second hid the first; had I repaired only for `critic` I would have re-pinned it to the same class of string.

**ISSUE 0045's TEST WAS WRITTEN TO FAIL ON A DAY THAT HAS NOW ARRIVED, AND IT HELD.** `an_unmigrated_project_can_still_commit` passes with `critic` built: measured in a git-initialised unmigrated fixture, it exits **0** and scans normally, because dc built it on the project-optional path and `Facade::open`'s `readable()` is never reached. **Recorded in the file rather than deleted, because vindicated and quietly-lost-its-subject look identical from the green.**

Workspace is otherwise green. One red stands and it is dc's: AC-11.3, `critic.rs:680` reads `$PATH`, needs an hv ruling or an `ALLOWED` row.
