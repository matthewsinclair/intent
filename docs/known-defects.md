# Known defects in v3.0.0

**Every defect on this page has been run against v3.0.0 itself.** Not against `main`, not inferred from our issue register: driven against the published binary, which reports `intent 3.0.0 (80d8b2ca)`. Where a claim could not be driven it is not on the page, and the last section says what that leaves out.

That distinction is not pedantry. The first draft of this page was written from our issue titles and it was wrong in both directions: it described defects that arrived after the tag and are not in this build, and it described one that is real with the wrong symptom. Driving it against the shipped binary was the only thing that found either.

**A defect is on this page if you can hit it by following the documentation correctly.** Something that only bites a maintainer editing the register, or a team sharing one checkout, is recorded against the issue rather than here.

Two things worth knowing before the list. **An issue being closed in our register does not mean the fix is in your build** — the register tracks `main`, and several fixes landed after the tag. And **an issue being open does not always mean it is still broken.**

## A stray directory disables the whole project

**Any `STnnnn` directory anywhere under `intent/st/` is picked up as a thread, and one it cannot read stops every command** (`intent#0011`). A staging or scratch copy at `intent/st/staging/ST0099/` is enough. What you get is not a duplicate row in a listing, it is:

```
  error: this project has not been migrated to Intent v3 -- it declares Intent 3.0.0,
         and 1 steel thread carries v2 canon this binary cannot read (ST0099)
  remedy: run `intent upgrade` to migrate this project to Intent v3
```

on every verb, including ones that have nothing to do with the stray thread. The remedy is misleading: the project is fine and one directory is not. Keep working copies of threads outside `intent/st/`.

**A second route reaches the same wall.** A thread placed under an `_inbox/` status directory produces the identical stop (`intent#0066`), rather than the invisibility that issue records. Two different stray-directory shapes, one symptom, and in both cases the message names a thread the operator never created.

## Threads

**`intent st list` shows only in-progress threads and discloses the filter nowhere** (`intent#0121`). Driven: two threads, one `WIP` and one `Triage`; the default listing shows one row and `--status all` shows both, with nothing in the default output indicating that anything was filtered. A short list reads as a short project.

**`intent st hydrate` reports `exists` for a file it has just created** (`intent#0083`). Driven by deleting `info.md` and hydrating: the file is restored and reported with the same `exists:` prefix as the one that was already there. The output cannot distinguish a restore from a no-op, so you cannot tell what it did.

**A migrated v2 project loses some thread slugs** (`intent#0080`). Driven against a real 2.19.0 estate captured from this repository's history and migrated by v3.0.0: 21 of 56 threads come through with no slug. The register records this as affecting every migrated thread and blanking the whole column; it does not. The `Slug` column renders normally and most threads carry one, so the ones that do not are easy to miss.

**A mistyped subcommand becomes a thread or an issue, silently** (`intent#0223`). Driven on v3.0.0: `intent st new help` returns `created: ST0001` and `intent issues add help` returns `created: intent/.canon/issues/0001.json`, both at exit 0. The word after the verb is a title positional, so any bare subcommand or flag name typed there -- `help`, `start`, `severity` -- is accepted as the title and written to your project. The two arms that should catch it both work: `--help` prints help, and omitting the argument entirely refuses at exit 1. It is the bare word that lands.

The row cannot be repaired afterwards: an issue title and body are write-once (`intent#0151`, `intent#0090`), so a thread or issue created this way can only be closed, never renamed. **This project's own register carries five of them** -- two issues and three threads, all titled after intent subcommands, all created in one sitting.

## Criteria and tests

**`intent ac new` on an id that already exists replaces the row instead of refusing** (`intent#0119`). Driven: creating `AC-01.1` twice returns `ok: AC-01.1 created` both times. The replacement is a full write, so a field you do not supply is written empty rather than preserved, and there is no verb that edits a criterion in place. Treat `ac new` as create-only and read `ac list` before re-running it on an id you are unsure of. **v3.0.1 closes this**; on v3.0.0 the verb that repairs is the verb that destroys.

**`intent ac list` never prints the criterion text** (`intent#0168`). Driven: a criterion whose text is a distinctive sentence does not have that sentence anywhere in the listing. It prints ids, coverage and satisfaction, so planning from its output means planning from ids. There is no `ac show`.

**A criterion cannot record what would discharge it until it is discharged** (`intent#0211`). `intent ac satisfy` is the only verb that takes evidence and `--evidence <ref>` is required on it, so there is no way to write down what a criterion is waiting for while it is still open. Driven on v3.0.0: `ac satisfy --help` reads `Usage: intent ac satisfy --evidence <ref> <STID> <ACID>`. Planning notes for an open criterion have to live outside the tool.

**`intent at lint --fix` is advertised and refuses** (`intent#0139`). `at lint --help` documents it as _Migrate the mechanical part of a legacy row_; calling it exits non-zero without doing so.

## Editing

**No verb writes a thread's title, objective, context, body or preamble** (`intent#0185`). Driven: `intent st edit ST0001 info` refuses with `is generated from the model, so an edit here is lost at the next render`, and its remedy says to author it with `intent st` — which has no verb that writes those fields. Edit `intent/.canon/st/<ID>.json`, then `intent sync --to-store`, then `intent sync --to-disk`.

**`intent issues add` creates an issue whose body no verb can write** (`intent#0090`). Driven: the created issue has a body of length zero and nothing can fill it. Issue titles and work package bodies have the same write-once door (`intent#0154`).

**`intent edit` sends you to a route that gives the same refusal** (`intent#0153`). Driven: `intent edit ST0001` refuses with `is generated from the model, so an edit here is lost at the next render` and a remedy saying to author it with `intent st`. Following that remedy -- `intent st edit ST0001 info` -- produces the identical refusal. The remedy names the door you just came through.

**Addressing an issue with `intent edit` refuses with a remedy that offers the thing it just refused, and glues a bare `a` onto a vowel-initial noun** (`intent#0081`). Driven on v3.0.0, using the address form its own `--help` prescribes (`Usage: intent edit <ADDRESS> [FILE]`):

```
  $ intent edit intent:///issues/0001
  error: `issue` is not something that can be realised to disk ...
    remedy: address an ARTEFACT instead -- a thread or an issue. A `issue` has no files
    of its own, so there is nothing for realisation to create ...
```

Two things are wrong in one message. **The remedy says `a thread or an issue`** and an issue is exactly what was refused, so following it returns you to the refusal — that half is corrected after `v3.0.0`, which now says `a steel thread`. **`A \`issue\`` is the article bug** and it is not corrected: seven of the fourteen entity names are vowel-initial, and this site builds the article by hand rather than asking the noun for it. The refusal is still telling you the right thing; only its grammar and its remedy are wrong.

## Syncing

**`intent sync --to-store` reports two contradictory things in one breath** (`intent#0069`, `intent#0111`). Driven on a thread-scoped sync:

```
  note: the store and the extract agree; this restore overwrites nothing
  ok: store replaced from the extract, 1 thread(s)
```

The first line says nothing changed and the second says the store was replaced. A thread-scoped call also describes itself as acting on the whole store.

**A file authored in canon alone never reaches disk, and the sync says it worked** (`intent#0082`). Driven: an attachment added to `intent/.canon/st/ST0001.json` and then synced produces `ok: extract written for 1 thread(s)` and no file. The count of files under `intent/st/` does not move.

This bounds the canon-editing route that [Getting started](getting-started.md) uses for thread fields: editing canon and syncing works for a thread's `objective` and `context`, and does **not** work for adding a file. `intent st attach` is the writer for attachments, and no direction of `sync` is.

**Text appended to a generated view AFTER its `_Generated by Intent v...` banner is discarded, and `sync --to-store` reports success** (`intent#0192`). Driven:

```
  $ printf '\n## Hand Added\n\nTEXT\n' >> intent/st/ST0001/info.md
  $ intent sync --to-store
  ok: store rewritten from the canon extract, 2 thread(s)
  $ intent sync --to-disk && grep -c TEXT intent/st/ST0001/info.md
  0
```

The same section inserted **before** the banner is refused, by name, with the text left intact — so this is one hole in a working guard rather than a missing one. `intent doctor` does report the drift as `view-skew`, so the loss is detectable after the fact; what does not report it is the verb you ran to make the edit land. **Append above the banner, or put the text in `## Objective` or `## Context`, which are the two sections that round-trip.**

## Searching

**A hyphen in a search query is read as SQL and leaks the error** (`intent#0194`). Driven on both builds: `intent search canon-ignore` exits 1 with `sqlite: no such column: ignore`, while `intent search canon` returns hits normally. The query goes to FTS5 unescaped, so the hyphen is parsed as an operator and the term after it as a column name. Any query containing `-` fails the same way, which includes most of this project's own vocabulary -- `read-back`, `at-lint`, `to-write`. Quote nothing and search a single word; there is no escaping syntax that helps, because the escaping is missing on the tool's side of the call.

**What search gets right, so this is not read as worse than it is:** an unindexed project says so rather than returning an empty list, in the tool's own words -- `nothing is indexed, so this search could not have matched -- an empty result here does NOT mean <term> is absent`. That is the failure mode that would actually mislead a reader, and it is closed.

## The daemon

**`intentd --help` starts a daemon instead of printing help** (`intent#0162`). On v3.0.0 the binary inspects argv for `--version` and then serves regardless of what else is there, so any argument it does not recognise -- `--help` and `-h` included -- falls through to starting a real daemon under your real `$HOME`. It binds, it publishes, and it does not return. While it is up, every other Intent session on the machine has its store verbs refused at `rc=2` by a daemon nobody meant to start.

Do not type it. If you already have, find the process and stop it: `pgrep -fl intentd`, then `intent daemon stop` or kill the pid. **v3.0.1 closes this** -- the fixed binary prints usage for `--help` and refuses any other argument with a remedy, on the stated ground that starting a daemon by accident takes every session on the machine down together.

**How this entry was established, because the obvious check is the defect.** Running `intentd --help` on the published build to confirm the behaviour would reproduce the outage on the machine doing the checking. So the two binaries were compared statically instead: the v3.0.1 help text is absent from the v3.0.0 binary and present in the current one, with a control string both carry, so an unreadable binary cannot masquerade as an unfixed one. The behaviour itself was driven first-hand on 2026-08-30, once, before it was understood -- which is how it was found.

## Declared and not implemented

Each of these is listed in `--help` and refuses when called. Driven against v3.0.0, exit codes as shown.

**`intent daemon` and its subcommands** (`intent#0163`) — `error: daemon is a known command that is not implemented yet`, exit 2.

**`intent claude rules validate`** (`intent#0156`) — the same refusal at exit 2. The command works in the v2 shell estate.

**`intent ext remove`** (`intent#0177`) — `unrecognized subcommand`. `ext` creates and has no way to undo.

**`intent agents` on its own** (`intent#0175`) — exit 2, while `intent agents sync` and `intent agents validate` both work. The bare family verb is an unwired dispatcher, not a broken feature; run `intent agents --help` for the verbs that are wired.

## Declared, accepted, and ignored

These flags are documented in `--help`, accepted without complaint, and read by nothing. Passing one changes no behaviour and reports no error.

**`intent agents init --template`** (`intent#0180`) and **`intent llm usage_rules --symlink`** (`intent#0181`). Both confirmed present in v3.0.0's own help output.

**`intent claude upgrade` has no `--skip-settings`** (`intent#0143`). v2 had a flag to decline the Claude Code settings file. v3.0.0's `claude upgrade --help` offers only `--apply`, `--force` and `--help`, so there is no way to ask it to leave your settings alone.

**`intent claude skills uninstall --force`** (`intent#0078`). The flag is in `--help` and the call behind it takes no force argument at all, so passing it changes nothing. Driven on v3.0.0 against a skill this build did not write:

```
  $ intent claude skills uninstall hand-made --force
    hand-made                    removed (0 file(s)); left 1 this build did not install: SKILL.md
  ok: 1 changed, 0 already settled, 0 need a decision
```

The file is still on disk afterwards, and **that is the tool being careful rather than the tool failing** — it will not delete what it has no record of writing, and the per-skill line says so, gives the count, and names the file. **What is wrong is one line lower: the summary says `1 changed` when no file changed.** Read the per-skill line, not the total.

## Recorded against v3.0.0 and NOT present in it

These are in our register and you will not hit them on this build. They are listed because finding an open issue that describes your version is otherwise alarming.

**`intent st new` writes no files** (`intent#0079`) — not true of v3.0.0, where `st new` writes `info.md` and `acceptance.md`. This arrived after the tag.

**`intent edit` ignores the kind it was given** (`intent#0149`, `intent#0189`) — v3.0.0's `edit` does not accept `--path` at all, refusing with `unexpected argument`. Post-tag.

**`intent modules find` is unimplemented** (`intent#0067`) and **`modules check` routes you to `intent upgrade`** (`intent#0122`) -- neither is true of v3.0.0. Driven with a `MODULES.md` present, `modules find` returns the row, and `modules check` routes to `intent lang init`.

**`intent init` bakes an absolute path into `.claude/settings.json`** (`intent#0016`), **`st edit`'s refusal names an empty list** (`intent#0145`), and **`at edit` is kind-blind** (`intent#0146`) -- none reproduce on v3.0.0. The settings file carries no absolute path, the refusal names `acceptance.md, info.md`, and `at edit` does not exist there at all.

**`intent fc` dispatches on kind by hand** (`intent#0171`) and **`ac list` renders a fiat-closed criterion wrongly** (`intent#0137`) — neither `intent fc` nor `intent at fc` exists in v3.0.0; both refuse with `unrecognized subcommand`. Post-tag.

## Migrating from v2

The v2 ingest has its own defect set and its own recovery routes, covered where you meet them in [Migrating from v2](migrating-from-v2.md): evidence discarded silently from criteria authored unsatisfied (`intent#0133`), and a measurement that cannot tell "nothing was lost" from "nothing was measured" (`intent#0098`). `intent st repair` is declared retired and was never built in v3.0.0 either (`intent#0118`).

## What this page does not cover

**This is the driven set, not the whole register.** Intent's open issues include defects that need a condition this page's checks cannot create from a fresh project — a migrated v2 estate, a bucketed thread, an installed skill, a running daemon. Those are real and they are not described here, because describing an undriven defect is how the first version of this page came to be wrong.

If you hit something not listed, that is the gap rather than a surprise. The register is the fuller record, and `intent doctor` reports on your own project.

## Reading this against your own build

`intent --version` names the build you are on. Everything here was driven against `intent 3.0.0 (80d8b2ca)`, the published tag. A source install from `main` behaves differently, and several entries above are already fixed there.

**The register itself cannot tell you which build a row describes** (`intent#0191`). An issue says what is broken and carries no field naming the version it was broken in, so `intent issues list` mixes rows about the published tag with rows about `main`. This page is the partition, drawn by hand and by driving each row: what is above is what a v3.0.0 reader can hit, and the section before last names the rows that read alarmingly and are not present. **If you find an open issue that seems to describe your version, check here before believing it.**
