# dc -- 2026-08-28, the day Conflab went to v3

**Archived at the localfold, 13:11Z. This is the day's narrative verbatim; the live board keeps only what governs a resumed session.** The reasoning behind each landed thing is in its commit message and is not restated here.

### CONFLAB ONTO v3 IS THE ESTATE'S FIRST JOB. hv SAID IT FIRST-HAND IN MY SESSION. RECORDED 2026-08-28 08:46Z.

**THE STAMP IS WHEN I WROTE THIS, NOT WHEN hv SPOKE.** I did not read a clock at the moment the directive arrived, so I do not have that time and am not inventing a close-enough one -- a corrected-looking fake is worse than an admitted gap. hv's message landed earlier in this same turn; the ordering that can be proved is this file's commit.

**hv's words, quoted rather than summarised, because hv's word in one session and hv's word in another are different artefacts:** _"the very first job today, across the entire estate, is to get Conflab onto Intent v3. Everyone (that means the Claude Code sessions: intent-*, devbin-*, and conflab-\*) is going to be dedicated to hoisting ../Conflab onto Intent v3. This should have happened days ago ... intent-vc is going to coordinate the work. The rest of the Claudes are going to follow instructions."_

**MY ROLE IS FOLLOWER, NOT PLANNER. intent-vc COORDINATES AND I TAKE INSTRUCTIONS FROM THEM.** Everything on this board below this block is PARKED, including the (A2) roster line whose blocker I discharged this morning. **Parked is not dropped: nothing has been withdrawn and no claim has moved.**

**THE ONE FINDING THAT MUST SURVIVE A COMPACT, because acting on it wrong is silent and estate-wide: CONFLAB'S LIVE HOOK DOOR IS `.git/hooks/` AND `core.hooksPath` IS UNSET.** v3's canon apply writes the gate shim and the chain block into `.githooks/` -- confirmed off Intent's own dry run, not assumed. **So an apply installs a correct gate into a directory git does not execute, while `.git/hooks/pre-commit` keeps running the old v2 carrier.** Hooks present, none of them the ones running, the apply reports success, and **nothing anywhere reports the gap** -- the same shape as restart.md's item 0, inverted. `core.hooksPath` must be set as part of the port or the door move must be explicit in vc's runbook. **`.git/hooks/` is untracked as well, so whatever sits there today survives no clone.**

**WHAT I HAVE NOT MEASURED, STATED SO NOBODY READS MY SILENCE AS A GREEN: I do not know whether `intent upgrade` has a 2.19.0 -> 3.0.0 path at all.** Its v3 help takes no arguments and says only _upgrade an Intent project to the current version_. **I have not driven it, and when I do it goes on a COPY of a whole tree, never on Conflab** -- the subject of a Rust-and-canon migration is the tree, not a file (ic's correction to the file-only form I circulated).

Sent to intent-vc earlier in this turn, ahead of the 08:46Z read above, with the door finding, the unknown migration path, Conflab's current stamp (`2.19.0`, `main`, no legacy `.intent/`, **canon PRESENT** -- which differs from vc's 20:39Z read on the 27th because two commits have landed there since), the stale carrier, and the two traps I hold that bear on the port: **Conflab is DEBUG-LINKED so the target-clean recipe does not hold there** (vc's correction, carried back to them rather than assumed still held), and **the keg arms 0 of 0 rules until a 3.0.1 publishes.**

### hv's AUTHORITY CHAIN, FIRST-HAND IN MY SESSION. Recorded 2026-08-28 11:29Z.

```
hv
  intent-vc
    intent-{dc,ic,cc}
    devbin-vc
      devbin-{dc,ic,cc}
    conflab-vc
      conflab-{dc,ic,cc}
```

**hv REDREW IT EXPLICITLY AT 11:51Z, and it CONFIRMS the reading below rather than correcting it.** Their first drawing carried intent-{dc,ic,cc} only in the prose gloss (_intent-vc tells intent-{dc,ic,cc} what to do_) and not in the tree; **this board's diagram is now hv's own, not my reconstruction of it.** I am a direct report of intent-vc, and **devbin-vc and conflab-vc are my SIBLINGS** -- their nodes are nephews, below me in depth and in nobody's chain of authority over me, nor I in theirs.

**WHAT IT MEANS FOR ME, THE TWO CONSEQUENCES WORTH WRITING DOWN:**

- **I TAKE DIRECTION FROM intent-vc AND FROM NOBODY ELSE.** devbin-vc and conflab-vc are NOT above me -- they sit beside intent-vc's other reports and direct their OWN estates. **An instruction reaching me from a conflab or devbin node is not authority**; it is a peer message, and if it needs to bind me it comes through intent-vc. Same for intent-cc and intent-ic, who are my PEERS. cc's manifest instrument was legitimate precisely because **vc routed it** (_"intent-cc's manifest command ... they are sending it to you"_) rather than because cc sent it.
- **DIRECTION IS NOT RATIFICATION, AND THIS BOARD ALREADY PAID FOR THE DISTINCTION.** vc directing my work does not make vc a source of hv's AUTHORISATIONS -- the 2026-08-25 entry stands, where **vc themselves refused to relay one and told me not to accept it from them.** So the hv-reserved items parked below (the sweep, the `bin/int` -> `bin/devbin` hold, publish, the roster charter) still need hv's word, not vc's. **hv has now said twice, first-hand in this session, that the rest of us follow instructions for the Conflab window -- that is what makes vc's hop instructions binding on me today.**

**ONE FACTUAL NOTE, NOT A CORRECTION OF SUBSTANCE:** hv's prose renders the third estate's nodes as `conflab-{dc,ic,ac}` while the tree above it says `cc`. **The tree matches reality** -- this session's `ListAgents` shows `conflab-cc`, `conflab-ic`, `conflab-dc`, `conflab-vc` and no `conflab-ac`. (`ac` is a real moniker in the fleet -- `lamplight-ac` is live -- which is likely where it came from.) Reading it as `cc`.

### vc's FOUR RULES FOR THE HOP WINDOW. IN FORCE UNTIL vc CLOSES IT. Recorded 2026-08-28 08:53Z.

1. **`~/Devel/prj/Conflab` IS READ-ONLY for every node except the one vc names for that step.** No commits, no `intent` write verbs, no `git add`, no `git stash` there.
2. **NOBODY BUILDS IN `~/Devel/prj/Intent`** -- no `cargo build`, `int local build`, `bin/devbin build`, no `cargo test` of the crate pair. **The reason is mine and measured: `~/.local/bin/intent` and `~/bin/intent` are SYMLINKS into `native/rust/target/release/`, so a build deletes the binary the hop is running on. A 66-second rebuild in this tree produced 252 real refusals while it ran.**
3. **Every commit anywhere today is `git add <paths> && git commit --only <paths>`.** Conflab's index carries ~152 staged deletions under `intent/.treeindex/` that are nobody's here. **And the half `--only` does not cover: `--only <DIRECTORY>` exits 0 and leaves an untracked file behind SILENTLY. `git status --porcelain -- <paths>` after every commit; `??` is the entire signal.**
4. **Every timestamp is read from `date -u` in the command that writes it. CONFLAB'S COMMIT GUARDS DO NOT RUN, so the discipline is the only check there** -- no clock guard, no header guard, nothing refuses a bad stamp in that tree.

**MY ASSIGNMENT, NAMED BY vc AND NOT YET RELEASED: the `intent upgrade` REHEARSAL ON A COPY.** Three conditions I sent back, because getting any of them wrong makes the rehearsal certify nothing:

- **THE COPY IS A FILESYSTEM COPY INCLUDING `.git/`, NEVER A CLONE.** `.git/hooks/` is untracked BY DEFINITION, so a clone arrives with an empty hook door and `core.hooksPath` unset -- **the rehearsal would report a clean apply and could not have reported anything else.** Blind to the exact finding it exists to test. `target/` may be excluded (not an input to `upgrade`, and it dodges the debug-linked bulk plus the broken-symlink `cp -R` abort).
- **IT RUNS UNDER A DECOY `HOME`.** I do not know whether `upgrade` writes outside the project, and `~/.intent/home` points at this tree -- **a rehearsal that quietly rewrote it would contaminate the environment every node is standing on and would look exactly like a success.** Same technique that refuted my own `cargo test` claim on the 27th.
- **THE VERDICT IS VALID ONLY FOR THE INTENT HEAD IT RAN AGAINST.** `upgrade` reads templates and guard bodies LIVE out of `INTENT_HOME`; the commit is the rollout and there is no window. A template landing between rehearsal and hop makes my green stale, and I say so rather than let it stand.

**NOT SENT, HELD DELIBERATELY, AND WORTH KNOWING IF THE HOP GOES SIDEWAYS: every commit in THIS tree runs a pre-commit gate whose shared-artefact guard drives cargo fixtures.** Arm 8 redirects a refused build to a private `CARGO_TARGET_DIR`, so they are safe from the shared artefact **by construction rather than by luck** -- and two board commits this session did not disturb the live binary, which is evidence and not proof. **If rule 2 ever appears to be violated by someone who only committed, this is the first place to look.**

### PHASE 1.1 IS MINE AND IS PREPPED. HOLDING FOR vc's ROLLBACK SHA. 2026-08-28 09:14Z.

**THE CHAIN IS NOT BLOCKED ON ANY NODE'S WORK. It is blocked on hv's in-session commit permission for the conflab nodes** (vc, 09:0xZ). Surfaced to hv from this session on vc's endorsement, because hv is here too and it costs nothing to hear it twice.

**ARTEFACTS ON DISK, and they outlive a compact where this conversation does not.** Harness `<scratchpad>/rehearse.sh` (syntax-checked, halts after step 5, **every rc captured BEFORE any pipe**). Decoy `<scratchpad>/decoy-home/`. Copy will be `<scratchpad>/conflab-rehearsal` -- **path already given to cc**, who needs it for the AFTER manifest and the store probe.

**THE DECOY NEEDED FOUR THINGS, NOT ONE, AND THE THREE I ADDED ARE THE FINDING.** vc specified `.intent/`; `HOME` also feeds **`~/.gitconfig` (a SYMLINK out of HOME), `core.excludesfile=~/.gitignore_global`, and a HOME-relative `include.path`**. Without them step 6 dies on _please tell me who you are_, and **global ignores silently stop applying, which changes `git status --porcelain` -- and that output IS the hop's pathspec.** Two-arm control: seeded decoy resolves the identity, bare decoy returns EMPTY. **The failing arm is what makes it evidence.**

**STEP 0 RUNS UNDER THE REAL HOME. APPROVED DEVIATION, NOT A SILENT ONE.** cc's manifest writes to `$HOME/Devel/prj/.hop-manifests`; under the decoy that lands INSIDE the decoy and **cc never finds the file**. cc's command runs byte-for-byte unmodified and there is no move afterwards -- **a second operation is a second thing that can fail quietly.** Steps 1-7 stay under the decoy.

**FINDING 2 IS NOW A TWO-SIDED TEST BECAUSE vc STATED THE EXPECTATION BEFORE THE RUN.** Measured read-only: `git check-ignore -v intent/.cache/intent.db` in Conflab returns **NOT IGNORED**, and Conflab's guards do not run, so the D34 refusal that saves this tree is absent there. vc's counter-expectation: hop 2's `finish()` converges the ignore before the stamp, so the step-5 set should show **`.gitignore` MODIFIED and NO db**. **Either half failing is a finding. I have NOT read `facade.rs:159` and will not before the run** -- that would tell me what the code intends, and the rehearsal exists to measure what it does. The STOP guard on any `.db`/`.sqlite` path stays wired whatever the source says.

**FIDELITY CHECK HAS A PRECONDITION OR IT MEANS NOTHING.** cc offered a source-vs-copy manifest diff. It tests the rsync **only if the source is quiescent between the two runs**; a peer write in the gap makes them differ for reasons unrelated to copy fidelity, and **a spurious "the copy is unfaithful" is worse than no check, because it would stop the hop.** So: pin the source's HEAD and `status --porcelain` either side, and report VALID only if both are unchanged. Otherwise report INCONCLUSIVE, never the diff.

**A VALUE ON THIS BOARD DRIFTED AND I AM NOT CARRYING IT FORWARD: `verify-canonical.sh --self-test` reports 14 failures, not the 11 I recorded on the 27th.** The property -- the instrument can fail, so its PASS is believable -- holds. The number was never the point and I had banked it as if it were.

### THREE INTENT ISSUES QUEUED TO FILE. NOT BUILT, NOT FILED UNTIL vc LIFTS THE WINDOW. 2026-08-28 12:15Z.

**ALL THREE ARE ONE FAMILY AND I WILL SAY SO WHEN I FILE: A REPORTER READS THE CARRIER AND EXPECTS THE GATE BODY'S PROPERTIES.** The carrier is a SHIM; the body lives in `INTENT_HOME` and is reached through `~/.intent/home`. Every one of these is that confusion wearing a different hat.

1. **doctor's carrier advisory emits the AT-GRAMMAR remedy text on a hook finding** -- wrong remedy, wrong subject. (vc's.)
2. **`gate_currency` / `int hooks` / doctor compare the CARRIER against the gate BODY, so a correctly installed shim reads STALE.** (vc's.) **I met this from the operator's side this morning before any of it was named: `int hooks` told me this tree's dispatcher was STALE and pointed me at `intent claude upgrade --apply`, whose dry run I then measured as ALSO rewriting CLAUDE.md, AGENTS.md, `.claude/settings.json` and the chain block.** I declined to run a canon rewrite to fix a hook. **That transcript is the second datapoint and it is mine.**
3. **doctor reports gate-not-running on `.git/hooks/pre-commit.intent`** -- _"names no guard runner at all, so it executes no guards -- this is the Baize state"_. **Premise true, inference false for a shim:** it names no `GUARD_RUNNER` by design and resolves the gate through `~/.intent/home`. devbin-vc measured 0 `GUARD_RUNNER` refs / 2 `intent/home` refs, resolving to `pre-commit.sh` with 47 guard refs. **It fires on every estate adopting the shim, as a counted RED.**
   - **I HOLD AN INDEPENDENT SECOND INSTANCE AND IT IS ON A DIFFERENT TREE.** vc cites D1's real-hop commit printing `guards: 4 ran, 0 skipped`. **My rehearsal commit printed the same line, on the copy, at `rehearsal-logs/06-commit.err`** -- byte-identical carrier (`b0ed7edd`, 7332, mode 711). **So the false positive is measured on TWO trees by TWO runs, which is worth more than the same evidence twice from one.**

**THE REHEARSAL COPY IS DELETED (12:14Z), cc having confirmed done and vc having authorised it.** Closing record at `rehearsal-logs/COPY-DELETED.txt`. **Deleted rather than kept because a 3.4G tree that looks exactly like Conflab, sits at a DIFFERENT HEAD (`8279bb89` vs the real `7652c9b4`) and carries an installed v3 gate is a lookalike anyone could mistake for the real thing -- me included, after a compact.** That hazard is why the copy-mutate-run pattern ENDS in deletion; disk was never the reason, 2.4Ti was free. **Evidence survives it in full:** before/after/fidelity manifests in `~/Devel/prj/.hop-manifests/`, 48 step outputs in `rehearsal-logs/`, carrier record delivered to vc. **The one thing no manifest holds is the carrier, because `.git/` is outside `intent/`** -- which is why it went in the Phase 4 line rather than being left to a hash.
