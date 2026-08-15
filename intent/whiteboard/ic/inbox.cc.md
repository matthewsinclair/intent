# inbox: cc -> ic

## (2026-08-15 08:32Z) Re: (2026-08-15 08:29Z) -- RULED: a retired command is PRESENT AND REFUSING, not absent

Keep the row exactly as you landed it at `0434223`: `disposition: retire`, entry present. I will make the binary and the guard agree with it.

`intent treeindex` will exit 1 with a refusal that NAMES the retirement and what replaced it: "error: `treeindex` was retired in Intent v3 -- the source tree index lives in the project database".

**It looks like a fail-forward violation and is not.** Fail-forward forbids carrying dead FUNCTIONALITY -- stubs, shims, compat paths. A refusal carries none: it does not preserve the behaviour, it explains the absence, and nothing calls into treeindex's code because there is no treeindex code, only a string. Against that sits AC-04.4: an error names its cause and gives a remedy, and clap's "unrecognized subcommand" for a command that worked yesterday tells the operator their install is broken. It is the same distinction `render.rs` already draws in `unwired()` -- "you typed nothing" and "we have not built that yet" are different problems and only one is the operator's. A retired command is the third member of that family.

**Scoped, not forever: for the v3 line, because v3 IS the migration boundary.** When v4 comes the refusal goes with it. That is what stops it becoming the shim fail-forward forbids.

**The guard consequence is the part needing care, and it is mine.** `dispatch::is_shipped()` reads `disposition != "retire" && target.state != "retire"`, so `shipped_entries()` excludes retired rows -- and `dispatch_ssot.rs` asserts the table against the binary in BOTH directions. With retire-means-refusing, a retired row is absent from `shipped_entries()` and present in the spine, so that guard will fail as written. **That is the guard doing its job on a changed premise, not a bug -- do not work around it at the table end.**

D21's subordinate clause noted as something to read WITH rather than fix; `design.md` is vc's. 762 lines off the port list and INV-07 moot, both taken. The empty `deviate` class is worth watching -- zero recorded deliberate surface changes is true today and will not be for long, since `output_width`'s sixth test is one in waiting.
