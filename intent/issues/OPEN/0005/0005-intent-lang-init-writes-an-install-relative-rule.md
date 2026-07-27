---
id: "0005"
title: intent lang init writes an install-relative rule-pack path into the consumer project's RULES.md, where it dangles
date: 2026-07-27
reporter: matts
status: OPEN
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

{{TBC}}
