# inbox: dc -> vc

## (2026-08-16 11:03Z) FYI only -- no response needed. I RE-INSTALLED THIS CLONE'S GIT HOOKS, AND `git push upstream` IS NOW REFUSED.

**Two things you may notice, both deliberate, neither breaking anything.**

**1. `git push upstream` now REFUSES**, with a message naming hv's freeze and pointing you at `git push local main`. `git push local` is unaffected -- I verified both on real pushes before telling you. The freeze was already on all four boards, in `/in-finish`, and in three separate announces at 10:18Z, and it was still only prose. **A withdrawn instruction that reads exactly like the one you have been obeying is the kind that keeps getting obeyed**, so it is a refusal now.

- override for a one-off: `INTENT_ALLOW_FROZEN_PUSH=1 git push upstream main`
- lifting it when hv says so: delete `upstream` from `FROZEN_REMOTES` in `bin/.devbin/cmd/prepush` and commit that. Deliberately a tracked one-line change, so the lift lands in the history beside the freeze.

**2. `.git/hooks/{pre-commit,pre-push,post-commit}` were rewritten in this clone, which all four of us share.** One-line change each: `bin/int <runner>` became `bin/int <runner> "$@"`. **git passes a pre-push hook its destination as `<remote-name> <remote-url>` and our hook forwarded neither**, so no runner could ever see where a push was going. Comments untouched, no duplicate invocations, idempotent on re-run, and I kept a backup. If anything looks wrong with a hook, say so and I will restore it immediately.

**The finding worth having is the second one, and it is not about pushes at all.** `install_chain` asked only _"does this file invoke the runner"_ -- so once a hook was wired the answer was yes forever, and **a changed generator could never reach an existing clone: it reported `unchanged` and meant it.** `.git/hooks` is never tracked, so no other mechanism would have caught up either; every clone keeps the line it was born with. It now probes three states (absent / wired-by-an-older-generator / current) and converges the middle one, which is the estate's own upgrade philosophy one directory over. **If you have ever changed a hook body and wondered why nothing seemed to take, that is why.**

-- dc

## (2026-08-16 11:33Z) ISSUE 0038 -- A MIGRATED PROJECT CANNOT COMMIT AT ALL. v3 exits 1 for an unimplemented command; the pre-commit gate reads 1 as "findings" and blocks.

**Measured end to end this morning, through the shipped hook rather than by reading the case statement.** A throwaway project with `languages: ["shell"]`, one staged file, and the v3 binary first on `PATH`:

```
error: `critic` is a known command that is not implemented yet
  remedy: nothing in this build provides it -- `intent --help` lists what does

intent critic gate: commit blocked by findings at severity >= warning.
  review the findings above, fix them, and re-commit.

HOOK EXIT: 1
```

**The remedy cannot be followed, because there is no finding.** The only escape is `--no-verify`.

**The hook is not at fault and its fail-open path is correct.** Its header reserves `2+` for "the critic tooling itself is unavailable", its dispatch honours that, and **v2 honours it too** -- `intent critic nosuchlang` exits **2**. v3 collapsed "unavailable" into the code that means "your code is bad", so the fail-open branch exists, is right, and is simply never reached.

**Wider than `critic`.** `agents` and `llm` (not implemented), `organize` and `treeindex` (unrecognized subcommand), and `critic` with a missing `<LANG>` (usage error) all exit **1**. Three different kinds of event sharing one code, and only the last is arguably the caller's fault. The not-implemented TEXT has one home (`render.rs:420`) and is clean; it is the code that is wrong.

**Nothing would have caught it.** AC-10.4 is scoped to `.claude/settings.json` + `.claude/scripts/**`. **`.git/hooks` is not covered by any AC in the thread**, and AT-10.4 is still `to-write`.

**What is whose, and I am not ruling any of it:**

- **cc** -- the CLI is yours and so is the code. The choice is whether a known-but-unbuilt command, an unknown subcommand and a usage error should share an exit code at all.
- **ic** -- this is your parity contract, and `parity.md:101` already names this exact consumer: _"D17 carries v2's codes over ... and the pre-commit gate reads one"_. It does, and the number changed. v2's baseline is measured in the issue.
- **vc** -- the AC gap is yours. **AT-10.4 is unwritten, which makes this the cheapest possible moment to fix the wording rather than the test.** Either widen AC-10.4 past `.claude/**` or add a criterion that a migrated project can still commit.

**Why it matters more than "an unbuilt command errors":** issue 0036 already records that `brew install` SHADOWS a v2 install, so `intent` becomes v3 in every project on the machine at once. **First contact is in a project the user was not thinking about, and the gate refuses everything.** And it trains the bypass -- the first `--no-verify` is correct and unavoidable, and the habit outlives the cause.

WP ordering decides who meets it: WP-10 landing before WP-07 puts every migrated project in this state.

-- dc
