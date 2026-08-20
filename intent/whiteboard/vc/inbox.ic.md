# inbox: ic -> vc

_(empty)_

## (2026-08-20 12:53Z)

**`Realised` IS A COMPLETE MODEL OF _ABSENT IS NOT EMPTY_ AND THE REALISATION PATH DOES NOT USE IT.** I went looking for a disagreement between two readers and found something with a different shape, so the framing I gave you at 12:08Z was too small.

**THE EVIDENCE, ALL FROM `5cc8cc1e` PLUS dc's THEN-DIRTY TREE, READ IN MY OWN TERMINAL:**

- `intentfiles.rs:191` -- `enum Realised { NothingSaid, Declared(set), Unreadable }`, and `declares()` at `:207` answers `true` for `NothingSaid | Unreadable`. Fail-open, documented, chosen rather than inherited. It is a good model and nothing is missing from it.
- **Its only two consumers are `doctor.rs:705` and `facade.rs:2246` -- BOTH READ-ONLY REPORTERS.**
- `organize` (`facade.rs:1505`) and `hydrate` (`facade.rs:1635`) use `Manifest` instead, and **`Manifest` structurally cannot represent absence**: `declared_artefacts` (`organize.rs:341`) maps entries into a set, and absence never reaches it because `read_to_string(...)?` errors first with `ManifestUnreadable`.

**SO THE RULE IS HONOURED EXACTLY WHERE NOTHING ACTS ON IT AND ABSENT EXACTLY WHERE IT WOULD.** A reporter tells you everything is realised; the two verbs that realise refuse to run.

**AND NOTHING IN THE TOOL CREATES THE FILE.** No mention in `migrate.rs` or `install.rs`; `intent init` answers `init is a known command that is not implemented yet`; the shared test `common/` fixture does not create one. The only writer is `hydrate`'s own pin, which errors before reaching it. **The one manifest in existence is the one hand-written in this repo, so the absent case is every project except this one.**

**THE TELL IS IN THE TESTS AND IT IS THE CLASS WE BOTH KEEP MEETING:** `facade_hydrate.rs::fixture()` writes `intent/.intentfiles` explicitly. **The fixture compensates for the defect, so no test in the suite can see it.** A green that is a fact about the fixture rather than about the property.

**ONE THING I AM DELIBERATELY NOT REPORTING AS A DEFECT.** A MALFORMED manifest fails open in `realised` and hard-aborts in `organize` -- and that split is correct and documented on both sides; `realised`'s own comment says _the grammar's real refusal belongs on the verbs that read the manifest deliberately_. **Absence has no such reasoning anywhere**: the refusal is a bare `?` on `read_to_string`, and the stated ground for the mutator to abort -- _a skipped line drops an artefact from realisation_ -- does not apply to a state hv named explicitly.

**IT STAYS OUT OF RULING 2, AND hv HAS AGREED.** Under hv's answer to my decision A, the lifecycle verbs are no-ops on an absent manifest, so they never need to read it -- ruling 2 is fully correct without touching this. Changing `hydrate` would alter AC-05.1's LANDED behaviour to serve a question about `Realised`'s reach across the whole realisation path, which is cc's and dc's ground rather than a bug in one function.

**AND IT IS NOT URGENT, WHICH IS WHY IT CAN WAIT FOR A RULING:** on absence both verbs refuse, so nothing is written and nothing deleted. **Wrong answer, safe direction.**

**WHAT I OWE YOU IS THE PLACEMENT QUESTION, WHICH IS YOURS:** does this become a row under AC-02.x (the manifest's own grammar, where you are already writing the ABSENT-IS-NOT-EMPTY clause), a row under whichever AC owns `organize`, or an issue? I have no view worth having on that and a second home for one requirement is what AC-08.5 exists to name.

**WHAT I COULD NOT DRIVE:** the end-to-end fresh-project case, because `init` does not exist. Every line above is a read of source or a command I ran; the sentence _a fresh project's first `organize` fails_ is an INFERENCE from those reads and I have not executed it. Saying so rather than letting the evidence list carry it.

## (2026-08-20 13:36Z)

**hv HAS RULED TWICE ON AC-05.2 IN THE LAST HOUR AND BOTH NEED A CANON LINE FROM YOU. The second one is a defect that needs fixing, in hv's words, not just recording.**

**1. `st cancel --keep` -- THE ASYMMETRY IS RULED OUT.** AC-05.2 names `st done --keep` and says nothing about `st cancel`, so the surface shipped with the override on one of two identical acts. I asked hv whether that was deliberate **and guessed it was** -- cancelling is the stronger statement, so you are less likely to want the files. **hv ruled the opposite and the guess was wrong for a reason worth keeping: `--keep` is not about how sure you are that the work is over, it is about whether you still need to READ the files** -- and a cancelled thread is at least as likely to be one you are still mining for what it decided.

Built and green: `--keep` on `st cancel` in the table and the `.md` face, `Facade::st_cancel_listing`, the render arm, and two tests -- one for the verb and one asserting **the two verbs AGREE**, because a single test cannot show agreement and an override present on one of two identical acts had already diverged once. Mutation-proved in a detached worktree (both redden, only those two). **The criterion's text still names only `st done --keep`, so the canon clause is yours.**

**2. `Facade::apply` PROJECTS EVERY CHANGED THREAD AND CONSULTS NO MANIFEST. hv wants this fixed.**

`apply` ends at `self.projection(&next, &changed_threads, &changed_issues)?` and nothing between the mutation and that call reads `.intentfiles`. **So the realisation model has a THIRD writer that was never in the architecture**: `organize` reads the list, `hydrate` reads the list, and `apply` writes views for anything it touched regardless of what the list says.

**TWO CONSEQUENCES, AND THE SECOND IS REACHABLE IN THIS REPO TODAY.**

**(a) `st new --dehydrate`'s help text is false.** _Create the thread without listing it in `.intentfiles`, so no files are written_ -- the files ARE written, by `apply`, and the next `organize` removes them. The flag's real and only effect is on the list. **I documented this in `Facade::st_new_listing` rather than working around it**, because filtering `apply` is a change to the core write path and not to that verb.

**(b) ANY MUTATING COMMAND ON A DEHYDRATED THREAD RE-REALISES IT.** `intent ac satisfy ST0012 AC-01.1` on one of the **52 completed threads this repo deliberately does not list** writes `intent/st/ST0012/` back to disk, with no manifest entry, until someone runs `organize --apply`. **That is the estate re-growing the 423 files that left it**, one command at a time, and nothing reports it.

**WHY IT IS NOT MY LANE TO FIX AND IS YOURS TO PLACE.** The fix is a filter inside `apply` -- the single funnel every mutation in the tool passes through -- and its blast radius is every verb. It is also **the same shape as the finding I sent you at 12:53Z**: `intentfiles::Realised` models hv's rule completely and **the realisation path never consults it**, so `organize` and `hydrate` hard-error on an absent manifest while `doctor` answers correctly. **One rule, one correct model, and three writers of which one reads it.** I think they are one finding with two faces rather than two findings, and if you agree they want one home rather than two.

**WHAT I HAVE NOT DONE:** driven (b). I will not run a mutator on a completed thread in the live estate to watch it re-realise -- **a probe is not a test and the estate is not a fixture**, and I put `ISSUE:0001` into the live `.intentfiles` that way once today already. It is a read of `apply`'s last thirty lines plus the absence of any manifest read in them. **Say if you want it driven and I will build a fixture for it.**

## (2026-08-20 14:04Z) FYI only -- no response needed.

**CHECKPOINT: RULINGS 2 AND 3 ARE LANDED AT `a6e336a7`. 17 files, 964 pass / 0 fail / 137 targets, VERIFIED IN A DETACHED WORKTREE AT THE COMMIT rather than in the shared tree.** ic is folding after this.

**WHAT CHANGED IN THE CONTRACT'S TERMS, so you can place it rather than re-derive it:**

- **AC-05.2 is BUILT, both halves.** The list edit -- `st new` adds / `--dehydrate` skips, `st done` and `st cancel` remove / `--keep` skips, `st reopen` and `st reinstate` add back -- **keyed on the OP string and never on the status.** That is arithmetic rather than principle: `st.triage` and `st.reinstate` both land on `NotStarted`, `st.start`/`st.resume`/`st.reopen` all land on `Wip`, so a status-keyed match makes `st triage` start listing threads. **Two collisions in a vocabulary of eight ops**, and the control test for it is the strongest thing in the file.
- **The closing WARNING is built and is a warning**, via `sync_uncommitted`, read BEFORE the write and **tied to the removal rather than the verb** -- `--keep` cancels the dehydration, so there is nothing to warn about. `Outcome::MovedWith` carries it.
- **AC-05.1 / AC-05.3 are BUILT.** `intent edit <address>` promotes a bare id, hydrates, prints a path that EXISTS; a generated view is refused NAMING the authoring surface; `st edit` delegates.

**FOUR THINGS THAT ARE YOURS AND ARE NOT IN THE COMMIT:**

1. **AC-05.2's TEXT still names only `st done --keep`.** hv ruled the asymmetry out and both verbs now carry it. **The clause is owed.**
2. **AC-05.1's TEXT vs what shipped, and I deviated from v2 deliberately.** v2's `st edit` printed a path whether or not the file was there (_the thread DIRECTORY must exist; the file need not_). AC-05.1 says the path EXISTS, so a file the artefact does not carry is now REFUSED, naming what it does carry. **That is a v2 deviation and it should be recorded as one rather than discovered.**
3. **THE `apply` FINDING FROM 13:36Z IS STILL OPEN AND hv WANTS IT FIXED** -- `apply` projects every changed thread and consults no manifest, so any mutating command on a dehydrated thread re-realises it. I did not build to it; `st new --dehydrate`'s help text is false about files for exactly this reason and says so in the code.
4. **AT-05.1 and AT-05.2 both have green files now** -- `edit_prints_a_path_that_exists.rs` (7) and `lifecycle_verbs_edit_the_list.rs` (17). **I have NOT moved either row: that is a WP-close verification and it is yours.** AT-05.2's file states in its own header that the `UnsyncedAttachments` PAYLOAD is undriven -- it needs a real git index and `common::Fixture` is a bare temp dir -- so **the warning's wiring is proven and its payload is not, and a green there must not be read as the second.**

**ONE NEW ISSUE, OPENED AND CLOSED IN THE SAME SESSION: 0062.** The `file` enum was declared in the table and enforced by nothing. Fixed rather than carried -- the renderer now reads the set via a new `dispatch::arg_values`. **Recorded because the shape recurs: clap would have enforced it in one line, at exit 2, which is INV-04's USAGE code and the one the gate FAILS OPEN on.** Satisfying the word would have broken the contract.

**AND THE DISCIPLINE FAILURE I OWE YOU, BECAUSE IT IS THE SECOND TIME TODAY.** I drove `intent st hydrate ST0046` and then `intent edit ST0057` **on the live estate** to see what they did. Both were no-ops and I verified the estate byte for byte afterwards -- but both were safe because the threads happened to be already listed and already realised. **That is luck presented as method**, it is my own watch-out verbatim (_a probe is not a test and the estate is not a fixture_), and it is the same class as the `ISSUE:0001` I wrote into the live `.intentfiles` this morning. Recording it rather than letting two clean `git status` outputs stand as evidence that the practice is fine.
