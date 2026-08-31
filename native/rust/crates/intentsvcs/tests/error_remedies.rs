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

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::facade::{FacadeError, ListEdit};
use intentsvcs::model::{AcKind, AtKind, AtStatus};
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
  out.push((
    "unknown schema face",
    facade
      .schema(Some("not-a-face"))
      .expect_err("a face the types do not generate is refused by name"),
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
        AtStatus::ToWrite,
        None,
      )
      .expect_err("a create must not replace"),
  ));
  out.push((
    "an edit naming no field",
    facade
      .at_edit("ST0056", "AT-03.1", None, None, None)
      .expect_err("an edit with nothing to change is refused, not reported unchanged"),
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
  emptied.write_issue(&common::sample_issue(21));
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
    FacadeError::NoSuchThread { .. } => "NoSuchThread",
    FacadeError::ThreadExists { .. } => "ThreadExists",
    FacadeError::IssueExists { .. } => "IssueExists",
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
    FacadeError::NoSuchFace { .. } => "NoSuchFace",
    FacadeError::IllegalTransition { .. } => "IllegalTransition",
    FacadeError::ReasonRequired { .. } => "ReasonRequired",
    FacadeError::EvidenceRequired { .. } => "EvidenceRequired",
    FacadeError::DescopeTargetMissing { .. } => "DescopeTargetMissing",
    FacadeError::DescopeTargetRequired { .. } => "DescopeTargetRequired",
    FacadeError::Unmigrated(_) => "Unmigrated",
    FacadeError::BelowMigrationFloor(_) => "BelowMigrationFloor",
    FacadeError::Write(_) => "Write",
    FacadeError::ViewsNotWritten { .. } => "ViewsNotWritten",
    FacadeError::Store(_) => "Store",
    FacadeError::Ingest(_) => "Ingest",
    FacadeError::NoSuchFormat { .. } => "NoSuchFormat",
    FacadeError::LossyFormat { .. } => "LossyFormat",
    FacadeError::ExportRoundTripFailed { .. } => "ExportRoundTripFailed",
    FacadeError::NoSuchIssue { .. } => "NoSuchIssue",
    FacadeError::MigrationBlocked(_) => "MigrationBlocked",
    FacadeError::MigrationHalted { .. } => "MigrationHalted",
    FacadeError::EgestFromRefusedIngest { .. } => "EgestFromRefusedIngest",
    FacadeError::EgestWouldEmptyTheEstate { .. } => "EgestWouldEmptyTheEstate",
    FacadeError::WriteWouldEmptyAnAuthoredBody { .. } => "WriteWouldEmptyAnAuthoredBody",
    FacadeError::Realise(_) => "Realise",
    FacadeError::Organize(_) => "Organize",
    FacadeError::Intentfiles(_) => "Intentfiles",
    FacadeError::ManifestUnreadable { .. } => "ManifestUnreadable",
    FacadeError::ManifestMalformed { .. } => "ManifestMalformed",
    FacadeError::NotHydratable { .. } => "NotHydratable",
    FacadeError::NoManifestToUnlistFrom { .. } => "NoManifestToUnlistFrom",
    FacadeError::DehydrationRefused { .. } => "DehydrationRefused",
    FacadeError::NotEditable { .. } => "NotEditable",
    FacadeError::NoSuchEditable { .. } => "NoSuchEditable",
    FacadeError::FieldNotWritable { .. } => "FieldNotWritable",
    FacadeError::ValueNotRecordable { .. } => "ValueNotRecordable",
    FacadeError::Install(_) => "Install",
    FacadeError::RootFile(_) => "RootFile",
  }
}

/// The variants a reader of this file should expect to see provoked.
///
/// Some are reachable only through a failing filesystem or a damaged store, and
/// those are declared here as deliberately-not-provoked rather than left to
/// look like oversights -- an exemption that is announced, never inferred
/// (ST0048's rule).
const ALL_VARIANTS: &[&str] = &[
  "ValueNotRecordable",
  "NotHydratable",
  "NoManifestToUnlistFrom",
  "DehydrationRefused",
  "NotEditable",
  "NoSuchEditable",
  "Organize",
  "Intentfiles",
  "ManifestUnreadable",
  "ManifestMalformed",
  "NoSuchThread",
  "ThreadExists",
  "IssueExists",
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
  "NoSuchFace",
  "IllegalTransition",
  "ReasonRequired",
  "EvidenceRequired",
  "DescopeTargetMissing",
  "DescopeTargetRequired",
  "Unmigrated",
  "BelowMigrationFloor",
  "Write",
  "ViewsNotWritten",
  "Store",
  "Ingest",
  "NoSuchFormat",
  "LossyFormat",
  "WriteNotAddressable", // PUT to a server-assigned id -- `mutation_create_splits_two_ways.rs`
  "ExportRoundTripFailed",
  "NoSuchIssue",
  "MigrationBlocked",
  "MigrationHalted",
  "EgestFromRefusedIngest",
  "EgestWouldEmptyTheEstate",
  "WriteWouldEmptyAnAuthoredBody",
  "Realise",
  "Install",
  "RootFile",
];

/// Variants that need a broken world rather than a bad call, and are covered by
/// the tests that break that world instead.
const NOT_PROVOKED_HERE: &[&str] = &[
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
  // `RootFile` reaches the facade through `agents_generate` alone, and only
  // when the INSTALL is damaged -- a template under `lib/templates/` unreadable
  // or malformed -- which is a property of the world, not of the call. That
  // the type carries a remedy at all is held by `remedy_coverage.rs`; the
  // texts themselves are read by no test yet, which is stated rather than
  // implied by a citation that could not go red.
  "RootFile",
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
  "BadQuery", // FTS5 syntax -- `facade_search.rs` territory
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

  let err = result.expect_err("the write must fail");
  let rendered = err.render();
  assert!(
    rendered.contains("caused by:"),
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
