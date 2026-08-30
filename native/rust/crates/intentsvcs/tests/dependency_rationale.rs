//! AT-08.10 / AC-08.10: every dependency `intentd` declares carries a written
//! rationale, and it lives beside the declaration that pins the version.
//!
//! **THIS IS THE HALF NOTHING CHECKED.** AC-08.10 has two obligations. The
//! second -- D06 survives every addition, so `intentd` never reaches a SQLite
//! driver -- is enforced by `dep_graph_guard.rs`, and this file CITES that
//! rather than growing a second copy of it (see the last test, which checks the
//! citation is live rather than re-deriving the rule). The first -- that each
//! addition carries a written argument -- was a convention held only by whoever
//! last read the manifest, and a convention is exactly what stops being true
//! the first time somebody adds a dependency in a hurry.
//!
//! **THE POPULATION IS `intentd`'S OWN DECLARATIONS, NOT THE WORKSPACE'S.**
//! AC-08.10 says *each dependency added to `intentd`*, and the difference is
//! load-bearing rather than pedantic: `serde`, `ulid`, `time`, `thiserror` and
//! others are pinned under `[workspace.dependencies]` with no rationale at all,
//! and they are not in scope. A check written over the workspace would go red
//! on its first run against eight innocent rows, and the cheap way to make it
//! pass would be to weaken it -- which is the failure mode where a real
//! criterion gets laundered through a green.
//!
//! **WHERE THE RATIONALE MUST LIVE IS DERIVED, NOT DECREED: it goes beside the
//! declaration that PINS THE VERSION.** `intentd` inherits both of its current
//! dependencies with `.workspace = true`, so their versions are pinned in the
//! root manifest and their arguments live there too -- one home, so a reader
//! who changes the version meets the argument for it. A dependency `intentd`
//! pinned itself would be argued in `intentd/Cargo.toml`. The check follows
//! whichever manifest the declaration points at rather than looking in a fixed
//! place.
//!
//! **WHAT THIS CANNOT DO, STATED SO NOBODY READS ITS GREEN AS MORE THAN IT IS:
//! it detects ABSENCE, never quality.** A comment block that argues badly
//! passes, and no mechanical check could rule otherwise -- judging whether an
//! argument is good is review, and review is vc's. What it catches is the case
//! that actually happens: a line added with nothing above it.
//!
//! The adjacency rule is strict -- the rationale is the contiguous run of
//! comment lines immediately above the declaration, with no blank line between.
//! A blank line makes this go red, and that is the intended direction: the
//! property being enforced is that the argument sits AGAINST the thing, and a
//! false red costs one moved comment while a false green costs the rule.

use std::fs;
use std::path::Path;
use testkit::workspace_root;

// ---------------------------------------------------------------------------
// The mechanism. It takes manifest CONTENT rather than paths, so the positive
// controls below can drive it to both verdicts on planted fixtures instead of
// asserting against a repository they would have to break first.
// ---------------------------------------------------------------------------

/// The TOML sections whose entries are dependency declarations.
const DEPENDENCY_SECTIONS: &[&str] = &["dependencies", "dev-dependencies", "build-dependencies"];

/// One dependency a manifest declares, and whether its version is pinned here
/// or inherited from the workspace.
#[derive(Debug, PartialEq, Eq)]
struct Declared {
  name: String,
  inherits_workspace: bool,
}

/// Does this line declare `name`?
///
/// Compares the whole key rather than a prefix, so `serde` does not match
/// `serde_json`, and takes the segment before any `.` so that both
/// `tokio = ...` and `tokio.workspace = ...` answer for `tokio`.
fn declares(line: &str, name: &str) -> bool {
  let trimmed = line.trim();
  if trimmed.starts_with('#') {
    return false;
  }
  match trimmed.split_once('=') {
    Some((lhs, _)) => lhs.trim().split('.').next().map(str::trim) == Some(name),
    None => false,
  }
}

/// Every dependency the manifest declares, deduplicated by name.
///
/// **IT PANICS ON A LINE IT CANNOT CLASSIFY RATHER THAN SKIPPING IT.** A parser
/// that silently drops what it does not understand reports an empty population
/// as a clean one, and a check whose corpus is empty passes for free. Refusing
/// is the cheapest instrument there is: it cannot be wrong quietly.
fn declared_dependencies(manifest: &str) -> Vec<Declared> {
  let mut found: Vec<Declared> = Vec::new();
  let mut in_section = false;

  for raw in manifest.lines() {
    let line = raw.trim();

    if line.starts_with('[') {
      let header = line.trim_start_matches('[').trim_end_matches(']');
      // `[dependencies.foo]` declares `foo` and then opens a table of ITS keys,
      // none of which are declarations, so the section flag closes behind it.
      if let Some((head, sub)) = header.split_once('.') {
        if DEPENDENCY_SECTIONS.contains(&head) {
          push_unique(&mut found, sub, false);
          in_section = false;
          continue;
        }
      }
      in_section = DEPENDENCY_SECTIONS.contains(&header);
      continue;
    }

    if !in_section || line.is_empty() || line.starts_with('#') {
      continue;
    }

    let Some((lhs, rhs)) = line.split_once('=') else {
      panic!(
        "a line in a dependency section is neither a table header nor a `key = value`, so this check could not read the manifest: {raw:?}"
      );
    };
    let lhs = lhs.trim();
    let inherits = match lhs.split_once('.') {
      Some((_, "workspace")) => rhs.trim() == "true",
      _ => rhs.replace(' ', "").contains("workspace=true"),
    };
    let name = lhs.split('.').next().expect("a non-empty key").trim();
    push_unique(&mut found, name, inherits);
  }

  found
}

/// Record `name`, OR-ing `inherits` into an entry that is already there.
///
/// A dependency written across several keys (`foo.workspace`, `foo.features`)
/// is one declaration, and only one of those lines carries the inheritance.
fn push_unique(found: &mut Vec<Declared>, name: &str, inherits: bool) {
  if let Some(existing) = found.iter_mut().find(|d| d.name == name) {
    existing.inherits_workspace |= inherits;
    return;
  }
  found.push(Declared {
    name: name.to_string(),
    inherits_workspace: inherits,
  });
}

/// The contiguous comment block immediately above `name`'s declaration.
///
/// `None` means the declaration is not in this manifest at all, which is a
/// different answer from an empty block and is reported differently below.
fn rationale_above(manifest: &str, name: &str) -> Option<String> {
  let lines: Vec<&str> = manifest.lines().collect();
  let at = lines.iter().position(|l| declares(l, name))?;
  let mut block: Vec<&str> = Vec::new();
  for line in lines[..at].iter().rev() {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
      break;
    }
    block.push(trimmed.trim_start_matches('#').trim());
  }
  block.reverse();
  Some(block.join(" "))
}

/// Does this block say anything beyond naming the dependency?
///
/// The floor is derived from the content rather than pinned to a length: a
/// rationale must contain at least one word that is not the dependency's own
/// name. That rules out both the empty block and `# tokio`, and it needs no
/// magic number -- a threshold on words or characters would be a claim about
/// how long a good argument is, which is not a claim this file can support.
fn is_a_rationale(block: &str, name: &str) -> bool {
  block
    .split(|c: char| !c.is_alphanumeric() && c != '_')
    .filter(|word| word.chars().any(char::is_alphabetic))
    .any(|word| !word.eq_ignore_ascii_case(name))
}

// ---------------------------------------------------------------------------
// The assertion, against the real manifests.
// ---------------------------------------------------------------------------

/// Every dependency `member` declares that carries no written rationale.
///
/// **ONE IMPLEMENTATION, TWO CALL SITES.** The assertion below runs it on the
/// real manifests and requires the answer to be empty; the control after that
/// runs it on the same real manifests with one unargued dependency spliced in
/// and requires the answer to name it. A control that re-implemented this would
/// be asserting that a copy agrees with itself.
fn unargued_dependencies(member: &str, workspace: &str, member_label: &str) -> Vec<String> {
  let mut out = Vec::new();
  for dep in declared_dependencies(member) {
    let (home, label) = if dep.inherits_workspace {
      (workspace, "the workspace manifest")
    } else {
      (member, member_label)
    };
    match rationale_above(home, &dep.name) {
      None => out.push(format!(
        "{}: inherited from the workspace, but {} pins no version for it",
        dep.name, label
      )),
      Some(block) if !is_a_rationale(&block, &dep.name) => out.push(format!(
        "{}: no written rationale in the comment block above its declaration in {label}",
        dep.name
      )),
      Some(_) => {}
    }
  }
  out
}

/// The two manifests the check reads, and the label for the member's own.
fn real_manifests() -> (String, String, &'static str) {
  let root = workspace_root();
  (
    read(&root.join("crates").join("intentd").join("Cargo.toml")),
    read(&root.join("Cargo.toml")),
    "intentd/Cargo.toml",
  )
}

#[test]
fn every_dependency_intentd_declares_carries_a_written_rationale() {
  let (intentd, workspace, label) = real_manifests();

  // **AN EMPTY POPULATION IS A FAILURE TO MEASURE, NOT A PASS.** If the
  // manifest is moved, renamed, or restructured into a form the parser reads as
  // sectionless, the assertion below becomes vacuous and this test goes green
  // while checking nothing. `read` panics on a missing file for the same
  // reason: neither absence nor unreadability may arrive here as a clean sheet.
  let declared = declared_dependencies(&intentd);
  assert!(
    !declared.is_empty(),
    "no dependency declarations were found in {label} -- this check measured nothing and must not be read as a pass"
  );

  let unargued = unargued_dependencies(&intentd, &workspace, label);
  assert!(
    unargued.is_empty(),
    "AC-08.10: every dependency intentd declares carries a written rationale beside the declaration that pins its version.\n  {}",
    unargued.join("\n  ")
  );
}

#[test]
fn the_check_finds_an_unargued_dependency_spliced_into_the_real_manifest() {
  // **THE MUTATION IS OF THE SUBJECT, NOT OF A FIXTURE.** The fixtures below
  // show the mechanism flips; they say nothing about it flipping on the corpus
  // this criterion is actually about. So this takes intentd's real manifest,
  // adds the exact line a hurried addition would add -- a dependency with
  // nothing above it -- and requires the check to name it.
  //
  // It is spliced IN MEMORY rather than written to disk deliberately: the
  // manifests are compiled in and shared by four sessions, so mutating one on
  // disk puts every peer's build in an unexplained state for as long as the
  // mutation stands. What that leaves unproven is only the path joins, and
  // those panic rather than passing quietly.
  let (intentd, workspace, label) = real_manifests();
  assert!(
    unargued_dependencies(&intentd, &workspace, label).is_empty(),
    "the unmutated manifest is the baseline and must be clean before the mutation means anything"
  );

  let mutated = intentd.replace("[dependencies]", "[dependencies]\nhurried-addition = \"1\"");
  assert_ne!(
    mutated, intentd,
    "the mutation must actually change the source before any verdict from it counts"
  );

  let found = unargued_dependencies(&mutated, &workspace, label);
  assert_eq!(
    found.len(),
    1,
    "exactly the spliced dependency should be unargued, not a cascade: {found:?}"
  );
  assert!(
    found[0].starts_with("hurried-addition:"),
    "the refusal names the offending dependency: {found:?}"
  );
}

#[test]
fn splicing_an_argued_dependency_into_the_real_manifest_stays_clean() {
  // **THE CONTROL THAT MAKES THE ONE ABOVE MEAN SOMETHING.** Without it, the
  // red could come from splicing ANYTHING in -- a check that refused every
  // addition would pass that test and be useless. This adds a dependency the
  // same way and argues it, and requires the answer to stay empty.
  let (intentd, workspace, label) = real_manifests();
  let mutated = intentd.replace(
    "[dependencies]",
    "[dependencies]\n# Serves the HTTP half of D56; the socket half needs no framework.\nhurried-addition = \"1\"",
  );
  assert_ne!(
    mutated, intentd,
    "the mutation must actually change the source"
  );
  assert!(
    unargued_dependencies(&mutated, &workspace, label).is_empty(),
    "an addition that carries its argument is not what this check refuses"
  );
}

// ---------------------------------------------------------------------------
// Positive controls. The check above is only worth its green if it is SHOWN to
// go red, so each fixture below is a manifest that differs from a passing one
// in exactly the property under test.
// ---------------------------------------------------------------------------

const ARGUED: &str = "\
[dependencies]
# Serves the HTTP half, because the socket half needs no framework at all.
axum = \"0.8\"
";

const UNARGUED: &str = "\
[dependencies]
axum = \"0.8\"
";

#[test]
fn the_fixtures_differ_only_in_the_property_under_test() {
  // **THE CONTROL FOR THE CONTROLS.** A pair of fixtures that differ in some
  // OTHER way would drive both verdicts while proving nothing about rationale,
  // which is this estate's most-repeated instrument failure: the good fixture
  // and the bad fixture have to be shown to differ where the claim says.
  assert_eq!(
    declared_dependencies(ARGUED),
    declared_dependencies(UNARGUED),
    "both fixtures declare the same dependency; only the comment block differs"
  );
}

#[test]
fn a_dependency_with_a_written_rationale_passes() {
  let block = rationale_above(ARGUED, "axum").expect("axum is declared");
  assert!(
    is_a_rationale(&block, "axum"),
    "a comment block arguing the addition is a rationale"
  );
}

#[test]
fn a_dependency_added_with_nothing_above_it_is_refused() {
  let block = rationale_above(UNARGUED, "axum").expect("axum is declared");
  assert!(
    !is_a_rationale(&block, "axum"),
    "an addition with no comment block is exactly the case this check exists for"
  );
}

#[test]
fn a_bare_restatement_of_the_name_is_not_a_rationale() {
  // Naming the thing is not arguing for it, and this is the form a check with a
  // presence-only floor would wave through.
  let manifest = "[dependencies]\n# axum\naxum = \"0.8\"\n";
  let block = rationale_above(manifest, "axum").expect("axum is declared");
  assert!(!block.is_empty(), "the block is present");
  assert!(
    !is_a_rationale(&block, "axum"),
    "a block that only repeats the dependency's name says nothing about it"
  );
}

#[test]
fn a_blank_line_breaks_adjacency_and_that_is_deliberate() {
  let manifest = "[dependencies]\n# Serves the HTTP half of D56.\n\naxum = \"0.8\"\n";
  let block = rationale_above(manifest, "axum").expect("axum is declared");
  assert!(
    !is_a_rationale(&block, "axum"),
    "the rationale must sit AGAINST the declaration; a false red here costs one moved comment"
  );
}

#[test]
fn an_inherited_dependency_is_argued_in_the_workspace_manifest() {
  let member = "[dependencies]\ntokio.workspace = true\n";
  let declared = declared_dependencies(member);
  assert_eq!(
    declared,
    vec![Declared {
      name: "tokio".to_string(),
      inherits_workspace: true,
    }],
    "a `.workspace = true` declaration names the dependency and points at the root manifest for its version"
  );
}

#[test]
fn a_dependency_split_across_keys_is_one_declaration() {
  let member = "[dependencies]\ntokio.workspace = true\ntokio.features = [\"macros\"]\n";
  assert_eq!(
    declared_dependencies(member),
    vec![Declared {
      name: "tokio".to_string(),
      inherits_workspace: true,
    }],
    "two lines about one dependency are one declaration, and the inheritance survives the merge"
  );
}

#[test]
fn only_dependency_sections_are_read() {
  // `version.workspace = true` under `[package]` is not a dependency, and a
  // parser that read it would report a dependency named `version` that no
  // manifest could ever argue for.
  let member =
    "[package]\nname = \"x\"\nversion.workspace = true\n\n[dependencies]\naxum = \"0.8\"\n";
  assert_eq!(
    declared_dependencies(member),
    vec![Declared {
      name: "axum".to_string(),
      inherits_workspace: false,
    }],
    "package metadata is not a dependency declaration"
  );
}

#[test]
#[should_panic(expected = "could not read the manifest")]
fn a_line_the_parser_cannot_classify_refuses_rather_than_reading_as_empty() {
  // The failure this guards against is the quiet one: an unreadable manifest
  // yielding an empty population, which passes every assertion it should fail.
  declared_dependencies("[dependencies]\nthis line is not a declaration\n");
}

// ---------------------------------------------------------------------------
// The citation. AC-08.10's D06 half is enforced elsewhere and is NOT re-derived
// here.
// ---------------------------------------------------------------------------

#[test]
fn the_cited_d06_guard_is_still_here_and_still_names_rusqlite() {
  // **A CITATION, NOT A SECOND COPY.** AC-08.10 requires that D06's rusqlite
  // prohibition survives every addition, and `dep_graph_guard.rs` already
  // enforces exactly that -- re-asserting it here would be two homes for one
  // rule, and the copy would be the one that goes stale.
  //
  // What this DOES check is that the citation is live. A reader sent to a home
  // that was deleted or renamed is the same defect walking backwards: the D06
  // half of AC-08.10 would be silently unenforced while this file still claimed
  // it was covered.
  let guard = workspace_root()
    .join("crates")
    .join("intentsvcs")
    .join("tests")
    .join("dep_graph_guard.rs");
  let body = read(&guard);
  assert!(
    body.contains("rusqlite"),
    "{} is AC-08.10's D06 enforcement and must still name the crate it confines",
    guard.display()
  );
}

fn read(path: &Path) -> String {
  fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}
