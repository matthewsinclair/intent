---
id: "0022"
title: The no-template fallback heredocs in intent_st and intent_wp are a divergent second copy of generated content, and both have already drifted from the templates they shadow
date: 2026-08-14
reporter: matts
status: CLOSED
severity: medium
---

# 0022: The no-template fallback heredocs in intent_st and intent_wp are a divergent second copy of generated content, and both have already drifted from the templates they shadow

## Tags

highlander, templates, silent-failure, steel-threads, work-packages, prune

## Summary

`intent st new` and `intent wp new` each carried a heredoc fallback that wrote a "minimal" `info.md` whenever the templates could not be found. That is a second copy of generated content, which project rule 6 (single template source) forbids -- and both copies had already drifted from the templates they shadow, in ways that contradict live contracts.

Raised by cc as "adjacent, not fixed" and pulled into v2.19.0 by hv on the batching principle: anything that goes in should go in before the tag rather than trickle after it.

## Reproduction

Read at HEAD, then exercised against a deliberately broken install (a tree with `bin/` and no `lib/templates`).

**The WP fallback wrote a retired form.** `bin/intent_wp:138` emitted:

```
## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2
```

The live template (`lib/templates/prj/st/WP/info.md:19`) says the opposite:

```
## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`,
under the `WP-NN` heading (single source of truth). Do not restate ACs here.
```

So the shadow copy was actively instructing users to do the thing the real template forbids -- restating ACs in the work package -- in a form ST0044 retired when `acceptance.md` became the single home.

**The ST fallback shipped an incomplete thread.** `bin/intent_st:368` wrote `info.md` and nothing else. A thread born from it silently lacked `acceptance.md`, `design.md`, `impl.md` and `tasks.md` -- so it had no acceptance contract at all, and every gate that reads one would find nothing to read.

## Root Cause

Rule 6 exists for exactly this: **nothing keeps a shadow copy honest.** The templates are edited by people working on templates; the heredocs are edited by nobody, because they are invisible until an install breaks. Both drifted, and neither drift was noticed -- the WP one had been contradicting its own template since ST0044.

Underneath it is a second fault, and it is the same one as issue 0021: the fallback treats a broken install as **a condition to work around rather than a condition to report**. `TEMPLATE_DIR` missing means `INTENT_HOME` is wrong or the install is incomplete. Quietly substituting divergent content hides that, and hands the user a document that looks right and is not -- `IN-AG-NO-SILENT-001`.

The 0010 drift guard did cover the placeholder strings, so the objective warning could not silently stop firing. But guarding a second generator only keeps two copies in step for as long as someone keeps running the guard; it does not make the second copy correct, and it did not catch the `## Acceptance Criteria` divergence, because that was never what it looked at.

## Impact

- **Low blast radius, high dishonesty.** The fallbacks fire only when `lib/templates/` is missing, ie a broken or partial install -- so in a healthy tree they are dead code. But when they do fire, they produce a steel thread with no acceptance contract, or a work package instructing the reader to violate the single-source rule, and they report success either way.
- **The user is not told their install is broken**, which is the one thing they need to know and the only thing that would let them fix it.
- Rule 6 is a project rule with a mechanical registry behind it; a standing violation in the two most-used creation commands undermines it everywhere else.

## Proposed Fix

**Delete both heredocs and fail loudly.** Do not update them to match the templates -- that restores two copies and buys another year of drift.

1. `bin/intent_st`: replace the fallback branch with `error`, naming `$TEMPLATE_DIR` and the resolved `INTENT_HOME`.
2. `bin/intent_wp`: the same, naming `$TEMPLATE_FILE`.
3. Remove the partially-created directory before erroring, so a refusal leaves nothing behind and the claim "nothing was created" is true.
4. Invert the two 0010 drift guards that assert the constants still match the heredocs -- there is now exactly one generator per document, and the guard should pin that there is no longer a second one to drift.
5. Guard: a broken install makes `st new` and `wp new` refuse, name the path, and leave no directory; a healthy install still writes all five ST documents and a WP carrying `## Acceptance` and not the retired checkbox form; and mechanically, no `TEMPLATE` heredoc returns to either creator.

## Related

- 0021 -- the same week, the same shape: a second mechanism Intent shipped, left unmaintained, that drifted where nobody was looking. Its lesson applies verbatim -- presence reads as correctness to whoever finds it.
- 0010 -- built the drift guard over the four generators, which is what kept the placeholder half of this honest, and is now inverted for the two that are gone.
- ST0044 / ST0048 -- made `acceptance.md` the single home for ACs, which is the contract the WP heredoc had been contradicting ever since.

## Resolutions

**Fixed in v2.19.0 (before the cut, on hv's batching instruction). Raised by cc; executed by vc.**

All five proposed items taken as filed, including the explicit refusal to update the heredocs in place.

**Both fallbacks deleted.** `bin/intent_st` and `bin/intent_wp` now call `error` naming the template path it looked for and the resolved `INTENT_HOME`, and each removes the directory it had already created (`rmdir`, which succeeds only if empty) so a refusal genuinely leaves nothing behind. Comments at both sites record what the deleted copies had drifted into, so the reason survives the code.

**The two 0010 drift guards are inverted**, from "the constant still matches the heredoc" to "there is no second generator carrying it". Their original comment made this issue's argument -- _"the fallback is a second generator of the same document; if it drifts, threads born from it are invisible to the warning"_ -- which was the right worry and the wrong remedy.

### Verification

**Behavioural, both directions.** Against a deliberately broken install (`bin/` present, `lib/templates` empty), `st new` and `wp new` both exit 1 with the path named. Against a healthy install, `st new` still writes all five documents including `acceptance.md`, and `wp new` still carries the template's `## Acceptance` pointer.

**The "nothing was created" claim is checked, not asserted** -- the error message makes a statement about the post-state, and a verifier of results may not state conclusions it has not verified (the `assert_written` lesson from this release). The guard counts `ST[0-9][0-9][0-9][0-9]` directories after a refusal and requires zero, and checks the WP directory is absent.

**Mutation battery, sacrificial worktree, restored between each:**

| Mutation                             | Kills  | Reading                                    |
| ------------------------------------ | ------ | ------------------------------------------ |
| P1 ST `error` swallowed to a no-op   | 1, 2   | the refusal itself is pinned               |
| P2 WP `error` swallowed              | 3, 4   | same, other creator                        |
| P3 ST leaves its directory behind    | 2 only | the leaves-nothing claim is pinned         |
| P4 WP leaves its directory behind    | 4 only | same, other creator                        |
| P5 the path dropped from the message | 1 only | naming the path is pinned, not incidental  |
| P6 a `TEMPLATE` heredoc creeps back  | 4, 7   | the mechanical anti-regression guard bites |

P6 is the one that matters for keeping this fixed: it greps both creators rather than watching the one branch that was edited.

**Collateral:** `no_template_fallback` (7), `objective_placeholder` (12, after inversion), `st_commands` (54), `wp_commands` (30), `st_zero_commands` (33), `credo_checks_residue`, `st_list_all_vocabulary`, `st_enumeration`, `at_grammar_lint`, `output_width`, `helpers`, `docs_completeness`, `modules_commands`, and both integration decks -- all green. `intent critic shell` clean on both changed files.

### Noted, not fixed

`error()` in `bin/intent_helpers:7` prints `Error: ` with a capital, while the documented CLI voice is the lowercase Rust-style family (`ok:`, `created:`, and `warning:` since `8aba5ab`). Same class as the `warning()` correction in this release, but it is a wider blast radius -- tests across the suite assert on the current string -- so it is left for its own issue rather than folded into a pre-cut batch.
