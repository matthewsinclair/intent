    bin/devbin howto [<topic>|<glob>] [--core]

List, search and print instructions written for LLMs. A topic tells an LLM how to do one task the same way each time. devbin ships its own topics, and a project adds its own. Start with `bin/devbin howto forllms`, which says how to find, read and follow every other topic.

## The answers

    (none)     list every topic: its name, its source and its summary
    <topic>    print the topic, under a first line naming its source
    <glob>     list the topics whose name, summary or a keyword match it
    --core     read devbin's own topics alone

A word holding `*`, `?` or `[` is a glob. It matches in any case, against a topic's name, its whole summary or any one keyword, so `bin/devbin howto '*release*'` finds a topic whose summary names a release. A list never prints a topic.

An exact name prints the topic. Its first line is `howto <topic>: core, devbin <version> (<path>)` for one of devbin's own, or `howto <topic>: project (<path>)` for the project's, with the path relative to the project root. The topic follows, below its header.

A word that names no topic prints the nearest names, every one at the least edit distance from the word, and exits 1. A glob that matches no topic exits 1 too. `bin/devbin howto all` is such a word: howto has no options, so there is no `all` to run.

## Where topics live

devbin's own topics ship in the vendored library, at `bin/.devbin/lib/howto/<topic>/index.md`. A project's own topics live at `bin/.devbin/howto/<topic>/index.md`, beside its commands in `bin/.devbin/cmd/` and its pages in `bin/.devbin/help/`, and like them they are the project's: install and upgrade never write there. Any other file in a topic's folder is a part that its index names by relative path.

A project topic overrides devbin's topic of the same name. The list shows both, and marks devbin's as overridden. `--core` passes over the project's topics, so `bin/devbin howto <topic> --core` prints devbin's copy. `forllms` is devbin's alone: a project copy is listed as ignored and never printed.

## The header

Each `index.md` opens with a header, and the topic follows it:

    ---
    summary: One line that says what the topic is for.
    keywords: [release, cut, tag]
    ---

The header holds `summary` and `keywords`, each once, and nothing else. A keywords list too long for one line may run over indented lines below `keywords:`, as Prettier writes it. A header that is not that shape is refused at exit 1, with the file and what is wrong: no opening `---`, no closing `---`, an empty or missing summary, keywords that are not a list, or another key. A list prints the topics it can read and names each one it cannot.

## Exit codes and output

Exit 0 when the topic was printed or the list read, 1 otherwise. A second word, or an option other than `--core`, is refused at exit 1. howto writes nothing and prints no colour, on a terminal or off one.
