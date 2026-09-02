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

## (2026-08-29 18:45Z) FYI only -- no response needed.

**ANNOUNCE -- I AM REFRESHING THE MACHINE STORE, AND I HAVE 0144.** Both on hv's word, first-hand in my session at 18:4xZ: I put two questions and hv answered _"1: Do it / 2: Yes"_.

**THE STORE REFRESH.** `intent doctor` reports `intent/.cache/intent.db` **model-inconsistent** -- the runtime store does not match a rebuild from committed canon, so read verbs answer stale. Cause is almost certainly `6f2a4610` adding `0144.json` to canon with no resync. **I am running `intent sync --to-store` and NOT bare `sync`**: bare syncs both directions and would additionally write views to disk, which is a wider act than the finding asks for and collides with a live peer. `intent/.cache/` is gitignored, so this changes no tracked byte.

**WHILE IT RUNS, ANY FIGURE READ THROUGH A READ VERB CHANGES UNDER YOU.** vc: you are active and your board already quotes `ST0068 4/9, ST0056 69/133` -- **those came from the stale store, as did mine.** Re-read them after my close notice rather than carrying them. **I will announce the END as well as the opening** -- ic's 2026-08-29 half-hour hold came from an announcement with no retraction, and that is the half that gets forgotten.

**AND I HAVE 0144, SO WE DO NOT BOTH TAKE IT.** vc: your bounce focus names it and it is routed to cc in `intent/wip.md`, so I read yours as follow-up rather than as a claim -- **but I am saying so rather than assuming it.** hv confirmed the ordering explicitly: 0144 BEFORE the `info.md` round-trip. If you were about to start it, say so and I will stand down.

**ONE CORRECTION AGAINST MYSELF, since it was load-bearing for a day.** My 17:58Z fold said `native/rust` was held at 0 dirty for vc's rebuild. **ic recorded that window closing at 17:31Z -- 27 minutes BEFORE I wrote it.** The hold was already lifted when I asserted it. Pair measured current at boot; `intent` names `f4a2271f` and `git diff --name-only f4a2271f..HEAD -- native/rust surface` is empty. **I could not check it as a SET: `intentd --version` prints no commit**, so only the commit gate compares the two markers.

## (2026-09-02 13:56Z) THREE OF MY OWN ENTRIES ABOVE ARE DEAD -- DISCHARGE NOTICE, NOTHING NEEDED FROM YOU

**APPENDED, NOT EDITED IN PLACE.** The entries above stay exactly as delivered, wrong premises included, because a rewritten escalation loses the evidence of what someone was told at the time. Your ruling via vc, and the shape is vc's.

**I re-drove all five of my entries before routing anything to you today. Measurement killed three, and one of those was my largest.**

**[1] `sync` SKIPPING UNTRACKED BYTES (2026-08-25) -- LANDED, AND MY FRAMING WAS WRONG RATHER THAN MERELY STALE.** I put it to you as a scope question with **no cheap middle**: either build the staged-but-untracked vs untracked-and-unstaged distinction, or drop the skip from 3.0.0. **The code took a third option I said did not exist.** `sync.rs:506`, in its own words: _it keys on IGNORED, never on untracked. A `thread.json` you just created and have not committed must still ingest -- that is what most of a working session looks like._ Keying on ignored **dissolves** the discriminator instead of implementing it, so the silent no-op I asked you to rule on has no subject. **You were asked to choose between two options when a better third existed. That is on me.**

**[5] `~/.intent/home` POINTING AT A DELETED TEMP WORKTREE (2026-08-27) -- GONE, AND NOBODY TOLD ME.** This was the largest thing I ever escalated to you: the pointer named a vanished scratchpad worktree under a peer's session, and I argued **the 17-estate sweep could not run until it was sound**. Driven today: the pointer resolves to `/Users/matts/Devel/prj/Intent`, the directory exists, and the shim answers `state: OK`, rc=0. **Had I not re-driven it, I would have handed you a dead blocker with a sweep still parked behind it.**

**[4, SECOND HALF] `autotests = false` SET EXPLICITLY -- RULED AND EXECUTED.** All three crates carry it (`intent-cli`, `intentd`, `intentsvcs`). Nothing outstanding.

**WHAT IS STILL LIVE FROM THOSE ENTRIES:** the AT row-keying convention (**you have now ruled it: cite the TEST NAME**, and I am executing) and `Node`'s model / WP-14, whose **safety claim I have retracted** -- I told you the deferral was safe because `AC-08.5` pinned the partition sizes in both directions, and it does not; `AC-08.5` is the daemon-watch criterion. vc drove it independently and found no pin anywhere in ST0056. **The property that made that deferral ignorable is one I can no longer stand behind**, which is why it is back in front of you rather than sitting quietly on my board.

-- cc

## (2026-09-02 17:44Z) NINE ITEMS THAT ARE YOURS TO RULE, CONSOLIDATED AT THE END OF A BUILD DAY

**APPENDED, NOT EDITED IN PLACE**, as before. **Routed live to vc the same minute**, because a write here is the durable half and never the delivery. Nothing below is a request to act tonight; it is the set of things I am NOT free to decide, gathered so you can see them together rather than as they surface.

**Four landed today and none of them is on this list** -- `0206`'s mutation half, both schema refusals naming their artefact, `0196`'s cross-repo half, and issue `0214`. Those needed no ruling.

### A. BLOCKING SOMETHING, WITH A NAMED COST

**[1] `AC-06.1`'s COVERAGE HALF NEEDS `burn.sh` RE-RUN, AND I DO NOT LAUNCH IT.** `coverage_map.sh` refuses to publish and is RIGHT to: the burn TSV no longer covers the estate. The remedy is a DOUBLE full-estate sweep that hung for 3.5 hours once. **Full-suite runs are yours, so this one cannot move without you.**

**[2] THE `INTENT_BIN` FLIP MUST HAPPEN BEFORE THAT RE-BASELINE OR THE WALL TIME IS PAID TWICE.** The estate's default `INTENT_BIN` is `bin/intent`, **the v2 SHELL SCRIPT**. It is THREE binaries, not two: `test_helper.bash:21` -> v2 shell, `run_v2_suite.bash:55` -> v3 DEBUG, `~/.local/bin/intent` -> v3 RELEASE. An undeclared population of build targets. **Flip THEN re-baseline. Sequencing is the whole of the decision and it is not mine to take.**

**[3] WP-08 IS 12/12 GREEN AND `wp done` IS AN XS AWAY. I AM NOT TAKING IT.** Closing it marks the daemon DONE while its family carries zero conformance coverage -- a green over an uncovered surface. vc endorses the hold. **What it is waiting for is your word, not more work.**

### B. A CONTRADICTION BETWEEN A DOCUMENT AND THE CODE

**[4] THE MIGRATOR DOES NOT COMMIT, SO THE DOCUMENTED ROLLBACK HAS NO SUBJECT.** `migration.md` Phase B step 7 says _one commit_ and `AC-00.8` says _one visible commit_. Measured: `intent upgrade` returns rc=0 with HEAD unchanged and 23 paths dirty. **_One visible commit_ is an OPERATOR CONVENTION, not a migrator guarantee.** Either the migrator commits or the doc and the row stop saying it does. **Both are defensible and I will not pick.**

### C. DISPOSITIONS -- ISSUES NOBODY HAS RULED IN OR OUT

**[5] `0086` AND `0063` WOULD HAVE TO BE WRITTEN, NOT RECOVERED**, and `0095` / `0096` are one-word stubs. Four rows that cost real work or should be closed as never-specified.

**[6] `0192` -- AND IF YOU RULE IT IN, THE REFUSAL GOES IN `info_read_back`, NOT `authored_regions`.** The placement is decided; only the in-or-out is not.

**[7] THE DAEMON-LOCK RACE HAS NO ISSUE AT ALL.** It was found mid-hold and never filed. **It wants a record whatever you decide about fixing it**, and that part I can do on one word.

**[8] `0205`'s FOURTH BLOCK IS VENDORED.** `bin/.devbin/lib/builtins:66` sits under an integrity manifest, so it goes UPSTREAM or is ACCEPTED. The other three are ic's and are ordinary.

**[9] `0214`, FILED TODAY, NEEDS A CUT DECISION.** `form::Field::editable`'s doc names a guard -- `offers_an_edit_the_surface_refuses` -- that appears exactly once in the tree, in that comment. **It is on `AC-17.1`'s path rather than beside it:** two realisers agreeing about which fields are editable is a precondition for _the same edit reaches an identical store state_. Small to build. In or out of 3.0.1.

### AND ONE THING I AM NOT ASKING YOU TO RULE, RECORDED BECAUSE IT REACHED ANOTHER PROJECT

**`0196` NOW CARRIES A CROSS-REPOSITORY DATA-INTEGRITY HALF**, measured by lamplight-cc from outside this tree while a release build ran here: six `intent` verbs in one script, **verb 1 wrote, verbs 2-6 died on `command not found`, and `set -e` did not catch it** -- leaving a half-written acceptance contract in Lamplight's git-tracked canon with no exit status describing it. The remedy is recorded (build to a staging target and `mv` in; a rename is atomic). **`bin/.devbin` is dc's, so I have filed the evidence and taken nothing.** Flagging it here only because its blast radius is every project on this machine and the next one caught by it may be mid-ingest across hundreds of threads rather than mid-six-verb-script.

-- cc
