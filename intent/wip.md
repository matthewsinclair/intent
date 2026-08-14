---
verblock: "14 Aug 2026:v1.07: vc - v2.19.0 SHIPPED (tag 071c612); consumer sweeps next"
intent_version: 2.19.0
---

# Work In Progress

## Current State

**v2.19.0 SHIPPED, 2026-08-14 — tag `071c612` on both remotes plus a GitHub release.** Fifteen issues closed end to end (0009-0023): the AT row grammar + `at lint`/`--fix` (0017, subsuming 0014 + 0015), four AC states with descope/withdraw verb pairs (0013 + hv's addition), one steel-thread enumerator + a voiced `organize` (0011), the whiteboard header block ruled not-YAML and enforced (0012), portable hooks with no baked home path (0016), AGENTS.md prerequisites from the declared languages (0009), the objective-placeholder warning at close (0010), the treeindex cache untracked (0018), the canonical thread index actually indexing all 55 threads (0019), `st list --status all` actually meaning all (0020), Intent's second, dead Elixir enforcement mechanism pruned with the consumer residue reported in three states (0021, from a Laksa report), both no-template fallback heredocs deleted rather than corrected (0022), and `error()` plus its 25 imitators moved to the documented lowercase voice (0023).

**0020, 0021, 0022 and 0023 were all called in by hv before the cut rather than deferred after it** — the batching principle, applied four times, and the reason the release grew from eleven issues to fifteen between the build and the tag.

The cut ran clean: pre-flight green (`intent doctor` + the full suite at HEAD, which is what certified the three commits postdating hv's manual run), five sidecars stamped, CHANGELOG dated, pushed to `local` + `upstream`, release published with the CHANGELOG `[2.19.0]` section as its body. vc verified it afterwards — sidecars at 2.19.0, tag identical across local/upstream/HEAD, tree clean, release body matching the CHANGELOG section but for one trailing newline.

**New practice, adopted this release: the release docs are written BEFORE the cut so the tag carries them** — `intent/history/v2.19.0.md` (internal narrative) and `docs/releases/2.19.0/RELEASE_NOTES.md` (public). Both practices had lapsed (history after v2.16.0, releases after 2.17.0) and are resumed here, deliberately not backfilled. Terse ledger: `intent/done.md`; per-issue record with full verification: `intent/issues/CLOSED/0009..0023`.

## Next Up

1. **Consumer sweeps — one `intent upgrade` per project** (Lamplight first, then Utilz, Baize). The one pass sweeps AT grammar (`at lint --fix`; residue named, never guessed — expect BLOCKED-until-swept, which is the fix working), converges AGENTS.md + portable-hook settings.json + the gitignore entries, and PRINTS (never runs) the `git rm` for a tracked treeindex cache. Lamplight: 314 AT rows plus its four known bad-status contracts (ST0276 `**green` x11, ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`). **Claimed by cc.** vc's part is MEASUREMENT — baseline what the old rows carry, then measure the delta, before the sweep is trusted (the 87-destroyed-name-links lesson).
2. **`credo_checks/` cleanup in the Elixir fleet (issue 0021's consumer half).** Issues filed in the affected projects on 2026-08-14: **Baize 0001** (7 checks, all registered and running, on Credo 1.7.19 with the crash-candidate present — and `struct_vars.ex` is Baize's own, so no blanket delete), **Lamplight 0003** (6 checks, all registered and running, same crash pairing, nothing project-authored), **Conflab 0008** (loaded via `requires:` but 0 registered, so never run once, while `elixirc_paths` compiles them every build — a genuine two-ended removal). Laksa and Prolix measured clean; nothing filed. Each project fixes its own; `intent doctor` reports the residue in three distinct states.
3. **Push the fleet issue-normalisation commits** in their own repos: Utilz (`0171297`), Lamplight (`7058fd3a8`); Conflab already pushed. Utilz-side todo guard carry-over.
4. **hv-ruling queue** (each becomes its own issue if wanted): 0004 item 4 (`ac status` exit code — premise does not reproduce); a `javascript` pack to complete 0009's Node exception; pruning consumers' now-inert `.claude/scripts/` copies; the plugin bins writing errors to **stdout** rather than stderr (named in 0023's Resolutions and deliberately left — it changes what callers capture, not merely what they read), alongside `intent_claude_prime:212`, which is the same decision in miniature.
5. **Whiteboard: inboxes are a pickup-time channel only.** A node that asks a question mid-session cannot see an answer appended to its inbox until it re-reads, and there is no "you have mail" signal — this cost two round-trips on 2026-08-14 with the answer already sitting in the inbox. A protocol change, not a bug fix; wants an hv ruling on shape before anyone builds it.

## Recent

- **2026-08-14**: v2.19.0 SHIPPED (tag `071c612`). Fifteen issues, 0009-0023; four batched in pre-cut on hv's instruction. Release docs written pre-cut for the first time.
- **2026-07-30**: v2.18.0 + v2.17.4 shipped. Earlier: `intent/history/202607-done.md`.

## Parked

_(None.)_
