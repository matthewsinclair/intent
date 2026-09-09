//! `AT-15.2` (ST0056) / `AC-15.2`: **no surviving skill names a verb or flag the
//! shipped surface does not carry, measured against the dispatch table rather
//! than against memory of it.**
//!
//! # THE REFERENCE IS THE BINARY'S OWN PREDICATE, NOT A SECOND COPY OF IT
//!
//! What "shipped" means is decided by [`dispatch::shipped_entries`], which is
//! the same function [`crate::spine::build`] uses to build the surface. This
//! test calls it rather than re-deriving `disposition` + `target.state` in its
//! own words. **A re-derivation would be a second home for the ship rule, and
//! it would drift the first time either side changed** -- at which point this
//! check would report a green about a surface the binary does not have.
//!
//! # EXTRACTION IS STRUCTURAL, BECAUSE THE LEXICAL FORM IS UNDECIDABLE
//!
//! Skills deliberately name things they are telling you NOT to do -- `in-essentials`
//! carries `# BAD -- manual creation` blocks, and rule 8 names `intent fc`
//! precisely to forbid it. A classifier that tried to sort naming-to-instruct
//! from naming-to-forbid would be asking whether prose performs a semantic act,
//! **which `AC-00.16`'s instrument measured as not machine-decidable: three
//! calibrated spellings failed in three different directions over one corpus.**
//!
//! So this does not classify. It takes the row at its word -- `AC-15.2` says
//! **NAMES**, not "instructs" -- and reads only CODE CONTEXTS: fenced blocks and
//! inline backtick spans. That is a structural property of the document, not a
//! guess about intent. **The justification is that a reader can type anything in
//! a code span**, so a code span naming a verb that does not exist is a defect
//! whichever way the surrounding prose leans.
//!
//! # THE ZERO IS NOT BELIEVED UNTIL THE CHECK HAS FOUND A PLANT
//!
//! `AC-15.2` requires this two-sided: *it must find a planted stale reference
//! before its zero is believed*. Both verdicts are driven here on fixtures built
//! in-test, so the control cannot rot separately from the check and cannot be
//! skipped by anyone running the suite. A check that has only ever returned zero
//! is indistinguishable from one whose extractor matches nothing.
//!
//! # WHAT THIS DOES NOT COVER, STATED SO THE UNION IS OWNABLE (`AC-00.16`)
//!
//! - **PATHS and FILE LAYOUTS -- excluded on an argument, not for convenience,
//!   and this is the honest limit of the row as written.** The "names" reading
//!   that makes the verb axis decidable does NOT transfer to paths, because a
//!   path in an example is a PLACEHOLDER by nature: `in-essentials` shows
//!   `mkdir -p intent/st/ST0000` under a `# BAD` heading, and `ST0000` is a
//!   stand-in that is *supposed* not to exist. A stale path and a placeholder
//!   path are structurally identical, so an existence check would report
//!   correct prose as defective and push an author to delete it. **`AC-15.2`
//!   names four axes and two of them are not decidable by this method; that is
//!   recorded on `AT-15.2` as an open question for hv rather than resolved by
//!   an instrument amending its own criterion.**
//! - **Prose outside code formatting.** A skill naming a dead verb in running
//!   prose with no backticks is invisible to this. That is the deliberate cost
//!   of refusing a lexical classifier.
//! - **Short flags.** `-s` is as likely to be another tool's argument as
//!   Intent's, and a false positive would push an author to delete correct prose.
//! - **Argument shapes.** `intent st new` resolving is not a claim that the
//!   arguments a skill shows are correct.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use intent_cli::dispatch;
use testkit::repo_root;

const SKILLS_DIR: &str = "intent/plugins/claude/skills";

/// Floors. Both populations are asserted non-empty and above a floor BEFORE the
/// property is checked, because `{} == {}` passes for free.
const MIN_SKILLS: usize = 10;
const MIN_CLAIMS: usize = 5;
const MIN_SHIPPED: usize = 50;

/// Every code context in a markdown document: fenced blocks and inline spans.
///
/// Fences win over inline spans -- a fence's body is taken whole and not
/// re-scanned for backticks, so a fenced block containing a stray backtick
/// cannot split the rest of the document into alternating "code" and "prose".
fn code_spans(md: &str) -> Vec<String> {
  let mut out = Vec::new();
  let mut in_fence = false;
  for line in md.lines() {
    let trimmed = line.trim_start();
    if trimmed.starts_with("```") {
      in_fence = !in_fence;
      continue;
    }
    if in_fence {
      out.push(line.to_string());
      continue;
    }
    // Inline spans: take the text between paired backticks, ignoring an odd one.
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
      if chars[i] == '`' {
        if let Some(close) = (i + 1..chars.len()).find(|&j| chars[j] == '`') {
          out.push(chars[i + 1..close].iter().collect());
          i = close + 1;
          continue;
        }
        break;
      }
      i += 1;
    }
  }
  out
}

/// A token is a verb candidate only if it is a bare lowercase word. This drops
/// placeholders (`<id>`, `$VAR`, `{name}`), flags, and paths, none of which are
/// verbs and all of which would otherwise be read as one.
fn is_verb_token(t: &str) -> bool {
  !t.is_empty()
    && t
      .chars()
      .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    && t.chars().next().is_some_and(|c| c.is_ascii_lowercase())
}

/// Every `intent ...` invocation in a code span, as up to three verb tokens.
///
/// An invocation whose next token is a flag (`intent --version`) yields an empty
/// claim and is dropped: the root command with a root flag names no subcommand,
/// so there is nothing for the table to carry.
fn claims(spans: &[String]) -> BTreeSet<Vec<String>> {
  let mut out = BTreeSet::new();
  for span in spans {
    let toks: Vec<&str> = span.split_whitespace().collect();
    for (i, t) in toks.iter().enumerate() {
      // `intent` as the command word, not as part of a path like `intent/st`.
      if *t != "intent" {
        continue;
      }
      let mut claim = Vec::new();
      for t2 in toks.iter().skip(i + 1).take(3) {
        if !is_verb_token(t2) {
          break;
        }
        claim.push(t2.to_string());
      }
      if !claim.is_empty() {
        out.insert(claim);
      }
    }
  }
  out
}

/// Every `--flag` a code span attaches to an `intent` invocation, paired with
/// the claim it followed.
///
/// Short flags are deliberately excluded: `-s` in a skill is as likely to be
/// another tool's argument as Intent's, and a false positive here would push a
/// skill author to delete correct prose.
fn flag_claims(spans: &[String]) -> BTreeSet<(Vec<String>, String)> {
  let mut out = BTreeSet::new();
  for span in spans {
    let toks: Vec<&str> = span.split_whitespace().collect();
    for (i, t) in toks.iter().enumerate() {
      if *t != "intent" {
        continue;
      }
      let mut claim = Vec::new();
      let mut j = i + 1;
      while j < toks.len() && claim.len() < 3 && is_verb_token(toks[j]) {
        claim.push(toks[j].to_string());
        j += 1;
      }
      if claim.is_empty() {
        continue;
      }
      // Flags belonging to THIS invocation: up to the next `intent`, `|` or `&&`.
      while j < toks.len() {
        let t2 = toks[j];
        if t2 == "intent" || t2 == "|" || t2 == "&&" || t2 == ";" {
          break;
        }
        if let Some(name) = t2.strip_prefix("--") {
          let name = name.split('=').next().unwrap_or(name);
          if !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
            out.insert((claim.clone(), format!("--{name}")));
          }
        }
        j += 1;
      }
    }
  }
  out
}

/// Whether any declared path is a prefix of this claim.
///
/// Longest-prefix, because 99 of the table's 144 entry paths are subcommands
/// (`st new`, `claude rules show`). A claim of `st new "a thread"` resolves on
/// `st new`; a claim of `st frobnicate` resolves on `st`, which is correct --
/// the VERB `st` ships, and whether `frobnicate` is a valid subcommand of it is
/// the argument-shape question this test declares out of scope.
fn resolves(claim: &[String], shipped: &BTreeSet<String>) -> bool {
  (1..=claim.len()).any(|n| shipped.contains(&claim[..n].join(" ")))
}

/// Every claim in a skills tree that no shipped path carries.
fn stale_in_tree(dir: &Path, shipped: &BTreeSet<String>) -> BTreeSet<(String, String)> {
  let mut out = BTreeSet::new();
  let Ok(entries) = fs::read_dir(dir) else {
    return out;
  };
  for e in entries.flatten() {
    let skill = e.file_name().to_string_lossy().to_string();
    let doc = e.path().join("SKILL.md");
    if !doc.is_file() {
      continue;
    }
    let Ok(text) = fs::read_to_string(&doc) else {
      continue;
    };
    for claim in claims(&code_spans(&text)) {
      if !resolves(&claim, shipped) {
        out.insert((skill.clone(), claim.join(" ")));
      }
    }
  }
  out
}

/// Every flag claim in a skills tree that its resolved entry does not declare.
///
/// A flag is accepted if the entry the claim resolves to declares it, OR if any
/// ANCESTOR path does -- `intent st new --daemon` is legitimate when `--daemon`
/// sits on `st`. Ancestors are consulted because clap propagates a parent's
/// flags to its subcommands, so refusing them here would report correct prose
/// as stale.
fn stale_flags_in_tree(
  dir: &Path,
  shipped: &BTreeSet<String>,
  flags: &std::collections::BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<(String, String)> {
  let mut out = BTreeSet::new();
  let Ok(entries) = fs::read_dir(dir) else {
    return out;
  };
  for e in entries.flatten() {
    let skill = e.file_name().to_string_lossy().to_string();
    let doc = e.path().join("SKILL.md");
    let Ok(text) = fs::read_to_string(&doc) else {
      continue;
    };
    for (claim, flag) in flag_claims(&code_spans(&text)) {
      // Resolve to the longest shipped prefix; unresolvable verbs are the verb
      // axis's finding, not this one's, so they are skipped here rather than
      // counted twice.
      let Some(n) = (1..=claim.len())
        .rev()
        .find(|&n| shipped.contains(&claim[..n].join(" ")))
      else {
        continue;
      };
      let declared = (1..=n).any(|k| {
        flags
          .get(&claim[..k].join(" "))
          .is_some_and(|s| s.contains(&flag))
      });
      if !declared {
        out.insert((
          skill.clone(),
          format!("intent {} {flag}", claim[..n].join(" ")),
        ));
      }
    }
  }
  out
}

fn shipped_paths() -> BTreeSet<String> {
  let table = dispatch::table();
  dispatch::shipped_entries(&table)
    .into_iter()
    .map(|e| e.path.clone())
    .collect()
}

/// Shipped path -> the flag spellings it declares that actually SHIP.
///
/// A `retire`d flag is declared and never reaches clap, so a skill naming one
/// is naming something the surface does not carry -- which is exactly this
/// row's subject. Filtering on `ships()` rather than on presence is what makes
/// the difference visible.
fn shipped_flags() -> std::collections::BTreeMap<String, BTreeSet<String>> {
  let table = dispatch::table();
  dispatch::shipped_entries(&table)
    .into_iter()
    .map(|e| {
      let spellings = e
        .flags
        .iter()
        .filter(|f| f.ships())
        .flat_map(|f| f.spellings.iter().cloned())
        .filter(|s| s.starts_with("--"))
        .collect::<BTreeSet<String>>();
      (e.path.clone(), spellings)
    })
    .collect()
}

fn skill_count(dir: &Path) -> usize {
  fs::read_dir(dir)
    .map(|rd| {
      rd.flatten()
        .filter(|e| e.path().join("SKILL.md").is_file())
        .count()
    })
    .unwrap_or(0)
}

#[test]
fn invariant_no_surviving_skill_names_a_verb_the_surface_does_not_carry() {
  let root = repo_root();
  let skills = root.join(SKILLS_DIR);

  // ---- POPULATION FIRST, PROPERTY SECOND ----
  let shipped = shipped_paths();
  assert!(
    shipped.len() >= MIN_SHIPPED,
    "the REFERENCE is empty or implausibly small ({} shipped paths, floor {}). \
     Every claim would resolve to nothing and this check would report the whole \
     catalogue as stale, or -- with the assertion inverted -- pass for free. \
     Fix the table read before reading any verdict below.",
    shipped.len(),
    MIN_SHIPPED
  );

  let n_skills = skill_count(&skills);
  assert!(
    n_skills >= MIN_SKILLS,
    "the SUBJECT is empty or implausibly small ({n_skills} skills at {SKILLS_DIR}, \
     floor {MIN_SKILLS}). A zero from an empty corpus is not a zero about skills."
  );

  let all_claims: usize = fs::read_dir(&skills)
    .map(|rd| {
      rd.flatten()
        .filter_map(|e| fs::read_to_string(e.path().join("SKILL.md")).ok())
        .map(|t| claims(&code_spans(&t)).len())
        .sum()
    })
    .unwrap_or(0);
  assert!(
    all_claims >= MIN_CLAIMS,
    "the EXTRACTOR matched {all_claims} claims across {n_skills} skills (floor \
     {MIN_CLAIMS}). An extractor that matches nothing makes this check pass \
     without examining anything -- which is the failure this estate keeps \
     finding in its own instruments."
  );

  // ---- THE CONTROL, BEFORE THE VERDICT ----
  // Two planted fixtures drive the check to BOTH verdicts. This runs first so a
  // green below can never be the kind produced by an extractor that sees nothing.
  let tmp = tempfile::tempdir().expect("tempdir");
  let plant = |name: &str, body: &str| {
    let d = tmp.path().join(name);
    fs::create_dir_all(&d).unwrap();
    fs::write(d.join("SKILL.md"), body).unwrap();
  };
  // A: names a verb that ships. Must NOT fire.
  plant(
    "control-clean",
    "Run `intent st new \"a thread\"` then `intent todo`.\n",
  );
  // B: names a verb that does not ship. MUST fire.
  plant(
    "control-stale",
    "Run `intent frobnicate --now` to do the thing.\n\n```sh\nintent quux list\n```\n",
  );

  // C: names a SHIPPED verb with a flag the surface does not carry. MUST fire on
  //    the flag axis and must NOT fire on the verb axis.
  plant(
    "control-flag",
    "Run `intent st new \"x\" --frobnicate` for this.\n",
  );

  let flags = shipped_flags();
  let control_flags = stale_flags_in_tree(tmp.path(), &shipped, &flags);
  assert!(
    control_flags.iter().any(|(s, _)| s == "control-flag"),
    "THE FLAG AXIS DID NOT FIND ITS OWN PLANT. `intent st new --frobnicate` names \
     a flag no shipped entry declares, so this must fire. It did not, so the flag \
     extractor or its ancestor walk is broken and the flag verdict below is worth \
     nothing. Caught: {control_flags:?}"
  );
  assert!(
    !control_flags.iter().any(|(s, _)| s == "control-clean"),
    "THE FLAG AXIS FIRED ON A CLEAN FIXTURE, which means it rejects flags the \
     table declares and every verdict it produces is noise. Caught: {control_flags:?}"
  );

  let control = stale_in_tree(tmp.path(), &shipped);
  let fired: BTreeSet<&str> = control.iter().map(|(s, _)| s.as_str()).collect();
  assert!(
    fired.contains("control-stale"),
    "THE CHECK DID NOT FIND ITS OWN PLANT. `intent frobnicate` and `intent quux \
     list` are named in a code span and neither is a shipped path, so this must \
     fire. It did not, so the extractor or the resolver is broken and the \
     verdict on the real catalogue below is worth nothing. Caught: {control:?}"
  );
  assert!(
    !fired.contains("control-clean"),
    "THE CHECK FIRED ON A CLEAN FIXTURE. `intent st new` and `intent todo` both \
     resolve against the shipped table, so a hit here means the resolver rejects \
     valid verbs and every verdict it produces is noise. Caught: {control:?}"
  );

  // ---- THE VERDICT ----
  // The population is PRINTED, not merely floored. `AC-00.16` obliges an
  // instrument to declare its reach, and a floor that passes silently tells a
  // later reader the check ran without telling them over what. Visible under
  // `--nocapture`; the floors above are what enforce it either way.
  let n_flag_claims: usize = fs::read_dir(&skills)
    .map(|rd| {
      rd.flatten()
        .filter_map(|e| fs::read_to_string(e.path().join("SKILL.md")).ok())
        .map(|t| flag_claims(&code_spans(&t)).len())
        .sum()
    })
    .unwrap_or(0);
  eprintln!(
    "AT-15.2 reach: {n_skills} skill(s) examined at {SKILLS_DIR}; {all_claims} verb \
     claim(s) and {n_flag_claims} flag claim(s) extracted from code contexts; \
     resolved against {} shipped path(s) and their shipped flags, from \
     dispatch::shipped_entries(). COVERED: the verb and flag axes. NOT COVERED: \
     the path and file-layout axes (a placeholder path is structurally identical \
     to a stale one -- see the module header), any claim outside a code span, \
     short flags, and argument shapes.",
    shipped.len()
  );

  let stale_flags = stale_flags_in_tree(&skills, &shipped, &flags);
  assert!(
    stale_flags.is_empty(),
    "{} skill(s) name a FLAG the shipped surface does not carry:\n{}\n\nA flag \
     with `disposition: retire` is declared and never reaches clap, so naming one \
     points a reader at something `--help` will not show them.",
    stale_flags.len(),
    stale_flags
      .iter()
      .map(|(s, c)| format!("  {s}: {c}"))
      .collect::<Vec<_>>()
      .join("\n")
  );

  let stale = stale_in_tree(&skills, &shipped);
  assert!(
    stale.is_empty(),
    "{} skill(s) name a verb the shipped surface does not carry, examined {} \
     skills against {} shipped paths:\n{}\n\nEach line is `<skill>: <claim>`. A \
     skill keyed to a v2 name keeps reading as correct prose while pointing at \
     nothing. Fix the skill, or -- if the verb SHOULD ship -- the table is what \
     is wrong.",
    stale.len(),
    n_skills,
    shipped.len(),
    stale
      .iter()
      .map(|(s, c)| format!("  {s}: intent {c}"))
      .collect::<Vec<_>>()
      .join("\n")
  );
}
