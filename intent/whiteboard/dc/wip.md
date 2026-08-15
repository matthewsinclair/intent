---
node: dc
name: DevX Claude
role: worker
session_id: 482cf2fc-6b49-4a0d-8d76-38b3c981924c
heartbeat_at: 2026-08-15 14:45Z
status: paused
focus: "PAUSED after localfold. WP-11 claimed and WIP: AC-11.2 CLOSED end to end -- both binaries signed AND notarised (Accepted), proven from a quarantined copy. Next on the bounce: cargo-dist + tap (AC-11.1), and the cross-platform signing seam."
claims: [ST0056/11]
---

# DevX Claude (dc)

## THE TRUTH MODEL -- canon, ratified, in my own words

hv reversed D01 on 2026-08-15 and vc has rolled it out. This is what I hold, and I hold it in preference to anything earlier on this board or in my head.

1. **The intentdb IS the durable single source of truth. Nothing on disk is truth** -- not `thread.json`, not the `.md` views, not `events.jsonl`. They are secondary artefacts of the same kind, so there is no Highlander contest between them: none of them is a competing claim.
2. **All of `intentsvcs` works FROM the db.** Not from files with the db as an index.
3. **Sync runs BOTH ways** -- disk-to-db and db-to-disk -- manual or daemon-triggered.
4. **The typed Rust API is the ONLY door in.** That is why db contents conform to the schema **by construction** rather than by anyone checking. Structural, not procedural -- the same distinction as a guard that refuses versus a doc that reminds, which is the thing I care most about in my own lane.
5. **Re-creating the db from an extract is a CAPABILITY, not a licence to treat the db as disposable.** Being able to rebuild a thing is not the same as it being safe to destroy.
6. **Ingest of a well-formed `.md`/`.json` yields well-formed db items ONLY because it passes the HARD GATE of the intentsvcs API.** The gate does the work; the file format does not.
7. **MIGRATIONS ARE NORMAL.** "No DB migrations, ever" is DELETED -- hv never asked for it. Anything justified by "we can never migrate" is resting on a constraint nobody made.
8. **The real standing requirement is PLATFORM AND DATA-MODEL OPENNESS** (AC-02.6): always a 1-1 mapping between db entities and an equivalent `.json`/`.md`, lossless, **usable without Intent**. That is what bidirectional sync is FOR -- never being locked in. Not backup, not disposability, not migration-avoidance.

**The state machines are RATIFIED too** (ST / WP / AC, `data-model.md`). `st new` enters at `Triage`; no terminal states; WP has no Hold/Cancelled; AC collapses two fields into one four-valued enum. `wp done` is refused on a BLOCKED gate AND `doctor` reports status-disagrees-with-gate, because **a status that was true when set becomes a false green the moment its contract grows.** New verbs are red tests now: `st triage/hold/resume/reopen/reinstate`, `wp reopen/unstart`.

## DOING

- **Nothing in flight.** Localfolded at hv's instruction; WP-11 continues on the bounce from the TODO below.

## TODO

0. **WP-11 (Distribution) -- MINE, WIP. CONTINUE HERE ON THE BOUNCE.**
   - **DONE and closed: AC-11.2.** Both binaries Developer ID signed (Geodica `76BQL8L47U`) AND notarised -- Apple returned **Accepted**, and `int macos verify` proves it from a quarantined copy (`source=Notarized Developer ID`). `int macos <doctor|sign|notarize|verify|env|store-creds>` is ported from Lamplight. **CI needs no Apple secrets** -- Lamplight signs locally and so do we.
   - **DONE: the release profile.** `lto=fat`, `codegen-units=1`, `strip=debuginfo`; `intent` 9,949,792 -> 8,084,128. Rejected `strip=symbols` (7,096,576) on IN-AG-NO-SILENT-001 grounds -- 988 KB buys 10,064 named frames over 144.
   - **DONE: one-vs-two binaries** confirmed against D18 and the workspace.
   - **NEXT: AC-11.1 -- cargo-dist wiring + the tap formula.** Nothing is wired yet (`grep cargo-dist` finds nothing anywhere).
   - **THE OPEN DESIGN QUESTION, and it is the reason AC-11.1 is not mechanical: THE CROSS-PLATFORM SIGNING SEAM.** Signing is LOCAL and macOS-only, but cargo-dist builds every target in CI on tag push. A Linux artefact needs no signature; a macOS artefact built on a CI runner cannot be signed by a key that only exists on this laptop. So either CI builds Linux while macOS is built-and-signed locally, or CI builds everything and macOS artefacts get a local signing post-step before the release is published. **Do not wire cargo-dist before answering this** -- `dist init` generates a release workflow, and generating one that publishes unsigned macOS artefacts on tag push is the failure worth avoiding.
   - **HELD, and must NOT land before WP-12 cutover: `int build release` gaining `Cargo.toml` to its sidecar sync.** Correct for a v3 release and WRONG today -- the Rust workspace is versioned independently at `3.0.0-dev`, so wiring it now would make a v2 release (say 2.19.1) stamp `2.19.1` into `Cargo.toml`. The decision is made; the wiring is gated.

1. **Issues 0030 and 0031 filed against `intent upgrade`, DEFERRED not done.** Both are `bin/**` v2 edits under hv's DEFAULT-DEFER, and neither is a show-stopper because the namespace rule already contains them.
   - **0030 (medium): `intent upgrade` stamps `date +%Y%m%d-%H%M%S` -- LOCAL time** (`intent_upgrade:117`). Does not sort chronologically across a DST fall-back, so an oldest-first retention deletes the newer artefact. **Latent only because nothing sweeps `.backup/` root**, which is exactly what my layout rule guarantees; it goes live the moment anyone extends retention to `upgrade/`.
   - **0031 (low): `--backup-dir` basenames straight into `.backup/`**, so `--backup-dir db` lands a rollback artefact inside cc's snapshot namespace. **The one collision that survives the layout**, because the layout confines mechanisms to directories and this flag lets a user put one inside another's.
2. **Release mechanics -- now specified, sequenced behind WP-10.** Versioned schema and upgrade paths (migrations are normal, every consumer's db must survive a bump); `intent upgrade` taking a D35 snapshot before it mutates; and **a clone is now a rebuild**, so "does a fresh clone reconstitute its DB through the ingest gate" joins fresh-clone-and-build as a release check. vc measured the live DB at zero model rows, so this is a WP-10 precondition and not an emergency -- I am not front-running it.
3. **`intent/.cache/` is a name that contradicts the model.** cc's under D21, explicitly not ruled. Raised twice now; not mine to move.
4. **`core.hooksPath` adoption -- open for hv/cc, technically unblocked.** `.git/hooks/` is never tracked, so a fresh clone gets every guard and nothing invoking them. `int hooks` makes that VISIBLE; it does not close it. What remains is only that `lib/templates/` is cc's lane.
5. **`bin/` boundary** stays open for hv (cc's split adopted as proposed).
6. Issues **0026**, **0027** are cc's under DEFAULT-DEFER; **0028** (stale index) is one sentence of documentation touching every node's commit habit; **0029** is cc's decision, not just cc's fix.

## Watch-outs

Facts about this estate, not reminders. Everything amounting to "remember to" is worthless here -- three nodes broke rules they had personally written, on the day they wrote them.

- **A BARE MACH-O BINARY CANNOT BE STAPLED, and `spctl -a -t exec` REPORTS "rejected" ON A CORRECTLY SIGNED CLI.** Both look like defects and neither is. `stapler` writes tickets into `.app`/`.pkg`/`.dmg`; there is nowhere to put one in a bare executable, so the ticket lives on Apple's servers -- conflab has shipped that way since July. And spctl's `-t exec` policy is for app bundles, so it answers "does not seem to be an app" with a perfectly valid signature attached. **`codesign --verify --strict` is the check that means anything here.**
- **READ THE WHOLE OUTPUT, NOT ITS LAST LINE.** I reported `spctl` "rejected" for two binaries as one finding to vc; they were a trust failure and a category error with identical first words. Truncating to `tail -1` hid the clause that distinguished them. Same shape as taking `conflab` off `PATH` when the question was about the Cellar artefact: **a short answer that fits the expectation is the one to look at twice.**
- **NEVER `git pull --rebase` IN THIS SHARED TREE.** I ran it reflexively before a push and it **refused, because the index held peers' uncommitted work** -- which is the only reason it was harmless. A rebase in a tree three other sessions are working in rewrites history under them while their edits are unstaged. The push had nothing to rebase onto anyway. **Push; if it is rejected as non-fast-forward, coordinate -- do not rewrite.**
- **A peer's `.git/index.lock` means a peer is running git. WAIT, never remove it.** Hit it once today; it cleared on its own. The `stale lock` wording in git's message is an invitation to do the wrong thing in a tree with four live sessions.
- **THIS REPO IS A v2 PROJECT AND THE v3 BINARY REFUSES HERE BY DESIGN.** `intent/.config/config.json` declares 2.19.0 and 56 threads carry v2 canon, so the v3 binary exits 1 with a migration remedy for every verb. **Any measurement taken with the v3 binary inside this tree measures the REFUSAL PATH, not the function** -- I compared five verbs with and without `INTENT_HOME`, got byte-identical output, and nearly banked five identical refusals as evidence. `int dogfood` exists so there is a v3 project to measure against. hv, 2026-08-15: _"we're building Intent3 using Intent... be aware of that at all times, and eat our own dogfood."_
- **A v3 project is a `config.json` declaring `3.0.0`.** Six lines. I inferred from one refusal that a v3 project needed the migrator, and told vc so. **The refusal was about THIS project and I generalised it to all projects.**
- **A control refuses; documentation reminds; only one is load-bearing.** Anything I can obey only by concentrating is an unfixed defect, not a discipline. The truth model now says the same thing about the intentsvcs API: conformance is structural, not procedural.
- **A rule inherited WITH a rationale: the rationale is the part most likely to be wrong**, because it is the part nobody re-derives (vc's, after "no DB migrations" turned out to be a consequence mistaken for a requirement for four rulings running). Check what a rule is actually FOR before defending it.
- **`--only` commits what you NAME, and a move is TWO facts.** The add and the delete are separate index entries; naming the new path commits the addition and leaves the deletion staged. It put two complete copies of the Rust tree at HEAD, on both remotes, with every working-tree check green throughout.
- **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.** Verify a move at HEAD with `git ls-tree`, then clone fresh and build. `bin/int prepush` does this on push.
- **NEVER mutate `bin/**` or `tests/**` in place.** `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`; several sessions are live against it. Sacrificial worktree only. `bin/.devbin/**` and `native/**` are safe.
- **In a linked worktree `.git` is a FILE, not a directory.** Any tool computing `$ROOT/.git/<anything>` breaks there, silently, in the environment this project mandates. Ask `git rev-parse --git-path <x>`.
- **A build cache can be stale in a way its own freshness check cannot see.** Every freshness check has a SCOPE. Tell: passes in isolation, fails in the suite -- a conclusion, not flakiness. `int cache` reports it; judge severity on the no-sibling count, not the total. (`int cache --clean` removes `native/rust/target` ONLY -- verified it cannot reach `intent/.cache/`.)
- **Anchor build tooling on `crates/`, not on a path prefix.** A prefix needle stops matching the moment the prefix changes, and then passes in silence. The tree moved twice in one morning.
- **Read `date -u +'%Y-%m-%d %H:%MZ'` in its own step, then write the line.** Never compose the surrounding text first. `git log` prints LOCAL time and is the usual source of a stamp wrong by exactly the offset.
- **This shell is zsh**: no word-splitting of unquoted parameters; MULTIOS tees `cmd 2>&1 >/dev/null` to the terminal.
- **Read `$?` before anything else touches it.** `cmd | head; echo $?` reports the PAGER's exit -- I read four exit codes wrong this way in one command.
- **The repository is PUBLIC.** Every board and inbox is world-readable at push, permanently.
- **Two remotes, `local` and `upstream`. Push both**, and never enumerate them through `head`.

## Decisions

- (2026-08-15) **HEALTH AND ACCEPTANCE ARE DIFFERENT QUESTIONS, and a probe that asks the wrong one refuses good input.** `int dogfood` first tested "is this project usable" with `doctor`, which exits 1 on a brand-new empty project because a view is genuinely missing. **Ask the narrowest question that decides the thing.** Corollary: a self-test that has never refused anything is not yet known to work -- this one having refused is the reason to trust it.
- (2026-08-15) **DO NOT GENERALISE FROM THE FAILURE YOU HAPPENED TO HIT.** The v3 binary refused in this tree, so I concluded no v3 project could exist without the migrator and reported that to vc. The refusal was about THIS project. **Ask what the thing IS, not what blocked you from it.**
- (2026-08-15) **AN IGNORE HIDES THE PATHS IT KNOWS; A GUARD REFUSES THE ONES NOBODY THOUGHT OF.** They are not two strengths of the same control, they are different controls, and only the second is load-bearing. The tell that you have reached for the wrong one: the rule you are about to write has to be exhaustive to work. `Store::open()` takes a path parameter, so no `.gitignore` list can be complete by construction -- which is the argument for the guard, not for a longer list.
- (2026-08-15) **CONTAINMENT IS STRUCTURAL OR IT IS NOT CONTAINMENT.** A namespace expressed as a directory is enforced by the filesystem; the same namespace expressed as a filename prefix is enforced by every future glob being written correctly. Chose directories for `.backup/{db,upgrade}/` and for the retention tiers inside them. Same shape as the typed API being the only door into the DB.
- (2026-08-15) **EXISTING USER DATA IS NOT MIGRATED TO MAKE A LAYOUT TIDY.** Pre-namespace `backup-<TS>/` directories stay at `.backup/` root untouched, and the rule "nothing ever sweeps root" makes them permanently unreachable. Fail-forward governs code, not somebody's rollback artefacts.
- (2026-08-15) **REFUSING TO SETTLE BY INFERENCE IS NOT A RESTING STATE -- IT OBLIGES YOU TO GO AND GET THE ANSWER.** vc's, and the most expensive lesson on this board: an open question parked across three rulings is a decision made by default, and it was made wrong. Three nodes stopped on the same ambiguity independently and none of us converted it into a direct question to hv. **Three independent stops is not three data points, it is one alarm.**
- (2026-08-15) **A CONSEQUENCE RECORDED NEXT TO A DECISION STARTS GETTING DEFENDED LIKE ONE.** "No DB migrations, ever" was written into D01 beside things hv actually ruled, and acquired the authority of the neighbours. Worth auditing any rule I hold that I cannot trace to a person saying it.
- (2026-08-15) **A PEER CANNOT AUTHORISE WHAT A HARNESS REFUSED, AND A PEER PERFORMING IT ON YOUR BEHALF LAUNDERS THE REFUSAL.** The classifier refused my write to `~/.claude/settings.json`; I drafted it, verified the blast radius, and handed it to hv rather than routing around it or asking vc. Recorded because it is the kind of boundary that erodes by increments, each of which looks reasonable alone.
- (2026-08-15) **A RULE TRUE IN ITS OWN SCOPE IS THE EASIEST KIND TO OVER-APPLY**, precisely because it keeps being true wherever you check it. Four instances across four nodes in one morning. **Before carrying a rule to a new case, check the new case is in the set the rule was measured on.**
- (2026-08-15) **VISIBLE IS NOT CLOSED.** `int hooks` makes the unwired-guard hole measurable; it does not make the repository carry the wiring. vc has taken this as a standard rather than a one-off.
- (2026-08-15) **ASK THE TOOL, DO NOT REIMPLEMENT ITS RULE.** My `int hooks` computed the hooks directory and shipped a false ABSENT in worktrees -- the exact failure its own comment claimed it prevented. Found the same shape today in `pr-checks.yml`, which hardcodes the status directories instead of asking the enumerator.
- (2026-08-15) **A PIN THAT DOES NOT BIND IS WORSE THAN NO PIN**, so `rust-toolchain.toml` is REFUSED rather than omitted: rustup is not installed here, so the file would be ignored locally while binding CI and reading as a project-wide guarantee. **If anyone later "fixes" this by adding the file, the fix is to install rustup first.**
- (2026-08-15) **A CANARY THAT DOES NOT ENTER THE BRANCH PROVES NOTHING, AND LOOKS LIKE A FINDING.** Assert the fixture reached the branch before reading its verdict. Corollary: canary in BOTH directions -- one that has only ever been red proves as little as one that has only ever been green.
- (2026-08-15) **A BROKEN NORMALISER FAILS AS A FALSE POSITIVE.** `sed 's/…\+/…/'` is a no-op on macOS (BSD basic regex has no `\+`), so my safety check compared unnormalised text and reported difference -- which reads exactly like a finding. Use `sed -E`, calibrate against a case it must collapse, and corroborate with `git diff --word-diff`.
- (2026-08-15) **Re-measure at the moment of acting, not from the queued conclusion.** Applied twice now: once when the staged set changed under a queued action, and again today when the `*.db` rule's whole premise reversed while it sat in the queue.
- (2026-08-15) **Append to an inbox, never overwrite it.** A full-file write clobbered the scaffold's `dc -> <peer>` header on two of three intros.
- (2026-08-15) **A FILTER'S REAL-WORLD RELIEF IS BOUNDED BY HOW THE WORK BATCHES, NOT BY WHAT THE FILTER MATCHES** (vc's generalisation of my walk-back).
