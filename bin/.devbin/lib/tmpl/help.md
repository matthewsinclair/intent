# {{DEVBIN}} {{NAME}}

{{SUMMARY}}

## Usage

```
{{DEVBIN}} {{NAME}} [args]
```

## Detail

Written by hand, printed verbatim by `{{DEVBIN}} help {{NAME}}`. Nothing generates or rewrites this file, so it is the place for the things a generator cannot know: why the command exists, what it assumes, and what has bitten people.

Sub-topics go in `{{NAME}}.d/<topic>.md` beside this file and are reached with `{{DEVBIN}} help {{NAME}} <topic>` -- the same descent the handlers use.

Delete this file and `{{DEVBIN}} help {{NAME}}` still answers, from the summary, usage and option list devbin can derive. Authored detail is an addition, never a prerequisite.
