# Command reference

**This section is generated, and it is generated against a specific release rather than against `main`.** That distinction is the reason it exists as its own build step instead of being written by hand.

## Why it is not written by hand

Intent's command surface is declared in a register — `surface/dispatch-table.json` — which is the one home for what verbs exist, what arguments and flags they take, and what they do. **A hand-typed reference beside that register would be a transcribed copy of a measured mapping, and it would drift from the thing it copies.** So the reference is emitted from the register by a generator, and the emitted output names the revision it was made from.

## Why the revision matters more than you would expect

**The register describes `main`. A release is a point on `main`, and the two are not the same claim.**

This has already bitten. At the time of writing, `ac edit` and `at edit` are declared shipped in the register and are in no published release. A reference written from the register without pinning a revision would document verbs the installed tool does not have — and a reader following it would get `error:` from a command the documentation told them to run.

So the generated reference states, per verb, whether it is present in the release this documentation describes. **A verb in the register is not a verb in your binary.**

## Reading it against your own install

The register is a claim about a source tree. Your binary is the authority on itself:

```
  $ intent --version
  $ intent <family> --help
```

Where this reference and your binary disagree, **your binary is right** and the disagreement is worth reporting.

## The surface, by family

Intent ships 118 commands across these families. Each family's page covers its verbs, arguments, flags, exit codes and refusals.

| Family                                                              | What it manages                                                       |
| ------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `st`                                                                | Steel threads — create, move through states, show, attach, hydrate    |
| `wp`                                                                | Work packages                                                         |
| `ac`                                                                | Acceptance criteria, and the close gate                               |
| `at`                                                                | Acceptance tests                                                      |
| `issues`                                                            | Project issues                                                        |
| `todo`                                                              | The flat generated work view                                          |
| `claude`                                                            | Claude Code integration — rules, skills, hooks, whiteboard, subagents |
| `agents`                                                            | `AGENTS.md` generation and validation                                 |
| `lang`                                                              | Declared languages                                                    |
| `critic`                                                            | Per-language rule checking                                            |
| `ext`                                                               | User extensions                                                       |
| `plugin`                                                            | Plugin discovery                                                      |
| `config`                                                            | Project configuration                                                 |
| `sync`                                                              | Store, canon and disk reconciliation                                  |
| `modules`                                                           | The module registry                                                   |
| `init`, `upgrade`, `doctor`, `backup`, `export`, `search`, `ingest` | Project lifecycle and tooling                                         |

**Nine commands are retired** and refuse with an exit code that distinguishes "this was removed" from "this was never built": `st repair`, `st organize`, `issues hydrate`, `issues dehydrate`, `organize`, `lang sync`, `treeindex`, `help`, `st_zero`.

**Eight of those never shipped in v3 at all — but `st repair` did, and it is being removed.** Measured against the published tag's own register: `v3.0.0` shipped 117 commands **including `st repair`**; `HEAD` ships 118 with `st repair` retired. **So a v3.0.0 operator can run it today and it will disappear under them at the next release.** If you have it in a script, that is the one to look for.

---

_The per-family pages are generated by the cut-surface generator and land here keyed to the release they describe. Until they do, `intent <family> --help` is the authority._
