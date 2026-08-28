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

---

# AFTERNOON, folded 2026-08-28 14:25Z

The reasoning below has a commit home; it is archived verbatim because a
board is not where a narrative belongs once it is in a message that cannot
be edited. Records: 69ea2657, a12147c1, 6945356e, 0120e8a5, 92570169, e9e71246.

## DOING, as it stood

## DOING

**NOTHING IN FLIGHT. The family hv assigned is closed. Today's earlier narrative is verbatim in `.history/20260828/wip.md`; what follows is the cold-session minimum.**

**THE MESSAGE-MECHANISM FAMILY -- 0105, 0106, 0109, 0112, 0113 -- LANDED AND CLOSED.** `69ea2657` (the three shell reporters), `a12147c1` (doctor.rs + finding.rs), `6945356e` (the closures). One class: **a reporter that reads the CARRIER and expects the GATE BODY's properties, or states a premise that was true when written.** Not one of them ever failed loudly. Reasoning is in the commit messages and is NOT restated here.

- **0105 was routed ADJACENT, not assigned** -- "take it if 0106's fix naturally covers it". **IT DID NOT and I took it anyway**, because splitting them leaves the estate red either way. Said so in the commit rather than letting `covered by` overstate what happened.
- **THE ONE THING NOT IN A COMMIT MESSAGE: not every new arm should fail the control, and saying so is the honest report.** Five new arms, FOUR go red when `carrier_shape` is pinned to Monolithic. The fifth passes under both BY DESIGN -- it asserts the Baize check SURVIVES the fix. **"5 new arms, 4 discriminate, 1 is a regression guard" is the true sentence; "5 arms, all driven" would have been the flattering one.**

**WHAT MY LANE DELIVERED, AS AN INDEX.** The Phase 1.1 rehearsal on a filesystem copy under a decoy HOME **predicted the real hop exactly** -- step-5 set 39 vs my 40, the difference being the `intent/events.jsonl` my run caught (now issue `0101`), and the carrier landed byte-identical to what I measured. **Three catches came from checking OTHER nodes' instruments against mine, not from my own plan:** cc's manifest would have written INTO my decoy where they could never find it; a `git clone` would have been structurally blind to the hook-door question it existed to test; and vc's "expect minutes for a cold clippy" was inverted -- seconds, because `gate_rust` returns early with no staged `.rs`.

**ISSUES: `0106` (low) and `0109` (medium) OPEN AND MINE. `0107` and `0108` CLOSED into devbin-cc's `0105`, my evidence folded in.** Detail in `39f938dd`, `1d1e0f4f`, `7f8b0a97`.

**THE SHIM CARRIER IS NOW INSTALLED ON THIS TREE (vc, hv's D1 option 1).** `int hooks` no longer reports STALE, because `gate_currency` takes the shim branch and compares against the shim template -- **`7109c7a2` working live.** Consequence to hold: **`0105` and `0106` are now reproducible HERE, so our own `doctor --verbose` carries two known false readings.** Anyone acting on that output needs to know first.

## The struck decision, in full

- **2026-08-28 -- ~~I CHECKED WHOSE THE RED WAS BEFORE REPORTING IT~~ RETRACTED WITHIN THE HOUR, BY cc, AND THE RETRACTION IS THE ENTRY. STRUCK IN PLACE RATHER THAN EDITED AWAY.** I wrote this as a lesson learned and it was the error itself, banked as a win.
  - **WHAT I CLAIMED:** a suite arm failed on `steel_threads.md` skew, the cause was a peer's uncommitted `render.rs`, and one command settled it -- does my own diff touch anything rendering the subject.
  - **WHAT IS TRUE:** `intentsvcs` HAS NO DEPENDENCY ON `intent-cli`, so that test can never compile or link `render.rs` -- the attribution was impossible, not merely wrong. The arm fails on a CLEAN tree at HEAD (rc=101). The real cause is `b4d63b44` (2026-08-27) putting the title into the view while `view_skew_check.rs:142` still asserts the view carries no title column; the test's last change is `608e9721`, **2026-08-20**, seven days earlier and untouched since. **Red on main for a day, and today was the first time anyone ran it.** All three verified on my own drive, not taken on cc's word.
  - **THE CLASS, WHICH IS THE ONLY PART WORTH KEEPING: EXONERATION AND ATTRIBUTION ARE TWO CLAIMS AND I MEASURED ONE.** "My diff does not render the subject" establishes that it is NOT MINE and says NOTHING about whose it is. I ran the half that cleared me, and asserted the half that accused someone else on no measurement at all. Either disproof was one command -- run the test on a clean tree, or grep the accused diff for the subject.
  - **AND I PUT IT INSIDE A SENTENCE CLAIMING I HAD CHECKED** -- _"I checked before saying it, which is the only reason this is a note to you and not the same false accusation I made yesterday."_ **That is FAMILY 6 from a new direction and less than six hours after cc taught me it:** the unmeasured claim rode inside a TRUE claim about my own rigour, which is a better carrier than a compliment because nobody audits a sentence about checking. **The louder I am about having measured, the less anyone will ask what I measured.**
  - **AND I DID IT A THIRD TIME INSIDE THE RETRACTION ITSELF, WHICH IS WHY THIS IS STRUCTURAL AND NOT CARELESSNESS.** Declining to authorise cc, I wrote that _"whether `steel_threads.md` should carry the title is the feature owner's call"_ -- asserting an OPEN feature question I had not checked for. It was already RULED: hv's parity direction, built by `b4d63b44`, and vc regenerated the view to it at `7d57fa9c` the same morning (verified, not taken on vc's word). **The refusal was right and its stated reason was invented.** Three instances in one afternoon, the third while retracting the second: **the reflex is to supply a REASON alongside a correct judgement, and the reason is where the unmeasured claim goes.** A judgement needs no borrowed premise to be right; when I cannot name what I measured, the honest form is "not mine to give" and nothing after it.
  - **cc's line ~~kept~~ WITHDRAWN BY ITS AUTHOR, and the correction is the surviving form.** They wrote that my red arm was _the better outcome -- shouting rather than lying_ against their `Super_Seded` control, which stayed green while false. **It is not better: a red nobody runs is not louder than a green, it is the same silence with a better excuse.** `view_skew_check` was red from the 27th and the first person to hear it was the first person to run it. **RED AND GREEN ARE THE SAME SIGNAL UNTIL SOMETHING EXECUTES THEM** -- so the colour is not the class.
  - **THE CLASS IS AN EXPECTATION WHOSE PREMISE A LANDED DECISION RETIRED, WITH NOTHING CONNECTING THE DECISION TO THE EXPECTATION.** Three costumes so far: cc's `Super_Seded` control (stale green the moment hv's D3 ruling put `superseded` into the vocabulary), my `view_skew_check` arm (stale red for a day), and `dispatch_ssot`'s bootstrap probe. **Only the third was found by anything other than luck**, which is the measurement that says this class has no detector -- and it is the same shape as `0112`, `0113` and `0106`, which I spent today fixing one instance at a time.

## The (C)-exhaust ruling block (2026-08-25), retired

- (2026-08-25) **hv RULED, UNPROMPTED AND VERBATIM: the `(C)` line is NOT required and never was.** _"This isn't a problem, has never been a problem, and is not something that I suggested we go looking for. The only constraint is that I DO NOT WANT ANY CLAUDE EXHAUST IN MY COMMITS. EVER. The end."_ **Killed a three-node sweep in one line:** vc's 10 missing-`(C)` commits, cc's 3, hv's own 2 -- **not findings**, retracted rather than carried as closed. cc's structural claim that the two lines are one block keeps its refutation (`(C)`-absent-and-trailer-present is a population of ZERO) but **loses its consequence**, and my 12-of-12 counter-datapoint went with it. **The proposed guard is therefore ONE-DIRECTIONAL: absent-trailer only.**
- (2026-08-25) **hv's SCOPE FINDING, AND IT IS ABOUT US RATHER THAN ABOUT THE COMMITS:** _not something that I suggested we go looking for._ I raised the Claude exhaust; the `(C)` sweep grew out of it across three nodes with measurements, mutual corrections, a withdrawn hypothesis and a remedy recommendation, **and nobody asked for any of it.** Each of us measured carefully. **CAREFUL MEASUREMENT OF AN UNASKED QUESTION STILL COSTS A DAY.**
- (2026-08-25) **THE EXHAUST ITSELF IS A TIGHT DISJUNCTION AND ONLY hv CAN CLOSE IT** (mine, adopted by vc to hv in my words). `includeCoAuthoredBy: false` is set, `attribution.commit` is byte-identical to the injected instruction's first line -- **but NO settings key anywhere holds the `Claude-Session` string**, global or project. **So EITHER that key gates the generated line despite its name and manual omission expires, OR nothing in configuration gates it, the omission is PERMANENT, and every fleet project inherits it.** No third branch survives; I exhausted the settings locations. **The deciding check: one commit from a session STARTED AFTER the change, then `grep -c '^Claude-Session:'`.** Neither vc nor I can run it from a session predating it. **90 carriers, 0 `Co-Authored-By`, first on main `55fc4a50` 2026-08-24.**
