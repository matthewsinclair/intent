//! AT-05.1 / AC-05.1: the dispatch table is the SSOT -- the clap surface and
//! its help text are GENERATED from it, asserted by test.
//!
//! **Asserted in both directions, against the shipped binary.** Nothing in the
//! table may be absent from the surface, and nothing in the surface may be
//! absent from the table. One direction alone is not the property: a spine
//! that shipped every table entry plus five invented ones would pass a
//! table-to-surface check, and a spine that shipped three entries would pass a
//! surface-to-table check.
//!
//! The comparison runs against the REAL binary's `--help` output rather than
//! against the loader's structs. Comparing the loader to the table would prove
//! that serde works; comparing the binary to the table proves the surface a
//! user meets came from it.

use std::process::Command;

use intent_cli::dispatch;

fn help(args: &[&str]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .arg("--help")
    .output()
    .expect("run the v3 binary");
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// The family names the binary actually offers, read out of its help.
fn surface_families() -> Vec<String> {
  let text = help(&[]);
  let commands = text
    .split("Commands:")
    .nth(1)
    .expect("the root help lists commands");
  commands
    .lines()
    .map(str::trim_start)
    .take_while(|l| !l.starts_with("Options:"))
    .filter_map(|l| l.split_whitespace().next())
    .filter(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_lowercase() || c == '_'))
    .map(str::to_string)
    .collect()
}

#[test]
fn every_shipped_family_in_the_table_reaches_the_surface() {
  let table = dispatch::table();
  let surface = surface_families();

  let mut missing = Vec::new();
  for family in &table.families {
    let Some(entry) = family.entries.iter().find(|e| e.verb().is_none()) else {
      continue;
    };
    if !entry.is_shipped() {
      continue;
    }
    if !surface.contains(&family.name) {
      missing.push(family.name.clone());
    }
  }
  assert!(
    missing.is_empty(),
    "the table ships these families and the binary does not offer them: {missing:?}"
  );
}

#[test]
fn nothing_reaches_the_surface_that_is_not_in_the_table() {
  let table = dispatch::table();
  // Both halves of the table describe the surface: `families` is the ported v2
  // surface, `new_surface` the commands v3 adds. A check that knew only the
  // first would read every ADDITION as an undocumented invention -- which is
  // exactly what it did when `search` and `schema` were first wired.
  let known: Vec<&str> = table
    .families
    .iter()
    .map(|f| f.name.as_str())
    .chain(table.new_surface.iter().map(|e| e.path.as_str()))
    .collect();

  let invented: Vec<String> = surface_families()
    .into_iter()
    .filter(|f| !known.contains(&f.as_str()) && f != "help")
    .collect();
  assert!(
    invented.is_empty(),
    "the binary offers commands the table does not describe: {invented:?} -- a surface entry with no table row is a second, undocumented declaration"
  );
}

/// The other direction for the added commands. Without this, `new_surface`
/// could be widened to silence the check above while the command itself never
/// shipped -- the table would describe a surface nobody could reach.
#[test]
fn every_added_command_in_the_table_reaches_the_surface() {
  let table = dispatch::table();
  let surface = surface_families();

  let missing: Vec<String> = table
    .new_surface
    .iter()
    .filter(|e| e.is_shipped())
    .map(|e| e.path.clone())
    .filter(|p| !surface.contains(p))
    .collect();
  assert!(
    missing.is_empty(),
    "the table declares these added commands and the binary does not offer them: {missing:?}"
  );
  assert!(
    surface.contains(&"search".to_string()) && surface.contains(&"schema".to_string()),
    "AC-06.4 and AC-06.5 name these two by hand, so they are asserted by name as well as by the sweep"
  );
}

/// **No unbuilt verb tells the operator whose work package it is.**
///
/// This test used to assert the opposite, and the inversion is the honest
/// record of what happened rather than something to tidy away. It read
/// `an_unbuilt_command_names_the_work_package_that_owes_it`, and it was a good
/// test of a bad idea: the message hardcoded WP-06 for everything, which was
/// wrong for two of the six added commands, so the fix was to read the owner
/// from the table and the test pinned it there.
///
/// D37 says the whole category does not belong in output -- Intent's own
/// project-management state never reaches Intent's users, because a consumer of
/// the tool does not care about a work package in the Intent project. So the
/// right answer was never "name the correct WP"; it was "name no WP", and a
/// test asserting a more accurate leak is still a test asserting a leak.
///
/// **Swept over the whole unbuilt surface, not sampled.** The old form checked
/// two commands, which is how a message that got the citation back on a third
/// would have passed.
#[test]
fn no_unbuilt_command_leaks_intents_own_project_state() {
  let dir = tempfile::tempdir().expect("tempdir");
  let run = |args: &[&str]| {
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .args(args)
      .current_dir(dir.path())
      .output()
      .expect("run the v3 binary");
    String::from_utf8_lossy(&out.stderr).to_string()
  };

  let families = surface_families();
  assert!(
    families.len() > 5,
    "the sweep found almost no surface to walk: {families:?}"
  );

  let mut leaks = Vec::new();
  let mut seen = 0;
  for family in &families {
    let text = run(&[family.as_str()]);
    seen += 1;
    for leak in ["ST00", "WP-", "AC-", "AT-"] {
      if text.contains(leak) {
        leaks.push(format!("`intent {family}` says {leak}: {}", text.trim()));
      }
    }
  }

  assert_eq!(seen, families.len(), "every family was run");
  assert!(
    leaks.is_empty(),
    "shipped output carries Intent's own project-management state (D37):\n  {}",
    leaks.join("\n  ")
  );
}

#[test]
fn a_retired_family_does_not_reach_the_surface() {
  let surface = surface_families();
  assert!(
    !surface.contains(&"organize".to_string()),
    "`organize` is a ratified retire (hv, 2026-08-14): a strictly structured model cannot hold data in the wrong place, so the disorder it repairs cannot arise"
  );
}

/// Every leaf verb, per family, both ways.
#[test]
fn every_shipped_verb_reaches_its_family_and_none_is_invented() {
  let table = dispatch::table();
  let mut problems = Vec::new();

  for family in &table.families {
    let shipped: Vec<&str> = family
      .entries
      .iter()
      .filter(|e| e.is_shipped())
      .filter_map(|e| e.verb())
      .collect();
    if shipped.is_empty() {
      continue;
    }

    let text = help(&[family.name.as_str()]);
    let Some(block) = text.split("Commands:").nth(1) else {
      problems.push(format!("{}: help lists no commands", family.name));
      continue;
    };
    let offered: Vec<String> = block
      .lines()
      .map(str::trim_start)
      .take_while(|l| !l.starts_with("Options:"))
      .filter_map(|l| l.split_whitespace().next())
      .filter(|w| !w.is_empty())
      .map(str::to_string)
      .collect();

    for verb in &shipped {
      if !offered.iter().any(|o| o == verb) {
        problems.push(format!(
          "{} {verb}: in the table, absent from the surface",
          family.name
        ));
      }
    }
    for verb in &offered {
      if verb != "help" && !shipped.contains(&verb.as_str()) {
        problems.push(format!(
          "{} {verb}: on the surface, absent from the table",
          family.name
        ));
      }
    }
  }

  assert!(
    problems.is_empty(),
    "surface and table disagree:\n  {}",
    problems.join("\n  ")
  );
}

/// The help TEXT comes from the table too, not from a second set of strings.
#[test]
fn help_text_is_the_tables_help_text() {
  let table = dispatch::table();
  let text = help(&["st"]);

  let new_entry = table
    .families
    .iter()
    .find(|f| f.name == "st")
    .and_then(|f| f.entries.iter().find(|e| e.path == "st new"))
    .expect("st new is in the table");
  assert!(
    !new_entry.help.is_empty(),
    "precondition: the table carries help text for st new"
  );
  assert!(
    text.contains(&new_entry.help),
    "the binary's help for `st new` is not the table's:\n  table: {}\n  surface: {text}",
    new_entry.help
  );
}

/// The table is versioned and stamped with what it was measured against, so a
/// surface built from it can always name its provenance.
#[test]
fn the_table_names_its_schema_and_the_commit_it_was_measured_at() {
  let table = dispatch::table();
  assert!(table.schema.starts_with("intent/dispatch-table@"));
  assert!(
    !table.measured_at.is_empty(),
    "a measurement names the revision it covers, never HEAD"
  );
}

/// The invariants ic recorded are carried, so INV-02's pin has something to
/// point at from inside the binary.
#[test]
fn the_recorded_invariants_are_carried() {
  let table = dispatch::table();
  let ids: Vec<&str> = table.invariants.iter().map(|i| i.id.as_str()).collect();
  assert!(ids.contains(&"INV-02"), "got: {ids:?}");
  assert!(ids.contains(&"INV-04"), "the critic exit-code exception");
}

/// The Options block of one command's help, which is where a flag surfaces.
///
/// The LAST `Options:` deliberately: a family with verbs prints `Commands:`
/// first, and scanning the whole output would let a verb's description answer
/// for a flag that is not there.
fn options_block(args: &[&str]) -> String {
  let text = help(args);
  match text.rsplit_once("Options:") {
    Some((_, rest)) => rest.to_string(),
    None => String::new(),
  }
}

/// Whether a help block offers this exact spelling.
///
/// A boundary check rather than `contains`, because `--fix` is a substring of
/// `--fixup` and a flag that is absent would read as present the day a longer
/// one is added beside it.
fn offers(block: &str, spelling: &str) -> bool {
  let mut from = 0;
  while let Some(at) = block[from..].find(spelling) {
    let at = from + at;
    let after = block[at + spelling.len()..].chars().next();
    let before = at.checked_sub(1).and_then(|i| block[i..].chars().next());
    let bounded = !matches!(after, Some(c) if c.is_ascii_alphanumeric() || c == '-')
      && !matches!(before, Some(c) if c.is_ascii_alphanumeric());
    if bounded {
      return true;
    }
    from = at + spelling.len();
  }
  false
}

/// **A FLAG'S DISPOSITION IS HONOURED AT THE FLAG LEVEL** (EXP-05, ic).
///
/// The entry-level disposition was honoured and the flag-level one was ignored,
/// so a `retire` or `pending` flag on a shipped command was built anyway and
/// `--help` advertised what no renderer would answer. ic measured it from
/// outside with `surface_check.sh`; this is the same property inside the suite,
/// **and it is here because the fix was INVISIBLE TO THE WHOLE RUST SUITE**:
/// with the skip removed, all 339 tests still passed and only the shell
/// instrument noticed. A property whose only witness is a script nobody runs in
/// CI is a property that regresses on the next refactor.
///
/// Both directions, and the counts are asserted so neither can go vacuous. A
/// version of this that only checked "declared flags are present" would pass on
/// the original defect, since the defect was a flag being present that should
/// not have been.
#[test]
fn a_flags_disposition_decides_whether_it_reaches_the_surface() {
  let table = dispatch::table();
  let entries: Vec<&dispatch::Entry> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .filter(|e| e.is_shipped())
    .collect();

  let mut wrong = Vec::new();
  let (mut shipped, mut withheld) = (0, 0);
  for entry in entries {
    let args: Vec<&str> = entry.path.split_whitespace().collect();
    let block = options_block(&args);
    if block.is_empty() {
      continue;
    }
    for flag in &entry.flags {
      // clap's own; it supplies them whatever the table says, which is why the
      // table marks them `intrinsic` rather than `keep`.
      if flag
        .spellings
        .iter()
        .any(|s| s == "--help" || s == "-h" || s == "help")
      {
        continue;
      }
      let present = flag.spellings.iter().any(|s| offers(&block, s));
      match (flag.ships(), present) {
        (true, true) => shipped += 1,
        (false, false) => withheld += 1,
        (true, false) => wrong.push(format!(
          "MISSING  `{}` {:?} -- declared `{}` (ships) and the surface does not offer it",
          entry.path, flag.spellings, flag.disposition
        )),
        (false, true) => wrong.push(format!(
          "PRESENT  `{}` {:?} -- declared `{}` (does not ship) and the surface offers it",
          entry.path, flag.spellings, flag.disposition
        )),
      }
    }
  }

  assert!(
    wrong.is_empty(),
    "the binary and the table disagree about which flags exist:\n  {}",
    wrong.join("\n  ")
  );
  assert!(
    shipped > 0 && withheld > 0,
    "this check needs both kinds to be discriminating: {shipped} shipped and {withheld} withheld. \
     With either at zero it is asserting only one direction and would pass on the defect it was \
     written for"
  );
}

/// **A REMEDY MUST NOT PROMISE AN EMPTY CATEGORY** (ic, measured).
///
/// The unbuilt-command refusal sent every reader to `intent <x> --help` "for
/// the verbs that are". On a family that has verbs that is a real lead; on a
/// LEAF it costs the reader a command and returns a help block listing nothing.
/// ic swept it: 17 unimplemented commands, **9 of them leaves**.
///
/// A remedy that cannot be acted on is worse than none, because it reads as a
/// lead and spends the reader's next move. Both populations are asserted
/// non-empty: with either at zero this is checking one branch and would pass on
/// the defect.
#[test]
fn an_unbuilt_leaf_does_not_send_the_reader_to_an_empty_help() {
  let table = dispatch::table();
  let (mut leaves, mut families) = (0, 0);
  let mut wrong = Vec::new();

  for family in &table.families {
    let Some(entry) = family.entries.iter().find(|e| e.verb().is_none()) else {
      continue;
    };
    if !entry.is_shipped() {
      continue;
    }
    let out = Command::new(env!("CARGO_BIN_EXE_intent"))
      .arg(&family.name)
      .output()
      .expect("run the v3 binary");
    let text = format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    );
    if !text.contains("is a known command that is not implemented yet") {
      continue;
    }

    let has_verbs = family
      .entries
      .iter()
      .any(|e| e.verb().is_some() && e.is_shipped());
    let points_at_own_help = text.contains(&format!("intent {} --help", family.name));
    if has_verbs {
      families += 1;
      if !points_at_own_help {
        wrong.push(format!(
          "`{}` has verbs and its remedy does not name them: {text}",
          family.name
        ));
      }
    } else {
      leaves += 1;
      if points_at_own_help {
        wrong.push(format!(
          "`{}` has NO verbs and its remedy sends the reader to a help block that lists none: \
           {text}",
          family.name
        ));
      }
    }
  }

  assert!(wrong.is_empty(), "{}", wrong.join("\n"));
  assert!(
    leaves > 0 && families > 0,
    "both shapes must be exercised to be discriminating: {leaves} unbuilt leaves and {families} \
     unbuilt families with verbs"
  );
}

/// **The authored placeholder reaches the usage line, and requiredness reaches
/// the usage block** (issue 0035 / EXP-07).
///
/// `Flag` deserialised four of a row's fields and dropped four more, so 35
/// declared `value`s reached nothing and clap fell back to printing our own
/// internal argument id at the reader: `--evidence <evidence>` where the table
/// says `<ref>`. **The table was authored, committed, reviewed and inert.**
///
/// The requiredness half had a second tell worth recording, because it is what
/// a dropped declaration looks like from the outside: `--reason`'s help string
/// ends with a hand-written `-- REQUIRED`, and `--evidence`'s does not, for two
/// flags the table declares identically. **When a mechanism stops carrying a
/// fact, the fact comes back by hand, in one place and not the other.**
///
/// Driven from the declaration rather than a list of flags, and counted in both
/// directions, so it cannot pass by examining nothing.
#[test]
fn a_flags_declared_placeholder_and_requiredness_reach_the_surface() {
  let table = dispatch::table();
  let entries: Vec<&dispatch::Entry> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .filter(|e| e.is_shipped())
    .collect();

  let mut wrong = Vec::new();
  let (mut placeholders, mut requireds) = (0, 0);
  for entry in entries {
    let args: Vec<&str> = entry.path.split_whitespace().collect();
    let block = options_block(&args);
    if block.is_empty() {
      continue;
    }
    let usage = help(&args);
    for flag in &entry.flags {
      if !flag.ships() {
        continue;
      }
      let Some(long) = flag.spellings.iter().find(|s| s.starts_with("--")) else {
        continue;
      };
      if let Some(value) = &flag.value
        && flag.kind != "bool"
      {
        // The three shapes the table writes, normalised to what a reader should
        // see. The delimiters are the table showing the rendering and clap adds
        // its own, so they come off; a trailing `...` is arity and clap prints
        // it after the name. **`<path> ...` is the row that caught the first
        // version of this check** -- it stripped delimiters from both ends
        // blindly and asked for `<path> ...>`, which is the same class of
        // mistake as the defect under test.
        let repeated = value.trim_end().ends_with("...");
        let head = value.trim().trim_end_matches("...").trim();
        let inner = head
          .trim_start_matches(['<', '['])
          .trim_end_matches(['>', ']']);
        let want = format!("{long} <{inner}>{}", if repeated { "..." } else { "" });
        if block.contains(&want) {
          placeholders += 1;
        } else {
          wrong.push(format!(
            "PLACEHOLDER  `{}` {long} declares value {value:?} and the surface does not show `{want}`",
            entry.path
          ));
        }
      }
      if flag.required {
        // A required flag appears in clap's usage line unbracketed. An optional
        // one never does -- it lives in `[OPTIONS]` -- so this distinguishes
        // the two rather than merely finding the spelling somewhere.
        let shown = usage
          .lines()
          .find(|l| l.starts_with("Usage:"))
          .is_some_and(|l| l.contains(long.as_str()));
        if shown {
          requireds += 1;
        } else {
          wrong.push(format!(
            "REQUIRED  `{}` {long} is declared required and the usage line presents it as optional",
            entry.path
          ));
        }
      }
    }
  }

  assert!(
    wrong.is_empty(),
    "the table declares what the surface does not carry:\n  {}",
    wrong.join("\n  ")
  );
  assert!(
    placeholders >= 20 && requireds >= 3,
    "only {placeholders} placeholders and {requireds} required flags were exercised -- the \
     enumeration is collapsing, and a check that examines nothing passes"
  );
}

/// **AC-06.8's second half: a withheld flag is reported, not silently absent.**
///
/// The ruling has two halves -- `pending` does not ship, and its absence is not
/// silent. The first worked; the second **did not exist**, and ic's diagnosis of
/// why is the part worth keeping: it was ruled into `intentsvcs::doctor`, which
/// cannot read a table that is `include_str!`'d into the CLI crate. **A
/// mitigation ruled into the one place that structurally cannot perform it**,
/// which is why nobody noticed it was missing -- there was no obvious hole where
/// the code should have been.
///
/// Driven from the declaration, both directions: every `pending` flag on a
/// shipped entry must be named, and no flag that SHIPS may be reported as
/// withheld. Without the second half this passes on a report that lists every
/// flag in the table.
#[test]
fn a_withheld_flag_is_named_by_doctor_and_a_shipped_one_is_not() {
  let dir = tempfile::tempdir().expect("tempdir");
  std::fs::create_dir_all(dir.path().join("intent/.config")).expect("mkdir");
  std::fs::write(
    dir.path().join("intent/.config/config.json"),
    "{\"intent_version\":\"3.0.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  )
  .expect("write config");

  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .arg("doctor")
    .current_dir(dir.path())
    .output()
    .expect("run doctor");
  let text = String::from_utf8_lossy(&out.stdout).to_string();

  let table = dispatch::table();
  let (mut withheld, mut shipped) = (0, 0);
  let mut wrong = Vec::new();
  for entry in dispatch::shipped_entries(&table) {
    for flag in &entry.flags {
      let named = text.lines().filter(|l| l.starts_with("surface:")).any(|l| {
        l.contains(&format!("`{}`", entry.path))
          && flag.spellings.iter().any(|s| l.contains(s.as_str()))
      });
      if flag.disposition == "pending" {
        withheld += 1;
        if !named {
          wrong.push(format!(
            "SILENT   `{}` {:?} is withheld and doctor does not say so",
            entry.path, flag.spellings
          ));
        }
      } else if flag.ships() && named {
        shipped += 1;
        wrong.push(format!(
          "REPORTED `{}` {:?} ships and doctor calls it withheld",
          entry.path, flag.spellings
        ));
      } else if flag.ships() {
        shipped += 1;
      }
    }
  }

  assert!(
    wrong.is_empty(),
    "doctor's account of the withheld surface disagrees with the table:\n  {}",
    wrong.join("\n  ")
  );
  assert!(
    withheld > 0 && shipped > 0,
    "this needs both kinds to be discriminating: {withheld} withheld and {shipped} shipped"
  );

  // **A withholding is not a defect and must not inflate the finding count**,
  // or a reader learns to ignore the one number that carries the verdict.
  //
  // Asserted against the report's own arithmetic rather than against a clean
  // project: a bare config-only project has findings of its own, so "expect
  // zero" was a fact about the fixture and not about the property. Findings
  // print their first line at column 0 and their remedy indented; the surface
  // lines and the summary are the two other shapes.
  let summary = text
    .lines()
    .find(|l| l.starts_with("doctor: "))
    .expect("the summary line");
  let reported: usize = summary
    .trim_start_matches("doctor: ")
    .split_whitespace()
    .next()
    .and_then(|n| n.parse().ok())
    .unwrap_or_else(|| panic!("the summary leads with a count: {summary}"));
  let printed = text
    .lines()
    .filter(|l| {
      !l.starts_with(' ')
        && !l.starts_with("surface:")
        && !l.starts_with("doctor: ")
        && !l.is_empty()
    })
    .count();
  assert_eq!(
    reported, printed,
    "the count must be of FINDINGS: {withheld} withheld flags were printed and the summary says \
     {reported} across {printed} finding lines"
  );
}

/// **Issue 0039: an alias declared on a shipped row IS that command.**
///
/// `surface/dispatch-table.json` declares `aliases` on five entries and
/// `pub struct Entry` did not have the field, so serde dropped it in silence.
/// Four of the five are `disposition: keep` -- the one classification that
/// promises the v2 spelling survives -- and `at done` / `at notdone`, which v2
/// documents in its own help as "Aliases for green | red", did not exist in
/// the binary at all. Every instrument reported agreement, because a JSON file
/// cannot say whether anyone is listening.
///
/// **The assertion is equality of behaviour, not presence in help text.**
/// Searching the help for the word `done` would pass on any command whose help
/// happens to contain it; running both spellings and comparing what came back
/// is the property the word "alias" actually means. It needs no arguments to
/// do it: with none, the canonical spelling reports its missing positionals
/// and a spelling that does not exist reports `unrecognized subcommand`, so
/// the two answers differ exactly when the alias is missing.
///
/// Discovered from the table rather than listed here, so an alias added to a
/// sixth row is covered on the day it is authored -- and counted, because a
/// loop over an empty set is a test that passes by not looking.
#[test]
fn every_declared_alias_on_a_shipped_row_is_the_command_it_aliases() {
  let table = dispatch::table();
  let mut checked = 0;

  for entry in dispatch::shipped_entries(&table) {
    for alias in entry.alias_verbs() {
      let (head, verb) = entry
        .path
        .rsplit_once(' ')
        .expect("an aliased entry sits inside a family");

      let by_alias = run_raw(&[head, alias]);
      let by_name = run_raw(&[head, verb]);
      // clap echoes the spelling it was invoked with into its usage line, so
      // the alias is normalised to the canonical name before comparing --
      // otherwise the test would demand that an alias lie about how it was
      // called.
      let normalised = by_alias.replace(&format!("{head} {alias}"), &format!("{head} {verb}"));

      assert_eq!(
        normalised, by_name,
        "`{head} {alias}` must be `{head} {verb}`; it is declared in the table as an alias on a \
         shipped row, and v2 answers to it"
      );
      checked += 1;
    }
  }

  assert!(
    checked >= 2,
    "the table declares aliases on shipped rows and this test found {checked} -- either they were \
     removed from the canon or this loop stopped seeing them, and both want a human"
  );
}

/// **A retired row's alias stays retired**, which is the half that a naive fix
/// gets wrong.
///
/// `st organize` is `disposition: retire` and carries `st organise`. Registering
/// aliases without asking whether the row ships would bring a withdrawn command
/// back through its old spelling while the canonical one stays gone -- a
/// command that exists only under the name nobody chose to keep.
///
/// **The assertion moved from the MESSAGE to the SURFACE, and the reason is
/// worth keeping.** This test read `out.contains("unrecognized subcommand")`,
/// which was an exact proxy for "did not resolve" only because clap's generic
/// error was the sole alternative to resolving. Issue 0044 added a third
/// outcome -- refused BY NAME as retired -- and the proxy reddened on a change
/// that strengthened the very thing it guards. **A test measuring the mechanism
/// of a refusal instead of the refusal fails when the mechanism improves**, so
/// what it now asks is whether clap knows the spelling at all. If the surface
/// does not carry it, it cannot resolve, whatever the message says.
#[test]
fn a_retired_rows_alias_does_not_come_back() {
  let table = dispatch::table();
  let retired: Vec<&str> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .filter(|e| !e.is_shipped() && !e.aliases.is_empty())
    .flat_map(|e| e.alias_verbs())
    .collect();

  assert!(
    !retired.is_empty(),
    "no retired row carries an alias, so this test proves nothing -- check the table before \
     deleting it, because the case it guards is cheap to reintroduce"
  );

  let surface = intent_cli::spine::build(&table);
  let st = surface
    .get_subcommands()
    .find(|c| c.get_name() == "st")
    .expect("the shipped surface carries the `st` family");

  for alias in retired {
    let registered = st.get_subcommands().any(|verb| {
      verb.get_name() == alias || verb.get_all_aliases().any(|declared| declared == alias)
    });
    assert!(
      !registered,
      "`st {alias}` is registered on the shipped clap surface. It is an alias on a RETIRED row, so registering it revives a withdrawn command under the one \
       spelling nobody chose to keep -- and the canonical `st organize` would still be gone, which is the state hardest to diagnose from outside"
    );

    // And end to end, because a surface assertion cannot see a second path that
    // answers the spelling. Either refusal is correct here; what is not is
    // running.
    let out = run_raw(&["st", alias]);
    assert!(
      out.contains("unrecognized subcommand") || out.contains("was retired in Intent v3"),
      "`st {alias}` neither resolved to nothing nor named itself retired, so something answered it: {out}"
    );
  }
}

/// stdout + stderr of one invocation, with no `--help` appended.
fn run_raw(args: &[&str]) -> String {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(args)
    .output()
    .expect("run the v3 binary");
  format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  )
}

/// **The root `--help` line comes from the table** (EXP-08, vc's ask).
///
/// This was the ONE `.about("...")` literal in the CLI. Every family, entry,
/// verb and flag already read their help from the table; the root did not, so
/// the SSOT claim was false in exactly the place an agent reads first -- and it
/// went through a different code path from the rest, which is why
/// `help_text_is_the_tables_help_text` could not see it. That test spot-checks
/// one command on the argument that the MECHANISM carries the others, and the
/// root was not on the mechanism.
///
/// **Not circular.** It compares the SHIPPED BINARY's first line to the table
/// read independently, so a literal reintroduced in the spine fails it however
/// plausible the literal is -- including one that happens to match today, which
/// would then drift the first time the table's wording changed.
#[test]
fn the_root_help_line_is_the_tables_root_help() {
  let table = dispatch::table();
  assert!(
    !table.root_help.trim().is_empty(),
    "the table declares no root help, so comparing against it would assert nothing -- \
     `root_help` is deliberately not `serde(default)` for this reason"
  );

  let text = help(&[]);
  let first = text
    .lines()
    .find(|line| !line.trim().is_empty())
    .expect("the root help has a first line");

  assert_eq!(
    first.trim(),
    table.root_help.trim(),
    "the first line a reader meets must be the table's, not a literal beside it"
  );
}

/// **And an unchecked root would be invisible**, so the discriminator is
/// asserted rather than assumed: the root's help is not any family's help.
///
/// Without this, a spine that rendered the FIRST FAMILY's about line at the
/// root would satisfy the test above on any table where the two happened to
/// match, and nothing would say which one was being read.
#[test]
fn the_root_help_is_not_borrowed_from_a_family() {
  let table = dispatch::table();
  let family_helps: Vec<&str> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .filter(|e| e.verb().is_none())
    .map(|e| e.help.as_str())
    .collect();
  assert!(
    !family_helps.is_empty(),
    "no family helps to compare against, so this test proves nothing"
  );
  assert!(
    !family_helps.contains(&table.root_help.as_str()),
    "the root help is identical to a family's, so the two are indistinguishable in the shipped \
     output and the assertion above cannot tell which one it read"
  );
}

/// **`Entry::is_shipped()` and the canon's `populations` block are bound, and
/// the binding is why the predicate may stay a predicate** (ic, 2026-08-17,
/// closing 0037's remaining half).
///
/// The four populations had four homes: `lib_surface.sh`, `implemented_check.
/// sh`'s own `EXCLUDED`, four inline walks in `surface_check.sh`, and this
/// predicate. The one home is now `.populations` in the table, generated and
/// refused-on-skew by `gen_dispatch_table.sh`, and every shell consumer reads
/// it.
///
/// **The binary deliberately does NOT read it.** `is_shipped()` fails OPEN
/// across two fields and is currently the only thing stopping a single
/// hand-edit from shipping a retired command; replacing it with a lookup would
/// make the list a third field carrying that power, and a list is edited by
/// people who believe they are editing data. So the predicate stays and this
/// test is what stops it drifting from the block.
///
/// **Two witnesses at the two moments, not one witness twice.** The generator
/// arm catches a block edited away from the rows; this catches a predicate
/// edited away from the block. The generator is a shell tool nobody runs on a
/// push, which is why the second half has to live in the suite -- cc's argument
/// about `KEY_UNCLASSED`, and it applies unchanged here.
///
/// Order is compared, not just membership: the block is generated in table
/// order, so a set-equal-but-reordered block means it was hand-edited, and the
/// next regeneration would produce a diff nobody asked for.
#[test]
fn the_populations_block_and_the_shipping_predicate_agree() {
  let table = dispatch::table();
  let raw: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  let pops = &raw["populations"];
  assert!(
    pops.is_object(),
    "the table has no `populations` block, so this test would pass by having nothing to compare \
     -- and every consumer is back to re-deriving the four populations by hand, which is the \
     defect 0037 is about"
  );

  let list = |key: &str| -> Vec<String> {
    pops[key]
      .as_array()
      .unwrap_or_else(|| panic!("`populations.{key}` is a list"))
      .iter()
      .map(|v| v.as_str().expect("a path is a string").to_string())
      .collect()
  };

  // Every row, both homes, in table order -- the enumerator this whole class is
  // about. `.families[].entries[]` alone is 104 where declared is 112.
  let rows: Vec<&dispatch::Entry> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .collect();

  let declared: Vec<String> = rows.iter().map(|e| e.path.clone()).collect();
  let shipped: Vec<String> = rows
    .iter()
    .filter(|e| e.is_shipped())
    .map(|e| e.path.clone())
    .collect();
  let retired: Vec<String> = rows
    .iter()
    .filter(|e| !e.is_shipped())
    .map(|e| e.path.clone())
    .collect();
  // `not_probed` is authored and carries a reason per member, so the paths come
  // out of the objects. The key is NOT `nonreturning`: that name admitted only
  // one of the two reasons these are excluded for, and `claude upgrade` -- which
  // returns fine and installs into the operator's real `~/.claude` -- fell out
  // of the list because it did not fit the name.
  let not_probed: Vec<String> = pops["not_probed"]
    .as_array()
    .expect("`populations.not_probed` is a list")
    .iter()
    .map(|m| {
      m["path"]
        .as_str()
        .expect("a not_probed member has a path")
        .to_string()
    })
    .collect();
  let probeable: Vec<String> = shipped
    .iter()
    .filter(|p| !not_probed.contains(p))
    .cloned()
    .collect();

  for (key, computed) in [
    ("declared", &declared),
    ("shipped", &shipped),
    ("retired", &retired),
    ("probeable", &probeable),
  ] {
    assert_eq!(
      &list(key),
      computed,
      "`populations.{key}` disagrees with what the types compute from the rows. The block is the \
       one home every shell consumer reads and the predicate is what the binary dispatches on, so \
       a disagreement means one of them is telling somebody a confident wrong answer about which \
       commands exist."
    );
  }

  // The arithmetic is asserted rather than left to the two lists happening to
  // agree today. A row retired in `new_surface` would otherwise be counted by
  // neither, and both comparisons above would still pass.
  assert_eq!(
    shipped.len() + retired.len(),
    declared.len(),
    "shipped + retired does not equal declared, so a row is being counted by neither and the \
     partition has a hole that per-list comparison cannot see"
  );
}
