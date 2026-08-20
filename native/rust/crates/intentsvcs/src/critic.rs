//! The headless rule critic -- the mechanical subset of the rule library,
//! enforced without an LLM.
//!
//! **THIS COMMAND IS THE PRE-COMMIT GATE.** `lib/templates/hooks/pre-commit.sh`
//! shells out to it per declared language, and fifteen other projects reach the
//! same runner through one symlink. Every design choice below is downstream of
//! that: a critic that answers wrongly does not produce a wrong report, it
//! produces an unguarded commit in sixteen repositories.
//!
//! # Exit codes are built to the CODE and the GATE, not to the surface table
//!
//! **THE DISPATCH TABLE IS WRONG ABOUT THIS COMMAND AND BUILDING TO IT WOULD
//! SHIP A SILENT, PERMANENT GATE BYPASS.** Three places in
//! `surface/dispatch-table.json` -- INV-04's rule, INV-02's
//! `implementation_note`, and this family's `family_notes` -- assert that exit 2
//! means findings-present, each citing `bin/intent_critic:89,95` as evidence.
//! Both cited lines are error paths (`error_out()` and the no-argument help).
//! Findings-present is `:348`, and it is **1**. Exit **3** (a rule was armed and
//! could not be enforced) is used at `:334` and `:347` and INV-04 does not
//! mention it at all.
//!
//! The consequence is exact rather than theoretical. `pre-commit.sh` blocks on
//! 1 and on 3, and its `*)` arm -- where 2 lands -- prints `invocation error;
//! fail-open` and never sets `AGGREGATE`. A critic built faithfully to the table
//! would therefore exit 2 on every finding, the gate would fail open, and the
//! commit would land unchecked -- **while being correct against the surface
//! SSOT, so conformance would pass.** Unimplemented is loud and temporary; that
//! would have been quiet and permanent.
//!
//! So this module implements what v2 does and what the gate reads:
//!
//! | code | meaning                                                    | gate    |
//! | ---- | ---------------------------------------------------------- | ------- |
//! | 0    | clean -- everything ASKED came back empty                  | passes  |
//! | 1    | findings present                                           | BLOCKS  |
//! | 2    | usage / invocation error -- the gate itself is broken      | fails open |
//! | 3    | refused -- an ARMED rule's TOOL IS ABSENT on this machine  | BLOCKS  |
//!
//! The gate's own comment states the principle the table inverts: *a gate should
//! fail open on its own breakage and closed on yours.* 2 means our breakage.
//! Reported to the surface owner; this module does not edit their file, and it
//! does not wait for it either.
//!
//! # Two axes, because one of them is a property of the rule and one is a
//! property of the invocation
//!
//! Carried over from v2's census (vc's ruling, 2026-08-19) because collapsing
//! them is the founding defect of this whole command: a runner that skipped a
//! rule without saying so returned 0 as though the question had been put.
//!
//! - **Arming** -- could ANYTHING answer this rule? `Armed` / `Declared` (an
//!   explicit "no greppable proxy is authoritative") / `Unrunnable` (a block
//!   whose every line the contract must refuse) / `Undeclared` (nobody decided).
//! - **Disposition** -- did THIS invocation ask? `Ran` / `ToolAbsent` /
//!   `OutOfContext` / `NotApplicable`.
//!
//! A fifth arming value was v2's first design and was wrong: it would put a
//! property of the invocation into a key whose other members are properties of
//! the rule, and `Armed` would then mean two things depending on which member
//! you read.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::rules::{Library, RulesError};

/// The languages that have a HEADLESS code critic -- the roster `--languages`
/// prints and the roster an unknown language is refused against.
///
/// **ONE HOME, BECAUSE TWO DRIFT SILENTLY IN THE DIRECTION SOMEONE WILL
/// ACTUALLY TAKE THEM.** This was a pair of literals twenty-six lines apart in
/// `render.rs`, with a comment between them asserting the gate *carries no
/// language knowledge of its own and cannot drift from this registry* --
/// singular, while there were two (cc, 2026-08-20). Add a sixth language to the
/// refusal list alone and `critic <newlang>` runs, `--languages` omits it,
/// `bin/.devbin/lib/cmd/check` reads that list, and **the language is silently
/// dropped from the check loop -- the failure reads as a pass.** The opposite
/// order fails loudly at the refusal, so the dangerous direction is the one
/// that looks like the smaller edit.
///
/// **IT IS NOT [`crate::rules::LANGUAGES`] AND MUST NOT BE COLLAPSED INTO IT.**
/// That is the RULE-PACK roster and carries nine -- `agnostic`, `prose`,
/// `author` and `content` besides these five. Those have rules and no headless
/// runner: `author` and `content` are a deliberate clean no-op here because
/// prose critique is on-demand via the subagent. **Two rosters that overlap are
/// not one roster**, and merging them would make `--languages` advertise four
/// languages this command cannot mechanically check.
pub const HEADLESS_LANGUAGES: [&str; 5] = ["elixir", "rust", "swift", "lua", "shell"];

/// How serious a rule's findings are. Ordered, because `--severity-min` filters
/// on it and a filter needs a total order rather than a set of names.
///
/// **`Critical` is the LOWEST discriminant deliberately**, so `>=` reads as
/// "at least this serious" against a minimum. Inverting it would make the
/// filter's comparison read backwards at every call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
  Critical,
  Warning,
  Recommendation,
  Style,
}

impl Severity {
  /// Parse a rule's `severity:` field or the `--severity-min` value.
  ///
  /// **An unknown severity is `None` rather than a default.** A rule whose
  /// severity does not parse is a broken rule, and silently treating it as
  /// `Warning` would let a malformed critical rule report at a level nobody
  /// chose (IN-AG-NO-SILENT-001).
  pub fn parse(s: &str) -> Option<Self> {
    match s.trim() {
      "critical" => Some(Self::Critical),
      "warning" => Some(Self::Warning),
      "recommendation" => Some(Self::Recommendation),
      "style" => Some(Self::Style),
      _ => None,
    }
  }

  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Critical => "critical",
      Self::Warning => "warning",
      Self::Recommendation => "recommendation",
      Self::Style => "style",
    }
  }

  /// Does this severity clear the run's minimum?
  ///
  /// `Critical` is lowest, so "at least as serious as the minimum" is `<=`.
  pub fn clears(&self, min: Severity) -> bool {
    *self <= min
  }
}

/// Whether anything COULD answer this rule. A property of the rule and the
/// project's configuration, never of this run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arming {
  /// A greppable block with at least one runnable line, or a named tool.
  Armed,
  /// An explicit "No greppable proxy is authoritative for this rule".
  Declared,
  /// A block every line of which the strict-proxy contract must refuse.
  Unrunnable,
  /// None of the above -- nobody has decided whether this is checkable.
  Undeclared,
}

impl Arming {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Armed => "armed",
      Self::Declared => "declared",
      Self::Unrunnable => "unrunnable",
      Self::Undeclared => "undeclared",
    }
  }
}

/// Whether THIS invocation actually put the question. A property of the run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
  /// The question was put to these files.
  Ran,
  /// Armed on a tool this machine does not have.
  ToolAbsent(String),
  /// Armed on a tool that does not belong in this run -- a whole-workspace
  /// analyser is not a per-file gate.
  OutOfContext(String),
  /// Nothing to run; the arming axis already said why.
  NotApplicable,
}

impl Disposition {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Ran => "ran",
      Self::ToolAbsent(_) => "not-run:tool-absent",
      Self::OutOfContext(_) => "not-run:out-of-context",
      Self::NotApplicable => "n-a",
    }
  }
}

/// One rule's standing in this run, on both axes.
#[derive(Debug, Clone)]
pub struct CensusRow {
  pub rule_id: String,
  pub arming: Arming,
  pub disposition: Disposition,
  /// What would answer it -- `grep`, a tool name, or nothing.
  pub by: String,
}

/// One match. Carries the rule that produced it, because a finding without its
/// rule sends the reader hunting for which of sixty-four rules objected.
#[derive(Debug, Clone)]
pub struct Finding {
  pub rule_id: String,
  pub severity: Severity,
  pub path: PathBuf,
  /// 1-indexed, matching how every editor and `grep -n` count.
  pub line_no: usize,
  pub line: String,
}

/// The whole answer: what was found, and -- equally load-bearing -- what was
/// never asked.
///
/// **THE CENSUS IS NOT DECORATION AND MUST NOT BE DROPPED FROM OUTPUT.** A clean
/// result covers what was ASKED and says nothing about the rest; a runner that
/// prints findings without the census reports a green over questions it never
/// put, which is this command's founding defect.
#[derive(Debug, Clone)]
pub struct Report {
  pub lang: String,
  pub findings: Vec<Finding>,
  pub census: Vec<CensusRow>,
  /// Rules whose PROXY the ST0039 contract refused -- reported in the census
  /// and, deliberately, NOT an exit-3 condition.
  ///
  /// **THIS FIELD ONCE DROVE EXIT 3 AND THAT WAS WRONG IN BOTH DIRECTIONS
  /// (vc, 2026-08-20).** The header table above said 3 meant *a rule was armed
  /// and could not be enforced*, `exit_code` keyed it on this set, and the two
  /// are different populations five lines apart in one file -- INV-04's shape
  /// one file over, with the header asserting one meaning and the code
  /// implementing another. v2 sets `CRITIC_REFUSED` in exactly one place,
  /// `bin/intent_critic:319`, inside the `c_absent` block: **an unrunnable
  /// proxy is REPORTED and never refuses; an absent TOOL refuses.**
  ///
  /// **THE DIRECTION IS WHAT MADE IT SERIOUS**: a project that armed a
  /// shellcheck rule on a machine without shellcheck was silently PASSED, which
  /// is the sentence AC-07.4 exists to forbid. CI images and new laptops are
  /// routinely that machine.
  pub refused: Vec<String>,
}

impl Report {
  pub fn armed(&self) -> usize {
    self
      .census
      .iter()
      .filter(|r| r.arming == Arming::Armed)
      .count()
  }

  pub fn ran(&self) -> usize {
    self
      .census
      .iter()
      .filter(|r| r.disposition == Disposition::Ran)
      .count()
  }

  pub fn total(&self) -> usize {
    self.census.len()
  }

  /// Rules armed on a tool this machine does not have. **This is what exit 3
  /// means**, and it is the only condition v2 refuses on.
  pub fn unenforced(&self) -> Vec<&str> {
    // Sorted, for the reason every other id list here is: walk order is
    // undiffable and means nothing to a reader.
    let mut out: Vec<&str> = self
      .census
      .iter()
      .filter(|r| matches!(r.disposition, Disposition::ToolAbsent(_)))
      .map(|r| r.rule_id.as_str())
      .collect();
    out.sort_unstable();
    out
  }

  /// The exit code the gate will read. **See the module note: 1 is findings,
  /// 3 is an armed rule whose tool is absent, and neither is 2.**
  ///
  /// **REFUSAL OUTRANKS FINDINGS.** Both block, so the order only decides which
  /// remedy the operator is handed -- and "a rule you armed could not be
  /// enforced" is actionable in a way "fix these findings" is not when the run
  /// was also incomplete. Reporting findings while silently dropping a refusal
  /// would tell someone their code is the problem when our coverage is.
  ///
  /// **AN UNRUNNABLE PROXY IS NOT A REFUSAL**, however much the word invites it:
  /// nobody can act on it, it is our defect rather than the project's, and v2
  /// reports it in the census at exit 0. See [`Report::refused`].
  pub fn exit_code(&self) -> i32 {
    if !self.unenforced().is_empty() {
      3
    } else if !self.findings.is_empty() {
      1
    } else {
      0
    }
  }
}

#[derive(Debug, Error)]
pub enum CriticError {
  #[error("{0}")]
  Rules(#[from] RulesError),
  #[error("cannot read `{path}`: {source}")]
  Read {
    path: PathBuf,
    source: std::io::Error,
  },
  /// A pattern the contract admitted and the engine could not compile. **This
  /// is our defect, not the operator's, and it is surfaced rather than skipped**
  /// -- a pattern silently dropped is a rule silently unenforced.
  #[error("rule `{rule_id}` publishes a proxy this build cannot compile: {detail}")]
  Uncompilable { rule_id: String, detail: String },
}

/// True iff the line is a single, simple `grep` invocation the headless runner
/// can execute faithfully. The accepted shape is:
///
/// ```text
/// grep [-r|-n|-E|-rn|-rE|-nE|-rnE|--include=GLOB ...] '<pattern>' [<path>...]
/// ```
///
/// **THIS IS AN INJECTION BOUNDARY AND IT IS NOT RELAXED.** v2's
/// `critic_proxy_is_simple` carries that warning in as many words and this port
/// keeps the shape character-for-character rather than "improving" it. The rules
/// below are the contract (ST0039), not heuristics:
///
/// - Exactly one `grep`. No pipes, no chaining.
/// - Flag clusters drawn from `{r, n, E}` only, or `--include=GLOB`. `-L`, `-v`,
///   `-l`, `-c`, `-o`, `-w`, `-x` and the `-A`/`-B` context forms are REFUSED.
/// - **`-A` and `-B` being refused is load-bearing rather than incidental**: a
///   rule whose violation is the ABSENCE of a neighbouring justification cannot
///   be proxied at all, because the neighbouring line is unreachable by
///   construction. Several rules say exactly this and declare no proxy.
/// - The pattern is single-quoted. Metacharacters inside it are regex, not
///   shell.
/// - Path arguments carry no shell metacharacter, so a pipeline cannot arrive
///   disguised as an argument.
///
/// Empty lines and `#` comments return `false` -- not a candidate, which the
/// caller skips silently and which is distinct from refused.
pub fn proxy_is_simple(line: &str) -> bool {
  let trimmed = line.trim();
  if trimmed.is_empty() || trimmed.starts_with('#') {
    return false;
  }
  let Some(rest) = trimmed.strip_prefix("grep") else {
    return false;
  };
  // `grepfoo` is not `grep`: the command must end at a space.
  if !rest.starts_with(char::is_whitespace) {
    return false;
  }

  let mut cursor = rest.trim_start();

  // Flags first, and only the admitted ones.
  loop {
    if cursor.starts_with("--include=") {
      let end = cursor.find(char::is_whitespace).unwrap_or(cursor.len());
      // `--include=` with nothing after it is not a glob.
      if end <= "--include=".len() {
        return false;
      }
      cursor = cursor[end..].trim_start();
      continue;
    }
    if let Some(flags) = cursor.strip_prefix('-') {
      let end = flags.find(char::is_whitespace).unwrap_or(flags.len());
      let cluster = &flags[..end];
      if cluster.is_empty() || !cluster.chars().all(|c| matches!(c, 'r' | 'n' | 'E')) {
        return false;
      }
      cursor = flags[end..].trim_start();
      continue;
    }
    break;
  }

  // Then exactly one single-quoted pattern.
  let Some(after_open) = cursor.strip_prefix('\'') else {
    return false;
  };
  let Some(close) = after_open.find('\'') else {
    return false;
  };
  let mut tail = after_open[close + 1..].trim_start();

  // Then path arguments carrying no shell metacharacter.
  while !tail.is_empty() {
    let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
    let arg = &tail[..end];
    if arg
      .chars()
      .any(|c| matches!(c, '\'' | '|' | ';' | '&' | '<' | '>' | '$' | '`'))
    {
      return false;
    }
    tail = tail[end..].trim_start();
  }

  true
}

/// Pull the pattern out of a line already known to be simple.
fn pattern_of(line: &str) -> Option<String> {
  let open = line.find('\'')?;
  let rest = &line[open + 1..];
  let close = rest.find('\'')?;
  Some(rest[..close].to_string())
}

/// The `Greppable proxy` fenced block inside a rule's `## Detection` section.
///
/// Ported from v2's awk verbatim in its ordering, because the ordering is the
/// specification: the section gate comes first, the proxy marker second, the
/// fence third. A block that appears before the marker is not a proxy block,
/// and a `## ` heading ends the search whether or not a fence was ever opened.
pub fn extract_greppable_block(body: &str) -> String {
  let mut in_detection = false;
  let mut after_marker = false;
  let mut in_block = false;
  let mut out = String::new();

  for line in body.lines() {
    if line.trim_end() == "## Detection" {
      in_detection = true;
      continue;
    }
    if in_detection && line.starts_with("## ") {
      break;
    }
    if in_detection
      && !after_marker
      && (line.contains("Greppable proxy") || line.contains("greppable proxy"))
    {
      after_marker = true;
      continue;
    }
    if in_detection && after_marker && !in_block && line.trim_end() == "```bash" {
      in_block = true;
      continue;
    }
    if in_detection && in_block && line.trim_end() == "```" {
      break;
    }
    if in_detection && in_block {
      out.push_str(line);
      out.push('\n');
    }
  }
  out
}

/// Every acceptable pattern in a proxy block, plus whether any line was refused.
///
/// **REFUSAL IS PER-RULE, NOT PER-LINE, AND IT IS RETURNED RATHER THAN
/// PRINTED.** v2 emits one deduped stderr note per rule; carrying the fact back
/// to the caller instead lets the exit code depend on it, which is the half v2
/// does through a separate `CRITIC_REFUSED` flag.
pub fn patterns_from_block(block: &str) -> (Vec<String>, bool) {
  let mut patterns = Vec::new();
  let mut refused = false;
  for line in block.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
      continue;
    }
    if proxy_is_simple(line) {
      if let Some(p) = pattern_of(line) {
        patterns.push(p);
      }
    } else {
      refused = true;
    }
  }
  (patterns, refused)
}

/// Read a scalar out of a rule file's YAML-ish frontmatter.
///
/// Deliberately line-oriented and deliberately not a YAML parser: the fields
/// this needs (`critic_tool`, `critic_tool_context`) are single-line scalars,
/// and pulling a parser in to read two of them would make the critic's
/// behaviour depend on a grammar nothing else here uses.
fn frontmatter_scalar(body: &str, key: &str) -> Option<String> {
  let mut lines = body.lines();
  if lines.next()?.trim_end() != "---" {
    return None;
  }
  for line in lines {
    if line.trim_end() == "---" {
      return None;
    }
    if let Some(v) = line.strip_prefix(&format!("{key}:")) {
      let v = v.trim();
      if v.is_empty() {
        return None;
      }
      return Some(v.trim_matches('"').to_string());
    }
  }
  None
}

/// Tabs to four spaces, then cut to 200 characters -- v2's `sed`/`cut` pair.
///
/// **THE CUT IS BY CHARACTER, NOT BY BYTE.** `cut -c` on a UTF-8 line and a
/// byte slice in Rust disagree the moment a rule matches a line containing any
/// non-ASCII character, and a byte slice would panic on a split codepoint
/// rather than merely differ. The finding is a report, not data; truncating it
/// mid-character is a cosmetic bug, panicking on it is a gate outage.
fn truncate_content(line: &str) -> String {
  let expanded = line.replace('\t', "    ");
  expanded.chars().take(200).collect()
}

/// The `disabled:` list from a project's `.intent_critic.yml`.
///
/// **NOT A YAML PARSER, AND THE KEY IS `disabled`.** v2 reads exactly two
/// shapes at `critic_runner.sh:340,353` -- an inline `disabled: [A, B]` and a
/// block list under `disabled:` -- and nothing else in this file is consulted
/// by the runner. (`critic_runner.sh:485`'s comment calls the field
/// `disabled_rules`; the code reads `disabled`. The comment is stale and the
/// template agrees with the code.)
///
/// **AN UNKNOWN RULE ID IS TOLERATED SILENTLY, WHICH IS v2'S BEHAVIOUR AND IS
/// RIGHT HERE**: a project pinning a rule id that a later Intent renames should
/// not have its commits refused by its own opt-out.
pub fn parse_disabled(text: &str) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  let mut in_block = false;
  for line in text.lines() {
    let trimmed = line.trim_end();
    if let Some(rest) = trimmed.strip_prefix("disabled:") {
      let rest = rest.trim();
      if let Some(inner) = rest.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
        for item in inner.split(',') {
          let id = item.trim().trim_matches(['"', '\'']).trim();
          if !id.is_empty() {
            out.insert(id.to_string());
          }
        }
        return out;
      }
      if rest.is_empty() {
        in_block = true;
      }
      continue;
    }
    if in_block {
      let t = trimmed.trim_start();
      if let Some(item) = t.strip_prefix("- ") {
        // A trailing `# reason: ...` is the documented convention.
        let id = item.split('#').next().unwrap_or("").trim();
        let id = id.trim_matches(['"', '\'']).trim();
        if !id.is_empty() {
          out.insert(id.to_string());
        }
        continue;
      }
      if !t.is_empty() {
        in_block = false;
      }
    }
  }
  out
}

/// Read an inline `[A, B, C]` list out of a rule's frontmatter.
///
/// Only the inline form, because that is the only form `critic_tool_codes` is
/// written in and the only form v2's `rule_fm_list` is fed here.
fn frontmatter_list(body: &str, key: &str) -> Vec<String> {
  // The inline form: `critic_tool_codes: [SC2086, SC2046]`.
  if let Some(raw) = frontmatter_scalar(body, key) {
    let inner = raw.trim().trim_start_matches('[').trim_end_matches(']');
    return inner
      .split(',')
      .map(|t| t.trim().trim_matches(['"', '\'']).to_string())
      .filter(|t| !t.is_empty())
      .collect();
  }
  // **THE BLOCK FORM IS NOT AN ALTERNATIVE SPELLING, IT IS THE ONE
  // `applies_to` ACTUALLY USES.** Supporting only the inline form read every
  // `applies_to` as absent, and an absent `applies_to` means UNIVERSAL -- so
  // the omission did not fail closed, it made every scoped rule fire on every
  // file. Measured: `IN-EX-TEST-002` (`test/**/*_test.exs`) reporting against
  // a file under `lib/`.
  let mut out = Vec::new();
  let mut lines = body.lines();
  if lines.next().map(|l| l.trim_end()) != Some("---") {
    return out;
  }
  let mut inside = false;
  for line in lines {
    let t = line.trim_end();
    if t == "---" {
      break;
    }
    if t.trim_start() == format!("{key}:") {
      inside = true;
      continue;
    }
    if inside {
      let l = t.trim_start();
      if let Some(item) = l.strip_prefix("- ") {
        let v = item.trim().trim_matches(['"', '\'']).to_string();
        if !v.is_empty() {
          out.push(v);
        }
        continue;
      }
      // Any non-item line at frontmatter level ends the block.
      if !l.is_empty() {
        break;
      }
    }
  }
  out
}

/// A rule's `applies_to` glob as an anchored regex.
///
/// **ANCHORED AS `(^|/)<re>$` SO UMBRELLA LAYOUTS MATCH (ST0038).** A rule
/// declared `lib/**/*.ex` must fire on `apps/control/lib/foo.ex` as well as on
/// `lib/foo.ex`, so the pattern is suffix-anchored rather than rooted.
///
/// Substitution ORDER is the specification and it is why the placeholders
/// exist: `**/` must be consumed before `**`, and `**` before `*`, or a single
/// `*` rewrite eats half of every double.
fn glob_to_regex(glob: &str) -> String {
  let mut g = glob.replace('.', "\\.");
  g = g.replace("**/", "\u{1}");
  g = g.replace("**", "\u{2}");
  g = g.replace('*', "[^/]*");
  g = g.replace('\u{1}', "(.*/)?");
  g = g.replace('\u{2}', ".*");
  format!("(^|/){g}$")
}

/// Does this rule's `applies_to` admit this file?
///
/// **NO `applies_to` MEANS UNIVERSAL, WHICH IS WHY GETTING THE PARSE WRONG IS
/// DANGEROUS IN THE FIRING DIRECTION.** An absent declaration is a rule that
/// applies everywhere; a declaration nobody could read looks identical to one.
fn applies_to_file(globs: &[String], file: &Path) -> bool {
  if globs.is_empty() {
    return true;
  }
  let path = file.to_string_lossy();
  globs.iter().any(|g| {
    regex::Regex::new(&glob_to_regex(g))
      .map(|re| re.is_match(&path))
      .unwrap_or(false)
  })
}

/// Drive `shellcheck` for one rule over one file.
///
/// **THE RULE'S DECLARED CODES SELECT WHICH FINDINGS ARE ITS BUSINESS.** Without
/// that filter every shellcheck-armed rule would claim every shellcheck finding,
/// so two rules would each report the whole file.
///
/// **NO `--enable`, NO `--severity`, NO `--shell`.** A flag that changes what
/// the tool SEES is the runner's judgement, never a rule file's -- v2 records
/// `IN-RS-CODE-001` as the case that proves it, where the remedy is the ABSENCE
/// of a flag.
///
/// **DEDUPED ON THE LINE, matching the grep path.** shellcheck reports per
/// COLUMN, so one line carrying two defects this rule owns arrives twice and
/// renders as the same line printed twice -- two identical lines read as two
/// defects and any count taken off them overstates.
fn shellcheck_findings(
  file: &Path,
  text: &str,
  rule_id: &str,
  severity: Severity,
  codes: &[String],
) -> Vec<Finding> {
  if codes.is_empty() {
    return Vec::new();
  }
  let Ok(out) = std::process::Command::new("shellcheck")
    .arg("--format=gcc")
    .arg(file)
    .output()
  else {
    return Vec::new();
  };
  let stdout = String::from_utf8_lossy(&out.stdout);
  let mut seen: BTreeSet<usize> = BTreeSet::new();
  let mut findings = Vec::new();
  for line in stdout.lines() {
    if !codes.iter().any(|c| line.contains(&format!("[{c}]"))) {
      continue;
    }
    // `--format=gcc` is `file:line:col: severity: message [SCxxxx]`.
    let Some(line_no) = line
      .split(':')
      .nth(1)
      .and_then(|n| n.trim().parse::<usize>().ok())
    else {
      continue;
    };
    if !seen.insert(line_no) {
      continue;
    }
    let content = text.lines().nth(line_no - 1).unwrap_or_default();
    findings.push(Finding {
      rule_id: rule_id.to_string(),
      severity,
      path: file.to_path_buf(),
      line_no,
      line: truncate_content(content),
    });
  }
  findings
}

/// Is a named critic tool on this machine?
///
/// **IT ASKS THE OPERATING SYSTEM RATHER THAN READING `$PATH`, AND THAT IS
/// AC-11.3 RATHER THAN A STYLE CHOICE.** The first version walked
/// `env::var("PATH")` by hand, which made `no_intent_home` fail with
/// `critic.rs: reads $PATH` -- the shipped surface reads exactly one
/// environment variable and that test says a further read needs an hv ruling
/// and a row in `ALLOWED`, not a quiet addition. **I refused an ext-rule-pack
/// resolver on that same wall and deleted the module rather than allowlist
/// through it, then walked into it here myself** (found by ic, 2026-08-20).
///
/// **NO RULING IS NEEDED, BECAUSE THE READ WAS NEVER REQUIRED.** A child
/// process inherits `PATH` whether or not the parent looks at it, so spawning
/// resolves the tool without the surface reading anything.
///
/// **AND SPAWNING FIXES A SECOND DEFECT IN THE SAME LINE, WHICH IS THE ONE THAT
/// MATTERED (ic).** The hand-rolled walk tested `candidate.is_file()`, which
/// does not check the executable bit -- so a NON-EXECUTABLE regular file named
/// `shellcheck` anywhere on `PATH` reported the tool AVAILABLE. That feeds the
/// arming census, producing a rule counted ASKED that could never have been
/// asked: **a false clean, and the same shape as the tool-armed rules this
/// module already got wrong once.** The OS enforces the bit; a manual walk has
/// to remember to, and this one did not.
///
/// `.status().is_ok()` tests that the process STARTED, not what it answered --
/// a tool that rejects `--version` is still present, and a non-zero exit is an
/// answer rather than an absence.
///
/// Cost is one spawn per per-file tool. Context is tested BEFORE availability,
/// so a workspace analyser never reaches this: on the current library that is
/// two `shellcheck` probes on a `critic shell` run and none on any other.
fn tool_available(tool: &str) -> bool {
  // The first word only: `cargo clippy` is available iff `cargo` is.
  let exe = tool.split_whitespace().next().unwrap_or(tool);
  std::process::Command::new(exe)
    .arg("--version")
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null())
    .status()
    .is_ok()
}

/// Classify one rule on both axes, and collect its patterns if it has any.
fn classify(body: &str) -> (Arming, Disposition, String, Vec<String>) {
  if let Some(tool) = frontmatter_scalar(body, "critic_tool") {
    let ctx = frontmatter_scalar(body, "critic_tool_context").unwrap_or_else(|| "per-file".into());
    // **CONTEXT IS TESTED BEFORE AVAILABILITY AND THE ORDER IS THE RULING.**
    // Reporting `tool-absent` for a workspace analyser during a per-file run
    // states a fact about the host in answer to a question about the
    // invocation, and the reader would go and install something that still
    // would not run. Absence matters in the run that WOULD have used it.
    let disposition = if ctx != "per-file" {
      Disposition::OutOfContext(tool.clone())
    } else if !tool_available(&tool) {
      Disposition::ToolAbsent(tool.clone())
    } else {
      Disposition::Ran
    };
    return (Arming::Armed, disposition, tool, Vec::new());
  }

  let block = extract_greppable_block(body);
  if !block.trim().is_empty() {
    let (patterns, _refused) = patterns_from_block(&block);
    if !patterns.is_empty() {
      return (Arming::Armed, Disposition::Ran, "grep".into(), patterns);
    }
    return (
      Arming::Unrunnable,
      Disposition::NotApplicable,
      "-".into(),
      Vec::new(),
    );
  }

  if body
    .to_lowercase()
    .contains("no greppable proxy is authoritative")
  {
    (
      Arming::Declared,
      Disposition::NotApplicable,
      "-".into(),
      Vec::new(),
    )
  } else {
    (
      Arming::Undeclared,
      Disposition::NotApplicable,
      "-".into(),
      Vec::new(),
    )
  }
}

/// Run the mechanical critic for one language over an explicit file list.
///
/// **THE FILE LIST IS EXPLICIT AND THERE IS NO DEFAULT SCAN.** v2 scans nothing
/// unless `--files` or `--staged` is given, and that is right: a critic that
/// walks the tree when asked nothing turns a gate into a whole-repo audit at
/// every commit, and the operator who wanted one file gets a wall.
/// **`disabled` IS APPLIED BEFORE CLASSIFICATION, NOT AFTER MATCHING**, so a
/// disabled rule contributes no census row at all. Filtering findings instead
/// would leave the rule counted as ASKED, and the census would then report a
/// question this run deliberately did not put -- the exact defect the two-axis
/// census exists to prevent, arriving through the opt-out.
pub fn run(
  lib: &Library,
  lang: &str,
  files: &[PathBuf],
  severity_min: Severity,
  disabled: &BTreeSet<String>,
) -> Result<Report, CriticError> {
  let all = lib.rules()?;
  let mut census = Vec::new();
  let mut findings = Vec::new();
  let mut refused: BTreeSet<String> = BTreeSet::new();

  // Read each candidate file ONCE rather than once per rule. Twelve armed rules
  // against a staged set is twelve reads of the same bytes otherwise, and the
  // gate runs on every commit.
  let mut contents: Vec<(PathBuf, String)> = Vec::new();
  for f in files {
    match std::fs::read_to_string(f) {
      Ok(text) => contents.push((f.clone(), text)),
      // **A FILE THAT IS NOT UTF-8 IS SKIPPED, NOT AN ERROR.** A staged binary
      // is an ordinary thing to commit and refusing the whole run over one
      // would make the gate unusable; a rule cannot match bytes it cannot read
      // and says nothing about them either way.
      Err(e) if e.kind() == std::io::ErrorKind::InvalidData => continue,
      Err(source) => {
        return Err(CriticError::Read {
          path: f.clone(),
          source,
        });
      }
    }
  }

  for rule in all
    .iter()
    .filter(|r| r.language == lang && !disabled.contains(&r.id))
  {
    let Some((_, body)) = lib.show(&rule.id)? else {
      continue;
    };
    let (arming, disposition, by, patterns) = classify(&body);

    if arming == Arming::Unrunnable {
      refused.insert(rule.id.clone());
    }

    // **A NON-ACTIVE RULE STILL APPEARS IN THE CENSUS AND NEVER FIRES, WHICH IS
    // v2'S SHAPE AND IS DELIBERATE TO PRESERVE.** `critic_arming_census` does
    // not read `status` while `critic_apply_rule` does, so a retired rule is
    // counted among what COULD be asked and is not asked. Reproduced rather
    // than tidied: the census is a statement about the library, and dropping
    // the row would quietly shrink the denominator every reader is comparing
    // against. All 64 rules are `active` today, so this arm has no live
    // population.
    let active = frontmatter_scalar(&body, "status")
      .map(|v| v == "active")
      .unwrap_or(true);

    // **`applies_to` IS A PER-FILE FILTER, SO IT CANNOT LIVE IN `classify`.**
    // An absent declaration means UNIVERSAL, which is why failing to read one
    // fires the rule everywhere rather than nowhere -- the dangerous direction.
    let globs = frontmatter_list(&body, "applies_to");
    let applicable: Vec<&(PathBuf, String)> = contents
      .iter()
      .filter(|(path, _)| applies_to_file(&globs, path))
      .collect();

    // **A TOOL-ARMED RULE MUST ACTUALLY RUN ITS TOOL, OR THE CENSUS LIES.**
    // The first version of this module classified these `armed`/`ran` and then
    // produced no findings, because the finding loop below is gated on having
    // grep patterns. **The census output was byte-identical to v2's, so a
    // parity check on the REPORT passed while the ACT diverged** -- two rules
    // counted as ASKED that were never put. That is this command's founding
    // defect arriving through the one path that looks like success.
    if active && disposition == Disposition::Ran && patterns.is_empty() && by != "grep" && by != "-"
    {
      // **AN EMPTY SEVERITY DEFAULTS TO `warning`; A NON-EMPTY ONE THAT DOES
      // NOT PARSE IS AN ERROR.** v2 defaults the empty case the same way. The
      // two are not the same state: nothing declared is an omission with an
      // obvious reading, whereas `severity: crticial` is a broken rule that
      // would silently rank below every filter (IN-AG-NO-SILENT-001).
      let severity = if rule.severity.trim().is_empty() {
        Severity::Warning
      } else {
        Severity::parse(&rule.severity).ok_or_else(|| CriticError::Uncompilable {
          rule_id: rule.id.clone(),
          detail: format!("severity `{}` is not one this build knows", rule.severity),
        })?
      };
      let codes = frontmatter_list(&body, "critic_tool_codes");
      match by.as_str() {
        "shellcheck" => {
          if severity.clears(severity_min) {
            for (path, text) in &applicable {
              findings.extend(shellcheck_findings(path, text, &rule.id, severity, &codes));
            }
          }
        }
        // v2 no-ops clippy here too. It is unreachable in practice: all three
        // clippy rules declare `workspace` context and are caught by the
        // out-of-context arm before they ever arrive.
        "clippy" => {}
        // **AN UNDRIVABLE TOOL IS A REFUSAL, WHICH FOLLOWS v2'S COMMENT RATHER
        // THAN v2'S CODE, AND THE DIVERGENCE IS DELIBERATE AND STATED.** v2
        // prints a stderr note and returns 0, while the comment directly above
        // that function says an unknown tool "is a REFUSAL and never a silent
        // skip". The gate's own definition of 3 is the same sentence: the
        // project armed a rule, it could not be enforced, and the remedies are
        // the developer's. **Blast radius today is nil -- the library names
        // exactly two tools, shellcheck and clippy, so this arm has NO live
        // population** and needs a fixture rather than an estate run.
        _ => {
          refused.insert(rule.id.clone());
        }
      }
    }

    if active && disposition == Disposition::Ran && !patterns.is_empty() {
      // **AN EMPTY SEVERITY DEFAULTS TO `warning`; A NON-EMPTY ONE THAT DOES
      // NOT PARSE IS AN ERROR.** v2 defaults the empty case the same way. The
      // two are not the same state: nothing declared is an omission with an
      // obvious reading, whereas `severity: crticial` is a broken rule that
      // would silently rank below every filter (IN-AG-NO-SILENT-001).
      let severity = if rule.severity.trim().is_empty() {
        Severity::Warning
      } else {
        Severity::parse(&rule.severity).ok_or_else(|| CriticError::Uncompilable {
          rule_id: rule.id.clone(),
          detail: format!("severity `{}` is not one this build knows", rule.severity),
        })?
      };
      if severity.clears(severity_min) {
        // **MULTI-PATTERN UNION, DEDUPED ON (file, line) WITHIN THE RULE.**
        // v2 accumulates every pattern's hits and `sort -u`s them, so two
        // patterns of the SAME rule striking one line report once. Across
        // rules they still both report, because two rules objecting to one
        // line is two facts rather than a duplicate -- which is why the key
        // is scoped to this rule and reset below.
        let mut hits: BTreeSet<(PathBuf, usize)> = BTreeSet::new();
        for pat in &patterns {
          // **ALWAYS ERE, WHATEVER THE PROXY LINE'S FLAGS SAID.** v2 runs
          // `grep -nE "$pattern" "$file"` and discards the proxy's own flags
          // and path arguments entirely, so `-E` is not a property of the
          // line, it is the engine. Honouring the flags here would make v3
          // stricter than v2 on the BRE lines and diverge the gate.
          let re = regex::Regex::new(pat).map_err(|e| CriticError::Uncompilable {
            rule_id: rule.id.clone(),
            detail: e.to_string(),
          })?;
          for (path, text) in &applicable {
            for (i, line) in text.lines().enumerate() {
              if re.is_match(line) {
                hits.insert((path.clone(), i + 1));
              }
            }
          }
        }
        for (path, line_no) in hits {
          let line = applicable
            .iter()
            .find(|(p, _)| *p == path)
            .and_then(|(_, t)| t.lines().nth(line_no - 1))
            .unwrap_or_default();
          findings.push(Finding {
            rule_id: rule.id.clone(),
            severity,
            path,
            line_no,
            line: truncate_content(line),
          });
        }
      }
    }

    census.push(CensusRow {
      rule_id: rule.id.clone(),
      arming,
      disposition,
      by,
    });
  }

  Ok(Report {
    lang: lang.to_string(),
    findings,
    census,
    refused: refused.into_iter().collect(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  // ---- the injection boundary -------------------------------------------
  //
  // **THESE ARE THE MOST LOAD-BEARING TESTS IN THE FILE.** `proxy_is_simple`
  // decides which strings from a markdown file this runner will execute as a
  // pattern. Every refusal below is a shape that was refused for a reason, and
  // relaxing any of them is a decision that belongs to ST0039 rather than to
  // whoever is making a rule fire today.

  #[test]
  fn the_documented_shape_is_admitted() {
    assert!(proxy_is_simple("grep -rnE 'Process\\.sleep\\(' test/"));
    assert!(proxy_is_simple("grep -rn 'def user_fixture' test/"));
    assert!(proxy_is_simple("grep 'plain'"));
    assert!(proxy_is_simple("grep --include=*.ex -rn 'x' lib/"));
    // Leading and trailing whitespace is the block's indentation, not content.
    assert!(proxy_is_simple("  grep -rnE 'x' lib/  "));
  }

  #[test]
  fn context_flags_are_refused_because_the_neighbouring_line_is_the_point() {
    // A rule whose violation is the ABSENCE of an adjacent justification cannot
    // be proxied at all. Admitting -A/-B would let such a rule half-fire and
    // report the construct while never evaluating the condition.
    assert!(!proxy_is_simple("grep -A2 'x' lib/"));
    assert!(!proxy_is_simple("grep -B5 'x' lib/"));
    assert!(!proxy_is_simple("grep -rnA 2 'x' lib/"));
  }

  #[test]
  fn inverting_and_listing_flags_are_refused() {
    // -L and -v inverate the match; -l/-c/-o change what a "hit" IS, so a
    // finding's line number would stop meaning what the renderer prints.
    for line in [
      "grep -v 'x' lib/",
      "grep -L 'x' lib/",
      "grep -l 'x' lib/",
      "grep -c 'x' lib/",
      "grep -o 'x' lib/",
      "grep -w 'x' lib/",
      "grep -x 'x' lib/",
    ] {
      assert!(!proxy_is_simple(line), "should refuse: {line}");
    }
  }

  #[test]
  fn a_pipeline_cannot_arrive_disguised_as_an_argument() {
    // This is the injection case proper: everything here parses as "grep, a
    // pattern, some args" to a lazy reader.
    for line in [
      "grep -rn 'x' lib/ | xargs rm",
      "grep -rn 'x' lib/; rm -rf /",
      "grep -rn 'x' lib/ && curl evil",
      "grep -rn 'x' $(echo lib)",
      "grep -rn 'x' `echo lib`",
      "grep -rn 'x' lib/ > /dev/null",
    ] {
      assert!(!proxy_is_simple(line), "should refuse: {line}");
    }
  }

  #[test]
  fn a_line_that_is_not_a_grep_is_not_a_candidate() {
    assert!(!proxy_is_simple(""));
    assert!(!proxy_is_simple("   "));
    assert!(!proxy_is_simple("# a comment"));
    assert!(!proxy_is_simple("rg -n 'x' lib/"));
    // `grepfoo` is not `grep`: the command must end at whitespace.
    assert!(!proxy_is_simple("grepfoo 'x'"));
    // No quoted pattern at all.
    assert!(!proxy_is_simple("grep -rn lib/"));
  }

  // ---- applies_to, the scoping regression --------------------------------

  #[test]
  fn a_test_scoped_rule_does_not_fire_on_lib_code() {
    // **THE REGRESSION THIS FILE EXISTS FOR.** The first version of this module
    // read only inline frontmatter lists, so every block-form `applies_to` read
    // as ABSENT -- and absent means UNIVERSAL. It did not fail closed; it made
    // every scoped rule fire on every file. Caught by diffing against v2, which
    // reported one finding where this reported two.
    let globs = vec!["test/**/*_test.exs".to_string()];
    assert!(!applies_to_file(&globs, Path::new("/tmp/x/lib/fixture.ex")));
    assert!(applies_to_file(
      &globs,
      Path::new("/tmp/x/test/demo_test.exs")
    ));
  }

  #[test]
  fn a_glob_matches_umbrella_layouts_as_well_as_top_level() {
    // ST0038: `lib/**/*.ex` must fire on `apps/control/lib/foo.ex`, which is
    // why the regex is suffix-anchored rather than rooted.
    let globs = vec!["lib/**/*.ex".to_string()];
    assert!(applies_to_file(&globs, Path::new("lib/foo.ex")));
    assert!(applies_to_file(
      &globs,
      Path::new("apps/control/lib/foo.ex")
    ));
    assert!(applies_to_file(&globs, Path::new("lib/deep/nested/foo.ex")));
    assert!(!applies_to_file(&globs, Path::new("test/foo.ex")));
    // A single `*` is one path component, so it must not cross a slash.
    let one = vec!["lib/*.ex".to_string()];
    assert!(one.len() == 1 && applies_to_file(&one, Path::new("lib/foo.ex")));
    assert!(!applies_to_file(&one, Path::new("lib/deep/foo.ex")));
  }

  #[test]
  fn no_applies_to_means_universal() {
    assert!(applies_to_file(&[], Path::new("anything/at/all.txt")));
  }

  #[test]
  fn a_dot_in_a_glob_is_a_literal_dot() {
    // Without escaping, `*.ex` would match `fooXex`.
    let globs = vec!["lib/*.ex".to_string()];
    assert!(!applies_to_file(&globs, Path::new("lib/fooXex")));
  }

  // ---- the frontmatter readers -------------------------------------------

  #[test]
  fn both_list_forms_are_read() {
    let inline = "---\ncritic_tool_codes: [SC2086, SC2046]\n---\nbody";
    assert_eq!(
      frontmatter_list(inline, "critic_tool_codes"),
      ["SC2086", "SC2046"]
    );
    let block = "---\napplies_to:\n  - \"test/**/*_test.exs\"\n  - \"lib/**/*.ex\"\nstatus: active\n---\nbody";
    assert_eq!(
      frontmatter_list(block, "applies_to"),
      ["test/**/*_test.exs", "lib/**/*.ex"]
    );
    // A key that is absent is an empty list, not a panic.
    assert!(frontmatter_list(inline, "applies_to").is_empty());
  }

  #[test]
  fn the_disabled_list_is_read_in_both_forms_and_ignores_reasons() {
    let inline = "severity_min: warning\ndisabled: [IN-EX-CODE-001, IN-SH-CODE-002]\n";
    let got = parse_disabled(inline);
    assert!(got.contains("IN-EX-CODE-001") && got.contains("IN-SH-CODE-002"));
    // The documented convention is a trailing `# reason: ...` on each entry.
    let block = "disabled:\n  - IN-EX-CODE-001 # reason: legacy module\n  - IN-SH-CODE-002\n";
    let got = parse_disabled(block);
    assert!(
      got.contains("IN-EX-CODE-001"),
      "a trailing reason must not become part of the id"
    );
    assert!(got.contains("IN-SH-CODE-002"));
    assert_eq!(got.len(), 2);
    // An empty list disables nothing -- and must not disable everything.
    assert!(parse_disabled("disabled: []\n").is_empty());
  }

  // ---- the proxy block ----------------------------------------------------

  #[test]
  fn the_block_is_found_only_after_the_marker_inside_detection() {
    let body = "## Detection\n\nSome prose.\n\n```bash\nnot-the-proxy\n```\n\nGreppable proxy:\n\n```bash\ngrep -rn 'yes' lib/\n```\n";
    let block = extract_greppable_block(body);
    assert!(block.contains("grep -rn 'yes' lib/"));
    assert!(
      !block.contains("not-the-proxy"),
      "a fence BEFORE the marker is not a proxy block"
    );
  }

  #[test]
  fn a_later_section_ends_the_search() {
    let body = "## Detection\n\nGreppable proxy:\n\n## Bad\n\n```bash\ngrep -rn 'no' lib/\n```\n";
    assert!(extract_greppable_block(body).trim().is_empty());
  }

  #[test]
  fn a_block_whose_every_line_is_refused_yields_no_patterns() {
    // This is the `unrunnable` arming state, and it drives exit 3.
    //
    // **NO RULE IN THE SHIPPED LIBRARY IS IN THIS STATE -- MEASURED
    // 2026-08-20, ZERO ACROSS ALL FIVE LANGUAGES.** So this arm cannot be
    // exercised by any estate run, and a green over the estate would say
    // nothing about it. That is the reason it is a constructed fixture and the
    // reason the zero is written down here rather than left implicit.
    let (patterns, refused) = patterns_from_block("grep -A2 'x' lib/\ngrep -v 'y' lib/\n");
    assert!(patterns.is_empty());
    assert!(refused, "a block of refused lines must report the refusal");
  }

  #[test]
  fn a_mixed_block_keeps_the_runnable_lines_and_still_reports_the_refusal() {
    let (patterns, refused) = patterns_from_block("grep -rn 'ok' lib/\ngrep -A2 'no' lib/\n");
    assert_eq!(patterns, ["ok"]);
    assert!(refused);
  }

  // ---- the exit contract --------------------------------------------------

  #[test]
  fn an_absent_tool_refuses_and_an_unrunnable_proxy_does_not() {
    // **THE DEFECT THIS GUARDS WAS LIVE AND FAILED OPEN (vc, 2026-08-20).**
    // `exit_code` keyed on the unrunnable-PROXY set while the header table said
    // 3 meant an armed rule that could not be enforced -- two populations, five
    // lines apart, one file. v2 sets `CRITIC_REFUSED` in exactly one place
    // (`bin/intent_critic:319`, the `c_absent` block), so a machine missing
    // shellcheck refused there and PASSED here.
    let base = Report {
      lang: "shell".into(),
      findings: Vec::new(),
      census: Vec::new(),
      refused: Vec::new(),
    };

    let absent = Report {
      census: vec![CensusRow {
        rule_id: "IN-SH-CODE-001".into(),
        arming: Arming::Armed,
        disposition: Disposition::ToolAbsent("shellcheck".into()),
        by: "shellcheck".into(),
      }],
      ..base.clone()
    };
    assert_eq!(
      absent.exit_code(),
      3,
      "an armed rule whose tool is absent must BLOCK"
    );
    assert_eq!(absent.unenforced(), ["IN-SH-CODE-001"]);

    // An unrunnable proxy is reported and never refuses -- our defect, not the
    // project's, and nobody can act on it.
    let unrunnable = Report {
      refused: vec!["IN-SH-CODE-009".into()],
      census: vec![CensusRow {
        rule_id: "IN-SH-CODE-009".into(),
        arming: Arming::Unrunnable,
        disposition: Disposition::NotApplicable,
        by: "-".into(),
      }],
      ..base.clone()
    };
    assert_eq!(
      unrunnable.exit_code(),
      0,
      "an unrunnable proxy must not block"
    );

    // **OUT-OF-CONTEXT MUST NOT REFUSE EITHER, and this arm is load-bearing:**
    // all three clippy rules are `workspace`, so refusing here would block
    // every rust commit in the estate.
    let ooc = Report {
      census: vec![CensusRow {
        rule_id: "IN-RS-CODE-001".into(),
        arming: Arming::Armed,
        disposition: Disposition::OutOfContext("clippy".into()),
        by: "clippy".into(),
      }],
      ..base
    };
    assert_eq!(
      ooc.exit_code(),
      0,
      "a workspace analyser out of context must not block"
    );
  }

  #[test]
  fn refusal_outranks_findings_and_neither_is_two() {
    // **THE WHOLE POINT OF THIS FILE.** `pre-commit.sh` blocks on 1 and 3 and
    // FAILS OPEN on 2, so a critic that returned 2 for findings would silently
    // disarm the gate in every repository that runs it.
    let base = Report {
      lang: "shell".into(),
      findings: Vec::new(),
      census: Vec::new(),
      refused: Vec::new(),
    };
    assert_eq!(base.exit_code(), 0);

    let finding = Finding {
      rule_id: "IN-SH-CODE-001".into(),
      severity: Severity::Critical,
      path: PathBuf::from("x.sh"),
      line_no: 1,
      line: "echo $x".into(),
    };
    let with_findings = Report {
      findings: vec![finding.clone()],
      ..base.clone()
    };
    assert_eq!(with_findings.exit_code(), 1);

    let with_refusal = Report {
      census: vec![CensusRow {
        rule_id: "IN-SH-CODE-001".into(),
        arming: Arming::Armed,
        disposition: Disposition::ToolAbsent("shellcheck".into()),
        by: "shellcheck".into(),
      }],
      ..base.clone()
    };
    assert_eq!(with_refusal.exit_code(), 3);

    // A clean run that refused is not a clean run, and a run with findings that
    // ALSO refused must report the refusal: the operator's remedy differs.
    let both = Report {
      findings: vec![finding],
      census: vec![CensusRow {
        rule_id: "IN-SH-CODE-001".into(),
        arming: Arming::Armed,
        disposition: Disposition::ToolAbsent("shellcheck".into()),
        by: "shellcheck".into(),
      }],
      ..base
    };
    assert_eq!(both.exit_code(), 3);
  }

  #[test]
  fn every_headless_language_has_a_rule_pack() {
    // The two rosters are deliberately different sizes, but they cannot be
    // unrelated: a language with a headless critic and no rule pack would
    // advertise itself in `--languages`, accept a run, and find nothing --
    // a clean result over a library that does not exist.
    for l in HEADLESS_LANGUAGES {
      assert!(
        crate::rules::LANGUAGES.contains(&l),
        "{l} has a headless critic and no rule pack"
      );
    }
  }

  #[test]
  fn severity_orders_so_a_minimum_reads_as_at_least_this_serious() {
    assert!(Severity::Critical.clears(Severity::Warning));
    assert!(Severity::Warning.clears(Severity::Warning));
    assert!(!Severity::Recommendation.clears(Severity::Warning));
    assert!(!Severity::Style.clears(Severity::Warning));
    assert!(Severity::Style.clears(Severity::Style));
    // An unknown severity is None, never a silent default.
    assert!(Severity::parse("crticial").is_none());
  }

  /// Truncation counts CHARACTERS, never bytes.
  ///
  /// The emphasis was in the function NAME and clippy's `non_snake_case` is
  /// denied in CI, so it read green locally and red on both platforms. A test
  /// name is not a place for shouting; a doc comment is, and it is the one a
  /// reader sees in the failure output anyway.
  #[test]
  fn content_is_truncated_by_character_not_by_byte() {
    // A byte slice would panic on a split codepoint rather than merely differ,
    // which turns a cosmetic bug into a gate outage.
    let line = "é".repeat(300);
    let out = truncate_content(&line);
    assert_eq!(out.chars().count(), 200);
    let tabs = truncate_content("\tx");
    assert_eq!(tabs, "    x");
  }
}
