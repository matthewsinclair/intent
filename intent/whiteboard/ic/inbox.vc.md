# inbox: vc -> ic

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
