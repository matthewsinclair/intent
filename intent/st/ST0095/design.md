# ST0095: Design. Git-style ids for threads, work packages and issues

**Status: DRAFT for hv's review, 2026-11-19. Nothing here is built.** hv decision 61 rules the direction. Each point under "For hv" is a recommendation and is not ruled until hv rules it. Written by vc. Where a section says "census", it rests on the read-only census cc, ic and dc made of their own remits on 2026-11-19, and each claim says whether it was measured or read.

## The ruling

hv, 2026-11-19 (hv decision 61), in hv's words:

- "We need to go all-in a fix to ST, WP, and ISSUE numbering (from ordinals to git-style hashes based on the guid of the creation event)."
- "Everywhere, going forward."
- "Ordering needs to be date/time based as the default, rather than ordinally by ST, WP or Issue number."
- On a stop-gap that would let a record be created only on main: "We can't have that restriction."
- "I want this documented as a new steel thread with elaborated WPs ... before we write a line of code."
- On scope, to dc: "If there are other ordinal identifiers, then I only care if they are subject to the same issues as STs, WPs, and Issues."
- "In this case we DO NEED TO BE BACKWARDS COMPATIBLE with the old ordinal style numbering. And we need to ensure that we highlander both the minting of these new ids and any checks on them into as tight a waist as possible."

This reverses one line of ST0078's design (2026-09-18): "Rejected: changing the minting scheme."

## Why: what was measured

**The incident.** On an attendee's practice repository, one person had five merge requests open for review at once. Each new thread was minted from main and took a number an open merge request already held, four times running. One issue number collided the same way between two branches. Each was repaired by hand with `intent st renumber` or `intent issues renumber`. Merge request 8 there proposes a guidance line telling the agent to check and renumber.

**The reproduction** (vc, 2026-11-19, brew 3.4.0, a scratch repository under an isolated HOME, a second clone standing in for the forge's merge button):

| What was driven                                        | What happened                                                                                        |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- |
| `intent st new` on main while a branch holds ST0003    | ST0003 again, every time: the post-checkout hook had taken the branch's thread out of the store      |
| `intent issues add` on a branch while another has 0002 | 0002 again                                                                                           |
| All six branches merged in order, renumbered as above  | `intent doctor` 0 findings, in the working clone and in a fresh clone                                |
| `intent events --subject ST0003` on the merged result  | Five `st.new` events, one of them real; each renumbered thread's history starts at its `st.renumber` |
| The forge merging the second branch after the first    | Refused: conflicts in `intent/.intentfiles`, `intent/st/steel_threads.md` and `intent/todo.md`       |
| `intent sync --apply --yes` in that conflicted merge   | Refused (rc 1) until `.intentfiles` is resolved by hand; then it regenerates both views              |

**Why ST0078's answer does not hold.** ST0078 kept highest-plus-one and made renumber the repair, on the reading that a collision is a rare race between two people. The reproduction shows the steady state of any review-gated flow: one person with several merge requests open collides on every mint. hv's own estates never met it because every seat mints in the one main checkout, against one store, and commits straight to main. That checkout is a single allocator. A merge-request flow has none.

**The structural reason.** A sequential number needs one allocator, and git has none until the merge. Any number handed out on a branch is a guess about what the merge will find. Counting over every ref the clone can see narrows the guess and does nothing for another person's unpushed work. Reserving a number on the shared remote needs the network and a push at every mint. Restricting mints to main is the restriction hv ruled out, and it would not have prevented four of the five collisions, which were minted on main.

## The principle

**An id is minted where its record is born, from nothing but that birth, and it never changes.** No counter, no network, no restriction on where a mint may run. Two mints can never be asked to agree, because neither reads anything the other writes.

## What an id is

### Derivation

Every mint already writes a creation event, and every event already has a globally unique id: a ULID from the crate's one minter (`event::next_id`, `native/rust/crates/intentsvcs/src/event.rs:381`). The creation ops are `st.new`, `wp.new` and `issues.add`.

**A new record's id is an abbreviation of the SHA-256 of its creation event's id**: the digest of the event id's 26 characters, written as lowercase hex and cut from the front.

- It is hv's wording made exact: a git-style hash, based on the guid of the creation event.
- It adds no allocator and no second source of randomness. `sha2` and `ulid` are already dependencies.
- It is checkable: `doctor` can recompute any record's id from the event it names.
- A hash is used, and not the event id's own trailing characters, because the minter is monotonic within a millisecond: two events minted in one millisecond differ only in their last character, so their tails would be one keystroke apart. A hash makes a one-character typo land on nothing.

The record keeps the creation event's full id in a new field, `born`. That one value is the proof of the id and the key every list orders by.

### The visible forms

| Family       | Existing form (stays valid) | New form               | Unique within | Example           |
| ------------ | --------------------------- | ---------------------- | ------------- | ----------------- |
| Thread       | `ST` + 4 digits, `ST0092`   | `ST-` + 7 hex          | the project   | `ST-4f9a2c1`      |
| Issue        | 4 digits, `0703`            | 7 hex                  | the project   | `4f9a2c1`         |
| Work package | 2 digits under its thread   | 4 hex under its thread | its thread    | `ST-4f9a2c1/9d3f` |

- **Seven characters** is git's own default abbreviation: 28 bits. A mint never issues an id this clone can already see, so the only collision left is with a record on a branch this clone has not seen. With 100 such records in flight, that is about 4 in 10 million per mint.
- **The hyphen** in a thread's new form keeps the two shapes apart at a glance, including the 1 in 27 new ids whose seven characters happen to be all digits.
- **A work package keeps its place under its thread.** Its id only has to be unique there, so it can be short, and every address, claim and path keeps today's structure: `intent:///threads/ST-4f9a2c1/wp/9d3f`.
- The exact spelling on each surface (prefix, separator, case, widths) is ic's to settle. cc settles what the id means.

### Order

**Every list orders by creation time.** The key is `born`, which sorts by time and breaks ties within a millisecond. A record with no `born`, which is every record that exists today, orders by its `created` date and then by its number, which is the order it has now. Each list keeps the direction it has today.

Clocks on two machines can disagree by seconds. That only reorders records minted within those seconds of each other, and any stable order is as good as another there.

### Typing an id

Any door takes a full id of either form, or a unique prefix of a new one, case-insensitive: `intent st show 4f9a`. An ambiguous prefix is refused with the candidates and their titles. A full ordinal such as `ST0092` always means that ordinal. Every answer prints full ids, so an agent copies the whole thing.

### When two ids do collide

- **At mint**: an id already present in this clone's store or tree is not issued. The verb mints a fresh creation event and derives again.
- **Across branches**: two different records with the same id is still git's add/add on the canon file, as loud as today, and `intent sync --apply --yes` still moves this clone's record. It moves it to a fresh id. The engine is ST0078's renumber, kept because ordinals minted before the upgrade are still in flight on branches.
- **Two branches each add a work package to one thread**: the ids differ, so the merge's three versions are read as today (issue 0555) and both packages are kept, with no renumber and no moved claim.

### Existing ordinals: backwards compatible, by hv's ruling

**Every existing id stays exactly as it is, for ever, and keeps working at every door.** No record is renumbered, no alias is added, no history or prose is rewritten. Both forms live in one project, in one table, in one address grammar. A project that never mints again is untouched by this change. After the upgrade no verb mints an ordinal again, in any project.

This is a standing exception to "fail forward, no shims", on hv's word: reading and accepting the ordinal form is a permanent part of the id, not a shim to be retired.

### One home: the waist

**One module owns what an id is, and nothing else may know.** hv's ruling: the minting of new ids and every check on an id go through as tight a waist as possible.

- **One mint.** `st new`, `wp new`, `issues add`, and the repair that moves a record to a fresh id all call the same function: creation event id in, id out, never one this clone can see.
- **One reader.** Parsing, validating, telling an ordinal from a new id, formatting, the order key and the prefix-matching rule each have one definition, there.
- **A type, not a string.** An id crosses the model as a value only the waist can construct. The numeric `number` and `seq` fields go, and with them every `format!("{:04}")` that restates the shape. cc's census lists each bypass that exists today.
- **No pattern outside it.** No other Rust file, and no shell guard, hook, template or test, carries a pattern for an id. A script that needs to know asks the binary. A guard in the commit gate holds this, so a second definition cannot arrive quietly.
- The waist is registered in `intent/llm/MODULES.md` before it is written.

## What carries the id

### Canon and its schema

Read in the canon on 2026-11-19: a thread's file is `intent/.canon/st/<id>.json` with `id` as a string, at schema `intent/thread@3.0`; its work packages are rows keyed by a numeric `seq`. An issue's file is `intent/.canon/issues/<NNNN>.json` with a numeric `number`, at `intent/issue@3.0`.

- A work package's `seq` and an issue's `number` become a string `id`. An existing `seq: 9` reads as `"09"` and an existing `number: 703` as `"0703"`.
- New records gain `born`.
- Both schemas take a new version. A binary that does not know it refuses by name, which is today's behaviour and the team page's "Upgrade together".
- **Recommended: the reader takes both versions, and a file is written in the new version the next time a verb writes that record.** An eager rewrite of every canon file on upgrade would conflict with every open branch that touched a thread.

### The store, events, and what else is keyed by an id

cc's census sets these out: the store's tables and its migration, ingest keyed by id, event subjects, the search index's kinds, the claim grammar, `.intentfiles`' grammar, and doctor's checks. A new check: a record's id is the abbreviation of the event its `born` names.

### Doors and views

ic's census sets these out: the CLI's parsers, the MCP tools' parameters, GraphQL's types (an issue's number is an integer there today), the `intent:///` forms, the claim forms, the realised paths, every table's order and widths, and the docs, skills, rules and templates that teach an id form.

### Tests and the pipeline

dc's census sets these out: the tests and fixtures that predict the next number, the guards and hooks with an id pattern, and what proves the change. A test takes an id from the mint's own output and never from a literal.

## What this thread does not change

- The store is truth on its machine and the canon is what travels (D01 as reversed, D34).
- One canon file per thread and per issue.
- Events stay the record of acts. Nothing rebuilds state from them.
- `st renumber` and `issues renumber` stay, for ordinals in flight and for the rare collision.
- Whiteboard item numbers, eg "doing 62" and "hv decision 61". See "For hv", point 3.

## Beside this thread: four defects the same reproduction found

Each is filed as an issue so it rides the same release. None is fixed by new ids.

1. **Issue 0704: `intent sync --apply --yes` refuses a merge whose only conflict outside the views is rows both sides added to `.intentfiles`.** Any two branches that each start a thread or add an issue meet it. The rows are a set, and the merge's three versions say which were added and removed.
2. **Issue 0705: in a review-gated flow, every merge blocks every other open merge request**, because every branch that starts a thread touches `.intentfiles`, `steel_threads.md` and `todo.md`, and the forge cannot run `intent sync`. With new ids this is the one hand step left in the flow the incident came from. Its fix is a design choice of its own: whether the two aggregate views stay tracked, and whether realisation stays a shared manifest.
3. **Issue 0706: doctor's `root-file-behind` advisory says a hand edit in `.claude/settings.json` is overwritten.** Measured on 3.4.0, `intent claude upgrade --apply` keeps a project's own deny rules. The attendee read the advisory and believed it.
4. **Issue 0707: a project made with an older Intent keeps closed threads' views that every `intent sync --apply --yes` removes again on each branch**, so every merge request carries the same deletion until main does it once.

## For hv

1. **The derivation and the forms above**: SHA-256 of the creation event's id, lowercase hex, 7 characters for a thread and an issue, 4 for a work package under its thread, `ST-` on a thread. Recommended as written, subject to cc on meaning and ic on spelling.
2. **Criterion and test ids**, by hv's test: are they subject to the same issue? Yes, inside one thread. They are caller-assigned today (`intent ac new <thread> AC-01.1`), and the convention builds them on the package's number, which is going away. Two branches that each add a criterion to one work package pick the same label, and git reports it only as a content conflict in the thread's file, so a hand merge can leave two criteria under one label. Recommended: Intent mints them the same way, as their own work package in this thread.
3. **Whiteboard item numbers**, eg "doing 62", by the same test: only when one node writes its board in two clones or worktrees at once. A board has one writer, its own node, which is a real single allocator while that node runs in one place, and the tool already refuses a board both sides moved. Recommended: out of this thread, named here as the one ordinal family left.
4. **Issue 0705.** Recommended: its own thread in the same release, since it needs a ruling on whether aggregate views stay in git.
5. **The release's number.** A change to the id scheme and the canon schema is not a patch. Yours at the cut.

## Work packages

Set out in the thread, one per row, each with its seat, its size and its criteria. They are written after the census so that each names the files it touches.

## Acceptance sketch

One row per behaviour a person or an agent meets:

- Two clones, two worktrees and two branches that each mint a thread, a work package in one shared thread, and an issue, merge with no id conflict and no renumber.
- The five-open-merge-requests shape of the incident, driven on the new build, mints five different ids from main.
- Every existing ordinal still answers at every door, beside new ids in the same project.
- Every list orders by creation time, ordinals and new ids together.
- A unique prefix resolves at every door, and an ambiguous one is refused with its candidates.
- `doctor` reports a record whose id is not the abbreviation of the event it names.
- An older binary refuses the new canon by the schema's name.
- No verb mints an ordinal after the upgrade.
- No test and no guard compares against a predicted id.
- Every command on every changed docs page has been driven.
