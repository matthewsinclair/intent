# Intent v3.0.2

**v3.0.2 gives Intent a search surface, and fixes what a full documentation audit found in v3.0.1.** Every document in the repository was read against v3.0.1 as built and corrected where the two disagreed; the defects that exercise turned up are fixed here. **If you are running v3.0.1, upgrade.** Most of these faults are silent by construction -- a verb that destroys a file and reports success, a critic that reports clean over rules it never read, an install whose every commit is refused -- so the output you have been reading is not evidence that you have not hit them.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** Every claim below was verified against the tree the release is being cut from: by reading the code that produces the behaviour, and by running that tree's own build read-only to see a verb's wording rather than quoting it from memory. The "before" half is the record in [Known defects](../../known-defects.md), each entry of which was driven against the published v3.0.1 keg. Anything ruled but not yet landed is deliberately absent rather than described in advance: a release note that documents a verb the release does not contain is the same defect as a manual page that does, and it is harder to withdraw.

## Added

**`intent search` answers one envelope, and it says how fresh it is.** A search returns its hits grouped by tier -- lexical text, and structural symbols -- ranked within a tier and never blended across tiers, with both denominators beside them: what matched, and what came back under your limit. **The index's own freshness travels in the same answer.** When the index is behind, or a path was skipped, the answer says so and names the paths, so a result is never a confident subset of a tree it could not read. `--json` and the MCP tool return that same value from the same call, so a script, an agent and a person cannot be told different things about one query.

**`intent search --kind def <name>` answers whether a thing with that name already exists**, from the tree rather than from a registry someone remembered to update. Symbols come from each language's own tree-sitter tags query, so a language is a grammar and nothing else -- there is no per-language logic here to fall out of step with the grammar. **References are name-matched occurrences and every surface says so**: nothing in this release resolves a name to what it points at, and a tool that implied otherwise would be making a claim it cannot support.

**`intent search --outline <path>` lists a file's symbols with their spans, and `--context <name>` gives a definition with the places that call it.** These are the answers grep cannot give. They replace reading a whole file with reading one span, which is what an agent asking _what is in here_ actually needs, and what a person asking _who calls this_ has been doing by hand.

**`intent search --sql <statement>` runs ONE read-only statement over this store.** It answers a class of question no verb does -- joins across the model and the index, in one pass -- on a read-only connection that cannot write whatever the statement says, bounded by a row cap and a work budget so a careless query cannot sit on the database. A second statement, a write, or a state-changing pragma is refused by name rather than silently ignored. Both denominators are reported, as they are for a text search.

**`intent index status` says what the index holds and every path it will not hold, with the reason -- and names the paths rather than counting them.** `intent index rebuild` walks the scope and rewrites what the index holds. **The two are deliberately different programs**: a status reads the rows and never walks the tree, so an operator who runs both can see when the index is behind the disk. A status that quietly re-walked would always agree with itself and could never report that.

**The index holds the repository, not just the model.** Prose on disk -- the README, the docs tree, the boards -- and source in every language the project declares are indexed alongside the store's own entities. **Every exclusion is a row carrying its reason**, rather than a silence: a file too large, a path outside the scope, a language with no grammar in this build.

**`/search` in the explorer is a resident pane.** It used to lend the terminal to the CLI, so results printed to the screen and were gone on the next repaint. The hits are rows now: Enter on an entity opens its view, Enter on a file opens the file, and the freshness line sits where the reader meets it before deciding whether to trust the list.

**The MCP tool descriptions say when NOT to use the tool.** A model choosing between the index and grep needs the half a help line never carries, so the register row that describes each tool now carries it and the tool a session sees says it.

**The canon routes a prior-art check through the index.** Every skill, template and rule that told a model how to check whether something already exists now names `intent search --kind def <name>` first, with grep as the fallback for exactly the case the answer names -- when the index says it is not complete for the paths that matter -- and a project's own module registry searched as well, where it keeps one.

**`intent claude upgrade --apply` declares Intent's MCP server to Claude Code.** A `.mcp.json` naming `intent mcp` is seeded when the project has none, so a session reaches the tools without anyone configuring it. **A project that already has one keeps it untouched, including under `--force`**, and `--skip-settings` now declines this file as well as `.claude/settings.json` -- one flag for the wiring Claude Code reads, because deleting a seeded file is not a way to decline it when the next run seeds it again.

## Changed

**The documentation says what v3.0.1 does.** The README, the install, migration and known-defects pages, the command reference, the concept pages, the guides under `intent/docs/`, the skills, the subagents, the rule library, and the comments in the release scripts, the workflows and the menubar app were each measured against the build and rewritten where they were wrong. **Hardcoded counts are gone from all of them**: where the tool reports a figure about itself, the page now names the command that reports it, because a number written into a page is true on the day it is typed and unfalsifiable afterwards.

**The templates `intent claude upgrade --apply` writes into a project say what v3 does.** The generated `AGENTS.md` and `CLAUDE.md`, and the seeded `usage-rules.md` and `.intent_critic.yml`, no longer point at documentation "at the Intent install", which a Homebrew install does not carry, and no longer describe `$INTENT_HOME` or v2's leading-zeros rule, or say that `intent critic` reads the file's severity threshold -- only the pre-commit gate does. A project picks the generated files up on its next `intent claude upgrade --apply`. **`usage-rules.md` and `.intent_critic.yml` are yours once seeded and are not rewritten.**

**The shipped hook scripts' comments** describe how v3 installs and runs them. Only comments changed; every hook behaves as it did.

**The command register's prose** -- which the command reference is generated from -- is corrected: the exit codes stated for `INV-04`, argument notes that claimed behaviour the build does not have, and the `intent claude` verb list, which still named the retired `prime`. None of the changed text reaches a command's help or output, so no command behaves or reads differently.

**The published schema faces carry 3.0.1.** At the v3.0.1 tag they still said 3.0.0. Their generator re-stamped them and only the version line changed.

## Fixed

**`intent init` refuses a directory that already holds files it writes, and names every one.** It tested for `intent/.config/config.json` and nothing else, then wrote its starter content with no further check -- so running it where a `CLAUDE.md`, an `AGENTS.md`, an `intent/wip.md`, an `intent/llm/RULES.md` or an `intent/llm/ARCHITECTURE.md` already existed destroyed that file and listed it as created. **The absence of a config makes a directory not a project; it never made it empty.** Every destination is now checked before anything is written, and a collision names all of them at once rather than stopping at the first.

**`intent st hydrate` refuses to write over a view whose bytes differ, and `--overwrite` names what it discards.** Every verify step went into the write set unconditionally, so a hand edit to a realised view -- or any work an unregistered writer had left there -- was replaced by the render and reported afterwards as written, at exit 0. An overwrite is a removal of the bytes that were there, and `dehydrate` has refused this exact signature since it was written, because the difference may be a hand edit and nothing on disk says which. The two verbs now answer alike. **`intent doctor`'s remedy for a skewed view names `--overwrite`**, because the bare verb now refuses.

**A realisation verb removes nothing.** `intent st hydrate`, `intent edit --path` and `intent st edit` build the estate's plan and narrow it to one artefact's directory, because classification needs the estate as its denominator -- and the narrowed plan could carry removals, which the run performed. **A verb whose whole job is to make files exist could delete one on its way past**, printing a single path and nothing else. All three now refuse, name every file the plan would have removed, and point at `intent organize`, which is the verb that reconciles an estate.

**`intent organize --apply` names every path it will remove before it removes one, and performs the plan it printed.** It computed and performed in one call and rendered afterwards, so the first time a path reached the screen it was already gone; `--apply --quiet` named nothing at all, because removals shared a predicate with the lines a quiet run is entitled to withhold. The plan is now printed first and the act is pinned to it, refusing if the tree moved in between -- **a promise about which files go, made about a different run, is worse than no promise.** The preview also predicts the directories a removal empties, which ran only under `--apply` before, so a preview could say nothing would be pruned while the apply removed directories the plan had not named. `--quiet` may withhold what a run wrote and may not withhold what it removed.

**`intent st dehydrate` names every path it will remove before it removes one.** Its `removed:` and `pruned:` lines printed from the report, which is after the act, so the first mention of a path was in the past tense. The future-tense lines now print from the plan, above the past-tense lines from the report, so the two can be compared by eye.

**The MCP `organize` tool removes nothing its caller was not shown.** One call with `apply: true` removed files, and the first and only account of which files was the response that came back afterwards. **A machine caller has no moment of looking, so the moment is made into a protocol**: call once without applying to see the plan, then echo the `plan` digest that answer carries. Applying without one is refused, and the refusal says what to do. The echo is checked rather than merely required -- a digest that no longer matches this tree refuses, because the estate moved between the two calls and the removals about to run are not the ones that were shown.

**`intent claude skills uninstall` and `intent claude subagents uninstall` no longer destroy an edit you made.** They removed every file they had recorded writing without asking whether those bytes were still their own, so a skill or subagent you installed and then edited was deleted at exit 0, under a line reading `removed (N file(s))` -- the same line, from the same code, that a run destroying nothing prints. A unit whose recorded files no longer match their recorded checksum is now held, nothing is removed, and the exit is the one every other undecided state takes. **`--force` removes it and names the checksum of what it discarded**, which is what its help always claimed it did. A file you keep beside a skill that Intent did not install is not a modification of it and does not hold the removal.

**Both uninstall verbs name the files they deleted.** They reported the removed files as a count while naming the files they left behind, so the only half of the line an operator could act on described the half that needed no action, and the paths that were gone -- the ones you would check a backup against -- were withheld. Both halves are now named.

**`intent claude subagents sync` takes the `--dry-run` that `intent claude skills sync` takes.** Both families share one preview in one function, but only the skills row declared the flag, so on subagents it was refused at the parser as an unexpected argument. Since a `sync` now holds a subagent you have edited, **the preview is how you decide whether to type `--force`**, and it was available for one payload kind and not the other. `--force`'s help on that family also said it overwrites an agent manifest; it overwrites the subagent's own file, and now says so.

**`intent claude upgrade --apply` holds a `.claude/settings.json` it did not write.** It overwrote the file unconditionally, in the same function that already held a hand-authored `CLAUDE.md`, so a project with its own Claude Code settings -- permissions, a model pin, hooks of its own -- lost them to a command run to refresh its documentation. A settings file that has never carried Intent's hook door is now held and named; `--force` overwrites it and says so in its help; `--skip-settings` still skips. A file Intent did write is still updated, so a hook fix reaches projects as it always did.

**`.intent_critic.yml`'s `disabled:` list is read when its key line carries a trailing comment.** The shape the documentation showed put the comment on the key line, and in that shape nothing was disabled and every rule stayed armed -- **with no line anywhere saying so, because an empty opt-out list is a legitimate state.** A project that opted a rule out by following the docs was being linted against it.

**`intent critic` refuses a `--format` it does not serve.** An unknown format rendered text at exit 0, so a consumer asking for a machine-readable report got prose and a success code and had no way to tell. It now refuses at exit 2, which is what a usage error takes.

**`intent critic` prints nothing before a refusal.** A run that was about to refuse printed an `ok:` summary line first, so the output said both that the run succeeded and that it did not. At exit 2 the refusal is now the whole output.

**`intent critic` does not count a file it could not read as a file it checked.** Where shellcheck declined a file, the run folded it into the total it reported as examined, so coverage nothing had read was counted as coverage. The file is now reported as not run, and the rules that were armed on it are named -- a decline is a property of one file rather than of the machine, so it is reported rather than made a refusal.

**A Homebrew install carries the subagents.** The v3.0.1 keg shipped without `intent/plugins/claude/subagents`, so `intent claude subagents list` answered `no subagents in this install` at exit 0 and there was nothing for `intent claude subagents install` to install -- the `critic-<lang>` family included. The support archive now carries that tree, and **the release refuses to build one that omits a directory the binary resolves by name at run time.**

**A fresh install says how to finish it.** The pre-commit gate a project installs finds Intent through `~/.intent/home`, which only `intent bootstrap` writes, so every commit was refused after a first install -- and the refusal said to reinstall, which writes no pointer. It now names `intent bootstrap`, and the formula says the same in a caveat. **Homebrew cannot do it for you**: its `post_install` runs with a throwaway HOME and cannot write yours.

**A release cannot be tagged with schema faces stamped for another version.** The v3.0.1 tag carried `schema/*` reading `INTENT_VER: 3.0.0`, because the published faces are generated from the crate's own version at compile time and the release stamped the version without regenerating them. The release now regenerates the faces through their own generator, in the release commit, and refuses to tag unless every published face carries the version being cut. It also refuses at pre-flight if a tree's faces disagree with its own `VERSION` before anything moves.

**A new project's `intent/wip.md` carries its own date and author.** The template stamped every project's `wip.md` with a fixed 2025 date and one person's name; `intent init` now fills in the date and the project's author.

## Removed

**Nothing that worked in an installed v3.0.1 is removed here.** If you have a working script against the `intent` command, this release does not break it. What follows is template trees and help files that no v3 binary reads, and which a Homebrew install never carried.

**The worker-bee extension seed** (`lib/templates/ext-seeds/`). Nothing in v3 read it, and `intent ext` is declared and not implemented.

**Template trees nothing in v3 reads**: `lib/templates/archetypes/`, `lib/templates/issues/`, `lib/templates/prime/` and `lib/templates/_treeindexignore`, and the per-language templates under `intent/plugins/agents/templates/`.

**`lib/help/`**, v2's help files. No v3 binary reads them.

## Upgrading

```
  $ brew upgrade matthewsinclair/intent/intent
  $ intent --version
  $ intent index rebuild
```

**The third line is the one that is new, and a search before it cannot find anything.** The index is built by `intent index rebuild`, and kept current after that by a running `intentd`, which reindexes what changes under the paths it watches. A search against an index nothing has built returns nothing at exit 0 -- the shape a genuine miss takes -- so the tool says which one it was, on stderr, in its own words: `nothing is indexed, so this search could not have matched`. **Read that line before concluding a name is absent from your tree.**

**Your runtime store is migrated on first open, and the upgrade is a ONE-WAY DOOR.** v3.0.1 wrote schema version 18 and v3.0.2 speaks 23. The first v3.0.2 command to touch a project migrates its store in place, through every step between, without asking -- and **nothing migrates it back.** A store written by a newer `intent` than the one you are running is refused outright, with the remedy stated as _upgrade intent rather than migrating the store down_. Forward is implemented; backward is not. **So take a snapshot with v3.0.1 first if you might want to go back**: `intent backup` writes the store, still at its old schema, to `intent/.backup/db/`, and copying that file back over `intent/.cache/intent.db` by hand is the only way back that always works -- no restore verb ships, and anything written after the snapshot is lost with it.

**Every rung between the two versions is the search index's own tables.** The store's entities -- threads, work packages, criteria, tests, issues, attachments -- are neither rewritten nor read by them, so the migration's risk is confined to a cache the tool can rebuild from your tree. **Your project's files are not touched by this.** What moves is the runtime store under `intent/.cache/`, which is rebuilt from the files except for its event log, which exists only in the store.

**If you run more than one v3 install, the first v3.0.2 command to touch a shared project ends the older install's access to it** -- an older machine, a colleague who has not upgraded, a pinned CI image. The refusal that install then gives names the store and both version numbers, which is enough to diagnose, and does not say that an upgrade elsewhere caused it.

**The fixes to the destructive verbs stop the loss; they do not recover one.** If a realisation verb, an `organize --apply`, or an `uninstall` has already removed bytes on v3.0.1, this release cannot reconstruct them. What it changes is that the next such run names what it is about to remove, first.
