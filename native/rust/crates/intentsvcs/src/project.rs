//! The project: a root directory, its config, and the canonical location of
//! every artefact under it.
//!
//! **This is the one place paths are decided.** v2's defect class was that
//! every reader reimplemented parsing and path resolution, so every reader
//! answered confidently from partial evidence; the fix is not "be careful", it
//! is having exactly one module that can answer "where does a thread live".
//! Nothing else in intentsvcs joins a path by hand.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::model::Attachment;

/// The largest attachment canon will carry, in bytes.
///
/// **THIS REPLACED A THREE-ENTRY EXTENSION ALLOWLIST, AND THE ALLOWLIST WAS THE
/// DEFECT.** `["md", "txt", "sh"]` decided what could be carried by asking the
/// FILENAME, one step above the place that decides text-vs-opaque by DECODING.
/// So the opaque-attachment machinery -- `Attachment::opaque`, the `blob`
/// column, the `CHECK ((text IS NULL) <> (blob IS NULL))` that makes the two
/// forms exclusive -- was complete and **unreachable**: a `.pdf` was classified
/// out before anything opened it. Both halves were individually right and the
/// composition was the defect.
///
/// **And it was not only binaries.** Measured across the fleet's `intent/st`
/// directories: this project's own tree carries 196 `.tap` files and 2 `.tsv`,
/// Conflab 20 `.json` and 10 `.lensmd`, Lamplight 6 `.cli` and 2 `.py` -- every
/// one of them valid UTF-8, and every one excluded, because the gate asked the
/// extension rather than the bytes.
///
/// **A SIZE IS THE HONEST GATE BECAUSE SIZE IS THE ACTUAL COST.** The committed
/// canon extract carries attachment bytes INLINE, so an opaque row is base64 in
/// a versioned JSON file. Measured on Lamplight, the fleet's worst case: of 35
/// files reaching the classifier, 8 sit above this cap (14.8 MiB of
/// `ST0167/WP/01/screenshots/*.png`, which would be roughly 20 MiB of base64 in
/// committed canon) and 27 sit below it (3.6 MiB, carried).
///
/// **1 MiB IS A PLACEHOLDER AND IS DELIBERATELY NOT TUNED** (vc, 2026-08-26).
/// It stands until attachment bytes live outside the extract, content-addressed;
/// a finely-chosen number would read as a considered limit rather than as the
/// stopgap it is. **Over the cap is a REFUSAL, never a deletion** -- the file is
/// reported by name with its size and left exactly where it is.
///
/// **One consequence carried openly rather than solved: a mode bit does not
/// survive.** `text` is content, so an executable written back from the store
/// arrives without its `+x`. A script that has to be `chmod +x` is recoverable
/// and a script nobody kept is not.
pub const ATTACHMENT_CAP_BYTES: u64 = 1024 * 1024;

/// Whether an attachment of this size is carried into canon.
///
/// **ONE HOME, AND THAT IS THE WHOLE POINT OF IT BEING A FUNCTION RATHER THAN A
/// COMPARISON WRITTEN TWICE.** Two callers need this answer: the carrier
/// ([`Project::collect_attachments`]) and the surface that reports what was NOT
/// carried (`doctor`). If those two ever disagree, a file is refused by one and
/// unreported by the other -- which is precisely the silent gap the report
/// exists to close, arriving through the door built to prevent it.
///
/// **Inclusive at the boundary**: a file of exactly [`ATTACHMENT_CAP_BYTES`] is
/// carried. A cap is a maximum, and an off-by-one here is invisible to any test
/// that only tries one oversized file.
pub fn within_attachment_cap(bytes: u64) -> bool {
  bytes <= ATTACHMENT_CAP_BYTES
}

/// Why a string does NOT name a place inside the thread.
///
/// **A KIND RATHER THAN A SENTENCE, because the remedy has to differ per
/// cause.** `FacadeError::remedy`'s own contract is that no two variants share
/// a remedy text -- *a remedy that fits two different causes is telling the
/// operator to guess which one they hit* (AC-04.4). Returning prose from here
/// would have forced the write door onto a shared variant, and the first build
/// of this check did exactly that: a correct refusal about an attachment path
/// carrying the remedy *`PUT` json to a caller-assigned id*, which is right for
/// a different caller entirely. **That is `0268`'s class -- a correct message
/// printed for a situation it was not written for -- arriving inside the fix
/// for `0262`.** The kind is what keeps the two apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathFault {
  /// No path at all.
  Empty,
  /// Absolute, so it names a place on the machine rather than in the thread.
  Absolute,
  /// Climbs out of the thread with `..`, whether or not it climbs back.
  Escapes,
  /// Relative to the REPOSITORY -- the shape `0262` is filed on.
  RepoRelative {
    /// The path as it should have been written.
    corrected: String,
    /// What the given path would actually have named, which is what makes it
    /// obviously wrong when read.
    would_name: String,
  },
  /// Resolves inside the thread and is not written plainly, so canon would hold
  /// it as a second spelling of one file.
  Unnormalised {
    /// The plain spelling.
    corrected: String,
  },
}

impl PathFault {
  /// The sentence naming what is wrong with THIS path. The remedy is a separate
  /// question with a separate home -- see the type's own doc.
  /// `home` is the thread's directory relative to the project root, as
  /// [`Project::relative`] spells it, so the project's own intent directory is
  /// the one named (issue 0729).
  pub fn why(&self, shown: &str, home: &str) -> String {
    let home = Path::new(home);
    match self {
      Self::Empty => {
        "an attachment path is empty, so it names no file inside the thread".to_string()
      }
      Self::Absolute => format!(
        "`{shown}` is absolute, and an attachment path is relative to the thread -- it names a place on the machine rather than a place in {}",
        home.display()
      ),
      Self::Escapes => format!(
        "`{shown}` climbs out of the thread with `..`, and an attachment lives inside {}",
        home.display()
      ),
      Self::RepoRelative { would_name, .. } => format!(
        "`{shown}` is relative to the REPOSITORY and this path is relative to the THREAD, so it would name `{would_name}`"
      ),
      Self::Unnormalised { .. } => format!(
        "`{shown}` is not written plainly, so canon would hold it as a second spelling of one file"
      ),
    }
  }
}

/// Whether this string names a place INSIDE the thread, and why not when it
/// does not.
///
/// **THE QUESTION IS THE ADDRESS, NOT THE FILE.** [`Project::classify`] answers
/// *whose file is this* and deliberately consults nothing about the path's
/// shape; [`within_attachment_cap`] answers *do the bytes fit*. This is the
/// third of that family: **does this path name somewhere in the thread at all**.
/// It is a function rather than a check written at the write door for the
/// reason `within_attachment_cap` gives above -- every door that writes an
/// attachment has to refuse the same set, and two copies drift into one door
/// refusing what another accepts.
///
/// # Why the repo-relative form is the case worth naming
///
/// The commit gate that refuses an uncarried attachment prints
/// `intent st attach <ST> <rel-path> --from <file>`, and *relative* to a reader
/// means relative to the REPOSITORY -- it is the path `git status` has just
/// printed at them, in hand at the moment they are asked for it. This verb
/// wants it relative to the THREAD and said so nowhere. Before this check the
/// other spelling returned `ok:` at rc=0 and minted a SECOND attachment row for
/// a file that already had one: resolving nowhere, removed by no verb, reported
/// by no `doctor` (`0262`).
///
/// # Four more the same door let through, every one driven
///
/// Measured on `da5919e8` before this existed: an empty path, an absolute one,
/// one climbing out with `..`, and the unnormalised `./x` and `a/../x` forms
/// were ALL accepted at rc=0 and written into canon -- eight paths offered,
/// eight rows minted. They are refused here rather than in five places.
///
/// **Blast radius, measured rather than assumed**: over this estate's 369
/// distinct attachment paths at that commit, **zero** are refused by this rule.
/// It is additive, and it is not a migration.
///
/// # The one thing this deliberately gives up
///
/// A thread could legitimately hold its own `intent/st/...` subdirectory, and
/// that path is now unaddressable. It is refused rather than normalised because
/// normalising silently REINTERPRETS an address instead of correcting the
/// person who wrote it, and because an accepted-and-wrong write is the failure
/// this whole check exists to stop. The `why` names what the path would have
/// meant, so the loss is visible rather than mysterious.
///
/// `home` is the thread's directory relative to the project root, as
/// [`Project::relative`] spells it (issue 0729).
pub fn attachment_path_fault(rel: &Path, home: &Path) -> Option<PathFault> {
  use std::path::Component;

  let shown = rel.display().to_string();
  if shown.is_empty() {
    return Some(PathFault::Empty);
  }

  for c in rel.components() {
    match c {
      Component::RootDir | Component::Prefix(_) => return Some(PathFault::Absolute),
      Component::ParentDir => return Some(PathFault::Escapes),
      _ => {}
    }
  }

  // The repo-relative form is checked before normalisation so the better
  // message wins on a path that is both. The threads' directory is `home`'s
  // parent, the project's own and never a spelled `intent/st` (issue 0729).
  let threads = home.parent().unwrap_or(home);
  if rel.starts_with(threads) {
    let inside: PathBuf = rel
      .components()
      .skip(threads.components().count() + 1)
      .collect();
    let corrected = if inside.as_os_str().is_empty() {
      "<the path inside the thread>".to_string()
    } else {
      inside.display().to_string()
    };
    return Some(PathFault::RepoRelative {
      corrected,
      would_name: home.join(&shown).display().to_string(),
    });
  }

  // One rule for every unnormalised spelling -- `./x`, `a/./b`, `a//b`, a
  // trailing separator. Canon stores the STRING, so two spellings of one file
  // are two rows, which is `0262`'s defect reached by a quieter route.
  let normalised: PathBuf = rel
    .components()
    .filter(|c| !matches!(c, Component::CurDir))
    .collect();
  if normalised.as_os_str() != rel.as_os_str() {
    return Some(PathFault::Unnormalised {
      corrected: normalised.display().to_string(),
    });
  }

  None
}

/// What a file sitting under a thread's directory is.
///
/// These partition the directory: every file is exactly one. **`doctor` still
/// reports a remainder by path, and that property is what the whole scheme
/// rests on** -- a disk that becomes optional destroys whatever nothing said
/// was uncovered. What changed is the remainder's MEANING: it used to be "an
/// extension we do not carry" and it is now "over the cap", reported with the
/// byte count. It will be much shorter, and it is now a statement about the
/// files rather than about a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadFile {
  /// Rendered from the model: the thread's cover, its acceptance contract, a
  /// work package's cover.
  GeneratedView,
  /// `thread.json` -- the committed canon itself.
  Canon,
  /// **ONE FILE WITH THE THREAD'S OWN, SPELLED OTHERWISE** (issue 0719): a
  /// name a disk that folds case (macOS's default) stores as a generated view
  /// or the canon file, eg `Acceptance.md` or `wp/01/info.md`. `of` is the
  /// exact spelling of that file. No door carries, renames or claims it: each
  /// refuses it by name, and `doctor` counts it.
  CaseVariant { of: PathBuf },
  /// Anything a human put here. Carried verbatim when it fits under
  /// [`ATTACHMENT_CAP_BYTES`]; reported by name with its size when it does not.
  ///
  /// **There is no fourth variant, and the one that used to be here retired
  /// with the extension allowlist that created it.** `Unattached` meant "an
  /// extension we do not carry", which was never a fact about the FILE -- only
  /// about a list. Whether the bytes can be carried is a question about their
  /// SIZE, and size is not knowable from a path, so it is asked by
  /// [`within_attachment_cap`] where the bytes are, not here.
  Attachment,
}

/// The `backup` block (D35). Absent in every config written before the schedule
/// became a key, hence the default.
///
/// **THE PERIOD IS CONFIGURATION AND THE REPORT IS NOT.** hv, 2026-08-26, on
/// finding `doctor` measuring against a schedule nobody had chosen: "Who came up
/// with a rule that the intent db had to be backed up every 24h?" and "this
/// _has_ to be a configuration param" and "But it can't be hardcoded" -- then,
/// on whether the check itself should be switchable, "I don't want it turned
/// off." So the period moved into the file and nothing moved the report out of
/// it.
///
/// That is the line `surface/dispatch-table.md` had already drawn at
/// `deliberately_not_keys.1`: ANY SWITCH THAT SILENCES BACKUP FAILURE is not a
/// key, because this is the backup of the durable SSOT and giving silence a
/// supported name manufactures the failure it would be reporting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupConfig {
  /// `keys.0`, default true. Whether the DAEMON takes this project's scheduled
  /// backups.
  ///
  /// **IT GATES THE SWEEP AND NOTHING ELSE, AND THAT IS THE RATIFIED SHAPE
  /// RATHER THAN AN IMPLEMENTATION CHOICE.** `intent backup` still takes one
  /// when it is typed, `intent doctor` still reports a stale backup, and
  /// `BackupFailing` still fires. So this expresses *do not take them for me*
  /// and never *do not tell me they are old*.
  ///
  /// **THE APPARENT CONFLICT WITH hv WAS A HOMONYM** (vc, 2026-08-30). hv said
  /// *I don't want it turned off* on 2026-08-26, and `keys.0` says a backup can
  /// be turned off; both sentences carry `turn off` and `backup` and they are
  /// two different switches. There is deliberately no setting that silences a
  /// STALE BACKUP -- **the combination is the design: a preference you can
  /// express and a consequence you cannot hide.**
  ///
  /// Read it only through [`crate::backup::due`], which is the one decision
  /// this gates.
  #[serde(default = "default_backup_enabled")]
  pub enabled: bool,
  /// How often a snapshot is expected. Default `daily`.
  ///
  /// **A word or a whole-number duration, and nothing else** (D35, hv,
  /// 2026-08-15; widened by ST0080; `surface/dispatch-table.md` `keys.1`):
  /// `hourly`, `daily`, `weekly`, `<N>h` or `<N>d`. A value outside that is
  /// reported rather than rounded to the nearest plausible period. Read it only
  /// through [`crate::backup::schedule`], which is what enforces that.
  #[serde(default = "default_backup_schedule")]
  pub schedule: String,
  /// The `backup.retain` tiers (D35, `dispatch-table.md` `keys.2`-`keys.4`).
  ///
  /// **TYPED HERE BECAUSE IT WAS BEING READ OUT OF `Config::extra` AND COULD
  /// NOT BE** (cc, 2026-08-30). `BackupConfig` is a named field, so serde's
  /// flatten hands `extra` only the keys no named field claimed -- and the
  /// moment `backup` became a named field for `schedule`'s sake, the whole
  /// block stopped reaching `extra` and [`crate::backup::Retention::from_project`]
  /// began returning its hardcoded default for every project in the estate.
  /// **The fix for one half of this block silently broke the other half**, and
  /// nothing failed, because the values it fell back to were the ones the
  /// tests asserted.
  ///
  /// **`None` WHEN THE FILE HAS NO `retain` BLOCK, SO A BLOCK THAT IS SET CAN BE
  /// TOLD FROM ONE THAT IS DEFAULTED.** The tiers apply their defaults either
  /// way; the distinction exists for `doctor`, which names a config that sets
  /// `retain` beside [`keep`](Self::keep) because `keep` then decides alone.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub retain: Option<RetainConfig>,
  /// `keys.5`, absent by default. **When it is set it is the WHOLE pruning
  /// rule**: the newest N good snapshots are kept and `retain` is not read
  /// (ST0080, vc under hv's go, 2026-09-25). One rule, never two beside each
  /// other, so no snapshot is ever held by one and removed by the other.
  ///
  /// **AT LEAST 1, REFUSED ON READ OTHERWISE.** A retain tier of 0 disables
  /// one tier while the others still hold snapshots; a keep of 0 would remove
  /// the snapshot the same cycle had just written, which nobody means.
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    deserialize_with = "at_least_one"
  )]
  pub keep: Option<u32>,
}

/// `backup.keep` as written, refusing 0 by name rather than by serde's
/// generic "expected a nonzero" wording, which names no key.
fn at_least_one<'de, D>(de: D) -> Result<Option<u32>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  match Option::<u32>::deserialize(de)? {
    Some(0) => Err(serde::de::Error::custom(
      "backup.keep must be at least 1: a keep of 0 would remove the snapshot just taken \
       -- remove the key to prune by `backup.retain` instead",
    )),
    other => Ok(other),
  }
}

fn default_backup_enabled() -> bool {
  true
}

fn default_backup_schedule() -> String {
  "daily".to_string()
}

impl Default for BackupConfig {
  fn default() -> Self {
    Self {
      enabled: default_backup_enabled(),
      schedule: default_backup_schedule(),
      retain: None,
      keep: None,
    }
  }
}

/// How many snapshots to keep in each retention tier.
///
/// **THE KEY NAMES ARE ic's, NOT MINE, AND THAT IS THE WHOLE POINT OF THE
/// ROW.** `dispatch-table.md` says it in the ratification: *Key names are
/// SURFACE, so ic names them and cc implements against these; cc was told not
/// to invent them.* The first implementation read `keep_daily`, `keep_weekly`
/// and `keep_monthly` at the top of the `backup` object -- **invented names at
/// an invented nesting**, which is the `backup.every_hours` defect the
/// schedule key was rewritten to remove, in the same file, three times over
/// and never noticed because the names appeared nowhere except on the lines
/// that read them.
///
/// **ABSENT IS THE DEFAULT AND ZERO IS A CHOICE**, which is `keys.4.note` and
/// is the reason each field carries its own `default` rather than the struct
/// being read as all-or-nothing: *absence and zero must not be the same value
/// in a retention policy, because one of them deletes backups.*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainConfig {
  /// `keys.2`, default 7.
  #[serde(default = "default_retain_daily")]
  pub daily: u32,
  /// `keys.3`, default 4. A snapshot enters this tier by being the newest of
  /// its ISO week.
  #[serde(default = "default_retain_weekly")]
  pub weekly: u32,
  /// `keys.4`, default 12. A snapshot enters this tier by being the newest of
  /// its calendar month.
  ///
  /// **12 IS THE RATIFIED DEFAULT AND THE CODE HELD 6.** Nothing had ever read
  /// a config value here, so the hardcoded fallback WAS the policy -- an
  /// estate keeping half the monthly history its own surface promised, with no
  /// way to say otherwise.
  #[serde(default = "default_retain_monthly")]
  pub monthly: u32,
}

fn default_retain_daily() -> u32 {
  7
}

fn default_retain_weekly() -> u32 {
  4
}

fn default_retain_monthly() -> u32 {
  12
}

impl Default for RetainConfig {
  fn default() -> Self {
    Self {
      daily: default_retain_daily(),
      weekly: default_retain_weekly(),
      monthly: default_retain_monthly(),
    }
  }
}

/// The `index` block: what the search index will not read.
///
/// **ONE VALUE, AND IT IS A CONFIG VALUE BECAUSE THE DEFAULT IS A MEASUREMENT
/// OF ONE ESTATE.** `index::corpus::DEFAULT_MAX_FILE_BYTES` was chosen against
/// this repository's own tree, where the largest file in the disk corpus is
/// under a megabyte; a project whose documents are larger is not wrong, and a
/// cap it cannot move would silently drop them with a row saying `too-large`.
///
/// The cap is a floor on nothing: it is compared with `>`, so a file exactly at
/// it is held. A project that wants everything indexed sets it high rather than
/// to zero, which would skip every file it has.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexConfig {
  /// Files larger than this are in scope and not read. Default:
  /// [`crate::index::corpus::DEFAULT_MAX_FILE_BYTES`].
  #[serde(default = "default_max_file_bytes")]
  pub max_file_bytes: u64,
}

fn default_max_file_bytes() -> u64 {
  crate::index::corpus::DEFAULT_MAX_FILE_BYTES
}

impl Default for IndexConfig {
  fn default() -> Self {
    Self {
      max_file_bytes: default_max_file_bytes(),
    }
  }
}

impl IndexConfig {
  /// Is this block exactly what an absent block means?
  fn is_default(&self) -> bool {
    self == &Self::default()
  }
}

/// The `whiteboard` block: what the coordination estate is allowed to grow to.
///
/// **THE BOUNDS ARE THE POINT OF MODELLING THE BOARD AND NOT A TIDINESS
/// SETTING.** The measurement that opened this work is in the WP's own design:
/// three LLM nodes wrote roughly 100KB of board in two days, one inbox reached
/// 32KB, and the human's whole board was 308 bytes. The ratio is the
/// specification -- the human's board is what a board is for, and the other
/// three are what happens without a mechanism. A rule stated by the node that
/// then breaks it is the definition of a rule needing a mechanism rather than
/// better intentions, and `vc/wip.md` broke its own written rule about board
/// length in the same file that stated it.
///
/// **SO EVERY BOUND HERE IS ENFORCED BY REFUSAL, NEVER BY TRUNCATION.** A
/// truncating write accepts the call and silently keeps less than it was
/// given, which is `IN-AG-NO-SILENT-001` on a write path; the refusal states
/// the bound and the remedy and writes nothing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhiteboardConfig {
  /// The largest single entry body, in bytes. Compared with `>`, so a body
  /// exactly at the bound is accepted.
  ///
  /// **A BOUND IS SET FROM THE CORPUS IT GOVERNS, AND THE SETTING RECORDS WHICH
  /// CORPUS AND WHEN** (ic's rule, and ic's measurement is why it is a rule).
  /// The first value here was 2000, chosen as "a long paragraph" against the
  /// 32KB inbox that motivated the work -- and measured against the traffic it
  /// would actually govern it refused the MEDIAN case.
  #[serde(default = "default_wb_body_bytes")]
  pub body_bytes: usize,
  /// The most LIVE messages one inbox may hold -- an inbox being one ordered
  /// (sender, recipient) pair, as the file form already is.
  #[serde(default = "default_wb_live_messages")]
  pub live_messages: usize,
  /// The most LIVE items one node may hold OF ONE KIND.
  ///
  /// **PER KIND RATHER THAN PER BOARD, because the kinds grow at different
  /// rates and for different reasons.** Watch-outs accumulate and are the
  /// population this bound is really about; DOING is small by definition and a
  /// shared bound would let a long watch-out list refuse a node's next piece of
  /// work, which is the bound punishing the wrong thing.
  #[serde(default = "default_wb_live_items")]
  pub live_items: usize,
  /// Nodes the bounds do not apply to.
  ///
  /// **THE HUMAN IS UNBOUNDED BY DEFAULT AND THAT IS NOT AN EXEMPTION FOR
  /// SENIORITY.** The bounds exist to hold a class of writer that produces
  /// volume without noticing; the measurement says the human is not that
  /// writer -- their whole board was 308 bytes against one LLM inbox at 32KB --
  /// and a person stopped mid-sentence by a machine's idea of length has no
  /// recourse and did not opt in. Every LLM node can read the refusal and
  /// shorten the entry.
  ///
  /// **THE EXEMPTION IS THEREFORE WORTH LITTLE IN PRACTICE, WHICH IS THE POINT**
  /// (ic): it exempts the node that writes the shortest entries. It is here for
  /// the case it was reasoned about, not because it does work today.
  #[serde(default = "default_wb_unbounded")]
  pub unbounded: Vec<String>,
}

/// Fitted to the corpus it governs: every entry in every
/// `intent/whiteboard/*/inbox.*.md` on this repository, read 2026-09-12.
/// Forty-five entries, median 2423 bytes, largest 7113. This clears all of them
/// with headroom.
///
/// **IT GOVERNS ITEMS TOO, AND THERE IT BITES THE TAIL RATHER THAN THE MEDIAN.**
/// The same read found 138 items across the five boards, median 220 bytes, one
/// at 11164 -- so this refuses exactly one item that exists today, an 11KB
/// watch-out, which is the outlier the bound is for.
///
/// **THE VALUE IT REPLACED WAS 2000 AND IT WOULD HAVE REFUSED 62% OF THAT
/// CORPUS**, the median included. That failure is not an error anybody debugs:
/// it is a node discovering mid-report that its durable escalation will not fit,
/// whose cheapest way out is to say less. A ceiling that refuses the median case
/// teaches shorter reports rather than tidier ones, which is the opposite of
/// what the bound is for -- the board should carry the pointer and the artefact
/// should carry the account, and a squeezed report loses the account without
/// gaining the pointer.
fn default_wb_body_bytes() -> usize {
  8_192
}

fn default_wb_live_messages() -> usize {
  20
}

/// Fitted to the corpus it governs, on the rule `body_bytes` records: every
/// item in every `intent/whiteboard/*/wip.md` on this repository, read
/// 2026-09-12. The most any one (node, kind) holds is 31 -- one node's
/// watch-outs -- so a bound at or below that would refuse boards that exist
/// today, and this leaves room above the observed maximum while still bounding.
fn default_wb_live_items() -> usize {
  40
}

fn default_wb_unbounded() -> Vec<String> {
  vec![crate::model::HYPERVISOR.to_string()]
}

impl Default for WhiteboardConfig {
  fn default() -> Self {
    Self {
      body_bytes: default_wb_body_bytes(),
      live_messages: default_wb_live_messages(),
      live_items: default_wb_live_items(),
      unbounded: default_wb_unbounded(),
    }
  }
}

impl WhiteboardConfig {
  /// Is this block exactly what an absent block means?
  fn is_default(&self) -> bool {
    self == &Self::default()
  }

  /// Do the bounds apply to this node?
  pub fn bounds_apply_to(&self, node: &str) -> bool {
    !self.unbounded.iter().any(|n| n == node)
  }
}

/// The `embed` block: the semantic tier's endpoint, if a project has one.
///
/// **ABSENT IS THE NORMAL CASE AND IT IS NOT A GAP.** Without it the semantic
/// tier is not built for this project and a semantic question is refused with a
/// remedy naming these keys; the lexical and structural tiers answer as they
/// always did.
///
/// `key` is here because an endpoint may want one, and it is the operator's to
/// put in their own config; nothing in this build writes it, reads it from an
/// environment variable, or logs it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EmbedConfig {
  /// An OpenAI-compatible embeddings endpoint.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub endpoint: Option<String>,
  /// The model name sent with every request and stored beside every vector.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub model: Option<String>,
  /// The width the model returns. A vector of another width is refused rather
  /// than stored, because a ranking over mixed widths is a ranking of nothing.
  #[serde(default)]
  pub dims: usize,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub key: Option<String>,
}

impl EmbedConfig {
  fn is_default(&self) -> bool {
    self == &Self::default()
  }
}

/// Where a project's two agent contracts live (ST0083): `root` keeps `AGENTS.md`
/// and `CLAUDE.md` at the project root, where the agents.md cross-tool spec's
/// readers open them; `dotclaude` keeps both under `.claude/`, for a tree whose
/// root is read by people rather than by tools and whose one agent, Claude Code,
/// reads `.claude/` on its own.
///
/// **AN ENUM AND NOT A PATH**: both values are places Claude Code reads
/// natively, and a free path would admit a place no tool reads. **ABSENT MEANS
/// `root` AND THE DEFAULT IS NEVER WRITTEN BACK**, for the `index` block's
/// reason, so no existing `config.json` changes a byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Contracts {
  #[default]
  Root,
  Dotclaude,
}

impl Contracts {
  /// Is this the default, which a rewrite leaves out of the file?
  fn is_root(&self) -> bool {
    *self == Self::Root
  }

  /// The directory the two contracts live in, under the project root `root`.
  ///
  /// **THE ONE ANSWER TO WHERE THE CONTRACTS ARE.** `init`, `agents`, canon,
  /// the sync scope and through it the daemon's ingest all derive the place
  /// from this, so no site can put a contract where another site will not look.
  pub fn dir(self, root: &Path) -> PathBuf {
    match self {
      Self::Root => root.to_path_buf(),
      Self::Dotclaude => root.join(".claude"),
    }
  }
}

/// **A VALUE OTHER THAN THE TWO IS REFUSED BY NAME, WITH THE TWO IT ACCEPTS.**
/// The derived form's error names neither the key nor a non-string value's
/// type, so the reader of `{path} is not valid Intent config: ...` could not
/// tell which line of the file to change or what to change it to.
impl<'de> Deserialize<'de> for Contracts {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    match value.as_str() {
      Some("root") => Ok(Self::Root),
      Some("dotclaude") => Ok(Self::Dotclaude),
      _ => Err(serde::de::Error::custom(format!(
        "`contracts` is {value}, and it accepts \"root\" or \"dotclaude\""
      ))),
    }
  }
}

/// The per-project config (`intent/.config/config.json`).
///
/// Unknown fields are PERMITTED here, and that is deliberate rather than an
/// oversight of D05. config.json is the one canon file v2 also writes, plugins
/// extend it with their own blocks, and a v3 binary that refused a config it
/// did not fully recognise would brick a project the moment any other tool
/// touched it. Strictness belongs on the artefacts v3 owns outright.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
  pub intent_version: String,
  #[serde(default)]
  pub project_name: String,
  #[serde(default)]
  pub author: String,
  /// Stamped at migration (D15); absent on a v2 project, which is how the
  /// migrator tells "not yet migrated" from "migrated".
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub project_id: Option<String>,
  #[serde(default = "default_intent_dir")]
  pub intent_dir: String,
  #[serde(default)]
  pub languages: Vec<String>,
  /// Where the two agent contracts live (ST0083); absent means `root`, and the
  /// default is never written back.
  #[serde(default, skip_serializing_if = "Contracts::is_root")]
  pub contracts: Contracts,
  /// The `todo` block (D44). Absent in every v2 config, hence the default.
  #[serde(default)]
  pub todo: TodoConfig,
  /// The `backup` block (D35). Absent in every config written before the
  /// schedule became a key, hence the default.
  #[serde(default)]
  pub backup: BackupConfig,
  /// The `embed` block. Absent unless a project has an embedder, and not
  /// written back when it is empty, for the `index` block's reason.
  #[serde(default, skip_serializing_if = "EmbedConfig::is_default")]
  pub embed: EmbedConfig,
  /// The `index` block. Absent in every config written before the search index
  /// had a value to set, hence the default -- and **NOT WRITTEN BACK WHEN IT IS
  /// THE DEFAULT**, as `doctor`'s block is not: a block serialised
  /// unconditionally changes the bytes of every existing project's
  /// `config.json` on the next rewrite, and `config.json` is in the sync
  /// corpus, so that rewrite is an edit the watcher sees and the ingest acts
  /// on. Nobody asked for it and nothing reports it as a change of meaning.
  ///
  /// **THIS WAS FIRST WRITTEN AS A MEASUREMENT AND IT WAS NOT ONE.** The
  /// daemon's subscription arms red while it was being built, the block looked
  /// like the cause, and four runs with the change against four without put the
  /// reds on both sides -- that pair is load-sensitive and reds about one run in
  /// four either way. The reason above stands on its own; the evidence I
  /// thought I had did not.
  #[serde(default, skip_serializing_if = "IndexConfig::is_default")]
  pub index: IndexConfig,
  /// The `whiteboard` block. Absent in every config written before the board
  /// had bounds, hence the default, and not written back when it is the
  /// default for the `index` block's reason.
  #[serde(default, skip_serializing_if = "WhiteboardConfig::is_default")]
  pub whiteboard: WhiteboardConfig,
  /// The `doctor` block (issue 0065, hv decision 14). Absent unless a project
  /// has acknowledged something, and never written when empty.
  #[serde(default, skip_serializing_if = "DoctorConfig::is_empty")]
  pub doctor: DoctorConfig,
  /// The project's OWN pre-commit guards (issue 0426): each an argv `run`,
  /// resolved from the project root, and an optional `when` path.
  /// The pre-commit hook reads and dispatches them after Intent's roster;
  /// typed here so `doctor` can name a hook line that no declaration covers.
  /// Absent unless a project declares one, and never written when empty.
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub guards: Vec<GuardDecl>,
  /// The `commands` block (issue 0678): the build and test commands a project
  /// without `bin/devbin` declares, which `AGENTS.md` names. Absent unless a
  /// project declares one, and not written back when empty, for the `index`
  /// block's reason.
  #[serde(default, skip_serializing_if = "CommandsConfig::is_empty")]
  pub commands: CommandsConfig,
  /// Everything else in the file, carried so a rewrite never drops a block
  /// this version does not know about.
  ///
  /// **`st_prefix` lands here, and that is the retirement working rather than
  /// a leak** (issue 0040, hv). The field is gone from the type -- the prefix
  /// is fixed at `crate::id::THREAD_PREFIX` -- so a v2 project that
  /// declared it keeps the declaration in the file, byte for byte, instead of
  /// having it silently dropped on the first rewrite. v3 does not honour it.
  /// Retiring a knob nobody uses is fine; retiring it under someone who does,
  /// without telling them, is the silent data change this thread exists to
  /// prevent.
  #[serde(flatten)]
  pub extra: serde_json::Map<String, serde_json::Value>,
}

/// **THE FIRST MUTATION ROUTE TO THE PROJECT MARKER, AND IT IS BIGGER THAN THE
/// ARM THAT ASKED FOR IT.**
///
/// Until this existed, `config.json` was written exactly once -- `init.rs`
/// lays it down and REFUSES if one is already there -- and nothing anywhere
/// updated it. `intent lang init`/`remove` are the first verbs that need to,
/// which makes them a new write path to the one file that decides whether a
/// directory is an Intent project at all, rather than "wire an arm".
///
/// **The dangerous half is already closed by [`Config::extra`] and not by
/// anything here**: a typed round trip through a struct that does not model
/// every key would silently drop the ones it does not know, and `#[serde(flatten)]`
/// is what stops that. `st_prefix` is the worked example -- retired from the
/// type, still in the file. The rewrite below is only safe BECAUSE of that
/// field, so the two must not be separated.
///
/// **Top-level key ORDER is kept too, since issue `0091`**: `write_config`
/// writes the file's own members in the file's order and changes only the
/// ones the caller changed. What still comes back alphabetised is the INSIDE of
/// an unmodelled block, because it round-trips through a `serde_json::Map`,
/// which is sorted in this build -- content-preserving and diff-noisy, as
/// before.
/// A fresh project identity: D15's cloud seam, minted.
///
/// **ONE HOME, BECAUSE TWO PATHS MINT AND A PROJECT HAS EXACTLY ONE IDENTITY.**
/// `init` mints for a project being created and the migration mints for one
/// being carried forward, and those are the only two ways a project comes to
/// exist. A second `Uuid::new_v4()` literal at either site would be two homes
/// for the one fact D15 ratifies -- and the drift would be invisible, because
/// both spellings produce a perfectly good UUID and nothing downstream can tell
/// which function made it.
///
/// This follows [`BackupConfig::default`] immediately above, which exists for
/// the same reason and says so: *a second literal here is how a default drifts
/// from the thing that reads it*.
pub fn mint_project_id() -> String {
  uuid::Uuid::new_v4().to_string()
}

/// The identity an upgrade gives a project in git that has none (issue
/// Ie99dbb6), derived from the repository's root commit, which every branch
/// shares: two branches upgraded apart give the project ONE identity, where
/// two mints gave it two and the merge conflicted on config.json. A SHA-256 of
/// the commit, shaped as a UUID, so the value reads as every other project id
/// does. Pure: the caller asks git, and mints with [`mint_project_id`] where
/// there is no commit to derive from.
pub fn derived_project_id(root_commit: &str) -> String {
  use sha2::{Digest, Sha256};
  let digest = Sha256::digest(format!("intent project id\0{root_commit}").as_bytes());
  let mut bytes = [0u8; 16];
  bytes.copy_from_slice(&digest[..16]);
  uuid::Builder::from_random_bytes(bytes)
    .into_uuid()
    .to_string()
}

impl Config {
  /// The project's identity, or `None` when it has never been minted.
  ///
  /// **AN EMPTY STRING IS ABSENT, NOT A VALUE, AND THAT IS THE WHOLE REASON
  /// THIS METHOD EXISTS.** `project_id` is `Option<String>` and every read site
  /// reached it through `.unwrap_or_default()`, which turns *this project has
  /// no identity* into `""` -- an answer, of the right type, that every
  /// consumer downstream is then correct about. That is
  /// `IN-AG-NO-SILENT-001` in the accessor of a ratified seam.
  ///
  /// **IT WAS NOT A HYPOTHETICAL.** Three subscription arms asserted that an
  /// event's project matched the feed's, both sides arrived as `""`, and all
  /// three passed: an equality between two defaults is a tautology wearing a
  /// comparison's clothes. Callers that must branch on absence ask this;
  /// callers that genuinely want a display string can still default, but they
  /// now have to say so.
  pub fn identity(&self) -> Option<&str> {
    self.project_id.as_deref().filter(|id| !id.is_empty())
  }

  /// Declare a language. `true` when the array actually changed.
  ///
  /// **APPENDS, never sorts, and idempotent** -- v2's `add_project_language`
  /// does exactly this (`bin/intent_helpers:541`), and the order is the
  /// author's rather than the tool's. Sorting would rewrite a line nobody
  /// asked to change on the first declaration after an upgrade.
  pub fn declare_language(&mut self, lang: &str) -> bool {
    if self.languages.iter().any(|l| l == lang) {
      return false;
    }
    self.languages.push(lang.to_string());
    true
  }

  /// Undeclare a language. `true` when the array actually changed.
  ///
  /// Removes EVERY occurrence, matching v2's `.languages - [$lang]`. A
  /// duplicate should not exist, and if one does, a remove that left the
  /// second copy behind would report success and change nothing an operator
  /// can see.
  pub fn undeclare_language(&mut self, lang: &str) -> bool {
    let before = self.languages.len();
    self.languages.retain(|l| l != lang);
    self.languages.len() != before
  }

  /// `intent config set <key> <value>` on this config, in memory (issue 0678):
  /// `true` when the value changed. An empty value removes the key.
  ///
  /// **ONE LIST OF THE KEYS THIS BUILD SETS, [`SettableKey::ALL`]**, which this
  /// dispatches on and the refusal of any other key names, so the two cannot
  /// disagree. A key outside it is refused rather than written as an unknown
  /// member: one the build does not set reads as configured and does nothing.
  pub fn set(&mut self, key: &str, value: &str) -> Result<bool, ConfigSetError> {
    let key = SettableKey::require(key)?;
    // **ONE LINE, BECAUSE THE VALUE LANDS IN A FENCE** (vc's condition on the
    // shape): a newline would end the command and start another line of the
    // contract that nobody declared.
    if value.contains(['\n', '\r']) {
      return Err(ConfigSetError::MultiLine {
        key: key.name().to_string(),
      });
    }
    let wanted = (!value.is_empty()).then(|| value.to_string());
    let slot = match key {
      SettableKey::Build => &mut self.commands.build,
      SettableKey::Test => &mut self.commands.test,
    };
    if *slot == wanted {
      return Ok(false);
    }
    *slot = wanted;
    Ok(true)
  }
}

/// A config key `intent config set` writes in this build (issue 0678).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettableKey {
  Build,
  Test,
}

impl SettableKey {
  /// Every key this build sets, in the order a refusal names them.
  pub const ALL: [Self; 2] = [Self::Build, Self::Test];

  /// The dotted key as `config set` takes it.
  pub fn name(self) -> &'static str {
    match self {
      Self::Build => "commands.build",
      Self::Test => "commands.test",
    }
  }

  pub fn parse(key: &str) -> Option<Self> {
    Self::ALL.into_iter().find(|k| k.name() == key)
  }

  /// [`Self::parse`], or the refusal naming every key this build sets.
  pub fn require(key: &str) -> Result<Self, ConfigSetError> {
    Self::parse(key).ok_or_else(|| ConfigSetError::NotSettable {
      key: key.to_string(),
    })
  }
}

/// Why `intent config set` wrote nothing (issue 0678).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigSetError {
  /// A key this build does not set: the command it asks for is not built.
  #[error("`intent config set` is not built for `{key}` in this build")]
  NotSettable { key: String },
  /// A value of more than one line.
  #[error("the value for `{key}` is more than one line")]
  MultiLine { key: String },
}

impl crate::remedy::Remedy for ConfigSetError {
  fn remedy(&self) -> String {
    match self {
      Self::NotSettable { .. } => format!(
        "this build sets {}; any other key is edited in intent/.config/config.json by hand",
        SettableKey::ALL
          .iter()
          .map(|k| format!("`{}`", k.name()))
          .collect::<Vec<_>>()
          .join(" and ")
      ),
      Self::MultiLine { key } => format!(
        "give `{key}` one line: it is printed as one command in AGENTS.md, so chain several with `&&`"
      ),
    }
  }
}

/// The `commands` block (issue 0678): what a project without `bin/devbin`
/// builds and tests with, as the contract names it. Intent never runs either.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandsConfig {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub build: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub test: Option<String>,
}

impl CommandsConfig {
  pub fn is_empty(&self) -> bool {
    self.build.is_none() && self.test.is_none()
  }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigWriteError {
  #[error("cannot serialise the project config")]
  Encode(#[source] serde_json::Error),
  #[error("cannot write the project config at {path}")]
  Write {
    path: String,
    #[source]
    source: crate::write_set::WriteError,
  },
  /// The file on disk could not be read back as a project config, so there is
  /// no baseline to write the change onto. Nothing was written.
  #[error("cannot read the project config at {path} back before rewriting it: {reason}")]
  Unreadable { path: String, reason: String },
}

impl crate::remedy::Remedy for ConfigWriteError {
  fn remedy(&self) -> String {
    match self {
      // Not an operator fault and not fixable by them: a Config that will not
      // encode is a defect in this build, so the remedy says so rather than
      // sending someone to a verb that cannot help.
      Self::Encode(_) => {
        "this is a defect in this build rather than a fault in your project -- report it with the config that provoked it".to_string()
      }
      Self::Write { path, .. } => {
        format!("check that {path} and the directory holding it are writable by the account running intent")
      }
      Self::Unreadable { path, .. } => {
        format!("nothing was written -- repair {path} by hand so it parses as JSON with an `intent_version`, then re-run")
      }
    }
  }
}

/// Write a [`Config`] back to `<root>/intent/.config/config.json`.
///
/// **Atomically, via `crate::write_set::write_atomically` rather than a
/// second copy of that logic.** A half-written `config.json` is not a damaged
/// file, it is a directory that has stopped being an Intent project -- every
/// verb discovers a project by finding and parsing this one file -- so the
/// temp-plus-rename is load-bearing here in a way it is not for a view that can
/// simply be re-rendered. v2 reached the same conclusion by hand: both
/// `add_project_language` and `remove_project_language` write through `mktemp`
/// and `mv`.
///
/// **ONLY WHAT THE CALLER CHANGED IS WRITTEN** (issue `0091`). Serialising the
/// whole [`Config`] materialised every `#[serde(default)]` into the file, so the
/// first `lang init` froze `author`, `intent_dir`, `todo` and `backup` at that
/// day's defaults and the project silently stopped tracking the tool. v2 edited
/// the one key with `jq` and left the rest alone; this is the same rule for a
/// typed writer -- see `changed_members`. A key absent from the file stays
/// absent unless its value changed, and a key present keeps its place and,
/// unless changed, its bytes.
pub fn write_config(root: &Path, config: &Config) -> Result<(), ConfigWriteError> {
  let path = Project::config_path(root);
  let unreadable = |reason: String| ConfigWriteError::Unreadable {
    path: path.display().to_string(),
    reason,
  };
  let wanted: Members = serde_json::to_string(config)
    .and_then(|text| serde_json::from_str(&text))
    .map_err(ConfigWriteError::Encode)?;
  let members = match std::fs::read_to_string(&path) {
    // No file yet: there is nothing to track, so the whole config is the write.
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => wanted,
    Err(e) => return Err(unreadable(e.to_string())),
    Ok(text) => {
      let file: Members = serde_json::from_str(&text).map_err(|e| unreadable(e.to_string()))?;
      let read: Config = serde_json::from_str(&text).map_err(|e| unreadable(e.to_string()))?;
      let read = match serde_json::to_value(&read).map_err(ConfigWriteError::Encode)? {
        serde_json::Value::Object(map) => map,
        _ => return Err(unreadable("a project config is a JSON object".to_string())),
      };
      changed_members(&file, &read, wanted)
    }
  };
  // Two-space pretty, plus the trailing newline `init.rs` writes, so a config
  // this rewrites keeps the shape of one it laid down.
  let mut body = serde_json::to_string_pretty(&members).map_err(ConfigWriteError::Encode)?;
  body.push('\n');
  crate::write_set::write_atomically(&path, body.as_bytes()).map_err(|source| {
    ConfigWriteError::Write {
      path: path.display().to_string(),
      source,
    }
  })
}

/// A JSON object's members IN THE ORDER THEY WERE READ.
///
/// `serde_json::Map` is sorted in this build, so a rewrite through a `Value`
/// would alphabetise every config it touched. This keeps the file's own order,
/// and a [`Config`]'s field order for keys the file does not have yet.
struct Members(Vec<(String, serde_json::Value)>);

impl Members {
  fn get(&self, key: &str) -> Option<&serde_json::Value> {
    self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
  }
}

impl Serialize for Members {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(self.0.len()))?;
    for (k, v) in &self.0 {
      map.serialize_entry(k, v)?;
    }
    map.end()
  }
}

impl<'de> Deserialize<'de> for Members {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct Visit;
    impl<'de> serde::de::Visitor<'de> for Visit {
      type Value = Members;
      fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a JSON object")
      }
      fn visit_map<A: serde::de::MapAccess<'de>>(self, mut access: A) -> Result<Members, A::Error> {
        let mut members = Vec::new();
        while let Some(entry) = access.next_entry::<String, serde_json::Value>()? {
          members.push(entry);
        }
        Ok(Members(members))
      }
    }
    deserializer.deserialize_map(Visit)
  }
}

/// The config to write: the FILE, with only the members the caller changed.
///
/// `read` is the file as a [`Config`] reads it, defaults filled in, so "changed"
/// means changed from what the caller was handed -- a default the caller left
/// alone equals its `read` value and is not written. Pure, so the rule is
/// testable without a filesystem.
///
/// - A key in the file keeps its position. It takes the caller's value if that
///   differs from `read`, keeps its own bytes if not, and is dropped if the
///   caller no longer carries it (an `Option` set to `None`, or a key removed
///   from [`Config::extra`]).
/// - A key not in the file is added, after the file's keys, only if its value
///   differs from `read` -- which for such a key is the default.
///
/// **The one case this cannot tell apart:** setting a key the file lacks to
/// exactly its default leaves it absent, still tracking the tool. Nothing in a
/// typed value says the caller meant it; the file saying nothing is the honest
/// record of that.
fn changed_members(
  file: &Members,
  read: &serde_json::Map<String, serde_json::Value>,
  wanted: Members,
) -> Members {
  let changed = |key: &str, value: &serde_json::Value| read.get(key) != Some(value);
  let mut out = Vec::with_capacity(file.0.len());
  for (key, own) in &file.0 {
    match wanted.get(key) {
      Some(value) if changed(key, value) => out.push((key.clone(), value.clone())),
      Some(_) => out.push((key.clone(), own.clone())),
      None if read.contains_key(key) => {}
      None => out.push((key.clone(), own.clone())),
    }
  }
  for (key, value) in wanted.0 {
    if file.get(&key).is_none() && changed(&key, &value) {
      out.push((key, value));
    }
  }
  Members(out)
}

/// The `doctor` block: finding classes this project has decided to keep.
///
/// **An acknowledgement moves a class OUT OF THE VERDICT AND NEVER OUT OF THE
/// REPORT** (hv, decision 14, on issue 0065). The class still runs and still
/// prints, as its name, this reason and its count; what it stops doing is
/// counting and failing the run. A check that disappears is worse than one
/// that shouts, so there is no form of this that hides one.
///
/// **In config because it is committed, per-project and read in a diff** --
/// the place a reader can see and audit a decision to keep something.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorConfig {
  /// Finding class -> why it is kept. The key is the class's name as `doctor`
  /// prints it (eg `backup-stale`); one naming no class is itself reported.
  #[serde(default)]
  pub acknowledged: std::collections::BTreeMap<String, String>,
}

impl DoctorConfig {
  fn is_empty(&self) -> bool {
    self.acknowledged.is_empty()
  }
}

/// One entry of the `guards` array: a project guard as its author declared it.
///
/// **`run` IS AN ARGV, NOT A COMMAND LINE**, so the hook never hands it to a
/// shell and a path with a space in it means what it says. `run[0]` is a path
/// relative to the project root, and it must be tracked: a guard a fresh clone
/// does not receive is refused by the hook rather than skipped.
///
/// **AN UNKNOWN KEY IS REFUSED** (issue 0616, vc's ruling of 2026-10-23). A
/// misspelt `when` parsed as a guard with no `when`, which runs on every commit,
/// in the binary and in the gate alike. vc's census that day found guards
/// declared in one estate, with `run` alone, so the refusal reaches no
/// declaration written before it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuardDecl {
  pub run: Vec<String>,
  /// A path relative to the project root, NOT a pattern: the hook skips the
  /// guard as not applicable while nothing exists there. Absent means the guard
  /// applies to every commit.
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub when: Option<String>,
}

/// The field separator of a guard line: US (0x1f), the commit gate's own.
///
/// **NOT TAB, WHICH WAS THE GATE'S FIRST SPELLING.** Tab is whitespace to `read`,
/// so the empty `when` of a guard that applies to every commit collapsed into
/// the field after it, and the guard was skipped as not applicable. A separator
/// that is not whitespace keeps an empty field a field.
pub const GUARD_FIELD_SEPARATOR: char = '\u{1f}';

/// Why a declared guard cannot be handed to the commit gate (issue 0616).
///
/// What serde refuses -- a `guards` that is not an array, an entry that is not
/// an object, a `run` that is not strings, an unknown key -- is refused before
/// this, by every verb, as [`ProjectError::Config`] with its position. These are
/// what serde accepts and a guard line cannot carry.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GuardError {
  #[error("guards[{index}] in intent/.config/config.json has an empty run")]
  EmptyRun { index: usize },
  #[error(
    "guards[{index}].run[{element}] in intent/.config/config.json is empty, and a guard line cannot carry an empty argument"
  )]
  EmptyArgument { index: usize, element: usize },
  #[error(
    "guards[{index}].{field} in intent/.config/config.json holds a control character, and a guard line cannot carry one"
  )]
  ControlCharacter { index: usize, field: String },
}

impl crate::remedy::Remedy for GuardError {
  fn remedy(&self) -> String {
    match self {
      Self::EmptyRun { index } => format!(
        "give guards[{index}] a run whose first element is a tracked, executable file, or remove the entry"
      ),
      Self::EmptyArgument { index, element } => {
        format!("give guards[{index}].run[{element}] a value, or take it out of the run")
      }
      Self::ControlCharacter { index, field } => format!(
        "take the control character out of guards[{index}].{field}: the gate reads one guard per line, so no field can hold a newline, a tab or the separator"
      ),
    }
  }
}

/// The declared guards as the commit gate reads them, one line each:
/// `<index> US <when> US <run[0]> [US <run[n]> ...]`, with an empty `when` for
/// a guard that applies to every commit (issue 0616).
///
/// **THE ONE VALIDATION OF A GUARD DECLARATION.** The gate carried its own jq
/// program for this beside [`Config::guards`], two readers of one field with two
/// sets of rules and nothing to report where they disagreed. `intent guards`
/// prints these lines, and the gate runs what it is given.
///
/// **NO LINE ENDS IN AN EMPTY FIELD**, because `run` is never empty and no
/// argument in it is: bash 3.2's `read -a` drops a trailing empty field, while
/// it keeps the empty `when` in the middle of a line.
pub fn guard_lines(guards: &[GuardDecl]) -> Result<Vec<String>, GuardError> {
  guards
    .iter()
    .enumerate()
    .map(|(index, guard)| guard_line(index, guard))
    .collect()
}

/// One guard's line, or why it cannot be carried.
fn guard_line(index: usize, guard: &GuardDecl) -> Result<String, GuardError> {
  if guard.run.is_empty() {
    return Err(GuardError::EmptyRun { index });
  }
  let when = guard.when.as_deref().unwrap_or("");
  if when.chars().any(char::is_control) {
    return Err(GuardError::ControlCharacter {
      index,
      field: "when".to_string(),
    });
  }
  for (element, arg) in guard.run.iter().enumerate() {
    if arg.is_empty() {
      return Err(GuardError::EmptyArgument { index, element });
    }
    if arg.chars().any(char::is_control) {
      return Err(GuardError::ControlCharacter {
        index,
        field: format!("run[{element}]"),
      });
    }
  }
  let mut fields = vec![index.to_string(), when.to_string()];
  fields.extend(guard.run.iter().cloned());
  Ok(fields.join(&GUARD_FIELD_SEPARATOR.to_string()))
}

/// The `todo` block: how much of the DONE bucket a TERMINAL render shows.
///
/// **Configuration, not state, and that is the whole of D44.** v2 kept a
/// watermark inside the generated file and read it back out, which made a
/// disposable view its own database -- deleting it silently resurrected every
/// flushed item. There is nothing durable here: every completed date is
/// already in the model, so the DONE bucket is a QUESTION asked at render time
/// rather than a set anyone maintains.
///
/// **It is config rather than a flag because all six `todo` verbs regenerate
/// the file**, so a flag on any one of them is a silent-revert generator -- the
/// next verb rewrites without it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoConfig {
  /// How far back the terminal's DONE bucket reaches. Default 24.
  ///
  /// **The unit is hours and the resolution is a DAY, which is a property of
  /// the data rather than of this field.** `steel_thread.completed` is a
  /// domain date -- `YYYY-MM-DD`, no time component, carried from v2 and never
  /// re-stamped -- so a cutoff finer than a day has nothing to bite on. The
  /// unit stays hours because that is what D44 ruled and what a longer window
  /// wants to be expressed in.
  ///
  /// **A value the data cannot honour is REFUSED rather than rounded** -- see
  /// [`TodoConfig::window`], which is the only supported way to read this.
  #[serde(default = "default_window_hours")]
  pub window_hours: u32,
}

fn default_window_hours() -> u32 {
  24
}

impl Default for TodoConfig {
  fn default() -> Self {
    Self {
      window_hours: default_window_hours(),
    }
  }
}

/// A configured window the data cannot honour.
///
/// Its own type rather than a string, because two callers need it -- the render
/// path refuses with it and `doctor` reports it -- and a message assembled
/// twice is a message that drifts once.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
  "todo.window_hours is {configured}, which is not a whole number of days -- steel thread completion is recorded as a date with no time component, so a cutoff \
   of {configured}h is truncated to a date and the window means different things at different times of day"
)]
pub struct UnhonourableWindow {
  /// What `config.json` asked for.
  pub configured: u32,
  /// The finest interval the data can distinguish, from
  /// [`crate::model::COMPLETED_RESOLUTION_HOURS`].
  pub resolution: u32,
}

impl UnhonourableWindow {
  /// The rule itself, with the resolution passed in rather than read.
  ///
  /// **Split out so the self-retirement claim is TESTABLE instead of merely
  /// asserted in a comment.** A test can hand this a resolution of `1` -- what
  /// [`crate::model::COMPLETED_RESOLUTION_HOURS`] becomes the day `completed`
  /// gains a time component -- and observe that nothing at all is refused. A
  /// guard that says in prose that it will retire itself is a guard nobody
  /// checks; this one is measured.
  ///
  /// **A resolution of `0` or `1` refuses nothing, and the `0` half is the
  /// guard rather than a courtesy.** `is_multiple_of(0)` is `self == 0`, so
  /// without the short-circuit a resolution of zero would refuse EVERY non-zero
  /// window -- the failure inverted, on the one edit this code exists to
  /// anticipate.
  pub fn check(window_hours: u32, resolution: u32) -> Result<u32, Self> {
    if resolution <= 1 || window_hours.is_multiple_of(resolution) {
      Ok(window_hours)
    } else {
      Err(Self {
        configured: window_hours,
        resolution,
      })
    }
  }

  fn remedy_text(&self) -> String {
    let down = self.configured - (self.configured % self.resolution);
    let up = down + self.resolution;
    format!(
      "set todo.window_hours to a whole multiple of {} -- {down} or {up} -- in intent/.config/config.json",
      self.resolution
    )
  }
}

impl crate::remedy::Remedy for UnhonourableWindow {
  fn remedy(&self) -> String {
    self.remedy_text()
  }
}

impl TodoConfig {
  /// The configured window, or the reason the data cannot honour it.
  ///
  /// **THE ONLY SUPPORTED READ OF `window_hours`, and the refusal is vc's
  /// ruling** (2026-08-17). Two spellings were offered and both rejected:
  /// `window_days`, because it forecloses `completed` gaining a time component,
  /// which is live -- the field is date-resolution because v2 was, and nothing
  /// rules v3 must stay so; and hours-with-a-comment, because that leaves a
  /// config value silently meaning something other than it says.
  ///
  /// **The failure it prevents is worse than rounding, which is how it was
  /// first described.** The cutoff is `date('now', '-Nh')`, truncated to a
  /// date -- so at 02:00 a 6-hour window reaches back into yesterday and at
  /// 12:00 it does not. The same configuration produces different DONE buckets
  /// depending on the hour it is read at, with nothing on screen to say why.
  /// A named refusal at the point of use beats a silent divergence between what
  /// a setting says and what it does, which is the class this thread has now
  /// found six times in two days.
  ///
  /// **It retires itself** -- see [`crate::model::COMPLETED_RESOLUTION_HOURS`].
  ///
  /// Zero is honourable and deliberately not special-cased into "show
  /// everything": it is a whole number of days and it means what it says, a
  /// DONE bucket reaching back to the start of today.
  pub fn window(&self) -> Result<u32, UnhonourableWindow> {
    UnhonourableWindow::check(self.window_hours, crate::model::COMPLETED_RESOLUTION_HOURS)
  }
}

pub(crate) fn default_intent_dir() -> String {
  "intent".to_string()
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
  #[error("no Intent project found at or above {0} (looked for intent/.config/config.json)")]
  NotFound(String),
  /// A config declaring an `intent_version` before [`OLDEST_READ_MAJOR`]: a
  /// config this binary cannot read, answered as one ([`Self::config_unread`]).
  #[error(
    "{path} declares Intent {declared}, a version before {major}, which this build cannot read",
    major = OLDEST_READ_MAJOR
  )]
  PreV3 { path: String, declared: String },
  /// Issue 0609: the root is a bare git repository, or sits inside one with no
  /// `.git` entry between them. Carries the repository's path.
  #[error("{0} is {bare}", bare = BARE_REPOSITORY)]
  BareRepository(String),
  #[error("reading {path}: {source}")]
  Io {
    path: String,
    #[source]
    source: std::io::Error,
  },
  #[error("{path} is not valid Intent config: {source}")]
  Config {
    path: String,
    #[source]
    source: serde_json::Error,
  },
  /// A canon file whose name is no stored id of its collection's family
  /// (ST0095 WP-06, AC-06.5). Refused by name and never skipped: a record no
  /// reader takes is one the next write of its collection loses.
  #[error("{path} is named as no {kind} id, so no reader can say which record it is")]
  CanonNameNotAnId { path: String, kind: &'static str },
}

impl ProjectError {
  /// Whether this refusal is a config the binary cannot read at all: no config
  /// file, a file it could not read, text that is not JSON, JSON naming no
  /// `intent_version`, or one declaring a version before
  /// [`OLDEST_READ_MAJOR`]. Any other refusal of a config is one it read and
  /// whose declaration it refused, such as an unknown key in a guard.
  ///
  /// **THE COMMIT GATE'S READERS ANSWER THE TWO DIFFERENTLY** (vc's ruling,
  /// 2026-10-23): could not judge for the first, which the gate fails open on,
  /// and a refusal for the second, which refuses the commit. The JSON is read
  /// again to tell them apart, because serde's error does not say which it was.
  pub fn config_unread(&self) -> bool {
    match self {
      Self::Config { path, .. } => std::fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .is_none_or(|config| config.get("intent_version").is_none()),
      Self::NotFound(_) | Self::PreV3 { .. } | Self::BareRepository(_) | Self::Io { .. } => true,
      Self::CanonNameNotAnId { .. } => false,
    }
  }
}

impl crate::remedy::Remedy for ProjectError {
  fn remedy(&self) -> String {
    match self {
      Self::NotFound(_) => {
        "run `intent init` here, or change to a directory inside an Intent project".to_string()
      }
      Self::PreV3 { path, declared } => format!(
        "this project's config declares Intent {declared}, a version before {OLDEST_READ_MAJOR}, and this build does not convert one: run `intent upgrade` with Intent 3.5.1, the last release that converts a project from before {OLDEST_READ_MAJOR}, then use this build. If the config was edited by accident and the project's canon is already Intent {OLDEST_READ_MAJOR}'s, `git checkout -- {path}` restores the committed config"
      ),
      // A clone of the repository may already hold a project, so `intent init`
      // is named for a clone that has none rather than as the next command. The
      // files an earlier `init` left are named from `init`'s own list, so an
      // operator undoing one deletes what it wrote and nothing of git's.
      Self::BareRepository(repository) => format!(
        "a project lives in a working copy, never in a bare repository: run Intent in a clone of it (`git clone {repository} <dir>`), where `intent init` starts a project if the clone has none. If an earlier `intent init` wrote files here, delete `intent/` and the root files it wrote ({}) and leave everything git keeps there",
        crate::init::root_files().join(", ")
      ),
      Self::Io { path, .. } => {
        format!("check that {path} exists and that this user can read it")
      }
      // The parse error above carries the line and column, which is the whole
      // of the action -- and the second clause matters more than the first:
      // this file is committed, so git holds the version before the edit.
      Self::Config { path, .. } => format!(
        "the parse error above names the position in {path}; if you did not mean to edit it, `git checkout -- {path}` restores the committed version"
      ),
      Self::CanonNameNotAnId { path, .. } => format!(
        "a canon file is named by its record's stored id: rename {path} to the id its own `id` field holds, or move it out of the canon directory if it is no record"
      ),
    }
  }
}

/// The oldest `intent_version` major this binary reads. A config declaring an
/// earlier one is refused at [`Project::open`] as a config it cannot read
/// ([`ProjectError::PreV3`]).
pub const OLDEST_READ_MAJOR: u64 = 3;

/// The canon directory's name under the intent directory, spelt once for
/// [`CANON_DIR`] and the canon directories inside it (ST1c2db93 P7). A macro
/// rather than a `const` because `concat!` takes only literals.
macro_rules! canon_dir_name {
  () => {
    ".canon"
  };
}

/// `.canon`, every artefact's committed canon, relative to the intent
/// directory: the one spelling [`Project::canon_dir`] and each canon path take.
pub const CANON_DIR: &str = canon_dir_name!();

/// `.canon/st`, thread canon, relative to the intent directory: a thread's
/// file and its attachment directory sit in it.
pub const CANON_ST_DIR: &str = concat!(canon_dir_name!(), "/st");

/// `.canon/issues`, issue canon, relative to the intent directory.
pub const CANON_ISSUES_DIR: &str = concat!(canon_dir_name!(), "/issues");

/// One thread's canon path, relative to the intent directory (D57-1).
///
/// **ONE SPELLING, TWO CONSUMERS, AND THE SECOND ONE IS WHY THIS IS A
/// FUNCTION.** [`Project::thread_json`] resolves it against a root; the
/// exporter emits it as the name INSIDE the portable extract, promising that
/// "the paths are the real ones so a refusal names a file the operator can go
/// and look at". Those were two independent `format!` calls, and the issue arm
/// beside it had already shipped the resulting defect -- `issues/46.json`
/// written where every reader opened `issues/0046.json`. Two ends agreeing by
/// convention is how that happens; agreeing by construction is this.
pub fn canon_thread_rel(id: &str) -> String {
  format!("{CANON_ST_DIR}/{id}.json")
}

/// One OPAQUE attachment's canon path, relative to the intent directory
/// (ST0057 AC-03.1, D57-7).
///
/// **A function for the same reason [`canon_thread_rel`] is one, and the
/// consumers are further apart here**: the exporter names the file, `ingest`
/// opens it, and `organize` will one day copy it back onto disk. Three
/// `format!` calls agreeing by convention is exactly the shape that shipped
/// `issues/46.json`, and a disagreement here is worse than that one was -- an
/// unreadable path means the bytes are gone, not merely unfindable, because
/// the working copy is what dehydration removed.
///
/// **The thread's canon file and its attachment directory are SIBLINGS with
/// the same stem**: `.canon/st/ST0056.json` beside `.canon/st/ST0056/`. That
/// is not a collision -- one is a file and one is a directory -- and it keeps
/// a thread's whole record in one place a reader can list.
pub fn canon_blob_rel(id: &str, path: &str) -> String {
  format!("{}/{path}", canon_thread_dir_rel(id))
}

/// A thread's attachment directory, relative to the intent directory: the one
/// spelling of `.canon/st/<thread>`, which [`canon_blob_rel`] and
/// [`attachment_name`]'s containment test build from. It holds the opaque
/// attachments' bytes and nothing else, whatever the canon format.
pub fn canon_thread_dir_rel(id: &str) -> String {
  format!("{CANON_ST_DIR}/{id}")
}

/// Where a thread's records live in the release's format, relative to the
/// intent directory (ST0096 WP-05): `.canon/records/<thread>/`, one directory
/// per kind of child beneath it.
///
/// **NOT INSIDE THE THREAD'S ATTACHMENT DIRECTORY**, which holds an opaque
/// attachment's bytes at the attachment's own path. Sharing it would let an
/// attachment's path land on a record (`wp/notes.json` read as a package, a
/// file named `wp` against the directory, or `WP/03/x.png` inside `wp/` on a
/// disk that folds case). Reserving the record directories' names instead
/// would refuse an image under `WP/`, Intent's own convention. Two roots make
/// the collision impossible rather than refused.
pub const CANON_RECORDS_DIR: &str = concat!(canon_dir_name!(), "/records");

/// The directories of a thread's records in the release's format, under
/// [`CANON_RECORDS_DIR`] (ST0096 WP-05): one for each kind of child, named here once so
/// the writer and the reader cannot spell one two ways.
pub const CANON_WP_DIR: &str = "wp";
/// See [`CANON_WP_DIR`].
pub const CANON_AC_DIR: &str = "ac";
/// See [`CANON_WP_DIR`].
pub const CANON_AT_DIR: &str = "at";
/// See [`CANON_WP_DIR`].
pub const CANON_ATTACHMENTS_DIR: &str = "attachments";

/// One kind of a thread's records in the release's format, relative to the
/// intent directory: `.canon/records/<thread>/<dir>` (ST0096 WP-05).
pub fn canon_children_rel(thread: &str, dir: &str) -> String {
  format!("{}/{dir}", canon_records_rel(thread))
}

/// One thread's records directory, relative to the intent directory:
/// `.canon/records/<thread>`, which holds a directory per kind of child
/// (ST0096 WP-05). A renumber moves it whole.
pub fn canon_records_rel(thread: &str) -> String {
  format!("{CANON_RECORDS_DIR}/{thread}")
}

/// One work package's canon path in the release's format, relative to the
/// intent directory (ST0096 WP-05): its own file in its thread's directory,
/// named by its stored id, so two branches that change two packages of one
/// thread change two files.
pub fn canon_wp_rel(thread: &str, wp: &str) -> String {
  format!("{}/{wp}.json", canon_children_rel(thread, CANON_WP_DIR))
}

/// One criterion's canon path in the release's format, named by its label
/// (ST0096 WP-05).
pub fn canon_ac_rel(thread: &str, label: &str) -> String {
  format!("{}/{label}.json", canon_children_rel(thread, CANON_AC_DIR))
}

/// One acceptance test's canon path in the release's format, named by its
/// label (ST0096 WP-05).
pub fn canon_at_rel(thread: &str, label: &str) -> String {
  format!("{}/{label}.json", canon_children_rel(thread, CANON_AT_DIR))
}

/// One attachment's record in the release's format, named by its path
/// (ST0096 WP-05). An opaque attachment's bytes stay at [`canon_blob_rel`].
pub fn canon_attachment_rel(thread: &str, path: &str) -> String {
  format!(
    "{}/{path}.json",
    canon_children_rel(thread, CANON_ATTACHMENTS_DIR)
  )
}

/// The key two paths share when some disk stores them as ONE FILE (issue
/// 0711, cc's ruling of 2026-11-19). Two paths with one key must never both be
/// written.
///
/// **Built from the path's components**, so `a/./b.png`, `a//b.png` and
/// `a/b.png` are one key, as they are one file, and **each component is
/// Unicode-lowercased**, so `Design.png` and `design.png` are one key, as a
/// case-insensitive disk (macOS's default) stores them as one file.
///
/// **Asked on every file system, not only where the disk folds case**:
/// canon travels by git between macOS and Linux, so a pair harmless on the
/// disk that wrote it collides on the next clone made on a Mac.
///
/// # WHAT IT DOES NOT CATCH, AND WHAT IT CATCHES TOO MUCH, said out loud
///
/// - **Unicode normalisation.** Names that differ only as NFC against NFD
///   are one file on APFS and two keys here.
/// - **Full case folding.** Lowercasing is not folding: `ſ` against `s`, and
///   `ς` against `σ`, are two keys here although a folding disk may store
///   each pair as one file.
/// - **`..` is kept as written, never resolved**, because resolving it is only
///   lexical, so `x/../b.png` and `b.png` are two keys. The doors that admit a
///   name refuse one that is not normalised ([`attachment_name`]); canon read
///   back and a hand-built model are not checked for it, and this key does not
///   depend on it.
/// - **A name that is not UTF-8 is read lossily**, so two such names can share
///   a key: an over-match, which refuses where it should not, and never lets a
///   write through.
///
/// **Who asks it.** The migration's batch guard, the legacy bucket merge, the
/// prune's verdict on a bucket file, and [`Project::classify`] for the thread's
/// own files (issue 0719), and the attachment door and projection's sidecar
/// write (issue 0718). Issue 0722 moves the doors that read a live disk onto
/// it (`sync --to-store`'s merge, `organize`'s plan, doctor's attachment
/// drift and the rest it names); until it lands, those doors still compare
/// paths byte for byte. A door that compares two paths for "one file" by
/// itself is a second copy of this.
pub(crate) fn one_file_key(path: &Path) -> String {
  path
    .components()
    .filter(|part| !matches!(part, std::path::Component::CurDir))
    .map(|part| part.as_os_str().to_string_lossy().to_lowercase())
    .collect::<Vec<_>>()
    .join("/")
}

/// How a file on a live disk answers against a thread's attachment rows (issue
/// 0722, cc's meaning of 2026-11-20). Asked by the `--to-store` carry and by
/// `organize`, so neither compares two paths for "one file" itself; the
/// other doors that read a live disk (doctor's attachment drift, the to-store
/// preview, the attachment reads and realise) are 0722's later sites.
/// `Project::classify`'s own-file check (issue 0719) asks `one_file_key`
/// directly, since it answers a name against the thread's own files, not rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DiskMatch<'a> {
  /// A row spelled exactly as the file is.
  Exact(&'a str),
  /// A row whose spelling differs only as [`one_file_key`] folds, on a disk
  /// that folds case: the file IS that row's working copy, and the name stays
  /// the row's, since no rename is ever inferred.
  WorkingCopy(&'a str),
  /// A row whose spelling differs only as [`one_file_key`] folds, on a disk
  /// that does not fold: two files, which collide on the next clone made on a
  /// Mac. Refused or reported by name, never claimed.
  CasePair(&'a str),
  /// No row is this file.
  New,
}

/// Answer `disk` (a path relative to the thread's directory) against `rows`,
/// the thread's attachment paths: the exact row first, then the one-file key.
/// `folds` is whether the disk folds case ([`Project::disk_folds_case`]).
/// **PURE**, so both answers of `folds` are driven on any host.
pub(crate) fn disk_match<'a>(
  rows: impl IntoIterator<Item = &'a str>,
  disk: &str,
  folds: bool,
) -> DiskMatch<'a> {
  let key = one_file_key(Path::new(disk));
  let mut keyed = None;
  for row in rows {
    if row == disk {
      return DiskMatch::Exact(row);
    }
    if keyed.is_none() && one_file_key(Path::new(row)) == key {
      keyed = Some(row);
    }
  }
  match keyed {
    Some(row) if folds => DiskMatch::WorkingCopy(row),
    Some(row) => DiskMatch::CasePair(row),
    None => DiskMatch::New,
  }
}

/// The first two of `paths` that are one file on a disk that folds case
/// (issues 0718 and 0722), as `(held, path)`: `held` the earlier, `path` the
/// one that would land on it. Asked by the projection's sidecars, realise,
/// doctor's canon finding and a thread's canon layout
/// (`canon_format::encode_thread_files`), so none of them spells the comparison. Migration's
/// own collision check (`migrate.rs`, v2 files under v2 tooling) and the
/// attach door's one-against-many check (`disk_match`'s shape) keep theirs.
pub(crate) fn key_pair<'a>(paths: impl IntoIterator<Item = &'a str>) -> Option<(&'a str, &'a str)> {
  let mut seen: std::collections::HashMap<String, &'a str> = std::collections::HashMap::new();
  paths.into_iter().find_map(|path| {
    seen
      .insert(one_file_key(Path::new(path)), path)
      .map(|held| (held, path))
  })
}

/// Whether the disk folds case, as a door that only reports reads it (issue
/// 0722): the answer, or `None` with the finding that says it could not be
/// read, which the caller adds to its own. One home for the read, by
/// [`Project::disk_folds_case`]'s rule.
pub(crate) fn fold_or_finding(
  project: &Project,
) -> (Option<bool>, Option<crate::finding::Finding>) {
  match project.disk_folds_case() {
    Ok(folds) => (Some(folds), None),
    Err(cause) => (
      None,
      Some(crate::finding::Finding::new(
        project.relative(&project.intent_dir()),
        crate::finding::FindingClass::CaseFoldUnread,
        format!("whether this disk folds case could not be read: {cause}"),
      )),
    ),
  }
}

/// What a door that only reports says of `name`, which differs from `other`
/// only in case, by the fold it read (issue 0722): nothing where the disk folds,
/// since there they are one file; a case pair where it does not; and the
/// unread fold's own class where the fold is unknown. Either way neither is
/// taken for the other. `file` is where the finding points.
pub(crate) fn case_pair_finding(
  fold: Option<bool>,
  file: impl Into<String>,
  name: &str,
  other: &str,
) -> Option<crate::finding::Finding> {
  let (class, why) = match fold {
    Some(true) => return None,
    Some(false) => (
      crate::finding::FindingClass::CasePair,
      "and this disk keeps them as two files where a case-insensitive disk (macOS's default) keeps one",
    ),
    None => (
      crate::finding::FindingClass::CaseFoldUnread,
      "and whether this disk keeps them as one file or two could not be read",
    ),
  };
  Some(crate::finding::Finding::new(
    file,
    class,
    format!("`{name}` and `{other}` differ only in case, {why}, so neither is taken for the other"),
  ))
}

/// `path`'s one-file key, read from `root` (issue 0722): the key
/// [`disk_match`] compares, so a twin that differs in a directory's case
/// (`wp/01/info.md` beside `WP/01/info.md`) is found as one in a name's. A
/// path not under `root` has none.
pub(crate) fn key_from(root: &Path, path: &Path) -> Option<String> {
  Some(one_file_key(path.strip_prefix(root).ok()?))
}

/// Paths read from a live disk, by their one-file key (issue 0722): built once
/// per read of the tree, so finding a path's file spelled in another case is
/// one lookup rather than a walk of the tree for every row a sparse projection
/// leaves off disk. Whether a twin it finds is the path's own file or a second
/// one is the disk's fold, which the caller holds. `organize` keys absolute
/// paths from the project root, doctor project-relative ones from the empty
/// path.
pub(crate) struct DiskKeys<'a, P: AsRef<Path>> {
  root: &'a Path,
  by_key: std::collections::BTreeMap<String, Vec<&'a P>>,
}

impl<'a, P: AsRef<Path>> DiskKeys<'a, P> {
  pub(crate) fn new(paths: impl IntoIterator<Item = &'a P>, root: &'a Path) -> Self {
    let mut by_key: std::collections::BTreeMap<String, Vec<&'a P>> =
      std::collections::BTreeMap::new();
    for path in paths {
      if let Some(key) = key_from(root, path.as_ref()) {
        by_key.entry(key).or_default().push(path);
      }
    }
    Self { root, by_key }
  }

  /// Every path that is `path`'s by the key alone, spelled otherwise, in the
  /// order read: all of them, so none of a pair's other spellings is left to
  /// read as a file nothing carries.
  pub(crate) fn twins(&self, path: &Path) -> impl Iterator<Item = &'a P> {
    key_from(self.root, path)
      .and_then(|key| self.by_key.get(&key))
      .into_iter()
      .flatten()
      .copied()
      .filter(move |p| p.as_ref() != path)
  }
}

/// Why an attachment path cannot be carried (ST0057 AC-03.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadName {
  /// Its canon path would land outside the thread's own canon directory.
  EscapesCanon { would_be: String },
  /// It is not already normalised, so two different attachment paths would
  /// share one canon file and the second write would destroy the first.
  Collides { would_be: String },
  /// It cannot be addressed: the URL built from it does not parse back to the
  /// same path.
  NotAddressable { url: String, reason: String },
}

impl std::fmt::Display for BadName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::EscapesCanon { would_be } => write!(
        f,
        "its canon copy would be written to {would_be}, outside the thread's own directory"
      ),
      Self::Collides { would_be } => write!(
        f,
        "it is not already normalised: its canon copy would be written to {would_be}, which is \
         also where the attachment actually named that would go -- two paths, one file, and the \
         second write destroys the first"
      ),
      Self::NotAddressable { url, reason } => {
        write!(f, "it cannot be addressed -- {url} {reason}")
      }
    }
  }
}

/// Whether an attachment path can be given BOTH a canon path and a URL
/// (ST0057 AC-03.3, D57-8).
///
/// # It is DERIVED from the two consumers, never a list of bad characters
///
/// A hand-written character list is a second opinion about what those two
/// mechanisms accept, and a second opinion drifts the moment either of them
/// changes -- silently, and in the direction that admits a name one of them
/// cannot take. So each half is answered by ASKING the mechanism:
///
/// - **The canon half** builds the real path through [`canon_blob_rel`] and
///   requires it to stay inside the thread's own canon directory once `.` and
///   `..` are resolved. This is not a tidiness rule: hydration WRITES to that
///   path, so a name that escapes it is a write outside the thread, chosen by
///   whoever named the file.
/// - **The URL half** builds `intent:///threads/{id}/attachments/{path}` and
///   requires [`crate::address::parse`] to return the SAME path. Round-tripping
///   through the real parser is what stops this rule and the addressing layer
///   disagreeing: a path that survives the trip is addressable by construction,
///   and one that does not is rejected for the parser's own reason rather than
///   for a reason invented here.
///
/// # Lexical resolution, deliberately, and it is the stricter answer
///
/// `..` is resolved textually rather than by asking the filesystem. A
/// filesystem answer depends on what exists at the moment of asking -- a
/// symlink, a directory not yet created -- so the same name could be legal on
/// one machine and not another, and the check would pass on the estate that
/// wrote it and refuse on the clone that received it.
///
/// # WHAT THIS DOES NOT CATCH, said out loud
///
/// **The URL half is exactly as strict as [`crate::address::parse`], which is
/// the point and also the limit.** Measured against the accepted set: a name
/// containing a SPACE, a `#`, or a control character such as a newline is
/// ACCEPTED, because the parser takes them. A `#` would truncate the address in
/// any standard URL reader, and a newline breaks every line-oriented tool that
/// ever handles the path.
///
/// **That is not a hole to patch here.** Adding a character list would rebuild
/// the second opinion this function exists to avoid, and it would then disagree
/// with the addressing layer in the direction where a name is refused although
/// the parser would happily take it. **If those names should be refused, the
/// parser is where it belongs -- and then both consumers gain it at once, which
/// is the whole return on deriving the rule.** Reported to whoever owns D57-8
/// rather than worked around.
///
/// **So a green from this function means "canon and the addressing layer can
/// both take this name", and does NOT mean "this name is safe in a shell".**
/// The two are different claims and only the first is being made.
pub fn attachment_name(thread: &str, path: &str) -> Result<(), BadName> {
  let rel = canon_blob_rel(thread, path);
  let prefix = format!("{}/", canon_thread_dir_rel(thread));

  // Resolve `.` and `..` lexically. A leading `..` pops past the prefix and the
  // containment test below fails, which is the whole point.
  let mut resolved: Vec<&str> = Vec::new();
  for part in rel.split('/') {
    match part {
      "." | "" => {}
      ".." => {
        resolved.pop();
      }
      other => resolved.push(other),
    }
  }
  let resolved = resolved.join("/");
  // **RESOLUTION MUST BE A NO-OP, not merely land inside the directory, and the
  // difference is a collision rather than a nicety.** `a/./b.md` resolves to
  // `a/b.md`, which is contained -- and is the SAME canon file as the
  // attachment actually named `a/b.md`. Two distinct attachment paths, one
  // sidecar, and the second write silently destroys the first. Requiring the
  // path to be already-normalised catches that, and catches `..`, `//`,
  // a trailing separator and the empty path in the same rule instead of four.
  //
  // Found by probing the accepted set rather than by reading this function: the
  // containment test passed `a/./b.md` and looked correct doing it.
  // **CONTAINMENT FIRST, and the order is not cosmetic.** Both tests fire for
  // `../x.md`, and only one of them describes it: it escapes the thread, and it
  // also happens to be un-normalised. Answering "collision" for a path
  // traversal sends an operator looking for a duplicate that does not exist,
  // and buries the write-outside-the-thread finding under a tidiness one.
  //
  // The first version had these the other way round and every escape in the
  // probe set came back mislabelled -- correct refusals, wrong reasons, which
  // is the shape a reader trusts and acts on.
  if !resolved.starts_with(&prefix) || resolved.len() == prefix.len() {
    return Err(BadName::EscapesCanon { would_be: resolved });
  }
  // **Reported as a COLLISION and not as an escape, because `a/./b.md` resolves
  // INSIDE the directory.** Calling that "outside the thread" would describe a
  // fault the file does not have.
  if resolved != rel {
    return Err(BadName::Collides { would_be: resolved });
  }

  let url = crate::address::Entity::Attachment {
    thread: thread.to_string(),
    path: path.to_string(),
  }
  .local_url();
  match crate::address::parse(&url) {
    Ok(address) => match &address.entity {
      crate::address::Entity::Attachment {
        path: round_tripped,
        ..
      } if round_tripped == path => Ok(()),
      crate::address::Entity::Attachment {
        path: round_tripped,
        ..
      } => Err(BadName::NotAddressable {
        url,
        reason: format!("reads back as {round_tripped:?} rather than {path:?}"),
      }),
      other => Err(BadName::NotAddressable {
        url,
        reason: format!("reads back as {other:?} rather than as an attachment"),
      }),
    },
    Err(e) => Err(BadName::NotAddressable {
      url,
      reason: format!("does not parse: {e}"),
    }),
  }
}

/// One issue's canon path, relative to the intent directory. Zero-padded,
/// which is the half that was wrong before.
/// The project-state canon file, `.canon/project.json`.
///
/// A path function beside the others rather than a literal at the one call
/// site: the openness declaration on the `project` table names this path, and
/// `tests/openness.rs` resolves every declared path against the tree.
pub fn canon_project_rel() -> String {
  format!("{CANON_DIR}/project.json")
}

pub fn canon_issue_rel(id: crate::id::IssueId) -> String {
  format!("{CANON_ISSUES_DIR}/{}.json", id.stored())
}

/// `major.minor.patch`, ignoring any pre-release suffix.
///
/// Returns `None` rather than guessing: a version that does not parse is not
/// evidence of a version before [`OLDEST_READ_MAJOR`], so [`Project::open`]
/// does not refuse on it.
fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
  let core = text.split(['-', '+']).next()?;
  let mut parts = core.split('.').map(str::parse::<u64>);
  match (parts.next(), parts.next(), parts.next()) {
    (Some(Ok(major)), Some(Ok(minor)), Some(Ok(patch))) => Some((major, minor, patch)),
    _ => None,
  }
}

/// What a bare repository is to a project, in the words every refusal of one
/// uses (issue 0609): `ProjectError`'s message, and the registry's label for a
/// root it will not register.
pub const BARE_REPOSITORY: &str =
  "a bare git repository, which has no working tree to hold a project";

/// The bare git repository `root` is, or sits inside, if there is one (issue
/// 0609).
///
/// **STRUCTURAL, WITH NO `git` SPAWNED.** [`Project::open`] runs on every
/// command, the PostToolUse hooks' `intent critic` and `intent search` among
/// them, so a subprocess here would be paid on every tool call in every
/// session. A bare repository is what `git init --bare` writes: `HEAD`,
/// `objects/` and `refs/` together, and a config whose `[core]` says
/// `bare = true`. A working copy's `.git` directory holds the same three and
/// says `bare = false`, so it never answers here.
///
/// **THE WALK STOPS AT THE FIRST `.git` ENTRY, AS GIT'S OWN DISCOVERY DOES.** A
/// `.git` directory marks a working copy and a `.git` file a linked worktree,
/// and both are where a project belongs -- a worktree kept inside a bare
/// repository's directory included, the bare-plus-worktrees layout, whose bare
/// ancestor is the repository it was made from and not the place it lives.
pub fn bare_repository(root: &Path) -> Option<PathBuf> {
  for dir in root.ancestors() {
    if dir.join(".git").exists() {
      return None;
    }
    if is_bare_repository(dir) {
      return Some(dir.to_path_buf());
    }
  }
  None
}

fn is_bare_repository(dir: &Path) -> bool {
  dir.join("HEAD").is_file()
    && dir.join("objects").is_dir()
    && dir.join("refs").is_dir()
    && std::fs::read_to_string(dir.join("config")).is_ok_and(|config| declares_bare(&config))
}

/// Whether a git config's `[core]` section sets `bare` to true, in any of git's
/// spellings of a true boolean. Section and key names are case-insensitive to
/// git, and a value may carry quotes or a trailing comment.
fn declares_bare(config: &str) -> bool {
  let mut in_core = false;
  for line in config.lines().map(str::trim) {
    if let Some(header) = line.strip_prefix('[') {
      let name = header.split_once(']').map_or(header, |(name, _)| name);
      in_core = name.trim().eq_ignore_ascii_case("core");
      continue;
    }
    let Some((key, value)) = line.split_once('=') else {
      continue;
    };
    if in_core && key.trim().eq_ignore_ascii_case("bare") {
      let value = value.split(['#', ';']).next().unwrap_or_default();
      return matches!(
        value.trim().trim_matches('"').to_ascii_lowercase().as_str(),
        "true" | "yes" | "on" | "1"
      );
    }
  }
  false
}

/// Whether a project-relative path matches one of `patterns`, read with the
/// search surfaces' glob, `*` within one segment: the one matcher behind
/// [`Project::generated_view_matcher`] and [`Project::aggregate_view_matcher`].
fn glob_matcher(patterns: Vec<String>) -> impl Fn(&str) -> bool + use<> {
  move |rel: &str| {
    patterns
      .iter()
      .any(|pattern| crate::search::glob_matches(pattern, rel))
  }
}

/// The threads' directory, relative to the intent directory, spelt once for
/// [`Project::st_dir`] and the index view inside it. A macro rather than a
/// `const` because `concat!` takes only literals.
macro_rules! st_dir_name {
  () => {
    "st"
  };
}

/// The thread index view's file name, spelt once for [`STEEL_THREADS_VIEW_NAME`]
/// and the view's path. A macro for the reason `st_dir_name!` is one.
macro_rules! steel_threads_view_name {
  () => {
    "steel_threads.md"
  };
}

/// The thread index view's file name (ST1c2db93 P7).
pub const STEEL_THREADS_VIEW_NAME: &str = steel_threads_view_name!();

/// The thread index view, relative to the intent directory: beside the threads
/// it indexes, at v2's path (vc ruling, 2026-08-14).
const STEEL_THREADS_VIEW: &str = concat!(st_dir_name!(), "/", steel_threads_view_name!());

/// The flat DOING / TODO / DONE view, relative to the intent directory.
pub const TODO_VIEW: &str = "todo.md";

/// The generated cover's file name, a thread's and a work package's alike
/// (ST1c2db93 P7): the one spelling every path to a cover is built from.
pub const INFO_VIEW: &str = "info.md";

/// A thread's generated acceptance contract's file name (ST1c2db93 P7).
pub const ACCEPTANCE_VIEW: &str = "acceptance.md";

/// `WP`, a work package's directory under its thread's (ST1c2db93 P7): the
/// one spelling [`Project::wp_rel`] and every package path pattern take.
pub const WP_DIR: &str = "WP";

/// The INDEX views, relative to the intent directory: each is rendered from
/// every record at once, so neither names a single artefact. `organize` keeps
/// them, and `init` writes them, so its occupied check reads them here, before
/// a `Project` exists.
pub const INDEX_VIEWS: [&str; 2] = [STEEL_THREADS_VIEW, TODO_VIEW];

/// A resolved project.
#[derive(Debug, Clone)]
pub struct Project {
  root: PathBuf,
  config: Config,
}

impl Project {
  /// `intent/.config/config.json` under `root`. The marker file's location is
  /// FIXED -- it is what identifies a project, so it cannot itself be read
  /// from the project's config.
  pub fn config_path(root: &Path) -> PathBuf {
    root.join("intent").join(".config").join("config.json")
  }

  /// Open the project rooted exactly at `root`.
  ///
  /// A root that [`bare_repository`] places in a bare git repository is refused
  /// before its config is read (issue 0609). Every door opens through here --
  /// the CLI's gate by way of [`Project::discover`], the daemon and the
  /// registry -- so a project an `intent init` once wrote into a bare
  /// repository is inert at all of them.
  ///
  /// A config declaring an `intent_version` before [`OLDEST_READ_MAJOR`] is
  /// refused here too, as a config this binary cannot read
  /// ([`ProjectError::PreV3`]), so every door refuses it the same way.
  pub fn open(root: &Path) -> Result<Self, ProjectError> {
    if let Some(repository) = bare_repository(root) {
      return Err(ProjectError::BareRepository(
        repository.display().to_string(),
      ));
    }
    let path = Self::config_path(root);
    let text = std::fs::read_to_string(&path).map_err(|source| ProjectError::Io {
      path: path.display().to_string(),
      source,
    })?;
    let config: Config = serde_json::from_str(&text).map_err(|source| ProjectError::Config {
      path: path.display().to_string(),
      source,
    })?;
    if parse_version(&config.intent_version).is_some_and(|(major, _, _)| major < OLDEST_READ_MAJOR)
    {
      return Err(ProjectError::PreV3 {
        path: path.display().to_string(),
        declared: config.intent_version,
      });
    }
    Ok(Self {
      root: root.to_path_buf(),
      config,
    })
  }

  /// Walk up from `start` to the first directory holding an Intent config.
  ///
  /// The marker is the config file's own presence -- never an environment
  /// variable. A generic env var answers "did someone set a variable?" while
  /// meaning "am I in a project?", and those agree often enough to look correct
  /// and differ exactly when it matters (issue 0025).
  pub fn discover(start: &Path) -> Result<Self, ProjectError> {
    for dir in start.ancestors() {
      if Self::config_path(dir).is_file() {
        return Self::open(dir);
      }
    }
    Err(ProjectError::NotFound(start.display().to_string()))
  }

  pub fn root(&self) -> &Path {
    &self.root
  }

  /// The name of the directory this project lives in.
  ///
  /// **THE DIRECTORY NAME, NOT `config.project_name`** (hv, 2026-09-03, ruled
  /// on the `explore` info row and applied here unchanged). The configured name
  /// cannot discriminate two checkouts of one project -- `Intentv2` declares
  /// itself `Intent` -- and telling those apart is the question an operator has
  /// when they look at output and ask which estate produced it.
  ///
  /// **ONE HOME, BECAUSE THE SECOND CALLER ARRIVED AND PROVED IT WAS A RULE
  /// RATHER THAN A LINE.** `explore` resolved this inline; `st list` now asks
  /// the same question of the same object, and a second copy of a four-line
  /// resolution carrying a ruling in its comment is how the ruling comes to be
  /// stated in one place and quietly not applied in the other.
  ///
  /// **`None` RATHER THAN A PLACEHOLDER**, so each caller owns its own
  /// fallback: a root with no final component (`/`) is a real state, and the
  /// right answer differs by surface -- `explore` renders an empty corner,
  /// `st list` keeps its `ID` heading. Inventing one here would force both.
  /// **NOT the same question as `intent init`'s** (`render.rs`), which asks
  /// what a NEW project should be CALLED and falls back to `project`; that one
  /// reads the cwd and has no `Project` to ask.
  pub fn directory_name(&self) -> Option<String> {
    self
      .root
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
  }

  /// This project's spelling of [`relative`].
  pub fn relative(&self, path: &Path) -> String {
    relative(&self.root, path)
  }

  pub fn config(&self) -> &Config {
    &self.config
  }

  /// Keep this handle in step after [`crate::canon::relayout`] has written a new
  /// `contracts` key to disk (ST0083), so a facade that moved the contracts does
  /// not go on answering from the layout it left.
  pub(crate) fn set_contracts(&mut self, contracts: Contracts) {
    self.config.contracts = contracts;
  }

  /// Whether the disk holding this project folds case (issue 0722), so two
  /// names that share `one_file_key` are one file there.
  ///
  /// **ASKED BY READING, NEVER BY WRITING** (cc's meaning), so `doctor
  /// --read-only` stays a read: the nearest directory at or above the intent
  /// directory whose name has a letter that changes case, on the intent
  /// directory's own device, is looked up under its own name and under that
  /// name with its case flipped, and the disk folds when both name one file
  /// (one device and one inode).
  ///
  /// **AN UNREAD FOLD NEVER LICENSES A WRITE ON A GUESS**, and this is the one
  /// statement of that rule for every door that asks: a door that writes
  /// refuses (`organize`), and a door that only carries reports and carries
  /// nothing it cannot place (the `--to-store` carry). Never a guess either way.
  ///
  /// **ONE READ PER PROJECT, AND ITS TWO KNOWN LIMITS FAIL TOWARD A REPORTED PAIR,
  /// NEVER A LOST ROW** (cc's meaning, 2026-11-20, vc's yes). (a) ext4 folds case
  /// per directory (`chattr +F`), so a folding `intent/st` under a parent that
  /// does not fold reads as not folding. (b) An intent directory symlinked from
  /// a case-sensitive volume into a folding one is looked up beside the link,
  /// and reads as not folding. Both read "does not fold", where nothing is
  /// written over and nothing is dropped silently: the cost is a pair finding
  /// the disk would not make. A read per directory comes when a seat meets one.
  pub fn disk_folds_case(&self) -> std::io::Result<bool> {
    use std::os::unix::fs::MetadataExt;
    let dir = self.intent_dir();
    let device = std::fs::metadata(&dir)?.dev();
    let flip = |name: &str| -> String {
      name
        .chars()
        .flat_map(|c| -> Vec<char> {
          match c.is_lowercase() {
            true => c.to_uppercase().collect(),
            false => c.to_lowercase().collect(),
          }
        })
        .collect()
    };
    let probe = dir.ancestors().find_map(|p| {
      let name = p.file_name()?.to_str()?;
      let flipped = flip(name);
      (flipped != name).then(|| (p.to_path_buf(), p.with_file_name(flipped)))
    });
    let Some((real, twin)) = probe else {
      return Err(std::io::Error::other(format!(
        "no directory at or above `{}` has a letter whose case can flip, so nothing shows whether this disk folds case",
        dir.display()
      )));
    };
    let real = std::fs::metadata(&real)?;
    if real.dev() != device {
      return Err(std::io::Error::other(format!(
        "the nearest directory above `{}` whose case can flip is on another device, so it cannot show whether this disk folds case",
        dir.display()
      )));
    }
    match std::fs::metadata(twin) {
      Ok(twin) => Ok(twin.dev() == real.dev() && twin.ino() == real.ino()),
      Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(false),
      Err(source) => Err(source),
    }
  }

  /// `intent/` -- configurable in v2, so read rather than assumed.
  pub fn intent_dir(&self) -> PathBuf {
    self.root.join(&self.config.intent_dir)
  }

  pub fn st_dir(&self) -> PathBuf {
    self.intent_dir().join(st_dir_name!())
  }

  /// `intent/st/<ID>/` -- a thread's directory, whether or not it exists.
  pub fn thread_dir(&self, id: &str) -> PathBuf {
    self.st_dir().join(id)
  }

  /// `intent/.intentfiles` -- the realisation manifest (AC-02.1..02.5).
  ///
  /// **Under `intent_dir`, so it MOVES with a configured one**, which is the
  /// difference between it and [`Project::config_path`]. That one is a fixed
  /// bootstrap point by design -- something has to be findable before anything
  /// is configured. Nothing has to find the manifest before the project is
  /// open, so it has no claim on a fixed location and takes the ordinary rule
  /// instead. `Fixture::with_intent_dir` is what catches the alternative: a
  /// site spelling `intent/.intentfiles` itself lands somewhere else entirely.
  pub fn intentfiles_path(&self) -> PathBuf {
    self.intent_dir().join(".intentfiles")
  }

  /// `intent/.canon/` -- every artefact's committed structured canon (D57-1).
  ///
  /// **A DOT DIRECTORY THAT MUST BE COMMITTED, AND IT IS THE ONLY ONE.** The
  /// siblings beside it -- `.cache/`, `.backup/` -- are
  /// every one of them gitignored, so `intent/.<x>/` currently reads as "local,
  /// never travels". This breaks that pattern deliberately: hv's requirement is
  /// that `intent/st` stop holding a bajillion files, and D29 requires canon to
  /// travel. **A future tidy-up adding `intent/.*/` to `.gitignore` would
  /// silently un-commit the entire estate**, which is why AC-01.2 checks by
  /// CLONING rather than by reading the ignore file: the question is what git
  /// DOES, not what a rule appears to say.
  pub fn canon_dir(&self) -> PathBuf {
    self.intent_dir().join(CANON_DIR)
  }

  /// `intent/whiteboard/` -- the coordination estate, one directory per node.
  pub fn whiteboard_dir(&self) -> PathBuf {
    self.intent_dir().join("whiteboard")
  }

  /// One node's board, `intent/whiteboard/<node>/board.json`.
  ///
  /// **NOT under `.canon/`, and that is the D30 table's own choice rather than
  /// an inconsistency.** A thread's canon sits apart from its rendered views
  /// because the views are many files in a directory of their own; a board is
  /// one node's whole state, and it belongs beside the markdown a reader of
  /// that node already opens. The file is still canon in every other sense:
  /// tool-written, validated strictly on ingest, and the durable form the
  /// extract carries.
  /// The board's team declaration, `whiteboard/declaration.json`, beside the
  /// node directories (ST0092 WP-03). A file at the top level is never read as a
  /// node, since a node is a directory holding a `board.json`.
  pub fn declaration_json(&self) -> PathBuf {
    self.whiteboard_dir().join("declaration.json")
  }

  pub fn board_json(&self, node: &str) -> PathBuf {
    self.whiteboard_dir().join(node).join("board.json")
  }

  /// A node's board as the markdown view, `whiteboard/<node>/wip.md`.
  pub fn wb_board_view(&self, node: &str) -> PathBuf {
    self.whiteboard_dir().join(node).join("wip.md")
  }

  /// The verbatim copy `wb migrate` keeps of one of a node's hand-authored
  /// files, `whiteboard/<node>/.history/pre-migration/<file>`: its `wip.md`,
  /// and each inbox holding a line the model cannot carry (issue 0438).
  ///
  /// **UNDER `.history/` SO IT IS A SNAPSHOT LIKE EVERY FOLD'S ARCHIVE**, read
  /// and carried by the same walk, and undated because a board is migrated once.
  /// **THE COPY KEEPS ITS SOURCE'S NAME**, so no second spelling of a board's
  /// file names exists to disagree with the directory it was read from.
  pub fn wb_pre_migration_snapshot(&self, node: &str, file: &str) -> PathBuf {
    self
      .whiteboard_dir()
      .join(node)
      .join(".history")
      .join("pre-migration")
      .join(file)
  }

  /// One ordered pair's inbox view, `whiteboard/<recipient>/inbox.<sender>.md`:
  /// it sits with the recipient, who reads and clears it.
  pub fn wb_inbox_view(&self, recipient: &str, sender: &str) -> PathBuf {
    self
      .whiteboard_dir()
      .join(recipient)
      .join(format!("inbox.{sender}.md"))
  }

  /// Every node directory that holds a `board.json`, sorted.
  ///
  /// **A NODE DIRECTORY WITHOUT ONE IS NOT LISTED, AND THAT IS THE WHOLE
  /// POINT.** Every board on this estate is hand-authored markdown until the
  /// cutover, so most node directories hold no board file and never have; a
  /// walk that listed them anyway would hand the reader a path it then had to
  /// treat as missing. An absent whiteboard directory is the same answer as an
  /// empty one -- a project that has never run the protocol has no board, which
  /// is not a defect to report.
  pub fn board_nodes(&self) -> Result<Vec<String>, ProjectError> {
    let Ok(entries) = std::fs::read_dir(self.whiteboard_dir()) else {
      return Ok(Vec::new());
    };
    let mut nodes: Vec<String> = entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.join("board.json").is_file())
      .filter_map(|p| p.file_name().and_then(|s| s.to_str()).map(str::to_string))
      .collect();
    nodes.sort();
    Ok(nodes)
  }

  /// The committed structured canon for one thread, `.canon/st/<ID>.json`.
  ///
  /// **One file per artefact, not one consolidated file** (D57-1 rejects
  /// option B). Four nodes commit into this estate; a single `threads.jsonl`
  /// is a merge-conflict generator, and per-artefact means two nodes editing
  /// two threads touch two paths and never collide.
  pub fn thread_json(&self, id: &str) -> PathBuf {
    self.intent_dir().join(canon_thread_rel(id))
  }

  /// `.canon/events/` -- the committed event files, one per event, under
  /// dated directories (ST0078 P1).
  pub fn events_dir(&self) -> PathBuf {
    self.canon_dir().join(crate::event::EVENTS_DIR)
  }

  /// Where one written event's committed file lives.
  pub fn event_file(
    &self,
    e: &crate::event::Envelope,
  ) -> Result<PathBuf, crate::event::EventFileError> {
    Ok(self.events_dir().join(crate::event::file_rel(e)?))
  }

  /// `.canon/st/` -- thread canon, flat, one file per thread.
  pub fn canon_st_dir(&self) -> PathBuf {
    self.intent_dir().join(CANON_ST_DIR)
  }

  /// `.canon/st/<ID>/` -- a thread's attachment directory in canon,
  /// [`canon_thread_dir_rel`] under the intent directory, whether or not it
  /// exists.
  pub fn canon_thread_dir(&self, id: &str) -> PathBuf {
    self.intent_dir().join(canon_thread_dir_rel(id))
  }

  /// The committed project state, `.canon/project.json`.
  pub fn project_json(&self) -> PathBuf {
    self.intent_dir().join(canon_project_rel())
  }

  /// `.canon/issues/` -- issue canon.
  ///
  /// **The whole directory moved, because at the time the whole directory WAS
  /// canon.** `intent/issues/` held nothing but `<nnnn>.json`.
  ///
  /// **THE SECOND HALF OF THAT SENTENCE -- _unlike a thread, an issue has no
  /// realised markdown to leave behind_ -- WAS TRUE UNTIL ST0069 WP-01 AND IS
  /// NOT ANY MORE.** An issue now realises to [`Self::issue_view`], so
  /// `intent/issues/` is populated again, by the renderer this time rather
  /// than by canon. The two are different directories doing different jobs and
  /// the names no longer tell them apart on sight, which is why this says so.
  pub fn issues_dir(&self) -> PathBuf {
    self.intent_dir().join(CANON_ISSUES_DIR)
  }

  /// `intent/issues/` -- where an issue's generated view is realised.
  ///
  /// Deliberately NOT [`Self::issues_dir`], which is canon under `.canon/`.
  pub fn issues_view_dir(&self) -> PathBuf {
    self.intent_dir().join("issues")
  }

  /// What a file under a thread's directory IS, given its path relative to
  /// that directory.
  ///
  /// **One classifier, because the alternative is three lists that drift.**
  /// Ingest needs to know what to carry, `doctor` needs to know what is
  /// uncovered, and the renderer needs to know what it owns -- and the moment
  /// those are three separate answers, a file can be an attachment to one and
  /// a view to another. Every caller asks here.
  pub fn classify(rel: &Path) -> ThreadFile {
    // **THE THREAD'S OWN FILES ARE MATCHED THROUGH THE ONE-FILE KEY** (issue
    // 0719), so a name a disk folding case stores as one of them is never an
    // attachment: matched by exact string, `Acceptance.md` was carried, and the
    // next render on macOS wrote the view over the author's file.
    let plain: PathBuf = rel
      .components()
      .filter(|part| !matches!(part, std::path::Component::CurDir))
      .collect();
    let key = one_file_key(&plain);
    let own = match key.split('/').collect::<Vec<_>>().as_slice() {
      // A generated view is checked FIRST and by its full shape, not its name.
      // `info.md` at the thread root is the generated cover; `WP/01/info.md` is
      // the work package's. A file called `info.md` three levels down under a
      // parity directory is neither, and matching on the bare name would take
      // an author's file and call it ours.
      //
      // The package arm's key is `"wp"`, not [`WP_DIR`]: the key is the
      // one-file key, lowercased, so `WP/`, `wp/` and `Wp/` all arrive as
      // `wp`. The path it answers is built from `WP_DIR`, the one spelling.
      [INFO_VIEW] => Some((PathBuf::from(INFO_VIEW), ThreadFile::GeneratedView)),
      [ACCEPTANCE_VIEW] => Some((PathBuf::from(ACCEPTANCE_VIEW), ThreadFile::GeneratedView)),
      ["wp", _, INFO_VIEW] => plain.components().nth(1).map(|seq| {
        (
          Path::new(WP_DIR).join(seq).join(INFO_VIEW),
          ThreadFile::GeneratedView,
        )
      }),
      // **NOT dead after D57-1's relocation, and worth saying so.** Canon now
      // lives at `.canon/st/<ID>.json`, so no healthy thread directory holds a
      // `thread.json` -- but a v2 tree does, and so does a tree caught
      // mid-move. Classifying it as canon is what keeps it out of the
      // attachment carry; deleting the arm would make a stale canon file an
      // ATTACHMENT and carry the migrator's own output back into canon as
      // though a human wrote it.
      ["thread.json"] => Some((PathBuf::from("thread.json"), ThreadFile::Canon)),
      _ => None,
    };
    match own {
      Some((of, file)) if of == plain => file,
      Some((of, _)) => ThreadFile::CaseVariant { of },
      // **Everything the renderer does not own is the author's.** No extension
      // is consulted, because an extension answers a question about a NAME and
      // the question here is whose file this is. Whether its bytes fit in canon
      // is a separate question with a separate home
      // ([`within_attachment_cap`]), asked where the bytes are.
      None => ThreadFile::Attachment,
    }
  }

  /// What every door says of a [`ThreadFile::CaseVariant`], after the path it
  /// names (issue 0719): the one sentence, so doctor's finding, the carry's
  /// refusal and each door's error cannot describe it two ways. The remedy is
  /// [`crate::finding::FindingClass::CaseVariant`]'s.
  pub fn case_variant_detail(of: &Path) -> String {
    format!(
      "is one file with the thread's own `{}` on a disk that folds case (macOS's default), so no door carries, renames or claims it",
      of.display()
    )
  }

  /// [`Project::case_variant_detail`] with its remedy, the whole refusal a
  /// door gives that reports it as text rather than as a
  /// `FacadeError::CaseVariant`: the carry, legacy's prune, the explorer.
  pub fn case_variant_refusal(of: &Path) -> String {
    format!(
      "{} -- {}",
      Project::case_variant_detail(of),
      crate::finding::FindingClass::CaseVariant.remedy()
    )
  }

  /// The committed structured canon for one issue, `.canon/issues/<nnnn>.json`.
  pub fn issue_json(&self, id: crate::id::IssueId) -> PathBuf {
    self.intent_dir().join(canon_issue_rel(id))
  }

  /// The per-machine directory beside the canon (D21): the store and every
  /// file kept with it, ignored and never committed. One home for its path.
  pub fn cache_dir(&self) -> PathBuf {
    self.intent_dir().join(".cache")
  }

  /// The SQLite db (D21) -- gitignored, and the durable SSOT rather than a
  /// cache. Gitignored is about TRANSPORT, not authority: the committed
  /// extract is how the estate travels, and the store is re-created from it on
  /// a fresh clone.
  pub fn db_path(&self) -> PathBuf {
    self.cache_dir().join("intent.db")
  }

  /// The manifest of the paths the last `intent upgrade` wrote or removed, for
  /// `git add --pathspec-from-file` (issue 0745): beside the store, ignored
  /// with it, per-machine and never committed.
  pub fn upgrade_paths(&self) -> PathBuf {
    self.cache_dir().join("upgrade.paths")
  }

  /// The lock a board write holds from reading the store to recording what it
  /// landed (issue 0662), beside the store it serialises and gitignored with it.
  pub fn board_lock_path(&self) -> PathBuf {
    self.cache_dir().join("board.lock")
  }

  /// Where a language's level-3 toolchain builds (ST0076 WP-05, vc decision 25
  /// (6)): a directory of Intent's own beside the store, and never the
  /// project's build directory, so resolving a project leaves its own build
  /// artefacts as they were.
  pub fn resolve_cache_dir(&self, lang: &str) -> PathBuf {
    self.cache_dir().join("resolve").join(lang)
  }

  /// The thread index view, one of [`INDEX_VIEWS`]. Beside the threads it
  /// indexes, at v2's path (vc ruling, 2026-08-14): a rewrite that already moves
  /// plenty does not need to move this too.
  pub fn steel_threads_view(&self) -> PathBuf {
    self.intent_dir().join(STEEL_THREADS_VIEW)
  }

  /// The flat DOING / TODO / DONE view, one of [`INDEX_VIEWS`].
  pub fn todo_view(&self) -> PathBuf {
    self.intent_dir().join(TODO_VIEW)
  }

  /// Every one of [`INDEX_VIEWS`], under this project's intent directory.
  pub fn index_views(&self) -> [PathBuf; 2] {
    INDEX_VIEWS.map(|view| self.intent_dir().join(view))
  }

  /// The AGGREGATE views, as project-relative patterns whose `*` is one path
  /// segment: the index views and every thread's acceptance view, each rendered
  /// from many records.
  ///
  /// **THE ONE DECLARATION OF WHAT NO CLONE COMMITS (ST0096, hv decisions 66 and
  /// 74).** A change to one record rewrites an aggregate view, so two branches
  /// that change different records both write it, and a forge cannot merge the
  /// second. So every clone renders its own from its store, and git ignores
  /// them. The `.gitignore` writer, the upgrade's untrack, doctor's advisory,
  /// the verbs' `wrote:` lines and the commit gate read the set here. Each
  /// pattern is derived from the method that makes the real path, with `*` for
  /// the thread, as [`Self::generated_view_patterns`] derives its own.
  pub fn aggregate_view_patterns(&self) -> Vec<String> {
    let rel = |p: PathBuf| self.relative(&p);
    let mut patterns: Vec<String> = self.index_views().into_iter().map(rel).collect();
    patterns.push(rel(self.acceptance_view("*")));
    patterns
  }

  /// Whether a project-relative path is one of the aggregate views, matched as
  /// [`Self::generated_view_matcher`] matches, `*` within one segment.
  pub fn aggregate_view_matcher(&self) -> impl Fn(&str) -> bool + use<> {
    glob_matcher(self.aggregate_view_patterns())
  }

  /// A thread's generated cover.
  pub fn info_view(&self, id: &str) -> PathBuf {
    self.thread_dir(id).join(INFO_VIEW)
  }

  /// A work package's directory relative to its thread's, `WP/<NN>`: the one
  /// spelling every package path is built from.
  pub fn wp_rel(seq: crate::id::WpId) -> PathBuf {
    PathBuf::from(WP_DIR).join(seq.stored())
  }

  /// `intent/st/<ID>/WP/<NN>/` -- a work package's directory, whether or not it
  /// exists.
  pub fn wp_dir(&self, id: &str, seq: crate::id::WpId) -> PathBuf {
    self.thread_dir(id).join(Self::wp_rel(seq))
  }

  /// A work package's generated cover, `st/<ID>/WP/<NN>/info.md`.
  pub fn wp_info_view(&self, id: &str, seq: crate::id::WpId) -> PathBuf {
    self.wp_dir(id, seq).join(INFO_VIEW)
  }

  /// A thread's generated acceptance contract + coverage map.
  pub fn acceptance_view(&self, id: &str) -> PathBuf {
    self.thread_dir(id).join(ACCEPTANCE_VIEW)
  }

  /// One issue's generated view, `issues/<nnnn>.md` (AC-01.1).
  ///
  /// **The same stored form the canon path uses** ([`canon_issue_rel`]):
  /// `issues/46.json` once shipped where every reader opened
  /// `issues/0046.json`, because two ends formatted the same number
  /// independently. Both now ask [`crate::id::IssueId::stored`], and the pair
  /// is still pinned by a test.
  pub fn issue_view(&self, id: crate::id::IssueId) -> PathBuf {
    self.issues_view_dir().join(format!("{}.md", id.stored()))
  }

  /// **THE ONE ANSWER TO WHICH THREAD OR ISSUE OWNS A PATH** (ST0095 WP-14).
  ///
  /// A thread owns its realised directory and everything in it (its cover,
  /// its acceptance view, its work packages' covers, its attachments), and its
  /// canon record with everything under `.canon/st/<ID>/` beside it. An issue
  /// owns its canon record and its view. Nothing else is owned by a thread or
  /// an issue: a project-level view, a board, a directory that is not an id,
  /// and a v2 status bucket, whose paths are legacy's to read.
  ///
  /// `path` is relative to the project root, or absolute. **The id is read by
  /// [`crate::id::ThreadId::parse`] and [`crate::id::IssueId::parse`], and a
  /// record's path is confirmed against the layout methods above**, so no
  /// pattern of an id and no spelling of the layout is kept here a second time,
  /// and a configured `intent_dir` is honoured as they honour it. It reads no
  /// canon: whether the record exists is the caller's question.
  pub fn owner_of(&self, path: &Path) -> Option<crate::id::Named> {
    use crate::id::{IssueId, Named, ThreadId};
    let path = self.root.join(path);
    // The first name under `dir`, and whether anything lies below it.
    let head = |dir: PathBuf| -> Option<(String, bool)> {
      let mut under = path.strip_prefix(dir).ok()?.components();
      let first = under.next()?.as_os_str().to_str()?.to_string();
      Some((first, under.next().is_some()))
    };
    let stem = |name: &str| Path::new(name).file_stem()?.to_str().map(str::to_string);
    if let Some((id, _)) = head(self.st_dir()) {
      return ThreadId::parse(&id).ok().map(Named::Thread);
    }
    if let Some((name, beside)) = head(self.canon_st_dir()) {
      if beside {
        return ThreadId::parse(&name).ok().map(Named::Thread);
      }
      let id = ThreadId::parse(&stem(&name)?).ok()?;
      return (self.thread_json(&id) == path).then_some(Named::Thread(id));
    }
    // A record file of a thread's in the release's format (ST0096 WP-05):
    // `.canon/records/<thread>/<kind>/...` is the thread's.
    if let Some((name, true)) = head(self.intent_dir().join(CANON_RECORDS_DIR)) {
      return ThreadId::parse(&name).ok().map(Named::Thread);
    }
    if let Some((name, false)) = head(self.issues_dir()) {
      let number = IssueId::parse(&stem(&name)?).ok()?;
      return (self.issue_json(number) == path).then_some(Named::Issue(number));
    }
    if let Some((name, false)) = head(self.issues_view_dir()) {
      let number = IssueId::parse(&stem(&name)?).ok()?;
      return (self.issue_view(number) == path).then_some(Named::Issue(number));
    }
    None
  }

  /// The generated views as formatter-ignore GLOBS, relative to the project
  /// root -- what `crate::facade::converge_formatter_exclusion` writes into a
  /// consumer's `.prettierignore` (AC-07.6).
  ///
  /// **THIS SITS HERE, TOUCHING THE VIEW-PATH METHODS, ON PURPOSE.** A roster
  /// kept anywhere else is one somebody must remember to extend on the day they
  /// add a view, which is the day they are thinking about something else --
  /// `openness.rs` makes the argument for enumerating tables from the DDL and
  /// `generated_views_are_not_formatted.rs` makes it for this exact set.
  ///
  /// **PATTERNS, NOT PATHS, AND THE DIFFERENCE IS LOAD-BEARING.**
  /// [`crate::views::render_all`] yields the views that exist NOW. A converger
  /// fed from it would write an empty file on a fresh project and then have to
  /// grow it per thread -- which is a second writer of generated content,
  /// arriving to fix a second-writer bug. The file therefore holds a small
  /// fixed pattern set, the tests check concrete paths, and **prettier's own
  /// resolution is the oracle that joins the two**, so neither side has to
  /// model the other.
  ///
  /// **EACH IS DERIVED FROM THE METHOD THAT PRODUCES THE REAL PATH WHERE A
  /// WILDCARD CAN PASS THROUGH IT**, with `*` standing in for the id, so a view whose path moves
  /// takes its pattern with it and cannot drift silently. The work-package one
  /// is assembled by hand because [`Self::wp_info_view`] takes a typed
  /// [`crate::id::WpId`] and no wildcard can pass through that -- stated rather than
  /// hidden, because it is the one line here that a path change could leave
  /// behind.
  pub fn generated_view_patterns(&self) -> Vec<String> {
    let rel = |p: PathBuf| self.relative(&p);
    // Every aggregate view is a generated view, so the aggregate set is read
    // here rather than spelt a second time. The cover goes in after the index
    // views and before the acceptance view, where the formatter's ignore file
    // has always listed it, so a new project's file reads as it always has.
    let mut patterns = self.aggregate_view_patterns();
    patterns.insert(INDEX_VIEWS.len(), rel(self.info_view("*")));
    patterns.extend([
      rel(self.thread_dir("*").join(WP_DIR).join("*").join(INFO_VIEW)),
      // **Assembled by hand for the reason the work-package one is**:
      // `issue_view` takes a typed `IssueId` and no wildcard survives that.
      // Same caveat, same place, so the two sit together.
      rel(self.issues_view_dir().join("*.md")),
      rel(self.wb_board_view("*")),
      rel(self.wb_inbox_view("*", "*")),
    ]);
    patterns
  }

  /// Whether a project-relative path is one of the generated views
  /// [`Self::generated_view_patterns`] names, for a caller that asks about many
  /// paths: the patterns are built once and matched with the search surfaces'
  /// glob, `*` within one segment.
  ///
  /// **ONE PREDICATE.** `plan::classify` sorts a merge's views with it, and the
  /// file index asks it so that a generated view's row is only ever the bytes
  /// its renderer wrote (issue 0630).
  pub fn generated_view_matcher(&self) -> impl Fn(&str) -> bool + use<> {
    glob_matcher(self.generated_view_patterns())
  }

  /// Every file under a thread's directory, sorted, each as a path RELATIVE to
  /// that directory -- the address [`Attachment::path`] carries.
  ///
  /// Recursive, so `parity/cmd-st.md` and `WP/01/notes.md` are reached without
  /// a per-level collection. **Gitignored paths are excluded (D29): a path git
  /// does not carry cannot be canon**, which is also what keeps a stray
  /// `.DS_Store` out of the report by rule rather than by a special case.
  /// Every carried attachment under a thread's directory, plus the files that
  /// could not be carried and why.
  ///
  /// **THE ONE COLLECTOR, and "one" is the requirement rather than a tidiness
  /// preference** (vc, condition 2). The migrator grew this walk first and
  /// `sync` needed the same one; two walks over the same directory answering
  /// the same question drift, and the drift would land exactly where it just
  /// cost us -- content carried when it arrives by migration and dropped when
  /// a person types it, with nothing comparing the two paths.
  ///
  /// **Refusals are RETURNED, not recorded**, because the two callers classify
  /// them differently: the migrator files them against a thread's open/closed
  /// disposition, and `sync` has no such axis. Returning `(path, reason)`
  /// pairs lets each build its own finding without this function knowing about
  /// either. A file that cannot be read is NEVER silently skipped -- it comes
  /// back named, which is the posture `sync` already takes on undecodable
  /// content.
  pub fn collect_attachments(&self, id: &str) -> (Vec<Attachment>, Vec<(String, String)>) {
    let dir = self.thread_dir(id);
    let mut carried = Vec::new();
    let mut refused = Vec::new();
    for rel in Project::files_in(&dir) {
      let path = dir.join(&rel);
      let name = self.relative(&path);
      match Project::classify(&rel) {
        ThreadFile::Attachment => {}
        // Consumed by the parsers -- carrying them here as well would give one
        // file two homes in the model.
        ThreadFile::GeneratedView | ThreadFile::Canon => continue,
        // **REFUSED BY NAME, NEVER SKIPPED** (issue 0719): carried, it is the
        // author's file a disk folding case stores as the thread's own, and the
        // next render writes over it.
        ThreadFile::CaseVariant { of } => {
          refused.push((name, Project::case_variant_refusal(&of)));
          continue;
        }
      }
      // **THE SIZE IS ASKED OF THE METADATA, BEFORE THE READ.** The read below
      // is unbounded, so checking afterwards means loading the file in order to
      // be told it was too big -- harmless for a 2 MiB screenshot and not
      // harmless the first time somebody commits something large. A cap that
      // only applies after the cost has been paid is not a cap.
      match std::fs::metadata(&path) {
        Ok(meta) if !within_attachment_cap(meta.len()) => {
          refused.push((
            name,
            format!(
              "is {} bytes, over the {ATTACHMENT_CAP_BYTES}-byte cap, so it stays on disk and is not carried into canon",
              meta.len()
            ),
          ));
          continue;
        }
        // Unreadable metadata is not a size verdict. Fall through to the read,
        // which produces the honest "could not be read" refusal below rather
        // than a guess about how big it is.
        Ok(_) | Err(_) => {}
      }
      // **THE NAMING GATE, AT INGEST** (ST0057 AC-03.3). A path that cannot be
      // given both a canon path and a URL is refused HERE, which is the only
      // door into the model -- so a name nothing could store or address never
      // becomes canon in the first place.
      //
      // **REJECTION IS NOT RETROACTIVE, and that falls out rather than being
      // arranged.** Refusing here means the file is not carried; `organize`
      // then meets a `ThreadFile::Attachment` the store does not hold and
      // reports it UNCLAIMED at row five, which it never removes because it is
      // the only copy. So an existing violator is named on every run and left
      // exactly where its author put it.
      let attachment_rel = rel.to_string_lossy().replace('\\', "/");
      if let Err(bad) = crate::project::attachment_name(id, &attachment_rel) {
        refused.push((name, bad.to_string()));
        continue;
      }
      match std::fs::read(&path) {
        // **FORM FOLLOWS CONTENT, and this is the only place that decides it**
        // (ST0057 AC-03.2). Valid UTF-8 is carried inline as `text`; anything
        // else is carried as bytes, in a sidecar file under canon.
        //
        // **It is decided by DECODING, never by the extension.** The extension
        // already answered a different question one step above -- whether the
        // file is carried at all -- and reusing it here would make `.sh` mean
        // "text" for a shell script with one Latin-1 byte in a comment, which
        // is precisely the file that would be silently mangled.
        //
        // **This arm used to REFUSE**, with "not valid UTF-8, so it cannot be
        // carried as text", and the refusal was right for as long as canon had
        // nowhere to put the bytes. It now has one, and a refusal here would
        // leave the file uncarried, therefore unrecoverable, therefore pinned
        // to disk forever -- the condition `.intentfiles` exists to end.
        Ok(raw) => match String::from_utf8(raw) {
          Ok(text) => carried.push(Attachment::new(rel.to_string_lossy(), text)),
          Err(not_text) => carried.push(Attachment::opaque(
            rel.to_string_lossy(),
            not_text.into_bytes(),
          )),
        },
        // Still a refusal, and a different kind: unreadable is not the same as
        // unrepresentable. Nothing here can carry bytes it could not obtain,
        // and carrying an empty attachment in their place would record a file
        // that never existed.
        Err(e) => refused.push((name, format!("could not be read: {e}"))),
      }
    }
    carried.sort_by(|a, b| a.path.cmp(&b.path));
    (carried, refused)
  }

  pub fn thread_files(&self, id: &str) -> Vec<PathBuf> {
    Project::files_in(&self.thread_dir(id))
  }

  /// Every file under a directory the CALLER names, relative to it, honouring
  /// the project's ignore rules (D29).
  ///
  /// An associated function because it consults nothing on `Project` -- the
  /// same reason [`Project::classify`] is one.
  ///
  /// **IT WAS `thread_files_in` UNTIL 2026-09-05 AND THE BODY NEVER CHANGED.**
  /// Nothing in it is thread-specific -- its own doc said so, in the sentence
  /// above about consulting nothing -- and the whiteboard enumeration in
  /// `sync` is the second population to need exactly this walk. The choice was
  /// a second gitignore-aware walker or a name that stops narrowing a general
  /// function to its first caller, and Highlander settles that: a name is
  /// cheaper to fix than a duplicate is to keep in step. Renamed rather than
  /// aliased, because a deprecation stub is the two-homes shape with a label
  /// on it.
  pub fn files_in(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut walk = ignore::WalkBuilder::new(dir);
    walk
      .hidden(false)
      .git_ignore(true)
      .parents(true)
      .git_global(false)
      .git_exclude(false);
    for entry in walk.build().filter_map(Result::ok) {
      let path = entry.into_path();
      if path.is_file()
        && let Ok(rel) = path.strip_prefix(dir)
      {
        out.push(rel.to_path_buf());
      }
    }
    out.sort();
    out
  }

  /// Every thread id with committed canon, sorted. Absent `st/` is an empty
  /// project, not an error -- `intent init` creates the directory lazily.
  pub fn thread_ids(&self) -> Result<Vec<crate::id::ThreadId>, ProjectError> {
    let Ok(entries) = std::fs::read_dir(self.canon_st_dir()) else {
      return Ok(Vec::new());
    };
    let mut ids = Vec::new();
    for path in entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.extension().is_some_and(|x| x == "json"))
    {
      ids.push(self.canon_name(&path, "steel thread", crate::id::ThreadId::parse)?);
    }
    // Typed, as [`Self::issue_numbers`] is (ST1c2db93 P8), and ordered by the
    // stored text it always was: a thread's typed order puts every ordinal
    // before every hash, which would reorder what a reader of this list
    // reports on an estate holding both.
    ids.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
    Ok(ids)
  }

  /// The stored id a canon file's name gives, read by `parse`, or the refusal
  /// of a name that is no `kind` id (ST0095 WP-06, AC-06.5): never skipped.
  fn canon_name<T>(
    &self,
    path: &Path,
    kind: &'static str,
    parse: impl Fn(&str) -> Result<T, crate::id::IdError>,
  ) -> Result<T, ProjectError> {
    path
      .file_stem()
      .and_then(|stem| stem.to_str())
      .and_then(|stem| parse(stem).ok())
      .ok_or_else(|| ProjectError::CanonNameNotAnId {
        path: self.relative(path),
        kind,
      })
  }

  /// Every issue with committed canon, by id, sorted.
  ///
  /// A file whose name is not an issue id is passed over here, as it always
  /// was; ST0095 WP-06 turns that into a refusal by name.
  pub fn issue_numbers(&self) -> Result<Vec<crate::id::IssueId>, ProjectError> {
    let Ok(entries) = std::fs::read_dir(self.issues_dir()) else {
      return Ok(Vec::new());
    };
    let mut ids = Vec::new();
    for path in entries
      .filter_map(Result::ok)
      .map(|e| e.path())
      .filter(|p| p.extension().is_some_and(|x| x == "json"))
    {
      ids.push(self.canon_name(&path, "issue", crate::id::IssueId::parse)?);
    }
    ids.sort_unstable();
    Ok(ids)
  }
}

/// A path as Intent names it: relative to the project root, forward-slashed on
/// every platform.
///
/// The one home for this. Findings, the file index and the skew check all
/// report paths, and three private copies of "make it relative" is precisely
/// the shape of drift the Highlander rule exists to prevent -- the copies
/// agree until one of them handles a prefix mismatch differently, and then two
/// subsystems disagree about what to call the same file.
pub fn relative(root: &Path, path: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .components()
    .map(|c| c.as_os_str().to_string_lossy())
    .collect::<Vec<_>>()
    .join("/")
}

/// What `intent edit <path>` should do with a file (AC-05.1, hv 2026-08-19).
///
/// **A generated view is REFUSED and the refusal names where to author
/// instead.** Canon had this deferred at `surface/dispatch-table.json:834` on
/// the argument that emitting the path unchanged is defensible if the skew
/// check catches the edit. AC-03.4 is green and does catch it -- **but it
/// catches it AFTER it is written, and the user's work is already gone.
/// Detection is not prevention**, and this estate has lost work to exactly
/// this twice: a row authored into `acceptance.md` dies at the next
/// `--to-disk`, and AC-03.16 exists because a generated view was inviting
/// authoring in itself.
///
/// **And a refusal that only refuses is worse than useless here**, because the
/// user has a real edit to make and no idea where it goes. So the disposition
/// carries the destination.
///
/// Measured, so the refusal is not read as broader than it is: ST0056's thread
/// directory holds 13 files at depth 1 and **only two are generated views**.
/// The other eleven are attachments, authored on disk, and `edit` hands those
/// over without hesitation.
/// Is this a THREAD's `info.md` -- the depth-1 cover, not a work package's?
///
/// **ONE HOME FOR THE DEPTH-1 RULE.** [`Project::classify`] and
/// [`Project::edit_disposition`] both need it and they need it to AGREE: the
/// first says this file is a generated view, the second says this particular
/// generated view round-trips. Two copies of `depth == 1 && name == "info.md"`
/// would be two answers to one question -- the defect AC-02.5 names -- and
/// they would diverge on the day a view MOVES, not the day one is added.
///
/// A work package's `WP/<NN>/info.md` is deliberately NOT this. It renders
/// through `wp_info` with its own field set, and hv's ruling of 2026-08-29 was
/// about the thread cover; extending it downward is a separate decision
/// carrying its own allow-list, not a free consequence of this one.
fn is_thread_info(rel: &Path) -> bool {
  rel.components().count() == 1 && rel.file_name().and_then(|n| n.to_str()) == Some(INFO_VIEW)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditDisposition {
  /// The file on disk IS the authoring surface. Hand over the path.
  Open,
  /// Generated, but some sections ROUND-TRIP: open it, and `sync --to-store`
  /// reads those sections back into the model.
  ///
  /// **A THIRD STATE, BECAUSE TWO COULD NOT SAY THIS.** `Open` promises the
  /// file on disk IS the authoring surface, which is false here -- everything
  /// outside `round_trips` is regenerated. `Refuse` was what `info.md` got
  /// until hv ruled on 2026-08-29, and it sent the operator to verbs that
  /// cannot write `objective` or `context` at all. **Both were wrong in
  /// different directions, and a boolean cannot hold a partial authority.**
  OpenRoundTrip {
    round_trips: &'static [&'static str],
  },
  /// Generated from the model. Refuse, and say what authors it.
  Refuse { author_with: &'static str },
  /// One file with the thread's own `of` on a disk that folds case (issue
  /// 0719). Refuse, and name both: no door opens it.
  CaseVariant { of: PathBuf },
}

impl Project {
  /// Whether `intent edit` may hand this path to an editor.
  ///
  /// **Derived from [`Project::classify`] rather than from a list of
  /// filenames**, because a second answer to "what is this file" is the
  /// two-declarations defect AC-02.5 names -- and it would drift the day
  /// somebody adds a view.
  pub fn edit_disposition(rel: &Path) -> EditDisposition {
    match Project::classify(rel) {
      // **hv RULED THE THREAD COVER OPEN ON 2026-08-29**, first-hand, after
      // driving `intent st edit 68` and meeting the refusal: _I want to edit
      // the ST and then I want a sync to know that's been edited and update
      // the db._ The refusal was correct about the FILE and wrong about the
      // operator's need -- its remedy said author it with `intent st`, and not
      // one of the seventeen `intent st` verbs writes `objective` or `context`.
      ThreadFile::GeneratedView if is_thread_info(rel) => EditDisposition::OpenRoundTrip {
        round_trips: crate::views::INFO_ROUND_TRIP_SECTIONS,
      },
      ThreadFile::GeneratedView => {
        let name = rel.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        EditDisposition::Refuse {
          author_with: match name {
            // Named per view rather than one generic message: an operator who
            // wanted to add a criterion and one who wanted to retitle a thread
            // need different verbs, and "this is generated" sends both to the
            // same dead end.
            ACCEPTANCE_VIEW => "`intent ac` for criteria and `intent at` for test rows",
            // **THE ONE OTHER VIEW THIS ARM REACHES IS A WORK PACKAGE'S COVER**
            // (`classify` names three, and the thread cover opens above). Its
            // remedy named `intent wp`, which has no writer for a package's prose
            // (issue 0334); `set` is the verb that writes it.
            _ => "`intent set intent:///threads/<ST>/wp/<NN> objective|body --from <file>`",
          },
        }
      }
      // **EVERY VALUE OF THIS FIELD MUST COMPLETE THE SENTENCE `author it with
      // ...`, AND THIS ONE DID NOT.** Both consumers interpolate it that way --
      // `FacadeError::NotEditable`'s remedy (`facade.rs:686`) and the
      // attachment `put` arm -- so a value that is a CLAUSE rather than a
      // phrase prints "author it with canon is written by the verbs".
      //
      // **NOTHING CAUGHT IT BECAUSE EACH DOOR REACHES ONLY ONE VALUE.** `st
      // edit` appends `.md` to its argument, so it can never classify a file as
      // `Canon` and only ever prints the view arm, which composes; the
      // attachment address only ever reaches this arm, because `address::parse`
      // refuses a view name one layer lower. Two values, two consumers, and
      // each pairing exercised the half that happened to read correctly.
      ThreadFile::Canon => EditDisposition::Refuse {
        author_with: "the verbs that write canon; `intent st`, `intent wp`, `intent ac`, `intent at`",
      },
      // An attachment is AUTHORED on disk -- authority runs the other way, and
      // `--to-store` ingests what you wrote. Files over the cap are not ours to
      // refuse: the estate holds files Intent does not model and never claimed
      // to, and refusing them would make `edit` narrower than the directory.
      ThreadFile::Attachment => EditDisposition::Open,
      ThreadFile::CaseVariant { of } => EditDisposition::CaseVariant { of },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// **A CONFIG DECLARING A VERSION BEFORE 3 IS AN UNREADABLE CONFIG.** It is
  /// refused at open, as a config this binary cannot read, so the commit gate's
  /// readers answer could-not-judge for it and every other door refuses it,
  /// and the refusal names the version it declares.
  #[test]
  fn a_config_declaring_a_version_before_3_is_refused_as_unreadable() {
    let dir = tempfile::tempdir().expect("tempdir");
    let declared = format!("{}.0.0", OLDEST_READ_MAJOR - 1);
    let path = Project::config_path(dir.path());
    std::fs::create_dir_all(path.parent().expect("a config sits in a directory"))
      .expect("config dir");
    std::fs::write(&path, format!(r#"{{"intent_version":"{declared}"}}"#)).expect("seed");

    let refused = Project::open(dir.path()).expect_err("a config before 3 is refused");
    assert!(
      matches!(&refused, ProjectError::PreV3 { declared: d, .. } if *d == declared),
      "expected the pre-3 refusal naming {declared}, got {refused:?}"
    );
    assert!(
      refused.config_unread(),
      "a config before 3 must answer as unread"
    );
    assert!(
      crate::remedy::Remedy::remedy(&refused).contains(&declared),
      "the remedy names the version the config declares"
    );

    let current = format!("{OLDEST_READ_MAJOR}.0.0");
    std::fs::write(&path, format!(r#"{{"intent_version":"{current}"}}"#)).expect("reseed");
    Project::open(dir.path()).expect("a config declaring the oldest read version opens");
  }
  fn git(args: &[&std::ffi::OsStr]) {
    let status = std::process::Command::new("git")
      .args(args)
      .status()
      .expect("run git");
    assert!(status.success(), "git {args:?} failed");
  }

  /// Issue 0609's controls, on what git itself writes. `git init --bare` reads
  /// as a bare repository from its own root and from a directory inside it
  /// with no `.git` between them; a working copy and its own `.git` directory,
  /// whose config says `bare = false`, do not.
  #[test]
  fn a_bare_repository_is_found_from_its_root_and_from_inside_it() {
    let dir = tempfile::tempdir().expect("tempdir");
    let bare = dir.path().join("x.git");
    let copy = dir.path().join("copy");
    git(&[
      "init".as_ref(),
      "-q".as_ref(),
      "--bare".as_ref(),
      bare.as_os_str(),
    ]);
    git(&["init".as_ref(), "-q".as_ref(), copy.as_os_str()]);
    let inside = bare.join("inside");
    std::fs::create_dir(&inside).expect("a directory inside the bare repository");

    assert_eq!(bare_repository(&bare), Some(bare.clone()));
    assert_eq!(bare_repository(&inside), Some(bare.clone()));
    assert_eq!(bare_repository(&copy), None);
    assert_eq!(bare_repository(&copy.join(".git")), None);
  }

  /// The bare-plus-worktrees layout: a worktree kept inside a bare repository's
  /// directory carries its own `.git` file, and the walk stops there rather
  /// than refusing the worktree for the repository above it.
  #[test]
  fn a_worktree_inside_a_bare_repository_is_where_a_project_lives() {
    let dir = tempfile::tempdir().expect("tempdir");
    let bare = dir.path().join("x.git");
    git(&[
      "init".as_ref(),
      "-q".as_ref(),
      "--bare".as_ref(),
      bare.as_os_str(),
    ]);
    let worktree = bare.join("main");
    std::fs::create_dir(&worktree).expect("the worktree's directory");
    std::fs::write(
      worktree.join(".git"),
      format!("gitdir: {}\n", bare.join("worktrees/main").display()),
    )
    .expect("the worktree's .git file");

    assert_eq!(bare_repository(&worktree), None);
  }

  /// hv's ruling (2026-08-19): `intent edit` refuses a generated view and
  /// names the authoring surface instead. Detection is not prevention -- the
  /// skew check catches a hand-edited view AFTER the work is gone.
  ///
  /// **NARROWED BY hv ON 2026-08-29, AND THE THREAD COVER IS NOW THE
  /// EXCEPTION.** hv drove `intent st edit 68`, met this refusal, and ruled
  /// that `info.md` round-trips its authored sections. **The 2026-08-19 ruling
  /// is not reversed** -- `acceptance.md`, a WP's cover and canon still refuse,
  /// and the cover is still generated for every byte outside its allow-list.
  /// What changed is that a refusal whose remedy named `intent st` was sending
  /// the operator to seventeen verbs none of which writes `objective` or
  /// `context`, so the remedy was a dead end rather than a redirection.
  #[test]
  fn edit_refuses_generated_views_and_opens_authored_files() {
    match Project::edit_disposition(Path::new("acceptance.md")) {
      EditDisposition::Refuse { author_with } => assert!(
        author_with.contains("intent ac"),
        "acceptance.md must name the verb that authors it, said `{author_with}`"
      ),
      other => panic!("acceptance.md is GENERATED and must be refused, got {other:?}"),
    }

    // **The thread cover opens, and it opens into a PARTIAL authority.** The
    // disposition carries the allow-list rather than a bare yes, because the
    // caller has to be able to say which sections survive -- `Open` would
    // promise the whole file is authored, which is exactly the false promise
    // that loses an edit to `## Scope`.
    match Project::edit_disposition(Path::new("info.md")) {
      EditDisposition::OpenRoundTrip { round_trips } => {
        assert_eq!(round_trips, crate::views::INFO_ROUND_TRIP_SECTIONS);
        assert!(
          !round_trips.is_empty(),
          "an empty allow-list would open the file and carry nothing"
        );
      }
      other => panic!("hv ruled the thread cover open on 2026-08-29, got {other:?}"),
    }

    // **A WORK PACKAGE'S COVER IS NOT THE THREAD'S AND STILL REFUSES.** This
    // is the arm that would go quiet if `is_thread_info` ever stopped checking
    // depth -- both files are called `info.md`, and only the depth tells them
    // apart.
    // **AND ITS REMEDY NAMES THE VERB THAT WRITES A PACKAGE'S PROSE** (issue
    // 0334): it named `intent wp`, and no `intent wp` verb writes a body.
    match Project::edit_disposition(&PathBuf::from("WP").join("01").join("info.md")) {
      EditDisposition::Refuse { author_with } => assert!(
        author_with.contains("intent set") && !author_with.contains("intent wp"),
        "a work package's cover must name `intent set`, said `{author_with}`"
      ),
      other => panic!("a work package's cover is GENERATED and must be refused, got {other:?}"),
    }
    assert!(matches!(
      Project::edit_disposition(Path::new("thread.json")),
      EditDisposition::Refuse { .. }
    ));

    // **The discriminating half, and it is the larger population.** ST0056
    // holds 13 files at depth 1 and only two are views; refusing the rest
    // would make `edit` narrower than the directory it serves.
    for authored in [
      "design.md",
      "data-model.md",
      "parity.md",
      "impl.md",
      "tasks.md",
      "migration.md",
      "install.md",
      "deferred.md",
      "critic-gate.md",
      "output-contracts.md",
      "realisation.md",
    ] {
      assert_eq!(
        Project::edit_disposition(Path::new(authored)),
        EditDisposition::Open,
        "{authored} is AUTHORED on disk -- the file is the authoring surface"
      );
    }

    // `info.md` deep under a parity directory is an author's file, not our
    // cover -- classify decides by shape, and edit inherits that.
    assert_eq!(
      Project::edit_disposition(&PathBuf::from("parity").join("fixtures").join("info.md")),
      EditDisposition::Open
    );
  }

  fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = dir.path().join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir");
    std::fs::write(
      config.join("config.json"),
      r#"{"intent_version":"3.0.0","project_name":"Fixture","author":"cc","intent_dir":"intent","languages":["rust"],"plugins":{"claude":{}}}"#,
    )
    .expect("write config");
    dir
  }

  #[test]
  fn thread_ids_keep_the_stored_texts_order() {
    // ST1c2db93 P8: an ordinal beside a hashed thread whose text sorts first.
    // The typed order puts every ordinal first; this list keeps the text's.
    use crate::id::{HASH_HEX, THREAD_PREFIX, ThreadId};
    let dir = fixture();
    let project = Project::open(dir.path()).expect("open");
    let ordinal = ThreadId::from_ordinal(59).expect("an ordinal that fits");
    let hashed = ThreadId::parse(&format!(
      "{THREAD_PREFIX}{:a<width$}",
      "0012",
      width = HASH_HEX
    ))
    .expect("a whole hashed thread id");
    assert!(
      ordinal < hashed && hashed.as_str() < ordinal.as_str(),
      "precondition: the typed order and the text order disagree on this pair"
    );
    std::fs::create_dir_all(project.canon_st_dir()).expect("mkdir");
    for id in [&ordinal, &hashed] {
      std::fs::write(project.thread_json(id), "{}").expect("write a canon file");
    }
    assert_eq!(
      project.thread_ids().expect("the canon names its threads"),
      [hashed, ordinal],
      "stored-text order, as the list always gave"
    );
  }

  #[test]
  fn discover_walks_up_to_the_root() {
    let dir = fixture();
    let deep = dir.path().join("intent").join("st").join("ST0001");
    std::fs::create_dir_all(&deep).expect("mkdir");
    let project = Project::discover(&deep).expect("discover");
    assert_eq!(project.root(), dir.path());
    assert_eq!(project.config().project_name, "Fixture");
  }

  #[test]
  fn an_unknown_config_block_is_carried_not_dropped() {
    let dir = fixture();
    let project = Project::open(dir.path()).expect("open");
    assert!(
      project.config().extra.contains_key("plugins"),
      "a block this version does not model must survive a read"
    );
  }

  #[test]
  fn paths_are_derived_from_the_configured_intent_dir() {
    let dir = fixture();
    let project = Project::open(dir.path()).expect("open");
    assert_eq!(
      project.thread_json("ST0056"),
      dir.path().join("intent/.canon/st/ST0056.json")
    );
    assert_eq!(
      project.issue_json(crate::id::IssueId::from_ordinal(21).expect("an issue id")),
      dir.path().join("intent/.canon/issues/0021.json")
    );
    // **The negative arm, beside the positive one (AC-01.6).** Canon moved and
    // the VIEWS did not: `thread_dir` still answers the directory a reader
    // browses, because `info.md` and `acceptance.md` hang off it. Asserting
    // only the first two would pass for a wholesale relocation that emptied
    // `intent/st/` entirely.
    assert_eq!(
      project.thread_dir("ST0056"),
      dir.path().join("intent/st/ST0056")
    );
    assert_eq!(
      project.db_path(),
      dir.path().join("intent/.cache/intent.db")
    );
  }

  /// Declaring APPENDS and is idempotent -- v2's `add_project_language`.
  ///
  /// Order is the author's. A sort here would rewrite a line nobody asked to
  /// change on the first declaration after an upgrade, which is a diff an
  /// operator has to read and then decide was harmless.
  #[test]
  fn declaring_a_language_appends_once_and_keeps_the_order_it_was_given() {
    let mut config = Config {
      intent_version: "3.0.0".to_string(),
      project_name: "Fixture".to_string(),
      author: "cc".to_string(),
      project_id: None,
      intent_dir: "intent".to_string(),
      languages: vec!["rust".to_string(), "elixir".to_string()],
      contracts: Contracts::default(),
      todo: TodoConfig::default(),
      backup: BackupConfig::default(),
      embed: EmbedConfig::default(),
      index: IndexConfig::default(),
      whiteboard: WhiteboardConfig::default(),
      doctor: DoctorConfig::default(),
      guards: Vec::new(),
      commands: CommandsConfig::default(),
      extra: serde_json::Map::new(),
    };

    assert!(
      config.declare_language("shell"),
      "a new language changes the array"
    );
    assert_eq!(config.languages, vec!["rust", "elixir", "shell"]);

    assert!(
      !config.declare_language("shell"),
      "declaring twice must report no change rather than appending a duplicate"
    );
    assert_eq!(
      config.languages,
      vec!["rust", "elixir", "shell"],
      "and the array is untouched, in the order it was authored"
    );
  }

  /// Undeclaring removes every occurrence, and an absent language is a no-op.
  #[test]
  fn undeclaring_removes_every_copy_and_reports_when_nothing_changed() {
    let mut config = Config {
      intent_version: "3.0.0".to_string(),
      project_name: "Fixture".to_string(),
      author: "cc".to_string(),
      project_id: None,
      intent_dir: "intent".to_string(),
      languages: vec!["rust".to_string(), "shell".to_string(), "rust".to_string()],
      contracts: Contracts::default(),
      todo: TodoConfig::default(),
      backup: BackupConfig::default(),
      embed: EmbedConfig::default(),
      index: IndexConfig::default(),
      whiteboard: WhiteboardConfig::default(),
      doctor: DoctorConfig::default(),
      guards: Vec::new(),
      commands: CommandsConfig::default(),
      extra: serde_json::Map::new(),
    };

    assert!(config.undeclare_language("rust"));
    assert_eq!(
      config.languages,
      vec!["shell"],
      "a remove that left a second copy would report success and change nothing visible"
    );
    assert!(
      !config.undeclare_language("rust"),
      "removing what is not there is a no-op that says so"
    );
  }

  /// **THE ONE THIS WHOLE WRITE PATH RESTS ON: A REWRITE DROPS NOTHING.**
  ///
  /// `config.json` is the only file that makes a directory an Intent project,
  /// v2 also writes it, and plugins extend it with blocks this type has never
  /// heard of. A typed round trip through a struct that modelled only what it
  /// knew would delete the rest SILENTLY -- the operator's next clue would be a
  /// tool that stopped working. `st_prefix` is here on purpose: it is retired
  /// from the type and must still survive the file.
  #[test]
  fn a_rewrite_carries_every_block_this_version_does_not_model() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let path = Project::config_path(root);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(
      &path,
      r#"{
  "intent_version": "3.0.0",
  "project_name": "Fixture",
  "author": "cc",
  "created": "2026-08-25",
  "intent_dir": "intent",
  "st_prefix": "ST",
  "languages": ["rust"],
  "plugins": { "claude": { "enabled": true } }
}
"#,
    )
    .expect("seed config");

    let before: Config =
      serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("parse");
    let mut after = before.clone();
    assert!(after.declare_language("shell"));
    write_config(root, &after).expect("write the config back");

    let reread: serde_json::Value =
      serde_json::from_str(&std::fs::read_to_string(&path).expect("reread")).expect("parse json");

    assert_eq!(
      reread["languages"],
      serde_json::json!(["rust", "shell"]),
      "the change we asked for must be there"
    );
    for key in ["created", "st_prefix", "plugins"] {
      assert!(
        !reread[key].is_null(),
        "the rewrite dropped {key:?}, which this type does not model -- \
         that is the silent data loss `Config::extra` exists to prevent:\n{reread:#}"
      );
    }
    assert_eq!(
      reread["plugins"]["claude"]["enabled"],
      serde_json::json!(true),
      "and an unmodelled block survives with its contents, not merely its key"
    );
  }

  /// **A LANGUAGE WRITE KEEPS THE PROJECT'S GUARDS** (issue 0426).
  ///
  /// `lang init` and `lang remove` are the verbs that rewrite `config.json`, and
  /// a guard they dropped would stop running at the next commit with nothing
  /// said. Both shapes are driven -- a guard with a `when` and one without --
  /// and the argv is compared whole, so a reordered or stringified `run` fails.
  #[test]
  fn a_language_write_keeps_the_projects_guards_argv_and_when_intact() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let path = Project::config_path(root);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    let guards = serde_json::json!([
      { "run": ["bin/hooks/check-docs", "--strict", "a b"], "when": "docs" },
      { "run": ["bin/hooks/always"] }
    ]);
    std::fs::write(
      &path,
      format!(r#"{{"intent_version":"3.0.0","languages":["rust"],"guards":{guards}}}"#),
    )
    .expect("seed");

    let mut config: Config =
      serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("parse");
    assert_eq!(
      config.guards,
      vec![
        GuardDecl {
          run: vec![
            "bin/hooks/check-docs".into(),
            "--strict".into(),
            "a b".into()
          ],
          when: Some("docs".into()),
        },
        GuardDecl {
          run: vec!["bin/hooks/always".into()],
          when: None
        },
      ],
      "the declaration reads as typed guards"
    );

    assert!(config.declare_language("shell"));
    write_config(root, &config).expect("lang init's write");
    assert!(config.undeclare_language("rust"));
    write_config(root, &config).expect("lang remove's write");

    let reread: serde_json::Value =
      serde_json::from_str(&std::fs::read_to_string(&path).expect("reread")).expect("parse json");
    assert_eq!(
      reread["languages"],
      serde_json::json!(["shell"]),
      "{reread:#}"
    );
    assert_eq!(
      reread["guards"], guards,
      "a language write changed the guards:\n{reread:#}"
    );
  }

  /// The rewritten file keeps the shape `init.rs` lays down.
  ///
  /// Two-space indent and a trailing newline. Not cosmetic: this file is read
  /// by v2's `jq`-based helpers and by humans in a diff, and a rewrite that
  /// reflowed it would make every subsequent `lang init` look like a
  /// whole-file change.
  #[test]
  fn a_rewritten_config_keeps_the_shape_init_lays_down() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let path = Project::config_path(root);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, r#"{"intent_version":"3.0.0","languages":[]}"#).expect("seed");

    let mut config: Config =
      serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("parse");
    config.declare_language("rust");
    write_config(root, &config).expect("write");

    let body = std::fs::read_to_string(&path).expect("reread");
    assert!(
      body.ends_with("}\n"),
      "a trailing newline, as init writes: {body:?}"
    );
    assert!(
      body.contains("\n  \"intent_version\""),
      "two-space indent, as init writes: {body}"
    );
  }

  /// **WRITING ONE KEY DOES NOT FREEZE THE DEFAULTS** (issue `0091`).
  ///
  /// The file declares four keys, one of them a CHOSEN non-default. After a
  /// `lang init`-shaped write, the defaults it never declared are still absent,
  /// so the project goes on tracking the tool; the chosen value is still its
  /// own; and the file's keys keep their order.
  #[test]
  fn writing_one_key_leaves_undeclared_defaults_unwritten() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let path = Project::config_path(root);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(
      &path,
      r#"{"intent_version":"3.0.0","project_name":"Fixture","languages":["rust"],"todo":{"window_hours":48}}"#,
    )
    .expect("seed");

    let mut config: Config =
      serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("parse");
    assert!(config.declare_language("shell"));
    write_config(root, &config).expect("write");

    let body = std::fs::read_to_string(&path).expect("reread");
    let reread: serde_json::Value = serde_json::from_str(&body).expect("parse json");
    assert_eq!(
      reread["languages"],
      serde_json::json!(["rust", "shell"]),
      "{body}"
    );
    for key in ["author", "intent_dir", "backup"] {
      assert!(
        reread.get(key).is_none(),
        "{key:?} was never declared and is now frozen at today's default:\n{body}"
      );
    }
    assert_eq!(
      reread["todo"]["window_hours"],
      serde_json::json!(48),
      "a chosen value is the project's own and survives: {body}"
    );
    let at = |key: &str| body.find(&format!("\"{key}\"")).expect("key present");
    assert!(
      at("intent_version") < at("project_name")
        && at("project_name") < at("languages")
        && at("languages") < at("todo"),
      "the file's own key order is kept: {body}"
    );
  }

  /// Issue 0722: a file on disk answers against the rows by its exact row
  /// first, then by the one-file key, and the fold decides what a key-only
  /// match is. Pure, so both answers of the fold are driven on every host.
  #[test]
  fn a_disk_file_answers_by_its_exact_row_first_then_by_the_key_as_the_disk_folds() {
    let rows = ["Design.png", "notes.md", "design-old.png"];
    for folds in [true, false] {
      assert_eq!(
        disk_match(rows, "notes.md", folds),
        DiskMatch::Exact("notes.md"),
        "folds={folds}"
      );
      assert_eq!(
        disk_match(rows, "plan.md", folds),
        DiskMatch::New,
        "folds={folds}"
      );
    }
    assert_eq!(
      disk_match(rows, "design.png", true),
      DiskMatch::WorkingCopy("Design.png")
    );
    assert_eq!(
      disk_match(rows, "design.png", false),
      DiskMatch::CasePair("Design.png")
    );
    assert_eq!(
      disk_match(["design.png", "Design.png"], "Design.png", false),
      DiskMatch::Exact("Design.png"),
      "an exact row wins over a keyed one wherever it sits"
    );
  }
}
