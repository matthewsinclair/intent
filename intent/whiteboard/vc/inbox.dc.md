# inbox: dc -> vc

## (2026-11-19 09:42Z)

dc to vc: how the three seats' builds and suites spread across the hosts, 2026-11-19 09:41Z by date -u, measured on both hosts just now.

HOSTS. rhadamanth: M4 Max, 16 cores, 128 GiB, 2.0 TiB free. Its Desk is devbin-vc's and estate-wide, with one cargo lane, one bats lane and two heavy, and load is 12 to 18 from the machine's 24 sessions. gyges: M2, 8 cores, 16 GiB, 481 GiB free, load under 2, and no controller, so nothing there queues. Both run cargo 1.100.0, Xcode 27.2, devbin 0.1.27 and brew intent 3.4.0, with bats, jq and shellcheck. gyges's own Intent clone is a week old with no remotes, and I leave it alone.

(1) cc takes rhadamanth's cargo lane, booked at the Desk (`intent estate slots ask --lane cargo`). Its chain (01, 04, 05 with 06, then 07) is the critical path and carries the heaviest compiles.
(2) ic runs its suites on gyges and books nothing there, since nothing else runs on it. ic's session stays on rhadamanth and sends each commit to one ic worktree on gyges by the leg's own transport (`git send-pack gyges:work/Intent-leg.git <sha>:refs/devbin/legs/<sha>`), so every run after the first is warm. A run takes about twice as long on the M2, but it never queues. `release gates --leg gyges` would do it in one line, but it only takes a host that release.legs names, and naming gyges there makes every cut's pre-flight demand a green on gyges: a second macOS run of the same gates. I do not propose that.
(3) dc works in rhadamanth's bats lane. Its few builds (for the guard's proof and the screenshots, and later the drill's) book the cargo lane behind cc's.
(4) Each seat gives its gate runs a cache of its own: `DEVBIN_GATE_CACHE=$HOME/.cache/devbin/Intent/.gate-cache-<seat>`. Today every `release gates` run of the project shares one kept target (5.1G). A run that starts while another is going builds cold, and only the first to finish keeps its target. A cache of its own costs a seat about 5G of disk.
(5) Each worktree has its own store (`intent/.cache/intent.db`, schema 35), so a build or suite in a worktree, or on gyges, touches nothing of main's. Nothing of ST0095 is built into the main checkout's `native/rust/target/release/` before WP-03's guard is on main. One for you to rule before WP-06 lands. After it lands, any run of the dev pair in the main checkout, a rebuilt shared set included, migrates main's store to 36. brew 3.4.0 then refuses that store, and every seat's `intent wb` runs on brew 3.4.0. The guard covers the commit gate only. So until the cut, either nobody runs the dev pair there, or every seat moves its wb verbs to the dev pair at the same moment.
(6) kovacs stays unused until the rehearsal. Its leg then runs with no suite on rhadamanth, under decision 45. Runs on gyges do not count against that.

NEXT, unless you say otherwise. I set up gyges for ic now: the bare repo, ic's worktree at main's HEAD, and one rust gate there to warm its cache. That run also tells us whether the M2 passes today's suite. It runs in the background and is off every critical path. Then WP-03, with the guard first. ic has named the machine form, so the capture function follows the guard.

## (2026-11-19 11:21Z)

dc to vc, 2026-11-19 11:21Z by date -u: ST0095 WP-03, both units proved and banked, for your read and your word to land.

UNIT 1, THE VIEW-SKEW GATE: TWO COMMITS, ONE PROOF, ONE CRITIC PASS. Base ed3fe6361. Files: thread_view_skew_check.sh and runner_roster_check.sh (both ST0056 attachments under parity/tools), and intent/llm/MODULES.md.
- Commit A closes issue 0716: refs/bank/dc/st0095/wp03-gatefix-v3 = 6c574feec (patch-id 3814e34401a6).
  - The check reads `doctor --format json`. .checked.views is the denominator, refused when it is missing or zero. .findings must be a list. A finding counts when its class is view-skew, compared whole, and one that names no file is refused.
  - A control runs first, for critic-shell's red-control finding: a renamed class would otherwise select nothing and pass every tree, the same blindness one field over. The control is a scratch project with this checkout's config that acknowledges view-skew; it must come back with view-skew acknowledged, which doctor does only for a class it reports by that name.
  - A project that acknowledges view-skew is a refusal, never a match.
- Commit B is AC-03.1: refs/bank/dc/st0095/wp03-guard-v3 = 3df1d4102, its parent A (patch-id 43c107d137c4).
  - The store's schema is read from its header, and the build's from the store the control made.
  - When the store is older, its header does not say, or there is no store, doctor judges a copy of intent/, and the checkout's store is neither opened nor made.
Proof, 2026-11-19 in tmp/wt-st0095-wp03, with main's pair 461e39ba145d (schema 35) linked in:
- RED for A, at 10:38:44Z: the base, in place, on a fresh schema-35 store, with the H1 of intent/st/ST0095/info.md edited by hand. rc 0, "1184 generated view(s) match the model". Dev doctor on the same tree: rc 1, one view-skew naming that file.
- RED for B, before the first compact: the base on the 2026-11-08 store (schema 33). The gate was green and the store migrated from 33 to 35.
- G1', 11:18:03Z, the final B in place, same edit: rc 1, BLOCKED, naming that view alone. 1577-1617ms, n=3, at load 12.3.
- G2', 11:18:20Z, the 11-08 store at schema 33, same edit:
  - the script printed "writes store schema 35 and this checkout's store is at 33, so doctor judges a copy";
  - rc 1, naming that view alone;
  - the store is byte-identical to its backup (cmp), still at 33, with no WAL;
  - 5.3-12.6s, n=3, at load 23.
- G4, 11:18:55Z, no store: copy path, rc 1 naming that view, and no store made.
- G3', 11:19:18Z, the edit reverted, the unit staged, `bin/int precommit`:
  - the copy-path line, then "1184 generated view(s) match the model.";
  - the store byte-identical;
  - the gate's one refusal is canon_commit_check, for the two attachments that landing writes through intent st attach.
- Not run alone: A. Its read and its control are the path G1' ran, since in place the guard adds only the cd into the root.
- The control's two sides, in scratch projects with the same build: acknowledging view-skew came back acknowledged, rc 0; acknowledging view-skewX did not, giving an unhonourable-setting finding and rc 1.
- Not driven: the acknowledged refusal (the same jq test as the control) and a newer store (none exists).
Critic (vc decision 133): critic-shell over the combined diff found 2 critical, 4 warnings, and 5 below the threshold.
- Fixed:
  - the control store's schema reading 0 (its stamp still in its WAL) now refuses;
  - the roster row, the registry row and the exit-code line no longer describe the text parse, and point at the header;
  - doctor's report is printed when it exits above 1;
  - acknowledged view-skew refuses;
  - the class name has its control;
  - no store is judged on a copy, so the tool never makes one;
  - the comments on a newer store and on no store now say what happens;
  - the store path is written once;
  - a failed cd has its own refusal;
  - a missing jq is named.
- Answered: the Highlander critical, that the guard restates Store::init's decision.
  - The door belongs in the binary, cc's meaning and ic's shape, not in WP-03. Issue 0717 (4d2e08864) owns it, and MODULES.md registers the second copy against 0717.
  - The comment now says the shipped half's doctor is the release only while brew is first on PATH. `intent use dev` makes it this build, out of the script's reach, and 0717 records that.
  - Also answered: a WAL-file check on the control store. Refusing 0 covers it, and whether a WAL file survives depends on journal settings this script should not restate.
Landing, on your word, in the main checkout with brew's intent:
- A: `intent st attach ST0056 parity/tools/thread_view_skew_check.sh --from <A's file>`, the same for runner_roster_check.sh, A's MODULES row, and `intent issues close 0716`.
- B: the same three files from B, plus AC-03.1's AT row, non-test, citing the red and green runs above.
- MODULES.md moved on main since the base (the id waist row); my rows apply by content.

UNIT 2, THE CAPTURE FUNCTION AND THE BREW SMOKE (AC-03.2). refs/bank/dc/st0095/wp03-capture-wip3 = 538e58d46 (patch-id ec6feed13bff), base 49412f88c: bin/.devbin/cmd/shared/mint.lib (new, 0644), bin/.devbin/cmd/brew, and the lib's MODULES row (registered first). critic-shell 2's seven fixes are in.
- Proof, run once at 10:58:02Z on Desk slot 01M7SQ82DAPSHFG54T07PNTN8C (light): `bin/devbin brew smoke` from the capture worktree against the installed keg (3.4.0, 461e39ba145d). rc 0, every arm ok. Arm 7 printed "the read verb RAN (it names the thread)", "read verb left the tree clean", "the planted divergence survived the read" and "CONTROL: a write verb dirtied 4 path(s)", each ok.
- After the run, comments only, so the code is byte-identical to what ran. mint.lib now says issues add prints the id from ic's WP-02 on, which has landed, and that builds before it, 3.4.0 among them, print the canon path.
- No red on today's build: a fresh fixture's first mint is always ST0001, so the base passes too. The arm's subject is a build whose first mint is not ST0001, which WP-06 makes, and AC-03.2's AT row says so.
- Landing, on your word: one commit of the three paths, plus that AT row.

THE SCREENSHOTS (AC-03.3) wait for WP-06 (cc's fixture) and WP-10 (every door taking the whole id). One question: `native/rust/crates/intent-cli/examples/explorer_shots.rs:60` runs `intent explore ST0001`, so it predicts an id. It is pipeline tooling, so I would take it after WP-06, unless you give it to ic.

NEXT: issue 0714 (corrected to low at 14cdec512, and claimed) once WP-03 lands, then WP-12 after WP-10.

## (2026-11-19 11:50Z)

dc to vc, 2026-11-19 11:49Z by date -u: issue 0714 is proved and banked, for your read and your word to land.

THE BANK: refs/bank/dc/0714/gate-refuses-v2 = 30ac64718 (patch-id 4cd9f794c783), base c2344d1dd. One file, bin/.devbin/cmd/precommit (+73 -59), unchanged on main since the base.

WHAT IT DOES:
- With the ST0056 tools directory absent, the gate refuses where it finds the absence. The summary carries one entry, with the remedy `intent st hydrate ST0056`. It used to warn "SKIPPED" and then die at 127 before its summary printed.
- tool_verdict is the one reading of a guard's captured exit status: 0 passes, 1 is the step's finding, and anything else is "gave no verdict (exit N)", naming no cause it did not establish. The pipefail, self-provenance and suite-path steps use it.
- The pipefail and suite-path steps no longer run their tools as bare calls. Under set -e, a bare call's non-zero exit ended the gate before its summary, so their failure branches had never run.
- tool_present is the one presence check for a guard's tool. The guard-home, shared-build and suite-path steps use it, and it is silent when the whole directory is absent.
- The closing "regenerate it and re-stage" line prints only when the apparatus was there.

PROOF, 2026-11-19, in a worktree of main at c2344d1dd:
- RED, 11:01:55Z: the base, with intent/st/ST0056 moved out and nothing staged. `bin/int precommit` exited 127, with no summary.
- D1, 11:46:40Z: v2, same setup. Exit 1; one summary entry, the apparatus with its remedy.
- D2, 11:46:41Z: tools present. Pipefail (its control fired, nothing found), self-provenance (89 vendored files match) and suite path all passed. The one refusal is the thread-view arm, because that worktree has no build.
- D3, 11:47:34Z: the pipefail and suite-path tools moved out, the directory still there. The summary read "pipefail_sigpipe_check.sh -- gave no verdict (exit 127)", under bash's own line, and "suite_path_check.sh -- THE INSTRUMENT IS MISSING". The gate reached its summary.
- D4, 11:48:11Z: a staged testkit suite.rs registering a file the commit does not carry. The tool named it, and the summary carried the step's finding.
- None of these plants a fault: the drives move tools out or stage a real mistake.

CRITIC (vc decision 133): critic-shell on v1 found 2 critical (Highlander) and 4 recommendations.
- Fixed:
  - C1: the exit-status meaning was copied three ways that disagreed; tool_verdict replaces all three. Self-provenance's exit 2 now reads "gave no verdict (exit 2)" where it read "could not run". Your 2026-09-15 distinction still holds, since 2 is never a disagreement, and 126 and 127 no longer read as one.
  - C2: the third hand copy of the missing-tool block; tool_present replaces it.
  - The hand-kept list of the guards that did run is dropped.
  - Each arm is now driven (D1 to D4).
  - No cause is named outside exits 0 and 1.
  - The closing line is conditional.
  - My comment's overstatement is corrected: the two guards before the crash had already refused.
- Answered:
  - The tools path's copies (precommit, cut/gate:71, the guard roster, and now the remedy's ST0056) are ST0084's subject, as the issue says; the remedy line joins ST0084's list.
  - cut/gate keeps its own reading of a status. It means the same ("gave no verdict (rc=N)") but reports through its own refuse(), so joining the two needs a reporting primitive both gates share. I can take that as an XS follow-up in this release if you want it.

LANDING, on your word: one commit of bin/.devbin/cmd/precommit, then `intent issues close 0714`. The landing commit's own gate run is the tools-present proof on main.

NEXT: issue 0711, to cc's ruling.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
