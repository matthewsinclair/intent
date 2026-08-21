---
node: dc
name: DevX Claude
role: worker
session_id: d2fad1a7-ad92-47bc-befb-0f130c964137
heartbeat_at: 2026-08-21 15:32Z
status: paused
focus: "**LOCALFOLDED AND HOLDING FOR A BOUNCE, 2026-08-21 PM. Four commits landed and the gate moved 62 -> 63 of 67.** Mine: `ecea0eeb` fixture default, `1005ab88` int hooks foreign-tree warning, `5d2b1f0d` canon_commit_check ADMITTED AND DISPATCHED, plus `provenance_fields_check.sh` (AT-11.7) carried in cc's `d8dd6dc6`. **AC-03.6's blocker is CLEARED and the row is cc's to green -- I will not certify my own wiring.** **THREE FALSE FINDINGS WITHDRAWN, ALL FROM ONE METHOD ERROR: `int hooks` resolves from the BINARY's location, and I grepped away the line that said so.** **STILL OPEN AND MINE: AC-01.5 form 1, and the guard-resolution mechanism, which the `.envrc` did NOT close.**"
claims: [ST0056/07, ST0056/11]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260821/wip.md` -- the afternoon session is appended under its own heading. This file is the COLD-SESSION MINIMUM.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.**
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** The ordering that cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in.** **`intentdb` IS RETIRED AND NAMES NO COMPONENT.** The crates are `intent-cli`, `intentd`, `intentsvcs`; `intentsvcs` solely owns the db and `intentd` is a CLIENT exactly as the CLI is. Diagram `design.md:12-17`.

## The environment

- **v2 LIVES IN `~/Devel/prj/Intentv2`** (`v2-maintenance` at `fb45e9ea`). **Its gate was ARM C LIVE and I repaired it** -- dispatcher was absent, `gate ABSENT` -> `WIRED`, planted violation now `rc=1 / guards: 4 ran, 0 skipped`.
- **`intent` ON PATH IS v2.19.0. Drive v3 explicitly: `./native/rust/target/debug/intent`.**
- **`.envrc` EXISTS AND IS BLOCKED -- it does NOTHING until hv runs `direnv allow`.** Even then it fires only at an interactive prompt: **every node commits through non-interactive tool calls, so it is inert for us.**
- **`int hooks` ANSWERS ABOUT THE TREE THE `int` SCRIPT LIVES IN, NOT YOUR CWD.** It now warns when they differ (`1005ab88`). To ask about another clone, run ITS `bin/int`.

## DOING

**Nothing in flight. Working tree carries only peers' work. No `target/dc` exists, so prune-at-fold is vacuous for me a second day running.**

## TODO

### 1. ST0057 AC-01.5's REMEDY -- MINE, ON THE GATE, AND STILL UNBUILT

**The hole is the CLONE, not the guard.** `.githooks/pre-commit.intent` is **gitignored at `.gitignore:158`**, so no clone can ever receive it, and `.githooks/pre-commit:6` is `[ -x ]` with **no `else`**.

**DRIVEN END TO END THIS SESSION** in a throwaway clone at `510d4b10`: wired properly, a stamp with **no trailing Z** -- clock-guard check B, syntactic, no tolerance -- **committed at rc=0**, while ten parity guards printed ~60 lines of confident output. **THE HAZARD IS NOT SILENCE; IT IS REASSURING NOISE FROM A DIFFERENT ROSTER.** Two gate routes exist in that file: the chain block (shipped guards + critic) and `bin/int precommit` at `:95` (parity roster). Only the first is silent.

**FORM 2 IS ALREADY BUILT -- `int hooks` reports `gate ABSENT` correctly. AC-01.5 IS FORM 1 ONLY: make the chain fail loud.** Sites, both clear of the `bin/` collision: the generator `intent/plugins/claude/bin/intent_claude_upgrade` (`canon_emit_chain_block`, ~line 436) and the tracked `.githooks/pre-commit`. **Generator FIRST -- single template source -- or the next `claude upgrade` overwrites the instance.** Discriminate three states as `pre-commit-guards.sh` does one level down: absent / present-but-not-executable / runs.

### 2. THE GUARD-RESOLUTION MECHANISM -- STILL OPEN, AND THE `.envrc` DID NOT CLOSE IT

Recorded on hv's board as mine and OPEN. **Driven: in a non-interactive shell `INTENT_HOME` is still `Intentv2`, so this repo's guards resolve out of the frozen v2 tree.** All six guard files are byte-identical today; drift starts at the next guard change.

**MY PROPOSED FIX, NOT YET PUT TO hv: a self-hosted Intent checkout resolves guards from its OWN tracked `lib/templates/hooks/`, never from `$INTENT_HOME`.** No environment dependency, works in hooks and tests and tool calls, can name the tree it resolved from, and leaves consumer projects on the `$INTENT_HOME` path unchanged. **hv declined direnv and hand-refresh BY NAME; a wrong answer must be LOUD.**

### 3. ROUTED OUT, NOT MINE TO BUILD

- **`sync` skips untracked bytes, LOUDLY (hv ruled, I put four options).** Handed to cc 15:33Z -- **`sync` is Rust and Rust is cc's lane.** Warning must stop saying _the commit gate will_ refuse, and must not fire on the syncing node's OWN new file (staged-but-untracked and untracked-and-unstaged are different states).
- **AC-03.6 is cc's row to green.** Blocker cleared by `5d2b1f0d`; evidence handed over 15:26Z. **I will not certify my own wiring.**

### 4. MINE AND UNSTARTED

- **`cmd/macos` provenance writer** so `provenance_fields_check.sh` has a green to reach. **TRAP FOUND, NOT YET WALKED INTO: `codesign --force` REWRITES THE BINARY IN PLACE, so a hash taken at STAGE time never matches shipped bytes** (`cmd/macos:882-895`); nothing may hash until `verify_notarised` passes. **And `:1294` parses `commit:` with a `sed` -- ADD fields, never rename that one, or `publish` breaks.**
- **`thread_view_skew_check.sh` admission -- STILL HELD, AND I DECLINED hv's RELEASE ON THE MERITS.** Re-derived: release binary `2026-08-20 15:01` vs `migrate.rs`/`facade.rs` `2026-08-20 17:57`, **2h56m stale and unchanged.** The staleness refusal it is conditional on does not exist. Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`, never a copy.
- **AT-11.6 BLOCKED** on the contract conflict routed to vc. The live unattributable pair is still in the gate output every commit.
- **WP-07 hosting sweep** needs a driven re-measure through `render.rs:495`; the disposition route is a DEAD END.

## Watch-outs

**Today's instances are verbatim in `.history/20260821/wip.md`. These are the CLASSES.**

- **A FILTER THAT DROPS THE SUBJECT LINE TURNS A CORRECT REPORT INTO A CONFIDENT WRONG ONE.** `int hooks` names its tree on line 1; I grepped for `gate|WIRED` and **produced three false findings from one true report.** Before believing any tool, check it is answering about the thing you are standing in.
- **A FALSE CONFESSION IS CAUGHT BY NOTHING WE OWN** (vc's, and I was the real cause). Every rule we have guards claims that FAVOUR the claimant. **A claim that inconveniently blames you passes every check, because owning a fault reads as rigour and gets no audit.** Check a self-accusation as hard as a self-serving one.
- **A BARE `git reset` IS A SHARED-INDEX OPERATION** -- no pathspec means every peer's stage. Our rule names `--only` and says nothing about this. **`--only` DOES preserve a peer stage, both directions, driven by vc.**
- **`sync --to-disk <ID>` REWRITES EVERY ATTACHMENT FILE OF THAT THREAD FROM THE STORE.** A whole-file second writer over peers' uncommitted work. **I nearly committed ~130 lines of cc's under my name; a line-count check was the only control.**
- **CROSS-OWNER ATOMICITY IS IMPOSSIBLE WITH `--only`.** A new parity tool needs a roster row; the row lives in a peer's file; path-scoped commits cannot split them. **Expect a briefly-inconsistent HEAD and check for it: the roster reads the runner from the WORKTREE, so a split reads GREEN locally and DISAGREE in a fresh clone.**
- **A BENEFIT RECLASSIFIED AS INCIDENTAL ESCAPES THE RULING THAT GOVERNS IT** (vc's mechanism for how the `.envrc` claim passed hv's explicit decline).
- **THE ROSTER TABLE IS A SINGLE-QUOTED SHELL STRING -- ONE APOSTROPHE BREAKS THE PARSE.** I broke it twice, **the second time inside the sentence warning about apostrophes.** Count them to 0 BEFORE writing.
- **A STATUS FIELD CAN BE NAMED THE OPPOSITE OF ITS VALUE.** `direnv status` prints `Found RC allowed 1` and **1 MEANS NOT-ALLOWED.** I read the word and not the value.
- **`zsh -i -c` DOES NOT FIRE direnv** (hook is on `precmd`). Use `direnv exec .`.
- **AN EXISTENCE TEST BEFORE A PATH TEST.** `canon_commit_check.sh` lives under **ST0056**'s tools dir while covering an **ST0057** row.
- **A GREEN IS ONLY ABOUT THE QUESTION THE INSTRUMENT ASKS.** A cross-check reconciling because both sides share an error; a true measurement of a different property offered as proof; **a zero from an instrument never shown able to produce a non-zero** -- my first red-arm candidate passed under BOTH values and proved nothing.
- **CONSENSUS IS NOT CORROBORATION WHEN THE NODES SHARE A METHOD.** Four self-reports off four boards is evidence; four readings of one instrument is not.
- **DATE-ONLY GRANULARITY CANNOT SEQUENCE TWO SAME-DAY hv RULINGS** in opposite directions on one item. Attribution is solved; ORDERING is not.
- **STANDING CONSTRAINTS.** `git commit --only <paths>`, **per-file and never a directory pathspec** (a directory sweeps peers' inbox writes to you). Push `local` only; confirm with hv before `upstream`. NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. **The markdown formatter is a second writer.** **Run the suite through `tests/run_tests.sh`, never `bats` directly** -- though `test_helper.bash:93` is fixed at `ecea0eeb`, so a direct single-file run is now safe.
- **FOUR SHELL CRITIC FINDINGS ARE DELIBERATELY NOT FIXED.** `bin/intent_st:1187`/`:1208` and `bin/intent_treeindex:220` are **intentional word-splitting**; `bin/intent_st:1353` is a fragment of a multi-line `sed`. **A sweep driven to zero breaks three live paths.**

## Decisions

- (2026-08-21) **`bin/` IS dc's LANE** (hv, live channel, this session). Announce to cc before touching it. `bin/.devbin/cmd/**` is Intent's own; **`bin/devbin` and `bin/.devbin/lib/**` are VENDORED from `~/Devel/prj/Devbin`** and are not this repo's to edit -- the vendor-down is hv's to time.
- (2026-08-21) **THE GATE COST STANDS AT ~7.3s.** hv chose unconditional dispatch over a path trigger, with cc's 3.6-4.9s figure put explicitly. **Re-time before moving any row on cost grounds and name the revision.**
- (2026-08-21) **hv RELEASED ALL THREE HELD ITEMS; I ACTED ON TWO AND DECLINED ONE ON THE MERITS.** `test_helper.bash:93` landed after re-deriving what the hold asked for; `canon_commit_check.sh` admitted; **`thread_view_skew_check.sh` held, because its condition is still live.**
- (2026-08-21) **A ROSTER ROW AND ITS RUNNER MUST BE ONE COMMIT.** Either disagrees alone.
- (2026-08-21) **SYNC SKIPS UNTRACKED BYTES, LOUDLY** (hv). Canon must never name bytes no reader can obtain -- **which is AC-03.6's own subject.** Fix at the source, not the door.
- (2026-08-20) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE** (vc). Prevention and refusal are different criteria.
- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES.** Absence is decided at the filesystem, once, by the caller that touches it.
- (2026-08-20) **`CARGO_TARGET_DIR` FIXES FREQUENCY, NOT AUTHORSHIP** (cc). Only a CLEAN TREE reaches authorship.
