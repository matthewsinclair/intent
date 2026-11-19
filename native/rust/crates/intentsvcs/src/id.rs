//! **ONE MODULE OWNS A THREAD, WORK PACKAGE AND ISSUE ID, END TO END** (ST0095,
//! hv decision 62: "highlander both the minting of these new ids and any checks
//! on them into as tight a waist as possible").
//!
//! An id is a typed value, [`ThreadId`], [`WpId`] or [`IssueId`], and only this
//! module makes one: from its stored form ([`ThreadId::parse`] and its
//! siblings), from what an operator typed ([`normalise_thread_id`] and its
//! siblings), from a number a wire form carries ([`IssueId::from_ordinal`]), or
//! as the next one ([`ThreadId::next_after`]). Nothing outside it formats,
//! parses, scans or does arithmetic on one. **AN ID HAS NO `Display`, NO NUMERIC
//! ACCESSOR AND NO WIDTH**, so a `{:04}` written against one does not compile,
//! and the compiler lists every site that would do arithmetic on an id or pad
//! it by hand. **THAT GUARD STOPS AT TEXT.** A thread id reads as its stored
//! `&str` ([`ThreadId::as_str`], and `Deref`), so a site that already holds the
//! text can still compose an address from it with a `format!`, and only a
//! review finds that: build a package's address with [`wp_address`] and split
//! one with [`split_wp_address`].
//!
//! **EVERY ID IS AN ORDINAL TODAY**: `ST0092`, issue `0703`, and a work package's
//! `09` under its thread. Each type holds one private `Form`, so a second
//! family is a variant here and an arm in each method, and nowhere else.
//!
//! **STORED AND SHOWN ARE TWO FORMS, AND EVERY CALLER CHOOSES ONE.** The stored
//! form is what canon, file names, `.intentfiles` rows, claims, `related`,
//! event subjects, URIs and `--json` carry, and it is compared whole. The shown
//! form is what a table, a view or a message prints. For an ordinal the two are
//! the same bytes, and a caller that picks the wrong one is wrong by
//! construction on the day they differ, which is why there is no `Display` to
//! pick for it.
//!
//! The canon, store and transport faces of each type are at the end of this
//! file, beside the representation they encode, and nowhere else.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;

/// The prefix of a steel thread's id.
///
/// hv retired `st_prefix` (issue 0040): the prefix is FIXED, and it was spelled
/// four separate times before one home existed for it.
pub const THREAD_PREFIX: &str = "ST";
/// How many digits follow [`THREAD_PREFIX`] in an ordinal. Zero-padded, fixed
/// width.
pub const THREAD_DIGITS: usize = 4;
/// How many digits an ordinal issue id carries. Zero-padded, fixed width, no
/// prefix: `intent/.canon/issues/0001.json` is the on-disk form this describes.
pub const ISSUE_DIGITS: usize = 4;
/// How many digits an ordinal work package id carries under its thread:
/// `ST0092/09`, `intent/st/ST0092/WP/09/`.
pub const WP_DIGITS: usize = 2;

/// How a claim names an ISSUE, as [`crate::intentfiles::Sigil::Issue`] spells
/// it in the manifest, with its separator.
///
/// Declared here rather than read from `intentfiles`, which depends on this
/// module (`Sigil::accepts` asks [`IssueId::parse`]); reaching the other way
/// would invert that, and `the_claim_prefix_is_the_manifests_own_spelling`
/// holds the two identical.
pub const CLAIM_ISSUE_PREFIX: &str = "ISSUE:";

/// The forms [`is_claim_address`] admits, as the sentence a refusal tells the
/// user to type.
///
/// **ONE HOME FOR WHAT EVERY DOOR SAYS, BESIDE THE ONE HOME FOR WHAT IT
/// ADMITS** (0519). `every_form_the_refusal_names_is_one_the_validator_admits`
/// holds each example here to the validator. The reverse is not mechanical: a
/// widening that teaches the validator another kind fails nothing until this
/// sentence and that test's list change with it.
pub const CLAIM_ADDRESS_FORMS: &str =
  "a thread as `ST0000`, a work package as `ST0000/01`, or an issue as `ISSUE:0000`";

/// **THE ONE SENTENCE OF A THREAD ID'S FORM** that every refusal quotes (the
/// Highlander sweep's C13: three refusals each said it in their own words).
pub const THREAD_ID_FORM: &str = "a steel-thread id is ST followed by four digits, eg ST0000";

/// The one sentence of an issue id's form, as [`THREAD_ID_FORM`] is a thread's.
pub const ISSUE_ID_FORM: &str = "an issue id is four digits, eg 0001";

/// Which collection an operator's spelling names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdKind {
  Thread,
  Issue,
}

impl IdKind {
  /// The word an error message uses. Prose, not a wire form.
  pub fn as_str(&self) -> &'static str {
    match self {
      IdKind::Thread => "steel thread",
      IdKind::Issue => "issue",
    }
  }

  /// The noun WITH its article. A message that assembles the article at the
  /// use site says "a issue" the first time somebody adds a vowel, which is
  /// what the first driven run of the operator door did.
  pub fn with_article(&self) -> &'static str {
    match self {
      IdKind::Thread => "a steel thread",
      IdKind::Issue => "an issue",
    }
  }

  /// The tag an operator can type to name this collection explicitly.
  pub fn tag(&self) -> char {
    match self {
      IdKind::Thread => 's',
      IdKind::Issue => 'i',
    }
  }

  /// How many digits this collection's ordinal id carries.
  pub fn digits(&self) -> usize {
    match self {
      IdKind::Thread => THREAD_DIGITS,
      IdKind::Issue => ISSUE_DIGITS,
    }
  }

  /// The sentence that says what this collection's id looks like:
  /// [`THREAD_ID_FORM`] or [`ISSUE_ID_FORM`].
  pub fn form(&self) -> &'static str {
    match self {
      IdKind::Thread => THREAD_ID_FORM,
      IdKind::Issue => ISSUE_ID_FORM,
    }
  }
}

/// Why a spelling does not name what the caller asked for.
///
/// **`WrongCollection` EXISTS SO A CATEGORY ERROR IS NEVER REPORTED AS A
/// NOT-FOUND.** `st show i59` names an issue, and answering it with *no steel
/// thread i59 in this project* sends an operator looking for something that was
/// never addressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
  /// Nothing about the spelling is an id: `foo`, `46abc`, `---`, empty.
  NotAnId,
  /// Digits, but more of them than the fixed width admits.
  OutOfRange,
  /// A well-formed id for the OTHER collection.
  WrongCollection { named: IdKind },
  /// Untagged digits at a door that does not know which collection is meant:
  /// the thread and the issue the digits would name, for a resolver that can
  /// ask the project which exists.
  Ambiguous { thread: ThreadId, issue: IssueId },
}

impl fmt::Display for IdError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      IdError::NotAnId => f.write_str("not an id"),
      IdError::OutOfRange => f.write_str("more digits than an id of its kind carries"),
      IdError::WrongCollection { named } => write!(f, "names {}", named.with_article()),
      IdError::Ambiguous { thread, issue } => write!(
        f,
        "names steel thread {} or issue {}",
        thread.shown(),
        issue.shown()
      ),
    }
  }
}

impl std::error::Error for IdError {}

/// One form an id takes. Private, so a family is a variant here and nowhere
/// else; the derived order is the family key (an ordinal by its number).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Form {
  /// Minted highest-plus-one by every Intent before ST0095, and valid for ever.
  Ordinal(u32),
}

/// A steel thread's id: `ST0092`.
///
/// **IT HOLDS ITS STORED FORM, AND DEREFS TO IT.** A thread id is a string in
/// every place it is kept (canon, paths, claims, `related`, the store), so a
/// caller that needs `&str` gets the stored form, which is the one those places
/// take. What it does not get is a `Display` or a number: a table or a message
/// asks for [`ThreadId::shown`], and nothing outside this module can do
/// arithmetic on one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreadId {
  stored: String,
  form: Form,
}

impl Ord for ThreadId {
  fn cmp(&self, other: &Self) -> Ordering {
    self.form.cmp(&other.form)
  }
}

impl PartialOrd for ThreadId {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl std::ops::Deref for ThreadId {
  type Target = str;

  fn deref(&self) -> &str {
    &self.stored
  }
}

impl AsRef<str> for ThreadId {
  fn as_ref(&self) -> &str {
    &self.stored
  }
}

/// A work package's id under its thread: the `09` of `ST0092/09`. It never
/// stands alone; [`wp_address`] renders it with its thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WpId(Form);

/// An issue's id: `0703`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IssueId(Form);

/// An ordinal's exact stored digits: `width` ASCII digits, nothing else.
fn ordinal_digits(digits: &str, width: usize) -> Result<Form, IdError> {
  if digits.len() != width || !digits.bytes().all(|b| b.is_ascii_digit()) {
    return Err(IdError::NotAnId);
  }
  digits
    .parse()
    .map(Form::Ordinal)
    .map_err(|_| IdError::NotAnId)
}

/// Whether `n` fits an ordinal of `width` digits.
///
/// **`format!("{:0N}")` IS A MINIMUM WIDTH, NOT A MAXIMUM, AND SO IS v2's
/// `printf "ST%04d"`.** v2 turns `99999` into `ST99999` and then reports a
/// missing thread; unchecked, this would reproduce it exactly.
fn fits(n: u32, width: usize) -> Result<Form, IdError> {
  (n.to_string().len() <= width)
    .then_some(Form::Ordinal(n))
    .ok_or(IdError::OutOfRange)
}

/// The next ordinal after the highest of `taken`, or 1 for none.
fn next_ordinal(taken: impl Iterator<Item = Form>, width: usize) -> Result<Form, IdError> {
  let highest = taken
    .map(|form| match form {
      Form::Ordinal(n) => n,
    })
    .max()
    .unwrap_or(0);
  fits(highest.saturating_add(1), width)
}

impl Form {
  fn padded(&self, width: usize) -> String {
    match self {
      Form::Ordinal(n) => format!("{n:0width$}"),
    }
  }

  /// The digits with no padding, for the tagged spellings `s59` and `i59`.
  fn bare(&self) -> String {
    match self {
      Form::Ordinal(n) => n.to_string(),
    }
  }

  fn ordinal(&self) -> u32 {
    match self {
      Form::Ordinal(n) => *n,
    }
  }
}

impl ThreadId {
  fn of(form: Form) -> Self {
    Self {
      stored: format!("{THREAD_PREFIX}{}", form.padded(THREAD_DIGITS)),
      form,
    }
  }

  /// The thread a stored spelling names: `ST` and four digits, exactly.
  pub fn parse(stored: &str) -> Result<Self, IdError> {
    let digits = stored.strip_prefix(THREAD_PREFIX).ok_or(IdError::NotAnId)?;
    ordinal_digits(digits, THREAD_DIGITS).map(Self::of)
  }

  /// The thread with ordinal `n`, for a number a wire or legacy form carries.
  pub fn from_ordinal(n: u32) -> Result<Self, IdError> {
    fits(n, THREAD_DIGITS).map(Self::of)
  }

  /// The id after the highest of `taken`, `ST0001` for none.
  pub fn next_after<'a>(taken: impl IntoIterator<Item = &'a ThreadId>) -> Result<Self, IdError> {
    next_ordinal(taken.into_iter().map(|id| id.form), THREAD_DIGITS).map(Self::of)
  }

  /// The stored form, borrowed: canon, file and directory names, claims,
  /// `related`, `.intentfiles`, event subjects, URIs and `--json`.
  pub fn as_str(&self) -> &str {
    &self.stored
  }

  /// The stored form, owned.
  pub fn stored(&self) -> String {
    self.stored.clone()
  }

  /// The form a table, a view or a message prints.
  pub fn shown(&self) -> String {
    self.stored.clone()
  }

  /// The lowercase form a branch or a ref takes.
  pub fn ref_name(&self) -> String {
    self.stored.to_ascii_lowercase()
  }

  /// The explicitly tagged short spelling an operator can type: `s59`.
  pub fn tagged(&self) -> String {
    format!("{}{}", IdKind::Thread.tag(), self.form.bare())
  }
}

/// **A THREAD COMPARES WITH A STORED SPELLING WHOLE, AND WITH NOTHING ELSE.**
/// The facade's thread verbs take the stored spelling a door already
/// normalised, so a lookup is this comparison: equal exactly when the string
/// is this id's stored form. An operator's spelling (`46`, `s46`) is not a
/// stored one and compares unequal; it reaches here through
/// [`normalise_thread_id`] first.
impl PartialEq<str> for ThreadId {
  fn eq(&self, other: &str) -> bool {
    self.stored == other
  }
}

impl PartialEq<&str> for ThreadId {
  fn eq(&self, other: &&str) -> bool {
    self == *other
  }
}

impl PartialEq<String> for ThreadId {
  fn eq(&self, other: &String) -> bool {
    self == other.as_str()
  }
}

impl WpId {
  /// The work package a stored spelling names under its thread: two digits,
  /// exactly.
  pub fn parse(stored: &str) -> Result<Self, IdError> {
    ordinal_digits(stored, WP_DIGITS).map(Self)
  }

  /// The work package with ordinal `n`, for a number a wire form carries.
  pub fn from_ordinal(n: u32) -> Result<Self, IdError> {
    fits(n, WP_DIGITS).map(Self)
  }

  /// The id after the highest of `taken` in one thread, `01` for none.
  pub fn next_after(taken: impl IntoIterator<Item = WpId>) -> Result<Self, IdError> {
    next_ordinal(taken.into_iter().map(|id| id.0), WP_DIGITS).map(Self)
  }

  /// The stored form under its thread: `09`.
  pub fn stored(&self) -> String {
    self.0.padded(WP_DIGITS)
  }

  /// The form a table, a view or a message prints under its thread.
  pub fn shown(&self) -> String {
    self.stored()
  }

  /// The heading a view gives the package: `WP-09`.
  pub fn heading(&self) -> String {
    format!("{WP_HEADING_PREFIX}{}", self.shown())
  }

  /// Whether this is the criterion group `00`, which names the thread itself
  /// rather than one of its packages.
  pub fn is_thread_level(&self) -> bool {
    self.0 == Form::Ordinal(0)
  }
}

impl IssueId {
  /// The issue a stored spelling names: four digits, exactly.
  pub fn parse(stored: &str) -> Result<Self, IdError> {
    ordinal_digits(stored, ISSUE_DIGITS).map(Self)
  }

  /// The issue with ordinal `n`, for a number a wire or legacy form carries.
  pub fn from_ordinal(n: u32) -> Result<Self, IdError> {
    fits(n, ISSUE_DIGITS).map(Self)
  }

  /// The id after the highest of `taken`, `0001` for none.
  pub fn next_after(taken: impl IntoIterator<Item = IssueId>) -> Result<Self, IdError> {
    next_ordinal(taken.into_iter().map(|id| id.0), ISSUE_DIGITS).map(Self)
  }

  /// The stored form: `0703`.
  pub fn stored(&self) -> String {
    self.0.padded(ISSUE_DIGITS)
  }

  /// The form a table, a view or a message prints.
  pub fn shown(&self) -> String {
    self.stored()
  }

  /// How a board claims it and `.intentfiles` declares it: `ISSUE:0703`.
  pub fn claim(&self) -> String {
    format!("{CLAIM_ISSUE_PREFIX}{}", self.stored())
  }

  /// The explicitly tagged short spelling an operator can type: `i59`.
  pub fn tagged(&self) -> String {
    format!("{}{}", IdKind::Issue.tag(), self.0.bare())
  }
}

/// What separates the thread from the package in a work package's address.
const WP_SEPARATOR: char = '/';

/// What a package's heading puts before its id, and what an operator may type
/// before one: `WP-03`.
const WP_HEADING_PREFIX: &str = "WP-";

/// A work package's whole stored address: `ST0092/09`. Claims, event subjects,
/// `--json` and the `created:` line carry it.
pub fn wp_address(thread: &str, wp: WpId) -> String {
  format!("{thread}{WP_SEPARATOR}{}", wp.stored())
}

/// A work package's address as a table or a message prints it.
pub fn wp_address_shown(thread: &ThreadId, wp: WpId) -> String {
  format!("{}{WP_SEPARATOR}{}", thread.shown(), wp.shown())
}

/// A work package's address split into its thread's part and its package's,
/// both as written: the inverse of [`wp_address`], and the one reader of the
/// separator. Neither part is read here, because each caller reads them with
/// the reader its spelling calls for. `None` when the text names no package.
pub fn split_wp_address(text: &str) -> Option<(&str, &str)> {
  text.split_once(WP_SEPARATOR)
}

/// Is this a thing a board can claim: a steel thread, one of its work
/// packages, or an issue?
///
/// **ONE HOME FOR EVERY DOOR THAT ADMITS A CLAIM.** `wb claim` asks it of a
/// value typed at the verb and `wb migrate` of a value read off a board's
/// header. The issue form is the manifest's own spelling, `ISSUE:0512` (hv,
/// decision 29); the bare `0512` is refused, because two spellings of one
/// address is the drift Highlander names.
pub fn is_claim_address(claim: &str) -> bool {
  if let Some(issue) = claim.strip_prefix(CLAIM_ISSUE_PREFIX) {
    return IssueId::parse(issue).is_ok();
  }
  match split_wp_address(claim) {
    None => ThreadId::parse(claim).is_ok(),
    Some((thread, wp)) => ThreadId::parse(thread).is_ok() && WpId::parse(wp).is_ok(),
  }
}

// ---------------------------------------------------------------------------
// The operator door: what a person typed, before any store is asked.
// ---------------------------------------------------------------------------

/// An operator's spelling, decomposed.
struct Spelling {
  tag: Option<IdKind>,
  seq: u32,
}

/// Case-insensitive prefix strip. The CLI door is case-insensitive on purpose
/// and [`crate::intentfiles::Sigil::parse`] is case-SENSITIVE on purpose: a
/// manifest is committed and diffed, so one spelling keeps the diff about the
/// change, while a hand-typed argument is neither.
fn strip_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
  // `get` rather than an index: a spelling that is not ASCII can put the
  // prefix's length inside a character, where an index would panic.
  s.get(..prefix.len())
    .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
    .then(|| &s[prefix.len()..])
}

/// Decompose a spelling into the collection it names (if any) and its number.
///
/// **THE TAG ORDER IS LOAD-BEARING: `ST` IS TRIED BEFORE `s`.** Both name the
/// thread collection and `ST0059` starts with `S`, so a one-character test
/// first would take `T0059` as the digits and refuse a canonical id.
fn spelling(raw: &str) -> Result<Spelling, IdError> {
  let t = raw.trim();
  // `0021.json` and `0021.md` are the same issue as `0021`: an operator who
  // copied a padded id out of a filename must not be told it does not exist.
  let t = t
    .strip_suffix(".json")
    .or_else(|| t.strip_suffix(".md"))
    .unwrap_or(t);

  // The tag letters are [`IdKind::tag`]'s, read here rather than spelled again.
  let (thread_tag, issue_tag) = (
    IdKind::Thread.tag().to_string(),
    IdKind::Issue.tag().to_string(),
  );
  let (tag, digits) = if let Some(d) = strip_ci(t, THREAD_PREFIX) {
    (Some(IdKind::Thread), d)
  } else if let Some(d) = strip_ci(t, &thread_tag) {
    (Some(IdKind::Thread), d)
  } else if let Some(d) = strip_ci(t, &issue_tag) {
    (Some(IdKind::Issue), d)
  } else {
    (None, t)
  };

  Ok(Spelling {
    tag,
    seq: ordinal_of(digits)?,
  })
}

/// The number an operator's digits spell, whatever their padding.
///
/// Trimmed BEFORE parsing. v2's `normalise_st_id` carries `10#` because
/// `printf %04d` under /bin/bash 3.2 reads a leading zero as OCTAL: `0044`
/// became ST0036, a different real thread. Rust's `parse` is decimal by
/// construction, so the workaround is not ported; the property it protects is,
/// and it has a test. Trimming here is for the width check, not for base.
fn ordinal_of(digits: &str) -> Result<u32, IdError> {
  if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
    return Err(IdError::NotAnId);
  }
  let bare = digits.trim_start_matches('0');
  let bare = if bare.is_empty() { "0" } else { bare };
  bare.parse().map_err(|_| IdError::OutOfRange)
}

/// An operator's spelling of a THREAD id.
///
/// **THE ACCEPTED SET IS v2's, DRIVEN RATHER THAN READ** (`intent_helpers:688`):
/// `46`, `ST46`, `0046`, `046` and `ST0046` all name `ST0046`. `s46` and `S46`
/// are additions, an explicit tag for the one door that cannot know the
/// collection from its verb. v2's two holes are not ported: it glued `ST` onto
/// anything (`foo` became `STfoo`, then a not-found for a non-name), and it had
/// the width hole `fits` documents.
pub fn normalise_thread_id(raw: &str) -> Result<ThreadId, IdError> {
  let s = spelling(raw)?;
  match s.tag {
    Some(IdKind::Issue) => Err(IdError::WrongCollection {
      named: IdKind::Issue,
    }),
    _ => ThreadId::from_ordinal(s.seq),
  }
}

/// An operator's spelling of an ISSUE id: `21`, `0021`, `0021.json` and `i21`
/// are one issue, as v2's `bin/intent_issues:normalize_id` made them.
pub fn normalise_issue_id(raw: &str) -> Result<IssueId, IdError> {
  let s = spelling(raw)?;
  match s.tag {
    Some(IdKind::Thread) => Err(IdError::WrongCollection {
      named: IdKind::Thread,
    }),
    _ => IssueId::from_ordinal(s.seq),
  }
}

/// An operator's or a v2 file's spelling of a WORK PACKAGE id under its thread:
/// `3`, `03` and `WP-03` are one package, as the v2 importer and the CLI's
/// `<thread>/<wp>` doors have always read them.
pub fn normalise_wp_id(raw: &str) -> Result<WpId, IdError> {
  let t = raw.trim();
  let digits = strip_ci(t, WP_HEADING_PREFIX).unwrap_or(t).trim();
  WpId::from_ordinal(ordinal_of(digits)?)
}

/// What a spelling names at a door that knows the collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Named {
  Thread(ThreadId),
  Issue(IssueId),
}

/// An operator's spelling at a door that does NOT know which collection is
/// meant: `intent edit <address>`, where the argument may name either.
///
/// **UNTAGGED DIGITS ARE AMBIGUOUS HERE AND NOWHERE ELSE.** A verb-scoped door
/// reads the collection off its own verb; this one has nothing to read it off,
/// and `59` names both `ST0059` and issue `0059`. **This door is PURE and never
/// reads the store**, so for a bare number it reports the ambiguity with both
/// candidates, and [`crate::resolve`] asks the project which exists; the tagged
/// spellings `s59` and `i59` settle it in the argument itself.
pub fn normalise_id(raw: &str) -> Result<Named, IdError> {
  let s = spelling(raw)?;
  match s.tag {
    Some(IdKind::Thread) => ThreadId::from_ordinal(s.seq).map(Named::Thread),
    Some(IdKind::Issue) => IssueId::from_ordinal(s.seq).map(Named::Issue),
    None => Err(IdError::Ambiguous {
      thread: ThreadId::from_ordinal(s.seq)?,
      issue: IssueId::from_ordinal(s.seq)?,
    }),
  }
}

/// What an operator who typed `raw` would type to name the `wanted` collection's
/// id with the same number: `i21` asked of a thread door is `s21`. The remedy a
/// [`IdError::WrongCollection`] refusal offers. `None` when `raw` spells no id,
/// or one past the wanted kind's width.
pub fn respelled_as(raw: &str, wanted: IdKind) -> Option<String> {
  let s = spelling(raw).ok()?;
  match wanted {
    IdKind::Thread => ThreadId::from_ordinal(s.seq).ok().map(|id| id.tagged()),
    IdKind::Issue => IssueId::from_ordinal(s.seq).ok().map(|id| id.tagged()),
  }
}

// ---------------------------------------------------------------------------
// The one scanner: which ids a text names.
// ---------------------------------------------------------------------------

/// Every steel thread `text` names, in the order they first appear, each once.
///
/// **ONE SCANNER WHERE THERE WERE TWO** (`legacy.rs` read v2 prose with one,
/// `contract.rs` read cited lines with another; the same window rule, written
/// twice). An id here is `ST` and four digits wherever they sit, with no
/// boundary required either side, which is what both did.
pub fn thread_ids_in(text: &str) -> Vec<ThreadId> {
  let bytes = text.as_bytes();
  let width = THREAD_PREFIX.len() + THREAD_DIGITS;
  let mut out: Vec<ThreadId> = Vec::new();
  for start in 0..bytes.len().saturating_sub(width - 1) {
    let Some(window) = text.get(start..start + width) else {
      continue;
    };
    if let Ok(id) = ThreadId::parse(window)
      && !out.contains(&id)
    {
      out.push(id);
    }
  }
  out
}

/// Whether `line` carries `token` (a criterion or test id such as `AT-3.5`) as
/// a whole token (issue 0670): an occurrence counts only where the character
/// after it is not a digit, so `AT-3.5` is not found inside `AT-3.50`. The
/// census at `refs/bank/vc/0670/` measured that this reddens no row.
pub fn carries_token(line: &str, token: &str) -> bool {
  line
    .match_indices(token)
    .any(|(i, _)| !line[i + token.len()..].starts_with(|c: char| c.is_ascii_digit()))
}

// ---------------------------------------------------------------------------
// Criterion and test labels: the group between the dash and the dot.
// ---------------------------------------------------------------------------

/// The group a criterion or test label carries for the thread itself.
pub const THREAD_GROUP: &str = "00";

/// What a criterion or test label's group names: `AC-03.1` and `AC-3.1` both
/// name package `03`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
  /// The group [`THREAD_GROUP`]: the thread itself.
  Thread,
  /// One of the thread's work packages.
  Package(WpId),
  /// A group that is neither, eg `AC-banana`.
  Unreadable,
}

/// The text between a label's dash and its dot, or the whole label when it
/// has neither. Only a renderer that must show an unreadable group asks for
/// the text; every comparison asks [`group_of`].
pub fn group_text(label: &str) -> &str {
  label
    .split_once('-')
    .and_then(|(_, rest)| rest.split_once('.'))
    .map_or(label, |(group, _)| group)
}

/// **THE ONE READER OF A LABEL'S GROUP** (issue 0712). There were five, and
/// the string ones compared `1` with `01`, so a package whose criteria were
/// written `AC-1.x` had none in scope for its gate, which then passed it
/// unjudged. The group is read as a package id, so both spellings name the
/// same package everywhere a group is compared.
pub fn group_of(label: &str) -> Group {
  if label.split_once('-').is_none() {
    return Group::Unreadable;
  }
  match normalise_wp_id(group_text(label)) {
    Ok(wp) if wp.is_thread_level() => Group::Thread,
    Ok(wp) => Group::Package(wp),
    Err(_) => Group::Unreadable,
  }
}

/// The one order for a thread's criterion and test ids (issue 0643): natural,
/// not lexical. An id compares run by run, a run of digits as a number and any
/// other run as text, so `AT-01.2` precedes `AT-01.10` and `AC-1.1` precedes
/// `AC-10.1`. Ids that differ only in leading zeros (`AT-1.2`, `AT-01.2`) fall
/// back to their bytes, so the order is total.
pub fn id_order(a: &str, b: &str) -> Ordering {
  fn runs(s: &str) -> impl Iterator<Item = &str> {
    let mut rest = s;
    std::iter::from_fn(move || {
      let first = rest.chars().next()?;
      let digits = first.is_ascii_digit();
      let end = rest
        .find(|c: char| c.is_ascii_digit() != digits)
        .unwrap_or(rest.len());
      let (run, tail) = rest.split_at(end);
      rest = tail;
      Some(run)
    })
  }
  let number = |run: &str| run.trim_start_matches('0').to_string();
  let mut left = runs(a);
  let mut right = runs(b);
  loop {
    match (left.next(), right.next()) {
      (None, None) => return a.cmp(b),
      (None, Some(_)) => return Ordering::Less,
      (Some(_), None) => return Ordering::Greater,
      (Some(x), Some(y)) => {
        let both_digits = x.starts_with(|c: char| c.is_ascii_digit())
          && y.starts_with(|c: char| c.is_ascii_digit());
        let order = if both_digits {
          let (x, y) = (number(x), number(y));
          x.len().cmp(&y.len()).then_with(|| x.cmp(&y))
        } else {
          x.cmp(y)
        };
        if order.is_ne() {
          return order;
        }
      }
    }
  }
}

// ---------------------------------------------------------------------------
// The faces: canon (serde), the published JSON Schema, the store and GraphQL.
//
// **EACH ENCODES THE REPRESENTATION ABOVE, SO EACH LIVES HERE.** Today a thread
// id travels as its stored string, and a work package or issue id as the bare
// ordinal number its canon field has always held (`"seq": 9`, `"number": 703`),
// so every face is byte-for-byte what it was before these types existed.
// ---------------------------------------------------------------------------

impl serde::Serialize for ThreadId {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&self.stored)
  }
}

impl<'de> serde::Deserialize<'de> for ThreadId {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    let stored = String::deserialize(deserializer)?;
    ThreadId::parse(&stored)
      .map_err(|e| serde::de::Error::custom(format!("`{stored}` is not a steel thread id: {e}")))
  }
}

macro_rules! ordinal_faces {
  ($ty:ident, $what:literal) => {
    impl serde::Serialize for $ty {
      fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(self.0.ordinal())
      }
    }

    impl<'de> serde::Deserialize<'de> for $ty {
      fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let n = u32::deserialize(deserializer)?;
        $ty::from_ordinal(n)
          .map_err(|e| serde::de::Error::custom(format!("{n} is not {}: {e}", $what)))
      }
    }

    impl schemars::JsonSchema for $ty {
      fn inline_schema() -> bool {
        true
      }

      fn schema_name() -> Cow<'static, str> {
        <u32 as schemars::JsonSchema>::schema_name()
      }

      fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <u32 as schemars::JsonSchema>::json_schema(generator)
      }
    }

    impl rusqlite::types::ToSql for $ty {
      fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(i64::from(self.0.ordinal())))
      }
    }

    impl rusqlite::types::FromSql for $ty {
      fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let n = value.as_i64()?;
        u32::try_from(n)
          .map_err(|_| rusqlite::types::FromSqlError::OutOfRange(n))
          .and_then(|n| {
            $ty::from_ordinal(n).map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
          })
      }
    }

    impl async_graphql::OutputType for $ty {
      fn type_name() -> Cow<'static, str> {
        <u32 as async_graphql::OutputType>::type_name()
      }

      fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
        <u32 as async_graphql::OutputType>::create_type_info(registry)
      }

      fn resolve(
        &self,
        ctx: &async_graphql::ContextSelectionSet<'_>,
        field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
      ) -> impl std::future::Future<Output = async_graphql::ServerResult<async_graphql::Value>> + Send
      {
        let n = self.0.ordinal();
        async move { <u32 as async_graphql::OutputType>::resolve(&n, ctx, field).await }
      }
    }
  };
}

ordinal_faces!(WpId, "a work package id");
ordinal_faces!(IssueId, "an issue id");

impl schemars::JsonSchema for ThreadId {
  fn inline_schema() -> bool {
    true
  }

  fn schema_name() -> Cow<'static, str> {
    <String as schemars::JsonSchema>::schema_name()
  }

  fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    <String as schemars::JsonSchema>::json_schema(generator)
  }
}

impl rusqlite::types::ToSql for ThreadId {
  fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
    Ok(rusqlite::types::ToSqlOutput::from(self.stored.as_str()))
  }
}

impl rusqlite::types::FromSql for ThreadId {
  fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
    let stored = value.as_str()?;
    ThreadId::parse(stored).map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
  }
}

impl async_graphql::OutputType for ThreadId {
  fn type_name() -> Cow<'static, str> {
    <String as async_graphql::OutputType>::type_name()
  }

  fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
    <String as async_graphql::OutputType>::create_type_info(registry)
  }

  fn resolve(
    &self,
    ctx: &async_graphql::ContextSelectionSet<'_>,
    field: &async_graphql::Positioned<async_graphql::parser::types::Field>,
  ) -> impl std::future::Future<Output = async_graphql::ServerResult<async_graphql::Value>> + Send
  {
    let stored = self.stored.clone();
    async move { <String as async_graphql::OutputType>::resolve(&stored, ctx, field).await }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_claim_prefix_is_the_manifests_own_spelling() {
    // **THE TEST THAT MAKES THE COPY LEGITIMATE.** `CLAIM_ISSUE_PREFIX` is
    // declared here rather than read from `intentfiles`, because `intentfiles`
    // depends on this module and reaching back would invert the layering. A
    // copy a test holds identical cannot drift; a copy held by discipline does,
    // and then two doors disagree about how an issue is named.
    assert_eq!(
      CLAIM_ISSUE_PREFIX,
      format!("{}:", crate::intentfiles::Sigil::Issue.as_str()),
      "the claim prefix and the manifest sigil are one fact"
    );
  }

  #[test]
  fn every_form_the_refusal_names_is_one_the_validator_admits() {
    // The examples are read out of the sentence, never retyped here, so the
    // next edit to it stays under this assertion.
    let forms: Vec<&str> = CLAIM_ADDRESS_FORMS.split('`').skip(1).step_by(2).collect();
    // A new kind lands in `is_claim_address`, in `CLAIM_ADDRESS_FORMS` and in
    // this list TOGETHER; nothing here notices a validator widened alone.
    assert_eq!(
      forms,
      ["ST0000", "ST0000/01", "ISSUE:0000"],
      "one example per kind the validator admits: thread, work package, issue"
    );
    for form in forms {
      assert!(
        is_claim_address(form),
        "the refusal tells the user to type `{form}` and the validator refuses it"
      );
    }
  }

  #[test]
  fn a_board_claims_an_issue_by_the_manifest_form_and_by_nothing_else() {
    let issue = |n| IssueId::from_ordinal(n).expect("an ordinal that fits");
    assert!(is_claim_address("ISSUE:0512"), "hv's decision 29 form");
    assert_eq!(issue(512).claim(), "ISSUE:0512");
    assert!(is_claim_address(&issue(1).claim()), "the formatter agrees");

    // **REFUSED, EACH FOR ITS OWN STATED REASON**, so a later widening has to
    // argue with the reason rather than notice a gap.
    assert!(
      !is_claim_address("0512"),
      "the bare form: two spellings of one address is the drift Highlander names"
    );
    assert!(
      !is_claim_address("the pre-commit gate"),
      "free text: two claims could then be compared only by string equality"
    );
    assert!(
      !is_claim_address("ISSUE:512"),
      "four digits, derived from ISSUE_DIGITS"
    );
    assert!(!is_claim_address("ISSUE:00512"), "five is not four");
    assert!(!is_claim_address("ISSUE:"), "a prefix is not an address");
    assert!(!is_claim_address("ISSUE:abcd"), "digits, not a width alone");
    assert!(
      !is_claim_address("issue:0512"),
      "the manifest is case-SENSITIVE"
    );

    // The thread forms are untouched by the widening, which is the half a
    // change like this breaks silently.
    assert!(is_claim_address("ST0079"));
    assert!(is_claim_address("ST0079/01"));
    assert!(!is_claim_address("ST0079/1"));
    assert!(!is_claim_address("ST0079/WP-01"));
  }

  #[test]
  fn a_spelling_that_is_not_ascii_is_refused_and_never_panics() {
    // `strip_ci` once indexed at a prefix's length, which can land inside a
    // character: `st show` of an accented letter, a criterion labelled with two
    // of them, and an address ending in them each panicked instead of being
    // refused. `\u{e9}` is two bytes in UTF-8, so a one-byte prefix ends inside it.
    assert_eq!(normalise_thread_id("\u{e9}"), Err(IdError::NotAnId));
    assert_eq!(normalise_issue_id("\u{e9}"), Err(IdError::NotAnId));
    assert_eq!(normalise_wp_id("\u{e9}\u{e9}"), Err(IdError::NotAnId));
    assert_eq!(group_of("AC-\u{e9}\u{e9}.1"), Group::Unreadable);
  }

  #[test]
  fn the_thread_group_is_the_package_form_of_zero() {
    // **THE TEST THAT MAKES THE COPY LEGITIMATE**, as for the claim prefix: a
    // label's group is compared against `THREAD_GROUP`, and it is the package
    // form of ordinal zero, so the constant and the form cannot drift apart.
    let zero = WpId::from_ordinal(0).expect("zero fits a package's width");
    assert_eq!(THREAD_GROUP, zero.stored());
    assert!(zero.is_thread_level());
  }
}
