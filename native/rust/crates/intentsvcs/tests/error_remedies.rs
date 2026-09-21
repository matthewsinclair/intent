//! AT-04.4 / AC-04.4: every facade error is typed and renders a remedy with
//! its full cause chain -- no same-text-for-different-causes collapses.
//!
//! **The last clause is what makes this AC non-vacuous, and it is easy to miss.**
//! A test that merely asserts "an error has a remedy" passes on an
//! implementation where every remedy reads "check your input" -- which is
//! exactly the v2 behaviour this replaces. So the assertions here are
//! PAIRWISE: two distinct causes must render distinguishably, checked across
//! the whole variant set rather than sampled.
//!
//! **"The whole variant set" was a claim in this comment and nothing made it
//! true.** `provoked_errors` is hand-built, so the sentence above described an
//! intention rather than a mechanism. Measured against this file at
//! `c1e630cf`: **SIX reachable variants had no assertion here at all** --
//! `NotSatisfied`, `OffScope`, `WrongOffScopeState`, `IllegalTransition`,
//! `ReasonRequired` and `DescopeTargetMissing`. The claim is now carried by
//! `every_variant_is_provoked_or_declared_elsewhere`, and the exemptions are
//! declared rather than implied.
//!
//! ST0069 AT-14.3 and AT-14.5 cite this file: every over-bound whiteboard write
//! is provoked and refused by name, and a write whose acting node is unregistered
//! or unnamed is refused rather than guessed.

use crate::common::{Fixture, facade_ctx, sample_thread, v2_estate, v2_thread};
use intentsvcs::facade::{Facade, FacadeError, ListEdit};
use intentsvcs::model::{AcKind, AcceptanceTest, AtKind, AtStatus};
use intentsvcs::organize::Mode;
use intentsvcs::remedy::Remedy;

/// Provoke each error through the real facade, so the set under test is what
/// operators can actually reach -- not a hand-built list that could drift from
/// the code that raises them.
fn provoked_errors() -> Vec<(&'static str, FacadeError)> {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  // Seeded rather than pushed-into-empty: the ORDER matters, because each
  // error after the first is provoked against a facade the previous calls have
  // already mutated.
  let mut out: Vec<(&'static str, FacadeError)> = vec![(
    "unknown thread",
    facade.st_show("ST9999").expect_err("no such thread"),
  )];
  // Reachable by a bad CALL since `Facade::schema` landed -- before it, the
  // variant existed and nothing on the facade raised it, so its exemption
  // below cited a CLI arm that composed its own refusal string instead.
  // **THE SQL DOOR'S GATE REFUSES BEFORE IT OPENS ANYTHING**, which is what
  // makes these four provokable here: they are decisions about TEXT, so they
  // do not need a store on disk the way the four cited below do.
  for (label, statement, limit) in [
    ("a batch at the sql door", "select 1; select 2", None),
    ("nothing at the sql door", "  -- just a comment\n", None),
    (
      "an unterminated literal at the sql door",
      "select 'abc",
      None,
    ),
    ("a limit above the ceiling", "select 1", Some(usize::MAX)),
  ] {
    out.push((
      label,
      facade
        .search_sql(statement, limit)
        .expect_err("the gate refuses this before it reaches a store"),
    ));
  }
  out.push((
    "unknown schema face",
    facade
      .schema(Some("not-a-face"))
      .expect_err("a face the types do not generate is refused by name"),
  ));
  // **A STRUCTURAL DOOR ASKED FOR A TIER IT CANNOT ANSWER** (ST0076 WP-04):
  // the refusal needs no indexed symbol, so this empty fixture reaches it.
  out.push((
    "a structural door asked only the lexical tier",
    facade
      .context(
        "anything",
        &intentsvcs::search::SearchQuery {
          tiers: vec![intentsvcs::search::Tier::Lexical],
          ..Default::default()
        },
      )
      .expect_err("the structural tier alone answers a context search"),
  ));
  // **A SEARCH BY TARGET OF AN INDEX WHERE NOTHING IS RESOLVED** (ST0076
  // WP-07): this fixture has never stored a resolution, which is the state the
  // refusal exists for.
  out.push((
    "a search by target where no language has stored a resolution",
    facade
      .context(
        "anything",
        &intentsvcs::search::SearchQuery {
          target: Some("crate::anything()".to_string()),
          ..Default::default()
        },
      )
      .expect_err("nothing resolved can answer a target"),
  ));
  // **AN UNREGISTERED NODE, AND THE DISCRIMINATING CASE IS THE EMPTY ROSTER.**
  // This fixture has never run `wb register`, so the refusal has nothing to
  // list -- which is precisely the state that would otherwise be answered with
  // a board carrying no items and no messages, meaning the opposite thing. The
  // refusal firing here is what says the reader refuses on REGISTRATION rather
  // than on having a roster to compare against.
  out.push((
    "a board nobody has registered",
    facade
      .board("zz")
      .expect_err("no roster is registered in this fixture, so no moniker resolves"),
  ));
  // **THE BOUNDS NEED A ROSTER, so one is registered here** -- `wb_ask` refuses
  // an unregistered node before it ever looks at a body, so a bound refusal
  // provoked without a roster would be the roster refusal wearing its name.
  for (node, name, role) in [
    ("cc", "Control Claude", "control"),
    ("hv", "Hypervisor", "hypervisor"),
  ] {
    facade
      .wb_register(node, name, role)
      .expect("register the node");
  }
  // A node read off its hand-authored header is registered and not migrated.
  let dc = fx.root().join("intent/whiteboard/dc");
  std::fs::create_dir_all(&dc).expect("node dir");
  std::fs::write(
    dc.join("wip.md"),
    "---\nnode: dc\nname: DevX Claude\nrole: worker\nstatus: active\n---\n",
  )
  .expect("a hand-authored header");
  out.push((
    "a registration whose arguments disagree with the board's own header",
    facade
      .wb_register("dc", "devbin-dc", "dc")
      .expect_err("the header on disk says who dc is"),
  ));
  // A header lacking a field refuses the whole roster, and is removed again so
  // the roster registered next is `dc` alone.
  let zz = fx.root().join("intent/whiteboard/zz");
  std::fs::create_dir_all(&zz).expect("node dir");
  std::fs::write(
    zz.join("wip.md"),
    "---\nnode: zz\nname: Roleless Claude\nstatus: active\n---\n",
  )
  .expect("a header without a role");
  out.push((
    "a roster registration that meets a board header lacking a field",
    facade
      .register_roster()
      .expect_err("a header without a role cannot be registered"),
  ));
  std::fs::remove_dir_all(&zz).expect("remove the roleless board");
  facade
    .register_roster()
    .expect("register dc from its header");
  out.push((
    "a board write on a node whose board is still its markdown",
    facade
      .wb_touch("dc")
      .expect_err("an unmigrated node's board is its markdown, and a render would erase it"),
  ));
  // A stranger's inbox refuses the carry before it writes, and is removed again
  // so `dc`'s directory is as it was for every call after this one.
  std::fs::write(
    dc.join("inbox.zz.md"),
    "# inbox: zz -> dc\n\n## (2026-09-14 12:00Z)\n\na message from nowhere\n",
  )
  .expect("a stranger's inbox");
  out.push((
    "a migration that meets an inbox from an unregistered sender",
    facade
      .wb_migrate("dc", false)
      .expect_err("a message row names its sender, so a stranger's inbox refuses the carry"),
  ));
  std::fs::remove_file(dc.join("inbox.zz.md")).expect("remove the stranger's inbox");
  // **ONE BYTE OVER, which is the criterion's own discriminating case.** A
  // refusal provoked with a wildly oversized body passes whether the
  // comparison is `>` or `>=` and whether the bound is the configured one or
  // any smaller number.
  let bound = fx.facade().project().config().whiteboard.body_bytes;
  out.push((
    "a body one byte over the bound",
    facade
      .wb_ask("cc", "hv", &"x".repeat(bound + 1), None, false)
      .expect_err("the bound is enforced by refusal, never by truncation"),
  ));
  // Fill one inbox to its bound, then ask once more.
  let inbox_bound = fx.facade().project().config().whiteboard.live_messages;
  for i in 0..inbox_bound {
    facade
      .wb_ask("cc", "hv", &format!("message {i}"), None, false)
      .expect("inside the bound");
  }
  // **THE ITEM BOUND IS PER KIND, so the provocation fills ONE kind.** Filling a
  // board's worth of mixed kinds would pass on a per-board bound too, and the
  // per-kind reading is the thing that keeps a long watch-out list from refusing
  // a node's next piece of work.
  let item_bound = fx.facade().project().config().whiteboard.live_items;
  for i in 0..item_bound {
    facade
      .wb_decide("cc", &format!("decision {i}"))
      .expect("inside the bound");
  }
  out.push((
    "one kind at its item bound",
    facade
      .wb_decide("cc", "one too many")
      .expect_err("one past the configured per-kind bound is refused"),
  ));
  // **THE BOARD IT REFUSES IS ONE WITH ROWS, AND `cc` HAS THEM BY NOW.** The
  // migration cannot tell its own earlier carry from live work written since,
  // so it refuses rather than guessing -- and provoking it on a node that has
  // been written to through the ordinary verbs is the state an operator will
  // actually meet.
  out.push((
    "a migration into a board that already holds rows",
    facade
      .wb_migrate("cc", false)
      .expect_err("a board with items is not carried a second time"),
  ));
  out.push((
    "a kind another verb owns",
    facade
      .wb_add("cc", intentsvcs::model::WbItemKind::Decision, "a decision")
      .expect_err("`decision` has one writer and it is `wb decide`"),
  ));
  out.push((
    "a directive on a board that is not hv's",
    facade
      .wb_add(
        "cc",
        intentsvcs::model::WbItemKind::Directive,
        "a directive",
      )
      .expect_err("a standing directive is written on hv's board alone"),
  ));
  out.push((
    "a registered moniker named again with other values",
    facade
      .wb_register("cc", "Someone Else", "control")
      .expect_err("a registered node keeps the name it registered with"),
  ));
  out.push((
    "a correction of a moniker nobody registered",
    facade
      .wb_correct("zz", "Zed", "worker")
      .expect_err("a correction never creates a node"),
  ));
  out.push((
    "a claim that is not an address",
    facade
      .wb_claim("cc", "the whole of ST0069")
      .expect_err("a claim names something the board can point at"),
  ));
  out.push((
    "an inbox at its bound",
    facade
      .wb_ask("cc", "hv", "one too many", None, false)
      .expect_err("one past the configured bound is refused"),
  ));
  // **THE PROVOCATION THAT USED TO BE HERE WAS `organize` ON A PROJECT WITH NO
  // MANIFEST, AND IT STOPPED REFUSING (ST0057 AC-04.7).** Absent is now nobody
  // having said, so it is not an error at all -- and the comment that stood
  // here said *"a project without one refuses"*, which was the premise rather
  // than the detail. **A provocation whose subject stops refusing goes green by
  // failing to provoke**, which is why this file asserts the variant it got
  // rather than merely that it got one.
  //
  // The two manifest faults that remain are both REAL, and neither is absence:
  // a manifest that is there and cannot be READ, and one that is there and will
  // not PARSE. A directory in the file's place is the first, needs no `chmod`,
  // and behaves the same on every platform this ships to.
  std::fs::create_dir(fx.path("intent/.intentfiles")).expect("a directory in the manifest's place");
  out.push((
    "unreadable realisation manifest",
    facade
      .organize(Mode::Apply)
      .expect_err("a manifest that is there and cannot be read is refused"),
  ));
  std::fs::remove_dir(fx.path("intent/.intentfiles")).expect("take it away again");
  std::fs::write(fx.path("intent/.intentfiles"), "NOTASIGIL:ST0056\n")
    .expect("write a malformed manifest");
  // **An address form that names nothing realisation can create.** Provoked
  // rather than exempted: it needs no broken world, only an ordinary call with
  // an address that is perfectly valid and names a thing with no file form.
  out.push((
    "an address that is not an artefact",
    facade
      .hydrate(&intentsvcs::address::Address {
        authority: None,
        entity: intentsvcs::address::Entity::Event {
          id: "1".to_string(),
        },
        format: None,
      })
      .expect_err("an event has no file form"),
  ));
  // **PROVOKED, AND IT NEEDS NOTHING BROKEN EITHER: a date the caller typed.**
  // `--date` is the only way to record a completion that already happened, so
  // the value arrives from a human and the model has to refuse the ones that
  // are not days. `2026-02-30` is the case worth pinning rather than a
  // malformed string: it matches `YYYY-MM-DD` exactly, so a shape check alone
  // admits it and canon ends up holding a date no reader can turn back into
  // one.
  out.push((
    "a stated date that is not a day",
    facade
      .st_cancel_listing(
        "ST0056",
        "overtaken",
        ListEdit::AsDeclared,
        Some("2026-02-30"),
      )
      .expect_err("the thirtieth of February is not a day"),
  ));

  // **PROVOKED RATHER THAN EXEMPTED, FOR THE REASON THE EVENT CASE ABOVE
  // GIVES: it needs no broken world, only an ordinary call.** A thread's
  // `status` is owned by a ratified state machine, so the narrow setter sends
  // the caller to the lifecycle verb instead of landing the value without the
  // transition check, the gate and the recorded reason.
  out.push((
    "a field a state machine owns",
    facade
      .set(
        &intentsvcs::address::Address {
          authority: None,
          entity: intentsvcs::address::Entity::Thread {
            id: "ST0056".to_string(),
          },
          format: None,
        },
        "status",
        serde_json::json!("done"),
      )
      .expect_err("a state machine's field is not set through the narrow setter"),
  ));
  out.push((
    "malformed realisation manifest",
    facade
      .organize(Mode::Apply)
      .expect_err("a manifest with an unknown sigil is refused"),
  ));
  // **AND THE SAME BROKEN FILE REFUSES A LIFECYCLE VERB, THROUGH A DIFFERENT
  // VARIANT, WHICH IS THE WHOLE REASON THERE ARE TWO.** `edit_list` hands the
  // malformed TEXT to `pin`, so the refusal is about the edit not being
  // expressible and carries no path -- the caller supplied the text and knows
  // which file it came from. `organize` above opened the file itself, so its
  // refusal names it. Provoked here rather than exempted because it needs no
  // broken world, only an ordinary `st new` over a manifest somebody mistyped.
  // **A DIFFERENT PARSE FAULT, DELIBERATELY.** Both variants delegate to the
  // parse error for their remedy, so provoking this one with the SAME unknown
  // sigil would make two causes share a remedy -- which this file refuses, on
  // the ground that a remedy fitting two causes tells the operator to guess.
  // Per-fault remedies are the whole design; using two faults is what
  // exercises it, rather than a weakness worked around.
  // **THE VEHICLE IS A THROWAWAY THREAD, NOT THE FIXTURE'S OWN.** The first
  // repair after hv's ruling cancelled `ST0056` here, which provoked the parse
  // fault correctly and CONSUMED the subject a later arm needs: `GateBlocked`
  // stopped being provoked, because closing an already-cancelled thread is a
  // different error. A control used as a subject is spent, and nothing about
  // the file says so -- so this creates its own victim and leaves ST0056 alone.
  let doomed = facade
    .st_new("a thread that exists only to be cancelled over a bad manifest")
    .expect("st new does not touch the list, so it survives the malformed file below");
  std::fs::write(fx.path("intent/.intentfiles"), "NONSENSE\n")
    .expect("a line that is not an entry at all");
  out.push((
    "a lifecycle verb over a malformed manifest",
    // **THE VEHICLE MOVED FROM `st new` TO `st cancel` ON hv's 2026-08-27
    // RULING**, which took `st.new` out of the list-editing set: the old
    // provocation stopped provoking, and a fault that cannot be reached is a
    // fault this file cannot render. `st.cancel` still edits the list, is
    // reachable from every live state, and hits the same parse.
    facade
      .st_cancel(&doomed, "a thread whose listing cannot be written")
      .expect_err("the list edit cannot be expressed against a manifest that will not parse"),
  ));
  out.push((
    "a file cited on a non-test row",
    // The sample thread's AT-03.2 is non-test at `n-a` (0146).
    facade
      .at_edit(
        "ST0056",
        "AT-03.2",
        Some("some/test.rs".to_string()),
        None,
        None,
        None,
        None,
      )
      .expect_err("a non-test row asserts prose instead of a file"),
  ));
  out.push((
    "unknown work package",
    facade.wp_start("ST0056", 99).expect_err("no such wp"),
  ));
  out.push((
    "unknown criterion",
    facade
      .ac_satisfy("ST0056", "AC-99.9", "x")
      .expect_err("no such ac"),
  ));
  out.push((
    "unknown test",
    facade
      .at_set("ST0056", "AT-99.9", AtStatus::Green, None)
      .expect_err("no such at"),
  ));
  out.push((
    "computed satisfaction",
    facade
      .ac_satisfy("ST0056", "AC-03.1", "x")
      .expect_err("test-backed"),
  ));
  // **THE CHILD-ROW HALVES OF ISSUE 0131's REFUSAL, AND THEY ARE PROVOKED HERE
  // RATHER THAN EXEMPTED, WHICH IS THE DIFFERENCE FROM THEIR TWO SIBLINGS.**
  // `ThreadExists` and `IssueExists` need two facades over one on-disk store,
  // because the collision is detected by a UNIQUE constraint inside the write.
  // A criterion and a test are CHILD rows with no such constraint, so the check
  // is made against loaded canon and one bad call reaches it.
  out.push((
    "criterion id already taken",
    facade
      .ac_new("ST0056", "AC-03.2", "a reworded sentence", AcKind::NonTest)
      .expect_err("a create must not replace"),
  ));
  out.push((
    "acceptance test id already taken",
    facade
      .at_new(
        "ST0056",
        "AT-03.1",
        AtKind::Test,
        None,
        None,
        vec!["AC-03.1".to_string()],
        None,
      )
      .expect_err("a create must not replace"),
  ));
  out.push((
    "an edit naming no field",
    facade
      .at_edit("ST0056", "AT-03.1", None, None, None, None, None)
      .expect_err("an edit with nothing to change is refused, not reported unchanged"),
  ));
  // Issue 0337: `AT-03.2` is a non-test row, so a `green` verdict does not fit it.
  out.push((
    "a test verdict on a non-test row",
    facade
      .at_set("ST0056", "AT-03.2", AtStatus::Green, None)
      .expect_err("green on a non-test row is refused"),
  ));
  // Issue 0324: `st done` reads the packages AFTER the gate, so the thread is
  // exempt (the gate passes) and still has work package 3 WIP. Its own fixture,
  // so the calls around it see the facade they always did.
  out.push(("closing a thread with a work package still open", {
    let exempt = Fixture::new();
    let mut thread = sample_thread("ST0056");
    thread.acceptance = Some(intentsvcs::model::AcceptanceMode::Exempt);
    exempt.write_thread(&thread);
    exempt
      .facade()
      .st_done("ST0056")
      .expect_err("st done with an open work package is refused")
  }));
  // Issue 0325: a row covering a criterion that does not exist breaks the
  // contract, and the refusal carries its own remedy rather than the `put`
  // door's.
  out.push((
    "a test covering a criterion that does not exist",
    facade
      .at_new(
        "ST0056",
        "AT-99.1",
        AtKind::Test,
        None,
        None,
        vec!["AC-99.9".to_string()],
        None,
      )
      .expect_err("a row covering no criterion is refused"),
  ));
  // **PROVOKED RATHER THAN EXEMPTED, because it is provokable and an exemption
  // is a claim nobody re-drives** (issue 0207). The refusal needs a row whose
  // note is longer than the incoming one and not contained in it, so the setup
  // call lands the long note and the provoker shrinks it. **The setup is an
  // ASSERTION, not a convenience**: if `at_set` ever stops accepting the long
  // note, this provoker would fail for a reason that has nothing to do with
  // the variant, which is the collapse this file records four times already.
  //
  // **THE SETUP MUST ITSELF EXTEND, and this is here because it did not.** It
  // first landed as a bare long string, correct under the `shorter` predicate
  // and refused under containment -- so the setup failed and took three tests
  // in this file down with it. **The `expect` is what made that loud**, which
  // is what its own comment above claimed it would do; a setup written as a
  // convenience would have provoked the wrong error and this file would have
  // gone on reporting coverage it did not have.
  let existing = facade
    .st_show("ST0056")
    .expect("the fixture thread is readable")
    .tests
    .iter()
    .find(|t| t.id == "AT-03.1")
    .and_then(|t| t.note.clone())
    .unwrap_or_default();
  facade
    .at_set(
      "ST0056",
      "AT-03.1",
      AtStatus::Green,
      Some(format!(
        "{existing} -- the adjudication history this row carries, at a length that makes the loss unambiguous."
      )),
    )
    .expect("a row must be able to gain a longer note by EXTENDING the one it has");
  out.push((
    "a --note that would drop the note it replaces",
    facade
      .at_set(
        "ST0056",
        "AT-03.1",
        AtStatus::Green,
        Some("shorter".to_string()),
      )
      .expect_err("a note that does not contain the existing one is refused"),
  ));
  // The two export refusals a bad ARGUMENT can reach. Provoked here rather
  // than declared elsewhere because that is the point of this file: they are
  // the pair most at risk of collapsing into one message, and the remedy check
  // below is what stops "there is no such format" being said about a format
  // that exists and is declined.
  out.push((
    "unknown export format",
    facade
      .export(Some("xml"))
      .expect_err("there is no xml projection"),
  ));
  // **FOURTH TIME, AND THE RULE FOR AVOIDING IT IS WRITTEN TWELVE LINES BELOW.**
  // This provoked `LossyFormat` with `md`, and on 2026-08-20 `md` stopped
  // refusing: AC-06.3 made it `Projection::Realises`, so the provoker stopped
  // provoking and `expect_err` panicked -- the same shape as the three cases
  // this file already records.
  //
  // **`yaml` IS CHOSEN FROM THE DURABLE GROUND RATHER THAN FROM WHAT REFUSES
  // TODAY**, which is the property the earlier swaps found by accident and
  // this one applies on purpose. `md` was refused on a CLASSIFICATION -- it
  // was being judged by the interchange rule -- and a classification is
  // exactly what a ruling can change, which is what happened. `yaml` is
  // refused on a MEASUREMENT: PyYAML 6.0.3 resolves 6 of 24 hazardous scalars
  // to the wrong types, including every ISO date in the canon. Reversing that
  // needs a new measurement of the world, not a decision about scope.
  out.push((
    "refused export format",
    facade
      .export(Some("yaml"))
      .expect_err("yaml is read back as the wrong types by common consumers"),
  ));
  // **THE THIRD TIME THIS FILE HAS BEEN CAUGHT BY THE SAME MECHANISM, and the
  // first two are commented seventy lines below** (issue 0053). This provoked
  // `NotOffScope` with `ac_reinstate` on AC-03.1 -- test-kind at `Computed`, which
  // IS `AcState::entry(Test)`, so once the verb stopped refusing its own target
  // state the provoker stopped provoking and `expect_err` panicked.
  //
  // **The route is chosen from the DECLARED machine rather than from what is
  // refused today**, which is the property the two earlier swaps found by
  // accident. `ac.reinstate` declares its edges only from `withdrawn`, so every
  // in-scope state that is not the verb's own target is durably refusable --
  // AC-03.2 is non-test and SATISFIED, so it is refused for a reason a ruling
  // cannot reverse without changing the machine itself.
  out.push((
    "reinstate in-scope",
    facade
      .ac_reinstate("ST0056", "AC-03.2")
      .expect_err("in scope, and not at the state reinstate targets"),
  ));

  // **`ScopeUnchanged` was provoked here and the variant is gone** (hv,
  // 2026-08-17): a repeated withdrawal is a self-loop, accepted at exit 0, so
  // there is no longer an error to collect. The behaviour it used to provoke is
  // asserted as an OUTCOME in `facade_acceptance.rs`; a refusal roster is the
  // wrong place to keep a case that is no longer a refusal.
  facade
    .ac_withdraw("ST0056", "AC-03.1", "r", None)
    .expect("withdraw");

  // AC-03.1 is now withdrawn, so the thread cannot close.
  facade
    .at_set("ST0056", "AT-03.1", AtStatus::Red, None)
    .unwrap();
  facade
    .at_set("ST0056", "AT-03.7", AtStatus::Red, None)
    .unwrap();
  facade
    .ac_reinstate("ST0056", "AC-03.1")
    .expect("back in scope so the gate has something to block on");
  out.push((
    "gate blocked",
    facade.st_done("ST0056").expect_err("gate blocks"),
  ));
  // **The fiat close's own refusal, provoked rather than declared elsewhere.**
  // AC-03.1 is test-backed and back at `computed` by the reinstate above, which
  // is one of the two states `ac.fc` is declared from -- so the first close
  // succeeds and the second is refused for a reason the machine guarantees:
  // `fiat` is not a from-state. **The route is chosen from the DECLARED machine
  // rather than from what is refused today**, which is the property issue 0053
  // cost this file three times.
  facade
    .ac_fc("ST0056", "AC-03.1", "hv closed it on authority", "hv")
    .expect("the first close lands");
  out.push((
    "already fiat-closed",
    facade
      .ac_fc("ST0056", "AC-03.1", "and again", "hv")
      .expect_err("a requirement closed on authority cannot be closed again"),
  ));
  facade
    .ac_reinstate("ST0056", "AC-03.1")
    .expect("put it back where the rest of this fixture expects it");
  // **Six of the refusals below were reachable and asserted nowhere in this
  // file**, measured at `c1e630cf` -- so the module doc's "the whole variant
  // set rather than sampled" was already false before today's two variants
  // existed to widen it. Found by the coverage check below on its first run,
  // which is the argument for having written it.
  //
  // AC-03.2 is the fixture's only NON-TEST criterion, so it is the one that can
  // reach the kind-gated refusals at all, and it is walked through the states
  // deliberately: satisfied -> withdrawn -> back in scope -> unsatisfied. The
  // order IS the fixture here.
  out.push((
    "descope target does not exist",
    facade
      .ac_descope("ST0056", "AC-03.2", "ST9999", None, None)
      .expect_err("no such thread"),
  ));
  out.push((
    "descope target not named",
    facade
      .ac_descope("ST0056", "AC-03.2", "  ", None, None)
      .expect_err("blank target"),
  ));

  facade
    .ac_withdraw("ST0056", "AC-03.2", "the premise did not reproduce", None)
    .expect("withdraw the non-test criterion");
  out.push((
    "satisfy something out of scope",
    facade
      .ac_satisfy("ST0056", "AC-03.2", "x")
      .expect_err("withdrawn"),
  ));
  out.push((
    "rescope what was withdrawn",
    facade
      .ac_rescope("ST0056", "AC-03.2")
      .expect_err("rescope undoes a descope, not a withdrawal"),
  ));
  // **MOVED HERE FROM THE `unsatisfied` STEP BELOW, and this is the SECOND time
  // this file has been caught by the same mechanism.** `NotSatisfied` was
  // provoked by `ac_unsatisfy` on an already-unsatisfied criterion -- which hv's
  // self-loop ruling makes an accepted no-op, so the provoker stopped provoking
  // and `expect_err` panicked. The comment fifty lines down records the identical
  // swap being made for `IllegalTransition` on the same day; nobody then asked
  // which OTHER provoker depended on a state being refusable.
  //
  // `withdrawn` is a durable route to the same refusal: `ac.unsatisfy` is declared
  // from `satisfied` alone, so every refusal it can produce means "not satisfied",
  // and the facade maps the declared machine's `IllegalTransition` onto this
  // variant rather than hand-checking the from-state ahead of the self-loop test.
  out.push((
    "nothing to unsatisfy",
    facade.ac_unsatisfy("ST0056", "AC-03.2").expect_err(
      "a withdrawn criterion is not satisfied, and unsatisfy is declared only from satisfied",
    ),
  ));

  facade
    .ac_reinstate("ST0056", "AC-03.2")
    .expect("back in scope, unsatisfied");
  out.push((
    "evidence required",
    facade
      .ac_satisfy("ST0056", "AC-03.2", "  ")
      .expect_err("blank evidence"),
  ));
  out.push((
    "reason required",
    facade
      .ac_withdraw("ST0056", "AC-03.2", "   ", None)
      .expect_err("blank reason"),
  ));
  // The from-state refusal, which is a different failure from every guard above
  // it: the value is fine and the thread is in the wrong state to receive it.
  //
  // **This provoked the refusal with `st_resume` until self-loops became legal
  // (hv, 2026-08-17), and the swap is not cosmetic.** `st.resume` TARGETS `wip`
  // and the fixture thread is `wip`, so that call is now a self-loop -- accepted
  // at exit 0 -- and the provocation quietly stopped provoking. What decides a
  // self-loop is whether the current state equals the verb's TARGET, not whether
  // the verb is declared from the current state, so a provocation has to name a
  // verb whose target differs. `st.triage` lands on `not-started` and is declared
  // only from `triage`, which is a real movement the machine refuses.
  out.push((
    "illegal transition",
    facade
      .st_triage("ST0056")
      .expect_err("triage is declared only from `triage`, targets `not-started`, and the fixture thread is `wip` -- a refused movement rather than a no-op"),
  ));

  out.push((
    "unknown issue",
    fx.facade().issue_show(9999).expect_err("no such issue"),
  ));
  out.push((
    "PUT to a server-assigned id",
    facade
      .put(
        &intentsvcs::address::parse("intent:///threads/ST0058").expect("resolves"),
        "{}",
      )
      .expect_err("a thread id is server-assigned -- POST to the collection"),
  ));

  // **ITS OWN FIXTURE, because provoking this one POISONS the store it is
  // provoked against.** The whole property is that the block PERSISTS, so
  // reaching for the shared facade would refuse every later egest in this
  // function for a reason that has nothing to do with the case being made --
  // and a provocation whose side effect is a second, unrelated refusal is how
  // a roster starts asserting the wrong thing about the right variant.
  let refused_ingest = Fixture::new();
  refused_ingest.write_thread(&sample_thread("ST0056"));
  let mut blocked = refused_ingest.facade();
  // **ANY refused load will do, and using the CHEAPEST one is the point.** The
  // variant is about the recorded OUTCOME, not about the cause that produced
  // it, so provoking it from a schema refusal rather than from the store-level
  // one that motivated it is a small independent check that the guard did not
  // quietly become specific to a single failure.
  refused_ingest.write_raw_thread("ST0057", r#"{"schema":"intent/thread@3.0","id":"ST0057"}"#);
  blocked
    .sync_from_disk(&intentsvcs::sync::Scope::All)
    .expect_err("canon missing its mandatory fields refuses the ingest");
  out.push((
    "egest from a store whose last ingest was refused",
    blocked
      .sync_to_disk(&intentsvcs::sync::Scope::All)
      .expect_err("the store may be older than the canon this would overwrite"),
  ));

  // Its own fixture too, and for a different reason from the one above: this
  // one needs a store that is WARM and wrong, which no call on a healthy facade
  // produces. One non-empty table is what makes it warm -- an entirely empty
  // store is COLD and warms itself from the files on the next open.
  let emptied = Fixture::new();
  emptied.write_thread(&sample_thread("ST0056"));
  emptied.write_issue(&crate::common::sample_issue(21));
  emptied
    .facade_on_disk()
    .sync_to_disk(&intentsvcs::sync::Scope::All)
    .expect("a healthy estate projects");
  {
    let mut store = intentsvcs::store::Store::open(&emptied.project().db_path()).expect("open");
    let (_, issues) = store.load_canon().expect("read");
    store.rebuild(&[], &issues).expect("the threads are gone");
  }
  out.push((
    "egest that would empty a populated estate",
    emptied
      .facade_on_disk()
      .sync_to_disk(&intentsvcs::sync::Scope::All)
      .expect_err("the store holds no threads and the estate has one"),
  ));

  // **BOTH OF DEHYDRATION'S REFUSALS, PROVOKED RATHER THAN EXEMPTED, ON A
  // FIXTURE OF THEIR OWN.** Its own fixture for the reason the two above give:
  // the shared facade has been walked through a broken manifest, a malformed
  // one and a nonsense one by the time it gets here, and both of these need a
  // manifest that PARSES to reach their own refusal rather than the parser's.
  //
  // **THE ORDER IS FORCED AND IT IS THE INTERESTING PART**: the refusal needs
  // the manifest present, and the absence needs it gone, so the present case
  // must come first and the file is taken away between them.
  let dehydrating = Fixture::new();
  dehydrating.write_thread(&sample_thread("ST0056"));
  let mut d = dehydrating.facade();
  let st0056 = intentsvcs::address::Address {
    authority: None,
    entity: intentsvcs::address::Entity::Thread {
      id: "ST0056".to_string(),
    },
    format: None,
  };
  // A manifest that parses and declares the thread, so `unpin` has something to
  // remove and the plan classifies the thread's files as this run's to act on.
  std::fs::write(
    dehydrating.path("intent/.intentfiles"),
    "STEELTHREAD:ST0056\n",
  )
  .expect("a manifest declaring the one thread");
  let realised = d.hydrate(&st0056).expect("a declared thread realises");
  assert!(
    !realised.is_empty(),
    "the provocation below needs a realised file to hand-edit; hydrate produced none, so this \
     case would pass by failing to provoke -- the class the manifest-absence comment above names"
  );
  for file in &realised {
    std::fs::write(file, "a hand edit no render could have produced\n")
      .expect("make the bytes disagree with the store");
  }
  out.push((
    "a realised file the store cannot be shown to hold",
    d.dehydrate(&st0056)
      .expect_err("a file whose bytes the store cannot reproduce is refused, never removed"),
  ));
  std::fs::remove_file(dehydrating.path("intent/.intentfiles"))
    .expect("take the manifest away entirely");
  out.push((
    "dehydrate with no manifest to unlist from",
    d.dehydrate(&st0056)
      .expect_err("absent means nobody has said, so there is no list to remove an entry from"),
  ));

  // **ISSUE 0206's REFUSAL, AND IT NEEDS A FIXTURE OF ITS OWN.** Every
  // provocation above runs against `fx.facade()`, which is an IN-MEMORY store;
  // two of those share no database, so the record cannot move under one of them
  // and this variant is unreachable there. A second on-disk fixture is the
  // cheapest honest provocation -- and a fixture that cannot exhibit the defect
  // could not have provoked its refusal.
  let shared = Fixture::new();
  shared.write_thread(&sample_thread("ST0056"));
  let mut first = shared.facade_on_disk();
  let mut second = shared.facade_on_disk();
  first
    .ac_edit(
      "ST0056",
      "AC-03.1",
      Some("the edit that lands".to_string()),
      None,
    )
    .expect("the first write is ordinary");
  out.push((
    "a record that moved under the write",
    second
      .ac_edit(
        "ST0056",
        "AC-03.2",
        Some("the edit derived from a stale record".to_string()),
        None,
      )
      .expect_err("the second facade holds a snapshot the first has already superseded"),
  ));

  // **`0262`'s REFUSAL, PROVOKED RATHER THAN EXEMPTED.** The path here is the
  // exact spelling the commit gate's own remedy invites -- repo-relative, which
  // to a reader is what `git status` prints -- and before this variant existed
  // it returned `ok:` and minted a second attachment row for a file that
  // already had one.
  out.push((
    "an attachment path relative to the repository",
    facade
      .put_attachment(
        &intentsvcs::address::Address {
          authority: None,
          entity: intentsvcs::address::Entity::Attachment {
            thread: "ST0056".to_string(),
            path: "intent/st/ST0056/parity/probe.txt".to_string(),
          },
          format: None,
        },
        b"PROBE\n",
      )
      .expect_err("a repo-relative attachment path names nowhere in the thread"),
  ));

  // **ST0078 WP-02's REFUSAL, PROVOKED.** A renumber onto an id the store
  // already holds: here the thread's own, which is the smallest bad call.
  out.push((
    "a renumber onto an id the store holds",
    facade
      .st_renumber("ST0056", "ST0056")
      .expect_err("ST0056 is taken by ST0056"),
  ));

  // **`0394`'s REFUSAL, PROVOKED.** A detach naming an attachment the thread
  // does not carry: nothing is removed, and the remedy says where the paths it
  // does carry are.
  out.push((
    "a detach of an attachment the thread does not carry",
    facade
      .detach_attachment(&intentsvcs::address::Address {
        authority: None,
        entity: intentsvcs::address::Entity::Attachment {
          thread: "ST0056".to_string(),
          path: "never-attached.md".to_string(),
        },
        format: None,
      })
      .expect_err("ST0056 carries no attachment at never-attached.md"),
  ));
  // **`0460`'s THREE REFUSALS, PROVOKED.** A link to a thread that does not
  // exist, a link from a thread to itself, and the drop of a link the thread
  // does not carry: nothing is written by any of them.
  out.push((
    "a relate naming a thread the project does not carry",
    facade
      .st_relate("ST0056", "ST9999", None)
      .expect_err("ST9999 is not a thread here"),
  ));
  out.push((
    "a relate of a thread to itself",
    facade
      .st_relate("ST0056", "ST0056", None)
      .expect_err("a thread is not related to itself"),
  ));
  out.push((
    "an unrelate of a link the thread does not carry",
    facade
      .st_unrelate("ST0056", "ST9999")
      .expect_err("ST0056 carries no link to ST9999"),
  ));
  // **ST0076 WP-05's REFUSAL, PROVOKED.** Level 3 asked of a build carrying no
  // resolver for any declared language: nothing runs, and the remedy says what
  // still answers.
  out.push((
    "level 3 asked of a build with no resolver",
    facade
      .index_resolve(None, false, &[])
      .expect_err("no resolver is carried"),
  ));
  out.push((
    "an address naming another project",
    facade
      .post(
        &intentsvcs::address::parse("intent://elsewhere/threads").expect("resolves"),
        "{}",
      )
      .expect_err("another project's address is refused"),
  ));

  // **`0398`'s REFUSAL, PROVOKED.** A read naming a document the thread does not
  // carry: `ST0056` carries `reference.md` and `parity/cmd-st.md`, so the remedy
  // is the door that attaches an `impl.md` rather than a list to pick from.
  out.push((
    "a read of a document the thread does not carry",
    facade
      .st_attachment("ST0056", "impl.md")
      .expect_err("ST0056 carries no impl.md"),
  ));

  // **`0270`'s REFUSAL, AND IT NEEDS A FIXTURE OF ITS OWN** -- the same reason
  // 0206's does, one paragraph up. Every provocation above runs against one
  // facade in sequence, so by this point `AT-03.1` has already been driven to
  // `red` by an earlier arm and `at_set` short-circuits at `AlreadyThere`
  // BEFORE reaching this guard. **The first spelling of this arm hit exactly
  // that and reported a success where a refusal was required.**
  //
  // A clean thread with one `to-write` row citing a file that does not exist is
  // the shape: it is legal today, exempt from `absent_at`, and one `at red` away
  // from a finding that refuses every commit in the repository.
  let armed = Fixture::new();
  let mut armed_thread = sample_thread("ST0056");
  armed_thread.tests = vec![AcceptanceTest {
    id: "AT-09.1".to_string(),
    file: Some("crates/intentsvcs/tests/never_written.rs".to_string()),
    status: AtStatus::ToWrite,
    ..armed_thread.tests[0].clone()
  }];
  armed.write_thread(&armed_thread);
  // **THE FIXTURE CREATES THE FILES ITS ROWS CITE**, so the absence has to be
  // arranged rather than assumed. Without this the row cites a path that EXISTS,
  // the guard correctly does not fire, and the arm reports a successful
  // transition -- a subject that cannot exhibit the condition, which is the
  // third sighting of that shape on this estate today.
  let _ = std::fs::remove_file(
    armed
      .root()
      .join("crates/intentsvcs/tests/never_written.rs"),
  );
  assert!(
    !armed
      .root()
      .join("crates/intentsvcs/tests/never_written.rs")
      .exists(),
    "the citation must be absent or this arm proves nothing"
  );
  let mut armed_facade = armed.facade();
  let absent_verdict = armed_facade
    .at_set("ST0056", "AT-09.1", AtStatus::Red, None)
    .expect_err("a red verdict on a citation that does not exist is refused");
  // **THE VARIANT IS ASSERTED, NOT ASSUMED.** A provocation that fails to
  // provoke is green, and this one already failed silently once by reaching
  // `NoSuchTest` instead -- caught only because that variant's remedy collided
  // with this one's in `no_two_distinct_causes_render_the_same_text`.
  assert_eq!(
    variant(&absent_verdict),
    "VerdictCitesAbsentFile",
    "this arm must reach 0270's guard rather than an earlier refusal: {absent_verdict}"
  );
  out.push((
    "a verdict on a row whose cited file is absent",
    absent_verdict,
  ));

  // **`0271`'s TWO REFUSALS, PROVOKED RATHER THAN DECLARED, BECAUSE THE WHOLE
  // ISSUE IS THAT THEY DID NOT HAPPEN.** `migration.md` states three
  // preconditions "refused by name, not worked around" and only the floor
  // existed: a no-git estate converted 349 threads at exit 0 with the word
  // `git` absent from the entire run, and a dirty tree converted over two
  // uncommitted paths without mentioning dirt. A variant listed in
  // `ALL_VARIANTS` and reached by nothing would restate that defect one level
  // up -- the roster would claim coverage the suite does not have.
  //
  // **EACH GETS ITS OWN ESTATE, AND THE ORDER IS THE DISCRIMINATOR.** The
  // no-git arm must not be a git repository at all; the dirty arm must BE one
  // and be dirty, which is `git_init` with nothing committed. Sharing a fixture
  // would make the second arm depend on what the first left behind, and
  // `v2_estate` is deliberately git-free so both remain writable.
  let no_git = v2_estate();
  v2_thread(&no_git, "ST0001", "WIP");
  let without_git = Facade::upgrade(&no_git.project(), &facade_ctx())
    .expect_err("a v2 estate with no git repository is refused");
  assert_eq!(
    variant(&without_git),
    "MigrationWithoutGit",
    "this arm must reach the git precondition rather than an earlier refusal: {without_git}"
  );
  out.push(("a migration with no git repository", without_git));

  let dirty = v2_estate();
  dirty.git_init();
  v2_thread(&dirty, "ST0001", "WIP");
  let over_dirt = Facade::upgrade(&dirty.project(), &facade_ctx())
    .expect_err("a v2 estate with a dirty tree is refused");
  assert_eq!(
    variant(&over_dirt),
    "MigrationOverDirtyTree",
    "this arm must reach the clean-tree precondition, and reaching \
     MigrationWithoutGit here would mean an initialised repo with no commits is \
     being read as no repository at all: {over_dirt}"
  );
  out.push(("a migration over a dirty tree", over_dirt));

  // `sync --apply --plan <digest>` naming a plan the tree does not have.
  let plan_fx = Fixture::new();
  let moved = plan_fx
    .facade()
    .sync_apply(
      &intentsvcs::sync::Scope::All,
      intentsvcs::plan::Asking {
        yes: false,
        terminal: false,
        shown: Some("a plan nobody printed".to_string()),
      },
      &mut |_| intentsvcs::plan::Decision::Decline,
    )
    .expect_err("a digest the tree does not have is refused");
  out.push(("an apply pinned to a plan the tree moved past", moved));

  out
}

/// The variant a value is, as an EXHAUSTIVE match.
///
/// **The module doc says this file checks "the whole variant set rather than
/// sampled", and until now nothing made that true.** `provoked_errors` is a
/// hand-built list; a variant added to the facade and not to it was covered by
/// no assertion at all, and the doc claiming otherwise is the same shape as the
/// model comment that claimed empty evidence was unconstructible -- a written
/// guarantee standing in for a mechanism.
///
/// The match is what closes it: **a new variant does not compile until someone
/// adds an arm here**, and the arm is one line away from the test below telling
/// them to provoke it. The residual is stated rather than hidden: the arm and
/// [`ALL_VARIANTS`] are two lists, so a variant added to the match and not to
/// the list still slips the coverage check. That is a much smaller hole than
/// the one it replaces, and it is the smallest this gets without reflection or
/// a derive dependency.
fn variant(err: &FacadeError) -> &'static str {
  match err {
    FacadeError::WriteNotAddressable { .. } => "WriteNotAddressable",
    FacadeError::RowBreaksContract { .. } => "RowBreaksContract",
    FacadeError::VerdictWrongForKind { .. } => "VerdictWrongForKind",
    FacadeError::OpenWorkPackages { .. } => "OpenWorkPackages",
    FacadeError::AttachmentPathNotInThread { .. } => "AttachmentPathNotInThread",
    FacadeError::NoSuchAttachment { .. } => "NoSuchAttachment",
    FacadeError::NoSuchRelatedTarget { .. } => "NoSuchRelatedTarget",
    FacadeError::RelatedToItself { .. } => "RelatedToItself",
    FacadeError::NoSuchRelated { .. } => "NoSuchRelated",
    FacadeError::NotCarried { .. } => "NotCarried",
    FacadeError::CrossProjectAddress { .. } => "CrossProjectAddress",
    FacadeError::VerdictCitesAbsentFile { .. } => "VerdictCitesAbsentFile",
    FacadeError::NoSuchThread { .. } => "NoSuchThread",
    FacadeError::ThreadExists { .. } => "ThreadExists",
    FacadeError::IssueExists { .. } => "IssueExists",
    FacadeError::RenumberTargetTaken { .. } => "RenumberTargetTaken",
    FacadeError::RenumberDiskStep { .. } => "RenumberDiskStep",
    FacadeError::SyncPlanMoved { .. } => "SyncPlanMoved",
    FacadeError::RenumberNotMerging { .. } => "RenumberNotMerging",
    FacadeError::SyncDiskStep { .. } => "SyncDiskStep",
    FacadeError::Git(_) => "Git",
    FacadeError::CriterionExists { .. } => "CriterionExists",
    FacadeError::TestExists { .. } => "TestExists",
    FacadeError::NothingToChange { .. } => "NothingToChange",
    FacadeError::NoSuchWorkPackage { .. } => "NoSuchWorkPackage",
    FacadeError::NoSuchCriterion { .. } => "NoSuchCriterion",
    FacadeError::NoSuchTest { .. } => "NoSuchTest",
    FacadeError::GateBlocked { .. } => "GateBlocked",
    FacadeError::ComputedSatisfaction { .. } => "ComputedSatisfaction",
    FacadeError::NotOffScope { .. } => "NotOffScope",
    FacadeError::NotSatisfied { .. } => "NotSatisfied",
    FacadeError::AlreadyFiatClosed { .. } => "AlreadyFiatClosed",
    FacadeError::OffScope { .. } => "OffScope",
    FacadeError::WrongOffScopeState { .. } => "WrongOffScopeState",
    FacadeError::BadQuery { .. } => "BadQuery",
    FacadeError::SearchUnanswerable { .. } => "SearchUnanswerable",
    FacadeError::SqlMoreThanOneStatement => "SqlMoreThanOneStatement",
    FacadeError::SqlNoStatement => "SqlNoStatement",
    FacadeError::SqlUnterminated => "SqlUnterminated",
    FacadeError::SqlWouldWrite => "SqlWouldWrite",
    FacadeError::SqlOutOfReach { .. } => "SqlOutOfReach",
    FacadeError::SqlOverBudget => "SqlOverBudget",
    FacadeError::SqlLimitAboveCeiling { .. } => "SqlLimitAboveCeiling",
    FacadeError::StructuralTierNotAsked => "StructuralTierNotAsked",
    FacadeError::SqlDidNotRun { .. } => "SqlDidNotRun",
    FacadeError::NoSuchFace { .. } => "NoSuchFace",
    FacadeError::IllegalTransition { .. } => "IllegalTransition",
    FacadeError::ReasonRequired { .. } => "ReasonRequired",
    FacadeError::EvidenceRequired { .. } => "EvidenceRequired",
    FacadeError::DescopeTargetMissing { .. } => "DescopeTargetMissing",
    FacadeError::DescopeTargetRequired { .. } => "DescopeTargetRequired",
    FacadeError::Unmigrated(_) => "Unmigrated",
    FacadeError::BelowMigrationFloor(_) => "BelowMigrationFloor",
    FacadeError::MigrationWithoutGit => "MigrationWithoutGit",
    FacadeError::MigrationOverDirtyTree { .. } => "MigrationOverDirtyTree",
    FacadeError::Write(_) => "Write",
    FacadeError::ViewsNotWritten { .. } => "ViewsNotWritten",
    FacadeError::Embed(_) => "Embed",
    FacadeError::Store(_) => "Store",
    FacadeError::Ingest(_) => "Ingest",
    FacadeError::NoSuchFormat { .. } => "NoSuchFormat",
    FacadeError::LossyFormat { .. } => "LossyFormat",
    FacadeError::ExportRoundTripFailed { .. } => "ExportRoundTripFailed",
    FacadeError::NoSuchIssue { .. } => "NoSuchIssue",
    FacadeError::MalformedIssueId { .. } => "MalformedIssueId",
    FacadeError::MigrationBlocked(_) => "MigrationBlocked",
    FacadeError::MigrationHalted { .. } => "MigrationHalted",
    FacadeError::EgestFromRefusedIngest { .. } => "EgestFromRefusedIngest",
    FacadeError::EgestWouldEmptyTheEstate { .. } => "EgestWouldEmptyTheEstate",
    FacadeError::EgestFromStaleStore { .. } => "EgestFromStaleStore",
    FacadeError::IngestOutpacedByWrites { .. } => "IngestOutpacedByWrites",
    FacadeError::WriteWouldEmptyAnAuthoredBody { .. } => "WriteWouldEmptyAnAuthoredBody",
    FacadeError::Realise(_) => "Realise",
    FacadeError::Organize(_) => "Organize",
    FacadeError::Intentfiles(_) => "Intentfiles",
    FacadeError::ManifestUnreadable { .. } => "ManifestUnreadable",
    FacadeError::ManifestMalformed { .. } => "ManifestMalformed",
    FacadeError::NotHydratable { .. } => "NotHydratable",
    FacadeError::NoManifestToUnlistFrom { .. } => "NoManifestToUnlistFrom",
    FacadeError::HydrationWouldOverwrite { .. } => "HydrationWouldOverwrite",
    FacadeError::RealisationWouldRemove { .. } => "RealisationWouldRemove",
    FacadeError::DehydrationRefused { .. } => "DehydrationRefused",
    FacadeError::NotEditable { .. } => "NotEditable",
    FacadeError::NoSuchEditable { .. } => "NoSuchEditable",
    FacadeError::FieldNotWritable { .. } => "FieldNotWritable",
    FacadeError::ValueNotRecordable { .. } => "ValueNotRecordable",
    FacadeError::NoteWouldBeLost { .. } => "NoteWouldBeLost",
    FacadeError::FileOnANonTestRow { .. } => "FileOnANonTestRow",
    FacadeError::Install(_) => "Install",
    FacadeError::RootFile(_) => "RootFile",
    FacadeError::Canon(_) => "Canon",
    FacadeError::RecordMovedUnderTheWrite { .. } => "RecordMovedUnderTheWrite",
    FacadeError::NoFormForEntity { .. } => "NoFormForEntity",
    FacadeError::EntityUnserialisable { .. } => "EntityUnserialisable",
    FacadeError::WbNodeNotRegistered { .. } => "WbNodeNotRegistered",
    FacadeError::WbBodyOverBound { .. } => "WbBodyOverBound",
    FacadeError::WbInboxFull { .. } => "WbInboxFull",
    FacadeError::WbItemsFull { .. } => "WbItemsFull",
    FacadeError::WbClaimMalformed { .. } => "WbClaimMalformed",
    FacadeError::WbAlreadyCarried { .. } => "WbAlreadyCarried",
    FacadeError::WbSendersNotRegistered { .. } => "WbSendersNotRegistered",
    FacadeError::WbHeaderIncomplete { .. } => "WbHeaderIncomplete",
    FacadeError::WbNotMigrated { .. } => "WbNotMigrated",
    FacadeError::WbKindHasItsOwnVerb { .. } => "WbKindHasItsOwnVerb",
    FacadeError::WbDirectiveOffHv { .. } => "WbDirectiveOffHv",
    FacadeError::WbDirectivesOnAnotherBoard { .. } => "WbDirectivesOnAnotherBoard",
    FacadeError::WbRegisteredDifferently { .. } => "WbRegisteredDifferently",
    FacadeError::WbRegisterDisagreesWithHeader { .. } => "WbRegisterDisagreesWithHeader",
    FacadeError::WbCorrectUnregistered { .. } => "WbCorrectUnregistered",
    FacadeError::WbUncarried { .. } => "WbUncarried",
    FacadeError::WbSnapshotInTheWay { .. } => "WbSnapshotInTheWay",
    FacadeError::WbNoActingNode => "WbNoActingNode",
    FacadeError::NoResolver { .. } => "NoResolver",
    FacadeError::NothingResolved { .. } => "NothingResolved",
    FacadeError::NoSuchTarget { .. } => "NoSuchTarget",
  }
}

/// The variants a reader of this file should expect to see provoked.
///
/// Some are reachable only through a failing filesystem or a damaged store, and
/// those are declared here as deliberately-not-provoked rather than left to
/// look like oversights -- an exemption that is announced, never inferred
/// (ST0048's rule).
const ALL_VARIANTS: &[&str] = &[
  "Embed",
  "SqlMoreThanOneStatement",
  "SqlNoStatement",
  "SqlUnterminated",
  "SqlWouldWrite",
  "SqlOutOfReach",
  "SqlOverBudget",
  "SqlLimitAboveCeiling",
  "StructuralTierNotAsked",
  "SqlDidNotRun",
  "ValueNotRecordable",
  "NoteWouldBeLost",
  "FileOnANonTestRow",
  "NotHydratable",
  "NoManifestToUnlistFrom",
  "HydrationWouldOverwrite",
  "RealisationWouldRemove",
  "DehydrationRefused",
  "NotEditable",
  "NoSuchEditable",
  "NoSuchAttachment",
  "NoSuchRelatedTarget",
  "RelatedToItself",
  "NoSuchRelated",
  "NotCarried",
  "CrossProjectAddress",
  "Organize",
  "Intentfiles",
  "ManifestUnreadable",
  "ManifestMalformed",
  "NoSuchThread",
  "ThreadExists",
  "IssueExists",
  "RenumberTargetTaken",
  "RenumberDiskStep",
  "SyncPlanMoved",
  "RenumberNotMerging",
  "SyncDiskStep",
  "Git",
  "CriterionExists",
  "TestExists",
  "NothingToChange",
  "NoSuchWorkPackage",
  "NoSuchCriterion",
  "NoSuchTest",
  "GateBlocked",
  "ComputedSatisfaction",
  "NotOffScope",
  "NotSatisfied",
  "AlreadyFiatClosed",
  "OffScope",
  "WrongOffScopeState",
  "BadQuery",
  "SearchUnanswerable",
  "NoSuchFace",
  "IllegalTransition",
  "ReasonRequired",
  "EvidenceRequired",
  "DescopeTargetMissing",
  "DescopeTargetRequired",
  "Unmigrated",
  "BelowMigrationFloor",
  "MigrationWithoutGit",
  "MigrationOverDirtyTree",
  "Write",
  "ViewsNotWritten",
  "Store",
  "Ingest",
  "NoSuchFormat",
  "LossyFormat",
  "WriteNotAddressable", // PUT to a server-assigned id -- `mutation_create_splits_two_ways.rs`
  "RowBreaksContract",
  "VerdictWrongForKind",
  "OpenWorkPackages",
  "ExportRoundTripFailed",
  "NoSuchIssue",
  "MalformedIssueId",
  "MigrationBlocked",
  "MigrationHalted",
  "EgestFromRefusedIngest",
  "EgestWouldEmptyTheEstate",
  "EgestFromStaleStore",
  "IngestOutpacedByWrites",
  "WriteWouldEmptyAnAuthoredBody",
  "Realise",
  "Install",
  "RootFile",
  "Canon",
  "RecordMovedUnderTheWrite",
  "WbNodeNotRegistered",
  "WbBodyOverBound",
  "WbInboxFull",
  "WbItemsFull",
  "WbClaimMalformed",
  "WbAlreadyCarried",
  "WbSendersNotRegistered",
  "WbHeaderIncomplete",
  "WbNotMigrated",
  "WbKindHasItsOwnVerb",
  "WbDirectiveOffHv",
  "WbDirectivesOnAnotherBoard",
  "WbRegisteredDifferently",
  "WbRegisterDisagreesWithHeader",
  "WbCorrectUnregistered",
  "WbUncarried",
  "WbSnapshotInTheWay",
  "WbNoActingNode",
  "NoResolver",
  "NothingResolved",
  "NoSuchTarget",
];

/// Variants that need a broken world rather than a bad call, and are covered by
/// the tests that break that world instead.
const NOT_PROVOKED_HERE: &[&str] = &[
  // **RAISED BY THE RENDERER AND BY NOTHING ON THE FACADE.** `WbNoActingNode`
  // answers "nothing said which node is writing", and the facade's whiteboard
  // doors all TAKE the acting node as a parameter -- it is the CLI's
  // `--node`/`INTENT_NODE` resolution that can come up empty. It lives on
  // `FacadeError` so its message and remedy sit beside the refusals it is read
  // next to; provoking it here would mean constructing the value rather than
  // reaching it, which asserts nothing about a path the estate takes.
  "WbNoActingNode",
  // **PROVOKED WHERE A HAND-AUTHORED BOARD IS.** `wb migrate` reads a board off
  // disk, and this refusal needs one carrying `## Standing directives` on a node
  // that is not `hv`: `wb_migrate_carries_a_board.rs` has that fixture and
  // drives it, and building one here would be a second copy of that fixture.
  "WbDirectivesOnAnotherBoard",
  // The same reading for the carry's two refusals of vc decision 20: one needs a
  // board holding a unit the model cannot carry, and the other a pre-migration
  // snapshot already on disk with other bytes. Both fixtures are in
  // `wb_migrate_carries_a_board.rs`, which drives each.
  "WbUncarried",
  "WbSnapshotInTheWay",
  // **PROVOKED WHERE A RESOLUTION IS STORED.** A target no resolved row names is
  // refused only when resolved targets end the same way, so it needs indexed
  // source and a resolution run joined to it: intent-cli's
  // `a_search_by_target_asks_one_question.rs` seeds that estate and drives the
  // refusal through the binary, and building one here would be a second copy
  // of that fixture.
  "NoSuchTarget",
  // **UNREACHABLE THROUGH EVERY DOOR THAT EXISTS TODAY, AND KEPT FOR THE SAME
  // REASON THE OTHERS HERE ARE KEPT: THE ALTERNATIVE WAS A LIE.** ST0069
  // WP-01's `issue_home` turns a manifest id into an issue's view path.
  // `Sigil::accepts` gates a manifest line on `model::is_issue_id` and
  // `Entity::Issue` is minted behind the same predicate, so nothing but four
  // digits reaches the parse -- there is no facade call that provokes this and
  // constructing one here would assert a path the estate cannot take.
  //
  // It exists because the two alternatives were worse. `unwrap_or(0)` resolves
  // a malformed id to issue `0000`'s view and then realises or removes the
  // WRONG FILE at exit 0; reusing `NoSuchIssue { number: 0 }` -- an idiom
  // already in `facade.rs` -- prints "no issue 0000 in this project", which is
  // a sentence about a project's contents for a fault in an id's shape.
  "MalformedIssueId",
  // **THE EMBEDDER'S REFUSALS ARE DRIVEN WHERE THE EMBEDDER IS**, in
  // `the_semantic_tier_is_staged_and_its_seams_hold.rs`: the Null one refuses a
  // semantic query with the remedy naming the configuration, and the HTTP one
  // is driven against a listener that answers. Provoking it through a facade
  // call here would need a project configured with an endpoint and a socket
  // behind it, which is that file's fixture and not this one's.
  "Embed",
  // **BOTH LANDED IN HEAD WITHOUT THIS ARM, BY ic, AT db3f947a -- THE SAME
  // CLASS `Realise` BELOW RECORDS AGAINST SOMEONE ELSE.** `agents generate`
  // and `agents validate` moved onto the facade so MCP could reach them, the
  // two wrapping variants were added with `#[from]`, and this test binary
  // stopped compiling for every node. Nothing ran it for a day: the CLI's
  // suites build the CLI's tests and not this crate's. Found at the next
  // `cargo check --workspace --all-targets`, which is the instrument that
  // should have preceded that landing.
  //
  // `Install` is reachable only when the running binary sits OUTSIDE an
  // Intent install: `install::resolve` walks up from `current_exe` looking for
  // `lib/templates/`, and every test binary in this workspace sits under the
  // repository root, which is one. No argument a facade call can pass moves
  // the executable. The resolver's own arms are driven in `install.rs`'s unit
  // tests, where the executable path is a parameter rather than a fact about
  // the process.
  "Install",
  // **PROVOKED AND READ END TO END, ONE CRATE OVER** (hv, 2026-09-12: silent
  // deletion). `intent-cli/tests/no_removal_is_unannounced.rs` realises a
  // thread, edits a view, runs `st hydrate`, and asserts the refusal NAMES the
  // path it would have written over -- which is this variant's whole text --
  // then runs `--overwrite` and asserts the path is named before the write.
  // Provoking it here would need a realised estate with a divergent view, which
  // is the CLI fixture's shape rather than this file's.
  "HydrationWouldOverwrite",
  // Its sibling, and covered in the same file and for the same reason: the
  // arm plants a view-shaped file the store does not carry under a realised
  // thread, runs `st hydrate` and `edit --path`, and asserts both refuse and
  // leave it. The state needs a realised estate with a stray file in it.
  "RealisationWouldRemove",
  // `RootFile` reaches the facade through `agents_generate` alone, and only
  // when the INSTALL is damaged -- a template under `lib/templates/` unreadable
  // or malformed -- which is a property of the world, not of the call. That
  // the type carries a remedy at all is held by `remedy_coverage.rs`; the
  // texts themselves are read by no test yet, which is stated rather than
  // implied by a citation that could not go red.
  "RootFile",
  // `Canon` reaches the facade through `claude_upgrade` alone (0351), and only
  // when the WORLD refuses: a template in the install unreadable, or a project
  // path unwritable. Like `RootFile`, a property of the filesystem rather than
  // of the call.
  "Canon",
  // **DECLARED BY ic, NOT BY ITS AUTHOR, AND THEY SHOULD OVERTURN IT IF IT IS
  // WRONG.** The variant landed in HEAD without this arm, so the workspace did
  // not compile for any node; the arm is mechanical and the exemption is the
  // one judgement in it. `Realise` wraps a realisation failure -- a filesystem
  // that would not write -- which is the broken-world class this list is for.
  // If it is reachable by a bad CALL, it belongs provoked instead.
  "Realise",
  // Needs a TREE in a particular state rather than a bad call -- a hand-edited
  // view, a tree that moved mid-apply, an attachment divergence, or an unmet
  // ship precondition. All four are driven where the state can be built:
  // `organize_dehydration_gate.rs`, `organize_moment_of_act_digest.rs`,
  // `organize_attachment_divergence.rs` and `dehydration_ship_gate.rs`.
  "Organize",
  "Write",           // an unwritable directory -- `write_set_rollback.rs`
  "ViewsNotWritten", // the same, after the DB has committed
  "Store",           // a damaged SQLite file
  "Ingest",          // schema-invalid canon -- `ingest_refusal.rs`
  "Unmigrated",      // an older store -- `unmigrated_project.rs`
  // **BOTH HALVES OF ISSUE 0131's REFUSAL, AND `ThreadExists` HAS MOVED HERE
  // FROM BEING UNREACHABLE.** Its old exemption read "needs a colliding id,
  // which `st new` allocates around", which was true of the pre-check it was
  // raised from -- a test against the same canon `next_thread_id()` had just
  // read, false by construction. Since 0131 the collision is detected by the
  // UNIQUE constraint inside the write, so BOTH are reachable, and both are
  // provoked in `a_create_refuses_a_key_that_is_taken.rs` by opening two
  // facades on one on-disk store before either writes. Not provoked HERE
  // because that needs two facades and a shared store rather than a bad call.
  "ThreadExists",
  "IssueExists",
  // A renumber's disk move refused by the filesystem, a property of the world
  // rather than of the call. Driven in `renumber_moves_an_id_and_what_names_it.rs`,
  // which makes the threads directory unwritable and asserts the refusal, its
  // remedy, and that nothing was renumbered.
  "RenumberDiskStep",
  // `sync --apply`'s repair of a merge (ST0078 WP-05). A twice-minted id with
  // no merge in progress needs an add/add left by a stopped rebase or
  // cherry-pick; a disk step refused and git itself failing are properties of
  // the world. The merge the repair exists for is driven in
  // `a_pull_is_repaired_by_one_command.rs`.
  "RenumberNotMerging",
  "SyncDiskStep",
  "Git",
  "BadQuery", // FTS5 syntax -- `facade_search.rs` territory
  // Issue 0443: a store fault met while ANSWERING a search. Needs the store
  // broken underneath a live facade, which a bad call cannot do; provoked in
  // `a_store_fault_is_not_a_bad_query.rs`.
  "SearchUnanswerable",
  // Needs a projection that LIES -- a format claiming to round-trip and
  // dropping data. Only `export::project_with` can be handed one, and
  // `export_round_trip.rs` does exactly that; a call through the facade cannot
  // reach it, because every format the roster carries is honest.
  "ExportRoundTripFailed",
  // Needs the DISK and the MODEL to disagree -- a canon file carrying authored
  // prose that the open facade's store has never ingested -- which is a state
  // built by editing a file between two calls, not by any argument this file
  // can pass. Driven in `write_refuses_to_empty_an_authored_body.rs`, which
  // builds exactly that gap and asserts the refusal, its byte count, its
  // remedy, and both controls. **The citation goes red if that file stops
  // provoking it**, which is the only thing that makes an exemption a cover.
  "WriteWouldEmptyAnAuthoredBody",
  // The same class as the one above: canon on disk that moved after the open
  // facade's store was warmed, which is a file edit between two calls. Driven
  // in `a_stale_store_does_not_overwrite_committed_canon.rs`, which asserts the
  // committed correction survives and the refusal names its thread.
  "EgestFromStaleStore",
  // Needs another connection to commit inside each of an ingest pass's three
  // renders, which is the world moving under the call rather than an argument
  // this file can pass. The guard it retries is driven in
  // `a_sync_writes_no_render_the_store_has_moved_past.rs`, where a second
  // facade's edit lands between the snapshot and the file commit.
  "IngestOutpacedByWrites",
  // Both need a v2 estate rather than a bad call, and both are the migration
  // door rather than a verb: `MigrationBlocked` needs live-thread residue and
  // `MigrationHalted` needs the filesystem to fail PART WAY THROUGH an
  // `upgrade`, after the writes have committed. Neither is reachable from a
  // facade a test can open, because `Facade::upgrade` exists precisely because
  // there is no `Facade` to be had until it has run.
  "MigrationBlocked",
  "MigrationHalted",
  // Needs a project DECLARING a sub-floor version, which is a property of the
  // world rather than of the call, and reachable only through the migration
  // door for the same reason as the two above. Driven end to end in
  // `intent-cli/tests/upgrade_command.rs`, with the same estate AT the floor as
  // the control -- a refusal arm alone passes against a migrator that refuses
  // everything, which is the mirror of the defect this variant closes.
  "BelowMigrationFloor",
  // **BOTH ARE `intent edit`'s, AND BOTH ARE ASSERTED IN
  // `edit_prints_a_path_that_exists.rs` RATHER THAN MERELY REACHED THERE.**
  // `NotEditable` is matched for its `author_with`, and the two generated views
  // are required to name DIFFERENT surfaces -- so a generic refusal reddens it.
  // `NoSuchEditable` is matched for a non-empty `present`, so a refusal that
  // named only what is missing reddens it too. **A citation that cannot go red
  // is not a cover**, which is why what each one asserts is written here and
  // not just the filename.
  //
  // They are cited rather than provoked because both need a REALISED artefact
  // to refuse about -- `edit` hydrates before it decides -- and this file's
  // `provoked_errors` builds errors from calls that fail, not from calls that
  // succeed at their first step and refuse at their second.
  "NotEditable",
  "NoSuchEditable",
  // **THE SQL DOOR'S OTHER FOUR, ASSERTED IN
  // `intent-cli/tests/the_sql_door_is_read_only.rs` AND NOT MERELY REACHED
  // THERE.** They are cited rather than provoked because each needs a store ON
  // DISK to refuse about -- the door opens a second, read-only connection by
  // path, and this file's fixtures are in memory.
  //
  // What each cover asserts, so a reader can tell whether it could go red:
  // `SqlWouldWrite` -- five write shapes are refused AND the thread count is
  // read before and after, so a refusal that happened for another reason fails
  // it. `SqlOutOfReach` -- the refusal must NAME `ATTACH` or `PRAGMA`, and the
  // attached file must not exist afterwards. `SqlTimedOut` -- a recursive CTE
  // that never finishes is stopped, and the message must be the bound's rather
  // than any other refusal. `SqlDidNotRun` -- a statement naming no table
  // carries SQLite's own words out to the operator.
  "SqlWouldWrite",
  "SqlOutOfReach",
  "SqlOverBudget",
  "SqlDidNotRun",
];

#[test]
fn every_variant_is_provoked_or_declared_elsewhere() {
  let covered: std::collections::BTreeSet<&str> =
    provoked_errors().iter().map(|(_, e)| variant(e)).collect();

  let missing: Vec<&&str> = ALL_VARIANTS
    .iter()
    .filter(|v| !covered.contains(**v) && !NOT_PROVOKED_HERE.contains(*v))
    .collect();
  assert!(
    missing.is_empty(),
    "these variants are neither provoked here nor declared as covered elsewhere: {missing:?} -- \
     the module doc says this file checks the whole variant set, so a variant with no assertion \
     anywhere makes that claim false"
  );

  // The mirror: a name in the exemption list that IS provoked means the
  // exemption has gone stale and is now hiding a variant rather than
  // explaining one.
  let stale: Vec<&&str> = NOT_PROVOKED_HERE
    .iter()
    .filter(|v| covered.contains(**v))
    .collect();
  assert!(
    stale.is_empty(),
    "these are declared unreachable here and were provoked anyway: {stale:?}"
  );
}

#[test]
fn every_error_renders_a_message_and_a_remedy() {
  for (label, err) in provoked_errors() {
    let rendered = err.render();
    assert!(
      rendered.starts_with("error: "),
      "{label}: the rendering leads with the lowercase voice (0023): {rendered}"
    );
    assert!(
      rendered.contains("\n  remedy: "),
      "{label}: every error tells the operator what to DO: {rendered}"
    );
    assert!(
      !err.remedy().is_empty(),
      "{label}: an empty remedy is a remedy-shaped hole"
    );
  }
}

/// The anti-collapse assertion, checked pairwise over the whole set.
#[test]
fn no_two_distinct_causes_render_the_same_text() {
  let errors = provoked_errors();
  for (i, (label_a, a)) in errors.iter().enumerate() {
    for (label_b, b) in errors.iter().skip(i + 1) {
      assert_ne!(
        a.render(),
        b.render(),
        "'{label_a}' and '{label_b}' render identically -- an operator hitting either one cannot tell which they hit"
      );
      assert_ne!(
        a.to_string(),
        b.to_string(),
        "'{label_a}' and '{label_b}' share a message"
      );
      assert_ne!(
        a.remedy(),
        b.remedy(),
        "'{label_a}' and '{label_b}' share a remedy -- a remedy that fits two causes tells the operator to guess"
      );
    }
  }
}

/// Each message names the specific artefact, not just its kind. "no such
/// thread" is a category; "no steel thread ST9999" is an answer.
#[test]
fn every_message_names_the_artefact_it_is_about() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let cases: Vec<(FacadeError, &str)> = vec![
    (facade.st_show("ST9999").unwrap_err(), "ST9999"),
    (facade.wp_start("ST0056", 42).unwrap_err(), "WP-42"),
    (
      facade.ac_satisfy("ST0056", "AC-77.7", "x").unwrap_err(),
      "AC-77.7",
    ),
    (
      facade
        .at_set("ST0056", "AT-77.7", AtStatus::Green, None)
        .unwrap_err(),
      "AT-77.7",
    ),
  ];
  for (err, needle) in cases {
    assert!(
      err.to_string().contains(needle),
      "the message names {needle}, got: {err}"
    );
  }
}

/// A wrapped error keeps its cause chain. Collapsing to the outermost sentence
/// is what made two different problems print the same line in v2.
#[cfg(unix)]
#[test]
fn a_wrapped_failure_renders_its_full_cause_chain() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade
    .st_hold("ST0056", "waiting on the fleet")
    .expect("a legal mutation from wip");

  let mode = fx.make_readonly("intent");
  let result = facade.st_cancel("ST0056", "superseded by the v3 line");
  fx.restore_mode("intent", mode);

  // Issue 0376: the write landed, so the verb reports it with a note rather
  // than refusing (vc, ruled 2026-09-14), and the note carries what this
  // error carried: the message, its whole chain, and the one remedy.
  let outcome = result.expect("the write landed, so the verb reports it rather than refusing");
  let (_, cause, caused_by, remedy) = crate::common::landed_note(outcome.notes());
  let rendered = format!("{cause}\n{}\n{remedy}", caused_by.join("\n"));
  assert!(
    !caused_by.is_empty(),
    "the underlying I/O failure is reported, not swallowed by the outer message: {rendered}"
  );
  assert!(
    rendered.contains("todo.md"),
    "the chain names the file that actually failed: {rendered}"
  );
  // D01 REVERSED (hv, 2026-08-15): this used to assert "nothing was changed",
  // and that is now FALSE. The DB is the SSOT and it is written first, so by
  // the time a file write fails the mutation IS recorded -- what failed is the
  // projection of it onto disk.
  //
  // The new assertion is the stronger one, because the hazard inverted with
  // the model. Under the old order the operator's risk was believing a change
  // had landed when it had not; under the new one it is RETRYING a change that
  // already landed. So the text must lead with what succeeded and say plainly
  // not to repeat it -- a remedy that merely described the I/O error would be
  // accurate and would still get the estate mutated twice.
  assert!(
    rendered.contains("the change is recorded"),
    "the message leads with what SUCCEEDED, so the operator does not read a projection failure as a failed mutation: {rendered}"
  );
  assert!(
    rendered.contains("do NOT retry"),
    "the remedy names the actual hazard under D01-as-reversed, which is a second application of a change that already landed: {rendered}"
  );
  // **This remedy has now been edited twice, for two different reasons, and
  // the pair is the point.** The first draft told the operator to run `intent
  // sync` -- disk -> db, which would have destroyed the change this error
  // calls safe -- and was fixed by warning them OFF it. That warning was then
  // the entire remedy for exactly as long as there was no db -> disk direction
  // to point AT, and AC-03.9 landed one the same day. So a remedy that only
  // said "do not" went from honest to under-serving without anybody touching
  // it: the same class as the first edit, arriving from the opposite side.
  //
  // The assertions therefore check the two surviving PROPERTIES rather than
  // the sentence, because the sentence has already moved twice.
  assert!(
    !rendered.contains("run `intent sync`"),
    "the remedy must never RECOMMEND the disk -> db direction -- that is the data-loss instruction this assertion exists to keep out, and it was once here: {rendered}"
  );
  assert!(
    rendered.contains("disk -> db") && rendered.contains("Do NOT reach"),
    "it still warns off that direction by name: {rendered}"
  );
  assert!(
    rendered.contains("intent st sync"),
    "and it names the repair that EXISTS, rather than telling the operator to wait for the next mutation as it did before AC-03.9: {rendered}"
  );
}

/// A board write whose views did not land names the door that lands a BOARD,
/// and names the two that do not (0487).
///
/// **THE TWO NEGATIVE ASSERTIONS ARE THE ARM.** The positive one -- that the
/// text says `wb touch` -- would pass over the old remedy the day someone
/// appended a sentence to it. What this exists to catch is the remedy sending
/// a reader to a command that reports a clean run and leaves the board stale,
/// which is what `intent st sync` did here for as long as this note existed,
/// and what `intent organize` would do if the issue's own ask had been built
/// as written: `.intentfiles` carries no whiteboard row and `organize.rs` no
/// board code, so neither renders a board.
///
/// **WHAT IS NOT DRIVEN HERE:** the failure is injected as a read-only
/// directory rather than as the contended store the defect was seen under
/// (Lamplight, 2026-09-19, four peers running `pickup`). Both reach this note
/// through the same arm of `land_board_write_noting`, and a lock is the
/// expensive one to stage; the remedy TEXT is what this arm is about. The
/// locked case was driven by hand against a real contended store.
#[cfg(unix)]
#[test]
fn a_board_whose_views_did_not_land_names_the_board_door() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  facade
    .wb_register("cc", "Control Claude", "control")
    .expect("register the node");

  // **THE NODE'S OWN DIRECTORY, NOT THE WHITEBOARD ROOT.** `WriteSet` writes
  // through a SIBLING temp file and a rename, so the directory that has to
  // refuse is the one holding the target. A read-only `intent/whiteboard` was
  // the first attempt here and it injected nothing at all: `cc/` already
  // exists by then, the registration having rendered it, and a read-only
  // parent does not stop a write inside a writable child. The arm passed
  // vacuously -- no note, and `landed_note` panicking is what caught it.
  let mode = fx.make_readonly("intent/whiteboard/cc");
  let landed = facade.wb_touch("cc");
  fx.restore_mode("intent/whiteboard/cc", mode);

  // The row is committed before the views are landed (vc, ruled 2026-09-14),
  // so the verb reports rather than refusing -- the same shape as the thread
  // path above.
  landed.expect("the row landed, so the verb reports the view failure as a note");
  let (step, cause, _caused_by, remedy) = crate::common::landed_note(&facade.take_notes());
  let rendered = format!("{step}\n{cause}\n{remedy}");
  assert!(
    step.contains("landing the board's views"),
    "the step names what failed, so the note is about the board and not a thread: {rendered}"
  );
  assert!(
    remedy.contains("intent wb touch --node"),
    "it names the door that actually lands a board view -- a board write, and nothing else: {rendered}"
  );
  // **TESTED AS "DOES NOT RECOMMEND", NOT AS "DOES NOT MENTION", and the
  // first draft of this arm got that wrong and was red for it.** The remedy
  // names both wrong doors ON PURPOSE, because a reader who has read decision
  // 16 reaches for organize and a reader of the old text reaches for `st
  // sync`; warning them off by name is the whole point. So the property is
  // that the only command it tells anyone to RUN is the board write -- the
  // same shape the thread arm above uses to keep `intent sync` out of its own
  // remedy while still naming it.
  assert!(
    !remedy.contains("run `intent st sync`"),
    "it must not SEND anyone to the thread door, which rewrites a thread's views, reports success, and leaves the board exactly as stale as it found it: {rendered}"
  );
  assert!(
    !remedy.contains("run `intent organize`"),
    "nor to organize, for the same reason: no whiteboard row in .intentfiles and no board code in organize.rs, so it renders no board: {rendered}"
  );
  assert!(
    remedy.contains("NOT `intent organize`") && remedy.contains("NOT `intent st sync`"),
    "and it warns off both by name rather than leaving them looking untested: {rendered}"
  );
  assert!(
    remedy.contains("safe in the store"),
    "it leads with what SUCCEEDED, so the row is not written twice: {rendered}"
  );
}

/// The gate's refusal carries the gate's own verdict line, so the operator
/// sees WHICH criteria blocked rather than being told to go and look.
#[test]
fn the_gate_refusal_carries_the_verdict_line() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  facade
    .at_set("ST0056", "AT-03.1", AtStatus::Red, None)
    .unwrap();
  facade
    .at_set("ST0056", "AT-03.7", AtStatus::Red, None)
    .unwrap();

  let err = facade.st_done("ST0056").expect_err("blocked");
  let rendered = err.render();
  assert!(rendered.contains("gate: ST0056 BLOCKED"), "{rendered}");
  assert!(rendered.contains("AC-03.1"), "{rendered}");
}
