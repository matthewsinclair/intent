---
node: ic
name: Interface Claude
role: interface
session_id: 0ccc7c30-24c1-48ce-b698-ab212286083e
heartbeat_at: 2026-08-20 17:32Z
status: active
focus: "**FOLDED HARD 17:20Z ON hv'S ORDER, THEN QUIET BUT REACHABLE while vc drives the five-language suite to zero.** Everything of mine is committed and nothing is in flight. AC-08.4 green, AC-05.3 satisfied, AC-08.5 correctly red, D57-8 amended at `c5320329`. **NEXT AND DELIBERATELY NOT STARTED: AC-07.7** -- held back rather than rushed, because its red-first arm is the thing that gets dropped under time pressure and dropping it is worse than not starting."
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT, NOTHING UNCOMMITTED.** Quiet-but-reachable per hv via vc 17:19Z; vc is driving all five declared languages plus the critics to zero and **anything of mine in `intent-cli`, `address.rs`, the parity harness or `intentsvcs/tests/` comes back to me.** **COMPACTED 17:32Z AND STILL REACHABLE** -- nothing was lost, because every trap is written into the row rather than held in memory, and the inbox is a file.

**THE WHOLE DAY IS AT `.history/20260820/`** -- `wip-1720Z.md` is this board verbatim before the fold. Every rule below is stripped to the rule; **the instance that taught it is in the archive, and a trimmed board is a reading decision, not a record.**

## ON RESUME

**`e7d038c3` is the landing, verified in a detached worktree AT the commit with zero dirty: 139 targets / 982 passed / 0 failed, clippy `-D warnings` rc=0. Nothing of mine sits with any peer.**

## TODO

1. **AC-07.7 / AT-07.7 -- THE FOUR COLLECTION FORMS RESOLVE.** Mine, minted `c5320329`, **not started.** `Threads`, `AcCollection`, `Issues`, `WpCollection` are addressable and nothing asserts their resolution; AC-07.1's population is _every ENTITY form_, so it never reached them. **Three traps, in the row rather than in memory.** (a) **The red-first arm must be `AcCollection` specifically** -- the other three ARE the POST clause's server-assigned populations, so a test sourced from that obvious paragraph reaches them and passes without ever touching the fourth, which is the form the finding is about. An AC id is author-assigned; it is the one collection that clause cannot reach. (b) **The denominator comes from the amended fence, never from `address.rs`** -- read out of the implementation it agrees by construction. (c) **`d57_8_forms()` is not its home**: growing that list to thirteen would silently move AC-07.1's population and make a satisfied criterion mean something it was never assessed against.
2. **`doctor --json`** (cc's ask): `Serialize` on `doctor::Report` plus a row in `surface/dispatch-table.json`; `Finding` already derives it. **`thread_view_skew_check.sh` parses doctor's TEXT as a dated workaround and must be DELETED when the face lands, not kept beside it.** Traps: `--json` is declared at both family and verb level where a family has a bare form, and the tool already spells this two ways (`--json` vs `critic --format json`) -- raise the divergence, do not rule it.
3. **`declared_but_unwired.rs` gets a SYNTHETIC member** (cc's ruling): `UNWIRED = &["st dehydrate"]` borrows a live defect as its fixture, so it reds for a good change.
4. **NOT MINE TO RULE:** whether the `BEGIN/END INTENT` marker grammar survives. hv left it out of ruling 4; vc raises it.

## Watch-outs

### Mechanical

- **`git commit --only` is PATH-scoped, not hunk-scoped** -- it takes a peer's unstaged work in any path it names, and two bodies of work in one file cannot be split. **Only a detached worktree AT the revision sees a broken published tree.** Never `cp` a shared source aside to mutate it.
- **The Bash tool is zsh and its cwd PERSISTS between calls** -- absolute paths always. Unquoted globs in `--include='*.rs'` are a hard error; unquoted `$var` does not word-split; `grep -c` exits 1 on zero; **never `$?` after a pipe.** **The last two compound**: a redirect plus a pipe turns a wrong-cwd miss into `rc=0` and no output, which reads as _searched and found nothing_. **An instrument that cannot say WHERE it looked cannot report an absence.**
- **`cargo test --workspace` stops at the first failing target -- always `--no-fail-fast`**, and **never pipe the log through `tail` before counting.** Write the whole log; count with `awk '/^test result:/'`.
- **Keep `CARGO_TARGET_DIR` inside the tree being built** (`native/rust/target/ic`, gitignored) -- `INTENT_HOME` walks up from the binary's path.
- **A clean `git apply --3way` is not a correct rebase** -- it reports on TEXT; only the suite reports on meaning. It also STAGES, so the following diff must be `git diff HEAD`.
- **Never drive a mutator on the live estate.** A probe is not a test and the estate is not a fixture.
- **The markdown formatter is a second writer** -- `_..._` in table prose.

### Estate

- **An attachment is authored on disk, so a divergence means the STORE is stale.** `sync --to-store <ID>` takes the disk copy and **`sync --help` says the opposite**; it TAKES ids, thread-scoped.
- **`intentsvcs` is the dependency root** -- announce the blast radius, not the files, and **announce before adding an enum variant.**
- **The live channel is unguarded** -- use commits when you need ordering you can prove.

### Judgement

- **ASK WHAT THE INSTRUMENT WOULD SAY IF THE THING IT MEASURES WERE GONE, AND COULD THIS HAVE COME BACK THE OTHER WAY.** Everything below is an instance.
- **A change that would conveniently green your own work is the one to stop and route.** The tell, not the virtue.
- **A denominator belongs to a FILE, not to a topic** -- and adjacent bullets read as cause and effect with neither one claiming it.
- **A row's TITLE can promise more than its BODY, and no instrument here reaches it.** So the discriminator _what does satisfying this row completely still leave broken?_ **must be asked against the BODY, never the title** -- against a title it returns _nothing_ every time, for exactly the rows where it matters, **and that nothing is indistinguishable from a correct answer.**
- **One planted write proves a test fires; a PAIR proves neither row is the other wearing a different id.**
- **A cross-check reconciles when both sides share the same error, and it then reads as confirmation** -- and it self-heals. **A number measured in a shared checkout is about a tree nobody else has.**
- **Two hand-written literals compared to each other observe nothing**, and **an empty gap over an unstated denominator is a vacuous green** -- state the population IN the assertion.
- **A declared list stops covering the day a variant is added.** Rust cannot enumerate variants, so **make an exhaustive match the witness** -- a new variant then fails to COMPILE where a case must be added.
- **An instrument that borrows a live instance has made the defect a fixture.** Synthesise it.
- **An absence is only evidence within the scope you actually read, and a call boundary is not a scope boundary.** **A claim carried forward from memory is not a measurement.**
- **A substring standing in for a syntactic fact is ST0039's greppable proxy one level up.**
- **A gate count mixes three kinds and only one is work**: not built, built and unverified, verified and unmoved.

## Decisions

- (2026-08-20) **Clap is the wrong layer to enforce a declared vocabulary** -- it rejects at exit 2, INV-04's USAGE code, the one the gate FAILS OPEN on. Enforce in the renderer at exit 1, reading the set from the table.
- (2026-08-20) **`intent critic`'s usage-error exit 2 is correct and stays in v3** (ic). _A gate should fail open on its own breakage and closed on yours._
- (2026-08-20) **Reporters fail open; actors refuse** (dc, on `realised_for_action`).
- (2026-08-19) **A realised artefact is one whose COVER VIEW exists, never one whose directory does** (vc).
- (2026-08-19) **Four criteria left the precondition block without being withdrawn** -- AC-03.6, AC-06.3, AC-06.4, AC-07.5. The block is about what GATES, not what is wanted; every one is still owed.
