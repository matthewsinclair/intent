---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-09-01 09:10Z
status: active
focus: "ST0064 app pair underway (AC-01.3 M + AC-01.5 S; one app build proves both via the bundle rig; ST0064 not ST0056 so no vc window needed). Done+landed this session: 0205 refactor (rode 11528266), corpus re-pin (035114c9), coherence-trap issue 0210 (d7744332). WP-14 may return from ST0069 (L, hv owns); ST0065 + WP-17 stay hv-gated."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Cold start (2026-09-01 07:50Z, post-compact). Aggressive localfold; pre-fold verbatim + sha-verified at .history/20260901/wip-fold-0750Z.md (f00ddd62). RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this hot tree; the delivered binary lags HEAD.**

## DOING

**ST0064 app pair -- AC-01.3 (M) + AC-01.5 (S).** 01.3: add ONE real query through `intent graphql` to the app (today it uses `daemon status --format json`); 01.5: intent:// -> `intent edit --path` resolver. ONE app build (`bin/devbin macos app-build`) + ONE bundle-launch rig (scratchpad/ac0109_launch.sh) proves both. ST0064 is single-writer + a separate thread from ST0056, so no vc serialization window needed. Landed this session: 0205 refactor (rode vc's combined 11528266), corpus re-pin (035114c9), coherence-trap issue 0210 (d7744332). WP-14 (whiteboard+inboxes in the store, L) MAY come back from ST0069 -- vc's reading to hv, ownership hv's, not mine yet. ST0065 + WP-17 rework stay hv-gated.

## TODO

1. **Satisfy ST0064 01.3/01.5** (wiring landed 7e84538a; these are the PROOFS). 01.3 (M): add ONE real query through `intent graphql` to the app (today it uses `daemon status --format json`) -- design point: what query, where. RE-DRIVEN 2026-09-01: the executor is REAL -- `intent graphql '{threads{id status}}'` returns `{"data":...}` rc=0 IN-PROCESS, so 01.3 is NOT blocked on the graphql executor (that was AC-09.2's MCP tool `intent_graphql`, a different thing) and NOT on cc's daemon. 01.5 (S): intent:// -> `intent edit --path` rig-proof. Both: rig-proof via scratchpad/ac0109_launch.sh -- `defaults write com.matthewsinclair.intent.macos IntentProjectRoot <a real project>`, launch the bundle, trigger a project verb (intent:// -> `intent edit --path`), confirm it resolves THIS project + that invalid/unconfigured behave per conditions (i)/(ii). While in IntentCLI, sharpen the D07 comment to "the disagreement will never announce itself" (0206's framing).
2. **DONE -- 0205 harness refactor.** Three blocks converted to quoted-heredoc, byte-identical consumer lookups + immunity positive-control; rode vc's combined commit 11528266 (three-way ST0056 canon coherence -- see 0210). builtins:66 stayed off-queue (vendored).
3. **cc-gated 01.2/01.6:** Swift side present + correct (Health reads cc's connect-then-lock order; projection above route(); STALE!=ABSENT remedy). Needs cc's live daemon + both false-positive states CONSTRUCTED (not waited for). Coord cc.
4. **01.7 signing (dc + hv):** app-sign/notarize; dc owns the devbin pipeline + 3 header edits; notarize = hv's ADC. Reconcile `int macos sign|notarize|verify` vs `app-*`.
5. **01.4 console (cc):** tailing console on cc's `intent daemon logs`; tail-orphan trap is CLI-side, verified vs SIGTERM/SIGINT/SIGKILL SEPARATELY. Not built.
6. **`intent app start|stop|restart`** (hv, new user verb): controls the INSTALLED app; new_surface `app` family; coord cc.
7. **Explorer (hv-driven):** hv rebuild `dvb build all` + `intent explore`; open Q: does the Lotus menu SELECTION function (on_key has no MENU block)? Mark Option A RULED+LANDED in the artifact.
8. **DONE -- estate_corpus.sh re-pin retirement** (035114c9). All 4 members declare intent_version 3.0.0 at HEAD; rule retired for every member, pins unchanged (all five resolve STATE=here), only the hoist prose moved.

## Watch-outs -- mechanisms only

1. **Non-test AC closes via `intent ac satisfy`, NEVER an AT; use the ROW'S STATED INSTRUMENT.** AC-01.9 named a minimal-env BUNDLE launch, so a component test would be a substitute. Re-drive + positive-control before satisfying.
2. **Concurrent canon on SHARED threads -- TWO hazards (ST0056; ST0064 safe, single writer + thread-level race via apply_envelopes).** (a) GIT: intermixed uncommitted edits -- `--only` is path-scoped not hunk-scoped, `git add -p` unavailable; coordinate with the pen, HANDS OFF THE INDEX. (b) STORE (0206, HIGH, MEASURED 6/10 same-thread): two facade verbs each write the whole record back, the second's stale snapshot SILENTLY overwrites the first. DISCIPLINE: announce + commit in the same breath on a shared-thread canon verb. canon_race_check.sh is its harness.
3. **parity/tools: `bash -n` is NOT a check on a notes/table block** (0205) -- embedded quotes turn prose into shell, syntactically valid so bash -n passes. Source it + read stderr; keep a live pre-existing row as a control. Safe form = quoted-heredoc.
4. **The macOS app is `native/macos/Intent/` (Swift, xcodegen).** `bin/devbin macos app-build` = unsigned Debug to ~/.local/state (off-tree). IntentCLI = the one shell-out home (binary + env + child CWD); LoginShell = PATH capture; Health.decode consumes `daemon status --format json`. dc owns `int macos`; ic is the builder. A NEW swift file regenerates the tracked xcodeproj -- prefer same-file.
5. **A dated RULING RECORD is superseded, never edited.** info.md's "3.0.1" is now CORRECT AS WRITTEN (hv closed the number at v3.0.1, 09-01) -- no note, no edit. RUSTFMT before `git add` on rust. Attachment edits: `intent st attach <ST> <rel> --from <file>` (disk->store directional; running intentd auto-ingests). dispatch-table.md is generated. cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` == your set. RE-RUN TOOLS, never quote a peer's count.

## Decisions

- **ST0064 WP-01 = 3/9 (ic).** 01.1+01.8 (9cad4780), 01.9 by the stated bundle-launch instrument (d0cec969). project-CWD WIRING landed (7e84538a): IntentCLI sets child CWD to a validated configured root (ProjectConfig), condition (i) loud-refuse + (ii) interim-for-D07 in code. Launch/inspect rig at scratchpad/ac0109_launch.sh.
- **project-CWD: vc RULED (a)** -- per-app-instance configured root, NOT a machine registry. D07 ratifies a registry but it is UNBUILT (release scope, hv's); a second resolution path beside CWD walk-up is 0204's shape.
- **AC-09.6 CLOSED (rode vc's 337d6451).** Re-driven 138/60/0, positive-controlled; vc verified it survived 0206.
- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv first-hand 2026-09-01 08:32Z, verbatim at 3183990a; via vc). Everything outstanding goes in; cost is not a constraint; no external consumer. The 3.0.1-vs-3.1.0 conditional does NOT fire -- ST0064/info.md's three 3.0.1 occurrences are CORRECT AS WRITTEN, no superseding note, nothing of mine to touch. SCARCITY-REGISTER SHAPE ("not in this cut" / "after the tag" / "ship it red" / "new machinery in a tag window") is retired as a class -- do not re-litigate items in that frame.
- **hv: Option A RULED + LANDED** (f9709004 + ec9e03b9, 123 tui tests). Coloured chip already built; hv saw a stale binary.
- **ST0064 IN the next release** (hv 11:56Z); daemon is ST0056 WP-08's; signing IN (A8 reversed).
- **cc: `intent edit <address> --path` is the AC-01.5 door**; openAddress routes through it. Health is a PROJECTION above route(); STALE never unlinks (AC-08.12).
- **0210 (HIGH, mine): canon commits on a SHARED thread deadlock under intentd auto-ingest.** The `.canon/st/<ST>.json` extract names the STORE's union of every dirty-and-ingested attachment; with intentd running, one node's dirty parity/tools file blocks every node's canon commit on that thread, and committing the attachment WITHOUT the extract fails the other way. Coherent-with-git-across-all-nodes or no commit. The guard names the FILE not the culprit -> three inferences to reach "wait for a peer". Hit it live on 0205 (four nodes); resolved by vc's single combined commit 11528266. NOT established: reach beyond ST0056; behaviour with intentd stopped. **DISCIPLINE (vc, until fixed): ST0056 attachment writes are SERIALISED through vc -- announce before touching parity/tools.** Any command that regenerates the extract (even `issues add`) can dirty ST0056.json into whoever holds the window.
