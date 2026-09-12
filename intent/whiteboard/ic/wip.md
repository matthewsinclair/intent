---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-12 11:31Z
status: active
focus: "WP-19 and WP-21 CLOSED; WP-20 pure half and AC-24.2 landed; the SQL door reads the index. EVERYTHING LEFT ON MY LANE WAITS ON cc's `--kind def`: AC-24.3's flags, AC-24.5's sweep (ruled, unexecuted, recorded below) and dc's structural door. Unblocked: AC-24.7's specification."
claims: []
---

# Interface Claude (ic)

## DOING

- **ST0069 WP-19.** The envelope (e3ef53aad), the AT rows (28de5113d... see canon), and the two index verbs (e839108d2, AT-19.6 at 6440e9348). **CLOSES ON AC-19.4 ALONE** -- the source tokeniser's measured recall, which needs cc's `src_sections` before it can measure anything. cc is building it now; I hold the fixture until the table exists.
- **ST0069 WP-20, my half: LANDED** (b96a73c2a). `index::symbols` over each grammar's own `tags.scm`, five features all OFF by default until hv rules the size. **`tree-sitter-bash` SHIPS NO TAGS QUERY**, so `shell` names zero symbols for 1,378,576 bytes; vc ruled `lang-bash` stays declared and off until upstream ships one, and nobody writes one here.
- **ST0069 WP-24, next.** AC-24.2's generated when-and-when-not tool descriptions can land now. AC-24.3's `--outline` / `--context` are FLAGS on the `search` row (vc: no new verbs) and wait on cc's symbols table. AC-24.5's canon sweep: file list to vc BEFORE touching one file, and it KEEPS `intent modules find` -- retiring it is hv's ruling and hv has not given it.
- **AC-24.5's SWEEP: RULED BY vc AND NOT YET EXECUTED. RELEASES WHEN cc's `intent search --kind def` ANSWERS ON MAIN** -- a canon naming a verb that says not-implemented is the defect the doc audit spent a day removing. The six files: `rules/agnostic/highlander/RULE.md:112` (its "grep for prior art" clause BECOMES the sentence), `skills/in-plan/SKILL.md:37`, `skills/in-review/SKILL.md:52`, `skills/in-standards/SKILL.md:17`, `lib/templates/llm/_AGENTS.md:252` and `_CLAUDE.md:21` (registry pointer keeps its registry-conditional shape and gains the index route BEFORE it), plus the two regenerated roots. One commit; the red arm is the audit's own grep for the old wording. **vc's sentence, verbatim, to place with each local lead-in kept:**

  > To check for prior art, ask the index first: `intent search --kind def <name>` answers whether a thing with that name already exists anywhere in the tree, and the answer carries the index's own freshness. When it says the index is not complete for the paths that matter, fall back to grep. Where the project keeps a registry, `intent modules find <name>` searches that as well.

  `intent modules find` STAYS wherever a registry exists -- its retirement is AC-20.6 and hv's, unruled. The exclusions are approved as stated: a critic's detection grep is a mechanism, not advice about finding code; AC-24.5's agent-guide half is met BY CONSTRUCTION and the AT-24.5 row says so in its note rather than skipping it.

- **THE NARROW DOOR I OWE dc:** `structural_for(paths, name) -> Structural::{Answer{symbols}, CannotAnswer{why}}`, the freshness rule behind the facade so dc's hook cannot reach it. Signature agreed, built when cc's symbols table lands.

## TODO

**WP-19, then WP-21, then WP-24's verbs, descriptions and skills** -- hv ruled ST0069 into 3.0.2 and vc set the order; WP-17 closed 2026-09-12. Each one: shape to vc BEFORE code, own worktree with its IN-TREE target dir, isolated HOME, red before green, the register row and any flag in one commit, and it lands on main when it closes on its criteria.

**The batched reference regeneration** -- ONE run, both halves keyed to v3.0.2, AFTER the tag exists and after the last search package. `docs/reference/**` from `gen_reference.sh` and `cut-surface.md` from `gen_cut_surface.sh`, which is keyed to the TAG. The signal is vc's. Collected so far: cc's batch 2 (printed output only), cc's organize preview lines, dc's single-line `claude upgrade --force` help change. On the pages' own contract -- per-verb help and arguments, no output lines, no per-verb exit tables -- none of it is a reason to regenerate; the one run re-keys everything anyway.

## Holds -- work I am NOT doing, each with the condition that releases it

1. **The palette `Home`/`End` flip** -- RELEASES WHEN hv sets post-3.0.1 work and names it.
2. **The unruled ic-lane defects** -- issue 0303 (the register's `as-observed` rows) and `subagents/.manifest/global-agents.json` (three bats tests assert it). RELEASES WHEN hv rules either in or out.

## Watch-outs -- one line each; the paragraphs and their worked cases are in `.history/`

- Shared tree: `git add <paths>` then `git commit --only <the same paths>`, in one call; never remove a peer's `index.lock` -- wait, then re-issue the SAME command.
- Build and test only in a private detached worktree under an isolated `HOME` (`CARGO_HOME`/`RUSTUP_HOME` at the real toolchain); read `cat ~/.intent/home` before removing any worktree.
- bats reads the corpus of the tree its binary lives in (`current_exe`): never point `INTENT_BIN` across trees.
- `docs/reference/**` and `surface/dispatch-table.md` are generated by tools under `intent/st/ST0056/parity/tools/`; editing one is an ST0056 attach, so ask vc for the window first.
- A count in a doc is DEAD (hv): name the thing, or the verb that reports the figure. A threshold that instructs the reader is not a count.
- Name a deletion to vc before it lands when vc has not ruled on that file.
- Land by patch, and diff the commit against the tested patch.
- A new dispatch-table ROW moves the status count, `legal_pairs` and the three populations; a new FLAG moves none. Insert as TEXT: `jq` reformats the file.
- To prove a write survives a daemon ingest, force one.
- The Bash tool is zsh: an unquoted `$var` does not word-split, a bare `====` aborts the command, and an unquoted `--include=*.md` aborts it too.
- Read the clock in the same command as the stamp.
- `intent fc` is the human's verb, even in a sandbox.
- Verify in a private CLONE or worktree with ITS OWN IN-TREE target dir: an out-of-tree `CARGO_TARGET_DIR` puts the binary where no `lib/templates/` sits above it, and the install-resolution tests go red for the harness.
- Diff a shared aggregator file (`tests/suite.rs`) before `git add`: `--only` takes whatever a peer left in it, and a `mod` registration must land in the same commit as the file it names.
- Nothing in this workspace may read a clock (`one_clock.rs`, empty exemption list): bound work in SQLite instructions, not seconds.
- `contention_wait_is_chosen.rs` counts the constant's NAME in `store.rs`, comments included -- a second mention is a second home.
- A diagnostic may say less than it would like; it may not change which error the verb answers (the lenient manifest read in `closing_notes`).
- A generated view (`surface/dispatch-table.md`) is regenerated by its tool and never hand-edited; the pre-commit skew guard blocks the commit otherwise.
- An AT row citing a file is a citation only if the FILE carries the row's id: the close gate checks the literal string.
- After editing `intentsvcs`, build `intentd` before the intent-cli suite: `refuse_a_stale_sibling_daemon` fails the whole daemon class in 0.00s, which reads as a broken box (23 failures, all cleared by one `cargo build -p intentd`).
- A private worktree carries a STAGED copy across a `checkout --detach`: `git checkout -- .` restores from the INDEX, not HEAD, so the old version outlives the rebase and the build fails against names that exist in no commit. `git reset --hard <new head>` is the move.
- The register's census moves with a row: `populations.{declared,shipped,probeable}`, `legal_pairs`'s `n` and `census_note`, the new-surface family count in `dispatch::tests`, and `flags: []` on a family row. Four guards, each refusing by name.
- An idempotent mutation withheld from MCP needs `recoverability_anomaly` on the row: the generator refuses the silence and says do not bend the label.
- **A text edit in a shared aggregator is located by its ROW, never by a pattern**: a replace on two common field lines put this row's anomaly on `st hydrate`. Caught by parsing the file back and asking which row carried it.
- **Never `git stash` in the shared checkout**: the stash list holds other sessions' entries back to v2.3.0, and a pop can apply a stranger's work. Mutate in place and restore.
- **Never write the register back through a serialiser**: `json.dumps` reformatted 49 lines nobody touched. Insert as text.
- `git commit --only <path>` commits the WORKING TREE version of that path, not the staged one -- so a peer's staged edit in a file you are writing lands in YOUR commit under YOUR message. Check `git status` for `MM` before adding.
- After editing intentsvcs, build intentd before the intent-cli suite (the staleness guard reds the whole daemon class in 0.00s).
