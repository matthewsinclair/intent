---
summary: How an LLM finds, reads and follows a devbin howto, and how to report back.
keywords: [howto, llm, agent, protocol, discovery, instructions]
---

# How to use `bin/devbin howto`

A howto is a set of instructions for an LLM. It tells you how to do one task the same way each time, in any project that uses devbin. Read this topic before you follow any other.

## Find a howto

1. Run `bin/devbin howto` to list every topic, with its name, its source and a one-line summary.
2. Run `bin/devbin howto '<glob>'` to list the topics whose name, summary or keywords match, eg `bin/devbin howto '*release*'`.
3. A list never prints a topic. Choose one name from it.

## Read a howto

1. Run `bin/devbin howto <topic>` with the exact name. Read all of the topic before you start.
2. The first line names the source: core (it ships with devbin, at the version that the line shows) or project (this repository's own topics, in `bin/.devbin/howto/`). The line also prints the topic's path.
3. A project topic overrides a core topic with the same name. Run `bin/devbin howto <topic> --core` to read the core copy.
4. If the name matches no topic, the command lists the nearest names and exits non-zero. Do not guess. Run it again with an exact name.
5. A topic can name other files in its folder. Read each one when the topic tells you to.

## Follow a howto

1. Do the steps in the order that the topic gives. Do not skip a step. If a step does not apply, say so in your report.
2. The user's words win over a howto. The project's own rules (eg `AGENTS.md`, `CLAUDE.md` and `usage-rules.md`) also win over a howto. If a step conflicts with either, stop at that step and ask.
3. A howto gives you no permission that you do not already have. If a step needs a permission, an approval or a decision that you do not have, stop at that step and ask.
4. Use the tools that the topic names, as it names them. A core topic puts the steps for one tool (eg Intent or devbin) in a block with that tool's label. Skip a block whose tool the project does not use.
5. Read each timestamp that you write from a clock (`date -u`). Never write one from memory.

## Report back

1. If the topic gives a report form, use it.
2. If not, report: the topic's first line; each step, done or not done, and why; what you changed, with paths and commit ids; and what is still open.

## Propose a new howto

1. A core topic belongs in devbin's `lib/howto/<topic>/index.md`. Propose it to devbin's maintainers.
2. A project topic goes in this repository's `bin/.devbin/howto/<topic>/index.md`, beside the project's own commands. The project owns that directory, and a devbin upgrade never changes it. Start the file with the header: a one-line summary and a list of keywords.
3. Write each step as one short instruction, in order, with each tool named exactly.
4. A core topic is generic. It names no client, no person and no private project, and it puts the steps for one tool in a block with that tool's label.
5. A project topic can name its own project's parts. It follows that project's rules on what it can name.
