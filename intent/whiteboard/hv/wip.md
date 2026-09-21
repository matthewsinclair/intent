---
node: hv
name: Hypervisor
role: hypervisor
session_id: none
heartbeat_at: 2026-09-13 10:24Z
status: active
focus: "Board fixed for the store model on hv's word, 2026-09-13 10:21Z, by vc's hand: five sections, each line a ruling with its date. The 3.0.1 record, the ten cull decisions and the full dated directive record are verbatim at .history/20260913/wip-precutover-1018Z.md; earlier archives under .history/20260911/ and .history/20260828/."
claims: []
---

# Hypervisor (hv)

## DOING

_(none)_

## TODO

_(none)_

## Holds

_(none)_

## Standing directives

_(none)_

## Watch-outs

- **Reading the write-up of a class is not protection from it** (2026-08-22, ic's synthesis, corroborated by all four nodes): every save that day came from a mechanism, none from a rule; a rule is at its least protective in the sentence that asserts it (dc, 2026-08-23); the correct scope of a rule is not visible from the incident that produced it, so a rule is provisional until something on the other side of its boundary has tested it. The full argument is in the snapshot.
- **The `sync` skip ruling against AC-03.6** (2026-08-23, deliberately not ruled by vc): hv chose among four options dc authored and the four survive nowhere, so only hv can say what was chosen. There is no attachment prune at all and canon-side edits cannot supply one, because attachments live in the store.
- **This board records what was ruled and never what was done** (2026-08-22): before briefing anyone off a directive here, drive whether it is already built.
- **hv's rulings reach this board second-hand, dated to the day, and a same-day reversal is invisible** (2026-08-21): either hv's word carries a time, or a reversal is announced.
- **A benefit reclassified as a side effect escapes the ruling that governs it** (2026-08-21, vc's own, struck at source).
- **`intent sync --to-disk <ID>` is a whole-file second writer over every attachment of that thread** (dc, 2026-08-21): announce the files, not the thread.
- **A directive not reachable from the site that needs it is an open question in practice.** This board was a stub for eight weeks while peers were told to read it.
- **Attribute hv's live stamps, never assert them.** The live channel is unguarded; a quoted live stamp launders into the committed record past a guard posted at the wrong door.
- **Known and shipping as-is unless ruled otherwise** (2026-09-11): `a_daemon_outlives_nobody.rs` flakes roughly 1 in 20 in parallel, mechanism unconfirmed; a SIGTERM'd daemon does not checkpoint its WAL, and a 559 MB WAL was truncated by hand on 2026-09-10.

## Decisions

- (2026-08-21) This board's writer is `vc`, on hv's ruling; every entry carries the date hv said it and a pointer to where, and when hv selects among options vc authored the authority is hv's and the wording is vc's.
- (2026-09-01 08:32Z, hv first-hand) Completeness beats schedule, without a cost ceiling: there is no tag window and no external consumer, so the scarcity register is retired as a class and a recommendation resting on "there is not time" is re-derived on its merits or withdrawn. The 3.0.1 number this closed has since shipped; ST0069 goes into 3.0.2 on hv's ruling of 2026-09-12, and it is not cut until every non-cancelled WP is Done.
- (2026-08-26 12:20Z, hv first-hand) hv fires a cut in hv's own terminal: the release confirmation reads `/dev/tty`, this estate never passes `--no-confirm` from a tool session, and the push gate stays human. The act of releasing is hv's hand; "get the release done" is not "release it" (2026-08-25).
- (2026-08-22, hv first-hand to all four nodes) vc holds the pen: a ruling vc makes under it is a vc ruling under a delegated pen, never an hv ruling, written `ratified_in: "vc, <date>, under hv's pen granted 2026-08-22; <record>"`, and never enters the HV bucket. The pen moved the authority, not the aim.
- (2026-08-19, restated 2026-08-21) The hv inbox's reader is vc: a write is the durable half and never the delivery; hv needs context, a question and options in the live channel.
- (2026-08-21) Open items go back to the workstreams; there is no hv gate on them.
- (2026-08-21) A false claim in a landed commit is corrected forward, never rewritten.
- (2026-08-21, hv verbatim) The word `intentdb` is retired corpus-wide; the SQLite db is the durable SSOT and the crates are `intent-cli`, `intentd`, `intentsvcs`.
- (2026-08-21, provenance unverified, advisory until hv confirms) Every node prunes its own `target/<node>` at fold.
- (2026-08-21, state verified, authorship not) `upstream` is pushable; the freeze lift of 2026-08-20 is standing.
- (2026-08-21, cc's sharpening) Announce a write to a shared file to everyone and a write to a claim to the claim-holder; canon is the first.
- (2026-08-29 16:30Z, vc, a docs call) Prose pages use em dashes; generated reference pages use `--`.
- (2026-09-13) **ONE STAMP RULE FOR EVERY DEV-BUILT ARTEFACT: intent, intentd and Intent.app carry the repository HEAD current when they are built, so a manual check is one equality; the brew-installed release carries the tag's hash.** Reverses the 2026-08-26 scoped identity in build-support/source_commit.rs with its cost taken: a commit landing mid-build marks a correct pair behind, and byte-identical Rust at two HEADs carries two stamps. dirty- stays scoped to the Rust tree. cc lands it; vc rebuilds the set after.
- (2026-09-15, corrects decision 8 forward) The install root is XDG since ST0074 WP-05: the home pointer is ~/.local/share/intent/home, and ~/.intent/home no longer exists. Brew stays installed and unlinked, and is now pinned; the dev tree remains the delivery.
- (2026-09-16, hv first-hand in prose, answering vc) Three rulings. (1) devbin's whiteboard-store defects (devbin 1824c75, letters A to O, no K, sent through vc on hv's routing) are FIXED IN THIS RELEASE, as soon as possible: hv, verbatim, "We can't ship this kind of defect so it needs fixing asap." They join the close-out; vc files one issue per letter and routes them. (2) Go on the landing queue vc judged green: cc's 0331 (b) as train 8, then dc's devbin twin, then dc's Decision A record, each on vc's word with the HEAD hash. (3) The pickup gap is fixed: wb pickup renders peers as headers only, so no node sees another board's watch-outs, decisions or standing directives at boot (found by dc, 2026-09-16).
- (2026-09-17, hv first-hand in prose, taking vc's four recommendations; vc's clock read 09:39Z just after) Four rulings. (1) The rebuild runs now, before the go, and again after ST0076 WP-04 lands; hv ran it, intentd restarted at 12:00Z, and vc verified the pair at c800b8319 with the store at schema 28. (2) AC-00.1's search reads `intent search --subkind method --in AddressError` and WP-04 builds the search that runs on filters alone, with AC-04.1 carrying the clause; vc's split of --subkind from --kind stands. (3) The pickup session-id fix joins the close-out, as issue 0433. (4) The go is given as vc reviewed the three lanes' plans, with one whole-suite judging run per train in the worktree that already holds its stack, vc judging from the logs, the diff and a read-only check that the worktree equals the bank.
- (2026-09-17 18:58Z, hv first-hand in prose, answering vc's escalation on the SQLite bump) DEPENDENCY BUMPS ON THE RUST SIDE ARE NOT GATED. hv, verbatim: "Well, if it needs a bump (or if anything in the rust side of the house needs a bump, then bump it). That should never be a problem to move the deps forward. Just do it." So `libsqlite3-sys` moves off 0.30.1 where 0442 needs it, and the same goes for any other Rust dependency a lane finds it needs; no node asks again for a dep bump as such. WHAT THIS DOES NOT CHANGE, vc's reading of the boundary: a bump is still a change that banks, is judged, takes a whole-suite run and a rebuild like any other, and it is still under NO RELEASE and NO PUSH -- "just do it" removes the permission question, not the train.
- (2026-09-17 19:01Z, hv first-hand in prose, taking all five of vc's recommendations at once: "Ok, do all of those.") FIVE RULINGS, wording vc's, authority hv's. (1) ST0076 CLOSES NOW on its gate, 14/14 with every work package Done; dc's AC-06.2 re-drive on the installed pair attaches as evidence to a closed thread rather than gating it, because holding a passing thread for a confirmation already ruled optional is how an option becomes a condition. (2) THE TRAWL: Part A is GO -- Riffle cc, Molt hv, Prolix hv and vc, arca_cli cc, Courses hv -- and Part B takes vc's disposition: drop all six classes, every dropped unit kept in its node's pre-migration copies, the 16 units that read as asks of hv plus Conflab cc's 12-row decision table returned as 12 holds on the asking node's own board, and Gtools' hv board registered. One mechanical commit per estate in its own repo, doctor after each, vc checks each. NO PUSH. (3) 0426 IS SATISFIED BY dc's REHEARSAL, which drove the mechanism to both verdicts; the LIVE guards adoption pass is a separate item and stays blocked behind the devbin `int hooks` defect, which nobody works around. (4) devbin#0083 AND dc's `int hooks` FINDING are routed by hv to the Devbin estate and NEITHER GATES THIS RELEASE; vc fixing them from here stays available as a named cross-estate exception if hv would rather they were simply done. (5) THE VERSION IS 3.1.0, a minor: ST0076 adds a verb and an index level that did not exist, which is new surface rather than a repair, and the fixes ride along. cc writes the CHANGELOG section and docs/releases/3.1.0/RELEASE_NOTES.md at that number instead of a placeholder.
- (2026-09-17 21:16Z, hv first-hand, answering vc's three questions in order: "1: Both in / 2: Yes / 3: Tag") THREE RULINGS, plus one instruction for after the bounce. (1) **THE `libsqlite3-sys` BUMP AND THE NEW DETECTOR ARE BOTH IN 3.1.0.** So `0442` closes on a repair, a detector and an honest unreproduced cause rather than on a known-defects line: the detector is a document present in the index with no row in the content table, plus fts5's own per-table check, against a rebuild of the same database, and it cannot be built on 3.46.0's checker, whose secure-delete allowance is one hard-coded leaf shape. One rebuild and one whole-suite run are owed with it. (2) **`docs/known-defects.md` IS RE-DRIVEN AGAINST 3.1.0 AT THE CUT** -- every entry driven against a fresh build the way the page was originally made, not a framing sentence over v3.0.1's driven record. That retitles the page and settles the nine entries quoting output that no longer renders, because a re-drive replaces a transcript rather than editing it. (3) **THE `docs/reference` BASELINE IS THE PREVIOUS RELEASE TAG**, `git describe --abbrev=0`, ruled with the concrete case in view: under this rule the v3.0.3 cut measured against v3.0.2, which was tagged and whose artefacts were never published. It does not bite the 3.1.0 cut, whose previous tag is v3.0.3. ic writes the sentence into the open section of `intent/docs/releasing.md` and the staleness check is unblocked. **AND THE INSTRUCTION, FOR AFTER THE BOUNCE**: hv, verbatim -- "I recommend seriously and aggressively leaning out wip.md and restart.md back to bare bones basics. But do that post bounce."

---

_Generated by Intent v3.2.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
