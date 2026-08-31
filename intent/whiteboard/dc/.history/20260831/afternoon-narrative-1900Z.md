# dc, 2026-08-31 afternoon -- the reasoning behind the 1900Z fold

Instances, not rules. The rules that survived are on the board as classes.

## AC-02.3's `stated` half, derived at last (cd5e5258)

Agreed with vc days ago and never started because hv had not sequenced it. hv went afk with the pen delegated to vc, vc sequenced it, and it took twenty minutes and found two false rows.

**27 of 36 `stated` rows quoted nothing but the bare id.** So the verification was _does the string `intent#0137` appear in this file_ -- and the file has a section titled **Recorded against v3.0.0 and NOT present in it**. `0137` and `0171` were in it. Both claimed the page states them; the page says neither command exists in v3.0.0.

Each was re-dispositioned on its own evidence, deliberately not sharing one:

- `0171` -- keg, bare dir: `intent fc` and `intent at fc` both answer `unrecognized subcommand` at rc=1, refused before a store is touched. `intent fc` exists on main.
- `0137` -- is about `ac list` RENDERING a fiat-closed criterion, so `fc` being absent does not settle it. Drove the keg's own `ac` surface: ten verbs, no fiat door, and `fiat` scores 0 across root/ac/at/st/wp help. The state cannot be created there.

**The other eight ids in that section were already correct.** That is what makes it two rows that slipped rather than me misreading the section, and I would not have believed the finding without it.

The instrument now catches the class, and I proved it fires on the OLD manifest before trusting it: RED on exactly 0137+0171, GREEN on the corrected one. Its region control reports lines 131-144 holding 10 ids; an independent per-id section map agreed by a different computation.

## 0086 (8b8458c4) -- stating the TRUE half of a defect is not stating the defect

`migrating-from-v2.md:51` already named `help` among the eight retired commands. So the page was not silent -- it stated the retirement, which is true, and said nothing about the remedy, which is the defect. A reader who types `intent help`, reads `there is no v3 replacement -- remove it from any script that calls it`, then finds the page confirming the retirement, **comes away agreed with and still wrong**. `intent --help` answers at rc=0 on that same build.

That is what the disposition column is for, and I nearly marked it `stated` off the retirement sentence.

## The release (68db9a2d) -- a declaration is not a behaviour

vc relayed 13 new commands. The artefact says in bold, beside the number, that it is a DECLARATION and not a behaviour claim -- and `0163` records the whole `daemon` family declared AND refusing at the tag. So the number alone could have meant nothing.

Drove the non-mutating subset: `daemon status` answers on a socket, `graphql '{ __typename }'` returns real JSON. **The daemon family went from "known command that is not implemented yet" to answering.** That is the strongest single fact for the not-a-patch case, and it exists only because someone read the caveat the file prints about itself.

Nine of the thirteen are undriven and recorded as undriven.

## --title (f6d37b18) -- the part worth keeping is the stop

vc handed me `--title`. I read the dispatch entry before writing and found hv had re-sequenced `issues edit` to **cc** after 0183 (b1bf4cea), with cc's own signature on the row's anomaly note from earlier the same day. Measured it three ways rather than trusting one field.

**A delegated pen sequences work; it does not reassign a build hv already sequenced to someone else who is currently in the file.** I stopped, handed cc the measurement so they would start from a drive rather than my summary, and offered to take it on their word. They gave it explicitly and kept the MCP disposition.

The build itself: the gap was two fields wide, `--title` and `--severity`, both writable once at creation and never again. One facade method taking three `Option`s, because one operator act is one transaction and because `None` and `Some("")` are different acts. The slug is derived from the title and had to move with it -- the divergent-copy shape at field scale, inside one record, where no reader compares the two.

## Corrections I made to myself, in one afternoon

- **Told vc `--title` needed a clean `native/rust` window. It did not.** `releasebuild.lib:69` redirects a dirty build to a private target dir and leaves the shared artefact untouched. I had reasoned from a guard ARM'S NAME, and the name was about the refusal rather than about what follows it. Read the body next time.
- **My board said hv was owed "two items of one shape".** Two sections away the same board recorded that `issues edit` does not exist on the keg. `--note` is a gap in a shipping verb and the whole of why AC-02.1 is red; `--title` was a flag on a verb nobody had. One is a release blocker and the other is scope.
- **`--note` is on FOUR at verbs, not two.** vc found it, I drove it. vc warned AC-02.1 could go green on a partial landing; I checked the row rather than agreeing, and it cannot -- AC-02.1 names no verb and no flag, it is satisfied by DRIVING the page to a PASS gate. A criterion written as a journey is immune to the inventory defect a checklist would have had.
- Called the keg a "real file". It is a symlink to `../libexec/bin/intent` -- still inside the keg, so still unaffected by dev rebuilds, which is the property that mattered, but not what I said.

## vc's manifest row, and the window a guard cannot see

vc edited the disposition TSV on disk without `st attach`, leaving disk at 139 rows and the store at 138. I found it, did not fix it, and asked -- because `intent/.canon/st/ST0056.json` was staged in the index and `git commit --only` takes the WORKING TREE version of a path, so my write could have landed inside someone else's commit. vc attached it themselves (0fb29356) and noted the staged file was in fact my own board.

**vc's own framing is the keeper**: the canon guard fires at COMMIT, so it bounds the damage and does not close the window between the edit and the commit -- and that window is exactly where a shared tree puts four sessions. I nearly hit the other half: I had been running `git add` on that path all afternoon, so an uncommitted peer row was one command from landing in my commit under my authorship.
