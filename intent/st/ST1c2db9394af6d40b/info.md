---
st_id: ST1c2db93
title: Highlander residue after ST0095 and ST0096: one home for every id and realisation concern
status: Triage
created: 2026-11-22
completed:
---

# ST1c2db93: Highlander residue after ST0095 and ST0096: one home for every id and realisation concern

## Objective

Every id concern and every realisation concern in Intent has exactly one home, in every language, so that ST0095's AC-00.1 and ST0096's AC-00.1, descoped here on hv decision 86, are met.

ST0095's and ST0096's close-time Highlander sweep (2026-11-22, at 813037fcd) found the behavioural defects that are now fixed in 3.5.1 (hv decision 86: I88f333f, Ib383ad5, I91dd457, I4fc4495, I78176db, Ic7d939d, If83ae7c, and I9ed9ed4 for main's self-hosting tests). What remains is duplication that behaves the same today: second copies of formatting, paths, label handling, kind spellings, test fixtures and shipped prose. The full list, each row with file and line, is in the banked reports:

- refs/bank/vc/st0095/highlander-close-A-intentsvcs-20261122, -B-rust-rest-20261122, -C-non-rust-20261122 (the close sweep)
- refs/bank/vc/st0095/close-E-start-vs-close-20261122 (every start cluster against today's code: 42 residual, 23 of them missed by the close sweep)
- refs/bank/vc/st0095/close-F1-critic-rust-20261122, close-F2-critic-rust-42de-20261122, close-F3-critic-shell-20261122 (the catch-up critics' findings)

Rulings carried in: a criterion label's group stays any digit width (hv 2026-09-08); legacy.rs's v2 import keeps its head-only label check as the one stated exception until this thread decides whether v2 import reads labels through id::label_kind; read_named and scoped do not yet say "this is an address", which this thread decides.

The thread is planned against those reports and sized when it is triaged. It closes when an independent sweep of the whole tree finds one home for every concern, or answers each remaining one on the record.

## Context

Carried in from cc's d86c reading of ST0096's catch-up item (5), which vc judged out to this thread on 2026-11-22 (refs/bank/cc/d86c/v1-notes, section "(5), judged out to ST1c2db93"): (a) one predicate for "rendered by an older Intent" across organize's gate, doctor's skew and the write guard; (b) which of the store and the disk a reporter takes as the canon format. Carried in from cc's board (todo 56): the ST0095 WP-04 critic answers that later packages owed.

Triage, 2026-11-23 (cc, against a834dc637): 33 pieces, P1 to P33, and 24 concerns judged resolved or not real, banked as the blob refs/bank/cc/st1c2/triage.

Ruled by intent-vc under hv's pen, 2026-11-23, each as cc recommended: P2, subjects take the slash form; P10, read_named and scoped refuse an address; P12, legacy's v2 label check stays head-only (moot since U4a removed legacy.rs); P19, the find_*_mut twins are accepted; P23, doctor keeps reading the disk for the canon format; P24, Deref stays on the typed ids; P31, the ST0057 .intentfiles tools are retired; P32, census reach is ST0097's; P33, the ST0056 path literal in the gate is accepted.

Landed. In 3.5.1, the behavioural defects of the close sweep (hv decision 86). In 3.5.2: U3 (00ddbb494), U1 (8ddb863d1), U2 (d1928ba2a, with I086e953), and U4, the removal of every v2-only path (e1831ddf9, 2f864f6f1 and 75d209050); P31 went with ST0097's units A and B (8e041900d). In 3.5.3: P22 and P30 (b15c70157), which hv moved to that release on 2026-11-23.

Phase 0 for 3.5.4 (hv decision 99, "Go all, thread in", 2026-11-25): cc, ic and dc each checked the rows of the seven reports in their remit at bd1aa3c6c, read only. design.md records each row's disposition and the residue. Each residual concern is one package of this thread, with its seat and size in its body. vc decision 134 adds the retirement of the v2 bats-estate inventory. The packages wait for hv's review.

Run plan (vc, 2026-11-25): each builder works in its own worktree and makes one commit for each package. Before it banks, it runs clippy and critic-<lang> on its diff. vc lands each bank after reading it against its package. One gate all and one test shell at main's HEAD, after the last bank lands, decide the batch, and that is also the cut's gate on the way to the cut.

## Work Packages

Each work package is a record of its own: `intent wp list ST1c2db93` lists them, and `intent wp show ST1c2db93/<id>` shows one. A realised package's cover is `WP/<id>/info.md` beside this file. This cover carries the thread's own fields only.

## Acceptance

Acceptance criteria and tests are records of their own: `intent ac list ST1c2db93` and `intent at list ST1c2db93` list them, and `intent ac gate ST1c2db93` says whether the contract passes. `acceptance.md` beside this file is a GENERATED VIEW of them, and a row authored there is discarded by the next sync. The verbs write the contract: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
