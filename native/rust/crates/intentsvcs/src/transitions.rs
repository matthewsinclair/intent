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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
  /// The verb exists to move this field.
  Direct,
  /// The verb's subject is `via`; this field moves as a consequence.
  Incidental { via: &'static str },
}

/// One step the service layer offers on one field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
  /// The event-log op, eg `ac.satisfy`, so an edge is traceable from the log
  /// back to the declaration that promised it.
  pub verb: &'static str,
  /// Source values. **Empty means "any current value"** -- which is how
  /// `intent st start` reopens a completed thread, and therefore why
  /// `completed` is not a trap despite there being no verb named `reopen`.
  pub from: &'static [&'static str],
  pub to: &'static str,
  pub kind: EdgeKind,
}

impl Edge {
  pub const fn direct(verb: &'static str, from: &'static [&'static str], to: &'static str) -> Self {
    Self {
      verb,
      from,
      to,
      kind: EdgeKind::Direct,
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
      initial: &["not-started"],
      // No `from` guards anywhere: `st start` accepts any current value, which
      // is what makes `completed` and `cancelled` leavable. Measured against
      // `Facade::set_thread_status`, not assumed from the verb names.
      edges: &[
        Edge::direct("st.start", &[], "wip"),
        Edge::direct("st.done", &[], "completed"),
        Edge::direct("st.cancel", &[], "cancelled"),
      ],
      orphans: &[
        (
          "tbc",
          "no verb produces it. v2 treats `TBC` as the DISPLAY of `Not Started` (bin/intent_st:120), not a distinct status, so this is very likely a display alias reified into the model rather than a missing mutation. Zero instances in this estate. Model question for hv, not a mutation gap",
        ),
        (
          "hold",
          "real v2 vocabulary (bin/intent_st:989) with no v2 command that sets it -- v2 reaches it by hand-editing frontmatter. Zero instances in this estate. Needs a verb or removal; queued for hv with `acceptance`",
        ),
      ],
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
      edges: &[
        Edge::direct("wp.start", &[], "wip"),
        Edge::direct("wp.done", &[], "done"),
      ],
      orphans: &[],
    },
  },
  Field {
    entity: "WorkPackage",
    field: "scope",
    disposition: Disposition::Unbuilt {
      owed_by: "WP-06",
      note: "the t-shirt size, set once by `wp new` and never again by any verb, in v2 or v3. Re-sizing a work package as its shape becomes clear is ordinary workflow, so under D32 this owes a mutation; it is not a lifecycle state, so it is not this AC's trap case",
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
    field: "scope",
    disposition: Disposition::State {
      initial: &["in-scope"],
      // The one field that was already closed, and the reason it is: descope
      // and withdraw were each built WITH their inverse, and the inverses
      // refuse each other's state rather than collapsing into one verb.
      edges: &[
        Edge::direct("ac.descope", &["in-scope"], "descoped"),
        Edge::direct("ac.withdraw", &["in-scope"], "withdrawn"),
        Edge::direct("ac.rescope", &["descoped"], "in-scope"),
        Edge::direct("ac.reinstate", &["withdrawn"], "in-scope"),
      ],
      orphans: &[],
    },
  },
  Field {
    entity: "Criterion",
    field: "satisfied",
    disposition: Disposition::State {
      initial: &[ABSENT],
      // THE instance hv ruled on. `ac.satisfy` had no inverse, so a verifier
      // whose evidence proved incomplete had to hand-edit acceptance.md.
      //
      // All four SCOPE verbs appear here as well as on `scope`, because one
      // verb moves two fields: v2 strips the row's whole tail on every scope
      // change, on the way out (bin/intent_acceptance:1191) as well as on the
      // way back (:1250), so a criterion that changes scope loses the evidence
      // it carried before the move. One verb, two declared edges, all executed.
      edges: &[
        Edge::direct("ac.satisfy", &[ABSENT, "false"], "true"),
        Edge::direct("ac.unsatisfy", &["true"], ABSENT),
        Edge::incidental("ac.descope", &[], ABSENT, "scope"),
        Edge::incidental("ac.withdraw", &[], ABSENT, "scope"),
        Edge::incidental("ac.rescope", &[], ABSENT, "scope"),
        Edge::incidental("ac.reinstate", &[], ABSENT, "scope"),
      ],
      orphans: &[(
        "false",
        "nothing produces it. Strict ingest reads v3 canon only and never writes the field; no verb writes `false`; and the view renders `None` and `Some(false)` identically (`satisfied.unwrap_or(false)`, views.rs:443). So the domain has three representable values and two meanings. WP-10 decides whether v2's 13 `satisfied: no` rows in this estate migrate to absent or to false -- if false, this stops being an orphan and the entry comes out",
      )],
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
