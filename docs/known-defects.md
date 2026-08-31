# Known defects in v3.0.0

**This page is derived, not remembered.** Its members come from Intent's own issue register as it stood at the v3.0.0 cut, and the derivation is checked by `intent/st/ST0056/parity/tools/docs_defect_disposition_check.sh`, which refuses to report unless every member of that set is either stated here or recorded as something a reader cannot reach.

**A defect is on this page if you can hit it by following the documentation correctly.** That is the whole test. Something that only bites a maintainer editing the register, or a five-session team sharing one checkout, is recorded elsewhere and deliberately kept off this page.

Two things worth knowing before the list. **An issue being closed in our register does not mean the fix is in your build** — several entries below are closed on `main` and still live in v3.0.0, because the fix landed after the tag. And **an issue being open does not always mean it is still broken**; where we know that, the entry says so.

## Installing and upgrading

**`intent init` bakes an absolute path into `.claude/settings.json`** (`intent#0016`). The generated settings carry a path from the machine that generated them, so the file is not portable between machines or checkouts.

**`intent claude upgrade` is parsed but unwired** (`intent#0077`). It accepts the command and does nothing, so nothing regenerates the root `CLAUDE.md` in a project self-hosted on v3. Regenerate the agent contract with `intent agents sync`, which does work.

**There is no v3 equivalent of `intent claude upgrade --skip-settings`** (`intent#0143`). v2 had a flag to decline the Claude Code settings file; v3 does not, so `claude upgrade` has no way to leave your settings alone. Combined with the entry above this is currently moot, and it will stop being moot when that verb is wired.

**`intent upgrade` on a v2 project blocks on an interactive prompt with no terminal** (`intent#0071`). In CI, or any non-interactive context, it waits for a read that can never arrive. Run it from a terminal.

## Threads

**`intent st new` reports `created` and writes no files** (`intent#0079`). The thread is real and it is listed in `.intentfiles` exactly like a materialised one, but nothing appears on disk until you run `intent sync --to-disk`. The `--dehydrate` flag's own help implies the default writes them; it does not.

**`intent st list` shows only in-progress threads and does not say so** (`intent#0121`). Threads in other statuses are absent from the output with nothing indicating a filter is applied, so a short list reads as a short project. Pass `--status all` to see everything.

**`intent st list` counts any `STnnnn` directory at any depth** (`intent#0011`). A staging or scratch area under `intent/st/` becomes duplicate live threads in the listing. Keep working copies outside `intent/st/`.

**`intent st show` and `intent st list` cannot see threads in `_inbox/`** (`intent#0066`). Four of the five status directories resolve; `_inbox/` does not, so a thread there is invisible to `st show`, `st list`, `ac gate`, `at lint` and `todo` alike, and reads as a thread that does not exist rather than one that is filed elsewhere.

**A migrated thread has no slug, so `intent st list` renders a blank column for the whole estate** (`intent#0080`). `st new` computes a slug and migration does not, so the two doors that create threads disagree on the same field.

**`intent st hydrate` reports `exists` for a file it just created** (`intent#0083`). Its output cannot distinguish restoring a missing file from doing nothing, so you cannot tell from the result which happened.

**`intent st edit` writes on its own refusal** (`intent#0145`). For a known thread it can exit 1 and still have written, and the remedy it prints names an empty list, so the message does not tell you what to do next.

## Work packages

**`intent wp list` returns success and zero rows for threads whose work packages are in the store** (`intent#0103`). The rows exist and the listing does not show them, so an empty result does not mean an empty thread.

## Criteria and tests

**`intent ac new` on an id that already exists replaces the row instead of refusing** (`intent#0119`). The replacement is a full write, so any field you do not supply is written empty rather than preserved. In v3.0.0 there is no verb that edits a criterion in place, so the verb that repairs is the verb that destroys. The same shape applies to `at new`. Treat `ac new` as create-only and read `ac list` before re-running it on an id you are unsure about.

**`intent ac list` never prints the criterion text** (`intent#0168`). It prints ids, coverage and satisfaction, so planning from its output means planning from ids. There is no `ac show`.

**`intent ac list` renders a fiat-closed criterion as `satisfied: no`** (`intent#0137`). A second rendering site bypasses the one composer, so the listing disagrees with the criterion's actual state.

**`intent at lint --fix` is advertised in `--help` and refuses when called** (`intent#0139`). The flag is declared and answers that it is not implemented.

**`intent at green`'s help asserts a guard that is not enforced** (`intent#0142`). It says a green status is reachable only from red. That described v2; v3 does not enforce it, so a test can go straight from `to-write` to `green` without ever being observed failing. Go through `red` anyway.

**`intent at edit` ignores the kind it was given** (`intent#0146`), and the remedy it prints will walk a non-test row into carrying both a file and prose, which `at lint` has no rule for.

**An unsatisfied criterion's note can be written only by migration** (`intent#0140`). No verb authors or edits one, so the field is readable, publishable, and unreachable from the command line.

## Editing anything

**No verb writes a thread's title, objective, context, body or preamble** (`intent#0185`). The refusal you get if you try names `intent st` as the place to author those fields, and no `intent st` verb writes them. Edit `intent/.canon/st/<ID>.json` and run `intent sync --to-store`, then `intent sync --to-disk`.

**No entity's authored prose can be edited after creation** (`intent#0154`). Issue bodies, issue titles and work package bodies all have write-once doors, so a value that was wrong when it was created stays wrong.

**`intent issues add` creates a body that no verb can write** (`intent#0090`), and the refusal that makes it unreachable names the form rather than the field, so the message points at the wrong thing.

**`intent edit` accepts a kind and then answers about a different entity** (`intent#0149`, `intent#0189`). `intent edit issue 148 --path` refuses with `no steel thread ST0148 in this project`. The parser accepts the kind, the resolver drops it, and the refusal is about an entity you never named. Being told about a thread means the tool discarded the one piece of information it asked you for.

**`intent edit` refuses the address form its own remedy tells you to use** (`intent#0153`).

## Syncing and the store

**`intent sync` misdescribes its own scope, twice** (`intent#0069`). A thread-scoped `--to-store` reports that the whole store was replaced, and an unscoped run can report that the store and the extract agree while they hold different numbers of issues.

**`intent sync --to-store` can report overwriting nothing and replacing the store in one breath** (`intent#0111`), and its agreement verdict never examines an addition, so a new row is not what it checks.

**`intent sync --to-disk` does not materialise a new attachment** (`intent#0082`). A file authored in canon alone never reaches disk and nothing reports that it did not. `intent st attach` is the writer for attachments; no direction of `sync` is.

## The daemon

**`intentd --help` starts a daemon instead of printing help** (`intent#0162`). Asking the binary to describe itself runs it.

**`intent daemon start` and `intent daemon stop` are declared in `--help` and unwired** (`intent#0163`). They are listed as commands and do not do the thing they name.

## The agent contract, skills and rules

**`intent agents` on its own answers `not implemented yet`, while its subcommands work** (`intent#0175`). The bare family verb is an unwired dispatcher; `intent agents sync` and `intent agents validate` both run correctly. Run `intent agents --help` for the verbs that are wired.

**`intent agents init --template` is declared, documented in help, and accepted, and no renderer reads it** (`intent#0180`). Passing it changes nothing and reports nothing.

**`intent llm usage_rules --symlink` is declared, documented in help, and accepted, and the function behind it takes no arguments** (`intent#0181`). Same shape as the entry above.

**`intent claude skills uninstall --force` reports success while leaving the skill on disk** (`intent#0078`). It prints `ok: 1 changed` at exit 0 and the skill remains fully loadable.

**`intent claude rules validate` is declared and not implemented in v3** (`intent#0156`). It refuses at exit 2. The same command works in the v2 shell estate.

**`intent modules find` and `intent modules check` are not implemented in v3** (`intent#0067`), and their refusal routes you to `intent upgrade`, which cannot help (`intent#0122`). The remedy cites the canon as the authority for behaviour that canon stopped describing on 2026-08-24. If you need to query `MODULES.md`, grep it directly.

**`intent ext` creates and cannot remove** (`intent#0177`). There is no `ext remove`, so an extension you add through the tool has to be removed by hand.

## Other

**`intent todo notdone` and `intent todo toggle` mutate through helpers that bypass the facade** (`intent#0176`), which is the door every other write goes through.

**`intent fc` dispatches on kind by hand** (`intent#0171`), putting four different operations behind one verb with no single door, and exposes a one-way mutation over MCP.

**Two shipped messages glue a bare `a` in front of a vowel-initial noun** (`intent#0081`), so you will see `a issue` rather than `an issue`.

## Migrating from v2

The v2 ingest has its own defect set and its own recovery routes, and they are covered where you meet them: see [Migrating from v2](migrating-from-v2.md). In summary, the ingest can discard authored prose that sits between two recognised fields (`intent#0124`), splice a row so that text is lost at the head and duplicated in the middle (`intent#0126`), drop a note field entirely and leave a stub (`intent#0127`), rewrite an authored full stop into its own field delimiter (`intent#0129`), and it has two behaviours for one input shape of which only the destructive one is detectable (`intent#0138`). A retired refusal also leaves the ingest refusing a thread on a label the collector had already stopped honouring (`intent#0084`).

**Criteria authored as unsatisfied with an evidence clause lost the evidence when ingested by v3.0.0** (`intent#0133`), and [Migrating from v2](migrating-from-v2.md) covers how to tell whether your project is affected.

## How to read this page against your own build

`intent --version` names the build you are on. Everything here is measured against v3.0.0, the published tag. If you are on a source install from `main`, some of these are fixed and this page does not track that: it describes what shipped.
