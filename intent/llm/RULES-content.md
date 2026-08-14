# RULES-content.md

Mandatory rules for this project's web content -- pages, posts, and marketing copy. Enforced by the shared `prose` base, the `content` rule pack, and the `critic-prose` subagent.

## The Two Tiers

Content rules split by how they are checked:

- **style (mechanical, default)** -- greppable checks: the shared prose hygiene (banned filler and `eg`-not-`e.g.`, no vanity metrics, heading hygiene, the mechanical trope pass) plus the content-specific web mechanics -- page meta present (title, description, canonical), image alt-text, descriptive link text. Runs on every `critic-prose review`.
- **craft (judgment, on instruction)** -- critic-as-reader: scannability and web voice, one clear primary call to action, and reading level matched to the audience. Runs via `critic-prose craft-check` (or `/in-detrope`) when asked.

Read the packs via the installed Intent tool: `intent claude rules list --lang prose` (the shared base) and `--lang content` (the web-specific rules); `intent claude rules show <id>` to read one.

## NEVER DO

- NEVER publish a page without `title`, `description`, and `canonical` meta (`IN-CO-STYLE-001`).
- NEVER ship a content image with no meaningful alt text; decorative images declare `alt=""` (`IN-CO-STYLE-002`).
- NEVER use "click here" / "read more" / a bare URL as link text (`IN-CO-STYLE-003`).
- NEVER ship LLM-assisted copy without the mechanical trope pass (`IN-PR-STYLE-004`); run the full `/in-detrope` before publication.
- NEVER write `e.g.` or `i.e.` -- house style is `eg` / `ie`; avoid the banned filler in `IN-PR-STYLE-001`.
- NEVER pad copy with vanity metrics or unearned numbers (`IN-PR-STYLE-002`).
- NEVER leave a conversion page with no primary call to action, or bury it under five competing ones (`IN-CO-CRAFT-002`).
- NEVER manually wrap lines in markdown files.

## Write for the Scanning Reader

Web readers scan before they read. Put the point first, keep paragraphs short, let subheadings carry the argument, and pitch the language to the audience you have (`IN-CO-CRAFT-001`, `IN-CO-CRAFT-003`).

## Detrope Every Step

detrope has two forms and one home -- `intent/plugins/claude/skills/in-detrope/data/trope-catalog.md`. The mechanical grep runs by default; the full `/in-detrope` LLM diagnosis runs under direct instruction. Detrope at every content-production step; never fork the trope catalogue.

## Project-Specific Rules

<!-- Add project-specific content rules below this line. Cite IN-CO-* / IN-PR-* IDs where applicable. -->
