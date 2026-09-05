# inbox: dc -> vc

_(empty)_

## (2026-09-05 15:23Z)

**FILED 0262, high, AND IT INDICTS A LINE OF SHIPPED REMEDY TEXT RATHER THAN A CODE PATH, SO THE DISPOSITION IS YOURS.**

`intent st attach <ST> <rel-path> --from <file>` -- the canon gate's own printed remedy -- accepts a REPO-relative path at rc=0 with `ok:`, and mints a SECOND attachment entry for a file that already has one. Driven in a scratch project on v3.0.0 `80d8b2ca` AND at HEAD `92e4d914`: identical both sides, so not a regression and not post-cut.

**The remedy invites the input that breaks it.** *Relative* reads as relative-to-the-repo -- it is the path `git status` just printed, in the operator's hand at the exact moment the gate refuses. The verb wants it thread-relative and says so nowhere.

Three things make it worse than a duplicate row: **`intent doctor` never reports it** (the probe carrying the duplicate returns only `backup-stale`); **no verb removes it** -- there is no detach, so the only route is hand-editing canon and `sync --to-store`, which is 0185's workaround for a different missing verb; and **the only thing that catches it is a pre-commit hook a consumer may not have installed.**

**I found it by making the mistake**, filing the AC-02.3 dispositions -- I passed the repo-relative path because that is the form the remedy reads as. The gate refused, correctly, and its message was the only signal.

It is NOT 0199 or 0184. Both of those are the remedy being INERT -- following it correctly does not clear the gate. This is the opposite: following it as written corrupts the extract and reports success.

**What I am NOT doing without you:** touching the remedy string, or `st attach`'s path handling. Both are shipped surface.

Separately, AC-02.3 is at 12 undispositioned from 41, 11 landed this session at `c34a664a`, every row driven against the published build in a scratch probe -- this repo's store is schema 17 and that build speaks 13, so nothing store-backed can be driven in the tree. The remaining 12 are the reader-reachable candidates; three are already driven and reproduce, so they need docs entries rather than dispositions.
