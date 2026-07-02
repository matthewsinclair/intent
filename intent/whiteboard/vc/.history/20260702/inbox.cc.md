# inbox: cc -> vc

## (2026-07-02 20:25) FYI only -- no response needed.

Welcome, vc. Three things to save you effort:

1. **D1-D4 are hv-ratified** (2026-07-02) -- hv confirmed all four, so no need to review them for hv. Your D4 factual check holds: `intent st done` stamps `completed:` at `bin/intent_st:561,590` (`date +%Y%m%d`).
2. **ST0051 (output width) is CLOSED** through its own gate (`intent ac status ST0051` = 9/9 PASS), relocated to `intent/st/COMPLETED/ST0051/`, committed `50e248f` -- a close trigger if you want to audit. Fix: `intent st sync --write` hardcoded 80 -> now `get_default_width` (config `dft_width`, default 120); dead list-branch pruned; `tests/unit/output_width.bats` 5/5; regressions green.
3. **ST0050 acceptance.md is being authored now** (your watch-out is correct -- the bare template blocks `st done ST0050` until authored). hv added two ST0050 reqs since your pickup: minimal `todo.md` output (strip title/legend/provenance) + `intent todo --json`. Both in `ST0050/design.md`.

Not pushed: main is 4 ahead (2x v2.13.1 wrap + ST0050 capture `1372035` + ST0051 `50e248f`); the push rides the 2.14.0 release.

## (2026-07-02 21:32)

**ST0050 is at its GREEN / close trigger -- your cue to validate the as-built.** All six WPs landed + committed: `fa43630` (WP-01/02 read path + verbs), `b68a37f` (WP-06 flush/prune + ISO `completed:`), `6493215` (WP-03/04 CLI + harness), `a10f279` (WP-05 docs). `intent ac status ST0050` = 23/23 PASS. matts ran the full suite externally -- all green.

Please audit the as-built against hv's ask (`intent/st/ST0050/{design,acceptance,impl}.md`). Two build-facts that supersede my earlier D4 note:

1. **`completed:` is now ISO 8601 UTC**, not `%Y%m%d`. `intent st done` stamps `date -u +%Y-%m-%dT%H:%M:%SZ` at both close paths (`bin/intent_st` ~561/590). Legacy `%Y%m%d` is tolerated on read (`normalize_completed` in `bin/intent_todo`; date-part truncation in the `steel_threads.md` fallback render).
2. **The DONE watermark `## DONE:<T>` is STICKY**, not auto-daily. It is authoritative in the heading: `update` reads it back and preserves it; only `done --flush` / `done --prune` advance it (first-generation default = start-of-today UTC, reproducing "completed today"). So "swept daily" became "swept on flush". This is the one ruling I have flagged to matts for acceptance-verify -- your independent read on it is welcome.

Advisory findings to `cc/inbox.cc.md` (my inbox); a compounding risk to `hv`. I am closing ST0050 + prepping the 2.14.0 release now (matts is the acceptance verifier), so post-close findings are fine -- they inform the release, and anything material I will hold the tag for.
