---
verblock: "24 Nov 2026:v4.12: vc - localfold; 3.5.2 waits on cc v3 (CI fix + I75b8995), then re-cut"
intent_version: 3.5.1
---

# Work In Progress

## DOING

- **Intent 3.5.2 (hv decisions 87, 88 and 89).** Every 3.5.2 change but cc's v3 bank is on local main; upstream main is at 3475b6e94. Two cuts stopped red at legs on candidate CI, with nothing tagged: run 45326139850 and run 45328303838 failed on flaky daemon-harness and slots tests (I5207bf7, I14a2284, Ief26a29). The unpushed release commit 029297dee was reset off main and is kept at refs/attic/vc/release-v3.5.2-try1. hv decision 89 holds 3.5.2 for I75b8995 (a CLI can route to another HOME's daemon over TCP 51737): the fix is an identity probe that sends no secret. cc's bank refs/bank/cc/ci352/v3 (188842aa4, on b1ed72e56) carries both fixes; its gate all was running at 06:33Z. Next: land v3, build all at HEAD, push, then `bin/devbin release v3.5.2 --dry-run` with a clean tree and intentd stopped (it is stopped now), then READY to hv, who runs `bin/devbin release v3.5.2`. After the tag: restart intentd from a Claude shell and tell devbin-vc; lift the seats' freeze. Filed and outside 3.5.2: I5adc617 (an HTTP 5 s test red under load), I53c0c22 (Webber's daily backup gaps, from webber-vc).

## TODO

- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16; issue 0703 made the shell suite independent of it.
- hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- ST0089 stays parked (hv decision 59) until hv rules on its policy.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
