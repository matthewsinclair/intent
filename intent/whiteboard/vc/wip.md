---
node: vc
name: Validation Claude
role: validation
session_id: e48565a9-8dc8-4718-bb68-37a3462a0a36
heartbeat_at: 2026-08-15 01:26Z
status: active
focus: "hv AFK. D30 + WP-14 cut. Five gates PASS; WP-06 4/7. 31/93, 93 AT rows. New for hv: does treeindex port (D21) or retire (AC-13.1)?"
claims: [ST0056]
---

# Validation Claude (vc)

## DOING

- **Working autonomously; hv AFK until morning.** Board kept to pointers now that D30 contracts the bound -- detail lives in the artefact, not here.

## TODO

- **WP-06 is the only blocked gate**: AC-06.1, AC-06.2, AC-06.3, AC-06.6, AC-06.7 -- all cc's. **AC-06.7 is one step away**: its canon and search arms are both verified; it needs a `WP/<NN>/info.md` view to exist so canon -> view -> canon has a view to pass through.
- **AC-00.1 carries the 28 deferred non-core `pending` rows.** ic's to name, gated here, not forgiven. Check it is still on the row at WP-12.
- **For hv**: `intent ac` has no path from satisfied back to unsatisfied -- `satisfy` is one-way and `rescope`/`reinstate` only undo a descope, so I hand-edited a line the CLI exists to own. v3's `ac` surface should carry it. I carry this; ic is not duplicating.
- **For hv, disclosed rather than discovered**: I pushed 155 commits to `local` (Dropbox) on the recorded standing authorisation "push authorised as soon as it makes sense", and named that authorisation to hv at the time. `upstream` (public GitHub) is still 155 behind and I deliberately left it -- a public push mid-rewrite is outward-facing in a way a Dropbox folder is not. **cc flagged that tonight's ask was for COMMITS and did not mention pushing**, which is fair: the authorisation was standing rather than given tonight, and anything touching a remote is worth hv hearing from us rather than finding. It is in both our closes.
- **FOR HV, NEW AND THE SHARPEST OF THESE: does `treeindex` PORT or RETIRE?** D21 -- an hv WP-01 closure -- says "the treeindex cache location is unchanged **until WP-06 ports the command**", so it assumes a PORT. AC-13.1 retires it whole, and AC-13.1 is **vc-specced under standing authorisation, not ratified**. Standing authorisation does not reach a ratified decision, so I cannot settle it. One answer settles ic's blocked register row, WP-06's port list, and WP-13's T0. **This one has a cost clock on it, which is cc's reason and puts it first rather than merely sharpest**: they reach the tool-infrastructure families in AC-06.1 shortly, `treeindex` and `fileindex` are both in that block, and porting a command that is retiring is work that then has to be un-done. **Blast radius is ONE CLAUSE and AC-01.4 does NOT reopen** (ic's scoping, verified): D21's actual decision is `intent/.cache/` gitignored whole-dir with the DB inside, and that stands either way -- what conflicts is its subordinate forward-looking note about someone else's future work. D21 needs one clause amended after hv rules; the closed AC that rests on it is untouched.
- **For hv, v3 acceptance surface (two, both found by using the apparatus)**: `intent ac` has no path from satisfied back to unsatisfied; and `intent at green` checks a cited file's EXISTENCE, never that it is committed (`bin/intent_acceptance:1337`), so an AT can go green on a file living in one working tree. `git ls-files --error-unmatch` is the cheap stronger predicate; it also cannot tell a real file from a symlink.
- **Still unfolded for hv**: the D01 reading (recorded as runtime, reversible in one line); `intent critic`'s exit-2 overload, ahead of the 15-row usage bundle because it is the only one with a live consumer; whether `fileindex` follows `treeindex`; whether `todo --flush` survives, which decides whether the watermark field exists; WP-06's name.
- **WP-10 precondition, from cc**: measure L2/L3 failures per fleet member at its named revision before anyone rules on whether a broken reference in a CLOSED thread carries or blocks.

## Verification kit

- A detached worktree at `$CLAUDE_JOB_DIR/tmp/vc-verify` builds the binary at any named revision, so cc's WIP is never what gets measured. `git worktree prune` clears it if the job dir goes.
- `$CLAUDE_JOB_DIR/tmp/v3fix` is a **migrated** v3 fixture with its own `git init` -- the only way to exercise ingest, sync and search now that an unmigrated project correctly refuses. Recipe: `config.json` at 3.0.0, `st new`, prose into `thread.json` (**never** into a generated view), `sync`. Issue bodies are AUTHORED under D02, so hand-writing `issues/<n>.json` + `.md` is the correct fixture method, not a workaround for the unwired verbs.
- Falsify before flipping: perturb the artefact the test asserts against and watch the right subset go red. Used on AT-06.5 (a committed face), AT-03.8 (an unset model field), AT-06.4 (a before/after across the fix itself).

## Watch-outs

Measurement rules earned on this thread live in `intent/st/ST0056/parity.md` under `## Measurement rules`, not here -- a board does not outlive the session that writes it. What follows is operational to this node.

- **Read `$?` before anything else touches it.** `cmd | head; echo $?` reports the PAGER's exit. It fired three times in one session and manufactured two clean defects that do not exist (`intent search` "exiting 0 on a usage error"; `ac gate` "printing BLOCKED and exiting 0" -- both are exit 1), each one send away from reaching cc as a bug in their code. Redirect to a file, or use `${PIPESTATUS[0]}`.
- **Scope every grep to the thing being counted.** `grep -c UNCLASSIFIED` counted the class-rules prose as data rows; `find | wc -l` counted `COMPLETED/` threads that v2's default view excludes, inflating a real finding by 194; a `list`-anywhere match swept ten `claude rules list` files into a core-family count. **All of them made a finding stronger.** Anchor the pattern, then calibrate it against a known-good case before believing it.
- **Confirming a peer's finding by re-running the peer's own command is not corroboration.** cc reported `intent/steel_threads.md` absent; I "confirmed" it on the same wrong path and ruled on it. The file is at `intent/st/steel_threads.md`.
- **Correction standing on the record: `8abbbaf`'s message claims it also committed the formatted form of two files; it changed one.** The mechanism that message then offers -- `git commit --only` bypassing a pre-commit hook's `git add` -- is **unverified and probably false**. Not amended: rewriting shared history for a cosmetic fix while two peers commit is the worse trade.
- **Whiteboard stamps are READ from `date -u`, per stamp, always -- and I have now failed at this FOUR times in one session** (`00:03Z` vs a real `00:00Z`, refused by the guard; `00:47Z` vs `00:40Z`; `01:25Z` written before a clock that said `01:24Z` and which happened to become true, which is luck rather than correctness). Writing the rule, enforcing the rule on others, and citing the rule in the same message did not stop it. **That is not a discipline problem to try harder at; it is the argument for D30 making the API the only writer of a timestamp**, and it is the strongest evidence WP-14 has.
- **Re-read an inbox from disk immediately before appending** (cc's rule). It is the recipient's to empty.
- **Never mutate `bin/**` or `tests/**` in place.** Two mechanisms: `~/.local/bin/intent` symlinks into this repo, AND the BATS suite reads the live working tree (`no_absolute_home_paths.bats:37,100,103`). Sacrificial worktrees only.
- **`git add <paths>` + bare `git commit` commits the WHOLE INDEX.** Use `git commit --only <paths>` verbatim, never `-A`. It has already cost once.
- **The machine-global gitignore ignores `*.sql`** -- committed faces need their `!` exception; `git check-ignore -v` any new non-json artefact. Verified still true after adding `intent/.cache/`.
- **This shell is zsh**: no word-splitting of unquoted parameters, and MULTIOS makes `cmd 2>&1 >/dev/null` tee stdout to the terminal.
- **The live channel does not survive a peer's restart; the inbox does.** cc's session vanished from the socket roster mid-send and reappeared renamed. Write the durable copy first and treat the live ping as the accelerant, never the delivery.
- Release-window mechanics live in `intent/restart.md`'s checklist.

## Decisions

Working decisions are archived once a committed artefact carries them -- see `.history/` for the trail with the file that carries each. What remains governs how this node behaves rather than what any file says.

- (2026-08-15) **A count is not a diagnosis.** 97-against-98 was a true number and an invented cause; the register was complete at the revision it named. Ancestry and a two-way set difference settle it, arithmetic does not. My single worst call of the thread came from reading a count and skipping the check.
- (2026-08-15) **File a defect under its own noun, even when that reopens a closed WP.** AC-03.7 reopened a Done WP-03 rather than ride into WP-06 as a convenient new AC. A Done WP shipping an ingest broken on every Mac is a false green, and filing under a convenient noun is how L4/L5 nearly shipped missing at WP-04.
- (2026-08-15) **Verify the implementation against the MODEL before calling it wrong.** cc's `collect_wp_text` looked like an AC-wording problem and was a correct implementation of a defective model. The bug was one layer down, and going in expecting to find a wording fix is what surfaced D28.
- (2026-08-15) **When a rule has a precondition, land the precondition or the rule is theatre.** D29 excludes gitignored paths; `intent/.cache/` was not gitignored, so D29 would not have covered the DB it was written to protect.
- (2026-08-14) **Verify a claim by re-running its evidence, never by reading its account.** Both directions in one window. The single claim I did not re-run became a wrong ruling.
- (2026-08-14) **The contract leads the build or it trails it, and trailing costs more.** Rulings surfaced by builders inside the first hour of touching code were invisible from the documents alone. A contract is a live artefact during a build, not a gate before one.
- (2026-08-14) **hv standing authorisation is not review.** "Go with your recs, unless they're existential" authorises proceeding; it does not record hv reading each ruling. It does not reach a ratified decision either -- D01 stays hv's.
- (2026-08-14) **hv ruling: cc and ic write the code; vc ensures.** vc holds the ST0056 claim as steward and does not build. The one exception taken this session was a one-line protective `.gitignore`, on an asymmetry: a binary DB in git history is expensive to undo, the line costs nothing.
- (2026-08-14) **hv bounce rulings, still standing**: `corrected` parity class ratified; migration carry policy ratified (CLOSED threads lossless-by-carrying, LIVE threads BLOCKED-until-clean, neither ever lossy); `organize` planned vestigial; push authorised as soon as it makes sense; v2 maintenance default-defer, show-stoppers only.
- (2026-07-02) vc fires on a close/green/freeze trigger or an hv request, never on in-flight edits. Advisory only; findings to the owner's inbox, compounding risk to hv; vc claims no STs (ST0056 stewardship excepted).
