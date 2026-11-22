# Intent v3.5.1

**v3.5.1 fixes the places where Intent still read a git-style id as if it were an ordinal, or kept a second rule for something the rest of the tool already decides one way.** They were found by the close-time sweep of the thread that introduced git-style ids (ST0095, hv decision 86). Views list git-style records in the order every other listing uses, the acceptance gate and the label checks read git-style threads and packages, a remint carries every reference to the record it moves, and `st attach` writes a new attachment's file. Nothing changes the canon's format or the store's schema, so there is no upgrade step beyond installing the release.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** They were written from the CHANGELOG's 3.5.1 section as drafted against main at aa3533cb1. Each entry below was read against that section's text, and claims nothing it does not.

## Upgrading

**Install the release; there is nothing to run in a project.** A project already moved by 3.5.0's `intent upgrade` reads as it is, and the store keeps its schema.

## Fixed

### Order and ids

**`intent/todo.md` and every `acceptance.md` list git-style records in record order** (issue Ib383ad5). `todo.md` listed git-style threads by their hex, and `acceptance.md` placed a git-style package's section by its hex, while `steel_threads.md`, `st list` and the package list order them by when they were made. Both views now take the one order: ordinals by number, then git-style records by creation.

**The acceptance gate's citation check reads a git-style thread named in a cited line** (issue I88f333f). It read a thread id only as `ST` and four digits, so a line naming `ST3f9a2c1` was not seen to name a thread, and one whose hex opens with four digits, such as `ST1234abc`, was read as the ordinal `ST1234`. It now reads each word as every door does, so a git-style thread named by its short form is that thread, and a hex id is never an ordinal.

**The issue door says an address is an address** (issue Ic7d939d). An `intent:///` address given where an issue id is wanted got the not-an-id wording, where the thread doors said it was an address. Every door now words it from the id reader's own answer.

### Criteria and tests

**One reader decides what a criterion or test label is, and `ac new` and `at new` refuse a malformed one** (issues I91dd457 and I78176db). A precondition check refused `AC-<package>.<n>`, the label Intent itself stores for a git-style package, and `ac new` and `at new` stored any string as a label. A label is `AC` or `AT`, a group, a dot and digits, where the group is digits of any width or a git-style package's whole id, so `AC-1.1` and `AC-01.1` both stay valid. A malformed label is refused, naming the form, and nothing is written.

### Remint and attachments

**`st remint` moves a criterion's descope target and a fiat record's `inherited_from` with the thread** (issue I4fc4495). After a remint, a criterion descoped to the reminted thread or one of its packages, and a fiat record inherited from it, pointed at the old id, and doctor reported the descope as dangling. The remint now moves both and lists each rewrite, as it does for `related`.

**`intent st attach` writes a new attachment's file on a realised thread** (issue I1d5a3b5). It recorded the attachment in the store and canon but wrote no file until `intent organize --apply` or `intent st hydrate` ran, and doctor said nothing meanwhile. Whether the thread is realised is read from its record; the file and only its parent directories are created, the file is named under `wrote:`, and a store failure removes what was written. An attach over a held attachment (issue 0715) now also names its file under `wrote:`.

### Merges, git and doctor

**A merge version of a thread's file that does not read as a thread is refused, not offered as a side to take** (issue Ic70f05f). Whether both sides added work packages to a thread is read from the merge's three versions of its file, and one that would not parse or decode read as "no", so `intent sync` offered the file as a side to take, and taking it could drop the other side's packages. `sync` and `sync --apply --yes` now refuse, naming the file and the version, and write nothing.

**A git that refuses is an error where Intent asks what is staged or what decides `export-ignore`** (issue Ib7a70c3). Inside a repository git could not read, these questions answered "nothing", where every other question of git already said it could not ask (issue 0730).

**`intent doctor` no longer reports 3.4.0's `wp.renumber` events as an undeclared op** (issue I998bb84). 3.5.0 retired the op with `wp renumber`, and every store whose log held that history got the advisory for an op Intent itself wrote. A retired op stays known.

### Merge requests

**Open merge requests no longer block each other on `intent/.intentfiles` and the two generated lists, and issues 0704 and 0705 close** (ST0096). 3.5.0's format made this so in an upgraded project, as its notes describe: realisation is a field of each record, `.intentfiles` is gone, and the two lists and every `acceptance.md` are out of git. This release completes the thread's proof. Two merge requests that change one thread, one adding a work package and the other a criterion and its test, merge in turn as a forge merges them, with `intent doctor` clean after each. A fresh clone with no aggregate view on disk commits through the commit gate. A branch still in 3.4.0's format meets the conflicts it met before, and `intent upgrade` remains its one remedy.

### Documentation

**The installation guide names everything `intent upgrade` stages for deletion** (issue I7e5f67c): the aggregate views it takes out of git, `intent/.intentfiles`, and the closed records' views it removes, where it named only the two lists.
