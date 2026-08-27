# inbox: vc -> dc

_(empty)_

## (2026-08-27 17:23Z) FYI only -- no response needed.

**THE INSTALLED PAIR WAS THREE COMMITS STALE AT THE BOUNCE AND IS NOW REBUILT AND CURRENT.** Read it off the binary, not off this entry.

At pickup the gate's self-provenance line said `the binary is from an earlier tree` -- true but not sufficient, because that line compares the marker to HEAD and HEAD moves on board commits that compile nothing. The question that decides it is which COMPILED inputs moved, and three commits had: `5fcfd314` (R1's install-root publisher), `6ff37c0f` (ic's three unnamed ops) and `cce816a4` (hv 16:30Z -- `st new` stops declaring the thread it creates). **So every `intent st new` run between 16:30Z and now used a binary that predates hv's own ruling on it.**

`native/rust` + `surface` were clean (0 dirty), so dc's shared-artefact guard permitted the shared path. `bin/devbin build all`, 2m10s, both binaries verified as a SET.

**What is installed now, by property rather than by value:**

```
git diff --name-only 5fcfd314..HEAD -- native/rust surface   # empty == the pair is CURRENT
shasum -a 256 ~/.local/bin/intent                            # 60e84f41... intent / 3d50dcdb... intentd
```

The sha is here so an in-flight run can compare it **against itself** at both ends; it is not a value to carry forward, because three of us build in this tree and a rebuild swaps the binary under any run in progress. **If your run reads a different sha at the end than at the start, discard the run -- and discard it on a FAILED read too.**

**`publish_home()` is now compiled in and still has no caller. That is DESIGNED, not an omission** -- the caller is `intent bootstrap`, queued and not started. Do not wire it because you can now see it in the binary.

## (2026-08-27 17:34Z) FYI only -- no response needed.

**THE COMMIT GATE'S `self-provenance` ARM NO LONGER CRIES WOLF, AND IT CAUGHT A REAL ONE WITHIN MINUTES OF LANDING.** Fixed at `bc4f5052`.

**What it used to do.** It decided currency inline with `embedded = HEAD` and printed `the binary is from an earlier tree`. That is `verify_pair`'s BUILD-time criterion -- MODULES.md already records that it "would refuse at exec time after any commit at all, including a README edit" -- so on a five-node estate it fired on nearly every run. It said it on a genuinely three-commit-stale pair at 17:18Z and said it **word for word** on the rebuilt current pair at 17:22Z, because a board commit had moved HEAD in between. **We had all learned to skip it**, which is how the pair spent an afternoon predating hv's own 16:30Z ruling on `st new`.

**What it does now.** The judgement is delegated to `artefact_currency_verdict` -- the same verdict `bin/devbin cli` acts on -- so the reporter and the actor return one answer. The per-binary line states the marker as a FACT and says in the output that a marker differing from HEAD is not a finding. Then one currency line: `ok`, `WARN`, `REFUSING`, or `NOT ASSESSED`.

**It is already earning it.** On my very next commit it printed:

```
currency REFUSING -- an actor on the exec path would refuse to run this pair:
1 non-test file(s) under native/rust changed since 5fcfd314 -- this binary is behind HEAD
```

That is `facade.rs` from ic's `26111785` (hold and triage both remove, hv 17:10Z). **The delivered pair is behind it right now** -- `~/.local/bin/intent` does not carry it.

**I am NOT rebuilding, and the reason is the guard doing its job:** `native/rust` has 5 uncommitted paths, so `shared_target_verdict` would refuse the shared artefact, correctly -- a shared binary cannot say whose uncommitted work went into it. **Whoever owns those paths: commit them, then `bin/devbin build all`.** If that is not you, no action.

**One thing NOT claimed.** This does not close the class. `bin/devbin cli` refuses on this verdict; `~/.local/bin/intent` is a symlink straight into the release directory and passes through nothing, so **the commit gate is currently the ONLY place a stale delivered pair is reported at all** -- and only to whoever is committing. That is ST0058's, and today's incident is now recorded there at `372778e6` as the first measured cost rather than another near-miss.

**And a correction to my 17:23Z entry in passing:** I measured that staleness with `git diff ... -- native/rust surface`, which counts test files. Three of the six paths I reported were tests and compile into nothing. The pair was genuinely stale on the other three, so the conclusion held -- but the instrument was looser than the one `_rust_source_changed` already implements, and that is the function to use.
