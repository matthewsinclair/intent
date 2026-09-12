---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 14:14Z
status: active
focus: "vc's QUIET WINDOW is open and the five steps are serial and mine. Step 1 is measured and answered. DOING is step 2, WP-22, half built and banked as a patch. Then the hook, the Local shapes, the final rehearsal. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-1414Z.md`.** Everything landed today is carried by its commits and the CHANGELOG, not here.

## DOING -- step 2 of the quiet window: WP-22, HALF BUILT AND NOT COMMITTED

**THE DIFF IS NOT IN GIT.** It is `scratchpad/wp22-BANKED.patch` (313 lines, four files) and live in the worktree `scratchpad/wt-dc`, which sits at `034bf8f54`. If both are gone, rebuild from this section.

BUILT, compiling: `Op::Search { query, ask }` and `Response::Search { answer }` on the wire; the search envelope's `Deserialize`; the roster CONSTRUCTOR vc ruled; the daemon's handler calling the same `search_all` the daemonless path calls.

NOT BUILT: `--no-reconcile` on the search row (register edited BY POSITION, markdown regenerated, never prettier); the daemonless reconcile-then-query through cc's `index_refresh(None)`; the one parity arm -- same tree, daemon and daemonless, byte-identical envelopes.

**Three decisions inside it that are load-bearing and would be re-litigated if they were not written down.** `Response::Search` carries `serde_json::Value` and not the typed envelope, because a hit holds `score: f64`, `Response` derives `Eq`, and a raw float cannot satisfy `Eq` while `serde_json::Number` can -- the same reason `Response::Graphql` already carries a value. `IndexFreshness` has a HAND-WRITTEN `Deserialize` that recomputes `complete` rather than reading it, so a peer cannot send `complete: true` beside a non-empty `stale`. And `Response::search` lives in `wire.rs` rather than the daemon, because `intentd` has no `serde_json` and adding one would buy a new manifest dependency plus a second place deciding how the envelope becomes JSON.

## TODO -- vc's five steps, serial, in this order

- **Step 3: the hook (AC-24.4)**, shape in three lines to vc first. Served from the install, never blocking, reads the symbol a grep pattern named, calls `intent search --context <name> --json`, appends the structural answer, and **appends NOTHING when the envelope's `index.complete` is false**. The freshness rule is the envelope's and the hook grows none of its own. Also: cite AT-24.1 on `canon_seeds_the_mcp_declaration_once.rs`, and review ic's AC-24.6 specification (on main at `7c62a4e6a`) in ONE message saying whether its safety condition is the same one the hook enforces.
- **Step 4: the two Local shapes measured**, one build each, sizes only, announced first. (a) `fastembed = "4"` on ONNX Runtime with `hf-hub`; (b) `candle-core`/`candle-transformers` `0.9` with `tokenizers = "0.21"`. Same instrument as the grammars, and **the runtime must ANSWER rather than merely compile** or a zero delta reads as a free runtime. Report build-or-not and wall-clock beside the delta; keep the two policy points (a downloads a model on first use; neither gives TLS) OUT of the byte count -- they are hv's.
- **Step 5: the final rehearsal** on the last HEAD, `--dry-run`, every gate line verbatim, `intent backup` taken deliberately and the report saying why, `~/.intent/home` read before and after, loads stated. **vc's one-re-run rule**: a red confined to `daemon_watch`/`daemon_subscriptions` re-runs the WHOLE rehearsal once and both runs' gate lines are reported; a second consecutive red on that family halts to vc, and any red outside it halts on the first.
- **CHANGELOG**: ic writes the Added lines for the search packages; my Fixed lines stay mine.

## Holds

- **vc is DARK (banking and folding).** Every landing and every report goes into `intent/whiteboard/vc/inbox.dc.md` with a same-turn `date -u` stamp as well as being messaged. **Anything needing a ruling WAITS in the inbox and is not guessed.**
- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **THIS HOST HAS NO IDLE, AND "ALONE" MEANS ONE TARGET RATHER THAN AN IDLE HOST.** Measured in the window: with every Intent node silent, the one-minute load floors around 10 to 15 -- `fileproviderd` at 101%, iTerm, App Tamer, three CoreSimulator processes, none of them ours to pause. The daemon family reds ~3 of 6 alone at load 40 and 0 of 6 alone at load 15, so it is load and not a defect; and no measurement any node took today was on an idle host.
- **A COST MEASUREMENT IS NOT A CONSEQUENCE MEASUREMENT.** My watch-cost numbers were right about events, cached paths and wakeups, and could not have seen the ingest loop the same registration caused, because they counted paths and never ran an ingest.
- **A DEPENDENCY COMPILED BUT NEVER REFERENCED IS NOT IN THE BINARY.** `lto = "fat"` plus macOS dead-stripping drops it, so a size measurement that only adds the crate reads a real cost as ZERO. Reference it behind `env::var_os` and make the control that it ANSWERS, not that it compiled.
- **THE ARITHMETIC IS AN INSTRUMENT CHECK.** Five grammar deltas summed to MORE than a shared-runtime model allows, which is impossible -- and that impossibility is what revealed they excluded the tree-sitter runtime.
- **A FILE THAT "NEVER LANDS" STILL HAS TO BE GONE.** My measurement harness sat unregistered in `tests/` and reddened `no_orphan_suite_member` and `one_clock` four runs out of four. The guards run against the TREE, not the commit.
- **RESULTS COME BACK AS A PATCH, NEVER A WHOLE-FILE COPY** (vc, a rule of the cut). A copy silently reverts whatever landed on main while you were building, and the diff looks exactly like your own work -- mine nearly deleted cc's `suite.rs` registration.
- **A TEST ASSERTS ITS CLAIM, NOT ITS CONTAINER** (vc, a rule of the cut). Twice in one day: a bats test pinned the sentence around its claim, and `carrier_is_installed_beside_the_block` pinned a LIST's length around its claim.
- **A SETUP STEP THAT FAILS SILENTLY LEAVES AN INSTRUMENT THAT STILL ANSWERS.** `intent init --name X` is not v3's spelling; it refused at rc 1 and every `intent critic` run after it looked normal, because the rule library resolves from the INSTALL ROOT.
- **`Op::Registry` LISTS the daemon's projects and does not REGISTER one**, and a watch only starts when a project-scoped op routes there. My arm's first run reported the dispatch broken while the dispatch was fine.
- **A SECOND ENUMERATION OF A SET IS A SECOND STATEMENT OF SCOPE**, and this thread paid for it twice. Enumerate once, decide once.
- **RUN THE WHOLE REHEARSAL, NOT THE STEP YOU EXPECT TO FAIL.** The first rehearsal refused at `intent doctor` and never reached the test gate, so a real committed defect sat red on main behind an unrelated refusal.
- **A TEMPLATE OR SHELL PAYLOAD EDIT IS DRIVEN WITH THE BATS SUITE**; its text is asserted there and nowhere in cargo.
- **A DISCIPLINE ON YOUR BOARD IS NOT A FLAG ON YOUR COMMAND LINE.** I wrote "under an isolated HOME" and had set none; the phrase came off this board rather than off the command.
- **`git stash` IS SHARED ACROSS EVERY WORKTREE OF ONE REPO.** Control a diff with `git diff > patch; git checkout -- <paths>; git apply patch`.
- **NEVER run a formatter over a file you are editing by hand.** The register is edited BY POSITION and only its markdown regenerated.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call**, and on a lock refusal re-issue the SAME command. Peers land often; a retry loop is worth having.
- **Every suite and build from a private worktree with its IN-TREE target dir** under an isolated HOME, with `CARGO_HOME` pointed at the real one.
- **D42: a clock value goes into a board or a message only from a `date -u` read in the same turn's output.**
- **A GATE THAT READS A GITIGNORED PATH CANNOT BE REHEARSED IN A CLONE.** Take `intent backup` in the clone deliberately and say why.
- **The Bash tool's shell is zsh: unquoted `$var` does NOT word-split.** Messages go in a file, through `-F`.

## Decisions

- **devbin `0047` (hv, 2026-09-01): option 3, the split.** Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.
