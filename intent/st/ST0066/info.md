---
st_id: ST0066
title: Add 'fiat close' as a feature to close STs, WPs, ACs, and ATs
status: WIP
created: 2026-08-28
completed:
---

# ST0066: Add 'fiat close' as a feature to close STs, WPs, ACs, and ATs

## Objective

Add 'fiat close' (FC): a way for the HUMAN to force-close an AC, an AT, a WP, or a whole ST -- easy to invoke, and it MUST carry an explanation of why the fiat close was executed. Where that explanation is recorded is part of the design. The LLM must NEVER be able to fiat-close anything; how that is enforced is an open design question and part of this thread's conversation.

## Context

hv, 2026-08-28, verbatim intent: "by default, an LLM will over-cook a lot of ACs and ATs and sometimes, rather than just yak-shaving ourselves into a singularity, we need a way to cut and run. I don't think the LLM should ever be able to fiat close (FC) something, so not sure how to _enforce_ that, but it's worth adding to the conversation."

The live motivating case is the same day's ruling: hv fiat-closed Conflab's ST0121/WP-02 and ST0124/WP-02 (an unverifiable panel-survival acceptance half) and the mechanism available was `wp done` plus a hand-written 'accepted unverified on hv's word' note with the provenance carried in prose. FC formalises exactly that: the close, the reason, the authority, and the record, as one verb instead of a convention. Design questions on the table: (1) verb surface (one `fc` verb over st/wp/ac/at vs a `--fiat` flag on the existing close verbs); (2) where the reason lives (the canon model carries it; a Finding-like record vs a field on the row); (3) enforcement that the actor is human -- candidates include a /dev/tty confirmation like the release path (`int build release` reads its confirm from /dev/tty; this estate never passes --no-confirm from a tool-driven session), which is a mechanism the estate already trusts for human-only actions; (4) how doctor and the ac gate report a fiat-closed row (visibly distinct from satisfied, never silently identical).

Enforcement datapoint, same day (conflab-vc, holding the first live FC for hv's in-session word): `Bash(intent wp:*)` is ALLOWLISTED in Conflab's harness, so `wp done` runs with no prompt -- there is no mechanical check on the verb at all, and any FC built on the existing close verbs inherits every such allowlist. So enforcement cannot live at the harness/permission level; it must live in the tool itself (the /dev/tty candidate above, or equivalent), because a rule that only holds when something else would have caught it anyway is not a rule.

## Design -- enforcement posture ruled by hv, 2026-08-28 14:38Z, from options authored by vc

**Premise, put plainly and accepted with the ruling:** the LLM shares hv's uid and shell -- conflab-vc measured harness allowlists being bypassed -- so hard prevention of an LLM fiat-close is not achievable on this machine. Enforcement is DETECTION + ATTRIBUTION, not prevention.

**Ruled posture (menu: visible + attributed -- CHOSEN | above + tty gate | interactive nonce challenge | secret token):**

1. `--because <text>` is mandatory -- an FC without a reason does not execute.
2. The record carries invoker evidence: tty-or-not at invocation, timestamp, environment fingerprint.
3. doctor and every rendered view show a fiat-closed row DISTINCTLY, forever -- a fiat-closed AC/AT/WP/ST never renders as an ordinarily satisfied one.
4. The LLM-side prohibition lands in the rules/skills contract, so an LLM invoking FC violates its own loaded rules and is critic-refusable.

**Detail ruled by hv, 2026-08-28 14:54Z, from a package menu (chosen: one verb, in-model, cascades | declined: flags-on-existing-verbs + ledger; terminal-entities-only):**

1. **One verb:** `intent fc <target> --because "<text>"`, addressing all four kinds (ST, WP, AC, AT); refuses without `--because`.
2. **The record lives IN THE MODEL, on the closed entity:** `{by, date, because, invoker evidence (tty-or-not, timestamp, env fingerprint)}` -- it travels through every sync and every view because it is part of the entity, not beside it.
3. **Cascade:** FC on an ST or WP fiat-closes its open children, and every cascaded child carries an inherited-fiat marker naming the ancestor FC it descends from -- nothing in the tree ever renders as ordinarily closed.
4. **Gate semantics:** a fiat-closed requirement COUNTS as closed (that is FC's purpose -- unblocking the gate) and renders distinctly wherever it appears.

Declined alternatives recorded so an option never on the menu is distinguishable from one refused: `--fiat` flags on the existing close verbs with a separate fiat ledger (an FC'd AC is not "satisfied", and a ledger is a second home for state the model should carry); FC on terminal entities only with no cascade (cleanest per-entity semantics, defeats the cut-and-run use case on a whole over-cooked ST).

**RESOLVED DURING BUILD, recorded rather than deleted because an open list is what a reader checks the code against.** The model field shapes and their extract schema are built: `FiatRecord { because, by, at, invoker, inherited_from, inherited_event }` at **schema 16**, with the three faces at **DDL 13 / SDL 14 / JSON 16**. The six ACs are minted from this design and the builder is dc.

**STILL OPEN, AND EACH IS OPEN FOR A DIFFERENT REASON.**

1. **How `doctor` words a fiat row -- UNBUILT.** `finding.rs` carries no fiat path at all today. This is `AC-00.5`'s subject, and the row has a vacuity risk of the kind `AC-00.4` was rewritten for: *wherever it reports one* passes by absence if `doctor` has no path that can report a fiat-closed entity, so establishing that such a path exists comes before building anything against it.
2. **Whether `0116`'s partial-coverage case gets a distinct rendering when the FC'd AC was half-covered.** `0116` is OPEN and names this thread as the other side of its own gap: FC is the authority-based exit for an unverifiable half, and `0116` is about making that half VISIBLE so the exit is a choice rather than a blind green.
3. **`D4` (minutia 3) -- HELD FOR hv, NOT RULED: does the fiat record carry the accepted-unverified half as free text in `because`, or as a STRUCTURED field?** dc leans structured; it widens the record, so it is hv's. **hv's standing instruction is that dc BUILDS AROUND IT**, and the build did: nothing in the shipped record depends on the answer, so the ruling can land later without a migration.

The motivating case stands as above (D2, `wp done` plus hand-written provenance was the only mechanism).

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0066.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
