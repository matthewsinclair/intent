# ST0095: Design. Git-style ids for threads, work packages and issues

**Status: under way since 2026-11-19 09:38Z by vc's date -u, on hv's go (hv decision 65). hv decision 66 adds ST0096 beside it and hv decision 67 the Highlander check.** hv decisions 61 to 64 rule the direction and the two forks the seats raised. Written by vc from its own measurements and from the read-only census cc, ic and dc each made of their remit the same day. A claim says whether it was measured or read. The three censuses are banked under `refs/bank/cc/st0095/`, `refs/bank/ic/st0095/` and `refs/bank/dc/st0095/`, and vc's reproduction under `refs/bank/vc/st0095/`; an anchor in a work package comes from them. The points still open are under "For hv at review".

## The rulings

hv, 2026-11-19, in hv's words unless marked:

- "We need to go all-in a fix to ST, WP, and ISSUE numbering (from ordinals to git-style hashes based on the guid of the creation event). This is a P0 fix that needs to be done asap." (decision 61)
- "Everywhere, going forward." Lists order by "date/time ... rather than ordinally". A stop-gap that restricts where a record may be created: "We can't have that restriction." (61)
- "In this case we DO NEED TO BE BACKWARDS COMPATIBLE with the old ordinal style numbering. And we need to ensure that we highlander both the minting of these new ids and any checks on them into as tight a waist as possible." (62)
- Other ordinals count only "if they are subject to the same issues as STs, WPs, and Issues". (62)
- **The stored id is long, and the short form is how it is shown and typed, as in git.** hv chose cc's position over the first draft. cc's words: "the stored id must be long enough never to collide, and the 7 characters are how it is shown and typed, as in git." (64)
- **Criterion and test ids stay caller-assigned**, and only their group follows the work package's id. (64)
- Whiteboard item numbers stay out. Issue 0705 gets its own thread in the same release, ST0096. (63, 66)

This reverses one line of ST0078's design (2026-09-18): "Rejected: changing the minting scheme."

## Why: what was measured

**The incident.** On an attendee's practice repository, one person had five merge requests open for review at once. Each new thread was minted from main and took a number an open merge request already held, four times running. One issue number collided the same way. Each was repaired by hand with `intent st renumber` or `intent issues renumber`.

**The reproduction** (vc, brew 3.4.0, a scratch repository under an isolated HOME, a second clone standing in for the forge's merge button):

| What was driven                                        | What happened                                                                                        |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- |
| `intent st new` on main while a branch holds ST0003    | ST0003 again, every time: the post-checkout hook had taken the branch's thread out of the store      |
| `intent issues add` on a branch while another has 0002 | 0002 again                                                                                           |
| All six branches merged in order, renumbered as above  | `intent doctor` 0 findings, in the working clone and in a fresh clone                                |
| `intent events --subject ST0003` on the merged result  | Five `st.new` events, one of them real; each renumbered thread's history starts at its `st.renumber` |
| The forge merging the second branch after the first    | Refused: conflicts in `intent/.intentfiles`, `intent/st/steel_threads.md` and `intent/todo.md`       |
| `intent sync --apply --yes` in that conflicted merge   | Refused (rc 1) until `.intentfiles` is resolved by hand; then it regenerates both views              |

**Why ST0078's answer does not hold.** ST0078 kept highest-plus-one and made renumber the repair, reading a collision as a rare race between two people. In a review-gated flow it is the steady state: one person with several merge requests open collides on every mint. hv's own estates never met it because every seat mints in the one main checkout, against one store, and commits straight to main. That checkout is a single allocator. A merge-request flow has none.

**The structural reason.** A sequential number needs one allocator, and git has none until the merge. Any number handed out on a branch is a guess about what the merge will find.

## The principle

**An id is minted where its record is born, from nothing but that birth, and it never changes.** No counter, no network, no restriction on where a mint may run. Two mints are never asked to agree, because neither reads anything the other writes.

## What an id is

### The definition (cc's, as the id's meaning)

Every mint already writes a creation event, and every event already has a globally unique id: a ULID from the crate's one minter (`event::next_id`, `native/rust/crates/intentsvcs/src/event.rs:381`). The creation ops are `st.new`, `wp.new` and `issues.add`.

**A new id is its kind's marker followed by the lowercase hex SHA-256 of its creation event's id, cut from the front.** The digest is `model::sha256_hex` over the event id exactly as `next_id` returns it. There is no second hasher and no second source of randomness.

The record carries the creation event's id in a new canon field, `origin`. The id is derived from it and never stored beside it as a second fact: `doctor` verifies that a hashed id is its marker plus the front of `sha256(origin)`. Each creation verb mints its event envelope first and derives the id from it, the order the fiat cascade already uses (`facade.rs:19790`).

### Stored, shown, typed

| Form       | A thread and an issue               | A work package, in its thread | Where it is used                                                                                                       |
| ---------- | ----------------------------------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **Stored** | marker + 16 hex                     | 8 hex                         | Canon file names, realised directories, `.intentfiles` rows, claims, `related`, criterion groups, event subjects, URIs |
| **Shown**  | marker + the first 7                | the first 4                   | Every table, view and message. Fixed, so a committed view is the same bytes on every branch                            |
| **Typed**  | any unique prefix of at least 4 hex | the same                      | Every door, with or without the marker, in any case                                                                    |

- **Sixty-four bits do not collide.** So nothing ever moves a hashed record, and no repair is written for one.
- **A shared short form is an ambiguity, never a misdirection.** The door refuses it and lists each candidate with its kind, title and created date. With a 7-character stored id, the repair would move one record, and every short reference already written on its branch would then name the other record with no error. That is today's defect made rarer, and it is why hv chose the long stored id.
- **The mint keeps short forms apart.** It refuses an id equal to one this clone can see, which at 64 bits never fires. It also draws a new creation event id when the new short form equals that of a visible record of its own kind: before anything is written, inside the one mint, and a bounded number of times before it refuses loudly. Without that, a project of 3,000 ids would have about a 1 in 60 chance of two rows showing one short form. With it, two rows share one only if two branches that have not met minted them: about 4 in 10 million per mint, with 100 records in flight. `doctor` then names both as an advisory, and nothing moves.
- **Nothing stored is a prefix.** Claims, `related`, criterion groups and event subjects compare exact strings today, and they stay correct only if what is stored is whole.

### The forms, by kind

| Kind            | Ordinal form: existing, and valid for ever | Hashed form, stored                         | Shown       |
| --------------- | ------------------------------------------ | ------------------------------------------- | ----------- |
| Thread          | `ST0092`                                   | `ST3f9a2c1d4e5f6a7b`                        | `ST3f9a2c1` |
| Issue           | `0703`                                     | `I3f9a2c1d4e5f6a7b`                         | `I3f9a2c1`  |
| Work package    | `ST0092/09`                                | `ST3f9a2c1d4e5f6a7b/c2e41a0b`               | `WP-c2e4`   |
| Criterion, test | `AC-09.3`, caller-assigned                 | `AC-c2e41a0b.3`, the group its package's id | `AC-c2e4.3` |

- **Every hashed id carries a kind marker** (cc, as meaning, and dc from the pipeline): bare hex can be an English word or a plausible file name, three places rely on an id not looking like a word (`intentfiles.rs:38-42`, `legacy.rs:3366`, `contract.rs:267-283`), and `organize` removes a file in `intent/issues/` that it takes for an undeclared view. ic first preferred a bare issue id and withdrew it on cc's evidence. The letters are ic's: a thread is `ST` plus hex, an issue is `I` plus hex, and a work package takes no letter, because it never stands alone.
- **No spelling is both an ordinal and a hash.** An ordinal is at most 4 decimal digits (2 for a work package). Anything longer is a hash or its prefix.
- **An existing ordinal thread gains hashed work packages** from the switch on, so one thread carries both families: `ST0094/01` beside `ST0094/c2e41a0b`.
- A branch or ref takes the lowercase id, as the estate's refs do today.

### Order

**The key is the id's family.** Ordinals come first, by number: that is the order they were minted in, and the order they have today. Hashed records follow, by `origin`, whose first 48 bits are the minting millisecond. Each list keeps its present buckets and direction.

So a project that holds only ordinals renders every view exactly as it does now, and no committed view in any estate re-renders on upgrade. `created` cannot be the key: it is a date with no time, and a work package has none.

### Typing an id

- **An ordinal-shaped spelling is always the ordinal.** `46`, `0046`, `ST46`, `ST0046`, `s46` and `i21` keep today's meaning.
- **Anything else of at least 4 hex characters is a hash prefix**, and one made only of digits needs 5. The pure half of this is ic's length rule; which record a prefix names is decided once, against the store, in `resolve.rs`, for the CLI, the TUI and MCP.
- An ambiguous prefix is refused with its candidates, and none is picked.
- Every answer prints the id's shown form, and `--json` and the canon carry the stored one.
- **The one bend in backwards compatibility**: today leading zeros are trimmed, so `00046` reaches ST0046. A five-digit token is a hash prefix now, so that spelling stops. See "For hv at review", point 1.

### Existing ordinals: backwards compatible, by hv's ruling

**Every existing id stays exactly as it is, for ever, and keeps working at every door.** No record is renumbered, no alias is added, no history or prose is rewritten, and the event log's subjects stay as written. Both families live in one project, one table and one grammar. After the upgrade no verb mints an ordinal again, in any project.

This is a standing exception to "fail forward, no shims", on hv's word: accepting the ordinal form is a permanent part of the id.

**Ordinals already in flight.** A branch cut before the upgrade can still carry an ordinal that collides when it merges. `intent sync --apply --yes` repairs it by moving this clone's record to a hashed id, derived from its own creation event where the log holds one. `st remint <id>` and `issues remint <id>` do the same by hand, with one argument, and replace `st renumber` and `issues renumber`. `next_free_id`'s arithmetic retires.

### One home: the waist

**One module, `intentsvcs::id`, owns an id end to end, and nothing outside it makes, reads, writes, scans or checks one.** It absorbs `model.rs`'s id section and is registered in `intent/llm/MODULES.md` before it is written.

1. **Types.** `ThreadId`, `WpId` and `IssueId`, each an ordinal or a hash, with no constructor outside the module. They replace `Thread.id: String`, `WorkPackage.seq: u32` and `Issue.number: u32`. A `{:04}` then does not compile against an id, so the compiler lists every site: about 70 hand formatters in 17 files today (ic's count).
2. **One mint**, called by `st new`, `wp new`, `issues add` and the transition's re-mint, and nowhere else.
3. **One stored-form reader**, for canon names and contents, `.intentfiles`, claims, `related`, addresses, event subjects and realised paths. It refuses by name anything that is neither family.
4. **One operator reader**: the pure spelling, and `resolve` above it.
5. **One formatter**: the stored form, the shown form, and each rendered form a surface needs (claim, manifest row, path stem, URI segment, `WP-` heading, lowercase ref).
6. **One order**: the family key above.
7. **One text scanner** for "which ids does this text name", replacing three (`legacy.rs:3366`, `contract.rs:267-283`, `contract.rs:351-359`).
8. **One check**: `verify(id, origin)`, for `doctor`.
9. **One sentence of forms** that every refusal, help string, MCP description and the guide quote, in place of today's hand-spelled copies.
10. **One read door on the binary** for scripts: is this token an id, of which kind, and which record does it name. No shell guard, hook, template or workflow carries a pattern of its own, and a guard in the commit gate holds that for everything the compiler cannot see.

## What changes

### The model, the canon and the store (cc)

- **Canon.** `Issue.number` and `WorkPackage.seq` become a string `id`; `origin` is added to a thread, a work package and an issue, written for hashed records only. The thread and issue schemas take a new version, and every canon struct stays `deny_unknown_fields`.
- **The upgrade rewrites the canon's format once.** `intent upgrade` turns `703` into `"0703"` and `1` into `"01"` in every issue file and every thread file with a work package. It is byte-deterministic, so a branch upgraded on its own merges cleanly with main upgraded on its own. It changes no id. This is new work in `intent upgrade`, which rewrites nothing today.
- **Why eager and not as files are next written**: an older binary refuses a canon file by its schema's name and reads nothing, which is the only thing that stops it. Brew 3.4.0 silently skips an issue file whose name is not a decimal number and then deletes it (`project.rs:2645-2661`, `store.rs:5009`). The bumped thread schema is what refuses it first.
- **The store** goes from schema 35 to 36 with two table rebuilds: `issues.number` is the rowid, and `wps.seq` is an integer that would turn an all-digit id into a number.
- **The published faces** (JSON Schema, DDL, the GraphQL SDL) and the export bundle take a new version. They are ic's side of the contract, so they need both seats.
- **The new binary refuses a canon name of neither family by name.** The silent skip is a No Silent Errors defect that exists today.

### The merge (cc)

Hashed ids do not end the conflicts measured above, because two branches that each mint still each add a row to the same three files. ST0096 takes those three files out of git (hv decision 66), and this package keeps the merge exact for what stays in git.

- **`plan.rs` learns the hashed family.** Today `classify` sends any canon path that is not an ordinal to take-a-side, which discards one side's record (`plan.rs:608-708`). This lands in the same unit as the mint, never after it.
- **`.intentfiles` and the two aggregate views are no longer merged here.** ST0096 moves realisation into each record (cc's R1) and stops tracking the two lists, so in an upgraded project none of the three is in git. A branch cut before the upgrade runs `intent upgrade` before it merges, so a set merge would have no case left. cc dropped both on 2026-11-19, AC-05.2 is withdrawn, and issue 0704 is superseded by ST0096.
- **Two branches that each add a work package to one thread** have different ids, so both are kept with no repair and no moved claim.

### Criteria and tests (cc)

Their ids stay as the caller assigns them. **The group between the dash and the dot is the work package's id**, and four places read it: the close gate (`contract.rs:19-33`, `59-65`), `wp show` and `wp gate` (`facade.rs:5582-5590`), `acceptance.md` (`views.rs:1301-1318`) and `doctor` (`doctor.rs:820-830`, `1244-1258`, `1427-1431`). A door takes a prefix for the group and stores it whole.

**The riskiest site in the census is here.** When no criterion matches a package's group, the gate passes and rolls up to the thread's contract (`contract.rs:888-896`). One missed site would let `wp done` close a hashed package with none of its criteria judged.

### Doors, tables and views (ic)

- The CLI's parsers, the length rule, and prefix resolution at every door, with the ambiguity refusal.
- Two path grammars go through the type: `intent:///` and the navigator's `/thread/<id>/wps/<seq>` (`nav.rs:204-307`).
- GraphQL's `issue(number: Int!)`, `Issue.number` and `WorkPackage.seq` become `id: String!` with no alias, and a creation instant is added. The Swift app is the only client and moves in the same cut. MCP passes every id as a string already, so only its descriptions change.
- Tables fit their id column to its content, never clipped, and the issues table gains a Created column.
- The three mint verbs print the id they minted in one machine-readable form.
- `st remint` and `issues remint` replace the two renumber verbs.

### The TUI, the app and the served page (ic)

- **The TUI**: its source formats ids from `number` and `seq` (`render.rs:7350`, `7630`, `7791-7808`); the navigator re-spells a work package through a number, so `0712` becomes `712` (`nav.rs:584`); the loaders parse numbers; and two forms declare number widgets (`forms.json:173`, `232`).
- **The Swift app** holds ids as opaque strings and needs one hashed fixture. **The served page** treats ids as opaque; its refusal examples name ordinals only.

### Tests and the pipeline (dc)

- **A test takes an id from the mint, never from a literal.** dc's grep found 48 Rust test files that predict a mint (108 functions, 396 sites) and 35 more to read one by one. 266 only name records a fixture seeded, and stay valid.
- **No flag or environment variable on the shipped binary pins an id.** A facade test binds what the mint returns. A record that needs a known id is seeded through the fixture writer. The explorer screenshots seed fixed ids, which keeps their byte-identical PNGs.
- **Seven places gate on their own copy of the id rule**, worst first: the brew smoke predicts ST0001 and runs after publish (`bin/.devbin/cmd/brew:80`, `370`, `392`); `rulings_check.sh:194` in the commit gate would refuse a commit that cites a new thread's file; `pr-checks.yml:35` fails a pull request that names a new thread; `session-context.sh:47`, shipped to every estate, misses a new id silently; `declared_kind_check.sh:175`; `canon-ignore-guard.sh:78-79`; and the explorer screenshots, a cut step.
- **One layer of proof**: one two-clone test, the renumber test's rig inverted, beside the waist's own tests. The cut's gate covers the rest.

### Docs, skills, rules and templates

hv asked for this package by name. ic's census: about 90 teaching lines and 221 example lines across 21 files; the team page's five collision sections, which are rewritten from a drive of the new build; the shipped skills (`in-essentials`, `in-whiteboard`, `in-plan`), two rules, one subagent, `_AGENTS.md`, and `_wip.md` with `session-context.sh`. The 21 reference pages, `cut-surface.md` and the screenshots regenerate in the cut's own steps.

## Upgrading a project, and the fleet

1. **Every machine that opens the project upgrades Intent first.** An older binary refuses the new canon by name, and stops there.
2. **`intent upgrade` once per project**, committed as one mechanical change. No id changes.
3. **An open branch upgrades before it merges.** The format migration is deterministic, so the merge is clean.
4. **`intent claude upgrade --apply` in each estate**, in the same visit as hv decision 41's sweep, so the hooks and `session-context.sh` re-render.
5. **The store migration is one way.** A brew 3.4.0 on the same machine refuses a store a newer build has migrated.

**A project not yet upgraded** (cc's meaning, 2026-11-19, put to hv as a change a user sees): the new build reads it and works on its ordinals as 3.4.0 does, and writes canon back in the project's own format, so nothing is rewritten behind the user's back and a machine still on 3.4.0 keeps working beside it. The first mint (`st new`, `wp new`, `issues add`, and the transition's re-mint) refuses and writes nothing, with `intent upgrade` as its one remedy. doctor names the upgrade as an advisory that is not counted. The format is one fact per project, the schema of `.canon/project.json`, which `intent upgrade` writes; 3.4.x already reads that file on every read of the disk and refuses an unknown schema by name, writing and deleting nothing, so it refuses an upgraded project whatever it holds. That closes risk 3.

**While this is being built, that reaches the main checkout by itself** (dc's finding). The repository's own commit gate runs the dev binary's `doctor` on the store of the checkout that is committing (`bin/.devbin/cmd/precommit:302-330`, `thread_view_skew_check.sh:84`), and every seat shares that build target. Once the shared target holds a binary with the new store schema, the next commit in the main checkout migrates the main store, and brew 3.4.0, first on every PATH, then refuses it: every `intent wb` verb and both hook carriers. So the first thing dc builds is a gate that judges a copy of the store whenever the build's store schema is newer than the store's. With that in place the packages land on main one at a time, as usual. A build is still never run by hand against the main checkout, and a worktree has its own store.

## Risks, the silent ones first

1. **A hashed work package closes with its criteria unjudged** (the gate's roll-up, above). It does not ship without a test of exactly that.
2. **A merge discards a hashed record** if the mint lands before `plan.rs` knows the family.
3. **Brew 3.4.0 deletes a hashed issue** in a project that has no thread to refuse on. Every project with one thread is protected by the schema bump.
4. **The brew smoke goes red after the release is public.** It is fixed before the rehearsal.
5. **Mixed versions across the fleet**: `.intentfiles` fails open in an older build, which then realises everything.
6. **Search**: how the stemmer treats a hex token is an inference, not a run. cc checks it before search is relied on for ids.

## What this thread does not change

- The store is truth on its machine and the canon is what travels (D01 as reversed, D34). One canon file per thread and per issue.
- Events stay the record of acts. Nothing rebuilds state from them.
- Whiteboard item numbers, eg "doing 62". A board has one writer, its own node, so they collide only if one seat writes its board in two places. hv ruled them out (decision 63).

## Beside this thread

- **Issue 0704** (sync refuses an `.intentfiles` conflict) is superseded by ST0096, which takes `.intentfiles` out of git (cc, 2026-11-19). It closes when ST0096 lands.
- **Issue 0705**: in a review-gated flow every merge still blocks every other open merge request, because the forge cannot run `intent sync`. It is ST0096, in the same release (hv decision 66): the two lists leave git, and realisation moves into each record.
- **Issue 0706** (doctor's advisory on `.claude/settings.json`) and **issue 0707** (closed threads' views on a project made with an older Intent) ride the same release.
- cc's census found seven defects that exist today. Three are absorbed by its packages here, and cc filed the other four as issues 0708 to 0711.

## For hv at review

1. **The `00046` bend**: a five-digit, over-padded spelling of an ordinal stops resolving. Recommended: accept it. None of the five forms v2 accepted is affected.
2. **Who writes the docs package**: ic, by hv decision 66, since ac was not started. ic drafts it while it waits for cc's resolver.
3. **cc's chain is the long pole.** Its part is XL in seven packages that land in order. ic's and dc's parts finish sooner.
4. **The release's number**, at the cut.
5. **The first mint in a project not yet upgraded** refuses with `intent upgrade` as its one remedy (cc's meaning, under "Upgrading a project"). The alternative is a mint that runs the upgrade itself, a mechanical rewrite riding inside an unrelated change. Recommended: the refusal.

## Work packages

Each is one seat's, with its size, what it lands after, and its criteria in `acceptance.md`.

| WP  | What                                                       | Seat | Size | Lands after |
| --- | ---------------------------------------------------------- | ---- | ---- | ----------- |
| 01  | Core tests take ids from the mint                          | cc   | S    | -           |
| 02  | The mint verbs say their id, and CLI tests read it         | ic   | M    | -           |
| 03  | The commit gate spares the main store; scripts capture ids | dc   | M    | -           |
| 04  | The waist: one module owns an id                           | cc   | L    | 01          |
| 05  | The merge knows both families                              | cc   | S    | 04          |
| 06  | The hashed family                                          | cc   | L    | 05          |
| 07  | Prefix resolution                                          | cc   | M    | 06          |
| 08  | The order                                                  | cc   | S    | 06          |
| 09  | Criteria follow their package, and doctor verifies         | cc   | S    | 06          |
| 10  | Every door takes and shows both families                   | ic   | L    | 02, 07      |
| 11  | The TUI, the app and the served page                       | ic   | M    | 10          |
| 12  | The pipeline asks the waist; the drill; the fleet          | dc   | M    | 03, 10      |
| 13  | Docs, skills, rules and templates teach the new ids        | ic   | L    | this review |

Packages 01 to 04 are behaviour-neutral: they land green on today's ordinals, and after them the scheme touches only the waist and the few tests whose subject is the scheme. Package 05's `classify` change and package 06's mint land as one unit.

## The proof

- Two clones, and two worktrees, that each mint a thread, a work package in one shared thread, and an issue from one base, merge with different ids and no repair.
- The five-open-merge-requests shape of the incident, driven on the new build for the team page, mints five different ids from main.
- **The drill hv asked for**: in a clone of this repository upgraded with the new build, a hashed thread, work package and issue are minted and committed through the whole commit gate, with `doctor` and the pull-request check passing, before the rehearsal. ST0095 stays an ordinal beside them. Hashes are not used by hand before that: 3.4.0 would skip and then delete them.
