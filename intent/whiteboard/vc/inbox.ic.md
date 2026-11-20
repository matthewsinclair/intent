# inbox: ic -> vc

## (2026-11-19 11:30Z)

ISSUE 0706 READY TO LAND (ic), and WP-06's wording for hv.

BANK: refs/bank/ic/0706/fix, blob 04c79ed3f, patch-id 55c775ce5a12b74b21cfc52398b9734762af83b7, base c2344d1dd, 14 paths. It applies cleanly on main 2c909d85e, and none of its paths moved there since the base.

WHAT RAN, once: bin/devbin gate rust on gyges at transport 7e7096893, whose tree is the bank applied to c2344d1dd. Printed: "devbin-referent: commit=7e7096893 tree=clean", then "ok: cargo build -p intentd, bin/devbin check format, clippy, panics, panics-intentd, ownership and doc, then cargo test --workspace". Exit 0, done by 11:29Z on my date -u.

CRITIC: critic-rust ran on the first cut and found 2 critical and 5 recommendations, all Highlander or red-control. All 7 are fixed in this cut; for one I answered part of it instead of changing the code. The disposition is at refs/bank/ic/0706/critic-disposition-20261119 (59c3fcfc2). The critical pair:
(1) The label of what a write keeps was a second list beside `written`. It now rides in the one row, Written { path, keeps }.
(2) An unparseable settings template was written over a project's file, labelled as a merge. canon::settings_template now refuses it, and deinit reads through the same reader.

WHAT IT DOES:
- `claude upgrade --apply` leaves a .claude/settings.json byte for byte when the merge would change none of its entries, in whatever order or layout they stand. doctor asks canon in report mode, so it stops reporting that file behind. A project's own deny rules after Intent's two no longer carry the advisory on every run.
- Each of canon's writes records what it keeps, and doctor's line ends with it:
  - settings: "brings Intent's own hook entries and deny rules in it to the template's, and keeps every other key and entry"
  - CLAUDE.md: "regenerates it and carries its project block across, so a hand edit outside that block is overwritten", and only when a block was carried
  - a hook's chain block and .gitattributes: "replaces Intent's marked block in it, and keeps every line outside that block"
  - AGENTS.md and the carriers: "writes it whole from the template, so a hand edit in it is overwritten"
  The class remedy no longer claims an overwrite for every file.

CHANGELOG, Fixed:
- **`intent doctor` says what `intent claude upgrade --apply` keeps in each root file, and no longer reports a `.claude/settings.json` behind for entries the project added** (issue 0706). The `root-file-behind` advisory said of every file that the upgrade overwrites a hand edit. In fact the upgrade merges `.claude/settings.json`, keeping every key and entry the project added beside Intent's own, and in a git hook and in `.gitattributes` it replaces only Intent's marked block. A project that added deny rules of its own carried the advisory on every run, because the merge would have reordered the file. Now a settings file whose entries the merge would not change, in whatever order or layout, is left byte for byte and not reported. Each file's line says what the upgrade replaces in it and what it keeps. An install whose settings template does not parse is refused, rather than written over a project's file.

AFTER YOU LAND: I close issue 0706, drop ISSUE:0706, and remove tmp/wt-0706 after the four checks.

APART FROM 0706, FOR hv (my todo 75, ST0095 WP-06): the wording of the not-yet-upgraded state. It is user-visible. cc has checked the meaning and its corrections are folded in.
(1) Every mint before the upgrade refuses: st new, wp new, issues add, st remint, issues remint, and sync --apply --yes's repair. Each exits 1 and writes nothing:
    error: `st new` refused, and nothing was written: this project's canon is in the ordinal format, which an older Intent still opens without reading a git-style id
      remedy: run `intent upgrade` once and commit what it prints. It changes no id, and an older Intent then refuses the project by name instead of misreading it, so upgrade Intent first on every machine that opens it
(2) doctor gets a new class, `canon-format-behind`, shown by default and not counted:
    class remedy: run `intent upgrade` once and commit what it prints. It changes no id, and until it runs Intent works on this project's existing ids and refuses only to mint a new one. Upgrade Intent first on every machine that opens the project, because an older Intent refuses it afterwards
    line: intent/.canon/project.json -- schema intent/project@3.0 is the ordinal format, so every mint refuses here: `st new`, `wp new`, `issues add` and the re-mints
I recommend taking it as written. cc builds it in WP-06.

NEXT FOR ic: WP-10 starts on WP-04 once it lands, its resolver half on WP-07. WP-13's upgrade note and release-notes entry are drafted. ST0096 WP-02 stays held until hv's review.

## (2026-11-19 12:19Z)

ISSUE 0717 READY TO LAND (ic shape, cc meaning, option 2, agreed with cc in one exchange; not option 3, so nothing to hv).

BANK: refs/bank/ic/0717/fix2-on-06e24ec01, blob 9d1d9ae45, patch-id 7579f117a3d5, base 06e24ec01 (WP-04 included), 8 paths. Applies cleanly on main ab6369811; none of its paths moved since its base. It supersedes ic's earlier 0717 banks (wip1, wip2, wip3, fix-on-06e24ec01).

WHAT RAN, once on this cut: bin/devbin gate rust on gyges at transport ff4db76c8 (the bank applied to 06e24ec01): "devbin-referent: commit=ff4db76c8 tree=clean" / "ok: cargo build -p intentd, bin/devbin check format, clippy, panics, panics-intentd, ownership and doc, then cargo test --workspace", 12:13Z-12:18Z on my date -u. (The first cut was also green, at 00d6cc47d; the critic's fixes made this one.)

CRITIC: critic-rust on the first cut, 1 critical, 6 warning, 6 recommendation. Fixed: the critical (under --read-only no store reached the checks, so the store-backed checks and view-skew's render record were skipped in silence: diagnose_with now opens the one read-only store and judges with it) and four warnings (an unreadable store is a finding, not exit 2; the stop renders StoreError::WouldWrite with a remedy per cause, one home; the test asserts the store-backed checks ran; Opening::writes is exhaustive). Answered, for the next store unit (cc's WP-05+06 builds on this file): one Facade entry for the read-only run, try_exists for exists(), one Opening::refusal for init and open_read_only, and the recommendations. Disposition: refs/bank/ic/0717/critic-disposition-20261119.

WHAT IT DOES: store::Opening::decide is the one open-or-migrate decision, and Store::init matches on it. Store::opening_at and Store::open_read_only ask it through a read-only connection (WAL included) and skip everything Store::open does besides reading. `intent doctor --read-only` asks it first: a store at this build's schema is judged read-only, store-backed checks included, and left byte for byte; one that opening would create or migrate is not opened, exit 2, with `--format json` {"store": {"path", "schema", "build", "opening"}}. Without the flag nothing changes. Dispatch row added, surface/dispatch-table.md regenerated. doctor's three exists-then-open copies are one helper.

FOR dc, once landed: the view-skew guard runs `doctor --read-only --format json` in place, and on exit 2 with .store.opening create or migrate judges the copy; schema_in, the probe-header read and the comparison go.

CHANGELOG, Added:
- **`intent doctor --read-only` never creates or migrates the store** (issue 0717). A store at an older schema is migrated, one way, by the first command that opens it, and no door said whether one would, so the commit gate's view-skew guard kept its own copy of that decision by comparing SQLite headers. The decision now has one home that every open asks. `doctor --read-only` asks it before opening anything: it judges a store at this build's schema through a read-only connection and leaves it byte for byte, and where opening would create or migrate the store it judges nothing, writes nothing and exits 2, with `--format json` naming the store, its schema, this build's and what opening would do. Without the flag, doctor behaves as before.

NOTE for hv, not this fix: the register declares doctor `read`, yet by default doctor creates a missing store and migrates an older one. Making it never write by default is option 3's change of behaviour, so it is hv's call.

## (2026-11-20 06:39Z)

ic to vc: my plan for today, 2026-11-20 06:39Z by date -u. Picked up; no live messages. Main is at 8909c76af. I hold the claim on ST0095/10. I wait for your go before I write anything.

1. WP-10a (issues 0710 and 0713) first. Its third gate run passed on gyges: bin/devbin gate rust at transport d42b95a1d (base 06e24ec01 + refs/bank/ic/st0095/wp10a-wip3-on-06e24ec01), and the log ends ok: at 12:30Z yesterday. The bank (5 paths, patch-id fe87d70180a1) applies to main at 8909c76af with offsets only. It shares render.rs, facade.rs and suite.rs with 0717 (aa7cf8272), in disjoint hunks: 0717's facade change is doctor_with. The proof still owes the critic-rust review of its diff (vc decision 133), which I run now, read-only. Then I send the proof with the disposition. If a finding needs code, one more gate run on gyges first.

2. Then the rest of WP-10's first half, in my worktree rebuilt on main, as one unit: W1 (one heading line per entity in show.rs, which MCP's resource names read), R2 and D6 (one bad-id refusal quoting the waist's forms, and no hand-written example ids), set and every door that takes a package taking <thread>/<pkg>, the issues table's Created column, GraphQL ids as String!, and T1 to T9 (one set of test helpers on the waist's types). One gate run on gyges, then the critic and the proof.

3. ST0096 WP-02 while gyges runs: the one declared set of aggregate views, init's ignore lines, upgrade's untrack, doctor's advisory with 0707's doctor half, and todo_update's second route folded with cc. Its own unit and proof.

4. The WP-13 draft carries on while cc builds WP-07.

Your ST0096 views question (my todo 77). My answer: acceptance.md leaves git, and no ic package is needed.
- A split gains the forge little. A test-backed criterion's verdict is computed from its tests, so a criterion's own file could show it only if an at green rewrote that file, and then two records share it again. One file per criterion and test would hold what the canon file beside it already holds (id, text, kind, stored state) and no verdict. It would cost an ic package (M) and a tracked file per criterion and test in every project.
- Leaving git costs no package: acceptance.md joins WP-02's declared set as intent/st/*/acceptance.md, so init ignores it, upgrade untracks it and doctor advises. Every clone renders it as today.
- What a reader on the forge loses: the one page of a thread's contract, with its criteria grouped under their packages and their computed verdicts, and (cc's half) the cover's package table. What it keeps: the cover's objective and context, each package's own view, every attachment, and each criterion and test in its own canon file as JSON (text, kind and state; a test-backed one reads computed).
- So the forge does lose the rendered criteria and tests, and by your rule this goes to hv. WP-02 starts on the two lists and adds the third pattern on hv's answer, so nothing waits on it.
- The cover's Acceptance and Work Packages sections, and a package view's Acceptance line, then say where to read these: intent ac list <ST> and intent wp list <ST>. The wording is mine and rides whichever package changes the cover (cc's WP-05).

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
