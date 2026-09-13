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

- **(2026-08-26 20:38Z, held for hv) The staging-dir build dangles the delivered pair for the length of a rebuild**, and consumers refuse in that window. Condition: hv rules on dc's item (2), build private then move into place, and on whether gates refuse while the pair dangles.
- **(2026-08-27 17:11Z, held for hv) dc's `bin/int` to `bin/devbin` rename goes after the carrier sweep.** Condition: hv orders the rename; both spellings exist today.
- **(2026-08-26 20:49Z, held for hv) Lamplight's 3,738 stranded bucket files are not carried per file by `st attach`.** Condition: hv chooses between the WP-02 bulk ingest now landed on Intent, an overnight per-file run, or the buckets staying as history.
- **(2026-08-26 20:35Z, held for hv) Lamplight triage's store half is blocked by 22 non-UTF-8 files refusing `sync --to-store` estate-wide.** Condition: hv moves or renames them, rules they leave the tree, or takes the ingest fix.

## Watch-outs

- **Reading the write-up of a class is not protection from it** (2026-08-22, ic's synthesis, corroborated by all four nodes): every save that day came from a mechanism, none from a rule; a rule is at its least protective in the sentence that asserts it (dc, 2026-08-23); the correct scope of a rule is not visible from the incident that produced it, so a rule is provisional until something on the other side of its boundary has tested it. The full argument is in the snapshot.
- **A catch-all commit sweeps a live five-node checkout** (2026-08-22, open for hv): `6c603b21` carried four nodes' in-flight work; the index and canon are shared objects even when the commit is not. Every commit here names its paths, hv's included.
- **The `sync` skip ruling against AC-03.6** (2026-08-23, deliberately not ruled by vc): hv chose among four options dc authored and the four survive nowhere, so only hv can say what was chosen. There is no attachment prune at all and canon-side edits cannot supply one, because attachments live in the store.
- **This board records what was ruled and never what was done** (2026-08-22): before briefing anyone off a directive here, drive whether it is already built.
- **hv's rulings reach this board second-hand, dated to the day, and a same-day reversal is invisible** (2026-08-21): either hv's word carries a time, or a reversal is announced.
- **A benefit reclassified as a side effect escapes the ruling that governs it** (2026-08-21, vc's own, struck at source).
- **A false confession passes every check, because it costs the claimant** (2026-08-21): `--only` preserves a peer's stage; a bare `git reset` is a shared-index operation in a multi-node checkout.
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
- (2026-08-27 16:54Z, hv first-hand) Delivery while in development is the dev tree: brew stays installed and unlinked, the install root is `~/.intent/`, and making brew the delivery mechanism again is hv's to choose.
- (2026-08-21, hv verbatim) The word `intentdb` is retired corpus-wide; the SQLite db is the durable SSOT and the crates are `intent-cli`, `intentd`, `intentsvcs`.
- (2026-08-21, provenance unverified, advisory until hv confirms) Every node prunes its own `target/<node>` at fold.
- (2026-08-21, state verified, authorship not) `upstream` is pushable; the freeze lift of 2026-08-20 is standing.
- (2026-08-21, cc's sharpening) Announce a write to a shared file to everyone and a write to a claim to the claim-holder; canon is the first.
- (2026-08-29 16:30Z, vc, a docs call) Prose pages use em dashes; generated reference pages use `--`.
- (2026-08-25 21:33Z, hv first-hand) The `claude ws` family survived the 3.0.0 cut with an expiry enforced by AC-14.12; that expiry is discharged at the ST0069 WP-14 cutover of 2026-09-13.
- (2026-09-13) **ONE STAMP RULE FOR EVERY DEV-BUILT ARTEFACT: intent, intentd and Intent.app carry the repository HEAD current when they are built, so a manual check is one equality; the brew-installed release carries the tag's hash.** Reverses the 2026-08-26 scoped identity in build-support/source_commit.rs with its cost taken: a commit landing mid-build marks a correct pair behind, and byte-identical Rust at two HEADs carries two stamps. dirty- stays scoped to the Rust tree. cc lands it; vc rebuilds the set after.

---

_Generated by Intent v3.0.1 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
