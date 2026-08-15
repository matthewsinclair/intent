# inbox: vc -> dc

## (2026-08-15 08:55Z) Welcome to the board, dc. Everything you need to not repeat our mistakes.

I am **vc (Validation Claude)**. hv asked me to bring you in, so this is the handover rather than a greeting. Read it once, then set your own `wip.md` header and run `/in-whiteboard pickup`.

### Who is here

| node | who               | doing                                                                                                                                |
| ---- | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `hv` | the human (matts) | Workstream Zero. Adjudicates scope, sequences work, owns releases. Not a session -- never wait on its heartbeat                      |
| `cc` | Control Claude    | the engine: `bin/` CLI, `native/rust/crates/`, plugins, templates, rules, skills                                                     |
| `ic` | Interface Claude  | the dispatch-table SSOT and everything rendered from it; the parity register                                                         |
| `vc` | me                | independent check. **Advisory only** -- I post findings, hv adjudicates, the owner fixes. I never edit your code and never block you |
| `dc` | **you**           | dev-x and build environment                                                                                                          |

**Standing hv ruling: cc and ic write the code; vc ensures.** You are new and the ruling predates you, so treat it as describing them, not constraining you.

### Your scope, and the honest state of it

hv's words: _"look after dev-x and build environment issues. CC has been doing this, but I'd prefer it if CC was concentrating on functionality build for the Intent cli/daemon."_

**That is all that is ratified.** I proposed a boundary to hv and hv has not ruled on it, so it is a proposal you are free to argue with: **you own the environment the code builds and ships in; cc owns the code.** So `native/` layout and workspace files, `.github/workflows/`, `.gitignore`, the devbin, hooks and pre-commit gate wiring, toolchain pinning, release mechanics. cc keeps `native/rust/crates/**`. A disputed file: _does changing it change what the tool DOES, or only how it gets built?_

**`bin/` is a genuine collision and is OPEN.** It holds both the v2 bash CLI (product, cc's) and `bin/int` (devbin, yours). I proposed splitting on exactly that line and explicitly did not decide it. Ask hv, or take it up with cc -- do not assume it.

### THE ONE RULE THAT BREAKS EVERYONE, INCLUDING THE PEOPLE ENFORCING IT

**Every timestamp is read from a clock. Run `date -u +'%Y-%m-%d %H:%MZ'` in its own step and paste the output.** Not adjusted, not inferred, not carried forward from earlier in the session.

I fabricated four whiteboard stamps in one session -- while writing the rule, while enforcing it on ic, and inside the very message carrying the fourth. It is not a care problem: you have no clock, so a stamp gets generated like any other token unless you interrupt composition to go and read one. Concentrating harder demonstrably does not work. There is a pre-commit guard that will refuse your commit; do not treat a green as proof, it only catches three of the shapes.

Corollaries: trailing `Z` is mandatory; `git log` prints LOCAL time and is the usual source of a stamp wrong by exactly the offset; never rewrite a peer's stamp; **never repair your own fabricated stamp by inventing a better one** -- annotate it unverifiable and move on.

### Commit discipline, and the piece we learned three hours ago

- **`git commit --only <paths>`, NEVER `-A` or bare `git commit`.** Four sessions share this working tree; a bare commit sweeps a peer's staged index. This morning `--only` stopped a commit sweeping three files that were not cc's, one of them ic's inbox. Keep it.
- **AND: a move is two facts.** `--only` commits what you name, and a rename is an add plus a delete in separate index entries. cc named the additions; the deletions stayed staged; a commit titled "all native code moves to `native/rust/`" left **two complete copies of the Rust source tree** at HEAD and pushed both to both remotes -- five of the duplicated files divergent rather than identical, and root `Cargo.toml` still pointing a workspace at the stale copy, so a fresh clone would have built the wrong code. Everything local was green throughout.
- **Therefore: after any move, verify at HEAD (`git ls-tree`), never on disk.** cc's addition, which is better than mine: clone into a tempdir and build it. That is the only check that would have caught it.
- **The general form, and it is the most useful thing on this page: a green suite is evidence about the tree you HAVE and never about the tree you PUSHED.** My own verification an hour before -- lint, six gates, two ACs re-run -- was correct and could not have seen it, because every one of those reads the working tree and the working tree was right.
- **DO NOT ADD CLAUDE TO COMMITS. EVER.** No `Co-Authored-By`, no AI attribution. End the body with `(C) hello@matthewsinclair.com`.
- **Do not use `git stash` in this repo.**
- **Two remotes: `local` and `upstream`. Push both.** hv's instruction, verbatim: _"never use head -1 when examining what remotes exist. You'll miss them, otherwise."_

### Two traps specific to this repo that will bite you in week one

- **NEVER mutate `bin/**` or `tests/**` in place.** `~/.local/bin/intent` symlinks into this repo, so editing `bin/` changes the live tool mid-session; and the BATS suite reads the working tree (`no_absolute_home_paths.bats:37,100,103`), so editing `tests/` changes the thing measuring you. Use a sacrificial worktree.
- **This shell is zsh.** No word-splitting of unquoted parameters (a `for` loop over a string of args passes ONE argument); MULTIOS tees `cmd 2>&1 >/dev/null` to your terminal.
- **Read `$?` before anything else touches it.** `cmd | head; echo $?` reports the PAGER's exit. It manufactured two defects that did not exist.
- **Never `head` a list you are counting.** cc lost the eleventh of eleven rows that way and published the wrong count. A frequency-sorted list is worse: it puts the RARE value last, and the rare value is the one that decides the rule.

### Where the work is

**ST0056 is Intent v3.0.0** -- a full Rust rewrite. `intentsvcs` (model + SQLite store + file canon) plus an `intent` CLI that runs in-process or over GraphQL to `intentd`. Design canon in `intent/st/ST0056/design.md` (D01-D32); the contract is `intent/st/ST0056/acceptance.md`.

State right now, measured at 08:55Z:

```
ac:   31/94 satisfied -- BLOCKED
lint: ST0056 ok -- 94 AT row(s) conform
gate: 01 PASS 4/4 | 02 PASS 5/5 | 03 PASS 8/8
gate: 04 BLOCKED 5/6 (AC-04.6) | 05 PASS 4/4 | 06 BLOCKED 4/7 (AC-06.1, AC-06.3, AC-06.6)
```

**D01 is the load-bearing decision and you should read it before touching build config**: durable truth is committed schema-validated JSON canon; the SQLite DB is a rebuildable runtime index; `rm intent.db` is always safe and there are never DB migrations. Anything in your lane that treats the DB as precious is wrong by construction.

### Your backlog, as I see it -- argue with it

1. **Two apparatus guards are ruled and unwired, and they have been nobody's for a day because they are gate wiring.** `provenance_check.sh` (built, at `9e7a7be`) and `view_skew_check.sh` (ic built it this morning). Both belong in the pre-commit gate. This is the cleanest first job and it is unambiguously yours.
2. **Make cc's fresh-clone-and-build a standing post-move check** rather than a thing cc now remembers.
3. **`.github/workflows/rust.yml`** followed the `native/rust/` move this morning; worth a real CI run to confirm rather than assuming.
4. **`bin/int`** just grew `build daemon` and `build all` from hv. cc has a TODO to port Conflab's binary flavour switch and staleness report (`a671354`).
5. **`gen_inventory.sh` does not honour an `OUT` override**, which makes 26 `cmd-*.md` apparatus views unverifiable. One-line fix, ic's finding.

### How to reach us

Append to `<node>/inbox.dc.md` -- you are the sole writer of those four files, and each of us is the sole reader of ours. Entry format is `## (YYYY-MM-DD HH:MMZ)` then the body. Reply into the opposite-direction inbox with `Re: <their stamp>`. Your `wip.md` header block **looks like YAML and is not** -- it is line-oriented `key: value`, one line per key, and quotes inside a value are literal and never escaped. Writing `\"` puts a backslash in your board.

Set your header now: `name`, `role`, `session_id`, `status: active`, and a real `focus`. It currently says `name: dc` and `(new workstream -- not yet started)`.

Anything I have written here that you find to be wrong, tell me -- that is the job, and it runs in both directions.

-- vc
