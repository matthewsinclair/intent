//! **ONE MODULE OWNS A THREAD, WORK PACKAGE AND ISSUE ID, END TO END** (ST0095,
//! hv decision 62: "highlander both the minting of these new ids and any checks
//! on them into as tight a waist as possible").
//!
//! An id is a typed value, [`ThreadId`], [`WpId`] or [`IssueId`], and only this
//! module makes one: from its stored form ([`ThreadId::parse`] and its
//! siblings), from what an operator typed ([`normalise_thread_id`] and its
//! siblings), from a number a wire form carries ([`IssueId::from_ordinal`]), or
//! new, from its creation event, by [`mint`], the one mint. Nothing outside it formats,
//! parses, scans or does arithmetic on one. **AN ID HAS NO `Display`, NO NUMERIC
//! ACCESSOR AND NO WIDTH**, so a `{:04}` written against one does not compile,
//! and the compiler lists every site that would do arithmetic on an id or pad
//! it by hand. **THAT GUARD STOPS AT TEXT.** A thread id reads as its stored
//! `&str` ([`ThreadId::as_str`], and `Deref`), so a site that already holds the
//! text can still compose an address from it with a `format!`, and only a
//! review finds that: build a package's address with [`wp_address`] and split
//! one with [`split_wp_address`].
//!
//! **`Deref<Target = str>` ON [`ThreadId`] STAYS** (cc's architecture call on the
//! WP-04 critic's R7, ST0095 WP-07). The stored form is the only string form any
//! store, claim, path or subject compares, and `Deref` gives exactly that form
//! and nothing else. The defects R7 feared are a prefix or a shown form compared
//! as if it were stored, and both are closed elsewhere: at the door, where
//! [`resolve_thread`] and its siblings turn every typed spelling into a whole
//! typed id before anything compares, and in messages, which take
//! [`ThreadId::shown`]. Removing `Deref` would put `as_str()` at every store call
//! for no change in meaning.
//!
//! **A PREFIX IS RESOLVED, NEVER STORED** (ST0095 WP-07). What a person types is
//! read with no store by [`read_thread`], [`read_issue`] and [`read_wp`]: a whole
//! id, or the front of a hashed one, at least [`MIN_PREFIX_HEX`] hex digits. The
//! resolvers complete a front against an [`IdScope`], the store injected, and
//! answer one id, several or none; nothing stores what was typed.
//!
//! **AN ID IS AN ORDINAL OR A HASH**: `ST0092`, issue `0703` and a work
//! package's `09` under its thread, minted by every Intent before ST0095 and
//! valid for ever, or the front of the SHA-256 of the record's creation event's
//! id, which is all [`mint`] makes. Each type holds one private `Form`, so a
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

/// The marker before a hashed issue id's hex (ic's letter, ST0095): `I3f9a2c1d4e5f6a7b`.
/// A hashed thread id carries [`THREAD_PREFIX`]; a work package carries none,
/// because it never stands alone.
pub const ISSUE_MARKER: &str = "I";
/// How many hex digits a hashed thread or issue id stores (hv decision 64:
/// "Store 16, show 7"), cut from the front of its creation event's SHA-256.
pub const HASH_HEX: usize = 16;
/// How many hex digits a hashed work package id stores under its thread.
pub const WP_HASH_HEX: usize = 8;
/// How many of a hashed thread or issue id's hex digits a table, a view or a
/// message shows.
pub const SHOWN_HEX: usize = 7;
/// How many of a hashed work package id's hex digits are shown.
pub const WP_SHOWN_HEX: usize = 4;
/// The fewest hex digits a typed prefix of a hashed id carries (ST0095 WP-07;
/// design, "Typed"). A prefix made of digits alone needs one more than its
/// kind's ordinal width as well, so that no spelling is both an ordinal and a
/// prefix ([`prefix_floor`]). Every shown form is at least this long, so a
/// short form copied out of a table is always a prefix a door takes.
pub const MIN_PREFIX_HEX: usize = 4;

// Every shown form is a prefix a door takes, held where the constants are, so
// a change to any of them that breaks it does not compile.
const _: () = assert!(SHOWN_HEX >= MIN_PREFIX_HEX && WP_SHOWN_HEX >= MIN_PREFIX_HEX);

/// How a claim names an ISSUE, as [`crate::intentfiles::Sigil::Issue`] spells
/// it in the manifest, with its separator.
///
/// Declared here rather than read from `intentfiles`, which depends on this
/// module (`Sigil::accepts` asks [`IssueId::parse`]); reaching the other way
/// would invert that, and `the_claim_prefix_is_the_manifests_own_spelling`
/// holds the two identical.
pub const CLAIM_ISSUE_PREFIX: &str = "ISSUE:";

/// What separates a sigil from its id, in a claim and in a manifest line:
/// the `:` of `ISSUE:0703` and `STEELTHREAD:ST0092`.
pub const SIGIL_SEPARATOR: char = ':';

/// The worked example's hex, as a thread or an issue shows it: the front of
/// [`EXAMPLE_HEX`]. A macro, so each form sentence below is built from it.
macro_rules! example_shown {
  () => {
    "3f9a2c1"
  };
}

/// The worked example's package hex, as a package shows it: the front of
/// [`EXAMPLE_WP_HEX`].
macro_rules! example_wp_shown {
  () => {
    "c2e4"
  };
}

/// The worked example's whole stored hex, as a macro so a form sentence can
/// name it whole: [`EXAMPLE_HEX`].
macro_rules! example_hex {
  () => {
    concat!(example_shown!(), "d4e5f6a7b")
  };
}

/// The worked example's whole stored package hex: [`EXAMPLE_WP_HEX`].
macro_rules! example_wp_hex {
  () => {
    concat!(example_wp_shown!(), "1a0b")
  };
}

/// **THE ONE WORKED EXAMPLE OF A HASHED ID** that a form sentence, a doc or a
/// schema comment spells: a thread's or an issue's whole stored hex. It names
/// no record, so a scanner for ids in shipped text exempts it through
/// [`is_example`] (issue 0736).
pub const EXAMPLE_HEX: &str = example_hex!();

/// The worked example of a hashed package's whole stored hex, as
/// [`EXAMPLE_HEX`] is a thread's or an issue's.
pub const EXAMPLE_WP_HEX: &str = example_wp_hex!();

/// The forms [`is_claim_address`] admits, as the sentence a refusal tells the
/// user to type.
///
/// **ONE HOME FOR WHAT EVERY DOOR SAYS, BESIDE THE ONE HOME FOR WHAT IT
/// ADMITS** (0519). `every_form_the_refusal_names_is_one_the_validator_admits`
/// holds each example here to the validator. The reverse is not mechanical: a
/// widening that teaches the validator another kind fails nothing until this
/// sentence and that test's list change with it.
///
/// **IT NAMES BOTH FAMILIES** (ST1c2db93 P4): each kind's ordinal and its
/// git-style id whole, as a board stores a claim, built from the worked
/// example.
pub const CLAIM_ADDRESS_FORMS: &str = concat!(
  "a thread as `ST0000` or `ST",
  example_hex!(),
  "`, a work package as `ST0000/01` or `ST",
  example_hex!(),
  "/",
  example_wp_hex!(),
  "`, or an issue as `ISSUE:0000` or `ISSUE:I",
  example_hex!(),
  "`"
);

/// The forms a door that takes a thread or one of its packages admits (`todo
/// done`, `fc`, `ac gate`), as the sentence its refusal quotes: each kind in
/// both families, the git-style one as a table shows it, which the door
/// completes (ST1c2db93 P4).
pub const TARGET_FORMS: &str = concat!(
  "a thread as `ST0000` or `ST",
  example_shown!(),
  "`, or a work package as `ST0000/02` or `ST",
  example_shown!(),
  "/",
  example_wp_shown!(),
  "`"
);

/// **THE ONE SENTENCE OF A THREAD ID'S FORM** that every refusal quotes (the
/// Highlander sweep's C13: three refusals each said it in their own words).
pub const THREAD_ID_FORM: &str = concat!(
  "a steel-thread id is ST followed by four digits, eg ST0000, ",
  "or a hashed id or a unique prefix of its hex, eg ST",
  example_shown!()
);

/// The one sentence of an issue id's form, as [`THREAD_ID_FORM`] is a thread's.
pub const ISSUE_ID_FORM: &str = concat!(
  "an issue id is four digits, eg 0001, or a hashed id or a unique prefix of its hex, eg I",
  example_shown!()
);

/// The one sentence of a work package's address, as [`THREAD_ID_FORM`] is a
/// thread id's (cc's meaning, 2026-11-20): its thread's id, a slash and its own
/// id, in either family, each part whole or a unique prefix of its hex. The
/// stored address carries both ids whole.
pub const WP_ADDRESS_FORM: &str = concat!(
  "a work package's address is its thread's id, a slash and its own id, ",
  "eg ST0000/01 or ST",
  example_shown!(),
  "/",
  example_wp_shown!()
);

/// The tag an operator can type before a thread's digits: `s59` is `ST0059`.
const THREAD_TAG: char = 's';

/// The tag an operator can type before an issue's digits: `i59` is `0059`.
const ISSUE_TAG: char = 'i';

/// **THE ONE ENUM OF THE ID FAMILIES** (ST0095 WP-14): a steel thread, a work
/// package, an issue. Nothing else names them as a set. An enum that also names
/// kinds that are not ids (`search::HitKind`, `address::Entity`) keeps its own
/// variants and maps its id variants from this one.
///
/// **IT OWNS ONE SPELLING PER SURFACE, AND EVERY SURFACE ASKS IT.** A work
/// package was spelled `wp`, `work-package`, `work_package` and `work package`
/// by eight enums and string sets, and search read one spelling where ingest
/// wrote another (issue 0708). Each surface's spelling is one method here, and
/// each reader parses through that method's inverse, which accepts every
/// spelling the surface ever wrote. What is persisted is never respelled: an
/// event file and the store keep the spelling they were written with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdKind {
  Thread,
  WorkPackage,
  Issue,
}

impl IdKind {
  /// Every family, in the order a count names them: threads, their work
  /// packages, then issues.
  pub const ALL: [IdKind; 3] = [IdKind::Thread, IdKind::WorkPackage, IdKind::Issue];

  /// The word prose uses, in a message or a plan's line. Not a wire form.
  pub const fn as_str(&self) -> &'static str {
    match self {
      IdKind::Thread => "steel thread",
      IdKind::WorkPackage => "work package",
      IdKind::Issue => "issue",
    }
  }

  /// The noun WITH its article. A message that assembles the article at the
  /// use site says "a issue" the first time somebody adds a vowel, which is
  /// what the first driven run of the operator door did.
  pub fn with_article(&self) -> &'static str {
    match self {
      IdKind::Thread => "a steel thread",
      IdKind::WorkPackage => "a work package",
      IdKind::Issue => "an issue",
    }
  }

  /// The tag an operator can type to name this family explicitly. `None` for
  /// a work package, which is named under its thread (`ST0000/01`) and has no
  /// tag of its own.
  pub fn tag(&self) -> Option<char> {
    match self {
      IdKind::Thread => Some(THREAD_TAG),
      IdKind::WorkPackage => None,
      IdKind::Issue => Some(ISSUE_TAG),
    }
  }

  /// How many digits this family's ordinal id carries.
  pub fn digits(&self) -> usize {
    match self {
      IdKind::Thread => THREAD_DIGITS,
      IdKind::WorkPackage => WP_DIGITS,
      IdKind::Issue => ISSUE_DIGITS,
    }
  }

  /// The sentence that says what this family's id looks like:
  /// [`THREAD_ID_FORM`], [`WP_ADDRESS_FORM`] or [`ISSUE_ID_FORM`].
  pub fn form(&self) -> &'static str {
    match self {
      IdKind::Thread => THREAD_ID_FORM,
      IdKind::WorkPackage => WP_ADDRESS_FORM,
      IdKind::Issue => ISSUE_ID_FORM,
    }
  }

  /// **THE WIRE SPELLING**: `thread`, `wp`, `issue`. An event's subject type,
  /// persisted in event files and the event log; search's `--kind` and a
  /// hit's `kind`; and an address's entity form, which `forms.json` names.
  pub fn wire(&self) -> &'static str {
    match self {
      IdKind::Thread => "thread",
      IdKind::WorkPackage => "wp",
      IdKind::Issue => "issue",
    }
  }

  /// The family a wire spelling names: the inverse of [`IdKind::wire`], whose
  /// spellings are the only ones that surface ever wrote. `None` for any other
  /// word, which may name a kind that is not an id (`file`, `node`).
  pub fn of_wire(word: &str) -> Option<IdKind> {
    IdKind::ALL.into_iter().find(|kind| kind.wire() == word)
  }

  /// **THE ROW SPELLING**: `thread`, `work-package`, `issue`. A prose
  /// section's owner in the store, which ingest writes and search reads, and
  /// the `kind` of an `outstanding` row or a `todo` item in their JSON, which
  /// this type's `Serialize` writes.
  pub fn row(&self) -> &'static str {
    match self {
      IdKind::Thread => "thread",
      IdKind::WorkPackage => "work-package",
      IdKind::Issue => "issue",
    }
  }

  /// The family a row spelling names: the inverse of [`IdKind::row`]. Ingest
  /// has written no other spelling into the store, so a section with any other
  /// owner (`file`, `board`, `wb_node`) is owned by no thread, package or issue.
  pub fn of_row(word: &str) -> Option<IdKind> {
    IdKind::ALL.into_iter().find(|kind| kind.row() == word)
  }

  /// The type column of `intent outstanding`, as hv wrote it: ST, WP, Issue.
  pub fn label(&self) -> &'static str {
    match self {
      IdKind::Thread => "ST",
      IdKind::WorkPackage => "WP",
      IdKind::Issue => "Issue",
    }
  }

  /// What a count calls this family: threads, work packages, issues.
  pub fn plural(&self) -> &'static str {
    match self {
      IdKind::Thread => "threads",
      IdKind::WorkPackage => "work packages",
      IdKind::Issue => "issues",
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
  /// Hex that is no ordinal and fewer digits than a prefix of a hashed id
  /// carries: `floor` is the fewest it needs ([`prefix_floor`]).
  TooShort { floor: usize },
  /// An address where an id was wanted (issue Ic7d939d): the readers say so,
  /// so every door words it as an address and none decides it on its own.
  Address,
}

impl fmt::Display for IdError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      IdError::NotAnId => f.write_str("not an id"),
      IdError::Address => f.write_str("an address, not an id"),
      IdError::OutOfRange => f.write_str("more digits than an id of its kind carries"),
      IdError::TooShort { floor } => write!(
        f,
        "fewer hex digits than a prefix of a hashed id carries: at least {floor}"
      ),
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
/// else. The derived order is the family key: every ordinal, by its number,
/// before every hashed id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Form {
  /// Minted highest-plus-one by every Intent before ST0095, and valid for ever.
  Ordinal(u32),
  /// Minted from a creation event's id (ST0095): the front of its SHA-256 in
  /// lowercase hex, as many digits as the kind stores ([`HASH_HEX`] or
  /// [`WP_HASH_HEX`]), held as their value so an id stays `Copy`.
  Hashed(u64),
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

/// A hashed id's exact stored hex: `width` lowercase hex digits, nothing else.
/// Uppercase is refused here, because a stored form is compared whole; an
/// operator's spelling is lowercased by the door before it gets this far.
fn hashed_hex(hex: &str, width: usize) -> Result<Form, IdError> {
  if hex.len() != width
    || !hex
      .bytes()
      .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
  {
    return Err(IdError::NotAnId);
  }
  Ok(Form::Hashed(
    hex.bytes().fold(0, |value, b| (value << 4) | nibble(b)),
  ))
}

/// One hex digit's value.
fn nibble(b: u8) -> u64 {
  char::from(b).to_digit(16).map_or(0, u64::from)
}

/// The form a creation event's id mints: the front `width` hex digits of its
/// SHA-256, through the crate's one hasher ([`crate::model::sha256_hex`]), so
/// there is no second hasher and no second source of randomness.
fn digest_front(origin: &str, width: usize) -> Form {
  Form::Hashed(
    crate::model::sha256_hex(origin.as_bytes())
      .bytes()
      .take(width)
      .fold(0, |value, b| (value << 4) | nibble(b)),
  )
}

impl Form {
  /// The stored spelling of the id's digits: an ordinal zero-padded to
  /// `ordinal_width`, a hash as `hash_width` lowercase hex digits.
  fn spelled(&self, ordinal_width: usize, hash_width: usize) -> String {
    match self {
      Form::Ordinal(n) => format!("{n:0ordinal_width$}"),
      Form::Hashed(value) => format!("{value:0hash_width$x}"),
    }
  }

  /// The digits a table shows: an ordinal whole, a hash's first `shown_width`.
  fn shown(&self, ordinal_width: usize, hash_width: usize, shown_width: usize) -> String {
    match self {
      Form::Ordinal(_) => self.spelled(ordinal_width, hash_width),
      Form::Hashed(_) => self
        .spelled(ordinal_width, hash_width)
        .chars()
        .take(shown_width)
        .collect(),
    }
  }

  /// The digits with no padding, for the tagged spellings `s59` and `i59`. A
  /// hash has none: its marker already names its kind.
  fn bare(&self) -> Option<String> {
    match self {
      Form::Ordinal(n) => Some(n.to_string()),
      Form::Hashed(_) => None,
    }
  }

  /// The ordinal's number, or `None` for a hash.
  fn ordinal(&self) -> Option<u32> {
    match self {
      Form::Ordinal(n) => Some(*n),
      Form::Hashed(_) => None,
    }
  }

  fn is_hashed(&self) -> bool {
    matches!(self, Form::Hashed(_))
  }

  /// Whether this is a hashed id whose stored hex, `hash_width` digits, begins
  /// with `prefix`. No ordinal begins with a prefix: a prefix names a hash.
  fn hex_begins(&self, hash_width: usize, prefix: &str) -> bool {
    self.is_hashed() && self.spelled(0, hash_width).starts_with(prefix)
  }
}

impl ThreadId {
  fn of(form: Form) -> Self {
    Self {
      stored: format!("{THREAD_PREFIX}{}", form.spelled(THREAD_DIGITS, HASH_HEX)),
      form,
    }
  }

  /// The thread a stored spelling names: `ST` and four digits, or `ST` and
  /// sixteen lowercase hex digits, exactly.
  pub fn parse(stored: &str) -> Result<Self, IdError> {
    let rest = stored.strip_prefix(THREAD_PREFIX).ok_or(IdError::NotAnId)?;
    ordinal_digits(rest, THREAD_DIGITS)
      .or_else(|_| hashed_hex(rest, HASH_HEX))
      .map(Self::of)
  }

  /// The thread id a creation event's id mints (ST0095): `ST` and the front of
  /// the SHA-256 of `origin`.
  pub fn from_origin(origin: &str) -> Self {
    Self::of(digest_front(origin, HASH_HEX))
  }

  /// Whether this is a hashed id minted from `origin`: [`verify`], the one
  /// check, which doctor asks. An ordinal has no origin to answer to.
  pub fn derives_from(&self, origin: &str) -> bool {
    verify(self, Some(origin))
  }

  /// Whether this id is a hash rather than an ordinal.
  pub fn is_hashed(&self) -> bool {
    self.form.is_hashed()
  }

  /// The thread with ordinal `n`, for a number a wire or legacy form carries.
  pub fn from_ordinal(n: u32) -> Result<Self, IdError> {
    fits(n, THREAD_DIGITS).map(Self::of)
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

  /// The form a table, a view or a message prints: the whole ordinal, or `ST`
  /// and the first [`SHOWN_HEX`] hex digits of a hash.
  pub fn shown(&self) -> String {
    format!(
      "{THREAD_PREFIX}{}",
      self.form.shown(THREAD_DIGITS, HASH_HEX, SHOWN_HEX)
    )
  }

  /// The lowercase form a branch or a ref takes.
  pub fn ref_name(&self) -> String {
    self.stored.to_ascii_lowercase()
  }

  /// The explicitly tagged short spelling an operator can type: `s59`. A hash
  /// is tagged by its marker, so its tagged spelling is its shown form.
  pub fn tagged(&self) -> String {
    self
      .form
      .bare()
      .map_or_else(|| self.shown(), |digits| format!("{THREAD_TAG}{digits}"))
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
  /// The work package a stored spelling names under its thread: two digits, or
  /// eight lowercase hex digits, exactly.
  pub fn parse(stored: &str) -> Result<Self, IdError> {
    ordinal_digits(stored, WP_DIGITS)
      .or_else(|_| hashed_hex(stored, WP_HASH_HEX))
      .map(Self)
  }

  /// The work package with ordinal `n`, for a number a wire form carries.
  pub fn from_ordinal(n: u32) -> Result<Self, IdError> {
    fits(n, WP_DIGITS).map(Self)
  }

  /// The work package id a creation event's id mints (ST0095): the front of
  /// the SHA-256 of `origin`, with no marker, because it never stands alone.
  pub fn from_origin(origin: &str) -> Self {
    Self(digest_front(origin, WP_HASH_HEX))
  }

  /// Whether this is a hashed id minted from `origin`.
  pub fn derives_from(&self, origin: &str) -> bool {
    verify(self, Some(origin))
  }

  /// Whether this id is a hash rather than an ordinal.
  pub fn is_hashed(&self) -> bool {
    self.0.is_hashed()
  }

  /// The stored form under its thread: `09`, or eight hex digits.
  pub fn stored(&self) -> String {
    self.0.spelled(WP_DIGITS, WP_HASH_HEX)
  }

  /// The form a table, a view or a message prints under its thread: the whole
  /// ordinal, or a hash's first [`WP_SHOWN_HEX`] hex digits.
  pub fn shown(&self) -> String {
    self.0.shown(WP_DIGITS, WP_HASH_HEX, WP_SHOWN_HEX)
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
  /// The issue a stored spelling names: four digits, or [`ISSUE_MARKER`] and
  /// sixteen lowercase hex digits, exactly.
  pub fn parse(stored: &str) -> Result<Self, IdError> {
    ordinal_digits(stored, ISSUE_DIGITS)
      .or_else(|_| {
        stored
          .strip_prefix(ISSUE_MARKER)
          .ok_or(IdError::NotAnId)
          .and_then(|hex| hashed_hex(hex, HASH_HEX))
      })
      .map(Self)
  }

  /// The issue with ordinal `n`, for a number a wire or legacy form carries.
  pub fn from_ordinal(n: u32) -> Result<Self, IdError> {
    fits(n, ISSUE_DIGITS).map(Self)
  }

  /// The issue id a creation event's id mints (ST0095): [`ISSUE_MARKER`] and
  /// the front of the SHA-256 of `origin`.
  pub fn from_origin(origin: &str) -> Self {
    Self(digest_front(origin, HASH_HEX))
  }

  /// Whether this is a hashed id minted from `origin`.
  pub fn derives_from(&self, origin: &str) -> bool {
    verify(self, Some(origin))
  }

  /// Whether this id is a hash rather than an ordinal.
  pub fn is_hashed(&self) -> bool {
    self.0.is_hashed()
  }

  /// The stored form: `0703`, or the marker and sixteen hex digits.
  pub fn stored(&self) -> String {
    let digits = self.0.spelled(ISSUE_DIGITS, HASH_HEX);
    if self.0.is_hashed() {
      format!("{ISSUE_MARKER}{digits}")
    } else {
      digits
    }
  }

  /// The form a table, a view or a message prints: the whole ordinal, or the
  /// marker and a hash's first [`SHOWN_HEX`] hex digits.
  pub fn shown(&self) -> String {
    let digits = self.0.shown(ISSUE_DIGITS, HASH_HEX, SHOWN_HEX);
    if self.0.is_hashed() {
      format!("{ISSUE_MARKER}{digits}")
    } else {
      digits
    }
  }

  /// How a board claims it and `.intentfiles` declares it: `ISSUE:0703`.
  pub fn claim(&self) -> String {
    format!("{CLAIM_ISSUE_PREFIX}{}", self.stored())
  }

  /// The explicitly tagged short spelling an operator can type: `i59`. A hash
  /// is tagged by its marker, so its tagged spelling is its shown form.
  pub fn tagged(&self) -> String {
    self
      .0
      .bare()
      .map_or_else(|| self.shown(), |digits| format!("{ISSUE_TAG}{digits}"))
  }
}

/// How many creation event ids [`mint`] draws for one record before it refuses.
/// A draw past the first follows one whose short form a visible record of the
/// kind already shows. At [`SHOWN_HEX`] hex digits that needs a project of
/// tens of thousands of records to happen twice running, so reaching this bound
/// says the draw is broken, not that the project is large.
pub const MINT_DRAWS: usize = 8;

/// A kind of id the one mint makes: derived from a creation event's id, with a
/// short form no two records of the kind may show.
pub trait Mintable: Sized + PartialEq {
  /// The kind, as the mint's refusal names it.
  const WHAT: &'static str;
  /// The hashed id `origin` mints.
  fn minted_from(origin: &str) -> Self;
  /// The form a table prints, which the mint keeps apart.
  fn short(&self) -> String;
  /// The whole stored spelling, which canon holds and a finding names.
  fn whole(&self) -> String;
  /// Whether this id is of the hashed family, which [`record_order`] asks of
  /// every kind of record id through this one bound (ST0095 WP-08).
  fn hashed(&self) -> bool;
}

impl Mintable for ThreadId {
  const WHAT: &'static str = IdKind::Thread.as_str();
  fn minted_from(origin: &str) -> Self {
    Self::from_origin(origin)
  }
  fn short(&self) -> String {
    self.shown()
  }
  fn whole(&self) -> String {
    self.stored()
  }
  fn hashed(&self) -> bool {
    self.is_hashed()
  }
}

impl Mintable for WpId {
  const WHAT: &'static str = IdKind::WorkPackage.as_str();
  fn minted_from(origin: &str) -> Self {
    Self::from_origin(origin)
  }
  fn short(&self) -> String {
    self.shown()
  }
  fn whole(&self) -> String {
    self.stored()
  }
  fn hashed(&self) -> bool {
    self.is_hashed()
  }
}

impl Mintable for IssueId {
  const WHAT: &'static str = IdKind::Issue.as_str();
  fn minted_from(origin: &str) -> Self {
    Self::from_origin(origin)
  }
  fn short(&self) -> String {
    self.shown()
  }
  fn whole(&self) -> String {
    self.stored()
  }
  fn hashed(&self) -> bool {
    self.is_hashed()
  }
}

/// Why a mint made no id. A record id's mint ([`mint`]) runs inside the verb
/// that asked, before any write, so `Taken` and `ShortFormsShared` wrote
/// nothing. `Seeded` comes from the crate's event-id minter, which some verbs
/// reach after an act (organize's record of what it moved), so it claims
/// nothing about writes: a verb that had already written names that itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintError {
  /// The id a draw derived is one this clone already holds. At sixty-four bits
  /// that is a broken draw, never chance.
  Taken { what: &'static str, id: String },
  /// Every one of [`MINT_DRAWS`] draws gave a short form a visible record of
  /// the kind already shows.
  ShortFormsShared { what: &'static str, draws: usize },
  /// The seeded build's sequence file could not give the next event id (ST0095
  /// AC-03.3): absent, unreadable, not a ULID, or at the end of its increments.
  /// **No fallback to a random id**, which would make the run look reproducible
  /// when it is not. Only a build with the `seeded-ids` feature, and so only a
  /// debug build, has this variant.
  #[cfg(feature = "seeded-ids")]
  Seeded {
    file: std::path::PathBuf,
    why: String,
  },
}

impl fmt::Display for MintError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MintError::Taken { what, id } => {
        write!(
          f,
          "the new {what}'s id {id} is one this project already holds"
        )
      }
      MintError::ShortFormsShared { what, draws } => write!(
        f,
        "every one of {draws} new {what} ids shows a short form a {what} here already shows"
      ),
      #[cfg(feature = "seeded-ids")]
      MintError::Seeded { file, why } => write!(
        f,
        "the seeded id sequence at {} gave no next id: {why}",
        file.display()
      ),
    }
  }
}

impl std::error::Error for MintError {}

impl crate::remedy::Remedy for MintError {
  fn remedy(&self) -> String {
    match self {
      #[cfg(feature = "seeded-ids")]
      MintError::Seeded { file, .. } => crate::event::seeded::remedy(file),
      MintError::Taken { .. } | MintError::ShortFormsShared { .. } => {
        "nothing was written. A mint draws a fresh creation event for every id, so this is a \
         defect in Intent, not in the project: report it with the command that met it"
          .to_string()
      }
    }
  }
}

/// **THE ONE CHECK OF A HASHED ID** (ST0095 design.md item 8, WP-09): whether
/// `id` is the hash of `origin`, the creation event its record names, by the
/// mint's own derivation ([`Mintable::minted_from`]), so no second hasher
/// exists. A hashed id whose record names no origin does not verify. An
/// ordinal has no origin to answer to, so it never does either: doctor asks
/// only of a hashed id.
pub fn verify<I: Mintable>(id: &I, origin: Option<&str>) -> bool {
  id.hashed() && origin.is_some_and(|origin| I::minted_from(origin) == *id)
}

/// **THE ONE MINT** (ST0095 WP-06): a new record's id, and the creation event
/// id it derives from, which the record carries as `origin` and its creation
/// event carries as its own id. `st new`, `wp new`, `issues add` and the
/// re-mints call it, and nothing else makes an id.
///
/// `draw` gives a creation event's id: the crate's one minter of event ids in
/// every caller but a test of this function, so there is no second source of
/// randomness. `visible` is every id of the kind this clone can see where the
/// new one will live: the project's threads or issues, or one thread's work
/// packages.
///
/// It refuses an id equal to a visible one, which at sixty-four bits never
/// fires. It draws again while the new short form equals a visible record's,
/// so two records that have met never show one short form, and it refuses
/// after [`MINT_DRAWS`] draws rather than loop. A draw that cannot give an id
/// (the seeded build's sequence file, ST0095 AC-03.3) is the refusal it names.
pub fn mint<I: Mintable>(
  visible: &[I],
  mut draw: impl FnMut() -> Result<String, MintError>,
) -> Result<(String, I), MintError> {
  for _ in 0..MINT_DRAWS {
    let origin = draw()?;
    let id = I::minted_from(&origin);
    if visible.contains(&id) {
      return Err(MintError::Taken {
        what: I::WHAT,
        id: id.short(),
      });
    }
    let short = id.short();
    if visible.iter().all(|seen| seen.short() != short) {
      return Ok((origin, id));
    }
  }
  Err(MintError::ShortFormsShared {
    what: I::WHAT,
    draws: MINT_DRAWS,
  })
}

/// What separates the thread from the package in a work package's address.
const WP_SEPARATOR: char = '/';

/// What a package's heading puts before its id, and what an operator may type
/// before one: `WP-03`. Public so that a scanner reading a package heading in
/// text takes the marker from here (ST1c2db93, ic's MCP scanner).
pub const WP_HEADING_PREFIX: &str = "WP-";

/// **THE ONE WRITER OF A THREAD-SCOPED SUBJECT**: `thread`, the separator and
/// `tail`, eg `ST0092/AC-01.1` or an attachment's `ST0092/design.md`. Event
/// subjects and refusals name a thing under a thread this way, and
/// [`split_wp_address`] is its inverse.
pub fn scoped_address(thread: &str, tail: &str) -> String {
  format!("{thread}{WP_SEPARATOR}{tail}")
}

/// A work package's whole stored address: `ST0092/09`. Claims, event subjects,
/// `--json` and the `created:` line carry it.
pub fn wp_address(thread: &str, wp: WpId) -> String {
  scoped_address(thread, &wp.stored())
}

/// A work package's address as a table or a message prints it.
pub fn wp_address_shown(thread: &ThreadId, wp: WpId) -> String {
  scoped_address(&thread.shown(), &wp.shown())
}

/// The form a table, a view or a message prints of a STORED id, read whole:
/// a thread's (`ST` and 16 hex shows `ST` and 7; an ordinal as stored), an
/// issue's, or a work package's address `<thread>/<package>` (each part by
/// its own family, the package's 8 hex shown as 4). `None` when the text is
/// no stored id of any family, so a caller decides what a non-id is and no
/// damaged id is printed as if it were one. Never a prefix read: an
/// all-digit short form such as `ST0123456` is a shown form, not a stored
/// one, and is `None` here (ST0095 WP-11, AC-11.1).
///
/// The parses cannot take one another's text (cc's read): a thread id begins
/// `ST`, an issue id is four digits or `I` and hex, and only an address holds
/// the separator, so the first typed parse that takes the text is the only
/// one that could.
pub fn shown_of(stored: &str) -> Option<String> {
  if let Ok(thread) = ThreadId::parse(stored) {
    return Some(thread.shown());
  }
  if let Ok(issue) = IssueId::parse(stored) {
    return Some(issue.shown());
  }
  let (thread, package) = split_wp_address(stored)?;
  let thread = ThreadId::parse(thread).ok()?;
  let package = WpId::parse(package).ok()?;
  Some(wp_address_shown(&thread, package))
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
  read_claim(claim).is_whole()
}

/// A claim's parts by the kind each names, as written and not yet read: what
/// [`read_claim`] splits a claim into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Claim<'a> {
  /// No separator and no issue prefix: a thread's part.
  Thread(&'a str),
  /// `<thread>/<package>`: a work package's two parts.
  Package(&'a str, &'a str),
  /// What follows [`CLAIM_ISSUE_PREFIX`]: an issue's part.
  Issue(&'a str),
}

impl Claim<'_> {
  /// Whether every part is its kind's whole stored id, which is what a board
  /// stores: [`is_claim_address`].
  pub fn is_whole(&self) -> bool {
    match self {
      Claim::Thread(thread) => ThreadId::parse(thread).is_ok(),
      Claim::Package(thread, wp) => ThreadId::parse(thread).is_ok() && WpId::parse(wp).is_ok(),
      Claim::Issue(issue) => IssueId::parse(issue).is_ok(),
    }
  }
}

/// **THE ONE READER OF THE CLAIM GRAMMAR**: `ST0000`, `ST0000/01`,
/// `ISSUE:0000` and their git-style forms, split by kind. Each part is left as
/// written, so [`is_claim_address`] asks for it whole and a door that
/// completes a prefix reads it with its kind's reader.
pub fn read_claim(claim: &str) -> Claim<'_> {
  if let Some(issue) = claim.strip_prefix(CLAIM_ISSUE_PREFIX) {
    return Claim::Issue(issue);
  }
  match split_wp_address(claim) {
    None => Claim::Thread(claim),
    Some((thread, wp)) => Claim::Package(thread, wp),
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

  // The tag letters are the consts [`IdKind::tag`] reads, never spelled again.
  let (thread_tag, issue_tag) = (THREAD_TAG.to_string(), ISSUE_TAG.to_string());
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
  match hashed_spelling(raw) {
    Some(Named::Thread(id)) => return Ok(id),
    Some(Named::Issue(_)) => {
      return Err(IdError::WrongCollection {
        named: IdKind::Issue,
      });
    }
    None => {}
  }
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
  match hashed_spelling(raw) {
    Some(Named::Issue(id)) => return Ok(id),
    Some(Named::Thread(_)) => {
      return Err(IdError::WrongCollection {
        named: IdKind::Thread,
      });
    }
    None => {}
  }
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
  // A whole stored hash, in any case; a shorter hex spelling is a prefix, which
  // only the store can resolve (ST0095 WP-07).
  if digits.len() == WP_HASH_HEX
    && let Ok(form) = hashed_hex(&digits.to_ascii_lowercase(), WP_HASH_HEX)
  {
    return Ok(WpId(form));
  }
  WpId::from_ordinal(ordinal_of(digits)?)
}

/// A whole hashed thread or issue spelling, in any case: `ST` or [`ISSUE_MARKER`]
/// and sixteen hex digits. `None` for anything else, an ordinal spelling or a
/// prefix included, because a prefix names a record only against the store
/// (ST0095 WP-07).
fn hashed_spelling(raw: &str) -> Option<Named> {
  let t = raw.trim();
  let hex = |rest: &str| hashed_hex(&rest.to_ascii_lowercase(), HASH_HEX).ok();
  if let Some(form) = strip_ci(t, THREAD_PREFIX).and_then(hex) {
    return Some(Named::Thread(ThreadId::of(form)));
  }
  strip_ci(t, ISSUE_MARKER)
    .and_then(hex)
    .map(|form| Named::Issue(IssueId(form)))
}

/// A thread or an issue, by its id: what a spelling names at a door that knows
/// the collection, and what owns a path
/// ([`Project::owner_of`](crate::project::Project::owner_of)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Named {
  Thread(ThreadId),
  Issue(IssueId),
}

impl Named {
  /// The thread this names, when it names one.
  pub fn thread(&self) -> Option<&ThreadId> {
    match self {
      Named::Thread(id) => Some(id),
      Named::Issue(_) => None,
    }
  }

  /// The family this names.
  pub fn kind(&self) -> IdKind {
    match self {
      Named::Thread(_) => IdKind::Thread,
      Named::Issue(_) => IdKind::Issue,
    }
  }

  /// The stored spelling of the id this names.
  pub fn stored(&self) -> String {
    match self {
      Named::Thread(id) => id.stored(),
      Named::Issue(number) => number.stored(),
    }
  }
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
  if let Some(named) = hashed_spelling(raw) {
    return Ok(named);
  }
  let s = spelling(raw)?;
  match s.tag {
    Some(IdKind::Thread) => ThreadId::from_ordinal(s.seq).map(Named::Thread),
    Some(IdKind::Issue) => IssueId::from_ordinal(s.seq).map(Named::Issue),
    // No tag names a work package ([`IdKind::tag`] is `None` for one), so
    // `spelling` never answers this; a package is named under its thread.
    Some(IdKind::WorkPackage) => Err(IdError::NotAnId),
    None => Err(IdError::Ambiguous {
      thread: ThreadId::from_ordinal(s.seq)?,
      issue: IssueId::from_ordinal(s.seq)?,
    }),
  }
}

// ---------------------------------------------------------------------------
// Prefix resolution (ST0095 WP-07): what a typed spelling names in a scope.
// ---------------------------------------------------------------------------

/// What an operator typed, read with no store: a whole id, or the front of a
/// hashed one, which only the ids in a scope can complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spelled<T> {
  /// An ordinal in any spelling the doors took before ST0095, or a hashed id's
  /// whole stored hex, with or without its marker and in any case.
  Whole(T),
  /// The lowercase front of a hashed id's hex: at least [`prefix_floor`]
  /// digits, and fewer than the kind stores.
  Prefix(String),
}

/// The fewest hex digits a prefix spelled as `hex` needs, at a kind whose
/// ordinal is `ordinal_width` digits: [`MIN_PREFIX_HEX`], and one more than the
/// ordinal width when it is digits alone, because an ordinal-shaped spelling is
/// always the ordinal. Five for a thread or an issue, four for a package.
pub fn prefix_floor(hex: &str, ordinal_width: usize) -> usize {
  if hex.bytes().all(|b| b.is_ascii_digit()) {
    MIN_PREFIX_HEX.max(ordinal_width + 1)
  } else {
    MIN_PREFIX_HEX
  }
}

/// `rest`, what follows a kind's optional marker, read as a hashed id's hex:
/// its whole stored hex or a prefix of it, lowercased. `None` when it is no hex
/// at all or ordinal-shaped, which the ordinal readers answer.
fn typed_hex(
  rest: &str,
  ordinal_width: usize,
  hash_width: usize,
) -> Option<Result<String, IdError>> {
  let hex = rest.trim().to_ascii_lowercase();
  if hex.is_empty() || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
    return None;
  }
  if hex.bytes().all(|b| b.is_ascii_digit()) && hex.len() <= ordinal_width {
    return None;
  }
  if hex.len() > hash_width {
    return Some(Err(IdError::OutOfRange));
  }
  let floor = prefix_floor(&hex, ordinal_width);
  if hex.len() < floor {
    return Some(Err(IdError::TooShort { floor }));
  }
  Some(Ok(hex))
}

/// A typed hex spelling as [`Spelled`]: the whole stored hex is the id itself.
fn spelled_hex<T>(hex: String, hash_width: usize, whole: impl Fn(Form) -> T) -> Spelled<T> {
  match hashed_hex(&hex, hash_width) {
    Ok(form) if hex.len() == hash_width => Spelled::Whole(whole(form)),
    _ => Spelled::Prefix(hex),
  }
}

/// What every URL carries and no id spelling does, so an address given where an
/// id is wanted is told from a malformed id by the readers below, before any
/// part of it is read as a fragment of an id (vc, 2026-09-15; issue Ic7d939d).
/// The scheme itself stays [`crate::address`]'s to spell.
const ADDRESS_MARK: &str = "://";

/// [`IdError::Address`] when `raw` is an address: the readers' first question.
fn refuse_an_address(raw: &str) -> Result<(), IdError> {
  if raw.contains(ADDRESS_MARK) {
    return Err(IdError::Address);
  }
  Ok(())
}

/// **THE ONE MARKED-HEX PREDICATE**: whether `t` is `marker` and then hex a
/// door of that kind takes, whole or a prefix long enough. A word that only
/// begins with the letter (`ic`, `stab`) is not.
fn marked_hex(t: &str, marker: &str, ordinal_width: usize) -> bool {
  matches!(
    strip_ci(t, marker).and_then(|rest| typed_hex(rest, ordinal_width, HASH_HEX)),
    Some(Ok(_))
  )
}

/// An operator's spelling of a THREAD, read with no store: everything
/// [`normalise_thread_id`] takes, and a hashed thread's hex or a prefix of it,
/// its `ST` marker optional. An issue's marker on hex is the other collection.
pub fn read_thread(raw: &str) -> Result<Spelled<ThreadId>, IdError> {
  refuse_an_address(raw)?;
  let t = raw.trim();
  if marked_hex(t, ISSUE_MARKER, ISSUE_DIGITS) {
    return Err(IdError::WrongCollection {
      named: IdKind::Issue,
    });
  }
  let rest = strip_ci(t, THREAD_PREFIX).unwrap_or(t);
  match typed_hex(rest, THREAD_DIGITS, HASH_HEX) {
    Some(hex) => Ok(spelled_hex(hex?, HASH_HEX, ThreadId::of)),
    None => normalise_thread_id(raw).map(Spelled::Whole),
  }
}

/// An operator's spelling of an ISSUE, read with no store, as [`read_thread`]
/// reads a thread's: its [`ISSUE_MARKER`] optional, and a thread's marker on
/// hex the other collection.
pub fn read_issue(raw: &str) -> Result<Spelled<IssueId>, IdError> {
  refuse_an_address(raw)?;
  let t = raw.trim();
  if marked_hex(t, THREAD_PREFIX, THREAD_DIGITS) {
    return Err(IdError::WrongCollection {
      named: IdKind::Thread,
    });
  }
  let rest = strip_ci(t, ISSUE_MARKER).unwrap_or(t);
  match typed_hex(rest, ISSUE_DIGITS, HASH_HEX) {
    Some(hex) => Ok(spelled_hex(hex?, HASH_HEX, IssueId)),
    None => normalise_issue_id(raw).map(Spelled::Whole),
  }
}

/// An operator's spelling of a WORK PACKAGE under its thread, read with no
/// store: everything [`normalise_wp_id`] takes, and a hashed package's hex or
/// a prefix of it, `WP-` optional. `1234` is longer than a package ordinal, so
/// it is a prefix.
pub fn read_wp(raw: &str) -> Result<Spelled<WpId>, IdError> {
  refuse_an_address(raw)?;
  let t = raw.trim();
  let rest = strip_ci(t, WP_HEADING_PREFIX).unwrap_or(t);
  match typed_hex(rest, WP_DIGITS, WP_HASH_HEX) {
    Some(hex) => Ok(spelled_hex(hex?, WP_HASH_HEX, WpId)),
    None => normalise_wp_id(raw).map(Spelled::Whole),
  }
}

/// **THE ONE READER OF AN ADDRESS'S PACKAGE SEGMENT** (ST1c2db93 P9): what
/// [`read_wp`] takes, whole or a prefix, but never a view's `WP-` heading,
/// which no address carries. The address's parse asks it once, and every
/// reader after takes the typed value.
pub fn address_segment(raw: &str) -> Result<Spelled<WpId>, IdError> {
  if strip_ci(raw.trim(), WP_HEADING_PREFIX).is_some() {
    return Err(IdError::NotAnId);
  }
  read_wp(raw)
}

/// Whether `raw` is a spelling a door of `kind` takes, whole or a prefix, with
/// no store asked: for a face's parse before a store is open, and for an
/// address's segments. Its refusal is [`refusal`]'s.
pub fn check_spelling(raw: &str, kind: IdKind) -> Result<(), IdError> {
  match kind {
    IdKind::Thread => read_thread(raw).map(drop),
    IdKind::WorkPackage => read_wp(raw).map(drop),
    IdKind::Issue => read_issue(raw).map(drop),
  }
}

/// **WHICH KIND OF RECORD `word` NAMES, WHEN IT IS SPELLED EXACTLY AS A FACE
/// PRINTS ONE**, for a scanner that looks for ids in text it did not write
/// (issue 0736): a thread's `ST` id, a hashed issue's `I` id, or a package's
/// `WP-` heading, each in the form a table, view or message shows it or the
/// form the store keeps, marker and case included.
///
/// **NOT AN OPERATOR'S SPELLING.** A door takes `s59`, `st0059`, a bare hex
/// prefix or any case; text that only mentions a record spells it the one way
/// a face printed it, so a word that differs from both of an id's forms is not
/// one here. That keeps an English word behind a marker (`STAFFED`, `Iced`)
/// from being read as an id. A prefix shorter or longer than the shown width
/// is refused for the same reason, and is not a missed leak: no face prints
/// one, so Intent cannot have written it.
///
/// **THE CALLER OWNS ITS WORD BOUNDARIES.** One word is one id: a package's
/// address (`ST0096/05`) is a thread's id, a `/` and the package's bare form,
/// and a caller that splits on `/` sees the thread. The waist reads ids
/// ([`marked`]), labels ([`label_at`]) and head-numbers ([`head_number_at`]),
/// and decides which of them are examples. Where to split, which to count, and
/// heads the waist does not name are each scanner's own.
///
/// **A BARE ISSUE ORDINAL IS OUT OF REACH.** An ordinal issue is shown as its
/// four digits with no marker (`0736`), which prose cannot tell from any other
/// four-digit number, so a scanner that must catch one needs a context of its
/// own, such as the word `issue` before it. The rest of the ordinal family, and
/// every hashed id, carries its marker.
pub fn marked(word: &str) -> Option<IdKind> {
  // A hashed id's shown form: the marker and exactly the shown width of hex,
  // lowercase as a face prints it.
  let shown_hex =
    |marker: &str, hex: &str, width: usize| hex.len() == width && word == format!("{marker}{hex}");
  if word.starts_with(THREAD_PREFIX) {
    match read_thread(word) {
      Ok(Spelled::Whole(id)) if word == id.shown() || word == id.stored() => {
        return Some(IdKind::Thread);
      }
      Ok(Spelled::Prefix(hex)) if shown_hex(THREAD_PREFIX, &hex, SHOWN_HEX) => {
        return Some(IdKind::Thread);
      }
      _ => {}
    }
  }
  if let Some(rest) = word.strip_prefix(WP_HEADING_PREFIX) {
    match read_wp(word) {
      Ok(Spelled::Whole(id)) if rest == id.shown() || rest == id.stored() => {
        return Some(IdKind::WorkPackage);
      }
      Ok(Spelled::Prefix(hex)) if shown_hex(WP_HEADING_PREFIX, &hex, WP_SHOWN_HEX) => {
        return Some(IdKind::WorkPackage);
      }
      _ => {}
    }
  }
  if word.starts_with(ISSUE_MARKER) {
    match read_issue(word) {
      Ok(Spelled::Whole(id)) if id.is_hashed() && (word == id.shown() || word == id.stored()) => {
        return Some(IdKind::Issue);
      }
      Ok(Spelled::Prefix(hex)) if shown_hex(ISSUE_MARKER, &hex, SHOWN_HEX) => {
        return Some(IdKind::Issue);
      }
      _ => {}
    }
  }
  None
}

/// **WHETHER `word` IS AN EXAMPLE RATHER THAN A RECORD**, for a scanner that
/// has found an id with [`marked`] and must not count a format example as
/// one of ours (issue 0736): the zero family, all zeros behind its marker
/// (`ST0000`, `WP-00`), or the worked example of a hashed id, [`EXAMPLE_HEX`]
/// or [`EXAMPLE_WP_HEX`] or a front of it, behind its marker. One predicate,
/// so every scanner exempts the same words.
///
/// A minted id whose front happens to equal the example's is exempt too: a
/// miss of about 1 in 16^7 for a thread or an issue, accepted so that the
/// format text can name a real-looking id (cc's meaning, issue 0736).
pub fn is_example(word: &str) -> bool {
  let behind = |marker: &str, example: &str| {
    word
      .strip_prefix(marker)
      .is_some_and(|rest| !rest.is_empty() && (all_zeros(rest) || example.starts_with(rest)))
  };
  behind(THREAD_PREFIX, EXAMPLE_HEX)
    || behind(ISSUE_MARKER, EXAMPLE_HEX)
    || behind(WP_HEADING_PREFIX, EXAMPLE_WP_HEX)
}

/// Whether `part` is all zeros: the zero rule that marks a format example
/// (`ST0000`, `WP-00`, `AC-0.0`, `AT-00`). [`is_example`], [`label_at`] and
/// [`head_number_at`] each ask it, so the waist spells the rule once.
fn all_zeros(part: &str) -> bool {
  part.bytes().all(|b| b == b'0')
}

/// The ids a typed prefix is completed against: the records the store holds.
///
/// **THE STORE IS INJECTED, AS PRESENCE IS** ([`crate::resolve`],
/// [`crate::nav::land`]): the facade implements it from its canon and a test
/// from a literal set, so every door asks the one rule below and none of them
/// writes a second.
pub trait IdScope {
  /// Every thread this project holds.
  fn threads(&self) -> Vec<ThreadId>;
  /// Every issue this project holds.
  fn issues(&self) -> Vec<IssueId>;
  /// Every work package `thread` holds: the scope a package's id is minted in,
  /// and so the scope its prefix is completed in.
  fn wps(&self, thread: &ThreadId) -> Vec<WpId>;
}

/// The scope of a door that has no project, where no prefix completes: the
/// TUI's lobby, and a test that holds no records.
pub struct NoRecords;

impl IdScope for NoRecords {
  fn threads(&self) -> Vec<ThreadId> {
    Vec::new()
  }
  fn issues(&self) -> Vec<IssueId> {
    Vec::new()
  }
  fn wps(&self, _thread: &ThreadId) -> Vec<WpId> {
    Vec::new()
  }
}

/// What a typed spelling names in its scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Found<T> {
  /// Exactly one: an ordinal-shaped spelling or a whole hashed id as typed,
  /// never looked up, because whether it exists is the lookup's question; or
  /// the one id in scope a prefix begins.
  One(T),
  /// A prefix no id in scope begins.
  Nothing,
  /// A prefix several ids in scope begin: every one, in the family order,
  /// and none picked.
  Many(Vec<T>),
}

impl<T> Found<T> {
  /// **THE ONE FOLD OF HITS INTO AN ANSWER**: none, one, or several, none
  /// picked.
  fn from_hits(mut hits: Vec<T>) -> Found<T> {
    match hits.len() {
      0 => Found::Nothing,
      // `pop` on a one-element vector is its one element.
      1 => hits.pop().map_or(Found::Nothing, Found::One),
      _ => Found::Many(hits),
    }
  }

  /// The same answer about another value.
  pub fn map<U>(self, f: impl Fn(T) -> U) -> Found<U> {
    match self {
      Found::One(id) => Found::One(f(id)),
      Found::Nothing => Found::Nothing,
      Found::Many(ids) => Found::Many(ids.into_iter().map(f).collect()),
    }
  }
}

/// Every id in `scope` that `prefix` begins, as one answer.
fn completed<T: Ord>(scope: Vec<T>, begins: impl Fn(&T) -> bool) -> Found<T> {
  let mut hits: Vec<T> = scope.into_iter().filter(|id| begins(id)).collect();
  hits.sort();
  hits.dedup();
  Found::from_hits(hits)
}

fn threads_begun(scope: &impl IdScope, prefix: &str) -> Found<ThreadId> {
  completed(scope.threads(), |id| id.form.hex_begins(HASH_HEX, prefix))
}

fn issues_begun(scope: &impl IdScope, prefix: &str) -> Found<IssueId> {
  completed(scope.issues(), |id| id.0.hex_begins(HASH_HEX, prefix))
}

/// **THE ONE RESOLVER OF A TYPED THREAD** (ST0095 WP-07): the thread `raw`
/// names in `scope`. An ordinal-shaped spelling is always the ordinal, a whole
/// hashed id is itself, and a prefix is completed against the threads the
/// scope holds. A refusal of the spelling is [`refusal`]'s to word.
pub fn resolve_thread(raw: &str, scope: &impl IdScope) -> Result<Found<ThreadId>, IdError> {
  Ok(match read_thread(raw)? {
    Spelled::Whole(id) => Found::One(id),
    Spelled::Prefix(hex) => threads_begun(scope, &hex),
  })
}

/// The one resolver of a typed issue, as [`resolve_thread`] is a thread's.
pub fn resolve_issue(raw: &str, scope: &impl IdScope) -> Result<Found<IssueId>, IdError> {
  Ok(match read_issue(raw)? {
    Spelled::Whole(id) => Found::One(id),
    Spelled::Prefix(hex) => issues_begun(scope, &hex),
  })
}

/// The one resolver of a typed work package under `thread`, completed only
/// against that thread's packages: a prefix never reaches another thread's.
pub fn resolve_wp(
  thread: &ThreadId,
  raw: &str,
  scope: &impl IdScope,
) -> Result<Found<WpId>, IdError> {
  Ok(match read_wp(raw)? {
    Spelled::Whole(id) => Found::One(id),
    Spelled::Prefix(hex) => complete_wp(thread, &hex, scope),
  })
}

/// A package's prefix, already read, completed only among `thread`'s
/// packages: what [`resolve_wp`] does after its read, for a caller that holds
/// the typed segment (an address's).
pub fn complete_wp(thread: &ThreadId, hex: &str, scope: &impl IdScope) -> Found<WpId> {
  completed(scope.wps(thread), |id| id.0.hex_begins(WP_HASH_HEX, hex))
}

/// The one resolver at the door that does not know the collection (`intent
/// edit`, [`normalise_id`]'s callers; cc's rule 5). A marked spelling names its
/// kind. Bare hex is completed against threads and issues together, so a
/// prefix both begin is one ambiguity across both kinds, threads first. Untagged
/// ordinal digits stay [`IdError::Ambiguous`], which the project's ladder
/// ([`crate::resolve`]) answers as before.
pub fn resolve_named(raw: &str, scope: &impl IdScope) -> Result<Found<Named>, IdError> {
  match read_named(raw)? {
    NamedSpelling::Thread(Spelled::Whole(id)) => Ok(Found::One(Named::Thread(id))),
    NamedSpelling::Thread(Spelled::Prefix(hex)) => {
      Ok(threads_begun(scope, &hex).map(Named::Thread))
    }
    NamedSpelling::Issue(Spelled::Whole(id)) => Ok(Found::One(Named::Issue(id))),
    NamedSpelling::Issue(Spelled::Prefix(hex)) => Ok(issues_begun(scope, &hex).map(Named::Issue)),
    NamedSpelling::Either(hex) => Ok(complete_either(&hex, scope)),
  }
}

/// Bare hex, already read as [`NamedSpelling::Either`], completed against
/// threads and issues together: a prefix both begin is one ambiguity across
/// both kinds, threads first. For a caller that holds the reading, so nothing
/// is read twice.
pub fn complete_either(hex: &str, scope: &impl IdScope) -> Found<Named> {
  let threads = threads_begun(scope, hex).map(Named::Thread);
  let issues = issues_begun(scope, hex).map(Named::Issue);
  let hits: Vec<Named> = [threads, issues]
    .into_iter()
    .flat_map(|found| match found {
      Found::One(named) => vec![named],
      Found::Nothing => Vec::new(),
      Found::Many(named) => named,
    })
    .collect();
  Found::from_hits(hits)
}

/// What a spelling at the door that does not know the collection is, read
/// with no store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamedSpelling {
  /// A thread: tagged, marked, or a whole or partial hash after `ST`.
  Thread(Spelled<ThreadId>),
  /// An issue: tagged, marked, or a whole or partial hash after its marker.
  Issue(Spelled<IssueId>),
  /// Bare hex no ordinal could be: the front of a thread's or an issue's hash,
  /// which only the store can tell apart.
  Either(String),
}

/// [`resolve_named`]'s reading, with no store: untagged ordinal digits are
/// [`IdError::Ambiguous`], as [`normalise_id`] has them.
///
/// **A MARKER IS `named_reading`'S TO COUNT, AND NOWHERE ELSE'S.** Hex behind
/// a marker that is too short for a prefix reads here exactly as a word that
/// begins with the letter (`ST3f9` as `stab`, `i3f9` as `ic`), so this door,
/// which does not know the collection, does not call either a prefix. A door
/// that knows its kind ([`read_thread`], [`read_issue`]) refuses the short one
/// naming the minimum.
pub fn read_named(raw: &str) -> Result<NamedSpelling, IdError> {
  refuse_an_address(raw)?;
  match named_reading(raw) {
    NamedReading::Thread => return read_thread(raw).map(NamedSpelling::Thread),
    NamedReading::Issue => return read_issue(raw).map(NamedSpelling::Issue),
    NamedReading::Both => {}
  }
  if let Some(hex) = typed_hex(raw.trim(), THREAD_DIGITS.max(ISSUE_DIGITS), HASH_HEX) {
    return hex.map(NamedSpelling::Either);
  }
  match normalise_id(raw)? {
    Named::Thread(id) => Ok(NamedSpelling::Thread(Spelled::Whole(id))),
    Named::Issue(id) => Ok(NamedSpelling::Issue(Spelled::Whole(id))),
  }
}

/// The address segment a prefix of `kind`'s hex carries: the prefix behind
/// its family's marker, so the segment reads as that family's, and a
/// package's bare (ST1c2db93 P21). It borrows the hex, so a caller holding
/// one builds no [`Spelled`] to spell it; each `segment` spells its prefix here.
pub fn prefix_segment(kind: IdKind, hex: &str) -> String {
  match kind {
    IdKind::Thread => format!("{THREAD_PREFIX}{hex}"),
    IdKind::Issue => format!("{ISSUE_MARKER}{hex}"),
    IdKind::WorkPackage => hex.to_string(),
  }
}

impl Spelled<ThreadId> {
  /// The spelling an address segment carries: the stored id, or the prefix
  /// behind the thread's marker, so the segment reads as a thread's.
  pub fn segment(&self) -> String {
    match self {
      Spelled::Whole(id) => id.stored(),
      Spelled::Prefix(hex) => prefix_segment(IdKind::Thread, hex),
    }
  }

  /// Whether this spelling names the thread stored as `stored`: that id
  /// whole, or the front of its hex. A cited line names a hashed thread by
  /// its shown front, so the gate asks this and never compares strings.
  pub fn names(&self, stored: &str) -> bool {
    match self {
      Spelled::Whole(id) => id == stored,
      Spelled::Prefix(hex) => {
        ThreadId::parse(stored).is_ok_and(|id| id.form.hex_begins(HASH_HEX, hex))
      }
    }
  }
}

impl Spelled<IssueId> {
  /// The spelling an address segment carries, as a thread's does.
  pub fn segment(&self) -> String {
    match self {
      Spelled::Whole(id) => id.stored(),
      Spelled::Prefix(hex) => prefix_segment(IdKind::Issue, hex),
    }
  }
}

impl Spelled<WpId> {
  /// The spelling an address segment carries: the stored id, or the prefix.
  pub fn segment(&self) -> String {
    match self {
      Spelled::Whole(id) => id.stored(),
      Spelled::Prefix(hex) => prefix_segment(IdKind::WorkPackage, hex),
    }
  }
}

/// What an operator who typed `raw` would type to name the `wanted` collection's
/// id with the same number: `i21` asked of a thread door is `s21`. The remedy a
/// [`IdError::WrongCollection`] refusal offers. `None` when `raw` spells no id,
/// or one past the wanted kind's width, or when a work package is wanted, which
/// no number alone names.
pub fn respelled_as(raw: &str, wanted: IdKind) -> Option<String> {
  let s = spelling(raw).ok()?;
  match wanted {
    IdKind::Thread => ThreadId::from_ordinal(s.seq).ok().map(|id| id.tagged()),
    IdKind::WorkPackage => None,
    IdKind::Issue => IssueId::from_ordinal(s.seq).ok().map(|id| id.tagged()),
  }
}

/// A spelling a door refused: what is wrong with it, and what to type instead
/// (ST0095 WP-10; the Highlander sweep's D6).
///
/// **THE ONE WORDING OF A BAD ID, FOR EVERY FACE.** The CLI worded each
/// [`IdError`] itself, MCP folded every one into a message of its own, and each
/// carried a hand-written example. A face renders this one: the CLI as an error
/// and its remedy, MCP as one argument error, and a facade door through its own
/// error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
  /// What is wrong with the spelling.
  pub message: String,
  /// What to type instead: a respelling that works, or the sentence of the
  /// wanted id's form.
  pub remedy: String,
}

/// The refusal of `raw`, for the reason `e` gives, at a door that wanted
/// `wanted`.
///
/// **THE WANTED COLLECTION IS THE CALLER'S TO SUPPLY.** An [`IdError`] is a fact
/// about a spelling and holds no door's context: `i59` is a well-formed id, and
/// whether it is wrong depends on which door was asked.
pub fn refusal(raw: &str, e: &IdError, wanted: IdKind) -> Refusal {
  let (message, remedy) = match e {
    IdError::NotAnId => (
      format!("`{raw}` is not {} id", wanted.with_article()),
      wanted.form().to_string(),
    ),
    IdError::OutOfRange => (
      format!("`{raw}` is out of range for {} id", wanted.with_article()),
      wanted.form().to_string(),
    ),
    IdError::TooShort { floor } => (
      format!(
        "`{raw}` is too short to name {} id: a prefix of a hashed id is at least {floor} hex digits",
        wanted.with_article()
      ),
      wanted.form().to_string(),
    ),
    // **THE REFUSAL OFFERS THE SPELLING THAT WORKS**: the other collection is
    // named, and never reported as a not-found.
    IdError::WrongCollection { named } => (
      format!(
        "`{raw}` names {}, and this verb takes {}",
        named.with_article(),
        wanted.with_article()
      ),
      respelled_as(raw, wanted).map_or_else(
        || wanted.form().to_string(),
        |respelled| format!("`{respelled}` names the {}", wanted.as_str()),
      ),
    ),
    IdError::Ambiguous { thread, issue } => return ambiguity(raw, thread, issue),
    IdError::Address => return address_where_id_wanted(raw, wanted),
  };
  Refusal { message, remedy }
}

/// The refusal of an address where a door takes a bare id: the address named
/// whole, and the remedy the id's form, never an id read out of the address,
/// which can name another project's record (issue 0338).
pub fn address_where_id_wanted(raw: &str, wanted: IdKind) -> Refusal {
  Refusal {
    message: format!(
      "`{raw}` is an address, and this verb takes {} id",
      wanted.with_article()
    ),
    remedy: format!("name the {} by its id: {}", wanted.as_str(), wanted.form()),
  }
}

/// What a `<thread>/<tail>` spelling is (ST0095 WP-10; the Highlander sweep's
/// W4 and the WP-10 critic's first finding).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scoped {
  /// The tail is a package id: a work package's address. Either part may be a
  /// prefix, which [`resolve_scoped`] completes against a scope.
  Package(Spelled<ThreadId>, Spelled<WpId>),
  /// The tail is no package id, eg `AC-01.1` or an attachment's path: another
  /// thing under the thread, as the event log's subjects spell it.
  Under(Spelled<ThreadId>, String),
}

/// **THE ONE READER OF `<thread>/<tail>`**, so every door that takes one reads
/// the head through the thread door and decides the tail one way. `None` when
/// the text is not that shape: no slash, or a head that spells no id at all.
///
/// **A PART THAT BEGINS AS AN ID AND IS NOT ONE IS REFUSED**, worded for the
/// part that failed: a head that names an issue, is out of range or is too
/// short to be a prefix, or a tail that spells a package id and is no package
/// (`ST0001/100`), which is refused as a package and never re-read whole as a
/// thread (the WP-04 critic's W4). A tail that spells no package id is no
/// refusal: it is [`Scoped::Under`], because the event log writes a
/// criterion's, a test's and an attachment's subjects in this shape. Pure: no
/// store is asked.
pub fn scoped(raw: &str) -> Result<Option<Scoped>, Refusal> {
  if let Err(e) = refuse_an_address(raw) {
    return Err(refusal(raw, &e, IdKind::Thread));
  }
  let Some((head, tail)) = split_wp_address(raw) else {
    return Ok(None);
  };
  let thread = match read_thread(head) {
    Ok(thread) => thread,
    Err(IdError::NotAnId) => return Ok(None),
    Err(e) => return Err(refusal(head, &e, IdKind::Thread)),
  };
  match read_wp(tail) {
    Ok(wp) => Ok(Some(Scoped::Package(thread, wp))),
    Err(IdError::NotAnId) => Ok(Some(Scoped::Under(thread, tail.to_string()))),
    Err(e) => Err(package_part_refused(raw, tail, &e)),
  }
}

/// What a thread-or-package target names: a thread as the thread door spelt
/// it, or a work package with both its ids whole and typed, so no caller
/// parses the thread's id again (the WP-10 critic's ThreadId re-parses).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
  Thread(String),
  Package(ThreadId, WpId),
}

/// **THE ONE CLASSIFIER OF A THREAD-OR-PACKAGE TARGET** (ST1c2db93 P21), read
/// by [`scoped`], the one reader of `<thread>/<tail>` (the WP-04 critic's W4):
/// a package part that is no package is refused naming that part, and never
/// re-read whole as a thread. A package with a part typed as a prefix is
/// completed by `complete`, which a face gives as the facade's
/// [`crate::facade::Facade::resolve_wp`] so the project is opened only when a
/// prefix needs it; `refuse` is the face's rendering of a refusal.
pub fn target<E>(
  raw: &str,
  complete: impl FnOnce() -> Result<(ThreadId, WpId), E>,
  refuse: impl Fn(Refusal) -> E,
) -> Result<Target, E> {
  match scoped(raw).map_err(&refuse)? {
    None => Ok(Target::Thread(raw.to_string())),
    Some(Scoped::Package(Spelled::Whole(thread), Spelled::Whole(wp))) => {
      Ok(Target::Package(thread, wp))
    }
    Some(Scoped::Package(..)) => complete().map(|(thread, wp)| Target::Package(thread, wp)),
    Some(Scoped::Under(_, tail)) => {
      Err(refuse(package_part_refused(raw, &tail, &IdError::NotAnId)))
    }
  }
}

/// What a `<thread>/<tail>` spelling names in a scope: [`scoped`]'s reading,
/// with each prefix completed, its ids whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopedFound {
  /// A work package's address: its thread and its own id.
  Package(ThreadId, WpId),
  /// Another thing under one thread, the tail as typed.
  Under(ThreadId, String),
}

/// [`scoped`], resolved against `scope`: the thread first, then the package
/// only among that thread's packages, so a package prefix never reaches
/// another thread's. `None` when the text is not `<thread>/<tail>`. A thread
/// part that names several threads or none is the answer, and no package is
/// looked for under it.
pub fn resolve_scoped(raw: &str, scope: &impl IdScope) -> Result<Option<ScopedFound>, Unresolved> {
  let (thread, tail) = match scoped(raw).map_err(Unresolved::Refused)? {
    None => return Ok(None),
    Some(Scoped::Package(thread, wp)) => (thread, Ok(wp)),
    Some(Scoped::Under(thread, tail)) => (thread, Err(tail)),
  };
  let thread = match thread {
    Spelled::Whole(id) => id,
    Spelled::Prefix(hex) => threads_begun(scope, &hex).one(&[IdKind::Thread], Candidate::Thread)?,
  };
  Ok(Some(match tail {
    Err(tail) => ScopedFound::Under(thread, tail),
    Ok(Spelled::Whole(wp)) => ScopedFound::Package(thread, wp),
    Ok(Spelled::Prefix(hex)) => {
      let wp = complete_wp(&thread, &hex, scope).one(&[IdKind::WorkPackage], |wp| {
        Candidate::Wp(thread.clone(), wp)
      })?;
      ScopedFound::Package(thread, wp)
    }
  }))
}

/// **THE ONE REFUSAL OF A PACKAGE PART**: `<thread>/<tail>` at a door that
/// takes a work package, refused as a package for the reason `e` gives,
/// naming the part that failed and never re-read whole as a thread (the WP-04
/// critic's W4). A tail that is no package id at all is [`IdError::NotAnId`].
pub fn package_part_refused(raw: &str, tail: &str, e: &IdError) -> Refusal {
  // A tail that is no id at all is named as the package part's fault.
  let why = match e {
    IdError::NotAnId => "no package id".to_string(),
    other => other.to_string(),
  };
  Refusal {
    message: format!("`{raw}` is not a work package: its package part `{tail}` is {why}"),
    remedy: WP_ADDRESS_FORM.to_string(),
  }
}

/// The refusal of a package's prefix where its whole id was wanted: one a
/// completion did not finish, reaching a door that reads only whole ids.
pub fn prefix_not_whole(hex: &str) -> Refusal {
  Refusal {
    message: format!("`{hex}` is a prefix of a work-package id, not a whole one"),
    remedy: WP_ADDRESS_FORM.to_string(),
  }
}

/// The refusal of a target that names no work package, at a door that takes
/// one: its remedy is [`WP_ADDRESS_FORM`].
pub fn not_a_package(raw: &str) -> Refusal {
  Refusal {
    message: format!("`{raw}` is not a work package"),
    remedy: WP_ADDRESS_FORM.to_string(),
  }
}

/// The refusal of an untagged number that names both a thread and an issue, at
/// a door that takes either: [`refusal`]'s wording and the address door's
/// (`AddressError::AmbiguousId`), one sentence for both.
pub fn ambiguity(raw: &str, thread: &ThreadId, issue: &IssueId) -> Refusal {
  Refusal {
    message: format!("`{raw}` names both a steel thread and an issue"),
    remedy: format!(
      "`{}` names the steel thread, `{}` the issue",
      thread.tagged(),
      issue.tagged()
    ),
  }
}

/// The stored subject ids an event subject's spelling names (issue 0713; cc's
/// meaning, 2026-11-20).
///
/// **A FILTER OVER HISTORY, SO AN UNTAGGED NUMBER NAMES EVERY KIND IT COULD.**
/// `94` and `0094` name thread `ST0094` and issue `0094`, and a filter matches
/// the events of both, each row carrying its kind, so nothing is picked for the
/// caller. A tagged spelling (`ST94`, `s94`, `i94`) narrows to its kind, a work
/// package's address names its one package, and a subject that spells no id,
/// a node or a board, is matched as typed. A hash prefix is WP-07's to resolve.
///
/// **`<thread>/<tail>` IS READ BY [`scoped`]**, the one reader every door
/// shares: a package's address names its one package, a criterion's, a test's
/// or an attachment's subject is matched with its thread's id normalised, and
/// a part that begins as an id and fails is refused, never matched against
/// nothing in silence (`i1/2`, `ST0001/100`).
///
/// **A PREFIX IS COMPLETED AGAINST `scope` AND STORED WHOLE** (ST0095 WP-07,
/// AC-07.2): the log's subjects are whole ids, so a filter compares whole ids.
/// A prefix that names several records is refused with them, never widened to
/// all of them; a marked prefix that names nothing is refused as unknown. Bare
/// hex that is too short or names nothing is matched as typed, because a node
/// or a board may be named so (`cc`).
pub fn subject_ids(raw: &str, scope: &impl IdScope) -> Result<Vec<String>, Unresolved> {
  match resolve_scoped(raw, scope)? {
    Some(ScopedFound::Package(thread, wp)) => return Ok(vec![wp_address(thread.as_str(), wp)]),
    Some(ScopedFound::Under(thread, tail)) => {
      return Ok(vec![scoped_address(thread.as_str(), &tail)]);
    }
    None => {}
  }
  let reading = named_reading(raw);
  match resolve_named(raw, scope) {
    Ok(Found::Nothing) if reading == NamedReading::Both => Ok(vec![raw.to_string()]),
    Ok(found) => match found.one(&reading.kinds(), Candidate::of)? {
      Named::Thread(id) => Ok(vec![id.stored()]),
      Named::Issue(id) => Ok(vec![id.stored()]),
    },
    Err(IdError::Ambiguous { thread, issue }) => Ok(vec![thread.stored(), issue.stored()]),
    Err(IdError::NotAnId) => Ok(vec![raw.to_string()]),
    Err(IdError::TooShort { .. }) if reading == NamedReading::Both => Ok(vec![raw.to_string()]),
    Err(e) => Err(Unresolved::Refused(refusal(raw, &e, reading.wanted(raw)))),
  }
}

/// One record a typed spelling could name, its ids whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Candidate {
  Thread(ThreadId),
  Wp(ThreadId, WpId),
  Issue(IssueId),
}

impl Candidate {
  /// The candidate a [`Named`] is.
  pub fn of(named: Named) -> Self {
    match named {
      Named::Thread(id) => Candidate::Thread(id),
      Named::Issue(id) => Candidate::Issue(id),
    }
  }

  /// Its kind.
  pub fn kind(&self) -> IdKind {
    match self {
      Candidate::Thread(_) => IdKind::Thread,
      Candidate::Wp(..) => IdKind::WorkPackage,
      Candidate::Issue(_) => IdKind::Issue,
    }
  }
}

/// Why a typed spelling names no one record (ST0095 WP-07). A face renders
/// each its own way from these values; none of them is prose but the refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unresolved {
  /// It is no spelling of the id wanted: the waist's one wording.
  Refused(Refusal),
  /// It names no record here: every kind it was looked for as.
  Nothing(Vec<IdKind>),
  /// It names several records here: every one, none picked.
  Many(Vec<Candidate>),
}

impl<T> Found<T> {
  /// The one id, or why there is not one: nothing of `kinds`, or several.
  pub fn one(self, kinds: &[IdKind], candidate: impl Fn(T) -> Candidate) -> Result<T, Unresolved> {
    match self {
      Found::One(id) => Ok(id),
      Found::Nothing => Err(Unresolved::Nothing(kinds.to_vec())),
      Found::Many(ids) => Err(Unresolved::Many(ids.into_iter().map(candidate).collect())),
    }
  }
}

/// Which collections a spelling at the door that does not know the collection
/// is resolved in: a marked spelling's kind, or both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NamedReading {
  Thread,
  Issue,
  Both,
}

impl NamedReading {
  /// The kind a refusal of `raw` at this door names as wanted: the marked
  /// kind, else the kind a tag names, else a thread.
  fn wanted(&self, raw: &str) -> IdKind {
    match self {
      NamedReading::Thread => IdKind::Thread,
      NamedReading::Issue => IdKind::Issue,
      NamedReading::Both => spelling(raw)
        .ok()
        .and_then(|s| s.tag)
        .unwrap_or(IdKind::Thread),
    }
  }

  fn kinds(&self) -> Vec<IdKind> {
    match self {
      NamedReading::Thread => vec![IdKind::Thread],
      NamedReading::Issue => vec![IdKind::Issue],
      NamedReading::Both => vec![IdKind::Thread, IdKind::Issue],
    }
  }
}

/// The kind a refusal at the door that does not know the collection names as
/// wanted (ST1c2db93 P5): what `raw`'s marker or tag names, else a thread.
pub fn named_wanted(raw: &str) -> IdKind {
  named_reading(raw).wanted(raw)
}

/// How [`resolve_named`] reads `raw`: hex after a thread's or an issue's marker
/// names that kind, and anything else is asked of both.
fn named_reading(raw: &str) -> NamedReading {
  let t = raw.trim();
  // A marker counts only before hex a door takes, so a word that begins
  // with the letter (`ic`, a node) is not read as a short prefix.
  if marked_hex(t, THREAD_PREFIX, THREAD_DIGITS) {
    NamedReading::Thread
  } else if marked_hex(t, ISSUE_MARKER, ISSUE_DIGITS) {
    NamedReading::Issue
  } else {
    NamedReading::Both
  }
}

// ---------------------------------------------------------------------------
// The one scanner: which ids a text names.
// ---------------------------------------------------------------------------

/// Every steel thread `text` names, in the order they first appear, each once.
///
/// **ONE SCANNER WHERE THERE WERE TWO** (`contract.rs` read cited lines with one,
/// and a second reader read prose; issue I88f333f made it read
/// through the waist). A word is a run of ASCII letters and digits, and a word
/// names a thread when [`marked`] reads it as one: an ordinal, a hashed id's
/// stored form, or its shown front. So `ST3f9a2c1` is that thread's front, and
/// `ST1234abc` is a hashed thread's front, never the ordinal `ST1234`.
pub fn thread_ids_in(text: &str) -> Vec<Spelled<ThreadId>> {
  let mut out: Vec<Spelled<ThreadId>> = Vec::new();
  for word in text.split(|c: char| !c.is_ascii_alphanumeric()) {
    if marked(word) != Some(IdKind::Thread) {
      continue;
    }
    if let Ok(named) = read_thread(word)
      && !out.contains(&named)
    {
      out.push(named);
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
  label_parts(label).map_or(label, |(_, group, _)| group)
}

/// **THE ONE SPLIT OF A LABEL**: the text before its dash, its group, and the
/// text after its dot. [`group_text`] and [`regrouped`] both read it.
fn label_parts(label: &str) -> Option<(&str, &str, &str)> {
  let (head, rest) = label.split_once('-')?;
  let (group, tail) = rest.split_once('.')?;
  Some((head, group, tail))
}

/// `label` with its group replaced by `wp`'s stored id, so a group typed as a
/// prefix is stored whole (ST0095 WP-07): the inverse of [`group_text`] for
/// one group. A label with no group comes back as it is.
pub fn regrouped(label: &str, wp: WpId) -> String {
  match label_parts(label) {
    Some((head, _, tail)) => format!("{head}-{}.{tail}", wp.stored()),
    None => label.to_string(),
  }
}

/// The sequence a label carries after its dot, when it is a number:
/// `AC-01.3` carries 3. Read through [`label_parts`], so no caller strips a
/// head of its own (ST1c2db93 close sweep, A 1.5).
pub fn label_seq(label: &str) -> Option<u32> {
  label_parts(label)?.2.parse().ok()
}

/// A label of `kind` in `group` at `seq`, eg `AC-01.3`: the one constructor
/// of a label, which [`label_parts`] splits back.
pub fn label_of(kind: LabelKind, group: &str, seq: u32) -> String {
  format!("{}-{group}.{seq}", kind.head())
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

/// What a criterion or test label names: the head before its dash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelKind {
  /// `AC-<group>.<n>`: an acceptance criterion.
  Criterion,
  /// `AT-<group>.<n>`: an acceptance test.
  Test,
}

impl LabelKind {
  /// **EVERY KIND OF LABEL, IN ONE LIST** (ST1c2db93, from critic-rust on ic's
  /// efe0): [`label_kind`] and each scanner that walks the kinds take them
  /// from here. This list is kept by hand beside the enum. The compiler makes
  /// [`Self::head`] and [`Self::noun`] answer for a new variant, but nothing
  /// makes this list carry it, so add a new kind to the enum and to this list
  /// in the same change.
  pub const ALL: [LabelKind; 2] = [LabelKind::Criterion, LabelKind::Test];

  /// The head a label of this kind opens with, before its dash.
  pub fn head(self) -> &'static str {
    match self {
      LabelKind::Criterion => "AC",
      LabelKind::Test => "AT",
    }
  }

  /// What a label of this kind labels, for a refusal to name.
  pub fn noun(self) -> &'static str {
    match self {
      LabelKind::Criterion => "criterion",
      LabelKind::Test => "test",
    }
  }

  /// The form a label of this kind takes, for a refusal to name.
  pub fn form(self) -> String {
    format!("{}-<group>.<n>", self.head())
  }

  /// The worked examples of a label of this kind, in both id families, for a
  /// published description to quote: the zero family and the worked package,
  /// eg `AC-0.0 or AC-c2e41a0b.1`. [`label_at`] reads both as examples, so a
  /// scanner of published text does not count them.
  pub fn examples(self) -> String {
    format!("{0}-0.0 or {0}-{EXAMPLE_WP_HEX}.1", self.head())
  }
}

/// **THE ONE READER OF A WHOLE LABEL'S SHAPE** (issues I91dd457 and I78176db).
/// There were three: the gate block's own digits-only check, which refused the
/// `AC-<hex>.n` label [`regrouped`] stores for a git-style package; legacy's
/// head-only check, kept as the one stated exception for v2's foreign format;
/// and none at all on the create path, which stored any string.
///
/// **TWO QUESTIONS, TWO READERS, AND THEY DO NOT DISAGREE.** This one answers
/// whether `label` IS a label: its kind's head, a dash, a group, a dot and a
/// sequence of digits, where the group is digits of any width (hv, 2026-09-08:
/// the width is not the rule's to fix) or a git-style package's whole stored
/// id, as [`regrouped`] writes it. [`group_of`] answers which package a label
/// NAMES, so a wide group such as `AC-123.4` is a label that names no package,
/// and the gate's package check says so, as it does for a two-digit group
/// naming a package the thread does not hold. `None` for anything that is not
/// a label.
pub fn label_kind(label: &str) -> Option<LabelKind> {
  let (head, group, seq) = label_parts(label)?;
  let kind = LabelKind::ALL
    .into_iter()
    .find(|kind| kind.head() == head)?;
  let digits = |text: &str| !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit());
  let grouped = digits(group) || WpId::parse(group).is_ok();
  (grouped && digits(seq)).then_some(kind)
}

/// **THE ONE READER OF A CRITERION OR TEST LABEL IN TEXT** (ST1c2db93 P13):
/// the end of the label that starts at byte `i` of `text`, if one does, for a
/// scanner that keeps tracker ids out of published text. The shape is
/// [`label_kind`]'s, so no scanner writes a second label grammar, and a
/// git-style package's group is read as its whole stored id (`AC-5d7e9a1b.1`),
/// as an ordinal one (`AC-02.8`) is. A group in its four shown hex
/// (`AC-5d7e.1`), which an operator may type, is not a label here, because a
/// stored label carries the whole id. The label runs to the end of its
/// sequence's digits, and only a letter or digit there refuses it: `AC-02.8's`
/// is the label and an apostrophe. The caller owns the left boundary.
///
/// **AN EXAMPLE IS NOT A LABEL.** The zero family is this reader's own rule: a
/// label whose group and sequence are all zeros (`AC-0.0`, `AT-00.0`). The
/// worked package is [`is_example`]'s rule, so the label reader and the id
/// reader exempt the same worked example: a group that is [`EXAMPLE_WP_HEX`]
/// (`AC-c2e41a0b.1`), which the MCP input schemas publish. A group of zeros
/// with a sequence that is not is the thread's own group, so `AC-00.1` is a
/// label. [`is_example`] holds for `WP-00`, so it is asked only of a group
/// that is not all zeros.
pub fn label_at(text: &[u8], i: usize) -> Option<usize> {
  let run = |from: usize, keep: fn(&u8) -> bool| {
    (from..text.len())
      .find(|&at| !keep(&text[at]))
      .unwrap_or(text.len())
  };
  let dash = run(i, u8::is_ascii_alphanumeric);
  if text.get(dash) != Some(&b'-') {
    return None;
  }
  let dot = run(dash + 1, u8::is_ascii_alphanumeric);
  if text.get(dot) != Some(&b'.') {
    return None;
  }
  let end = run(dot + 1, u8::is_ascii_digit);
  if text.get(end).is_some_and(u8::is_ascii_alphanumeric) {
    return None;
  }
  let label = std::str::from_utf8(&text[i..end]).ok()?;
  label_kind(label)?;
  let (_, group, seq) = label_parts(label)?;
  let example = (all_zeros(group) && all_zeros(seq))
    || (!all_zeros(group) && is_example(&format!("{WP_HEADING_PREFIX}{group}")));
  (!example).then_some(end)
}

/// **THE ONE READER OF A BARE HEAD-NUMBER IN TEXT** (ST1c2db93, cc's meaning):
/// the end of the number behind the head that starts at byte `i` of `text`, if
/// one does, for a scanner that keeps tracker ids out of published text. The
/// calling shape is [`label_at`]'s, and the caller owns the left boundary.
///
/// A head is a [`LabelKind`]'s head and a dash (`AC-`, `AT-`), or
/// [`WP_HEADING_PREFIX`]. The number is the whole run of ASCII digits after
/// the head, and the match ends at its last digit: `WP-10a` reads `WP-10`,
/// and the digit-led front of a git-style heading, `WP-9a8b7c6d`, reads
/// `WP-9`. A letter after the head is not a number, so `AC-c2e41a0b` and
/// `WP-c2e4` give `None`.
///
/// A number of all zeros is a format example and gives `None` only when it is
/// the whole group: the text ends after it, or the byte after it is not an
/// ASCII letter or digit (`AC-0`, `AT-00.0`, `WP-00`). That is the group
/// boundary that [`label_at`] reads, and the zero rule that [`is_example`] and
/// [`label_at`] also ask. A zero-led hex front is not an example, so
/// `WP-0a8b7c6d` reads `WP-0`.
pub fn head_number_at(text: &[u8], i: usize) -> Option<usize> {
  let rest = text.get(i..)?;
  let head = LabelKind::ALL
    .iter()
    .map(|kind| kind.head().as_bytes())
    .find(|head| rest.starts_with(head) && rest.get(head.len()) == Some(&b'-'))
    .map(|head| head.len() + 1)
    .or_else(|| {
      rest
        .starts_with(WP_HEADING_PREFIX.as_bytes())
        .then_some(WP_HEADING_PREFIX.len())
    })?;
  let from = i + head;
  let end = (from..text.len())
    .find(|&at| !text[at].is_ascii_digit())
    .unwrap_or(text.len());
  let number = std::str::from_utf8(&text[from..end]).ok()?;
  let whole_group = !text.get(end).is_some_and(u8::is_ascii_alphanumeric);
  let example = whole_group && all_zeros(number);
  (!number.is_empty() && !example).then_some(end)
}

/// The one order for a thread's criterion and test ids (issue 0643): natural,
/// not lexical. An id compares run by run, a run of digits as a number and any
/// other run as text, so `AT-01.2` precedes `AT-01.10` and `AC-1.1` precedes
/// `AC-10.1`. Ids that differ only in leading zeros (`AT-1.2`, `AT-01.2`) fall
/// back to their bytes, so the order is total.
///
/// **IT IS ALSO THE ORDER A THREAD'S RECORD FILES GIVE THEIR CRITERIA AND
/// TESTS** (ST0096 WP-05; `label_order` folded in, ST1c2db93 P14): a reader's
/// order, not a stored one, since each record is a file of its own and no
/// canon byte depends on it.
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

/// The section a label sits under, in acceptance.md and in every list of a
/// thread's criteria or tests: the thread's [`THREAD_GROUP`], a package by its
/// stored id however the label spells its group (issue 0712), or an unreadable
/// group by its own text.
pub fn section_of(label: &str) -> String {
  match group_of(label) {
    Group::Thread => THREAD_GROUP.to_string(),
    Group::Package(wp) => wp.stored(),
    Group::Unreadable => group_text(label).to_string(),
  }
}

/// **WHERE A SECTION SITS IN ITS THREAD** (ST1c2db93 744e): the thread's own
/// [`THREAD_GROUP`] first, then each package's section in `packages`' order,
/// then a section that names no package of the thread. `packages` is the
/// thread's package list, which every loader puts in [`record_order`], so a
/// git-style package's section sits by when the package was made, never by
/// its hex (issue Ib383ad5's class). The rank is read from the section's text,
/// so a section has one rank, whatever its labels spell.
pub fn section_rank(section: &str, packages: &[WpId]) -> usize {
  if section == THREAD_GROUP {
    return 0;
  }
  WpId::parse(section)
    .ok()
    .and_then(|wp| packages.iter().position(|p| *p == wp))
    .map_or(packages.len() + 1, |at| at + 1)
}

/// The order of a thread's sections: by [`section_rank`], then by
/// [`id_order`] on the section's text. [`id_order`] is total, so two sections
/// are equal only when their text is.
pub fn section_order(packages: &[WpId], a: &str, b: &str) -> Ordering {
  section_rank(a, packages)
    .cmp(&section_rank(b, packages))
    .then_with(|| id_order(a, b))
}

/// **THE ONE ORDER OF A THREAD'S CRITERIA OR TESTS** (ST1c2db93 744e): by
/// [`section_order`] on each label's [`section_of`], then by [`id_order`]
/// within a section. acceptance.md and every face that lists them sort by it,
/// through `Thread::criteria_in_order` and `Thread::tests_in_order`. The
/// stored order stays [`id_order`]'s, so no canon byte moves.
pub fn thread_label_order(packages: &[WpId], a: &str, b: &str) -> Ordering {
  section_order(packages, &section_of(a), &section_of(b)).then_with(|| id_order(a, b))
}

// ---------------------------------------------------------------------------
// The order of records: ordinals by number, then hashed records by creation.
// ---------------------------------------------------------------------------

/// A record a listing orders (ST0095 WP-08): its typed id and the `origin`, the
/// creation event's ULID, it was minted from.
pub trait Ordered {
  type Id: Ord + Mintable;
  fn order_id(&self) -> &Self::Id;
  fn order_origin(&self) -> Option<&str>;
}

/// **THE ONE ORDER FOR A LISTING OF RECORDS** (ST0095 WP-08, design.md
/// "Order"): ordinals first, by number; then hashed records, by creation, which
/// is their `origin` read as text, since a ULID's front is its 48-bit
/// millisecond time. A hashed record with no `origin`, which the mint never
/// makes, sorts after every one that has one, by its id, and doctor names it
/// (WP-09); it is never dropped or refused here.
///
/// **ASCENDING ONLY** (cc's rider, 2026-11-20). A listing that runs newest first
/// reverses this at its own site and never asks a second comparator. In an
/// upgraded project every hashed record is newer than every ordinal, so the
/// reversed order is still newest first.
pub fn record_order<R: Ordered>(a: &R, b: &R) -> Ordering {
  let (x, y) = (a.order_id(), b.order_id());
  match (x.hashed(), y.hashed()) {
    (false, false) => x.cmp(y),
    (false, true) => Ordering::Less,
    (true, false) => Ordering::Greater,
    (true, true) => match (a.order_origin(), b.order_origin()) {
      (Some(p), Some(q)) => p.cmp(q).then_with(|| x.cmp(y)),
      (Some(_), None) => Ordering::Less,
      (None, Some(_)) => Ordering::Greater,
      (None, None) => x.cmp(y),
    },
  }
}

// ---------------------------------------------------------------------------
// The faces: canon (serde), the published JSON Schema, the store and GraphQL.
//
// **EACH ENCODES THE REPRESENTATION ABOVE, SO EACH LIVES HERE.** In canon every
// id travels as its stored string (ST0095 WP-06), a work package's `"04"` or
// `"c2e41a0b"` and an issue's `"0703"` or `"I3f9a2c1d4e5f6a7b"`. The ordinal
// format a project `intent upgrade` has not moved carries a work package or
// issue id as the bare number its canon field has always held (`"seq": 9`,
// `"number": 703`): that face is [`ordinal_face`], which only
// `crate::canon_format`'s frozen V3 structs name. The store's face (a TEXT
// column) and GraphQL's (a String!) are the stored string too, so a hashed id
// reads back through every face.
// ---------------------------------------------------------------------------

/// A family travels in JSON as its row spelling, [`IdKind::row`]: the `kind`
/// an `outstanding` row and a `todo` item have always carried.
impl serde::Serialize for IdKind {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(self.row())
  }
}

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
        serializer.serialize_str(&self.stored())
      }
    }

    impl<'de> serde::Deserialize<'de> for $ty {
      fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let stored = String::deserialize(deserializer)?;
        $ty::parse(&stored)
          .map_err(|e| serde::de::Error::custom(format!("`{stored}` is not {}: {e}", $what)))
      }
    }

    impl schemars::JsonSchema for $ty {
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

    impl ordinal_face::Ordinal for $ty {
      const WHAT: &'static str = $what;

      fn number(&self) -> Option<u32> {
        self.0.ordinal()
      }

      fn from_number(n: u32) -> Result<Self, IdError> {
        $ty::from_ordinal(n)
      }

      fn spelled(&self) -> String {
        self.stored()
      }
    }

    // The store holds the stored form as TEXT since schema 36, so an all-digit
    // id keeps its leading zero and a hashed one has a column to live in.
    impl rusqlite::types::ToSql for $ty {
      fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.stored()))
      }
    }

    impl rusqlite::types::FromSql for $ty {
      fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        $ty::parse(value.as_str()?).map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
      }
    }

    impl async_graphql::OutputType for $ty {
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
        let stored = self.stored();
        async move { <String as async_graphql::OutputType>::resolve(&stored, ctx, field).await }
      }
    }
  };
}

ordinal_faces!(WpId, "a work package id");
ordinal_faces!(IssueId, "an issue id");

/// The face a work package or issue id has in the ordinal canon of a project
/// `intent upgrade` has not moved: the bare number its field has always held
/// (ST0095 WP-06). Named only by `crate::canon_format`'s frozen V3 structs,
/// with `#[serde(with = "crate::id::ordinal_face")]`.
///
/// **A HASHED ID HAS NO NUMBER, AND THIS FACE REFUSES IT LOUDLY** rather than
/// writing one. The V3 encoder refuses a hashed record before it gets here.
pub mod ordinal_face {
  use super::IdError;

  /// An id this face can carry: a work package's or an issue's.
  pub trait Ordinal: Sized {
    /// What the id is, for a refusal: "a work package id".
    const WHAT: &'static str;
    /// The ordinal's number, or `None` for a hashed id.
    fn number(&self) -> Option<u32>;
    /// The ordinal `n`.
    fn from_number(n: u32) -> Result<Self, IdError>;
    /// The stored spelling, for a refusal.
    fn spelled(&self) -> String;
  }

  pub fn serialize<T: Ordinal, S: serde::Serializer>(
    id: &T,
    serializer: S,
  ) -> Result<S::Ok, S::Error> {
    match id.number() {
      Some(n) => serializer.serialize_u32(n),
      None => Err(serde::ser::Error::custom(format!(
        "{} is hashed and has no number to write in the ordinal canon",
        id.spelled()
      ))),
    }
  }

  pub fn deserialize<'de, T: Ordinal, D: serde::Deserializer<'de>>(
    deserializer: D,
  ) -> Result<T, D::Error> {
    let n = <u32 as serde::Deserialize>::deserialize(deserializer)?;
    T::from_number(n).map_err(|e| serde::de::Error::custom(format!("{n} is not {}: {e}", T::WHAT)))
  }
}

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

  /// `shown_of`'s three claims (ST0095 WP-11, cc's ask), each held by an arm:
  /// a stored id of each family reads as its own typed shown form, text that is
  /// no stored id is `None`, and an all-digit SHOWN form is not read as stored.
  #[test]
  fn shown_of_reads_a_stored_id_of_each_family_whole_and_nothing_else() {
    let thread = ThreadId::from_origin(EXAMPLE_HEX);
    let issue = IssueId::from_origin(EXAMPLE_HEX);
    let package = WpId::from_origin(EXAMPLE_WP_HEX);
    let ordinal = ThreadId::from_ordinal(56).expect("an ordinal thread");
    let address = wp_address(&thread.stored(), package);
    assert_eq!(shown_of(&thread.stored()), Some(thread.shown()));
    assert_eq!(shown_of(&ordinal.stored()), Some(ordinal.shown()));
    assert_eq!(shown_of(&issue.stored()), Some(issue.shown()));
    assert_eq!(shown_of(&address), Some(wp_address_shown(&thread, package)));
    for not_an_id in ["outstanding", "wps", "", "AC-01.1"] {
      assert_eq!(shown_of(not_an_id), None, "{not_an_id:?}");
    }
    // An all-digit short form with a leading zero, found through the waist
    // rather than spelled: the first origin in a bounded list whose thread
    // shows only digits and starts with a zero (about one in 270; the search
    // is deterministic, so whether the bound holds one is decided once).
    let digits = (0..4096)
      .map(|n| ThreadId::from_origin(&format!("all-digit origin {n}")))
      .find(|t| {
        let shown = t.shown();
        let tail = &shown[THREAD_PREFIX.len()..];
        t.is_hashed() && tail.starts_with('0') && tail.chars().all(|c| c.is_ascii_digit())
      })
      .expect("an all-digit short form with a leading zero within the bound");
    assert_eq!(
      shown_of(&digits.shown()),
      None,
      "a short form is not a stored id, however many digits it has"
    );
    assert_eq!(shown_of(&issue.shown()), None, "nor an issue's short form");
    assert_eq!(
      shown_of(&wp_address_shown(&thread, package)),
      None,
      "nor a package's address as a table prints it"
    );
    assert_eq!(shown_of(&digits.stored()), Some(digits.shown()));
    assert!(
      digits.stored().starts_with(&digits.shown()),
      "the short form is the stored id's own leading digits, a leading zero included: {} of {}",
      digits.shown(),
      digits.stored()
    );
  }
  use super::*;

  /// [`is_example`] holds the zero family and the worked hashed example, every
  /// id the form sentences spell, and no minted id (issue 0736).
  #[test]
  fn is_example_holds_the_zero_family_and_the_worked_example_only() {
    let shown = &EXAMPLE_HEX[..SHOWN_HEX];
    let wp_shown = &EXAMPLE_WP_HEX[..WP_SHOWN_HEX];
    for word in [
      format!("{THREAD_PREFIX}0000"),
      format!("{WP_HEADING_PREFIX}00"),
      format!("{THREAD_PREFIX}{shown}"),
      format!("{THREAD_PREFIX}{EXAMPLE_HEX}"),
      format!("{ISSUE_MARKER}{shown}"),
      format!("{ISSUE_MARKER}{EXAMPLE_HEX}"),
      format!("{WP_HEADING_PREFIX}{wp_shown}"),
      format!("{WP_HEADING_PREFIX}{EXAMPLE_WP_HEX}"),
    ] {
      assert!(is_example(&word), "`{word}` is a format example");
    }
    // Each form sentence spells only examples: every id `marked` finds in it.
    for form in [THREAD_ID_FORM, ISSUE_ID_FORM, WP_ADDRESS_FORM] {
      for word in form.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-')) {
        assert!(
          marked(word).is_none() || is_example(word),
          "`{word}` in `{form}` is an id that is no example"
        );
      }
    }
    for minted in [
      ThreadId::from_origin("a minted thread").shown(),
      IssueId::from_origin("a minted issue").shown(),
      WpId::from_origin("a minted package").heading(),
      ThreadId::from_ordinal(56).expect("an ordinal").shown(),
    ] {
      assert!(!is_example(&minted), "`{minted}` names a record");
    }
  }

  /// [`marked`] finds every id in each form a face prints or the store keeps,
  /// hashed and ordinal, and nothing that differs from both (issue 0736).
  #[test]
  fn marked_reads_an_id_as_a_face_prints_it_and_nothing_else() {
    let thread = ThreadId::from_origin("a hashed thread");
    let issue = IssueId::from_origin("a hashed issue");
    let package = WpId::from_origin("a hashed package");
    let ordinal_thread = ThreadId::from_ordinal(56).expect("an ordinal thread");
    let ordinal_package = WpId::from_ordinal(9).expect("an ordinal package");
    for (word, kind) in [
      (thread.shown(), IdKind::Thread),
      (thread.stored(), IdKind::Thread),
      (ordinal_thread.shown(), IdKind::Thread),
      (issue.shown(), IdKind::Issue),
      (issue.stored(), IdKind::Issue),
      (package.heading(), IdKind::WorkPackage),
      (
        format!("{WP_HEADING_PREFIX}{}", package.stored()),
        IdKind::WorkPackage,
      ),
      (ordinal_package.heading(), IdKind::WorkPackage),
    ] {
      assert_eq!(
        marked(&word),
        Some(kind),
        "`{word}` names a {}",
        kind.as_str()
      );
    }

    // A prefix between the shown and stored widths is neither form, and nor
    // is one shorter than the shown width.
    let longer = &thread.stored()[..THREAD_PREFIX.len() + SHOWN_HEX + 1];
    let shorter = &thread.shown()[..THREAD_PREFIX.len() + SHOWN_HEX - 1];
    for word in [
      shorter,
      "STAFFED",
      "Iced",
      "Ideas",
      longer,
      &thread.shown().to_ascii_uppercase(),
      &thread.shown().to_ascii_lowercase(),
      &ordinal_thread.tagged(),
      // An ordinal issue is shown as its digits alone; no face prints `I0736`.
      &format!(
        "{ISSUE_MARKER}{}",
        IssueId::from_ordinal(736).expect("an ordinal").stored()
      ),
    ] {
      assert_eq!(
        marked(word),
        None,
        "`{word}` is not an id as a face prints one"
      );
    }
  }

  #[test]
  fn the_claim_prefix_is_the_manifests_own_spelling() {
    // **THE TEST THAT MAKES THE COPY LEGITIMATE.** `CLAIM_ISSUE_PREFIX` is
    // declared here rather than read from `intentfiles`, because `intentfiles`
    // depends on this module and reaching back would invert the layering. A
    // copy a test holds identical cannot drift; a copy held by discipline does,
    // and then two doors disagree about how an issue is named.
    assert_eq!(
      CLAIM_ISSUE_PREFIX,
      crate::intentfiles::declared_key(crate::intentfiles::Sigil::Issue, ""),
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
    let git_thread = format!("{THREAD_PREFIX}{EXAMPLE_HEX}");
    let expected = [
      "ST0000".to_string(),
      git_thread.clone(),
      "ST0000/01".to_string(),
      scoped_address(&git_thread, EXAMPLE_WP_HEX),
      "ISSUE:0000".to_string(),
      format!("{CLAIM_ISSUE_PREFIX}{ISSUE_MARKER}{EXAMPLE_HEX}"),
    ];
    assert_eq!(
      forms, expected,
      "one example per kind and family the validator admits: thread, work package, issue"
    );
    for form in forms {
      assert!(
        is_claim_address(form),
        "the refusal tells the user to type `{form}` and the validator refuses it"
      );
    }
  }

  #[test]
  fn the_claim_forms_name_a_git_style_example() {
    // ST1c2db93 P4: a git-style id is what `st new` mints now, so the sentence
    // a refusal quotes names one, and the validator takes it.
    let git_issue = IssueId::parse(&format!("{ISSUE_MARKER}{EXAMPLE_HEX}"))
      .expect("the worked example is a whole hashed issue id");
    assert!(
      CLAIM_ADDRESS_FORMS.contains(&format!("`{}`", git_issue.claim())),
      "the claim forms name a git-style issue: {CLAIM_ADDRESS_FORMS}"
    );
    assert!(is_claim_address(&git_issue.claim()));
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
  fn the_named_and_scoped_readers_refuse_an_address_as_an_address() {
    // ST1c2db93 P10: as read_thread, read_issue and read_wp already do, so
    // an address is never read as a malformed id or matched as typed.
    let url = "intent:///threads/ST0001/wp/01";
    assert_eq!(read_named(url), Err(IdError::Address));
    assert_eq!(resolve_named(url, &NoRecords), Err(IdError::Address));
    assert_eq!(
      scoped(url),
      Err(refusal(url, &IdError::Address, IdKind::Thread))
    );
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
  fn a_subject_names_every_record_its_spelling_could() {
    let ids = |raw: &str| subject_ids(raw, &NoRecords).expect("a subject the door reads");
    assert_eq!(
      ids("94"),
      ["ST0094", "0094"],
      "an untagged ordinal names a thread and an issue alike"
    );
    assert_eq!(ids("0094"), ["ST0094", "0094"], "padded, the same two");
    assert_eq!(ids("ST94"), ["ST0094"], "a thread's tag narrows to it");
    assert_eq!(ids("s94"), ["ST0094"], "and so does its short tag");
    assert_eq!(ids("i94"), ["0094"], "an issue's tag narrows to it");
    assert_eq!(
      ids("56/2"),
      ["ST0056/02"],
      "a package's address names its one package"
    );
    assert_eq!(ids("ic"), ["ic"], "a node is matched as typed");
    assert_eq!(
      ids("cc"),
      ["cc"],
      "a node whose name is hex too short for a prefix is matched as typed"
    );
    let refused = |raw: &str| match subject_ids(raw, &NoRecords) {
      Err(Unresolved::Refused(refusal)) => refusal,
      other => panic!("`{raw}` should be refused as a spelling, and was {other:?}"),
    };
    assert_eq!(
      refused("i1/2").remedy,
      "`s1` names the steel thread",
      "a package under an issue's spelling is refused about its head, never matched as typed"
    );
    assert_eq!(
      ids("1/AC-01.1"),
      ["ST0001/AC-01.1"],
      "a criterion's subject is matched as the log writes it, its thread normalised"
    );
    let out_of_range = format!("56/{}", u32::MAX);
    assert!(
      refused(&out_of_range)
        .message
        .contains(&u32::MAX.to_string()),
      "a package half that spells a package id and fails is refused, naming that half"
    );
    // **FIVE DIGITS ARE A PREFIX NOW** (hv decision 70), so a thread or an
    // issue spelled with them is looked for, and here names nothing.
    assert_eq!(
      subject_ids("ST99999/02", &NoRecords),
      Err(Unresolved::Nothing(vec![IdKind::Thread]))
    );
    assert_eq!(
      subject_ids("i99999", &NoRecords),
      Err(Unresolved::Nothing(vec![IdKind::Issue]))
    );
  }

  #[test]
  fn a_refusal_quotes_the_form_or_the_spelling_that_works() {
    let thread = IdKind::Thread;
    assert_eq!(
      refusal("foo", &IdError::NotAnId, thread).remedy,
      THREAD_ID_FORM
    );
    assert_eq!(
      refusal("ST99999", &IdError::OutOfRange, IdKind::Issue).remedy,
      ISSUE_ID_FORM
    );
    let wrong = IdError::WrongCollection {
      named: IdKind::Issue,
    };
    assert_eq!(
      refusal("i59", &wrong, thread).remedy,
      "`s59` names the steel thread"
    );
    let ambiguous = normalise_id("59").expect_err("an untagged number names both collections");
    assert_eq!(
      refusal("59", &ambiguous, thread).remedy,
      "`s59` names the steel thread, `i59` the issue"
    );
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

  /// A creation event id, as `event::next_id` mints them: the origin the hashed
  /// family's tests derive every expectation from, through the one hasher.
  const ORIGIN: &str = "01M7SQ544Q2FK5BNMYAMXV3Z4Z";
  const OTHER_ORIGIN: &str = "01M7SQ544Q2FK5BNMYAMXV3Z50";

  #[test]
  fn a_hashed_id_is_its_marker_and_the_front_of_its_origins_sha256() {
    let digest = crate::model::sha256_hex(ORIGIN.as_bytes());
    let front = |n: usize| digest.chars().take(n).collect::<String>();

    let thread = ThreadId::from_origin(ORIGIN);
    assert_eq!(
      thread.stored(),
      format!("{THREAD_PREFIX}{}", front(HASH_HEX))
    );
    assert_eq!(
      thread.shown(),
      format!("{THREAD_PREFIX}{}", front(SHOWN_HEX))
    );
    assert_eq!(ThreadId::parse(&thread.stored()), Ok(thread.clone()));

    let issue = IssueId::from_origin(ORIGIN);
    assert_eq!(issue.stored(), format!("{ISSUE_MARKER}{}", front(HASH_HEX)));
    assert_eq!(issue.shown(), format!("{ISSUE_MARKER}{}", front(SHOWN_HEX)));
    assert_eq!(IssueId::parse(&issue.stored()), Ok(issue));

    let wp = WpId::from_origin(ORIGIN);
    assert_eq!(wp.stored(), front(WP_HASH_HEX));
    assert_eq!(wp.shown(), front(WP_SHOWN_HEX));
    assert_eq!(
      wp.heading(),
      format!("{WP_HEADING_PREFIX}{}", front(WP_SHOWN_HEX))
    );
    assert_eq!(WpId::parse(&wp.stored()), Ok(wp));
  }

  #[test]
  fn an_id_answers_to_the_origin_it_was_minted_from_and_to_no_other() {
    assert!(ThreadId::from_origin(ORIGIN).derives_from(ORIGIN));
    assert!(!ThreadId::from_origin(ORIGIN).derives_from(OTHER_ORIGIN));
    assert!(IssueId::from_origin(ORIGIN).derives_from(ORIGIN));
    assert!(WpId::from_origin(ORIGIN).derives_from(ORIGIN));
    let ordinal = ThreadId::from_ordinal(46).expect("an ordinal that fits");
    assert!(
      !ordinal.derives_from(ORIGIN),
      "an ordinal was minted from no event, so no origin is its own"
    );
  }

  #[test]
  fn no_stored_spelling_is_both_an_ordinal_and_a_hash() {
    let thread = ThreadId::from_origin(ORIGIN);
    assert!(thread.is_hashed());
    assert!(!ThreadId::parse("ST0046").expect("an ordinal").is_hashed());
    assert!(!IssueId::parse("0703").expect("an ordinal").is_hashed());
    assert!(!WpId::parse("09").expect("an ordinal").is_hashed());
    // A stored form is compared whole, so its case is part of it: lowercase hex
    // is a stored thread id, and the same digits in uppercase are not.
    assert!(ThreadId::parse("STabcdef0123456789").is_ok());
    assert_eq!(ThreadId::parse("STABCDEF0123456789"), Err(IdError::NotAnId));
    // Seven hex digits are the shown form, not a stored one.
    assert_eq!(ThreadId::parse(&thread.shown()), Err(IdError::NotAnId));
  }

  #[test]
  fn every_ordinal_orders_before_every_hash() {
    let last_ordinal = ThreadId::from_ordinal(9999).expect("the last ordinal");
    assert!(last_ordinal < ThreadId::from_origin(ORIGIN));
    let last_issue = IssueId::from_ordinal(9999).expect("the last ordinal");
    assert!(last_issue < IssueId::from_origin(ORIGIN));
  }

  #[test]
  fn a_door_reads_a_whole_hashed_spelling_in_any_case_and_names_its_collection() {
    let thread = ThreadId::from_origin(ORIGIN);
    let issue = IssueId::from_origin(ORIGIN);
    assert_eq!(normalise_thread_id(&thread.stored()), Ok(thread.clone()));
    assert_eq!(
      normalise_thread_id(&thread.stored().to_ascii_uppercase()),
      Ok(thread.clone())
    );
    assert_eq!(normalise_issue_id(&issue.stored()), Ok(issue));
    assert_eq!(
      normalise_thread_id(&issue.stored()),
      Err(IdError::WrongCollection {
        named: IdKind::Issue
      })
    );
    assert_eq!(normalise_id(&thread.stored()), Ok(Named::Thread(thread)));
    let wp = WpId::from_origin(ORIGIN);
    assert_eq!(normalise_wp_id(&wp.stored().to_ascii_uppercase()), Ok(wp));
    // The ordinal spellings keep their meaning beside the hashed ones.
    assert_eq!(normalise_thread_id("46"), ThreadId::from_ordinal(46));
  }

  /// AT-14.1 (ST0095 AC-14.1): each surface reads a family back from the one spelling it
  /// writes, and JSON carries the row spelling.
  #[test]
  fn each_family_reads_back_from_its_own_spelling_on_every_surface() {
    for kind in IdKind::ALL {
      assert_eq!(IdKind::of_wire(kind.wire()), Some(kind), "{kind:?}");
      assert_eq!(IdKind::of_row(kind.row()), Some(kind), "{kind:?}");
      let json = serde_json::to_value(kind).expect("a family serialises");
      assert_eq!(json, serde_json::Value::from(kind.row()), "{kind:?}");
    }
  }

  /// **WHAT IS PERSISTED IS NEVER RESPELLED**: an event file's subject type and
  /// a store section's owner keep the spellings they were written with, so a
  /// reader of an old file or row still finds its family.
  #[test]
  fn the_persisted_spellings_are_the_ones_always_written() {
    let wire: Vec<&str> = IdKind::ALL.iter().map(IdKind::wire).collect();
    assert_eq!(wire, ["thread", "wp", "issue"]);
    let row: Vec<&str> = IdKind::ALL.iter().map(IdKind::row).collect();
    assert_eq!(row, ["thread", "work-package", "issue"]);
    assert_eq!(IdKind::of_wire("work-package"), None);
    assert_eq!(IdKind::of_row("wp"), None);
  }

  #[test]
  fn a_work_package_has_no_operator_tag_and_no_number_respells_to_one() {
    assert_eq!(IdKind::WorkPackage.tag(), None);
    assert_eq!(respelled_as("s21", IdKind::WorkPackage), None);
    assert_eq!(respelled_as("s21", IdKind::Issue).as_deref(), Some("i21"));
  }

  /// The scope a test holds: ids it lists, and no store.
  struct Held {
    threads: Vec<ThreadId>,
    issues: Vec<IssueId>,
    wps: Vec<(ThreadId, WpId)>,
  }

  impl IdScope for Held {
    fn threads(&self) -> Vec<ThreadId> {
      self.threads.clone()
    }
    fn issues(&self) -> Vec<IssueId> {
      self.issues.clone()
    }
    fn wps(&self, thread: &ThreadId) -> Vec<WpId> {
      self
        .wps
        .iter()
        .filter(|(owner, _)| owner == thread)
        .map(|(_, wp)| *wp)
        .collect()
    }
  }

  /// A thread whose stored hex is `hex`, padded with zeros to the stored width.
  fn thread_of_hex(hex: &str) -> ThreadId {
    ThreadId::parse(&format!("{THREAD_PREFIX}{hex:0<HASH_HEX$}")).expect("a stored thread id")
  }

  #[test]
  fn every_shown_form_is_a_prefix_a_door_takes() {
    let thread = ThreadId::from_origin(ORIGIN);
    let wp = WpId::from_origin(ORIGIN);
    let scope = Held {
      threads: vec![thread.clone()],
      issues: Vec::new(),
      wps: vec![(thread.clone(), wp)],
    };
    assert_eq!(
      resolve_thread(&thread.shown(), &scope),
      Ok(Found::One(thread.clone()))
    );
    assert_eq!(
      resolve_wp(&thread, &wp.heading(), &scope),
      Ok(Found::One(wp))
    );
    assert_eq!(resolve_wp(&thread, &wp.shown(), &scope), Ok(Found::One(wp)));
  }

  #[test]
  fn the_floor_is_the_minimum_and_one_past_the_ordinal_for_digits_alone() {
    assert_eq!(prefix_floor("3f9a", THREAD_DIGITS), MIN_PREFIX_HEX);
    assert_eq!(
      prefix_floor("1234", THREAD_DIGITS),
      MIN_PREFIX_HEX.max(THREAD_DIGITS + 1)
    );
    assert_eq!(
      prefix_floor("1234", WP_DIGITS),
      MIN_PREFIX_HEX.max(WP_DIGITS + 1)
    );
    let short = "f".repeat(MIN_PREFIX_HEX - 1);
    assert_eq!(
      read_thread(&short),
      Err(IdError::TooShort {
        floor: MIN_PREFIX_HEX
      })
    );
    let wp_digits = "1".repeat(WP_DIGITS + 1);
    assert_eq!(
      read_wp(&wp_digits),
      Err(IdError::TooShort {
        floor: prefix_floor(&wp_digits, WP_DIGITS)
      }),
      "longer than a package ordinal, shorter than a prefix"
    );
  }

  #[test]
  fn an_ordinal_shaped_spelling_never_reaches_a_hash_that_begins_with_its_digits() {
    let digits = "1".repeat(THREAD_DIGITS);
    let hashed = thread_of_hex(&"1".repeat(THREAD_DIGITS + 1));
    let scope = Held {
      threads: vec![hashed.clone()],
      issues: Vec::new(),
      wps: Vec::new(),
    };
    for typed in [digits.clone(), format!("{THREAD_PREFIX}{digits}")] {
      assert_eq!(
        resolve_thread(&typed, &scope),
        Ok(Found::One(
          normalise_thread_id(&digits).expect("an ordinal")
        )),
        "`{typed}` is the ordinal"
      );
    }
    let longer = "1".repeat(THREAD_DIGITS + 1);
    assert_eq!(
      resolve_thread(&longer, &scope),
      Ok(Found::One(hashed)),
      "one digit more is the hash's prefix"
    );
  }

  #[test]
  fn a_prefix_names_one_several_or_none_and_never_picks() {
    let front = "3f9a";
    let (one, two) = (
      thread_of_hex(&format!("{front}1")),
      thread_of_hex(&format!("{front}2")),
    );
    let scope = Held {
      threads: vec![two.clone(), one.clone()],
      issues: vec![IssueId::from_origin(ORIGIN)],
      wps: Vec::new(),
    };
    assert_eq!(
      resolve_thread(front, &scope),
      Ok(Found::Many(vec![one.clone(), two.clone()])),
      "every candidate, in the family order"
    );
    assert_eq!(
      resolve_thread(&format!("{front}1").to_ascii_uppercase(), &scope),
      Ok(Found::One(one.clone()))
    );
    assert_eq!(resolve_thread("abcd", &scope), Ok(Found::Nothing));
    assert_eq!(
      read_thread(&format!("{ISSUE_MARKER}{front}")),
      Err(IdError::WrongCollection {
        named: IdKind::Issue
      }),
      "an issue's marker on hex names the other collection"
    );
  }

  #[test]
  fn a_package_prefix_is_completed_only_among_its_own_threads_packages() {
    let (mine, other) = (
      ThreadId::from_origin(ORIGIN),
      ThreadId::from_origin(OTHER_ORIGIN),
    );
    let wp = WpId::from_origin(ORIGIN);
    let scope = Held {
      threads: vec![mine.clone(), other.clone()],
      issues: Vec::new(),
      wps: vec![(mine.clone(), wp)],
    };
    assert_eq!(resolve_wp(&mine, &wp.shown(), &scope), Ok(Found::One(wp)));
    assert_eq!(resolve_wp(&other, &wp.shown(), &scope), Ok(Found::Nothing));
  }

  #[test]
  fn a_regrouped_label_carries_the_whole_package_id() {
    let wp = WpId::from_origin(ORIGIN);
    assert_eq!(
      regrouped(&format!("AC-{}.3", wp.shown()), wp),
      format!("AC-{}.3", wp.stored())
    );
    assert_eq!(regrouped("AC", wp), "AC", "a label with no group is left");
  }

  /// A record for [`record_order`]: a thread id and the origin it was minted
  /// from, or none.
  struct Rec(ThreadId, Option<&'static str>);

  impl Ordered for Rec {
    type Id = ThreadId;
    fn order_id(&self) -> &ThreadId {
      &self.0
    }
    fn order_origin(&self) -> Option<&str> {
      self.1
    }
  }

  /// AT-08.1 (ST0095 AC-08.1): ordinals by number, then hashed records by
  /// creation, then a hashed record with no origin. The two hashed records are
  /// chosen so their hashes run against their creation, so an order by hash
  /// fails here. A descending listing reverses this at its own site.
  #[test]
  fn records_order_ordinals_by_number_then_hashed_records_by_creation() {
    let (early, late) = testkit::against_creation(ThreadId::from_origin);
    let unborn = testkit::CREATION_ORIGINS
      .iter()
      .copied()
      .find(|o| *o != early && *o != late)
      .expect("a third origin");
    let ordinal = |n| ThreadId::from_ordinal(n).expect("an ordinal");
    let expected = vec![
      ordinal(2),
      ordinal(10),
      ThreadId::from_origin(early),
      ThreadId::from_origin(late),
      ThreadId::from_origin(unborn),
    ];
    let mut records = [
      Rec(ThreadId::from_origin(unborn), None),
      Rec(ThreadId::from_origin(late), Some(late)),
      Rec(ordinal(10), None),
      Rec(ThreadId::from_origin(early), Some(early)),
      Rec(ordinal(2), None),
    ];
    records.sort_by(record_order);
    let ascending: Vec<ThreadId> = records.iter().map(|r| r.0.clone()).collect();
    assert_eq!(
      ascending, expected,
      "ordinals by number, hashed records by creation, one with no origin last"
    );
  }

  /// AT-08.2 (ST0095 AC-08.2): for a project that holds only ordinals, the one
  /// order is the order its ids had before WP-08, by number, origin or none, so
  /// every listing and every view built from one renders as it did.
  #[test]
  fn ordinals_alone_keep_the_order_their_numbers_gave() {
    let numbers = [1_u32, 2, 9, 10, 59, 100, 9999];
    let origins = [None, Some("01M7SQ544Q2FK5BNMYAMXV3Z4Z")];
    for (a, b) in numbers
      .iter()
      .flat_map(|a| numbers.iter().map(move |b| (*a, *b)))
    {
      for (oa, ob) in origins
        .iter()
        .flat_map(|x| origins.iter().map(move |y| (*x, *y)))
      {
        let x = Rec(ThreadId::from_ordinal(a).expect("an ordinal"), oa);
        let y = Rec(ThreadId::from_ordinal(b).expect("an ordinal"), ob);
        assert_eq!(
          record_order(&x, &y),
          a.cmp(&b),
          "ordinals {a} and {b} order by number, whatever origin they carry"
        );
      }
    }
  }
}
