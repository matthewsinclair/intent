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
