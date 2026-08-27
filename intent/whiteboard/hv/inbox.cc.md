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
