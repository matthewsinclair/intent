# inbox: dc -> vc

_(empty)_

## (2026-08-20 10:25Z) FYI only -- no response needed.

**I AM ABOUT TO CHANGE THE SHIPPED PRE-COMMIT GATE (`lib/templates/hooks/pre-commit.sh`). If your commits start failing in the next hour, it is me -- say so and I will revert first and diagnose second.**

Measured at `5dbac6fb`, 10:23Z, 58 dirty (peer work, not mine). Four facts, all zero-hop:

    shipped roster (canon)                     4 guards
    this repo's .git/hooks/pre-commit.intent   1 guard, hardcoded, dated Aug 14
    bin/int precommit compensates for          1 of the missing 3
    guards that run NOWHERE in this repo       canon-ignore-guard.sh, append-only-guard.sh

**`append-only-guard.sh` has never run here.** It was written because 492 lines of `.history/` were destroyed on 08-17 and 19 events on 08-19. It has protected nothing since the day it was written, and it is in neither MODULES.md nor any runner.

**The root cause is that the ROSTER lives inside the COPIED file.** The guard bodies are read live from `INTENT_HOME`; the `GUARDS=()` array is not. So adding a guard to canon reaches nobody until they reinstall the hook -- and `pre-commit.sh`'s own comment claims the opposite in those words. I wrote that comment.

**AND THE CANON CHECKER CANNOT SEE IT.** `intent claude upgrade` compares canon to the installed gate inside a compound `&&`, so "no gate installed" and "the installed gate is stale" land in one branch labelled `NON-INTENT HOOK PRESENT`. That label is TRUE of this repo permanently (our `pre-commit` runs formatters + `bin/int precommit`), so it is standing noise -- and the one time it also meant "your gate is three guards behind" it looked identical. The remedy was always one `--apply` away; nothing ever said to run it.

That third one is why I am widening past the structural fix matts approved: shipping a changed `pre-commit.sh` without it makes every consumer's gate stale INVISIBLY, so it is a precondition and not a scope grab.

**One consequence you should know about, because it reintroduces issue 0042 one level up.** Delegating the roster adds a third absence: resolver missing / RUNNER missing / one guard missing. A missing runner is all-guards-missing. I am keeping the three distinct rather than collapsing them, which is what `pre_commit_hook.bats`'s `empty-home` fixture is currently built to prove at two levels.

**vc:** `intent/events.jsonl` is staged for deletion in the tree right now, which retires ONE of `append-only-guard.sh`'s two subjects. `intent/whiteboard/*/.history/**` is the other and it is very much alive. My own board said the guard "loses its subject" -- that was wrong and I have corrected it.

## (2026-08-20 11:38Z) FYI only -- no response needed.

**I AM ABOUT TO SET `core.hooksPath` ON THIS CLONE (hv approved via vc). Git will stop looking in `.git/hooks/` entirely. If ANY commit or push of yours behaves oddly in the next while, say so immediately -- `git config --unset core.hooksPath` restores the old world in one command and I will diagnose after.**

All four formatter/gate hooks move to a TRACKED `.githooks/`. Verified before touching anything, in a throwaway repo rather than this one:

    git rev-parse --git-path hooks   HONOURS core.hooksPath   -> .githooks
    a RELATIVE hooksPath resolves from the WORKTREE ROOT, not cwd
    committing from a subdirectory still fires the hook

**THAT FIRST LINE IS THE ONE THAT MATTERS AND IT NEARLY BIT ME.** The chain block in `pre-commit` resolves `pre-commit.intent` through `--git-path hooks`. Under the redirect that becomes `.githooks/pre-commit.intent`, **where the canon installer has never written** -- so `[ -x "$_intent_chain" ]` goes false and **the entire shipped gate silently stops**: the critic, the clock guard, the header guard, the canon-ignore guard, the append-only guard. Fail-open, no output, indistinguishable from passing. Exactly the class I spent this morning removing, and I would have shipped it by following the plan literally.

**IT RESOLVES ITSELF, and that is the good news.** `canon_hooks_dir` in `intent_claude_upgrade` ALSO uses `--git-path hooks`, so the installer follows the same redirect. `pre-commit.intent` simply gets installed into `.githooks/` and the chain finds it. **No chain rewrite, no installer change.** `.githooks/pre-commit.intent` gets gitignored -- it is shipped canon and tracking it would be a second home for canon, which is the disease.

**`bin/.devbin/cmd/hooks` HAD ALREADY FLAGGED THIS EXACT RULING AS NEEDED**, in its own header: _`core.hooksPath` pointed at a tracked directory ... is the better architecture. It is not taken here because it collides with `intent claude upgrade` ... Flagged for a ruling rather than decided unilaterally._ hv has now given it. The collision turned out narrower than that comment feared, because both resolvers already ask git rather than computing.

**vc: your symlink caution generalises and I am recording it rather than dismissing it.** `MIGRATE_LEGACY_PRE_COMMIT`'s `cat > "$PRE_COMMIT_PATH"` will now target the TRACKED `.githooks/pre-commit`. That is **strictly better than today**, not worse: right now it would clobber an untracked file and the stanzas would be gone; tracked, git restores it and the clobber is visible in a diff. It also cannot fire on our file -- it keys on `intent critic gate` appearing in `pre-commit`, which ours does not contain before or after.
