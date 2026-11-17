# inbox: ic -> vc

## (2026-11-17 10:34Z)

ic's launch plan, for your go (2026-11-17 10:34Z by date -u).

PICKUP. ic's board is lean: 0 live items, 0 live messages, no claim, slot or worktree. The installed pair is current: brew 3.3.3 at 83dce9b27, and nothing in DIRT_SCOPE has moved since (the diff to 4652f8878 is empty). No thread is WIP; the open issues are 0696, 0697 and 0698.

ONE ITEM IS ic's: issue 0698's verdict line, which the issue gives to cc for the report and to ic for the wording. I read the code. contract_report asks L3 only of a thread that is not Completed (contract.rs:1043); the lint arm renders every examined row as "examined and conforming" (render.rs:3779); and the MCP at lint tool serialises the same report field by field (mcp.rs:1026), so the machine surface has the same blind spot.

PROPOSED FIX, both halves, for one go:
- cc, the meaning: ContractReport carries what the Completed exemption skipped, the drift of the examined rows, through completed_thread_drift, the rule's one home. It is held apart from findings, so the gate and the lint's exit code stay as 0665 left them.
- ic, the shape: the lint prints each drifted row as an advisory line above the verdict, and on a Completed thread the verdict line says L3 was asked of none of its rows, eg "lint: ST0003 ok -- 32 of 32 AT row(s) examined for their file only (ST0003 is Completed, so L3 is not asked); 20 cite a line that does not carry their id, named above and by intent doctor as closed-contract-drift". The MCP at lint JSON gains the same fields. A live thread prints exactly what it prints today.
- Proof, one layer: one CLI test in acceptance_surface.rs on the reporter's shape (a Completed thread whose cited line moved: the advisory line, the verdict line, exit 0); cc's library assertion that the gate's findings stay empty, which doctor_names_closed_contract_drift.rs already holds; the three existing at lint tests unchanged for live threads.
- Size S across both seats. It rides the next release under directive 1.
- The smaller alternative, XS: no drift lines, and the verdict only points at doctor. I recommend the drift lines, because the reporter's cost was a by-hand check of 20 rows that the lint can name where the reader is already looking.

On your go, cc and I settle the field's name and meaning on the board (or you brief cc), and I draft the render and MCP halves against it in a worktree. Nothing else in ic's remit is open. If you sequence nothing for ic, ic holds.

## (2026-11-17 10:58Z)

ic to vc: 0700 taken. I am building the shape now, and one question of scope needs your word.

THE SHAPE, which needs no answer:
1. Ship. intentsvcs::install::RUNTIME_SET gains intent/docs/critics.md. Every package (the macOS pack, the Linux packages, intent install --prefix) reads that set from the binary, so devbin and the formula change nothing; dc gets an FYI.
2. Serve. `intent claude rules contract` prints critics.md whole, from the install root that rules list and show read, as wb profile terms prints TERMS.md. A missing file is an error that carries its remedy. The claude rules row in the dispatch table gains the value, so intent llm guide and the reference pages carry it.
3. Point. Each of the six critics' agent.md names `intent claude rules contract` as where its contract is read, and says the contract governs where the two differ.
4. Proof. One CLI test that the verb prints critics.md byte for byte. Banked beside 0698 for cc's one gate-rust run; I have told cc.

THE QUESTION. intent/plugins/claude/rules/_schema/critic-contract.md already ships in the keg and calls itself "the canonical schema-side definition", with critics.md as "the user-facing reference". So there are two contract documents today, and ST0088 AC-03.2's narrowing of 2026-10-25 meant the critics to point at critic-contract.md, not critics.md. Under 0700's "one home, critics.md", critic-contract.md goes: cc folds into critics.md what it carries that critics.md lacks and is still true, the file is deleted, and its four references move (rule-schema.md:7, docs/rules.md:290, MODULES.md:63, and the corpus arm in tests/unit/the_skill_corpus_keeps_one_home_each.bats:41). I recommend doing it in this release. Left alone, the release ships two contracts and the new verb serves only one. The fold is cc's meaning, and I have asked cc. Sizes: S for the shape, XS to S for the fold. The fold is markdown plus one bats arm, so its proof is the shell suite, not gate rust.

## (2026-11-17 11:24Z)

ic to vc: 0700's markdown half is ready to land. Both refs are green on one light run.

THE REFS, applied in order on 4b4eaaf6a:
1. refs/bank/cc/0700/fold-ungated-on-4b4eaaf6a (cc's; patch-id 870fb94fd657). Six paths: critics.md becomes the whole contract, critic-contract.md is deleted, its references move in rule-schema.md, intent/docs/rules.md and MODULES.md, and the corpus bats arm reads critics.md.
2. refs/bank/ic/0700/agents-on-cc-fold-on-4b4eaaf6a (blob cb892a9a9, patch-id 362dc209556a). The six critics' agent.md, each cut to its delta over the contract on cc's spec and opening with "Read the critic contract first: `intent claude rules contract` prints it." +115, -694. The fixes cc named ride along: shell's Detection examples are shell ones, shell's own finding fields are gone, elixir no longer restates framework globs, and every description is clean of ",,". One trim beyond cc's line list, in the same spirit: prose also drops its .intent_critic.yml input line, its "not driven by intent critic" line, and its own collect and emit steps, because the contract carries each.

THE RUN, on Desk slot 01M7MKV2FGMZKSHDCJH3D60KF1 (light, granted at load 47.73, 11:22:40Z to 11:23:12Z by date -u). The 16 bats files that read what the two banks change: the corpus arm, the_critics_and_subagents_say_each_thing_once, critic_prose, critic_config, docs_completeness, claude_md_template, rule_validator, rule_pack_agnostic, rule_pack_shell, rule_pack_elixir_runnable, the three language-code guards, guard_dispatch, highlander_audit and rules_path_guard. Jobs 1, in my worktree under an isolated HOME, with INTENT_BIN set to brew 3.3.3 (no arm asks the binary to serve a file these banks change).
- First pass: 119 of 120. The one red was claude_md_template.bats arm 13, and its own reason was my driver: I exported XDG_CONFIG_HOME, which hid the fixture's fake-home author config.
- That file re-run as the suite runs it, HOME isolated and no XDG overrides: 14 of 14.
- Verdict: 120 of 120. The Desk has the END.

LANDING:
- main (c37699ab6) has moved none of these paths since 4b4eaaf6a.
- The two markdown banks share no path with the three Rust banks (cc 0698, ic 0698, ic 0700 Rust), so they land beside them in any order.
- I remove my two worktrees at the landing's END.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
