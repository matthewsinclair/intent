# vc board content archived 2026-08-14 (localfold before compact; the v2.19.0 issue-corpus day, all delivered)

## DOING (delivered)

- Bounce audit of the seven close-out commits at HEAD `5b98a0e`. Verdict CLEAN -- F1-F5 all verified fixed by re-running the original repros (refusals fire, contract byte-identical, probed collision cause), U6/U7 verified behaviorally (declared-language prereqs + count agreement at 55; placeholder warn fires/stays-silent correctly and never blocks), 0018 fully landed (0 tracked, ignored, cache usable), all ten Resolutions read (both corrections verbatim + attributed), docs pass teaches only the new form, 10/10 guard files PASS, and two self-chosen mutations both killed (no fifth vacuous guard found where probed). Entry in `cc/inbox.vc.md` (23:46).
- U1-U5 as-built audit at HEAD `4a0ea96`: substantially sound; findings F1 MEDIUM (`at na`/`at red` arm-mismatch writes a grammar-invalid row then falsely reports "file was NOT updated"), F2 MEDIUM (`ac satisfy` does not refuse off-scope ACs), F3-F5 LOW. Entry 22:36.
- hv-directed triage of the nine open issues 0009-0017: verdict table, two filed-record corrections (0014 bare-`and` works / fused punctuation is the drop; 0011 no false `Moved` -- `set -e` aborts), seven-unit work order, four ratified rulings. Entry 20:59. Issue 0018 filed on hv's treeindex ruling (22:58).
- hv "fix it" x2 executed directly by vc (2026-08-14): `warning()` lowercase voice (`8aba5ab`); and the index residual, which self-refuted on repro into issue **0019** -- the filed residual was WRONG (the five arguments were never read; Created was always correct for rows that appeared), the real defect was `sync --write` composing the WIP-only default into the all-threads index, empty since the last release close. Filed + fixed + closed (`ba52339`); own index 0 -> 55 rows; the correction of vc's own claim recorded in 0019's Resolutions with the 0014 discipline. Both fixes mutation-checked.

## Watch-outs (resolved or recorded elsewhere)

- The two MEDIUM findings (new verb surface) -- fixed on the bounce, guards mutation-proven.
- 0017 `path::name` migration hazard + gate-strictness estate flip -- handled in U1 (two-arm grammar, `--fix`, ledger step); CHANGELOG says so.
- L2 applies to COMPLETED threads -- accepted consequence, recorded in 0017's Resolutions.
- treeindex ruling -- resolved by hv, issue 0018, fixed + closed.
- `warning()` voice -- resolved by hv "fix it", `8aba5ab`.

## Decisions (settled, executed)

- (2026-08-13) Triage rulings delivered with the work order: 0017 single-release strict gate + `--fix`; 0012 fork = NOT-YAML; 0009 prerequisites from declared languages, no probe back-fill; 0010 warn-not-block. 0014 + 0015 close via 0017; 0013 `struck` deferral overtaken same-day by hv's direct withdraw-verb instruction. All executed and recorded in the respective Resolutions.

# ---- second archive, 2026-08-14 (localfold before compact; the v2.19.0 SHIP day) ----

## DOING (delivered)

- **v2.19.0 SHIPPED and VERIFIED.** Tag `071c612` on both remotes + GitHub release. Cut verified: five sidecars at 2.19.0, CHANGELOG dated, tag identical across local/upstream/HEAD, tree clean, release body byte-identical to the CHANGELOG `[2.19.0]` section but for one trailing newline. Globalfold done (`aea67dd`).
- **Fifteen issues, 0009-0023.** vc fixed and closed four of them pre-cut on hv's batching instruction: 0020 (`st list --status all` membership), 0021 (the dead `credo_checks/` mechanism, from a Laksa report), 0022 (both no-template fallback heredocs), 0023 (`error()` voice, 26 sites). Each mutation-proven; each with Resolutions recording the judgement calls and the mistakes.
- **Release docs written PRE-cut for the first time** (`intent/history/v2.19.0.md` + `docs/releases/2.19.0/RELEASE_NOTES.md`), so the tag carries them. Both practices had lapsed -- history after v2.16.0, releases after 2.17.0 -- resumed, deliberately not backfilled.
- **Lamplight sweep baseline taken** (`intent/analysis/20260814-lamplight-at-sweep-baseline.md`): 1639 AT rows, not the 314 our notes claimed; 70% carry a shape `--fix` refuses. Sweep itself left to cc, who claimed it.
- **`credo_checks/` consumer half**: issues filed in Baize (0001), Lamplight (0003), Conflab (0008), each written to that project's measured state. Laksa + Prolix clean, nothing filed.

## Corrections absorbed this day (all mine, all caught by someone else or by an implausible number)

- cc caught my `bin/intent_st:731-741` line reference, stale within a day of being written. Re-anchored on the comment string; the correction and its reason recorded rather than the number quietly swapped.
- cc caught four documents claiming the suite was green "at HEAD" when three code commits postdated the run. Repaired by naming the commit the run covered and letting pre-flight speak for HEAD.
- cc caught `e1e2300` half-sweeping `wip.md` -- count moved, enumeration did not.
- My own first Lamplight status measurement reported "30+ distinct statuses" from an unscoped grep. The scoped pass says 9. Caught because the number was implausible.

---

# PM session archive (2026-08-14) -- ST0056 opened

## DONE (pm)

- **ST0056 (Intent v3.0.0) planning arc, hv-assigned to vc**: rubber-duck session ratified the architecture (schema-as-truth after hv's pushback on md-as-truth; intentsvcs layering; one intentd per machine, IN the 3.0.0 gate; JSON canon; strict ingest; MCP tiered surface; migration floor v2.19.0; Homebrew core; cloud seams project_id/principal/event-log/server-block). Lamplight `native/cli` + Conflab `native/daemon` trawled as prior art -- conflabd's stack maps nearly 1:1 onto intentd (async-graphql+axum, rmcp streamable HTTP, CLI-owned launchd lifecycle, mgmt plane, debounced watching, policy stamps, TN3171 cert lesson).
- **Docs landed**: design.md (architecture + D01-D17 + alternatives + stack), info.md rewritten (LLM Preamble removed per hv), tasks.md (ladder + deps), impl.md (prior-art record), acceptance.md (ST-level gate AC-00.1..8 + WP-01 group, lint-clean 10 rows), 12 WP info files populated with sizes. wip.md + restart.md moved to the ST0056 era.
- **Board ops**: split protocol with cc agreed (cc counts post-sweep as stop condition, vc re-counts as record); cc corrected on the clock-skew watch-out (my BST-as-Z heartbeat bug, not skew); ST0056 claim + assignment notice sent and cleared by cc.

## Decisions folded out (recorded in restart.md / memory / issue records)

- Mutation batteries hard-fail on unchanged source; reproduce in a sacrificial worktree; line numbers expire -- anchor on symbols; alarming one-line-grep numbers are findings to check; own claims get the refutation discipline.

---

# PM-2 session archive (2026-08-14) -- WP-01 closed, WP-02 handed over

- WP-01 closed through the gate (4/4) on hv ratification; specs authored (data-model / migration / parity + 62-AC contract, lint-clean); D18-D21 closed the open questions.
- WP-02 foundation built by vc pre-ruling (workspace, model, store, faces, 4 mutation-proven guards, CI) at 5e4b766; global-gitignore *.sql trap caught by reading the commit file list.
- hv ruling: cc + ic write the code, vc stewards. Handover briefs sent; cc landed the SDL face (732affa, gates 4/6) and is bringing devbin into bin/ (entry point bin/int; bin/intent untouched); ic landed the full parity deep pass (26 cmd files, 94-row register, INTENT_BIN retarget, 711/1235 reach the CLI) and localfolded.
- cc killed the sweep program with evidence (Lamplight at 2.19.0, their hv ruled AT remediation dead) -- steward docs updated: corrected class proposed in parity.md, WP-10 corpus reworded in tasks.md, migration policy question opened in migration.md, the model.rs one-authority law reworded.

## PM-3 (EOD wrap)

- The bounce: hv ruled all five agenda items (corrected RATIFIED; carry policy RATIFIED -- lossless-by-carrying CLOSED / BLOCKED-until-clean LIVE; organize planned vestigial by construction; push when it makes sense; v2 maintenance default-defer, show-stoppers only). Recorded in parity.md + migration.md (`736033d`); builders notified.
- First pushes of the v3 estate to both remotes. First rust CI run GREEN in 1m47s (31812129560: macOS+Linux, fmt --check + clippy -D warnings + tests) and Intent Tests green on the same `736033d`. AC-02.1 satisfied by that named evidence.
- 0024 close review: sound, close stands. Fix verified on the as-built (denominator narrows with the findings, out-of-scope rows copied verbatim, scope in the label); guard 5/5 at HEAD in a clean worktree. Two findings: cc's board cited the unpushed pre-amend twin `1f5e354` (landed commit `e685e90`); test 2's `grep -qv` latent vacuousness. Both actioned by cc; `8b7d382` verified.
- WP-02 CLOSED on cc's claim: AC-02.6 renumbered to AC-04.5 / AT-04.5 with provenance, `at lint` clean (60 rows), gate ST0056/02 PASS 5/5, `wp done` recorded. cc's applied-is-not-reached mutation lesson absorbed into parity.md's working rules.
- hv inbox sweep on hv's instruction: all six hv-direction inboxes (`hv/inbox.{cc,ic,vc}`, `{cc,ic,vc}/inbox.hv`) verified empty -- nothing to clear.
- ic's clock guard (`ddac6ba` + the Re:-anchor fix `98ce764`): whiteboard stamps now need the trailing `Z` at the pre-commit gate; vc's 14:34/15:03 unmarked entries predate it and stand as-is in peers' archives.

---

## PM-4 -- the three-node AFK window (17:05Z -> 22:18Z)

hv went AFK, handed all three nodes the pen, and asked how far we get on the Rust CLI and services layer unattended. Both builders were idle at pickup. Dispatched cc to WP-03 whole and ic to the dispatch-table SSOT in parallel; vc stewarded and did not build.

### Delivered

- **WP-03 CLOSED 6/6** at cc's `476f1e1` -- 15 targets / 60 passed, verified by re-running. **D24 mechanically protected**: mutated `sync.rs:161` to a stat gate in a sacrificial worktree, exactly one test red, prediction written first.
- **WP-04 CLOSED 5/5** at `ce2bb3b`, after a **bounce**. v2's gate enforces five rules and v3 had two; `cmd_ac_gate` calls `at_lint_report` and blocks on it, so L1-L5 are GATE rules. `at lint` is a validator the gate calls, not a read surface -- filing it under the wrong noun is how L4/L5 nearly shipped missing. Then independently enumerated all EIGHT verdict paths in `cmd_ac_gate` (cc had studied one) and confirmed every constructible one is covered; `intent_wp:156` and `intent_st:470` call the gate and check nothing themselves.
- **WP-05 at 3/4**, blocked on AC-05.3 (register 97 rows vs 98 files, missing `whiteboard_clock_guard.bats`). AC-05.4 satisfied by vc review: 713 lines, zero DB/fs/path reaches.
- **WP-13 created and specced whole** -- project search in four tiers (T0 retire treeindex + in-handoff, T1 FTS, T2 tree-sitter, T3 semantic, T4 LSP parked). tree-sitter rather than an Elixir-specific parser because the `languages` array is already the grammar manifest; **T2 is the chunker T3 needs**, so the staging is a dependency rather than a convenience.
- **Contract 62 -> 77 ACs**, every one an addition. Five came from cross-checking all twelve deliverable lists against all sixty-two ACs.
- **D22-D27 landed**; hv's standing authorisation converted them from PROVISIONAL to ADOPTED, with what "adopted" means defined once in design.md so it is not confused with review.
- ic delivered 27 families / 92 entries and folded; seven measurement rules landed in parity.md.

### Decisions now living in committed artefacts (archived from the live board)

Each is in the ST canon; a second copy here is the divergent-copy drift Highlander exists to stop.

- Deliverable lists are not gated -> acceptance.md, WP-02 finding.
- Derive the law, do not audit the instances -> D23.
- When contract and narrative disagree, the contract governs -> D24.
- A divergent copy proves itself the moment the original moves -> data-model.md schema-face section.
- Unratified is an answer given by default -> README roster + design.md adoption note.
- Message text is not in the parity contract -> D27.
- The dispatch table must leave the ST tree -> D26.
- Calibrate before believing a zero; clean-by-luck vs clean-by-construction; a file named after a command that does not test it; file-level classification is structurally blind; success is reported by the mechanism -> parity.md `## Measurement rules`.

### The window's own lesson

Three wrong premises crossed between nodes and three were caught in one hop -- ic's clap overclaim, my zsh probe, my `at set` verb name. The one I did NOT re-run, cc's absent-file report, became a wrong ruling until hv caught it. Against a failure mode that is plausible and silent by construction, that ratio is the measurement worth keeping; artefact sizes are not.
