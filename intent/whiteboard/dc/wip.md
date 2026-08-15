---
node: dc
name: DevX Claude
role: worker
session_id: 482cf2fc-6b49-4a0d-8d76-38b3c981924c
heartbeat_at: 2026-08-15 09:02Z
status: active
focus: "Day one. Index hazard CLOSED -- the charter is at HEAD and what remains staged is formatting-only. Next is the two ruled-but-unwired pre-commit guards, which have been nobody's for a day."
claims: []
---

# DevX Claude (dc)

## DOING

- **Picked up 08:57Z.** Charter is hv's words via `whiteboard/README.md` (uncommitted): _dev-x and build environment, so that cc concentrates on functionality for the CLI / daemon_. cc, ic and vc all wrote intros within two minutes of each other (08:54Z / 08:55Z / 08:55Z); their three independent readings of my lane agree, which is worth more than any one of them.
- **Operating provisionally under vc's UNRATIFIED boundary**: I own the environment the code builds and ships in (`native/` layout + workspace files, `.github/workflows/`, `.gitignore`, devbin, hooks + pre-commit wiring, toolchain pinning, release mechanics); cc owns `native/rust/crates/**`. Disputed file test: _does changing it change what the tool DOES, or only how it gets built?_ **`bin/` is an open collision and I have not assumed it.**

## TODO -- in this order

1. ~~**THE INDEX**~~ **CLOSED 09:02Z without an unstage, and the way it closed is the point.** At 08:57Z the staged `intent/whiteboard/README.md` had no `dc` row and said "the roster is four", so a bare commit would have deleted this node from the roster. By 09:02Z vc had committed the README and the charter is at HEAD (`git show HEAD:...` carries the DevX Claude row). What remains staged -- `intent/llm/MODULES.md`, `dc/inbox.vc.md`, `vc/inbox.dc.md` -- is formatting-only (MODULES.md's entire HEAD-to-index delta is **one table separator dash run, four characters**, by `git diff --word-diff`), and two of the three are vc's in-flight commit content. **So the correct action was none.** Held the whole time because it spanned two peers' files; the hazard was resolved by its owner while I measured.
2. **Wire the two guards into pre-commit.** `provenance_check.sh` (`9e7a7be`) and `view_skew_check.sh` (`d470f62`). ic: one authorisation, not two -- same slot, same argument -- and it needs hv rather than ic's re-derivation. vc calls it the cleanest first job and unambiguously mine. Precedent is the clock guard: opt-in by directory presence, fires only on what the current commit touches, refuses rather than auto-corrects, prints the right value so the fix is a copy-paste. `view_skew_check.sh` is already `--changed <paths>...` so it costs nothing on a commit touching no generated view. **Add to the gate, never replace** -- it already runs prettier and the critic.
3. **Port Conflab's `use` + `cli --bin`** (hv's ask, handed over by cc). `Conflab bin/.devbin/cmd/{use,cli}` exist to port from. **Settle the semantics before porting**: Conflab switches two builds of ONE program; Intent's axis is three-valued (v2-bash / v3-local / v3-brew) because `~/.local/bin/intent` points at the v2 bash CLI, a different PROGRAM. Measured additionally: `which -a intent` returns THREE reachable copies (`~/.local/bin/intent`, `~/bin/intent`, `Intent/bin/intent`) and the first two are both symlinks to the third -- so the ambiguity is already live. There is no brew formula (`brew list intent` -> no such formula), so the brew arm has nothing to link until WP-11.
4. **Pin the Rust toolchain.** `native/rust/` has `rustfmt.toml` and **no `rust-toolchain.toml`**; local is Homebrew rustc 1.97.1, CI is `dtolnay/rust-toolchain@stable`. Both gates (`cargo fmt --check`, `clippy -D warnings`) are version-sensitive, so a stable release can turn CI red with zero code change, and local-green/CI-red has no mechanism today.
5. **CI swallows failures in my lane.** `tests.yml` runs integration tests as `bats ... || echo "Integration tests completed with status: $?"` -- the job cannot fail, and the `$?` there reads the wrong command anyway (the `$?`-after-a-pipe trap already on vc's board). The `run_tests.sh`-absent fallback is `bats "$f" || true`. `IN-AG-NO-SILENT-001` applied to the build environment.
6. **Make cc's fresh-clone-and-build a standing post-move check** (vc's #2) rather than something cc now has to remember. This is the only check in the estate that reads the repository rather than the working tree.
7. **Stale-but-fresh build state.** 2.6G at `native/rust/target/`; cc lost an hour and 1.2G to artefacts compiled against the old `CARGO_MANIFEST_DIR` that cargo's own fingerprint called fresh.

## Watch-outs

Facts about this estate, not reminders. Everything amounting to "remember to" is worthless here -- three nodes broke rules they had personally written, on the day they wrote them.

- **A control refuses; documentation reminds; only one is load-bearing.** Anything I can obey only by concentrating is an unfixed defect, not a discipline.
- **`--only` commits what you NAME, and a move is TWO facts.** The add and the delete are separate index entries; naming the new path commits the addition and leaves the deletion staged. It put two complete copies of the Rust tree at HEAD, on both remotes, five files divergent. Every working-tree check was green throughout and structurally could not have seen it.
- **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.** Verify a move at HEAD with `git ls-tree`, then clone fresh and build.
- **NEVER mutate `bin/**` or `tests/**` in place.** `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`; four sessions are live against it. Sacrificial worktree only. `bin/.devbin/**` and `native/**` are safe.
- **A build cache can be stale in a way its own freshness check cannot see.** Every freshness check has a SCOPE. Tell: passes in isolation, fails in the suite -- a conclusion, not flakiness.
- **Anchor build tooling on `crates/`, not on a path prefix.** A prefix needle stops matching the moment the prefix changes, and then passes in silence. The tree moved twice in one morning.
- **Read `date -u +'%Y-%m-%d %H:%MZ'` in its own step, then write the line.** Never compose the surrounding text first. `git log` prints LOCAL time and is the usual source of a stamp wrong by exactly the offset.
- **This shell is zsh**: no word-splitting of unquoted parameters; MULTIOS tees `cmd 2>&1 >/dev/null` to the terminal.
- **Read `$?` before anything else touches it**; never `head` a list you are counting.
- **v3 correctly REFUSES in this repository** (unmigrated 2.19.0). BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.
- **Two remotes, `local` and `upstream`. Push both**, and never enumerate them through `head`.

## Decisions

- (2026-08-15) **`\+` IS A BSD SED NO-OP, AND A BROKEN NORMALISER FAILS AS A FALSE POSITIVE.** `sed 's/[[:space:]]\+/ /g'` does nothing on macOS -- BSD basic regex has no `\+` quantifier -- so my safety check silently compared unnormalised text and returned SUBSTANTIVE on three files that were formatting-only. I was one step from reporting "do not unstage, there is real content here" on the strength of a no-op. **A normaliser that fails to normalise does not fail loudly; it reports difference, which reads exactly like a finding.** Use `sed -E` (or `perl -pe`) and calibrate the normaliser against a case it must collapse before trusting a verdict from it. `cat -A` is GNU-only here too; `cat -e`. Corroborate any formatting-only verdict with `git diff --word-diff`, which needs no normaliser at all.
- (2026-08-15) **Re-measure at the moment of acting, not from the queued conclusion.** The staged set changed twice in five minutes and the dangerous file left it entirely. Acting on the 08:57Z measurement at 09:02Z would have disrupted a peer's in-flight commit to fix a hazard that no longer existed.
- (2026-08-15) **Append to an inbox, never overwrite it.** A full-file write clobbered the scaffold's `dc -> <peer>` header with the inbound direction on two of three intros. The path already encodes the routing; the header exists so the file is self-describing when read alone, and I broke exactly that.
- (2026-08-15) **Check that index-only content is re-derivable before calling its destruction lossless.** The rule of thumb in circulation (worktree == HEAD) gets the answer right here and the reasoning backwards; where worktree == HEAD and the index differs, the index is the only copy of that content in existence. What makes this case safe is that the difference is formatting a formatter regenerates -- a fact that had to be measured.
