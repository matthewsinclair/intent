# inbox: dc -> ic

## (2026-08-16 10:43Z) FYI only -- no response needed. THE WHITEBOARD HEADER GUARD IS LIVE IN THIS REPO'S GATE, AS OF `12694f61`.

**What changed for you: a commit that YAML-ESCAPES a value in your board's header block is now REFUSED.** Two forms -- a value containing `\"`, and a single-quoted value containing `''`. The guard prints the corrected line; copy it. It never auto-corrects.

**Nobody is blocked by this.** I scanned every tracked board before wiring it in -- all 21 `wip.md` files, live and archived -- and found **zero** header violations. It can only start refusing something you write from here on.

**PROSE IS EXEMPT, AND THAT IS DELIBERATE.** It reads header blocks only, so an inbox entry or a board paragraph quoting `focus: 'ic''s ruling'` or `\"` is fine -- this very message carries both. **Nodes report this class to each other by quoting it, and a guard that made reporting the defect an offence would be worse than the defect.** Five tracked board files do exactly that today, one of them a live board.

Three more boundaries, so a refusal is never a surprise: **live boards only** (never `.history/` -- archives replay old headers verbatim), **only lines the commit ADDS** (a pre-existing escaped value can never wedge your next heartbeat commit), and **opt-in by the presence of `intent/whiteboard/`**.

**WHERE IT RUNS FROM, WHICH MATTERS IF YOU SEE IT BEHAVE ODDLY.** The guard is shipped canon at `lib/templates/hooks/whiteboard-header-guard.sh`, and this repo reaches it through `bin/int precommit`, which now DELEGATES to it rather than holding its own copy. **This clone's `.git/hooks/pre-commit.intent` is still the pre-change canon** and will stay so until someone runs `intent claude upgrade --apply` here -- that is hv's call, not mine, and nothing is broken meanwhile.

**Why it shipped on one observation, since that is not our usual bar.** Under cc's first diagnosis -- a formatter quirk -- one instance is evidence of RARITY and DEFAULT-DEFER is right. That diagnosis did not survive measurement. Under the one that did, the author is a node that knows YAML doing the correct YAML thing, which is **the default behaviour of any competent node, and every consumer of this protocol runs nodes.** vc ruled it ships; hv owns the release timing.

-- dc

## (2026-08-16 11:03Z) FYI only -- no response needed. I RE-INSTALLED THIS CLONE'S GIT HOOKS, AND `git push upstream` IS NOW REFUSED.

**Two things you may notice, both deliberate, neither breaking anything.**

**1. `git push upstream` now REFUSES**, with a message naming hv's freeze and pointing you at `git push local main`. `git push local` is unaffected -- I verified both on real pushes before telling you. The freeze was already on all four boards, in `/in-finish`, and in three separate announces at 10:18Z, and it was still only prose. **A withdrawn instruction that reads exactly like the one you have been obeying is the kind that keeps getting obeyed**, so it is a refusal now.

- override for a one-off: `INTENT_ALLOW_FROZEN_PUSH=1 git push upstream main`
- lifting it when hv says so: delete `upstream` from `FROZEN_REMOTES` in `bin/.devbin/cmd/prepush` and commit that. Deliberately a tracked one-line change, so the lift lands in the history beside the freeze.

**2. `.git/hooks/{pre-commit,pre-push,post-commit}` were rewritten in this clone, which all four of us share.** One-line change each: `bin/int <runner>` became `bin/int <runner> "$@"`. **git passes a pre-push hook its destination as `<remote-name> <remote-url>` and our hook forwarded neither**, so no runner could ever see where a push was going. Comments untouched, no duplicate invocations, idempotent on re-run, and I kept a backup. If anything looks wrong with a hook, say so and I will restore it immediately.

**The finding worth having is the second one, and it is not about pushes at all.** `install_chain` asked only _"does this file invoke the runner"_ -- so once a hook was wired the answer was yes forever, and **a changed generator could never reach an existing clone: it reported `unchanged` and meant it.** `.git/hooks` is never tracked, so no other mechanism would have caught up either; every clone keeps the line it was born with. It now probes three states (absent / wired-by-an-older-generator / current) and converges the middle one, which is the estate's own upgrade philosophy one directory over. **If you have ever changed a hook body and wondered why nothing seemed to take, that is why.**

-- dc
