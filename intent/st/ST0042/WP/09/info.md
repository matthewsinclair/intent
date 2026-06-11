---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-09
title: "Test-suite hardening"
scope: M
status: Not Started
---

# WP-09: Test-suite hardening

## Objective

Fix the test-suite defects of theme T10. Part A (the real-`~/.claude` pollution fix) is the first work of the whole execution arc because every other WP runs the suite; Part B (vacuous tests, coverage gaps) lands last.

## Evidence

- F-TEST-1 (HIGH, confirmed): two `intent_upgrade_dispatcher.bats` tests (stamp test `:48`, unknown-version test `:86`) run `intent upgrade` with no fake HOME, so the upgrade tail-call (`intent_upgrade:427-441`) overwrites the developer's real `~/.claude` skills/agents on every suite run.
- F-TEST-9: fake-HOME isolation copy-pasted in six test files, absent in the seventh (the direct cause of F-TEST-1).
- F-TEST-4/5/6 (MEDIUM/LOW): `critic_report_format.bats` asserts on heredoc constants defined inside itself; `critic_dispatch.bats` tests a test-local reimplementation of dispatch logic; `critic_config.bats` only verifies the host YAML parser works -- all green regardless of product behaviour.
- F-TEST-3 (MEDIUM): six modules exercised by no test (`intent_llm`, `intent_organise`, `intent_minimal`, `intent_main`, `stp`, `intent_claude_prime`).
- F-TEST-11/12 (LOW): a permanently-`skip`ped cross-FS test; a dead `intent init` invocation with no assertion.

## Deliverables

- Part A (FIRST, lands before WP-01): fake-HOME helper promoted into `tests/test_helper.bash`; all seven files use it; the two offending dispatcher tests isolated; proof that a full suite run leaves real `~/.claude` untouched (checksum before/after).
- Part B (last): vacuous critic tests replaced with assertions against real product behaviour; coverage added for modules still alive post-WP-06 (`intent_llm`, `intent_organise`, `intent_claude_prime`); permanently-skipped test fixed or deleted; dead init invocation given an assertion or removed.

## Acceptance Criteria

- [ ] Full suite run produces zero writes to the real `$HOME/.claude` (verified by checksum/mtime comparison).
- [ ] No test asserts only on data it defines itself or on host-tool behaviour.
- [ ] Every live `bin/` + plugin module has at least one behavioural test.
- [ ] Full bats suite green.

## Dependencies

- Part A: none (deliberately first). Part B: after WP-06 so deleted modules are not given coverage.
