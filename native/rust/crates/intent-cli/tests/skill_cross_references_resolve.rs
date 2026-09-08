//! `AT-15.4` (ST0056) / `AC-15.4`: **every cross-reference in the surviving
//! skill catalogue resolves.**
//!
//! The row names three kinds of reference. **Only two of them are here, and the
//! omission is deliberate rather than partial work.**
//!
//! # `chains_to:` IS NOT CHECKED HERE, AND MUST NOT BE ADDED
//!
//! `AT-00.4` (ST0065) already resolves every `chains_to:` target against the
//! catalogue, in `every_skill_has_a_live_caller.rs`, as its `Finding::Dangling`
//! arm -- with its own planted control. **A second copy here would be a
//! Highlander violation in the row whose whole subject is references that stop
//! resolving**, and two catalogue walkers drift in exactly the way a dangling
//! reference does: silently, with both sides looking healthy. A reader who
//! notices the gap should read that file, not close it here.
//!
//! # WHAT THE OTHER TWO ARE, AND WHY THE SECOND DIRECTION IS THE UNCHECKED ONE
//!
//! `every_skill_has_a_live_caller.rs` already COLLECTS `/in-*` names out of
//! prose -- but it uses them to answer *is this skill reached*, which is the
//! opposite direction from *does this citation resolve*. A document naming
//! `/in-retired` simply adds a name that matches no skill, and nothing there
//! fires. **The two directions share an extractor and do not share a failure
//! mode**, which is the whole reason this arm needed its own instrument.
//!
//! # THE SAME EXTRACTOR IS SAFE IN ONE DIRECTION AND UNSAFE IN THE OTHER
//!
//! That asymmetry is not a curiosity, it is this file's main hazard. In the
//! reachability direction a spurious extracted name is HARMLESS -- it lands in
//! a set of things that reach skills and matches nothing. In the resolution
//! direction the same spurious name becomes a **false dangling reference**, and
//! a check with a standing false positive is one nobody runs twice.
//!
//! **The hazard is live in the catalogue today, not hypothetical, and it is
//! DRIVEN rather than argued.** `in-session` documents a sentinel path --
//! `/tmp/intent/in-session-<UUID>.sentinel` -- and a path-blind scan reads
//! `in-session-` out of it as a cited skill that does not exist. Measured
//! 2026-09-08: that is the only such string in 23 skills, and it is enough to
//! redden this check permanently. So `skill_refs_in` guards on the character
//! BEFORE the slash, and
//! [`a_path_that_merely_contains_a_skill_name_is_not_a_citation`] pins both
//! halves of that guard -- because a guard that suppressed everything would
//! pass the negative arm alone.
//!
//! **THE GUARD WAS MUTATION-TESTED AGAINST THE REAL TREE, AND THE FIRST RUN
//! FAILED TO BITE.** Deleting the guard left the real-tree arm GREEN, because a
//! `trim_end_matches('-')` elsewhere in the same function was quietly repairing
//! `in-session-` into a name that resolves. The header claim and the code had
//! diverged before either shipped: **the prose asserted a guard was load-bearing
//! over a corpus where a second mechanism made it unreachable.** The trim is
//! gone (see `skill_refs_in`), and with one mechanism the mutation now yields
//! exactly one finding -- `in-session` citing `in-session-` -- which is the
//! number an independent scan of the same corpus predicted.
//!
//! # WHAT A GREEN HERE DOES NOT MEAN
//!
//! This resolves references statically. It establishes that a name a skill
//! cites exists as a directory, and that a rule id it names exists in the rule
//! library. It does not establish that following the reference does anything
//! useful, and it cannot -- `AC-00.2` covers whether an agent can execute the
//! instruction, and that is a read rather than a test.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use testkit::repo_root;

/// A reference in one skill's prose that resolves to nothing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Finding {
  /// `from` cites `/in-<to>` and no such skill is in the catalogue.
  DanglingSkill { from: String, to: String },
  /// `from` names rule `id` and the rule library does not carry it.
  DanglingRule { from: String, id: String },
}

/// What one skill cites.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cites {
  from: String,
  skills: Vec<String>,
  rules: Vec<String>,
}

/// Every `/in-<name>` INVOCATION in a body of prose.
///
/// **THE GUARD IS THE CHARACTER BEFORE THE SLASH**, and it is what separates an
/// invocation from a path segment. `intent/plugins/claude/skills/in-detrope/...`
/// and `/tmp/intent/in-session-<UUID>` both contain a skill name after a slash;
/// neither is a citation, and both would be reported as dangling by a scan that
/// only looked forward. Two skills carry the first shape and one carries the
/// second, so this is load-bearing at the current catalogue rather than
/// defensive.
///
/// **A TRAILING `-` IS DELIBERATELY NOT TRIMMED, AND THE FIRST DRAFT OF THIS
/// FILE DID TRIM IT.** Trimming looks like tidying and is a false-negative
/// generator: `/tmp/intent/in-session-<UUID>` becomes `in-session-`, and a trim
/// turns it into `in-session`, which resolves. **That made the sentinel path
/// read as a valid citation of a real skill** -- harmless in its result and
/// fatal to the instrument, because it MASKED the guard below. Caught by
/// removing the guard and finding the real-tree arm still green when it should
/// have reddened; without the mutation run the header's claim that the guard is
/// load-bearing would have shipped as prose over a check that could not have
/// exercised it. **One mechanism, so a mutation to it bites.**
fn skill_refs_in(text: &str) -> Vec<String> {
  fn path_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.' | b'-' | b'/')
  }
  let b = text.as_bytes();
  let mut out = Vec::new();
  for (i, &c) in b.iter().enumerate() {
    if c != b'/' {
      continue;
    }
    if i > 0 && path_char(b[i - 1]) {
      continue;
    }
    let start = i + 1;
    let mut end = start;
    while end < b.len()
      && (b[end].is_ascii_lowercase() || b[end].is_ascii_digit() || b[end] == b'-')
    {
      end += 1;
    }
    let name = &text[start..end];
    if name.starts_with("in-") && name.len() > 3 {
      out.push(name.to_string());
    }
  }
  out
}

/// Every rule id named in a body of prose.
///
/// The shape is `IN-<2 letters>-<CATEGORY>-<3 digits>`, hand-scanned for the
/// same reason `chains_of` is hand-parsed in `every_skill_has_a_live_caller.rs`:
/// no dependency is worth adding to read a fixed-width identifier, and a
/// tolerant scan cannot fail to read one that is really there.
fn rule_ids_in(text: &str) -> Vec<String> {
  let b = text.as_bytes();
  let mut out = Vec::new();
  let mut i = 0usize;
  while i + 3 < b.len() {
    if &b[i..i + 3] != b"IN-" || (i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'-')) {
      i += 1;
      continue;
    }
    let mut end = i + 3;
    while end < b.len()
      && (b[end].is_ascii_uppercase() || b[end].is_ascii_digit() || b[end] == b'-')
    {
      end += 1;
    }
    let cand = &text[i..end];
    // `IN-AG-PFIC-001` -- four dash-separated fields, the last three digits.
    let fields: Vec<&str> = cand.split('-').collect();
    if fields.len() == 4
      && fields[1].len() == 2
      && !fields[2].is_empty()
      && fields[3].len() == 3
      && fields[3].bytes().all(|c| c.is_ascii_digit())
    {
      out.push(cand.to_string());
    }
    i = end.max(i + 1);
  }
  out
}

/// **THE WHOLE CHECK, AS A PURE FUNCTION**, so it can be driven against planted
/// inputs where the answer is known. An assertion written straight over the real
/// tree can only ever be observed passing.
fn unresolved(
  cites: &[Cites],
  skills: &BTreeSet<String>,
  rules: &BTreeSet<String>,
) -> Vec<Finding> {
  let mut out = Vec::new();
  for c in cites {
    for to in &c.skills {
      if !skills.contains(to) {
        out.push(Finding::DanglingSkill {
          from: c.from.clone(),
          to: to.clone(),
        });
      }
    }
    for id in &c.rules {
      if !rules.contains(id) {
        out.push(Finding::DanglingRule {
          from: c.from.clone(),
          id: id.clone(),
        });
      }
    }
  }
  out.sort();
  out.dedup();
  out
}

/// **THE POPULATION IS DERIVED FROM THE TREE, NEVER LISTED.** A skill added
/// tomorrow is in this check without anyone editing it.
fn catalogue(dir: &Path) -> Vec<String> {
  let Ok(entries) = fs::read_dir(dir) else {
    return Vec::new();
  };
  let mut names: Vec<String> = entries
    .flatten()
    .filter(|e| e.path().is_dir())
    .map(|e| e.file_name().to_string_lossy().to_string())
    .collect();
  names.sort();
  names
}

/// Every rule id the library declares, read from the `id:` line of each
/// `RULE.md`.
///
/// **READ FROM DISK RATHER THAN FROM `intent claude rules list`, AND THE TWO
/// WERE COMPARED BEFORE CHOOSING**: 66 files, 66 ids, identical sets on
/// 2026-09-08. Disk is the right source because it needs no installed binary --
/// a test that shelled out would be measuring whichever `intent` happened to be
/// on PATH, which is a different subject from the tree under test.
fn rule_library(dir: &Path) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  let mut stack = vec![dir.to_path_buf()];
  while let Some(d) = stack.pop() {
    let Ok(entries) = fs::read_dir(&d) else {
      continue;
    };
    for e in entries.flatten() {
      let p = e.path();
      if p.is_dir() {
        stack.push(p);
        continue;
      }
      if !p.file_name().is_some_and(|n| n == "RULE.md") {
        continue;
      }
      let Ok(text) = fs::read_to_string(&p) else {
        continue;
      };
      if let Some(line) = text.lines().find(|l| l.starts_with("id:")) {
        out.insert(line[3..].trim().to_string());
      }
    }
  }
  out
}

fn cites_under(dir: &Path, names: &[String]) -> Vec<Cites> {
  let mut out = Vec::new();
  for name in names {
    let Ok(text) = fs::read_to_string(dir.join(name).join("SKILL.md")) else {
      continue;
    };
    out.push(Cites {
      from: name.clone(),
      skills: skill_refs_in(&text),
      rules: rule_ids_in(&text),
    });
  }
  out
}

#[test]
fn every_skill_and_rule_a_skill_cites_resolves() {
  let root = repo_root();
  let skills_dir = root.join("intent/plugins/claude/skills");
  let names = catalogue(&skills_dir);
  let skills: BTreeSet<String> = names.iter().cloned().collect();
  let rules = rule_library(&root.join("intent/plugins/claude/rules"));
  let cites = cites_under(&skills_dir, &names);

  // **THE DENOMINATORS ARE ASSERTED BEFORE THE ZERO IS BELIEVED.** An empty
  // catalogue, an empty rule library, or an extractor that found nothing all
  // produce the same clean result as a catalogue with no dangling references.
  assert!(
    skills.len() > 1 && !rules.is_empty(),
    "population came back as {} skills / {} rules -- a check over an empty set \
     agrees with everything",
    skills.len(),
    rules.len()
  );
  let cited_skills: usize = cites.iter().map(|c| c.skills.len()).sum();
  let cited_rules: usize = cites.iter().map(|c| c.rules.len()).sum();
  assert!(
    cited_skills > 0,
    "no skill cites another by name, so the /in-* arm is asserting nothing over \
     {} skills",
    skills.len()
  );
  assert!(
    cited_rules > 0,
    "no skill names a rule id, so the rule arm is asserting nothing over {} \
     rules",
    rules.len()
  );

  let findings = unresolved(&cites, &skills, &rules);
  assert!(
    findings.is_empty(),
    "skills cite names that resolve to nothing (over {cited_skills} skill \
     citations and {cited_rules} rule citations): {findings:#?}"
  );
}

/// **CONTROL: A CITATION OF A RETIRED SKILL MUST FIRE.** This is the shape a
/// retirement leaves behind -- the citing skill is healthy in every other
/// respect, and its prose still reads correctly.
#[test]
fn a_citation_of_a_skill_that_does_not_exist_is_found() {
  let skills: BTreeSet<String> = ["in-alpha".to_string()].into_iter().collect();
  let rules: BTreeSet<String> = ["IN-AG-PFIC-001".to_string()].into_iter().collect();
  let cites = vec![Cites {
    from: "in-alpha".into(),
    skills: vec!["in-retired".into()],
    rules: Vec::new(),
  }];
  assert_eq!(
    unresolved(&cites, &skills, &rules),
    vec![Finding::DanglingSkill {
      from: "in-alpha".into(),
      to: "in-retired".into()
    }],
    "a citation left behind by a retirement is not reported"
  );
}

/// **CONTROL: A RULE ID THAT THE LIBRARY DOES NOT CARRY MUST FIRE.** A renamed
/// or withdrawn rule leaves prose pointing at an id nothing serves, and
/// `intent claude rules show` on it fails at the moment someone needs it.
#[test]
fn a_rule_id_the_library_does_not_carry_is_found() {
  let skills: BTreeSet<String> = ["in-alpha".to_string()].into_iter().collect();
  let rules: BTreeSet<String> = ["IN-AG-PFIC-001".to_string()].into_iter().collect();
  let cites = vec![Cites {
    from: "in-alpha".into(),
    skills: Vec::new(),
    rules: vec!["IN-AG-GONE-009".into()],
  }];
  assert_eq!(
    unresolved(&cites, &skills, &rules),
    vec![Finding::DanglingRule {
      from: "in-alpha".into(),
      id: "IN-AG-GONE-009".into()
    }],
    "a rule id the library does not carry is not reported"
  );
}

/// **AND THE PREDICATE MUST STAY QUIET WHEN EVERYTHING RESOLVES**, or the two
/// controls above pass for a function that reports unconditionally.
#[test]
fn everything_resolving_reports_nothing() {
  let skills: BTreeSet<String> = ["in-alpha".to_string(), "in-beta".to_string()]
    .into_iter()
    .collect();
  let rules: BTreeSet<String> = ["IN-AG-PFIC-001".to_string()].into_iter().collect();
  let cites = vec![Cites {
    from: "in-alpha".into(),
    skills: vec!["in-beta".into()],
    rules: vec!["IN-AG-PFIC-001".into()],
  }];
  assert!(
    unresolved(&cites, &skills, &rules).is_empty(),
    "a fully resolving catalogue is reported as broken"
  );
}

/// **THE FALSE POSITIVE THAT WOULD RETIRE THIS CHECK, PINNED IN BOTH
/// DIRECTIONS.**
///
/// The negative half alone would pass for an extractor that returned nothing at
/// all, so the positive half is in the SAME fixture: one string that must be
/// ignored and one that must be found, in one document.
#[test]
fn a_path_that_merely_contains_a_skill_name_is_not_a_citation() {
  let text = "Run `/in-session` first.\n\
              Expected sentinel: /tmp/intent/in-session-<UUID>.sentinel\n\
              The catalogue lives at intent/plugins/claude/skills/in-detrope/data/x.md\n";
  let found = skill_refs_in(text);
  assert_eq!(
    found,
    vec!["in-session".to_string()],
    "the path-aware guard is wrong in one direction or the other -- it must find \
     the invocation and ignore both paths, and it returned {found:?}"
  );
}

/// **AND THE RULE SCANNER MUST NOT INVENT IDS EITHER.** The shape is narrow on
/// purpose: prose about rules is full of near-misses, and each one this accepted
/// would be a standing red.
#[test]
fn the_rule_scanner_takes_the_shape_and_nothing_adjacent() {
  let text = "See IN-AG-PFIC-001 and IN-EX-CODE-006.\n\
              Not these: IN-AGENT-001, IN-AG-PFIC-0001, XIN-AG-PFIC-001, IN-AG-PFIC-00.\n";
  assert_eq!(
    rule_ids_in(text),
    vec!["IN-AG-PFIC-001".to_string(), "IN-EX-CODE-006".to_string()],
    "the rule-id scanner accepted a near-miss or dropped a real id"
  );
}
