# ST0087 design: Intent conforms to the dev, opt and brew contract

Written on 2026-10-25 by vc from Intent's measured column against the contract (2026-10-24, at e6b784790) and the contract as landed. What is marked measured was measured on rhadamanth; the rest is design to be measured before it is built.

## The contract and its one home

The contract has one home, devbin's `docs/dev-opt-brew.md`, landed at Devbin commit aeed31e4 and amended by devbin issues 0213 (the no-manifest brew keg, decision 58) and 0216. Its sections T1 to T10 are cited below by number and never restated; where this document says SAME, Intent follows the section as written, and where it says Intent's reading, the contract already admits it (the compiled-tool readings were Intent's requests, said SAME by all four estates on 2026-10-24 as draft v0.4). hv's rulings that frame it: one approach for Molt, Utilz, Intent and devbin; the user's links first on PATH (option B); this thread third in 3.2.7 after issue 0635 and the devbin v0.1.14 re-vendor (vc decision 71), both landed before this design was written.

## What Intent is, in the contract's terms

Intent is a compiled tool: two binaries, `intent` and `intentd`, plus a support tree the binaries read and exec. Its runtime set (T1) is the pair built from one commit plus the support paths git names: `lib/templates`, `intent/plugins/claude` and `intent/plugins/agents/plugin.json`. Measured: that list exists once today as `SUPPORT_PATHS` in `bin/.devbin/cmd/shared/support.lib`, and every release asset takes it from the commit with `git archive` (issue 0626). Under T1 the list lives in the binary as one function, and the packaging reads it from there; the exact read is the implementer's (a flag on `install` that prints the set is the recommendation), measured against `support.lib`'s callers before it is chosen.

The manifest (T3) is `<root>/.intent-install`. The link set (T5) is `~/.local/bin/intent` and `~/.local/bin/intentd`, moved as one: measured, the CLI finds the daemon only as `intentd` beside its own executable (`intentd_candidates` in intent-cli's `render.rs`) and refuses a stale sibling by its version, so a split set is a real failure.

## T1, the trees

- dev is the git checkout; its runtime is the release pair cargo builds into `native/rust/target/release` (`bin/devbin build all` builds in staging, verifies the pair as a set and promotes by rename) plus the checkout's support paths. The root is found by walking up from the symlink-resolved executable to the first directory holding `lib/templates` (`intentsvcs::install::home`); no environment variable names the tree (`INTENT_HOME` is inert, measured). SAME.
- opt is `intent install --prefix <dir>`, which does not exist today; its layout is the keg's `libexec` layout exactly. The configured place is `INTENT_INSTALL_PREFIX`, declared by the sleeve as Molt's is, read by `install` alone. No default. SAME.
- brew is `$(brew --prefix)/opt/intent/libexec`. Measured keg: `libexec/bin/{intent,intentd}`, `libexec/lib/templates`, `libexec/intent/plugins`, with Homebrew's `bin/intent` and `bin/intentd` as links into it. SAME, already true.
- The compiled-tool reading: `source_commit` is the commit the pair was BUILT from, which in dev can lag HEAD. In the contract.

## T2 and T3, publish and the manifest

`intent install` runs in dev only; from opt or brew it is refused by name. Dirty, for Intent, is: any support path differs from the built commit in git, or the pair's compiled-in stamp names a dirty build or a commit behind HEAD. Refused by name, no override; live project records (`intent/`) never count. An existing prefix is replaced only with `--force` and only over a manifest that says `tool: intent`; anything else is refused. `--managed-by brew` writes a manifest that refuses `install` and `use` writes into that tree. The manifest is T3's exact form; the binaries are checksummed as installed (the signed bytes on macOS), which is what the user has.

The brew tree is published differently and conforms: the formula does not run `intent install` in the keg because the keg has no checkout; it installs release assets (Developer ID signed and notarised binaries on macOS, static musl packages on Linux, the support tarball) and writes the manifest itself in T3's form with `managed_by: brew`. Today's kegs carry no manifest at all: by T3 as amended, a keg whose root resolves under the opt link answers brew with a doctor note naming `brew upgrade intent`, never a failure, until a release whose formula writes the manifest; in opt a prefix with no manifest is not Intent's and `use opt` refuses it.

## T4, tree identity and the version line

One function names the tree, in the contract's order, and every caller asks it; doctor fails on unknown. The line moves from today's `intent <version> (<sha40>) <provenance>` (measured on the dev pair and on the keg's binary) to `intent <version> (<tree>, <sha12>[+dirty]) at <root> <provenance>`. The sha is the built commit, read from the binary, never HEAD at run time; doctor says when HEAD has moved past the build (today's currency check). `+dirty` in dev when the support set differs from the built commit or the pair was built from a dirty tree; never in opt or brew. The trailing provenance word stays (`release` for a release-stamped build, `dev` for a dev build), read by the gate's self-provenance arm and the build's pair verification; readers of the first four fields ignore it. `at <root>` is absolute; for brew it is the opt link, which the tool prints when `current_exe` canonicalises through `Cellar/intent/<version>` to the same directory. When a dev tree's git cannot be read, the sha field says `unknown` and doctor fails saying why.

The readers that move with the line, all in WP-01's bank: `bin/int`; the devbin commands `macos`, `brew`, `local`, `linux`, `prepush`, `shared/brew.lib`, `shared/releasestamp.lib` and `build.d/release`; the bats files that match the line; the ST0056 parity tools; the formula's `test do`, a substring match on the version that survives any shape.

## T5, the switch

`intent use [dev|opt|brew]` as WP-03 states it; `use` ends in `intent daemon restart` because there is one daemon per user and the running one is the previous tree's binary. Hooks, skills and templates follow the switch at once in every session, since each hook resolves from the running binary's own install root.

What option B flips for Intent, measured:

1. `intent/docs/releasing.md`'s step `brew unlink intent && brew pin intent && intent daemon restart` goes. Today the keg is unlinked and pinned precisely to keep it off PATH while the dev pair is in use (`brew list --pinned` names intent; `/opt/homebrew/bin/intent` does not exist). Under B the keg stays linked, `brew upgrade intent` may move prod freely, and the user links decide; `bin/int brew smoke --reinstall` leaves the keg linked.
2. Nothing flips for `bin/int` or `bin/devbin`: both are reached by relative path from the checkout.
3. PATH on rhadamanth (measured): `brew shellenv` in `.zprofile` prepends Homebrew's bin after `path_helper`, and it appears twice, so Molt-matts puts `~/.local/bin` ahead of the FIRST `/opt/homebrew/bin` with a prepend. That is Molt's change, through molt-vc, and WP-05's proof depends on it.
4. The hazard the switch cannot see: a process started without the login PATH (the Claude Code desktop app, Chrome's native host, cron) whose PATH has `/opt/homebrew/bin` and not `~/.local/bin` resolves brew's `intent` for Claude Code's hooks (`intent claude hook ...` in `.claude/settings.json`) and for the MCP server (`"command": "intent"` in `.mcp.json`) while the maintainer is on dev. Today those processes find no `intent` there, because the keg is unlinked, and fail loudly; under B they silently run prod. `launchctl getenv PATH` is empty on this machine, so each app inherits its launcher's PATH. The contract names this (T5, what the shadow check cannot see); Intent's docs say it in the maintainer section, and whether launchd's PATH is set is Molt's decision.

## T6 and T7, upgrading and tests

Intent has no self-upgrade verb and `intent upgrade` is taken (it upgrades a project's `intent/` directory), which T6 allows: the procedure per tree is documented in `docs/install.md`'s maintainer section (WP-04). Intent ships no test or lint verb, so T7 holds by construction; doctor's manifest check in opt and brew is an arm beside the existing currency arm.

## T8, the formula

Rendered, never typed, by `bin/int brew formula` from the release's version and per-asset checksums; the verbs align to `render` and `publish`; the input shape stays assets, not a tag and a revision, because the formula installs assets (T2). `install` links both binaries into Homebrew's bin, `test do` runs the version line, the caveats name `intent bootstrap` (brew's post_install cannot reach the user's HOME) and `intent use`.

## T9, devbin's switch in this checkout

Intent vendors devbin, so this checkout is one of the proof sites for devbin's switch: `bin/devbin doctor` names the install the switch chose and why; `bin/devbin upgrade` reads it. Measured on 2026-10-24: `devbin` resolved only to Homebrew's, with no `~/.local/bin/devbin`; the re-vendor to 0.1.14 landed at 141dcf9ff on 2026-10-25.

## T10, the proof

WP-05 as stated. The Intent proof runs only once `install`, `use`, the manifest and the tree word exist, after the thread's banks land; the kovacs half after the release carrying WP-04 reaches the tap.

## Work packages

- WP-01 The tree function, the runtime set and the version line (T1, T4), M.
- WP-02 `intent install` and the manifest, with doctor's manifest arms (T2, T3), M.
- WP-03 `intent use` and the shadow check (T5), M.
- WP-04 The formula, the release tooling and the docs (T2 brew, T6, T8), S.
- WP-05 The proof (T9, T10), S.

Order: WP-01 first, since every other package reads the tree function and prints the line; WP-02 and WP-03 may proceed in parallel over it; WP-04 after both, since it is the cut-facing half; WP-05 last. Each package is one bank per node, judged whole at molt-vc's desk under the rule that any change under crates/*/tests runs the intentd suite and a surface change runs the workspace suite.

## Out of scope

- Molt-matts' PATH change and launchd's PATH (Molt's, through molt-vc).
- A self-upgrade verb for Intent (T6 allows none; a name would be hv's product decision).
- Changing the contract's text (devbin's; a correction goes to devbin-vc).
- WP-02 of ST0081 (Linux assets on each tag's release), which stands on its own.

## To measure before the design is final

- Every caller of `SUPPORT_PATHS` in `bin/.devbin/cmd/`, before the list moves into the binary, so the packaging reads one source.
- Every reader of the version line, by driving each named command against a pair that prints the new line in a worktree, before WP-01 banks.
- How `current_exe` resolves for the keg's binaries on macOS and Linux, so `at <root>` prints the opt link on both.
- Whether the gate's self-provenance arm and the pair verification read the provenance word by position or by name, so the trailing word survives the move.
