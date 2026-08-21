//! THE declared state-transition graph: every closed-domain field in the
//! model, and the service verbs that move an entity between its values
//! (D32, AC-04.6).
//!
//! **The rule this exists to make checkable.** hv ruled that a state an entity
//! can enter and cannot leave is a missing mutation, not a missing flag. That
//! is a property of a graph, so it is only checkable if the graph is written
//! down: Rust has no reflection, nothing can enumerate [`Facade`]'s methods at
//! runtime, and no test can discover the edges by itself.
//!
//! **What stops this table going stale.** It is the ANSWER; the QUESTION comes
//! from the committed JSON Schema, which is generated from the model types and
//! drift-checked. `mutation_completeness.rs` walks that schema for every field
//! with a closed value domain and REFUSES any field this table does not
//! classify -- so a state field added to the model fails on the day it is
//! added, with nobody having to remember this file exists. It is the
//! `deny_unknown_fields` posture (D05) applied to the transition graph, and it
//! is the reason the table binds D30's whiteboard entities in advance: when
//! they enter the model their state fields enter the schema, and the schema is
//! what drives the check.
//!
//! **What stops a row lying.** Every edge names the event-log verb that
//! performs it -- the same string [`Facade::apply`] records -- and the test
//! EXECUTES each edge against a fixture. A declared edge whose mutation does
//! not exist fails; a mutation deleted from the facade takes its test arm with
//! it. The residual direction is safe: an UNDECLARED mutation can only add an
//! edge, and adding edges can only make a graph more closed, so the failure it
//! causes is a false alarm rather than a false pass.
//!
//! [`Facade`]: crate::facade::Facade
//! [`Facade::apply`]: crate::facade::Facade

/// Whether an edge is a way out of a value IN ITS OWN RIGHT, or a side effect
/// of a verb whose subject is a different field.
///
/// **This distinction was found by mutation-testing AT-04.6 and it is the
/// difference between the rule and a technicality.** With scope changes
/// clearing satisfaction, deleting `ac.unsatisfy` still left `satisfied: true`
/// formally leavable -- via descope-then-rescope. That exit is real and
/// useless: to withdraw a claim of evidence you would have to move the
/// requirement to another thread and bring it back, which is the hand-edit
/// problem in a new costume and records two false facts on the way. So an
/// incidental edge counts for REACHABILITY (it genuinely produces the value)
/// and never discharges a TRAP.
///
/// **It has NO current user, and it stays -- which reverses what cc's board
/// said, on evidence that arrived after.** The defect it was built for is gone:
/// the AC collapse left one field where there were two, so no verb moves a
/// second field as a side effect and nothing constructs an `Incidental` edge.
/// The board's rule was "delete it unless a non-AC user appears", and none did.
///
/// What changed the answer is what the variant actually protects.
/// [`Edge::exits`] is `leaves() && kind == Direct`, so WITHOUT this variant
/// `exits` collapses into `leaves` and the trap check silently starts
/// accepting technicality exits again -- for whatever field-crossing verb
/// arrives next, with nobody present to notice the property was dropped. The
/// cost of keeping it is one variant and one test; the cost of deleting it is a
/// subtle property that took mutation testing to find in the first place.
/// Recorded rather than quietly kept, because "unused" is the correct reading
/// of the code and the wrong reading of the design.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
  /// The verb exists to move this field.
  Direct,
  /// The verb's subject is `via`; this field moves as a consequence.
  Incidental { via: &'static str },
}

/// A precondition the verb enforces BEFORE taking the edge.
///
/// Declared rather than left to the implementation because a guard is the half
/// of a transition a success-path test never sees: an edge that lands on the
/// right value from the right state proves nothing about what happens when the
/// precondition is unmet, and "unmet" is the case the guard exists for.
///
/// **A verb can have more than one, and the list is the reason this is a slice
/// rather than a value.** `ac.satisfy` is both "non-test criteria only" and
/// "evidence recorded", and while the guard was single-valued the second one
/// could not be written down -- so it was not written down, and then it was not
/// enforced. An absent precondition is the empty list; there is no `None`
/// value, because two spellings of "no guard" is the three-representations-for
/// -two-meanings defect this module already pruned from `satisfied: false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guard {
  /// The verb takes a reason, records it on the entity (`status_reason`, or the
  /// state's own `reason` for a criterion), and puts it in the event envelope.
  /// Refuses an empty or blank one.
  ReasonRecorded,
  /// The close gate must PASS.
  GatePass,
  /// The verb applies only to a `(non-test)` criterion. A test-backed one has
  /// no stored satisfaction to write.
  NonTestOnly,
  /// The thread the requirement is being moved to must exist.
  TargetExists,
  /// **The verb records evidence, and refuses an empty or blank one.**
  ///
  /// Its own guard rather than a second use of [`Guard::ReasonRecorded`],
  /// because the two answer different questions and the refusal has to say
  /// which. A reason explains a decision the operator took; evidence is the
  /// substitute for a test result on a criterion that has none -- `contract.rs`
  /// puts it as "a human judgement with no green to read". Telling someone
  /// satisfying a criterion that they owe a "reason" points them at the wrong
  /// thing.
  EvidenceRecorded,
}

/// One step the service layer offers on one field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
  /// The event-log op, eg `ac.satisfy`, so an edge is traceable from the log
  /// back to the declaration that promised it.
  pub verb: &'static str,
  /// Source values. **Empty means "any current value"**, which is now a
  /// deliberate exception rather than the norm: the ratified machines
  /// (data-model.md) name specific from-states, and an empty list survives only
  /// where the field genuinely has no ordering -- `at.set` and `wp.rescope`,
  /// where any value legitimately reaches any other.
  ///
  /// **It used to be on every edge, and that is the defect conformance found.**
  /// A graph declared with `from: &[]` everywhere is CLOSED -- nothing can be a
  /// trap when every verb accepts every state -- so the closure check passed on
  /// a graph that let `st done` fire from `cancelled`. Closure is the weaker
  /// question; a graph can be closed and still be the wrong graph.
  pub from: &'static [&'static str],
  pub to: &'static str,
  pub kind: EdgeKind,
  pub guard: &'static [Guard],
}

impl Edge {
  pub const fn direct(verb: &'static str, from: &'static [&'static str], to: &'static str) -> Self {
    Self {
      verb,
      from,
      to,
      kind: EdgeKind::Direct,
      guard: &[],
    }
  }

  pub const fn guarded(
    verb: &'static str,
    from: &'static [&'static str],
    to: &'static str,
    guard: &'static [Guard],
  ) -> Self {
    Self {
      verb,
      from,
      to,
      kind: EdgeKind::Direct,
      guard,
    }
  }

  pub const fn incidental(
    verb: &'static str,
    from: &'static [&'static str],
    to: &'static str,
    via: &'static str,
  ) -> Self {
    Self {
      verb,
      from,
      to,
      kind: EdgeKind::Incidental { via },
      guard: &[],
    }
  }

  /// Whether this edge can be taken from `value`.
  pub fn accepts(&self, value: &str) -> bool {
    self.from.is_empty() || self.from.contains(&value)
  }

  /// Whether taking it from `value` actually moves the field. A verb that
  /// accepts a value and lands on it again is not a way out of it.
  pub fn leaves(&self, value: &str) -> bool {
    self.accepts(value) && self.to != value
  }

  /// Whether it is an EXIT: it moves the field and it is the verb's own
  /// subject.
  pub fn exits(&self, value: &str) -> bool {
    self.leaves(value) && self.kind == EdgeKind::Direct
  }
}

/// What the service layer owes a closed-domain field.
#[derive(Debug, Clone, Copy)]
pub enum Disposition {
  /// A lifecycle state: the service layer must be able to enter it and leave
  /// it.
  State {
    /// Values an entity can be created in.
    initial: &'static [&'static str],
    edges: &'static [Edge],
    /// Values with no inbound edge, each with the reason. **Declared rather
    /// than tolerated**: the set is asserted exactly, so a NEW unreachable
    /// value fails while these known ones stay recorded with their evidence.
    orphans: &'static [(&'static str, &'static str)],
  },
  /// No service verb touches this field yet. The variant IS the count, so the
  /// debt is visible rather than hidden -- the same posture the dispatch-table
  /// guard takes for an unbuilt command.
  ///
  /// An `Unbuilt` field must declare NO edges, so the day a mutation lands the
  /// disposition is contradicted and the test says so.
  ///
  /// **"Carries no edges" is the WEAKER question, and `entry` is the other
  /// half.** Edges are the exits, not the entrances: a field with no exits is
  /// harmless only if nothing can put a value into it. So the variant carries
  /// how a value gets IN, and that half is measured by driving canon rather
  /// than asserted here.
  ///
  /// **It carried an `owed_by: "WP-06"` beside the note until D37, and dropping
  /// it is not cosmetic.** `intentsvcs` is a library another project can depend
  /// on, so a field whose job is to name which of OUR work packages will build
  /// something is Intent's roadmap sitting one `println!` away from a
  /// stranger's terminal -- which is the exact structural leak AC-00.9 was
  /// written from. Which work package owes this belongs in the contract that
  /// tracks it; what a reader needs is `note`, which says what is unavailable.
  Unbuilt { note: &'static str, entry: Entry },
  /// A value fixed when the entity is authored and moved by no SERVICE VERB
  /// afterwards, so no mutation is owed and the absence of edges is the finished
  /// answer rather than a gap. Authoring the file and re-ingesting is how it
  /// changes, which is the ruling's point and not a loophole in it.
  ///
  /// **It is NOT called `Attribute`, and the near-miss is worth the sentence.**
  /// data-model.md calls this row "an attribute of a thread, not a lifecycle",
  /// which is what the variant is for -- but the SAME document calls
  /// `WorkPackage.scope` "an attribute of the package", and scope is a `State`
  /// whose values `wp rescope` moves freely. So "attribute" names two
  /// dispositions in the ratified vocabulary, and a variant carrying that word
  /// would invite exactly one wrong edit: moving scope here because the document
  /// appears to say so. The phrase that picks out this row and only this row is
  /// **"immutable after creation"**, from the same ruling.
  ///
  /// Quoted rather than cited by line: the ruling lives under data-model.md's
  /// "The four `Unbuilt` fields" heading, the document is vc's and under active
  /// edit, and a line number in a peer's live file is a promise that goes stale
  /// without anybody touching this one.
  ///
  /// **It exists to stop `Unbuilt` meaning two things.** `Unbuilt` is a DEBT:
  /// the variant is the count, and the day the verb lands the row becomes a
  /// `State`. A field nothing will ever move is not a smaller debt, it is a
  /// different claim, and filing it as `Unbuilt` reports a build that is never
  /// coming -- which is a permanent red that a reader eventually learns to
  /// discount.
  ///
  /// **`ruled` is what makes reclassification cost something, and without it
  /// this variant is an escape hatch**: any red `Unbuilt` row could be
  /// relabelled here and the red would go away. A row may only claim nothing is
  /// owed by citing who ruled it, in the `<node> <YYYY-MM-DD>` form this file
  /// already uses for Machine 4. It is provenance for a reviewer and is never
  /// printed -- `intent-cli`'s `no_pm_state_in_output.rs` is what holds that,
  /// since it drives the binary rather than reading this table.
  Immutable {
    note: &'static str,
    entry: Entry,
    ruled: &'static str,
  },
}

/// How a value reaches a field no service verb writes.
///
/// **The distinction exists because the two cases want opposite conclusions.**
/// A field nothing can enter is genuinely absent, and the absence is the whole
/// story. A field authored canon CAN enter, with no verb to move it, is an
/// entity sitting in a value nothing can change -- which is a trap wearing an
/// absence's label, and the mutation is owed rather than the debt merely
/// declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
  /// Nothing outside the service layer can put a value here either, so there
  /// is no entity holding an unleaveable value and nothing is owed.
  Inert,
  /// **Authored canon puts a value here and no verb can move it.** Measured by
  /// driving ingest, never assumed: `mutation_completeness.rs` writes canon
  /// carrying a non-initial value, reads it back through the facade, and
  /// requires the measurement and this declaration to agree in BOTH
  /// directions. A row claiming `Inert` that canon can populate fails, and so
  /// does one claiming `Authored` after the entry path closes.
  Authored,
}

/// One closed-domain field of one entity. `entity` is the JSON Schema
/// definition name, so the test can join this table to the schema walk without
/// a translation layer that could itself drift.
pub struct Field {
  pub entity: &'static str,
  pub field: &'static str,
  pub disposition: Disposition,
}

/// The value a `bool` field takes when the key is absent. Named because
/// `satisfied` is `Option<bool>` and the absent case is a real value of the
/// domain, not a gap in it.
pub const ABSENT: &str = "absent";

pub const FIELDS: &[Field] = &[
  Field {
    entity: "Thread",
    field: "status",
    disposition: Disposition::State {
      // Entry is `triage`, ratified. `st new` used to land on `not-started`.
      initial: &["triage"],
      // **The ratified machine, transcribed** (data-model.md "Machine 1").
      // `mutation_completeness.rs` holds a SECOND transcription of the same
      // table and asserts the two agree, so a typo in either is a failure
      // rather than a quiet divergence from the document both come from.
      edges: &[
        Edge::direct("st.triage", &["triage"], "not-started"),
        Edge::direct("st.start", &["triage", "not-started"], "wip"),
        Edge::direct("st.resume", &["hold"], "wip"),
        Edge::guarded(
          "st.hold",
          &["not-started", "wip"],
          "hold",
          &[Guard::ReasonRecorded],
        ),
        Edge::guarded("st.done", &["wip"], "completed", &[Guard::GatePass]),
        Edge::guarded(
          "st.cancel",
          &["triage", "not-started", "wip", "hold"],
          "cancelled",
          &[Guard::ReasonRecorded],
        ),
        Edge::guarded("st.reopen", &["completed"], "wip", &[Guard::ReasonRecorded]),
        Edge::guarded(
          "st.reinstate",
          &["cancelled"],
          "not-started",
          &[Guard::ReasonRecorded],
        ),
      ],
      // **Both former orphans are answered by ratification rather than by a
      // build.** `tbc` was a display alias reified into the model and is now
      // `triage`, a real state with a real entry point and a real exit; `hold`
      // was v2 vocabulary reachable only by hand-editing frontmatter and now
      // has `st hold` in and `st resume` out. Neither was a mutation gap and
      // both were recorded here rather than forgotten, which is what the
      // orphan list is for.
      orphans: &[],
    },
  },
  Field {
    entity: "Thread",
    field: "acceptance",
    // **hv ruled it: immutable after creation, no machine, no edge, owed
    // nothing.** `Option<AcceptanceMode>` is an attribute, not a lifecycle, so
    // changing it is AUTHORING -- and the row spent two days as `Unbuilt`
    // reporting a verb that was never coming.
    //
    // **This does not reverse hv's earlier reading of v2, because the two
    // rulings are about different things.** v2's `bin/intent_acceptance:987`
    // tells the user to "add 'acceptance: exempt' to its frontmatter" from
    // inside an ERROR PATH, ie prescribes a hand-edit to work around a verb it
    // is missing. The ruling here is that authoring the field is the interface
    // rather than a workaround. Whether v2's message is still a defect in v2's
    // voice is hv's to say, not this table's.
    disposition: Disposition::Immutable {
      note: "the close-gate exemption, fixed when the thread is authored and moved by nothing afterwards. Three threads in this estate use it",
      // `st_new` hardcodes `None`, so `exempt` is a value only canon supplies.
      // **`Entry::Authored` is the right measurement either side of the
      // reclassification**, because it measures a property -- canon can put a
      // value here -- rather than the classification the ruling changed.
      entry: Entry::Authored,
      ruled: "hv 2026-08-17",
    },
  },
  Field {
    entity: "WorkPackage",
    field: "status",
    disposition: Disposition::State {
      initial: &["not-started"],
      // The ratified machine, transcribed (data-model.md "Machine 2").
      //
      // **`Cancelled` IS AT WP LEVEL AS OF 2026-08-21, AND THE PREVIOUS RULING
      // IS PRESERVED HERE BECAUSE IT WAS REASONED RATHER THAN OVERLOOKED.**
      // Machine 2 proposed no `Hold` or `Cancelled` -- _"a work package that
      // stops mattering is a scope change on the thread, not a state on the
      // package"_ -- and flagged it _"Open for hv if that is wrong"_. Nothing
      // tracked that question, so the default shipped by silence for four
      // months. It was wrong, and a live consumer proved it: scope removed,
      // every AC withdrawn, and `wp done` refused forever because the gate
      // correctly declines to infer an exemption from an emptied contract
      // (ST0048). The only announced exemption was THREAD-scoped, so closing
      // one unit meant discarding the standing of all 37 of its ACs.
      //
      // `Hold` is still not proposed: a paused package is `wip` that nobody is
      // touching, and no consumer has hit the absence.
      //
      // **`wp.reopen` is the verb whose absence was causing live damage.**
      // Adding an AC to a closed WP reopens it in the contract, nothing undid
      // `wp done`, and three of this thread's five WPs were carrying a status
      // that disagreed with their own gate -- two of them written by the
      // verifier enforcing the rule that names the class.
      edges: &[
        Edge::direct("wp.start", &["not-started"], "wip"),
        Edge::direct("wp.unstart", &["wip"], "not-started"),
        Edge::guarded("wp.done", &["wip"], "done", &[Guard::GatePass]),
        Edge::guarded("wp.reopen", &["done"], "wip", &[Guard::ReasonRecorded]),
        // Mirrors `st.cancel`: reachable from every live state, reason required.
        // NOT `Guard::GatePass` -- this verb is the announcement that there is
        // no contract left to gate on, so gating it rebuilds the deadlock.
        Edge::guarded(
          "wp.cancel",
          &["not-started", "wip", "done"],
          "cancelled",
          &[Guard::ReasonRecorded],
        ),
        // Mirrors `st.reinstate`: lands on `not-started`, never on the status
        // held before cancellation -- that value is recorded nowhere, so
        // restoring it would be a guess wearing the authority of a verb.
        Edge::guarded(
          "wp.reinstate",
          &["cancelled"],
          "not-started",
          &[Guard::ReasonRecorded],
        ),
      ],
      orphans: &[],
    },
  },
  Field {
    entity: "WorkPackage",
    field: "scope",
    disposition: Disposition::State {
      // EVERY size is an initial value, because `wp_new` takes the size from
      // the caller. That is what makes this a State rather than an inert
      // attribute, and it is the distinction vc's test drew and this table
      // originally got wrong: a value the caller supplies at creation has been
      // ENTERED, so with no verb able to change it, all six were traps.
      // Neither v2 nor v3 had the verb.
      //
      // **`absent` is the seventh, and it arrives by INGEST rather than by a
      // caller** -- which is precisely the entry route vc's second condition on
      // this AC names: a field that can arrive in a state by ingest has been
      // entered, whether or not any verb puts it there. A v2 work package
      // predating the frontmatter convention has no `scope:` line at all, and
      // the migration now carries that as "nobody recorded one" instead of
      // substituting a medium.
      //
      // It needs no new edge: every `wp.rescope` edge below declares an EMPTY
      // `from` set, so it already leaves any state including this one. Adding
      // it here is declaring the state, not opening an exit.
      initial: &["XS", "S", "M", "L", "XL", "XXL", "absent"],
      edges: &[
        Edge::direct("wp.rescope", &[], "XS"),
        Edge::direct("wp.rescope", &[], "S"),
        Edge::direct("wp.rescope", &[], "M"),
        Edge::direct("wp.rescope", &[], "L"),
        Edge::direct("wp.rescope", &[], "XL"),
        Edge::direct("wp.rescope", &[], "XXL"),
      ],
      orphans: &[],
    },
  },
  Field {
    entity: "Criterion",
    field: "kind",
    disposition: Disposition::Unbuilt {
      note: "test-backed against non-test. Converting one to the other when its test gets written is ordinary workflow and currently needs a hand-edit",
      // The note already names the hand-edit, which IS the trap: `legacy.rs`
      // decides the kind on the way in, so every criterion in the estate holds
      // a value authored canon chose and no verb can revise.
      entry: Entry::Authored,
    },
  },
  Field {
    entity: "Criterion",
    field: "state",
    disposition: Disposition::State {
      // **TWO entry values, and that is the collapse working.** An authored
      // criterion is created `unsatisfied`; a test-backed one is created
      // `computed`, meaning nothing about its satisfaction is stored at all.
      // The two-field model could not express that -- it had `satisfied:
      // Option<bool>` on every criterion and a linter rule keeping it empty on
      // the test-backed ones.
      initial: &["computed", "unsatisfied"],
      // The ratified AC machine (data-model.md "Machine 3"), transcribed.
      //
      // **`ac.rescope` and `ac.reinstate` each declare TWO edges** because
      // where a criterion lands coming back into scope depends on its kind:
      // `AcState::entry(kind)`. That is not a special case bolted on -- it is
      // the same fact as the two initial values, seen from the other side.
      //
      // **No `EdgeKind::Incidental` anywhere.** It existed because one verb
      // moved two fields: a scope change also had to clear `satisfied`, so
      // `ac.descope` declared an edge on a field that was not its subject.
      // With one field there is no second field to move as a side effect, and
      // the mechanism built this morning for the mutation that survived is
      // retired by the structure rather than by a decision.
      edges: &[
        // **TWO guards, and the second one is here because it could not be
        // written down before.** `ac.satisfy` has always been both "non-test
        // only" and "evidence recorded"; with a single-valued guard column only
        // one of them fitted, the one that fitted was enforced, and the other
        // was left to a renderer arm that used `unwrap_or_default()`. ic traced
        // that to an AC recorded Satisfied with empty evidence, printing `ok:`
        // and counting toward the close gate (2026-08-15). **The table could
        // not express the requirement, so nothing was checking that it held.**
        Edge::guarded(
          "ac.satisfy",
          &["unsatisfied"],
          "satisfied",
          &[Guard::NonTestOnly, Guard::EvidenceRecorded],
        ),
        Edge::guarded(
          "ac.unsatisfy",
          &["satisfied"],
          "unsatisfied",
          &[Guard::NonTestOnly],
        ),
        Edge::guarded(
          "ac.descope",
          &["computed", "unsatisfied", "satisfied"],
          "descoped",
          &[Guard::TargetExists],
        ),
        Edge::guarded(
          "ac.withdraw",
          &["computed", "unsatisfied", "satisfied"],
          "withdrawn",
          &[Guard::ReasonRecorded],
        ),
        Edge::direct("ac.rescope", &["descoped"], "unsatisfied"),
        Edge::direct("ac.rescope", &["descoped"], "computed"),
        Edge::direct("ac.reinstate", &["withdrawn"], "unsatisfied"),
        Edge::direct("ac.reinstate", &["withdrawn"], "computed"),
      ],
      // **The `satisfied: false` orphan is GONE, by construction rather than by
      // a decision.** The old table recorded it with evidence: nothing produced
      // it, no verb wrote it, and `views.rs` rendered `None` and `Some(false)`
      // identically -- three representable values for two meanings. One enum
      // has no such value to be orphaned.
      orphans: &[],
    },
  },
  Field {
    entity: "AcceptanceTest",
    field: "kind",
    disposition: Disposition::Unbuilt {
      note: "test against non-test, the AT-side mirror of `Criterion.kind` and owed for the same reason",
      entry: Entry::Authored,
    },
  },
  Field {
    entity: "AcceptanceTest",
    field: "status",
    disposition: Disposition::State {
      initial: &["to-write"],
      // `at_set` takes any status and guards nothing, so the graph is complete
      // by construction. Noted as a DIVERGENCE for the register rather than
      // celebrated: v2 documents `at green` as "reachable only from red"
      // (surface/dispatch-table.json), so v3 is more closed here and less
      // faithful. Adding the guard back would keep the graph closed either
      // way; which way it goes is ic's call, not this table's.
      edges: &[
        Edge::direct("at.set", &[], "to-write"),
        Edge::direct("at.set", &[], "red"),
        Edge::direct("at.set", &[], "green"),
        Edge::direct("at.set", &[], "n-a"),
      ],
      orphans: &[],
    },
  },
  Field {
    entity: "Issue",
    field: "status",
    // **MACHINE 4** (data-model.md, hv 2026-08-17), transcribed. This row was
    // `Disposition::Unbuilt` until the ruling: the read verbs shipped, `add` /
    // `close` / `open` did not, and both values were reachable only by
    // authoring canon directly -- so `Closed` was entered and unleaveable and
    // AC-04.6's second condition held it.
    //
    // **It is DECLARED FROM v2's MEASURED BEHAVIOUR, not designed.** The
    // `intent issues` family is `keep`-classified with `target.state:
    // as-observed`, so the graph is whatever `bin/intent_issues` does.
    disposition: Disposition::State {
      initial: &["open"],
      // **NO GUARDS, DELIBERATELY, and this is the one row where the empty
      // guard list is a ratified fact rather than an absence.** v2 has none --
      // `move_issue` checks a directory and moves it -- the row is `keep`, and
      // hv's ruling says inventing one here "would be a parity break wearing a
      // ratification".
      //
      // **What a guard added here WOULD cost, recorded so the empty list is not
      // mistaken for a cheap one.** `Guard::ReasonRecorded` is the only variant
      // that could mean anything for an issue, and `Issue` has no
      // `status_reason` field to record it in -- so declaring it would be a
      // MODEL change, not a table edit, and the model change is what would
      // force the enforcement. That is a weaker protection than the one
      // `check_gate` now gives the thread machine, and the general gap is filed
      // as issue 0051.
      edges: &[
        Edge::direct("issues.close", &["open"], "closed"),
        Edge::direct("issues.open", &["closed"], "open"),
      ],
      // `issues add` is the entry, not an edge -- same treatment as `st new`
      // and `wp new`, whose target is the `initial` value above. A creation
      // verb has no from-state to declare.
      orphans: &[],
    },
  },
];

pub fn find(entity: &str, field: &str) -> Option<&'static Field> {
  FIELDS
    .iter()
    .find(|f| f.entity == entity && f.field == field)
}

/// Every declared edge for one verb on one field.
fn edges_for(entity: &str, field: &str, verb: &'static str) -> impl Iterator<Item = &'static Edge> {
  find(entity, field)
    .and_then(|f| match &f.disposition {
      Disposition::State { edges, .. } => Some(*edges),
      // Both edgeless variants, named rather than caught by a `_`, so the day a
      // third one lands the compiler asks here instead of assuming.
      Disposition::Unbuilt { .. } | Disposition::Immutable { .. } => None,
    })
    .unwrap_or(&[])
    .iter()
    .filter(move |e| e.verb == verb)
}

/// Whether the declared graph permits `verb` from `current`.
///
/// **The service layer consults this instead of restating it, and that is a
/// deliberate reversal of how the table started.** Writing the from-states
/// twice -- once in a table the test reads and once in a guard the code runs --
/// is the Highlander violation that produces exactly the drift AC-04.6 exists
/// to catch: a graph the tests describe and the code does not implement. So
/// there is ONE machine, the service enforces it, and the test's job moves up a
/// level to checking that this table is a faithful transcription of the
/// ratified document (`mutation_completeness.rs` holds the second copy).
pub fn permits(entity: &str, field: &str, verb: &'static str, current: &str) -> bool {
  edges_for(entity, field, verb).any(|e| e.accepts(current))
}

/// The values `verb` is declared to accept, so a refusal can name them rather
/// than telling the operator to go and look (AC-04.4).
pub fn accepted_from(entity: &str, field: &str, verb: &'static str) -> Vec<&'static str> {
  let mut seen: Vec<&'static str> = Vec::new();
  for edge in edges_for(entity, field, verb) {
    for value in edge.from {
      if !seen.contains(value) {
        seen.push(value);
      }
    }
  }
  seen
}

/// The guards declared for `verb`, so the service enforces the written ones
/// rather than the ones it remembers.
///
/// A verb declared over several edges -- `ac.rescope` lands on two states
/// depending on the criterion's kind -- carries its guards on all of them, so
/// the first non-empty list is the verb's. Returning the first list rather than
/// the union is deliberate: a verb whose edges disagreed about their
/// preconditions would be a defect in the table, and quietly unioning them is
/// how that defect would become invisible.
pub fn guard_for(entity: &str, field: &str, verb: &'static str) -> &'static [Guard] {
  edges_for(entity, field, verb)
    .map(|e| e.guard)
    .find(|g| !g.is_empty())
    .unwrap_or(&[])
}

/// Values with no way out, whatever put them there.
///
/// **The two conditions of the mutation-completeness rule differ only in what
/// counts as ENTRY, so they share this one exit computation.** [`traps`] asks
/// the narrower question -- entered by the declared graph -- and this asks it of
/// every value in the domain, because authored canon can carry any value the
/// schema accepts. Splitting entry from exit is what makes the two answerable
/// with one definition of "no way out" instead of two that could drift.
///
/// **Its failure mode is returning nothing, and most of its callers assert a set
/// is empty** -- so breaking it toward empty is invisible to them.
/// `the_wider_reading_fires_on_unbuilt_rows_only` is the arm that requires it to
/// find something.
pub fn exitless(values: &[String], edges: &[Edge]) -> Vec<String> {
  values
    .iter()
    .filter(|v| !edges.iter().any(|e| e.exits(v)))
    .cloned()
    .collect()
}

/// Values that can be ENTERED and not LEFT: hv's ruling, as a computation.
///
/// A value is reachable if an entity can be created in it or ANY edge lands on
/// it; it is a trap if it is reachable and no DIRECT edge moves the field off
/// it. The asymmetry is deliberate and is what [`EdgeKind`] exists for -- a
/// side effect of some other verb genuinely produces a value, and genuinely is
/// not a way out of one.
pub fn traps(values: &[String], initial: &[&str], edges: &[Edge]) -> Vec<String> {
  exitless(values, edges)
    .into_iter()
    .filter(|v| reachable(v, initial, edges))
    .collect()
}

/// Values nothing can produce -- the mirror question, reported separately
/// because an unreachable value is a modelling defect rather than a missing
/// mutation, and the two want different fixes.
pub fn unreachable(values: &[String], initial: &[&str], edges: &[Edge]) -> Vec<String> {
  values
    .iter()
    .filter(|v| !reachable(v, initial, edges))
    .cloned()
    .collect()
}

fn reachable(value: &str, initial: &[&str], edges: &[Edge]) -> bool {
  initial.contains(&value) || edges.iter().any(|e| e.to == value)
}
