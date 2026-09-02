//! `AT-00.4` (ST0065) / `AC-00.4`: **a surviving skill has a live consumer, and
//! liveness is TRANSITIVE.**
//!
//! **THE TRANSITIVE DIRECTION IS THE ONE NOBODY CHECKS AND IT IS WHY vc SPLIT
//! THIS OFF `AC-00.5`.** A skill with a live caller is still dead if what it
//! chains TO was retired -- and the caller looks perfectly healthy throughout,
//! because nothing it declares is wrong. Buried under the retirement half, that
//! direction had no instrument and would have been swept by a reader testing
//! the obvious side.
//!
//! # Two routes to liveness, and both are legitimate
//!
//! **A skill is reached either by another skill's `chains_to`, or by being
//! INVOKED BY NAME in a canon document** (`/in-review` in `usage-rules.md`, say).
//! Measured before the predicate was written: seven of the shipped skills are
//! chained to by nothing at all -- `in-session`, `in-review`, `in-plan`,
//! `in-debug`, `in-autopsy`, `in-cost-analysis`, `in-tca-init` -- and every one
//! of them is an ENTRY POINT an operator types. **A checker that knew only
//! `chains_to` would report seven dead skills, all of them false**, and a
//! checker with seven standing false positives is one nobody runs twice.
//!
//! # What this cannot see, stated so a green is not read as more
//!
//! It establishes that something REFERS to each skill, not that the reference
//! is honoured at run time. A canon document naming `/in-review` in a sentence
//! nobody follows still counts as a caller here. **That is the honest limit of
//! a static check over prose**, and `AC-00.2` -- can an agent actually execute
//! the instruction -- is the row that covers the rest of it, by a read.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use testkit::repo_root;

/// One skill, as this check needs it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Skill {
  name: String,
  chains_to: Vec<String>,
}

/// Why a skill is not live.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Finding {
  /// `from` chains to `to`, and `to` is not a skill that exists.
  Dangling { from: String, to: String },
  /// Nothing chains to it and no canon document invokes it by name.
  Unreached { name: String },
}

/// The `chains_to` list out of a `SKILL.md`'s frontmatter.
///
/// **HAND-PARSED RATHER THAN THROUGH A YAML READER, AND THE REASON IS THE SAME
/// ONE THE WHITEBOARD HEADER GIVES**: this frontmatter is hand-written prose
/// and the tool has never required it to be valid YAML. A strict parser would
/// fail on a block this check is meant to read, and failing to read one is
/// indistinguishable in the result from a skill that chains to nothing.
fn chains_of(front_matter: &str) -> Vec<String> {
  let Some(at) = front_matter.find("chains_to:") else {
    return Vec::new();
  };
  let rest = &front_matter[at..];
  let Some(open) = rest.find('[') else {
    return Vec::new();
  };
  let Some(close) = rest[open..].find(']') else {
    return Vec::new();
  };
  rest[open..open + close]
    .split('"')
    .skip(1)
    .step_by(2)
    .map(str::to_string)
    .collect()
}

/// **THE POPULATION IS DERIVED FROM THE TREE, NEVER LISTED** -- `AC-00.7`'s
/// property, relied on here: a skill added tomorrow is in this check without
/// anyone editing it.
fn skills_under(dir: &Path) -> Vec<Skill> {
  let mut out = Vec::new();
  let Ok(entries) = fs::read_dir(dir) else {
    return out;
  };
  let mut names: Vec<String> = entries
    .flatten()
    .filter(|e| e.path().is_dir())
    .map(|e| e.file_name().to_string_lossy().to_string())
    .collect();
  names.sort();
  for name in names {
    let manifest = dir.join(&name).join("SKILL.md");
    let Ok(text) = fs::read_to_string(&manifest) else {
      continue;
    };
    let front = text
      .strip_prefix("---\n")
      .and_then(|t| t.split_once("\n---"))
      .map(|(f, _)| f)
      .unwrap_or("");
    out.push(Skill {
      name,
      chains_to: chains_of(front),
    });
  }
  out
}

/// Every `/name` invocation appearing in the given files.
fn invocations_in(files: &[PathBuf]) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  for file in files {
    let Ok(text) = fs::read_to_string(file) else {
      continue;
    };
    for piece in text.split('/').skip(1) {
      let name: String = piece
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();
      if name.starts_with("in-") && name.len() > 3 {
        out.insert(name);
      }
    }
  }
  out
}

/// **THE WHOLE CHECK, AS A PURE FUNCTION**, so it can be driven against planted
/// sets where the answer is known. An assertion written straight over the real
/// tree can only ever be observed passing.
fn dead(skills: &[Skill], invoked: &BTreeSet<String>) -> Vec<Finding> {
  let present: BTreeSet<&str> = skills.iter().map(|s| s.name.as_str()).collect();
  let mut out = Vec::new();
  for skill in skills {
    for target in &skill.chains_to {
      if !present.contains(target.as_str()) {
        out.push(Finding::Dangling {
          from: skill.name.clone(),
          to: target.clone(),
        });
      }
    }
  }
  let chained: BTreeSet<&str> = skills
    .iter()
    .flat_map(|s| s.chains_to.iter())
    .map(String::as_str)
    .collect();
  for skill in skills {
    if !chained.contains(skill.name.as_str()) && !invoked.contains(&skill.name) {
      out.push(Finding::Unreached {
        name: skill.name.clone(),
      });
    }
  }
  out
}

fn canon_docs(root: &Path) -> Vec<PathBuf> {
  let mut out = vec![
    root.join("AGENTS.md"),
    root.join("CLAUDE.md"),
    root.join("usage-rules.md"),
  ];
  let skills = root.join("intent/plugins/claude/skills");
  if let Ok(entries) = fs::read_dir(&skills) {
    for entry in entries.flatten() {
      out.push(entry.path().join("SKILL.md"));
    }
  }
  out
}

#[test]
fn every_shipped_skill_is_reached_and_every_chain_resolves() {
  let root = repo_root();
  let skills = skills_under(&root.join("intent/plugins/claude/skills"));
  assert!(
    skills.len() > 1,
    "the skill population came back as {} -- a check whose population is empty \
     agrees with everything",
    skills.len()
  );
  assert!(
    skills.iter().any(|s| !s.chains_to.is_empty()),
    "no skill declares a `chains_to`, so the transitive half of this check is \
     asserting nothing"
  );

  let invoked = invocations_in(&canon_docs(&root));
  let findings = dead(&skills, &invoked);
  assert!(
    findings.is_empty(),
    "skills are declared and not reachable, or chain to something that does not \
     exist: {findings:#?}"
  );
}

/// **CONTROL ONE: A CHAIN TO A SKILL THAT DOES NOT EXIST MUST FIRE.** This is
/// the transitive direction, and it is the reason the row was split off the
/// retirement criterion -- the chaining skill is healthy in every other respect.
#[test]
fn a_chain_to_a_skill_that_does_not_exist_is_found() {
  let skills = vec![
    Skill {
      name: "in-alpha".into(),
      chains_to: vec!["in-retired".into()],
    },
    Skill {
      name: "in-beta".into(),
      chains_to: Vec::new(),
    },
  ];
  let invoked: BTreeSet<String> = ["in-alpha".to_string(), "in-beta".to_string()]
    .into_iter()
    .collect();
  assert_eq!(
    dead(&skills, &invoked),
    vec![Finding::Dangling {
      from: "in-alpha".into(),
      to: "in-retired".into()
    }],
    "a skill chaining to one that was retired is not reported"
  );
}

/// **CONTROL TWO: A SKILL NOTHING REACHES MUST FIRE** -- and it is the half a
/// checker walking only FROM the roster would find, which is why control one
/// exists beside it. One without the other is an instrument with a blind side.
#[test]
fn a_skill_nothing_reaches_is_found() {
  let skills = vec![
    Skill {
      name: "in-alpha".into(),
      chains_to: vec!["in-beta".into()],
    },
    Skill {
      name: "in-beta".into(),
      chains_to: Vec::new(),
    },
    Skill {
      name: "in-orphan".into(),
      chains_to: Vec::new(),
    },
  ];
  // `in-alpha` is invoked by name; `in-orphan` is invoked by nobody and chained
  // to by nobody.
  let invoked: BTreeSet<String> = ["in-alpha".to_string()].into_iter().collect();
  assert_eq!(
    dead(&skills, &invoked),
    vec![Finding::Unreached {
      name: "in-orphan".into()
    }],
    "a skill nothing chains to and nothing invokes is not reported"
  );
}

/// **AND THE PREDICATE MUST NOT CALL AN ENTRY POINT DEAD.** Seven shipped
/// skills are chained to by nothing and reached only by being typed; a checker
/// that knew only `chains_to` would report all seven, and standing false
/// positives are how a check stops being run.
#[test]
fn a_skill_reached_only_by_being_invoked_by_name_is_live() {
  let skills = vec![Skill {
    name: "in-entry".into(),
    chains_to: Vec::new(),
  }];
  let invoked: BTreeSet<String> = ["in-entry".to_string()].into_iter().collect();
  assert!(
    dead(&skills, &invoked).is_empty(),
    "an entry point invoked by name is reported dead, which is the false \
     positive that would retire this check"
  );
  assert_eq!(
    dead(&skills, &BTreeSet::new()),
    vec![Finding::Unreached {
      name: "in-entry".into()
    }],
    "with the invocation removed it must be reported -- otherwise the arm above \
     passes for a predicate that never reports anything"
  );
}
