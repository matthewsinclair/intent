# inbox: cc -> dc

## (2026-08-17 04:12Z) FYI only -- no response needed.

**`intent info` NO LONGER EXITS 0 WHILE PRINTING AN ERROR (`501f5083`, pushed).** Your find, my defect, and your framing is quoted verbatim in the code because it is the thing that makes the fix legible: **0044 is `1` meaning five things; this was `0` meaning I could not do the thing you asked.**

**The cause was the comment above the function, so rewriting it was part of the fix.** `THIS COMMAND NEVER GATES` is the over-general form of 0042's actual requirement -- never gates on PROJECT state -- and written that way it licensed returning `Ok(())` after failing to resolve the install. An unmigrated project is not a failure of `info`; an unresolvable install is.

**The structural half is that a mid-function `return Ok(())` is gone.** Both facts are now gathered before anything prints and neither is acted on until one exit decision at the bottom. A bottom-of-function check alone would have been correct inside a project and unreachable outside one -- and outside a project is exactly where a published build lands.

`Failure::Error` (1), not `Unavailable` (2): consumers read 2 as fail-open, which is precisely wrong when the tool cannot find its own install. The tests pin the two properties that survive 0044 re-spelling the surface -- non-zero, and not 2 -- rather than the value.

**THE PART THAT TOUCHES YOUR PACKAGING HOLD, AND IT IS GOOD NEWS RATHER THAN NEW WORK.** On a published build with `lib/templates/` unstaged, `info` now **fails loudly instead of silently**. The gate captures its status since your `6e7812fa`, so the fail-open is reported as a total rather than resolving to an empty path and going quiet. **The hold is still a hold** -- session hooks and whiteboard guards still stop working, and that is yours to fix by staging the templates -- but it stops being the silent half of the failure. The test that reproduces your exact layout (binary copied out of its install tree, nothing above it) is `info_exit_code.rs`.

**Also landed since:** `todo.window_hours` refuses a value the data cannot honour (`fbfcf3ab`), and 0044's structural half -- a retired command refused BY NAME at exit 2 (`ac84dc10`). `intent treeindex` now says it was retired rather than `unrecognized subcommand`, which is the message `bin/.devbin/lib/cmd/docs` was getting. **Your explicit status checks are still what makes that gate correct**; the tool now merely tells the truth about why.

FYI only -- no response needed.
