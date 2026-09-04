---
node: cc
name: Control Claude
role: control
session_id: 98a46c38-f370-4d67-b2c5-c2536e0ae8f9
commit_session_id: 01XYetoGJWvBxvL4PE8sGZTu -- HARNESS-ANNOUNCED ONLY, NOT WITNESSED. I have authored no commit this session, so there is no trailer to read it off and the honest provenance is the announcement alone. dc reads theirs off the artefact; mine is one source short of that until I commit. POINT-IN-TIME, one session; a restart mints a new one.
heartbeat_at: 2026-09-04 07:02Z
status: active
focus: "BOOTED AND HOLDING ON hv INSTRUCTION 2026-09-04 07:02Z -- boot complete, waiting on vc for direction. NOTHING STARTED, NOTHING IN FLIGHT; status stays active because a hold is not a session ending. THREE THINGS RE-MEASURED AT THIS BOOT RATHER THAN CARRIED FORWARD FROM MY OWN BOARD. (1) intentd is STILL pid 66522, started 2026-09-01 13:29Z, against a disk binary rebuilt 2026-09-03 14:44Z -- so it still answers about 09-01 and the six daemon-gated TODO items are gated exactly as they were last night. (2) THE DELIVERED PAIR IS CURRENT IN EVERYTHING THAT MATTERS AND THE BARE SHA COMPARISON SAYS OTHERWISE: marker 6dac00f7, and the deciding test -- diff over native/rust surface from marker to HEAD -- returns TWO FILES, both intentd TESTS. Behind HEAD, by nothing that changes what the binary does. (3) vc RELEASE OF 2026-09-03 17:43Z IS UNCONSUMED: both no-canon items AND the canon batch under mandatory read-verify-retry were released to me and I did nothing with them before EOD. NO FIGURES FROM MEMORY; RUN THE VERBS."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT. BOOTED AND HOLDING ON hv INSTRUCTION, WAITING ON vc FOR DIRECTION.** All four of my inboxes are empty of unhandled entries. **The tree is NOT clean and HEAD moved during my boot**: HEAD is `04c43798f` (ic's own boot-and-hold, one file), and four board files are dirty while **`dc/wip.md` is STAGED in the shared index** -- so a bare `git commit` from here carries dc bytes under my signature. `add` + `commit --only` or nothing.

**THE ONE THING THIS BOOT CHANGES ABOUT MY OWN STATE, AND IT IS A RELEASE I NEVER SPENT.** vc released me at 17:43Z yesterday -- both no-canon items and the canon batch, the batch under mandatory read-verify-retry -- and EOD landed before I took any of it. **So I am not blocked on vc for the batch; I am blocked on vc for DIRECTION, which is what hv instructed.** The distinction matters because the first would be a hold with a condition I could check myself, and this is not one.

**WHAT I RE-MEASURED RATHER THAN RECALLED, WITH THE INSTRUMENT NAMED.** The daemon: `ps -o lstart -p 66522` against `ls -l` on the resolved symlink target -- same process as yesterday, still older than the binary beside it. The pair: `intent --version` gives the marker and `git diff --name-only <marker>..HEAD -- native/rust surface` is the DECIDING test, not the sha inequality; it returns two intentd test files. **Reading the differing shas as staleness would have been watch-out A again -- a true reading of provenance offered as a claim about currency.**

**WHAT THE PEN DOES NOT COVER, KEPT BECAUSE I WILL BE TEMPTED TO READ IT WIDER ON THE BOUNCE:** stopping the shared daemon, the rebuild window (`0196`), ADC signing, `intent claude skills sync`, and the tag. **`intent fc` is hv's alone, pen or no pen.** A general instruction is never a specific grant.

## TODO -- startable, mine, smallest first

- **XS** `0095`/`0096` -- CLOSE as never-specified, reason on the record. Driven: empty in title AND body. **They are `0223` debris, not rows anybody failed to specify.**
- **XS** File the `implemented_check.sh` false-positive class.
- **XS-S** `render.rs:1594` `browsed()` -- stop saying _needs a running daemon and none is running_ while one answers. Size JUDGED, not measured.
- **S** `0063` -- FIELD MIGRATION: the title's 187 chars into the body, short title left. vc's ruling; no knowledge recovered, nothing invented. **NOT a close.**
- **S** `0205` -- vendored fourth block ACCEPTED with its reason at `bin/.devbin/lib/builtins:66`.
- **S** Migrator-commit -- `migration.md` Phase B step 7 and `AC-00.8` stop claiming _one commit_. vc ruled: correct doc and row, do NOT build the commit.
- **S** `implemented_check.sh` fix -- classify on marker AND rc=2.
- **M** `0192` RULED IN -- refusal in `info_read_back`, placement already decided.
- **S-M** `browse` daemon half -- an entity page and an open path, so `AC-17.6`'s one-model-one-service holds. **This is the item I sized XS off `--help` and got wrong; this size is off the code and is still a judgement.**
- **S-M** `SERVED_BY_DAEMON` is ONE entry (`render.rs:235`). **RECOVERED FROM vc's INBOX DURING THIS FOLD -- it was not in the report I sent hv.** Not a hazard: exclusions refuse loudly, and the discharge condition is already in the code -- it becomes a projection of the dispatch table rather than a second home. Size JUDGED.
- **L, AND THE SIZE IS A GUESS** WP-06's 9 unmet CLI rows (`ext` 5, `config` 3, `learn` 1). An aggregate never individually sized since the audit. Treat as unsized.

## Holds -- mine, with the condition that releases each

- **XS** `WP-08` close -- RELEASED WHEN conformance coverage exists for the daemon, or the gap is explicitly accepted on the record. My `0216` harness is one piece and I am not arguing it discharges the condition.
- **M** `AC-06.1`'s coverage half -- RELEASED WHEN a burn TSV covering the estate exists AND `INTENT_BIN` resolves to one binary rather than three. `coverage_map.sh` refuses to publish and is RIGHT to.
- **L** `0216`/`0226` fix -- RELEASED WHEN a monotonic version the ingest does not own exists. The obvious fix collides with `written_at`, which the ingest rewrites wholesale.

## Decisions owed by hv -- question, options, recommendation

- **Should `at green` run the L3 arm?** (i) warn, do not refuse (ii) refuse (iii) leave. **REC (i)** -- refusing breaks the legitimate write-then-cite order, which is the order that produced this morning's outage.
- **`INTENT_BIN` flip and re-baseline -- which order?** (i) flip then re-baseline (ii) re-baseline then flip (iii) neither this cut. **REC (i)** -- the default is `bin/intent`, the v2 SHELL SCRIPT, and it is three binaries not two; the other order pays the wall time twice.
- **`burn.sh` re-run, or accept `AC-06.1`'s coverage half red?** (i) run (ii) accept red and say so on the row (iii) descope the half. **REC (i), and it is hv's because full-suite runs are.**
- **`config` bare resolves to `target: undefined` -- what should it do?** (i) print the resolved config (ii) print help (iii) refuse with a remedy. **REC (i), FLAGGED: this rests on my reading of the surface, not on a census of bare noun verbs.**
- **`agents` bare is `pending-hv`.** Same options. **REC: whatever (d) gets** -- two bare nouns answering differently is the defect, not either answer.
- **`WP-08`: endorse vc's hold or override?** **REC endorse.** A blocker that erodes because the blocked party built one piece of the thing is how conditions stop meaning anything.
- **Flip `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"` to a gate?** (i) clear the class then gate (ii) stay report-only (iii) gate now. **REC (i)** -- five of six targets I once called absent exist under another name. **PREMISE MOVED: the account lived in `0214`, which is now CLOSED.** Lint half is dc's.

## Open, no owner

- **Something WALKS the CLI surface.** Seven CLI-token-titled creations in `event_log`, two episodes eight days apart, machine-paced, `st` then `issues` both times. ic found no generator and named no suspect.
- **Is my unfiled daemon-lock race a duplicate of `0210`?** Adjacent ground, NOT compared.
- **Does one fix serve both `0216` and `0226`?** Same collision, opposite symptoms. Not driven.
- **Which symptom you get may depend where in the window you land** -- live tree showed the silent form, the harness at high contention the refusing form. Hypothesis.

## Watch-outs

**Folded into families. The rules are kept; the worked examples are in `.history/20260903/wip-prefold-1718Z.md` and are not repeated here.**

**A. THE INSTRUMENT ANSWERED A QUESTION ADJACENT TO THE ONE ASKED, AND ANSWERED IT CORRECTLY.** The dominant family and the one that keeps arriving while I am being careful. Instances: `--help` read as an arm; a RENDERED issue read as a field; a green gate read as behaviour; a name census promoted to a claim about mechanisms; `grep -i FAILED` matching `0 failed`; a gate verdict read from the working tree with nothing saying which tree; a display filter (`if d<1800`) standing as a claim about the population; a constructed variable quoted as measured, including by its author; a base rate wearing the safety question's clothes; a claim that sounds like physics exempted from measurement. **RULE: name the FIELD you read and the INSTRUMENT you read it with. `--help`, a clap subcommand and a declared row are statements about the PLAN; only an arm or a drive is evidence about the BUILD.**

**B. CONTROLS, OR THE READING IS NOT EVIDENCE.** A control that cannot distinguish _safe_ from _never tried_ is not a control, and it must vary the axis the check actually reads. **An instrument that is part of its own population is worse than a noisy one -- it is CONFIRMATORY** (`Op::Registry` is in `wire::UNCOUNTED` for exactly this reason; before reading a meter in a loop, establish that reading it is not an event the meter counts). **RULE: positive-control the instrument before its silence means anything.**

**C. ARITHMETIC AND SAMPLE SIZE.** **n=2 is not a result about a stochastic process** -- I fired a pre-committed disconfirming condition on two samples, into an artefact, with the variance already visible in a sweep I had printed. **A total you did not enumerate is not a total you may publish**, and adding to someone else's total requires reading what is in theirs. **RULE: state n and state the variance; below n=5 on a process you have watched vary, the honest sentence is _observed twice, not characterised_.**

**D. PREMISES.** **Drive a ruling's premise before building on it** -- of four build rulings in one day, THREE had false premises, and today `0086` (closed), `0214` (closed) and `0063` (a field migration, not a rewrite) all moved under rulings that named them. **Assert the premise your fix rests on as a test, in the direction that would embarrass you.**

**A2. THE CLOCK RULE IS A RULE ABOUT IDENTIFIERS, AND I FOUND THAT OUT BY BREAKING IT ON A SHA AT THIS BOOT.** `restart.md` generator 2 is _fabrication with the correct value present_ and it is written up entirely in terms of TIMESTAMPS. **It is not a fact about clocks. It is a fact about any opaque token a reader cannot check by looking at it** -- I typed HEAD as `f1ff2f81` into a message to vc with the real `f1ff2f824` four lines up in my own tool output, and it was caught only because I re-ran `git rev-parse` for an unrelated reason. **A wrong sha is worse than a wrong stamp: a stamp lands in a range a reader can smell, and a sha resolves or it does not, so a plausible one sends the reader to `git cat-file` and not to me.** RULE: **substitute the command, never the value** -- the same remedy the clock rule already gives -- and that covers shas, pids, issue numbers and line numbers, none of which the written rule names.

**E. THIS BOX AND THIS SHELL.** `cmd | head` reports head's status -- **done again today.** The Bash tool's shell is zsh: unquoted `$var` does not word-split and an unmatched glob (`--include=*.md`) aborts the whole command -- **hit again today.** A stale binary cannot answer a question about HEAD. `cargo check --workspace --all-targets` -- the flag is the half memory drops.

**F. THE SHARED CHECKOUT.** Canon cannot be split, so every canon commit is silently multi-node. **`add` + `commit --only` is the only safe write; a live `index.lock` is a WAIT, never a removal, and the retry is the SAME command re-issued, never a recomposed one** -- both exercised today. Two correct rules can take the machine down: a release build DELETES the shared pair before building (`0196`) while the dirty-tree guard REDIRECTS. **Currency is a PROPERTY, never a value; a pin trailing HEAD is the correct steady state.**

**G. `0216` AND WRITING CANON.** **The read-verify-retry loop is a REQUIREMENT, not good practice** -- it replaces _one verb at a time_, because **the debouncer sees WRITES, NOT AUTHORS, so spacing protects only against your own burst.** Verify on an OBSERVABLE, never on a duration, and never on the tool's `ok`. **After a revert the DISK WINS, so store and disk agreeing AT THE OLD VALUE is the signature, not the exclusion** (vc). `intent st attach` writes store AND canon and NEVER the disk file (`0082`).

## Decisions

- (2026-09-03, ic->cc) **THE CLASS IS THE UNDRIVEN NUMBER, NOT THE WRONG ARTEFACT.** ic's framing supersedes mine and carries their name.
- (2026-09-03, cc+vc) **CONTENTION IS `0216`'s VARIABLE -- not spacing, not corpus size.** And **refusals (`0226`) and silent losses (`0216`) TRADE OFF**, so a single counter prints _fewer losses under load_ and reads as the defect improving.
- (2026-09-02, cc+vc) **`0216`'s FIX IS DAEMON-SIDE AND NOT USAGE DISCIPLINE.** A hazard reachable by an ordinary shell loop cannot be mitigated by how carefully nodes write.
- (2026-09-02, vc) **A RIDER CANNOT BE VIOLATED BY A CASE ITS OWN HAZARD CANNOT REACH.**
- (2026-09-02, vc) **`WP-14` AND ALL 12 OF ITS ACs WERE DESCOPED WHOLE TO ST0069** by hv on 2026-08-30. `AC-09.5`'s wip/boards half goes with it.
- (2026-09-02) **TWO MACHINE PROJECTIONS OF ONE VALUE MUST NOT DRIFT; A HUMAN RENDERING OF IT IS NOT A COPY AT ALL.**
- (2026-09-02) **CONTENT COMPARISON DOMINATES A VERSION COUNTER FOR A COMPARE-AND-SWAP.** A dominance argument ends a design debate that competing assertions cannot.
- (2026-09-02, vc) **AN EXCLUSION MUST BE VISIBLE WHEREVER THE BEHAVIOUR IS CLAIMED, NOT ONLY WHERE THE CHECK LIVES.**
- (2026-09-01, hv) **v3.0.1 IS FEATURE COMPLETE, THERE IS NO TAG WINDOW AND NO EXTERNAL CONSUMER, AND COST IS NOT A CONSTRAINT.** The scarcity register is retired as a class.
- (2026-09-01) **A REFUSAL THAT CANNOT SAY WHAT IT FOUND MAKES ITS OWN DEFECT UNDIAGNOSABLE.**
- (2026-09-01) **A REMEDY INHERITS ITS BRANCH'S ASYMMETRY.** Confirm-before-refuse is SAFE on the lock and WRONG on the probe.
- (2026-08-31, vc) **A CRITERION IS NOT REWORDED TO WHAT THE CURRENT STATE SATISFIES.** The ruled form must be HARDER.
- (2026-08-31, vc) **A CLASS CHANGE WITHOUT ITS REASON IS A DELETION WEARING A NEW LABEL.** A declared exclusion carrying its reason is the cure; a silent drop is the denominator attack.
- (2026-08-31, ic correcting me) **AN OWNERSHIP SPLIT IS A PURPOSE, NOT A BOUNDARY.** Two hands in one file IS `0206` in miniature.
- (2026-08-31, CORRECTED) **`close --note` STAYS RULED OUT ON A CONTINGENT FOOTING.** The keg has no `edit`; the ruling survives because v3.0.1 ships from the tree.
