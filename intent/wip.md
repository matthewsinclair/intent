---
verblock: "23 Nov 2026:v4.11: vc - localfold on hv's second call; 3.5.2 waits on U4 and devbin 0.1.29"
intent_version: 3.5.1
---

# Work In Progress

## DOING

- **Intent 3.5.2 (hv decisions 87 and 88, 2026-11-23: fix everything, release asap).** Main is pushed to local and upstream at e1e84abb8, after build all rc 0 at that HEAD. Every 3.5.2 bank is landed except cc's ST1c2db93 U4, the v2 removal, which comes as three banks: U4a (survey P1 to P5, whose Desk rerun is queued), U4b (AT rows to n-a, the v2 parity tools, dispatch rows and aliases, docs) and the metadata strip, last. U4's proof upgrades v3.4.0, v3.5.0, v3.5.1 and a 3.5.1 fleet repo to doctor 0, and its bank carries its CHANGELOG entry. After U4: build all and push; dc's releasing.md step 2(b) (reference against v3.5.1, known-defects re-driven and retitled, explorer shots); the kovacs rehearsal through the Desk; the whole local gate with clippy at the tip; then vc's READY to hv, who fires the cut. READY also waits on devbin 0.1.29 (I514ea74: the formula's brew style offense on brew 7.0.15), which needs hv's cut of devbin and hv's brew upgrade, then devbin-vc's sweep into Intent. AE's attendees take 3.5.2 from the tap, so a repo upgraded at 3.5.1 must get 0 doctor findings with no action, and no hook may change. vc's board doing 67 has the commits.

## TODO

- hv: whether a brew intentd should run on kovacs at all. One has run there since 2026-11-16; issue 0703 made the shell suite independent of it.
- hv: devbin's decision 150 lists Intent among issue 0326's adopters, where hv decision 36 keeps `intent use` and `intent install` as Intent's own.
- ST0089 stays parked (hv decision 59) until hv rules on its policy.
- Parked for hv since 2026-11-10: threads ST0060, ST0077, ST0082, ST0084 and ST0085; Laksa's and Conflab's releases; the deferred list. Their text is `git show f16dce4a7:intent/wip.md`.
