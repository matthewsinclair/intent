# ST0036 Design: `.intent/` → `intent/.config/`

This document captures the provisional design rationale for the directory relocation. Individual implementation decisions surface during Phase 0 forensic WP authoring.

## Canon decisions (provisional; finalize in Phase 0)

### D1. New location is `intent/.config/`

Alternatives considered:

- `intent/config/` (no leading dot) — breaks the "configuration is hidden from default listings" semantic that `.intent/` originally conveyed. Rejected.
- `intent/.intent/` — mirrors Git's `.git/` convention but duplicates the project name redundantly. Rejected.
- `intent/_config/` — underscore-prefix is used for Intent's template internals (`lib/templates/`). Overloading it is confusing. Rejected.

**Chosen:** `intent/.config/`. Leading dot preserves the "hidden config" semantic; `.config/` matches the generic convention used by many tools (`~/.config/`, `.vscode/`).

### D2. Migration is atomic; fail-forward; no backwards-compat

Following Intent's established posture:

- No symlink from `.intent/` → `intent/.config/` left behind.
- No "support both paths for one version" period.
- No configuration flag to retain the old location.

The migration function (`migrate_v2_9_0_to_v2_10_0` — already scaffolded during ST0035 retarget) runs once on first invocation of a v2.10.0-aware binary:

1. Check: if `intent/.config/` already exists and `.intent/` does not, no-op.
2. If `.intent/` exists: create `intent/` if absent; move `.intent/` → `intent/.config/` via `mv`; remove the now-empty `.intent/`.
3. Stamp `intent/.config/config.json` with `intent_version: "2.10.0"`.

If the move fails mid-run, the project is in an inconsistent state. Mitigation:

- Write `intent/.config/.migration-in-progress` sentinel before the move; remove after the stamp completes.
- Subsequent invocations detect a non-empty sentinel and report "migration interrupted; recover manually" with a short recovery doc.
- The move itself is a single `mv` — POSIX `rename()` is atomic on the same filesystem; cross-filesystem moves fall back to copy + checksum + delete.

Concrete algorithm lives in `WP/01/info.md` once Phase 0 elaborates.

### D3. User-curated files under `.intent/` are preserved verbatim

`.intent/` typically contains:

- `config.json` — project metadata. Moves to `intent/.config/config.json`.
- `backup/` — local file backups. Moves to `intent/.config/backup/`.
- `cache/` — transient cache data (gitignored). Moves to `intent/.config/cache/`.
- Occasionally user-added custom files. Preserved by whole-directory move.

The migration does not try to understand contents; it moves the tree.

### D4. Shared rollout with ST0035

No separate ST0036 rollout. ST0035's canary (WP15) and fleet (WP16) WPs carry both the LLM canon changes and the directory move in a single `intent upgrade` per fleet project.

Consequence: ST0036's implementation WPs (WP01 through the pre-rollout Intent self-apply) must land **before** ST0035/WP14 (Intent self-dogfood). After that, ST0035/WP14–WP17 carry both concerns.

### D5. CHANGELOG + migration guide

v2.10.0's CHANGELOG entry (drafted during ST0035 retarget) flags the breaking change prominently. A new doc `intent/docs/migration-v2.10.0.md` explains in prose what moved and what user-side updates are needed (script / alias / CI path updates).

## Scope (provisional)

### In scope

- `bin/intent_config` + `bin/intent_helpers` + `bin/intent_upgrade` — path constants and probes flip from `.intent/` to `intent/.config/`.
- Every string literal referencing `.intent/` across `bin/`, `intent/plugins/`, `lib/`, `tests/`, `intent/usr/`, `intent/docs/`.
- Template files (`lib/templates/**/*`) that reference the old path.
- Generator output (root `AGENTS.md` / `CLAUDE.md` / `usage-rules.md`) — ensure new path is used.
- `.gitignore` templates + `.treeindexignore` canonical patterns.
- All BATS fixtures + scratch-project setup helpers (`create_test_project` in `tests/lib/test_helper.bash`).
- Migration function completion + unit tests + integration test against a populated `.intent/`.
- New doc: `intent/docs/migration-v2.10.0.md`.
- Intent self-apply (shared with ST0035/WP14).
- Canary + fleet rollout (shared with ST0035/WP15 + WP16).

### Out of scope

- `~/.intent/` (the **user-level** extension root at `$HOME/.intent/ext/`, introduced in v2.9.0). That stays at the home level; it is not per-project metadata.
- `~/.claude/` (user-level Claude Code config).
- Any renaming of `intent/` itself.
- Features beyond what's needed to relocate.

## Risk register (provisional)

| Risk                                                | Likelihood | Impact | Mitigation                                                                                   |
| --------------------------------------------------- | ---------- | ------ | -------------------------------------------------------------------------------------------- |
| User CI / aliases / editor plugins break silently   | High       | Medium | Prominent CHANGELOG entry; migration guide; inline upgrade notice on first run.              |
| Mid-migration failure leaves inconsistent state     | Low        | High   | Sentinel file; single-transaction `mv`; documented recovery; BATS test for interrupted case. |
| Some `.intent/` file has a hard-coded absolute path | Low        | Medium | Phase 0 audits `.intent/` contents across the fleet to identify any such files.              |
| Third-party tooling hard-codes `.intent/`           | Medium     | Low    | Accept — fail-forward. Doc the change so users know where to look.                           |
| Slippage pushes ST0035 rollout                      | Medium     | Medium | ST0036 WPs have ordering deps on ST0035/WP13; Phase 0 sequencing locks order.                |
| Cross-filesystem `mv` fails (rare)                  | Low        | Low    | Implementation falls back to copy + checksum + delete on `EXDEV`.                            |

## Open questions (resolve in Phase 0)

1. Migration sentinel file name — `intent/.config/.migration-in-progress` vs something terser?
2. `intent/docs/migration-v2.10.0.md` tone — prose or checklist?
3. Backup directory placement — `intent/.config/backup/` (simple; nested under `.config/`) vs `intent/.backup/` (elevates backup to a peer of `.config/`)? The former is simpler; the latter mirrors Git-ish patterns where state lives beside config.
4. Should the migration emit a terminal banner on first run warning the user of the move, or rely solely on the CHANGELOG + migration guide?
