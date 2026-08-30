---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-30 21:38Z
status: active
focus: "Eighth fold, pre-compact on hv's call. The serving-match DESIGN is fully ruled (vc, five exchanges, nothing open) and the DATA prerequisites landed this hour: reach-vs-door corrections + info narrowed + 24 flag narrows + errata 2 in the evidence doc. The BOUNCE executes the build plan in TODO 0 -- it is complete enough to code from cold."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold board verbatim at `.history/20260830/wip-fold-2137Z.md` (eighth fold). Cold-session minimum: state, not story. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree.**

## DOING

**LANDING COMMIT (this hour): surface table corrections + evidence errata 2 + canon extract, all ruled by vc live.** doctor facade store->doctor (reach, not door); info NARROWED (composing arm, facade removed, mcp_narrowed note); 24 terminal-channel flag narrows via NEW flag key `exposed_on_mcp` (default TRUE, registered in key_classes.flag.declaration; doctor --fix narrow REVERTED -- its `retire` disposition already records it); llm-guide/surface row updated (face: vocabulary STRUCK -- empty population, each candidate empty for a different reason); st sync preview = explicit facade-gap entry; vc's 3.0.0 LIMIT recorded with discharge condition. Canon extract committed WHOLE: it also carries vc's two committed tool records the old extract lacked -- store, files and extract now agree (verified bytes 12330/10735 tracked+clean).

## TODO

0. **THE BOUNCE: BUILD THE SERVING MATCH. Every design question is CLOSED -- do not re-ask, just build:**
   - **dispatch.rs**: Flag gains `exposed_on_mcp: bool`, `#[serde(default = "flag_exposed_default")]` -> TRUE. Doc comment records cc's default-direction objection AND its answer (row-level key REFUSES on absence -- dispatch.rs ~:259/:1308, test `a_row_that_declares_neither_mcp_field_does_not_load`; the flag key trims parameters, never adds a tool; cc withdrew, vc confirmed after measuring).
   - **intentsvcs (cc cleared it, folded at d8fa3043)**: rootfiles.rs gains the validate check fn + report struct (missing/symlink/non-regular/regular + four-section FLOOR list moved verbatim with its not-derived-from-template comment); facade.rs gains ADDITIVE `agents_generate` (install::home + rootfiles::render("AGENTS.md", config, RenderContext{version: self ctx, todo_watermark: None})) and `agents_validate`. FacadeError: reuse a fitting arm else ADD a variant -- NEVER widen one (cc's rider). vc ruling (c): these are facade gaps closed by methods; the fn lands in intentsvcs NOT face-side.
   - **render.rs**: agents arms switch to the methods -- BYTE-IDENTICAL, baselines at scratchpad/baseline_{validate_full,validate_warn,validate_missing,generate}.txt, drive before+after (vc's condition; fixtures scratchpad/mcpfix/{full,bare}); doctor_json extracted from render_doctor_json (print stays); pub(crate): scope_of, thread_spec, status_filter, t_shirt, artefact_path (AC-05.3's one edit door -- call IT, count stays 1).
   - **mcp.rs**: schema() class fixes -- publish disposition=="keep" ONLY (the drop-skip matched ZERO flags: vocabulary is keep/retire/intrinsic; retire+intrinsic were being published -- intent_doctor advertised fix AND help), skip !flag.exposed_on_mcp, skip kind=="subcommand" args (todo's command). Header records: in-process-only LIMIT + discharge ("becomes false when the tier routes through dispatch(op), the 3.x destination"), open-per-call contract (render.rs's open(); open_exclusive for st sync -- the one exclusive-family tool).
   - **serve()**: `pub fn serve(f: &mut Facade, ctx: &FacadeContext, path: &str, args: &Value) -> Result<Value, ServeError>`; ServeError{UnknownTool, Args, Refused(FacadeError)}. SERVED roster + TWO-SIDED GATE: roster==tools() paths both directions; in-memory drive (tempdir + config.json per intentsvcs tests/common shape + Project::discover + open_in_memory) proving every roster path answers NOT-UnknownTool; negative control.
   - **Arm map (from today's arm reads, all verified)**: st new REFUSES start=true, remedy names intent_st_start (vc's compose ruling); st done/cancel: ListEdit::Suppressed on keep else AsDeclared, date->on; st sync REFUSES !write (remedy names read tools), write->sync_to_disk(sync::Scope::All); st edit via artefact_path + dispatch::arg_values("edit","file") vocabulary, returns {path} always; hydrate/dehydrate via address::promote; ac status/gate via scope_of -> f.gate(st, scope); at green/red/na -> at_set(AtStatus::X) from path; ac new kind test|non-test (absent==non-test); wp verbs thread_spec+scope_of (REQUIRE WP scope, mirror wp_target's refusal); wp rescope t_shirt(); issues: numeric id, IssueSeverity::parse+SPELLINGS refusal, --from narrowed so body only; todo + todo list -> todo_buckets (serde); todo update -> ok record; doctor -> Facade::doctor(f.project(), ctx, Some(f.store())) + shared doctor_json; agents -> the two new methods. OUTPUT: model types (Thread/WP/AcceptanceTest/Issue/TodoBuckets) serde direct; AcRow/Outcome{moved,already,notes[]}/contract::Verdict{pass|exempt|blocked,fiat,unsatisfied,line}/ContractReport{findings,rows} projected via public readers IN THE TIER -- no intentsvcs derives. Notes travel (Outcome doc names this caller); never silent.
   - Then: rustfmt --edition 2024 MY files only; cargo build -p intent-cli; test; byte-compare drives; commit (check peers' hunks first -- .github/workflows/tests.yml is a PEER'S edit, never stage it).
1. **Then the 7 `claude subagents` narrowing rows** (hv-ruled via dc: reshape to one --kind lifecycle; wiring the old shape would contradict the ruled surface). dc builds the CLI half behind the reshape; I author the new rows when it lands.
2. **Then the rmcp server arm** (`intent mcp`, stdio, Tier C dep design.md:91/166) -- ANNOUNCE the Cargo.lock move to cc+dc BEFORE landing.
3. **AC-09.6 satisfy still waits on hv's class decisions** (narrows/gaps/unwired), now MINUS what today settled (info, agents pair, llm guide, surface, subagents). vc carries the list.
4. Standing queue: 0142 structural half (dc's census G/H is evidence, mine); TUI remainder (status picker, EMBED pty, intent edit wiring); AC-17.1 browser realiser (cc's /op stable, token 0600, Shutdown refused over HTTP; web face is DEV-TREE only until devbin build all); AC-17.10; WP-16; ST0064 parked.

## Watch-outs -- mechanisms only

1. **jq `//` SWALLOWS false**: `.key // "unset"` reports false as unset -- use `has()` when the value is boolean. Cost a wrong read of my own edit today.
2. **REACH IS NOT A DOOR**: a field populated by measuring what code touches, read as declaring what serves it, is a plausible home for a claim it does not carry. Four of 55 facade values were reach (doctor, info, agents x2). The serving match + gate makes the discriminator permanent.
3. **A PREDICATE THAT CANNOT MATCH ITS SUBJECT RETURNS THE NUMBER THAT MEANS SUCCESS**: schema()'s disposition=="drop" skip vs a vocabulary with no "drop" -- matched zero flags forever, published retire+intrinsic flags to agents. Sweep tests must positive-control the FILTER, not just the output.
4. **HIDE-CLASSIFY DISCIPLINE (vc)**: every flag hidden from MCP is first terminal-channel, honest-refusal, or DEFECT -- only the first two hide without a filing; hiding a defect removes it from the one surface where somebody was looking.
5. **A HAND-ROLLED SPAN FINDER FAILS ON render.rs; syn IS THE FLOOR** (armscan, five controls, five caught defects -- in the evidence attachment).
6. **HUNK-SCOPED STAGING + PINNED PRIVATE INDEX + HEAD-PIN + ambient reset**: unchanged, see fold archive. Peers' current dirt: tests.yml (not mine), their boards. Both peers FOLDED with nothing uncommitted in my paths -- but re-measure at pickup.
7. **STALE BINARY**: build in the same breath as the first drive. Baselines were captured against a freshly built HEAD binary.
8. **ABSOLUTE PATHS**: the shell cwd resets between calls; relative sed/ls silently misses.
9. **ANNOUNCE ANY DISK->STORE SYNC FIRST; `intent st attach` is the surgical verb.** Done this hour for the evidence doc; extract committed WHOLE because store/files/extract now agree -- a hunk-filtered extract would have RE-minted divergence.
10. **Scratchpad worktree `scratchpad/wt-tui` still registered + stale** -- `git worktree remove` when convenient.

## Decisions

- **2026-08-30 vc ruling (c): agents generate/validate are FACADE GAPS closed by adding methods in intentsvcs** -- rootfiles was in the services crate all along; validating generated content is a services concern.
- **2026-08-30 vc: the face: vocabulary is STRUCK, not pending** -- empty population once measured; a pending spelling is an invitation.
- **2026-08-30 vc LIMIT: 3.0.0 MCP serves in-process only; AC-08.2's dual-path does NOT extend to the MCP face.** Discharge: routing through dispatch(op), the 3.x destination. In the evidence doc AND (on the bounce) mcp.rs's header.
- **2026-08-30 vc + cc, after measurement: row-level exposed_on_mcp REFUSES on absence (stronger than any default); flag-level defaults TRUE** -- the flag key trims parameters, never adds a tool. cc's objection + answer go in the field's doc.
- **2026-08-30 hv (standing, via dc): claude subagents rows NARROW; the --kind lifecycle is the wiring path.**
- **2026-08-30 ic (at c5d66741): Esc never quits; quit is an act.**
