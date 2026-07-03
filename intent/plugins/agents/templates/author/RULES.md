# RULES-author.md

Mandatory authoring rules for this project's prose and courseware. Enforced by the `author` rule pack and the `critic-author` subagent.

## The Two Tiers

Author rules split by how they are checked:

- **style (mechanical, default)** -- greppable house-style and trope checks: banned filler and `eg`-not-`e.g.`, no vanity metrics, front-matter and learning-objectives present, heading hygiene, and the mechanical trope pass. Runs on every `critic-author review`.
- **craft (judgment, on instruction)** -- critic-as-reader: voice and register consistency, cross-chapter continuity, citation and attribution discipline, and the full `/in-detrope` diagnosis. Runs via `critic-author craft-check` (or `/in-detrope`) when asked.

Read the full author rule pack via the installed Intent tool: `intent claude rules list --lang author` to enumerate, `intent claude rules show <id>` to read one.

## NEVER DO

- NEVER ship LLM-assisted prose without the mechanical trope pass (`IN-AU-STYLE-005`); run the full `/in-detrope` before publication (`IN-AU-CRAFT-003`).
- NEVER write `e.g.` or `i.e.` -- house style is `eg` / `ie`; avoid the banned filler in `IN-AU-STYLE-001`.
- NEVER pad prose with vanity metrics or unearned numbers (`IN-AU-STYLE-002`).
- NEVER fabricate, misattribute, or leave unsourced a quotation, statistic, or named study (`IN-AU-CRAFT-004`).
- NEVER let voice or register drift into marketing copy across chapters (`IN-AU-CRAFT-001`).
- NEVER silently redefine a term an earlier chapter established (`IN-AU-CRAFT-002`).
- NEVER manually wrap lines in markdown files.

## Detrope Every Step

detrope has two forms and one home -- `intent/plugins/claude/skills/in-detrope/data/trope-catalog.md`. The mechanical grep runs by default; the full `/in-detrope` LLM diagnosis runs under direct instruction. Detrope at every content-production step; never fork the trope catalogue.

## Project-Specific Rules

<!-- Add project-specific authoring rules below this line. Cite IN-AU-* IDs where applicable. -->
