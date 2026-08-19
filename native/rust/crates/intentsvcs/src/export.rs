//! `intent export --format <fmt>` -- the openness door (AC-06.6, D03, D34).
//!
//! **What this is for.** AC-02.6 requires the data to be usable without this
//! tool, and D34 makes that requirement load-bearing rather than courteous: the
//! extract is the interchange, so a lossy one does not inconvenience someone
//! exporting data, it destroys truth at the clone boundary where nobody typed
//! anything. This module is the one place a non-canonical projection is
//! produced, and it is built so that a lossy projection cannot be produced at
//! all.
//!
//! **How that is achieved, and it is the design rather than a test.** Every
//! format in [`FORMATS`] declares what it can do. A [`Projection::RoundTrips`]
//! format carries BOTH halves -- the writer and the reader -- and [`project`]
//! runs them against each other on the real bundle before returning a byte of
//! it. If what came back does not re-derive the canon byte for byte, the export
//! is REFUSED. A [`Projection::Lossy`] format never gets that far: it is refused
//! by name, with the reason and with what to use instead.
//!
//! So AC-06.6's disjunction -- "round-trips, or is refused by name rather than
//! emitted lossily" -- is not a property this code is tested for on a fixture.
//! It is the only thing this code can do, on every estate, including the ones
//! no fixture anticipated. **That matters here more than it usually would**,
//! because the failure being guarded against is silent: an export that drops a
//! field looks exactly like one that does not, and the loss is discovered by
//! whoever reads it back, somewhere else, later.
//!
//! **One roster, and no second place to add a format.** [`FORMATS`] is a slice
//! of [`Format`] values and there is deliberately no parallel enum: an enum
//! would mean a variant list and a spec list that a new format has to be added
//! to twice, and the guard walking one of them would report complete coverage
//! of the other's contents. The hand-kept roster inside the instrument is the
//! defect this project has already paid for once, so the instrument and the
//! subject are the same slice.

use serde::{Deserialize, Serialize};

use crate::event::{self, Envelope};
use crate::faces::INTENT_VER;
use crate::model::{Issue, Thread, to_canonical_json};

/// The `schema` field value for an export bundle.
pub const BUNDLE_SCHEMA: &str = "intent/export@3.0";

/// The whole estate as ONE document -- what a projection projects.
///
/// **This is a different shape from the canon, not a different encoding of
/// it.** The canon is a tree: one `thread.json` per thread, one `issue.json`
/// per issue, and `events.jsonl` beside them. The bundle is a single artefact
/// carrying all of it, which is what makes it the thing that travels (D34) and
/// the thing a stranger's script can open without knowing our directory
/// layout. `--format json` is therefore a real projection and not a copy: the
/// bytes it emits exist nowhere in the tree.
///
/// **The event log is IN, and it is the reason this is an interchange rather
/// than a dump.** D34's measurement is that the log is the one table that is
/// both durable truth and not reconstructible from the files; an extract
/// without it carries the present and loses how it was arrived at.
///
/// **The search index is OUT, and that exemption is quantitative rather than
/// tidy** -- 98.6% of a real store's bytes are `doc_sections_*`, all of it
/// derived from files already on disk and rebuilt locally. Truth travels at
/// roughly the size of the canon.
///
/// **Authored prose is OUT, and the reason is that it needs no door.**
/// `design.md`, `impl.md`, `tasks.md` and an issue's body are markdown sitting
/// in the repository, already usable without Intent by anyone with a text
/// editor. AC-02.6's problem is the STRUCTURED half -- the data that lives in a
/// schema and a database -- which is the half this carries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bundle {
  /// Always [`BUNDLE_SCHEMA`].
  pub schema: String,
  /// The build that wrote this artefact, so a consumer reading it in a year
  /// knows what wrote it. Not a time (D42): a version is a fact about a
  /// binary, and it is compiled in rather than asked for.
  pub intent_version: String,
  /// The project's UUID (D15). Empty on a pre-migration project, recorded
  /// honestly rather than invented.
  pub project_id: String,
  pub threads: Vec<Thread>,
  pub issues: Vec<Issue>,
  pub events: Vec<Envelope>,
}

impl Bundle {
  pub fn new(
    project_id: &str,
    threads: Vec<Thread>,
    issues: Vec<Issue>,
    events: Vec<Envelope>,
  ) -> Self {
    Self {
      schema: BUNDLE_SCHEMA.to_string(),
      intent_version: INTENT_VER.to_string(),
      project_id: project_id.to_string(),
      threads,
      issues,
      events,
    }
  }
}

/// A format's writer. The cause on failure is carried into the refusal rather
/// than discarded, because "the exporter failed" without a cause is the
/// silent-error shape one layer up.
///
/// **The error is `serde_json::Error` and it used to be a bare string, which is
/// the change worth explaining.** Both halves below returned a stringly-typed
/// error, and both reached it the same way -- a `map_err(|e| e.to_string())`
/// applied at the point of failure, which flattened a structured error into
/// prose the instant it was born. Everything a caller could have done with it
/// -- `classify`, `line`, `column`, distinguishing a syntax error from a data
/// one -- was gone before any caller saw it, and what arrived at the refusal
/// was the same text either way. **Carrying the type to the boundary and
/// formatting it THERE gives the identical message and keeps the information**;
/// the `map_err` pair is deleted rather than moved, so the two adapters are now
/// the bare calls they always wanted to be.
///
/// **It also names a constraint that was previously implicit: these aliases now
/// say the round-tripping formats are serde ones.** That is true of every
/// format in [`FORMATS`] and the register records why the one non-serde
/// candidate was withdrawn -- so a future format outside serde is a design
/// moment that should have to change this line, rather than one that quietly
/// stringifies through it.
///
/// **The prose above avoids spelling the stringly-typed signature it is
/// describing, and that is not squeamishness.** `IN-RS-CODE-004` is detected by
/// grep, so a comment quoting the shape it forbids is reported as an instance
/// of it -- which makes explaining a fix indistinguishable from committing the
/// defect. The whiteboard header guard reaches the same conclusion from the
/// other side and refuses to scan prose at all, for the same reason: a rule
/// whose detector cannot tell a subject from a mention taxes the person
/// documenting the repair.
type Emit = fn(&Bundle) -> Result<String, serde_json::Error>;

/// A format's reader -- the half that makes a round-trip claim checkable.
/// Typed for the reason [`Emit`] is.
type Read = fn(&str) -> Result<Bundle, serde_json::Error>;

/// What a format can do with a bundle.
pub enum Projection {
  /// Writes it and reads it back. **Both halves are required to be in the
  /// roster**: a format that could only be written could not have its claim
  /// checked, which is the state AC-06.6 exists to rule out.
  RoundTrips { emit: Emit, read: Read },
  /// Cannot carry the canon back, so it is refused by name.
  Lossy {
    /// Why it cannot -- the operator's actual question.
    because: &'static str,
    /// What to reach for instead. A refusal with no route is a dead end.
    instead: &'static str,
  },
}

pub struct Format {
  pub name: &'static str,
  pub help: &'static str,
  pub projection: Projection,
}

/// THE roster. Adding a format means adding a row here and nothing else.
pub const FORMATS: &[Format] = &[
  Format {
    name: "json",
    help: "the whole estate as one JSON document -- lossless, and the default",
    projection: Projection::RoundTrips {
      emit: emit_json,
      read: read_json,
    },
  },
  Format {
    // **REFUSED ON A MEASUREMENT, and the measurement is the whole entry.**
    // design.md:57 says YAML export is "trivial via serde" and names it as the
    // reason v3 can refuse YAML canon without refusing YAML users. It was
    // built, and the numbers below are why it is not shipped.
    //
    // Measured 2026-08-16 on this workspace, `serde_norway` 0.9.42 (the
    // maintained `serde_yaml` fork) emitting 24 hazardous scalars:
    //
    //   - **our own reader: 24 of 24 survive.** The emitter follows the YAML
    //     1.2 core schema, so it quotes `007`, `.inf`, `null`, `~`, `0x1F` and
    //     `true`, and reads its own output back exactly. AC-06.6's round-trip
    //     test PASSES.
    //   - **PyYAML 6.0.3 `safe_load`: 6 of 24 are silently corrupted.** It
    //     resolves with YAML 1.1, where the emitter's plain scalars mean
    //     something else: `no` -> `False`, `yes` -> `True`, `on` -> `True`,
    //     `off` -> `False`, `12:30` -> `750`, and `2026-08-14` -> a `date`.
    //
    // **The last one disqualifies it on its own**: every `created` and
    // `completed` field in the canon is an ISO date string, so the most widely
    // deployed YAML reader in existence turns a documented string field into
    // another type on every thread in the estate. The others are ordinary
    // English words that appear in titles and prose.
    //
    // Emitting a `%YAML 1.2` directive does not help -- measured: PyYAML
    // ACCEPTS the directive and then resolves 1.1 anyway, so the document would
    // carry a claim about itself that its reader ignores.
    //
    // **This is exactly the case AC-06.6's second arm exists for, and it is
    // subtler than the `md` row below.** Nothing here is broken from where we
    // stand: the file is valid YAML, we read it back perfectly, and the round
    // trip is green. The loss happens in the consumer, which is the one place
    // an interchange artefact must not lose anything -- under D34 a lossy
    // extract does not inconvenience someone, it destroys truth where nobody
    // typed anything.
    //
    // **It also corrects design.md:57 rather than merely declining to
    // implement it.** The 0012 quoting scar that put JSON in the canon is not
    // a fact about canon; it is a fact about YAML, and it does not stop being
    // true because the file is called an export. Raised with vc, who owns the
    // design record.
    name: "yaml",
    help: "REFUSED -- valid YAML that common readers resolve to the wrong types",
    projection: Projection::Lossy {
      because: "a YAML document cannot say which YAML version reads it, and the two disagree: emitted 1.2-correctly, `no` `yes` `on` `off` come back as booleans under YAML 1.1, `12:30` as the integer 750, and a date like `2026-08-14` as a date object rather than the string the canon holds -- measured at 6 of 24 hazardous values corrupted by PyYAML 6.0.3, which is what most consumers use",
      instead: "Use `--format json`, which every YAML parser also accepts -- YAML 1.2 is a superset of JSON, so a YAML consumer can read the JSON export directly and get the types the canon actually holds",
    },
  },
  Format {
    // **design.md:57 names `md` as an export projection and it is not one**,
    // which is worth stating here rather than quietly omitting: markdown is
    // the GENERATED VIEW of the canon (D02). A view renders what a reader
    // needs and drops what only a machine needs -- an id that is implied by a
    // heading, a state that is implied by a column, a field with no column at
    // all -- so nothing can read one back into the model it came from. That is
    // not a gap in the renderer; it is what a view IS.
    //
    // This is exactly the case AC-06.6's second arm is written for, and it is
    // the one that would have been shipped: emitting markdown is easy, the
    // renderer already exists, and the result looks entirely correct until
    // someone tries to re-read it.
    name: "md",
    help: "REFUSED -- markdown is the generated view, and a view cannot be read back",
    projection: Projection::Lossy {
      because: "markdown is the generated VIEW of the canon: it renders what a reader needs and drops what only a machine needs, so no reader can turn it back into the model it came from",
      instead: "The views are already in the tree -- `intent sync --to-disk` rewrites them; for data a program will read, use `--format json`",
    },
  },
];

/// The default when `--format` is absent.
///
/// **A default is safe HERE and would not be on `sync`**, and the difference is
/// worth naming because the rule looks the same from a distance. `sync`'s two
/// directions differ in DESTRUCTIVENESS, so guessing wrong destroys work; the
/// formats here differ only in encoding, every one of them is a read, and
/// guessing wrong costs the operator one re-run.
pub const DEFAULT_FORMAT: &str = "json";

/// Why an export did not happen. Three causes, kept apart because the operator
/// has to do something different about each -- and one of them is not their
/// fault at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportRefusal {
  /// No such format.
  ///
  /// **The two lists are separate because listing the roster is wrong**, and
  /// this was found by running the command rather than by reading it. The
  /// first version answered `xml` with "one of: json, yaml, md" -- offering,
  /// as the remedy for a refusal, two more formats that also refuse. The
  /// operator's next move would have been a second refusal, which is the
  /// remedy-that-cannot-be-acted-on defect in a new place.
  ///
  /// `refused` is still carried rather than dropped: someone who typed `xml`
  /// may well try `yaml` next, and knowing it is recognised and declined saves
  /// them the attempt.
  Unknown {
    name: String,
    /// Formats that actually produce an artefact.
    emits: Vec<String>,
    /// Formats the roster recognises and declines.
    refused: Vec<String>,
  },
  /// A format in the roster that declares it cannot carry the canon back.
  Lossy {
    name: String,
    because: &'static str,
    instead: &'static str,
  },
  /// A format that CLAIMS to round-trip and did not, on this estate.
  ///
  /// **This is our defect and not the operator's**, and it is reported at all
  /// because the alternative is writing the artefact anyway. Its whole value
  /// is that it happens instead of a silent loss.
  RoundTripFailed { name: String, detail: String },
}

/// Look a format up by name.
pub fn find(name: &str) -> Option<&'static Format> {
  FORMATS.iter().find(|f| f.name == name)
}

/// Every format the roster carries, in declaration order.
pub fn names() -> Vec<&'static str> {
  FORMATS.iter().map(|f| f.name).collect()
}

/// The formats that actually produce an artefact -- what "one of:" may offer.
pub fn emitting_names() -> Vec<&'static str> {
  FORMATS
    .iter()
    .filter(|f| matches!(f.projection, Projection::RoundTrips { .. }))
    .map(|f| f.name)
    .collect()
}

/// The formats the roster recognises and declines. Worth telling someone about,
/// and never worth offering.
pub fn refused_names() -> Vec<&'static str> {
  FORMATS
    .iter()
    .filter(|f| matches!(f.projection, Projection::Lossy { .. }))
    .map(|f| f.name)
    .collect()
}

/// Project a bundle into a named format, or refuse.
///
/// **The verification is not optional and there is no flag to skip it.** It
/// runs on the bundle actually being exported rather than on a fixture, so the
/// guarantee attaches to the artefact in the operator's hand: what this returns
/// has been read back and shown to re-derive the canon byte for byte.
pub fn project(bundle: &Bundle, name: &str) -> Result<String, ExportRefusal> {
  let Some(format) = find(name) else {
    return Err(ExportRefusal::Unknown {
      name: name.to_string(),
      emits: emitting_names().iter().map(|s| s.to_string()).collect(),
      refused: refused_names().iter().map(|s| s.to_string()).collect(),
    });
  };
  project_with(bundle, format)
}

/// Project through a format VALUE rather than through a roster lookup.
///
/// **Split out so the verifier can be driven by a format the roster does not
/// contain**, which is the only way to demonstrate that it has teeth: a guard
/// that only ever sees formats which pass cannot distinguish "they all
/// round-trip" from "nothing is being checked". A test constructs a projection
/// that drops a field and requires this to refuse it.
///
/// Splitting it also separates two genuinely different jobs -- deciding WHICH
/// format, and applying one -- so neither is reachable only through the other.
pub fn project_with(bundle: &Bundle, format: &Format) -> Result<String, ExportRefusal> {
  match &format.projection {
    Projection::Lossy { because, instead } => Err(ExportRefusal::Lossy {
      name: format.name.to_string(),
      because,
      instead,
    }),
    Projection::RoundTrips { emit, read } => {
      let text = emit(bundle).map_err(|detail| ExportRefusal::RoundTripFailed {
        name: format.name.to_string(),
        detail: format!("writing it failed: {detail}"),
      })?;
      verify(bundle, &text, format.name, *read).map(|()| text)
    }
  }
}

/// Read the emitted text back and require it to re-derive the canon exactly.
///
/// It takes the reader as an argument rather than re-matching the projection:
/// the caller has already established that this format has one, and a second
/// match here would need an arm for a case that cannot occur -- an unreachable
/// branch whose only possible behaviour is to pass something it did not check.
fn verify(bundle: &Bundle, text: &str, name: &str, read: Read) -> Result<(), ExportRefusal> {
  let refuse = |detail: String| ExportRefusal::RoundTripFailed {
    name: name.to_string(),
    detail,
  };
  let back = read(text).map_err(|e| refuse(format!("reading it back failed: {e}")))?;

  // **Compared as CANON rather than as bundles**, and the difference is the
  // criterion's own wording: "re-ingests to a byte-identical canon". A `==` on
  // two `Bundle` values would answer a weaker question -- whether the values
  // are equal -- where what is promised is that the FILES come back the same,
  // which additionally pins field order and the canonical encoding.
  let before = canon_parts(bundle).map_err(|e| refuse(format!("canon of the source: {e}")))?;
  let after = canon_parts(&back).map_err(|e| refuse(format!("canon of the round-trip: {e}")))?;

  if before.len() != after.len() {
    return Err(refuse(format!(
      "the round-trip carries {} canon file(s), the source has {}",
      after.len(),
      before.len()
    )));
  }
  for ((path, source), (back_path, back_text)) in before.iter().zip(after.iter()) {
    if path != back_path {
      return Err(refuse(format!(
        "the round-trip reordered the canon: expected {path}, got {back_path}"
      )));
    }
    if source != back_text {
      return Err(refuse(format!(
        "{path} did not survive: {}",
        first_difference(source, back_text)
      )));
    }
  }
  Ok(())
}

/// The canon a bundle re-derives: one entry per file the estate would carry,
/// in the canonical bytes those files hold.
///
/// The paths are the real ones so a refusal names a file the operator can go
/// and look at, rather than an index into a list they cannot see.
pub fn canon_parts(bundle: &Bundle) -> Result<Vec<(String, String)>, serde_json::Error> {
  let mut out = Vec::with_capacity(bundle.threads.len() + bundle.issues.len() + 1);
  for thread in &bundle.threads {
    out.push((
      crate::project::canon_thread_rel(&thread.id),
      to_canonical_json(thread)?,
    ));
  }
  for issue in &bundle.issues {
    out.push((
      // **ZERO-PADDED, and it was not.** This emitted `issues/46.json` while
      // every reader resolved `issues/0046.json` -- two spellings of one path,
      // and the exporter's was the one no reader could open. A thread id
      // arrives already padded as text; a `u32` is padded by whoever formats
      // it, so the two ends had to agree by CONVENTION and did not.
      //
      // **THE CONVENTION IS GONE, WHICH IS THE ACTUAL FIX** (D57-1's
      // relocation): both arms now call the one function that spells a canon
      // path, so there is no second end to disagree. The note stays because it
      // says why a shared spelling is required rather than tidy -- delete it
      // and the next person to want a literal here has only the aesthetics.
      crate::project::canon_issue_rel(issue.number),
      to_canonical_json(issue)?,
    ));
  }
  out.push((event::JSONL.to_string(), event::to_jsonl(&bundle.events)?));
  Ok(out)
}

/// The OPAQUE attachments a bundle carries, as `(canon path, bytes)` (ST0057
/// AC-03.1).
///
/// **Separate from [`canon_parts`] because the two cannot share a return type,
/// and that is the design rather than a limitation.** `canon_parts` returns
/// `String`s; an opaque attachment's content is not a `String` and the whole
/// point of AC-03.2 is that it never becomes one. Widening `canon_parts` to
/// bytes would make every JSON caller handle a `Vec<u8>` it will never receive,
/// and would let a future writer put opaque bytes through the inline path with
/// nothing objecting.
///
/// **They partition rather than overlap**: a path is in exactly one of the two,
/// because `text.is_none()` decides it and a canon JSON path is never an
/// attachment path. So "everything canon holds" is their union, and a caller
/// that writes both has written all of it.
///
/// Empty for every project in this estate today -- there are no opaque
/// attachments to carry -- which is a fact about the corpus and not about the
/// function, and is why the test for it constructs its own.
pub fn canon_blobs(bundle: &Bundle) -> Vec<(String, Vec<u8>)> {
  let mut out = Vec::new();
  for thread in &bundle.threads {
    for att in &thread.attachments {
      // **Asked of `blob`, not of `is_opaque`, and the difference is a file.**
      // An opaque attachment whose sidecar was never loaded is `is_opaque()`
      // and has no bytes; emitting it would write an EMPTY file over the only
      // copy of its content. `blob` is `Some` exactly when there is something
      // to write.
      if let Some(raw) = &att.blob {
        out.push((
          crate::project::canon_blob_rel(&thread.id, &att.path),
          raw.clone(),
        ));
      }
    }
  }
  out
}

/// Name the first byte that differs, with a little of each side around it.
///
/// **A diff that says only "they differ" costs the reader the whole
/// investigation**, and this failure is one they cannot reproduce by hand --
/// the emitted text is not written anywhere, because refusing means not writing
/// it. So the refusal has to carry the evidence with it.
fn first_difference(source: &str, back: &str) -> String {
  let at = source
    .char_indices()
    .zip(back.char_indices())
    .find(|((_, a), (_, b))| a != b)
    .map(|((i, _), _)| i)
    .unwrap_or_else(|| source.len().min(back.len()));
  let from = at.saturating_sub(30);
  format!(
    "first difference at byte {at} -- source {:?}, round-trip {:?}",
    window(source, from, at + 30),
    window(back, from, at + 30)
  )
}

fn window(text: &str, from: usize, to: usize) -> &str {
  let start = (0..=from.min(text.len()))
    .rev()
    .find(|i| text.is_char_boundary(*i))
    .unwrap_or(0);
  let end = (to.min(text.len())..=text.len())
    .find(|i| text.is_char_boundary(*i))
    .unwrap_or(text.len());
  &text[start..end]
}

// ---------------------------------------------------------------------------
// The projections
// ---------------------------------------------------------------------------

/// The same canonical convention every other JSON artefact uses -- 2-space
/// pretty, LF, trailing newline -- so the bundle reads like the canon it
/// carries.
fn emit_json(bundle: &Bundle) -> Result<String, serde_json::Error> {
  to_canonical_json(bundle)
}

fn read_json(text: &str) -> Result<Bundle, serde_json::Error> {
  serde_json::from_str(text)
}

// **There is no YAML writer here, and its absence is a result rather than an
// omission.** One was built, against `serde_norway` 0.9.42, and it passed its
// round trip 24 hazards out of 24 -- see the `yaml` row in [`FORMATS`] for what
// a third-party reader then did with the same bytes, and why the dependency was
// removed along with the code. The measurement that replaced it is worth more
// than the feature would have been: it says what `--format json` is FOR, which
// is that a YAML consumer can read it and get the types the canon holds.
