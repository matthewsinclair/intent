//! **`AC-09.4`'s SECOND CLAUSE: *no hand-maintained command list exists* --
//! a claim about the ESTATE, not about the guide** (ruled by vc, 2026-08-30).
//!
//! The clause was very nearly read as guide-scoped, and vc's argument against
//! that is structural rather than a preference: **a guide that renders from the
//! table keeps no list of its own BY DEFINITION**, so a guide-scoped clause 2
//! would restate clause 1. A reading that makes half a row redundant is the
//! wrong reading -- clause 2 is doing different work or it is doing none.
//!
//! **THIS FILE IS `AT-09.4`'s CITED ARTEFACT, AND IT WITNESSES CLAUSE 2 ONLY**
//! (ic, 2026-08-31; re-cited from `llm_guide_gen.rs`, a file that never
//! existed). Clause 1 -- *`intent llm` renders the agent guide from the
//! dispatch table* -- is witnessed in `src/guide.rs`'s own tests, where
//! `7ecb1e62` proved DERIVATION rather than agreement by mutation: point
//! `command_reference` at the compiled-in table instead of the one it was
//! handed and 13 pass while 1 fails. A row citing one file for a two-clause
//! criterion names the clause the file carries and where the other lives, so
//! the green cannot be read as this file proving both.
//!
//! # What is actually being guarded
//!
//! `bin/intent_help` hand-maintained a list of commands behind a skip list, and
//! it was correct on the day it was typed. **It did not break by being wrong.
//! It broke by being separate** -- the act that invalidated it was never the
//! act that updated it. So the property is not *the list is right today*; it is
//! *every such list is either derived from the table or declared, with what
//! must become TRUE for it to go*.
//!
//! **A roster of exceptions each carrying a discharge condition cannot rot
//! quietly. One carrying reasons why a thing is TOLERABLE rots the moment it
//! stops being tolerable, and nothing notices.** That distinction is vc's, made
//! the same day they deleted `EXPECTED_DISAGREEMENTS`' last entry because it
//! named its own discharge condition and the condition had been met.
//!
//! # The population is MULTI-TOKEN paths, and that is a real limit
//!
//! A literal spelled exactly `"st list"` is a string that exists nowhere but
//! the command surface. **Single-token paths are ordinary words** -- `st`,
//! `at`, `info`, `edit`, `help` -- and matching on them is not a check but
//! noise: measured 2026-08-30, `render.rs` alone contains 35 of them as exact
//! literals, in messages, argument ids and match arms. A test flagging 35 sites
//! that are nearly all innocent is one nobody keeps.
//!
//! **SO THIS DOES NOT CATCH A ROSTER MADE ONLY OF SINGLE-TOKEN COMMANDS**, eg
//! `["doctor", "upgrade", "organize"]`. That hole is stated here rather than
//! discovered later. Closing it needs per-`const` attribution -- bounding each
//! `const NAME: &[..] = &[..];` span and scanning only inside it, which narrows
//! the same sweep from 35 sites to 4. **It was deliberately not built**: a
//! hand-rolled span finder over `render.rs` is the same class of instrument
//! that failed three times in one afternoon on this estate, each failure
//! producing a plausible answer rather than a detectable one. An unsound check
//! is worse than a narrow one, because a narrow one knows what it missed.
//!
//! Filed as the extension rather than left implicit: the four `const`s that
//! sweep surfaced are recorded in [`DECLARED`] with their classifications, so
//! the judgement exists on disk even where the automated population does not
//! reach -- **a near-miss classified in a node's head is an exclusion recorded
//! nowhere**.

mod common;
use common::{declared_paths, shipped_sources, string_literals};

/// Why a shipped source file is allowed to spell a command path exactly.
#[derive(Debug)]
enum Why {
  /// A roster of command paths, tolerated, carrying **what must become TRUE**
  /// for it to stop being needed.
  Discharges(&'static str),
  /// Not a roster of command paths: the collision is coincidental.
  ///
  /// Recorded rather than filtered, because a filter is a judgement with no
  /// author and no date.
  NotARoster(&'static str),
  /// A roster held against the table by a named test, so it cannot drift.
  CheckedBy(&'static str),
}

/// The one reason all 52 of `mcp.rs`'s path literals share -- the serving
/// match's SERVED roster and arms (vc's (a)-now ruling, 2026-08-30).
///
/// **CHECKED, NOT MERELY TOLERATED**: `mcp::tests::the_roster_and_the_tool_\
/// population_agree_both_ways` holds the roster against the table-generated
/// tool population in BOTH directions, and `mcp::tests::every_roster_path_\
/// reaches_an_arm` drives each path to an arm through a real in-memory facade
/// -- so the list cannot drift from the table, which is exactly the property
/// this guard exists to demand of a hand-kept list. The match itself goes when
/// the MCP tier routes through `dispatch(op)`, the 3.x destination
/// (`mcp.rs`'s header LIMIT).
const MCP_SERVING_MATCH: &str = "the MCP serving match: SERVED + arms, held against the \
   table-generated tool population both directions by \
   `mcp::tests::the_roster_and_the_tool_population_agree_both_ways` and driven by \
   `mcp::tests::every_roster_path_reaches_an_arm`; the match discharges when the tier routes \
   through `dispatch(op)`, the 3.x destination";

/// Every (file, exact command-path literal) this estate declares, and why.
///
/// **BOTH DIRECTIONS ARE ASSERTED.** An undeclared hit is a new hand-kept
/// roster; a declared entry with no hit is an exception that has outlived its
/// subject, which is the failure mode that makes an exceptions list rot into a
/// list of excuses.
///
/// **TWO SITES, ONE ENTRY, AND THE REASON MUST COVER BOTH** -- which the first
/// version of this roster did not, and the mutation that made the entry stale
/// is what showed it: the failure printed the same pair twice.
///
/// A declaration keyed on (file, path) cannot tell two sites apart, so it must
/// account for every one of them or it is silently excusing a site nobody
/// examined. That is the same defect this file exists to catch, one level up.
const DECLARED: &[(&str, &str, Why)] = &[
  (
    "intent-cli/src/render.rs",
    "daemon status",
    Why::CheckedBy(
      "NOT A ROSTER AND NOT A COINCIDENCE: it is the LOOKUP KEY `enum_flag(a, \"daemon status\", \
     \"--format\")` uses to find the row's declared values, and the same literal names the row \
     back to the operator in the refusal. **The first MULTI-token label the scanner has met** -- \
     `doctor`'s is a single token and goes unscanned -- so this shape is new to the guard rather \
     than newly wrong. \
     HELD AGAINST THE TABLE AT RUNTIME, WHICH IS WHY THIS IS `CheckedBy` AND NOT MERELY \
     TOLERATED: `enum_flag` resolves the literal against `dispatch::table()` and takes a DIFFERENT \
     branch when it does not match -- `Failure::Unavailable` at exit 2, `the dispatch table \
     declares no values`, rather than the exit-1 refusal an undeclared VALUE gets. So a typo \
     cannot pass as a working lookup. \
     DRIVEN BY `format_roster_is_honoured::the_daemon_status_lookup_resolves_and_refuses_by_name`, \
     which asserts the exit CODE and not merely that it refused -- a mistyped label refuses too, \
     more loudly and about the wrong thing. Mutation driven in a detached worktree: doubling the \
     space in the literal reds that arm on the 2-versus-1 assertion, naming the cause. \
     THE ARM IS THE CONDITION OF THIS CLASSIFICATION (vc, 2026-08-31): a runtime check nothing \
     exercises is a guard nobody runs, and `CheckedBy` on the strength of an unexercised refusal \
     would be declaring a guarantee from a mechanism no test has made fire. Without that arm this \
     entry is `NotARoster` and nothing stronger.",
    ),
  ),
  (
    "intent-cli/src/render.rs",
    "st list",
    Why::Discharges(
      "TWO SITES SPELL IT AND THEY DISCHARGE TOGETHER. (1) `SERVED_BY_DAEMON`, the roster; (2) the \
     `served(\"st list\", ..)` CALL SITE, where the path is spelled so the router can resolve it \
     -- its own note says a site that forgets to route falls through with a zero delta, so \
     spelling it there is what makes the omission red by name rather than invisible. \
     BOTH GO WHEN THE TABLE CARRIES THE MAPPING: `SERVED_BY_DAEMON` discharges when \
     `surface/dispatch-table.json` declares each path's serving `Op`, or its absence -- at that \
     point `daemon_op_for` reads the table and the roster is a projection rather than a second \
     home (cc, 2026-08-30, quoted verbatim), and the call site resolves through the same \
     declaration. THE SHORTHAND THIS REPLACED -- `it discharges when the daemon serves the \
     surface` -- COULD NEVER BECOME TRUE: `version`, `info`, `init` and the `lang` verbs need no \
     store, so they will never be daemon-served. An exception whose condition is unmeetable \
     reads like the kind that cannot rot and behaves like the kind that does.",
    ),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st new",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st start",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st done",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st cancel",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st triage",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st hold",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st resume",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st reopen",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st reinstate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st hydrate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st dehydrate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st list",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st show",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st edit",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "st sync",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp new",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp start",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp done",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp reopen",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp cancel",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp reinstate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp unstart",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp rescope",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp list",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "wp show",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac list",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac status",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac satisfy",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac unsatisfy",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac gate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac descope",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac rescope",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac withdraw",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac reinstate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac new",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "ac edit",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "at list",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "at lint",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "at green",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "at red",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "at na",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "at new",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "at edit",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "issues list",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "issues add",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "issues show",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "issues close",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "issues open",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "todo list",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "todo update",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "agents generate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
  (
    "intent-cli/src/mcp.rs",
    "agents validate",
    Why::CheckedBy(MCP_SERVING_MATCH),
  ),
];

/// The four `const`s a per-`const` sweep surfaced on 2026-08-30, classified by
/// hand because the automated population above does not reach single-token
/// paths.
///
/// **THIS IS EVIDENCE, NOT A CHECK, AND IT SAYS SO.** Nothing asserts these
/// today. It exists because vc's ruling requires the classification to live on
/// disk rather than in a message, and because the sweep is the thing a later
/// reader would otherwise re-run from scratch.
const SWEPT_BY_HAND: &[(&str, &str, Why)] = &[
  (
    "intent-cli/src/render.rs",
    "SERVED_BY_DAEMON",
    Why::Discharges("the entry above; this is the one the automated check reaches"),
  ),
  (
    "intent-cli/src/guide.rs",
    "KNOWN_OVERLAP",
    Why::CheckedBy(
      "`guide.rs`'s own tests, which hold each overlap against the table's two rows for that \
       path rather than against a typed list",
    ),
  ),
  (
    "intentsvcs/src/address.rs",
    "VIEW_NAMES",
    Why::NotARoster(
      "generated-view FILE names, not command paths; `info` collides because a view and a \
       command share a word. Its real defect -- nothing asserts it agrees with \
       `Project::classify` -- is issue 0170",
    ),
  ),
  (
    "intentsvcs/src/event.rs",
    "KNOWN_OPS",
    Why::NotARoster(
      "event op names, dotted (`ac.edit`, `at.fc`); `init` collides because one op is spelled \
       like one command. An op roster is a sibling concern with its own homes",
    ),
  ),
];

/// The two the sweep could NOT bound, kept because a refusal is a result.
///
/// **`MIGRATIONS` and `FIELDS` are multi-line `const`s whose span the finder
/// could not close** -- and it REFUSED rather than guessing a boundary, which
/// is the behaviour worth keeping. Classified by reading them: SQL migration
/// bodies and transition field declarations. Neither carries a command path.
const SWEEP_REFUSED: &[(&str, &str)] = &[
  ("intentsvcs/src/store.rs", "MIGRATIONS"),
  ("intentsvcs/src/transitions.rs", "FIELDS"),
];

fn multi_token_paths() -> Vec<String> {
  declared_paths()
    .into_iter()
    .filter(|p| p.contains(' '))
    .collect()
}

/// `shipped_sources` yields absolute paths; the roster names them the way a
/// person would. Anchored on `crates/` so the answer does not depend on where
/// the repository is checked out.
fn short(path: &std::path::Path) -> String {
  let s = path.to_string_lossy();
  match s.find("crates/") {
    Some(i) => s[i + "crates/".len()..].to_string(),
    None => s.to_string(),
  }
}

#[test]
fn every_exact_command_path_literal_in_shipped_source_is_declared() {
  let multi = multi_token_paths();
  assert!(
    multi.len() > 50,
    "the table declares only {} multi-token paths, which is too few to be the real surface -- \
     `declared_paths` has stopped reading a row home and every assertion below is weakened",
    multi.len()
  );

  let mut found: Vec<(String, String)> = Vec::new();
  let mut files_scanned = 0usize;
  for file in shipped_sources() {
    let code = std::fs::read_to_string(&file).expect("read shipped source");
    files_scanned += 1;
    for lit in string_literals(&code) {
      if multi.contains(&lit) {
        let site = (short(&file), lit);
        // Deduped: one file may spell one path at several sites, and a
        // declaration keyed on (file, path) cannot tell them apart -- so the
        // REASON must cover every site, and the message must not imply the
        // check found two different things.
        if !found.contains(&site) {
          found.push(site);
        }
      }
    }
  }
  assert!(
    files_scanned > 10,
    "only {files_scanned} shipped files were scanned, so this test examined almost nothing"
  );

  // The control: the check must be able to SEE the one site the estate knows
  // about. A scanner that returned nothing at all would satisfy the
  // undeclared-is-empty assertion below perfectly.
  assert!(
    found
      .iter()
      .any(|(f, p)| f == "intent-cli/src/render.rs" && p == "st list"),
    "the known roster site was not found, so this scan is blind and its silence means nothing: \
     {found:?}"
  );

  let undeclared: Vec<&(String, String)> = found
    .iter()
    .filter(|(f, p)| !DECLARED.iter().any(|(df, dp, _)| df == f && dp == p))
    .collect();
  assert!(
    undeclared.is_empty(),
    "shipped source spells {} command path(s) exactly, with nothing declaring why: {undeclared:?}\n\
     A hand-kept command list does not break by being WRONG, it breaks by being SEPARATE -- so \
     either derive it from the dispatch table, or add it to DECLARED with what must become TRUE \
     for it to go.",
    undeclared.len()
  );
}

/// **AN EXCEPTION THAT HAS OUTLIVED ITS SUBJECT IS THE FAILURE THIS WHOLE FILE
/// IS ABOUT**, wearing the clothes of diligence: it reads as a considered
/// allowance and is a note about code that no longer exists.
#[test]
fn no_declared_exception_has_outlived_its_subject() {
  let multi = multi_token_paths();
  let mut found: Vec<(String, String)> = Vec::new();
  for file in shipped_sources() {
    let code = std::fs::read_to_string(&file).expect("read shipped source");
    for lit in string_literals(&code) {
      if multi.contains(&lit) {
        found.push((short(&file), lit));
      }
    }
  }

  let stale: Vec<&str> = DECLARED
    .iter()
    .filter(|(df, dp, _)| !found.iter().any(|(f, p)| f == df && p == dp))
    .map(|(_, dp, _)| *dp)
    .collect();
  assert!(
    stale.is_empty(),
    "DECLARED still excuses {stale:?}, which shipped source no longer spells -- the roster is \
     now a record of a decision about code that is gone, and it will go on reading like a \
     considered allowance"
  );
}

/// The hand-swept classifications are held to the one thing that CAN be checked
/// about them: each names a real file, and each carries a reason.
///
/// **This does not verify the classifications**, and saying so is the point --
/// a test that appeared to check them would be worse than one that admits it
/// does not.
#[test]
fn the_hand_swept_classifications_name_real_files_and_carry_reasons() {
  let shipped: Vec<String> = shipped_sources().iter().map(|p| short(p)).collect();
  assert!(
    !SWEPT_BY_HAND.is_empty() && !SWEEP_REFUSED.is_empty(),
    "both sweep records are empty, so the loops below examine nothing"
  );
  for (file, name, why) in SWEPT_BY_HAND {
    assert!(
      shipped.contains(&file.to_string()),
      "the sweep records `{name}` in `{file}`, which is not a shipped source file"
    );
    let reason = match why {
      Why::Discharges(s) | Why::NotARoster(s) | Why::CheckedBy(s) => s,
    };
    assert!(
      reason.len() > 20,
      "`{name}` carries a reason too short to be one: {reason:?}"
    );
  }
  for (file, name) in SWEEP_REFUSED {
    assert!(
      shipped.contains(&file.to_string()),
      "the sweep refused on `{name}` in `{file}`, which is not a shipped source file"
    );
  }
}

// ---------------------------------------------------------------------------
// The hand-maintained list this file's own scan could not see
// ---------------------------------------------------------------------------
//
// **CLAUSE 2 IS A CLAIM ABOUT THE ESTATE, AND THE ESTATE INCLUDES SHIPPED JSON
// PAYLOAD.** Everything above scans Rust string literals, so a hand-maintained
// command list living in a `plugin.json` was outside its population from the
// day it was written -- not missed, unreachable. `intent/plugins/claude/
// plugin.json` carried seven entries and `intent plugin show claude` printed
// them.
//
// **AND IT HAD ROTTED EXACTLY THE WAY THIS FILE'S HEADER PREDICTS: not by being
// wrong, by being separate.** Measured 2026-08-30 by invoking all eight entries
// across both shipped manifests, three failed and each in a different way --
// `claude prime` answering `2` unwired (its v2 script was pruned that
// afternoon), `audit` answering `1` with `unrecognized subcommand` because the
// binary has no such command at all, and `agents` answering `2` at the family
// root while `agents sync` and `agents validate` both answer `0`. Not one of
// the three acts that caused those was an act that would have updated a JSON
// file in a plugin directory.
//
// **hv RULED THE LIST OUT RATHER THAN CORRECTED** (2026-08-30): `plugin.json`
// stops carrying commands. The surface has a home -- `surface/dispatch-table
// .json`, which the binary is BUILT from -- and a plugin manifest restating it
// is a third copy that can only ever drift from a source it does not read.

/// Every shipped plugin manifest, as `plugins::discover` would find them.
fn shipped_plugin_manifests() -> Vec<(String, serde_json::Value)> {
  let root = testkit::repo_root().join("intent/plugins");
  let mut out = Vec::new();
  for entry in std::fs::read_dir(&root).expect("intent/plugins is readable") {
    let dir = entry.expect("a readable dir entry").path();
    let manifest = dir.join("plugin.json");
    if !manifest.is_file() {
      continue;
    }
    let text = std::fs::read_to_string(&manifest).expect("a readable plugin manifest");
    let value: serde_json::Value = serde_json::from_str(&text).expect("a plugin manifest is JSON");
    out.push((
      dir.file_name().unwrap().to_string_lossy().to_string(),
      value,
    ));
  }
  out
}

/// **NOT VACUOUS, AND THAT IS WHY IT IS PHRASED THIS WAY.** The obvious form --
/// *every command a manifest declares is answered by the binary* -- passes over
/// an empty list, so it would go quiet the moment the lists came out and stay
/// quiet if one grew back small and wrong. This one reads real files and fails
/// on regrowth.
#[test]
fn no_shipped_plugin_manifest_hand_maintains_a_command_list() {
  let manifests = shipped_plugin_manifests();
  assert!(
    manifests.len() >= 2,
    "only {} plugin manifest(s) found -- the population is empty or the path \
     moved, and a green here would mean nothing",
    manifests.len()
  );

  let offenders: Vec<String> = manifests
    .iter()
    .filter_map(|(name, v)| {
      let n = v.get("commands")?.as_array()?.len();
      (n > 0).then(|| format!("{name}: declares {n} command(s)"))
    })
    .collect();

  assert!(
    offenders.is_empty(),
    "a plugin manifest hand-maintains a command list, which is a third home for \
     the surface and can only drift from `surface/dispatch-table.json`:\n  {}\n\
     the plugin's identity -- name, version, description -- is its own to state; \
     what commands the binary answers is not.",
    offenders.join("\n  ")
  );
}

/// The manifests still say who they are. Removing the command list must not
/// have emptied the file of the thing `intent plugin show` exists to print.
#[test]
fn a_plugin_manifest_still_declares_its_own_identity() {
  for (dir, v) in shipped_plugin_manifests() {
    for field in ["name", "version", "description"] {
      let present = v
        .get(field)
        .and_then(|f| f.as_str())
        .is_some_and(|s| !s.trim().is_empty());
      assert!(
        present,
        "{dir}/plugin.json has no usable `{field}` -- a plugin that cannot name \
         itself is worse than one that over-declares its commands"
      );
    }
  }
}
