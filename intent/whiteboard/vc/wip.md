---
node: vc
name: Validation Claude
role: validation
session_id: 1aa05d4a-6da2-4c42-98c6-de024aebab69
heartbeat_at: 2026-08-29 23:51Z
status: active
focus: "hv AFK, vc HAS THE PEN for specification, adjudication and review; cc/dc/ic build. `0148` LANDED at cd1bf7d7. hv ruled all seven decisions, every recommendation taken, recorded on hv's board with menus and provenance. Suite is ONE red and it is the ruled Machine 5 trigger."
claims: [ST0056, ST0057, ST0058, ST0060, ST0064, ST0066, ST0068]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES IN `intent/restart.md`.** Pre-fold body at `.history/20260829/wip-fold-2350Z.md`; six folds today are all in that directory.

## DOING

**THE PEN, while hv is AFK.** vc specifies, adjudicates and reviews; cc, dc and ic make it work. A workstream with a question posts it with options; anything vc and the asker cannot resolve waits for hv.

**COMMIT SEQUENCE IN FLIGHT -- three nodes, two entangled files, ordered so nobody signs anyone else's name.** `intentsvcs/src/lib.rs` declares `pub mod form` (ic) and `pub mod daemon` (cc) and NEITHER source is committed, so it can only land when both exist. `intent-cli/src/render.rs` carries five added functions across three owners. **Order: ic commits `form.rs` alone -> dc takes `render.rs` whole with their ST0066 body -> cc commits `lib.rs` + `daemon.rs` + `userstate.rs` + `cli_routing.rs`.** Each names the other's content explicitly.

## OPEN

1. **OWED BY ME, in order:** the `AC-00.4` rewrite hv assigned (`hv/wip.md` -- AC-00.4 would otherwise pass VACUOUSLY for the WP kind); **`ST0064`'s criteria, which is zero today** and whose one WP is scoped `S` against a 2,470-line Geodica reference (size comes from ic's read, never a second guess); the **`fork`/`SOCK_CLOEXEC` false positive into `AC-08.3`** (cc measured 1 in 300 with a sibling thread spawning children, 0 in 2000 without); the four missing `data-model.md` property tables (`Attachment`, `Legacy`, `Related`, `Envelope`); **ST0068 AC-02.1 and AC-02.3**; and **WP-15**, the 26-skill triage.
2. **SUITE STATE, MEASURED AT `cd1bf7d7`:** `cargo test -p intentsvcs` rc=101, **one failure**: `a_machine_ratified_in_prose_is_actually_trivial`, `left: 2 right: 1`. **That is the Machine 5 trigger firing exactly as hv ruled it should**, not a regression. The two attachment-drift reds cleared with `0148`.
3. **FOR hv WHEN BACK, and nothing is blocked on them:** the parked stack -- `0143`'s `--skip-settings`, ST0065's three questions. Everything else hv had open is ruled.
4. **A SENTENCE THAT IS FALSE WHICHEVER WAY IT IS READ.** `userstate.rs` says _the whole of Intent's per-user state is one directory an operator can inspect or delete_, meaning `~/.intent/`; D19 puts the daemon's logs and plist under `~/.local/share/intent/`. D19 wins (a numbered decision outranks a module's habit) and **the sentence still has to be fixed, because it misleads whoever writes the plist either way.**

## Watch-outs

**Mechanisms only. Incidents are in the fold archives.**

1. **A TRUE RESULT FROM AN INSTRUMENT THAT COULD NOT HAVE ANSWERED DIFFERENTLY.** The dominant class all day. Positive-control the INSTRUMENT, not the subject.
2. **A GATE THAT REPORTS ON A COMMIT CANNOT BE EXERCISED BY A WORKING TREE, AND THREE INSTRUMENTS DEFAULT THE WRONG WAY.** `machine_table_check` and `canon_commit_check` both read INDEX-else-HEAD; run bare they compare HEAD to HEAD and print a pass. **I nearly cleared dc's blocker on one such green.** `--staged` is the flag that makes the canon check answer about your commit; `DOC=`/`RS=` overrides make the machine check read the worktree without touching the shared index. **The safe-looking bare form silently answers about the wrong tree.**
3. **A BLOCKED COMMIT STILL RUNS THE FORMATTERS, SO A REFUSED COMMIT IS NOT IDEMPOTENT.** `0148`'s first attempt was refused; the formatters then rewrote `tui-design.md` AFTER the sync had recorded its bytes, and the retry failed for a DIFFERENT reason than the first attempt. **Re-sync before retrying a refused commit.** The formatter is a second writer between sync and commit and a refusal is one of the moments it writes.
4. **PROSE WRITTEN INTO A RENDERED VIEW IS DESTROYED SILENTLY BY THE NEXT SYNC.** I hand-wrote WP-17's objective into `WP/17/info.md`, whose own footer says it is rendered from the model; `sync --to-store` re-rendered it from an empty canon field and the prose was gone with nothing reporting a loss. **The ruled path is the EXTRACT.** Executed by the node that minted the criterion warning about it.
5. **A CLAIM OUTLIVES ITS BASIS AND NOTHING ANNOUNCES IT.** Fifth firing today across four nodes, including one where the stale half was an observation about a peer rather than by them. **True when measured, false when sent, with nothing in the wording saying which.**
6. **RELAYING IS AUTHORING, AND `fn fc` PROVES IT.** I told dc to name "cc's fc arm"; it is dc's, `fn fc(` appears zero times in HEAD, and **three nodes independently converged on the same wrong owner.** That is not carelessness three times -- it is a property of a file nobody can see whole. cc is the only one who measured rather than recalled.
7. **A RULE FAILS IN THE ARTEFACT THAT STATES IT.** Eight instances. The sharpest is dc's `fully_populated_row()`, whose doc comment names the exact trap for two earlier fields and which caught the next field added.
8. **A CRITERION THAT RESTATES ITS SOURCE INSTEAD OF CITING IT ROTS SILENTLY.** Four today, all mine, each caught by the node building against it: a count (`AC-08.10`), line numbers 62 lines stale (`AC-17.10`), a precis that lost a case (`AC-08.3`), a transport a later ruling moved (`AC-00.3`). **All four are now derived or cited.**
9. **A PATTERN THAT MATCHES A SUBSTRING OF A DIFFERENT WORD.** `grep -c arity` returned 241; **134 were the word `parity`.** Structural walk gave 126. Same family as `^\s*AC-` against a stream printing `ac: AC-17.7`.
10. **`$?` AFTER A PIPE IS THE LAST STAGE'S.** Fourth firing, mine.
11. **A TWO-WAY COMPARISON CANNOT ATTRIBUTE A DIFFERENCE; THE FIX IS A THIRD INPUT.** canon-vs-worktree cannot say who moved. canon-vs-worktree-vs-HEAD identified my stale `design.md` and dc's swept `data-model.md` in one pass.
12. **A CAPABILITY THAT WORKS AND IS UNLABELLED IS INDISTINGUISHABLE FROM ONE THAT DOES NOT EXIST.** `target/debug/intent` answered all evening while four nodes recorded the CLI as dead.
13. **THE ONE ACT EVERYBODY BELIEVES IS READ-ONLY MUTATES SHARED DURABLE STATE.** `cargo test` opens the live root and runs the migration ladder. **And _no downgrade_ is not _unrecoverable_: the remedy is forward and it took 58 seconds.** The severity was blast radius, never data.
14. **A SYNC WELDS A PEER'S FILE INTO YOUR COMMIT.** Canon NAMES the bytes it sweeps and `canon_commit_check` gates on it, so a peer's unfinished work becomes your blocker. **And the order is not a preference: sync FIRST, commit together -- committing first and syncing after leaves THAT commit divergent forever, because the criterion is a property of every commit rather than of HEAD.**
15. **A HUNK-SCOPED COMMIT EXISTS AND NOBODY KNOWS IT.** `git add` is file-scoped and `--only` is path-scoped, but a commit need not come from the worktree: replay your edits onto HEAD's version, `git hash-object -w`, commit that blob from a private index, and verify by grepping the result for the peers' symbols. cc built it and dropped it. **Ordering is better when the queue moves; this is for when it does not.**
16. **DESIGN AGAINST THE LARGEST REAL SUBJECT.** The strawman found ten defects on ST0056 that were invisible on ST0058.

## Decisions

**LIVE ONLY -- superseded decisions are deleted, never struck through.**

- **hv RULED SEVEN on 2026-08-29 23:38Z and every vc recommendation was taken.** Fiat edges guarded `ReasonRecorded`, `### Machine 5`, a closed Guard vocabulary; `AC-08.9` ratified and `AC-08.10` reworded to derive its count; ST0064 narrowed to the menubar app with WP-08 building the daemon; the fiat record's accepted-unverified half is a STRUCTURED field; `AC-00.3` cites D56 rather than naming a transport; **the daemon binds `127.0.0.1:0` and publishes its address, so no port literal exists**; `Facade::put` refuses to write `fiat` fields. **Full menus and provenance on `hv/wip.md`, not here.**
- **`D56`: `intentd` emits JSON ONLY and every renderer is generic.** The deciding argument is ST0064's SwiftUI menubar app, which cannot consume HTML. Recorded in `design.md` and `tui-design.md` 10a.
- **Conflab is the daemon template; Geodica is its simplified second iteration.** `conflabd` is 54k lines of Rust with a 10.3k-line mgmt plane on loopback TCP. **Its authn/authz does NOT transfer** -- that exists to bridge a browser app to cloud `conflabc`, and there is no `intentc`. **What does not drop out with the cloud: a loopback TCP port is reachable by every process on the machine, so the HTTP half carries one auto-generated token and the socket half carries none.**
- **`intent browse` SHIPS.** `ST0058 AC-00.6` refuses a flag and its twin DISAGREEING about whether a capability exists, not both existing.
- **ic's register questions resolve with ZERO new fields:** `invariants` already houses cross-row rules; `AC-00.6` generalises to _every spelling of one capability agrees about whether it exists_, covering the sub-row `fc` split; `arity` already exists on `args` with `0..1` in its closed vocabulary and needs extending to `flags`.
- **Announce a write to a SHARED file to everyone; announce a write to a CLAIM to the claim-holder.**
- **An opening announcement states the PROPERTY that ends it, not a promise to send a second message.**
- **Docs are written against the CUT, never against `main`.** Em dash in prose pages; `--` in generated reference pages.
