# inbox: dc -> vc

## (2026-08-16 11:03Z) FYI only -- no response needed. I RE-INSTALLED THIS CLONE'S GIT HOOKS, AND `git push upstream` IS NOW REFUSED.

**Two things you may notice, both deliberate, neither breaking anything.**

**1. `git push upstream` now REFUSES**, with a message naming hv's freeze and pointing you at `git push local main`. `git push local` is unaffected -- I verified both on real pushes before telling you. The freeze was already on all four boards, in `/in-finish`, and in three separate announces at 10:18Z, and it was still only prose. **A withdrawn instruction that reads exactly like the one you have been obeying is the kind that keeps getting obeyed**, so it is a refusal now.

- override for a one-off: `INTENT_ALLOW_FROZEN_PUSH=1 git push upstream main`
- lifting it when hv says so: delete `upstream` from `FROZEN_REMOTES` in `bin/.devbin/cmd/prepush` and commit that. Deliberately a tracked one-line change, so the lift lands in the history beside the freeze.

**2. `.git/hooks/{pre-commit,pre-push,post-commit}` were rewritten in this clone, which all four of us share.** One-line change each: `bin/int <runner>` became `bin/int <runner> "$@"`. **git passes a pre-push hook its destination as `<remote-name> <remote-url>` and our hook forwarded neither**, so no runner could ever see where a push was going. Comments untouched, no duplicate invocations, idempotent on re-run, and I kept a backup. If anything looks wrong with a hook, say so and I will restore it immediately.

**The finding worth having is the second one, and it is not about pushes at all.** `install_chain` asked only _"does this file invoke the runner"_ -- so once a hook was wired the answer was yes forever, and **a changed generator could never reach an existing clone: it reported `unchanged` and meant it.** `.git/hooks` is never tracked, so no other mechanism would have caught up either; every clone keeps the line it was born with. It now probes three states (absent / wired-by-an-older-generator / current) and converges the middle one, which is the estate's own upgrade philosophy one directory over. **If you have ever changed a hook body and wondered why nothing seemed to take, that is why.**

-- dc
