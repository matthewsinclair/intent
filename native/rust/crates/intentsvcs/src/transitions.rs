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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guard {
  /// Nothing beyond the from-state.
  None,
  /// The verb takes a reason, records it on the entity's `status_reason`, and
  /// puts it in the event envelope. Refuses an empty one.
  ReasonRecorded,
  /// The close gate must PASS.
  GatePass,
  /// The verb applies only to a `(non-test)` criterion. A test-backed one has
  /// no stored satisfaction to write.
  NonTestOnly,
  /// The thread the requirement is being moved to must exist.
  TargetExists,
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
  pub guard: Guard,
}

impl Edge {
  pub const fn direct(verb: &'static str, from: &'static [&'static str], to: &'static str) -> Self {
    Self {
      verb,
      from,
      to,
      kind: EdgeKind::Direct,
      guard: Guard::None,
    }
  }

  pub const fn guarded(
    verb: &'static str,
    from: &'static [&'static str],
    to: &'static str,
    guard: Guard,
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
      guard: Guard::None,
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
  /// No service verb touches this field yet. Carries the work package that
  /// owes it, so the debt is counted rather than hidden -- the same posture
  /// the dispatch-table guard takes for an unbuilt command.
  ///
  /// An `Unbuilt` field must declare NO edges, so the day a mutation lands the
  /// disposition is contradicted and the test says so.
  Unbuilt {
    owed_by: &'static str,
    note: &'static str,
  },
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
        Edge::direct("st.start", &["not-started"], "wip"),
        Edge::direct("st.resume", &["hold"], "wip"),
        Edge::guarded(
          "st.hold",
          &["not-started", "wip"],
          "hold",
          Guard::ReasonRecorded,
        ),
        Edge::guarded("st.done", &["wip"], "completed", Guard::GatePass),
        Edge::guarded(
          "st.cancel",
          &["triage", "not-started", "wip", "hold"],
          "cancelled",
          Guard::ReasonRecorded,
        ),
        Edge::guarded("st.reopen", &["completed"], "wip", Guard::ReasonRecorded),
        Edge::guarded(
          "st.reinstate",
          &["cancelled"],
          "not-started",
          Guard::ReasonRecorded,
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
    disposition: Disposition::Unbuilt {
      owed_by: "WP-06",
      note: "the close-gate exemption. v2 has NO verb for it either -- `bin/intent_acceptance:987` instructs the user to \"add 'acceptance: exempt' to its frontmatter\", ie the tool's own error message prescribes hand-editing the file the tool owns, which is hv's ruled defect in v2's voice. Three threads in this estate use it. The verb spelling is ic's lane, so it is named as owed rather than invented here",
    },
  },
  Field {
    entity: "WorkPackage",
    field: "status",
    disposition: Disposition::State {
      initial: &["not-started"],
      // The ratified machine, transcribed (data-model.md "Machine 2"). No
      // `Hold` or `Cancelled` at WP level: a work package that stops mattering
      // is a scope change on the thread, not a state on the package.
      //
      // **`wp.reopen` is the verb whose absence was causing live damage.**
      // Adding an AC to a closed WP reopens it in the contract, nothing undid
      // `wp done`, and three of this thread's five WPs were carrying a status
      // that disagreed with their own gate -- two of them written by the
      // verifier enforcing the rule that names the class.
      edges: &[
        Edge::direct("wp.start", &["not-started"], "wip"),
        Edge::direct("wp.unstart", &["wip"], "not-started"),
        Edge::guarded("wp.done", &["wip"], "done", Guard::GatePass),
        Edge::guarded("wp.reopen", &["done"], "wip", Guard::ReasonRecorded),
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
      initial: &["XS", "S", "M", "L", "XL", "XXL"],
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
      owed_by: "WP-06",
      note: "test-backed against non-test. Converting one to the other when its test gets written is ordinary workflow and currently needs a hand-edit",
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
        Edge::guarded(
          "ac.satisfy",
          &["unsatisfied"],
          "satisfied",
          Guard::NonTestOnly,
        ),
        Edge::guarded(
          "ac.unsatisfy",
          &["satisfied"],
          "unsatisfied",
          Guard::NonTestOnly,
        ),
        Edge::guarded(
          "ac.descope",
          &["computed", "unsatisfied", "satisfied"],
          "descoped",
          Guard::TargetExists,
        ),
        Edge::guarded(
          "ac.withdraw",
          &["computed", "unsatisfied", "satisfied"],
          "withdrawn",
          Guard::ReasonRecorded,
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
      owed_by: "WP-06",
      note: "test against non-test, the AT-side mirror of `Criterion.kind` and owed for the same reason",
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
    disposition: Disposition::Unbuilt {
      owed_by: "WP-06",
      note: "the whole `issues` family is unported, so there is no verb to open or close one. Both values are reachable only by authoring canon directly",
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
      Disposition::Unbuilt { .. } => None,
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

/// The guard declared for `verb`, so the service enforces the written one
/// rather than one it remembers.
pub fn guard_for(entity: &str, field: &str, verb: &'static str) -> Guard {
  edges_for(entity, field, verb)
    .map(|e| e.guard)
    .find(|g| *g != Guard::None)
    .unwrap_or(Guard::None)
}

/// Values that can be ENTERED and not LEFT: hv's ruling, as a computation.
///
/// A value is reachable if an entity can be created in it or ANY edge lands on
/// it; it is a trap if it is reachable and no DIRECT edge moves the field off
/// it. The asymmetry is deliberate and is what [`EdgeKind`] exists for -- a
/// side effect of some other verb genuinely produces a value, and genuinely is
/// not a way out of one.
pub fn traps(values: &[String], initial: &[&str], edges: &[Edge]) -> Vec<String> {
  values
    .iter()
    .filter(|v| reachable(v, initial, edges))
    .filter(|v| !edges.iter().any(|e| e.exits(v)))
    .cloned()
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
