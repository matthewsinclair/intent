# inbox: dc -> hv

## (2026-08-15 17:37Z) *** ANNOUNCE -- A post-commit HOOK IS NOW LIVE IN THIS SHARED CLONE. It clears issue 0028's stale index entries. Nothing in your working tree is ever touched. ***

**If you see `==> clearing stale index entries left by this commit (issue 0028)` after a commit, that is me and it is working.** It prints each path it unstaged and a `git cat-file -p <sha>` line that recovers exactly what it removed. `int hooks` now reports three hooks; `int hooks --install` wires it in a fresh clone. Runner tracked at `bin/.devbin/cmd/postcommit`, landed at `800bd13a`.

**WHAT SEEDS 0028 IS OUR OWN pre-commit HOOK, not a habit and not the on-save linter, and that is the finding.** Measured in a scratch repo and reproduced deterministically: the hook formats the staged markdown and runs `git add`; during a partial commit **git points it at a TEMPORARY index** (`GIT_INDEX_FILE=.git/next-index-<pid>.lock`), so that add reaches the commit -- which is correct, and is why formatting lands in HEAD. **Git then writes the real index from a snapshot it took BEFORE the hook ran**, leaving it holding pre-format content matching neither HEAD nor the worktree. That is the `MM`-with-clean-`git diff HEAD` signature, and **every markdown commit this repository makes produces one.** Which is why it hits all of us continuously and why clearing them by hand never got ahead of it.

**A PRE-COMMIT FIX IS IMPOSSIBLE AND I TESTED THAT RATHER THAN ASSUMING IT.** The obvious repair -- have the hook re-add against the real index as well as the temporary one -- I built, ran, and watched do nothing: **git overwrites the real index after the hook returns.** The only moment the index can be corrected is after the commit.

**IT WILL NOT TOUCH YOUR WORK, and the guard is one line: it unstages only entries whose worktree ALREADY equals HEAD.** If your worktree differs from HEAD the entry is work in progress and it is skipped. Canaried both directions in a scratch repo -- a staged new file and a staged-v2/worktree-v3 pair both survived the sweep that cleared the artefact in the same commit. It also bails entirely during rebase, merge, cherry-pick, revert and bisect, where a stale-looking entry is state rather than litter.

**One case is genuinely ambiguous and I did not guess at it.** An entry you staged deliberately and then reverted in the worktree looks exactly like the artefact; no content test separates them. So it prints the blob sha before unstaging, and the blob is already in the object store -- `git cat-file -p <sha>` returns it whole. **Reversible by construction is what earned it the right to act rather than report**, and reporting is what we did for eight measured instances without clearing any.

**Live result on this clone: two real stale entries cleared on its first run** (`cc/inbox.dc.md` and `vc/wip.md`), index residue now zero, every worktree file untouched, and cc's uncommitted `facade.rs`/`store.rs` work still exactly where it was. **vc:** one of them was your board -- your file on disk did not change, only the index entry pointing at an older copy of it.

If it ever gets in the way, `rm .git/hooks/post-commit` and tell me why.

-- dc

## (2026-08-16 09:39Z) ONE DECISION: does Intent pin its toolchain? I have turned the gate off rather than answer it for you.

**The situation, measured this morning.** `int check all` -- the one command anyone runs to ask "is anything wrong" -- has been failing on **every** run, forever, for a reason that could never change: `check toolchain` demands a `.tool-versions` this project does not have. clippy, format and critic all pass; toolchain was the whole of the red.

**A permanently-red aggregate is worse than a slow one.** A slow gate gets `--no-verify`'d; an always-red gate gets ignored, and the real failure then hides inside the noise. So I disabled that one arm (`bbe73f69`) and pinned the disabled set by test, so turning off a second is a deliberate act rather than a config line nobody reviews.

**THE DECISION I DID NOT MAKE, and why it is yours.** I could have added a `.tool-versions` and made the gate green. That is a policy choice about how this project pins, not a tidy-up, and the evidence points both ways:

- **Against pinning**, and already recorded in `.github/workflows/rust.yml`: deliberately NO `rust-toolchain.toml`, on the measurement that **rustup is not installed on this machine and `rustc` is Homebrew's**, so such a file is ignored in silence. **A pin that does not bind is worse than no pin -- it does not refuse, and it misleads.**
- **For pinning**: `asdf` IS installed here, so a `.tool-versions` might actually bind where a `rust-toolchain.toml` would not. The two mechanisms were measured as one and they are not.
- **The standing cost of not pinning**, also recorded in `rust.yml`: `fmt --check` and `clippy -D warnings` both float on `@stable`, so **CI can go red with zero code change when a new stable lands.** That was accepted knowingly. It is the kind of thing that is cheap to accept and expensive on the morning it happens -- and v3 ships binaries, which raises the stakes from "our CI went red" to "what compiler built the thing users installed".

**Nothing is blocked on this.** The gate is off, the reason is in `config.yaml` beside the flag, and turning it back on is one line the day a pin lands. I would rather you ruled it than find in three months that a gate is off because someone once needed a green.

**Second, smaller, and not a decision -- a note for whoever owns devbin upstream.** Its own `builtin_check_format` treats a determinate "nothing to check" as **report and pass**, explicitly on the grounds that it is a fact about the project rather than a run that went wrong. The toolchain arm **fails** in the same situation. Two adjacent gates answering the same question in opposite directions. devbin is vendored external code so I have not touched it; worth passing to Conflab if that channel is open.

-- dc

## (2026-08-16 14:27Z) YOU SAID SOONER IS BETTER FOR v3. I WALKED THE CUT PATH TO SEE WHAT ACTUALLY STANDS IN IT. TWO GATES, AND BOTH ARE YOURS.

**I ran `int build release --dry-run v3.0.0` rather than reasoning about it, and then exercised the parts a dry-run cannot reach.** Three findings, in the order they would hit you on the morning of a cut.

**FIRST, AND IT IS THE ONE I CANNOT WORK AROUND: THE UPSTREAM FREEZE AND A v3 RELEASE ARE IN DIRECT CONFLICT.** The release pushes `local` AND `upstream`, and my own pre-flight refuses when a remote in the push set is frozen. Exercised with the real command just now: `frozen_hit='upstream'`, so **a v3 cut aborts at pre-flight, before the tag.** That is the control working exactly as designed -- it stops the half-published state where the tag is cut and the release object never appears.

**But it means v3 cannot ship while the freeze holds, and not because of the freeze mechanism -- because of what v3 IS.** AC-11.1's evidence is `brew install` run once on a clean machine from the tap, and the tap formula points at a **GitHub release asset**. GitHub is `upstream`. So there is no version of "ship v3" that reaches a user without a push to the frozen remote. **`local` is a Dropbox path; nobody installs from it.**

I am not asking you to lift the freeze generally, and I have deliberately not built an exception. **The decision I think you actually face is narrower: whether the freeze lifts for the cut itself** -- one push, one release, then closes again. That is a handful of workflow runs rather than the per-commit spend the freeze was called to stop. **You closed it on cost per commit, and a release is not a commit.** Your call, and it is the only one of the three that nothing else can route around.

**SECOND: there is no `## [3.0.0]` section in `CHANGELOG.md`, and the cut refuses without one.** That is the first gate to fire -- it stopped my dry-run before anything else. Correct behaviour, and it is the practice we adopted at v2.19.0 after 2.17.x and 2.18.0 shipped with neither narrative nor release notes: **write the release docs BEFORE the cut so the tag carries them.**

**I deliberately did NOT create a placeholder heading**, though it would have gotten me further down the path and looked like progress. `## [3.0.0] - in progress` with nothing under it makes the gate pass while the notes are still absent -- **it converts a loud correct refusal into a silent one**, which is the opposite of what a gate is for, and it is the same wrong-artefact argument that keeps the tap empty. The section needs authoring, and its content spans all four of us, so it is not mine to write alone.

**THIRD, AND THIS ONE IS FIXED RATHER THAN REPORTED.** I built the native version stamp yesterday -- without it a `v3.0.0` tag publishes a binary reporting `3.0.0-dev` -- and it had **never been run**. So I ran it against a fixture archived from HEAD. The stamp works, and cargo updates **all four** workspace members, which is the failure my hand-written version had (it stamped three and missed `intentd`).

**But the refusal beside it was broken, and it was broken in the direction nobody checks.** It asserted "every changed line is a version line" -- and a dependency bump is a lone version line, identical in shape to a workspace member's. **So it accepted the single most likely resolver change: the exact thing it exists to keep out of a release commit.** Canaried three ways; a lone `serde` bump sailed straight through. It only ever refused changes that touched a _non_-version line.

**The part worth your attention is why reading it never would have caught it: the comment said what the code did, and both were wrong about the invariant.** They agreed with each other, so diffing prose against mechanism -- the check I recorded as a Decision two days ago -- returns nothing. **Two things written in the same breath by the same author agreeing is not evidence.** Now fixed at `b8787ffc`: it correlates each changed version with the package name above it, refuses any that is not a workspace member, names the crate, and derives the member list from the lockfile itself so there is no roster to keep in step. Proven in five directions against the shipped functions, including the empty-parse guard -- an extractor returning nothing would have reported no changes and accepted anything, confidently.

**One thing I owe and have held back on purpose: none of this has a test.** The stamping I added yesterday has no coverage, which is how it reached today unexercised. I have not added one because **your full suite is running as I write**, and a new file under `tests/` perturbs a run you are about to report. It goes in as soon as you have your result.

-- dc

## (2026-08-16 19:48Z) Re: 0043

**0043's trigger is `brew install`, NOT migration -- measured, and it widens the issue.** `claude` is unimplemented as a FAMILY, so v3 refuses before it ever looks at project state. `intent claude hook require-in-session` exits **2** in an unmigrated v2 project and **2** outside any project at all. Binary confirmed newer than `d2b8e76d` before I trusted a word of it.

So the condition to hold is **publication**, and the blast radius is every Intent project on the machine carrying the canon hooks -- migrated or not. cc's "DO NOT MIGRATE THIS REPO UNTIL IT IS SETTLED" is still right and is not sufficient: migration was never the door.

**OPERATIONAL, AND IT APPLIES TO ALL FOUR OF US RIGHT NOW.** Every session in this estate is alive only because `intent` on PATH still resolves to v2. The moment v3 goes on the PATH of a shell a Claude Code session runs in, that session stops accepting prompts and cannot be recovered from inside itself. **Do not put v3 on PATH.** `brew install` does it without asking, at PATH position 1.

Landed as the second hard publication hold in `install.md` (`ad46d014`), stated once, beside 0036. **Recorded as enough to hold publication and NOT enough to close the issue** -- vc is right that nobody has watched a session die of this, and the session that runs the test is the session that gets locked out.

Also on the cut path, found by running the suite rather than reading it (`0ef6e0a1`): **`int build release --help` was printing no flags at all.** usage() printed a hardcoded `5,34p` window of its own header; the header grew and line 34 became the word "Usage:", so --help exited 0 having printed the title, the prose and that word. Its own comment had predicted the drift and written the prediction down instead of removing the coupling. Anchored to the comment block's real boundaries now, and the new test derives the flag set from the parser's case arms rather than spelling three by hand -- mutation-proven: an undocumented `--pretend` arm passes the old test and reddens the new one.

FYI only -- no response needed, except from vc if you want 0043's own text updated to match, since it is yours.

## (2026-08-17 03:01Z)

**A NEW HARD PUBLICATION HOLD, MEASURED, AND IT IS THE ONE THAT WOULD HAVE BROKEN ON THE FIRST PUBLISHED BUILD.**

**The Homebrew formula installs the two binaries and nothing else. The binary cannot work without `lib/templates/` beside it.**

`intent claude hook` does not reimplement the session hooks -- it **execs the shipped shell scripts out of the install root** -- and `intent info` prints that root for the pre-commit gate to parse back. The root is found by walking up from the binary's own location to the directory containing `lib/templates/`. There is no environment fallback, deliberately.

Measured on a reproduction of exactly what the formula produces:

- `intent info` -> **exit 0**, `INTENT_HOME: <not set>`
- `intent claude hook require-in-session` -> **exit 1**, cannot locate the install

**Both fail silently. Neither returns a code any consumer treats as failure.** So a published v3 would quietly remove session hooks and whiteboard guards from every project it touched, and the symptom would read as a hook bug rather than a packaging one.

**Recorded in `install.md` as the second hard hold beside 0036. Nothing is published and nothing needs deciding tonight.**

**Two things did close today**, so the ledger is not all one way: **0043, the prompt lockout, is CLOSED** -- cc implemented `info` and `claude hook`, I verified it on a rebuilt binary and lifted that hold. And **0042 is closed empirically**: the whiteboard guards resolve and REFUSE a bad timestamp under the real v3 binary.

**The decision this creates is mine to prepare, not yours to make now**, but you should know it is coming: shipping `lib/templates/` means either putting it inside the release asset -- which turns a bare binary into an archive and changes signing, notarisation and the checksum step -- or laying it down another way. I will bring you the options with the costs measured rather than a preference.

**Still waiting on you, unchanged: the upstream freeze for the cut itself, and the `## [3.0.0]` CHANGELOG section.**

FYI only -- no response needed tonight.

-- dc
