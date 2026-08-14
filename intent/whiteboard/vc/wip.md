---
node: vc
name: Validation Claude
role: validation
session_id: 15e0a23e-58f3-4575-882d-e23418452447
heartbeat_at: 2026-08-14T14:15Z
status: paused
focus: "ST0056 steward, released for compact. WP-02 at 4/6 with cc; ic parity landed; bounce agenda for hv is queued in TODO."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- (released for compact; stewardship resumes on pickup)

## TODO

- **The bounce agenda -- hv rulings queued, in rough priority:**
  1. Ratify (or strike) the **`corrected` parity class** -- drafted in parity.md from ic's census (unknown-flag exit 0, --help failures on 10/27, the 45/12/2 stream census).
  2. **Migration policy for never-swept estates** -- drafted as the open question in migration.md: lossless-by-carrying for CLOSED threads vs BLOCKED-until-clean for live ones; forced by Lamplight's hv ruling AT remediation dead (~1158 rows permanent).
  3. **The organize Highlander** (ic's gap 1): `intent organize` and `intent st organize` are two implementations of one job, both registered in MODULES.md; which survives into v3 is a ruling, then a register row.
  4. **AC-02.6 at WP-02 close**: red-until-WP-04 vs descope-to-WP-04 (`event_log_envelopes.rs` cannot exist before facade verbs).
  5. **v2 maintenance scope during the build** -- still unstated as policy; 0024 got a one-off go.
- **Review WP-02 at cc's close claim**: AC-02.1 flips on the first green CI run (needs a push); AC-02.6 per ruling above; verify the SDL face + AcScopeView projection guards on the as-built.
- **Review ic's register follow-ups**: the 95th test file (`at_lint_wp_scope.bats`, post-baseline) has no row; the 40 `split` files want per-test rows.
- **0024 close review** (fire-on-claim): cc's scope fix in `bin/intent_acceptance` + guard; also flows into the WP-05 scope-honouring conformance property.

## Watch-outs

- **Never mutate `bin/**` or `tests/**` in place** -- `~/.local/bin/intent` symlinks into this repo; sacrificial worktrees only. `crates/**` has no symlink hazard, but suites run concurrently.
- **The machine-global gitignore ignores `*.sql`** -- committed faces need their `!` exception; `git check-ignore -v` any new non-json artefact.
- **This shell is zsh; MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal.**
- **Commit by explicit pathspec, never `-A`** -- cc and ic run concurrently; devbin's arrival has `bin/release` mid-move in the tree.
- Release-window mechanics live in `intent/restart.md`'s checklist.

## Decisions

- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc keeps the ST0056 claim as steward (contract, verification at WP closes, hv interface) and does not build. The WP-02 foundation vc built pre-ruling was mutation-proven before handover and is cc's to extend.
- (2026-08-14) **Contract changes route through vc even when proposed by builders** -- ic's `corrected` class and cc's scope-honouring property were both drafted into parity.md by vc, marked for hv ratification; cc's law finding was resolved by rewording the law (authored-once is about where names are authored, not every transport spelling them identically).
- (2026-08-14) **Stamp heartbeats from `date -u`, never a local clock reading suffixed `Z`.**
- (2026-07-02) vc fires on cc's close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs (ST0056 stewardship excepted).
