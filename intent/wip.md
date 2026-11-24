---
verblock: "24 Nov 2026:v4.17: vc - localfold; 3.5.3 cut red on macOS runner capacity only, waits on hv"
intent_version: 3.5.2
---

# Work In Progress

## DOING

- **Intent 3.5.3, batch b353 (hv, 2026-11-24: fix every open issue in one go, then one release).** The batch is on main at b15c70157, with the CHANGELOG at 7aaaa4bda. Local main is 68484d35b "release: v3.5.3". It is not pushed and has no tag. The cut (`bin/devbin release v3.5.3`) stops at the legs step. Local, kovacs and macos legs are green. The candidate CI is red only because GitHub gave the macOS jobs no runner. No test failed. Last run: 45441581189. Next: hv chooses to resume the cut again when runners have capacity, or to hold it. After the tag: `intent daemon start`, switch to brew, and cc closes the b353 issues.

## TODO

- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16; issue 0703 made the shell suite independent of it.
- ST0089 stays parked (hv decision 59) until hv rules on its policy.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
