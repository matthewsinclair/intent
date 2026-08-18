# Tasks - ST0055: Add `intent issues` command

## Tasks

- [x] WP-01 Foundation: MODULES.md row, Intent-owned template, lazy scaffold, dispatch skeleton + help, id-alloc + slugify reuse
- [x] WP-02 Create & list: `add` (id/slug/stamp, `--severity`, `new` alias), `list --kind open/closed/all`
- [x] WP-03 Inspect & lifecycle: `show` (+`--json`), `close`, `open` (move NNNN dir + status mirror), RESOLVED normalisation
- [x] Companion chore: `sanitize_title` pipe fix at st/wp/issues input boundary + guard test
- [~] WP-04 Gate & integration: bats green (22 new), `intent help` wired, doctor green; critic-shell in progress; ac gate + matts verification pending
- [ ] WP-05 Fleet normalisation (POST-SHIP, cross-repo): Lamplight, Conflab, Utilz, Intent

## Task Notes

- Local single-file bats: `intent_issues.bats` 19/19, `title_pipe_sanitize_guard.bats` 3/3. Full suite is matts's to run.
- WP-05 runs after the command ships and is dogfooded; it does not gate the Intent release.

## Dependencies

- WP-02/03 depend on WP-01 (module skeleton + helpers). WP-04 gates on WP-01..03. WP-05 depends on a shipped command.
