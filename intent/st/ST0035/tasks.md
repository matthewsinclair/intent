# Tasks - ST0035: Canonical LLM Config + Fleet Rollout

## Work packages (forensic detail lives in each `WP/NN/info.md`)

- [x] **WP01** — Self-upgrade Intent to v2.9.1 + cancel ST0010 / ST0015. `WP/01/info.md`. Size: XS. Deps: —.
- [x] **WP02** — Refresh root `usage-rules.md` to current-as-built state. `WP/02/info.md`. Size: S. Deps: WP01.
- [x] **WP03** — Write `intent/docs/working-with-llms.md` canon tech note. `WP/03/info.md`. Size: M. Deps: WP02.
- [ ] **WP04** — Ship `.claude/settings.json` template with SessionStart + Stop hooks. `WP/04/info.md`. Size: M. Deps: WP01.
- [ ] **WP05** — Implement `bin/intent_critic` headless critic runner. `WP/05/info.md`. Size: L. Deps: —.
- [ ] **WP06** — Ship `.git/hooks/pre-commit` template (critic gate). `WP/06/info.md`. Size: S. Deps: WP05.
- [ ] **WP07** — Ship `.intent_critic.yml` default template. `WP/07/info.md`. Size: XS. Deps: WP05.
- [ ] **WP08** — Rewrite root `AGENTS.md` generator (move from intent/llm/ to root). `WP/08/info.md`. Size: M. Deps: WP03.
- [ ] **WP09** — Rewrite root `CLAUDE.md` template (Claude-specific overlay). `WP/09/info.md`. Size: S. Deps: WP08.
- [ ] **WP10** — Delete deprecated artefacts (intent/llm/AGENTS.md, \_llm_preamble.md). `WP/10/info.md`. Size: XS. Deps: WP08.
- [ ] **WP11** — Extend `intent claude upgrade` to apply all canon artefacts. `WP/11/info.md`. Size: M. Deps: WP04, WP06, WP07, WP08, WP09.
- [ ] **WP12** — Socrates/Diogenes FAQ + cross-refs in agent.md files. `WP/12/info.md`. Size: XS. Deps: WP03.
- [ ] **WP13** — Update Intent's own CLAUDE.md to reference canon. `WP/13/info.md`. Size: S. Deps: WP03, WP09.
- [ ] **WP14** — Self-apply canon to Intent repo (dogfood). `WP/14/info.md`. Size: S. Deps: WP11, WP13.
- [ ] **WP15** — Canary rollout: Conflab, Lamplight, Laksa. `WP/15/info.md`. Size: M. Deps: WP14.
- [ ] **WP16** — Fleet rollout: remaining 13 projects (12 Intent + Pplr). `WP/16/info.md`. Size: L. Deps: WP15.
- [ ] **WP17** — Verification sweep + dogfood journal. `WP/17/info.md`. Size: S. Deps: WP16, WP18.
- [ ] **WP18** — Review and update (or retire) `intent/usr/*.md`. `WP/18/info.md`. Size: M. Deps: WP03 (soft WP14).

## Critical path

WP01 → WP02 → WP03 → WP08 → WP09 → WP11 → WP14 → WP15 → WP16 → WP17.

WP17 has two gating inputs: the fleet rollout (WP16) and the user-doc review close (WP18).

Parallelisable branches once WP01 lands:

- WP05 (bin/intent_critic) can start independently of WP02/03 — it's gated only on general understanding of the rule schema.
- WP04 (settings.json template) can start after WP01 in parallel with WP02/03.
- WP06, WP07 need WP05.
- WP12 needs WP03 but doesn't block WP11.
- WP13 can start as soon as WP09 lands.
- WP18 needs WP03 (done) plus a soft dep on WP14; it runs in parallel with WP15/WP16 and must land before WP17 verification starts.

## Dependencies at a glance

| WP   | Blocks                           | Blocked by         |
| ---- | -------------------------------- | ------------------ |
| WP01 | 02, 04                           | —                  |
| WP02 | 03                               | 01                 |
| WP03 | 08, 12                           | 02                 |
| WP04 | 11                               | 01                 |
| WP05 | 06, 07                           | —                  |
| WP06 | 11                               | 05                 |
| WP07 | 11                               | 05                 |
| WP08 | 09, 10, 11                       | 03                 |
| WP09 | 11, 13                           | 08                 |
| WP10 | — (cleanup, no downstream block) | 08                 |
| WP11 | 14                               | 04, 06, 07, 08, 09 |
| WP12 | — (docs overlay, no block)       | 03                 |
| WP13 | 14                               | 03, 09             |
| WP14 | 15                               | 11, 13             |
| WP15 | 16                               | 14                 |
| WP16 | 17                               | 15                 |
| WP17 | — (ST completion)                | 16, 18             |
| WP18 | 17                               | 03 (soft 14)       |

## Phase 0 gate

No WP starts until Phase 0 (this doc + info.md + design.md + all 17 WP info.md files) is committed and reviewed by the user.

## Open decisions blocking WP start

- Open #1 (version 2.9.1 vs 2.10.0) — blocks WP01.
- Open #2 (hook strictness) — affects WP04 acceptance criteria.
- Open #3 (pre-commit threshold) — affects WP07 defaults.
- Open #4 (PostToolUse advisory) — affects WP04 hook set.

Open #5 (cancelled ST location) resolved during Phase 0: `intent/st/CANCELLED/` is the existing convention.

## Task Notes

Keep this file as a high-signal index. All detail (objectives, deliverables, acceptance criteria, risks, verification) lives in `WP/NN/info.md`. Update checkboxes here as WPs flip Done via `intent wp done ST0035/NN`.
