# dc -- archived 2026-08-15 10:55Z (localfold, day one)

Node created and picked up 08:57Z. Everything below is DONE and landed on both remotes; the live board keeps only what is still open.

## Charter, as established

hv's words via `whiteboard/README.md`: _dev-x and build environment, so that cc concentrates on functionality for the CLI / daemon_. cc, ic and vc each wrote an intro within two minutes of each other (08:54Z / 08:55Z / 08:55Z) and their three independent readings of the lane agreed, which is worth more than any one of them.

Boundary adopted as **proposed-pending-hv**, in cc's words: `bin/intent`, `bin/intent_*` -> cc; `bin/int`, `bin/devbin`, `bin/.devbin/**` -> dc. The load-bearing argument is not ownership but the freeze -- `bin/intent*` is the measurement baseline the whole parity contract is scored against, so if it moves, ic's burn figures and register rows are measuring a moving target. "cc's" means "cc is the one who has to refuse".

## Landed -- ten commits, both remotes, CI green on all four legs

    f8948cc  guards wired + CI un-swallowed + *.bak class + toolchain pin refused
    7acab9c  bin/int prepush -- clone HEAD, build it, run the binaries
    bfd3e16  whiteboard commits off the test matrix
    49f0676  bin/int hooks -- three-state wiring report
    73e5d64  bin/int cache -- stale-cache detection
    9defbbd  int hooks false-ABSENT fix (worktrees)
    0a2e72b  int cache severity split (cc's discriminator)
    plus board folds

**The two guards (hv approved, a flat "Ok").** `bin/int precommit` runs `provenance_check.sh` and `view_skew_check.sh`, chained LAST in `.git/hooks/pre-commit` so the gate sees the set that actually lands rather than the pre-prettier one. Deliberately not in `lib/templates/hooks/pre-commit.sh`, which ships to every consumer while these guards check `intent/st/ST0056/parity/**`. Not merged, either -- two scripts, two invariants, two messages, and the runner names which refused; vc's separation ruling intact. Mutation-tested with four canaries in a sacrificial worktree, all discriminating.

**CI could not fail on integration tests.** A separate step ran one of the two integration files under `|| echo`, so it always exited 0; `run_tests.sh` already covers `integration/` and propagates status, so the step was redundant as well as toothless. Deleted along with a `bats || true` fallback. Measured before deleting: the suite passes, so the swallow was hiding nothing at the time. Result on `f8948cc`: `rust` success both legs, `Intent Tests` success all four -- and the second-order fact is the one that matters, that the suite is now green through a workflow that CAN fail on integration tests.

**`bin/int prepush`** -- clone HEAD, build it, run the binaries. vc left the pre-commit/pre-push/CI trade to me and measurement settled it: ~16s cold, too slow per-commit, and CI reports only after the bad state is already public. Path-triggered; a whiteboard-only push skips in 0.5s. Canaried against a fixture reproducing `a1a949c`'s exact signature -- root `Cargo.toml` plus `native/rust/Cargo.toml` both at HEAD -- REFUSED, both manifests named.

**The repository at HEAD verified sound**, which nobody had done since the half-move: fresh clone, cold build 14.02s, `intent` and `intentd` both produced and both running `3.0.0-dev`, exactly one workspace manifest, no stray root manifest, no duplicate tree.

**`*.bak` ignored as a class**; `/AGENTS.md.bak` pruned as redundant. `.claude/settings.local.json.bak` was the only unignored untracked file in a repository `gh` reports PUBLIC. vc's finding, verified here rather than taken on report.

**Whiteboard commits off the test matrix**, as an ignore-list rather than an allow-list: `paths-ignore` fails safe, `paths:` fails silent. Only `intent/whiteboard/**` went in, measured inert -- the two suites naming those paths both `create_test_project` first. `intent/st/**` deliberately NOT ignored despite the temptation, because I had not measured whether its suites use fixtures.

**`int hooks`** (vc's ruling, check-first). Three states -- WIRED / UNWIRED / ABSENT -- because the middle one, a hook that is present and executable and invoking something else, looks installed and protects nothing. Guard names read from the runner, never listed, so the roster cannot rot. Four canaries: fresh clone -> ABSENT (the hole reproduced); legacy hook -> UNWIRED; `--install` -> wires both and pre-existing content survives; three installs -> idempotent.

**`int cache`** -- cc's stale-cache class, measured still live: 228 dep files naming `native/target`, a directory that does not exist. Discriminated against a cold clone at the same revision (zero stale, 222 correct) so the difference is the cache, not the code. Warns rather than refusing, because the residue is latent and a gate that refuses over a usually-harmless condition gets switched off. cc then supplied the severity split -- 181 superseded vs 30 with no sibling, all proc-macro host artefacts, i.e. exactly the code-generating half -- and cleaned it: 32s, 246 tests passed, 3.1G -> 1.7G.

## Refusals, both deliberate

**No `rust-toolchain.toml`.** Measured: cargo and rustc are Homebrew's real binaries, rustup is NOT INSTALLED, and a file pinning 1.70.0 is ignored in silence. It would bind CI alone while reading as a project-wide guarantee. `rust.yml` records the toolchain per run instead, which produced a previously-unknown fact: CI and local are both `rustc 1.97.1 (8bab26f4f 2026-07-14)`, commit hash included.

**The auto-mode brief write was refused by the harness classifier** and not routed around, and vc was not asked to do it instead.

## Corrections made and taken

- **vc's** "two defects in that CI line" -- the `$?` half does not hold; `A || echo "$?"` propagates correctly (tested at 42 and 7). vc re-tested and withdrew it.
- **ic's** "unstaging is lossless where worktree == HEAD" -- backwards; that condition is exactly when the index holds the only copy. vc ruled the same.
- **Mine**: the path filter walk-back. A board-only commit still fired the suite because my push carried ic's `20e8c4b`. Mechanism correct; my commit message would have had a reader over-expect.
- **Mine**: I shipped a false-ABSENT defect in `int hooks` that lived forty minutes. In a linked worktree `.git` is a FILE, so `$ROOT/.git/hooks` does not resolve and the check reported ABSENT on a wired worktree -- the exact failure that file's own comment claimed it prevented. Fixed by asking `git rev-parse --git-path hooks` instead of reimplementing git's rule.
- **vc's** withdrawn `core.hooksPath` defect. Published at `high`, refuted by cc, withdrawn after re-running: `intent claude upgrade` resolves through `canon_hooks_dir()` and follows a redirect. vc's error was grepping for the string and concluding the mechanism was absent -- the correct API never needs to name it.

## afternoon -- hold lifted, hv gave every node the pen

Landed and pushed as `e5ecf9d`, `964adad`, `08bf4eb`. Kept here because the reasoning is in the commit messages and the board only needs to carry what is still open.

## DOING

- **A DEFECT I SHIPPED THIS MORNING, FOUND BY RUNNING THE TOOL RATHER THAN READING IT.** `int hooks` reported **two** guards while the gate enforced **three**. It derived the roster by grepping `$TOOLS/[a-z]*.sh` out of the runner, and I wrote in its own source that reading the names from the runner meant the roster "cannot rot". **It rotted the same day** -- the new DB guard is inline, not a `$TOOLS/` script, so the grep could not see it. **This is the failure the command exists to expose, committed by the command**: something that looks installed and protects less than it claims, one level up, answering confidently to whoever consults it to find out. Root error: **anchoring on a PATH SHAPE rather than the structural fact** -- my own watch-out, self-inflicted one file over. Fixed at `964adad` by the Decision I had already recorded: `int precommit` declares its roster ONCE, the run's step labels and a new `--list-guards` both read it, and `int hooks` asks.

  **The hazard I caught before shipping is the better half.** The obvious version ran `<runner> --list-guards` and read the exit code to detect support. **Measured first: `prepush` tests only `--force` and otherwise FALLS THROUGH AND RUNS**, so that probe would have cloned and cold-built the repo (~16s) every time anyone asked what the hooks were wired to. **A probe with a side effect is not a probe.** Capability now detected from the source; roster still from the runner. Four canaries -- and the first attempt at two proved nothing, because a fresh clone has no hooks so `guards_of` was never reached and empty output reads exactly like "no guards".

- **`.backup/` NAMESPACE NAMED -- cc unblocked, and it was the one thing of mine gating another node.** Delivered at 13:19Z.

  ```
  .backup/
    db/<tier>/<UTC>.db        D35 rolling snapshots.  cc's.   tier = daily|weekly|monthly
    upgrade/<UTC>/            `intent upgrade` rollback artefacts.  mine.
  ```

  **The namespace is a DIRECTORY, never a filename prefix**, and that is the whole decision: `.backup/db-<TS>/` beside `.backup/backup-<TS>/` was the smaller change and would have made containment depend on every future glob being written correctly. A directory makes the filesystem enforce it. Tier is a directory for the same reason -- a mis-globbed daily sweep is confined to dailies. **Nothing ever sweeps `.backup/` root**, so the two pre-namespace artefacts on this machine, and every one across the fleet, are permanently safe by construction: no migration, no move, no cleanup. I am not relocating existing user rollback data to make a layout tidy.

- **A NO-DATABASE-ENTERS-HISTORY GUARD in `int precommit`, and it is the _right_ control rather than the obvious one.** vc ruled the ignore file stays a PATH rule -- a blanket `*.db` there asserts a durability policy about a whole class for every consumer, and cannot work anyway because `Store::open()` takes a path PARAMETER. So the class protection went where it REFUSES: **an ignore silently hides the paths it already knows; a guard blocks the ones nobody thought of.** Two detectors: by name (catches `-wal`/`-shm`, which carry their own headers and are not SQLite-format at all) and by SQLite magic in the **staged blob** (catches a database committed under any name). Content-probes only what git already calls binary, so it stays off every text file in a large commit.

  **Six canaries in a sacrificial clone, both directions**: clean→0; `real.db`→refused by name; **SQLite under `renamed_as_data.bin`→refused by content, with the binary set printed first to prove the branch was entered**; a non-SQLite PNG→**passes**, so it is not merely refusing all binaries; `stray.db-wal`→refused; and **apparatus absent + staged db→still refuses**, which is why the guard moved above the ST0056 skip and the skip stopped being an `exit 0`.

- **`pr-checks.yml` now asks the tool.** `./bin/intent st show "$ID"` replaces the hardcoded `{COMPLETED,NOT-STARTED,CANCELLED}` list. vc's deciding reason is the right one and it is not cost: **a directory layout does not survive the port and a command name does** -- v3 holds status as a FIELD, not a directory. Verified in a clean clone with no config or cache, which is what that job has: flat/WIP, relocated/Completed, relocated/Not Started, absent, and malformed all return the right code.

- **`.gitignore` states D34 rather than folklore.** The ignore is correct on the **ceiling** -- git delta-compresses SQLite well; it is FTS5's ~1.95x expansion against GitHub's 100 MB hard block that decides it. Recording the real reason because we all had a correct conclusion resting on a wrong one.

## late afternoon -- WP-11, signing, and the schema defect

Landed and pushed: `556d1d0f` (int macos), `7cb29cec` (notarised + verify), plus the release profile and the dogfood fixture. Kept here because the reasoning is in the commit messages.

- **BOTH RELEASE BINARIES ARE DEVELOPER ID SIGNED.** hv removed the conditional -- _"Having Intent properly signed using my Geodica Apple Developer Connection keys is the right way to go regardless of whether or not brew needs it"_ -- and pointed at **Lamplight, which already signs its CLI and Wrighter with the same ADC**. Its `bin/.devbin/cmd/macos.d/` is the same devbin dispatcher Intent adopted, so `int macos <doctor|sign|notarize|env|store-creds>` is a **port, not an invention** (`556d1d0f`). One file rather than their `.d/` split, because Intent has no `.app`, no entitlements and no installer pkg -- their bundle walk and productsign half have no counterpart. **Signing needed no credential ceremony**: the identity was already in the keychain. Result is structurally identical to conflab -- same authority chain, `flags=0x10000(runtime)`, `TeamIdentifier=76BQL8L47U`, secure timestamp -- and both binaries still run.

  **REMAINING, and it is the hv item**: notarisation is written and UNTESTED; it needs one interactive `int macos store-creds`. Key material I should not handle. **AC-11.2 is decided and half implemented -- not marking it satisfied**, because its evidence is "decision-log entry + a notarised artefact" and the second half does not exist.

- **DOGFOOD FOUND A REAL DEFECT, ~40 minutes after the change landed, and it is the first live instance of the class D34 created.** A v3 project made this morning, opened by the current binary: `error: could not read the committed canon / no such column: state in SELECT ... FROM criteria`. The ratified AC enum added `state`; the existing DB still has `scope` + `satisfied`. **The shape is the bad one: `CREATE TABLE IF NOT EXISTS` makes the DDL apply a NO-OP on an existing DB, so `Store::open()` reports SUCCESS and hands back a store on the old schema** -- the open path succeeds on a database it cannot read, and nothing fails until a query names the new column. **No `user_version`, no `schema_version`, so detection is impossible today** and no migration could dispatch even once written. `store.rs:4` already states "MIGRATIONS ARE NORMAL"; the policy is written and the mechanism is not built. **Checked the debug binary first** -- fails identically -- because I had just changed the release profile and it was the obvious thing to blame. Sent to cc (lane) and vc (the invariant has no AC behind it).

- **hv RULING RELAYED 13:33Z -- `rm intent.db` should not exist as an operation anywhere.** hv: _"Why would anything in Intent EVER do this? If the db is the durable SSOT, this should simply NEVER BE A THING."_ Measured whole-repo before relaying: **production is CLEAN** (zero in `bin/`, zero in `crates/*/src/` -- `write_set.rs`'s removes are file-canon rollback), and cc has already fixed most doc comments. What survives is **three live test operations** (`store_rebuild.rs:150`, `cli_end_to_end.rs:575`, `search_surface.rs:56`) and **canon still pricing work in it** (`AT-14.11` to-write with `rm intent.db` AS ITS METHOD, `acceptance.md:156`, `WP/13/info.md:45`, `migration.md:27`, `restart.md:5`). Sent to vc (canon) and cc (tests); **I wrote none of it** -- relaying a ruling is not writing canon. **The argument that makes it more than stale wording: `rm intent.db` was never safe even under OLD D01** -- `event_log` has no canon path, so it destroys the audit trail AC-04.5 requires. The phrase was doing damage while it was still officially correct.

**AC-11.2 CLOSED END TO END.** hv stored the notary credentials; submission `cc52d5da-c974-4820-87a7-a583a95ffa68` came back **Accepted**. `int macos verify` proves it from a quarantined copy -- `source=Notarized Developer ID`, both binaries, runs. **CI needs no Apple secrets**: Lamplight's `ci.yml` references none, so signing is local on the machine holding the identity, and the escalation is retired rather than answered.

**Three corrections to my own work, all surfaced by hv running the tool**: `doctor` printed a BLANK signature field for correctly-signed binaries (`codesign` prints `Signature=adhoc` but `Signature size=N` -- a space, not an `=`, so the display went blank exactly when the news was good); `notarize` passed two sources to `ditto -c -k`, which takes one; and I had written into the source that spctl cannot assess a bare CLI, when `-t exec` was simply the wrong policy type.

---

## evening -- WP-11 distribution, closed out to the WP-12 boundary

Archived from the live board at 2026-08-15 15:28Z. Everything here is DONE or is now held in project canon (design.md D39/D40, acceptance.md AC-11.1 / AC-11.2 / AC-11.4, MODULES.md), which is why it left the board: **a fact recorded in canon does not need a second home on a node board, and a second home is how the two drift.**

### DONE -- the distribution leg

- **hv RULED BOTH OPEN QUESTIONS, direct, on my recommendations** (now D39/D40): v3.0.0 ships **macOS arm64 ONLY**, and the tap is **`matthewsinclair/homebrew-intent`** with artefacts on the source repo's own releases -- no `-dist` repo, which exists for Conflab only because Conflab is closed-source. Recorded with the reversibility attached: a Linux artefact needs no signature, so a Linux leg is purely additive whenever the platform reach is wanted back.
- **THE SEAM, answered by the estate four months before I asked it.** Conflab carries both a local and a CI signing path behind the repo variable `MACOS_RELEASE_CI`, which is **`off`** since 2026-04-16; the tap's whole 0.5.3 -> 0.6.0 run shipped from `bin/release --local`. The gate shape is the part worth stealing: with the variable off the macOS jobs **skip** rather than run unsigned, so "tag push publishes unsigned macOS artefacts" is structurally impossible rather than merely unlikely.
- **cargo-dist 0.32.0 DOES NOT NOTARISE** -- `notarytool` 0 hits, `notariz` 0, `stapler` 0, `altool` 0, `xcrun` 0. It signs only, and does that by importing a base64 p12 from `CODESIGN_CERTIFICATE` into a temp keychain, ie the CI-secrets posture. DEFERRED by ruling and **uninstalled**, after the ruling rather than before (vc's point: removing it earlier would make the same measurement cost money to repeat).
- **`int macos stage`** -- names artefacts per target triple from `rustc -vV`, proves each staged copy, then checksums; refuses if anything is unproven. **`verify_notarised <dir> <file>` is ONE implementation** serving both `verify` and `stage`.
- **`int macos formula`** -- generated tap formula, version read from the staged binary itself. **Refusal inherited structurally**: its only input is a file `stage` writes exclusively for proven artefacts.
- **The tap is live and deliberately formula-free.** A formula pointing at a nonexistent release would let `brew tap` succeed and `brew install` fail with a download error -- "the tap is broken" instead of "the release is not out yet".
- **AC-11.2 closed**; both submissions recorded (`cc52d5da`, `5eddb54a`). vc rewrote its evidence line to state the artefact is **transient and is not the evidence** -- the criterion is the decision recorded and the mechanism implemented, never a binary on disk in a directory any peer's `cargo build` can reach.
- **The release profile**: `lto=fat`, `codegen-units=1`, `strip=debuginfo`; 9,949,792 -> 8,084,128. Rejected `strip=symbols` (7,096,576) on No-Silent-Errors grounds -- 988 KB buys 10,064 named frames over 144.

### Decisions archived -- superseded by a control, or now held in canon

- **MEASURE THE TOOL BEFORE YOU DESIGN THE SEAM AROUND IT.** A question can be malformed and still feel rigorous, because rigour is about how you answer, not whether the question was worth answering. Corollary and the cheaper habit: **look for the estate's existing answer FIRST** -- Conflab had shipped this exact shape for four months and it was installed on my own machine.
- **A REVEALED PREFERENCE BEATS A STATED ONE, AND A DISABLED MECHANISM IS EVIDENCE.** A capability deliberately switched off is the finding.
- **AN AC NAMES THE OUTCOME; THE MECHANISM BELONGS IN THE WORK PACKAGE** (vc's, earned from AC-11.1 naming cargo-dist). A criterion naming a tool can be invalidated by measuring that tool while the thing the project wanted is still achievable -- a contract defect, not a discovery.
- **HEALTH AND ACCEPTANCE ARE DIFFERENT QUESTIONS.** Ask the narrowest question that decides the thing.
- **DO NOT GENERALISE FROM THE FAILURE YOU HAPPENED TO HIT.** Ask what the thing IS, not what blocked you from it.
- **AN IGNORE HIDES THE PATHS IT KNOWS; A GUARD REFUSES THE ONES NOBODY THOUGHT OF.** The tell you reached for the wrong one: the rule has to be exhaustive to work.
- **CONTAINMENT IS STRUCTURAL OR IT IS NOT CONTAINMENT.** A namespace as a directory is enforced by the filesystem; as a filename prefix it is enforced by every future glob being written correctly.
- **EXISTING USER DATA IS NOT MIGRATED TO MAKE A LAYOUT TIDY.** Fail-forward governs code, not somebody's rollback artefacts.
- **A CONSEQUENCE RECORDED NEXT TO A DECISION STARTS GETTING DEFENDED LIKE ONE** ("no DB migrations, ever" acquiring the authority of its neighbours).
- **A RULE TRUE IN ITS OWN SCOPE IS THE EASIEST KIND TO OVER-APPLY**, precisely because it keeps being true wherever you check it.
- **ASK THE TOOL, DO NOT REIMPLEMENT ITS RULE** -- `int hooks` recomputing the hooks dir; `pr-checks.yml` hardcoding status directories instead of asking the enumerator. Both fixed.
- **A PIN THAT DOES NOT BIND IS WORSE THAN NO PIN** -- `rust-toolchain.toml` REFUSED rather than omitted; rustup is not installed, so it would bind CI while reading as a project-wide guarantee. If anyone later "fixes" this by adding the file, the fix is to install rustup first.
- **A BROKEN NORMALISER FAILS AS A FALSE POSITIVE.** BSD basic regex has no `\+`, so `sed 's/…\+/…/'` is a silent no-op. Use `sed -E` and calibrate against a case it must collapse.
- **Re-measure at the moment of acting, not from the queued conclusion.**
- **Append to an inbox, never overwrite it.**
- **A FILTER'S REAL-WORLD RELIEF IS BOUNDED BY HOW THE WORK BATCHES, NOT BY WHAT THE FILTER MATCHES** (vc's generalisation).

### The correction this fold made, and why it is worth its own heading

The live board carried **"`codesign --verify --strict` is the check that means anything here"** as a watch-out, and carried my own measurement refuting it **two lines below**. Both were true when written and only one survived the day. vc had also given that same claim to hv as a recommendation and retracted it in writing rather than quietly editing it.

**A board can hold a refuted claim and its refutation simultaneously without either one looking wrong**, because each reads as a finding in isolation and nothing compares them. Archiving would have preserved the contradiction; the fold merged them instead. **Folding is not only compaction -- it is the only moment anything re-reads the whole board at once, which makes it the only moment a contradiction of this shape is visible at all.**
