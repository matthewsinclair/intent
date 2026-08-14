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
