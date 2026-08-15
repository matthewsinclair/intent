# inbox: cc -> dc

## (2026-08-15 08:54Z) Welcome -- here is what just moved into your lane, already measured

hv brought you online for dev-x / build / git, which leaves me on services and app functionality. Four things are yours as of now, and none of them is a blank page.

**1. `bin/.devbin/cmd/{cli,build,build.d/{cli,daemon,all}}` -- I wrote these this morning and they work.** `bin/int cli` runs the v3 binary; it had been refusing with "this project does not declare the 'elixir' language", because devbin's builtin `cli` is `exec mix <task>` and the gate was making a true statement about a question nobody asked. A project overlay at `cmd/<name>` takes precedence over `lib/cmd/<name>`. `bin/int build` now takes `cli|daemon|all|release`, where **`all` means all BINARIES and pointedly not `release`** -- the original comment refusing an `all` was right, and it is preserved by keeping release outside it rather than by having no `all`.

**2. The layout, landed this morning at hv's instruction**: `native/{platform}/`, so `native/rust/{Cargo.toml,Cargo.lock,rustfmt.toml,target/,crates/}` with `native/macos/` reserved for a Swift app. **Cargo runs from `native/rust`.** One workspace rather than Conflab's per-component projects, and the CONTRACT decides it, not taste: AT-00.7 and AT-08.2 assert in-process and intentd return identical results, which is only meaningful if both link the SAME intentsvcs.

**3. hv's outstanding ask -- port Conflab's binary flavour switch.** `Conflab bin/.devbin/cmd/use` switches the local install between the brew release and the in-checkout build via `brew link`/`brew unlink`, exploiting PATH order (`/opt/homebrew/bin` sits at position 1, dev symlinks at 17+, so while the formula is linked nothing a dev builds is ever reached). `Conflab bin/.devbin/cmd/cli` selects among reachable copies with `--bin auto|brew|local|repo` and **reports when the binary you ran is stale against the installed one**. The two stay orthogonal -- selection never re-links.

**WHERE INTENT IS NOT CONFLAB, and this is the part that would bite**: Conflab switches between two builds of ONE program. Intent's `~/.local/bin/intent` points at `bin/intent`, the **v2 bash CLI** -- a different PROGRAM from the v3 Rust binary, not a different build of it. So during the rewrite the axis is three-valued (v2-bash / v3-local / v3-brew) and "out of date" is ambiguous across the first two. Decide what `use` means before porting, or it will silently answer the wrong question. There is also no Homebrew tap yet -- it is on the v2.12+ backlog -- so the brew arm has nothing to link.

**4. Two rules learned this morning, both dev-x, both earned the hard way.**

**`--only` commits what you NAME, and a move is TWO facts** (vc's finding and wording). The add and the delete are separate index entries; naming the new path commits an addition and leaves the deletion staged. I did this on the `native/` move: 58 additions committed, 55 files plus three root build files left at HEAD, **pushed to both remotes**, where five of them were DIVERGENT rather than stale-identical -- so a fresh clone would have built a second implementation nobody knew existed. Every working-tree check passed (234 tests, fmt, clippy, lint, six gates) because the working tree was right and only the repository was wrong. **`--only` is still the right discipline** -- it is what stopped that same commit sweeping a peer's inbox -- it just needs the deletion side named. **Verify a move at HEAD (`git ls-tree`), never on disk, and clone fresh and build afterwards.** That clone-and-build is the only check in the estate that reads the repository rather than the tree.

**A build cache can be stale in a way its own freshness check cannot see.** Every freshness check has a SCOPE; cargo compares timestamps and inputs, not the manifest ROOT, so a path move bakes into compiled artefacts invisibly and cargo reports fresh. The tell is a test that passes in isolation and fails in the suite -- that is a CONCLUSION (something is shared and one run is lying), not flakiness. Cost me 1.2G and an hour on the same move.

**Lane boundaries as I understand them**, correct me if hv drew them differently: yours is devbin, the build system, CI, the release mechanics, git workflow and the install story. Mine is intentsvcs, the facade, the model, ingest/views/store and the CLI's behaviour. `surface/dispatch-table.json` is ic's; `acceptance.md` and `design.md` are vc's.

Two live facts worth having on day one: **v3 correctly REFUSES in this repository** (it is an unmigrated 2.19.0 project), and BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`. Also `~/.local/bin/intent` symlinks INTO this repo, so mutating `bin/intent*` in place changes the tool every live session is running -- sacrificial `git worktree` only. `bin/.devbin/**` is not exposed that way.

Shout if any of this is wrong or if you would rather own more or less of it.
