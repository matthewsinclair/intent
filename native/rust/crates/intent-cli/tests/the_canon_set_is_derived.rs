//! `AT-00.7` (ST0065) / `AC-00.7`: **the canon set is enumerable by DERIVATION
//! from the tree, never by a hand-written list.**
//!
//! **THIS IS A CLAIM ABOUT THE SET, LIVE AND CHECKABLE AT ANY MOMENT, RATHER
//! THAN A CLAIM ABOUT ANY REVIEW.** vc's amendment, and it is what makes the row
//! mintable at all: *we reviewed them all* is retrospective and unfalsifiable,
//! *the population is derivable* is neither.
//!
//! # The control is the whole instrument
//!
//! A derivation and a hand-written list agree exactly as long as the list is
//! current, **which is the state a list is in right up until the moment it is
//! wrong** -- so an assertion that today's enumeration matches today's tree
//! passes for both and distinguishes nothing. The arm that discriminates is
//! **add a member to a planted tree and require the enumeration to grow with no
//! edit to the code.** A hand-written list cannot pass that; a derivation cannot
//! fail it.
//!
//! # Reach, declared
//!
//! This establishes that the SET is derivable, not that every consumer of it
//! derives rather than lists. `/in-session`'s language table, for one, is
//! hand-written prose -- a legitimate finding for `AC-00.2`'s read rather than
//! something this check can see, because whether a table is a stale copy or a
//! deliberate mapping is a judgement about meaning.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use testkit::repo_root;

/// The canon set: the bootstrap documents a project is given, plus the skills.
///
/// **DERIVED BY WALKING, AND THE ONLY LITERALS ARE THE TWO PLACES TO LOOK** --
/// the root for documents, the skills directory for skills. A skill added
/// tomorrow is in this set with nothing edited, which is the property.
fn canon_set(root: &Path) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  if let Ok(entries) = fs::read_dir(root) {
    for entry in entries.flatten() {
      let path = entry.path();
      if path.extension().is_some_and(|e| e == "md") && path.is_file() {
        out.insert(format!(
          "doc:{}",
          path.file_name().unwrap_or_default().to_string_lossy()
        ));
      }
    }
  }
  if let Ok(entries) = fs::read_dir(root.join("intent/plugins/claude/skills")) {
    for entry in entries.flatten() {
      if entry.path().join("SKILL.md").is_file() {
        out.insert(format!("skill:{}", entry.file_name().to_string_lossy()));
      }
    }
  }
  out
}

#[test]
fn the_shipped_canon_set_derives_and_carries_the_documents_and_the_skills() {
  let root = repo_root();
  let set = canon_set(&root);
  assert!(
    set.len() > 1,
    "the derived canon set came back as {} -- an enumeration that finds nothing \
     agrees with every coverage claim ever made",
    set.len()
  );
  for expected in ["doc:AGENTS.md", "doc:CLAUDE.md", "doc:usage-rules.md"] {
    assert!(
      set.contains(expected),
      "`{expected}` is a canon bootstrap document and the derivation does not \
       find it, so any coverage claim over this set has a hole exactly its size"
    );
  }
  assert!(
    set.iter().filter(|m| m.starts_with("skill:")).count() > 1,
    "the derivation found no skills, so half the set it exists to enumerate is \
     invisible to it"
  );
}

/// **THE CONTROL, AND IT IS THE ROW.** Against a planted tree the enumeration
/// must GROW when a member is added, with nothing edited. A hand-written list
/// -- the thing `AC-00.7` refuses -- fails this arm by construction, and passes
/// every other arm in this file while it happens to be current.
#[test]
fn a_member_added_to_the_tree_appears_without_the_code_being_edited() {
  let tmp = tempfile::tempdir().expect("fixture");
  let root = tmp.path();
  let skills = root.join("intent/plugins/claude/skills");
  fs::create_dir_all(skills.join("in-one")).expect("mk");
  fs::write(skills.join("in-one/SKILL.md"), "# one\n").expect("write");
  fs::write(root.join("AGENTS.md"), "# agents\n").expect("write");

  let before = canon_set(root);
  assert_eq!(
    before,
    ["doc:AGENTS.md".to_string(), "skill:in-one".to_string()]
      .into_iter()
      .collect::<BTreeSet<String>>(),
    "the planted baseline is not what was planted, so the growth below would be \
     measuring the fixture rather than the derivation"
  );

  // Two members added -- one of each kind, because a derivation could plausibly
  // walk one and list the other.
  fs::create_dir_all(skills.join("in-two")).expect("mk");
  fs::write(skills.join("in-two/SKILL.md"), "# two\n").expect("write");
  fs::write(root.join("CLAUDE.md"), "# claude\n").expect("write");

  let after = canon_set(root);
  let grew: Vec<&String> = after.difference(&before).collect();
  assert_eq!(
    grew,
    vec![&"doc:CLAUDE.md".to_string(), &"skill:in-two".to_string()],
    "the enumeration did not grow by exactly the two members added -- a set that \
     does not track the tree is a list wearing a derivation's name"
  );
}

/// **AND A MEMBER REMOVED MUST LEAVE**, which is the direction a stale list gets
/// wrong in the other way: it goes on naming something that is gone, and a
/// coverage claim over it then reports examining a file that does not exist.
#[test]
fn a_member_removed_from_the_tree_leaves_the_set() {
  let tmp = tempfile::tempdir().expect("fixture");
  let root = tmp.path();
  let skills = root.join("intent/plugins/claude/skills");
  fs::create_dir_all(skills.join("in-doomed")).expect("mk");
  fs::write(skills.join("in-doomed/SKILL.md"), "# doomed\n").expect("write");
  assert!(canon_set(root).contains("skill:in-doomed"));

  fs::remove_dir_all(skills.join("in-doomed")).expect("rm");
  assert!(
    !canon_set(root).contains("skill:in-doomed"),
    "a removed skill is still enumerated, so the set outlives the tree"
  );
}
