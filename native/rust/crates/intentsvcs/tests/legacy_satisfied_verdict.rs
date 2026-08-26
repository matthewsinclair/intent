//! **A RATIFIED CONTRACT WAS SILENTLY INVERTED BY AN EXACT MATCH, AND THE
//! MIGRATION EXITED 0.**
//!
//! `field` returns a `satisfied:` value whole -- parenthetical and all -- and
//! the arm that read it was `(Some("yes"), Some(e))`. So `satisfied: yes (hv
//! signed off 2026-06-22)` matched nothing and fell into a `_` catch-all that
//! DEFAULTED to unsatisfied.
//!
//! # Measured, not reasoned, on Courses ST0002 at `d18aca7^`
//!
//! Ten criteria. **2 of 2 carrying a bare `yes` survived; 8 of 8 carrying a
//! parenthetical were downgraded.** Perfect correlation, no exceptions. The
//! phrase `hv signed off` survived neither in canon nor in the regenerated
//! view, so a COMPLETED thread arrived recording eight of ten criteria
//! unsatisfied and nothing anywhere reported it.
//!
//! **THE CATCH-ALL WAS THE WHOLE DEFECT.** A classifier whose default bucket
//! absorbs the unrecognised case cannot report that it met one -- so the
//! failure is invisible by construction, and the louder the author was about
//! their evidence the more likely they were to trip it. The projects with the
//! richest sign-off records are the worst affected, which is the worst
//! correlation such a bug could have.
//!
//! # Why the fixture is COPIED and not composed
//!
//! `FIXTURE` below is that file byte for byte. A fixture an author writes can
//! only encode what the author already believes -- and this defect turns
//! entirely on a spelling nobody would think to invent, because everybody
//! writing a test for `satisfied:` writes `satisfied: yes`. That is precisely
//! the spelling that WORKED.

mod common;

use common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::legacy;
use intentsvcs::model::{AcKind, AcState};

/// Courses `intent/st/COMPLETED/ST0002/acceptance.md` at `d18aca7^`, verbatim.
const FIXTURE: &str = r#"---
verblock: "24 Jun 2026:v0.4: matts - All ACs satisfied (live on laksa.io, hv-confirmed); ST0002 complete"
st_id: ST0002
title: "Course2.0 content packaging for Laksa and e-book sales -- acceptance contract"
---

# ST0002 Course2.0 content packaging for Laksa and e-book sales -- Acceptance

> Canonical acceptance contract for ST0002. All ACs are non-test (doc / eyeball / gate), satisfied by named evidence. Complete (2026-06-24): design gate (AC-00.1) + contract/content (AC-01.1..03.1) + platform/integration (AC-04.1..09.1) all satisfied -- the courseware is live on laksa.io. Done is read from this map, never a hand-ticked box.

## Acceptance Criteria

### ST-level / design gate

- AC-00.1 (non-test) The design (`design.md`), the 10-WP breakdown with CC/LC ownership, and the `courseware.yaml` contract (`interface-contract.md`) are reviewed and ratified by hv -- evidence: hv reviewed + signed off 2026-06-21 -- satisfied: yes

### Contract + content (CC: WP-01..03)

- AC-01.1 (non-test) `courseware.yaml` schema is defined with worked examples for 001 and 002, agreed by cc + lc -- evidence: `interface-contract.md` v0.3 + lc ACK (`cc/inbox.lc.md` 2026-06-21 17:40) with redlines 1-3 folded in -- satisfied: yes (hv signed off 2026-06-22)
- AC-02.1 (non-test) Each course emits both artefact kinds in the contract layout: downloadable PDFs (from `bin/publish`) + browseable markdown -- evidence: validated 2026-06-22 -- all `courseware.yaml` artefact paths resolve (`{version}`->0.1.0) + browseable sources present (validator: ALL GOOD); 001 PDF + 13 002 docset PDFs exist at v0.1.0 -- satisfied: yes (hv signed off 2026-06-22)
- AC-03.1 (non-test) 001 + 002 each carry a valid `courseware.yaml` + the frontmatter/hierarchy the content type needs -- evidence: both `courseware.yaml` written + validated (ALL GOOD); 002 content re-tagged to lc's ratified lesson frontmatter (type/order/group/role) after lc built WP-04 to it; cc verified the tagging against lc's as-built `hierarchy.ex`/`config.ex` (group-ordering, course-root exclusion, H1 title fallback) 2026-06-22; contract v0.3 -- satisfied: yes (hv re-signed 2026-06-22 on cc's code-level verification; lc courtesy ack pending)

### Platform (LC, built in Laksa: WP-04..08)

- AC-04.1 (non-test) A `courseware` content type renders a course's browseable hierarchy from its `courseware.yaml` -- evidence: rendered course on a dev-local site -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-05.1 (non-test) Both artefact kinds are gated: browseable + downloads require an entitlement; `free_sample` is public -- evidence: paywall demo -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-06.1 (non-test) A courseware theme presents the course (nav, breadcrumb, lesson view, downloads) -- evidence: themed site -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-07.1 (non-test) A Stripe purchase grants an entitlement that unlocks the product's `grants`; **no charity** anywhere; rate limiting **reuses** Laksa's existing mechanism; comp/grant works **without Backpex** -- evidence: live on laksa.io (hv-confirmed 2026-06-24); NB lc deferred the full e2e Stripe round-trip test -- legs covered + live redirect verified (COMPLETED/ST0076/impl.md) -- satisfied: yes

### Integration (CC + LC: WP-09..10)

- AC-08.1 (non-test) Dev-x works: the course dirs symlinked into `Laksa/priv/laksa/sites` serve dev-local; the same `courseware.yaml` works under prod GitHub sync -- evidence: dev-local render + a prod sync -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)
- AC-09.1 (non-test) End-to-end on laksa.io: a course deployed, paywalled, browseable + PDF downloadable, a test purchase unlocks both -- evidence: live (or staging) walkthrough -- satisfied: yes (live on laksa.io, hv-confirmed 2026-06-24)

## Acceptance Tests

Content / platform thread; every AC is non-test (doc / eyeball / gate), satisfied by named evidence. No code ATs apply.

- Coverage: every AC carries inline evidence + satisfied state; non-test by construction. Gates: AC-00.1 (design sign-off), AC-01.1 (contract agreed), AC-09.1 (end-to-end).
"#;

fn v2_estate(fixture: &Fixture, acceptance: &str) {
  v2_estate_at(fixture, "Completed", acceptance)
}

fn v2_estate_at(fixture: &Fixture, status: &str, acceptance: &str) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
  fixture.write_file(
    "intent/st/ST0002/info.md",
    &format!("---\nverblock: \"24 Jun 2026:v0.4: matts - x\"\nintent_version: 2.19.0\nstatus: {status}\nslug: a-slug\ncreated: 20260624\ncompleted: 20260624\n---\n\n# ST0002: A thread\n\n## Objective\n\nShip it.\n"),
  );
  fixture.write_file("intent/st/ST0002/acceptance.md", acceptance);
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

/// **THE REGRESSION, DRIVEN ON THE REAL FILE: every non-test criterion that
/// claimed satisfaction arrives SATISFIED, parenthetical or not.**
#[test]
fn a_parenthetical_after_yes_does_not_downgrade_the_criterion() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);
  let thread = &scan.threads[0];

  let non_test: Vec<_> = thread
    .criteria
    .iter()
    .filter(|c| c.kind == AcKind::NonTest)
    .collect();
  assert_eq!(
    non_test.len(),
    10,
    "the fixture declares ten non-test criteria; a different count means the ROW parser moved and \
     this test is no longer about the verdict parser"
  );

  let downgraded: Vec<&str> = non_test
    .iter()
    .filter(|c| matches!(c.state, AcState::Unsatisfied))
    .map(|c| c.id.as_str())
    .collect();
  assert!(
    downgraded.is_empty(),
    "criteria that recorded `satisfied: yes` arrived UNSATISFIED -- a completed thread's ratified \
     contract, silently reversed: {downgraded:?}"
  );
}

/// **AND THE PARENTHETICAL IS CARRIED, because the verdict without its warrant
/// is the half that cannot be checked.**
#[test]
fn the_parenthetical_is_carried_as_evidence_rather_than_dropped() {
  let fixture = Fixture::new();
  v2_estate(&fixture, FIXTURE);
  let scan = scan(&fixture);

  let ac = scan.threads[0]
    .criteria
    .iter()
    .find(|c| c.id == "AC-01.1")
    .expect("AC-01.1 is in the fixture");

  let AcState::Satisfied { evidence } = &ac.state else {
    panic!(
      "AC-01.1 recorded `satisfied: yes (hv signed off 2026-06-22)`: {:?}",
      ac.state
    );
  };
  assert!(
    evidence.contains("hv signed off 2026-06-22"),
    "the sign-off naming a person and a date is the WARRANT for the claim, and dropping it while \
     keeping the verdict preserves an assertion nobody can check: {evidence}"
  );
  assert!(
    evidence.contains("interface-contract.md"),
    "and the row's own `evidence:` field is still there beside it: {evidence}"
  );
}

/// **AN UNRECOGNISED VERDICT REFUSES. IT DOES NOT DEFAULT.**
///
/// This is the arm that makes the fix a fix rather than one more spelling
/// added to a list. The old code could not report meeting a value it did not
/// know, because its default bucket swallowed it; the next unanticipated
/// spelling would have failed exactly as silently.
#[test]
fn a_verdict_that_is_neither_yes_nor_no_is_refused_rather_than_defaulted() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: somewhere -- satisfied: probably\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].criteria.is_empty(),
    "an unreadable verdict must not be carried as though it had been read"
  );
  // **The refusal is RECORDED, and it routes by the thread's state like every
  // other finding does.** This fixture is a CLOSED thread, so the finding
  // CARRIES rather than blocking -- `legacy_scope_carry.rs`'s whole subject.
  // Asserting on `residue` here was this test's own first draft and it failed
  // for the right reason: a silent drop and a carried finding are the same
  // `criteria.is_empty()`, and only the second is recoverable.
  assert!(
    scan
      .carried
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "a refused row on a closed thread must be CARRIED, not dropped in silence: {:?}",
    scan.carried
  );
}

/// The same unreadable verdict on a LIVE thread BLOCKS instead of carrying.
///
/// The pair matters more than either half: it shows the refusal is subject to
/// the estate's carry policy rather than being a special case that happens to
/// print something.
#[test]
fn the_same_unreadable_verdict_on_a_live_thread_blocks() {
  let fixture = Fixture::new();
  v2_estate_at(
    &fixture,
    "WIP",
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: somewhere -- satisfied: probably\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "an unreadable verdict in a LIVE thread must block the migration: {:?}",
    scan.residue
  );
}

/// An unclosed parenthetical is a truncation, and reading it as a bare `yes`
/// would silently discard whatever the truncation ate.
#[test]
fn an_unclosed_parenthetical_is_refused() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing -- evidence: somewhere -- satisfied: yes (hv signed off\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].criteria.is_empty(),
    "a truncated verdict must refuse rather than round to the nearest readable one"
  );
}

/// A row making NO claim is not a malformed one -- absent and unreadable are
/// different, and the refusal above must not swallow the ordinary case.
#[test]
fn a_row_with_no_satisfied_field_is_carried_unsatisfied() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing nobody has ruled on yet\n",
  );
  let scan = scan(&fixture);

  let ac = &scan.threads[0].criteria[0];
  assert!(
    matches!(ac.state, AcState::Unsatisfied),
    "a claim nobody made reads as unsatisfied: {:?}",
    ac.state
  );
  assert!(
    !scan
      .residue
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "and it is NOT a finding -- absent is not unreadable"
  );
}

/// **`n/a` IS KNOWN VOCABULARY AND IS CARRIED, NOT REFUSED.**
///
/// The arm that makes this fix a fix rather than a second defect. Measured
/// across the estate's `acceptance.md` AC rows: `yes` 1836, `yes (note)` 614,
/// `no (note)` 180, `no` 159, **`n/a` 20**. Those twenty fell into the old
/// catch-all and read unsatisfied; a refusal would DROP them from canon
/// entirely, **losing more than the bug being fixed ever did**.
///
/// It reads unsatisfied -- what it already did -- rather than being mapped to
/// `Descoped` or `Withdrawn`, both of which carry a reason and a destination
/// nobody wrote. Inventing one is the same offence as inventing evidence.
#[test]
fn n_a_is_known_vocabulary_and_is_not_refused() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-01.1 (non-test) A thing that does not apply -- satisfied: n/a\n",
  );
  let scan = scan(&fixture);

  assert_eq!(
    scan.threads[0].criteria.len(),
    1,
    "`n/a` is v2 vocabulary, not a malformed value -- refusing it drops a row that used to land"
  );
  assert!(
    matches!(scan.threads[0].criteria[0].state, AcState::Unsatisfied),
    "and it reads exactly as it did before the fix: {:?}",
    scan.threads[0].criteria[0].state
  );
  assert!(
    !scan
      .carried
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "and it raises no finding -- known is not unreadable"
  );
}

/// **THE THIRTEEN ROWS THE ESTATE ACTUALLY CARRIES, EXTRACTED FROM DISK BY
/// SCRIPT RATHER THAN COMPOSED.** Twelve are Lamplight's and one is Devbin's;
/// Conflab has none. Only the AC ID was rewritten, and it was rewritten on ALL
/// thirteen rather than on the two that collided, so there is no question of
/// which ones were touched -- everything from `(non-test)` onward is byte
/// verbatim, including the parenthetical under test.
///
/// **WHAT THEY HAVE IN COMMON IS A ` -- ` INSIDE THE `satisfied:`
/// PARENTHETICAL.** `field()` cut the value at the first ` -- ` regardless of
/// bracket depth, so the closing `)` fell outside the slice and
/// `satisfied_verdict` correctly refused an unclosed parenthetical. The refusal
/// was right; the truncation that manufactured it was not.
///
/// **BEFORE ic's REFUSAL LANDED, THESE INGESTED AS `yes` READ UNSATISFIED** --
/// a ratified sign-off silently inverted, which is the same defect `2aa82d17`
/// was committed to fix, arriving by a second route nobody had counted.
const LOSSY_ROWS: &str = r#"## Acceptance Criteria

- AC-01.1 (non-test) Equivalence evidence in `WP/11/lamplight-equivalence.md`: suite-scoped mix, whole-umbrella credo, framework-half lua, native/cli rust, the seal preflight green on the ported fence, the six check gates with `check deps` red on BOTH sides (the authored permanent red, carried as a matching red), version byte-probes plus `version check` against four real sidecars, and every deliberate difference owned -- no `test all` appearing, the bare-gate rc shifts, the check-all order, the cli option-scoping exposure probed against the Rust CLI's real flag surface -- evidence: `WP/11/lamplight-equivalence.md`: identical sets on all four sealed gates (2802/29667 credo, 49/49 lua, the same 20 rust targets, 43/43 scoped mix), the deps matching red naming the same two deps both sides, twelve byte-identical probes, zero colliding clap flags -- satisfied: yes (amended 2026-08-09 post-switch: the "no `test all` appearing" difference was hv-REVERSED the same day -- the aggregate now exists in hv's shape, rust lua credo mix with the ten-minute leg last, Lamplight `402614f3b`; the pack carries the addendum)
- AC-02.1 (non-test) The memories posture is typed + recorded (no playthrough identity -> no memories, by stated design) and `conversation.ex`'s self-contradicting "one Highlander home" moduledoc is corrected to the as-built truth -- evidence: the corrected moduledoc + a design line in WP/17/info.md -- satisfied: yes (memories design line in `WP/17/info.md` As-built [`pctx_id: nil`, `retrieve_memories/1` never called]; `conversation.ex` moduledoc now states the Cafe-layer truth + the shared ContextProvider/Prompt.Training spine -- vc read 2026-07-15)
- AC-03.1 (non-test) The austen s01e01 regression from the diagnosis resolves end-to-end: `lay the table` and `lay fork` no longer dead-end in a bare `Usage: lay` -- each yields a completed action, a single precise clarifying question, or an in-fiction goal response; `lay fork on table` no longer silently binds a junk position -- it surfaces an `illegal_value` clarification naming the legal positions instead of ticking a wrong bind (the most safety-relevant of the four diagnosed rows, since it doesn't even surface as a failure today); `lay the fork on the right` still binds and returns the correct "wrong side" consequence (no regression of correct behaviour). -- evidence: the hv-ruled done-bar (2026-07-02): a `.brief.md` playtest (eg `content/austen/dinner_with_austen/experiences/austen_s01e01/play/the_attentive_newcomer.brief.md`) run through the Wrighter Test infra (the ST0253 runner) against austen s01e01, the LLM playing the brief to a COHERENT conclusion on a `perf` session -- satisfied: yes (2026-07-05, run-3 `transcripts/austen-brief-playtest-run3.md`: completion `:terminal`, all 7 rubric objectives met, Breaks none, survey 5/5 enjoyment/clarity/immersion -- the WP-20 fix resolved the run-2 lay-`<piece>` wall; the player laid the setting and played the whole experience through)
- AC-04.1 (non-test) IC presentation half: the player surfaces hand the FULL tagged view to the render layer (the storyfield `to_player` boundary calls retired in favour of the client-side default filter off the tag) and a debug toggle renders `:internal` items -- the WP verification: an internal emission reaches the client, hidden by default, shown when toggled. -- evidence: the WEB player is built -- `PlayLive` retires the boot + per-turn `to_player` (holds the full view), `Player.visible_items/2` is the client-side default filter (player_facing; `show_internal?` reveals internal), driven by the existing role-gated `@debug?` toggle (`User.worldwright?/1`); index-keyed path media derives from the same `shown_transcript/2` projection + re-derives on toggle. Fenced by `player_test.exs` + `play_live_test.exs`. `:session_expired` humanised. story_channel/iOS deferred (gen3, hv-ruled). -- satisfied: yes (hv lived pass 2026-07-11: the debug-on Storyfield transcript shows internal emissions -- the `(plot_node_entered/exited)` observability markers + the raw command tokens -- revealed by the worldwright debug toggle; hidden-by-default proven by the `player_test.exs` fences)
- AC-05.1 (non-test) `combo_lock` is playable in the harness end-to-end: step wheels, request rounds, reach each of success, fail, give_up, and cancel; rounds feedback renders the verdict mask -- evidence: lived play (ic build-verify, then hv under AC-00.2) -- satisfied: yes (2026-07-24: ic build-verify at the wire drove all four outcomes + the refusal set over live Bandit HTTP, incl. the exhausted-fail metadata shape; hv then lived-played in the browser and signed off with screenshots -- combo_lock to fail-by-budget_exhausted at rounds 5 with the mask row rendered and the full metadata line, wheel_grid 4x4 to success in 4 rounds (the grid layout LIVED ahead of WP-05's formal proof), and the palette flip in BOTH directions (theme-zero recoloring from host --color-* with zero widget code); give_up + cancel reached at the wire and AT-covered)
- AC-06.1 (non-test) `wheel_grid` is playable in the harness (the dialtwiddler shape: 4x4, four values per cell) -- evidence: lived play (ic build-verify, then hv under AC-00.2) -- satisfied: yes (2026-07-24: hv lived-played wheel_grid 4x4 to SUCCESS in the browser during the WP-04 sign-off -- pre-WP-05 code; post-theming, ic wire-verified wheel_grid under the noir pack over live Bandit (grid mount payload, full 16-position mask round, theme riding the create response); then hv lived-played the WP-05 build in the browser and signed off: wheel_grid under contact_printer in TWIDDLE mode -- the 4x4 optic grid played by rotating tiles, orientation representing the value -- to success in 4 rounds with the mask reconciled all-match, screenshot on the record. hv's mid-build play of the same surface caught the stale-mask defect that `975b67a71` fixes; this sign-off is on the fixed build)
- AC-07.1 (non-test) as-built `apps/gwidget/README.md` for an external consumer: the model (space/oracle/codec/rounds/outcome), the mount API, the theming contract, the harness, and the give_up-vs-cancel outcome semantics (design DD-5; vc F4) -- no Lamplight/Intent/PM vocabulary -- evidence: the doc at HEAD, vc-checked at close -- satisfied: yes (2026-07-24: `apps/gwidget/README.md` rewritten as-built from the code -- the five-piece model table, a worked `Session` play with the real return shapes, the closed outcome set with the give_up-vs-cancel distinction spelled out as the thing a host must get right (F4's home), the registry, text play, the `mount` API + the thin-adapter rule, the three theme-slot families incl. twiddle, the contract artifact + its regeneration command, the harness, and an add-a-kind procedure. Written for someone using the library from outside: no steel-thread, work-package, acceptance, or Lamplight vocabulary anywhere in it)
- AC-08.1 (non-test) `table_lay` is playable in the harness end-to-end under both packs, all four outcomes reachable -- evidence: lived play (ic build-verify, then hv under AC-00.2) -- satisfied: yes (2026-07-24: ic wire-verified over live Bandit -- created under `regency`, laid the pieces wrong (per-piece mask `match/match/miss`), laid them right (success, rounds 2), and confirmed the zero pack renders the SAME placement game (`mode: "place"` rides from the kind, `theme: {}`); all four outcomes covered at AT-07.2's second arm with the intermediate budget spends asserted. hv then lived-played it in the browser under theme-pack ZERO and signed off: the placement board with neutral numeric labels ("1: 1", "2: 3", "3: 2"), the slot rail 0-5, all three mask cells `=`, `outcome: success -- rounds: 4`, screenshot on the record. The `regency`-dressed browser play rides hv's AC-00.2 sweep, which names table_lay under both packs)
- AC-09.1 (non-test) The plate above the handoff's wash boundary (42.9% from top at the desktop centre axis; 52.2% on phone) is unmodified -- no gradient stop reaches it -- and the effective ground behind any text run is at least ground-100 at 78%, met per text run by ST0347's reading ground (`.reading-ground`, ground-100/80 behind every prose-bearing emission). -- evidence: derived geometry table in impl.md (WP-05 section) + hv's live word 2026-08-11: "what I'm looking at is right... the rest of it so far is all LGTM" -- satisfied: yes _(Re-worded at close on hv's ruling: the original row said "top 60% unmodified" from the handoff's PROSE, while the treatment ships the handoff's VALUES verbatim -- the reference disagreed with itself, and hv ruled the rendered values right. The vignette is unchanged; the 78% clause is met by construction since the reading ground landed (ST0347 impl, live round).)_
- AC-10.1 (non-test) No crank/play run ever executes the engine or the player-LLM in the Wrighter BEAM. -- evidence: the Wrighter crank/play path calls only the remote client (no `Playtest.run`/`Handcrank` in-BEAM); code + hv lived-confirmed 2026-07-14 -- satisfied: yes (CODE HALF verified 2026-07-13: `apps/wrighter/lib` has ZERO in-BEAM engine-run calls -- the only `Handcrank` refs are `CrankParser`/`CrankResult` [tape-parse + result struct, not the runner]; the Wrighter crank/play driver is `TestRunClient.run` at `test/index_live.ex:1042` via `dispatch_remote_run`, test-backed by AT-04.1 "walk local, crank/play via remote client". LIVED half CONFIRMED 2026-07-14 -- in the hv Wrighter->Frontdesk remote session the Wrighter BEAM made no engine/player-LLM calls; both the engine and the player-LLM ran server-side.)
- AC-11.1 (non-test) A remote run persists no world and leaves no resident session on the server (the ephemeral session is discarded). -- evidence: server-side assertion (AT-03.10) + hv lived-confirmed 2026-07-14 -- satisfied: yes (server-side assertion LANDED + GREEN as AT-03.10 [hv full suite + a targeted run both passed]: after a completed streamed run the persisted-world census is unchanged AND the run's Control.Session id -- captured off run_started -- is absent from Session.list_sessions/0, proving the finish/2 -> Runner.halt ephemeral discard against the real deterministic Playtest.run lifecycle. LIVED check CONFIRMED 2026-07-14 -- the hv Wrighter->Frontdesk remote session left no orphan world/session on the running Frontdesk server.)
- AC-12.1 (non-test) Between an engine response and the next command a "player is deciding" indicator shows while the engine computes the next turn. -- evidence: hv lived-confirmed 2026-07-13 -- the "player is deciding" spinner showed between turns in the Frontdesk play@perf/high runs -- satisfied: yes (as-built: the deciding pause is the transport spinner -- the shared `SessionGlue` door sets `playtest_running: true` at run kickoff and renders that pending async state, cleared when the run completes; the streamed `:player_deciding` event is received as an explicit no-op, the spinner already covering the pause)
- AC-13.1 (non-test) A remote Wrighter crank displays each command's engine-response prose on the watch-it-play deck, through the shared `SessionGlue` `:engine_view` handler rendering `view.transcript`. -- evidence: lived Wrighter crank run -- satisfied: yes (hv-confirmed 2026-07-02 -- a lived Wrighter remote crank renders the full per-command transcript: command echo -> actor beat -> located prose -> outcome; the shared `SessionGlue` `{:engine_view, turn, view}` head + `TurnLog.replace_last/2` fill the paired turn's realised `view.transcript` in place. Backed by a deterministic red-to-green LiveView test.)
"#;

/// Each row's witness: a fragment of the parenthetical lying PAST the point the
/// old `field()` cut at. **Asserting the row merely parses is not enough** -- a
/// fix that made the bracket close while still dropping the tail would satisfy
/// "thirteen criteria arrived" and quietly lose what hv signed. The witness is
/// the half that cannot be faked by rounding to the nearest readable value.
const WITNESSES: &[(&str, &str)] = &[
  (
    "AC-01.1",
    r#"-- the aggregate now exists in hv's shape, rust lua credo mi"#,
  ),
  ("AC-02.1", r#"-- vc read 2026-07-15)"#),
  (
    "AC-03.1",
    r#"-- the WP-20 fix resolved the run-2 lay-`<piece>` wall; the"#,
  ),
  (
    "AC-04.1",
    r#"-- the `(plot_node_entered/exited)` observability markers +"#,
  ),
  (
    "AC-05.1",
    r#"-- combo_lock to fail-by-budget_exhausted at rounds 5 with t"#,
  ),
  (
    "AC-06.1",
    r#"-- pre-WP-05 code; post-theming, ic wire-verified wheel_grid"#,
  ),
  (
    "AC-07.1",
    r#"-- the five-piece model table, a worked `Session` play with"#,
  ),
  (
    "AC-08.1",
    r#"-- created under `regency`, laid the pieces wrong (per-piece"#,
  ),
  (
    "AC-10.1",
    r#"-- the only `Handcrank` refs are `CrankParser`/`CrankResult`"#,
  ),
  (
    "AC-11.1",
    r#"-- captured off run_started -- is absent from Session.list_s"#,
  ),
  (
    "AC-12.1",
    r#"-- the shared `SessionGlue` door sets `playtest_running: tru"#,
  ),
  (
    "AC-13.1",
    r#"-- a lived Wrighter remote crank renders the full per-comman"#,
  ),
];

/// **THE THIRTEEN REAL ROWS INGEST, AND THEY CARRY THEIR WHOLE PARENTHETICAL.**
#[test]
fn a_double_dash_inside_a_parenthetical_does_not_truncate_the_verdict() {
  let fixture = Fixture::new();
  v2_estate(&fixture, LOSSY_ROWS);
  let scan = scan(&fixture);
  let thread = &scan.threads[0];

  // TWELVE, NOT THIRTEEN, AND THE MISSING ONE IS NAMED RATHER THAN ROUNDED
  // AWAY: `AC-09.1` carries a SECOND and unrelated defect and has its own arm
  // below. A count of 13 here would hide it; a count of 12 with no explanation
  // would be worse.
  assert_eq!(
    thread.criteria.len(),
    12,
    "twelve of the thirteen real rows differ from canon only by a ` -- ` inside the parenthetical; \
     a shortfall means `field()` is still cutting inside a bracket and the row is being refused \
     for a truncation the parser invented"
  );

  let unsatisfied: Vec<&str> = thread
    .criteria
    .iter()
    .filter(|c| !matches!(c.state, AcState::Satisfied { .. }))
    .map(|c| c.id.as_str())
    .collect();
  assert!(
    unsatisfied.is_empty(),
    "every one of these rows records `satisfied: yes` on a ratified contract: {unsatisfied:?}"
  );
}

/// **AND THE TAIL SURVIVES.** Driven per row against a fragment that lies past
/// the old cut, so a fix that widens parsing without widening what is CARRIED
/// stays red here.
#[test]
fn the_parenthetical_past_the_old_cut_is_carried_not_dropped() {
  let fixture = Fixture::new();
  v2_estate(&fixture, LOSSY_ROWS);
  let scan = scan(&fixture);
  let thread = &scan.threads[0];

  let mut lost = Vec::new();
  for (id, witness) in WITNESSES {
    let c = thread
      .criteria
      .iter()
      .find(|c| c.id == *id)
      .unwrap_or_else(|| panic!("{id} did not ingest at all"));
    let AcState::Satisfied { evidence } = &c.state else {
      lost.push(format!("{id}: not satisfied"));
      continue;
    };
    if !evidence.contains(witness) {
      lost.push(format!("{id}: evidence lost the tail past the old cut"));
    }
  }
  assert!(
    lost.is_empty(),
    "the verdict survived but its warrant did not -- carrying `yes` while dropping what hv signed \
     preserves the claim and destroys the half that makes it checkable: {lost:?}"
  );
}

/// **THE THIRTEENTH ROW IS A DIFFERENT DEFECT AND IS DELIBERATELY LEFT
/// REFUSED.** Lamplight `ST0345` writes its note in markdown italics --
/// `satisfied: yes _(Re-worded at close on hv's ruling: ...)_` -- so the value
/// ends `)_` and `satisfied_verdict`'s `strip_suffix(')')` refuses it. Its
/// brackets balance perfectly and `field_end` returns the whole span, so the
/// truncation defect above is genuinely fixed for this row too.
///
/// **IT IS RECORDED AS A BOUNDARY RATHER THAN FIXED IN PASSING.** Widening the
/// verdict vocabulary is a separate ruling from widening where a field ENDS,
/// and the two must not ride in together: one is a parser reading its own
/// separator correctly, the other is a decision about which spellings of a
/// human verdict the tool will accept. **A test that quietly passed this row
/// would be the well-formed substitute this file already warns about.**
#[test]
fn a_note_wrapped_in_markdown_emphasis_is_still_refused() {
  let fixture = Fixture::new();
  v2_estate(
    &fixture,
    "## Acceptance Criteria\n\n- AC-09.1 (non-test) A thing -- evidence: somewhere -- satisfied: yes _(re-worded on hv's ruling -- the reference disagreed with itself)_\n",
  );
  let scan = scan(&fixture);

  assert!(
    scan.threads[0].criteria.is_empty(),
    "the emphasis wrapper is a SECOND defect with a different cause; if this row now ingests, \
     somebody widened the verdict vocabulary and this boundary needs re-ruling, not deleting"
  );
}
