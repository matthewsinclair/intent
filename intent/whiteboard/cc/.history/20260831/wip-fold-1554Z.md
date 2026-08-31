---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-31 15:38Z
status: active
focus: "2026-08-31 15:38Z. THE RESOLVER IS LANDED (e2fcf350) -- hv ladder above a pure promote, width arm out, two TUI kind-repairs, Unlanded gains Ambiguous AND Unresolvable, all four outcomes driven, 252 targets green. ONE DEVIATION AWAITING vc: land derives its resolver probe from present rather than taking a second injected closure. Also landed: the shell-page copy button (hv chore, 05660ad1), ic diagnosis of the NotHydratable remedy (5e7a1a5c), and a854d7c3 REVERTED (4e7f0d7f) because the tree was shipping the option hv declined by name. NEXT: scoped issue sync, close --note, 0086/0087 bodies."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0057/00]
---

# Control Claude (cc)

## DOING

**NOTHING IN FLIGHT.** Tree clean of me, no worktrees, no daemons of mine, every landing post-verified against its intended path set.

## TODO

- **THE RESOLVER -- RULED IN ALL THREE PARTS, AND IT IS THE NEXT BUILD.** Lives ABOVE `promote`, which is PURE and must stay so (`mcp.rs` calls it three times inside argument validation, before any work begins). Returns a VALUE, never prose: `Resolved(Address)` / `Ambiguous(Vec<Address>)` / `Unresolvable { searched }` -- because `nav` renders candidates as a list, MCP needs structured data, and a CLI verb prints a refusal. **The width arm comes out in the same change** (`model.rs`: untagged 4-digit tokens are silently assigned to ISSUE, so hv's ladder would never fire on the ids anyone types). **Scope, honestly: resolver + six conversions + two TUI kind-repairs + one injected closure in `nav` + `Unlanded::Ambiguous` + the width arm, as ONE change.**
- **SCOPED ISSUE SYNC** -- `facade.rs`'s `Some(_) => Vec::new()` means any scoped sync writes ZERO issue files. `Scope` is `All | Threads` with no issue form. **The refusal half is already built and I wrongly claimed it was not** -- `check_scope` has always refused an unmatched id; only `sync_overwrite` skips it, and that path refuses anyway.
- **`close --note`** -- ruled MY way: `status_reason` on `Issue`, ungated, matching `Thread` and `WorkPackage`. **`0063` unblocks with it.**
- **`0086`/`0087`** -- write the ARCHAEOLOGY RESULT as the body under the provenance marker (vc's ruling): the search result is a measurement I established, not a finding I invented, and an empty body cannot distinguish _nobody wrote one_ from _somebody looked and there was nothing_. **`0095`/`0096` are hv's disposition and vc carries them.**

## Watch-outs

**A FILTERED RUN REPORTS THE FILTER'S VERDICT.** I piped a gating test suite through `grep` and the harness said **exit 0** -- grep's status, not cargo's. Green notification, red suite, on the run meant to gate a commit. My own board already carried this class (`... | head; echo $?`). **And `cargo test` STOPS AT THE FIRST FAILING TARGET**, so one red lib test hides every integration test behind it -- `--no-fail-fast` is the house idiom for a reason. **Use `--manifest-path`: cwd drifts between calls and a bare `cargo test` then fails to find `Cargo.toml` and exits 101, which reads exactly like a test failure.**

**MEASURE THE SHIPPED BINARY INSTEAD OF INFERRING FROM CODE OR DATES.** `/opt/homebrew/Cellar/intent/3.0.0_1/bin/intent` is the v3.0.0 cut, read-only, drivable under a temp `HOME` with `INTENT_HOME` unset (dc). It corrected my own `0189`: I said the code asked for _another verb's_ argument name; at the tag it was **`edit`'s OWN name**, left behind by a rename and masked by a fallback added for `st edit`. **A defensive fallback added for one caller swallows a defect belonging to another** -- without it the rename would have failed loudly. dc published six false shipped-defect claims sourced the same way.

**A REMEDY THAT IS ACTIONABLE AND WRONG IS WORSE THAN A BARE ONE, AND THE CANONICAL FORM IS UNLEARNABLE WITHOUT AN EMIT.** `intent:///` -- scheme, EMPTY authority, path. Measured on this estate's own boards: the two-slash form TEN times against ONE canonical use, produced independently by four people who work on the tool daily. **Nothing emits it, so nobody could learn it.** Standards-correct (`file:///`) and learnable are different properties.

**A WORKED EXAMPLE IN A MESSAGE MEETS GUARDS A LITERAL CANNOT SATISFY.** `address_resolution_single_home` refuses the scheme spelled anywhere outside a comment; `no_pm_state_in_output` refuses an id from the reader's own project; and the two disagree about `ST0000` (source guard sanctions it, runtime guard rejects any `ST00`). **Render the example from `to_url()` with a placeholder** -- it satisfies both AND cannot go stale, and a stale worked example teaches the form the tool refuses.

**MY EXTRACT WRITES RENDER, SO A PEER'S STORE-ONLY EDIT LANDS ON DISK UNDER MY HAND.** The write SET is scoped to changed entities so nothing is clobbered, but the render is not. Announce it, leave it staged for them; **I do not commit a peer's bytes under my message.** And **a peer's STAGED-but-uncommitted file blocks my unrelated commit** -- guards read the staged index. The tell was a partition MOVING between two of my own commits, not an error message. Say so and wait; never `--no-verify`, never unstage a peer's path.

**A GENERATED VIEW MUST BE REGENERATED WHEN ITS SOURCE MOVES, AND ITS GENERATOR REFUSES PROSE THAT IS NOT A FORMATTER FIXED POINT.** `dispatch-table.md` from `.json`; the repo normalises emphasis to `_underscores_`, and asterisks are refused with a diff rather than silently rewritten.

**A FIXTURE THAT CANNOT EXHIBIT WHAT THE ARM CLAIMS REDS FOR THE WRONG REASON -- THE FRIENDLY HALF OF THE VACUITY CLASS.** Two in one file: a file the fixture does not carry, then a generated view the verb refuses. **And the remaining candidate was the DEFAULT, which would have passed for an arm that ignored the argument entirely.** Assert the argument by its EFFECT.

**A GUARD CAN NAME ITS OWN EXIT CONDITION, AND EXECUTING IT IS NOT WIDENING IT.** `declared_values_are_enforced` recorded `edit`'s kind slot as a known defect and said _it moves to `Enforced` when the resolver reads the kind_ -- then insisted **move the row, do not widen it** when that came true. **A guard instructing its successor.**

**BRING THE SHAPE, DO NOT PICK IT -- AND THE VALUE IS THE CLAUSE NEITHER OF US HAS.** vc ruled the endpoint discriminator on ground I did not have (the split is earned by the REMEDY, not the vocabulary); I replaced their provenance rule with the SPELLING rule after a census showed two sites breaking that theirs called safe. **Both directions happened in one day.**

**zsh: an unmatched glob (`--include=*.rs`) ABORTS the command; `$var` does NOT word-split; `||` after a pipeline binds to the LAST STAGE.** The tell is UNIFORMITY across a set that should have differed. **A REFUSAL CAN BE A RETRY** (`index.lock`, SQLite's second writer) -- wait on a live lock, never clear it. **NEVER start `intentd` under the real `$HOME`. NEVER invoke `intent fc`.** rustfmt reformats what you just wrote, and touching a source file makes `RealDaemon` refuse a now-stale sibling -- rebuild `-p intentd` after formatting.

## Decisions

- (2026-08-31) **THE THIRD ENDPOINT STATE IS A PROJECTION ABOVE `route()`, AND THE SPLIT IS EARNED BY THE REMEDY** (vc, on my six-case measurement). Two discriminators agreed on five cases and differed on the orphaned descriptor; both satisfied the row, so the criterion did not force it. Ruled onto **ABSENT** -- an orphan has no holder to investigate, so STALE would declare a remedy nobody can carry out. **Order is load-bearing: route first, lock second.**
- (2026-08-31) **A CONTRACT WITH A CONSUMER IS ASKED FOR, NEVER MINTED.** ic named `daemon status --format json`'s shape; I built that. **No `removable` field** -- the state IS the remedy, so a bool beside it is two homes for one fact and the UI would gate on the cached one.
- (2026-08-31) **THE ARCHAEOLOGY IS THE FILING COMMIT, NOT AN EARLIER REVISION.** Every empty-body canon file appears EXACTLY ONCE in history. The prose survived in the commit that filed the empty record -- its MESSAGE BODY for twelve, a BOARD DIFF for four. **Transcribed verbatim under a provenance marker; paraphrase would make me the author of four nodes' findings.**
- (2026-08-31) **THE EMPTY-BODY COUNT MOVED BECAUSE THE DEFECT IS STILL RECRUITING**, not because anyone miscounted. hv's 20 were filed 08-20..26; `0164` on 08-30, after the ruling, by the mechanism `0090` describes. **Both numbers are right about their day; reconcile neither.**
- (2026-08-31) **`close --note` IS A HIGHLANDER GAP, NOT A NEW FIELD** -- `Thread` and `WorkPackage` carry `status_reason`; `Issue` is the one of three without it. **`body` is affirmatively wrong**: its doc says _carried whole and never parsed_, so a status verb would smuggle machine text into the author's field.
- (2026-08-31) **A SEPARABLE COMMIT IS THE HONEST MOVE WHEN AUTHORITY IS CONTESTED.** hv asked me directly for the show-emit; vc reports hv ruled refusal-only. **The two proposals are not the same thing** -- vc's `--json` version was a canon schema change plus three projections that do not exist; mine is one line through the existing text renderer. Landed alone at `a854d7c3` so it reverts alone, and flagged rather than resolved silently.
- (2026-08-31) **`edit` TAKES A WHOLE ADDRESS, hv's RULING OVER vc's RECOMMENDATION**, with the cost stated before it was taken: the verb parses its own positionals. **The objection is in the code rather than answered** -- a second argument grammar inside one verb, at risk at the next argument added.
