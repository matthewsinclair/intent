---
verblock: "05 Apr 2026:v0.2: matts - As-built: 6 lenses complete, 9 internal ACIs"
intent_version: 2.8.0
status: WIP
---

# WP-02: Pilot Extraction -- Intent + Lamplight + MeetZaya

## Objective

Apply all six extraction lenses to Intent (process), Lamplight (code), and MeetZaya (failure). Produce raw ACI candidates. Calibrate the protocol.

## As-Built (2026-04-05)

All 6 lenses applied. 9 internal ACIs produced. Calibration notes updated. Detrope gate passed (mechanical 0 flags, full LLM analysis done with remediation applied).

### Lens Results

| Lens                      | Repos analyzed                                                     | ACIs                   | Notes                                              |
| ------------------------- | ------------------------------------------------------------------ | ---------------------- | -------------------------------------------------- |
| 1: Rule Archaeology       | Intent (28 edits), Lamplight (10), MeetZaya (17)                   | ACI-006, 007, 008, 009 | Cross-repo comparison most valuable                |
| 2: Plan-Outcome Delta     | Intent (4 STs), Lamplight (4 STs), MeetZaya (5 STs)                | ACI-022                | Agent-assisted deep analysis of 13 STs             |
| 3: Correction Mining      | Intent (14 sessions via autopsy)                                   | None standalone        | Thin corpus, 6 corrections. Supports existing ACIs |
| 4: Architecture Forensics | Lamplight (renames, cafe namespace), MeetZaya (regression cascade) | ACI-010, 011           | git shortstat + rename filter most effective       |
| 5: Methodology Evolution  | Cross-repo timeline (all 3)                                        | ACI-012                | Synthetic lens, best after Lenses 1+4              |
| 6: Failure Archaeology    | Lamplight (17 cancelled STs), MeetZaya (13 not-started)            | ACI-021                | Agent-assisted, found batch-creation pattern       |

### What's Missing

- MeetZaya user interview for non-coding failure reasons (Lens 6 blocker)
- Lamplight session autopsy (72 sessions, 861MB -- not attempted this session due to corpus size)
- Lens 2 findings from Lamplight and MeetZaya are rich but not yet converted to additional ACIs (data captured in agent output, available for future extraction)

## Deliverables

- [x] Lens-by-lens findings for Intent, Lamplight, and MeetZaya
- [ ] MeetZaya failure case study (coding reasons found; non-coding reasons require user interview)
- [x] 20-25 raw ACI candidates (22 total including WP-03 landscape ACIs)
- [x] Protocol calibration notes (updated in extraction-protocol.md, all 6 lenses checked off)

## Acceptance Criteria

- [x] Each lens produced findings (or documented why not)
- [ ] MeetZaya produces 5-8 failure-specific insights (3 found from data; non-coding reasons pending interview)
- [x] Clear signal on which lenses are highest-yield (Lens 1 and Lens 6 highest; Lens 3 lowest for Intent's small corpus)

## Dependencies

WP-01 (need ACI format and extraction protocol).
