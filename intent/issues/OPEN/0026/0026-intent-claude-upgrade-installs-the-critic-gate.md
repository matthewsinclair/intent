---
id: "0026"
title: the canon installer reports a hard-coded hook path, doctor has no hook check, and hooksPath resolution is unguarded
date: 2026-08-15
reporter: matts
status: OPEN
severity: medium
---

# 0026: the canon installer reports a hard-coded hook path, doctor has no hook check, and hooksPath resolution is unguarded

## Correction notice

**The first version of this issue was WRONG in its central claim and was filed at `high` on that basis.** It asserted that `intent claude upgrade` hard-codes `.git/hooks` and therefore installs the critic gate where git never looks when `core.hooksPath` is redirected -- a false green on a security gate. **That does not happen.** cc refuted it with a reproduction; vc re-ran the reproduction rather than accepting the refutation, and it holds. The corrected finding is smaller and real: a misleading display label plus two coverage gaps. Severity dropped `high` -> `medium`. The original text is in this file's git history.

**How the false claim was produced, because it is the more useful part.** vc grepped the tracked corpus for the string `hooksPath`, found it only in `bin/.devbin/cmd/hooks` (written that morning), and concluded the mechanism was absent everywhere else. **The correct API never needs to name it** -- that is the entire point of asking git rather than composing a path. _Absence of a mechanism's NAME is not absence of the mechanism._ The grep was accurate; the inference from it was not.

## Summary

`intent claude upgrade` resolves the hook directory correctly. `canon_hooks_dir()` (`intent/plugins/claude/bin/intent_claude_upgrade:412`) resolves through `git rev-parse --git-path hooks`, and the emitted chain block does the same at run time (`:439`), so a chained install re-resolves per invocation. That API honours `core.hooksPath`, so the gate is written and chained where git actually reads.

Three defects remain around that correct core:

1. **The reported path is a hard-coded literal.** Seven `canon_print ".git/hooks/pre-commit"` call sites pass a constant string. Under a redirect the installer writes to (say) `myhooks/pre-commit` and prints `.git/hooks/pre-commit`.
2. **`bin/intent_doctor` has zero occurrences of the word "hook".** The diagnostic cannot report on the gate at all, in any state.
3. **No test covers a redirected `core.hooksPath`.** The correct resolution at `:412` is unguarded; it could be "simplified" to a literal and every test would stay green.

## Reproduction

The resolver, verified on git 2.55.0:

```
git init -q . && mkdir myhooks
git rev-parse --git-path hooks     # -> .git/hooks
git config core.hooksPath myhooks
git rev-parse --git-path hooks     # -> myhooks
# distinct pre-commit in BOTH, then commit -> "RAN: myhooks/pre-commit"
```

Resolver and run-time agree. The installer asks git the same question git asks itself.

Measured on 2026-08-15:

```
canon_print ".git/hooks/pre-commit"  (literal)   7 call sites
grep -ci hook bin/intent_doctor                  0
grep -rn core.hooksPath tests/                   no matches
```

## Root Cause

**Defect 1** is a reporting-versus-resolution split: the value used to WRITE is resolved, and the value used to REPORT is a constant beside it. Nothing keeps the two in step because they are not the same expression.

The general shape, and it inverts the one the first version of this issue asserted: **the status line reports neither where it wrote nor where git reads -- it reports a constant.** A consumer with a redirected `hooksPath` who checks the printed path finds nothing there and concludes the gate is NOT installed, when it is. The tool's output produces a wrong conclusion in the opposite direction from the original claim, which is why the original claim was plausible.

**Defects 2 and 3** are absences: a diagnostic that never looks at hooks, and correct behaviour with no test holding it in place.

## Impact

Nobody's gate is silently off. The cost is a misleading report to any consumer using `core.hooksPath`, no diagnostic coverage of the hook in any state, and a correct implementation one careless simplification away from becoming the defect this issue originally alleged -- with no test to catch it.

Exposure is unmeasured: no fleet survey for a redirected `hooksPath` has been run and this issue does not claim one. The Intent repo itself is unaffected (`core.hooksPath` unset).

## Proposed Fix

cc's, and small:

1. Pass the resolved path to `canon_print` instead of the literal, so the output names the directory git will actually read.
2. Add a bats case that sets `core.hooksPath`, runs the installer, and asserts the gate lands in the redirected directory -- guarding the behaviour that is already correct.
3. Give `intent doctor` a hook check with three states -- WIRED (present and invoking the runner), UNWIRED (present, executable, invoking something else), ABSENT -- resolved through the same git API. `bin/.devbin/cmd/hooks` is a working reference, including reading guard names from the runner rather than a roster that can rot.

Under v2 DEFAULT-DEFER this is not a show-stopper.

## Related

- ST0035 -- shipped the critic pre-commit gate (WP-06)
- **`.git/hooks` is never tracked, so a fresh clone receives every guard and nothing invoking them.** That finding is dc's, is untouched by this correction, and is separate from this issue. Whether Intent should ADOPT `core.hooksPath` at a tracked directory remains open for hv -- it would shrink the per-clone action to one config command and make hook bodies reviewable. **The objection that adopting it would orphan the installer's output is withdrawn**: the installer already asks git where to write, so it follows a redirect automatically.

## Resolutions

{{TBC}}
