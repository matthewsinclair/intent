//! `AT-00.1` (ST0065) / `AC-00.1`: **a rule has exactly one home, and a canon
//! document that needs another's rule POINTS rather than copies.**
//!
//! The rule library owns the text; a canon document cites the ID. `CLAUDE.md`
//! argues this against itself -- the four cross-language principles are stated
//! in `AGENTS.md` and deliberately not restated, *because a second copy would be
//! a Highlander violation in the document that defines the rule.*
//!
//! # WHAT THIS CATCHES, AND THE HALF IT CANNOT -- STATED FIRST BECAUSE IT IS THE
//! HONEST LIMIT OF THE ROW
//!
//! It catches **VERBATIM reproduction**: a canon document carrying a span of a
//! rule's own summary. **It does NOT catch a PARAPHRASE, and the row's own note
//! asked for exactly that** -- *prose forks by rewording, not by duplication*.
//! That is true, and it is not mechanisable here: deciding whether two
//! differently-worded paragraphs state one rule is a judgement about meaning,
//! which is the kind `AC-00.2` and `AC-00.3` were minted NON-TEST for.
//!
//! **SO A GREEN HERE IS NOT EVIDENCE THAT NO RULE HAS TWO HOMES.** It is
//! evidence that none has two homes by copy-paste, which is the cheap half.
//! Recorded rather than glossed, and raised with vc, because a test whose stated
//! purpose exceeds its reach is the vacuous instrument wearing a criterion's
//! name.
//!
//! # Why a span rather than the whole summary
//!
//! A document quoting a rule in full is the easy case and nobody does it. The
//! failure that actually happens is a paragraph lifted out of a rule and
//! re-homed, so the check is a sliding window over the summary's words: any
//! twelve consecutive words appearing in a canon document is a copy.

use std::fs;
use std::path::{Path, PathBuf};

use testkit::repo_root;

/// The words of one rule's summary, and its id.
#[derive(Debug, Clone)]
struct Rule {
  id: String,
  summary: Vec<String>,
}

/// **HAND-PARSED, FOR THE REASON THE WHITEBOARD HEADER IS**: these files are
/// hand-written and a strict reader that failed on one would silently drop a
/// rule from the population, which reads in the result as a rule nobody copied.
fn rules_under(dir: &Path) -> Vec<Rule> {
  fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
      return;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        walk(&path, out);
      } else if path.file_name().is_some_and(|n| n == "RULE.md") {
        out.push(path);
      }
    }
  }
  let mut files = Vec::new();
  walk(dir, &mut files);
  files.sort();

  let mut out = Vec::new();
  for file in files {
    let Ok(text) = fs::read_to_string(&file) else {
      continue;
    };
    let Some(id) = text
      .lines()
      .find_map(|l| l.strip_prefix("id:").map(|v| v.trim().to_string()))
    else {
      continue;
    };
    // `summary: >` followed by indented continuation lines.
    let mut summary = Vec::new();
    let mut inside = false;
    for line in text.lines() {
      if line.starts_with("summary:") {
        inside = true;
        continue;
      }
      if inside {
        if line.starts_with([' ', '\t']) {
          summary.extend(line.split_whitespace().map(str::to_string));
        } else {
          break;
        }
      }
    }
    out.push(Rule { id, summary });
  }
  out
}

/// Every canon document a project is given.
fn canon_docs(root: &Path) -> Vec<PathBuf> {
  ["AGENTS.md", "CLAUDE.md", "usage-rules.md"]
    .iter()
    .map(|n| root.join(n))
    .filter(|p| p.is_file())
    .collect()
}

/// How many words of a rule must appear consecutively before it is a copy.
const SPAN: usize = 12;

/// Rules whose own words appear inside `text`. **A PURE FUNCTION**, so it can be
/// driven against a planted document where the answer is known -- an assertion
/// written straight over the real tree can only be observed passing.
fn copied_into(text: &str, rules: &[Rule]) -> Vec<String> {
  let flat: String = format!(
    " {} ",
    text.split_whitespace().collect::<Vec<_>>().join(" ")
  );
  let mut out = Vec::new();
  for rule in rules {
    if rule.summary.len() < SPAN {
      continue;
    }
    let copied = rule
      .summary
      .windows(SPAN)
      .any(|w| flat.contains(&format!(" {} ", w.join(" "))));
    if copied {
      out.push(rule.id.clone());
    }
  }
  out
}

#[test]
fn no_canon_document_reproduces_a_rules_own_words() {
  let root = repo_root();
  let rules = rules_under(&root.join("intent/plugins/claude/rules"));
  assert!(
    rules.len() > 10,
    "only {} rules were parsed -- a population this small means the parse \
     failed, and an empty population agrees with every document ever written",
    rules.len()
  );
  assert!(
    rules.iter().filter(|r| r.summary.len() >= SPAN).count() > 10,
    "almost no rule has a summary long enough to check, so this test is \
     asserting over a population it cannot see"
  );

  let docs = canon_docs(&root);
  assert!(!docs.is_empty(), "no canon documents were found to check");
  for doc in &docs {
    let text = fs::read_to_string(doc).expect("a canon document reads");
    let copied = copied_into(&text, &rules);
    assert!(
      copied.is_empty(),
      "{} reproduces the words of {copied:?} -- the rule library owns that text \
       and a canon document must cite the id instead. Two copies agree on the \
       day they are written and drift after, invisibly, because both read fine",
      doc.display()
    );
  }
}

/// **THE CONTROL: A PLANTED COPY MUST FIRE.** Without this the assertion above
/// passes for a `copied_into` that returns nothing at all, which is the shape a
/// parse failure produces silently.
#[test]
fn a_document_carrying_a_rules_own_words_is_found() {
  let rules = vec![Rule {
    id: "IN-XX-TEST-001".into(),
    summary: "never duplicate code paths modules or logic for the same concern two implementations drift over time"
      .split_whitespace()
      .map(str::to_string)
      .collect(),
  }];
  let innocent = "This document cites IN-XX-TEST-001 and says nothing else about it.";
  assert!(
    copied_into(innocent, &rules).is_empty(),
    "a document that merely CITES the rule is reported as copying it, which \
     would make the check fire on the very thing the criterion asks for"
  );

  let copying = "Some preamble. never duplicate code paths modules or logic for the same concern two implementations drift over time. Some more.";
  assert_eq!(
    copied_into(copying, &rules),
    vec!["IN-XX-TEST-001".to_string()],
    "a document reproducing the rule's own words is not reported"
  );
}

/// **AND CITING THE ID IS THE BEHAVIOUR THE ROW WANTS, SO IT IS ASSERTED
/// POSITIVELY RATHER THAN LEFT AS THE ABSENCE OF A FAILURE.** The agnostic rule
/// ids appear in the canon set, which is what makes the pointer real: a
/// document that neither copies nor cites is not compliant, it is silent.
#[test]
fn the_canon_documents_cite_rules_by_id() {
  let root = repo_root();
  let cited: String = canon_docs(&root)
    .iter()
    .filter_map(|p| fs::read_to_string(p).ok())
    .collect();
  let rules = rules_under(&root.join("intent/plugins/claude/rules"));
  let mentioned = rules.iter().filter(|r| cited.contains(&r.id)).count();
  assert!(
    mentioned > 0,
    "no canon document cites any rule by id, so `points rather than copies` is \
     satisfied only by saying nothing at all"
  );
}
