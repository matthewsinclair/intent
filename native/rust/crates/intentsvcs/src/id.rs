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

/// The one sentence of a work package's address, as [`THREAD_ID_FORM`] is a
/// thread id's (cc's meaning, 2026-11-20): its thread's id, a slash and its own
/// id. The stored address carries both ids whole. Its example names only the
/// family every door accepts today; the hashed family joins all three form
/// sentences in the change that teaches the doors to read it (ST0095 WP-07).
pub const WP_ADDRESS_FORM: &str =
  "a work package's address is its thread's id, a slash and its own id, eg ST0000/01";

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
  pub fn as_str(&self) -> &'static str {
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

  /// Whether this is a hashed id minted from `origin`, which doctor asks of a
  /// record's `origin` field. An ordinal has no origin to answer to.
  pub fn derives_from(&self, origin: &str) -> bool {
    self.form.is_hashed() && Self::from_origin(origin) == *self
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
    self.0.is_hashed() && Self::from_origin(origin) == *self
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
    self.0.is_hashed() && Self::from_origin(origin) == *self
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
}

impl Mintable for ThreadId {
  const WHAT: &'static str = "steel thread";
  fn minted_from(origin: &str) -> Self {
    Self::from_origin(origin)
  }
  fn short(&self) -> String {
    self.shown()
  }
}

impl Mintable for WpId {
  const WHAT: &'static str = "work package";
  fn minted_from(origin: &str) -> Self {
    Self::from_origin(origin)
  }
  fn short(&self) -> String {
    self.shown()
  }
}

impl Mintable for IssueId {
  const WHAT: &'static str = "issue";
  fn minted_from(origin: &str) -> Self {
    Self::from_origin(origin)
  }
  fn short(&self) -> String {
    self.shown()
  }
}

/// Why the one mint made no id. Nothing was written: the mint runs inside the
/// verb that asked, before any write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintError {
  /// The id a draw derived is one this clone already holds. At sixty-four bits
  /// that is a broken draw, never chance.
  Taken { what: &'static str, id: String },
  /// Every one of [`MINT_DRAWS`] draws gave a short form a visible record of
  /// the kind already shows.
  ShortFormsShared { what: &'static str, draws: usize },
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
    }
  }
}

impl std::error::Error for MintError {}

impl crate::remedy::Remedy for MintError {
  fn remedy(&self) -> String {
    "nothing was written. A mint draws a fresh creation event for every id, so this is a defect \
     in Intent, not in the project: report it with the command that met it"
      .to_string()
  }
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
/// after [`MINT_DRAWS`] draws rather than loop.
pub fn mint<I: Mintable>(
  visible: &[I],
  mut draw: impl FnMut() -> String,
) -> Result<(String, I), MintError> {
  for _ in 0..MINT_DRAWS {
    let origin = draw();
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
  /// The tail is a package id: a work package's address.
  Package(ThreadId, WpId),
  /// The tail is no package id, eg `AC-01.1` or an attachment's path: another
  /// thing under the thread, as the event log's subjects spell it.
  Under(ThreadId, String),
}

/// **THE ONE READER OF `<thread>/<tail>`**, so every door that takes one reads
/// the head through the thread door and decides the tail one way. `None` when
/// the text is not that shape: no slash, or a head that spells no id at all.
///
/// **A PART THAT BEGINS AS AN ID AND IS NOT ONE IS REFUSED**, worded for the
/// part that failed: a head that names an issue or is out of range, or a tail
/// that spells a package id and is out of range. A tail that spells no package
/// id is no refusal: it is [`Scoped::Under`], because the event log writes a
/// criterion's, a test's and an attachment's subjects in this shape. Pure: no
/// store is asked.
pub fn scoped(raw: &str) -> Result<Option<Scoped>, Refusal> {
  let Some((head, tail)) = split_wp_address(raw) else {
    return Ok(None);
  };
  let thread = match normalise_thread_id(head) {
    Ok(thread) => thread,
    Err(IdError::NotAnId) => return Ok(None),
    Err(e) => return Err(refusal(head, &e, IdKind::Thread)),
  };
  match normalise_wp_id(tail) {
    Ok(wp) => Ok(Some(Scoped::Package(thread, wp))),
    Err(IdError::NotAnId) => Ok(Some(Scoped::Under(thread, tail.to_string()))),
    Err(_) => Err(not_a_package(raw)),
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
/// nothing in silence (`i1/2`, `ST99999/02`). Pure: no store is asked.
pub fn subject_ids(raw: &str) -> Result<Vec<String>, Refusal> {
  match scoped(raw)? {
    Some(Scoped::Package(thread, wp)) => return Ok(vec![wp_address(thread.as_str(), wp)]),
    Some(Scoped::Under(thread, tail)) => return Ok(vec![format!("{}/{tail}", thread.stored())]),
    None => {}
  }
  match normalise_id(raw) {
    Ok(Named::Thread(id)) => Ok(vec![id.stored()]),
    Ok(Named::Issue(id)) => Ok(vec![id.stored()]),
    Err(IdError::Ambiguous { thread, issue }) => Ok(vec![thread.stored(), issue.stored()]),
    Err(IdError::NotAnId) => Ok(vec![raw.to_string()]),
    Err(e) => {
      let wanted = spelling(raw)
        .ok()
        .and_then(|s| s.tag)
        .unwrap_or(IdKind::Thread);
      Err(refusal(raw, &e, wanted))
    }
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
  fn a_subject_names_every_record_its_spelling_could() {
    let ids = |raw: &str| subject_ids(raw).expect("a subject the door reads");
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
    let refused = |raw: &str| subject_ids(raw).expect_err("a subject the door refuses");
    assert_eq!(
      refused("i1/2").remedy,
      "`s1` names the steel thread",
      "a package under an issue's spelling is refused about its head, never matched as typed"
    );
    assert_eq!(
      refused("ST99999/02").message,
      "`ST99999` is out of range for a steel thread id"
    );
    assert_eq!(
      refused("i99999").message,
      "`i99999` is out of range for an issue id",
      "worded for the kind the tag reaches for"
    );
    assert_eq!(
      ids("1/AC-01.1"),
      ["ST0001/AC-01.1"],
      "a criterion's subject is matched as the log writes it, its thread normalised"
    );
    let out_of_range = format!("56/{}", u32::MAX);
    assert_eq!(
      refused(&out_of_range),
      not_a_package(&out_of_range),
      "a package half that spells a package id and fails is refused"
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
}
