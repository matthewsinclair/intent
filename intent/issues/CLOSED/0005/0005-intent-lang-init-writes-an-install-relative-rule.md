---
id: "0005"
title: intent lang init writes an install-relative rule-pack path into the consumer project's RULES.md, where it dangles
date: 2026-07-27
reporter: matts
status: CLOSED
severity: medium
---

# 0005: intent lang init writes an install-relative rule-pack path into the consumer project's RULES.md, where it dangles

## Tags

lang, rules, docs, consumer-project

## Summary

`intent lang init <lang>` appends an entry to the consumer project's `intent/llm/RULES.md` reading ``- **<lang>** -- rule pack at `intent/plugins/claude/rules/<lang>/`; concretised RULES at `intent/llm/RULES-<lang>.md`.`` The second path is correct. **The first is correct only relative to the Intent installation root, and it is written verbatim into a file that lives in a different repository**, where it reads as project-relative and resolves to nothing.

Nothing is functionally broken -- rule resolution works fine -- so this is a pointer/documentation defect rather than a failure. It is raised because the dangling path appears inside a rules file that both humans and coding agents read specifically to find out where rules live, and following it costs real time before the reader concludes the tree is simply absent.

## Reproduction

In any consumer project that has run `intent lang init` (observed in Lamplight, `~/Devel/prj/Lamplight`, Intent v2.17.3):

```
$ ls -d intent/plugins
ls: intent/plugins: No such file or directory

$ rg -n 'rule pack at' intent/llm/RULES.md
521:- **elixir** -- rule pack at `intent/plugins/claude/rules/elixir/`; concretised RULES at `intent/llm/RULES-elixir.md`.
522:- **lua** -- rule pack at `intent/plugins/claude/rules/lua/`; ...
523:- **swift** -- ...
524:- **rust** -- ...
526:- **author** -- ...
527:- **content** -- ...
```

Six entries, six dangling paths. Resolution itself is healthy, and its output is what shows the real home:

```
$ intent claude rules show IN-EX-CODE-006
# Rule: IN-EX-CODE-006
# Provenance: canon
# Source: /Users/matts/Devel/prj/Intent/intent/plugins/claude/rules/elixir/code/module-highlander/RULE.md
```

## Root Cause

`bin/intent_lang:177` builds the entry with the path hard-coded as a literal:

```sh
local entry="- **${lang}** -- rule pack at \`intent/plugins/claude/rules/${lang}/\`; concretised RULES at \`intent/llm/RULES-${lang}.md\`."
```

That literal is accurate inside the Intent repository, where `intent/plugins/claude/rules/` genuinely exists. It is emitted unchanged into the target project.

**`intent lang init` never creates the directory it points at.** Per `lang_show`, the subcommand installs exactly two files per language -- `intent/llm/RULES-<lang>.md` and `intent/llm/ARCHITECTURE-<lang>.md`. Searching `bin/` and `lib/` finds no command that vendors `intent/plugins/` into a consumer project, so the pointer is dangling by construction in every project rather than being a per-project misconfiguration.

The same literal appears in the help text (`lib/help/lang.help.md:17`, `lib/help/rules.help.md:15`) and in `bin/intent_lang:104`'s console output, which is consistent -- and consistently ambiguous about which root the path is relative to.

## Impact

Low functional, non-trivial navigational. The written contract of a consumer project's `RULES.md` points its readers at a directory that is not there, in the one section whose job is to say where the rules are. It is also inside an `<!-- intent-lang-packs:start -->` managed block, so a project cannot correct it locally -- a hand-edit is regenerated away on the next `intent lang` run. That is what makes it worth a tool fix rather than a downstream one.

## Proposed Fix

Any of these would close it; the first is the smallest:

1. **Point at the command, not the filesystem.** Replace the path with the resolution the reader can actually run: ``- **<lang>** -- rules via `intent claude rules list --lang <lang>` (served by the installed Intent tool, not vendored here); concretised RULES at `intent/llm/RULES-<lang>.md`.`` This also matches what the generated `CLAUDE.md` already tells projects.
2. **Qualify the root** -- state that the path is relative to the Intent installation, so a reader knows not to look locally.
3. **Emit the resolved absolute path** at init time, the way `intent claude rules show` reports `# Source:`. Accurate, but it bakes one machine's layout into a committed file, so it is the weakest of the three.

## Minor, same function -- worth folding into the same fix

`lang_packs_add_entry` inserts a blank line around each appended entry, so the managed block accrues inconsistent spacing as languages are added. Observed in Lamplight after a seventh `intent lang init`:

```
- **author** -- ...
- **content** -- ...
                          <- blank
- **shell** -- ...
                          <- blank
<!-- intent-lang-packs:end -->
```

Purely cosmetic, and only noted because anyone fixing the path above will already be inside that function.

## Related

- Observed while auditing the `intent/llm/` documentation lane in Lamplight, where five hand-maintained `RULES-<lang>.md` files carried the same dangling path and were corrected locally. Only the `RULES.md` block could not be, because it is tool-managed.

## Resolutions

FIXED + CLOSED (2026-07-29), shipped in v2.17.4, together with 0008 (its generator-into-consumer twin). Proposed fix 1 is implemented. The minor spacing item does NOT reproduce from the tool -- evidence below.

**1. The entry names the command, not a path.** `lang_packs_entry()` is new in `bin/intent_lang` and is the single source of the entry text; it now reads ``- **<lang>** -- rules via `intent claude rules list --lang <lang>` (served by the installed Intent tool, not vendored into this project); concretised RULES at `intent/llm/RULES-<lang>.md`.`` This is resolution the reader can actually run, it is accurate in any repository, and it matches what the generated `CLAUDE.md` already tells projects. The console output at `bin/intent_lang:104` and `lib/help/lang.help.md` were updated to say the same thing. `lib/help/rules.help.md:15` was qualified rather than rewritten -- its paths genuinely refer to the Intent installation (it is the rule-*authoring* guide), so it now says so explicitly and gives `$INTENT_HOME/...`.

**Widened beyond the report: the fix had to heal existing projects.** As specified, changing the string alone would have fixed nothing anywhere it matters. `lang_packs_add_entry` returned early whenever any entry for the language existed, so every project that had already run `intent lang init` -- which is every affected project, by definition -- would have kept the dangling path for good: a tool-managed block that a tool re-run would not correct and a hand-edit would not survive. It is now an upsert: insert when absent, REWRITE when present but not canonical, no-op when already right. A single `intent lang init <lang>` heals a stale entry, which is the fail-forward shape -- the generator owns the block, so the generator repairs it.

**Highlander.** The needle identifying an entry existed in three places, each hard-coding the old wording (`lang_packs_add_entry`, `lang_packs_remove_entry`, and the `lang remove` call site at `bin/intent_lang:378`). It is now `lang_packs_entry_needle()`, matching `- **<lang>** -- ` in ANY revision of the wording -- which is exactly what lets the upsert and the removal recognise a pre-fix entry. The third copy was found by testing rather than by reading: `lang remove` silently stopped removing entries until it was updated too.

**A trap worth recording.** Both the entry and the needle begin with `- `, so `grep -F "$needle"` parses them as an option bundle and fails. Every such grep now passes `--`. The pre-fix needle began with `*`, which is why this only surfaced on the way out.

**Minor spacing item -- does not reproduce.** Three sequential `intent lang init` runs, plus repeated re-runs and a remove/re-add cycle, produce a block with no blank lines between entries and none before the end marker. Neither `lang_packs_add_entry` nor `lang_packs_insert_block` emits one. The Lamplight observation is most likely a markdown formatter running on save (the entries sit in a list inside a managed block, which is the kind of thing prettier reflows) rather than tool output. Nothing was changed for it -- inventing a fix for a defect that cannot be reproduced would be worse than leaving it. Raise it again with a reproduction if it recurs after this release.

**Guards.** Two tests added to `tests/unit/intent_lang.bats`: the entry names the command and no longer names a directory the project does not have (asserting the directory's absence too, so the test cannot pass vacuously); and a re-run heals a seeded old-form entry in place without duplicating it. Four existing assertions on the old wording were updated. Full unit suite (1105 tests) and both integration files green.
