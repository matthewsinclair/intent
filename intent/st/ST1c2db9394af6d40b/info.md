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

Also carried in: cc's d86c reading of ST0096's catch-up item (5), judged out to this thread by vc on 2026-11-22. It is in refs/bank/cc/d86c/v1-notes, section "(5), judged out to ST1c2db93". In short:

- (a) organize::gate already asks views::rendered_by_an_older_intent (0539). The upgrade's gate_in_any_format adds a comparison against a render in the EARLIER canon format (earlier_renders, 0746), which neither doctor's skew nor the write guard holds. One home means the predicate takes the render pair rather than a format, threaded through views::skew and the write guard: a design change across three doors, over M.
- (b) RenderContext.canon_format comes from the store (Facade::canon_format) at every acting door, while doctor reads the disk (intentfiles::reported_format, with a V3 fallback) on purpose, and reports a store-vs-disk mismatch as canon_format_behind. One home needs a ruling on which is the truth for a reporter, store or disk.

Also carried in from cc's board (todo 56, archived at the localfold after 3.5.1, with ST0095's packages all done): the ST0095 WP-04 critic answers that were owed by later packages. Read each against today's code before planning it, because d86 (bfe387912) answered part of C1 and R1. WP-07's C1 (the address segments read once by the resolver), W2 (the facade's messages take shown() via a typed ThreadId), W4 (the composite reader and scope_of's fallback, 'no steel thread ST0001/100'), AcState::Descoped.to typed as ThreadId with v2 migration's refusal, and R7 (the architecture call on Deref and typed ids). WP-08's W12 (section and listing order by origin). WP-09's W9 (the sixteen thread/label subjects) and R1 (the label kind). ic's WP-10 held W1, W3 and R2.

Triage, 2026-11-23 (cc, read against a834dc637): 33 pieces and 24 concerns judged resolved or not real, banked as the blob refs/bank/cc/st1c2/triage. Units for 3.5.2: U1, the id grammar (P1, P2, P3, P4, P5, P8, P9, P10, the render half of P12, P14, P15); U2, realisation, kinds and paths (P6, P7, P11, P16, P17, P18, P20, P21); U3, tests and prose (P13, P25, P26, P27, P28a, P28b, P29). P31 moved to dc's ST0097 unit B with the other ST0057 retirements.

Deferred to the next release, on hv's word relayed by intent-vc on 2026-11-23: P22, one predicate for "rendered by an older Intent" across organize's gate, doctor's skew and the write guard (sized L, and it changes doctor and the upgrade gate); and P30, the canon-ignore guard naming a git-style canon file (hv: no hook change ships to attendees).

Ruled by intent-vc under hv's pen, 2026-11-23, each as cc recommended: P23, doctor keeps reading the disk for the canon format; P24, Deref stays on the typed ids; P2, subjects take the slash form; P10, read_named and scoped refuse an address; P12, legacy's v2 label check stays head-only; P19, the find_*_mut twins are accepted; P31, the ST0057 .intentfiles tools are retired (hv ruled the same for ST0057 AT-04.6); P32, census reach is ST0097's; P33, the ST0056 path literal in the gate is accepted.

## Work Packages

Each work package is a record of its own: `intent wp list ST1c2db93` lists them, and `intent wp show ST1c2db93/<id>` shows one. A realised package's cover is `WP/<id>/info.md` beside this file. This cover carries the thread's own fields only.

## Acceptance

Acceptance criteria and tests are records of their own: `intent ac list ST1c2db93` and `intent at list ST1c2db93` list them, and `intent ac gate ST1c2db93` says whether the contract passes. `acceptance.md` beside this file is a GENERATED VIEW of them, and a row authored there is discarded by the next sync. The verbs write the contract: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
