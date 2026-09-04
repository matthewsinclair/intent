# dc -- 2026-09-04, the day's reasoning

Written at the localfold, 15:47Z, on hv's instruction, ahead of a compact. The live board keeps the RULES; this keeps the reasoning that produced them. Pre-fold verbatim beside this file at `wip-prefold-1547Z.md`, sha `83042d10`, 52,888 bytes, `cmp`-verified.

## What landed

- **`7a94b42e`** -- `skills sync` gains a preview. `Payload::sync` splits into `sync` / `sync_preview` over one private `run_sync`, with `plan` as the single home for "what would change" and `materialise` EXECUTING what it returns.
- **`9043f1767`** (ic's combined unfreeze, 22 files) -- my parity pair, the `0233` roster-guard fix, and B1 + C1.
- **`9fe2ee46`** -- `skills sync --dry-run`: dispatch-table row and renderer arm in ONE commit.
- **`6c4b864e`** carried my board update; **`9fe2ee46`** carried ic's `explore` hunk. Neither rewritten.

## The shape of the day, and it is one shape

**Four times, a written rule was present, recent, and authored or quoted by the person who then did not run it.** cc had a complete account of a guard's destructive-attribution class four lines above a surviving instance of it. I built the synthesised-index instrument in the morning to close exactly this class in a peer's reasoning, then aimed it at myself and it could not see my defect. ic broke `add` + `commit --only` ninety seconds after landing a commit about attribution care. And I did not run `git diff --stat` before committing a shared file, with that rule sitting verbatim in my own memory.

That is hv's thesis on their board -- reading the write-up of a class is not protection from it -- and today is the strongest evidence for it this estate has produced.

## The deadlock, and why every step of it was correct

An hour was spent on an option set built around a symptom. I told hv twice that the head of the chain was somewhere it was not: first cc's `browsed()` diff, then hv's own word. It was neither. `runner_roster_check.sh` was `MM` -- I staged it, edited it again for the `0233` fix, re-attached canon to the NEW bytes, and never re-staged -- so canon named bytes the index did not carry and `canon_commit_check` refused for exactly the reason it printed.

**Every refusal in the chain was TRUE.** The canon guard was right that a canon commit must carry its bytes; the roster guard was right that a tool and its row land together. Correct rules composing into a state where nobody can move is not a bug in any of them, which is why the remedy is a sentence rather than a tool: say which option you are TAKING.

**`MM` named the cause exactly and was read past about four times across two nodes**, including in runs where each of us was specifically diagnosing why commits were refused.

## The measurements that mattered

- **Four synthesised-index runs** settled the option set: A (cc's files) rc=1, B (my pair + canon) rc=0, C and D rc=1. The method turned "queued" into a return code for me and turned my recommendation into a refuted premise for cc within twenty minutes.
- **27 -> 4 test failures** after repairing the environment. 23 were the daemon fixture refusing because the debug `intentd` predated ic's `store.rs`. The remaining four were one cause in ic's `tui/` files.
- **A clean census of 78 shipped files** found exactly two with more than one `#[cfg(test)]`. My first census instrument was noisy and I re-ran it before trusting the answer.
- **The AC-00.3 population, derived rather than inherited**, found `claude subagents show` -- a fifth verb the row itself already named and that no working list carried. vc ruled A1 covers it, as an explicit extension rather than as always-implied.

## Two guards, one removed and one kept

In `payload.rs` I wrote two guards against a preview persisting the manifest. Mutation testing killed neither: each was sufficient alone. I collapsed to one door and proved the mutation dies. In `canon_mandated_verbs_check.sh` the phrase clause of the predicate excludes zero members of its population, so no mutation of it can fail either -- kept, for the recorded opposite case, with its obituary in the header.

## Attribution

Three adjacency errors in one day, all from different directions, none resolvable except by reading the artefact: vc and ic each attributed my staged tool to someone else from co-staging; I nearly attributed a `cargo fmt` diff from file history. cc's framing is the durable one -- a shared index destroys authorship and then offers adjacency in its place, which is worse than offering nothing.
