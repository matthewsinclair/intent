---
verblock: "02 Jul 2026:v0.1: matts - Initial version"
st_id: ST0051
title: "intent output width: dft_width config for generated files, terminal width for stdout -- acceptance contract"
---

# ST0051 intent output width: dft_width config for generated files, terminal width for stdout -- Acceptance

> Canonical acceptance contract for ST0051. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).
>
> Non-test ACs carry their state inline -- `-- evidence: <ref> -- satisfied: yes|no` on the AC line; test-backed ACs are satisfied by a green covering AT (computed, never written). Multi-AC coverage on an AT is comma-separated.
>
> Exemption (ST0048): the close-gate is fail-by-default -- a unit with an empty or missing contract is refused. A unit that is deliberately AC-free (eg a pure content / authorial task) declares `acceptance: exempt` in the frontmatter above; the gate then passes and announces the exemption. Omit it (the default) and the contract is enforced. Never inferred from emptiness; always declared.

## Acceptance Criteria

### ST-level

None -- WP-distributed.

### WP-01 -- dft_width config + width helper (status: Not Started)

- AC-01.1 `get_default_width` returns the project config's `dft_width` value when it is set.
- AC-01.2 `get_default_width` returns 120 when `dft_width` is absent or empty in config.
- AC-01.3 (non-test) `get_default_width` is the sole file-width source, reuses `read_config_field`, and leaves `get_terminal_width` unchanged (Highlander -- no duplicate width logic). -- evidence: review of `bin/intent_helpers` (get_default_width wraps get_config_field; get_terminal_width untouched) -- satisfied: yes

### WP-02 -- destination-based width in intent st (status: Not Started)

- AC-02.1 `intent st sync --write` sizes `steel_threads.md` from `get_default_width`, independent of the terminal width the command runs under.
- AC-02.2 `intent st list` renders to stdout at the live terminal width (`get_terminal_width`).
- AC-02.3 an explicit `--width N` overrides both the file path and the stdout path.
- AC-02.4 (non-test) the regenerated `steel_threads.md` renders every current thread's slug without ellipsis at width 120. -- evidence: the committed `intent/st/steel_threads.md` (ST0046 slug renders in full) -- satisfied: yes

### WP-03 -- audit + principle (status: Not Started)

- AC-03.1 (non-test) every other file-generating command is either non-columnar (width-agnostic) or routed through `get_default_width` for its file output; the audit is recorded in `impl.md`. -- evidence: `impl.md` generator-audit table -- satisfied: yes
- AC-03.2 (non-test) the file-vs-stdout width rule is documented where a future generator author will see it. -- evidence: `usage-rules.md` "### Generated file width" + the `get_default_width` helper comment + MODULES.md -- satisfied: yes

## Acceptance Tests

### WP-01

- AT-01.1 tests/unit/output_width.bats::dft_width_read_from_config -- covers AC-01.1 -- status: green
- AT-01.2 tests/unit/output_width.bats::dft_width_defaults_to_120 -- covers AC-01.2 -- status: green
- Coverage: AC-01.1 + AC-01.2 covered; AC-01.3 is non-test (Highlander review + guard).

### WP-02

- AT-02.1 tests/unit/output_width.bats::sync_write_uses_default_width_not_terminal -- covers AC-02.1 -- status: green
- AT-02.2 tests/unit/output_width.bats::list_uses_terminal_width -- covers AC-02.2 -- status: green
- AT-02.3 tests/unit/output_width.bats::explicit_width_overrides_both -- covers AC-02.3 -- status: green
- Coverage: AC-02.1 + AC-02.2 + AC-02.3 covered; AC-02.4 is non-test (eyeball on the committed file).

### WP-03

- Coverage: AC-03.1 + AC-03.2 are non-test (audit note + doc entry); no AT.
