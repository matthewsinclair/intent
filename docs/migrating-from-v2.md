# Migrating from v2

**Intent v2 was a Bash implementation; v3 is a Rust rewrite with a different storage model.** The hop is a one-way ingest, it is not reversible in place, and there is a known defect in the v3.0.0 ingest that you should check for before you do anything else.

A v2 binary **refuses a v3 project** at exit 2, deliberately. The two do not coexist in one tree.

## Check this first

**Criteria authored as unsatisfied _with an evidence clause_ lost the evidence when ingested by v3.0.0.**

In v2 you could write a criterion that said, in effect, "not satisfied, and here is what we have so far". In v3.0.0 the unsatisfied state carries no payload, so there was nowhere for that clause to go — and the ingest sent the case to a wildcard rather than refusing it. **The evidence was discarded silently.** Nothing warned, nothing logged, and the resulting row looks exactly like a criterion that never had evidence.

**This is a representable-state regression in the model, not a parser bug**, and the distinction decides your recovery route:

- **On a build that predates the fix — which includes the published v3.0.0 — do not re-run the ingest expecting recovery.** The model still cannot hold the state, so a second run destroys it again. Your v2 source in git history is the only recovery route.
- **On a build that carries the fix**, the state is representable and a re-ingest from your v2 source becomes a real recovery route.

**Ask your binary rather than trusting this page.** The fix landed in source on 2026-08-29 and is not an ancestor of the v3.0.0 tag; whether the build you are standing on carries it is a property you can test and a claim this document would go stale on.

### How to tell whether you are affected

**Count rows whose pre-hop authored form carried an evidence clause together with `satisfied: no`.** Read your own v2 source out of git history at the path the ingest read; you do not need to compare against anything v3 produced, and you do not need to trust anyone's number but your own.

Three things a hand-rolled scanner gets wrong, and each of them is worth more than the query:

**Template scaffold is not an authored claim.** A bracketed evidence value — `[named evidence]`, `[a doc / eyeball / gate criterion]` — is the placeholder the v2 template shipped. Nothing was destroyed because nothing was ever said. On one fleet sweep this was over a third of all counted rows, and for one project it was the entire apparent exposure. **This is the filter a hand-written scanner misses.**

**Generated views are not authored source.** After the hop, the file at the same path is a v3 generated view carrying a banner. Counting it reads v3's output as v2's input.

**Scan per thread, never per path.** v2 kept threads in status-bucket directories, and projects collapsed those into a flat layout before hopping — so **one thread has several historical paths, each holding a frozen snapshot** from whenever it left that bucket. A scanner walking every `acceptance.md` counts one criterion once per bucket it ever sat in, each at whatever stale verdict that snapshot froze. That is not merely double-counting: a criterion can read `satisfied: no` at a July snapshot and `satisfied: yes` post-collapse, so a stale path makes a safe row look destroyed. **Correcting this took one fleet's aggregate down by more than a factor of three, and took three projects to zero.**

### Exposure is not damage, and a zero is not always a zero

**An exposed row is one that would have lost evidence, not one confirmed to have lost it.** Confirming a row means comparing your canon against your own authored source — a second step. Until you have done it, say "predicted, unconfirmed" in those words.

**And "nothing was measured" prints the same headline as "nothing is exposed".** If no v2-authored form could be recovered at all, that has two causes which look identical from git: the project was **born under v3** (genuinely nothing to lose), or its **v2 history was squashed or imported** (the v2 form existed and is gone, and this is not measurable from git). A confident zero that cannot tell those apart is a zero by construction.

## What changes

|                          | v2                                              | v3                                                     |
| ------------------------ | ----------------------------------------------- | ------------------------------------------------------ |
| **Source of truth**      | Markdown files, parsed                          | A store with a schema; files are generated             |
| **What git reviews**     | The Markdown                                    | JSON canon extracts at `intent/.canon/`                |
| **Config location**      | `.intent/` at the top level                     | `intent/.config/`                                      |
| **Languages**            | Detected from files present                     | Declared in config, explicitly                         |
| **Criterion state**      | `satisfied: yes/no` plus a separate scope field | One state enum with six values, four carrying payloads |
| **Editing a thread doc** | Edit the file                                   | Use a verb, or edit canon and sync                     |

**The last row is the one that changes daily habits.** Files under `intent/st/<ID>/` are generated in v3. Editing one is not reported as an error; it is silently overwritten at the next sync. See [The store](concepts/the-store.md) for the edit path that works.

**Eight v2 commands are retired** and refuse with an exit code that distinguishes removed from never-built: `st organize`, `issues hydrate`, `issues dehydrate`, `organize`, `lang sync`, `treeindex`, `help`, `st_zero`.

**A ninth, `st repair`, is declared retired at `HEAD` and was never built in v3.0.0 either.** In the published tag it answered `is a known command that is not implemented yet`; it now answers `was retired in Intent v3`. **Nothing you can run changes across the hop** -- only the message you get for running something that never worked.

**This paragraph previously said `st repair` works in the release you are hopping onto, and cited the register for it: `the published tag's register declares 117 commands including it`.** The citation was accurate and the conclusion was not. **That register's `shipped` population is derived as everything declared minus everything retired, so a command that was declared, never implemented, and not marked retired is counted in it.** It answers _is this row declared and live_, never _was this command ever built_, and the count itself does not say which. If you are deriving anything from our register, that is the distinction to hold.

**One of the eight lies to you about its replacement** (`intent#0086`). On `v3.0.0`, `intent help` refuses at exit 2 with `` `intent help` was retired in Intent v3 and is not a command in this build ``, and offers the remedy `there is no v3 replacement -- remove it from any script that calls it`. **That remedy is false.** `intent --help` answers on the same build at exit 0 and prints the command list. The capability is present by one spelling and absent by the other, and **the refusal tells you to delete the call when changing one character would have worked**. At `HEAD` the retirement is reversed and `intent help` answers again, so this is a defect of the published tag rather than of v3 as such.

**`intent llm usage_rules` answers about a different file in v3, and its exit code flips when you have no project-owned rules** (`intent#0215`). v2 printed `$INTENT_HOME/usage-rules.md` -- **the copy inside the Intent install** -- so every project on a machine got the same bytes. v3 prints **your project's own root `usage-rules.md`**, which is what an operator asking _what are the rules here_ means, and is the only answer that cannot silently print rules your project has edited away from. The consequence to script against:

| your project                | v2                         | v3                                      |
| --------------------------- | -------------------------- | --------------------------------------- |
| has a root `usage-rules.md` | exit 0, the INSTALL's copy | exit 0, YOUR file                       |
| has none                    | exit 0, the INSTALL's copy | **exit 1**, with a remedy and no output |

If you have a consumer that pipes this verb, the second row is the one that will bite: seed the file with `intent claude upgrade --apply`, which writes it only when absent and never touches it afterwards.

**This was labelled `v2 parity` in our own source until 2026-09-02, and the reason it survived is worth more to you than the fix.** Intent is developed in a checkout of itself, and **in a self-hosted checkout the install root and the project root are the same directory** -- so `$INTENT_HOME/usage-rules.md` and `<project>/usage-rules.md` are one file, and every comparison we ran agreed. The divergence exists only in a project that is not Intent, which is every project except ours. If you are checking our parity claims against your own tree, that is the class to check first.

## Doing the hop

```
  $ intent ingest
  $ intent doctor
  $ intent st list
```

Then read a thread you know well and check it against what you remember writing. **Do this before you do anything else with the project**, because the ingest defect above is silent and the longer you work on top of it the harder the recovery gets.

## The honest cost

**This is a rewrite, and a rewrite is not a version bump.** Budget for the hop as real work rather than a command:

- **The ingest itself** is fast, and it is the smallest part.
- **Verifying it** is the real cost. Every thread you care about wants a read-through against what you remember, and the more history the project has the longer that takes.
- **Habits change.** The generated-views rule and the sync-before-commit ordering both catch people once, and both are cheap to learn and annoying to learn by accident.
- **Tooling you wrote against v2's files will break**, because the files are now outputs. Anything that parsed `info.md` wants rewriting against the canon extracts or the CLI's own output.

**Do it on a project you can afford to be wrong about first.** The ingest defect above was found on real projects, after the hop, by people who were not looking for it.

---

Back to [the documentation index](index.md).

**`intent <family> help` does not answer, on any family** (`intent#0203`). In v2, `intent st help` and its siblings printed the family's usage. On `v3.0.0` all nine families -- `st`, `wp`, `ac`, `at`, `todo`, `issues`, `claude`, `agents`, `lang` -- answer `error: unrecognized subcommand 'help'` at exit 1. **Use `--help` instead: `intent st --help` answers on the same build.** The top-level spelling is the confusing part, because the two diverged: bare `intent help` also refuses on `v3.0.0` (that is `intent#0086`, whose remedy text is itself wrong), and it has since been restored at `HEAD` while the per-family spelling has not. So a script that called `intent wp help` needs `intent wp --help`, and it needs it on every version of v3 published so far.
