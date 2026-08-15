---
node: vc
name: Validation Claude
role: validation
session_id: e48565a9-8dc8-4718-bb68-37a3462a0a36
heartbeat_at: 2026-08-15 08:44Z
status: active
focus: "hv's native/ reorganisation is in flight in cc's hands; ruled the acceptance.md rewrite anchored, not blanket. AC-03.4 ruled back to ic as a guard, not an AC. Verifying after cc's commit."
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **VERIFY AFTER cc's `native/` COMMIT.** hv ruled all Rust material moves under `native/`. Ruled to cc: one commit including acceptance.md, and the rewrite is **anchored to the AT row grammar, not a blanket prefix sed** -- 74 `crates/` occurrences, 73 on AT rows, 1 at acceptance.md:209 inside a DATED finding that must not move. Dry-run verified 73 lines, line count identical. On landing: `intent at lint ST0056`, all six gates, and specifically re-verify AC-03.7 / AT-03.9 (machine independence) and AC-03.8 (canon round-trip), because a directory move changes corpus shape and those are the two ACs whose evidence is about exactly that.

## TODO

- **WP-04 reopened at 5/6 by AC-04.6** (D32 mutation completeness). WP-06 is 4/7: AC-06.1, AC-06.3, AC-06.6. AC-06.3 is mine and ic's; the rest are cc's.
- **AC-00.1 carries the 28 deferred non-core `pending` rows.** ic's to name, gated here, not forgiven.
- **`intent/whiteboard/README.md:18` still describes cc's lane as `crates/`.** Hand-authored roster, no single writer -- needs `native/`. Flagged to cc; owned by nobody, which is why it will rot.
- **ONE QUESTION STILL OPEN FOR HV, and it is the only existential one left**: does "durable state is in the db" (D32) reverse D01? D01 says durable truth is committed JSON canon and the DB is rebuildable -- `rm intent.db` always safe, no DB migrations ever, git can review the model. Recorded as NOT reversing it, because hv's contrast was model-versus-scattered-md. Two nodes stopped on it independently. **Never settle this by inference.**
- **Two apparatus guards ruled, both ic's to build, both still unwired.** (a) `provenance_check.sh` into pre-commit -- and it is more load-bearing than it looks: **`pertest.md` cannot be re-derived from committed state by anything** (`gen_pertest.sh` needs burn.sh's uncommitted TAP), so for that one artefact the stamp is the ONLY guard in existence. (b) AC-03.4 ruled 08:43Z: a sibling `view_skew_check.sh`, **not** an AC and **not** merged into provenance_check -- different invariants behind one exit code is `intent critic`'s exit-2 overload rebuilt in new apparatus. Path-triggered, since `gen_dispatch_table.sh` reads only `$IN`. `gen_inventory.sh` does not honour `OUT`, so `cmd-*.md` is unverifiable until it does.
- **WP-10 precondition, from cc**: measure L2/L3 failures per fleet member at its named revision before ruling on whether a broken reference in a CLOSED thread carries or blocks.

## Verification kit

- Detached worktree at `$CLAUDE_JOB_DIR/tmp/vc-verify` builds the binary at any named revision, so a peer's WIP is never what gets measured.
- `$CLAUDE_JOB_DIR/tmp/v3fix` is a **migrated** v3 fixture with its own `git init` -- the only way to exercise ingest/sync/search now an unmigrated project correctly refuses. Recipe: `config.json` at 3.0.0, `st new`, prose into `thread.json` (**never** a generated view), `sync`. Issue bodies are AUTHORED under D02, so hand-writing `issues/<n>.json` + `.md` is correct, not a workaround.
- **Falsify before flipping.** Perturb the artefact the test asserts against and watch the right subset go red. Used on every AC closed this session.
- **Refuse at partial coverage.** Three times this session stopping short was right; twice a peer caught me first.

## Watch-outs

Measurement rules live in `intent/st/ST0056/parity.md` under `## Measurement rules` -- twelve now, not here. A board does not outlive the session that writes it. What follows is operational to this node.

- **THE CLOCK. Four fabrications in one session, while writing the rule, enforcing it on a peer, and citing it in the message carrying the fourth.** It is not a care problem and concentrating harder is demonstrably not the fix -- there is no internal clock to be approximately right about, so a stamp is generated like any other token unless composition is interrupted to run `date -u`. **Run the command, in its own step, and paste the output.** The structural fix is D30/WP-14 making the API the only writer.
- **Never `head` a list you are counting.** cc lost the eleventh of eleven scope spellings this way -- and a frequency-sorted list puts the RARE value last, which is the one that decides the rule. hv's version: never `head -1` when examining what remotes exist.
- **Read `$?` before anything else touches it.** `cmd | head; echo $?` reports the PAGER's exit; it manufactured two clean defects that do not exist. Redirect to a file, or `${PIPESTATUS[0]}`.
- **Scope every grep to the thing being counted, then calibrate against a known-good case.** Prose in a class-rules table has counted as data rows twice.
- **Assert the environment before measuring in it.** `$CLAUDE_JOB_DIR/tmp` is inside `/Users/matts/.claude`, which is a git repo; a fixture needing no-git needs its own `git init` or a path with no repo above it.
- **Confirming a peer's finding by re-running the peer's own command is not corroboration.**
- **Never mutate `bin/**` or `tests/**` in place** -- `~/.local/bin/intent` symlinks here AND the BATS suite reads the live working tree. Sacrificial worktrees only.
- **`git commit --only <paths>`, never `-A`** -- a bare commit sweeps a peer's staged index.
- **The machine-global gitignore ignores `*.sql`**; committed faces need their `!` exception. `git check-ignore -v` any new non-json artefact.
- **This shell is zsh**: no word-splitting of unquoted parameters; MULTIOS tees `cmd 2>&1 >/dev/null` to the terminal.
- **The live channel does not survive a peer's restart; the inbox does.** Durable copy first, live ping as accelerant.

## Decisions

Archived once a committed artefact carries them -- see `.history/`. What remains governs how this node behaves.

- (2026-08-15) **A control refuses; documentation reminds; only one is load-bearing** (cc's compression). All three nodes broke rules they had written that day; the only two mechanisms that held both refused and asked nobody to remember. Treat a rule you can obey only by concentrating as an unfixed defect.
- (2026-08-15) **Constraints are the claims most worth checking**, because they are the ones that stop work happening. I refused a re-sweep on an unmeasured cost. A finding gets scrutinised because it asks for action; a constraint gets accepted because it asks for none.
- (2026-08-15) **The convenient answer needs checking hardest**, because nothing else will check it for you. `retire` would have deleted ic's row and my question in one move, which is exactly why it needed the ratification check first.
- (2026-08-15) **Refuse at partial coverage, and say which part is missing.** Two of three sources is the AC-05.3 error wearing different clothes.
- (2026-08-15) **A count is not a diagnosis.** Ancestry and a two-way set difference settle it; arithmetic does not.
- (2026-08-15) **File a defect under its own noun, even when that reopens a closed WP.** A Done WP with a failing gate is the false green this contract exists to prevent.
- (2026-08-15) **Verify the implementation against the MODEL before calling it wrong.** cc's `collect_wp_text` was a correct implementation of a defective model; that is how D28 surfaced.
- (2026-08-14) **Verify a claim by re-running its evidence, never by reading its account.** The single claim I did not re-run became a wrong ruling.
- (2026-08-14) **The contract leads the build or it trails it, and trailing costs more.**
- (2026-08-14) **hv standing authorisation is not review, and does not reach a ratified decision.** AC-13.1 was vc-specced and contradicted a ratified D21 -- which is exactly what blocked ic's register row until hv ruled it as D31.
- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc holds the ST0056 claim as steward and does not build.
- (2026-07-02) vc fires on a close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv.
