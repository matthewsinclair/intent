---
node: dc
name: DevX Claude
role: worker
session_id: 482cf2fc-6b49-4a0d-8d76-38b3c981924c
heartbeat_at: 2026-08-15 10:56Z
status: paused
focus: "Localfolded and HOLDING at hv's instruction while hv and vc settle D01. hv has ruled the intentdb IS the durable SSOT -- that reverses D01 and lands two consequences in my lane. Day one work is landed on both remotes, CI green."
claims: []
---

# DevX Claude (dc)

## DOING

- **HOLDING, at hv's instruction.** hv and vc are settling D01 and will come back with "something definitive and canonical". Day one is folded; `.history/20260815/wip.md` carries the full record of what landed. Nothing of mine is in flight.

- **hv's D01 RULING, recorded before it is canon so it is not lost across a compact.** hv, verbatim in substance: **the intentdb is the durable single source of truth; everything else is a secondary artefact.** The DB can be _recreated_ from previously-extracted JSON, and a properly-formatted `.md` can be ingested **only such that it passes through the hard gate of the intentsvcs API** to become properly-formed DB items. hv was emphatic that this has been said more than once. **This reverses D01 as written** (`design.md:172`: durable = committed JSON, runtime = rebuildable SQLite, `rm intent.db` always safe, no DB migrations ever), and it settles the ambiguity `design.md:197` had recorded as open: the estate was built to the RUNTIME-SSOT reading, and hv means the DURABLE one. **Not mine to write into canon** -- vc is announcing, and D01 is in the hv-ratified set.

- **My landed work carries NO D01 assumption -- audited, not assumed.** `grep` over `bin/.devbin/cmd/**`, `.github/workflows/**` and `.gitignore` for `intent.db` / `sqlite` / `rebuildable` / `D01` returns nothing. Everything I built is about how the code builds and ships, which is orthogonal to where truth lives. **Nothing needs revisiting on my side when the ruling lands** -- but see the two consequences below, which are new work rather than rework.

## TODO

1. **D01's reversal lands two things in MY lane, both new work, neither actionable until the canon text exists.** `design.md:197` already named them:
   - **"No DB migrations ever" reverses.** Durable truth in SQLite means a versioned schema and real upgrade paths -- which is release mechanics, and mine. Every consumer's DB becomes something that must survive a version bump.
   - **`rm intent/.cache/` stops being always-safe.** Backup and restore become a requirement rather than a non-question, and `intent upgrade` acquires a data-safety obligation it does not have today.
2. **`*.db` ignore gap -- FOUND AND DELIBERATELY HELD.** The canonical path is protected (`intent/.cache/intent.db` via `.gitignore:82`, whose comment shows this class nearly bit once already), but a stray DB at `intent.db` or `native/rust/intent.db` is not ignored, in a PUBLIC repo. A `*.db` class rule closes it -- same shape as the `*.bak` fix -- and is reversible with one `!` line. **Held because no stray `.db` exists on disk (checked, only the canonical one), so there is no live exposure, and adding a rule touching DB durability while D01 is being reopened is the wrong moment.** Thirty seconds once the ruling lands.
3. **`.claude/settings.json` auto-mode brief: DRAFTED AND UNAPPLIED, hv to apply.** The harness classifier refused the write; I did not route around it and did not ask a peer to. Draft at `scratchpad/automode-env.json`, verified to change only `.autoMode.environment` and leave `permissions.defaultMode` untouched. Measured corrections: **repository visibility is PUBLIC, not "assume private"**; default branch `main`, not unknown; the project is bash-plus-Rust, not Elixir; plus the multi-session working-tree hazards, the `bin/intent` symlink, the unsafe stashes, and the fact that 60 whiteboard files are world-readable at push.
4. **`core.hooksPath` architecture question -- open for hv/cc, and now with no technical blocker.** The unwired-guard hole is VISIBLE (`int hooks`) and NOT closed: `.git/hooks/` is never tracked, so a fresh clone gets every guard and nothing invoking them. `core.hooksPath` at a tracked directory is the better architecture. I declined it on the grounds it would orphan `intent claude upgrade`'s output; **vc published that as a canon defect, cc refuted it, and vc withdrew it** -- the installer resolves through `canon_hooks_dir()` and follows a redirect. So what remains is purely that `lib/templates/` is cc's lane. My caution was right in principle and wrong in its stated reason.
5. **`bin/` boundary** stays open for hv (cc's split adopted as proposed).
6. **`use` / `cli --bin` flavour switch: OFF the list, not blocked.** hv ruled the v2-bash arm out of scope, so the axis is two-valued (brew v3 vs local dev v3) -- Conflab's shape exactly, a straight port with no design problem left -- and it is gated on a tap existing (WP-11). Carrying it as blocked would mean re-reading it at every pickup forever.

## Watch-outs

Facts about this estate, not reminders. Everything amounting to "remember to" is worthless here -- three nodes broke rules they had personally written, on the day they wrote them.

- **A control refuses; documentation reminds; only one is load-bearing.** Anything I can obey only by concentrating is an unfixed defect, not a discipline.
- **`--only` commits what you NAME, and a move is TWO facts.** The add and the delete are separate index entries; naming the new path commits the addition and leaves the deletion staged. It put two complete copies of the Rust tree at HEAD, on both remotes, five files divergent, with every working-tree check green throughout.
- **A green suite is evidence about the tree you HAVE, never the tree you PUSHED.** Verify a move at HEAD with `git ls-tree`, then clone fresh and build. `bin/int prepush` does this on push.
- **NEVER mutate `bin/**` or `tests/**` in place.** `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`; several sessions are live against it. Sacrificial worktree only. `bin/.devbin/**` and `native/**` are safe.
- **In a linked worktree `.git` is a FILE, not a directory.** Any tool computing `$ROOT/.git/<anything>` breaks there, silently and in the environment this project mandates. Ask `git rev-parse --git-path <x>`; never compute a path git already resolves.
- **A build cache can be stale in a way its own freshness check cannot see.** Every freshness check has a SCOPE. Tell: passes in isolation, fails in the suite -- a conclusion, not flakiness. `int cache` reports it; judge severity on the no-sibling count, not the total.
- **Anchor build tooling on `crates/`, not on a path prefix.** A prefix needle stops matching the moment the prefix changes, and then passes in silence. The tree moved twice in one morning.
- **Read `date -u +'%Y-%m-%d %H:%MZ'` in its own step, then write the line.** Never compose the surrounding text first. `git log` prints LOCAL time and is the usual source of a stamp wrong by exactly the offset.
- **This shell is zsh**: no word-splitting of unquoted parameters; MULTIOS tees `cmd 2>&1 >/dev/null` to the terminal.
- **Read `$?` before anything else touches it.** `cmd | head; echo $?` reports the PAGER's exit -- I read four exit codes wrong this way in one command. Never `head` a list you are counting.
- **The repository is PUBLIC.** 60 whiteboard files are tracked, so every board and inbox is world-readable at push, permanently.
- **Two remotes, `local` and `upstream`. Push both**, and never enumerate them through `head`.

## Decisions

- (2026-08-15) **A PEER CANNOT AUTHORISE WHAT A HARNESS REFUSED, AND A PEER PERFORMING IT ON YOUR BEHALF LAUNDERS THE REFUSAL.** The classifier refused my write to `~/.claude/settings.json`. I did not route around it and did not ask vc to do it instead; I drafted it, verified the blast radius and handed it to hv. vc confirmed they would have refused the alternative. Recorded because it is the kind of boundary that erodes quietly and by increments, each of which looks reasonable alone.
- (2026-08-15) **A RULE TRUE IN ITS OWN SCOPE IS THE EASIEST KIND TO OVER-APPLY, precisely because it keeps being true wherever you check it.** Four instances across four nodes in one morning. **Before carrying a rule to a new case, check the new case is in the set the rule was measured on.**
- (2026-08-15) **VISIBLE IS NOT CLOSED.** `int hooks` makes the unwired-guard hole measurable; it does not make the repository carry the wiring. Calling it closed would be the false green the check exists to expose. vc has taken this as a standard rather than a one-off.
- (2026-08-15) **ASK THE TOOL, DO NOT REIMPLEMENT ITS RULE.** My `int hooks` computed the hooks directory and shipped a false ABSENT in worktrees -- the exact failure its own comment claimed it prevented. A second implementation of somebody else's rule is a divergence waiting for the case you did not think of.
- (2026-08-15) **A PIN THAT DOES NOT BIND IS WORSE THAN NO PIN**, so `rust-toolchain.toml` is REFUSED rather than omitted: rustup is not installed here, so the file would be ignored locally while binding CI and reading as a project-wide guarantee. **If anyone later "fixes" this by adding the file, the fix is to install rustup first.**
- (2026-08-15) **A CANARY THAT DOES NOT ENTER THE BRANCH PROVES NOTHING, AND LOOKS LIKE A FINDING.** Assert the fixture reached the branch before reading its verdict. Corollary: canary a check in BOTH directions -- one that has only ever been red proves as little as one that has only ever been green.
- (2026-08-15) **A BROKEN NORMALISER FAILS AS A FALSE POSITIVE.** `sed 's/…\+/…/'` is a no-op on macOS (BSD basic regex has no `\+`), so my safety check compared unnormalised text and reported difference -- which reads exactly like a finding. Use `sed -E`, calibrate against a case it must collapse, and corroborate with `git diff --word-diff`, which needs no normaliser at all.
- (2026-08-15) **Re-measure at the moment of acting, not from the queued conclusion.** The staged set changed twice in five minutes and the dangerous file left it entirely.
- (2026-08-15) **Append to an inbox, never overwrite it.** A full-file write clobbered the scaffold's `dc -> <peer>` header on two of three intros.
- (2026-08-15) **A FILTER'S REAL-WORLD RELIEF IS BOUNDED BY HOW THE WORK BATCHES, NOT BY WHAT THE FILTER MATCHES** (vc's generalisation of my walk-back).
