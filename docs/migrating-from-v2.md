# Migrating from v2

**Intent v2 was a Bash implementation; v3 is a Rust rewrite with a different storage model.** The hop is a one-way migration, it is not reversible in place, and it is real work rather than a command.

**Take v2 off your `PATH` once a project is migrated.** The last released v2, v2.19.0, does not recognise a v3 project: it reads the tree as if it were v2 and writes to it the same way, so `intent st new` under v2.19.0 creates a v2 thread directory inside the v3 project, at exit 0.

## Doing the hop

```
  $ intent ingest
  $ intent upgrade
  $ intent doctor
  $ intent st list
```

**Commit first.** `intent upgrade` refuses to convert a project that is not under git or has uncommitted changes, because the migration is one commit and `git revert` of that commit is the rollback.

`intent ingest` checks that the project's markdown would migrate and names each line it could not carry; **it writes nothing.** `intent upgrade` does the migration. It reports each section it did not carry as-is (a section still byte-identical to the v2 template is dropped, because no author wrote it), then tells you to commit the canon and the generated views. Thread directories stay where v2 put them, status-bucket directories included.

Then read a thread you know well and check it against what you remember writing. **Do this before you do anything else with the project**: the longer you work on top of a migration you have not checked, the harder any recovery gets.

### A project older than v2.19.0

**v3 converts a project that v2.19.0 last upgraded, and nothing older.** Bring an older project to v2.19.0 with v2's own `intent upgrade` first: v2.19.0 is the `v2.19.0` tag of this repository. Two things v3 tells you on an older project are wrong:

- A project stamped with an older v2 version is refused with a remedy that says to run `install intent@2 && intent upgrade`. No tap provides an `intent@2` formula.
- A project from before v2.10.0, which keeps its config at a top-level `.intent/`, is not recognised as a project at all: v3 says `no Intent project found` and suggests `intent init`. **Do not run `intent init` on it.** Upgrade it with v2.19.0.

## If you migrated with v3.0.0

**v3.0.0 dropped the evidence clause of a criterion authored unsatisfied.** In v2 you could write a criterion that said, in effect, "not satisfied, and here is what we have so far" (`-- evidence: ... -- satisfied: no`). v3.0.0's unsatisfied state carried no payload, so the clause had nowhere to go, and the migration discarded it without a warning. The resulting row looks exactly like a criterion that never had evidence.

**v3.0.1 carries the clause** as the unsatisfied criterion's note, which `intent ac show` prints. So a project you migrate today is not affected.

**A project already migrated with v3.0.0 does not recover by re-running `intent upgrade`.** On a project with committed canon, `upgrade` re-emits each thread from that canon rather than converting the v2 markdown again, so the lost clause stays lost. Your v2 source in git history still holds it: read the pre-hop `acceptance.md`, and write each clause back with `intent ac edit <ID> <AC> --note "<clause>"`.

### How to tell whether you are affected

**Count rows whose pre-hop authored form carried an evidence clause together with `satisfied: no`.** Read your own v2 source out of git history at the path the migration read; you do not need to compare against anything v3 produced, and you do not need to trust anyone's number but your own.

Three things a hand-rolled scanner gets wrong, and each of them is worth more than the query:

**Template scaffold is not an authored claim.** A bracketed evidence value, such as `[named evidence]` or `[a doc / eyeball / gate criterion]`, is the placeholder the v2 template shipped. Nothing was destroyed because nothing was ever said. **This is the filter a hand-written scanner misses**, and on a real fleet it was a large share of the apparent exposure.

**Generated views are not authored source.** After the hop, the file at the same path is a v3 generated view carrying a banner. Counting it reads v3's output as v2's input.

**Scan per thread, never per path.** v2 kept threads in status-bucket directories, and a project that collapsed them into a flat layout has **several historical paths for one thread, each holding a frozen snapshot** from whenever it left that bucket. A scanner walking every `acceptance.md` counts one criterion once per bucket it ever sat in, each at whatever stale verdict that snapshot froze. That is not merely double-counting: a criterion can read `satisfied: no` at an old snapshot and `satisfied: yes` later, so a stale path makes a safe row look destroyed.

### Exposure is not damage, and a zero is not always a zero

**An exposed row is one that would have lost evidence, not one confirmed to have lost it.** Confirming a row means comparing your canon against your own authored source, which is a second step. Until you have done it, say "predicted, unconfirmed" in those words.

**And "nothing was measured" prints the same headline as "nothing is exposed".** If no v2-authored form could be recovered at all, that has two causes which look identical from git: the project was **born under v3** (genuinely nothing to lose), or its **v2 history was squashed or imported** (the v2 form existed and is gone, and this is not measurable from git). A confident zero that cannot tell those apart is a zero by construction.

## What changes

|                          | v2                                              | v3                                                                                                     |
| ------------------------ | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| **Source of truth**      | Markdown files, parsed                          | A store with a schema; files are generated                                                             |
| **What git reviews**     | The Markdown                                    | JSON canon extracts at `intent/.canon/`                                                                |
| **Config location**      | `.intent/` at the top level                     | `intent/.config/`                                                                                      |
| **Languages**            | Detected from files present                     | Declared in config, explicitly                                                                         |
| **Criterion state**      | `satisfied: yes/no` plus a separate scope field | One state; a state that needs a record carries it (evidence, a note, a reason, the thread it moved to) |
| **Editing a thread doc** | Edit the file                                   | Use a verb                                                                                             |

**The last row is the one that changes daily habits.** `info.md` and `acceptance.md` under `intent/st/<ID>/` are generated in v3. A hand-edit to one never reaches the store: `intent doctor` reports it as view-skew, and the next render overwrites it. See [The store](concepts/the-store.md) for the edit path that works.

**These v2 commands are retired** and refuse at exit 2 with `` `intent <command>` was retired in Intent v3 and is not a command in this build ``: `st organize`, `st repair`, `st_zero`, `st bootstrap`, `issues hydrate`, `issues dehydrate`, `lang sync`, `treeindex`, `agents template`, `claude prime`. A command v3 declares but has not built also refuses at exit 2, with `` `<command>` is a known command that is not implemented yet ``, so it is the message, not the exit code, that tells the two apart. `intent organize` is a v3 command: it reconciles the tree with `intent/.intentfiles`.

**`intent help` and `intent <command> help` answer in v3.0.1**, as `intent --help` and `intent <command> --help` do. On v3.0.0 both refused, and `intent help`'s refusal said wrongly that there was no replacement; a script written against v3.0.0 that switched to `--help` works on both.

**`intent llm usage_rules` answers about a different file in v3, and its exit code flips when you have no project-owned rules.** v2 printed `$INTENT_HOME/usage-rules.md`, **the copy inside the Intent install**, so every project on a machine got the same bytes. v3 prints **your project's own root `usage-rules.md`**, which is what an operator asking _what are the rules here_ means, and is the only answer that cannot silently print rules your project has edited away from. The consequence to script against:

| your project                | v2                         | v3                                      |
| --------------------------- | -------------------------- | --------------------------------------- |
| has a root `usage-rules.md` | exit 0, the INSTALL's copy | exit 0, YOUR file                       |
| has none                    | exit 0, the INSTALL's copy | **exit 1**, with a remedy and no output |

If you have a consumer that pipes this verb, the second row is the one that will bite: seed the file with `intent claude upgrade --apply`, which writes it only when absent and never touches it afterwards.

## The honest cost

**This is a rewrite, and a rewrite is not a version bump.** Budget for the hop as real work rather than a command:

- **The migration itself** is fast, and it is the smallest part.
- **Verifying it** is the real cost. Every thread you care about wants a read-through against what you remember, and the more history the project has the longer that takes.
- **Habits change.** The generated-views rule catches people once, and it is cheap to learn and annoying to learn by accident.
- **Tooling you wrote against v2's files will break**, because the files are now outputs. Anything that parsed `info.md` wants rewriting against the canon extracts or the CLI's own output.

**Do it on a project you can afford to be wrong about first.**

---

Back to [the documentation index](index.md).
