# inbox: cc -> hv

## (2026-08-25 22:42Z) NEEDS A RULING BEFORE THE CUT -- `sync` skipping untracked bytes

**HELD ON YOU AND NOT BUILT. I am not asking to build it; I am asking whether it is IN or OUT of v3.0.0.**

**dc's caution is the sharp part, and it is what makes this a ruling rather than a task:** the skip must tell STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED. **If it cannot, a legitimate two-step workflow becomes a SILENT NO-OP** -- the user stages, syncs, and the sync reports success over bytes it never read. **That is this estate's own class arriving in the product: a mechanism upstream of the subject answering in the subject's voice.**

**Why it is escalated rather than decided by me: the two readings are not a design choice, they are a scope question.** IN means building the distinction, because shipping the skip without it ships the silent no-op. OUT means the skip does not land at all in v3.0.0. **There is no cheap middle**, and picking one on your behalf would be picking the release's scope on your behalf.

## (2026-08-25 22:42Z) FYI ONLY -- NO RESPONSE NEEDED -- `Node`'s model (ST0056/WP-14)

**Held with you, and I want it on record that THE DEFERRAL IS SAFE BY CONSTRUCTION -- so this does NOT need a decision before the cut.**

**The AC-08.5 partition sizes are pinned in BOTH directions.** A later reification therefore REDS the cover rather than letting the row go quietly stale: **the failure mode of deferring this is a red row, not a silent drift.** That is the only reason I am comfortable holding it, and it is the reason you can ignore this entry tonight.

**Recording it because it lived on my board and nowhere else.** If the pinning ever comes out of AC-08.5 this stops being safe and becomes a ruling; nothing currently proposes that.

## (2026-08-25 22:42Z) FYI ONLY -- NO RESPONSE NEEDED -- why these are the first three entries you have ever had from me

**`hv/inbox.cc.md` was the `_(empty)_` sentinel until this commit -- not for tonight, for the whole project.** Driven at 22:38Z: `vc` 22 entries, `dc` 2, `ic` 2 (both last written 2026-08-21), **`cc` zero.**

**The two items above are things I have been holding YOU on while keeping the only copy on my own board.** You reached them tonight by reading that board -- which is also how you reached a claim on it that had expired twenty minutes after I wrote it.

**vc has the sharpest version and it cuts against their own 22: writing is not delivery.** A full inbox and an empty one are **both** consistent with you not having what you need. **I am not claiming this fixes that** -- only that state which existed in exactly one place now exists in two, and one of them is a surface with a named reader.

## (2026-08-27 11:58Z) TEST-TARGET CONSOLIDATION -- HOLDING FOR YOUR WORD AT SOURCE, AND ONE MEASUREMENT THAT CHANGES THE PRICE

**I have your ruling only via vc, so I have not started.** vc relayed it correctly and told me not to start on a relay, which is right. This is me asking you to confirm directly, plus the one thing I found that you should have before it stands.

**NOT RE-LITIGATING THE 2.8 PER CENT.** vc was explicit that you ruled with the number in front of you -- consistency bought deliberately, not a disk fix. I have no argument with that and I am not reopening it.

**THE MEASUREMENT: THE SCARIEST COST IS NEAR-ZERO FOR INTENT, AND IT IS THE ONE THAT MADE THIS LOOK RISKY.** vc flagged that separate binaries are separate PROCESSES and merged they become threads in one, so anything touching env, cwd or a store handle breaks as FLAKINESS rather than as a clean failure. That was my own finding and it was the right thing to fear. Driven, not reasoned:

    env::set_var          0 occurrences in the entire workspace
    env::remove_var       0
    env::set_current_dir  0
    Fixture               tempfile::tempdir(), unique per instance
    INTENT_HOME / cwd     40 sites, ALL of them cmd.env(..) or .current_dir(..) on a CHILD Command

**Every one of those is per-child-process, so it cannot leak between threads.** And cargo's harness ALREADY runs tests as threads inside each binary -- 127 binaries today means 127 thread pools, not 127 sequential runs. **Merging does not introduce a new execution model; it enlarges one that is already there.**

So the risk I would have written a red arm for turns out to have no subject here. I will still write the arm, because a property nothing enforces expires silently, but it guards future code rather than rescuing present code.

**THE REAL COST, STATED HONESTLY, IS THE CANON RE-CITE AND IT IS ATOMIC.** 168 of 334 AT rows cite a `tests/*.rs` path. `at lint` L2 refuses a row citing a file that does not exist and L3 refuses a file not carrying the row's literal id, so **every stale row fails from the moment the files merge until the last row is re-cited**. There is no partial landing and no bisectable middle. The ids survive -- they live in function names -- so it is mechanical, but it is 168 rows of canon in one commit.

**TWO THINGS I WOULD LIKE RULED, BOTH SMALL:**

1. **vc's convention -- AT rows cite the TEST NAME, not the file path.** vc has called this and I think it is right: name-keyed survives consolidation permanently, path-keyed turns every future build-layout change into a contract change. It is ~210 rows estate-wide across three projects that disagree today, which is the divergence you are paying to remove. I will follow vc unless you say otherwise; flagging it only because it is a canon convention rather than a build detail.

2. **`autotests = false` set EXPLICITLY.** The single-file-per-target convention is a SILENT DEFAULT and it is what produced 167 targets with nobody deciding. Without the flag a future `tests/quick.rs` becomes target 2 and nothing reports it -- the consistency you are buying decays with no one noticing. Cheap, and it makes the ruling self-enforcing.

**SEQUENCE IS YOURS.** I am holding a migration queue from vc -- hop 2 refusing on shortfall, one `.canon` emitter, `ST0057/13` bucket ingest verified per file by content, then the prune. Consolidation collides with none of it. Say which goes first.

Nothing is burning and I am not blocked on this: my lanes are clean and I am holding for the machine-health stop to lift regardless.

-- cc

## (2026-08-27 21:35Z) ITEM (B) IS LANDED -- AND THE CHECK OF WHETHER IT WAS SAFE TO RUN FOUND THE THING THAT MUST STOP THE SWEEP

**(B) is `f8a78e05`.** `canon::apply` now installs the shim as `<hooks>/pre-commit.intent`, beside the chain block that sources it. Ruled by you, relayed by vc, confirmed with you at source before I started.

**THE DEFECT WAS SHARPER THAN "A CAPABILITY IS MISSING", AND THAT IS THE PART WORTH KEEPING.** The chain block canon has always written is `if [ -x "$_intent_chain" ]; then ... fi` -- with no `else`. Nothing in either tree wrote the carrier. **So the one verb whose job is wiring the gate produced, unaided, a project that passes every commit at rc=0 while every report anyone reads says the gate is wired.** The shim's own contract is _refusing, not skipping_ (your ruling 4); that contract was unreachable from one layer up, because a shim never installed is not a shim that refuses, it is a `[ -x ]` that is false. Baize is the measured instance.

**I DID NOT MINT A VERB.** vc framed (B) as a missing verb. The capability was missing; the verb was not. `apply` already resolved the install root, already read templates through the reader that IS vc's stated first correctness property, already resolved the hooks directory, and already reported written/unchanged/preserved/held. Minting surface here would repeat exactly what you rejected at 17:11Z for `publish_home`'s caller. **If you meant a literal new verb, say so and I will move it** -- the code is the same either way.

**NOW THE THING THAT IS NOT MINE AND IS BIGGER THAN (B). `~/.intent/home` ON THIS MACHINE POINTS AT A DELETED TEMP WORKTREE IN A PEER'S SCRATCHPAD.** Measured 21:32Z: the pointer names a `scratchpad/wt-pair` path under session `6bbf2186-...` (ic's), `ls -d` says No such file or directory, and `pre-commit-shim.sh --where` answers `state: UNUSABLE`, rc=1.

**IT IS INERT TODAY AND (B) IS PRECISELY WHAT ARMS IT.** No estate has the shim as its carrier yet -- this tree's is the old copied gate body, 21 Aug, divergent from both templates -- so nothing consults the pointer and every commit works. **The first `intent claude upgrade --apply` anywhere installs the shim, the shim reads that pointer, and it refuses. Correctly. Every commit in that estate then blocks, and the verb that did it printed `written`.**

**NEITHER HALF IS BUGGY, WHICH IS WHY IT NEEDS YOU RATHER THAN A FIX.** `bootstrap` resolves the root from `current_exe()`, so a binary run out of a scratchpad worktree publishes that worktree -- and `publish_home` had nothing to refuse, because at that moment it genuinely WAS an install. It stopped being one when someone tidied up. The shim never auto-repairs a stale pointer, deliberately. **So a correct pointer becomes a wrong pointer through a third party's cleanup, and nothing anywhere notices until the gate refuses.** The 17-estate sweep cannot run before that is sound.

**I HAVE NOT TOUCHED `~/.intent/home`.** It is machine-global, it points into another node's session, and rewriting it would be choosing an install root on your behalf -- the one act the shim's design refuses. vc has it for sequencing; ic has the mechanism; dc has the consequence for `gate_currency`, which will call a correctly-installed shim `dispatcher STALE` because it compares against the gate body.

**ONE ASSUMPTION I MADE RATHER THAN BLOCKING ON, STATED SO YOU CAN REVERSE IT.** vc named the tracked-hooks question and deliberately left it unruled. I found the estate had already answered it -- `.gitignore:156` ignores `.githooks/pre-commit.intent` while `.githooks/` itself is tracked, with the reason in MODULES.md: tracking it would be a second home for canon. **So my next commit REPORTS when the carrier lands somewhere tracked rather than refusing to write it.** Refusing would leave Laksa with no gate at all, which is strictly worse than today, and reporting is the reversible choice. Say the word and I will invert it.
## (2026-08-29 18:45Z)   FYI only -- no response needed.

**ANNOUNCE -- I AM REFRESHING THE MACHINE STORE, AND I HAVE 0144.** Both on hv's word, first-hand in my session at 18:4xZ: I put two questions and hv answered _"1: Do it / 2: Yes"_.

**THE STORE REFRESH.** `intent doctor` reports `intent/.cache/intent.db` **model-inconsistent** -- the runtime store does not match a rebuild from committed canon, so read verbs answer stale. Cause is almost certainly `6f2a4610` adding `0144.json` to canon with no resync. **I am running `intent sync --to-store` and NOT bare `sync`**: bare syncs both directions and would additionally write views to disk, which is a wider act than the finding asks for and collides with a live peer. `intent/.cache/` is gitignored, so this changes no tracked byte.

**WHILE IT RUNS, ANY FIGURE READ THROUGH A READ VERB CHANGES UNDER YOU.** vc: you are active and your board already quotes `ST0068 4/9, ST0056 69/133` -- **those came from the stale store, as did mine.** Re-read them after my close notice rather than carrying them. **I will announce the END as well as the opening** -- ic's 2026-08-29 half-hour hold came from an announcement with no retraction, and that is the half that gets forgotten.

**AND I HAVE 0144, SO WE DO NOT BOTH TAKE IT.** vc: your bounce focus names it and it is routed to cc in `intent/wip.md`, so I read yours as follow-up rather than as a claim -- **but I am saying so rather than assuming it.** hv confirmed the ordering explicitly: 0144 BEFORE the `info.md` round-trip. If you were about to start it, say so and I will stand down.

**ONE CORRECTION AGAINST MYSELF, since it was load-bearing for a day.** My 17:58Z fold said `native/rust` was held at 0 dirty for vc's rebuild. **ic recorded that window closing at 17:31Z -- 27 minutes BEFORE I wrote it.** The hold was already lifted when I asserted it. Pair measured current at boot; `intent` names `f4a2271f` and `git diff --name-only f4a2271f..HEAD -- native/rust surface` is empty. **I could not check it as a SET: `intentd --version` prints no commit**, so only the commit gate compares the two markers.
